//! `plan.md` PHASE-04: `serve`'s two ingress arms, the second anchor, and
//! what a refusal costs the thread a person is looking at. `plan.md`
//! PHASE-05 extends this same file: AC-6's three anchor cases — the one
//! ADR-004 has been waiting for since slice 003 among them — R-15's bound
//! held from both directions, and the flood that must not reach the
//! backend.
//!
//! Every case here drives the **production** `serve` with a **real bound
//! `Ingress`** over a real Unix domain socket and a real child process — the
//! same function `wiring.rs` and `scheduling.rs` drive, with one more argument.
//! It is the only module in this target that opens a socket, and it opens one
//! because an envelope's effect on the loop is not observable any other way.
//!
//! **The writer's side is `std::os::unix::net`, on `spawn_blocking`.**
//! `tokio::net` reaches this target only by feature unification through
//! `goad-shell`, and `crates/goad/Cargo.toml` is not this phase's to change;
//! the standard library's socket depends on nothing this crate does not
//! already declare, and running it on the blocking pool keeps it off the
//! thread `serve` is on.
//!
//! **`plan.md` PHASE-04/EX-11 and PHASE-05/EX-5 — the same rule, stated
//! twice because two phases wrote to this file — govern it whole:** every
//! case that lets an exchange complete pins that exchange's `next_check`,
//! because `controller.rs`'s re-arm reads it (SPEC-001/R-26) and an unpinned
//! script decides when the next scheduled firing lands. Each member says
//! what it pinned and why. VT-2 (PHASE-04's) is not a member — it asserts a
//! view, not a count or a time — and VT-8 (PHASE-04's) is not in this file at
//! all: it is a unit case in `controller.rs`'s own `#[cfg(test)] mod tests`,
//! because the spacing's boundary instant is reachable only by constructing
//! it. PHASE-05's own six members are named `PHASE-05/VT-1`..`VT-6` in their
//! doc comments, to keep them apart from PHASE-04's own VT-1..VT-8.

use std::cell::Cell;
use std::io::{Read as _, Write as _};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

use goad::controller::{Controller, Ending, Frame, serve};
use goad::glass::{Glass, SlintGlass};
use goad::wire::{Cancel, Command, Notice, Stimulus};
use goad_shell::ingress::bind;
use slint::{ComponentHandle, Model as _};
use tokio::sync::mpsc;
use tokio::task::LocalSet;

use crate::driving::{host, instant};
use crate::harness::{
  TIMEOUT, current_view_token, glass_over, now, stub_clock, until, window_and_tray,
};
use crate::scripting::{invocations, logging_backend, scripted};
use crate::waiting::{LIVENESS_BOUND, within};

/// `design.md` §5.2's own example envelope. `+10:00`, deliberately: A-3's
/// claim is that the same instant comes back out as `Z`, and VT-1 checks it
/// at the point of use rather than trusting the line.
const ENVELOPE: &str = r#"{"source":"reddit-watcher","kind":"reddit-opened","timestamp":"2026-08-22T17:10:00+10:00","data":{"count_last_hour":4}}"#;

/// Not one JSON document — `Refusal::Malformed`, the shape refusal
/// PHASE-05/VT-5 and VT-6 need. The same literal
/// `crates/goad-shell/tests/integration/ingress.rs`'s own malformed cases
/// use.
const MALFORMED: &str = "not json";

/// An exchange that shows nothing and pins the next check a minute out — far
/// enough that no scheduled firing lands inside any window this file measures,
/// and **stated** rather than defaulted (EX-11).
const NEXT_CHECK_A_MINUTE_OFF: &str = r#"{"view":null,"next_check":"60 seconds"}"#;
/// The same, with one option to answer, for VT-2.
const A_VIEW: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"yes","label":"Yes"}]},"next_check":"60 seconds"}"#;

/// PHASE-04/VT-5's measured window: the flat-out writer's whole run. Far
/// shorter than the spacing, which is what makes every reply in it a refusal
/// the loop decided **while idle** — the state SPEC-003/R-15 obliges the host
/// to report. Reused by PHASE-05/VT-6 for its malformed flood: "far shorter
/// than the spacing" is the same requirement either way.
const FLAT_OUT_WINDOW: Duration = Duration::from_millis(500);
/// VT-7's anti-spin window: how long the parked arm is watched for a
/// presentation that must not come.
const ANTI_SPIN_WINDOW: Duration = Duration::from_millis(500);

/// The floor, mirrored. `controller::MINIMUM_SPACING` is private on purpose
/// (D-5: a host operational budget, not a value anything outside the loop
/// reads), so a case that needs to reason about it states it here — the same
/// call `renderer/scheduling.rs:52`'s `FLOOR_MILLIS` makes, and the mirror is
/// checked by nothing but this comment.
const MINIMUM_SPACING: Duration = Duration::from_secs(3);

const _: () = assert!(
  FLAT_OUT_WINDOW.as_millis() * 2 < MINIMUM_SPACING.as_millis(),
  "VT-5's window must be far shorter than the spacing, or `too_soon` is not what its \
   excess replies are"
);
const _: () = assert!(
  ANTI_SPIN_WINDOW.as_millis() * 2 < MINIMUM_SPACING.as_millis(),
  "VT-7's window must close before `serve`'s initial arm fires, or the firing assertion 3 \
   turns on lands inside the window assertion 2 asserts nothing happens in"
);

// ---------------------------------------------------------------------------
// The socket, and a watcher's side of one connection
// ---------------------------------------------------------------------------

/// A path no other case will collide with, cleared before it is handed out —
/// `std::env::temp_dir()` and the process id, because `tempfile` is not on the
/// manifest allowlist (`plan.md` PL-3) and this phase adds no dependency.
fn socket_path(case: &str) -> PathBuf {
  let path = std::env::temp_dir().join(format!("goad-serve-{case}-{}.sock", std::process::id()));
  match std::fs::remove_file(&path) {
    Ok(()) | Err(_) => (),
  }
  path
}

/// Removes the socket **and the lock file beside it**: the host unlinks
/// neither (`SPEC-003/R-5`), so a case that only removes the socket leaves
/// two files per run in `temp_dir()` instead of one.
fn cleanup(path: &Path) {
  match std::fs::remove_file(path) {
    Ok(()) | Err(_) => (),
  }
  match std::fs::remove_file(goad_shell::ingress::lock_path(path)) {
    Ok(()) | Err(_) => (),
  }
}

/// One connection: connect, write one envelope and a newline, read the one
/// line back, close. Blocking, and always called from the blocking pool.
fn write_one(path: &Path, envelope: &str) -> String {
  let mut stream = UnixStream::connect(path).expect("the host must be listening");
  stream
    .write_all(envelope.as_bytes())
    .expect("writing the envelope must succeed");
  stream
    .write_all(b"\n")
    .expect("writing the terminator must succeed");
  let mut reply = String::new();
  stream
    .read_to_string(&mut reply)
    .expect("the host must reply and close");
  reply
}

/// One envelope, off the thread `serve` runs on.
async fn send(path: &Path, envelope: &str) -> String {
  let path = path.to_owned();
  let envelope = envelope.to_owned();
  tokio::task::spawn_blocking(move || write_one(&path, &envelope))
    .await
    .expect("the writer must not panic")
}

/// A writer emitting `envelope` flat out for `window`: one connection after
/// another, each awaiting its reply before opening the next, with nothing
/// between them.
///
/// `envelope` is a parameter — not always `ENVELOPE` — so PHASE-05/VT-6's
/// flood of `MALFORMED` bytes shares this rather than repeating it (DRY):
/// the shape is identical, only the payload differs.
async fn flat_out(path: &Path, window: Duration, envelope: &'static str) -> Vec<String> {
  let path = path.to_owned();
  tokio::task::spawn_blocking(move || {
    let deadline = Instant::now() + window;
    let mut replies = Vec::new();
    while Instant::now() < deadline {
      replies.push(write_one(&path, envelope));
    }
    replies
  })
  .await
  .expect("the writer must not panic")
}

fn parsed(reply: &str) -> serde_json::Value {
  serde_json::from_str(reply).expect("the reply must be JSON")
}

fn accepted(reply: &str) -> bool {
  parsed(reply)["accepted"] == serde_json::json!(true)
}

fn reason(reply: &str) -> String {
  parsed(reply)["reason"]
    .as_str()
    .expect("a refusal must name a reason")
    .to_owned()
}

/// The *n*th (1-indexed) request the backend logged, parsed.
fn logged_request(log: &Path, n: usize) -> serde_json::Value {
  let text = std::fs::read_to_string(log).expect("the invocation log must exist by now");
  let line = text
    .lines()
    .nth(n - 1)
    .expect("a request must be logged at this position");
  serde_json::from_str(line).expect("a logged request is valid JSON")
}

// ---------------------------------------------------------------------------
// The counting glass (PL-8)
// ---------------------------------------------------------------------------

/// A `Glass` that counts presentations and delegates to the **real** one.
///
/// It wraps `SlintGlass` rather than replacing it because the cost
/// `review-design.md` F-15 names is `glass.rs:67-121`'s own work — eleven
/// window properties, two `VecModel` rebuilds, the tray image and the tooltip,
/// `show()`/`hide()` — and a stub would measure none of it. `Rc<Cell<usize>>`
/// is enough: `serve`'s `G` is not required to be `Send`.
///
/// It lives here rather than in `harness.rs` because this file is its only
/// consumer today (PL-8); the phase that needs a second one moves it.
#[derive(Debug)]
struct CountingGlass {
  inner: SlintGlass,
  presentations: Rc<Cell<usize>>,
}

impl Glass for CountingGlass {
  fn present(&mut self, frame: Frame<'_>) {
    self
      .presentations
      .set(self.presentations.get().saturating_add(1));
    self.inner.present(frame);
  }
}

// ---------------------------------------------------------------------------
// VT-1 — AC-1, SPEC-003/R-11: an envelope becomes one evaluation
// ---------------------------------------------------------------------------

/// **EX-11 member.** The one exchange it lets complete pins `next_check` a
/// minute off, so the assertion about *how many* requests reached the backend
/// cannot be answered by a scheduled firing the script chose by default.
/// *Liveness, bounded by `LIVENESS_BOUND`.*
#[tokio::test]
async fn a_well_formed_envelope_produces_one_evaluation_carrying_all_four_fields() {
  let path = socket_path("vt1");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (mut command, log) = logging_backend("logs-the-request-then-answers", "ingress-vt1");
  command.arguments.push(NEXT_CHECK_A_MINUTE_OFF.to_owned());
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (_tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let reply = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });
      let reply = send(&path, ENVELOPE).await;
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
      stopper.stop();
      let served = handle.await.expect("serve must not panic");
      assert_eq!(served.ending, Ending::Stopped);
      reply
    })
    .await;
  cleanup(&path);

  assert!(
    accepted(&reply),
    "a well-formed envelope is accepted: {reply}"
  );
  assert_eq!(invocations(&log), 1, "exactly one evaluation, not two");

  let request = logged_request(&log, 1);
  assert_eq!(request["type"], "evaluate");
  assert_eq!(request["event"]["source"], "reddit-watcher");
  assert_eq!(request["event"]["kind"], "reddit-opened");
  assert_eq!(
    request["event"]["data"],
    serde_json::json!({"count_last_hour": 4}),
    "`data` is opaque and reaches the backend unchanged"
  );

  // A-3, checked at the point of use rather than trusted: `+10:00` in, `Z`
  // out, the **same instant**.
  let carried = request["event"]["timestamp"]
    .as_str()
    .expect("every event carries a timestamp");
  assert!(
    carried.ends_with('Z'),
    "the host renders an instant in UTC: {carried}"
  );
  assert_eq!(
    carried.parse::<jiff::Timestamp>().expect("RFC 3339"),
    "2026-08-22T17:10:00+10:00"
      .parse::<jiff::Timestamp>()
      .expect("RFC 3339"),
    "the envelope's instant is carried, not re-read"
  );

  // The request's `now` is the **host's** own instant, not the envelope's.
  assert_eq!(
    request["now"], "2026-01-01T00:00:00Z",
    "`now` is the host's clock (SPEC-003/R-11), not the event's timestamp"
  );
}

// ---------------------------------------------------------------------------
// VT-2 — AC-2: the view reaches the screen and is answerable
// ---------------------------------------------------------------------------

/// **Not an EX-11 member**: it asserts a view on screen and an answer landing,
/// not a count or a time. The instructions still name their `next_check`,
/// because a case that reads better for saying so costs nothing.
/// *Liveness, bounded by `LIVENESS_BOUND`.*
#[tokio::test]
async fn the_view_an_ingested_evaluation_returns_reaches_the_window_and_is_answerable() {
  let path = socket_path("vt2");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("ingress-vt2", &[A_VIEW, NEXT_CHECK_A_MINUTE_OFF]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });
      let reply = send(&path, ENVELOPE).await;
      assert!(accepted(&reply), "the envelope is accepted: {reply}");

      until(LIVENESS_BOUND, || window.get_heading() == "Proceed?").await;
      let view = current_view_token(&window).expect("the view must be on screen");
      tx.send(Command::Choose {
        view,
        option: "yes".to_owned(),
      })
      .await
      .expect("the channel must accept the answer");

      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      until(LIVENESS_BOUND, || !window.window().is_visible()).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    invocations(&log),
    2,
    "the answer reached the backend: an ingested view is answered like any other"
  );
}

// ---------------------------------------------------------------------------
// VT-3 — AC-4, SPEC-003/R-12: `engaged`, decided in the inner arm
// ---------------------------------------------------------------------------

/// **EX-11 member.** The exchange it holds open answers with `@slow-view`'s
/// own pinned body (`next_check` 45 minutes, `answers-as-instructed.sh`), so
/// nothing is scheduled inside the window and the only thing that can advance
/// the invocation count is the exchange under test.
///
/// `@slow-view` is the vehicle because its `sleep 0.2` is in the
/// **foreground**: nothing is backgrounded, so the exchange is provably still
/// running for its length. The refusal is asserted to have arrived *before*
/// the exchange completed — the view had not yet reached the window — which is
/// what makes it the inner arm's answer rather than the outer arm's.
/// *Liveness, bounded by `LIVENESS_BOUND` against a 200 ms exchange, ~25x.*
#[tokio::test]
async fn an_envelope_arriving_during_an_exchange_is_refused_engaged_before_it_completes() {
  let path = socket_path("vt3");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("ingress-vt3", &["@slow-view"]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;

      // The exchange is now in its foreground `sleep 0.2`.
      let reply = send(&path, ENVELOPE).await;
      let landed = current_view_token(&window).is_some();

      assert_eq!(
        reason(&reply),
        "engaged",
        "an exchange in flight is the state answer, and it is the only one this arm gives"
      );
      assert!(
        !landed,
        "the refusal came back before the exchange it was refused for completed"
      );

      // Liveness: the exchange the arrival did not disturb still completes.
      until(LIVENESS_BOUND, || window.get_heading() == "Still there?").await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    invocations(&log),
    1,
    "a refused envelope re-invokes nothing: the same exchange was resumed"
  );
}

// ---------------------------------------------------------------------------
// VT-4 — AC-4, SPEC-003/R-12: `too_soon`, and the anchor that decides it
// ---------------------------------------------------------------------------

/// **EX-11 member.** Both exchanges it lets complete pin `next_check` a minute
/// off, so the second envelope's fate turns on the event anchor and on nothing
/// the scheduler did.
///
/// The liveness control is the half that makes the refusal mean something: an
/// envelope **outside** the spacing is accepted, so `too_soon` is the anchor
/// speaking and not a listener that serves nothing at all.
///
/// *This case waits out `MINIMUM_SPACING` by construction and is therefore
/// exempt from VA-2's 10x margin rule*: the control cannot be observed before
/// three seconds have passed, the only bound governing it is `LIVENESS_BOUND`,
/// and D-5 rejected a configurable spacing precisely so that no test could buy
/// time by moving a bound.
#[tokio::test]
async fn a_second_envelope_inside_the_spacing_is_refused_too_soon_and_says_how_long() {
  let path = socket_path("vt4");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted(
    "ingress-vt4",
    &[NEXT_CHECK_A_MINUTE_OFF, NEXT_CHECK_A_MINUTE_OFF],
  );
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (_tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      let first = send(&path, ENVELOPE).await;
      assert!(accepted(&first), "the first envelope is accepted: {first}");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
      // Waited for by the exchange's own **absorption**, not merely its
      // start: `invocations(&log) >= 1` proves only that it began
      // (`scheduling.rs`'s `absorbed_line` doc comment states the race this
      // avoids), and a second envelope arriving before it is absorbed would
      // be refused `engaged` rather than `too_soon` — a race, not a defect
      // in the anchor, but one that must not be let decide this assertion.
      until(LIVENESS_BOUND, || {
        window.get_next_check()
          == goad::diagnostics::next_check_line(instant("2026-01-01T00:01:00Z"))
      })
      .await;

      let refused = send(&path, ENVELOPE).await;
      assert_eq!(reason(&refused), "too_soon");
      let retry_after_ms = parsed(&refused)["retry_after_ms"]
        .as_u64()
        .expect("a `too_soon` refusal carries `retry_after_ms` (SPEC-003/R-14)");
      assert!(
        retry_after_ms > 0 && retry_after_ms <= 3000,
        "the remaining spacing is inside the three seconds it was measured from: {retry_after_ms}"
      );

      // The control. Waiting what the host itself said to wait is the point:
      // R-14 rounds **up**, so this writer arrives at or after the anchor.
      tokio::time::sleep(Duration::from_millis(retry_after_ms)).await;
      let after = send(&path, ENVELOPE).await;
      assert!(
        accepted(&after),
        "an envelope outside the spacing is accepted, so the refusal above was the anchor \
         and not a listener refusing everything: {after}"
      );

      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    invocations(&log),
    2,
    "two accepted envelopes, two evaluations — the refused one began nothing"
  );
}

// ---------------------------------------------------------------------------
// VT-5 — AC-5, SPEC-003/R-12, and `review-design.md` F-15's settlement
// ---------------------------------------------------------------------------

/// The flat-out writer, and the number F-15 asked for.
///
/// **EX-11 member.** The one exchange it lets complete pins `next_check` a
/// minute off, so no scheduled firing presents anything inside the measured
/// window and the presentation count is the refusals' alone.
///
/// The window opens **after** that exchange has been absorbed, which is what
/// makes every refusal in it one the loop decided **while idle** — the state
/// SPEC-003/R-15 obliges the host to report to a person, and therefore the
/// state whose cost F-15 is about. Three assertions:
///
/// 1. the **invocation** count is bounded — one accepted evaluation per
///    spacing, over a window far shorter than the spacing;
/// 2. every excess reply says `too_soon`;
/// 3. the **presentation** count over that window equals the number of
///    refusals that caused it. One refusal costs one presentation by
///    construction (`design.md` §5.5); this fixes the cost **at one**, so a
///    change that raised it — or that added a second route to the surface —
///    fails here rather than in front of a person.
///
/// What it holds is the cost per refusal, **not** a ceiling on the writer: a
/// test detects, it does not prevent (`design.md` §8 R6).
///
/// *The measured window is 500 ms; the bound governing 3 is `LIVENESS_BOUND`.*
#[tokio::test]
async fn a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal() {
  let path = socket_path("vt5");
  let (window, tray) = window_and_tray();
  let presentations = Rc::new(Cell::new(0_usize));
  let glass = CountingGlass {
    inner: glass_over(&window, &tray),
    presentations: Rc::clone(&presentations),
  };
  let (command, log) = scripted("ingress-vt5", &[NEXT_CHECK_A_MINUTE_OFF]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (_tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");
  let counted = Rc::clone(&presentations);

  let local = LocalSet::new();
  let (served, replies, cost) = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      // Prime the anchor, and let the exchange it began be absorbed before the
      // window opens: from here the loop is idle, so every refusal below is
      // one R-15 obliges the host to report.
      let first = send(&path, ENVELOPE).await;
      assert!(accepted(&first), "the first envelope is accepted: {first}");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
      until(LIVENESS_BOUND, || {
        window.get_next_check()
          == goad::diagnostics::next_check_line(instant("2026-01-01T00:01:00Z"))
      })
      .await;

      let before = counted.get();
      let replies = flat_out(&path, FLAT_OUT_WINDOW, ENVELOPE).await;
      // The last refusal's presentation lands after its reply does, so wait
      // for the count to arrive rather than racing it — then assert it did not
      // overshoot.
      let refusals = replies.len();
      let reached = within(LIVENESS_BOUND, || {
        counted.get().saturating_sub(before) >= refusals
      })
      .await;
      let cost = counted.get().saturating_sub(before);
      assert!(
        reached,
        "every refusal is reported to a person (R-15): {cost} presentations for {refusals} refusals"
      );

      stopper.stop();
      (handle.await.expect("serve must not panic"), replies, cost)
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert!(
    !replies.is_empty(),
    "the writer must actually have written something"
  );

  // 1 — the evaluation rate is not the writer's to raise.
  assert_eq!(
    invocations(&log),
    1,
    "one accepted evaluation per spacing, whatever the writer's rate"
  );

  // 2 — the excess are refused, and refused for the reason that is true.
  for reply in &replies {
    assert_eq!(
      reason(reply),
      "too_soon",
      "every envelope inside the spacing, with the loop idle, is `too_soon`: {reply}"
    );
  }

  // 3 — F-15's number. One presentation per refusal, and not one more.
  assert_eq!(
    cost,
    replies.len(),
    "one refused envelope costs exactly one presentation: {} refusals, {cost} presentations",
    replies.len()
  );
}

// ---------------------------------------------------------------------------
// VT-7 — AC-12, SPEC-003/R-16, and R-15's last clause: the closed channel
// ---------------------------------------------------------------------------

/// The path `plan.md` PHASE-04/EX-8 specifies and nothing else drives: the
/// accept task is gone, so `arrival()` yields `None`.
///
/// **EX-11 member, twice over** — it counts presentations *and* turns on when
/// a firing happens. What it pins: nothing completes an exchange before the
/// anti-spin window, so the only deadline in force during it is `serve`'s own
/// initial arm at `MINIMUM_SPACING`; the window is 500 ms and opens as soon as
/// the fold is visible, which is well inside three seconds, so the scheduled
/// firing assertion 3 turns on cannot land inside it. Assertion 3 is provoked
/// only after the window has closed and been asserted.
///
/// **How `None` is reached.** `bind` spawns the accept task onto whatever
/// runtime is entered when it is called, so this case builds a **second**
/// multi-thread runtime, binds under its guard, keeps the `Ingress`, and
/// `shutdown_background()`s that runtime — dropping its tasks, and with them
/// the channel's only sender. `shutdown_background` rather than `drop`,
/// because dropping a `Runtime` inside an async context panics. No production
/// API is added for it: the real cause is a panic in the accept task, and this
/// is the same observable with no panic to provoke.
///
/// **Where each assertion reads its number.** The fold is read off the live
/// route — the window's own `diagnostic_lines`, written unconditionally by
/// `glass.rs` — and **not** off `Served.controller`: assertion 3's exchange is
/// absorbed, and `absorb` replaces the whole retained `Diagnostics`, so by the
/// time `serve` returns the fold is gone. That the fold happened **once** is
/// held by assertion 2: a second fold would cost a second presentation.
#[tokio::test]
async fn a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating() {
  let path = socket_path("vt7");
  let (window, tray) = window_and_tray();
  let presentations = Rc::new(Cell::new(0_usize));
  let glass = CountingGlass {
    inner: glass_over(&window, &tray),
    presentations: Rc::clone(&presentations),
  };
  let (command, log) = scripted("ingress-vt7", &[NEXT_CHECK_A_MINUTE_OFF]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (_tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let counted = Rc::clone(&presentations);

  let accepting = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("a second runtime must build");
  let ingress = {
    let _entered = accepting.enter();
    bind(&path).expect("binding a fresh path must succeed")
  };
  accepting.shutdown_background(); // drops the accept task, and its sender

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      // 1 — one fold, on the surface, naming that ingress has stopped.
      until(LIVENESS_BOUND, || {
        window.get_diagnostic_lines().row_count() == 1
      })
      .await;
      let line = window
        .get_diagnostic_lines()
        .row_data(0)
        .expect("the fold must be on the surface");
      assert!(
        line.contains("ingress has stopped"),
        "the one refusal that answers no envelope says so: {line}"
      );

      // 2 — the arm parks. A closed `mpsc::Receiver` is ready on **every**
      // poll, so an arm that folded and `continue`d without dropping the
      // receiver would charge one full presentation per iteration, forever.
      // This is the assertion the case exists for.
      let settled = counted.get();
      tokio::time::sleep(ANTI_SPIN_WINDOW).await;
      assert_eq!(
        counted.get(),
        settled,
        "the arm parked: no presentation advanced over {ANTI_SPIN_WINDOW:?} after the fold"
      );

      // 3 — the host still evaluates. `serve`'s initial arm fires at
      // `MINIMUM_SPACING`, outside the window just asserted.
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    invocations(&log),
    1,
    "a dead accept task changes nothing about the schedule"
  );
}

// =============================================================================
// PHASE-05 — AC-6: the two anchors, independent in both directions;
// SPEC-003/R-15's bound, held from both sides; AC-12/R-16, the flood.
// =============================================================================
//
// `plan.md:1493-1585` whole. Doc comments below say which alternative each
// AC-6 case falsifies (EX-1) and, for every exchange a case lets complete,
// what its `next_check` was pinned to and why (EX-5, restating PHASE-04/EX-11
// over this file).

/// PHASE-05's own short instruction. Defined here rather than reached for in
/// `scheduling.rs` (`plan.md`'s implementer note: this module states its
/// own).
const INSTRUCT_300MS: &str = r#"{"view":null,"next_check":"300 milliseconds"}"#;
/// The priming instruction VT-2 opens with: short enough that the real
/// scheduled firing it produces (T0) lands well inside any window this
/// section measures.
const INSTRUCT_100MS: &str = r#"{"view":null,"next_check":"100 milliseconds"}"#;
/// VT-2's discriminator: the **same** string answers both the scheduled
/// exchange at T0 and the ingested exchange at T0+ε, so the two hypotheses
/// agree on everything either exchange resolves to and disagree only about
/// whether the ingested one wrote `floor_until` (VT-2's own doc comment).
const INSTRUCT_1S: &str = r#"{"view":null,"next_check":"1 second"}"#;

// ---------------------------------------------------------------------------
// PHASE-05/VT-1 — AC-6 (i), *does not delay*
// ---------------------------------------------------------------------------

/// **PHASE-05/VT-1 — AC-6 (i), *does not delay*.** Falsifies the third
/// alternative `docs/adr/004-scheduled-firings-are-spaced-from-the-previous-scheduled-firing.md`
/// §Alternatives considered lists — an anchor on "the last thing the host
/// did", written on *any* attempted firing rather than a scheduled one
/// alone. An ingested firing never writes `floor_until`
/// (`controller.rs`'s `ingest`: one write site, `event_floor_until`).
///
/// **EX-5.** No priming exchange: `floor_until` starts already elapsed
/// (`controller.rs:601`) and nothing has fired the timer yet at test start,
/// so the one envelope this case sends is the *first* attempted firing of
/// any kind — the only write `floor_until` could receive here is exactly the
/// one under test. It pins its own `next_check` short (300 ms, the "short
/// `next_check`" AC-6(i) names); the resulting **scheduled** firing — the
/// timer completing on that instruction — is itself pinned a minute off, so
/// nothing else fires inside the window this case measures.
///
/// Under the anchor the resulting scheduled firing is unfloored and lands
/// near 300 ms. Under the rejected alternative, the ingested firing would
/// write `floor_until = now + MINIMUM_SPACING` on its own attempt, and
/// `deadline_after` would floor that same firing to ~3 s. **No separate
/// anti-fire window is needed here (EX-4 does not apply to this case): the
/// bound below is itself the falsifying signal**, the same shape
/// PHASE-04/VT-4's exemption note describes for a wait that *is* the bound
/// under test.
#[tokio::test]
async fn an_ingested_firing_never_writes_the_scheduled_floor() {
  let path = socket_path("p5vt1");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("p5-vt1", &[INSTRUCT_300MS, NEXT_CHECK_A_MINUTE_OFF]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (_tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      let reply = send(&path, ENVELOPE).await;
      assert!(accepted(&reply), "the envelope is accepted: {reply}");

      // Well under `MINIMUM_SPACING` (3s): if the ingested firing had
      // written `floor_until` on its own attempt, this firing would be
      // floored to ~3s and this bound would time out rather than the
      // firing landing early.
      until(Duration::from_secs(2), || invocations(&log) >= 2).await;

      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    invocations(&log),
    2,
    "one accepted envelope, one resulting scheduled firing — nothing else"
  );
}

// ---------------------------------------------------------------------------
// PHASE-05/VT-2 — AC-6 (ii), *does not advance* — ADR-004's own debt
// ---------------------------------------------------------------------------

/// **PHASE-05/VT-2 — AC-6 (ii), *does not advance* — the case ADR-004 has
/// been waiting for since slice 003, and the one CD-3 amends the record to
/// name.** Falsifies the second alternative ADR-004 lists: spacing applied
/// only when the predecessor was itself a scheduled firing, tracked as a
/// boolean (`review-design.md` F-2's finding, restated in ADR-004
/// §Alternatives considered).
///
/// Setup, in order: (1) a priming exchange — a person's `Requested`
/// stimulus, never floored — instructs 100 ms, so the **first real
/// scheduled firing**, T0, lands soon; (2) T0's own exchange writes
/// `floor_until = T0 + MINIMUM_SPACING` (`serve`'s timer arm, unconditional)
/// and is answered `next_check` due at **T0+1s** — under the anchor this is
/// floored to T0+3s (`deadline_after(T0, 1s, T0+3s) = T0+3s`); (3) once T0's
/// exchange has been absorbed, an envelope arrives at T0+ε and is accepted —
/// the event anchor starts already elapsed and nothing has written it yet —
/// and **its own exchange is answered the identical instruction**, so its
/// own resolved deadline is also no later than T0+1s
/// (`controller.rs:507-512`, SPEC-001/R-26) and the two hypotheses disagree
/// only about whether this ingested firing wrote `floor_until`. **EX-5**:
/// both exchanges this case lets complete are pinned as stated above, and
/// the fourth (whichever lands from the floor's release) is pinned a minute
/// off so nothing further fires inside the window measured.
///
/// Under the anchor `floor_until` is untouched by the ingested firing, so
/// its own (also-1s) resolution is *again* floored to T0+3s and the standing
/// deadline does not move. Under the boolean, the ingested firing clears the
/// flag the floor is conditioned on, its own unfloored resolution wins, and
/// the next scheduled evaluation reaches the backend at ~T0+1s instead —
/// `canon-delta.md` CD-3, `design.md` §9.
///
/// **EX-4**: the anti-fire window (to T0+2.7s, 300 ms inside the 3 s floor)
/// is paired with the liveness assertion below it, so a dead anchor — one
/// that never fires again at all — cannot pass this case by omission.
#[tokio::test]
async fn an_ingested_firing_does_not_advance_the_scheduled_floor() {
  let path = socket_path("p5vt2");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted(
    "p5-vt2",
    &[
      INSTRUCT_100MS,
      INSTRUCT_1S,
      INSTRUCT_1S,
      NEXT_CHECK_A_MINUTE_OFF,
    ],
  );
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      // Priming: a person's own `Requested` evaluate — never floored.
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;

      // T0: the first real *scheduled* firing — the timer, not a person —
      // produced by the priming exchange's 100 ms instruction. `floor_until`
      // is written here, unconditionally, by `serve`'s timer arm.
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      let t0 = Instant::now();

      // Wait for T0's own exchange to be absorbed before sending the
      // envelope: the rendered `next_check` (1s) differs from the priming
      // exchange's own (100ms), so its arrival is the observable proof.
      until(LIVENESS_BOUND, || {
        window.get_next_check()
          == goad::diagnostics::next_check_line(instant("2026-01-01T00:00:01Z"))
      })
      .await;

      // T0+ε: the ingested firing. Accepted — the event anchor starts
      // already elapsed and this is its first attempt.
      let reply = send(&path, ENVELOPE).await;
      assert!(accepted(&reply), "the envelope is accepted: {reply}");
      until(LIVENESS_BOUND, || invocations(&log) >= 3).await;

      // Anti-fire: comfortably inside the 3s floor (300ms margin), the
      // fourth invocation — the scheduled evaluation the floor is holding
      // back — must not have landed.
      let anti_fire = Duration::from_millis(2700).saturating_sub(t0.elapsed());
      tokio::time::sleep(anti_fire).await;
      assert_eq!(
        invocations(&log),
        3,
        "the ingested firing must not have advanced the scheduled floor"
      );

      // Liveness, paired per EX-4: the floor does release it, eventually.
      until(LIVENESS_BOUND, || invocations(&log) >= 4).await;

      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
}

// ---------------------------------------------------------------------------
// PHASE-05/VT-3 — AC-6 (iii), the event anchor is not cleared
// ---------------------------------------------------------------------------

/// **PHASE-05/VT-3 — AC-6 (iii), the event anchor is not cleared.** Holds
/// CD-1's new rule, about which ADR-004 makes no claim at all — there is no
/// alternative to falsify here, only the new floor to hold: a **scheduled**
/// firing landing between two envelopes does not clear `event_floor_until`.
///
/// **EX-5.** The first envelope's own exchange is pinned short (300 ms) so
/// the scheduled firing it produces lands *inside* the three-second event
/// spacing that same envelope opened — the arrangement this case needs; the
/// scheduled firing's own `next_check` is pinned a minute off so nothing
/// else fires inside the window measured.
#[tokio::test]
async fn a_scheduled_firing_does_not_clear_the_event_floor() {
  let path = socket_path("p5vt3");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted(
    "p5-vt3",
    &[
      INSTRUCT_300MS,
      NEXT_CHECK_A_MINUTE_OFF,
      NEXT_CHECK_A_MINUTE_OFF,
    ],
  );
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (_tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      // The first envelope: accepted, opens the event spacing, and its own
      // 300ms instruction produces the scheduled firing below.
      let first = send(&path, ENVELOPE).await;
      assert!(accepted(&first), "the first envelope is accepted: {first}");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;

      // The scheduled firing, well inside the still-open event spacing.
      // Waited for by its own **absorption**, not merely its start:
      // `invocations(&log) >= 2` proves only that the exchange began
      // (`scheduling.rs`'s `absorbed_line` doc comment states the race this
      // avoids), and a second envelope arriving before it is absorbed would
      // be refused `engaged` rather than `too_soon` — a race, not a defect
      // in the anchor, but one that must not be let decide this assertion.
      until(LIVENESS_BOUND, || {
        window.get_next_check()
          == goad::diagnostics::next_check_line(instant("2026-01-01T00:01:00Z"))
      })
      .await;

      // A second envelope, still inside the *event* spacing the first
      // envelope opened: still `too_soon` — the scheduled firing between
      // them must not have cleared it.
      let second = send(&path, ENVELOPE).await;
      assert_eq!(
        reason(&second),
        "too_soon",
        "a scheduled firing must not have cleared the event anchor: {second}"
      );

      // Liveness: the event spacing does release, eventually — the anchor is
      // a floor, not a permanent lock. A third envelope, sent after the
      // remaining spacing the refusal itself named, is accepted.
      let retry_after_ms = parsed(&second)["retry_after_ms"]
        .as_u64()
        .expect("a `too_soon` refusal carries `retry_after_ms`");
      tokio::time::sleep(Duration::from_millis(retry_after_ms)).await;
      let third = send(&path, ENVELOPE).await;
      assert!(
        accepted(&third),
        "outside the event spacing the envelope is accepted, so the refusal above was the \
         anchor and not a listener refusing everything: {third}"
      );
      until(LIVENESS_BOUND, || invocations(&log) >= 3).await;

      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
}

// ---------------------------------------------------------------------------
// PHASE-05/VT-4 — R-15, positive: a refusal decided while idle is seen
// ---------------------------------------------------------------------------

/// **PHASE-05/VT-4 — R-15, positive.** A refusal the loop decides while
/// idle — `too_soon`, decided only while idle (`design.md` §5.4 step 3) —
/// reaches the diagnostics surface a person reads.
///
/// **EX-5.** The accepted exchange that precedes the refusal is pinned a
/// minute off, so no scheduled firing intervenes between the refusal and the
/// read and supersedes what the surface holds (`absorb` replaces the whole
/// retained `Diagnostics`, PHASE-04 finding F-b).
///
/// Read off `Served.controller`'s retained diagnostics after the loop stops
/// — the deterministic fallback `plan.md`'s implementer notes allow when the
/// live window's timing would be awkward: nothing absorbs between the
/// refusal and the stop, so the retained value is exactly what the refusal
/// folded.
#[tokio::test]
async fn a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface() {
  let path = socket_path("p5vt4");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("p5-vt4", &[NEXT_CHECK_A_MINUTE_OFF]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (_tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      let first = send(&path, ENVELOPE).await;
      assert!(accepted(&first), "the first envelope is accepted: {first}");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
      until(LIVENESS_BOUND, || {
        window.get_next_check()
          == goad::diagnostics::next_check_line(instant("2026-01-01T00:01:00Z"))
      })
      .await;

      let refused = send(&path, ENVELOPE).await;
      assert_eq!(reason(&refused), "too_soon", "{refused}");

      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  let lines = served.controller.frame(false).diagnostics.lines();
  assert!(
    lines
      .iter()
      .any(|line| line.contains("too_soon") && line.contains("was refused")),
    "R-15: the idle refusal must be on the surface a person reads: {lines:?}"
  );
}

// ---------------------------------------------------------------------------
// PHASE-05/VT-5 — R-15, negative: a refusal decided during an exchange is not
// ---------------------------------------------------------------------------

/// **PHASE-05/VT-5 — R-15, negative.** The same bound, from the other side:
/// a shape refusal (`malformed`) decided **during** an exchange does not
/// reach the surface — it is answered to its writer, and superseded by
/// `absorb` before any presentation, because the inner arm's `continue`
/// never calls `glass.present` (`design.md` §5.2, *the inner arm folds the
/// same refusal*). This is the case `design.md` §5.4 step 1 in the inner arm
/// exists to make buildable — PHASE-04's own finding recorded it as *"built
/// but not driven [t]here"*; this is what drives it. **This is what makes
/// SPEC-003/R-15's bound a claim rather than an excuse**: a refusal reported
/// only when it happens to land while idle, and silently dropped whenever it
/// doesn't, would not be a report a person could rely on.
///
/// **What it reads, and why that and not the retained value**
/// (`review-code.md` F-23). It reads the **live** window while the exchange
/// is still in its sleep. Read at the end instead, the case is green in both
/// worlds — the one where the arm presents nothing, and the one where it
/// calls `glass.present(controller.frame(notice.raised()))` and a person sees
/// the refusal — because `absorb` replaces the whole retained `Diagnostics`
/// either way. The live read discriminates: adding that `present` call turns
/// this red. `landed` is the guard that keeps the negative non-vacuous, in
/// the same shape as the sibling below.
///
/// **EX-5.** `@slow-view`'s own `next_check` is pinned 45 minutes by the
/// script itself (`tests/backends/answers-as-instructed.sh`) — the same
/// vehicle PHASE-04/VT-3 uses — so nothing fires between `absorb` and the
/// read.
#[tokio::test]
async fn a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface() {
  let path = socket_path("p5vt5");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("p5-vt5", &["@slow-view"]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let (served, surface, landed) = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;

      // The exchange is now in its foreground `sleep 0.2`.
      let reply = send(&path, MALFORMED).await;
      assert!(
        !accepted(&reply),
        "malformed bytes are refused whatever the host's state: {reply}"
      );
      assert_eq!(reason(&reply), "malformed");

      // Read the **live** surface here, not the retained `Diagnostics` at the
      // end: the reply is written before the inner arm yields, and the
      // exchange is still in its sleep, so this is the only moment a
      // presentation of this refusal could be seen at all.
      let surface: Vec<String> = {
        let lines = window.get_diagnostic_lines();
        (0..lines.row_count())
          .filter_map(|row| lines.row_data(row))
          .map(|line| line.to_string())
          .collect()
      };
      let landed = current_view_token(&window).is_some();

      // Liveness: the exchange the arrival did not disturb still completes
      // and is absorbed.
      until(LIVENESS_BOUND, || window.get_heading() == "Still there?").await;
      stopper.stop();
      (handle.await.expect("serve must not panic"), surface, landed)
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert!(
    !landed,
    "the read must have happened while the exchange was still running, which is the only \
     moment the refusal could have reached the surface"
  );
  assert!(
    surface.iter().all(|line| !line.contains("was refused")),
    "R-15: a refusal decided during an exchange is reported to its writer and to nobody else \
     — no presentation carried it to the window: {surface:?}"
  );

  // The residue does not survive `absorb` either. This half is not the claim
  // — `absorb` would wipe it whether or not a person had already seen it
  // (`review-code.md` F-23) — it is here because the two together say the
  // refusal reaches no frame at any point in the exchange's life.
  let lines = served.controller.frame(false).diagnostics.lines();
  assert!(
    lines.iter().all(|line| !line.contains("was refused")),
    "the shape refusal must have been superseded by `absorb`: {lines:?}"
  );
}

// ---------------------------------------------------------------------------
// R-15's last clause, from the side no case reached: ingress dies *during* an
// exchange — `review-code.md` F-3
// ---------------------------------------------------------------------------

/// **The contrast case to the one above, and the one nothing drove.**
/// `a_dead_accept_task_is_folded_once_…` kills the accept task while `serve`
/// is idle, so it only ever exercises the **outer** arm; VT-5 above drives the
/// inner arm with a *shape* refusal, which R-15 says a person need not see.
/// The ingress-stopped `unavailable` is neither: R-15 says this surface is the
/// only report there is, and the loop has no later chance, because
/// `Ingress::arrival` parks the arm as it yields `None`.
///
/// Left to the outer loop's own presentation it reached no frame at all — not
/// a race, but guaranteed, because the only non-cancelling exit from the inner
/// loop is `absorb`, which replaces the whole retained `Diagnostics`
/// (`controller.rs`). So the fold is asserted **on the live window**, while the
/// exchange it arrived during is still running.
///
/// **How the inner arm is reached rather than the outer one.** The evaluate is
/// queued on the command channel *before* `serve` starts, and the outer
/// `select!` is `biased` with commands above ingress — so the first iteration
/// takes the command, starts the exchange, and the closed channel is first
/// observed by the inner arm. `@slow-view`'s `sleep 0.2` is in the foreground,
/// so the exchange is provably still running while the assertion reads.
///
/// **The timing this rests on, and why it is safe** (`review-code.md` F-3,
/// recorded as an observation rather than a finding). `shutdown_background`
/// defers the accept task's drop, so in principle the outer arm could observe
/// the closed channel first, before the queued command starts the exchange.
/// The race runs the **safe** way: what precedes the inner arm's first poll is
/// a subprocess spawn, so load lengthens the exchange and favours the inner
/// arm, and losing the race fails this case rather than passing it wrongly —
/// the fold would reach a frame by the outer loop's own presentation and the
/// `landed` assertion would still hold, but `row_count() == 1` would be
/// satisfied by the wrong arm and the case would no longer be testing what it
/// names. Measured 12/12 stable. **If this case ever flakes, that is where to
/// look, and it has been looked at once already** — join it to the flaky-test
/// list in `slice-004.md` Follow-ups rather than starting from zero.
///
/// **EX-5.** `@slow-view` pins its own `next_check` 45 minutes out
/// (`tests/backends/answers-as-instructed.sh`), so nothing fires between the
/// fold and the read.
#[tokio::test]
async fn ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface() {
  let path = socket_path("f3-inner-stop");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("f3-inner-stop", &["@slow-view"]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

  let accepting = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("a second runtime must build");
  let ingress = {
    let _entered = accepting.enter();
    bind(&path).expect("binding a fresh path must succeed")
  };
  accepting.shutdown_background(); // drops the accept task, and its sender

  let local = LocalSet::new();
  let served = local
    .run_until(async {
      // Queued before `serve` runs, so the biased command arm wins the first
      // iteration and the closed channel is met by the inner arm.
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the send");
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      until(LIVENESS_BOUND, || {
        window.get_diagnostic_lines().row_count() == 1
      })
      .await;
      let line = window
        .get_diagnostic_lines()
        .row_data(0)
        .expect("the fold must be on the surface");
      let landed = current_view_token(&window).is_some();

      assert!(
        line.contains("ingress has stopped"),
        "the one refusal that answers no envelope says so: {line}"
      );
      assert!(
        !landed,
        "the report reached a frame while the exchange was still running, which is the only \
         moment it could: `absorb` replaces the whole surface"
      );

      // Liveness: the exchange the dead channel did not disturb still
      // completes, and the host is still evaluating afterwards.
      until(LIVENESS_BOUND, || window.get_heading() == "Still there?").await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    invocations(&log),
    1,
    "a dead accept task changes nothing about the exchange it interrupted"
  );
}

// ---------------------------------------------------------------------------
// PHASE-05/VT-6 — AC-12, SPEC-003/R-16: the flood
// ---------------------------------------------------------------------------

/// **PHASE-05/VT-6 — AC-12, SPEC-003/R-16.** After a flood of malformed
/// envelopes the host still evaluates: an invocation lands, and **none** of
/// the malformed envelopes produced one. Both halves, or the absence
/// assertion is vacuous (`plan.md`'s own words for this case).
///
/// **EX-5.** Nothing is dispatched during the flood — every malformed
/// envelope is refused at step 1, before either anchor is read or written
/// (`controller.rs`'s `ingest`/`refuse_during_exchange`, step 1) — so the
/// only thing that can ever fire is `serve`'s own initial arm, armed
/// unconditionally at `MINIMUM_SPACING` from process start. The one exchange
/// this lets complete pins `next_check` a minute off, so nothing else fires
/// inside the window measured — this is the same firing PHASE-04/VT-7 pins
/// the same way, for the same reason.
#[tokio::test]
async fn after_a_flood_of_malformed_envelopes_the_host_still_evaluates() {
  let path = socket_path("p5vt6");
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("p5-vt6", &[NEXT_CHECK_A_MINUTE_OFF]);
  let backend = host(command, TIMEOUT, now());
  let controller = Controller::new();
  let (_tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let ingress = bind(&path).expect("binding a fresh path must succeed");

  let local = LocalSet::new();
  let (served, replies) = local
    .run_until(async {
      let handle = tokio::task::spawn_local(async move {
        serve(
          backend,
          controller,
          rx,
          cancel,
          Notice::new(),
          stub_clock,
          glass,
          ingress,
        )
        .await
      });

      let replies = flat_out(&path, FLAT_OUT_WINDOW, MALFORMED).await;
      assert_eq!(
        invocations(&log),
        0,
        "no malformed envelope may reach the backend"
      );

      // Liveness: the initial arm still fires, well inside `LIVENESS_BOUND`
      // of the flood's own window closing (~2.5s of `MINIMUM_SPACING`
      // remain).
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
      stopper.stop();
      (handle.await.expect("serve must not panic"), replies)
    })
    .await;
  cleanup(&path);

  assert_eq!(served.ending, Ending::Stopped);
  assert!(
    !replies.is_empty(),
    "the flood must actually have written something"
  );
  for reply in &replies {
    assert_eq!(reason(reply), "malformed", "{reply}");
  }
  assert_eq!(
    invocations(&log),
    1,
    "exactly the one firing the flood did not prevent"
  );
}

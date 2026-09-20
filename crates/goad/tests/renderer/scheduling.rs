//! `plan.md` PHASE-02, item 18: the `select!` timer arm, the floor anchor,
//! and the six timed assertions that drive every branch of it but the
//! refusal one (PHASE-03's) — VT-2..VT-8.
//!
//! Every case here drives the production `serve`, the same function
//! `wiring.rs`'s own `mod serving`/`mod interaction`/`mod cancellation` call,
//! against a real child process — there is no second, test-only loop.

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use goad::controller::{Controller, Ending, serve};
use goad::wire::{Cancel, Command, Notice, Stimulus};
use goad_semantics::protocol::canonical::Timestamp;
use goad_shell::clock::ClockError;
use goad_shell::config::{BackendConfig, Command as ShellCommand, Config, ScheduleConfig};
use goad_shell::ingress::Ingress;
use tokio::sync::mpsc;
use tokio::task::LocalSet;

use crate::driving::{DEFAULT_POLL, host_from, instant};
use crate::harness::{
  TIMEOUT, current_view_token, glass_over, logging_scripted, now, stub_clock, until,
  window_and_tray,
};
use crate::scripting::{invocations, scripted};
use crate::waiting::LIVENESS_BOUND;

/// A successful exchange with nothing to show and no instruction — the
/// caller's own `default_poll` governs the next check.
const NOTHING_INSTRUCTED: &str = r#"{"view":null}"#;
/// A response instructing a wait short enough for a test to observe inside
/// `TIMEOUT`, without specifying an absolute instant (so `stub_clock`'s
/// fixed `now` does not matter — VT-4..VT-7's shared instruction).
const INSTRUCT_100MS: &str = r#"{"view":null,"next_check":"100 milliseconds"}"#;
/// A response instructing a wait far longer than any window this suite
/// measures — VT-6's pending deadline, and VT-7's un-fired supersession.
const INSTRUCT_60S: &str = r#"{"view":null,"next_check":"60 seconds"}"#;
/// One option, so a `Choose` has something to name (VT-5).
const A_VIEW: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"yes","label":"Yes"}]},"next_check":"45 minutes"}"#;
/// An absolute instant at the far edge of what the wire admits: `wait_for`
/// is total across jiff's range, so this is the longest wait a backend can
/// hand the loop (~8000 years).
const INSTRUCT_FAR_FUTURE: &str = r#"{"view":null,"next_check":"9999-12-01T00:00:00Z"}"#;
/// An instant well before `stub_clock`'s fixed `now` (2026-01-01) —
/// PHASE-03's AC-4 case, a backend that instructs a resolved instant that has
/// already elapsed.
const PAST_INSTANT: &str = r#"{"view":null,"next_check":"2020-01-01T00:00:00Z"}"#;
/// The floor, mirrored. `controller::MINIMUM_SPACING` is private on purpose
/// (D-14: a host operational budget, not a value anything outside the loop
/// reads), so a test that needs to reason about it states it here and the
/// mirror is checked by nothing but this comment.
const FLOOR_MILLIS: u128 = 3_000;

/// Every other anti-spin window in this module opens at the same event the
/// floor is measured from, so load can only delay a firing *into* the window
/// and never past it. VT-6's window is the exception: it opens after a third
/// invocation that a person's own exchange has to reach first, so the wait
/// before it is inside the floor's three seconds and its bound is
/// load-bearing for the assertion's correctness.
///
/// Two seconds, not the workspace's five: `FLOOR_SAFE_BOUND + WINDOW <
/// FLOOR_MILLIS` is what keeps a loaded gate from letting the floor expire
/// inside the window and reporting a machine-load delay as a floor defect.
/// The inequality is asserted below rather than left to a reader, because
/// widening the shared liveness bound silently broke it once.
const FLOOR_SAFE_BOUND: Duration = Duration::from_secs(2);
/// VT-6's anti-spin window.
const ANTI_SPIN_WINDOW: Duration = Duration::from_millis(500);
const _: () = assert!(
  FLOOR_SAFE_BOUND.as_millis() + ANTI_SPIN_WINDOW.as_millis() < FLOOR_MILLIS,
  "VT-6's window must close before the floor expires, or a loaded gate reports \
   load as a floor defect"
);

/// PHASE-03's `default_poll`: shorter than the floor (`MINIMUM_SPACING`, 3s),
/// so the process's first scheduled firing lands well inside every window
/// this module measures (D-5).
const POLL_100MS: jiff::SignedDuration = jiff::SignedDuration::from_millis(100);

/// A `Config` around one command, with a caller-chosen `default_poll` — built
/// directly from `goad_shell::config`'s `pub` fields, per PL-3: nothing is
/// added to `tests/support/driving.rs` for it.
fn config_with_poll(command: ShellCommand, default_poll: jiff::SignedDuration) -> Config {
  Config {
    backend: BackendConfig {
      command,
      timeout: TIMEOUT,
    },
    schedule: ScheduleConfig { default_poll },
    ingress: None,
  }
}

/// The `event.kind` of the *n*th (1-indexed) request `logging_scripted`'s
/// backend received.
fn request_kind(log: &Path, n: usize) -> String {
  let text = std::fs::read_to_string(log).expect("the invocation log must exist by now");
  let line = text
    .lines()
    .nth(n - 1)
    .expect("a request must be logged at this position");
  let value: serde_json::Value =
    serde_json::from_str(line).expect("a logged request is valid JSON");
  value["event"]["kind"]
    .as_str()
    .expect("every request carries event.kind")
    .to_owned()
}

/// The observable that proves an exchange was **absorbed**, not merely
/// started: the next-check line the production glass wrote for `rfc3339`.
///
/// The invocation log is written by the backend script *before* it reads the
/// request and answers, so `invocations(&log) >= n` proves only that the nth
/// exchange **began**. A case that stops the loop, or opens a window, on that
/// signal races the exchange it is about: `Cancel` is the first arm of the
/// second `select!`, so a stop arriving mid-flight drops the call, `absorb`
/// never runs, and every assertion about what the exchange resolved fails —
/// three runs in five under the full renderer target, measured.
///
/// `glass.present(controller.frame(notice.raised()))` runs at the top of the
/// iteration *after* `absorb`, so this line cannot be read before the exchange
/// completed and was folded in. Where two exchanges resolve the same instant
/// it proves *an* absorption rather than a particular one, which is all any
/// of these cases need.
fn absorbed_line(rfc3339: &str) -> String {
  goad::diagnostics::next_check_line(instant(rfc3339))
}

/// VT-2 / VT-3 — AC-1, SPEC-002/R-1 and R-5's first half. A `default_poll`
/// (100 ms) shorter than the floor is honoured for the process's **first**
/// scheduled firing, because `floor_until` starts in the past (D-5): the
/// second invocation is the scheduled one, `event.kind == "scheduled"`.
/// *Liveness. Measured ~270 ms against the 5 s liveness bound, ~18x.*
#[tokio::test]
async fn a_short_default_poll_is_honoured_unfloored_for_the_first_scheduled_check() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = logging_scripted("vt2-3", &[NOTHING_INSTRUCTED]);
  let config = config_with_poll(command, jiff::SignedDuration::from_millis(100));
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(invocations(&log), 2);
  assert_eq!(
    request_kind(&log, 2),
    "scheduled",
    "the second request is the host's own, not a person's"
  );
}

/// VT-4 — AC-2, from an `evaluate`. A far `default_poll` (30 min) would not
/// fire a second time inside any window this suite affords; the person's own
/// first evaluate instructs 100 ms, and the scheduled firing that instruction
/// produces lands well within it. *Liveness. Measured ~270 ms against the
/// 5 s liveness bound, ~18x.*
#[tokio::test]
async fn an_instruction_from_an_evaluate_shortens_the_wait_past_a_far_default_poll() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("vt4", &[INSTRUCT_100MS]);
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(invocations(&log), 2);
}

/// VT-5 — AC-2, from a `respond`. The first response carries a view; the
/// `Choose` naming it is answered with a 100 ms instruction, and the third
/// invocation — the scheduled one — lands well within `until(LIVENESS_BOUND)` despite
/// the far `default_poll`. The view token is read off the window's own
/// options model, as `harness::current_view_token` does; never minted
/// independently. *Liveness. Measured ~270 ms against the 5 s liveness bound,
/// ~18x.*
#[tokio::test]
async fn an_instruction_from_a_respond_shortens_the_wait_past_a_far_default_poll() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("vt5", &[A_VIEW, INSTRUCT_100MS]);
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || window.get_heading() == "Proceed?").await;
      let view = current_view_token(&window).expect("the view must be on screen");
      tx.send(Command::Choose {
        view,
        option: "yes".to_owned(),
        edits: Vec::new(),
      })
      .await
      .expect("the channel must accept the click");
      until(LIVENESS_BOUND, || invocations(&log) >= 3).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(invocations(&log), 3);
}

/// VT-6 — AC-3, earlier supersedes. A first instruction of 60 s leaves a far
/// deadline pending; a person-driven `evaluate` then instructs 100 ms, and
/// the scheduled firing lands inside `until(LIVENESS_BOUND)` — which it cannot do if
/// the pending 60 s deadline had survived (this is the case
/// `max(retained, incoming)` gets wrong, as `schedule::resolve`'s own doc
/// says). *Liveness. Measured ~270 ms against the 5 s liveness bound, ~18x.*
#[tokio::test]
async fn an_earlier_instruction_supersedes_a_pending_far_deadline() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("vt6", &[INSTRUCT_60S, INSTRUCT_100MS]);
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the person-driven send");
      until(LIVENESS_BOUND, || invocations(&log) >= 3).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(invocations(&log), 3);
}

/// VT-7 — AC-3, later supersedes. A first instruction of 100 ms; a second
/// `evaluate`, enqueued on a capacity-one channel **before** the first
/// exchange completes (FD-4 — the send only returns once the first command
/// is dequeued, so there is no race), instructs 60 s. **No** invocation
/// beyond the second inside a 300 ms window — deliberately longer than the
/// superseded 100 ms deadline, because the whole content of the test is that
/// that deadline did not survive. The second invocation's `event.kind` is
/// `"requested"`, not `"scheduled"`, which is the evidence the second command
/// really was dispatched from the channel and not from the timer arm.
/// *Anti-fire.*
#[tokio::test]
async fn a_later_instruction_supersedes_and_the_earlier_deadline_does_not_fire() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = logging_scripted("vt7", &[INSTRUCT_100MS, INSTRUCT_60S]);
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      // Blocks until the first send is dequeued, which is necessarily
      // before that exchange's outcome is even awaited, let alone absorbed
      // (FD-4) — so this is enqueued before the first exchange completes by
      // construction, not by timing.
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the second, queued-behind send");
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      assert_eq!(
        request_kind(&log, 2),
        "requested",
        "the second invocation came from the channel, not the timer arm"
      );

      tokio::time::sleep(Duration::from_millis(300)).await;
      assert_eq!(
        invocations(&log),
        2,
        "the superseded 100 ms deadline must not have fired"
      );

      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
}

/// VT-8 — AC-7. With a far `default_poll` and the channel drained, the loop
/// is parked on the timer arm with nothing else ready; `Cancel::stop()` from
/// the driving task still ends `serve` well inside `TIMEOUT`. Its failure
/// mode is a hang, not a late value (S-5). *Liveness. Expected: at once,
/// against `TIMEOUT` (2 s), ~2000x margin.*
#[tokio::test]
async fn a_stop_issued_while_parked_on_the_timer_arm_ends_serve_well_inside_the_timeout() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("vt8", &[NOTHING_INSTRUCTED]);
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

  let local = LocalSet::new();
  let (elapsed, served) = local
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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      // The exchange completed and the loop has looped back to the top
      // `select!` with an empty channel and a far sleep — parked on the
      // timer arm, the scenario AC-7 is about. The rendered line is what
      // says so: it is written at the top of that iteration, after `absorb`.
      // A fixed 20 ms settle stood here, which is a delay used as
      // synchronisation — the shape F-22 is about.
      until(LIVENESS_BOUND, || {
        window.get_next_check() == absorbed_line("2026-01-01T00:30:00Z")
      })
      .await;
      let start = std::time::Instant::now();
      stopper.stop();
      let served = handle.await.expect("serve must not panic");
      (start.elapsed(), served)
    })
    .await;

  assert!(
    elapsed < TIMEOUT,
    "cancellation did not win over the timer arm well inside TIMEOUT (S-5): took {elapsed:?}"
  );
  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(invocations(&log), 1, "no further invocation after the stop");
}

// ---------------------------------------------------------------------------
// PHASE-03 — what the floor bounds, and what a failure does not stop
// (`plan.md` PHASE-03, VT-1..VT-6). Every case still drives the production
// `serve` against a real child process; nothing here is a second loop.
// ---------------------------------------------------------------------------

/// PHASE-03/VT-1 — AC-4, SPEC-002/R-6. A backend that instructs a past
/// instant on **every** response, `default_poll` far: the second invocation
/// (the process's first, unfloored scheduled firing, D-5) lands inside
/// `until(LIVENESS_BOUND)`, and the count then holds at 2 across a 500 ms window — 6x
/// under the 3 s floor. After the loop stops, the retained `next_check` is
/// the instruction itself, verbatim and unadjusted by the floor.
#[tokio::test]
async fn a_past_instant_on_every_response_fires_once_and_then_holds_at_the_floor() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted(
    "vt1-past-every-response",
    &[PAST_INSTANT, PAST_INSTANT, PAST_INSTANT, PAST_INSTANT],
  );
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      until(LIVENESS_BOUND, || {
        window.get_next_check() == absorbed_line("2020-01-01T00:00:00Z")
      })
      .await;
      tokio::time::sleep(Duration::from_millis(500)).await;
      assert_eq!(
        invocations(&log),
        2,
        "the floor must hold the host to one unfloored scheduled firing"
      );
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(invocations(&log), 2);
  assert_eq!(
    served.controller.frame(false).next_check,
    Some(instant("2020-01-01T00:00:00Z")),
    "SPEC-002/R-6: the retained next_check is the instruction, unadjusted by the floor"
  );
}

/// PHASE-03/VT-2 — AC-5, AC-8. A backend failing every invocation
/// (`@garbage`), `default_poll` 100 ms: the second invocation lands inside
/// `until(LIVENESS_BOUND)` — the clock is not stopped (AC-8) — and the count then holds
/// across a 500 ms window (AC-5). The retained `next_check` after the loop
/// stops is unaffected by the failure (SPEC-001/R-29).
#[tokio::test]
async fn a_failing_backend_is_retried_unprompted_never_faster_than_the_floor() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted(
    "vt2-failing-backend",
    &["@garbage", "@garbage", "@garbage", "@garbage"],
  );
  let config = config_with_poll(command, POLL_100MS);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      until(LIVENESS_BOUND, || {
        window.get_next_check() == absorbed_line("2026-01-01T00:00:00.100Z")
      })
      .await;
      tokio::time::sleep(Duration::from_millis(500)).await;
      assert_eq!(
        invocations(&log),
        2,
        "a failing backend must not be retried faster than the floor"
      );
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    served.controller.frame(false).next_check,
    Some(instant("2026-01-01T00:00:00.100Z")),
    "SPEC-001/R-29: the retained next_check is unaffected by the failure"
  );
}

/// A clock that succeeds exactly once and then fails on every subsequent
/// read. `Clock` is a `fn` pointer (`goad_shell::clock`) and cannot capture, so the
/// count lives in a `static`; this fixture is used by exactly one test
/// (PHASE-03/VT-3, below) — `cargo test` runs cases in parallel threads that
/// share one binary's statics.
static CLOCK_READS: AtomicUsize = AtomicUsize::new(0);

fn succeeds_once_then_fails() -> Result<Timestamp, ClockError> {
  if CLOCK_READS.fetch_add(1, Ordering::SeqCst) == 0 {
    Ok(now())
  } else {
    Err(ClockError::BeforeEpoch)
  }
}

/// PHASE-03/VT-3 — AC-9. `default_poll` 100 ms with `succeeds_once_then_fails`:
/// the startup exchange completes and arms ~100 ms out; the timer arm wins,
/// advances `floor_until`, and dispatches the scheduled evaluation, whose
/// `stamp` is the clock's second read and fails; the refusal re-arms at
/// `floor_until`, 3 s out. Inside a 500 ms window: exactly one `NoClock`
/// refusal line, the invocation count still 1, and the retained `next_check`
/// unchanged by the refusal (SPEC-001/R-8).
#[tokio::test]
async fn a_clock_that_fails_after_the_startup_exchange_refuses_and_holds() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("vt3-clock-fails-once", &[NOTHING_INSTRUCTED]);
  let config = config_with_poll(command, POLL_100MS);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          succeeds_once_then_fails,
          glass,
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
      until(LIVENESS_BOUND, || {
        window.get_next_check() == absorbed_line("2026-01-01T00:00:00.100Z")
      })
      .await;
      tokio::time::sleep(Duration::from_millis(500)).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    invocations(&log),
    1,
    "the scheduled firing's clock refusal must never reach the backend"
  );
  let lines = served.controller.frame(false).diagnostics.lines().to_vec();
  assert_eq!(
    lines.len(),
    1,
    "exactly one refusal line, from the scheduled firing's failed stamp; got {lines:?}"
  );
  assert!(
    lines[0].contains("system clock could not be read"),
    "the one refusal must be NoClock; got {lines:?}"
  );
  assert_eq!(
    CLOCK_READS.load(Ordering::SeqCst),
    2,
    "the floor must stop the clock from being re-read in a tight loop after the refusal (VA-3)"
  );
  assert_eq!(
    served.controller.frame(false).next_check,
    Some(instant("2026-01-01T00:00:00.100Z")),
    "SPEC-001/R-8: a clock failure must not discard the retained next_check"
  );
}

/// PHASE-03/VT-4 — the vacuity control for VT-3: the same shape with a clock
/// that never fails reaches a second invocation inside `until(LIVENESS_BOUND)`, so
/// VT-3's "count still one" is not passing because nothing was ever
/// scheduled.
#[tokio::test]
async fn the_same_shape_with_a_working_clock_reaches_a_second_invocation() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("vt4-clock-control", &[NOTHING_INSTRUCTED]);
  let config = config_with_poll(command, POLL_100MS);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    invocations(&log),
    2,
    "vacuity control for VT-3: a working clock does reach the scheduled firing"
  );
}

/// PHASE-03/VT-5 — AC-4's other successor case, SPEC-002/R-3. A past instant
/// on the first response only, then no instruction: the second (unfloored
/// scheduled) invocation lands inside `until(LIVENESS_BOUND)`, the count then holds at 2
/// across a 500 ms window, and the elapsed value is consumed — `resolve`'s
/// second arm applies `now + default_poll`, so the retained `next_check`
/// after the loop stops is neither `2020-01-01…` nor left dangling but
/// exactly `now + default_poll` (S-23). The third invocation is deliberately
/// not awaited: the floor puts it 3 s out.
#[tokio::test]
async fn a_one_off_past_instruction_is_consumed_and_cadence_resumes() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("vt5-one-off-past", &[PAST_INSTANT, NOTHING_INSTRUCTED]);
  let config = config_with_poll(command, POLL_100MS);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      until(LIVENESS_BOUND, || {
        window.get_next_check() == absorbed_line("2026-01-01T00:00:00.100Z")
      })
      .await;
      tokio::time::sleep(Duration::from_millis(500)).await;
      assert_eq!(
        invocations(&log),
        2,
        "SPEC-002/R-3: the elapsed instruction fires once and does not re-fire"
      );
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    served.controller.frame(false).next_check,
    Some(instant("2026-01-01T00:00:00.100Z")),
    "SPEC-002/R-3: the elapsed value was consumed and the default poll applies"
  );
}

/// PHASE-03/VT-6 — SPEC-002/R-4's second half, R-5's second half. VT-1's
/// backend and `default_poll`, plus a person: once the second invocation has
/// landed, the driving task sends `Command::Evaluate(Stimulus::Requested)`.
/// Invocation 3 lands inside `until(FLOOR_SAFE_BOUND)` of that send — a
/// person's evaluation is not delayed by the floor (R-5) — and the count is
/// then exactly 3 and does not move across `ANTI_SPIN_WINDOW` — the person's
/// action did not clear or reset the floor (R-4). Without the floor, the
/// backend's next past instant would fire a fourth invocation at once and
/// fail the window (VA-3).
///
/// **Why this one does not take `LIVENESS_BOUND`.** Its window opens after
/// the *third* invocation, while the floor is measured from the *second*, so
/// everything between them is spent inside the floor's three seconds. Both
/// waits therefore take `FLOOR_SAFE_BOUND`, and the inequality
/// `FLOOR_SAFE_BOUND + ANTI_SPIN_WINDOW < FLOOR_MILLIS` is asserted at
/// compile time. Without it a loaded gate that delays the person's exchange
/// lets the floor expire inside the window, a fourth invocation lands, and
/// the failure reads *"the person's exchange must not have cleared or reset
/// the floor"* — accusing the production floor of a defect the machine
/// caused. A liveness bound blown here says what actually happened.
#[tokio::test]
async fn a_person_acting_mid_cadence_does_not_clear_the_floor() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted(
    "vt6-person-mid-cadence",
    &[PAST_INSTANT, PAST_INSTANT, PAST_INSTANT, PAST_INSTANT],
  );
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(FLOOR_SAFE_BOUND, || invocations(&log) >= 2).await;

      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("R-5: a person's evaluate must be dispatched without waiting for the floor");
      until(FLOOR_SAFE_BOUND, || invocations(&log) >= 3).await;
      tokio::time::sleep(ANTI_SPIN_WINDOW).await;
      assert_eq!(
        invocations(&log),
        3,
        "R-4: the person's exchange must not have cleared or reset the floor"
      );
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
}

/// SPEC-002/R-4 and R-5, from the other side: a refusal that did **not**
/// come from the timer arm must leave the standing deadline exactly where it
/// is. The one exchange instructs 60 s, so a far deadline stands and no
/// scheduled firing is due; `floor_until` is still the process's own start
/// instant, which is in the past. A `Choose` naming a view that was never
/// retained is then refused as `Refused::SupersededView`, read off the tray
/// the production glass wrote.
///
/// *Anti-fire. A 500 ms window against a 60 s deadline.* The window cannot
/// fail under load, because load can only delay a firing. It discriminates
/// because a re-arm on this refusal would reset the sleep to `floor_until`
/// — an instant already in the past — and fire an unwanted scheduled
/// evaluation within a millisecond or two.
#[tokio::test]
async fn a_refusal_that_did_not_come_from_the_timer_leaves_the_deadline_standing() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("refusal-off-the-timer", &[INSTRUCT_60S]);
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(2);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || invocations(&log) >= 1).await;

      tx.send(Command::Choose {
        view: "a token nothing ever minted".to_owned(),
        option: "yes".to_owned(),
        edits: Vec::new(),
      })
      .await
      .expect("the channel must accept the click");
      until(LIVENESS_BOUND, || {
        tray.get_hover_text().contains("since been replaced")
      })
      .await;

      tokio::time::sleep(Duration::from_millis(500)).await;
      assert_eq!(
        invocations(&log),
        1,
        "the refused click must not have re-armed the sleep at the floor"
      );
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert!(
    current_view_token(&window).is_none(),
    "nothing was ever retained, which is why the click is refused"
  );
}

/// SPEC-001/R-45 at the loop level: the longest instruction the wire admits
/// reaches the arithmetic that arms the sleep, and the host survives it.
///
/// The unit test beside `deadline_after` pins the clamp; this pins that a
/// far-future instruction actually travels the whole way there through a
/// real backend, a real `Host` and the production `serve` — the path
/// `wait_for_is_total_at_the_future_edge_of_representable_time` cannot
/// reach, because it tests the pure function alone. Before the add was
/// checked, whether this panicked depended on `std::time::Instant`'s
/// platform representation, which no document states.
///
/// *No timed assertion.* If the arm panicked, the task would die and
/// `handle.await` would carry the panic.
#[tokio::test]
async fn an_instruction_at_the_far_edge_of_time_arms_the_sleep_without_panicking() {
  let (window, tray) = window_and_tray();
  let glass = glass_over(&window, &tray);
  let (command, log) = scripted("far-future-instruction", &[INSTRUCT_FAR_FUTURE]);
  let config = config_with_poll(command, DEFAULT_POLL);
  let backend = host_from(config, now());
  let controller = Controller::new();
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();

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
          Ingress::none(),
        )
        .await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(LIVENESS_BOUND, || {
        window.get_next_check() == absorbed_line("9999-12-01T00:00:00Z")
      })
      .await;
      stopper.stop();
      handle
        .await
        .expect("serve must not panic on a far-future instruction")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(
    served.controller.frame(false).next_check,
    Some(instant("9999-12-01T00:00:00Z")),
    "SPEC-001/R-28: the instruction is stored and reported as given, whatever the timer does with it"
  );
  assert_eq!(
    invocations(&log),
    1,
    "an instruction eight thousand years out must not have fired again"
  );
}

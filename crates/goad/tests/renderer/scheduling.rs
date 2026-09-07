//! `plan.md` PHASE-02, item 18: the `select!` timer arm, the floor anchor,
//! and the six timed assertions that drive every branch of it but the
//! refusal one (PHASE-03's) — VT-2..VT-8.
//!
//! Every case here drives the production `serve`, the same function
//! `wiring.rs`'s own `mod serving`/`mod interaction`/`mod cancellation` call,
//! against a real child process — there is no second, test-only loop.

use std::path::{Path, PathBuf};
use std::time::Duration;

use goad::controller::{Controller, Ending, serve};
use goad::wire::{Cancel, Command, Stimulus};
use goad_shell::config::{BackendConfig, Command as ShellCommand, Config, ScheduleConfig};
use tokio::sync::mpsc;
use tokio::task::LocalSet;

use crate::driving::{DEFAULT_POLL, host_from, invocations, logging_backend, scripted};
use crate::harness::{
  TIMEOUT, current_view_token, glass_over, now, stub_clock, until, window_and_tray,
};

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
  }
}

/// Like `driving::scripted`, but against `logs-the-request-then-answers.sh`
/// rather than `answers-as-instructed.sh`: the invocation log holds each raw
/// request rather than the literal string `invoked`, which is what VT-3 and
/// VT-7 need to read `event.kind` off (`answers-as-instructed.sh` never reads
/// its own stdin, so it cannot report what it received). Not added to
/// `tests/support/driving.rs` — this target is its only consumer.
fn logging_scripted(case: &str, instructions: &[&str]) -> (ShellCommand, PathBuf) {
  let (mut command, log) = logging_backend("logs-the-request-then-answers", case);
  command.arguments.extend(
    instructions
      .iter()
      .map(|instruction| (*instruction).to_owned()),
  );
  (command, log)
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

/// VT-2 / VT-3 — AC-1, SPEC-002/R-1 and R-5's first half. A `default_poll`
/// (100 ms) shorter than the floor is honoured for the process's **first**
/// scheduled firing, because `floor_until` starts in the past (D-5): the
/// second invocation is the scheduled one, `event.kind == "scheduled"`.
/// *Liveness. Expected ~105 ms against `until(2 s)`, 19x margin.*
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
        serve(backend, controller, rx, cancel, stub_clock, glass).await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(Duration::from_secs(2), || invocations(&log) >= 2).await;
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
/// produces lands well within it. *Liveness. Expected ~105 ms against
/// `until(2 s)`, 19x margin.*
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
        serve(backend, controller, rx, cancel, stub_clock, glass).await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(Duration::from_secs(2), || invocations(&log) >= 2).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(invocations(&log), 2);
}

/// VT-5 — AC-2, from a `respond`. The first response carries a view; the
/// `Choose` naming it is answered with a 100 ms instruction, and the third
/// invocation — the scheduled one — lands well within `until(2 s)` despite
/// the far `default_poll`. The view token is read off the window's own
/// options model, as `harness::current_view_token` does; never minted
/// independently. *Liveness. Expected ~105 ms against `until(2 s)`, 19x
/// margin.*
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
        serve(backend, controller, rx, cancel, stub_clock, glass).await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(Duration::from_secs(2), || {
        window.get_heading() == "Proceed?"
      })
      .await;
      let view = current_view_token(&window).expect("the view must be on screen");
      tx.send(Command::Choose {
        view,
        option: "yes".to_owned(),
      })
      .await
      .expect("the channel must accept the click");
      until(Duration::from_secs(2), || invocations(&log) >= 3).await;
      stopper.stop();
      handle.await.expect("serve must not panic")
    })
    .await;

  assert_eq!(served.ending, Ending::Stopped);
  assert_eq!(invocations(&log), 3);
}

/// VT-6 — AC-3, earlier supersedes. A first instruction of 60 s leaves a far
/// deadline pending; a person-driven `evaluate` then instructs 100 ms, and
/// the scheduled firing lands inside `until(2 s)` — which it cannot do if
/// the pending 60 s deadline had survived (this is the case
/// `max(retained, incoming)` gets wrong, `schedule.rs:204-208`). *Liveness.
/// Expected ~105 ms against `until(2 s)`, 19x margin.*
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
        serve(backend, controller, rx, cancel, stub_clock, glass).await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(Duration::from_secs(2), || invocations(&log) >= 1).await;
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the person-driven send");
      until(Duration::from_secs(2), || invocations(&log) >= 3).await;
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
        serve(backend, controller, rx, cancel, stub_clock, glass).await
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
      until(Duration::from_secs(2), || invocations(&log) >= 2).await;
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
        serve(backend, controller, rx, cancel, stub_clock, glass).await
      });
      tx.send(Command::Evaluate(Stimulus::Requested))
        .await
        .expect("the channel must accept the first send");
      until(Duration::from_secs(2), || invocations(&log) >= 1).await;
      // The exchange completed and the loop has looped back to the top
      // `select!` with an empty channel and a far sleep — parked on the
      // timer arm, the scenario AC-7 is about. A short settle so the loop
      // is genuinely back at that `select!`, not still mid-iteration.
      tokio::time::sleep(Duration::from_millis(20)).await;
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

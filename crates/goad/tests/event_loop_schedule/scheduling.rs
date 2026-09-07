//! `plan.md` PHASE-05, VT-1/VT-2: AC-10, proved in the arrangement it will
//! run in, minus the platform (A-1).
//!
//! Copies `event_loop/closing.rs`'s topology — the whole of `start`'s own
//! composition, minus argument parsing and the process exit code — and adds
//! one thing `closing.rs` has no need of: a second `slint::spawn_local`
//! task that watches the invocation log and trips `Cancel` once the
//! scheduled evaluation has landed, because the test body itself cannot
//! poll while `run_event_loop_until_quit` is running (VT-2 — a version of
//! this test that tried to observe from the test body would simply hang;
//! `research.md` Thread 5's spike S-1 is the prior art for the watcher-task
//! shape).
//!
//! One `#[test]` fn only: `init_integration_test_with_system_time`'s own
//! doc comment says its init "can only be called once per process" (D-12).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use goad::clock::wall_clock;
use goad::controller::{Controller, Ending, serve};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad::install::install;
use goad::wire::{Cancel, Command, Stimulus, Wire};
use goad_shell::backend::process::ProcessBackend;
use goad_shell::config::{BackendConfig, Config, ScheduleConfig};
use goad_shell::host::Host;
use i_slint_backend_testing::init_integration_test_with_system_time;
use slint::{ComponentHandle, VecModel};
use tokio::sync::mpsc;

use crate::scripting::{invocations, scripted};
use crate::waiting::{LIVENESS_BOUND, within};

/// A response with nothing to show and no instruction, so the caller's own
/// `default_poll` governs the next check — the same shape
/// `renderer/scheduling.rs`'s VT-2/VT-3 use for the same reason.
const NOTHING_INSTRUCTED: &str = r#"{"view":null}"#;

/// Short enough to observe well inside `LIVENESS_BOUND`, long enough that a
/// loaded gate cannot mistake scheduling jitter for a missed firing
/// (design.md §9's AC-10 row: ~270 ms measured against the 5 s liveness
/// bound, ~18x).
const DEFAULT_POLL: jiff::SignedDuration = jiff::SignedDuration::from_millis(100);

/// AC-10 (`plan.md` PHASE-05/VT-1). `start`'s own composition, real in every
/// component but the Slint platform (EX-5, EX-6): a multi-thread tokio
/// runtime with its `EnterGuard` held for the loop's life, a real window
/// and tray, `install`'s callback table, a real channel and `Cancel`,
/// `SlintGlass`, `ProcessBackend` against a real child, the production
/// `serve`, driven by `slint::spawn_local` under
/// `init_integration_test_with_system_time()`.
///
/// The first evaluation is dispatched through the ordinary channel; a
/// second, watcher `spawn_local` task waits for the invocation log to show
/// a second invocation — the unprompted, scheduled one `default_poll`
/// produces — and only then trips `Cancel`. `serve` returns
/// `Ending::Stopped`, `quit_event_loop` ends the loop, and
/// `run_event_loop_until_quit` returns. *Liveness: measured ~270 ms against
/// the 5 s liveness bound, ~18x (design.md §9).*
#[test]
fn a_scheduled_evaluation_fires_under_the_production_topology() {
  init_integration_test_with_system_time();

  // Step 2 of `start`: the runtime, entered for the whole of the loop's
  // life — without it the first poll of a `tokio::process` future panics.
  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("a runtime builds in a test process");
  let _entered = runtime.enter();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");

  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let wire = Wire::new(tx.clone(), cancel.clone(), window.as_weak());
  install(&window, &tray, &wire);

  let glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
  );

  let (command, log) = scripted("event-loop-schedule", &[NOTHING_INSTRUCTED]);
  let config = Config {
    backend: BackendConfig {
      command,
      timeout: Duration::from_secs(2),
    },
    schedule: ScheduleConfig {
      default_poll: DEFAULT_POLL,
    },
  };
  let now = wall_clock().expect("the real wall clock reads fine in a test process");
  let backend = ProcessBackend::new(config.backend.command.clone(), config.backend.timeout);
  let host = Host::new(config, backend, now);

  // What the loop ended with, read back on this same thread once
  // `run_event_loop_until_quit` returns — `spawn_local`'s future and this
  // function body share the one thread the testing backend runs on.
  let ending = Rc::new(RefCell::new(None));
  let recorded = Rc::clone(&ending);
  let _serving = slint::spawn_local(async move {
    let served = serve(host, Controller::new(), rx, cancel, wall_clock, glass).await;
    *recorded.borrow_mut() = Some(served.ending);
    // The crate's only `quit_event_loop` call site, mirrored from `main.rs`
    // exactly: matched, not discarded, because `let _ =` trips
    // `let_underscore_must_use`.
    match slint::quit_event_loop() {
      Ok(()) | Err(_) => (),
    }
  })
  .expect("the event loop accepts the host task");

  // The watcher (VT-2's answer to "the test body cannot poll while the
  // event loop is running"): a second task, on the same thread, that
  // dispatches the first evaluation, waits for the scheduled second
  // invocation, and only then stops the loop from inside it.
  //
  // The watcher never panics on the wait. A
  // panic here would unwind inside the Slint event loop's own poll, so
  // `stopper.stop()` would never run, `serve` would never return, its
  // `quit_event_loop` would never be called, and
  // `run_event_loop_until_quit()` on the test thread would have nothing to
  // end it — an indefinitely wedged gate rather than a red one, since
  // `cargo test` imposes no outer timeout. So the wait yields a bool, the
  // loop is stopped either way, and the assertion is made on the test thread
  // after the loop has ended.
  let watcher_log = log.clone();
  let observed = Rc::new(RefCell::new(false));
  let watcher_observed = Rc::clone(&observed);
  let _watching = slint::spawn_local(async move {
    tx.send(Command::Evaluate(Stimulus::Requested))
      .await
      .expect("the channel must accept the first send");
    // Waiting on the invocation log is right here, and the sweep for F-22
    // left it alone deliberately: nothing this target asserts depends on the
    // second exchange having been **absorbed**. Its claims are that the
    // firing happened, that the loop ended, and that the count is 2 — all
    // facts about the log itself. A case asserting what the exchange
    // resolved would have to wait on the rendered next-check line instead
    // (`renderer/scheduling.rs::absorbed_line`).
    *watcher_observed.borrow_mut() =
      within(LIVENESS_BOUND, || invocations(&watcher_log) >= 2).await;
    stopper.stop();
  })
  .expect("the event loop accepts the watcher task");

  slint::run_event_loop_until_quit()
    .expect("the headless testing backend's loop can run, and quit when asked");

  assert!(
    *observed.borrow(),
    "the scheduled evaluation never landed within {LIVENESS_BOUND:?}"
  );
  assert_eq!(
    *ending.borrow(),
    Some(Ending::Stopped),
    "the watcher must have tripped Cancel after observing the scheduled firing"
  );
  assert_eq!(
    invocations(&log),
    2,
    "startup, then exactly one unprompted, scheduled firing"
  );
}

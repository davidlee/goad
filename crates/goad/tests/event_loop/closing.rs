//! Item 14e (design.md §9): the wiring that exists nowhere else. A real
//! close request runs `Wire::stop` and returns `KeepWindowShown`, `serve`
//! returns, then `quit_event_loop` runs and `run_event_loop_until_quit`
//! returns.
//!
//! The whole of `start`'s own composition (§5.4), minus argument parsing and
//! the process exit code: a real window and tray, `install`'s callback
//! table, a real channel, `Cancel`, `SlintGlass`, and the production `serve`
//! — driven by a real, headless Slint event loop rather than `block_on`,
//! which is the one thing `init_no_event_loop()` cannot provide.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use goad::clock::wall_clock;
use goad::controller::{Controller, Ending, serve};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad::install::install;
use goad::wire::{Cancel, Command, Wire};
use goad_shell::backend::process::ProcessBackend;
use goad_shell::config::{BackendConfig, Command as ShellCommand, Config, ScheduleConfig};
use goad_shell::host::Host;
use i_slint_backend_testing::init_integration_test_with_mock_time;
use slint::platform::WindowEvent;
use slint::{ComponentHandle, VecModel};
use tokio::sync::mpsc;

/// `init_integration_test_with_mock_time` "can only be called once per
/// process" (its own doc comment) — which is why this target carries
/// exactly one `#[test]` fn.
#[test]
fn a_real_close_request_ends_serve_and_then_the_loop() {
  init_integration_test_with_mock_time();

  // Step 2 of `start`: the runtime, entered for the whole of the loop's
  // life — without it the first poll of a `tokio::process` future panics.
  // Nothing here spawns a process, but `ProcessBackend::new` is
  // constructed the same way `start` constructs it, and ADR-002/A-3's
  // guard is that this test exercises the real composition rather than a
  // stand-in for it.
  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("a runtime builds in a test process");
  let _entered = runtime.enter();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");

  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let wire = Wire::new(tx, cancel.clone(), window.as_weak());
  install(&window, &tray, &wire);

  let glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
  );

  let config = Config {
    backend: BackendConfig {
      command: ShellCommand::new("true", Vec::new()),
      timeout: Duration::from_secs(2),
    },
    schedule: ScheduleConfig {
      default_poll: jiff::SignedDuration::from_mins(30),
    },
    ingress: None,
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

  // The real close request. No pointer or key event exists for this in
  // `i_slint_backend_testing`'s search API, so it is dispatched directly —
  // `Window::dispatch_event` is the same call a real windowing backend
  // makes on the user's behalf, and it runs the callback `install` wired
  // (`on_close_requested`), then hides the window only if that callback
  // asked for `HideWindow`; ours asks for `KeepWindowShown`.
  let closing_window = window.clone_strong();
  let _closing = slint::spawn_local(async move {
    closing_window
      .window()
      .dispatch_event(WindowEvent::CloseRequested);
  })
  .expect("the event loop accepts the close-dispatch task");

  slint::run_event_loop_until_quit()
    .expect("the headless testing backend's loop can run, and quit when asked");

  assert_eq!(
    *ending.borrow(),
    Some(Ending::Stopped),
    "a close request must trip Cancel, not close the channel"
  );
}

// crates/goad/src/main.rs — stratum 3
//
// **The same deny as `lib.rs`, because a crate root is not a crate**
// (`review-code.md` F-T2). `#![deny(…)]` is an inner attribute on one
// compilation unit, and `crates/goad` has two roots: cargo builds this binary
// as a separate crate that merely *depends* on the library, so `lib.rs`'s own
// `#![deny]` does not reach a line of this file. `Cargo.toml`'s `[lints]
// workspace = true` is exclusive — a package-local `[lints.clippy]` table
// cannot sit beside it — so a crate-root attribute is the mechanism; it was
// simply put in one of the two roots. `lib.rs`'s comment carries the
// reasoning and is not repeated here.
#![deny(clippy::wildcard_enum_match_arm)]

use std::path::Path;
use std::process::ExitCode;
use std::rc::Rc;

use goad::controller::Controller;
use goad::diagnostics;
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad::install::install;
use goad::pending::Debounce;
use goad::startup::{self, Launch, StartupError};
use goad::wire::{Cancel, Command, Notice, Stimulus, Wire};
use goad_shell::backend::process::ProcessBackend;
use goad_shell::clock;
use goad_shell::config::Config;
use goad_shell::host::Host;
use slint::{ComponentHandle, VecModel};
use tokio::sync::mpsc;

fn main() -> ExitCode {
  match run() {
    Ok(()) => ExitCode::SUCCESS,
    Err(error) => {
      diagnostics::report_startup(&error); // "goad: {error}" on stderr
      ExitCode::from(2)
    }
  }
}

/// `main` cannot use `?`, because it returns `ExitCode`. This is the fallible
/// half, and it is the only place a `StartupError` is produced.
fn run() -> Result<(), StartupError> {
  // The environment is passed in, not reached for, because `arguments` is pure
  // over both (§9 item 17). The closure is not redundant and cannot be replaced
  // by `&std::env::var_os`: `var_os` is generic over `K: AsRef<OsStr>`, and a
  // generic fn item does not coerce to `&dyn Fn(&str) -> Option<OsString>`.
  match startup::arguments(std::env::args_os(), &|name| std::env::var_os(name))? {
    Launch::Help => {
      diagnostics::print_usage(); // stdout, and `run` returns Ok
      Ok(())
    }
    Launch::Config(path) => start(&path),
  }
}

fn start(path: &Path) -> Result<(), StartupError> {
  // 1. The config, the clock and the backend, before any UI exists. The
  //    command and timeout are cloned out of the config for the transport;
  //    the config itself is not yet moved — step 3 still needs to read its
  //    `ingress` field before step 4 moves it into the host.
  let config = Config::load(path).map_err(StartupError::Config)?;
  let now = clock::wall_clock().map_err(StartupError::Clock)?;
  let backend = ProcessBackend::new(config.backend.command.clone(), config.backend.timeout);

  // 2. The runtime, entered for the whole of the loop's life. Without the guard
  //    the first poll of a `tokio::process` future on the Slint thread panics
  //    with *there is no reactor running*.
  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .map_err(StartupError::Runtime)?;
  let _entered = runtime.enter(); // dropped after the loop returns

  // 3. The socket the configuration names, bound before any window exists: a
  //    bind failure must be fatal before a person sees anything.
  let ingress = startup::listener(config.ingress.as_ref())?;

  // 4. The host, complete. The config is now moved into it, having been read
  //    for the last time immediately above.
  let host = Host::new(config, backend, now);

  // 5. The components. The app id is set after the first component exists and
  //    before anything is shown: the app icon comes from it and the `icon`
  //    property is silently dropped, but `set_xdg_app_id` does not initialize
  //    the platform on its own — constructing a component is what selects the
  //    backend. Called first it fails with *No default Slint platform was
  //    selected*, which is a startup failure on every run (measured against
  //    slint 1.17.1; the renderer tests install the testing backend themselves,
  //    so nothing in the gate constructs the real one).
  let window = PromptWindow::new().map_err(StartupError::Platform)?;
  slint::set_xdg_app_id("goad").map_err(StartupError::Platform)?;
  let tray = Tray::new().map_err(StartupError::Platform)?;

  // 6. The bridge. One `Wire`, cloned into each callback and nowhere else.
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let notice = Notice::new();
  let wire = Wire::new(tx.clone(), cancel.clone(), notice.clone());
  //    The debounce, created **here** and not inside `install`, because
  //    `SlintGlass` is given a clone of the same handle (step 7): the
  //    callbacks and the glass must share one map, never hold two
  //    (`design.md` §9, R10).
  let pending = Rc::new(Debounce::new());
  install(&window, &tray, &wire, &pending); // the callback table

  // 7. The glass. The `VecModel` is created once and lives for the process;
  //    `present` re-hands its `ModelRc` on every call, so no property has to
  //    survive a hide. `new` also writes the initial tray icon and tooltip,
  //    because the tray registers nothing until a non-empty image is assigned
  //    and the loop's first `present` happens after the event loop starts.
  //
  //    `Rc::clone(&pending)` and **not** a second `Debounce`: the overlay only
  //    overlays where the glass reads the map the callbacks write. Two values
  //    compile, run, and measure nothing (`design.md` §8 R10).
  let glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
    Rc::clone(&pending),
  );

  // 8. The first evaluation enters through the ordinary channel, so item 11
  //    exercises the real path. The channel is empty and holds one, so this
  //    cannot fail; an `Err` is still reported rather than unwrapped.
  tx.try_send(Command::Evaluate(Stimulus::Startup))
    .map_err(|_returned| StartupError::Enqueue)?;

  // 9. One task, one loop, one quit. The `JoinHandle` is bound and dropped:
  //    dropping it does not drop the future, which is why nothing is retained.
  let _task = slint::spawn_local(async move {
    let _served = goad::controller::serve(
      host,
      Controller::new(),
      rx,
      cancel,
      notice,
      clock::wall_clock,
      glass,
      ingress,
    )
    .await;
    // The crate's ONLY `quit_event_loop` call site (F-20). Its `Err` says only
    // that the loop is already gone, and is matched rather than discarded
    // because `let _ =` trips `let_underscore_must_use`.
    match slint::quit_event_loop() {
      Ok(()) | Err(_) => (),
    }
  })
  .map_err(StartupError::EventLoop)?;

  slint::run_event_loop_until_quit().map_err(StartupError::Platform)?;
  Ok(())
}

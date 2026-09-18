//! **The debounce, measured** — design.md §5.1's *the timer delivers one entry
//! and re-arms while the map is not empty*.
//!
//! The design reads the re-arm off the locked source:
//! `start_or_restart_timer` carries the old `being_activated` flag onto the
//! replacement, and `maybe_activate_timers` puts the previous callback back
//! only where the permanent store is still `Empty` — "if not, it means the
//! invoked callback has restarted its own timer with a new callback"
//! (`i-slint-core-1.17.1/timers.rs:328-334`, `:365-370`). The whole delivery
//! rule rests on that, so it is measured here rather than believed.
//!
//! What the case drives is the **real** `edited` callback: `invoke_edited`
//! runs the closure a keystroke runs, so what is timed is the wiring rather
//! than a second spelling of it.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use goad::generated::{FieldEdit, Kind, PromptWindow, Tray};
use goad::install::install;
use goad::pending::Pending;
use goad::wire::{Cancel, Command, Notice, Wire};
use i_slint_backend_testing::init_integration_test_with_system_time;
use slint::SharedString;
use tokio::sync::mpsc;

/// `tests/support/waiting.rs`'s own bound, restated rather than included:
/// the poll loop that file exists to share is not what this case needs, and
/// pulling the module in for one constant leaves its other two items dead.
const LIVENESS_BOUND: Duration = Duration::from_secs(5);

/// Two entries held in one window, and the channel holds one command. The
/// first tick can deliver one of them; the second can only arrive because the
/// timer re-armed itself from inside its own callback.
///
/// The loop is stopped by whichever comes first: both edits arriving, or a
/// stop timer at the liveness bound. **Never by a panic inside a loop task** —
/// one there would never reach the code that stops the loop, so the
/// assertions are outside it and a timed-out run fails on a count of one
/// rather than hanging (`tests/support/waiting.rs`'s own note).
#[test]
fn the_timer_delivers_one_edit_per_tick_and_re_arms_for_the_next() {
  init_integration_test_with_system_time();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  let pending = Pending::new();
  // Capacity one, exactly as `main.rs` builds it: this is the constraint that
  // makes one-per-tick the most the timer can deliver.
  let (sender, mut commands) = mpsc::channel::<Command>(1);
  let wire = Wire::new(sender, Cancel::new(), Notice::new());
  install(&window, &tray, &wire, &pending);

  for field in ["noted", "other"] {
    window.invoke_edited(
      SharedString::from("v1"),
      SharedString::from("morning"),
      SharedString::from(field),
      FieldEdit {
        kind: Kind::Text,
        checked: false,
        text: SharedString::from(field),
      },
    );
  }

  let delivered: Rc<RefCell<Vec<Command>>> = Rc::new(RefCell::new(Vec::new()));
  let collecting = Rc::clone(&delivered);
  slint::spawn_local(async move {
    while collecting.borrow().len() < 2 {
      let Some(command) = commands.recv().await else {
        break;
      };
      collecting.borrow_mut().push(command);
    }
    match slint::quit_event_loop() {
      Ok(()) | Err(_) => (),
    }
  })
  .expect("the collector task must spawn");

  // The only thing standing between a broken re-arm and a hung gate.
  let stop = slint::Timer::default();
  stop.start(
    slint::TimerMode::SingleShot,
    LIVENESS_BOUND,
    || match slint::quit_event_loop() {
      Ok(()) | Err(_) => (),
    },
  );

  let started = std::time::Instant::now();
  slint::run_event_loop_until_quit().expect("the headless loop must run and return");
  let elapsed = started.elapsed();

  let delivered = delivered.borrow();
  assert_eq!(
    delivered.len(),
    2,
    "one entry per tick, and the second only arrives because the timer re-armed \
     itself from inside its own callback: {delivered:?}"
  );
  assert!(
    delivered.iter().all(|command| matches!(
      command,
      Command::Edit { view, option, .. } if view == "v1" && option == "morning"
    )),
    "each carries the view its entry was made on: {delivered:?}"
  );
  assert!(
    elapsed >= Duration::from_millis(150),
    "the first edit waited out the window rather than being sent where it was raised: \
     {elapsed:?}"
  );
  assert_eq!(
    pending.shown("v1", "morning", "noted"),
    None,
    "an enqueued send is what empties the map"
  );
  assert_eq!(pending.shown("v1", "morning", "other"), None);
}

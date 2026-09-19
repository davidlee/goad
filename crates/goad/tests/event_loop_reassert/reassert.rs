//! `plan.md` PHASE-01/**VT-4** — **AC-5**: *a present that changes nothing
//! disturbs nothing.*
//!
//! Two presents carrying the same frame leave the convergence counter and the
//! element-construction counter exactly where they were. Both counters are
//! production markup (`design.md` §7 D15): a test-only copy of the field
//! markup would be a parallel implementation of the thing under test, and what
//! it measured would be the copy's element lifetimes rather than this
//! window's.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use goad::controller::{Controller, Exchanged};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::{Glass, SlintGlass};
use goad_semantics::protocol::canonical::{Timestamp, View, ViewId};
use goad_semantics::protocol::normalize::read_response;
use goad_shell::backend::transport::Captured;
use goad_shell::host::{Outcome, Presented};
use i_slint_backend_testing::init_integration_test_with_system_time;
use slint::{ComponentHandle, VecModel};

/// One option, two `boolean` fields. Two and not one so that "no element was
/// rebuilt" has more than one element to be true of.
const A_FORM: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"read","kind":"boolean","label":"Read"}]}]},"next_check":"45 minutes"}"#;

/// How often the stepper below runs. Long enough that the loop renders between
/// two steps — which is what pumps the change trackers the guard hangs off
/// (`i-slint-core/window.rs:805`) — and short enough that the case is quick.
const STEP: Duration = Duration::from_millis(50);

/// The stop the case cannot be allowed to run without. A predicate that never
/// becomes true must fail on a count rather than wedge the gate, so nothing
/// here panics from inside the loop: the steps record, the loop is quit, and
/// every assertion is made on the test thread
/// (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
const LIVENESS_BOUND: Duration = Duration::from_secs(5);

/// What one reading of the window says. Three numbers rather than two: the
/// epoch is what the guard hangs off, so a case that did not read it could
/// pass with an epoch frozen at zero and no handler ever running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Reading {
  inits: i32,
  reasserts: i32,
  epoch: i32,
}

fn read(window: &PromptWindow) -> Reading {
  Reading {
    inits: window.get_inits(),
    reasserts: window.get_reasserts(),
    epoch: window.get_epoch(),
  }
}

/// A controller with `A_FORM` retained, built from a hand-made `Outcome`
/// rather than by driving a child process: what this case is about is what a
/// second `present` of one frame does, and a scripted backend would cost a
/// runtime and a `LocalSet` to reach the same `Prepared`.
fn retaining() -> Controller {
  let now = Timestamp::new(
    "2026-01-01T00:00:00Z"
      .parse()
      .expect("the fixture instant must parse"),
  );
  let view: View = read_response(A_FORM.as_bytes(), now)
    .expect("the fixture must normalize")
    .value
    .view()
    .expect("the fixture carries a view")
    .clone();

  let mut controller = Controller::new();
  controller.absorb(
    Exchanged::Evaluation,
    Outcome {
      view: Some(Presented {
        view_id: ViewId::new("v1"),
        view,
      }),
      next_check: now,
      discarded: Vec::new(),
      stderr: Captured::default(),
      failure: None,
      cleanup: None,
    },
  );
  controller
}

/// **AC-5.** Present the same frame twice, with the loop running in between,
/// and neither counter moves.
///
/// The four steps are a repeated `slint::Timer` rather than four calls in a
/// row, and that is the whole arrangement: a `changed` handler is run by the
/// property evaluator the loop drives, so two presents made back to back
/// inside one function would leave the guard un-fired and the case green for
/// the wrong reason. Each step hands control back to the loop.
///
/// **`drawn > 0` is not decoration.** A case that asserted "the counters did
/// not move" over a window that drew nothing would pass on an empty form, so
/// the first reading is checked to be a real one before the second is compared
/// to it — and the epoch is checked to have *moved*, because a present that
/// bumped nothing would also leave both counters at rest.
///
/// What tells *the guard found agreement* from *the guard never ran* is the
/// negative control the plan requires (PHASE-01/VT-5), run before this red was
/// believed: with the difference test removed the guard writes on every fire,
/// `reasserts` moves, and this case fails. That is recorded in `notes.md`
/// rather than committed, because a control is a mutation of the production
/// code and not a second test.
#[test]
fn a_second_present_of_the_same_frame_rebuilds_nothing_and_corrects_nothing() {
  init_integration_test_with_system_time();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  let mut glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
  );
  let controller = retaining();

  // Recorded from inside the loop, read on the test thread once
  // `run_event_loop_until_quit` has returned — the two share the one thread
  // the testing backend runs on.
  let readings: Rc<RefCell<Vec<Reading>>> = Rc::new(RefCell::new(Vec::new()));

  let recording = Rc::clone(&readings);
  let stepped = window.clone_strong();
  let stepper = slint::Timer::default();
  let mut step = 0_u8;
  stepper.start(slint::TimerMode::Repeated, STEP, move || {
    step += 1;
    match step {
      // The first present: the rows are written because this glass has shown
      // nothing, so this is where the elements are constructed.
      1 | 3 => glass.present(controller.frame(false)),
      // Read *after* the loop has run the change handlers the present before
      // it armed.
      2 => recording.borrow_mut().push(read(&stepped)),
      4 => {
        recording.borrow_mut().push(read(&stepped));
        quit();
      }
      _ => quit(),
    }
  });

  // The only thing standing between a step that never arrives and a hung gate.
  let stop = slint::Timer::default();
  stop.start(slint::TimerMode::SingleShot, LIVENESS_BOUND, quit);

  slint::run_event_loop_until_quit().expect("the headless loop must run, and quit when asked");

  let readings = readings.borrow();
  let [first, second] = readings.as_slice() else {
    panic!("the stepper must have taken both readings within {LIVENESS_BOUND:?}: {readings:?}");
  };

  assert!(
    first.inits > 0,
    "the window must have drawn something for a survival count to mean anything: {first:?}"
  );
  assert_ne!(
    first.epoch, second.epoch,
    "the second present must have bumped the epoch, or no guard ran at all and \
     the counters below are at rest for the wrong reason: {first:?} then {second:?}"
  );
  assert_eq!(
    second.inits, first.inits,
    "the same view was presented again: not one element may have been rebuilt — \
     {first:?} then {second:?}"
  );
  assert_eq!(
    second.reasserts, first.reasserts,
    "and nothing diverged, so no guard may have written a widget back — \
     {first:?} then {second:?}"
  );
}

/// The crate's `quit_event_loop` spelling, mirrored from `main.rs`: matched
/// rather than discarded, because `let _ =` trips `let_underscore_must_use`.
fn quit() {
  match slint::quit_event_loop() {
    Ok(()) | Err(_) => (),
  }
}

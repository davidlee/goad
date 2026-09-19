//! `plan.md` PHASE-08/**VT-6** — *a person who clears a numeric field to
//! retype is not fought*, and the measurement EX-7 turns on.
//!
//! A `number` field the host holds as `0` is cleared, and a present lands
//! **inside** the 150 ms debounce window, before the host has recorded
//! anything. The widget must still be empty afterwards, and the convergence
//! counter must still be at zero: writing `"0"` back over an empty field is
//! the defect `numeric_guard.rs` measured before this slice had an overlay,
//! and it is the whole of what the guard's one exception was licensed by
//! (`design.md` §5.2, §8 R4, §9 A-2).
//!
//! **What makes this a measurement and not a re-statement.** The case as the
//! tree ships passes for a reason that has nothing to do with the exception: a
//! cleared field is a pending `AdjustedText("")`, `interpret` reads it as the
//! text `""` beside the number the field already held, and the channel and the
//! widget then agree as strings on their own. EX-7 asks whether that is
//! *actually* so, by running this case again with the exception deleted. The
//! two readings below are what make either run legible, and the injection pass
//! in `notes.md` records all four corners — overlay and exception, each
//! present and absent.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use goad::controller::{Controller, Exchanged};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::{Glass, SlintGlass};
use goad::install::install;
use goad::pending::Debounce;
use goad::wire::{Cancel, Command, Notice, Wire};
use goad_semantics::protocol::canonical::{Timestamp, View, ViewId};
use goad_semantics::protocol::normalize::read_response;
use goad_shell::backend::transport::Captured;
use goad_shell::host::{Outcome, Presented};
use i_slint_backend_testing::{ElementQuery, init_integration_test_with_system_time};
use slint::{ComponentHandle, VecModel};
use tokio::sync::mpsc;

/// One option, **two unbounded `number`** fields. Unbounded so that
/// `slider_bounds` refuses them and the numeric text control is drawn, and so
/// that what each is drawn showing is the as-drawn zero — the value the
/// guard's exception names.
///
/// Two, because the case makes two contrasting claims about the same widget
/// state and each needs a field the other has not already moved: `counted` is
/// cleared while the host is listening, `also` while it is not.
const TWO_NUMBER_FIELDS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"counted","kind":"number","label":"How many?"},{"id":"also","kind":"number","label":"And then?"}]}]},"next_check":"45 minutes"}"#;

/// What the host holds the field as before anybody touches it —
/// `view_model::spelled` of the as-drawn zero. Written out rather than
/// imported, because a case that read the production spelling would agree with
/// it by construction and could not see the two diverge.
const AS_DRAWN: &str = "0";

/// How often the stepper runs. Long enough that the loop renders between two
/// steps — which is what pumps the change trackers the guard hangs off
/// (`i-slint-core/window.rs:805`, `event_loop_reassert`) — and short enough
/// that a present lands well inside the 150 ms debounce window.
const STEP: Duration = Duration::from_millis(50);

/// The stop the case cannot be allowed to run without. A predicate that never
/// becomes true must fail on a count rather than wedge the gate, so nothing
/// here panics from inside the loop: the steps record, the loop is quit, and
/// every assertion is made on the test thread
/// (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
const LIVENESS_BOUND: Duration = Duration::from_secs(5);

/// The step from which the host is handed nothing. The stepper goes on
/// draining, so the capacity-1 channel never fills and the entry-leaves-on-
/// enqueue rule is exercised rather than sidestepped; what stops is the
/// `Controller::edit` that would record it. That is *the host did **not**
/// record this*, and it is the only thing that tells it from *not yet*.
const SILENT: u8 = 11;

/// The last step. 17 × 50 ms = 850 ms against a 150 ms debounce, which leaves
/// every tick several steps of room on either side: a tick that is merely late
/// is not read as a tick that never came.
const STEPS: u8 = 17;

/// What one reading says: what the widget is **showing**, what the draft holds
/// for it, and the two instrument counters.
///
/// Both halves are needed and neither implies the other. The screen is where a
/// write-back is visible; the draft is what would reach a backend. A case
/// reading only the draft would pass with the widget blanked, and one reading
/// only the screen would pass with nothing recorded anywhere.
#[derive(Debug, Clone, PartialEq)]
struct Reading {
  shown: String,
  also_shown: String,
  drafted: Option<f64>,
  inits: i32,
  reasserts: i32,
}

fn read(window: &PromptWindow, controller: &Controller, view: &str) -> Reading {
  Reading {
    shown: shown(window, "counted"),
    also_shown: shown(window, "also"),
    drafted: drafted(controller, view, "counted"),
    inits: window.get_inits(),
    reasserts: window.get_reasserts(),
  }
}

/// One field's control, by the identity every tier selects on (R-14).
fn control(window: &PromptWindow, field: &str) -> i_slint_backend_testing::ElementHandle {
  let described = field.to_owned();
  ElementQuery::from_root(window)
    .match_predicate(move |element| element.accessible_description().as_deref() == Some(&described))
    .find_first()
    .unwrap_or_else(|| panic!("no control described {field:?}"))
}

/// What the **screen** shows for one field — the control's own
/// `accessible-value`, which the widget binds two-way to its `text`
/// (`fluent/lineedit.slint:13`).
fn shown(window: &PromptWindow, field: &str) -> String {
  control(window, field)
    .accessible_value()
    .unwrap_or_else(|| panic!("{field} declares no accessible-value"))
    .to_string()
}

/// Clear the field the way a person does when they mean to retype it.
///
/// `set_accessible_value` assigns the widget's `text` and calls `edited` from
/// inside the markup (`fluent/lineedit.slint:16`), reaching no `TextInput`
/// insertion logic — which is exactly the shape a paste has, and is why
/// `input-type: decimal` is no part of what reaches the host.
fn clear(window: &PromptWindow, field: &str) {
  control(window, field).set_accessible_value("");
}

/// What the draft holds for the field, read back through `answer` — the only
/// reader of the draft this crate exposes. `R-57` makes a `number`'s value a
/// JSON number, so this is the number and never the text beside it.
fn drafted(controller: &Controller, view: &str, field: &str) -> Option<f64> {
  let (_, answer) = controller.answer(view, "morning").ok()?;
  answer
    .values
    .iter()
    .find(|(id, _)| id.as_str() == field)
    .and_then(|(_, value)| value.as_f64())
}

/// A controller with [`TWO_NUMBER_FIELDS`] retained, built from a hand-made
/// `Outcome` rather than by driving a child process: what this case is about
/// is what a present writes, and a scripted backend would cost a runtime and a
/// `LocalSet` to reach the same `Prepared`.
fn retaining() -> Controller {
  let now = Timestamp::new(
    "2026-01-01T00:00:00Z"
      .parse()
      .expect("the fixture instant must parse"),
  );
  let view: View = read_response(TWO_NUMBER_FIELDS.as_bytes(), now)
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

/// **PHASE-08/VT-6.** A present inside the debounce window leaves a cleared
/// numeric field cleared.
#[test]
fn a_present_inside_the_window_does_not_write_a_zero_back_over_a_cleared_field() {
  init_integration_test_with_system_time();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  // A shown window clips, and left at its preferred size a form of several
  // fields is unreachable to the query — the same reason every `serve`-driven
  // renderer case declares a size.
  ComponentHandle::window(&window).set_size(slint::PhysicalSize::new(600, 600));

  // **Capacity one, as production has it**, and one `Debounce` created before
  // both readers. `Rc::clone` into the glass and a borrow into `install`: two
  // `Debounce` values compile, run, and leave every assertion below green
  // while measuring nothing (`design.md` §8 R10).
  let (tx, mut rx) = mpsc::channel::<Command>(1);
  let wire = Wire::new(tx, Cancel::new(), Notice::new());
  let pending = Rc::new(Debounce::new());
  install(&window, &tray, &wire, &pending);
  let mut glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
    Rc::clone(&pending),
  );

  let mut controller = retaining();
  let view = controller
    .frame(false)
    .shown
    .expect("the fixture must retain a view")
    .view_id
    .as_str()
    .to_owned();

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

    // Every step drains whatever the timer has enqueued and — until [`SILENT`]
    // — applies it exactly as `serve`'s `Command::Edit` arm does. The drain
    // never stops, so the channel never fills and no send is ever refused;
    // what stops is the recording.
    while let Ok(command) = rx.try_recv() {
      if let Command::Edit {
        view: on,
        option,
        field,
        reported,
      } = command
        && step < SILENT
      {
        let _refused = controller.edit(&on, &option, &field, &reported);
      }
    }

    // **The run, at 50 ms a step.** What matters is when each action happens
    // relative to the 150 ms window, so the schedule is written here once.
    //
    // ```
    //  1  present         the form is built; `inits` has something to count
    //  2  read            both fields are drawn showing the as-drawn zero
    //  3  clear `counted` the window opens → the tick is due at step 6
    //  4  present         inside the window, before the tick: the entry is held
    //  5  read            a step after the present, because the guard is a
    //                       `changed` handler and runs when the loop renders
    //  9  present         after the tick, with the entry recorded
    // 10  read
    // 11  clear `also`    `SILENT` has passed, so this one is drained and
    //                       dropped → its tick is due at step 14
    // 15  present         the entry left the map on the enqueue and the host
    //                       never recorded it, so this present must correct it
    // 16  read
    // 17  quit
    // ```
    match step {
      3 => clear(&stepped, "counted"),
      11 => clear(&stepped, "also"),
      2 | 5 | 10 | 16 => recording
        .borrow_mut()
        .push(read(&stepped, &controller, &view)),
      1 | 4 | 9 | 15 => glass.present(controller.frame(false)),
      _ => {}
    }

    if step >= STEPS {
      quit();
    }
  });

  // The only thing standing between a step that never arrives and a hung gate.
  let stop = slint::Timer::default();
  stop.start(slint::TimerMode::SingleShot, LIVENESS_BOUND, quit);

  slint::run_event_loop_until_quit().expect("the headless loop must run, and quit when asked");

  let readings = readings.borrow();
  let [drawn, held, recorded, corrected] = readings.as_slice() else {
    panic!("the stepper must have taken all four readings within {LIVENESS_BOUND:?}: {readings:?}");
  };

  // **The form was drawn**, or every count below is at rest for the wrong
  // reason and `reasserts == 0` is a fact about an empty window.
  assert!(
    drawn.inits > 0,
    "the window must have drawn something for a survival count to mean anything: {drawn:?}"
  );
  assert_eq!(
    (
      drawn.shown.as_str(),
      drawn.also_shown.as_str(),
      drawn.drafted
    ),
    (AS_DRAWN, AS_DRAWN, Some(0.0)),
    "before anyone touches them both fields show what they submit, so `the held \
     number is zero` is a fact about them and not an assumption: {drawn:?}"
  );

  // --- the host has not recorded this *yet* ---

  assert_eq!(
    held.shown, "",
    "a present inside the debounce window leaves the cleared field cleared — writing \
     `\"0\"` back here is the defect this case exists for: {held:?}"
  );
  assert_eq!(
    held.reasserts, 0,
    "and it writes nothing back, so nothing was corrected and then corrected again: \
     {held:?}"
  );

  // --- the host has recorded it ---

  assert_eq!(
    recorded.shown, "",
    "and once the tick has delivered the entry the field is still cleared: {recorded:?}"
  );
  assert_eq!(
    recorded.drafted,
    Some(0.0),
    "with the number the field already held standing, because `\"\"` is not a finite \
     parse and the host repairs nothing: {recorded:?}"
  );
  assert_eq!(
    recorded.reasserts, 0,
    "the convergence counter has stayed at zero across the whole run: {recorded:?}"
  );
  assert_eq!(
    recorded.inits, drawn.inits,
    "and no element was destroyed to do it: the view_id never changed, so the rows \
     were never rebuilt: {drawn:?} then {recorded:?}"
  );

  // --- the host did *not* record this ---
  //
  // **The claim that keeps the exception deleted.** `also` was cleared after
  // [`SILENT`], so its entry left the map on the enqueue and the host never
  // recorded it: the channel carries what `also` was drawn showing, the widget
  // is empty, and the two disagree for the one reason the guard exists —
  // AC-6, *a dropped edit is corrected, element preserved*.
  //
  // The guard's old exception would suppress exactly this convergence, because
  // the widget is empty and the number the channel holds is zero. So the
  // exception is not merely dead under the overlay; under the overlay it is
  // **wrong**, and this reading is what fails if anybody restores it.

  assert_eq!(
    corrected.also_shown, AS_DRAWN,
    "an edit the host never recorded is corrected on the next present, even though \
     the widget is empty and the held number is zero: {corrected:?}"
  );
  assert_eq!(
    corrected.shown, "",
    "and only that field — the one beside it, whose clear the host *did* record, is \
     left exactly as the person left it: {corrected:?}"
  );
  assert_eq!(
    corrected.reasserts, 1,
    "exactly one convergence in the whole run, and it is that one: {corrected:?}"
  );
  assert_eq!(
    corrected.inits, drawn.inits,
    "still with no element destroyed: {drawn:?} then {corrected:?}"
  );
}

/// The crate's `quit_event_loop` spelling, mirrored from `main.rs`: matched
/// rather than discarded, because `let _ =` trips `let_underscore_must_use`.
fn quit() {
  match slint::quit_event_loop() {
    Ok(()) | Err(_) => (),
  }
}

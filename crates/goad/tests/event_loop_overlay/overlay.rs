//! `plan.md` PHASE-06/**VT-1**, **VT-2** and **VT-3** — **AC-6**: *a refused or
//! dropped edit is corrected, element preserved.*
//!
//! Two contrasting claims in one `#[test]` fn, which is the working limit for a
//! loop target, and they are the two halves of what this phase exists to
//! separate (`design.md` §5.3):
//!
//! - **the host has not recorded this *yet*.** While an entry is held, a
//!   present shows what the person typed. Three presents make it, one inside
//!   the debounce window before any tick, one between the two ticks — where one
//!   field's value comes off the draft and the other off the map — and one
//!   after both. The convergence counter stays at **zero** across all three,
//!   and both values reach the draft (**VT-3**). **VT-2** is this claim's
//!   negative control: with the overlay removed the first present reverts both
//!   widgets and this reading fails.
//! - **the host did *not* record this.** From [`SILENT`] on, the stepper drains
//!   the channel and hands the controller nothing. A fourth keystroke is held,
//!   ticked out, and **leaves** the map on the enqueue — and the present after
//!   it puts the draft's older value back, counting exactly one convergence and
//!   constructing no element (**VT-1**).
//!
//! The order is the injection's: the assertions run first-reading first, so an
//! injection aimed at the second claim is not masked by the first. Removing
//! `pending.rs`'s `if enqueued` clear reddens the last reading alone; removing
//! the overlay reddens the first.

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

/// One option, two `text` fields. Two, because the claim *one field's value
/// came off the draft and the other off the map* needs two fields to be true
/// of, and because a single entry cannot tell a tick from a tick and a re-arm.
const TWO_TEXT_FIELDS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"noted","kind":"text","label":"Anything to add?"},{"id":"also","kind":"text","label":"And then?"}]}]},"next_check":"45 minutes"}"#;

/// What the person types, and then types again. The third text matters: the
/// last reading asserts the widget holds the **first** one, which a revert to
/// an empty field would also satisfy if the field had never been typed into.
const FIRST: &str = "walked before breakfast";
const SECOND: &str = "and again after";
const THIRD: &str = "and a second thought";

/// How often the stepper runs. Long enough that the loop renders between two
/// steps — which is what pumps the change trackers the guard hangs off
/// (`i-slint-core/window.rs:805`, `event_loop_reassert`) — and short enough
/// that two keystrokes land inside one 150 ms debounce window.
const STEP: Duration = Duration::from_millis(50);

/// The step from which the host is handed nothing. The stepper goes on
/// draining, so the capacity-1 channel never fills and the entry-leaves-on-
/// enqueue rule is exercised rather than sidestepped; what stops is the
/// `Controller::edit` that would record it. That is *the host did not record
/// this*, and it is the only thing that tells it from *not yet*.
const SILENT: u8 = 14;

/// The stop the case cannot be allowed to run without. A predicate that never
/// becomes true must fail on a count rather than wedge the gate, so nothing
/// here panics from inside the loop: the steps record, the loop is quit, and
/// every assertion is made on the test thread
/// (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
const LIVENESS_BOUND: Duration = Duration::from_secs(5);

/// The last step. 21 × 50 ms = 1050 ms against a 150 ms debounce, which leaves
/// every tick several steps of room: a tick that is merely late is not read as
/// a tick that never came.
const STEPS: u8 = 21;

/// What one reading says: what each widget is **showing**, what the draft
/// holds for it, and the two instrument counters.
///
/// Both halves are needed and neither implies the other. The screen is where a
/// revert is visible; the draft is what would reach a backend. A case reading
/// only the draft would pass with every widget blanked, and one reading only
/// the screen would pass with nothing recorded anywhere.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Reading {
  noted_shown: String,
  also_shown: String,
  noted_drafted: Option<String>,
  also_drafted: Option<String>,
  inits: i32,
  reasserts: i32,
}

fn read(window: &PromptWindow, controller: &Controller, view: &str) -> Reading {
  Reading {
    noted_shown: shown(window, "noted"),
    also_shown: shown(window, "also"),
    noted_drafted: drafted(controller, view, "noted"),
    also_drafted: drafted(controller, view, "also"),
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

/// What the **screen** shows for one text field — the control's own
/// `accessible-value`, which the widget binds two-way to its `text`
/// (`fluent/lineedit.slint:13`).
fn shown(window: &PromptWindow, field: &str) -> String {
  control(window, field)
    .accessible_value()
    .unwrap_or_else(|| panic!("{field} declares no accessible-value"))
    .to_string()
}

/// Type into one field the way `set_accessible_value` does — it assigns the
/// widget's `text` and calls `edited` from inside the markup
/// (`widgets/fluent/lineedit.slint:16`), reaching no `TextInput` insertion
/// logic, which is exactly the shape a paste has.
fn type_into(window: &PromptWindow, field: &str, text: &str) {
  control(window, field).set_accessible_value(text);
}

/// What the draft holds for one of `morning`'s fields, read back through
/// `answer` — the only reader of the draft this crate exposes.
fn drafted(controller: &Controller, view: &str, field: &str) -> Option<String> {
  let (_, answer) = controller.answer(view, "morning").ok()?;
  answer
    .values
    .iter()
    .find(|(id, _)| id.as_str() == field)
    .and_then(|(_, value)| value.as_str())
    .map(ToOwned::to_owned)
}

/// A controller with `TWO_TEXT_FIELDS` retained, built from a hand-made
/// `Outcome` rather than by driving a child process: what this case is about
/// is what a present writes, and a scripted backend would cost a runtime and a
/// `LocalSet` to reach the same `Prepared`.
fn retaining() -> Controller {
  let now = Timestamp::new(
    "2026-01-01T00:00:00Z"
      .parse()
      .expect("the fixture instant must parse"),
  );
  let view: View = read_response(TWO_TEXT_FIELDS.as_bytes(), now)
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

/// **PHASE-06/VT-1, VT-2 and VT-3.** A present inside the debounce window
/// shows what the person typed; a present after the entry has left puts the
/// draft's value back.
#[test]
fn a_present_shows_a_held_edit_and_corrects_one_the_host_never_recorded() {
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

    // Every step drains whatever the timer has enqueued, and — until
    // [`SILENT`] — applies it exactly as `serve`'s `Command::Edit` arm does.
    // The drain never stops, so the channel never fills and no send is ever
    // refused; what stops is the recording.
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

    // **The run, at 50 ms a step.** Three actions, and what matters is when
    // each happens relative to the 150 ms window — so the schedule is written
    // here once rather than spelled across the arms, which do the same three
    // things at different times.
    //
    // ```
    //  1  present          the form is built; `inits` has something to count
    //  2  type `noted`     the window opens
    //  3  type `also`      and restarts, so both are held inside one window
    //                        → tick 1 due at step 6, tick 2 (the re-arm) at 9
    //  4  present          inside the window, before any tick: both values
    //                        come off the map
    //  5  read             a step after the present, because the guard is a
    //                        `changed` handler and runs when the loop renders
    //  7  present          between the ticks: one value off the draft, one
    //                        still off the map
    //  8  read
    // 12  present          after both ticks: both values off the draft
    // 13  read
    // 14  type `noted`     a fourth keystroke the host is never told about,
    //                        because `SILENT` has passed → tick 3 at step 17
    // 20  present          the entry left the map on the enqueue, so this one
    //                        corrects the widget
    // 21  read, and quit
    // ```
    match step {
      2 => type_into(&stepped, "noted", FIRST),
      3 => type_into(&stepped, "also", SECOND),
      14 => type_into(&stepped, "noted", THIRD),
      5 | 8 | 13 | 21 => recording
        .borrow_mut()
        .push(read(&stepped, &controller, &view)),
      1 | 4 | 7 | 12 | 20 => glass.present(controller.frame(false)),
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
  let [inside, between, after, corrected] = readings.as_slice() else {
    panic!("the stepper must have taken all four readings within {LIVENESS_BOUND:?}: {readings:?}");
  };

  // **The form was drawn**, or every count below is at rest for the wrong
  // reason and `reasserts == 0` is a fact about an empty window.
  assert!(
    inside.inits > 0,
    "the window must have drawn something for a survival count to mean anything: {inside:?}"
  );

  // --- the host has not recorded this *yet* (VT-3; VT-2 is its control) ---

  assert_eq!(
    (inside.noted_shown.as_str(), inside.also_shown.as_str()),
    (FIRST, SECOND),
    "a present inside the debounce window shows what the person typed, from the map: {inside:?}"
  );
  assert_eq!(
    inside.reasserts, 0,
    "and it writes nothing back, because the channel already agrees with the widget: {inside:?}"
  );
  assert_eq!(
    (between.noted_shown.as_str(), between.also_shown.as_str()),
    (FIRST, SECOND),
    "between the two ticks one field's value is the draft's and the other is still the \
     map's, and neither widget moves: {between:?}"
  );
  assert_eq!(
    between.reasserts, 0,
    "still nothing written back: {between:?}"
  );
  assert_eq!(
    (
      after.noted_drafted.as_deref(),
      after.also_drafted.as_deref()
    ),
    (Some(FIRST), Some(SECOND)),
    "two ticks and no answer, and everything typed inside the one window reached the \
     draft: {after:?}"
  );
  assert_eq!(
    (after.noted_shown.as_str(), after.also_shown.as_str()),
    (FIRST, SECOND),
    "with both values now the draft's, the widgets still show them: {after:?}"
  );
  assert_eq!(
    after.reasserts, 0,
    "and the convergence counter has stayed at zero across both ticks — no widget was \
     reverted at any point while the host was catching up: {after:?}"
  );

  // --- the host did *not* record this (VT-1) ---

  assert_eq!(
    corrected.noted_drafted.as_deref(),
    Some(FIRST),
    "the fourth keystroke was drained and dropped, so the draft still holds what it \
     held: {corrected:?}"
  );
  assert_eq!(
    corrected.noted_shown.as_str(),
    FIRST,
    "and the entry having left the map on the enqueue, the present puts the draft's \
     value back on the screen: {corrected:?}"
  );
  assert_eq!(
    corrected.reasserts, 1,
    "exactly one convergence — the field the host never recorded, and not the one \
     beside it: {corrected:?}"
  );
  assert_eq!(
    corrected.inits, inside.inits,
    "and no element was destroyed to do it: the view_id never changed, so the rows were \
     never rebuilt: {inside:?} then {corrected:?}"
  );
}

/// The crate's `quit_event_loop` spelling, mirrored from `main.rs`: matched
/// rather than discarded, because `let _ =` trips `let_underscore_must_use`.
fn quit() {
  match slint::quit_event_loop() {
    Ok(()) | Err(_) => (),
  }
}

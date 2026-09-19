//! `plan.md` PHASE-05/**VT-5** and **VT-6** — *the timer delivers, one entry
//! per tick, and re-arms while the map is not empty.*
//!
//! Two contrasting claims in one `#[test]` fn, which is the working limit for
//! a loop target (`plan.md` PHASE-09's note): after the first tick **exactly
//! one** edit has reached the controller, and after the second **both** have —
//! the second reachable only through the re-arm. VT-6 is the negative control
//! that keeps the second honest: with the re-arm removed the first claim still
//! passes and the second fails.

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

/// One option, two `text` fields. Two, because one entry cannot tell *the
/// timer delivered* from *the timer delivered and re-armed*.
const TWO_TEXT_FIELDS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"noted","kind":"text","label":"Anything to add?"},{"id":"also","kind":"text","label":"And then?"}]}]},"next_check":"45 minutes"}"#;

/// How often the stepper runs. Well under the 150 ms debounce, so both fields
/// are typed into inside **one** window and the step that reads after a tick
/// is not the step that armed it.
const STEP: Duration = Duration::from_millis(25);

/// The stop the case cannot be allowed to run without. A predicate that never
/// becomes true must fail on a count rather than wedge the gate, so nothing
/// here panics from inside the loop: the steps record, the loop is quit, and
/// every assertion is made on the test thread
/// (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
const LIVENESS_BOUND: Duration = Duration::from_secs(5);

/// Long enough for two debounce windows and their ticks, at [`STEP`] each.
/// 30 × 25 ms = 750 ms against a 150 ms debounce — five windows' worth, so a
/// tick that is merely late is not read as a tick that never came.
const STEPS: u8 = 30;

/// A controller with `TWO_TEXT_FIELDS` retained, built from a hand-made
/// `Outcome` rather than by driving a child process: what this case is about
/// is what the timer delivers, and a scripted backend would cost a runtime and
/// a `LocalSet` to reach the same `Prepared`. `event_loop_reassert::retaining`
/// is the precedent and this is the same shape.
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

/// Type into one field the way `set_accessible_value` does — it assigns the
/// widget's `text` and calls `edited` from inside the markup
/// (`widgets/fluent/lineedit.slint:16`), reaching no `TextInput` insertion
/// logic, which is exactly the shape a paste has.
fn type_into(window: &PromptWindow, field: &str, text: &str) {
  let described = field.to_owned();
  ElementQuery::from_root(window)
    .match_predicate(move |element| element.accessible_description().as_deref() == Some(&described))
    .find_first()
    .unwrap_or_else(|| panic!("no control described {field:?}"))
    .set_accessible_value(text);
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

/// What one reading of the run says: how many `Command::Edit`s the controller
/// has been handed, and what the draft holds for each field.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Reading {
  handled: usize,
  noted: Option<String>,
  also: Option<String>,
}

/// **PHASE-05/VT-5 and VT-6.** Two text fields typed into inside one debounce
/// window: the timer delivers one edit, re-arms, and delivers the other.
///
/// The two readings are taken at fixed steps either side of the second tick,
/// and both are asserted, because they are the two halves of the delivery
/// rule and an injection aimed at one must not be masked by the other:
///
/// - **one per tick.** The command channel holds one (`main.rs:86`) and
///   `serve` shares the UI thread, so one command per tick is the most that is
///   ever available. A timer that sent both entries at once would fail the
///   first reading.
/// - **and it re-arms.** A `slint::Timer` may be restarted from inside its own
///   callback (`i-slint-core-1.17.1/timers.rs:348-372`), and the second entry
///   is reachable by no other route: nothing here answers, so there is no
///   `Choose` to flush it. **VT-6** is that claim's negative control — with
///   the re-arm removed, the second reading holds one edit and this case goes
///   red while the first reading still passes.
///
/// The draft is read through `answer`, so what is asserted is a value that
/// would reach a backend rather than a private field: a delivery that reached
/// the controller and recorded nothing would fail here.
#[test]
fn the_timer_delivers_one_edit_per_tick_and_re_arms_while_the_map_is_not_empty() {
  init_integration_test_with_system_time();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  // A shown window clips, and left at its preferred size a form of several
  // fields is unreachable to the query — the same reason every `serve`-driven
  // renderer case declares a size.
  ComponentHandle::window(&window).set_size(slint::PhysicalSize::new(600, 600));

  let mut glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
  );
  let mut controller = retaining();
  let view = controller
    .frame(false)
    .shown
    .expect("the fixture must retain a view")
    .view_id
    .as_str()
    .to_owned();

  // **Capacity one, as production has it.** The timer's send therefore
  // competes with nothing, and a second send made before the stepper drains
  // would come back `Full` — which is the case the entry-leaves-on-enqueue
  // rule exists for and which the delivery rule must not depend on.
  let (tx, mut rx) = mpsc::channel::<Command>(1);
  let wire = Wire::new(tx, Cancel::new(), Notice::new());
  let pending = Rc::new(Debounce::new());
  // The production callback table, so what is driven below is the `edited`
  // closure `main.rs` installs and not a second copy of it.
  install(&window, &tray, &wire, &pending);

  // Recorded from inside the loop, read on the test thread once
  // `run_event_loop_until_quit` has returned — the two share the one thread
  // the testing backend runs on.
  let readings: Rc<RefCell<Vec<Reading>>> = Rc::new(RefCell::new(Vec::new()));
  let handled: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));

  let recording = Rc::clone(&readings);
  let counting = Rc::clone(&handled);
  let stepped = window.clone_strong();
  let stepper = slint::Timer::default();
  let mut step = 0_u8;
  stepper.start(slint::TimerMode::Repeated, STEP, move || {
    step += 1;

    // Every step drains whatever the timer has enqueued and applies it exactly
    // as `serve`'s `Command::Edit` arm does — the walk, the interpretation and
    // the record. A refusal is counted as handled all the same: what is under
    // test is what the timer delivered, and a refused delivery would show up
    // as a draft that never changed.
    while let Ok(command) = rx.try_recv() {
      if let Command::Edit {
        view: named,
        option,
        field,
        reported,
      } = command
      {
        *counting.borrow_mut() += 1;
        let _refused = controller.edit(&named, &option, &field, &reported);
      }
    }

    match step {
      // The first present builds the form, so the two controls exist to be
      // typed into.
      1 => glass.present(controller.frame(false)),
      // Both fields inside one 150 ms window: at 25 ms a step, step 3 is 25 ms
      // after step 2 and the second keystroke restarts the one timer.
      2 => type_into(&stepped, "noted", "walked before breakfast"),
      3 => type_into(&stepped, "also", "and again after"),
      // Past the first tick (150 ms after step 3, which is step 9) and well
      // short of the second.
      11 => recording.borrow_mut().push(Reading {
        handled: *counting.borrow(),
        noted: drafted(&controller, &view, "noted"),
        also: drafted(&controller, &view, "also"),
      }),
      // Past the second tick, which only the re-arm can have produced.
      _ if step >= STEPS => {
        recording.borrow_mut().push(Reading {
          handled: *counting.borrow(),
          noted: drafted(&controller, &view, "noted"),
          also: drafted(&controller, &view, "also"),
        });
        quit();
      }
      _ => {}
    }
  });

  // The only thing standing between a step that never arrives and a hung gate.
  let stop = slint::Timer::default();
  stop.start(slint::TimerMode::SingleShot, LIVENESS_BOUND, quit);

  slint::run_event_loop_until_quit().expect("the headless loop must run, and quit when asked");

  let readings = readings.borrow();
  let [after_one, after_two] = readings.as_slice() else {
    panic!("the stepper must have taken both readings within {LIVENESS_BOUND:?}: {readings:?}");
  };

  assert_eq!(
    after_one.handled, 1,
    "one entry per tick, and nothing answered — the channel holds one, so a timer that \
     sent both at once could not have been delivered anyway: {after_one:?}"
  );
  // **Which** of the two arrives first is not asserted, and must not be: no
  // order is promised over the map and none is needed, because the keys are
  // distinct by construction. What is asserted is that exactly one field has
  // moved off its as-drawn `""`, and that the one that moved carries **its
  // own** text — a delivery that reached the draft under the wrong key would
  // satisfy the count and fail this.
  assert_eq!(
    [after_one.noted.as_deref(), after_one.also.as_deref()]
      .into_iter()
      .filter(|held| held.is_some_and(|text| !text.is_empty()))
      .count(),
    1,
    "one tick, one field off its as-drawn value: {after_one:?}"
  );
  assert!(
    matches!(
      after_one.noted.as_deref(),
      Some("" | "walked before breakfast")
    ),
    "whatever reached `noted` is what was typed into `noted`: {after_one:?}"
  );
  assert!(
    matches!(after_one.also.as_deref(), Some("" | "and again after")),
    "and whatever reached `also` is what was typed into `also`: {after_one:?}"
  );

  assert_eq!(
    after_two.handled, 2,
    "the timer re-armed while the map was not empty, and nothing else could have \
     delivered the second: no answer was given, so there was no `Choose` to flush it — \
     {after_one:?} then {after_two:?}"
  );
  assert_eq!(
    (after_two.noted.as_deref(), after_two.also.as_deref()),
    (Some("walked before breakfast"), Some("and again after")),
    "and everything typed inside the one window reached the draft: {after_two:?}"
  );
}

/// The crate's `quit_event_loop` spelling, mirrored from `main.rs`: matched
/// rather than discarded, because `let _ =` trips `let_underscore_must_use`.
fn quit() {
  match slint::quit_event_loop() {
    Ok(()) | Err(_) => (),
  }
}

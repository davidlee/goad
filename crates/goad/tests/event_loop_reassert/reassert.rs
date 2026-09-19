//! **Two contrasting claims about the same counter**, in one arrangement.
//!
//! `plan.md` PHASE-01/**VT-4** — **AC-5**: *a present that changes nothing
//! disturbs nothing.* Two presents carrying the same frame leave the
//! convergence counter and the element-construction counter exactly where they
//! were.
//!
//! `plan.md` PHASE-09/**VT-8** — **A-5**: and a present that *does* change
//! something moves it. A `ComboBox` is driven with no `Wire` installed, so the
//! host never learns of the choice; the next present writes the draft's index
//! back and the widget converges on it.
//!
//! The two are contrasting on purpose — one says the counter stays at rest and
//! the other says it can move — so neither passes for the other's reason, and
//! an injection aimed at one is not masked by the other. AC-5 is measured
//! first, on a window nothing has touched.
//!
//! Both counters are production markup (`design.md` §7 D15): a test-only copy
//! of the field markup would be a parallel implementation of the thing under
//! test, and what it measured would be the copy's element lifetimes rather
//! than this window's.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use goad::controller::{Controller, Exchanged};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::{Glass, SlintGlass};
use goad::pending::Debounce;
use goad_semantics::protocol::canonical::{Timestamp, View, ViewId};
use goad_semantics::protocol::normalize::read_response;
use goad_shell::backend::transport::Captured;
use goad_shell::host::{Outcome, Presented};
use i_slint_backend_testing::{
  AccessibleRole, ElementHandle, ElementQuery, init_integration_test_with_system_time,
};
use slint::platform::{Key, PointerEventButton, WindowEvent};
use slint::{ComponentHandle, LogicalPosition, VecModel};

/// One option, two `boolean` fields and a **`choice`**. Two booleans and not
/// one so that "no element was rebuilt" has more than one element to be true
/// of; the `choice` is what VT-8 drives, and its two alternatives carry
/// different labels so that *which* one the box is sitting on is readable off
/// the widget.
const A_FORM: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"read","kind":"boolean","label":"Read"},{"id":"mood","kind":"choice","label":"How did it go?","options":[{"id":"badly","label":"Badly"},{"id":"fine","label":"Fine"}]}]}]},"next_check":"45 minutes"}"#;

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
/// pass with an epoch frozen at zero and no handler ever running. The fourth
/// field is the `ComboBox`'s own account of what it is showing, which is what
/// tells *the guard ran and wrote* from *the counter moved for some other
/// reason*.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Reading {
  inits: i32,
  reasserts: i32,
  epoch: i32,
  chosen: String,
}

fn read(window: &PromptWindow) -> Reading {
  Reading {
    inits: window.get_inits(),
    reasserts: window.get_reasserts(),
    epoch: window.get_epoch(),
    chosen: shown(window),
  }
}

/// The `ComboBox`, found by the role it declares (`fluent/combobox.slint:28`).
/// This target installs no callback table and drives one widget, so a bare
/// role query is unambiguous where `tests/renderer/` needs an option scope.
fn combo_box(window: &PromptWindow) -> ElementHandle {
  ElementQuery::from_root(window)
    .match_accessible_role(AccessibleRole::Combobox)
    .find_first()
    .expect("the form draws one combo box")
}

/// What the `ComboBox` says it is showing — its `current-value`, published as
/// its accessible value (`fluent/combobox.slint:32`).
fn shown(window: &PromptWindow) -> String {
  combo_box(window)
    .accessible_value()
    .map(|value| value.to_string())
    .unwrap_or_default()
}

/// One key, pressed and released on whatever holds focus.
fn press(window: &PromptWindow, key: Key) {
  let window = ComponentHandle::window(window);
  window.dispatch_event(WindowEvent::KeyPressed { text: key.into() });
  window.dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

/// Click the box at its own centre — the three events
/// `ElementHandle::mock_single_click` dispatches, written out.
///
/// **Written out because `mock_single_click` cannot be called from here.** It
/// advances mock time between press and release (`search_api.rs:968-976`), and
/// that calls `TimerList::maybe_activate_timers`, which asserts *"Recursion in
/// timer code"* when it is reached from inside a timer callback — which the
/// stepper below is. Measured, not guessed. The events are the same three, at
/// the same point, and the point is a **window** coordinate because the box is
/// an ordinary laid-out element of the window rather than something inside a
/// popup.
fn click(window: &PromptWindow, element: &ElementHandle) {
  let origin = element.absolute_position();
  let size = element.size();
  let position = LogicalPosition::new(origin.x + size.width / 2.0, origin.y + size.height / 2.0);
  let window = ComponentHandle::window(window);
  window.dispatch_event(WindowEvent::PointerMoved { position });
  window.dispatch_event(WindowEvent::PointerPressed {
    position,
    button: PointerEventButton::Left,
  });
  window.dispatch_event(WindowEvent::PointerReleased {
    position,
    button: PointerEventButton::Left,
  });
}

/// Choose the **next** alternative, the way a person does: click the box open,
/// arrow down, press Return.
///
/// Not a click on the row. A pointer event is dispatched at an element's
/// absolute centre, and an item inside an embedded popup reports a popup-local
/// position (`i-slint-core/item_tree.rs:628-630`), which dispatch then reads
/// as a window one — so a click aimed at a row misses. The arrow reaches the
/// same `select(index)` a row's `clicked` reaches
/// (`common/combobox-base.slint:20-39`), which is what makes this a driver and
/// not a shortcut (PHASE-09/VA-1).
fn choose_the_next(window: &PromptWindow) {
  click(window, &combo_box(window));
  press(window, Key::DownArrow);
  press(window, Key::Return);
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

/// **AC-5, then A-5.** Present the same frame twice, with the loop running in
/// between, and neither counter moves — then change a widget behind the host's
/// back, present again, and watch the counter move.
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
///
/// **VT-8's claim is the contrast, and it is measured last so that AC-5's is
/// measured on a window nothing has touched.** No `Wire` is installed here, so
/// `root.edited` reaches nothing and the choice never becomes a
/// `Command::Edit`: the widget is showing an answer the host does not hold.
/// The third present is where the guard finds that, writes the draft's index
/// back and counts itself — which is the *only* measurement of A-5, the
/// assumption that a `ComboBox`'s `current-index` survives a `values` rewrite
/// like the other four controls' state does.
///
/// Reading the box's own value beside the counter is what separates *the guard
/// wrote this widget* from *something incremented the counter*: an increment
/// with the box still reading `Fine` would be a correction that did not
/// correct.
#[test]
fn a_second_present_corrects_nothing_and_a_widget_the_host_never_heard_from_is_corrected() {
  init_integration_test_with_system_time();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  // A `Debounce` of its own, because this case installs no callback table and
  // so has nothing to share one with: no control here can raise an edit, so the
  // overlay has nothing to overlay. Where a case **does** call `install`, the
  // glass must be given a clone of that handle (`design.md` §8 R10).
  let mut glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
    Rc::new(Debounce::new()),
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
      1 | 3 | 6 => glass.present(controller.frame(false)),
      // Read *after* the loop has run the change handlers the present before
      // it armed.
      2 | 4 => recording.borrow_mut().push(read(&stepped)),
      // The divergence AC-5's two readings cannot produce: a widget changed
      // by a person, on a step of its own so that the present after it is the
      // first thing that could correct it.
      5 => choose_the_next(&stepped),
      7 => {
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
  let [first, second, third] = readings.as_slice() else {
    panic!(
      "the stepper must have taken all three readings within {LIVENESS_BOUND:?}: {readings:?}"
    );
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

  // **A-5**, and the contrast. Everything above says the counter stays where
  // it was; everything below says it can move, and that the widget it moved
  // for is the one that had drifted.
  assert_eq!(
    second.chosen, "Badly",
    "the box was sitting on the field's first alternative, which is what the \
     draft holds for a `choice` nobody has answered: {second:?}"
  );
  assert!(
    third.reasserts > second.reasserts,
    "a widget the host never heard from is corrected on the next present, and the \
     guard counts itself doing it: {second:?} then {third:?}"
  );
  assert_eq!(
    third.chosen, "Badly",
    "and `current-index` is back at the draft's — an increment with the box still \
     reading `Fine` would be a correction that did not correct: {third:?}"
  );
  assert_eq!(
    third.inits, first.inits,
    "the third present carried the same view as the first two, so it wrote values \
     and not rows: {first:?} then {third:?}"
  );
}

/// The crate's `quit_event_loop` spelling, mirrored from `main.rs`: matched
/// rather than discarded, because `let _ =` trips `let_underscore_must_use`.
fn quit() {
  match slint::quit_event_loop() {
    Ok(()) | Err(_) => (),
  }
}

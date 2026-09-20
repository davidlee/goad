//! **What an open `datetime` picker does to the view underneath it**, driven
//! rather than argued.
//!
//! Both pickers are root singletons declared outside the
//! `if root.mode == WindowMode.prompt` block (`ui/app.slint:946`, `:960`), so
//! neither the row rebuild nor `hide()` reaches them (`glass.rs:189-195`,
//! `:225-231`), and both bind `PopupClosePolicy.no-auto-close`
//! (`fluent/datepicker.slint:23`, `fluent/time-picker.slint:24`). Nothing in
//! `goad` closes an active popup: `close_all_popups` has one caller,
//! `WindowInner::set_component` (`i-slint-core-1.17.1/window.rs:608`, `:1979`),
//! and this crate calls `set_component` once, at construction.
//!
//! This file is the measurement of what that costs a person, and it is written
//! with **controls on both sides of the claim** — the same click, and the same
//! keystroke, made once with no picker up and once with a picker up, so that a
//! silent no-op cannot be read as a driver that missed.

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
use slint::platform::{PointerEventButton, WindowEvent};
use slint::{ComponentHandle, LogicalPosition, SharedString, VecModel};

/// The view a picker is opened under. One option, a `text` field to type into
/// and a `datetime` field to open the picker from — the smallest form in which
/// *pointer reached the form* and *keyboard reached the form* are two separate
/// readings.
const MORNING: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"noted","kind":"text","label":"Anything to add?"},{"id":"when","kind":"datetime","label":"When?"}]}]},"next_check":"45 minutes"}"#;

/// The view that replaces it. **A different option id**, so the new view's
/// control cannot be confused with the old one's in a callback log, and so
/// that finding it at all is evidence the rows were rebuilt — `present` writes
/// the row model only where the `ViewId` changed (`glass.rs:189-195`).
const EVENING: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"evening","label":"Evening","fields":[{"id":"noted","kind":"text","label":"Anything to add?"},{"id":"when","kind":"datetime","label":"When?"}]}]},"next_check":"45 minutes"}"#;

/// How often the stepper runs — long enough that the loop renders between two
/// steps, which is what lays the window out and pumps the change trackers a
/// pointer event is hit-tested against, and short enough that the case is
/// quick (`event_loop_reassert/reassert.rs` states the same reason).
const STEP: Duration = Duration::from_millis(30);

/// The stop the case cannot run without. Nothing panics from inside the loop:
/// the steps record, the loop is quit, and every assertion is made on the test
/// thread (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
const LIVENESS_BOUND: Duration = Duration::from_secs(20);

/// One reading of the window. `picker` is the date picker's own chrome found
/// in the element tree; `chosen` and `edits` are the two host callbacks' whole
/// history to that point, kept as history rather than as counts so that a
/// failure message says *which* view a callback named.
#[derive(Debug, Clone)]
struct Reading {
  /// Which step took it. Read only through `Debug`, in the assertion messages
  /// below — which is a use dead-code analysis does not count, and the reason
  /// this carries an `expect` rather than being deleted: a failure that did
  /// not say *which* reading it is reporting is a failure nobody can place.
  #[expect(
    dead_code,
    reason = "read through the derived `Debug` in every assertion message; \
      dead-code analysis intentionally ignores derived impls"
  )]
  at: &'static str,
  picker: bool,
  chosen: Vec<String>,
  edits: Vec<String>,
}

/// Whether the date picker is up, read off the element tree.
///
/// `ElementQuery` walks `active_popups` as well as the window's own tree
/// (`i-slint-backend-testing-1.17.1/search_api.rs:296-312`), which is what
/// lets this target see inside a popup at all. *Next month* is a label only
/// `DatePickerBase` declares (`common/datepicker_base.slint:382`), so it
/// cannot be satisfied by anything this form draws.
fn picker_is_up(window: &PromptWindow) -> bool {
  button_labelled(window, "Next month").is_some()
}

fn button_described(window: &PromptWindow, description: &str) -> Option<ElementHandle> {
  let wanted = description.to_string();
  ElementQuery::from_root(window)
    .match_accessible_role(AccessibleRole::Button)
    .match_predicate(move |element| {
      element
        .accessible_description()
        .is_some_and(|found| found.as_str() == wanted)
    })
    .find_first()
}

fn button_labelled(window: &PromptWindow, label: &str) -> Option<ElementHandle> {
  let wanted = label.to_string();
  ElementQuery::from_root(window)
    .match_accessible_role(AccessibleRole::Button)
    .match_predicate(move |element| {
      element
        .accessible_label()
        .is_some_and(|found| found.as_str() == wanted)
    })
    .find_first()
}

/// The `LineEdit` a field id describes. Selected by description *and* by
/// declaring an accessible value, which is what tells a `LineEdit` from the
/// field container that answers to the same description
/// (`ui/app.slint:470-477`).
fn line_edit_described(window: &PromptWindow, description: &str) -> Option<ElementHandle> {
  let wanted = description.to_string();
  ElementQuery::from_root(window)
    .match_predicate(move |element| {
      element
        .accessible_description()
        .is_some_and(|found| found.as_str() == wanted)
        && element.accessible_value().is_some()
    })
    .find_first()
}

/// Click one element at its own centre — the three events
/// `ElementHandle::mock_single_click` dispatches, written out.
///
/// **A real pointer event, deliberately.** `invoke_accessible_default_action`
/// runs the declared action unconditionally and reaches neither `enabled` nor
/// hit testing, so it cannot say whether input *arrives*; that is the whole
/// question here. Written out rather than called because `mock_single_click`
/// advances mock time between press and release
/// (`search_api.rs:968-976`), which asserts *"Recursion in timer code"* when
/// it is reached from inside a timer callback — which the stepper is.
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

/// One key, pressed and released on whatever holds focus.
fn key(window: &PromptWindow, text: SharedString) {
  let window = ComponentHandle::window(window);
  window.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
  window.dispatch_event(WindowEvent::KeyReleased { text });
}

fn now() -> Timestamp {
  Timestamp::new(
    "2026-01-01T00:00:00Z"
      .parse()
      .expect("the fixture instant must parse"),
  )
}

fn view_of(source: &str) -> View {
  read_response(source.as_bytes(), now())
    .expect("the fixture must normalize")
    .value
    .view()
    .expect("the fixture carries a view")
    .clone()
}

/// An `Outcome` built by hand rather than by driving a child process: what
/// this case is about is what a *present* does to an open popup, and a
/// scripted backend would cost a runtime and a `LocalSet` to reach the same
/// `Prepared` (`event_loop_reassert/reassert.rs` takes the same shortcut for
/// the same reason).
fn outcome(view: Option<Presented>) -> Outcome {
  Outcome {
    view,
    next_check: now(),
    discarded: Vec::new(),
    stderr: Captured::default(),
    failure: None,
    cleanup: None,
  }
}

fn presenting(view_id: &str, source: &str) -> Outcome {
  outcome(Some(Presented {
    view_id: ViewId::new(view_id),
    view: view_of(source),
  }))
}

/// **A picker open across a view replacement, and across `hide()`.**
///
/// `design.md` §5.4 *Picking a datetime* and §5.5's edges table end a pick in
/// exactly one of `accepted`, `canceled`, or `compose` failing; §8 **R5**
/// prices a view replacement as *the field clears under the caret*. Neither
/// contemplates a picker still on screen after the view it belongs to is gone.
/// This case holds the consequence of that: **a present that replaces the view
/// leaves the person able to answer the new one.**
///
/// Three claims, each with its own control taken first on the same window with
/// the same driver:
///
/// | control (no picker up) | claim (picker up, view replaced) |
/// |---|---|
/// | a pointer click on the option raises `chosen` | it still does, for the new view |
/// | a keystroke reaches the `LineEdit` and raises `edited` | it still does |
/// | — | the picker is not still in the element tree, and does not outlive `hide()` |
///
/// The controls are what stop this case passing for the wrong reason. A driver
/// that aimed at the wrong coordinate, or a window that was never laid out,
/// would raise nothing *whether or not* a picker were up; the control readings
/// say the same click and the same keystroke did raise something moments
/// earlier, so a silence afterwards is the popup's and not the driver's
/// (`docs/memory/tests-asserting-proxies.md`).
///
/// **What the tree said when this was written**, in case the shape of the
/// failure is ever needed: the picker survived both the replacement and
/// `hide()`; a pointer click on the new option and a keystroke aimed at its
/// `LineEdit` both raised nothing, while the *same* option dispatched past hit
/// testing raised `chosen` normally — so the widget was alive and only input
/// routing was blocked; `Escape` did not dismiss the picker; and completing
/// the pick raised `edited` naming the *replaced* view's token, which
/// `Controller::edit` refuses `SupersededView`.
#[test]
fn an_open_picker_does_not_outlive_the_view_it_belongs_to_and_the_form_stays_answerable() {
  init_integration_test_with_system_time();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  // A `Debounce` of its own, because this case installs no callback table and
  // so has nothing to share one with (`design.md` §8 R10).
  let mut glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
    Rc::new(Debounce::new()),
  );

  // The two host callbacks, recorded rather than routed. `install` is not
  // called: what a `Command` does with a click is `controller`'s business and
  // is measured elsewhere; what this case needs is whether the markup raised
  // the callback at all.
  let chosen: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
  let edits: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

  let recording_chosen = Rc::clone(&chosen);
  window.on_chosen(move |view, option| {
    recording_chosen
      .borrow_mut()
      .push(format!("{view}/{option}"));
  });
  let recording_edits = Rc::clone(&edits);
  window.on_edited(move |view, option, field, edit| {
    recording_edits
      .borrow_mut()
      .push(format!("{view}/{option}/{field}={}", edit.text));
  });

  let mut controller = Controller::new();
  controller.absorb(Exchanged::Evaluation, presenting("v1", MORNING));

  // Recorded from inside the loop, read on the test thread once
  // `run_event_loop_until_quit` has returned.
  let readings: Rc<RefCell<Vec<Reading>>> = Rc::new(RefCell::new(Vec::new()));

  let recording = Rc::clone(&readings);
  let chosen_at = Rc::clone(&chosen);
  let edits_at = Rc::clone(&edits);
  let stepped = window.clone_strong();
  let stepper = slint::Timer::default();
  let mut step = 0_u8;
  stepper.start(slint::TimerMode::Repeated, STEP, move || {
    let read = |at: &'static str| {
      recording.borrow_mut().push(Reading {
        at,
        picker: picker_is_up(&stepped),
        chosen: chosen_at.borrow().clone(),
        edits: edits_at.borrow().clone(),
      });
    };
    // Each act is a step of its own, and every reading is the step after the
    // act it reports on: a pointer event is hit-tested against laid-out
    // geometry and a `changed` handler is run by the property evaluator the
    // loop drives, so two of these back to back inside one call would measure
    // a window that had not caught up.
    step += 1;
    match step {
      1 => glass.present(controller.frame(false)),
      // The two controls, on a window with no popup up.
      2 => click_button_described(&stepped, "morning"),
      3 => {
        read("A the option answered by pointer, no picker up");
        click_line_edit(&stepped, "noted");
      }
      4 => key(&stepped, "x".into()),
      5 => {
        read("B the text field typed into, no picker up");
        click_button_described(&stepped, "when");
      }
      // The arrangement: a picker open, and the view replaced under it.
      6 => {
        read("C the picker open, under v1");
        controller.absorb(Exchanged::Evaluation, presenting("v2", EVENING));
        glass.present(controller.frame(false));
      }
      7 => {
        read("D the view replaced, with the picker open");
        click_button_described(&stepped, "evening");
      }
      8 => {
        read("E the new view's option answered by pointer");
        click_line_edit(&stepped, "noted");
      }
      9 => key(&stepped, "y".into()),
      10 => {
        read("F the new view's text field typed into");
        // **Reopened, and the reopening is load-bearing.** The hide claim
        // below is about `hide()` and not about the replacement above it, so
        // it needs a picker that is up *for this step's reason*. Without this
        // click, a host that dismissed pickers on the replacement alone would
        // reach the hide with nothing open and read green on a claim it never
        // exercised (`docs/memory/tests-asserting-proxies.md`) — measured:
        // that is exactly what the first draft of this file did.
        click_button_described(&stepped, "when");
      }
      11 => {
        read("G the picker reopened, under v2");
        // `Shift::Closed`: an answered view with nothing to replace it, which
        // is `present`'s `Surface::Hidden` and `window.hide()`.
        controller.absorb(Exchanged::Answer, outcome(None));
        glass.present(controller.frame(false));
      }
      12 => {
        read("H after hide()");
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
  let [
    control_click,
    control_key,
    opened,
    replaced,
    clicked,
    typed,
    reopened,
    hidden,
  ] = readings.as_slice()
  else {
    panic!(
      "the stepper must have taken all eight readings within {LIVENESS_BOUND:?}: {readings:?}"
    );
  };

  // The controls first. Everything below is read against these, and a case in
  // which they did not hold would be measuring its own driver.
  assert_eq!(
    control_click.chosen,
    vec!["v1/morning".to_string()],
    "a pointer click on the option must answer the view when no picker is up, \
     or nothing below separates a blocked click from a missed one: {control_click:?}"
  );
  assert_eq!(
    control_key.edits,
    vec!["v1/morning/noted=x".to_string()],
    "a keystroke must reach the text field when no picker is up, for the same \
     reason: {control_key:?}"
  );
  assert!(
    !control_key.picker && opened.picker,
    "the arrangement itself: no picker up while the controls were taken, and a \
     picker up once the field's button was clicked — {control_key:?} then {opened:?}"
  );

  // The claim.
  assert!(
    !replaced.picker,
    "a present carrying a new view id replaces the form, and the picker belonging \
     to the form it replaced must not still be on screen: {opened:?} then {replaced:?}"
  );
  assert_eq!(
    clicked.chosen,
    vec!["v1/morning".to_string(), "v2/evening".to_string()],
    "and the new view must be answerable by the same pointer click that answered \
     the old one: {clicked:?}"
  );
  assert_eq!(
    typed.edits,
    vec![
      "v1/morning/noted=x".to_string(),
      "v2/evening/noted=y".to_string()
    ],
    "and its text field must take a keystroke, by the same route the control took \
     one: {typed:?}"
  );
  // The hide claim, and the precondition without which it measures nothing: a
  // picker that is up *going into* the hide. A host that dismissed pickers on
  // the replacement alone and not on `hide()` reads red here and green
  // everywhere else, which is the point of taking the reading separately.
  assert!(
    reopened.picker,
    "the arrangement for the claim below: the field's button must reopen a \
     picker once the view has been replaced, or `hide()` is handed a window \
     with nothing open and the assertion after this one is vacuous: {reopened:?}"
  );
  assert!(
    !hidden.picker,
    "and a picker must not outlive the window it was opened from: {reopened:?} \
     then {hidden:?}"
  );
}

/// Click the `Button` a description names, or fail the case by leaving the
/// callback log short — the stepper may not panic, so a control that is not
/// there is reported by the assertion that reads its reading.
fn click_button_described(window: &PromptWindow, description: &str) {
  if let Some(element) = button_described(window, description) {
    click(window, &element);
  }
}

fn click_line_edit(window: &PromptWindow, description: &str) {
  if let Some(element) = line_edit_described(window, description) {
    click(window, &element);
  }
}

/// The crate's `quit_event_loop` spelling, mirrored from `main.rs`: matched
/// rather than discarded, because `let _ =` trips `let_underscore_must_use`.
fn quit() {
  match slint::quit_event_loop() {
    Ok(()) | Err(_) => (),
  }
}

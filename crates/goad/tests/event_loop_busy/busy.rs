//! **`busy` means *your answer is in flight*** — `review-code.md` **F-A1**
//! and **F-R2**, and **AC-4**: *typing into a `text` field records every
//! character.*
//!
//! One arrangement, three readings, each a real key event delivered to a
//! focused `LineEdit` in a laid-out window, and the three differ in one thing
//! only: what the frame before them carried as `busy`.
//!
//! | reading | the frame before it | the key |
//! |---|---|---|
//! | A — the control | nothing engaged | is recorded |
//! | B — the claim | `engage(Exchanged::Evaluation)`: a scheduled poll, a tray check, an ingested event | is recorded |
//! | C — the contract kept | `engage(Exchanged::Answer)`: the person's own click | is **not** recorded |
//!
//! A is what stops B passing for the wrong reason: a driver aiming at the
//! wrong coordinate, or a window that was never laid out, would record
//! nothing whether or not anything were engaged
//! (`docs/memory/tests-asserting-proxies.md`). C is what stops the repair
//! being read as *`busy` was deleted*: slice 003's double-submit guard is
//! still there, and after the narrowing it fires exactly when it wants to —
//! `Command::Choose` has one origin (`install.rs:40`) and is the only road to
//! `Pending::Respond`, so a `Respond` is always the person's own click.
//!
//! What a failure here looked like before the repair: every reading after A
//! showed the key discarded, because `engage` set `busy` for any exchange and
//! Slint *drops* input for a disabled item rather than queueing it.

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
  ElementHandle, ElementQuery, init_integration_test_with_system_time,
};
use slint::platform::{PointerEventButton, WindowEvent};
use slint::{ComponentHandle, LogicalPosition, SharedString, VecModel};

/// One option, one `text` field. The smallest form in which *a keystroke
/// reached the field* is a reading.
const A_FORM: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"noted","kind":"text","label":"Anything to add?"}]}]},"next_check":"45 minutes"}"#;

/// How often the stepper runs. Long enough that the loop renders between two
/// steps — which is what lays the window out and pumps the change trackers a
/// pointer event is hit-tested against — and short enough that the case is
/// quick (`event_loop_reassert/reassert.rs` states the same reason).
const STEP: Duration = Duration::from_millis(30);

/// The stop the case cannot run without. Nothing panics from inside the loop:
/// the steps record, the loop is quit, and every assertion is made on the test
/// thread (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
const LIVENESS_BOUND: Duration = Duration::from_secs(10);

/// One reading. `edits` is the `edited` callback's whole history to that
/// point rather than a count, so a failure message says *which* keystroke was
/// recorded; `shown` is what the widget itself is displaying, which is the
/// same claim read off the screen instead of off a callback; `busy` is the
/// arrangement, without which the two above are facts about an unknown frame.
#[derive(Debug, Clone)]
struct Reading {
  /// Which step took it. Read only through `Debug`, in the assertion messages
  /// below.
  #[expect(
    dead_code,
    reason = "read through the derived `Debug` in every assertion message; \
      dead-code analysis intentionally ignores derived impls"
  )]
  at: &'static str,
  busy: bool,
  shown: String,
  edits: Vec<String>,
}

/// The `LineEdit` a field id describes. Selected by description *and* by
/// declaring an accessible value, which is what tells a `LineEdit` from the
/// field container that answers to the same description (`ui/app.slint`).
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

/// What the widget is showing — its `accessible-value`, which `LineEdit` binds
/// two-way to its `text` (`fluent/lineedit.slint:13`).
fn shown(window: &PromptWindow, field: &str) -> String {
  line_edit_described(window, field)
    .and_then(|element| element.accessible_value())
    .map(|value| value.to_string())
    .unwrap_or_default()
}

/// Click one element at its own centre — the three events
/// `ElementHandle::mock_single_click` dispatches, written out.
///
/// **A real pointer event, deliberately**: it is what gives the `LineEdit`
/// focus, and an accessible action would reach neither hit testing nor
/// `enabled`. Written out rather than calling `mock_single_click`, which
/// advances mock time between press and release (`search_api.rs:968-976`) and
/// so asserts *"Recursion in timer code"* when it is reached from inside a
/// timer callback — which the stepper below is.
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

/// One key, pressed and released on whatever holds focus. **The instrument**:
/// this is the road `enabled` gates, and the only one.
fn key(window: &PromptWindow, text: SharedString) {
  let window = ComponentHandle::window(window);
  window.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
  window.dispatch_event(WindowEvent::KeyReleased { text });
}

/// A controller with `A_FORM` retained, built from a hand-made `Outcome`
/// rather than by driving a child process: what this case is about is what a
/// frame's `busy` does to a key event, and a scripted backend would cost a
/// runtime and a `LocalSet` to reach the same `Prepared`
/// (`event_loop_reassert/reassert.rs` takes the same shortcut for the same
/// reason).
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

/// **F-A1, F-R2.** A key typed while the host is polling the backend is
/// recorded; a key typed while the person's own answer is in flight is not.
///
/// `install` is deliberately **not** called: what a `Command` does with an
/// edit is `controller`'s business and is measured elsewhere, and what this
/// case needs is whether the markup raised the callback at all. One
/// consequence is worth stating, because it is why each reading's `shown` is
/// a single character rather than a growing word: with nothing recording the
/// edits, the draft stays empty, so the guard finds the widget diverged at
/// every present and writes it back to `""` before the next key is typed.
#[test]
fn a_key_is_recorded_while_the_host_polls_and_dropped_while_the_answer_is_in_flight() {
  init_integration_test_with_system_time();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  // A shown window clips, and left at its preferred size the field is
  // unreachable to the query — `event_loop_overlay/overlay.rs` states the
  // same reason.
  ComponentHandle::window(&window).set_size(slint::PhysicalSize::new(600, 600));

  // A `Debounce` of its own, because this case installs no callback table and
  // so has nothing to share one with (`design.md` §8 R10).
  let mut glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
    Rc::new(Debounce::new()),
  );

  let edits: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
  let recording_edits = Rc::clone(&edits);
  window.on_edited(move |view, option, field, edit| {
    recording_edits
      .borrow_mut()
      .push(format!("{view}/{option}/{field}={}", edit.text));
  });

  let mut controller = retaining();

  // Recorded from inside the loop, read on the test thread once
  // `run_event_loop_until_quit` has returned — the two share the one thread
  // the testing backend runs on.
  let readings: Rc<RefCell<Vec<Reading>>> = Rc::new(RefCell::new(Vec::new()));

  let recording = Rc::clone(&readings);
  let edits_at = Rc::clone(&edits);
  let stepped = window.clone_strong();
  let stepper = slint::Timer::default();
  let mut step = 0_u8;
  stepper.start(slint::TimerMode::Repeated, STEP, move || {
    let read = |at: &'static str| {
      recording.borrow_mut().push(Reading {
        at,
        busy: stepped.get_busy(),
        shown: shown(&stepped, "noted"),
        edits: edits_at.borrow().clone(),
      });
    };
    // Each act is a step of its own, and every reading is the step after the
    // act it reports on: a pointer event is hit-tested against laid-out
    // geometry and the guard is a `changed` handler, so two of these back to
    // back inside one call would measure a window that had not caught up.
    step += 1;
    match step {
      1 => glass.present(controller.frame(false)),
      2 => click_line_edit(&stepped, "noted"),
      3 => key(&stepped, "a".into()),
      // A — the control. Nothing is engaged, and the key was recorded.
      4 => {
        read("A a key with nothing engaged");
        controller.engage(Exchanged::Evaluation);
        glass.present(controller.frame(false));
      }
      5 => key(&stepped, "b".into()),
      // B — the claim. The host is mid-exchange with the backend, and the
      // person goes on typing.
      6 => {
        read("B a key while an evaluation is in flight");
        controller.engage(Exchanged::Answer);
        glass.present(controller.frame(false));
      }
      7 => key(&stepped, "c".into()),
      // C — the contract that is kept. Their own answer is in flight, and the
      // form is inert.
      8 => {
        read("C a key while the person's own answer is in flight");
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
  let [control, polling, answering] = readings.as_slice() else {
    panic!(
      "the stepper must have taken all three readings within {LIVENESS_BOUND:?}: {readings:?}"
    );
  };

  // The control first. Everything below is read against it, and a case in
  // which it did not hold would be measuring its own driver.
  assert_eq!(
    control.edits,
    vec!["v1/morning/noted=a".to_string()],
    "a key must reach the field when nothing is engaged, or nothing below \
     separates a dropped keystroke from a missed one: {control:?}"
  );
  assert!(
    !control.busy,
    "the arrangement: nothing engaged, so the frame carried `busy = false`: {control:?}"
  );

  // The claim. The observable first, the frame flag that explains it second:
  // a failure should name the keystroke that went missing, not the property.
  assert_eq!(
    polling.edits,
    vec![
      "v1/morning/noted=a".to_string(),
      "v1/morning/noted=b".to_string()
    ],
    "a key typed while the host is polling the backend must be recorded, by \
     the same route the control's was: {control:?} then {polling:?}"
  );
  assert!(
    !polling.busy,
    "and the reason it was: an evaluation is the host's own business and does \
     not disable the form — the narrowing itself, read off the frame: {polling:?}"
  );
  assert_eq!(
    polling.shown, "b",
    "the widget itself shows the character, and not the empty field a discarded \
     key would leave: {polling:?}"
  );

  // The contract that is kept: slice 003's double-submit guard.
  assert!(
    answering.busy,
    "the person's own answer still disables the form: {answering:?}"
  );
  assert_eq!(
    answering.edits, polling.edits,
    "so a key typed while it is in flight raises nothing — slice 003's \
     double-submit guard fires exactly where it was written to: {polling:?} \
     then {answering:?}"
  );
  assert_eq!(
    answering.shown, "",
    "and the widget shows the draft's empty value rather than the character, \
     which is the same claim read off the screen: {answering:?}"
  );
}

/// Click the `LineEdit` a description names, or fail the case by leaving the
/// callback log short — the stepper may not panic, so a control that is not
/// there is reported by the assertion that reads its reading.
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

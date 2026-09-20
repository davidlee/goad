//! `plan.md` PHASE-05/**EX-4**, **VA-1** — *a `Full` send clears nothing.*
//!
//! One `text` field is typed into, so the map holds exactly one entry and the
//! reading is unambiguous. The channel is then **occupied** before the tick
//! can fire, and left occupied across it:
//!
//! ```text
//!  reading  what has happened                          held  handled
//!  A        typed, channel occupied, before any tick      1        0
//!  B        a tick has fired against a full channel       1        0
//!  C        the channel drained, a later tick fired       0        1
//! ```
//!
//! **B is the claim.** The entry must still be there: the send came back
//! `Full`, so it delivered nothing, and `tick` must not have removed it. **C
//! is what stops B passing for the wrong reason** — an entry that stands
//! because the timer never fired at all would read the same at B, so the case
//! goes on to drain and shows the *same* entry arriving once a send can
//! succeed. Without C, `arm` deleted entirely would leave B green.
//!
//! **A is the control on the setup**, and on nothing else: it says the typing
//! reached the map at all, so that B's `held == 1` is an entry that survived a
//! tick rather than an entry that was never there. It does **not** observe the
//! channel — the `assert!(enqueued)` at step 3 is what holds the occupancy,
//! and the injection below is what shows B depends on it.
//!
//! The draft is read through `answer` at C, so what is asserted is a value
//! that would reach a backend rather than a private field.
//!
//! **Injection pass**, each mutation applied to the tree, the target run, the
//! message read, and the file restored from a copy:
//!
//! | # | mutation | reading |
//! |---|---|---|
//! | I1 | `pending.rs:212-214`'s `if enqueued` removed, so `tick` always removes | **red at B** — `held: 0, handled: 0` against `(1, 0)`. The claim |
//! | I2 | the re-arm neutered (`if false && …`) | **red at C alone** — `held: 1, handled: 0` against `(0, 1)`; B passes, so the two readings discriminate independently |
//! | I3 | the occupying send drained again immediately, so the channel is empty at tick time | **red at B** — `held: 0, handled: 0`: with a channel that is not full the send succeeds and the entry goes, which is what makes B's reading a fact about `Full` |

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use goad::controller::{Controller, Exchanged};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::{Glass, SlintGlass};
use goad::install::install;
use goad::pending::Debounce;
use goad::wire::{Cancel, Command, Notice, Stimulus, Wire};
use goad_semantics::protocol::canonical::{Timestamp, View, ViewId};
use goad_semantics::protocol::normalize::read_response;
use goad_shell::backend::transport::Captured;
use goad_shell::host::{Outcome, Presented};
use i_slint_backend_testing::{ElementQuery, init_integration_test_with_system_time};
use slint::{ComponentHandle, VecModel};
use tokio::sync::mpsc;

/// One option, one `text` field. **One, deliberately**: `event_loop_debounce`
/// needs two to tell *delivered* from *delivered and re-armed*, and this case
/// needs one so that `held` counts the entry under test and nothing else.
const ONE_TEXT_FIELD: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"noted","kind":"text","label":"Anything to add?"}]}]},"next_check":"45 minutes"}"#;

/// How often the stepper runs. Well under the 150 ms debounce, so the step
/// that types is not the step that reads after a tick.
const STEP: Duration = Duration::from_millis(25);

/// The stop the case cannot be allowed to run without: a predicate that never
/// becomes true must fail on a count rather than wedge the gate, so nothing
/// here panics from inside the loop.
const LIVENESS_BOUND: Duration = Duration::from_secs(5);

/// Long enough for several debounce windows at [`STEP`] each, so a tick that
/// is merely late is not read as a tick that never came.
const STEPS: u8 = 30;

/// A controller with `ONE_TEXT_FIELD` retained, built from a hand-made
/// `Outcome` rather than by driving a child process — `event_loop_debounce`'s
/// `retaining` is the precedent and this is the same shape.
fn retaining() -> Controller {
  let now = Timestamp::new(
    "2026-01-01T00:00:00Z"
      .parse()
      .expect("the fixture instant must parse"),
  );
  let view: View = read_response(ONE_TEXT_FIELD.as_bytes(), now)
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

/// Type into the field the way `set_accessible_value` does — it assigns the
/// widget's `text` and calls `edited` from inside the markup
/// (`widgets/fluent/lineedit.slint:16`), which is the shape a paste has.
fn type_into(window: &PromptWindow, field: &str, text: &str) {
  let described = field.to_owned();
  ElementQuery::from_root(window)
    .match_predicate(move |element| element.accessible_description().as_deref() == Some(&described))
    .find_first()
    .unwrap_or_else(|| panic!("no control described {field:?}"))
    .set_accessible_value(text);
}

/// What the draft holds for `morning`'s field, read back through `answer` —
/// the only reader of the draft this crate exposes.
fn drafted(controller: &Controller, view: &str, field: &str) -> Option<String> {
  let (_, answer) = controller.answer(view, "morning").ok()?;
  answer
    .values
    .iter()
    .find(|(id, _)| id.as_str() == field)
    .and_then(|(_, value)| value.as_str())
    .map(ToOwned::to_owned)
}

/// One reading: where the run had got to, how many entries the debounce map
/// still holds, and how many `Command::Edit`s have reached the controller.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Reading {
  at: &'static str,
  held: usize,
  handled: usize,
  noted: Option<String>,
}

/// **PHASE-05/EX-4 and VA-1** — the entry leaves on the enqueue, so a `Full`
/// send leaves it exactly where it was.
#[test]
fn a_tick_whose_send_comes_back_full_clears_nothing_and_the_entry_is_delivered_later() {
  init_integration_test_with_system_time();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  // A shown window clips, and left at its preferred size a form is unreachable
  // to the query — the same reason every `serve`-driven renderer case declares
  // a size.
  ComponentHandle::window(&window).set_size(slint::PhysicalSize::new(600, 600));

  let mut controller = retaining();
  let view = controller
    .frame(false)
    .shown
    .expect("the fixture must retain a view")
    .view_id
    .as_str()
    .to_owned();

  // **Capacity one, as production has it** (`main.rs:87`). This is what makes
  // the whole case possible: one command in the channel and the next send
  // comes back `Full`.
  let (tx, mut rx) = mpsc::channel::<Command>(1);
  let wire = Wire::new(tx, Cancel::new(), Notice::new());
  let pending = Rc::new(Debounce::new());
  // The production callback table, so what is driven below is the `edited`
  // closure `main.rs` installs and not a second copy of it.
  install(&window, &tray, &wire, &pending);
  // **The glass reads the same handle** — one `Debounce`, cloned into the
  // callbacks and the glass, never two values (`design.md` §8 R10).
  let mut glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
    Rc::clone(&pending),
  );

  // Recorded from inside the loop, read on the test thread once
  // `run_event_loop_until_quit` has returned — the two share the one thread
  // the testing backend runs on.
  let readings: Rc<RefCell<Vec<Reading>>> = Rc::new(RefCell::new(Vec::new()));
  let handled: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));

  let recording = Rc::clone(&readings);
  let counting = Rc::clone(&handled);
  let held_at = Rc::clone(&pending);
  let stepped = window.clone_strong();
  // Moved into the stepper: `install` already holds every clone the callbacks
  // need, and nothing out here uses it again.
  let occupying = wire;
  let quit = || slint::quit_event_loop().expect("the loop must be quittable");

  let stepper = slint::Timer::default();
  let mut step = 0_u8;
  stepper.start(slint::TimerMode::Repeated, STEP, move || {
    step += 1;

    let read = |at: &'static str, reading_from: &Controller| {
      recording.borrow_mut().push(Reading {
        at,
        held: held_at.carried().len(),
        handled: *counting.borrow(),
        noted: drafted(reading_from, &view, "noted"),
      });
    };

    match step {
      // The first present builds the form, so the control exists to type into.
      1 => glass.present(controller.frame(false)),
      2 => type_into(&stepped, "noted", "walked before breakfast"),
      // **Occupy the channel**, in the same step and so inside the same
      // debounce window. `Evaluate` rather than a second `Edit`: it is the
      // command a scheduled poll or a tray *check now* puts there in
      // production, it is not routed through the debounce, and it cannot be
      // mistaken for the entry under test when the drain below reads the
      // channel.
      3 => {
        let enqueued = occupying.send(Command::Evaluate(Stimulus::Requested));
        assert!(
          enqueued,
          "the occupying send must succeed into an empty channel, or the case measures nothing"
        );
      }
      // Before any tick can have fired: 150 ms after step 2 is step 8.
      4 => read("A typed, channel occupied, before any tick", &controller),
      // **Past the first tick, with the channel never drained.** The tick's
      // send came back `Full`, so the entry must still be held.
      11 => read("B a tick has fired against a full channel", &controller),
      // Drain — and now the re-armed tick has somewhere to put it. The
      // `Evaluate` is discarded rather than applied: this target holds no
      // backend, and what it was for was occupying the channel.
      12 => {
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
      }
      // Every step past the drain applies whatever the re-armed tick delivered,
      // exactly as `serve`'s `Command::Edit` arm does.
      _ if step > 12 => {
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
        if step >= STEPS {
          read("C the channel drained, a later tick fired", &controller);
          quit();
        }
      }
      _ => {}
    }
  });

  // The only thing standing between a step that never arrives and a hung gate.
  let stop = slint::Timer::default();
  stop.start(slint::TimerMode::SingleShot, LIVENESS_BOUND, quit);

  slint::run_event_loop_until_quit().expect("the headless loop must run, and quit when asked");

  let readings = readings.borrow();
  let [a, b, c] = readings.as_slice() else {
    panic!(
      "the stepper must have taken all three readings within {LIVENESS_BOUND:?}: {readings:?}"
    );
  };

  assert_eq!(
    (a.held, a.handled),
    (1, 0),
    "the typed edit must be held and nothing delivered before any tick: {a:?}"
  );

  // **The finding.** `pending.rs:212-214` removes the entry only `if enqueued`;
  // replace that with an unconditional remove and this is the assertion that
  // goes red. Under the tree as it stands the send came back `Full`, delivered
  // nothing, and the entry stands.
  assert_eq!(
    (b.held, b.handled),
    (1, 0),
    "a tick whose send came back `Full` delivered nothing, so it must have cleared nothing: {b:?}"
  );

  // **And the entry that stood is the same one, still deliverable** — which is
  // what stops B passing because no tick ever fired.
  assert_eq!(
    (c.held, c.handled),
    (0, 1),
    "once the channel drains, the re-armed tick must deliver the entry that stood: {c:?}"
  );
  assert_eq!(
    c.noted.as_deref(),
    Some("walked before breakfast"),
    "and what it delivered is what was typed, read back through `answer`: {c:?}"
  );
}

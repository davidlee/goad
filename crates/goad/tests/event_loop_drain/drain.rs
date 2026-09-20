//! **A tick that lands mid-exchange must not cost a person what they typed**
//! — `review-code.md` **F-R3**, and **AC-5**: *a present that changes nothing
//! about a field does not disturb it.*
//!
//! The window the defect lived in, in four readings:
//!
//! ```text
//!  A  typed, ticked, served      nothing in flight — the control
//!  B  an evaluation in flight, and a keystroke during it   held == 1
//!  C  the tick fired inside the exchange                   held == 0
//!       the value is now in the channel alone: the map let it go on the
//!       enqueue (`pending.rs`) and `serve` has not served it, because it is
//!       parked awaiting the backend
//!  D  the exchange completed, and the present that follows it
//! ```
//!
//! At **D** the old loop presented *before* it served the queued edit, so the
//! guard found the widget diverged from a draft that had never seen the
//! keystroke and wrote the pre-typing value back over it; the next statement
//! served the edit and presented again, writing it back. Two guard writes
//! across a completion that should have disturbed nothing — `reasserts` is
//! AC-5's own instrument and it counts them both. The repair drains
//! `commands` before the present, so there is one present and no write.
//!
//! **Why `reasserts` and not the widget's text.** Both presents happen inside
//! one poll of `serve` — `glass.present` is synchronous and the queued
//! `Command::Edit` is already ready at the `select!` below it — so the text
//! ends up correct either way and only the counter can tell the two apart.
//! The guard still *runs* at each of them, because `present` ends in
//! `window.show()` and `WindowInner::show` calls `ensure_tree_instantiated`,
//! which runs the change handlers (`i-slint-core-1.17.1/window.rs:648-663`,
//! `:1628`). Measured, not assumed: the injection pass is in `notes.md`.
//!
//! **What holds `reasserts == 0` to meaning something** is the epoch beside
//! it: a present that bumped nothing would leave the counter at rest too. And
//! what holds the whole arrangement to meaning something is `held`, the
//! debounce map's own size, read on both sides of the tick — without the
//! `1 → 0` this case would be measuring an edit the overlay still covered
//! (`docs/memory/tests-asserting-proxies.md`).
//!
//! This case is also where `busy`'s narrowing is measured through `serve`:
//! the keystroke at **B** is delivered by a real key event while a real
//! evaluation is in flight, and if that exchange disabled the form the way it
//! did before `review-code.md` F-A1 was repaired, there would be no edit to
//! hold, no tick, and nothing to drain.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use goad::controller::{Controller, Exchanged, serve};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad::install::install;
use goad::pending::{DEBOUNCE, Debounce};
use goad::wire::{Cancel, Command, Notice, Stimulus, Wire};
use goad_semantics::protocol::canonical::{Request, Timestamp, View, ViewId};
use goad_semantics::protocol::normalize::read_response;
use goad_shell::backend::transport::{Backend, Captured, Exchange};
use goad_shell::clock::wall_clock;
use goad_shell::config::{BackendConfig, Command as Spawned, Config, ScheduleConfig};
use goad_shell::host::{Host, Outcome, Presented};
use goad_shell::ingress::Ingress;
use i_slint_backend_testing::{
  ElementHandle, ElementQuery, init_integration_test_with_system_time,
};
use slint::platform::{PointerEventButton, WindowEvent};
use slint::{ComponentHandle, LogicalPosition, SharedString, VecModel};
use tokio::sync::{mpsc, oneshot};

/// One option, one `text` field — the smallest form this claim needs, and one
/// field rather than two so that `held` counts the entry under test and
/// nothing else.
const A_FORM: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"noted","kind":"text","label":"Anything to add?"}]}]},"next_check":"45 minutes"}"#;

/// What the held backend answers with: nothing to show, and a check far
/// enough out that it schedules nothing inside this case. No view, so the
/// fold is `Shift::Retained` — the form and the draft survive the exchange,
/// which is the ordinary result of a poll and the only one in which "the
/// widget was written back" is a defect rather than a replacement.
const NOTHING_INSTRUCTED: &str = r#"{"view":null,"next_check":"45 minutes"}"#;

/// What the person types: one character before the exchange, one during it.
const FIRST: &str = "x";
const SECOND: &str = "y";

/// How often the stepper runs — long enough that the loop renders between two
/// steps, which is what lays the window out and pumps the change trackers,
/// and short enough that the whole case finishes well inside the 3 s
/// `MINIMUM_SPACING` at which `serve`'s standing timer would fire an
/// evaluation of its own.
///
/// **Halved to 25 ms for reading B's margin** (`review-code.md` F-B4). B claims
/// the entry is *still* in the map, so it must be taken inside the debounce
/// window — and that is the one bound here that **load pushes towards
/// violation**, because a stalled stepper stretches the interval it caps. At
/// 50 ms it was measured violated twice under oversubscription, at 153 ms and
/// 742 ms, passing on the order two timers happened to be dispatched in. The
/// other two bounds are the safe direction: load stretches them *away* from
/// their limits. 25 ms is `event_loop_debounce`'s and `event_loop_full`'s step,
/// so the loop is known to render inside one.
const STEP: Duration = Duration::from_millis(25);

/// How many steps a debounce tick lands after the keystroke that armed it.
///
/// **Still a literal, but no longer a copy** (`review-code.md` F-T1's class,
/// applied here as well as in `full.rs`). It used to restate `pending.rs` in a
/// comment, with every step number below read against it — so a tuning change
/// to the production constant left the schedule saying one thing and doing
/// another, silently. The assertion below is what makes it derived in the only
/// sense that matters: a `DEBOUNCE` this number is wrong for **fails to
/// compile**. Written out rather than computed because the workspace lint set
/// denies both the integer division and the cast that computing it needs, and
/// an `#[expect]` for each would buy nothing the assertion does not already
/// hold.
const TICK_STEPS: u128 = 6;

const _: () = assert!(
  DEBOUNCE.as_millis() == STEP.as_millis() * TICK_STEPS,
  "a tick lands TICK_STEPS steps after the key, so the debounce must be exactly that many steps"
);

const _: () = assert!(
  STEP.as_millis() < DEBOUNCE.as_millis(),
  "reading B is taken one step after the key and claims the entry is still held, so one step must fit inside the debounce window"
);

const _: () = assert!(
  STEP.as_millis() * 10 > DEBOUNCE.as_millis(),
  "reading C is taken ten steps after the key and claims the tick has fired, so ten steps must exceed the debounce window"
);

/// The stop the case cannot run without. Nothing panics from inside the loop:
/// the steps record, the loop is quit, and every assertion is made on the test
/// thread (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
const LIVENESS_BOUND: Duration = Duration::from_secs(20);

/// One reading of the window, the debounce and the backend.
///
/// `held` is the debounce map's size and `exchanging` is whether the backend
/// has been entered and not yet answered: between them they say *which* of
/// the four moments above this reading is, which is the difference between
/// measuring the defect and measuring an edit the overlay still covered.
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
  shown: String,
  held: usize,
  exchanging: bool,
  inits: i32,
  reasserts: i32,
  epoch: i32,
}

/// A `Backend` that answers only when the case lets it.
///
/// `goad-shell/tests/integration/fake.rs` is the prior art and answers from a
/// script with a ready future; this one hands each exchange a `oneshot` the
/// case holds the other end of, because what is under test here is what the
/// rest of the program does *while* an exchange is outstanding. A `sleep` in
/// a scripted backend would be the same arrangement with a race in it.
///
/// It counts entries and answers rather than reporting "in flight" directly:
/// two monotonic counters can be read from the stepper thread without a lock
/// and cannot disagree with each other.
struct HeldBackend {
  released: VecDeque<oneshot::Receiver<()>>,
  entered: Arc<AtomicUsize>,
  answered: Arc<AtomicUsize>,
}

impl Backend for HeldBackend {
  fn exchange(&mut self, _request: &Request) -> impl Future<Output = Exchange> + Send {
    let released = self.released.pop_front();
    let entered = Arc::clone(&self.entered);
    let answered = Arc::clone(&self.answered);
    async move {
      entered.fetch_add(1, Ordering::Relaxed);
      // Past the end of the script it answers at once, which is the positive
      // control every scripted backend needs — and, here, what stops an
      // unplanned exchange wedging the gate instead of reddening it.
      if let Some(released) = released {
        // A dropped sender releases it too: teardown, not an answer.
        match released.await {
          Ok(()) | Err(_) => (),
        }
      }
      answered.fetch_add(1, Ordering::Relaxed);
      Exchange {
        result: Ok(NOTHING_INSTRUCTED.as_bytes().to_vec()),
        stderr: Captured::default(),
        cleanup: None,
      }
    }
  }
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
/// **A real pointer event**, because it is what gives the `LineEdit` focus,
/// and because `invoke_accessible_default_action` reaches neither hit testing
/// nor `enabled`. Written out rather than calling `mock_single_click`, which
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

fn click_line_edit(window: &PromptWindow, description: &str) {
  if let Some(element) = line_edit_described(window, description) {
    click(window, &element);
  }
}

/// One key, pressed and released on whatever holds focus. **A real key
/// event**: `set_accessible_value` would assign the text and call `edited`
/// from inside the markup, reaching neither `TextInput::key_event` nor the
/// `enabled` gate this case's arrangement turns on.
fn key(window: &PromptWindow, text: SharedString) {
  let window = ComponentHandle::window(window);
  window.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
  window.dispatch_event(WindowEvent::KeyReleased { text });
}

fn now() -> Timestamp {
  wall_clock().expect("the real wall clock reads fine in a test process")
}

/// A controller with `A_FORM` already retained, handed to `serve` so that the
/// form is on screen from its first present: the exchange this case holds is
/// the *second* thing to happen to the window, not the first.
fn retaining() -> Controller {
  let view: View = read_response(A_FORM.as_bytes(), now())
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
      next_check: now(),
      discarded: Vec::new(),
      stderr: Captured::default(),
      failure: None,
      cleanup: None,
    },
  );
  controller
}

/// **F-R3.** A debounce tick that enqueues while an exchange is in flight
/// costs the person nothing: the present that follows the exchange writes no
/// widget back, and what they typed during it is on the screen and in the
/// draft.
#[test]
fn a_tick_enqueued_during_an_exchange_survives_the_present_that_follows_it() {
  init_integration_test_with_system_time();

  // The runtime, entered for the whole of the loop's life — `serve`'s
  // `tokio::time::sleep_until` needs one (`event_loop_schedule` states the
  // same).
  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("a runtime builds in a test process");
  let _entered = runtime.enter();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  // A shown window clips, and left at its preferred size the field is
  // unreachable to the query (`event_loop_overlay/overlay.rs`).
  ComponentHandle::window(&window).set_size(slint::PhysicalSize::new(600, 600));

  // Capacity one, as production has it (`main.rs`), and one `Debounce`
  // created before both readers: `Rc::clone` into the glass and a borrow into
  // `install` (`design.md` §8 R10).
  let (tx, rx) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let stopper = cancel.clone();
  let notice = Notice::new();
  let wire = Wire::new(tx, cancel.clone(), notice.clone());
  let pending = Rc::new(Debounce::new());
  install(&window, &tray, &wire, &pending);

  let glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
    Rc::clone(&pending),
  );

  let (release, released) = oneshot::channel::<()>();
  let entered = Arc::new(AtomicUsize::new(0));
  let answered = Arc::new(AtomicUsize::new(0));
  let backend = HeldBackend {
    released: VecDeque::from(vec![released]),
    entered: Arc::clone(&entered),
    answered: Arc::clone(&answered),
  };
  let config = Config {
    // Never spawned — `HeldBackend` is the transport — but `Host` is built
    // around a `Config` and a `Config` names a command.
    backend: BackendConfig {
      command: Spawned::new("bash", vec!["-c".to_owned(), "exit 0".to_owned()]),
      timeout: Duration::from_secs(30),
    },
    schedule: ScheduleConfig {
      default_poll: jiff::SignedDuration::from_secs(3600),
    },
    ingress: None,
  };
  let host = Host::new(config, backend, now());

  let _serving = slint::spawn_local(async move {
    serve(
      host,
      retaining(),
      rx,
      cancel,
      notice,
      wall_clock,
      glass,
      Ingress::none(),
    )
    .await;
    // The crate's only `quit_event_loop` spelling, mirrored from `main.rs`.
    match slint::quit_event_loop() {
      Ok(()) | Err(_) => (),
    }
  })
  .expect("the event loop accepts the host task");

  // Recorded from inside the loop, read on the test thread once
  // `run_event_loop_until_quit` has returned — the two share the one thread
  // the testing backend runs on.
  let readings: Rc<RefCell<Vec<Reading>>> = Rc::new(RefCell::new(Vec::new()));

  let recording = Rc::clone(&readings);
  let held_at = Rc::clone(&pending);
  let stepped = window.clone_strong();
  // Moved into the stepper: `install` already holds every clone the
  // callbacks need, and nothing here uses it again.
  let sending = wire;
  let mut release = Some(release);
  let stepper = slint::Timer::default();
  let mut step = 0_u8;
  stepper.start(slint::TimerMode::Repeated, STEP, move || {
    let read = |at: &'static str| {
      recording.borrow_mut().push(Reading {
        at,
        shown: shown(&stepped, "noted"),
        held: held_at.carried().len(),
        exchanging: entered.load(Ordering::Relaxed) > answered.load(Ordering::Relaxed),
        inits: stepped.get_inits(),
        reasserts: stepped.get_reasserts(),
        epoch: stepped.get_epoch(),
      });
    };
    // **The run, at `STEP` a step against `DEBOUNCE`** — named rather than
    // restated, because the sentence that stood here said *50 ms a step* for
    // as long as it took someone to read it against `STEP`, which F-B4 had
    // halved to 25 ms (`review-code.md` F-C4). The `const` assertions above
    // bound `STEP` against `DEBOUNCE`; nothing bounds prose, so prose does not
    // carry numbers it does not own.
    //
    // Every reading is a step after the act it reports on: a pointer event is
    // hit-tested against laid-out geometry, and a present's guard writes are
    // counted when `serve` gets to them.
    //
    // ```
    //  1  —                 `serve` presents the form it was handed
    //  4  click             the field takes focus
    //  6  type FIRST        tick due at 12; `serve` is parked in the outer select
    // 14  read A            served and presented: the control
    // 16  Evaluate          the exchange starts, and the backend is held
    // 18  type SECOND       a real key event, while the exchange is in flight
    // 19  read B            the entry is in the map; the overlay still covers it
    // 28  read C            the tick fired at 24: the entry left on the enqueue
    // 30  release           the backend answers; `serve` folds and presents
    // 34  read D            the claim
    // 36  stop              and the loop ends when `serve` returns
    // ```
    //
    // **B is one step after the key and C is ten**, which is the asymmetry the
    // margins above are about: B's bound is the only one a stalled stepper
    // walks towards.
    //
    // **The whole run is a bound too, and it is the one that has actually been
    // seen to fail** (`review-code.md` F-C5). Step 1 to the last step must stay
    // under `controller::MINIMUM_SPACING` — 3 s — or `serve`'s standing timer
    // fires an unplanned evaluation into the middle of the case. Nothing
    // asserts it. F-B4's re-index kept every interval between readings the same
    // (B's excepted, halved on purpose) and still made the **run** longer, 17
    // steps at 50 ms becoming 36 at 25: measured at **~877 ms** (876.9 / 877.2
    // / 878.3 over three runs on an idle machine), against 799 ms before, so
    // the margin fell from 3.75x to 3.42x. F-B4's *"at no wall-clock
    // cost"* was true of the intervals and not of the total, which is the
    // distinction to keep: a future change to this schedule is made against
    // 876.7 ms, not against "no cost". Widening the bound or asserting the
    // total is the stepper-harness follow-up's business, not this file's.
    step += 1;
    match step {
      4 => click_line_edit(&stepped, "noted"),
      6 => key(&stepped, FIRST.into()),
      14 => read("A typed, ticked and served with nothing in flight"),
      16 => {
        sending.send(Command::Evaluate(Stimulus::Requested));
      }
      18 => key(&stepped, SECOND.into()),
      19 => read("B typed during the exchange, still held"),
      28 => read("C the tick enqueued it inside the exchange"),
      30 => {
        if let Some(release) = release.take() {
          match release.send(()) {
            Ok(()) | Err(()) => (),
          }
        }
      }
      34 => read("D the exchange folded, and the present that followed"),
      36 => stopper.stop(),
      _ => {}
    }
  });

  // The only thing standing between a step that never arrives and a hung gate.
  let stop = slint::Timer::default();
  stop.start(slint::TimerMode::SingleShot, LIVENESS_BOUND, quit);

  slint::run_event_loop_until_quit().expect("the headless loop must run, and quit when asked");

  let readings = readings.borrow();
  let [control, typed, enqueued, folded] = readings.as_slice() else {
    panic!("the stepper must have taken all four readings within {LIVENESS_BOUND:?}: {readings:?}");
  };

  // --- the control, and the arrangement ---

  assert!(
    control.inits > 0,
    "the window must have drawn the form for any count below to mean anything: {control:?}"
  );
  assert_eq!(
    control.shown, FIRST,
    "an edit typed with nothing in flight reaches the draft and stays on the \
     screen — if this fails, the driver missed the field and nothing below \
     measures what it claims: {control:?}"
  );
  assert_eq!(
    control.reasserts, 0,
    "and the present that served it wrote no widget back: {control:?}"
  );
  assert!(
    !control.exchanging,
    "nothing was in flight for the control, which is what makes it one: {control:?}"
  );

  // The narrowing of `busy`, measured through `serve`: a real key event
  // delivered while a real evaluation is outstanding.
  assert!(
    typed.exchanging,
    "the arrangement: the backend must have been entered and not yet answered \
     when the second character was typed: {typed:?}"
  );
  assert_eq!(
    typed.shown,
    format!("{FIRST}{SECOND}"),
    "a key typed while the host is mid-exchange must reach the field — before \
     `busy` was narrowed the form was disabled here and the character was \
     discarded (`review-code.md` F-A1): {typed:?}"
  );
  assert_eq!(
    typed.held, 1,
    "and the debounce must be holding it, which is what the overlay shows \
     from: {typed:?}"
  );

  // The window the defect lived in, established rather than assumed.
  assert!(
    enqueued.exchanging,
    "still mid-exchange ten steps later, and a tick is due {TICK_STEPS} steps after the key, or it did not land \
     inside one: {typed:?} then {enqueued:?}"
  );
  assert_eq!(
    enqueued.held, 0,
    "and the tick has enqueued the edit, so the map has let it go — the value \
     is now in the channel alone, held by neither the overlay nor the draft. \
     Without this reading the case would be measuring an edit the overlay \
     still covered: {typed:?} then {enqueued:?}"
  );

  // --- the claim ---

  assert_ne!(
    folded.epoch, enqueued.epoch,
    "the exchange must have folded and presented, or the counter below is at \
     rest because no guard ran: {enqueued:?} then {folded:?}"
  );
  assert_eq!(
    folded.reasserts, enqueued.reasserts,
    "and that present must have written no widget back. It is the present the \
     old loop made *before* serving the queued edit, against a draft that had \
     never seen it — two guard writes across a completion that should have \
     disturbed nothing (`review-code.md` F-R3): {enqueued:?} then {folded:?}"
  );
  assert_eq!(
    folded.shown,
    format!("{FIRST}{SECOND}"),
    "and what they typed is still on the screen — which, with no guard write \
     at that present, is the draft agreeing with it: {folded:?}"
  );
  assert_eq!(
    folded.inits, control.inits,
    "no element was destroyed to do any of it: the view was retained, so the \
     rows were never rebuilt: {control:?} then {folded:?}"
  );
}

/// The crate's `quit_event_loop` spelling, mirrored from `main.rs`: matched
/// rather than discarded, because `let _ =` trips `let_underscore_must_use`.
fn quit() {
  match slint::quit_event_loop() {
    Ok(()) | Err(_) => (),
  }
}

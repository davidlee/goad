//! The claim, in three readings:
//!
//! ```text
//!  A  the form on screen, nothing in flight        busy == false  — the control
//!  B  the person's own answer outstanding          busy == true   — the claim
//!  C  the answer landed                            busy == false
//! ```
//!
//! **What holds B to meaning something** is `entered` beside it: the backend's
//! own count of exchanges it has been handed. Without it, a click that missed
//! the button would read `busy == false` at B and the case would fail for the
//! wrong reason — and, worse, a case asserting only `busy == true` could pass
//! on an exchange the person did not start.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use goad::controller::{Controller, serve};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad::install::install;
use goad::pending::Debounce;
use goad::wire::{Cancel, Command, Notice, Stimulus, Wire};
use goad_semantics::protocol::canonical::{Request, Timestamp};
use goad_shell::backend::transport::{Backend, Captured, Exchange};
use goad_shell::clock::wall_clock;
use goad_shell::config::{BackendConfig, Command as Spawned, Config, ScheduleConfig};
use goad_shell::host::Host;
use goad_shell::ingress::Ingress;
use i_slint_backend_testing::{
  AccessibleRole, ElementHandle, ElementQuery, init_integration_test_with_system_time,
};
use slint::platform::{PointerEventButton, WindowEvent};
use slint::{ComponentHandle, LogicalPosition, VecModel};
use tokio::sync::{mpsc, oneshot};

/// One option with one field. The option's `Button` is what the person
/// presses, and pressing it is the only road to `Pending::Respond`
/// (`install.rs:40`, `controller.rs:312`).
const A_FORM: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"noted","kind":"text","label":"Anything to add?"}]}]},"next_check":"45 minutes"}"#;

/// No view, so the answer folds `Shift::Retained` and the form survives it —
/// the ordinary result of answering, and the one in which reading C is about
/// `busy` clearing rather than about the form being replaced.
const NOTHING_INSTRUCTED: &str = r#"{"view":null,"next_check":"45 minutes"}"#;

/// How often the stepper runs. The same 30 ms `event_loop_busy` and
/// `event_loop_picker` use, and nothing here is timed against a production
/// deadline: no debounce is armed and the case finishes far inside
/// `MINIMUM_SPACING`.
const STEP: Duration = Duration::from_millis(30);

/// The stop the case cannot run without: a predicate that never becomes true
/// must fail on a count rather than wedge the gate. Nothing panics from inside
/// the loop — the steps record and every assertion is made on the test thread.
const LIVENESS_BOUND: Duration = Duration::from_secs(20);

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
  /// The form's own title, so the control asserts the form is *on screen*
  /// rather than inferring it from a flag that is also false before the first
  /// present.
  heading: String,
  /// Whether the option's `Button` is findable and what it reports for
  /// `enabled` — the click below goes through hit testing, so a button that is
  /// missing and a button that is disabled must not read the same.
  button: Option<bool>,
  busy: bool,
  /// The backend's own count of exchanges it has been handed. This is what
  /// separates "the answer is in flight" from "the click missed".
  entered: usize,
  answered: usize,
}

/// A `Backend` that answers only when the case lets it — `event_loop_drain`'s,
/// unchanged, and shaped there for the same reason: what is under test is what
/// the rest of the program does *while* an exchange is outstanding, and a
/// `sleep` in a scripted backend would be that arrangement with a race in it.
struct HeldBackend {
  /// One body per exchange, in order. The first is the evaluation that puts
  /// the form on screen — **and it is what gives the host an outstanding
  /// interaction to answer**: interaction identity is the *host's*
  /// (`goad-shell`'s `StateError::NoOutstandingView`), so a view handed
  /// straight to the `Controller` is one the host never issued and a `Choose`
  /// against it is refused rather than sent.
  bodies: VecDeque<&'static str>,
  /// Held only where there is a receiver to wait on; the evaluation answers at
  /// once and the person's answer is the one this case holds.
  released: VecDeque<Option<oneshot::Receiver<()>>>,
  entered: Arc<AtomicUsize>,
  answered: Arc<AtomicUsize>,
}

impl Backend for HeldBackend {
  fn exchange(&mut self, _request: &Request) -> impl Future<Output = Exchange> + Send {
    let released = self.released.pop_front().flatten();
    let body = self.bodies.pop_front().unwrap_or(NOTHING_INSTRUCTED);
    let entered = Arc::clone(&self.entered);
    let answered = Arc::clone(&self.answered);
    async move {
      entered.fetch_add(1, Ordering::Relaxed);
      // Past the end of the script it answers at once, which stops an
      // unplanned exchange wedging the gate instead of reddening it.
      if let Some(released) = released {
        // A dropped sender releases it too: teardown, not an answer.
        match released.await {
          Ok(()) | Err(_) => (),
        }
      }
      answered.fetch_add(1, Ordering::Relaxed);
      Exchange {
        result: Ok(body.as_bytes().to_vec()),
        stderr: Captured::default(),
        cleanup: None,
      }
    }
  }
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

/// Click one element at its own centre — the three events
/// `ElementHandle::mock_single_click` dispatches, written out because that
/// helper advances mock time between press and release and so asserts
/// *"Recursion in timer code"* when it is reached from inside a timer
/// callback, which the stepper below is.
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

/// **A real pointer event, and it has to be.** The option `Button` binds
/// `enabled: !root.busy`, and `invoke_accessible_default_action` reaches
/// neither hit testing nor `enabled` — so a click raised that way would answer
/// a disabled button and the guard this case is about would be invisible.
fn click_button_described(window: &PromptWindow, description: &str) {
  if let Some(element) = button_described(window, description) {
    click(window, &element);
  }
}

fn now() -> Timestamp {
  wall_clock().expect("the real wall clock reads fine in a test process")
}

/// **F-B1.** A person's own answer, sent by a real click through the
/// production loop, is shown in flight — which is `serve` handing `engage` the
/// `Exchanged` the pending exchange actually carries.
#[test]
fn an_answer_sent_through_serve_engages_and_a_landed_one_disengages() {
  init_integration_test_with_system_time();

  // The runtime, entered for the whole of the loop's life — `serve`'s
  // `tokio::time::sleep_until` needs one.
  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("a runtime builds in a test process");
  let _entered = runtime.enter();

  let window = PromptWindow::new().expect("the headless testing backend always builds a window");
  let tray = Tray::new().expect("the headless testing backend always builds a tray");
  // A shown window clips, and left at its preferred size the button is
  // unreachable to the query.
  ComponentHandle::window(&window).set_size(slint::PhysicalSize::new(600, 600));

  // Capacity one, as production has it, and one `Debounce` created before both
  // readers (`design.md` §8 R10).
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
    bodies: VecDeque::from(vec![A_FORM, NOTHING_INSTRUCTED]),
    released: VecDeque::from(vec![None, Some(released)]),
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
      Controller::new(),
      rx,
      cancel,
      notice,
      wall_clock,
      glass,
      Ingress::none(),
    )
    .await;
    match slint::quit_event_loop() {
      Ok(()) | Err(_) => (),
    }
  })
  .expect("the event loop accepts the host task");

  let readings: Rc<RefCell<Vec<Reading>>> = Rc::new(RefCell::new(Vec::new()));

  let recording = Rc::clone(&readings);
  let stepped = window.clone_strong();
  // Moved into the stepper: `install` already holds every clone the callbacks
  // need, and nothing here uses it again.
  let sending = wire;
  let entered_at = Arc::clone(&entered);
  let answered_at = Arc::clone(&answered);
  let mut release = Some(release);
  let stepper = slint::Timer::default();
  let mut step = 0_u8;
  stepper.start(slint::TimerMode::Repeated, STEP, move || {
    let read = |at: &'static str| {
      recording.borrow_mut().push(Reading {
        at,
        heading: stepped.get_heading().to_string(),
        button: button_described(&stepped, "morning")
          .map(|element| element.accessible_enabled().unwrap_or(false)),
        busy: stepped.get_busy(),
        entered: entered_at.load(Ordering::Relaxed),
        answered: answered_at.load(Ordering::Relaxed),
      });
    };
    // Every reading is the step after the act it reports on: a pointer event
    // is hit-tested against laid-out geometry, and `serve` presents from its
    // own task rather than from this callback.
    //
    // ```
    //  2  Evaluate       a real evaluation, answered at once with the form
    //  5  read A         the control: the form is up and nothing is in flight
    //  6  click morning  a real click on the option button
    //  9  read B         the claim: their own answer is outstanding
    // 10  release        the backend answers it
    // 13  read C         and it has landed
    // 14  stop
    // ```
    step += 1;
    match step {
      2 => {
        sending.send(Command::Evaluate(Stimulus::Requested));
      }
      5 => read("A the form on screen, nothing in flight"),
      6 => click_button_described(&stepped, "morning"),
      9 => read("B the person's own answer outstanding"),
      10 => {
        if let Some(release) = release.take() {
          match release.send(()) {
            Ok(()) | Err(()) => (),
          }
        }
      }
      13 => read("C the answer landed"),
      14 => stopper.stop(),
      _ => {}
    }
  });

  // The only thing standing between a step that never arrives and a hung gate.
  let stop = slint::Timer::default();
  stop.start(
    slint::TimerMode::SingleShot,
    LIVENESS_BOUND,
    || match slint::quit_event_loop() {
      Ok(()) | Err(_) => (),
    },
  );

  slint::run_event_loop_until_quit().expect("the headless loop must run, and quit when asked");

  let readings = readings.borrow();
  let [idle, outstanding, landed] = readings.as_slice() else {
    panic!(
      "the stepper must have taken all three readings within {LIVENESS_BOUND:?}: {readings:?}"
    );
  };

  // The control first. A case in which it did not hold would be measuring its
  // own driver.
  assert!(
    idle.heading == "Proceed?"
      && idle.button == Some(true)
      && !idle.busy
      && idle.entered == 1
      && idle.answered == 1,
    "the arrangement: one evaluation has been and gone, its form is on screen \
     with its option button live, and `busy` is false because nothing is in \
     flight: {idle:?}"
  );
  // **What this control does not hold, said here so the next reader does not
  // have to measure it** (`review-code.md` F-C1). It is tempting to read the
  // `!idle.busy` above as the narrowing's other half — that the evaluation
  // which put this form up did *not* engage. It is not. This reading is taken
  // after the evaluation has landed (`entered: 1, answered: 1`), and `absorb`
  // clears `busy` on landing whatever engaged it, so the engage is already
  // gone and `!idle.busy` is true either way. Measured: making every exchange
  // engage leaves this target green.
  //
  // The property is real and is held by `event_loop_drain`, whose arrangement
  // reads `busy` while an exchange is still outstanding — which is where a
  // spurious engage is visible. A reading that earned the sentence would have
  // to be taken there, not here.

  // The arrangement for the claim, and it is what stops the claim passing for
  // the wrong reason: the click must actually have reached the button and
  // started an exchange that is still outstanding.
  assert!(
    outstanding.entered == 2 && outstanding.answered == 1,
    "a real click on the option button must have started exactly one exchange \
     and it must still be outstanding, or the assertion below is about a \
     window nothing was asked of: {idle:?} then {outstanding:?}"
  );

  // The claim.
  assert!(
    outstanding.busy,
    "and `serve` must engage for it — the exchange it started is the person's \
     own answer, and `busy` is what disables the option button that would send \
     a second one (slice 003's double-submit guard, AC-4). `serve` supplies the \
     `Exchanged` that `engage` maps, and hardcoding `Evaluation` there leaves \
     every other target in this crate green: {outstanding:?}"
  );

  assert!(
    landed.answered == 2 && !landed.busy,
    "and it must clear once the answer has landed: {outstanding:?} then {landed:?}"
  );
}

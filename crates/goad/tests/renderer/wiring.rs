//! design.md §9 item 11: every wiring surface reachable with no component and
//! no event loop is not this file's claim — 11e (local refusals), 11f (the
//! five DT transitions), 11g (back-pressure) and 11i (`busy` clearing) are,
//! and PHASE-07 built them with no `serve` and no loop. PHASE-10 adds item
//! 11a-d and 11h (the reducer's seven rows and the one production `serve`,
//! `mod rows`/`interaction`/`serving`) and item 14a-d (cancellation,
//! `mod cancellation`) — the surfaces that need `serve` to exist.
//!
//! `#[cfg(test)]` on the declaration, not on the file, for the same reason
//! every other module here carries it (`clippy::tests_outside_test_module`).

use std::rc::Rc;
use std::time::Duration;

use goad::clock::ClockError;
use goad::controller::{Controller, Exchanged, Surface};
use goad::diagnostics::{BUSY_NOTICE, Refused};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::{Glass, SlintGlass};
use goad::wire::{Cancel, Command, Stimulus, Wire};
use goad_semantics::protocol::canonical::Timestamp;
use i_slint_backend_testing::{ElementHandle, ElementQuery, init_no_event_loop};
use slint::{ComponentHandle, Model, VecModel};
use tokio::sync::mpsc;

use crate::driving::{host, instant, invocations, quiet_event, scripted};

const TIMEOUT: Duration = Duration::from_secs(2);

fn now() -> Timestamp {
  instant("2026-01-01T00:00:00Z")
}

/// A `Clock` (`fn() -> Result<Timestamp, ClockError>`) fixed to [`now`]. A
/// plain top-level `fn`, not a closure: `Clock` is a `fn` pointer type
/// (`clock.rs`), the same reason `serve` itself takes one rather than a
/// `dyn Fn`. Shared by `mod serving`, `mod interaction` and `mod
/// cancellation`, all of which call `serve` directly.
#[expect(
  clippy::unnecessary_wraps,
  reason = "must match `Clock`'s `fn() -> Result<Timestamp, ClockError>` \
    signature to be passed to `serve`; a test fixture never needs to \
    exercise the error arm (test code, outside VA-3's src/-only budget)"
)]
fn stub_clock() -> Result<Timestamp, ClockError> {
  Ok(now())
}

/// Two named options, so a `Choose` can name the wrong one (VT-5) or the
/// right one, and so `busy`'s controls (VT-9) have something to be
/// enabled or disabled.
const TWO_OPTIONS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"yes","label":"Yes"},{"id":"no","label":"No"}]},"next_check":"45 minutes"}"#;
/// A second, distinct view — for DT-2, replacing the one above.
const A_SECOND_VIEW: &str = r#"{"view":{"kind":"choice","title":"Still there?","options":[{"id":"ok","label":"OK"}]},"next_check":"45 minutes"}"#;
/// A failure with no view — an unsupported protocol version, the same
/// shape `table.rs`'s `UNSUPPORTED_VERSION` uses.
const A_PROTOCOL_FAILURE: &str = r#"{"protocol":2,"view":null}"#;
/// A successful exchange with nothing new to show — `table.rs`'s
/// `ACCEPTS_A`, restated: `view: null` with no failure. What `view: null`
/// means for the outstanding interaction depends on which entry point
/// produced it (VT-2, AC-6) — this fixture is silent on that; the caller
/// picks `evaluate` or `respond`.
const CLEAN_NO_VIEW: &str = r#"{"view":null,"next_check":"90 minutes"}"#;

fn window_and_tray() -> (PromptWindow, Tray) {
  init_no_event_loop();
  (
    PromptWindow::new().expect("a headless window must construct"),
    Tray::new().expect("a headless tray must construct"),
  )
}

fn glass_over(window: &PromptWindow, tray: &Tray) -> SlintGlass {
  SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
  )
}

fn in_diagnostic_mode(window: &PromptWindow) -> bool {
  ElementHandle::find_by_accessible_label(window, "diagnostics")
    .next()
    .is_some()
}

fn in_prompt_mode(window: &PromptWindow) -> bool {
  ElementHandle::find_by_accessible_label(window, "options")
    .next()
    .is_some()
}

fn nothing_to_report_shown(window: &PromptWindow) -> bool {
  ElementHandle::find_by_accessible_label(window, "Nothing to report.")
    .next()
    .is_some()
}

fn accessible_enabled_of(window: &PromptWindow, description: &str) -> Option<bool> {
  let description = description.to_owned();
  ElementQuery::from_root(window)
    .match_predicate(move |element| {
      element.accessible_description().as_deref() == Some(description.as_str())
    })
    .find_first()
    .and_then(|element| element.accessible_enabled())
}

/// AC-6's "no window" / "the window follows the interaction", read off the
/// window itself rather than `Controller::frame().surface` — `Surface::Prompt`
/// is set whenever the mode is not `Diagnostic`, view or no view, so it alone
/// cannot distinguish "shown" from "hidden" the way `show()`/`hide()` does
/// (PHASE-10 repair, VT-2/VT-3).
fn window_shown(window: &PromptWindow) -> bool {
  window.window().is_visible()
}

/// The view token a real click would carry, read off the options model
/// exactly as `app.slint`'s `chosen` callback does (`option.view`) — so a
/// test builds a `Command::Choose` from what the window actually holds,
/// never from a second, independent minting of the same value (PHASE-10
/// repair, VT-4).
fn current_view_token(window: &PromptWindow) -> Option<String> {
  window
    .get_options()
    .row_data(0)
    .map(|row| row.view.to_string())
}

/// Poll `predicate` on a short fixed interval until it is true, panicking if
/// it never is within `bound`. The one shape every `serve`-driven test needs
/// to observe an event the driving code does not control directly — an
/// invocation landing, a view arriving — rather than assume a fixed delay
/// covers it (PHASE-10 repair, VT-4/VT-10).
async fn until(bound: Duration, mut predicate: impl FnMut() -> bool) {
  let deadline = std::time::Instant::now() + bound;
  loop {
    if predicate() {
      return;
    }
    assert!(
      std::time::Instant::now() < deadline,
      "condition did not become true within {bound:?}"
    );
    tokio::time::sleep(Duration::from_millis(5)).await;
  }
}

/// VT-5 — item 11e (AC-6, F-13). Both refusals: no backend contact, the
/// presentation retained, one diagnostic line.
mod refusals {
  use super::{
    Controller, Exchanged, Refused, TIMEOUT, TWO_OPTIONS, host, invocations, now, quiet_event,
    scripted,
  };

  #[tokio::test]
  async fn an_unknown_option_is_refused_with_no_backend_contact_and_a_retained_presentation() {
    let (command, log) = scripted("wiring-unknown-option", &[TWO_OPTIONS]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    let view = controller
      .frame()
      .shown
      .expect("a view must be retained")
      .view_id
      .as_str()
      .to_owned();
    let contacted_before = invocations(&log);

    let refusal = controller
      .answer(&view, "not-an-option")
      .expect_err("an option the presentation does not carry must be refused");
    assert!(matches!(refusal, Refused::UnknownOption { .. }));
    controller.refuse(&refusal);

    let frame = controller.frame();
    assert!(frame.shown.is_some(), "the presentation must be retained");
    assert_eq!(frame.diagnostics.lines().len(), 1);
    assert_eq!(
      invocations(&log),
      contacted_before,
      "a local refusal must not contact the backend"
    );
  }

  #[tokio::test]
  async fn a_broken_clock_is_refused_with_no_backend_contact_and_a_retained_presentation() {
    let (command, log) = scripted("wiring-no-clock", &[TWO_OPTIONS]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    let contacted_before = invocations(&log);

    controller.refuse(&Refused::NoClock {
      detail: "the system clock could not be read".to_owned(),
    });

    let frame = controller.frame();
    assert!(frame.shown.is_some(), "the presentation must be retained");
    assert_eq!(frame.diagnostics.lines().len(), 1);
    assert_eq!(
      invocations(&log),
      contacted_before,
      "a local refusal must not contact the backend"
    );
  }
}

/// VT-6 — item 11f. The five DT transitions (design.md §5.4), read as
/// `Surface` from the frame and as the element tree.
mod transitions {
  use i_slint_backend_testing::ElementHandle;

  use super::{
    A_PROTOCOL_FAILURE, A_SECOND_VIEW, Controller, Exchanged, Glass, Surface, TIMEOUT, TWO_OPTIONS,
    accessible_enabled_of, glass_over, host, in_diagnostic_mode, in_prompt_mode,
    nothing_to_report_shown, now, quiet_event, scripted, window_and_tray,
  };

  /// DT-1 and DT-5 together: a clean outcome clears diagnostics and the
  /// tray, but a window a person opened stays open on
  /// "Nothing to report." — the one case the tray (now) and the window
  /// (what happened) are allowed to disagree.
  #[tokio::test]
  async fn dt1_a_clean_outcome_under_diagnostic_mode_clears_lines_but_leaves_the_window_open() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("wiring-dt1", &[A_PROTOCOL_FAILURE]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    controller.open_diagnostics();
    glass.present(controller.frame());
    assert!(in_diagnostic_mode(&window));
    assert!(!nothing_to_report_shown(&window), "a fault line is present");
    assert_eq!(
      controller.frame().diagnostics.state(),
      goad::diagnostics::TrayState::Fault
    );

    let cleared = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, cleared);
    glass.present(controller.frame());

    assert_eq!(
      controller.frame().surface,
      Surface::Diagnostics,
      "Focus::Diagnostics is untouched by a Retained fold"
    );
    assert!(
      in_diagnostic_mode(&window),
      "the window a person opened stays open"
    );
    assert!(
      nothing_to_report_shown(&window),
      "diagnostics must clear on a clean outcome"
    );
    assert_eq!(
      controller.frame().diagnostics.state(),
      goad::diagnostics::TrayState::Idle,
      "the tray returns to idle immediately — disagreeing with the still-open window (DT-5)"
    );
  }

  /// DT-2: an incoming view under diagnostic mode outranks the record —
  /// `Focus` clears and the presentation is replaced.
  #[tokio::test]
  async fn dt2_a_new_view_under_diagnostic_mode_returns_to_prompt_mode() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("wiring-dt2", &[TWO_OPTIONS, A_SECOND_VIEW]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    controller.open_diagnostics();
    glass.present(controller.frame());
    assert!(in_diagnostic_mode(&window));

    let replacing = backend.evaluate(now(), quiet_event(now())).await;
    let shift = controller.absorb(Exchanged::Evaluation, replacing);
    glass.present(controller.frame());

    assert_eq!(shift, goad::controller::Shift::Replaced);
    assert_eq!(controller.frame().surface, Surface::Prompt);
    assert!(in_prompt_mode(&window));
    assert_eq!(window.get_heading(), "Still there?");
    let options_list = ElementHandle::find_by_accessible_label(&window, "options")
      .next()
      .expect("the options list container must be found");
    assert_eq!(
      options_list.accessible_item_count(),
      Some(1),
      "the presentation is replaced, not merged: one option, not three"
    );
  }

  /// DT-3: a failure while the prompt is visible changes no mode — the
  /// question stays, the tray goes to fault.
  #[tokio::test]
  async fn dt3_a_failure_under_prompt_mode_leaves_the_question_in_place() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("wiring-dt3", &[TWO_OPTIONS, A_PROTOCOL_FAILURE]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());
    assert_eq!(window.get_heading(), "Proceed?");

    let failing = backend.evaluate(now(), quiet_event(now())).await;
    let shift = controller.absorb(Exchanged::Evaluation, failing);
    glass.present(controller.frame());

    assert_eq!(shift, goad::controller::Shift::Retained);
    assert_eq!(controller.frame().surface, Surface::Prompt);
    assert_eq!(
      window.get_heading(),
      "Proceed?",
      "the question did not go away"
    );
    assert!(in_prompt_mode(&window));
    assert_eq!(accessible_enabled_of(&window, "yes"), Some(true));
    assert_eq!(
      controller.frame().diagnostics.state(),
      goad::diagnostics::TrayState::Fault
    );
  }

  /// DT-4: `close-diagnostics()` returns to the retained prompt intact.
  #[tokio::test]
  async fn dt4_leaving_diagnostic_mode_restores_the_retained_prompt_intact() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("wiring-dt4", &[TWO_OPTIONS]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    controller.open_diagnostics();
    glass.present(controller.frame());
    assert!(in_diagnostic_mode(&window));

    controller.close_diagnostics();
    glass.present(controller.frame());

    assert_eq!(controller.frame().surface, Surface::Prompt);
    assert!(in_prompt_mode(&window));
    assert_eq!(
      window.get_heading(),
      "Proceed?",
      "the retained prompt's own heading"
    );
  }
}

/// VT-7 — item 11g (F-5). A second command sent while the channel is full
/// sets `notice` and does not enter `Diagnostics`; the next `present`
/// clears it.
mod back_pressure {
  use slint::ComponentHandle;

  use super::{
    BUSY_NOTICE, Cancel, Command, Controller, Glass, Stimulus, Wire, glass_over, mpsc,
    window_and_tray,
  };

  #[test]
  fn a_full_channel_sets_notice_and_the_next_present_clears_it() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (tx, held_open) = mpsc::channel::<Command>(1);
    tx.try_send(Command::Evaluate(Stimulus::Requested))
      .expect("the first send must have room in a fresh capacity-1 channel");
    let wire = Wire::new(tx, Cancel::new(), window.as_weak());

    wire.send(Command::OpenDiagnostics);

    assert_eq!(
      window.get_notice(),
      BUSY_NOTICE,
      "a command sent into a full channel must report BUSY_NOTICE, not vanish"
    );

    let controller = Controller::new();
    assert!(
      controller.frame().diagnostics.is_clear(),
      "back-pressure must not enter Diagnostics"
    );
    glass.present(controller.frame());

    assert_eq!(
      window.get_notice(),
      "",
      "the next present must clear notice (design.md §5.3)"
    );
    drop(held_open); // keeps the receiver alive until here, deliberately unread
  }
}

/// VT-9 — item 11i (F-21). `busy` returns to `false` on both outcomes, and
/// the option controls read `accessible_enabled == true` in the element
/// tree. VA-2's negative control is the one that matters: an `absorb`
/// that does not clear `engaged` leaves every control disabled.
mod busy {
  use super::{
    A_PROTOCOL_FAILURE, Controller, Exchanged, Glass, TIMEOUT, TWO_OPTIONS, accessible_enabled_of,
    glass_over, host, now, quiet_event, scripted, window_and_tray,
  };

  #[tokio::test]
  async fn busy_clears_and_controls_re_enable_after_a_success() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("wiring-busy-success", &[TWO_OPTIONS]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    controller.engage();
    glass.present(controller.frame());
    assert!(window.get_busy(), "the exchange must be shown in flight");

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());

    assert!(!controller.frame().busy);
    assert!(!window.get_busy());
    assert_eq!(accessible_enabled_of(&window, "yes"), Some(true));
    assert_eq!(accessible_enabled_of(&window, "no"), Some(true));
  }

  #[tokio::test]
  async fn busy_clears_and_controls_re_enable_after_a_failure() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("wiring-busy-failure", &[TWO_OPTIONS, A_PROTOCOL_FAILURE]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());

    controller.engage();
    glass.present(controller.frame());
    assert!(window.get_busy());

    let failing = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, failing);
    glass.present(controller.frame());

    assert!(!controller.frame().busy);
    assert!(!window.get_busy());
    assert_eq!(accessible_enabled_of(&window, "yes"), Some(true));
    assert_eq!(accessible_enabled_of(&window, "no"), Some(true));
  }
}

/// VT-1 — item 11a (AC-7). design.md §5.4's seven-row reducer table
/// (`Exchanged`, whether a view came back, whether a failure came with it),
/// each row's `Shift` read straight off `Controller::absorb`'s fold.
///
/// Rows 1-4 and 6 are driven by a real backend (`tokio::process::Command`);
/// row 5 and row 7 are, per the design's own text, unreachable *through the
/// controller* — the renderer's token and `Host`'s state are written in the
/// same fold and cannot diverge — and are asserted on a constructed
/// `Outcome`, still folded through the production `absorb`.
mod rows {
  use goad::controller::{Controller, Exchanged, Shift};
  use goad::glass::Glass;
  use goad_semantics::protocol::canonical::ViewId;
  use goad_semantics::protocol::normalize::read_response;
  use goad_shell::backend::transport::Captured;
  use goad_shell::error::{BackendError, StateError};
  use goad_shell::host::{Failure, Outcome, Presented};

  use super::{
    A_PROTOCOL_FAILURE, CLEAN_NO_VIEW, TIMEOUT, TWO_OPTIONS, current_view_token, glass_over, host,
    invocations, now, quiet_event, scripted, window_and_tray, window_shown,
  };

  /// Row 1: either entry point, a view in hand, no failure — `Replaced`,
  /// whatever was on the glass before is dropped whole. Read off the window
  /// a real glass presented, not `Controller::frame().shown` (PHASE-10
  /// repair).
  #[tokio::test]
  async fn row_1_a_view_in_hand_always_replaces() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("row-1", &[TWO_OPTIONS]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    let shift = controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());

    assert_eq!(shift, Shift::Replaced);
    assert!(window_shown(&window));
    assert_eq!(window.get_heading(), "Proceed?");
  }

  /// Row 2: `Evaluation`, no view, no failure — nothing to add, `Retained`.
  #[tokio::test]
  async fn row_2_an_evaluation_with_nothing_new_is_retained() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("row-2", &[CLEAN_NO_VIEW]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    let shift = controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());

    assert_eq!(shift, Shift::Retained);
    assert!(!window_shown(&window), "nothing was ever shown");
  }

  /// Row 3: `Answer`, no view, no failure — the answer was taken and there
  /// is nothing further to show, so the interaction `Closed`.
  #[tokio::test]
  async fn row_3_an_answer_with_nothing_further_closes() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("row-3", &[TWO_OPTIONS, CLEAN_NO_VIEW]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let presented = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, presented);
    glass.present(controller.frame());
    let view = current_view_token(&window).expect("a view must be retained");
    let (view_id, answer) = controller
      .answer(&view, "yes")
      .expect("the retained option must answer");

    let outcome = backend.respond(now(), view_id, answer).await;
    let shift = controller.absorb(Exchanged::Answer, outcome);
    glass.present(controller.frame());

    assert_eq!(shift, Shift::Closed);
    assert!(!window_shown(&window));
  }

  /// Row 4: `Evaluation`, no view, a failure — nothing to add and nothing
  /// to blame the person for, `Retained`.
  #[tokio::test]
  async fn row_4_an_evaluation_with_a_failure_and_no_view_is_retained() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("row-4", &[A_PROTOCOL_FAILURE]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    let shift = controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());

    assert_eq!(shift, Shift::Retained);
    assert!(!window_shown(&window));
  }

  /// Row 5: `Answer`, no view, `Failure::State` — not reachable through the
  /// controller, because `answer()` refuses a stale or absent view before a
  /// backend is ever contacted (that is `Refused::SupersededView`, a
  /// renderer-local refusal, not this row). Constructed directly, still
  /// folded through the production `absorb`, exactly as design.md directs.
  #[tokio::test]
  async fn row_5_an_answer_refused_by_host_state_is_retained_with_no_backend_contact() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (_command, log) = scripted("row-5", &[]);
    let mut controller = Controller::new();
    let outcome = Outcome {
      view: None,
      next_check: now(),
      discarded: Vec::new(),
      stderr: Captured::default(),
      failure: Some(Failure::State(StateError::NoOutstandingView {
        named: ViewId::new("row-5"),
      })),
      cleanup: None,
    };

    let shift = controller.absorb(Exchanged::Answer, outcome);
    glass.present(controller.frame());

    assert_eq!(shift, Shift::Retained);
    assert!(!window_shown(&window));
    assert_eq!(
      invocations(&log),
      0,
      "row 5 is constructed, not driven — the backend is never spawned"
    );
  }

  /// Row 6: `Answer`, no view, `Failure::Backend` — the same `Retained`
  /// shift as row 5, from the other stratum (design.md: "one `Shift` and two
  /// tests, because the diagnostics differ and item 11 must show both").
  #[tokio::test]
  async fn row_6_an_answer_with_a_backend_failure_and_no_view_is_retained() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("row-6", &[TWO_OPTIONS, A_PROTOCOL_FAILURE]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let presented = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, presented);
    glass.present(controller.frame());
    let view = current_view_token(&window).expect("a view must be retained");
    let (view_id, answer) = controller
      .answer(&view, "yes")
      .expect("the retained option must answer");

    let outcome = backend.respond(now(), view_id, answer).await;
    let shift = controller.absorb(Exchanged::Answer, outcome);
    glass.present(controller.frame());

    assert_eq!(shift, Shift::Retained);
    assert!(window_shown(&window), "the prior presentation stays");
    assert_eq!(window.get_heading(), "Proceed?");
  }

  /// Row 7: either entry point, a view **and** a failure together —
  /// unreachable in production (`accept` never mints a view alongside a
  /// failure), and written as `Replaced` rather than a panic on a value the
  /// host itself produced. The view is a real parse of `TWO_OPTIONS`
  /// (`read_response`, the same function a live exchange uses), not a
  /// hand-built `Choice` — `Choice`'s fields are private to `canonical.rs`
  /// on purpose (I14).
  #[tokio::test]
  async fn row_7_a_view_and_a_failure_together_still_replaces() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let now = now();
    let parsed =
      read_response(TWO_OPTIONS.as_bytes(), now).expect("the fixture is a valid response");
    let view = parsed
      .value
      .view()
      .cloned()
      .expect("the fixture carries a view");
    let mut controller = Controller::new();
    let outcome = Outcome {
      view: Some(Presented {
        view_id: ViewId::new("row-7"),
        view,
      }),
      next_check: now,
      discarded: Vec::new(),
      stderr: Captured::default(),
      failure: Some(Failure::Backend(BackendError::ExitStatus { code: Some(1) })),
      cleanup: None,
    };

    let shift = controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());

    assert_eq!(shift, Shift::Replaced);
    assert!(window_shown(&window));
    assert_eq!(window.get_heading(), "Proceed?");
  }
}

/// VT-2 — item 11b, the row AC-6 turns on (F-14). A successful `respond`
/// with `view: None` closes the interaction and the window goes away; a
/// successful `evaluate` with `view: None` while an interaction is
/// outstanding leaves the question on screen. Both, in one test, or AC-6 is
/// being asserted in the shape F-14 showed wrong.
///
/// VT-3 — item 11c. A failed `respond` keeps the window, and a retry on the
/// same `ViewId` then succeeds.
///
/// VT-4 — item 11d (R-33). A `Choose` bearing a superseded view's token is
/// refused with no backend contact and the invocation log unmoved; the
/// negative control is the same sequence with no intervening `evaluate`,
/// where the click is answered.
mod interaction {
  use std::time::Duration;

  use goad::controller::{Controller, Ending, Exchanged, Shift, serve};
  use goad::glass::Glass;
  use goad::wire::{Cancel, Command, Stimulus};
  use tokio::sync::mpsc;
  use tokio::task::LocalSet;

  use super::{
    A_PROTOCOL_FAILURE, CLEAN_NO_VIEW, TIMEOUT, TWO_OPTIONS, current_view_token, glass_over, host,
    in_prompt_mode, invocations, now, quiet_event, scripted, stub_clock, until, window_and_tray,
    window_shown,
  };

  /// VT-2 / AC-6, both halves. Read off a real `SlintGlass`'s window — "no
  /// window" and "the question on screen" are element-tree claims, not
  /// `Controller::frame().surface` (PHASE-10 repair).
  #[tokio::test]
  async fn view_null_follows_the_interaction_not_the_message() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("vt2", &[TWO_OPTIONS, CLEAN_NO_VIEW, CLEAN_NO_VIEW]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());
    assert!(window_shown(&window), "the question must be on screen");
    assert!(in_prompt_mode(&window));
    assert_eq!(window.get_heading(), "Proceed?");

    // Half one: an `evaluate` returning `view: null` while a question is
    // outstanding leaves it exactly as it was.
    let cleared = backend.evaluate(now(), quiet_event(now())).await;
    assert_eq!(
      controller.absorb(Exchanged::Evaluation, cleared),
      Shift::Retained
    );
    glass.present(controller.frame());
    assert!(
      window_shown(&window),
      "an evaluate returning view:null must not close an outstanding question"
    );
    assert!(in_prompt_mode(&window));
    assert_eq!(
      window.get_heading(),
      "Proceed?",
      "the question did not move"
    );

    // Half two: a `respond` returning `view: null` closes the interaction —
    // the answer was taken and there is nothing further to show.
    let view = current_view_token(&window).expect("half one must not have cleared it");
    let (view_id, answer) = controller
      .answer(&view, "yes")
      .expect("the retained option must answer");
    let closing = backend.respond(now(), view_id, answer).await;

    assert_eq!(controller.absorb(Exchanged::Answer, closing), Shift::Closed);
    glass.present(controller.frame());
    assert!(
      !window_shown(&window),
      "a respond returning view:null must leave goad with no window (AC-6)"
    );
  }

  /// VT-3.
  #[tokio::test]
  async fn a_failed_respond_keeps_the_window_and_a_retry_on_the_same_view_succeeds() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("vt3", &[TWO_OPTIONS, A_PROTOCOL_FAILURE, CLEAN_NO_VIEW]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame());
    let view = current_view_token(&window).expect("a view must be retained");

    let (view_id, answer) = controller
      .answer(&view, "yes")
      .expect("the retained option must answer");
    let failing = backend.respond(now(), view_id, answer).await;
    assert_eq!(
      controller.absorb(Exchanged::Answer, failing),
      Shift::Retained
    );
    glass.present(controller.frame());
    assert!(
      window_shown(&window),
      "the window stays after a failed respond"
    );
    assert!(in_prompt_mode(&window));
    assert_eq!(
      window.get_heading(),
      "Proceed?",
      "the question did not go away"
    );

    let (retry_view_id, retry_answer) = controller
      .answer(&view, "yes")
      .expect("the same view answers again");
    let succeeding = backend.respond(now(), retry_view_id, retry_answer).await;

    assert_eq!(
      controller.absorb(Exchanged::Answer, succeeding),
      Shift::Closed,
      "the retry succeeds"
    );
    glass.present(controller.frame());
    assert!(
      !window_shown(&window),
      "the retry's success finally closes the interaction"
    );
  }

  /// VT-4, positive: the queued click names a view an intervening `evaluate`
  /// has already superseded — R-33's staleness, driven through the real
  /// `mpsc` channel and the production `serve`, not a direct
  /// `Controller::absorb` fold (PHASE-10 repair). View **A** goes on screen;
  /// a slow exchange is sent and, while it is provably still in flight (the
  /// backend's own invocation observed via the log — not a fixed delay), a
  /// `Choose` naming view A's token is queued behind it; the slow exchange
  /// lands view **B**; the queued click is then dequeued and refused as
  /// `Refused::SupersededView`, read off the tray tooltip the production
  /// glass wrote and off `Served.controller`'s retained diagnostics.
  #[tokio::test]
  async fn a_click_naming_a_superseded_view_is_refused_with_no_backend_contact() {
    let (window, tray) = window_and_tray();
    let glass = glass_over(&window, &tray);
    let (command, log) = scripted("vt4-positive", &[TWO_OPTIONS, "@slow-view"]);
    let backend = host(command, TIMEOUT, now());
    let controller = Controller::new();
    let (tx, rx) = mpsc::channel::<Command>(2);
    let cancel = Cancel::new();
    let stopper = cancel.clone();

    let local = LocalSet::new();
    let served = local
      .run_until(async {
        let handle = tokio::task::spawn_local(async move {
          serve(backend, controller, rx, cancel, stub_clock, glass).await
        });

        tx.send(Command::Evaluate(Stimulus::Requested))
          .await
          .expect("the channel must accept the first send");
        until(Duration::from_secs(2), || {
          window.get_heading() == "Proceed?"
        })
        .await;
        let stale_view = current_view_token(&window).expect("view A must be on screen");

        // The intervening evaluate: a slow exchange that will land view B
        // while the queued click still names view A.
        tx.send(Command::Evaluate(Stimulus::Requested))
          .await
          .expect("the channel must accept the second send");
        until(Duration::from_secs(2), || invocations(&log) >= 2).await;

        tx.send(Command::Choose {
          view: stale_view,
          option: "yes".to_owned(),
        })
        .await
        .expect("the channel must accept the queued click");

        // The refusal, read off the tray tooltip the production glass wrote
        // — the element tree, not a peek at the controller mid-flight.
        until(Duration::from_secs(2), || {
          tray.get_hover_text().contains("since been replaced")
        })
        .await;
        assert_eq!(
          window.get_heading(),
          "Still there?",
          "the intervening evaluate's view B must have landed"
        );

        stopper.stop();
        handle.await.expect("serve must not panic")
      })
      .await;

    assert_eq!(served.ending, Ending::Stopped);
    assert_eq!(
      invocations(&log),
      2,
      "the superseded click must never reach the backend, and the invocation log must not advance"
    );
    assert!(
      served
        .controller
        .frame()
        .diagnostics
        .lines()
        .iter()
        .any(|line| line.contains("since been replaced")),
      "the retained diagnostic must be the superseded-answer line"
    );
  }

  /// VT-4, negative control: the same sequence with no intervening
  /// `evaluate` — the click names the view that is still retained, and it is
  /// answered. Through `serve` and the real channel, exactly as the positive
  /// case (PHASE-10 repair).
  #[tokio::test]
  async fn the_negative_control_with_no_intervening_evaluate_the_click_is_answered() {
    let (window, tray) = window_and_tray();
    let glass = glass_over(&window, &tray);
    let (command, log) = scripted("vt4-negative", &[TWO_OPTIONS, CLEAN_NO_VIEW]);
    let backend = host(command, TIMEOUT, now());
    let controller = Controller::new();
    let (tx, rx) = mpsc::channel::<Command>(2);
    let cancel = Cancel::new();
    let stopper = cancel.clone();

    let local = LocalSet::new();
    let served = local
      .run_until(async {
        let handle = tokio::task::spawn_local(async move {
          serve(backend, controller, rx, cancel, stub_clock, glass).await
        });

        tx.send(Command::Evaluate(Stimulus::Requested))
          .await
          .expect("the channel must accept the first send");
        until(Duration::from_secs(2), || {
          window.get_heading() == "Proceed?"
        })
        .await;
        let view = current_view_token(&window)
          .expect("with no intervening evaluate the retained view is still on screen");

        tx.send(Command::Choose {
          view,
          option: "yes".to_owned(),
        })
        .await
        .expect("the channel must accept the click");
        until(Duration::from_secs(2), || !window_shown(&window)).await;

        stopper.stop();
        handle.await.expect("serve must not panic")
      })
      .await;

    assert_eq!(served.ending, Ending::Stopped);
    assert_eq!(
      invocations(&log),
      2,
      "the click did reach the backend, unlike the positive control"
    );
    assert!(
      !window_shown(&window),
      "the answered click closed the interaction"
    );
  }
}

/// VT-8 — item 11h. One `serve`, no duplicate: the test calls `serve` —
/// the same function `main` wraps — so a loop-body change cannot pass here
/// and fail in production.
mod serving {
  use goad::controller::{Controller, Ending, serve};
  use goad::wire::{Cancel, Command, Stimulus};
  use tokio::sync::mpsc;

  use super::{
    TIMEOUT, TWO_OPTIONS, glass_over, host, in_prompt_mode, now, scripted, stub_clock,
    window_and_tray,
  };

  #[tokio::test]
  async fn serve_drives_one_exchange_through_the_production_loop() {
    let (window, tray) = window_and_tray();
    let glass = glass_over(&window, &tray);
    let (command, _log) = scripted("vt8", &[TWO_OPTIONS]);
    let backend = host(command, TIMEOUT, now());
    let controller = Controller::new();
    let (tx, rx) = mpsc::channel::<Command>(1);
    tx.try_send(Command::Evaluate(Stimulus::Requested))
      .expect("room in a fresh capacity-1 channel");
    drop(tx); // closes the channel once the one command is dequeued

    let served = serve(backend, controller, rx, Cancel::new(), stub_clock, glass).await;

    assert_eq!(served.ending, Ending::Closed);
    assert!(served.controller.frame().shown.is_some());
    assert_eq!(window.get_heading(), "Proceed?");
    assert!(in_prompt_mode(&window));
  }
}

/// Item 14, cheap tier, with `serve` under `block_on` (design.md §5.4). What
/// AC-12 can honestly observe is what the host holds; that the child is gone
/// is explicitly **not** asserted (§5.4, R-48).
///
/// `serve`'s future is `!Send` (generic over `B` and `G`), so driving it
/// concurrently with the test's own `Cancel::stop()` call needs
/// `tokio::task::LocalSet` — `tokio::spawn` requires `Send` and does not
/// apply here.
mod cancellation {
  use std::time::{Duration, Instant};

  use goad::controller::{Controller, Ending, serve};
  use goad::wire::{Cancel, Command, Stimulus};
  use tokio::sync::mpsc;
  use tokio::task::LocalSet;

  use super::{
    TIMEOUT, glass_over, host, invocations, now, scripted, stub_clock, until, window_and_tray,
  };

  /// VT-10 — item 14a. With an exchange in flight against `@hang` and a
  /// **2 s** configured timeout (`TIMEOUT`), tripping `Cancel` ends the task
  /// in **under 250 ms**, measured from the `Cancel::stop()` call to `serve`
  /// returning.
  ///
  /// VT-11 — item 14b, in the same run: `serve` **returns** a `Served`
  /// (rather than the task hanging or panicking), so the exchange future was
  /// dropped rather than abandoned unpolled.
  #[tokio::test]
  async fn tripping_cancel_mid_exchange_ends_serve_well_under_the_timeout() {
    let (window, tray) = window_and_tray();
    let glass = glass_over(&window, &tray);
    let (command, log) = scripted("vt10", &["@hang"]);
    let backend = host(command, TIMEOUT, now());
    let controller = Controller::new();
    let (tx, rx) = mpsc::channel::<Command>(1);
    tx.try_send(Command::Evaluate(Stimulus::Requested))
      .expect("room in a fresh capacity-1 channel");
    let cancel = Cancel::new();
    let stopper = cancel.clone();

    let local = LocalSet::new();
    let (elapsed, served) = local
      .run_until(async {
        let handle = tokio::task::spawn_local(async move {
          serve(backend, controller, rx, cancel, stub_clock, glass).await
        });
        // The exchange is in flight, observed rather than assumed: `@hang`
        // writes its invocation to `log` (`answers-as-instructed.sh`)
        // before it execs into `sleep 30`, so the count advancing is the
        // process having actually started and begun hanging (PHASE-10
        // repair — this precondition previously rested on a bare 100 ms
        // sleep). This wait is setup time, not part of the measured
        // interval.
        until(Duration::from_secs(1), || invocations(&log) >= 1).await;
        let start = Instant::now();
        stopper.stop();
        let served = handle.await.expect("serve must not panic");
        (start.elapsed(), served)
      })
      .await;

    // Measured (`notes.md`, PHASE-10 sheet, after the repair below): ~79-106
    // µs across repeated runs, now that the exchange is observed rather than
    // assumed in flight — three orders of magnitude under the 250 ms bound.
    assert!(
      elapsed < Duration::from_millis(250),
      "cancellation took {elapsed:?}, which is not well under the 2 s timeout (S-5)"
    );
    assert_eq!(served.ending, Ending::Stopped);
  }

  /// VT-12 — item 14c. A stop request arriving in the same poll as a ready
  /// command wins (`biased`), and a stop request that arrives *before*
  /// `stopped()` is first awaited still ends the loop — the level-held
  /// property a bare `Notify` lacks. Tripping `Cancel` **before** `serve` is
  /// even called demonstrates both at once: the first `select!` sees a
  /// tripped signal and a ready command together, and `stopped()`'s very
  /// first poll already observes the trip.
  #[tokio::test]
  async fn a_stop_tripped_before_the_first_poll_wins_over_a_ready_command() {
    let (window, tray) = window_and_tray();
    let glass = glass_over(&window, &tray);
    let (command, log) = scripted("vt12", &[]);
    let backend = host(command, TIMEOUT, now());
    let controller = Controller::new();
    let (tx, rx) = mpsc::channel::<Command>(1);
    tx.try_send(Command::Evaluate(Stimulus::Requested))
      .expect("room in a fresh capacity-1 channel");
    let cancel = Cancel::new();
    cancel.stop(); // tripped before `serve` is even called

    let served = serve(backend, controller, rx, cancel, stub_clock, glass).await;

    assert_eq!(served.ending, Ending::Stopped);
    assert!(
      served.controller.frame().shown.is_none(),
      "the queued command was never processed"
    );
    assert_eq!(invocations(&log), 0, "the backend was never contacted");
  }

  /// VT-13 — item 14d. On `Ending::Stopped` the receiver's buffer is left
  /// unread: a command queued **behind** the in-flight exchange produces no
  /// further invocation.
  #[tokio::test]
  async fn on_stop_a_command_queued_behind_the_exchange_is_left_unread() {
    let (window, tray) = window_and_tray();
    let glass = glass_over(&window, &tray);
    let (command, log) = scripted("vt13", &["@hang"]);
    let backend = host(command, TIMEOUT, now());
    let controller = Controller::new();
    let (tx, rx) = mpsc::channel::<Command>(2);
    tx.try_send(Command::Evaluate(Stimulus::Requested))
      .expect("room for the exchange's own command"); // starts the @hang exchange
    tx.try_send(Command::Evaluate(Stimulus::Requested))
      .expect("room for the one queued behind it"); // never dequeued
    let cancel = Cancel::new();
    let stopper = cancel.clone();

    let local = LocalSet::new();
    let served = local
      .run_until(async move {
        let handle = tokio::task::spawn_local(async move {
          serve(backend, controller, rx, cancel, stub_clock, glass).await
        });
        tokio::time::sleep(Duration::from_millis(100)).await;
        stopper.stop();
        handle.await.expect("serve must not panic")
      })
      .await;

    assert_eq!(served.ending, Ending::Stopped);
    assert_eq!(
      invocations(&log),
      1,
      "the queued second command must never reach the backend"
    );
  }
}

//! design.md §9 item 11: the wiring surfaces reachable with no `serve` and
//! no event loop — 11e (local refusals), 11f (the five DT transitions),
//! 11g (back-pressure) and 11i (`busy` clearing). Items 11a-d, 11h and
//! 14a-d are PHASE-10's, once `serve` exists (PL-10, plan.md:1217-1305).
//!
//! `#[cfg(test)]` on the declaration, not on the file, for the same reason
//! every other module here carries it (`clippy::tests_outside_test_module`).

use std::rc::Rc;
use std::time::Duration;

use goad::controller::{Controller, Exchanged, Surface};
use goad::diagnostics::{BUSY_NOTICE, Refused};
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::{Glass, SlintGlass};
use goad::wire::{Cancel, Command, Stimulus, Wire};
use goad_semantics::protocol::canonical::Timestamp;
use i_slint_backend_testing::{ElementHandle, ElementQuery, init_no_event_loop};
use slint::{ComponentHandle, VecModel};
use tokio::sync::mpsc;

use crate::driving::{host, instant, invocations, quiet_event, scripted};

const TIMEOUT: Duration = Duration::from_secs(2);

fn now() -> Timestamp {
  instant("2026-01-01T00:00:00Z")
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

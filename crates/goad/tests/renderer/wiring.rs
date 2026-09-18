//! design.md §9 item 11: every wiring surface reachable with no component and
//! no event loop is not this file's claim — 11e (local refusals), 11f (the
//! five DT transitions), 11g (back-pressure) and 11i (`busy` clearing) are,
//! and PHASE-07 built them with no `serve` and no loop. PHASE-10 adds item
//! 11a-d and 11h (the reducer's seven rows and the one production `serve`,
//! `mod rows`/`interaction`/`serving`) and item 14a-d (cancellation,
//! `mod cancellation`) — the surfaces that need `serve` to exist. Slice 007
//! PHASE-04 adds `mod editing`: the edit path end to end, from a refusal made
//! against retained state to a value on the answer, and the only module here
//! whose fixtures carry fields.
//!
//! `#[cfg(test)]` on the declaration, not on the file, for the same reason
//! every other module here carries it (`clippy::tests_outside_test_module`).

use goad::controller::{Controller, Exchanged, Surface};
use goad::diagnostics::{BUSY_NOTICE, Refused};
use goad::generated::PromptWindow;
use goad::glass::Glass;
use goad::wire::{Cancel, Command, Notice, Stimulus, Wire};
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;
use tokio::sync::mpsc;

use crate::driving::{host, quiet_event};
use crate::harness::{
  TIMEOUT, current_view_token, element_described, field_described, glass_over, now, stub_clock,
  until, window_and_tray,
};
use crate::scripting::{invocations, scripted};
use crate::waiting::LIVENESS_BOUND;

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

/// Whether an option's **control** is enabled, by the identity the tests
/// select on.
///
/// The type filter is not decoration, and it is not this helper's to restate.
/// An option that carries fields also carries a field container answering to
/// the same `option.id`, and an unscoped `find_first` would take whichever the
/// walk reached first — declaration order, which nothing pins. A `groupbox`
/// declares no `accessible-enabled`, so the wrong element answers `None`,
/// which reads like a missing property and is not one.
/// [`harness::element_described`] carries that filter, its reason, and the
/// description predicate; this helper is only the property read off what it
/// finds (`review-code.md` F-13).
fn accessible_enabled_of(window: &PromptWindow, description: &str) -> Option<bool> {
  element_described(window, description).and_then(|element| element.accessible_enabled())
}

/// AC-6's "no window" / "the window follows the interaction", read off the
/// window itself rather than `Controller::frame().surface` — `Surface::Prompt`
/// is set whenever the mode is not `Diagnostic`, view or no view, so it alone
/// cannot distinguish "shown" from "hidden" the way `show()`/`hide()` does
/// (PHASE-10 repair, VT-2/VT-3).
fn window_shown(window: &PromptWindow) -> bool {
  window.window().is_visible()
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
      .frame(false)
      .shown
      .expect("a view must be retained")
      .view_id
      .as_str()
      .to_owned();
    let contacted_before = invocations(&log);

    let refusal = controller
      .answer(&view, "not-an-option")
      .expect_err("an option the presentation does not carry must be refused");
    assert!(matches!(refusal, Refused::UnknownOption));
    controller.refuse(&refusal);

    let frame = controller.frame(false);
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

    let frame = controller.frame(false);
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
    A_PROTOCOL_FAILURE, A_SECOND_VIEW, CLEAN_NO_VIEW, Controller, Exchanged, Glass, Surface,
    TIMEOUT, TWO_OPTIONS, accessible_enabled_of, glass_over, host, in_diagnostic_mode,
    in_prompt_mode, nothing_to_report_shown, now, quiet_event, scripted, window_and_tray,
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
    glass.present(controller.frame(false));
    assert!(in_diagnostic_mode(&window));
    assert!(!nothing_to_report_shown(&window), "a fault line is present");
    assert_eq!(
      controller.frame(false).diagnostics.state(),
      goad::diagnostics::TrayState::Fault
    );

    let cleared = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, cleared);
    glass.present(controller.frame(false));

    assert_eq!(
      controller.frame(false).surface,
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
      controller.frame(false).diagnostics.state(),
      goad::diagnostics::TrayState::Idle,
      "the tray returns to idle immediately — disagreeing with the still-open window (DT-5)"
    );
  }

  /// PHASE-04/VT-2: the standing next-check line reaches its own window
  /// property, "" before any exchange has resolved one; it does not
  /// affect `Diagnostics::state()` (a standing schedule is not a fault);
  /// and — the regression D-17 exists to prevent — "Nothing to report."
  /// still renders on a clean outcome with a next check standing, because
  /// the line is not a `diagnostic-lines` row.
  #[tokio::test]
  async fn vt2_the_next_check_line_has_its_own_property_and_leaves_the_sentinel_and_the_tray_alone()
  {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("wiring-vt2", &[CLEAN_NO_VIEW]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    controller.open_diagnostics();
    glass.present(controller.frame(false));
    assert_eq!(
      window.get_next_check(),
      "",
      "no exchange has resolved a next check yet"
    );

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame(false));

    assert_eq!(
      window.get_next_check(),
      "next check (instructed): 2026-01-01T01:30:00Z",
      "the value reaching the property is `Frame::next_check` rendered, not \
       merely something non-empty: the backend instructed 90 minutes against \
       the harness's fixed `now`"
    );
    assert_eq!(
      controller.frame(false).diagnostics.state(),
      goad::diagnostics::TrayState::Idle,
      "a standing schedule is not a fault"
    );
    assert!(
      nothing_to_report_shown(&window),
      "the next-check line is not a diagnostic-lines row, so the sentinel still renders (D-17)"
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
    glass.present(controller.frame(false));
    assert!(in_diagnostic_mode(&window));

    let replacing = backend.evaluate(now(), quiet_event(now())).await;
    let shift = controller.absorb(Exchanged::Evaluation, replacing).shift;
    glass.present(controller.frame(false));

    assert_eq!(shift, goad::controller::Shift::Replaced);
    assert_eq!(controller.frame(false).surface, Surface::Prompt);
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
    glass.present(controller.frame(false));
    assert_eq!(window.get_heading(), "Proceed?");

    let failing = backend.evaluate(now(), quiet_event(now())).await;
    let shift = controller.absorb(Exchanged::Evaluation, failing).shift;
    glass.present(controller.frame(false));

    assert_eq!(shift, goad::controller::Shift::Retained);
    assert_eq!(controller.frame(false).surface, Surface::Prompt);
    assert_eq!(
      window.get_heading(),
      "Proceed?",
      "the question did not go away"
    );
    assert!(in_prompt_mode(&window));
    assert_eq!(accessible_enabled_of(&window, "yes"), Some(true));
    assert_eq!(
      controller.frame(false).diagnostics.state(),
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
    glass.present(controller.frame(false));
    assert!(in_diagnostic_mode(&window));

    controller.close_diagnostics();
    glass.present(controller.frame(false));

    assert_eq!(controller.frame(false).surface, Surface::Prompt);
    assert!(in_prompt_mode(&window));
    assert_eq!(
      window.get_heading(),
      "Proceed?",
      "the retained prompt's own heading"
    );
  }
}

/// VT-7 — item 11g (F-5). A second command sent while the channel is full
/// raises the back-pressure signal and does not enter `Diagnostics`; the
/// notice the next `present` writes **survives** every further present, and
/// only the person's next successful send lowers it (design.md §5.3, §5.4).
mod back_pressure {
  use super::{
    BUSY_NOTICE, Cancel, Command, Controller, Glass, Notice, Stimulus, Wire, glass_over, mpsc,
    window_and_tray,
  };

  #[test]
  fn a_full_channel_raises_the_notice_and_only_a_successful_send_lowers_it() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (tx, mut receiver) = mpsc::channel::<Command>(1);
    tx.try_send(Command::Evaluate(Stimulus::Requested))
      .expect("the first send must have room in a fresh capacity-1 channel");
    let notice = Notice::new();
    let wire = Wire::new(tx, Cancel::new(), notice.clone());

    wire.send(Command::OpenDiagnostics);

    assert_eq!(
      window.get_notice(),
      "",
      "the edge raises a signal and writes no window property: `Glass::present` is the only writer"
    );

    let controller = Controller::new();
    glass.present(controller.frame(notice.raised()));
    assert_eq!(
      window.get_notice(),
      BUSY_NOTICE,
      "a command sent into a full channel must report BUSY_NOTICE, not vanish"
    );

    glass.present(controller.frame(notice.raised()));
    assert_eq!(
      window.get_notice(),
      BUSY_NOTICE,
      "the notice must outlive the present that corrects the dropped action, and every \
       present after it (design.md §5.4)"
    );

    // The loop reads, so the one slot is free and the person's next action
    // gets through. That — not a present — is what lowers the notice.
    receiver
      .try_recv()
      .expect("the held command is there to be read");
    wire.send(Command::CloseDiagnostics);
    glass.present(controller.frame(notice.raised()));

    assert_eq!(
      window.get_notice(),
      "",
      "a successful send lowers the signal, and the next present clears the window (design.md §5.3)"
    );

    assert!(
      controller.frame(notice.raised()).diagnostics.is_clear(),
      "back-pressure is not a fault: it never enters Diagnostics"
    );
  }

  /// The `Frame` half, with no channel, no window and no platform: the
  /// controller retains nothing for this and carries what it is given.
  #[test]
  fn the_frame_carries_the_notice_it_is_given() {
    let controller = Controller::new();
    assert!(
      controller.frame(true).notice,
      "a raised signal reaches the glass through the frame"
    );
    assert!(
      !controller.frame(false).notice,
      "and a lowered one does too: the property is written either way, every present"
    );
  }
}

/// VT-9 — item 11i (F-21). `busy` returns to `false` on both outcomes, and
/// the option controls read `accessible_enabled == true` in the element
/// tree. VA-2's negative control is the one that matters: an `absorb`
/// that does not clear `engaged` leaves every control disabled.
mod busy {
  use super::{
    A_PROTOCOL_FAILURE, ComponentHandle, Controller, Exchanged, Glass, PromptWindow, TIMEOUT,
    TWO_OPTIONS, accessible_enabled_of, glass_over, host, now, quiet_event, scripted,
    window_and_tray,
  };

  /// Both cases below read a control off a **shown** window, and a shown
  /// window clips: `ElementQuery` skips any element outside the nearest
  /// clipping ancestor's rect
  /// (`i-slint-backend-testing-1.17.1/search_api.rs:373-375` →
  /// `i-slint-core-1.17.1/item_tree.rs:399-408`, a geometric test). Left to
  /// its preferred size the window is 65px tall, the options `ScrollView`
  /// fits one control, and the second option's button is simply unreachable
  /// — `accessible_enabled_of` then answers `None`, which reads like a
  /// missing accessible property and is not one.
  ///
  /// Sizing the viewport states what the test can see. It is **not** a claim
  /// about what the window should be: the product's own size is AC-10's and
  /// slice 008's. Before this, these two cases found their second control
  /// only because the preferred height happened to exceed two `fluent`
  /// buttons by 18px — a layout nothing declares, which is the shape
  /// `docs/memory/a-green-test-can-assert-a-proxy.md` records.
  fn with_room_for_every_control(window: &PromptWindow) {
    ComponentHandle::window(window).set_size(slint::PhysicalSize::new(400, 400));
  }

  #[tokio::test]
  async fn busy_clears_and_controls_re_enable_after_a_success() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("wiring-busy-success", &[TWO_OPTIONS]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();
    with_room_for_every_control(&window);

    controller.engage();
    glass.present(controller.frame(false));
    assert!(window.get_busy(), "the exchange must be shown in flight");

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame(false));

    assert!(!controller.frame(false).busy);
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
    with_room_for_every_control(&window);

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame(false));

    controller.engage();
    glass.present(controller.frame(false));
    assert!(window.get_busy());
    // The direction that discriminates, and the one this case's own name
    // presupposes: *re-enable* is a claim that they were disabled. `Some(true)`
    // is also what an **unbound** `enabled` answers, so the two assertions at
    // the foot cannot tell `enabled: !root.busy` from its absence and neither
    // can any other enabled-state assertion in this workspace
    // (`review-code.md` F-2). Readable here and not in the case above, which
    // engages before absorbing anything and so has no control to read
    // (F-7).
    assert_eq!(accessible_enabled_of(&window, "yes"), Some(false));
    assert_eq!(accessible_enabled_of(&window, "no"), Some(false));

    let failing = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, failing);
    glass.present(controller.frame(false));

    assert!(!controller.frame(false).busy);
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
    let shift = controller.absorb(Exchanged::Evaluation, outcome).shift;
    glass.present(controller.frame(false));

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
    let shift = controller.absorb(Exchanged::Evaluation, outcome).shift;
    glass.present(controller.frame(false));

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
    glass.present(controller.frame(false));
    let view = current_view_token(&window).expect("a view must be retained");
    let (view_id, answer) = controller
      .answer(&view, "yes")
      .expect("the retained option must answer");

    let outcome = backend.respond(now(), view_id, answer).await;
    let shift = controller.absorb(Exchanged::Answer, outcome).shift;
    glass.present(controller.frame(false));

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
    let shift = controller.absorb(Exchanged::Evaluation, outcome).shift;
    glass.present(controller.frame(false));

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

    let shift = controller.absorb(Exchanged::Answer, outcome).shift;
    glass.present(controller.frame(false));

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
    glass.present(controller.frame(false));
    let view = current_view_token(&window).expect("a view must be retained");
    let (view_id, answer) = controller
      .answer(&view, "yes")
      .expect("the retained option must answer");

    let outcome = backend.respond(now(), view_id, answer).await;
    let shift = controller.absorb(Exchanged::Answer, outcome).shift;
    glass.present(controller.frame(false));

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

    let shift = controller.absorb(Exchanged::Evaluation, outcome).shift;
    glass.present(controller.frame(false));

    assert_eq!(shift, Shift::Replaced);
    assert!(window_shown(&window));
    assert_eq!(window.get_heading(), "Proceed?");
  }
}

/// F-1 (review-code 002, round 1). AC-9's body half, driven through the
/// full pipeline `mapper.rs` stops short of: `read_response` → `present` →
/// `Glass::present`, read back off the window `SlintGlass` actually wrote
/// to, not off `Prepared::presentation` (which `mapper.rs` and
/// `reception.rs` already cover). Closes `glass.rs::styled`'s two live
/// arms — `Body::Plain` and the accepted-markdown half of `Body::Rich` —
/// and the `body-degraded` flag the marker in `tree.rs` reads.
///
/// Residual, recorded rather than hidden: this cannot detect
/// `ui/app.slint`'s `StyledText { text: root.body; }` binding itself being
/// severed — `PromptWindow::get_body()` reads the `in property` `set_body`
/// wrote, independently of whether any element still binds to it, and
/// `i-slint-backend-testing` 1.17.1 has no element-tree readback for a
/// `styled-text`-typed element (`StyledText` gets no default
/// `accessible-role`/`accessible-label`, unlike `Text`, and the language
/// has no `styled-text`-to-`string` conversion to construct one — checked
/// against the Slint compiler's `lower_accessibility.rs` and against
/// upstream's own `styled_text.rs` test, which stops at the same property
/// round trip). Closing that residual needs either a new production
/// property (a design-shape change) or an upstream capability that does
/// not exist today; left for the ledger rather than papered over.
mod body_content {
  use goad::controller::{Controller, Exchanged};
  use goad::glass::Glass;
  use slint::StyledText;

  use super::{TIMEOUT, glass_over, host, now, quiet_event, scripted, window_and_tray};

  const A_PLAIN_BODY: &str = r#"{"view":{"kind":"choice","title":"T","options":[{"id":"ok","label":"OK"}],"body":{"kind":"text","value":"a plain body"}},"next_check":"45 minutes"}"#;
  const AN_ACCEPTED_MARKDOWN_BODY: &str = r#"{"view":{"kind":"choice","title":"T","options":[{"id":"ok","label":"OK"}],"body":{"kind":"markdown","value":"**bold**"}},"next_check":"45 minutes"}"#;
  /// U+E541 is Slint's private-use interpolation placeholder (mapper.rs's
  /// `rejected_markdown_degrades_to_plain_and_is_reported_undrawn`):
  /// `StyledText::from_markdown` rejects it, so the body degrades to plain.
  const A_REJECTED_MARKDOWN_BODY: &str = "{\"view\":{\"kind\":\"choice\",\"title\":\"T\",\"options\":[{\"id\":\"ok\",\"label\":\"OK\"}],\"body\":{\"kind\":\"markdown\",\"value\":\"\u{e541}\"}},\"next_check\":\"45 minutes\"}";

  /// A plain body reaches the window unchanged, and is never reported
  /// degraded.
  #[tokio::test]
  async fn a_plain_body_reaches_the_window_and_is_not_degraded() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("body-content-plain", &[A_PLAIN_BODY]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame(false));

    assert_eq!(
      window.get_body(),
      StyledText::from_plain_text("a plain body")
    );
    assert!(!window.get_body_degraded());
  }

  /// Markdown the parser accepts is retained rich, and is not degraded.
  #[tokio::test]
  async fn accepted_markdown_reaches_the_window_rich_and_is_not_degraded() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("body-content-rich", &[AN_ACCEPTED_MARKDOWN_BODY]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame(false));

    assert_eq!(
      window.get_body(),
      StyledText::from_markdown("**bold**").expect("this markdown must parse")
    );
    assert!(!window.get_body_degraded());
  }

  /// Markdown the parser rejects still reaches the window, as literal
  /// text, and `body-degraded` is written `true` — the property `tree.rs`'s
  /// marker test reads.
  #[tokio::test]
  async fn rejected_markdown_reaches_the_window_as_plain_and_is_degraded() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (command, _log) = scripted("body-content-degraded", &[A_REJECTED_MARKDOWN_BODY]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    glass.present(controller.frame(false));

    assert_eq!(
      window.get_body(),
      StyledText::from_plain_text("\u{e541}"),
      "a rejected body still reaches the window, as literal text"
    );
    assert!(window.get_body_degraded());
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
  use goad::controller::{Controller, Ending, Exchanged, Shift, serve};
  use goad::glass::Glass;
  use goad::wire::{Cancel, Command, Notice, Stimulus};
  use goad_shell::ingress::Ingress;
  use tokio::sync::mpsc;
  use tokio::task::LocalSet;

  use super::{
    A_PROTOCOL_FAILURE, CLEAN_NO_VIEW, LIVENESS_BOUND, TIMEOUT, TWO_OPTIONS, current_view_token,
    glass_over, host, in_prompt_mode, invocations, now, quiet_event, scripted, stub_clock, until,
    window_and_tray, window_shown,
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
    glass.present(controller.frame(false));
    assert!(window_shown(&window), "the question must be on screen");
    assert!(in_prompt_mode(&window));
    assert_eq!(window.get_heading(), "Proceed?");

    // Half one: an `evaluate` returning `view: null` while a question is
    // outstanding leaves it exactly as it was.
    let cleared = backend.evaluate(now(), quiet_event(now())).await;
    assert_eq!(
      controller.absorb(Exchanged::Evaluation, cleared).shift,
      Shift::Retained
    );
    glass.present(controller.frame(false));
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

    assert_eq!(
      controller.absorb(Exchanged::Answer, closing).shift,
      Shift::Closed
    );
    glass.present(controller.frame(false));
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
    glass.present(controller.frame(false));
    let view = current_view_token(&window).expect("a view must be retained");

    let (view_id, answer) = controller
      .answer(&view, "yes")
      .expect("the retained option must answer");
    let failing = backend.respond(now(), view_id, answer).await;
    assert_eq!(
      controller.absorb(Exchanged::Answer, failing).shift,
      Shift::Retained
    );
    glass.present(controller.frame(false));
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
      controller.absorb(Exchanged::Answer, succeeding).shift,
      Shift::Closed,
      "the retry succeeds"
    );
    glass.present(controller.frame(false));
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
          serve(
            backend,
            controller,
            rx,
            cancel,
            Notice::new(),
            stub_clock,
            glass,
            Ingress::none(),
          )
          .await
        });

        tx.send(Command::Evaluate(Stimulus::Requested))
          .await
          .expect("the channel must accept the first send");
        until(LIVENESS_BOUND, || window.get_heading() == "Proceed?").await;
        let stale_view = current_view_token(&window).expect("view A must be on screen");

        // The intervening evaluate: a slow exchange that will land view B
        // while the queued click still names view A.
        tx.send(Command::Evaluate(Stimulus::Requested))
          .await
          .expect("the channel must accept the second send");
        until(LIVENESS_BOUND, || invocations(&log) >= 2).await;

        tx.send(Command::Choose {
          view: stale_view,
          option: "yes".to_owned(),
          edits: Vec::new(),
        })
        .await
        .expect("the channel must accept the queued click");

        // The refusal, read off the tray tooltip the production glass wrote
        // — the element tree, not a peek at the controller mid-flight.
        until(LIVENESS_BOUND, || {
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
        .frame(false)
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
          serve(
            backend,
            controller,
            rx,
            cancel,
            Notice::new(),
            stub_clock,
            glass,
            Ingress::none(),
          )
          .await
        });

        tx.send(Command::Evaluate(Stimulus::Requested))
          .await
          .expect("the channel must accept the first send");
        until(LIVENESS_BOUND, || window.get_heading() == "Proceed?").await;
        let view = current_view_token(&window)
          .expect("with no intervening evaluate the retained view is still on screen");

        tx.send(Command::Choose {
          view,
          option: "yes".to_owned(),
          edits: Vec::new(),
        })
        .await
        .expect("the channel must accept the click");
        until(LIVENESS_BOUND, || !window_shown(&window)).await;

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

/// PHASE-04. The draft is retained, the screen is written back from it every
/// present, and `answer()` submits a value for every drawn field of the
/// option it names and none for any other.
mod editing {
  use goad::controller::{Controller, Ending, Exchanged, serve};
  use goad::draft::Reported;
  use goad::generated::PromptWindow;
  use goad::glass::Glass;
  use goad::wire::{Cancel, Command, Notice, Stimulus};
  use goad_semantics::protocol::canonical::{FieldId, UserResponse};
  use goad_shell::ingress::Ingress;
  use slint::{ComponentHandle, Model};
  use tokio::sync::mpsc;
  use tokio::task::LocalSet;

  use super::{
    LIVENESS_BOUND, Refused, TIMEOUT, accessible_enabled_of, current_view_token, field_described,
    glass_over, host, invocations, now, quiet_event, scripted, stub_clock, until, window_and_tray,
  };

  /// A two-option form. Both options carry drawn fields and **share a field
  /// id**: R-52 scopes a field id to its option, and the protocol fixtures
  /// two options sharing one as legal, which is what gives R-58's "nor a
  /// field of an option it is not answering" clause something to be false of.
  ///
  /// `morning` also carries a `text` field, which this renderer does not
  /// draw. R-58 says the response is silent about such a field rather than
  /// carrying a default for it, so its absence from `values` is an assertion
  /// and not an oversight.
  const TWO_FORMS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"read","kind":"boolean","label":"Read"},{"id":"noted","kind":"number","label":"How many?"}]},{"id":"evening","label":"Evening","fields":[{"id":"read","kind":"boolean","label":"Read"},{"id":"tidied","kind":"boolean","label":"Tidied"}]}]},"next_check":"45 minutes"}"#;

  /// One option, five fields, two blocks: two under a heading the backend
  /// authored, then three carrying no `group` at all — an **untitled** block,
  /// not a missing one. Five and not two so that "in declared order" has more
  /// than one way to be wrong.
  const TWO_BLOCKS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"only","label":"Only","fields":[{"id":"one","kind":"boolean","label":"One","group":"Before you go"},{"id":"two","kind":"boolean","label":"Two","group":"Before you go"},{"id":"three","kind":"boolean","label":"Three"},{"id":"four","kind":"boolean","label":"Four"},{"id":"five","kind":"boolean","label":"Five"}]}]},"next_check":"45 minutes"}"#;

  /// A controller holding `document`, and the view token the markup hands
  /// back on a callback. The view travels through the real backend, the real
  /// normalizer and the real fold, so the blocks under test are the mapper's
  /// own output and not a literal a test assembled.
  async fn retaining(case: &str, document: &str) -> (Controller, String) {
    let (command, _log) = scripted(case, &[document]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();

    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);
    let view = controller
      .frame(false)
      .shown
      .expect("the fixture must retain a view")
      .view_id
      .as_str()
      .to_owned();
    (controller, view)
  }

  /// The submitted keys. `values` is a `BTreeMap`, so this is its own
  /// iteration order rather than a re-sort — which is also why an expected
  /// list here is written sorted and says nothing about declared order.
  fn submitted_keys(answer: &UserResponse) -> Vec<&str> {
    answer.values.keys().map(FieldId::as_str).collect()
  }

  fn submitted_value<'a>(answer: &'a UserResponse, field: &str) -> Option<&'a serde_json::Value> {
    answer
      .values
      .iter()
      .find(|(id, _)| id.as_str() == field)
      .map(|(_, value)| value)
  }

  /// The row model's blocks for one option row: each block's heading and its
  /// field ids, in model order. This is `option_rows`'s own output read back
  /// off the window — the value the markup's two `for`s walk — and not a
  /// second derivation of it.
  fn blocks_of_row(window: &PromptWindow, row: usize) -> Vec<(String, Vec<String>)> {
    window
      .get_options()
      .row_data(row)
      .expect("the row must be in the model")
      .blocks
      .iter()
      .map(|block| {
        (
          block.heading.to_string(),
          block
            .fields
            .iter()
            .map(|field| field.id.to_string())
            .collect(),
        )
      })
      .collect()
  }

  /// Every case below reads a control off a **shown** window, and a shown
  /// window clips: `ElementQuery` skips anything outside the nearest clipping
  /// ancestor's rect. Left at its preferred size the window fits one control
  /// and a form of several is unreachable — the helper answers `None`, which
  /// reads like a missing element and is not one. `mod busy`'s
  /// `with_room_for_every_control` is the precedent and PHASE-02's F-6 the
  /// measurement.
  ///
  /// Sizing the viewport states what the test can see. It is **not** a claim
  /// about what the window should be: the product's own size is AC-10's and
  /// slice 008's, and F-6 stays open regardless of this line.
  fn with_room_for_the_form(window: &PromptWindow) {
    ComponentHandle::window(window).set_size(slint::PhysicalSize::new(600, 600));
  }

  /// VT-1 — every selector `edit` refuses on, and the refusal each one
  /// earns. Two selectors can fail for reasons that look alike from the
  /// outside, so the case asserts which of the three failed, not merely that
  /// something did.
  #[tokio::test]
  async fn an_edit_is_refused_by_each_selector_that_fails_and_records_nothing() {
    let (mut controller, view) = retaining("edit-refusals", TWO_FORMS).await;

    assert_eq!(
      controller.edit(
        "a-token-from-a-replaced-view",
        "morning",
        "read",
        &Reported::Checked(true)
      ),
      Err(Refused::SupersededView),
      "identity is checked first, and a stale token is refused for the reason true of it"
    );
    assert_eq!(
      controller.edit(&view, "not-an-option", "read", &Reported::Checked(true)),
      Err(Refused::UnknownOption)
    );
    assert_eq!(
      controller.edit(&view, "morning", "not-a-field", &Reported::Checked(true)),
      Err(Refused::UnknownField)
    );
    assert_eq!(
      controller.edit(&view, "morning", "tidied", &Reported::Checked(true)),
      Err(Refused::UnknownField),
      "a field the *other* option declares is not this one's: R-52 scopes a field id to its option"
    );
    assert_eq!(
      controller.edit(&view, "morning", "noted", &Reported::Checked(true)),
      Err(Refused::UnknownField),
      "an undrawn field never entered a block, so the walk that admits an edit and the walk \
       that submits a value are the same walk"
    );

    let (_, answer) = controller
      .answer(&view, "morning")
      .expect("the option still answers");
    assert!(
      answer
        .values
        .values()
        .all(|value| value == &serde_json::Value::Bool(false)),
      "not one of the five refusals recorded anything: {:?}",
      answer.values
    );

    let mut nothing_retained = Controller::new();
    assert_eq!(
      nothing_retained.edit(&view, "morning", "read", &Reported::Checked(true)),
      Err(Refused::SupersededView),
      "with nothing retained the view is superseded, not the option unknown — there is no \
       presentation for an option to be missing from"
    );
  }

  /// VT-2 — one edit, then the answer it qualifies. R-35 and P-3 in one
  /// case: a half-filled form goes out as it stands, carrying the drawn
  /// value for the field nobody touched, and the host fills no gap on the
  /// person's behalf.
  #[tokio::test]
  async fn an_answer_carries_a_value_for_every_drawn_field_of_the_option_it_names() {
    let (mut controller, view) = retaining("edit-recorded", TWO_FORMS).await;

    controller
      .edit(&view, "morning", "read", &Reported::Checked(true))
      .expect("a field the option declares records");

    let (_, answer) = controller
      .answer(&view, "morning")
      .expect("the edited option answers");
    assert_eq!(submitted_keys(&answer), vec!["read", "stretched"]);
    assert_eq!(
      submitted_value(&answer, "read"),
      Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
      submitted_value(&answer, "stretched"),
      Some(&serde_json::Value::Bool(false)),
      "the untouched field carries the value it was drawn with, not an absence"
    );
  }

  /// VT-3 — **AC-8, and `canon-delta.md`'s R-58 in one case.** Two options,
  /// each carrying fields and sharing the field id `read`. Answering
  /// `morning` carries exactly the fields drawn of `morning`: not `evening`'s
  /// `tidied`, and not `noted`, which was reported undrawn. The shared id is
  /// two independent keys, which is the half a single-option fixture cannot
  /// reach. This is the test SPEC-001 §7's new R-58 row will name.
  ///
  /// **It asserts at `answer()`, and that is the vehicle the design names for
  /// R-58** (`design.md` §9, AC-8) — not AC-1's, which reads `values` off the
  /// wire and belongs to a later phase. What keeps this from being the proxy
  /// §9's closing paragraph warns about is `stretched`: a walk over the
  /// *draft's* keys cannot produce a key for a field nobody touched, so the
  /// expected list below is one D6 fails rather than one it also satisfies.
  #[tokio::test]
  async fn an_answer_carries_no_value_for_another_option_or_for_an_undrawn_field() {
    let (mut controller, view) = retaining("r58", TWO_FORMS).await;
    assert!(
      controller
        .frame(false)
        .diagnostics
        .lines()
        .iter()
        .any(|line| line.contains("not drawn: option morning field noted")),
      "the fixture must actually carry an undrawn field for its absence below to mean anything"
    );

    controller
      .edit(&view, "morning", "read", &Reported::Checked(true))
      .expect("morning declares `read`");
    controller
      .edit(&view, "evening", "tidied", &Reported::Checked(true))
      .expect("evening declares `tidied`");

    let (_, morning) = controller
      .answer(&view, "morning")
      .expect("morning answers");
    assert_eq!(
      submitted_keys(&morning),
      vec!["read", "stretched"],
      "exactly morning's drawn fields: `tidied` is another option's and `noted` was not drawn"
    );
    assert_eq!(
      submitted_value(&morning, "read"),
      Some(&serde_json::Value::Bool(true))
    );

    let (_, evening) = controller
      .answer(&view, "evening")
      .expect("evening answers");
    assert_eq!(submitted_keys(&evening), vec!["read", "tidied"]);
    assert_eq!(
      submitted_value(&evening, "read"),
      Some(&serde_json::Value::Bool(false)),
      "the shared id under a different option is a different key: morning's tick is not this one"
    );
    assert_eq!(
      submitted_value(&evening, "tidied"),
      Some(&serde_json::Value::Bool(true))
    );
  }

  /// VT-5 — the screen is written back from the draft on the next present.
  /// Read off a shown window's element tree by the option-scoped query, which
  /// is the screen and not the row model: a `checked` that never reached a
  /// control would pass a model assertion.
  ///
  /// Both options are read, because the shared field id is the case an
  /// unscoped query answers wrongly while reporting no ambiguity.
  #[tokio::test]
  async fn the_next_present_writes_every_control_back_from_the_draft() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    with_room_for_the_form(&window);
    let (mut controller, view) = retaining("screen-from-draft", TWO_FORMS).await;

    controller
      .edit(&view, "morning", "read", &Reported::Checked(true))
      .expect("morning declares `read`");
    glass.present(controller.frame(false));

    for (option, field, expected) in [
      ("morning", "read", true),
      ("morning", "stretched", false),
      ("evening", "read", false),
      ("evening", "tidied", false),
    ] {
      let control = field_described(&window, option, field)
        .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"));
      assert_eq!(
        control.accessible_checked(),
        Some(expected),
        "{option}/{field} must read the draft's value"
      );
    }
  }

  /// `enabled: !root.busy` on **both** controls it binds — the checkbox this
  /// slice added and the option button beside it. Before this case neither was
  /// pointed at by anything, and deleting both bindings left the whole gate
  /// green (`review-code.md` F-2).
  ///
  /// Asserted in **both** directions and the busy one first, because
  /// `Some(true)` is what an *unbound* `enabled` answers — the same as a bound
  /// one at rest — so the enabled assertion alone cannot tell a live binding
  /// from a deleted one. Every pre-existing enabled-state assertion in this
  /// workspace is in that blind direction, which is how the gap survived. The
  /// pair is what says a control follows `busy` rather than merely being
  /// enabled at rest.
  ///
  /// Here rather than in `mod busy` because of the **checkbox**, which needs a
  /// view carrying fields: `mod busy`'s fixture is `TWO_OPTIONS`, which carries
  /// none. Its first case could not hold even the button's half — it engages
  /// before absorbing anything, so at its busy present the window holds no
  /// options and `accessible_enabled_of` answers `None`, measured. Its
  /// **second** case absorbs and presents before engaging, so the button is
  /// readable there and now asserts it (`review-code.md` F-7, which caught this
  /// comment generalising the first case's constraint to both). Reading the
  /// checkbox through `field_described` keeps this one at the screen rather
  /// than the row model.
  ///
  /// What this protects is `design.md` §5.4: the window is inert exactly while
  /// `serve`'s outer loop is not reading commands, so the one-slot command
  /// channel is never asked to hold two edits at once.
  #[tokio::test]
  async fn both_controls_are_disabled_while_an_exchange_is_in_flight_and_enabled_after_it() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    with_room_for_the_form(&window);
    // Its own backend rather than `retaining`'s, because the case needs a
    // *second* exchange to land: two instructions, one per invocation.
    let (command, _log) = scripted("field-busy", &[TWO_FORMS, TWO_FORMS]);
    let mut backend = host(command, TIMEOUT, now());
    let mut controller = Controller::new();
    let outcome = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, outcome);

    let enabled = |shown: &PromptWindow| {
      field_described(shown, "morning", "read")
        .expect("morning declares `read`")
        .accessible_enabled()
    };

    controller.engage();
    glass.present(controller.frame(false));
    assert_eq!(
      enabled(&window),
      Some(false),
      "a checkbox must be inert while the host is mid-exchange"
    );
    assert_eq!(
      accessible_enabled_of(&window, "morning"),
      Some(false),
      "and so must the button that would answer with it"
    );

    let landed = backend.evaluate(now(), quiet_event(now())).await;
    controller.absorb(Exchanged::Evaluation, landed);
    glass.present(controller.frame(false));
    assert_eq!(
      enabled(&window),
      Some(true),
      "and live again once the exchange has landed"
    );
    assert_eq!(accessible_enabled_of(&window, "morning"), Some(true));
  }

  /// VT-7 — **AC-2's middle link.** `option_rows` is the only host code that
  /// puts a backend's declared order and its headings into the row model;
  /// PHASE-03/VT-3 verifies `View → present()` and PHASE-02/VT-4 verifies
  /// `row model → screen`, and without this the link between them is
  /// verified by nothing.
  #[tokio::test]
  async fn the_row_model_carries_the_blocks_and_their_fields_in_declared_order() {
    let (window, tray) = window_and_tray();
    let mut glass = glass_over(&window, &tray);
    let (controller, _view) = retaining("blocks", TWO_BLOCKS).await;

    glass.present(controller.frame(false));

    assert_eq!(
      blocks_of_row(&window, 0),
      vec![
        (
          "Before you go".to_owned(),
          vec!["one".to_owned(), "two".to_owned()]
        ),
        (
          String::new(),
          vec!["three".to_owned(), "four".to_owned(), "five".to_owned()]
        ),
      ],
      "two blocks in declared order, each carrying its fields in declared order; the second \
       renders `heading: None` as \"\", which the markup reads as untitled rather than missing"
    );
  }

  /// VT-4 — an `Edit` through the real channel and the production loop.
  ///
  /// Success is a **non**-event: there is nothing to exchange, so `dispatch`
  /// answers `None`, the loop continues to the top and presents, and the
  /// backend is never contacted. A refusal takes the single existing refusal
  /// site — the path `Choose` already uses — so the line reaches a person
  /// with no new reporting path added.
  #[tokio::test]
  async fn an_edit_starts_no_exchange_and_a_refused_one_reports_where_a_refused_click_does() {
    let (window, tray) = window_and_tray();
    let glass = glass_over(&window, &tray);
    let (command, log) = scripted("edit-through-serve", &[TWO_FORMS]);
    let backend = host(command, TIMEOUT, now());
    let controller = Controller::new();
    let (tx, rx) = mpsc::channel::<Command>(2);
    let cancel = Cancel::new();
    let stopper = cancel.clone();

    let local = LocalSet::new();
    let served = local
      .run_until(async {
        let handle = tokio::task::spawn_local(async move {
          serve(
            backend,
            controller,
            rx,
            cancel,
            Notice::new(),
            stub_clock,
            glass,
            Ingress::none(),
          )
          .await
        });

        tx.send(Command::Evaluate(Stimulus::Requested))
          .await
          .expect("the channel must accept the first send");
        until(LIVENESS_BOUND, || window.get_heading() == "Proceed?").await;
        let view = current_view_token(&window).expect("the form must be on screen");

        tx.send(Command::Edit {
          view: view.clone(),
          option: "morning".to_owned(),
          field: "read".to_owned(),
          value: Reported::Checked(true),
        })
        .await
        .expect("the channel must accept the edit");
        until(LIVENESS_BOUND, || {
          blocks_of_row(&window, 0)
            .iter()
            .any(|(_, fields)| fields.contains(&"read".to_owned()))
            && checked_in_row_model(&window, "read")
        })
        .await;

        tx.send(Command::Edit {
          view,
          option: "morning".to_owned(),
          field: "not-a-field".to_owned(),
          value: Reported::Checked(true),
        })
        .await
        .expect("the channel must accept the refused edit");
        until(LIVENESS_BOUND, || {
          tray.get_hover_text().contains("to a field of the option")
        })
        .await;

        stopper.stop();
        handle.await.expect("serve must not panic")
      })
      .await;

    assert_eq!(served.ending, Ending::Stopped);
    assert_eq!(
      invocations(&log),
      1,
      "neither the recorded edit nor the refused one is an exchange: only the evaluate ran"
    );
    assert!(
      served
        .controller
        .frame(false)
        .diagnostics
        .lines()
        .iter()
        .any(|line| line.contains("could not match that control to a field of the option it names")),
      "the refusal must be the one that says which of the two selectors failed"
    );
  }

  /// `read`'s value in the window's two channels, as the markup would read it:
  /// the row carries the `slot` and the value lives at that index of `values`
  /// (design.md §5.2).
  fn checked_in_row_model(window: &PromptWindow, field: &str) -> bool {
    let values = window.get_values();
    window
      .get_options()
      .row_data(0)
      .into_iter()
      .flat_map(|row| row.blocks.iter().collect::<Vec<_>>())
      .flat_map(|block| block.fields.iter().collect::<Vec<_>>())
      .filter(|row| row.id == field)
      .filter_map(|row| values.row_data(usize::try_from(row.slot).ok()?))
      .any(|value| value.checked)
  }
}

/// VT-8 — item 11h. One `serve`, no duplicate: the test calls `serve` —
/// the same function `main` wraps — so a loop-body change cannot pass here
/// and fail in production.
mod serving {
  use goad::controller::{Controller, Ending, serve};
  use goad::wire::{Cancel, Command, Notice, Stimulus};
  use goad_shell::ingress::Ingress;
  use tokio::sync::mpsc;

  use super::{
    TIMEOUT, TWO_OPTIONS, glass_over, host, in_prompt_mode, now, scripted, stub_clock,
    window_and_tray,
  };

  #[tokio::test]
  async fn serve_drives_one_exchange_through_the_production_loop() {
    let (window, tray) = window_and_tray();
    let glass = glass_over(&window, &tray);
    // Named for this case, not for a criterion id. `scheduling.rs` already
    // holds `"vt8"`, both files are one test binary, and a marker path is
    // qualified by pid — so the two shared one file and `marker`'s `clear`
    // deleted whichever was written first, at one failure in six runs
    // (`review-code.md` F-5).
    let (command, _log) = scripted("wiring-serve-one-exchange", &[TWO_OPTIONS]);
    let backend = host(command, TIMEOUT, now());
    let controller = Controller::new();
    let (tx, rx) = mpsc::channel::<Command>(1);
    tx.try_send(Command::Evaluate(Stimulus::Requested))
      .expect("room in a fresh capacity-1 channel");
    drop(tx); // closes the channel once the one command is dequeued

    let served = serve(
      backend,
      controller,
      rx,
      Cancel::new(),
      Notice::new(),
      stub_clock,
      glass,
      Ingress::none(),
    )
    .await;

    assert_eq!(served.ending, Ending::Closed);
    assert!(served.controller.frame(false).shown.is_some());
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
  use goad::wire::{Cancel, Command, Notice, Stimulus};
  use goad_shell::ingress::Ingress;
  use tokio::sync::mpsc;
  use tokio::task::LocalSet;

  use super::{
    LIVENESS_BOUND, TIMEOUT, glass_over, host, invocations, now, scripted, stub_clock, until,
    window_and_tray,
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
          serve(
            backend,
            controller,
            rx,
            cancel,
            Notice::new(),
            stub_clock,
            glass,
            Ingress::none(),
          )
          .await
        });
        // The exchange is in flight, observed rather than assumed: `@hang`
        // writes its invocation to `log` (`answers-as-instructed.sh`)
        // before it execs into `sleep 30`, so the count advancing is the
        // process having actually started and begun hanging (PHASE-10
        // repair — this precondition previously rested on a bare 100 ms
        // sleep). This wait is setup time, not part of the measured
        // interval.
        until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
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

    let served = serve(
      backend,
      controller,
      rx,
      cancel,
      Notice::new(),
      stub_clock,
      glass,
      Ingress::none(),
    )
    .await;

    assert_eq!(served.ending, Ending::Stopped);
    assert!(
      served.controller.frame(false).shown.is_none(),
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
      .run_until(async {
        let handle = tokio::task::spawn_local(async move {
          serve(
            backend,
            controller,
            rx,
            cancel,
            Notice::new(),
            stub_clock,
            glass,
            Ingress::none(),
          )
          .await
        });
        // Observed rather than assumed in flight, the same repair PL-17
        // recorded for VT-10 above (F-2, review-code 002 round 1): this
        // precondition previously rested on a bare 100 ms sleep.
        until(LIVENESS_BOUND, || invocations(&log) >= 1).await;
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

//! design.md §9 items 6-10: the renderer's cheap tier, headless, no display
//! server. Each `#[test]` constructs its own `PromptWindow` and calls
//! `init_no_event_loop()` itself, never behind a shared guard — the
//! platform is thread-local and cargo runs test functions on separate
//! threads (`research.md` Thread 3, T-B).

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use goad::generated::{OptionRow, PromptWindow, WindowMode};
use i_slint_backend_testing::{ElementHandle, ElementQuery, init_no_event_loop};
use slint::{ModelRc, SharedString, VecModel};

type TestResult = Result<(), Box<dyn Error>>;

const OPTIONS: [(&str, &str, &str); 3] = [
  ("opt-a", "Yes", "view-1"),
  ("opt-b", "No", "view-1"),
  ("opt-c", "Ask later", "view-1"),
];

fn window() -> Result<PromptWindow, Box<dyn Error>> {
  init_no_event_loop();
  Ok(PromptWindow::new()?)
}

fn rows(entries: &[(&str, &str, &str)]) -> ModelRc<OptionRow> {
  let rows: Vec<OptionRow> = entries
    .iter()
    .map(|(id, label, view)| OptionRow {
      id: SharedString::from(*id),
      label: SharedString::from(*label),
      view: SharedString::from(*view),
    })
    .collect();
  ModelRc::new(VecModel::from(rows))
}

/// The identity the tests select on, and the only unambiguous one (D10,
/// R-14): never by label, which two options may share.
fn element_described(window: &PromptWindow, description: &str) -> Option<ElementHandle> {
  let description = description.to_string();
  ElementQuery::from_root(window)
    .match_predicate(move |element| {
      element.accessible_description().as_deref() == Some(description.as_str())
    })
    .find_first()
}

/// VT-1 — item 6 (AC-10, R5). Every other assertion in this file rests on
/// the element query API being live; without `with_debug_info` it returns
/// empty and every one of them passes vacuously (E-1, A-3).
#[test]
fn a_known_element_is_found() -> TestResult {
  let window = window()?;
  let options_list = ElementHandle::find_by_accessible_label(&window, "options").next();
  assert!(
    options_list.is_some(),
    "the options list container must be found; its absence means debug info \
     is missing, not that the window has no options (S-3)"
  );
  Ok(())
}

/// VT-2 — item 7 (AC-4, E-2). Heading, body and one control per option, in
/// the order given, counted through `accessible-item-count` — never
/// `find_all().len()`, because the list virtualises.
#[test]
fn heading_body_and_one_control_per_option_render_in_order() -> TestResult {
  let window = window()?;
  window.set_heading("Proceed?".into());
  window.set_options(rows(&OPTIONS));

  let heading_found = ElementHandle::find_by_accessible_label(&window, "Proceed?").next();
  assert!(
    heading_found.is_some(),
    "the heading Text must be found by its label"
  );

  let body_found = ElementQuery::from_root(&window)
    .match_inherits("StyledText")
    .find_first();
  assert!(
    body_found.is_some(),
    "a StyledText is present in prompt mode; its content is asserted on \
     Presentation in the mapper tier, not here (design.md §5.2)"
  );

  let options_list = ElementHandle::find_by_accessible_label(&window, "options")
    .next()
    .ok_or("the options list container must be found")?;
  assert_eq!(options_list.accessible_item_count(), Some(OPTIONS.len()));

  for (index, (id, label, _view)) in OPTIONS.iter().enumerate() {
    let button = element_described(&window, id)
      .ok_or_else(|| format!("no control with accessible-description {id:?}"))?;
    assert_eq!(button.accessible_item_index(), Some(index));
    assert_eq!(button.accessible_label().as_deref(), Some(*label));
  }
  Ok(())
}

/// VT-3 — item 8 (AC-5, R-14, D10). Activating a control fires `chosen`
/// with the right `OptionId` **and** the right view token — including when
/// two options share a label, which is why selection is by
/// `accessible_description` and never by label.
#[test]
fn activating_a_control_fires_chosen_with_the_right_id_and_view_token() -> TestResult {
  let window = window()?;
  window.set_options(rows(&[
    ("opt-a", "Yes", "view-1"),
    ("opt-b", "Yes", "view-1"), // shares opt-a's label, deliberately (R-14)
  ]));

  let captured: Rc<RefCell<Option<(String, String)>>> = Rc::new(RefCell::new(None));
  {
    let captured = Rc::clone(&captured);
    window.on_chosen(move |view, id| {
      *captured.borrow_mut() = Some((view.to_string(), id.to_string()));
    });
  }

  let second = element_described(&window, "opt-b").ok_or("no control described opt-b")?;
  second.invoke_accessible_default_action();

  assert_eq!(
    *captured.borrow(),
    Some(("view-1".to_string(), "opt-b".to_string()))
  );
  Ok(())
}

/// VT-4 — item 9 (AC-6, AC-11, E-1). The diagnostic empty state, asserted
/// by presence when there is nothing to report and by absence once there
/// is — the pairing E-1 requires of every absence-shaped assertion.
///
/// VT-5 — item 10 (AC-11, R3): the absence half is not vacuous. Broken by
/// changing `ui/app.slint`'s guard from
/// `if root.diagnostic-lines.length == 0` to `if true`, so the placeholder
/// never leaves; rerun with `cargo test -p goad --test renderer
/// the_diagnostic_empty_state_is_present_only_when_empty`, reverted after.
/// Output pasted in `notes.md`.
#[test]
fn the_diagnostic_empty_state_is_present_only_when_empty() -> TestResult {
  let window = window()?;
  window.set_mode(WindowMode::Diagnostic);
  window.set_diagnostic_lines(ModelRc::new(VecModel::from(Vec::<SharedString>::new())));
  assert!(
    ElementHandle::find_by_accessible_label(&window, "Nothing to report.")
      .next()
      .is_some(),
    "the empty-state placeholder must be present when there are no lines"
  );

  window.set_diagnostic_lines(ModelRc::new(VecModel::from(vec![SharedString::from(
    "a diagnostic line",
  )])));
  assert!(
    ElementHandle::find_by_accessible_label(&window, "Nothing to report.")
      .next()
      .is_none(),
    "the placeholder must not survive once a line exists"
  );
  Ok(())
}

/// F-1 (review-code 002, round 1). AC-9's degradation half: the "shown as
/// plain text" marker is present exactly when `body-degraded` is set, and
/// absent otherwise — the same presence/absence pairing E-1 requires,
/// applied to `app.slint:37` rather than to the diagnostic empty state.
///
/// Broken by deleting `if root.body-degraded: Text { text: "shown as plain
/// text"; }` from `ui/app.slint`: this test fails (the presence half, once
/// `body_degraded` is true, finds nothing). Reverted after; output pasted
/// in `notes.md`.
#[test]
fn the_degradation_marker_is_present_only_when_the_body_is_degraded() -> TestResult {
  let window = window()?;
  window.set_body_degraded(false);
  assert!(
    ElementHandle::find_by_accessible_label(&window, "shown as plain text")
      .next()
      .is_none(),
    "the marker must not appear when the body is not degraded"
  );

  window.set_body_degraded(true);
  assert!(
    ElementHandle::find_by_accessible_label(&window, "shown as plain text")
      .next()
      .is_some(),
    "the marker must appear once the body is degraded"
  );
  Ok(())
}

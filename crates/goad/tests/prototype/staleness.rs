//! **I-H** — an entry is used only against the view it was made on
//! (design.md §5.5).
//!
//! The rule matters because the map is keyed by (option, field) and those are
//! strings a replacement view is free to reuse. Two of I-H's three sites are
//! probed here: an entry is *shown* only on its own view, and a stale entry
//! *drained* into a `Choose` is refused rather than recorded. The third — the
//! timer's `Command::Edit` carrying the entry's own view — is the same check
//! in `controller::edit` and is reached by the second probe's own call.

use goad::diagnostics::Refused;
use goad::draft::Reported;
use goad::glass::Glass;
use goad::wire::PendingEdit;

use crate::harness::{A_FORM, retaining, rig, value_of};

fn typed_on(view: &str) -> PendingEdit {
  PendingEdit {
    view: view.to_owned(),
    option: "morning".to_owned(),
    field: "noted".to_owned(),
    value: Reported::Typed("from the old view".to_owned()),
  }
}

/// The first site. The replacement declares the same option and the same
/// field, which is the whole difficulty: the key matches and the view does
/// not.
///
/// **Driven red**: dropping `&& held.view == view` from `Pending::shown`
/// writes the old view's typing into the new view's widget and fails here.
#[test]
fn an_entry_made_on_a_replaced_view_is_not_shown_on_the_replacement() {
  let mut rig = rig();
  rig.pending.record(&rig.wire, typed_on("v1"));

  let on_its_own_view = retaining("v1", A_FORM);
  rig.glass.present(on_its_own_view.frame(false));
  assert_eq!(
    value_of(&rig.window, "morning", "noted").map(|value| value.text.to_string()),
    Some("from the old view".to_owned()),
    "the entry's own view shows it — otherwise the assertion below proves nothing"
  );

  let replacement = retaining("v2", A_FORM);
  rig.glass.present(replacement.frame(false));
  assert_eq!(
    value_of(&rig.window, "morning", "noted").map(|value| value.text.to_string()),
    Some(String::new()),
    "the replacement's `noted` is untouched, whatever the old view's entry says"
  );
}

/// The third site. A `Choose` on the retained view carrying an edit made on a
/// replaced one: that edit is refused `SupersededView` and **the answer still
/// goes** — it is about the view that is retained, and nothing about it is
/// incomplete (§5.5's edge table).
#[test]
fn a_stale_carried_edit_is_refused_and_the_answer_still_goes() {
  let mut controller = retaining("v2", A_FORM);

  let (view_id, answer) = controller
    .choose("v2", "morning", vec![typed_on("v1")])
    .expect("the answer is about the retained view and still goes");

  assert_eq!(view_id.as_str(), "v2");
  assert_eq!(
    answer
      .values
      .iter()
      .find(|(id, _)| id.as_str() == "noted")
      .map(|(_, value)| value.clone()),
    Some(serde_json::json!("")),
    "the stale edit recorded nothing: `noted` answers as it was drawn"
  );
  assert!(
    controller
      .frame(false)
      .diagnostics
      .lines()
      .iter()
      .any(|line| line.contains("has since been replaced")),
    "and the person is told the typing was discarded: {:?}",
    controller.frame(false).diagnostics.lines()
  );
}

/// The other refusal a carried edit can earn, and it is not the same posture.
/// An option or field the retained view does not declare is the markup and the
/// retained presentation disagreeing — a renderer bug rather than a race — so
/// **no answer is sent**.
#[test]
fn a_carried_edit_naming_an_undeclared_field_stops_the_answer() {
  let mut controller = retaining("v1", A_FORM);

  assert_eq!(
    controller.choose(
      "v1",
      "morning",
      vec![PendingEdit {
        view: "v1".to_owned(),
        option: "morning".to_owned(),
        field: "never-declared".to_owned(),
        value: Reported::Typed("x".to_owned()),
      }]
    ),
    Err(Refused::UnknownField),
    "an answer the host knows was built from an incomplete draft is worse than a refusal"
  );
}

/// Identity of the **command** is checked first, so a `Choose` on a replaced
/// view refuses the whole thing and records nothing — including its carried
/// edits, however fresh they are.
#[test]
fn a_choose_on_a_replaced_view_records_none_of_its_carried_edits() {
  let mut controller = retaining("v2", A_FORM);

  assert_eq!(
    controller
      .choose("v1", "morning", vec![typed_on("v2")])
      .err(),
    Some(Refused::SupersededView)
  );

  let (_, answer) = controller
    .answer("v2", "morning")
    .expect("the retained view still answers");
  assert_eq!(
    answer
      .values
      .iter()
      .find(|(id, _)| id.as_str() == "noted")
      .map(|(_, value)| value.clone()),
    Some(serde_json::json!("")),
    "the carried edit named the retained view and was still not applied"
  );
}

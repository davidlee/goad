//! What the person did, and the one place it becomes a submitted value
//! (design.md §5.2, §5.3).
//!
//! A module of its own rather than a map inside `reception.rs` because it has
//! rules of its own — absent means as-drawn, keys are (option, field) pairs,
//! and `submitted` is the single application of `SPEC-001/R-57`. Pure: no
//! clock, no file, no socket, and no Slint type. Stratum 3, and the pure half
//! of it.
//!
//! What it holds is deliberately unreachable from outside. `answer()` builds a
//! response by walking the *presentation's* declared fields and looking each
//! one up here — never by walking this — and the absence of any way to
//! enumerate what is held is what makes that a property of the type rather
//! than a convention (`SPEC-001/R-58`, design.md §5.1).

use goad_semantics::protocol::canonical::{FieldId, OptionId};

/// The draft's value type, and the one place a widget's state becomes a
/// submitted value.
///
/// One variant, deliberately. The seam exists so that drawing a second kind
/// later is a `submitted` arm rather than a reshaping of `Command`, `Draft`,
/// `install.rs` and every test that builds a command — it is not an
/// invitation to write the second variant before something draws it
/// (design.md §7, D11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Edited {
  Checked(bool),
}

/// Keyed by (option, field): `SPEC-001/R-52` makes a field id unique only
/// *within* an option, and two options sharing one is legal and fixtured, so
/// the option half is what keeps two ticks apart.
///
/// Not a `BTreeMap`, because `OptionId` is deliberately not `Ord`: in
/// `canonical.rs` an id is ordered exactly when the protocol keys a serialized
/// map by it — `FieldId` is and says so, `OptionId` and `AlternativeId` are
/// not — and a host's storage is no reason to spend that distinction. A view's
/// options and a form's fields are both handfuls; the scan is free.
///
/// No `PartialEq`: over a `Vec` it would be insertion-order sensitive, and a
/// draft's behaviour is `state_of`, not its representation. Tests assert
/// through the accessor.
#[derive(Debug, Default, Clone)]
pub struct Draft(Vec<(OptionId, FieldId, Edited)>);

impl Draft {
  /// What the field should show. Absent means **as drawn**: the protocol
  /// carries no `field.value`, so a checkbox nobody has touched is unticked.
  #[must_use]
  pub fn state_of(&self, option: &OptionId, field: &FieldId) -> Edited {
    self
      .0
      .iter()
      .find(|(held_option, held_field, _)| held_option == option && held_field == field)
      .map_or(Edited::Checked(false), |(_, _, value)| value.clone())
  }

  /// Records what the person did to one field of one option, replacing
  /// whatever that key held.
  ///
  /// Written as a remove-then-append rather than an in-place update because
  /// the two are indistinguishable from outside: nothing enumerates the
  /// draft and it carries no `PartialEq`, so insertion order is not part of
  /// its behaviour. The spelling that does not need the borrow checker
  /// argued with is the one to take.
  pub fn record(&mut self, option: OptionId, field: FieldId, value: Edited) {
    self
      .0
      .retain(|(held_option, held_field, _)| held_option != &option || held_field != &field);
    self.0.push((option, field, value));
  }
}

/// `SPEC-001/R-57`, in one total match: one site turns a widget's state into a
/// submitted value, so the mapping cannot drift apart across the codebase.
///
/// It is **not** the compiler's guard against the *protocol* growing a kind.
/// `Edited` is host-local; a sixth `FieldKind` leaves this match exhaustive.
/// The site that breaks is the mapper arm in `present()`, which matches the
/// canonical `FieldKind` and must sort the new kind into drawn or
/// `Undrawn::FieldForm`. What this match guards is the *host* growing a drawn
/// kind without deciding what it submits.
#[cfg_attr(
  not(test),
  expect(
    dead_code,
    reason = "PHASE-04 calls this from `controller.rs::answer`. A type that \
              lands one phase before its caller is the transient case \
              `Cargo.toml` names where it holds `dead_code` at `warn` rather \
              than `deny`; the attribute is scoped to `not(test)` because the \
              unit tests below are already a caller. Self-clearing: \
              `unfulfilled_lint_expectations` fails the gate the moment \
              `controller.rs` calls it."
  )
)]
pub(crate) fn submitted(edited: &Edited) -> serde_json::Value {
  match edited {
    Edited::Checked(value) => serde_json::Value::Bool(*value),
  }
}

// The crate-external `tests/renderer/` tiers cannot reach a `pub(crate)`
// function, and `submitted` is `pub(crate)` for the reason its own doc gives.
// Tested here, inline, in the `#[cfg(test)] mod tests` shape `controller.rs`,
// `goad-shell/src/state.rs` and `goad-semantics/src/schedule.rs` already use
// for a stratum-internal pure function.
#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::{FieldId, OptionId, Timestamp, View};
  use goad_semantics::protocol::normalize::read_response;

  use super::{Draft, Edited, submitted};

  /// Ids can only be *read* off a normalized view: `OptionId::new` and
  /// `FieldId::new` are `pub(super)` in `goad-semantics`, so this renderer
  /// clones an id and cannot mint one (design.md §5.2). This fixture is what
  /// that costs — a one-option, one-field view, parsed, and its two ids
  /// taken.
  fn ids(option: &str, field: &str) -> (OptionId, FieldId) {
    let document = serde_json::json!({
      "view": {
        "kind": "choice",
        "title": "T",
        "options": [{
          "id": option,
          "label": "L",
          "fields": [{ "id": field, "kind": "boolean", "label": "F" }]
        }]
      }
    });
    let now = Timestamp::new(
      "2026-01-01T00:00:00Z"
        .parse()
        .expect("the fixture must be an instant"),
    );
    let view = read_response(document.to_string().as_bytes(), now)
      .expect("the fixture must normalize")
      .value
      .view()
      .expect("the fixture carries a view")
      .clone();
    let View::Choice(choice) = &view;
    let opt = choice
      .options()
      .as_slice()
      .first()
      .expect("the fixture's one option");
    let declared = opt.fields().as_slice().first().expect("its one field");
    (opt.id().clone(), declared.id().clone())
  }

  /// VT-1, first clause: nothing recorded is not an error and not an absent
  /// answer — it is the box as it was drawn, which for a checkbox is
  /// unticked. The protocol carries no `field.value` to say otherwise.
  #[test]
  fn an_unrecorded_field_reads_as_drawn() {
    let (option, field) = ids("opt", "one");
    assert_eq!(
      Draft::default().state_of(&option, &field),
      Edited::Checked(false)
    );
  }

  /// VT-1, second clause: `SPEC-001/R-52` scopes a field id to its option, so
  /// two options may declare the same one and the spec fixtures that case as
  /// legal. The option half of the key is what keeps the two ticks apart.
  #[test]
  fn one_field_id_under_two_options_is_two_independent_keys() {
    let (first, field) = ids("opt-a", "shared");
    let (second, _) = ids("opt-b", "shared");
    let mut draft = Draft::default();

    draft.record(first.clone(), field.clone(), Edited::Checked(true));

    assert_eq!(draft.state_of(&first, &field), Edited::Checked(true));
    assert_eq!(
      draft.state_of(&second, &field),
      Edited::Checked(false),
      "the other option's field of the same name is a different key"
    );
  }

  /// VT-1, third clause. Asserted in both directions from a **ticked** start,
  /// so neither assertion is one a `state_of` that always answers as-drawn
  /// could also supply (`docs/memory/a-green-test-can-assert-a-proxy.md`).
  #[test]
  fn recording_a_field_again_replaces_what_it_held() {
    let (option, field) = ids("opt", "one");
    let mut draft = Draft::default();

    draft.record(option.clone(), field.clone(), Edited::Checked(true));
    assert_eq!(draft.state_of(&option, &field), Edited::Checked(true));

    draft.record(option.clone(), field.clone(), Edited::Checked(false));
    assert_eq!(
      draft.state_of(&option, &field),
      Edited::Checked(false),
      "the second record replaces the first rather than sitting behind it"
    );
  }

  /// VT-2 — `SPEC-001/R-57`'s `boolean` clause, at the one site that applies
  /// it. This is the test that requirement's `SPEC-001` §7 row will name.
  #[test]
  fn a_boolean_field_submits_a_json_boolean() {
    assert_eq!(
      submitted(&Edited::Checked(true)),
      serde_json::Value::Bool(true)
    );
    assert_eq!(
      submitted(&Edited::Checked(false)),
      serde_json::Value::Bool(false)
    );
  }
}

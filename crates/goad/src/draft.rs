//! What the person did, and the one place it becomes a submitted value
//! (design.md §5.2, §5.3).
//!
//! A module of its own rather than a map inside `reception.rs` because it has
//! rules of its own — keys are (option, field) pairs, absent is answerable
//! only by something that knows the field's kind, and `submitted` is the
//! single application of `SPEC-001/R-57`. Pure: no
//! clock, no file, no socket, and no Slint type. Stratum 3, and the pure half
//! of it.
//!
//! What it holds is deliberately unreachable from outside. `answer()` builds a
//! response by walking the *presentation's* declared fields and looking each
//! one up here — never by walking this — and the absence of any way to
//! enumerate what is held is what makes that a property of the type rather
//! than a convention (`SPEC-001/R-58`, design.md §5.1).

use goad_semantics::protocol::canonical::{AlternativeId, FieldId, OptionId, Timestamp};
use jiff::tz::Offset;

/// A finite `f64`: private field, fallible constructor, so the only way to
/// hold one is to have checked it.
///
/// This is I-G as a property of the type rather than of the call sites.
/// `serde_json::Value::from(f64)` turns `NaN` and both infinities into JSON
/// `null`, which `SPEC-001/R-57` does not admit, so a non-finite submitted
/// number is not merely unwritten here — it is unrepresentable
/// (design.md §5.2, §5.5 I-G).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Finite(f64);

/// Sound, not a convenience: `Finite` excludes `NaN`, which is the only `f64`
/// that makes `PartialEq` less than an equivalence relation. Every value this
/// type can hold is equal to itself, so `Edited` — which carries one — can
/// keep the `Eq` it has always had.
impl Eq for Finite {}

impl Finite {
  pub const ZERO: Self = Self(0.0);

  /// `None` for `NaN` and both infinities, and nothing else.
  #[must_use]
  pub fn new(value: f64) -> Option<Self> {
    value.is_finite().then_some(Self(value))
  }

  #[must_use]
  pub fn get(self) -> f64 {
    self.0
  }
}

/// The draft's value type, and the one place a widget's state becomes a
/// submitted value.
///
/// One variant per drawn kind, and the mapping to JSON is `submitted` below.
/// `Adjusted` holds the text beside the number because the text is what the
/// widget displays and what the guard compares, while `submitted` reads the
/// number and never the text (design.md §5.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Edited {
  /// `SPEC-001/R-57`: JSON boolean.
  Checked(bool),
  /// `SPEC-001/R-57`: JSON string.
  Typed(String),
  /// `SPEC-001/R-57`: JSON number, from `number`. `text` is what was typed
  /// and never reaches the wire.
  Adjusted { number: Finite, text: String },
  /// `SPEC-001/R-57`: the alternative's id, as a string.
  ///
  /// An `AlternativeId` rather than a `String` because `AlternativeId::new`
  /// is `pub(super)` in `goad-semantics`: this crate can clone one off a view
  /// it drew and cannot mint one, which is how AC-8 and `R-52` are held
  /// (§5.5 I-D).
  Chosen(AlternativeId),
  /// `SPEC-001/R-57`: RFC 3339, with an explicit offset.
  Picked { instant: Timestamp, offset: Offset },
}

/// What a widget reported, in the widget's own terms — the most a Slint
/// callback can know. `Command::Edit` and `PendingEdit` carry it; `resolve`
/// (`view_model.rs`) turns it into an [`Edited`].
///
/// Two of the five values can only be formed where the retained presentation
/// is, and a callback is not there. `Chosen` is the first: an `AlternativeId`
/// can only be cloned off a drawn view, so the index travels and is resolved
/// against the drawn field's alternatives. `AdjustedText` is the second: a
/// text that does not parse finitely keeps the number the field already
/// holds, and only the draft — or, for an untouched field, the value it was
/// drawn showing — knows what that is (design.md §5.2, D12).
///
/// Six variants for five kinds, because a `number` has two controls and which
/// one is drawn is already a first-class decision (D16, D17).
///
/// It does **not** hold I-G. `AdjustedValue` is a bare `f32` and admits both
/// infinities and `NaN`: a `Slider` callback has nothing to report a refusal
/// to and no draft to leave alone, so the check belongs one step later. I-G is
/// held by `Finite` at the wire and by `resolve` at the boundary (F-42).
#[derive(Debug, Clone, PartialEq)]
pub enum Reported {
  /// `CheckBox`.
  Checked(bool),
  /// The text `LineEdit`.
  Typed(String),
  /// The numeric `LineEdit` — the text as typed, unparsed.
  AdjustedText(String),
  /// `Slider` — its own `value`, an `f32` by nature (D16).
  AdjustedValue(f32),
  /// `ComboBox` — its `current-index`.
  Chosen(u32),
  /// Composed in the callback, by `instant.rs`.
  Picked { instant: Timestamp, offset: Offset },
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
  /// What the person recorded for this field, or `None` for a field nobody
  /// has touched.
  ///
  /// It used to answer `Checked(false)` for an absent key, which worked while
  /// a boolean was the only kind. It cannot answer for the other four,
  /// because the right answer depends on the kind and the draft does not know
  /// the kind: that is `view_model::as_drawn`'s job, and the callers of this
  /// deliberately do not treat the `None` alike. `controller.rs` applies
  /// `as_drawn`, because `SPEC-001/R-58` forbids omitting a value for a drawn
  /// field; `glass.rs` reads the `None` directly, because a `datetime` button
  /// reading `1970-01-01T00:00:00+00:00` would be the host showing a person
  /// an answer nobody gave (design.md §5.2).
  #[must_use]
  pub fn state_of(&self, option: &OptionId, field: &FieldId) -> Option<Edited> {
    self
      .0
      .iter()
      .find(|(held_option, held_field, _)| held_option == option && held_field == field)
      .map(|(_, _, value)| value.clone())
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
pub(crate) fn submitted(edited: &Edited) -> serde_json::Value {
  match edited {
    Edited::Checked(value) => serde_json::Value::Bool(*value),
    Edited::Typed(text) => serde_json::Value::String(text.clone()),
    // `Value::from(f64)` answers `null` for a non-finite, which `R-57` does
    // not admit. `Finite` is what makes that unreachable, and it is the whole
    // of what makes this arm a JSON number (§5.5 I-G).
    Edited::Adjusted { number, .. } => serde_json::Value::from(number.get()),
    Edited::Chosen(alternative) => serde_json::Value::String(alternative.as_str().to_owned()),
    // The offset is explicit rather than `Z`: it is the offset the person
    // picked in, and it falls out of the same call for every datetime,
    // including the epoch an untouched field submits (D-6, D-7).
    Edited::Picked { instant, offset } => {
      serde_json::Value::String(instant.instant().display_with_offset(*offset).to_string())
    }
  }
}

// The crate-external `tests/renderer/` tiers cannot reach a `pub(crate)`
// function, and `submitted` is `pub(crate)` for the reason its own doc gives.
// Tested here, inline, in the `#[cfg(test)] mod tests` shape `controller.rs`,
// `goad-shell/src/state.rs` and `goad-semantics/src/schedule.rs` already use
// for a stratum-internal pure function.
#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::Timestamp;
  use jiff::tz::Offset;

  use super::{Draft, Edited, Finite, submitted};
  use crate::fixture::{alternative_id, ids};

  /// VT-1, first clause: nothing recorded is no longer an answer this module
  /// can give. It used to read as `Checked(false)`, which was right while a
  /// boolean was the only kind; the right answer now depends on the kind, and
  /// the kind is `view_model::as_drawn`'s to know.
  #[test]
  fn an_unrecorded_field_is_absent() {
    let (option, field) = ids("opt", "one");
    assert_eq!(Draft::default().state_of(&option, &field), None);
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

    assert_eq!(draft.state_of(&first, &field), Some(Edited::Checked(true)));
    assert_eq!(
      draft.state_of(&second, &field),
      None,
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
    assert_eq!(draft.state_of(&option, &field), Some(Edited::Checked(true)));

    draft.record(option.clone(), field.clone(), Edited::Checked(false));
    assert_eq!(
      draft.state_of(&option, &field),
      Some(Edited::Checked(false)),
      "the second record replaces the first rather than sitting behind it"
    );
  }

  /// I-G at its narrowest: the three `f64` values `serde_json` would turn
  /// into JSON `null`, refused by the only constructor there is.
  #[test]
  fn finite_refuses_nan_and_both_infinities() {
    assert_eq!(Finite::new(f64::NAN), None);
    assert_eq!(Finite::new(f64::INFINITY), None);
    assert_eq!(Finite::new(f64::NEG_INFINITY), None);
  }

  /// The other half of it: every finite `f64` is admitted, including the two
  /// extremes a `number` bound may legally carry under `R-17`.
  #[test]
  fn finite_admits_every_finite_number() {
    for value in [0.0, -0.0, 1.5, f64::MIN, f64::MAX, f64::MIN_POSITIVE] {
      assert_eq!(
        Finite::new(value).map(Finite::get),
        Some(value),
        "{value} is finite and must be admitted unchanged"
      );
    }
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

  /// `R-57`'s `text` clause. Verbatim, including the empty string: an empty
  /// text field answers `""`, not an omitted key.
  #[test]
  fn a_text_field_submits_a_json_string() {
    assert_eq!(
      submitted(&Edited::Typed("  spaced  ".to_owned())),
      serde_json::json!("  spaced  ")
    );
    assert_eq!(
      submitted(&Edited::Typed(String::new())),
      serde_json::json!("")
    );
  }

  /// `R-57`'s `number` clause, and I-G at the wire: the number is submitted
  /// and the text beside it never is. Asserted against a value whose text
  /// form differs from its own, so a `submitted` that read the text would
  /// fail rather than coincide.
  #[test]
  fn a_number_field_submits_the_number_and_never_the_text() {
    let edited = Edited::Adjusted {
      number: Finite::new(1.5).expect("1.5 is finite"),
      text: "1,5".to_owned(),
    };
    assert_eq!(submitted(&edited), serde_json::json!(1.5));
    assert!(
      submitted(&edited).is_number(),
      "`R-57` admits a JSON number here and nothing else"
    );
  }

  /// `R-57`'s `choice` clause: the alternative's **id**, as a JSON string —
  /// not its label and not its index.
  #[test]
  fn a_choice_field_submits_the_alternative_id_as_a_string() {
    let second = alternative_id(&["red", "green"], 1);
    assert_eq!(
      submitted(&Edited::Chosen(second)),
      serde_json::json!("green")
    );
  }

  /// `R-57`'s `datetime` clause: RFC 3339 with an **explicit** offset, which
  /// is the offset the person picked in. `+00:00` rather than `Z` even at the
  /// epoch, because it falls out of the same call as every other datetime
  /// (D-6, D-7).
  #[test]
  fn a_datetime_field_submits_rfc_3339_with_an_explicit_offset() {
    let epoch = Timestamp::new(jiff::Timestamp::UNIX_EPOCH);
    assert_eq!(
      submitted(&Edited::Picked {
        instant: epoch,
        offset: Offset::UTC,
      }),
      serde_json::json!("1970-01-01T00:00:00+00:00")
    );
    assert_eq!(
      submitted(&Edited::Picked {
        instant: epoch,
        offset: Offset::constant(-5),
      }),
      serde_json::json!("1969-12-31T19:00:00-05:00"),
      "the instant is the same one; the offset is what the person picked in"
    );
  }
}

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

use goad_semantics::protocol::canonical::{
  AlternativeId, FieldId, Finite, OptionId, Submitted, Timestamp,
};
use jiff::tz::Offset;

/// What the draft holds, and the only thing `submitted` maps.
///
/// One variant per drawn field kind, and the `submitted` match below is what
/// makes adding a sixth a decision about what it puts on the wire rather than
/// an omission (design.md §5.2).
///
/// **No `Eq`**, because `Adjusted` carries a `Finite` and the leaf declines it
/// — see `Finite`'s own doc for why a hand-written impl anywhere in this
/// tower is the wrong answer. Nothing needs it: the cases that compare an
/// `Edited` or a `Command` need `PartialEq` only.
#[derive(Debug, Clone, PartialEq)]
pub enum Edited {
  /// `R-57`: a JSON boolean.
  Checked(bool),
  /// `R-57`: a JSON string.
  Typed(String),
  /// `R-57`: a JSON number, taken from `number`.
  ///
  /// `text` is what the person typed — what the widget displays and what its
  /// guard compares itself against — and it never reaches the wire. The two
  /// are separate because `f64` → text is not injective, so no re-format of
  /// the number can stand in for the text a person is halfway through typing
  /// (§5.2, §7 D13).
  Adjusted { number: Finite, text: String },
  /// `R-57`: the alternative's id, as a string.
  ///
  /// An `AlternativeId` and not a `String`: `AlternativeId::new` is
  /// `pub(super)` in `goad-semantics`, so this crate can only clone one off a
  /// view it drew. That is how `R-52` and §5.5 I-D are held by the types
  /// rather than by a check (§7 D12).
  Chosen(AlternativeId),
  /// `R-57`: RFC 3339, carrying the offset the person picked in (§7 D4).
  Picked { instant: Timestamp, offset: Offset },
}

impl Edited {
  /// What this field submits, as stratum 1 names it: a projection, one
  /// variant onto its counterpart, that decides no JSON type.
  /// `Submitted::to_json` decides that.
  #[must_use]
  pub fn submitted(&self) -> Submitted {
    match self {
      Self::Checked(value) => Submitted::Boolean(*value),
      Self::Typed(text) => Submitted::Text(text.clone()),
      // The number, and never the text beside it: the text is what the widget
      // displays and §5.2 keeps it off the wire.
      Self::Adjusted { number, .. } => Submitted::Number(*number),
      Self::Chosen(alternative) => Submitted::Choice(alternative.clone()),
      Self::Picked { instant, offset } => Submitted::DateTime {
        instant: *instant,
        offset: *offset,
      },
    }
  }
}

/// What a widget reported, in the widget's own terms — the most a Slint
/// callback can honestly say.
///
/// Six variants for five kinds, because a `number` has two controls and which
/// one is drawn is already a first-class decision. The asymmetry with `Edited`
/// is the point: a callback cannot mint an `AlternativeId` and cannot know the
/// last representable number a numeric field held, so both are interpreted
/// where the retained presentation is — `view_model::interpret` (§5.2, §7
/// D25).
///
/// It does **not** hold §5.5 I-G: `AdjustedValue(f32)` admits `NaN` and both
/// infinities like any other `f32`. A non-finite is refused by `interpret` at
/// the boundary and by `Finite` at the wire, and by neither boundary type.
#[derive(Debug, Clone, PartialEq)]
pub enum Reported {
  /// `CheckBox`.
  Checked(bool),
  /// The text `LineEdit`.
  Typed(String),
  /// The numeric `LineEdit` — the text as typed, unparsed and unrepaired.
  AdjustedText(String),
  /// A `Slider`'s own value, an `f32` by nature (§7 D16).
  AdjustedValue(f32),
  /// A `ComboBox`'s `current-index`.
  Chosen(u32),
  /// Composed in the callback, from the two pickers.
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
  /// What the draft holds for this field, or `None` where nobody has touched
  /// it.
  ///
  /// An `Option`, where this used to answer `Checked(false)` for an absent
  /// key. That worked while a boolean was the only kind and cannot answer for
  /// the other four: the right answer depends on the field's kind, and the
  /// draft does not know the kind (design.md §5.2).
  ///
  /// **The two callers do not treat the `None` alike, and that is the point.**
  /// `glass.rs` reads it as *untouched* and shows the field as it was drawn;
  /// `controller::answer` applies `view_model::as_drawn`, because
  /// `SPEC-001/R-58` forbids omitting a value for a drawn field. Keeping
  /// `glass.rs` out of `as_drawn` is what makes the `datetime` epoch a fact
  /// about the wire rather than a fact about the screen.
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
/// The site that breaks is `view_model::drawn_form`, which matches the
/// canonical `FieldKind` and must sort the new kind into drawn or reported.
/// Reported is no longer somewhere it can simply go: every kind draws, so
/// `FieldForm` is uninhabited and the destination has to be **re-created** —
/// a variant given back — before a new kind can be sent there. What this match
/// guards is the *host* growing a drawn kind without deciding what it
/// submits.
pub(crate) fn submitted(edited: &Edited) -> serde_json::Value {
  edited.submitted().to_json()
}

// The crate-external `tests/renderer/` tiers cannot reach a `pub(crate)`
// function, and `submitted` is `pub(crate)` for the reason its own doc gives.
// Tested here, inline, in the `#[cfg(test)] mod tests` shape `controller.rs`,
// `goad-shell/src/state.rs` and `goad-semantics/src/schedule.rs` already use
// for a stratum-internal pure function.
#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::{
    AlternativeId, FieldId, FieldKind, Finite, Opt, OptionId, Submitted, Timestamp, View,
  };
  use goad_semantics::protocol::normalize::read_response;
  use jiff::tz::Offset;

  use super::{Draft, Edited};

  /// No id in this module is minted, and none can be: `OptionId::new`,
  /// `FieldId::new` and `AlternativeId::new` are all `pub(super)` in
  /// `goad-semantics` (§5.5 I-D). A unit that needs one parses a view and
  /// clones it off, which is what this fixture is for — a one-option view
  /// carrying one field of the caller's own wire form.
  fn an_option_declaring(option: &str, field: &serde_json::Value) -> Opt {
    let document = serde_json::json!({
      "view": {
        "kind": "choice",
        "title": "T",
        "options": [{ "id": option, "label": "L", "fields": [field] }]
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
    choice
      .options()
      .as_slice()
      .first()
      .expect("the fixture's one option")
      .clone()
  }

  /// A one-option view's option id, paired with the id of the one `boolean`
  /// field it declares.
  fn ids(option: &str, field: &str) -> (OptionId, FieldId) {
    let declared = an_option_declaring(
      option,
      &serde_json::json!({ "id": field, "kind": "boolean", "label": "F" }),
    );
    let id = declared
      .fields()
      .as_slice()
      .first()
      .expect("its one field")
      .id()
      .clone();
    (declared.id().clone(), id)
  }

  /// The first alternative's id of a declared `choice`, read off a normalized
  /// view for the reason above.
  fn an_alternative_id() -> AlternativeId {
    let option = an_option_declaring(
      "opt",
      &serde_json::json!({
        "id": "pick",
        "kind": "choice",
        "label": "F",
        // The wire key is `options`, because that is the key a backend author
        // writes (`SPEC-001/R-16`, `R-53`); the canonical type is
        // `Alternatives`, because an alternative is a value and a view's
        // option is an action.
        "options": [
          { "id": "first", "label": "First" },
          { "id": "second", "label": "Second" }
        ]
      }),
    );
    let field = option.fields().as_slice().first().expect("its one field");
    let FieldKind::Choice { alternatives } = field.kind() else {
      panic!("the fixture declares a choice");
    };
    alternatives
      .as_slice()
      .first()
      .expect("`Alternatives::new` rejects an empty list")
      .id()
      .clone()
  }

  /// VT-1, first clause: nothing recorded is not an error and not an absent
  /// answer — it is *untouched*, and what an untouched field shows and
  /// submits depends on its kind, which the draft does not know. That is why
  /// this answers `None` rather than the `Checked(false)` it used to:
  /// `view_model::as_drawn` is where the as-drawn value is decided, and
  /// `as_drawn_answers_every_kind` there is where this clause's other half
  /// now lives (design.md §5.2, PHASE-02/EX-4).
  #[test]
  fn an_unrecorded_field_is_untouched_rather_than_a_value() {
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

  /// VT-3 — `Edited::submitted` is the identity on each kind: every variant
  /// lands on its counterpart carrying the same value, and an `Adjusted`'s
  /// text — the widget's, never the wire's — is the one thing dropped. Each
  /// expectation is built from the literal the `Edited` was built from, not
  /// from the `Edited` itself.
  #[test]
  fn the_projection_to_submitted_is_the_identity_on_each_kind() {
    assert_eq!(Edited::Checked(true).submitted(), Submitted::Boolean(true));
    assert_eq!(
      Edited::Typed("typed".to_owned()).submitted(),
      Submitted::Text("typed".to_owned())
    );
    assert_eq!(
      Edited::Adjusted {
        number: Finite::new(1.5).expect("1.5 is finite"),
        text: "1.50".to_owned(),
      }
      .submitted(),
      Submitted::Number(Finite::new(1.5).expect("1.5 is finite")),
      "the number, and never the text beside it"
    );
    assert_eq!(
      Edited::Chosen(an_alternative_id()).submitted(),
      Submitted::Choice(an_alternative_id())
    );
    let Submitted::Choice(chosen) = Edited::Chosen(an_alternative_id()).submitted() else {
      panic!("a chosen alternative projects to a choice");
    };
    assert_eq!(chosen.as_str(), "first", "the fixture's own id");
    assert_eq!(
      Edited::Picked {
        instant: Timestamp::new(jiff::Timestamp::UNIX_EPOCH),
        offset: Offset::constant(-5),
      }
      .submitted(),
      Submitted::DateTime {
        instant: Timestamp::new(jiff::Timestamp::UNIX_EPOCH),
        offset: Offset::constant(-5),
      }
    );
  }
}

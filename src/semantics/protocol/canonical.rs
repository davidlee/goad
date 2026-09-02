//! Canonical protocol types — `design.md` §5.2.
//!
//! Every field on an inbound type is `pub(super)`: writable from within
//! `semantics::protocol`, which is where normalization lives, and read-only to
//! everything else through the accessors. Outside this module a canonical value
//! can only have come out of `normalize_response`. That is P1 with a compiler
//! behind it rather than a comment (D30, I1), and widening these back to `pub`
//! is R10 — the named risk, not a tidy-up.
//!
//! The outbound request types are the deliberate exception. They are
//! host-authored, nothing untrusted reaches them (D5), and their fields are
//! public.
//!
//! This module handles backend-derived data, so it carries the module-level
//! deny D53 leaves to the modules that do. `semantics/error.rs` declines it and
//! says the first such module owes one; this is that module.
#![deny(clippy::arithmetic_side_effects)]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Serialize, Serializer};

use crate::semantics::error::{BoundsError, ProtocolError};

// ---------------------------------------------------------------------------
// Scalars
// ---------------------------------------------------------------------------
//
// Each is a newtype rather than a bare `String` so that a view id, an option
// id, a field id and an alternative id cannot be passed for one another: they
// are addresses in four different namespaces (I15, F-61) and the compiler is
// the cheapest place to say so.
//
// Only `ViewId` and `Timestamp` carry a public constructor. Both name values
// the *host* authors — a minted id (D13) and a clock read — so neither asserts
// that a backend said anything. `OptionId`, `AlternativeId` and `FieldId` are
// backend-authored addresses, and a public constructor would let a caller mint
// an id no backend ever sent; a caller answering a view clones the one the view
// carries. User decision 2026-08-29, `plan-log.md`.

/// `{now}#{seq}` — D13. Host-authored, so publicly constructible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ViewId(String);

impl ViewId {
  pub fn new(raw: impl Into<String>) -> Self {
    Self(raw.into())
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }
}

/// Names a *view's* option — what `UserResponse.option` selects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OptionId(String);

impl OptionId {
  pub(super) fn new(raw: impl Into<String>) -> Self {
    Self(raw.into())
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }
}

/// A value a `choice` field may take — what a response *submits*, as
/// `values[field_id]`. Not an [`OptionId`]: two namespaces, two types (F-61).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AlternativeId(String);

impl AlternativeId {
  pub(super) fn new(raw: impl Into<String>) -> Self {
    Self(raw.into())
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }
}

/// Keys `UserResponse.values`, hence `Ord`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct FieldId(String);

impl FieldId {
  pub(super) fn new(raw: impl Into<String>) -> Self {
    Self(raw.into())
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }
}

/// An instant, always supplied as `now` rather than read from a clock (I3).
///
/// jiff runs with `default-features = false` (D4), so there is no tzdb here and
/// nothing in this module can acquire one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(jiff::Timestamp);

impl Timestamp {
  pub fn new(instant: jiff::Timestamp) -> Self {
    Self(instant)
  }

  pub fn instant(self) -> jiff::Timestamp {
    self.0
  }
}

/// Written by hand rather than derived: jiff's own `serde` support is a
/// dependency-feature change, and A2 measured that `collect_str` over jiff's
/// `Display` already produces the RFC 3339 form `draft-spec.md:232` requires.
impl Serialize for Timestamp {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_str(&self.0)
  }
}

/// Opaque presentation hints. Nothing in `semantics/` or `shell/` branches on a
/// key here; the renderer is the only thing that may (I7).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hints(BTreeMap<String, serde_json::Value>);

impl Hints {
  pub(super) fn new(map: BTreeMap<String, serde_json::Value>) -> Self {
    Self(map)
  }

  pub fn as_map(&self) -> &BTreeMap<String, serde_json::Value> {
    &self.0
  }
}

// ---------------------------------------------------------------------------
// Inbound: canonical types
// ---------------------------------------------------------------------------

/// `view: None` is "nothing to show"; `schedule: None` is "no instruction
/// supplied". The two are independent, which is why neither is folded into the
/// other.
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
  pub(super) view: Option<View>,
  pub(super) schedule: Option<Timestamp>,
}

impl Response {
  pub fn view(&self) -> Option<&View> {
    self.view.as_ref()
  }

  pub fn schedule(&self) -> Option<Timestamp> {
    self.schedule
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
  Choice(Choice),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
  pub(super) title: String,
  pub(super) body: Option<Content>,
  pub(super) options: Options,
}

impl Choice {
  pub fn title(&self) -> &str {
    &self.title
  }

  pub fn body(&self) -> Option<&Content> {
    self.body.as_ref()
  }

  pub fn options(&self) -> &Options {
    &self.options
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Opt {
  pub(super) id: OptionId,
  pub(super) label: String,
  pub(super) fields: Fields,
}

impl Opt {
  pub fn id(&self) -> &OptionId {
    &self.id
  }

  pub fn label(&self) -> &str {
    &self.label
  }

  pub fn fields(&self) -> &Fields {
    &self.fields
  }
}

/// Four variants, none of which a v0 renderer draws. That is P3 with brief
/// §11.1 naming the future implementor — not dead weight to trim.
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
  Text(String),
  Markdown(String),
  Html(String),
  Uri(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
  pub(super) id: FieldId,
  pub(super) kind: FieldKind,
  pub(super) label: String,
  pub(super) hints: Hints,
}

impl Field {
  pub fn id(&self) -> &FieldId {
    &self.id
  }

  pub fn kind(&self) -> &FieldKind {
    &self.kind
  }

  pub fn label(&self) -> &str {
    &self.label
  }

  pub fn hints(&self) -> &Hints {
    &self.hints
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldKind {
  Text,
  Boolean,
  DateTime,
  Number(NumberRange),
  /// Not `Options`: an alternative is a value, not an action, and carries no
  /// fields of its own. F-54, D46.
  Choice {
    alternatives: Alternatives,
  },
}

/// Deliberately id and label only, and deliberately **not** an [`OptionId`]: a
/// view's option is *selected*, an alternative is *submitted*. F-61, D52.
#[derive(Debug, Clone, PartialEq)]
pub struct Alternative {
  pub(super) id: AlternativeId,
  pub(super) label: String,
}

impl Alternative {
  pub fn id(&self) -> &AlternativeId {
    &self.id
  }

  pub fn label(&self) -> &str {
    &self.label
  }
}

// ---------------------------------------------------------------------------
// Checked collections
// ---------------------------------------------------------------------------
//
// All three hold I15: every identifier a response names is unique within the
// scope that names it. `Options` and `Alternatives` additionally reject empty —
// a choice with no options is unrenderable and a `choice` field with no
// alternatives is unanswerable.
//
// `Fields` does **not**: R-15 says an option MAY carry fields, §5.5's table has
// no empty-fields row, the taxonomy has no `EmptyFields`, and `Opt.fields` is
// not an `Option`, so an option with no fields is a `Fields` holding none.
// `design.md:704`'s blanket comment over the three says otherwise and is
// over-general; the F-52 paragraph beneath it argues only duplicates. User
// decision 2026-08-30, `plan-log.md`.

/// I15 in one place: every identifier a response names is unique within the
/// scope that names it.
///
/// One helper rather than three copies of a `BTreeSet` walk, because
/// `design.md:717` states this as one rule deliberately and three copies would
/// be that document's own restatement defect expressed in code.
///
/// It covers **uniqueness only**. Non-emptiness is a second rule holding over
/// two of the three collections, not three, so it stays inline in the two
/// constructors that have one: bundling both into a single helper would
/// generalise exactly as far as the blanket comment that got `Fields` wrong in
/// the first place.
fn ensure_unique_ids<T>(
  items: &[T],
  id_of: impl Fn(&T) -> &str,
  duplicate: impl Fn(String, String) -> ProtocolError,
  at: &str,
) -> Result<(), ProtocolError> {
  let mut seen = BTreeSet::new();
  for item in items {
    let id = id_of(item);
    if !seen.insert(id) {
      return Err(duplicate(id.to_owned(), at.to_owned()));
    }
  }
  Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct Options(Vec<Opt>);

impl Options {
  /// # Errors
  ///
  /// `EmptyOptions` when `options` is empty — a choice with nothing to pick is
  /// unrenderable — and `DuplicateOptionId` when two options share an id, which
  /// would leave `UserResponse.option` unable to address one of the pair. Both
  /// name `at`, because a constructor called in isolation has no path context
  /// and the caller is the only thing that does.
  pub fn new(options: Vec<Opt>, at: &str) -> Result<Self, ProtocolError> {
    if options.is_empty() {
      return Err(ProtocolError::EmptyOptions { at: at.to_owned() });
    }
    ensure_unique_ids(
      &options,
      |option| option.id.as_str(),
      |id, at| ProtocolError::DuplicateOptionId { id, at },
      at,
    )?;
    Ok(Self(options))
  }

  pub fn as_slice(&self) -> &[Opt] {
    &self.0
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alternatives(Vec<Alternative>);

impl Alternatives {
  /// # Errors
  ///
  /// `EmptyAlternatives` when `alternatives` is empty, and
  /// `DuplicateAlternativeId` when two share an id — a duplicate does not
  /// collide a key here but makes the *submitted* value ambiguous, which is the
  /// same defect arriving from the other side. Never the `Options` errors: after
  /// F-61 an alternative id is not an option id, and an error saying otherwise
  /// asserts something the path never establishes.
  pub fn new(alternatives: Vec<Alternative>, at: &str) -> Result<Self, ProtocolError> {
    if alternatives.is_empty() {
      return Err(ProtocolError::EmptyAlternatives { at: at.to_owned() });
    }
    ensure_unique_ids(
      &alternatives,
      |alternative| alternative.id.as_str(),
      |id, at| ProtocolError::DuplicateAlternativeId { id, at },
      at,
    )?;
    Ok(Self(alternatives))
  }

  pub fn as_slice(&self) -> &[Alternative] {
    &self.0
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Fields(Vec<Field>);

impl Fields {
  /// # Errors
  ///
  /// `DuplicateFieldId` when two fields share an id, naming `at`:
  /// `UserResponse.values` is keyed by field id, so two such fields have one
  /// response key between them and cannot be answered independently (F-52).
  ///
  /// Empty is **not** an error — see the note above this block.
  pub fn new(fields: Vec<Field>, at: &str) -> Result<Self, ProtocolError> {
    ensure_unique_ids(
      &fields,
      |field| field.id.as_str(),
      |id, at| ProtocolError::DuplicateFieldId { id, at },
      at,
    )?;
    Ok(Self(fields))
  }

  pub fn as_slice(&self) -> &[Field] {
    &self.0
  }
}

/// Checked: each bound finite, and `min <= max` when both are present.
///
/// Bounds are semantics under brief §3.4 — they constrain which answers are
/// valid — so a range the host cannot interpret costs the sender the message
/// rather than being discarded (P2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberRange {
  min: Option<f64>,
  max: Option<f64>,
}

impl NumberRange {
  /// # Errors
  ///
  /// `NotFinite` naming the offending bound, and `Inverted` when `min > max`.
  ///
  /// `NotFinite` is unreachable from the wire — JSON expresses neither `NaN`
  /// nor infinity, and both fail in `serde_json` before any bounds check runs.
  /// It is checked anyway because this constructor is public API and the claim
  /// is about what the type can hold, not about who supplied it (D39, F-36).
  pub fn new(min: Option<f64>, max: Option<f64>) -> Result<Self, BoundsError> {
    if let Some(bound) = min
      && !bound.is_finite()
    {
      return Err(BoundsError::NotFinite {
        bound: "min",
        found: bound,
      });
    }
    if let Some(bound) = max
      && !bound.is_finite()
    {
      return Err(BoundsError::NotFinite {
        bound: "max",
        found: bound,
      });
    }
    if let (Some(low), Some(high)) = (min, max)
      && low > high
    {
      return Err(BoundsError::Inverted {
        min: low,
        max: high,
      });
    }
    Ok(Self { min, max })
  }

  pub fn min(self) -> Option<f64> {
    self.min
  }

  pub fn max(self) -> Option<f64> {
    self.max
  }
}

// ---------------------------------------------------------------------------
// Outbound: requests
// ---------------------------------------------------------------------------
//
// Host-authored, so the fields are public and nothing here is checked: the
// wire/canonical duality is inbound only (D5). This is the one place VA-2
// expects to find `pub` fields.

#[derive(Debug, Clone, PartialEq)]
pub enum Request {
  Evaluate(Evaluate),
  Respond(Respond),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Evaluate {
  pub now: Timestamp,
  pub event: Event,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Respond {
  pub view_id: ViewId,
  pub now: Timestamp,
  pub response: UserResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Event {
  pub source: String,
  pub kind: String,
  pub timestamp: Timestamp,
  /// Opaque to the host — brief §7, R-9.
  pub data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserResponse {
  pub option: OptionId,
  /// Opaque to the host — R-9.
  pub values: BTreeMap<FieldId, serde_json::Value>,
}

/// The version field and the discriminant, in one shape.
///
/// `protocol` is written on every request (R-1) and is not part of either
/// payload, so it cannot live on `Evaluate` or `Respond`; flattening an
/// internally-tagged enum beside it is what produces
/// `{"protocol": 1, "type": "evaluate", …}` with the payload's own keys at the
/// top level, which is the form `draft-spec.md:232` gives.
#[derive(Serialize)]
struct Envelope<'a> {
  protocol: u32,
  #[serde(flatten)]
  body: Body<'a>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum Body<'a> {
  Evaluate {
    now: Timestamp,
    event: &'a Event,
  },
  Respond {
    view_id: &'a ViewId,
    now: Timestamp,
    response: &'a UserResponse,
  },
}

/// R-1: every request carries `"protocol": 1`. R-6: every request carries a
/// `"type"` of `evaluate` or `respond`.
const PROTOCOL_VERSION: u32 = 1;

impl Serialize for Request {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let body = match self {
      Self::Evaluate(evaluate) => Body::Evaluate {
        now: evaluate.now,
        event: &evaluate.event,
      },
      Self::Respond(respond) => Body::Respond {
        view_id: &respond.view_id,
        now: respond.now,
        response: &respond.response,
      },
    };
    Envelope {
      protocol: PROTOCOL_VERSION,
      body,
    }
    .serialize(serializer)
  }
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;

  use super::{
    Alternative, AlternativeId, Alternatives, Evaluate, Event, Field, FieldId, FieldKind, Fields,
    Hints, NumberRange, Opt, OptionId, Options, Request, Respond, Timestamp, UserResponse, ViewId,
  };
  use crate::semantics::error::{BoundsError, ProtocolError};

  const AT: &str = "view.options";

  fn opt(id: &str, fields: Fields) -> Opt {
    Opt {
      id: OptionId::new(id),
      label: id.to_owned(),
      fields,
    }
  }

  fn field(id: &str) -> Field {
    Field {
      id: FieldId::new(id),
      kind: FieldKind::Text,
      label: id.to_owned(),
      hints: Hints::new(BTreeMap::new()),
    }
  }

  fn alternative(id: &str) -> Alternative {
    Alternative {
      id: AlternativeId::new(id),
      label: id.to_owned(),
    }
  }

  fn instant(raw: &str) -> Timestamp {
    Timestamp::new(raw.parse::<jiff::Timestamp>().unwrap())
  }

  /// Bit equality rather than `==`: exact, and it does not trip `float_cmp`.
  fn same(left: f64, right: f64) -> bool {
    left.to_bits() == right.to_bits()
  }

  // -- VT-1: one case per rejection ----------------------------------------

  #[test]
  fn an_empty_options_is_rejected_and_names_where() {
    let error = Options::new(Vec::new(), AT).unwrap_err();
    assert!(
      matches!(&error, ProtocolError::EmptyOptions { at } if at.as_str() == AT),
      "{error}"
    );
  }

  #[test]
  fn duplicate_option_ids_are_rejected_naming_the_id_and_where() {
    let options = vec![opt("yes", Fields::default()), opt("yes", Fields::default())];
    let error = Options::new(options, AT).unwrap_err();
    assert!(
      matches!(&error, ProtocolError::DuplicateOptionId { id, at }
        if id == "yes" && at.as_str() == AT),
      "{error}"
    );
  }

  #[test]
  fn duplicate_field_ids_within_one_option_are_rejected() {
    let error = Fields::new(vec![field("minutes"), field("minutes")], AT).unwrap_err();
    assert!(
      matches!(&error, ProtocolError::DuplicateFieldId { id, at }
        if id == "minutes" && at.as_str() == AT),
      "{error}"
    );
  }

  #[test]
  fn an_empty_alternatives_is_rejected_as_alternatives_never_as_options() {
    let error = Alternatives::new(Vec::new(), AT).unwrap_err();
    assert!(
      matches!(&error, ProtocolError::EmptyAlternatives { at } if at.as_str() == AT),
      "{error}"
    );
  }

  #[test]
  fn duplicate_alternative_ids_are_rejected_as_alternatives_never_as_options() {
    let error =
      Alternatives::new(vec![alternative("later"), alternative("later")], AT).unwrap_err();
    assert!(
      matches!(&error, ProtocolError::DuplicateAlternativeId { id, at }
        if id == "later" && at.as_str() == AT),
      "{error}"
    );
  }

  #[test]
  fn an_inverted_number_range_is_rejected() {
    let error = NumberRange::new(Some(10.0), Some(1.0)).unwrap_err();
    assert!(
      matches!(&error, BoundsError::Inverted { min, max } if same(*min, 10.0) && same(*max, 1.0)),
      "{error}"
    );
  }

  #[test]
  fn a_non_finite_bound_is_rejected_naming_which_bound() {
    let error = NumberRange::new(Some(f64::NAN), None).unwrap_err();
    assert!(
      matches!(&error, BoundsError::NotFinite { bound, found } if *bound == "min" && found.is_nan()),
      "{error}"
    );
    let upper = NumberRange::new(None, Some(f64::INFINITY)).unwrap_err();
    assert!(
      matches!(&upper, BoundsError::NotFinite { bound, .. } if *bound == "max"),
      "{upper}"
    );
  }

  #[test]
  fn a_range_with_one_bound_or_none_is_accepted() {
    // The guard is on inversion, not on absence: `min` alone and `max` alone
    // are both meaningful, and neither can invert.
    assert!(NumberRange::new(Some(5.0), None).is_ok());
    assert!(NumberRange::new(None, Some(120.0)).is_ok());
    assert!(NumberRange::new(None, None).is_ok());
    let range = NumberRange::new(Some(5.0), Some(120.0)).unwrap();
    assert!(range.min().is_some_and(|bound| same(bound, 5.0)));
    assert!(range.max().is_some_and(|bound| same(bound, 120.0)));
  }

  #[test]
  fn an_option_with_no_fields_is_accepted() {
    // R-15: an option MAY carry fields, and R-15's verification row asks for
    // fixtures with and without. There is no `EmptyFields` in the taxonomy and
    // §5.5's table has no row for it. Decided 2026-08-30, `plan-log.md`.
    let fields = Fields::new(Vec::new(), AT).unwrap();
    assert_eq!(fields.as_slice(), &[]);
    assert!(Options::new(vec![opt("yes", fields)], AT).is_ok());
  }

  // -- VT-3: the negative case ---------------------------------------------

  #[test]
  fn the_same_field_id_in_different_options_is_accepted() {
    // I15's scope is per-option, not per-view: a response names one option and
    // one flat value map, so two options may each carry a `minutes` field
    // (R-52, `draft-spec.md:360`). This is what shows the scope is right —
    // a per-view check would pass every rejection test above and still be wrong.
    let left = opt("a", Fields::new(vec![field("minutes")], AT).unwrap());
    let right = opt("b", Fields::new(vec![field("minutes")], AT).unwrap());
    let options = Options::new(vec![left, right], AT).unwrap();
    assert_eq!(options.as_slice().len(), 2);
  }

  // -- VT-2: request wire form ---------------------------------------------
  //
  // Asserted against the literal JSON at `draft-spec.md:232`, parsed to
  // `serde_json::Value` so key order is not asserted but a missing `protocol`
  // or `type` is. A round trip would pass with the version field absent, which
  // is the whole point of the criterion.

  fn json(raw: &str) -> serde_json::Value {
    serde_json::from_str(raw).unwrap()
  }

  #[test]
  fn an_evaluate_serializes_to_the_spec_s_wire_form() {
    let request = Request::Evaluate(Evaluate {
      now: instant("2026-08-23T04:12:00Z"),
      event: Event {
        source: "timer".to_owned(),
        kind: "scheduled".to_owned(),
        timestamp: instant("2026-08-23T04:12:00Z"),
        data: json("{}"),
      },
    });
    assert_eq!(
      serde_json::to_value(&request).unwrap(),
      json(
        r#"{ "protocol": 1, "type": "evaluate", "now": "2026-08-23T04:12:00Z",
              "event": { "source": "timer", "kind": "scheduled",
                         "timestamp": "2026-08-23T04:12:00Z", "data": {} } }"#
      )
    );
  }

  #[test]
  fn a_respond_serializes_to_the_spec_s_wire_form() {
    let mut values = BTreeMap::new();
    values.insert(FieldId::new("minutes"), json("20"));
    let request = Request::Respond(Respond {
      view_id: ViewId::new("2026-08-23T04:12:00Z#3"),
      now: instant("2026-08-23T04:14:31Z"),
      response: UserResponse {
        option: OptionId::new("later"),
        values,
      },
    });
    assert_eq!(
      serde_json::to_value(&request).unwrap(),
      json(
        r#"{ "protocol": 1, "type": "respond", "now": "2026-08-23T04:14:31Z",
              "view_id": "2026-08-23T04:12:00Z#3",
              "response": { "option": "later", "values": { "minutes": 20 } } }"#
      )
    );
  }

  #[test]
  fn every_request_kind_carries_the_version_and_a_discriminant() {
    // R-1 and R-6 over both kinds at once, so a third kind added without an
    // envelope fails here rather than at PHASE-04. The snapshots above would
    // each catch their own kind; this catches the rule.
    let kinds = [
      (
        Request::Evaluate(Evaluate {
          now: instant("2026-08-23T04:12:00Z"),
          event: Event {
            source: "timer".to_owned(),
            kind: "scheduled".to_owned(),
            timestamp: instant("2026-08-23T04:12:00Z"),
            data: json("{}"),
          },
        }),
        "evaluate",
      ),
      (
        Request::Respond(Respond {
          view_id: ViewId::new("2026-08-23T04:12:00Z#3"),
          now: instant("2026-08-23T04:14:31Z"),
          response: UserResponse {
            option: OptionId::new("later"),
            values: BTreeMap::new(),
          },
        }),
        "respond",
      ),
    ];
    for (request, discriminant) in kinds {
      let value = serde_json::to_value(&request).unwrap();
      assert_eq!(value.get("protocol"), Some(&json("1")), "{value}");
      assert_eq!(
        value.get("type").and_then(serde_json::Value::as_str),
        Some(discriminant)
      );
    }
  }
}

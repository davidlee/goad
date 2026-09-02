//! Wire to canonical, and the only path between them — `design.md` §5.2.
//!
//! Total in the sense that matters: a message either normalizes or names why
//! it did not, and the parts it can lose without losing the message come back
//! beside it rather than as failures (P2).
//!
//! `now` is a parameter. Stratum 1 reads no clock (I3), and every relative
//! `next_check` resolves against the argument.
//!
//! **The `at` path.** `design.md` §6 leaves accumulation to implementation and
//! fixes only the contract — the named error, the retained string, the path.
//! The grammar here is the dotted and bracketed form §5.2's own examples use,
//! `view.options[1].fields[2].kind`, because a second spelling would put the
//! corpus's asserted strings at odds with the design's prose. Paths name the
//! **wire** shape, so a `choice` field's alternatives are reached through
//! `options`, which is the key a backend author wrote.
//!
//! This module handles backend-derived data, so it carries the module-level
//! deny D53 leaves to such modules (I9, R-46).
#![deny(clippy::arithmetic_side_effects)]

use crate::semantics::error::{ProtocolError, ScheduleError};
use crate::semantics::protocol::canonical::{
  Alternative, AlternativeId, Alternatives, Choice, Content, Field, FieldId, FieldKind, Fields,
  Hints, NumberRange, Opt, OptionId, Options, Response, Timestamp, View,
};
use crate::semantics::protocol::wire::{
  WireAlternative, WireChoice, WireContent, WireContentValue, WireField, WireOpt, WireResponse,
  WireView,
};
use crate::semantics::schedule;

/// A canonical value, and the parts the message lost without losing the
/// message (P2).
///
/// `Debug` only. `Clone` and `PartialEq` would have to reach through
/// `Discarded` into `ScheduleError`, which derives neither, and widening the
/// taxonomy's derives to buy a convenience nothing needs is not this phase's
/// call — `semantics/error.rs` is not a surface here.
#[derive(Debug)]
pub struct Normalized<T> {
  pub value: T,
  pub discarded: Vec<Discarded>,
}

/// A closed enum of one variant, deliberately: P2's eligibility test is meant
/// to be applied on purpose, so a second variant is the moment someone has to
/// argue it (D10).
#[derive(Debug)]
pub enum Discarded {
  Schedule {
    raw: serde_json::Value,
    reason: ScheduleError,
  },
}

/// The protocol version this host implements. A response may omit it (R-2); it
/// may not declare another one (R-3).
const PROTOCOL_VERSION: u32 = 1;

/// Read a permissive wire response as a canonical one.
///
/// Three things happen, in this order, and the order is the contract: the
/// declared version is checked, the view is normalized, and `next_check` is
/// resolved against `now`. A scheduling failure is a discard rather than an
/// error, so it cannot short-circuit the view — the message survives losing its
/// schedule and does not survive losing its view.
///
/// # Errors
///
/// Every `ProtocolError` but `Schedule`, which by construction never arrives
/// here as an `Err`: an unusable `next_check` is a discard (P2, R-25).
///
/// `Json` does arrive, and not only from the caller that deserialized the
/// bytes. The wire types leave a view's payload, a content block and a `choice`
/// field's alternatives untyped so their discriminants can be dispatched before
/// anything beside them is bound, which means each of those is deserialized
/// *here* — and a shape serde refuses at that point is the typed shape error
/// `design.md` §5.2 calls for.
pub fn normalize_response(
  wire: WireResponse,
  now: Timestamp,
) -> Result<Normalized<Response>, ProtocolError> {
  if let Some(found) = wire.protocol
    && found != PROTOCOL_VERSION
  {
    return Err(ProtocolError::UnsupportedProtocolVersion { found });
  }

  // Omission and `null` are different claims here, and only here: `null`
  // asserts "there is nothing to show" while omission asserts nothing at all,
  // so the host may not manufacture the assertion on the backend's behalf
  // (D25, F-5, R-10, R-11).
  let view = match wire.view {
    None => return Err(ProtocolError::MissingField { field: "view" }),
    Some(None) => None,
    Some(Some(wire_view)) => Some(normalize_view(wire_view)?),
  };

  let mut discarded = Vec::new();
  // `WireResponse.next_check` is `Option<Value>`, and serde maps *both*
  // omission and an explicit `null` to `None` — measured, not assumed. That is
  // what makes D50's rule structural for this field rather than a check: an
  // explicit `null` never reaches `schedule::parse`, which would have called it
  // `NotAString` and reported a discard R-51 forbids.
  let schedule = match wire.next_check {
    None => None,
    Some(raw) => match schedule::parse(&raw, now) {
      Ok(instant) => Some(instant),
      Err(reason) => {
        discarded.push(Discarded::Schedule { raw, reason });
        None
      }
    },
  };

  Ok(Normalized {
    // A struct literal, not a constructor: `canonical`'s fields are
    // `pub(super)` and this module is inside that scope, which is exactly the
    // access D30 granted normalization. Adding a constructor there instead
    // would be R10, the named risk.
    value: Response { view, schedule },
    discarded,
  })
}

/// Dispatch on the view's own discriminant.
///
/// `WireView` binds `kind` and leaves the rest untyped precisely so that this
/// match happens before anything under it is read: a serde enum would collapse
/// an unrecognised kind into "data did not match any variant" and lose the
/// string worth reporting (D8, F-6, R-12).
fn normalize_view(wire: WireView) -> Result<View, ProtocolError> {
  match wire.kind.as_str() {
    "choice" => {
      let choice: WireChoice = serde_json::from_value(wire.rest).map_err(ProtocolError::Json)?;
      Ok(View::Choice(normalize_choice(choice)?))
    }
    _ => Err(ProtocolError::UnsupportedPrimitive {
      kind: wire.kind,
      at: "view.kind".to_owned(),
    }),
  }
}

fn normalize_choice(wire: WireChoice) -> Result<Choice, ProtocolError> {
  let body = match wire.body {
    None => None,
    Some(raw) => Some(normalize_content(&raw, "view.body")?),
  };

  // An absent `options` offers nothing to pick, which is the defect an empty
  // list has, so both arrive at `Options::new` and both are `EmptyOptions`
  // (R-13). The wire type makes it optional so that this decision is
  // normalization's rather than serde's.
  let at = "view.options";
  let options = each_indexed(wire.options.unwrap_or_default(), at, normalize_opt)?;

  Ok(Choice {
    title: wire.title,
    body,
    options: Options::new(options, at)?,
  })
}

/// Normalize a wire sequence, handing each element the path it sits at.
///
/// The three sequences the protocol has — a view's options, an option's fields,
/// a `choice` field's alternatives — each walk a list, index it into the path,
/// and hand the result to a checked constructor. One walk rather than three
/// keeps the path grammar in a single place: `{at}[{index}]` is written once and
/// so cannot drift between the three, which is exactly the drift the corpus
/// asserts against literally.
fn each_indexed<W, C>(
  items: Vec<W>,
  at: &str,
  each: impl Fn(W, &str) -> Result<C, ProtocolError>,
) -> Result<Vec<C>, ProtocolError> {
  let mut normalized = Vec::with_capacity(items.len());
  for (index, item) in items.into_iter().enumerate() {
    normalized.push(each(item, &format!("{at}[{index}]"))?);
  }
  Ok(normalized)
}

/// A bare string is text (brief §10.1); anything else names its own kind.
fn normalize_content(raw: &serde_json::Value, at: &str) -> Result<Content, ProtocolError> {
  if let Some(text) = raw.as_str() {
    return Ok(Content::Text(text.to_owned()));
  }
  let tag: WireContent = serde_json::from_value(raw.clone()).map_err(ProtocolError::Json)?;
  let payload = || -> Result<String, ProtocolError> {
    serde_json::from_value::<WireContentValue>(raw.clone())
      .map(|content| content.value)
      .map_err(ProtocolError::Json)
  };
  match tag.kind.as_str() {
    "text" => Ok(Content::Text(payload()?)),
    "markdown" => Ok(Content::Markdown(payload()?)),
    "html" => Ok(Content::Html(payload()?)),
    // Carried, never dereferenced — R-19 and I7's rule about what the host
    // reads versus what it merely holds.
    "uri" => Ok(Content::Uri(payload()?)),
    _ => Err(ProtocolError::UnsupportedPrimitive {
      kind: tag.kind,
      at: format!("{at}.kind"),
    }),
  }
}

fn normalize_opt(wire: WireOpt, at: &str) -> Result<Opt, ProtocolError> {
  let at = format!("{at}.fields");
  let fields = each_indexed(wire.fields.unwrap_or_default(), &at, normalize_field)?;
  Ok(Opt {
    id: OptionId::new(wire.id),
    label: wire.label,
    // Empty is legal here and only here: R-15 says an option *may* carry
    // fields, so `Fields::new` checks uniqueness and nothing else.
    fields: Fields::new(fields, &at)?,
  })
}

fn normalize_field(wire: WireField, at: &str) -> Result<Field, ProtocolError> {
  let WireField {
    id,
    kind,
    label,
    min,
    max,
    options,
    hints,
  } = wire;
  Ok(Field {
    kind: normalize_field_kind(&kind, min, max, options, at)?,
    id: FieldId::new(id),
    label,
    // Everything else the field object carried. Opaque: nothing in `semantics/`
    // or `shell/` may branch on a key here (I7, R-18).
    hints: Hints::new(hints),
  })
}

/// A modelled key used where its kind gives it no meaning.
///
/// Not a hint and not ignored: serde binds `min`, `max` and `options` before
/// `kind` is dispatched — measured, not assumed — so such a key cannot fall
/// through to `hints`, and the only alternative to reporting it is losing a
/// value the sender meant (F-45, D43, R-50).
fn inapplicable(key: &'static str, kind: &str, at: &str) -> ProtocolError {
  ProtocolError::InapplicableKey {
    key,
    kind: kind.to_owned(),
    at: at.to_owned(),
  }
}

/// The three kinds that admit no modelled key beyond `id`, `kind` and `label`.
///
/// Order is fixed so that a field carrying several inapplicable keys reports
/// the same one every run.
fn reject_every_extra_key(
  kind: &str,
  at: &str,
  min: Option<f64>,
  max: Option<f64>,
  options: Option<&serde_json::Value>,
) -> Result<(), ProtocolError> {
  if min.is_some() {
    return Err(inapplicable("min", kind, at));
  }
  if max.is_some() {
    return Err(inapplicable("max", kind, at));
  }
  if options.is_some() {
    return Err(inapplicable("options", kind, at));
  }
  Ok(())
}

/// R-16's five kinds, each admitting only the keys it has a meaning for.
///
/// Applicability is judged *inside* each arm rather than before the match, so an
/// unrecognised kind reports itself rather than one of its keys: a key
/// misplaced on a kind the host does not implement is the kind's failure, and
/// naming the key would send a backend author to fix the wrong thing.
fn normalize_field_kind(
  kind: &str,
  min: Option<f64>,
  max: Option<f64>,
  options: Option<serde_json::Value>,
  at: &str,
) -> Result<FieldKind, ProtocolError> {
  match kind {
    "text" => {
      reject_every_extra_key(kind, at, min, max, options.as_ref())?;
      Ok(FieldKind::Text)
    }
    "boolean" => {
      reject_every_extra_key(kind, at, min, max, options.as_ref())?;
      Ok(FieldKind::Boolean)
    }
    "datetime" => {
      reject_every_extra_key(kind, at, min, max, options.as_ref())?;
      Ok(FieldKind::DateTime)
    }
    // Bounds are semantics: they constrain which answers are valid, so an
    // uninterpretable range costs the sender the message rather than being
    // discarded (R-17, P2).
    "number" => {
      if options.is_some() {
        return Err(inapplicable("options", kind, at));
      }
      NumberRange::new(min, max)
        .map(FieldKind::Number)
        .map_err(ProtocolError::Bounds)
    }
    "choice" => {
      if min.is_some() {
        return Err(inapplicable("min", kind, at));
      }
      if max.is_some() {
        return Err(inapplicable("max", kind, at));
      }
      Ok(FieldKind::Choice {
        alternatives: normalize_alternatives(options, at)?,
      })
    }
    _ => Err(ProtocolError::UnsupportedPrimitive {
      kind: kind.to_owned(),
      at: format!("{at}.kind"),
    }),
  }
}

/// A `choice` field's alternatives — values the answer submits, not actions it
/// selects (F-61, D52, R-53).
///
/// The path says `options`, because that is the key a backend author wrote. The
/// *type* is not `Options` and the errors are not the option ones: an error
/// naming a duplicate option id here would assert that the id is an option id,
/// which nothing on this path establishes.
fn normalize_alternatives(
  options: Option<serde_json::Value>,
  at: &str,
) -> Result<Alternatives, ProtocolError> {
  let at = format!("{at}.options");
  // Absent is the same offer as empty — nothing to answer with — and reaches
  // the same `EmptyAlternatives`, as an absent `options` on a view does.
  let raw = options.unwrap_or_else(|| serde_json::Value::Array(Vec::new()));
  let items: Vec<serde_json::Value> = serde_json::from_value(raw).map_err(ProtocolError::Json)?;
  let alternatives = each_indexed(items, &at, normalize_alternative)?;
  Alternatives::new(alternatives, &at)
}

fn normalize_alternative(raw: serde_json::Value, at: &str) -> Result<Alternative, ProtocolError> {
  // `fields` is a protocol key wherever the protocol admits it, and this is not
  // one of those places — so it is rejected rather than ignored as unmodelled
  // (F-55, R-53). An explicit `null` is exempt for the reason every other
  // `null` is: it asserts nothing, so nothing is lost by reading it as
  // omission (D50, R-51).
  if raw.get("fields").is_some_and(|nested| !nested.is_null()) {
    return Err(inapplicable("fields", "choice", at));
  }
  let wire: WireAlternative = serde_json::from_value(raw).map_err(ProtocolError::Json)?;
  Ok(Alternative {
    id: AlternativeId::new(wire.id),
    label: wire.label,
  })
}

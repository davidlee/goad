//! Inbound wire types — `design.md` §5.2.
//!
//! Permissive by construction, and only inbound: requests are host-authored, so
//! they have one type and it is canonical (D5). Three rules shape everything
//! here.
//!
//! **No `deny_unknown_fields`, anywhere** (I10, R-4, R-5). A key this module
//! does not name is a backend written against a newer host, and rejecting it
//! would make every additive protocol change breaking.
//!
//! **`view` distinguishes omission from `null`, and nothing else does** (D25,
//! D50, F-5, F-50). `null` there is a positive assertion — "there is nothing to
//! show" — while omission asserts nothing at all, so the field needs a
//! presence-preserving deserializer. For every other modelled field an explicit
//! `null` means exactly what omission means, which serde already gives us and
//! which R-51 requires produce no discard.
//!
//! **A value the protocol may reject in its own words stays untyped here.**
//! `next_check`, `body` and a `choice` field's `options` are
//! `serde_json::Value` so that normalization can dispatch them and name the
//! failure precisely (`NotAString`, `UnsupportedPrimitive`, `InapplicableKey`).
//! A tighter wire type would collapse each of those into one serde error and
//! take the whole message with it, which P2 forbids for `next_check` and F-6
//! forbids for the rest.
//!
//! Nothing here inspects a value or does arithmetic — deserialization only —
//! but it is the crate's front door for backend-derived data and the module
//! deny costs nothing, so it carries one (D53 as amended, I9).
#![deny(clippy::arithmetic_side_effects)]

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer};

/// The inbound envelope.
///
/// `protocol` is optional because brief §8.2's own examples omit it, and
/// requiring it would reject every backend written against the brief (R-2). A
/// version the host does not implement is still refused, by normalization
/// rather than by serde, so the error can name what it found (R-3).
#[derive(Debug, Deserialize)]
pub struct WireResponse {
  #[serde(default)]
  pub protocol: Option<u32>,

  /// Outer `Option`: was the field present at all. Inner: was it `null`.
  /// `None` => omitted, `Some(None)` => explicit null, `Some(Some(v))` => a
  /// view.
  #[expect(
    clippy::option_option,
    reason = "the three states are the requirement, not an accident of nesting: `null` asserts               there is nothing to show and omission asserts nothing at all, and collapsing them               would have the host manufacture the backend's assertion (D25, F-5, R-10, R-11).               The lint's own suggested alternative is a custom enum; `design.md` §5.2 fixes this               shape and names the `present` helper, and EX-1 requires both, so replacing them               would be a design change made to satisfy a style lint. Written here rather than               scoped away, which is what §9's reason-carrying exception exists for."
  )]
  #[serde(default, deserialize_with = "present")]
  pub view: Option<Option<WireView>>,

  /// Untyped, so `"next_check": 45` is a reportable discard rather than a serde
  /// failure that costs the message (D6, P2, R-25).
  #[serde(default)]
  pub next_check: Option<serde_json::Value>,
}

/// serde maps both an absent field and an explicit `null` to `None`, so the
/// outer layer has to be supplied rather than inferred from nesting.
///
/// # Errors
///
/// Whatever `T`'s own `Deserialize` reports. This wrapper adds no failure of
/// its own; it only refuses to collapse two answers into one.
fn present<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
  T: Deserialize<'de>,
  D: Deserializer<'de>,
{
  T::deserialize(deserializer).map(Some)
}

/// A view, read as its discriminant and everything else.
///
/// The `kind` is bound on its own and `rest` is left untyped so that
/// normalization dispatches rather than serde: an unrecognised view kind must
/// be `UnsupportedPrimitive { kind, at }` naming the string it found, and a
/// serde enum would give a generic "did not match any variant" instead (D8,
/// F-6, R-12).
///
/// `design.md` §6 offers this encoding as implementation latitude and it is
/// taken as offered. That `#[serde(flatten)]` here does not disturb the
/// presence-preserving deserializer on `view` above it was measured before this
/// type was written, not assumed (`notes.md` PHASE-04, A2).
#[derive(Debug, Deserialize)]
pub struct WireView {
  pub kind: String,
  #[serde(flatten)]
  pub rest: serde_json::Value,
}

/// The one view kind v0 admits (R-13).
#[derive(Debug, Deserialize)]
pub struct WireChoice {
  pub title: String,
  /// A bare string or a tagged object, left untyped and dispatched in
  /// normalization so an unrecognised content kind keeps its own named error
  /// (D38, R-19). Brief §10.1's own example is the bare string.
  #[serde(default)]
  pub body: Option<serde_json::Value>,
  #[serde(default)]
  pub options: Option<Vec<WireOpt>>,
}

/// A **view's** option — what a response selects (R-8). The only wire type
/// carrying `fields`: an alternative is a value rather than an action, so it
/// has none (F-54, R-53).
#[derive(Debug, Deserialize)]
pub struct WireOpt {
  pub id: String,
  pub label: String,
  #[serde(default)]
  pub fields: Option<Vec<WireField>>,
}

/// One field on an option.
///
/// `min`, `max` and `options` are declared for every kind because one struct
/// deserializes all five and the discriminant is only read afterwards. Serde
/// binds them **before** `kind` is dispatched — measured, not assumed
/// (`notes.md` PHASE-04, A3) — so a `min` on a text field cannot fall through
/// to `hints`, and normalization must reject it as `InapplicableKey` or lose it
/// silently. That is F-45, and it is the price of the flatten below rather than
/// an accident of it.
///
/// `hints` is flat, not nested, because brief §10.2's own example writes
/// `multiline` alongside `id` and `kind`. Everything else on the object is a
/// hint, which is also the honest reading of "likely presentation hints over
/// time" (D37, F-38, R-18). The stated cost is that a misspelled **optional**
/// key becomes a hint; a misspelled **required** one still fails, because a
/// declared field stays required after flattening (A4).
#[derive(Debug, Deserialize)]
pub struct WireField {
  pub id: String,
  pub kind: String,
  pub label: String,
  #[serde(default)]
  pub min: Option<f64>,
  #[serde(default)]
  pub max: Option<f64>,
  /// A `choice` field's alternatives. Untyped for R-53: the dispatch has to
  /// look for a `fields` key and reject it, which serde binding a typed vector
  /// would have swallowed.
  #[serde(default)]
  pub options: Option<serde_json::Value>,
  #[serde(flatten)]
  pub hints: BTreeMap<String, serde_json::Value>,
}

/// A value a `choice` field may take (F-61, R-53).
///
/// Id and label only. `fields` is deliberately **not** declared: it is a
/// protocol key wherever the protocol admits it, and this is not one of those
/// places, so normalization checks the raw object for it and raises
/// `InapplicableKey` (F-55). Declaring it here would have made the rejection
/// look like a field this type carries.
#[derive(Debug, Deserialize)]
pub struct WireAlternative {
  pub id: String,
  pub label: String,
}

/// The discriminant half of a tagged content block: `{"kind": …, "value": …}`
/// (R-19).
///
/// Split from its payload for the reason `WireView` is split from `WireChoice`:
/// an unrecognised content kind must be `UnsupportedPrimitive` naming the
/// string it found, which is only possible if `kind` is read before anything
/// beside it is bound. Reading the same object twice is what buys that, and it
/// costs one map lookup.
#[derive(Debug, Deserialize)]
pub struct WireContent {
  pub kind: String,
}

/// The payload half, read only once `kind` has been dispatched.
///
/// A well-named kind whose `value` is missing or is not a string is the typed
/// shape error `design.md` §5.2 calls for — serde's own message, reaching the
/// caller as `ProtocolError::Json`.
#[derive(Debug, Deserialize)]
pub struct WireContentValue {
  pub value: String,
}

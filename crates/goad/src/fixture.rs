//! Canonical values, built the only way this crate can build them: by
//! normalizing a wire document.
//!
//! `OptionId::new`, `FieldId::new` and `AlternativeId::new` are all
//! `pub(super)` in `goad-semantics`, and `Alternative`'s two fields are
//! private with no public constructor, so stratum 3 can clone an id or an
//! `Alternatives` off a view it drew and can mint neither (§5.5 I-D). That is
//! the property the host wants, and this module is what it costs: a test that
//! needs a canonical value parses one.
//!
//! `#[cfg(test)]` at the module tree, so none of it reaches the binary.

use goad_semantics::protocol::canonical::{
  AlternativeId, Alternatives, FieldId, FieldKind, Opt, OptionId, Timestamp, View,
};
use goad_semantics::protocol::normalize::read_response;

/// One option carrying the fields the caller declares in wire form, put
/// through `read_response` and handed back.
pub(crate) fn option_declaring(option: &str, fields: &serde_json::Value) -> Opt {
  let document = serde_json::json!({
    "view": {
      "kind": "choice",
      "title": "T",
      "options": [{ "id": option, "label": "L", "fields": fields }]
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

/// A one-option, one-`boolean`-field view, and its two ids.
pub(crate) fn ids(option: &str, field: &str) -> (OptionId, FieldId) {
  let declared = option_declaring(
    option,
    &serde_json::json!([{ "id": field, "kind": "boolean", "label": "F" }]),
  );
  let first = declared
    .fields()
    .as_slice()
    .first()
    .expect("its one field")
    .clone();
  (declared.id().clone(), first.id().clone())
}

/// The `Alternatives` of a `choice` field declaring one alternative per id,
/// each labelled with its id upper-cased so a test can tell the two halves
/// apart.
pub(crate) fn alternatives(ids: &[&str]) -> Alternatives {
  let declared = option_declaring(
    "opt",
    &serde_json::json!([{
      "id": "pick",
      "kind": "choice",
      "label": "F",
      "options": ids
        .iter()
        .map(|id| serde_json::json!({ "id": id, "label": id.to_uppercase() }))
        .collect::<Vec<_>>(),
    }]),
  );
  let kind = declared
    .fields()
    .as_slice()
    .first()
    .expect("its one field")
    .kind()
    .clone();
  match kind {
    FieldKind::Choice { alternatives } => alternatives,
    other => panic!("the fixture declares a choice, not {other:?}"),
  }
}

/// The id of one of those alternatives, by position.
pub(crate) fn alternative_id(ids: &[&str], at: usize) -> AlternativeId {
  alternatives(ids)
    .as_slice()
    .get(at)
    .expect("the fixture declares that alternative")
    .id()
    .clone()
}

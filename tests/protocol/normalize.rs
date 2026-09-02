//! PHASE-04's two corpora: `normalize_response` read as protocol documentation.
//!
//! The seam is PHASE-03's, unchanged — `Fixture`, `Corpus`, `outcome_tag` and
//! `assert_corpus` come from `runner.rs`, and this file adds only the reading of
//! its own payload. That the shared half needed no edit to admit a corpus whose
//! input is a whole wire response is the property it was split for.
//!
//! **Two corpora, because one of them holds text `serde_json` will not parse.**
//! `fixtures/protocol/` is the corpus proper: `input` is the wire response as a
//! JSON value, exactly as a backend would emit it, and it is the one that has to
//! read as documentation (AC-9). `fixtures/protocol-text/` carries the two
//! literals JSON cannot hold — `NaN` and `1e400` — as raw document text in a
//! JSON string, because such a file would otherwise fail at *envelope* parse and
//! never assert its protocol claim at all. The escaping that makes that form
//! unreadable is exactly why it is not the default (`notes.md` PHASE-04, item 4).
//!
//! **An accepting fixture states the whole canonical value, not a probe into
//! it.** `expect.accepted.canonical` is the entire normalized response rendered
//! back to JSON, so a fixture says what a wire document *means* rather than what
//! one of its parts does — which is what makes the corpus usable to
//! `draft-spec.md` §7. It also discharges R-4/R-5 without a second mechanism: an
//! unmodelled field that survived normalization would appear in the rendering
//! and break the case.
//!
//! **`expect.accepted.discarded` is required, never optional.** For most cases
//! the assertion *is* the empty list: R-51 turns on `null` producing no discard,
//! and a corpus where silence is the default would assert it nowhere.

use serde_json::{Value, json};

use goad::semantics::error::{BoundsError, ProtocolError, ScheduleError};
use goad::semantics::protocol::canonical::{
  Alternative, Choice, Content, Field, FieldKind, NumberRange, Opt, Response, View,
};
use goad::semantics::protocol::normalize::{Discarded, Normalized, normalize_response};
use goad::semantics::protocol::wire::WireResponse;

use crate::runner::{Corpus, Fixture, assert_corpus, outcome_tag};

// ---------------------------------------------------------------------------
// The canonical value, rendered back to JSON
// ---------------------------------------------------------------------------
//
// Externally tagged throughout — `{"markdown": "…"}`, `{"number": {…}}` — the
// same discipline `expect` itself uses, so a reader meets one convention rather
// than three. Every match below is exhaustive on purpose: a variant added to
// `canonical.rs` cannot reach the corpus without an arm here.

fn render_response(response: &Response) -> Value {
  json!({
    "view": response.view().map_or(Value::Null, render_view),
    "schedule": response
      .schedule()
      .map_or(Value::Null, |instant| json!(instant.instant().to_string())),
  })
}

fn render_view(view: &View) -> Value {
  match view {
    View::Choice(choice) => json!({ "choice": render_choice(choice) }),
  }
}

fn render_choice(choice: &Choice) -> Value {
  json!({
    "title": choice.title(),
    "body": choice.body().map_or(Value::Null, render_content),
    "options": choice.options().as_slice().iter().map(render_opt).collect::<Vec<_>>(),
  })
}

fn render_content(content: &Content) -> Value {
  match content {
    Content::Text(text) => json!({ "text": text }),
    Content::Markdown(text) => json!({ "markdown": text }),
    Content::Html(text) => json!({ "html": text }),
    Content::Uri(text) => json!({ "uri": text }),
  }
}

fn render_opt(option: &Opt) -> Value {
  json!({
    "id": option.id().as_str(),
    "label": option.label(),
    "fields": option.fields().as_slice().iter().map(render_field).collect::<Vec<_>>(),
  })
}

fn render_field(field: &Field) -> Value {
  json!({
    "id": field.id().as_str(),
    "kind": render_field_kind(field.kind()),
    "label": field.label(),
    "hints": field.hints().as_map(),
  })
}

fn render_field_kind(kind: &FieldKind) -> Value {
  match kind {
    FieldKind::Text => json!("text"),
    FieldKind::Boolean => json!("boolean"),
    FieldKind::DateTime => json!("datetime"),
    FieldKind::Number(range) => json!({ "number": render_range(*range) }),
    FieldKind::Choice { alternatives } => json!({
      "choice": alternatives.as_slice().iter().map(render_alternative).collect::<Vec<_>>(),
    }),
  }
}

fn render_range(range: NumberRange) -> Value {
  json!({ "min": range.min(), "max": range.max() })
}

fn render_alternative(alternative: &Alternative) -> Value {
  json!({ "id": alternative.id().as_str(), "label": alternative.label() })
}

/// P2's discard list. `raw` is carried verbatim so a fixture shows the value the
/// backend actually sent, and `reason` names the variant.
fn render_discarded(discarded: &[Discarded]) -> Value {
  Value::Array(
    discarded
      .iter()
      .map(|item| match item {
        Discarded::Schedule { raw, reason } => json!({
          "schedule": { "raw": raw, "reason": schedule_error_name(reason) },
        }),
      })
      .collect(),
  )
}

// ---------------------------------------------------------------------------
// The error taxonomy, rendered back to JSON
// ---------------------------------------------------------------------------

/// Exhaustive by design: a thirteenth `ProtocolError` cannot be added without an
/// arm here, which is what stops a variant entering the taxonomy with no fixture
/// able to name it (VT-2).
///
/// `Json` renders as a bare tag. The design's claim about a malformed document
/// is that it is a `Json` error and nothing more — `serde_json`'s message is not
/// a contract, and a fixture asserting one would break on a dependency bump. The
/// message still reaches a failing run, through `mismatch` below.
fn render_error(error: &ProtocolError) -> Value {
  match error {
    ProtocolError::Json(_) => json!({ "Json": Value::Null }),
    ProtocolError::UnsupportedProtocolVersion { found } => {
      json!({ "UnsupportedProtocolVersion": { "found": found } })
    }
    ProtocolError::UnsupportedPrimitive { kind, at } => {
      json!({ "UnsupportedPrimitive": { "kind": kind, "at": at } })
    }
    ProtocolError::InapplicableKey { key, kind, at } => {
      json!({ "InapplicableKey": { "key": key, "kind": kind, "at": at } })
    }
    ProtocolError::MissingField { field } => json!({ "MissingField": { "field": field } }),
    ProtocolError::EmptyOptions { at } => json!({ "EmptyOptions": { "at": at } }),
    ProtocolError::DuplicateOptionId { id, at } => {
      json!({ "DuplicateOptionId": { "id": id, "at": at } })
    }
    ProtocolError::DuplicateFieldId { id, at } => {
      json!({ "DuplicateFieldId": { "id": id, "at": at } })
    }
    ProtocolError::DuplicateAlternativeId { id, at } => {
      json!({ "DuplicateAlternativeId": { "id": id, "at": at } })
    }
    ProtocolError::EmptyAlternatives { at } => json!({ "EmptyAlternatives": { "at": at } }),
    ProtocolError::Bounds(bounds) => json!({ "Bounds": render_bounds(bounds) }),
    ProtocolError::Schedule(schedule) => json!({ "Schedule": schedule_error_name(schedule) }),
  }
}

fn render_bounds(bounds: &BoundsError) -> Value {
  match bounds {
    BoundsError::NotFinite { bound, found } => {
      json!({ "NotFinite": { "bound": bound, "found": found } })
    }
    BoundsError::Inverted { min, max } => json!({ "Inverted": { "min": min, "max": max } }),
  }
}

fn schedule_error_name(error: &ScheduleError) -> &'static str {
  match error {
    ScheduleError::NotAString { .. } => "NotAString",
    ScheduleError::MissingOffset { .. } => "MissingOffset",
    ScheduleError::CalendarUnit { .. } => "CalendarUnit",
    ScheduleError::OutOfRange { .. } => "OutOfRange",
    ScheduleError::Unparseable { .. } => "Unparseable",
  }
}

// ---------------------------------------------------------------------------
// The two checkers
// ---------------------------------------------------------------------------

/// What the host does with a backend's bytes: deserialize, then normalize.
///
/// The serde failure becomes `ProtocolError::Json` here rather than inside
/// `normalize_response`, because that is where it happens in the host too — the
/// composition is stratum 2's (`design.md` §5.2, *Host*), and a corpus that
/// wrapped it differently would be asserting against a path nothing runs.
fn outcome(
  wire: Result<WireResponse, serde_json::Error>,
  fixture: &Fixture<'_>,
) -> Result<Normalized<Response>, ProtocolError> {
  normalize_response(wire.map_err(ProtocolError::Json)?, fixture.now)
}

fn render_normalized(normalized: &Normalized<Response>) -> Value {
  json!({
    "canonical": render_response(&normalized.value),
    "discarded": render_discarded(&normalized.discarded),
  })
}

fn mismatch(expected: &Value, actual: &Value, note: &str) -> String {
  let expected = serde_json::to_string_pretty(expected).unwrap_or_else(|_| "?".to_owned());
  let actual = serde_json::to_string_pretty(actual).unwrap_or_else(|_| "?".to_owned());
  format!("expected {expected}\n    got {actual}{note}")
}

/// Both corpora read the same two outcome tags; only the route into
/// `WireResponse` differs.
fn compare(
  outcome: Result<Normalized<Response>, ProtocolError>,
  expect: &Value,
) -> Result<(), String> {
  match outcome_tag(expect)? {
    ("accepted", expected) => match outcome {
      Ok(normalized) => {
        let actual = render_normalized(&normalized);
        if actual == *expected {
          Ok(())
        } else {
          Err(mismatch(expected, &actual, ""))
        }
      }
      Err(error) => Err(mismatch(
        expected,
        &render_error(&error),
        &format!("\n    the message was rejected: {error}"),
      )),
    },
    ("error", expected) => match outcome {
      Err(error) => {
        let actual = render_error(&error);
        if actual == *expected {
          Ok(())
        } else {
          Err(mismatch(expected, &actual, &format!("\n    ({error})")))
        }
      }
      Ok(normalized) => Err(mismatch(
        expected,
        &render_normalized(&normalized),
        "\n    the message was accepted",
      )),
    },
    (tag, _) => Err(format!(
      "`expect` names `{tag}`, which this corpus does not read"
    )),
  }
}

/// `input` is the wire response as a JSON value — what a backend emits.
fn check_protocol(fixture: &Fixture<'_>) -> Result<(), String> {
  let wire = serde_json::from_value::<WireResponse>(fixture.input.clone());
  compare(outcome(wire, fixture), fixture.expect)
}

/// `input` is a JSON **string** holding the document text verbatim, for the two
/// literals a `serde_json::Value` cannot carry.
fn check_protocol_text(fixture: &Fixture<'_>) -> Result<(), String> {
  let text = fixture
    .input
    .as_str()
    .ok_or_else(|| "`input` is not a string of document text".to_owned())?;
  let wire = serde_json::from_str::<WireResponse>(text);
  compare(outcome(wire, fixture), fixture.expect)
}

const PROTOCOL: Corpus = Corpus {
  root: "tests/protocol/fixtures/protocol",
  check: check_protocol,
};

/// Separate from `PROTOCOL` so the vacuity guard covers each directory on its
/// own: emptying either one fails by itself.
const PROTOCOL_TEXT: Corpus = Corpus {
  root: "tests/protocol/fixtures/protocol-text",
  check: check_protocol_text,
};

#[test]
fn every_protocol_fixture_states_what_a_wire_document_means() {
  assert_corpus(&PROTOCOL);
}

#[test]
fn the_two_numeric_literals_json_cannot_express_are_refused_before_normalization() {
  assert_corpus(&PROTOCOL_TEXT);
}

// ---------------------------------------------------------------------------
// VT-2: every variant of the taxonomy is named by a fixture
// ---------------------------------------------------------------------------
//
// `render_error` above is exhaustive, so a thirteenth `ProtocolError` cannot be
// added without an arm. That stops a variant being *unrenderable*; it does not
// stop one being untested. This is the other half: the corpus itself is read
// back, and a variant no fixture names fails the run.
//
// **Two variants are exempt, and both exemptions are the design's rather than
// this phase's.**
//
// `ProtocolError::Schedule` never arrives as an `Err` at all. An unusable
// `next_check` is a discard on an otherwise successful parse (P2, R-25), which
// is what `Normalized::discarded` is for — so the fixture that names a schedule
// failure names it there, and asserting it as an error would assert the
// opposite of the requirement. `design.md` §5.2 states this in as many words
// under AC-6.
//
// `BoundsError::NotFinite` is unreachable from the wire. JSON expresses neither
// `NaN` nor an infinity, and `serde_json` refuses both while parsing — measured
// before this corpus was written, and again by the two `protocol-text` fixtures,
// which assert `Json` in its place (F-36, D39). The variant stays in the
// taxonomy because `NumberRange::new` is public API and the claim is about what
// the type can hold; a fixture asserting it would be a test that cannot fail.

/// One instance per `ProtocolError` variant, so the tag set is derived from the
/// type rather than typed out beside it.
fn every_protocol_error() -> Vec<ProtocolError> {
  let at = "view.options[0]".to_owned();
  let id = "later".to_owned();
  vec![
    ProtocolError::Json(json_error()),
    ProtocolError::UnsupportedProtocolVersion { found: 2 },
    ProtocolError::UnsupportedPrimitive {
      kind: "slider".to_owned(),
      at: at.clone(),
    },
    ProtocolError::InapplicableKey {
      key: "min",
      kind: "text".to_owned(),
      at: at.clone(),
    },
    ProtocolError::MissingField { field: "view" },
    ProtocolError::EmptyOptions { at: at.clone() },
    ProtocolError::DuplicateOptionId {
      id: id.clone(),
      at: at.clone(),
    },
    ProtocolError::DuplicateFieldId {
      id: id.clone(),
      at: at.clone(),
    },
    ProtocolError::DuplicateAlternativeId { id, at: at.clone() },
    ProtocolError::EmptyAlternatives { at },
    ProtocolError::Bounds(BoundsError::Inverted {
      min: 10.0,
      max: 1.0,
    }),
    ProtocolError::Schedule(ScheduleError::Unparseable {
      raw: "tomorrow morning".to_owned(),
    }),
  ]
}

fn json_error() -> serde_json::Error {
  serde_json::from_str::<Value>("{").unwrap_err()
}

/// The single key of an externally tagged object.
///
/// Reads through `outcome_tag` rather than walking the map again: the tagging
/// discipline belongs to the shared half, and a second implementation of it
/// here could disagree with the one every fixture actually goes through.
fn sole_tag(value: &Value) -> Option<String> {
  outcome_tag(value).ok().map(|(tag, _)| tag.to_owned())
}

/// Every fixture in a corpus, as the JSON it is. Reading the corpus back is a
/// different job from running it, so it does not go through `Corpus::run`; it is
/// one walk regardless, shared by the two coverage tests below.
fn fixtures_of(corpus: &Corpus) -> Vec<Value> {
  let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(corpus.root);
  let entries = std::fs::read_dir(&root)
    .unwrap_or_else(|error| panic!("{}: could not be read: {error}", root.display()));
  let mut fixtures = Vec::new();
  for entry in entries {
    let path = entry.expect("a directory entry").path();
    if path.extension().is_none_or(|extension| extension != "json") {
      continue;
    }
    let text = std::fs::read_to_string(&path).expect("a fixture");
    fixtures.push(serde_json::from_str(&text).expect("a fixture"));
  }
  fixtures
}

/// Every `expect.error` tag the two corpora name, and every `Bounds` tag under
/// them.
fn tags_named_by_fixtures() -> (Vec<String>, Vec<String>) {
  let mut errors = Vec::new();
  let mut bounds = Vec::new();
  for corpus in [&PROTOCOL, &PROTOCOL_TEXT] {
    for fixture in fixtures_of(corpus) {
      let Some(error) = fixture.get("expect").and_then(|expect| expect.get("error")) else {
        continue;
      };
      if let Some(tag) = sole_tag(error) {
        if tag == "Bounds"
          && let Some(inner) = error.get("Bounds").and_then(sole_tag)
        {
          bounds.push(inner);
        }
        errors.push(tag);
      }
    }
  }
  (errors, bounds)
}

#[test]
fn every_reachable_error_in_the_taxonomy_is_named_by_a_fixture() {
  let (errors, bounds) = tags_named_by_fixtures();
  for error in every_protocol_error() {
    let Some(tag) = sole_tag(&render_error(&error)) else {
      panic!("`{error}` does not render as an externally tagged object");
    };
    // See the block comment above for why this one cannot be a fixture.
    if tag == "Schedule" {
      assert!(
        !errors.contains(&tag),
        "a fixture asserts `Schedule` as an error, but an unusable schedule is a \
         discard on an accepted message (P2, R-25)"
      );
      continue;
    }
    assert!(
      errors.contains(&tag),
      "no fixture names `{tag}`, so nothing in the corpus can fail if it stops being raised"
    );
  }
  assert!(
    bounds.contains(&"Inverted".to_owned()),
    "no fixture names the one bounds failure a JSON document can express"
  );
  assert!(
    !bounds.contains(&"NotFinite".to_owned()),
    "a fixture asserts `NotFinite`, which JSON cannot express and no wire document can reach \
     — a test that cannot fail (F-36, D39)"
  );
}

/// The other half of the same claim: a schedule failure *is* named by a
/// fixture, on the channel the design puts it on.
#[test]
fn a_schedule_failure_is_named_by_a_fixture_as_a_discard() {
  let mut reasons = Vec::new();
  for fixture in fixtures_of(&PROTOCOL) {
    let discarded = fixture
      .get("expect")
      .and_then(|expect| expect.get("accepted"))
      .and_then(|accepted| accepted.get("discarded"))
      .and_then(Value::as_array);
    for item in discarded.into_iter().flatten() {
      if let Some(reason) = item
        .get("schedule")
        .and_then(|schedule| schedule.get("reason"))
        .and_then(Value::as_str)
      {
        reasons.push(reason.to_owned());
      }
    }
  }
  assert!(
    !reasons.is_empty(),
    "no fixture shows a scheduling failure surviving as a discard, which is the whole of P2's \
     granularity rule for this field"
  );
}

//! The inbound envelope — `design.md` §5.2, `SPEC-003/R-9`, R-10, R-13.
//!
//! Permissive-in / precise-diagnostic, the same split `File` → `Config`
//! (`config.rs`) and `protocol/wire.rs` → `protocol/normalize.rs` already use
//! in this workspace: [`normalize`] is the only door from bytes a watcher wrote
//! into a canonical [`Event`], and past it nothing is unvalidated.
//!
//! **The envelope is shape, never meaning** (`SPEC-003` P-A). `kind` is
//! never matched against a list, `data` is never read into, and `timestamp` is
//! never compared to `now` — the one comparison this module makes is `source`
//! against the single reserved string R-13 names.
#![deny(clippy::arithmetic_side_effects)]

use std::fmt;

use goad_semantics::error::{ProtocolError, json_type_name};
use goad_semantics::protocol::canonical::{Event, Timestamp};
use goad_semantics::protocol::wire::reject_duplicate_keys;

/// The four keys `SPEC-003` §6.2 admits, and none beside them.
const KEYS: [&str; 4] = ["source", "kind", "timestamp", "data"];

/// Why an envelope was refused before it reached `Event`.
///
/// One fault per way R-9, R-10 and R-13 refuse a wire envelope, so a caller
/// building the wire's `invalid_envelope` / `reserved_source` reply reads it
/// off the discriminant rather than a message (`SPEC-003` §6.3).
#[derive(Debug)]
pub enum EnvelopeFault {
  /// The bytes are not one JSON document at all — not one of R-9's own
  /// clauses (those all presuppose well-formed JSON), but `normalize` must
  /// still name it rather than panic on attacker-controlled bytes.
  Malformed,
  /// A well-formed top-level value that is not an object.
  NotAnObject { found: &'static str },
  /// One of the four required keys is absent.
  Missing { key: &'static str },
  /// `source`, `kind` or `timestamp` present and not a string.
  WrongType {
    key: &'static str,
    found: &'static str,
  },
  /// `source` or `kind` present, a string, and empty.
  Empty { key: &'static str },
  /// A key beside the four.
  Unknown { key: String },
  /// The same key twice, at any depth.
  Duplicate { key: String },
  /// `timestamp` parses as a civil datetime: well-formed, no offset.
  MissingOffset { raw: String },
  /// `timestamp` is neither an offset instant nor a bare civil datetime.
  Unparseable { raw: String },
  /// `source == "host"`, reserved to evaluations the host originates
  /// (R-13, `SPEC-001/R-56`).
  ReservedSource,
}

impl fmt::Display for EnvelopeFault {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Malformed => write!(f, "the bytes are not one JSON document"),
      Self::NotAnObject { found } => write!(f, "expected an object, found a JSON {found}"),
      Self::Missing { key } => write!(f, "missing required key `{key}`"),
      Self::WrongType { key, found } => {
        write!(f, "`{key}` must be a string, found a JSON {found}")
      }
      Self::Empty { key } => write!(f, "`{key}` must not be empty"),
      Self::Unknown { key } => write!(f, "unknown key `{key}`"),
      Self::Duplicate { key } => write!(f, "duplicate key `{key}`"),
      Self::MissingOffset { raw } => write!(f, "timestamp has no UTC offset: {raw}"),
      Self::Unparseable { raw } => write!(f, "unparseable timestamp: {raw}"),
      Self::ReservedSource => {
        write!(
          f,
          "source \"host\" is reserved to evaluations the host originates"
        )
      }
    }
  }
}

impl std::error::Error for EnvelopeFault {}

/// Turn the bytes a watcher wrote into a canonical [`Event`], or name why they
/// cannot be.
///
/// Three steps, in this order: the duplicate-key walk (reusing
/// `goad_semantics::protocol::wire::reject_duplicate_keys`, which is also
/// where a document that fails to parse at all is discovered — no second walk
/// is written); the top-level shape check; then each of the four keys in turn.
///
/// # Errors
///
/// One [`EnvelopeFault`] per way the bytes are not an admissible envelope.
pub fn normalize(bytes: &[u8]) -> Result<Event, EnvelopeFault> {
  let object = as_object(parse(bytes)?)?;
  envelope(object)
}

/// The duplicate-key walk, then a generic parse. The second parse cannot fail
/// where the first succeeded — the walk already consumed the same bytes as one
/// JSON document — so this is not a second reader of validity, only of shape.
fn parse(bytes: &[u8]) -> Result<serde_json::Value, EnvelopeFault> {
  reject_duplicate_keys(bytes).map_err(|error| match error {
    ProtocolError::DuplicateKey { key } => EnvelopeFault::Duplicate { key },
    _malformed => EnvelopeFault::Malformed,
  })?;
  serde_json::from_slice(bytes).map_err(|_malformed| EnvelopeFault::Malformed)
}

/// R-9's first clause: a well-formed value that is not an object, naming the
/// type found.
fn as_object(
  value: serde_json::Value,
) -> Result<serde_json::Map<String, serde_json::Value>, EnvelopeFault> {
  match value {
    serde_json::Value::Object(object) => Ok(object),
    other => Err(EnvelopeFault::NotAnObject {
      found: json_type_name(&other),
    }),
  }
}

/// R-9's remaining clauses, over an object already known to have no duplicate
/// key at any depth: any key beside the four, then each of the four in turn.
/// `source == "host"` (R-13) is checked once `source` is known to be a
/// non-empty string, and is the only comparison made on any value here (P-A).
fn envelope(
  mut object: serde_json::Map<String, serde_json::Value>,
) -> Result<Event, EnvelopeFault> {
  if let Some(unknown) = object.keys().find(|key| !KEYS.contains(&key.as_str())) {
    return Err(EnvelopeFault::Unknown {
      key: unknown.to_owned(),
    });
  }

  let source = take_string(&mut object, "source")?;
  let kind = take_string(&mut object, "kind")?;
  let timestamp = take_timestamp(&mut object, "timestamp")?;
  let data = object
    .remove("data")
    .ok_or(EnvelopeFault::Missing { key: "data" })?;

  if source.is_empty() {
    return Err(EnvelopeFault::Empty { key: "source" });
  }
  if source == "host" {
    return Err(EnvelopeFault::ReservedSource);
  }
  if kind.is_empty() {
    return Err(EnvelopeFault::Empty { key: "kind" });
  }

  Ok(Event {
    source,
    kind,
    timestamp,
    data,
  })
}

/// `key`, present and a string, or the fault that says which it failed to be.
fn take_string(
  object: &mut serde_json::Map<String, serde_json::Value>,
  key: &'static str,
) -> Result<String, EnvelopeFault> {
  match object.remove(key) {
    None => Err(EnvelopeFault::Missing { key }),
    Some(serde_json::Value::String(value)) => Ok(value),
    Some(other) => Err(EnvelopeFault::WrongType {
      key,
      found: json_type_name(&other),
    }),
  }
}

/// `timestamp`: present, a string, and R-10's one admitted form — RFC 3339
/// with an explicit offset. The same jiff two-step `schedule.rs`'s
/// `parse_instruction` uses, absolute parse first so an offset form is never
/// misread as offsetless (`SPEC-001/R-22`'s argument, mirrored rather than
/// shared code because the two report through different fault types) — with
/// no span fallback, because R-10 admits only the absolute form.
fn take_timestamp(
  object: &mut serde_json::Map<String, serde_json::Value>,
  key: &'static str,
) -> Result<Timestamp, EnvelopeFault> {
  let raw = take_string(object, key)?;
  if let Ok(instant) = raw.parse::<jiff::Timestamp>() {
    return Ok(Timestamp::new(instant));
  }
  if raw.parse::<jiff::civil::DateTime>().is_ok() {
    return Err(EnvelopeFault::MissingOffset { raw });
  }
  Err(EnvelopeFault::Unparseable { raw })
}

#[cfg(test)]
mod tests {
  use super::{EnvelopeFault, normalize};

  /// `design.md`'s own example, `SPEC-003` §6.2 — compact and on one
  /// line, so a case can substitute one key's value with a plain
  /// [`str::replace`] the way `config.rs`'s own tests substitute a TOML line.
  const GOOD: &str = r#"{"source":"reddit-watcher","kind":"reddit-opened","timestamp":"2026-08-22T17:10:00+10:00","data":{"count_last_hour":4}}"#;

  fn fault(bytes: &str) -> EnvelopeFault {
    match normalize(bytes.as_bytes()) {
      Err(fault) => fault,
      Ok(event) => panic!("accepted, as {event:?}"),
    }
  }

  // ---- VT-11: the accepted shapes ----

  #[test]
  fn the_design_s_own_example_normalizes() {
    let event = normalize(GOOD.as_bytes()).expect("the design's own example must normalize");
    assert_eq!(event.source, "reddit-watcher");
    assert_eq!(event.kind, "reddit-opened");
    assert_eq!(event.data, serde_json::json!({"count_last_hour": 4}));
  }

  #[test]
  fn a_null_data_is_accepted() {
    let text = GOOD.replace(r#"{"count_last_hour":4}"#, "null");
    let event = normalize(text.as_bytes()).expect("null data must be accepted");
    assert_eq!(event.data, serde_json::Value::Null);
  }

  #[test]
  fn data_carrying_a_nested_object_and_an_array_reaches_event_unchanged() {
    let text = GOOD.replace(
      r#"{"count_last_hour":4}"#,
      r#"{"nested":{"n":1},"list":[1,2,3]}"#,
    );
    let event = normalize(text.as_bytes()).expect("nested data must be accepted");
    assert_eq!(
      event.data,
      serde_json::json!({"nested": {"n": 1}, "list": [1, 2, 3]})
    );
  }

  #[test]
  fn timestamps_far_from_now_are_carried_unjudged() {
    for raw in ["1970-01-01T00:00:00Z", "3000-01-01T00:00:00Z"] {
      let text = GOOD.replace("2026-08-22T17:10:00+10:00", raw);
      let event = normalize(text.as_bytes())
        .unwrap_or_else(|fault| panic!("{raw} must be carried unjudged, refused as {fault}"));
      assert_eq!(event.timestamp.instant(), raw.parse().unwrap());
    }
  }

  // ---- VT-2: a well-formed value that is not an object ----

  #[test]
  fn a_non_object_top_level_is_refused_naming_the_type_found() {
    for (json, kind) in [
      ("[1, 2, 3]", "array"),
      ("\"reddit-opened\"", "string"),
      ("45", "number"),
      ("true", "boolean"),
      ("null", "null"),
    ] {
      assert!(
        matches!(fault(json), EnvelopeFault::NotAnObject { found } if found == kind),
        "a top-level {kind} was not refused naming it"
      );
    }
  }

  // ---- VT-3: each of the four keys missing ----

  #[test]
  fn each_of_the_four_keys_missing_is_refused_naming_it() {
    for (text, key) in [
      (
        r#"{"kind":"reddit-opened","timestamp":"2026-08-22T17:10:00+10:00","data":{}}"#,
        "source",
      ),
      (
        r#"{"source":"reddit-watcher","timestamp":"2026-08-22T17:10:00+10:00","data":{}}"#,
        "kind",
      ),
      (
        r#"{"source":"reddit-watcher","kind":"reddit-opened","data":{}}"#,
        "timestamp",
      ),
      (
        r#"{"source":"reddit-watcher","kind":"reddit-opened","timestamp":"2026-08-22T17:10:00+10:00"}"#,
        "data",
      ),
    ] {
      assert!(
        matches!(fault(text), EnvelopeFault::Missing { key: found } if found == key),
        "a missing `{key}` was not refused naming it: {:?}",
        fault(text)
      );
    }
  }

  // ---- VT-4: the three typed keys, wrong-typed ----
  //
  // `data` admits any JSON value (`design.md` §5.2's field table), so there is
  // no wrong-typed case for it — VT-3 above is the one that covers all four.

  #[test]
  fn each_typed_key_wrong_typed_is_refused_naming_it() {
    for (needle, replacement, key) in [
      (r#""source":"reddit-watcher""#, r#""source":45"#, "source"),
      (r#""kind":"reddit-opened""#, r#""kind":45"#, "kind"),
      (
        r#""timestamp":"2026-08-22T17:10:00+10:00""#,
        r#""timestamp":45"#,
        "timestamp",
      ),
    ] {
      let text = GOOD.replace(needle, replacement);
      assert!(
        matches!(
          fault(&text),
          EnvelopeFault::WrongType { key: found, found: "number" } if found == key
        ),
        "a wrong-typed `{key}` was not refused naming it: {:?}",
        fault(&text)
      );
    }
  }

  // ---- VT-5: empty source and empty kind ----

  #[test]
  fn an_empty_source_or_kind_is_refused_naming_it() {
    for (needle, replacement, key) in [
      (r#""source":"reddit-watcher""#, r#""source":"""#, "source"),
      (r#""kind":"reddit-opened""#, r#""kind":"""#, "kind"),
    ] {
      let text = GOOD.replace(needle, replacement);
      assert!(
        matches!(fault(&text), EnvelopeFault::Empty { key: found } if found == key),
        "an empty `{key}` was not refused naming it: {:?}",
        fault(&text)
      );
    }
  }

  // ---- VT-6: a fifth key beside the four ----

  #[test]
  fn a_fifth_key_beside_the_four_is_refused_naming_it() {
    let text = GOOD.replacen('{', r#"{"protocol":1,"#, 1);
    assert!(
      matches!(fault(&text), EnvelopeFault::Unknown { key } if key == "protocol"),
      "a fifth key was not refused naming it: {:?}",
      fault(&text)
    );
  }

  // ---- VT-7: a duplicate key, top-level and nested inside `data` ----

  #[test]
  fn a_top_level_duplicate_key_is_refused_naming_it() {
    let text = GOOD.replacen('{', r#"{"source":"duplicate","#, 1);
    assert!(
      matches!(fault(&text), EnvelopeFault::Duplicate { key } if key == "source"),
      "a top-level duplicate `source` was not refused naming it: {:?}",
      fault(&text)
    );
  }

  #[test]
  fn a_duplicate_key_nested_inside_data_is_refused_naming_it() {
    let text = GOOD.replace(
      r#"{"count_last_hour":4}"#,
      r#"{"count_last_hour":4,"count_last_hour":5}"#,
    );
    assert!(
      matches!(fault(&text), EnvelopeFault::Duplicate { key } if key == "count_last_hour"),
      "a duplicate key nested inside `data` was not refused naming it: {:?}",
      fault(&text)
    );
  }

  // ---- VT-8: the two timestamp faults, distinct from each other and from
  // wrong-typed (asserting the discriminant, not the message) ----

  #[test]
  fn an_offsetless_instant_is_refused_distinctly_from_an_unparseable_one() {
    let offsetless = GOOD.replace(
      r#""timestamp":"2026-08-22T17:10:00+10:00""#,
      r#""timestamp":"2026-08-22T17:10:00""#,
    );
    assert!(
      matches!(
        fault(&offsetless),
        EnvelopeFault::MissingOffset { raw } if raw == "2026-08-22T17:10:00"
      ),
      "an offsetless instant was not refused as MissingOffset: {:?}",
      fault(&offsetless)
    );

    let unparseable = GOOD.replace(
      r#""timestamp":"2026-08-22T17:10:00+10:00""#,
      r#""timestamp":"not a timestamp""#,
    );
    assert!(
      matches!(
        fault(&unparseable),
        EnvelopeFault::Unparseable { raw } if raw == "not a timestamp"
      ),
      "an unparseable timestamp was not refused as Unparseable: {:?}",
      fault(&unparseable)
    );
  }

  // ---- VT-9: source == "host" ----

  #[test]
  fn a_reserved_source_is_refused_with_every_other_field_valid() {
    let text = GOOD.replace(r#""source":"reddit-watcher""#, r#""source":"host""#);
    assert!(
      matches!(fault(&text), EnvelopeFault::ReservedSource),
      "source \"host\" was not refused as reserved: {:?}",
      fault(&text)
    );
  }

  // ---- bytes that are not JSON at all: soundness, not a named VT ----

  #[test]
  fn bytes_that_are_not_json_are_refused_as_malformed() {
    assert!(matches!(fault("not json"), EnvelopeFault::Malformed));
  }
}

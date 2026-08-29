//! The AC-6 taxonomy, stratum 1 half — `design.md` §5.2.
//!
//! Parse and validation failures only. The transport and host-state halves
//! (`BackendError`, `CleanupFailure`, `StateError`) live in stratum 2 and wrap
//! these; that split is the seam ADR-001 predicted the error taxonomy would
//! acquire, and it is why a flat enum spanning both does not appear here.
//!
//! Nothing in this module inspects a value or does arithmetic, so it carries no
//! module-level `#![deny(clippy::arithmetic_side_effects)]` (D53 as amended).
//! The first module that handles backend-derived data at run time owes one.

use std::fmt;

/// Stratum 1: the wire was malformed, or it was well-formed and said something
/// the protocol does not admit.
#[derive(Debug)]
pub enum ProtocolError {
  Json(serde_json::Error),
  UnsupportedProtocolVersion {
    found: u32,
  },
  /// `at` is a path, per F-6: once fields and alternatives nest, naming the
  /// kind without naming the place is a puzzle rather than a diagnostic.
  UnsupportedPrimitive {
    kind: String,
    at: String,
  },
  /// A modelled key the kind does not admit. Rejected rather than demoted to a
  /// hint: serde consumes it before `kind` is dispatched, so ignoring it means
  /// losing it silently (D43).
  InapplicableKey {
    key: &'static str,
    kind: String,
    at: String,
  },
  MissingField {
    field: &'static str,
  },
  EmptyOptions {
    at: String,
  },
  DuplicateOptionId {
    id: String,
    at: String,
  },
  DuplicateFieldId {
    id: String,
    at: String,
  },
  DuplicateAlternativeId {
    id: String,
    at: String,
  },
  EmptyAlternatives {
    at: String,
  },
  Bounds(BoundsError),
  Schedule(ScheduleError),
}

/// Bounds are semantics, not presentation: they constrain which answers are
/// valid, so a range the host cannot interpret costs the sender the message
/// rather than being discarded (P2).
#[derive(Debug)]
pub enum BoundsError {
  /// Unreachable from JSON — `NaN` and `1e400` both fail in `serde_json` before
  /// any bounds check runs. Kept regardless: `NumberRange::new` is public API
  /// and the claim is about what the *type* can hold, not about who supplied
  /// it (D39, F-36). Do not tidy this away.
  NotFinite { bound: &'static str, found: f64 },
  /// `min > max`, which makes every answer invalid.
  Inverted { min: f64, max: f64 },
}

/// An invalid scheduling value is a typed error but does **not** arrive as an
/// `Err`: it travels inside `Normalized::discarded` on an otherwise successful
/// parse. AC-6 asks for a distinct typed error, which this is; it never asked
/// for the call to fail.
#[derive(Debug)]
pub enum ScheduleError {
  /// `"next_check": 45`.
  NotAString { found: &'static str },
  /// `2026-08-22T18:00:00` — broken out from `Unparseable` because it is the
  /// single most likely backend mistake and "you omitted the offset" is
  /// debuggable where "unparseable" is not (brief §13).
  MissingOffset { raw: String },
  /// `"1 month"` — a calendar unit has no fixed length.
  CalendarUnit { raw: String },
  /// Parses, but `now + span` leaves the representable range.
  OutOfRange { raw: String },
  /// `"tomorrow morning"`.
  Unparseable { raw: String },
}

impl fmt::Display for ProtocolError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Json(inner) => write!(f, "malformed JSON: {inner}"),
      Self::UnsupportedProtocolVersion { found } => {
        write!(f, "unsupported protocol version {found}")
      }
      Self::UnsupportedPrimitive { kind, at } => {
        write!(f, "unsupported primitive `{kind}` at {at}")
      }
      Self::InapplicableKey { key, kind, at } => {
        write!(f, "key `{key}` does not apply to a `{kind}` at {at}")
      }
      Self::MissingField { field } => write!(f, "missing required field `{field}`"),
      Self::EmptyOptions { at } => write!(f, "no options at {at}"),
      Self::DuplicateOptionId { id, at } => write!(f, "duplicate option id `{id}` at {at}"),
      Self::DuplicateFieldId { id, at } => write!(f, "duplicate field id `{id}` at {at}"),
      Self::DuplicateAlternativeId { id, at } => {
        write!(f, "duplicate alternative id `{id}` at {at}")
      }
      Self::EmptyAlternatives { at } => write!(f, "no alternatives at {at}"),
      Self::Bounds(inner) => write!(f, "invalid bounds: {inner}"),
      Self::Schedule(inner) => write!(f, "invalid schedule: {inner}"),
    }
  }
}

impl fmt::Display for BoundsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NotFinite { bound, found } => write!(f, "`{bound}` is not finite: {found}"),
      Self::Inverted { min, max } => write!(f, "min {min} is above max {max}"),
    }
  }
}

impl fmt::Display for ScheduleError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NotAString { found } => write!(f, "schedule must be a string, found {found}"),
      Self::MissingOffset { raw } => write!(f, "schedule has no UTC offset: {raw}"),
      Self::CalendarUnit { raw } => {
        write!(
          f,
          "schedule uses a calendar unit, which has no fixed length: {raw}"
        )
      }
      Self::OutOfRange { raw } => write!(f, "schedule leaves the representable range: {raw}"),
      Self::Unparseable { raw } => write!(f, "unparseable schedule: {raw}"),
    }
  }
}

impl std::error::Error for ProtocolError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Json(inner) => Some(inner),
      Self::Bounds(inner) => Some(inner),
      Self::Schedule(inner) => Some(inner),
      _ => None,
    }
  }
}

impl std::error::Error for BoundsError {}

impl std::error::Error for ScheduleError {}

#[cfg(test)]
mod tests {
  use super::{BoundsError, ProtocolError, ScheduleError};

  /// The values a variant's `Display` must name.
  ///
  /// An exhaustive match, deliberately: a variant added to the taxonomy without
  /// an arm here fails to compile. That is VT-3's actual job — stopping a
  /// variant being declared with a field nothing ever formats.
  fn must_name(error: &ProtocolError) -> Vec<String> {
    match error {
      ProtocolError::Json(inner) => vec![inner.to_string()],
      ProtocolError::UnsupportedProtocolVersion { found } => vec![found.to_string()],
      ProtocolError::UnsupportedPrimitive { kind, at } => vec![kind.clone(), at.clone()],
      ProtocolError::InapplicableKey { key, kind, at } => {
        vec![(*key).to_owned(), kind.clone(), at.clone()]
      }
      ProtocolError::MissingField { field } => vec![(*field).to_owned()],
      ProtocolError::EmptyOptions { at } | ProtocolError::EmptyAlternatives { at } => {
        vec![at.clone()]
      }
      ProtocolError::DuplicateOptionId { id, at }
      | ProtocolError::DuplicateFieldId { id, at }
      | ProtocolError::DuplicateAlternativeId { id, at } => vec![id.clone(), at.clone()],
      ProtocolError::Bounds(inner) => vec![inner.to_string()],
      ProtocolError::Schedule(inner) => vec![inner.to_string()],
    }
  }

  fn bounds_must_name(error: &BoundsError) -> Vec<String> {
    match error {
      BoundsError::NotFinite { bound, found } => vec![(*bound).to_owned(), found.to_string()],
      BoundsError::Inverted { min, max } => vec![min.to_string(), max.to_string()],
    }
  }

  fn schedule_must_name(error: &ScheduleError) -> Vec<String> {
    match error {
      ScheduleError::NotAString { found } => vec![(*found).to_owned()],
      ScheduleError::MissingOffset { raw }
      | ScheduleError::CalendarUnit { raw }
      | ScheduleError::OutOfRange { raw }
      | ScheduleError::Unparseable { raw } => vec![raw.clone()],
    }
  }

  fn assert_names(error: &impl std::fmt::Display, values: &[String]) {
    let rendered = error.to_string();
    for value in values {
      assert!(
        rendered.contains(value.as_str()),
        "`{rendered}` never names the `{value}` it carries"
      );
      assert!(
        !value.is_empty(),
        "a case that expects nothing asserts nothing"
      );
    }
    assert!(
      !values.is_empty(),
      "`{rendered}` has no expectations, so it is untested"
    );
  }

  fn json_error() -> serde_json::Error {
    serde_json::from_str::<serde_json::Value>("{").unwrap_err()
  }

  /// One instance per `ProtocolError` variant, in `design.md` §5.2's order.
  fn every_protocol_error() -> Vec<ProtocolError> {
    vec![
      ProtocolError::Json(json_error()),
      ProtocolError::UnsupportedProtocolVersion { found: 7 },
      ProtocolError::UnsupportedPrimitive {
        kind: "slider".to_owned(),
        at: "$.view.fields[0]".to_owned(),
      },
      ProtocolError::InapplicableKey {
        key: "min",
        kind: "text".to_owned(),
        at: "$.view.fields[1]".to_owned(),
      },
      ProtocolError::MissingField {
        field: "protocol_version",
      },
      ProtocolError::EmptyOptions {
        at: "$.view".to_owned(),
      },
      ProtocolError::DuplicateOptionId {
        id: "later".to_owned(),
        at: "$.view.options".to_owned(),
      },
      ProtocolError::DuplicateFieldId {
        id: "note".to_owned(),
        at: "$.view.fields".to_owned(),
      },
      ProtocolError::DuplicateAlternativeId {
        id: "amber".to_owned(),
        at: "$.view.fields[2].alternatives".to_owned(),
      },
      ProtocolError::EmptyAlternatives {
        at: "$.view.fields[2]".to_owned(),
      },
      ProtocolError::Bounds(BoundsError::Inverted {
        min: 10.0,
        max: 1.0,
      }),
      ProtocolError::Schedule(ScheduleError::Unparseable {
        raw: "tomorrow morning".to_owned(),
      }),
    ]
  }

  fn every_bounds_error() -> Vec<BoundsError> {
    vec![
      BoundsError::NotFinite {
        bound: "min",
        found: f64::NAN,
      },
      BoundsError::Inverted {
        min: 10.0,
        max: 1.0,
      },
    ]
  }

  fn every_schedule_error() -> Vec<ScheduleError> {
    vec![
      ScheduleError::NotAString { found: "number" },
      ScheduleError::MissingOffset {
        raw: "2026-08-22T18:00:00".to_owned(),
      },
      ScheduleError::CalendarUnit {
        raw: "1 month".to_owned(),
      },
      ScheduleError::OutOfRange {
        raw: "100000 days".to_owned(),
      },
      ScheduleError::Unparseable {
        raw: "tomorrow morning".to_owned(),
      },
    ]
  }

  #[test]
  fn every_protocol_error_display_names_what_it_carries() {
    for error in every_protocol_error() {
      assert_names(&error, &must_name(&error));
    }
  }

  #[test]
  fn every_bounds_error_display_names_what_it_carries() {
    for error in every_bounds_error() {
      assert_names(&error, &bounds_must_name(&error));
    }
  }

  #[test]
  fn every_schedule_error_display_names_what_it_carries() {
    for error in every_schedule_error() {
      assert_names(&error, &schedule_must_name(&error));
    }
  }

  /// EX-2's second half: `std::error::Error`, not just `Display`. A wrapping
  /// variant must also hand back what it wraps, or the chain stops here.
  #[test]
  fn the_taxonomy_implements_error_and_wrapping_variants_expose_their_source() {
    fn is_error<E: std::error::Error>(_: &E) {}

    is_error(&ProtocolError::MissingField { field: "view" });
    is_error(&BoundsError::Inverted {
      min: 10.0,
      max: 1.0,
    });
    is_error(&ScheduleError::NotAString { found: "number" });

    for error in every_protocol_error() {
      let wrapping = matches!(
        error,
        ProtocolError::Json(_) | ProtocolError::Bounds(_) | ProtocolError::Schedule(_)
      );
      assert_eq!(
        std::error::Error::source(&error).is_some(),
        wrapping,
        "`{error}` disagrees with itself about whether it wraps another error"
      );
    }
  }
}

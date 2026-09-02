//! `next_check`: one canonical instant, or one named `ScheduleError` —
//! `design.md` §5.2 and §5.5, brief §9, `draft-spec.md` R-21…R-28.
//!
//! Two pure functions, and the clock is not one of them. `now` is a parameter
//! on both (I3): stratum 1 reads no clock, and this is the module where that
//! would be most tempting.
//!
//! This module handles backend-derived data *and* does time arithmetic, so it
//! carries the module-level deny D53 (as amended) leaves to such modules. The
//! consequence is deliberate: bare `+` on an instant does not compile here, so
//! `checked_add` is the only spelling available — and the overflow it reports
//! **is** `OutOfRange`, which makes forgetting the check impossible rather than
//! merely discouraged.
#![deny(clippy::arithmetic_side_effects)]

use crate::semantics::error::ScheduleError;
use crate::semantics::protocol::canonical::Timestamp;

/// The JSON type name `NotAString` reports. `&'static str` by construction, so
/// the diagnostic names a type and never formats the offending value.
fn json_type_name(value: &serde_json::Value) -> &'static str {
  match value {
    serde_json::Value::Null => "null",
    serde_json::Value::Bool(_) => "boolean",
    serde_json::Value::Number(_) => "number",
    serde_json::Value::String(_) => "string",
    serde_json::Value::Array(_) => "array",
    serde_json::Value::Object(_) => "object",
  }
}

/// Read a wire `next_check` as one instant, or name why it is not one.
///
/// The argument is the untyped JSON value rather than a `&str` because
/// `NotAString` exists to report `"next_check": 45`, and naming what was found
/// there requires seeing it (R-25).
///
/// **`Value::Null` is not this function's case.** An explicit `null` means what
/// omission means for every modelled field but `view` (D50, R-51), and it must
/// produce no discard at all — so the caller elides it before calling. Handed a
/// `Null` this function says `NotAString { found: "null" }`, which is the
/// discard R-51 forbids. The rule is normalization-wide rather than per-field,
/// so it lives at the one place that can apply it uniformly:
/// `normalize_response`, which PHASE-04 lands.
///
/// The failure kinds separate structurally — a parse that succeeds or fails, a
/// conversion that succeeds or fails — so no branch below reads an error
/// message. Measured on jiff 0.2.35, `notes.md` PHASE-03.
///
/// # Errors
///
/// One `ScheduleError` per way of not being an instant, and they are distinct
/// because brief §13 wants a backend author to be able to act on the message:
/// `NotAString` when the value is not a string at all, naming the JSON type
/// found instead (R-25); `MissingOffset` for an absolute instant written
/// without one, which is the single most likely mistake (R-22); `CalendarUnit`
/// for months or years, whose length is not fixed without a calendar (R-23);
/// `OutOfRange` for a span that is well formed but lands outside representable
/// time; and `Unparseable` for everything else, including a span whose
/// magnitude the duration grammar itself refuses.
///
/// None of these fails a message. Every one is a discard under P2 — the
/// instruction is lost, scheduling is not (R-25).
pub fn parse(value: &serde_json::Value, now: Timestamp) -> Result<Timestamp, ScheduleError> {
  match value.as_str() {
    None => Err(ScheduleError::NotAString {
      found: json_type_name(value),
    }),
    Some(raw) => parse_instruction(raw, now),
  }
}

/// R-21's two forms, in the one order that tells them apart.
///
/// Absolute is tried first and that is load-bearing, not incidental: an offset
/// form such as `"2026-08-23T05:00:00+10:00"` parses as an instant **and** as a
/// civil datetime, so trying civil first would report `MissingOffset` for a
/// value that carries one. Below that the three parsers do partition — no
/// string parses as both a civil datetime and a span — so the remaining order
/// is a matter of naming rather than of correctness.
fn parse_instruction(raw: &str, now: Timestamp) -> Result<Timestamp, ScheduleError> {
  if let Ok(instant) = raw.parse::<jiff::Timestamp>() {
    // Stored exactly as given, past or future. Clamping to `now` would have the
    // host rewrite the backend's instruction (R-28, D29, F-13).
    return Ok(Timestamp::new(instant));
  }
  // R-22: an offsetless instant is the most likely backend mistake, and "you
  // omitted the offset" is debuggable where "unparseable" is not (brief §13).
  if raw.parse::<jiff::civil::DateTime>().is_ok() {
    return Err(ScheduleError::MissingOffset {
      raw: raw.to_owned(),
    });
  }
  let Ok(span) = raw.parse::<jiff::Span>() else {
    return Err(ScheduleError::Unparseable {
      raw: raw.to_owned(),
    });
  };
  // R-23/R-24: days and weeks resolve as exactly 24 and 168 hours; months and
  // years do not resolve at all, because their length needs a calendar and D4
  // leaves jiff without one. This conversion failing *is* the calendar-unit
  // case — there is no other way for it to fail.
  let Ok(duration) = span.to_duration(jiff::SpanRelativeTo::days_are_24_hours()) else {
    return Err(ScheduleError::CalendarUnit {
      raw: raw.to_owned(),
    });
  };
  // A span can be well-formed and still land outside representable time. That
  // is a different boundary from the per-unit bound the span parser enforces,
  // and the two report differently.
  match now.instant().checked_add(duration) {
    Ok(instant) => Ok(Timestamp::new(instant)),
    Err(_) => Err(ScheduleError::OutOfRange {
      raw: raw.to_owned(),
    }),
  }
}

/// Brief §9's three arms, in one place: the latest **valid** instruction, else
/// the retained value, else `now + default_poll` (R-26). The result is always a
/// concrete instant — there is no unresolved state (R-27).
///
/// `incoming` is `Option`, not `Result`, because an invalid `next_check` never
/// reaches here: normalization has already turned it into `None` plus a
/// `Discarded::Schedule` (P2). So `None` means "no usable instruction supplied"
/// and the retained value stands — which is R-25's "preserves rather than
/// disables", expressed as a type rather than as a comment.
///
/// Latest-valid-wins is **issue order, not `max`**. A valid `incoming` wins even
/// when it is earlier than `retained`: brief §9 and §22's point 8 say a later
/// instruction supersedes an earlier one, and a backend asking to be checked
/// sooner is exactly that (R-28). `max(retained, incoming)` passes the obvious
/// cases and silently overrides this one.
///
/// `retained` is `Option` so that seeding goes through this function too. The
/// host's resolved check is not itself an `Option` (`design.md` §5.3) — it is
/// seeded at construction, and seeding is the `(None, None)` arm, so `now +
/// default_poll` is written once here instead of a second time in stratum 2.
///
/// `default_poll` is a `jiff::SignedDuration` rather than a `std::time::
/// Duration` because stratum 1 is jiff-native throughout; converting at the
/// config boundary keeps a fallible conversion out of a function that must be
/// total. It is the configured poll interval, which is rejected at load unless
/// it is positive. At the very edge of representable time the addition
/// saturates rather than failing, because R-27 admits no unresolved answer.
pub fn resolve(
  retained: Option<Timestamp>,
  incoming: Option<Timestamp>,
  default_poll: jiff::SignedDuration,
  now: Timestamp,
) -> Timestamp {
  match (incoming, retained) {
    (Some(instruction), _) => instruction,
    (None, Some(resolved)) => resolved,
    (None, None) => Timestamp::new(
      now
        .instant()
        .checked_add(default_poll)
        .unwrap_or(jiff::Timestamp::MAX),
    ),
  }
}

#[cfg(test)]
mod tests {
  use crate::semantics::protocol::canonical::Timestamp;
  use crate::semantics::schedule::{parse, resolve};

  fn instant(rfc3339: &str) -> Timestamp {
    Timestamp::new(rfc3339.parse().unwrap())
  }

  fn now() -> Timestamp {
    instant("2026-08-23T04:12:00Z")
  }

  // VT-1 and VT-3 are not here. They are parse cases with a JSON input and one
  // expected outcome each, so they are fixtures — `design.md` §9, and
  // `tests/protocol/fixtures/schedule/`. They were written here first, as the
  // red step that drove each `ScheduleError` variant into existence, and
  // deleted once the corpus stated the same contract: two statements of one
  // rule is how a repair gets left standing at half the places that state it.
  //
  // VT-2 stays, and belongs here. Resolution takes four typed arguments, two of
  // them `Option` and one a `SignedDuration`, and encoding those as JSON buys a
  // reader of the protocol nothing.

  // ---- VT-2: resolution over (retained, incoming, default, now) ----

  const HOUR: jiff::SignedDuration = jiff::SignedDuration::from_hours(1);

  #[test]
  fn a_valid_incoming_instruction_supersedes_the_retained_one() {
    let resolved = resolve(
      Some(instant("2026-08-23T06:00:00Z")),
      Some(instant("2026-08-23T09:00:00Z")),
      HOUR,
      now(),
    );
    assert_eq!(resolved, instant("2026-08-23T09:00:00Z"));
  }

  /// Latest-valid-wins is issue order, not `max`. This is the case that tells
  /// the two apart, and the only one that does: a backend asking to be checked
  /// *sooner* is still the later instruction (brief §22.8, R-26, R-28).
  #[test]
  fn a_valid_incoming_instruction_wins_even_when_it_is_earlier_than_the_retained_one() {
    let resolved = resolve(
      Some(instant("2026-08-23T09:00:00Z")),
      Some(instant("2026-08-23T06:00:00Z")),
      HOUR,
      now(),
    );
    assert_eq!(resolved, instant("2026-08-23T06:00:00Z"));
  }

  #[test]
  fn with_no_incoming_instruction_the_retained_value_stands() {
    let resolved = resolve(Some(instant("2026-08-23T09:00:00Z")), None, HOUR, now());
    assert_eq!(resolved, instant("2026-08-23T09:00:00Z"));
  }

  /// The seeding arm. `Host::new` reaches it with both sides absent, which is
  /// why `retained` is an `Option` at all (`design.md` §5.3, R-27).
  #[test]
  fn with_nothing_retained_and_nothing_incoming_the_default_poll_is_added_to_now() {
    let resolved = resolve(None, None, HOUR, now());
    assert_eq!(resolved, instant("2026-08-23T05:12:00Z"));
  }

  /// R-25's "preserves rather than disables", at this layer. An invalid
  /// `next_check` never arrives as an error here — normalization has already
  /// discarded it, so it arrives as `incoming: None` — and the retained
  /// schedule is therefore untouched.
  #[test]
  fn an_instruction_discarded_upstream_arrives_as_none_and_preserves_the_retained_value() {
    let discarded_upstream = parse(&serde_json::json!(45), now());
    assert!(discarded_upstream.is_err());
    let incoming = discarded_upstream.ok();

    let resolved = resolve(Some(instant("2026-08-23T09:00:00Z")), incoming, HOUR, now());
    assert_eq!(resolved, instant("2026-08-23T09:00:00Z"));
  }
}

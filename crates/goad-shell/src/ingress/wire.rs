//! The reply, as bytes — `SPEC-003` §6.3.
//!
//! One definition, read by both halves of the exchange: the listener writes it
//! and a client reads it. Two structs for one wire object drift, and nothing
//! fails when they do (005/D-2).

use serde::{Deserialize, Serialize};

/// The five fields §6.3 admits, in the order they are written.
///
/// **No `deny_unknown_fields`.** A field this host does not model is not an
/// error: CLAUDE.md's second invariant makes parsing permissive and refuses
/// to narrow the wire to what one reader happens to implement. A sixth field
/// from a newer host must not turn a perfectly good verdict into a fault.
///
/// `protocol` and `accepted` are `Option` **on the read side only**: the
/// listener sets both on every reply, and the bytes are unchanged by their
/// being optional here (asserted in `mod.rs`'s
/// `the_reply_s_bytes_are_exactly_these`). Requiring them of a reader would
/// narrow what a client accepts for a field it does not use, which is the
/// same invariant again (005/D-11). Absence of `accepted` is then a *named
/// breach* of §6.3 for the reader to report — not a parse failure, and not a
/// guess.
///
/// No field carries `#[serde(default)]`: serde already reads a missing
/// `Option` field as `None`, so all five are absent-tolerant as written
/// (`an_empty_object_parses_and_carries_nothing` below is what holds that).
/// The three `skip_serializing_if`s are serialize-side only — they are why an
/// accepted reply is two fields rather than five.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Reply {
  pub protocol: Option<u8>,
  pub accepted: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reason: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub retry_after_ms: Option<u64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub detail: Option<String>,
}

#[cfg(test)]
mod tests {
  use super::Reply;

  fn parsed(text: &str) -> Reply {
    serde_json::from_str(text).unwrap_or_else(|error| panic!("{text} must parse: {error}"))
  }

  /// 005/PHASE-01/VT-2. What this asserts is only that `{}` **deserializes**;
  /// whether it is a *usable* answer is §6.3's question, and PHASE-02's
  /// `read_reply` is what answers it (`NonConforming`, not `Unreadable`).
  #[test]
  fn an_empty_object_parses_and_carries_nothing() {
    assert_eq!(
      parsed("{}"),
      Reply {
        protocol: None,
        accepted: None,
        reason: None,
        retry_after_ms: None,
        detail: None,
      }
    );
  }

  #[test]
  fn a_reply_without_protocol_parses() {
    assert_eq!(parsed(r#"{"accepted":true}"#).accepted, Some(true));
  }

  #[test]
  fn an_unknown_field_is_not_a_fault() {
    let reply = parsed(r#"{"protocol":2,"accepted":false,"reason":"too_soon","severity":"warn"}"#);
    assert_eq!(reply.protocol, Some(2));
    assert_eq!(reply.reason.as_deref(), Some("too_soon"));
  }

  /// The absent optionals are absent from the bytes, not written as `null` —
  /// `skip_serializing_if` — while `protocol` and `accepted` are written
  /// whatever they hold.
  #[test]
  fn writing_and_reading_are_the_same_shape() {
    let sent = Reply {
      protocol: Some(1),
      accepted: Some(false),
      reason: Some("engaged".to_owned()),
      retry_after_ms: None,
      detail: Some("an exchange was already in flight".to_owned()),
    };
    let text = serde_json::to_string(&sent).expect("a reply of primitives serializes");
    assert!(!text.contains("retry_after_ms"), "{text}");
    assert_eq!(parsed(&text), sent);
  }
}

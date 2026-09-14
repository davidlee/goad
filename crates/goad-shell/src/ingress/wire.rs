//! The reply, as bytes — `SPEC-003` §6.3.
//!
//! One definition, read by both halves of the exchange: the listener writes it
//! and a client reads it. Two structs for one wire object drift, and nothing
//! fails when they do (005/D-2).

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
/// **Every field is a `Value`, and none of them can fail to parse**
/// (`review-code.md` F-1, F-11). Two rules were in play and they pull against
/// each other; this is the one that wins, and why.
///
/// The loser was *narrow only as far as JSON's own types do* — true of `bool`
/// for `accepted` and `String` for `reason`, and false of `u8` and `u64` for
/// two JSON *numbers*, since JSON has one number type and `1`, `1.0` and `1e0`
/// are one value. Fixing only the numbers left the other three able to fail
/// at the parse step, where a wrong type is reported as *"not one JSON
/// object"* about a document that plainly is one. The winner is therefore:
/// **a field whose value a reader adjudicates must be typed so that its parse
/// cannot fail**, or its wrongness is reported one step too early, in a
/// sentence that is false of it. A `Value` field is what *lets* a breach be
/// named; the naming itself is
/// [`client::read_reply`](super::client::read_reply)'s, which checks all five
/// against §6.3 and says which rule each one breaks.
///
/// **What that costs the write side, and what holds it.** `Reply` is
/// deliberately bidirectional (005/D-2) — one definition, because two drift
/// and nothing fails when they do — so a type the reader gave up is a type
/// the *writer* gave up too: `protocol: Some(json!("1"))` used to be
/// `error[E0308]` and now compiles (F-13). **Three** things hold the write
/// side now, and the third is the honest one (F-16):
///
/// 1. [`Reply::written`] is the only production constructor, and it takes
///    `bool`, `u64` and `&str`. The host cannot misspell a field without
///    going around it, and `protocol` it cannot set at all — that is
///    [`VERSION`], not an argument.
/// 2. `#[non_exhaustive]` makes the struct literal unavailable **outside**
///    `goad-shell`, so from `goad` and `goad-emit` [`Reply::written`] is not
///    merely the only constructor used, it is the only one there is. The
///    fields stay `pub` because the *reader* needs them.
/// 3. Inside `goad-shell` the literal is still available — the tests below
///    and `ingress::what_the_listener_writes_parses_back_to_what_it_built`
///    use it — so in-crate this is **one writer by convention and review**,
///    not by the type. What is asserted is that one writer's *output*:
///    `ingress::the_reply_s_bytes_are_exactly_these` pins the bytes
///    [`Reply::written`]'s arguments become, byte for byte, for an acceptance
///    and a refusal. A second in-crate writer built by literal would be
///    asserted by nothing, which is why there is not one.
///
/// Anyone changing these field types changes [`Reply::written`]'s signature in
/// the same breath.
///
/// No field carries `#[serde(default)]`: serde already reads a missing
/// `Option` field as `None`, so all five are absent-tolerant as written
/// (`an_empty_object_parses_and_carries_nothing` below is what holds that),
/// and a JSON `null` reads as `None` for the same reason. The three
/// `skip_serializing_if`s are serialize-side only — they are why an accepted
/// reply is two fields rather than five.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Reply {
  pub protocol: Option<Value>,
  pub accepted: Option<Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reason: Option<Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub retry_after_ms: Option<Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub detail: Option<Value>,
}

/// The version this host writes on every reply — `SPEC-003` §6.3's
/// *"this contract's version, not SPEC-001's"*. There is one, and
/// [`Reply::written`] is why no caller can write another.
pub const VERSION: u8 = 1;

impl Reply {
  /// The host's own reply, from typed parts. **The only thing that builds a
  /// `Reply` in production**, and the answer to what holds the write side now
  /// that the fields are `Value` (F-13).
  ///
  /// `retry_after_ms` is `u64` because that is what the host has — the
  /// rounding `SPEC-003/R-14` requires happens before this call, in
  /// `ingress::reply`, where the `Duration` still exists to round.
  #[must_use]
  pub fn written(
    accepted: bool,
    reason: Option<&str>,
    retry_after_ms: Option<u64>,
    detail: Option<&str>,
  ) -> Self {
    Self {
      protocol: Some(Value::from(VERSION)),
      accepted: Some(Value::from(accepted)),
      reason: reason.map(Value::from),
      retry_after_ms: retry_after_ms.map(Value::from),
      detail: detail.map(Value::from),
    }
  }
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
    assert_eq!(
      parsed(r#"{"accepted":true}"#).accepted,
      Some(serde_json::json!(true))
    );
  }

  #[test]
  fn an_unknown_field_is_not_a_fault() {
    let reply = parsed(r#"{"protocol":2,"accepted":false,"reason":"too_soon","severity":"warn"}"#);
    assert_eq!(reply.protocol, Some(serde_json::json!(2)));
    assert_eq!(reply.reason, Some(serde_json::json!("too_soon")));
  }

  /// The absent optionals are absent from the bytes, not written as `null` —
  /// `skip_serializing_if` — while `protocol` and `accepted` are written on
  /// every reply. Built through [`Reply::written`] because that is the only
  /// thing that builds one in production (F-13), so the round trip under test
  /// is the host's own.
  #[test]
  fn writing_and_reading_are_the_same_shape() {
    let sent = Reply::written(
      false,
      Some("engaged"),
      None,
      Some("an exchange was already in flight"),
    );
    let text = serde_json::to_string(&sent).expect("a reply of primitives serializes");
    assert!(!text.contains("retry_after_ms"), "{text}");
    assert_eq!(parsed(&text), sent);
  }
}

//! The client half of `SPEC-003` — one envelope out, one verdict back.
//!
//! It sits beside the listener rather than inside the binary that calls it
//! (005/D-1): the reply is half a contract stratum 2 owns, and a client here
//! is within reach of the **real** listener in one process, which is what
//! `tests/integration/ingress.rs` uses it for.
//!
//! **Blocking, and deliberately unbounded.** `SPEC-003` §6.4 says a host not
//! making progress delays an arrival as it delays a due check, and the writer
//! waits with it rather than being told something untrue. One `UnixStream`
//! needs no runtime, so nothing here is `async` and nothing here names
//! `tokio`; a caller wanting a deadline wraps the process (`timeout 5 …`).
//!
//! The split in this module is what makes §6.3 testable without a socket:
//! [`read_reply`] is pure and public, and [`send`] is the only part that
//! touches the transport.

use std::io::{BufRead as _, BufReader, Write as _};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::Path;

use goad_semantics::protocol::canonical::Event;

use super::wire::Reply;

/// What the host said about one envelope.
///
/// `Answered`, not `Verdict`: the integration tier's fake judge already owns
/// that name for the other end of this exchange (`design.md` §5.2).
///
/// `PartialEq` because a case asserts the whole value — a refusal is its
/// token, its advice and its detail together, and three field assertions say
/// less than one.
#[derive(Debug, PartialEq)]
pub enum Answered {
  /// The host took the envelope.
  Accepted,
  /// The host refused it, and said why.
  ///
  /// `reason` is carried **verbatim**, known token or not: the set is closed
  /// at eight today (`SPEC-003/R-14`), and a ninth from a newer host is a
  /// refusal a client can report without understanding it. `retry_after_ms`
  /// is advice and is reported, never obeyed (`SPEC-003` §6.3).
  Refused {
    reason: String,
    retry_after_ms: Option<u64>,
    detail: Option<String>,
  },
}

/// Why no verdict was obtained. Every variant is *the client's* account of
/// what went wrong, and each names which side was at fault.
///
/// No `PartialEq`: two of the five carry an error type that has none. Cases
/// match on the variant.
#[derive(Debug)]
pub enum SendFault {
  /// Nothing is listening at the path, or the connection was refused. The
  /// host is not running, or is not running where the caller looked.
  Unreachable(std::io::Error),
  /// The connection broke after it was made.
  Faulted(std::io::Error),
  /// The host closed with nothing on the connection. `SPEC-003/R-8` makes
  /// one reply per envelope the contract, so silence is the host's breach and
  /// not a verdict to guess at.
  NoReply,
  /// Bytes arrived that are not one JSON object: `[1,2]`, `not json`.
  Unreadable(serde_json::Error),
  /// One JSON object arrived and breached `SPEC-003` §6.3. The string says
  /// which rule — this is the host's fault, reported as the host's.
  NonConforming(&'static str),
}

/// Send one envelope and wait for the one reply.
///
/// Connect, write the event's own serialization and the `\n` terminator
/// (`SPEC-003/R-6`), shut the write half, then read to the first newline or
/// EOF.
///
/// **Both terminators are written, and each would do alone** — measured
/// against this host, which admits either (the VA-1 note on
/// `a_sent_envelope_is_accepted_and_reaches_the_judge_as_the_event_it_was`).
/// That is deliberate rather than redundant: R-6 admits both, so writing both
/// works against a host that reads to a newline *and* against one that reads
/// to EOF, while writing one narrows the client to hosts implementing that
/// one. It is CLAUDE.md's permissive-wire invariant pointed at the writing
/// side — do not narrow what this sends to what the current listener happens
/// to accept.
///
/// The envelope is [`Event`]'s own `Serialize` output and no struct of this
/// module's (005/D-3): `Event` is already exactly §6.2's four fields, and a
/// second definition of one wire object drifts.
///
/// **This call has no deadline** — see the module doc and `SPEC-003` §6.4.
///
/// # Errors
///
/// A [`SendFault`] naming what went wrong and whose fault it was: the path
/// was unreachable, the connection faulted, the host said nothing, or the
/// bytes it did say were not a verdict.
pub fn send(socket: &Path, event: &Event) -> Result<Answered, SendFault> {
  let envelope = envelope_line(event);

  let mut stream = UnixStream::connect(socket).map_err(SendFault::Unreachable)?;
  stream
    .write_all(envelope.as_bytes())
    .map_err(SendFault::Faulted)?;
  stream.write_all(b"\n").map_err(SendFault::Faulted)?;
  stream
    .shutdown(Shutdown::Write)
    .map_err(SendFault::Faulted)?;

  let mut line = String::new();
  let read = BufReader::new(&stream)
    .read_line(&mut line)
    .map_err(SendFault::Faulted)?;
  if read == 0 {
    return Err(SendFault::NoReply);
  }
  answer(&line)
}

/// The envelope's bytes — `SPEC-003` §6.2, which [`Event`] already is.
///
/// Returns a `String` rather than a `Result` for the reason `ingress::reply`
/// does on the other side of the same exchange: an [`Event`] of two strings, a
/// timestamp and a `serde_json::Value` serializes infallibly, so a failure
/// here is a defect in this module and not a caller's mistake. Keeping it out
/// of [`send`] is not a way around `clippy::unwrap_in_result` — it is what
/// makes the claim local: this function is the whole of what is claimed to be
/// infallible, and it takes no path that could make it false.
fn envelope_line(event: &Event) -> String {
  #[expect(
    clippy::unwrap_used,
    reason = "an Event of primitives and a serde_json::Value serializes infallibly (reply()'s \
              own precedent for this argument)"
  )]
  serde_json::to_string(event).unwrap()
}

/// The reply's bytes, turned into a verdict.
///
/// Private and separate from [`send`] only so that [`SendFault::Unreadable`]
/// — a property of *bytes*, which [`read_reply`] never sees — is reachable
/// without a socket.
fn answer(line: &str) -> Result<Answered, SendFault> {
  let reply: Reply = serde_json::from_str(line).map_err(SendFault::Unreadable)?;
  read_reply(reply)
}

/// `SPEC-003` §6.3, applied to one parsed reply. Pure: every conformance rule
/// below is testable with no socket, which is the point of the split.
///
/// `protocol` is not read at all. Requiring it would narrow what a client
/// accepts from a conforming-enough host for a field it does not use
/// (CLAUDE.md's permissive-wire invariant, 005/D-11); the host writes it on
/// every reply regardless.
///
/// # Errors
///
/// [`SendFault::NonConforming`] for the two shapes §6.3 does not admit: a
/// reply carrying no `accepted`, and a refusal carrying no `reason`. Both are
/// the host's breach. Neither can be reported as a refusal, because a caller
/// branching on the reason token would have nothing to branch on — and
/// reporting *refused* without one would be the untruth (`design.md` §5.5).
pub fn read_reply(reply: Reply) -> Result<Answered, SendFault> {
  match (reply.accepted, reply.reason) {
    (None, _no_verdict) => Err(SendFault::NonConforming(
      "the reply carries no `accepted` field",
    )),
    (Some(true), _reason_is_not_read_on_an_acceptance) => Ok(Answered::Accepted),
    (Some(false), None) => Err(SendFault::NonConforming(
      "the refusal carries no `reason` token",
    )),
    (Some(false), Some(reason)) => Ok(Answered::Refused {
      reason,
      retry_after_ms: reply.retry_after_ms,
      detail: reply.detail,
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::{Answered, SendFault, answer, read_reply};
  use crate::ingress::wire::Reply;

  /// 005/PHASE-02/VT-5 throughout: every case here is one `SPEC-003` §6.3
  /// rule over one reply shape, with no socket anywhere.
  fn verdict_of(text: &str) -> Result<Answered, SendFault> {
    let reply: Reply =
      serde_json::from_str(text).unwrap_or_else(|error| panic!("{text} must parse: {error}"));
    read_reply(reply)
  }

  fn breach_of(text: &str) -> SendFault {
    verdict_of(text).expect_err("this reply breaches §6.3")
  }

  #[test]
  fn a_reply_that_says_accepted_is_an_acceptance() {
    assert_eq!(
      verdict_of(r#"{"protocol":1,"accepted":true}"#).expect("an accepted reply is a verdict"),
      Answered::Accepted
    );
  }

  #[test]
  fn protocol_is_not_required_of_a_reply() {
    assert_eq!(
      verdict_of(r#"{"accepted":true}"#).expect("protocol is not read"),
      Answered::Accepted
    );
  }

  /// `{}` parses — every field of `Reply` is `Option` — and is then a named
  /// breach rather than a parse failure (`design.md` §5.5).
  #[test]
  fn an_empty_object_is_a_named_breach_not_an_acceptance() {
    let fault = breach_of("{}");
    assert!(matches!(fault, SendFault::NonConforming(_)), "{fault:?}");
  }

  #[test]
  fn a_refusal_with_no_reason_is_a_named_breach() {
    let fault = breach_of(r#"{"protocol":1,"accepted":false}"#);
    assert!(matches!(fault, SendFault::NonConforming(_)), "{fault:?}");
  }

  #[test]
  fn an_unknown_reason_token_is_carried_verbatim() {
    assert_eq!(
      verdict_of(r#"{"protocol":1,"accepted":false,"reason":"a_ninth_reason"}"#)
        .expect("a token this client does not know is still a refusal"),
      Answered::Refused {
        reason: "a_ninth_reason".to_owned(),
        retry_after_ms: None,
        detail: None,
      }
    );
  }

  #[test]
  fn retry_after_ms_and_detail_are_carried_as_given() {
    let text = r#"{"protocol":1,"accepted":false,"reason":"too_soon","retry_after_ms":1500,"detail":"wait"}"#;
    assert_eq!(
      verdict_of(text).expect("a refusal is a verdict"),
      Answered::Refused {
        reason: "too_soon".to_owned(),
        retry_after_ms: Some(1500),
        detail: Some("wait".to_owned()),
      }
    );
  }

  #[test]
  fn an_unknown_field_beside_the_five_is_not_a_fault() {
    assert_eq!(
      verdict_of(r#"{"accepted":true,"severity":"warn"}"#)
        .expect("an unmodelled field is not an error"),
      Answered::Accepted
    );
  }

  /// `Unreadable` is about bytes, which `read_reply` never sees — so these
  /// two go through `answer`, the one step between the wire and the rule.
  #[test]
  fn bytes_that_are_not_one_json_object_are_unreadable() {
    for text in ["[1,2]", "not json"] {
      let fault = answer(text).expect_err("these bytes are not one JSON object");
      assert!(
        matches!(fault, SendFault::Unreadable(_)),
        "{text}: {fault:?}"
      );
    }
  }
}

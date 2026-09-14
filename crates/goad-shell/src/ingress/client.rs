//! The client half of `SPEC-003` — one envelope out, one verdict back.
//!
//! It sits beside the listener rather than inside the binary that calls it
//! (005/D-1): the reply is half a contract stratum 2 owns, and a client here
//! is within reach of the **real** listener in one process, which is what
//! `tests/integration/ingress.rs` uses it for.
//!
//! **Blocking, and deliberately unbounded in time.** `SPEC-003` §6.4 says a
//! host not making progress delays an arrival as it delays a due check, and
//! the writer waits with it rather than being told something untrue. One
//! `UnixStream` needs no runtime, so nothing here is `async` and nothing here
//! names `tokio`; a caller wanting a deadline wraps the process
//! (`timeout 5 …`). It is **not** unbounded in bytes — see [`REPLY_LIMIT`].
//!
//! The split in this module is what makes §6.3 testable without a socket:
//! [`read_reply`] is pure and public, and [`send`] is the only part that
//! touches the transport.
//!
//! # The reply path decides from meaning, not from encoding
//!
//! The reply travels three steps, and **each step owns its own fault**, which
//! is what lets every refusal name the side that was wrong (`review-code.md`
//! F-1, F-3, F-4, F-5, F-11):
//!
//! ```text
//!   the connection ──▶ one line of bytes ──▶ a Reply ──▶ an Answered
//!                    │                     │           │
//!         Unreachable / Faulted        Unreadable   NonConforming
//!         Oversized / NoReply
//! ```
//!
//! [`reply_line`] bounds and frames; nothing there looks at what the bytes
//! say. `serde_json` decides whether they are one JSON object. [`read_reply`]
//! then decides what the object **means**, and every disagreement it finds —
//! a field of the wrong JSON type included — is a named breach of a numbered
//! rule, never a parse failure.
//!
//! The rule that keeps the steps from bleeding into each other is [`Reply`]'s:
//! **a field whose value this client adjudicates is typed so that its parse
//! cannot fail**, or its wrongness is reported one step too early, as bytes,
//! in a sentence that is false of them. All five are.
//!
//! # Where that stops
//!
//! `SPEC-003` §3 P-D: *a clause that cannot say where its exception would be
//! is a clause that has not been checked.* What follows is a **scope**, not a
//! list. A list here was completed three times and was wrong three times, each
//! time because it was built by cross-producting shapes someone had thought
//! of — a method that can only ever confirm the shapes already thought of
//! (`review-code.md` F-17).
//!
//! **The scope.** Every document that is **grammatically one JSON object** and
//! is nevertheless refused as not being one is refused over a fault in one of
//! the five fields [`Reply`] declares. An unmodelled field's value is
//! *skipped* rather than built, so every limit that comes of building one — a
//! number's range, a `\u` escape's code point, nesting depth, a name seen
//! twice — cannot arise there. What is left for a skipped field is finding
//! where its value ends, which is the grammar itself: so an unmodelled field
//! can cost a verdict only when the bytes are **not JSON at all**, and there
//! *"not one JSON object"* is true of them.
//!
//! That holds because of a property of the code — [`answer`] runs the
//! [`Reply`] parse first, and `serde_json` skips an unknown field without
//! constructing a value — and not because of an enumeration, so it does not go
//! stale when someone fails to imagine a shape.
//!
//! **Shapes measured so far**, as illustration and explicitly not as the
//! claim. Each is one JSON object; each is `Unreadable` in a declared field
//! and costs nothing in an unmodelled one:
//!
//! - nesting past `serde_json`'s recursion limit of 128 levels;
//! - a number outside the range it represents — `1e999`, or a 400-digit
//!   integer. RFC 8259 §6 lets a parser limit number range, so this is that
//!   parser's limit and not a defect here; closing it needs
//!   `arbitrary_precision`, which changes `Number` for every crate in the
//!   workspace. Raised, and **declined**;
//! - a lone surrogate, `"\ud800"` — §7 admits `\u` and four hex digits without
//!   requiring a pair, and §8.2 leaves the rest to the implementation;
//! - a duplicated *declared* name. §4 makes the meaning of one unpredictable,
//!   and this one is ambiguous about the field the verdict turns on, so
//!   failing is right; only the sentence is coarse.
//!
//! **One asymmetry, recorded because a reader tracing the boundary meets it.**
//! Raw invalid UTF-8 inside a string is `Unreadable` in a declared field and
//! *accepted* in an unmodelled one, so the skip is more lenient than the
//! grammar there (§8.1 requires UTF-8). That widens what is accepted and never
//! narrows it, so the scope above still holds.
//!
//! All of this is exit 2 either way with the host at fault, so what the
//! coarseness costs is the precision of one sentence and not a caller's
//! decision. The scope's two halves are pinned by
//! `a_declared_field_serde_json_will_not_build_a_value_for_is_unreadable` and
//! `an_unmodelled_field_that_is_still_json_cannot_cost_a_verdict`, and
//! `bytes_that_are_not_one_json_object_are_unreadable` pins the case where the
//! sentence is simply true wherever the bytes sit.

use std::io::{BufRead as _, BufReader, Read as _, Write as _};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::Path;

use goad_semantics::protocol::canonical::Event;
use serde_json::Value;

use super::wire::Reply;

/// Bounds the bytes one **reply** may consume, and is the same 64 KiB
/// [`ENVELOPE_LIMIT`](super::ENVELOPE_LIMIT) bounds the host's read of an
/// envelope at — `SPEC-003` §6.4's `bytes per read | 64 KiB` row, whose
/// stated reason is that *"an unbounded read from an untrusted writer is the
/// defect `SPEC-001/R-43` names on the other socket"*. §6.4 names the host's
/// direction; the defect is the read, not the direction, so the number is
/// aliased rather than restated and the exchange is bounded symmetrically
/// (`review-code.md` F-3).
///
/// A host that never terminates a reply is a host bug, and the answer to it
/// is [`SendFault::Oversized`] — a refusal naming the host — rather than an
/// allocation that runs until the OOM killer turns a caller's exit 2 into a
/// signal.
pub const REPLY_LIMIT: usize = super::ENVELOPE_LIMIT;

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
/// No `PartialEq`: two of the six carry an error type that has none. Cases
/// match on the variant.
#[derive(Debug)]
pub enum SendFault {
  /// Nothing is listening at the path, or the connection was refused. The
  /// host is not running, or is not running where the caller looked.
  Unreachable(std::io::Error),
  /// The connection broke after it was made. **The transport's fault, and
  /// only the transport's**: a fault of the host's *bytes* is `Unreadable`,
  /// and the two are told apart by reading the reply as bytes rather than
  /// demanding text of it (`review-code.md` F-5).
  Faulted(std::io::Error),
  /// The host closed with nothing on the connection. `SPEC-003/R-8` makes
  /// one reply per envelope the contract, so silence is the host's breach and
  /// not a verdict to guess at.
  NoReply,
  /// The host wrote more than [`REPLY_LIMIT`] bytes without ending the reply.
  /// The host's breach: `SPEC-003` §6.3's reply is one line, and this one did
  /// not end inside the bound §6.4 states.
  Oversized { limit: usize },
  /// Bytes arrived that are not one JSON object: `[1,2]`, `not json`, or
  /// bytes that are not UTF-8 — JSON text *is* UTF-8, so those are the same
  /// fault and name the same side, the host's serializer.
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
/// It does have a byte bound: [`REPLY_LIMIT`].
///
/// # Errors
///
/// A [`SendFault`] naming what went wrong and whose fault it was: the path
/// was unreachable, the connection faulted, the host said nothing, said more
/// than the bound, or the bytes it did say were not a verdict.
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

  answer(&reply_line(&stream)?)
}

/// One reply: until the first newline or end of input, whichever comes first,
/// bounded in bytes at [`REPLY_LIMIT`]. Returns the line **without** its
/// terminator.
///
/// **Bytes, not text.** A `String` here would make the reader demand UTF-8
/// and report the host's mis-encoded bytes as an `io::ErrorKind::InvalidData`
/// — a *connection* fault, pointing a caller at a socket that did its job
/// (`review-code.md` F-5). JSON text is UTF-8 by definition, so the encoding
/// is `serde_json`'s question and is answered one step later, where the
/// answer names the host.
///
/// The framing and the byte bound are one read: the stream is wrapped in
/// `take(REPLY_LIMIT + 1)` before `read_until` runs, so a reply that reaches
/// the cap without a newline is distinguishable from one that ends exactly at
/// it — `read_envelope`'s own `+ 1` idiom, mirrored here because this is the
/// mirror of that read.
fn reply_line(stream: &UnixStream) -> Result<Vec<u8>, SendFault> {
  let cap = u64::try_from(REPLY_LIMIT)
    .unwrap_or(u64::MAX)
    .saturating_add(1);
  let mut line = Vec::new();
  BufReader::new(stream.take(cap))
    .read_until(b'\n', &mut line)
    .map_err(SendFault::Faulted)?;

  if line.is_empty() {
    return Err(SendFault::NoReply);
  }
  if line.last() == Some(&b'\n') {
    line.pop();
  } else if line.len() > REPLY_LIMIT {
    return Err(SendFault::Oversized { limit: REPLY_LIMIT });
  }
  Ok(line)
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
/// without a socket. This is the **one** place that fault can arise.
///
/// **Two questions, and the order between them is load-bearing.** The fields
/// are read first, and only then is the shape settled by [`one_json_object`].
///
/// Reading the fields does not answer the shape question on its own: serde's
/// derive accepts a struct in *sequence* form, so `[1,2]` deserializes into a
/// `Reply` of `protocol: 1, accepted: 2` once every field is a `Value` that
/// cannot refuse it — an array reported as a §6.3 breach instead of as bytes
/// that are not one JSON object.
///
/// But asking the shape question *first*, by parsing every value in the
/// document, made the guard **stricter than the parse it guards**
/// (`review-code.md` F-15(iii)): a value in a field `Reply` skips — nested
/// past `serde_json`'s recursion limit, or a number outside its range — was
/// refused by the guard and accepted by the parse, and a caller whose
/// envelope the host had taken was told no usable answer could be had. That
/// is `wire.rs`'s *"a sixth field from a newer host must not turn a perfectly
/// good verdict into a fault"* failing in the one direction it is written to
/// prevent. The field parse skips an unknown field without building a value
/// for it, so running it first makes it the strictest thing in the path, and
/// the shape question is then answered without parsing anything twice.
fn answer(line: &[u8]) -> Result<Answered, SendFault> {
  let reply: Reply = serde_json::from_slice(line).map_err(SendFault::Unreadable)?;
  one_json_object(line)?;
  read_reply(reply)
}

/// Is a line that has **already parsed as a [`Reply`]** one JSON object?
///
/// After that parse succeeds the document can only be an object or an array —
/// every other JSON value is an `invalid type` error against a struct — so
/// the first byte that is not JSON whitespace settles it: `{` is the object
/// and anything else is the sequence form. The four bytes skipped are
/// RFC 8259 §2's whitespace, which is what makes the fast path hit for every
/// conforming reply.
///
/// **Which half of this is load-bearing, exactly.** The byte answers only
/// *yes*; every *no* goes to the `Map` read. What the byte carries is the
/// **acceptance** path, and it carries the F-15(iii) fix with it: a reply
/// whose unmodelled field the `Map` read would choke on is accepted here
/// without that read ever running.
///
/// The `Map` read is where the fault's sentence comes from — the only way to
/// obtain the `serde_json::Error` [`SendFault::Unreadable`] carries.
///
/// **Its `Ok` arm is unreachable today, deliberately kept, and untestable**
/// (F-18). `serde_json` accepts exactly `0x20`, `0x09`, `0x0A` and `0x0D`
/// before a value and this function skips exactly those four, so the two
/// whitespace sets coincide: a parse that has already succeeded leaves `{` for
/// every object and `[` for every array and nothing else, `{` takes the fast
/// path, and the read therefore runs only for arrays — where it always errors.
/// No case reaches that arm and none can write one.
///
/// It stays because it insures against the two sets **diverging**: if
/// `serde_json`'s whitespace ever widens or this one narrows, the byte stops
/// being exact, and the `Map` read is what makes that cost one extra parse
/// instead of one wrong refusal. Read it as a bound on a future defect rather
/// than as a path, and do not delete it for want of coverage.
///
/// # Errors
///
/// [`SendFault::Unreadable`], carrying `serde_json`'s own account of why the
/// document is not one JSON object.
fn one_json_object(line: &[u8]) -> Result<(), SendFault> {
  let first = line
    .iter()
    .find(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r'));
  if first == Some(&b'{') {
    return Ok(());
  }
  match serde_json::from_slice::<serde_json::Map<String, Value>>(line) {
    Ok(_an_object_after_all) => Ok(()),
    Err(error) => Err(SendFault::Unreadable(error)),
  }
}

/// `SPEC-003` §6.3, applied to one parsed reply. Pure: every conformance rule
/// below is testable with no socket, which is the point of the split.
///
/// `protocol` is not read at all — not its presence and not its value
/// (`review-code.md` F-1). Reading it would narrow what a client accepts from
/// a conforming-enough host for a field it does not use (CLAUDE.md's
/// permissive-wire invariant, 005/D-11); the host writes it on every reply
/// regardless, and the destructure below names the field `_unread` so that is
/// visible at the one place it could stop being true.
///
/// **What that concedes** (F-12): a host that one day writes `protocol: 2`
/// gets §6.3 *version 1* semantics applied to its reply, silently, at both
/// ends. That is the right trade today and there is nothing here to do about
/// it — this contract has one version and no negotiation step, so reading the
/// field could only ever refuse a reply this client understands. Closing it is
/// `SPEC-003`'s job, not a client's.
///
/// # Two rules, in order
///
/// **Types first.** Every field §6.3 gives a type is checked against it, and a
/// wrong one is a named breach wherever it appears — including
/// `retry_after_ms` on an acceptance, where the value is of no use to anyone
/// but a `"banana"` there is still a host that does not know its own contract
/// (F-14).
///
/// **Then the verdict.** What is refused beyond a type is a **contradiction
/// about the verdict**, and only that. §6.3 states three co-occurrences; this
/// enforces the one that can make a caller wrong about whether the envelope
/// was taken, and carries the others. A well-typed `retry_after_ms` beside
/// `engaged` is surplus advice a conforming writer may ignore (§6.3), and a
/// ninth reason token from a newer host may legitimately carry it — refusing
/// that would narrow the wire over a field that cannot mislead anyone (F-14).
///
/// # Errors
///
/// [`SendFault::NonConforming`], naming the rule broken:
///
/// - a field whose JSON type is not the one §6.3 gives it;
/// - a reply carrying no `accepted`;
/// - a refusal carrying no `reason`;
/// - an **acceptance carrying a `reason`**. §6.3 says `reason` is present
///   *exactly* when `accepted` is `false`, so this reply's two modelled
///   fields disagree about the verdict — and an ambiguous message fails
///   rather than being guessed at (CLAUDE.md invariant 2, F-4). Reading
///   `accepted` as the authoritative one would report *the host took it* for
///   a host whose refusal path set `reason` and forgot to clear `accepted`,
///   which is the one shape of that bug nothing else here catches;
/// - a `retry_after_ms` that is not a whole number of milliseconds (R-14).
///
/// None can be reported as a refusal: a caller branching on the reason token
/// would have nothing to branch on, and reporting *refused* without one would
/// be the untruth (`design.md` §5.5).
pub fn read_reply(reply: Reply) -> Result<Answered, SendFault> {
  let Reply {
    protocol: _unread,
    accepted,
    reason,
    retry_after_ms,
    detail,
  } = reply;

  let accepted = flag(accepted)?;
  let reason = text(reason, "`reason` is not a JSON string (SPEC-003 6.3)")?;
  let retry_after_ms = whole_millis(retry_after_ms)?;
  let detail = text(detail, "`detail` is not a JSON string (SPEC-003 6.3)")?;

  match (accepted, reason) {
    (None, _no_verdict) => Err(SendFault::NonConforming(
      "the reply carries no `accepted` field",
    )),
    (Some(true), None) => Ok(Answered::Accepted),
    (Some(true), Some(_reason)) => Err(SendFault::NonConforming(
      "`accepted` is true and a `reason` is present; SPEC-003 6.3 admits \
       `reason` exactly when `accepted` is false",
    )),
    (Some(false), None) => Err(SendFault::NonConforming(
      "the refusal carries no `reason` token",
    )),
    (Some(false), Some(reason)) => Ok(Answered::Refused {
      reason,
      retry_after_ms,
      detail,
    }),
  }
}

/// `accepted`, as the JSON boolean §6.3 says it is. Absent — or `null`, which
/// serde reads as absent — is `None`, and is the caller's *"carries no
/// `accepted` field"* to name, not this function's.
///
/// # Errors
///
/// [`SendFault::NonConforming`] if it is present and is not a boolean.
fn flag(value: Option<Value>) -> Result<Option<bool>, SendFault> {
  match value {
    None => Ok(None),
    Some(Value::Bool(flag)) => Ok(Some(flag)),
    Some(_not_a_boolean) => Err(SendFault::NonConforming(
      "`accepted` is not a JSON boolean (SPEC-003 6.3)",
    )),
  }
}

/// `reason` or `detail`, as the JSON string §6.3 says each is. `breach` is the
/// caller's sentence because [`SendFault::NonConforming`] carries a
/// `&'static str` and two fields need two sentences.
///
/// # Errors
///
/// [`SendFault::NonConforming`] if it is present and is not a string.
fn text(value: Option<Value>, breach: &'static str) -> Result<Option<String>, SendFault> {
  match value {
    None => Ok(None),
    Some(Value::String(text)) => Ok(Some(text)),
    Some(_not_a_string) => Err(SendFault::NonConforming(breach)),
  }
}

/// `retry_after_ms`, as the *whole number of milliseconds* `SPEC-003/R-14`
/// says it is — and R-14 does not say how a host spells one. JSON has a
/// single number type, so `1800`, `1800.0` and `1.8e3` are one value and one
/// verdict; refusing the last two would narrow the wire to the encoders that
/// happen to emit integers (`review-code.md` F-1).
///
/// Permissiveness stops at the spelling. A fractional, negative or
/// out-of-range value is not a whole number of milliseconds and is not
/// rounded, clamped or guessed at — it is the host's named breach. So is a
/// value that is not a number at all: `retry_after_ms` is a field this client
/// models, and invariant 2's permissiveness covers only fields it does not.
///
/// # Errors
///
/// [`SendFault::NonConforming`], naming which of those it was.
fn whole_millis(value: Option<Value>) -> Result<Option<u64>, SendFault> {
  let Some(value) = value else {
    return Ok(None);
  };
  let Some(number) = value.as_number() else {
    return Err(SendFault::NonConforming(
      "`retry_after_ms` is not a JSON number",
    ));
  };
  if let Some(exact) = number.as_u64() {
    return Ok(Some(exact));
  }
  // Anything serde could not hold as a `u64` it holds as an `f64`, and the
  // two steps below are what turn one back into the other **without a cast**
  // (`as_conversions` is denied workspace-wide, and it is denied for this
  // reason: `as` would saturate a huge float and truncate a fractional one,
  // silently, into a millisecond count a caller would act on).
  //
  // `fract` is the meaning check — `1800.5` is not a whole number of
  // milliseconds, and `{:.0}` would *round* it rather than refuse it. Past
  // that guard the value is an integer, `{:.0}` renders exactly that integer,
  // and `parse` is what refuses a negative or out-of-range one.
  number
    .as_f64()
    .filter(|number| number.fract() == 0.0)
    .and_then(|number| format!("{number:.0}").parse().ok())
    .map(Some)
    .ok_or(SendFault::NonConforming(
      "`retry_after_ms` is not a whole number of milliseconds (SPEC-003/R-14)",
    ))
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

  /// `protocol` is not read at all, which is a claim about its *value* and
  /// not only about its presence (`review-code.md` F-1). JSON has one number
  /// type — `1.0` **is** `1` — and a client that refuses a reply over the
  /// spelling of a field it has just said it does not read has narrowed the
  /// wire to what one host happens to emit.
  #[test]
  fn protocol_is_not_read_whatever_it_says() {
    for text in [
      r#"{"protocol":1,"accepted":true}"#,
      r#"{"protocol":1.0,"accepted":true}"#,
      r#"{"protocol":300,"accepted":true}"#,
      r#"{"protocol":"1","accepted":true}"#,
      r#"{"protocol":null,"accepted":true}"#,
    ] {
      assert_eq!(
        verdict_of(text).unwrap_or_else(|fault| panic!("{text}: {fault:?}")),
        Answered::Accepted
      );
    }
  }

  /// `retry_after_ms` is *"a whole number of milliseconds"* (`SPEC-003/R-14`),
  /// and R-14 does not say how a host spells one. `1800`, `1800.0` and
  /// `1.8e3` are one JSON number, so they are one verdict.
  #[test]
  fn a_whole_retry_after_ms_is_read_however_it_is_spelled() {
    for text in [
      r#"{"accepted":false,"reason":"too_soon","retry_after_ms":1800}"#,
      r#"{"accepted":false,"reason":"too_soon","retry_after_ms":1800.0}"#,
      r#"{"accepted":false,"reason":"too_soon","retry_after_ms":1.8e3}"#,
    ] {
      assert_eq!(
        verdict_of(text).unwrap_or_else(|fault| panic!("{text}: {fault:?}")),
        Answered::Refused {
          reason: "too_soon".to_owned(),
          retry_after_ms: Some(1800),
          detail: None,
        },
        "{text}"
      );
    }
  }

  /// The other half of the same rule: permissiveness is about *spelling*,
  /// never about meaning. None of these is a whole number of milliseconds, so
  /// none of them is rounded, clamped or guessed at — and each is the host's
  /// named breach rather than unreadable bytes, because the bytes are one
  /// perfectly good JSON object.
  #[test]
  fn a_retry_after_ms_that_is_not_a_whole_number_of_milliseconds_is_a_named_breach() {
    for text in [
      r#"{"accepted":false,"reason":"too_soon","retry_after_ms":1800.5}"#,
      r#"{"accepted":false,"reason":"too_soon","retry_after_ms":-1}"#,
      r#"{"accepted":false,"reason":"too_soon","retry_after_ms":"1800"}"#,
      r#"{"accepted":false,"reason":"too_soon","retry_after_ms":true}"#,
    ] {
      let fault = breach_of(text);
      assert!(
        matches!(fault, SendFault::NonConforming(_)),
        "{text}: {fault:?}"
      );
    }
  }

  /// Every field §6.3 gives a type is adjudicated against that type, and a
  /// wrong one is the host's **named** breach — not a parse failure reported
  /// as *"not one JSON object"* about something that plainly is one
  /// (`review-code.md` F-11).
  #[test]
  fn a_modelled_field_of_the_wrong_type_is_a_named_breach() {
    for text in [
      r#"{"accepted":"yes"}"#,
      r#"{"accepted":1}"#,
      r#"{"accepted":false,"reason":5}"#,
      r#"{"accepted":false,"reason":["too_soon"]}"#,
      r#"{"accepted":false,"reason":"too_soon","detail":7}"#,
    ] {
      let fault = breach_of(text);
      assert!(
        matches!(fault, SendFault::NonConforming(_)),
        "{text}: {fault:?}"
      );
    }
  }

  /// The sharp end of the same rule (`review-code.md` F-14): `retry_after_ms`
  /// is a field this client models, so its **type** is adjudicated wherever
  /// it appears — including on an acceptance, where its value is of no use to
  /// anyone. It used to pass through silently as exit 0, two lines from the
  /// code that refuses the same field for being `1800.5`.
  #[test]
  fn a_wrong_typed_retry_after_ms_is_a_named_breach_even_on_an_acceptance() {
    let fault = breach_of(r#"{"protocol":1,"accepted":true,"retry_after_ms":"banana"}"#);
    assert!(matches!(fault, SendFault::NonConforming(_)), "{fault:?}");
  }

  /// The other side of that line, and the line itself: what is refused is a
  /// contradiction **about the verdict**, not any co-occurrence §6.3 names.
  /// A well-typed `retry_after_ms` beside a reason that is not `too_soon` is
  /// surplus advice a caller may ignore, and a ninth reason from a newer host
  /// may carry it — refusing it would narrow the wire for a field that cannot
  /// mislead anyone about whether the envelope was taken.
  #[test]
  fn surplus_but_well_typed_advice_is_carried_rather_than_refused() {
    assert_eq!(
      verdict_of(r#"{"accepted":false,"reason":"engaged","retry_after_ms":500}"#)
        .expect("a stray advisory field does not sink a clear refusal"),
      Answered::Refused {
        reason: "engaged".to_owned(),
        retry_after_ms: Some(500),
        detail: None,
      }
    );
    assert_eq!(
      verdict_of(r#"{"accepted":true,"retry_after_ms":500,"detail":"fine"}"#)
        .expect("an acceptance carrying well-typed surplus is an acceptance"),
      Answered::Accepted
    );
  }

  /// `SPEC-003` §6.3: *"`reason` is present exactly when `accepted` is
  /// `false`"*. A reply whose two modelled fields disagree about the verdict
  /// is ambiguous, and an ambiguous message fails rather than being guessed at
  /// (CLAUDE.md invariant 2, `review-code.md` F-4). Reading `accepted` as
  /// authoritative would report *the host took it* to every caller of a host
  /// whose refusal path set `reason` and forgot to clear `accepted`.
  #[test]
  fn an_acceptance_that_also_carries_a_reason_is_a_named_breach() {
    let fault = breach_of(r#"{"protocol":1,"accepted":true,"reason":"too_soon"}"#);
    assert!(matches!(fault, SendFault::NonConforming(_)), "{fault:?}");
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
  /// go through `answer`, the one step between the wire and the rule.
  ///
  /// `[1,2]` is the load-bearing one now that every `Reply` field is a
  /// `Value`: serde's derive reads a struct from a *sequence* as well as a
  /// map, so without `answer`'s shape guard an array parses into a `Reply`
  /// and an array gets reported as a §6.3 breach.
  ///
  /// The last is the scope's other edge (F-17): bytes that are not JSON cost
  /// a verdict from an **unmodelled** field too, and must — the sentence is
  /// true of them, which is exactly what the sibling cases' "that is still
  /// JSON" excludes.
  #[test]
  fn bytes_that_are_not_one_json_object_are_unreadable() {
    for bytes in [
      &b"[1,2]"[..],
      b"not json",
      b"\xff\xfe",
      br#"{"accepted":true,"unmodelled":"\q"}"#,
    ] {
      let fault = answer(bytes).expect_err("these bytes are not one JSON object");
      assert!(
        matches!(fault, SendFault::Unreadable(_)),
        "{bytes:?}: {fault:?}"
      );
    }
  }

  /// One half of the module doc's scope: a document that **is** one JSON
  /// object, refused as not being one, over a fault in a field `Reply`
  /// declares. The shapes here illustrate the scope and do not enumerate it —
  /// the list has been "complete" three times and wrong three times, so the
  /// count is deliberately not asserted and none should be added back
  /// (`review-code.md` F-17).
  ///
  /// This case is also the signal if any of these stops being an exception:
  /// `arbitrary_precision` would red the range shape, and it should, because
  /// the clause it illustrates would then be wrong about it.
  #[test]
  fn a_declared_field_serde_json_will_not_build_a_value_for_is_unreadable() {
    let deep = format!(
      r#"{{"accepted":true,"retry_after_ms":{}1{}}}"#,
      "[".repeat(130),
      "]".repeat(130)
    );
    for bytes in [
      deep.as_bytes(),
      br#"{"accepted":false,"reason":"too_soon","retry_after_ms":1e999}"#,
      br#"{"accepted":true,"detail":"\ud800"}"#,
      br#"{"accepted":true,"accepted":false}"#,
    ] {
      let fault = answer(bytes).expect_err("serde_json yields no value for these");
      assert!(
        matches!(fault, SendFault::Unreadable(_)),
        "{bytes:?}: {fault:?}"
      );
    }
  }

  /// The other half of the scope, and the constraint the shape question must
  /// not break (`wire.rs`): *"a sixth field from a newer host must not turn a
  /// perfectly good verdict into a fault"* — the project's second invariant,
  /// on the axis it exists to hold.
  ///
  /// **"That is still JSON" is the whole of the qualification**, and it is not
  /// hedging: an unmodelled field whose bytes are not JSON at all — `"\q"`,
  /// an unbalanced bracket, a bare control byte in a string — is refused
  /// wherever it sits, and `bytes_that_are_not_one_json_object_are_unreadable`
  /// is where that is pinned. What this case holds is that everything the
  /// grammar admits costs nothing: the parse skips an unknown field without
  /// building a value for it, so range, depth and repetition in one are free,
  /// and nothing running before or after may be stricter than that parse
  /// (`review-code.md` F-15(iii)).
  #[test]
  fn an_unmodelled_field_that_is_still_json_cannot_cost_a_verdict() {
    let deep = format!(
      r#"{{"accepted":true,"extra":{}1{}}}"#,
      "[".repeat(130),
      "]".repeat(130)
    );
    for text in [
      deep.as_str(),
      r#"{"accepted":true,"extra":1e999}"#,
      r#"{"accepted":true,"severity":1,"severity":2}"#,
    ] {
      assert_eq!(
        answer(text.as_bytes()).unwrap_or_else(|fault| panic!("{text}: {fault:?}")),
        Answered::Accepted,
        "{text}"
      );
    }
  }
}

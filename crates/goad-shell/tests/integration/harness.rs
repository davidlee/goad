//! What every case in this tier needs beyond the shared host-driving helpers: a
//! transport pointed at a backend script, a request to send, and the diagnostics
//! that describe what came back.
//!
//! Helpers, not a framework. Cases name their own timeout rather than sharing a
//! constant — the timeout cases want a short one so the suite stays fast, and
//! the success cases want one long enough that a healthy exchange cannot
//! flake. Anything two of the three case files need lives here; anything one
//! of them needs stays there. What both *tiers* need lives in
//! `tests/support/driving.rs` instead (`design.md` §12.8).

use std::path::Path;
use std::time::Duration;

use goad_semantics::error::ProtocolError;
use goad_semantics::protocol::canonical::{Evaluate, Event, Request, Timestamp};
use goad_semantics::protocol::normalize::Discarded;
use goad_shell::backend::process::ProcessBackend;
use goad_shell::backend::transport::Exchange;
use goad_shell::config::Command;
use goad_shell::error::{BackendError, CleanupFailure, StateError};
use goad_shell::host::{Failure, Outcome};

use crate::driving::event;

/// Re-exported so the transport cases keep naming one module. `backend`,
/// `marker` and `clear` are `scripted`'s dependencies and moved with it (§12.8);
/// this tier calls all three directly as well, through `harness::`.
pub(crate) use crate::driving::{backend, clear, marker};

/// A transport pointed at one script, with the timeout this case wants.
pub(crate) fn transport(name: &str, timeout: Duration) -> ProcessBackend {
  ProcessBackend::new(backend(name), timeout)
}

/// A request. Which one barely matters to a transport — it serializes whatever
/// it is handed — so cases that do not care about the payload use this.
pub(crate) fn evaluate() -> Request {
  padded_evaluate("")
}

/// The same, with `Event.data` carrying `padding`. `data` is opaque to the host
/// (R-9), so a large one is a legitimate request; the `Io` case needs one past
/// the pipe buffer, because a write that fits the buffer succeeds even when the
/// reader has already gone.
pub(crate) fn padded_evaluate(padding: &str) -> Request {
  let now = Timestamp::new(jiff::Timestamp::UNIX_EPOCH);
  Request::Evaluate(Evaluate {
    now,
    event: Event {
      source: "test".to_owned(),
      kind: "poll".to_owned(),
      timestamp: now,
      data: serde_json::json!({ "padding": padding }),
    },
  })
}

/// What an exchange came back with, as a sentence.
///
/// `Debug`-based formatting is denied crate-wide and the test tiers here answer
/// that by giving their diagnostics a `Display` rather than by excepting the
/// lint — `crates/goad-boundary/src/scan.rs`'s `Breach` is the precedent. A panic
/// message is the one thing in a test that is certain to be read.
pub(crate) fn describe(result: &Result<Vec<u8>, BackendError>) -> String {
  match result {
    Ok(bytes) => format!("a {}-byte response", bytes.len()),
    Err(error) => error.to_string(),
  }
}

/// The same for the cleanup channel, whose `None` is the interesting case.
pub(crate) fn describe_cleanup(cleanup: Option<&CleanupFailure>) -> String {
  match cleanup {
    None => "disposed of cleanly".to_owned(),
    Some(failure) => failure.to_string(),
  }
}

/// The captured stderr as text. Lossy on purpose: what a case asserts about a
/// diagnostic stream is that a message arrived, and no fixture here writes
/// anything that is not UTF-8.
pub(crate) fn stderr(exchange: &Exchange) -> String {
  String::from_utf8_lossy(&exchange.stderr.bytes).into_owned()
}

/// The pids of this process's children that are running something whose command
/// line contains `needle`.
///
/// Read from `/proc`, with no external tool: the devshell declares neither
/// `procps` nor `coreutils`, so `pgrep` would be an ambient dependency of the
/// test tier. Two properties this rests on, both measured. A grandchild whose
/// parent has died is **not** here — it reparents to init — which is what makes
/// R-48's claim about *children* the one a grandchild fixture cannot falsify.
/// And the needle is not a nicety: `cargo test` runs a target's cases as threads
/// of one process, so an unfiltered count sees every concurrently running case's
/// child and would report another case's ordinary work as this one's leak.
pub(crate) fn children_running(needle: &str) -> Vec<String> {
  children()
    .into_iter()
    .filter(|pid| command_line(pid).contains(needle))
    .collect()
}

/// This process's children, by pid.
///
/// Unfiltered, because the filter cannot be trusted for this question: a backend
/// that `exec`s — which two of them do, deliberately — is no longer named by its
/// script in `/proc`, so a needle over `tests/backends/` reports "clean" for a
/// leaked `sleep 30`. Found by breaking the kill. What makes the unfiltered form
/// usable under `cargo test`'s parallelism is settling rather than sampling: a
/// concurrently running case's child goes away on its own, and a leak does not.
pub(crate) fn children() -> Vec<String> {
  let mut pids = Vec::new();
  let Ok(tasks) = std::fs::read_dir("/proc/self/task") else {
    return pids;
  };
  for task in tasks.flatten() {
    if let Ok(text) = std::fs::read_to_string(task.path().join("children")) {
      pids.extend(text.split_whitespace().map(str::to_owned));
    }
  }
  pids
}

/// A process's argument vector, spaces for the NULs. Empty if it has gone, which
/// is not an error: enumeration and inspection cannot be atomic.
fn command_line(pid: &str) -> String {
  std::fs::read_to_string(format!("/proc/{pid}/cmdline"))
    .unwrap_or_default()
    .replace('\0', " ")
}

/// The pid a backend reported on its own first line of stderr.
///
/// The convention every misbehaving script here follows, and the only
/// confirmation of disposal that is independent of the host's own report: the
/// host says it killed something, and this says what. Empty is a failure rather
/// than a pid, because `alive("")` would answer for `/proc` itself.
pub(crate) fn reported_pid(exchange: &Exchange) -> String {
  let pid = stderr(exchange)
    .lines()
    .next()
    .unwrap_or_default()
    .trim()
    .to_owned();
  assert!(
    pid.chars().all(|character| character.is_ascii_digit()) && !pid.is_empty(),
    "the backend reported no pid; its stderr began {}",
    stderr(exchange).escape_debug()
  );
  pid
}

/// Is a pid still live?
///
/// `/proc` rather than `kill -0`, for the reason the enumeration above gives:
/// the devshell declares no `procps` or `coreutils`, so shelling out reaches for
/// whatever is ambient — and it spawned a child of its own on every call, which
/// the children enumeration then had to see go by. A zombie counts as live under
/// both readings, which is right: it has not been reaped.
pub(crate) fn alive(pid: &str) -> bool {
  Path::new("/proc").join(pid).exists()
}

// ---------------------------------------------------------------------------
// PHASE-08 — a whole host over the real transport, and the invocation witness
// ---------------------------------------------------------------------------

/// The argument vector for the deno example — `examples/typescript/backend.ts`.
///
/// Rooted at the crate for the reason `backend` gives: a test binary's working
/// directory is not something to rely on. The example's own README uses a
/// relative path, which is right for a user's config and wrong here.
///
/// `-A` grants the script the user's full authority, which is what brief §14
/// says a backend has. It is not a sandbox with a hole in it; there is no
/// sandbox.
pub(crate) fn example() -> Command {
  let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/typescript/backend.ts");
  Command::new(
    "deno",
    vec![
      "run".to_owned(),
      "-A".to_owned(),
      script.display().to_string(),
    ],
  )
}

/// An event the example backend answers with a view.
pub(crate) fn prompting_event(now: Timestamp) -> Event {
  event(now, 90)
}

// ---------------------------------------------------------------------------
// The host tier's own diagnostics
// ---------------------------------------------------------------------------

/// What an outcome came back with, as a sentence.
///
/// Moved here from `tests/support/driving.rs` at PHASE-06 (PL-4, review-code
/// round 1): it was declared "host-driving" by §12.8's original enumeration,
/// but the `renderer` target's own `table.rs` ended up not calling it —
/// every panic message there names the row id instead — so it is no longer
/// called by both including targets, and belongs with the tier that does
/// call it. `driving.rs`'s own `choice`/`presented` no longer call this: a
/// private `no_view` there restates just the "no view" half they need,
/// since that shared file cannot reach into `harness.rs` (§12.8).
pub(crate) fn describe_outcome(outcome: &Outcome) -> String {
  match (&outcome.failure, &outcome.view) {
    (Some(failure), _) => format!("a failure: {failure}"),
    (None, Some(presented)) => format!("a view carrying {}", presented.view_id.as_str()),
    (None, None) => "nothing to show, and no failure".to_owned(),
  }
}

pub(crate) fn backend_error(outcome: &Outcome) -> &BackendError {
  match &outcome.failure {
    Some(Failure::Backend(error)) => error,
    _ => panic!(
      "expected a backend failure; got {}",
      describe_outcome(outcome)
    ),
  }
}

pub(crate) fn state_error(outcome: &Outcome) -> &StateError {
  match &outcome.failure {
    Some(Failure::State(error)) => error,
    _ => panic!("expected a refusal; got {}", describe_outcome(outcome)),
  }
}

/// The `ProtocolError` inside an outcome's failure, or a panic naming what came
/// instead. Every wire and validation refusal reaches a caller this way: `read`
/// maps both `from_slice` and `normalize_response` into `BackendError::Protocol`.
pub(crate) fn protocol_error(outcome: &Outcome) -> &ProtocolError {
  match backend_error(outcome) {
    BackendError::Protocol(error) => error,
    other => panic!("expected a protocol failure; got {other}"),
  }
}

/// The one discard an outcome carries, or a panic saying how many there were.
///
/// Every case that reads a discard expects exactly one, and "the first of
/// several" is a different claim from "the only one" — a second discard nobody
/// looked at is the kind of thing this tier exists to notice.
pub(crate) fn only_discard(outcome: &Outcome) -> &Discarded {
  match outcome.discarded.as_slice() {
    [discarded] => discarded,
    other => panic!(
      "expected exactly one discard; got {}, on {}",
      other.len(),
      describe_outcome(outcome)
    ),
  }
}

/// Whatever the backend wrote to stderr on the exchange behind this outcome.
/// Lossy for the reason `stderr` gives: what a case asserts is that a message
/// arrived, and nothing here writes bytes that are not UTF-8.
pub(crate) fn stderr_of(outcome: &Outcome) -> String {
  String::from_utf8_lossy(&outcome.stderr.bytes).into_owned()
}

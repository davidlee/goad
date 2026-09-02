//! What every case in this tier needs: a backend script located from the test
//! binary, a transport pointed at it, and a request to send.
//!
//! Three functions, not a framework. Cases name their own timeout rather than
//! sharing a constant — the timeout cases want a short one so the suite stays
//! fast, and the success cases want one long enough that a healthy exchange
//! cannot flake.

use std::path::Path;
use std::time::Duration;

use goad::semantics::protocol::canonical::{Evaluate, Event, Request, Timestamp};
use goad::shell::backend::process::ProcessBackend;
use goad::shell::backend::transport::Exchange;
use goad::shell::error::{BackendError, CleanupFailure};

/// The argument vector for one of `tests/backends/`'s scripts.
///
/// `bash` is argv[0] and the script is argv[1], which is R-36's rule and AC-12's
/// own example — so the scripts need neither a shebang nor an executable bit.
/// Rooted at the crate, not the cwd, exactly as `tests/protocol/boundary.rs`
/// does it: a test binary's working directory is not something to rely on.
pub(crate) fn backend(name: &str) -> Vec<String> {
  let script = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests/backends")
    .join(format!("{name}.sh"));
  vec!["bash".to_owned(), script.display().to_string()]
}

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
/// lint — `tests/protocol/boundary.rs`'s `Breach` is the precedent. A panic
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

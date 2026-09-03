//! What every case in this tier needs: a backend script located from the test
//! binary, a transport pointed at it, and a request to send.
//!
//! Three functions, not a framework. Cases name their own timeout rather than
//! sharing a constant — the timeout cases want a short one so the suite stays
//! fast, and the success cases want one long enough that a healthy exchange
//! cannot flake.

use std::path::{Path, PathBuf};
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

/// A path in the temp directory that no other case will collide with, cleared
/// before it is handed out.
///
/// The backend writes it to report something no in-band channel can carry,
/// because the host kills the backend the moment it has what it needs.
pub(crate) fn marker(name: &str) -> PathBuf {
  let path = std::env::temp_dir().join(format!("goad-{name}-{}", std::process::id()));
  clear(&path);
  path
}

/// Remove a marker if it is there. Its absence is the normal case at both ends —
/// before the case, because nothing has written it yet, and after, because a
/// failing case is one where it never appeared.
pub(crate) fn clear(path: &Path) {
  match std::fs::remove_file(path) {
    Ok(()) | Err(_) => (),
  }
}

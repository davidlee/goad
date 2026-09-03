//! What every case in this tier needs: a backend script located from the test
//! binary, a transport or a whole host pointed at it, a request to send, and
//! the diagnostics that describe what came back.
//!
//! Helpers, not a framework. Cases name their own timeout rather than sharing a
//! constant — the timeout cases want a short one so the suite stays fast, and
//! the success cases want one long enough that a healthy exchange cannot
//! flake. Anything two of the three case files need lives here; anything one
//! of them needs stays there.

use std::path::{Path, PathBuf};
use std::time::Duration;

use goad::semantics::protocol::canonical::{Evaluate, Event, Request, Timestamp, ViewId};
use goad::shell::backend::process::ProcessBackend;
use goad::shell::backend::transport::Exchange;
use goad::shell::config::{BackendConfig, Config, ScheduleConfig};
use goad::shell::error::{BackendError, CleanupFailure, StateError};
use goad::shell::host::{Failure, Host, Outcome};

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
pub(crate) fn example() -> Vec<String> {
  let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/typescript/backend.ts");
  vec![
    "deno".to_owned(),
    "run".to_owned(),
    "-A".to_owned(),
    script.display().to_string(),
  ]
}

/// The default poll every host here is seeded with, so a case that asserts a
/// `next_check` has one number to reason about.
pub(crate) const DEFAULT_POLL: jiff::SignedDuration = jiff::SignedDuration::from_mins(30);

/// A `Config` built around one command.
///
/// Constructed rather than parsed: `Config`'s fields are `pub` and the TOML
/// route would mean quoting an absolute path into a document, which is a
/// property of the grammar `config.rs`'s own tests already hold. Nothing here
/// is about configuration parsing.
pub(crate) fn config(command: Vec<String>, timeout: Duration) -> Config {
  Config {
    backend: BackendConfig { command, timeout },
    schedule: ScheduleConfig {
      default_poll: DEFAULT_POLL,
    },
  }
}

/// A host over the **real** process transport, pointed at one command.
///
/// This is the composition stratum 3 will perform: the transport is built from
/// the configuration's own command and timeout, so a case cannot accidentally
/// point the two at different backends. Returned by value and driven through as
/// many exchanges as a case likes — `evaluate` and `respond` take `&mut self`
/// (I6), so a sequence is sequential by construction and PHASE-10/EX-2's
/// one-host requirement needs nothing further.
pub(crate) fn host(
  command: Vec<String>,
  timeout: Duration,
  now: Timestamp,
) -> Host<ProcessBackend> {
  let config = config(command, timeout);
  let backend = ProcessBackend::new(config.backend.command.clone(), config.backend.timeout);
  Host::new(config, backend, now)
}

/// An event the example backend has nothing to say about.
///
/// `data` is opaque to the host (R-9) and is where these two events differ.
/// Both scripted backends read the same key, and `answers-a-round-trip.sh`
/// matches these exact two values — it has no JSON parser, so a third value is
/// a broken fixture and it says so on stderr.
pub(crate) fn quiet_event(now: Timestamp) -> Event {
  event(now, 0)
}

/// An event the example backend answers with a view.
pub(crate) fn prompting_event(now: Timestamp) -> Event {
  event(now, 90)
}

fn event(now: Timestamp, minutes_since_entry: u32) -> Event {
  Event {
    source: "test".to_owned(),
    kind: "scheduled".to_owned(),
    timestamp: now,
    data: serde_json::json!({ "minutes_since_entry": minutes_since_entry }),
  }
}

/// A backend script that keeps a log of its invocations, and the path to it.
///
/// The path travels as **argv[2]**, which is how a command is parameterized
/// when nothing interposes a shell (R-36): no environment variable to set — a
/// process-wide, racy thing to do under `cargo test`'s in-process parallelism —
/// and no JSON for bash to parse. Each case names its own log, so concurrent
/// cases cannot read each other's lines.
///
/// The log is the only evidence of a *non*-event — "the backend was not
/// spawned" — that does not come from the host's own report of itself, which is
/// PHASE-06's lesson about bounds applied to a refusal.
pub(crate) fn logging_backend(name: &str, case: &str) -> (Vec<String>, PathBuf) {
  let log = marker(&format!("invocations-{case}"));
  let mut command = backend(name);
  command.push(log.display().to_string());
  (command, log)
}

/// How many times the script has run. Absent is zero: a log with no lines and a
/// log that was never created are the same claim.
pub(crate) fn invocations(log: &Path) -> usize {
  std::fs::read_to_string(log)
    .unwrap_or_default()
    .lines()
    .count()
}

// ---------------------------------------------------------------------------
// The host tier's own diagnostics
// ---------------------------------------------------------------------------

/// An instant from its RFC 3339 spelling, for a fixture that states one.
pub(crate) fn instant(rfc3339: &str) -> Timestamp {
  Timestamp::new(rfc3339.parse().expect("the fixture must be an instant"))
}

/// What an outcome came back with, as a sentence.
///
/// The tier's `Display`-not-`Debug` rule, as `describe` states it for the
/// transport. Shared by `host.rs` and `round_trip.rs`, which make the same
/// claims against a fake and against a process.
pub(crate) fn describe_outcome(outcome: &Outcome) -> String {
  match (&outcome.failure, &outcome.view) {
    (Some(Failure::Backend(error)), _) => format!("a backend failure: {error}"),
    (Some(Failure::State(error)), _) => format!("a refusal: {error}"),
    (None, Some(presented)) => format!("a view carrying {}", presented.view_id.as_str()),
    (None, None) => "nothing to show, and no failure".to_owned(),
  }
}

/// The id of the view an outcome carries, or a panic naming what came instead.
pub(crate) fn presented(outcome: &Outcome) -> &ViewId {
  match &outcome.view {
    Some(presented) => &presented.view_id,
    None => panic!("expected a view; got {}", describe_outcome(outcome)),
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

/// Whatever the backend wrote to stderr on the exchange behind this outcome.
/// Lossy for the reason `stderr` gives: what a case asserts is that a message
/// arrived, and nothing here writes bytes that are not UTF-8.
pub(crate) fn stderr_of(outcome: &Outcome) -> String {
  String::from_utf8_lossy(&outcome.stderr.bytes).into_owned()
}

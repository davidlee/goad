//! The scripted-backend half of slice 001's test helpers, split out of
//! `driving.rs` at slice 003 PHASE-05 (D-18, FD-3, PL-5) so a target that
//! only drives a scripted backend — and never composes a `Host` through
//! `driving.rs`'s own `host`/`host_from`/`config` — can include this file
//! alone and satisfy `dead_code` (`warn`, promoted to an error by the gate's
//! `-D warnings`) without restating anything or suppressing the lint.
//!
//! Included the same way `driving.rs` is: `#[path =
//! "../../../../tests/support/scripting.rs"]`, four levels up from
//! `crates/<member>/tests/<target>/`, the repository root
//! (`docs/memory/cargo-test-cwd-is-package-root-not-workspace-root.md`).

use std::path::{Path, PathBuf};

use goad_shell::config::Command;

/// The argument vector for one of `tests/backends/`'s scripts.
///
/// `bash` is argv[0] and the script is argv[1], which is R-36's rule and AC-12's
/// own example — so the scripts need neither a shebang nor an executable bit.
/// Rooted at the crate, not the cwd, exactly as `crates/goad-boundary/src/scan.rs`
/// does it: a test binary's working directory is not something to rely on.
pub(crate) fn backend(name: &str) -> Command {
  let script = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../tests/backends")
    .join(format!("{name}.sh"));
  Command::new("bash", vec![script.display().to_string()])
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
pub(crate) fn logging_backend(name: &str, case: &str) -> (Command, PathBuf) {
  let log = marker(&format!("invocations-{case}"));
  let mut command = backend(name);
  command.arguments.push(log.display().to_string());
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

/// A backend told what to do, one instruction per invocation.
///
/// The instructions travel as argv[3…], behind the invocation log argv[2] that
/// `logging_backend` supplies — the same parameterization for the same reason
/// (R-36). An instruction is a response body, or one of the four sentinels
/// `tests/backends/answers-as-instructed.sh` documents.
///
/// One script rather than one per misbehaviour, because a `Host` is built
/// around one command and EX-2 runs the whole misbehaving suite through a
/// single `Host`: a backend that varies by invocation is the only shape that
/// admits. Giving the per-mode cases a second mechanism would then mean the
/// suite case and the individual cases were running different backends.
pub(crate) fn scripted(case: &str, instructions: &[&str]) -> (Command, PathBuf) {
  let (mut command, log) = logging_backend("answers-as-instructed", case);
  command.arguments.extend(
    instructions
      .iter()
      .map(|instruction| (*instruction).to_owned()),
  );
  (command, log)
}

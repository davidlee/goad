//! The spawn helpers every file of this target shares.
//!
//! `check`, `check_with_stdout_full`, `code_of`, `stderr_of` and `stdout_of`
//! are copies of `goad-emit`'s `crates/goad-emit/tests/binary/exchange.rs`
//! (`emit`, `emit_with_stdout_full`, `code_of`, `stderr_of`, `stdout_of`): one
//! test target cannot include another crate's helper, and no `tests/support/`
//! file holds them (FU-5).
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Output;

use goad_shell::config::Command;

/// Begins every line `goad-check` writes to stderr (SPEC-004's stderr line).
pub(crate) const PREFIX: &str = "goad-check: ";

/// A file committed beside this target, by an absolute path: a test's working
/// directory is the package root, which is not something to rely on.
pub(crate) fn fixture(name: &str) -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("tests/binary")
    .join(name)
}

/// The argv form against `backend`, with each event file in the order given.
pub(crate) fn against(backend: &Command, events: &[&Path]) -> Vec<String> {
  events
    .iter()
    .flat_map(|event| ["--event".to_owned(), event.display().to_string()])
    .chain(["--".to_owned(), backend.program.clone()])
    .chain(backend.arguments.iter().cloned())
    .collect()
}

/// The built binary, not `cargo run`. A copy of `goad-emit`'s `emit`.
pub(crate) fn check(arguments: &[impl AsRef<OsStr>]) -> Output {
  std::process::Command::new(env!("CARGO_BIN_EXE_goad-check"))
    .args(arguments)
    .output()
    .expect("the built binary must be runnable")
}

/// The same spawn with standard output on `/dev/full`, where every write
/// fails. A copy of `goad-emit`'s `emit_with_stdout_full`.
pub(crate) fn check_with_stdout_full(arguments: &[impl AsRef<OsStr>]) -> Output {
  let full = std::fs::OpenOptions::new()
    .write(true)
    .open("/dev/full")
    .expect("/dev/full must be openable for writing");
  std::process::Command::new(env!("CARGO_BIN_EXE_goad-check"))
    .args(arguments)
    .stdout(full)
    .output()
    .expect("the built binary must be runnable")
}

/// A copy of `goad-emit`'s `code_of`.
pub(crate) fn code_of(output: &Output) -> i32 {
  output
    .status
    .code()
    .expect("the binary must exit rather than be signalled")
}

/// A copy of `goad-emit`'s `stderr_of`.
pub(crate) fn stderr_of(output: &Output) -> String {
  String::from_utf8(output.stderr.clone()).expect("goad-check writes UTF-8")
}

/// A copy of `goad-emit`'s `stdout_of`.
pub(crate) fn stdout_of(output: &Output) -> String {
  String::from_utf8(output.stdout.clone()).expect("goad-check writes UTF-8")
}

/// Status 2, not judged, with SPEC-004's line **last** on stderr.
pub(crate) fn assert_not_judged(output: &Output) {
  let stderr = stderr_of(output);
  assert_eq!(code_of(output), 2, "{stderr}");
  assert_last_stderr_line_is_the_checker_s(output);
}

/// SPEC-004/R-14's row: the last stderr line on a non-zero status is the
/// checker's own.
pub(crate) fn assert_last_stderr_line_is_the_checker_s(output: &Output) {
  let stderr = stderr_of(output);
  let last = stderr.lines().last().unwrap_or_default();
  assert!(last.starts_with(PREFIX), "the last stderr line: {stderr}");
}

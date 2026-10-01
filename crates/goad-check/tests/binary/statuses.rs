//! The spawn helpers and the cases.
//!
//! The helpers are copies of `goad-emit`'s
//! `crates/goad-emit/tests/binary/exchange.rs` (`emit`,
//! `emit_with_stdout_full`, `code_of`, `stderr_of`, `stdout_of`): one test
//! target cannot include another crate's helper, and no `tests/support/` file
//! holds them (FU-5).
use std::path::PathBuf;
use std::process::Output;

/// Begins every line `goad-check` writes to stderr (SPEC-004's stderr line).
const PREFIX: &str = "goad-check: ";

/// A file committed beside this target, by an absolute path: a test's working
/// directory is the package root, which is not something to rely on.
fn fixture(name: &str) -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("tests/binary")
    .join(name)
}

/// The built binary, not `cargo run`. A copy of `goad-emit`'s `emit`.
fn check(arguments: &[&str]) -> Output {
  std::process::Command::new(env!("CARGO_BIN_EXE_goad-check"))
    .args(arguments)
    .output()
    .expect("the built binary must be runnable")
}

/// The same spawn with standard output on `/dev/full`, where every write
/// fails. A copy of `goad-emit`'s `emit_with_stdout_full`.
fn check_with_stdout_full(arguments: &[&str]) -> Output {
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
fn code_of(output: &Output) -> i32 {
  output
    .status
    .code()
    .expect("the binary must exit rather than be signalled")
}

/// A copy of `goad-emit`'s `stderr_of`.
fn stderr_of(output: &Output) -> String {
  String::from_utf8(output.stderr.clone()).expect("goad-check writes UTF-8")
}

/// A copy of `goad-emit`'s `stdout_of`.
fn stdout_of(output: &Output) -> String {
  String::from_utf8(output.stdout.clone()).expect("goad-check writes UTF-8")
}

/// Status 2, not judged, with SPEC-004's line **last** on stderr.
fn assert_not_judged(output: &Output) {
  let stderr = stderr_of(output);
  assert_eq!(code_of(output), 2, "{stderr}");
  let last = stderr.lines().last().unwrap_or_default();
  assert!(last.starts_with(PREFIX), "the last stderr line: {stderr}");
}

/// A failure before the first exchange writes no report line, at PHASE-12
/// too. Without this a case is green against the interim end, which is also
/// status 2 with a `goad-check: ` line last.
fn assert_no_report(output: &Output) {
  assert!(output.stdout.is_empty(), "{}", stdout_of(output));
}

/// SPEC-004/R-11: a question answered is status 0. The usage block goes to
/// stdout, so it can be paged.
#[test]
fn help_prints_the_usage_block_on_stdout_and_exits_0() {
  let output = check(&["--help"]);

  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  let stdout = stdout_of(&output);
  assert!(stdout.starts_with("usage: goad-check"), "{stdout}");
  assert!(stdout.contains("--config"), "{stdout}");
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
}

/// The package version on stdout, status 0, and nothing else.
#[test]
fn version_prints_the_package_version_on_stdout_and_exits_0() {
  let output = check(&["--version"]);

  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  assert_eq!(stdout_of(&output).trim_end(), env!("CARGO_PKG_VERSION"));
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
}

#[test]
fn an_unreadable_config_exits_2_and_says_who_spoke() {
  let absent = fixture("absent.toml");
  let output = check(&["--config", &absent.to_string_lossy()]);

  assert_not_judged(&output);
  assert_no_report(&output);
}

/// SPEC-003/R-13, through `envelope::normalize`: an event file the host's
/// socket would refuse is refused here. The configuration is loadable, so no
/// earlier step fails first.
#[test]
fn a_reserved_source_event_file_exits_2() {
  let config = fixture("loadable.toml");
  let event = fixture("reserved-source.json");
  let output = check(&[
    "--config",
    &config.to_string_lossy(),
    "--event",
    &event.to_string_lossy(),
  ]);

  assert_not_judged(&output);
  assert_no_report(&output);
}

/// `Command::from_argv`'s rule, the host's for an empty command.
#[test]
fn an_empty_argv_is_a_usage_error() {
  let output = check(&["--"]);

  assert_not_judged(&output);
  assert_no_report(&output);
}

/// The `--help` half; PHASE-12/VT-2 adds the run half. A question whose
/// answer never reached stdout was not answered, so it is not status 0.
#[test]
fn a_report_that_cannot_be_written_exits_2() {
  let output = check_with_stdout_full(&["--help"]);

  assert_not_judged(&output);
}

/// The interim end: every step before the first exchange succeeds, no
/// exchange is made, and nothing is judged. The report is the no-view line
/// alone, with no verdict line. PHASE-12/EX-6 deletes this case.
#[test]
fn a_run_with_no_exchange_exits_2_with_no_verdict() {
  let config = fixture("loadable.toml");
  let output = check(&["--config", &config.to_string_lossy()]);

  assert_not_judged(&output);
  assert_eq!(
    stdout_of(&output),
    "no exchange returned a view, so respond was not exercised\n"
  );
}

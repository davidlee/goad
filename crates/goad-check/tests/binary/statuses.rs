//! The cases for each status a run can end on before or without its
//! exchanges, and the edges a run shares with them.
use crate::process::{
  against, assert_not_judged, check, check_with_stdout_full, code_of, fixture, stderr_of, stdout_of,
};
use crate::scripting;
use std::process::Output;

/// A failure before the first exchange writes no report line. Without this a
/// case is green against a run that reported and then failed, which is also
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

/// A question whose answer never reached stdout was not answered, and a run
/// whose report never reached stdout delivered no verdict: neither is status
/// 0 or 1. The run half uses a conforming backend, so stdout is the only
/// fault in it.
#[test]
fn a_report_that_cannot_be_written_exits_2() {
  assert_not_judged(&check_with_stdout_full(&["--help"]));

  let (backend, _log) = scripting::scripted("report-unwritten", &[]);
  assert_not_judged(&check_with_stdout_full(&against(&backend, &[])));
}

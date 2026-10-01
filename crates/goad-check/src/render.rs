//! Every line the binary can write, as a `String` or a `const`, with no sink.
//!
//! Pure, as `goad-emit`'s `render.rs` is: a line is asserted with a literal,
//! and the one place a stream is touched is `main`. Every stderr line begins
//! `goad-check: `, the name SPEC-004's stderr line carries. A fault from
//! below is interpolated through its own `Display` and never restated, so the
//! words that say which requirement or side a fault concerns are the host's,
//! not this crate's (`design.md` §5.5 I-1).

use crate::StartupFault;
use crate::args::UsageError;

/// One `const`, no trailing newline: `report`'s `writeln!` supplies it.
///
/// The paragraph on the forms is prose about `args::parse`, whose doc table is
/// the authority; a change there is what must drive a change here.
pub(crate) const USAGE: &str = "usage: goad-check [--config PATH] [--event FILE]...
       goad-check [--timeout SPAN] [--event FILE]... -- PROGRAM [ARG]...
       goad-check -h | --help
       goad-check --version

Runs a goad backend the way the host runs it and reports every exchange.

  --config PATH   the goad configuration naming the backend's command and
                  timeout. Without it, and without --, the host's default
                  configuration path.
  --timeout SPAN  with --, how long one exchange may take; defaults to 5s.
                  A configuration file states its own, so --timeout is
                  refused without --.
  --event FILE    one event envelope, as a watcher would write it to the
                  host's socket. Repeatable; sent in the order given.
  -- PROGRAM ...  the backend's command, in place of a configuration file.";

/// What the report says when no exchange returned a view: respond, the
/// half of the protocol that answers one, went unexercised (`design.md`
/// §5.2.5). Until PHASE-12 no exchange is made, so this is the whole report.
pub(crate) const NO_VIEW: &str = "no exchange returned a view, so respond was not exercised";

/// The interim end of a run: everything before the first exchange succeeded,
/// and there is no exchange yet, so nothing was judged. PHASE-12 deletes it.
pub(crate) const NOT_YET_IMPLEMENTED: &str =
  "goad-check: the exchanges are not yet implemented, so nothing was judged";

/// A usage error, naming what was wrong and not reprinting the usage block:
/// `--help` is where the page lives.
#[must_use]
pub(crate) fn usage_error_line(error: &UsageError) -> String {
  match error {
    UsageError::Empty(flag) => format!("goad-check: {flag} was given an empty value"),
    UsageError::NoValue(flag) => format!("goad-check: {flag} needs a value after it"),
    UsageError::Repeated(flag) => format!("goad-check: {flag} was given more than once"),
    UsageError::NotUtf8(flag) => {
      format!("goad-check: {flag} was given a value that is not UTF-8")
    }
    UsageError::Timeout(fault) => format!("goad-check: {fault}"),
    UsageError::EmptyCommand => {
      "goad-check: the command after -- names no program, so there is nothing to spawn".to_owned()
    }
    UsageError::TimeoutWithoutCommand => "goad-check: --timeout is accepted only with --; a \
                                          configuration file states its own timeout"
      .to_owned(),
    UsageError::ConfigWithCommand => {
      "goad-check: --config and -- exclude each other; give one or the other".to_owned()
    }
    UsageError::Unknown(flag) => {
      format!("goad-check: unknown flag {flag}; run `goad-check --help` for usage")
    }
    UsageError::Positional(argument) => {
      format!("goad-check: unexpected argument {argument}; the backend's command goes after --")
    }
  }
}

/// Why the run never reached its first exchange. A file's fault names the
/// file, because the remedy is a thing done to that file.
#[must_use]
pub(crate) fn startup_error_line(fault: &StartupFault) -> String {
  match fault {
    StartupFault::NoPath => "goad-check: neither XDG_CONFIG_HOME nor HOME names a directory, \
                             so there is no configuration path to read; pass --config, or the \
                             command after --"
      .to_owned(),
    StartupFault::ConfigUnreadable { path, fault } => {
      format!("goad-check: {} could not be read: {fault}", path.display())
    }
    StartupFault::ConfigUnparseable { path, fault } => {
      format!("goad-check: {}: {fault}", path.display())
    }
    StartupFault::EventUnreadable { path, fault } => {
      format!(
        "goad-check: event file {} could not be read: {fault}",
        path.display()
      )
    }
    StartupFault::EventRefused { path, fault } => {
      format!("goad-check: event file {}: {fault}", path.display())
    }
    StartupFault::Clock(fault) => format!("goad-check: the clock could not be read: {fault}"),
    StartupFault::Runtime(fault) => {
      format!("goad-check: the runtime the exchanges run on could not be built: {fault}")
    }
  }
}

/// A `--help` or `--version` answer that standard output refused, in the
/// words `goad-emit` and the host use: one rule for the edge across the
/// binaries (`design.md` §5.2.5).
#[must_use]
pub(crate) fn answer_unwritten_line(fault: &std::io::Error) -> String {
  format!("goad-check: the answer could not be written to standard output: {fault}")
}

/// A report line that standard output refused: the report is the answer, so
/// a report not written is a run not judged (`design.md` §5.2.5).
#[must_use]
pub(crate) fn report_unwritten_line(fault: &std::io::Error) -> String {
  format!("goad-check: the report could not be written to standard output: {fault}")
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use goad_shell::clock::ClockError;
  use goad_shell::error::ConfigError;
  use goad_shell::ingress::envelope::EnvelopeFault;

  use super::{
    NOT_YET_IMPLEMENTED, USAGE, answer_unwritten_line, report_unwritten_line, startup_error_line,
    usage_error_line,
  };
  use crate::StartupFault;
  use crate::args::UsageError;

  const PREFIX: &str = "goad-check: ";

  fn refused() -> std::io::Error {
    std::io::Error::from(std::io::ErrorKind::StorageFull)
  }

  /// Each usage error paired with a phrase only its own arm says, so a case
  /// holds the mapping and not just the set (`goad-emit`'s F-7).
  #[test]
  fn every_usage_error_says_what_was_wrong_and_reprints_nothing() {
    let timeout = goad_shell::config::positive_duration("--timeout", "0s")
      .expect_err("a zero timeout is refused");
    let cases: [(UsageError, &str); 10] = [
      (
        UsageError::Empty("--config"),
        "--config was given an empty value",
      ),
      (
        UsageError::NoValue("--event"),
        "--event needs a value after it",
      ),
      (
        UsageError::Repeated("--timeout"),
        "--timeout was given more than once",
      ),
      (
        UsageError::NotUtf8("--"),
        "-- was given a value that is not UTF-8",
      ),
      (
        UsageError::Timeout(timeout),
        "--timeout must be greater than zero",
      ),
      (UsageError::EmptyCommand, "names no program"),
      (
        UsageError::TimeoutWithoutCommand,
        "--timeout is accepted only with --",
      ),
      (
        UsageError::ConfigWithCommand,
        "--config and -- exclude each other",
      ),
      (
        UsageError::Unknown("--configs".to_owned()),
        "unknown flag --configs",
      ),
      (
        UsageError::Positional("stray".to_owned()),
        "unexpected argument stray",
      ),
    ];
    for (error, said) in cases {
      let line = usage_error_line(&error);
      assert!(line.starts_with(PREFIX), "{line}");
      assert!(line.contains(said), "{line} must say {said}");
      assert!(!line.contains("usage:"), "{line} reprints the usage block");
    }
  }

  /// Each file's fault names the file and says which fault it was; the clock
  /// and the runtime name what failed.
  #[test]
  fn every_startup_fault_says_what_failed_and_names_its_file() {
    let config = Path::new("/home/someone/.config/goad/config.toml");
    let event = Path::new("/home/someone/events/woke.json");
    let cases: [(StartupFault, Option<&Path>, &str); 7] = [
      (
        StartupFault::NoPath,
        None,
        "neither XDG_CONFIG_HOME nor HOME",
      ),
      (
        StartupFault::ConfigUnreadable {
          path: config.to_owned(),
          fault: std::io::Error::from(std::io::ErrorKind::NotFound),
        },
        Some(config),
        "could not be read",
      ),
      (
        StartupFault::ConfigUnparseable {
          path: config.to_owned(),
          fault: ConfigError::EmptyCommand,
        },
        Some(config),
        "names no program",
      ),
      (
        StartupFault::EventUnreadable {
          path: event.to_owned(),
          fault: std::io::Error::from(std::io::ErrorKind::NotFound),
        },
        Some(event),
        "event file",
      ),
      (
        StartupFault::EventRefused {
          path: event.to_owned(),
          fault: EnvelopeFault::ReservedSource,
        },
        Some(event),
        "is reserved",
      ),
      (
        StartupFault::Clock(ClockError::BeforeEpoch),
        None,
        "the clock could not be read",
      ),
      (StartupFault::Runtime(refused()), None, "runtime"),
    ];
    let mut lines = Vec::new();
    for (fault, file, said) in cases {
      let line = startup_error_line(&fault);
      assert!(line.starts_with(PREFIX), "{line}");
      assert!(line.contains(said), "{line} must say {said}");
      if let Some(file) = file {
        assert!(
          line.contains(&*file.to_string_lossy()),
          "{line} must name {file:?}"
        );
      }
      lines.push(line);
    }
    let mut distinct = lines.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(
      distinct.len(),
      lines.len(),
      "two faults read the same: {lines:?}"
    );
  }

  #[test]
  fn an_unwritten_answer_and_an_unwritten_report_are_told_apart() {
    let answer = answer_unwritten_line(&refused());
    let report = report_unwritten_line(&refused());
    assert!(answer.starts_with(PREFIX), "{answer}");
    assert!(report.starts_with(PREFIX), "{report}");
    assert!(
      answer.contains("the answer could not be written"),
      "{answer}"
    );
    assert!(
      report.contains("the report could not be written"),
      "{report}"
    );
  }

  #[test]
  fn the_interim_end_says_nothing_was_judged_on_stderr() {
    assert!(NOT_YET_IMPLEMENTED.starts_with(PREFIX));
    assert!(NOT_YET_IMPLEMENTED.contains("nothing was judged"));
  }

  #[test]
  fn the_usage_block_states_both_forms() {
    assert!(USAGE.starts_with("usage: goad-check"), "{USAGE}");
    assert!(USAGE.contains("--config PATH"), "{USAGE}");
    assert!(USAGE.contains("-- PROGRAM"), "{USAGE}");
  }
}

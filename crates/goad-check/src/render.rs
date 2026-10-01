//! Every line the binary can write, as a `String` or a `const`, with no sink.
//!
//! Pure, as `goad-emit`'s `render.rs` is: a line is asserted with a literal,
//! and the one place a stream is touched is `main`. Every stderr line begins
//! `goad-check: `, the name SPEC-004's stderr line carries. A fault from
//! below is interpolated through its own `Display` and never restated, so the
//! words that say which requirement or side a fault concerns are the host's,
//! not this crate's (`design.md` §5.5 I-1).

use std::fmt::Display;

use goad_semantics::error::{AtFault, Requirement};
use goad_semantics::protocol::canonical::{UserResponse, View};
use goad_semantics::protocol::normalize::Discarded;

use crate::args::UsageError;
use crate::run::{CHAIN_BOUND, Judged};
use crate::{RunFault, StartupFault};

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

/// What the report says, last before the verdict, when no exchange returned
/// a view: respond, the half of the protocol that answers one, went
/// unexercised (`design.md` §5.2.5).
pub(crate) const NO_VIEW: &str = "no exchange returned a view, so respond was not exercised";

/// The checker's own claim against its probe, beside the failure the host
/// reported (`design.md` §5.2.2).
const PROBE_CLAIM: &str = "a backend MUST tolerate a kind it does not recognise";

/// The heading of one planned request's evaluate.
#[must_use]
pub(crate) fn evaluate_line(kind: &str) -> String {
  format!("evaluate {kind}")
}

/// The heading of a respond in a chain: the option answered and the values
/// sent, by field, so a backend that refuses them is not silently blamed
/// (`design.md` §5.5 *Edges*).
#[must_use]
pub(crate) fn respond_line(answer: &UserResponse) -> String {
  let values: Vec<String> = answer
    .values
    .iter()
    .map(|(field, value)| format!("{}={value}", field.as_str()))
    .collect();
  if values.is_empty() {
    format!("  respond {} with no values", answer.option.as_str())
  } else {
    format!(
      "  respond {} with {}",
      answer.option.as_str(),
      values.join(" ")
    )
  }
}

/// Every channel of one exchange's outcome, as `design.md` §5.2.2's table
/// gives it: what came back, each refusal, and the backend's stderr.
#[must_use]
pub(crate) fn exchange_lines(judged: &Judged) -> Vec<String> {
  let next_check = judged.next_check.instant();
  let mut lines = vec![match &judged.view {
    Some(presented) => match &presented.view {
      View::Choice(choice) => format!(
        "  view {}: {:?} · next check {next_check}",
        presented.view_id.as_str(),
        choice.title()
      ),
    },
    None => format!("  view: null · next check {next_check}"),
  }];
  lines.extend(
    judged
      .failure
      .iter()
      .map(|failure| refusal_line(failure.fault(), failure.requirement(), failure)),
  );
  lines.extend(judged.discarded.iter().map(|discard| match discard {
    Discarded::Schedule { reason, .. } => {
      refusal_line(reason.fault(), reason.requirement(), discard)
    }
  }));
  lines.extend(
    judged
      .cleanup
      .iter()
      .map(|cleanup| refusal_line(cleanup.fault(), cleanup.requirement(), cleanup)),
  );
  lines.extend(
    String::from_utf8_lossy(&judged.stderr.bytes)
      .lines()
      .map(|line| format!("  stderr: {line}")),
  );
  if judged.stderr.truncated {
    lines.push("  stderr truncated: the host kept only its first part".to_owned());
  }
  lines
}

/// A refusal, naming its side and requirement through their own `Display`s,
/// never spelling either (`design.md` §5.5 I-1).
fn refusal_line(side: AtFault, requirement: Requirement, what: &dyn Display) -> String {
  format!("  REFUSED  {side}  SPEC-001/{requirement}  {what}")
}

/// The claim R-56's condition makes, as a refusal line: its id is stratum
/// 1's, and its text and side are the checker's own (`design.md` §5.2.2).
#[must_use]
pub(crate) fn probe_claim_line() -> String {
  refusal_line(AtFault::Backend, Requirement::R56, &PROBE_CLAIM)
}

/// The chain bound reached: an observation, not a refusal (`design.md`
/// §5.2.2).
#[must_use]
pub(crate) fn chain_bound_line() -> String {
  format!(
    "  chain bound of {CHAIN_BOUND} responds reached: the last view was left unanswered, \
     which is an observation and not a refusal"
  )
}

/// The report's last line.
#[must_use]
pub(crate) fn verdict_line(refused: usize) -> String {
  match refused {
    0 => "verdict: accepted, no exchange refused".to_owned(),
    refused => format!("verdict: refused, {refused} exchange(s) refused"),
  }
}

/// The last stderr line of a run refused (SPEC-004's stderr line).
#[must_use]
pub(crate) fn refused_line(refused: usize) -> String {
  format!(
    "goad-check: {refused} exchange(s) refused; the report on standard output names each \
     refusal's side and requirement"
  )
}

/// Why a run that had begun its exchanges delivered no verdict.
#[must_use]
pub(crate) fn run_fault_line(fault: &RunFault) -> String {
  match fault {
    RunFault::Clock(fault) => {
      format!("goad-check: the clock could not be read mid-run, so nothing was judged: {fault}")
    }
    RunFault::ReportUnwritten(fault) => report_unwritten_line(fault),
    RunFault::State(error) => format!(
      "goad-check: the checker answered a view the host was not holding, which is the \
       checker's own defect, so nothing was judged: {} SPEC-001/{} {error}",
      error.fault(),
      error.requirement()
    ),
  }
}

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

  use goad_semantics::protocol::canonical::{Timestamp, ViewId};
  use goad_semantics::protocol::normalize::read_response;
  use goad_shell::backend::transport::Captured;
  use goad_shell::error::{BackendError, StateError};

  use super::{
    USAGE, answer_unwritten_line, chain_bound_line, exchange_lines, refused_line,
    report_unwritten_line, respond_line, run_fault_line, startup_error_line, usage_error_line,
    verdict_line,
  };
  use crate::args::UsageError;
  use crate::run::{CHAIN_BOUND, Judged, answer};
  use crate::{RunFault, StartupFault};

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

  fn epoch() -> Timestamp {
    Timestamp::new(jiff::Timestamp::UNIX_EPOCH)
  }

  /// An exchange that returned nothing, with `failure` and `stderr` given.
  fn judged(failure: Option<BackendError>, stderr: Captured) -> Judged {
    Judged {
      view: None,
      next_check: epoch(),
      discarded: Vec::new(),
      stderr,
      failure,
      cleanup: None,
    }
  }

  /// Shape only: the side and the id are the host's `Display`s, and their
  /// values are asserted as literals in the binary tier (I-1 reads `src`).
  #[test]
  fn a_refusal_line_carries_the_failure_s_own_words_under_its_spec() {
    let failure = BackendError::ExitStatus { code: Some(1) };
    let said = failure.to_string();
    let lines = exchange_lines(&judged(Some(failure), Captured::default()));

    let refusals: Vec<&String> = lines
      .iter()
      .filter(|line| line.contains("SPEC-001/"))
      .collect();
    assert_eq!(refusals.len(), 1, "{lines:?}");
    assert!(
      refusals.iter().all(|line| line.contains(&said)),
      "{lines:?}"
    );
  }

  #[test]
  fn the_backend_s_stderr_is_shown_line_by_line_and_its_truncation_flagged() {
    let stderr = Captured {
      bytes: b"first said\nsecond said\n".to_vec(),
      truncated: true,
    };
    let lines = exchange_lines(&judged(None, stderr)).join("\n");

    assert!(lines.contains("first said"), "{lines}");
    assert!(lines.contains("second said"), "{lines}");
    assert!(lines.contains("truncated"), "{lines}");
    let untruncated = exchange_lines(&judged(None, Captured::default())).join("\n");
    assert!(!untruncated.contains("truncated"), "{untruncated}");
  }

  /// The values sent, by field: the answer is built from a view the way the
  /// run builds it.
  #[test]
  fn a_respond_line_names_the_option_and_each_field_sent() {
    let bytes = br#"{"view":{"kind":"choice","title":"Asked","options":[{"id":"chosen","label":"C",
      "fields":[{"id":"first_field","kind":"text","label":"F"},
      {"id":"second_field","kind":"boolean","label":"S"}]}]}}"#;
    let response = read_response(bytes, epoch()).expect("a view the host accepts");
    let view = response.value.view().expect("the response carries a view");
    let line = respond_line(&answer(view));

    assert!(line.contains("chosen"), "{line}");
    assert!(line.contains("first_field="), "{line}");
    assert!(line.contains("second_field="), "{line}");
  }

  #[test]
  fn the_verdict_and_the_status_1_line_tell_accepted_from_refused() {
    assert!(verdict_line(0).starts_with("verdict: accepted"));
    assert!(verdict_line(2).starts_with("verdict: refused"));
    assert!(verdict_line(2).contains('2'));
    assert!(refused_line(2).starts_with(PREFIX));
    assert!(chain_bound_line().contains(&CHAIN_BOUND.to_string()));
  }

  #[test]
  fn every_run_fault_says_nothing_was_judged_and_what_stopped_it() {
    let state = StateError::NoOutstandingView {
      named: ViewId::new("answered"),
    };
    let said = state.to_string();
    let cases = [
      (RunFault::Clock(ClockError::BeforeEpoch), "clock".to_owned()),
      (
        RunFault::ReportUnwritten(refused()),
        "the report could not be written".to_owned(),
      ),
      (RunFault::State(state), said),
    ];
    for (fault, stopped) in cases {
      let line = run_fault_line(&fault);
      assert!(line.starts_with(PREFIX), "{line}");
      assert!(line.contains(&stopped), "{line} must say {stopped}");
    }
  }

  #[test]
  fn the_usage_block_states_both_forms() {
    assert!(USAGE.starts_with("usage: goad-check"), "{USAGE}");
    assert!(USAGE.contains("--config PATH"), "{USAGE}");
    assert!(USAGE.contains("-- PROGRAM"), "{USAGE}");
  }
}

//! The AC-6 taxonomy, stratum 2 half — `design.md` §5.2.
//!
//! These wrap `semantics`' parse and validation errors rather than replacing
//! them: what the wire said and what the transport managed are different
//! questions, and ADR-001 predicted the taxonomy would split along that seam.
//!
//! Nothing here inspects a value or does arithmetic, so no module-level
//! `#![deny(clippy::arithmetic_side_effects)]` (D53 as amended). `process.rs`,
//! which computes over lengths a backend chose, carries one.

use std::fmt;
use std::time::Duration;

use goad_semantics::error::{AtFault, ProtocolError, Requirement, SpanFault};
use goad_semantics::protocol::canonical::ViewId;

/// Why an exchange produced no response body.
///
/// `Spawn` and `Io` carry the operating system's own error because the
/// diagnostic is the whole value of them: "command not found" and "permission
/// denied" are different mistakes with different fixes (brief §13).
#[derive(Debug)]
pub enum BackendError {
  /// The command could not be started — nothing was spawned, so there is
  /// nothing to have cleaned up after.
  Spawn(std::io::Error),
  /// The backend did not complete an exchange within its configured window.
  /// `after` is that window, not the elapsed time: disposal is bounded
  /// separately and a call waits at most the sum (R-41).
  Timeout { after: Duration },
  /// A non-zero exit discards the body it came with, however well that body
  /// parsed (D15, R-40). `None` means the child was signalled rather than
  /// exiting.
  ExitStatus { code: Option<i32> },
  /// The stdout bound was exceeded. A failure, unlike the stderr bound: the
  /// host cannot act on a response it refused to finish reading (R-43).
  OutputTooLarge { limit: usize },
  /// A stdio handle was absent after a successful spawn — a value the host
  /// itself asked for, so its absence is not backend-derived (F-35).
  PipeMissing,
  /// Writing the request or reading the response failed outright.
  Io(std::io::Error),
  /// The bytes arrived and did not mean anything the protocol admits. Raised
  /// where `from_slice` runs, which is the host and not the transport: this
  /// transport returns bytes and parses nothing.
  Protocol(ProtocolError),
}

/// Whether the host disposed of the child, which is a **second dimension** and
/// not another `BackendError`.
///
/// What the backend did and whether the host cleaned up after it are
/// independent facts, and D42's mistake was forcing them into one precedence
/// contest — a timeout says *this invocation failed*, a cleanup failure says
/// *this invocation may still have consequences after the call returns*, and
/// the second outlives the first (F-48, F-53).
#[derive(Debug)]
pub enum CleanupFailure {
  /// Kill, reap and the stderr drain did not all complete within the budget.
  ///
  /// Named for what the host **observed** rather than for what it might mean:
  /// on elapse the child may be alive, dying, exited-but-unreaped, or perfectly
  /// fine with only its stderr held open by a grandchild — and the last is the
  /// case that actually occurs, so `Orphaned` would have been a false statement
  /// about the common path (F-63).
  TimedOut { after: Duration },
  /// `start_kill` or `wait` failed outright.
  Io(std::io::Error),
}

/// Why a caller's answer was refused before any backend was consulted.
///
/// Separate from `BackendError` because the backend did nothing wrong, and it
/// was not asked: `respond` checks the id against host state *before* touching
/// the transport, so a stale answer never reaches a backend author's code
/// (R-32, `design.md:1600`). Two variants rather than one because the
/// diagnostics differ — "there is no interaction open" and "you answered the
/// previous one" are different mistakes with different fixes (D24, F-8, F-15).
///
/// It sits here and not in `semantics/` because staleness is a fact about host
/// state, not about the message: the same bytes are valid or stale depending on
/// what the host is holding, so stratum 1 cannot adjudicate it.
#[derive(Debug)]
pub enum StateError {
  /// Nothing is outstanding. The host has no interaction to answer.
  NoOutstandingView { named: ViewId },
  /// An interaction is outstanding and this is not it — the caller answered a
  /// superseded one. Both ids are carried, because "which one did you mean"
  /// is the question the diagnostic has to settle.
  StaleViewId { named: ViewId, outstanding: ViewId },
}

/// Why a configuration file could not be turned into a `Config`.
///
/// A fifth error type rather than a variant of an existing one, because a config
/// file is none of the subjects the others are about: `ProtocolError` and its two
/// companions are about a *backend's* message, `BackendError` about an exchange
/// that ran, `CleanupFailure` about disposal, and `StateError` about an id a
/// *caller* named. This is the user's own file, read before any backend exists —
/// so reusing `BackendError` would blame a backend that has not been spawned,
/// which is the argument `StateError` already rests on. User decision 2026-09-03;
/// `plan-log.md`.
///
/// Every variant is fatal at construction (`design.md:1236`): there is no backend
/// to run without a config, and guessing a command is not available to us.
#[derive(Debug)]
pub enum ConfigError {
  /// The file is missing or could not be read.
  Read(std::io::Error),
  /// Not TOML, or TOML that is not this shape — a missing section, a missing
  /// key, a value of the wrong type. `toml`'s own message carries the line, the
  /// column and a caret excerpt, so nothing is added to it here.
  Syntax(Box<toml::de::Error>),
  /// A duration string the grammar refuses — `"1 month"`, whose length is not
  /// fixed without a calendar, `"09:00:00"`, which is a time of day, or
  /// something that is not a duration at all. The key is carried because which
  /// one it was is half the diagnostic, and the fault is the other half: it is
  /// `schedule::parse_span`'s own, so the config file and `next_check` refuse
  /// the same strings for the same reasons (F-4).
  Duration {
    key: &'static str,
    raw: String,
    fault: SpanFault,
  },
  /// `command = []`, or `command = [""]`. There is nothing to spawn (F-41).
  EmptyCommand,
  /// A duration that parsed but is zero or negative. A zero timeout fails every
  /// exchange and a zero poll is a busy loop, so neither is a configuration the
  /// host can honour (`design.md:1744`).
  NonPositive { key: &'static str },
  /// `ingress.path = ""`. An unusable value is not representable past this
  /// boundary, the same argument `EmptyCommand` rests on for the backend's
  /// command (`design.md` §5.2).
  EmptyPath { key: &'static str },
}

impl BackendError {
  /// The requirement this kind of refusal names (SPEC-001/R-59; `design.md`
  /// §5.2.3). `PipeMissing` and `Io` name R-45: no requirement names either
  /// failure, and R-45 governs what the host does with one.
  #[must_use]
  pub fn requirement(&self) -> Requirement {
    match self {
      Self::Spawn(_) => Requirement::R44,
      Self::Timeout { .. } => Requirement::R41,
      Self::ExitStatus { .. } => Requirement::R40,
      Self::OutputTooLarge { .. } => Requirement::R43,
      Self::PipeMissing | Self::Io(_) => Requirement::R45,
      Self::Protocol(inner) => inner.requirement(),
    }
  }

  /// The side this kind of refusal's cause lies on (SPEC-001/R-59).
  #[must_use]
  pub fn fault(&self) -> AtFault {
    match self {
      Self::Spawn(_) => AtFault::Configuration,
      Self::Timeout { .. } | Self::ExitStatus { .. } | Self::OutputTooLarge { .. } => {
        AtFault::Backend
      }
      Self::PipeMissing => AtFault::Host,
      Self::Io(_) => AtFault::Environment,
      Self::Protocol(inner) => inner.fault(),
    }
  }
}

impl CleanupFailure {
  /// The requirement this kind of refusal names (SPEC-001/R-59).
  #[must_use]
  pub fn requirement(&self) -> Requirement {
    match self {
      Self::TimedOut { .. } | Self::Io(_) => Requirement::R48,
    }
  }

  /// The side this kind of refusal's cause lies on (SPEC-001/R-59).
  #[must_use]
  pub fn fault(&self) -> AtFault {
    match self {
      Self::TimedOut { .. } => AtFault::Backend,
      Self::Io(_) => AtFault::Environment,
    }
  }
}

impl StateError {
  /// The requirement this kind of refusal names (SPEC-001/R-59).
  #[must_use]
  pub fn requirement(&self) -> Requirement {
    match self {
      Self::NoOutstandingView { .. } | Self::StaleViewId { .. } => Requirement::R32,
    }
  }

  /// The side this kind of refusal's cause lies on (SPEC-001/R-59).
  #[must_use]
  pub fn fault(&self) -> AtFault {
    match self {
      Self::NoOutstandingView { .. } | Self::StaleViewId { .. } => AtFault::Host,
    }
  }
}

impl fmt::Display for StateError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NoOutstandingView { named } => {
        write!(
          f,
          "no interaction is outstanding, so {} answers nothing",
          named.as_str()
        )
      }
      Self::StaleViewId { named, outstanding } => {
        write!(
          f,
          "{} is superseded; the outstanding interaction is {}",
          named.as_str(),
          outstanding.as_str()
        )
      }
    }
  }
}

impl fmt::Display for ConfigError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Read(inner) => write!(f, "configuration could not be read: {inner}"),
      Self::Syntax(inner) => write!(f, "configuration is not valid: {inner}"),
      Self::Duration { key, raw, fault } => {
        write!(
          f,
          "{key} = \"{raw}\" is not a duration this host can resolve: {fault}"
        )
      }
      Self::EmptyCommand => write!(
        f,
        "backend.command names no program, so there is nothing to spawn"
      ),
      Self::NonPositive { key } => write!(f, "{key} must be greater than zero"),
      Self::EmptyPath { key } => write!(f, "{key} must not be empty"),
    }
  }
}

impl std::error::Error for StateError {}

impl std::error::Error for ConfigError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Read(inner) => Some(inner),
      Self::Syntax(inner) => Some(inner),
      Self::Duration { fault, .. } => Some(fault),
      Self::EmptyCommand | Self::NonPositive { .. } | Self::EmptyPath { .. } => None,
    }
  }
}

impl fmt::Display for BackendError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Spawn(inner) => write!(f, "backend could not be spawned: {inner}"),
      Self::Timeout { after } => {
        write!(f, "backend did not respond within {}ms", after.as_millis())
      }
      Self::ExitStatus { code: Some(code) } => write!(f, "backend exited with status {code}"),
      Self::ExitStatus { code: None } => write!(f, "backend was terminated by a signal"),
      Self::OutputTooLarge { limit } => {
        write!(f, "backend wrote more than {limit} bytes to stdout")
      }
      Self::PipeMissing => write!(f, "a stdio handle was absent after spawn"),
      Self::Io(inner) => write!(f, "backend I/O failed: {inner}"),
      Self::Protocol(inner) => write!(f, "backend response rejected: {inner}"),
    }
  }
}

impl fmt::Display for CleanupFailure {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::TimedOut { after } => {
        write!(
          f,
          "backend was not disposed of within {}ms",
          after.as_millis()
        )
      }
      Self::Io(inner) => write!(f, "disposing of the backend failed: {inner}"),
    }
  }
}

impl std::error::Error for BackendError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Spawn(inner) | Self::Io(inner) => Some(inner),
      Self::Protocol(inner) => Some(inner),
      Self::Timeout { .. }
      | Self::ExitStatus { .. }
      | Self::OutputTooLarge { .. }
      | Self::PipeMissing => None,
    }
  }
}

impl std::error::Error for CleanupFailure {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Io(inner) => Some(inner),
      Self::TimedOut { .. } => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use std::time::Duration;

  use goad_semantics::error::{AtFault, ProtocolError};
  use goad_semantics::protocol::canonical::ViewId;

  use super::{BackendError, CleanupFailure, StateError};

  /// `design.md` §5.2.3's table, row for row, spelled as the table spells it
  /// and as stratum 1's tables do. Copied from the table, not from the code.
  /// Exhaustive, so a new variant has no row until one is decided. `Protocol`
  /// delegates: its row is whatever the wrapped error answers.
  fn backend_row(error: &BackendError) -> (String, AtFault) {
    match error {
      BackendError::Spawn(_) => ("R-44".to_owned(), AtFault::Configuration),
      BackendError::Timeout { .. } => ("R-41".to_owned(), AtFault::Backend),
      BackendError::ExitStatus { .. } => ("R-40".to_owned(), AtFault::Backend),
      BackendError::OutputTooLarge { .. } => ("R-43".to_owned(), AtFault::Backend),
      BackendError::PipeMissing => ("R-45".to_owned(), AtFault::Host),
      BackendError::Io(_) => ("R-45".to_owned(), AtFault::Environment),
      BackendError::Protocol(inner) => (inner.requirement().to_string(), inner.fault()),
    }
  }

  fn cleanup_row(error: &CleanupFailure) -> (String, AtFault) {
    match error {
      CleanupFailure::TimedOut { .. } => ("R-48".to_owned(), AtFault::Backend),
      CleanupFailure::Io(_) => ("R-48".to_owned(), AtFault::Environment),
    }
  }

  fn state_row(error: &StateError) -> (String, AtFault) {
    match error {
      StateError::NoOutstandingView { .. } | StateError::StaleViewId { .. } => {
        ("R-32".to_owned(), AtFault::Host)
      }
    }
  }

  fn io_error() -> std::io::Error {
    std::io::Error::other("refused")
  }

  /// One instance per row, and more where one row covers distinct shapes:
  /// `ExitStatus` with and without a code, and `Protocol` wrapping two errors
  /// whose answers differ, so the delegation is seen to follow the inner one.
  fn every_backend_error() -> Vec<BackendError> {
    vec![
      BackendError::Spawn(io_error()),
      BackendError::Timeout {
        after: Duration::from_secs(5),
      },
      BackendError::ExitStatus { code: Some(1) },
      BackendError::ExitStatus { code: None },
      BackendError::OutputTooLarge { limit: 1024 },
      BackendError::PipeMissing,
      BackendError::Io(io_error()),
      BackendError::Protocol(ProtocolError::MissingField { field: "view" }),
      BackendError::Protocol(ProtocolError::NestedHints {
        at: "view.options[0].fields[0]".to_owned(),
      }),
    ]
  }

  fn every_cleanup_failure() -> Vec<CleanupFailure> {
    vec![
      CleanupFailure::TimedOut {
        after: Duration::from_secs(2),
      },
      CleanupFailure::Io(io_error()),
    ]
  }

  fn every_state_error() -> Vec<StateError> {
    vec![
      StateError::NoOutstandingView {
        named: ViewId::new("v-1"),
      },
      StateError::StaleViewId {
        named: ViewId::new("v-1"),
        outstanding: ViewId::new("v-2"),
      },
    ]
  }

  #[test]
  fn every_backend_error_names_a_requirement_and_a_side() {
    for error in every_backend_error() {
      assert_eq!(
        (error.requirement().to_string(), error.fault()),
        backend_row(&error),
        "`{error}` disagrees with its row"
      );
    }
  }

  #[test]
  fn every_cleanup_failure_names_a_requirement_and_a_side() {
    for error in every_cleanup_failure() {
      assert_eq!(
        (error.requirement().to_string(), error.fault()),
        cleanup_row(&error),
        "`{error}` disagrees with its row"
      );
    }
  }

  #[test]
  fn every_state_error_names_a_requirement_and_a_side() {
    for error in every_state_error() {
      assert_eq!(
        (error.requirement().to_string(), error.fault()),
        state_row(&error),
        "`{error}` disagrees with its row"
      );
    }
  }
}

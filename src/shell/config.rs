//! The TOML the host is started with — `design.md` §5.2's Config block.
//!
//! Brief §5's three values and nothing else (the OQ-4 decision), parsed once at
//! startup and immutable afterwards. There is no hot reload: `design.md:1236`
//! makes a malformed or missing config fatal at construction, because there is
//! no backend to run without one.
//!
//! No module-level `#![deny(clippy::arithmetic_side_effects)]` here. The lint
//! follows the data, not the directory (D53 as amended), and a config file is
//! the user's own — nothing in this module computes over anything a backend
//! chose.

use std::path::Path;

use serde::Deserialize;

use crate::shell::error::ConfigError;

/// The parsed form.
///
/// Durations resolve at load, so nothing downstream carries an unparsed string.
/// The two are different types on purpose: `timeout` is what
/// `ProcessBackend::new` and tokio take, and `default_poll` is what
/// `semantics::schedule::resolve` takes. Converting here is what
/// `schedule.rs`'s own doc comment means by "at the config boundary".
#[derive(Debug)]
pub struct Config {
  pub backend: BackendConfig,
  pub schedule: ScheduleConfig,
}

#[derive(Debug)]
pub struct BackendConfig {
  /// An argument vector, never a shell string: no quoting rules and no
  /// injection surface, and it is what makes `["bash", "./backend.sh"]` work
  /// without a shebang (R-36).
  pub command: Vec<String>,
  pub timeout: std::time::Duration,
}

#[derive(Debug)]
pub struct ScheduleConfig {
  pub default_poll: jiff::SignedDuration,
}

/// The file as written, before anything is checked.
///
/// The same wire/canonical split `semantics` uses, for the same reason: the
/// permissive form is what serde can express, and the checks that make a value
/// usable do not fit in a `Deserialize`. `EmptyCommand` spelled as a
/// deserialization failure would name the wrong subject.
#[derive(Deserialize)]
struct File {
  backend: FileBackend,
  schedule: FileSchedule,
}

#[derive(Deserialize)]
struct FileBackend {
  command: Vec<String>,
  timeout: String,
}

#[derive(Deserialize)]
struct FileSchedule {
  default_poll: String,
}

impl Config {
  /// Read and check a configuration file.
  ///
  /// # Errors
  ///
  /// `ConfigError::Read` if the file is missing or unreadable; otherwise
  /// whatever [`Config::parse`] reports.
  pub fn load(path: &Path) -> Result<Self, ConfigError> {
    let text = std::fs::read_to_string(path).map_err(ConfigError::Read)?;
    Self::parse(&text)
  }

  /// Check a configuration that has already been read.
  ///
  /// Separate from `load` because every rejection this type makes is about the
  /// text rather than the file, so the tests that state them need no filesystem.
  ///
  /// # Errors
  ///
  /// `Syntax` if the text is not this shape; `Duration` if a duration string is
  /// not one the host can resolve; `EmptyCommand` and `NonPositive` for the two
  /// values that parse and still cannot be honoured.
  pub fn parse(text: &str) -> Result<Self, ConfigError> {
    let file: File = toml::from_str(text).map_err(|error| ConfigError::Syntax(Box::new(error)))?;
    if file.backend.command.is_empty() {
      return Err(ConfigError::EmptyCommand);
    }
    Ok(Self {
      backend: BackendConfig {
        command: file.backend.command,
        timeout: unsigned("backend.timeout", &file.backend.timeout)?,
      },
      schedule: ScheduleConfig {
        default_poll: signed("schedule.default_poll", &file.schedule.default_poll)?,
      },
    })
  }
}

/// One duration string, in `next_check`'s own grammar, that must also be usable.
///
/// The two jiff calls restate `semantics::schedule`'s conversion deliberately —
/// user decision 2026-09-03, `plan-log.md`. What must not diverge is the
/// *grammar*, which is jiff's rather than this crate's; sharing the code would
/// have meant either a `ScheduleError` naming a config key, or a public helper
/// in stratum 1 that this phase's Surfaces do not reach. Config admits only the
/// relative form: `next_check`'s absolute-instant branch is not a duration.
///
/// The positivity check is not a fallout of parsing. Both `"0s"` and `"-1s"`
/// parse — measured, `notes.md` PHASE-07 — so each rejection EX-1 asks for is a
/// check that had to be written.
fn signed(key: &'static str, raw: &str) -> Result<jiff::SignedDuration, ConfigError> {
  let named = |detail| ConfigError::Duration {
    key,
    raw: raw.to_owned(),
    detail,
  };
  let span = raw.parse::<jiff::Span>().map_err(named)?;
  let resolved = span
    .to_duration(jiff::SpanRelativeTo::days_are_24_hours())
    .map_err(named)?;
  if resolved.is_zero() || resolved.is_negative() {
    return Err(ConfigError::NonPositive { key });
  }
  Ok(resolved)
}

/// The same, for the one value whose consumer is tokio rather than jiff.
///
/// The conversion runs *after* the positivity check, so the only way it can fail
/// is a magnitude jiff can represent and `std` cannot — which is a duration the
/// host cannot honour, and is reported as one rather than silently clamped.
fn unsigned(key: &'static str, raw: &str) -> Result<std::time::Duration, ConfigError> {
  let resolved = signed(key, raw)?;
  std::time::Duration::try_from(resolved).map_err(|detail| ConfigError::Duration {
    key,
    raw: raw.to_owned(),
    detail,
  })
}

#[cfg(test)]
mod tests {
  use super::Config;
  use crate::shell::error::ConfigError;

  /// The design's own example, `design.md:1134`.
  const GOOD: &str = r#"
[backend]
command = ["deno", "run", "-A", "./backend.ts"]
timeout = "5s"

[schedule]
default_poll = "30m"
"#;

  fn rejection(text: &str) -> ConfigError {
    match Config::parse(text) {
      Err(error) => error,
      Ok(config) => panic!("accepted, as {config:?}"),
    }
  }

  // ---- EX-1: the three values ----

  #[test]
  fn the_design_s_own_example_loads() {
    let config = Config::parse(GOOD).expect("the example in the design must load");
    assert_eq!(
      config.backend.command,
      ["deno", "run", "-A", "./backend.ts"]
    );
    assert_eq!(config.backend.timeout, std::time::Duration::from_secs(5));
    assert_eq!(
      config.schedule.default_poll,
      jiff::SignedDuration::from_mins(30)
    );
  }

  #[test]
  fn a_configuration_is_read_from_a_file_and_a_missing_one_says_so() {
    let path = std::env::temp_dir().join(format!("goad-config-{}.toml", std::process::id()));
    std::fs::write(&path, GOOD).expect("the temp directory must be writable");
    let loaded = Config::load(&path).expect("the file just written must load");
    assert_eq!(loaded.backend.timeout, std::time::Duration::from_secs(5));

    std::fs::remove_file(&path).expect("the file just read must be removable");
    match Config::load(&path) {
      Err(ConfigError::Read(_)) => (),
      Err(other) => panic!("a missing file was refused as {other}"),
      Ok(config) => panic!("a missing file loaded, as {config:?}"),
    }
  }

  // ---- VT-2: one case per EX-1 rejection clause, each naming its error ----

  #[test]
  fn a_missing_section_is_refused_by_the_parser_rather_than_by_a_check() {
    let text = GOOD.replace("[schedule]", "[schedul]");
    assert!(
      matches!(rejection(&text), ConfigError::Syntax(_)),
      "a missing section was not refused as a syntax error: {}",
      rejection(&text)
    );
  }

  #[test]
  fn an_empty_command_is_rejected_because_there_is_nothing_to_spawn() {
    let text = GOOD.replace(r#"["deno", "run", "-A", "./backend.ts"]"#, "[]");
    assert!(
      matches!(rejection(&text), ConfigError::EmptyCommand),
      "an empty command was not rejected as such: {}",
      rejection(&text)
    );
  }

  #[test]
  fn a_zero_timeout_is_rejected_because_it_fails_every_exchange() {
    let text = GOOD.replace(r#"timeout = "5s""#, r#"timeout = "0s""#);
    assert!(
      matches!(
        rejection(&text),
        ConfigError::NonPositive {
          key: "backend.timeout"
        }
      ),
      "a zero timeout was not rejected as such: {}",
      rejection(&text)
    );
  }

  #[test]
  fn a_zero_default_poll_is_rejected_because_it_is_a_busy_loop() {
    let text = GOOD.replace(r#"default_poll = "30m""#, r#"default_poll = "0s""#);
    assert!(
      matches!(
        rejection(&text),
        ConfigError::NonPositive {
          key: "schedule.default_poll"
        }
      ),
      "a zero default poll was not rejected as such: {}",
      rejection(&text)
    );
  }
}

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

use crate::semantics::schedule::parse_span;
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
  pub command: Command,
  pub timeout: std::time::Duration,
}

/// What to spawn: a program and its arguments.
///
/// An argument vector, never a shell string: no quoting rules and no injection
/// surface, and it is what makes `["bash", "./backend.sh"]` work without a
/// shebang (R-36). Split rather than kept as one `Vec` so that the empty
/// command — the one argv with nothing to spawn — is not representable past
/// this boundary, and the transport has no `else` arm to report it in the
/// backend's voice (F-3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
  pub program: String,
  pub arguments: Vec<String>,
}

impl Command {
  pub fn new(program: impl Into<String>, arguments: Vec<String>) -> Self {
    Self {
      program: program.into(),
      arguments,
    }
  }

  /// `None` for the empty vector, which is `EmptyCommand`'s case.
  fn from_argv(argv: Vec<String>) -> Option<Self> {
    let mut argv = argv.into_iter();
    let program = argv.next()?;
    Some(Self::new(program, argv.collect()))
  }
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
///
/// `deny_unknown_fields` throughout, and that is the opposite of the wire's
/// rule on purpose: I10's permissiveness is for a backend written against a
/// newer host, and a config file is the user's own, read once, with its author
/// at the keyboard. A key the host does not read is a mistake to report, and
/// `toml` reports it by name and line (F-5).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
  backend: FileBackend,
  schedule: FileSchedule,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileBackend {
  command: Vec<String>,
  timeout: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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
    let command = Command::from_argv(file.backend.command).ok_or(ConfigError::EmptyCommand)?;
    Ok(Self {
      backend: BackendConfig {
        command,
        timeout: unsigned("backend.timeout", &file.backend.timeout)?,
      },
      schedule: ScheduleConfig {
        default_poll: signed("schedule.default_poll", &file.schedule.default_poll)?,
      },
    })
  }
}

/// One duration string, in the product's one grammar, that must also be usable.
///
/// The grammar is `schedule::parse_span`'s — shared, not restated, so the
/// config file and `next_check` refuse the same strings for the same reasons
/// (F-4). Config admits only the relative form: `next_check`'s
/// absolute-instant branch is not a duration.
///
/// The positivity check is not a fallout of parsing. Both `"0s"` and `"-1s"`
/// parse — measured, `notes.md` PHASE-07 — so each rejection EX-1 asks for is a
/// check that had to be written.
fn signed(key: &'static str, raw: &str) -> Result<jiff::SignedDuration, ConfigError> {
  let resolved = parse_span(raw).map_err(|fault| ConfigError::Duration {
    key,
    raw: raw.to_owned(),
    fault,
  })?;
  if resolved.is_zero() || resolved.is_negative() {
    return Err(ConfigError::NonPositive { key });
  }
  Ok(resolved)
}

/// The same, for the one value whose consumer is tokio rather than jiff.
///
/// `std::time::Duration` holds every non-negative `SignedDuration`, so after the
/// positivity check the conversion can fail only for a value that check has
/// already refused — which is why its failure is reported as that refusal and
/// not as a second error nothing can reach.
fn unsigned(key: &'static str, raw: &str) -> Result<std::time::Duration, ConfigError> {
  let resolved = signed(key, raw)?;
  std::time::Duration::try_from(resolved).or(Err(ConfigError::NonPositive { key }))
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
    assert_eq!(config.backend.command.program, "deno");
    assert_eq!(
      config.backend.command.arguments,
      ["run", "-A", "./backend.ts"]
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

  /// F-5: a config file is the user's own, read once, with its author at the
  /// keyboard — so an unknown key is a mistake to report, not a newer backend
  /// to tolerate (which is what I10's permissiveness is about, on the wire).
  /// Brief §5's own illustrative file carries `socket` and `[logging]`, both
  /// outside this slice; a user who copies it must hear that they did nothing.
  #[test]
  fn an_unknown_key_is_refused_and_named() {
    for (text, key) in [
      (
        GOOD.replace(
          "timeout = \"5s\"",
          "timeout = \"5s\"\nsocket = \"/tmp/goad.sock\"",
        ),
        "socket",
      ),
      (format!("{GOOD}\n[logging]\nlevel = \"info\"\n"), "logging"),
      (
        GOOD.replace("default_poll", "poll_interval"),
        "poll_interval",
      ),
    ] {
      let refused = rejection(&text);
      assert!(
        matches!(&refused, ConfigError::Syntax(_)) && refused.to_string().contains(key),
        "unknown key `{key}` was not refused naming it: {refused}"
      );
    }
  }

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

  /// F-28: the grammar's own refusals, each surfacing as `Duration` naming
  /// the key, and the negative case as `NonPositive` — because `"-30m"` parses.
  #[test]
  fn a_duration_the_grammar_refuses_is_rejected_naming_the_key() {
    for (raw, what) in [
      ("1 month", "a calendar unit"),
      ("soon", "prose"),
      ("09:00:00", "a time of day"),
    ] {
      let text = GOOD.replace(r#"timeout = "5s""#, &format!(r#"timeout = "{raw}""#));
      let refused = rejection(&text);
      assert!(
        matches!(
          &refused,
          ConfigError::Duration { key: "backend.timeout", raw: found, .. } if found == raw
        ),
        "{what} was not refused as a duration fault naming the key: {refused}"
      );
    }
  }

  #[test]
  fn a_negative_duration_is_rejected_as_non_positive() {
    let text = GOOD.replace(r#"default_poll = "30m""#, r#"default_poll = "-30m""#);
    assert!(
      matches!(
        rejection(&text),
        ConfigError::NonPositive {
          key: "schedule.default_poll"
        }
      ),
      "a negative default poll was not rejected as such: {}",
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

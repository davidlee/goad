//! The TOML the host is started with — `design.md` §5.2's Config block.
//!
//! Brief §5's three values, plus the event-ingress key slice 004 adds, parsed
//! once at startup and immutable afterwards. There is no hot reload:
//! `design.md:1236` makes a malformed or missing config fatal at construction,
//! because there is no backend to run without one.
//!
//! No module-level `#![deny(clippy::arithmetic_side_effects)]` here. The lint
//! follows the data, not the directory (D53 as amended), and a config file is
//! the user's own — nothing in this module computes over anything a backend
//! chose.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::ConfigError;
use goad_semantics::schedule::parse_span;

/// Where the configuration is read from when a caller is given no path: the
/// XDG basedir rule, stated once for every caller rather than once per binary.
///
/// Pure over the environment it is handed, so the table below is a test rather
/// than a claim.
///
/// | environment | answer |
/// |---|---|
/// | `XDG_CONFIG_HOME` set and **absolute** | `$XDG_CONFIG_HOME/goad/config.toml` |
/// | `XDG_CONFIG_HOME` unset, empty or relative, `HOME` non-empty | `$HOME/.config/goad/config.toml` |
/// | neither names a directory | `None` — what an absent default means is the caller's to say |
///
/// `HOME` is used **as given** and is not required to be absolute: the XDG
/// basedir spec states the absoluteness rule for `XDG_CONFIG_HOME` and states
/// nothing of the kind for `HOME`, and a relative `HOME` is a broken
/// environment nothing here can repair and none of this should silently
/// reinterpret.
///
/// The environment arrives as a function rather than being read here:
/// `clippy.toml` disallows `std::env::var`, and a rule that takes its input as
/// an argument is testable without mutating the process.
pub fn default_path(env: &dyn Fn(&str) -> Option<OsString>) -> Option<PathBuf> {
  match env("XDG_CONFIG_HOME").filter(|value| PathBuf::from(value).is_absolute()) {
    Some(xdg) => Some(PathBuf::from(xdg).join("goad/config.toml")),
    None => match env("HOME") {
      Some(home) if !home.is_empty() => Some(PathBuf::from(home).join(".config/goad/config.toml")),
      _neither_names_a_directory => None,
    },
  }
}

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
  /// `None` means no listener: today's behaviour exactly (AC-7).
  pub ingress: Option<IngressConfig>,
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

  /// `None` for the empty vector and for an empty program, which are the two
  /// spellings of `EmptyCommand`'s case: neither names anything to spawn
  /// (F-41).
  fn from_argv(argv: Vec<String>) -> Option<Self> {
    let mut argv = argv.into_iter();
    let program = argv.next().filter(|program| !program.is_empty())?;
    Some(Self::new(program, argv.collect()))
  }
}

#[derive(Debug)]
pub struct ScheduleConfig {
  /// How often the host evaluates on its own account when nothing else
  /// prompts it. Any duration `parse_span`'s grammar accepts is valid here,
  /// including one shorter than the host's own minimum spacing between
  /// scheduled evaluations — refusing it would invent a rule the grammar
  /// itself does not carry (SPEC-001/R-21). A value below that spacing is
  /// honoured, unfloored, for the process's **first** scheduled evaluation;
  /// every scheduled evaluation after that is floored (SPEC-002/R-5).
  pub default_poll: jiff::SignedDuration,
}

/// Where the event-ingress socket listens — `design.md` §5.2. Absent from
/// `Config` means no listener is bound, which is today's behaviour exactly
/// (AC-7); nothing about `default_poll` or `timeout` changes because of it.
#[derive(Debug)]
pub struct IngressConfig {
  pub path: PathBuf,
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
  /// Absent unless the file writes `[ingress]` — the section is optional,
  /// unlike its siblings (AC-7).
  #[serde(default)]
  ingress: Option<FileIngress>,
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileIngress {
  path: String,
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
  /// not one the host can resolve; `EmptyCommand`, `EmptyPath` and
  /// `NonPositive` for the values that parse and still cannot be honoured.
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
      ingress: file.ingress.map(ingress_config).transpose()?,
    })
  }
}

/// `ingress.path` must be non-empty: an unusable value is not representable
/// past this boundary, the same argument `EmptyCommand` rests on.
fn ingress_config(file: FileIngress) -> Result<IngressConfig, ConfigError> {
  if file.path.is_empty() {
    return Err(ConfigError::EmptyPath {
      key: "ingress.path",
    });
  }
  Ok(IngressConfig {
    path: PathBuf::from(file.path),
  })
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
  use crate::error::ConfigError;

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

  /// Both spellings of nothing to spawn: no argv at all, and an argv whose
  /// program is the empty string (F-41). The second would otherwise load and
  /// fail at every exchange as a spawn error in the backend's voice.
  #[test]
  fn an_empty_command_is_rejected_because_there_is_nothing_to_spawn() {
    for empty in ["[]", r#"[""]"#, r#"["", "./backend.ts"]"#] {
      let text = GOOD.replace(r#"["deno", "run", "-A", "./backend.ts"]"#, empty);
      assert!(
        matches!(rejection(&text), ConfigError::EmptyCommand),
        "command {empty} was not rejected as empty: {}",
        rejection(&text)
      );
    }
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

  // ---- VT-1: the fourth value, `[ingress]` ----

  #[test]
  fn an_ingress_section_loads_with_its_path() {
    let text = format!("{GOOD}\n[ingress]\npath = \"./goad.sock\"\n");
    let config = Config::parse(&text).expect("a well-formed ingress section must load");
    let ingress = config.ingress.expect("the section was present");
    assert_eq!(ingress.path, std::path::PathBuf::from("./goad.sock"));
  }

  #[test]
  fn an_empty_ingress_path_is_refused() {
    let text = format!("{GOOD}\n[ingress]\npath = \"\"\n");
    assert!(
      matches!(
        rejection(&text),
        ConfigError::EmptyPath {
          key: "ingress.path"
        }
      ),
      "an empty ingress path was not rejected as such: {}",
      rejection(&text)
    );
  }

  #[test]
  fn an_unknown_key_inside_ingress_is_refused_and_named() {
    let text = format!("{GOOD}\n[ingress]\npath = \"./goad.sock\"\nmode = \"0600\"\n");
    let refused = rejection(&text);
    assert!(
      matches!(&refused, ConfigError::Syntax(_)) && refused.to_string().contains("mode"),
      "unknown key `mode` inside [ingress] was not refused naming it: {refused}"
    );
  }

  // ---- VT-10: AC-7's first half — absent means no listener ----

  #[test]
  fn with_no_ingress_section_ingress_is_none() {
    let config = Config::parse(GOOD).expect("the design's own example must load");
    assert!(
      config.ingress.is_none(),
      "no [ingress] section must yield ingress: None, not {:?}",
      config.ingress
    );
  }

  // ---- 005/PHASE-01/VT-1: the default configuration path's rows ----
  //
  // The same rows `crates/goad`'s `startup::arguments` tests assert through
  // its `[]` arm. They stay where they are: this tier asserts the rule, that
  // one asserts that the host still reaches it.

  /// One row of the environment, as `default_path` takes it.
  fn env_of<'a>(
    pairs: &'a [(&'a str, &'a str)],
  ) -> impl Fn(&str) -> Option<std::ffi::OsString> + 'a {
    move |name| {
      pairs
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, value)| std::ffi::OsString::from(*value))
    }
  }

  #[test]
  fn an_absolute_xdg_config_home_is_the_path() {
    assert_eq!(
      super::default_path(&env_of(&[
        ("XDG_CONFIG_HOME", "/xdg"),
        ("HOME", "/home/it")
      ])),
      Some("/xdg/goad/config.toml".into())
    );
  }

  #[test]
  fn a_relative_xdg_config_home_is_ignored_and_home_answers() {
    assert_eq!(
      super::default_path(&env_of(&[
        ("XDG_CONFIG_HOME", "relative"),
        ("HOME", "/home/it")
      ])),
      Some("/home/it/.config/goad/config.toml".into()),
      "the XDG basedir spec states the absoluteness rule; a relative value is not a path"
    );
  }

  /// The table's XDG row reads *unset, empty or relative* and the empty spelling
  /// had no case: an empty value is not an absolute path, so `HOME` answers —
  /// and `$XDG_CONFIG_HOME` prefixed onto nothing is never the answer.
  #[test]
  fn an_empty_xdg_config_home_is_ignored_and_home_answers() {
    assert_eq!(
      super::default_path(&env_of(&[("XDG_CONFIG_HOME", ""), ("HOME", "/home/it")])),
      Some("/home/it/.config/goad/config.toml".into())
    );
  }

  #[test]
  fn with_no_xdg_config_home_the_path_is_under_home() {
    assert_eq!(
      super::default_path(&env_of(&[("HOME", "/home/it")])),
      Some("/home/it/.config/goad/config.toml".into())
    );
  }

  #[test]
  fn an_empty_home_names_no_directory() {
    assert_eq!(
      super::default_path(&env_of(&[("HOME", "")])),
      None,
      "an empty HOME names no directory, so `/.config/goad/config.toml` is not the answer"
    );
  }

  #[test]
  fn with_neither_variable_there_is_no_default_path() {
    assert_eq!(super::default_path(&env_of(&[])), None);
  }
}

//! The command line, parsed — pure over the argument vector, so the table on
//! [`parse`] is a set of tests rather than a claim.
//!
//! Hand-rolled, as `goad-emit`'s `args.rs` is and for its reason: prior art
//! already in the workspace, and an argument parser no stratum needs would be
//! paid for in every gate run.

use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

use goad_shell::config::{self, Command};
use goad_shell::error::ConfigError;

const CONFIG: &str = "--config";
const EVENT: &str = "--event";
const TIMEOUT: &str = "--timeout";
/// Everything after it, in flag position, is the backend's command.
const SEPARATOR: &str = "--";

/// The argv form's timeout when `--timeout` is not given: the value
/// `exercisers/demo.toml` uses (`design.md` §5.2.1).
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// The flags that take a value, as a type, so [`split`] and the fold in
/// [`parse`] cannot disagree about which tokens consume the token after them
/// (`goad-emit`'s F-2).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Flag {
  Config,
  Event,
  Timeout,
}

impl Flag {
  const ALL: [Self; 3] = [Self::Config, Self::Event, Self::Timeout];

  /// The flag this token names, or `None` — which includes `-h`, `--help`,
  /// `--version` and the separator, none of which takes a value.
  fn of(token: &std::ffi::OsStr) -> Option<Self> {
    let text = token.to_str()?;
    Self::ALL.into_iter().find(|flag| flag.name() == text)
  }

  /// As written on the command line, which is how every message names it.
  fn name(self) -> &'static str {
    match self {
      Self::Config => CONFIG,
      Self::Event => EVENT,
      Self::Timeout => TIMEOUT,
    }
  }
}

/// One argument before the separator, with its position already resolved: a
/// token consumed as some flag's value is never a flag.
#[derive(Debug)]
enum Argument {
  /// A value-taking flag and the token after it — `None` when it was the last
  /// argument, which is [`UsageError::NoValue`].
  Flagged(Flag, Option<OsString>),
  /// A token in flag position: `-h`, `--help`, `--version`, or something this
  /// command line does not take.
  Bare(OsString),
}

/// The arguments before the first separator in flag position, paired, and
/// the tokens after it — `None` when there is no separator. The one place
/// that decides which tokens are values and which are the command's.
fn split(mut rest: impl Iterator<Item = OsString>) -> (Vec<Argument>, Option<Vec<OsString>>) {
  let mut arguments = Vec::new();
  while let Some(argument) = rest.next() {
    if argument == SEPARATOR {
      return (arguments, Some(rest.collect()));
    }
    arguments.push(match Flag::of(&argument) {
      Some(flag) => Argument::Flagged(flag, rest.next()),
      None => Argument::Bare(argument),
    });
  }
  (arguments, None)
}

/// What the arguments asked for. `Help` and `Version` are outcomes rather
/// than an early exit hidden in parsing, so `main` keeps its one exit-status
/// decision.
#[derive(Debug, PartialEq)]
pub(crate) enum Invocation {
  Check(Request),
  Help,
  Version,
}

/// One run's whole subject: where the backend's configuration comes from, and
/// the author's event files, in the order given. The files are paths here:
/// `main` reads and normalizes them.
#[derive(Debug, PartialEq)]
pub(crate) struct Request {
  pub(crate) source: Source,
  pub(crate) events: Vec<PathBuf>,
}

/// The two command forms of `design.md` §5.2.1.
#[derive(Debug, PartialEq)]
pub(crate) enum Source {
  /// The config form: `--config PATH`, or `None` for the host's default
  /// path, which `main` resolves, because resolving it reads the environment.
  File(Option<PathBuf>),
  /// The argv form: the command after the separator, and `--timeout` or its
  /// default.
  Argv { command: Command, timeout: Duration },
}

/// Why the arguments name no invocation. Each variant carries the flag or the
/// argument at fault, or is itself the combination at fault.
#[derive(Debug)]
pub(crate) enum UsageError {
  /// The flag was given with an empty value: there is no file at `""`.
  Empty(&'static str),
  /// The flag was the last argument, with nothing after it to be its value.
  NoValue(&'static str),
  /// The flag was given twice, where it takes one value. Which was meant is
  /// not guessed at.
  Repeated(&'static str),
  /// A value that is not UTF-8 where a `String` is wanted: `--timeout`'s, or
  /// a token of the command, which `config::Command` holds as `String`s.
  NotUtf8(&'static str),
  /// `--timeout`'s value, refused by the host's own rule for a usable timeout,
  /// `config::positive_duration`.
  Timeout(ConfigError),
  /// The separator with no program after it, or an empty one: refused by
  /// `config::Command::from_argv`, the host's own rule for an empty command.
  EmptyCommand,
  /// `--timeout` without the separator. In the config form a file supplies
  /// the timeout, whether named or defaulted.
  TimeoutWithoutCommand,
  /// `--config` with the separator: two configuration sources, and which was
  /// meant is not guessed at.
  ConfigWithCommand,
  /// Something beginning with `-` that is none of the flags.
  Unknown(String),
  /// A bare argument before the separator. The command goes after it.
  Positional(String),
}

/// The command line, as a value.
///
/// `argv` is `std::env::args_os()` **whole**, program name included: the skip
/// lives here, inside the function the table tests.
///
/// | arguments | behaviour |
/// |---|---|
/// | `-h` or `--help` in flag position, before the separator | [`Invocation::Help`], whatever else is wrong with the line |
/// | `--version` in flag position, before the separator | [`Invocation::Version`] |
/// | both `--help` and `--version` | [`Invocation::Help`], in either order |
/// | nothing | the config form, at the host's default path |
/// | `--config PATH` | the config form, at that path |
/// | `-- PROGRAM [ARG]...` | the argv form, through `config::Command::from_argv`; every token after the separator is the command's, `--help` included |
/// | `--timeout SPAN` with the separator | the argv form's timeout, judged by `config::positive_duration`; without it, five seconds |
/// | `--event FILE`, repeatable | the files, in the order given |
/// | a token after a value-taking flag | that flag's value, whatever it spells |
/// | `--timeout` without the separator | [`UsageError::TimeoutWithoutCommand`] |
/// | `--config` with the separator | [`UsageError::ConfigWithCommand`] |
/// | the separator with no program, or an empty one | [`UsageError::EmptyCommand`] |
/// | `--config` or `--timeout` given twice | [`UsageError::Repeated`] |
/// | a value-taking flag with nothing after it | [`UsageError::NoValue`] |
/// | anything else beginning with `-` | [`UsageError::Unknown`] |
/// | anything else before the separator | [`UsageError::Positional`] |
///
/// # Errors
///
/// A [`UsageError`] naming what is wrong. Nothing here reads an environment,
/// a clock or a file.
pub(crate) fn parse(argv: impl Iterator<Item = OsString>) -> Result<Invocation, UsageError> {
  let (arguments, command) = split(argv.skip(1));
  let in_flag_position = |wanted: &str| {
    arguments.iter().any(|argument| match argument {
      Argument::Bare(bare) => bare == wanted,
      Argument::Flagged(..) => false,
    })
  };
  if in_flag_position("-h") || in_flag_position("--help") {
    return Ok(Invocation::Help);
  }
  if in_flag_position("--version") {
    return Ok(Invocation::Version);
  }

  let mut config: Option<PathBuf> = None;
  let mut timeout: Option<Duration> = None;
  let mut events: Vec<PathBuf> = Vec::new();

  for argument in arguments {
    let (flag, given) = match argument {
      Argument::Flagged(flag, given) => (flag, given.ok_or(UsageError::NoValue(flag.name()))?),
      Argument::Bare(bare) => return Err(stray(&bare)),
    };
    let name = flag.name();
    match flag {
      Flag::Config => once(&mut config, name, path(name, given)?)?,
      Flag::Event => events.push(path(name, given)?),
      Flag::Timeout => once(&mut timeout, name, duration(&given)?)?,
    }
  }

  let source = match (config, command) {
    (Some(_), Some(_)) => return Err(UsageError::ConfigWithCommand),
    (config, None) => match timeout {
      Some(_) => return Err(UsageError::TimeoutWithoutCommand),
      None => Source::File(config),
    },
    (None, Some(command)) => Source::Argv {
      command: program(command)?,
      timeout: timeout.unwrap_or(DEFAULT_TIMEOUT),
    },
  };
  Ok(Invocation::Check(Request { source, events }))
}

/// A flag's value as a non-empty path. No UTF-8 requirement: a path need not
/// be UTF-8.
fn path(flag: &'static str, raw: OsString) -> Result<PathBuf, UsageError> {
  if raw.is_empty() {
    return Err(UsageError::Empty(flag));
  }
  Ok(PathBuf::from(raw))
}

/// `--timeout`'s value, by the host's own rule for a usable timeout.
fn duration(raw: &OsString) -> Result<Duration, UsageError> {
  let text = raw.to_str().ok_or(UsageError::NotUtf8(TIMEOUT))?;
  config::positive_duration(TIMEOUT, text).map_err(UsageError::Timeout)
}

/// The tokens after the separator, by the host's own rule for an empty
/// command.
fn program(tokens: Vec<OsString>) -> Result<Command, UsageError> {
  let argv = tokens
    .into_iter()
    .map(|token| token.into_string().or(Err(UsageError::NotUtf8(SEPARATOR))))
    .collect::<Result<Vec<String>, UsageError>>()?;
  Command::from_argv(argv).ok_or(UsageError::EmptyCommand)
}

/// Fills a slot that must be filled at most once.
fn once<T>(slot: &mut Option<T>, flag: &'static str, filling: T) -> Result<(), UsageError> {
  if slot.is_some() {
    return Err(UsageError::Repeated(flag));
  }
  *slot = Some(filling);
  Ok(())
}

/// An argument this command line does not take, told apart by its leading
/// `-` so the message can say *unknown flag* for a typo.
fn stray(argument: &OsString) -> UsageError {
  let shown = argument.to_string_lossy().into_owned();
  if shown.starts_with('-') {
    UsageError::Unknown(shown)
  } else {
    UsageError::Positional(shown)
  }
}

#[cfg(test)]
mod tests {
  use std::ffi::OsString;
  use std::path::PathBuf;
  use std::time::Duration;

  use goad_shell::config::Command;
  use goad_shell::error::ConfigError;

  use super::{Invocation, Request, Source, UsageError, parse};

  /// `parse` takes `args_os()` whole, program name included.
  fn parsed(arguments: &[&str]) -> Result<Invocation, UsageError> {
    let argv =
      std::iter::once(OsString::from("goad-check")).chain(arguments.iter().map(OsString::from));
    parse(argv)
  }

  fn checked(arguments: &[&str]) -> Request {
    match parsed(arguments).expect("these arguments are well-formed") {
      Invocation::Check(request) => request,
      other => panic!("expected a check, found {other:?}"),
    }
  }

  fn refused(arguments: &[&str]) -> UsageError {
    parsed(arguments).expect_err("these arguments are not well-formed")
  }

  fn argv(program: &str, arguments: &[&str]) -> Command {
    Command::new(program, arguments.iter().map(ToString::to_string).collect())
  }

  #[test]
  fn no_arguments_is_the_config_form_at_the_default_path() {
    assert_eq!(
      checked(&[]),
      Request {
        source: Source::File(None),
        events: Vec::new(),
      }
    );
  }

  #[test]
  fn config_names_the_file_to_load() {
    assert_eq!(
      checked(&["--config", "./goad.toml"]).source,
      Source::File(Some(PathBuf::from("./goad.toml")))
    );
  }

  #[test]
  fn the_command_after_the_separator_is_the_argv_form_with_a_five_second_timeout() {
    assert_eq!(
      checked(&["--", "bash", "./backend.sh"]).source,
      Source::Argv {
        command: argv("bash", &["./backend.sh"]),
        timeout: Duration::from_secs(5),
      }
    );
  }

  #[test]
  fn timeout_sets_the_argv_forms_timeout() {
    assert_eq!(
      checked(&["--timeout", "2s", "--", "bash", "./backend.sh"]).source,
      Source::Argv {
        command: argv("bash", &["./backend.sh"]),
        timeout: Duration::from_secs(2),
      }
    );
  }

  /// The order is the request plan's (`design.md` §5.2.2, step 5), so it is
  /// the content of the claim: three files, out of lexical order.
  #[test]
  fn events_are_kept_in_the_order_given() {
    assert_eq!(
      checked(&[
        "--event", "b.json", "--event", "c.json", "--event", "a.json"
      ])
      .events,
      [
        PathBuf::from("b.json"),
        PathBuf::from("c.json"),
        PathBuf::from("a.json"),
      ]
    );
  }

  #[test]
  fn a_timeout_with_config_is_refused() {
    assert!(matches!(
      refused(&["--config", "./goad.toml", "--timeout", "2s"]),
      UsageError::TimeoutWithoutCommand
    ));
  }

  /// The config form with no `--config` reads the default path's file, which
  /// supplies the timeout as a named one does.
  #[test]
  fn a_timeout_with_neither_config_nor_a_command_is_refused() {
    assert!(matches!(
      refused(&["--timeout", "2s"]),
      UsageError::TimeoutWithoutCommand
    ));
  }

  #[test]
  fn config_with_a_command_is_refused() {
    assert!(matches!(
      refused(&["--config", "./goad.toml", "--", "bash", "./backend.sh"]),
      UsageError::ConfigWithCommand
    ));
  }

  /// `parse_span` alone accepts a zero span; the host's rule does not.
  #[test]
  fn a_zero_timeout_is_refused() {
    assert!(matches!(
      refused(&["--timeout", "0s", "--", "bash"]),
      UsageError::Timeout(ConfigError::NonPositive { key: "--timeout" })
    ));
  }

  #[test]
  fn a_negative_timeout_is_refused() {
    assert!(matches!(
      refused(&["--timeout", "-1s", "--", "bash"]),
      UsageError::Timeout(ConfigError::NonPositive { key: "--timeout" })
    ));
  }

  #[test]
  fn a_timeout_that_is_not_a_span_is_refused() {
    assert!(matches!(
      refused(&["--timeout", "soon", "--", "bash"]),
      UsageError::Timeout(ConfigError::Duration {
        key: "--timeout",
        ..
      })
    ));
  }

  #[test]
  fn an_empty_command_is_refused() {
    assert!(matches!(refused(&["--"]), UsageError::EmptyCommand));
  }

  #[test]
  fn an_empty_program_is_refused() {
    assert!(matches!(
      refused(&["--", "", "./backend.sh"]),
      UsageError::EmptyCommand
    ));
  }

  #[test]
  fn help_is_help_wherever_it_appears_before_the_separator() {
    for arguments in [
      &["-h"][..],
      &["--help"][..],
      &["--config", "./goad.toml", "--help"][..],
      &["--timeout", "2s", "--help"][..],
    ] {
      assert_eq!(parsed(arguments).expect("help parses"), Invocation::Help);
    }
  }

  #[test]
  fn version_is_version() {
    assert_eq!(
      parsed(&["--version"]).expect("version parses"),
      Invocation::Version
    );
  }

  #[test]
  fn help_wins_over_version_in_either_order() {
    for arguments in [&["--help", "--version"][..], &["--version", "--help"][..]] {
      assert_eq!(parsed(arguments).expect("help parses"), Invocation::Help);
    }
  }

  /// A backend may take `--help` itself; after the separator it is the
  /// command's argument, not a question to the checker.
  #[test]
  fn every_token_after_the_separator_is_the_commands() {
    assert_eq!(
      checked(&["--", "./backend", "--help", "--version", "--config"]).source,
      Source::Argv {
        command: argv("./backend", &["--help", "--version", "--config"]),
        timeout: Duration::from_secs(5),
      }
    );
  }

  #[test]
  fn a_help_token_in_value_position_is_a_value() {
    assert_eq!(
      checked(&["--config", "--help"]).source,
      Source::File(Some(PathBuf::from("--help")))
    );
  }

  #[test]
  fn an_unknown_flag_is_refused_naming_it() {
    match refused(&["--config=./goad.toml"]) {
      UsageError::Unknown(flag) => assert_eq!(flag, "--config=./goad.toml"),
      other => panic!("expected Unknown, found {other:?}"),
    }
  }

  #[test]
  fn a_bare_argument_before_the_separator_is_refused() {
    match refused(&["./backend.sh"]) {
      UsageError::Positional(argument) => assert_eq!(argument, "./backend.sh"),
      other => panic!("expected Positional, found {other:?}"),
    }
  }

  #[test]
  fn a_flag_with_no_value_is_refused() {
    assert!(matches!(
      refused(&["--event"]),
      UsageError::NoValue("--event")
    ));
  }

  #[test]
  fn an_empty_config_or_event_path_is_refused() {
    assert!(matches!(
      refused(&["--config", ""]),
      UsageError::Empty("--config")
    ));
    assert!(matches!(
      refused(&["--event", ""]),
      UsageError::Empty("--event")
    ));
  }

  #[test]
  fn a_repeated_config_is_refused() {
    assert!(matches!(
      refused(&["--config", "a.toml", "--config", "b.toml"]),
      UsageError::Repeated("--config")
    ));
  }

  #[test]
  fn a_repeated_timeout_is_refused() {
    assert!(matches!(
      refused(&["--timeout", "2s", "--timeout", "3s", "--", "bash"]),
      UsageError::Repeated("--timeout")
    ));
  }

  #[test]
  fn a_command_token_that_is_not_utf8_is_refused() {
    use std::os::unix::ffi::OsStringExt as _;
    let argv = [
      OsString::from("goad-check"),
      OsString::from("--"),
      OsString::from_vec(vec![0xff, 0xfe]),
    ];
    assert!(matches!(
      parse(argv.into_iter()).expect_err("invalid UTF-8 is refused"),
      UsageError::NotUtf8("--")
    ));
  }
}

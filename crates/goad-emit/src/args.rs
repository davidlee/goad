//! The command line, parsed — pure over the argument vector, so the table on
//! [`parse`] is a set of tests rather than a claim.
//!
//! Hand-rolled, for the reason `crates/goad/src/startup.rs` is (005/D-6):
//! four flags, prior art already in the workspace, and a dependency no
//! stratum needs is paid for in every gate run.

use std::ffi::OsString;
use std::path::PathBuf;

const SOURCE: &str = "--source";
const KIND: &str = "--kind";
const DATA: &str = "--data";
const SOCKET: &str = "--socket";

/// What the arguments asked for. `Help` and `Version` are outcomes rather
/// than an early exit hidden in parsing, so `main` keeps its one exit-code
/// decision — `startup::arguments`' own reason for the same shape.
#[derive(Debug, PartialEq)]
pub(crate) enum Invocation {
  Send(Request),
  Help,
  Version,
}

/// One invocation's whole subject: one envelope, and where to send it.
///
/// `socket` is `None` when no `--socket` was given, which is the signal to
/// consult the host's configuration — the *only* thing that consults it
/// (`design.md` §5.5).
#[derive(Debug, PartialEq)]
pub(crate) struct Request {
  pub(crate) source: String,
  pub(crate) kind: String,
  pub(crate) data: serde_json::Value,
  pub(crate) socket: Option<PathBuf>,
}

/// Why the arguments name no invocation. Every variant carries the flag or
/// the argument at fault, because a usage error that does not say which one
/// leaves the caller re-reading the whole line.
#[derive(Debug)]
pub(crate) enum UsageError {
  /// A required flag was not given at all.
  Missing(&'static str),
  /// The flag was given with an empty value. An empty `source` or `kind` is
  /// not an envelope `SPEC-003` §6.2 admits, and an empty socket path is
  /// unusable — the argument `config.rs` makes for `ingress.path`.
  Empty(&'static str),
  /// The flag was the last argument, with nothing after it to be its value.
  NoValue(&'static str),
  /// The flag was given twice. Not "the last wins": which was meant is a
  /// guess, and `startup::arguments` refuses the same guess for its own
  /// positional argument.
  Repeated(&'static str),
  /// The value is not UTF-8, where the envelope needs a `String`. `--socket`
  /// is exempt: a path need not be UTF-8.
  NotUtf8(&'static str),
  /// `--data` is parsed here rather than on the wire (005/D-5), so a
  /// malformed value is a local message instead of a round trip.
  NotJson {
    raw: String,
    fault: serde_json::Error,
  },
  /// Something beginning with `-` that is none of the five flags.
  Unknown(String),
  /// A bare argument. Emit takes none — the host takes a configuration path
  /// positionally and emit has no equivalent (`design.md` §5.5, F-6).
  Positional(String),
}

/// The command line, as a value.
///
/// `argv` is `std::env::args_os()` **whole**, program name included: the skip
/// lives here, inside the function the table tests, rather than at a call site
/// no test covers (`startup::arguments`' own reasoning).
///
/// | arguments | behaviour |
/// |---|---|
/// | `-h` or `--help`, anywhere | [`Invocation::Help`] — the usage block on stdout, exit 0. Checked before anything else, so a mistyped line can still ask for help |
/// | `--version`, anywhere | [`Invocation::Version`] — the package version, exit 0. `--help` wins if both appear |
/// | `--source S --kind K` | [`Invocation::Send`], `data` JSON `null`, `socket` `None` — the configuration is consulted for the path |
/// | `--data JSON` | parsed **locally** (005/D-5); a malformed value is [`UsageError::NotJson`] and no connection is made |
/// | `--socket PATH` | that path, and **no configuration is read at all** (AC-4) |
/// | a flag given twice | [`UsageError::Repeated`] — which was meant is not guessed at |
/// | a flag with nothing after it | [`UsageError::NoValue`] |
/// | `--source` or `--kind` absent | [`UsageError::Missing`] |
/// | any of the four with an empty value | [`UsageError::Empty`] |
/// | anything else beginning with `-` | [`UsageError::Unknown`], naming it. `--source=S` is one of these: values are separate arguments |
/// | anything else | [`UsageError::Positional`], naming it |
///
/// # Errors
///
/// A [`UsageError`] naming the flag or argument at fault. Nothing here reads
/// an environment, a clock, a file or a socket, so a usage error costs no
/// connection.
pub(crate) fn parse(argv: impl Iterator<Item = OsString>) -> Result<Invocation, UsageError> {
  let rest: Vec<OsString> = argv.skip(1).collect();
  if rest
    .iter()
    .any(|argument| argument == "-h" || argument == "--help")
  {
    return Ok(Invocation::Help);
  }
  if rest.iter().any(|argument| argument == "--version") {
    return Ok(Invocation::Version);
  }

  let mut source: Option<String> = None;
  let mut kind: Option<String> = None;
  let mut data: Option<serde_json::Value> = None;
  let mut socket: Option<PathBuf> = None;

  let mut arguments = rest.into_iter();
  while let Some(argument) = arguments.next() {
    match argument.to_str() {
      Some(SOURCE) => once(
        &mut source,
        SOURCE,
        text(SOURCE, &value(SOURCE, &mut arguments)?)?,
      )?,
      Some(KIND) => once(&mut kind, KIND, text(KIND, &value(KIND, &mut arguments)?)?)?,
      Some(DATA) => {
        let raw = text(DATA, &value(DATA, &mut arguments)?)?;
        let parsed =
          serde_json::from_str(&raw).map_err(|fault| UsageError::NotJson { raw, fault })?;
        once(&mut data, DATA, parsed)?;
      }
      Some(SOCKET) => {
        let path = path(SOCKET, &value(SOCKET, &mut arguments)?)?;
        once(&mut socket, SOCKET, path)?;
      }
      _not_a_flag_this_takes => return Err(stray(&argument)),
    }
  }

  Ok(Invocation::Send(Request {
    source: source.ok_or(UsageError::Missing(SOURCE))?,
    kind: kind.ok_or(UsageError::Missing(KIND))?,
    data: data.unwrap_or(serde_json::Value::Null),
    socket,
  }))
}

/// The argument after a flag, or [`UsageError::NoValue`].
fn value(
  flag: &'static str,
  arguments: &mut impl Iterator<Item = OsString>,
) -> Result<OsString, UsageError> {
  arguments.next().ok_or(UsageError::NoValue(flag))
}

/// A flag's value as a non-empty `String`.
fn text(flag: &'static str, raw: &OsString) -> Result<String, UsageError> {
  let text = raw.to_str().ok_or(UsageError::NotUtf8(flag))?;
  if text.is_empty() {
    return Err(UsageError::Empty(flag));
  }
  Ok(text.to_owned())
}

/// A flag's value as a non-empty path. **No UTF-8 requirement**: a path that
/// is not UTF-8 is a path a host can still be listening on, and narrowing
/// what emit accepts would be the renderer mistake pointed at a filesystem.
fn path(flag: &'static str, raw: &OsString) -> Result<PathBuf, UsageError> {
  if raw.is_empty() {
    return Err(UsageError::Empty(flag));
  }
  Ok(PathBuf::from(raw))
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
/// `-` so the message can say *unknown flag* rather than *unexpected
/// argument* for a typo.
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

  use super::{Invocation, Request, UsageError, parse};

  /// `parse` takes `args_os()` whole, program name included, so every case
  /// here says so rather than a call site no test covers.
  fn parsed(arguments: &[&str]) -> Result<Invocation, UsageError> {
    let argv =
      std::iter::once(OsString::from("goad-emit")).chain(arguments.iter().map(OsString::from));
    parse(argv)
  }

  fn sent(arguments: &[&str]) -> Request {
    match parsed(arguments).expect("these arguments are well-formed") {
      Invocation::Send(request) => request,
      _other => panic!("expected a send"),
    }
  }

  fn refused(arguments: &[&str]) -> UsageError {
    parsed(arguments).expect_err("these arguments are not well-formed")
  }

  #[test]
  fn source_and_kind_alone_send_with_null_data_and_no_socket() {
    assert_eq!(
      sent(&["--source", "w", "--kind", "k"]),
      Request {
        source: "w".to_owned(),
        kind: "k".to_owned(),
        data: serde_json::Value::Null,
        socket: None,
      }
    );
  }

  #[test]
  fn data_is_parsed_locally() {
    assert_eq!(
      sent(&["--source", "w", "--kind", "k", "--data", r#"{"n":4}"#]).data,
      serde_json::json!({"n": 4})
    );
  }

  #[test]
  fn socket_is_taken_as_a_path() {
    assert_eq!(
      sent(&["--source", "w", "--kind", "k", "--socket", "./goad.sock"]).socket,
      Some(PathBuf::from("./goad.sock"))
    );
  }

  #[test]
  fn help_is_help_wherever_it_appears() {
    for arguments in [
      &["-h"][..],
      &["--help"][..],
      &["--source", "w", "--help"][..],
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
  fn a_missing_source_names_the_flag() {
    assert!(matches!(
      refused(&["--kind", "k"]),
      UsageError::Missing("--source")
    ));
  }

  #[test]
  fn a_missing_kind_names_the_flag() {
    assert!(matches!(
      refused(&["--source", "w"]),
      UsageError::Missing("--kind")
    ));
  }

  #[test]
  fn an_empty_source_is_refused() {
    assert!(matches!(
      refused(&["--source", "", "--kind", "k"]),
      UsageError::Empty("--source")
    ));
  }

  #[test]
  fn an_empty_kind_is_refused() {
    assert!(matches!(
      refused(&["--source", "w", "--kind", ""]),
      UsageError::Empty("--kind")
    ));
  }

  #[test]
  fn an_empty_socket_is_refused() {
    assert!(matches!(
      refused(&["--source", "w", "--kind", "k", "--socket", ""]),
      UsageError::Empty("--socket")
    ));
  }

  #[test]
  fn unparseable_data_is_refused_carrying_the_text_that_failed() {
    match refused(&["--source", "w", "--kind", "k", "--data", "{oops"]) {
      UsageError::NotJson { raw, .. } => assert_eq!(raw, "{oops"),
      other => panic!("expected NotJson, found {other:?}"),
    }
  }

  #[test]
  fn a_repeated_flag_is_refused() {
    assert!(matches!(
      refused(&["--source", "w", "--source", "x", "--kind", "k"]),
      UsageError::Repeated("--source")
    ));
  }

  #[test]
  fn an_unknown_flag_is_refused_naming_it() {
    match refused(&["--source", "w", "--kind", "k", "--socket=x"]) {
      UsageError::Unknown(flag) => assert_eq!(flag, "--socket=x"),
      other => panic!("expected Unknown, found {other:?}"),
    }
  }

  #[test]
  fn a_bare_argument_is_refused() {
    match refused(&["--source", "w", "--kind", "k", "stray"]) {
      UsageError::Positional(argument) => assert_eq!(argument, "stray"),
      other => panic!("expected Positional, found {other:?}"),
    }
  }

  #[test]
  fn a_flag_with_no_value_is_refused() {
    assert!(matches!(
      refused(&["--source", "w", "--kind"]),
      UsageError::NoValue("--kind")
    ));
  }

  #[test]
  fn a_value_that_is_not_utf8_is_refused_where_a_string_is_wanted() {
    use std::os::unix::ffi::OsStringExt as _;
    let argv = [
      OsString::from("goad-emit"),
      OsString::from("--source"),
      OsString::from_vec(vec![0xff, 0xfe]),
      OsString::from("--kind"),
      OsString::from("k"),
    ];
    assert!(matches!(
      parse(argv.into_iter()).expect_err("invalid UTF-8 is refused"),
      UsageError::NotUtf8("--source")
    ));
  }
}

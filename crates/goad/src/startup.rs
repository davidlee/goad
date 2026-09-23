// crates/goad/src/startup.rs — stratum 3. `Launch`, `StartupError` and
// `arguments` are one module: they are the three things `run` needs and the
// three things item 17 drives, and none of them names a Slint type (F-17).

use std::ffi::OsString;
use std::path::PathBuf;

use goad_shell::config::IngressConfig;
use goad_shell::ingress::{self, Ingress, IngressError};

/// What the arguments asked for. `Help` and `Version` answer and stop, and
/// they are outcomes here rather than an early `exit` hidden inside argument
/// parsing — so the exit-code decision stays in one place, `exit::status`
/// over whatever `run` answers (§5.4's entry point). **Named, never
/// counted**, the rule `StartupError`'s doc below states: this enum's count
/// was hand-incremented from two to three when `Version` arrived, which is
/// the act PHASE-04/EX-3 forbids (`review-code.md` F-10).
#[derive(Debug, PartialEq, Eq)]
pub enum Launch {
  Help,
  Version,
  Config(PathBuf),
}

/// Every way `run` can fail to reach the event loop, and the exact text of
/// each. `NoConfigPath` and `Usage` come from argument and environment
/// handling, `AnswerUnwritten` from answering a question, and the rest from
/// `start`'s steps. The loop's own ending, once
/// the loop is reached, is not among these: it travels in `Ended`, built by
/// `exit::ended`, and no arm here speaks for it (010/PHASE-02). **Named,
/// never counted** — a count is stale at the next variant and nothing in the
/// gate reads it, which is how the number here said eight with nine in the
/// enum (PHASE-04/EX-3, `review-code.md` F-4). `Debug`, `Display`,
/// `std::error::Error` with the **default** `source()`, and no `PartialEq` —
/// a `slint::PlatformError` inside it has none (F-17).
///
/// The number every variant of this type answers is `exit::status`'s single
/// `Err` arm, which reads no variant here — a variant cannot be filed under a
/// different number without someone editing that arm and writing a per-cause
/// judgement down where a reviewer sees it. What the number means is the
/// spec's. `nix/module.nix` depends on it by value, and
/// `tests/binary/exit_codes.rs` is what holds it.
#[derive(Debug)]
pub enum StartupError {
  /// Neither `XDG_CONFIG_HOME` nor `HOME` names a directory, and no argument
  /// was given.
  NoConfigPath,
  /// Two or more positional arguments.
  Usage,
  /// The configuration file is missing or could not be read. The path is
  /// carried because the remedy is a thing done *to that file*, and
  /// `ConfigError`'s own text names no file (`goad-emit`'s `StartupFault`
  /// makes the same cut, against the same `ConfigError::Read`).
  ConfigUnreadable {
    path: PathBuf,
    fault: std::io::Error,
  },
  /// The file was read and is not a configuration this host could start on.
  /// Stratum 2's own error, unprefixed — it already says what was wrong with
  /// the contents, so the path is all this arm adds.
  ConfigUnparseable {
    path: PathBuf,
    fault: goad_shell::error::ConfigError,
  },
  /// The wall clock could not be read.
  Clock(goad_shell::clock::ClockError),
  /// The async runtime could not be built.
  Runtime(std::io::Error),
  /// A Slint platform call failed — `set_xdg_app_id`, `PromptWindow::new`,
  /// or `Tray::new`.
  Platform(slint::PlatformError),
  /// The host task could not be scheduled onto the event loop.
  EventLoop(slint::EventLoopError),
  /// The first evaluation could not be enqueued into a fresh, empty,
  /// capacity-1 channel.
  Enqueue,
  /// The configured ingress socket could not be bound.
  Ingress(IngressError),
  /// `--help` or `--version` was asked, and its answer could not be written
  /// to standard output. A question whose answer reached nobody was not
  /// answered, so it is not an end *as asked* (010 `review-code.md` F-3).
  AnswerUnwritten(std::io::Error),
}

impl std::fmt::Display for StartupError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::NoConfigPath => write!(
        f,
        "neither XDG_CONFIG_HOME nor HOME names a directory, so there is no configuration path; pass one as the single argument"
      ),
      Self::Usage => write!(
        f,
        "too many arguments: goad takes at most one, the path of the configuration file; run `goad --help` for usage"
      ),
      Self::ConfigUnreadable { path, fault } => {
        write!(f, "{} could not be read: {fault}", path.display())
      }
      Self::ConfigUnparseable { path, fault } => write!(f, "{}: {fault}", path.display()),
      Self::Clock(error) => write!(f, "{error}"),
      Self::Runtime(error) => write!(f, "the async runtime could not be started: {error}"),
      Self::Platform(error) => write!(f, "the display could not be opened: {error}"),
      Self::EventLoop(error) => write!(f, "the event loop would not accept the host task: {error}"),
      Self::Enqueue => write!(f, "the first request could not be enqueued"),
      Self::Ingress(error) => write!(f, "{error}"),
      Self::AnswerUnwritten(error) => write!(
        f,
        "the answer could not be written to standard output: {error}"
      ),
    }
  }
}

impl std::error::Error for StartupError {}

/// The listener a configuration names, or the handle a host with none holds.
/// `None` touches nothing — no bind, no filesystem entry (AC-7's second half).
/// `Some` binds and wraps `bind`'s error; `main::start` calls this and nothing
/// else decides it.
///
/// # Errors
/// [`StartupError::Ingress`] when a configured path cannot be bound.
pub fn listener(configured: Option<&IngressConfig>) -> Result<Ingress, StartupError> {
  match configured {
    None => Ok(Ingress::none()),
    Some(config) => ingress::bind(&config.path).map_err(StartupError::Ingress),
  }
}

/// Pure over the arguments and the environment it is handed, so the table below
/// is a test rather than a claim (§9 item 17).
///
/// `argv` is `std::env::args_os()` **whole**, program name included: the skip
/// lives here, inside the function the table tests, rather than at a call site
/// no test covers. The rows below count what is left after it.
///
/// | arguments | behaviour |
/// |---|---|
/// | none | whatever [`goad_shell::config::default_path`] answers, which is where its own table states the XDG rule; `None` ⇒ [`StartupError::NoConfigPath`], whose text names both variables. |
/// | `-h` or `--help` | the usage block on stdout, exit 0 — its text is §5.4's, and `--help` is its only destination |
/// | `--version` | [`crate::diagnostics::version_line`] on stdout, exit 0 |
/// | exactly one, anything else | that path, verbatim; `$XDG_CONFIG_HOME` is not consulted |
/// | two or more | [`StartupError::Usage`] on stderr, exit 2 — the host does not guess which was meant |
///
/// # Errors
/// [`StartupError::Usage`] for two or more positional arguments;
/// [`StartupError::NoConfigPath`] when none is given and neither variable
/// names a directory.
pub fn arguments(
  argv: impl Iterator<Item = OsString>,
  env: &dyn Fn(&str) -> Option<OsString>,
) -> Result<Launch, StartupError> {
  let rest: Vec<OsString> = argv.skip(1).collect();

  match rest.as_slice() {
    [] => goad_shell::config::default_path(env)
      .map(Launch::Config)
      .ok_or(StartupError::NoConfigPath),
    [only] if only == "-h" || only == "--help" => Ok(Launch::Help),
    // Before the catch-all, or `--version` is a path and the host opens a
    // file by that name.
    [only] if only == "--version" => Ok(Launch::Version),
    [only] => Ok(Launch::Config(PathBuf::from(only))),
    _ => Err(StartupError::Usage),
  }
}

// crates/goad/src/startup.rs — stratum 3. `Launch`, `StartupError` and
// `arguments` are one module: they are the three things `run` needs and the
// three things item 17 drives, and none of them names a Slint type (F-17).

use std::ffi::OsString;
use std::path::PathBuf;

/// What the arguments asked for. Two outcomes, and `--help` is one of them
/// rather than an early `exit` hidden inside argument parsing — so `main` keeps
/// its single exit-code decision (§5.4's entry point).
#[derive(Debug, PartialEq, Eq)]
pub enum Launch {
  Help,
  Config(PathBuf),
}

/// The eight variants, and their exact text. Two come from argument and
/// environment handling, six from the steps after it. `Debug`, `Display`,
/// `std::error::Error` with the **default** `source()`, and no `PartialEq` —
/// a `slint::PlatformError` inside it has none (F-17).
#[derive(Debug)]
pub enum StartupError {
  /// Neither `XDG_CONFIG_HOME` nor `HOME` names a directory, and no argument
  /// was given.
  NoConfigPath,
  /// Two or more positional arguments.
  Usage,
  /// Stratum 2's own configuration error, unwrapped and unprefixed.
  Config(goad_shell::error::ConfigError),
  /// The wall clock could not be read.
  Clock(crate::clock::ClockError),
  /// The async runtime could not be built.
  Runtime(std::io::Error),
  /// A Slint platform call failed — `set_xdg_app_id`, `PromptWindow::new`,
  /// `Tray::new`, or `run_event_loop_until_quit`.
  Platform(slint::PlatformError),
  /// The host task could not be scheduled onto the event loop.
  EventLoop(slint::EventLoopError),
  /// The first evaluation could not be enqueued into a fresh, empty,
  /// capacity-1 channel.
  Enqueue,
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
      Self::Config(error) => write!(f, "{error}"),
      Self::Clock(error) => write!(f, "{error}"),
      Self::Runtime(error) => write!(f, "the async runtime could not be started: {error}"),
      Self::Platform(error) => write!(f, "the display could not be opened: {error}"),
      Self::EventLoop(error) => write!(f, "the event loop would not accept the host task: {error}"),
      Self::Enqueue => write!(f, "the first request could not be enqueued"),
    }
  }
}

impl std::error::Error for StartupError {}

/// Pure over the arguments and the environment it is handed, so the table below
/// is a test rather than a claim (§9 item 17).
///
/// `argv` is `std::env::args_os()` **whole**, program name included: the skip
/// lives here, inside the function the table tests, rather than at a call site
/// no test covers. The rows below count what is left after it.
///
/// | arguments | behaviour |
/// |---|---|
/// | none | `$XDG_CONFIG_HOME/goad/config.toml` when that variable is set and **absolute**; otherwise `$HOME/.config/goad/config.toml`. `HOME` unset or empty ⇒ [`StartupError::NoConfigPath`], whose text names both variables. |
/// | `-h` or `--help` | the usage block on stdout, exit 0 — its text is §5.4's, and `--help` is its only destination |
/// | exactly one, anything else | that path, verbatim; `$XDG_CONFIG_HOME` is not consulted |
/// | two or more | [`StartupError::Usage`] on stderr, exit 2 — the host does not guess which was meant |
///
/// `HOME` is used **as given** and is not required to be absolute: the XDG
/// basedir spec states the absoluteness rule for `XDG_CONFIG_HOME` and states
/// nothing of the kind for `HOME`, and a relative `HOME` is a broken
/// environment the host cannot repair and should not silently reinterpret.
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
    [] => match env("XDG_CONFIG_HOME").filter(|value| PathBuf::from(value).is_absolute()) {
      Some(xdg) => Ok(Launch::Config(PathBuf::from(xdg).join("goad/config.toml"))),
      None => match env("HOME") {
        Some(home) if !home.is_empty() => Ok(Launch::Config(
          PathBuf::from(home).join(".config/goad/config.toml"),
        )),
        _ => Err(StartupError::NoConfigPath),
      },
    },
    [only] if only == "-h" || only == "--help" => Ok(Launch::Help),
    [only] => Ok(Launch::Config(PathBuf::from(only))),
    _ => Err(StartupError::Usage),
  }
}

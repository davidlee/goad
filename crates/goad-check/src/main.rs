//! `goad-check`: a backend author's conformance checker, stratum 3.
//!
//! Headless. It drives a `goad_shell::host::Host` against the author's
//! backend and reports what the host saw (`design.md` §5.2.1).

mod args;
mod render;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use goad_semantics::protocol::canonical::Event;
use goad_shell::backend::process::ProcessBackend;
use goad_shell::clock::{self, ClockError};
use goad_shell::config::{self, BackendConfig, Config, ScheduleConfig};
use goad_shell::error::ConfigError;
use goad_shell::host::Host;
use goad_shell::ingress::envelope::{self, EnvelopeFault};
use goad_shell::report::{line_to, try_line_to};
use goad_shell::version::version_line;

use crate::args::{Invocation, Request, Source};

/// The argv form's `schedule.default_poll`, which no file states. It only
/// affects how the report shows a resolved next check when none was sent
/// (`design.md` §5.2.1).
const DEFAULT_POLL: jiff::SignedDuration = jiff::SignedDuration::from_mins(30);

/// Why the run never reached its first exchange. Every one is status 2: no
/// verdict was delivered (`design.md` §5.2.5).
#[derive(Debug)]
pub(crate) enum StartupFault {
  /// The config form with no `--config`, and neither `XDG_CONFIG_HOME` nor
  /// `HOME` names a directory, so there is no default path to try.
  NoPath,
  /// The configuration file is absent or cannot be read. Split from
  /// `ConfigUnparseable` where the path is in hand, as `goad`'s `start` does:
  /// `ConfigError` names no file.
  ConfigUnreadable {
    path: PathBuf,
    fault: std::io::Error,
  },
  /// The file was read and is not a configuration the host could start on.
  ConfigUnparseable { path: PathBuf, fault: ConfigError },
  /// An `--event` file is absent or cannot be read.
  EventUnreadable {
    path: PathBuf,
    fault: std::io::Error,
  },
  /// An `--event` file was read and is not an envelope the host would
  /// accept on its socket: `envelope::normalize` refused it.
  EventRefused { path: PathBuf, fault: EnvelopeFault },
  /// The wall clock could not be read, so no request has a `now`.
  Clock(ClockError),
  /// The current-thread runtime the exchanges run on could not be built.
  Runtime(std::io::Error),
}

/// The one impure file: the only place the checker reads the environment, a
/// file or the clock. What it decides with what it reads lives in [`args`] and
/// [`render`].
///
/// Each status reachable until PHASE-12 is a literal: 0, a question answered,
/// in [`answer`], and 2, not judged, in [`not_judged`] alone. 1 and the
/// verdict are PHASE-12's (`design.md` §5.2.5).
fn main() -> ExitCode {
  match args::parse(std::env::args_os()) {
    Err(error) => not_judged(&render::usage_error_line(&error)),
    Ok(Invocation::Help) => answer(render::USAGE),
    // The version is this binary's own, and the revision is read at compile
    // time, as `goad-emit`'s `main` does and for its reasons.
    Ok(Invocation::Version) => answer(&version_line(
      env!("CARGO_PKG_VERSION"),
      option_env!("GOAD_REVISION"),
    )),
    Ok(Invocation::Check(request)) => match prepare(request) {
      Err(fault) => not_judged(&render::startup_error_line(&fault)),
      Ok(()) => unjudged_end(),
    },
  }
}

/// The steps before the first exchange, in `design.md` §5.2.2's order, as
/// `goad`'s `start` builds the host: the configuration, each event file, the
/// clock, a current-thread runtime (§5.4 *Concurrency*), the backend and the
/// `Host`.
///
/// The runtime, the events and the host are built and not yet used: the
/// exchanges that use them are PHASE-12's.
///
/// # Errors
///
/// The first step that fails, which ends the run with no verdict.
fn prepare(request: Request) -> Result<(), StartupFault> {
  // A closure, not `&std::env::var_os`: that is generic over its key, and a
  // generic fn item does not coerce to `&dyn Fn` (`goad-emit`'s `exchange`).
  let config = configuration(request.source, &|name| std::env::var_os(name))?;
  let _events = normalized(&request.events)?;
  let now = clock::wall_clock().map_err(StartupFault::Clock)?;
  let _runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .map_err(StartupFault::Runtime)?;
  let backend = ProcessBackend::new(config.backend.command.clone(), config.backend.timeout);
  let _host = Host::new(config, backend, now);
  Ok(())
}

/// The configuration either form names. The config form loads a file by the
/// host's rules, at `--config` or the host's default path; the argv form
/// states one, with no `[ingress]`, because the checker opens no socket.
///
/// # Errors
///
/// No default path, or a file that cannot be read or is not a configuration.
fn configuration(
  source: Source,
  env: &dyn Fn(&str) -> Option<OsString>,
) -> Result<Config, StartupFault> {
  match source {
    Source::Argv { command, timeout } => Ok(Config {
      backend: BackendConfig { command, timeout },
      schedule: ScheduleConfig {
        default_poll: DEFAULT_POLL,
      },
      ingress: None,
    }),
    Source::File(given) => {
      let path = match given {
        Some(path) => path,
        None => config::default_path(env).ok_or(StartupFault::NoPath)?,
      };
      // Split where the path is in hand, as `goad`'s `start` does:
      // `ConfigError` names no file.
      match Config::load(&path) {
        Err(ConfigError::Read(fault)) => Err(StartupFault::ConfigUnreadable { path, fault }),
        Err(fault) => Err(StartupFault::ConfigUnparseable { path, fault }),
        Ok(config) => Ok(config),
      }
    }
  }
}

/// Each `--event` file, read and normalized as the host's socket would, in
/// the order given.
///
/// # Errors
///
/// The first file that cannot be read, or that `envelope::normalize` refuses.
fn normalized(paths: &[PathBuf]) -> Result<Vec<Event>, StartupFault> {
  paths.iter().map(|path| event(path)).collect()
}

fn event(path: &Path) -> Result<Event, StartupFault> {
  let bytes = std::fs::read(path).map_err(|fault| StartupFault::EventUnreadable {
    path: path.to_path_buf(),
    fault,
  })?;
  envelope::normalize(&bytes).map_err(|fault| StartupFault::EventRefused {
    path: path.to_path_buf(),
    fault,
  })
}

/// The interim end (PHASE-04/EX-3). No exchange is made, so the report is the
/// no-view line alone and nothing is judged. PHASE-12/EX-6 replaces it with
/// the exchanges and the verdict.
fn unjudged_end() -> ExitCode {
  match try_line_to(std::io::stdout().lock(), render::NO_VIEW) {
    Err(fault) => not_judged(&render::report_unwritten_line(&fault)),
    Ok(()) => not_judged(render::NOT_YET_IMPLEMENTED),
  }
}

/// A question's answer, on stdout: status 0 only if it arrived (SPEC-004/R-8's
/// *only if*).
fn answer(line: &str) -> ExitCode {
  match try_line_to(std::io::stdout().lock(), line) {
    Ok(()) => ExitCode::SUCCESS,
    Err(fault) => not_judged(&render::answer_unwritten_line(&fault)),
  }
}

/// **The one status 2.** Every cause of *not judged* arrives here as a line
/// already rendered, so the status reads no cause (`design.md` §5.2.5). The
/// line goes to stderr, best effort, and is the last line written there.
fn not_judged(line: &str) -> ExitCode {
  line_to(std::io::stderr().lock(), line);
  ExitCode::from(2)
}

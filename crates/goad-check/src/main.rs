//! `goad-check`: a backend author's conformance checker, stratum 3.
//!
//! Headless. It drives a `goad_shell::host::Host` against the author's
//! backend and reports what the host saw (`design.md` §5.2.1).

mod args;
mod render;
mod run;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use goad_semantics::protocol::canonical::{Event, Timestamp};
use goad_shell::backend::process::ProcessBackend;
use goad_shell::clock::{self, ClockError};
use goad_shell::config::{self, BackendConfig, Config, ScheduleConfig};
use goad_shell::error::{ConfigError, StateError};
use goad_shell::host::{Host, Presented};
use goad_shell::ingress::envelope::{self, EnvelopeFault};
use goad_shell::report::{line_to, try_line_to};
use goad_shell::version::version_line;

use crate::args::{Invocation, Request, Source};
use crate::run::{Judged, Planned, Tally};

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

/// Why a run that had begun its exchanges delivered no verdict. Every one is
/// status 2 (`design.md` §5.2.5).
#[derive(Debug)]
pub(crate) enum RunFault {
  /// The wall clock could not be read for a request's `now`.
  Clock(ClockError),
  /// A report line standard output refused.
  ReportUnwritten(std::io::Error),
  /// The host refused an answer before consulting the backend: the checker
  /// named a `view_id` wrongly, which is its own defect.
  State(StateError),
}

/// The one impure file: the only place the checker reads the environment, a
/// file or the clock, or writes a stream. What it decides with what it reads
/// lives in [`args`] and [`run`], and every line's words in [`render`].
///
/// Each way to a status is a literal: 0, a question answered, in [`answer`];
/// 0 and 1, a verdict delivered, in [`verdict`], the one cut; and 2, not
/// judged, in [`not_judged`] alone (`design.md` §5.2.5).
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
      Ok(prepared) => run(prepared),
    },
  }
}

/// The steps before the first exchange, in `design.md` §5.2.2's order, as
/// `goad`'s `start` builds the host: the configuration, each event file, the
/// clock, a current-thread runtime (§5.4 *Concurrency*), the backend and the
/// `Host`.
///
/// # Errors
///
/// The first step that fails, which ends the run with no verdict.
fn prepare(request: Request) -> Result<Prepared, StartupFault> {
  // A closure, not `&std::env::var_os`: that is generic over its key, and a
  // generic fn item does not coerce to `&dyn Fn` (`goad-emit`'s `exchange`).
  let config = configuration(request.source, &|name| std::env::var_os(name))?;
  let events = normalized(&request.events)?;
  let now = clock::wall_clock().map_err(StartupFault::Clock)?;
  let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .map_err(StartupFault::Runtime)?;
  let backend = ProcessBackend::new(config.backend.command.clone(), config.backend.timeout);
  let host = Host::new(config, backend, now);
  Ok(Prepared {
    events,
    runtime,
    host,
  })
}

/// What [`prepare`] built, which the run consumes.
struct Prepared {
  events: Vec<Event>,
  runtime: tokio::runtime::Runtime,
  host: Host<ProcessBackend>,
}

/// Every planned exchange, then the verdict. Only a [`RunFault`] stops the
/// run short (`design.md` §5.4).
fn run(prepared: Prepared) -> ExitCode {
  let Prepared {
    events,
    runtime,
    mut host,
  } = prepared;
  match runtime.block_on(exchanges(&mut host, events)) {
    Err(fault) => not_judged(&render::run_fault_line(&fault)),
    Ok(tally) => match deliver(&tally) {
      Err(fault) => not_judged(&render::run_fault_line(&fault)),
      Ok(()) => verdict(tally.refused),
    },
  }
}

/// The plan, in order, each request's chain followed before the next. What
/// an exchange reports never stops the run: a backend that fails at startup
/// is still asked the rest.
///
/// # Errors
///
/// A [`RunFault`], which ends the run where it arose.
async fn exchanges(host: &mut Host<ProcessBackend>, events: Vec<Event>) -> Result<Tally, RunFault> {
  let mut tally = Tally::default();
  let mut a_host_kind_made_no_failure = false;
  for planned in run::plan(events) {
    let now = now()?;
    let event = planned.event(now);
    emit(&render::evaluate_line(&event.kind))?;
    let judged = judged(host.evaluate(now, event).await)?;
    let charged = match planned {
      Planned::Host(_) => {
        a_host_kind_made_no_failure |= judged.failure.is_none();
        false
      }
      Planned::Probe => run::charges_the_probe(&judged, a_host_kind_made_no_failure),
      Planned::Given(_) => false,
    };
    let view = report(&mut tally, judged)?;
    if charged {
      emit(&render::probe_claim_line())?;
    }
    follow(host, &mut tally, view).await?;
  }
  Ok(tally)
}

/// Answer each view a chain returns, up to the chain bound, with the
/// `view_id` the host minted for it.
///
/// # Errors
///
/// A [`RunFault`], which ends the run where it arose.
async fn follow(
  host: &mut Host<ProcessBackend>,
  tally: &mut Tally,
  mut view: Option<Presented>,
) -> Result<(), RunFault> {
  for _ in 0..run::CHAIN_BOUND {
    let Some(Presented {
      view_id,
      view: shown,
    }) = view
    else {
      return Ok(());
    };
    let answer = run::answer(&shown);
    let now = now()?;
    emit(&render::respond_line(&answer))?;
    view = report(tally, judged(host.respond(now, view_id, answer).await)?)?;
  }
  if view.is_some() {
    emit(&render::chain_bound_line())?;
  }
  Ok(())
}

/// One exchange's lines written and counted; its view, for the chain.
///
/// # Errors
///
/// A report line standard output refused.
fn report(tally: &mut Tally, judged: Judged) -> Result<Option<Presented>, RunFault> {
  for line in render::exchange_lines(&judged) {
    emit(&line)?;
  }
  tally.record(&judged);
  Ok(judged.view)
}

/// The report's closing lines: the no-view line, if no exchange returned a
/// view, then the verdict.
///
/// # Errors
///
/// A report line standard output refused.
fn deliver(tally: &Tally) -> Result<(), RunFault> {
  if !tally.viewed {
    emit(render::NO_VIEW)?;
  }
  emit(&render::verdict_line(tally.refused))
}

/// **The one cut** of a delivered verdict: 0 with no refusal, 1 with at
/// least one, whose last stderr line is the checker's (`design.md` §5.2.5).
fn verdict(refused: usize) -> ExitCode {
  if refused > 0 {
    line_to(std::io::stderr().lock(), &render::refused_line(refused));
    ExitCode::from(1)
  } else {
    ExitCode::SUCCESS
  }
}

/// # Errors
///
/// `Failure::State`, as a [`RunFault`].
fn judged(outcome: goad_shell::host::Outcome) -> Result<Judged, RunFault> {
  run::judged(outcome).map_err(RunFault::State)
}

/// The wall clock, for one request's `now`.
///
/// # Errors
///
/// The clock could not be read.
fn now() -> Result<Timestamp, RunFault> {
  clock::wall_clock().map_err(RunFault::Clock)
}

/// One report line, on stdout, through `try_line_to`: the report is the
/// answer, so a line not written is a run not judged.
///
/// # Errors
///
/// The line standard output refused.
fn emit(line: &str) -> Result<(), RunFault> {
  try_line_to(std::io::stdout().lock(), line).map_err(RunFault::ReportUnwritten)
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

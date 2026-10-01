//! `goad-check`: a backend author's conformance checker, stratum 3.
//!
//! Headless. It drives a `goad_shell::host::Host` against the author's
//! backend and reports what the host saw (`design.md` §5.2.1).

mod args;
mod render;

use std::path::PathBuf;
use std::process::ExitCode;

use goad_shell::clock::ClockError;
use goad_shell::error::ConfigError;
use goad_shell::ingress::envelope::EnvelopeFault;

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

fn main() -> ExitCode {
  ExitCode::SUCCESS
}

// crates/goad-emit/src/main.rs — stratum 3.
mod args;
mod render;

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use goad_semantics::protocol::canonical::{Event, Timestamp};
use goad_shell::clock::{self, ClockError};
use goad_shell::config::Config;
use goad_shell::error::ConfigError;
use goad_shell::ingress::client::{self, Answered};
use goad_shell::report::line_to;

use crate::args::{Invocation, Request};

/// Why the envelope never left. Every one of these is exit 2: emit got no
/// usable answer, and the caller's envelope was never judged (005/D-4).
///
/// **Four of the five are `AC-4`'s configuration road**, split the way AC-4
/// splits it because the remedies differ — write a configuration, fix its
/// path, fix its contents, add an `[ingress]` section. The fifth is the
/// clock: `design.md` §5.4 reads one between the arguments and the socket,
/// and a clock that cannot be read stops the envelope in the same way and at
/// the same cost, so it is reported through the same door rather than through
/// a fifth `render` function.
#[derive(Debug)]
pub(crate) enum StartupFault {
  /// Neither `XDG_CONFIG_HOME` nor `HOME` names a directory, so there is no
  /// default configuration path to try. `--socket` does not reach here.
  NoPath,
  /// The configuration path was discovered and the file is absent or cannot
  /// be read.
  Unreadable {
    path: PathBuf,
    fault: std::io::Error,
  },
  /// The file was read and is not a configuration this host could start on.
  /// Emit reads the **host's** file, so every fault in it is emit's too, not
  /// only the `[ingress]` section's.
  Unparseable { path: PathBuf, fault: ConfigError },
  /// A valid configuration with no `[ingress]` section: the host it describes
  /// is not configured to listen, so there is no socket to write to.
  NoIngress { path: PathBuf },
  /// The wall clock could not be read, so the envelope has no `timestamp`
  /// (`SPEC-003/R-10` admits no envelope without one).
  ClockUnreadable(ClockError),
}

/// The one impure file (`design.md` §4 principle 1): the only place emit
/// reads an environment, a clock, a file or a socket. Everything it decides
/// with what it reads lives in [`args`] and [`render`], where a test needs no
/// fixture.
///
/// Three exit codes, and they are about **who was wrong** (005/D-4): 0 the
/// host took it, 1 the host refused it and said why, 2 emit got no usable
/// answer — a usage error, a configuration that names no socket, a path
/// nothing is listening at, or a reply `SPEC-003` §6.3 does not admit.
/// `std::process::exit` is a `disallowed-method`, so `main` returns an
/// [`ExitCode`] and every path returns through it.
fn main() -> ExitCode {
  match args::parse(std::env::args_os()) {
    Err(error) => {
      to_stderr(&render::usage_error_line(&error));
      ExitCode::from(2)
    }
    Ok(Invocation::Help) => {
      to_stdout(render::USAGE);
      ExitCode::SUCCESS
    }
    Ok(Invocation::Version) => {
      // The revision is read at compile time and never at run time —
      // `crates/goad`'s `run` carries the reasoning, and this is the same
      // rule for the same variable. Handed on unjudged: set-but-empty is
      // unset, and `render::version_line` is where that is decided and
      // tested (`review-code.md` F-2).
      to_stdout(&render::version_line(option_env!("GOAD_REVISION")));
      ExitCode::SUCCESS
    }
    Ok(Invocation::Send(request)) => exchange(request),
  }
}

/// One envelope, one reply, one exit code — `design.md` §5.4's sequence, in
/// the order it draws it: the socket path before the clock, because a
/// configuration fault should not first spend a syscall, and both before any
/// connection.
fn exchange(request: Request) -> ExitCode {
  // Passed in, not reached for, so `socket_path` is testable without touching
  // the process environment. The closure cannot be `&std::env::var_os`:
  // that is generic over `K: AsRef<OsStr>`, and a generic fn item does not
  // coerce to `&dyn Fn(&str) -> Option<OsString>` (`crates/goad`'s `run`).
  let socket = match socket_path(&request, &|name| std::env::var_os(name)) {
    Ok(socket) => socket,
    Err(fault) => return startup_failed(&fault),
  };
  let now = match clock::wall_clock() {
    Ok(now) => now,
    Err(fault) => return startup_failed(&StartupFault::ClockUnreadable(fault)),
  };

  match client::send(&socket, &envelope(request, now)) {
    Ok(Answered::Accepted) => ExitCode::SUCCESS,
    Ok(refused) => {
      to_stderr(&render::refused_line(&refused));
      ExitCode::from(1)
    }
    Err(fault) => {
      to_stderr(&render::fault_line(&fault, &socket));
      ExitCode::from(2)
    }
  }
}

fn startup_failed(fault: &StartupFault) -> ExitCode {
  to_stderr(&render::startup_error_line(fault));
  ExitCode::from(2)
}

/// Where to write, and the whole of what reads a configuration.
///
/// **`--socket` short-circuits everything** (AC-4): no environment is read,
/// no file is opened, and a configuration that does not exist is not a fault.
/// Without it the host's *default* path is discovered by the rule the host
/// itself uses — one rule, one definition (`goad_shell::config::default_path`)
/// — and the file there must be a configuration the host could start on,
/// because it is the host's own file.
///
/// # Errors
///
/// The four ways the configuration road ends at exit 2, each naming the path.
fn socket_path(
  request: &Request,
  env: &dyn Fn(&str) -> Option<OsString>,
) -> Result<PathBuf, StartupFault> {
  if let Some(explicit) = &request.socket {
    return Ok(explicit.clone());
  }
  let path = goad_shell::config::default_path(env).ok_or(StartupFault::NoPath)?;
  match Config::load(&path) {
    Err(ConfigError::Read(fault)) => Err(StartupFault::Unreadable { path, fault }),
    Err(fault) => Err(StartupFault::Unparseable { path, fault }),
    Ok(configured) => configured
      .ingress
      .map(|ingress| ingress.path)
      .ok_or(StartupFault::NoIngress { path }),
  }
}

/// The envelope — `SPEC-003` §6.2's four fields, which [`Event`] already is
/// (005/D-3). Pure over the clock reading, so the key set is a test.
fn envelope(request: Request, now: Timestamp) -> Event {
  Event {
    source: request.source,
    kind: request.kind,
    timestamp: now,
    data: request.data,
  }
}

/// `print_stdout` and `print_stderr` are denied workspace-wide, and
/// `report::line_to` is what 005 lifted to stratum 2 so that a second binary
/// could write a line without a second answer to *what happens when the sink
/// fails*. Both outlets are one line; nothing here formats.
fn to_stdout(line: &str) {
  line_to(std::io::stdout().lock(), line);
}

fn to_stderr(line: &str) {
  line_to(std::io::stderr().lock(), line);
}

#[cfg(test)]
mod tests {
  use std::ffi::OsString;
  use std::path::{Path, PathBuf};

  use super::{Request, StartupFault, envelope, socket_path};

  const GOOD: &str = "[backend]\ncommand = [\"deno\", \"run\", \"./backend.ts\"]\ntimeout = \"5s\"\n\n[schedule]\ndefault_poll = \"30m\"\n";

  /// A configuration directory of this case's own, under `temp_dir()` —
  /// `config.rs`'s own precedent, `tempfile` not being on the manifest
  /// allowlist and this crate carrying no dev-dependency at all (AC-7).
  fn config_home(case: &str, contents: Option<&str>) -> PathBuf {
    let home = std::env::temp_dir().join(format!("goad-emit-{case}-{}", std::process::id()));
    let directory = home.join("goad");
    std::fs::create_dir_all(&directory).expect("the fixture directory must be creatable");
    let file = directory.join("config.toml");
    match contents {
      Some(text) => std::fs::write(&file, text).expect("the fixture must be writable"),
      None => match std::fs::remove_file(&file) {
        Ok(()) | Err(_) => (),
      },
    }
    home
  }

  fn env_at(home: &Path) -> impl Fn(&str) -> Option<OsString> + use<> {
    let home = OsString::from(home);
    move |name| match name {
      "XDG_CONFIG_HOME" => Some(home.clone()),
      _nothing_else_is_consulted => None,
    }
  }

  fn request(socket: Option<&str>) -> Request {
    Request {
      source: "w".to_owned(),
      kind: "k".to_owned(),
      data: serde_json::Value::Null,
      socket: socket.map(PathBuf::from),
    }
  }

  /// PHASE-03/VT-3, AC-4.
  #[test]
  fn a_socket_flag_wins_over_a_configuration_naming_another_path() {
    let home = config_home(
      "socket-wins",
      Some(&format!(
        "{GOOD}\n[ingress]\npath = \"./from-config.sock\"\n"
      )),
    );
    let resolved = socket_path(&request(Some("./explicit.sock")), &env_at(&home))
      .expect("an explicit socket needs no configuration");
    assert_eq!(resolved, PathBuf::from("./explicit.sock"));
  }

  /// PHASE-03/VT-3. The second half, and the one that matters for `just demo`:
  /// `--socket` is honoured where there is no configuration to read at all
  /// (F-6).
  #[test]
  fn a_socket_flag_is_honoured_with_no_configuration_at_all() {
    let resolved = socket_path(&request(Some("./explicit.sock")), &|_never_consulted| None)
      .expect("an explicit socket consults nothing");
    assert_eq!(resolved, PathBuf::from("./explicit.sock"));
  }

  /// PHASE-03/VT-6, first of four.
  #[test]
  fn no_discoverable_path_is_a_named_fault() {
    let fault = socket_path(&request(None), &|_neither_variable_is_set| None)
      .expect_err("there is nowhere to look");
    assert!(matches!(fault, StartupFault::NoPath), "{fault:?}");
  }

  #[test]
  fn an_absent_configuration_file_is_a_named_fault() {
    let home = config_home("absent", None);
    let fault = socket_path(&request(None), &env_at(&home)).expect_err("the file is not there");
    assert!(
      matches!(fault, StartupFault::Unreadable { .. }),
      "{fault:?}"
    );
  }

  #[test]
  fn an_unparseable_configuration_is_a_named_fault() {
    let home = config_home("unparseable", Some("[backend]\ncommand = "));
    let fault = socket_path(&request(None), &env_at(&home)).expect_err("that is not TOML");
    assert!(
      matches!(fault, StartupFault::Unparseable { .. }),
      "{fault:?}"
    );
  }

  #[test]
  fn a_configuration_with_no_ingress_section_is_a_named_fault() {
    let home = config_home("no-ingress", Some(GOOD));
    let fault =
      socket_path(&request(None), &env_at(&home)).expect_err("that host is not listening");
    assert!(matches!(fault, StartupFault::NoIngress { .. }), "{fault:?}");
  }

  /// A configuration that *does* name a socket is where the path comes from
  /// with no `--socket` — the positive control for the four faults above.
  #[test]
  fn a_configured_ingress_path_is_what_is_written_to() {
    let home = config_home(
      "configured",
      Some(&format!(
        "{GOOD}\n[ingress]\npath = \"./from-config.sock\"\n"
      )),
    );
    let resolved =
      socket_path(&request(None), &env_at(&home)).expect("this configuration names a socket");
    assert_eq!(resolved, PathBuf::from("./from-config.sock"));
  }

  /// PHASE-03/VT-5. `design.md` §5.5 assumes `Event`'s own `Serialize` output
  /// is exactly `SPEC-003` §6.2's four keys. A field added to `Event` for
  /// `SPEC-001`'s benefit would silently widen this envelope, so the
  /// assumption is pinned rather than restated.
  ///
  /// The timestamp comes from `wall_clock` rather than a parsed literal for a
  /// reason worth keeping: **this crate cannot name `jiff` at all** (AC-7),
  /// so `Timestamp::new` is out of reach here and the only way to hold one is
  /// the way `main` holds one. That is 005/D-9 discharged — the clock arrives
  /// from stratum 2, and stratum 3 needs no date library.
  #[test]
  fn the_envelope_carries_exactly_the_four_keys() {
    let now = goad_shell::clock::wall_clock().expect("the wall clock must be readable");
    let event = envelope(request(None), now);
    let text = serde_json::to_string(&event).expect("an event of primitives serializes");
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("what we wrote is JSON");
    let object = parsed.as_object().expect("an envelope is one JSON object");
    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["data", "kind", "source", "timestamp"]);
  }
}

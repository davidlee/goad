//! Item 17 — the startup surface, mostly pure functions with no window;
//! `listener` is the exception, since binding a socket is what it does
//! (design.md §9 item 17, §5.4's exact strings). `StartupError`'s `Display`
//! for **every** variant it has and `ClockError`'s for every one of its,
//! asserted verbatim; the usage block produced by one `const` and
//! byte-identical wherever it appears; a usage error's text not containing
//! the usage block; both `source()`s `None`; the argument table's rows; and
//! the two stderr outlets' exact strings, via their pure `_line` half (F-7).
//!
//! No test here runs the binary or asserts an exit code (§9's own rule) —
//! `main`'s one `match` over `run()`'s `Result` is what chooses the code,
//! and every value that `match` sees is already covered here. That covers
//! the **arms**; it does not cover the **constant** either arm names, which
//! `nix/module.nix` depends on by value. `tests/binary/exit_codes.rs` holds
//! that, one tier up (`review-code.md` F-1).

use std::ffi::OsString;
use std::path::PathBuf;

use goad::diagnostics::{USAGE, print_usage, report_platform_line, report_startup_line};
use goad::startup::{Launch, StartupError, arguments, listener};
use goad_shell::clock::ClockError;
use goad_shell::config::IngressConfig;
use goad_shell::error::ConfigError;

fn argv(rest: &[&str]) -> impl Iterator<Item = OsString> {
  std::iter::once(OsString::from("goad")).chain(rest.iter().map(OsString::from))
}

fn no_env(_name: &str) -> Option<OsString> {
  None
}

mod display_text {
  use super::{ClockError, ConfigError, StartupError};

  #[test]
  fn no_config_path() {
    assert_eq!(
      StartupError::NoConfigPath.to_string(),
      "neither XDG_CONFIG_HOME nor HOME names a directory, so there is no configuration path; pass one as the single argument"
    );
  }

  #[test]
  fn usage() {
    assert_eq!(
      StartupError::Usage.to_string(),
      "too many arguments: goad takes at most one, the path of the configuration file; run `goad --help` for usage"
    );
  }

  /// 006/PHASE-04/VT-1, AC-6. The file the host tried is named, because the
  /// remedy is a thing done *to that file* — until 006 this line was
  /// `ConfigError`'s own, which says *configuration could not be read* and
  /// not which configuration (`research.md` S-4).
  #[test]
  fn config_unreadable_names_the_path_and_the_fault() {
    let path = std::path::PathBuf::from("/nonexistent/wat.toml");
    let rendered = StartupError::ConfigUnreadable {
      path: path.clone(),
      fault: std::io::Error::other("permission denied"),
    }
    .to_string();
    assert_eq!(
      rendered,
      "/nonexistent/wat.toml could not be read: permission denied"
    );
    assert!(rendered.contains(path.display().to_string().as_str()));
  }

  /// 006/PHASE-04/VT-1, AC-6. The second arm, and the unprefixed spelling:
  /// `ConfigError`'s own text already says what was wrong with the file, so
  /// the path is all this arm adds.
  #[test]
  fn config_unparseable_names_the_path_and_renders_the_fault_unprefixed() {
    let path = std::path::PathBuf::from("/nonexistent/wat.toml");
    let rendered = StartupError::ConfigUnparseable {
      path: path.clone(),
      fault: ConfigError::EmptyCommand,
    }
    .to_string();
    assert_eq!(
      rendered,
      format!("/nonexistent/wat.toml: {}", ConfigError::EmptyCommand)
    );
    assert!(rendered.contains(path.display().to_string().as_str()));
  }

  #[test]
  fn clock_before_epoch() {
    assert_eq!(
      StartupError::Clock(ClockError::BeforeEpoch).to_string(),
      ClockError::BeforeEpoch.to_string()
    );
    assert_eq!(
      ClockError::BeforeEpoch.to_string(),
      "the system clock reads before 1970-01-01T00:00:00Z"
    );
  }

  #[test]
  fn clock_out_of_range() {
    let jiff_error = jiff::Timestamp::from_nanosecond(i128::MAX).unwrap_err();
    let expected =
      format!("the system clock is outside the range this host represents: {jiff_error}");
    assert_eq!(ClockError::OutOfRange(jiff_error).to_string(), expected);
  }

  #[test]
  fn runtime() {
    let io_error = std::io::Error::other("no threads available");
    let expected = format!("the async runtime could not be started: {io_error}");
    assert_eq!(StartupError::Runtime(io_error).to_string(), expected);
  }

  #[test]
  fn platform() {
    let platform_error = slint::PlatformError::from("no display");
    let expected = format!("the display could not be opened: {platform_error}");
    assert_eq!(StartupError::Platform(platform_error).to_string(), expected);
  }

  #[test]
  fn event_loop() {
    let loop_error = slint::EventLoopError::EventLoopTerminated;
    let expected = format!("the event loop would not accept the host task: {loop_error}");
    assert_eq!(StartupError::EventLoop(loop_error).to_string(), expected);
  }

  #[test]
  fn enqueue() {
    assert_eq!(
      StartupError::Enqueue.to_string(),
      "the first request could not be enqueued"
    );
  }

  /// AC-9's stratum 3 half: `Display` renders the `IngressError` unprefixed
  /// — no repeated wrapper text, like `Config`'s and `Clock`'s arms above —
  /// and the message names the path.
  #[test]
  fn ingress_is_unwrapped_and_unprefixed_and_names_the_path() {
    use goad_shell::ingress::{BindFault, IngressError};

    let path = std::path::PathBuf::from("/does/not/exist.sock");
    let error = IngressError {
      path: path.clone(),
      fault: BindFault::InUse,
    };
    let expected = error.to_string();
    assert_eq!(StartupError::Ingress(error).to_string(), expected);
    assert!(expected.contains(path.display().to_string().as_str()));
  }
}

mod source_walk {
  use std::error::Error;

  use super::{ClockError, ConfigError, StartupError};

  /// AC-8, F-47: a chain-walking reporter must not be able to print an
  /// already-rendered inner message a second time. The type overrides
  /// nothing, so every variant, however many there are, answers `None` — the
  /// arms added since are named here as they arrive rather than counted.
  #[test]
  fn startup_error_source_is_always_none() {
    use goad_shell::ingress::{BindFault, IngressError};

    assert!(StartupError::NoConfigPath.source().is_none());
    assert!(StartupError::Usage.source().is_none());
    assert!(
      StartupError::ConfigUnreadable {
        path: std::path::PathBuf::from("/nonexistent/wat.toml"),
        fault: std::io::Error::other("permission denied"),
      }
      .source()
      .is_none()
    );
    assert!(
      StartupError::ConfigUnparseable {
        path: std::path::PathBuf::from("/nonexistent/wat.toml"),
        fault: ConfigError::EmptyCommand,
      }
      .source()
      .is_none()
    );
    assert!(
      StartupError::Clock(ClockError::BeforeEpoch)
        .source()
        .is_none()
    );
    assert!(StartupError::Enqueue.source().is_none());
    assert!(
      StartupError::Ingress(IngressError {
        path: std::path::PathBuf::from("/does/not/exist.sock"),
        fault: BindFault::InUse,
      })
      .source()
      .is_none()
    );
  }

  #[test]
  fn clock_error_source_is_always_none() {
    assert!(ClockError::BeforeEpoch.source().is_none());
    let jiff_error = jiff::Timestamp::from_nanosecond(i128::MAX).unwrap_err();
    assert!(ClockError::OutOfRange(jiff_error).source().is_none());
  }
}

mod usage_block {
  use super::{USAGE, print_usage};

  #[test]
  fn is_one_const_with_no_trailing_newline() {
    assert!(!USAGE.ends_with('\n'));
    assert_eq!(
      USAGE,
      "usage: goad [<config-path>]\n       goad -h | --help\n       goad --version\n\nWith no argument the configuration is read from\n$XDG_CONFIG_HOME/goad/config.toml, and from $HOME/.config/goad/config.toml when\nXDG_CONFIG_HOME is unset, empty, or not absolute."
    );
  }

  /// Byte-identical wherever it appears: there is exactly one `const`, and
  /// `print_usage`'s only job is to hand it, unmodified, to `line_to` — no
  /// second copy exists for a real stdout capture to distinguish it from.
  /// The actual byte stream on stdout is VA-2's, by hand, against the built
  /// binary — this crate's own test process shares stdout across
  /// concurrently running tests, and redirecting the real fd here is not a
  /// safe way to observe it.
  #[test]
  fn print_usage_does_not_panic() {
    print_usage();
  }
}

/// F-7: the two stderr outlets' exact strings (design.md §5.4), asserted
/// against the pure half of each — no sink to fake, no subprocess.
mod stderr_outlets {
  use super::{StartupError, report_platform_line, report_startup_line};

  #[test]
  fn report_startup_line_is_the_error_prefixed_with_goad() {
    assert_eq!(
      report_startup_line(&StartupError::Usage),
      "goad: too many arguments: goad takes at most one, the path of the configuration file; run `goad --help` for usage"
    );
    assert_eq!(
      report_startup_line(&StartupError::NoConfigPath),
      "goad: neither XDG_CONFIG_HOME nor HOME names a directory, so there is no configuration path; pass one as the single argument"
    );
  }

  #[test]
  fn report_platform_line_is_the_detail_in_its_sentence() {
    assert_eq!(
      report_platform_line("no display"),
      "goad: the window could not be drawn: no display"
    );
  }

  /// AC-9's stratum 3 half, the other outlet: `diagnostics::report_startup_line`
  /// renders `Ingress` exactly as it renders every sibling — `goad: {error}`
  /// — since it is generic over `StartupError`'s own `Display` rather than
  /// matching on the variant.
  #[test]
  fn report_startup_line_renders_ingress_like_its_siblings() {
    use goad_shell::ingress::{BindFault, IngressError};

    let error = StartupError::Ingress(IngressError {
      path: std::path::PathBuf::from("/run/goad/ingress.sock"),
      fault: BindFault::InUse,
    });
    assert_eq!(report_startup_line(&error), format!("goad: {error}"));
  }
}

mod usage_error_does_not_reprint_the_block {
  use super::{StartupError, USAGE};

  #[test]
  fn usage_error_text_does_not_contain_the_usage_block() {
    let text = StartupError::Usage.to_string();
    assert!(!text.contains(USAGE));
  }

  #[test]
  fn no_config_path_text_does_not_contain_the_usage_block() {
    let text = StartupError::NoConfigPath.to_string();
    assert!(!text.contains(USAGE));
  }
}

mod arguments_table {
  use std::ffi::OsString;

  use super::{Launch, StartupError, arguments, argv, no_env};

  #[test]
  fn zero_arguments_xdg_config_home_set_absolute() {
    let env = |name: &str| match name {
      "XDG_CONFIG_HOME" => Some("/xdg".into()),
      "HOME" => Some("/home/it".into()),
      _ => None,
    };
    assert_eq!(
      arguments(argv(&[]), &env).unwrap(),
      Launch::Config("/xdg/goad/config.toml".into())
    );
  }

  #[test]
  fn zero_arguments_xdg_config_home_unset() {
    let env = |name: &str| match name {
      "HOME" => Some("/home/it".into()),
      _ => None,
    };
    assert_eq!(
      arguments(argv(&[]), &env).unwrap(),
      Launch::Config("/home/it/.config/goad/config.toml".into())
    );
  }

  #[test]
  fn zero_arguments_xdg_config_home_empty() {
    let env = |name: &str| match name {
      "XDG_CONFIG_HOME" => Some("".into()),
      "HOME" => Some("/home/it".into()),
      _ => None,
    };
    assert_eq!(
      arguments(argv(&[]), &env).unwrap(),
      Launch::Config("/home/it/.config/goad/config.toml".into())
    );
  }

  #[test]
  fn zero_arguments_xdg_config_home_relative() {
    let env = |name: &str| match name {
      "XDG_CONFIG_HOME" => Some("relative".into()),
      "HOME" => Some("/home/it".into()),
      _ => None,
    };
    assert_eq!(
      arguments(argv(&[]), &env).unwrap(),
      Launch::Config("/home/it/.config/goad/config.toml".into())
    );
  }

  #[test]
  fn zero_arguments_xdg_config_home_absolute() {
    let env = |name: &str| match name {
      "XDG_CONFIG_HOME" => Some("/xdg".into()),
      _ => None,
    };
    assert_eq!(
      arguments(argv(&[]), &env).unwrap(),
      Launch::Config("/xdg/goad/config.toml".into())
    );
  }

  #[test]
  fn zero_arguments_home_unset_and_no_xdg() {
    assert!(matches!(
      arguments(argv(&[]), &no_env),
      Err(StartupError::NoConfigPath)
    ));
  }

  #[test]
  fn zero_arguments_home_empty_and_no_xdg() {
    let env = |name: &str| match name {
      "HOME" => Some("".into()),
      _ => None,
    };
    assert!(matches!(
      arguments(argv(&[]), &env),
      Err(StartupError::NoConfigPath)
    ));
  }

  #[test]
  fn zero_arguments_home_relative_is_used_as_given() {
    let env = |name: &str| match name {
      "HOME" => Some("relative".into()),
      _ => None,
    };
    assert_eq!(
      arguments(argv(&[]), &env).unwrap(),
      Launch::Config("relative/.config/goad/config.toml".into())
    );
  }

  #[test]
  fn zero_arguments_home_absolute() {
    let env = |name: &str| match name {
      "HOME" => Some("/home/it".into()),
      _ => None,
    };
    assert_eq!(
      arguments(argv(&[]), &env).unwrap(),
      Launch::Config("/home/it/.config/goad/config.toml".into())
    );
  }

  #[test]
  fn one_argument_is_the_path_verbatim_and_ignores_xdg() {
    let env = |name: &str| match name {
      "XDG_CONFIG_HOME" => Some("/xdg".into()),
      _ => None,
    };
    assert_eq!(
      arguments(argv(&["./here.toml"]), &env).unwrap(),
      Launch::Config("./here.toml".into())
    );
  }

  #[test]
  fn dash_h() {
    assert_eq!(arguments(argv(&["-h"]), &no_env).unwrap(), Launch::Help);
  }

  #[test]
  fn dash_dash_help() {
    assert_eq!(arguments(argv(&["--help"]), &no_env).unwrap(), Launch::Help);
  }

  /// 006/PHASE-03/VT-3, AC-4. The guard sits **before** the catch-all arm, so
  /// `--version` is an invocation rather than a path: until 006 it fell
  /// through to `Launch::Config("--version")` and the host reported that it
  /// could not read a configuration file by that name.
  #[test]
  fn dash_dash_version() {
    assert_eq!(
      arguments(argv(&["--version"]), &no_env).unwrap(),
      Launch::Version
    );
  }

  /// 006/PHASE-03/VT-3. `--version` is a whole invocation and not a flag that
  /// attaches to one: two arguments are still two arguments, and the host does
  /// not guess which was meant (design.md §5.2(e)).
  #[test]
  fn a_path_beside_the_version_flag_is_still_too_many_arguments() {
    assert!(matches!(
      arguments(argv(&["x", "--version"]), &no_env),
      Err(StartupError::Usage)
    ));
  }

  #[test]
  fn two_arguments() {
    assert!(matches!(
      arguments(argv(&["a", "b"]), &no_env),
      Err(StartupError::Usage)
    ));
  }

  /// Every row counts arguments **after** the program name, which
  /// `arguments` skips itself — `argv` here is written the way
  /// `std::env::args_os()` actually yields it, program name first.
  #[test]
  fn the_program_name_is_written_first_and_skipped_by_the_function_not_the_test() {
    assert!(matches!(
      arguments(std::iter::once(OsString::from("goad")), &no_env),
      Err(StartupError::NoConfigPath)
    ));
  }
}

/// `listener` — `main::start`'s own decision of `None` versus `Some`
/// (EX-2). `bind` needs a reactor, so its cases are `#[tokio::test]`s, unlike
/// every other function in this file.
mod listener {
  use std::os::unix::fs::FileTypeExt;

  use super::{IngressConfig, PathBuf, StartupError, listener};
  use crate::scripting::claim;

  /// A path no other case will collide with, cleared before it is handed out —
  /// the third such helper in this test binary, and held by the same
  /// instrument as the other two (`review-code.md` F-11). The stake is the
  /// same one `ingress.rs` names: two cases sharing a name share a listener,
  /// and `cleanup` below unlinks the lock file beside the socket, so the
  /// second case to start would clear the first's mid-run.
  ///
  /// The kind is **this helper's alone**, which is what keeps the key exact.
  /// These paths are `goad-startup-…` and `ingress.rs`'s are `goad-serve-…`, so
  /// a kind shared between the two would report a collision where there is
  /// none. See `claim` for why the rule is one kind per helper rather than one
  /// kind per prefix (`review-code.md` F-21).
  fn socket_path(case: &str) -> PathBuf {
    claim("startup socket", case);
    let path =
      std::env::temp_dir().join(format!("goad-startup-{case}-{}.sock", std::process::id()));
    match std::fs::remove_file(&path) {
      Ok(()) | Err(_) => (),
    }
    path
  }

  /// Removes the socket **and the lock file beside it**: the host unlinks
  /// neither (`SPEC-003/R-5`).
  fn cleanup(path: &std::path::Path) {
    match std::fs::remove_file(path) {
      Ok(()) | Err(_) => (),
    }
    match std::fs::remove_file(goad_shell::ingress::lock_path(path)) {
      Ok(()) | Err(_) => (),
    }
  }

  /// EX-2's `Some` arm: a fresh path binds and returns an `Ingress`. `Ok`
  /// alone does not prove a bind happened — `Ingress` exposes nothing from
  /// outside the crate to tell a real listener from `Ingress::none()` — so
  /// this also asserts the filesystem entry `bind` leaves behind: a real
  /// Unix-domain socket at `path`, which `Ingress::none()`'s path never
  /// produces.
  ///
  /// Paired with `none_binds_nothing` below over the same function: together
  /// the two hold SPEC-003/R-1's *if and only if*, which is a claim about the
  /// **configuration** and so cannot be held by any case that calls `bind`
  /// directly (`docs/slices/004/review-code.md` F-25).
  #[tokio::test]
  async fn some_path_binds() {
    let path = socket_path("some");
    let result = listener(Some(&IngressConfig { path: path.clone() }));
    assert!(result.is_ok(), "{result:?} was not Ok");
    let file_type = std::fs::metadata(&path)
      .unwrap_or_else(|error| panic!("{path:?} was not created: {error}"))
      .file_type();
    assert!(
      file_type.is_socket(),
      "{path:?} is a {file_type:?}, not a socket"
    );
    cleanup(&path);
  }

  /// EX-2's `Some` arm, refused: a regular file at the path is not a socket,
  /// and the error names the path (AC-9).
  #[tokio::test]
  async fn some_path_that_is_a_regular_file_names_the_path() {
    let path = socket_path("regular-file");
    std::fs::write(&path, b"not a socket").unwrap();

    let error = listener(Some(&IngressConfig { path: path.clone() })).unwrap_err();
    assert!(matches!(error, StartupError::Ingress(_)));
    assert!(
      error
        .to_string()
        .contains(path.display().to_string().as_str())
    );

    cleanup(&path);
  }

  /// EX-2's `None` arm, and AC-7's second half: nothing is bound and nothing
  /// is written to the filesystem — a directory watched across the call
  /// contains no new entry, paired with `some_path_binds`'s positive over the
  /// same function so the negative is not vacuous.
  #[tokio::test]
  async fn none_binds_nothing() {
    let dir = std::env::temp_dir().join(format!("goad-startup-none-{}", std::process::id()));
    match std::fs::remove_dir_all(&dir) {
      Ok(()) | Err(_) => (),
    }
    std::fs::create_dir(&dir).unwrap();

    let before: Vec<_> = std::fs::read_dir(&dir).unwrap().collect();
    assert!(before.is_empty());

    let result = listener(None);
    assert!(result.is_ok(), "{result:?} was not Ok");

    let after: Vec<_> = std::fs::read_dir(&dir).unwrap().collect();
    assert!(
      after.is_empty(),
      "listener(None) wrote an entry into {dir:?}"
    );

    std::fs::remove_dir_all(&dir).unwrap();
  }
}

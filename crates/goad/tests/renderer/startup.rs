//! Item 17 — the startup surface, as pure functions with no window
//! (design.md §9 item 17, §5.4's exact strings). `StartupError`'s `Display`
//! for each of its eight variants and `ClockError`'s for both of its,
//! asserted verbatim; the usage block produced by one `const` and
//! byte-identical wherever it appears; a usage error's text not containing
//! the usage block; both `source()`s `None`; and the argument table's rows.
//!
//! No test here runs the binary or asserts an exit code (§9's own rule) —
//! `main`'s one `match` over `run()`'s `Result` is what chooses the code,
//! and every value that `match` sees is already covered here.

use std::ffi::OsString;

use goad::clock::ClockError;
use goad::diagnostics::{USAGE, print_usage};
use goad::startup::{Launch, StartupError, arguments};
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

  #[test]
  fn config_is_unwrapped_and_unprefixed() {
    let expected = ConfigError::Read(std::io::Error::other("permission denied")).to_string();
    let wrapped = StartupError::Config(ConfigError::Read(std::io::Error::other(
      "permission denied",
    )))
    .to_string();
    assert_eq!(wrapped, expected);
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
}

mod source_walk {
  use std::error::Error;

  use super::{ClockError, StartupError};

  /// AC-8, F-47: a chain-walking reporter must not be able to print an
  /// already-rendered inner message a second time.
  #[test]
  fn startup_error_source_is_always_none() {
    assert!(StartupError::NoConfigPath.source().is_none());
    assert!(StartupError::Usage.source().is_none());
    assert!(
      StartupError::Clock(ClockError::BeforeEpoch)
        .source()
        .is_none()
    );
    assert!(StartupError::Enqueue.source().is_none());
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
      "usage: goad [<config-path>]\n       goad -h | --help\n\nWith no argument the configuration is read from\n$XDG_CONFIG_HOME/goad/config.toml, and from $HOME/.config/goad/config.toml when\nXDG_CONFIG_HOME is unset, empty, or not absolute."
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

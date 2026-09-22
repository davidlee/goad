//! Spawning the built binary, shared by every case at this tier.
//!
//! Transcribed from `crates/goad-emit/tests/binary/exchange.rs` rather than
//! reinvented: one convention for spawning a goad binary, not two. It lives in
//! its own module because two case files need it and a helper copied into the
//! second is the same drift this tier exists to catch.
use std::process::{Command, Output};

/// The binary itself, not `cargo run`: `CARGO_BIN_EXE_goad` is set for a test
/// target in the binary's own package, and cargo has already built it.
pub(crate) fn goad(arguments: &[&str]) -> Output {
  command(arguments)
    .output()
    .expect("the built binary must be runnable")
}

/// The same spawn with **neither** configuration variable set, so the process
/// cannot discover a path and cannot reach the machine's real configuration.
/// `env_remove` and not `env_clear`: the loader still needs the rest.
pub(crate) fn goad_with_no_config_home(arguments: &[&str]) -> Output {
  command(arguments)
    .env_remove("XDG_CONFIG_HOME")
    .env_remove("HOME")
    .output()
    .expect("the built binary must be runnable")
}

fn command(arguments: &[&str]) -> Command {
  let mut command = Command::new(env!("CARGO_BIN_EXE_goad"));
  command.args(arguments);
  command
}

pub(crate) fn code_of(output: &Output) -> i32 {
  output
    .status
    .code()
    .expect("the binary must exit rather than be signalled")
}

pub(crate) fn stderr_of(output: &Output) -> String {
  String::from_utf8(output.stderr.clone()).expect("goad writes UTF-8")
}

pub(crate) fn stdout_of(output: &Output) -> String {
  String::from_utf8(output.stdout.clone()).expect("goad writes UTF-8")
}

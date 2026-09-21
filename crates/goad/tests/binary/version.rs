//! `--version`, against the built binary. The helpers are
//! `crates/goad-emit/tests/binary/exchange.rs`'s, transcribed rather than
//! reinvented: one convention for spawning a goad binary, not two.
use std::process::Output;

fn goad(arguments: &[&str]) -> Output {
  std::process::Command::new(env!("CARGO_BIN_EXE_goad"))
    .args(arguments)
    .output()
    .expect("the built binary must be runnable")
}

fn code_of(output: &Output) -> i32 {
  output
    .status
    .code()
    .expect("the binary must exit rather than be signalled")
}

fn stderr_of(output: &Output) -> String {
  String::from_utf8(output.stderr.clone()).expect("goad writes UTF-8")
}

fn stdout_of(output: &Output) -> String {
  String::from_utf8(output.stdout.clone()).expect("goad writes UTF-8")
}

/// 006/PHASE-03/VT-2, AC-4: the version on **stdout**, so a caller can read it
/// as a value, and exit 0.
///
/// Until 006 this invocation was read as a configuration path: `goad` opened a
/// file called `--version`, failed, and exited 2 with a line on stderr. Both
/// halves of that regression are pinned here — the code, and stderr being
/// empty.
#[test]
fn version_prints_the_package_version_on_stdout_and_exits_0() {
  let output = goad(&["--version"]);

  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  assert_eq!(stdout_of(&output).trim_end(), env!("CARGO_PKG_VERSION"));
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
}

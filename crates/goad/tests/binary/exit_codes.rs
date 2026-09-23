//! The exit code `exit::status` chooses, against the built binary.
//!
//! **`nix/module.nix` depends on the failure code by value.** Its `Service`
//! block carries `RestartPreventExitStatus=2`, and that directive is the whole
//! argument OQ-1 gave for the module living in this repository rather than in
//! a consumer's: the numeral is this repository's contract, so this repository
//! holds it. Before these cases, `ExitCode::from(2)` could be changed to
//! `from(1)` and the whole gate stayed green — measured, `review-code.md` F-1
//! — and the unit would then restart a host that never started, the one end
//! its policy leaves to a person.
//!
//! `tests/renderer/startup.rs` holds the **numbers** — `exit::status` is a
//! pure function, and that tier holds every shape it can see. What it cannot
//! reach is the **process**: that the number it answers is what a caller of
//! the binary actually observes. That is the cut between the two tiers and
//! the reason these cases are here and not there.
//!
//! Every case is headless: `process::command`'s spawn removes
//! `WAYLAND_DISPLAY`, `WAYLAND_SOCKET` and `DISPLAY`, so a case that reaches
//! `start` past step 4 fails fast at `PromptWindow::new`, with the display's
//! line, instead of opening a real host.
use std::path::PathBuf;

use goad::diagnostics::{USAGE, report_startup_line};
use goad::startup::StartupError;

use crate::process::{
  code_of, goad, goad_with_no_config_home, goad_with_stdout_full, stderr_of, stdout_of,
};

/// `run` answering `Ok(Ended::AsAsked)` is exit 0, and `--help` is the arm
/// that reaches it with something on stdout. The block itself is one `const`
/// asserted verbatim one tier down; what this holds is that `main` reaches it,
/// puts it on **stdout**, and stops there.
#[test]
fn help_prints_the_usage_block_on_stdout_and_exits_0() {
  let output = goad(&["--help"]);

  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  assert_eq!(stdout_of(&output).trim_end(), USAGE);
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
}

/// `StartupError::Usage` — the arm that settles before anything is opened, so
/// it costs no file and no socket. The whole line is pinned, which also pins
/// the `"goad: "` prefix on a real process: `report_startup_line` is asserted
/// pure one tier down, and this says the process actually writes it, to
/// **stderr**, with nothing on stdout.
#[test]
fn too_many_arguments_exits_2_and_says_who_spoke() {
  let output = goad(&["one.toml", "two.toml"]);

  assert_eq!(code_of(&output), 2, "{}", stderr_of(&output));
  assert_eq!(
    stderr_of(&output).trim_end(),
    report_startup_line(&StartupError::Usage)
  );
  assert!(output.stdout.is_empty(), "{}", stdout_of(&output));
}

/// `StartupError::NoConfigPath` — no argument, and neither variable naming a
/// directory. Exit 2 like every other startup failure: the host distinguishes
/// them in the **line**, never in the code (005/D-4 makes the same cut for
/// `goad-emit`, where 1 and 2 mean different things because a refusal is an
/// answer; a host that never started has no answer to give).
#[test]
fn no_argument_and_no_configuration_home_exits_2() {
  let output = goad_with_no_config_home(&[]);

  assert_eq!(code_of(&output), 2, "{}", stderr_of(&output));
  assert_eq!(
    stderr_of(&output).trim_end(),
    report_startup_line(&StartupError::NoConfigPath)
  );
}

/// `StartupError::ConfigUnreadable` — 006/AC-6's arm, at the tier that runs
/// the binary. The `fault` is the OS's own text and varies, so what is
/// asserted is the arm's **own** rendering up to it: the path, then a space,
/// then *could not be read*. The remedy is a thing done to that file, and
/// until 006 this line named no file at all (`research.md` S-4).
///
/// **Each case asserts the prefix only its own arm produces** (F-7, and
/// `goad-emit`'s `render.rs` says it first). Naming the file is common to
/// both arms, and so is exiting 2 and saying `goad: `, so a case asserting
/// only those three survives routing a read failure into the parse arm —
/// after which a person whose configuration is absent reads
/// `goad: <path>: configuration could not be read`, the doubled prefix
/// `design.md` §7 D2 chose two arms to avoid.
#[test]
fn an_unreadable_configuration_exits_2_and_says_only_what_its_own_arm_says() {
  let output = goad(&["/nonexistent/wat.toml"]);

  assert_eq!(code_of(&output), 2, "{}", stderr_of(&output));
  let stderr = stderr_of(&output);
  assert!(
    stderr.starts_with("goad: /nonexistent/wat.toml could not be read: "),
    "{stderr}"
  );
}

/// `StartupError::ConfigUnparseable` — the other half of 006's split, and the
/// one that proves the split is a split: a file that *was* read and is not a
/// configuration exits with the same code as one that could not be read, and
/// names the same path.
///
/// Its own rendering is the path and a **colon**, with stratum 2's message
/// behind it unprefixed — this arm adds nothing but the file name, because
/// `ConfigError` already says what was wrong with the contents. Asserted to
/// the colon and no further: the text past it is `toml`'s, carries a line, a
/// column and a caret excerpt, and is not this tier's to pin.
#[test]
fn an_unparseable_configuration_exits_2_and_says_only_what_its_own_arm_says() {
  let path = scratch_config("unparseable", "this is not toml {{{");

  let output = goad(&[&path.display().to_string()]);
  let stderr = stderr_of(&output);
  drop(std::fs::remove_file(&path));

  assert_eq!(code_of(&output), 2, "{stderr}");
  assert!(
    stderr.starts_with(&format!("goad: {}: ", path.display())),
    "{stderr}"
  );
}

/// `StartupError::Ingress` — `startup::listener` runs at `start` step 3,
/// before any Slint call, so a regular file at the configured path is
/// refused there and this case stays headless.
///
/// **The status alone would not do.** Headless, a bindable path also exits 2
/// — at `PromptWindow::new`, with the display's line — so a case asserting
/// only `code_of(&output)` is green whether the socket bound or not. What
/// tells the two apart is the **prefix**: `goad: ` and the socket's own path,
/// which only `StartupError::Ingress`'s line begins with (P-1,
/// `design-log.md`).
#[test]
fn an_unbindable_ingress_path_exits_2() {
  let socket = scratch_path("an_unbindable_ingress_path", "socket");
  std::fs::write(&socket, "").expect("the scratch socket path must be writable");
  let config = scratch_config(
    "an_unbindable_ingress_path",
    &format!(
      "[backend]\ncommand = [\"true\"]\ntimeout = \"5s\"\n\n[schedule]\ndefault_poll = \"30s\"\n\n[ingress]\npath = \"{}\"\n",
      socket.display()
    ),
  );

  let output = goad(&[&config.display().to_string()]);
  let stderr = stderr_of(&output);
  drop(std::fs::remove_file(&config));
  drop(std::fs::remove_file(&socket));

  assert_eq!(code_of(&output), 2, "{stderr}");
  assert!(
    stderr.starts_with(&format!("goad: {}: ", socket.display())),
    "{stderr}"
  );
}

/// `StartupError::AnswerUnwritten` — a question whose answer never reached
/// standard output was not answered, so it is not exit 0 (R-1's *only if*):
/// it is a failure before the loop call, 2, with its line on stderr. Both
/// questions, at this tier and not one down: the write that fails is to the
/// process's own stdout, which only a spawn can put on a device that refuses
/// it — `print_usage` and `print_version` lock the stdout of whatever process
/// calls them, and at the renderer tier that is the test harness's.
#[test]
fn an_answer_that_cannot_be_written_exits_2() {
  for question in ["--help", "--version"] {
    let output = goad_with_stdout_full(&[question]);
    let stderr = stderr_of(&output);

    assert_eq!(code_of(&output), 2, "{question}: {stderr}");
    assert!(
      stderr.starts_with("goad: the answer could not be written to standard output: "),
      "{question}: {stderr}"
    );
  }
}

/// A file of this case's own, named for the case and the process, so a
/// parallel run cannot collide with a sibling.
fn scratch_config(case: &str, contents: &str) -> PathBuf {
  let path = std::env::temp_dir().join(format!("goad-{case}-{}.toml", std::process::id()));
  std::fs::write(&path, contents).expect("the scratch configuration must be writable");
  path
}

/// A path of this case's own, named the same way `scratch_config` names its
/// file, for a case that needs a scratch path `scratch_config` does not
/// write itself — here, the regular file occupying the would-be socket.
fn scratch_path(case: &str, extension: &str) -> PathBuf {
  std::env::temp_dir().join(format!("goad-{case}-{}.{extension}", std::process::id()))
}

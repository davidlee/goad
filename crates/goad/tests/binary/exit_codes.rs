//! The exit code `main` chooses, against the built binary.
//!
//! **`nix/module.nix` depends on the failure code by value.** Its `Service`
//! block carries `RestartPreventExitStatus=2`, and that directive is the whole
//! argument OQ-1 gave for the module living in this repository rather than in
//! a consumer's: the numeral is this repository's contract, so this repository
//! holds it. Before these cases, `ExitCode::from(2)` could be changed to
//! `from(1)` and the whole gate stayed green — measured, `review-code.md` F-1
//! — and the consequence is the one the directive exists to prevent:
//! `Restart=on-failure` restarting a host that cannot start, on a bad
//! configuration or an ingress socket already held, until systemd's start
//! limiter gives up.
//!
//! `tests/renderer/startup.rs` covers the **arms** — every value `main`'s one
//! `match` over `run()`'s `Result` can see. What it cannot see is the constant
//! each arm names, because no pure test runs a process. That is the cut
//! between the two tiers and the reason these cases are here and not there.
//!
//! Every case is headless: each reaches its answer before the first Slint
//! call (006/design.md §5.4). A case that reached `start` past step 4 would
//! need a compositor and would red on every machine the gate runs on.
use std::path::PathBuf;

use goad::diagnostics::{USAGE, report_startup_line};
use goad::startup::StartupError;

use crate::process::{code_of, goad, goad_with_no_config_home, stderr_of, stdout_of};

/// `Ok(())` is exit 0, and `--help` is the arm that reaches it with something
/// on stdout. The block itself is one `const` asserted verbatim one tier down;
/// what this holds is that `main` reaches it, puts it on **stdout**, and stops
/// there.
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
/// the binary. The `fault` is the OS's own text and varies, so the path is
/// what is asserted: the remedy is a thing done *to that file*, and until 006
/// this line named no file at all (`research.md` S-4).
#[test]
fn an_unreadable_configuration_exits_2_and_names_the_path() {
  let output = goad(&["/nonexistent/wat.toml"]);

  assert_eq!(code_of(&output), 2, "{}", stderr_of(&output));
  let stderr = stderr_of(&output);
  assert!(stderr.starts_with("goad: "), "{stderr}");
  assert!(stderr.contains("/nonexistent/wat.toml"), "{stderr}");
}

/// `StartupError::ConfigUnparseable` — the other half of 006's split, and the
/// one that proves the split is a split: a file that *was* read and is not a
/// configuration exits with the same code as one that could not be read, and
/// names the same path.
#[test]
fn an_unparseable_configuration_exits_2_and_names_the_path() {
  let path = scratch_config("unparseable", "this is not toml {{{");

  let output = goad(&[&path.display().to_string()]);
  let stderr = stderr_of(&output);
  drop(std::fs::remove_file(&path));

  assert_eq!(code_of(&output), 2, "{stderr}");
  assert!(stderr.starts_with("goad: "), "{stderr}");
  assert!(stderr.contains(&path.display().to_string()), "{stderr}");
}

/// A file of this case's own, named for the case and the process, so a
/// parallel run cannot collide with a sibling.
fn scratch_config(case: &str, contents: &str) -> PathBuf {
  let path = std::env::temp_dir().join(format!("goad-{case}-{}.toml", std::process::id()));
  std::fs::write(&path, contents).expect("the scratch configuration must be writable");
  path
}

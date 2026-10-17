//! The shipped examples under the checker (`design.md` §5.2.6).
//!
//! Every directory under `kit/skills/goad-backend/examples/` is copied to a
//! temporary directory and `goad-check --config config.toml` is started
//! there, with the example's event files and a temporary `HOME` and `XDG_*`.
//! The helpers are example-blind; only the triage case names an example.
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use std::process::Output;

use serde_json::Value;

/// The line `render` writes when no exchange returned a view. A third copy,
/// after `render::NO_VIEW` and `tests/binary/run.rs`'s.
const NO_VIEW: &str = "no exchange returned a view, so respond was not exercised";

/// What every run has: the copy the checker starts in, and the temporary
/// directories the example's environment names.
struct Handout {
  root: PathBuf,
}

impl Handout {
  fn example(&self) -> PathBuf {
    self.root.join("example")
  }

  fn downloads(&self) -> PathBuf {
    self.root.join("downloads")
  }

  fn documents(&self) -> PathBuf {
    self.root.join("documents")
  }
}

/// Every directory under the examples root, sorted. Enumerated, never listed.
fn examples() -> Vec<PathBuf> {
  let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kit/skills/goad-backend/examples");
  let mut found: Vec<PathBuf> = fs::read_dir(&root)
    .unwrap_or_else(|e| panic!("the examples root {} must be readable: {e}", root.display()))
    .map(|entry| entry.expect("a directory entry").path())
    .filter(|path| path.is_dir())
    .collect();
  found.sort();
  found
}

/// Directories and regular files, recursively. A unix socket is skipped by
/// type: a host run in an example's directory leaves one there, and opening
/// it to copy it fails. Anything else (a symlink) is not expected.
fn copy_tree(from: &Path, to: &Path) {
  fs::create_dir_all(to).expect("the copy's directory");
  for entry in fs::read_dir(from).expect("a directory to copy") {
    let entry = entry.expect("a directory entry");
    let (source, target) = (entry.path(), to.join(entry.file_name()));
    let kind = entry.file_type().expect("a file type");
    if kind.is_dir() {
      copy_tree(&source, &target);
    } else if kind.is_file() {
      fs::copy(&source, &target).expect("a file to copy");
    } else if !kind.is_socket() {
      panic!(
        "{} is neither a file, a directory nor a socket",
        source.display()
      );
    }
  }
}

/// A temporary directory per `case` and example, left after the run so a
/// failure can be read; the pid last, so it cannot end `.sock`. A helper of
/// `tests/support/scripting.rs` `claim`'s class, unheld by it for the reason
/// `claim`'s doc gives `goad-emit`'s two: including the file costs `dead_code`.
fn handout(case: &str, example: &Path) -> Handout {
  let name = example.file_name().expect("an example has a name");
  let root = std::env::temp_dir().join(format!(
    "goad-kit-{case}-{}-{}",
    name.to_string_lossy(),
    std::process::id()
  ));
  if root.exists() {
    fs::remove_dir_all(&root).expect("a stale handout to clear");
  }
  copy_tree(example, &root.join("example"));
  Handout { root }
}

/// The checker, started in the copy in the config form with each of its event
/// files, every inherited `XDG_*` removed and the temporary ones set. `PATH`
/// is inherited: the interpreters are the devshell's.
fn run_checker(handout: &Handout) -> Output {
  let example = handout.example();
  let mut events: Vec<PathBuf> = fs::read_dir(example.join("events"))
    .map_or_default(|dir| dir.map(|entry| entry.expect("an entry").path()).collect());
  events.sort();
  let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_goad-check"));
  command
    .current_dir(&example)
    .args(["--config", "config.toml"]);
  for event in events {
    command
      .arg("--event")
      .arg(event.strip_prefix(&example).expect("under the copy"));
  }
  for (name, _) in std::env::vars_os() {
    if name.to_string_lossy().starts_with("XDG_") {
      command.env_remove(name);
    }
  }
  let root = &handout.root;
  command
    .env("HOME", root.join("home"))
    .env("XDG_STATE_HOME", root.join("state"))
    .env("XDG_DOWNLOAD_DIR", handout.downloads())
    .env("XDG_DOCUMENTS_DIR", handout.documents());
  command.output().expect("the built binary must be runnable")
}

/// A copy of `tests/binary/process.rs`'s `code_of`.
fn code_of(output: &Output) -> i32 {
  output
    .status
    .code()
    .expect("the binary must exit rather than be signalled")
}

/// A copy of `tests/binary/process.rs`'s `stdout_of`.
fn stdout_of(output: &Output) -> String {
  String::from_utf8(output.stdout.clone()).expect("goad-check writes UTF-8")
}

/// A copy of `tests/binary/process.rs`'s `stderr_of`.
fn stderr_of(output: &Output) -> String {
  String::from_utf8(output.stderr.clone()).expect("goad-check writes UTF-8")
}

/// Why this run is not acceptable: exit 0, a respond line, and no no-view line.
fn complaint(output: &Output) -> Option<String> {
  let stdout = stdout_of(output);
  let answered = stdout.lines().any(|line| line.starts_with("  respond "));
  let unanswered = stdout.lines().any(|line| line == NO_VIEW);
  (code_of(output) != 0 || !answered || unanswered).then(|| {
    format!(
      "exit {}, a view was answered: {}\nstdout:\n{stdout}\nstderr:\n{}",
      code_of(output),
      answered && !unanswered,
      stderr_of(output)
    )
  })
}

#[test]
fn each_shipped_example_is_accepted_by_the_checker() {
  let examples = examples();
  assert!(!examples.is_empty(), "the kit ships no example to check");
  let failures: BTreeMap<String, String> = examples
    .iter()
    .filter_map(|example| {
      let output = run_checker(&handout("accepted", example));
      let name = example.file_name()?.to_string_lossy().into_owned();
      complaint(&output).map(|why| (name, why))
    })
    .collect();
  assert!(
    failures.is_empty(),
    "examples not accepted:\n{}",
    failures
      .iter()
      .map(|(name, why)| format!("--- {name}: {why}"))
      .collect::<Vec<_>>()
      .join("\n")
  );
}

#[test]
fn downloads_triage_moves_the_file_it_was_asked_about() {
  let example = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../kit/skills/goad-backend/examples/downloads-triage");
  let handout = handout("triage", &example);
  let event: Value = serde_json::from_str(
    &fs::read_to_string(handout.example().join("events/file-arrived.json"))
      .expect("the triage example's event file"),
  )
  .expect("the event file is JSON");
  let name = event["data"]["name"]
    .as_str()
    .expect("the event names a file");
  let content = format!("content unique to pid {}", std::process::id());
  fs::create_dir_all(handout.downloads()).expect("the downloads directory");
  fs::write(handout.downloads().join(name), &content).expect("the arrived file");

  let output = run_checker(&handout);

  assert_eq!(
    code_of(&output),
    0,
    "{}\n{}",
    stdout_of(&output),
    stderr_of(&output)
  );
  assert!(
    !handout.downloads().join(name).exists(),
    "the file was left in downloads:\n{}",
    stdout_of(&output)
  );
  assert_eq!(
    fs::read_to_string(handout.documents().join(name)).expect("the file moved to documents"),
    content
  );
}

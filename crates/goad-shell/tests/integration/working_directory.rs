//! Where a loaded configuration is anchored, and what the backend sees from
//! there. The anchor is the canonical directory of the file, however the file
//! was reached; a backend loaded from it runs in that directory with `PWD`
//! naming it, a relative program with a `/` is executed by its anchored path,
//! and a failure to spawn names both (SPEC-001/R-60).
//!
//! Every case loads its configuration from a file with `Config::load`, in a
//! directory of its own: the test binary's working directory — the crate — is
//! never the configuration's, which is what makes each case red under the old
//! rule. Whatever a case puts on disk — a backend, a decoy, a log — it writes
//! into that directory.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::harness::{describe, evaluate};
use crate::scripting::claim;
use goad_shell::backend::process::ProcessBackend;
use goad_shell::backend::transport::{Backend, Exchange};
use goad_shell::config::Config;
use goad_shell::error::BackendError;

const CONFIG: &str = r#"
[backend]
command = ["bash", "backend.sh"]
timeout = "5s"

[schedule]
default_poll = "30m"

[ingress]
path = "./goad.sock"
"#;

/// A directory of this case's own under `std::env::temp_dir()`, emptied before
/// it is handed out and held by `claim` under a kind of its own. The name ends
/// `.d` after the pid, so it can never equal a `marker` path, which ends in
/// the pid (`claim`, *one kind per helper*).
fn directory(case: &str) -> PathBuf {
  claim("working directory", case);
  let path = std::env::temp_dir().join(format!("goad-wd-{case}-{}.d", std::process::id()));
  match std::fs::remove_dir_all(&path) {
    Ok(()) | Err(_) => (),
  }
  std::fs::create_dir_all(&path).expect("the temp directory must be writable");
  path
}

/// `design.md` §5.5's I-1, for the string anchor: the command's working
/// directory and the socket path name the **target** directory, not the link
/// that led to it. A link in a directory component leaves the kernel's working
/// directory the same either way — a directory reached through a link is the
/// target directory — so it is held on the loaded value, from which `PWD`, a
/// relative program's name and the socket path are all taken.
#[test]
fn a_config_reached_through_a_linked_directory_is_anchored_at_the_target() {
  let root = directory("linked");
  let real = root.join("real");
  std::fs::create_dir(&real).expect("the target directory must be creatable");
  let link = root.join("link");
  std::os::unix::fs::symlink(&real, &link).expect("the link must be creatable");
  std::fs::write(real.join("config.toml"), CONFIG).expect("the configuration must be writable");

  let config = Config::load(&link.join("config.toml")).expect("the file just written must load");

  let anchor = std::fs::canonicalize(&real).expect("the target directory exists");
  assert_ne!(anchor, link, "the control: the link is not the anchor");
  assert_eq!(
    config.backend.command.working_directory,
    Some(anchor.clone())
  );
  assert_eq!(
    config.ingress.expect("the section was present").path,
    anchor.join("goad.sock")
  );
  std::fs::remove_dir_all(&root).expect("the case's directory must be removable");
}

/// What the target backend answers. Each case's answer is asserted, so a run of
/// some other backend is a wrong answer and not a coincidence.
const ANSWER: &str = r#"{"view":null}"#;

/// A configuration whose `command` is `command`, each argument quoted by
/// `toml`'s own serializer.
fn configuration(command: &[&str]) -> String {
  let command: Vec<String> = command
    .iter()
    .map(|argument| toml::Value::from(*argument).to_string())
    .collect();
  format!(
    "[backend]\ncommand = [{}]\ntimeout = \"5s\"\n\n[schedule]\ndefault_poll = \"30m\"\n",
    command.join(", ")
  )
}

/// `contents` written to `path` and made executable.
fn write_executable(path: &Path, contents: &str) {
  std::fs::write(path, contents).expect("the backend must be writable");
  std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
    .expect("the backend must be made executable");
}

/// A backend that appends where it ran (`pwd -P`) and the name it was run by
/// (`$0`) to the file its first argument names, then answers `answer`. A
/// shebang and an executable bit, so `["bash", "backend.sh", log]` and
/// `["./backend.sh", log]` run it alike.
fn logging_script(path: &Path, answer: &str) {
  write_executable(
    path,
    &format!(
      "#!/bin/sh\n\
       cat >/dev/null\n\
       printf '%s\\n%s\\n' \"$(pwd -P)\" \"$0\" >>\"$1\"\n\
       printf '%s\\n' '{answer}'\n"
    ),
  );
}

/// `config.toml` in `directory`, naming `command`.
fn config_in(directory: &Path, command: &[&str]) -> PathBuf {
  let path = directory.join("config.toml");
  std::fs::write(&path, configuration(command)).expect("the configuration must be writable");
  path
}

/// One exchange with the backend the configuration at `path` names.
async fn exchanged(path: &Path) -> Exchange {
  let config = Config::load(path).expect("the file just written must load");
  let mut backend = ProcessBackend::new(config.backend.command, config.backend.timeout);
  backend.exchange(&evaluate()).await
}

/// The body an exchange answered, as text.
fn answered(exchange: &Exchange) -> String {
  let body = exchange
    .result
    .as_ref()
    .unwrap_or_else(|_| panic!("the backend must answer: {}", describe(&exchange.result)));
  String::from_utf8_lossy(body).trim().to_owned()
}

/// What a backend wrote to its log, or nothing if it never ran.
fn logged(log: &Path) -> String {
  std::fs::read_to_string(log).unwrap_or_default()
}

/// The canonical form of `path`, as text: what a backend's `pwd -P` and `PWD`
/// are compared against, byte for byte.
fn canonical(path: &Path) -> String {
  std::fs::canonicalize(path)
    .expect("the case's directory exists")
    .to_str()
    .expect("the temp directory is UTF-8")
    .to_owned()
}

/// AC-1: `["bash", "backend.sh"]` reads `backend.sh` beside the
/// configuration, and receives the argument as written.
#[tokio::test]
async fn a_relative_argument_resolves_beside_the_config_from_another_directory() {
  let root = directory("argument");
  let log = root.join("log");
  logging_script(&root.join("backend.sh"), ANSWER);
  let config = config_in(&root, &["bash", "backend.sh", argument(&log)]);

  let exchange = exchanged(&config).await;

  assert_eq!(answered(&exchange), ANSWER);
  assert_eq!(logged(&log), format!("{}\nbackend.sh\n", canonical(&root)));
  std::fs::remove_dir_all(&root).expect("the case's directory must be removable");
}

/// AC-1, through a link to the file: the backend beside the **target** runs.
/// The decoy beside the link is a real backend that answers differently, so
/// an implementation that anchored at the link's directory would be seen to.
#[tokio::test]
async fn a_linked_config_runs_its_backend_beside_the_target() {
  let root = directory("linked-file");
  let (real, elsewhere) = (root.join("real"), root.join("elsewhere"));
  std::fs::create_dir(&real).expect("the target directory must be creatable");
  std::fs::create_dir(&elsewhere).expect("the link's directory must be creatable");
  let log = root.join("log");
  logging_script(&real.join("backend.sh"), ANSWER);
  let decoy = r#"{"view":null,"next_check":"1 minute"}"#;
  logging_script(&elsewhere.join("backend.sh"), decoy);
  let config = config_in(&real, &["bash", "backend.sh", argument(&log)]);
  let link = elsewhere.join("config.toml");
  std::os::unix::fs::symlink(&config, &link).expect("the link must be creatable");

  let exchange = exchanged(&link).await;

  assert_ne!(decoy, ANSWER, "the control: the decoy answers differently");
  assert_eq!(answered(&exchange), ANSWER);
  assert_eq!(logged(&log), format!("{}\nbackend.sh\n", canonical(&real)));
  std::fs::remove_dir_all(&root).expect("the case's directory must be removable");
}

/// AC-1: `["./backend.sh"]` is executed as the anchored path, which a script
/// receives as its `$0` whatever argv[0] says (`design.md` §7 D5). The
/// platform's own route — `./backend.sh` under `current_dir` — would run it
/// too, and give `$0` as written.
#[tokio::test]
async fn a_relative_program_with_a_slash_is_executed_as_the_anchored_path() {
  let root = directory("program");
  let log = root.join("log");
  logging_script(&root.join("backend.sh"), ANSWER);
  let config = config_in(&root, &["./backend.sh", argument(&log)]);

  let exchange = exchanged(&config).await;

  assert_eq!(answered(&exchange), ANSWER);
  let anchor = canonical(&root);
  assert_eq!(logged(&log), format!("{anchor}\n{anchor}/backend.sh\n"));
  std::fs::remove_dir_all(&root).expect("the case's directory must be removable");
}

/// AC-1, for a program that is not a script: a copy of the devshell's `sh` —
/// the real binary, not a link, so its argv[0] is what it reports — beside the
/// configuration as `shell`. `sh -c` with no operand gives argv[0] as `$0`,
/// which is the anchored path, as a script's is; `arg0(program)` would give
/// `./shell` here and leave a script unchanged.
#[tokio::test]
async fn a_native_program_receives_its_anchored_path_as_its_name() {
  let root = directory("native");
  let log = root.join("log");
  let found = std::process::Command::new("sh")
    .args(["-c", "command -v sh"])
    .output()
    .expect("the devshell has a sh");
  let sh = std::fs::canonicalize(String::from_utf8_lossy(&found.stdout).trim())
    .expect("the sh the devshell names exists");
  std::fs::copy(&sh, root.join("shell")).expect("sh must be copyable");
  let body = format!(
    "cat >/dev/null; printf '%s\\n' \"$0\" >>'{}'; printf '%s\\n' '{ANSWER}'",
    argument(&log)
  );
  let config = config_in(&root, &["./shell", "-c", &body]);

  let exchange = exchanged(&config).await;

  assert_eq!(answered(&exchange), ANSWER);
  assert_eq!(logged(&log), format!("{}/shell\n", canonical(&root)));
  std::fs::remove_dir_all(&root).expect("the case's directory must be removable");
}

/// AC-1, `design.md` §5.5 I-4: `current_dir` leaves `PWD` as inherited, and a
/// runtime that is not a shell reads it as is. `python3` does, where a shell
/// would repair a stale one and hide the regression (`review-design.md` F-7).
/// No relative path in the command, so the backend runs wherever the spawn
/// puts it, and the case turns on `PWD` alone.
#[tokio::test]
async fn the_backend_s_pwd_names_its_working_directory() {
  let root = directory("pwd");
  let log = root.join("log");
  let body = "import json, os, sys; sys.stdin.read(); \
              open(sys.argv[1], \"w\").write(os.environ[\"PWD\"]); \
              print(json.dumps({\"view\": None}))";
  let config = config_in(&root, &["python3", "-c", body, argument(&log)]);

  let exchange = exchanged(&config).await;

  assert_eq!(answered(&exchange), r#"{"view": null}"#);
  assert_eq!(logged(&log), canonical(&root));
  std::fs::remove_dir_all(&root).expect("the case's directory must be removable");
}

/// AC-1: a program not beside its configuration is reported with the path
/// that was executed and the directory it was looked for in — neither of
/// which the person may have typed, when they reached the file through a link
/// (`design.md` §7 D13).
#[tokio::test]
async fn a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for() {
  let root = directory("missing");
  let config = config_in(&root, &["./missing.sh"]);

  let exchange = exchanged(&config).await;

  let anchor = std::fs::canonicalize(&root).expect("the case's directory exists");
  let Err(
    error @ BackendError::Spawn {
      executable,
      working_directory,
      ..
    },
  ) = &exchange.result
  else {
    panic!(
      "expected a spawn failure, got {}",
      describe(&exchange.result)
    );
  };
  assert_eq!(
    executable.as_os_str(),
    anchor.join("missing.sh").as_os_str()
  );
  assert_eq!(working_directory.as_deref(), Some(anchor.as_path()));
  let report = error.to_string();
  assert!(
    report.contains(&format!(
      "{}/missing.sh (in {})",
      canonical(&root),
      canonical(&root)
    )),
    "{report}"
  );
  std::fs::remove_dir_all(&root).expect("the case's directory must be removable");
}

/// `path` as a command argument.
fn argument(path: &Path) -> &str {
  path.to_str().expect("the temp directory is UTF-8")
}

//! Where a loaded configuration is anchored: the canonical directory of the
//! file, however the file was reached. The spawn's half of this — what the
//! backend sees from that directory — is added beside these cases.

use std::path::PathBuf;

use crate::scripting::claim;
use goad_shell::config::Config;

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

/// `design.md` §5.2's I-1, for the string anchor: `PWD`, the socket path and
/// its byte length all see the **target** directory, not the link that led to
/// it. Not observable at the spawn, where a working directory reached through
/// a directory link *is* the target directory.
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

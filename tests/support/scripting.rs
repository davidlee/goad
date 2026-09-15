//! The scripted-backend half of slice 001's test helpers, split out of
//! `driving.rs` at slice 003 PHASE-05 (D-18, FD-3, PL-5) so a target that
//! only drives a scripted backend — and never composes a `Host` through
//! `driving.rs`'s own `host`/`host_from`/`config` — can include this file
//! alone and satisfy `dead_code` (`warn`, promoted to an error by the gate's
//! `-D warnings`) without restating anything or suppressing the lint.
//!
//! Included the same way `driving.rs` is: `#[path =
//! "../../../../tests/support/scripting.rs"]`, four levels up from
//! `crates/<member>/tests/<target>/`, the repository root
//! (`docs/memory/cargo-test-cwd-is-package-root-not-workspace-root.md`).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use goad_shell::config::Command;

/// The argument vector for one of `tests/backends/`'s scripts.
///
/// `bash` is argv[0] and the script is argv[1], which is R-36's rule and AC-12's
/// own example — so the scripts need neither a shebang nor an executable bit.
/// Rooted at the crate, not the cwd, exactly as `crates/goad-boundary/src/scan.rs`
/// does it: a test binary's working directory is not something to rely on.
pub(crate) fn backend(name: &str) -> Command {
  let script = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../tests/backends")
    .join(format!("{name}.sh"));
  Command::new("bash", vec![script.display().to_string()])
}

/// The names handed out in this process, per **kind** of path, so that the
/// uniqueness these helpers promise is held by something rather than by
/// everyone remembering.
///
/// Per process, which is the scope a collision actually has: every path here is
/// qualified by pid, so two *targets* may use one name freely and two cases in
/// one binary may not. `cargo test` runs a target's cases on parallel threads,
/// hence the `Mutex`. Keyed by kind as well as name because the kinds mint
/// different paths — `goad-<name>-<pid>` and `goad-serve-<name>-<pid>.sock` —
/// and two callers of *different* kinds sharing a name do not collide.
static CLAIMED: LazyLock<Mutex<BTreeSet<(&'static str, String)>>> = LazyLock::new(Mutex::default);

/// Reserve `name` for one caller of `kind`, for the life of this test binary.
///
/// The instrument behind the "a path no other case will collide with" promises
/// in this repository's test support. Call it wherever such a path is minted;
/// it is independent of what the path is *for*, which is why one registry
/// serves every kind (`review-code.md` F-5, F-9, F-11).
///
/// **Five helpers make that promise and this holds four** (F-17). The four are
/// `marker` below and the `socket_path`s in
/// `crates/goad/tests/renderer/ingress.rs`,
/// `crates/goad/tests/renderer/startup.rs` and
/// `crates/goad-shell/tests/integration/ingress.rs`. The fifth,
/// `crates/goad-emit/tests/binary/exchange.rs`, is not held and the reason is
/// mechanical rather than an oversight: its target does not `#[path]`-include
/// this file, and adding the include costs nine `dead_code` warnings — an
/// error under the gate's `-D warnings`, which is the reason this file is split
/// from `driving.rs` in the first place. Reaching it means moving `claim` to a
/// support file of its own, which is a follow-up (`slice-007.md`), not a line
/// here.
///
/// **One kind per helper** is what keeps the key exact — two helpers sharing a
/// kind would report a collision between paths that differ. A kind is *not* a
/// concept and, despite three of the four reading that way, it is not the
/// prefix either: the four mint `goad-<name>-<pid>`,
/// `goad-serve-<name>-<pid>.sock`, `goad-startup-<name>-<pid>.sock` and
/// `goad-ingress-<name>-<pid>.sock`, and `marker`'s prefix is the bare `goad-`
/// that all of them share (F-21). What separates a marker's paths from a
/// socket's is the `.sock` suffix. A fifth kind must therefore be checked
/// against the paths the others mint, not assumed distinct because its name is.
///
/// # Panics
///
/// When one name is claimed twice for one kind in one test binary. Until slice
/// 007 nothing held this: `scheduling.rs` and `wiring.rs` both asked for
/// `"vt8"`, so they shared one path, and `clear` below — which runs at
/// **handout** — meant whichever case started second deleted the other's log
/// mid-run. It failed one run in six, which is worse than failing every run:
/// `just check` is the gate, and a gate that is green five times in six is not
/// one. A second, silent pair (`table.rs`'s `"table-inert-option"`, minted
/// twice — by the two of the loop's thirty-three rows that carry a
/// `RespondFabricated` turn) was found by this assertion on the first run
/// after it was added.
///
/// A panic rather than a source scan because the breach is exact and local. A
/// scan would have to parse call sites, could not see a name built at runtime —
/// which is exactly `table.rs`'s shape — and would report a file where this
/// reports the case.
pub(crate) fn claim(kind: &'static str, name: &str) {
  let fresh = CLAIMED
    .lock()
    .expect("the name registry must not be poisoned")
    .insert((kind, name.to_owned()));
  assert!(
    fresh,
    "two cases in one test binary claimed the {kind} name {name:?}{}. Such a path is qualified \
     by pid, not by case, so the two share one file and whichever case starts second clears it \
     from under the first. Give one of them a name of its own.",
    spelled_at_the_call_site(name)
  );
}

/// The call site spells an unprefixed case name, and the registry holds the
/// prefixed one, so a panic quoting only the key names a string that appears
/// nowhere in the repository (`review-code.md` F-10). This closes the last step
/// for the one prefix that exists.
fn spelled_at_the_call_site(name: &str) -> String {
  name
    .strip_prefix("invocations-")
    .map_or_else(String::new, |case| {
      format!(
        ", which the call site spells `scripted({case:?}, …)` or `logging_backend(.., {case:?})`"
      )
    })
}

/// A path in the temp directory that no other case will collide with, cleared
/// before it is handed out.
///
/// The backend writes it to report something no in-band channel can carry,
/// because the host kills the backend the moment it has what it needs.
///
/// # Panics
///
/// Through [`claim`], when one name is asked for twice in one test binary.
pub(crate) fn marker(name: &str) -> PathBuf {
  claim("marker", name);
  let path = std::env::temp_dir().join(format!("goad-{name}-{}", std::process::id()));
  clear(&path);
  path
}

/// Remove a marker if it is there. Its absence is the normal case at both ends —
/// before the case, because nothing has written it yet, and after, because a
/// failing case is one where it never appeared.
pub(crate) fn clear(path: &Path) {
  match std::fs::remove_file(path) {
    Ok(()) | Err(_) => (),
  }
}

/// A backend script that keeps a log of its invocations, and the path to it.
///
/// The path travels as **argv[2]**, which is how a command is parameterized
/// when nothing interposes a shell (R-36): no environment variable to set — a
/// process-wide, racy thing to do under `cargo test`'s in-process parallelism —
/// and no JSON for bash to parse. Each case names its own log, so concurrent
/// cases cannot read each other's lines — held by [`claim`], not by convention.
///
/// The log is the only evidence of a *non*-event — "the backend was not
/// spawned" — that does not come from the host's own report of itself, which is
/// PHASE-06's lesson about bounds applied to a refusal.
pub(crate) fn logging_backend(name: &str, case: &str) -> (Command, PathBuf) {
  let log = marker(&format!("invocations-{case}"));
  let mut command = backend(name);
  command.arguments.push(log.display().to_string());
  (command, log)
}

/// How many times the script has run. Absent is zero: a log with no lines and a
/// log that was never created are the same claim.
pub(crate) fn invocations(log: &Path) -> usize {
  std::fs::read_to_string(log)
    .unwrap_or_default()
    .lines()
    .count()
}

/// A backend told what to do, one instruction per invocation.
///
/// The instructions travel as argv[3…], behind the invocation log argv[2] that
/// `logging_backend` supplies — the same parameterization for the same reason
/// (R-36). An instruction is a response body, or one of the four sentinels
/// `tests/backends/answers-as-instructed.sh` documents.
///
/// One script rather than one per misbehaviour, because a `Host` is built
/// around one command and EX-2 runs the whole misbehaving suite through a
/// single `Host`: a backend that varies by invocation is the only shape that
/// admits. Giving the per-mode cases a second mechanism would then mean the
/// suite case and the individual cases were running different backends.
pub(crate) fn scripted(case: &str, instructions: &[&str]) -> (Command, PathBuf) {
  let (mut command, log) = logging_backend("answers-as-instructed", case);
  command.arguments.extend(
    instructions
      .iter()
      .map(|instruction| (*instruction).to_owned()),
  );
  (command, log)
}

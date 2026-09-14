//! §5.6, §9 item 3: the manifest allowlist. An allowlist, not a denylist — a
//! denylist of runtime, renderer and filesystem crate names requires
//! classifying an unbounded universe and lets a new dependency through by not
//! being on it. **Names only**: a permitted dependency performing I/O on
//! stratum 1's behalf is invisible here — that is `purity`'s job, and its own
//! misses are named beside it.
//!
//! Three of the workspace's five members carry no allowlist here. `goad` and
//! `goad-emit` are stratum 3, which may name both strata below it, so there is
//! nothing for an allowlist to withhold; `goad-boundary` is not a stratum at
//! all — it is the test-only member that checks the other two. This
//! instrument's subjects are exactly the two strata whose value is what they
//! *cannot* reach, which is how POL-001 §Verification scopes it: an entry "in a
//! stratum 1 or 2 manifest".
//!
//! The consequence, stated so it is not rediscovered: **a stratum-3 manifest is
//! billed by nothing here.** `crates/goad-emit`'s freedom from the renderer is
//! a fact about its own dependency table (005/AC-7), held by the crate edge and
//! by review, not by this file. Extending the instrument to stratum 3 is a
//! standing Follow-up, and would amend POL-001 rather than apply it.

use std::path::Path;

use goad_boundary::manifest::unpermitted;
use goad_boundary::scan::{Breach, report, workspace_root};

const STRATUM_1: &[&str] = &["jiff", "serde", "serde_json"];
const STRATUM_2: &[&str] = &[
  "jiff",
  "serde",
  "serde_json",
  "goad-semantics",
  "tokio",
  "toml",
];

fn is_vacuous(breaches: &[Breach]) -> bool {
  breaches
    .iter()
    .any(|breach| matches!(breach, Breach::Vacuous { .. }))
}

fn names(breaches: &[Breach], expected: &str) -> bool {
  breaches
    .iter()
    .any(|breach| matches!(breach, Breach::Token { token, .. } if token.as_ref() == expected))
}

#[test]
fn tokio_in_the_plain_dependencies_table_is_refused() {
  let text = "[dependencies]\ntokio = \"1\"\n";
  let breaches = unpermitted(Path::new("fixture.toml"), text, STRATUM_1)
    .expect_err("tokio is not on stratum 1's allowlist");
  assert!(names(&breaches, "tokio"), "{}", report(&breaches));
}

#[test]
fn tokio_in_dev_dependencies_is_refused() {
  let text = "[dev-dependencies]\ntokio = \"1\"\n";
  let breaches = unpermitted(Path::new("fixture.toml"), text, STRATUM_1)
    .expect_err("a runtime in dev-dependencies is still a runtime in the test build");
  assert!(names(&breaches, "tokio"), "{}", report(&breaches));
}

#[test]
fn tokio_in_build_dependencies_is_refused() {
  let text = "[build-dependencies]\ntokio = \"1\"\n";
  let breaches = unpermitted(Path::new("fixture.toml"), text, STRATUM_1)
    .expect_err("tokio is not on stratum 1's allowlist");
  assert!(names(&breaches, "tokio"), "{}", report(&breaches));
}

#[test]
fn tokio_under_a_target_specific_dependencies_table_is_refused() {
  let text = "[target.'cfg(unix)'.dependencies]\ntokio = \"1\"\n";
  let breaches = unpermitted(Path::new("fixture.toml"), text, STRATUM_1)
    .expect_err("a target-specific table is a dependencies table at depth (EX-4)");
  assert!(names(&breaches, "tokio"), "{}", report(&breaches));
}

#[test]
fn tokio_renamed_behind_package_is_refused() {
  let text = "[dependencies]\nclock = { package = \"tokio\", version = \"1\" }\n";
  let breaches = unpermitted(Path::new("fixture.toml"), text, STRATUM_1)
    .expect_err("a rename does not launder the crate");
  assert!(
    names(&breaches, "tokio"),
    "the key `clock` must not stand in for the `package` value:\n{}",
    report(&breaches)
  );
}

#[test]
fn a_manifest_with_no_dependency_table_is_vacuous() {
  let text = "[package]\nname = \"fixture\"\n";
  let breaches = unpermitted(Path::new("fixture.toml"), text, STRATUM_1)
    .expect_err("no dependency table at all is the same guard `Scan::run` carries");
  assert!(is_vacuous(&breaches), "{}", report(&breaches));
}

#[test]
fn the_real_stratum_1_manifest_is_clean() {
  let path = workspace_root().join("crates/goad-semantics/Cargo.toml");
  let text = std::fs::read_to_string(&path).expect("crates/goad-semantics/Cargo.toml must exist");
  if let Err(breaches) = unpermitted(&path, &text, STRATUM_1) {
    panic!("{}", report(&breaches));
  }
}

#[test]
fn the_real_stratum_2_manifest_is_clean() {
  let path = workspace_root().join("crates/goad-shell/Cargo.toml");
  let text = std::fs::read_to_string(&path).expect("crates/goad-shell/Cargo.toml must exist");
  if let Err(breaches) = unpermitted(&path, &text, STRATUM_2) {
    panic!("{}", report(&breaches));
  }
}

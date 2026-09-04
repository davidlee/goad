//! AC-11: the host does not understand the user's domain, so it may not name it
//! — over every member's `src/`.
//!
//! Also the home of the two vacuity controls and the word-matching rules, which
//! belong to `goad_boundary::scan` rather than to either configuration of it:
//! `direction.rs` has one scan and no controls of its own, and a third module
//! for two helpers would be a module per function.
//!
//! One `#[test]` over three `Scan`s rather than three `#[test]`s: `Breach`
//! carries the path, so a failure names the member either way, and PHASE-02
//! replaces the whole hand-written list with enumeration of
//! `workspace.members` (PL-12, §5.6).

use goad_boundary::scan::{Breach, Scan, mentions, report};

/// `slice-001.md:147`. Lower-case; the walk matches case-insensitively and by
/// word.
const DOMAIN: &[&str] = &[
  "habit",
  "streak",
  "journal",
  "site",
  "goal",
  "reminder",
  "compliance",
];

/// One per workspace member, by hand for exactly one phase. PHASE-02/EX-6 reads
/// `workspace.members` instead, so a new member cannot arrive unscanned.
const NO_DOMAIN_VOCABULARY: [Scan; 3] = [
  Scan {
    root: "crates/goad-semantics/src",
    forbidden: DOMAIN,
  },
  Scan {
    root: "crates/goad-shell/src",
    forbidden: DOMAIN,
  },
  Scan {
    root: "crates/goad-boundary/src",
    forbidden: DOMAIN,
  },
];

/// Fails naming *every* breach, not the first. `run` cannot return `Ok(0)`, so
/// arriving at `Ok` at all is the vacuity guard discharging.
pub(crate) fn assert_clean(scan: &Scan) {
  if let Err(breaches) = scan.run() {
    panic!("{}", report(&breaches));
  }
}

#[test]
fn no_host_source_file_names_the_user_s_domain() {
  for scan in &NO_DOMAIN_VOCABULARY {
    assert_clean(scan);
  }
}

/// A directory that exists and holds files, none of them Rust. Nothing is
/// forbidden, so the only thing that can fail this is the guard — and it fails
/// for a different reason from `RENAMED_AWAY` below, which is the whole point of
/// keeping both. `docs/adr` is at the workspace root, which is what the roots
/// here are relative to, so it resolves to a directory that is really there.
const NOTHING_TO_INSPECT: Scan = Scan {
  root: "docs/adr",
  forbidden: &[],
};

/// The threat in its literal form: `crates/goad-semantics/src/` renamed, the
/// scan left pointing at where it used to be.
const RENAMED_AWAY: Scan = Scan {
  root: "crates/goad-semantics-renamed/src",
  forbidden: &[],
};

#[test]
fn a_scan_that_inspects_no_rust_files_fails() {
  let breaches = NOTHING_TO_INSPECT
    .run()
    .expect_err("a scan inspecting nothing must fail");
  assert!(
    breaches
      .iter()
      .any(|breach| matches!(breach, Breach::Vacuous { .. })),
    "expected a vacuity breach, got:\n{}",
    report(&breaches)
  );
}

#[test]
fn a_scan_whose_directory_was_renamed_away_fails() {
  let breaches = RENAMED_AWAY
    .run()
    .expect_err("a scan over a missing root must fail");
  assert!(
    breaches
      .iter()
      .any(|breach| matches!(breach, Breach::Vacuous { .. })),
    "expected a vacuity breach, got:\n{}",
    report(&breaches)
  );
}

/// F-14: AC-11 is about names, so the match is by word — an identifier
/// segment — and not by substring. F-36: a plural or an all-caps prefix still
/// names the domain, and comment text is not code, so it is not scanned.
#[test]
fn a_token_matches_a_word_and_not_a_substring_of_one() {
  for (caught, token) in [
    ("pub struct SiteView", "site"),
    ("let site_id = 1;", "site"),
    ("mod site;", "site"),
    ("pub struct Habits", "habit"),
    ("mod habits;", "habit"),
    ("struct Sites;", "site"),
    ("pub struct HTTPSite", "site"),
    ("SITE_ID", "site"),
    ("let habit2 = 1;", "habit"),
    ("use crate::shell::host::Host;", "crate::shell"),
    ("  crate::shell::config::Command::new(x)", "crate::shell"),
  ] {
    assert!(
      mentions(caught, token),
      "{caught:?} names the domain and was missed"
    );
  }
  for (clean, token) in [
    ("// the call sites", "site"),
    ("/// the Site", "site"),
    ("//! habits", "habit"),
    ("let x = 1; // a habit", "habit"),
    ("let websites = 1;", "site"),
    ("let offsite = 1;", "site"),
    ("let habitat = 1;", "habit"),
    ("// see crate::shell for the composition", "crate::shell"),
  ] {
    assert!(
      !mentions(clean, token),
      "{clean:?} does not name the domain and was caught"
    );
  }
}

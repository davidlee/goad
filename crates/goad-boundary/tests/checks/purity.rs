//! AC-3: the stratum 1 purity scan. ADR-001 asks for "no I/O and no async
//! runtime", and neither Cargo resolution nor the manifest allowlist can see a
//! direct `std` reach — it needs no manifest entry and is not a crate edge
//! (D25, `design.md` §5.6). This is **one more configured `Scan`**, and it is
//! a regression tripwire, not a proof: `use std::{fs, process};` names neither
//! token and passes; a later alias introduced any other way is not caught;
//! and I/O performed on stratum 1's behalf by a permitted dependency is
//! invisible to it — that is the allowlist's job.

use std::collections::BTreeSet;
use std::path::PathBuf;

use goad_boundary::scan::{Breach, Scan, report};

/// `std::time::Duration` is deliberately absent: a duration is a quantity,
/// not a clock, and forbidding it would refuse a pure value (D25).
const FORBIDDEN: &[&str] = &[
  "std::fs",
  "std::process",
  "std::net",
  "std::os",
  "std::env",
  "std::thread",
  "std::io",
  "std::time::SystemTime",
  "std::time::Instant",
];

fn fixture(scenario: &str) -> Scan {
  Scan {
    root: PathBuf::from("crates/goad-boundary/tests/fixtures/purity").join(scenario),
    extensions: &["rs"],
    excluded_dirs: &["tests", "target"],
    forbidden: FORBIDDEN,
  }
}

/// Positive: each forbidden token, planted in a stratum-1-shaped source.
#[test]
fn each_forbidden_token_planted_in_a_stratum_1_source_is_caught() {
  let breaches = fixture("planted")
    .run()
    .expect_err("nine planted tokens must all be caught");
  let caught: BTreeSet<&str> = breaches
    .iter()
    .filter_map(|breach| match breach {
      Breach::Token { token, .. } => Some(token.as_ref()),
      Breach::Vacuous { .. } | Breach::Unreadable { .. } | Breach::GlobMember { .. } => None,
    })
    .collect();
  for token in FORBIDDEN {
    assert!(
      caught.contains(token),
      "{token} was not caught:\n{}",
      report(&breaches)
    );
  }
}

/// The cut still applies here, and the control above is therefore not
/// vacuous: a token named only inside a comment is documentation, not a
/// reach, and must not be flagged.
#[test]
fn a_forbidden_token_named_only_inside_a_comment_is_not_caught() {
  fixture("commented")
    .run()
    .expect("a token only inside a comment must not be flagged");
}

/// The real thing, clean.
#[test]
fn the_real_stratum_1_source_names_none_of_the_nine() {
  let scan = Scan {
    root: PathBuf::from("crates/goad-semantics/src"),
    extensions: &["rs"],
    excluded_dirs: &["tests", "target"],
    forbidden: FORBIDDEN,
  };
  if let Err(breaches) = scan.run() {
    panic!("{}", report(&breaches));
  }
}

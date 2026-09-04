//! AC-15's direction half. The dependency-graph half is the build gate — this
//! catches the `use crate::shell::…` a crate edge cannot see because it is not
//! a crate edge.

use goad_boundary::scan::Scan;

use crate::vocabulary::assert_clean;

/// Stratum 1 looks only down. The three tokens are retired at PHASE-02, in the
/// same change as the manifest allowlist that replaces them (§5.6, PL-2); until
/// then this is the only instrument holding "no runtime named in stratum 1".
const STRATUM_1_LOOKS_ONLY_DOWN: Scan = Scan {
  root: "crates/goad-semantics/src",
  forbidden: &["crate::shell", "crate::bin", "tokio"],
};

#[test]
fn stratum_1_names_neither_the_shell_a_binary_nor_the_runtime() {
  assert_clean(&STRATUM_1_LOOKS_ONLY_DOWN);
}

//! The `--version` line's text, which every binary shares.
//!
//! What a binary prints when asked its version is one rule, not one per
//! binary: each reads its own build's version and revision and hands both
//! here. Writing the line is `crate::report`'s; this module only composes it.

/// The `--version` answer: the package version, and the revision the build
/// stamped — when it stamped one.
///
/// **No placeholder for the unstamped case.** `cargo install` stamps nothing
/// and a tarball consumer of the flake stamps nothing either; both say
/// `0.1.0` and stop there. A `(revision unknown)` would be a binary reporting
/// a fact it does not have.
///
/// **No prefix**, unlike a binary's stderr lines (`goad: `, `goad-emit: `):
/// the prefix is there because stderr must say who spoke, and a direct answer
/// to a direct question on stdout need not.
///
/// **Set-but-empty is unset**, the rule `crates/goad`'s `build.rs` states for
/// `SLINT_STYLE`: a flake consumed as a tarball has no revision to stamp and
/// stamps `""`, which must read as unstamped and not as a revision whose name
/// is empty. The test is **here**, inside the one function that decides the
/// line, and not at the call site: a rule spelled at each caller is a rule
/// the unit tier cannot reach and two transcriptions that can disagree
/// (`review-code.md` F-2).
///
/// **Both are parameters**, so this stays pure and every branch is a test
/// rather than a build configuration. The revision is the build's, read by
/// each binary with `option_env!("GOAD_REVISION")`. The version is too:
/// `env!("CARGO_PKG_VERSION")` expands in the crate that compiles it, so read
/// here it would name this crate's version, not the binary's. Each binary
/// passes its own.
#[must_use]
pub fn version_line(version: &str, revision: Option<&str>) -> String {
  match revision.filter(|revision| !revision.is_empty()) {
    Some(revision) => format!("{version} ({revision})"),
    None => version.to_owned(),
  }
}

#[cfg(test)]
mod tests {
  use super::version_line;

  /// 006/PHASE-03/VT-1, AC-5's rendering half. The **stamped** branch is
  /// unreachable anywhere else under `cargo test`: nothing in the gate sets
  /// `GOAD_REVISION`, so each binary's own `--version` case can only ever see
  /// the bare form. This case is the real assertion for it, not a proxy for
  /// one (`docs/memory/tests-asserting-proxies.md`).
  #[test]
  fn a_stamped_build_names_its_revision_beside_the_version() {
    assert_eq!(version_line("0.1.0", Some("08528b5")), "0.1.0 (08528b5)");
  }

  /// No placeholder. A build that stamped no revision says only what is
  /// known — the `cargo install` path is exactly this branch, and
  /// `(revision unknown)` would be a binary reporting a fact it does not have
  /// (design.md §4, principle 2).
  #[test]
  fn an_unstamped_build_says_only_the_version() {
    assert_eq!(version_line("0.1.0", None), "0.1.0");
  }

  /// **Set-but-empty is unset** (`review-code.md` F-2). `flake.nix` stamps
  /// `self.shortRev or self.dirtyShortRev or ""`, and the third branch is
  /// what a tarball consumer gets: `GOAD_REVISION=""` reaches `option_env!`
  /// as `Some("")`, which must read as unstamped. Without the filter the line
  /// is `0.1.0 ()`.
  #[test]
  fn a_build_stamped_with_an_empty_revision_is_an_unstamped_build() {
    assert_eq!(version_line("0.1.0", Some("")), "0.1.0");
  }
}

//! AC-13, AC-14: the host does not understand the user's domain, so it may not
//! name it (`CLAUDE.md` invariant 1) — over every member `workspace.members`
//! names, `.slint` as well as `.rs`, excluding `tests/` and `target/` (D13,
//! D17). One scan template, enumerated rather than hand-listed (§5.6): a new
//! member cannot arrive unscanned. Also the home of the two vacuity controls
//! and the word-matching rules, which belong to `goad_boundary::scan` rather
//! than to `allowlist` or `purity`, neither of which has a control of its
//! own.

use std::borrow::Cow;
use std::path::PathBuf;

use goad_boundary::members::{crate_name, members};
use goad_boundary::scan::{Breach, Scan, code_of, mentions, report, workspace_root};

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

fn domain_scan(root: PathBuf) -> Scan {
  Scan {
    root,
    extensions: &["rs", "slint"],
    excluded_dirs: &["tests", "target"],
    forbidden: DOMAIN,
  }
}

/// Fails naming *every* breach, not the first. `run` cannot return `Ok(0)`, so
/// arriving at `Ok` at all is the vacuity guard discharging.
pub(crate) fn assert_clean(scan: &Scan) {
  if let Err(breaches) = scan.run() {
    panic!("{}", report(&breaches));
  }
}

/// EX-6: no hand-written per-member list. `workspace.members` is read for
/// itself, so a new member arrives already covered.
#[test]
fn no_workspace_member_names_the_users_domain() {
  let root_manifest = workspace_root().join("Cargo.toml");
  let member_dirs = members(&root_manifest).expect("the real workspace root has real members");
  assert!(
    !member_dirs.is_empty(),
    "a workspace with no members is the vacuity guard one level up"
  );
  for member in member_dirs {
    assert_clean(&domain_scan(member));
  }
}

/// F-3 (review-code 002, round 1): AC-14 names "crate name" as a covered
/// surface, and a crate name lives in `Cargo.toml`, which `domain_scan`
/// above never opens (`extensions: &["rs", "slint"]`). A member declared
/// and depended on by manifest alone would otherwise carry no source-side
/// obligation to repeat its own name.
#[test]
fn no_member_manifest_names_the_users_domain_in_its_own_crate_name() {
  let root_manifest = workspace_root().join("Cargo.toml");
  let member_dirs = members(&root_manifest).expect("the real workspace root has real members");
  for member in member_dirs {
    let manifest = workspace_root().join(&member).join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest)
      .unwrap_or_else(|error| panic!("{}: {error}", manifest.display()));
    let name = crate_name(&manifest, &text).unwrap_or_else(|breach| panic!("{breach}"));
    for token in DOMAIN {
      assert!(
        !mentions(&name, token),
        "{name:?} ({}) names the domain word {token:?}",
        manifest.display()
      );
    }
  }
}

/// Positive control for the check above: a fixture manifest whose
/// `[package].name` carries a forbidden word is caught the same way a real
/// member's would be — never a file on disk, since `crate_name` takes text,
/// not a path to read.
#[test]
fn a_forbidden_word_in_a_crate_name_is_caught() {
  let text = "[package]\nname = \"goad-habits\"\n";
  let name = crate_name(PathBuf::from("fixture/Cargo.toml").as_path(), text)
    .expect("the fixture manifest has a `[package].name`");
  assert!(
    mentions(&name, "habit"),
    "{name:?} must be caught as naming \"habit\""
  );
}

/// A directory that exists and holds files, none of them `.rs` or `.slint`.
/// Nothing is forbidden, so the only thing that can fail this is the guard —
/// and it fails for a different reason from `RENAMED_AWAY` below, which is
/// the whole point of keeping both. `docs/adr` is at the workspace root,
/// which is what the roots here are relative to, so it resolves to a
/// directory that is really there.
#[test]
fn a_member_directory_with_no_rust_or_slint_file_fails_naming_itself() {
  let scan = Scan {
    root: PathBuf::from("docs/adr"),
    extensions: &["rs", "slint"],
    excluded_dirs: &["tests", "target"],
    forbidden: &[],
  };
  let breaches = scan.run().expect_err("a scan inspecting nothing must fail");
  assert!(
    breaches
      .iter()
      .any(|breach| matches!(breach, Breach::Vacuous { .. })),
    "expected a vacuity breach, got:\n{}",
    report(&breaches)
  );
}

/// The threat in its literal form: `crates/goad-semantics/src/` renamed, the
/// scan left pointing at where it used to be.
#[test]
fn a_scan_whose_directory_was_renamed_away_fails() {
  let scan = Scan {
    root: PathBuf::from("crates/goad-semantics-renamed/src"),
    extensions: &["rs", "slint"],
    excluded_dirs: &["tests", "target"],
    forbidden: &[],
  };
  let breaches = scan
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

/// A glob in `workspace.members` hides from a reader exactly what this check
/// exists to make visible, so `members` fails on it rather than expanding it
/// (D17, PS-3).
#[test]
fn a_glob_in_workspace_members_fails() {
  let fixture =
    workspace_root().join("crates/goad-boundary/tests/fixtures/manifests/glob-members.toml");
  let breaches = members(&fixture).expect_err("a glob member entry must be refused");
  assert!(
    breaches
      .iter()
      .any(|breach| matches!(breach, Breach::GlobMember { .. })),
    "expected a `GlobMember` breach, got:\n{}",
    report(&breaches)
  );
}

/// Positive, `.slint`: a forbidden word in a component name, and in an
/// `accessible-label` — the new half of D13, proving `.slint` files are
/// actually reached by the extension filter rather than merely parseable as
/// text.
#[test]
fn a_forbidden_word_in_slint_markup_is_caught() {
  let scan = Scan {
    root: PathBuf::from("crates/goad-boundary/tests/fixtures/vocabulary"),
    extensions: &["slint"],
    excluded_dirs: &["tests", "target"],
    forbidden: DOMAIN,
  };
  let breaches = scan
    .run()
    .expect_err("a component name and an accessible-label both name the domain");
  assert!(
    breaches
      .iter()
      .any(|breach| matches!(breach, Breach::Token { token, .. } if token.as_ref() == "site")),
    "the component name was not caught:\n{}",
    report(&breaches)
  );
  assert!(
    breaches
      .iter()
      .any(|breach| matches!(breach, Breach::Token { token, .. } if token.as_ref() == "habit")),
    "the accessible-label was not caught:\n{}",
    report(&breaches)
  );
}

/// F-14: AC-13 is about names, so the match is by word — an identifier
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
  ] {
    assert!(
      !mentions(clean, token),
      "{clean:?} does not name the domain and was caught"
    );
  }
}

/// D13's string-aware comment cut, over `mentions` rather than `code_of`
/// directly: the property that matters is which tokens survive, not the
/// intermediate string.
#[test]
fn a_string_hides_no_token_that_follows_it_on_the_same_line() {
  for (line, token) in [
    // An ordinary string.
    (r#"let t = "habit";"#, "habit"),
    // A URL inside a string, on the same line as a later string: the old
    // cut (truncate at the first `//`) hid everything after it.
    (
      r#"let url = "https://example.com"; let t = "site";"#,
      "site",
    ),
    // An escaped quote does not close the string early.
    (r#""a\"b"; let t = "habit";"#, "habit"),
    // A raw string's own `//` does not start a comment, and its closing
    // `"#` does not leak into the code that follows.
    (r##"r#"https://x"#; let t = "site";"##, "site"),
  ] {
    assert!(mentions(line, token), "{line:?} should have caught {token}");
  }
}

/// D13's costs, demonstrated rather than only named: a same-line block
/// comment is cut, and a lifetime is never mistaken for a string delimiter.
#[test]
fn a_same_line_block_comment_is_cut_and_a_lifetime_opens_no_string() {
  assert!(!mentions("/* habit */ let x = 1;", "habit"));
  assert!(!mentions("let x: &'static str = \"ok\"; // habit", "habit"));
}

/// F-4 (review-code 002, round 1): a char literal holding a quote used to
/// desynchronise the cut — the *next* real string's opening quote was read
/// as this one's spurious close, re-entering `Code` inside that string, so
/// a `//` there truncated the line early and hid what followed. Each fixture
/// is the reviewer's own evidence, restated as a control: `mentions` must
/// still reach the token past the char literal, on both sides of it.
#[test]
fn a_char_literal_holding_a_quote_does_not_desynchronise_the_cut() {
  for (line, token) in [
    (r#"let q = '"'; let s = "//"; let habit = 1;"#, "habit"),
    (r#"if b == b'"' { let s = "//"; let site = 1; }"#, "site"),
    (r#"let q = '\''; let s = "//"; let habit = 1;"#, "habit"),
  ] {
    assert!(
      mentions(line, token),
      "{line:?} should have caught {token} past the char literal"
    );
  }
  // The false-positive direction the same defect opened: a real `//`
  // comment, past a char literal, must still be cut.
  assert!(!mentions(r#"let q = '"'; // the call sites"#, "site"));
}

/// The reason `code_of` returns `Cow` rather than `&str`: an interior block
/// comment is replaced by **one space**, never by nothing. Without the space,
/// `site` and `view` would merge into the single, unsplittable word
/// `siteview` — a false negative in the direction that matters, since
/// neither half has a case boundary for `camel_segments` to split on (D13).
#[test]
fn an_interior_block_comment_is_replaced_by_a_space_not_nothing() {
  assert!(mentions("let x = site/*x*/view;", "site"));
}

/// VT-4: `code_of`'s own return-type contract, not just what it feeds
/// `mentions`. `Borrowed` when nothing was cut or the cut ran to end of line
/// — the common case, and every line in the tree today; `Owned` with exactly
/// one inserted space only when an interior `/* … */` closed mid-line.
#[test]
fn code_of_borrows_unless_an_interior_block_closed_mid_line() {
  assert!(matches!(code_of("let x = 1;"), Cow::Borrowed("let x = 1;")));
  assert!(matches!(
    code_of("let x = 1; // trailing"),
    Cow::Borrowed("let x = 1; ")
  ));
  assert!(matches!(
    code_of("let x = 1; /* unterminated"),
    Cow::Borrowed("let x = 1; ")
  ));
  match code_of("let x = site/*x*/view;") {
    Cow::Owned(line) => assert_eq!(line, "let x = site view;"),
    Cow::Borrowed(line) => {
      panic!("an interior block that closed mid-line must be owned, got {line:?}")
    }
  }
}

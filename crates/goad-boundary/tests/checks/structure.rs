//! Item 14f (design.md §9): a source scan asserting `quit_event_loop` has
//! exactly one call site in `crates/goad/src/`, and that the renderer holds
//! no `tokio::spawn` handle — mirroring slice 001's
//! `the_only_spawn_is_the_child` (`crates/goad-shell/tests/shape/
//! transport_shape.rs`). Not `crate::scan::Scan`: that walk's contract is
//! presence-forbidding over a directory, and "exactly one" is not
//! expressible in it (`plan-log.md` PL-6).
//!
//! Scoped to **production** code: a source is read only up to its own
//! `#[cfg(test)]` line, if it has one, which is where every inline test
//! module in this crate starts and where it stays until the file ends
//! (measured — `wire.rs` and `controller.rs` are the two files with one, and
//! `wire.rs`'s own test module legitimately drives `Cancel::stopped()` with
//! `tokio::spawn`, which is not the handle item 14f is about).

use std::path::{Path, PathBuf};

use goad_boundary::scan::{code_of, mentions, workspace_root};

/// Stratum 3, AC-6 instrument (a)'s subject: no production line here may
/// name the identifier `resolve`.
const SUBJECT_DIR: &str = "crates/goad/src";
/// Stratum 2, AC-6 instrument (b)'s subject: `schedule::resolve` may occur
/// only in `Host`.
const SHELL_SUBJECT_DIR: &str = "crates/goad-shell/src";

/// One line of **production** code that named the needle: read up to (and
/// excluding) the file's own `#[cfg(test)]` line, comments and string
/// contents already stripped by `code_of`.
struct Occurrence {
  path: PathBuf,
  line: usize,
}

/// Every `.rs` file under `dir`, at any depth, sorted so a failure reads
/// the same way twice. Recursive (F-10, review-code 002 round 1): the
/// directory being flat today is not a build-checked fact, only a fact a
/// human happened to measure, and `ADR-001`'s Verification section is
/// explicit about not trusting that arrangement — a second
/// `quit_event_loop` under a subdirectory must not leave every assertion
/// below passing on a count of one it never saw.
///
/// Directory-parameterised (PHASE-04/FD-2): AC-6's two instruments scan two
/// different subject directories, so the walk can no longer close over one
/// module constant.
fn subject_files(dir: &str) -> Vec<PathBuf> {
  let root = workspace_root().join(dir);
  let mut files = Vec::new();
  walk_rs_files(&root, &mut files);
  files.sort();
  files
}

/// The recursion `crate::scan::Scan::walk` demonstrates, restated here
/// rather than reused: that walk is private, and coupled to `Scan`'s own
/// token-breach collection — this one only lists paths.
fn walk_rs_files(dir: &Path, files: &mut Vec<PathBuf>) {
  for entry in std::fs::read_dir(dir).unwrap_or_else(|error| panic!("{}: {error}", dir.display())) {
    let path = entry
      .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
      .path();
    if path.is_dir() {
      walk_rs_files(&path, files);
    } else if path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs") {
      files.push(path);
    }
  }
}

fn production_lines(path: &Path) -> Vec<(usize, String)> {
  let text =
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
  let mut lines = Vec::new();
  for (index, line) in text.lines().enumerate() {
    if line.trim() == "#[cfg(test)]" {
      break;
    }
    lines.push((index + 1, code_of(line).into_owned()));
  }
  lines
}

/// The substring matcher — for the three existing needles, none of which
/// is a bare identifier (`quit_event_loop(`, `tokio::spawn`,
/// `slint::spawn_local(` all carry punctuation `str::contains` is exactly
/// right for).
fn occurrences_of(dir: &str, needle: &str) -> Vec<Occurrence> {
  let mut found = Vec::new();
  for path in subject_files(dir) {
    for (line, code) in production_lines(&path) {
      if code.contains(needle) {
        found.push(Occurrence {
          path: path.clone(),
          line,
        });
      }
    }
  }
  found
}

/// The identifier/path matcher — `goad_boundary::scan::mentions`, for
/// AC-6's two instruments. A token containing `::` is matched as a
/// substring (instrument (b), `"schedule::resolve"`); a bare token is
/// matched as an identifier word, catching a brace-grouped `use` as
/// readily as a call (instrument (a), `"resolve"`) — `mentions` already
/// branches on this, so one function serves both (FD-2: "two matchers,
/// named"; the other is `occurrences_of` above). `mentions` re-applies
/// `code_of` to lines `production_lines` has already stripped — harmless
/// on already-stripped text, not a second pass with different results.
fn mentions_occurrences_of(dir: &str, token: &str) -> Vec<Occurrence> {
  let mut found = Vec::new();
  for path in subject_files(dir) {
    for (line, code) in production_lines(&path) {
      if mentions(&code, token) {
        found.push(Occurrence {
          path: path.clone(),
          line,
        });
      }
    }
  }
  found
}

fn report(found: &[Occurrence]) -> String {
  found
    .iter()
    .map(|occurrence| format!("{}:{}", occurrence.path.display(), occurrence.line))
    .collect::<Vec<_>>()
    .join("\n")
}

/// F-10 (review-code 002, round 1), the presence half E-1 pairs with the
/// vacuity guard below: a `.rs` file one directory down is found, not just
/// one directly under the root — proving the walk actually descends rather
/// than merely being written to.
#[test]
fn the_walk_descends_into_a_subdirectory() {
  let root = workspace_root().join("crates/goad-boundary/tests/fixtures/structure");
  let mut files = Vec::new();
  walk_rs_files(&root, &mut files);
  assert!(
    files.iter().any(|path| path.ends_with("nested/marker.rs")),
    "found:\n{}",
    files
      .iter()
      .map(|path| path.display().to_string())
      .collect::<Vec<_>>()
      .join("\n")
  );
}

/// The guard: a scan of a renamed-away directory finds nothing and would
/// otherwise pass every assertion below vacuously (E-1). Runs for both
/// subject directories (PHASE-04/EX-3): a second directory added for AC-6
/// must not be exempt from the same guard the first one has always had.
#[test]
fn the_subject_directories_are_found_and_are_not_empty() {
  for dir in [SUBJECT_DIR, SHELL_SUBJECT_DIR] {
    assert!(
      !subject_files(dir).is_empty(),
      "{dir} inspected no `.rs` file — renamed, emptied, or misspelled"
    );
  }
}

/// EX-5, item 14f, first half: exactly one call site, with no exception
/// carved out for `Wire::send`'s `Closed` arm — because that arm no longer
/// calls it (F-20).
#[test]
fn quit_event_loop_has_exactly_one_call_site() {
  let found = occurrences_of(SUBJECT_DIR, "quit_event_loop(");
  assert_eq!(found.len(), 1, "found:\n{}", report(&found));
}

/// Item 14f, second half: the token is `tokio::spawn`, not `spawn` —
/// `slint::spawn_local`, the one spawn this crate is built around, is a
/// different call entirely and must stay through (mirrors
/// `the_only_spawn_is_the_child`'s reasoning for naming `spawn` rather than
/// `command.spawn()` there, the other way round).
#[test]
fn the_renderer_holds_no_tokio_spawn_handle() {
  let found = occurrences_of(SUBJECT_DIR, "tokio::spawn");
  assert!(found.is_empty(), "found:\n{}", report(&found));
}

/// The negative above is not vacuous merely because no `tokio::spawn`
/// exists: the crate does spawn its one task, just not that way.
#[test]
fn slint_spawn_local_is_the_one_spawn_this_crate_uses() {
  let found = occurrences_of(SUBJECT_DIR, "slint::spawn_local(");
  assert_eq!(found.len(), 1, "found:\n{}", report(&found));
}

/// AC-6 (a), item 8 (design.md §5.5 I-1a): the timer never resolves a
/// schedule itself, and this holds regardless of import shape — a
/// brace-grouped `use goad_semantics::schedule::{resolve, wait_for};`
/// would defeat a path grep (F-3) but not this identifier-word match. The
/// file count is asserted first so the absence below cannot pass on a walk
/// that inspected nothing.
#[test]
fn no_production_line_in_the_renderer_names_the_identifier_resolve() {
  assert!(
    !subject_files(SUBJECT_DIR).is_empty(),
    "{SUBJECT_DIR} inspected no `.rs` file — renamed, emptied, or misspelled"
  );
  let found = mentions_occurrences_of(SUBJECT_DIR, "resolve");
  assert!(found.is_empty(), "found:\n{}", report(&found));
}

/// AC-6 (b): the path `schedule::resolve` occurs exactly twice in
/// `crates/goad-shell/src`'s production code, both in `host.rs` — measured
/// at `host.rs:128` and `:259` (design.md §9's AC-6 row). Asserted as a
/// count and a set of file names, never as line numbers (D-16): a line
/// moving inside `host.rs` is not this instrument's business.
#[test]
fn schedule_resolve_is_called_only_from_host() {
  assert!(
    !subject_files(SHELL_SUBJECT_DIR).is_empty(),
    "{SHELL_SUBJECT_DIR} inspected no `.rs` file — renamed, emptied, or misspelled"
  );
  let found = mentions_occurrences_of(SHELL_SUBJECT_DIR, "schedule::resolve");
  assert_eq!(found.len(), 2, "found:\n{}", report(&found));
  let files: std::collections::BTreeSet<Option<&str>> = found
    .iter()
    .map(|occurrence| {
      occurrence
        .path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
    })
    .collect();
  assert_eq!(
    files,
    std::collections::BTreeSet::from([Some("host.rs")]),
    "found:\n{}",
    report(&found)
  );
}

/// Controls on this file's own counting and its test-scope cut — `code_of`
/// itself is already proven against real fixtures by `purity.rs` and
/// `vocabulary.rs`.
mod counting_itself {
  use super::{code_of, mentions, production_lines, workspace_root};

  #[test]
  fn a_token_named_only_in_a_comment_is_not_counted() {
    let line = "  // the crate's only quit_event_loop() call site";
    assert!(!code_of(line).contains("quit_event_loop("));
  }

  #[test]
  fn a_real_call_site_is_counted() {
    let line = "    match slint::quit_event_loop() {";
    assert!(code_of(line).contains("quit_event_loop("));
  }

  /// AC-6 (a)'s word matcher, controls (PHASE-04/VT-5). `resolve` named
  /// only in a comment is not counted — the same cut `code_of` already
  /// gives the substring matcher above, proven again for `mentions`.
  #[test]
  fn resolve_named_only_in_a_comment_is_not_counted_by_the_word_matcher() {
    let line = "  // see schedule::resolve for how this is seeded";
    assert!(!mentions(line, "resolve"));
  }

  /// A brace-grouped `use` is exactly the shape I-1a warns a path grep
  /// misses (F-3): the identifier is named, so the word matcher catches
  /// it even though no line contains the substring `schedule::resolve`.
  #[test]
  fn a_brace_grouped_use_naming_resolve_is_counted_by_the_word_matcher() {
    let line = "use goad_semantics::schedule::{resolve, wait_for};";
    assert!(mentions(line, "resolve"));
  }

  /// `resolved` is an ordinary English participle, not the identifier
  /// `resolve` or its plural `resolves` — `mentions`'s own contract
  /// (`scan.rs`'s doc comment: "matches the token or its plural"). Decided
  /// and asserted, per EX-2/VT-5, rather than left for a reader to work
  /// out from the matcher's source.
  #[test]
  fn the_participle_resolved_is_not_counted_by_the_word_matcher() {
    let line = "// the outcome resolved cleanly on the first try";
    assert!(!mentions(line, "resolve"));
  }

  /// `wire.rs` is the fixture already on disk that proves the cut is not
  /// vacuous: it names `tokio::spawn` **after** its own `#[cfg(test)]` line,
  /// so a version of this check with no cut would fail today, and the cut's
  /// absence would be caught the moment this test is run against it.
  #[test]
  fn wire_rs_names_tokio_spawn_only_after_its_cfg_test_line() {
    let path = workspace_root().join("crates/goad/src/wire.rs");
    let whole = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{error}"));
    assert!(
      whole.contains("tokio::spawn"),
      "the fixture this control rests on no longer names tokio::spawn at all"
    );
    let cut = production_lines(&path);
    assert!(
      cut.iter().all(|(_, code)| !code.contains("tokio::spawn")),
      "the cut let a post-#[cfg(test)] line through"
    );
  }
}

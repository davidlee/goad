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

use goad_boundary::scan::{code_of, workspace_root};

const SUBJECT_DIR: &str = "crates/goad/src";

/// One line of **production** code that named the needle: read up to (and
/// excluding) the file's own `#[cfg(test)]` line, comments and string
/// contents already stripped by `code_of`.
struct Occurrence {
  path: PathBuf,
  line: usize,
}

/// Every `.rs` file directly under `SUBJECT_DIR`, sorted so a failure reads
/// the same way twice. Non-recursive — the directory is flat today
/// (`find crates/goad/src -type f`, measured) — so a subdirectory arriving
/// unscanned is a defect the next `git ls-files` diff surfaces, not a silent
/// gap.
fn subject_files() -> Vec<PathBuf> {
  let dir = workspace_root().join(SUBJECT_DIR);
  let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
    .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
    .filter_map(Result::ok)
    .map(|entry| entry.path())
    .filter(|path| path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs"))
    .collect();
  files.sort();
  files
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

fn occurrences_of(needle: &str) -> Vec<Occurrence> {
  let mut found = Vec::new();
  for path in subject_files() {
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

fn report(found: &[Occurrence]) -> String {
  found
    .iter()
    .map(|occurrence| format!("{}:{}", occurrence.path.display(), occurrence.line))
    .collect::<Vec<_>>()
    .join("\n")
}

/// The guard: a scan of a renamed-away directory finds nothing and would
/// otherwise pass every assertion below vacuously (E-1).
#[test]
fn the_subject_directory_is_found_and_is_not_empty() {
  assert!(
    !subject_files().is_empty(),
    "{SUBJECT_DIR} inspected no `.rs` file — renamed, emptied, or misspelled"
  );
}

/// EX-5, item 14f, first half: exactly one call site, with no exception
/// carved out for `Wire::send`'s `Closed` arm — because that arm no longer
/// calls it (F-20).
#[test]
fn quit_event_loop_has_exactly_one_call_site() {
  let found = occurrences_of("quit_event_loop(");
  assert_eq!(found.len(), 1, "found:\n{}", report(&found));
}

/// Item 14f, second half: the token is `tokio::spawn`, not `spawn` —
/// `slint::spawn_local`, the one spawn this crate is built around, is a
/// different call entirely and must stay through (mirrors
/// `the_only_spawn_is_the_child`'s reasoning for naming `spawn` rather than
/// `command.spawn()` there, the other way round).
#[test]
fn the_renderer_holds_no_tokio_spawn_handle() {
  let found = occurrences_of("tokio::spawn");
  assert!(found.is_empty(), "found:\n{}", report(&found));
}

/// The negative above is not vacuous merely because no `tokio::spawn`
/// exists: the crate does spawn its one task, just not that way.
#[test]
fn slint_spawn_local_is_the_one_spawn_this_crate_uses() {
  let found = occurrences_of("slint::spawn_local(");
  assert_eq!(found.len(), 1, "found:\n{}", report(&found));
}

/// Controls on this file's own counting and its test-scope cut — `code_of`
/// itself is already proven against real fixtures by `purity.rs` and
/// `vocabulary.rs`.
mod counting_itself {
  use super::{code_of, production_lines, workspace_root};

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

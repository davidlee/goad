//! Item 14f (design.md §9): a source scan asserting `quit_event_loop` has
//! exactly one call site in `crates/goad/src/`, and that the renderer holds
//! no `tokio::spawn` handle — mirroring slice 001's
//! `the_only_spawn_is_the_child` (`crates/goad-shell/tests/shape/
//! transport_shape.rs`). Not `crate::scan::Scan`: that walk's contract is
//! presence-forbidding over a directory, and "exactly one" is not
//! expressible in it (`plan-log.md` PL-6).
//!
//! Scoped to **production** code: each `#[cfg(test)]` **item** is skipped and
//! the scan resumes after it, rather than the file being cut off at the first
//! such line. Every inline test module in
//! `crates/goad/src` runs to the end of its file today, so the two read the
//! same tree; `crates/goad-shell/src` has three files whose test module starts
//! before their midpoint, and nothing but this scan's own shape stops
//! production code from following one. `wire.rs`'s own test module
//! legitimately drives `Cancel::stopped()` with `tokio::spawn`, which is not
//! the handle item 14f is about.

use std::path::{Path, PathBuf};

use goad_boundary::scan::{code_of, code_without_literals, mentions, workspace_root};

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

/// Every line of **production** code in `path`, comments and string contents
/// already handled by `code_of`, with each `#[cfg(test)]` item skipped.
///
/// The skip is over the attributed **item**, not over the rest of the file:
/// a cut that broke at the first
/// `#[cfg(test)]` line read nothing after it, so a file with production code
/// following an inline test module would go half-unread while still counting
/// as one inspected file. No subject file has that shape today; nothing made
/// that a checked fact, and `crates/goad-shell/src` has three files whose test
/// module starts before their midpoint.
///
/// A braced item (`mod tests { … }`) is skipped until its brace depth returns
/// to zero; an unbraced one (`#[cfg(test)] use …;`) ends on its own line.
/// Attribute and doc lines between the `#[cfg(test)]` and the item it attaches
/// to are skipped without ending the skip.
///
/// The depth is counted over `code_without_literals`, not `code_of`: a brace
/// inside a string or char literal is not a brace, and one of them would
/// desynchronise the count for every line after it — the same partial
/// blinding, by another route, that the item-scoped skip exists to prevent.
fn production_lines(path: &Path) -> Vec<(usize, String)> {
  let text =
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
  let mut lines = Vec::new();
  let mut skipping: Option<Skip> = None;
  for (index, line) in text.lines().enumerate() {
    match skipping.as_mut() {
      // Braces are counted over text with **literals cut as well as
      // comments**: `code_of` leaves string and char contents intact, on
      // purpose, and one unbalanced `{` inside a literal in a test module
      // would otherwise hold the skip open for the rest of the file.
      Some(skip) => {
        if skip.consume(&code_without_literals(line)) {
          skipping = None;
        }
      }
      None if line.trim() == "#[cfg(test)]" => skipping = Some(Skip::default()),
      None => lines.push((index + 1, code_of(line).into_owned())),
    }
  }
  lines
}

/// The state of a `#[cfg(test)]` item being skipped: how deep inside its
/// braces the scan is, and whether it has met a brace at all yet.
#[derive(Default)]
struct Skip {
  depth: usize,
  opened: bool,
}

impl Skip {
  /// Feed one line of stripped code. Returns whether the item ended on it.
  fn consume(&mut self, code: &str) -> bool {
    let opens = code.matches('{').count();
    let closes = code.matches('}').count();
    if opens > 0 {
      self.opened = true;
    }
    self.depth = self.depth.saturating_add(opens).saturating_sub(closes);
    if self.opened {
      return self.depth == 0;
    }
    // Not braced yet: a line ending in `;` is the whole item; anything else
    // is a further attribute or doc line on the way to it.
    code.trim_end().ends_with(';')
  }
}

/// Every production line in `dir` whose code satisfies `names_it`, as one
/// walk. The three matchers below differ only in the predicate they hand it.
fn occurrences_where(dir: &str, mut names_it: impl FnMut(&str) -> bool) -> Vec<Occurrence> {
  let mut found = Vec::new();
  for path in subject_files(dir) {
    for (line, code) in production_lines(&path) {
      if names_it(&code) {
        found.push(Occurrence {
          path: path.clone(),
          line,
        });
      }
    }
  }
  found
}

/// The substring matcher — for the three existing needles, none of which
/// is a bare identifier (`quit_event_loop(`, `tokio::spawn`,
/// `slint::spawn_local(` all carry punctuation `str::contains` is exactly
/// right for).
fn occurrences_of(dir: &str, needle: &str) -> Vec<Occurrence> {
  occurrences_where(dir, |code| code.contains(needle))
}

/// The identifier matcher — `goad_boundary::scan::mentions`, for AC-6's
/// instrument (a). A bare token is matched as an identifier **word**,
/// catching a brace-grouped `use` as readily as a call; `mentions`'s other
/// branch, a token containing `::` matched as a plain substring, is defeated
/// by exactly that `use` and is not what either AC-6 instrument takes.
/// `mentions` re-applies `code_of` to lines `production_lines` has already
/// stripped — harmless on already-stripped text, not a second pass with
/// different results.
fn mentions_occurrences_of(dir: &str, token: &str) -> Vec<Occurrence> {
  occurrences_where(dir, |code| mentions(code, token))
}

/// The call matcher — AC-6's instrument (b). Does this line **call**
/// `resolve`?
///
/// `resolve(` with no identifier byte before it, over
/// `code_without_literals`, so neither `my_resolve(` nor a `resolve(` inside
/// a message string counts. That is the whole of it, and it is deliberately
/// narrow: it says nothing about `resolve_from`, `resolve_to`, or the word
/// `resolve` in prose, none of which SPEC-002/R-2 has a view on.
///
/// Import-shape-blind, which is the property the path substring lacked: a
/// call spelled `schedule::resolve(…)` and one spelled `resolve(…)` after a
/// brace-grouped `use goad_semantics::schedule::{resolve, parse};` both carry
/// `resolve(`, so both count.
fn calls_resolve(code: &str) -> bool {
  let stripped = code_without_literals(code);
  let bytes = stripped.as_bytes();
  stripped.match_indices("resolve(").any(|(at, _)| {
    at == 0
      || bytes
        .get(at - 1)
        .is_none_or(|&before| !before.is_ascii_alphanumeric() && before != b'_')
  })
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
///
/// **Lines as well as files.** A non-zero file
/// count cannot tell a fully-read file from a half-read one, so a blinded
/// file used to be indistinguishable from a clean one. Two further claims
/// close that: every subject file yields at least one production line, and
/// each directory's total clears a floor. Measured on this tree:
/// `crates/goad/src` reads 1907 production lines and `crates/goad-shell/src`
/// reads 1224. The floors sit well under both — they are a tripwire for a
/// scan that stopped reading, not a line-count assertion a refactor should
/// have to service. What makes a *partial* blinding impossible is
/// `production_lines`'s item-scoped cut and its own control below, not this
/// number.
#[test]
fn the_subject_directories_are_found_and_are_not_empty() {
  for (dir, floor) in [(SUBJECT_DIR, 1000), (SHELL_SUBJECT_DIR, 900)] {
    let files = subject_files(dir);
    assert!(
      !files.is_empty(),
      "{dir} inspected no `.rs` file — renamed, emptied, or misspelled"
    );
    let mut total = 0;
    for path in &files {
      let read = production_lines(path).len();
      assert!(
        read > 0,
        "{} yielded no production line at all — the scan is blind to this file",
        path.display()
      );
      total += read;
    }
    assert!(
      total >= floor,
      "{dir} yielded {total} production lines, under the {floor} this tree measures — \
       the scan stopped reading somewhere"
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

/// AC-6 (b), SPEC-002/R-2: `schedule::resolve` is **called** from exactly two
/// places in `crates/goad-shell/src`'s production code, both in `host.rs`, and
/// nowhere takes it as a value without calling it.
///
/// **What the count is about.** R-2 is *"the host MUST NOT resolve a next
/// check anywhere but the one resolution SPEC-001/R-26 describes"*, so the
/// thing to count is the resolving **call**. The count is load-bearing and is
/// the only assertion here that is: a third call added inside `host.rs` would
/// pass the file-set check and is caught by nothing else.
///
/// **What it is deliberately not about.** An earlier form of this instrument
/// counted every production line naming the identifier `resolve` — nine of
/// them, including a user-facing message string in `error.rs` and six
/// `resolve_from`/`resolve_to` lines. Rewording a diagnostic or renaming a
/// private helper turned the boundary suite red without anything having
/// resolved a schedule, and an instrument that reds for reasons its
/// requirement has no view on invites the repair *bump the number* — after
/// which nobody reads its report again. `calls_resolve` counts the call form
/// only, over `code_without_literals`, so neither a message nor a helper name
/// is a boundary fact.
///
/// **What the narrowing would have cost, closed here.** A call matcher cannot
/// see `let f = schedule::resolve;` followed by `f(…)`. The second assertion
/// closes that for the qualified form: every production line naming the path
/// `schedule::resolve` must also be a call line. A rename-import
/// (`use … as r;`) remains outside both, as §5.5 I-1a already concedes.
///
/// Asserted as a count and a set of file names, never as line numbers: a line
/// moving inside `host.rs` is not this instrument's business.
#[test]
fn schedule_resolve_is_called_only_from_host() {
  let called = occurrences_where(SHELL_SUBJECT_DIR, calls_resolve);
  assert_eq!(called.len(), 2, "found:\n{}", report(&called));
  let files: std::collections::BTreeSet<Option<&str>> = called
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
    report(&called)
  );

  let named_but_not_called = occurrences_where(SHELL_SUBJECT_DIR, |code| {
    code_without_literals(code).contains("schedule::resolve") && !calls_resolve(code)
  });
  assert!(
    named_but_not_called.is_empty(),
    "`schedule::resolve` is named without being called — a value taken here is \
     a call site the count above cannot see:\n{}",
    report(&named_but_not_called)
  );
}

/// Controls on this file's own counting and its test-scope cut — `code_of`
/// itself is already proven against real fixtures by `purity.rs` and
/// `vocabulary.rs`.
mod counting_itself {
  use super::{
    calls_resolve, code_of, code_without_literals, mentions, production_lines, workspace_root,
  };

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
  ///
  /// The fixture is **code**, not a comment: a commented fixture is stripped
  /// by `code_of` before the word matcher ever sees the participle, so the
  /// assertion would hold for a reason that has nothing to do with the
  /// participle rule. Verified non-vacuous by breaking
  /// `is_singular_or_plural_of` to strip a trailing `d` and watching this
  /// test go red.
  #[test]
  fn the_participle_resolved_is_not_counted_by_the_word_matcher() {
    let line = "    let resolved = outcome;";
    assert!(
      !code_of(line).is_empty(),
      "the fixture must survive comment stripping, or this control is vacuous"
    );
    assert!(!mentions(line, "resolve"));
  }

  /// The cut is over the **item** the `#[cfg(test)]` attribute introduces,
  /// not over the rest of the file: production code after an inline test
  /// module is production code, and a cut that took the whole tail would
  /// read none of it while reporting the same non-zero file count. Today no
  /// subject file has such a tail, which
  /// is precisely why the fixture is on disk rather than in the tree.
  #[test]
  fn production_after_an_inline_test_module_is_still_read() {
    let path = workspace_root()
      .join("crates/goad-boundary/tests/fixtures/structure/production_after_tests.rs");
    let read = production_lines(&path);
    let names = |needle: &str| read.iter().any(|(_, code)| code.contains(needle));
    assert!(
      names("before_the_module"),
      "nothing before the module was read"
    );
    assert!(
      names("after_the_module"),
      "the cut took the whole tail: production code after an inline test module went unread"
    );
    assert!(
      !names("inside_the_module"),
      "the cut let the test module's own body through"
    );
  }

  /// `code_without_literals`, the text the brace count and the call matcher
  /// both read: a `{` inside a string, a raw string or a char literal is not
  /// a brace, and neither is a `resolve(` inside a message.
  #[test]
  fn a_brace_inside_a_literal_is_not_a_brace() {
    for line in [
      r#"  let opening = "a JSON fragment: {";"#,
      r##"  let raw = r#"another one: {"#;"##,
      r"  let brace = '{';",
    ] {
      let stripped = code_without_literals(line);
      assert!(
        !stripped.contains('{'),
        "{line} still holds a brace after stripping: {stripped}"
      );
    }
  }

  /// The complement: a real brace outside a literal survives, so the strip is
  /// not simply deleting everything.
  #[test]
  fn a_brace_outside_a_literal_survives_the_strip() {
    let line = r#"  mod tests { let s = "no brace here"; }"#;
    let stripped = code_without_literals(line);
    assert_eq!(stripped.matches('{').count(), 1, "{stripped}");
    assert_eq!(stripped.matches('}').count(), 1, "{stripped}");
  }

  /// A literal left open at end of line is cut from its opening delimiter —
  /// `code_of` returns such a line intact, which is right for a scan that
  /// reads words and wrong for one that counts braces.
  #[test]
  fn an_unterminated_literal_is_cut_rather_than_returned_intact() {
    let line = r#"  let s = "an unterminated fragment {"#;
    assert!(!code_without_literals(line).contains('{'));
    assert!(code_of(line).contains('{'));
  }

  /// Neighbouring identifiers are not joined across a cut literal, for the
  /// same reason the block-comment cut leaves a space.
  #[test]
  fn a_cut_literal_leaves_a_space_rather_than_joining_its_neighbours() {
    let stripped = code_without_literals(r#"site"x"view"#);
    assert!(!stripped.contains("siteview"), "{stripped}");
  }

  /// `calls_resolve`, AC-6 (b)'s matcher: the two call spellings count, and
  /// the three shapes the old identifier count reded on do not.
  #[test]
  fn the_call_matcher_counts_calls_and_nothing_else() {
    for line in [
      "    let seed = schedule::resolve(None, None, config.schedule.default_poll, now);",
      "    let seed = resolve(None, None, poll, now);",
    ] {
      assert!(calls_resolve(line), "not counted as a call: {line}");
    }
    for line in [
      "    let next_check = self.resolve_from(value.schedule(), now);",
      "    self.state.resolve_to(next_check);",
      r#"          "{key} = \"{raw}\" is not a duration this host can resolve: {fault}""#,
      "  fn my_resolve(x: u32) -> u32 { x }",
      "use goad_semantics::schedule::resolve;",
    ] {
      assert!(!calls_resolve(line), "wrongly counted as a call: {line}");
    }
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

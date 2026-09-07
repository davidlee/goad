//! One configured walk over a directory tree, looking for tokens a file may not
//! name.
//!
//! The load-bearing part is not the matching, it is the vacuity guard: a grep
//! over a directory that has been renamed away finds no violations, and a run
//! that reports "no violations" for that has stopped testing anything. Extend
//! the *configuration*, not the walk — `crates/goad-boundary/tests/checks/` is
//! where the configurations live, and where every control for this file is.
//!
//! Roots are relative to the **workspace root**, resolved as
//! `CARGO_MANIFEST_DIR` joined with `../..`: this crate sits at depth two like
//! every other member, and a test binary's working directory is not something
//! to rely on.

use std::borrow::Cow;
use std::fmt;
use std::path::{Path, PathBuf};

/// One walk, configured per subject. `root` is a directory relative to the
/// workspace root — a workspace member's own directory, once `crate::members`
/// enumerates it, or a fixture directory a control names directly.
#[derive(Debug)]
pub struct Scan {
  pub root: PathBuf,
  /// File extensions inspected, without the dot: `["rs", "slint"]` (D13).
  pub extensions: &'static [&'static str],
  /// Directory names excluded anywhere in the walk: `["tests", "target"]`.
  /// Excluded by name, not by a fixed layout, so a member with an unusual
  /// shape is still covered without an edit.
  pub excluded_dirs: &'static [&'static str],
  /// Lower-case; matched case-insensitively and by word, so a `Habit` type
  /// cannot hide behind its capital and `HabitView` cannot hide behind its
  /// suffix.
  pub forbidden: &'static [&'static str],
}

/// Why a check failed. `Vacuous` is the reason this crate exists; `GlobMember`
/// is `crate::members`' own guard — a `workspace.members` entry a reader
/// cannot enumerate hides exactly what this crate exists to make visible.
#[derive(Debug)]
pub enum Breach {
  Token {
    path: PathBuf,
    line: usize,
    /// Borrowed for a scan's own `forbidden` list (`&'static str` already);
    /// owned for a dependency name read out of manifest text at runtime,
    /// which cannot be `&'static` without leaking it (`crate::manifest`).
    token: Cow<'static, str>,
  },
  Vacuous {
    root: PathBuf,
  },
  Unreadable {
    path: PathBuf,
    error: String,
  },
  /// A glob in `workspace.members`. It hides from a reader exactly what this
  /// crate exists to make visible, so it fails rather than being expanded.
  GlobMember {
    entry: String,
  },
}

impl fmt::Display for Breach {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Token { path, line, token } => {
        write!(f, "{}:{line}: forbidden token `{token}`", path.display())
      }
      Self::Vacuous { root } => write!(
        f,
        "{}: inspected no scannable file — renamed, emptied, or misspelled",
        root.display()
      ),
      Self::Unreadable { path, error } => {
        write!(
          f,
          "{}: could not be read, so was not inspected: {error}",
          path.display()
        )
      }
      Self::GlobMember { entry } => write!(
        f,
        "`workspace.members` entry \"{entry}\" is a glob, and hides what it names"
      ),
    }
  }
}

pub fn report(breaches: &[Breach]) -> String {
  breaches
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>()
    .join("\n")
}

impl Scan {
  fn resolved_root(&self) -> PathBuf {
    workspace_root().join(&self.root)
  }

  /// `Ok(n)` is the number of files inspected.
  ///
  /// # Errors
  ///
  /// Lists *every* breach, not the first, so one run names all the work: a
  /// forbidden token, an unreadable path, and — the guard — a walk that
  /// inspected nothing at all.
  pub fn run(&self) -> Result<usize, Vec<Breach>> {
    let root = self.resolved_root();
    let mut inspected = 0;
    let mut breaches = Vec::new();
    self.walk(&root, &mut inspected, &mut breaches);
    // The guard. Not `else`: a walk can both find nothing and fail to read,
    // and the vacuity is the finding worth naming either way.
    if inspected == 0 {
      breaches.push(Breach::Vacuous { root });
    }
    if breaches.is_empty() {
      Ok(inspected)
    } else {
      Err(breaches)
    }
  }

  fn is_excluded(&self, path: &Path) -> bool {
    path
      .file_name()
      .and_then(|name| name.to_str())
      .is_some_and(|name| self.excluded_dirs.contains(&name))
  }

  fn is_scanned_extension(&self, path: &Path) -> bool {
    path
      .extension()
      .and_then(|extension| extension.to_str())
      .is_some_and(|extension| self.extensions.contains(&extension))
  }

  fn walk(&self, dir: &Path, inspected: &mut usize, breaches: &mut Vec<Breach>) {
    let entries = match std::fs::read_dir(dir) {
      Ok(entries) => entries,
      Err(error) => {
        breaches.push(Breach::Unreadable {
          path: dir.to_owned(),
          error: error.to_string(),
        });
        return;
      }
    };
    // Sorted so a failure reads the same way twice.
    let mut paths = Vec::new();
    for entry in entries {
      match entry {
        Ok(entry) => paths.push(entry.path()),
        Err(error) => breaches.push(Breach::Unreadable {
          path: dir.to_owned(),
          error: error.to_string(),
        }),
      }
    }
    paths.sort();
    for path in paths {
      if path.is_dir() {
        if !self.is_excluded(&path) {
          self.walk(&path, inspected, breaches);
        }
      } else if self.is_scanned_extension(&path) {
        self.inspect(&path, inspected, breaches);
      }
    }
  }

  fn inspect(&self, path: &Path, inspected: &mut usize, breaches: &mut Vec<Breach>) {
    let text = match std::fs::read_to_string(path) {
      Ok(text) => text,
      Err(error) => {
        breaches.push(Breach::Unreadable {
          path: path.to_owned(),
          error: error.to_string(),
        });
        return;
      }
    };
    *inspected += 1;
    for (offset, line) in text.lines().enumerate() {
      for &token in self.forbidden {
        if mentions(line, token) {
          breaches.push(Breach::Token {
            path: path.to_owned(),
            line: offset + 1,
            token: Cow::Borrowed(token),
          });
        }
      }
    }
  }
}

/// The workspace root, resolved the same way every target in this tree
/// resolves it: `CARGO_MANIFEST_DIR` joined with `../..`. A test binary's
/// working directory is not something to rely on (§5.6, §12.8).
pub fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Does a line of code name the token? A path token — one containing `::` —
/// is matched as a substring, since no identifier word can contain one (F-45).
/// A word token must appear as an identifier **word** rather than as a
/// substring: `site` must catch `SiteView`, `site_id`, `Sites`, `HTTPSite` and
/// `site2`, and must not catch `websites` or `offsite` (F-14, F-36, F-49).
///
/// Comment text is cut off first by `code_of` — a vocabulary check is about
/// what the code names, and "call sites" in prose is not a domain type. Words
/// are then split on every non-alphanumeric byte — which separates identifier
/// segments (`_`, `::`, `.`) — at each case boundary, which is what separates
/// `Site` from `View` and `HTTP` from `Site`, and between a letter and a
/// digit. A word matches the token or its plural: `Habits` names the domain as
/// plainly as `Habit`. Lowercasing happens after the split, so the case
/// boundaries are still there to split on.
///
/// Known and accepted, because nothing in `src/` has any of them: an all-caps
/// compound such as `SITEID` has no boundary to split on (F-49).
pub fn mentions(line: &str, token: &str) -> bool {
  let code = code_of(line);
  if token.contains("::") {
    return code.contains(token);
  }
  code
    .split(|c: char| !c.is_ascii_alphanumeric())
    .flat_map(camel_segments)
    .any(|word| is_singular_or_plural_of(word, token))
}

/// The line with comments removed, string literals intact (D13).
///
/// A per-line state machine, restarted at every line: `Code` watches for a
/// `"` (a plain string), an `r` opening a raw string (`r"…"`, `r#"…"#`, …), a
/// `//` (the rest of the line is a comment), or a `/*` (a block comment). A
/// plain string ends at an unescaped `"`; a raw string ends at a `"` followed
/// by exactly as many `#`s as it opened with. `'` opens a string state only
/// when it closes a char literal three or four bytes later (`char_literal_len`)
/// — `'"'`, `'\''`, `b'"'` — so a quote inside one cannot be mistaken for a
/// string's own open quote; otherwise it is a lifetime (`&'static str`) and
/// is skipped, not a delimiter (F-4, review-code 002 round 1).
///
/// Ending a line still inside a string returns the line intact — an
/// unterminated string is content, and content is scanned. A block comment
/// that does not close on this line truncates the line at its `/*`, the same
/// as `//`. A block comment that *does* close on this line is cut out and
/// replaced by **one space**, never by nothing: `Site/*x*/View` must not
/// become the single word `SiteView` — a joined pair would be a false
/// negative in the direction that matters.
///
/// Borrowed when nothing was cut or the cut ran to end of line — the common
/// case; owned only when an interior `/* … */` closed and left code behind
/// it.
///
/// Four costs, named rather than assumed away (D13): a Rust string literal
/// spanning lines is not tracked across the break, so a `//` inside a
/// continued string hides the rest of that line; a `/* */` block spanning
/// lines cuts line one at `/*` and scans the rest of the *next* line as code;
/// `r"` is recognised in `.slint` too, where the construct does not exist,
/// which is inert there; and an all-caps compound has no case boundary for
/// `mentions` to split on.
///
/// A fifth cost stood here unnamed (F-4, review-code 002 round 1): a char
/// literal holding a quote (`'"'`, `b'"'`, `'\''`) desynchronised the cut —
/// the *next* real string's opening quote was read as a spurious close,
/// re-entering `Code` inside that string, and a `//` there truncated the
/// line early. It is now closed, not merely named: `char_literal_len` gives
/// `'` a fifth transition, so it opens no string state when it closes a char
/// literal three or four bytes later.
pub fn code_of(line: &str) -> Cow<'_, str> {
  strip(line, Literals::Kept)
}

/// The line with comments removed **and every string and char literal
/// replaced by one space**, delimiters included.
///
/// `code_of`'s sibling, over the same state machine, differing only in what
/// it does with a literal once it has found where it ends. For a caller whose
/// question is about the *structure* of the code rather than the words in it
/// — counting braces to find where an item ends, or spotting a call form —
/// a literal's contents are not code, and an unbalanced `{` inside one
/// desynchronises the count for the rest of the file.
///
/// One space, never nothing, for the same reason the block-comment cut uses
/// one: two identifiers either side of a literal must not be joined into a
/// third that neither of them is.
///
/// A literal left open at end of line is cut from its opening delimiter,
/// which is where `code_of` instead returns the line intact — an
/// unterminated literal is content to a scan that reads words, and is not
/// code to a scan that counts braces.
#[must_use]
pub fn code_without_literals(line: &str) -> Cow<'_, str> {
  strip(line, Literals::Cut)
}

/// What `strip` does with a literal: leave its contents alone (`code_of`, for
/// the scans that read words) or replace the whole literal with one space
/// (`code_without_literals`, for the scans that read structure).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Literals {
  Kept,
  Cut,
}

fn strip(line: &str, literals: Literals) -> Cow<'_, str> {
  #[derive(Clone, Copy)]
  enum State {
    Code,
    Str,
    RawStr(usize),
  }

  let bytes = line.as_bytes();
  let mut state = State::Code;
  let mut owned: Option<String> = None;
  let mut copied_up_to = 0usize;
  let mut i = 0usize;
  // Where the literal now open began, when one is open and is to be cut.
  let mut literal_from: Option<usize> = None;

  while let Some(&b) = bytes.get(i) {
    match state {
      State::Code if b == b'"' => {
        state = State::Str;
        if literals == Literals::Cut {
          literal_from = Some(i);
        }
        i += 1;
      }
      State::Code if b == b'r' && starts_raw_string(bytes, i) => {
        let hashes = raw_hash_count(bytes, i);
        state = State::RawStr(hashes);
        if literals == Literals::Cut {
          literal_from = Some(i);
        }
        i += hashes + 2; // `r`, the hashes, the opening quote
      }
      State::Code if b == b'\'' => {
        // A char literal's own quote is not a lifetime's `'` (F-4): skip
        // past it whole, so the quote it carries cannot be mistaken for a
        // string's opening one. A lifetime has no closing `'` at either
        // offset a char literal's content leaves it at, so it falls
        // through to the plain `i += 1` below, unconsumed.
        match char_literal_len(bytes, i) {
          Some(len) if literals == Literals::Cut => {
            cut_out(line, &mut owned, &mut copied_up_to, i, i + len);
            i += len;
          }
          Some(len) => i += len,
          None => i += 1,
        }
      }
      State::Code if b == b'/' && bytes.get(i + 1) == Some(&b'/') => {
        return finish(line, owned, copied_up_to, i);
      }
      State::Code if b == b'/' && bytes.get(i + 1) == Some(&b'*') => {
        match line.get(i + 2..).and_then(|rest| rest.find("*/")) {
          Some(offset) => {
            let close_at = i + 2 + offset + 2;
            cut_out(line, &mut owned, &mut copied_up_to, i, close_at);
            i = close_at;
          }
          None => return finish(line, owned, copied_up_to, i),
        }
      }
      State::Str if b == b'\\' => i += 2,
      State::Str if b == b'"' => {
        state = State::Code;
        i += 1;
        if let Some(from) = literal_from.take() {
          cut_out(line, &mut owned, &mut copied_up_to, from, i);
        }
      }
      State::RawStr(hashes) if b == b'"' && closes_raw_string(bytes, i, hashes) => {
        state = State::Code;
        i += hashes + 1;
        if let Some(from) = literal_from.take() {
          cut_out(line, &mut owned, &mut copied_up_to, from, i);
        }
      }
      State::Code | State::Str | State::RawStr(_) => i += 1,
    }
  }

  // A literal still open at end of line: cut from where it began, so no
  // brace inside it is counted and no half-literal is read as code.
  if let Some(from) = literal_from {
    return finish(line, owned, copied_up_to, from);
  }
  finish(line, owned, copied_up_to, line.len())
}

/// Replace `line[from..to]` with one space in the owned buffer, materialising
/// it if this is the first cut. The block-comment arm's own move, factored
/// out so the literal cuts spell it once rather than four times.
fn cut_out(
  line: &str,
  owned: &mut Option<String>,
  copied_up_to: &mut usize,
  from: usize,
  to: usize,
) {
  let buf = owned.get_or_insert_with(String::new);
  if let Some(before) = line.get(*copied_up_to..from) {
    buf.push_str(before);
  }
  buf.push(' ');
  *copied_up_to = to;
}

/// `owned`, if any, already holds `line[..copied_up_to]` with its interior
/// blocks replaced; this appends the clean tail up to `cut_at` and returns it,
/// or borrows `line[..cut_at]` directly when no block ever fired.
fn finish(line: &str, owned: Option<String>, copied_up_to: usize, cut_at: usize) -> Cow<'_, str> {
  match owned {
    Some(mut buf) => {
      if let Some(tail) = line.get(copied_up_to..cut_at) {
        buf.push_str(tail);
      }
      Cow::Owned(buf)
    }
    None => Cow::Borrowed(line.get(..cut_at).unwrap_or(line)),
  }
}

/// Is `bytes[r_pos]` (an `r`) immediately followed by zero or more `#` and
/// then a `"` — the raw-string opener?
fn starts_raw_string(bytes: &[u8], r_pos: usize) -> bool {
  let mut j = r_pos + 1;
  while bytes.get(j) == Some(&b'#') {
    j += 1;
  }
  bytes.get(j) == Some(&b'"')
}

/// The number of `#`s a raw-string opener at `r_pos` carries.
fn raw_hash_count(bytes: &[u8], r_pos: usize) -> usize {
  let mut j = r_pos + 1;
  let mut hashes = 0;
  while bytes.get(j) == Some(&b'#') {
    hashes += 1;
    j += 1;
  }
  hashes
}

/// Is the `"` at `quote_pos` followed by exactly `hashes` more `#`s — closing
/// a raw string opened with that many?
fn closes_raw_string(bytes: &[u8], quote_pos: usize, hashes: usize) -> bool {
  (0..hashes).all(|offset| bytes.get(quote_pos + 1 + offset) == Some(&b'#'))
}

/// The byte length of a char literal opening at `bytes[at]` (a `'`), if this
/// is one rather than a lifetime — `&'static`, `'a` — which has no closing
/// `'` at either offset a literal's content leaves it at (F-4). Two shapes:
/// an escape (`'\''`, `'\n'`) at four bytes, quote-backslash-char-quote; a
/// single byte at three, quote-char-quote (`'"'`, `'x'`, and `b'"'`'s
/// quoted half, the `b` itself being ordinary code one byte earlier). A
/// multi-byte escape (`'\u{1F600}'`) or character is not one of the two
/// shapes and is left as a lifetime — D13's cost, unchanged by this fix.
fn char_literal_len(bytes: &[u8], at: usize) -> Option<usize> {
  if bytes.get(at + 1) == Some(&b'\\') && bytes.get(at + 3) == Some(&b'\'') {
    return Some(4);
  }
  if bytes.get(at + 1).is_some_and(|&c| c != b'\\') && bytes.get(at + 2) == Some(&b'\'') {
    return Some(3);
  }
  None
}

fn is_singular_or_plural_of(word: &str, token: &str) -> bool {
  word.eq_ignore_ascii_case(token)
    || word
      .strip_suffix(['s', 'S'])
      .is_some_and(|stem| stem.eq_ignore_ascii_case(token) || is_es_plural(stem, token))
}

fn is_es_plural(stem: &str, token: &str) -> bool {
  stem
    .strip_suffix(['e', 'E'])
    .is_some_and(|stem| stem.eq_ignore_ascii_case(token))
}

/// `SiteView` → `Site`, `View`; `HTTPSite` → `HTTP`, `Site`; `habit2` →
/// `habit`, `2`. A segment with no boundary is itself.
fn camel_segments(segment: &str) -> Vec<&str> {
  let mut words = Vec::new();
  let mut start = 0;
  let bytes = segment.as_bytes();
  // Zipped with itself, and `get` rather than `[]`, because `indexing_slicing` is
  // `deny` and `clippy.toml`'s `allow-indexing-slicing-in-tests` stopped applying
  // the moment this walk became a library rather than a test module
  // (`review-plan.md` F-36). The reads are the same two bytes as before.
  for ((offset, &c), &previous) in bytes.iter().enumerate().skip(1).zip(bytes) {
    let lower_to_upper = c.is_ascii_uppercase() && previous.is_ascii_lowercase();
    let letter_to_digit = c.is_ascii_digit() != previous.is_ascii_digit();
    let acronym_ends = c.is_ascii_lowercase()
      && previous.is_ascii_uppercase()
      && offset >= 2
      && bytes.get(offset - 2).is_some_and(u8::is_ascii_uppercase);
    if lower_to_upper || letter_to_digit {
      words.push(&segment[start..offset]);
      start = offset;
    } else if acronym_ends {
      words.push(&segment[start..offset - 1]);
      start = offset - 1;
    }
  }
  words.push(&segment[start..]);
  words
}

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

use std::fmt;
use std::path::{Path, PathBuf};

/// One walk, configured per subject. `root` is relative to the workspace root.
#[derive(Debug)]
pub struct Scan {
  pub root: &'static str,
  /// Lower-case; matched case-insensitively and by word, so a `Habit` type
  /// cannot hide behind its capital and `HabitView` cannot hide behind its
  /// suffix.
  pub forbidden: &'static [&'static str],
}

/// Why a scan failed. `Vacuous` is the reason this file exists.
#[derive(Debug)]
pub enum Breach {
  Token {
    path: PathBuf,
    line: usize,
    token: &'static str,
  },
  Vacuous {
    root: PathBuf,
  },
  Unreadable {
    path: PathBuf,
    error: String,
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
        "{}: inspected no .rs files — renamed, emptied, or misspelled",
        root.display()
      ),
      Self::Unreadable { path, error } => {
        write!(
          f,
          "{}: could not be read, so was not inspected: {error}",
          path.display()
        )
      }
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
  fn root(&self) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../..")
      .join(self.root)
  }

  /// `Ok(n)` is the number of files inspected.
  ///
  /// # Errors
  ///
  /// Lists *every* breach, not the first, so one run names all the work: a
  /// forbidden token, an unreadable path, and — the guard — a walk that
  /// inspected nothing at all.
  pub fn run(&self) -> Result<usize, Vec<Breach>> {
    let root = self.root();
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
        self.walk(&path, inspected, breaches);
      } else if path.extension().is_some_and(|extension| extension == "rs") {
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
      for token in self.forbidden {
        if mentions(line, token) {
          breaches.push(Breach::Token {
            path: path.to_owned(),
            line: offset + 1,
            token,
          });
        }
      }
    }
  }
}

/// Does a line of code name the token? A path token — one containing `::` —
/// is matched as a substring, since no identifier word can contain one (F-45).
/// A word token must appear as an identifier **word** rather than as a
/// substring: `site` must catch `SiteView`, `site_id`, `Sites`, `HTTPSite` and
/// `site2`, and must not catch `websites` or `offsite` (F-14, F-36, F-49).
///
/// Comment text is cut off first — AC-11 is about what the code names, and
/// "call sites" in prose is not a domain type. Words are then split on every
/// non-alphanumeric byte — which separates identifier segments (`_`, `::`,
/// `.`) — at each case boundary, which is what separates `Site` from `View`
/// and `HTTP` from `Site`, and between a letter and a digit. A word matches the
/// token or its plural: `Habits` names the domain as plainly as `Habit`.
/// Lowercasing happens after the split, so the case boundaries are still there
/// to split on.
///
/// Known and accepted, because nothing in `src/` has any of them: the comment
/// cut is a `//` search, so a `//` inside a string literal hides the rest of
/// that line and a `/* */` block is not cut at all; an all-caps compound such
/// as `SITEID` has no boundary to split on (F-49).
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

/// The line with any `//` comment removed.
fn code_of(line: &str) -> &str {
  line.find("//").map_or(line, |comment| &line[..comment])
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

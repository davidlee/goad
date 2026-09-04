//! AC-15's *direction* half and AC-11's vocabulary check, as tests rather than
//! intentions.
//!
//! Both are one walk under two configurations. The load-bearing part is not the
//! matching, it is the vacuity guard: a grep over a directory that has been
//! renamed away finds no violations, and a run that reports "no violations" for
//! that has stopped testing anything. PHASE-09 extends this file — extend the
//! configuration, not the walk.

use std::fmt;
use std::path::{Path, PathBuf};

/// One walk, configured twice. `root` is relative to the crate root.
struct Scan {
  root: &'static str,
  /// Lower-case; matched case-insensitively and by word, so a `Habit` type
  /// cannot hide behind its capital and `HabitView` cannot hide behind its
  /// suffix.
  forbidden: &'static [&'static str],
}

/// Why a scan failed. `Vacuous` is the reason this file exists.
#[derive(Debug)]
enum Breach {
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

fn report(breaches: &[Breach]) -> String {
  breaches
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>()
    .join("\n")
}

impl Scan {
  fn root(&self) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(self.root)
  }

  /// `Ok(n)` is the number of files inspected. `Err` lists *every* breach, not
  /// the first, so one run names all the work.
  fn run(&self) -> Result<usize, Vec<Breach>> {
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
fn mentions(line: &str, token: &str) -> bool {
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
  for (offset, &c) in bytes.iter().enumerate().skip(1) {
    let previous = bytes[offset - 1];
    let lower_to_upper = c.is_ascii_uppercase() && previous.is_ascii_lowercase();
    let letter_to_digit = c.is_ascii_digit() != previous.is_ascii_digit();
    let acronym_ends = c.is_ascii_lowercase()
      && previous.is_ascii_uppercase()
      && offset >= 2
      && bytes[offset - 2].is_ascii_uppercase();
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

/// Fails naming *every* breach, not the first. `run` cannot return `Ok(0)`, so
/// arriving at `Ok` at all is the vacuity guard discharging.
fn assert_clean(scan: &Scan) {
  if let Err(breaches) = scan.run() {
    panic!("{}", report(&breaches));
  }
}

/// AC-15's direction half. The dependency-graph half is the build gate — this
/// catches the `use crate::shell::…` a feature flag cannot see.
const STRATUM_1_LOOKS_ONLY_DOWN: Scan = Scan {
  root: "src/semantics",
  forbidden: &["crate::shell", "crate::bin", "tokio"],
};

/// AC-11, `slice-001.md:147`. The host does not understand the user's domain,
/// so it may not name it.
const NO_DOMAIN_VOCABULARY: Scan = Scan {
  root: "src",
  forbidden: &[
    "habit",
    "streak",
    "journal",
    "site",
    "goal",
    "reminder",
    "compliance",
  ],
};

#[test]
fn stratum_1_names_neither_the_shell_a_binary_nor_the_runtime() {
  assert_clean(&STRATUM_1_LOOKS_ONLY_DOWN);
}

#[test]
fn no_host_source_file_names_the_user_s_domain() {
  assert_clean(&NO_DOMAIN_VOCABULARY);
}

/// A directory that exists and holds files, none of them Rust. Nothing is
/// forbidden, so the only thing that can fail this is the guard.
const NOTHING_TO_INSPECT: Scan = Scan {
  root: "docs/adr",
  forbidden: &[],
};

/// The threat in its literal form: `src/semantics/` renamed, the scan left
/// pointing at where it used to be.
const RENAMED_AWAY: Scan = Scan {
  root: "src/semantics-renamed",
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

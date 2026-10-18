//! The kit stands alone (`design.md` §5.2.6, I-5).
//!
//! A consumer receives `kit/` and nothing else, so nothing in it may name a
//! path outside it: neither a relative path that climbs out, nor a path this
//! repository tracks outside the kit. The tracked paths are read from git when
//! the test runs, never listed.
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::tree;

/// Which half of I-5 a text breaks.
#[derive(Debug, PartialEq)]
enum Half {
  /// A relative path that, resolved from its file's directory, leaves `kit/`.
  Escaping,
  /// `<name>/<segment>` that starts a path tracked outside `kit/`.
  Mention,
}

/// One breach: its line, its half, and the text that breaks it.
#[derive(Debug)]
struct Breach {
  line: usize,
  half: Half,
  text: String,
}

/// What `trim_matches` takes off a token's ends before it is read as a path:
/// quotes, backticks, brackets, braces, parentheses, and the `,` and `;` that
/// JSON and prose put after one. Not `.`, which begins `../`.
const AROUND: &[char] = &[
  '"', '\'', '`', '(', ')', '[', ']', '{', '}', '<', '>', ',', ';',
];

/// A character a path continues through.
fn continues(c: char) -> bool {
  c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')
}

/// The repository root, from this package's manifest.
fn repository() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every path git tracks outside `kit/`. A missing `git` or `.git` fails;
/// it never skips.
fn tracked_outside() -> Vec<String> {
  let output = Command::new("git")
    .arg("ls-files")
    .current_dir(repository())
    .output()
    .expect("git must be runnable: I-5 reads the tracked tree");
  assert!(
    output.status.success(),
    "git ls-files at {} failed: {}",
    repository().display(),
    String::from_utf8_lossy(&output.stderr)
  );
  let listed = String::from_utf8(output.stdout).expect("git lists UTF-8 paths");
  let paths: Vec<&str> = listed.lines().collect();
  assert!(
    paths.iter().any(|path| path.starts_with("kit/"))
      && paths.iter().any(|path| path.starts_with("crates/")),
    "git ls-files listed nothing under kit/ or crates/: the wrong root?\n{listed}"
  );
  paths
    .into_iter()
    .filter(|path| !path.starts_with("kit/"))
    .map(str::to_owned)
    .collect()
}

/// Each breach in `text`, for a file in `dir` (relative to the repository
/// root).
fn breaches(text: &str, dir: &Path, tracked: &[String]) -> Vec<Breach> {
  let mut found = Vec::new();
  for (index, line) in text.lines().enumerate() {
    let breach = |half, breaking: &str| Breach {
      line: index + 1,
      half,
      text: breaking.to_owned(),
    };
    found.extend(
      line
        .split_whitespace()
        .map(|token| token.trim_matches(AROUND))
        .filter(|token| escapes(token, dir))
        .map(|token| breach(Half::Escaping, token)),
    );
    found.extend(
      mentions(line)
        .filter(|mention| names_tracked(mention, tracked))
        .map(|mention| breach(Half::Mention, mention)),
    );
  }
  found
}

/// A relative path holding `../` that, resolved lexically from `dir`, is not
/// under `kit/`, or climbs above the root.
fn escapes(token: &str, dir: &Path) -> bool {
  if !token.contains("../") || token.starts_with('/') {
    return false;
  }
  let mut stack: Vec<String> = dir
    .components()
    .map(|component| component.as_os_str().to_string_lossy().into_owned())
    .collect();
  for component in token.split('/') {
    match component {
      "" | "." => {}
      ".." => {
        if stack.pop().is_none() {
          return true;
        }
      }
      name => stack.push(name.to_owned()),
    }
  }
  stack.first().is_none_or(|top| top != "kit")
}

/// Every `<name>/<segment>` in `line` whose `<name>` does not continue a path:
/// both whole runs of path characters, and the character before `<name>` none
/// a path continues through, nor `/`.
fn mentions(line: &str) -> impl Iterator<Item = &str> {
  let run = move |from: usize| {
    line[from..]
      .find(|c: char| !continues(c))
      .map_or(line.len(), |length| from + length)
  };
  line.char_indices().filter_map(move |(start, first)| {
    let before = line[..start].chars().next_back();
    if !continues(first) || before.is_some_and(|b| continues(b) || b == '/') {
      return None;
    }
    let slash = run(start);
    if !line[slash..].starts_with('/') {
      return None;
    }
    let end = run(slash + 1);
    (end > slash + 1).then(|| &line[start..end])
  })
}

/// `mention` is a tracked path, or the start of one up to a `/`: a whole
/// component, so `crates/goad-s` is not `crates/goad-shell`.
fn names_tracked(mention: &str, tracked: &[String]) -> bool {
  tracked.iter().any(|path| {
    path
      .strip_prefix(mention)
      .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
  })
}

/// The notional directory of `a_path_outside_the_kit_is_refused`'s strings.
const REFERENCE: &str = "kit/skills/goad-backend/reference";

#[test]
fn a_path_outside_the_kit_is_refused() {
  let tracked = tracked_outside();
  let check = |text: &str| breaches(text, Path::new(REFERENCE), &tracked);
  let refused = [
    (
      "see ../../../../crates/goad-shell/src",
      Half::Escaping,
      "../../../../crates/goad-shell/src",
    ),
    (
      "see crates/goad-shell/src",
      Half::Mention,
      "crates/goad-shell",
    ),
  ]
  .into_iter()
  .filter(|(text, half, breaking)| {
    !matches!(check(text).as_slice(), [found] if found.line == 1 && found.half == *half && found.text == *breaking)
  });
  let accepted = [
    "see ../examples/focus-check/README.md",
    "install into .claude/skills/",
    "the manifest, `.claude-plugin/plugin.json`",
    "in /elsewhere/crates/goad-shell/src",
    "not crates/goad-s",
  ]
  .into_iter()
  .filter(|text| !check(text).is_empty());
  let complaints: Vec<String> = refused
    .map(|(text, ..)| text)
    .chain(accepted)
    .map(|text| format!("{text}: {:?}", check(text)))
    .collect();
  assert!(
    complaints.is_empty(),
    "cases not held:\n{}",
    complaints.join("\n")
  );
}

#[test]
fn nothing_in_the_kit_names_a_path_outside_it() {
  let tracked = tracked_outside();
  let files = tree::files(&tree::root());
  assert!(!files.is_empty(), "the kit holds no file to check");
  let mut found = Vec::new();
  for file in &files {
    let relative = Path::new("kit").join(file.strip_prefix(tree::root()).expect("under the kit"));
    let dir = relative.parent().expect("a file has a directory");
    for breach in breaches(&tree::text(file), dir, &tracked) {
      found.push(format!(
        "{}:{}: {:?} {}",
        relative.display(),
        breach.line,
        breach.half,
        breach.text
      ));
    }
  }
  assert!(
    found.is_empty(),
    "the kit names paths outside it:\n{}",
    found.join("\n")
  );
}

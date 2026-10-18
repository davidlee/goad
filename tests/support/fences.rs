//! A `CommonMark` fence scanner: backtick and tilde fences, no dependency.
//!
//! Included through `#[path]` by `goad-check`'s `kit` target (the fence gate)
//! and `goad-shell`'s `integration` target (`round_trip.rs`, reading a
//! README's config). Every symbol here is used by both, or `dead_code` fails
//! the gate at the includer that leaves one unused
//! (`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`), so
//! the file holds the scanner and nothing else. It replaced `round_trip.rs`'
//! own `fenced_block`, which found only a backtick fence of one exact info
//! string.
//!
//! Indented code blocks are not fences, and are not seen.
use std::ops::Range;

/// One fenced code block.
pub(crate) struct Fence<'a> {
  /// The info string, trimmed: its words are the language and anything after.
  pub(crate) info: &'a str,
  /// The content lines' byte range in the document: from the line after the
  /// opener to the start of the closer. A range rather than a `&str`, so a
  /// caller can locate the block as well as read it.
  pub(crate) body: Range<usize>,
}

/// Every fence in `markdown`, in document order.
///
/// An opener is a line indented at most three spaces, then three or more of
/// one fence character, then the info string; a backtick opener whose info
/// string holds a backtick is not one. The closer is the next such line of at
/// least as many of the **same** character, followed only by whitespace.
///
/// # Panics
///
/// On a fence with no closer, naming its opener's line. `CommonMark` runs one to
/// the end of the document, which would let an unclosed fence swallow the
/// fences after it and shrink a checked set unseen.
pub(crate) fn fences(markdown: &str) -> Vec<Fence<'_>> {
  let mut found = Vec::new();
  let mut open: Option<(char, usize, &str, usize)> = None;
  let mut offset = 0;
  for line in markdown.split_inclusive('\n') {
    let start = offset;
    offset += line.len();
    match open {
      None => {
        open = run(line)
          .filter(|(mark, _, info)| *mark != '`' || !info.contains('`'))
          .map(|(mark, length, info)| (mark, length, info.trim(), offset));
      }
      Some((mark, length, info, body)) => {
        if run(line).is_some_and(|(m, l, rest)| m == mark && l >= length && rest.trim().is_empty())
        {
          found.push(Fence {
            info,
            body: body..start,
          });
          open = None;
        }
      }
    }
  }
  if let Some((.., body)) = open {
    panic!(
      "the fence opened at line {} is never closed",
      markdown[..body].lines().count()
    );
  }
  found
}

/// A line's leading run of one fence character, at most three spaces in:
/// the character, the run's length, and the rest of the line.
fn run(line: &str) -> Option<(char, usize, &str)> {
  let unindented = line.trim_start_matches(' ');
  if line.len() - unindented.len() > 3 {
    return None;
  }
  let mark = unindented
    .chars()
    .next()
    .filter(|c| matches!(c, '`' | '~'))?;
  let rest = unindented.trim_start_matches(mark);
  let length = unindented.len() - rest.len();
  (length >= 3).then_some((mark, length, rest))
}

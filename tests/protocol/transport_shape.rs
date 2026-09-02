//! Three regressions in `process.rs`, asserted against its source text.
//!
//! Each was a repair in the design review, so each is a regression with a name:
//! the drain is a sub-future and not a task (F-49), there is no lock around the
//! buffer it writes into (F-36, D44), and no `?` sits between the spawn and the
//! cleanup budget where it could skip disposal (F-41). None of the three is
//! observable from a passing exchange — a transport that spawns a task still
//! answers correctly — which is why they are read rather than run.
//!
//! **Not in `boundary.rs`**, and not by extending it. `Scan` there is a
//! forbidden-token walk over a *directory*, and only the first of these is that
//! shape: the second constrains an occurrence's shape and the third a region.
//! Generalising `Scan` to carry a per-line predicate and a region state would
//! rework it to serve an unrelated question, against its own instruction to
//! extend the configuration and not the walk. What is kept is the idea of the
//! guard, whose form changes with the subject: there, that the walk inspected
//! files; here, that the file was found, read, and had the anchors in it.

use std::fmt;
use std::path::{Path, PathBuf};

/// The subject. One file, not a tree.
struct Source {
  path: &'static str,
}

const TRANSPORT: Source = Source {
  path: "src/shell/backend/process.rs",
};

/// One line of code, with the comments and string literals taken out.
struct Code {
  number: usize,
  text: String,
}

/// Why a check failed. The last two are the guard: a check that cannot find its
/// subject, or its anchors within it, has stopped testing anything.
#[derive(Debug)]
enum Breach {
  Token { line: usize, token: &'static str },
  Spawn { found: Vec<String> },
  QuestionMark { line: usize },
  Unreadable { path: PathBuf, error: String },
  NoAnchor { path: PathBuf, anchor: &'static str },
}

impl fmt::Display for Breach {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Token { line, token } => {
        write!(
          f,
          "line {line}: `{token}` — the drain borrows, it does not share"
        )
      }
      Self::Spawn { found } => write!(
        f,
        "the only permitted spawn is the child's, `command.spawn()`; found: {}",
        found.join(" | ")
      ),
      Self::QuestionMark { line } => write!(
        f,
        "line {line}: `?` between the spawn and the cleanup budget — a return there skips disposal (F-41)"
      ),
      Self::Unreadable { path, error } => {
        write!(f, "{}: could not be read: {error}", path.display())
      }
      Self::NoAnchor { path, anchor } => write!(
        f,
        "{}: no line holds `{anchor}` — renamed, restructured, or the check is looking at the wrong file",
        path.display()
      ),
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

impl Source {
  /// The file's code, with comments and string literals removed.
  ///
  /// Crude on purpose: a tiny state machine over `"` and `//`, which handles a
  /// `//` inside a string and a `"` inside a comment and nothing more exotic.
  /// The file is ours, and a check that is easy to read beats one that is hard
  /// to fool. It knows nothing of block comments or raw strings; if
  /// `process.rs` grows either, this is what to extend.
  fn code(&self) -> Result<Vec<Code>, Breach> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(self.path);
    let text = std::fs::read_to_string(&path).map_err(|error| Breach::Unreadable {
      path: path.clone(),
      error: error.to_string(),
    })?;
    let lines = text
      .lines()
      .enumerate()
      .map(|(offset, line)| Code {
        number: offset + 1,
        text: strip(line),
      })
      .collect();
    Ok(lines)
  }

  /// The lines between the spawn and the cleanup budget — F-41's region.
  ///
  /// It ends at the budget rather than after it: the design's own structure
  /// uses `?` *inside* the budget, where every arm still reports on the cleanup
  /// channel. What must not appear is a `?` on the way there.
  fn region(&self) -> Result<Vec<Code>, Vec<Breach>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(self.path);
    let code = self.code().map_err(|breach| vec![breach])?;
    let Some(spawn) = code.iter().position(|line| line.text.contains(".spawn()")) else {
      return Err(vec![Breach::NoAnchor {
        path,
        anchor: ".spawn()",
      }]);
    };
    let after_spawn = code.into_iter().skip(spawn);
    let mut region: Vec<Code> = Vec::new();
    for line in after_spawn {
      let budget = line.text.contains("CLEANUP_LIMIT");
      region.push(line);
      if budget {
        return Ok(region);
      }
    }
    Err(vec![Breach::NoAnchor {
      path,
      anchor: "CLEANUP_LIMIT",
    }])
  }
}

/// Comments and string literals out, in one pass.
fn strip(line: &str) -> String {
  let mut out = String::new();
  let mut in_string = false;
  let mut escaped = false;
  let mut previous = ' ';
  for character in line.chars() {
    if in_string {
      if escaped {
        escaped = false;
      } else if character == '\\' {
        escaped = true;
      } else if character == '"' {
        in_string = false;
      }
      continue;
    }
    if character == '"' {
      in_string = true;
      continue;
    }
    if character == '/' && previous == '/' {
      out.pop();
      break;
    }
    out.push(character);
    previous = character;
  }
  out
}

/// Nothing is shared, because nothing is spawned: the drain is a sub-future of
/// the exchange and borrows the buffer it fills (D44, F-49). An `Arc` or a
/// `Mutex` appearing here means that stopped being true.
#[test]
fn the_transport_shares_nothing_with_anything() {
  let code = TRANSPORT.code().unwrap_or_else(|breach| panic!("{breach}"));
  let breaches: Vec<Breach> = code
    .iter()
    .flat_map(|line| {
      ["Arc", "Mutex"]
        .into_iter()
        .filter(move |token| line.text.contains(token))
        .map(move |token| Breach::Token {
          line: line.number,
          token,
        })
    })
    .collect();
  assert!(breaches.is_empty(), "{}", report(&breaches));
  assert!(!code.is_empty(), "{} held no code", TRANSPORT.path);
}

/// The only thing this transport spawns is the child.
///
/// The token is `spawn`, not `tokio::spawn`: naming the one API would leave
/// `Handle::spawn`, `spawn_blocking`, `spawn_local` and `JoinSet::spawn`
/// through, and F-49's leak needs only one of them (F-12). So every occurrence
/// is counted and exactly one shape is permitted — `command.spawn()`, where
/// `command` is the `Command` builder bound just above it.
#[test]
fn the_only_spawn_is_the_child() {
  let code = TRANSPORT.code().unwrap_or_else(|breach| panic!("{breach}"));
  let found: Vec<String> = code
    .iter()
    .filter(|line| line.text.contains("spawn"))
    .map(|line| line.text.trim().to_owned())
    .collect();
  let permitted = found.len() == 1 && found.iter().all(|line| line.contains("command.spawn()"));
  assert!(permitted, "{}", Breach::Spawn { found });
  // …and `command` is the builder, so that occurrence really is `Command`'s
  // `spawn` and not some other receiver's.
  assert!(
    code
      .iter()
      .any(|line| line.text.contains("let mut command = Command::new(")),
    "`command` is not bound to a `Command`, so the check above proves nothing"
  );
}

/// No `?` between the spawn and the cleanup budget. Once a child exists, every
/// return must dispose of it; a `?` would hand that job to `kill_on_drop`,
/// which I13 keeps as a backstop for the paths where no code of ours runs at
/// all — not as the mechanism a returning path relies on (F-41, R-48).
#[test]
fn nothing_returns_between_the_spawn_and_the_cleanup_budget() {
  let region = TRANSPORT
    .region()
    .unwrap_or_else(|breaches| panic!("{}", report(&breaches)));
  let breaches: Vec<Breach> = region
    .iter()
    .filter(|line| line.text.contains('?'))
    .map(|line| Breach::QuestionMark { line: line.number })
    .collect();
  assert!(breaches.is_empty(), "{}", report(&breaches));
  assert!(
    region.len() > 1,
    "the region is one line long, so it is not the region"
  );
}

// ---------------------------------------------------------------------------
// The guard. Both checks above are greps, and a grep whose subject moved
// reports "clean" for a file it never opened.
// ---------------------------------------------------------------------------

const RENAMED_AWAY: Source = Source {
  path: "src/shell/backend/process-renamed.rs",
};

/// `src/shell/backend/process.rs` renamed, the check left pointing at where it
/// used to be.
#[test]
fn a_check_whose_subject_is_not_there_fails() {
  let breach = RENAMED_AWAY
    .code()
    .err()
    .expect("a check over a missing file must fail");
  assert!(matches!(breach, Breach::Unreadable { .. }), "{breach}");
}

/// The region's own guard: a file with no spawn in it yields no region rather
/// than an empty one that trivially contains no `?`.
#[test]
fn a_region_with_no_anchors_in_it_fails() {
  const NOT_THE_TRANSPORT: Source = Source {
    path: "src/shell/error.rs",
  };
  let breaches = NOT_THE_TRANSPORT
    .region()
    .err()
    .expect("a region without a spawn in it must fail");
  assert!(
    breaches
      .iter()
      .any(|breach| matches!(breach, Breach::NoAnchor { .. })),
    "{}",
    report(&breaches)
  );
}

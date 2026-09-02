//! The table-driven corpus runner, and the scheduling corpus PHASE-03 wrote it
//! for.
//!
//! `design.md` §9: fixtures are data files rather than Rust literals, so the
//! corpus is reviewable by someone who knows the protocol and not the tests.
//! `notes.md` PHASE-03, *The fixture format*, is the format's statement of
//! record and this file is what makes it true.
//!
//! **The envelope is shared and the payload is not.** `Envelope` owns
//! `requirement`, `description` and `now`, and file discovery is one walk; the
//! payload — `input` and `expect` — comes back untyped, and each corpus supplies
//! the closure that reads it. PHASE-04's corpus has a whole wire response for an
//! input and its own `expect` keys, and it can add both without touching
//! anything this phase's cases go through. That is why `expect` is not a closed
//! Rust enum here.
//!
//! The load-bearing part is not the matching, it is the vacuity guard, for the
//! reason `boundary.rs` states: a walk over a directory that has been renamed
//! away finds no failures, and reporting success for that has stopped testing
//! anything.
//!
//! PHASE-04 took the split *The fixture format* left open: its two corpora live
//! in `normalize.rs` and reach the shared half through `pub(crate)`. Nothing
//! above the divider changed to admit them, which is the property this seam was
//! built for.

use std::fmt;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use goad::semantics::error::ScheduleError;
use goad::semantics::protocol::canonical::Timestamp;
use goad::semantics::schedule::parse;

// ---------------------------------------------------------------------------
// The shared half: the envelope, discovery, and the guard
// ---------------------------------------------------------------------------

/// One fixture file. `deny_unknown_fields` because fixtures are ours: I10/R-4's
/// permissiveness is about the inbound *protocol*, where an unknown key is a
/// backend written against a newer host. A fixture is not a backend, so
/// strictness costs nothing and catches `expct` on the day it is typed.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
  /// The `R-N` ids this case verifies. `ls` over the directory is then a
  /// coverage report against `draft-spec.md` §4.
  requirement: Vec<String>,
  /// One sentence, present tense, about the protocol — not about the assertion.
  /// This is the half that makes the corpus documentation.
  description: String,
  /// RFC 3339 with an offset. Every case is deterministic; nothing reads a
  /// clock (I3).
  now: String,
  /// Corpus-specific and untyped, so `45` and `null` are expressible without a
  /// second key.
  input: serde_json::Value,
  /// A single-key object, externally tagged, so the key names the outcome kind.
  expect: serde_json::Value,
}

/// What a case hands its checker: the payload, plus the `now` the envelope
/// already parsed.
pub(crate) struct Fixture<'a> {
  pub(crate) now: Timestamp,
  pub(crate) input: &'a serde_json::Value,
  pub(crate) expect: &'a serde_json::Value,
}

/// Why a corpus run failed. Three kinds, and they are different questions:
/// nothing was inspected; a file is not a fixture; a fixture's claim is false.
#[derive(Debug)]
enum Fault {
  Vacuous {
    root: PathBuf,
  },
  /// The corpus could not be enumerated, or a file in it could not be read as
  /// a fixture at all. Distinct from `Mismatch`: nothing was asserted here, so
  /// there is no protocol claim to have broken.
  Malformed {
    path: PathBuf,
    reason: String,
  },
  /// The fixture was read, and the protocol did not do what it says.
  Mismatch {
    path: PathBuf,
    description: String,
    reason: String,
  },
}

impl fmt::Display for Fault {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Vacuous { root } => write!(
        f,
        "{}: ran no fixtures — renamed, emptied, or misspelled",
        root.display()
      ),
      Self::Malformed { path, reason } => write!(f, "{}: {reason}", path.display()),
      Self::Mismatch {
        path,
        description,
        reason,
      } => write!(f, "{}: {description}\n    {reason}", path.display()),
    }
  }
}

fn report(faults: &[Fault]) -> String {
  faults
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>()
    .join("\n")
}

/// What a corpus supplies: the reading of its own payload. `Ok(())` is the
/// protocol behaving as the fixture says; `Err` is the sentence a failure
/// report prints under the description.
pub(crate) type Check = fn(&Fixture<'_>) -> Result<(), String>;

/// A directory of fixtures and the checker that reads their payloads.
pub(crate) struct Corpus {
  /// Relative to the crate root.
  pub(crate) root: &'static str,
  pub(crate) check: Check,
}

impl Corpus {
  fn root(&self) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(self.root)
  }

  /// `Ok(n)` is the number of cases run. `Err` lists *every* fault, not the
  /// first, so one run names all the work.
  fn run(&self) -> Result<usize, Vec<Fault>> {
    let root = self.root();
    let mut faults = Vec::new();
    let paths = fixture_paths(&root, &mut faults);
    // The guard, and it counts what was *found*, not what passed. Counting
    // passes would make a corpus whose every case failed also claim to have run
    // nothing, which is a second and false accusation. Not `else` on the read
    // failure above: a directory can both fail to be read and hold no fixtures,
    // and the vacuity is the finding worth naming either way.
    if paths.is_empty() {
      faults.push(Fault::Vacuous { root });
    }
    for path in &paths {
      if let Err(fault) = self.run_case(path) {
        faults.push(fault);
      }
    }
    if faults.is_empty() {
      Ok(paths.len())
    } else {
      Err(faults)
    }
  }

  fn run_case(&self, path: &Path) -> Result<(), Fault> {
    let envelope = read_envelope(path)?;
    let now = envelope.now.parse::<jiff::Timestamp>().map(Timestamp::new);
    let Ok(now) = now else {
      return Err(Fault::Malformed {
        path: path.to_owned(),
        reason: format!(
          "not a fixture: `now` is not an RFC 3339 instant: {}",
          envelope.now
        ),
      });
    };
    let fixture = Fixture {
      now,
      input: &envelope.input,
      expect: &envelope.expect,
    };
    (self.check)(&fixture).map_err(|reason| Fault::Mismatch {
      path: path.to_owned(),
      description: envelope.description.clone(),
      reason,
    })
  }
}

/// One flat directory of `.json` files, sorted so a failure reads the same way
/// twice. Flat by design — the filename is the index.
fn fixture_paths(root: &Path, faults: &mut Vec<Fault>) -> Vec<PathBuf> {
  let entries = match std::fs::read_dir(root) {
    Ok(entries) => entries,
    Err(error) => {
      faults.push(Fault::Malformed {
        path: root.to_owned(),
        reason: format!("the corpus directory could not be read: {error}"),
      });
      return Vec::new();
    }
  };
  let mut paths = Vec::new();
  for entry in entries {
    match entry {
      Ok(entry) => {
        let path = entry.path();
        if path
          .extension()
          .is_some_and(|extension| extension == "json")
        {
          paths.push(path);
        }
      }
      Err(error) => faults.push(Fault::Malformed {
        path: root.to_owned(),
        reason: format!("a directory entry could not be read: {error}"),
      }),
    }
  }
  paths.sort();
  paths
}

fn read_envelope(path: &Path) -> Result<Envelope, Fault> {
  let text = std::fs::read_to_string(path).map_err(|error| Fault::Malformed {
    path: path.to_owned(),
    reason: format!("could not be read: {error}"),
  })?;
  let envelope: Envelope = serde_json::from_str(&text).map_err(|error| Fault::Malformed {
    path: path.to_owned(),
    reason: format!("not a fixture: {error}"),
  })?;
  // An empty `requirement` is a case nobody can justify, and it is exactly the
  // shape a copy-pasted fixture takes.
  if envelope.requirement.is_empty() {
    return Err(Fault::Malformed {
      path: path.to_owned(),
      reason: "not a fixture: `requirement` names no R-N id".to_owned(),
    });
  }
  Ok(envelope)
}

/// Read `expect`'s external tag: the single key naming the outcome kind, and
/// its value.
///
/// Shared rather than per-corpus, because the *tagging* is the format's and only
/// the tags are the corpus's. Rejecting a second key is the point: an `expect`
/// carrying both `instant` and `error` claims two things, and a checker that
/// reads whichever it looks for first would silently verify one of them.
pub(crate) fn outcome_tag(
  expect: &serde_json::Value,
) -> Result<(&str, &serde_json::Value), String> {
  let object = expect
    .as_object()
    .ok_or_else(|| "`expect` is not an object".to_owned())?;
  let mut entries = object.iter();
  let (Some((key, value)), None) = (entries.next(), entries.next()) else {
    return Err(format!(
      "`expect` must be an object of exactly one key naming the outcome kind, found {}",
      object.len()
    ));
  };
  Ok((key.as_str(), value))
}

/// Fails naming *every* fault, not the first. `run` cannot return `Ok(0)`, so
/// arriving at `Ok` at all is the vacuity guard discharging.
pub(crate) fn assert_corpus(corpus: &Corpus) {
  if let Err(faults) = corpus.run() {
    panic!("{}", report(&faults));
  }
}

// ---------------------------------------------------------------------------
// The scheduling corpus — VT-1, VT-3, EX-4
// ---------------------------------------------------------------------------

/// The variant name a fixture's `{"error": …}` names. An exhaustive match by
/// design: a sixth `ScheduleError` cannot be added without an arm here.
fn error_name(error: &ScheduleError) -> &'static str {
  match error {
    ScheduleError::NotAString { .. } => "NotAString",
    ScheduleError::MissingOffset { .. } => "MissingOffset",
    ScheduleError::CalendarUnit { .. } => "CalendarUnit",
    ScheduleError::OutOfRange { .. } => "OutOfRange",
    ScheduleError::Unparseable { .. } => "Unparseable",
  }
}

/// This corpus reads two outcome tags: `{"instant": "<RFC 3339>"}` and
/// `{"error": "<ScheduleError variant>"}`.
fn check_schedule(fixture: &Fixture<'_>) -> Result<(), String> {
  let outcome = parse(fixture.input, fixture.now);
  match outcome_tag(fixture.expect)? {
    ("instant", expected) => {
      let expected = expected
        .as_str()
        .ok_or_else(|| "`expect.instant` is not a string".to_owned())?
        .parse::<jiff::Timestamp>()
        .map_err(|error| format!("`expect.instant` is not an RFC 3339 instant: {error}"))?;
      match outcome {
        Ok(actual) if actual.instant() == expected => Ok(()),
        Ok(actual) => Err(format!("expected {expected}, got {}", actual.instant())),
        Err(error) => Err(format!(
          "expected {expected}, got {} ({error})",
          error_name(&error)
        )),
      }
    }
    ("error", expected) => {
      let expected = expected
        .as_str()
        .ok_or_else(|| "`expect.error` is not a string".to_owned())?;
      match outcome {
        Err(error) if error_name(&error) == expected => Ok(()),
        Err(error) => Err(format!(
          "expected {expected}, got {} ({error})",
          error_name(&error)
        )),
        Ok(actual) => Err(format!("expected {expected}, got {}", actual.instant())),
      }
    }
    (tag, _) => Err(format!(
      "`expect` names `{tag}`, which this corpus does not read"
    )),
  }
}

const SCHEDULE: Corpus = Corpus {
  root: "tests/protocol/fixtures/schedule",
  check: check_schedule,
};

#[test]
fn every_scheduling_fixture_states_what_the_protocol_does() {
  assert_corpus(&SCHEDULE);
}

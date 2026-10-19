//! The coverage test (`design.md` §5.2.6, *The reference's structure*).
//!
//! Every requirement a refusal can name, and the checker's own R-56 claim, is
//! explained in the reference under a heading of its own, so a report line
//! leads a reader straight to its explanation, and never outside the kit.
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use goad_semantics::error::{BoundsError, ProtocolError, Requirement, ScheduleError};
use goad_semantics::protocol::canonical::ViewId;
use goad_shell::error::{BackendError, CleanupFailure, StateError};

use crate::fences::fences;
use crate::tree;

/// What a report prints before a refusal's id (`render`'s `refusal_line`;
/// the binary tier's `a_line_names` cases hold that spelling). Every id a
/// refusal can name is SPEC-001's.
const SPEC: &str = "SPEC-001/";

// Each taxonomy below has a match naming every variant, with no `_` arm,
// beside a builder making one instance per variant in the match's order.
// The limit, as `design.md` §5.2.6 states it: a new variant fails to compile
// only at the match; whether its author then adds an instance beside the arm
// — two for `InapplicableKey`, one per id it answers — is review, not an
// assertion. Stable Rust cannot force it without a variant-enumerating
// derive, which `notes.md` §Open carries. The builders are a third copy of
// the private ones in `goad_semantics::error` and `goad_shell::error`, which
// a crate edge keeps out of reach.

fn protocol_variant(error: &ProtocolError) -> String {
  match error {
    ProtocolError::Json(_) => "ProtocolError::Json".to_owned(),
    ProtocolError::Shape(_) => "ProtocolError::Shape".to_owned(),
    ProtocolError::DuplicateKey { .. } => "ProtocolError::DuplicateKey".to_owned(),
    ProtocolError::NestedHints { .. } => "ProtocolError::NestedHints".to_owned(),
    ProtocolError::UnsupportedProtocolVersion { .. } => {
      "ProtocolError::UnsupportedProtocolVersion".to_owned()
    }
    ProtocolError::UnsupportedPrimitive { .. } => "ProtocolError::UnsupportedPrimitive".to_owned(),
    ProtocolError::InapplicableKey { key, .. } => format!("ProtocolError::InapplicableKey({key})"),
    ProtocolError::MissingField { .. } => "ProtocolError::MissingField".to_owned(),
    ProtocolError::EmptyOptions { .. } => "ProtocolError::EmptyOptions".to_owned(),
    ProtocolError::DuplicateOptionId { .. } => "ProtocolError::DuplicateOptionId".to_owned(),
    ProtocolError::DuplicateFieldId { .. } => "ProtocolError::DuplicateFieldId".to_owned(),
    ProtocolError::DuplicateAlternativeId { .. } => {
      "ProtocolError::DuplicateAlternativeId".to_owned()
    }
    ProtocolError::EmptyAlternatives { .. } => "ProtocolError::EmptyAlternatives".to_owned(),
    ProtocolError::Bounds(inner) => format!("ProtocolError::Bounds({})", bounds_variant(inner)),
    ProtocolError::Schedule(inner) => {
      format!("ProtocolError::Schedule({})", schedule_variant(inner))
    }
  }
}

/// `InapplicableKey` twice, once for each id it answers.
fn every_protocol_error() -> Vec<ProtocolError> {
  let at = || "$.view".to_owned();
  let mut every = vec![
    ProtocolError::Json(serde_json::from_str::<serde_json::Value>("{").expect_err("truncated")),
    ProtocolError::Shape(serde_json::from_str::<u32>("\"1\"").expect_err("a string")),
    ProtocolError::DuplicateKey {
      key: "view".to_owned(),
    },
    ProtocolError::NestedHints { at: at() },
    ProtocolError::UnsupportedProtocolVersion { found: 2 },
    ProtocolError::UnsupportedPrimitive {
      kind: "slider".to_owned(),
      at: at(),
    },
    ProtocolError::InapplicableKey {
      key: "min",
      kind: "text".to_owned(),
      at: at(),
    },
    ProtocolError::InapplicableKey {
      key: "fields",
      kind: "choice".to_owned(),
      at: at(),
    },
    ProtocolError::MissingField { field: "view" },
    ProtocolError::EmptyOptions { at: at() },
    ProtocolError::DuplicateOptionId {
      id: "now".to_owned(),
      at: at(),
    },
    ProtocolError::DuplicateFieldId {
      id: "note".to_owned(),
      at: at(),
    },
    ProtocolError::DuplicateAlternativeId {
      id: "up".to_owned(),
      at: at(),
    },
    ProtocolError::EmptyAlternatives { at: at() },
  ];
  every.extend(every_bounds_error().into_iter().map(ProtocolError::Bounds));
  every.extend(
    every_schedule_error()
      .into_iter()
      .map(ProtocolError::Schedule),
  );
  every
}

fn bounds_variant(error: &BoundsError) -> String {
  match error {
    BoundsError::NotFinite { .. } => "BoundsError::NotFinite".to_owned(),
    BoundsError::Inverted { .. } => "BoundsError::Inverted".to_owned(),
  }
}

fn every_bounds_error() -> Vec<BoundsError> {
  vec![
    BoundsError::NotFinite {
      bound: "min",
      found: f64::NAN,
    },
    BoundsError::Inverted { min: 2.0, max: 1.0 },
  ]
}

fn schedule_variant(error: &ScheduleError) -> String {
  match error {
    ScheduleError::NotAString { .. } => "ScheduleError::NotAString".to_owned(),
    ScheduleError::MissingOffset { .. } => "ScheduleError::MissingOffset".to_owned(),
    ScheduleError::TimeOfDay { .. } => "ScheduleError::TimeOfDay".to_owned(),
    ScheduleError::CalendarUnit { .. } => "ScheduleError::CalendarUnit".to_owned(),
    ScheduleError::OutOfRange { .. } => "ScheduleError::OutOfRange".to_owned(),
    ScheduleError::Unparseable { .. } => "ScheduleError::Unparseable".to_owned(),
  }
}

fn every_schedule_error() -> Vec<ScheduleError> {
  let raw = |raw: &str| raw.to_owned();
  vec![
    ScheduleError::NotAString { found: "number" },
    ScheduleError::MissingOffset {
      raw: raw("2026-08-23T18:00:00"),
    },
    ScheduleError::TimeOfDay { raw: raw("18:00") },
    ScheduleError::CalendarUnit {
      raw: raw("1 month"),
    },
    ScheduleError::OutOfRange {
      raw: raw("100000000 days"),
    },
    ScheduleError::Unparseable {
      raw: raw("tomorrow morning"),
    },
  ]
}

fn backend_variant(error: &BackendError) -> String {
  match error {
    BackendError::Spawn { .. } => "BackendError::Spawn".to_owned(),
    BackendError::Timeout { .. } => "BackendError::Timeout".to_owned(),
    BackendError::ExitStatus { .. } => "BackendError::ExitStatus".to_owned(),
    BackendError::OutputTooLarge { .. } => "BackendError::OutputTooLarge".to_owned(),
    BackendError::PipeMissing => "BackendError::PipeMissing".to_owned(),
    BackendError::Io(_) => "BackendError::Io".to_owned(),
    BackendError::Protocol(inner) => {
      format!("BackendError::Protocol({})", protocol_variant(inner))
    }
  }
}

fn every_backend_error() -> Vec<BackendError> {
  let mut every = vec![
    BackendError::Spawn {
      executable: "no-such-program".into(),
      working_directory: None,
      source: io::Error::other("no such program"),
    },
    BackendError::Timeout {
      after: Duration::from_secs(5),
    },
    BackendError::ExitStatus { code: Some(1) },
    BackendError::OutputTooLarge { limit: 1024 },
    BackendError::PipeMissing,
    BackendError::Io(io::Error::other("broken pipe")),
  ];
  every.extend(
    every_protocol_error()
      .into_iter()
      .map(BackendError::Protocol),
  );
  every
}

fn cleanup_variant(failure: &CleanupFailure) -> String {
  match failure {
    CleanupFailure::TimedOut { .. } => "CleanupFailure::TimedOut".to_owned(),
    CleanupFailure::Io(_) => "CleanupFailure::Io".to_owned(),
  }
}

fn every_cleanup_failure() -> Vec<CleanupFailure> {
  vec![
    CleanupFailure::TimedOut {
      after: Duration::from_secs(2),
    },
    CleanupFailure::Io(io::Error::other("wait failed")),
  ]
}

fn state_variant(error: &StateError) -> String {
  match error {
    StateError::NoOutstandingView { .. } => "StateError::NoOutstandingView".to_owned(),
    StateError::StaleViewId { .. } => "StateError::StaleViewId".to_owned(),
  }
}

fn every_state_error() -> Vec<StateError> {
  vec![
    StateError::NoOutstandingView {
      named: ViewId::new("v-1"),
    },
    StateError::StaleViewId {
      named: ViewId::new("v-1"),
      outstanding: ViewId::new("v-2"),
    },
  ]
}

/// Every id a report can print, as it prints it, each with what can make it
/// print that id: the variants answering it, and the checker's R-56 claim.
fn required() -> BTreeMap<String, BTreeSet<String>> {
  let mut required: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
  let mut add = |id: Requirement, by: String| {
    required
      .entry(format!("{SPEC}{id}"))
      .or_default()
      .insert(by);
  };
  for e in every_protocol_error() {
    add(e.requirement(), protocol_variant(&e));
  }
  for e in every_bounds_error() {
    add(e.requirement(), bounds_variant(&e));
  }
  for e in every_schedule_error() {
    add(e.requirement(), schedule_variant(&e));
  }
  for e in every_backend_error() {
    add(e.requirement(), backend_variant(&e));
  }
  for e in every_cleanup_failure() {
    add(e.requirement(), cleanup_variant(&e));
  }
  for e in every_state_error() {
    add(e.requirement(), state_variant(&e));
  }
  add(Requirement::R56, "the checker's R-56 claim".to_owned());
  required
}

/// The ids `markdown` gives a heading of their own: a heading line outside
/// every fence, naming exactly one requirement id, contributes that id.
fn heading_ids(markdown: &str) -> BTreeSet<String> {
  let fenced: Vec<_> = fences(markdown)
    .into_iter()
    .map(|fence| fence.body)
    .collect();
  let mut found = BTreeSet::new();
  let mut offset = 0;
  for line in markdown.split_inclusive('\n') {
    let start = offset;
    offset += line.len();
    if fenced.iter().any(|body| body.contains(&start)) || !is_heading(line) {
      continue;
    }
    if let [id] = ids(line).as_slice() {
      found.insert(id.clone());
    }
  }
  found
}

/// One to six `#` at the start of the line, then a space or the line's end.
/// An indented or setext heading is not counted: the reference uses neither,
/// and missing one reds the coverage test rather than passing it.
fn is_heading(line: &str) -> bool {
  let line = line.trim_end_matches(['\n', '\r']);
  let rest = line.trim_start_matches('#');
  (1..=6).contains(&(line.len() - rest.len())) && (rest.is_empty() || rest.starts_with(' '))
}

/// Every requirement id in `text`: `SPEC-`, digits, `/R-`, digits, each
/// digit run read whole. So `SPEC-001/R-32` is one id, and `SPEC-001/R-3` is
/// never found inside it.
fn ids(text: &str) -> Vec<String> {
  let digits = |s: &str| s.bytes().take_while(u8::is_ascii_digit).count();
  let mut found = Vec::new();
  for (at, _) in text.match_indices("SPEC-") {
    let spec = "SPEC-".len();
    let spec_digits = digits(&text[at + spec..]);
    let after = at + spec + spec_digits;
    if spec_digits > 0
      && let Some(rest) = text[after..].strip_prefix("/R-")
      && digits(rest) > 0
    {
      found.push(text[at..after + "/R-".len() + digits(rest)].to_owned());
    }
  }
  found
}

/// A heading of its own, a heading naming no id or two, body text, and a
/// heading inside a fence.
const CONTROL: &str = "\
# Control

### SPEC-001/R-32 — alone in a heading

Body text names SPEC-001/R-40.

```text
### SPEC-001/R-36 — a heading line inside a fence
```

### SPEC-001/R-41 and SPEC-001/R-42 — two ids in one heading
";

#[test]
fn a_requirement_id_counts_only_alone_in_a_heading_outside_a_fence() {
  let found = heading_ids(CONTROL);
  let wrong: Vec<String> = [
    ("SPEC-001/R-32", true, "alone in a heading"),
    ("SPEC-001/R-3", false, "absent, but a prefix of R-32"),
    ("SPEC-001/R-40", false, "in body text only"),
    ("SPEC-001/R-36", false, "in a fence only"),
    ("SPEC-001/R-41", false, "in a heading with another id"),
    ("SPEC-001/R-42", false, "in a heading with another id"),
  ]
  .into_iter()
  .filter(|(id, counts, _)| found.contains(*id) != *counts)
  .map(|(id, counts, place)| {
    let verdict = if counts { "not found" } else { "found" };
    format!("{id} ({place}) was {verdict}")
  })
  .collect();
  assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn every_requirement_a_refusal_can_name_is_explained_in_the_reference() {
  let dir = tree::root().join("skills/goad-backend/reference");
  let documents: Vec<PathBuf> = tree::files(&dir)
    .into_iter()
    .filter(|file| file.extension().is_some_and(|extension| extension == "md"))
    .collect();
  assert!(
    !documents.is_empty(),
    "reference/ holds no Markdown: {}",
    dir.display()
  );
  let found: BTreeSet<String> = documents
    .iter()
    .flat_map(|document| heading_ids(&tree::text(document)))
    .collect();
  let missing: Vec<String> = required()
    .into_iter()
    .filter(|(id, _)| !found.contains(id))
    .map(|(id, by)| {
      let by: Vec<String> = by.into_iter().collect();
      format!(
        "{id} ({}) has no heading of its own under reference/",
        by.join(", ")
      )
    })
    .collect();
  assert!(missing.is_empty(), "{}", missing.join("\n"));
}

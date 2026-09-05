//! What a person did, and why an evaluation was asked for — design.md §5.3.
//!
//! `Wire` and `Cancel`, the callback-facing halves of this module, are
//! PHASE-07's: they need the `mpsc` channel and the event loop this phase
//! does not build. This file carries only what PHASE-06's controller and
//! failure-case table need to compile against: the vocabulary a callback
//! would enqueue, and the envelope an evaluation stimulus builds.

use goad_semantics::protocol::canonical::{Event, Timestamp};
use serde_json::Value;

/// What a person did. There is deliberately **no** `Shutdown` variant:
/// stopping is a decision, not a queue position, and it travels out of band
/// (design.md §5.4, F-4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
  Evaluate(Stimulus),
  /// Both strings are opaque **selectors**, matched against retained state
  /// and never parsed back into a value. `view` is the `ViewId` the markup
  /// was given; without it a delayed click answers whichever interaction
  /// happens to be outstanding when it is dequeued (F-13).
  Choose {
    view: String,
    option: String,
  },
  OpenDiagnostics,
  CloseDiagnostics,
}

/// Why an evaluation is being asked for. The variants are the `Event.kind`
/// strings one for one (design.md §6, OQ-7): `"startup"`, `"requested"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stimulus {
  Startup,
  Requested,
}

impl Stimulus {
  /// `"startup"` | `"requested"`. The host's own vocabulary, naming a
  /// stimulus and never a domain.
  #[must_use]
  pub fn kind(self) -> &'static str {
    match self {
      Self::Startup => "startup",
      Self::Requested => "requested",
    }
  }

  /// The whole envelope, so the loop builds no `Event` by hand: every field
  /// is `pub` (`canonical.rs:490-497`), so no accessor is owed.
  #[must_use]
  pub fn event(self, now: Timestamp) -> Event {
    Event {
      source: "host".to_owned(),
      kind: self.kind().to_owned(),
      timestamp: now,
      data: Value::Null,
    }
  }
}

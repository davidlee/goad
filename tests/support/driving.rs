//! The host-driving half of slice 001's test helpers, shared by every tier that
//! drives a `Host` (`design.md` §12.8).
//!
//! It is not a crate and has no manifest: each consumer's `main.rs` includes it
//! with `#[path = "../../../../tests/support/driving.rs"]`, four levels up from
//! `crates/<member>/tests/<target>/`, which is the repository root — uniform
//! because every member sits at depth two. Nothing outside such an include may
//! name it. Paths it resolves are `CARGO_MANIFEST_DIR` joined with `../../`, for
//! the same reason: a test binary's working directory is not something to rely
//! on.
//!
//! The cut is the intersection of what the tiers use. At PHASE-01 the only
//! consumer is `goad-shell`'s `integration` target, and PHASE-06 re-settles the
//! boundary when the renderer's target includes the same file (PL-4).

use std::time::Duration;

use goad_semantics::protocol::canonical::{Choice, Event, Timestamp, UserResponse, View, ViewId};
use goad_shell::backend::process::ProcessBackend;
use goad_shell::config::{BackendConfig, Command, Config, ScheduleConfig};
use goad_shell::host::{Host, Outcome};

/// The transport's own cleanup budget, restated because it is private to
/// `process.rs` and the bounds that read it are about it. If it changes there,
/// the assertions that use this one are wrong until this does too.
///
/// Stated **once**, here, rather than a second time per tier: two of three
/// statements of one number are what nothing updates (D23, applied to a test).
pub(crate) const CLEANUP_LIMIT: Duration = Duration::from_millis(500);

// ---------------------------------------------------------------------------
// A whole host over the real transport, and the invocation witness
// ---------------------------------------------------------------------------

/// The default poll every host here is seeded with, so a case that asserts a
/// `next_check` has one number to reason about.
pub(crate) const DEFAULT_POLL: jiff::SignedDuration = jiff::SignedDuration::from_mins(30);

/// A `Config` built around one command.
///
/// Constructed rather than parsed: `Config`'s fields are `pub` and the TOML
/// route would mean quoting an absolute path into a document, which is a
/// property of the grammar `config.rs`'s own tests already hold. Nothing here
/// is about configuration parsing.
pub(crate) fn config(command: Command, timeout: Duration) -> Config {
  Config {
    backend: BackendConfig { command, timeout },
    schedule: ScheduleConfig {
      default_poll: DEFAULT_POLL,
    },
  }
}

/// A host over the **real** process transport, pointed at one command.
///
/// This is the composition stratum 3 will perform: the transport is built from
/// the configuration's own command and timeout, so a case cannot accidentally
/// point the two at different backends. Returned by value and driven through as
/// many exchanges as a case likes — `evaluate` and `respond` take `&mut self`
/// (I6), so a sequence is sequential by construction and PHASE-10/EX-2's
/// one-host requirement needs nothing further.
pub(crate) fn host(command: Command, timeout: Duration, now: Timestamp) -> Host<ProcessBackend> {
  host_from(config(command, timeout), now)
}

/// The same composition from a `Config` that came from somewhere else — a file
/// a reader would copy, for instance (F-16).
pub(crate) fn host_from(config: Config, now: Timestamp) -> Host<ProcessBackend> {
  let backend = ProcessBackend::new(config.backend.command.clone(), config.backend.timeout);
  Host::new(config, backend, now)
}

/// An event the example backend has nothing to say about.
///
/// `data` is opaque to the host (R-9) and is where these two events differ.
/// Both scripted backends read the same key, and `answers-a-round-trip.sh`
/// matches these exact two values — it has no JSON parser, so a third value is
/// a broken fixture and it says so on stderr.
pub(crate) fn quiet_event(now: Timestamp) -> Event {
  event(now, 0)
}

/// The body both events share. `pub(crate)` because `prompting_event` stays
/// with the transport tier and calls across (PHASE-01/EX-7).
pub(crate) fn event(now: Timestamp, minutes_since_entry: u32) -> Event {
  Event {
    source: "test".to_owned(),
    kind: "scheduled".to_owned(),
    timestamp: now,
    data: serde_json::json!({ "minutes_since_entry": minutes_since_entry }),
  }
}

// ---------------------------------------------------------------------------
// What a host answered, as a sentence a panic can carry
// ---------------------------------------------------------------------------

/// An instant from its RFC 3339 spelling, for a fixture that states one.
pub(crate) fn instant(rfc3339: &str) -> Timestamp {
  Timestamp::new(rfc3339.parse().expect("the fixture must be an instant"))
}

/// The two sentences `harness.rs`'s `describe_outcome` and this file's own
/// panics both need to say — stated once, here, rather than twice (F-8,
/// review-code 002 round 1). `describe_outcome` itself stays in
/// `harness.rs`, moved there at PHASE-06 (§12.8's cut re-settled) for a
/// third arm ("a view carrying {id}") the `renderer` target's `table.rs`
/// never needs — but the two arms below are needed by both, so they belong
/// with the file both already include rather than with the tier that added
/// a third.
pub(crate) fn failure_or_nothing(outcome: &Outcome) -> String {
  match &outcome.failure {
    Some(failure) => format!("a failure: {failure}"),
    None => "nothing to show, and no failure".to_owned(),
  }
}

/// The choice a backend returned, or a diagnostic naming what came instead.
///
/// `View` has one variant today, so this is a projection rather than a match —
/// but it is the projection every case that reads a view needs, and the
/// exhaustive `let` is what will point at them all when a second kind arrives.
pub(crate) fn choice(outcome: &Outcome) -> &Choice {
  match &outcome.view {
    Some(presented) => {
      let View::Choice(choice) = &presented.view;
      choice
    }
    None => panic!("expected a view; got {}", failure_or_nothing(outcome)),
  }
}

/// An answer naming an option of the view just presented, with a value for
/// whichever field that option carried.
///
/// `OptionId` and `FieldId` have no public constructor (D30, I15), so an answer
/// is assembled out of the view it answers — which is what a renderer does.
pub(crate) fn answer_first_option(outcome: &Outcome) -> UserResponse {
  let option = choice(outcome)
    .options()
    .as_slice()
    .first()
    .expect("a choice carries at least one option");
  let values = option
    .fields()
    .as_slice()
    .iter()
    .map(|field| {
      (
        field.id().clone(),
        serde_json::json!("whatever the user typed"),
      )
    })
    .collect();
  UserResponse {
    option: option.id().clone(),
    values,
  }
}

/// The id of the view an outcome carries, or a panic naming what came instead.
pub(crate) fn presented(outcome: &Outcome) -> &ViewId {
  match &outcome.view {
    Some(presented) => &presented.view_id,
    None => panic!("expected a view; got {}", failure_or_nothing(outcome)),
  }
}

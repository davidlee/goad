//! The request plan and what the run decides from what the host reported.
//!
//! Pure, as [`crate::render`] is: no stream, no clock, no process. `main`
//! drives the exchanges this module plans, and writes the lines `render`
//! words (`design.md` §5.2.2).

use goad_semantics::error::AtFault;
use goad_semantics::protocol::canonical::{
  Event, HOST_SOURCE, Stimulus, Submitted, Timestamp, UserResponse, View,
};
use goad_semantics::protocol::normalize::Discarded;
use goad_shell::backend::transport::Captured;
use goad_shell::error::{BackendError, CleanupFailure, StateError};
use goad_shell::host::{Failure, Outcome, Presented};

/// The R-56 probe's kind: one no host originates, so a backend that fails on
/// it fails on a kind it does not recognise (SPEC-001/R-56).
pub(crate) const PROBE_KIND: &str = "goad-check-unrecognised";

/// How many responds one planned request's chain may take. A view still
/// returned by the last is left unanswered and reported as an observation,
/// not a refusal (`design.md` §5.2.2).
pub(crate) const CHAIN_BOUND: usize = 8;

/// One request of the plan.
pub(crate) enum Planned {
  /// One of the host's own kinds, which the R-56 condition reads.
  Host(Stimulus),
  /// The R-56 probe.
  Probe,
  /// An `--event` file's envelope.
  Given(Event),
}

impl Planned {
  /// The event this request evaluates. The host's own kinds and the probe
  /// carry the host's source, `now`, and no data, as `Stimulus::event` builds
  /// them; a given envelope is sent as it was normalized.
  #[expect(
    clippy::default_trait_access,
    reason = "the probe's `data` is `null`; spelling it `Value::default()` names `serde_json`, \
              which is not a dependency of this crate and may not become one"
  )]
  pub(crate) fn event(&self, now: Timestamp) -> Event {
    match self {
      Self::Host(stimulus) => stimulus.event(now),
      Self::Probe => Event {
        source: HOST_SOURCE.to_owned(),
        kind: PROBE_KIND.to_owned(),
        timestamp: now,
        data: Default::default(),
      },
      Self::Given(event) => event.clone(),
    }
  }
}

/// The request plan, in `design.md` §5.2.2's order: the host's own kinds, the
/// probe, then each given envelope in the order given.
pub(crate) fn plan(given: Vec<Event>) -> impl Iterator<Item = Planned> {
  [
    Planned::Host(Stimulus::Startup),
    Planned::Host(Stimulus::Requested),
    Planned::Host(Stimulus::Scheduled),
    Planned::Probe,
  ]
  .into_iter()
  .chain(given.into_iter().map(Planned::Given))
}

/// The answer to a view: its first option, with each of that option's
/// fields as nobody touched it (`Submitted::as_drawn`), so the values are
/// the host's and none is chosen here.
pub(crate) fn answer(view: &View) -> UserResponse {
  match view {
    View::Choice(choice) => {
      let option = choice.options().first();
      UserResponse {
        option: option.id().clone(),
        values: option
          .fields()
          .as_slice()
          .iter()
          .map(|field| {
            (
              field.id().clone(),
              Submitted::as_drawn(field.kind()).to_json(),
            )
          })
          .collect(),
      }
    }
  }
}

/// An `Outcome` the run goes on from: its failure, if any, is about the
/// backend's exchange. `Failure::State` is not; it is the checker's own
/// defect, and ends the run (`design.md` §5.2.2).
pub(crate) struct Judged {
  pub(crate) view: Option<Presented>,
  pub(crate) next_check: Timestamp,
  pub(crate) discarded: Vec<Discarded>,
  pub(crate) stderr: Captured,
  pub(crate) failure: Option<BackendError>,
  pub(crate) cleanup: Option<CleanupFailure>,
}

impl Judged {
  /// Whether the host reported a refusal on any channel: a failure, a
  /// discarded instruction or a cleanup failure (`design.md` §5.2.5).
  pub(crate) fn refused(&self) -> bool {
    self.failure.is_some() || !self.discarded.is_empty() || self.cleanup.is_some()
  }
}

/// The outcome, unless its failure is `Failure::State`.
///
/// # Errors
///
/// The `StateError`, which no cooperating run reaches: the checker answers
/// only with the `view_id` the host minted.
pub(crate) fn judged(outcome: Outcome) -> Result<Judged, StateError> {
  let Outcome {
    view,
    next_check,
    discarded,
    stderr,
    failure,
    cleanup,
  } = outcome;
  let failure = match failure {
    None => None,
    Some(Failure::Backend(error)) => Some(error),
    Some(Failure::State(error)) => return Err(error),
  };
  Ok(Judged {
    view,
    next_check,
    discarded,
    stderr,
    failure,
    cleanup,
  })
}

/// R-56's condition (`design.md` §5.2.2): the probe's evaluate failed on the
/// backend's side, and at least one of the host's own kinds' evaluates made
/// no failure. A backend that fails alike on every kind, or a failure on
/// another side, is not charged. Only evaluates are read: a respond in any
/// chain is judged as any other exchange.
pub(crate) fn charges_the_probe(probe: &Judged, a_host_kind_made_no_failure: bool) -> bool {
  a_host_kind_made_no_failure
    && probe
      .failure
      .as_ref()
      .is_some_and(|failure| failure.fault() == AtFault::Backend)
}

/// What the verdict is cut from.
#[derive(Default)]
pub(crate) struct Tally {
  /// How many exchanges the host reported a refusal on.
  pub(crate) refused: usize,
  /// Whether any exchange returned a view, so respond was exercised.
  pub(crate) viewed: bool,
}

impl Tally {
  pub(crate) fn record(&mut self, judged: &Judged) {
    self.viewed |= judged.view.is_some();
    if judged.refused() {
      self.refused = self.refused.saturating_add(1);
    }
  }
}

#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::Stimulus;

  use super::PROBE_KIND;

  /// Every `Stimulus`. The `match` has no `_` arm, so a new variant fails to
  /// compile here until it is listed, rather than going unchecked.
  fn every_stimulus() -> [Stimulus; 3] {
    match Stimulus::Startup {
      Stimulus::Startup | Stimulus::Requested | Stimulus::Scheduled => (),
    }
    [Stimulus::Startup, Stimulus::Requested, Stimulus::Scheduled]
  }

  #[test]
  fn the_probe_kind_is_none_of_the_host_s_own() {
    for stimulus in every_stimulus() {
      assert_ne!(PROBE_KIND, stimulus.kind(), "{stimulus:?}");
    }
  }
}

//! What the host is holding between exchanges — `design.md` §5.3.
//!
//! Nothing is written to disk (the OQ-6 decision), so the state space is one
//! process lifetime wide. A plain struct behind `&mut self`, not an
//! `Arc<Mutex<…>>`: brief §12 serializes backend exchanges and allows one
//! outstanding interaction, so there is no concurrency to protect against and a
//! lock would invent a state space the brief says to avoid (D14).
//!
//! No module-level `#![deny(clippy::arithmetic_side_effects)]`. The lint follows
//! the data (D53 as amended), and everything here is host-authored: a `view_id`
//! the host minted, an instant the caller passed in, and a counter the host
//! owns.

use crate::semantics::protocol::canonical::{Timestamp, ViewId};
use crate::shell::error::StateError;

/// The interaction the host is waiting for an answer to, if there is one.
///
/// `Option`, and that is I5: at most one is outstanding, and a queue is the
/// general concurrency semantics brief §12 says not to introduce (R-31).
#[derive(Debug)]
pub struct State {
  outstanding: Option<Outstanding>,
  resolved_check: Timestamp,
  next_seq: u64,
}

/// `design.md:1167`, as written.
#[derive(Debug)]
struct Outstanding {
  view_id: ViewId,
  /// Not read anywhere in slice 001: no `Outcome` field carries it, no
  /// diagnostic names it, and no criterion asks for it. Kept because
  /// `design.md:1167` and PHASE-07/EX-2 both name it, and dropping it would be
  /// a design change made to satisfy a lint — user decision 2026-09-03,
  /// `plan-log.md`. The expectation self-clears via
  /// `unfulfilled_lint_expectations` the moment something reads it, so this is
  /// a recorded gap rather than a silent one.
  #[expect(
    dead_code,
    reason = "design.md:1167 names it and nothing in slice 001 reads it yet; \
              the expectation clears itself when a reader appears"
  )]
  issued_at: Timestamp,
}

impl State {
  /// Seed the state at construction.
  ///
  /// `resolved_check` is **not** an `Option`: brief §9 resolves to a concrete
  /// instant in every case, so there is no unresolved state to represent (I4,
  /// R-27). Seeding runs through `schedule::resolve`'s `(None, None)` arm rather
  /// than adding `now + default_poll` a second time here.
  pub fn new(resolved_check: Timestamp) -> Self {
    Self {
      outstanding: None,
      resolved_check,
      next_seq: 0,
    }
  }

  /// The instant the host will next ask the backend for something.
  pub fn resolved_check(&self) -> Timestamp {
    self.resolved_check
  }

  /// Move the schedule. Only a *successful* exchange may call this: every
  /// failure path leaves the resolved check exactly as it was (R-29, P2).
  pub fn resolve_to(&mut self, instant: Timestamp) {
    self.resolved_check = instant;
  }

  /// Mint a `view_id` and make it the outstanding interaction.
  ///
  /// A returned view **replaces** any interaction already outstanding, and the
  /// replaced id becomes stale immediately (R-33). Replacement rather than
  /// queueing is the only reading of brief §12 that does not require a queue.
  ///
  /// The value is `{now, RFC 3339}#{seq}` — D13, and jiff's `Timestamp` displays
  /// as RFC 3339 already, so the format string *is* the specification. Four
  /// reasons over a UUID, of which the third is why this is worth a test: a
  /// fixed `now` and a fixed counter produce a fixed id, so a fixture can assert
  /// the exact value rather than capture whatever came out.
  pub fn issue(&mut self, now: Timestamp) -> ViewId {
    let minted = ViewId::new(format!("{}#{}", now.instant(), self.next_seq));
    self.next_seq = self.next_seq.saturating_add(1);
    self.outstanding = Some(Outstanding {
      view_id: minted.clone(),
      issued_at: now,
    });
    minted
  }

  /// Close the outstanding interaction. Idempotent.
  pub fn close(&mut self) {
    self.outstanding = None;
  }

  /// Is this the interaction the host is holding?
  ///
  /// Called *before* the transport is touched. A stale answer must not reach the
  /// backend at all: forwarding it would make every backend author responsible
  /// for ordering, which is what brief §12 puts in the host (R-32,
  /// `design.md:1600`).
  ///
  /// # Errors
  ///
  /// `NoOutstandingView` when nothing is open, `StaleViewId` when something else
  /// is — two variants because they are different mistakes with different fixes
  /// (D24). Neither changes any state: rejecting an answer leaves the
  /// outstanding interaction intact (R-34).
  pub fn verify(&self, named: &ViewId) -> Result<(), StateError> {
    match &self.outstanding {
      None => Err(StateError::NoOutstandingView {
        named: named.clone(),
      }),
      Some(outstanding) if &outstanding.view_id == named => Ok(()),
      Some(outstanding) => Err(StateError::StaleViewId {
        named: named.clone(),
        outstanding: outstanding.view_id.clone(),
      }),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::State;
  use crate::semantics::protocol::canonical::{Timestamp, ViewId};
  use crate::shell::error::StateError;

  fn instant(rfc3339: &str) -> Timestamp {
    Timestamp::new(rfc3339.parse().expect("the fixture must be an instant"))
  }

  fn seeded() -> State {
    State::new(instant("2026-08-23T04:12:00Z"))
  }

  // ---- VT-5: `view_id` determinism (D13's third reason) ----

  #[test]
  fn a_fixed_now_and_counter_produce_the_id_the_design_documents() {
    let mut state = seeded();
    let now = instant("2026-08-23T04:12:00Z");
    assert_eq!(state.issue(now).as_str(), "2026-08-23T04:12:00Z#0");
    assert_eq!(state.issue(now).as_str(), "2026-08-23T04:12:00Z#1");
    assert_eq!(state.issue(now).as_str(), "2026-08-23T04:12:00Z#2");
    // `design.md:1216`'s own worked example, reached by the counter alone.
    assert_eq!(state.issue(now).as_str(), "2026-08-23T04:12:00Z#3");
  }

  #[test]
  fn the_counter_separates_ids_minted_from_the_same_instant_and_now_separates_restarts() {
    let mut first = seeded();
    let mut second = seeded();
    let now = instant("2026-08-23T04:12:00Z");
    assert_ne!(first.issue(now), first.issue(now), "the counter must move");
    assert_eq!(
      second.issue(now),
      ViewId::new("2026-08-23T04:12:00Z#0"),
      "a fresh state restarts the counter, and only `now` separates the ids"
    );
  }

  // ---- EX-2 and EX-3 at the level that owns them ----

  #[test]
  fn a_minted_id_is_outstanding_and_verifies() {
    let mut state = seeded();
    let issued = state.issue(instant("2026-08-23T04:12:00Z"));
    assert!(state.verify(&issued).is_ok());
  }

  #[test]
  fn nothing_outstanding_names_the_variant_that_says_so() {
    let state = seeded();
    let named = ViewId::new("2026-08-23T04:12:00Z#0");
    match state.verify(&named) {
      Err(StateError::NoOutstandingView { named: reported }) => assert_eq!(reported, named),
      other => panic!("an answer against an idle host was not refused as such: {other:?}"),
    }
  }

  #[test]
  fn a_replaced_id_is_stale_and_the_replacement_is_named() {
    let mut state = seeded();
    let now = instant("2026-08-23T04:12:00Z");
    let first = state.issue(now);
    let second = state.issue(now);
    match state.verify(&first) {
      Err(StateError::StaleViewId { named, outstanding }) => {
        assert_eq!(named, first);
        assert_eq!(outstanding, second, "the diagnostic must name the live id");
      }
      other => panic!("a superseded id was not refused as stale: {other:?}"),
    }
  }

  #[test]
  fn a_rejection_leaves_the_outstanding_interaction_intact() {
    let mut state = seeded();
    let live = state.issue(instant("2026-08-23T04:12:00Z"));
    assert!(
      state
        .verify(&ViewId::new("2026-08-23T04:12:00Z#99"))
        .is_err(),
      "the arrangement is a rejection; without one this test asserts nothing"
    );
    assert!(
      state.verify(&live).is_ok(),
      "R-34: refusing an answer must not close the interaction it was not for"
    );
  }

  #[test]
  fn closing_returns_the_host_to_idle_and_is_idempotent() {
    let mut state = seeded();
    let issued = state.issue(instant("2026-08-23T04:12:00Z"));
    state.close();
    state.close();
    assert!(matches!(
      state.verify(&issued),
      Err(StateError::NoOutstandingView { .. })
    ));
  }
}

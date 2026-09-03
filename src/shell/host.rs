//! The composition point — `design.md` §5.2's Host block and §5.4's sequence.
//!
//! Transport, then `serde_json::from_slice`, then `normalize_response`, then
//! schedule resolution and the state update. Everything below the entry points
//! is one of those four steps or the answer to one of them failing.
//!
//! This module carries the arithmetic deny because it is where a backend's bytes
//! are parsed, normalized and composed into an `Outcome` — it handles
//! backend-derived data throughout, which is what I9 and R-46 place the lint on
//! (D53 as amended, plan review finding F-2). `config.rs` and `state.rs` do not:
//! a config file is the user's own and a `view_id` is host-minted, and the rule
//! is about the data rather than the directory.
#![deny(clippy::arithmetic_side_effects)]

use crate::semantics::error::ProtocolError;
use crate::semantics::protocol::canonical::{
  Evaluate, Event, Request, Respond, Response, Timestamp, UserResponse, View, ViewId,
};
use crate::semantics::protocol::normalize::{Discarded, Normalized, normalize_response};
use crate::semantics::protocol::wire::WireResponse;
use crate::semantics::schedule;
use crate::shell::backend::transport::{Backend, Captured, Exchange};
use crate::shell::config::Config;
use crate::shell::error::{BackendError, CleanupFailure, StateError};
use crate::shell::state::State;

/// A view and the identity minted for it, inseparable by construction.
///
/// F-23: an earlier draft kept the id private in `State`, which left a renderer
/// holding a view it had no way to answer. Pairing them rather than adding a
/// second `Option<ViewId>` field is the same move as `Options` and
/// `NumberRange` — the invalid combination is not representable, so no caller
/// has to check for it (I14, D32).
#[derive(Debug)]
pub struct Presented {
  pub view_id: ViewId,
  pub view: View,
}

/// Why the host took no action on an exchange.
///
/// Two strata, kept apart: `Backend` is what the backend or the wire did, and
/// `State` is a refusal that never reached a backend at all.
#[derive(Debug)]
pub enum Failure {
  Backend(BackendError),
  State(StateError),
}

/// What one call produced.
///
/// A struct with a `failure` field rather than a `Result<Success, Failure>`
/// because **every** call resolves a `next_check`, failed ones included: brief
/// §9's fallback is not conditional on the exchange working, and a `Result`
/// would have put that instant on the success side and made the caller
/// reconstruct it on the error path (D23, I12).
#[derive(Debug)]
pub struct Outcome {
  /// `Some` = render this, and answer it with the `view_id` inside. `None` =
  /// nothing to show: either the backend's explicit `view: null` or a failed
  /// exchange, and `failure` says which.
  pub view: Option<Presented>,
  /// Always concrete — brief §9 resolves in every case, including failure.
  pub next_check: Timestamp,
  /// Parts the message lost without losing the message. P2's discard list.
  pub discarded: Vec<Discarded>,
  /// Whatever the backend wrote to stderr, whether or not the exchange worked
  /// (R-42).
  pub stderr: Captured,
  /// `Some` = the **host** took no action on this exchange beyond reporting it.
  ///
  /// It does **not** mean nothing happened. Brief §8.3 lets a backend perform
  /// arbitrary side effects while handling a response and brief §14 gives it the
  /// user's own authority, so a backend can write a file, send a message, and
  /// *then* time out. A failure is a statement about the host's own state, never
  /// about the backend's effects, and nothing may treat it as one (F-32, R-49).
  /// It is also why there is no retry: the host cannot know what a failed
  /// exchange already did.
  pub failure: Option<Failure>,
  /// `Some` = the host could not establish that it disposed of the backend
  /// process. A *host* condition, orthogonal to `failure`, which is the
  /// *backend's* outcome (F-48, F-53).
  pub cleanup: Option<CleanupFailure>,
}

/// What a `view: null` means for the outstanding interaction.
///
/// The one place `evaluate` and `respond` differ once the bytes are in, and it
/// is F-46: answering an `evaluate` with nothing means the backend was asked
/// whether it had anything *new*, not whether the open question still stands, so
/// the interaction survives. Answering an accepted `respond` with nothing means
/// the answer was taken and there is nothing further to show, so it closes.
#[derive(Clone, Copy)]
enum WhenNothingToShow {
  LeaveOutstanding,
  CloseOutstanding,
}

/// The host: one transport, one configuration, one piece of state.
#[derive(Debug)]
pub struct Host<B: Backend> {
  backend: B,
  config: Config,
  state: State,
}

impl<B: Backend> Host<B> {
  /// Seed the host at construction.
  ///
  /// `now` is a parameter here as it is everywhere: whoever owns real time is
  /// slice 003's timer, and this keeps the clock out of the slice entirely (I3).
  /// Seeding goes through `schedule::resolve`'s `(None, None)` arm so that
  /// `now + default_poll` is written once, in stratum 1, rather than a second
  /// time here.
  pub fn new(config: Config, backend: B, now: Timestamp) -> Self {
    let seed = schedule::resolve(None, None, config.schedule.default_poll, now);
    Self {
      backend,
      config,
      state: State::new(seed),
    }
  }

  /// Ask the backend whether an event calls for anything.
  pub async fn evaluate(&mut self, now: Timestamp, event: Event) -> Outcome {
    let request = Request::Evaluate(Evaluate { now, event });
    self
      .exchange(now, &request, WhenNothingToShow::LeaveOutstanding)
      .await
  }

  /// Answer the outstanding interaction.
  ///
  /// The `view_id` is checked against host state **before** the transport is
  /// touched: a stale answer must not reach the backend at all, because
  /// forwarding it would make every backend author responsible for ordering,
  /// which is what brief §12 puts in the host (R-32, `design.md:1600`). Nothing
  /// else about the answer is checked — whether it is acceptable is the
  /// backend's judgement, and field values pass through opaque (R-35).
  pub async fn respond(
    &mut self,
    now: Timestamp,
    view_id: ViewId,
    answer: UserResponse,
  ) -> Outcome {
    if let Err(refusal) = self.state.verify(&view_id) {
      // No backend was consulted, so there is no stderr to carry and nothing to
      // have disposed of — and the schedule does not move, exactly as it does
      // not for a failed exchange (R-29, R-34).
      return self.no_action(Failure::State(refusal), Captured::default(), None);
    }
    let request = Request::Respond(Respond {
      view_id,
      now,
      response: answer,
    });
    self
      .exchange(now, &request, WhenNothingToShow::CloseOutstanding)
      .await
  }

  /// One exchange, and the four steps that follow it.
  ///
  /// `evaluate` and `respond` share this whole: they differ in the request they
  /// build and in what an absent view means, and nothing else. Two bodies would
  /// be two places for the schedule rule and the failure rule to drift apart.
  async fn exchange(
    &mut self,
    now: Timestamp,
    request: &Request,
    nothing_to_show: WhenNothingToShow,
  ) -> Outcome {
    let Exchange {
      result,
      stderr,
      cleanup,
    } = self.backend.exchange(request).await;

    let bytes = match result {
      Ok(bytes) => bytes,
      Err(error) => return self.no_action(Failure::Backend(error), stderr, cleanup),
    };

    let normalized = match read(&bytes, now) {
      Ok(normalized) => normalized,
      Err(error) => {
        return self.no_action(
          Failure::Backend(BackendError::Protocol(error)),
          stderr,
          cleanup,
        );
      }
    };

    self.accept(now, normalized, nothing_to_show, stderr, cleanup)
  }

  /// A message the host acted on.
  fn accept(
    &mut self,
    now: Timestamp,
    normalized: Normalized<Response>,
    nothing_to_show: WhenNothingToShow,
    stderr: Captured,
    cleanup: Option<CleanupFailure>,
  ) -> Outcome {
    let Normalized { value, discarded } = normalized;

    // The retained value is always `Some` — `resolved_check` is seeded at
    // construction and is not an `Option` (I4) — so this is R-26's first two
    // arms. The third is reachable only from `new`.
    let next_check = schedule::resolve(
      Some(self.state.resolved_check()),
      value.schedule(),
      self.config.schedule.default_poll,
      now,
    );
    self.state.resolve_to(next_check);

    let view = if let Some(view) = value.view() {
      Some(Presented {
        // Minting replaces whatever was outstanding, and the replaced id becomes
        // stale immediately (R-33). That is true of both entry points.
        view_id: self.state.issue(now),
        view: view.clone(),
      })
    } else {
      match nothing_to_show {
        WhenNothingToShow::LeaveOutstanding => (),
        WhenNothingToShow::CloseOutstanding => self.state.close(),
      }
      None
    };

    Outcome {
      view,
      next_check,
      discarded,
      stderr,
      failure: None,
      cleanup,
    }
  }

  /// An outcome on which the host changed nothing.
  ///
  /// The schedule is reported as it already stood: every failure path leaves
  /// `resolved_check` exactly as it was, because the alternative — a failed
  /// exchange clearing or extending the schedule — turns a broken backend into a
  /// silent host (R-29, P2, EX-5). The outstanding interaction is untouched for
  /// the same reason (R-34).
  fn no_action(
    &self,
    failure: Failure,
    stderr: Captured,
    cleanup: Option<CleanupFailure>,
  ) -> Outcome {
    Outcome {
      view: None,
      next_check: self.state.resolved_check(),
      discarded: Vec::new(),
      stderr,
      failure: Some(failure),
      cleanup,
    }
  }
}

/// The bytes a backend wrote, as a canonical response.
///
/// `from_slice` is where `BackendError::Protocol` arises and where R-38's
/// framing rule is enforced: the transport returns bytes and parses nothing, so
/// this is the one place in the host that reads what a backend wrote. A body
/// that is not **exactly one** JSON document never reaches normalization —
/// empty stdout is an unexpected EOF, a second document is trailing content, and
/// bytes that are not UTF-8 are an invalid code point. All three are
/// `serde_json::Error`, so all three are `ProtocolError::Json`.
///
/// Trailing *whitespace* is not trailing content, which is serde's reading and
/// the right one: a backend ending its document with a newline is not sending
/// two.
fn read(bytes: &[u8], now: Timestamp) -> Result<Normalized<Response>, ProtocolError> {
  let wire: WireResponse = serde_json::from_slice(bytes).map_err(ProtocolError::Json)?;
  normalize_response(wire, now)
}

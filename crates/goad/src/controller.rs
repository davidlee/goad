//! The controller — design.md §5.3's retained state and §5.4's reducer —
//! and, from PHASE-10, `serve` itself: the loop, `Pending`, `Ending` and
//! `Served`. `Wire` and `Cancel` are `wire.rs`'s (PHASE-07); everything else
//! testable with no component and no runtime lands here — the fold, and now
//! the loop that drives it.

use std::collections::BTreeMap;

use goad_shell::backend::transport::Backend;
use goad_shell::host::{Host, Outcome};
use tokio::select;
use tokio::sync::mpsc;

use goad_semantics::protocol::canonical::{Event, Timestamp, UserResponse, ViewId};
use goad_semantics::schedule::wait_for;

use crate::clock::Clock;
use crate::diagnostics::{Diagnostics, Refused};
use crate::glass::Glass;
use crate::reception::{Prepared, Received, receive};
use crate::wire::{Cancel, Command, Stimulus};

/// What the person is looking at. One window, three states, **one value** —
/// "is it visible" and "which mode" are not separable facts, and treating
/// them as two is F-15.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
  Hidden,
  Prompt,
  Diagnostics,
}

/// Whether the person has asked to read the diagnostics. Set only by
/// `OpenDiagnostics`; cleared by `CloseDiagnostics` and by a fold that
/// replaces the presentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Focus {
  Automatic,
  Diagnostics,
}

/// What a fold did to the outstanding interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shift {
  Replaced,
  Retained,
  Closed,
}

/// What one folded exchange tells the loop. `next_check` is **not** an
/// `Option`: `Outcome::next_check` is concrete on every outcome including
/// failures (`goad-shell/src/host.rs:76`), so an exchange that completed
/// always resolved one (design.md §5.2, D-15).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Absorbed {
  pub shift: Shift,
  pub next_check: Timestamp,
}

/// Which entry point produced an `Outcome`. The reducer needs it because
/// `Host` treats `view: None` differently for the two (`host.rs:100-109`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exchanged {
  Evaluation,
  Answer,
}

/// Why the loop stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
  /// The stop signal was tripped.
  Stopped,
  /// Every sender was dropped. Only reachable at teardown.
  Closed,
}

/// The loop, and everything it owned, handed back.
#[derive(Debug)]
pub struct Served<B: Backend, G: Glass> {
  pub ending: Ending,
  pub host: Host<B>,
  pub controller: Controller,
  pub glass: G,
}

/// Everything the glass needs, borrowed. Total: every property but `notice`
/// is written from this, every time.
#[derive(Debug, Clone, Copy)]
pub struct Frame<'a> {
  pub surface: Surface,
  pub shown: Option<&'a Prepared>,
  pub diagnostics: &'a Diagnostics,
  pub busy: bool,
  pub next_check: Option<Timestamp>,
}

/// What the controller retains. One value, no Slint types, so it is testable
/// without a platform and the fold is provable in isolation. That is the
/// complete retained state: the presentation and its `ViewId` and its
/// canonical options (inside `Prepared`), the diagnostics, the window mode
/// and its visibility (`Surface`, derived below), and whether an exchange is
/// in flight. Nothing else is retained anywhere in the renderer.
#[derive(Debug)]
pub struct Controller {
  shown: Option<Prepared>,
  diagnostics: Diagnostics,
  focus: Focus,
  engaged: bool,
  /// The last resolved next check, for display only. Written by `absorb`,
  /// read by `frame()`. `None` until the first exchange completes; there is
  /// no accessor beyond `frame()` — it would exist only to be unwrapped
  /// (design.md §5.2, D-15).
  next_check: Option<Timestamp>,
}

impl Default for Controller {
  fn default() -> Self {
    Self::new()
  }
}

impl Controller {
  #[must_use]
  pub fn new() -> Self {
    Self {
      shown: None,
      diagnostics: Diagnostics::default(),
      focus: Focus::Automatic,
      engaged: false,
      next_check: None,
    }
  }

  /// The surface is **derived**, never stored — three inputs, three outputs,
  /// no combination unnamed (F-15).
  #[must_use]
  fn surface(&self) -> Surface {
    match (self.focus, self.shown.is_some()) {
      (Focus::Diagnostics, _) => Surface::Diagnostics,
      (Focus::Automatic, true) => Surface::Prompt,
      (Focus::Automatic, false) => Surface::Hidden,
    }
  }

  /// Fold one completed exchange. The **only** place host state and
  /// renderer state are reconciled, and the function design.md §5.4's table
  /// specifies. It calls `receive` and folds the `Received`, so the
  /// `Outcome` is consumed exactly once, here.
  ///
  /// **It clears `engaged` before it returns**, unconditionally and whatever
  /// the `Shift` — the exchange it is folding is the exchange that has just
  /// ended, and there is no outcome for which the controls should stay
  /// disabled (F-21).
  pub fn absorb(&mut self, exchanged: Exchanged, outcome: Outcome) -> Absorbed {
    let Received {
      prepared,
      refused,
      next_check,
      diagnostics,
    } = receive(outcome);

    let shift = reduce(exchanged, prepared.is_some(), refused);
    match shift {
      Shift::Replaced => {
        self.shown = prepared;
        self.focus = Focus::Automatic;
      }
      Shift::Closed => self.shown = None,
      Shift::Retained => {}
    }

    self.diagnostics = diagnostics;
    self.engaged = false;
    self.next_check = Some(next_check);
    Absorbed { shift, next_check }
  }

  /// Fold a refusal the renderer made itself. No backend was contacted, so
  /// the presentation is untouched — the same rule `no_action` follows
  /// (R-34).
  pub fn refuse(&mut self, refused: &Refused) {
    self.diagnostics = Diagnostics::refused(refused);
  }

  /// Resolve a click against retained state.
  ///
  /// # Errors
  ///
  /// [`Refused::SupersededView`] when `view` is not the retained token, or
  /// nothing is retained at all; [`Refused::UnknownOption`] when `option` is
  /// not one the retained presentation carries. Nothing is sent in either
  /// case.
  pub fn answer(&self, view: &str, option: &str) -> Result<(ViewId, UserResponse), Refused> {
    let prepared = self.shown.as_ref().ok_or(Refused::SupersededView)?;
    if prepared.view_id.as_str() != view {
      return Err(Refused::SupersededView);
    }
    let matched = prepared
      .presentation
      .options
      .iter()
      .find(|candidate| candidate.id.as_str() == option)
      .ok_or(Refused::UnknownOption)?;

    Ok((
      prepared.view_id.clone(),
      UserResponse {
        option: matched.id.clone(),
        values: BTreeMap::new(),
      },
    ))
  }

  pub fn open_diagnostics(&mut self) {
    self.focus = Focus::Diagnostics;
  }

  pub fn close_diagnostics(&mut self) {
    self.focus = Focus::Automatic;
  }

  /// An exchange is starting. Sets `engaged`, which the next frame carries.
  /// `absorb` clears it; nothing else sets or clears it. The two calls are
  /// one pair, in one place — `serve`'s exchange arm — so an exchange cannot
  /// leave the controls disabled for the rest of the process (F-21).
  pub fn engage(&mut self) {
    self.engaged = true;
  }

  /// Everything the glass needs, borrowed.
  #[must_use]
  pub fn frame(&self) -> Frame<'_> {
    Frame {
      surface: self.surface(),
      shown: self.shown.as_ref(),
      diagnostics: &self.diagnostics,
      busy: self.engaged,
      next_check: self.next_check,
    }
  }
}

/// design.md §5.4's reducer table, rows 1-7: a total match on `(Exchanged,
/// prepared.is_some(), refused)`, eight combinations, no `_` arm and no
/// `unreachable!()`. A view in hand (`prepared.is_some()`) always replaces —
/// rows 1 and 7, the latter unreachable in production (`accept` never mints
/// a view alongside a failure) but written as `Replaced` rather than as a
/// panic on a value the host itself produced. Grouped by resulting `Shift`
/// rather than left as eight arms, so no two arms share a body
/// (`clippy::match_same_arms`).
fn reduce(exchanged: Exchanged, has_view: bool, refused: bool) -> Shift {
  match (exchanged, has_view, refused) {
    (Exchanged::Evaluation | Exchanged::Answer, true, false | true) => Shift::Replaced,
    (Exchanged::Evaluation, false, false | true) | (Exchanged::Answer, false, true) => {
      Shift::Retained
    }
    (Exchanged::Answer, false, false) => Shift::Closed,
  }
}

/// A stamp, or the refusal that says why there is none. `Refused::NoClock`
/// renders `ClockError`'s `Display` once, at the one site that has it.
///
/// `serve`'s dispatch, below, is now its only caller — the `expect(dead_code)`
/// wrapper PHASE-06 spent one of A-2's slots on is removed in this phase,
/// which is the phase that first calls it from production code.
fn stamp(clock: Clock) -> Result<Timestamp, Refused> {
  clock().map_err(|error| Refused::NoClock {
    detail: error.to_string(),
  })
}

/// One exchange, resolved but not yet started: the arguments a `Host` entry
/// point needs, and nothing else. It exists so that the loop has **one**
/// future to select against the stop signal rather than two duplicated
/// `select!`s, and so that the thing cancellation drops is the exchange
/// itself rather than a wrapper around it.
#[derive(Debug)]
enum Pending {
  Evaluate {
    now: Timestamp,
    event: Event,
  },
  Respond {
    now: Timestamp,
    view_id: ViewId,
    answer: UserResponse,
  },
}

impl Pending {
  /// Which entry point this is, for the reducer. Derived rather than
  /// carried, so the two cannot disagree.
  fn exchanged(&self) -> Exchanged {
    match self {
      Self::Evaluate { .. } => Exchanged::Evaluation,
      Self::Respond { .. } => Exchanged::Answer,
    }
  }

  /// The instant `stamp` resolved this exchange's request against. The
  /// re-arm computes the wait from this, not from a fresh clock read
  /// (design.md §5.2, D-11).
  fn now(&self) -> Timestamp {
    match self {
      Self::Evaluate { now, .. } | Self::Respond { now, .. } => *now,
    }
  }
}

/// The host's own floor on how often it evaluates of its own accord
/// (SPEC-002/R-4). Not configurable, not visible to a backend, and never
/// applied to anything a person asked for — only to a scheduled evaluation
/// the loop began on its own (design.md §5.2/§5.4, D-2, D-3).
const MINIMUM_SPACING: std::time::Duration = std::time::Duration::from_secs(3);

/// Which arm of the first `select!` produced a command to dispatch. A bare
/// `Command` carries no provenance, and the conditional re-arm after a
/// refusal (EX-7) has nothing to be true of without this discriminant
/// (design.md §5.4, F-12).
#[derive(Debug)]
enum Fired {
  Command(Command),
  Scheduled,
}

/// Serve commands until stopped. **This is the production controller**: a
/// `spawn_local` block around it is one line (PHASE-08's), and the cheap
/// test tier drives the identical call under `block_on` (D9). There is no
/// second implementation of the loop and no test-only harness for it.
///
/// An ordinary `async fn`, carrying **no** attribute at all —
/// `clippy::future_not_send` does not reach this signature: it drops `Send`
/// obligations that mention a type parameter at the top level, and `serve`'s
/// future is `!Send` only through `B` and `G` (design.md §5.5, A-5,
/// measured).
///
/// Everything is taken by value because `slint::spawn_local` needs a
/// `'static` future, and handed back in `Served` so a test can read what it
/// did.
pub async fn serve<B, G>(
  mut host: Host<B>,
  mut controller: Controller,
  mut commands: mpsc::Receiver<Command>,
  cancel: Cancel,
  clock: Clock,
  mut glass: G,
) -> Served<B, G>
where
  B: Backend + 'static,
  G: Glass + 'static,
{
  let started = tokio::time::Instant::now();
  // Always armed (D-5): if the startup evaluation is never dispatched or is
  // refused, this initial arm fires at `MINIMUM_SPACING` and the host
  // recovers by itself.
  let mut sleep = Box::pin(tokio::time::sleep_until(started + MINIMUM_SPACING));
  // Nothing scheduled has fired yet, so nothing is floored: the first
  // scheduled firing of the process is unfloored (D-5, D-3).
  let mut floor_until = started;

  let ending = loop {
    glass.present(controller.frame()); // busy = false here
    let fired = select! { biased;
      () = cancel.stopped()       => break Ending::Stopped,
      received = commands.recv()  => match received {
        None          => break Ending::Closed,
        Some(command) => Fired::Command(command),
      },
      () = &mut sleep => {
        // The one and only write site (design.md §5.3/§5.4, D-3).
        floor_until = tokio::time::Instant::now() + MINIMUM_SPACING;
        Fired::Scheduled
      }
    };
    // A refusal that came from this arm re-arms at the floor (EX-7); every
    // other refusal leaves the deadline untouched.
    let refusal_re_arms = matches!(fired, Fired::Scheduled);
    let command = match fired {
      Fired::Command(command) => command,
      // Goes through the same `stamp` as every other command (EX-6).
      Fired::Scheduled => Command::Evaluate(Stimulus::Scheduled),
    };

    // Exhaustive, no `_` arm, and every refusal path `continue`s to the top
    // — which presents with `busy = false` and clears `notice` in the same
    // call. Identity is checked before the clock: a superseded click is
    // refused for the reason that is true of it, and a broken clock does
    // not relabel it.
    let pending = match command {
      Command::OpenDiagnostics => {
        controller.open_diagnostics();
        continue;
      }
      Command::CloseDiagnostics => {
        controller.close_diagnostics();
        continue;
      }
      Command::Evaluate(stimulus) => match stamp(clock) {
        Ok(now) => Pending::Evaluate {
          now,
          event: stimulus.event(now),
        },
        Err(refused) => {
          controller.refuse(&refused);
          if refusal_re_arms {
            sleep.as_mut().reset(floor_until);
          }
          continue;
        }
      },
      Command::Choose { view, option } => match controller.answer(&view, &option) {
        Err(refused) => {
          controller.refuse(&refused);
          if refusal_re_arms {
            sleep.as_mut().reset(floor_until);
          }
          continue;
        }
        Ok((view_id, answer)) => match stamp(clock) {
          Ok(now) => Pending::Respond {
            now,
            view_id,
            answer,
          },
          Err(refused) => {
            controller.refuse(&refused);
            if refusal_re_arms {
              sleep.as_mut().reset(floor_until);
            }
            continue;
          }
        },
      },
    };
    let requested_at = pending.now();
    let exchanged = pending.exchanged();

    controller.engage();
    glass.present(controller.frame()); // busy = true, controls disabled

    // One future, built from the enum. `host` is borrowed mutably for
    // exactly as long as this block lives, which is this iteration;
    // `break` in the other arm drops it, which is what releases the borrow
    // before `Served` hands `host` back.
    let call = async {
      match pending {
        Pending::Evaluate { now, event } => host.evaluate(now, event).await,
        Pending::Respond {
          now,
          view_id,
          answer,
        } => host.respond(now, view_id, answer).await,
      }
    };

    select! { biased;
      () = cancel.stopped() => break Ending::Stopped, // `call` is DROPPED here
      outcome = call        => {
        let absorbed = controller.absorb(exchanged, outcome);
        let wait = wait_for(absorbed.next_check, requested_at);
        let deadline = std::cmp::max(tokio::time::Instant::now() + wait, floor_until);
        sleep.as_mut().reset(deadline);
      },
    }
  };
  Served {
    ending,
    host,
    controller,
    glass,
  }
}

// `stamp`'s only caller is `serve`, above. Tested here, inline, rather than
// left with no test at all: the crate-external `tests/renderer/` tiers
// cannot reach a private free
// function, and `crates/goad-shell/src/state.rs` and
// `crates/goad-semantics/src/schedule.rs` already use this same
// `#[cfg(test)] mod tests` shape for a stratum-internal pure function.
#[cfg(test)]
mod tests {
  use super::{Refused, stamp};
  use crate::clock::{ClockError, wall_clock};

  #[test]
  fn a_working_clock_is_returned_unchanged() {
    assert!(stamp(wall_clock).is_ok(), "the real clock does not fail");
  }

  #[test]
  fn a_broken_clock_becomes_a_refusal_naming_its_display() {
    fn fails() -> Result<goad_semantics::protocol::canonical::Timestamp, ClockError> {
      Err(ClockError::BeforeEpoch)
    }
    match stamp(fails) {
      Err(Refused::NoClock { detail }) => {
        assert_eq!(detail, ClockError::BeforeEpoch.to_string());
      }
      other => panic!("expected a `NoClock` refusal; got {other:?}"),
    }
  }
}

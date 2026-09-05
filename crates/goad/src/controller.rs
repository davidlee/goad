//! The controller — design.md §5.3's retained state and §5.4's reducer.
//!
//! `serve`, `Wire`, `Cancel`, `Pending`, `Ending` and `Served` are PHASE-07's
//! (and PHASE-10's `serve` body): they need the event loop and the `mpsc`
//! channel this phase does not build. What lands here is everything testable
//! with no component and no runtime — the fold itself.

use std::collections::BTreeMap;

use goad_semantics::protocol::canonical::{Timestamp, UserResponse, ViewId};
use goad_shell::host::Outcome;

use crate::clock::Clock;
use crate::diagnostics::{Diagnostics, Refused};
use crate::reception::{Prepared, Received, receive};

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

/// Which entry point produced an `Outcome`. The reducer needs it because
/// `Host` treats `view: None` differently for the two (`host.rs:100-109`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exchanged {
  Evaluation,
  Answer,
}

/// Everything the glass needs, borrowed. Total: every property but `notice`
/// is written from this, every time.
#[derive(Debug, Clone, Copy)]
pub struct Frame<'a> {
  pub surface: Surface,
  pub shown: Option<&'a Prepared>,
  pub diagnostics: &'a Diagnostics,
  pub busy: bool,
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
  pub fn absorb(&mut self, exchanged: Exchanged, outcome: Outcome) -> Shift {
    let Received {
      prepared,
      refused,
      // Resolved on every outcome, but nothing in this slice retains a
      // schedule — slice 003 fills that seam (design.md §5.4).
      next_check: _,
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
    shift
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
    let prepared = self.shown.as_ref().ok_or_else(|| Refused::SupersededView {
      named: view.to_owned(),
    })?;
    if prepared.view_id.as_str() != view {
      return Err(Refused::SupersededView {
        named: view.to_owned(),
      });
    }
    let matched = prepared
      .presentation
      .options
      .iter()
      .find(|candidate| candidate.id.as_str() == option)
      .ok_or_else(|| Refused::UnknownOption {
        named: option.to_owned(),
      })?;

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
/// Uncalled until PHASE-07's `serve` exists to dispatch a `Command`; landed
/// here now because EX-3 states it as part of this module's surface and its
/// only sensible home is beside the dispatch it serves.
// `#[cfg_attr(not(test), ...)]`, not a bare `#[expect]`: the `#[cfg(test)]
// mod tests` below gives `stamp` a real caller in the test build, where the
// expectation would go unfulfilled — `unfulfilled_lint_expectations` under
// `-D warnings` (measured). The gap is exactly the one `Cargo.toml`'s own
// `dead_code` comment names as the escape.
#[cfg_attr(
  not(test),
  expect(
    dead_code,
    reason = "PHASE-06's own surface (design.md §5.3's controller block, \
      EX-3) lands this beside the dispatch it serves; its only caller is \
      PHASE-07's `serve`, which does not exist yet. Spends one of A-2's two \
      remaining slots (notes.md, PHASE-06 sheet)."
  )
)]
fn stamp(clock: Clock) -> Result<Timestamp, Refused> {
  clock().map_err(|error| Refused::NoClock {
    detail: error.to_string(),
  })
}

// `stamp`'s only caller is PHASE-07's `serve`, which does not exist yet.
// Tested here, inline, rather than left with no test at all: the
// crate-external `tests/renderer/` tiers cannot reach a private free
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

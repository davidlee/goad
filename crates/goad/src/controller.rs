//! The controller — design.md §5.3's retained state and §5.4's reducer —
//! and, from PHASE-10, `serve` itself: the loop, `Pending`, `Ending` and
//! `Served`. `Wire` and `Cancel` are `wire.rs`'s (PHASE-07); everything else
//! testable with no component and no runtime lands here — the fold, and now
//! the loop that drives it.

use std::collections::BTreeMap;

use goad_shell::backend::transport::Backend;
use goad_shell::host::{Host, Outcome};
use goad_shell::ingress::{Answer, Arrival, Ingress, Refusal, UnavailableCause};
use tokio::select;
use tokio::sync::mpsc;

use goad_semantics::protocol::canonical::{Event, Timestamp, UserResponse, ViewId};
use goad_semantics::schedule::wait_for;

use crate::diagnostics::{Diagnostics, Refused};
use crate::draft::{Reported, submitted};
use crate::glass::Glass;
use crate::reception::{Prepared, Received, receive};
use crate::view_model::{PresentationField, PresentationOption, as_drawn, interpret};
use crate::wire::{Cancel, Command, Notice, PendingEdit, Stimulus};
use goad_shell::clock::Clock;

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
/// always resolved one (SPEC-001/R-27).
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
  /// Every sender was dropped.
  ///
  /// **Reachable from the test tiers, and in production from nowhere at all**
  /// (F-R9). Eight `mpsc::Sender<Command>` clones outlive the loop: `main.rs:86`
  /// binds `tx` for the whole of `start`, which outlives
  /// `run_event_loop_until_quit`; `Wire::new` takes one (`main.rs:89`) and
  /// `install` clones it into seven callbacks (`install.rs:39`, `:65`, `:82`,
  /// `:93`, `:99`, `:104`, `:109`) that live in the window's and the tray's
  /// callback tables for the life of the process; and the armed debounce timer's
  /// closure retains an eighth (F-R6). The production shutdown path is
  /// `Stopped`, via `Cancel` — `install.rs:95` and `:110` trip it.
  Closed,
}

/// The loop, and everything it owned, handed back.
#[derive(Debug)]
pub struct Served<B: Backend, G: Glass> {
  pub ending: Ending,
  pub host: Host<B>,
  pub controller: Controller,
  pub glass: G,
  /// Handed back the way `host`, `controller` and `glass` are, so that
  /// dropping it — and with it the accept task's channel — is `main`'s to do
  /// at the same moment it drops everything else (`design.md` §5.4,
  /// *Shutdown*).
  pub ingress: Ingress,
}

/// Everything the glass needs, borrowed. Total: every property is written
/// from this, every time.
#[derive(Debug, Clone, Copy)]
pub struct Frame<'a> {
  pub surface: Surface,
  pub shown: Option<&'a Prepared>,
  pub diagnostics: &'a Diagnostics,
  /// Whether **the person's own answer** is in flight — not whether the host
  /// is talking to the backend. See [`Controller::engage`].
  pub busy: bool,
  pub next_check: Option<Timestamp>,
  /// Whether back-pressure is outstanding. The one frame property the
  /// controller does not retain: the signal is held at the edge and sampled
  /// by `serve` at present time, so `Controller` keeps no field for it
  /// (design.md §5.3).
  pub notice: bool,
}

/// What the controller retains. One value, no Slint types, so it is testable
/// without a platform and the fold is provable in isolation. That is the
/// complete retained state: the presentation and its `ViewId` and its
/// canonical options and the draft answering it (`Prepared` holds the
/// presentation, its `ViewId` and the draft; the canonical options are inside
/// the presentation),
/// the diagnostics, the window mode and its visibility (`Surface`, derived
/// below), and whether the person's own answer is in flight (`engaged` —
/// [`Controller::engage`] states why it is not *any* exchange).
///
/// One thing the screen shows is retained and is **not** here: back-pressure.
/// It lives at the edge, in a `wire::Notice` the loop samples at present time,
/// so that the concurrency primitive stays where the others already are and
/// this value keeps its "no Slint types, no channels" property
/// (`design.md` §5.3).
#[derive(Debug)]
pub struct Controller {
  shown: Option<Prepared>,
  diagnostics: Diagnostics,
  focus: Focus,
  engaged: bool,
  /// The last resolved next check, for display only. Written by `absorb`,
  /// read by `frame()`. `None` until the first exchange completes; there is
  /// no accessor beyond `frame()` — it would exist only to be unwrapped: a
  /// resolved check is concrete in every case (SPEC-001/R-27), so the
  /// `Option` says "no exchange yet" and nothing else.
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
    let matched = selected(prepared, view, option)?;

    // **The walk is over what was drawn, never over the draft.** A value for
    // every drawn field of this option, no value for a field that was not
    // drawn, and a draft key that outlived its view cannot reach the wire —
    // three properties of `SPEC-001/R-58` with no check to forget, held by
    // the shape of the walk rather than by a rule an agent must remember.
    // With the `SupersededView` refusal above, every key submitted here
    // provably came from a field the currently-retained view declared.
    // `state_of` answers `None` for a field nobody touched, and `R-58`
    // forbids omitting a value for a drawn field — so this is one of the two
    // sites that apply `as_drawn`, and the only one in this file
    // (`design.md` §5.2). `glass.rs` deliberately is not the other: it reads
    // the `None` as *untouched* and shows an unpicked `datetime` as *not
    // set*, while the value here is the epoch.
    let values: BTreeMap<_, _> = drawn_fields(matched)
      .map(|field| {
        let edited = prepared
          .draft
          .state_of(&matched.id, &field.id)
          .unwrap_or_else(|| as_drawn(&field.kind));
        (field.id.clone(), submitted(&edited))
      })
      .collect();

    Ok((
      prepared.view_id.clone(),
      UserResponse {
        option: matched.id.clone(),
        values,
      },
    ))
  }

  /// Answer, carrying whatever the debounce was still holding.
  ///
  /// **Identity of the command is checked once, first**, exactly as it is for a
  /// click that carries nothing: a `Choose` whose `view` is not the retained
  /// token refuses the whole thing and records nothing. `selected` is that
  /// check and it is not repeated here.
  ///
  /// Past it, each carried edit is applied through the walk [`Self::edit`]
  /// already makes, and the two ways one can fail are **not** symmetrical:
  ///
  /// - **its `view` is not the retained one.** The person typed into a view
  ///   that has since been replaced and then answered the replacement. The
  ///   refusal is reported — the typing really was discarded — and **the
  ///   answer still goes**, because it is about the view that *is* retained
  ///   and nothing about it is incomplete. Two stale edits report **once**,
  ///   not twice, because `Diagnostics::refused` replaces rather than
  ///   accumulates.
  /// - **its option or field is not one the retained view declares.** The
  ///   markup and the retained presentation disagree, which is a renderer bug
  ///   rather than a race. It takes the posture an out-of-range `ComboBox`
  ///   index already has: reported through the existing refusal site, nothing
  ///   further recorded, and **no answer sent** — an answer the host knows was
  ///   built from an incomplete draft is worse than a refusal a person can see
  ///   (`design.md` §5.2).
  ///
  /// The reported line lasts for the life of the exchange and no longer;
  /// `absorb` replaces the diagnostics wholesale when it folds. Nothing here
  /// promises longer, and D-36 is why nothing tries.
  ///
  /// No order is assumed over `edits`: the keys are distinct by construction,
  /// so any order yields the same draft.
  ///
  /// # Errors
  ///
  /// Whatever [`Self::answer`] refuses, and [`Refused::UnknownOption`] or
  /// [`Refused::UnknownField`] from a carried edit the retained view does not
  /// declare.
  pub fn choose(
    &mut self,
    view: &str,
    option: &str,
    edits: &[PendingEdit],
  ) -> Result<(ViewId, UserResponse), Refused> {
    // Identity before the edits: a `Choose` naming a replaced view is refused
    // for the reason that is true of it, and nothing it carries is applied.
    // The block is what ends the borrow — `selected` hands back a reference
    // into the retained presentation, and the loop below writes it.
    {
      let prepared = self.shown.as_ref().ok_or(Refused::SupersededView)?;
      selected(prepared, view, option)?;
    }

    let mut superseded = false;
    for edit in edits {
      match self.edit(&edit.view, &edit.option, &edit.field, &edit.value) {
        Ok(()) => {}
        // The one refusal that does not stop the answer.
        Err(Refused::SupersededView) => superseded = true,
        // Returned rather than reported here: `serve`'s single refusal site
        // reports what `dispatch` hands back, and a second write would say the
        // same thing twice.
        Err(refused) => return Err(refused),
      }
    }
    if superseded {
      self.refuse(&Refused::SupersededView);
    }

    self.answer(view, option)
  }

  /// Record what the person did to one field of one option.
  ///
  /// The only `&mut self` half of the pair: `answer` reads the draft and this
  /// writes it. The write is keyed by ids **cloned off the retained
  /// presentation**, never minted — `OptionId::new` and `FieldId::new` are
  /// `pub(super)` in `goad-semantics`, so a key that names nothing the
  /// backend declared is not constructible here (`design.md` §5.2).
  ///
  /// **The report is interpreted here, on the walk that was already being
  /// made.** A widget reports in its own terms — an index, a typed text — and
  /// only the retained presentation can say what those mean: which
  /// alternative an index names, and what number a text that no finite parse
  /// accepts falls back to. That is `view_model::interpret`, and the field it
  /// needs is `declared`, which the membership check has in hand
  /// (`design.md` §5.2, §7 D25).
  ///
  /// # Errors
  ///
  /// [`Refused::SupersededView`] when `view` is not the retained token, or
  /// nothing is retained at all; [`Refused::UnknownOption`] when `option` is
  /// not one the retained presentation carries; [`Refused::UnknownField`]
  /// when that option's blocks do not declare `field`, **and equally when
  /// `interpret` answers `None`** — an index no alternative has, a non-finite
  /// slider value, or a report whose variant is not the drawn field's kind.
  /// All three are renderer bugs and take the posture this refusal already
  /// has: reported, nothing recorded, and no new class added to the taxonomy
  /// (`design.md` §5.2). Nothing is recorded on any of those paths.
  pub fn edit(
    &mut self,
    view: &str,
    option: &str,
    field: &str,
    reported: &Reported,
  ) -> Result<(), Refused> {
    let prepared = self.shown.as_mut().ok_or(Refused::SupersededView)?;
    let matched = selected(prepared, view, option)?;
    // The same walk `answer` submits from, so a field that can be edited is
    // exactly a field that will be submitted. An undrawn field is not in a
    // block, so it is refused here and carries no value there.
    let declared = drawn_fields(matched)
      .find(|candidate| candidate.id.as_str() == field)
      .ok_or(Refused::UnknownField)?;
    // Cloned before the write, which is also what ends the borrow of the
    // presentation the walk above took.
    let (option_id, field_id) = (matched.id.clone(), declared.id.clone());
    // `state_of` answers `None` for a field nobody has touched; `interpret`
    // applies the as-drawn rule itself, so this site writes no fallback.
    let held = prepared.draft.state_of(&option_id, &field_id);
    let value = interpret(reported, held.as_ref(), &declared.kind).ok_or(Refused::UnknownField)?;
    prepared.draft.record(option_id, field_id, value);
    Ok(())
  }

  pub fn open_diagnostics(&mut self) {
    self.focus = Focus::Diagnostics;
  }

  pub fn close_diagnostics(&mut self) {
    self.focus = Focus::Automatic;
  }

  /// An exchange is starting. Sets `engaged`, which the next frame carries as
  /// `busy`.
  ///
  /// **`busy` means *your answer is in flight*, not *the host is talking to
  /// the backend*** (`review-code.md` F-A1, F-R2). Only an
  /// `Exchanged::Answer` engages. An `Exchanged::Evaluation` — a scheduled
  /// poll, a tray check, an ingested event — is the host's own business and
  /// leaves every control live, because Slint *discards* input for a disabled
  /// item rather than queueing it (`i-slint-core/items/text.rs:954`), so
  /// disabling the form for the length of a round trip loses the characters
  /// typed during it.
  ///
  /// What stays disabled is what the disable was written for: the option
  /// `Button`'s double-submit guard. `Command::Choose` has exactly one origin
  /// (`install.rs:40`) and is the only road to `Pending::Respond`, so a
  /// `Respond` is always the person's own click and the guard now fires
  /// exactly when it wants to and at no other time.
  ///
  /// `absorb` clears it; nothing else sets or clears it. The two calls are
  /// one pair, in one place — `serve`'s exchange arm — so an exchange cannot
  /// leave the controls disabled for the rest of the process (F-21). An
  /// evaluation's pair still holds: clearing what it never set is a no-op.
  pub fn engage(&mut self, exchanged: Exchanged) {
    self.engaged = exchanged == Exchanged::Answer;
  }

  /// Everything the glass needs, borrowed. `notice` is passed in rather than
  /// read: it is the one frame property this controller does not retain
  /// (design.md §5.3). `&self`, and nothing here mutates.
  #[must_use]
  pub fn frame(&self, notice: bool) -> Frame<'_> {
    Frame {
      surface: self.surface(),
      shown: self.shown.as_ref(),
      diagnostics: &self.diagnostics,
      busy: self.engaged,
      next_check: self.next_check,
      notice,
    }
  }
}

/// The option a click or an edit named, or the refusal that says which
/// selector failed.
///
/// One statement of the two refusals `answer` and `edit` both make, in the
/// order they both make them: **identity before membership**, so a click
/// naming a replaced view is refused for the reason that is true of it rather
/// than for a missing option that was never looked for.
fn selected<'a>(
  prepared: &'a Prepared,
  view: &str,
  option: &str,
) -> Result<&'a PresentationOption, Refused> {
  if prepared.view_id.as_str() != view {
    return Err(Refused::SupersededView);
  }
  prepared
    .presentation
    .options
    .iter()
    .find(|candidate| candidate.id.as_str() == option)
    .ok_or(Refused::UnknownOption)
}

/// Every field the host **drew** of one option, in declared order.
///
/// Blocks are layout, not meaning, so a field's identity is the option's and
/// its own and never its block's — flattening them is what makes "the drawn
/// fields of this option" one sequence. This is the walk `SPEC-001/R-58` is
/// held by, and there is deliberately no counterpart over `Draft`: the draft
/// exposes no way to enumerate its keys, which is what makes walking the
/// declared fields a property of the types rather than a convention
/// (`design.md` §5.2, §5.5 I-3).
fn drawn_fields(option: &PresentationOption) -> impl Iterator<Item = &PresentationField> {
  option.blocks.iter().flat_map(|block| block.fields.iter())
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
  /// re-arm computes the wait from this, not from a fresh clock read:
  /// SPEC-002/R-2's whole input is the resolved instant the last exchange
  /// reported, and a second clock read would be an instant of the timer's
  /// own.
  fn now(&self) -> Timestamp {
    match self {
      Self::Evaluate { now, .. } | Self::Respond { now, .. } => *now,
    }
  }
}

/// The host's own floor on how often it evaluates of its own accord
/// (SPEC-002/R-4, SPEC-002/R-12). Not configurable, not visible to a backend,
/// and never applied to anything a person asked for.
///
/// **One constant, two anchors.** It spaces each bounded class of firing from
/// the previous firing of *its own* class and from nothing else — a scheduled
/// evaluation the loop began on its own (SPEC-002/R-5, `floor_until`), and an
/// evaluation an ingested event began (SPEC-002/R-12, `event_floor_until`).
/// There is deliberately no second constant: a configurable one would let a
/// test buy time by moving a bound (`design.md` D-5).
const MINIMUM_SPACING: std::time::Duration = std::time::Duration::from_secs(3);

/// The longest wait the host will park a timer on. Reached only when
/// `now + wait` would overflow the platform's monotonic clock.
///
/// A firing this clamp brings forward is harmless and self-correcting: the
/// exchange it produces resolves the schedule again from the instruction the
/// host still holds, so an instruction further out than a year is simply
/// re-armed a year at a time. Nothing stored or reported moves —
/// SPEC-001/R-28 governs the instruction, and this governs only when the
/// host fires (SPEC-002 §6, the same separation the minimum spacing rests
/// on).
///
/// `Duration::new`, not `from_secs`: `clippy::duration_suboptimal_units`
/// wants `Duration::from_days`, which is not yet stable, and POL-001 forbids
/// suppressing a lint to get past the gate.
const LONGEST_WAIT: std::time::Duration = std::time::Duration::new(365 * 24 * 60 * 60, 0);

/// The instant to park the sleep on: `wait` after `now`, never earlier than
/// the floor.
///
/// Total. `wait_for` is total across jiff's representable range, so a
/// backend instructing a `next_check` at the far edge of time hands this a
/// wait of ~10^11 seconds; `Instant + Duration` panics on overflow rather
/// than returning an `Option`, and a panic here would be a backend failure
/// taking the host down (SPEC-001/R-45). The platform representation that
/// happens to make the sum fit today is not something any document states,
/// so the arithmetic is checked rather than trusted: an overflowing wait is
/// clamped to `LONGEST_WAIT`, and a `now` so late that even that overflows
/// yields `now` itself, which fires at once and is then floored like any
/// other scheduled firing.
fn deadline_after(
  now: tokio::time::Instant,
  wait: std::time::Duration,
  floor: tokio::time::Instant,
) -> tokio::time::Instant {
  let arrives = now
    .checked_add(wait)
    .or_else(|| now.checked_add(LONGEST_WAIT))
    .unwrap_or(now);
  std::cmp::max(arrives, floor)
}

/// Which arm of the first `select!` produced a command to dispatch. A bare
/// `Command` carries no provenance, and the conditional re-arm after a
/// refusal (EX-7) has nothing to be true of without this discriminant: a
/// refused scheduled firing must re-arm at the floor rather than spin
/// (SPEC-002/R-4, R-8), and a refusal off any other arm must leave the
/// standing deadline alone.
#[derive(Debug)]
enum Fired {
  Command(Command),
  Scheduled,
  /// One arrival, still to be judged. It carries an `Arrival` and never an
  /// `Option<Arrival>`: a closed channel is disposed of in the arm that
  /// observed it, before any `Fired` is built, so nothing below this point
  /// has to ask whether ingress is still alive (`design.md` §5.2).
  Ingested(Arrival),
}

/// Whether the event spacing has elapsed at `now`.
///
/// `>=`, so a writer arriving *exactly at* the anchor is accepted. That is
/// forced by SPEC-003/R-14 rather than chosen: `retry_after_ms` is rounded
/// **up**, so a writer that waits exactly as long as it was told arrives at or
/// after the floor, and a strict comparison would make R-14's own sentence —
/// *after which the spacing will have elapsed* — false of the host's own
/// field. It is also what makes an anchor initialised to `started` mean
/// "already elapsed" at that instant.
///
/// A named function rather than an inline comparison because no case reaching
/// the host over a socket can tell the two directions apart — rounding up plus
/// a real sleep's overshoot puts every end-to-end waiter strictly past the
/// floor — so the boundary is reachable only by constructing it, which is what
/// this crate's own `#[cfg(test)] mod tests` is for.
fn spacing_elapsed(now: tokio::time::Instant, floor: tokio::time::Instant) -> bool {
  now >= floor
}

/// Answer one arrival's writer, and fold the same refusal onto the
/// diagnostics surface.
///
/// The two are one act: SPEC-003/R-8 owes the writer a reply and R-15 owes a
/// person the same fact, and a refusal that did one without the other would be
/// a second route to one of them. Whether the fold is ever *presented* is the
/// arm's business, not this function's — the outer arm `continue`s to the top
/// of the loop and presents; the inner arm resumes waiting and `absorb`
/// supersedes it, which is R-15's bound (`design.md` §5.2).
fn refuse_arrival(controller: &mut Controller, answer: Answer, refusal: &Refusal) {
  answer.refused(refusal);
  controller.refuse(&folded(refusal));
}

/// One refusal as the diagnostics surface holds it. **The only author of an
/// ingress line**: the wire token comes off `Refusal::reason()` and the prose
/// off its `Display`, so the eight-token set has exactly one speller here as
/// well as on the wire (`review-code.md` F-4). `ingress_stopped` below is the
/// second caller, and it exists because that refusal has no writer to answer —
/// not because it needs a second vocabulary.
fn folded(refusal: &Refusal) -> Refused {
  Refused::Ingress {
    reason: refusal.reason().to_owned(),
    detail: refusal.to_string(),
  }
}

/// The **inner** arm's judgement: `design.md` §5.4's steps 1 and 2, the only
/// two an arm reached during an exchange can decide. Shape first — an exchange
/// in flight does not turn a malformed envelope into `engaged` (SPEC-003 §5,
/// *Order of judgement*). Nothing here reads or writes either anchor, which is
/// what keeps `event_floor_until`'s *one write site* exact (I-4, P-3).
fn refuse_during_exchange(controller: &mut Controller, arrival: Arrival) {
  let (result, answer) = arrival.into_parts();
  let refusal = match result {
    Err(shape) => shape,
    Ok(_event) => Refusal::Engaged,
  };
  refuse_arrival(controller, answer, &refusal);
}

/// The one refusal that answers no envelope: the accept task has ended, and
/// nothing restarts it, so ingress is over for the life of the process and
/// this surface is the only report it has (`design.md` §5.2, SPEC-003/R-15).
///
/// Folded once. `Ingress::arrival` drops the receiver as it yields `None`, so
/// the arm parks from then on rather than spinning on a closed channel.
fn ingress_stopped() -> Refused {
  folded(&Refusal::Unavailable(UnavailableCause::IngressStopped))
}

/// The **outer** arm's judgement, whole: `design.md` §5.4's steps 1 and 3-5,
/// the steps only an arm reached while nothing is in flight can decide.
///
/// `None` is *there is nothing to exchange* — every refusal it decides has
/// already been answered to its writer and folded onto the surface, which is
/// why the loop `continue`s on it rather than passing it to the shared refusal
/// site (which would fold it a second time and consult `refusal_re_arms`).
///
/// **The anchor's one write site.** It is written on an *attempted* ingested
/// evaluation — steps 4 and 5 both, so a clock that cannot be read produces one
/// refusal per spacing rather than a spin (`design.md` §5.4) — and by nothing
/// else. `floor_until` is not named here and this is not named there
/// (SPEC-002/R-12, P-3).
fn ingest(
  arrival: Arrival,
  controller: &mut Controller,
  event_floor_until: &mut tokio::time::Instant,
  clock: Clock,
) -> Option<Pending> {
  let (result, answer) = arrival.into_parts();
  // 1 — a shape refusal is refused with itself, whatever the host's state.
  let event = match result {
    Ok(event) => event,
    Err(refusal) => {
      refuse_arrival(controller, answer, &refusal);
      return None;
    }
  };
  let arrived = tokio::time::Instant::now();
  // 3 — inside the spacing. Nothing was attempted, so nothing is written.
  if !spacing_elapsed(arrived, *event_floor_until) {
    let retry_after = event_floor_until.saturating_duration_since(arrived);
    refuse_arrival(controller, answer, &Refusal::TooSoon { retry_after });
    return None;
  }
  // `checked_add`, following the rule `deadline_after` states above: `Instant
  // + Duration` panics on overflow, and a panic here takes the host down. The
  // fallback leaves the anchor at `arrived`, so the spacing degenerates rather
  // than the process ending.
  *event_floor_until = arrived.checked_add(MINIMUM_SPACING).unwrap_or(arrived);
  match stamp(clock) {
    // 4 — the clock is unreadable. `unavailable` to the writer; the fold names
    // the clock, which is the same line a scheduled firing writes for the same
    // fault.
    Err(refused) => {
      answer.refused(&Refusal::Unavailable(UnavailableCause::ClockUnreadable));
      controller.refuse(&refused);
      None
    }
    // 5 — accepted. The reply leaves **before** the backend is called: the
    // listener awaits it before accepting the next connection, so a reply that
    // waited for the exchange would put `engaged` out of reach (I-2,
    // `design.md` §5.4).
    Ok(now) => {
      answer.accepted();
      Some(Pending::Evaluate { now, event })
    }
  }
}

/// One command, dispatched. Three of the five are done here and now and
/// produce no exchange — the two diagnostics commands, and an edit, which
/// writes retained state and is answered by the present at the top of the
/// next iteration. The other two produce a `Pending`, or the refusal that
/// says why there is none. `None` is *there is nothing to exchange*, which is
/// the loop's `continue`; an edit is the one command that can yield either
/// `None` or a refusal.
///
/// Lifted out of `serve` so that the ingested road and the command road meet
/// at one value: an arrival cannot become a `Command` — `Stimulus` is `Copy`
/// and hard-codes `source: "host"` (`wire.rs:62-64`, D-13) — so the join has to
/// be the `Pending` both roads produce.
fn dispatch(
  command: Command,
  controller: &mut Controller,
  clock: Clock,
) -> Option<Result<Pending, Refused>> {
  // Exhaustive, no `_` arm. Identity is checked before the clock: a superseded
  // click is refused for the reason that is true of it, and a broken clock does
  // not relabel it.
  match command {
    Command::OpenDiagnostics => {
      controller.open_diagnostics();
      None
    }
    Command::CloseDiagnostics => {
      controller.close_diagnostics();
      None
    }
    Command::Evaluate(stimulus) => Some(stamp(clock).map(|now| Pending::Evaluate {
      now,
      event: stimulus.event(now),
    })),
    Command::Choose {
      view,
      option,
      edits,
    } => Some(
      controller
        .choose(&view, &option, &edits)
        .and_then(|(view_id, answer)| {
          stamp(clock).map(|now| Pending::Respond {
            now,
            view_id,
            answer,
          })
        }),
    ),
    // An edit is not an exchange: it writes retained state and the loop
    // continues to the top, which presents and so writes the screen back from
    // the draft. `None` on success for that reason, and no clock is read —
    // nothing is being stamped. A refusal takes the single existing refusal
    // site, where `refusal_re_arms` is `false` by construction exactly as it
    // is for `Choose`.
    Command::Edit {
      view,
      option,
      field,
      reported,
    } => controller
      .edit(&view, &option, &field, &reported)
      .err()
      .map(Err),
  }
}

/// Serve commands until stopped. **This is the production controller**: a
/// `spawn_local` block around it is one line (PHASE-08's), and the cheap
/// test tier drives the identical call under `block_on` (D9). There is no
/// second implementation of the loop and no test-only harness for it.
///
/// An ordinary `async fn`, and **not** one silencing
/// `clippy::future_not_send`: that lint does not reach this signature at all,
/// because it drops `Send` obligations that mention a type parameter at the
/// top level and `serve`'s future is `!Send` only through `B` and `G`
/// (design.md §5.5, A-5, measured). The one attribute below says nothing
/// about `Send`; it is about the parameter count.
///
/// Everything is taken by value because `slint::spawn_local` needs a
/// `'static` future, and handed back in `Served` so a test can read what it
/// did.
#[expect(
  clippy::too_many_arguments,
  reason = "each parameter is a distinct owned resource, taken by value because \
            `slint::spawn_local` needs a `'static` future and handed back in \
            `Served` so a test can read what it did. The eighth is `notice`, \
            which `design.md` §5.3 puts here alongside `cancel` — the two edge \
            signals travel the same route. Grouping any of them would be an \
            abstraction that exists only to satisfy the count."
)]
pub async fn serve<B, G>(
  mut host: Host<B>,
  mut controller: Controller,
  mut commands: mpsc::Receiver<Command>,
  cancel: Cancel,
  notice: Notice,
  clock: Clock,
  mut glass: G,
  mut ingress: Ingress,
) -> Served<B, G>
where
  B: Backend + 'static,
  G: Glass + 'static,
{
  let started = tokio::time::Instant::now();
  // Always armed: if the startup evaluation is never dispatched or is
  // refused, this initial arm fires at `MINIMUM_SPACING` and the host
  // recovers by itself (SPEC-002/R-1 and R-8's last sentence).
  let mut sleep = Box::pin(tokio::time::sleep_until(started + MINIMUM_SPACING));
  // Nothing scheduled has fired yet, so nothing is floored: the first
  // scheduled firing of the process is unfloored, which is what lets a
  // `default_poll` shorter than the spacing be honoured once (SPEC-002 §6).
  let mut floor_until = started;
  // The event anchor, and the whole of this slice's new retained state
  // (`design.md` §5.3). It starts **already elapsed**, exactly as
  // `floor_until` does and for the same reason: each class is spaced from the
  // previous firing of its own class (SPEC-002/R-12, P-3), and there is no
  // previous ingested firing. The one other candidate,
  // `started + MINIMUM_SPACING`, is precisely the value this would hold if the
  // startup evaluation had written it — and the startup evaluation is not an
  // ingested firing. So the startup evaluation never makes an envelope
  // `too_soon`; an envelope arriving while it is still in flight is refused
  // `engaged` one step earlier, like any other.
  let mut event_floor_until = started;

  let ending = 'serving: loop {
    // **Drained before the present, and that order is the whole of it**
    // (`review-code.md` F-R3). `Debounce::tick` drops its entry the instant
    // `Wire::send` reports the command *enqueued* (`pending.rs`), so between
    // that enqueue and this loop serving the command, a person's keystrokes
    // are held by neither the overlay nor the draft. A present landing inside
    // that interval writes the pre-typing value back over the widget they are
    // typing into — and it lands there routinely, because the command is
    // enqueued while this loop is parked inside an exchange and the first
    // thing it does on coming out of one is present.
    //
    // So: apply every queued command that resolves without an exchange, and
    // stop at the first that needs one. `dispatch` is the same entry point the
    // `select!` below uses, so nothing is dispatched twice and nothing is
    // dropped; what it yields travels to the same `attempted` either way.
    //
    // **This bypasses the `biased` `cancel.stopped()` arm for one command,
    // and that is harmless.** The only drained command that reaches the
    // backend is a `Choose`, and the inner `select!` it lands in is `biased`
    // on `cancel.stopped()` too — so a stop already tripped breaks there and
    // drops `call` before it is ever polled, and no subprocess is spawned.
    let mut drained = None;
    while drained.is_none() {
      let Ok(command) = commands.try_recv() else {
        break;
      };
      drained = dispatch(command, &mut controller, clock);
    }

    glass.present(controller.frame(notice.raised())); // busy = false here

    // A drained command never came from the timer arm, so `refusal_re_arms`
    // is `false` for it by the same argument the refusal site below makes:
    // the flag is `matches!(fired, Fired::Scheduled)`, and only the timer arm
    // builds a `Fired::Scheduled`.
    let (attempted, refusal_re_arms) = if let Some(drained) = drained {
      (Some(drained), false)
    } else {
      let fired = select! { biased;
        () = cancel.stopped()       => break Ending::Stopped,
        received = commands.recv()  => match received {
          None          => break Ending::Closed,
          Some(command) => Fired::Command(command),
        },
        () = &mut sleep => {
          // The one and only write site: the spacing is measured between
          // scheduled firings and nothing else clears it (SPEC-002/R-4).
          floor_until = tokio::time::Instant::now() + MINIMUM_SPACING;
          Fired::Scheduled
        },
        // **Last**, so that a watcher emitting at machine rate cannot starve a
        // scheduled firing: `biased` means an always-ready arm starves
        // everything below it, and this is the arm an untrusted writer paces
        // (`design.md` §5.4).
        arrival = ingress.arrival() => match arrival {
          // Disposed of here, before any `Fired` is built: `refusal_re_arms` is
          // never reached, the standing deadline is not reset, and neither
          // anchor is written. A dead accept task changes nothing about the
          // schedule.
          None => {
            controller.refuse(&ingress_stopped());
            continue;
          }
          Some(arrival) => Fired::Ingested(arrival),
        },
      };
      // A refusal that came from the timer arm re-arms at the floor (EX-7);
      // every other refusal leaves the deadline untouched. Read before `fired`
      // is consumed below, and true of the whole iteration.
      let refusal_re_arms = matches!(fired, Fired::Scheduled);

      // Two roads, one exchange. An arrival becomes a `Pending::Evaluate`
      // directly, because `Stimulus` cannot carry an event (D-13); a command
      // takes the road it always has. `None` from either is *there is nothing to
      // exchange*.
      let attempted = match fired {
        Fired::Ingested(arrival) => {
          ingest(arrival, &mut controller, &mut event_floor_until, clock).map(Ok)
        }
        Fired::Command(command) => dispatch(command, &mut controller, clock),
        // Goes through the same `stamp` as every other command (EX-6).
        Fired::Scheduled => dispatch(
          Command::Evaluate(Stimulus::Scheduled),
          &mut controller,
          clock,
        ),
      };
      (attempted, refusal_re_arms)
    };
    // A diagnostics command, an edit the drain applied, or an arrival `ingest`
    // has already answered and folded. None of them has anything to send, and
    // none is a refusal this loop still owes a report for.
    let Some(attempted) = attempted else {
      continue;
    };

    // The one refusal site. It `continue`s to the top — which presents with
    // `busy = false`, and carries the back-pressure signal through like any
    // other present rather than clearing it — and re-arms only when this
    // iteration came from the timer arm (EX-7).
    //
    // One site rather than three: `refusal_re_arms` is `false` by
    // construction under `Command::Choose` and under `Command::Edit` alike,
    // because `Fired::Scheduled` becomes `Command::Evaluate(Stimulus::Scheduled)`
    // and nothing else, so the copies that used to sit inside that arm could
    // never run. Three copies of a conditional, two of them unreachable, say
    // the flag is orthogonal to the command when it is fully determined by
    // it.
    let pending = match attempted {
      Ok(pending) => pending,
      Err(refused) => {
        controller.refuse(&refused);
        if refusal_re_arms {
          sleep.as_mut().reset(floor_until);
        }
        continue;
      }
    };
    let requested_at = pending.now();
    let exchanged = pending.exchanged();

    controller.engage(exchanged);
    // `busy` iff this is the person's own answer, in which case the option
    // buttons are disabled and the rest of the form with them; an evaluation
    // presents with every control live (F-A1, F-R2).
    glass.present(controller.frame(notice.raised()));

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

    // A **loop**, so that refusing an arrival resumes waiting on the *same*
    // exchange: `call` is pinned across iterations and nothing here
    // re-invokes the backend. The outer loop's label is what keeps
    // cancellation breaking all the way out (`design.md` §5.4).
    let mut call = std::pin::pin!(call);
    loop {
      select! { biased;
        () = cancel.stopped()  => break 'serving Ending::Stopped, // `call` is DROPPED here
        outcome = &mut call    => {
          let absorbed = controller.absorb(exchanged, outcome);
          let wait = wait_for(absorbed.next_check, requested_at);
          sleep
            .as_mut()
            .reset(deadline_after(tokio::time::Instant::now(), wait, floor_until));
          break;
        },
        // **Last**, so a flood of arrivals the loop is only going to refuse
        // cannot starve the exchange it is waiting on. This arm reaches
        // §5.4's step 2 and stops: it neither reads nor writes either anchor.
        arrival = ingress.arrival() => match arrival {
          // **Presented here, and only on this branch.** This is the one
          // refusal that answers no envelope, so the surface is the only
          // report it has (SPEC-003/R-15) — and the arm this loop exits by
          // calls `absorb`, which replaces the whole retained `Diagnostics`,
          // so a fold left to the outer loop's own presentation is guaranteed
          // to be gone before any frame carries it (`review-code.md` F-3).
          //
          // It costs one presentation per **process**, not per refusal:
          // `Ingress::arrival` parks the arm as it yields `None`, so this
          // branch is reached at most once and no writer can reach it at all.
          // The `Some` branch below is the one an untrusted writer paces, and
          // it still presents nothing — `review-design.md` F-15's measured
          // cost and R-15's negative case both live there.
          None => {
            controller.refuse(&ingress_stopped());
            glass.present(controller.frame(notice.raised()));
          }
          Some(arrival) => refuse_during_exchange(&mut controller, arrival),
        },
      }
    }
  };
  Served {
    ending,
    host,
    controller,
    glass,
    ingress,
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
  use super::{Refused, deadline_after, spacing_elapsed, stamp};
  use goad_shell::clock::{ClockError, wall_clock};

  #[test]
  fn an_ordinary_wait_is_the_sum() {
    let now = tokio::time::Instant::now();
    let wait = std::time::Duration::from_secs(90);
    assert_eq!(deadline_after(now, wait, now), now + wait);
  }

  #[test]
  fn a_floor_later_than_the_sum_wins() {
    let now = tokio::time::Instant::now();
    let floor = now + std::time::Duration::from_secs(3);
    let deadline = deadline_after(now, std::time::Duration::from_millis(100), floor);
    assert_eq!(deadline, floor);
  }

  /// The whole point of the function. `wait_for` is total across jiff's
  /// range, so a backend instructing a `next_check` at the far edge of
  /// representable time hands the loop a wait of ~10^11 seconds — and
  /// `Instant + Duration` panics on overflow rather than returning an
  /// `Option`. A panic here is a backend failure taking the host down.
  #[test]
  fn a_wait_that_would_overflow_the_clock_is_clamped_rather_than_panicking() {
    let now = tokio::time::Instant::now();
    let deadline = deadline_after(now, std::time::Duration::MAX, now);
    assert_eq!(
      deadline,
      now + super::LONGEST_WAIT,
      "an overflowing wait is clamped, not panicked on"
    );
  }

  /// VT-8 — the spacing's boundary, and the only case that can tell `>=` from
  /// `>` apart. No end-to-end case can: `retry_after_ms` rounds **up**
  /// (SPEC-003/R-14) and a real waiter's `sleep` overshoots on top of that, so
  /// a writer that waits exactly as long as it was told arrives strictly past
  /// the floor and both comparisons accept it. The boundary instant is
  /// reachable only by constructing it, which is what this module is for
  /// (`deadline_after` and `stamp` are the same argument).
  ///
  /// No clock, no socket, no `serve`.
  #[test]
  fn a_writer_arriving_exactly_at_the_anchor_is_outside_the_spacing() {
    let floor = tokio::time::Instant::now() + std::time::Duration::from_secs(1);
    let nanosecond = std::time::Duration::from_nanos(1);

    assert!(
      spacing_elapsed(floor, floor),
      "at the anchor the spacing has elapsed: R-14 rounds `retry_after_ms` up, \
       so a writer that waited exactly that long arrives here and must be accepted"
    );
    assert!(
      !spacing_elapsed(floor - nanosecond, floor),
      "one nanosecond before the anchor is still inside the spacing"
    );
    assert!(
      spacing_elapsed(floor + nanosecond, floor),
      "one nanosecond after the anchor is outside it"
    );
  }

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

//! The debounce — `design.md` §5.1, §5.4. Stratum 3.
//!
//! A map of edits a control has raised and the host has not been told about
//! yet, and the one timer that tells it. **Keyed by (option, field)**, because
//! a person who types into one field and moves to another inside the window
//! would otherwise lose the first field's last keystrokes, and D-8 forbids
//! flushing on the switch — so the only place left to hold them is here.
//!
//! **It does not branch on kind.** An entry is a `Reported`; this module
//! neither reads it nor cares which control produced it. Which controls route
//! through the map is `design.md` §5.2's table — `text` and both `number`
//! controls, the ones a person changes continuously. `boolean`, `choice` and
//! `datetime` raise one discrete edit and it is sent where it is raised.
//!
//! **Every entry carries the view it was made on**, because an entry outlives
//! the view that produced it and the map is keyed by strings a replacement view
//! is free to reuse. `design.md` §5.5 **I-H** is one rule at three sites: an
//! entry is *shown* only while its view is the one being presented, *sent* by
//! the timer in a command carrying that view, and *drained* into a `Choose`
//! carrying that view alongside. This module owns the last two; the glass owns
//! the first.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::Duration;

use crate::draft::Reported;
use crate::wire::{Command, PendingEdit, Wire};

/// How long a control may keep changing before the host is told (D-4).
const DEBOUNCE: Duration = Duration::from_millis(150);

/// One edit, and the view it was raised on.
///
/// The view is not derivable from anything else the map holds: the key is a
/// pair of backend-supplied strings, and a replacement view may declare the
/// same pair. The `edited` callback already receives the view as its first
/// argument, so carrying it costs a field and no new plumbing — and it is the
/// only honest source for the `view` a deferred `Command::Edit` has to carry.
#[derive(Debug, Clone)]
struct Held {
  view: String,
  value: Reported,
}

/// The pending edits, and the timer that delivers them.
///
/// Two fields and no more, which is `design.md` §5.1 in as many words. The
/// `Wire` is **not** one of them: it is passed to each call, so this module
/// holds no way to reach the loop of its own accord and the ownership question
/// stays where `install.rs` already answers it. The armed timer's callback
/// does close over a `Wire` clone, which is unavoidable — a timer that
/// delivers has to reach the channel — and is the timer's own state rather
/// than a third field on this type.
///
/// Every method that arms takes `&Rc<Self>`, so the timer's callback can hold
/// the map it will read; the two an enclosing present or callback needs —
/// [`Self::carried`] and [`Self::delivered`] — take `&self`, so PHASE-06's
/// overlay reads it without one.
///
/// **Not `Pending`.** `controller.rs:408` already declares a private
/// `enum Pending` — the exchange a command turns into — and two private types
/// of that name in one crate, one of them behind an `Rc`, is a readability
/// trap. This is named for the mechanism every document in the slice calls it
/// by (`design.md` §5.1, *"`pending.rs` holds the debounce"*).
#[derive(Default)]
pub struct Debounce {
  /// `BTreeMap` rather than a hash map so that *the entry the timer takes* is
  /// a fact about the map rather than about an iteration order nothing pins.
  /// No order is promised over the **drained** edits and none is needed — the
  /// keys are distinct by construction, so any order yields the same draft
  /// (`design.md` §5.2).
  held: RefCell<BTreeMap<(String, String), Held>>,
  timer: slint::Timer,
}

/// By hand, because `slint::Timer` implements no `Debug` and
/// `missing_debug_implementations` is denied workspace-wide. It reports the
/// keys and the view each entry was made on — the two things a failure message
/// needs — and not the `Reported` values, which a person's typing is.
impl std::fmt::Debug for Debounce {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Debounce")
      .field(
        "held",
        &self
          .held
          .borrow()
          .iter()
          .map(|((option, field), entry)| (option.clone(), field.clone(), entry.view.clone()))
          .collect::<Vec<_>>(),
      )
      .finish_non_exhaustive()
  }
}

impl Debounce {
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Hold what one control just reported, and restart the window.
  ///
  /// The restart is on **every** keystroke, which is what makes the window
  /// *since your last pause* rather than 150 ms flat. Replacing an entry for a
  /// key that is already held is the intent: only the latest value of a field
  /// is worth sending, and the earlier one was never the person's answer.
  pub fn hold(
    self: &Rc<Self>,
    view: &str,
    option: &str,
    field: &str,
    value: Reported,
    wire: &Wire,
  ) {
    self.held.borrow_mut().insert(
      (option.to_owned(), field.to_owned()),
      Held {
        view: view.to_owned(),
        value,
      },
    );
    self.arm(wire);
  }

  /// Everything held, as the edits a `Command::Choose` carries — **without
  /// clearing anything**.
  ///
  /// The clear is [`Self::delivered`]'s, called only once the send that
  /// carries these has been enqueued. Splitting the two is the whole of §5.1's
  /// asymmetry: a `Full` send delivered nothing, so a drain that cleared as it
  /// read would lose the edits and the widgets showing them.
  #[must_use]
  pub fn carried(&self) -> Vec<PendingEdit> {
    self
      .held
      .borrow()
      .iter()
      .map(|((option, field), entry)| PendingEdit {
        view: entry.view.clone(),
        option: option.clone(),
        field: field.clone(),
        value: entry.value.clone(),
      })
      .collect()
  }

  /// The `Choose` that carried [`Self::carried`]'s edits was enqueued, so they
  /// are gone.
  ///
  /// It clears the whole map rather than removing the keys it was handed,
  /// because nothing can have been added since: both calls happen inside one
  /// synchronous Slint callback, on the UI thread, with no await between them.
  pub fn delivered(&self) {
    self.held.borrow_mut().clear();
  }

  /// Arm, or re-arm, the one timer.
  ///
  /// A `slint::Timer` may be restarted from inside its own callback:
  /// `start_or_restart_timer` preserves the `being_activated` flag and replaces
  /// the callback, and `maybe_activate_timers` puts the old callback back only
  /// where the callback did not restart its own timer
  /// (`i-slint-core-1.17.1/timers.rs:348-372`, `:330-334`). Read from the
  /// locked source and then measured under a real loop, because the whole
  /// delivery rule rests on it (`prototype-handback.md` P-11). The negative
  /// control is `tests/event_loop_debounce/`.
  fn arm(self: &Rc<Self>, wire: &Wire) {
    let holding = Rc::clone(self);
    let sending = wire.clone();
    self
      .timer
      .start(slint::TimerMode::SingleShot, DEBOUNCE, move || {
        holding.tick(&sending);
      });
  }

  /// One tick: send **one** entry, and re-arm while the map is not empty.
  ///
  /// One per tick is not a policy choice. The command channel holds one
  /// (`main.rs:86`) and `serve` shares the UI thread through `spawn_local`, so
  /// one command per tick is the most that is ever available. It costs nothing
  /// in correctness: the answer path drains whatever is left in a single
  /// command, so nothing waits on the timer to be *right*, only to be *early*
  /// (`design.md` §5.4).
  ///
  /// **The entry leaves on the enqueue, not on acceptance.** A refusal has
  /// already been reported and the guard corrects the widget on the next
  /// present; a `Full` send delivered nothing, so the entry must stand and be
  /// offered again on the next tick. That is why `Wire::send` reports at all
  /// (`design.md` §5.1, §5.2).
  ///
  /// The borrow is dropped before the send and taken again after it. Nothing
  /// re-enters this module from a `try_send` today, and the shape says so
  /// rather than relying on it.
  fn tick(self: &Rc<Self>, wire: &Wire) {
    let next = self
      .held
      .borrow()
      .iter()
      .next()
      .map(|(key, entry)| (key.clone(), entry.clone()));

    if let Some(((option, field), entry)) = next {
      let enqueued = wire.send(Command::Edit {
        view: entry.view,
        option: option.clone(),
        field: field.clone(),
        reported: entry.value,
      });
      if enqueued {
        self.held.borrow_mut().remove(&(option, field));
      }
    }

    if !self.held.borrow().is_empty() {
      self.arm(wire);
    }
  }
}

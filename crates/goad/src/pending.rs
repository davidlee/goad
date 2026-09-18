//! The debounce — design.md §5.1, §5.3.
//!
//! A map of pending edits keyed by (option, field), one [`slint::Timer`], and
//! nothing else. It does not branch on kind: an entry is a
//! [`Reported`](crate::draft::Reported) and this module neither reads it nor
//! cares which control produced it. Which controls route through here is
//! §5.2's table and is `install.rs`'s decision — `text` and both `number`
//! controls, because those are the ones a person changes continuously.
//!
//! **Keyed, not singular.** A person who types into one field and moves to
//! another inside the window would otherwise lose the first field's last
//! keystrokes: D-8 forbids flushing on the switch, so the only place left to
//! hold them is here.
//!
//! **Four participants, and none of them nests** (§5.3, *Nothing re-enters*).
//! The `edited` callback writes; the timer callback and `chosen` take;
//! `present` reads. `present` runs inside `serve`'s own task and never from
//! inside a widget callback, and neither the timer nor `chosen` presents —
//! both only enqueue. Every method below ends its borrow of the map before it
//! calls anything, so even a participant that did re-enter would find the cell
//! free.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use slint::{Timer, TimerMode};

use crate::draft::Reported;
use crate::wire::{Command, PendingEdit, Wire};

/// How long an edit waits before it becomes a command. D-4, a standing user
/// commitment: 150 ms is long enough that a keystroke run makes one send and
/// short enough that a person who stops typing sees the host agree with them
/// before they look away.
const DEBOUNCE: Duration = Duration::from_millis(150);

/// Every edit the host is holding, and the one timer that delivers them.
///
/// Lives behind an `Rc` shared by the callbacks and the glass. **It has to be
/// the same `Rc`**, and getting that wrong is silent: two `Pending` values
/// give an overlay that never overlays anything, with every case still green
/// and nothing measured (§8 R10). `main.rs` creates it once, before the
/// callback table.
#[derive(Default)]
pub struct Pending {
  /// Keyed by (option, field), in a `Vec` for the reason `Draft` is: a view's
  /// options and a form's fields are both handfuls, and `OptionId` is
  /// deliberately not `Ord`. The entries are `PendingEdit`s outright, because
  /// that is exactly what a `Command::Choose` carries — a second struct here
  /// would be a translation between two spellings of one value.
  entries: RefCell<Vec<PendingEdit>>,
  timer: Timer,
}

/// Hand-written because `slint::Timer` carries no `Debug`, by derive or by
/// impl, and `missing_debug_implementations` is `deny` — the same reason
/// `SlintGlass` writes one.
impl fmt::Debug for Pending {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("Pending")
      .field("entries", &self.entries.borrow().len())
      .finish_non_exhaustive()
  }
}

impl Pending {
  /// The one constructor. `Rc` in the return type rather than at every call
  /// site, because there is no use for a `Pending` that is not shared: the
  /// timer callback holds one handle and the glass holds another.
  #[must_use]
  pub fn new() -> Rc<Self> {
    Rc::new(Self::default())
  }

  /// Hold one edit, replacing whatever that (option, field) held, and start
  /// the window again.
  ///
  /// Restarting rather than letting the first window run out is what
  /// *debounce* means: a person still typing has not finished, and the send
  /// they want is the one carrying their last keystroke.
  pub fn record(self: &Rc<Self>, wire: &Wire, edit: PendingEdit) {
    {
      let mut entries = self.entries.borrow_mut();
      entries.retain(|held| held.option != edit.option || held.field != edit.field);
      entries.push(edit);
    }
    self.arm(wire);
  }

  /// The entry for this field **made on this view**, if there is one.
  ///
  /// The view check is I-H at its first site: an entry outlives the view that
  /// produced it, and the ids it is keyed by are strings a replacement view is
  /// free to reuse, so without this check a stale entry could be written into
  /// a new view's widget.
  #[must_use]
  pub fn shown(&self, view: &str, option: &str, field: &str) -> Option<Reported> {
    self
      .entries
      .borrow()
      .iter()
      .find(|held| held.option == option && held.field == field && held.view == view)
      .map(|held| held.value.clone())
  }

  /// Hand every entry to `send` as one command's worth, and take them back
  /// where the send did not enqueue it.
  ///
  /// **Every** entry, not the presented view's: a stale one carries its own
  /// view and the controller refuses it `SupersededView`, which is how the
  /// person is told their typing was discarded (§5.5's edge table). Filtering
  /// here would leave that row unreachable.
  ///
  /// The borrow is released before `send` runs, and the entries are put back
  /// only if it reports the command was not enqueued — an entry leaves the map
  /// when the send that carries it is **enqueued**, not when it is accepted.
  pub fn flush(&self, send: impl FnOnce(Vec<PendingEdit>) -> bool) {
    let drained = std::mem::take(&mut *self.entries.borrow_mut());
    let restored = drained.clone();
    if !send(drained) {
      // Nothing was delivered, so a second click must answer with the same
      // edits still attached — and the widgets still show them meanwhile,
      // because the entries are back here to be overlaid. A plain assignment
      // rather than a merge: `send` enqueues and nothing else, so no callback
      // can have written the map in between.
      *self.entries.borrow_mut() = restored;
    }
  }

  /// Start the window. One timer for the whole map, restarted by every edit.
  fn arm(self: &Rc<Self>, wire: &Wire) {
    let holder = Rc::clone(self);
    let sender = wire.clone();
    self.timer.start(TimerMode::SingleShot, DEBOUNCE, move || {
      holder.tick(&sender);
    });
  }

  /// Deliver **one** entry and re-arm while the map is not empty.
  ///
  /// One per tick is what the capacity-one command channel leaves available,
  /// not a choice made here: a Slint callback is synchronous and `serve`
  /// shares the UI thread, so a second `try_send` from inside one callback is
  /// certain of `Full`. It costs nothing in correctness — the answer drains
  /// whatever is left in one command, so nothing waits on the timer to be
  /// *right*, only to be *early*.
  ///
  /// **Re-arming from inside the callback is the timer's own documented
  /// behaviour**, read from the locked source rather than assumed:
  /// `start_or_restart_timer` carries the old `being_activated` flag onto the
  /// replacement (`i-slint-core-1.17.1/timers.rs:365-370`), and
  /// `maybe_activate_timers` puts the previous callback back only where the
  /// permanent store is still `Empty` — "if not, it means the invoked callback
  /// has restarted its own timer with a new callback" (`:328-334`). The whole
  /// re-arm rests on that.
  fn tick(self: &Rc<Self>, wire: &Wire) {
    let next = self.entries.borrow().first().cloned();
    if let Some(edit) = next {
      let carried = Command::Edit {
        view: edit.view.clone(),
        option: edit.option.clone(),
        field: edit.field.clone(),
        value: edit.value.clone(),
      };
      if wire.send(carried) {
        self
          .entries
          .borrow_mut()
          .retain(|held| held.option != edit.option || held.field != edit.field);
      }
    }
    if !self.entries.borrow().is_empty() {
      self.arm(wire);
    }
  }
}

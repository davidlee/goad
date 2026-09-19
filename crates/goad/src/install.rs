//! `crates/goad/src/install.rs` — stratum 3.
//!
//! `pub`, and in the library rather than in `main.rs`, because validation
//! item 14e drives it from a `tests/` target and a `tests/` target cannot
//! reach a binary crate or a private item (D28, F-30). `install` returns
//! `()`: every setter it calls is infallible (design.md §5.4).

use std::cell::Cell;
use std::rc::Rc;

use slint::platform::WindowEvent;
use slint::{CloseRequestResponse, ComponentHandle, Weak};

use crate::draft::Reported;
use crate::generated::{FieldEdit, Kind, PromptWindow, Tray};
use crate::instant;
use crate::pending::Debounce;
use crate::wire::{Command, Stimulus, Wire};
use crate::zoom::Zoom;

/// One function, seven installations, each owning its own `Wire` clone and
/// nothing else. Seven distinct binding names rather than seven
/// `let wire = wire.clone();` — the reason is readability, not the lint
/// table: naming each clone after the callback it feeds says which
/// callback owns which handle (design.md §5.4).
///
/// `pending` is the debounce, created by the caller because `glass.rs` is
/// given a clone of the **same** handle (PHASE-06): one value, cloned into two
/// places, never two values (`design.md` §9, R10). Two of the callbacks below
/// reach it — `edited` writes, `chosen` drains — and nothing else in this
/// function has any use for it.
pub fn install(window: &PromptWindow, tray: &Tray, wire: &Wire, pending: &Rc<Debounce>) {
  // **One send, carrying the flush.** The command channel holds one and a
  // Slint callback cannot yield, so a flush made of separate sends loses
  // everything after the first — not sometimes, always. The edits are read out
  // *before* the send and cleared only once it reports the command enqueued: a
  // `Full` send delivered nothing, and a second click must answer with the same
  // edits still attached (design.md §5.1, §5.4).
  let (chosen, choosing) = (wire.clone(), Rc::clone(pending));
  window.on_chosen(move |view, option| {
    let enqueued = chosen.send(Command::Choose {
      view: view.into(),
      option: option.into(),
      edits: choosing.carried(),
    });
    if enqueued {
      choosing.delivered();
    }
  });

  // The widget has already changed itself, so the report is what the person
  // now sees; the draft is what decides what they will see after the next
  // present. A dropped send is therefore visibly undone rather than silently
  // divergent (design.md §5.4).
  //
  // The closure does exactly two things: turn the `FieldEdit` into a
  // `Reported`, and route it. Everything else — the parse, the index, the
  // fallback, the refusal — happens in `controller::edit`, where the retained
  // presentation is.
  //
  // **Routing is not judgement about the value.** `debounced` reads the report's
  // own variant and nothing else; a held edit reaches the same
  // `controller::edit` a sent one does, one timer tick later or inside the
  // `Choose` that answers.
  let (editing, holding) = (wire.clone(), Rc::clone(pending));
  window.on_edited(move |view, option, field, edit| {
    let Some(report) = reported(&edit) else {
      return;
    };
    if debounced(&report) {
      holding.hold(&view, &option, &field, report, &editing);
      return;
    }
    editing.send(Command::Edit {
      view: view.into(),
      option: option.into(),
      field: field.into(),
      reported: report,
    });
  });

  let closing = wire.clone();
  // The three below discard `send`'s report deliberately: none of them holds
  // state that a dropped command would strand, and a `Full` send has already
  // raised the notice that explains it (`design.md` §5.4).
  window.on_close_diagnostics(move || {
    closing.send(Command::CloseDiagnostics);
  });

  // A built-in, not one of ours: closing the window quits, in either mode,
  // and the window is kept shown because the quit path is `serve`
  // returning (design.md §5.4).
  let quitting = wire.clone();
  window.window().on_close_requested(move || {
    quitting.stop();
    CloseRequestResponse::KeepWindowShown
  });

  let checking = wire.clone();
  tray.on_check_now(move || {
    checking.send(Command::Evaluate(Stimulus::Requested));
  });

  let showing = wire.clone();
  tray.on_show_diagnostics(move || {
    showing.send(Command::OpenDiagnostics);
  });

  let stopping = wire.clone();
  tray.on_quit(move || stopping.stop());

  // Magnification is the host's alone and never reaches `Command`, so it is
  // wired straight at the window rather than sent down the wire. One cell for
  // three items, because they are three views of one number (`zoom.rs`).
  let zoom = Rc::new(Cell::new(Zoom::NONE));

  let (magnifying, in_) = (window.as_weak(), Rc::clone(&zoom));
  tray.on_zoom_in(move || rescale(&magnifying, &in_, Zoom::larger));

  let (reducing, out) = (window.as_weak(), Rc::clone(&zoom));
  tray.on_zoom_out(move || rescale(&reducing, &out, Zoom::smaller));

  let (restoring, none) = (window.as_weak(), zoom);
  tray.on_zoom_reset(move || rescale(&restoring, &none, |_| Zoom::NONE));
}

/// What the widget said, in the widget's own terms — and **nothing else**.
///
/// The whole of the markup boundary's edit channel: one slot read per kind,
/// selected by the discriminant the control put there. There is no parse here,
/// no fallback value and no refusal, because a Slint callback has nothing to
/// report a refusal to and no draft to leave alone. `view_model::interpret`
/// makes every one of those judgements against the drawn field, and
/// `controller::edit` reports what it refuses (design.md §5.2, §7 D25).
///
/// `Kind::Text` reads `text` **verbatim**. A `LineEdit` admits every string —
/// `input-type` gates typing and nothing else, and `set_accessible_value` and
/// a paste reach no insertion logic at all — so there is nothing here that
/// could be validated even if this were the place for it (§8 R11, F-52).
///
/// **`None` now means two different things, and the `datetime` arm is why the
/// return type was never scaffolding.** For `number` and `choice` it is *no
/// control of this kind is drawn yet*: `view_model::drawn_form` draws
/// `boolean`, `text` and `datetime`, so a field of either other kind never
/// becomes a `FieldRow` and no control exists to raise this callback for one —
/// the arm is unreachable rather than declined. Each is filled by the phase
/// that draws its control: `number` PHASE-08, `choice` PHASE-09. A phase that
/// draws a control and forgets its arm here fails that phase's own first case
/// — the draft never sees the edit.
///
/// For `datetime` it means **the pick did not resolve**. The two pickers hand
/// back a civil date and time and `instant::compose` turns them into an instant
/// and the offset it resolved in — host-side, where it can fail: an
/// out-of-range integer, a civil date `Date::new` refuses, a `DateTime`
/// `to_zoned` refuses (`instant.rs:40-51`). That is §5.4's *"one `edited()`, or
/// nothing if `compose` fails"*, and nothing is recorded on that path: the
/// button still shows what it showed, which is the person's signal that the
/// pick did not take.
///
/// A fold or a gap is **not** on that path. jiff resolves both under
/// `Disambiguation::Compatible` — the fold takes the earlier occurrence, the
/// gap shifts forward — so an ambiguous civil time composes rather than
/// refusing, and the button then shows the instant it resolved to. A person
/// sees the shift instead of being deceived by it (§7 D19).
fn reported(edit: &FieldEdit) -> Option<Reported> {
  match edit.kind {
    Kind::Boolean => Some(Reported::Checked(edit.checked)),
    Kind::Text => Some(Reported::Typed(edit.text.to_string())),
    // The pair the **time** picker's `accepted` reported: `edit.date` is what
    // the date picker handed back one popup earlier, stashed on the window
    // root because nothing inside a popup survives its close (`ui/app.slint`,
    // design.md §5.4).
    Kind::Datetime => instant::compose(&edit.date, &edit.time)
      .map(|(instant, offset)| Reported::Picked { instant, offset }),
    Kind::Number | Kind::Choice => None,
  }
}

/// Whether this report goes through `pending.rs` rather than straight down the
/// wire.
///
/// **Which controls a person changes continuously**, and nothing else: a text
/// `LineEdit` raises `edited` per keystroke, a numeric one likewise, and a
/// `Slider` raises `changed` continuously through a drag. The other three raise
/// one discrete edit and there is nothing to debounce (`design.md` §5.2's
/// controls table, §7 D20).
///
/// It reads the report's variant and never the drawn field's kind, so the
/// decision belongs to the control that raised the edit rather than to a row
/// the host wrote — the same reason the markup's literals name their own kind
/// (§5.2). Total over `Reported`, no `_` arm: a seventh variant is a compile
/// error here rather than a control that silently stops being debounced.
fn debounced(report: &Reported) -> bool {
  match report {
    Reported::Typed(_) | Reported::AdjustedText(_) | Reported::AdjustedValue(_) => true,
    Reported::Checked(_) | Reported::Chosen(_) | Reported::Picked { .. } => false,
  }
}

/// Take one step and hand the window the scale factor it lands on.
///
/// `ScaleFactorChanged` is the compositor's own lever, so this magnifies
/// **everything the window draws** — the widget library's own padding, borders
/// and glyph metrics included. A `zoom` property multiplying lengths in the
/// markup would touch every literal in `app.slint` and still miss all of that.
///
/// The compositor's scale is divided back out of what the window reports
/// rather than remembered from startup; `Zoom::base_of` says why.
fn rescale(window: &Weak<PromptWindow>, zoom: &Cell<Zoom>, step: impl Fn(Zoom) -> Zoom) {
  let Some(window) = window.upgrade() else {
    return;
  };
  let was = zoom.get();
  let base = was.base_of(window.window().scale_factor());
  let now = step(was);
  zoom.set(now);
  window
    .window()
    .dispatch_event(WindowEvent::ScaleFactorChanged {
      scale_factor: now.applied_to(base),
    });
}

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

use crate::diagnostics::report_platform;
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
/// **`Kind::Number` is the one arm that reads a second slot to pick a slot.**
/// A `number` has two controls and they are the one pair of `Reported`
/// variants that share a kind, so `kind` alone cannot select between them —
/// and no slot *value* can either: an empty `text` is the measured
/// cleared-field case and must reach the draft as `AdjustedText("")`, so
/// *empty means the slider was at rest* would silently eat a real edit.
/// Guessing between them is an ambiguous message being guessed at rather than
/// failing, so the markup says which control it is and this reads `slider` and
/// **nothing else** (design.md §5.2, §7 D-38).
///
/// It reads the control's own literal, not the row's: `FieldRow.slider` is the
/// host's decision and `FieldEdit.slider` is the control's account of itself,
/// and the two agreeing is a fact about a correct renderer rather than a
/// tautology. `D-12` is untouched — `kind` still says `number` for both
/// controls, which is why `slider` is a second field and not a sixth `Kind`.
///
/// **`None` now means one thing, and that is a narrowing this phase made.** It
/// was two: *no control of this kind is drawn yet*, which was `choice`'s arm
/// until PHASE-09 drew it, and *the pick did not resolve*. All five kinds draw,
/// so the first reading has no arm left.
///
/// `None` means **the pick did not resolve**. The two pickers hand
/// back a civil date and time and `instant::compose` turns them into an instant
/// and the offset it resolved in — host-side, where it can fail: an
/// out-of-range integer, a civil date `Date::new` refuses, a `DateTime`
/// `to_zoned` refuses (`compose`'s `None` surface). That is §5.4's *"one
/// `edited()`, or nothing if `compose` fails"*, and nothing is recorded on
/// that path: the
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
    // The `Slider` sends its own `f32` and the numeric `LineEdit` sends its
    // text **verbatim**, unparsed and unrepaired: `f64::from_str` is applied
    // in `view_model::interpret`, where the field's held number is, and a
    // `1e400` or a pasted `12/25` reaches the draft as text with the last
    // representable number standing (§7 D23).
    Kind::Number if edit.slider => Some(Reported::AdjustedValue(edit.number)),
    Kind::Number => Some(Reported::AdjustedText(edit.text.to_string())),
    // The `ComboBox`'s `current-index`, unresolved. The markup cannot mint an
    // `AlternativeId` — `AlternativeId::new` is `pub(super)` in
    // `goad-semantics` — so the index travels and `view_model::interpret`
    // resolves it against the drawn field's own alternatives, on the walk
    // `controller::edit` already makes. An index no alternative has is case 1
    // of `interpret`'s `None` surface and takes the `Refused::UnknownField`
    // posture (design.md §5.2, §7 D12).
    //
    // A negative index is a renderer bug a `ComboBox` cannot produce —
    // `select` is called only with a repeater index — and it must not be
    // swallowed here. `u32::MAX` is an index no alternative has, so it takes
    // case 1 above and the refusal reaches a person; answering `None` would
    // report nothing at all.
    Kind::Choice => Some(Reported::Chosen(
      u32::try_from(edit.index).unwrap_or(u32::MAX),
    )),
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
  // **Reported rather than swallowed** (F-R8). This is the crate's only
  // `Weak::upgrade`, and it is unreachable today: the three zoom callbacks
  // live in the tray's callback table, which `SlintGlass` holds strongly
  // (`start` binds `window`, `PromptWindow::new()`, before either), so the
  // `PromptWindow` outlives every caller. What the
  // site must not do is let an action a person took vanish without a trace —
  // it is the one place in the renderer where that was possible.
  let Some(window) = window.upgrade() else {
    report_platform("zoom: the window was gone before the scale could be set");
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

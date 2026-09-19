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
use crate::wire::{Command, Stimulus, Wire};
use crate::zoom::Zoom;

/// One function, seven installations, each owning its own `Wire` clone and
/// nothing else. Seven distinct binding names rather than seven
/// `let wire = wire.clone();` — the reason is readability, not the lint
/// table: naming each clone after the callback it feeds says which
/// callback owns which handle (design.md §5.4).
pub fn install(window: &PromptWindow, tray: &Tray, wire: &Wire) {
  let chosen = wire.clone();
  window.on_chosen(move |view, option| {
    chosen.send(Command::Choose {
      view: view.into(),
      option: option.into(),
    });
  });

  // The widget has already changed itself, so the report is what the person
  // now sees; the draft is what decides what they will see after the next
  // present. A dropped send is therefore visibly undone rather than silently
  // divergent (design.md §5.4).
  //
  // The closure does exactly two things: turn the `FieldEdit` into a
  // `Reported`, and enqueue it. Everything else — the parse, the index, the
  // fallback, the refusal — happens in `controller::edit`, where the retained
  // presentation is.
  let editing = wire.clone();
  window.on_edited(move |view, option, field, edit| {
    let Some(report) = reported(&edit) else {
      return;
    };
    editing.send(Command::Edit {
      view: view.into(),
      option: option.into(),
      field: field.into(),
      reported: report,
    });
  });

  let closing = wire.clone();
  window.on_close_diagnostics(move || closing.send(Command::CloseDiagnostics));

  // A built-in, not one of ours: closing the window quits, in either mode,
  // and the window is kept shown because the quit path is `serve`
  // returning (design.md §5.4).
  let quitting = wire.clone();
  window.window().on_close_requested(move || {
    quitting.stop();
    CloseRequestResponse::KeepWindowShown
  });

  let checking = wire.clone();
  tray.on_check_now(move || checking.send(Command::Evaluate(Stimulus::Requested)));

  let showing = wire.clone();
  tray.on_show_diagnostics(move || showing.send(Command::OpenDiagnostics));

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
/// **`None` is *no control of this kind is drawn yet*, not a refusal.**
/// `view_model::undrawn_form` draws `boolean` alone, so a field of any other
/// kind never becomes a `FieldRow` and no control exists to raise this
/// callback for one — the arm is unreachable rather than declined. Each
/// remaining kind is filled by the phase that draws its control: `text`
/// PHASE-05, `datetime` PHASE-07 (which is also where `FieldEdit` grows the
/// `date` and `time` slots `Reported::Picked` is composed from), `number`
/// PHASE-08, `choice` PHASE-09. A phase that draws a control and forgets its
/// arm here fails that phase's own first case — the draft never sees the edit.
fn reported(edit: &FieldEdit) -> Option<Reported> {
  match edit.kind {
    Kind::Boolean => Some(Reported::Checked(edit.checked)),
    Kind::Text | Kind::Number | Kind::Choice | Kind::Datetime => None,
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

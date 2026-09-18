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
use crate::pending::Pending;
use crate::wire::{Command, PendingEdit, Stimulus, Wire};
use crate::zoom::Zoom;

/// One function, seven installations, each owning its own `Wire` clone and
/// nothing else. Seven distinct binding names rather than seven
/// `let wire = wire.clone();` — the reason is readability, not the lint
/// table: naming each clone after the callback it feeds says which
/// callback owns which handle (design.md §5.4).
pub fn install(window: &PromptWindow, tray: &Tray, wire: &Wire, pending: &Rc<Pending>) {
  // **The answer carries the flush.** Every pending edit travels inside the
  // one `Choose`, in no promised order — the keys are distinct, so any order
  // yields the same draft. It has to be one send: the channel holds one and a
  // Slint callback cannot yield, so a flush of *N* edits followed by a
  // `Choose` needs *N+1* slots and has one. `Pending::flush` puts the entries
  // back where the send did not take them, so a second click answers with the
  // same edits still attached (design.md §5.1, §5.4).
  let (chosen, choosing) = (wire.clone(), Rc::clone(pending));
  window.on_chosen(move |view, option| {
    choosing.flush(|edits| {
      chosen.send(Command::Choose {
        view: view.into(),
        option: option.into(),
        edits,
      })
    });
  });

  // The widget has already assigned itself, so the edit is what the person
  // now sees; the value channel is what decides what they will see after the
  // next present. A dropped send is therefore visibly undone rather than
  // silently divergent (design.md §5.4).
  //
  // **Which controls are held and which are sent where they are raised is
  // §5.2's table**, and it is the one thing here that is easy to get wrong in
  // the direction that costs nothing to write: `text` and both `number`
  // controls are the ones a person changes *continuously*, so they take the
  // debounce; `boolean`, `choice` and `datetime` raise one discrete edit and
  // holding it for 150 ms would buy nothing and delay the draft.
  let (editing, holding) = (wire.clone(), Rc::clone(pending));
  window.on_edited(move |view, option, field, edit| {
    let Some(value) = reported(&edit) else {
      return;
    };
    let carried = PendingEdit {
      view: view.into(),
      option: option.into(),
      field: field.into(),
      value,
    };
    if debounced(edit.kind) {
      holding.record(&editing, carried);
    } else {
      editing.send(Command::Edit {
        view: carried.view,
        option: carried.option,
        field: carried.field,
        value: carried.value,
      });
    }
  });

  let closing = wire.clone();
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

/// Whether this control's edits wait out the debounce window, per §5.2's
/// table. The map does not branch on kind; this is where the kind is read.
fn debounced(kind: Kind) -> bool {
  match kind {
    Kind::Text | Kind::Number => true,
    Kind::Boolean | Kind::Choice | Kind::Datetime => false,
  }
}

/// One edit, in the widget's own terms.
///
/// The mapping is straight — no parsing the boundary could not undo — which is
/// what `FieldEdit`'s kind discriminant buys: the callback reads the one slot
/// its control wrote.
///
/// `None` is a kind whose control this renderer does not draw yet, so no
/// markup can raise it: `FieldEdit` carries only the two slots the two drawn
/// controls write, and inventing a value for the other three would be a lie
/// the compiler cannot catch later. Each later phase turns one of these arms
/// into a read of the slot it adds.
fn reported(edit: &FieldEdit) -> Option<Reported> {
  match edit.kind {
    Kind::Boolean => Some(Reported::Checked(edit.checked)),
    Kind::Text => Some(Reported::Typed(edit.text.to_string())),
    Kind::Number | Kind::Choice | Kind::Datetime => None,
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

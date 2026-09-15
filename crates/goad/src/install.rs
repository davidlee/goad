//! `crates/goad/src/install.rs` — stratum 3.
//!
//! `pub`, and in the library rather than in `main.rs`, because validation
//! item 14e drives it from a `tests/` target and a `tests/` target cannot
//! reach a binary crate or a private item (D28, F-30). `install` returns
//! `()`: every setter it calls is infallible (design.md §5.4).

use slint::{CloseRequestResponse, ComponentHandle};

use crate::draft::Edited;
use crate::generated::{PromptWindow, Tray};
use crate::wire::{Command, Stimulus, Wire};

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

  // The widget has already flipped itself, so `checked` is what the person
  // now sees; the draft is what decides what they will see after the next
  // present. A dropped send is therefore visibly undone rather than silently
  // divergent (design.md §5.4).
  let editing = wire.clone();
  window.on_edited(move |view, option, field, checked| {
    editing.send(Command::Edit {
      view: view.into(),
      option: option.into(),
      field: field.into(),
      value: Edited::Checked(checked),
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
}

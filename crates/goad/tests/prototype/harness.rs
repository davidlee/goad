//! What the probes share: a window, a glass over the **same** `Pending` the
//! callbacks hold, a controller with a view retained, and the two readers that
//! join the structure channel to the value channel.
//!
//! No backend and no runtime. A `Prepared` is reachable from `receive` of a
//! hand-built `Outcome`, which is what `reception.rs` already does — driving a
//! scripted process to retain a view costs a `LocalSet` and a timeout and
//! would measure nothing these probes are about.

use std::rc::Rc;

use goad::controller::{Controller, Exchanged};
use goad::generated::{FieldValue, OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad::pending::Pending;
use goad::wire::{Cancel, Command, Notice, Wire};
use goad_semantics::protocol::canonical::{Timestamp, View, ViewId};
use goad_semantics::protocol::normalize::read_response;
use goad_shell::backend::transport::Captured;
use goad_shell::host::{Outcome, Presented};
use i_slint_backend_testing::init_no_event_loop;
use slint::{ComponentHandle, Model, VecModel};
use tokio::sync::mpsc;

/// The view every probe is built on: one option, one boolean and one text
/// field. Two kinds, because a probe that used one could not tell a rule about
/// fields from a rule about *the* field.
pub(crate) const A_FORM: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[
  {"id":"morning","label":"Morning","fields":[
    {"id":"read","kind":"boolean","label":"Read"},
    {"id":"noted","kind":"text","label":"Anything to add?"}]}]}}"#;

pub(crate) fn now() -> Timestamp {
  Timestamp::new(
    "2026-01-01T00:00:00Z"
      .parse()
      .expect("the fixture must be an instant"),
  )
}

fn view_of(document: &str) -> View {
  read_response(document.as_bytes(), now())
    .expect("the fixture must normalize")
    .value
    .view()
    .expect("the fixture carries a view")
    .clone()
}

/// A controller with `document` retained under `view_id`, and nothing else
/// absorbed. `view_id` is a parameter because I-H is a rule about two views
/// carrying the same ids, and a probe cannot pose that with one.
pub(crate) fn retaining(view_id: &str, document: &str) -> Controller {
  let mut controller = Controller::new();
  controller.absorb(
    Exchanged::Evaluation,
    Outcome {
      view: Some(Presented {
        view_id: ViewId::new(view_id),
        view: view_of(document),
      }),
      next_check: now(),
      discarded: Vec::new(),
      stderr: Captured::default(),
      failure: None,
      cleanup: None,
    },
  );
  controller
}

/// A window, a tray, a glass and the `Pending` all three share.
///
/// **The sharing is the point.** A probe that gave the glass and the callbacks
/// separate `Pending` values would get an overlay that never overlays
/// anything, with every case below still green and nothing measured
/// (design.md §8 R10).
pub(crate) struct Rig {
  pub(crate) window: PromptWindow,
  pub(crate) glass: SlintGlass,
  pub(crate) pending: Rc<Pending>,
  pub(crate) wire: Wire,
  pub(crate) commands: mpsc::Receiver<Command>,
}

pub(crate) fn rig() -> Rig {
  init_no_event_loop();
  let window = PromptWindow::new().expect("a headless window must construct");
  let tray = Tray::new().expect("a headless tray must construct");
  let pending = Pending::new();
  let glass = SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
    Rc::clone(&pending),
  );
  // Capacity one, exactly as `main.rs` builds it: a flush that needed two
  // slots would pass here and fail in production.
  let (sender, commands) = mpsc::channel::<Command>(1);
  let wire = Wire::new(sender, Cancel::new(), Notice::new());
  Rig {
    window,
    glass,
    pending,
    wire,
    commands,
  }
}

/// A field's `slot`, read off the structure channel exactly as the markup's
/// repeater would.
pub(crate) fn slot_of(window: &PromptWindow, option: &str, field: &str) -> Option<i32> {
  window
    .get_options()
    .iter()
    .filter(|row| row.id == option)
    .flat_map(|row| row.blocks.iter().collect::<Vec<_>>())
    .flat_map(|block| block.fields.iter().collect::<Vec<_>>())
    .find(|row| row.id == field)
    .map(|row| row.slot)
}

/// A field's value, joined across the two channels — the row carries the slot
/// and the value lives at that index of `values` (§5.2). This is what a
/// control's binding reads, and it is the only honest way to ask the window
/// what a field shows.
pub(crate) fn value_of(window: &PromptWindow, option: &str, field: &str) -> Option<FieldValue> {
  let slot = slot_of(window, option, field)?;
  window.get_values().row_data(usize::try_from(slot).ok()?)
}

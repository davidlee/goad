//! **The overlay** — design.md §5.3, *A field's value is the draft's,
//! overlaid*, and §5.5 I-H.
//!
//! The case that made it necessary is not exotic: `serve` presents at the top
//! of every iteration, so *any* command handled inside the 150 ms window
//! produces a present — a tray check, a diagnostics toggle, an evaluation
//! finishing. Without the overlay that present writes the draft's older value
//! into the slot, the guard sees a difference, and the widget is corrected to
//! a value the person replaced 40 ms ago.

use goad::draft::Reported;
use goad::glass::Glass;
use goad::wire::PendingEdit;

use crate::harness::{A_FORM, retaining, rig, value_of};

/// The probe. Nothing has reached the draft, and the screen shows what the
/// person typed.
///
/// **Driven red**: dropping the `pending.shown(...)` overlay from
/// `glass::drawn` and reading the draft alone leaves the slot empty and fails
/// here. That is the whole of what this watches.
#[test]
fn a_pending_entry_is_what_the_screen_shows() {
  let mut rig = rig();
  let controller = retaining("v1", A_FORM);

  rig.pending.record(
    &rig.wire,
    PendingEdit {
      view: "v1".to_owned(),
      option: "morning".to_owned(),
      field: "noted".to_owned(),
      value: Reported::Typed("half-typ".to_owned()),
    },
  );
  rig.glass.present(controller.frame(false));

  assert_eq!(
    value_of(&rig.window, "morning", "noted").map(|value| value.text.to_string()),
    Some("half-typ".to_owned()),
    "the entry is what the person is looking at, so it is what the channel carries"
  );
}

/// The draft is not touched by the overlay. The two are separate states and
/// the entry is *displayed*, not *recorded* — it reaches the draft only when
/// the timer's `Command::Edit` or the answer's `Choose` delivers it.
///
/// Asserted from a draft that already holds something, so the assertion is one
/// a glass that simply ignored the draft could not also satisfy.
#[test]
fn the_overlay_displays_without_recording() {
  let mut rig = rig();
  let mut controller = retaining("v1", A_FORM);
  controller
    .edit(
      "v1",
      "morning",
      "noted",
      &Reported::Typed("recorded".to_owned()),
    )
    .expect("morning declares `noted`");

  rig.pending.record(
    &rig.wire,
    PendingEdit {
      view: "v1".to_owned(),
      option: "morning".to_owned(),
      field: "noted".to_owned(),
      value: Reported::Typed("recorded, then more".to_owned()),
    },
  );
  rig.glass.present(controller.frame(false));

  assert_eq!(
    value_of(&rig.window, "morning", "noted").map(|value| value.text.to_string()),
    Some("recorded, then more".to_owned()),
    "the entry wins over the draft for this field"
  );
  let (_, answer) = controller
    .answer("v1", "morning")
    .expect("the option still answers");
  assert_eq!(
    answer
      .values
      .iter()
      .find(|(id, _)| id.as_str() == "noted")
      .map(|(_, value)| value.clone()),
    Some(serde_json::json!("recorded")),
    "and the draft still holds only what was recorded"
  );
}

/// The overlay is per (option, field) and not per view. A field with no entry
/// reads the draft, on the same present as a field that has one.
#[test]
fn a_field_with_no_entry_still_reads_the_draft() {
  let mut rig = rig();
  let mut controller = retaining("v1", A_FORM);
  controller
    .edit("v1", "morning", "read", &Reported::Checked(true))
    .expect("morning declares `read`");

  rig.pending.record(
    &rig.wire,
    PendingEdit {
      view: "v1".to_owned(),
      option: "morning".to_owned(),
      field: "noted".to_owned(),
      value: Reported::Typed("typing".to_owned()),
    },
  );
  rig.glass.present(controller.frame(false));

  assert_eq!(
    value_of(&rig.window, "morning", "read").map(|value| value.checked),
    Some(true),
    "its neighbour's pending entry does not reach this field"
  );
  assert_eq!(
    value_of(&rig.window, "morning", "noted").map(|value| value.text.to_string()),
    Some("typing".to_owned())
  );
}

//! **The two channels** — design.md §5.2, §5.5 I-B and I-F.
//!
//! What the split is for: a present that shows the same view rewrites the
//! state channel and leaves the structure channel — and therefore every
//! element beneath it — alone. The old glass rebuilt the row model on every
//! present, which destroyed and recreated every control, which is what AC-4 is
//! about.

use goad::controller::Controller;
use goad::draft::Reported;
use goad::glass::Glass;
use slint::Model;

use crate::harness::{A_FORM, rig, slot_of, value_of};

/// I-B: a field's slot is its index into `values`, by construction. Asserted
/// over two fields, because a single field's slot is `0` whatever the
/// numbering does.
#[test]
fn every_field_s_slot_is_its_index_into_the_value_channel() {
  let mut rig = rig();
  let controller = retained();
  rig.glass.present(controller.frame(false));

  assert_eq!(slot_of(&rig.window, "morning", "read"), Some(0));
  assert_eq!(slot_of(&rig.window, "morning", "noted"), Some(1));
  assert_eq!(
    rig.window.get_values().row_count(),
    2,
    "one value per drawn field, and nothing else"
  );
}

/// **The probe this phase exists for.** A second present of the same view
/// rewrites the value channel and destroys no element.
///
/// `inits` is the instrument: an element that is destroyed and recreated runs
/// `init` again, so a count that does not move is a count of elements that
/// survived. It is read after the first present so the number compared is a
/// real one — a probe that asserted `inits == 0` throughout would pass on a
/// window that drew nothing at all.
///
/// **Driven red**: removing `present`'s `if self.shown != showing` guard, so
/// the rows are written on every present, takes `inits` from 2 to 4 and fails
/// here. That is the defect the split exists to prevent and it is the only
/// thing this probe watches.
#[test]
fn a_second_present_of_the_same_view_rewrites_values_and_destroys_no_element() {
  let mut rig = rig();
  let mut controller = retained();
  rig.glass.present(controller.frame(false));

  let drawn = rig.window.get_inits();
  assert!(
    drawn > 0,
    "the window must have drawn something for a survival count to mean anything"
  );
  assert_eq!(
    value_of(&rig.window, "morning", "read").map(|value| value.checked),
    Some(false),
    "an untouched boolean is unticked"
  );

  controller
    .edit("v1", "morning", "read", &Reported::Checked(true))
    .expect("morning declares `read`");
  rig.glass.present(controller.frame(false));

  assert_eq!(
    rig.window.get_inits(),
    drawn,
    "the same view was presented again: not one element may have been rebuilt"
  );
  assert_eq!(
    value_of(&rig.window, "morning", "read").map(|value| value.checked),
    Some(true),
    "and the value channel carries what the draft now holds"
  );
}

/// I-F's third write, and the guard's only trigger. The epoch moves on
/// **every** present, including one that changed nothing — that is what makes
/// it a signal to re-assert rather than a record of what changed.
#[test]
fn the_epoch_moves_on_every_present_including_one_that_changed_nothing() {
  let mut rig = rig();
  let controller = retained();

  rig.glass.present(controller.frame(false));
  let first = rig.window.get_epoch();
  rig.glass.present(controller.frame(false));
  let second = rig.window.get_epoch();
  rig.glass.present(controller.frame(false));

  assert_ne!(
    first, second,
    "a present that changed nothing still bumps it"
  );
  assert_ne!(second, rig.window.get_epoch());
}

/// A new view **does** rebuild the structure channel: there is no interaction
/// state worth preserving across a replacement, and the slots are renumbered.
/// This is the negative half of the survival probe above — without it, a
/// `present` that never wrote the rows at all would pass that one.
#[test]
fn a_new_view_rebuilds_the_structure_channel() {
  let mut rig = rig();
  let first = retained();
  rig.glass.present(first.frame(false));
  let drawn = rig.window.get_inits();

  let replacement = crate::harness::retaining("v2", A_FORM);
  rig.glass.present(replacement.frame(false));

  assert!(
    rig.window.get_inits() > drawn,
    "a replacement view's rows are written, which destroys and recreates every element"
  );
}

fn retained() -> Controller {
  crate::harness::retaining("v1", A_FORM)
}

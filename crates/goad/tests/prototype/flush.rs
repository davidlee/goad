//! **The answer carries the flush** — design.md §5.1, §5.4 *An answer*.
//!
//! One send, because the command channel holds one and a Slint callback is
//! synchronous: a flush of *N* edits followed by a `Choose` needs *N+1* slots
//! and has one. The probes here drive the **real** `install`ed callbacks —
//! `invoke_chosen` and `invoke_edited` run the closure a person's click and
//! keystroke run — so what is measured is the wiring rather than a second
//! spelling of it.

use goad::draft::Reported;
use goad::generated::{FieldEdit, Kind, Tray};
use goad::install::install;
use goad::wire::{Command, PendingEdit};
use slint::SharedString;

use crate::harness::rig;

fn entry(field: &str, typed: &str) -> PendingEdit {
  PendingEdit {
    view: "v1".to_owned(),
    option: "morning".to_owned(),
    field: field.to_owned(),
    value: Reported::Typed(typed.to_owned()),
  }
}

/// The probe. Two held edits and one click produce **one** command carrying
/// both, and the map is empty afterwards.
///
/// **Driven red**: making `chosen` send the edits as separate `Command::Edit`s
/// before the `Choose` leaves the first on the channel and drops the rest,
/// and the `Choose` never arrives — which is the failure the carried `edits`
/// exist to prevent, and it fails here on the very first assertion.
#[test]
fn one_click_delivers_every_pending_edit_in_one_command() {
  let mut rig = rig();
  let tray = Tray::new().expect("a headless tray must construct");
  install(&rig.window, &tray, &rig.wire, &rig.pending);

  rig.pending.record(&rig.wire, entry("noted", "first"));
  rig.pending.record(&rig.wire, entry("other", "second"));

  rig
    .window
    .invoke_chosen(SharedString::from("v1"), SharedString::from("morning"));

  let Ok(Command::Choose {
    view,
    option,
    edits,
  }) = rig.commands.try_recv()
  else {
    panic!("the click must enqueue exactly one `Choose`");
  };
  assert_eq!((view.as_str(), option.as_str()), ("v1", "morning"));
  assert_eq!(edits.len(), 2, "both held edits travel inside it");
  assert!(edits.contains(&entry("noted", "first")));
  assert!(edits.contains(&entry("other", "second")));

  assert!(
    rig.commands.try_recv().is_err(),
    "one send, not one per edit: the channel holds one and a callback cannot yield"
  );
  assert_eq!(
    rig.pending.shown("v1", "morning", "noted"),
    None,
    "an enqueued send is what empties the map"
  );
}

/// The other half of *enqueued, not accepted*. With the channel already full
/// the `Choose` is lost entire — and nothing is cleared, so a second click
/// answers with the same edits still attached and the widgets meanwhile still
/// show them.
///
/// **Driven red**: clearing the map before the send, or ignoring
/// `Wire::send`'s result, loses the edit here.
#[test]
fn a_full_channel_loses_the_command_and_keeps_the_edits() {
  let mut rig = rig();
  let tray = Tray::new().expect("a headless tray must construct");
  install(&rig.window, &tray, &rig.wire, &rig.pending);

  rig.pending.record(&rig.wire, entry("noted", "first"));
  // Fill the one slot the channel has, so the click's own send cannot land.
  assert!(rig.wire.send(Command::OpenDiagnostics));

  rig
    .window
    .invoke_chosen(SharedString::from("v1"), SharedString::from("morning"));

  assert_eq!(
    rig.pending.shown("v1", "morning", "noted"),
    Some(Reported::Typed("first".to_owned())),
    "nothing was delivered, so the entry must stand"
  );
  assert_eq!(rig.commands.try_recv(), Ok(Command::OpenDiagnostics));
  assert!(
    rig.commands.try_recv().is_err(),
    "and the `Choose` really was dropped rather than queued behind it"
  );
}

/// §5.2's table, which is the easy thing to get wrong in the direction that
/// costs nothing to write. A `text` edit is **held**; a `boolean` edit is
/// **sent where it is raised**.
///
/// **Driven red**: routing every kind through the map leaves the channel empty
/// on the boolean half; routing none through it puts the text edit on the
/// channel and leaves the map empty on the other half. Both halves are
/// asserted so neither mistake passes.
#[test]
fn a_text_edit_is_held_and_a_boolean_edit_is_sent_where_it_is_raised() {
  let mut rig = rig();
  let tray = Tray::new().expect("a headless tray must construct");
  install(&rig.window, &tray, &rig.wire, &rig.pending);

  rig.window.invoke_edited(
    SharedString::from("v1"),
    SharedString::from("morning"),
    SharedString::from("noted"),
    FieldEdit {
      kind: Kind::Text,
      checked: false,
      text: SharedString::from("mid-type"),
    },
  );
  assert_eq!(
    rig.pending.shown("v1", "morning", "noted"),
    Some(Reported::Typed("mid-type".to_owned())),
    "a control a person changes continuously waits out the debounce"
  );
  assert!(
    rig.commands.try_recv().is_err(),
    "and nothing reaches the channel until the timer or an answer takes it"
  );

  rig.window.invoke_edited(
    SharedString::from("v1"),
    SharedString::from("morning"),
    SharedString::from("read"),
    FieldEdit {
      kind: Kind::Boolean,
      checked: true,
      text: SharedString::default(),
    },
  );
  assert_eq!(
    rig.commands.try_recv(),
    Ok(Command::Edit {
      view: "v1".to_owned(),
      option: "morning".to_owned(),
      field: "read".to_owned(),
      value: Reported::Checked(true),
    }),
    "one discrete edit, sent where it was raised"
  );
  assert_eq!(
    rig.pending.shown("v1", "morning", "read"),
    None,
    "and never held"
  );
}

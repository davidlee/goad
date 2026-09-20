//! `plan.md` PHASE-05: the form on the wire — AC-1, AC-4, AC-5 and AC-3's
//! remaining half.
//!
//! Slice 009 `plan.md` PHASE-01 adds two more at the foot, and they are a
//! different kind of case: they read an **instrument counter** off the
//! production markup to ask whether a present rebuilt the form or wrote it in
//! place (PHASE-01/VT-1, VT-2). Element identity is not otherwise observable,
//! which is why the counters exist at all (`design.md` §7 D15).
//!
//! **The only module in this target that reads what left the host.** Every
//! other tier verifies at the screen, at a pure function, or at the row
//! model; the first four cases drive a real child process through the
//! production `serve` and read the request that process received, off its own
//! invocation log.
//!
//! That is not a preference. A field test that reads the draft back through
//! `Controller` and stops there would stay green with `answer()` walking the
//! draft's keys instead of the declared fields — D6, the defect most worth
//! catching (`design.md` §9, `plan.md` S-7). So every case here either reads
//! the log or asserts something about the screen, and slice 007's VT-3 does
//! **both**, because for its AC-5 neither half implies the other.
//!
//! **Two `plan.md`s meet in this file and their criterion ids collide.** The
//! first four cases carry slice 007's VT-1 … VT-4; the last two carry slice
//! 009's PHASE-01/VT-1 and VT-2, and say so. Cite the slice with the id.
//!
//! The person is simulated all the way down. Every command these cases put on
//! the channel — the request for a check, each tick, each press — is fired by
//! activating a real element through the accessible tree, and the callbacks
//! they reach are the ones `goad::install::install` puts on the window and the
//! tray in production. No case builds a `Command` by hand.
//!
//! `#[cfg(test)]` on the declaration, not on the file, for the same reason
//! every other module here carries it (`clippy::tests_outside_test_module`).

use std::path::{Path, PathBuf};
use std::rc::Rc;

use goad::controller::{Controller, Ending, serve};
use goad::diagnostics::next_check_line;
use goad::generated::{PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad::install::install;
use goad::pending::Debounce;
use goad::wire::{Cancel, Command, Notice, Wire};
use goad_shell::backend::process::ProcessBackend;
use goad_shell::host::Host;
use goad_shell::ingress::Ingress;
use i_slint_backend_testing::{AccessibleRole, ElementHandle, ElementQuery};
use serde_json::Value;
use slint::{ComponentHandle, Model};
use tokio::sync::mpsc;
use tokio::task::LocalSet;

use crate::driving::{host, instant};
use crate::harness::{
  TIMEOUT, described, element_described, field_described, glass_overlaying, logging_scripted, now,
  stub_clock, until, value_of, window_and_tray, within_option,
};
use crate::scripting::invocations;
use crate::waiting::LIVENESS_BOUND;

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

/// One option, three `boolean` fields. Three and not one so that AC-1's
/// "exactly N keys" has more than one way to be wrong, and so that a case can
/// leave one box alone: the untouched field's key is the half a walk over the
/// draft cannot produce, and so the only key `answer()` walking the draft
/// could not also supply (`design.md` §9).
const THREE_FIELDS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"read","kind":"boolean","label":"Read"},{"id":"tidied","kind":"boolean","label":"Tidied"}]}]},"next_check":"45 minutes"}"#;

/// Two options whose fields **share the id `read`**. R-52 scopes a field id to
/// its option, so this is legal and is the whole of AC-4: two independent
/// keys that an implementation keyed by field alone would collapse into one.
const TWO_FORMS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"read","kind":"boolean","label":"Read"}]},{"id":"evening","label":"Evening","fields":[{"id":"read","kind":"boolean","label":"Read"},{"id":"tidied","kind":"boolean","label":"Tidied"}]}]},"next_check":"45 minutes"}"#;

/// One option carrying a `boolean` and a **`choice` whose field id is the
/// option's own id**. `R-52` scopes a field id to its option and `R-53` puts
/// alternative ids in a namespace of their own, so `morning` naming both an
/// option and one of that option's fields is legal — and the option's `Button`
/// and the field's `ComboBox` then answer to the same accessible description.
/// Every query below filters by role or by element type for that reason
/// (`design.md` §9, AC-8).
///
/// **Three alternatives, and the one chosen is the middle one.** Two would let
/// *the id that was chosen* and *the id that was not* be told apart by a coin
/// toss, and choosing the first or the last would let an index-off-by-one and
/// a first-alternative fallback both pass.
const A_CHOICE_FIELD: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"morning","kind":"choice","label":"How did it go?","options":[{"id":"badly","label":"Badly"},{"id":"fine","label":"Fine"},{"id":"well","label":"Well"}]}]}]},"next_check":"45 minutes"}"#;

/// One option carrying **all five kinds**, in an order none of them is
/// declared in anywhere else, and a **sixth** field so that `number`'s two
/// controls are both drawn: `rated` declares `[0, 10]`, which `slider_bounds`
/// admits, and `counted` declares no bound, which it refuses.
///
/// The five kinds are AC-1 and AC-2's whole subject, and the declared order is
/// part of the claim: a renderer that grouped by kind, or that drew them in
/// the order the markup's `if` chain tests them, would pass a case that only
/// counted controls.
const EVERY_KIND: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"noted","kind":"text","label":"Anything to add?"},{"id":"mood","kind":"choice","label":"How did it go?","options":[{"id":"badly","label":"Badly"},{"id":"fine","label":"Fine"},{"id":"well","label":"Well"}]},{"id":"rated","kind":"number","label":"Out of ten","min":0,"max":10},{"id":"when","kind":"datetime","label":"When?"},{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"counted","kind":"number","label":"How many?"}]}]},"next_check":"45 minutes"}"#;

/// One option carrying a `boolean` and **two `text` fields**. Two, because
/// AC-4's element half is about a person typing into one field and then
/// another inside one debounce window: a single field cannot tell a map keyed
/// by (option, field) from one holding a single edit.
const TWO_TEXT_FIELDS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"noted","kind":"text","label":"Anything to add?"},{"id":"also","kind":"text","label":"And then?"}]}]},"next_check":"45 minutes"}"#;

/// One option carrying a `boolean` and **two `datetime` fields**. Two, because
/// both pickers are root singletons shared by every `datetime` field in the
/// form, so a single field cannot tell a seed written per field from a seed
/// written once: with one field, opening on *this* field's pick and opening on
/// *the last* pick are the same observation (`design.md` §5.4, §7 D21).
const TWO_DATETIME_FIELDS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"when","kind":"datetime","label":"When?"},{"id":"until","kind":"datetime","label":"Until?"}]}]},"next_check":"45 minutes"}"#;

/// One option carrying a `boolean` and **three `number` fields whose ranges
/// take different controls**. `rated` declares `[0, 10]`, which
/// `slider_bounds` admits; `counted` declares no bound at all and `frozen`
/// declares a one-ulp span at `2^100`, and it refuses both.
///
/// The refusal is the **ordinary** case, not the exceptional one: the text
/// control is the one that always works and the `Slider` is the one with an
/// admissibility condition, which is the opposite of how a bounds-driven
/// reading makes it look (`design.md` §5.2).
///
/// **Two refusals and not one, because they fail different clauses.**
/// `counted` fails the first — there is no bound to round-trip — and a
/// bounds-driven implementation would catch that on its own. `frozen`'s bounds
/// both round-trip exactly and its span is finite and strictly positive; what
/// it fails is the *operability* clause, because a hundredth of one ulp is
/// below half an ulp and `increment()` would move nothing. With only `counted`
/// here, deleting the second and third clauses outright leaves this whole
/// target green — measured, as the injection pass records.
const THREE_NUMBER_FIELDS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"rated","kind":"number","label":"How was it?","min":0,"max":10},{"id":"counted","kind":"number","label":"How many?"},{"id":"frozen","kind":"number","label":"How precisely?","min":1.2676506002282294e30,"max":1.2676507513439569e30}]}]},"next_check":"45 minutes"}"#;

/// One option carrying a `number` whose **`min` is `f64::MAX`** — a legal
/// `R-17` bound whose `Display` is 309 characters, and which `R-58` requires a
/// value for whether or not anybody touches it.
///
/// A `min` with no `max` is legal and takes the text control, so this is the
/// spelling rule measured at the element rather than at the formatter
/// (`view_model::spelled`, PHASE-02/VT-5).
const A_HUGE_MINIMUM: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"huge","kind":"number","label":"How much?","min":1.7976931348623157e308}]}]},"next_check":"45 minutes"}"#;

/// One option carrying a `number` **drawn showing a value nobody typed**: a
/// `min` of `3` and no `max`. Legal under `R-17`, refused a slider by
/// `slider_bounds`'s first clause — a range missing a bound has no span — and
/// as-drawn `3`, which is the number an entry the parse refuses falls back to.
const A_NUMBER_DRAWN_AT_THREE: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"counted","kind":"number","label":"How many?","min":3}]}]},"next_check":"45 minutes"}"#;

/// A second view, distinguishable from every other fixture here by its title
/// so a case can wait on the replacement arriving. Its option and field ids
/// are its own, so a rebuild is visible at the screen as well as at the
/// instrument counter (slice 009 PHASE-01/VT-2).
const ANOTHER_FORM: &str = r#"{"view":{"kind":"choice","title":"Anything else?","options":[{"id":"evening","label":"Evening","fields":[{"id":"tidied","kind":"boolean","label":"Tidied"}]}]},"next_check":"45 minutes"}"#;

/// A successful exchange with nothing new to show. From an `evaluate` this is
/// `Shift::Retained`: the outstanding view, and its draft, are left exactly as
/// they were (`design.md` §5.4). The instant is absolute and far from the
/// fixtures' `45 minutes`, so the next-check line it produces is one no other
/// exchange in a case could have written.
const RETAINED: &str = r#"{"view":null,"next_check":"2026-06-01T00:00:00Z"}"#;
/// The instant `RETAINED` instructs, for the line the glass writes from it.
const RETAINED_AT: &str = "2026-06-01T00:00:00Z";

/// A second [`RETAINED`], at a second instant — for a case that has to observe
/// **two** presents it did not cause, and tell them apart.
const RETAINED_AGAIN: &str = r#"{"view":null,"next_check":"2026-07-01T00:00:00Z"}"#;
/// The instant `RETAINED_AGAIN` instructs.
const RETAINED_AGAIN_AT: &str = "2026-07-01T00:00:00Z";

// ---------------------------------------------------------------------------
// Reading the wire
// ---------------------------------------------------------------------------

/// The *n*th (1-indexed) request `logging_scripted`'s backend received, as
/// JSON. `scheduling.rs`'s `request_kind` is the precedent and reads the same
/// log for `event.kind`; this file reads it for what a person's answer
/// carried. Two readers of one log format, deliberately — EX-3 keeps each
/// where its one consumer is rather than growing a shared parser neither
/// file's assertions would be easier to read through.
fn logged(log: &Path, n: usize) -> Value {
  let text = std::fs::read_to_string(log).expect("the invocation log must exist by now");
  let line = text
    .lines()
    .nth(n - 1)
    .unwrap_or_else(|| panic!("a request must be logged at position {n}"));
  serde_json::from_str(line).expect("a logged request is valid JSON")
}

/// The option id the *n*th logged request answers under — `response.option`
/// (SPEC-001 §6.1).
fn answered_option(log: &Path, n: usize) -> String {
  logged(log, n)["response"]["option"]
    .as_str()
    .expect("a respond carries response.option")
    .to_owned()
}

/// The `values` map of the *n*th logged request, as the backend received it.
///
/// A map, so what a case compares against it is a **set** of keys and a value
/// for each — never a declared order. Declared order is AC-2's and is
/// asserted where the order exists: at the screen and at the row model.
fn submitted_values(log: &Path, n: usize) -> serde_json::Map<String, Value> {
  logged(log, n)["response"]["values"]
    .as_object()
    .expect("a respond carries response.values")
    .clone()
}

/// The keys of a submitted map, sorted, for comparison against a written-out
/// list.
///
/// Sorted here rather than relied upon. `serde_json::Map` is a `BTreeMap`
/// under this workspace's feature set — so the keys already arrive sorted —
/// and an `IndexMap` under `preserve_order`, which any crate in the graph
/// could switch on, because cargo unifies features. One line makes the
/// expected list a claim about the key set rather than about a feature flag.
fn keys_of(values: &serde_json::Map<String, Value>) -> Vec<&str> {
  let mut keys: Vec<&str> = values.keys().map(String::as_str).collect();
  keys.sort_unstable();
  keys
}

// ---------------------------------------------------------------------------
// Reading and driving the screen
// ---------------------------------------------------------------------------

/// The window's own size, declared so the query can reach the whole form.
///
/// A shown window clips: `ElementQuery` skips any element outside the nearest
/// clipping ancestor's rect
/// (`i-slint-backend-testing-1.17.1/search_api.rs:373-375` →
/// `i-slint-core-1.17.1/item_tree.rs:399-408`, a geometric test). Left to its
/// preferred size the window is 65px tall under `material`, the options
/// `ScrollView` fits one control, and every box in a form of several is simply
/// unreachable — the query answers `None`, which reads like a missing element
/// and is not one. `wiring.rs`'s `with_room_for_the_form` and `mod busy`'s
/// `with_room_for_every_control` declare a size for the same reason, at two
/// different values.
///
/// **It states what the test can see, and claims nothing about what the window
/// should be.** That the product clips its own second option at its preferred
/// size is unchanged by this line and is AC-10's, at slice 007 PHASE-06.
fn with_room_for_the_form(window: &PromptWindow) {
  ComponentHandle::window(window).set_size(slint::PhysicalSize::new(600, 600));
}

/// The option's control, or a panic naming what was not there. One line over
/// [`harness::element_described`], whose doc carries the type filter and why
/// it is there; this names the *activation* a case performs and keeps the
/// message in one place.
fn option_control(window: &PromptWindow, option: &str) -> ElementHandle {
  element_described(window, option).unwrap_or_else(|| panic!("no control described {option:?}"))
}

/// What the **screen** shows for one field: the checkbox's own
/// `accessible-checked`, found by the option-scoped query because a field id
/// is unique only within its option (R-52, `harness::field_described`).
fn checked_on_screen(window: &PromptWindow, option: &str, field: &str) -> bool {
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .accessible_checked()
    .unwrap_or_else(|| panic!("{option}/{field} declares no accessible-checked"))
}

/// What the screen shows for several of one option's fields, paired with
/// their ids so a failure names the box rather than a position.
fn screen_of<const N: usize>(
  window: &PromptWindow,
  option: &str,
  fields: [&'static str; N],
) -> Vec<(&'static str, bool)> {
  fields
    .into_iter()
    .map(|field| (field, checked_on_screen(window, option, field)))
    .collect()
}

/// One field's value as the **draft** holds it, read off the value channel the
/// last present wrote (`glass.rs::option_models`, `harness::value_of`).
///
/// **A synchronisation point, not an assertion.** A tick has reached the draft
/// only once a present has written the slot from it, and `start`'s channel
/// holds one (`mpsc::channel::<Command>(1)`): a second click sent before the
/// loop has drained the first is dropped by `Wire::send` and the tick
/// silently undoes itself.
/// Waiting on this is what keeps that from happening, and a drop then fails as
/// a timeout rather than as a wrong value.
///
/// A field the view does not declare reads as `false` rather than panicking,
/// because every caller is a poll predicate: the one thing a synchronisation
/// point may not do is fail before the condition it is waiting for is true.
fn drafted(window: &PromptWindow, option: &str, field: &str) -> bool {
  value_of(window, option, field).is_some_and(|value| value.checked)
}

/// What `glass.rs` writes into a `datetime` field's `text` slot before anybody
/// has picked one. Restated here rather than imported because `glass::NOT_SET`
/// is private: a case that read the constant would agree with the markup by
/// construction and could not see the two of them diverge.
const NOT_SET: &str = "not set";

/// Today, in the person's own zone — the seed an unpicked field's picker opens
/// on, read through the production function so a case's expectation and the
/// glass's seed cannot be two different days (`today_local`).
fn instant_today() -> (goad::generated::Date, goad::generated::Time) {
  goad::instant::today_local()
}

/// The RFC 3339 rendering of one civil pick, resolved in the system zone
/// exactly as `install.rs`'s `datetime` arm resolves it. Panics where
/// `compose` refuses, which for a date and hour a case names literally means
/// the test fixture is wrong rather than the host.
fn composed(year: i32, month: i32, day: i32, hour: i32) -> String {
  let date = goad::generated::Date { year, month, day };
  let time = goad::generated::Time {
    hour,
    minute: 0,
    second: 0,
  };
  let (instant, offset) =
    goad::instant::compose(&date, &time).expect("the case's own date and time must resolve");
  instant.instant().display_with_offset(offset).to_string()
}

/// The diagnostic lines the glass wrote to the window — the surface a person
/// reads an undrawn report on, rather than `Controller`'s own copy of it.
fn diagnostic_lines(window: &PromptWindow) -> Vec<String> {
  window
    .get_diagnostic_lines()
    .iter()
    .map(|line| line.to_string())
    .collect()
}

/// Click one box, the way a person does. Half of [`tick!`], which is what
/// every case calls: a click on its own says nothing about whether the edit
/// reached the loop.
fn click(window: &PromptWindow, option: &str, field: &str) {
  assert!(
    !drafted(window, option, field),
    "{option}/{field} is already ticked; every tick in this file goes the other way"
  );
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .invoke_accessible_default_action();
}

/// Type into one field, the way `set_accessible_value` does: it assigns the
/// widget's `text` and calls `edited` **from inside the markup**
/// (`widgets/fluent/lineedit.slint:16`), reaching no `TextInput` insertion
/// logic at all — the same shape a paste has. So this drives the host's own
/// `edited` callback and the control's self-assignment together, which is what
/// a keystroke does.
fn type_into(window: &PromptWindow, option: &str, field: &str, text: &str) {
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .set_accessible_value(text);
}

/// What the **screen** shows for one `datetime` field — the button's own
/// accessible label, which is its text: the composed value, or *not set*.
///
/// A `Button` announces what it says, so this is the one control in the form
/// whose displayed value is read as a label rather than as a value or a
/// checked state. That is also what tells the three controls apart in the
/// tree: a box declares `accessible-checked`, a line edit declares
/// `accessible-value`, and this declares neither.
fn shown_on_screen(window: &PromptWindow, option: &str, field: &str) -> String {
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .accessible_label()
    .unwrap_or_else(|| panic!("{option}/{field} declares no accessible-label"))
    .to_string()
}

/// Open one field's picker, the way a person does: the button's own default
/// action, which seeds the two root properties off that field's value slot and
/// shows the date picker (`ui/app.slint`).
fn open_picker(window: &PromptWindow, option: &str, field: &str) {
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .invoke_accessible_default_action();
}

/// The **one** control anywhere the window is showing — inside an open popup
/// included — whose accessible label satisfies `matching`.
///
/// `ElementQuery::find_all` walks `active_popups` as well as the window's own
/// tree (`search_api.rs:304-312`), which is what lets this target reach inside
/// a picker at all; PHASE-07/VA-1 records the measurement. Everything a picker
/// is driven by here declares `accessible-role: button` and an
/// `accessible-action-default` that calls its own `clicked` — a calendar day
/// cell (`common/datepicker_base.slint:59-63`), a clock face selector
/// (`common/time-picker-base.slint:129-133`) and a `StandardButton`. None of
/// them is dispatched at a coordinate, so none of them needs the popup laid
/// out.
///
/// **Exactly one, asserted rather than taken.** `find_first` would return
/// whichever the walk reached first and report no ambiguity, and the labels
/// these cases select on — a bare day number, an hour — are short enough that
/// a second match is a real risk rather than a theoretical one.
fn only_button(
  window: &PromptWindow,
  what: &str,
  matching: impl Fn(&str) -> bool + 'static,
) -> ElementHandle {
  let mut found = ElementQuery::from_root(window)
    .match_accessible_role(AccessibleRole::Button)
    .match_predicate(move |element| {
      element
        .accessible_label()
        .is_some_and(|label| matching(label.as_str()))
    })
    .find_all();
  let labels: Vec<String> = found
    .iter()
    .filter_map(|element| element.accessible_label().map(|label| label.to_string()))
    .collect();
  assert_eq!(
    found.len(),
    1,
    "expected exactly one {what}, found {labels:?}"
  );
  found.remove(0)
}

/// Press `OK` in whichever picker is open. Both pickers draw theirs as a
/// `StandardButton` of kind `ok`, whose text — and so its accessible label —
/// is `@tr("OK")` (`common/standardbutton.slint:17-32`).
fn accept(window: &PromptWindow) {
  only_button(window, "OK button", |label| label == "OK").invoke_accessible_default_action();
}

/// Press `Cancel` in whichever picker is open. Both pickers close themselves
/// and raise `canceled`, which this markup handles by doing nothing — the
/// whole edit is abandoned (`design.md` §7 D5).
fn cancel(window: &PromptWindow) {
  only_button(window, "Cancel button", |label| label == "Cancel")
    .invoke_accessible_default_action();
}

/// Choose a day of the displayed month. A calendar day cell carries the day
/// number as its accessible label (`common/datepicker_base.slint:46-63`), and
/// the displayed month is the seed's, so any day from 1 to 28 exists whatever
/// the seed is.
fn pick_day(window: &PromptWindow, day: i32) {
  let wanted = day.to_string();
  only_button(window, "calendar day cell", move |label| label == wanted)
    .invoke_accessible_default_action();
}

/// Choose an hour on the clock face. A selector's label is
/// `"{value} Hours or minutes of {total}"`, where `total` is 12 or 24 while
/// hours are being chosen and 60 once minutes are — so the prefix alone names
/// one cell, whichever `use-24-hour-format` the locale resolves to
/// (`common/time-picker-base.slint:121-133`).
fn pick_hour(window: &PromptWindow, hour: i32) {
  let wanted = format!("{hour} Hours");
  only_button(window, "clock face selector", move |label| {
    label.starts_with(&wanted)
  })
  .invoke_accessible_default_action();
}

/// Whether the open calendar is sitting on a given day. A day cell binds
/// `accessible-checked` to its own `selected`, which is
/// `selected-date == self.d` (`common/datepicker_base.slint:60-61`, `:163`) —
/// so this asks the widget what date it was seeded with rather than inferring
/// it from anything the host wrote.
fn day_selected(window: &PromptWindow, day: i32) -> bool {
  let wanted = day.to_string();
  only_button(window, "calendar day cell", move |label| label == wanted)
    .accessible_checked()
    .expect("a calendar day cell declares accessible-checked")
}

/// The hour the open time picker is sitting on, read off its own hour input —
/// an `accessible-role: text-input` labelled `"hour"` whose accessible value
/// is its text (`common/time-picker-base.slint:413-420`).
fn hour_shown(window: &PromptWindow) -> String {
  ElementQuery::from_root(window)
    .match_predicate(|element| element.accessible_label().as_deref() == Some("hour"))
    .find_first()
    .expect("the open time picker must declare an hour input")
    .accessible_value()
    .expect("the hour input declares an accessible-value")
    .to_string()
}

/// What the **screen** shows for one text field — the control's own
/// `accessible-value`, which the widget binds two-way to its `text`
/// (`fluent/lineedit.slint:13`).
fn typed_on_screen(window: &PromptWindow, option: &str, field: &str) -> String {
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .accessible_value()
    .unwrap_or_else(|| panic!("{option}/{field} declares no accessible-value"))
    .to_string()
}

/// Which control was drawn, by the **role** it declares — the discriminant
/// PHASE-07 learned to use after a control-identity claim about
/// `accessible-checked` turned out to be true of the wrong widget.
///
/// `Slider` and `TextInput` are the two `number` can be
/// (`fluent/slider.slint:23`, `fluent/lineedit.slint:11`), and `Checkbox` and
/// `Button` are what the fields beside them are.
fn role_of(window: &PromptWindow, option: &str, field: &str) -> AccessibleRole {
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .accessible_role()
    .unwrap_or_else(|| panic!("{option}/{field} declares no accessible-role"))
}

/// The range the **control itself** declares: its minimum and its maximum.
///
/// Read off the widget rather than off the row model, so a case asking *was a
/// range invented?* is asking the thing a person's screen reader would ask. A
/// `LineEdit` declares neither, which is the whole of AC-9's second half: an
/// unbounded `number` must not acquire a range on the way to the screen.
///
/// **`accessible-value-step` is deliberately not read here.** Slint binds it to
/// `min(root.step, (maximum - minimum) / 100)` (`fluent/slider.slint:29`) — a
/// cap — and this design's step is exactly that hundredth, so the reading is
/// the cap whatever the markup ships and would stay `0.1` on a slider left at
/// Slint's default step of `1`. What measures the shipped step is
/// [`step_once`], which moves the value by it.
fn range_on_screen(window: &PromptWindow, option: &str, field: &str) -> (Option<f32>, Option<f32>) {
  let control = field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"));
  (
    control.accessible_value_minimum(),
    control.accessible_value_maximum(),
  )
}

/// Take one step up the slider, the way a keyboard or an assistive technology
/// does: `accessible-action-increment` calls `base.increment()`, which is
/// exactly `set-value(value + step)` and returns without raising anything when
/// the result equals the value it already holds
/// (`common/slider-base.slint:117-131`).
///
/// So this measures the **shipped** step and the third clause of
/// `slider_bounds` at once: a slider whose step does not move the value is one
/// this call cannot move either.
fn step_once(window: &PromptWindow, option: &str, field: &str) {
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .invoke_accessible_increment_action();
}

/// Move one slider, the way an assistive technology does. A `Slider`'s
/// `accessible-action-set-value` parses the string and calls `base.set-value`
/// (`fluent/slider.slint:30-34`), which raises `changed` and nothing else
/// (`common/slider-base.slint:117-124`) — the same callback a pointer drag
/// raises, and the reason the markup binds `changed` rather than `released`.
///
/// No pointer and no layout, so this reaches the control under
/// `init_no_event_loop` exactly as [`type_into`] does.
fn slide_to(window: &PromptWindow, option: &str, field: &str, value: f32) {
  field_described(window, option, field)
    .unwrap_or_else(|| panic!("no control described {field:?} under {option:?}"))
    .set_accessible_value(value.to_string());
}

/// The `ComboBox` a `choice` field drew.
///
/// **Role-filtered, and that is not tidiness.** A view may legally carry a
/// field id equal to an option id (`R-52`), and the option's own `Button` then
/// answers to the same accessible description under the same scope — so
/// [`field_described`]'s unfiltered `find_first` would return whichever the
/// walk reached first and report no ambiguity. `element_described` filters by
/// element type for the mirror-image reason; this filters by role, because a
/// `ComboBox` is a composite and the type name beneath it is not the widget's.
fn combo_box(window: &PromptWindow, option: &str, field: &str) -> ElementHandle {
  within_option(window, option)
    .match_accessible_role(AccessibleRole::Combobox)
    .match_predicate(described(field))
    .find_first()
    .unwrap_or_else(|| panic!("no combo box described {field:?} under {option:?}"))
}

/// What the **screen** shows for one `choice` field: the `ComboBox`'s own
/// `current-value`, which it publishes as its accessible value
/// (`fluent/combobox.slint:32`).
///
/// A **label**, not an id. That is the point of reading it: the wire carries
/// the alternative's id and the screen carries its label, so a case that
/// asserts both is asserting they are different things.
fn chosen_on_screen(window: &PromptWindow, option: &str, field: &str) -> String {
  combo_box(window, option, field)
    .accessible_value()
    .unwrap_or_else(|| panic!("{option}/{field} declares no accessible-value"))
    .to_string()
}

/// Open one `ComboBox`'s popup, the way an assistive technology does:
/// `accessible-action-expand` calls `base.show-popup()`
/// (`fluent/combobox.slint:33`).
fn expand(window: &PromptWindow, option: &str, field: &str) {
  combo_box(window, option, field).invoke_accessible_expand_action();
}

/// Every `ListItem` the window is showing, in tree order — which for an open
/// `ComboBox` popup is the model's order, because the popup's repeater is
/// `for value[index] in root.model` (`fluent/combobox.slint:132`).
///
/// From the **root** and not from the option: a `PopupWindow` is not a
/// descendant of the element that opened it. `ElementQuery::find_all` walks
/// `active_popups` beside the window's own tree (`search_api.rs:304-312`),
/// which is the same reach PHASE-07's picker helpers use.
fn list_items(window: &PromptWindow) -> Vec<ElementHandle> {
  ElementQuery::from_root(window)
    .match_accessible_role(AccessibleRole::ListItem)
    .find_all()
}

/// The labels the open popup is offering, in declared order.
fn offered(window: &PromptWindow) -> Vec<String> {
  list_items(window)
    .iter()
    .map(|item| {
      item
        .accessible_label()
        .unwrap_or_else(|| panic!("a ListItem declares an accessible-label"))
        .to_string()
    })
    .collect()
}

/// Choose one alternative by the label a person reads, the way a person does:
/// **click the box open, then arrow down to the row and press Return.**
///
/// **Why not a click on the row, which is what `design.md` §9's driver table
/// names.** `ElementHandle::mock_single_click` dispatches at
/// `absolute_center()`, and `absolute_position` is `item.map_to_window(..)`,
/// which for an item inside an embedded `PopupWindow` stops at the **popup's**
/// own root — a popup is a separate item tree with no parent link
/// (`i-slint-core/item_tree.rs:628-630`). Pointer dispatch then translates by
/// the popup's origin in the window (`window.rs:848-856`,
/// `geom.contains(pos - coordinates)`), so a popup-local coordinate is read as
/// a window one and lands outside the popup. That is not the risk §8 **R9**
/// named and it is not fixed by the tier it named: PHASE-09/VA-1 measured the
/// rows acquiring real geometry — `(4, 4) 512x40`, then `(4, 44)`, then
/// `(4, 84)` — and the click still not arriving.
///
/// **The keyboard reaches the same function.** `move-selection-down()` is
/// `select(current-index + 1)` and a row's `clicked` is `select(index)`
/// (`common/combobox-base.slint:20-39`, `fluent/combobox.slint:138-142`) — one
/// function, so what this drives is what a pointer would have driven: the
/// index is assigned and `selected` is raised, once. The opening click is a
/// real `mock_single_click`, on the `ComboBox` itself, which is an ordinary
/// laid-out element of the window; it focuses the box and shows the popup
/// (`combobox-base.slint:124-127`), and the popup's `FocusScope` then takes
/// the arrow keys (`fluent/combobox.slint:113-122`).
///
/// **The popup must be closed when this is called**, which is the state a
/// person finds it in: the opening click is dispatched at the box's own centre
/// and an open popup covers it.
fn choose(window: &PromptWindow, option: &str, field: &str, label: &str) {
  combo_box(window, option, field).mock_single_click(slint::platform::PointerEventButton::Left);

  let offered = offered(window);
  let at = offered
    .iter()
    .position(|offered| offered == label)
    .unwrap_or_else(|| panic!("no alternative labelled {label:?}, only {offered:?}"));
  // Where the **widget** says it is, read off the row that declares itself
  // selected (`is-selected: index == root.current-index`,
  // `fluent/combobox.slint:134`) rather than off the host's own value channel,
  // which is what the case is about to assert.
  let from = list_items(window)
    .iter()
    .position(|item| item.accessible_item_selected() == Some(true))
    .expect("one row declares itself selected");

  // **One row at a time, and the caller waits in between.** Each arrow raises
  // `selected`, so it raises a `Command::Edit`; `start`'s channel holds one
  // (`mpsc::channel::<Command>(1)`) and a synchronous key press gives `serve`
  // no chance to drain, so a second press inside one call would have its
  // edit dropped by `Wire::send` and the draft would keep the first. That is
  // what a person
  // arrowing quickly gets too, and the guard corrects the widget on the next
  // present — but it is not what a case driving *one* choice means to say.
  assert_eq!(
    at.abs_diff(from),
    1,
    "{label:?} is {} rows from the current selection; choose one at a time and \
     wait for the draft between them",
    at.abs_diff(from)
  );
  press(
    window,
    if at > from {
      slint::platform::Key::DownArrow
    } else {
      slint::platform::Key::UpArrow
    },
  );
  // Return closes the popup and nothing else: the selection was raised by each
  // arrow (`combobox-base.slint:41-54`). Leaving it open would cover the
  // controls the rest of the case drives.
  press(window, slint::platform::Key::Return);
}

/// One key, pressed and released on the window the way a person's keyboard
/// reaches it. `ElementHandle` has no key API — keys go to whatever holds
/// focus, which is the point.
fn press(window: &PromptWindow, key: slint::platform::Key) {
  let window = ComponentHandle::window(window);
  window.dispatch_event(slint::platform::WindowEvent::KeyPressed { text: key.into() });
  window.dispatch_event(slint::platform::WindowEvent::KeyReleased { text: key.into() });
}

/// The index the **value channel** holds for one `choice` field — what the
/// widget's `current-index` is bound to, and so the host's own account of what
/// it drew.
fn indexed(window: &PromptWindow, option: &str, field: &str) -> i32 {
  value_of(window, option, field)
    .unwrap_or_else(|| panic!("no value slot for {field:?} under {option:?}"))
    .index
}

// ---------------------------------------------------------------------------
// Driving the production loop
// ---------------------------------------------------------------------------

/// Everything one case drives, assembled exactly as `main.rs` assembles it:
/// a headless window and tray, a glass over them, the room the query needs, a
/// host over a real child process, and `install`'s callback table around a
/// **capacity-1** channel (`start`'s `mpsc::channel::<Command>(1)`).
///
/// A plain struct built by a plain function, and every case runs its own
/// `LocalSet` around the production `serve` — the shape every other
/// `serve`-driven case in this target has. There is no second loop and no
/// test-only harness for it (EX-1). Wrapping the loop in a **function** of its
/// own would make that function an `async fn` holding a `!Send` component
/// handle, which `future_not_send` denies workspace-wide (`Cargo.toml:201`) —
/// which is why the loop below is a macro and this is not.
struct Rig {
  window: PromptWindow,
  tray: Tray,
  glass: SlintGlass,
  backend: Host<ProcessBackend>,
  commands: mpsc::Receiver<Command>,
  cancel: Cancel,
  notice: Notice,
  log: PathBuf,
}

/// [`Rig`] for one case, against `logs-the-request-then-answers.sh` answering
/// `instructions` in order.
///
/// **`case` is a path, not a label.** `scripting::marker` turns it into
/// `goad-invocations-<case>-<pid>` in the temp directory and **clears it at
/// handout**, and every case in this target shares one pid — so two cases given
/// the same name share one log, and whichever starts second clears it from
/// under the first.
///
/// `scripting::claim` now checks for that collision and panics naming the case,
/// so the names here being prefixed with this file's own — as `wiring.rs`'s and
/// `table.rs`'s are — is a courtesy to the reader rather than the only thing
/// standing between this target and an intermittent failure. It was the only
/// thing until slice 007's audit, and two pairs had already slipped past it
/// (`review-code.md` F-5, F-8).
fn rigged(case: &str, instructions: &[&str]) -> Rig {
  let (window, tray) = window_and_tray();
  with_room_for_the_form(&window);
  // One handle, bound before both readers and cloned into each. The glass
  // overlays what the callbacks hold, so a second `Debounce` here would leave
  // every case below green and measure nothing (`design.md` §8 R10).
  let pending = Rc::new(Debounce::new());
  let glass = glass_overlaying(&window, &tray, &pending);
  let (command, log) = logging_scripted(case, instructions);
  let backend = host(command, TIMEOUT, now());
  let (commands_in, commands) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let notice = Notice::new();
  // The one `Wire`, cloned into each callback and nowhere else — and the
  // sender lives only inside it, so nothing in this file can put a `Command`
  // on the channel except by activating an element a person would.
  //
  // The debounce likewise: reachable only through the callbacks and the
  // glass, so a case cannot hold an edit or flush one except by driving a
  // control. The `Rig` retains neither — the glass holds one clone and the
  // callbacks the other, which is all the sharing the overlay needs.
  install(
    &window,
    &tray,
    &Wire::new(commands_in, cancel.clone(), notice.clone()),
    &pending,
  );
  Rig {
    window,
    tray,
    glass,
    backend,
    commands,
    cancel,
    notice,
    log,
  }
}

// ---------------------------------------------------------------------------
// The cases
// ---------------------------------------------------------------------------

/// `tick!(window, option, field)` — tick one box and wait for the loop to
/// take it, which is what every tick in this file is.
///
/// **The wait is not optional.** The click flips the widget immediately —
/// that is the feedback — so the widget's own state says nothing about
/// whether the edit reached the loop, and `start`'s channel holds one
/// (`mpsc::channel::<Command>(1)`): a second click sent before the first is
/// drained is dropped by `Wire::send`, and the next present writes the tick
/// back off the screen.
/// Waiting on the draft's own projection is what keeps that from happening,
/// and a drop then fails as a timeout rather than as a wrong value.
///
/// A macro rather than a function for the same reason [`driving!`] is one:
/// the wait is an `.await`, and a function holding it would be an `async fn`
/// whose future holds a `!Send` component handle.
macro_rules! tick {
  ($window:expr, $option:expr, $field:expr) => {{
    let (window, option, field) = (&$window, $option, $field);
    click(window, option, field);
    until(LIVENESS_BOUND, || drafted(window, option, field)).await;
  }};
}

/// `settled!(window, tray, at)` — ask for a check, wait for the **fold** to
/// reach the screen, and answer with what the `datetime` field then shows.
///
/// **What a case asserting an absence needs.** *Nothing was recorded* cannot
/// be waited for, so a case that merely read the button after a cancel would
/// be asserting that nothing had arrived **yet** — and a defect that did
/// record something would fail later, as a dropped `Choose` and a timeout,
/// rather than here as a value. Driving a round trip the host must present
/// before the read turns that into a comparison: had the cancel recorded
/// anything, this present is where it would appear.
///
/// `view: null` folds as `Shift::Retained`, which leaves the draft exactly as
/// it was, so the round trip itself changes nothing it is being used to
/// observe. The wait is on the next-check line the production glass wrote from
/// the fold, at an instant no other exchange in the case could have
/// produced — `a_present_that_changes_nothing_leaves_a_half_filled_form_on_the_screen_and_on_the_wire`
/// is the precedent.
///
/// A macro rather than a function for the reason [`tick!`] and [`driving!`]
/// are: it holds an `.await`, and a function holding it would be an `async fn`
/// whose future holds a `!Send` `PromptWindow` — measured, as
/// `future_not_send` refusing exactly this (`Cargo.toml:201`).
macro_rules! settled {
  ($window:expr, $tray:expr, $at:expr) => {{
    let (window, tray) = (&$window, &$tray);
    let folded = next_check_line(instant($at));
    assert_ne!(
      window.get_next_check(),
      folded.as_str(),
      "the line must not already be there, or the wait below proves nothing"
    );
    // **Yield before sending.** `start`'s channel holds one
    // (`mpsc::channel::<Command>(1)`) and `serve` shares this thread through
    // `spawn_local`, so a step that
    // wrongly put a `Command::Edit` on the channel is still holding it when
    // this one runs: without the yield the check below is the send that is
    // dropped, and the defect fails as a timeout that names nothing rather
    // than as a value on the screen. In the case where nothing was recorded —
    // the one this macro is for — there is nothing on the channel and the
    // yield changes nothing.
    tokio::task::yield_now().await;
    tray.invoke_check_now();
    until(LIVENESS_BOUND, || {
      window.get_next_check() == folded.as_str()
    })
    .await;
    shown_on_screen(window, "morning", "when")
  }};
}

/// `driving!(rig, |window, tray, log| { … })` — the loop every case below
/// runs, expanded at each of them.
///
/// It takes a [`Rig`] apart, spawns the **production** `serve` on a
/// `LocalSet`, has the person ask for a check through the tray, waits for the
/// view to reach the window, runs the case's own body, stops the loop and
/// checks it ended stopped. It yields `(what the body produced, the log)`.
///
/// **A macro rather than a function, and the reason is the lint.** A function
/// wrapping this would be an `async fn` whose future holds a `PromptWindow`,
/// which is `!Send` by construction — and `future_not_send` is `deny`
/// workspace-wide (`Cargo.toml:201`), reaching an `-> impl Future` return just
/// as it reaches `async fn`. A macro expands into each case's own
/// `#[tokio::test]` body, where the lint does not apply because the expanded
/// test function is not async at all. It also puts the `serve` call in the
/// body below textually inside every case, which is what EX-1's *"no second
/// loop and no test-only harness"* is about. `table.rs:190`'s `observed!` is
/// the precedent for a case-file macro in this target.
macro_rules! driving {
  ($rig:expr, |$window:ident, $tray:ident, $log:ident| $body:block) => {{
    let Rig {
      window: $window,
      tray: $tray,
      glass,
      backend,
      commands,
      cancel,
      notice,
      log: $log,
    } = $rig;
    let stopper = cancel.clone();
    let produced = LocalSet::new()
      .run_until(async {
        let handle = tokio::task::spawn_local(async move {
          serve(
            backend,
            Controller::new(),
            commands,
            cancel,
            notice,
            stub_clock,
            glass,
            Ingress::none(),
          )
          .await
        });
        // The person asks for a check, through the tray callback `install`
        // wired — never through a `Command` a test built. The channel is
        // empty and holds one, so this send cannot be dropped.
        $tray.invoke_check_now();
        until(LIVENESS_BOUND, || $window.get_heading() == "Proceed?").await;

        let produced = $body;

        stopper.stop();
        let served = handle.await.expect("serve must not panic");
        assert_eq!(served.ending, Ending::Stopped);
        produced
      })
      .await;
    (produced, $log)
  }};
}

/// VT-1 — **AC-1.** One option carrying three `boolean` fields: three boxes
/// are found on screen by their own ids and two are ticked, the option's
/// button is pressed, and the `respond` that leaves the host carries `values`
/// with **exactly three keys**, each a JSON boolean equal to what the checkbox
/// showed.
///
/// AC-1's *"draws N checkboxes and one button"* is verified here by reaching
/// all four — `checked_on_screen` and `option_control` each panic naming what
/// they could not find. That there is **exactly** one control per option is
/// `tree.rs`'s `heading_body_and_one_control_per_option_render_in_order`,
/// counted through `accessible-item-count`, and that a zero-field option adds
/// no element at all is its `an_option_with_no_fields_adds_no_element`. This
/// case re-counts neither.
///
/// The screen is read immediately before the press and the wire is compared
/// **to it**, rather than both to a written-out expectation. That is AC-1's
/// own wording — *"each a JSON boolean matching what was on screen"* — and it
/// is the pairing the untouched third field makes worth having: `read` is on
/// the wire as `false` because the host drew it, not because anyone answered
/// it.
#[tokio::test]
async fn every_drawn_field_of_the_pressed_option_reaches_the_wire_as_the_screen_showed_it() {
  let (screen, log) = driving!(
    rigged("fields-vt1", &[THREE_FIELDS]),
    |window, tray, log| {
      tick!(window, "morning", "stretched");
      tick!(window, "morning", "tidied");

      let screen = screen_of(&window, "morning", ["read", "stretched", "tidied"]);

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      screen
    }
  );

  assert_eq!(
    screen,
    vec![("read", false), ("stretched", true), ("tidied", true)],
    "the three boxes must read back off the window as they were left"
  );

  let values = submitted_values(&log, 2);
  assert_eq!(
    keys_of(&values),
    vec!["read", "stretched", "tidied"],
    "exactly the three fields the host drew, and no other key: {values:?}"
  );
  for (field, shown) in screen {
    assert_eq!(
      values[field],
      Value::Bool(shown),
      "{field} left the host as a JSON boolean equal to what the screen showed"
    );
  }
}

/// VT-2 — **AC-4.** Two options carrying fields and **sharing the field id
/// `read`**: a box is ticked under each, one option's button is pressed, and
/// the request names that option and carries only that option's keys.
///
/// **The shared id is the point, not the fixture.** A case that ticked and
/// answered the same option would be green both where the design is right and
/// where the draft key's `option` half is ignored, because one option's
/// answer cannot tell the two readings apart. What tells them apart is
/// ticking the shared id under the option that is **not** answered: `read`
/// leaves the host as `false` under `morning` because `morning`'s `read` and
/// `evening`'s `read` are two independent keys (R-52), and a draft keyed by
/// field alone sends `true` — measured, by making `Draft::state_of` ignore the
/// option half and watching this case go red with the other three green.
///
/// So this case ticks **two** boxes where VT-2 as written ticks one. The
/// extra tick is what makes VA-1's named injection reach it; VT-2's own two
/// clauses — a box ticked under the answered option through the option-scoped
/// query, and that option's button pressed — are both still here.
#[tokio::test]
async fn a_field_id_shared_by_two_options_is_two_keys_and_only_the_answered_ones_are_sent() {
  let ((), log) = driving!(rigged("fields-vt2", &[TWO_FORMS]), |window, tray, log| {
    tick!(window, "morning", "stretched");
    tick!(window, "evening", "read");

    assert!(
      checked_on_screen(&window, "evening", "read"),
      "the tick landed on evening's box"
    );
    assert!(
      !checked_on_screen(&window, "morning", "read"),
      "and not on morning's, which shares its id: the query is scoped to the option"
    );

    option_control(&window, "morning").invoke_accessible_default_action();
    until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
  });

  assert_eq!(
    answered_option(&log, 2),
    "morning",
    "the request names the option that was pressed"
  );
  let values = submitted_values(&log, 2);
  assert_eq!(
    keys_of(&values),
    vec!["read", "stretched"],
    "exactly morning's own fields: `tidied` is evening's and never appears here"
  );
  assert_eq!(values["stretched"], Value::Bool(true));
  assert_eq!(
    values["read"],
    Value::Bool(false),
    "the shared id under the option that was not ticked is a different key, and it \
     goes out as it was drawn"
  );
}

/// VT-3 — **AC-5, both halves, and both required.**
///
/// A box is ticked, an `evaluate` answering `view: null` is folded as
/// `Shift::Retained` — a present that changed nothing — and then the form is
/// submitted. The box is **still ticked on screen**, and the value is still
/// `true` on the wire.
///
/// **A conjunction, where every other case in this file is a disjunction.**
/// EX-4's rule is *reads the log or asserts the screen*; here neither half
/// substitutes for the other, and both directions are measured (VA-1):
///
/// - with the markup's `checked` no longer reading the row model, the wire
///   half alone stays **green** while every box on the screen is wrong;
/// - with `answer()` submitting an empty map, the screen half alone stays
///   **green** while the wire carries nothing.
///
/// It is also the **only** case in this target that holds the fold itself:
/// making `Shift::Retained` drop the draft reddens this one and leaves the
/// other 181 green. `design.md` §8/R-8 accepts the risk of a scheduled firing
/// clobbering a half-filled form on the ground that `view: null` folds to
/// `Retained` and leaves the draft alone; that sentence is this case's
/// premise.
///
/// **The fold is waited for, not assumed.** The invocation log says an
/// exchange *began*: the script appends the request before it answers
/// (`logs-the-request-then-answers.sh:19`, `:26`, `:32`) and the host folds
/// the outcome in later still. So the wait is on the next-check line the
/// production glass wrote from the fold — `scheduling.rs`'s `absorbed_line` is
/// the precedent — and the instant is one only the second exchange could have
/// produced.
#[tokio::test]
async fn a_present_that_changes_nothing_leaves_a_half_filled_form_on_the_screen_and_on_the_wire() {
  let (still_ticked, log) = driving!(
    rigged("fields-vt3", &[THREE_FIELDS, RETAINED]),
    |window, tray, log| {
      tick!(window, "morning", "read");

      // The second exchange, asked for the way the first was.
      let folded = next_check_line(instant(RETAINED_AT));
      assert_ne!(
        window.get_next_check(),
        folded.as_str(),
        "the line must not already be there, or the wait below proves nothing"
      );
      tray.invoke_check_now();
      until(LIVENESS_BOUND, || {
        window.get_next_check() == folded.as_str()
      })
      .await;

      let still_ticked = screen_of(&window, "morning", ["read", "stretched", "tidied"]);

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 3).await;
      still_ticked
    }
  );

  assert_eq!(
    still_ticked,
    vec![("read", true), ("stretched", false), ("tidied", false)],
    "the present that changed nothing left the half-filled form on the screen"
  );

  let values = submitted_values(&log, 3);
  assert_eq!(keys_of(&values), vec!["read", "stretched", "tidied"]);
  assert_eq!(
    values["read"],
    Value::Bool(true),
    "and the draft it was written back from is still what the answer is built from"
  );
}

// ---------------------------------------------------------------------------
// Slice 009 PHASE-01 — the two channels
// ---------------------------------------------------------------------------

/// Slice 009 `plan.md` PHASE-01/**VT-1**. A present that replaces `values`
/// wholesale destroys no element.
///
/// The instrument is `inits`, a counter in production markup (§7 D15): an
/// element that is destroyed and recreated runs its `init` handler again, and
/// one written in place does not. Element identity is not otherwise
/// observable — reading a control's *value* cannot tell the two apart, because
/// a rebuilt element and a preserved one both end up holding the model's value
/// (`docs/memory/a-present-destroys-the-widget-it-writes.md`).
///
/// The second present is a real one, not a second call made by the test: a
/// `view: null` answer folds as `Shift::Retained`, and `serve` presents at the
/// top of every iteration, so the whole of what is driven is a person asking
/// for another check. The wait is on the next-check line the production glass
/// wrote from the fold — `a_present_that_changes_nothing_…` above is the
/// precedent for both the technique and the instant.
///
/// The count is read **after** the view is on screen, so the number compared
/// is a real one: a case that asserted `inits == 0` throughout would pass on a
/// window that drew nothing at all. That is what the `drawn > 0` assertion is.
#[tokio::test]
async fn a_present_of_the_same_view_rewrites_the_values_and_destroys_no_element() {
  let ((drawn, survived), _log) = driving!(
    rigged("fields-split-vt1", &[THREE_FIELDS, RETAINED]),
    |window, tray, log| {
      let drawn = window.get_inits();

      let folded = next_check_line(instant(RETAINED_AT));
      assert_ne!(
        window.get_next_check(),
        folded.as_str(),
        "the line must not already be there, or the wait below proves nothing"
      );
      tray.invoke_check_now();
      until(LIVENESS_BOUND, || {
        window.get_next_check() == folded.as_str()
      })
      .await;

      (drawn, window.get_inits())
    }
  );

  assert!(
    drawn > 0,
    "the window must have drawn something for a survival count to mean anything"
  );
  assert_eq!(
    survived, drawn,
    "the same view was presented again: not one element may have been rebuilt"
  );
}

/// Slice 009 `plan.md` PHASE-01/**VT-2**. A present carrying a **new**
/// `view_id` rebuilds the rows, and `inits` moves.
///
/// This is the control that shows VT-1's counter is capable of moving at all.
/// Without it a `present` that never wrote the row model under any
/// circumstance would pass VT-1 — and so would a counter wired to nothing.
///
/// The second fixture carries its own title so the wait has something to watch
/// that only the replacement could have produced; `ANOTHER_FORM` also declares
/// a different option and field, so the rebuild is visible at the screen as
/// well as at the counter.
#[tokio::test]
async fn a_present_carrying_a_new_view_rebuilds_the_rows() {
  let ((drawn, rebuilt), _log) = driving!(
    rigged("fields-split-vt2", &[THREE_FIELDS, ANOTHER_FORM]),
    |window, tray, log| {
      let drawn = window.get_inits();

      tray.invoke_check_now();
      until(LIVENESS_BOUND, || window.get_heading() == "Anything else?").await;

      (drawn, window.get_inits())
    }
  );

  assert!(drawn > 0, "the first view must have drawn its three fields");
  assert!(
    rebuilt > drawn,
    "a replacement view's rows are written, which destroys and recreates every \
     element beneath them: {drawn} then {rebuilt}"
  );
}

// ---------------------------------------------------------------------------
// Slice 009 PHASE-05 — `text`, and the debounce's delivery
// ---------------------------------------------------------------------------

/// Slice 009 `plan.md` PHASE-05/**VT-1**. A `text` field draws a `LineEdit`,
/// and the option still answers.
///
/// The control is found by the same option-scoped description query every
/// other field case uses, so *it drew* and *it is addressable* are one
/// assertion. What separates a `LineEdit` from the `CheckBox` beside it is the
/// property each declares: a text input carries an `accessible-value` and no
/// `accessible-checked`, and a checkbox the reverse. Both are read, in both
/// directions, because either one alone would pass on a window that drew two
/// of the same control.
///
/// The answer half is `R-57`'s typing for a kind nothing had submitted before:
/// an untouched `text` field goes out as a JSON **string**, and the empty one
/// is the as-drawn value rather than an absence — `R-58` forbids omitting a
/// value for a drawn field.
#[tokio::test]
async fn a_text_field_draws_a_line_edit_and_the_option_still_answers() {
  let ((typed, checked_value, typed_value), log) = driving!(
    rigged("fields-text-vt1", &[TWO_TEXT_FIELDS]),
    |window, tray, log| {
      let noted = field_described(&window, "morning", "noted")
        .expect("the text field must draw a control addressable by its own id");
      let stretched = field_described(&window, "morning", "stretched")
        .expect("and the boolean beside it must still draw one");

      let typed = (
        noted.accessible_value().map(|value| value.to_string()),
        noted.accessible_checked(),
      );
      let checked_value = (
        stretched.accessible_value().map(|value| value.to_string()),
        stretched.accessible_checked(),
      );
      let typed_value = typed_on_screen(&window, "morning", "also");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      (typed, checked_value, typed_value)
    }
  );

  assert_eq!(
    typed,
    (Some(String::new()), None),
    "a text input declares a value and no checked state"
  );
  assert_eq!(
    checked_value,
    (None, Some(false)),
    "and the checkbox beside it declares a checked state and no value — so the two \
     controls are told apart by what they are, not by where they sit"
  );
  assert_eq!(typed_value, "", "an untouched text field shows nothing");

  let values = submitted_values(&log, 2);
  assert_eq!(
    keys_of(&values),
    vec!["also", "noted", "stretched"],
    "every drawn field of the option, the two new ones included: {values:?}"
  );
  assert_eq!(
    values["noted"],
    Value::String(String::new()),
    "R-57: a text field leaves the host as a JSON string, and an untouched one carries \
     the empty string rather than no key at all"
  );
  assert_eq!(values["also"], Value::String(String::new()));
  assert_eq!(values["stretched"], Value::Bool(false));
}

/// Slice 009 `plan.md` PHASE-05/**VT-3** — **AC-4, the element half.**
///
/// Two text fields are typed into and then the option is pressed. Everything
/// typed reaches the draft and both controls still show it.
///
/// **The `inits` comparison here is vacuous, and saying otherwise was this
/// doc's defect** (F-S1). Between the two `get_inits()` readings there is no
/// `.await`, so `serve` — a `spawn_local` task on the same `LocalSet` — cannot
/// be scheduled; nothing has been enqueued for it in any case, because
/// `install`'s `on_edited` callback routes a `Reported::Typed` into
/// `Debounce::hold` rather than sending a command, and this target runs under
/// `init_no_event_loop`, so the
/// timer never fires. **No present occurs between the readings**, and the
/// equality is guaranteed by the executor rather than by D8. Mutation-confirmed:
/// delete the `if self.shown != showing` guard in `SlintGlass::present`, so
/// every present destroys every field element, and this case still passes.
///
/// It is kept, because the *draft and wire* half above is real and is this
/// case's actual subject. What it does not carry is AC-4's element half.
///
/// **That half is held, by four other cases**, each of which reddens under the
/// same mutation — `numeric_guard::a_present_inside_the_window_does_not_write_a_zero_back_over_a_cleared_field`,
/// `overlay::a_present_shows_a_held_edit_and_corrects_one_the_host_never_recorded`,
/// `reassert::a_second_present_corrects_nothing_and_a_widget_the_host_never_heard_from_is_corrected`,
/// and `fields::a_present_of_the_same_view_rewrites_the_values_and_destroys_no_element`.
/// So D8 is well held; what was wrong was `plan.md` §Coverage and `design.md`
/// §9 naming *this* case for it.
///
/// **The answer is what makes this tier possible.** This target runs under
/// `init_no_event_loop`, where no timer ever fires — so the only thing that
/// can deliver what was typed is the `Choose` the press sends, carrying the
/// map's whole contents in one command. The timer's own half of the delivery
/// rule is measured where a timer can fire, in `tests/event_loop_debounce/`.
///
/// **Two fields, not one.** A map keyed by (option, field) and a single held
/// edit are indistinguishable until a second field is typed into inside the
/// same window: with one entry the second keystroke would simply replace the
/// first field's, and only `also` would reach the wire.
///
/// The count is read after the view is on screen, so the number compared is a
/// real one — `drawn > 0` is what keeps it from passing on a window that drew
/// nothing.
#[tokio::test]
async fn two_text_fields_typed_into_inside_one_window_both_reach_the_wire_and_neither_is_rebuilt() {
  let ((drawn, survived, screen), log) = driving!(
    rigged("fields-text-vt3", &[TWO_TEXT_FIELDS]),
    |window, tray, log| {
      let drawn = window.get_inits();

      type_into(&window, "morning", "noted", "walked before breakfast");
      type_into(&window, "morning", "also", "and again after");

      let survived = window.get_inits();
      let screen = (
        typed_on_screen(&window, "morning", "noted"),
        typed_on_screen(&window, "morning", "also"),
      );

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      (drawn, survived, screen)
    }
  );

  assert!(
    drawn > 0,
    "the window must have drawn something for a survival count to mean anything"
  );
  assert_eq!(
    survived, drawn,
    "not one element may have been rebuilt while a person was typing into it: \
     {drawn} then {survived}"
  );
  assert_eq!(
    screen,
    (
      "walked before breakfast".to_owned(),
      "and again after".to_owned()
    ),
    "and both controls still show what was typed into them"
  );

  let values = submitted_values(&log, 2);
  assert_eq!(keys_of(&values), vec!["also", "noted", "stretched"]);
  assert_eq!(
    values["noted"],
    Value::String("walked before breakfast".to_owned()),
    "the first field's text survived the person moving to the second: {values:?}"
  );
  assert_eq!(
    values["also"],
    Value::String("and again after".to_owned()),
    "and the second field's reached the draft in the same one command"
  );
}

/// Slice 009 `plan.md` PHASE-07/**VT-2** — the picker chain, end to end.
///
/// The button opens the date picker, a day cell is chosen, `OK` closes it and
/// opens the time picker, an hour is chosen, and `OK` there composes and
/// reports. Everything is driven through `invoke_accessible_default_action`,
/// and every element it reaches declares an `accessible-action-default` that
/// calls its own `clicked` — so no step depends on a popup having been laid
/// out (`harness`-adjacent note on [`only_button`]).
///
/// **The draft holds one `Picked`, and the button shows it.** The two halves
/// are asserted against different derivations on purpose: the *shape* is
/// checked against a prefix built from the clock and the two literals a person
/// chose, which `instant::compose` had no part in, and the *exact* value is
/// checked against `compose` itself, which is what makes the screen and the
/// wire provably one string rather than two that happen to agree. The
/// arithmetic `compose` performs is PHASE-04/VT-1 … VT-3's and is not re-asserted
/// here.
///
/// The date is the current month's 15th because the calendar opens on the
/// seed's month and every month has a 15th; the hour is 3 because
/// `get-current-time` returns the selected hour unchanged while the period
/// selector holds its default (`common/time-picker-base.slint:494-503`).
#[tokio::test]
async fn picking_a_date_and_then_a_time_records_one_instant_and_the_button_shows_it() {
  let (today, _) = instant_today();
  let picked = composed(today.year, today.month, 15, 3);

  let ((shown, held), log) = driving!(
    rigged("fields-datetime-vt2", &[TWO_DATETIME_FIELDS]),
    |window, tray, log| {
      open_picker(&window, "morning", "when");
      pick_day(&window, 15);
      accept(&window);
      pick_hour(&window, 3);
      accept(&window);

      // **A synchronisation point, and the assertion is below it.** Waiting
      // for the slot to stop reading the sentinel rather than for it to hold
      // the expected string is what makes a defect fail as a comparison
      // naming two values instead of as a timeout naming none: a pick that
      // lands *wrong* satisfies this wait and then fails `assert_eq!`. A pick
      // that never lands at all can only ever be a timeout, and is.
      until(LIVENESS_BOUND, || {
        shown_on_screen(&window, "morning", "when") != NOT_SET
      })
      .await;
      let shown = shown_on_screen(&window, "morning", "when");
      let held = shown_on_screen(&window, "morning", "until");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      (shown, held)
    }
  );

  assert!(
    shown.starts_with(&format!("{:04}-{:02}-15T03:00:00", today.year, today.month)),
    "the button shows the day and the hour the person chose, on the month the \
     picker opened on: {shown:?}"
  );
  assert_eq!(
    shown, picked,
    "and it is the composed instant entire, offset included"
  );
  assert_eq!(
    held, NOT_SET,
    "the other datetime field was never picked, and one field's pick is not \
     the form's: {held:?}"
  );

  let values = submitted_values(&log, 2);
  assert_eq!(keys_of(&values), vec!["stretched", "until", "when"]);
  assert_eq!(
    values["when"],
    Value::String(picked.clone()),
    "R-57: the pick leaves the host as the string the button showed, carrying \
     the offset it was resolved in (D4): {values:?}"
  );
  assert_eq!(
    values["until"],
    Value::String("1970-01-01T00:00:00+00:00".to_owned()),
    "and the untouched one goes out as the epoch — the screen and the wire \
     disagree here on purpose (D1, D2)"
  );
}

/// Slice 009 `plan.md` PHASE-07/**VT-1** — the screen and the wire disagree,
/// on purpose.
///
/// An untouched `datetime` field draws a button reading *not set*, and
/// answering the option submits `1970-01-01T00:00:00+00:00` for it. Both
/// halves are required and neither implies the other: `R-58` forbids omitting
/// a value for a drawn field, so *something* has to go out, and D-6 chose a
/// value a backend can recognise as nobody's answer — while a button showing
/// that string would be the host claiming an answer nobody gave (§7 D1, D2).
///
/// The control-identity half is PHASE-05's, one kind on — and the discriminant
/// is **the role**, measured rather than assumed. A `Button` declares a
/// checked state just as a `CheckBox` does (it has a `checkable` property, and
/// binds `accessible-checked` whether or not anything set it), so *declares no
/// checked state* would have been a false statement that happened to pass for
/// the `LineEdit`. What separates all three is `accessible-role` — `button`,
/// `checkbox`, `text-input` — together with the value only a text input
/// declares.
#[tokio::test]
async fn an_untouched_datetime_reads_not_set_on_screen_and_submits_the_epoch() {
  let ((shown, declares), log) = driving!(
    rigged("fields-datetime-vt1", &[TWO_DATETIME_FIELDS]),
    |window, tray, log| {
      let when = field_described(&window, "morning", "when")
        .expect("a datetime field must draw a control addressable by its own id");
      let stretched = field_described(&window, "morning", "stretched")
        .expect("and the boolean beside it must still draw one");
      let declares = (
        when.accessible_value().map(|value| value.to_string()),
        when.accessible_role(),
        stretched.accessible_role(),
      );
      let shown = shown_on_screen(&window, "morning", "when");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      (shown, declares)
    }
  );

  assert_eq!(shown, NOT_SET, "the button says nobody has picked one");
  assert_eq!(
    declares,
    (
      None,
      Some(AccessibleRole::Button),
      Some(AccessibleRole::Checkbox)
    ),
    "it declares no value, which is what tells it from the line edit, and it \
     announces itself as a button where the box beside it announces a checkbox"
  );

  let values = submitted_values(&log, 2);
  assert_eq!(keys_of(&values), vec!["stretched", "until", "when"]);
  for field in ["when", "until"] {
    assert_eq!(
      values[field],
      Value::String("1970-01-01T00:00:00+00:00".to_owned()),
      "R-58 forbids omitting a value for a drawn field, and D-6's sentinel is \
       what goes out instead: {values:?}"
    );
  }
}

/// Slice 009 `plan.md` PHASE-07/**VT-3** — **the re-seed**, both halves.
///
/// A field that has been picked reopens its picker on **its own** pick; a
/// field that has not opens on today. Both pickers are root singletons shared
/// by every `datetime` field in the form, so a seed written once per present
/// rather than once per field would pass the first half and fail the second —
/// which is why the fixture carries two fields and this case reads both
/// (`design.md` §7 D21).
///
/// **Nothing survives a popup closing**, so neither half can be satisfied by
/// leakage: `show-popup` compiles to a fresh `::new()` on every show and the
/// closed instance is dropped from `active_popups`. What the reopened picker
/// is sitting on can only have come from the seed the button's handler wrote
/// out of that field's own value slot (`design.md` §5.4).
///
/// The day picked is chosen so that it is **not** today's: seeding from today
/// would otherwise satisfy the first half by accident, which is the shape
/// `docs/memory/tests-asserting-proxies.md` warns about.
///
/// No pointer event anywhere, so no dependency on a popup having been laid
/// out.
#[tokio::test]
async fn a_picked_field_reopens_on_its_own_pick_and_an_unpicked_one_opens_on_today() {
  let (today, _) = instant_today();
  let elsewhen = if today.day == 15 { 16 } else { 15 };
  let picked = composed(today.year, today.month, elsewhen, 3);

  let ((reopened, untouched, hour, landed), _log) = driving!(
    rigged("fields-datetime-vt3", &[TWO_DATETIME_FIELDS]),
    |window, tray, log| {
      open_picker(&window, "morning", "when");
      pick_day(&window, elsewhen);
      accept(&window);
      pick_hour(&window, 3);
      accept(&window);
      // The same synchronisation point, and for the same reason: the pickers
      // below are read only once the pick they are seeded from has reached
      // the slot they are seeded out of. That the pick that landed is the one
      // this case made is asserted below, so a seed is never read off a slot
      // whose contents were not checked.
      until(LIVENESS_BOUND, || {
        shown_on_screen(&window, "morning", "when") != NOT_SET
      })
      .await;

      // The field nobody has picked, first: its picker must open on today and
      // not on the pick the other field just made.
      open_picker(&window, "morning", "until");
      let untouched = (
        day_selected(&window, today.day),
        day_selected(&window, elsewhen),
      );
      cancel(&window);

      // Then the field that was picked, which must open on its own pick.
      open_picker(&window, "morning", "when");
      let reopened = (
        day_selected(&window, elsewhen),
        day_selected(&window, today.day),
      );
      accept(&window);
      let hour = hour_shown(&window);
      cancel(&window);

      let _ = &log;
      (
        reopened,
        untouched,
        hour,
        shown_on_screen(&window, "morning", "when"),
      )
    }
  );

  assert_eq!(
    landed, picked,
    "the pick this case seeds from is the one that was made, so every seed \
     read above was read off a slot holding a known value"
  );
  assert_eq!(
    untouched,
    (true, false),
    "the unpicked field opens on today, and not on the other field's pick — \
     which is what a seed written once per present rather than once per field \
     would get wrong"
  );
  assert_eq!(
    reopened,
    (true, false),
    "and the picked field opens on its own pick rather than on today"
  );
  assert_eq!(
    hour, "3",
    "the time half of the seed is retained too: the picker opens on the hour \
     that was picked, not on the widget's own default"
  );
}

/// Slice 009 `plan.md` PHASE-07/**VT-4** — cancelling abandons the whole edit,
/// at either picker.
///
/// Two abandonments, because they fail differently. Cancelling the **date**
/// picker never reaches a time at all; cancelling the **time** picker
/// abandons a date that has already been chosen and stashed — which is the
/// one a "cancel means keep the date at local midnight" reading would record
/// (§7 D5). After both, the draft has nothing for the field, the button still
/// reads *not set*, and the wire carries the epoch.
///
/// **A cancel is not a refusal, and neither raises a diagnostic line.** The
/// pane is read as well as the wire, because a host that recorded nothing by
/// reporting an error would satisfy the value assertions and fail the person.
#[tokio::test]
async fn cancelling_either_picker_records_nothing_and_leaves_the_button_alone() {
  let ((after_date, after_time, lines), log) = driving!(
    rigged(
      "fields-datetime-vt4",
      &[TWO_DATETIME_FIELDS, RETAINED, RETAINED_AGAIN]
    ),
    |window, tray, log| {
      open_picker(&window, "morning", "when");
      pick_day(&window, 15);
      cancel(&window);
      let after_date = settled!(window, tray, RETAINED_AT);

      open_picker(&window, "morning", "when");
      pick_day(&window, 15);
      accept(&window);
      pick_hour(&window, 3);
      cancel(&window);
      let after_time = settled!(window, tray, RETAINED_AGAIN_AT);

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 4).await;
      (after_date, after_time, diagnostic_lines(&window))
    }
  );

  assert_eq!(
    after_date, NOT_SET,
    "abandoning at the date picker leaves the button reading what it read"
  );
  assert_eq!(
    after_time, NOT_SET,
    "and abandoning at the time picker abandons the date that was already \
     chosen with it, rather than committing it at midnight"
  );
  assert!(
    lines.is_empty(),
    "a cancel is not a refusal and reports nothing: {lines:?}"
  );

  let values = submitted_values(&log, 4);
  assert_eq!(
    values["when"],
    Value::String("1970-01-01T00:00:00+00:00".to_owned()),
    "nothing was recorded, so the field goes out as it was drawn: {values:?}"
  );
}

// ---------------------------------------------------------------------------
// Slice 009 PHASE-08 — `number` and its two controls
// ---------------------------------------------------------------------------

/// Slice 009 `plan.md` PHASE-08/**VT-2**. A `number` whose range admits a
/// slider draws one; a `number` whose range does not draws the text control.
/// Both submit a number.
///
/// **The control is told apart by its role**, which is the only discriminant
/// that says what a widget *is* rather than what it happens to have declared.
/// The three roles are read in one reading so that a window drawing three of
/// the same control cannot satisfy any of them.
///
/// **The slider's range is the backend's own**, read off the widget rather
/// than off the row: `accessible-value-minimum` and `-maximum` are what a
/// screen reader is told, and the step is a hundredth of the span because
/// Slint's `accessible-value-step` caps at exactly that and this design's step
/// meets the cap (`fluent/slider.slint:29`).
///
/// The slider is then **moved**, which is the half that measures `slider:
/// true` reaching the host: its report is a `Reported::AdjustedValue`, and a
/// renderer sending `slider: false` instead would have the host read its empty
/// `text` slot and keep the number the field already held.
#[tokio::test]
async fn a_number_draws_a_slider_where_one_can_be_operated_and_a_text_field_otherwise() {
  let ((roles, slider_range, text_range, stepped, moved), log) = driving!(
    rigged("fields-number-vt2", &[THREE_NUMBER_FIELDS]),
    |window, tray, log| {
      let roles = (
        role_of(&window, "morning", "rated"),
        role_of(&window, "morning", "counted"),
        role_of(&window, "morning", "frozen"),
        role_of(&window, "morning", "stretched"),
      );
      let slider_range = range_on_screen(&window, "morning", "rated");
      let text_range = range_on_screen(&window, "morning", "counted");

      step_once(&window, "morning", "rated");
      let stepped = typed_on_screen(&window, "morning", "rated");

      slide_to(&window, "morning", "rated", 7.0);
      let moved = typed_on_screen(&window, "morning", "rated");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      (roles, slider_range, text_range, stepped, moved)
    }
  );

  assert_eq!(
    roles,
    (
      AccessibleRole::Slider,
      AccessibleRole::TextInput,
      AccessibleRole::TextInput,
      AccessibleRole::Checkbox
    ),
    "one kind, two controls — and `frozen` takes the text one even though both its \
     bounds round-trip exactly, because a step of a hundredth of one ulp moves \
     nothing. The boolean beside them is neither control"
  );
  assert_eq!(
    slider_range,
    (Some(0.0), Some(10.0)),
    "the slider declares the range the backend sent, and neither bound is one the \
     host chose"
  );
  assert_eq!(
    text_range,
    (None, None),
    "and the text control declares no range at all, because none was sent"
  );
  assert_eq!(
    stepped, "0.1",
    "one keyboard step is a hundredth of the declared span — the step the host \
     computed and proved moves the value, not Slint's default of 1"
  );
  assert_eq!(moved, "7", "and the slider goes where it is put");

  let values = submitted_values(&log, 2);
  assert_eq!(
    keys_of(&values),
    vec!["counted", "frozen", "rated", "stretched"],
    "every drawn field of the option: {values:?}"
  );
  assert_eq!(
    values["rated"],
    Value::from(7.0),
    "R-57: a number field leaves the host as a JSON number, and this one carries what \
     the slider was moved to — which only reaches the draft if the control reported \
     itself as a slider"
  );
  assert_eq!(
    values["counted"],
    Value::from(0.0),
    "and the untouched one carries what it was drawn showing rather than no key at all"
  );
  assert_eq!(
    values["frozen"],
    Value::from(1.267_650_600_228_229_4e30_f64),
    "as does the one no slider could be operated over: taking the text control costs \
     it nothing on the wire"
  );
  assert_eq!(values["stretched"], Value::Bool(false));
}

/// Slice 009 `plan.md` PHASE-08/**VT-3** — **AC-9.** An unbounded `number`
/// draws the text control, what is typed into it reaches the draft, and the
/// value that leaves the host is a JSON number with **no invented range**.
///
/// AC-9's second half is the one worth writing carefully. *No range appears
/// anywhere the backend did not send* is asserted three ways in one case,
/// because each alone would pass on a defect the others catch: the control
/// declares no minimum, maximum or step; what is submitted is neither clamped
/// nor defaulted to a bound; and the value is a JSON **number** rather than
/// the string the control carried it in.
///
/// The number typed is outside every range a bounds-driven implementation
/// might have invented — above a `max` of zero, below a `min` of zero, and not
/// a whole number — so a clamp of any kind would change it.
#[tokio::test]
async fn an_unbounded_number_submits_what_was_typed_and_invents_no_range() {
  let ((range, shown), log) = driving!(
    rigged("fields-number-vt3", &[THREE_NUMBER_FIELDS]),
    |window, tray, log| {
      type_into(&window, "morning", "counted", "-4.5");
      let range = range_on_screen(&window, "morning", "counted");
      let shown = typed_on_screen(&window, "morning", "counted");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      (range, shown)
    }
  );

  assert_eq!(
    range,
    (None, None),
    "a `number` the backend sent no bound for acquires none on the way to the screen"
  );
  assert_eq!(shown, "-4.5", "and the control shows what was put into it");

  let values = submitted_values(&log, 2);
  assert_eq!(
    values["counted"],
    Value::from(-4.5),
    "R-57: a JSON number, unclamped and unrounded — every bound a host might have \
     invented would have moved this one"
  );
  assert!(
    values["counted"].is_number(),
    "and a number rather than the string the control carried it in: {values:?}"
  );
}

/// Slice 009 `plan.md` PHASE-08/**VT-4**. A numeric text the parse refuses is
/// recorded verbatim, and the number the field already held stands.
///
/// **`set_accessible_value` is the point, not a convenience.**
/// `input-type: decimal` gates typed insertions and nothing else — the paste
/// path never consults it, and `set_accessible_value` assigns `text` and calls
/// `edited` from inside the markup (`fluent/lineedit.slint:16`) — so this
/// drives the class the control actually admits, which is every string.
///
/// `12/25` is chosen because it is what a **repair rule would get wrong**: a
/// rule that replaced one foreign character with `.` and parsed again would
/// read it as `12.25`, a number the screen never showed. The host repairs
/// nothing, so the text is recorded verbatim and the last representable number
/// stands (`design.md` §7 D23).
///
/// **Two readings, and neither implies the other.** The channel carries the
/// text, which is what the widget displays and what its guard compares itself
/// against; the wire carries the number. A host that *discarded* the refused
/// entry would put `3` back on the channel and still submit `3`, so the wire
/// alone cannot see it; a host that *repaired* the entry would leave `12/25`
/// on the channel and submit `12.25`, so the channel alone cannot see that.
///
/// The field is drawn at `3` rather than at zero so that *the number it
/// already held* is a number rather than a default. It cannot be a
/// **delivered** edit in this target: no timer fires under
/// `init_no_event_loop`, so the only thing that delivers a held entry is the
/// `Choose` a press carries — and an answer with no new view is
/// `Shift::Closed`, which takes the form down. The drawn minimum is the other
/// half of the same fallback rule (`view_model::interpret`).
#[tokio::test]
async fn a_numeric_text_the_parse_refuses_is_recorded_and_leaves_the_number_alone() {
  let ((channel, shown), log) = driving!(
    rigged("fields-number-vt4", &[A_NUMBER_DRAWN_AT_THREE, RETAINED]),
    |window, tray, log| {
      let drawn = value_of(&window, "morning", "counted")
        .expect("the number field must have a value slot")
        .text
        .to_string();
      assert_eq!(
        drawn, "3",
        "the field is drawn showing its declared minimum, or the fallback below is \
         a fallback to nothing"
      );

      type_into(&window, "morning", "counted", "12/25");

      // A present the case causes, so the reading below is of a channel the
      // production glass has written *since* the entry was made — the same
      // reason `settled!` drives a round trip rather than reading straight
      // after the edit.
      let folded = next_check_line(instant(RETAINED_AT));
      tray.invoke_check_now();
      until(LIVENESS_BOUND, || {
        window.get_next_check() == folded.as_str()
      })
      .await;

      let channel = value_of(&window, "morning", "counted")
        .expect("the number field must still have a value slot")
        .text
        .to_string();
      let shown = typed_on_screen(&window, "morning", "counted");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 3).await;
      (channel, shown)
    }
  );

  assert_eq!(
    channel, "12/25",
    "the text is recorded verbatim, so a present inside the window shows what was \
     entered rather than writing the host's own number back over it"
  );
  assert_eq!(shown, "12/25", "and that is what the control is showing");

  let values = submitted_values(&log, 3);
  assert_eq!(
    values["counted"],
    Value::from(3.0),
    "and the number the field already held stands — not `12.25`, which is what a \
     one-character repair would have read, and not `null`, which is what a \
     non-finite would serialise as"
  );
}

/// Slice 009 `plan.md` PHASE-08/**VT-5**. A `number` declaring `f64::MAX` as
/// its `min` draws a control carrying the `{:e}` spelling rather than 309
/// characters, and submits the `f64` it came from.
///
/// PHASE-02/VT-5 is the formatter's own unit; this is the element half, and
/// neither implies the other — a formatter can be right while nothing routes
/// the drawn value through it, which is exactly what `glass.rs`'s untouched
/// arm did until this phase.
///
/// Both halves are needed for a second reason: the spelling has to **re-parse
/// to the number it came from**, because it is what the guard compares itself
/// against and what a person editing the field starts from. Asserting the
/// screen without the wire would admit a shortened spelling that reads back as
/// something else.
#[tokio::test]
async fn a_number_too_long_to_write_out_is_drawn_in_scientific_notation() {
  let (shown, log) = driving!(
    rigged("fields-number-vt5", &[A_HUGE_MINIMUM]),
    |window, tray, log| {
      let shown = typed_on_screen(&window, "morning", "huge");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      shown
    }
  );

  assert_eq!(
    shown, "1.7976931348623157e308",
    "the `{{:e}}` spelling, not the 309 characters `Display` would have written"
  );
  assert_eq!(
    shown.parse::<f64>().ok(),
    Some(f64::MAX),
    "and it reads back as the number it came from, which is what the guard's \
     comparand needs"
  );

  let values = submitted_values(&log, 2);
  assert_eq!(
    values["huge"],
    Value::from(f64::MAX),
    "R-58: an untouched drawn field carries a value, and it is the one the screen \
     showed rather than an `f32` infinity"
  );
}

// ---------------------------------------------------------------------------
// Slice 009 PHASE-09 — `choice`
// ---------------------------------------------------------------------------

/// Slice 009 `plan.md` PHASE-09/**VT-1**. A `choice` field draws a `ComboBox`
/// whose model is the alternatives' **labels**, in declared order.
///
/// Three readings, because each alone would pass on a defect the others catch.
/// The role says a `ComboBox` was drawn and not one of the four controls
/// beside it. The popup's rows say what it was drawn *over*: the labels the
/// backend authored, in the order it declared them, and none of the ids — a
/// renderer that shipped ids would show a person `badly` where the backend
/// wrote `Badly`. And the value shows the field is sitting on its first
/// alternative, which is what `as_drawn` says an untouched `choice` holds.
///
/// The field id is the option's own id, so [`combo_box`]'s role filter is
/// load-bearing here rather than incidental.
#[tokio::test]
async fn a_choice_field_draws_a_combo_box_over_the_alternatives_labels() {
  let ((roles, labels, shown), _log) = driving!(
    rigged("fields-choice-vt1", &[A_CHOICE_FIELD]),
    |window, tray, log| {
      let roles = (
        combo_box(&window, "morning", "morning")
          .accessible_role()
          .expect("a combo box declares an accessible-role"),
        role_of(&window, "morning", "stretched"),
      );
      expand(&window, "morning", "morning");
      let labels = offered(&window);
      let shown = chosen_on_screen(&window, "morning", "morning");
      (roles, labels, shown)
    }
  );

  assert_eq!(
    roles,
    (AccessibleRole::Combobox, AccessibleRole::Checkbox),
    "the fifth kind draws its own control, and the boolean beside it is not it"
  );
  assert_eq!(
    labels,
    vec!["Badly", "Fine", "Well"],
    "the labels the backend authored, in the order it declared them — not the ids"
  );
  assert_eq!(
    shown, "Badly",
    "an untouched `choice` sits on its first alternative, which is what it submits"
  );
}

/// Slice 009 `plan.md` PHASE-09/**VT-2** — **AC-8.** Choosing an alternative
/// submits the **alternative's id**, and a view whose field id equals an
/// option id still answers correctly.
///
/// **Three things that could be sent and only one that may be.** The label is
/// what a person sees, the index is what the `ComboBox` reports, and the id is
/// what `R-57` requires — so the case asserts the label on screen, the index in
/// the value channel, and the id on the wire, and no two of them are the same
/// string. `fine` is the middle alternative, so an off-by-one and a
/// first-alternative fallback are both visible.
///
/// **The field id is the option id, and that is AC-8's other half.** Both the
/// field's `ComboBox` and the option's `Button` answer to the accessible
/// description `morning`; the case chooses through the one and answers through
/// the other, and a query that confused them would drive the wrong widget.
///
/// The host cannot mint an `AlternativeId` — `AlternativeId::new` is
/// `pub(super)` in `goad-semantics` — so an id reaching the wire was
/// necessarily cloned off the view the backend sent. That is why this is a
/// fact about the types and not a rule somebody follows (`design.md` §7 D12).
#[tokio::test]
async fn choosing_an_alternative_submits_its_id_where_the_field_id_is_the_options_own() {
  let ((shown, index), log) = driving!(
    rigged("fields-choice-vt2", &[A_CHOICE_FIELD]),
    |window, tray, log| {
      choose(&window, "morning", "morning", "Fine");
      // **A synchronisation point, and the assertions are below it.** Waiting
      // for the slot to stop reading the value it was *drawn* with, rather
      // than for it to hold the expected one, is what makes a choice that
      // lands **wrong** fail as a comparison naming two indices instead of as
      // a timeout naming none. A choice that never lands at all can only ever
      // be a timeout, and is.
      until(LIVENESS_BOUND, || {
        indexed(&window, "morning", "morning") != 0
      })
      .await;
      let shown = chosen_on_screen(&window, "morning", "morning");
      let index = indexed(&window, "morning", "morning");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      (shown, index)
    }
  );

  assert_eq!(shown, "Fine", "the label is what the person is looking at");
  assert_eq!(index, 1, "and the index is what the control reported");

  assert_eq!(answered_option(&log, 2), "morning");
  let values = submitted_values(&log, 2);
  assert_eq!(keys_of(&values), vec!["morning", "stretched"]);
  assert_eq!(
    values["morning"],
    Value::String("fine".to_owned()),
    "R-57: a `choice` leaves the host as the **alternative's id** — not the label the \
     person read and not the index the widget reported: {values:?}"
  );
  assert_eq!(
    values["stretched"],
    Value::Bool(false),
    "and the option answered is the one whose control was pressed, although its id is \
     also a field id"
  );
}

/// Which control each field of one option drew, in the order the query's walk
/// reaches them — the **screen's** own account of declared order, rather than
/// the row model's slot numbering, which is the host's.
///
/// The option's own control is filtered out by id: it answers to the option's
/// description, which no field here shares. The fixture that *does* share one
/// is [`A_CHOICE_FIELD`], and it is a different case.
fn form_on_screen(window: &PromptWindow, option: &str) -> Vec<(String, AccessibleRole)> {
  within_option(window, option)
    .find_all()
    .into_iter()
    .filter_map(|element| {
      let described = element.accessible_description()?.to_string();
      Some((described, element.accessible_role()?))
    })
    .filter(|(described, _)| described != option)
    .collect()
}

/// Slice 009 `plan.md` PHASE-09/**VT-3** — **AC-1.** All five kinds draw, in
/// declared order, and a `number` outside `slider_bounds` draws the text
/// control.
///
/// **Element queries only; nothing is operated.** What this asserts is that
/// six fields of five kinds each reached the screen as the control its kind
/// calls for, and that the order on screen is the backend's declared order and
/// not the order the markup's `if` chain tests kinds in — which, for this
/// fixture, is a different order.
///
/// `rated` and `counted` are both `number` and draw different controls: the
/// host decides on the declared range and on nothing else
/// (`view_model::slider_bounds`, §7 D17), so a list of six roles is what says
/// *one kind, two controls* without operating either.
#[tokio::test]
async fn all_five_kinds_draw_in_declared_order_and_a_numbers_control_is_the_hosts_choice() {
  let (form, _log) = driving!(
    rigged("fields-every-kind-vt3", &[EVERY_KIND]),
    |window, tray, log| { form_on_screen(&window, "morning") }
  );

  assert_eq!(
    form,
    vec![
      ("noted".to_owned(), AccessibleRole::TextInput),
      ("mood".to_owned(), AccessibleRole::Combobox),
      ("rated".to_owned(), AccessibleRole::Slider),
      ("when".to_owned(), AccessibleRole::Button),
      ("stretched".to_owned(), AccessibleRole::Checkbox),
      ("counted".to_owned(), AccessibleRole::TextInput),
    ],
    "five kinds, six fields, in the order the backend declared them — and \
     `rated` and `counted` are the same kind drawn two ways"
  );
}

/// Slice 009 `plan.md` PHASE-09/**VT-4** — **AC-2, untouched.** The option's
/// own control is pressed and nothing else is; the five as-drawn values leave
/// the host with the JSON types `R-57` names for their kinds.
///
/// Read off the **child process's own request log**, so what is asserted is
/// what a backend received rather than what the host believes it sent. Each
/// value is a different JSON type from at least one of its neighbours —
/// string, string, number, string, boolean, number — so a response that typed
/// them all alike has no way to pass.
///
/// `R-58` forbids omitting a value for a drawn field, so six keys arrive for
/// six fields, and each carries what the widget was drawn showing: `false`,
/// `""`, the declared minimum or `0`, the first alternative's **id**, and —
/// for `datetime` alone — a value the screen does **not** show. That last
/// divergence is D-6, and this is the only case that asserts the epoch's exact
/// spelling (`canon-delta.md` CD-1).
#[tokio::test]
async fn every_untouched_kind_leaves_the_host_with_the_json_type_r57_names() {
  let (shown, log) = driving!(
    rigged("fields-every-kind-vt4", &[EVERY_KIND]),
    |window, tray, log| {
      let shown = shown_on_screen(&window, "morning", "when");
      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      shown
    }
  );

  assert_eq!(
    shown, NOT_SET,
    "the screen says nobody picked, while the wire below carries a value"
  );

  let values = submitted_values(&log, 2);
  assert_eq!(
    keys_of(&values),
    vec!["counted", "mood", "noted", "rated", "stretched", "when"],
    "R-58: a value for every drawn field: {values:?}"
  );
  assert_eq!(values["stretched"], Value::Bool(false));
  assert_eq!(values["noted"], Value::String(String::new()));
  assert_eq!(
    values["rated"],
    Value::from(0.0),
    "a number carries its declared minimum"
  );
  assert_eq!(
    values["counted"],
    Value::from(0.0),
    "and one with no declared bound carries zero"
  );
  assert_eq!(
    values["mood"],
    Value::String("badly".to_owned()),
    "a choice carries its **first alternative's id**, which is what the box was \
     drawn showing the label of"
  );
  assert_eq!(
    values["when"],
    Value::String("1970-01-01T00:00:00+00:00".to_owned()),
    "and a datetime nobody picked carries the epoch, offset and all — the one \
     kind whose screen and wire disagree on purpose (D1, D2)"
  );
}

/// Slice 009 `plan.md` PHASE-09/**VT-5** — **AC-2, operated.** Every control
/// driven by its own driver first, and then the same log read again: the
/// per-kind typing holds for values a person produced, not only for the ones
/// the host drew.
///
/// **Six drivers, one per control, and none of them is a shortcut.** A
/// `CheckBox`'s default action, a `LineEdit`'s `set_accessible_value` — the
/// paste path, which validates nothing — a `Slider`'s `set_accessible_value`,
/// a `ComboBox` clicked open and arrowed, and both pickers through their own
/// buttons. The three continuous controls are debounced and are driven
/// **last**, so what reaches the wire for them is the answer's own flush
/// rather than a timer this tier cannot run (`design.md` §5.2).
///
/// `counted` is typed a value no bound could have produced — negative, and not
/// a whole number — so a clamp or a round of any kind would change it.
#[tokio::test]
async fn every_operated_kind_leaves_the_host_with_the_json_type_r57_names() {
  let (today, _) = instant_today();
  let picked = composed(today.year, today.month, 15, 3);

  let ((), log) = driving!(
    rigged("fields-every-kind-vt5", &[EVERY_KIND]),
    |window, tray, log| {
      // Two choices, one row each, with the draft waited for in between —
      // which is also what says the second is made from where the first left
      // the box rather than from where it started.
      choose(&window, "morning", "mood", "Fine");
      until(LIVENESS_BOUND, || indexed(&window, "morning", "mood") != 0).await;
      choose(&window, "morning", "mood", "Well");
      until(LIVENESS_BOUND, || indexed(&window, "morning", "mood") != 1).await;

      tick!(window, "morning", "stretched");

      open_picker(&window, "morning", "when");
      pick_day(&window, 15);
      accept(&window);
      pick_hour(&window, 3);
      accept(&window);
      until(LIVENESS_BOUND, || {
        shown_on_screen(&window, "morning", "when") != NOT_SET
      })
      .await;

      // The three continuous controls, last and undelivered: the timer cannot
      // fire at this tier, so these reach the wire inside the `Choose` below
      // or not at all.
      type_into(&window, "morning", "noted", "hello");
      type_into(&window, "morning", "counted", "-4.5");
      slide_to(&window, "morning", "rated", 7.0);

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
    }
  );

  let values = submitted_values(&log, 2);
  assert_eq!(
    keys_of(&values),
    vec!["counted", "mood", "noted", "rated", "stretched", "when"],
    "{values:?}"
  );
  assert_eq!(values["stretched"], Value::Bool(true));
  assert_eq!(values["noted"], Value::String("hello".to_owned()));
  assert_eq!(
    values["rated"],
    Value::from(7.0),
    "R-57: a slider's own value leaves as a JSON number"
  );
  assert_eq!(
    values["counted"],
    Value::from(-4.5),
    "as does a typed one, unclamped and unrounded"
  );
  assert_eq!(
    values["mood"],
    Value::String("well".to_owned()),
    "R-57: the alternative's id — the third one, so neither the first nor an \
     off-by-one reaches this: {values:?}"
  );
  assert_eq!(
    values["when"],
    Value::String(picked),
    "and the pick, composed host-side and carrying the offset it resolved in"
  );
}

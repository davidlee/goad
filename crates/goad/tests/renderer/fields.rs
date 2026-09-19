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
use i_slint_backend_testing::ElementHandle;
use serde_json::Value;
use slint::{ComponentHandle, Model};
use tokio::sync::mpsc;
use tokio::task::LocalSet;

use crate::driving::{host, instant};
use crate::harness::{
  TIMEOUT, element_described, field_described, glass_over, logging_scripted, now, stub_clock,
  until, value_of, window_and_tray,
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

/// One option carrying one `boolean` and one **`datetime`** field, which this
/// renderer does not draw. R-55 says the view is still shown and the option is
/// still answerable; R-58 says the response is silent about the undrawn field
/// rather than carrying a default for it.
///
/// The undrawn kind moves one phase at a time, because a fixture's undrawn
/// field has to name a kind that is *still* undrawn: `text` until PHASE-05
/// drew it, `datetime` until PHASE-07 draws it, `number` until PHASE-08, and
/// then there is nowhere left to move it and PHASE-09 deletes the case rather
/// than repairing it (`prototype-notes.md` P-13).
const A_DRAWN_AND_AN_UNDRAWN_FIELD: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"read","kind":"boolean","label":"Read"},{"id":"noted","kind":"datetime","label":"Anything to add?"}]}]},"next_check":"45 minutes"}"#;

/// One option carrying a `boolean` and **two `text` fields**. Two, because
/// AC-4's element half is about a person typing into one field and then
/// another inside one debounce window: a single field cannot tell a map keyed
/// by (option, field) from one holding a single edit.
const TWO_TEXT_FIELDS: &str = r#"{"view":{"kind":"choice","title":"Proceed?","options":[{"id":"morning","label":"Morning","fields":[{"id":"stretched","kind":"boolean","label":"Stretched"},{"id":"noted","kind":"text","label":"Anything to add?"},{"id":"also","kind":"text","label":"And then?"}]}]},"next_check":"45 minutes"}"#;

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
/// only once a present has written the slot from it, and the command channel
/// holds one (`main.rs:86`): a second click sent before the loop has drained
/// the first is dropped by `Wire::send` and the tick silently undoes itself.
/// Waiting on this is what keeps that from happening, and a drop then fails as
/// a timeout rather than as a wrong value.
///
/// A field the view does not declare reads as `false` rather than panicking,
/// because every caller is a poll predicate: the one thing a synchronisation
/// point may not do is fail before the condition it is waiting for is true.
fn drafted(window: &PromptWindow, option: &str, field: &str) -> bool {
  value_of(window, option, field).is_some_and(|value| value.checked)
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

// ---------------------------------------------------------------------------
// Driving the production loop
// ---------------------------------------------------------------------------

/// Everything one case drives, assembled exactly as `main.rs` assembles it:
/// a headless window and tray, a glass over them, the room the query needs, a
/// host over a real child process, and `install`'s callback table around a
/// **capacity-1** channel (`main.rs:86-90`).
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
  let glass = glass_over(&window, &tray);
  let (command, log) = logging_scripted(case, instructions);
  let backend = host(command, TIMEOUT, now());
  let (commands_in, commands) = mpsc::channel::<Command>(1);
  let cancel = Cancel::new();
  let notice = Notice::new();
  // The one `Wire`, cloned into each callback and nowhere else — and the
  // sender lives only inside it, so nothing in this file can put a `Command`
  // on the channel except by activating an element a person would.
  //
  // The debounce likewise: created here and reachable only through the
  // callbacks, so a case cannot hold an edit or flush one except by driving a
  // control. PHASE-06 gives `glass_over` a clone of this same handle, which is
  // when the `Rig` has to retain it; nothing here needs it yet.
  install(
    &window,
    &tray,
    &Wire::new(commands_in, cancel.clone(), notice.clone()),
    &Rc::new(Debounce::new()),
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
/// whether the edit reached the loop, and the command channel holds one
/// (`main.rs:86`): a second click sent before the first is drained is dropped
/// by `Wire::send`, and the next present writes the tick back off the screen.
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

/// VT-4 — **AC-3's remaining half.** An option carrying one `boolean` and one
/// `text` field, which this renderer does not draw: the view is still shown,
/// the option still answers, the `respond` carries the boolean key and **no
/// key for the text field** (R-55, R-58), and the undrawn report is on the
/// diagnostic surface.
///
/// The report is read off the window's own `diagnostic-lines` — the property
/// the diagnostics pane walks and the glass writes on every present — rather
/// than off `Controller`'s copy of it, so the assertion is about what reached
/// a person. Its **wording** is not asserted, here or anywhere: every
/// user-visible string in this renderer is held by review (`design.md` §9).
///
/// The two halves catch different things and both are measured: with the
/// `text` kind drawn rather than reported undrawn, the report is gone **and**
/// `noted` appears on the wire, and each half alone is enough to see it.
#[tokio::test]
async fn a_view_carrying_an_undrawn_field_is_still_shown_and_still_answers_its_drawn_keys() {
  let ((shown, lines), log) = driving!(
    rigged("fields-vt4", &[A_DRAWN_AND_AN_UNDRAWN_FIELD]),
    |window, tray, log| {
      let shown = ComponentHandle::window(&window).is_visible();
      let lines = diagnostic_lines(&window);

      tick!(window, "morning", "read");

      option_control(&window, "morning").invoke_accessible_default_action();
      until(LIVENESS_BOUND, || invocations(&log) >= 2).await;
      (shown, lines)
    }
  );

  assert!(
    shown,
    "R-55: a view carrying an undrawn field is still shown"
  );
  assert!(
    lines
      .iter()
      .any(|line| line.contains("option morning field noted")),
    "the undrawn field is reported where a person reads it: {lines:?}"
  );

  assert_eq!(answered_option(&log, 2), "morning");
  let values = submitted_values(&log, 2);
  assert_eq!(
    keys_of(&values),
    vec!["read"],
    "the drawn field's key and nothing else — R-58 is silent about `noted` rather \
     than carrying a default for it: {values:?}"
  );
  assert_eq!(values["read"], Value::Bool(true));
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
/// typed reaches the draft, and the `inits` counter is unchanged across the
/// typing — the element was **not destroyed while it was being typed into**,
/// which is the whole of what AC-4 is about and the thing no value assertion
/// can see (`docs/memory/a-present-destroys-the-widget-it-writes.md`).
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

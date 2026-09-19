//! design.md §9 items 6-10: the renderer's cheap tier, headless, no display
//! server. Each `#[test]` constructs its own `PromptWindow` and calls
//! `init_no_event_loop()` itself, never behind a shared guard — the
//! platform is thread-local and cargo runs test functions on separate
//! threads (`research.md` Thread 3, T-B).
//!
//! **No case here shows its window, and that is what makes an exhaustive
//! query sound.** `ElementQuery` skips any element clipped out of view
//! (`i-slint-backend-testing-1.17.1/search_api.rs:373-375` →
//! `i-slint-core-1.17.1/item_tree.rs:399-408`, a geometric test against the
//! nearest clipping ancestor). A window acquires its size, and the options
//! `ScrollView` its viewport, only when it is shown — which `Glass::present`
//! does and this file does not. A test that presents through the glass is
//! reading the screen through that viewport and can see fewer controls than
//! the model holds; one here sees them all.

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use goad::generated::{
  FieldBlock, FieldEdit, FieldRow, FieldValue, Kind, OptionRow, PromptWindow, WindowMode,
};
use i_slint_backend_testing::{AccessibleRole, ElementHandle, ElementQuery, init_no_event_loop};
use slint::{ModelRc, SharedString, VecModel};

use crate::harness::{described, element_described, field_described, within_option};

type TestResult = Result<(), Box<dyn Error>>;

/// What `edited` carries: the three selectors — view token, option id, field
/// id — and the typed report of what the widget did.
type EditedArgs = (String, String, String, FieldEdit);

const OPTIONS: [(&str, &str, &str); 3] = [
  ("opt-a", "Yes", "view-1"),
  ("opt-b", "No", "view-1"),
  ("opt-c", "Ask later", "view-1"),
];

fn window() -> Result<PromptWindow, Box<dyn Error>> {
  init_no_event_loop();
  Ok(PromptWindow::new()?)
}

fn rows(entries: &[(&str, &str, &str)]) -> ModelRc<OptionRow> {
  let rows: Vec<OptionRow> = entries
    .iter()
    .map(|(id, label, view)| OptionRow {
      id: SharedString::from(*id),
      label: SharedString::from(*label),
      view: SharedString::from(*view),
      blocks: ModelRc::default(),
    })
    .collect();
  ModelRc::new(VecModel::from(rows))
}

/// The option the field cases hang their blocks on. One is enough: a field
/// id is unique only within its option (R-52), and what that costs the
/// query is `field_described`'s subject, not this fixture's.
const AN_OPTION: (&str, &str, &str) = ("opt-a", "Yes", "view-1");

/// One **structure**-channel row. Its value is not here: `slot` is the field's
/// index into the window's `values`, which a case writes alongside
/// (design.md §5.2, §5.5 I-B).
///
/// The two are kept in step by hand in this file, where `glass.rs` keeps them
/// in step by construction — these cases build the models directly, which is
/// what lets them ask the markup a question the glass is not part of.
fn field(slot: i32, id: &str, label: &str) -> FieldRow {
  FieldRow {
    id: SharedString::from(id),
    label: SharedString::from(label),
    kind: Kind::Boolean,
    slot,
    // `slider` and its three arithmetic slots are `number`'s alone and are
    // read by nothing here (PHASE-08).
    ..FieldRow::default()
  }
}

/// The **value** channel for a run of boolean fields, in slot order: slot *n*
/// is `checked[n]`.
fn values(checked: &[bool]) -> ModelRc<FieldValue> {
  ModelRc::new(VecModel::from(
    checked
      .iter()
      .map(|checked| FieldValue {
        checked: *checked,
        ..FieldValue::default()
      })
      .collect::<Vec<FieldValue>>(),
  ))
}

fn block(heading: &str, fields: Vec<FieldRow>) -> FieldBlock {
  FieldBlock {
    heading: SharedString::from(heading),
    fields: ModelRc::new(VecModel::from(fields)),
  }
}

/// `AN_OPTION` carrying the blocks given. A `Vec` rather than a `ModelRc`,
/// because a caller that must hold the `Rc<VecModel<_>>` itself and reset it
/// cannot be handed one already wrapped (VT-2).
fn one_option_with(blocks: Vec<FieldBlock>) -> Vec<OptionRow> {
  vec![OptionRow {
    id: SharedString::from(AN_OPTION.0),
    label: SharedString::from(AN_OPTION.1),
    view: SharedString::from(AN_OPTION.2),
    blocks: ModelRc::new(VecModel::from(blocks)),
  }]
}

/// VT-1 — item 6 (AC-10, R5). Every other assertion in this file rests on
/// the element query API being live; without `with_debug_info` it returns
/// empty and every one of them passes vacuously (E-1, A-3).
#[test]
fn a_known_element_is_found() -> TestResult {
  let window = window()?;
  let options_list = ElementHandle::find_by_accessible_label(&window, "options").next();
  assert!(
    options_list.is_some(),
    "the options list container must be found; its absence means debug info \
     is missing, not that the window has no options (S-3)"
  );
  Ok(())
}

/// VT-2 — item 7 (AC-4, E-2). Heading, body and one control per option, in
/// the order given, counted through `accessible-item-count`: the count the
/// list declares and a reader announces, rather than a tally of whatever
/// elements a query happened to match.
///
/// **That is a preference here, not a prohibition.** The warning this
/// comment used to carry — never `find_all().len()`, because the list
/// virtualises — is true of a `ListView` and of nothing else in this
/// markup: a plain repeater creates every instance
/// (`i-slint-core-1.17.1/model/repeater.rs:143-154`, and `:156-158` scopes
/// the lazy algorithm to a `ListView` viewport), and `app.slint`'s options
/// are a `ScrollView` around a `VerticalLayout` with a plain `for`. What
/// does bound an exhaustive query is clipping, and the module comment above
/// says why it does not reach this file.
#[test]
fn heading_body_and_one_control_per_option_render_in_order() -> TestResult {
  let window = window()?;
  window.set_heading("Proceed?".into());
  window.set_options(rows(&OPTIONS));

  let heading_found = ElementHandle::find_by_accessible_label(&window, "Proceed?").next();
  assert!(
    heading_found.is_some(),
    "the heading Text must be found by its label"
  );

  let body_found = ElementQuery::from_root(&window)
    .match_inherits("StyledText")
    .find_first();
  assert!(
    body_found.is_some(),
    "a StyledText is present in prompt mode; its content is asserted on \
     Presentation in the mapper tier, not here (design.md §5.2)"
  );

  let options_list = ElementHandle::find_by_accessible_label(&window, "options")
    .next()
    .ok_or("the options list container must be found")?;
  assert_eq!(options_list.accessible_item_count(), Some(OPTIONS.len()));

  for (index, (id, label, _view)) in OPTIONS.iter().enumerate() {
    let button = element_described(&window, id)
      .ok_or_else(|| format!("no control with accessible-description {id:?}"))?;
    assert_eq!(button.accessible_item_index(), Some(index));
    assert_eq!(button.accessible_label().as_deref(), Some(*label));
  }
  Ok(())
}

/// VT-3 — item 8 (AC-5, R-14, D10). Activating a control fires `chosen`
/// with the right `OptionId` **and** the right view token — including when
/// two options share a label, which is why selection is by
/// `accessible_description` and never by label.
#[test]
fn activating_a_control_fires_chosen_with_the_right_id_and_view_token() -> TestResult {
  let window = window()?;
  window.set_options(rows(&[
    ("opt-a", "Yes", "view-1"),
    ("opt-b", "Yes", "view-1"), // shares opt-a's label, deliberately (R-14)
  ]));

  let captured: Rc<RefCell<Option<(String, String)>>> = Rc::new(RefCell::new(None));
  {
    let captured = Rc::clone(&captured);
    window.on_chosen(move |view, id| {
      *captured.borrow_mut() = Some((view.to_string(), id.to_string()));
    });
  }

  let second = element_described(&window, "opt-b").ok_or("no control described opt-b")?;
  second.invoke_accessible_default_action();

  assert_eq!(
    *captured.borrow(),
    Some(("view-1".to_string(), "opt-b".to_string()))
  );
  Ok(())
}

/// VT-4 — item 9 (AC-6, AC-11, E-1). The diagnostic empty state, asserted
/// by presence when there is nothing to report and by absence once there
/// is — the pairing E-1 requires of every absence-shaped assertion.
///
/// VT-5 — item 10 (AC-11, R3): the absence half is not vacuous. Broken by
/// changing `ui/app.slint`'s guard from
/// `if root.diagnostic-lines.length == 0` to `if true`, so the placeholder
/// never leaves; rerun with `cargo test -p goad --test renderer
/// the_diagnostic_empty_state_is_present_only_when_empty`, reverted after.
/// Output pasted in `notes.md`.
#[test]
fn the_diagnostic_empty_state_is_present_only_when_empty() -> TestResult {
  let window = window()?;
  window.set_mode(WindowMode::Diagnostic);
  window.set_diagnostic_lines(ModelRc::new(VecModel::from(Vec::<SharedString>::new())));
  assert!(
    ElementHandle::find_by_accessible_label(&window, "Nothing to report.")
      .next()
      .is_some(),
    "the empty-state placeholder must be present when there are no lines"
  );

  window.set_diagnostic_lines(ModelRc::new(VecModel::from(vec![SharedString::from(
    "a diagnostic line",
  )])));
  assert!(
    ElementHandle::find_by_accessible_label(&window, "Nothing to report.")
      .next()
      .is_none(),
    "the placeholder must not survive once a line exists"
  );
  Ok(())
}

/// F-1 (review-code 002, round 1). AC-9's degradation half: the "shown as
/// plain text" marker is present exactly when `body-degraded` is set, and
/// absent otherwise — the same presence/absence pairing E-1 requires,
/// applied to `app.slint:37` rather than to the diagnostic empty state.
///
/// Broken by deleting `if root.body-degraded: Text { text: "shown as plain
/// text"; }` from `ui/app.slint`: this test fails (the presence half, once
/// `body_degraded` is true, finds nothing). Reverted after; output pasted
/// in `notes.md`.
#[test]
fn the_degradation_marker_is_present_only_when_the_body_is_degraded() -> TestResult {
  let window = window()?;
  window.set_body_degraded(false);
  assert!(
    ElementHandle::find_by_accessible_label(&window, "shown as plain text")
      .next()
      .is_none(),
    "the marker must not appear when the body is not degraded"
  );

  window.set_body_degraded(true);
  assert!(
    ElementHandle::find_by_accessible_label(&window, "shown as plain text")
      .next()
      .is_some(),
    "the marker must appear once the body is degraded"
  );
  Ok(())
}

/// The option's field controls in tree order, depth-first pre-order —
/// `find_all` returns matches in walk order (`search_api.rs:303-312`).
fn fields_in_option(window: &PromptWindow, option: &str) -> Vec<SharedString> {
  within_option(window, option)
    .match_inherits("CheckBox")
    .find_all()
    .into_iter()
    .filter_map(|element| element.accessible_description())
    .collect()
}

/// Every label the option's **field container** puts on screen, in tree order —
/// block headings and checkbox labels interleaved, which is the sequence a
/// person actually reads.
///
/// `Text` rather than `CheckBox` because a heading is the one thing the field
/// markup draws that is not a control, and `accessible_description` cannot
/// reach it: a heading has no id to carry there, only its words. A checkbox
/// contributes its own internal `Text`, so this query returns **both** kinds
/// and that is the point — *which heading covers which fields* (AC-2) is a
/// claim about their order relative to each other, and a query returning only
/// headings could not make it.
///
/// Scoped by **role**, not by `within_option`, and that is what makes it
/// honest. Both the option's `Button` and its field container answer to
/// `option.id`, so `within_option` descends into the control as well — and the
/// `Button`'s own internal `Text` reports no accessible label while a
/// `CheckBox`'s reports one. Filtering the unlabelled away would work, and
/// would rest the case on an asymmetry inside `std-widgets` that nothing here
/// pins: a Slint release that labels `Button`'s inner `Text` too would fail
/// this file's heading case with a heading-shaped message for a reason that
/// has nothing to do with headings (`review-code.md` F-6). Scoping to the
/// container excludes the control instead, so **every** element in scope was
/// drawn by the field markup and none is dropped.
///
/// `Some("")` is therefore kept and is load-bearing: a `Text` bound to `""`
/// reports an empty label rather than none, so an untitled block that wrongly
/// drew a heading appears here as an empty string instead of vanishing.
/// Measured, not assumed.
fn labels_in_option(window: &PromptWindow, option: &str) -> Vec<SharedString> {
  ElementQuery::from_root(window)
    .match_predicate(described(option))
    .match_accessible_role(AccessibleRole::Groupbox)
    .match_descendants()
    .match_inherits("Text")
    .find_all()
    .into_iter()
    .map(|element| {
      element
        .accessible_label()
        .expect("every element the field markup draws under the container carries a label")
    })
    .collect()
}

/// VT-1 — A-1's pin. A block of two fields renders two controls, each found
/// by the option-scoped query. Its force is as much in **compiling** as in
/// passing: `blocks` generates as `ModelRc<FieldBlock>` from an array-typed
/// struct member, and a future Slint that types one differently fails here
/// rather than three phases later (design.md §5.5 A-1).
#[test]
fn a_block_of_two_fields_renders_a_control_for_each() -> TestResult {
  let window = window()?;
  // `values` before `options`, the order `present` writes them in and for the
  // same reason: a row evaluates `root.values[field.slot]` while it is being
  // instantiated (design.md §5.5 I-F).
  window.set_values(values(&[false, true]));
  window.set_options(ModelRc::new(VecModel::from(one_option_with(vec![block(
    "Before you go",
    vec![field(0, "stretched", "Stretched"), field(1, "read", "Read")],
  )]))));

  for (id, checked) in [("stretched", false), ("read", true)] {
    let control = field_described(&window, AN_OPTION.0, id)
      .ok_or_else(|| format!("no control described {id:?} under {:?}", AN_OPTION.0))?;
    assert_eq!(
      control.accessible_checked(),
      Some(checked),
      "the control must read the model's value, not a default"
    );
  }
  Ok(())
}

/// VT-2 — A-2's pin, and what the **structure** channel does when it is
/// written. A click assigns `checked` imperatively, which removes the binding
/// the markup declared; resetting the row model **destroys and rebuilds**
/// every element under the repeater
/// (`i-slint-core-1.17.1/model/repeater.rs:506-509`, `:361-381`), so a fresh
/// binding is established and the value channel is the authority again.
///
/// **What changed under it, and what did not.** `present` now writes the row
/// model only where the `view_id` it is showing has changed (slice 009
/// PHASE-01, `design.md` §7 D8), so the rebuild this case pins is no longer
/// what corrects a widget on an ordinary present — a guarded write triggered
/// by the epoch is, and that is the loop tier's
/// (`tests/event_loop_reassert/`). The rebuild is still exactly what a
/// **replacement view** gets, where there is no interaction state worth
/// preserving, and this is still the only case that pins it. The reason it is
/// never `set_row_data` is unchanged: that path updates the surviving element
/// and silently leaves it detached from the model (design.md §5.4).
///
/// The field starts **checked**, so every assertion here reads a value the
/// widget's own default cannot supply. Written the other way round it passes
/// with no `checked:` binding in the markup at all: a rebuilt `CheckBox`
/// defaults to unchecked, which is the same answer the model would have
/// given (`docs/memory/a-green-test-can-assert-a-proxy.md`).
///
/// The `Rc<VecModel<_>>` must be held and reset, exactly as
/// `SlintGlass::present` does (`glass.rs:159-162`). A test handing a fresh
/// `ModelRc` to `set_options` each time exercises nothing.
#[test]
fn a_model_reset_re_establishes_a_fields_checked_value() -> TestResult {
  let checked_field = || one_option_with(vec![block("", vec![field(0, "stretched", "Stretched")])]);
  let window = window()?;
  window.set_values(values(&[true]));
  let options = Rc::new(VecModel::from(checked_field()));
  window.set_options(ModelRc::from(Rc::clone(&options)));

  let checked_now =
    || field_described(&window, AN_OPTION.0, "stretched").and_then(|c| c.accessible_checked());
  assert_eq!(
    checked_now(),
    Some(true),
    "the control reads the model's value, which is not its own default"
  );

  field_described(&window, AN_OPTION.0, "stretched")
    .ok_or("no control")?
    .invoke_accessible_default_action();
  assert_eq!(
    checked_now(),
    Some(false),
    "the widget flips itself on click, so the feedback is immediate"
  );

  options.set_vec(checked_field());
  window.set_options(ModelRc::from(Rc::clone(&options)));

  assert_eq!(
    checked_now(),
    Some(true),
    "and the present that follows writes the model's value back over it"
  );
  Ok(())
}

/// VT-3 — activating a field control fires `edited` with all four arguments:
/// the view token, the option id, the field id and the `FieldEdit` the
/// control filled. The three selectors are what makes the command
/// addressable — a field id is unique only within its option (R-52), so
/// option and view are not decoration.
///
/// **The report is compared whole, not slot by slot**, which is how this case
/// also holds the markup literal to naming only the fields it means
/// (`plan.md` PHASE-03/EX-2): a `CheckBox` that wrote `text`, `number` or
/// `index` on its way past would fail here. And `kind` is the discriminant
/// `view_model::interpret` refuses a mismatch on, so a control reporting
/// someone else's kind is caught at the boundary it is raised from rather
/// than only in the controller.
#[test]
fn activating_a_field_control_fires_edited_with_all_four_selectors() -> TestResult {
  let window = window()?;
  window.set_values(values(&[false]));
  window.set_options(ModelRc::new(VecModel::from(one_option_with(vec![block(
    "",
    vec![field(0, "stretched", "Stretched")],
  )]))));

  let captured: Rc<RefCell<Option<EditedArgs>>> = Rc::new(RefCell::new(None));
  {
    let captured = Rc::clone(&captured);
    window.on_edited(move |view, option, field, edit| {
      *captured.borrow_mut() = Some((
        view.to_string(),
        option.to_string(),
        field.to_string(),
        edit,
      ));
    });
  }

  field_described(&window, AN_OPTION.0, "stretched")
    .ok_or("no control")?
    .invoke_accessible_default_action();

  assert_eq!(
    *captured.borrow(),
    Some((
      AN_OPTION.2.to_string(),
      AN_OPTION.0.to_string(),
      "stretched".to_string(),
      FieldEdit {
        kind: Kind::Boolean,
        checked: true,
        ..FieldEdit::default()
      }
    ))
  );
  Ok(())
}

/// VT-4 — AC-2's on-screen half. Two blocks over five fields render in
/// declared order, read off the tree rather than off a number the markup
/// supplied: `accessible-item-index` would assert what we wrote, not what
/// the walk found.
///
/// `find_all` is sound here despite the standing warning on
/// `heading_body_and_one_control_per_option_render_in_order`: virtualisation
/// is the `ListView` path only. A plain repeater creates every instance
/// (`i-slint-core-1.17.1/model/repeater.rs:143-154`, and `:156-158` scopes
/// the lazy algorithm to a `ListView` viewport), and this markup is a
/// `ScrollView` around a `VerticalLayout` with a plain `for`.
#[test]
fn a_fields_screen_order_is_its_declared_order_across_blocks() -> TestResult {
  let window = window()?;
  window.set_values(values(&[false; 5]));
  window.set_options(ModelRc::new(VecModel::from(one_option_with(vec![
    block(
      "Before you go",
      vec![field(0, "stretched", "Stretched"), field(1, "read", "Read")],
    ),
    block(
      "",
      vec![
        field(2, "walked", "Walked"),
        field(3, "called", "Called"),
        field(4, "slept", "Slept"),
      ],
    ),
  ]))));

  assert_eq!(
    fields_in_option(&window, AN_OPTION.0),
    vec!["stretched", "read", "walked", "called", "slept"],
    "screen order is declared order, across the block boundary and through \
     an untitled block"
  );
  Ok(())
}

/// AC-2's third link, for the **heading** — the half the chain was missing
/// (`review-code.md` F-1). `fields_in_option` collects checkbox descriptions
/// and reaches no heading at all, so before this case the whole of
/// `app.slint`'s heading markup could be deleted with the gate staying green,
/// while the checkbox half of the same element was held at the screen by six
/// cases.
///
/// Both of the markup's branches are asserted, because a positive assertion
/// alone would leave the second exactly as unheld as before:
///
/// - a **named** block puts its heading on screen, above its own fields and
///   below nothing — which is *which heading covers which fields*;
/// - an **untitled** block puts nothing there, which is the `if block.heading
///   != ""` guard. `heading: ""` is an untitled block and not a missing one
///   (`design.md` §5.5, and the markup's own comment), so the guard is a
///   claim about meaning and not a micro-optimisation.
///
/// The fixture is the declared-order case's, deliberately: one named block and
/// one untitled, which is the smallest shape in which both branches are live
/// at once.
#[test]
fn a_block_heading_reaches_the_screen_and_an_untitled_block_draws_none() -> TestResult {
  let window = window()?;
  window.set_values(values(&[false; 3]));
  window.set_options(ModelRc::new(VecModel::from(one_option_with(vec![
    block(
      "Before you go",
      vec![field(0, "stretched", "Stretched"), field(1, "read", "Read")],
    ),
    block("", vec![field(2, "walked", "Walked")]),
  ]))));

  assert_eq!(
    labels_in_option(&window, AN_OPTION.0),
    vec!["Before you go", "Stretched", "Read", "Walked"],
    "the heading is drawn once, above the two fields it covers and not above \
     the third; and the untitled block draws no heading at all, which an \
     unguarded Text would show here as an empty string between \"Read\" and \
     \"Walked\""
  );
  Ok(())
}

/// VT-5 — AC-6. An option with no fields adds **no element**: absent, not
/// empty. Paired with a sibling option that does have one, in the same
/// window, so the absence half is not vacuous and the guard is seen to be
/// per option rather than per view (E-1).
#[test]
fn an_option_with_no_fields_adds_no_element() -> TestResult {
  let window = window()?;
  window.set_values(values(&[false]));
  let mut rows = one_option_with(vec![block("", vec![field(0, "stretched", "Stretched")])]);
  rows.push(OptionRow {
    id: SharedString::from("opt-b"),
    label: SharedString::from("No"),
    view: SharedString::from(AN_OPTION.2),
    blocks: ModelRc::default(),
  });
  window.set_options(ModelRc::new(VecModel::from(rows)));

  assert!(
    element_described(&window, "opt-b").is_some(),
    "the option's own control is still drawn"
  );
  assert!(
    fields_in_option(&window, "opt-b").is_empty(),
    "and carries no field control"
  );
  assert_eq!(
    ElementQuery::from_root(&window)
      .match_accessible_role(AccessibleRole::Groupbox)
      .find_all()
      .len(),
    1,
    "one container, for the one option that has fields: the guard is per \
     option, and an empty `blocks` produces no container at all"
  );
  Ok(())
}

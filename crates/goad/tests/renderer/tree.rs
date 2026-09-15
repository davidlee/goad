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

use goad::generated::{FieldBlock, FieldRow, OptionRow, PromptWindow, WindowMode};
use i_slint_backend_testing::{AccessibleRole, ElementHandle, ElementQuery, init_no_event_loop};
use slint::{ModelRc, SharedString, VecModel};

type TestResult = Result<(), Box<dyn Error>>;

/// The four selectors `edited` carries: view token, option id, field id and
/// the new `checked`.
type EditedArgs = (String, String, String, bool);

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

fn field(id: &str, label: &str, checked: bool) -> FieldRow {
  FieldRow {
    option: SharedString::from(AN_OPTION.0),
    id: SharedString::from(id),
    label: SharedString::from(label),
    checked,
  }
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

/// The option's **control**, by the identity the tests select on (D10,
/// R-14): never by label, which two options may share.
///
/// The description alone no longer picks one element. The option's field
/// container answers to the same `option.id`, so that a field can be
/// addressed by a query scoped to its option (`field_described` below), and
/// `find_first` would otherwise return whichever the walk reached first —
/// declaration order, which nothing pins. The type filter is what keeps this
/// helper's contract: a control, with a default action and an item index,
/// and not the group that surrounds it.
fn element_described(window: &PromptWindow, description: &str) -> Option<ElementHandle> {
  let description = description.to_string();
  ElementQuery::from_root(window)
    .match_inherits("Button")
    .match_predicate(move |element| {
      element.accessible_description().as_deref() == Some(description.as_str())
    })
    .find_first()
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

/// Everything under the option that answers to `option.id` — the scope both
/// field queries below start from. `ElementQuery` has no accessible-description
/// matcher (`search_api.rs:232-287` lists its six builders), so the
/// description is read through `ElementHandle::accessible_description` inside
/// a predicate, which is the shape `element_described` already uses.
///
/// The option's control answers to `option.id` too, and is reached first; it
/// has no field beneath it, so the walk continues to the container.
fn within_option(window: &PromptWindow, option: &str) -> ElementQuery {
  let option = option.to_string();
  ElementQuery::from_root(window)
    .match_predicate(move |element| {
      element.accessible_description().as_deref() == Some(option.as_str())
    })
    .match_descendants()
}

/// A field's identity is **scoped, not composite**: the option's container
/// carries `option.id` and the field's control carries `field.id`, so the
/// query descends from the one to the other. `element_described`'s unscoped
/// `find_first` does not carry over — an option id is unique within a view
/// (R-14) but a field id only within an option (R-52), so an unscoped query
/// would take whichever came first and report no ambiguity, which is exactly
/// the case AC-4 exists to prove. Joining the two into one description was
/// rejected: ids are backend-supplied strings whose characters no requirement
/// constrains, so any separator can occur inside one (design.md §5.2).
fn field_described(window: &PromptWindow, option: &str, field: &str) -> Option<ElementHandle> {
  let field = field.to_string();
  within_option(window, option)
    .match_predicate(move |element| {
      element.accessible_description().as_deref() == Some(field.as_str())
    })
    .find_first()
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

/// VT-1 — A-1's pin. A block of two fields renders two controls, each found
/// by the option-scoped query. Its force is as much in **compiling** as in
/// passing: `blocks` generates as `ModelRc<FieldBlock>` from an array-typed
/// struct member, and a future Slint that types one differently fails here
/// rather than three phases later (design.md §5.5 A-1).
#[test]
fn a_block_of_two_fields_renders_a_control_for_each() -> TestResult {
  let window = window()?;
  window.set_options(ModelRc::new(VecModel::from(one_option_with(vec![block(
    "Before you go",
    vec![
      field("stretched", "Stretched", false),
      field("read", "Read", true),
    ],
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

/// VT-2 — A-2's pin. A click assigns `checked` imperatively; the next
/// present resets the model, which **destroys and rebuilds** every element
/// under the repeater (`i-slint-core-1.17.1/model/repeater.rs:506-509`,
/// `:361-381`), so the declarative binding comes back and the model is the
/// authority again. This is what makes the draft the single source of a
/// field's value, and it is why the repair for the focus that rebuild costs
/// is never `set_row_data` — that path updates the surviving element and
/// silently leaves it detached from the model (design.md §5.4).
///
/// The field starts **checked**, so every assertion here reads a value the
/// widget's own default cannot supply. Written the other way round it passes
/// with no `checked:` binding in the markup at all: a rebuilt `CheckBox`
/// defaults to unchecked, which is the same answer the model would have
/// given (`docs/memory/a-green-test-can-assert-a-proxy.md`).
///
/// The `Rc<VecModel<_>>` must be held and reset, exactly as
/// `SlintGlass::present` does (`glass.rs:87-91`). A test handing a fresh
/// `ModelRc` to `set_options` each time exercises nothing.
#[test]
fn a_model_reset_re_establishes_a_fields_checked_value() -> TestResult {
  let checked_field =
    || one_option_with(vec![block("", vec![field("stretched", "Stretched", true)])]);
  let window = window()?;
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

/// VT-3 — activating a field control fires `edited` with all four
/// selectors: the view token, the option id, the field id and the new
/// `checked`. Three of the four are what makes the command addressable —
/// a field id is unique only within its option (R-52), so option and view
/// are not decoration.
#[test]
fn activating_a_field_control_fires_edited_with_all_four_selectors() -> TestResult {
  let window = window()?;
  window.set_options(ModelRc::new(VecModel::from(one_option_with(vec![block(
    "",
    vec![field("stretched", "Stretched", false)],
  )]))));

  let captured: Rc<RefCell<Option<EditedArgs>>> = Rc::new(RefCell::new(None));
  {
    let captured = Rc::clone(&captured);
    window.on_edited(move |view, option, field, checked| {
      *captured.borrow_mut() = Some((
        view.to_string(),
        option.to_string(),
        field.to_string(),
        checked,
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
      true
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
  window.set_options(ModelRc::new(VecModel::from(one_option_with(vec![
    block(
      "Before you go",
      vec![
        field("stretched", "Stretched", false),
        field("read", "Read", false),
      ],
    ),
    block(
      "",
      vec![
        field("walked", "Walked", false),
        field("called", "Called", false),
        field("slept", "Slept", false),
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

/// VT-5 — AC-6. An option with no fields adds **no element**: absent, not
/// empty. Paired with a sibling option that does have one, in the same
/// window, so the absence half is not vacuous and the guard is seen to be
/// per option rather than per view (E-1).
#[test]
fn an_option_with_no_fields_adds_no_element() -> TestResult {
  let window = window()?;
  let mut rows = one_option_with(vec![block(
    "",
    vec![field("stretched", "Stretched", false)],
  )]);
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

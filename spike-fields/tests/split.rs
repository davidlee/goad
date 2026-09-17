//! Slice 009, D-9: structure and value on two properties.
//!
//! `rows` is repeated over and written only when the view changes. `values`
//! is a flat property indexed by the slot each row carries, rewritten
//! wholesale on every present. Nothing repeats over `values`, so the question
//! is whether `root.values[field.slot]` compiles, tracks, and costs no
//! element when the whole property is replaced.
//!
//! One `#[test]` fn: `init_integration_test_with_system_time` is once per
//! process.

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use spike_lib::{FieldValue, Kind, SlotRow, Split};

fn values(checked: bool, text: &str) -> ModelRc<FieldValue> {
    ModelRc::new(VecModel::from(vec![
        FieldValue {
            checked,
            text: SharedString::new(),
            number: 0.0,
        },
        FieldValue {
            checked: false,
            text: text.into(),
            number: 0.0,
        },
    ]))
}

#[test]
fn a_flat_value_model_tracks_and_a_wholesale_rewrite_costs_no_element() {
    i_slint_backend_testing::init_integration_test_with_system_time();

    let ui = Split::new().expect("the spike must build");
    ui.set_rows(ModelRc::new(VecModel::from(vec![
        SlotRow {
            id: "done".into(),
            label: "done".into(),
            kind: Kind::Boolean,
            slot: 0,
        },
        SlotRow {
            id: "note".into(),
            label: "note".into(),
            kind: Kind::Text,
            slot: 1,
        },
    ])));
    ui.set_values(values(false, ""));

    let ui_weak = ui.as_weak();
    let timer = slint::Timer::default();
    let mut ticks = 0;
    // Recorded at each tick so the assertions read a history rather than a
    // final state: `inits` after the first present is the number that says
    // whether a wholesale rewrite destroyed anything.
    let observed = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(i32, i32, bool)>::new()));
    let recording = observed.clone();
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(50),
        move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            ticks += 1;
            let checkbox = i_slint_backend_testing::ElementQuery::from_root(&ui)
                .match_descendants()
                .match_inherits("CheckBox")
                .find_first()
                .expect("the fixture draws one checkbox");
            match ticks {
                // Baseline: the elements exist, nothing has been touched.
                1 => recording.borrow_mut().push((
                    ui.get_inits(),
                    ui.get_reasserts(),
                    checkbox.accessible_checked().unwrap_or(false),
                )),
                // The click self-assigns and destroys the use-site binding.
                // The host hears nothing: this is the dropped edit.
                2 => checkbox.invoke_accessible_default_action(),
                // A present in place. The draft still says `false`, and the
                // whole `values` property is replaced, not written per row.
                3 => {
                    ui.set_values(values(false, ""));
                    ui.set_epoch(ui.get_epoch() + 1);
                }
                4 => recording.borrow_mut().push((
                    ui.get_inits(),
                    ui.get_reasserts(),
                    checkbox.accessible_checked().unwrap_or(false),
                )),
                // A second present, now in agreement: the guard must write
                // nothing at all. This is the caret-safety half.
                5 => {
                    ui.set_values(values(false, ""));
                    ui.set_epoch(ui.get_epoch() + 1);
                }
                6 => recording.borrow_mut().push((
                    ui.get_inits(),
                    ui.get_reasserts(),
                    checkbox.accessible_checked().unwrap_or(false),
                )),
                _ => slint::quit_event_loop().expect("the loop must stop"),
            }
        },
    );

    ui.show().expect("the window must show");
    slint::run_event_loop_until_quit().expect("the loop must run");

    let observed = observed.borrow();
    assert_eq!(observed.len(), 3, "three samples: {observed:?}");
    let (baseline_inits, baseline_reasserts, baseline_checked) = observed[0];
    let (after_present, reasserts_after, checked_after) = observed[1];
    let (after_second, reasserts_second, checked_second) = observed[2];

    assert_eq!(baseline_reasserts, 0, "nothing diverged before the click");
    assert!(!baseline_checked, "the fixture starts unticked");
    assert!(
        ui.get_fires() > 0,
        "the re-assert handler must fire at all (got {})",
        ui.get_fires()
    );

    // The question this file exists for.
    assert_eq!(
        after_present, baseline_inits,
        "replacing the whole `values` property must destroy no element \
         (inits {baseline_inits} -> {after_present})"
    );
    assert_eq!(
        reasserts_after, 1,
        "the guard must have written the clicked widget back once"
    );
    assert!(
        !checked_after,
        "the widget must have converged to the flat model's `false`"
    );

    // The caret-safety half: in agreement, the guard writes nothing.
    assert_eq!(
        after_second, baseline_inits,
        "still no element destroyed on a second present"
    );
    assert_eq!(
        reasserts_second, 1,
        "a present that agrees must write nothing (reasserts {reasserts_second})"
    );
    assert!(!checked_second, "and must leave the widget alone");
}

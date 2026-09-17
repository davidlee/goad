//! Slice 009 §5.2: a number drawn as a `LineEdit` compares numerically.
//!
//! The failure this excludes: a person clears the field to retype, the host
//! holds `0`, and a string guard sees `"" != "0"` and writes `"0"` back —
//! undoing the clear mid-edit. A numeric guard sees `0 == 0` and is quiet.

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use spike_lib::{FieldValue, Kind, SlotRow, Split};

#[test]
fn a_numeric_guard_does_not_fight_a_cleared_field() {
    i_slint_backend_testing::init_integration_test_with_system_time();

    let ui = Split::new().expect("the spike must build");
    ui.set_rows(ModelRc::new(VecModel::from(vec![SlotRow {
        id: "count".into(),
        label: "count".into(),
        kind: Kind::Number,
        slot: 0,
    }])));
    let values = || {
        ModelRc::new(VecModel::from(vec![FieldValue {
            checked: false,
            text: SharedString::new(),
            number: 0.0,
        }]))
    };
    ui.set_values(values());

    let ui_weak = ui.as_weak();
    let timer = slint::Timer::default();
    let mut ticks = 0;
    let observed = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(i32, i32, SharedString)>::new()));
    let recording = observed.clone();
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(50),
        move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            ticks += 1;
            let edit = i_slint_backend_testing::ElementQuery::from_root(&ui)
                .match_descendants()
                .match_inherits("LineEdit")
                .find_first()
                .expect("the fixture draws one line edit");
            match ticks {
                1 => recording.borrow_mut().push((
                    ui.get_inits(),
                    ui.get_reasserts(),
                    edit.accessible_value().unwrap_or_default(),
                )),
                // The person clears the field. The host holds 0 and hears
                // nothing it would act on.
                2 => edit.set_accessible_value(""),
                3 => {
                    ui.set_values(values());
                    ui.set_epoch(ui.get_epoch() + 1);
                }
                4 => recording.borrow_mut().push((
                    ui.get_inits(),
                    ui.get_reasserts(),
                    edit.accessible_value().unwrap_or_default(),
                )),
                _ => slint::quit_event_loop().expect("the loop must stop"),
            }
        },
    );

    ui.show().expect("the window must show");
    slint::run_event_loop_until_quit().expect("the loop must run");

    let observed = observed.borrow();
    assert_eq!(observed.len(), 2, "two samples: {observed:?}");
    assert_eq!(observed[0].2, "0", "as-drawn shows the number, not an empty box");
    assert!(ui.get_fires() > 0, "the guard must have run at all");
    assert_eq!(
        observed[1].1, 0,
        "a cleared field must not be written back (reasserts {})",
        observed[1].1
    );
    assert_eq!(
        observed[1].2, "",
        "and the clear must survive the present (text {:?})",
        observed[1].2
    );
}

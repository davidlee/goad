//! The same question as `identity.rs`, under a real event loop.
//!
//! `init_integration_test_with_system_time` can only be called once per
//! process, so this is one `#[test]` fn — the shape
//! `crates/goad/tests/event_loop_schedule` already uses.

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use spike_lib::{FieldRow, Kind, Spike};
use std::rc::Rc;

/// Does a `changed` handler fire under a real loop, where it does not under
/// `init_no_event_loop`? This is the difference between the re-assert being
/// testable and being human-verified only.
#[test]
fn a_change_handler_fires_under_a_real_event_loop() {
    i_slint_backend_testing::init_integration_test_with_system_time();

    let ui = Spike::new().expect("the spike must build");
    let empty: ModelRc<SharedString> = ModelRc::new(VecModel::from(Vec::<SharedString>::new()));
    let rows = vec![FieldRow {
        id: "done".into(),
        label: "done".into(),
        kind: Kind::Boolean,
        checked: false,
        text: SharedString::new(),
        number: 0.0,
        minimum: 0.0,
        maximum: 100.0,
        alternatives: empty,
    }];
    let model = Rc::new(VecModel::from(rows.clone()));
    ui.set_fields(ModelRc::from(model.clone()));

    let ui_weak = ui.as_weak();
    let timer = slint::Timer::default();
    let model_for_timer = model.clone();
    let mut ticks = 0;
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(50),
        move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            ticks += 1;
            if ticks > 4 {
                slint::quit_event_loop().expect("the loop must stop");
                return;
            }
            // Tick 1 clicks the checkbox, which self-assigns and destroys the
            // `checked: field.checked` binding — the state A-2 must recover
            // from, and the state the host never heard about.
            if ticks == 1 {
                i_slint_backend_testing::ElementQuery::from_root(&ui)
                    .match_descendants()
                    .match_inherits("CheckBox")
                    .find_first()
                    .expect("the fixture draws one checkbox")
                    .invoke_accessible_default_action();
                return;
            }
            // A present in place, exactly as the host would do it. The draft
            // still says `false`.
            model_for_timer.set_row_data(0, rows[0].clone());
            ui.set_probe(ui.get_probe() + 1);
            ui.set_epoch(ui.get_epoch() + 1);
        },
    );

    ui.show().expect("the window must show");
    slint::run_event_loop_until_quit().expect("the loop must run");

    assert!(
        ui.get_root_changes() > 0,
        "root-level `changed` must fire under a real loop (got {})",
        ui.get_root_changes()
    );
    assert!(
        ui.get_fires() > 0,
        "a repeated element's `changed` must fire under a real loop (got {})",
        ui.get_fires()
    );
    assert_eq!(
        ui.get_inits(),
        1,
        "and the element must never have been rebuilt"
    );

    // The claim the whole approach rests on: the widget was clicked, its
    // binding is gone, the host never heard — and a present in place still
    // brought it back to what the draft says, without destroying it.
    assert!(
        ui.get_reasserts() > 0,
        "the guard must have written through (got {})",
        ui.get_reasserts()
    );
    let checkbox = i_slint_backend_testing::ElementQuery::from_root(&ui)
        .match_descendants()
        .match_inherits("CheckBox")
        .find_first()
        .expect("the fixture draws one checkbox");
    assert_eq!(
        checkbox.accessible_checked(),
        Some(false),
        "A-2 holds with the element preserved: the draft is still the authority"
    );
}

//! Slice 009 G-2 — is a per-slot revision a workable guard trigger?
//!
//! The guard would stop comparing anything and converge when the host says
//! to. Two properties have to hold for that to be available at all:
//!
//!   1. Rewriting the whole `values` model with no revision changed must fire
//!      **nothing** — otherwise the guard writes over a person on every
//!      present, which is F-30 through a different door.
//!   2. Bumping one slot's revision must fire that slot's handler and leave
//!      its neighbour's alone.

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use spike_lib::{GuardValue, Kind, Revision, SlotRow};

fn widget(ui: &Revision, id: &str) -> i_slint_backend_testing::ElementHandle {
    let wanted = id.to_owned();
    i_slint_backend_testing::ElementQuery::from_root(ui)
        .match_descendants()
        .match_predicate(move |element| {
            element.accessible_description().is_some_and(|d| d.as_str() == wanted)
        })
        .find_first()
        .unwrap_or_else(|| panic!("the fixture must draw {id}"))
}

#[test]
fn a_per_slot_revision_converges_one_field_and_not_its_neighbour() {
    i_slint_backend_testing::init_integration_test_with_system_time();

    let ui = Revision::new().expect("the spike must build");
    ui.set_rows(ModelRc::new(VecModel::from(vec![
        SlotRow { id: "left".into(), label: "left".into(), kind: Kind::Number, slot: 0 },
        SlotRow { id: "right".into(), label: "right".into(), kind: Kind::Number, slot: 1 },
    ])));

    let values = |revisions: [i32; 2]| {
        ModelRc::new(VecModel::from(
            revisions
                .iter()
                .map(|revision| GuardValue {
                    text: SharedString::from("0"),
                    number: 0.0,
                    revision: *revision,
                })
                .collect::<Vec<_>>(),
        ))
    };
    ui.set_values(values([0, 0]));

    let observed = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(i32, String, String)>::new()));
    let recording = observed.clone();
    let ui_weak = ui.as_weak();
    let timer = slint::Timer::default();
    let mut tick = 0;
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(30),
        move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let sample = |ui: &Revision| {
                (
                    ui.get_reasserts(),
                    widget(ui, "left").accessible_value().unwrap_or_default().to_string(),
                    widget(ui, "right").accessible_value().unwrap_or_default().to_string(),
                )
            };
            tick += 1;
            match tick {
                // Both people are mid-edit: each widget holds something the
                // host does not.
                1 => {
                    widget(&ui, "left").set_accessible_value("4");
                    widget(&ui, "right").set_accessible_value("7");
                }
                // A present that changes no revision. Property 1.
                2 => ui.set_values(values([0, 0])),
                3 => recording.borrow_mut().push(sample(&ui)),
                // The host discards the right field's edit and says so.
                // Property 2.
                4 => ui.set_values(values([0, 1])),
                5 => recording.borrow_mut().push(sample(&ui)),
                _ => slint::quit_event_loop().expect("the loop must stop"),
            }
        },
    );

    ui.show().expect("the window must show");
    slint::run_event_loop_until_quit().expect("the loop must run");

    let observed = observed.borrow();
    assert_eq!(observed.len(), 2, "two samples: {observed:?}");
    assert_eq!(
        observed[0],
        (0, "4".to_owned(), "7".to_owned()),
        "a present with no revision change must fire nothing"
    );
    assert_eq!(
        observed[1],
        (1, "4".to_owned(), "0".to_owned()),
        "one bump must converge one field and leave the other alone"
    );
}

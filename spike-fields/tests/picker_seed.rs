//! Slice 009 G-3 — F-31, measured, and refuted.
//!
//! Two `datetime` fields, neither touched, share one popup. Both seed to the
//! same date, so the second open writes the value the seed property already
//! holds. F-31 read `DatePickerBase` and argued that the second field would
//! therefore open on the first field's pick: an in-popup selection destroys
//! `current-date`'s binding, and only `changed date` — which fires on a
//! change, not on a write — re-syncs it.
//!
//! That is true of one popup instance, and there is never a second open of
//! one. `show-popup` compiles to a fresh `::new()` every time and the closed
//! instance is dropped, so `current-date` is re-bound from `date` on each
//! open. This fixture measures that: probe 4 reopens the very field whose
//! pick would have leaked.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, VecModel};
use spike_lib::{Date, Pickers, Seed};

fn click(ui: &Pickers, description: &str) {
    let wanted = description.to_owned();
    i_slint_backend_testing::ElementQuery::from_root(ui)
        .match_descendants()
        .match_predicate(move |element| {
            element.accessible_description().is_some_and(|d| d.as_str() == wanted)
        })
        .find_first()
        .unwrap_or_else(|| panic!("the fixture must draw {description}"))
        .invoke_accessible_default_action();
}

/// The popup's own controls, found by the label Slint gives them: `OK` on the
/// dialog's `StandardButton`, and the day number on a calendar cell.
fn click_labelled(ui: &Pickers, label: &str) {
    let wanted = label.to_owned();
    i_slint_backend_testing::ElementQuery::from_root(ui)
        .match_descendants()
        .match_predicate(move |element| {
            element.accessible_label().is_some_and(|l| l.as_str() == wanted)
        })
        .find_first()
        .unwrap_or_else(|| panic!("the popup must offer {label}"))
        .invoke_accessible_default_action();
}

#[test]
fn a_second_untouched_field_must_not_open_on_the_first_ones_pick() {
    i_slint_backend_testing::init_integration_test_with_system_time();

    let ui = Pickers::new().expect("the spike must build");
    // Both fields untouched, so both seed to the same date — the ordinary
    // case, and the one D21 exists for.
    ui.set_seeds(ModelRc::new(VecModel::from(vec![
        Seed { date: Date { year: 2026, month: 9, day: 17 } },
        Seed { date: Date { year: 2026, month: 9, day: 17 } },
    ])));
    let accepted = Rc::new(RefCell::new(Vec::<Date>::new()));
    let seen = accepted.clone();
    let ui_weak = ui.as_weak();
    let timer = slint::Timer::default();
    let mut tick = 0;
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(30),
        move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let take = |ui: &Pickers, seen: &RefCell<Vec<Date>>| {
                seen.borrow_mut().push(ui.get_last_accepted());
            };
            tick += 1;
            match tick {
                // 1. What an untouched field's picker opens on, with nothing
                //    to have polluted it. The control for everything below.
                1 => click(&ui, "field-a"),
                2 => {
                    click_labelled(&ui, "OK");
                    take(&ui, &seen);
                }
                // 2. The same field, with a deliberate pick.
                3 => click(&ui, "field-a"),
                4 => click_labelled(&ui, "20"),
                5 => {
                    click_labelled(&ui, "OK");
                    take(&ui, &seen);
                }
                // 3. The other untouched field, seeded to the same date. This
                //    is the case D21 is written to prevent.
                6 => click(&ui, "field-b"),
                7 => {
                    click_labelled(&ui, "OK");
                    take(&ui, &seen);
                }
                // 4. Reopen the field that *was* picked. Its own seed is
                //    still the untouched one in this fixture, so a retained
                //    `current-date` would show here too.
                8 => click(&ui, "field-a"),
                9 => {
                    click_labelled(&ui, "OK");
                    take(&ui, &seen);
                }
                _ => slint::quit_event_loop().expect("the loop must stop"),
            }
        },
    );

    ui.show().expect("the window must show");
    slint::run_event_loop_until_quit().expect("the loop must run");

    let accepted = accepted.borrow();
    assert_eq!(accepted.len(), 4, "four probes: {accepted:?}");
    let (first_open, picked, second_field, reopened) =
        (&accepted[0], &accepted[1], &accepted[2], &accepted[3]);

    assert_eq!(first_open.day, 17, "an untouched field's first open is seeded");
    assert_eq!(picked.day, 20, "a pick inside the popup is what comes back");
    assert_eq!(
        second_field.day, 17,
        "the second untouched field opens on its own seed, even though \
         seeding it wrote the value the seed property already held"
    );
    assert_eq!(
        reopened.day, 17,
        "and the picked field itself reopens on its seed: no popup state \
         survives a close"
    );
}

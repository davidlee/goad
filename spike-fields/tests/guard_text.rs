//! Slice 009 G-1 — F-30, measured.
//!
//! The guard in `design.md` §5.2 compares the widget's text against the
//! host's `text` and converges where they differ, with one exception for a
//! cleared field over a held zero. Both policies below run **that same
//! markup**; the only thing that varies is what the host puts in `text`:
//!
//!   `Reformat` — the design as written: `text` is the host's rendering of
//!                the `f64` it parsed. Text → f64 → text is not the identity.
//!   `Verbatim` — the candidate: `text` is the string the person sent, and
//!                the `f64` beside it is what that string parsed to.
//!
//! Each case is a slot with its own script, so one window and one loop run
//! measure both policies against the same widget code.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use spike_lib::{Guard, GuardValue, Kind, SlotRow};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Policy {
    Reformat,
    Verbatim,
}

#[derive(Clone, Copy, Debug)]
enum Key {
    Append(char),
    Clear,
    Idle,
}

struct Case {
    id: &'static str,
    policy: Policy,
    /// Whether the host records what this case's edits carry. `false` is
    /// AC-6: the command is lost or refused and the draft does not move.
    records: bool,
    keys: [Key; 7],
    expected_text: &'static str,
    expected_number: f64,
}

/// What the host holds for one field.
#[derive(Clone, Debug)]
struct Held {
    text: String,
    number: f64,
}

/// §5.2's parse rule (D-16): empty is zero; a lone comma with no dot is the
/// decimal separator; then `f64::from_str`. `None` is what `Finite::new`
/// refuses — a parse failure or a non-finite result.
fn parse(text: &str) -> Option<f64> {
    if text.is_empty() {
        return Some(0.0);
    }
    let normalised = if !text.contains('.') && text.matches(',').count() == 1 {
        text.replace(',', ".")
    } else {
        text.to_owned()
    };
    normalised.parse::<f64>().ok().filter(|value| value.is_finite())
}

fn record(held: &mut Held, policy: Policy, typed: &str) {
    match (policy, parse(typed)) {
        // The design as written: the number is the record, and the text is
        // re-derived from it.
        (Policy::Reformat, Some(value)) => {
            held.number = value;
            held.text = format!("{value}");
        }
        // Nothing representable, so nothing recorded at all.
        (Policy::Reformat, None) => {}
        // The candidate: the text is the record too.
        (Policy::Verbatim, Some(value)) => {
            held.number = value;
            held.text = typed.to_owned();
        }
        // The number the host can still answer with stands; the text is what
        // the person is holding, so the guard has nothing to fight.
        (Policy::Verbatim, None) => {
            held.text = typed.to_owned();
        }
    }
}

const CASES: &[Case] = &[
    Case {
        id: "reformat-fraction",
        policy: Policy::Reformat,
        records: true,
        keys: [Key::Clear, Key::Append('1'), Key::Append('.'), Key::Append('0'),
               Key::Append('5'), Key::Idle, Key::Idle],
        // The person typed `1.05`. F-30 says they do not get it.
        expected_text: "105",
        expected_number: 105.0,
    },
    Case {
        id: "verbatim-fraction",
        policy: Policy::Verbatim,
        records: true,
        keys: [Key::Clear, Key::Append('1'), Key::Append('.'), Key::Append('0'),
               Key::Append('5'), Key::Idle, Key::Idle],
        expected_text: "1.05",
        expected_number: 1.05,
    },
    Case {
        id: "reformat-negative",
        policy: Policy::Reformat,
        records: true,
        keys: [Key::Clear, Key::Append('-'), Key::Append('3'), Key::Idle,
               Key::Idle, Key::Idle, Key::Idle],
        // `-` alone does not parse, so nothing is recorded and the guard
        // replaces the sign before the digit arrives.
        expected_text: "3",
        expected_number: 3.0,
    },
    Case {
        id: "verbatim-negative",
        policy: Policy::Verbatim,
        records: true,
        keys: [Key::Clear, Key::Append('-'), Key::Append('3'), Key::Idle,
               Key::Idle, Key::Idle, Key::Idle],
        expected_text: "-3",
        expected_number: -3.0,
    },
    Case {
        // `input-type: decimal` admits this: Slint validates through an `f32`
        // parse, where `1e400` is infinity and therefore `Some`. The host's
        // `Finite` refuses it. Under `Verbatim` the widget keeps the text and
        // the last representable number stands.
        id: "verbatim-overflow",
        policy: Policy::Verbatim,
        records: true,
        keys: [Key::Clear, Key::Append('1'), Key::Append('e'), Key::Append('4'),
               Key::Append('0'), Key::Append('0'), Key::Idle],
        expected_text: "1e400",
        expected_number: 1e40,
    },
    Case {
        // AC-6. The edit reaches nothing, the draft does not move, and the
        // next present must put the widget back.
        id: "verbatim-dropped",
        policy: Policy::Verbatim,
        records: false,
        keys: [Key::Append('4'), Key::Idle, Key::Idle, Key::Idle, Key::Idle,
               Key::Idle, Key::Idle],
        expected_text: "0",
        expected_number: 0.0,
    },
    Case {
        // The measured case `numeric_guard.rs` found: a clear in flight, with
        // the host still holding zero. The exception is what keeps it.
        id: "verbatim-clear-race",
        policy: Policy::Verbatim,
        records: false,
        keys: [Key::Clear, Key::Idle, Key::Idle, Key::Idle, Key::Idle,
               Key::Idle, Key::Idle],
        expected_text: "",
        expected_number: 0.0,
    },
];

fn widget(ui: &Guard, id: &str) -> i_slint_backend_testing::ElementHandle {
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
fn the_guard_must_not_fight_ordinary_typing() {
    i_slint_backend_testing::init_integration_test_with_system_time();

    let ui = Guard::new().expect("the spike must build");

    ui.set_rows(ModelRc::new(VecModel::from(
        CASES
            .iter()
            .enumerate()
            .map(|(slot, case)| SlotRow {
                id: case.id.into(),
                label: case.id.into(),
                kind: Kind::Number,
                slot: i32::try_from(slot).expect("seven slots"),
            })
            .collect::<Vec<_>>(),
    )));

    // Every field starts as-drawn: the number zero, shown as `0`.
    let held = Rc::new(RefCell::new(
        CASES
            .iter()
            .map(|_| Held { text: "0".to_owned(), number: 0.0 })
            .collect::<Vec<_>>(),
    ));

    // The debounce, fired immediately: an edit the host records lands before
    // the present that follows it. That is the window F-30 turns on.
    let recording = held.clone();
    ui.on_edited(move |slot, text| {
        let slot = usize::try_from(slot).expect("a slot is an index");
        if CASES[slot].records {
            record(&mut recording.borrow_mut()[slot], CASES[slot].policy, text.as_str());
        }
    });

    let present = {
        let held = held.clone();
        move |ui: &Guard| {
            ui.set_values(ModelRc::new(VecModel::from(
                held.borrow()
                    .iter()
                    .map(|one| GuardValue {
                        text: SharedString::from(one.text.as_str()),
                        number: one.number as f32,
                        revision: 0,
                    })
                    .collect::<Vec<_>>(),
            )));
            ui.set_epoch(ui.get_epoch() + 1);
        }
    };
    present(&ui);

    let trace = Rc::new(RefCell::new(Vec::<String>::new()));
    let final_text = Rc::new(RefCell::new(Vec::<String>::new()));

    let ui_weak = ui.as_weak();
    let timer = slint::Timer::default();
    let mut tick = 0usize;
    let recorded = trace.clone();
    let settled = final_text.clone();
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(30),
        move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            if tick < 7 {
                for (slot, case) in CASES.iter().enumerate() {
                    let element = widget(&ui, case.id);
                    let current = element.accessible_value().unwrap_or_default();
                    match case.keys[tick] {
                        Key::Append(character) => {
                            element.set_accessible_value(format!("{current}{character}"));
                        }
                        Key::Clear => element.set_accessible_value(""),
                        Key::Idle => {}
                    }
                    let _ = slot;
                }
                present(&ui);
                recorded.borrow_mut().push(format!(
                    "tick {tick}: {:?}",
                    CASES
                        .iter()
                        .map(|case| widget(&ui, case.id).accessible_value().unwrap_or_default().to_string())
                        .collect::<Vec<_>>()
                ));
                tick += 1;
            } else {
                *settled.borrow_mut() = CASES
                    .iter()
                    .map(|case| widget(&ui, case.id).accessible_value().unwrap_or_default().to_string())
                    .collect();
                slint::quit_event_loop().expect("the loop must stop");
            }
        },
    );

    ui.show().expect("the window must show");
    slint::run_event_loop_until_quit().expect("the loop must run");

    let trace = trace.borrow();
    let final_text = final_text.borrow();
    let held = held.borrow();

    assert!(ui.get_fires() > 0, "the guard must have run at all");

    for (slot, case) in CASES.iter().enumerate() {
        assert_eq!(
            final_text[slot], case.expected_text,
            "{}: widget text\n{}",
            case.id,
            trace.join("\n")
        );
        assert_eq!(
            held[slot].number, case.expected_number,
            "{}: the number the host would submit\n{}",
            case.id,
            trace.join("\n")
        );
    }
}

// Spike: does the guarded re-assert preserve interaction state, and does one
// fat struct carry five kinds? Throwaway.
use std::cell::RefCell;
use std::rc::Rc;

use slint::{Model, ModelRc, SharedString, VecModel};

slint::include_modules!();

/// Stands in for the host's `Draft`: the authority the widgets converge to.
#[derive(Default)]
struct Draft {
    rows: Vec<FieldRow>,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = Spike::new()?;

    let alternatives = |items: &[&str]| -> ModelRc<SharedString> {
        ModelRc::new(VecModel::from(
            items.iter().map(|s| SharedString::from(*s)).collect::<Vec<_>>(),
        ))
    };

    let row = |id: &str, label: &str, kind: Kind| FieldRow {
        id: id.into(),
        label: label.into(),
        kind,
        checked: false,
        text: SharedString::new(),
        number: 0.0,
        minimum: 0.0,
        maximum: 100.0,
        alternatives: alternatives(&[]),
    };

    let initial = vec![
        row("done", "Did the thing", Kind::Boolean),
        FieldRow { text: "".into(), ..row("notes", "Notes", Kind::Text) },
        FieldRow { number: 30.0, ..row("mins", "Minutes", Kind::Number) },
        FieldRow {
            alternatives: alternatives(&["low", "medium", "high"]),
            text: "medium".into(),
            ..row("mood", "Mood", Kind::Choice)
        },
        row("when", "When", Kind::Datetime),
    ];

    let draft = Rc::new(RefCell::new(Draft { rows: initial.clone() }));
    let model = Rc::new(VecModel::from(initial));
    ui.set_fields(ModelRc::from(model.clone()));

    // U-3: the callback hands the whole row back; the host reads the slot the
    // kind names. No parsing, no per-kind callback.
    {
        let draft = draft.clone();
        let ui_weak = ui.as_weak();
        ui.on_edited(move |edit| {
            let mut draft = draft.borrow_mut();
            if let Some(held) = draft.rows.iter_mut().find(|r| r.id == edit.id) {
                match edit.kind {
                    Kind::Boolean => held.checked = edit.checked,
                    Kind::Text | Kind::Choice | Kind::Datetime => held.text = edit.text.clone(),
                    Kind::Number => held.number = edit.number,
                }
            }
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_log(format!("edited {} → {:?}", edit.id, describe(&edit)).into());
            }
        });
    }

    // U-1: a present that writes every row in place. The element is never
    // destroyed; the epoch bump is what lets each row re-assert.
    let present_in_place = {
        let draft = draft.clone();
        let model = model.clone();
        let ui_weak = ui.as_weak();
        move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            for (i, held) in draft.borrow().rows.iter().enumerate() {
                model.set_row_data(i, held.clone());
            }
            ui.set_epoch(ui.get_epoch() + 1);
        }
    };

    {
        let present = present_in_place.clone();
        ui.on_present_row_data(move || present());
    }

    // The contrast: today's `set_vec`.
    {
        let draft = draft.clone();
        let model = model.clone();
        let ui_weak = ui.as_weak();
        ui.on_present_set_vec(move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            model.set_vec(draft.borrow().rows.clone());
            ui.set_epoch(ui.get_epoch() + 1);
        });
    }

    // A-2's case: the draft and the widget disagree because the host never
    // heard the edit. A present must correct the widget.
    {
        let draft = draft.clone();
        let present = present_in_place.clone();
        ui.on_diverge(move || {
            {
                let mut draft = draft.borrow_mut();
                for held in &mut draft.rows {
                    match held.kind {
                        Kind::Boolean => held.checked = !held.checked,
                        Kind::Text => held.text = "corrected by the draft".into(),
                        Kind::Number => held.number = 7.0,
                        _ => {}
                    }
                }
            }
            present();
        });
    }

    // Drive the question without hands: bump `probe` (root-level `changed`)
    // and present in place (per-row `changed`) on a timer, and read the
    // counters off the window.
    let timer = slint::Timer::default();
    {
        let present = present_in_place.clone();
        let ui_weak = ui.as_weak();
        timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_millis(800),
            move || {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_probe(ui.get_probe() + 1);
                }
                present();
            },
        );
    }

    ui.run()
}

fn describe(edit: &FieldRow) -> String {
    match edit.kind {
        Kind::Boolean => format!("{}", edit.checked),
        Kind::Number => format!("{}", edit.number),
        _ => format!("{:?}", edit.text),
    }
}

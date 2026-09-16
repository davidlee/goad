//! Does a present destroy the element the person is interacting with?
//!
//! `inits` counts element construction: an element that is destroyed and
//! recreated runs `init` again. `reasserts` counts the guarded write actually
//! firing. Together they say whether interaction state could have survived,
//! without needing to synthesise a keystroke.

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use spike_lib::{FieldRow, Kind, Spike};
use std::rc::Rc;

fn rows() -> Vec<FieldRow> {
    let empty: ModelRc<SharedString> = ModelRc::new(VecModel::from(Vec::<SharedString>::new()));
    let make = |id: &str, kind: Kind| FieldRow {
        id: id.into(),
        label: id.into(),
        kind,
        checked: false,
        text: SharedString::new(),
        number: 0.0,
        minimum: 0.0,
        maximum: 100.0,
        alternatives: empty.clone(),
    };
    vec![
        make("done", Kind::Boolean),
        make("notes", Kind::Text),
        make("mins", Kind::Number),
    ]
}

/// Four instrumented widget kinds are in the markup; three are in this
/// fixture, so a rebuild costs exactly three `init`s.
const ROWS: i32 = 3;

struct Harness {
    ui: Spike,
    model: Rc<VecModel<FieldRow>>,
}

impl Harness {
    fn new() -> Self {
        i_slint_backend_testing::init_no_event_loop();
        let ui = Spike::new().expect("the spike must build");
        let model = Rc::new(VecModel::from(rows()));
        ui.set_fields(ModelRc::from(model.clone()));
        let harness = Self { ui, model };
        harness.settle();
        harness
    }

    /// The repeater builds lazily and change handlers run inside
    /// `WindowInner::ensure_tree_instantiated`, which an element query is what
    /// reaches from outside. Walking the tree is therefore the pump, not a
    /// read.
    fn settle(&self) {
        let found = i_slint_backend_testing::ElementQuery::from_root(&self.ui)
            .match_descendants()
            .find_all();
        assert!(
            !found.is_empty(),
            "the query must see the tree — without `with_debug_info` it returns \
             empty and every assertion below passes vacuously"
        );
    }

    fn present_in_place(&self, rows: Vec<FieldRow>) {
        for (index, row) in rows.into_iter().enumerate() {
            self.model.set_row_data(index, row);
        }
        self.ui.set_epoch(self.ui.get_epoch() + 1);
        self.settle();
    }

    fn checkbox(&self) -> i_slint_backend_testing::ElementHandle {
        i_slint_backend_testing::ElementQuery::from_root(&self.ui)
            .match_descendants()
            .match_inherits("CheckBox")
            .find_first()
            .expect("the fixture draws one checkbox")
    }

    fn present_set_vec(&self, rows: Vec<FieldRow>) {
        self.model.set_vec(rows);
        self.ui.set_epoch(self.ui.get_epoch() + 1);
        self.settle();
    }
}

/// U-1, the load-bearing claim: a present that writes rows in place does not
/// destroy the elements, so nothing the person is doing can be interrupted.
#[test]
fn a_present_in_place_builds_no_new_elements() {
    let harness = Harness::new();
    let before = harness.ui.get_inits();
    assert!(before > 0, "the repeater must have built something to measure");

    harness.present_in_place(rows());
    harness.present_in_place(rows());
    harness.present_in_place(rows());

    assert_eq!(
        harness.ui.get_inits(),
        before,
        "three presents in place must construct no element"
    );
}

/// The negative control, and today's behaviour: `set_vec` resets the model,
/// which clears the repeater's instances outright.
#[test]
fn a_present_by_set_vec_rebuilds_every_element() {
    let harness = Harness::new();
    let before = harness.ui.get_inits();

    harness.present_set_vec(rows());

    assert_eq!(
        harness.ui.get_inits(),
        before + ROWS,
        "set_vec must rebuild every row — this is the mechanism being replaced"
    );
}




/// The assumption underneath everything: does clicking a `CheckBox` really
/// destroy the `checked: field.checked` binding at the use site?
///
/// `fluent/checkbox.slint:27` does `root.checked = !root.checked`, and a
/// property assignment in Slint is supposed to replace whatever binding the
/// use site declared. If that is true, a present in place cannot correct a
/// clicked widget and A-2 needs the rebuild. If it is false, `set_row_data`
/// alone is sufficient and no re-assert mechanism is needed at all.
///
/// **Measured: it does.** After the click the widget holds `true` and a present
/// in place writing `false` leaves it `true`. So A-2 genuinely requires either a
/// rebuild or an imperative re-assert, and `set_row_data` alone is insufficient.
///
/// No `changed` handler is involved. This measures the binding and nothing else.
#[test]
fn does_a_click_destroy_the_use_site_binding() {
    let harness = Harness::new();
    let checkbox = harness.checkbox();

    checkbox.invoke_accessible_default_action();
    assert_eq!(
        checkbox.accessible_checked(),
        Some(true),
        "the click must take, or the rest measures nothing"
    );

    // The draft still says false. Write it in place, with no epoch and no
    // re-assert: only the declarative binding could carry this.
    harness.present_in_place(rows());

    let followed = harness.checkbox().accessible_checked();
    assert_eq!(
        followed,
        Some(true),
        "the binding is gone: a present in place cannot correct a clicked widget"
    );
}

/// Does the rebuild restore the binding? This is what A-2 relies on today and
/// it has never been asserted either.
#[test]
fn a_rebuild_restores_the_binding_a_click_destroyed() {
    let harness = Harness::new();
    harness.checkbox().invoke_accessible_default_action();
    assert_eq!(harness.checkbox().accessible_checked(), Some(true));

    harness.present_set_vec(rows());

    assert_eq!(
        harness.checkbox().accessible_checked(),
        Some(false),
        "a rebuilt element gets a fresh binding and converges to the draft"
    );
}

/// **Measured: `changed` fires nowhere under `init_no_event_loop`** — not in a
/// repeater and not at the root. Change trackers run inside
/// `WindowInner::ensure_tree_instantiated`, which this backend never reaches.
///
/// This is the finding with the sharpest consequence: the repo's whole
/// `tests/renderer/` tier uses `init_no_event_loop`, so a case written there
/// asserting the re-assert would pass while measuring nothing. `with_loop.rs`
/// is where the mechanism is actually observable.
#[test]
fn changed_does_not_fire_without_an_event_loop() {
    let harness = Harness::new();
    let before = harness.ui.get_root_changes();

    harness.ui.set_probe(harness.ui.get_probe() + 1);
    harness.settle();

    assert_eq!(
        harness.ui.get_root_changes(),
        before,
        "if this ever fires, the tier constraint below has been lifted"
    );
}

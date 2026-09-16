//! What the window asks the compositor for.
//!
//! Its own module because the claim is about the *window*, not about anything
//! drawn in it: every other module here asks the markup what it drew, and this
//! one asks how big the thing that holds it says it needs to be.
//!
//! It exists because slice 007 shipped without it and the defect it holds went
//! unseen for four slices. Measured at that audit, `PromptWindow` had **no
//! content-derived preferred size at all** — 50×65 for two plain options, for
//! two options with two fields each, and for one option with five, at which one
//! of two option controls is reachable. The mechanism was a `ScrollView`, which
//! propagates no preferred size from its content, over a `Window` declaring no
//! width, height or minimum of its own.
//!
//! **It is invisible under a compositor that sizes windows itself**, which is
//! why two human acceptance criteria named as its observers could not have seen
//! it, and why it wants a test rather than another pair of eyes
//! (`docs/memory/a-fixtures-size-is-not-the-products.md`).
//!
//! **Negative result: there is no width case here, and that is measured.** The
//! window asks for 420px whatever is on it — no option at all, a 420-character
//! option label, a 450-character checkbox label — because *nothing* propagates
//! a width upward, not even a control's own minimum. A case asserting that
//! independence passed with `preferred-width` deleted and passed again with the
//! `ScrollView` replaced by a plain layout, so it discriminated nothing and was
//! removed rather than kept green
//! (`docs/memory/a-green-test-can-assert-a-proxy.md`). The only assertion left
//! available is the literal in the markup, which a visual pass may re-tune and
//! which no reader would learn anything from.
use goad::generated::{FieldBlock, FieldRow, OptionRow, PromptWindow, WindowMode};
use i_slint_backend_testing::init_no_event_loop;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

/// One option carrying `fields` boolean fields under a single heading, or none
/// at all when `fields` is zero — an option with no fields draws no container,
/// which is the shape the 007 measurement used.
fn option_of(id: &str, fields: usize) -> OptionRow {
  let rows: Vec<FieldRow> = (0..fields)
    .map(|index| FieldRow {
      id: SharedString::from(format!("f{index}")),
      label: SharedString::from(format!("Field {index}")),
      checked: false,
    })
    .collect();
  OptionRow {
    id: SharedString::from(id),
    label: SharedString::from("Answer"),
    view: SharedString::from("v1"),
    blocks: if rows.is_empty() {
      ModelRc::default()
    } else {
      ModelRc::new(VecModel::from(vec![FieldBlock {
        heading: SharedString::from("Block"),
        fields: ModelRc::new(VecModel::from(rows)),
      }]))
    },
  }
}

/// The size a freshly shown window asks for, with `rows` on it.
///
/// A new window per call rather than one rewritten: the claim is about what a
/// window asks for when it is *opened* on a given shape, which is the moment a
/// compositor reads it.
fn asked_for(rows: Vec<OptionRow>) -> (u32, u32) {
  let window = PromptWindow::new().expect("a headless window must construct");
  window.set_mode(WindowMode::Prompt);
  window.set_heading(SharedString::from("Fill in your interstitial journal?"));
  window.set_options(ModelRc::new(VecModel::from(rows)));
  window.show().expect("a headless window must show");
  let size = window.window().size();
  window.hide().expect("a headless window must hide");
  (size.width, size.height)
}

/// The testing backend's platform is per **thread** and set once
/// (`i-slint-backend-testing/lib.rs:37-46`), so each case initialises it for
/// its own thread and then builds as many windows as it needs
/// (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
#[test]
fn the_height_a_window_asks_for_follows_what_is_on_it() {
  init_no_event_loop();

  let (_, plain) = asked_for(vec![option_of("a", 0), option_of("b", 0)]);
  let (_, two_each) = asked_for(vec![option_of("a", 2), option_of("b", 2)]);
  let (_, five) = asked_for(vec![option_of("a", 5)]);

  assert!(
    plain < five && five < two_each,
    "a window's asked-for height must order with the content on it, and these \
     three were identical at 007's audit: plain {plain}, five fields {five}, \
     two options of two {two_each}"
  );
}

/// The other half of the same repair, and the reason the propagation is capped
/// rather than restored whole: `R-15` places no bound on how many fields an
/// option carries, so a *conforming* backend can ask for a window taller than
/// the screen. Past the cap the scroller does its job and the window stops
/// growing.
#[test]
fn past_the_cap_more_content_does_not_make_the_window_taller() {
  init_no_event_loop();

  let (_, forty) = asked_for(vec![option_of("a", 40)]);
  let (_, eighty) = asked_for(vec![option_of("a", 80)]);

  assert_eq!(
    forty, eighty,
    "forty fields and eighty must ask for the same height once the cap bites"
  );
}

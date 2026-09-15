//! What two or more case files in this target need beyond the shared
//! host-driving helpers: a headless window and tray, a glass over them, a
//! fixed clock, the option-scoped element queries, and the poll loop
//! `serve`-driven tests use to observe an event the driving code does not
//! control directly. Anything two or more case files here need lives in this
//! file; anything one of them needs stays where it is. What both *tiers* need
//! lives in `tests/support/driving.rs` instead (`design.md` §12.8,
//! `crates/goad-shell/tests/integration/harness.rs:5-10`).
//!
//! The rule is *two or more*, and the set that satisfies it is not fixed:
//! `tree.rs` became a caller when `wiring.rs` needed the scoped field query
//! too, which is why nothing here names a closed list of case files.

use std::rc::Rc;
use std::time::Duration;

use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad_semantics::protocol::canonical::Timestamp;
use goad_shell::clock::ClockError;
use i_slint_backend_testing::{ElementHandle, ElementQuery, init_no_event_loop};
use slint::{ComponentHandle, Model, VecModel};

use crate::driving::instant;

pub(crate) const TIMEOUT: Duration = Duration::from_secs(2);

pub(crate) fn now() -> Timestamp {
  instant("2026-01-01T00:00:00Z")
}

/// A `Clock` (`fn() -> Result<Timestamp, ClockError>`) fixed to [`now`]. A
/// plain top-level `fn`, not a closure: `Clock` is a `fn` pointer type
/// (`goad_shell::clock`), the same reason `serve` itself takes one rather than a
/// `dyn Fn`. Shared by `mod serving`, `mod interaction` and `mod
/// cancellation`, all of which call `serve` directly.
#[expect(
  clippy::unnecessary_wraps,
  reason = "must match `Clock`'s `fn() -> Result<Timestamp, ClockError>` \
    signature to be passed to `serve`; a test fixture never needs to \
    exercise the error arm (test code, outside VA-3's src/-only budget)"
)]
pub(crate) fn stub_clock() -> Result<Timestamp, ClockError> {
  Ok(now())
}

pub(crate) fn window_and_tray() -> (PromptWindow, Tray) {
  init_no_event_loop();
  (
    PromptWindow::new().expect("a headless window must construct"),
    Tray::new().expect("a headless tray must construct"),
  )
}

pub(crate) fn glass_over(window: &PromptWindow, tray: &Tray) -> SlintGlass {
  SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
  )
}

/// The view token a real click would carry, read off the options model
/// exactly as `app.slint`'s `chosen` callback does (`option.view`) — so a
/// test builds a `Command::Choose` from what the window actually holds,
/// never from a second, independent minting of the same value (PHASE-10
/// repair, VT-4).
pub(crate) fn current_view_token(window: &PromptWindow) -> Option<String> {
  window
    .get_options()
    .row_data(0)
    .map(|row| row.view.to_string())
}

/// Everything under the option that answers to `option.id` — the scope every
/// field query starts from. `ElementQuery` has no accessible-description
/// matcher (`search_api.rs:232-287` lists its six builders), so the
/// description is read through `ElementHandle::accessible_description` inside
/// a predicate.
///
/// The option's control answers to `option.id` too, and is reached first; it
/// has no field beneath it, so the walk continues to the container.
pub(crate) fn within_option(window: &PromptWindow, option: &str) -> ElementQuery {
  let option = option.to_string();
  ElementQuery::from_root(window)
    .match_predicate(move |element| {
      element.accessible_description().as_deref() == Some(option.as_str())
    })
    .match_descendants()
}

/// A field's identity is **scoped, not composite**: the option's container
/// carries `option.id` and the field's control carries `field.id`, so the
/// query descends from the one to the other. An unscoped `find_first` does not
/// carry over — an option id is unique within a view (R-14) but a field id
/// only within an option (R-52), so an unscoped query would take whichever
/// came first and report no ambiguity, which is exactly the case AC-4 exists
/// to prove. Joining the two into one description was rejected: ids are
/// backend-supplied strings whose characters no requirement constrains, so any
/// separator can occur inside one (design.md §5.2).
///
/// Here rather than in one case file because two of them need it: `tree.rs`
/// asks the markup what it draws, and `wiring.rs` asks a presented window what
/// the draft put on it.
pub(crate) fn field_described(
  window: &PromptWindow,
  option: &str,
  field: &str,
) -> Option<ElementHandle> {
  let field = field.to_string();
  within_option(window, option)
    .match_predicate(move |element| {
      element.accessible_description().as_deref() == Some(field.as_str())
    })
    .find_first()
}

/// `waiting::within`, asserting: panics if `predicate` never becomes true
/// within `bound`. The shape every `serve`-driven test in this target needs
/// to observe an event the driving code does not control directly — an
/// invocation landing, a view arriving — rather than assume a fixed delay
/// covers it (PHASE-10 repair, VT-4/VT-10).
///
/// The poll itself is `tests/support/waiting.rs`'s, shared with the
/// event-loop tier; only the assertion is
/// this target's.
pub(crate) async fn until(bound: Duration, predicate: impl FnMut() -> bool) {
  assert!(
    crate::waiting::within(bound, predicate).await,
    "condition did not become true within {bound:?}"
  );
}

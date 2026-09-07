//! What two or more case files in this target need beyond the shared
//! host-driving helpers: a headless window and tray, a glass over them, a
//! fixed clock, and the poll loop `serve`-driven tests use to observe an
//! event the driving code does not control directly. Anything two or more of
//! `wiring.rs`/`table.rs`/`scheduling.rs` need lives here; anything one of
//! them needs stays where it is. What both *tiers* need lives in
//! `tests/support/driving.rs` instead (`design.md` §12.8,
//! `crates/goad-shell/tests/integration/harness.rs:5-10`).

use std::rc::Rc;
use std::time::Duration;

use goad::clock::ClockError;
use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad_semantics::protocol::canonical::Timestamp;
use i_slint_backend_testing::init_no_event_loop;
use slint::{ComponentHandle, Model, VecModel};

use crate::driving::instant;

pub(crate) const TIMEOUT: Duration = Duration::from_secs(2);

pub(crate) fn now() -> Timestamp {
  instant("2026-01-01T00:00:00Z")
}

/// A `Clock` (`fn() -> Result<Timestamp, ClockError>`) fixed to [`now`]. A
/// plain top-level `fn`, not a closure: `Clock` is a `fn` pointer type
/// (`clock.rs`), the same reason `serve` itself takes one rather than a
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

/// Poll `predicate` on a short fixed interval until it is true, panicking if
/// it never is within `bound`. The one shape every `serve`-driven test needs
/// to observe an event the driving code does not control directly — an
/// invocation landing, a view arriving — rather than assume a fixed delay
/// covers it (PHASE-10 repair, VT-4/VT-10).
pub(crate) async fn until(bound: Duration, mut predicate: impl FnMut() -> bool) {
  let deadline = std::time::Instant::now() + bound;
  loop {
    if predicate() {
      return;
    }
    assert!(
      std::time::Instant::now() < deadline,
      "condition did not become true within {bound:?}"
    );
    tokio::time::sleep(Duration::from_millis(5)).await;
  }
}

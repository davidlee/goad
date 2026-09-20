//! **What `busy` costs a person, measured with real key events.**
//!
//! A `[[test]]` target of its own, for the reason every `event_loop_*` target
//! here is one: `i_slint_backend_testing`'s per-process init "can only be
//! called once per process" (its own doc comment, D-12), so a case needing a
//! real loop cannot share a binary with `tests/renderer`'s
//! `init_no_event_loop()`.
//!
//! **Why it needs the loop at all**, which is the whole reason this file is
//! not four more lines in `tests/renderer/wiring.rs`: no case driven through
//! the accessibility surface can see an `enabled` binding.
//! `invoke_accessible_default_action` dispatches the declared action with no
//! `accessible-enabled` check
//! (`i-slint-backend-testing-1.17.1/search_api.rs:606-613`), and
//! `set_accessible_value` on a `LineEdit` is an assignment plus a call to
//! `edited` (`fluent/lineedit.slint:16`) — neither reaches
//! `TextInput::key_event` and its `enabled` gate (`i-slint-core/items/
//! text.rs:954`). Deleting every `enabled: !root.busy` from this project's
//! markup leaves the whole suite green, measured (`review-code.md` F-A1).
//! So the claim needs a **real** key event dispatched at a **laid-out**
//! window, and that is this tier.
#[cfg(test)]
mod busy;

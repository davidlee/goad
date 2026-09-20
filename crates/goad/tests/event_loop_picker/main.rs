//! **A `datetime` picker, open, while the view under it is replaced and then
//! closed** — the arrangement F-R1 names, run rather than reasoned.
//!
//! A `[[test]]` target of its own, and exactly one `#[test]` fn in it, for the
//! reason `event_loop/main.rs` and `event_loop_reassert/main.rs` both state:
//! `i_slint_backend_testing`'s per-process init "can only be called once per
//! process", so the unit of isolation is the target rather than the test
//! function (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
//!
//! **It must be the loop tier and cannot be the renderer tier.** What this
//! case measures is whether a *pointer event aimed at a control beneath an
//! open popup* reaches it, and a pointer event is resolved against laid-out
//! geometry. `tests/renderer/` runs under `init_no_event_loop`
//! (`tests/renderer/harness.rs:54`), which never lays the window out, so
//! `ElementHandle::absolute_position` there has nothing behind it. The
//! renderer tier's own picker cases drive every button by
//! `invoke_accessible_default_action` for exactly that reason
//! (`tests/renderer/fields.rs`, `only_button`) — and that API dispatches the
//! declared action unconditionally, reaching neither `enabled` nor hit
//! testing, so it cannot answer the question this file asks.
//!
//! **The init is `_with_system_time`**, like `event_loop_reassert`'s: the
//! stepping is a real `slint::Timer` and a mocked clock would stop it
//! advancing on its own.
//!
//! It includes none of `tests/support/`, for the reason
//! `event_loop_reassert/main.rs` gives: every symbol of every shared helper
//! would be unreachable from here and `dead_code` under `-D warnings` would
//! stop the gate.
#[cfg(test)]
mod picker;

//! The debounce's **timer** half, under a real if headless Slint event loop.
//!
//! `event_loop_reassert`'s arrangement, with one thing added that it has no
//! need of: a channel. `pending.rs` delivers through `Wire::send` and has no
//! other door, so a case that watches the timer deliver has to hold the
//! receiving end — but there is still **no runtime, no `serve` and no
//! `LocalSet`**. The stepper drains the channel itself and hands each command
//! to `Controller::edit`, which is what `serve`'s own `dispatch` arm does with
//! it and is the whole of what this target needs from the loop.
//!
//! It exists because a `slint::Timer` fires nowhere without an event loop, so
//! a case for the delivery rule written in `tests/renderer/` would be green
//! and measure nothing — which is `design.md` §8 R1. The **answer** path needs
//! no timer and is measured where it is cheaper, in
//! `tests/renderer/fields.rs` (PHASE-05/VT-3).
//!
//! A `[[test]]` target of its own, and exactly one `#[test]` fn in it, for the
//! reason `event_loop/main.rs` and `event_loop_reassert/main.rs` both already
//! state: `i_slint_backend_testing`'s per-process init "can only be called
//! once per process", so the unit of isolation is the target rather than the
//! test function
//! (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
//!
//! **The init is `_with_system_time`**, like `event_loop_reassert`'s: what is
//! under test here is a real `slint::Timer` reaching its deadline, and a
//! mocked clock would stop it advancing on its own.
//!
//! It includes none of `tests/support/`, for the reason `event_loop_reassert`
//! gives: every symbol of every shared helper would be unreachable from here
//! and `dead_code` under `-D warnings` would stop the gate.
#[cfg(test)]
mod debounce;

//! The **overlay**, under a real if headless Slint event loop.
//!
//! `event_loop_debounce`'s arrangement, with the glass given a clone of the
//! **same** `Debounce` the callback table was given — which is the whole of
//! what this target exists to measure. A case that constructs the two halves
//! separately gets an overlay that never overlays anything and stays green
//! (`design.md` §8 R10), so the construction here is the assertion's
//! precondition and is written in one place.
//!
//! It needs a real loop for two reasons at once, and neither is optional: a
//! `slint::Timer` fires nowhere without one, so an entry can leave the map only
//! here; and the guard is a `changed` handler, so a present that reverts a
//! widget reverts it only here (`design.md` §8 R1,
//! `docs/memory/change-handlers-need-an-event-loop.md`).
//!
//! A `[[test]]` target of its own, and exactly one `#[test]` fn in it, for the
//! reason the other four loop targets already state:
//! `i_slint_backend_testing`'s per-process init "can only be called once per
//! process", so the unit of isolation is the target rather than the test
//! function
//! (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
//!
//! **The init is `_with_system_time`**: what is under test includes a real
//! `slint::Timer` reaching its deadline, and a mocked clock would stop it
//! advancing on its own.
//!
//! It includes none of `tests/support/`, for the reason `event_loop_reassert`
//! gives: every symbol of every shared helper would be unreachable from here
//! and `dead_code` under `-D warnings` would stop the gate.
#[cfg(test)]
mod overlay;

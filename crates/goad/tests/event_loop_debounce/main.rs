//! The one thing about `pending.rs` that only a real event loop can show: the
//! timer fires at all, it delivers **one** entry per tick, and it **re-arms**
//! from inside its own callback so the second entry follows.
//!
//! A `[[test]]` target of its own for the reason `event_loop/main.rs` states —
//! `i_slint_backend_testing`'s initialisers "can only be called once per
//! process", so a loop-tier arrangement is one target and one `#[test]`
//! (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
//!
//! `slint::Timer` does not run under `init_no_event_loop`, measured (§9), so
//! nothing in `tests/prototype/` can watch this.
#[cfg(test)]
mod debounce;

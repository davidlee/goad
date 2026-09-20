//! The debounce's **refusal** half: what a tick does when the channel is full.
//!
//! `event_loop_debounce`'s arrangement with one thing removed — the stepper
//! does **not** drain on every step. That drain is why no case anywhere
//! produced a `Full` send from `Debounce::tick`, and so why the branch that
//! keeps a person's typing was held by nothing (`review-code.md` **F-S2**).
//! All three debounce-bearing loop targets drain on every step and say so
//! (`overlay.rs:236`, `numeric_guard.rs:241`, `debounce.rs:213`), and
//! `tests/renderer` never fires the timer at all. `wire.rs:331` unit-tests that
//! `send` *reports* `false`, which is a different claim from *the caller acts
//! on it*.
//!
//! The rule under test is `plan.md` PHASE-05/**EX-4** and **VA-1**: *an entry
//! leaves the map when the send that carries it is enqueued, not when it is
//! accepted*, and *a `Full` send clears nothing*. That asymmetry is the whole
//! reason `Wire::send` returns a `bool` (PHASE-05/EX-5), and in production it
//! is reachable whenever a tick lands while `serve` is awaiting an exchange
//! with a command already queued — the channel is capacity 1 (`start`'s
//! `mpsc::channel::<Command>(1)`) and `serve` does not drain while it is in
//! `select!`. A second tick 150 ms
//! into any exchange is the ordinary case.
//!
//! A `[[test]]` target of its own, and exactly one `#[test]` fn in it, for the
//! reason every sibling target states: `i_slint_backend_testing`'s per-process
//! init "can only be called once per process", so the unit of isolation is the
//! target rather than the test function
//! (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
//!
//! **The init is `_with_system_time`**, like `event_loop_debounce`'s: what is
//! under test is a real `slint::Timer` reaching its deadline, and a mocked
//! clock would stop it advancing on its own.
//!
//! It includes none of `tests/support/`, for the reason `event_loop_reassert`
//! gives: every symbol of every shared helper would be unreachable from here
//! and `dead_code` under `-D warnings` would stop the gate.
#[cfg(test)]
mod full;

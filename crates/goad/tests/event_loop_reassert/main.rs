//! The arrangement nothing else in this crate has: a **real, if headless,
//! Slint event loop around a glass presented directly**, with no runtime, no
//! channel and no `serve`.
//!
//! It exists because the guard that keeps a widget's value converged is a
//! `changed <property>` handler, and a `changed` handler fires nowhere under
//! `init_no_event_loop` — measured, not assumed
//! (`docs/memory/change-handlers-need-an-event-loop.md`, `research.md`
//! Thread 3). A case for it written in `tests/renderer/` would be green and
//! measure nothing, which is `design.md` §8 R1.
//!
//! A `[[test]]` target of its own, and exactly one `#[test]` fn in it, for the
//! reason `event_loop/main.rs` already states: `i_slint_backend_testing`'s
//! per-process init "can only be called once per process", so the unit of
//! isolation is the target rather than the test function
//! (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
//!
//! **The init is `_with_system_time`**, like `event_loop_schedule`'s and
//! unlike `event_loop`'s `_with_mock_time`: this target's stepping is a real
//! `slint::Timer`, and a mocked clock would stop it advancing on its own.
//!
//! It includes none of `tests/support/`. What this target drives is a
//! `Controller` and a `SlintGlass` and nothing else, so every symbol of every
//! shared helper would be unreachable from here and `dead_code` under
//! `-D warnings` would stop the gate
//! (`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`).
#[cfg(test)]
mod reassert;

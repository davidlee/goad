//! AC-10: the waiting mechanism proved in the arrangement production uses —
//! a real, if headless, Slint event loop, a multi-thread tokio runtime, its
//! `EnterGuard` held for the loop's life, `slint::spawn_local`, and the
//! production `serve` — driving a scheduled evaluation through a real
//! process backend.
//!
//! A separate `[[test]]` target from `renderer` and from `event_loop`, for
//! the same reason `event_loop` already is: `i_slint_backend_testing`'s
//! per-process init "can only be called once per process" (its own doc
//! comment), so mixing it with `init_no_event_loop()` in one binary is not
//! an option (`event_loop/main.rs`, D-12). This target's own init is
//! `init_integration_test_with_system_time()`, **not**
//! `event_loop`'s `_with_mock_time` — a mocked clock would stop tokio's
//! timers being the thing this test measures (design.md §5.5 A-1).
//!
//! **What is substituted, and what is not** (EX-6, F-8). Every component of
//! `main.rs`'s own arrangement is production's here — the multi-thread
//! runtime, the `EnterGuard`, a real `PromptWindow` and `Tray`,
//! `install`'s callback table, a real `mpsc` channel and `Cancel`,
//! `SlintGlass`, `ProcessBackend` against a real child, the production
//! `serve`, and `slint::spawn_local`. The **one** substitution is the Slint
//! platform itself: this target runs under `i_slint_backend_testing`'s
//! `TestingBackend`, never the platform `start` installs, and no headless
//! test can close that gap — it is stated here rather than searched for.
#[cfg(test)]
mod scheduling;

// Only the scripted-backend half of slice 001's test helpers (D-18, FD-3):
// this target builds its own `Config`/`Host` inline, exactly as
// `event_loop/closing.rs` does, and never calls `driving.rs`'s
// host-composition helpers — including that file too would leave most of
// its symbols unreachable from here, which `dead_code` (`-D warnings`)
// would catch.
#[cfg(test)]
#[path = "../../../../tests/support/scripting.rs"]
mod scripting;

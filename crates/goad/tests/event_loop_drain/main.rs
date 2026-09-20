//! **What a present lands on when a debounce tick fires mid-exchange** —
//! `review-code.md` **F-R3**, driven under the production `serve`.
//!
//! A `[[test]]` target of its own for the reason every `event_loop_*` target
//! here is one: `i_slint_backend_testing`'s per-process init "can only be
//! called once per process" (its own doc comment, D-12).
//!
//! **Why it is a whole topology rather than a stepper over a `Controller`.**
//! The interleave this file measures is `serve`'s own — an exchange parked in
//! the inner `select!` while the debounce timer enqueues on its own schedule —
//! and the three existing debounce-bearing loop targets cannot reach it,
//! because each drains the channel and applies the edit at the top of every
//! step (`event_loop_overlay/overlay.rs:238-249`). That is not a flaw in
//! them; it is why the defect was invisible, and the repair is production
//! being made to do what those harnesses already did.
//!
//! The one substitution is the backend, which is held rather than spawned:
//! the case needs an exchange that is *provably* still in flight while it
//! types, and a `sleep` in a shell script would be a race dressed as a
//! fixture. Everything else is `main.rs`'s own composition — a multi-thread
//! tokio runtime with its `EnterGuard` held for the loop's life, a real
//! window and tray, `install`'s callback table, a capacity-1 channel,
//! `SlintGlass`, the production `serve`, `slint::spawn_local` — minus the
//! Slint platform, which no headless test can supply.
#[cfg(test)]
mod drain;

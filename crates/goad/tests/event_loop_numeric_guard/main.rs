//! The **numeric guard's one exception**, under a real if headless Slint event
//! loop.
//!
//! `event_loop_overlay`'s arrangement — `install`'s callback table and the
//! glass sharing one `Debounce`, a stepper draining the channel and applying
//! what it drains — over a `number` field rather than two `text` ones.
//!
//! **A target of its own, and the reason is not that the arrangement is
//! novel.** It is that a loop target may hold exactly one `#[test]` fn
//! (`docs/memory/slint-testing-backend-initialises-once-per-process.md`), and
//! what this case is for is a **measurement taken twice**: `plan.md`
//! PHASE-08/EX-7 requires it run once as the tree ships and once with the
//! guard's exception removed, with both counts read. Folded into
//! `event_loop_overlay`'s single function, a failure on the second run could
//! not be told from a failure of PHASE-06's own claims, and that target's
//! stepper deliberately goes silent partway to measure a *dropped* edit —
//! which is the opposite of what this one needs.
//!
//! It needs a real loop for the same two reasons `event_loop_overlay` does,
//! and neither is optional: a `slint::Timer` fires nowhere without one, so an
//! entry can leave the debounce map only here; and the guard is a `changed`
//! handler, so a widget is written back only here (`design.md` §8 R1,
//! `docs/memory/change-handlers-need-an-event-loop.md`).
//!
//! **The init is `_with_system_time`**: what is under test includes a real
//! `slint::Timer` reaching its deadline, and a mocked clock would stop it
//! advancing on its own.
//!
//! It includes none of `tests/support/`, for the reason `event_loop_reassert`
//! gives: every symbol of every shared helper would be unreachable from here
//! and `dead_code` under `-D warnings` would stop the gate.
#[cfg(test)]
mod numeric_guard;

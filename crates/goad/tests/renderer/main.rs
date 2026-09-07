//! The cheap tier: headless, no display server, no socket opened
//! (design.md §5.1, `plan.md` EX-8). `#[cfg(test)]` on the declaration, not
//! on the module file itself, for `clippy::tests_outside_test_module` — a
//! `tests/` target is always built with `--test`, so the `cfg` is never off
//! (`crates/goad-boundary/tests/checks/main.rs` states the same reason).
//!
//! Eight modules today: `tree`, items 6-10 (PHASE-03); `mapper`/`tray`,
//! items 4, 5 and 16 (PHASE-04); `reception`, item 13 (PHASE-05); `table`,
//! item 12 (PHASE-06); `wiring`, item 11 in full and 14a-d (PHASE-07/10);
//! `startup`, item 17 (PHASE-08); `scheduling`, the timer arm (slice 003
//! PHASE-02).
//!
//! `driving` is the host-driving half of slice 001's test helpers
//! (design.md §12.8), shared with `crates/goad-shell/tests/integration`;
//! `table` and `wiring` are its callers in this target. `scripting` is the
//! scripted-backend half, split out at slice 003 PHASE-05 (D-18); `table`,
//! `wiring` and `scheduling` all call it too. `harness` is what two or more
//! of this target's own modules need (slice 003 PL-12); `scheduling` is one
//! of its callers.
#[cfg(test)]
mod harness;
#[cfg(test)]
mod mapper;
#[cfg(test)]
mod reception;
#[cfg(test)]
mod scheduling;
#[cfg(test)]
mod startup;
#[cfg(test)]
mod table;
#[cfg(test)]
mod tray;
#[cfg(test)]
mod tree;
#[cfg(test)]
mod wiring;

#[cfg(test)]
#[path = "../../../../tests/support/driving.rs"]
mod driving;
#[cfg(test)]
#[path = "../../../../tests/support/scripting.rs"]
mod scripting;
/// The shared poll loop behind `harness::until`, also included by the
/// `event_loop_schedule` target.
#[cfg(test)]
#[path = "../../../../tests/support/waiting.rs"]
mod waiting;

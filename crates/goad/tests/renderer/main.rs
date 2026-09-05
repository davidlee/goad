//! The cheap tier: headless, no display server, no socket opened
//! (design.md §5.1, `plan.md` EX-8). `#[cfg(test)]` on the declaration, not
//! on the module file itself, for `clippy::tests_outside_test_module` — a
//! `tests/` target is always built with `--test`, so the `cfg` is never off
//! (`crates/goad-boundary/tests/checks/main.rs` states the same reason).
//!
//! Four modules today: `tree`, items 6-10 (PHASE-03); `mapper`/`tray`,
//! items 4, 5 and 16 (PHASE-04); `reception`, item 13 (PHASE-05); `table`,
//! item 12 (PHASE-06). `wiring` and `startup` arrive with the phases that
//! give them something to test.
//!
//! `driving` is the host-driving half of slice 001's test helpers
//! (design.md §12.8), shared with `crates/goad-shell/tests/integration`;
//! `table` is its only caller in this target.
#[cfg(test)]
mod mapper;
#[cfg(test)]
mod reception;
#[cfg(test)]
mod table;
#[cfg(test)]
mod tray;
#[cfg(test)]
mod tree;

#[cfg(test)]
#[path = "../../../../tests/support/driving.rs"]
mod driving;

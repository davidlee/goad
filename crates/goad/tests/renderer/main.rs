//! The cheap tier: headless, no display server, no socket opened
//! (design.md §5.1, `plan.md` EX-8). `#[cfg(test)]` on the declaration, not
//! on the module file itself, for `clippy::tests_outside_test_module` — a
//! `tests/` target is always built with `--test`, so the `cfg` is never off
//! (`crates/goad-boundary/tests/checks/main.rs` states the same reason).
//!
//! One module today: `tree`, items 6-10 (PHASE-03). `mapper`, `wiring`,
//! `table`, `reception`, `tray` and `startup` arrive with the phases that
//! give them something to test.
#[cfg(test)]
mod tree;

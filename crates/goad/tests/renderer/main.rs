//! The cheap tier: headless, no display server, no socket opened
//! (design.md §5.1, `plan.md` EX-8). `#[cfg(test)]` on the declaration, not
//! on the module file itself, for `clippy::tests_outside_test_module` — a
//! `tests/` target is always built with `--test`, so the `cfg` is never off
//! (`crates/goad-boundary/tests/checks/main.rs` states the same reason).
//!
//! Two modules today: `tree`, items 6-10 (PHASE-03), and `mapper`/`tray`,
//! items 4, 5 and 16 (PHASE-04). `wiring`, `table`, `reception` and
//! `startup` arrive with the phases that give them something to test.
#[cfg(test)]
mod mapper;
#[cfg(test)]
mod tray;
#[cfg(test)]
mod tree;

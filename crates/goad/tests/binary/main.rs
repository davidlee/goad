//! Stratum 3 test target — **the built binary**, run as a process.
//!
//! What this tier holds is the one thing no other tier can see: that `main`
//! reaches an answer at all, on the stream and with the exit code a caller
//! reads (`design.md` §9). `arguments` returning `Launch::Version` and
//! `version_line` rendering it are both held as pure cases one tier down, and
//! neither says that the two are wired together.
//!
//! It is feasible **without a display** only because both zero-exits precede
//! any Slint call (006/design.md §5.4). Nothing here may construct a window:
//! a case that reached `start` would need a compositor and would red on every
//! headless machine the gate runs on.
//!
//! What it deliberately does not hold is the **stamped** revision. Nothing in
//! the gate sets `GOAD_REVISION`, so a process spawned here can only ever see
//! the bare form; `diagnostics.rs`'s own unit case is the real assertion for
//! the other branch, and a `starts_with` here would be a weaker case wearing
//! its clothes (`docs/memory/tests-asserting-proxies.md`).

// `#[cfg(test)]` on the declaration, as `goad-emit`'s own binary target
// explains: a `tests/` target is always built with `--test`, so this is never
// off, and without it `clippy::tests_outside_test_module` fires and
// `clippy.toml`'s `allow-expect-in-tests` does not apply.
#[cfg(test)]
mod version;

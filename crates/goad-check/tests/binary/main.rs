//! Stratum 3 test target — **the built binary**, run as a process.
//!
//! What this tier holds is the binary and nothing else: its exit status, its
//! stdout and its stderr (`design.md` §9).

// `#[cfg(test)]` on the declaration, as `goad-emit`'s binary target explains:
// a `tests/` target is always built with `--test`, so this is never off, and
// without it `clippy::tests_outside_test_module` fires and `clippy.toml`'s
// `allow-expect-in-tests` does not apply.
#[cfg(test)]
mod statuses;

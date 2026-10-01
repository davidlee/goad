//! Stratum 3 test target — **the built binary**, run as a process.
//!
//! What this tier holds is the binary and nothing else: its exit status, its
//! stdout and its stderr (`design.md` §9).

// `#[cfg(test)]` on the declarations, as `goad-emit`'s binary target explains:
// a `tests/` target is always built with `--test`, so this is never off, and
// without it `clippy::tests_outside_test_module` fires and `clippy.toml`'s
// `allow-expect-in-tests` does not apply. The `#[path]` include carries the
// attribute at its declaration site for the same reason, as
// `crates/goad-shell/tests/integration/main.rs` does.
#[cfg(test)]
mod process;

// The scripted backends a run is checked against. Included whole: this target
// uses `scripted`, `logging_backend` and `invocations`, which reach every other
// item in the file, so `dead_code` holds without a suppression.
#[cfg(test)]
#[path = "../../../../tests/support/scripting.rs"]
mod scripting;

#[cfg(test)]
mod run;

#[cfg(test)]
mod statuses;

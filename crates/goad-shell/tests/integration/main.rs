//! Stratum 2 test target — the transport tier, against real backend processes.
//! It is a target of `goad-shell`, so `cargo test -p goad-semantics` does not
//! build it at all; nothing gates it by feature.

// `#[cfg(test)]` on the declarations for `clippy::tests_outside_test_module`,
// as `crates/goad-semantics/tests/protocol/main.rs` explains: a `tests/` target
// is always built with `--test`, so this is never off. The `#[path]` include
// carries the attribute at its declaration site for the same reason.
#[cfg(test)]
#[path = "../../../../tests/support/driving.rs"]
mod driving;

#[cfg(test)]
mod harness;

#[cfg(test)]
mod fake;

#[cfg(test)]
mod failure_matrix;

#[cfg(test)]
mod host;

#[cfg(test)]
mod round_trip;

#[cfg(test)]
mod transport;

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

// The scripted-backend half of `driving.rs`, split out at slice 003
// PHASE-05 (D-18) so a target that never composes a `Host` can include it
// alone. This target still uses it, through `harness`'s re-export.
#[cfg(test)]
#[path = "../../../../tests/support/scripting.rs"]
mod scripting;

#[cfg(test)]
mod harness;

#[cfg(test)]
mod fake;

#[cfg(test)]
mod failure_matrix;

#[cfg(test)]
mod host;

#[cfg(test)]
mod ingress;

#[cfg(test)]
mod round_trip;

#[cfg(test)]
mod transport;

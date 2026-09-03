//! Stratum 2 test target — the transport tier, against real backend processes.
//! `required-features = ["shell"]` in `Cargo.toml` is what makes
//! `cargo test --no-default-features` skip this target rather than fail to
//! build it.

// `#[cfg(test)]` on the declarations for `clippy::tests_outside_test_module`,
// as `tests/protocol/main.rs` explains: a `tests/` target is always built with
// `--test`, so this is never off.
#[cfg(test)]
mod harness;

#[cfg(test)]
mod fake;

#[cfg(test)]
mod host;

#[cfg(test)]
mod transport;

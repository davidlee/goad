//! Stratum 2's source-shape target. Four properties of `process.rs` asserted
//! against its source text, which is why it sits with the crate whose source it
//! reads rather than with stratum 1's fixtures.

// `#[cfg(test)]` on the declaration for `clippy::tests_outside_test_module`, as
// `crates/goad-semantics/tests/protocol/main.rs` explains: a `tests/` target is
// always built with `--test`, so this is never off.
#[cfg(test)]
mod transport_shape;

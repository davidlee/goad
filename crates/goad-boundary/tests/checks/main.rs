//! The workspace-wide checks, as tests rather than intentions.
//!
//! `direction` is AC-15's direction half; `vocabulary` is AC-11's. Both are the
//! one walk in `goad_boundary::scan`, configured twice — and both of that walk's
//! vacuity controls live in `vocabulary`, beside the word-matching rules they
//! share.

// `#[cfg(test)]` on the declarations for `clippy::tests_outside_test_module`, as
// `crates/goad-semantics/tests/protocol/main.rs` explains: a `tests/` target is
// always built with `--test`, so this is never off.
#[cfg(test)]
mod direction;

#[cfg(test)]
mod vocabulary;

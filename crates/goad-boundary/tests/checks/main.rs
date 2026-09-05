//! The workspace-wide checks, as tests rather than intentions.
//!
//! Four instruments hold ADR-001's stratum 1 rule and one holds `CLAUDE.md`
//! invariant 1, each with a stated boundary the design does not let them share
//! (`design.md` §5.6, §9 item 3):
//!
//! - Cargo resolution — a stratum 1 source naming `goad_shell` or `tokio`
//!   fails to compile. No test in this crate; the compiler is the instrument.
//! - `allowlist` — the manifest allowlist. Names only.
//! - `purity` — the stratum 1 source scan. A regression tripwire, not a proof;
//!   its three misses are named beside it.
//! - `cargo test -p goad-semantics` — the gate's own third command, not a test
//!   here. It rejects nothing; it builds stratum 1 with exactly its own
//!   manifest's features.
//! - `vocabulary` — AC-13/AC-14, `CLAUDE.md` invariant 1, not one of the four.
//!
//! The `tokio` source grep this crate used to run (`direction.rs`) is retired
//! in this change: the allowlist and the purity scan are what replace it, and
//! carrying both the old grep and its replacement would claim the rule twice
//! at different strengths (§5.6, EX-9).

// `#[cfg(test)]` on the declarations for `clippy::tests_outside_test_module`, as
// `crates/goad-semantics/tests/protocol/main.rs` explains: a `tests/` target is
// always built with `--test`, so this is never off.
#[cfg(test)]
mod allowlist;
#[cfg(test)]
mod purity;
#[cfg(test)]
mod vocabulary;

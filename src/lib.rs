//! goad — a programmable personal intervention shell.
//!
//! ADR-001's strata are modules here rather than crates, per ADR-002. The
//! `cfg` below is not tidiness: Cargo resolves dependencies per crate target,
//! so gating stratum 2 behind the `shell` feature is what makes "stratum 1 has
//! no runtime in its dependency graph" a build gate rather than a convention
//! (D49, `design.md` §5.1).

pub mod semantics;

#[cfg(feature = "shell")]
pub mod shell;

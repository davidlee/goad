// crates/goad/src/lib.rs — the module tree, and nothing else (design.md
// §5.1). One `pub mod` line per phase; ten at PHASE-08, nine after 005
// lifted `clock` to stratum 2 (005/D-9), and ten again with `draft` — the
// one new file 007 adds, and the only pure one in this crate besides
// `view_model`.
pub mod controller;
pub mod diagnostics;
pub mod draft;
/// Canonical fixtures for the unit tests in this crate. `#[cfg(test)]`, so
/// it is not part of the binary's module tree — it is here rather than
/// duplicated inside two `mod tests` blocks because a canonical id can only
/// be *read* off a normalized view, which costs more than a test should
/// spend twice.
#[cfg(test)]
mod fixture;
pub mod generated;
pub mod glass;
pub mod install;
pub mod pending;
pub mod reception;
pub mod startup;
pub mod view_model;
pub mod wire;
pub mod zoom;

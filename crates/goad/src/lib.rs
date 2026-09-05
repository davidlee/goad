// crates/goad/src/lib.rs — the module tree, and nothing else (design.md
// §5.1). One `pub mod` line per phase; ten at PHASE-08. No `main.rs` yet.
pub mod clock;
pub mod controller;
pub mod diagnostics;
pub mod generated;
pub mod glass;
pub mod install;
pub mod reception;
pub mod view_model;
pub mod wire;

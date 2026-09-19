// crates/goad/src/lib.rs — the module tree, and nothing else (design.md
// §5.1). One `pub mod` line per phase; ten at PHASE-08, nine after 005
// lifted `clock` to stratum 2 (005/D-9), and ten again with `draft` — the
// one new file 007 adds, and the only pure one in this crate besides
// `view_model`. Slice 009 adds two more: `instant`, which holds the two
// impure reads `draft` may not make, and `pending`, which holds the
// debounce.
pub mod controller;
pub mod diagnostics;
pub mod draft;
pub mod generated;
pub mod glass;
pub mod install;
pub mod instant;
pub mod pending;
pub mod reception;
pub mod startup;
pub mod view_model;
pub mod wire;
pub mod zoom;

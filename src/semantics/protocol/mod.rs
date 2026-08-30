//! The protocol tier: wire types, canonical types, and the normalization that
//! is the only path between them.
//!
//! `canonical` is the whole of this module in PHASE-02. `wire` and `normalize`
//! arrive in PHASE-03 and PHASE-04; the layout is `design.md` §5.1's.

pub mod canonical;

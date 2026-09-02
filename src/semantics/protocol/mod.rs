//! The protocol tier: wire types, canonical types, and the normalization that
//! is the only path between them.
//!
//! `wire` is permissive and `canonical` is not; `normalize` is the one function
//! that crosses between them, so a canonical value outside this module can only
//! have come from it (P1, I1). The layout is `design.md` §5.1's.

pub mod canonical;
pub mod normalize;
pub mod wire;

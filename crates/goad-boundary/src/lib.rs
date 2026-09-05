//! The workspace's own invariant checks, as a member that depends on no other.
//!
//! It reads manifests and sources as text, which is what lets it scan every
//! stratum without reaching up into one (D17). Three modules: `members` says
//! who to scan, `scan` is the walk and the comment cut, `manifest` is the
//! allowlist.

pub mod manifest;
pub mod members;
pub mod scan;

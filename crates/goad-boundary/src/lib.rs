//! The workspace's own invariant checks, as a member that depends on no other.
//!
//! It reads manifests and sources as text, which is what lets it scan every
//! stratum without reaching up into one (D17). PHASE-02 adds `members` and
//! `manifest`; at PHASE-01 there is one module, because there is no allowlist
//! yet and an empty `manifest.rs` would be a module with no content and no test
//! (PL-9).

pub mod scan;

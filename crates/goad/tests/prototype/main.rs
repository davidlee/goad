//! **Prototype probes** — `docs/slices/009/prototype-notes.md`.
//!
//! Not the §9 validation table, and deliberately not an attempt at it: that is
//! the plan's job and most of the slice's cost. What is here is the mechanism
//! the prototype exists to test — two channels, the epoch, the overlay, I-H's
//! staleness rule, and the answer's one-send flush — each probed at the
//! cheapest tier that can see it.
//!
//! **The tier.** `init_no_event_loop` is what this target installs, so nothing
//! here can watch a `changed` handler run or a `slint::Timer` fire: both are
//! measured absent under it (design.md §9). Every probe below is written to
//! need neither. What needs them is the loop tier, and it is a separate
//! `[[test]]` target for the reason `event_loop/main.rs` states.
//!
//! **Every probe was driven red before it was kept**, by breaking the thing it
//! watches and confirming the failure. A probe that cannot be made to fail
//! pins nothing (`docs/memory/a-green-test-can-assert-a-proxy.md`,
//! `docs/memory/negative-control-must-compile.md`); the charter's P-ids record
//! which break each one answered to.
#[cfg(test)]
mod flush;
#[cfg(test)]
mod harness;
#[cfg(test)]
mod overlay;
#[cfg(test)]
mod split;
#[cfg(test)]
mod staleness;

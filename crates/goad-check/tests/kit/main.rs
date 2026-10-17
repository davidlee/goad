//! Stratum 3 test target — **the kit**, run as a person would run it.
//!
//! What this tier holds is what `kit/` ships: each example is started in a
//! copy of its own directory, under the built checker, in a temporary
//! environment (`design.md` §9, §5.2.6).

// `#[cfg(test)]` on the declaration, for the reason `tests/binary/main.rs`
// gives. Nothing here is `#[path]`-included: the helpers this tier needs are
// a few lines each, and each file it could include carries items this target
// does not use, which `dead_code` fails under `-D warnings`.
#[cfg(test)]
mod examples;

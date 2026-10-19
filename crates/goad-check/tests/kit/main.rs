//! Stratum 3 test target — **the kit**, run as a person would run it.
//!
//! What this tier holds is what `kit/` ships: each example is copied out and
//! the built checker is started outside the copy, on its config, in a
//! temporary environment; every `json` and `toml` fence in its Markdown goes
//! through the host's own doors; nothing in it names a path outside it; and
//! every requirement a refusal can name is explained in the reference
//! (`design.md` §9, §5.2.6).

// `#[cfg(test)]` on the declarations, for the reason `tests/binary/main.rs`
// gives. One file is `#[path]`-included: the fence scanner, shared with
// `goad-shell`'s `integration` target, every symbol in it used by both
// (`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`).
// The other support files carry items this target does not use, which
// `dead_code` fails under `-D warnings`.
#[cfg(test)]
#[path = "../../../../tests/support/fences.rs"]
mod fences;

#[cfg(test)]
mod coverage;

#[cfg(test)]
mod examples;

#[cfg(test)]
mod standalone;

#[cfg(test)]
mod tagged;

#[cfg(test)]
mod tree;

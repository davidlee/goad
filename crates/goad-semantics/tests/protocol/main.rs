//! Stratum 1 test target. `goad-semantics` has no async runtime anywhere in its
//! dependency graph, so naming `tokio` here is a compile error, which is the
//! point.

// `#[cfg(test)]` on the declaration, not for conditional compilation — a
// `tests/` target is always built with `--test`, so this is never off. It is
// there because `clippy::tests_outside_test_module` is `deny` and applies to
// integration targets as well as unit ones; marking the module satisfies the
// lint without a carve-out.
#[cfg(test)]
mod normalize;

#[cfg(test)]
mod runner;

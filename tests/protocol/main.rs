//! Stratum 1 test target. Runs in both feature columns, and in the
//! `--no-default-features` column it runs with no async runtime resolvable at
//! all — naming `tokio` here is a compile error, which is the point.

// `#[cfg(test)]` on the declaration, not for conditional compilation — a
// `tests/` target is always built with `--test`, so this is never off. It is
// there because `clippy::tests_outside_test_module` is `deny` and applies to
// integration targets as well as unit ones; marking the module satisfies the
// lint without a carve-out.
#[cfg(test)]
mod boundary;

#[cfg(test)]
mod normalize;

#[cfg(test)]
mod runner;

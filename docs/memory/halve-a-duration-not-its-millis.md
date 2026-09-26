# Halve a `Duration`, not its milliseconds — `integer_division` sees only the second

Learned at slice 011, PHASE-01 and PHASE-02, writing timed bounds such as
`I/2` and `2I` in `crates/goad/tests/renderer/ingress.rs`.

## The fact

`Cargo.toml` sets clippy's `integer_division` to `deny`. The lint fires on
division of **primitive integers** only. `Duration`'s `Div<u32>` and
`Mul<u32>` are not primitive arithmetic, so `interval / 2`, `interval * 2` and
`interval * 3` are clean. `interval.as_millis() / 2` divides a `u128` and is
denied, in test code too: `integer_division` is not one of `clippy.toml`'s
test-exempt lints.

## How to apply

- Write a derived bound as `Duration` arithmetic — `REFUSAL_PRESENT_INTERVAL / 2`
  — and convert to milliseconds, if at all, afterwards.
- Where integer division is the point (T2(a)'s whole intervals in a span),
  `u128::div_euclid` says so and is not the lint's subject.
- A clippy failure on `as_millis() / 2` is this lint, not a reason for an
  `#[allow]` (`docs/policy/001-the-phase-gate.md` §Compliance forbids that).

Related: `clippy-toml-test-exemptions-are-a-hidden-boundary.md`.

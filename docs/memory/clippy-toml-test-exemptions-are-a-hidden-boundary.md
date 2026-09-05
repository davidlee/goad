# `clippy.toml`'s `allow-*-in-tests` keys are a hidden boundary a file move crosses silently

Learned at slice 002, PHASE-01/PHASE-02, `review-plan.md` F-36.

## The fact

`clippy.toml` sets `unwrap_used`, `expect_used`, `panic`, and
`indexing_slicing` to `deny` workspace-wide, with all four exempted inside
test code via `allow-*-in-tests` keys. Nothing announces this boundary at
the point of use — a `.get()` instead of `[]`, no bare `unwrap`/`expect`,
is only enforced by the *destination* being non-test code.

Moving an item from a test target into a library therefore crosses **all
four** lint boundaries at once, silently, until the gate runs. This is not
one lint tightening — it is four, simultaneously, for every relocated item.

## Why it matters here

The workspace split (PHASE-01) and `goad-boundary`'s further rewrite
(PHASE-02) both moved code out of `tests/` and into a library crate. Each
crossing was confirmed live: the amended EX-5c criterion (`plan-log.md`
PL-13) named `clippy.toml`'s four keys as a fifth class of permitted change
beyond the four literal ones already listed, because a `bytes[i]` that was
fine in test code needed to become `bytes.get(i)` the moment it moved.

## How to apply

- Before moving any code from a test target into a library (or vice versa),
  check it against all four `clippy.toml` test-exemption keys, not just the
  one that happens to be visible in the diff.
- A relocation that trips one of the four is not a redesign — it is this
  boundary being crossed as expected — but the fix (`.get()` over indexing,
  no bare `unwrap`/`expect`/`panic`) still has to land in the same change,
  or the gate goes red.
- When auditing a relocation's diff for "moved unchanged vs. argued", treat
  a lint-driven micro-rewrite at this boundary as an expected, nameable
  class rather than a surprise finding each time.

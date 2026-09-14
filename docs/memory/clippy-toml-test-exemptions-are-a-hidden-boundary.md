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

## The other half: four lints that are *never* test-exempt

Confirmed again at slice 003, PHASE-05 and PHASE-06, directly against both
files. `clippy.toml`'s `allow-*-in-tests` keys cover the four above and **only**
those four. `dbg_macro`, `print_stdout`, `print_stderr` and `use_debug` are
`deny` in `Cargo.toml` with no test carve-out at all.

The practical consequence: a temporary `eprintln!("…{:?}", …)` dropped into a
test as instrumentation trips **two** of them at once, and has to come back out
before the gate runs. `println!` and `dbg!` are the same. There is no exemption
to reach for, and reaching for `#[allow]` instead is what
`docs/policy/001-the-phase-gate.md` §Compliance forbids.

Slice 003 hit this while measuring timed-test margins, and it is why those
margins had to be re-taken in a detached worktree rather than in the tree
(`docs/memory/timed-test-margins-are-measured-at-the-bound.md`).

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
- For temporary instrumentation in a test, expect no exemption: `eprintln!`,
  `println!`, `dbg!` and `{:?}` are all denied everywhere. Instrument in a
  throwaway worktree, or assert on a value instead of printing one.

## A third face: a `tests/` target gets no exemption without `#[cfg(test)]`

Confirmed at slice 005, PHASE-04. The four `allow-*-in-tests` keys apply to
code clippy considers test code, and an integration target under `tests/` is
**not** that by virtue of living there. A `tests/binary/main.rs` holding its
cases directly failed the gate with seven `expect_used` errors plus
`tests_outside_test_module` — on code `cargo test` ran green.

The workspace already knew: `crates/goad-shell/tests/integration/main.rs`
carries the comment saying so. The shape that works is the one that comment
describes — `main.rs` holds the module doc and `#[cfg(test)] mod …;`
declarations, the cases live in the modules beside it. The attribute is never
off (a `tests/` target is always built with `--test`), so it costs nothing and
buys the exemptions.

The trap is the order of discovery: `cargo test` is green throughout and only
`cargo clippy --all-targets` says otherwise. A phase that runs its own tests and
stops believes it is done.

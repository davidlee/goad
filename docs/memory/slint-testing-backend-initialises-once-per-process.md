# The Slint testing backend initialises once per process, so each event-loop topology needs its own test target

Learned at slice 002 (`tests/event_loop/` holding exactly one test) and acted on
at slice 003, PHASE-05 (`design.md` D-12).

## The fact

`i-slint-backend-testing`'s initialiser — `init_no_event_loop()` or
`init_integration_test_with_system_time()` — installs a platform **once for the
process**. A second test in the same binary that wants a different arrangement,
or a second real event loop, does not get one.

Cargo gives one binary per `[[test]]` target, so the unit of isolation is the
target, not the test function. That is why `crates/goad/tests/event_loop/`
contains exactly one test, and why slice 003 added a whole second target,
`crates/goad/tests/event_loop_schedule/`, rather than a second `#[test]` beside
the first.

## Why it matters

Reaching for "just add another test next to it" is the natural move and it
produces a test that passes for the wrong reason, or hangs. The cost of the
right answer is a `[[test]]` entry in the manifest and a `main.rs`, which is
small — but it also means every `pub(crate)` symbol in a `#[path]`-shared helper
must be reachable from the new includer too (see
`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`).

## How to apply

- One event-loop arrangement, one `[[test]]` target. Name the target for the
  arrangement, not for the feature under test.
- Before adding a test to an existing event-loop target, check what that
  target's `main.rs` already initialised. If your test needs a different
  platform or a second loop, it needs its own target.
- A test that must panic-fail from inside the Slint loop should record a value,
  stop the loop, and assert on the test thread. Panicking inside the loop is how
  a broken predicate turns into a hang instead of a failure
  (slice 003 `review-code.md` F-10 — after the repair, a broken predicate failed
  in 5.16 s instead of wedging the gate).

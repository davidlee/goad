# Measure a timed test's margin at the bound that actually governs, not at the test's wall time

Measured at slice 003's audit, 2026-09-08, correcting the figure the slice
itself had flagged as its tightest.

## The fact

Slice 003's `notes.md` handed the auditor a **6.9x** margin as the number most
likely to make a test flaky under load. Measured at the quantity the assertion
actually governs — the elapsed time inside the `until` poll helper, not the
test's total wall time — the same case is **15.2x**, and the worst margin
anywhere in the suite is **10.8x**.

The gap is a process spawn. The recorded span included the child process
starting, which precedes the wait and is not inside the bound.

Measured under five-fold CPU oversubscription (loadavg 164-170 on 32 cores, from
128 busy loops plus a concurrent release build):

| assertion | bound | worst elapsed under load | margin |
|---|---|---|---|
| the flagged case, an instruction from a `respond` | 2 s | 131.8 ms | 15.2x |
| worst of every `until` call in the suite | 2 s | 184.9 ms | 10.8x |

Ten further full-suite runs at that load: 13/13 green every time, 130 results,
zero failures. `just check` exits 0 at loadavg 164.

## Why it matters

A wall-time margin is wrong in both directions and there is no way to tell which
from the number alone. Slice 003 carried **two** recorded figures that were both
wrong — the design's prediction and the phase's correction of it — in opposite
directions. A future slice inheriting either would tune the wrong thing.

## How to apply

- Instrument the helper that owns the bound and print its elapsed time. Do not
  infer a margin from `cargo test`'s per-test wall time.
- Measure under real oversubscription, not on an idle box. A margin that only
  exists when nothing else is running is not a margin.
- To instrument without touching the tree under audit, build a detached
  `git worktree` at the commit, instrument there, measure, and remove it.
- Temporary `eprintln!`/`dbg!`/`{:?}` instrumentation trips lints that are
  **not** test-exempt here — see
  `docs/memory/clippy-toml-test-exemptions-are-a-hidden-boundary.md` — so it has to come back
  out before the gate runs.

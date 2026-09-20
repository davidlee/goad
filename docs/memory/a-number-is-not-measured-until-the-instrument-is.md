# A number is not measured until the instrument is

Slice 009's audit, re-measuring `F-C5`. The first attempt produced a confident,
precise, wrong figure and nearly went into the record.

## The fact

The task was to measure a loop-tier test's total wall-clock run. The instrument:
an `Instant` captured at the start, `elapsed()` printed at the end.

The `Instant` was declared **inside the per-tick closure**. It was therefore
re-created on every tick, and `elapsed()` measured the gap between two
consecutive ticks. It read **90 ns**.

90 ns is not a plausible duration for a test that steps a timer 36 times, and
that implausibility is the only thing that caught it. Had the bug shifted the
figure by 20% instead of by seven orders of magnitude, it would have been
believed, written into a comment, and used to price the next change.

The instrument was rebuilt before any number was believed. The rebuilt one read
~877 ms, which a second independent instrument later confirmed at ~875 ms.

## Why

Measurement feels like observation, so a number that comes out of a machine
inherits the machine's authority. But the number is only as good as the harness,
and a harness is code that nobody reviewed — it was written to answer a question
quickly and it is deleted as soon as the answer is written down.

The failure mode is specific to **closures and loops**: anything captured per
iteration that was meant to be captured once. A clock, a counter, a baseline,
an allocation high-water mark.

## How to apply

- **Sanity-check the magnitude before you trust the digits.** Ask what the
  number *should* be within an order of magnitude, and reconcile if it is not.
  This is the check that works; precision is not evidence.
- **Read the instrument's scope.** What is created once, and what is created per
  iteration? A single misplaced `let` is the whole class.
- **Prefer a second, differently-shaped instrument** over a re-run of the same
  one. Slice 009 ended with two, and their agreement is what makes the figure
  usable.
- Measure a **control** you already know the answer to, where one is available.
- The same rule as `a-negative-control-that-does-not-compile.md`, one step
  earlier in the pipeline: there, the control has to build before its result
  means anything; here, the clock has to be in the right scope.

Related: `a-performance-claim-in-a-comment-is-a-measurement-claim.md`,
`a-negative-control-that-does-not-compile.md`,
`timed-test-margins-are-measured-at-the-bound.md`.

# Rank a timed bound by which way load moves it, not by the size of its margin

Slice 009, `F-B4` and `F-C5` — where the bound with a **40x** nominal margin was
the one that failed, and the bound with **1.33x** was the one that was fixed.

## The fact

A timed test has a margin, and the margin's *size* is the obvious thing to rank
bounds by. It is the wrong thing. What decides whether a bound fails under load
is which **direction** load moves it.

Two bounds in one file, opposite shapes:

- **A bound the work must fit inside** — *step 9 to step 13 must exceed the
  150 ms debounce* — runs at 200 ms, a **1.33x** margin. Load makes the steps
  take *longer*, which pushes the measured interval **further above** the
  requirement. Load helps it. It was still fixed, because 1.33x is tight against
  scheduling jitter in either direction.
- **A liveness backstop** — *all thirteen readings within `LIVENESS_BOUND`* —
  had a **40x** nominal margin and is the one that actually failed. Load makes
  the stepper stall for tens of seconds, and the bound converts the stall into a
  red that **reads like a defect in the code under test**.

Three loop targets failed at roughly 6x CPU oversubscription, every one of them
on the backstop and none on an assertion.

## Why

A margin ratio is computed at rest and describes the distance to the bound *on
an idle machine*. Load does not scale all durations equally — it stretches
wall-clock waits and scheduler hand-offs enormously while leaving pure
computation nearly alone — so the ratio is not a prediction of anything.

The backstop is the worst case because its failure is **indistinguishable from
the failure it exists to report**. A stalled stepper and a hung system under
test produce the same red.

## How to apply

- **For every timed bound, write down which way load moves the measurement**,
  and put it in the comment beside the bound. That sentence is worth more than
  the ratio.
- **Widening a backstop is not the repair.** 40x was not enough; 400x would
  only move the machine at which it breaks. The repair is a harness that does
  not depend on wall-clock progress of a UI timer.
- **A load-sensitive gate is a real defect**, not a flake, and belongs in the
  ledger — it sits against a policy that says the gate exits 0. Slice 009 had
  two independent witnesses: a `just check` failing at loadavg 198, and a
  reviewer reproducing it under a controlled batch without being told.
- **Measure the whole run, not only the intervals.** A re-index that held every
  interval can still grow the total past a spacing bound — and the total was the
  bound that had actually been seen to fail.

Related: `timed-test-margins-are-measured-at-the-bound.md`,
`a-bound-is-not-tested-at-the-bound.md`,
`a-performance-claim-in-a-comment-is-a-measurement-claim.md`.

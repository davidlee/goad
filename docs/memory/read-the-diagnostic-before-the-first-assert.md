# Read a failure's diagnostic before the first assertion that can panic

Learned at slice 011, PHASE-03 (`docs/slices/011/notes.md` §Harvest).

## The fact

A quantity a test computes only after an assertion is lost on exactly the run
that most wants it: the one where the earlier assertion fails. In slice 011,
T2's per-firing gaps are checked in (b), after (a)'s count assertion. When
PHASE-03 instrumented a scratch copy to measure M9 (the production interval
shortened), M9 redded (a) first, so gaps printed inside (b) were never
reached. The instrumentation had to compute the gaps ahead of (a) to see the
per-firing lag on the runs that failed.

## How to apply

- In a case with several assertions over one record, derive every quantity the
  assertions share, or a failure message needs, before the first `assert!`.
- Put the derived values into each assertion's message, so the red run carries
  its own diagnosis without instrumenting a scratch copy.
- When instrumenting a scratch copy to measure a control, place the print
  ahead of the first assertion the mutation can red, not beside the assertion
  it belongs to.

Related: `timed-test-margins-are-measured-at-the-bound.md`,
`a-number-is-not-measured-until-the-instrument-is.md`.

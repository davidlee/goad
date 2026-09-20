# A review trend ends in mechanical verification, not in another round

Slice 009's audit ran four rounds of `review-code.md`, each one reviewing the
previous round's repairs. This is how it stopped, and why it did not run a
fifth.

## The fact

Each round reviewed repairs nobody else had looked at. The counts are not the
signal — the **shape** is:

| round | findings | what they were |
|---|---|---|
| 1 | 21 | one blocker and six majors of **live defect** |
| 2 | 13 | two majors about **what holds a repair** |
| 3 | 6 | one live defect, one coverage gap, **four claims wrong in prose** |
| 4 | 7 | **no behavioural defect at all** — seven claims wrong in prose |

Round 4's seven were: two counts, a step number, a wrap width, a uniqueness
claim, a cost priced by a mechanism that does not exist, and a citation class.

Repairing those seven writes new prose that nobody reviews — which is exactly
how the citation class regenerated across three rounds. So the question *"what
checks the repairs?"* is real, and *"nothing"* is the option that had produced a
finding every time it was taken.

**A fifth review round was not the answer.** Every round-4 finding is
script-checkable: recompute the counts by `grep`, resolve the citations by
script, recompute the interval arithmetic from the step arms, check the column
widths with `awk`. And every one of them is a thing a *reading* agent had
already got wrong at least once — including the reviewers.

Total cost of the mechanical pass: about thirty minutes, plus one `just check`
that the repairs needed anyway. Cost of a fifth agent round: a session.

## Why

Review rounds are the right instrument while findings are about **behaviour**,
because behaviour needs a mind to model it. They are the wrong instrument once
findings are about **the record** — counts, locations, arithmetic — because
those are precisely what a mind is bad at and a script is perfect at.

Running another round when the trend has crossed that line is not caution. It is
applying the expensive instrument to the class it is worst at, and the evidence
is in the ledger: three separate agents miscounted or mis-cited while reading
carefully.

## How to apply

- **Watch the composition of findings, not the count.** Round 3's count was not
  zero and the slice still moved to a verification pass, because the defects
  were gone.
- **When a round returns no behavioural defect, stop reviewing and start
  checking.** Name the instrument for each finding before you decide; if they
  are all greps, the decision is made.
- Write the script even for a one-off. The citation resolver was twenty lines,
  found four times what a careful reading had found, and re-runs in a second.
- **A one-off script in a scratchpad is not a gate instrument** and does not
  amend `POL-001`. Keep that distinction: it is what makes the cheap option
  available without a policy change.

Related: `enumerate-the-class-not-the-instances.md`,
`an-unmeasured-residue-is-not-a-small-residue.md`,
`cite-by-symbol-not-line-number.md`.

# A cost written in a comment is a measurement claim, and nothing checks it

Three instances in one audit — slice 009's `F-C5`, `F-D4` and `F-D6` — all the
same shape: a comment pricing something by a mechanism that does not exist.

## The fact

Comments about cost read as engineering judgement and are treated as settled.
They are assertions about runtime, and no compiler, lint or gate step reaches
one. The three:

- **`F-B4`'s repair claimed a re-index cost nothing.** True of the intervals it
  had held; false of the total, which grew 799 → ~877 ms.
- **`F-C5`'s correction then explained the growth backwards.** It said the
  re-index kept every interval the same and *still* made the run longer. Two
  intervals had changed, and one of them — `B → C`, stretched 150 → 225 ms —
  **was** the whole +75 ms. The run got longer *because* of it.
- **`F-D6`: a guard priced as free.** The doc said an element-wise comparison
  was *"the same work `set_vec` would do allocating the replacement"*. `set_vec`
  is a move plus `notify.reset()`: it allocates nothing and touches no element,
  so the comparison is work **added**, not shared — and the sentence's
  conclusion, that the guard costs nothing on the writing path, is wrong in the
  same step.

Each was written to correct the one before it.

## Why

A cost claim has two halves — the number, and the account of where it came from
— and they fail independently. The number is checkable by re-running something.
The **account** is checkable only by someone re-deriving the mechanism, which
nobody does, because the number beside it looks like evidence for both.

`F-D6` is the purest case: no number at all, just an appeal to a mechanism
(`set_vec` does element-wise work) that was never true. It survived a review
round because it is exactly the kind of sentence that sounds like it came from
reading the source.

## How to apply

- **State the instrument, not just the figure.** *"~877 ms"* is unfalsifiable
  prose. *"876.9 / 877.2 / 878.3 over three runs on an idle machine"* can be
  re-run and disagreed with.
- **Keep two independent measurements rather than choosing one.** Slice 009 ended
  with 876.9 / 877.2 / 878.3 from one instrument and 876.1 / 875.6 / 874.9 from
  another, and recorded **both with their sources named**. Two instruments
  agreeing to 0.4% is a stronger statement than either alone, and a reader who
  re-measures and gets 875 now finds that expected instead of a discrepancy.
- **Never carry a figure that appears in none of your runs.** One block quoted
  three measurements and then instructed the reader to price future changes
  against a fourth number, lower than all three — a survivor of an earlier draft.
- **If the comment explains *why* a cost is what it is, the explanation is a
  separate claim.** Derive it from something in the file — the step arms, the
  constant — so the next reader can check it without a profiler.
- Prefer a cost claim the code can hold: a `const _: () = assert!(…)`, or a
  bound the test itself asserts.

Related: `a-count-in-a-comment-is-a-claim-nothing-checks.md`,
`timed-test-margins-are-measured-at-the-bound.md`,
`a-number-is-not-measured-until-the-instrument-is.md`.

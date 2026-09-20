# Cite from an instrument that prints the number, never from a hand count

Five bad citations in slice 009 — and the fifth was written by the reviewer who
was verifying the fourth.

## The fact

Every wrong `path:line` in that slice was counted by hand off a
`sed -n 'a,bp'` window. Every citation taken from `grep -n` or `awk NR` has
held.

That is the whole pattern. It is not about who made the mistake — an earlier
version of this lesson said *"verify the responder's citations first"*, which
described the people rather than the cause. The cause is the **method**: a hand
count off a window is not checkable at a glance, so its being right is luck, and
luck does not survive five attempts.

## Why

`sed -n '280,300p'` prints twenty lines with no numbers on them. To cite one you
count rows in your head, and you are counting in the same breath as forming the
sentence the citation supports. Nothing about the result looks uncertain
afterwards — a line number is a line number.

`grep -n` and `awk` print the number **with** the line. There is no counting
step, so there is nothing to get wrong, and the transcript shows the number
beside the text that justifies it.

## How to apply

- **`grep -n` for the thing, then cite what it printed.** If the target has no
  distinctive text to grep for, `awk 'NR>=280 && NR<=300 {print NR": "$0}'`
  gives you a numbered window.
- Never cite a location you read with `sed -n 'a,bp'`, `head`, or an editor
  view, without re-deriving it from a numbering tool.
- Better still, **cite the symbol** and skip numbers entirely — see
  `cite-by-symbol-not-line-number.md`, which is what slice 009 eventually
  adopted after this rule alone failed to stop the class.
- The same shape applies to any count that ends up in prose: get it from
  `grep -c` or a length assertion, not from reading a list.

Related: `cite-by-symbol-not-line-number.md`,
`a-count-in-a-comment-is-a-claim-nothing-checks.md`,
`a-number-is-not-measured-until-the-instrument-is.md`.

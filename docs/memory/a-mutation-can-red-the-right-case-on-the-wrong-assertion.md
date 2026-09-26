# A mutation can red the right case on the wrong assertion — check which assertion, not only which case

Learned at slice 011, PHASE-02 (`docs/slices/011/notes.md`, the M8 row and its
Findings; `design-log.md`, *M8's red lands on R1; M8b added*).

## The fact

A mutation table that names only the case a mutation reds can pass a control
that does not hold the claim it was written for. Slice 011's M8 ("the top
present also resets the refusal deadline", the design's rejected alternative
D5) was predicted to red T3 on its **second** refusal's bound. It redded T3 on
the **first**. T3's pin was an accepted envelope, and that exchange's own
present moved the deadline before the first refusal was sent. The case was
right; the assertion it failed on was a fact about the test's pin, not about
D5. The control D5 needed, M8b (the deadline reset at the one exit a person's
no-exchange command takes), had to be added and redded the intended bound
alone.

A second face of the same fact: a mutation of a **global** knob — the
production interval, or the arm's own re-arm period — reds every case that
leans on it, not only its named case. That is expected, not a defect.

## How to apply

- Record, for each mutation, the **assertion** that went red and its message,
  not only the case. Compare it with the one the design names.
- If the red lands on a different assertion, find out whether the test's own
  setup (its pin, its fixture's first present) is what fails, before accepting
  the control. If it is, spell a second mutation at the site where the
  rejected behaviour can actually occur.
- For a global knob, list every case it reds, row by row, rather than
  narrowing the mutation until only one goes red.
- Do not retro-fit the design's label mid-slice. Record the drift for audit.

Related: `a-green-test-can-assert-a-proxy.md`,
`a-negative-control-that-does-not-compile.md`,
`a-repair-can-be-wrong-about-what-it-holds.md`.

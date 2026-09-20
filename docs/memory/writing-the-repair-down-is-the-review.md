# Writing the repair down is itself a review, and it finds a different class

Observed across slice 009's prototype and audit: the agent *integrating* a
finding found the blocker in three consecutive rounds, and the finding was of a
class the reviewers were not looking for.

## The fact

There are two jobs in a review loop, and they fail differently.

- A **reviewer** reads the artefact adversarially and looks for what is wrong
  with it. Reviewers find defects of **reasoning**: an unheld property, a case
  that asserts a proxy, a mechanism that does not work.
- An **integrator** writes the disposition and the Response — states what the
  repair will do, and why it closes the finding. Integrators find defects of
  **coherence**: two findings that contradict each other, a repair that would
  undo an earlier one, a claim in the finding that cannot be written down
  without noticing it is false.

The second class is invisible to the first, because a reviewer never has to make
the pieces agree in one paragraph. Writing the Response is what forces that.

## Why

Prose is a consistency checker. To write *"this repair closes the finding
because X"* you have to hold the finding, the code, and the other findings in
one sentence — and an inconsistency that survives reading will usually not
survive being written down.

Slice 009's clearest instance runs the other way and proves the same point:
`F-B9`'s Response **named the case that would have caught its own residue** and
declined to write it. The writing got the agent to the question; only running
the case answered it.

## How to apply

- **Keep the roles separate and say which you are wearing.** The ledger's
  Protocol is explicit: one agent may hold both, but must switch deliberately —
  disposing a finding while still wearing the raiser's hat is how a review talks
  itself into `aligned`.
- **Write the Response before you write the code.** If the paragraph is hard to
  write, the repair is wrong or the finding is.
- **Treat a Response that hedges as an open item**, not as a disclosure that
  discharges itself.
- **Disposition findings together, in one pass**, rather than one at a time.
  Contradictions between findings only show up side by side.

Related: `a-repair-can-be-wrong-about-what-it-holds.md`,
`audit-stage-needs-its-own-budget.md`,
`verify-the-enumeration-not-the-conclusion.md`.

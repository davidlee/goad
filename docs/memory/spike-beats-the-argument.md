# Build the thing instead of reasoning about it one more round

Slice 009's scoping produced a spike instead of a design section, and the spike
**overturned the framing it was built to confirm**.

## The fact

The re-present problem was scoped as a second design surface — a whole parallel
question about how a form should be rebuilt. A throwaway spike, written to
confirm that framing, showed it was **one decision**: `set_vec` destroys every
row element, `set_row_data` does not, and a guarded write in place solves the
whole of it.

No amount of further argument would have produced that, because the load-bearing
fact was a property of somebody else's library.

The same shape recurs whenever the question is *what does this actually do?*:

- Whether a test tier could reach inside a Slint popup — doubted in the design,
  measured as **yes** for queries and **no** for pointer events, which is a
  different and much more useful answer than either side of the argument.
- Whether a guard was held by any case — settled by deleting it and running the
  suite, in minutes, after a round of reasoning had not settled it.
- Whether a repair closed a finding — settled by the case the Response had
  named and declined to write.

## Why

An argument between two agents converges on whichever model is better
*articulated*, and neither model is the system. A spike is a measurement of the
system, and it routinely returns a third answer nobody was arguing for.

It is also cheaper than it looks. A spike is throwaway by construction: it does
not need tests, naming, or a place in the architecture.

## How to apply

- **When a design question turns on what the code or a dependency does, build
  the smallest thing that answers it**, before the next round of argument.
- **Commit the spike, then delete it in the commit that closes the design** —
  so the measurement survives in history and the tree stays clean. Slice 009's
  lives at `4f93d41`, a standalone cargo project rather than a workspace member,
  and the design cites it.
- **Injection-pass it.** A spike that agrees with you and cannot be made to
  disagree has measured nothing.
- **Say which framing it overturned.** The valuable output is usually not the
  answer but the discovery that the question was shaped wrongly.
- For **interaction** questions specifically, the spike is a person running the
  software: see `settle-a-usability-question-by-running-it.md`.

Related: `settle-a-usability-question-by-running-it.md`,
`price-the-rejected-option-against-code.md`,
`a-present-destroys-the-widget-it-writes.md`.

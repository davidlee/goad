# Never rule an option out on an estimated cost — the real reason is usually behavioural

Slice 009, repeatedly: every option rejected in this slice was rejected for what
it would *do*, not for what it would cost, and the two times cost was reached
for, the estimate was wrong.

## The fact

When several repairs or designs are on the table, the tempting way to choose is
to estimate effort. That estimate is the least reliable number in the
conversation, and it hides the real reason.

The options slice 009 rejected, and why:

- **Suppress the busy present** rather than narrow `busy` — rejected because it
  fixes the flash and leaves the **deafness**, now invisible until a slow
  backend. A behavioural reason.
- **Move the present** rather than drain the command queue — rejected as *the
  same effect in a less obvious form*.
- **Hold the entry until the edit is served** — rejected because it reverses a
  phase's exit criterion, which is the only reason a function returns a `bool`.
- **A `commands` arm in the inner `select!`** — rejected because it reopens
  re-entrancy the design closed.
- **A markup scan** to hold the re-assert counter — rejected on its **canon**
  cost: a fifth boundary instrument in a policy that enumerates them.
- **Deriving a constant** instead of asserting it — rejected because derivation
  rescales silently.

Not one of those is "too expensive." Each names a consequence.

## Why

An effort estimate is unfalsifiable at the moment it is made and embarrassing
later. Worse, it **terminates the discussion**: once an option is "too big",
nobody states the real objection, so the record carries no reason a future
reader can check or overturn.

And the estimate is often simply wrong in the cheap direction too. Slice 009's
`F-A1` repair — reasoned about as large — turned out to be one flag read in
exactly one place with one production call site.

`docs/AGENTS.md` says it outright: **do not defer a fix merely because it is
large**.

## How to apply

- **State the consequence, not the size.** *"This reverses PHASE-05/EX-4"* beats
  *"this is a big change."*
- **Where cost genuinely is the reason, measure it** — grep the call sites, read
  the function — and say what you measured. Slice 009's audit repeatedly
  *verified before pricing* and it changed the answer seven times.
- **`follow-up` is for a different unit of work, not for a large one.** Every
  follow-up in slice 009 says which: a spec amendment, a harness, a design
  change that would break an invariant.
- The converse holds too: don't rule an option **in** on an estimate. See
  `an-unmeasured-residue-is-not-a-small-residue.md`.

Related: `an-unmeasured-residue-is-not-a-small-residue.md`,
`verify-the-proposed-instrument.md`, `spike-beats-the-argument.md`.

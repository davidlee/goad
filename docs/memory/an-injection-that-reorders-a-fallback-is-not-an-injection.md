# An injection that reorders a fallback is not an injection

Slice 009, and the first attempt reported **1 passed** and looked like evidence.

## The fact

To prove a case would catch a defect, you break the production code and expect
red. The break has to actually remove the behaviour.

The instance: the code was `overlay.or(drafted)` and the case was meant to hold
*the overlay is consulted*. The injection written was `drafted.or(overlay)` —
which falls back to the overlay **precisely when the draft is empty**, the only
situation that matters. The behaviour was intact, the case passed, and the pass
was nearly recorded as "the defect does not reproduce."

**Injecting into an `or` chain means dropping a term, not moving it.**

## Three more shapes from the same slice

- **An injection can redden the wrong claim.** Removing `pending.rs`'s enqueue
  clear looks like the control for *the entry has left the map*, and it fails
  the **delivery** assertion instead — with nothing removed, `tick`'s
  `.iter().next()` returns the same first key forever and the second entry is
  never sent at all. An injection that reddens *a* claim is not evidence for
  *the* claim it was aimed at. Read the failure message, not the exit code.
- **A seed and the value it produces cannot be attacked separately.** An
  injection to the clock a picker opens on changed the *recorded* value before
  it changed what another field's picker showed, and failed a different
  assertion. Isolating "this field's seed, not that one's" needed the **slot**
  the handler reads to be redirected instead.
- **A green injection can be the correct answer, and has to be said out loud.**
  Planting one kind for every drawn kind left all 189 renderer cases green,
  because nothing could yet construct a drawn field of another kind. That is a
  true state of the world — but it is indistinguishable at a glance from a case
  asserting a proxy, so a phase that runs an injection and gets green owes the
  record a sentence saying which of the two it is, and which phase measures it
  first.

## How to apply

- State, before running it, **what behaviour the injection removes** — not what
  line it edits. If you cannot name the removed behaviour, the injection is a
  perturbation, not a control.
- Expect a **specific** failure, and check you got that one.
- A control must **compile**; see `a-negative-control-that-does-not-compile.md`.
  A control that fails to build greps identically to a passing one.
- **Revert against what you read, not against `git`.** `git diff --stat`
  compares to `HEAD`, which in the middle of a phase reports the phase's own
  work as an unreverted injection, every time. Copy the file's bytes out before
  the injection and compare them back after. And never revert with
  `git checkout -- <path>`: slice 009's prototype runner did exactly that
  against an uncommitted tree and reverted a whole phase.

Related: `a-negative-control-that-does-not-compile.md`,
`a-green-test-can-assert-a-proxy.md`,
`git-stash-forbidden-recover-read-only.md`.

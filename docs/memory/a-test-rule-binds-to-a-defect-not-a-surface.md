# Bind a test rule to the defect it must catch, not to the surface it reads

Learned at slice 007 — design §9's closing rule, and PHASE-04's cases, which
breach its letter and serve its purpose.

## The fact

Slice 007's design stated the rule that keeps its field tests honest as a rule
about **which surface** a test reads:

> Every field test either reads the wire or asserts something about the screen —
> except AC-5, which needs both.

`plan.md` restated it as a STOP condition. Then `plan.md`'s own sequencing put
PHASE-04's verification at `answer()`'s return value — neither the wire nor the
screen — while citing that same section as its authority. Both documents were
right about what to build and the rule was wrong about why.

The rule's *stated purpose* is given two sentences earlier: exclude a test that
reads the draft through `Controller` and stops, because such a test stays green
with `answer()` walking the draft's keys instead of the declared fields. That
purpose is a claim about a **defect**. PHASE-04's cases read neither wire nor
screen and they catch that defect — walking the draft reddens them — so the
rule's letter excludes tests its purpose wants.

## Why

A surface is a proxy for what you actually care about, and a rule written over a
proxy fails in both directions. It admits a test that reads the right surface
and asserts nothing that discriminates — which is exactly
`a-green-test-can-assert-a-proxy.md`, four green tests in slice 004 each
asserting something the regression they guarded would survive. And it excludes a
test that reads a different surface and discriminates perfectly.

Naming a surface is tempting because it is checkable by reading, and "does this
catch the defect?" is not. But the cheap check is checking the wrong thing.

## How to apply

- State a test rule as **"would this test go red if <the specific defect>
  happened?"** — not "does this test read the wire?". Name the defect; a rule
  whose defect you cannot name is not yet a rule.
- The way to *discharge* such a rule is to **make the defect and watch the test
  fail**. Delete the element, invert the binding, replace the lookup with a
  constant. Slice 007's audit review found three unheld claims that way and
  every repair was verified the same way. Revert the mutation precisely
  afterwards; never `git stash`.
- Beware assertions in the direction that cannot discriminate. An unbound Slint
  `enabled` answers `Some(true)`, the same as a bound one at rest, so every
  `== Some(true)` assertion in the workspace was blind to the binding being
  deleted. **Assert the state only the live mechanism can produce.**
- A surface rule is still useful as *guidance* — reading the wire usually is
  stronger. Keep it as advice; do not make it the gate.

Related: `a-green-test-can-assert-a-proxy.md`, `an-invocation-count-is-not-an-absorption.md`.

# Design log — Slice 009

Append-only, time-ordered. What was asked, what the user decided, why. Never
rewritten; superseded. Findings live in a ledger, not here.

## 2026-09-16 — scoping

**D-1 — Close 008 before opening this.** Asked because 008 was still
`Stage: in progress` with an open list. Decision: close it, carry the remainder
into its follow-ups and the roadmap, and open 009 against a clean record.
Done at `239e0c5`.

**D-2 — Draw all four undrawn kinds, not three.** The ask named datetime,
number/slider and dropdown; `text` is the fourth. Decision: all four, which
discharges the standing hazard slice 002 recorded rather than leaving a gap with
no argument behind it. `Undrawn::FieldForm` stays as the guard for a sixth kind.

**D-3 — Tier 2, on size rather than canon.** Two canon candidates were offered
and the question was declined at the time it was asked, on the ground that the
approach was not yet clear enough to size — *"I don't think we have enough to
know the tier size yet."* Reaffirmed after the spike. Neither `step` nor
SPEC-001/OQ-4 is taken; the tier comes from the design surface. `slice-009.md`
§Why tier 2.

**D-4 — A configurable per-backend text debounce, 150 ms for now.** User:
*"I expect we ultimately want a configurable debounce for text fields per
backend, but we can roll with eg 150ms for now."* So the value is a starting
point and not a constant to be argued over, and the configuration surface is
explicitly deferred rather than forgotten. What design still owes is the flush
point — OQ-3 — because a debounce means the draft lags the widget.

**D-5 — Spike the unknowns rather than design against them.** User: *"let's
spike out the unknowns rather than dive into design nitpicking."* Taken at the
point where the sizing question could not be answered from reading alone. It
overturned the framing it was built to confirm: the re-present problem had been
priced as a second design surface and is one decision. `research.md` Thread 3
carries what it measured, Thread 4 what it ruled out.

### A correction recorded, because it changed the slice's shape

The scoping agent framed "the form must survive a present" as a problem of
**value** persistence across four field kinds. The user rejected the premise —
*"it doesn't survive. It gets regenerated. If it's supposed to persist, the
backend passes it back as a value. What am I missing?"* — and was right:
`glass.rs::option_rows` builds every row from the draft, so values already
survive, and persistence across a round trip is `field.value`/OQ-2 and the
backend's.

What is actually lost is **interaction state** — caret, drag grab — and only for
`text` and `number`. The reframing is what made the problem small enough to
spike, and the spike is what made it one decision.

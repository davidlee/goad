# ADR-004: Scheduled firings are spaced from the previous scheduled firing

**Status:** accepted
**Date:** 2026-09-08

## Context

SPEC-001/R-28 requires the host to store a backend's scheduling instruction as
given, including one that has already passed. SPEC-001/R-26 returns the latest
valid instruction verbatim, so a retained instant is *replaced* by each new
instruction rather than consumed by having elapsed. Put together, a backend that
answers every exchange with an instant in the past never lets the host reach the
arm that would return it to its default cadence: the wait is zero, the host
fires, the response instructs the past again, and the wait is zero again. That is
an unbounded exchange loop — a busy loop in which the host spawns a process as
fast as the transport allows — and it is reachable by a backend that is merely
wrong, not malicious. Slice 001 raised it three times at audit (F-1, F-34, F-48)
and slice 003 is the first slice in which it can actually happen, because slice
003 is the first in which a resolved instant makes the host act.

The host cannot answer this by adjusting the instruction. R-28 forbids it, and
adjusting it would also destroy the one piece of evidence a person debugging a
backend needs. So the bound has to be on the host's own firing, which is a
different fact about a different subject, and the two must not be conflated.

That leaves the question this record exists for: spacing measured *from what*.
Three answers were available, and they are indistinguishable today. They stop
being indistinguishable in slice 004. The roadmap places external event ingress
there, and `docs/slices/003/slice-003.md` §Non-goals states the relationship
outright: a scheduled evaluation and an ingested event are two stimuli into one
path. An anchor defined as "the last thing the host did" is equivalent to "the
last scheduled firing" only while every other stimulus is a person, and a person
is rate-limited by being a person and by a capacity-one command channel. An
event source is neither. `docs/slices/003/review-design.md` F-2 is the finding
that named this: an event arriving at machine rate would clear the spacing before
every scheduled firing, and the busy loop the spacing exists to prevent would be
back, in a slice that had no reason to reopen a rule stated without its premise.

## Decision

We will bound the host's own scheduled firing rate with a **minimum spacing of
3 seconds**, measured from **the previous scheduled firing**, on the **monotonic**
clock the pending deadline already uses. Precisely:

- The spacing binds **scheduled firings only** — those the host begins of its own
  accord when a resolved next check comes due. It never delays an evaluation a
  person asked for, and never delays the host's startup evaluation.
- The anchor is the previous **scheduled** firing, not the previous evaluation of
  any kind, and not the previous thing the host did. **Nothing clears it**: no
  other stimulus, of any kind, present or future, resets the anchor or grants a
  firing that would otherwise be too soon.
- A scheduled firing the host refuses — for an unreadable clock, say — is still a
  scheduled firing for the purpose of the anchor, so a refusal cannot spin.
- The spacing is a constant of the host. It is not configurable, and no backend
  can read or influence it.
- It **adjusts nothing the host stores or reports**. SPEC-001/R-28 is untouched:
  what the host holds and reports is the instruction, and what the spacing moves
  is when the host acts on it.

The constant lives in stratum 3, beside the loop that applies it, and the pure
wait arithmetic in stratum 1 takes no spacing argument.

The rule is normative as SPEC-002/R-4, R-5 and R-6. This record holds the part a
requirement cannot: *why the anchor is what it is*, so that the slice which adds
a second stimulus has to argue with this document rather than inherit a rule
whose premise it is about to retire.

## Alternatives considered

- **Space every evaluation, a person's included.** Simplest, and it bounds the
  hazard. Rejected because it makes the host feel broken: a person choosing
  *Check now* would sometimes wait up to three seconds for no reason they can
  see, and the host's entire purpose is interaction.
- **Space only when the predecessor was itself a scheduled firing, tracked as a
  boolean.** Behaviourally identical to the chosen rule today, and it reads more
  naturally. Rejected on `review-design.md` F-2: a boolean cleared by "some other
  stimulus happened" states a rule about *other stimuli*, and its correctness
  rests on the premise that every other stimulus is human. Slice 004 retires that
  premise. The anchor makes no claim about other stimuli at all, which is what
  lets it survive.
- **Anchor to a retained wall-clock instant.** Rejected: a wall-clock anchor is
  exposed to a backwards clock jump, which would hand back the unbounded loop for
  as long as the jump lasts. The monotonic clock is the one the pending deadline
  already uses, and using it gives the spacing exactly one write site.
- **Make the spacing configurable.** Rejected: a bound a misconfiguration can
  remove is not a bound, and there is nothing a person could usefully set it to.

## Consequences

### Positive

- The busy loop is bounded by construction, whatever the backend does, and the
  bound is the only thing standing between the host and that outcome.
- The bound survives slice 004. Adding event ingress cannot weaken it by
  accident, because no stimulus clears the anchor. How an *ingested* event is
  bounded is a separate question that SPEC-002/R-5 forces slice 004 to answer in
  its own terms rather than inherit.
- Storing and reporting stay separate from firing, so the spacing coexists with
  SPEC-001/R-28 instead of contradicting it.
- One monotonic anchor means one write site for the floor, which is what makes
  the rule reviewable at a glance.

### Negative

- A configured `default_poll` shorter than three seconds is honoured for the
  process's first scheduled firing and silently spaced thereafter. Nothing tells
  the person their configuration is being bounded, and nothing tells the backend
  either — SPEC-002/OQ-2 carries that question.
- What the host reports as its next check is the instruction it holds, which is
  not always when it will fire. The spacing is one of three states in which the
  two differ (SPEC-002 §6, OQ-3).
- Three seconds is a judgement, not a measurement. It is long enough that a
  process spawn per interval is negligible and short enough to be invisible in
  ordinary use, but no evidence fixes it at three rather than two or five.
- The spacing's *necessity* is held by argument and by one break-and-revert
  experiment, not by a standing test. Deleting the constant would be caught by
  the anti-spin windows in the suite; deleting the *reason* would not be.

### Neutral

- The constant sits in stratum 3 rather than stratum 1. Three seconds is a host
  operational budget, like the default poll, the backend timeout and the
  transport's cleanup budget, all of which live above stratum 1; and the anchor
  makes the placement forced as well as tidy, since the floor is a comparison
  against a monotonic instant, a type stratum 1 cannot name. None of ADR-001's
  four instruments sees a policy constant placed downward, so the placement is
  held by argument rather than by scan.

## Verification

The rule is verified where SPEC-002 §7 says it is: a backend instructing the past
on every response is invoked a bounded number of times over a window far shorter
than the spacing, and a person acting in the middle of that cadence does not
raise the count. Both are integration tests in
`crates/goad/tests/renderer/scheduling.rs`.

The *anchor* — as against the spacing itself — **is now verified.** Slice 004
introduced the second stimulus this record was waiting for, and with it the one
situation in which the anchor and the boolean alternative disagree:

> `crates/goad/tests/renderer/ingress.rs::an_ingested_firing_does_not_advance_the_scheduled_floor`

A scheduled firing at T₀, an ingested firing at T₀+ε **whose own exchange
resolves to a deadline no later than T₀+1 s**, and a `next_check` due at T₀+1 s.
Under the anchor the scheduled evaluation waits until T₀+3 s; under a boolean
cleared by *some other stimulus happened*, the intervening ingested firing clears
the flag and it fires at T₀+1 s. The case asserts no scheduled evaluation reaches
the backend before T₀+3 s. That middle clause is what makes it this case rather
than a weaker one: an ingested exchange re-arms the pending deadline from what
its own backend answered (SPEC-001/R-26), so unless it resolves at least as
short, the deadline in force at T₀+1 s is one the anchor and the boolean agree
about and the test proves nothing.

Two sibling cases in the same file are **not** this discharge, and are named so
they are not mistaken for it. `::an_ingested_firing_never_writes_the_scheduled_floor`
falsifies the third alternative below — an anchor on *the last thing the host
did*. `::a_scheduled_firing_does_not_clear_the_event_floor` holds the **event**
anchor SPEC-002/R-12 adds, about which this record makes no claim.

What remains held by **review** is narrower than before, and is the part that
cannot be tested because its subject does not exist: the *choice* of an anchor
over the boolean for stimulus classes a host has yet to acquire. SPEC-002 §3 P-E
is where that generalisation now lives.

## References

- SPEC-002 (the host's scheduling behaviour) — R-4, R-5, R-6 and §3 P-B, P-D,
  which state the rule normatively.
- SPEC-001 (the host/backend interaction protocol) — R-26, R-28 and R-29, the
  resolution and storage rules that make the hazard reachable.
- ADR-001 (one-way strata) — why the constant is stratum 3's and the arithmetic
  is stratum 1's.
- `docs/slices/003/design.md` §7 D-3 (the anchor) and D-14 (the placement);
  §4 P-2 and P-2a.
- `docs/slices/003/review-design.md` F-2 — the finding that the boolean
  alternative's premise expires in slice 004. It did, and the expiry is the
  Verification section above.
- `docs/slices/004/design.md` §5.3 (the two anchors and how they are kept
  independent) and §9 (AC-6, whose case (ii) is the discharge above).
- `docs/roadmap.md` §004 — the slice that adds the second stimulus.

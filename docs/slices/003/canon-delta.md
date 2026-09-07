# Canon delta — Slice 003

**Status: proposed.** Nothing here is applied. Changes are applied during audit
and reconciliation, with explicit user endorsement, and recorded in `audit.md`'s
Reconciliation table.

Changes this slice makes to canon that **already exists**. New canon is drafted
in `draft-spec.md`, not here — SPEC-002, the host's scheduling behaviour, is
this slice's new document, and the reason the entries below are as small as they
are.

One entry per affected document and section: the document, the section, the
change as it will be stated, and why.

---

## CD-1 — SPEC-001 §4, Requests: a new requirement R-56

**Document:** `docs/specs/001-host-backend-protocol.md`
**Section:** §4 Requirements → Requests
**Kind:** new requirement. Ids are immutable and appended; R-55 is the current
highest.

### Why

R-7 requires an `evaluate` request to carry an event with a source and a kind,
and says nothing about their values. That was sufficient while every evaluation
meant the same thing. This slice makes a scheduled evaluation distinguishable
from a requested one (design §5.2, D-10) so that a backend can behave
differently for each — and a backend can only branch on a string the contract
fixes. Left unstated, the strings are an implementation detail a renderer-driven
host would be free to change, which is the failure this project exists to avoid.

**What the requirement must fix, and what it must leave open** (D-19, raised as `review-code.md` F-2). What a backend needs in order to branch is that
these three names *mean* these three things. Closing the set — *"one of exactly
three values … the host MUST NOT add a fourth"* — is a separate and stronger
claim, and it binds every conforming host, not the one this build ships. That
is the narrowing `CLAUDE.md`'s third invariant exists to prevent: *"Do not
narrow wire compatibility merely because the current renderer implements only a
subset of admitted protocol capabilities."* The slice's own roadmap collides
with the closed reading inside one slice — `slice-003.md` Non-goals puts event
ingress in slice 004, a second stimulus into the same path — so a closed set
would be amended immediately after promotion.

The tolerance half was never written anywhere: R-7 requires the field and says
nothing about unknown values. Stating it here is what lets the set stay open
without leaving a backend author to guess.

### The change, as it will be stated

Added to the *Requests* table, after R-9:

| id | requirement | verified by |
|----|-------------|-------------|
| R-56 | Every `evaluate` the host originates carries `event.source` of `"host"`, and an `event.kind` naming why the host is asking. Three kinds are named by this requirement and mean what it says they mean: `"startup"`, once, when a host starts; `"requested"`, when a person asked; `"scheduled"`, when a resolved next check came due. A host MUST NOT reuse one of these three for anything else, and a backend MAY branch on them. The set is **open**: a host MAY originate an `evaluate` whose kind is none of the three, and a backend MUST tolerate a kind it does not recognise — treating it as an evaluation whose reason it does not know, never as a protocol error. | §7 |

§7's verification table gains a row for R-56 naming the serialization unit
tests and the fixture that pins each kind's wire form. The tolerance clause is
a requirement on backends, which the host cannot verify; it is verified by
inspection, like SPEC-001's other backend-side obligations.

---

## CD-2 — SPEC-001 §6.1: the request illustration's event source

**Document:** `docs/specs/001-host-backend-protocol.md`
**Section:** §6.1 Request messages
**Kind:** record accuracy. No requirement changes.

### Why

The illustration reads `"event": { "source": "timer", "kind": "scheduled", … }`.
No host build has ever emitted `"timer"`: `Stimulus::event` writes
`source: "host"` for every stimulus (`crates/goad/src/wire.rs:57-65`). The `kind`
half is right and this slice adopts it verbatim. Until now the divergence was
harmless illustration; CD-1 makes both fields normative, so the example must
show what the host sends.

One further divergence in the same illustration is **left alone**: it shows
`"data": {}` where the host emits `"data": null` for every stimulus it
originates (`Stimulus::event` writes `Value::Null`). R-7 requires a data
payload and R-51 makes an explicit `null` mean what omission means, so both
spellings satisfy the contract and neither is normative. It is noted here so a
later reader does not mistake it for an oversight.

### The change, as it will be stated

```json
{ "protocol": 1, "type": "evaluate", "now": "2026-08-23T04:12:00Z",
  "event": { "source": "host", "kind": "scheduled",
             "timestamp": "2026-08-23T04:12:00Z", "data": {} } }
```

---

## CD-3 — SPEC-001 §2, Boundaries: where the timer's own rules are written

**Document:** `docs/specs/001-host-backend-protocol.md`
**Section:** §2 Scope → Boundaries
**Kind:** a pointer. The scope line is correct as written and does not move.

### Why

§2 puts *"the timer that decides when to evaluate"* out of scope, and its
Boundaries paragraph names the seam: *"The timer abuts it at the resolved
next-check instant — it consumes one and calls `evaluate`."* Both stay true.
What changes is that the timer's behaviour is now written down, in SPEC-002, and
a reader arriving at R-28 needs to know that a host-side minimum spacing on
firing exists and does not contradict it. Without the pointer, R-28 — *"the host MUST NOT
adjust a backend's instruction to a value it prefers"* — reads as forbidding the
floor, which it does not: R-28 governs what is stored and reported, and the
floor governs when the host fires.

### The change, as it will be stated

Appended to the Boundaries paragraph:

> What the host does with a resolved instant once it holds one — including the
> minimum spacing it places between its own scheduled firings, which adjusts no
> stored instruction — is SPEC-002's.

Conditional on SPEC-002 being promoted. If `draft-spec.md` is abandoned at
audit, CD-3 is withdrawn with it and CD-1 and CD-2 stand alone.

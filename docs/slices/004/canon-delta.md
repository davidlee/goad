# Canon delta — Slice 004

**Status: draft.** Nothing below is canon. It is this slice's working authority
until it is applied at audit and reconciliation, under explicit user
endorsement, and recorded in `audit.md`'s Reconciliation table
(`docs/AGENTS.md` §*Canon that does not exist yet, or must change*).

Changes this slice makes to canon that **already exists**. One entry per
affected document and section: the document, the section, the change as it will
be stated, and why.

**Do not edit `docs/specs/002-host-scheduling-behaviour.md` while the slice
runs.** Design, plan and execution cite the entries below exactly as they would
the real thing.

This file exists because the slice **raised itself to tier 2** at scoping:
deciding how an event-triggered evaluation is bounded amends SPEC-002, and
SPEC-002/R-5 says in terms that it must be decided rather than inherited
(`design-log.md` 2026-09-08).

---

## CD-1 — SPEC-002: an event-triggered evaluation is bounded on its own anchor

**Document:** `docs/specs/002-host-scheduling-behaviour.md`
**Sections:** §2 Scope, §3 Principles, §4 Requirements, §5 Behaviour, §7
Verification. Exact wording is design's, not scoping's.
**Kind:** new requirement, appended. R-11 is the current highest id.

### Why

SPEC-002/R-5 bounds scheduled firings and nothing else, and closes with an
obligation on this slice: *"a host that adds a stimulus other than a due check
MUST decide separately how that stimulus is bounded, and MUST NOT read this
requirement as covering it."* ADR-004 states the other half — the scheduled
anchor is cleared by nothing, so an event neither clears it nor is bounded by
it. Between the two there is a hole exactly the size of this slice, and it was
left deliberately.

### The change, as decided

An event-triggered evaluation MUST NOT begin less than a fixed minimum spacing
after the event-triggered evaluation that preceded it. The anchor is its own:
the scheduled anchor (R-4) neither clears it nor is cleared by it, and neither
anchor is written by the other's firing. The host therefore holds two monotonic
anchors, one per bounded stimulus.

An event arriving **inside** that spacing, or while an exchange is in flight, is
**refused at ingress, naming the bound**. The host holds no queue and no pending
event: the bound is observable at the socket rather than expressed as a silent
delay, and the retry decision is the watcher's (brief §7).

§5 also gains one sentence noting that a second stimulus raises the rate at
which **OQ-4**'s case is reached — an evaluation superseding a view a person is
mid-answering. OQ-4 itself stands unanswered; this slice does not resolve it
(`slice-004.md` OQ-8).

Open at scoping, for design: the spacing's **value**; whether it is a constant
of the host as R-4's is, or configurable; and how the rule is stated so that a
third stimulus inherits the shape rather than the number.

---

## CD-2 — SPEC-001/R-56: `"host"` is reserved as an event source

**Document:** `docs/specs/001-host-backend-protocol.md`
**Section:** §4 Requirements → Requests, R-56; and §7 Verification's R-56 row.
**Kind:** an added clause on an existing requirement. R-56's decided content is
not altered — the three kinds, their meanings, and the open set all stand.

### Why

R-56 fixes `source: "host"` for every `evaluate` **the host originates**, and
lets a backend branch on the three kinds it names. This slice adds evaluations
the host does *not* originate: an ingested event carries the watcher's own
source and kind, verbatim, which is brief §7's and §19's shape
(`design-log.md` 2026-09-08).

That leaves `source` doing work R-56 never says it does. A backend told it may
branch on `kind` needs to know that a `"scheduled"` it sees actually came due,
and under pass-through nothing stops a watcher from writing
`{"source": "host", "kind": "scheduled"}`. The requirement that made the three
kinds trustworthy silently stops making them so, in the slice that introduces
the second origin — and it stops without any document saying it did.

### The change, as decided

`"host"` is a **reserved** event source. A host MUST NOT emit an `evaluate`
carrying `source: "host"` for an event it did not itself originate, and MUST
refuse an ingested envelope that claims it. A backend MAY therefore read
`source == "host"` as meaning the host is asking on its own account, and R-56's
three kinds as meaning what R-56 says only under that source.

Open at scoping, for design: whether the refusal is stated in SPEC-001 (a rule
about what a host emits, which is this spec's subject) or in this slice's own
ingress rules (a rule about what a host accepts, which is not); and the exact
wording, which must not narrow the open set.

# Canon delta — Slice 004

**Status: draft.** Nothing below is canon. It is this slice's working authority
until it is applied at audit and reconciliation, under explicit user
endorsement, and recorded in `audit.md`'s Reconciliation table
(`docs/AGENTS.md` §*Canon that does not exist yet, or must change*).

Changes this slice makes to canon that **already exists**. One entry per
affected document and section: the document, the section, the change as it will
be stated, and why.

**Do not edit `docs/specs/002-host-scheduling-behaviour.md`,
`docs/specs/001-host-backend-protocol.md` or
`docs/adr/004-scheduled-firings-are-spaced-from-the-previous-scheduled-firing.md`
while the slice runs.** Design, plan and execution cite the entries below
exactly as they would the real thing.

This file exists because the slice **raised itself to tier 2** at scoping:
deciding how an event-triggered evaluation is bounded amends SPEC-002, and
SPEC-002/R-5 says in terms that it must be decided rather than inherited
(`design-log.md` 2026-09-08).

---

## CD-1 — SPEC-002: an event-triggered evaluation is bounded on its own anchor

**Document:** `docs/specs/002-host-scheduling-behaviour.md`
**Sections:** §2 Scope **including its Boundaries paragraph**, §3 Principles, §4
Requirements, §5 Behaviour, **§6 Interfaces & contracts**, §7 Verification.
Exact wording is design's, not scoping's.
**Kind:** new requirement, appended as **R-12**; R-11 is the current highest id.
Also a new principle in §3.

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

**Settled in design** (`design.md` D-5, `design-log.md` 2026-09-08). The three
items scoping left open are answered:

- **The value is three seconds** — the same `MINIMUM_SPACING` R-4 already names,
  applied against a second anchor. There is **one** constant, not two: two
  constants meaning the same kind of thing are two things free to drift when no
  evidence fixes either number.
- **It is not configurable.** ADR-004's argument transfers intact: a bound a
  misconfiguration can remove is not a bound.
- **The rule is stated as a property of a stimulus class**, so a third stimulus
  inherits the shape rather than the number. SPEC-002 §3 gains a principle —
  *each bounded stimulus class is spaced from the previous firing of its own
  class, on its own monotonic anchor, and no anchor is written by another's
  firing* — and **R-12** is its ingested instance. R-4 becomes the scheduled
  instance of the same principle without its decided content changing.

§5 gains the paragraph describing what the bound looks like from outside: a
refusal at the socket rather than a silent delay, and events lost under load,
visibly. §7 gains a verification row naming slice 004's AC-5 and AC-6 tests.

**Two statements outside §§3–5, §7 stop being true and are amended with them.**

- **§2 Boundaries** (`:45`) currently reads *"this spec abuts SPEC-001 at
  **exactly two points**. It **consumes** the resolved instant SPEC-001/R-26
  produces, and it **produces** an `evaluate` whose event kind SPEC-001/R-56
  names."* Both halves move. The second point narrows — SPEC-002 produces an
  `evaluate` whose event kind R-56 names **when the host is asking on its own
  account**; an ingested evaluation carries the watcher's own kind, which R-56
  does not name — and a **third** abutment appears, with SPEC-003: the ingested
  spacing R-12 requires is what SPEC-003 makes visible at the socket, as a
  refusal rather than a delay. The paragraph is restated as *two points with
  SPEC-001 and one with SPEC-003*, so the numeric claim survives being applied.
- **§6 Interfaces & contracts** (`:164`) currently reads *"The host owns: the
  pending wait, **the minimum spacing**, and the decision to fire."* After R-12
  the host owns one spacing and **two anchors**, which is the whole of the new
  rule (D-5). The sentence becomes *"the pending wait, the minimum spacing and
  the two anchors it is measured from, and the decision to fire"*, and the
  paragraph below it that calls the spacing a constant of the host gains the
  same "one constant, one anchor per bounded stimulus class" reading. Nothing
  else in §6 moves: the scheduled evaluation's wire form is unchanged, and
  SPEC-002 still adds no event kind of its own.

---

## CD-2 — SPEC-001/R-56: `"host"` is reserved as an event source

**Document:** `docs/specs/001-host-backend-protocol.md`
**Section:** §4 Requirements → Requests, R-56; §7 Verification's R-56 row; and
**§9 References**.
**Kind:** a **qualification of R-56's first clause**, plus an added clause. The
three kinds, their meanings and the open set are untouched; what changes is the
reach of the universal. R-56 as written quantifies over *every* `evaluate` the
host originates, and after this slice the host originates an `evaluate` carrying
a watcher's `source` — so the requirement and its own new clause contradict each
other unless the universal is narrowed on the page rather than in a slice's
head.

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

**R-56's first clause is qualified.** It reads *"Every `evaluate` the host
originates carries `event.source` of `"host"`…"*, and becomes *"Every `evaluate`
the host originates **on its own account** carries `event.source` of `"host"`…"*.
Without that qualifier the requirement is false of a conforming host the moment
it forwards an ingested event: the host is the party that emits that request —
no other process can — so on the plain reading it is one the host "originates"
and must carry `source: "host"`, which the new clause below then forbids.

`"host"` is a **reserved** event source. A host MUST NOT emit an `evaluate`
carrying `source: "host"` for an event it did not itself originate. A backend
MAY therefore read `source == "host"` as meaning the host is asking on its own
account, and R-56's three kinds as meaning what R-56 says only under that
source.

**§9 References gains `draft-spec.md`** (SPEC-003 on promotion), as the document
that states the ingress-side half. The trust the clause above grants a backend
holds only because an envelope claiming the reserved source is refused, and that
refusal is stated in another document; a reference is what stops the grant
citing nothing.

The *ingress-side* half of that guarantee — that an envelope claiming the
reserved source is refused — is stated where a rule about what a host accepts
belongs, and is **not** part of this amendment. See below.

**Settled in design** (`design.md` D-3, `design-log.md` 2026-09-08): the rule
**splits by subject**, and this entry therefore carries only half of what the
scoping draft above states.

- **SPEC-001/R-56 gains the emission clause only**, over a first clause narrowed
  to evaluations the host originates *on its own account* — `"host"` is reserved
  to those, a host MUST NOT emit it for an event it did not originate, and a
  backend MAY therefore read `source == "host"` as the host asking on its own
  account. That is a rule about what a host *emits*, which is this spec's
  subject. The three kinds, their meanings and the open set are untouched.
- **The refusal moves to `draft-spec.md`/R-13** — a rule about what a host
  *accepts*, which SPEC-001 does not own. It cites R-56 for why it exists, and
  R-56's own clause is what a backend's trust actually rests on.

The one narrowing is the qualifier on the first clause, and it narrows the
requirement's *reach*, not a backend's freedom: R-56's set of kinds stays open,
and the reservation is on one `source` value rather than on any `kind`.

---

## CD-3 — ADR-004: the anchor's verification is no longer held by review alone

**Document:** `docs/adr/004-scheduled-firings-are-spaced-from-the-previous-scheduled-firing.md`
**Section:** Verification (second paragraph); and References.
**Kind:** an accuracy amendment to a record whose **decision is unchanged**.
Opened in design (`design.md` §10), not at scoping.

### Why

ADR-004 §Verification states, of the anchor as against the spacing itself:

> The *anchor* — as against the spacing itself — is held by **review**. No
> standing test can distinguish the anchor from the boolean alternative, because
> the two agree on every stimulus that exists today; the case that separates them
> is the one slice 004 will introduce.

Slice 004 introduces it. The anchor and the boolean alternative disagree in
exactly one situation, and `design.md` §9's AC-6 case (ii) is that situation: a
**scheduled** firing at T₀, an ingested firing at T₀+ε, and a `next_check` due
at T₀+1 s. Under the anchor the scheduled evaluation waits until T₀+3 s; under a
boolean cleared by "some other stimulus happened", the intervening ingested
firing clears the flag and it fires at T₀+1 s. AC-6's other two cases are real
and are also named, but they do not reach this: case (i) falsifies the third
alternative ADR-004 lists — an anchor on "the last thing the host did" — and
case (iii) holds CD-1's new event anchor, about which ADR-004 makes no claim at
all. Leaving the paragraph as written would have a future reader believe the
claim is still unfalsifiable when one test now covers it; naming all three
without saying which one discharges the debt would be the same error in the
other direction. `docs/AGENTS.md` requires an ADR be kept accurate as
consequences are learned: *"it is the **decision** that is fixed, not the
document."*

### The change, as decided

The Verification section's second paragraph is amended to say that the anchor's
independence from the boolean alternative **is** now verified, naming by file
and function the AC-6 test that distinguishes them — the *advances* case,
`design.md` §9 (ii) — and naming the other two AC-6 tests as what they are: one
against the "last thing the host did" alternative, one holding the event anchor
CD-1 adds. What remains held by review is only the *choice* of anchor over the
boolean for stimuli that do not yet exist. References gains
`docs/slices/004/design.md` §5.3 and §9.

**The decision itself is not touched.** The spacing is still three seconds,
still measured from the previous scheduled firing, still cleared by nothing.

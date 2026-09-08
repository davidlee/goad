# Review — design — Slice 004

**Subject:** design — `docs/slices/004/design.md` (§1–§10, D-1..D-19, R1..R5)
together with the artefacts it is bound to: `draft-spec.md` (R-1..R-16),
`canon-delta.md` (CD-1..CD-3) and `slice-004.md` (scope, AC-1..AC-8).
**Reviewer:** fresh agent — Claude Opus 5 (1M context), adversarial raiser
**Opened:** 2026-09-08
**State:** open

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-09-08 — the design of event ingress, attacked against canon
and against the code it asserts things about.

This slice opens a socket. That is a new *inbound* edge on a host that until now
only ever spoke first, and every claim the design makes about it is checkable
against something already written down. Nothing here is rejected or raised on
assertion; each finding cites `path:line`, a spec rule id, or a design section.

**The invariants I hold it to** — `CLAUDE.md`, the five:

1. *The host does not understand the user's domain.* The envelope is the widest
   new surface the host has ever exposed to user-authored data. The vocabulary
   scan greps names; I am reading for **meaning**. A field the host reads, a
   refusal that depends on what an event *is*, a config key that presumes a kind
   of watcher, a module name that implies a subject matter — all are breaches the
   grep cannot see.
2. *Permissive wire, canonical internals.* I want to find the normalization door
   and stand in it. Anything past it that is still `serde_json::Value`-shaped, or
   still able to fail, is a breach. And the converse: an envelope the host
   refuses because it does not model a field, rather than because the field's
   meaning is ambiguous, is the same breach from the other side.
3. *Do not narrow wire compatibility for the current renderer.* The failure the
   project exists to avoid. Any R-n in the draft spec that exists because
   `crates/goad/tests/renderer/` cannot show something is a finding.
4. *A backend failure never takes the host down, and never leaves it unable to
   invoke the backend again.* I will ask whether **ingress** failure has the same
   property, because the design's transport arguments were written for a child
   process the host spawns, not for a peer that connects to it. A hung writer, a
   half-closed connection, a socket unlinked underneath the listener, a client
   that connects and never sends — none of these have a SPEC-001 analogue.
5. *Strata run one way (ADR-001).* `json_type_name` is being widened. That is a
   stratum-1 internal becoming a stratum-2 dependency, and I will check it
   against ADR-001 and ADR-003 rather than against the convenience it buys.

**Where I expect the bodies.**

- **D-13** — an ingested evaluation is not a `Stimulus`, and `wire.rs` is left
  untouched. I will build the `Pending::Evaluate` the design describes against
  the actual types and ask what the backend receives, and what it loses, versus
  a scheduled firing. "Not host-originated" is a claim about provenance; I will
  test whether it is doing real work or merely dodging an enum variant.
- **D-3** — normalization stays in stratum 2 on a user-authored/backend-authored
  split. I will look for that split in the canon. If it is not there, it was
  invented here, and inventing a principle mid-slice to justify a placement is
  precisely what a design review is for.
- **AC-6 / CD-3** — the design declares the criterion as literally worded
  unachievable and substitutes a reading. Substituted readings are where an
  achievable claim gets dressed as the intended one. I will read AC-6's intent
  from `slice-004.md`, not from the design's paraphrase of it, and I will check
  whether CD-3's amendment to ADR-004 §Verification claims more than the two
  named tests actually hold.
- **CD-1 / CD-2** — amendments must be minimal, stated as they will appear, and
  survive being applied. I will apply them mentally to SPEC-002 and SPEC-001 and
  re-read the surrounding rules for contradiction — particularly anything that
  quantifies over *all* evaluations or *all* sources.
- **Spacing and refusal (AC-5)** — refusing rather than delaying is a decision
  about someone else's retry behaviour. I will look for the interleaving where a
  well-behaved watcher is refused, and for whether "the host is engaged" is
  observable in a way that makes retry sane rather than a storm.
- **The two `select!` arms** — cancellation safety at every arm the accept future
  appears in, and whether an accepted connection can be dropped without a reply
  (AC-3). Slice 003 owns an anchor here that must not move.
- **A-1 / R1** — accepting from a `tokio::spawn`ed task while Slint owns the main
  thread is unmeasured. The design makes it phase one. I will ask what the design
  *does* if the probe fails, and whether the rest of the design is written as
  though the probe already succeeded.
- **AC-7** — 23 call sites gain `Ingress::none()`. Parked state, or branch? A
  branch changes behaviour under some interleaving, and "unchanged bodies" would
  then be false.
- **Four-artefact agreement** — design, draft spec, canon delta and slice-004
  must say the same thing. Where two differ, one is wrong, and the gap is the
  finding.

**What I will not do.** I do not dispose; severities are set at raise time and
are not negotiated afterwards. Findings that turn out to be wrong are withdrawn
in later rounds, not deleted.

**Round 2** — 2026-09-08 — the repairs, read as new work.

Round 1 moved prose across four documents and one commit. The standing risk in a
round like that is not the finding that was missed; it is the repair that fixes
its instance, states a new claim, and leaves the new claim unpropagated. So this
round asks three things of each repair. Does it do what its disposition said?
Does it fix the class or the instance? And does every artefact that repeats the
repaired claim now repeat the repaired version of it — the design's tables and
its two mermaid diagrams included, which are prose that does not get re-read.

Two repairs get particular weight because they replaced an argument rather than
a sentence: F-1's third AC-6 case, which has to actually separate the anchor
from the boolean, and F-3's new placement reason, which has to be sound rather
than merely different from the invented one it replaced. Two more are attacked
as **restatements of an overreach** — F-2 and F-8 — where the question is
whether the narrowed claim is now true and whether what was given up is stated
as residue rather than argued away. F-10 added a field to a versioned wire, so
it is attacked for what a new field always costs: an unstated edge, and a
principle it might have traded.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | fix-now | verified |
| F-2 | major | doc-wrong | verified |
| F-3 | major | doc-wrong | verified |
| F-4 | major | doc-wrong | verified |
| F-5 | minor | fix-now | verified |
| F-6 | minor | fix-now | verified |
| F-7 | minor | fix-now | verified |
| F-8 | minor | fix-now | verified |
| F-9 | minor | fix-now | verified |
| F-10 | minor | fix-now | verified |
| F-11 | nit | doc-wrong | verified |
| F-12 | minor | fix-now | |
| F-13 | minor | fix-now | |
| F-14 | minor | fix-now | |
| F-15 | minor | settle-in-code | |
| F-16 | nit | fix-now | |
| F-17 | nit | fix-now | |

### F-1 — Neither AC-6 test reaches ADR-004's undischarged case, so CD-3 would write a false verification claim into canon

**Severity:** blocker
**Location:** `design.md` §9 (the AC-6 row), `canon-delta.md` CD-3, `slice-004.md` AC-6

**Expected:** ADR-004 §Verification names one specific undischarged claim:

> The *anchor* — as against the spacing itself — is held by **review**. No
> standing test can distinguish the anchor from **the boolean alternative**,
> because the two agree on every stimulus that exists today; the case that
> separates them is the one slice 004 will introduce.

The boolean alternative is defined in ADR-004 §Alternatives considered: *"Space
only when the predecessor was itself a scheduled firing, tracked as a boolean"* —
a boolean **cleared by "some other stimulus happened"**. The two rules diverge in
exactly one situation: a scheduled firing, then an ingested firing, then a
scheduled check that comes due **inside** the 3 s window. Under the anchor the
scheduled firing waits until `previous scheduled + 3 s`; under the boolean the
intervening ingested firing has cleared the flag and it fires at once. That
divergence is the busy loop `docs/slices/003/review-design.md` F-2 predicted:
*"an event arriving at machine rate would clear the spacing before every
scheduled firing, and the busy loop the spacing exists to prevent would be
back."* `slice-004.md` AC-6 names it in one word — an ingested evaluation
neither delays **nor advances** a scheduled firing.

**Observed:** `design.md` §9's AC-6 row names two tests, and neither sets that
case up:

- (i) *"an ingested exchange falling between a short `next_check` and its firing
  does not push that firing out by the spacing — an ingested firing never writes
  `floor_until`"* — this is the **delays** half. It falsifies the third
  alternative ADR-004 lists (an anchor on "the last thing the host did"), and the
  boolean agrees with the anchor on it, because in that setup no scheduled firing
  precedes and `floor_until` is already in the past (`controller.rs:407`,
  `floor_until = started`).
- (ii) *"an ingested firing, then a scheduled firing, then a second event inside
  the ingested spacing is **still** `too_soon` — a scheduled firing never clears
  the event anchor"* — this is anchor-versus-boolean on the **event** floor, a
  rule CD-1 creates in this slice. ADR-004 makes no claim about an event floor;
  its debt is about `floor_until`, whose one write site is `controller.rs:420`.

So AC-6's **advances** clause — the only direction in which the boolean and the
anchor disagree about `floor_until`, and the direction in which the hazard lies —
is asserted by nothing. CD-3 nevertheless amends ADR-004 to say the anchor
*"**is** now verified"* and that leaving the paragraph *"would have a future
reader believe the claim is still unfalsifiable when it is now covered."* Applied
as written, CD-3 retires a live review obligation on the host's only defence
against an unbounded exchange loop, and replaces it with a citation to two tests
that do not hold it.

**Evidence:** `docs/adr/004-...md` §Decision (*"nothing clears it: no other
stimulus… resets the anchor or **grants a firing that would otherwise be too
soon**"*), §Alternatives considered (the boolean), §Verification (the debt);
`crates/goad/src/controller.rs:407` (`floor_until = started`), `:420` (the one
write site); `docs/slices/004/design.md` §9 AC-6 row; `canon-delta.md` CD-3 §The
change, as decided; `slice-004.md` AC-6 (*"neither delays nor advances"*).

The missing test is one row, not a redesign: previous **scheduled** firing at
T₀, an ingested firing at T₀+ε, a `next_check` due at T₀+1 s, and the assertion
that the scheduled evaluation does not reach the backend before T₀+3 s.

**Disposition:** fix-now
**Response:** Correct, and checked against ADR-004 §Decision, §Alternatives and
§Verification, and against `controller.rs:407` and `:420`. Neither named test
reaches the *advances* direction, which is the only one in which the anchor and
the boolean disagree about `floor_until`.

`design.md` §9's AC-6 row now names **three** cases and says which does what:
(i) *does not delay* — falsifies the "last thing the host did" alternative;
(ii) *does not advance* — **new**: a scheduled firing at T₀, an ingested firing
at T₀+ε, a `next_check` due at T₀+1 s, and the assertion that the scheduled
evaluation does not reach the backend before T₀+3 s; (iii) *the event anchor is
not cleared* — holds CD-1's new rule, on which ADR-004 makes no claim. The row
states in terms that **(ii) is the one that discharges ADR-004's debt**.

`draft-spec.md` §7's R-12 row is the row that carries AC-6 — no row cited AC-6
before — and it now names all three, spelling out (ii). CD-3's §Why is rewritten
to derive the divergence rather than assert it, and its §The change, as decided
now amends ADR-004 to name **the test that distinguishes the anchor from the
boolean by file and function**, and to say what the other two hold instead of
counting them toward the debt. AC-6 in `slice-004.md` is untouched.

**Outcome:** verified

### F-2 — `draft-spec.md` R-15 is unsatisfiable for at least two of the eight refusal reasons

**Severity:** major
**Location:** `draft-spec.md` R-15; `design.md` §5.1 (the part table), §5.2 (the
reply table), §5.4 (Shutdown); `crates/goad/src/controller.rs:155-183`

**Expected:** R-15 is normative and unqualified: *"A refusal MUST also be
reported on the host's own diagnostics surface, so that it is visible to a
person who is not the writer."* `design.md` §5.1 assigns that surface to
stratum 3 — *"`diagnostics` | 3 | the one line a person reads for a refusal"* —
and §5.2 says `Refused` gains `Ingress { reason, detail }`. Its §7 row verifies
it as *"renderer: a refusal appears on the diagnostics surface (AC-4)."*

**Observed:** two of the eight reasons in §6.3 cannot reach it.

1. **`engaged`.** It is decided by the loop, and by definition only *while an
   exchange is in flight* — i.e. from inside the inner `select!`. The controller's
   diagnostics is a **whole-value slot**, not a log: `refuse` assigns
   `self.diagnostics = Diagnostics::refused(refused)` (`controller.rs:183`) and
   `absorb` assigns `self.diagnostics = diagnostics` from the exchange's outcome
   (`controller.rs:173`). The only `glass.present` calls are at the top of the
   loop (`controller.rs:410`, `busy = false`) and immediately after `engage()`
   (`controller.rs:488`, `busy = true`). Neither runs between an inner-loop
   refusal and the `absorb` that overwrites it, so the `engaged` line is written
   and destroyed without ever being presented. Every `engaged` refusal — the
   commonest one a real watcher will meet — is invisible to a person.
2. **`unavailable` on shutdown.** §5.4 Shutdown: *"`serve` returns and hands
   `ingress` back inside `Served`; `main` drops it… An `Answer` already in flight
   is dropped, and the listener writes `unavailable` before closing."* That
   refusal is authored by the listener after the loop has returned. There is no
   loop left to call `controller.refuse`, and no `present` will ever run again.

The design's own model forbids the obvious repair: §5.1 P-2/D-4 makes the loop
the single reply site precisely so that *"a refusal [is put] on the diagnostics
surface without inventing a second channel for it"* — but the surface is
single-slot and write-only-when-presented, and the two cases above fall outside
both properties.

**Evidence:** `crates/goad/src/controller.rs:173`, `:183`, `:410`, `:488`;
`crates/goad/src/diagnostics.rs:141` (`Diagnostics::refused` builds a fresh
whole value); `draft-spec.md` R-15 and its §7 row; `design.md` §5.4 Shutdown.

**Disposition:** doc-wrong
**Response:** The observation holds at `controller.rs:173`, `:183`, `:410` and
`:488`, and the class is wider than the two instances named: it is **every
refusal the loop answers from the inner `select!`**, not only `engaged`. A shape
refusal that arrives during an exchange is overwritten by the same `absorb`.
`engaged` is merely the case that is *always* inside it.

R-15 is the defect and is restated to what the design can hold: a refusal
decided **while no exchange is in flight** must reach the diagnostics surface;
one decided during an exchange, and one decided after the loop has ended, are
reported to the writer only — with the requirement saying explicitly that this
is a bound on the surface and not a licence to be silent, since every refusal
reaches its writer under R-8.

`draft-spec.md` §6.3 gains a paragraph naming which reasons a person sees and
which are reply-only; `design.md` §5.2's reply table gains a *reaches a person?*
column stating the same per reason, with a note that the column is about the
diagnostics surface and not about the reply. `design.md` §5.1's part table row
for `diagnostics` and the paragraph above it are corrected — the model gives
refusals *a route* to the surface, not all of them. D-4's rejected alternative is
restated so the comparison it makes is still true. `draft-spec.md` §7's R-15 row
now verifies the bound **from both sides**, so the requirement is falsifiable
rather than merely narrower, and `design.md` §9's AC-4 row names those tests.

`slice-004.md` Follow-ups gains *Refusals a person cannot see*, naming a count or
a log as the obvious shapes and explicitly not choosing between them. AC-3 is
untouched.

**Outcome:** verified

### F-3 — D-3's decisive premise is invented in this slice, and the precedent it rests on is disanalogous

**Severity:** major
**Location:** `design.md` §2 F6, §5.1 (the part table), D-3; `slice-004.md`
§Scope (stratum 1 paragraph); `canon-delta.md` (untouched ADR-001)

**Expected:** ADR-001 §Decision assigns the strata their contents in terms:

> 1. **Pure semantic core** — protocol types, **wire-to-canonical
>    normalization**, schedule resolution.
> 2. **I/O shell** — backend transport, configuration, host operational state,
>    event ingress.

(`docs/adr/001-one-way-strata.md:34-37`.) ADR-001 §Context grounds that split in
brief §3.3's one line — *"permissive at the wire, canonical after it"* — and
nowhere distinguishes a **user-authored** wire from a **backend-authored** one.

**Observed:** `design.md` §2 F6 states the split as though it were a finding
about the code: *"a user-authored format normalizes in stratum 2 (`config.rs`'s
`File` → `Config`); a backend-authored one normalizes in stratum 1
(`protocol/normalize.rs`)"* — and then concedes in the same sentence that **the
rule does not decide the case**: *"The watcher is neither party, and F6 is what
decides which rule it takes."* A premise that admits the subject falls under
neither of its two arms cannot decide between them. D-3 and OQ-4 nevertheless
close on it, and `slice-004.md` §Scope hardens it further: *"a user-authored
format normalizes there, as `config.rs` already does."*

The `config.rs` precedent is disanalogous at exactly the point that matters.
`File` → `Config` normalizes a user-authored format into a **stratum-2** type
(`crates/goad-shell/src/config.rs:28`). The envelope normalizes a user-authored
format into `Event`, which is a **stratum-1 canonical type**
(`crates/goad-semantics/src/protocol/canonical.rs:490`). So the design puts the
only door from untrusted bytes into a stratum-1 canonical value in stratum 2,
and `Event`'s canonicality is thereafter enforced in two crates rather than one —
which is what `CLAUDE.md`'s second invariant (*"normalization is the only door
into the canonical types"*) exists to prevent.

This is not a claim that the placement is wrong; ADR-001 §Consequences concedes
in writing that *"placement questions will arise that the three names do not
settle on their own… that is a distinction someone has to make deliberately each
time."* It is a claim that the argument offered is not the one that would settle
it, and that two artefacts present an invented split as established canon. If
stratum 2 is the right home, the reason has to be stated as a decision — and,
per `docs/AGENTS.md`, a decision *"that could later be reversed by accident gets
its own ADR"*, which §10's canon table does not carry.

**Evidence:** `docs/adr/001-one-way-strata.md:34-37` and §Consequences/Negative;
`crates/goad-shell/src/config.rs:28`;
`crates/goad-semantics/src/protocol/canonical.rs:490`; `design.md` §2 F6 and D-3;
`slice-004.md` §Scope, stratum 1 paragraph; `design.md` §10 (no ADR listed).

**Disposition:** doc-wrong
**Response:** The argument was the defect; the **placement is unchanged**. The
user-authored / backend-authored split appears nowhere in ADR-001 or the brief,
and §2 F6 conceded in its own sentence that the split does not reach the case.
It is deleted from `design.md` §2 F6 and D-3 and from `slice-004.md` §Scope, and
replaced by the reason that actually decides it: **stratum 1's normalization
holds one contract — SPEC-001 — in one place, so a second host implementation is
held to the same normalization of the same wire.** The envelope is a different
contract with different parties that no backend ever sees; putting it in stratum
1 would make the protocol crate the home of two unrelated wires and give
`goad-semantics` a reason to change whenever the socket's contract does. Stated
as the deliberate call ADR-001 §Consequences says must be made each time.

The sharpest point is answered head-on in a new `design.md` §5.1 paragraph.
`CLAUDE.md`'s second invariant guards the **protocol's** canonical values —
`protocol/wire.rs` → `normalize.rs` → `canonical.rs`, which this slice does not
touch. `Event` holds no invariant a constructor could enforce: four `pub` fields
(`canonical.rs:490-497`), `source`/`kind` plain strings, `data` opaque by
SPEC-001/R-9, and the one field that can fail to be canonical is `timestamp`,
whose canonicality is carried entirely by `jiff::Timestamp` — and jiff is on
stratum 2's manifest allowlist (`allowlist.rs:19-27`). Constructing an `Event`
outside stratum 1 is **already the status quo**: `wire.rs:62`, stratum 3, has
built one for every host-originated evaluation since slice 002. So the envelope
adds a second producer of a transparent record, not a second gate on a guarded
one. What the argument does owe is stated rather than skipped: the offset rule is
now written twice (SPEC-001/R-22 and `draft-spec.md` R-10) and can drift, so R-10
is written to mirror R-22 and both parse through the same jiff two-step.

`design.md` §10 gains the ADR — *the event envelope normalizes in stratum 2* —
to be written at reconciliation, cited from D-3, `slice-004.md` §Scope and
§Governing canon, and `draft-spec.md` §9 References (whose ADR-001 line carried
the invented split and no longer does).

**Outcome:** verified

### F-4 — CD-2 leaves SPEC-001/R-56's universal unqualified, so a conforming host breaches R-56 the moment it forwards an ingested event

**Severity:** major
**Location:** `canon-delta.md` CD-2; `docs/specs/001-host-backend-protocol.md:91`

**Expected:** an amendment is minimal and *survives being applied*. After CD-2,
SPEC-001/R-56 must be true of every `evaluate` a conforming host emits.

**Observed:** R-56's first clause is a universal over requests, not over events:

> **Every `evaluate` the host originates** carries `event.source` of `"host"`,
> and an `event.kind` naming why the host is asking.

CD-2 explicitly declines to touch it — *"R-56's decided content is not altered —
the three kinds, their meanings, and the open set all stand"* — and adds only a
prohibition (*"a host MUST NOT emit an `evaluate` carrying `source: "host"` for
an event it did not itself originate"*). After the slice, the host emits an
`evaluate` carrying a watcher's `source` and `kind`. The host is the party that
originates that request — the backend never asks, and no other process can emit
it — so on the plain reading of R-56 the ingested `evaluate` is one *"the host
originates"* and must carry `source: "host"`, which CD-2's own new clause then
forbids. The amendment is internally consistent only if `originates` is silently
re-read as *originates on its own account*, which is a change to R-56's decided
content by any other name.

The design is aware of the distinction — D-13 and `slice-004.md` §Scope both turn
on *"an ingested evaluation is not host-originated"* — but that reading is
recorded in a slice, not in the requirement a second host implementation would be
held to. The repair is one qualifier in R-56's first clause.

A second, smaller consequence of the same omission: CD-2 grants a backend a right
(*"a backend MAY therefore read `source == "host"` as meaning the host is asking
on its own account"*) whose soundness rests entirely on `draft-spec.md` R-13,
which lives in a different document. CD-2's section list is *"§4 Requirements →
Requests, R-56; and §7 Verification's R-56 row"* — SPEC-001 §9 References is not
amended, so the granted trust will cite nothing.

**Evidence:** `docs/specs/001-host-backend-protocol.md:91`; `canon-delta.md`
CD-2 §Kind and §The change, as decided; `design.md` D-13;
`crates/goad/src/wire.rs:62-69` (`Stimulus::event` hard-codes
`source: "host"`, the only current emitter).

**Disposition:** doc-wrong
**Response:** CD-2 is the defect, exactly as stated:
`docs/specs/001-host-backend-protocol.md:91` quantifies over every `evaluate`
the host originates, the host is the only party that can emit an ingested one,
and CD-2's new clause then forbids what the unqualified universal requires.

CD-2's §The change, as decided now leads with the qualification: R-56's first
clause becomes *"Every `evaluate` the host originates **on its own account**…"*,
with the reason it is needed spelled out. Its §Kind no longer claims R-56's
decided content is unaltered — it says what is altered (the reach of the
universal) and what is not (the three kinds, their meanings, the open set). The
closing sentence that read *"the wording narrows nothing"* is corrected to name
the one narrowing and say it is of the requirement's reach, not of a backend's
freedom.

**§9 References** joins CD-2's section list, gaining `draft-spec.md` (SPEC-003 at
promotion) as the document stating the ingress-side refusal, so the trust the
clause grants a backend cites the document it actually depends on.

**Outcome:** verified

### F-5 — CD-1's section list omits two places in SPEC-002 that R-12 falsifies

**Severity:** minor
**Location:** `canon-delta.md` CD-1 §Sections;
`docs/specs/002-host-scheduling-behaviour.md:45`, `:164`

**Expected:** CD-1 states its sections as *"§2 Scope, §3 Principles, §4
Requirements, §5 Behaviour, §7 Verification"*, and the delta is applied to
exactly those at reconciliation. Anything else in SPEC-002 must stay true.

**Observed:** two statements outside that list stop being true once R-12 lands.

- **§2 Boundaries** (`:45`): *"this spec abuts SPEC-001 at **exactly two
  points**. It **consumes** the resolved instant SPEC-001/R-26 produces, and it
  **produces** an `evaluate` whose event kind SPEC-001/R-56 names."* After R-12,
  SPEC-002 also bounds an evaluation whose event kind is the **watcher's** and
  which R-56 does not name, and it abuts `draft-spec.md` at a third point (CD-1's
  own bound is *"visible at the socket"*, and the draft's §2 already declares that
  abutment from the other side). §2 Scope being on the list does not reach this
  paragraph, which makes a numeric claim of its own.
- **§6 Interfaces & contracts** (`:164`): *"The host owns: the pending wait, **the
  minimum spacing**, and the decision to fire."* This is the section that
  enumerates what the host owns, and after CD-1 the host owns **two anchors**.
  One constant, two anchors is CD-1's own formulation (D-5), and §6 records
  neither.

**Evidence:** `docs/specs/002-host-scheduling-behaviour.md:45`, `:164`;
`canon-delta.md` CD-1 §Sections; `design.md` D-5, §5.3.

**Disposition:** fix-now
**Response:** Both citations check out at `002-host-scheduling-behaviour.md:45`
and `:164`. CD-1's §Sections now reads *§2 Scope **including its Boundaries
paragraph**, §3, §4, §5, **§6 Interfaces & contracts**, §7*, and CD-1 gains a
block stating the change to each as it will appear:

- **§2 Boundaries** — both halves of the "exactly two points" claim move. The
  *produces* point narrows to an `evaluate` whose kind R-56 names **when the
  host is asking on its own account** (which is CD-2's qualifier, so the two
  deltas agree), and a **third** abutment appears, with SPEC-003: the ingested
  spacing R-12 requires is what SPEC-003 makes visible at the socket. The
  paragraph is restated as *two points with SPEC-001 and one with SPEC-003*, so
  the numeric claim survives being applied.
- **§6** — *"The host owns: the pending wait, the minimum spacing, and the
  decision to fire"* becomes *"…the minimum spacing **and the two anchors it is
  measured from**…"*, which is D-5's one-constant-two-anchors formulation, and
  the paragraph below it takes the same reading. It is stated that nothing else
  in §6 moves.

**Outcome:** verified

### F-6 — `design.md` §5.2's reply table contradicts §5.4 and `draft-spec.md` on who decides `unavailable`

**Severity:** minor
**Location:** `design.md` §5.2 (the reply table's last row) versus §5.4 (*Order
of judgement*, step 3) and `draft-spec.md` §6.3

**Expected:** §5.2 opens with *"this section is the design's statement of the
same thing and must not diverge from it [`draft-spec.md`]."*

**Observed:** the reply table gives one decider and one cause —

| `unavailable` | listener | the loop is gone — the host is stopping |

while §5.4 step 3 gives a second decider and a second cause — *"clock unreadable
→ `unavailable`, with `detail` naming the clock"* — decided inside the loop, and
then states the merged meaning two paragraphs later (*"it is stopping, or its
clock is unreadable"*). `draft-spec.md` §6.3 carries both. The table is the row a
reader building the listener will implement from, and it says the listener owns a
reason the loop must also be able to raise.

**Evidence:** `design.md` §5.2 reply table, §5.4 steps 1-4 and the paragraph
after; `draft-spec.md` §6.3 `unavailable` row.

**Disposition:** fix-now
**Response:** Correct — the table was the one place the second decider and the
second cause were missing. `design.md` §5.2's `unavailable` row now reads
*listener **or loop*** and carries both causes — the loop is gone and the host is
stopping (listener), or the clock is unreadable (loop, §5.4 step 3) — which
matches §5.4 and `draft-spec.md` §6.3 word for word in substance. The same row
also gained the *reaches a person?* column from F-2, which distinguishes the two
cases there as well.

**Outcome:** verified

### F-7 — a top-level JSON value that is not an object has no reason in the closed set

**Severity:** minor
**Location:** `draft-spec.md` R-9 and §6.3; `design.md` §5.2 reply table

**Expected:** R-14 makes the set in §6.3 **closed**, and AC-4 requires every
refusal to draw from it. R-9 requires *"one JSON **object** carrying exactly four
keys"*, so `[1,2,3]`, `"hello"`, `42` and `null` must each be refused.

**Observed:** neither reason fits. §6.3 defines `malformed` as *"the bytes were
not one JSON document"* — `[1,2,3]` **is** one JSON document — and
`invalid_envelope` as *"a missing, wrong-typed, empty, unknown or duplicated key,
or a timestamp the host could not read"*, all of which presuppose an object to
have keys in. `design.md` §5.2's table repeats both definitions verbatim. The
input is trivially reachable (`echo '[]' | socat - UNIX-CONNECT:…`), and stratum
1 already has the case and a vocabulary for it — `error.rs:14-18` notes
`json_type_name` exists partly because *"`Object`'s refusal of a non-object"*
reports it. One clause in `invalid_envelope`'s definition closes it; leaving it
open means R-14's *"exact token set"* test in §7 either fails or is written
around a gap.

**Evidence:** `draft-spec.md` R-9, R-14, §6.3; `design.md` §5.2 reply table;
`crates/goad-semantics/src/error.rs:14-18`.

**Disposition:** fix-now
**Response:** The gap is real: `[1,2,3]` is one JSON document, so `malformed`
does not fit, and `invalid_envelope`'s definition presupposed an object to have
keys in. Closed by one clause, identically in `draft-spec.md` §6.3 and
`design.md` §5.2: `invalid_envelope` now means *the top-level value was not a
JSON object; or a missing, wrong-typed, empty, unknown or duplicated key; or a
timestamp the host could not read*, with `detail` naming the key **or the type
found**. The set stays closed at eight and R-14's exact-token test is unaffected.

Fixed at the class rather than the instance: **R-9** gains the normative
sentence requiring the refusal (*"A top-level value that is well-formed JSON but
is not an object MUST be refused, naming the type found"*), because the reason
existing without a requirement behind it is the same gap one layer up;
`draft-spec.md` §7's R-9 row gains the case; and `design.md` §5.2's
`EnvelopeFault` vocabulary gains its seventh member, *not an object*, so the
precise diagnostic exists to name it.

**Outcome:** verified

### F-8 — "ingress cannot die silently mid-run" is false in two ways the design does not reach

**Severity:** minor
**Location:** `design.md` §5.5 (edge cases, the `accept()` row), §5.2
(`Ingress::arrival`'s signature); `slice-004.md` AC-12

**Expected:** the design claims it outright: *"The task exits **only** when the
loop's channel closes, so ingress cannot die silently mid-run (AC-12)."*

**Observed:** two mechanisms defeat it, neither covered.

1. **The signature cannot express a closed channel.** `pub async fn arrival(&mut
   self) -> Arrival` has no failure case and no `Option`. `Ingress::none()` is
   documented as *"**Never resolves** when nothing is bound"*, so the only
   behaviour available when a *bound* receiver's senders are all dropped — an
   accept task that ended for any reason other than the loop dropping the
   receiver, a panic inside it included — is the same never-resolving park. The
   loop then holds an arm that will never fire, with nothing to report and no way
   to tell that case from `none()`.
2. **The socket can be unlinked underneath a live listener.** Nothing re-probes
   the path after `bind`, and D-16 removes the host's own unlink. A `rm` of the
   path (or a second host reclaiming it under R-3, which the §6.1 bind race
   describes happening *at startup* and does not confine there) leaves the
   listener holding a bound fd no `connect` can reach. Every subsequent watcher
   gets `ECONNREFUSED`/`ENOENT`, the host reports nothing, and `just demo`'s
   one-liner silently stops working.

AC-12's own wording survives both — neither takes the host down nor stops it
invoking the backend — so the finding is against the design's stronger claim and
against the sentence in §5.5 that cites AC-12 as holding it.

**Evidence:** `design.md` §5.5 edge-case table, `accept()` row; §5.2
`Ingress::arrival` and `Ingress::none` doc comments; D-16; `draft-spec.md` §6.1
*Non-normative limit — the bind race*; `slice-004.md` AC-12.

**Disposition:** fix-now
**Response:** Both mechanisms hold, and they need different answers: one is a
type defect, the other is a residue. Fixed as the class — *the loop must be able
to tell "no arrivals will ever come" from "nothing was ever bound"* — rather than
as the two instances.

**The signature.** `Ingress::arrival` becomes
`pub async fn arrival(&mut self) -> Option<Arrival>`. `None` means a **bound**
receiver whose senders are all gone — an accept task that ended for any reason
other than this receiver being dropped, a panic included — and it is
distinguishable from `Ingress::none()`, which still never resolves at all. The
receiver is dropped on the way out, so the arm parks from then on rather than
spinning on a closed channel. The loop does something reportable with it: it
folds one `Refused::Ingress` (`unavailable`, detail naming that ingress has
stopped) onto the diagnostics surface. That is a refusal in substance — the
standing answer to every envelope from then on, and the one refusal whose writer
cannot be told directly — so **D-11 is not widened**; the surface still shows
refusals only.

**The sentence.** `design.md` §5.5's *"the task exits **only** when the loop's
channel closes, so ingress cannot die silently mid-run"* is split into what it
actually holds and what it does not. The `accept()` row keeps the ordinary case;
a new row states what the `None` path holds — *this* is what "cannot die
silently" means, and the loop says so once; and a third row states the unlinked
socket as a **stated residue**: nothing re-probes the path after `bind`, D-16
removes the host's own unlink, so a `rm` or a second host reclaiming the path
under R-3 leaves a bound descriptor no `connect` can reach, and the host reports
nothing because from its side nothing arrives. `draft-spec.md` §6.1 gains the
matching non-normative limit beside the bind race, and `slice-004.md`'s
single-instance follow-up gains it as that gap's second face. AC-12's wording
stands and is stated to survive both.

**Outcome:** verified

### F-9 — §6.4 calls 500 ms a bound on "time per connection" when only the read is bounded

**Severity:** minor
**Location:** `draft-spec.md` §6.4 and R-7; `design.md` §5.2
(`ENVELOPE_DEADLINE`), §5.1 I-2

**Expected:** §6.4's table row is headed **"time per connection"**, value 500 ms,
justified as *"a connection that never completes an envelope must not hold
ingress."* R-7 states the same bound as *"every **read** from a connection… in
both bytes and time."*

**Observed:** the two are different quantities, and only the read is bounded. A
connection's life is `accept → read (≤500 ms) → send Arrival → **await the
loop's reply** → write → close`, and the wait on the loop is bounded by nothing.
`serve` runs on Slint's executor via `slint::spawn_local` (`main.rs:102`), so
whenever the Slint event loop is not polling it — a slow present, a blocked main
thread — the arrival is neither judged nor timed out. I-2 makes this the
listener's *only* state: *"the listener awaits the reply before accepting the
next connection"*, so one unjudged arrival stops all ingress with no diagnostic
and no reply, for as long as the main thread is stuck.

This is a naming and completeness defect rather than a hazard the slice
introduces: it is the same main thread everything else already depends on. But
§6.4's justification is written as though the 500 ms closes it, and it does not.
Either the row is renamed to "time per **read**", or the wait for judgement gets
a bound of its own.

**Evidence:** `draft-spec.md` §6.4 and R-7; `design.md` §5.1 I-2, §5.2
`ENVELOPE_DEADLINE`; `crates/goad/src/main.rs:102` (`slint::spawn_local`).

**Disposition:** fix-now
**Response:** Correct, and the misnomer was in both rows, not only the timed one:
**bytes** per connection was equally a bound on the read. Both are renamed to
*per **read***, which is also the quantity R-7 already states, so §6.4 and R-7
now name the same thing.

The justification is repaired rather than deleted. `draft-spec.md` §6.4 gains
*What is not bounded, and why*: the connection's life is `accept → read
(bounded) → hand the arrival to the host → await its judgement → reply → close`,
and **the wait for judgement has no bound** — it ends when the host judges, on
the one thread it does everything else on. A host not making progress delays an
arrival exactly as it delays a due check, a person's click and a backend's
answer; ingress inherits that dependency rather than adding one. It is also said
why no number would help: a timeout on the wait would mean answering an envelope
the host had not judged.

What I-2 therefore implies is stated in I-2 itself: with at most one arrival
outstanding, an unjudged arrival stops **all** ingress for as long as the main
thread is not polling `serve` (`main.rs:102`, `slint::spawn_local`), and
`ENVELOPE_DEADLINE` does not reach it. `design.md` §5.2's constant carries the
same note at its declaration, so a reader implementing the listener cannot take
500 ms for a connection budget.

**Outcome:** verified

### F-10 — `retry_after` is computed and then made unreachable, in a contract that puts retry on the watcher

**Severity:** minor
**Location:** `design.md` §5.2 (`Refusal::TooSoon { retry_after }`);
`draft-spec.md` P-C, R-14, §6.3, OQ-1

**Expected:** P-C — *"The writer decides what to do about a refusal… The host's
obligation ends at telling the truth promptly."*

**Observed:** the host computes the one number that would let a writer decide
well and then puts it somewhere the contract forbids reading. `Refusal` carries
`TooSoon { retry_after }`; `reason()` returns the wire token and `Display`
produces `detail`; R-14 and §6.3 then say *"**Nothing may branch on it**, and its
wording is not part of this contract."* §6.3's advice for `too_soon` is
*"coalesce in the watcher"* and for `engaged` *"retry, or do not"* — with no
machine-readable interval anywhere. Against a 3 s bound, a writer that must not
parse prose has exactly two strategies: drop the event, or retry blind. Blind
retry against a 500 ms per-connection read budget is a connect loop, which is the
retry storm P-C's *"promptly"* is meant to make unnecessary.

OQ-1 defers this (*"Deferred until a client wants to act on it; `goad emit` is
the first that could"*), and R3 in `design.md` §8 already names the risk that the
closed vocabulary breaks one slice later. The observation is that the value
exists at the moment of refusal and is discarded — adding it later is a wire
change to a versioned contract whose first real client is slice 005.

**Evidence:** `design.md` §5.2 (`Refusal` payloads), §8 R3; `draft-spec.md` P-C,
R-14, §6.3, OQ-1.

**Disposition:** fix-now
**Response:** The number goes on the wire now rather than becoming a change to a
versioned contract one slice later. R-14 gains a second sentence: a `too_soon`
refusal MUST carry **`retry_after_ms`**, a whole number of milliseconds measured
at the moment of refusal, after which the spacing will have elapsed; no other
reason carries it, and a reader MAY act on it.

R-14's principle is preserved rather than traded away: `detail` remains
unbranchable prose whose wording is not part of the contract, and the new field
is structured, named, and typed. §6.3 gains the example line, the field's
paragraph — including that it is **advice, not a reservation**, since the host
holds nothing on the writer's behalf and an envelope sent afterwards may still be
refused for another reason — and the `too_soon` row's writer's-fix now points at
it. §7's R-14 row gains the assertion that a `too_soon` reply carries it and no
other reply does, and `design.md` §9's AC-4 row names that test.

OQ-1 is **answered, and says so**: it now records the answer, why deferring it
was the worse option, and what remains out of scope (any reservation or fairness
guarantee, which P-C forbids). `design.md` §5.2's reply example, its prose and
the `Refusal` paragraph all state that `TooSoon` is the one payload reaching the
wire as a field of its own.

**Outcome:** verified

### F-11 — D-18 makes a stratum-1 diagnostic helper permanent public API to avoid a two-line match

**Severity:** nit
**Location:** `design.md` D-18, §5.2 (*One stratum 1 touch*);
`crates/goad-semantics/src/error.rs:14-18`

**Expected:** D-18 offers one alternative — *"a second copy of it in stratum 2"* —
and rejects it.

**Observed:** there is a third. `json_type_name` is a total match on
`serde_json::Value`'s six discriminants over a type stratum 2 already depends on
directly (`serde_json` is on stratum 2's allowlist,
`crates/goad-boundary/tests/checks/allowlist.rs:19-27`). Widening it to `pub`
adds a permanent, versionless export to `goad-semantics` — a function whose own
doc comment scopes it to *"The one such table in the **crate**"* — to save a
six-arm match, and none of the four ADR-001 instruments looks at it: POL-001's
table says the crate-edge instrument sees only crate edges and the purity scan
only stratum 1's `std` reaches. The design is right that this is *"a visibility
change, not a dependency"* and that AC-11 is untouched; the point is that the
choice is between three options, not two, and the one taken is the only one that
enlarges a published surface.

**Evidence:** `crates/goad-semantics/src/error.rs:14-18`;
`crates/goad-boundary/tests/checks/allowlist.rs:19-27`;
`docs/policy/001-the-phase-gate.md` §Verification, the four-instrument table;
`design.md` D-18.

**Disposition:** doc-wrong
**Response:** The third option is real and `serde_json` is on stratum 2's
allowlist as cited, so D-18 offered two of three. **The widening stands**, and
D-18 now states the choice it actually was: a *second copy* in stratum 2 — two
tables naming the same six JSON types, free to drift, and the function's own
comment ("the one such table in the crate") false across the workspace; a *local
match* on `serde_json::Value`'s six discriminants inside `EnvelopeFault` — the
same six arms written a second time under another name; or the widening. It
beats both because the type names are a diagnostic vocabulary a person reads, and
two of them drifting is a worse defect than one export being wider than it needs
to be — a duplicate is a correctness risk, a wide export is a surface cost.

D-18 also now records what the finding's last clause establishes: **no ADR-001
instrument sees this choice** — crate edges see only crate edges, the manifest
allowlist only dependency entries, the purity scan only stratum 1's `std`
reaches, and `cargo test -p goad-semantics` rejects nothing
(`docs/policy/001-the-phase-gate.md` §Verification) — so it is held by that line
alone. `design.md` §5.2's *One stratum 1 touch* paragraph points at D-18 for the
three-way rather than restating it.

**Outcome:** verified

### F-12 — AC-6 case (ii) does not say the intervening exchange must preserve the deadline it depends on

**Severity:** minor
**Location:** `design.md` §9 (AC-6, case ii); `draft-spec.md` §7 R-12 row;
`canon-delta.md` CD-3 §Why

**Expected:** case (ii) is the one that now discharges ADR-004's debt (F-1's
repair, and CD-3 says so in terms), so its setup has to establish the situation
in which the anchor and the boolean disagree — and only that situation.

**Observed:** the row reads *"a **scheduled** firing at T₀, an ingested firing at
T₀+ε, and a `next_check` due at T₀+1 s — the scheduled evaluation does not reach
the backend before T₀+3 s."* The `next_check` is written as a single standing
fact, and it is not one. Every completed exchange re-arms the pending deadline
from the instruction *that* exchange's backend returned
(`crates/goad/src/controller.rs:507-512`, SPEC-001/R-26), and the ingested firing
at T₀+ε **is** an exchange — which is exactly the point the slice already
concedes in `slice-004.md` §Readings taken in design: *"The pending deadline still
moves after an ingested exchange, because the backend answered it with a
`next_check`."* So the deadline the assertion turns on is whatever the backend
answered the **ingested** evaluation with, not the T₀+1 s the scheduled one set.

The test is buildable — a scripted backend answering every exchange with the same
short `next_check` reproduces it — but nothing in the three artefacts says so, and
the failure mode is quiet: put the short `next_check` only on the scheduled
response and the ingested exchange re-arms the deadline to whatever the script's
default is, the assertion passes for the wrong reason, and the case ADR-004 was
waiting for is discharged on paper by a test that could not have failed under the
boolean either.

One clause fixes it: state that the ingested exchange must itself resolve to a
deadline no later than T₀+1 s, so the floor is what the two hypotheses disagree
about.

**Evidence:** `crates/goad/src/controller.rs:507-512` (`wait_for(absorbed.next_check,
requested_at)` then `deadline_after(…, floor_until)`); SPEC-001/R-26;
`slice-004.md` §Readings taken in design, the AC-6 bullet; `design.md` §9 AC-6
case (ii); `draft-spec.md` §7 R-12 row; `canon-delta.md` CD-3 §Why.

**Disposition:** fix-now
**Response:** The citation holds. `controller.rs:507-512` re-arms the pending
deadline from `wait_for(absorbed.next_check, requested_at)` after **every**
exchange, so the ingested firing at T₀+ε sets the deadline in force at T₀+1 s,
not the scheduled firing that preceded it — and `slice-004.md` §Readings already
concedes exactly that in another sentence.

One clause is added, in all three places the case is stated: the ingested firing
at T₀+ε must be one **whose own exchange resolves to a deadline no later than
T₀+1 s**. `design.md` §9's AC-6 row and `canon-delta.md` CD-3 §Why also say why
the clause is load-bearing rather than decorative — without it the deadline the
assertion turns on is one the anchor and the boolean agree about, so the test
could not have failed under the boolean either and the debt would be discharged
on paper. `draft-spec.md` §7's R-12 row carries the same setup and the same
reason in shorter form.

The sibling in `slice-004.md` §Readings is repaired rather than left standing:
the bullet that concedes the deadline moves now says what that costs the *does
not advance* case, so the plan inherits the constraint from the criterion it is
built against rather than from the design alone.

**Outcome:**

### F-13 — F-8's `None` path is not carried into `Fired`, and gives `unavailable` a third meaning three documents deny

**Severity:** minor
**Location:** `design.md` §5.2 (the `Fired` sentence, and the reply table's
`unavailable` row), §5.4 (the paragraph after the order of judgement);
`draft-spec.md` §6.3

**Expected:** the F-8 repair changed `Ingress::arrival` to return
`Option<Arrival>` and gave the loop something to do with `None` — *"it folds one
`Refused::Ingress` onto the diagnostics surface — reason `unavailable`, detail
naming that ingress has stopped — and parks the arm."* Every artefact that states
the type or the reason has to agree with that.

**Observed:** two do not, and both were left as they stood.

1. **The type.** `design.md` §5.2 still says *"`Fired` gains
   `Ingested(Arrival)`"*. `Fired::Ingested(Arrival)` cannot carry `None`, so the
   outer arm must now dispose of the closed-channel case **before** it builds a
   `Fired` — which also means it must not disturb `refusal_re_arms`
   (`controller.rs:429`, `matches!(fired, Fired::Scheduled)`) or the standing
   deadline on its way out. That is a branch in the most-reviewed function in the
   repository (§8 R2) which the design's own interface block does not describe.
2. **The reason.** §5.4 still closes with *"`unavailable` therefore means
   **exactly one thing**: the host cannot act on any envelope right now — it is
   stopping, or its clock is unreadable — and `detail` says which."* The `None`
   path is a third thing, and it is not "right now": it is permanent for the life
   of the process. The repaired §5.2 row carries only the same two causes
   (*"the loop is gone, the host is stopping (listener); or the clock is
   unreadable (loop, §5.4 step 3)"*), and `draft-spec.md` §6.3 — where the set is
   **normative and closed** — carries only *"it is stopping, or its clock is
   unreadable"*, with the writer's fix *"wait"*, which is advice that can never
   come true in this case.

The repair is right and the residue rows it added are right; what is missing is
the propagation. Either `unavailable`'s definition admits the third cause on the
page, or the diagnostics line the loop folds carries a reason of its own.

**Evidence:** `design.md` §5.2 (`arrival` doc comment, *What the loop does with
`None`*, the `Fired` sentence, the reply table); §5.4, the paragraph beginning
*"`unavailable` therefore means exactly one thing"*; `draft-spec.md` §6.3
`unavailable` row; `crates/goad/src/controller.rs:429`.

**Disposition:** fix-now
**Response:** Both halves check out, and both are propagation rather than
redesign — the F-8 repair was right and stopped one document short.

**The type.** `design.md` §5.2's stratum 3 block gains *`Fired` never sees a
closed channel*: each arm disposes of the `None` where it observes it, before
any `Fired` exists. The outer arm folds the `Refused::Ingress` and `continue`s,
so the next iteration's `glass.present` (`controller.rs:409-410`) is what puts
it in front of a person; the inner arm folds the same refusal and resumes
waiting on the exchange, where R-15's bound applies to it as to every refusal
decided inside one. Because no `Fired` is built, `refusal_re_arms`
(`controller.rs:429`) is not reached, the standing deadline is not reset and
neither anchor is written — a dead accept task changes nothing about the
schedule — and no new state is retained, so §5.3's table still holds. §5.5's
`None` row drops *"says so once"* for what is actually true: the loop **folds**
it once, and whether a person sees it is R-15's bound.

**The reason.** `unavailable`'s third cause is admitted on the page.
`design.md` §5.2 gains a paragraph naming it as the one cause that is
**permanent for the life of the process** rather than a condition of the moment;
§5.4's *"means exactly one thing"* paragraph becomes *one thing about the host
and three about why*; §5.2's reply table row carries all three deciders and
causes. Normatively, `draft-spec.md` §6.3's `unavailable` row and its
writer's-fix column now separate the two causes *wait* is sound advice for from
the one it is not, a new paragraph states the three causes and that the token
set stays closed at eight, and §5 gains *When ingress stops but the host does
not*. `slice-004.md`'s *Refusals a person cannot see* follow-up gains the one
interleaving in which this refusal is invisible.

Two consequences the finding did not name but the same class demands: the
reply table's closing sentence said *every refusal is reported to its writer*,
which the ingress-stopped cause falsifies — it is not the reply to any envelope,
so the sentence is narrowed to *every envelope's refusal* and the exception is
stated; and `draft-spec.md` §6.3's *Which refusals a person sees* paragraph took
the same correction.

**Outcome:**

### F-14 — §5.4's sequence diagram has the inner arm answering `engaged` unconditionally, which three repaired claims now contradict

**Severity:** minor
**Location:** `design.md` §5.4 (the `sequenceDiagram`, `else` branch) against
§5.4 (*Order of judgement*), §5.2 (the reply table's new column),
`draft-spec.md` §5 and §6.3, and `draft-spec.md` §7's R-15 row

**Expected:** shape beats state, in both artefacts and in both `select!`s.
`design.md` §5.4: *"Shape refusals take precedence over state refusals: a
malformed envelope is malformed regardless of timing."* `draft-spec.md` §5 says
the same normatively.

**Observed:** the diagram's `else` branch reads

```
  else an exchange in flight
    Note over S: inner arm — answered at once
    S-->>L: Refused(engaged)
```

— state, unconditionally, with no shape step before it. That was survivable while
nothing turned on it. The F-2 repair made three claims that do turn on it, all
requiring a **shape** refusal to be decidable *inside* an exchange:

- `design.md` §5.2's new column, which says a shape refusal reaches a person
  *"when the loop was idle"* — **and not otherwise**;
- `draft-spec.md` §6.3's new paragraph, *"a shape refusal reaches it when the
  host happened to be idle and not otherwise"*;
- `draft-spec.md` §7's R-15 row, whose negative half is *"the same refusal
  decided during an exchange does **not**"* — which is unbuildable if the inner
  arm answers `engaged` to everything, because then no shape refusal is ever
  decided during an exchange and the test has nothing to assert against.

Two readings are now live in one document, and the newer one is normative.

**Evidence:** `design.md` §5.4 sequence diagram, `else` branch; §5.4 *Order of
judgement*, step 1; §5.2 reply table, *reaches a person?* column;
`draft-spec.md` §5 *Order of judgement*, §6.3 *Which refusals a person sees*, §7
R-15 row.

**Disposition:** fix-now
**Response:** The diagram was the stale reading, and R-15's negative test is
unbuildable without the repair. Shape now beats state in both artefacts and in
both `select!`s.

`design.md` §5.4's *Order of judgement* is restated as **the ingress arms'**,
plural: step 1 is a shape refusal *in the inner arm as much as the outer* — an
exchange being in flight does not turn a malformed envelope into `engaged` —
step 2 is `engaged` and is the inner arm's only state answer, and steps 3–5
belong to the outer arm alone. The paragraph says in terms that step 1 in the
inner arm is what makes `draft-spec.md` R-15's negative test buildable.
`draft-spec.md` §5's normative sentence gains the same clause explicitly.

The diagram was checked end to end rather than at the one branch, and it
disagreed in more than one place: it showed two of the four steps and had no
`too_soon` and no clock branch at all. It is redrawn as five `alt` branches in
the order of judgement's own order, with the reply drawn once below the `alt`
because I-1 makes it one door in every branch, and with the anchor written in
both branches that attempt an evaluation (D-15).

Siblings taken with it: `design.md` §5.2's reply table row for `engaged` and
`draft-spec.md` §6.3's now say *an exchange in flight **and the envelope's shape
was good***, so no table states the old unconditional reading. One consequence
of the renumbering: `design.md` §5.2's `unavailable` row cited *§5.4 step 3* and
now cites step 4. Finding bodies above that cite the old numbers are historical
and are not edited.

**Outcome:**

### F-15 — R-15's MUST puts a `glass.present` on the UI thread for every refused envelope, at a machine-rate writer's pace

**Severity:** minor
**Location:** `draft-spec.md` R-15; `design.md` §5.1 (the model), §9 AC-5;
`crates/goad/src/controller.rs:409-410`

**Expected:** the repaired R-15 is a **MUST**: *"A refusal the host decides while
no exchange is in flight MUST also be reported on the host's own diagnostics
surface."* `too_soon` is decided only while idle, so every `too_soon` is covered.

**Observed:** the only way the loop reports one is `controller.refuse` followed by
`continue`, and `continue` returns to the top of the loop, whose first statement
is `glass.present(controller.frame())` (`controller.rs:409-410`). So each refused
envelope costs one full presentation — every Slint property the frame carries,
plus the tray icon and tooltip `SlintGlass::present` re-hands — on the main
thread, driven by whoever is writing to the socket.

AC-5 tests exactly this: *"a writer emitting flat out"*. Before this slice the
host's refusals came from a person's clicks or from a 3 s timer; ingress is the
first machine-rate refusal source, and R-15 now makes presenting each one
mandatory. That is a new path from an untrusted writer to the rate at which the
UI thread does work, and nothing measures it — §8's risk table worries about
`serve` outgrowing review (R2) and about timed-test margins (R4), not about this.
`ENVELOPE_LIMIT` and `ENVELOPE_DEADLINE` bound the listener's reads; nothing
bounds the presentations the loop makes on their behalf.

The observation is not that the repair was wrong — R-15 needed a floor, and this
is the shape the loop already has. It is that the repair introduced a rate
coupling that no artefact states, in the one place the design otherwise takes
care to say what an untrusted input can and cannot make the host do (I-3).

**Evidence:** `crates/goad/src/controller.rs:409-410`; `draft-spec.md` R-15
(MUST) and §6.3's *Which refusals a person sees*; `design.md` §9 AC-5, §8 R2 and
R4; `crates/goad/src/glass.rs:67-120` — one `present` clones the title, the
styled body and the option rows, rebuilds two `VecModel`s, writes ten window
properties, sets the tray image and tooltip, and calls `show()`/`hide()`.

**Disposition:** settle-in-code
**Response:** Both citations hold — `controller.rs:409-410` is the loop's first
statement and `refuse` + `continue` reaches it, and `glass.rs:67-120` is the
cost. R-15's MUST **stands**; it is not weakened to duck the price of holding
it. What was missing is that no artefact said an untrusted writer now paces the
UI thread's work, and no argument settles that — a number does.

**The phase that settles it:** the phase that builds `serve`'s ingress arms and
the anchor. **The test that settles it:** AC-5's flat-out writer, extended to
assert a bound on **presentations** over the window, not only on invocations.

Stated in three places so the phase inherits it rather than rediscovering it.
`design.md` §5.5 gains a paragraph immediately after the invariants table —
beside I-3, which is where the design otherwise says what an untrusted input can
and cannot make the host do — naming the path from a refused envelope to a full
presentation and saying that `ENVELOPE_LIMIT` and `ENVELOPE_DEADLINE` bound the
reads and nothing bounds the presentations made on their behalf. §8 gains **R6**
as a named risk, whose mitigation is the measurement rather than a retreat from
R-15. `design.md` §9's AC-5 row and `draft-spec.md` §7's R-12 row — the row that
carries AC-5 — both state the extension.

Per this ledger's Protocol, a `settle-in-code` finding that **survives its
phase returns here `contested`**: if the extended AC-5 cannot bound the
presentations, the disposition has failed and F-15 is open again, not closed by
having been dispositioned.

**Outcome:**

### F-16 — `retry_after_ms`'s rounding is unspecified, and the obvious implementation falsifies R-14's own words

**Severity:** nit
**Location:** `draft-spec.md` R-14, §6.3

**Expected:** R-14 defines the new field as *"a whole number of milliseconds,
measured at the moment of refusal, **after which the spacing will have
elapsed**"* — a claim about what is true once the writer has waited that long.

**Observed:** the natural implementation is
`retry_after.as_millis()`/`subsec_millis()`, which **truncates**. A remaining
spacing of 1800.6 ms becomes 1800, and a writer that waits exactly 1800 ms is
still inside the spacing and is refused `too_soon` again — with a value that
rounds to 0 or 1, and a second retry. R-14's sentence is then false of the host's
own field, on every refusal whose remainder is not a whole millisecond, which is
almost all of them.

`ceil` makes the sentence true and costs one method name. Worth pinning in the
requirement rather than leaving to the phase, because §6.3 already promises the
value is *"advice, not a reservation"* and this would make it advice that is
reliably one round-trip short.

**Evidence:** `draft-spec.md` R-14 (the second sentence), §6.3 (`retry_after_ms`
paragraph and the `too_soon` writer's-fix).

**Disposition:** fix-now
**Response:** Correct: truncation makes R-14's own sentence false of the field
on almost every refusal. R-14 now requires the value **rounded up**, and says
why in the requirement rather than leaving it to the phase — a truncated
remainder leaves a writer that waits exactly that long still inside the spacing.

The siblings are repaired with it, since the rounding is stated in four places
between the two documents: `draft-spec.md` §6.3's `retry_after_ms` paragraph
says the value is rounded up so that waiting exactly that long is outside the
spacing; §7's R-14 row asserts the rounding rather than only the field's
presence; and `design.md` §5.2's reply prose and §9's AC-4 row say the same, so
no artefact describes the field without its rounding.

**Outcome:**

### F-17 — SPEC-002's new requirement and SPEC-003's are both `R-12`, and each document cites "R-12" bare while pointing at the other

**Severity:** nit
**Location:** `canon-delta.md` CD-1 (§Kind, and the new §2 Boundaries block);
`draft-spec.md` §2 Boundaries, R-12, §9 References

**Expected:** ids are immutable and append-only, so both numbers are forced —
CD-1 states *"appended as **R-12**; R-11 is the current highest id"* for
SPEC-002, and the draft's own R-12 already exists. Nothing can renumber. What is
avoidable is the ambiguity that follows.

**Observed:** the two requirements are about the same behaviour from the two
sides of one seam, they cite each other, and both documents refer to *"R-12"*
unqualified in text that points at the other document:

- `draft-spec.md` §2: *"the spacing SPEC-002 requires of an ingested evaluation
  is what **R-12 below** makes visible at the socket"* — SPEC-003/R-12.
- CD-1's new §2 Boundaries wording, destined for SPEC-002: *"the ingested spacing
  **R-12** requires is what SPEC-003 makes visible at the socket"* —
  SPEC-002/R-12.

Each is locally correct under the doc-local convention and misleading to anyone
reading across, which is the only way these two are ever read. `draft-spec.md`
§9's SPEC-002 line still says *"the requirement that states the ingested spacing
itself"* without a number — filling that in as "R-12" is the natural next edit
and is the one that lands the collision in a References section.

Qualifying every cross-document mention (`SPEC-002/R-12`, `SPEC-003/R-12`) is the
whole fix.

**Evidence:** `canon-delta.md` CD-1 §Kind and the §2 Boundaries block;
`draft-spec.md` §2 Boundaries, R-12, §9 References;
`docs/specs/002-host-scheduling-behaviour.md` header (*"Cite from elsewhere as
SPEC-002/R-N"*).

**Disposition:** fix-now
**Response:** Both numbers are forced and neither may move, so qualification is
the whole fix. Every **cross-document** mention is now qualified and every
doc-local bare `R-n` is left as it was.

`draft-spec.md` §2 Boundaries reads *the spacing **SPEC-002/R-12** requires … is
what **this spec's own R-12** (below) makes visible at the socket*, and states
the convention once so a later editor does not undo it; R-12 itself now names
`SPEC-002/R-12` as the requirement that sets the spacing it reports; §5's *the
bound is SPEC-002's* becomes `SPEC-002/R-12`'s; and §9 References fills in the
line that pointed at *"the requirement that states the ingested spacing itself"*
with `SPEC-002/R-12`, which was the edit that would otherwise have landed the
collision in a References section. `canon-delta.md` CD-1 §Kind states the
convention for the entry and §2 Boundaries' destined wording carries both ids
qualified. Two siblings outside the finding's list took the same edit:
`design.md` §10's CD-1 row (*plus **SPEC-002/R-12** as its ingested instance*)
and `research.md`'s amendment-candidate bullet.

**On the draft's own number — checked, and reported rather than changed.**
`docs/AGENTS.md` says a draft spec is numbered only at promotion.
`draft-spec.md` complies: it is titled *SPEC-NNN*, its banner says it takes its
number at promotion, and **nothing in its own text calls it SPEC-003**. The one
occurrence inside the file is `SPEC-003/R-4` in the template's citation-form
comment, which is the template's example and not a self-reference. So the
premature-numbering case the brief anticipated does not arise, and the
cross-references in §2 and §9 are written in forms that survive the draft being
given any number — *this spec's own R-12*, and `SPEC-002/R-12` for the other
side. `design.md` and `canon-delta.md` do name SPEC-003, but only as the number
the draft takes on promotion and where the sentence is the wording destined for
canon; CD-1's block now says so in terms.

**Outcome:**


## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

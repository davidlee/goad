# Review — design — Slice 001

**Subject:** design — `docs/slices/001/design.md`, as at the tree state of
2026-08-23, together with the acceptance criteria in `slice-001.md` it claims to
discharge
**Reviewer:** fresh agent via codex MCP (GPT-5.5 Sol, default model), thread
`01a02caa-83d2-7950-a2a7-60c2bdc017e0` — reusable for further rounds
**Opened:** 2026-08-23
**State:** open — round 1 closed (F-1…F-22 all `verified`), round 2 findings
F-23…F-38 raised and awaiting disposition
**Rounds:** 1 — 22 findings, 1 blocker. 2 — 15 findings from the reviewer, 1
blocker, plus F-38 raised by me while verifying F-31.

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
past the gate. Reject a finding on **evidence**, never on assertion. Confirm each
disposition with the user before acting on it. Fix the class, not the instance,
and do not introduce new defects repairing old ones.

## Brief

Written before the review ran.

This design makes a large number of calls the brief does not settle, on a
codebase that does not exist yet. That combination is the risk: there is no code
to contradict a wrong decision, and no canon older than this slice to appeal to.
So the review is not looking for typos in signatures. It is looking for places
where the design has **talked itself into a position** that a reader in slice 002
or 005 will have to undo.

**Invariants the design is held to.**

1. Every acceptance criterion in `slice-001.md` is actually discharged by
   something in §9 — not by a plausible-sounding sentence elsewhere.
2. No decision in §7 contradicts another decision in §7.
3. Nothing in stratum 1 as designed can perform I/O, read a clock, or need a
   runtime (ADR-001, invariant I3).
4. Nothing designed here requires a protocol change in order for slice 002 to
   render, slice 003 to schedule, slice 004 to ingest an event, or slice 005 to
   add the socket transport. This is the slice's whole thesis; a breach is a
   blocker.
5. Where the design departs from `docs/brief.md`, it says so and says why. A
   silent departure is worse than a stated one.

**Where the bodies are likely buried.** Named in advance so the reviewer is not
credited for finding only what was easy:

- **P2's granularity rule** (§4). "A part may be discarded only when its absence
  is already a modelled state with defined semantics." Is that test sound, or
  does it license discarding more than intended? Apply it to `title`, to
  `options`, to `body`, and see whether the answers stay right.
- **AC-6 discharged through `discarded` rather than `Err`** (§5.2). AC-6 says
  each failure mode "maps to a distinct typed error". The design claims that
  arriving inside `Normalized::discarded` satisfies this. Reading or stretch?
- **AC-5 versus D18.** AC-5 requires stderr captured into diagnostics; D18
  accepts that a timed-out backend yields no stderr. Is AC-5 then false as
  written, and should it have been amended rather than reinterpreted?
- **Brief §13 versus D19.** The brief says a backend failure must not take down
  the host. D19 accepts an unbounded read that can OOM the host. Is "accepted
  with a follow-up" a legitimate disposition for a stated brief requirement, or
  is it a scope decision the user should have been asked to make explicitly?
- **`resolved_check` is not an `Option`** (§5.3) but brief §9 says "retain an
  existing valid scheduled check *if one exists*", and AC-4 takes an existing
  schedule as a parameter. Does the non-optional field quietly delete a state the
  brief distinguishes?
- **Clamping a past `next_check` to `now`** (§5.5). Brief §9 defines `next_check`
  as "do not evaluate before this point", which a past instant satisfies
  trivially. Is clamping an invention the host has no business making?
- **D16, replacement over queueing.** Brief §12 says "reject or ignore stale
  responses clearly" and "allow only one active interaction". Does replacement
  follow, or is it one reading among several?
- **D7's protocol-version asymmetry** and **D15's non-zero-exit precedence.**
  Both are choices presented as derivations. Check whether the brief actually
  supports them.
- **The claim that validation feedback is additive** (§5.5). Three properties are
  said to make it so. Try to construct a validation-feedback design that the
  types as specified cannot express without a version bump.
- **`Options` is reused** for `Choice.options` and `FieldKind::Choice.options`.
  Do those two actually have the same invariants?
- **`Outcome`'s stratum is never stated.** It is returned by `Host`, mentions
  views and diagnostics, and no section says which module owns it.
- **`Event` is mandatory on `evaluate`.** Is there a case — first startup, a
  manual poll — where there is no event, and what does the host send then?
- **Gold-plating in the type surface** (R7). The `FieldKind` and `Content`
  vocabularies are the largest thing being built and nothing renders them. Is
  every variant traceable to the brief, or did some arrive by symmetry?

**Round 1** — 2026-08-23 — the whole design document, against `docs/brief.md`,
ADR-001, ADR-002, `slice-001.md` and `research.md`.

## Findings

Raised by the reviewer in round 1. Severities are the raiser's and are not
negotiated. Dispositions are filled in by the responder after confirmation with
the user.

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | `fix-now` | `verified` |
| F-2 | major | `fix-now` | `verified` |
| F-3 | major | `fix-now` | `verified` |
| F-4 | major | `doc-wrong` | `verified` |
| F-5 | major | `doc-wrong` | `verified` |
| F-6 | major | `fix-now` | `verified` |
| F-7 | major | `doc-wrong` | `verified` |
| F-8 | major | `fix-now` | `verified` |
| F-9 | major | `fix-now` | `verified` |
| F-10 | major | `fix-now` | `verified` |
| F-11 | major | `fix-now` | `verified` |
| F-12 | major | `fix-now` | `verified` |
| F-13 | minor | `fix-now` | `verified` |
| F-14 | minor | `fix-now` | `verified` |
| F-15 | minor | `fix-now` | `verified` |
| F-16 | nit | `aligned` | `verified` |
| F-17 | nit | `aligned` | `verified` |
| F-18 | nit | `aligned` | `verified` |
| F-19 | nit | `aligned` | `verified` |
| F-20 | nit | `aligned` | `verified` |
| F-21 | nit | `aligned` | `verified` |
| F-22 | nit | `aligned` | `verified` |
| F-23 | blocker | | |
| F-24 | major | | |
| F-25 | major | | |
| F-26 | major | | |
| F-27 | major | | |
| F-28 | major | | |
| F-29 | major | | |
| F-30 | major | | |
| F-31 | major | | |
| F-32 | major | | |
| F-33 | major | | |
| F-34 | minor | | |
| F-35 | minor | | |
| F-36 | minor | | |
| F-37 | minor | | |
| F-38 | major | | |

### F-1 — The transport interface does not admit a stateful persistent socket without redesign

**Severity:** blocker
**Location:** `design.md` §5.2 D11; §5.3

**Expected:** The transport abstraction established in slice 001 must let slice 005 add an ordered persistent JSONL socket transport without changing the interface or introducing an unplanned state-management model.
**Observed:** `Backend::exchange` takes `&self`, and the ownership table says backend `B` is “never mutated.” A persistent socket must mutate connection, framing-buffer, and reconnection state. Slice 005 must therefore change the trait to `&mut self` or introduce interior mutability absent from — and contrary to — the stated ownership design.
**Evidence:** `design.md` §5.2 defines `fn exchange(&self, ...)`; §5.3 says backend `B` is “never mutated”; brief §6.1 requires a maintained ordered JSONL stream; review invariant 4 classifies a change needed for slice 005 as a blocker.

**Disposition:** `fix-now`
**Response:**

Accepted. `exchange` now takes `&mut self` (design §5.2, D22), and §5.3's
ownership table no longer claims the backend is never mutated.

The finding's own framing is worth one correction, not a contest: this is an
*interface* change rather than a *protocol* change, so brief-invariant 4 is
stretched slightly by the wording. It makes no difference to the disposition. The
argument that does the work is P3's — a seam justified by a named future
implementation has to fit that implementation, and slice 005's socket holds
mutable connection state. `&self` would have forced either interior mutability
guarding against concurrency brief §12 says does not exist, or a trait change in
the slice where two implementors already depend on it. Today it is one keyword.

**Outcome:** `verified` (round 2)

### F-2 — Unbounded stdout permits a backend failure to take down the host

**Severity:** major
**Location:** `design.md` §5.5, D19, R3

**Expected:** Backend failures must not take down the host.
**Observed:** The design deliberately leaves `wait_with_output()` reading stdout without a bound, allowing a printing backend to exhaust host memory.
**Evidence:** `brief.md` §13: “A backend failure must not take down the host.” `design.md` §5.5: “a backend stuck in a print loop can exhaust host memory”; D19: “accept: unbounded stdout read.”

**Disposition:** `fix-now`
**Response:**

Accepted, **reversing D19**, which the user had previously approved.

The deferral argued that F-2 and F-3 are one refactor and that no single AC
demanded both at once. The first half is true and the design still says so. The
second half was optimising slice size against a stated requirement: brief §13
says a backend failure must not take down the host, and an OOM is the host going
down. That is not a wart to be scheduled.

`wait_with_output()` is replaced by a manual bounded drain — stdout capped at
8 MiB and fatal on exceed (`BackendError::OutputTooLarge`), stderr capped at
256 KiB and truncated rather than fatal. The asymmetry is deliberate and is
argued in §5.4: truncated stdout would parse as malformed JSON and name the wrong
fault, while truncated stderr is still a useful diagnostic. Caps are constants,
not config keys — brief §5 names no such keys and P3 forbids inventing them.
Recorded as D27; R3 in §8 is closed rather than mitigated; I11 added.

**Outcome:** `verified` (round 2)

### F-3 — AC-5 is false for timed-out backends

**Severity:** major
**Location:** `design.md` §5.4 D18; §9 AC-5

**Expected:** The process transport captures stderr into diagnostics while enforcing its timeout.
**Observed:** The timeout path discards the `wait_with_output()` future and all accumulated stderr, yet §9 claims AC-5 is discharged by testing that “stderr [is] captured.”
**Evidence:** `slice-001.md` AC-5: “enforces a timeout, and captures stderr into diagnostics.” `design.md` §5.4: “A timed-out backend yields no stderr.” §9 maps AC-5 to “timeout, stderr captured.”

**Disposition:** `fix-now`
**Response:**

Accepted, **reversing D18**, and the same refactor as F-2.

Stderr now drains in its own task, so the buffer outlives a timeout on the
exchange future, and `BackendError::Timeout` carries it. The mechanism matters and
is stated in §5.4: killing the child closes the pipe, the drain task reaches EOF,
and the partial buffer arrives through the join handle — no shared mutex needed.
Where it can still stall is stated too, rather than glossed: a grandchild that
inherited the stderr fd keeps the pipe open, so the join takes a short grace
timeout and the host reports the timeout without stderr rather than waiting on a
process it does not manage. We do not kill process groups; brief §14 makes
backends trusted user programs, and reaching past the process we spawned is a
larger claim over the user's machine than this slice should make. Recorded as
D26; R8's impact drops; a stderr-then-sleep backend is added to §9's
misbehaving-backend list.

**Outcome:** `verified` (round 2)

### F-4 — The granularity rule contradicts its treatment of optional body content

**Severity:** major
**Location:** `design.md` §4 P2; §5.5

**Expected:** P2’s discard rule must produce consistent answers for `title`, `options`, and `body`.
**Observed:** `body` is explicitly modelled as `Option<Content>`, so its absence is a modelled state with defined semantics and passes P2’s stated test. Nevertheless, the design says every invalid `view` or `choice` structure rejects the whole response. Discarding a malformed body would also falsely assert “no context”, the same invented-semantics argument used to forbid discarding an invalid view.
**Evidence:** `design.md` §4: “A part may be discarded on its own only when its absence is already a modelled state with defined semantics”; §5.2 defines `body: Option<Content>`; §4’s outcome table says invalid “view or choice structure” rejects the whole message.

**Disposition:** `doc-wrong`
**Response:**

Accepted; the design's rule was the defect. P2's granularity test now has two
clauses (§4): a part may be discarded on its own only when its absence is a
modelled state **and** the protocol is what specifies the behaviour in that
absence.

The second clause is what the one-clause version was missing, and the reason is
general rather than about `body`: every `Option` field has an absent state, so
clause 1 alone admits all of them. The question is whether that state's meaning
was *specified* or *improvised*. `next_check` passes because brief §9 names the
fallback. `body` passes clause 1 and fails clause 2 — nothing says what a renderer
shows in place of a body that was sent and could not be read, so dropping it
renders a view the backend did not author. The design now says so explicitly, and
the outcome table carries the row.

**Outcome:** `verified` (round 2)

### F-5 — Omitted `view` is silently converted into the positive `view: null` assertion

**Severity:** major
**Location:** `design.md` §5.2; I10

**Expected:** Absence must mean only “not supplied”, while `view: null` carries the backend’s positive assertion that nothing should be shown.
**Observed:** `Option<WireView>` with `#[serde(default)]` deliberately makes an omitted `view` indistinguishable from explicit null, then canonicalizes both to “nothing to show”. This contradicts both P2’s stated treatment of `view: null` and I10.
**Evidence:** `design.md` §4: “`view: null` … is a positive assertion”; §5.2: “`view` absent and `view: null` are the same state”; I10: “the absence of a field never means more than ‘not supplied’.” Brief §8.2 demonstrates explicit `"view": null`.

**Disposition:** `doc-wrong`
**Response:**

Accepted, and the fix is stronger than tolerating the conflation: `view` is now a
**required** field. Absent yields `MissingField { field: "view" }` and rejects the
message; explicit `null` is accepted and means nothing to show. D25.

Two things this surfaced that the finding did not have to:

The wire shape is load-bearing and was verified rather than assumed. A bare
`#[serde(default)] Option<Option<T>>` does **not** distinguish absent from null —
serde's `Option` deserializer maps `null` to `None` at every nesting level, so the
distinction is silently lost. Confirmed by running it. The design now carries the
`deserialize_with` helper that supplies the outer layer explicitly, with a comment
saying why it exists.

I10 needed a word. It said "the absence of a field never means more than 'not
supplied'", which a required field contradicts. Narrowed to *unmodelled* fields,
which is what it was always about: permissiveness concerns fields the host does
not model, never the meaning of fields it does.

**Outcome:** `verified` (round 2)

### F-6 — Unsupported nested semantic primitives have no named-error contract

**Severity:** major
**Location:** `design.md` §5.2; §9 AC-2 and AC-6

**Expected:** Every unknown required semantic primitive must be rejected with the named `UnsupportedPrimitive` error.
**Observed:** The design specifies special handling only for an unknown `view.kind`. It does not specify how unknown `FieldKind` or `Content` variants avoid becoming generic serde errors, despite admitting both vocabularies into the protocol.
**Evidence:** `slice-001.md` AC-2 requires any unknown required semantic primitive to be rejected with a named error; AC-6 requires unsupported required primitives to map distinctly. `design.md` §5.2 limits its explicit contract to “An unrecognised `view.kind`”.

**Disposition:** `fix-now`
**Response:**

Accepted, and fixed as a class rather than as the instance. The contract is now
stated over the whole document: **any** unrecognised `kind` discriminant, at any
depth, yields `UnsupportedPrimitive { kind, at }` and rejects the message. D8
revised.

The path is the part worth adding beyond what the finding asked for. There are
three discriminant sites — the view, each field, each content block — and
`unsupported primitive "slider"` is a puzzle in a view with nine fields where
`… at view.options[1].fields[2].kind` is not. Brief §13 wants failures debuggable,
and at depth that requires the location. The path is a diagnostic string, not an
interpreted value, so P1's scope leaves it a `String`; how normalization
accumulates it stays implementation. §9 gains a fixture with an unknown kind
nested inside a field, asserting the reported path.

**Outcome:** `verified` (round 2)

### F-7 — The validation-feedback extension is not additive under the design’s own semantics

**Severity:** major
**Location:** `design.md` §5.5; I10

**Expected:** A protocol extension claimed to require no version bump must remain semantically safe when read by an older host.
**Observed:** An older host will ignore proposed `field.error`, `field.value`, and form-level fields. The design simultaneously states that a validation error is required semantics which a renderer must not be free to ignore. Without version or capability negotiation, an older host silently drops required semantics.
**Evidence:** `design.md` §5.5: “No protocol version bump”; I10 requires unknown inbound fields to be tolerated; the same section says “a renderer … must not be free to ignore a validation error.”

**Disposition:** `doc-wrong`
**Response:**

Accepted; the claim was overstated and the design said so too confidently.

"No protocol version bump" was wrong for the reason given: tolerating an unknown
field is not honouring it. An older host that ignores `field.error` shows the user
a form with no sign that anything was rejected, which is worse than refusing the
message. Additive at the wire is not additive in meaning, and the second is what
matters to the person looking at the screen.

The conclusion is narrowed to what actually holds and is still sufficient:
validation feedback needs **no breaking restructure** — no change to how a view is
shaped, how responses are addressed, or how values are carried — and it will need
either a version bump or a capability declaration so a backend can tell whether
the host will honour what it sends. That is what the versioned envelope is for,
which is why D7 carries the version from day one. The correction is left visible
in §5.5 rather than quietly rewritten, since the original reasoning is the kind
that recurs. The draft spec records the same thing as OQ-1 and OQ-2.

**Outcome:** `verified` (round 2)

### F-8 — AC-8 has no error type in the specified public API

**Severity:** major
**Location:** `design.md` §5.2; §9 AC-8

**Expected:** Unknown and stale `view_id` values must be rejected with a named typed error.
**Observed:** `respond` returns a bare, undefined `Outcome`; neither `ProtocolError` nor `BackendError` contains a stale/unknown interaction variant, and no host-state error taxonomy is defined.
**Evidence:** `slice-001.md` AC-8: “rejected with a named error.” `design.md` §5.2 lists every shown variant of `ProtocolError` and `BackendError`, none concerning `view_id`, while `Host::respond(...) -> Outcome`.

**Disposition:** `fix-now`
**Response:**

Accepted. AC-8 required a stale `view_id` to be rejected and the taxonomy had
nothing to name the rejection with — the only candidate was `BackendError`, which
would have blamed a backend that had not been consulted.

`StateError` is added in `shell/error.rs` with two variants, `NoOutstandingView`
and `StaleViewId`, because the diagnostics genuinely differ: "there is no
interaction open" and "you answered the previous one" are different mistakes with
different fixes. D24.

`Outcome` is also now defined rather than described (D23), and its shape is an
argument, not a formality: it is a struct with an `Option<Failure>` rather than a
`Result`, because **every** call resolves a `next_check`, failures included. Brief
§9's fallback is not conditional on the exchange working, and a `Result` would
have put that instant on the success side and made the caller reconstruct it on
the error path. `view: None` with `failure: Some(_)` is exactly "we could not
tell", which §4 argues must stay distinguishable from "there is nothing to show".
I12 records it.

**Outcome:** `verified` (round 2)

### F-9 — The canonical types are publicly constructible in invalid states

**Severity:** major
**Location:** `design.md` §5.2; I1

**Expected:** Canonical values must be constructible only through successful normalization.
**Observed:** The proposed canonical structs expose public fields, permitting direct construction. `FieldKind::Number` can contain `NaN` bounds or `min > max`; `Content::Uri` can contain an arbitrary string; and the shown `Choice`, `Opt`, and `Field` structures bypass normalization entirely.
**Evidence:** `design.md` §4 P1: “a distinct type that cannot be constructed except by normalization succeeding”; §5.2 declares the canonical struct fields `pub`; I1 says no canonical type can hold an unnormalized value.

**Disposition:** `fix-now`
**Response:**

Accepted, with a scope the user settled explicitly.

**Enforcement.** Canonical fields become `pub(super)` with accessors, so outside
`semantics::protocol` a canonical value can only have come out of
`normalize_response`. That is P1 with a compiler behind it rather than a comment.
`NumberRange` replaces the bare `min`/`max` pair with a checked constructor
rejecting non-finite bounds and inverted ranges, and `BoundsError` joins
`ProtocolError`. Bounds are semantics under brief §3.4 and constrain which answers
are valid, so an inverted range makes every answer invalid and `NaN` makes every
comparison false — neither is a state the protocol has a meaning for. D30.

**Scope.** P1 is now stated as governing the values the host *interprets* —
instants, bounds, identifiers, discriminants — and not the payloads it merely
carries: `Content::Uri`, `hints`, `Event.data`, the response `values` map. Without
that boundary P1 demands a URI parser for a string nothing dereferences, to
satisfy a principle whose purpose is protecting decisions the host does not make.
The boundary is explicitly not permanent: the moment the host interprets one of
them, it comes under P1. D31, and the cost is stated — a reader must now ask which
side of the line a value falls on. R10 carries the boilerplate-erosion risk.

**Outcome:** `verified` (round 2)

### F-10 — The jiff-based duration contract omits parse-success/resolution-failure cases

**Severity:** major
**Location:** `design.md` §5.2, A3; `design-log.md` date/time entry

**Expected:** Every accepted relative duration must normalize to a canonical instant or produce a correctly typed scheduling error.
**Observed:** The design parses the full friendly grammar into `jiff::Span` and applies `Timestamp::checked_add`, but the error taxonomy has no arithmetic, overflow, or unsupported-calendar-unit variant. jiff accepts spans containing days and months, while `Timestamp::checked_add(Span)` rejects non-zero units larger than hours.
**Evidence:** `design-log.md`: “relative forms parse to `jiff::Span` and resolve as `now.checked_add(span)`.” jiff 0.2.35 `span.rs` documents accepted forms including days and months; `timestamp.rs::checked_add_span` rejects `span.smallest_non_time_non_zero_unit_error()`. `design.md` §5.2 lists only `NotAString`, `MissingOffset`, and `Unparseable`.

**Disposition:** `fix-now`
**Response:**

Accepted, and confirmed empirically before disposing of it, because the finding's
severity depends on which of several failure shapes jiff actually produces.

What was observed against jiff 0.2.35: `"1 day"` parses to a `Span` of P1D and
then **fails** at `Timestamp::checked_add`, so the naive path rejects a duration
the design's own examples imply is fine. `SignedDuration` rejects it outright.
`SpanRelativeTo::days_are_24_hours()` resolves days and weeks exactly — `1 day` →
PT24H, `1 week` → PT168H, `1d 2h` → PT26H — and rejects months and years cleanly,
with no tzdb, so it stays legal in stratum 1 under D4.

So: resolution converts through `days_are_24_hours()`, and `ScheduleError` gains
`CalendarUnit` and `OutOfRange`. Both are discards under P2, not message
failures. D28, with the behaviour written into §5.5 and the fixtures into §9. The
original design said only "parses with jiff", which assumed a total function that
does not exist — that assumption, rather than the missing variants, was the
actual defect.

**Outcome:** `verified` (round 2)

### F-11 — The required draft protocol specification does not exist

**Severity:** major
**Location:** `design.md` §1; `slice-001.md` AC-13

**Expected:** Once the slice determines that missing protocol canon is needed, a draft specification must exist as the slice’s working authority.
**Observed:** The design says `docs/slices/001/draft-spec.md` is written incrementally and authoritative during execution, but the file is absent.
**Evidence:** `docs/AGENTS.md`, Documentation: “If the canon this slice needs is missing, … start the draft now”; `slice-001.md` AC-13 requires the file to exist; the current `docs/slices/001/` listing contains no `draft-spec.md`.

**Disposition:** `fix-now`
**Response:**

Accepted. `docs/slices/001/draft-spec.md` now exists, written from
`docs/templates/spec.md`: 47 requirements as `R-1` … `R-47`, every one appearing
in §7 with the test that verifies it, three principles, the wire formats, and
three open questions. No SPEC id is assigned — numbering happens at promotion,
per AC-14, and the status line says `draft`.

Worth recording rather than just doing: writing it found things the design had
not stated as requirements at all. R-30 (the backend must not supply a `view_id`
and the host must not accept one), R-34 (rejecting a stale response must not clear
the outstanding interaction) and R-47 (every refusal is reported) were implicit in
the design and are now falsifiable. That is the argument for the draft existing
during execution rather than at audit, which is what OQ-1's revised answer
claimed and this is the first evidence for it.

**Outcome:** `verified` (round 2)

### F-12 — The required canon delta is acknowledged but absent

**Severity:** major
**Location:** `design.md` §10

**Expected:** A slice that changes an existing canon record must carry `canon-delta.md` naming the document, the section, the proposed wording, and the reason.
**Observed:** The design says ADR-001 “needs a line”, identifies `canon-delta.md` as owed, but no such file exists.
**Evidence:** `docs/AGENTS.md`, *Canon that does not exist yet, or must change* requires `canon-delta.md`; `design.md` §10 records it as “owed”; the current slice directory contains no `canon-delta.md`.

**Disposition:** `fix-now`
**Response:**

Accepted. `docs/slices/001/canon-delta.md` now exists with one entry, CD-1,
against ADR-001's Verification section: the exact replacement text, the reason,
and what AC-15 does *not* cover stated inside the amendment itself rather than
only in the slice.

It also records what was considered and owes no delta — ADR-002 (its Verification
asked for the trigger check to be recorded in the design, and §3 records it), the
brief (intent, not canon), and the root `AGENTS.md` (a deliverable, not canon).
That negative list is the part that makes the file reviewable: without it, a
reader cannot tell whether ADR-002 was considered or overlooked.

**Outcome:** `verified` (round 2)

### F-13 — Past scheduling instructions are rewritten without support from the brief

**Severity:** minor
**Location:** `design.md` §5.5

**Expected:** A valid `next_check` instant means only that routine evaluation must not occur before that instant.
**Observed:** The design changes every past instant to `now`, altering the backend’s canonical instruction even though a past instant already imposes no remaining constraint.
**Evidence:** `brief.md` §9: “Do not perform the routine scheduled evaluation before this point.” `design.md` §5.5: past `next_check` “resolution clamps to no earlier than `now`”.

**Disposition:** `fix-now`
**Response:**

Accepted. The clamp is removed: a past `next_check` is stored as given. D29.

The finding is right about the principle, and it is the same principle P2 states
one level up — clamping is the host silently rewriting the backend's instruction
to one it prefers, which is precisely the invented semantics brief §3.3 forbids.
A past instant has an obvious and correct meaning: the next check is due, which
slice 003's timer expresses by firing immediately. A minimum wake interval, if one
is ever wanted, is a host policy that belongs where the timer is, applied visibly
rather than folded into normalization. R-28 in the draft spec states it as a
requirement so it cannot be reintroduced as a convenience.

**Outcome:** `verified` (round 2)

### F-14 — Tokio does not guarantee the synchronous reap claimed by the lifecycle design

**Severity:** minor
**Location:** `design.md` §5.4, D21

**Expected:** The lifecycle account must state tokio’s actual cancellation and process-reaping guarantees.
**Observed:** The design says dropping the timed-out future makes tokio “SIGKILL and reap the child”. `kill_on_drop` requests termination, but tokio documents subsequent reaping as best-effort with no guarantee about how quickly or how often it occurs.
**Evidence:** tokio 1.53.1 `process/mod.rs`, “Dropping/Cancellation” and “Unix Processes”: the runtime “will, on a best-effort basis, attempt to reap” and makes “No additional guarantees.” `design.md` §5.4 claims kill and reap as the immediate drop behaviour.

**Disposition:** `fix-now`
**Response:**

Accepted; the design overstated tokio's guarantee. `kill_on_drop` is documented as
best-effort and requires a live runtime to poll the reap, so a drop during
shutdown can leave a zombie.

§5.4 now kills and awaits explicitly on the timeout path — `start_kill()` then
`wait()` — which turns a best-effort claim into an observed one, and D21 is struck
in favour of D26. `kill_on_drop(true)` stays set, for the panic and cancellation
paths that do not run this code.

Note that D21's stated justification has also stopped being true, which is why the
decision is superseded rather than amended: it argued that `wait_with_output()`
consumes the child so an explicit kill is unavailable. F-2's fix removes
`wait_with_output()`, so the constraint that made `kill_on_drop` the only option is
gone. Two findings that looked independent were coupled through that call.

**Outcome:** `verified` (round 2)

### F-15 — `Outcome` has no declared stratum, module, or contract

**Severity:** minor
**Location:** `design.md` §5.2–§5.3

**Expected:** Every cross-stratum public type must have explicit ownership so ADR-001’s dependency direction remains checkable.
**Observed:** `Outcome` is returned by the shell’s public `Host`, carries semantic views plus shell diagnostics, but has no definition, module location, or stated owner.
**Evidence:** ADR-001 requires all dependencies to point downward and identifies type placement as a deliberate obligation. `design.md` §5.2 only says what `Outcome` “carries”; §5.3 lists diagnostics as owned by “nobody”.

**Disposition:** `fix-now`
**Response:**

Accepted, and resolved together with F-8 since they are the same gap seen from two
sides — F-8 that `Outcome` had no error to carry, F-15 that `Outcome` had no home.

`Outcome` is defined in `shell/host.rs`, stratum 2. The reason is not
convenience: it mixes canonical views from stratum 1 with transport and host-state
diagnostics that only exist in stratum 2, so placing it lower would drag
`BackendError` down with it and put I/O vocabulary inside the pure core. ADR-001's
own Negative section names this class of placement question as one someone must
decide deliberately each time, which is why it is now written down rather than
left to whoever types it first. D23.

**Outcome:** `verified` (round 2)

### F-16 — AC-6 through `discarded`: no defect found

**Severity:** nit
**Location:** `design.md` §5.2; `slice-001.md` AC-6

**Expected:** Invalid scheduling must map to a distinct typed error.
**Observed:** `ScheduleError` remains typed and is carried in `Normalized::discarded`; AC-6 does not require the top-level call itself to return `Err`.
**Evidence:** `slice-001.md` AC-6 says “maps to a distinct typed error”; `design.md` §5.2 defines `Discarded::Schedule { reason: ScheduleError }`.

**Disposition:** `aligned`
**Response:**

Confirmed, and the confirmation is the useful part: this was one of the thirteen
suspicions named in the Brief before the review ran, and the reviewer independently
found no defect. AC-6 says an invalid scheduling value "maps to a distinct typed
error", which `Discarded::Schedule { reason: ScheduleError }` satisfies; nothing in
AC-6 says the call returns `Err`, and P2 forbids it. §5.2 already states this
plainly and §9 maps AC-6 to both tiers. No change.

**Outcome:** `verified` (round 2)

### F-17 — Non-optional `resolved_check`: no defect found

**Severity:** nit
**Location:** `design.md` §5.3

**Expected:** Resolution must retain an existing schedule if present and otherwise use the default interval.
**Observed:** The pure resolver can accept an optional existing schedule, while initialized host state remains concrete because `Host::new` immediately applies the default.
**Evidence:** `brief.md` §9 supplies a default whenever no existing schedule exists; `design.md` §5.3 seeds `resolved_check` to `now + default_poll`.

**Disposition:** `aligned`
**Response:**

Confirmed. Brief §9 resolves to a concrete instant in every case — new valid
instruction, else retained, else `now + default_poll` — so there is no unresolved
state for an `Option` to represent. F-8's work strengthens this rather than
disturbing it: `Outcome::next_check` is non-optional for the same reason, and I12
now records it as an invariant. No change.

**Outcome:** `verified` (round 2)

### F-18 — Replacement rather than queueing: no defect found

**Severity:** nit
**Location:** `design.md` §5.3, D16

**Expected:** The host must keep at most one active interaction and avoid general concurrency semantics.
**Observed:** Replacing the sole outstanding interaction preserves that limit and makes the previous response stale.
**Evidence:** `brief.md` §12: “allow only one active/outstanding interaction” and “reject or ignore stale responses clearly”; D16 explicitly records replacement as a chosen interpretation.

**Disposition:** `aligned`
**Response:**

Confirmed. Brief §12 allows one active interaction and warns against general
concurrency semantics; a queue is exactly that, and replacement is the only reading
that avoids it. Named in the Brief as a suspicion and independently cleared. R-33
and R-34 in the draft spec now state the behaviour and its limit — the replaced id
becomes stale immediately, and rejecting a stale answer does not clear the
outstanding interaction. No change to the design.

**Outcome:** `verified` (round 2)

### F-19 — Protocol-version asymmetry and non-zero-exit precedence: no defect found

**Severity:** nit
**Location:** `design.md` §5.2, D7 and D15

**Expected:** Choices not dictated by the brief must be identified as choices rather than falsely presented as canon.
**Observed:** Both behaviours are explicitly listed among the brief’s ambiguities and recorded as decisions.
**Evidence:** `brief.md` §8.2 omits protocol fields from response examples; §13 requires versioning and handling non-zero exits but specifies no precedence. `design.md` A4, D7 and D15 identify both as choices.

**Disposition:** `aligned`
**Response:**

Confirmed for both D7 and D15. Both were named in the Brief as places the design
had chosen rather than derived, and §5.5 A4 already says so. D7's asymmetry follows
from authorship — requiring `protocol` inbound would reject every backend written
against brief §8.2's own examples, while ignoring it would be guessing at
semantics. D15 follows from not overruling a backend that disclaimed its own
output. F-7 sharpened what the version *buys* us, which makes D7 load-bearing
rather than merely defensible. No change.

**Outcome:** `verified` (round 2)

### F-20 — Reuse of `Options`: no defect found

**Severity:** nit
**Location:** `design.md` §5.2

**Expected:** Both top-level and field-level semantic choices require at least one distinguishable option.
**Observed:** The shared newtype enforces non-empty options and unique ids for both uses without imposing widget semantics.
**Evidence:** `brief.md` §10.2 names `choice` as a semantic field kind and models choices as identified options; `design.md` §5.2 limits `Options` invariants to non-empty membership and unique ids.

**Disposition:** `aligned`
**Response:**

Confirmed. `Options` carries exactly the two invariants a nested choice needs —
non-empty, unique ids — and a separate type would duplicate both checks to express
no additional constraint. F-9's work touches the neighbourhood without changing
this: `NumberRange` is a *new* checked newtype because bounds have an invariant
nothing else expressed, which is the same reasoning arriving at the opposite
answer. No change.

**Outcome:** `verified` (round 2)

### F-21 — Mandatory `Event` on evaluate: no defect found

**Severity:** nit
**Location:** `design.md` §5.2

**Expected:** Scheduled and externally triggered evaluations must share one semantic path.
**Observed:** The mandatory envelope accommodates both by representing a poll as a scheduler-originated event.
**Evidence:** `brief.md` §8.1 gives the scheduled-poll example with `source: "scheduler"` and `kind: "poll"` and says both evaluation sources should share the same semantic path.

**Disposition:** `aligned`
**Response:**

Confirmed. Brief §7 makes every evaluation event-driven, including the scheduled
case, so an `Option<Event>` would introduce a "spontaneous evaluation" state the
brief does not have and nothing would construct. Named in the Brief as a
suspicion; cleared. No change.

**Outcome:** `verified` (round 2)

### F-22 — Field and content vocabulary gold-plating: no defect found

**Severity:** nit
**Location:** `design.md` §5.2, R7

**Expected:** Unrendered protocol variants must be traceable to named future vocabulary in the brief.
**Observed:** `Text`, `Boolean`, `Number`, `DateTime` and `Choice` match brief §10.2; `Text`, `Markdown`, `Html` and `Uri` match brief §11.
**Evidence:** `brief.md` §10.2 lists the future semantic field vocabulary; §11 lists the longer-term content forms.

**Disposition:** `aligned`
**Response:**

Confirmed, and this is the suspicion the Brief was least sure of. Every
`FieldKind` and `Content` variant is named in brief §10 or §11.1, and P3's second
half is what bounds the surface — a seam is justified by a named future
implementation, not an imagined one. R7 in §8 keeps the pressure visible with a
stated signal: a `FieldKind` or hint the brief never mentions. F-9's additions do
not widen the vocabulary; `NumberRange` constrains a kind that was already there.
No change.

**Outcome:** `verified` (round 2)

### F-23 — The repaired `Outcome` withholds the `view_id` required to answer its view

**Severity:** blocker
**Location:** `design.md` §5.2, D23

**Expected:** Slice 002 must be able to render a returned view and later call `respond` with the host-minted identity for that exact interaction, without changing the slice-001 public interface.
**Observed:** `Outcome` exposes `view`, `next_check`, `discarded` and `failure`, but not `view_id`. The id remains private in `State`, so a renderer receiving the view has no value with which to answer it.
**Evidence:** `design.md` §5.2 defines all four `Outcome` fields and omits `view_id`; `slice-001.md` AC-7 requires the host to assign and record a `view_id`; brief §8.3 requires the subsequent response request to carry that id. Review invariant 4 makes a slice-002 interface change a blocker.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-24 — The transport return type cannot carry the repaired stderr diagnostics

**Severity:** major
**Location:** `design.md` §5.2 D12, D23, D27; `draft-spec.md` R-42, R-43

**Expected:** Stderr must be captured whatever the exchange outcome, reported with failures, and expose whether it was truncated.
**Observed:** `Backend::exchange` still returns only `Result<Vec<u8>, BackendError>`, and `Outcome` has no diagnostic or truncation field. Stderr from an exit-zero response is therefore unavailable — including when the JSON parse or normalization then fails — and the promised truncation flag has nowhere to go.
**Evidence:** `design.md` §5.2 defines the transport output as `Vec<u8>` and `Outcome` without diagnostics; `BackendError` carries stderr only for `Timeout` and `ExitStatus`. `draft-spec.md` §6.4 says stderr is captured "whatever the outcome"; R-42 requires it with failures; R-43's verification requires a successful stderr flood to return "a truncation flag".

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-25 — The bounded stderr drain does not specify continued draining after truncation

**Severity:** major
**Location:** `design.md` §5.4, D27

**Expected:** Stderr storage must stop growing at 256 KiB while the pipe continues to be drained, so a chatty backend can complete normally.
**Observed:** The design calls `read_capped(stderr, STDERR_LIMIT)` and says excess stderr is truncated, but never states whether bytes beyond the cap are consumed and discarded. Returning at the cap closes the reader and can give the backend `EPIPE`; leaving the pipe unread can block the backend into a timeout.
**Evidence:** `slice-001.md` AC-5 requires concurrent bounded reads without chatty-backend deadlock. `design.md` §5.4 says over-limit stderr "is not a failure in itself", but the sketched `read_capped` contract does not define draining beyond the retained buffer.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-26 — Non-timeout failures fall back to the reaping mechanism D26 rejects

**Severity:** major
**Location:** `design.md` §5.4, D26, D27; `draft-spec.md` R-45

**Expected:** Every path that abandons a live child must terminate and reap it reliably.
**Observed:** Explicit `start_kill()` plus `wait()` occurs only in the timeout arm. `Ok(res) => res` propagates early stdout-cap, stdin-I/O and stdout-I/O errors without cleanup, leaving `kill_on_drop` as the only mechanism on those paths — the mechanism D26 rejects as best-effort.
**Evidence:** `design.md` §5.4 shows `Ok(res) => res` with explicit cleanup only under `Err(_)`; D26 says `kill_on_drop` must not be relied upon; `draft-spec.md` R-45 requires every backend failure to leave the host able to invoke the backend again.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-27 — The timeout repair still has an admitted path that discards captured stderr

**Severity:** major
**Location:** `design.md` §5.4; `slice-001.md` AC-5; `draft-spec.md` R-42

**Expected:** A timeout must carry stderr already produced by the backend.
**Observed:** If a grandchild inherits stderr, the grace timeout abandons the drain task and reports the timeout "with no stderr". Because the task owns the buffer, already-read bytes are lost too — not just the tail.
**Evidence:** `design.md` §5.4 says the grace timeout reports "without stderr"; the same section later claims a timed-out backend yields "whatever stderr it had produced". `slice-001.md` AC-5 and `draft-spec.md` R-42 require stderr capture on the timeout path with no such exception.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-28 — R-30 contradicts permissive handling and is not implementable by the designed wire type

**Severity:** major
**Location:** `draft-spec.md` R-4, R-5, R-30; `design.md` §5.2

**Expected:** Unmodelled response fields must be ignored unless the wire model explicitly recognizes and rejects them.
**Observed:** R-30 requires rejection if a backend supplies `view_id`, while R-4 and R-5 require every unmodelled field to be ignored. `WireResponse` has no `view_id` member and uses no `deny_unknown_fields`, so serde discards it before normalization could enforce R-30.
**Evidence:** `draft-spec.md` R-4, R-5 and R-30 state the contradictory requirements; `design.md` §5.2 shows the complete `WireResponse` fields and mandates no `deny_unknown_fields` anywhere inbound.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-29 — The draft spec and design disagree about `view: null` while another interaction is outstanding

**Severity:** major
**Location:** `draft-spec.md` §5; `design.md` §5.5

**Expected:** The contract must state one behaviour for an evaluation returning `view: null` while an older interaction remains outstanding.
**Observed:** The design says `view: null` leaves any outstanding interaction alone. The draft spec says a response carrying `view: null` "does not" leave an interaction outstanding, without limiting that to an accepted `respond`.
**Evidence:** `design.md` §5.5 edge-case table: "any outstanding interaction is left alone". `draft-spec.md` §5: "A response carrying a view leaves an interaction outstanding; one carrying `view: null` does not."

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-30 — The new draft still fails AC-13's identity and first-line requirements

**Severity:** major
**Location:** `draft-spec.md:1`; `slice-001.md` AC-13

**Expected:** The draft's first line must state that it is not canon, and it must carry no SPEC id before promotion.
**Observed:** Its first line is `# SPEC-NNN: The host/backend interaction protocol`; the non-canon warning appears later. The placeholder SPEC identifier contradicts the claimed absence of one.
**Evidence:** `slice-001.md` AC-13 requires "a first line stating that it is not canon" and says "It carries no SPEC id". `draft-spec.md:1` contains `SPEC-NNN`.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-31 — The protocol spec does not define the wire encoding of most admitted variants

**Severity:** major
**Location:** `draft-spec.md` R-16, R-19, §6.2

**Expected:** A wire-contract specification must define how each admitted field kind and content form is represented, including the brief's plain-string body form.
**Observed:** The spec lists five field kinds and four content forms but shows only one number field and one tagged Markdown body. It does not define the text/boolean/datetime/choice field shapes, nor whether plain text is a string, `{"kind":"text",…}`, or both.
**Evidence:** `brief.md` §10.1 shows `"body": "Optional context"` as the required basic-choice form. `draft-spec.md` §6.2 shows only `{"kind":"markdown","value":…}`; R-16 and R-19 enumerate variants without their JSON shapes.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-32 — `failure: Some` falsely claims the backend call had no effect

**Severity:** major
**Location:** `design.md` §5.2, `Outcome`

**Expected:** A host failure must not imply that a user-owned backend performed no side effects.
**Observed:** The `failure` field is documented as "this call had no effect beyond being reported". A backend may perform arbitrary side effects and then time out, exit non-zero, or emit an invalid response.
**Evidence:** `design.md` §5.2 contains the no-effect claim. Brief §8.3 says a backend may perform arbitrary side effects when handling a response; brief §14 grants it normal user authority.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-33 — The hints invariant forbids the renderer behaviour hints exist to control

**Severity:** major
**Location:** `design.md` I7; `draft-spec.md` R-18

**Expected:** Presentation hints may be interpreted by the renderer while remaining irrelevant to semantic-core decisions.
**Observed:** Both documents prohibit the entire host from branching on hint keys. The renderer is part of the host and must inspect keys such as `multiline`, `placeholder` or `units` for them to affect presentation at all.
**Evidence:** Brief §10.2 calls these "presentation hints" and §3.4 says the renderer chooses widgets. `design.md` I7 says "The host never branches on a `hints` key"; `draft-spec.md` R-18 repeats "The host MUST NOT branch on any hint key".

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-34 — The corrected validation conclusion is contradicted by surviving decision and risk text

**Severity:** minor
**Location:** `design.md` D17, R4, §5.5

**Expected:** After F-7, the design must consistently state that validation feedback is wire-additive but requires version or capability negotiation to be semantically safe.
**Observed:** §5.5 carries the correction, while D17 still says "Validation feedback confirmed additive" and R4 says it was "proved additive", without the qualification.
**Evidence:** `design.md` §5.5 says the original no-version-bump conclusion was wrong; D17 and R4 retain the unqualified opposite claim.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-35 — The process sketch violates the draft's no-`expect` verification rule

**Severity:** minor
**Location:** `design.md` §5.4; `draft-spec.md` R-46

**Expected:** The implementation design and the verification contract must agree on whether `expect` is permitted outside tests.
**Observed:** The process sketch uses `child.stderr.take().expect("piped at spawn")`, while the spec requires clippy to deny `expect_used` outside tests.
**Evidence:** `design.md` §5.4 contains the `expect`; `draft-spec.md` §7, R-46's verification, says "clippy denying `unwrap_used` and `expect_used` outside tests".

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-36 — The specified NaN JSON fixture cannot exist

**Severity:** minor
**Location:** `draft-spec.md` §7, R-17 verification

**Expected:** Each verification case must be representable at the boundary it claims to test.
**Observed:** The spec calls for a JSON fixture containing a NaN bound, but JSON has no NaN literal; serde_json rejects it before `NumberRange` validation can produce `BoundsError::NotFinite`.
**Evidence:** `draft-spec.md` §7 assigns R-17 to fixtures containing "`NaN` and inverted bounds". RFC 8259 §6 excludes NaN and infinity from JSON numbers.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-37 — The canon-impact table still says the now-present delta is owed

**Severity:** minor
**Location:** `design.md` §10

**Expected:** `design.md`, as the current-truth artefact, must reflect that `canon-delta.md` now exists and only its application remains outstanding.
**Observed:** The canon-impact table still labels `canon-delta.md` itself "owed".
**Evidence:** `design.md` §10 says "`canon-delta.md` — owed"; `docs/slices/001/canon-delta.md` now exists with CD-1.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

### F-38 — The wire `Field` shape rejects the brief's own flat presentation keys

**Severity:** major
**Location:** `design.md` §5.2; `brief.md` §10.2

**Expected:** The wire types must accept the brief's own worked examples without modification, since those examples are what a backend author will copy.
**Observed:** Raised by me while verifying F-31, and not covered by it. Brief §10.2 places `multiline` **flat on the field object**, alongside `id`, `kind`, `label`, `min` and `max` — not inside a nested `hints` object. The design's `Field` carries `hints: Hints` as a distinct member, so `multiline: true` arrives as an unmodelled field and is silently discarded by R-4's ignore rule. The brief's example loses its presentation information with no error, which is the worst of both outcomes.
**Evidence:** `brief.md` §10.2's field example: `{"id":"notes","kind":"text","label":"Anything notable?","multiline":true}`. `design.md` §5.2 defines `pub struct Field { id, kind, label, hints }` with no flattening, and states that unknown fields are ignored.

**Disposition:** _pending_
**Response:**

**Outcome:** _pending_

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

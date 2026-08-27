# Review — design — Slice 001

**Subject:** design — `docs/slices/001/design.md`, as at the tree state of
2026-08-23, together with the acceptance criteria in `slice-001.md` it claims to
discharge
**Reviewer:** fresh agent via codex MCP (GPT-5.5 Sol, default model), thread
`01a02caa-83d2-7950-a2a7-60c2bdc017e0` — reusable for further rounds
**Opened:** 2026-08-23
**State:** **closed** 2026-08-26 by user decision, not by reaching `done` — see
Synthesis. F-1…F-38 `verified`; F-39 and F-43…F-47 `verified` at round 4; F-40,
F-41 and F-42 reopened at round 4 and superseded by F-48/F-49; F-48…F-58
`repaired` and never re-examined; F-59…F-63 `repaired` at round 5 and unverified.
No blocker outstanding — all 7 were repaired. 16 repairs carry no independent
confirmation
**Rounds:** 1 — 22 findings, 1 blocker, all repaired and verified. 2 — 15 findings,
1 blocker, plus F-38 raised by me while verifying F-31; all repaired and verified.
3 — 9 findings, no blocker, **every one a defect in a round-2 repair** rather than
in the original design; all nine repaired. 4 — a fresh reviewer with no thread
history (the codex session was stale), given the full ledger index and asked to
verify the round-3 repairs first: 6 findings, 3 blockers, and three round-3
repairs reopened as incomplete. Two of the blockers are original design defects
that three rounds had not found — the first round since round 1 to reach past the
review's own wake. Five of the round's findings — F-54, F-55, F-56,
F-57 and F-58 — were raised by the responder rather than the reviewer, the last
three while self-checking the round-4 repairs before the round-5 packet went
out. 5 — a second fresh reviewer, given the round-4 batch to verify: 4 findings,
2 blockers, and **none of them a defect in a round-4 repair**. Both blockers were
in the original F-53 and F-41 repairs from round 3, untouched by round 4 and
unexamined by rounds 3 and 4 — the second round running to reach past the
review's own wake. F-63 was raised by the responder sweeping the round-5 batch,
and is the first finding against an *empirical* claim: a case described in five
places and never run.

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
| F-23 | blocker | `fix-now` | `verified` |
| F-24 | major | `fix-now` | `verified` |
| F-25 | major | `fix-now` | `verified` |
| F-26 | major | `fix-now` | `verified` |
| F-27 | major | `fix-now` | `verified` |
| F-28 | major | `fix-now` | `verified` |
| F-29 | major | `fix-now` | `verified` |
| F-30 | major | `fix-now` | `verified` |
| F-31 | major | `fix-now` | `verified` |
| F-32 | major | `fix-now` | `verified` |
| F-33 | major | `fix-now` | `verified` |
| F-34 | minor | `fix-now` | `verified` |
| F-35 | minor | `fix-now` | `verified` |
| F-36 | minor | `fix-now` | `verified` |
| F-37 | minor | `fix-now` | `verified` |
| F-38 | major | `fix-now` | `verified` |
| F-39 | major | `fix-now` | `verified` |
| F-40 | major | `fix-now` | `reopened` → superseded by F-49 |
| F-41 | major | `fix-now` + `doc-wrong` | `reopened` → return path `verified`; cancellation superseded by F-49 |
| F-42 | major | `fix-now` + `doc-wrong` | `reopened` → repair **reversed** by F-48 |
| F-43 | major | `fix-now` | `verified` |
| F-44 | major | `fix-now` | `verified` |
| F-45 | major | `fix-now` | `verified` |
| F-46 | major | `fix-now` | `verified` |
| F-47 | minor | `fix-now` | `verified` |
| F-48 | blocker | `fix-now` | `repaired` |
| F-49 | major | `fix-now` | `repaired` |
| F-50 | blocker | `fix-now` | `repaired` |
| F-51 | blocker | `fix-now` | `repaired` |
| F-52 | major | `fix-now` | `repaired` |
| F-53 | major | `fix-now` | `repaired` |
| F-54 | major | `fix-now` | `repaired` |
| F-55 | major | `fix-now` | `repaired` |

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

**Disposition:** `fix-now`
**Response:**

Accepted, and this one is mine: the defect was introduced by the F-8 repair. The
prose it replaced said `Outcome` "carries what the caller must act on", and the
round-1 sequence diagram showed `view_id` in the outcome — I dropped it when I
turned the prose into a field list, then edited the diagram to match the smaller
struct rather than noticing the diagram was right.

The fix is not an extra `Option<ViewId>` field. `Outcome::view` is now
`Option<Presented>` with `Presented { view_id, view }`, so a view without its id
and an id without its view are both unrepresentable. That is the same move as
`Options` and `NumberRange` — the check no caller has to perform is the one the
type forecloses. D32, I14.

**Outcome:** `verified` (round 3)

### F-24 — The transport return type cannot carry the repaired stderr diagnostics

**Severity:** major
**Location:** `design.md` §5.2 D12, D23, D27; `draft-spec.md` R-42, R-43

**Expected:** Stderr must be captured whatever the exchange outcome, reported with failures, and expose whether it was truncated.
**Observed:** `Backend::exchange` still returns only `Result<Vec<u8>, BackendError>`, and `Outcome` has no diagnostic or truncation field. Stderr from an exit-zero response is therefore unavailable — including when the JSON parse or normalization then fails — and the promised truncation flag has nowhere to go.
**Evidence:** `design.md` §5.2 defines the transport output as `Vec<u8>` and `Outcome` without diagnostics; `BackendError` carries stderr only for `Timeout` and `ExitStatus`. `draft-spec.md` §6.4 says stderr is captured "whatever the outcome"; R-42 requires it with failures; R-43's verification requires a successful stderr flood to return "a truncation flag".

**Disposition:** `fix-now`
**Response:**

Accepted. `exchange` returns `Exchange { stdout: Vec<u8>, stderr: Captured }`,
and `Outcome` carries `stderr` uniformly; `Captured { bytes, truncated }` is where
D27's truncation flag lives.

The finding is sharper than it looks. Hanging `stderr` off `Timeout` and
`ExitStatus` made it reachable on exactly the two paths that already announce
themselves, and unreachable on the one that does not: a backend exits zero, writes
something unparseable, and has already explained why on stderr. That is the case
where stderr is the only evidence, and it was the case with none.

Worth recording, since it is a pattern rather than an incident: this is the second
change to the transport seam in one review, after F-1. Both times the signature
had been shaped to the process transport's happy path. D33.

**Outcome:** `verified` (round 3)

### F-25 — The bounded stderr drain does not specify continued draining after truncation

**Severity:** major
**Location:** `design.md` §5.4, D27

**Expected:** Stderr storage must stop growing at 256 KiB while the pipe continues to be drained, so a chatty backend can complete normally.
**Observed:** The design calls `read_capped(stderr, STDERR_LIMIT)` and says excess stderr is truncated, but never states whether bytes beyond the cap are consumed and discarded. Returning at the cap closes the reader and can give the backend `EPIPE`; leaving the pipe unread can block the backend into a timeout.
**Evidence:** `slice-001.md` AC-5 requires concurrent bounded reads without chatty-backend deadlock. `design.md` §5.4 says over-limit stderr "is not a failure in itself", but the sketched `read_capped` contract does not define draining beyond the retained buffer.

**Disposition:** `fix-now`
**Response:**

Accepted. Two readers, not one: `read_capped` for stdout stops at the limit and
drops the handle; `drain_capped` for stderr retains the first 256 KiB, sets
`truncated`, and keeps consuming to EOF.

The distinction is the finding's real content and it is not cosmetic. D27 says
over-long stderr is "truncated and flagged, not fatal", and a single `read_capped`
that returns at the cap makes that sentence false — the backend blocks on a full
pipe nobody is reading, and the exchange dies at the timeout instead of
succeeding. So "truncate" has to mean *stop storing*, and never *stop reading*.
Conversely stdout should stop reading, because closing the pipe is what makes a
flood stop rather than merely making our buffer stop growing — verified in round 1
when the flooding backend took `SIGPIPE`. One name for two behaviours hid a
deadlock behind a word. D34.

**Outcome:** `verified` (round 3)

### F-26 — Non-timeout failures fall back to the reaping mechanism D26 rejects

**Severity:** major
**Location:** `design.md` §5.4, D26, D27; `draft-spec.md` R-45

**Expected:** Every path that abandons a live child must terminate and reap it reliably.
**Observed:** Explicit `start_kill()` plus `wait()` occurs only in the timeout arm. `Ok(res) => res` propagates early stdout-cap, stdin-I/O and stdout-I/O errors without cleanup, leaving `kill_on_drop` as the only mechanism on those paths — the mechanism D26 rejects as best-effort.
**Evidence:** `design.md` §5.4 shows `Ok(res) => res` with explicit cleanup only under `Err(_)`; D26 says `kill_on_drop` must not be relied upon; `draft-spec.md` R-45 requires every backend failure to leave the host able to invoke the backend again.

**Disposition:** `fix-now`
**Response:**

Accepted, and it is the more embarrassing half of F-14. Having argued that
`kill_on_drop` is best-effort and must not be relied on, the design then relied on
it for three of the four exits — the stdout-cap error, the stdin write error and
the stdout read error — because only the timeout arm did anything explicit.

`reap` is now unconditional and idempotent, called after the error has been
decided rather than inside any one arm, so there is no path that abandons a live
child. `kill_on_drop(true)` stays set for panics and cancellation, which are the
paths that genuinely cannot run this code. D35, I13.

**Outcome:** `verified` (round 3)

### F-27 — The timeout repair still has an admitted path that discards captured stderr

**Severity:** major
**Location:** `design.md` §5.4; `slice-001.md` AC-5; `draft-spec.md` R-42

**Expected:** A timeout must carry stderr already produced by the backend.
**Observed:** If a grandchild inherits stderr, the grace timeout abandons the drain task and reports the timeout "with no stderr". Because the task owns the buffer, already-read bytes are lost too — not just the tail.
**Evidence:** `design.md` §5.4 says the grace timeout reports "without stderr"; the same section later claims a timed-out backend yields "whatever stderr it had produced". `slice-001.md` AC-5 and `draft-spec.md` R-42 require stderr capture on the timeout path with no such exception.

**Disposition:** `fix-now`
**Response:**

Accepted, and it reverses a choice I made deliberately an hour earlier. I had the
drain task return its buffer through the join handle specifically to avoid a
mutex, and noted the grandchild-inherits-stderr case as a residual stall. F-27's
point is that the join handle *is* the hole: abandoning the task discards
everything it had read, not merely the bytes still to come — so the mitigation for
the stall destroyed the diagnostic that was the reason for capturing stderr at
all.

The buffer is now an `Arc<Mutex<Captured>>` owned by the caller, so abandoning the
task still leaves every byte already read readable. The lock is justified here in a
way D14 refused for `State`: there the concurrency was hypothetical and brief §12
says not to invent it, here there are genuinely two tasks and one buffer, and it is
uncontended in the normal case. §5.4 now says that explicitly, so the two
decisions do not read as inconsistent. D36.

**Outcome:** `verified` (round 3)

### F-28 — R-30 contradicts permissive handling and is not implementable by the designed wire type

**Severity:** major
**Location:** `draft-spec.md` R-4, R-5, R-30; `design.md` §5.2

**Expected:** Unmodelled response fields must be ignored unless the wire model explicitly recognizes and rejects them.
**Observed:** R-30 requires rejection if a backend supplies `view_id`, while R-4 and R-5 require every unmodelled field to be ignored. `WireResponse` has no `view_id` member and uses no `deny_unknown_fields`, so serde discards it before normalization could enforce R-30.
**Evidence:** `draft-spec.md` R-4, R-5 and R-30 state the contradictory requirements; `design.md` §5.2 shows the complete `WireResponse` fields and mandates no `deny_unknown_fields` anywhere inbound.

**Disposition:** `fix-now`
**Response:**

Accepted; R-30 as written was unenforceable and contradicted R-4 and R-5. Since
`WireResponse` has no `view_id` member and nothing inbound uses
`deny_unknown_fields`, serde discards the field before normalization could refuse
it — the requirement described a check that cannot happen.

R-30 now states the property that actually matters and is actually testable: the
host mints every `view_id` and **never reads one from a response**; a backend that
sends one has it ignored under R-4, like any other unmodelled key. That is
verifiable by inspection — no inbound type has the field — and it forecloses the
real hazard, which was never a backend sending an id but the host one day
believing one.

**Outcome:** `verified` (round 3)

### F-29 — The draft spec and design disagree about `view: null` while another interaction is outstanding

**Severity:** major
**Location:** `draft-spec.md` §5; `design.md` §5.5

**Expected:** The contract must state one behaviour for an evaluation returning `view: null` while an older interaction remains outstanding.
**Observed:** The design says `view: null` leaves any outstanding interaction alone. The draft spec says a response carrying `view: null` "does not" leave an interaction outstanding, without limiting that to an accepted `respond`.
**Evidence:** `design.md` §5.5 edge-case table: "any outstanding interaction is left alone". `draft-spec.md` §5: "A response carrying a view leaves an interaction outstanding; one carrying `view: null` does not."

**Disposition:** `fix-now`
**Response:**

Accepted; the spec sentence was mine and it was careless. "A response carrying a
view leaves an interaction outstanding; one carrying `view: null` does not" is
true of an accepted `respond` and false of an `evaluate`, and the design's edge
table already said the opposite for the second case.

§5 now distinguishes them, and the distinction is worth the words: a `view: null`
answering an `evaluate` means "nothing new", which is not a withdrawal of a
question the user is still looking at. Reading it as one would clear a prompt off
the screen because a *poll* came back empty. The AC-9 corpus and R-10/R-11's
verification now carry a case for each meaning.

**Outcome:** `verified` (round 3)

### F-30 — The new draft still fails AC-13's identity and first-line requirements

**Severity:** major
**Location:** `draft-spec.md:1`; `slice-001.md` AC-13

**Expected:** The draft's first line must state that it is not canon, and it must carry no SPEC id before promotion.
**Observed:** Its first line is `# SPEC-NNN: The host/backend interaction protocol`; the non-canon warning appears later. The placeholder SPEC identifier contradicts the claimed absence of one.
**Evidence:** `slice-001.md` AC-13 requires "a first line stating that it is not canon" and says "It carries no SPEC id". `draft-spec.md:1` contains `SPEC-NNN`.

**Disposition:** `fix-now`
**Response:**

Accepted, and the irony is the point: the draft asserted it carried no SPEC id
while carrying one in its `<h1>`. I wrote the non-canon notice in the round-1 pass
specifically to satisfy AC-13 and left the template's `SPEC-NNN:` prefix in the
title above it.

The non-canon statement is now the first line of the file, the title is the
protocol's name with no id, and the only remaining reference to a placeholder id is
the sentence explaining that one used to be there. AC-13 is checkable by grep
again, which is how it was meant to be checkable.

**Outcome:** `verified` (round 3)

### F-31 — The protocol spec does not define the wire encoding of most admitted variants

**Severity:** major
**Location:** `draft-spec.md` R-16, R-19, §6.2

**Expected:** A wire-contract specification must define how each admitted field kind and content form is represented, including the brief's plain-string body form.
**Observed:** The spec lists five field kinds and four content forms but shows only one number field and one tagged Markdown body. It does not define the text/boolean/datetime/choice field shapes, nor whether plain text is a string, `{"kind":"text",…}`, or both.
**Evidence:** `brief.md` §10.1 shows `"body": "Optional context"` as the required basic-choice form. `draft-spec.md` §6.2 shows only `{"kind":"markdown","value":…}`; R-16 and R-19 enumerate variants without their JSON shapes.

**Disposition:** `fix-now`
**Response:**

Accepted, and verifying it turned up a second, worse instance which I raised as
F-38 rather than fold in silently.

The spec now defines every admitted shape rather than one example of each family:
all five field kinds in their wire form, all four content forms, and the rule that
a bare JSON string *is* `text`. That last is not a convenience — brief §10.1's
required v0 example is `"body": "Optional context"`, so a tagged-only `Content`
rejects the one interaction the brief says v0 must support.

The design side matters too, and is the part the finding could not see: the
tempting encoding for string-or-object is `#[serde(untagged)]`, and it is wrong
here. `untagged` collapses every failure into "data did not match any variant",
which destroys F-6's `UnsupportedPrimitive { kind, at }` — the error F-6 was raised
to obtain. So `body` stays `serde_json::Value` at the wire and `normalize`
dispatches, for the same reason `next_check` does. D38.

R-15, R-16 and R-19 also gained the shape rules, so the enumeration and the
encoding are no longer in different documents.

**Outcome:** `verified` (round 3)

### F-32 — `failure: Some` falsely claims the backend call had no effect

**Severity:** major
**Location:** `design.md` §5.2, `Outcome`

**Expected:** A host failure must not imply that a user-owned backend performed no side effects.
**Observed:** The `failure` field is documented as "this call had no effect beyond being reported". A backend may perform arbitrary side effects and then time out, exit non-zero, or emit an invalid response.
**Evidence:** `design.md` §5.2 contains the no-effect claim. Brief §8.3 says a backend may perform arbitrary side effects when handling a response; brief §14 grants it normal user authority.

**Disposition:** `fix-now`
**Response:**

Accepted. "This call had no effect beyond being reported" was a false claim about
a system the host does not control, and the danger is specific rather than
theoretical: read that way, `failure: Some(_)` invites a retry, and a retry after
a backend has already acted repeats whatever it did.

The doc comment now says the *host* took no action and recorded no state change,
and §5.2 states plainly that this is not a claim about the backend's effects.
Brief §8.3 lets a backend do arbitrary work handling a response and §14 gives it
the user's own authority, so a timeout can follow a completed side effect. This is
also, stated properly for the first time, the real argument for D-no-retry — not
just that nothing in the brief asks for retries, but that the host cannot know what
a failed exchange already did.

**Outcome:** `verified` (round 3)

### F-33 — The hints invariant forbids the renderer behaviour hints exist to control

**Severity:** major
**Location:** `design.md` I7; `draft-spec.md` R-18

**Expected:** Presentation hints may be interpreted by the renderer while remaining irrelevant to semantic-core decisions.
**Observed:** Both documents prohibit the entire host from branching on hint keys. The renderer is part of the host and must inspect keys such as `multiline`, `placeholder` or `units` for them to affect presentation at all.
**Evidence:** Brief §10.2 calls these "presentation hints" and §3.4 says the renderer chooses widgets. `design.md` I7 says "The host never branches on a `hints` key"; `draft-spec.md` R-18 repeats "The host MUST NOT branch on any hint key".

**Disposition:** `fix-now`
**Response:**

Accepted; I7 and R-18 were both wrong, and wrong in a way that would have been
quoted back at slice 002. "The host never branches on a `hints` key" forbids the
renderer from reading `multiline` — which is the entire reason hints exist, per
brief §10.2 and §3.4's "the renderer chooses widgets".

The invariant now names the strata it constrains: nothing in `semantics/` or
`shell/` may branch on a hint key, and the renderer is the one thing that may. That
is what the rule was always trying to say. It also makes the rule *checkable* in a
way the blanket version was not — the blanket version would have been violated by
the first correct renderer, so the only way to keep it green was to never write
one.

**Outcome:** `verified` (round 3)

### F-34 — The corrected validation conclusion is contradicted by surviving decision and risk text

**Severity:** minor
**Location:** `design.md` D17, R4, §5.5

**Expected:** After F-7, the design must consistently state that validation feedback is wire-additive but requires version or capability negotiation to be semantically safe.
**Observed:** §5.5 carries the correction, while D17 still says "Validation feedback confirmed additive" and R4 says it was "proved additive", without the qualification.
**Evidence:** `design.md` §5.5 says the original no-version-bump conclusion was wrong; D17 and R4 retain the unqualified opposite claim.

**Disposition:** `fix-now`
**Response:**

Accepted. Two sites survived the F-7 correction: D17's "validation feedback
confirmed additive" and R4's "proved additive". Both now carry the qualification —
no breaking restructure, but version or capability negotiation required.

This is the ordinary failure mode of correcting a claim in prose: the argument gets
fixed where it was made and not where it was summarised, and the summary is what a
later reader skims. R4 in particular is a risk row, which is exactly where an
over-confident "proved" does damage. Worth noting the class rather than just the
two lines: any correction to §5's reasoning should be checked against §7's decision
index and §8's risk table, because both restate it in one line.

**Outcome:** `verified` (round 3)

### F-35 — The process sketch violates the draft's no-`expect` verification rule

**Severity:** minor
**Location:** `design.md` §5.4; `draft-spec.md` R-46

**Expected:** The implementation design and the verification contract must agree on whether `expect` is permitted outside tests.
**Observed:** The process sketch uses `child.stderr.take().expect("piped at spawn")`, while the spec requires clippy to deny `expect_used` outside tests.
**Evidence:** `design.md` §5.4 contains the `expect`; `draft-spec.md` §7, R-46's verification, says "clippy denying `unwrap_used` and `expect_used` outside tests".

**Disposition:** `fix-now`
**Response:**

Accepted, both halves — the sketch and the requirement it violated.

The sketch's `expect` is gone; the handle is taken with `ok_or(BackendError::
PipeMissing)?`. But R-46's verification was also overreaching, and that is the
more useful half. R-46's *requirement* is that the host must not panic on any value
derived from a backend; its verification said "clippy denying `unwrap_used` and
`expect_used` outside tests", which is a broader rule about all values from any
source, including ones the host itself just created. The verification now names the
modules that handle backend-derived data, which is what the requirement is actually
about. A verification that is stricter than its requirement will be either weakened
under pressure or worked around, and both are worse than stating it correctly.

**Outcome:** `verified` (round 3)

### F-36 — The specified NaN JSON fixture cannot exist

**Severity:** minor
**Location:** `draft-spec.md` §7, R-17 verification

**Expected:** Each verification case must be representable at the boundary it claims to test.
**Observed:** The spec calls for a JSON fixture containing a NaN bound, but JSON has no NaN literal; serde_json rejects it before `NumberRange` validation can produce `BoundsError::NotFinite`.
**Evidence:** `draft-spec.md` §7 assigns R-17 to fixtures containing "`NaN` and inverted bounds". RFC 8259 §6 excludes NaN and infinity from JSON numbers.

**Disposition:** `fix-now`
**Response:**

Accepted, and confirmed by running it rather than by reading RFC 8259: serde_json
rejects `{"min": NaN}` with `expected value` and `{"min": 1e400}` with `number out
of range`, both before any bounds check can execute. So `BoundsError::NotFinite` is
unreachable from the wire and the fixture the spec asked for cannot be written.

The user's call was keep-and-correct rather than drop the variant, which is the
right way round: `NumberRange::new` is public API, P1's claim is about what the type
can hold and not about which caller supplied the value, and one comparison is
cheaper than a later argument over whether the invariant really holds. What was
wrong was the *claim*, so R-17's verification and §9's fixture now assert
`Protocol(Json)` for a NaN literal, and §5.2 records that the variant is a
constructor guard rather than a wire failure mode. Asserting the unreachable
variant would have been a test that cannot fail — the more dangerous of the two
errors, since it looks like coverage. D39.

**Outcome:** `verified` (round 3)

### F-37 — The canon-impact table still says the now-present delta is owed

**Severity:** minor
**Location:** `design.md` §10

**Expected:** `design.md`, as the current-truth artefact, must reflect that `canon-delta.md` now exists and only its application remains outstanding.
**Observed:** The canon-impact table still labels `canon-delta.md` itself "owed".
**Evidence:** `design.md` §10 says "`canon-delta.md` — owed"; `docs/slices/001/canon-delta.md` now exists with CD-1.

**Disposition:** `fix-now`
**Response:**

Accepted. §10 was written before `canon-delta.md` existed and still described it as
owed. The row now says the file exists with CD-1 and that what remains outstanding
is its *application* during reconciliation, not its authorship.

Small, but the artefact is supposed to be current truth — a reader checking what
this slice still owes would have gone looking for a file that was already there,
and the methodology's whole separation of artefact from log depends on the artefact
not lying about the present.

**Outcome:** `verified` (round 3)

### F-38 — The wire `Field` shape rejects the brief's own flat presentation keys

**Severity:** major
**Location:** `design.md` §5.2; `brief.md` §10.2

**Expected:** The wire types must accept the brief's own worked examples without modification, since those examples are what a backend author will copy.
**Observed:** Raised by me while verifying F-31, and not covered by it. Brief §10.2 places `multiline` **flat on the field object**, alongside `id`, `kind`, `label`, `min` and `max` — not inside a nested `hints` object. The design's `Field` carries `hints: Hints` as a distinct member, so `multiline: true` arrives as an unmodelled field and is silently discarded by R-4's ignore rule. The brief's example loses its presentation information with no error, which is the worst of both outcomes.
**Evidence:** `brief.md` §10.2's field example: `{"id":"notes","kind":"text","label":"Anything notable?","multiline":true}`. `design.md` §5.2 defines `pub struct Field { id, kind, label, hints }` with no flattening, and states that unknown fields are ignored.

**Disposition:** `fix-now`
**Response:**

Raised by me while verifying F-31; disposed of here as responder.

Brief §10.2's field example puts `multiline` **flat on the field object**, beside
`id`, `kind`, `label`, `min` and `max`. The design had `hints` as a nested member,
so `multiline` arrived as an unmodelled field and the no-`deny_unknown_fields` rule
discarded it *silently* — the brief's own example losing its presentation
information with no error at all. That is worse than F-31's rejection, because a
rejection is at least visible.

`WireField` now uses `#[serde(flatten)]`, making "every other key on the field
object" the definition of a hint — which is also the honest reading of brief
§10.2's "likely presentation hints over time". Verified by running it, including
the cost: a misspelled *optional* key (`minn`) becomes a hint, while a misspelled
*required* key still fails with `missing field 'label'`. So the exposure is
bounded by which keys are optional, which is narrower than flattening usually
implies.

The user chose flat-only over accepting both spellings. That is the right call for
a reason worth writing down: two accepted spellings for one thing is the ambiguity
brief §3.3 says must fail rather than be guessed at, and it would have doubled the
normalization paths to buy nothing. D37.

**Outcome:** `verified` (round 3)

### F-39 — Transport failures still cannot return stderr

**Severity:** major
**Location:** `design.md` §5.2, Transport

**Expected:** Every outcome, including timeout, non-zero exit, output overflow and transport I/O failure, carries the stderr captured before that failure.
**Observed:** `Backend::exchange` returns `Result<Exchange, BackendError>`, but only the `Ok(Exchange)` variant contains `Captured`, and `BackendError` deliberately contains no stderr. Every transport-level `Err` therefore loses the only value from which `Outcome::stderr` could be populated.
**Evidence:** `design.md` §5.2 specifies `Future<Output = Result<Exchange, BackendError>>`, `Exchange { stdout, stderr }`, and `BackendError::Timeout`, `ExitStatus`, `OutputTooLarge` and `Io` without capture. `draft-spec.md` R-42 requires stderr with every outcome, explicitly including timeout.

**Disposition:** `fix-now`
**Response:**

Accepted without argument, because the argument was already in this document. D23
says a value every path produces must not live on the success branch of a
`Result`, and that is exactly what `Result<Exchange, BackendError>` does to
`Captured`. I wrote the rule for `Outcome` and then built `Outcome` out of a type
that breaks it — the repair for F-24 moved stderr off the error variants and left
it stranded on `Ok`, which is the same defect one level up rather than a fix.

`exchange` now returns a bare `Exchange { result: Result<Vec<u8>, BackendError>,
stderr: Captured }`. The failure sits beside the capture rather than around it, so
no path can produce one without the other. The two places that legitimately have
no stderr — a command that never spawned, and a missing stdio handle — construct
it through `Exchange::failed`, which names that fact instead of leaving a reader
to wonder whether the empty capture is a bug. §5.2, D40, and the §5.4 sketch and
sequence diagram follow it through.

**Outcome:** `repaired` (round 3)

### F-40 — Abandoned stderr drains accumulate live tasks and descriptors without bound

**Severity:** major
**Location:** `design.md` §5.4, "Where this can still stall"

**Expected:** A backend cannot cause unbounded host resource growth, and a completed exchange leaves no transport work running indefinitely.
**Observed:** After the grace timeout the stderr `JoinHandle` is abandoned while the task still owns the pipe and an `Arc` holding up to 256 KiB. Dropping a tokio `JoinHandle` detaches rather than cancels. A backend can repeatedly leave stderr inherited by long-lived grandchildren, accumulating one task, pipe descriptor, allocation and `Arc` per exchange.
**Evidence:** `design.md` §5.4 says the grandchild can keep the pipe open indefinitely and that the host "abandon[s] the task". Tokio's `task::JoinHandle` contract says dropping the handle detaches the task. Contradicts `design.md` I11 and `draft-spec.md` R-45.

**Disposition:** `fix-now`
**Response:**

Correct, and it defeats the cap it sits next to. Dropping a tokio `JoinHandle`
detaches the task; it does not cancel it. So a backend that leaves a grandchild
holding the stderr fd on every exchange accumulates one live task, one pipe
descriptor and up to 256 KiB per call, indefinitely — a backend causing unbounded
host resource growth, which is precisely what I11 forbids and what capping the
buffers was meant to prevent. Bounding each buffer is worth nothing if the number
of buffers is unbounded.

The join now `abort()`s on the grace timeout. That this is safe rather than
merely tidy is F-27's doing: the bytes live in a caller-owned
`Arc<Mutex<Captured>>`, so aborting the task discards the task and not the
capture. The two repairs are load-bearing together — either alone would be
wrong. D41; I11 extended to name tasks and descriptors, not only memory.

**Outcome:** `repaired` (round 3)

### F-41 — The allegedly unconditional reap is bypassed by post-spawn return and cancellation paths

**Severity:** major
**Location:** `design.md` §5.4 process sketch; D35

**Expected:** Every path that abandons a spawned backend explicitly terminates and reaps it; drop-time cleanup alone is insufficient.
**Observed:** `child.stderr.take().ok_or(BackendError::PipeMissing)?` can return after spawning but before the unconditional `reap` site. Dropping the `exchange` future also bypasses all code after the timeout match, and the design expressly leaves cancellation to `kill_on_drop`, which cannot await reaping.
**Evidence:** The process sketch places the `PipeMissing` `?` before `reap(&mut child).await`. The following prose keeps `kill_on_drop` as the backstop for "panic and cancellation paths". `draft-spec.md` R-48 requires termination and reaping on every abandonment path and says drop-time cleanup alone is insufficient.

**Disposition:** `fix-now` on the return path; `doc-wrong` on R-48
**Response:**

Two halves, and only one of them is fixable.

**The `?` is a straight bug.** `child.stderr.take().ok_or(PipeMissing)?` returns
after the child exists and before `reap`, leaving the very cleanup D35 calls
unconditional to the mechanism D26 says not to rely on. The sketch now uses a
`let … else` that reaps before returning, and the spawn failure — the one case
with no child — returns before any child exists. The rule is stated over the
region rather than the line: no `?` past the spawn.

**R-48 was unachievable as I wrote it**, and this is `doc-wrong` rather than a
convenient narrowing, so it is worth being explicit about the difference. If the
exchange future is dropped, no code of ours runs: there is nothing to await a
reap with, and no amount of design changes that. A requirement that says
"reliance on drop-time cleanup alone is insufficient" over *every* abandonment
path therefore forbids the only mechanism that exists on one of them. R-48 now
binds every path that **returns** — which is where the real bug was — and states
plainly that cancellation relies on `kill_on_drop`, rather than pretending
otherwise. Narrowing a requirement to meet it is the move round 3's instructions
asked the reviewer to watch for; the defence here is that the old wording was not
merely unmet but unmeetable, and the part that *was* meetable got fixed rather
than written down.

**Outcome:** `repaired` (round 3)

### F-42 — Reaping failures are discarded after the outcome has already been fixed

**Severity:** major
**Location:** `design.md` §5.4 process sketch

**Expected:** Failure to kill or wait for a child is reported, and cannot be returned as a successful, fully reaped exchange.
**Observed:** The design decides `outcome` first and then calls `reap(&mut child).await` with no specified error path. Both `Child::start_kill` and `Child::wait` are fallible, so a kill or wait failure must either be swallowed or overwrite an already-selected error with no defined precedence.
**Evidence:** `design.md` §5.4 says "the error is decided before the reap" and sketches `reap(&mut child).await; attach_stderr(outcome, …)`. `draft-spec.md` R-47 requires every refusal to be reported; R-48 requires the child actually to be terminated and reaped.

**Disposition:** `fix-now`, with a user decision on precedence; `doc-wrong` on R-47
**Response:**

Real: `start_kill` and `wait` are both fallible and the sketch discarded their
results, so a child that could not be killed would be reported as a clean
exchange.

The user chose the precedence rule. In full: *already exited* is success, since
reaping unconditionally means most reaps run against a process that has already
gone, and "idempotent" has to mean that or D35 is unimplementable; a reap failure
with no prior error becomes `BackendError::Reap`; a reap failure alongside an
existing error is dropped. The last clause is the one that needs defending, and
the defence is informational rather than aesthetic — "we also could not kill it"
is a consequence of the timeout or overflow that made us abandon the child, and
reporting both buries the cause under its effect.

That rule needs **R-47 narrowed**, and flagging it as `doc-wrong` rather than
letting it pass is the point. R-47's "every refusal MUST be reported" is about
values the *backend supplied*: the sender can act on those, which is the whole
reason the requirement exists. It was never about the host's internal cleanup
telemetry, and read literally it contradicts the rule above. R-47 now says so,
and R-48 carries the reporting obligation for reap failures instead — so the
obligation moved rather than evaporated. D42.

**Outcome:** `repaired` (round 3)

### F-43 — The stdout-cap contract requires both stopping and not stopping the read

**Severity:** major
**Location:** `design.md` I11, D34; `draft-spec.md` R-43; `slice-001.md` AC-5

**Expected:** The design, the requirement and the acceptance criterion prescribe one consistent behaviour when stdout reaches its bound.
**Observed:** D34 requires `read_capped` to stop reading and drop stdout at 8 MiB. I11, R-43 and AC-5 all state that reaching a bound never stops the read, without limiting that to stderr. These cannot all be implemented.
**Evidence:** `design.md` D34: "stdout stops-and-closes". `design.md` I11: "capping never stops the read". `draft-spec.md` R-43: reaching a bound "MUST NOT stop the host reading the stream". `slice-001.md` AC-5 applies the same statement to both reads.

**Disposition:** `fix-now`
**Response:**

My generalisation, and sloppy. D34's whole argument is that the two streams
differ at the bound: stdout stops and closes, because the exchange is already
failing and closing the pipe is what makes the flood stop; stderr truncates what
it stores and keeps draining, because a chatty backend that works is not a broken
one and a full pipe nobody reads is a deadlock. I then wrote the stderr half as
though it were the rule for both, in I11, in R-43 and in AC-5 — three
restatements of a contract that contradicted its own source.

All three now qualify the claim to stderr and state the stdout behaviour beside
it. R-43 in particular is now longer, because the honest form of this requirement
is two sentences and the compressed form was the error.

**Outcome:** `repaired` (round 3)

### F-44 — The draft's canonical response example uses the rejected nested-hints spelling

**Severity:** major
**Location:** `draft-spec.md` §6.2

**Expected:** Field hints are flat keys, and the nested `hints` spelling is not accepted as an alternative representation.
**Observed:** The principal response example writes `"hints": { "units": "min" }`. With `WireField.hints` flattened, that is captured as one hint whose key is literally `"hints"` and whose value is an object — it neither produces the intended flat `units` hint nor is rejected as the forbidden nested spelling.
**Evidence:** `draft-spec.md` §6.2's response example contains the nested form; the same section's field-forms subsection says every hint is flat and rejects supporting a nested `hints` object. `design.md` D37 makes the same flat-only decision.

**Disposition:** `fix-now`
**Response:**

Embarrassing and exactly the class F-34 named: §6.2's principal response example
still wrote `"hints": { "units": "min" }`, two subsections above the text that
rejects the nested spelling. Under `#[serde(flatten)]` that example does not
produce a `units` hint and is not rejected either — it produces one hint whose key
is literally `"hints"`. The document's most-copied artefact contradicted the rule
the document had just adopted.

The example now writes `"units": "min"` flat. The field-forms subsection was
already correct, which is the tell: I fixed the normative text and left the
example, and the example is what a backend author actually reads.

**Outcome:** `repaired` (round 3)

### F-45 — Kind-inapplicable semantic keys have no defined rejection path

**Severity:** major
**Location:** `design.md` §5.2, `WireField`; `draft-spec.md` R-16

**Expected:** `min` and `max` are protocol keys only for `number`, and `options` only for `choice`; using a modelled key with another kind is rejected rather than silently ignored.
**Observed:** One shared `WireField` extracts `min`, `max` and `options` before dispatching on `kind`. No normalization check, error case or fixture exists for a text field carrying `min`, a number field carrying `options`, or a choice field carrying bounds. Those keys cannot become hints after serde has consumed them, so the input is silently lost.
**Evidence:** `design.md` §5.2 defines all three members on `WireField`; its edge table and §9 fixture list contain no inapplicable-key case. `draft-spec.md` R-16 says the other field kinds carry no additional protocol keys, and R-47 forbids silently absorbing invalid values.

**Disposition:** `fix-now`
**Response:**

Correct, and it undercuts the claim D37 rested on. `WireField` declares `min`,
`max` and `options` for all five kinds because one struct deserializes all five
and `kind` is only read afterwards — so serde consumes those keys *before*
dispatch, and they can no longer fall through to `hints`. A `min` on a text field
therefore vanishes with no error and no hint: silent absorption, which brief §3.3
and R-47 both forbid.

The user chose rejection over treating them as hints. That is the right call and
the reason is D37's own: unknown keys are presentation, known keys are contract,
so a contract key in a position where the contract gives it no meaning is a
contradiction rather than a decoration. Treating it as a hint would also make
`{"kind":"text","min":1}` a *successful* parse carrying a hint the renderer is
forbidden to branch on — worse than losing it, because it looks like it worked.

Normalization now checks applicability and raises
`InapplicableKey { key, kind, at }`, with the path for the same reason
`UnsupportedPrimitive` carries one. New requirement **R-50**, stated so it does
not collide with R-15: R-15 governs keys the spec does not name at all; R-50
governs named keys used where their kind gives them no meaning. Both edge-table
rows and both fixture directions — the misplaced key rejected, the unnamed key
still becoming a hint — are in §5.5 and §9. D43.

Worth recording what this does to D37's cost statement. D37 claimed the exposure
from flattening was narrow, being limited to misspelled *optional* keys. That was
only true if misplaced *modelled* keys were caught, and they were not — so the
claim was correct about a design that did not yet exist. It does now.

**Outcome:** `repaired` (round 3)

### F-46 — `view: null` still has two incompatible state-transition definitions

**Severity:** major
**Location:** `design.md` §5.4 state diagram and §5.5 edge table

**Expected:** A null view answering `respond` clears the answered interaction; a null view answering `evaluate` preserves an existing one.
**Observed:** The state diagram implements the request-dependent rule, but the edge table states without qualification that `view: null` leaves "any outstanding interaction" alone. An implementation following the table retains an interaction after its accepted answer.
**Evidence:** `design.md` §5.4: `Outstanding --> Idle: respond(matching id), view: null`. `design.md` §5.5: `view: null` means "any outstanding interaction is left alone". `draft-spec.md` §5 distinguishes the two requests explicitly.

**Disposition:** `fix-now`
**Response:**

Same class as F-44 and same cause. F-29 was repaired in the draft spec's §5, which
now distinguishes what `view: null` means answering an `evaluate` from what it
means answering a `respond`, and the design's state diagram already had it right.
The design's edge table did not: it said flatly that any outstanding interaction
is left alone, which an implementer following the table would read as "retain the
interaction after its answer was accepted".

The row is now two rows, one per request kind, each saying why. Splitting rather
than qualifying because the two behaviours are genuinely different edges, not one
edge with a caveat.

**Outcome:** `repaired` (round 3)

### F-47 — The decision and risk indexes retain mutually incompatible stderr contracts

**Severity:** minor
**Location:** `design.md` §7 D12, D18; §8 R8, R9

**Expected:** Restatements consistently identify `Exchange` / `Outcome` as stderr's owner, and require partial stderr to survive a timeout.
**Observed:** D12 still says the transport returns `Vec<u8>`. Struck D18 and risk R8 still say `Timeout` carries stderr, which that variant no longer does. R9 says the grandchild case reports the timeout "without stderr", contradicting the caller-owned-buffer repair.
**Evidence:** `design.md` D33 says the transport returns `Exchange { stdout, stderr }`; `BackendError::Timeout` has no stderr field; §5.4 says the `Arc<Mutex<Captured>>` exists so accumulated stderr survives abandonment; §8 R9 says to report the timeout without stderr.

**Disposition:** `fix-now`
**Response:**

Accepted, and the severity is generous — these are the same defect as F-43, F-44
and F-46, found in the indexes instead of the tables. D12 still described the
transport as returning `Vec<u8>`; struck D18 and risk R8 still said `Timeout`
carries stderr, which it stopped doing when F-24 moved the capture onto
`Exchange`; R9 still said the grandchild case reports without stderr, which F-27's
caller-owned buffer had already made false.

All four now match §5, and each carries the finding id that corrected it so the
next reader can see it was swept rather than never wrong.

The class matters more than the four instances, and it is now written into §9 as
a review step rather than left as a resolution to be more careful: **after any
change to §5, sweep the invariant and edge tables, the decision index, the risk
table, the AC map and fixture list, the draft spec's requirements and examples,
and the affected AC text in the slice card.** Six of round 3's nine findings were
this one pattern. The redundancy that causes it is deliberate — each contract is
stated where a reader will meet it — so the answer is to pay its cost on every
change, not to remove it.

**Outcome:** `repaired` (round 3)

### F-48 — F-42's repair still permits an exchange to return without successfully reaping the child

**Severity:** blocker
**Location:** `design.md` §5.4, D42; `slice-001.md` AC-5; `draft-spec.md` R-48

**Expected:** Every path that returns from an exchange has actually terminated and reaped the backend. A cleanup failure may have reporting precedence rules, but those rules cannot waive the lifecycle obligation.
**Observed:** The lifecycle sketch explicitly discards a reap failure whenever an earlier exchange error exists — `(Err(prior), _) => Err(prior)`. The rationale says the reap failure is deliberately dropped because the timeout or overflow is more informative. That settles which error is reported, not whether the process was terminated and reaped. The function can return after `reap()` itself reported failure.
**Evidence:** AC-5 requires every returning path to have "terminated and reaped the backend first". R-48 repeats that obligation before separately granting reporting precedence to an existing failure. The repair conflates "do not report the secondary cleanup error" with "the cleanup need not have succeeded". This reopens F-42: its repair is wrong.

**Disposition:** `fix-now`, reversing D42
**Response:**

Accepted, and the reviewer's own amendments to the proposed repair were taken
over mine on all three points.

The finding is right that precedence is not discharge: "do not report the
secondary error" and "the cleanup need not have succeeded" are different claims,
and D42 collapsed them. What made this reachable rather than theoretical is F-53,
which forces the reap to be bounded — an unbounded `wait` can block forever, and
a host blocked inside an exchange has been taken down by a backend.

Three corrections to what I first proposed, all the reviewer's:

1. **Two dimensions, not a precedence rule.** `Exchange` and `Outcome` now carry
   `cleanup: Option<CleanupFailure>` beside `result`/`failure`. What the backend
   did and whether the host disposed of it are independent facts; D42 got into
   trouble by forcing them into one channel, and ranking them was answering the
   wrong question. All four combinations are meaningful and §5.4 tabulates them.
   The precedence problem does not get solved, it stops existing — which is the
   tell that the shape was wrong rather than the ranking.
2. **`CleanupTimeout`, not `Orphaned`.** My name asserted a process state the
   failure path never establishes. Running it settles it: the case that actually
   fires is a backend that answered correctly and left `(sleep 30) &` holding the
   pipes — the child itself exits and is reaped, and only the drain stalls.
   `Orphaned` would have been a false statement about the commonest way the
   variant occurs.
3. **Keep the original failure.** "Backend timed out, then cleanup also timed
   out" is strictly more diagnostic than either alone. Two fields keep both with
   no recursive error type.

I13, R-48 and AC-5 are restated from "has reaped" to "initiates termination and
waits a bounded interval, and reports failure to observe cleanup". That is a
narrowing on its face, and the test of whether it is honest is whether an
obligation disappeared. It did not: *must reap, potentially forever* became *must
attempt within a hard bound* **and** *must report inability to establish
cleanup*, which is a stronger operational contract because it also protects host
liveness. D47, D48.

**Outcome:** `repaired` (round 4)

### F-49 — Cancellation reintroduces F-40's detached stderr-drain leak

**Severity:** major
**Location:** `design.md` §5.4, I11; `draft-spec.md` R-48

**Expected:** The F-40 repair must prevent an abandoned exchange from leaving a live stderr-drain task, pipe descriptor and retained buffer behind. Narrowing R-48's child-reap guarantee for cancellation must not silently discard the separate drain-task cleanup obligation.
**Observed:** The transport spawns the stderr drain as an independent tokio task. The only explicit abort happens in `collect_stderr`, after the body and the reap complete. If the enclosing exchange future is cancelled before reaching that call, its `JoinHandle` is simply dropped. The design itself explains that dropping a `JoinHandle` detaches rather than cancels, and that with an inherited stderr descriptor this leaves a task, descriptor and buffer alive indefinitely.
**Evidence:** I11 is unqualified: no exchange leaves a task, buffer or descriptor behind. R-48 carves out cancellation only by appealing to the child's `kill_on_drop`; it provides no equivalent disposition for the already-spawned drain task. This reopens F-40 and the cancellation portion of F-41: the child obligation was narrowed, but the drain obligation disappeared.

**Disposition:** `fix-now`, structurally — the repair is a deletion
**Response:**

Correct, and it is the third finding against one mistake, which is what finally
made the mistake visible. F-27 said the spawned task must not own the buffer, so
I added an `Arc<Mutex<Captured>>`. F-40 said the task must not outlive the
exchange, so I added `abort()`. F-49 says the abort cannot help on the one path
that runs no code of ours — cancellation drops the `JoinHandle`, and dropping a
handle detaches rather than cancels.

Three repairs to one decision is a signal I should have read two rounds earlier.
The mistake was `tokio::spawn` itself. The drain never needed a *task*; it needed
to make progress *concurrently*, and `select!` inside the existing task does that
exactly as well. So the repair removes code rather than adding it: no spawn, no
`Arc`, no `Mutex`, no `abort`, no join handle — the drain is a sub-future
borrowing a `&mut Captured` on the caller's stack.

Verified rather than argued, since the whole finding turns on a claim about
cancellation. Built both shapes: a sub-future's destructor runs the instant its
parent is dropped; a spawned task's does not, and is still running 100 ms later.
Also confirmed the concurrency it exists for — 4000 stderr lines, well past the
64 KiB pipe buffer, drained while the body read stdout, no deadlock.

This retires D36 and D41 rather than amending them. D36 claimed this was "the one
place a lock is right"; the honest reading is that D14 had already given the
right answer — brief §12 gives the host no concurrency to protect against — and
this design talked itself out of it by introducing the concurrency first. D44.

**Outcome:** `repaired` (round 4)

### F-50 — Explicit `null` is silently converted into omission for modelled fields

**Severity:** blocker
**Location:** `design.md` §5.2 wire types; `draft-spec.md` R-25, R-47; `slice-001.md` AC-6

**Expected:** An invalid backend-supplied `next_check` is discarded with a typed scheduling error and reported; it must not silently acquire the semantics of an omitted instruction.
**Observed:** `WireResponse.next_check` is `Option<serde_json::Value>`. The design immediately above documents the relevant serde behaviour — an `Option` maps both an absent field and an explicit `null` to `None`, which is why `view` needs a presence-preserving deserializer. No such mechanism exists for `next_check`. The stated reason for using `Value` is precisely to retain malformed values such as `45` so they produce a typed discard, but `null` never reaches that path.
**Evidence:** The design says every invalid `next_check` is discarded with a typed diagnostic, while absence alone has fallback semantics. R-25 requires an invalid value to be "discarded and reported"; R-47 forbids silently absorbing invalid backend-supplied values. AC-6 cannot be met for `next_check: null` by the specified wire type. The same structural problem exists for `protocol: Option<u32>`: an explicit `protocol: null` collapses into the permitted "protocol omitted" state.

**Disposition:** `fix-now`, as a statement rather than a type change
**Response:**

The observation is exact and I verified it before disposing: `{}` and
`{"next_check": null}` both deserialize to `None`, as do `{}` and
`{"protocol": null}`. So `null` and omission are indistinguishable for every
modelled field except `view`, which has the presence-preserving deserializer.

The user's decision is that this behaviour is right and the **silence about it**
was the defect. The reasoning is about who is on the other end: `null` is what an
ordinary serializer emits for an absent optional — `json.dumps({"next_check":
None})` is not a backend doing anything wrong — so treating it as an invalid
value would report a discard against most well-formed messages, from backends
written in the languages the brief expects. A wrong *type* is different in kind:
`"next_check": 45` is a value the backend meant, in a shape the protocol cannot
use, and that stays a reported discard.

So the repair is a rule, stated once and generally: **an explicit `null` means
what omission means, except where the protocol defines a distinct meaning for
`null`.** There is exactly one exception and the rule names it. That also
explains why `view` needs machinery the other fields do not, which the design had
implemented without ever saying. New requirement R-51; edge rows for all three
fields; and the fixture asserts an **empty** discard list, because here the
silence is the contract. D50.

**Outcome:** `repaired` (round 4)

### F-51 — ADR-001's dependency constraint cannot be satisfied by the proposed single-crate layout

**Severity:** blocker
**Location:** `design.md` §3, §5.1; `slice-001.md` governing canon / AC-15

**Expected:** Binding ADR-001 requires stratum 1 to build and test with no async runtime in its dependency graph.
**Observed:** The design puts stratum 1 and the tokio-using shell in one crate, then argues that tokio is a "stratum 2 dependency" and so stratum 1 does not acquire it. Cargo dependencies belong to a crate target, not to individual modules. Building or testing this library builds one crate whose dependency graph contains tokio; `semantics/` has no independently selectable dependency graph under the proposed design.
**Evidence:** The design effectively acknowledges the problem: its mechanical test inspects only source references to `tokio`, `crate::shell` and `crate::bin`, and says explicitly that `cargo tree` cannot observe a module boundary inside one crate. AC-15 is a partial source-level mitigation, while the governing-canon statement is stronger: the semantic core "must build and test with no renderer and no async runtime in its dependency graph". The design proves "semantics does not import tokio"; it does not, and in the proposed crate structure cannot, prove the binding build property.

**Disposition:** `fix-now`
**Response:**

Accepted as a blocker. This is the most serious finding since round 1 and the
first in three rounds to reach past the review's own wake into the original
design.

The design answered ADR-002's T1 with "tokio is a stratum 2 dependency and
stratum 1 does not link it". That is simply false. Cargo resolves dependencies
per crate target, not per module: in a single crate with a plain tokio
dependency, `cargo test` builds one graph containing tokio and `semantics/` has
no separately selectable graph at all. ADR-001's Decision — "no runtime in its
dependency graph" — was therefore untrue rather than merely unenforced, and
AC-15's grep proved the much weaker claim that `semantics/` contains no `tokio`
token. The design had even written down that `cargo tree` cannot see a module
boundary, and then relied on the thing that sentence rules out.

The user chose the feature gate over splitting to a workspace now. tokio becomes
an optional dependency behind a `shell` feature, with `shell/` under
`#[cfg(feature = "shell")]`. Verified by building it: `cargo tree
--no-default-features` has no tokio node, and `cargo test --no-default-features`
compiles and runs stratum 1 against serde, serde_json and jiff alone.

That converts half of ADR-001 from a review gate into a **build gate**, inside
one crate, which the ADR had assumed impossible before the split — so CD-1 gets
stronger rather than merely more accurate. The direction half stays a review
gate; the delta now says which is which.

Two canon consequences, both recorded rather than absorbed. ADR-002 names Slint
as "the first such dependency" to fire T1; tokio arrived a slice earlier and was
admitted only by gating it, so CD-2 corrects the annotation and records that
"make it optional" is an available answer to T1 — while noting, from ADR-002's
own rejected alternatives, why that answer does not extend to a Slint
build-dependency and so does not defer the split. D49.

**Outcome:** `repaired` (round 4)

### F-52 — Duplicate field ids produce a canonical interaction that cannot be answered unambiguously

**Severity:** major
**Location:** `design.md` §5.2 canonical types; `draft-spec.md` R-8, R-15

**Expected:** A canonical interaction must not contain identifiers whose response representation cannot distinguish the corresponding values.
**Observed:** The design makes option ids unique because duplicates make `respond` ambiguous, and `Options` has a checked constructor accordingly. But an option's fields remain a bare `Vec<Field>` with no field-id uniqueness invariant, while a response represents submitted values as a `BTreeMap<FieldId, Value>` — so two fields in one option sharing an id have a single response key and cannot be answered independently.
**Evidence:** R-8 fixes the response representation as a map from field id to submitted value; R-15 requires only that each field have an id, with no uniqueness rule. The verification plan for R-15 checks only options with and without fields, so the ambiguity is not exercised. This contradicts the design's own rationale for rejecting duplicate option ids and permits an invalid canonical state.

**Disposition:** `fix-now`
**Response:**

Correct, and it is my own argument left unapplied one level down — the same shape
as F-39. `Options` has a checked constructor precisely because duplicate option
ids make `respond` ambiguous about which option it names. `UserResponse.values`
is a `BTreeMap<FieldId, Value>`, so two fields in one option sharing an id have
one key between them and cannot be answered independently. Identical defect,
identical consequence, and the design used a bare `Vec<Field>`.

Repaired with the rule rather than the case, since three collections now need it:
**every identifier the response format uses as a key must be unique within the
scope that response addresses.** `Options`, `Fields` and `Alternatives` are
checked newtypes expressing it. `DuplicateFieldId { id, at }` joins the taxonomy,
and `EmptyOptions` and `DuplicateOptionId` gain paths for F-6's reason — with
alternatives and fields there are now several sites a duplicate can occur at.

The fixture that matters most is the negative one: the same field id in two
*different* options is legal and must be accepted, since that is what shows the
scope of the rule is right rather than merely strict. R-52, D45, I15.

**Outcome:** `repaired` (round 4)

### F-53 — The configured timeout does not cover the whole exchange as the spec requires

**Severity:** major
**Location:** `design.md` §5.4; `draft-spec.md` R-41

**Expected:** The configured timeout covers the whole process exchange.
**Observed:** The prose says all exchange steps sit inside one `tokio::time::timeout`. The sketch wraps only request writing, stdout reading and `child.wait()`. After it expires the design performs `reap(&mut child).await` outside the configured timeout and then waits again for the stderr drain under a separate grace timeout.
**Evidence:** R-41 is unambiguous: "A configured timeout covers the whole exchange." As designed, a call can exceed it by however long the reap takes plus the stderr grace. The executable sketch and the normative requirement specify different latency contracts.

**Disposition:** `fix-now`
**Response:**

Correct on the letter, and it is what makes F-48 reachable. R-41 said the
configured timeout covers the whole exchange; the sketch put the reap and the
stderr grace after it, so a call could exceed the configured bound by an
unstated amount.

The requirement cannot be made true as written — killing a child and reaping it
necessarily happen *after* the timeout that gave up on it. So the fix states the
real contract instead: `config.timeout` bounds the backend's opportunity to
respond, one `CLEANUP_LIMIT` bounds disposal, and a call waits at most the sum.

Two things beyond the wording. The two separate graces I had — one for the reap,
one for the drain — collapse into a single cleanup budget, which is simpler and
matches the reviewer's point that a cleanup timeout naturally covers drain
shutdown too. And the reviewer's condition on this disposition is honoured
explicitly: the bound is *in* the stated deadline semantics, not a hidden grace
period behind a configured number. Measured on the worst case — a grandchild
holding both pipes — 902 ms against a stated 900 ms bound, the remainder being
scheduling rather than waiting, which is why R-41 bounds what the host waits for
rather than promising a real-time guarantee. D48.

**Outcome:** `repaired` (round 4)

### F-54 — A choice field's options may carry fields the response format cannot express

**Severity:** major
**Location:** `design.md` §5.2 canonical types; `draft-spec.md` R-8, R-16

**Raised by:** the responder, while disposing F-52 — the same defect one level deeper, and the reason F-52's fix has to choose a scope.

**Expected:** Every field the protocol admits into a view can be submitted by the response format that answers it.
**Observed:** `FieldKind::Choice { options: Options }` reuses `Options`, whose element `Opt` carries `fields: Vec<Field>`. A choice *field*'s options may therefore carry fields of their own, recursively. `UserResponse` is `{ option: OptionId, values: BTreeMap<FieldId, Value> }` — one option id and one flat map — so there is no way to express which nested option was chosen, and a nested field's id shares a namespace with every outer field's id.
**Evidence:** `design.md` §5.2 defines `Opt` with `fields` and `FieldKind::Choice` over the same `Options` newtype. `draft-spec.md` R-8 fixes the flat response shape; R-16 says a `choice` field "MUST carry its own `options`" without excluding fields on them. F-20 examined the `Options` reuse and found no defect, but considered only the view side; the response side is where it fails. Nothing in the brief requires nested fields, so this is admitted surface that no requirement asked for — R-7 in §8's risk table names exactly this failure mode.

**Disposition:** `fix-now`
**Response:**

Raised by me while disposing F-52, and the user chose to narrow the type.

`FieldKind::Choice` reused `Options`, whose `Opt` carries `fields`, so a choice
field's options could carry fields recursively — while `UserResponse` is one
option id and one flat map. There is no way to say which nested option was
chosen, and a nested field's id shares a namespace with every outer field's.
That is admitted surface no requirement asked for and no response can express.

A choice field's option is now `Alternative { id, label }`. This deletes the
recursion rather than documenting it, which is the cheaper half of P3 and exactly
R-7's gold-plating risk caught before it shipped. Brief §10.2 puts fields on a
*view's* options and never on a field's, so nothing is lost.

Worth recording against F-20, which examined this same reuse and found no defect.
F-20 was not careless; it checked the view side, where the reuse is harmless. The
defect only appears when the type is read against the message that has to carry
an answer to it — the identical method that found F-31 and F-38. Checking a type
against itself is not the same as checking it against its round trip. D46, I16,
R-53.

**Outcome:** `repaired` (round 4)

### F-55 — The F-54 repair silently drops `fields` on a choice field's option

**Severity:** major
**Location:** `design.md` §5.2 wire and canonical types, §5.5 edge table; `draft-spec.md` R-53

**Raised by:** the responder, self-checking the round-4 repairs before claiming them done.

**Expected:** `fields` appearing on a `choice` field's option is refused with a named error, since R-53 forbids it and the response format cannot carry it.
**Observed:** The F-54 repair's edge-table row said the key is "unmodelled on `Alternative`, so ignored under I10". That confuses two layers: whether a key is modelled is a property of the **wire** type, not the canonical one, and the design never defines `WireOpt` at all — it is referenced by `WireChoice.options` and shown nowhere. If a choice field's options were to deserialize through a shared `WireOpt` carrying `fields`, serde would consume the key and it would vanish with no error and no hint.
**Evidence:** `design.md` §5.2 references `Vec<WireOpt>` without defining it. The same section, two subsections earlier, establishes for F-45 that a modelled key consumed before dispatch "cannot fall through to `hints`" and must be refused. `draft-spec.md` R-53 says a choice field's options "MUST NOT carry fields" and supplied no error, making it unenforceable prose. This is F-45's defect reintroduced by the F-54 repair, on the same page that repairs F-45.

**Disposition:** `fix-now`
**Response:**

Accepted without argument — it is mine, found by checking my own repairs against
the round they came from rather than against the finding they answered.

Two fixes. `WireOpt` is now defined, and defined as **the view's** option type,
which is the only place `fields` is admitted. And because `WireField.options` is
`serde_json::Value` and so dispatched by normalization rather than bound by
serde, the dispatch checks for `fields` explicitly and raises
`InapplicableKey { key: "fields", kind: "choice", at }` — the same variant F-45
introduced, for the same reason: `fields` is a protocol key everywhere the
protocol admits it, so this is the wrong *place* for a known key, not an unknown
key.

Two things worth recording beyond the fix. First, a dangling type name is a real
defect and not a presentational one: `WireOpt` was cited but never shown, and the
question it silently left open — does a choice field's option deserialize through
the same type as a view's? — is exactly the question the repair turned on.
Second, R-53 was a `MUST NOT` with no error behind it. A prohibition the taxonomy
cannot express is prose, and this ledger has now produced that shape twice (R-30
was the other, at F-28).

**Outcome:** `repaired` (round 4, self-raised)

### F-56 — The round-4 repairs were not swept through their restatement sites

**Severity:** major
**Location:** `design.md` §5.1, §5.4, §5.5 I11/I13, §7 D40, §9 AC map, §10;
`draft-spec.md` R-47; `design-log.md`

**Raised by:** the responder, re-reading the round-4 batch before the round-5 packet went out.

**Expected:** §9's own review step — *after any change to §5, re-read the invariant and edge tables, the decision index, the risks, the AC map and fixture list, `draft-spec.md`, and the affected AC text in the slice card* — is run against the round-4 batch, so no site still states a contract those repairs replaced.
**Observed:** Nine sites still state the pre-repair contract:

1. `design.md` §5.4 step 5 — "All of the above inside **one `tokio::time::timeout`** covering the whole exchange". F-53's defect verbatim: the sketch 55 lines below puts kill, reap and drain under a separate `CLEANUP_LIMIT`, and D48 and R-41 both say the total is the sum.
2. `draft-spec.md` R-47 — "where the host both fails an exchange and then fails to clean up after it, R-48 says which of the two is reported". D42's precedence rule, which D47 reversed; R-48 now says cleanup "MUST NOT be suppressed" and R-54 requires all four combinations distinguishable. Two requirements in one file, opposite answers.
3. `design.md` §5.1 — "This does **not** promote ADR-001's verification from review gate to build gate", written for AC-15's grep but stated over ADR-001's verification as a whole, 35 lines after D49 makes the dependency-graph half exactly that. Contradicts AC-15's own text in `slice-001.md` and CD-1.
4. §5.1, same paragraph — "gives the no-tokio-in-stratum-1 constraint a check it could not otherwise have, since `cargo tree` cannot see a boundary inside a single crate". D49 is that other check.
5. §5.5 I11 — held by "D27, D34, **D41**". D41 is struck, superseded by D44.
6. §5.5 I13 — held by "D35, **D42**". D42 is reversed by D47.
7. §7 D40 — describes `Exchange { result, stderr }`; the type has carried `cleanup` since D47.
8. §10 — "`canon-delta.md` … one entry (CD-1)", while the row directly above it cites CD-2.
   Also §5.4's sequence diagram, which returns `Exchange { result, stderr }` and an `Outcome` with no `cleanup` — the same two-field shape as D40, in a picture rather than a table, and the site a reader is most likely to skim as authoritative.
9. §9's AC map — "the four commands above", where five are listed; and `slice-001.md` AC-1 names four activities and never `--no-default-features`, which AC-15 now requires. Also §10's ADR-001 row, which still describes AC-15 as mechanising "part of" a review gate without naming the half that is no longer one.

More of the same family, found by running the two mechanical checks this finding's repair adds to §9, and repaired with it:

- `cleanup_only(&mut child, …)` is called in §5.4 and defined nowhere — F-55's dangling-`WireOpt` defect, in the section F-55's own repair touched.
- The variant F-48 spent a paragraph naming is `CleanupTimeout` in every piece of prose, the edge table, the risk table, the fixture list and §9's evidence — and `CleanupFailure::TimedOut` in the one place it is actually declared. The design argued the name and then did not use it.
- `ViewId`, `OptionId`, `FieldId`, `Timestamp` and `Hints` are written throughout §5.2 and declared nowhere, as is `Config`, which §5.4's sketch dereferences as `config.timeout`. §5.2 gives a TOML sample and no parsed type.
- `design-log.md` has no entry for F-55 at all, its round-4 heading reading "F-48…F-54", so the only record of that finding is this ledger.
- The invariant table runs I11, I13, I15, I16, I14, I12.

**Evidence:** §9 states the sweep as a standing review step and gives its provenance — six of round 3's nine findings were this one pattern. Round 4 changed more §5 contracts than round 3 did, across more sites (the cleanup dimension alone restates in `Exchange`, `Outcome`, I13, R-41, R-48, R-54, AC-5, AC-6, the edge table and §5.4's own table), and the sweep was not run before the batch was claimed done.

**Disposition:** `fix-now`
**Response:**

Accepted without argument; it is mine, and it is the failure the document
predicted about itself. The notes handed to round 5 named "the cleanup
dimension's restatements" as the fourth of four places new defects were most
likely, on the grounds that it has more sites than any prior change. That
prediction was correct and I wrote it without acting on it — naming a sweep as
outstanding is not the same as running it.

Every site above now matches §5, each carrying the finding id that corrected it.
Beyond the mechanical repair, two things are worth recording.

The **class is unchanged since round 3** and the countermeasure has now failed
once. §9's sweep was written as a review step precisely because no test can
observe that two English sentences disagree, and a review step is only worth the
discipline of running it. What this round shows is that the discipline does not
survive a large batch: the sweep was not skipped through carelessness on a single
change, it was not run at all against a batch of eight repairs. So it is restated
in §9 as a **per-batch** obligation with an explicit trigger — before any repair
batch is claimed complete, not after any single change — since "after any change"
is exactly the phrasing that let a batch fall between changes.

And the two dangling names are the *same* defect as the drift, not a separate
tidiness point. `WireOpt` (F-55) and `cleanup_only` are both a document naming
something it never defines; the reason F-55 mattered was that the undefined name
concealed the question the repair turned on. `cleanup_only` concealed a real one
too — whether the pipe-missing path pays the cleanup budget — and the answer, now
written down, is that it must, because I13 admits no exception once a child
exists.

**Outcome:** `repaired` (round 4, self-raised)

### F-57 — The `shell` feature gate has no test-target plumbing, so its own verification command cannot run

**Severity:** major
**Location:** `design.md` §5.1 manifest and layout, §9 verification commands and tiers; `slice-001.md` AC-1, AC-15

**Raised by:** the responder, checking the least-examined surface of the F-51 repair.

**Expected:** `cargo test --no-default-features` — AC-15's build gate and the mechanical form of ADR-001's dependency rule — runs against the crate this design specifies.
**Observed:** A feature selects dependencies; it does not stop cargo building every test target in the package. The manifest at §5.1 declares no test targets and no `required-features`, while §9's integration tier spawns processes on tokio. As specified, `cargo test --no-default-features` builds the integration target, fails to compile, and the build gate is unrunnable from the moment that tier exists. The F-51 probe passed only because the crate it ran in had no integration tests.
**Evidence:** `design.md` §5.1's `[features]` block is the whole manifest given, and §5.1's tree lists `tests/protocol/` and `tests/integration/` as bare directories — which are not cargo targets at all unless each carries a `main.rs`. §9's `cargo clippy --all-targets` likewise runs under default features only, so every `#[cfg(not(feature = "shell"))]` path and stratum 1 compiled without the shell above it are never linted, while AC-1 claims "zero warnings" unqualified. AC-15 asserts the gate holds; nothing in the design makes it runnable.

**Disposition:** `fix-now`
**Response:**

Accepted. This is the surface the notes named first as least examined, and it is
the ordinary consequence of a repair verified by probe: the probe answered
"does the feature give stratum 1 a clean graph?" — which it does — and could not
answer "does the command still run in the crate this design describes?", because
the probe crate had no integration tier to break it.

Three things added, all of them plumbing the F-51 repair needed and did not get:

1. **Declared test targets** with `required-features = ["shell"]` on the
   integration one, plus `autotests = false`. Cargo then *skips* the target whose
   features are unmet rather than failing to build it, which is what makes the
   gate runnable rather than aspirational.
2. **`tests/protocol/main.rs` and `tests/integration/main.rs`** in the layout. A
   directory under `tests/` is not a target; `required-features` needs a target
   to attach to, so the entry points have to be named.
3. **A second clippy invocation** under `--no-default-features`. A feature-gated
   crate has a build matrix, and a matrix checked in one column is unchecked.
   AC-1 now says so explicitly rather than counting commands.

The general point, and the reason this is `major` rather than `minor`: **a
constraint enforced by a build command acquires the build's own failure modes.**
D49 converted half of ADR-001 from a review gate into a build gate, which is
strictly better — and a build gate that does not run is worth less than the
review gate it replaced, because it reads as green.

**Outcome:** `repaired` (round 4, self-raised)

### F-58 — R-52's uniqueness rule is scoped to keys, and an alternative id is not a key

**Severity:** major
**Location:** `draft-spec.md` R-52; `design.md` §5.2 canonical types, §5.5 I15, §7 D45

**Raised by:** the responder, checking the F-52 and F-54 repairs against each other.

**Expected:** The rule the design states as its reason for three checked newtypes actually covers all three.
**Observed:** R-52 and I15 both state it as *every identifier the protocol uses as a **key** in a response*, and R-52 enumerates two scopes: option ids within a view, field ids within an option. An alternative id is never a key. `UserResponse` is `{ option: OptionId, values: BTreeMap<FieldId, Value> }`; the answer to a `choice` field is an alternative id submitted as the *value* at `values[field_id]`. So the stated rule justifies `Options` and `Fields` and does not reach `Alternatives`, which D45 introduces under it, I15 claims to hold, and the §5.5 edge table enforces with `DuplicateOptionId`.
**Evidence:** `design.md` §5.2 gives `Alternatives` a checked constructor "for the same reason" as the other two; `draft-spec.md` R-52 names only two scopes and gives the key-collision argument for both. A `choice` field with two alternatives sharing an id has no key collision — it has an ambiguous submitted value, which is a different mechanism and one no requirement states.

**Disposition:** `fix-now`, by widening the rule rather than dropping the newtype
**Response:**

Accepted, and it is the F-52 repair's own weakness: F-52's virtue was stating a
rule instead of three cases, and the rule as stated was drawn from the two cases
in front of me at the time. `Alternatives` arrived from F-54, one finding later,
and was folded under a rule that does not describe it.

Widened rather than narrowed, because the newtype is right and the sentence was
wrong: **every identifier a response names must be unique within the scope that
names it.** Option and field ids are named as *keys* — a duplicate leaves one of
the pair unaddressable. An alternative id is named as a *value* — a duplicate
leaves the submitted answer ambiguous about which alternative the user picked.
Different mechanism, same defect, arriving from the other side of the map.

Worth recording as method: this is the same test that found F-52 and F-54 — read
the type against the message that must carry its answer — applied for once to a
*rule* rather than to a type. A rule that covers the cases that produced it is
not yet a rule. R-52 now names three scopes and says which of them is a key and
which is a value; the fixture corpus gains the duplicate-alternative case, which
was untested because no requirement asked for it.

**Outcome:** `repaired` (round 4, self-raised)

### F-59 — Process result is fixed before exit status is known

**Severity:** blocker
**Location:** `design.md` §5.4, §7 D15; `draft-spec.md` R-40

**Raised by:** round 5 — fresh reviewer, no thread history.

**Expected:** A backend that exits non-zero must report `ExitStatus` and discard stdout, even if stdout is valid JSON.
**Observed:** §5.4's sketch races only the response body against the stderr drain, fixes `result` from that body, and runs `start_kill`/`wait` only in cleanup. Cleanup reports only `CleanupFailure`; it has no path back into `Exchange.result`. A backend can write valid JSON and exit non-zero while the design has already committed to `Ok`.
**Evidence:** The prose step list says "Await exit", and D15/R-40 require non-zero exit to beat parseable stdout. The sketch's `body` is write-stdin-and-read-stdout, `Ok(r) => r`, and the cleanup `wait` status is discarded. Reviewer asked for it to be settled by execution rather than argument.

**Disposition:** `fix-now`
**Response:**

Correct, and worse than stated. Cleanup does not merely fail to report the
status — it **kills the child first**, so on every path the status was destroyed
before it could be read. `BackendError::ExitStatus` was unreachable, R-40's own
fixture could not pass, and the design asserted the behaviour in prose four
paragraphs above the sketch that prevented it.

Introduced by F-53's repair. Collapsing two graces into one cleanup budget was
right; what went with it was `child.wait()`, which had been inside the timed
region and came out with them. The status is not disposal — waiting for a backend
to exit is the backend's opportunity to respond — so it goes back inside
`config.timeout`, and D48's total is unchanged.

Settled by execution, as the reviewer asked. The probe was extended to the F-59
shape and run: a backend writing a valid response then `exit 1` now returns
`ExitStatus { code: Some(1) }` with the body discarded and the stderr kept, in
2.7 ms; a plain success still returns `Ok` in 2.5 ms, so the cleanup budget is
not paid on the normal path. The run also settles a borrow question the repair
raises and the sketch now states: `body` holds `&mut child`, which the cleanup
budget needs back, so `body` lives in an inner scope that ends where the timed
region ends. It compiles.

D51.

**Outcome:** `repaired` (round 5)

### F-60 — Reopening F-49: cancellation repair deletes the task leak but not the child cleanup gap

**Severity:** blocker
**Location:** `slice-001.md` AC-5; `design.md` §5.4, I11; `draft-spec.md` R-48

**Raised by:** round 5.

**Expected:** AC-5 says a cancelled exchange leaves nothing behind. R-48 says an exchange must not leave behind any task or handle that drop-time cleanup would fail to cancel.
**Observed:** The F-49 repair removes `tokio::spawn`, so the drain is no longer detached — but cancellation still drops a live `Child` and relies on `kill_on_drop`, which this design itself calls best-effort and not a reaping guarantee. The repair proves "no detached drain task", not AC-5's stronger "nothing behind".
**Evidence:** §5.4 calls `kill_on_drop` a backstop, needing a live runtime to reap. R-48 says that where the exchange is dropped, drop-time cleanup "is the only mechanism there is and the host relies on it explicitly". Those two cannot establish AC-5.

**Disposition:** `fix-now`, as a narrowing — `doc-wrong`, not a mechanism change
**Response:**

Correct, and it is F-56's class once more: R-48 was narrowed for cancellation at
F-41 and AC-5 was never narrowed with it. The premise AC-5 gives is true — after
F-49 there is no task to detach — and the conclusion it draws from it is not,
because the child was never a task.

Put to the user as a fork, since the alternative is real: make AC-5 true, or
state the narrower claim. Making it true needs either a supervisor outside the
exchange to reap abandoned children — which is precisely the detached task F-49
deleted, returning in a different hat — or a process-group kill, which brief §14
refuses because backends are trusted user programs. Both are worse than the gap.

**Decided by the user:** narrow AC-5, and do not add a follow-up. So AC-5 now
says what the host can hold to — no task, buffer or descriptor survives a
cancelled exchange, structurally — and says plainly that the child falls to
`kill_on_drop`, which is best-effort. §5.4 carries the reasoning and names slice
003 as the slice that will meet this written down rather than discover it, since
a timer is the first thing that can cancel an exchange. I11 gains an explicit
scope line: it is about what the *host* holds, and I13 owns the child.

The test of an honest narrowing is whether an obligation disappeared. It did not:
the structural half got *stronger* at F-49 — "no task, buffer or descriptor" is
now held by there being nothing to abandon rather than by remembering to abort —
and the child half was never held at all. What changed is that the document stops
claiming it. F-41 is the precedent and the same standard applies. D54.

**Outcome:** `repaired` (round 5)

### F-61 — `Alternative` reuses `OptionId`, violating the namespace rule it was meant to enforce

**Severity:** major
**Location:** `design.md` §5.2 canonical types, I15, D45; `draft-spec.md` R-52

**Raised by:** round 5.

**Expected:** Identifier newtypes should prevent values from different response namespaces being passed for one another — and the design says a view's option id and a choice field's alternative id are different cases: one is selected by `UserResponse.option`, the other is submitted inside `UserResponse.values[field_id]`.
**Observed:** `Alternative { id: OptionId, … }` beside `UserResponse.option: OptionId`. A choice-field alternative id can be passed where a top-level option id is required, despite the design's claim that the compiler enforces the separation.
**Evidence:** §5.2 says the scalar newtypes exist so "a view id, an option id and a field id cannot be passed for one another", and then uses `OptionId` for both `Opt.id` and `Alternative.id`. F-58 fixed the rule's scope and left the identity type collapsed.

**Disposition:** `fix-now`
**Response:**

Correct, and it lands squarely on my own round-4 repair. The comment asserting
that the newtypes keep namespaces apart was added in the F-56 batch, three lines
above a struct that violated it. F-58 answered this question from the rule end
and stopped there; the type end is where it bites.

`AlternativeId` added. Two consequences taken rather than argued around, and they
go slightly beyond the literal finding: `DuplicateAlternativeId` and
`EmptyAlternatives` join the taxonomy, because raising `DuplicateOptionId`
against an alternative asserts that the id *is* an option id — which is the F-48
naming mistake exactly, a variant whose name states something the path does not
establish. The alternative was to keep the generic errors and disambiguate by the
`at` path, which works and is cheaper, and is wrong for the same reason
`Orphaned` was wrong.

Worth noting the sequence: F-54 deleted the recursion, F-58 widened the rule,
F-61 split the type. One defect, three findings, each of which looked complete
when it landed. Same shape as F-27/F-40/F-49 — and the tell was available here
too, since F-58's own repair had to write two clauses to describe one type.

D52.

**Outcome:** `repaired` (round 5)

### F-62 — The no-panic verification command does not enable the lints it claims enforce it

**Severity:** major
**Location:** `design.md` I9, §9 verification commands; `draft-spec.md` R-46

**Raised by:** round 5.

**Expected:** R-46 says no backend-derived value may panic the host, and its verification row says clippy denies `unwrap_used`, `expect_used`, `indexing_slicing` and panicking arithmetic in the modules handling backend-derived data.
**Observed:** The declared commands are `cargo clippy --all-targets -- -D warnings` and its no-default-features twin. Neither names those lints, and the manifest configures none. All four are *restriction* lints and allow-by-default: `-D warnings` does not enable them.
**Evidence:** §9 lists six commands, none naming the lints; §5.1's manifest has no `[lints]` section. Reviewer's settling test: add a deliberate `unwrap()` on a backend-derived path and run the six commands; if they pass, R-46 is unverified.

**Disposition:** `fix-now`
**Response:**

Correct, and mechanically checkable without running anything: the four lints are
allow-by-default restriction lints, so `-D warnings` raises the severity of
warnings that fire and never causes these to fire at all. I9 named "clippy lints"
as what holds it and no clippy lint held it.

Enabled where R-46 already said they belong — **per module**, as inner
`#![deny(...)]` attributes on the modules handling backend-derived data, rather
than crate-wide in `[lints.clippy]`. That scoping is R-46's own reasoning and not
a preference: the blanket form is what F-35 caught this design violating, on
`child.stdin.take()`, a value the *host* created, where an `unwrap` is a statement
about our own code. A restriction lint applied where it does not belong gets
`#[allow]`ed at the first inconvenience, and an allow-by-default lint that has
been allowed back is indistinguishable from one that was never enabled.

The general shape, which is the third instance this review has produced:
**a claim is held by a mechanism or it is not held.** F-51 was a canon rule that
no build enforced; F-57 was a build gate that could not run; F-62 is an invariant
whose named enforcement was off by default. Each read as green.

D53.

**Outcome:** `repaired` (round 5)

### F-63 — The grandchild case is described in five places and measured in none of them

**Severity:** major
**Location:** `design.md` §5.4, §5.5 edge table, §8 R9, §9; `draft-spec.md` R-41, R-48/R-54 verification

**Raised by:** the responder, running the F-56 sweep over the round-5 batch.

**Expected:** The document's headline empirical claim — "a backend answers correctly, a grandchild holds the pipes, the response is delivered and only cleanup fails" — describes the case that was actually run.
**Observed:** It does not. The preserved probe's case D is `(sleep 30) &`, and a bare subshell inherits **stdout as well as stderr**. With stdout held open, `read_to_end` never sees EOF, so the body cannot complete: that case times out. Its 902 ms is `config.timeout` **plus** `CLEANUP_LIMIT` — the cost of a *failed* exchange — while five sites describe a delivered response, which would cost the cleanup budget alone.
**Evidence:** §5.4 said "the child itself exits and is reaped, and only the drain stalls"; the edge table said the response is delivered; `draft-spec.md`'s R-48/R-54 row asserted "the response is delivered, the exchange reports no failure"; §9's fixture list and the 902 ms figure said the same. The probe source at `transport-probe.local.rs` case D settles it. This also matters to F-48, which used this case as the evidence for naming the variant `TimedOut` rather than `Orphaned`.

**Disposition:** `fix-now`, after running the case that was never run
**Response:**

Accepted; it is mine, and it is the one class of defect this review had been
treating as settled — an empirical claim, executed, and then *restated slightly
wrong* everywhere else. Four rounds of "execute any claim that can be executed"
did not protect against describing a different case from the one executed.

The missing run was cheap and it was not done: a grandchild holding stderr but
not stdout, which is `(sleep 30) >/dev/null &`. Run now, it returns
`Ok(response)` with `cleanup` set, in 303 ms. So the case the document described
is real — it had simply never been observed, while a different case had been, and
the two were spliced.

Both are now tabulated separately, because they differ in `result` and in cost:
stderr-only delivers the response and pays the cleanup budget; stdout-too fails
the exchange and pays both budgets. The second is the `Err` + `Some` row of
§5.4's two-dimension table, which until now the design called meaningful without
having produced one — so the table gained an observed instance at the same time.

Two things worth keeping. First, F-48's conclusion survives and its evidence
improves: the child exits and is reaped in *both* cases, so `Orphaned` would
still have been false, and the stderr-only run is now the case that actually
supports the sentence about it. Second, the honest limitation this exposes:
**a host cannot distinguish "the backend is still writing" from "the backend
exited and something else holds the pipe"** — both are a pipe with no EOF.
`config.timeout` is the only available answer, and the alternative — stopping at
the end of the first JSON document — would silently accept a truncated response
as complete. That is now stated rather than left as an implication.

**Outcome:** `repaired` (round 5, self-raised)

## Synthesis

Five rounds, 63 findings, 7 blockers. Closed by user decision on 2026-08-26 with
16 repairs (F-48…F-63) `repaired` and **not** independently verified. That is the
one thing a reader should carry away before anything else here: this ledger did
not reach its own definition of done, and was closed deliberately rather than
by satisfying it. The risk is stated at the end.

### What the review changed

Four things, none of them cosmetic.

**The transport was rebuilt three times.** The original design deferred bounded
pipes and stderr-on-timeout as one follow-up (D18, D19); F-2 and F-3 reversed
that in round 1. F-27, F-40 and F-49 then repaired the same `tokio::spawn`
mistake three times before the third one identified the mistake as the spawn
itself, and the fix was a deletion — no task, no `Arc`, no `Mutex`, no `abort`.
F-53 bounded cleanup and F-48 made it a second reporting dimension rather than a
precedence contest. F-59 then found that F-53's repair had carried `child.wait()`
out of the timed region with it, leaving `BackendError::ExitStatus` unreachable
on every path. The §5.4 sketch that stands is the fourth structure, and the only
one compiled and run against all seven backend cases.

**A binding canon constraint was false, not merely unenforced.** F-51: ADR-001
requires stratum 1 to build with no async runtime in its dependency graph, and
Cargo resolves dependencies per crate target, so a single crate with a plain
tokio dependency cannot satisfy it however the modules are arranged. Three rounds
had checked compliance and never the premise. The feature gate (D49) converts
half of ADR-001 from a review gate into a build gate — stronger than the ADR
assumed possible before the workspace split — and CD-1 records which half moved.

**The protocol lost surface it could not answer for.** F-52, F-54, F-58 and F-61
are one defect found four times: an identifier the response format cannot
address, or cannot address unambiguously. A choice field's options could carry
fields recursively that no flat `UserResponse` could express (F-54); duplicate
field ids collided in the response map (F-52); the rule stated to justify the
three checked newtypes covered only two of them (F-58); and the type kept one
`OptionId` across two namespaces (F-61). The design is narrower than when the
review started, in every case by deleting admitted surface no requirement asked
for.

**Three claims turned out to be held by nothing.** F-51 (canon no build
enforced), F-57 (a build gate whose command could not run), F-62 (an invariant
whose named clippy lints were allow-by-default). All three read as green. This is
the review's most transferable finding and it is not about this design:
**a claim is held by a mechanism or it is not held, and "we check it" is not a
mechanism.**

### What it confirmed

The parts nobody could break, named so a later reader does not reopen them: the
stratum split and its module layout; the wire/canonical duality and its
inbound-only scope (D5); P2's failure granularity, which survived every attempt
to find a value that should be fatal but is discardable or the reverse; the
version asymmetry (D7); and the decision that the host validates `view_id` and
nothing else (D17), attacked twice and held both times — once with the
validation-feedback round trip traced end to end to show it needs no breaking
restructure.

### Risks knowingly left standing

1. **Sixteen repairs are unverified.** F-48…F-58 were never re-examined — round 5
   was asked to verify them, found no defect in them, and spent itself on older
   material instead. F-59…F-63 are round 5's own repairs and nothing has looked
   at them. The base rate across this ledger is roughly 0.2 defects per repair
   and falling, which puts the expectation at two to four defects still resident,
   most likely in §5.4, restructured three times.
2. **The cancellation gap is real and stated.** On a dropped exchange no host
   code runs, so the child falls to `kill_on_drop`, which tokio documents as
   best-effort. AC-5 says so now (D54, F-60). Slice 003 introduces the first
   thing that can cancel an exchange.
3. **Direction across the stratum boundary is still a review gate.** AC-15's test
   greps three tokens; a re-export or a downward type leak passes it. ADR-002
   names this as its own risk and the workspace split is the only real answer.
4. **Two canon deltas are unapplied.** CD-1 and CD-2 await endorsement at audit.

### On the method, since the ledger is also a record of how it was reviewed

Rounds 4 and 5 both used a fresh reviewer with no thread history, and both
reached past what previous rounds had examined; the accumulating thread of rounds
1–3 did not. Six findings were raised by the responder against its own repairs,
three of them from one move — reading a repair against the round it came from
rather than the finding it answered. The most productive instrument across all
five rounds was executing claims rather than reasoning about them: seven probe
cases, five of which changed the design. Its blind spot is F-63, where the case
executed and the case described drifted apart while the numbers stayed real.

**State:** closed — 2026-08-26, by user decision, with the outstanding-repair
risk above accepted.

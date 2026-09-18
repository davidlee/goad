# Review — design — Slice 009

**Subject:** design — `docs/slices/009/design.md` (sections 1-10), with
`docs/slices/009/canon-delta.md` (CD-1, CD-2) as its canon debt, read against
`docs/specs/001-host-backend-protocol.md`, `docs/adr/001-one-way-strata.md`,
`docs/policy/001-the-phase-gate.md`, and the code in `crates/`.
**Reviewer:** fresh agent — Codex (gpt-5.6-sol) via MCP
**Opened:** 2026-09-17
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

**Round 1** — 2026-09-17 — the design as first drafted, before any finding.

What this review holds the design to:

- **The protocol is the contract.** The slice claims *no protocol change*. Any
  place the host's drawing decisions or its as-drawn values quietly constrain
  what a backend may send, or what it may read back, is a breach of that claim
  and of `R-55`.
- **The host invents nothing domain-shaped.** `R-35` bounds what the host may
  refuse; `R-58` bounds what it must submit. Five as-drawn values are host
  inventions and each must be defensible against both.
- **`Glass::present` is total** (`glass.rs:20-35`). Every property, every call.
  A design that introduces a conditional writer breaks a contract the file
  states about itself.
- **A tier must be able to observe what a case claims.** `research.md` Thread 3
  measured that `changed` fires nowhere under `init_no_event_loop`; the design's
  own R1 names this as the failure mode. The validation table is where it would
  be committed.
- **Types, not discipline.** Where the design claims an invariant is held *by a
  property of the types*, that claim is checkable against the code and must be.

Surfaces pointed at, deliberately as surfaces rather than as conclusions:
`view_id` as the in-place key; §5.2's as-drawn table against `R-58` and `R-35`;
the two channels against `present`'s totality; §9's tier assignments row by row;
`FieldForm` becoming uninhabited and what else in the tree assumes it is
constructible; and Thread 4's unverified claim, which §5 falsifies on
`wire.rs:128-132` — the falsification is what wants checking, not the claim.

One question is **open by user decision** and is not a finding: whether CD-1
states the `datetime` epoch normatively or descriptively. Deferred to promotion
at audit (§6).

**Round 2** — 2026-09-17 — the repairs. Two jobs, and they are separate: set the
terminal outcome on F-1 … F-18, which is the raiser's and not the responder's;
and attack the repaired text as new material, because eleven of the eighteen
dispositions rewrote a section rather than patching a line. The surfaces the
repairs created — §5.2's string-valued number channel and `slider_bounds`,
§5.3's second totality exception, §5.4's write order and picker seeding, §5.5's
I-F and I-G, §9's driver column, §10's `POL-001` residue argument, and CD-2's
two removals — did not exist when round 1's brief was written and are held to
the same invariants it named.

**Round 3** — 2026-09-18 — the integration. Two jobs again, and they are the
same two. Set the terminal outcome on F-19 … F-30 and F-32 … F-37, which is the
raiser's and not the responder's. And attack the integrated text as new material,
because that is what it is: the seven live findings of round 2 rewrote §5.2's
whole numeric account, all of §5.4's picking, §7 D13 and D21, §9's tier prose,
§5.5's I-G and two edges, and added a type and a function that no reviewer has
seen.

Held to the five invariants round 1 named, and to a sixth this round's own
material forces: **a value must be constructible where it is constructed.** F-37
is that failure — a type whose payload only the retained presentation can supply,
carried by a command a Slint callback builds. `AlternativeId` is not the only
`pub(super)` constructor in `goad-semantics`, and `Finite` is not the only payload
whose value depends on state the builder cannot see.

Surfaces, as surfaces:

- §5.2 from *A number at the markup boundary* to the end of the guard — the parse
  rule, the recording rule, the comparand, the exception, and the rejected
  revision. One argument now, across five paragraphs and two types.
- §5.2's `Reported` / `resolve` block, `Edited`'s new payload, and what
  `Command::Edit` and `PendingEdit` carry. New surface, one pair of eyes.
- §5.4's four paragraphs on picking, and §7 D21 — a widget whose behaviour two
  findings have now read wrongly in opposite directions.
- §9's control table, the pointer paragraph, the loop-tier paragraph and §8 R9 —
  what depends on a popup being laid out, and what a driver has been *named* for
  rather than measured.
- §5.5's I-G, A-2, A-3 and the two numeric edge rows, against §5.2 as it now
  stands.
- §5.1's site enumeration and §9's three widgetless obligations, against what the
  `Reported` split actually touches.

**Round 4** — 2026-09-18 — the integration, and the prototype's repairs. Two
jobs again, and they are the same two rounds 2 and 3 ran. Set the terminal
outcome on the seventeen carrying `_pending round 4_` — F-6, F-20, F-26, F-29,
F-32 and F-38 … F-49 — which is the raiser's and not the responder's. And attack
the text as new material, because two integrations have moved it since round 3
read it: round 3's thirteen findings plus the four the integration itself raised,
and then D-29 … D-32 with the prototype's repairs.

**A different model, and that cuts both ways.** Rounds 1-3 were one reviewer,
`gpt-5.6-sol`, three times. This round is a Claude agent (D-28 — Codex is out of
credits). A reviewer that has already read a document three times is not a fresh
reviewer, and a different model has a different set of blind spots rather than
fewer of them. What the Codex rounds were demonstrably good at: checking a claim
against a named source and finding it false — F-14, F-20, F-26, F-32 and F-35 are
all that shape, and the three known-bad citations in `notes.md` were all written
by responders rather than by those rounds. This round should expect to find
**less** of that class and should not manufacture it. What three rounds of that
shape can leave behind is the class this round holds itself to below.

What this review holds the design to — round 1's five, round 3's sixth, and three
this round's material forces:

- The five from round 1: the protocol is the contract; the host invents nothing
  domain-shaped; `Glass::present` is total; a tier must be able to observe what
  its case claims; and *types, not discipline* where the design says an invariant
  is held by a property of the types.
- Round 3's sixth: **a value must be constructible where it is constructed.**
- **A mechanism is one machine, not a set of paragraphs.** The delivery rule the
  F-38 … F-41 family produced is now stated across §5.1, §5.2, §5.3, §5.4 and
  §5.5 I-H: a keyed map, an overlay, a timer that re-arms, a drain into `Choose`,
  a clear-on-enqueue rule and a view check at three sites. Each paragraph was
  reviewed as a repair to the finding that produced it. None has been read
  against the others as a single machine that has to terminate, lose nothing, and
  duplicate nothing under every interleaving. That is the composition the last
  round created and nobody has attacked.
- **A rule that needs a test is a row in §9 with a named driver; prose binds
  nothing.** F-11 and F-44 are the same failure one round apart, so this stops
  being an observation and becomes something the design is held to. Applied to
  every rule the last two integrations added, I-H first, which is entirely prose.
- **A claim about a dependency is checked against the locked source, not against
  the design's reading of it.** `notes.md` §*Citations known bad* lists three and
  strikes two more; every one was a responder's. The integrations added claims
  about `i-slint-core`'s timer re-arm, `string_to_float`'s two paths and
  `items/text.rs`'s two-byte escape that were read rather than re-derived here.

Surfaces, as surfaces:

- §5.2's `interpret` — its three-case `None` against the thirty pairs the
  signature admits, and against *the text is recorded verbatim, always*.
- §5.2's number formatting: `Display`, `{:e}` past 24 characters, the grammar
  constraint D-32 puts on the plan, and whether format and parse are actually
  inverses over the texts the control admits.
- §5.2's two sets of admitted-but-unparseable texts, against
  `i-slint-core-1.17.1`'s own source.
- §9 row by row, every tier against its driver, and AC-4 in particular, which
  moved wholly to `tests/renderer/` on a measurement about `init`.
- §10's narrowing — that the `jiff` feature gates `compose` and `today_local` and
  nothing else — against `POL-001`'s residue.
- `DrawnKind::Choice` carrying the first id, and what else in §5.1's `FieldForm`
  enumeration that changes.
- I-H's three sites, and whether the overlay's property — *what the screen shows
  is what an answer would submit* — survives every row of §5.5's edges table.
- The delivery machine end to end: map, timer, overlay, drain, enqueue, view
  check. Interleavings, termination, and what §9 commissions of it.
- What §9 does **not** commission. The obligations table has sixteen rows and the
  design now names more mechanisms than that.

**Provenance, stated at raise time.** `notes.md` §*Waiting on the user* records
one open user decision and this round was told to leave it alone. That section
was **not read** before the findings below were raised; the Brief and every F-50
onward was written against `design.md`, the ledger and the code. Any finding here
that turns out to restate that decision is an independent second witness, and
this paragraph is what makes that checkable rather than asserted.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | fix-now | verified |
| F-2 | blocker | fix-now | verified |
| F-3 | major | fix-now | verified |
| F-4 | major | fix-now | verified |
| F-5 | major | fix-now | verified |
| F-6 | major | fix-now (re-disposed ×2) | verified |
| F-7 | major | doc-wrong | verified |
| F-8 | major | doc-wrong | verified |
| F-9 | major | fix-now | verified |
| F-10 | major | fix-now (re-disposed) | verified |
| F-11 | minor | fix-now (re-disposed) | verified |
| F-12 | minor | doc-wrong | verified |
| F-13 | major | fix-now | withdrawn |
| F-14 | blocker | fix-now (re-disposed) | verified |
| F-15 | major | fix-now (re-disposed) | verified |
| F-16 | major | fix-now | verified |
| F-17 | minor | fix-now | verified |
| F-18 | minor | fix-now | verified |
| F-19 | major | fix-now | verified |
| F-20 | major | fix-now (re-disposed) | verified |
| F-21 | major | fix-now | verified |
| F-22 | major | fix-now | verified |
| F-23 | minor | fix-now | verified |
| F-24 | major | fix-now | verified |
| F-25 | major | fix-now | verified |
| F-26 | major | fix-now (re-disposed) | verified |
| F-27 | minor | fix-now | verified |
| F-28 | major | fix-now | verified |
| F-29 | nit | fix-now (re-disposed) | verified |
| F-30 | blocker | fix-now | verified |
| F-31 | blocker | — | withdrawn |
| F-32 | nit | fix-now (re-disposed) | verified |
| F-33 | minor | fix-now | verified |
| F-34 | minor | fix-now | verified |
| F-35 | blocker | fix-now | verified |
| F-36 | major | fix-now | verified |
| F-37 | blocker | fix-now | verified |
| F-38 | blocker | fix-now | verified |
| F-39 | blocker | fix-now | verified |
| F-40 | blocker | fix-now | verified |
| F-41 | blocker | fix-now | verified |
| F-42 | major | fix-now | verified |
| F-43 | minor | fix-now | verified |
| F-44 | major | fix-now | verified |
| F-45 | minor | fix-now | verified |
| F-46 | minor | fix-now | verified |
| F-47 | major | fix-now | verified |
| F-48 | nit | fix-now | verified |
| F-49 | minor | fix-now | verified |
| F-50 | major | _pending_ | _pending_ |
| F-51 | minor | _pending_ | _pending_ |
| F-52 | major | _pending_ | _pending_ |
| F-53 | major | _pending_ | _pending_ |
| F-54 | major | _pending_ | _pending_ |
| F-55 | major | _pending_ | _pending_ |
| F-56 | minor | _pending_ | _pending_ |

### F-1 — The chosen system-time-zone implementation is compiled without system-time-zone support

**Severity:** blocker
**Location:** `design.md §5.2, instant.rs`

**Expected:** D-7 requires a completed pick to use the system-local zone and submit the offset in which the person picked; if that requires a feature on the workspace-wide `jiff` dependency, POL-001 requires the slice to state and argue that design decision.
**Observed:** The design calls `TimeZone::system()` but the workspace disables all `jiff` default features and enables no `tz-system` feature. In this build, `try_system()` is documented to fail because the feature is absent, so `system()` always takes its UTC-like fallback. Adding `tz-system` at the shared dependency is itself an unmade design decision because it unifies into stratum 1.
**Evidence:** `Cargo.toml:31-36`; `design-log.md:75-86` (D-7); `design.md:249-261`; `docs/policy/001-the-phase-gate.md:139-143`; locked `jiff-0.2.35/src/tz/timezone.rs:325-342`; `cargo tree -p goad -e features` shows `jiff` with no `tz-system` feature.

**Disposition:** fix-now
**Response:** Confirmed at the manifest: `jiff` is `{ version = "0.2", default-features = false }` in `[workspace.dependencies]`, every member takes `{ workspace = true }`, and `cargo tree -p goad -f "{p} {f}"` prints `jiff v0.2.35` with an **empty** feature list. `try_system` compiles to the `#[cfg(not(feature = "tz-system"))]` arm, which is an unconditional `Err`, and `system()` swallows it into `Etc/Unknown`; the `warn!` on that path is suppressed too, `logging` being off. D-7 would have submitted `+00:00` for every pick everywhere.

`tz-system = ["std", "dep:windows-link"]` and `tzdb-zoneinfo = ["std"]`, so the repair switches `std` and `alloc` on in a dependency stratum 1 shares. `POL-001` §Verification names that as **the residue** and requires it to be argued in the slice that takes it. Taken (D-11): the feature goes on the entry `crates/goad` inherits, `design.md` §10 carries the argument, and `slice-009.md` §Scope gains `crates/goad/Cargo.toml`. §3 gains the manifest fact so the next reader does not rediscover it.

**Outcome:** verified — round 2. On v0's primary Linux platform, `tz-system` plus `tzdb-zoneinfo` supplies both detection and resolution, and the design now owns the shared-feature decision.

### F-2 — The Slint number channel narrows legal protocol numbers from `f64` to `f32`

**Severity:** blocker
**Location:** `design.md §5.2`

**Expected:** Every finite `f64` admitted by `NumberRange` remains drawable without narrowing wire compatibility, and an adjusted number submits a JSON number as R-57 requires; D-6 additionally requires the untouched wire value not to contradict the displayed value.
**Observed:** `minimum`, `maximum`, `FieldValue.number`, and `FieldEdit.number` are all Slint `float`, which is Rust `f32`, while the canonical range and `Edited::Adjusted` are `f64`. A legal bound such as `1e100` becomes infinity at this boundary; precision is lost for less extreme values, and a non-finite value converted through `serde_json::Value::from(f64)` becomes JSON `null`, not a number. The design specifies no representability check or lossless boundary.
**Evidence:** `design.md:153-165,229-235`; `crates/goad-semantics/src/protocol/canonical.rs:405-459`; `SPEC-001/R-17`, `R-55`, `R-57`; locked Slint 1.17.1 `type-mappings.md:3-14`; locked `serde_json-1.0.151/src/value/from.rs:47-60`.

**Disposition:** fix-now
**Response:** Verified: Slint's `float` is `f32` and `int` is `i32` (`slint-1.17.1/type-mappings.md:13-16`); `NumberRange` holds `f64`. The first draft ran bounds, display **and** the submitted value through that channel.

Repaired at the class, not the site (D-12). The submitted number crosses as a **string** and is parsed host-side to `f64`; the `float` slot survives only as the guard's comparand and the `Slider`'s own value, neither of which reaches the wire. The control choice stops being inferred by the markup from a fact about the field: `FieldRow` carries `slider: bool`, decided by one named function in `view_model.rs` which today requires both bounds present and both round-tripping `f64`→`f32`→`f64` exactly. Any legal `f64` stays drawable and submittable losslessly; only which control draws it changes. F-14's second half is the same defect on the picker boundary and is repaired with it.

**Outcome:** verified — round 2. The submitted channel is now a string parsed directly to `f64`; the new guard and slider-admissibility defects are separate findings below.

### F-3 — The datetime value model cannot display the settled untouched state

**Severity:** major
**Location:** `design.md §5.2`

**Expected:** Settled D-6 says an untouched datetime submits the epoch while its button reads *not set*; a picked datetime reads as its composed value.
**Observed:** `FieldValue` has no touched/set discriminator, both value-building callers replace absence with `as_drawn`, and `as_drawn(datetime)` is the epoch rendered through the same `display_with_offset` path as a picked value. The proposed interface therefore cannot distinguish untouched from a person deliberately picking the epoch, and §5.2 alternately says the button shows *not set* and that the epoch renders as `1970-01-01T00:00:00+00:00`.
**Evidence:** `design-log.md:55-70` (D-6); `design.md:159-165,181,209-218,238-246`; `design.md:84-86` (P-3).

**Disposition:** fix-now
**Response:** Correct, and the two halves of §5.2 did contradict each other. The repair is that `state_of`'s `Option` is not collapsed at both call sites: `glass.rs` reads it **directly** for display — `None` renders *not set* — and `as_drawn` is applied only where a value must reach the wire, in `controller::answer`. That is what makes D-6's sentinel a wire fact rather than a screen fact, which is what D-6 chose it to be. §5.2, §5.3 and P-3 restated accordingly.

**Outcome:** verified — round 2. Display now preserves `state_of == None` while only `controller::answer` applies the epoch-valued `as_drawn`.

### F-4 — Datetime composition silently changes or guesses civil times at DST transitions

**Severity:** major
**Location:** `design.md §5.2, instant.rs`

**Expected:** D-7's “local time zone from system, abandon, don't lie” decision requires the submitted instant and offset to correspond to the civil date and time the person picked, or requires an explicit policy for a nonexistent or duplicated local time.
**Observed:** `DateTime::to_zoned` uses Jiff's `Disambiguation::Compatible`: a fold silently chooses the earlier occurrence and a gap silently chooses the later time. For example, `2024-03-10 02:30` in New York becomes `03:30 -04:00`. This succeeds, so the design's `None`/visible-no-op treatment does not cover it, and no user decision settles which occurrence of a fold to submit or whether a gap should be refused.
**Evidence:** `design-log.md:75-92` (D-7); `design.md:249-261,375`; locked `jiff-0.2.35/src/civil/datetime.rs:1327-1336`; locked `jiff-0.2.35/src/tz/ambiguous.rs:33-49`.

**Disposition:** fix-now
**Response:** Verified against `jiff-0.2.35/src/civil/datetime.rs` and `src/tz/ambiguous.rs`: `to_zoned` uses `Disambiguation::Compatible` and **succeeds** on both a fold and a gap, so §5.2's `None` treatment never sees them. D-13: keep `Compatible`, state it as a named edge in §5.5, and rely on the button showing the composed value — a shifted time the person can see is not the lie D-7 refused. Refusing an ambiguous pick was rejected: it leaves a person inside a fold unable to express 01:30 and has no affordance to explain itself.

**Outcome:** verified — round 2. D-13 explicitly accepts Jiff's `Compatible` fold/gap result and the button displays that composed result.

### F-5 — The design gives incompatible debounce contracts for numeric `LineEdit`s

**Severity:** major
**Location:** `design.md §5.2`

**Expected:** A planner can determine from the settled decision whether every number widget sends immediately, uses `released`, or participates in the pending debounce and answer flush.
**Observed:** The widget table says a one-bound or unbounded numeric `LineEdit` raises `edited, debounced`, but D-8 says number needs no debounce and “the debounce is text-only”; the system model likewise calls the pending state a text debounce. Sending each numeric keystroke risks the capacity-1 loss D-8 retains the text debounce to avoid, while debouncing it contradicts the settled scope and changes what `chosen` must flush.
**Evidence:** `design.md:132-134,172-185`; `design-log.md:99-120` (D-8); `slice-009.md:62-65`.

**Disposition:** fix-now
**Response:** Correct, and it is one symptom of the class F-9 names. §5.2's table and D-8's reported fact disagreed because the fact — *"`number` needs no debounce"* — rested on `released` being sufficient, which F-9 falsifies. Repaired together (D-14): one pending mechanism, kind-agnostic, per field. `text` and the numeric `LineEdit` debounce on `edited`; the `Slider` debounces on `changed` and flushes on `released`. D-8's *decision* — the flush point is the answer and nowhere else — is untouched.

**Outcome:** verified — round 2. Both numeric controls now use the same kind-agnostic pending/debounce contract as text.

### F-6 — One pending command cannot preserve edits to multiple text fields

**Severity:** major
**Location:** `design.md §5.1, pending.rs`

**Expected:** AC-4 and D-8's flush-by-construction claim hold for everything typed before an option is answered, including a form with more than one text field.
**Observed:** The proposed owner has one pending `Command::Edit` and one timer, and focus loss or field switching explicitly does not flush. If a person edits field A and then field B inside 150 ms, that state shape cannot retain both commands: replacing A loses it, while sending A on the switch violates D-8's “on answer, and nowhere else.” The answer path can flush only “whatever `pending.rs` is holding.”
**Evidence:** `design.md:132-134,265-272,287-300,318-323`; `design-log.md:99-107` (D-8); `slice-009.md:79-81` (AC-4); `SPEC-001/R-58`.

**Disposition:** fix-now
**Response:** Correct and sharp. A single pending `Command::Edit` loses field A's last keystrokes the moment a person moves to field B inside the window, and D-8 forbids flushing on the switch. Repaired at the state shape: `pending.rs` holds a map keyed by (option, field) with one timer, every entry flushed on answer in declared order. This also makes the mechanism kind-agnostic, which F-5 and F-9 need anyway — one repair, three findings.

**Outcome:** contested — round 2. The map retains both edits, but a synchronous callback cannot enqueue both edits and `Choose` through the production capacity-one channel; the claimed one-click FIFO flush is still false.

**Re-disposition (round 2):** fix-now
**Response:** Contest upheld, and it understates the problem. Verified: the command channel is `mpsc::channel::<Command>(1)` (`main.rs:86`) and `serve` shares the UI thread through `slint::spawn_local`. A Slint callback is synchronous and has no await, so `serve` cannot drain between two sends made inside one — the **second `try_send` of any flush always fails**, not occasionally. §5.4 called that the case where "the second `try_send` comes back `Full`", as though it were an edge. It is the only case, and it was already true before round 1: the pre-repair design flushed one edit and then `Choose`, which is two sends.

Repaired at the mechanism (D-15): **`Command::Choose` carries the pending edits.** One send, no race, FIFO no longer load-bearing; the controller applies them to the draft and then answers. D-8's decision — the debounce flushes when the draft becomes an answer — is then held by construction rather than by sequencing, which is what it always meant. §5.1, §5.3 and §5.4's *An answer* are rewritten; the edges table loses the "channel full when flushing on answer" row and gains the single-send case.

**Outcome:** contested — round 3. The one-command answer is present (`Choose` carries `Vec<PendingEdit>`), but the map's *timer* path still sends one `Command::Edit`, and handling it presents while the map's other entry is absent from the draft, so that widget is reasserted to its old value. Returns to open; F-41 states the mechanism in full.

**Re-disposition (round 3):** fix-now
**Response:** Contest upheld. "One `Command::Edit` goes down the channel, as
today" was true of a single pending slot and is not true of a map, and the
repair stopped at the answer path. The answer is not a second command shape —
it is the pair F-40 and F-41 now carry: the value channel shows what
`pending.rs` holds, so an entry the timer has not sent yet is not reverted by
the present that follows the one it did send; and the timer re-arms while the
map is non-empty, so every entry reaches the draft within a tick per entry.

This finding's own requirement — every typed field survives to the answer — was
already held by the `Choose` drain and is not what was wrong.

**Outcome:** verified — round 4. The map is keyed by (option, field) (§5.1), the `Choose` drain carries every entry in one send (§5.2 `wire.rs`), the timer re-arms while the map is not empty, and the overlay is what keeps the entry the timer has not yet sent from being reverted (§5.3, §5.5 I-H). Round 3's contest was that the timer path reverted the second field; that path is now covered by a mechanism rather than by an ordering.

### F-7 — Same `view_id` implying same structure is not a property of the types

**Severity:** major
**Location:** `design.md §5.5, I-A`

**Expected:** Because unchanged `view_id` is the sole invalidation key for retained row structure, the claimed implication must be enforced by the values that can reach `Glass::present`, as §5.5 says it is.
**Observed:** `State::issue` uses `saturating_add`, so the claimed strictly increasing sequence repeats after saturation. Independently, `Prepared` exposes `view_id` and `presentation` as separate public fields and `Frame.shown` publicly accepts a `Prepared`; code can clone one valid id into two preparations with different presentations. The production path normally constructs them together, but the types do not hold I-A, and `SlintGlass` would retain stale structure on either admitted collision.
**Evidence:** `design.md:325-329,353-356`; `design-log.md:147-156` (D-9); `crates/goad-shell/src/state.rs:64-81`; `crates/goad/src/reception.rs:33-40`; `crates/goad/src/controller.rs:94-107`.

**Disposition:** doc-wrong
**Response:** Half withdrawn, half upheld, and the disposition is for the half that stands. `next_seq` is `u64` (`goad-shell/src/state.rs:25`), so saturation is not reachable in any process that terminates. The second ground is the real one and is correct: `State::issue` is in `goad-shell`, `SlintGlass` is in `crates/goad`, `Prepared`'s fields are public, and `tests/renderer/` already builds `Prepared` values by hand. Nothing in the glass's own types holds the implication.

So §5.5 stops calling I-A a property of the types. It becomes an assumption naming the production mint that holds it, plus a **precondition on `present`'s caller** stated where a caller will read it. The design decision — `view_id` as the key — is unchanged; the claim about why it is safe is what was wrong.

**Outcome:** verified — round 2. The unreachable `u64` saturation ground is withdrawn; the surviving public-type ground is discharged by stating I-A as a caller precondition rather than a type property.

### F-8 — Skipping structure writes contradicts `Glass::present`'s totality contract

**Severity:** major
**Location:** `design.md §5.3`

**Expected:** `Glass::present` writes every property carried by a frame on every call; the trait states that this total rewrite is the recovery mechanism for a partial display update.
**Observed:** The design retains the previous id and deliberately does not rewrite the structural model for a same-id frame, yet says `Glass::present` “stays total” and does not amend the trait contract or replace its partial-update argument. Presentation structure is still carried by `Frame.shown`, so this is a semantic contract change, not merely an implementation detail of the value channel.
**Evidence:** `crates/goad/src/glass.rs:20-35`; `crates/goad/src/controller.rs:94-107`; `design.md:153-160,263-283`; `design-log.md:158-166` (D-9).

**Disposition:** doc-wrong
**Response:** Correct. `glass.rs:20-35` states totality as the trait's own contract and names its one deliberate exception, with the reason. Skipping the structural write for a same-id frame is a second exception, and D-9's *"`Glass::present` stays total"* asserted past it in one clause.

§5.3 now states the contract change explicitly and gives the exception its argument: the structural model is **retained state whose only writer is `present`**, written on exactly the frames that can change it, and a partial display update cannot leave it stale because the frame that would correct it is the frame that rebuilds it. The trait doc is code and is amended in the phase; the design says so rather than leaving it to be noticed.

**Outcome:** verified — round 2. The design now amends the trait contract, limits the exception to retained structure, and updates the retained id only after the infallible model writes.

### F-9 — Binding a slider only to `released` drops accessibility edits

**Severity:** major
**Location:** `design.md §5.2, The five widgets`

**Expected:** Every supported way of changing the bounded-number widget records an `Adjusted` edit, including the accessibility path the renderer's integration tests use to simulate a person.
**Observed:** Slint 1.17.1's pointer and keyboard paths emit `released`, but its accessibility set-value/increment/decrement actions call `set-value`, which emits only `changed`. The design listens to `released` alone, so an assistive-technology change updates the widget but never the draft and is reverted by the next present.
**Evidence:** `design.md:174-180`; `design-log.md:116-120,421-423` (D-7); locked Slint 1.17.1 `widgets/common/slider-base.slint:23-24,36-47,100-108,117-131` and `widgets/fluent/slider.slint:23-36`; `crates/goad/tests/renderer/fields.rs:17-21`.

**Disposition:** fix-now
**Response:** Verified in the locked sources: the pointer `up` path and `key-released` raise `released` (`common/slider-base.slint:42-46`, `:97-107`), but `accessible-action-set-value`, `-increment` and `-decrement` all route through `set-value`, which raises `changed` and nothing else (`fluent/slider.slint:30-36`, `slider-base.slint:114-131`). A `released`-only binding is therefore deaf to assistive technology, and — F-13 — to the test tier. The `Slider` binds `changed`, debounced, and flushes on `released`. See F-5 and F-6: one mechanism, three findings.

**Outcome:** verified — round 2. `Slider.changed` now records accessibility changes, while `released` remains only the eager flush.

### F-10 — The `FieldForm` transition leaves live consumers and canon claims unaccounted for

**Severity:** major
**Location:** `design.md §5.1, FieldForm`

**Expected:** Making `FieldForm` uninhabited accounts for every constructor and every claim predicated on an undrawn field, so the plan can rewrite the suite and the audit can promote canon that is true after all five kinds are drawn.
**Observed:** CD-2 mentions additions to the R-16/R-58 rows but omits that R-58 currently names a test whose required undrawn-text premise becomes false, and it omits R-55's now-false statement that option fields remain undrawn. The tree also has mapper tests that construct every disappearing variant and a generic diagnostic that will still say “this renderer draws boolean fields only” if a sixth kind later gives the enum a variant back; §9 names only the exhaustive mapper unit. This is a class of stale consumers, not a single test rename.
**Evidence:** `canon-delta.md:42-55`; `SPEC-001 §Verification:413-414,426`; `crates/goad/tests/renderer/mapper.rs:195-203,290-327,375-399`; `crates/goad/tests/renderer/wiring.rs:1326-1349`; `crates/goad/tests/renderer/fields.rs:67-71,612-629`; `crates/goad/src/diagnostics.rs:202-215`; `design.md:114-122,438,460`.

**Disposition:** fix-now
**Response:** Correct, and the enumeration matters more than any one site. §5.1 now lists every consumer that assumes the variant is constructible — `diagnostics.rs:202-215`'s line, whose text *"this renderer draws boolean fields only"* becomes both unreachable and false; `mapper.rs:195-203,290-327,375-399`; `wiring.rs:1326-1349`; `fields.rs:67-71,597-632` — and §9 carries the obligation to rewrite each.

CD-2 is rewritten rather than extended. It previously said the R-16/R-58 rows *gain* cases; it now also says that `SPEC-001` §Verification's R-58 row names a case whose premise — a view carrying an undrawn **field** — becomes unconstructible once all five kinds draw, and that the R-55 row's statement about option fields being undrawn is false after this slice. A `group` hint field is still drawn, and the surviving `Undrawn` variants are body-level, so there is no substitute construction.

**Outcome:** contested — round 2. The class is still incomplete: `reception.rs:753` is absent from the consumer inventory, and CD-2 removes the fields case but not the undrawn-field premise from the wiring case named in the same R-58 row.

**Re-disposition (round 2):** fix-now
**Response:** Contest upheld on its class, **not** on its citation: `reception.rs:753` does not exist — that file is 103 lines. Re-grepping for the class found two sites the round-1 inventory did miss, both doc comments that name `Undrawn::FieldForm` as the site a sixth kind reaches and become misleading once the variant cannot be constructed: `crates/goad/src/draft.rs:82` and `crates/goad/src/view_model.rs:31`. §5.1's table gains both.

The second half of the contest is right and is the more serious one, and it matches what the responder found independently: `SPEC-001` §Verification's R-58 row names **two** cases and this slice kills both. The MUST NOT case, `wiring.rs::an_answer_carries_no_value_for_another_option_or_for_an_undrawn_field` (`wiring.rs:1339`), runs over `TWO_FORMS` (`wiring.rs:1157`) whose `noted` field is `kind: "text"`, and opens with a guard assertion — *"the fixture must actually carry an undrawn field for its absence below to mean anything"* — which becomes **unsatisfiable by construction**. R-58's MUST NOT prohibits two things; after this slice the first, not submitting a value for a field it did not draw, is **unobservable**, and no substitute construction exists. CD-2 must record a permanent reduction in what the suite can assert about a normative rule, not a case that moves. Two further consumers of the same fixture, `wiring.rs:1246` and `:1305`, reach `Refused::UnknownField` through `noted` being undrawn; after this slice that refusal is reachable only from a fabricated field id, which is the posture §5.2 already gives an out-of-range `ComboBox` index.

**Outcome:** verified — round 3. The enumeration is at `design.md` §5.1 and names every site the tree search finds; CD-2 identifies both `R-58` cases and the stale `R-55` statement.

### F-11 — The validation ledger omits two event-loop obligations the design claims it covers

**Severity:** minor
**Location:** `design.md §9`

**Expected:** “What the plan must produce” includes the debounce timer firing and the choice-specific reassertion that A-5 says the loop test covers, because neither is observable in the no-loop renderer target.
**Observed:** The §9 loop rows require only the generic AC-5/AC-6 reassertion and AC-4's element preservation. They do not require waiting for the 150 ms timer or exercising a `ComboBox`; D-10 separately says the new binary holds the timer, while its concrete dropped-edit construction uses a checkbox. A conforming plan could therefore omit both claimed checks.
**Evidence:** `design.md:368-377,443-463`; `design-log.md:179-200` (D-10); `research.md:207-224`.

**Disposition:** fix-now
**Response:** Correct. §9 is the artefact that binds the plan, and a claim made only in `design-log.md` binds nothing. Both obligations are now rows: the debounce timer firing, and a `choice` re-asserting — the latter because A-5 rests on it and A-5 is one of the two unmeasured assumptions in §5.5. Folded into F-13's repair, which re-cuts the table so no row names a tier without naming the driver that operates the widget.

**Outcome:** contested — round 2. The timer firing and choice-specific reassertion are still prose below the table, not rows with drivers and assertions as the response claims.

**Re-disposition (round 2):** fix-now
**Response:** Contest upheld. The repair put both obligations in a numbered list below §9's table, and a claim outside the table binds a planner no more than a claim in a log does — which was the finding's whole point. Both become rows, each with its driver and its assertion: the debounce timer firing, and a `choice` re-asserting. Folded into F-25's rebuild of the table.

**Outcome:** verified — round 3. Both rows now name an executable driver — the timer case lets the loop run past 150 ms and observes exactly one edit; the `choice` case expands, clicks a `ListItem` and presents again.

### F-12 — `wire.rs` does not falsify Thread 4's observability claim on the cited production trace

**Severity:** minor
**Location:** `design-log.md D-9`

**Expected:** The cited mechanism demonstrates a reachable field-edit trace in which A is dropped, B succeeds and lowers the notice, and a present first observes the clean notice while A remains divergent.
**Observed:** `Wire::send` proves only that the signal records the last send. In the ordinary field-edit trace, a `Full` means another command is queued; after `recv`, `Command::Edit` is handled synchronously and `serve` continues to the top-level present before another UI callback can run. For an exchange command, `serve` presents `busy = true` and disables the widgets before awaiting. Thus the cited lines alone do not establish the A/B trace, and the production ordering shown by the controller is the opposite for the immediate-edit case.
**Evidence:** `design-log.md:134-145` (D-9); `crates/goad/src/wire.rs:108-132`; `crates/goad/src/controller.rs:661-675,738-744,789-822`; `crates/goad/src/main.rs:85-111`.

---

The remaining findings were raised by the responder acting as a **second
raiser**, after the round-1 reviewer's ledger was read and its surfaces
verified. The role switch is deliberate and declared, per this ledger's
Protocol: these are raised, not disposed, here.

**Disposition:** doc-wrong
**Response:** Upheld on the argument, not on the conclusion — and the distinction is the whole disposition. `serve` handles `Command::Edit` synchronously and `continue`s to the loop top, which presents before any UI callback can run again (`controller.rs:667-676`, `:739`); an exchange presents `busy = true` and disables every control before it awaits (`:818-819`). So the A-drops-then-B-lands-then-a-clean-notice trace is not reachable, and `wire.rs:128-132` does not establish it.

D-9's **second** kill is untouched and is sufficient on its own: even granting the host knows *that* an edit was refused, it does not know *which* widget diverged, so its only available response is a full rebuild — which destroys the caret this slice exists to protect. §5 and §7's D8 row now rest on that argument alone. `design-log.md` is append-only, so D-9 is not rewritten; D-14 records the correction and cites this finding.

**Outcome:** verified — round 2. The repaired text now gives the correct `serve` ordering: an edit is presented before another UI callback, and an exchange first presents the controls busy.

### F-13 — Three of the five kinds have no driver in the tier §9 assigns their case to

**Severity:** major
**Location:** `design.md §9`, rows AC-2, AC-8, AC-9

**Expected:** A case in `crates/goad/tests/renderer/` can make the widget it
tests raise the edit the design binds, using a driver the target already has.
**Observed:** The target's established drivers are
`invoke_accessible_default_action` and `set_accessible_value`. Neither reaches
three of the five kinds:

- `ComboBox` publishes `accessible-value <=> root.current-value`, which is
  read-only, and exactly one action, `accessible-action-expand`. Nothing
  changes the selection, so AC-8's case cannot choose an alternative.
- the `datetime` `Button`'s default action opens a `PopupWindow`, and no case
  in the target reaches inside one — `research.md` Thread 3 measured that a
  popup cannot even be repeated or made conditional.
- the bounded `number` is F-9's defect seen from the test side: the accessible
  set-value path raises `changed`, and §5.2 binds `released`.

`ElementHandle` does also offer `mock_single_click`, `mock_drag` and
`invoke_accessible_increment_action`, but no case in this repository uses any of
them and §9 names none. The class is one claim repeated three times: §9 assigns
a tier without asking whether that tier can operate the widget.
**Evidence:** `design.md:453-463`; `crates/goad/tests/renderer/fields.rs:223-230`;
`crates/goad/tests/renderer/harness.rs:99-168`; locked Slint 1.17.1
`widgets/fluent/combobox.slint:28-33`, `widgets/fluent/slider.slint:30-36`,
`widgets/common/slider-base.slint:114-121`;
`i-slint-backend-testing-1.17.1/search_api.rs:606-637,893-915,954-1068`;
`research.md:226-236`.

**Disposition:** fix-now
**Response:** Raised by the responder as second raiser; disposed as responder. §9 gains a **driver** column, and every row names the mechanism that operates the widget rather than only the tier that houses the case. Where the no-loop tier has no driver — a `ComboBox` selection, anything inside a picker popup — the case moves to the loop tier and the design says why.

D-10's constraint is unchanged and is the reason this is not free: one event-loop *arrangement*, one `[[test]]` target (`docs/memory/slint-testing-backend-initialises-once-per-process.md`). What D-10 was missing is the check, not the constraint; §9 now performs it per row, and the number of targets is the plan's to settle inside the rule.

**Outcome:** withdrawn — round 2. The finding mistook absence of an existing repository case for absence of a driver: the testing API queries active popups under `init_no_event_loop` and provides mock clicks there.

### F-14 — `instant.rs`'s composition panics where the design says it returns `None`

**Severity:** blocker
**Location:** `design.md §5.2, instant.rs`; `§5.5 A-4`

**Expected:** `compose` is total over anything a picker can hand it, and A-4's
stated cost of being wrong — "a visible no-op, not a wrong value" — is the
actual cost.
**Observed:** The design names `to_zoned`'s `Result` as the only fallibility on
the path. Both calls before it panic instead: `jiff::civil::date` panics
whenever `Date::new` would error, and `Date::at` panics on an out-of-range
hour, minute or second. So the failure mode A-4 prices as a no-op is a host
panic. Independently, the expression does not compile as written: Slint's
`Date` and `Time` carry `int` fields, which is `i32`, while `civil::date` takes
`(i16, i8, i8)` and `at` takes `(i8, i8, i8, i32)` — and a narrowing cast
truncates rather than refuses, which is F-2's defect on a second boundary.
**Evidence:** `design.md:249-261,375`; locked `jiff-0.2.35/src/civil/mod.rs:218-246`,
`src/civil/date.rs:1178-1224`; locked Slint 1.17.1
`widgets/common/datepicker_base.slint:7-11` and `type-mappings.md:13-16`.

**Disposition:** fix-now
**Response:** Raised by the responder as second raiser; disposed as responder. Verified in the locked sources: `civil::date` panics whenever `Date::new` would error (`jiff-0.2.35/src/civil/mod.rs:218-246`) and `Date::at` panics on an out-of-range hour, minute or second (`src/civil/date.rs:1178-1224`). §5.2 attributed the only fallibility to `to_zoned`, and A-4 priced being wrong as a visible no-op.

`compose` is restated over the **checked** constructors — `Date::new` and `Time::new`, both returning `Result` — so every arm of the failure is the `None` the design already handles, and A-4 is rewritten to say what being wrong now costs. The `i32`→`i16`/`i8` narrowing is F-2's defect on a second boundary and is repaired the same way: the conversion is checked, not cast.

**Outcome:** contested — round 2. Checked integer conversions and constructors remove the panic/cast defect, but `DateTime::to_zoned` can still fail near civil-time boundaries and is wrongly excluded from `compose`'s `None` surface.

**Re-disposition (round 2):** fix-now
**Response:** Contest upheld. The repair removed the panic and the cast but then wrote that the checked constructors "plus the two conversions is the whole of the `None`", which excludes `to_zoned` — and `to_zoned` returns a `Result` that can fail at the civil-time boundaries a `Date::new` accepts. Excluding it is the same error the finding was raised about, one call further along. `compose`'s `None` surface is stated as all four: the two integer conversions, the two civil constructors, **and** `to_zoned`.

**Outcome:** verified — round 3. `compose`'s `None` surface names the two integer conversion sets, the checked `Date::new` / `Time::new`, and `to_zoned`; A-4 prices exactly those four.

### F-15 — The order in which a present writes rows, values and the epoch is never stated

**Severity:** major
**Location:** `design.md §5.1, §5.3, §5.5`

**Expected:** The guard reads `root.values[field.slot]` when the epoch changes,
so the design states — as an invariant, where I-B already states the sibling
one — that `values` is written before the epoch is bumped, and that a new
view's row `set_vec` lands before any slot is read.
**Observed:** §5.1's diagram lists the three writes and §5.3 lists their owners,
but no section states an order and §5.5 carries no invariant for it. A present
that bumped the epoch first would run every guard against the previous present's
values; a new view that wrote `values` after the rows would index a shorter
array, which Slint answers with a default-initialized struct rather than an
error. `split.rs` exercised one order and the design does not record which.
**Evidence:** `design.md:108-112,190-196,263-283,349-366`;
`research.md:182-205`; `spike-fields/tests/split.rs`.

**Disposition:** fix-now
**Response:** Raised by the responder as second raiser; disposed as responder. The guard reads `root.values[field.slot]` when the epoch changes, so its correctness is an ordering property and the design stated none. §5.5 gains it as an invariant beside I-B, and §5.4 states it in the sequence where a reader meets it: rows first where they are written at all, then `values`, then the epoch — so a guard never runs against the previous present's values, and a slot is never read against a shorter array.

**Outcome:** contested — round 2. The repair prescribes rows then values, precisely the ordering that lets a newly instantiated row index the previous, shorter values array; values must be available before a new row can read its slot.

**Re-disposition (round 2):** fix-now
**Response:** Contest upheld, and the repair had the order backwards. A `set_vec` instantiates rows, and a new row evaluates `root.values[field.slot]` **during** instantiation — against whatever `values` holds at that moment, which under "rows then values" is the previous view's shorter array. The order is **`values`, then rows where they are written at all, then the epoch.** Writing new-view values while old rows still index them is harmless: those rows are destroyed by the next statement. I-F and §5.4 are corrected.

**Outcome:** verified — round 3. Values, then rows where structure changed, then the epoch — stated in §5.4 and as I-F, with the reason no row can evaluate against a stale or empty model.

### F-16 — §5.4 never says what the pickers hold when they open

**Severity:** major
**Location:** `design.md §5.4, Picking a datetime`

**Expected:** The state machine says what the date and time pickers are seeded
with when a field's button opens one, because both are retained root
singletons shared by every `datetime` field in the form.
**Observed:** The machine starts `Idle --> Date: button clicked, picking :=
(view, option, field)` and never writes the popup's value.
`DatePickerPopup` carries `in property <Date> date <=> base.date`, which
survives a close, so with no explicit write field B's picker opens on field A's
last pick. The obvious alternative — seeding from the draft — opens an
untouched field's picker at 1970, which is D-6's sentinel leaking into an
affordance D-6 chose the sentinel to keep out of.
**Evidence:** `design.md:332-347`; locked Slint 1.17.1
`widgets/fluent/datepicker.slint:13-24`, `widgets/fluent/time-picker.slint:13-24`;
`research.md:226-231`; `design-log.md:55-70` (D-6).

**Disposition:** fix-now
**Response:** Raised by the responder as second raiser; disposed as responder. Both pickers are retained root singletons — that is Thread 3's measured constraint, not an implementation choice — so their `date` and `time` survive a close and are shared by every `datetime` field in the form. §5.4's machine now writes the popup's value on open, from the draft where the field has been picked and from **today's date and 00:00 local** where it has not. Seeding an untouched field from the draft would open its picker at 1970, which is D-6's sentinel leaking into the one place D-6 chose the sentinel to keep it out of.

**Outcome:** verified — round 2. The repair now states both seeds—this field's draft, or today at local 00:00—and locates the clock/time-zone impurity in `instant.rs`; the missing bridge is a separate finding below.

### F-17 — A bounded `number` acquires a keyboard step the backend never sent

**Severity:** minor
**Location:** `design.md §5.2, The five widgets`

**Expected:** AC-9's principle — the host does not invent what the backend did
not send — covers granularity as well as range, or the design says why it does
not.
**Observed:** `SliderBase` declares `in property <float> step: 1`, and its
`key-pressed` handler rejects every key outright when `step <= 0`. So a
`Slider` drawn without a `step` still moves in units of 1 under the keyboard,
and cannot be given a continuous keyboard at all. On a field declared
`min: 0, max: 1` one arrow key traverses the entire range. `slice-009.md`
argues `step` out as *protocol*; neither it nor §5.2 notices that Slint
supplies one regardless.
**Evidence:** `design.md:174-180`; `slice-009.md:36-39`; locked Slint 1.17.1
`widgets/common/slider-base.slint:8,76-96,123-131`.

**Disposition:** fix-now
**Response:** Raised by the responder as second raiser; disposed as responder. `SliderBase` declares `step: 1` and rejects every key when `step <= 0` (`common/slider-base.slint:8`, `:76-80`), so a `Slider` drawn without a step still moves in units of 1 under the keyboard and cannot be given a continuous one. §5.2 sets `step` to `(maximum - minimum) / 100`, which is what Slint itself uses for `accessible-value-step` (`fluent/slider.slint:29`) — a presentational choice `R-18` leaves to the renderer, stated rather than inherited by default. The slice's non-goal on `step` is about **protocol** and is untouched.

**Outcome:** verified — round 2. The design makes the renderer-owned step explicit and gives its presentational rationale against R-18.

### F-18 — CD-1 will promote a rule that can submit a value outside a declared bound, and does not say so

**Severity:** minor
**Location:** `canon-delta.md CD-1`

**Expected:** Canon a backend author reads tells them what the host can send.
**Observed:** CD-1 states `number` → its declared minimum, or `0` where none
was declared. A range carrying only a `max` is legal and fixtured, so a field
declared `max: -10` submits `0` — outside the bound its own backend declared.
That is not an `R-35` breach, as round 1 established, and it is not a defect in
the choice; it is a consequence a backend author cannot discover from the text
as CD-1 currently words it.
**Evidence:** `canon-delta.md:20-27`; `SPEC-001/R-17`;
`crates/goad-semantics/src/protocol/canonical.rs::a_range_with_one_bound_or_none_is_accepted`;
`design.md:229-239`.

**Disposition:** fix-now
**Response:** Raised by the responder as second raiser; disposed as responder. Not a defect in the choice — round 1 established that submitting a value outside a declared bound is no `R-35` breach — but CD-1 is canon in waiting, and a backend author reading it cannot discover the consequence. CD-1 now states it: a range carrying only a `max` submits `0` as-drawn, which may lie outside that `max`, and `R-58`'s existing instruction — do not send the field when *unanswered* must be distinguishable — remains the way to avoid it.

**Outcome:** verified — round 2. CD-1 now states the max-only consequence and its relationship to R-35 and R-58.

### F-19 — The numeric reassertion guard collapses distinct legal `f64` edits to one `f32`

**Severity:** major
**Location:** `design.md §5.2, The guard`

**Expected:** AC-6's next present detects every divergence between a numeric widget and the retained draft, including a finite edit that was dropped or refused.
**Observed:** The numeric `LineEdit` guard compares `self.text.to-float()` with an `f32` comparand. Distinct finite `f64` strings can map to the same `f32`—for example both `1e100` and `1e101` map to positive infinity—so a dropped edit can differ from the draft while the guard reports equality. The offered argument that the value came from “this same widget” assumes the host recorded the edit, which is exactly what AC-6's case denies.
**Evidence:** `design.md:282-296,554-557,575`; `slice-009.md:85-86` (AC-6); locked Slint 1.17.1 `i-slint-core/src/string.rs:398-412` (`string_to_float` parses to `f32`); `SPEC-001/R-17` admits any finite `f64` bound.

**Disposition:** fix-now
**Response:** Correct, and it dismantles the defence the repair offered. `to-float` parses to `f32` (`i-slint-core/src/string.rs:398-412`), so distinct finite `f64`s collapse — `1e100` and `1e101` both to infinity — and the argument that the value "came from this same widget" assumes the host recorded the edit, which is precisely what AC-6's dropped-edit case denies.

The guard compares **text**, which is lossless, with one exception and no more: `self.text == ""` and the held value is zero. That is the measured cleared-field case (`numeric_guard.rs`) and nothing else, so the exception is as narrow as the evidence for it. Written as: converge unless the strings match, or unless the widget is empty and the held number is zero.

**Outcome:** verified — round 3. The `f32` comparand is gone: the guard compares the widget's text against the verbatim text retained beside the `Finite`, with only the measured empty-widget / held-zero exception.

### F-20 — `slider_bounds` admits legal ranges for which Slint's slider arithmetic is undefined or inoperable

**Severity:** major
**Location:** `design.md §5.2, slider_bounds`

**Expected:** `slider_bounds` answers `Some` only when the selected `Slider` can draw and operate over the full legal range; every other `R-17` range takes the always-working text control.
**Observed:** Exact endpoint round-trips are not sufficient. The predicate admits equal bounds such as `[1, 1]`, endpoints `[-f32::MAX, f32::MAX]` whose `f32` span is infinity, and `[0, f32::from_bits(1)]` whose `/ 100` step underflows to zero. Slint divides by `maximum - minimum` to place the thumb, and its keyboard rejects `step <= 0`, so these admitted sliders do not support the prose's claim.
**Evidence:** `design.md:229-247,270-276`; `SPEC-001/R-17` requires only finite bounds and `min <= max`; locked Slint 1.17.1 `widgets/common/slider-base.slint:78-80,113-115` and `widgets/fluent/slider.slint:74-76`.

**Disposition:** fix-now
**Response:** Correct: an exact endpoint round-trip is necessary and not sufficient. `[1, 1]` divides by a zero span, `[-f32::MAX, f32::MAX]` has an infinite one, and a span small enough makes `(max - min) / 100` underflow to a step of zero — which Slint's keyboard rejects outright. `slider_bounds` answers `Some` only when both endpoints round-trip exactly **and** the span is finite and strictly positive **and** the step is finite and strictly positive. Everything else takes the text control, which is where every one of these belongs anyway — a slider over a single value offers nothing.

**Outcome:** contested — round 3. The predicate rejects the three examples and still admits an inoperable step. `minimum = 2^100`, `maximum = 2^100 + 2^77`: both round-trip exactly, the `f32` span is finite and positive, and `span / 100` is finite and positive — but that step is below half an ulp at `minimum`, so `minimum + step` rounds back to `minimum` and `increment()`, which is exactly `set-value(value + step)` (`common/slider-base.slint:126-131`), does nothing. Positivity does not establish operability.

**Re-disposition (round 3):** fix-now
**Response:** Contest upheld, and the arithmetic checks out. At
`minimum = 2^100` the `f32` ulp is `2^77`, so a one-ulp span gives a step of
about `2^70.3` — below half an ulp — and `increment()`, which is exactly
`root.set-value(root.value + root.step)`
(`widgets/common/slider-base.slint:126-131`), returns the value unchanged.
Verified by hand against the locked source.

`slider_bounds` gains the condition it was missing, in the same form as the
other three — *Slint's arithmetic has to work afterwards*: `minimum + step`
must exceed `minimum`, and `maximum - step` must fall below `maximum`, both
evaluated in `f32`. That states operability directly rather than approximating
it with a magnitude rule, and it **subsumes** the third clause: a step that
underflows to zero fails it, and so does a non-finite one. The span clause
stays, because it is what makes the division safe to perform at all.

Everything the tightened predicate rejects takes the text control, which is
where a range a slider cannot operate belongs — the same answer §5.2 already
gives for equal bounds and an infinite span.

**Outcome:** verified — round 4. §5.2 carries the operability clause as the re-disposition stated it, and the arithmetic was re-derived from the locked source this round rather than taken from the Response: `increment()` is exactly `root.set-value(root.value + root.step)` and `set-value` returns immediately when the result equals the value it holds (`widgets/common/slider-base.slint:117-128`), and `key-pressed` rejects every key when `step <= 0` (`:78-80`). A step below half an ulp therefore freezes the slider, which is what the clause excludes.

### F-21 — I-G is a construction convention, not an invariant of the canonical internal type

**Severity:** major
**Location:** `design.md §5.5, I-G`

**Expected:** Past the checked markup boundary, the internal representation makes a non-finite submitted number unrepresentable, so every path to `submitted` produces the JSON number R-57 requires.
**Observed:** The proposed public variant is `Edited::Adjusted(f64)`, and public `Controller::edit` accepts any `Edited`; neither type excludes `NaN` or infinity. I-G's statement that the variant “is constructed only through a checked conversion” is therefore discipline, and an admitted internal value serializes as JSON `null`.
**Evidence:** `design.md:298-307,541-557`; `crates/goad/src/controller.rs:252-278`; locked `serde_json-1.0.151/src/value/from.rs:47-60`; `CLAUDE.md` (“Internal representations are canonical”); `SPEC-001/R-57`.

**Disposition:** fix-now
**Response:** Correct, and it is the sharper reading of `CLAUDE.md`'s *"internal representations are canonical"*. I-G said `Edited::Adjusted` "is constructed only through a checked conversion", which is a sentence about how people will behave. `Controller::edit` is public and takes any `Edited`; `f64` admits `NaN` and both infinities; `serde_json::Value::from(f64)` turns them into `null`, which `R-57` does not admit.

`Adjusted` comes to hold a checked finite newtype with a private field and a fallible constructor, so a non-finite submitted number is **unrepresentable** rather than merely unwritten. I-G then states a property of the type, which is what an invariant is.

**Outcome:** verified — round 3. `Edited::Adjusted` holds a `Finite` with a private field and a fallible constructor, so `submitted` cannot receive a non-finite value through `Edited`.

### F-22 — Picker seeding has no declared path from Rust state and time into the Slint button handler

**Severity:** major
**Location:** `design.md §5.2-§5.4`

**Expected:** The interfaces expose enough state or callbacks for the datetime button to seed the singleton popups from the typed draft or today's local date without parsing display text or inventing another impurity site.
**Observed:** `FieldValue` carries only display text, the only declared callback is UI-to-host `edited`, and `compose` is the only `instant.rs` interface. Nevertheless §5.4 assigns the button handler a typed draft/date/time read and today's local date. Slint cannot obtain either through the declared interface, so a planner must invent a reverse callback, more value fields, or markup-side parsing and decide where the impure call occurs.
**Evidence:** `design.md:199-217,309-328,366-370,403-410,494-521`.

**Disposition:** fix-now
**Response:** Correct, and it is a defect the F-16 repair introduced: §5.4 assigned the button handler a typed read of the draft and of today's local date, and §5.2 gave it no way to obtain either — `FieldValue` carries display text, the only declared callback runs UI-to-host, and `compose` is `instant.rs`'s only interface. A planner would have had to invent a reverse callback, extra value slots, or markup-side parsing of a formatted datetime, and to decide where a new impure call lives.

The seed is the **host's**, not the markup's. `FieldValue` gains typed `date` and `time` slots, written by `glass.rs` on every present beside the text — from the draft where the field has been picked, from today at 00:00 local where it has not. The button handler copies them into the popup and does nothing else. No reverse callback, no parsing, and the clock stays where `TimeZone::system()` already is.

**Outcome:** verified — round 3. `FieldValue` carries typed `date` / `time` slots, `glass.rs` writes them from `decompose` or `today_local`, and the button copies them to root seed properties the fresh popups bind to. No display-text parse and no reverse callback.

### F-23 — The replacement rationale for rejecting targeted rebuilds discards the refused command's identity

**Severity:** minor
**Location:** `design.md §7, D8`

**Expected:** An alternative is rejected on the actual information available at the loss site.
**Observed:** The design says the host can know that an edit was refused but cannot know which widget diverged. `try_send`, however, returns the complete refused `Command::Edit`, including view, option and field; `Wire::send` binds it as `_returned` and chooses to discard it. A targeted divergence signal may still be undesirable, but lack of identity is not the reason.
**Evidence:** `design.md:618`; `research.md:343-346`; `crates/goad/src/wire.rs:22-43,108-132`; Tokio's `TrySendError::Full(T)` value is the returned command.

**Disposition:** fix-now
**Response:** Correct, and this is the second time D8's rejection of *rebuild-when-refused* has rested on a bad argument. `TrySendError::Full(T)` returns the command, `Wire::send` binds it `_returned` and discards it deliberately (`wire.rs:126`), and a `Command::Edit` names view, option and field. The identity is available.

D8 is rewritten on the two grounds that survive. First, the alternative rests on an enumeration of the ways an edit can be lost, which `docs/memory/enumerate-the-class-not-the-instances.md` warns about and which Thread 4 never completed; the measured guard depends on no such enumeration. Second, even with the field in hand the only correction available without the epoch mechanism is a targeted row rebuild (Thread 4, *not taken but available*), which destroys the element of **precisely the field the person was typing in** — edits come from the field with the caret. Narrower than a whole-form rebuild, and fatal in the same way.

**Outcome:** verified — round 3. D8 now says `TrySendError::Full(T)` returns the whole command and cites the discard at `wire.rs:127-133`, rejecting targeted rebuilds on enumeration and caret grounds instead. The code binds and discards at `wire.rs:127-132`; the Response's `:126` was wrong.

### F-24 — The POL-001 residue argument attributes purity enforcement to a command canon says rejects nothing

**Severity:** major
**Location:** `design.md §10`

**Expected:** The shared-feature argument states the residual risk within POL-001's measured limits: the standalone semantics command checks only that stratum 1 builds with its own feature set, while review and the bounded source scan hold purity imperfectly.
**Observed:** The design claims `cargo test -p goad-semantics` would notice a stratum-1 source reaching for `std` and therefore the feature cannot hide a purity regression. The crate already uses `std`; POL-001 explicitly says this command “rejects nothing” and is not a purity check, while the actual purity scan does not reach I/O performed through a permitted dependency. The conclusion therefore overstates exactly the residue the section is required to argue honestly.
**Evidence:** `design.md:757-775`; `docs/policy/001-the-phase-gate.md:87-99,120-143`; `docs/adr/001-one-way-strata.md:34-42`; `crates/goad-semantics/src/error.rs:12,232-259` and `protocol/canonical.rs:19` (existing `std` use).

**Disposition:** fix-now
**Response:** Correct, and the overstatement is exactly the kind the section exists to prevent. `POL-001` says `cargo test -p goad-semantics` *"rejects nothing, and is not a purity check"*; the purity scan's stated blind spot is I/O performed on stratum 1's behalf by a permitted dependency; and `goad-semantics` already uses `std` (`error.rs:12`), so the premise that it would notice a `std` reach is false twice over.

§10 is rewritten to argue the residue honestly: **no gate command rejects this, and that is the point of `POL-001` requiring it to be argued.** What stands is narrower and true — `jiff` gains a capability, `goad-semantics` gains no call site, no import and no reachable behaviour, and `ADR-001`'s direction rule is about what stratum 1 may name and do. The residual risk is stated rather than dissolved: a future stratum 1 source could come to depend on a feature stratum 1 did not ask for, and only review would catch it.

**Outcome:** verified — round 3. §10 states the residue in the policy's own terms — no gate command rejects it, and only review would catch a later stratum 1 source depending on a unified capability. Matches `POL-001` §Verification.

### F-25 — The repaired driver table is based on a false no-loop limitation and still names outcomes instead of drivers

**Severity:** major
**Location:** `design.md §9`

**Expected:** Each automated row names the concrete API that can operate its control in the assigned tier, based on what the locked testing backend actually reaches.
**Observed:** §9 says no `tests/renderer` path reaches inside a popup and moves choice/datetime cases to the loop tier, yet the testing crate's own `init_no_event_loop` test opens a popup, queries its descendants through `active_popups`, and invokes an action inside it. It also exposes `mock_single_click` specifically for no-loop tests. The table then describes AC-8's driver only as “opened and an alternative selected” and gives no datetime popup driver at all; moving a case to a loop does not supply a mechanism.
**Evidence:** `design.md:655-685`; locked `i-slint-backend-testing-1.17.1/search_api.rs:289-310,952-974,1079-1090,1351-1444`; locked Slint 1.17.1 `widgets/fluent/combobox.slint:105-143`, `datepicker.slint:80-96`, `time-picker.slint:87-103`.

**Disposition:** fix-now
**Response:** Correct, and it follows from F-13 having been wrong. Verified independently: `i-slint-backend-testing`'s own `test_popups` runs under `init_no_event_loop`, and `ElementQuery::from_root` walks `active_popups` (`search_api.rs:296,309,343`), so the no-loop tier reaches inside a popup and has `mock_single_click` besides. The round-1 finding inferred a missing capability from no case in this repository using one — absence of a case is not absence of a driver.

§9 is rebuilt on what the API actually reaches. Every automated row names a concrete API, not an outcome: which query finds the control, which call operates it, and for a popup, which call opens it and which finds the item inside. Rows move back out of the loop tier wherever the no-loop tier can in fact drive them, and the loop tier keeps only what needs a real loop — the `changed` handlers and the debounce timer.

**Outcome:** verified — round 3. §9 names the operating call control by control, and the obligation table either refers to those drivers or names its own, reserving `changed` handlers and timers for a real loop.

### F-26 — The string-valued numeric control accepts a locale grammar that the host parser rejects

**Severity:** major
**Location:** `design.md §5.2, A number crosses as a string`

**Expected:** Text accepted by the numeric `LineEdit` crosses the string channel into the same numeric value, independent of the person's locale.
**Observed:** `input-type: decimal` validates with Slint's locale-aware `string_to_float`, replacing the locale decimal separator before parsing; the host is specified to call Rust `f64::from_str` on the original text, which accepts `.` but not a comma. In a comma-decimal locale, a control-approved value such as `1,5` is therefore refused and reasserted rather than recorded as `1.5`.
**Evidence:** `design.md:219-227,249-258,292-296`; locked Slint 1.17.1 `i-slint-core/src/string.rs:398-412` and `items/text.rs:2205-2229`; Rust `f64::from_str("1,5")` returns `ParseFloatError`.

**Disposition:** fix-now
**Response:** Correct. `input-type: decimal` validates through Slint's locale-aware `string_to_float`, which substitutes the locale decimal separator before parsing, while the host was specified to call `f64::from_str`, which accepts `.` and not `,`. In a comma-decimal locale the control approves `1,5` and the host refuses it, records nothing, and the guard then writes over the person.

D-16: the host parses with the rule the text was validated under — a single comma read as the decimal separator where no dot is present, then `f64::from_str`. Host-side, testable without a locale fixture, and numeric formatting rather than anything domain-shaped. Rejected: sending Slint's parsed float alongside the text, which would reintroduce as a fallback the `f32` path F-2 exists to remove.

**Outcome:** contested — round 3. The repair does not implement Slint's rule. `string_to_float` takes an arbitrary separator `char` from ICU and replaces *that* character, rejecting any `.` when the separator is not `.` (`i-slint-core/string.rs:398-412`; `i-slint-common/lib.rs:58-82`). A locale whose separator is neither `.` nor `,` is still accepted by the control and refused by the proposed host parser. The Response fixed the example locale, not the class.

**Re-disposition (round 3):** fix-now
**Response:** Contest upheld, and the class is wider than the repair assumed —
the separator is an arbitrary `char` from ICU
(`i-slint-common/lib.rs::decimal_separator_for_locale`), and it is **live in this
build**: `i-slint-core`'s default `std` feature enables
`i-slint-common/locale-decimal-separator` (`i-slint-core/Cargo.toml:82-95`).
Verified this session.

The separator is not reachable from host code. `SlintContext::locale_decimal_separator`
is `i-slint-core`, which `crates/goad` does not depend on, and the `slint` crate
re-exports neither it nor `string_to_float`. Reading it would mean a new
dependency or a second ICU lookup; both are out.

So the host stops trying to name the separator and instead accepts exactly the
class the control admits. `string_to_float` accepts a text one of two ways: the
separator is `.` and the text parses; or the separator is not `.`, the text
contains no `.`, and it parses once that one character is replaced
(`i-slint-core/string.rs:398-412`). The host mirrors that without knowing which
case it is in: parse the text as it stands; failing that, if it holds exactly one
character outside the numeric grammar, replace that character with `.` and parse
again. Empty text is still zero.

Which characters count as the numeric grammar is the plan's to pin down against
`f64::from_str`, and the rule stays what D-16 wanted: host-side, and testable
without a locale fixture.

**Outcome:** verified — round 4. The parse rule §5.2 now states is the one the re-disposition described, and it accepts every text `string_to_float` would accept without naming the separator (`i-slint-core-1.17.1/string.rs:398-412`, re-read this round). Whether the rule is needed at all is a separate question and is raised as **F-50**: the separator this application runs under is never set away from `.`. That does not make this finding's repair wrong — it makes it unreached.

### F-27 — §6 still closes OQ-4 with the one-binary constraint D-14 explicitly superseded

**Severity:** minor
**Location:** `design.md §6`

**Expected:** The open-question ledger names the current settled decision after D-14: real-loop cases use as many one-arrangement targets as their rows require, with the count settled by the plan.
**Observed:** OQ-4 is still closed by “D-10 — one new event-loop binary,” while §7 rejects that one-binary formulation and §9 leaves the number of targets to the plan. The design therefore gives the planner incompatible constraints in three sections.
**Evidence:** `design.md:593-599,624,702-708`; `design-log.md` D-14.

**Disposition:** fix-now
**Response:** Correct. §6 still closed OQ-4 with D-10's *one new event-loop binary*, which §7's D14 rewrote and §9 outgrew — three sections giving a planner three different constraints. §6's row is restated to what D-14 settled: as many one-arrangement targets as the rows require, the count settled by the plan.

**Outcome:** verified — round 3. The OQ-4 row now says the loop cases take as many one-arrangement targets as their rows need, count left to the plan. The one-binary constraint is gone.

### F-28 — A `Slider`'s edit cannot travel as a string, and F-2's repair made it

**Severity:** major
**Location:** `design.md §5.2, A number crosses as a string`
**Raised by:** the responder as second raiser, checking its own repair.

**Expected:** F-2's repair — the submitted number crosses as text — applies where
it removes a narrowing and nowhere it introduces a round trip.
**Observed:** It was applied to all three numeric paths, including the `Slider`,
whose `value` is `f32`. Putting that in `text` makes the markup format a float
whose string form Slint does not specify to round-trip, and the guard then
compares `self.value` against `f32(f64(that string))` — a difference that is an
artefact of the repair's own round trip, written back mid-drag. That is R4
exactly, reintroduced by the fix for F-2.
**Evidence:** `design.md:219-227` (the repair as written); `design.md:255`, `:280-296`
(the `Slider` row and the guard); `§8 R4`; locked Slint 1.17.1
`type-mappings.md:13-16`.

**Disposition:** fix-now
**Response:** Raised by the responder as second raiser against its own F-2 repair; disposed as responder. The repair routed **every** numeric edit through `text`, which is right for the two `LineEdit` controls and wrong for the `Slider`, whose `value` is `f32`: the markup would have to format a float whose string form Slint does not specify to round-trip, and the guard would then compare against a value parsed back from it — writing over a person mid-drag, which is R4 reintroduced by the fix for F-2.

`FieldEdit` keeps `number: float`, used **only** by the `Slider`. §5.2 states why that is not a narrowing: the `Slider` is admissible only over a range that is `f32`-exact (F-20 tightens that further), any `number` outside it draws the lossless text control, and `as_drawn` is computed host-side in `f64` and never crosses the boundary. What a slider can *produce* being granular is a property of the control — true of any slider, pixels included — not of the contract, which still admits and still answers every legal message.

**Outcome:** verified — round 3. The two numeric paths are distinct — text out of a `LineEdit`, a native `f32` out of a `Slider` — and the slider is reached only through the `f32`-admissibility predicate, so legal values outside it take the text control.

### F-29 — §5.1 still calls `TimeZone::system()` `instant.rs`'s one impure call

**Severity:** nit
**Location:** `design.md §5.1`
**Raised by:** the responder as second raiser, checking its own repair.

**Expected:** A repair does not contradict a section it did not touch.
**Observed:** §5.4's picker seeding (the F-16 repair) puts a read of today's local
date in `instant.rs` as well, so the module now has two impure calls. §5.1 still
says one.
**Evidence:** `design.md:176-178` against `design.md:366-370`.

**Disposition:** fix-now
**Response:** Raised by the responder as second raiser against its own F-16 repair; disposed as responder. §5.4's picker seeding put a read of today's local date in `instant.rs`, so §5.1's *"`TimeZone::system()` — the one impure call"* stopped being true. One sentence; listed because a repair contradicting a section it did not touch is the class round 2 was asked to look for, and this is the instance.

**Outcome:** contested — round 3. The sentence changed and the count is still wrong. §5.2 claims two reads, "the system zone, in `compose`, and the clock" in `today_local` — but `today_local() -> (Date, Time)` promises today's **local** date with no zone in scope, so it must read `TimeZone::system()` as well. The second system-zone call site is still unnamed.

**Re-disposition (round 3):** fix-now
**Response:** Contest upheld. `today_local` cannot answer a *local* date from the
clock alone; it reads `TimeZone::system()` too. There are two impure **kinds** of
read at **three** call sites: the clock, in `today_local`; and the system zone, in
`compose` and in `today_local` both. §5.1 and §5.2 name the three sites, because a
module whose whole reason to exist is holding the impurity should not undercount
where it performs it.

Nothing else moves: the manifest argument in §10 is about `TimeZone::system` being
called at all, which was already true.

**Outcome:** verified — round 4. §5.1 states it explicitly — *two impure kinds of read … performed at three sites: the clock in `today_local`, and the system zone in both `compose` and `today_local`* — and §5.2's `today_local` doc comment says *Reads the clock and the system zone*. The module no longer undercounts, which is what the finding asked for. Recorded rather than contested a third time: §5.2's closing prose still reads *Those two reads — the system zone, in `compose`, and the clock, here*, which enumerates the two **kinds** and is the sentence round 3 quoted. It is true of kinds and is contradicted by the doc comment two lines above it if read as an enumeration of sites.

### F-30 — The F-19 guard fights ordinary typing

**Severity:** blocker
**Location:** `design.md §5.2, The guard`
**Raised by:** the round-2 integrator (a fresh agent), against the F-19 repair.

**Expected:** The guard corrects a widget that diverged from the draft and is
silent otherwise — R4's standing risk, and what `numeric_guard.rs` measured.
**Observed:** The repair converges whenever `self.text != values[slot].text`,
and the held text is re-formatted from the `f64`, so text → `f64` → text is not
the identity. Typing `1.05`: after `1.0` the host holds `1` and formats `"1"`,
the guard overwrites, and the next keystroke yields `15`. Typing a negative
number: after `-` the parse fails, nothing is recorded, and the guard replaces
the sign with `"0"` — so a negative value cannot be entered at all if the
debounce fires between the sign and the first digit. Trailing zeros, `1e3` and a
comma-locale separator all go the same way. `to-float()` had none of this
because it compared numbers, which is why `numeric_guard.rs` passes over it.

F-19's objection to the `f32` comparand is correct and is not withdrawn. The
repair traded a rare extreme-value miss for common mid-typing corruption, which
is the worse of the two. Both the finding and its repair are wrong, so the
comparand needs an answer that is neither: a candidate worth putting to the user
is converging on **recency** rather than on difference — a per-field revision
the host advances when it writes, so the guard fires when the host's value is
newer than what the widget last sent, which is the question the guard is
actually asking.
**Evidence:** `design.md §5.2` *The guard*; `spike-fields/tests/numeric_guard.rs`;
`§8 R4`; F-19 above.

**Disposition:** fix-now
**Response:** Confirmed by measurement, and the finding's own example is off:
typing `1.05` under the design as written yields **`105`**, not `15`. The
negative case is worse than described — by the second keystroke the guard has
already replaced `-` with `0`, so the `3` lands on that and the sign is gone
rather than merely delayed.

This finding's objection and F-19's are both correct, and the comparand is the
wrong place to answer either. The defect underneath them is that the numeric
`LineEdit` is the only control whose held value is not what it displays: `f64`
→ text is not injective, so **no** comparison between the host's re-format and
the widget's text can be an identity, and every candidate comparand is a
different way of losing the same information. The repair makes it an identity
(D-18). `Edited::Adjusted` carries the text a person typed beside the `Finite`
it parsed to; `FieldValue.text` for a touched number is that text verbatim; the
guard compares string against string — which is what the *text* `LineEdit`
already does, and why its guard has never been in trouble. The one measured
exception, a cleared field over a held zero, is kept: it still earns its place
in the race between a clear and its own debounce.

Rejected: converging on **recency**, the per-slot revision this finding names as
a third candidate. It is mechanically available — a present that changes no
revision fires nothing, and one bump converges one field while its neighbour
stays mid-edit (`spike-fields/tests/revision.rs`) — and it is the better shape
in the abstract, because it deletes the comparand rather than correcting it.
It fails on this finding's own cases: `-` and `1e400` are edits the host cannot
record, which is exactly when a revision guard converges, so it writes over
them unless the host holds the text as well. Once the host holds the text there
is no comparand left to get wrong, and the revision buys only the deletion of
the one exception. Reconsidering it is cheap if that exception ever grows.

Measured in `spike-fields/tests/guard_text.rs`: seven slots against one guard,
two host policies, with an injection pass.

**Outcome:** verified — round 3. The host retains the typed text verbatim beside the last finite value and `FieldValue.text` is that text, so the guard compares like with like and stays quiet through negative prefixes, decimal intermediates, trailing zeros and overflow text.

### F-31 — The picker seed is a no-op exactly when two untouched fields are picked in turn

**Severity:** blocker
**Location:** `design.md §5.4, Picking a datetime`; `§7 D21`
**Raised by:** the round-2 integrator (a fresh agent), against the F-22 repair.

**Expected:** D21 — each popup is seeded on open, so field B's picker never
opens on field A's pick. That is the whole reason the decision exists.
**Observed:** A picker's `current-date` / `current-time` is bound to the `in`
property only until an in-popup selection assigns it, which destroys the
binding; after that only `changed date` / `changed time` re-syncs, and a
`changed` handler fires only when the value actually changes. Field A is
untouched, so it seeds to today; the person picks the 20th and accepts. Field B
is untouched, so it seeds to today **again** — the same value, no change, no
handler — and the popup still shows the 20th. Two untouched `datetime` fields
answered in turn is the ordinary case, and it is precisely what D21 was written
to prevent.
**Evidence:** `design.md §5.4` *Picking a datetime*, `§7 D21`; locked Slint
1.17.1 `widgets/fluent/datepicker.slint:15`,
`widgets/common/datepicker_base.slint`, `widgets/common/time-picker-base.slint`.

**Outcome:** withdrawn — the reading of `DatePickerBase` is correct and the
lifetime it assumes is not. `show-popup` compiles to a fresh
`#popup_window_id::new(...)` on **every** show, and the closed instance is
dropped from `active_popups`, so `current-date` cannot survive a close and
there is never a second open of the same instance to go stale. Measured: with
two untouched fields seeded identically, the second opens on its own seed, and
so does the picked field when reopened; giving the second field a differing
seed is the positive control, and the probe follows it
(`spike-fields/tests/picker_seed.rs`). Evidence: locked Slint 1.17.1
`i-slint-compiler/generator/rust.rs:3736-3763`,
`i-slint-core/window.rs:1955-1990`.

This is F-13's failure in the opposite direction, and the lesson is the mirror
of it: reading a widget's source tells you what an instance does, never how
long the instance lives. What the finding was right about survives as F-36 —
two arguments elsewhere in the design rest on the persistence it assumed.

### F-32 — §5.2 states Slint's accessible step rule as a bare quotient

**Severity:** nit
**Location:** `design.md §5.2`, the `step` paragraph
**Raised by:** the round-2 integrator (a fresh agent).

**Expected:** A citation says what the source says.
**Observed:** §5.2 justifies `(maximum - minimum) / 100` as "Slint's own for
`accessible-value-step`". Slint uses `min(root.step, (max - min) / 100)`. True
of the value as applied, not of the rule as stated.
**Evidence:** `design.md §5.2`; locked Slint 1.17.1 `widgets/fluent/slider.slint:29`.

**Disposition:** fix-now
**Response:** Correct as stated. Slint's rule is
`min(root.step, (root.maximum - root.minimum) / 100)`
(`fluent/slider.slint:29`), and under the design's own choice of `step` the two
agree — so the value is right and the sentence is not. §5.2 states the rule the
way Slint states it, and says why the design's `step` makes them coincide.

**Outcome:** contested — round 3. The formula is quoted and its gloss is still false. `min(root.step, span / 100)` is a **cap** on the accessible step, not "a floor on the step" as §5.2 now says. The Response promised to state what the source says; the integration reversed it.

**Re-disposition (round 3):** fix-now
**Response:** Contest upheld; the gloss was inverted when the repair was
integrated. `min(root.step, (maximum - minimum) / 100)` **caps** the accessible
step at a hundredth of the span — it is an upper bound, not a floor. Under this
design's `step`, which is exactly that hundredth, the cap binds at equality and
the two coincide, which is the only thing the sentence needed to say.

**Outcome:** verified — round 4. `accessible-value-step: min(root.step, (root.maximum - root.minimum) / 100)` (`widgets/fluent/slider.slint:29`, re-read this round), and §5.2 now says **caps** — an upper bound — and that this design's `step`, being exactly that hundredth, makes the cap bind at equality. The gloss matches the source in the direction it was inverted in.

### F-33 — §9's popup rows depend on a layout that no existing case exercises

**Severity:** minor
**Location:** `design.md §9`, the AC-8 and `choice`-re-asserting rows
**Raised by:** the round-2 integrator (a fresh agent), against its own F-25 repair.

**Expected:** A named driver works in the tier it is named for.
**Observed:** `mock_single_click` dispatches a pointer press and release at the
element's `absolute_center()`, so every row that clicks inside a popup needs
that popup laid out under `init_no_event_loop`. No case in this repository
exercises it. §9 names the injection pass as what proves the row can go red, which
is the right instrument; this is a planning risk rather than a defect, and it is
recorded so the plan meets it deliberately. If it fails, the rows move to the
loop tier and nothing else in the design changes.
**Evidence:** `design.md §9`; `i-slint-backend-testing-1.17.1/search_api.rs:968-974`.

**Disposition:** fix-now
**Response:** Correct, and narrower than raised. The date-picker chain does not
need `mock_single_click` at all: a calendar day cell is `accessible-role:
button` with the day number as its `accessible-label` and an
`accessible-action-default` (`common/datepicker_base.slint:59-63`), and the
dialog's OK is a `StandardButton` whose label is `OK`
(`common/standardbutton.slint:17-31`). Both drive through
`invoke_accessible_default_action`, which dispatches no pointer event and so
depends on no layout. Measured: `spike-fields/tests/picker_seed.rs` opens a
popup, picks a day and accepts, four times over, without one.

What survives is the `choice` row alone. `ListItem` carries an accessible role,
label, index and selected state but **no** default action
(`fluent/components.slint:15-19`), so AC-8 really does need `mock_single_click`
and really does depend on the popup being laid out. §9's rows are corrected to
say which of them needs a pointer, and §8 gains a row for the one that does.

**Outcome:** verified — round 3. The pointer/layout dependency is isolated to the `ComboBox`, whose `ListItem` has role, label, selection and index but no default action (`fluent/components.slint:49-53`); the date chain drives through default actions. R9 records the residual no-loop layout risk and the fallback tier.

### F-34 — Text that parses to a non-finite `f64` has no stated fate

**Severity:** minor
**Location:** `design.md §5.2, Parsing the text` and *`draft.rs`*
**Raised by:** the responder as second raiser, from the F-30 spike.

**Expected:** Every text the numeric control can hold has a stated behaviour on
the host side.
**Observed:** §5.2 states the parse rule in one place and `Finite`'s refusal of
a non-finite value in another, and never joins them. `input-type: decimal`
validates through Slint's `string_to_float`, which parses to `f32`, so `1e400`
is an infinity and is **accepted** by the control; `f64::from_str` then yields
infinity and `Finite::new` refuses it. The design does not say whether that
edit is dropped, refused, or recorded some other way — and under the guard as
written, a dropped one is written over on the next present, so the person
cannot leave it on screen either.
**Evidence:** `design.md §5.2`; locked Slint 1.17.1
`i-slint-core/string.rs:399-412`, `items/text.rs:2202-2229`;
`spike-fields/tests/guard_text.rs`, case `verbatim-overflow`.

**Disposition:** fix-now
**Response:** F-30's repair answers it, and §5.2 states it as one rule rather
than two halves: the host records the text, and the last representable number
stands. The widget keeps `1e400`, the guard is quiet because the strings agree,
and the wire keeps a finite value — measured as `1e400` on screen against
`1e40` on the host. The same rule covers `-`, `.` and `-.`, which the control
admits as len≤2 prefixes and which no parse will ever accept.

**Outcome:** verified — round 3. Every admitted text is retained, only a finite parse replaces the number, and the edge table repeats the rule for prefixes and for overflow.

### F-35 — The picker seeding mechanism §5.4 specifies does not compile

**Severity:** blocker
**Location:** `design.md §5.4, Picking a datetime`; `§5.2`'s `FieldValue`
**Raised by:** the responder as second raiser, from a compile error in the spike.

**Expected:** A stated interface can be implemented.
**Observed:** §5.4 says the button's handler "assigns those two slots to the two
popups and then shows the first". Slint rejects that outright: *"Cannot access
property or callback 'picker.date' inside of a Window from enclosing
component"*. A `PopupWindow`'s properties may be **bound** at its declaration
site; they may not be assigned from an enclosing component's handler. `show()`
is permitted, which is why the earlier spike did not find this — it only ever
called `show()`.
**Evidence:** `spike-fields/ui/spike.slint`, the `Pickers` component and its
comment; the `slint-build` 1.17.1 compile error against that file before the
rework; `design.md §5.4`.

**Disposition:** fix-now
**Response:** The seed becomes a root-owned property per popup — one `Date`,
one `Time` — which each popup **binds** to at its own declaration site. The
button's handler writes those two root properties from
`values[field.slot].date` / `.time` and then shows the first. `FieldValue`'s
two slots are unchanged, nothing runs host-ward, and the clock stays where
F-22's repair put it; only the direction of the last hop changes, from an
assignment into the popup to a binding out of it. Measured working in
`spike-fields/tests/picker_seed.rs`.

**Outcome:** verified — round 3. The root owns the seed properties, each popup binds at its declaration site, and the handler assigns only the root properties before `show()`.

### F-36 — Two arguments rest on a popup instance that does not persist

**Severity:** major
**Location:** `design.md §5.4, Picking a datetime` (closing paragraph); `§7 D21`; `§9`
**Raised by:** the responder as second raiser, from the F-31 measurement.

**Expected:** A tier assignment and a decision's rationale name a mechanism that
exists.
**Observed:** §5.4 says a rewritten seed is picked up "through a `changed date`
/ `changed time` handler of their own", and concludes that a case asserting
what a re-seeded picker opens on belongs in the loop tier. §7 D21's rationale is
that "without an explicit write, field B's picker would open on field A's last
pick". Both assume one popup instance living across opens. `show-popup`
compiles to a fresh `::new()` on every show and the closed instance is dropped,
so a seed is picked up by a **fresh binding**, no `changed` handler runs, and
there is no cross-field leakage to prevent. Seeding is still required — a
picked field must reopen on its pick rather than on the widget's default of
today — so D21 stands, on a different reason from the one it gives.
**Evidence:** locked Slint 1.17.1
`i-slint-compiler/generator/rust.rs:3736-3763` (`ShowPopupWindow` constructs a
new instance per show), `i-slint-core/window.rs:1955-1990` (the closed instance
is dropped from `active_popups`); `spike-fields/tests/picker_seed.rs`, probes 3
and 4.

**Disposition:** fix-now
**Response:** §5.4's closing paragraph is replaced by what was measured. D21 is
rewritten under its own id — §7 is current truth and its ids are immutable,
not its content — with seeding justified by the picked-field case rather than
by leakage between fields. §9's *"a picker picking up a **re**-written seed"*
loses its stated reason to sit in the loop tier.

That does not by itself move the row. Whether the no-loop tier can show a
popup and read a pick back is a driver question, and §9 answers driver
questions by naming the call — which is the check F-25's repair installed. The
row goes back through it rather than being reassigned here.

**Outcome:** verified — round 3. §5.4 states that every show constructs a fresh popup and a close drops it, D21 is justified by reopening a picked field on its own pick, and §9 treats the tier as a driver question.

### F-37 — Two of `Edited`'s five variants cannot be built where the command is built

**Severity:** blocker
**Location:** `design.md §5.2`, `wire.rs`'s `Command::Edit` / `PendingEdit` and
`draft.rs`'s `Edited`; `§7 D12`
**Raised by:** the round-3 integrator (a fresh agent), while integrating F-30.

**Expected:** A stated interface can be implemented.
**Observed:** `Command::Edit` and `PendingEdit` carry an `Edited`, so the `edited`
callback has to construct one. Two variants are not constructible there.

`Edited::Chosen` holds an `AlternativeId`. `AlternativeId::new` is `pub(super)` in
`goad-semantics`, so the value can only be *cloned off a retained view* — which is
the whole mechanism `§7 D12` rests AC-8 on, and §5.2 states it: "the ComboBox
reports its index, and `controller.edit` resolves the index against the drawn
field's alternatives". The callback holds a `Wire` clone and nothing else; the
presentation is behind the controller. So the index must travel, and no variant
can carry it.

F-30's repair adds a second instance of the same defect. After it, `Adjusted`
carries the typed text beside a `Finite`, and a text that does not parse finitely
keeps the number the field already holds — F-34's "the last representable number
stands". Which number that is depends on the draft and, for a field nobody has
touched, on the drawn kind's declared minimum. The callback knows neither, so it
cannot build that value either.

The two are one defect: `Edited` is being used both as what a widget reported and
as what the draft holds, and for two of five kinds those are different values
resolved at different places.
**Evidence:** `crates/goad/src/wire.rs:38-42`; `crates/goad/src/install.rs:37-45`
(the closure's whole capture is `editing`); `crates/goad/src/controller.rs:259-278`
(`edit`'s walk, which holds the declared field, and `&mut` the draft);
`crates/goad/src/draft.rs:50-57` (`state_of`); `design.md §5.2` *`draft.rs`*,
`§7 D12`; `canonical.rs:349-376`.

**Disposition:** fix-now
**Response:** One seam rather than two patches. A second value type names what a
callback can actually know, and one kind-directed function turns it into what the
draft may hold:

```rust
/// What a widget reported, in the widget's own terms — the most a callback can
/// know. `Command::Edit` and `PendingEdit` carry this.
pub enum Reported {
  Checked(bool),           // CheckBox
  Typed(String),           // text LineEdit
  AdjustedText(String),    // numeric LineEdit — the text as typed
  AdjustedValue(f32),      // Slider — its own value, an `f32` by nature (D16)
  Chosen(u32),             // ComboBox — `current-index`
  Picked { instant: Timestamp, offset: Offset },   // already composed (§5.4)
}

/// view_model.rs, beside `as_drawn`: what the draft should hold, given what the
/// widget reported and what the field shows now. `None` is a renderer bug — an
/// index no alternative has.
pub fn resolve(reported: &Reported, shown: &Edited, kind: &DrawnKind) -> Option<Edited>;
```

`controller::edit` calls `resolve` on the walk it already makes, with
`shown = state_of(..).unwrap_or_else(|| as_drawn(kind))` — the expression `answer`
already needs. `None` takes the `Refused::UnknownField` posture §5.2 already gives
an out-of-range index: reported, nothing recorded.

Six variants for five kinds, because `number` has two controls and §5.2 already
makes that a first-class fact (`slider: bool`, D16/D17). What the split buys
beyond making D12's and F-30's rules implementable: `Edited` becomes exactly what
the draft holds and `submitted` maps, `Reported` cannot express a non-finite
number or an unminted id **at all** — I-G and I-D get stronger rather than weaker
— and §5.2's locale-aware parse plus F-34's last-representable rule become one
pure function instead of a convention in a Slint closure.

Rejected:

- **An `Edited` with partial payloads** — `Adjusted { number: Option<Finite> }`
  and `Chosen` carrying an index until `controller::edit` normalizes them.
  `submitted` then needs arms for states the draft is promised never to hold,
  which is the "argument about why an `expect` is unreachable" §5.2 declines for
  `Finite::new`.
- **Resolving at submit time in `answer`** instead of at edit time. The
  presentation is there, but the last representable number is not: `1e400`'s text
  reparses to infinity, so the value would fall back to as-drawn and lose the
  `1e40` F-34's rule keeps on the wire.
- **`Finite::ZERO` as the fallback**, so `Draft::record` could merge without
  knowing the kind. It submits `0` for a field drawn showing its declared
  minimum — the quiet narrowing this design refuses everywhere else.

Cost: `Command::Edit`, `PendingEdit`, `Controller::edit`'s signature, one closure
in `install.rs`, and 26 `Edited::` sites — 10 in `draft.rs`'s own tests, 2 in its
`state_of` and `submitted`, 12 in `tests/renderer/wiring.rs`, one each in
`install.rs` and `glass.rs`. Every one is inside a surface this slice already
rewrites (§5.1, §9).

**Outcome:** verified — round 3. `Reported` carries only what a callback can construct, `controller::edit` resolves it against the held `Edited` and the retained `DrawnKind`, and both `Command::Edit` and `PendingEdit` carry `Reported`. Both construction failures are discharged. One false claim the split introduced is raised as F-42.

### F-38 — Pending edits lose their originating view and can be relabelled as edits to its replacement

**Severity:** blocker
**Location:** `design.md §5.1 pending.rs; §5.2 wire.rs; §5.4 Edges`
**Raised by:** round 3 (fresh reviewer)

**Expected:** Every delayed edit retains the `view_id` on which it occurred. Under `SPEC-001/R-32` and R-33, an edit from a replaced interaction must take the stale-view refusal path, never be applied to the replacement.
**Observed:** Pending state is only `Reported`, keyed by `(option, field)`; `PendingEdit` carries only option, field and value. On answer, all entries are put inside a `Choose` carrying the **current** view. If a replacement reuses the same option and field strings, an old pending edit is therefore accepted as an edit to the new view. If the strings differ it becomes `UnknownField`, not the `SupersededView` the edge table promises. The same missing value leaves the timer path with no stated source for the `view` required by `Command::Edit`.
**Evidence:** `design.md:194-207,655-686,690-697,768-785,933-935`; `crates/goad/src/wire.rs:24-42` explains why both current command variants carry `view`; `SPEC-001/R-32` and R-33 (`docs/specs/001-host-backend-protocol.md:145-146`).

**Disposition:** fix-now
**Response:** Correct, and the missing field is what the timer path needs anyway.
`PendingEdit` carries the `view` its edit was made on. The `edited` callback
already receives that view as its first argument, so it costs a struct field and
no new plumbing, and it is the only available source for the `view` a
`Command::Edit` requires once the send is deferred.

On a drain — timer or answer — an entry whose view is not the command's is
**discarded and reported** through the existing `SupersededView` site rather than
applied. That is what §5.5's edge row already promises and what `R-33` makes
true: the view was replaced, so the typing really was discarded, and saying so is
right. It also makes the map self-cleaning, so nothing accumulates across views.

Rejected: clearing the map when the row model is rebuilt. Same effect by a less
direct route — it needs the renderer to observe a new view separately from the
`set_vec` it already does, and it still leaves the timer path with no `view` to
send.

**Outcome:** verified — round 4. `PendingEdit` carries `view` (§5.2 `wire.rs`), §5.1 states the rule and §5.5 states it as I-H over three sites — shown, sent, drained — and the edges table carries a row for each. The display site, which the Response did not name and the integration added, is the one that closes the id-reuse case the finding was actually about.

### F-39 — The promised retry after a full answer send cannot be implemented through `Wire::send`

**Severity:** blocker
**Location:** `design.md §5.4, An answer; crates/goad/src/wire.rs:127-133`
**Raised by:** round 3 (fresh reviewer)

**Expected:** Because the design promises not to clear pending state until enqueue succeeds, the chosen callback must receive the `try_send` outcome (or the returned command) before it commits the drain.
**Observed:** The design says the callback drains pending entries into a consumed `Command::Choose`, calls one send, and nevertheless retains those entries when that send is `Full`. The stated/current interface is `Wire::send(&self, Command) -> ()`; it consumes the command, discards `TrySendError::Full(_returned)`, and exposes no success result. No interface change or alternate ownership protocol is specified. Thus the callback cannot distinguish success from `Full`, while `design.md:782-785` and the edge table assert that it does.
**Evidence:** `design.md:200-202,670-674,768-785,934`; `crates/goad/src/install.rs:25-31`; `crates/goad/src/wire.rs:127-133`.

**Disposition:** fix-now
**Response:** Correct. `Wire::send` reports whether the command was enqueued.
This is a return type rather than a new mechanism: `TrySendError::Full(command)`
already hands the whole command back at `wire.rs:127-133` and the site discards it
deliberately (D8), so the outcome is in hand and is being thrown away one line
before the caller that needs it.

The two callers this slice writes — the timer and `chosen` — clear `pending.rs`
only on an enqueued send, which is what §5.4 already promises. The existing
callers are unaffected: the result is advisory and the notice path is unchanged.

**Outcome:** verified — round 4. `pub fn send(&self, command: Command) -> bool` is declared in §5.2 with the enqueue contract beside it, and the existing site really does have the outcome in hand: `wire.rs:127-133` matches `Ok(())`, `TrySendError::Full(_returned)` and `Closed` separately today.

### F-40 — A present during the debounce treats a captured pending edit as a dropped edit and overwrites it

**Severity:** blocker
**Location:** `design.md §5.2 The guard; §5.3 ownership; §5.4 A keystroke`
**Raised by:** round 3 (fresh reviewer)

**Expected:** AC-4 and AC-5 require an edit already captured by the pending mechanism to survive a present during its 150 ms window; the guard should correct only an edit the host did not capture.
**Observed:** `pending.rs` and the draft are separate. Until the timer or answer flushes, the widget contains the new value while `glass.rs` derives `values` only from the old `Prepared` draft. Any intervening present writes that old value, bumps `epoch`, and the guard sees a difference and overwrites the widget—the exact path §5.4 calls a dropped edit. The empty-widget/held-zero exception covers one spelling only; ordinary text, numeric text, and a slider drag remain vulnerable. This also makes §9's human requirement of “a slider drag across a present” fail by construction.
**Evidence:** `design.md:194-206,399-443,690-708,729-733,737-766,1075`; `slice-009.md:84-91` (AC-4 through AC-6); the production loop presents at its top after every handled command (`crates/goad/src/controller.rs:738-744`).

**Disposition:** fix-now
**Response:** Correct, and it is the defect the whole pending map was always going
to have. *The host has not recorded this edit* and *the host has not recorded this
edit **yet*** are the same observation to the guard, and the debounce is what
created the second one. AC-6 rests on the first; §5.4's keystroke path assumed the
record always precedes the present, which holds only when nothing else is
handled in the window.

So the value channel is built from the draft **overlaid with what `pending.rs`
holds**: `glass.rs` takes a handle to the same `Rc` the callbacks hold, and a
pending entry for an (option, field) is preferred over the draft's value for it. A
present inside the window then writes back what the person typed, the strings
agree, and the guard is quiet. Nothing about the guard changes.

AC-6 keeps its meaning and gains precision: a widget is corrected exactly when the
host does not hold its value, where *hold* means the draft **or** pending. A
dropped or refused edit has left pending and never reached the draft, so it
converges as before; a `Full` send keeps its entry, so the widget stands and the
notice is what explains it.

What it costs, stated rather than absorbed. §5.3 says both models are derived from
`Prepared` alone and that there is therefore no cache and no invalidation rule.
`values` now also reads `pending.rs`. That is not a cache — it is live state with a
stated lifetime and a single writer — and the overlay has no invalidation rule
either, but the sentence has to say so rather than be quietly false.

One consequence to measure rather than assume: the overlay appears to **subsume**
§5.2's cleared-field exception, because a cleared field is a pending `""` and the
strings then agree on their own. The guard's comparand has been wrong three times
in this review and twice on reasoning, so the exception stays until
`numeric_guard.rs`'s case is re-run against the overlay and shown not to need it.

Rejected: a per-field suppression flag in `FieldValue`. It is the same information
spelled as *do not converge* rather than as *this is the value*, which leaves the
channel and the widget disagreeing on purpose and adds a second suppression
mechanism beside the one exception. Also rejected: dropping the debounce, which is
D-4 and a standing user commitment, not this review's to spend.

**Outcome:** verified — round 4. The overlay is stated in §5.3 as the draft's value preferred-over by a pending entry made on the view being presented, routed through `interpret` rather than a second mapping; the guard paragraph in §5.2 says what changed and what did not; D26 records it; and §9 carries the re-run of `numeric_guard.rs` against it rather than leaving the exception's fate in prose.

### F-41 — One single-edit timer cannot drain a map containing two pending fields without reverting one

**Severity:** blocker
**Location:** `design.md §5.1 pending.rs; §5.4 A keystroke`
**Raised by:** round 3 (fresh reviewer)

**Expected:** The map introduced for F-6 needs a delivery state machine that eventually records every entry while preserving each widget until it is recorded.
**Observed:** The design specifies one timer and says one `Command::Edit` leaves when it fires. With A and B pending, that can record only one. Handling that command immediately causes a present; the other entry is still absent from the draft and is reverted by the guard (F-40). Rearming the timer cannot prevent that intervening present, while not rearming strands the entry until answer. The validation table exercises one timed field and two fields only on the synchronous answer path, so neither row can catch this state.
**Evidence:** `design.md:194-206,696,737-756,1067,1073`; `crates/goad/src/controller.rs:652-675,738-744` (an edit is handled synchronously and the loop then presents).

**Disposition:** fix-now
**Response:** Correct, and two things answer it, neither a new command shape.

First, F-40's overlay: an entry still in `pending.rs` is what the value channel
shows, so the entry the timer has not sent yet is not reverted by the present that
follows the one it did send.

Second, the timer **re-arms while the map is non-empty**. The design said "one
`Command::Edit` goes down the channel, as today" and stopped, which was true of a
single pending slot and is not of a map. One entry per tick reaches the draft, and
the answer drains whatever is left in one `Choose` — so nothing waits on the
answer for correctness, only for immediacy. The channel holds one command
(`main.rs:86`), so one per tick is the most that is available; that is a
consequence of D22's constraint rather than a choice made here.

§9 gains the row that would have caught this: two fields edited inside one window,
the loop run past the debounce without answering, both values in the draft and
neither widget reverted. The existing rows exercise one timed field, and two
fields only on the synchronous answer path.

**Outcome:** verified — round 4. The re-arm is stated in §5.1 with the locked-source reading behind it, which checks out: `start_or_restart_timer` copies `being_activated` forward and `maybe_activate_timers` re-emplaces the old callback only where the register is still `Empty` (`i-slint-core-1.17.1/timers.rs:330-334`, `:348-360`). §9's *two fields, one window, no answer* row is the case that would have caught the original.

### F-42 — `Reported` can express non-finite slider values despite the claimed type invariant

**Severity:** major
**Location:** `design.md §5.2 Reported/resolve; §5.5 I-G`
**Raised by:** round 3 (fresh reviewer)

**Expected:** If the design claims non-finiteness is excluded by the boundary type, `Reported` must use a finite type; otherwise `resolve` must state and test the rejection path.
**Observed:** `Reported::AdjustedValue(f32)` admits `NaN` and both infinities. The design nevertheless says `Reported` “cannot express a non-finite number” and later that it carries a typed number only as text. `resolve`'s documented `None` surface names only an out-of-range choice index, leaving an implementer to invent what happens for a non-finite slider report (and for other report/kind mismatches). `Edited::Adjusted(Finite, ...)` still protects the wire, but the stronger claimed boundary invariant is false.
**Evidence:** `design.md:529-564,902-908`; Rust's `f32` type represented by that variant includes `NaN`, positive infinity and negative infinity.

**Disposition:** fix-now
**Response:** Correct, and the false claim is the integrator's rather than the
finding's subject. `resolve`'s `None` surface is stated in full, the way §5.2
already states `compose`'s four fallible steps and for the same reason — a step
left off the list becomes an `unwrap`. There are two: an index no alternative has,
**and** a non-finite `AdjustedValue`. Both are renderer bugs and take the
`Refused::UnknownField` posture: reported, nothing recorded.

§5.2's claim becomes what is true. `Reported` cannot express an id nobody declared
— that half stands, and `AlternativeId::new` is why. A number reaches the draft
only through `resolve`, which refuses a non-finite one, so I-G is held by `Finite`
at the wire and by `resolve` at the boundary. It is not held by the shape of
`Reported`, and §5.5 I-G says so.

Rejected: giving the variant a `Finite` payload. `Finite::new` would then run
inside a Slint closure, which has nothing to report a refusal to and no draft to
leave alone — the refusal belongs where the other renderer-bug refusals already
are.

**Outcome:** verified — round 4. `interpret`'s `None` surface is stated in full and grew a third case (D-31); I-G now says in terms that the invariant is **not** held by the shape of `Reported`, and names the two places that do hold it.

### F-43 — The chosen callback cannot drain pending edits in declared field order from the state it owns

**Severity:** minor
**Location:** `design.md §5.1 pending.rs; §5.4 An answer`
**Raised by:** round 3 (fresh reviewer)

**Expected:** A promised ordering must be derivable where the design says it is applied, or the controller must be assigned the ordering explicitly.
**Observed:** The chosen callback is required to drain in declared field order, but its pending map contains only `(option, field) -> Reported`; neither the key nor `PendingEdit` carries a slot/order, and `install.rs` does not own the retained `Presentation`. It can produce insertion order or key order, not declaration order. The controller could reorder against its presentation, but the design assigns the ordering to `chosen` and does not specify such a step.
**Evidence:** `design.md:194-202,655-680,690-697,768-773`; current callback ownership is visible at `crates/goad/src/install.rs:24-31`.

**Disposition:** fix-now
**Response:** Correct. The ordering moves to where an order actually exists: the
controller applies carried edits on the declared-field walk it already makes for
`answer`, so the order is a property of that walk rather than of the map, and
§5.4 stops asking the callback for something it cannot derive.

Nothing observable changes either way — the keys are distinct by construction, so
applying them in any order yields the same draft. That is the reason the promise
was safe to make and is also the reason it was not worth making.

**Outcome:** verified — round 4. §5.2 states that no order is promised and why the promise was empty.

### F-44 — The validation ledger discusses picker reseeding but binds no test obligation for it

**Severity:** major
**Location:** `design.md §9`
**Raised by:** round 3 (fresh reviewer)

**Expected:** D21's repaired mechanism needs a table row that reopens an already-picked field and asserts the fresh popup starts from that field's retained date and time, with the tier and driver settled by the plan.
**Observed:** §9 calls reseeding a “case,” discusses which tier might drive it, and says the plan will settle it, but the obligations table has no such row. AC-2 completes one pick and asserts only the wire type; it never reopens the field or observes the seed. As with F-11, prose outside the table does not bind the plan, and this mechanism has already been wrong in two different ways.
**Evidence:** `design.md:827-867,984,1052-1075`; the only picker measurement cited was under a real loop (`research.md:289-318`, `spike-fields/tests/picker_seed.rs`).

**Disposition:** fix-now
**Response:** Correct, and it is this session's own gap: F-36's repair took the
re-seed out of the loop-tier list and left it in prose, which is exactly the
failure F-11 raised about outcome-only rows. §9 gains an obligations row.

What it asserts: a field that has been picked, reopened, opens its fresh popup on
**that field's** retained date and time rather than on today. Driver, named as a
call: the button's `invoke_accessible_default_action` to open, the day cell's and
the `OK` `StandardButton`'s default actions to pick and accept, then the same
button again and a query of the popup's `date`. No pointer, so no layout
dependency. Tier: `tests/renderer/`, on round 2's verified fact that
`ElementQuery`'s `find_all` walks `active_popups` under `init_no_event_loop`
(`search_api.rs:291-312`). If the popup cannot be found there the row moves to the
loop target — R9's shape, not a new rule.

**Outcome:** verified — round 4. §9's obligations table carries the row, with its tier, its driver named as a sequence of calls, and the fallback if the popup cannot be found.

### F-45 — CD-2 names an R-16 canon change that neither it nor the design specifies

**Severity:** minor
**Location:** `canon-delta.md CD-2; design.md §10`
**Raised by:** round 3 (fresh reviewer)

**Expected:** Every canon row named for amendment has a concrete stale statement or new verification case identified for audit reconciliation.
**Observed:** CD-2's heading paragraph says the R-16 row “gains cases,” but its three changes cover only R-57, R-58 and R-55. The design's own CD-2 summary likewise names only those three. `SPEC-001`'s existing R-16 verification row already enumerates all five kinds in their wire forms, so the missing change cannot be inferred from an expired subset claim.
**Evidence:** `canon-delta.md:51-67,69-101`; `design.md:1114-1120`; `docs/specs/001-host-backend-protocol.md:419`.

**Disposition:** fix-now
**Response:** Correct. The `R-16` mention goes rather than gaining a fourth
change. `SPEC-001` §Verification's `R-13, R-14, R-16` row is about the wire forms
normalization accepts: it already names `R-16-a-{text,boolean,datetime,choice}-field`
and both `number` fixtures, *"every kind in its wire form"*. Drawing a kind in a
renderer changes nothing it claims. CD-2's document line names the three rows its
three changes touch, which is what `design.md` §10's summary already said.

**Outcome:** verified — round 4. `canon-delta.md` CD-2 now says the `R-16` row was named in error and why, its **Document:** line names the three rows its three changes touch, and `design.md` §10's summary agrees.

### F-46 — F-43's re-disposition assigns the drain ordering to a walk that cannot carry it

**Severity:** minor
**Location:** `review-design.md` F-43 Response; `design.md §5.2 wire.rs; §5.4 An answer`
**Raised by:** the session integrating round 3 (a fresh agent), writing the repair down.

**Expected:** A disposition's stated mechanism is one the code can perform.
**Observed:** F-43's Response moves the ordering to *"the declared-field walk it
already makes for `answer`"*. `Controller::answer` is `&self` and walks
`drawn_fields(matched)` for the **answered option only**
(`controller.rs:216-241`), while §5.2 requires a carried edit to be applicable to
*another* option's field — *"a person can type into one option's field and then
answer another"* — and applying an edit needs `&mut self`. So that walk is
neither where nor when the carried edits are applied, and it cannot reach the
edits that are not the answered option's.
**Evidence:** `crates/goad/src/controller.rs:216-241` (`answer`, `&self`, one
option), `:259-279` (`edit`, `&mut self`, `selected` then the drawn walk);
`design.md` §5.2, the `PendingEdit` paragraph.

**Disposition:** fix-now
**Response:** Correct, and the finding's own substance survives intact: the
ordering promise is dropped, which is all F-43 asked for. What goes with it is
the replacement mechanism, because there is nothing to replace it with and
nothing that needs one — F-43's own argument is that the keys are distinct, so
any order yields the same draft.

So §5.2 states that **no order is promised** over the carried edits, and says why
the promise could not have been kept where it was assigned: the callback holds
`(option, field)` keys and no declaration order, and `answer`'s walk is `&self`
and covers one option. The controller applies each carried edit through the walk
`edit` already uses, which is what §5.2 already said and is the only walk that
reaches an edit to an option that is not being answered.

**Outcome:** verified — round 4. §5.2 says no order is promised, says why the promise could not have been kept where it was assigned, and routes each carried edit through `edit`'s walk. That walk really does reach another option's field: `Controller::edit` is `&mut self` and takes `option` as an argument, resolving it through `selected(prepared, view, option)` rather than through anything the answer selected (`controller.rs:259-279`).

### F-47 — The overlay stops AC-6's stated driver from reaching the dropped-edit case

**Severity:** major
**Location:** `design.md §9`, the AC-6 row
**Raised by:** the session integrating round 3 (a fresh agent), writing the repair down.

**Expected:** A validation row's driver still produces the state the row asserts
about, after the repair that changed that state.
**Observed:** AC-6's driver is *"`set_accessible_value` on a `LineEdit` whose
callback reaches no `Wire`, so nothing is recorded — the dropped-edit case
exactly — then a present"*. Under F-40's overlay that is no longer the
dropped-edit case: the `edited` callback writes the entry into `pending.rs`
before any `Wire` is involved, so the host *does* hold the value, the overlay
shows it, and the guard is correctly quiet. `reasserts` stays at zero and the row
passes only if it asserts nothing. F-40's Response states the new meaning — *"a
dropped or refused edit has left pending and never reached the draft"* — but the
row was not revisited against it.
**Evidence:** `design.md` §5.2 *The guard*, §5.3 *A field's value is the draft's,
overlaid*, §9's AC-6 row; `slice-009.md` AC-6.

**Disposition:** fix-now
**Response:** Correct, and it is this integration's own near miss — the row would
have been written green. The driver gains its missing half: the loop runs past
the debounce so the entry is **sent and leaves the map**, and only then is the
present taken. The `Wire` is one whose receiver nothing drains, so the first send
is enqueued (capacity one), the entry clears, and nothing is ever recorded — which
is the dropped-edit case stated in terms of where the value is rather than in
terms of which callback was wired.

§9's row says both halves and says why both are needed. `slice-009.md`'s AC-6 gains
the same precision in one sentence, because *dropped* has stopped meaning *the
command did not arrive* and started meaning *the host holds it in neither the
draft nor `pending.rs`*.

**Outcome:** verified — round 4. §9's AC-6 row carries both halves and says why both are needed, and `slice-009.md`'s AC-6 carries the matching sentence. The driver works as described: with nothing draining the receiver the first `try_send` still succeeds on a capacity-one channel, so the entry clears on enqueue and nothing is ever recorded.

### F-48 — A float inside `Command` makes the `Eq` derive fail, and a hand-written one would be unsound

**Severity:** nit
**Location:** `design.md §5.2`, `Reported` and `Edited`; `crates/goad/src/wire.rs:21`
**Raised by:** the session integrating round 3 (a fresh agent), writing the repair down.

**Expected:** A design that changes a type carried by an existing `derive` says
which derives survive.
**Observed:** `Command` derives `Debug, Clone, PartialEq, Eq` (`wire.rs:21`) and
`Edited` the same (`draft.rs:26`), and both hold only a `bool` today.
`Edited::Adjusted` gains a `Finite(f64)` and `Reported::AdjustedValue` an `f32`,
so `Eq` stops deriving on all three. The design says nothing, and the obvious
repair — writing `Eq` by hand, which compiles — is unsound over
`AdjustedValue(NaN)`, which F-42 has just established is expressible.
**Evidence:** `crates/goad/src/wire.rs:21`; `crates/goad/src/draft.rs:26`;
`crates/goad/src/view_model.rs:81` (`Body`, already `PartialEq` without `Eq`).

**Disposition:** fix-now
**Response:** Correct. One sentence in §5.2 beside `Reported`: `Command` and
`Edited` carry `PartialEq` and drop `Eq`, which is the shape `Body` already has,
and nothing needs `Eq` — the cases that compare commands need `PartialEq`. Written
down rather than left to be met as a derive error, because the wrong repair also
compiles.

**Outcome:** verified — round 4. §5.2 states which derives survive, on `Command` and `Edited`, and the `Finite` paragraph closes the same trap at the leaf.

### F-49 — The overlay is only observable if the callbacks and the glass hold one `Rc`, and a split handle fails silently

**Severity:** minor
**Location:** `design.md §5.3, §9`
**Raised by:** the session integrating round 3 (a fresh agent), writing the repair down.

**Expected:** A mechanism whose failure mode is a vacuous pass has the failure
named where the cases are commissioned.
**Observed:** F-40's repair has `glass.rs` take *"a handle to the same `Rc` the
callbacks hold"*, and says nothing about what happens if it does not. Two
signatures widen — `install` and `SlintGlass::new` — and four call sites pair
them (`main.rs`, `tests/renderer/`, `tests/event_loop/`,
`tests/event_loop_schedule/`). A test that constructs a `Pending` for each half
is the natural spelling, produces an overlay that never overlays anything, and
leaves every case green: `docs/memory/a-green-test-can-assert-a-proxy.md`'s
failure, one level up from the tests.
**Evidence:** `crates/goad/src/main.rs:90,97`;
`crates/goad/tests/renderer/fields.rs:287`, `harness.rs:60`;
`crates/goad/tests/event_loop/closing.rs:57,59`;
`crates/goad/tests/event_loop_schedule/scheduling.rs:84,86`.

**Disposition:** fix-now
**Response:** Correct. Three places carry it rather than one, because the risk is
that nobody looks: §5.3 says the `Rc` is created once before both and that
`main.rs`'s existing construction order already allows it; §8 gains **R10**, whose
signal is *a case asserting the overlay that passes without `install` having been
called*; and §9 lists the four call-site pairs alongside the other widening
signatures, with the rule that each pair is given one value and not two.

**Outcome:** verified — round 4. Three places carry it as the Response promised: §5.3's one-`Rc` paragraph, §8 R10 with the vacuous-pass signal named, and §9's list of the four call-site pairs with the rule that each is given one value.

### F-50 — §5.2's whole locale account rests on a decimal separator this application never sets

**Severity:** major
**Location:** `design.md §5.2`, *Parsing the text is done under the rule the
control validated it with* and *Two sets of texts*; §7 D23
**Raised by:** round 4 (fresh Claude agent)

**Expected:** A design mechanism carried for a hazard names a hazard the built
application can reach, or says that it cannot and why it is carried anyway.
**Observed:** `string_to_float` reads the separator from
`SlintContext::locale_decimal_separator`, which is a plain `Property<char>`
initialised to `i_slint_common::DEFAULT_DECIMAL_SEPARATOR` — `'.'` — and carries
no binding. Exactly three sites write it: `SlintContext::set_locale`, documented
*"Override the locale used for decimal separator detection (**testing only**)"*
and called from nowhere but `i-slint-backend-testing`; and two arms of
`translations::select_bundled_translation`, which requires the `.slint` file to
have been compiled with bundled translations **and** an explicit call to select
one. `set_bundled_languages` consults `sys_locale` for the *language index* and
does not touch the separator.

`crates/goad/build.rs` calls `compile_with_config` with `with_debug_info` and
`with_style` only — `with_bundled_translations` is never used — and neither
`crates/goad/src` nor `crates/goad/ui` nor `crates/goad/tests` names
`select_bundled_translation`, `set_locale` or any translation API. So the
separator is `'.'` for the whole life of every process this workspace builds, on
every platform, whatever the person's system locale says. `string_to_float`
always takes its `sep == '.'` branch.

The consequence is not that the parse rule is wrong — it is a superset and costs
nothing. It is that F-26's premise, D-23's rejected alternative, the *two sets*
paragraph's first set (*"the locale's separator"*, *"`-`, `,` and `-,` in a comma
one"*), §5.5's matching edge row and `notes.md` §*Facts verified by hand* item 4
(*"The ICU decimal separator is live in this build"*) all describe a mechanism
that is compiled in and never armed. Item 4 established that the **feature** is
enabled; nothing established that the **value** is ever populated. The
one-foreign-character substitution rule the design commissions the plan to build
therefore has, today, no case it is the answer to — and F-52 is what it does
instead.

**Evidence:** `i-slint-core-1.17.1/context.rs:122-125` (initialised to the
default, no binding), `:296-299` (the getter), `:302-309` (`set_locale`, "testing
only", the only other writer besides translations);
`i-slint-core-1.17.1/translations.rs:436-444` (the two writes, inside
`select_bundled_translation`), `:382-394` (`set_bundled_languages` sets only
`translations_dirty`); `i-slint-common-1.17.1/lib.rs:22`
(`DEFAULT_DECIMAL_SEPARATOR = '.'`); `i-slint-core-1.17.1/string.rs:398-412`;
`i-slint-backend-testing-1.17.1/internal_tests.rs:122-130` (the only caller of
`set_locale` in the locked tree); `crates/goad/build.rs:36-42`;
`slint-build-1.17.1/lib.rs:200` (`with_bundled_translations`, unused);
`design.md:308-332,352-371`; `notes.md` §*Facts verified by hand* item 4.

**Disposition:** _pending_

**Outcome:** _pending_

### F-51 — The numeric boundary's format direction is locale-blind while its parse direction is locale-aware

**Severity:** minor
**Location:** `design.md §5.2`, *A number at the markup boundary* and
*Formatting a number is the inverse of that rule*
**Raised by:** round 4 (fresh Claude agent)

**Expected:** §5.2 says the boundary *"is crossed differently in each direction,
and the rules are stated together because they are one account of one boundary
rather than four separate precautions."* An account of one boundary covers both
directions under the same fact.
**Observed:** Only the inbound direction is written against the separator. The
outbound rule is `f64`'s `Display`, or `{:e}` past 24 characters — both of which
always spell the decimal point `.`, unconditionally. Where the separator is not
`.`, that is text the control's own validator refuses: `accept_text_input`'s
two-byte escape does not apply to a candidate longer than two bytes, and
`string_to_float` returns `None` for **any** text containing a `.` when the
separator is not `.`. So a `number` field whose as-drawn value is non-integral —
`min: 2.5` is a legal `R-17` bound — is drawn showing `2.5` and then refuses
every inserted character, because each candidate still contains the dot. The
person can still select-all-and-retype, and can still delete (deletion is not
validated), so the field is not dead; it is not incrementally editable, which is
the one thing the design's whole guard account is about.

The same is true of every `{:e}` spelling, which also carries a `.`, and of the
`Slider`'s `AdjustedValue` text, which §5.2 says is *"the host's format of the
number"*.

Conditional on the same premise F-50 removes, so nothing reaches this today. It
is raised separately because the repair is a different one — F-50 decides whether
the account is carried at all; this decides whether the account, if carried, is
complete — and because §5.2 claims completeness for it in as many words.

**Evidence:** `i-slint-core-1.17.1/items/text.rs:2208-2229` (the `candidate.len()
<= 2` escape, then `string_to_float`), `string.rs:398-412` (`if
string.contains('.') { return None }`); Rust's `Display` and `LowerExp` for `f64`
both emit `.`; `design.md:302-307,373-390`.

**Disposition:** _pending_

**Outcome:** _pending_

### F-52 — Paste bypasses `input-type: decimal` entirely, so the admitted class is every string and the substitution rule fires on arbitrary text

**Severity:** major
**Location:** `design.md §5.2`, *Two sets of texts sit behind that rule* and the
parse rule; §5.5 Edges, the numeric-text row
**Raised by:** round 4 (fresh Claude agent)

**Expected:** §5.2's two sets are *"the class the control admits"*, and the parse
rule is justified by mirroring what `string_to_float` would have accepted: *"The
two agree everywhere the control can reach."*
**Observed:** `TextInput::paste` calls `paste_clipboard`, which calls
`TextInput::insert`, which performs **no** validation at all — it does not consult
`input_type` and never calls `accept_text_input`. `accept_text_input` is reached
from exactly two places, both key-event paths: a `KeyPressed` character insertion
and an IME `UpdateComposition`/`CommitComposition`. `StandardShortcut::Paste` is
dispatched before either, straight to `paste()`. So the class the control admits
is **every string**, not the two sets §5.2 names, and the design's own warning —
*"an implementer who reads the trio as the closed enumeration writes a three-case
test and never reaches the paste"* — applies one level up to the paragraph that
makes it.

The consequence is not confined to the prose. The parse rule's second clause is
*where exactly one character of the text falls outside the grammar `f64::from_str`
accepts, replace that one character with `.` and parse again*. Its only
justification is mirroring the separator substitution. Applied to text the
control never validated, it silently manufactures a number from arbitrary input:
a pasted `12/25` records `12.25`, `3:30` records `3.30`, `1x5` records `1.5`, and
`$5` records `0.5`. In each case the text is recorded verbatim and displayed, the
guard compares string against string and is correctly quiet, and the answer
carries a number the screen never showed. This is reachable with one keystroke by
a person pasting a value out of another application, which is the ordinary way a
number gets into a form.

`Reported::AdjustedText` and *the text is recorded verbatim, always* are both
total, so nothing panics and `Finite` still keeps the wire clean. What fails is
the design's stated correspondence between what the control admits and what the
host reads it as, and D-6's own rule against holding *"a value nobody gave, as
though someone gave it."*

**Evidence:** `i-slint-core-1.17.1/items/text.rs:1940-1958` (`paste` →
`paste_clipboard` → `insert`), `:1783-1827` (`insert`, no validation),
`:1032-1036` (`StandardShortcut::Paste` dispatched before the insertion path),
`:1067` and `:1117` (the only two `accept_text_input` call sites, both key
events), `:2202-2231` (`accept_text_input` itself); `design.md:334-371`,
`design.md` §5.5 Edges, the *numeric text that is not a number* row.

**Disposition:** _pending_

**Outcome:** _pending_

### F-53 — I-H names one place display and submission part company; §5.2's own rules give at least two more

**Severity:** major
**Location:** `design.md §5.5 I-H`
**Raised by:** round 4 (fresh Claude agent)

**Expected:** An invariant states a property that holds, and any exception list
attached to it is complete — this is the third time a claimed property has been
stronger than the design supports (F-21 on I-G as a convention, F-42 on
`Reported`).
**Observed:** I-H closes with *"for every field anyone has touched, what the
screen shows is what an answer would submit"* and then *"The one place display
and submission part company is an untouched `datetime`."* §5.2 gives two more,
both for a field somebody has touched:

- **A numeric text the host cannot parse finitely.** §5.2's own worked example:
  `1e400` *"stays on screen as `1e400` while the host keeps `1e40` as the number
  it would submit"*. The same holds for `-`, a lone separator, `inf` and `nan` —
  the text is displayed, the last representable number is submitted, and §5.5's
  own edges row says so.
- **A cleared numeric field.** The screen shows `""` and the answer submits `0`.
  §5.2 and the edges table both state it, and the guard's one exception exists
  precisely because the two disagree.

Neither is a defect in the behaviour — both are deliberate and argued. The defect
is the invariant, which is the sentence a reader trusts furthest and the one an
implementer would turn into an assertion. I-H's first two sentences (an entry is
used only against the view it was made on; while it exists it is what the screen
shows) are sound and are what the three sites hold; the generalisation past them
is not.

**Evidence:** `design.md` §5.5 I-H; `design.md:340-350` (`1e400` recorded
verbatim, `1e40` held), §5.5 Edges rows *numeric field cleared to `""`* and
*numeric text that is not a number*; `design.md` §5.2, the guard's exception.

**Disposition:** _pending_

**Outcome:** _pending_

### F-54 — §10's residue argument does not reach the workspace's existing decision to keep jiff's `std` out, which `clock.rs` implements and documents

**Severity:** major
**Location:** `design.md §10`, *The `jiff` feature, and why it is here*; §7 D18
**Raised by:** round 4 (fresh Claude agent)

**Expected:** `POL-001` requires the decision to add a feature to a dependency
shared with stratum 1 to be **argued** in the slice that takes it, because no gate
command rejects it. An argument that does not engage the one place in this
workspace where that same feature was already refused, on the record, is
incomplete.
**Observed:** `goad-shell/src/clock.rs` does not call `jiff::Timestamp::now()`.
It reads `SystemTime::now().duration_since(UNIX_EPOCH)` and rebuilds a
`jiff::Timestamp` from nanoseconds, and its doc says why in as many words:

> **Not** `jiff::Timestamp::now()`, which needs jiff's `std` feature — and
> features unify across the workspace build, so enabling it anywhere enables it
> in stratum 1, which carries `jiff` with `default-features = false` for exactly
> that reason (the workspace `Cargo.toml`'s `jiff` line, D25).

This slice enables `std` in stratum 1's workspace build — `tz-system = ["std",
…]` and `tzdb-zoneinfo = ["std"]` — which is exactly what that comment was
written to prevent. Three things follow that §10 does not say. The comment
becomes **false as a rationale**: after this slice the feature is on whatever
`clock.rs` does. The hand-rolled `SystemTime` arithmetic becomes a workaround for
a constraint that no longer binds — a second implementation of
`Timestamp::now()`, kept for a reason that has expired. And a prior slice's
recorded decision is reversed by a manifest line rather than by an argument that
names it.

This is the class §5.1 already handles well for `FieldForm`: a change that makes a
live doc comment untrue is enumerated, with what it costs, rather than left to be
found. §10's *"what the feature does not gate"* paragraph bounds the change to
`compose` and `today_local` **inside this slice**, which is true and is not the
whole reach.

Not an argument against the feature — D-7 needs it and §10's ADR-001 reasoning
stands. An argument that §10 is not yet the complete argument `POL-001` asks for,
and that the reconciliation this creates has no owner.

**Evidence:** `crates/goad-shell/src/clock.rs:46-52` (the rationale),
`:64-77` (the workaround it justifies); root `Cargo.toml:36` (`jiff =
{ version = "0.2", default-features = false }`); `jiff-0.2.35/Cargo.toml`
`tz-system = ["std", "dep:windows-link"]`, `tzdb-zoneinfo = ["std"]`,
`std = ["alloc", …]`; `crates/goad-semantics/Cargo.toml:17` (stratum 1 does carry
`jiff`); `design.md:1512-1570`; `docs/policy/001-the-phase-gate.md` §Verification.

**Disposition:** _pending_

**Outcome:** _pending_

### F-55 — A `Slider`'s `released` flush has no callback to arrive on, and is in neither enumeration of how an entry leaves `pending.rs`

**Severity:** major
**Location:** `design.md §5.1 pending.rs; §5.2`, the controls table and the Slint
block; §5.3 ownership; §7 D7
**Raised by:** round 4 (fresh Claude agent)

**Expected:** A behaviour the decisions table commits to has an interface it can
travel on, and the enumerations that say how pending state is emptied include it.
**Observed:** D7 and §5.2's controls table both commit to it: a `Slider` binds
`changed`, *"debounced; `released` flushes"*, so *"a drag's final value does not
wait on a timer."* Three things are missing for it.

The markup declares exactly one host-ward callback for a field —
`callback edited(string, string, string, FieldEdit)` — and `FieldEdit` carries
`kind`, `checked`, `text`, `number`, `index`, `date`, `time`. Nothing in that
surface distinguishes *record this and debounce it* from *record this and send it
now*, and the debounce map lives in Rust behind an `Rc` that only `install.rs`'s
closures reach, so the markup cannot flush it by itself. As declared, `released`
can only call `edited` again, which restarts the timer rather than flushing it —
the opposite of the promise.

§5.1 then says *"The two ways an edit leaves `pending.rs` are not symmetrical"*
and names the timer and the `Choose` drain; §5.3's ownership row gives the
lifetime as *"until the send that carries the entry is enqueued: the timer's, one
per tick, or the `Choose` `chosen` drains them all into."* Both are stated as
closed and neither admits a third exit. §5.5's edges table has no row for it and
§9 has no obligation for it, so nothing would discover the gap: a `released` that
silently restarts the debounce instead of flushing it looks exactly like a
working drag to every case in the table.

This is the same shape as `compose`'s four steps and `interpret`'s three cases,
which §5.2 states in full for the reason it gives — a case left off the list
becomes an `unwrap` in the implementation. Here a path left off the list becomes a
mechanism nobody writes.

**Evidence:** `design.md` §5.2's Slint block (the one `edited` callback and
`FieldEdit`'s fields), the controls table's `number, slider admissible` row, and
the `Slider` paragraph; `design.md` §5.1, *The two ways an edit leaves
`pending.rs`*; §5.3's pending-edits ownership row; §7 D7; §9's obligations table;
`widgets/common/slider-base.slint:114-131` (`released` is raised by the pointer
and keyboard paths only, which is why D7 binds `changed` as well).

**Disposition:** _pending_

**Outcome:** _pending_

### F-56 — §9's AC-2 row inherits two unproven popup capabilities and names neither fallback

**Severity:** minor
**Location:** `design.md §9`, the AC-2 row
**Raised by:** round 4 (fresh Claude agent)

**Expected:** A row whose driver depends on a capability §9 records as unproven
carries the same fallback the row that introduced the dependency carries. §9's own
rule: a row whose driver does not exist in its tier moves rather than being
written where it would be green.
**Observed:** AC-2's driver is *"the driver above for each control"* — all five,
including the two that need a popup under `init_no_event_loop`. §9 records both as
unproven there and gives each a fallback in the row that introduces it: AC-8's
`ComboBox` needs `mock_single_click` on a laid-out popup, which §8 R9 says no case
in this repository has yet produced, *"the row moves to the loop tier"*; and the
picker chain's row says *"If the popup cannot be found under `init_no_event_loop`
the row moves to the loop target."* AC-2 depends on both and says neither. If
either fallback fires, AC-2 has to move too, and nothing in the table records
that.

`slice-009.md`'s AC-2 could in principle be satisfied from as-drawn values alone —
an untouched `choice` already submits the first alternative's id and an untouched
`datetime` already submits an RFC 3339 epoch — which would be a materially weaker
case than the one the row's driver describes. That ambiguity is the second half of
the finding: the row does not say which it is.

**Evidence:** `design.md` §9, the AC-2 row, the AC-8 row, the *a picked field's
picker re-seed* row, the *One control in that table needs a pointer* paragraph;
`design.md` §8 R9; `slice-009.md` AC-2.

**Disposition:** _pending_

**Outcome:** _pending_


## Probed and sound — round 1

- The five as-drawn choices themselves do not breach R-35: R-58 requires a value for every drawn field, while R-35 leaves answer validity to the backend. In particular, `0` for a max-only number may be outside the stated range without authorizing the host to refuse the answer (`SPEC-001/R-35`, `R-58`; `design.md:229-239`). The representation defects are raised separately in F-2 and F-3.
- Choice identity is type-directed as claimed: `AlternativeId::new` is not available to `crates/goad`, alternatives are non-empty and unique, and resolving a reported index against the drawn presentation preserves the alternative/option namespace split (`crates/goad-semantics/src/protocol/canonical.rs:349-376`; `design.md:220-227`; `SPEC-001/R-52`). Slint's `ComboBox` exposes `current-index`, so its string-valued `selected` callback does not prevent the wrapper from reporting that index (locked Slint 1.17.1 `widgets/common/combobox-base.slint:10-30`).
- The split-channel slot mechanism has measured evidence: replacing the flat values property preserved element instances and the numeric empty-string guard was negative-controlled (`research.md:182-205`; `spike-fields/tests/split.rs`; `spike-fields/tests/numeric_guard.rs`).
- The broad tier boundary is correct: every existing `tests/renderer/` case initializes with `init_no_event_loop`, where `changed` was measured not to run, while `event_loop` and `event_loop_schedule` are separate one-test binaries suitable for real-loop observations (`crates/goad/tests/renderer/harness.rs:51-56`; `crates/goad/Cargo.toml:35-45`; `research.md:207-224`). F-11 is about missing cases inside that otherwise-correct assignment.
- Keeping an empty `FieldForm` can preserve the sixth-kind compile stop: the exhaustive `FieldKind` mapper must classify a new canonical variant, and an empty enum keeps the undrawn destination in the model. Existing constructions necessarily need rewriting, but the core compile-time mechanism is sound (`crates/goad/src/view_model.rs:131-155,218-227`; `design.md:114-130`). F-10 concerns the unenumerated consumers and stale verification record, not that mechanism.

## Probed and sound — round 2

- The first ground of F-7 is indeed wrong: `next_seq` is `u64`, so reaching `saturating_add`'s fixed point would require a non-terminating number of issued views. The surviving caller-precondition account matches the public `Prepared`/`Frame` surface (`goad-shell/src/state.rs:25,64-81`; `design.md:527-538`).
- The F-12 A/B trace is now stated correctly. `serve` handles an `Edit` synchronously and returns to `present` before receiving another command; exchange commands present `busy` before awaiting (`controller.rs:661-675,738-744,789-822`; `research.md:334-346`). F-23 concerns only the new, independent identity rationale.
- The second totality exception itself holds: row setters mutate local retained state infallibly, the row model's only writer is `present`, empty frames clear the retained id, and the id is committed at the end. A same-id structural collision remains the explicit caller precondition rather than an unacknowledged exception (`glass.rs:20-35,72-76`; `design.md:401-432,480-484`).
- On the primary v0 platform, the repaired Jiff feature pair is sufficient: Linux system-zone discovery uses `tz-system`, and the global database reads `/usr/share/zoneinfo` under `tzdb-zoneinfo`. `tzdb-bundle-platform` is needed for Windows, whose support is not a v0 objective (`docs/brief.md:193-197`; locked Jiff 0.2.35 `tz/timezone.rs:306-323`, `tz/db/mod.rs:94-110`).
- The five as-drawn choices still satisfy R-35/R-58, and the untouched number remains an `f64` entirely on the host path: only an edit crosses Slint as a string. CD-1 now exposes the max-only consequence (`design.md:319-364`; `canon-delta.md:19-31`; `SPEC-001/R-35`, `R-58`).
- Carrying `slider: bool` is a genuine host decision rather than a downstream inference: the declared markup row receives the selected control directly, and neither `Edited` nor the wire mapping branches on bounds. F-20 is about the current selector's insufficient admissibility predicate, not this separation (`design.md:189-247,280-283`).
- The split structure/value channel and same-view in-place mechanism remain supported by the measured spike: value replacement preserves element identity, and the epoch makes correction observable. The newly found defects are in particular comparands and write order, not in the two-channel premise (`research.md` Thread 3; `spike-fields/tests/split.rs`, `numeric_guard.rs`).
- Keeping `FieldForm` empty preserves the sixth-kind exhaustive-match stop. F-10 remains open because the repair did not enumerate every current consumer or both R-58 cases, not because the empty-enum mechanism is unsound (`design.md:130-155`; `view_model.rs::undrawn_form`).
- D-13 accurately describes Jiff's fold/gap behavior: `to_zoned` uses `Compatible`, selecting the earlier fold and shifting a gap forward. F-14 is confined to the separate boundary-error `Result` (`design.md:387-399`; locked Jiff 0.2.35 `civil/datetime.rs:1450-1472`).

## Probed and sound — round 3

- The lossless numeric path is correctly split at the markup boundary: a numeric `LineEdit` reports text, while only a slider whose endpoints round-trip exactly receives `f32` bounds (`design.md:262-291`). F-20 concerns operability of the slider predicate, and F-26 the locale parser, not the removal of the original all-`f32` narrowing.
- Once a report has been resolved, the draft's number really is canonical: `Finite` has a private field and fallible constructor, and `Edited::Adjusted` is the only numeric draft variant (`design.md:463-497`). F-42 is confined to the stronger, false claim about `Reported`.
- Choice identity is preserved by construction after resolution: the callback reports an index, `resolve` clones the `AlternativeId` from the retained `DrawnKind`, and `AlternativeId::new` is unavailable to `crates/goad` (`design.md:516-555`; `crates/goad-semantics/src/protocol/canonical.rs:349-376`).
- The `compose` failure account matches locked jiff: checked `Date::new`/`Time::new` are fallible, and `DateTime::to_zoned` returns an error near representational boundaries while resolving folds/gaps with `Compatible` (`jiff-0.2.35/src/civil/date.rs:245`, `time.rs:278`, `datetime.rs:1450-1472`; `design.md:608-646`). F-29 concerns the separate impurity count.
- The datetime popup lifetime and binding direction are sound: locked generation constructs a new popup on each show and the closed instance leaves `active_popups`; the measured root-property binding avoids the enclosing-window assignment error (`design.md:839-867`; `research.md:289-318`). F-44 concerns the missing durable validation row.
- The second exception to `Glass::present` totality is bounded to retained row structure whose only writer is `present`, and the values → rows → epoch order is stated consistently in lifecycle and I-F (`design.md:713-727,793-807,895-901`; `crates/goad/src/glass.rs:20-35`).
- The control-driver inventory matches the locked widget/testing APIs: `LineEdit` and `Slider` have accessible set-value paths, `ComboBox` requires a pointer click on a `ListItem` with no default action, and picker day/OK controls do expose default actions (`design.md:1019-1045`; `i-slint-backend-testing-1.17.1/search_api.rs:952-974`; `widgets/fluent/components.slint:49-53`). The real-loop boundary for property change handlers and timers is also supported by the measured tier (`research.md:208-225`).
- The current `FieldForm` consumer class is enumerated accurately: the source/docs, mapper cases, three wiring assumptions and the fields fixture found by the tree search all appear in `design.md:151-178`; CD-2 correctly identifies the stale R-57, R-58 and R-55 verification text (`canon-delta.md:61-101`). F-45 is only the unexplained additional R-16 mention.
- The jiff feature argument matches both authorities: `tz-system` and `tzdb-zoneinfo` each enable `std`, `TimeZone::try_system` is an unconditional error without `tz-system`, and POL-001 explicitly leaves shared-dependency features as review-only residue (`jiff-0.2.35/Cargo.toml:69-127`, `src/tz/timezone.rs:325-400`; `docs/policy/001-the-phase-gate.md:120-143`; `design.md:1122-1177`).
- CD-1's as-drawn values all satisfy the existing R-57 JSON types and R-58 totality; the max-only `0` consequence is now explicit (`canon-delta.md:14-39`). Per the review instruction, the open normative-versus-descriptive status of the datetime epoch is left to the user.

## Probed and sound — round 4

Each of these was re-derived from the locked source or the tree this round, not
read off a Response. Rounds 2 and 3 checked much of the same ground; where that
is so, what is recorded here is that the **integrated text** still says what the
source says, which is where the three known-bad citations were introduced.

- **The `resolve` constraint is stated correctly and is already in the gate.**
  `structure.rs:25` sets `SUBJECT_DIR = "crates/goad/src"`, and
  `no_production_line_in_the_renderer_names_the_identifier_resolve` asserts the
  file set is non-empty before asserting the absence. `scan::mentions`
  (`goad-boundary/src/scan.rs:225-234`) is exactly as §5.2 describes it: comments
  cut by `code_of`, string literals **kept** (`:275-277`), split on every
  non-ASCII-alphanumeric byte, then on camel boundaries, matched
  singular-or-plural. `resolve`, `resolves`, `resolve_index` and `Resolve` all
  trip it. `interpret` is clear of the domain-vocabulary list
  (`vocabulary.rs:18-25` — habit, streak, journal, site, goal, reminder,
  compliance, over `.rs` **and** `.slint`, so the new markup names are clear too),
  of the purity path list (`purity.rs:17-27`, whose subject is stratum 1 in any
  case), and of `structure.rs`'s three call-form greps (`quit_event_loop(`,
  `tokio::spawn`, `slint::spawn_local(`).
- **The timer may re-arm from inside its own callback.**
  `start_or_restart_timer` copies `being_activated` forward from the existing
  timer, and `maybe_activate_timers` re-emplaces the old callback only where the
  permanent store is still `Empty`, with the comment saying why
  (`i-slint-core-1.17.1/timers.rs:330-334`, `:348-360`). §5.1's whole re-arm rests
  on this and it reads as stated.
- **The slider arithmetic.** `increment()` is `root.set-value(root.value +
  root.step)`; `set-value` returns immediately when `root.value == value` and
  otherwise clamps into `[minimum, maximum]` and raises `changed`;
  `key-pressed` returns `reject` when `step <= 0`
  (`widgets/common/slider-base.slint:78-80,117-131`). `accessible-value-step` is
  `min(root.step, (root.maximum - root.minimum) / 100)`
  (`widgets/fluent/slider.slint:29`). F-20's operability clause and F-32's
  corrected gloss both hold against the source.
- **§9's principal driver really does raise an edit.**
  `ElementHandle::set_accessible_value` dispatches
  `AccessibilityAction::SetValue` (`i-slint-backend-testing-1.17.1/search_api.rs:637-644`),
  which invokes `accessible-action-set-value(v) => { text = v; edited(v); }`
  (`widgets/fluent/lineedit.slint:16` — the line the design cites, exactly). The
  `Slider`'s accessible actions route through `base.set-value` / `increment` /
  `decrement` (`fluent/slider.slint:30-36`), which raise `changed` and not
  `released`, which is D7's whole ground.
- **The guard's convergence write cannot re-enter `pending.rs`.** This is the one
  path that would have falsified §5.3's *Nothing re-enters*: a present bumps the
  epoch, the guard assigns `self.text`, and if that assignment raised `edited` the
  callback would write the map while `present` was reading it. It does not.
  `LineEditBase` raises `root.edited` only from `TextInput`'s own `edited`
  callback (`widgets/common/lineedit-base.slint:138-139`), and `TextInput` calls
  that from the key-insertion path and from `insert` — never from a property
  write. Fluent's clear (✕) icon is a third user-driven caller
  (`fluent/lineedit.slint:62-67`) and is an ordinary edit, not a re-entry.
- **The chained pickers never have two popups open at once.**
  `DatePickerPopup`'s OK does `root.close()` **before** `root.accepted(...)`
  (`fluent/datepicker.slint:80-96`), so the date popup is gone by the time the
  host's handler shows the time popup — which matters because `close-policy:
  no-auto-close` would otherwise leave both in `active_popups` for §9's element
  queries to walk.
- **The picker seed reaches both the display and the returned value.**
  `DatePickerBase` has `in property <Date> date` defaulting to today,
  `property <Date> current-date: root.date`, `selected-date <=> root.current-date`
  and `get-current-date()` returning `current-date`
  (`widgets/common/datepicker_base.slint:268,277,403,461-463`). D21's seed is not
  a property the widget ignores, and the *leaving it opens on today* rejection is
  literally the default binding.
- **`Controller::edit` reaches an option that is not the one being answered.**
  It is `&mut self`, takes `option` as an argument, and resolves it through
  `selected(prepared, view, option)` against the retained presentation
  (`controller.rs:259-279`). §5.2's carried-edit rule — *a person can type into
  one option's field and then answer another* — is applicable through the walk it
  names, which is what F-46 was about.
- **`DrawnKind::Choice` carrying the first id is not working around something the
  canonical type could answer.** `Alternatives` exposes only `as_slice() ->
  &[Alternative]` (`canonical.rs:349-376`), so `.first()` is genuinely an
  `Option`; `Alternatives::new` rejects the empty list at `:360-364`. Giving
  `Alternatives` a total `first()` would be the cleaner repair and is out of scope
  by `slice-009.md`'s own *nothing reaches `goad-semantics`*, so §5.2's
  enumeration of the ways out is complete for this slice.
- **§5.1's `FieldForm` consumer enumeration covers every occurrence in the tree.**
  `FieldForm` appears in four files; every occurrence in `diagnostics.rs`,
  `draft.rs` and `tests/renderer/mapper.rs` falls inside a range the table names,
  and `Undrawn`'s other consumers (`reception.rs:84`,
  `tests/renderer/reception.rs:296,492`) name no `FieldForm` at all. What the
  table does not name is `view_model.rs`'s own definition, its `Display` impl, and
  `undrawn_form` / `sift` — which are the change rather than consumers of it, and
  every one of which fails to compile rather than lying. Worth knowing when the
  plan prices it: the `Display` impl's arms go with the variants.
- **The jiff feature costs are as §10 states.** `tz-system = ["std",
  "dep:windows-link"]`, `tzdb-zoneinfo = ["std"]`, `std = ["alloc", "jcore/std",
  …]` (`jiff-0.2.35/Cargo.toml`), and `goad-semantics` does carry `jiff`
  (`crates/goad-semantics/Cargo.toml:17`), so the residue is real rather than
  hypothetical. F-54 is about what the argument omits, not about these numbers.

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

# Audit & reconciliation — Slice 009

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## PARTIAL — session 1 checkpoint (2026-09-20)

**Read this before anything else.** The audit is part-done. This section is the
state of it; delete it when the audit closes.

### What is finished

- The **Brief** below, written before any evidence was gathered (`aaeb5bf`).
- **Evidence**: the gate, the surface delta, stratum purity, and PHASE-04/VA-1's
  residue argument. The AC table and the VT/VA/VH walk are **not** done.
- **`review-code.md` round 1, two of three dimensions**: the pure layer and the
  suite. Seven findings — one blocker, two majors, two minors, two nits.
- **Four user decisions**, recorded in `audit-log.md` with their reasons.

### What is outstanding, in the order it should be taken

1. **The third review dimension has not reported.** `review-renderer` was still
   running when this session wrapped: shared mutable state across the Slint
   callback, the timer and the async serve loop; the guard and write order;
   handle splitting; the markup's per-control bindings; what a person loses and
   when; resource behaviour. **Re-run it as a fresh agent** — do not assume the
   surface is clean because no findings arrived. Its brief is in
   `review-code.md` §Brief. One question was put to it and not answered: what a
   `ComboBox` with its popup open, and a `datetime` `Button` with a picker
   open, cost when `busy` goes true mid-interaction. F-A1 has the other five
   of the seven `enabled: !root.busy` sites priced.
2. **F-S3 and the suite dimension's AC table** were truncated in transit and
   were not recovered. F-S3's claim, as far as it arrived: *AC-7's mechanism is
   held by no instrument in the gate* — `view_model.rs:317` (`drawn_form`) and
   the case at `:889`. Re-derive it rather than trusting this summary.
3. **The repairs**, all four decisions in `audit-log.md`. Then
   **`review-code.md` round 2 over the repairs themselves** — `docs/memory/`
   records that the rounds on the repairs are about half the total cost, so
   budget for them rather than treating round 2 as a formality.
4. **Reconciliation and Closure**, neither started. The rows already known are
   listed under Reconciliation below.

### The budget, revised

The handover that opened this audit planned two sessions and said a third would
be needed **if the slider became a repair rather than a follow-up**. It did —
the readout half is landing in this slice (`audit-log.md`). Plan **three**: this
one, one for the repairs and round 2, one for reconciliation and close.

### Two things not to rediscover

- **`git status` is clean and the suite is green at 592.** Two mutations were
  applied and reverted during this session to verify F-S1 and F-S2
  (`pending.rs`, `glass.rs`); both were restored from copies and the tree was
  checked clean afterwards. No mutation is in the tree.
- **`Alternatives::first` cannot panic**, **`R-58`'s MUST holds structurally**,
  **I-F's write order is correct**, and **`pending.rs`'s map is bounded and its
  timer terminates**. Each was checked against the code rather than the doc
  comment. Do not re-derive them.

## Brief

**Subject:** `a698217..HEAD` on `main` — 107 commits, 35 files and +7535/-595
under `crates/`, 19 under `docs/`. `a698217` is the pre-slice spike.

**Question.** For this slice to be finished, three things must hold that a
green gate does not establish: every drawn kind submits what `R-57` types and
nothing else reaches the wire; the mechanism that made a present harmless is
actually harmless to *a person*, not only to an element counter; and the
record — canon included — is true about a host that now draws all five kinds.
This audit checks those three, and the lines of attack below are chosen
because each is a place where the gate is structurally incapable of
disagreeing with them.

Written before looking. One inherited result is assumed rather than
rediscovered: **AC-5 is unmet** (VH-1, `notes.md` PHASE-09 sheet). This audit
does not re-establish it. It asks how wide the class is.

### Lines of attack

1. **The `busy` class, beyond the slider.** VH-1 landed one instance: a
   `TouchArea` disabled mid-press loses its grab. `busy` reaches seven
   `enabled: !root.busy` sites. A disabled `ComboBox` popup, a disabled picker
   mid-chain, a `LineEdit` disabled mid-keystroke — each is the same defect
   with a different cost, and only one of them has been priced. Fix the class,
   not the instance means knowing the class first. Whether any of them reaches
   **AC-4** (a dropped character) rather than only AC-5 decides the severity.

2. **What each AC's named case would survive.** The project's recorded failure
   mode (`docs/memory/a-green-test-can-assert-a-proxy.md`, and slice 004's four
   green tests) is a case that asserts a proxy. VH-1 found one — `fields.rs:558`
   drives `set-value` for a drag. The audit asks the same question of every
   AC-bearing case, and in particular of AC-5's own instrument: `reasserts` and
   `inits` cannot observe a pointer grab, so the criterion and the thing
   measuring it were never the same claim.

3. **A backend input that panics the host.** `Alternatives::first` carries an
   `expect` and a `# Panics` section, admitted by a lint exception, resting on
   an invariant normalization is supposed to hold. *A backend failure never
   takes the host down* is one of the five. Either normalization refuses a
   `choice` with no alternatives before that call is reachable, or the
   exception is a crash on input. This is checked against the normalizer, not
   against the doc comment.

4. **Narrowing the wire to fit the renderer.** The invariant the project exists
   to hold, applied in its inverse direction: this slice deletes the renderer
   subset, so the question is no longer what is undrawn but what is *quietly
   refused*. `slider_bounds` answering `None`, `Finite` refusing a non-finite,
   `f64`→`f32` at the markup boundary (§8 **R7**, two found in one round — a
   class, not two accidents), an `f32` index, a `max`-only range. Every legal
   `R-16`/`R-17`/`R-18` field must still draw and still answer.

5. **Unbounded host state.** `pending.rs` is a map keyed by (option, field)
   whose entries leave on enqueue, with a timer that re-arms while it is
   non-empty. A backend presenting fast, or a full channel, are both host-side
   growth conditions the design prices in prose. Also: whether the timer stops.

6. **Undeclared paths.** The strongest lead per `AGENTS.md`. Diff the paths
   actually touched against each phase's declared Surfaces, both directions.
   `examples/shell/backend.sh` is already known to be stale and outside every
   phase's Surfaces (`notes.md`); the question is what else is.

7. **Whether CD-1 and CD-2 are true of the tree.** CD-1 states five per-kind
   untouched values and one consequence; CD-2 claims three Verification rows
   went false, one of them *permanently* reducing what the suite can assert.
   Both are checked against the code and the suite independently of PHASE-09's
   EX-9, which is the same agent marking its own work.

### Invariants held to

The five in `CLAUDE.md`, in full — the domain-vocabulary boundary, permissive
wire against canonical internals, no renderer-driven narrowing, no backend
input that downs the host or leaves a refusal unattributed, and one-way strata.

**SPEC-001** R-16, R-17, R-18, R-35, R-52, R-53, R-55, R-57, R-58 — the set
`slice-009.md` declares binding, each walked rather than cited.
**POL-001** — and specifically its §Verification boundaries: the gate is four
ADR-001 instruments plus the vocabulary scan plus one residue nothing enforces
(**R8**, the `jiff` feature). The residue is held by this audit's reading,
because nothing else holds it.

### Out of scope, and why

The three upstream `std-widgets` defects (VH-1 lead 4). Not this project's, and
recorded as such. They are not re-litigated here; whether the project should
own its own pickers is a slice, not a finding.

## Evidence

### Tests and checks

`just check` — **exit 0**, run at `b1f9de4`. **592** tests across 25 targets,
matching the total PHASE-09 recorded. The gate is build, both test tiers, the
`deno check` of `examples/typescript/backend.ts`, `cargo clippy --workspace
--all-targets -D warnings`, and `cargo fmt --all --check`.

What that does **not** establish is stated once, here, rather than implied by
the number. `POL-001` §Verification names the boundaries: the gate is four
ADR-001 instruments plus the domain-vocabulary scan plus **one residue nothing
enforces** — a feature switched on in a dependency stratum 1 shares. This slice
takes that residue (`jiff`'s `tz-system` and `tzdb-zoneinfo`), and
`design.md` §10 carries the argument `POL-001` requires in place of a check.
Read and accepted: the argument is complete, it measures rather than predicts
what the feature does and does not gate, and it correctly narrows rather than
retires `clock.rs`'s workaround. **PHASE-04/VA-1 discharged.**

### Surface delta

Every path touched under `crates/` is inside some phase's declared Surfaces,
with two exceptions, both trivial and neither a lead:

- `crates/goad/tests/renderer/main.rs` — undeclared, +3/-1, module declarations
  for nothing but the files that *were* declared.
- `Cargo.lock` — undeclared, and a consequence of the `Cargo.toml` lines that
  were.

`crates/goad-semantics/src/protocol/canonical.rs` is **stratum 1 and was
touched**, which reads as a breach until the plan is consulted: PHASE-09's
Surfaces carry it explicitly, *"`Alternatives::first` alone — added 2026-09-19
with explicit user endorsement"*. The path is declared and the change is
sound. What is stale is `slice-009.md` §Governing canon, which still reads
*"all of this slice is stratum 3; nothing reaches `goad-semantics`"* — see
Reconciliation.

Declared and untouched: `crates/goad/tests/renderer/table.rs` (PHASE-01) and
`tests/support/` (PHASE-01, conditional on a helper being extracted — it was
not). Neither is dropped work.

`spike-fields/` is deleted, which D-19 required at design close.

**The undeclared path that matters is one nobody touched.**
`examples/shell/backend.sh` is outside every phase's Surfaces and is now false
about the running product in three places: its header says the form *"carries a
field of a kind this renderer does not draw, on purpose"* (`:15`); its `note`
field is annotated *"it will not appear in the window"* (`:101`) and *"its id
will be absent from the `values` recorded above"* (`:102`). All three were true
when written and none is now. The last is not stale prose — it describes
runtime behaviour a person running `just demo` will watch the host contradict.
The gate cannot see it: the example typecheck covers
`examples/typescript/backend.ts` and nothing covers this file.

### Stratum purity

`cargo test -p goad-semantics` builds and passes as part of the gate.
`Alternatives::first` is pure — it names no `src/shell/`, reads no clock,
filesystem or subprocess, and adds no dependency. ADR-001 holds.

### Acceptance criteria

<!-- filled from the human run, the wire log, and review-code.md's suite
     dimension -->

### Verification criteria

<!-- VT / VA / VH per phase -->

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Findings are not restated here.

- **Ledger:** `review-code.md`
- **State:** open · round 1 of at least 2, two of three dimensions reported
- **Outstanding blockers:** **F-A1** — dispositioned *fix now* by the user
  (`audit-log.md`), not yet repaired.

Round 1 raised seven: one blocker, two majors, two minors, two nits. Both
majors were **mutation-confirmed by the audit rather than accepted on the
reviewer's report**, which is the standard `docs/memory/` asks for — a claim
about the tree can go stale under the agent that made it.

## Verdict

<!-- The slice's closure story, written once, here. Draws on the ledger's
     synthesis and on the evidence above; restates neither. Does this slice do
     what it set out to do, and what is being accepted knowingly? -->

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

**Not started.** The rows below are the divergences this session established;
each still needs its change written and applied. Endorsement for the canon rows
is recorded in `audit-log.md`, except where the `done` column says otherwise.

| document | change | reason | done |
|----------|--------|--------|------|
| `canon-delta.md` CD-2 Change 3 | `undrawn_form` → `drawn_form` | cites a function PHASE-05 renamed; promoting it verbatim writes a dangling citation into canon | [ ] |
| `specs/001 §Verification`, `R-58` row | name the surviving half of the MUST NOT, and say the other half is held by the shape of `answer`'s walk | **canon is untrue now**: it cites two cases `24e8e82` deleted | [ ] |
| `specs/001 §Verification`, `R-57` row | retire *"review, not a test"*; name the new cases; and correct the closing citation from `present` to `drawn_form` | the premise — no renderer draws those kinds — expired this slice. The citation drift is CD-2's fourth change and CD-2 does not carry it | [ ] |
| `specs/001 §Verification`, `R-55` row | drop the sentence naming option fields; say where the sixth-kind path is held | no option field is undrawn on account of its kind any more | [ ] |
| `specs/001 §Verification`, `R-18` row | confirm or correct *"`view_model.rs::present` reads exactly one key"* | the read moved to `Run::of` (`view_model.rs:250`); same call tree, so possibly still true. **Unverified — check before touching** | [ ] |
| `specs/001 §7` | CD-1, **descriptive**, plus a clause for the cleared bounded `number` | `R-58` forbids omitting a value and `OQ-2` is unlanded, so the host must supply one; F-P2 shows the cleared case is stated nowhere | [ ] |
| `specs/001 §8` OQ-4 | evergreen replacement of *"no evidence asks for one yet"* | **user discussion open, not decided.** The fork is asymmetric: `R-18` already admits the hint half with no protocol change | [ ] |
| `slice-009.md` §Governing canon | *"nothing reaches `goad-semantics`"* is false | PHASE-09 touched `canonical.rs` with explicit endorsement, declared in `plan.md` | [ ] |
| `examples/shell/backend.sh` | three statements, `:15`, `:101`, `:102` | all three went false this slice; the gate cannot see this file | [ ] |
| `docs/roadmap.md` §Open decisions | record that OQ-4 stays shut and why | `design.md` §10 names this as owed at close | [ ] |
| `plan.md` §Coverage, `design.md` §9 AC-4 row | name a case for AC-4's element half that can observe a present | F-S1: the named case is vacuous; four others do hold the mechanism | [ ] |

**Design drift not reconciled:** not yet written. The candidates this session
found, each to be confirmed before it is recorded — `design.md` §9's AC-7 row
and D11 cite `undrawn_form`; §9's driver table and §8 **R9** are wrong about
the `ComboBox` and R9's mitigation is unavailable (PHASE-09/VA-1 has the
measurement); `:849-852` argues against a lint exception on a premise PHASE-09
removed, and one landed in `goad-semantics` instead; **A-6**'s *"costs focus,
not data"* is falsified by F-A1; and I-H's divergence list is missing both
F-P1's rounding and F-P2's cleared number.

## Closure

- [ ] All findings dispositioned; no blockers outstanding
- [ ] All acceptance criteria met, or explicitly waived by the user
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

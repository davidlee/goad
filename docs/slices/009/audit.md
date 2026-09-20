# Audit & reconciliation — Slice 009

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## PARTIAL — session 3 checkpoint (2026-09-20)

**Read this before anything else.** The audit is part-done. This section is the
state of it; it replaces session 2's, and it is deleted when the audit closes.

### What is finished

- The **Brief** below, written before any evidence (`aaeb5bf`).
- **Evidence**: the gate, the surface delta, stratum purity, PHASE-04/VA-1's
  residue argument, and the AC table. The VT/VA walk is **not** done.
- **`review-code.md` round 1, complete** — twenty-one findings, every one
  dispositioned with the user (`audit-log.md`, second entry).
- **Nineteen of twenty-one findings repaired**, across six commits, each
  gate-verified by this session running `just check` itself:
  - `db1d702` **F-R1** — an open picker is dismissed when the shown view
    changes. Injection pass B1/B2/B3 re-run against the committed tree.
  - `d9fe587` **F-R5, F-R8, F-S7, F-P1, F-R7, F-R9, F-P4, F-R6, F-P3** and the
    ledger's un-raised Note.
  - `4f0af74` **F-S2** (new loop target `event_loop_full/`), **F-S6**, **F-S1**.
  - `da4ced0` **F-S5** — the guards assign through counting functions.
  - `511d307` **F-S4** — PHASE-05's injection pass, run at last, all four red.
  - `faccd95` **F-S3** — `clippy::wildcard_enum_match_arm` denied for `goad`.
- **Ledger Responses written for all nineteen.** **Outcomes are blank
  throughout and that is deliberate** — see *The Outcome split* below.
- **Gate green at 597**, `just check` exit 0, zero warnings, tree clean at
  `faccd95`.

### What is outstanding, in the order it should be taken

1. **Two findings, both blocked on a user decision, neither started.**
   - **F-R4** — *one full present per refused ingress arrival, at a rate an
     untrusted writer sets.* **There is no repair that is not a design
     change**, and this was established rather than assumed: `option_models`
     computes each field's slot while building `values`, so splitting it
     duplicates the walk — the parallel implementation `CLAUDE.md` forbids —
     and `show()` on an already-visible window is the *totality* argument at
     `glass.rs:33-35`, not an oversight. What is actually open is **whether a
     refused arrival should present at all**; it currently does, because the
     refusal folds onto diagnostics and the present is what displays it. Put
     to the user, unanswered: re-disposition to `follow-up`, or take the
     design decision now.
   - **F-P2** — dispositioned `doc-wrong`, and its repair edits `design.md`,
     a record of intent. Its argument that this is not retro-fitting is sound
     (the three sentences are residue of D-16, which D-33 already reversed)
     and CD-1's cleared-number clause is already decided (`audit-log.md`), but
     **explicit endorsement was asked for and not given**. Do not write it
     without.
2. **`review-code.md` round 2, over the repairs.** Not started. **Four raises
   are already waiting**, each disclosed by the work rather than found by
   review, and none folded in quietly:
   - **`Focus::Diagnostics` leaves a picker up.** `Surface` derives from
     `(focus, shown.is_some())` (`controller.rs:162-167`), so the mode switch
     to `WindowMode::Diagnostic` empties the prompt block without changing
     `shown` — F-R1's dismiss branch is not taken. Outside F-R1's two measured
     shifts; the case for it is one more step in `event_loop_picker`.
   - **`wiring.rs::busy`'s two cases engage an `Answer` and fold an
     `Evaluation`**, a sequence that cannot happen after the narrowing. A green
     case whose setup is impossible is `docs/memory/tests-asserting-proxies.md`'s
     shape, and it sits inside the repair that closed the blocker. **Raise or
     clear it deliberately; do not inherit session 2's reasoning for leaving it.**
   - **`set_diagnostic_lines` allocates a fresh `VecModel` on every present**
     (`glass.rs:257-259`), so the diagnostics repeater rebuilds its whole list
     every time. **This is F-R5's exact class, one surface over, and F-R5's
     repair did not touch it.**
   - **The four `goad-shell`/`goad-semantics` wildcard matches** F-S3's repair
     deliberately did not cover. Not a defect — each chooses no behaviour from
     the variant — but the reasoning is in a comment in `goad/src/lib.rs` and
     nothing holds it.
3. **Re-walk AC-4 and AC-5.** The AC table still reads both **NOT MET**. That
   was true when written. Re-walk against the repairs — **do not edit it on the
   strength of them**.
4. **Reconciliation and Closure**, neither started. Eleven rows below, none
   applied; *Design drift not reconciled* unwritten.

### The Outcome split, agreed with the user this session

No finding has an `Outcome`, and the ledger is not done until every one is
`verified` or `withdrawn`. The original raisers were three agents that no
longer exist. The user's decision:

- **Round 2's reviewers set Outcomes** for the findings that produced real code
  — F-A1, F-R1, F-R2, F-R3, and this session's repair block. A fresh
  adversarial eye over the work the Responses describe writes `contested` with
  evidence rather than an opinion.
- **The orchestrator sets them as declared raiser** for the four `doc-wrong`
  and the nits, where there is no code to attack — saying so in the ledger.

### What not to rediscover

- **A finding can correctly identify that nothing holds a property and still be
  wrong about what would hold it.** This hit three times: **F-S6**'s
  channel-filling case drives F-S2's claim and not its own (at capacity 1 a
  `tick` iterating the whole map is indistinguishable from one-per-tick);
  **F-S5**'s approved observable-state case is unreachable, because the
  observable is the caret and no tier can see one; **F-S3**'s `trybuild` closer
  cannot express the property at all, since the hole is a wildcard that *does*
  compile. In each the disposition stood and the repair changed. **Verify a
  proposed instrument before building it**, exactly as a finding's claim about
  the tree is verified.
- **`trybuild` was authorised by the user and deliberately not fetched**, for
  the reason above. No dependency was added.
- **F-S3 cost no canon.** It is a lint-table entry inside the gate's existing
  clippy pass, not a fifth boundary instrument, so **POL-001 is untouched and
  the gate's instrument count is unchanged**.
- **597 is a sum over 28 `test result: ok` lines, not 597 distinct cases** —
  `just check` runs `cargo test --workspace` then `cargo test -p goad-semantics`,
  double-counting the pure tier. The inherited 592/595/596 have the same
  property. State it once in Evidence rather than letting the number imply a
  census.
- **F-S5's repair is measured as a pair**: under `root.reasserts += 1` beside
  the write, the finding's own mutation leaves `event_loop_reassert`,
  `_overlay` and `_numeric_guard` all three green; under the counting call
  `reassert.rs:298` goes red with `reasserts: 2` against `0`.
- **The ledger's un-raised Note was half wrong.** Its conclusion holds — the
  citation wants correcting — but both cited line ranges are accurate. The real
  hazard is that `TimerMode::SingleShot` and `CallbackVariant::SingleShot` are
  different types taking different arms: `Timer::start` boxes every callback as
  `MultiFire` whatever mode it is given (`timers.rs:84-93`).
- Still standing from earlier sessions, each checked against the code rather
  than a doc comment: **`Alternatives::first` cannot panic**, **`R-58`'s MUST
  holds structurally**, **`pending.rs`'s map is bounded and its timer
  terminates**, **no `enabled` binding is observable through the accessibility
  surface**, and **slint 1.18.0 fixes none of this slice's defects**.

### The budget

Four sessions planned; this is the third and it did not overrun. **Session 4
takes round 2, the AC re-walk, reconciliation and close** — and
`docs/memory/audit-stage-needs-its-own-budget.md` says the rounds on the
repairs are about half the cost, so **round 2 is not a formality and may want
session 5**. Nineteen findings' worth of repair is what round 2 is reviewing.

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

Two independent readings, and they do not agree about the same things. **The
human run** (VH-1, `notes.md` PHASE-09 sheet) answered a five-kind form twice
and its wire log discharges AC-1, AC-2, AC-3, AC-8 and AC-9 directly. **The
suite dimension of `review-code.md`** asked a different question of each
criterion — *what is the simplest production change that breaks this and leaves
its case green?* That table is below, and where the two disagree the mutation
wins, because a green run and an injection pass report different things.

| AC | verdict | evidence, and what holds it |
|---|---|---|
| AC-1 five kinds, declared order | **met** | `fields.rs:2120`; six (description, role) pairs in tree order, injection I-9 red. Confirmed on screen by VH-1 |
| AC-2 `R-57` types, untouched | **met** | `fields.rs:2158`, off the child process's own request log; six keys each a different JSON type from its neighbour. VH-1's run 2 is the untouched control |
| AC-2 `R-57` types, operated | **met** | `fields.rs:2222`. Partly self-agreeing for `datetime` — the expected value is computed by the production `instant::compose` — but the format is pinned by literals at `draft.rs:402`, `:439` and the shape by `fields.rs:1428` |
| AC-3 `R-58` | **met, with a knowing reduction** | `wiring.rs:1706` drives the *other option's field* half over two options sharing the id `read`. The *undrawn field* half is **unobservable by construction** once all five kinds draw; `canon-delta.md` CD-2 records that rather than substituting a case. Checked independently: `answer` walks `drawn_fields` and `Fields::new` refuses duplicate ids, so no entry can be dropped or collapsed |
| AC-4 every character recorded | **NOT MET** | **F-A1.** A character typed while an exchange is in flight is discarded, not deferred. The draft/wire half of the suite is real; the `inits` half is vacuous (**F-S1**) and the delivery rule's failure direction is untested (**F-S2**). Dispositioned *fix now* — `audit-log.md` |
| AC-5 a present disturbs nothing | **NOT MET** | **VH-1 and F-A1.** The element and write halves hold (`reassert.rs:236`, I-7 red). The caret and the drag are observed by no tier, and the drag fails on a person. Instrument fidelity is itself in question — **F-S5**. Dispositioned *fix now* |
| AC-6 refused or dropped edit corrected | **met** | `overlay.rs:191`, negative-controlled at `notes.md:2729`. A literal *refusal* is not separately driven; the guard is cause-blind and no surviving mutation was found |
| AC-7 undrawn reported, sixth kind a compile error | **met — held by nothing** | The `GroupHint` half is asserted. The compile-error half is real today but **no instrument in the gate keeps it real**: adding a `_` arm to `drawn_form` compiles, lints clean and leaves the gate green (**F-S3**) |
| AC-8 alternative id, not an option id | **met** | `fields.rs:2042`; label, index and id asserted as three different strings, I-4 and I-5 both red. VH-1's log carries `"fine"`, not `Fine` |
| AC-9 unbounded number, no invented range | **met** | `fields.rs:1803`, three prongs. VH-1's `counted` returned `0.0` with no range invented |
| AC-10 gate green, a person has answered | **partly** | `just check` exits 0 at 592. A person ran it (VH-1) — and what that run found is why AC-4 and AC-5 are unmet above |

**Two structural facts about the suite, both established by enumeration rather
than assertion, and both bearing on the repair.**

- **Only one field control is driven by real input events** — the `ComboBox`,
  in three cases (`fields.rs:2042`, `:2222`, `reassert.rs:236`), by
  `mock_single_click` and by hand-written `KeyPressed`/`PointerPressed`.
  `CheckBox`, both `LineEdit`s, the `Slider` and the `datetime` `Button` are
  reached **only** through the accessibility surface. **No case anywhere
  delivers a `KeyPressed` to a `LineEdit`**; every text entry is
  `set_accessible_value`, which `fluent/lineedit.slint:16` implements as an
  assignment plus a call to `edited`, reaching no `TextInput` insertion logic.
  That is the mechanical reason no tier ever moves a caret — and the reason
  F-A1 is invisible to all 592 tests.
- **No case operates any control while `root.busy` is true.** The three
  busy-aware cases read `accessible_enabled` and drive nothing. Of the seven
  `enabled: !root.busy` bindings only the two predating this slice are
  asserted at all: **deleting the binding from any of slice 009's five new
  controls leaves the whole suite green.**

### Verification criteria

Discharged by reading this session, against the code rather than the phase
sheet's own account: **PHASE-01/VA-2** (I-F's write order, `glass.rs:187-202`
— though see **F-S7** on whether it is observable at all), **PHASE-04/VA-1**
(the `jiff` residue argument), **PHASE-02/VA-1** (the vocabulary scan; the only
matches in `src/` and `app.slint` are the word *uninhabited*).

The rest of the VT/VA walk is **not done** and is session 2's. Two are already
contradicted by findings: **PHASE-05/T-8** is ticked for injection passes that
`notes.md` has no table for (**F-S4**), and **PHASE-05/VA-1** claims the
enqueue rule was confirmed when nothing exercises it (**F-S2**).

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Findings are not restated here.

- **Ledger:** `review-code.md`
- **State:** open · **round 1 complete**, all three dimensions plus the two
  attack areas the renderer dimension briefed and never reached. Round 2, over
  the repairs, has not run.
- **Outstanding blockers:** **none.** **F-A1** was the only one; it is
  dispositioned *fix now* (`audit-log.md`) and **repaired** at `665dcf3`,
  together with F-R2 and F-R3. The gate is green at 595.
- **Twenty-one findings**, every one dispositioned with the user: sixteen
  `fix-now`, four `doc-wrong`, one *settle first*.

Round 1 raised twenty-one. Five were **mutation-confirmed or measured by the
audit rather than accepted on the reviewer's report**, which is the standard
`docs/memory/` asks for — a claim about the tree can go stale under the agent
that made it. Two of those re-derivations changed the finding: **F-S3's stated
mutation does not lint clean**, and **F-R1 is wider than it was written** — the
picker survives `hide()` as well as a view replacement.

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

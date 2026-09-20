# Audit & reconciliation — Slice 009

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## PARTIAL — session 5 checkpoint (2026-09-20)

**Read this before anything else.** The audit is part-done. This section is the
state of it; it replaces session 4's, and it is deleted when the audit closes.

### What session 5 finished

- **Round 2 is complete, both dimensions.** The behaviour dimension's six
  Outcomes are all **`verified`**, none contested, every mutation re-run against
  production code rather than read. Its report raised **eight** new findings,
  `F-B1`–`F-B8`, two `major`. All transcribed in full, with a table row each.
- **`F-B9` raised by the orchestrator as declared raiser** — the last of the
  three held-back leads. The other two were found independently: `F-B2` is the
  `Focus::Diagnostics` picker, measured with a positive control, and `F-B4` is
  the liveness bound under load, instrumented at the bound. **Two witnesses
  each, not echoes.**
- **Thirteen dispositions, every one confirmed with the user** (`audit-log.md`,
  fifth entry). `F-S5` → **`follow-up`**; `F-B4` **splits**, its margin now and
  its harness a follow-up; the other eleven `fix-now` or `doc-wrong`.
- **Two verifications before pricing, and both changed the answer.** F-S5's
  contest re-ran and **holds** — the suite is green with the `CheckBox` written
  unconditionally and uncounted. F-T1's proposed closer was measured to *rescale
  silently*; what landed is a `const` assertion that **fails to compile**
  instead, which is cheaper and holds more.
- **All twelve repairs landed, with Responses in the ledger.** Three are
  injection-passed against the mutation that motivated them: F-B2, F-B1 and
  F-B6. F-T2's boundary claim is injection-passed too.

### The gate

**Exit 0** at `6993a56`. **30** `test result: ok` lines summing to **599** —
**564 distinct cases across 22 targets**, with `goad-semantics`' 35 built and
run twice under two feature configurations (599 − 35 = 564). **599 is a sum,
not a census**, and is not to be repeated as a count of cases.

Up from 29 / 597: one new target (`event_loop_answer`) and two new cases (its
own, and the tray identity case F-R5's repair owed).

### What is outstanding, in the order it should be taken

1. **Round 3 is owed, and it is the next thing.** Twelve repairs landed this
   session and **none has been reviewed**. `AGENTS.md` §*Audit & reconcile*:
   *"Repeat rounds until the repairs are themselves reviewed and nothing serious
   remains."* This is not a formality — round 2 found two majors and **contested
   a repair**, so the trend is not zero and
   `docs/memory/review-rounds-stop-on-a-measured-trend.md` does not license
   stopping. The surface: `665dcf3`..`68cbe90`, with `event_loop_answer`,
   `event_loop_picker`'s four new readings and `glass.rs`'s reordered present as
   fresh code. Ids `F-C1` onward.

   **Three things to put in its brief, because they are where this session is
   most likely to be wrong:**
   - **`glass.rs`'s present changed order.** The dismiss moved *above*
     `set_values` and now fires on a surface change. F-B8 is claimed closed
     **structurally** by that hoist. Have the reviewer settle on its own
     evidence whether the I-F transient is really closed again, and whether
     dismissing on every non-prompt present costs anything.
   - **`F-B9` is not injection-passed and its Response says so.** The case that
     would hold it is named there. Do not let a reviewer discover that as a
     finding; let it check whether the named case is the right one.
   - **`drain.rs`'s schedule was re-indexed wholesale.** Six green runs is not
     an injection pass. Its three new `const` assertions were checked against
     `DEBOUNCE` → 400 ms; its *step numbers* were not checked against anything.

2. **Four Outcomes have no raiser to return to** — `F-T1`–`F-T4`'s dimension
   closed. Round 3 sets them, or the orchestrator does as declared raiser, on
   the split the user already decided (`review-code.md`, §Findings).

3. **Re-walk AC-4 and AC-5.** The table still reads both **NOT MET**, which was
   true when written. Re-walk against the repairs — **do not edit it on the
   strength of them**. Note that F-B2 and F-B1 both bear on AC-4 now, and that
   both criteria's remaining halves (a caret, a drag) are observable by **no
   tier** and wait on VH-2.

4. **VH-2.** The user has undertaken to run it before close. The rig is
   committed: `GOAD_DEMO_PULSE=1 GOAD_DEMO_DELAY=3 just demo`, and `notes.md`
   §VH-2 carries ten observations, each naming what it settles and what failure
   looks like. Record what was **observed**, not that it was run.

5. **Verdict, Closure, Summary, Follow-ups**, none written.
   `slice-009.md` §Follow-ups must carry **F-R4**, the **slider quantisation**
   follow-up (`audit-log.md`, first entry), **F-S5** and **F-B4's harness
   half**. **Harvest → `docs/memory/` is not started** and the Harvest is ~120
   items; it wants its own pass.

### What not to rediscover

- **A repair can be wrong about what it holds, exactly as a finding can.** F-S5
  is the fourth instance and the first where it was the *repair*. Verified again
  this session on a quiet machine.
- **Verify a proposed instrument before pricing it.** Twice this session, and
  both times the proposal was worse than what verification produced: F-T1's
  derivation would have rescaled silently, and F-B4's *smaller* margin turned out
  to be the safe one.
- **A margin's size does not say which direction load moves it.** `drain.rs`'s
  1.33x is safe under load and its 3.0x is not. Read the bound, not the ratio.
- **Interaction identity is the host's, not the controller's.** A view handed
  straight to `Controller` is one `goad-shell` never issued, and a `Choose`
  against it is refused *"no interaction is outstanding"*. Found by building
  F-B1's case, not by reading.
- **Three properties are real, unheld, and not expressible as a case** — I-F
  (F-S7), one-edit-per-tick (F-S6), and PHASE-06/EX-3's refusing-`interpret`
  clause. Reaching for a case is the wrong move for all three.
- `examples/shell/backend.sh` and `examples/demo.toml` are reachable by **no
  gate command**. Check them by hand after editing.
- Still standing: **`Alternatives::first` cannot panic**, **`R-58`'s MUST holds
  structurally**, **`pending.rs`'s map is bounded and its timer terminates**,
  and **slint 1.18.0 fixes none of this slice's defects**.

### The budget

Five sessions planned, five spent. **Session 6 takes round 3 and its repairs;
session 7 takes the AC re-walk, VH-2, the close and the Harvest.** That is the
shape `docs/memory/audit-stage-needs-its-own-budget.md` predicts — the rounds
on the *repairs* are about half the cost — and this session is its third
confirmation. The Harvest lift alone is worth planning for and should not be
the tail of another pass.

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

`just check` — **exit 0**, re-run at `5227ec1` by this session. The gate is
build, both test tiers, the `deno check` of `examples/typescript/backend.ts`,
`cargo clippy --workspace --all-targets -D warnings`, and `cargo fmt --all
--check`.

**The number, stated once so it cannot imply a census.** The gate prints **597**
as a *sum* over 29 `test result: ok` lines, eight of which report zero. That is
**562 distinct cases across 21 targets**: `just check` runs `cargo test
--workspace` and then `cargo test -p goad-semantics`, so that crate's 35 are
built and run **twice**, under two feature configurations — which is the point
of the second command (`ADR-001`, `POL-001` §Verification: stratum 1 must stand
alone), not an accident. The inherited 584/592/595/596 all have the same
property. The slice began at 549+35.

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

**Applied.** Every row below is done, and the table is the record of what
changed rather than a list of intentions. Endorsement for the canon rows is in
`audit-log.md` — the first entry for CD-1, CD-2, the example and the roadmap;
the third for F-P2's `design.md` edit; the fourth for the two amendments CD-2
did not carry.

**Sixteen rows, not eleven.** Five of them are `notes.md`'s own reconciliation
list (`notes.md` §*What is owed*), endorsed 2026-09-19 and never transcribed
here — which is itself worth recording: a list kept in the work file did not
reach the closing argument, and only a re-read found it.

### Canon

| document | change | reason | done |
|----------|--------|--------|------|
| `specs/001 §6.2` | CD-1 promoted **descriptively**, as an *untouched value* table beside the sibling *what each kind submits* table, with both consequences: the `max`-only `0`, and F-P2's cleared bounded `number` submitting its `min` | `R-58` forbids omitting a value and `OQ-2` is unlanded, so the host must supply one, and a backend author could discover none of it. CD-1 named §7; §6.2 is where a backend author meets the sibling table, and the relocation is recorded here rather than made silently | [x] |
| `specs/001 §6.2` | *"including the four no renderer in this repository draws yet"* — an expired premise, on no list | this renderer draws all five. Found while placing CD-1 | [x] |
| `specs/001 §Verification`, `R-58` row | names the surviving half of the MUST NOT (`an_answer_carries_no_value_for_another_options_field`), and says the other half is held by the shape of `answer`'s walk rather than by a case | **canon was untrue**: both cited cases were deleted by PHASE-09 and both greps now return nothing | [x] |
| `specs/001 §Verification`, `R-57` row | *"review, not a test"* retired; the two `every_{untouched,operated}_kind_leaves_the_host_with_the_json_type_r57_names` cases and the three per-kind cases named; closing citation corrected `present` → `drawn_form` | the premise — no renderer draws those kinds — expired this slice. The citation is CD-2's fourth change and CD-2 did not carry it | [x] |
| `specs/001 §Verification`, `R-55` row | the option-fields sentence dropped; the sixth-kind path named as **two** things — `drawn_form`'s exhaustive match **and** `clippy::wildcard_enum_match_arm` | no option field is undrawn on account of its kind any more; and **F-S3 landed after CD-2 was drafted**, so CD-2's *"the match is the guard"* would have written a half-truth into canon (`audit-log.md`, fourth entry) | [x] |
| `specs/001 §Verification`, `R-18` row | **confirmed**, and the site named: the read is `Run::of`, reached from `present` through `sift` | the claim is true of the call tree and was carried here as unverified. What had decayed was its locating power | [x] |
| `specs/001 §8` OQ-4 | **not amended.** Still an open user discussion | the audit's position — that the fork is asymmetric, since `R-18` already admits the hint half with no protocol change — is recorded in `audit-log.md` and was not decided. `roadmap.md` carries what 009 found | [ ] — deliberately |
| `canon-delta.md` CD-1, CD-2 | corrected, then promoted; the file records what landed | `AGENTS.md`: a slice does not close holding an unpromoted draft | [x] |

### The slice's own record

| document | change | reason | done |
|----------|--------|--------|------|
| `slice-009.md` §Governing canon | *"nothing reaches `goad-semantics`"* replaced by the declared exception and its purity argument | PHASE-09 touched `canonical.rs` with explicit endorsement, declared in `plan.md` | [x] |
| `design.md` §5.2, §5.5 I-H, §5.5 edges, §9 A-2, §7 D13 | the cleared numeric field submits the number it held, not `0`; I-H gains that and F-P1's sub-minute offset; A-2 and D13 stop stating a guard exception PHASE-08/EX-7 removed | **F-P2**, endorsed. Three sentences are residue of D-16, reversed at D-33; the exception rows are `notes.md`'s third row, endorsed 2026-09-19 | [x] |
| `design.md` §5.5 **I-F** | recorded as a **forward constraint** — no `init` handler may read `root.values` — rather than an invariant this markup depends on | **F-S7**, settled by measurement: the transient is unobservable and no case can be written against it | [x] |
| `design.md` §8 **R9**, §9 driver table (AC-2 operated, AC-8) | R9's risk is coordinate mapping, not layout, and the mitigation it named does not exist; neither row moved, because the `ComboBox` answers to a click on itself plus keys | `notes.md`'s fifth row, endorsed 2026-09-19. PHASE-09/VA-1 has the measurement | [x] |
| `design.md` §9 **A-6** | the assumption holds; *"costs focus, not data"* is false | **F-A1** | [x] |
| `design.md` §7, the `choice` as-drawn bullet | the *report it as `Undrawn`* route was **live and unused**, not dead for the reason given; and one `expect` does live in `Alternatives::first`, by decision | `notes.md`'s fourth row, endorsed 2026-09-19 | [x] |
| `design.md` §9 AC-4 row, `plan.md` §Coverage and PHASE-05/VT-3 | the `inits` half is vacuous and why; the four cases that **do** hold the mechanism named | **F-S1** | [x] |
| `design.md` §5.2 comparand table, §1 AC-7 argument, D11, §9 AC-7 row; `research.md`; `canon-delta.md` | `undrawn_form` → `drawn_form`; and AC-7's argument gains F-S3's missing clause — the match stops a sixth kind only while no wildcard absorbs it | `notes.md`'s first row, endorsed 2026-09-19. **`canon-delta.md` is the one that mattered**: it is promoted into canon, so a stale identifier there would have landed there | [x] |
| `plan.md` PHASE-06/EX-3 | the refusing-`interpret` clause is discharged **by construction**, enumerated, not an untested path owed a case | `notes.md`'s second row. Enumerated at audit: only three `Reported` variants are debounced, each accepted against its own kind, and a kind mismatch would need one `view_id` to denote two presentations — which the host-minted counter forbids | [x] |
| `examples/shell/backend.sh` | three statements, `:15`, `:101`, `:102` | all three went false this slice; the gate cannot see this file, and one of them describes runtime behaviour a person running `just demo` watches the host contradict | [x] |
| `docs/roadmap.md` §Open decisions, §009 | OQ-4 stays shut, the trigger was not met, and what 009 produced is an **affordance cost** rather than an inexpressibility; the residue's fork named as asymmetric | `design.md` §10 names this as owed at close | [x] |

**Design drift not reconciled:** *(written at close)*

## Closure

- [ ] All findings dispositioned; no blockers outstanding
- [ ] All acceptance criteria met, or explicitly waived by the user
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

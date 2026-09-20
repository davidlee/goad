# Audit & reconciliation — Slice 009

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

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

`just check` — **exit 0**, re-run at close. The gate is build, both test tiers,
the `deno check` of `examples/typescript/backend.ts`, `cargo clippy --workspace
--all-targets -D warnings`, and `cargo fmt --all --check`.

**The number, stated once so it cannot imply a census.** The gate prints **600**
as a *sum* over **30** `test result: ok` lines, several of which report zero.
That is **565 distinct cases across 22 targets**: `just check` runs `cargo test
--workspace` and then `cargo test -p goad-semantics`, so that crate's 35 are
built and run **twice**, under two feature configurations — which is the point
of the second command (`ADR-001`, `POL-001` §Verification: stratum 1 must stand
alone), not an accident.

**600 is a sum, not a census, and is not to be repeated as a count of cases.**
The figures this audit passed through — 584, 592, 595, 596, 597, 599, 600 — all
have the same property, which is why the sentence above exists. The slice began
at 549 + 35. The audit added sixteen cases: round 1's repairs, then `F-B6`,
`F-B1`, `F-T1`'s `const` assertions, `F-C3`'s thirteenth reading and `F-C6`'s
repeater counter.

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

**Three independent readings, taken at different times and asking different
questions.**

- **VH-1** (`notes.md` PHASE-09 sheet) answered a five-kind form twice, before
  any audit repair. Its wire log discharges AC-1, AC-2, AC-3, AC-8 and AC-9
  directly, and it is what found **AC-4 and AC-5 unmet**.
- **The suite dimension of `review-code.md`** asked the mutation question of
  every criterion — *what is the simplest production change that breaks this and
  leaves its case green?* Where it and a green run disagree, the mutation wins:
  a run reports that something worked once, an injection pass reports what the
  gate would notice.
- **VH-2** (`notes.md` §VH-2) ran the repaired software across three sessions
  and ten observations, with `goad.service` stopped so a second host could not
  confound it. It carries the **only** readings that exist for a caret, a drag
  and a live picker, and it re-reads AC-2, AC-3, AC-8 and AC-9 against the tree
  as it now stands.

The table below is walked against the **readings**, not against the repairs.

| AC | verdict | evidence, and what holds it |
|---|---|---|
| AC-1 five kinds, declared order | **met** | `fields.rs:2120`; six (description, role) pairs in tree order, injection I-9 red. Confirmed on screen by VH-1 |
| AC-2 `R-57` types, untouched | **met** | `fields.rs:2158`, off the child process's own request log; six keys each a different JSON type from its neighbour. VH-1's run 2 is the untouched control, and **VH-2 observation 8** re-reads it on the repaired tree: `mood` untouched submits `"good"` |
| AC-2 `R-57` types, operated | **met** | `fields.rs:2222`. Partly self-agreeing for `datetime` — the expected value is computed by the production `instant::compose` — but the format is pinned by literals at `draft.rs:402`, `:439` and the shape by `fields.rs:1428` |
| AC-3 `R-58` | **met, with a knowing reduction** | `wiring.rs:1706` drives the *other option's field* half over two options sharing the id `read`. The *undrawn field* half is **unobservable by construction** once all five kinds draw; `canon-delta.md` CD-2 records that rather than substituting a case. Checked independently: `answer` walks `drawn_fields` and `Fields::new` refuses duplicate ids, so no entry can be dropped or collapsed. **VH-2's single submission** carried all six fields to the backend on the repaired tree |
| AC-4 every character recorded | **met** | Was **NOT MET** on **F-A1**: a character typed while an exchange was in flight was discarded rather than deferred. Three repairs changed it — `busy` narrowed to *your answer is in flight*, `commands` drained before the present (**F-R3**), and `serve`'s `engage` call site given a case at all (**F-B1**). Held mechanically by `event_loop_busy::a_key_is_recorded_while_the_host_polls_and_dropped_while_the_answer_is_in_flight` — **the first case in the crate to deliver a real `KeyPressed` to a `LineEdit`** — by `event_loop_drain::a_tick_enqueued_during_an_exchange_survives_the_present_that_follows_it`, and by `event_loop_full`'s three readings, which is the driver **F-S2** asked for. **F-S1**'s vacuous `inits` half is repaired in the case's own doc. Read on a person at **VH-2 run 1, observation 4**: *"no issues with text field (cursor, loss of entry)"*. The remaining drop — a key typed while *your own answer* is in flight — is the contract, not the defect, and is what the case's name says |
| AC-5 a present disturbs nothing | **met** | Was **NOT MET** on VH-1's measured drag failure. The element and write halves held throughout (`reassert.rs:236`, I-7 red). The caret and the drag are observable by **no tier** — that is why D-10 assigned them to a human run — and both now have their **first positive readings**, at **VH-2 run 1**: *"the flash of redrawing things is gone"* (observation 1) and *"the slider drag stop is no longer happening"* (observation 2). Two guards gained instruments during the audit: the repeater's, at **F-C6**'s `renderer/wiring.rs::a_re_present_with_unchanged_lines_does_not_rebuild_them`, and the picker's converse, at **F-C3**. **Instrument fidelity remains unheld and is a follow-up — F-S5**: a write to a guarded widget that bypasses the counter survives every case |
| AC-6 refused or dropped edit corrected | **met** | `overlay.rs:191`, negative-controlled at `notes.md:2729`. A literal *refusal* is still not separately driven; the guard is cause-blind and no surviving mutation was found. **Now also read in the human tier, which is the only one that sees both halves at once**: at **VH-2 run 1, observation 3** a dropped `Mood` — a `choice`, undebounced, held nowhere — reverted on the next present, while dropped keystrokes — debounced, held in `pending.rs` — did not. The two differed in exactly the direction AC-6's *dropped* clause names, and the observation was nearly written off as a defect before the footer notice was traced to `Wire::send`'s back-pressure on a capacity-1 channel |
| AC-7 undrawn reported, sixth kind a compile error | **met — held by nothing** | The `GroupHint` half is asserted. The compile-error half is real today but **no instrument in the gate keeps it real**: adding a `_` arm to `drawn_form` compiles, lints clean and leaves the gate green (**F-S3**) |
| AC-8 alternative id, not an option id | **met** | `fields.rs:2042`; label, index and id asserted as three different strings, I-4 and I-5 both red. VH-1's log carries `"fine"`, not `Fine`; **VH-2 observation 8** carries `"good"`, not `Good`, on the repaired tree |
| AC-9 unbounded number, no invented range | **met** | `fields.rs:1803`, three prongs. VH-1's `counted` returned `0.0` with no range invented, and **VH-2 observation 6** re-reads the harder case — `Pages written` **cleared to empty** submits `0.0`, not a minimum and not `""`. `0.0` is `serde_json`'s spelling of the `f64` zero; `view_model.rs::spelled` is `f64::to_string`, so the same value reads `0` on screen. Not a fourth screen/wire divergence — the bounded case, which *does* submit its minimum, is the one `design.md` §5.5 I-H gained a clause for (**F-P2**) |
| AC-10 gate green, a person has answered | **met** | `just check` exits **0** — **30 `test result: ok` lines summing to 600**, which is **565 distinct cases across 22 targets** with `goad-semantics`' 35 built and run twice under two feature configurations. 600 is a sum, not a census. A person ran the software twice: **VH-1** answered a five-kind form and found AC-4 and AC-5 unmet; **VH-2** ran the repairs across ten observations and gives both their first positive readings |

**Two structural facts about the suite, both established by enumeration rather
than assertion. Both were round 1's readings, and the repairs moved one of
them** — recorded here as the before and the after, because a claim about the
suite is exactly the kind this audit has watched go stale.

- **Round 1: only one field control is driven by real input events** — the
  `ComboBox`, in three cases (`fields.rs:2042`, `:2222`, `reassert.rs:236`).
  `CheckBox`, both `LineEdit`s, the `Slider` and the `datetime` `Button` were
  reached **only** through the accessibility surface, and **no case anywhere
  delivered a `KeyPressed` to a `LineEdit`** — every text entry was
  `set_accessible_value`, which `fluent/lineedit.slint:16` implements as an
  assignment plus a call to `edited`, reaching no `TextInput` insertion logic.
  That was the mechanical reason no tier could move a caret, and the reason
  F-A1 was invisible to the whole suite as it then stood.

  **This is no longer true, and F-A1's repair is what changed it.**
  `event_loop_busy` and `event_loop_drain` both dispatch real
  `KeyPressed`/`KeyReleased` to a focused `LineEdit`, and `drain.rs::key`'s doc
  says why in as many words: `set_accessible_value` *"would assign the text and
  call `edited` from inside the markup, reaching neither `TextInput::key_event`
  nor the `enabled` gate this case's arrangement turns on."* The caret itself is
  still observable by no tier — that half stands, and is why VH-2 was owed.

- **Round 1: no case operates any control while `root.busy` is true.** The three
  busy-aware cases read `accessible_enabled` and drove nothing. Of the seven
  `enabled: !root.busy` bindings only the two predating this slice were asserted
  at all: deleting the binding from any of slice 009's five new controls left
  the whole suite green.

  **Overtaken, and re-measured at close rather than left open.** The binding
  was deleted from each field control and the suite run — `cargo test -p goad
  --no-fail-fast`, `app.slint` restored from a copy and verified byte-identical
  to `21e5827` afterwards. **Two of the six are now held; four are held by
  nothing:**

  | control | `enabled: !root.busy` held by |
  |---|---|
  | `boolean` `CheckBox` | **yes** — `renderer::wiring::both_controls_are_disabled_while_the_answer_is_in_flight_and_enabled_after_it` |
  | `text` `LineEdit` | **yes** — `event_loop_busy::a_key_is_recorded_while_the_host_polls_and_dropped_while_the_answer_is_in_flight` |
  | `number` `Slider` | **no** |
  | `number` `LineEdit` | **no** |
  | `choice` `ComboBox` | **no** |
  | `datetime` `Button` | **no** |

  Removing all six reddens `event_loop_busy` and `renderer`; removing **only**
  the four leaves every target green, 205 passed in `renderer` and no failure
  anywhere. So the two that are held are held by the two cases this audit added
  — **F-A1/F-R2's and F-B1's** — and the four that are not were never covered by
  anything.

  **This is a coverage gap, not a defect**: the bindings are present and correct,
  and production disables all six. It is the same class as **F-S5** — a real
  property held by nobody — and it lands in §Follow-ups beside it, now with a
  number rather than as a suspicion.

### Verification criteria

`plan.md` carries **62** verification criteria across nine phases — **44 `VT`**,
each claiming a case, and **18 `VA`**, each claiming an argument. **All 62 are
now walked against the code**, which is what `docs/AGENTS.md` §*Audit &
reconcile* asks for and what session 1 deferred and no session picked up.

| phase | criteria | referenced in the record before this walk |
|---|---|---|
| PHASE-01 | 7 | 1 |
| PHASE-02 | 7 | 1 |
| PHASE-03 | 4 | 1 |
| PHASE-04 | 6 | 2 |
| PHASE-05 | 8 | 2 |
| PHASE-06 | 5 | 0 |
| PHASE-07 | 6 | 0 |
| PHASE-08 | 8 | 0 |
| PHASE-09 | 11 | 2 |

Nine of 62 had been referenced anywhere in `audit.md`, `audit-log.md`,
`review-code.md` or `notes.md`; 53 had not. The walk was finished rather than
carried because **two of the three criteria the audit had walked independently
were contradicted** — **PHASE-05/T-8** ticked for injection passes `notes.md`
has no table for (**F-S4**) and **PHASE-05/VA-1** claiming the enqueue rule
confirmed when nothing exercised it (**F-S2**). That is a rate on a sample of
three, not an estimate of 53, but it is what the walk is for.

**The 53 were never bare ticks.** Every one of the 62 has a substantive entry
in its phase sheet: a table row naming cases and injection letters, or a titled
argument. So the walk checks written evidence against the code, rather than
filling a void — which is why it cost one session and not four.

**It was split by what a script can reach.** `scratchpad/criteria.py` asks the
two questions F-S4 turned on; the other 30 were walked by hand. Both halves are
reproducible: the checker builds its own denominator from `cargo test -- --list`
rather than from any number written here.

#### The scripted half — 32 criteria

- **Every file-qualified case name a sheet entry gives exists in the built
  suite.** The one apparent exception is PHASE-09/**VA-3**, whose entry names
  eight `mapper.rs`, `fields.rs`, `wiring.rs` and `reception.rs` cases that are
  absent — *correctly*: VA-3's whole subject is their **deletion**, and it
  states per case what each no longer asserts and where the claim went.
- **Every injection letter cited resolves to a definition in its own phase
  sheet.** Sixteen criteria cite one; none is undefined.
- Thirteen criteria name a bare backticked symbol the suite does not carry. All
  thirteen are API, harness or `[[test]]`-directory names —
  `init_no_event_loop`, `mock_single_click`, `invoke_accessible_expand_action`,
  `cast_possible_truncation`, `event_loop_*` — each confirmed present in the
  tree. No claimed case is missing behind them.
- Independently: the built suite lists **565** distinct cases, which is the
  number §Closure states.

#### The hand half — 30 criteria

The 30 the script cannot reach — 18 `VA` arguments and 12 `VT`s whose sheet
entry names no case. All 30 walked; every one discharged. What the walk did
rather than read:

- **PHASE-05/VT-6's negative control was run, and it had never been.** PHASE-05
  is the one phase with no VT evidence table — that absence *is* F-S4 — so its
  six were walked from nothing. VT-5 and VT-6 are the two claims of the single
  `event_loop_debounce` case,
  `the_timer_delivers_one_edit_per_tick_and_re_arms_while_the_map_is_not_empty`.
  Removing the re-arm from `Debounce::tick` takes the target to
  `0 passed; 1 failed` at the **second** claim — `left: 1, right: 2`, *the timer
  re-armed while the map was not empty* — with the first claim still passing.
  That is exactly the discrimination the module doc claims. Reverted, `git diff`
  empty.
- **PHASE-06's recorded injection was re-run rather than taken on report.**
  Inverting the overlay's lookup in `carried` — `entry.view == view` to `!=` —
  reproduces the sheet's reading exactly: the first assertion, `("", "")`
  against the typed pair, `reasserts: 2`. The table is true about the code.
- **PHASE-05/VT-4** is
  `a_stale_carried_edit_is_refused_once_and_still_answers_but_an_undeclared_one_does_not`,
  and it asserts all four clauses the criterion names — the `SupersededView`
  refusal, **one** reported line for two stale edits, the answer still going,
  and `Err(Refused::UnknownField)` with no answer for the undeclared field. It
  carries a non-stale edit beside the stale ones, so a `choose` that abandoned
  every carried edit at the first refusal could not pass it. VT-1 and VT-2
  likewise located and read.
- **PHASE-01/VT-3 and VA-1 are held by construction**, which is stronger than
  the sheet claims: `FieldRow` no longer declares `checked` at all, so no case
  *can* read a value off the row and still compile. The two surviving `.checked`
  readers go through `value_of`, which is `get_values().row_data(slot)`.
- **PHASE-07/VA-2 re-checked against `app.slint` as it now stands**, not as
  PHASE-07 left it: six `root.edited(` sites since PHASE-08 and PHASE-09 added
  theirs, the `datetime` one still only on the time picker's `accepted`, and
  `canceled` still an empty handler at both pickers. No field holds half a pick.
- **PHASE-08/VT-1's single unit**,
  `slider_bounds_admits_only_a_range_a_slider_can_be_operated_over`, carries all
  four clauses the criterion names — ordinary range, equal bounds, an `f32` span
  of infinity, and the `2^100` ulp case — and its doc states why no clause
  subsumes another. VA-2's ` as f32` / ` as f64` grep still returns exactly one
  line, `exact_f32`'s own checked narrowing.
- **PHASE-09/VA-2's property is a compile-time one and survives**: `FieldForm`
  is still `pub enum FieldForm {}` and `drawn_form` still returns
  `Result<DrawnKind, FieldForm>`, so a sixth `FieldKind` is still a compile
  error that has to be sorted rather than defaulted.

#### What the walk found

**No criterion is unmet, and there is no second F-S2.** One record-vs-code
divergence, of **F-D3**'s class rather than F-S2's:

- **PHASE-05/VA-2's evidence names a case that no longer exists.**
  `wiring.rs::an_answer_carries_no_value_for_another_option_or_for_an_undrawn_field`
  was **halved and renamed** `…_for_another_options_field` by PHASE-09, which
  PHASE-09/VA-3 records in full. The sheet was true when written. Repaired the
  way this plan already repairs it elsewhere: PHASE-02/**VA-2** carries a
  parenthetical saying its sorter was renamed in a later phase and the evidence
  is left naming what it named then, and PHASE-05/VA-2 now carries the same
  note.

Line drift in three cited readings — PHASE-07/VA-1's `[[test]]` count of **6**
(now **12**, PHASE-08 and PHASE-09 having added targets), PHASE-08/VA-2's
`view_model.rs:502` and its three `app.slint` reads — is a point-in-time record
behaving as one. The **properties** each cites were re-checked and all three
hold; only the coordinates moved. This is the class `CLAUDE.md` names and
§Follow-ups #8 proposes an instrument for.

**The instruments.** `scratchpad/criteria.py` (the two scripted questions) and
`scratchpad/inject.py` (apply one defect, run one target, restore, verify the
restore by `git diff`). Neither is a gate instrument and neither is proposed as
one here; `criteria.py` is the nearest existing material for §Follow-ups #8.


## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Findings are not restated here.

- **Ledger:** `review-code.md`
- **State:** **closed after four rounds.** Every finding has a disposition
  confirmed with the user and a terminal Outcome.
- **Outstanding blockers:** **none.** **F-A1** was the only one ever raised; it
  was dispositioned *fix now* (`audit-log.md`) and repaired at `665dcf3`
  together with F-R2 and F-R3.
- **Forty-seven findings.** Thirty-five `fix-now` at raise, eight `doc-wrong`,
  one *settle first*, and three re-dispositioned to `follow-up` after their
  price was established rather than estimated — **F-R4**, **F-S5**, and
  **F-B4**'s harness half. Two were `contested` and returned to open —
  **F-B9** and **F-T3** — and both re-dispositions were confirmed at the source
  by the orchestrator before they were priced.

**The four rounds, and the shape is the argument for stopping.** Each round
reviewed the previous round's repairs, which no one else had looked at.

| round | findings | what they were |
|---|---|---|
| 1 | 21 | **one blocker and six majors of live defect** — `busy` deaf to input, a picker outliving its view, a present reverting a draft |
| 2 | 13 | **two majors about what holds a repair** — `serve`'s `engage` site held by no case, a repeater rebuilt every present |
| 3 | 6 | **one live defect, one coverage gap with production behaviour separately measured correct, and four claims wrong in prose** |
| 4 | 7 | **no behavioural defect at all.** Seven claims wrong in prose: two counts, a step number, a wrap width, a uniqueness, a cost priced by a mechanism that does not exist, and a citation class |

The defects were gone by round 4 and what remained was the record disagreeing
with the code. **That is why round 4 was scoped as a verification pass and why
there is no round 5** (`audit-log.md`, seventh entry): every round-4 finding is
script-checkable, and every one of them is a thing a *reading* agent had already
got wrong at least once.

**Verification was not taken on report.** Across the four rounds the audit
re-ran or re-measured rather than transcribing, and it changed an answer seven
times. Round 1: **F-S3's stated mutation does not lint clean**, and **F-R1 is
wider than it was written** — the picker survives `hide()` too. Round 2:
**F-T1's closer is cheaper than its own author proposed**, and the `const`
assertion makes the finding's mutation fail to *compile*. Round 3: both
contests confirmed at the vendored source, and **the audit's own first
instrument was wrong** — a clock declared inside the per-tick closure, reading
90 ns. Round 4: **F-D3's enumeration and F-D5's count were both corrected at
the source**, and both corrections widened the finding.

## Verdict

**The slice does what it set out to do, and it closes.** All ten acceptance
criteria are met — none waived — the gate exits 0, and a person has run the
software twice and seen the new behaviour. No blocker is outstanding.

**What it set out to do.** Take a renderer that drew one of five field kinds and
make it draw all five, without narrowing the wire contract to the subset the
renderer happens to implement. That is the project's own named failure mode, and
the boundary test and the reconciliation both hold it: `R-57`'s types and
`R-58`'s rule are now verified per kind rather than by review, and the premise
that justified the old wording — *"including the four no renderer in this
repository draws yet"* — is retired from canon because it expired.

**What the audit actually found, and it was not in the new drawing code.** The
slice replaced the mechanism by which a present reaches the form, and the defect
was there: `busy` meant *the host is talking to the backend*, and a disabled
Slint item **discards** input rather than queueing it. Every character typed
during a routine poll was lost. AC-4 and AC-5 were both unmet from that one
cause, and **no test in the suite could see it** — nothing delivered a real key
event to a text field, and nothing operated any control while `busy` was true.
It took a person running the software to find it, which is what
`AGENTS.md` §Tiers exists for and why VH-1's value was entirely in one of its
three observations.

**Two criteria were repaired rather than waived**, and that was the decision the
slice turned on. The Closure checklist would have admitted a waiver; the user
declined it, on the ground that suppressing the symptom left the deafness
intact and invisible until a slow backend. AC-4 and AC-5 now read **met**, on
readings a person took against the repaired build.

**What is accepted knowingly.** Five things, each owned in `slice-009.md`
§Follow-ups rather than closed here, and none deferred for being large:

- **`F-R4`** — one full present per refused ingress arrival. Deferred because
  the question underneath it belongs to canon: `SPEC-003/R-15` requires a
  refusal decided while idle to reach the diagnostics surface, and canon's own
  verification instrument reads the retained model rather than the window, so it
  would not report the change.
- **`F-S5`, and four unheld `enabled` bindings.** Both are the same class — a
  real property held by nobody — and both are now measured rather than
  suspected. The instrument-fidelity one was priced against its **canon** cost,
  not its code cost: a markup scan would be a fifth boundary instrument and
  `POL-001` §Verification enumerates them.
- **`F-B4`'s harness half.** Three loop targets fail at ~6x CPU
  oversubscription, on the liveness backstop rather than on any assertion. A
  40x nominal margin was not enough, so widening bounds is not the repair.
- **The tray icon's missing re-assertion path** — `F-R5`'s unpriced half, and
  the class is the durable part: *a repair that removes a redundant write also
  removes the self-healing that redundancy was accidentally providing.* Raised
  as a follow-up and not as a finding, deliberately: the cause of the
  disappearance is unknown and the witness was hedged.
- **The citation discipline is a discipline, not an instrument.** Seventeen
  in-repo line citations remain, none wrong today, nothing stopping the next
  edit from breaking one.

**The walk that was nearly skipped was finished instead.** The audit's
independent walk of `plan.md`'s **62** verification criteria had been deferred
in session 1 and picked up by nobody through six; 9 of 62 were referenced
anywhere in the record. It is now complete — §*Verification criteria* above —
split between a scripted half that asks whether every named case exists and
every cited injection is defined, and a hand half of 30. **No criterion is
unmet.** The two measurements the walk added rather than read are PHASE-05/VT-6's
negative control, run for the first time, and PHASE-06's recorded injection,
re-run and found true. One divergence surfaced, of F-D3's class: a PHASE-05
evidence row naming a case PHASE-09 renamed, repaired by the note PHASE-02/VA-2
already sets the precedent for.

The reason this was not carried as a follow-up is in the section itself: the
sentence that deferred it — *"the rest of the walk is not done and is session
2's"* — is exactly the artefact a follow-up row would have become. `docs/templates/slice/audit.md`
§Closure now carries a box for the walk, which is what was missing: the
checklist that ends a slice enumerated the acceptance criteria and never asked
about the verification ones.

**One thing is deliberately unfinished.** `SPEC-001` OQ-4's wording is an open
discussion, not an omission. The audit's position — that the fork is
asymmetric, because `R-18` already permits a renderer and only a renderer to
branch on a hint, so the hint half needs no protocol change — is recorded in
`audit-log.md` and was not decided. `roadmap.md` carries what 009 found.

**The honest summary of the review is that its later rounds were about the
record, not the code.** Rounds 1 and 2 found defects; rounds 3 and 4 found
sentences. Four claims wrong in prose, then seven — two counts, a step number, a
wrap width, a uniqueness, a cost priced by a mechanism that does not exist, and
a citation class that regenerated **twice inside the commits that repaired it**.
That is a real quality signal and it is not a code-quality one: this slice's
code is held by 565 cases and its prose is held by nobody, and the closing act
of the audit was to replace a reading agent with a twenty-line script and watch
it find three failures two careful readings had missed.

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
| `plan.md` PHASE-05/VA-2 | a parenthetical: the case its evidence names was halved and renamed by PHASE-09, and `notes.md` is left naming what it named then | found by the criteria walk. Same treatment PHASE-02/VA-2 already carries for `undrawn_form` → `drawn_form` | [x] |
| `docs/templates/slice/audit.md` §Closure, `docs/AGENTS.md` §Close | a box for the §Open sweep, and the clause naming §Open as where §Follow-ups is drawn from | **endorsed by the user.** The template's §Open already said *"candidates for follow-ups"* and nothing walked it; a box with no methodology behind it would have been a box, so both moved | [x] |
| `docs/templates/slice/audit.md` §Closure | a box for the verification-criteria walk, beside the acceptance-criteria one | **endorsed by the user.** The walk is asked for once, inside `AGENTS.md`'s Evidence bullet, and the checklist that ends a slice never asked again — which is how six sessions passed without the omission being noticed | [x] |
| `docs/roadmap.md` §*Still open from 003* | `SPEC-002`/OQ-4 gains what 009 changed about it — the host now retains a draft and a keyed pending map, so *typed into and not yet answered* is interaction state, which removes half of OQ-4's stated reason for being open — and a line saying it is not `SPEC-001`'s OQ-4 | found by the `notes.md` §Open sweep. The reasoning was already in `design.md` §8 R5; the line a later slice actually reads still said *"still a protocol question"* and nothing else. The Reconciliation row above covers **SPEC-001**'s OQ-4 — a different question with the same number, which is how this one was missed | [x] |
| `notes.md` §Open | nine entries dispositioned in a table at the head of the section | nothing in the lifecycle walks §Open against `slice-009.md` §Follow-ups, and this audit had not. One promoted (**#10**), one carried to the roadmap, one found **already resolved and left here in error**, three superseded by later phases, one settled, one discharged in place, one still open and correctly so | [x] |
| `examples/shell/backend.sh` | three statements, `:15`, `:101`, `:102` | all three went false this slice; the gate cannot see this file, and one of them describes runtime behaviour a person running `just demo` watches the host contradict | [x] |
| `docs/roadmap.md` §Open decisions, §009 | OQ-4 stays shut, the trigger was not met, and what 009 produced is an **affordance cost** rather than an inexpressibility; the residue's fork named as asymmetric | `design.md` §10 names this as owed at close | [x] |

**Design drift not reconciled.**

- **`design.md` §5.2's `to-float` reading (D-16) is gone from the code and its
  residue is now gone from the design**, but the *record of the reversal* stays
  where it happened — D-33 in `design-log.md`. The design document states the
  rule the code implements; it does not narrate having changed its mind.
- **`design.md` §9 **A-6** stands as written and is marked false in one clause.**
  The assumption holds; *"costs focus, not data"* does not, and `F-A1` is why.
  Amended as a measured note beside the reasoning rather than by rewriting the
  reasoning, because the assumption was reasonable when it was made and the
  measurement is the interesting part.
- **`design.md` §8 **R9**'s mitigation never existed**, and neither driver-table
  row moved. The risk was misidentified — coordinate mapping, not layout — and
  the `ComboBox` is answered by a click on itself plus keys. Recorded rather
  than retro-fitted: the design's *prediction* was wrong in a way worth
  preserving, since it is what sent PHASE-07 looking for a capability that was
  there all along.
- **`design.md` §5.5 **I-F** was demoted from an invariant to a forward
  constraint**, on measurement (`F-S7`): the transient it describes is
  unobservable and no case can be written against it. The design's original
  wording implied the markup depends on it. It does not; a future `init` handler
  could make it matter, which is what the constraint now says.

## Closure

- [x] **All findings dispositioned; no blockers outstanding.** 47 findings
      across four rounds in `review-code.md`, every one dispositioned with the
      user and carrying a terminal Outcome. `F-A1` was the only blocker ever
      raised and was repaired at `665dcf3`.
- [x] **All acceptance criteria met**, none waived. AC-4 and AC-5 were repaired
      rather than waived — the decision the slice turned on — and their final
      readings are a person's, because no test tier can see a caret or a drag.
- [x] **Each verification criterion in `plan.md` walked against the code.** All
      **62**, split between `scratchpad/criteria.py`'s two scripted questions
      (32) and a hand walk (30). None unmet. One F-D3-class divergence, repaired
      in `plan.md`. §*Verification criteria* carries the readings.
- [x] **Tests and checks green.** `just check` exits 0: **30** `test result: ok`
      lines summing to **600**, which is **565 distinct cases across 22
      targets**, `goad-semantics`' 35 built and run twice under two feature
      configurations. 600 is a sum, not a census.
- [x] **Specs / policy / ADRs reconciled, with user endorsement where amended.**
      Sixteen rows in the Reconciliation table above, endorsements cited to
      `audit-log.md`. `POL-001` is untouched and the gate's instrument count is
      unchanged. `SPEC-001` OQ-4 is deliberately not amended and says so.
- [x] **`canon-delta.md` promoted.** CD-1 and CD-2 both corrected first, then
      applied — CD-2's Change 3 amended before promotion because `F-S3` landed
      after it was drafted and would have written a half-truth into canon. No
      `draft-spec.md` was opened. The slice closes holding no unpromoted draft.
- [x] **`slice-009.md` Summary and Follow-ups written.** Nine follow-ups, each
      with the reason it is not in this slice.
- [x] **`notes.md` §Open swept against §Follow-ups.** Nine entries, each
      dispositioned in the section itself. The template carried no box for this
      when the sweep was done — the Harvest box below is the nearest and does
      not reach §Open, which is where a slice's unresolved candidates actually
      sit. Added with the user's endorsement, together with the clause in
      `AGENTS.md` §Close that gives it something to enforce.
- [x] **`notes.md` Harvest current; durable facts lifted to `docs/memory/`.**
      Nineteen new memories, plus two amendments where this slice **falsified**
      an existing one — `a-count-in-a-comment-is-a-claim-nothing-checks.md`
      claimed a stale `path:line` breaks when followed, and it does not; and
      `a-present-destroys-the-widget-it-writes.md` now carries `F-C6`'s
      two-path mechanism.
- [x] **`slice-009.md` stage set to `done`.**

**One process note for whoever opens the next slice.** This audit took **seven
sessions** against a plan of four, and the overrun was entirely in the review
loop: four rounds, each over the previous round's repairs, with the repairs
themselves costing about half the total. `docs/memory/audit-stage-needs-its-own-budget`
is confirmed for the fourth time. The thing that finally ended it was not
another round — it was replacing a reading agent with a twenty-line script,
which found three defects two careful readings had reported clean.

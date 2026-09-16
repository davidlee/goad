# Audit & reconciliation — Slice 007: the renderer grows a form

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Written 2026-09-15, before any evidence was gathered.** What follows is the
audit's declared line of attack, not its findings. It names what it intends to
check and — as load-bearingly — what it will not reach, so that a verdict later
cannot borrow authority from a check that was never run.

**Subject:** `9447973..1969a27` on `main` — the six execution phases of slice
007. `9447973` is the commit at which the design was resolved and the plan
accepted; everything after it is this slice's implementation.

**Question.** For 007 to be finished, seven things must be true. The audit
checks all seven, and the fourth is the gate:

1. **The ten acceptance criteria in `slice-007.md` are met**, each on evidence
   that could have come out the other way. AC-7 and AC-10 are already
   discharged below by a person and are not re-litigated; the other eight are
   walked from scratch against the code and the tests, not against the phase
   sheets' accounts of them.
2. **Every VT/VA/VH in `plan.md` is discharged**, across all six phases —
   `plan.md`'s Coverage table is a claim about which criterion carries which
   AC, and a criterion that is green for a reason other than the one the table
   names discharges nothing. PHASE-06 checked its own four paths; the
   slice-wide walk is this audit's, and it is not delegated to the phases'
   self-reports.
3. **The paths actually touched match the surfaces declared**, phase by phase.
   `notes.md` records one undeclared-path decision on the record (PHASE-04's
   D-3) and two Surfaces lines already known short. The audit diffs all six
   independently: a Surfaces line that is an under-count in four phases and
   exact in two is one finding about the class, and the class is what gets
   dispositioned, not six instances.
4. **`canon-delta.md` is promoted or abandoned, in writing.** This is the gate.
   CD-1 lands in `docs/specs/001-host-backend-protocol.md` as R-57 and R-58,
   each with a §7 verification row, plus the amendment to R-18's existing §7
   row — whose two stated facts PHASE-03 falsified. The audit will not close
   the slice holding an unpromoted draft, and will not write a character into
   `docs/specs/` without explicit user endorsement first.
5. **The five invariants hold in the shipped code**, by reading rather than by
   inference from a green gate. Specifically: no domain vocabulary in host
   types (and `group` is a hint key, not a host concept — it is **not** on the
   scanned word list, so the scan passing is necessary and not sufficient);
   normalization is still the only door; the renderer's `boolean`-only subset
   has not narrowed what the wire admits; a backend failure still cannot take
   the host down; and `src/semantics/` is unnamed by `src/shell/`. S-6's
   question — whether the draft has entered `Presentation` — is answered by
   reading `view_model.rs`, because **no instrument in the gate reaches it**.
6. **The code survives a full-strength adversarial review.** `review-code.md`,
   subject `implementation`, fresh agent, rounds unbounded until the repairs
   are themselves reviewed. The audit does not downgrade a blocker to clear
   the gate and does not defer a fix for being large.
7. **The record is true about what shipped.** Every divergence between a
   document and the code is dispositioned as *document stale* (amend, with
   endorsement), *code wrong* (a finding, fixed in this slice) or *neither*
   (a decision, taken to the user). `design.md` is not retro-fitted; where the
   implementation departed and the design stands, it is said so under *Design
   drift not reconciled*.

**The leads this audit declares in advance**, so that finding nothing is a
result rather than a shrug:

- **F-6 — the product clips its own second option at its preferred size.** It
  is open in two ways: unrepaired, and *unobserved by a person*, because under
  a tiling compositor the window never takes a preferred size. The audit
  reproduces it under an explicitly sized window before ruling on it, rather
  than trusting either the three fixtures that mask it or the demo that missed
  it.
- **Enumerations in this slice have been floors, not ceilings** — three
  Surfaces lines short, one complete. The audit treats every enumeration it
  meets as a claim to check, including `plan.md`'s Coverage table and the ACs'
  own citations.
- **Documents citing line numbers that have moved** — `design.md` §9/AC-1 and
  §8/R-5 on `scheduling.rs:95-125`, `canon-delta.md`'s *The gap* on
  `controller.rs:216`, `roadmap.md:313` and `slice-007.md:14` on a type that
  no longer exists. Each is a decision about which side is wrong, not a code
  defect, and each is dispositioned rather than tidied.
- **`design.md` §9 disagrees with itself** about whether every field test reads
  the wire or the screen. A rule and its own AC row cannot both stand; the
  audit takes it as a decision to the user.
- **`docs/memory/` is cited as if something checked it.**
  `path-flake-ref-breaks-on-demo-socket.md` is cited by `plan.md` and all six
  phase briefs and **does not exist**; `cite-requirements-not-finding-ids.md`
  is unenforced across at least eight files. Taken together as one claim about
  the directory, not as two typos.
- **Cheap refactors with no criterion behind them**, carried to the review pass
  rather than taken quietly: `wiring.rs`'s `accessible_enabled_of`, the
  viewport sizer's three copies, `tree.rs`'s two `OptionRow` literals, and the
  two cases sharing one invocation log under one pid.

**What this audit does not reach, stated here so no later line implies it
did.** `just check` exiting 0 is not evidence that stratum 3 is pure — none of
ADR-001's four instruments reaches it (POL-001 §Verification) — and it is not
evidence that `group` stayed a hint key, because `group` is not on the
vocabulary scan's word list. Both are held by reading and by review, and the
Verdict says so in those words or not at all.

<!-- This is the audit's scope — evidence, criteria, canon. The code review's
     own lines of attack belong in `review-code.md`'s Brief, not here. -->

## Evidence

<!-- What was run and what it said. Not a claim of correctness — the basis for
     one. -->

### Tests and checks

`just check` at `1969a27`, run by this audit rather than taken from the
handover: **exit 0, 535 passing cases across 21 test binaries.** That is the
slice as the phases left it, and it is the audit's starting condition.

After the review repairs (F-1, F-2, F-3, F-5 below): **exit 0, 537 cases across
21 binaries.** The two additions are the two new cases the repairs required; no
case was removed and no assertion was weakened.

The gate caught two defects in the audit's own repair work before they could
land — `std::collections::HashSet` is a disallowed type in this workspace, and
two bindings shadowed — which is the gate doing its job on the auditor.

**The gate's determinism was itself measured**, because it turned out not to
hold. The renderer target was run **six consecutive times: 6 green, 0 red.**
Before F-5's repair the same six runs produced one failure — see F-5. A gate
that is green five times in six is not a gate, and the number is here rather
than in the ledger because it is evidence about `just check`, not about a case.

**What the green does not reach**, restated from the Brief because a verdict
must not borrow it: no ADR-001 instrument reaches stratum 3's purity (POL-001
§Verification), and `group` is **not** on the vocabulary scan's word list —
confirmed by reading it, `crates/goad-boundary/tests/checks/vocabulary.rs:18-26`
lists `habit`, `streak`, `journal`, `site`, `goal`, `reminder`, `compliance` and
nothing else. Both are held below by reading.

### Acceptance criteria

All ten met. AC-7 and AC-10 are the user's own account and are below, unchanged.
The other eight were walked against the code, not against the phase sheets:

| AC | verdict | the evidence, and what would have falsified it |
|---|---|---|
| **AC-1** | met | `fields::every_drawn_field_of_the_pressed_option_reaches_the_wire_as_the_screen_showed_it` reads the request off the backend's own invocation log. Falsified by a key count that is not N, or a value that is not the JSON boolean the box showed |
| **AC-2** | met, and its third link now holds | The rule is `mapper::a_block_starts_wherever_the_group_value_changes_and_runs_are_never_merged`, with `::grouped_fields_separated_only_by_an_undrawn_field_are_one_block` and `::a_group_whose_every_field_is_undrawn_produces_no_block` on the edges. Read rather than inferred: `view_model.rs::blocks_from` sorts nothing, merges nothing and appends only — and the `Drawn` type is what makes *a run is over the drawn fields* structural, since an undrawn field never becomes one and so cannot break a run. **The chain's third link held the fields and not the heading** until this audit's F-1 repair; it does now, in both of the markup's branches |
| **AC-3** | met | `mapper::every_undrawn_kind_is_reported_by_option_field_and_form` asserts the `Undrawn` value, which is what the criterion names; `reception::a_view_carrying_an_undrawn_field_reaches_the_diagnostic_surface_through_receive` carries it to the surface; `fields::a_view_carrying_an_undrawn_field_is_still_shown_and_still_answers_its_drawn_keys` is R-55 at the wire. The rendered wording is held by review, as this project holds every diagnostic wording |
| **AC-4** | met | `fields::a_field_id_shared_by_two_options_is_two_keys_and_only_the_answered_ones_are_sent`. The shared field id is what makes it discriminating: with an unscoped selector the case would be green both where the design is right and where the draft key's option half is ignored |
| **AC-5** | met, both halves | `fields::a_present_that_changes_nothing_leaves_a_half_filled_form_on_the_screen_and_on_the_wire`. Neither half implies the other, which is the criterion's own point |
| **AC-6** | met | Checked by reading the diff, not by the tests passing. `table.rs`'s whole slice-wide diff is four `frame()` → `frame(false)` call sites and one `rustfmt` reflow — **no assertion's claim changed**. The `OptionRow` literal changes are the mechanical member the criterion anticipated. `FieldRow` lost a member during this audit (F-3), which touched the builder and no assertion |
| **AC-8** | met at this audit | Promoted: R-57 and R-58 are in SPEC-001 §4 with a §7 row each. The vehicles the §7 rows name are the ones AC-8 named — `draft.rs::tests::a_boolean_field_submits_a_json_boolean` and, for R-58, `wiring::editing::an_answer_carries_a_value_for_every_drawn_field_of_the_option_it_names` with `::an_answer_carries_no_value_for_another_option_or_for_an_undrawn_field`. The §7 R-58 row adds the wire vehicle PHASE-05 built, which AC-8 was written too early to name |
| **AC-9** | met, with its own qualifier intact | `just check` exit 0; the vocabulary scan and the four ADR-001 instruments pass. AC-9's text already says this is necessary and not sufficient, and the word list above is why |

### Verification criteria

**37 across six phases** — PHASE-01 four, 02 seven, 03 eight, 04 nine, 05 five,
06 four. All discharged.

`plan.md` says of itself that *"nothing in the gate checks this document against
itself"* and names the check: every `PHASE-0N/<id>` cross-reference resolves,
no id sequence has a gap or a duplicate, every count matches the list it
describes. **This audit ran it.** Result: no duplicate, no gap, and **zero
dangling cross-references** across the whole document. The 37 above is the
count, recomputed from the lists rather than copied from the prose.

The vehicles were checked to exist and to be named for what they assert, not
merely to be green — `mapper.rs` carries sixteen cases whose names are the rule
they hold, including the three edges (`a_group_hint_that_is_not_a_string_is_…`,
`an_absent_group_and_an_empty_one_are_…`,
`a_field_that_is_both_undrawn_and_badly_grouped_is_reported_twice`).

**One correction to the phases' own account.** `notes.md` Harvest recorded
PHASE-05's Surfaces line as *"the first complete Surfaces line in this slice …
those are exactly the four paths touched."* It is not: PHASE-05 also touched
`crates/goad/tests/renderer/tree.rs`, which its **own D-3 names** as an
undeclared surface in the same sheet. The sheet contradicts itself — D-3 counts
five paths and F-5 counts four — and the Harvest lifted the wrong one. See the
surface delta.

### Surface delta

Per phase, paths touched against `plan.md`'s Surfaces line, computed from the
commits rather than from the sheets:

| phase | declared | touched | undeclared |
|---|---|---|---|
| **01** | 11 | 12 | `crates/goad/ui/app.slint` — its own EX-7 sends the phase there |
| **02** | 4 | 5 | `crates/goad/tests/renderer/wiring.rs` — named in S-2's fourth allowance, absent from Surfaces |
| **03** | 6 | 6 | **none — exact** |
| **04** | 8 | 11 | `src/draft.rs` (its EX-3 sends it there), `tests/renderer/{harness.rs,tree.rs}` (D-3, referred up and approved during execution) |
| **05** | 4 | 5 | `tests/renderer/tree.rs` — the other half of the same helper lift, recorded in its D-3 and missed by its F-5 |
| **06** | 4 | 4 | **none — exact** |

No path was touched that `slice-007.md` §Scope does not admit — it names
`crates/goad/tests/renderer/` as a directory — so **there is no scope breach**.
Every undeclared path is either a criterion sending the phase somewhere its own
Surfaces line forgot, or a decision referred up and recorded at the time.

The finding is about the class, and it is sharper than the phases could see.
**Five of six Surfaces lines are short; only PHASE-03's and PHASE-06's are
exact** — and PHASE-06's is exact partly because two of its four paths are
documents. The Harvest offered PHASE-05 as the negative case that bounded the
class to *enumerations go stale*. The negative case is itself an instance, so
the class is not bounded that way: **in this slice, every Surfaces line that had
anything to omit omitted something.** A phase cannot check its own Surfaces line
against the paths it is about to touch, because it writes the sheet before it
does the work and nothing re-reads it afterwards — which is why this is the
audit's finding four times over and never a phase's.

Declared-but-untouched: none, in any phase.

<!-- Everything above is the audit's to gather. The two entries below are
     PHASE-06/EX-5: the *human* evidence, which exists only at the moment a
     person ran the software and which a later agent cannot reproduce. They are
     written here rather than left in `notes.md` because `docs/AGENTS.md`
     §Tiers requires it, and they are the user's account and not the
     implementer's. `notes.md`'s PHASE-06 sheet carries the fuller record,
     including what the user said that was not actioned. -->

### AC-7 — a person filled the form and read the record

**Observed by the user, 2026-09-15**, on `just demo` against
`examples/shell/backend.sh`, on a niri (Wayland) session. The implementing agent
did not see the window; the account below is the user's.

The demo's form carries six fields on one option: five `boolean`, drawn, in
three blocks — one ungrouped, then two `group` values — and one `text` field,
`note`, of a kind this renderer does not draw. The second option carries none.

1. Before answering, the user opened the diagnostic surface from the tray and
   confirmed the undrawn report: *"diagnostics - TIL it has a right click menu.
   confirmed"*. That is **AC-3's human half** — the field is reported, the view
   is still shown, and the option still answers.
2. The user ticked three of the five boxes by mouse, left two alone, and
   pressed the option's button **once**.
3. Reopening the diagnostic surface, the record read — verbatim, from the
   user's screenshot:

   ```
   stderr: answered 2026-09-15T09:26:36.425909399Z#1: option yes, values {"desk":true,"focused":false,"outside":false,"started":true,"tired":true}
   ```

   and in the user's own words: *"the bools are correct, no note"*.

**Five keys from one exchange**: one for each field the renderer drew of the
option answered, `true` for the three ticked and `false` for the two left alone,
and **no key for the undrawn `text` field**. AC-7 met, and with it a person's
sight of AC-1, AC-3 and `canon-delta.md`'s R-58.

### AC-10 — a person judged the look, and the bound held

**Observed by the user, 2026-09-15**, over two builds.

The first build was reported unusable in one specific way: the layouts
distributed the window's full height between the fields, so two members of one
block sat further apart than two blocks did. Actioned **inside AC-10's bound**
— `alignment: start` on the block container and on each block, so the declared
spacing states grouping rather than a minimum. The user on the second build:
*"form spacing looks a lot saner now"*.

Asked the criterion's two questions directly — whether the fields visibly belong
to the option that answers rather than to the one below, and whether each
heading visibly covers its own fields and not the ungrouped one above — the
user's acceptance was:

> *"i'll call it legible enough for now, nothing that can't survive until
> holistic design work"*

**AC-10 met, and the qualifier is part of it:** this is legibility sufficient to
close 007, deferring explicitly to the holistic pass that is 008. Quoting the
acceptance without the qualifier would overstate it.

What the user said that fell **outside** the bound was recorded verbatim and not
actioned, per PHASE-06/EX-4 and `plan.md` S-8 — no word wrap and no text
selection on the diagnostic surface, the vertical distribution above the block
container, and the title clipped at the top of the window. All four are in
`notes.md`'s PHASE-06 sheet and are 008's brief. The bound was on what could be
changed, never on what could be said.

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`
- **State:** resolved · **outstanding blockers: none**
- **Rounds:** three. Round 1 attacked the slice's code and raised F-1 to F-4;
  the audit raised F-5 independently. Round 2 attacked round 1's **repairs** and
  raised F-6 to F-10 — five defects in five repairs, including two false
  statements in the ledger's own Response. Round 3 attacked round 2's repairs.
- **Ten findings: two major, seven minor, one nit.** Nine `fix-now`, one
  `follow-up` (F-4, landed in `slice-007.md` §Follow-ups, which is what that
  disposition owes). None downgraded, none deferred for being large.
- Every repair was verified by **making the defect and watching the test fail**,
  and the mutation and its result are in the finding. That is the only discharge
  this project accepts for a claim about what a test holds.

## Verdict

**The slice closes.** It did what it set out to do, and the thing it set out to
do was the harder half of what it looked like.

007 read as *draw some checkboxes*. What made it tier 2 is that drawing a field
forces the host to say what a submitted value **is**, and no rule said. That gap
is closed by R-57 and R-58, in the spec, with verification rows — not by a
convention the code happens to follow. The distinction matters here more than
usual: this project exists to avoid a protocol that acquires the shape of one
renderer, and R-57 types all five field kinds while this renderer draws one.
That was a live temptation with a concrete case behind it, and the slice did not
take it.

**What the audit is confident about, and why.** Not because the gate is green —
it was green at every phase boundary and was green while the whole of the
heading markup could be deleted without a test noticing. The confidence comes
from three things the gate does not supply: R-58 is held by the **shape** of
`answer()`'s walk and by `Draft` exposing no enumeration, so the rule has no
check to forget; each repair was discharged by making its defect and watching a
test fail; and a person filled the form and read five correct keys off one
exchange.

**What is being accepted knowingly.**

- **The window has no content-derived preferred size** — 50×65 regardless of
  content, measured. It is a real defect, it is open, and 008 owns it. It is
  accepted here because window sizing was 007's declared non-goal from scoping,
  and because closing it properly is a layout decision rather than a patch. What
  changed at audit is that 008 inherits a mechanism (`ScrollView` absorbs the
  preferred size; nothing declares one) instead of a symptom.
- **Stratum 3's purity is held by reading and by review, not by an instrument.**
  This is ADR-001's known boundary, POL-001 states it, and the slice's S-6 named
  the specific temptation — the draft entering `Presentation`. Read and clean:
  `view_model.rs` carries no `checked`, no mutable member, no widget handle.
  That is a person's assurance, and the audit says so rather than implying the
  gate reached it.
- **`group` is not on the vocabulary scan's word list**, so the scan passing
  says nothing about the slice's one new hint key. Held by review: `"group"`
  appears at exactly **one** non-test site in the workspace,
  `view_model.rs:188`, which is the renderer — the one component R-18 permits to
  branch on a hint.
- **R-57's four undrawn kinds are review, not a test**, and will stay so until a
  renderer draws them. The §7 row says this in the spec's own established
  convention rather than in a slice document.
- **F-4's unbounded diagnostic lists.** A conforming backend can make the
  undrawn report linear in a number it chooses. Nothing crashes, the class is
  pre-existing, and it is a follow-up rather than a patch because bounding one
  list and not the others would be fixing the instance.
- **Three states this slice's tests do not reach.** No round forced a genuine
  `Full` on the one-slot channel in a running `serve`, killed a backend mid-form
  with a draft outstanding, or drove a real ingress arrival at a window holding
  a half-filled form. All three are expressible with the existing harness, so
  this is budget and not tooling, and the second is the one that is 007's own:
  it is the state this slice introduced. Also unreached: `choice`, `number` and
  `datetime` are covered as mapper values only — nothing drives one through
  `serve` to a `respond`. This is stated here, and carried to Follow-ups,
  because the alternative is a silence a reader would take for coverage. It is
  the honest gap in this audit's evidence and nothing above borrows from it.

**What this slice should be remembered for.** Five rounds of code review found
twenty-four defects that six phases and a green gate did not, and **round 2
found five of them in round 1's repairs** — made carefully, by the agent holding
the whole audit, immediately after reading the standard they failed to meet.
That is the argument for unbounded rounds, demonstrated rather than asserted: a
review that stopped at round 1's repairs would have shipped those five. Rounds 3,
4 and 5 then found eleven more, eight of them in the ledger's own prose.

The two code classes are lifted to `docs/memory/`: a claim held at the row model
is not held at the screen, and an assertion in the direction an unbound property
already answers discriminates nothing. The third class is about the record
rather than the code, and rounds 2, 4 and 5 each supplied it: **the false
statements were not careless transcriptions but small strengthenings of claims
that were already true** — a scope widened, an enumeration substituted for a
class, a count carried across the edit that moved it. Prose asserting more than
it holds is the same defect as a test asserting more than it holds.

And the gate was not deterministic while everyone treated it as the arbiter —
one run in six, from two cases sharing a temp path. It is held by an instrument
now, and that instrument kept earning itself: a second collision on its first
run, then a third and fourth helper making the same unheld promise, then a fifth
and sixth. It reaches four of the six. **Four successive rounds each wrote down
the list of helpers they had found, and three of those lists were short within
the round that wrote them** — the last because a helper of the class need not be
called `socket_path`, need not mint a socket, and need not live under `tests/`.
The instrument's doc now carries the class and the grep that enumerates it
instead of a list, which is the durable repair; the remaining two are a
follow-up with a measured obstacle.

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

Every row below was **endorsed by the user before it was written**, 2026-09-15.
No canon was touched otherwise.

| document | change | reason | done |
|----------|--------|--------|------|
| `specs/001-host-backend-protocol.md §4` | **R-57** added — a submitted value's JSON type is fixed by the field's `kind`, for all five kinds | `canon-delta.md` CD-1, promoted. Drawing a field forces the host to state this and no rule did; §6.1's single example was the only evidence anywhere | [x] |
| `specs/001-host-backend-protocol.md §4` | **R-58** added — a `respond` carries values for exactly the fields the host drew of the option answered | CD-1. Two claims, so two rows: R-52/R-53 are the precedent. R-8 fixes the map's *shape* and not its totality; R-35 forbids *refusing* an incomplete answer and is silent on producing one | [x] |
| `specs/001-host-backend-protocol.md §6.1` | the `respond` example carries three values, one of them `false` | CD-1. One value implied a one-key map; the map is over every drawn field, and a `false` there is a drawn-and-unticked box rather than a default | [x] |
| `specs/001-host-backend-protocol.md §6.2` | a kind→JSON-type table beside *Field forms* | CD-1. R-57 restated where a backend author meets the field object's keys; normative text stays in §4 | [x] |
| `specs/001-host-backend-protocol.md §7` | verification rows for R-57 and R-58 | AC-8. R-57 at `draft.rs::submitted`, its single enforcement site, with the four undrawn kinds review-not-test; R-58 in both directions separately plus the wire vehicle PHASE-05 built | [x] |
| `specs/001-host-backend-protocol.md §7` | **R-18's existing row amended** — evidence only; verdict and requirement untouched | The row claimed `hints` is read in `src/` only by `normalize.rs`, and that *"the renderer, the one component that may, does not exist yet."* PHASE-03 falsified both. `view_model.rs::present` is now the second and only other reader, of exactly one key, and it is the renderer | [x] |
| `specs/001-host-backend-protocol.md §8` | **OQ-4** added — a date without a time | CD-1. What remains open once R-57 types `datetime` as an RFC 3339 `date-time`. SPEC-001's numbering; not SPEC-002's separate OQ-4 | [x] |
| `slices/007/canon-delta.md` | two copy-edits **before** promotion | R-57's drafted text carried two `and`s; *The gap* still said `controller.rs:216` "sends an empty map", which it has not since PHASE-04. A draft is promoted verbatim, so it is corrected first rather than silently diverging from what landed | [x] |
| `roadmap.md` §007 | marked **done**; the stale `Undrawn::OptionFields` reference rewritten | A live planning document, not canon. That bullet now describes finished work, and named a type that no longer exists. The scoping argument is kept because 008, 009 and OQ-2 still rest on it | [x] |
| `docs/memory/` | six entries written | Durable facts lifted from the Harvest, per §Close. `path-flake-ref-breaks-on-demo-socket.md` (cited by `plan.md` and all six phase briefs and **never written**), `a-count-in-a-comment-is-a-claim-nothing-checks.md`, `a-fixtures-size-is-not-the-products.md`, `a-test-rule-binds-to-a-defect-not-a-surface.md`, and two from rounds 4-5: `enumerate-the-class-not-the-instances.md` and `a-false-claim-is-usually-a-true-one-strengthened.md` | [x] |
| `slices/007/notes.md` §Harvest | PHASE-05's F-5 corrected; F-6 replaced with the audit's measurement | F-5 claimed PHASE-05's Surfaces line was the slice's first complete one. It is not, and the same sheet's D-3 says so | [x] |
| `slices/007/slice-007.md` | Summary and Follow-ups written; stage `done` | §Close | [x] |

**Not amended, and each checked rather than assumed:** SPEC-002 (OQ-4 stays
open — this slice answered it and **withdrew** the answer under D13, and
reconciliation must not read that history as having settled it); SPEC-003 (no
stimulus, no envelope, no listener change); ADR-001 and ADR-003 (nothing new
crosses a crate edge, `goad-semantics` unchanged); ADR-004 (not applicable to
the diff — its interaction with the deferred hold is carried in the Harvest and
in §Follow-ups, not filed under "not applicable"); POL-001 (`just check`
unchanged, its command block untouched).

**Design drift not reconciled.** `design.md` is a record of intent at a point in
time and has **not** been retro-fitted. Four divergences stand, each by
decision:

1. **§9's closing rule is narrower than its own purpose.** It says *"Every field
   test either reads the wire or asserts something about the screen — except
   AC-5"*, and `plan.md`'s S-7 restates it. PHASE-04's `wiring::editing` cases
   read `answer()`'s return — neither — while `plan.md`'s sequencing cites §9 as
   its authority for doing so. They are not proxies: walking the draft instead
   of the drawn fields reddens them, which is exactly the defect the rule's
   stated purpose names. So the rule's **letter** excludes tests its **purpose**
   wants. The design stands; the sharper rule — bind to the defect, not the
   surface — is lifted to
   `docs/memory/a-test-rule-binds-to-a-defect-not-a-surface.md`, which is where
   the next slice will meet it.
2. **§9/AC-1 and §8/R-5 cite `scheduling.rs:95-125`** for `logging_scripted`,
   which PHASE-05's EX-2 moved to `harness.rs:172`. Two line cites in a record
   of intent, not claims about behaviour.
3. **§9/AC-8's row names two stratum-3 vehicles** and does not name the wire
   vehicle PHASE-05 later built. It was written before that phase existed. The
   §7 row now in SPEC-001 — which is canon and current — names all three, so the
   gap is closed where it matters and the design is left as the forecast it was.
4. **§6's OQ-4 resolution line contradicts §5.5's edge table**, in the same
   document, about what an empty `group` means. §5.5 says `"group": ""` is an
   **untitled block**, not reported; §6's one-liner says `""` is **ungrouped**
   and unreported. Those are different behaviours: `Run::key` gives `Named("")`
   the key `Some("")` and `Ungrouped` the key `None`, so an empty-group field
   adjacent to an ungrouped one produces **two** headingless blocks rather than
   one — which is what §5.5 says, what the markup's comment says, and what
   `mapper::an_absent_group_and_an_empty_one_are_blocks_with_no_heading` pins by
   name. The code is right and §5.5 is right; §6's summary of it is not, and was
   not when it was written. `slice-007.md`'s OQ table carried the same error and
   **is** corrected, because a slice card is current truth and is the first
   thing a future reader opens. §6 stands, recorded here.

5. **§5.2's *The build* quotes a code block that does not lint.** It quotes
   `std::env::var("SLINT_STYLE")` verbatim; `clippy.toml` disallows the call and
   the gate is `-D warnings`. The design's *reasoning* for the explicit read is
   sound and survives untouched — only the implicit claim that the quoted block
   compiles clean was never true. The instance is closed (PHASE-02 D-2); the
   class is that **a design quoting a code block has asserted that block lints,
   and nothing checks it until a phase runs.**

`slice-007.md` §Purpose is likewise left as the statement of the problem the
slice opened on, including its reference to `Undrawn::OptionFields`. It is what
was true when the slice was scoped; §Summary states what is true now.

## Closure

- [x] **All findings dispositioned; no blockers outstanding.** **Twenty-four
      across five rounds**, every one `verified`. Twenty-two `fix-now`, two
      `follow-up`, both landed in `slice-007.md` §Follow-ups. No `blocker` was
      raised, no severity was downgraded to clear the gate, and no fix was
      deferred for being large — F-17's and F-22's deferral is a *measured*
      obstacle (the target cannot include the instrument without failing the
      gate), not a size. Rounds 4 and 5 found no code defect; the stopping rule
      and its reasoning are in `review-code.md` §Brief and §Synthesis rather
      than left implicit.
- [x] **All acceptance criteria met**, none waived. AC-7 and AC-10 by a person,
      2026-09-15; the other eight walked against the code in §Evidence. AC-2's
      third link was vacuous for the heading and now holds, in both of the
      markup's branches.
- [x] **Tests and checks green.** `just check` exit 0, **537 cases across 21
      binaries**. Determinism measured, not assumed: six consecutive runs green,
      where the same six produced one failure before F-5's repair.
- [x] **Specs reconciled, with explicit user endorsement before anything was
      written.** Seven edits to SPEC-001, itemised above. Five other canon
      documents checked and correctly unamended.
- [x] **`canon-delta.md` promoted** — CD-1 landed as R-57 and R-58 with a §7 row
      each, plus R-18's evidence amendment and OQ-4. Nothing abandoned. The
      slice does not close holding an unpromoted draft, and it is not.
- [x] **`slice-nnn.md` Summary and Follow-ups written.** Thirteen follow-ups,
      each with the decision behind it recorded here or in `review-code.md`.
- [x] **`notes.md` Harvest current; durable facts lifted to `docs/memory/`.**
      Six entries, one of which seven documents had already been citing for the
      length of the slice without it existing. The last two are the review's own
      result rather than the code's: enumerate the class and the grep that finds
      it, never the instances; and a false claim in a repair is almost always a
      true one that was strengthened, which is why re-reading does not catch it
      and re-measuring does.
- [x] **`slice-nnn.md` stage set to `done`.**

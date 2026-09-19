# Plan log — Slice 009

Append-only, time-ordered. What was asked, what the user decided, and why.
Decisions only: findings live in `review-plan.md`, current truth lives in
`plan.md`.

**Id sequences.** This slice already carries three that collide —
`design.md` §7's `Dn`, `design-log.md`'s `D-n` and `prototype-notes.md`'s `P-n`.
This file adds none: entries are cited by their dated heading.

---

## 2026-09-19 — the plan is accepted, and the adversarial review is not run

**Raised.** `docs/AGENTS.md` §Plan puts two things to the user in order: whether
to subject the plan to adversarial review in `review-plan.md`, and then
acceptance. Both were put, with the observation that the design review had found
a blocker in three consecutive rounds and so a plan review is not a formality.

**Decided.** User: *"accept the plan, and begin phase 01"*. `plan.md` as written
at `44fbd8e` is accepted, and **no plan review runs**. `review-plan.md` is not
created.

**What that costs, recorded rather than argued.** The plan is unreviewed, so the
first adversarial reading of it happens at `review-code.md`, after the code
exists. Two things partly cover it. The plan stage was itself a verification
pass — it re-derived the design's `path:line` citations from `grep -n` and found
the sixth known-bad one (`design.md` §8 R5's `controller.rs:753-761`, corrected
to `:661-675`), which is the class of defect a review round finds. And the phase
plan is a second reading per phase: `AGENTS.md` requires that expanding a phase
into a sheet stop and go back rather than quietly repair, so nine expansions each
get a chance to catch what a review would have.

What is **not** covered is the plan's shape — the phase boundaries, the ordering
argument, and whether the Coverage table's mapping actually discharges each
acceptance criterion. Those were checked once, by the agent that wrote them. If
a phase boundary turns out wrong, the signal will be a phase that cannot end
green, and the response is to go back to plan rather than to widen the phase.

**Stage** moves to `executing`, and PHASE-01 begins. Its entry criteria are
discharged: EN-1 by `design-log.md` D-37 and this entry, EN-2 by `just check`
exiting 0 on a clean tree at `44fbd8e` (run 2026-09-19).

## 2026-09-19 — PHASE-02's Surfaces line was short by one file

**Raised.** PHASE-02's EX-3 requires that *"`Edited` and everything above it
drop the `Eq` derive"*. `Command` is above it (`wire.rs:18-28` carries an
`Edited` in `Command::Edit`), so the criterion compels an edit to
`crates/goad/src/wire.rs` — and PHASE-02's **Surfaces** line did not name that
file. The executing agent made the edit, correctly: one derive line and the doc
that says why (`ec0c7bf`).

The exposure is smaller than it looks. `slice-009.md` §Scope **does** name
`wire.rs`, and that card is what `audit.md`'s surface diff is read against, so
the audit's strongest instrument was never going to see an undeclared path.
What was wrong is the phase-level record: a phase whose Surfaces list is short
cannot be used to say what a phase was allowed to touch.

**Decided.** Amend, rather than leave it to audit's Reconciliation table. User:
*"proceed"*, on a recommendation to amend. `plan.md` PHASE-02 §Surfaces gains
`crates/goad/src/wire.rs`, scoped to *the `Eq` derive on `Command` and its doc,
nothing else*, and dated. This edits an accepted artefact, which is the only
reason it was the user's call and not the plan stage's.

**Noticed while doing it, and not acted on.** PHASE-03's EX-3 reads *"`Command::Edit`
carries a `Reported`, and `Command` drops its `Eq` derive with `Edited`"* — and
the second clause is **already true on entry**, discharged by PHASE-02. The two
criteria overlap rather than conflict: dropping `Edited`'s `Eq` breaks `Command`'s
derive in the same compile, so it could not have waited for PHASE-03. Left as
written, because criterion ids are immutable and a criterion that is already true
is discharged, not failed. PHASE-03's agent is told so up front.

## 2026-09-19 — three more Surfaces lines, and the class behind all four

**Raised.** PHASE-03's agent stopped twice rather than deciding alone: once on
`crates/goad/src/install.rs` being absent from PHASE-08's and PHASE-09's
Surfaces, and once on `crates/goad/tests/renderer/tree.rs` being absent from its
own. Both verified before acting.

- **`install.rs`** is named at `plan.md` PHASE-03, PHASE-05 and PHASE-07 and
  nowhere else, yet PHASE-08 draws the two `number` controls and PHASE-09 the
  `ComboBox`, and each needs its own arm in the `edited` mapper PHASE-03 writes.
- **`tree.rs`** holds the only case outside the declared Surfaces that widening
  `callback edited` breaks: `activating_a_field_control_fires_edited_with_all_four_selectors`
  binds `window.on_edited` directly, and `type EditedArgs` holds its arguments.
  Confirmed by instrument: `grep -rn "on_edited" crates/goad/` returns exactly
  two sites, `install.rs:43` and `tree.rs:435`.

**Decided.** Amend all three, dated and scoped, under the endorsement the user
gave for PHASE-02's: *"amend ph02 surfaces as you recommend"*. The `tree.rs`
repair is PHASE-03's to make — the fourth element of `EditedArgs` becomes a
`FieldEdit` and the assertion reads the `checked` slot — and it strengthens
rather than weakens the case, which exists to prove all four selectors arrive.

**The class, which is the part worth keeping.** Four Surfaces lines have now
been short, and it is one cause rather than four mistakes. The plan's Surfaces
lines were derived from `design.md` §9's enumeration, and §9 enumerates
**constructors** — *"twelve in `tests/renderer/wiring.rs`, ten in `draft.rs`'s
own tests, and the one closure in `install.rs`"*. A **binder of a markup
callback** is not a constructor, and neither is a file that only has to change
because a type above it changed (`wire.rs`'s `Eq`). So the enumeration was
accurate and its reach was not what the Surfaces lines needed. §9's count of
twelve was never wrong: `tree.rs` is a thirteenth site the count never claimed
to cover.

**How far the class reaches, bounded rather than feared.** `tree.rs` is the only
test file that binds a markup callback at all — it binds both `on_edited` and
`on_chosen`, and no other file under `crates/goad/tests/` binds any. Of the
phases still to run, only PHASE-03 changes a markup callback's **signature**;
PHASE-05 changes `Command::Choose`'s shape host-side and leaves
`callback chosen(string, string)` alone, and PHASE-07 through PHASE-09 add
*fields* to `FieldEdit`, which does not break a binding that reads one slot.
PHASE-09 already names `tree.rs`. So the sweep is complete and no further phase
is exposed by this cause.

No criterion changed in any of the three phases, and no id was renumbered.

## 2026-09-19 — a doc the phase makes stale is amended in that phase

**Raised.** PHASE-03's EX-5 widens what earns `Refused::UnknownField`: it is now
also what a report `interpret` cannot make sense of earns — an index no
alternative has, a non-finite slider value, or a report whose variant is not the
drawn field's kind. The variant's own doc at `crates/goad/src/diagnostics.rs:61-66`
describes only the old path, and `diagnostics.rs` was not in PHASE-03's Surfaces.
No behaviour changes: the variant and the rendered line are both untouched, and
the line — *"could not match that control to a field of the option it names"* —
reads correctly for the new path too.

**Decided.** Amend, in this phase. **The slice has already settled this class
twice and both times the answer was the same**: `design.md:1042` says of the
`Glass::present` contract *"The trait's doc is code and is amended in the phase
that lands this"*, and PHASE-04/EX-5 does exactly that for `clock.rs:47-53`
rather than leaving it to audit's Reconciliation table. A divergence *discovered*
at audit belongs in that table; one the slice creates knowingly does not.
PHASE-03 §Surfaces gains `crates/goad/src/diagnostics.rs`, scoped to that doc
comment alone.

Leaving it would have stood a stale doc through PHASE-05 to PHASE-08 — which are
precisely the phases that make the new path reachable in production — for the
sake of reaching `diagnostics.rs`'s natural home in PHASE-09.

**Two clauses are stale, not one.** The agent named the first sentence. The
fourth is worse: *"Only reachable from a stale or malformed callback"*. After
EX-5 a well-formed, current callback reporting a non-finite slider value earns
this refusal, and that is neither stale nor malformed.

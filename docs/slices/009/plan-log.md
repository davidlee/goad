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

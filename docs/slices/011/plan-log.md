# Plan log — Slice 011

Append-only record of the plan *conversation* — what was asked, what was
decided, in time order. Never rewrite an entry; supersede it with a later one.

Only decisions live here. An adversarial review of the plan, if one runs, is
owned by its ledger (`review-plan.md`).

## Decisions

### 2026-09-26 — the planner's own decisions, under the autonomy grant

The planning agent took each decision below as its own. None touches canon,
and the orchestrator granted that authority (`design-log.md`, *the autonomy
grant*). None is in `design.md`. Each is open to the user at plan acceptance.

- **PL-1 — red and green share a phase.** Every phase ends green on
  `just check`, so T2 and T4 are written and seen red in PHASE-02, in the same
  phase as the loop that turns them green. There is no separate "red" phase.
  *Rejected:* a red phase committed with failing or `#[ignore]`d cases. That
  would break the gate, or hide the cases from it.
- **PL-2 — T3 and VT-7's added assertion are green-first.** Traced at plan,
  both pass on today's loop, so neither has a red before the change. Their
  evidence is the mutation table (M5, M6 and M8 for T3; R2 for VT-7). The
  phase sheets must say so rather than claim a red. T4's red on today's loop
  is its precondition. That is recorded, but it is not T4's evidence; M7 is.
- **PL-3 — T1 lands in PHASE-01.** Removing `CountingGlass` there leaves VT-5's
  third assertion with nothing to count. Dropping it at once, which is the
  design's end state, beats moving it onto the log and deleting it a phase
  later. The per-refusal cost goes unasserted for one phase, in which the loop
  does not change.
- **PL-4 — M0 runs twice.** It runs in PHASE-01 over T3 and VT-7, to prove the
  recorder before anything depends on it, and in PHASE-02 over T2–T4, as
  `design.md` §9 names.
- **PL-5 — how the mutations are spelled.** `unused` is `deny`, so M0 is
  spelled `drop(lines);` rather than a deleted call, and M1 is spelled
  `if !surface_stale { continue 'serving; }` rather than a deleted write. Both
  natural spellings fail to compile, and a control that does not compile
  proves nothing.
- **PL-6 — T4 pins the schedule first**, with a `Command::Evaluate` answered a
  minute off, as T2 does. Otherwise `serve`'s initial arm, at
  `MINIMUM_SPACING`, is a firing the case would have to argue cannot land
  inside it. The pin keeps A on the leading edge, because
  `next_refusal_present` has never fired.
- **PL-7 — load is PHASE-03's.** The phase measures margins at the bound, ten
  runs at rest and ten at the oversubscription slice 003 used, with temporary
  instrumentation copied in and out. It runs the whole renderer target, as the
  gate does. A red under load is a STOP, and the bound is not widened. M9 runs
  at least ten times each way. At rest every run must red. Under load at least
  one must red, and a run that stays green is recorded rather than failed:
  review Round 4 predicted it, and the plan's trace puts the threshold near a
  25 ms per-firing lag. The phase does not start its load until the
  orchestrator confirms that no other session is measuring on the machine.
- **PL-8 — AC-6 is run in PHASE-03, against a committed writer.**
  `docs/slices/011/flood.py`, standard library only, lets a person, and the
  auditor, reproduce the flood after the research scratchpad is gone. The
  person runs it in PHASE-03, so that a usability problem surfaces before
  audit. Audit cites the observation in `audit.md` §Evidence, which is where
  AC-6 requires it.

### 2026-09-26 — plan accepted, with no plan review

The user accepted `plan.md` as committed at `d2617c1`, PL-1..PL-8 standing,
and declined a `review-plan.md` round — the 010 precedent. The orchestrator
writes each phase sheet, so the plan's one adversarial reading before code is
by someone who will not execute it.

# Plan — Slice NNN: {title}

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

<!-- Phase ids (PHASE-NN) and criterion ids (EN-/EX-/VT-/VA-/VH-N) are
     immutable: edits append, never renumber, so the sequence goes
     non-monotonic after a split and that is expected. Criterion ids are local
     to their phase — cite another phase's phase-qualified (PHASE-03/EX-2).
     Verification modes — VT: automated test. VA: agent check. VH: human
     acceptance.
     Progress is NOT recorded here. Status lives in `notes.md`. -->

## Overview

<!-- What the whole plan achieves and how the phases add up to the slice's
     acceptance criteria. -->

## Sequencing & rationale

<!-- Why this order: what each phase unlocks, where the dependencies are, and
     which phases could be reordered or dropped. Size each phase so one agent
     can finish it in one session, bookkeeping included. -->

## Coverage

<!-- Every acceptance criterion in `slice-nnn.md`, mapped to the phase and
     criterion that discharges it. A gap here is a gap in the plan. -->

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-01/EX-2 |

---

## PHASE-01 — <name>

**Objective:** <one sentence: what is true at the end that was not true at the
start.>

**Surfaces:** <paths this phase may touch. Disjoint phases can run in
parallel; overlapping ones cannot.>

**Entry**
- EN-1 —

**Exit**
- EX-1 —

**Verification**
- VT-1 — <test file / name, and what it asserts>
- VA-1 — <what the agent must check, and against what>
- VH-1 — <what the human must accept>

**Notes for the implementer**

<!-- Anything an agent who has read the design still would not know: prior art,
     gotchas, the order to attack it in, what NOT to touch. -->

---

## PHASE-02 — <name>

**Objective:**

**Surfaces:**

**Entry**
- EN-1 —

**Exit**
- EX-1 —

**Verification**
- VT-1 —

**Notes for the implementer**

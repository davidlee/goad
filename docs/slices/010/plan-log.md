# Plan log — Slice 010

Append-only record of the plan *conversation* — what was asked, what was
decided, in time order. Never rewrite an entry; supersede it with a later one.

Only decisions live here. An adversarial review of the plan, if one runs, is
owned by its ledger (`review-plan.md`).

## Decisions

### 2026-09-23 — P-1, found while verifying the design, sent back to design

- **Asked:** the planner's verification of `design.md` against the tree found
  the ingress case unable to tell its failure from a display's (`notes.md`
  Handover). A design defect, so the decision belongs to design and is recorded
  there — `design-log.md`, *P-1, raised at plan* — and not copied here.
- **Decided:** "take both" / "compare the sites".
- **Consequence for the plan:** PHASE-03 carries the display-free spawn and the
  case's stderr prefix, and the second binary mutation (`start` handing
  `startup::listener` `None`). The `Launch` doc sentence found in the same pass
  is PHASE-02/EX-3, inside a declared surface, and needed no decision.

### 2026-09-23 — the plan is accepted, and no plan review runs

- **Asked:** `docs/AGENTS.md` §Plan puts two things to the user, in order —
  whether to subject `plan.md` to adversarial review in `review-plan.md`, and
  then acceptance.
- **Decided:** user: *"accept the plan; write phase 01 sheet"*. `plan.md` as
  written at `448f678` is accepted, and **no plan review runs**;
  `review-plan.md` is not created.
- **Consequence:** the stage moves to `executing`. PHASE-01's sheet is written
  in `notes.md` §Phase sheets; PHASE-01 itself has not begun.

**What the skipped review costs, recorded rather than argued.** `plan.md` is
unreviewed, so the first adversarial reading of the implementation surface is
`review-code.md`, after the code exists. Two things partly cover it, and one
thing is not covered at all.

Covered: the plan stage was itself a verification pass over `design.md` against
the tree, and it found a design defect rather than a citation slip — **P-1**,
the ingress case that could not tell its own failure from a display's, which
went back to design and landed in `9f0a503`. And the phase plan is a second
reading per phase: `docs/AGENTS.md` §Phase plan requires that expanding a phase
into a sheet **stop and go back** if the plan is wrong, rather than quietly
repairing it, so three expansions each get a chance to catch what a round would
have.

Not covered: the plan's **shape** — the phase boundaries, the ordering argument
in §Sequencing, and whether §Coverage's mapping actually discharges each
acceptance criterion. Those were written once, by the agent that wrote them, and
read once, here. If a boundary turns out wrong the signal is a phase that cannot
end green, and the response is to go back to plan rather than to widen the
phase.

**PHASE-01/EN-1 is discharged**, and by measurement rather than by inheritance:
this entry is the acceptance; HEAD is `448f678`; `just check` exits **0** on a
clean tree there. Gate total **615**, `cargo test --workspace` **580** — the
gate runs `cargo test -p goad-semantics` as a command of its own, so that
crate's 30 + 5 are counted twice and the gate total is always exactly 35 above
the workspace one. The two targets PHASE-01 touches read **58** (`goad` lib) and
**208** (`goad` `tests/renderer`), both from the workspace run.

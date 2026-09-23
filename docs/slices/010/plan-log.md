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

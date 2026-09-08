# Notes — Slice 004

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the A-1 probe | pending | |
| PHASE-02 — the configuration key, and the envelope | pending | |
| PHASE-03 — the socket's lifecycle, and the accepted path | pending | |
| PHASE-08 — the read budgets, and the closed reason set | pending | |
| PHASE-04 — `serve`'s ingress arms, the second anchor, and what a refusal costs | pending | |
| PHASE-05 — the two anchors, and what a person can see | pending | |
| PHASE-06 — binding at startup, and the demo a person runs | pending | |
| PHASE-07 — the sweep, the spec's own table, and the gate | pending | |

Rows are in **execution order** — PHASE-08 is the listener's second half and
runs between 03 and 04 (`plan.md` PL-10). Phase ids are immutable, so a split
appends a number rather than renumbering.

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — <name>

**Objective:** <copied from plan.md>

**Reading list**
<!-- path:line references, the design sections that bind, prior art. -->

**Assumptions & STOP conditions**
<!-- What is being taken on faith, and the specific conditions under which the
     agent must stop and consult the user rather than improvise. -->

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [ ]

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-08 · design accepted · `b0953e6`

### Produced
<!-- What now exists: modules, contracts, docs. -->

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

- **A repair sweep finds prose and misses the binding site.** Three of the
  design review's four rounds yielded the same class: a repair correct where it
  landed, not carried to the artefact that states the same thing normatively.
  Both round-3 contests were this, and both times the missed site was the more
  binding one — a spec requirement (R-15's universal), and a sequence diagram in
  which position is time. Prose siblings get swept; a MUST and a picture do not.
  **How to apply:** when dispositioning a `doc-wrong`, name the most binding
  artefact by hand in the repair brief rather than trusting the repairer to
  sweep for it. Candidate for `docs/memory/`.

- **A reviewer's supporting example is not evidence until someone checks it.**
  The plan review's F-27 was a false clause the reviewer supplied in F-25's
  body, repeated in its round-3 reply, and adopted whole into the plan — that
  the design restates neither `SPEC-003/R-8` nor R-9, when it restates both. The
  reviewer caught it only because it was told to check the orchestrator's own
  edit hardest. **How to apply:** a reviewer is the last person who will check
  its own example, so an example adopted from a finding gets verified by whoever
  writes it into a document. A wrong reason beside a right one is worse than no
  reason: an agent who checks the wrong one has cause to doubt the right one.
  Candidate for `docs/memory/`.

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

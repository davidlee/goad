# Notes — Slice 004

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the A-1 probe | pending | |
| PHASE-02 — the configuration key, and the envelope | pending | |
| PHASE-03 — the listener, the reply, and the socket's lifecycle | pending | |
| PHASE-04 — `serve`'s ingress arms, the second anchor, and what a refusal costs | pending | |
| PHASE-05 — the two anchors, and what a person can see | pending | |
| PHASE-06 — binding at startup, and the demo a person runs | pending | |
| PHASE-07 — the sweep, the spec's own table, and the gate | pending | |

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

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

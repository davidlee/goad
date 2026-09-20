# Notes — Slice 006: packaging and the startup surface

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the crane packages | pending | |
| PHASE-02 — the home-manager module | pending | |
| PHASE-03 — `--version`, on both binaries | pending | |
| PHASE-04 — the configuration path, named | pending | |
| PHASE-05 — the cutover, and the evidence | pending | |

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

**Fresh as of:** <yyyy-mm-dd> · <phase or stage> · <commit>

### Produced
<!-- What now exists: modules, contracts, docs. -->

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

- **A documented non-nix build path** — raised at design, 2026-09-20, and
  deliberately deferred by the user (`design-log.md`, *the non-NixOS path is
  `cargo install`, and C is a follow-up*). It is a design goal that goad runs on
  non-NixOS systems; this slice states in `design.md` that the non-nix path is
  plain `cargo install --path crates/goad --locked`, needing neither the wrapper
  nor `~/.config/goad/env`, but nothing documents or verifies it. Candidate
  scope for the follow-up slice: where that statement lives for a reader who is
  not holding this design, and whether anything checks it.

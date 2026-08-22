# Audit & reconciliation — Slice NNN

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** <commit range / branch under audit>
**Lines of attack:** <what this audit probes, and the invariants it holds the
slice to. Write this before looking, so the audit is not shaped by what is
easy to find.>

## Evidence

<!-- What was run and what it said. Not a claim of correctness — the basis for
     one. -->

- **Tests / checks:** <commands run, results>
- **Acceptance criteria:** each AC in `slice-nnn.md`, met / not met, with the
  evidence.
- **Verification criteria:** each VT/VA/VH in `plan.md`, discharged or not.
- **Surface delta:** paths actually changed vs. the surfaces each phase
  declared. Undeclared paths are the highest-signal lead — scope creep, a
  missed design update, or an undocumented touch. Declared-but-untouched means
  dropped work or a stale design. Neither is automatically a finding; both are
  places to look.

## Code review

<!-- Adversarial, by a fresh agent where possible. Findings are append-only and
     keep their ids across rounds. -->

### Round 1 — <reviewer> — YYYY-MM-DD

| id | severity | location | finding | disposition | resolution |
|----|----------|----------|---------|-------------|------------|
| F-1 | blocker / major / minor / question | `path:line` | | | |

<!-- severity — blocker: must not ship. major: real defect or design breach.
       minor: worth fixing, not urgent. question: needs an answer before it
       can be graded.
     disposition — aligned: observation correct, nothing to change.
       fix-now: code fix, inside this slice. spec-wrong: the code is right and
       the document is stale — goes to Reconciliation below. tolerated:
       accepted drift, with a written rationale. deferred: becomes a follow-up
       in `slice-nnn.md`.
     No finding may be left undispositioned at close. Do not downgrade a
     blocker to dodge the gate, and do not defer merely because the fix is
     large. -->

**Synthesis:** <the closure story: what the audit found, what it changed, and
the risks it knowingly leaves standing.>

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

| document | change | reason | done |
|----------|--------|--------|------|
| `specs/NNN-…md §4` | | code diverged at `path:line`; code is right | [ ] |

**Design drift not reconciled:** <where the implementation departs from
`design.md` and the design was left as-is, with the reason. The design is a
record of intent at a point in time; it is not retro-fitted to the code
without saying so.>

## Closure

- [ ] All findings dispositioned; no blockers outstanding
- [ ] All acceptance criteria met, or explicitly waived by the user
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

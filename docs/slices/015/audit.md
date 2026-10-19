# Audit & reconciliation — Slice 015

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `c68a624..53d140a` on branch `slice-015` (worktree
`.claude/worktrees/slice-015`): PHASE-01 through PHASE-04, ending
`015: PHASE-04 — VH-1 run by the person; phase done`.

**Question:** the slice is finished when a config whose `command` and
`ingress.path` are relative works however goad, `goad-check --config` and
`goad-emit` were started — through a symbolic link too — and the record says
so: the kit with no *make every path absolute* advice, and SPEC-001 /
SPEC-003 carrying the rule once `canon-delta.md` is promoted.

What this audit checks, decided before looking:

1. **The gate.** `just check` exits 0 on `53d140a`, run by the auditor, with a
   `TMPDIR` short enough for the 107-byte socket control.
2. **Each AC** (AC-1 … AC-5) against the code and the tests, not the phase
   sheets' claims. AC-4 and AC-5's person half from PHASE-04's *VH-1* record.
   AC-3's canon half is promotion, which is not yet done.
3. **Each VT / VA / VH** in `plan.md`, walked against the tree: the named
   test exists, asserts what the criterion says, and is green. Natural-red and
   mutation tables checked for missing rows, uncompiled mutants, and reds for a
   reason other than the one claimed; a sample of mutations re-run by the
   auditor — at least one per phase judged weakest — by scratchpad
   copy-and-restore, `--no-fail-fast`.
4. **Surface delta.** Each phase commit range's touched paths against that
   phase's declared Surfaces; every undeclared path classified.
5. **Reconciliation.** One row per `canon-delta.md` change; each §7 test
   citation resolves to a `fn` in the tree; any other canon / doc / code
   divergence classified. Drafted only: canon is not edited here.
6. **Design drift.** Where shipped code departs from `design.md`.

Out of scope here: in-depth code review (a fresh reviewer, `review-code.md`)
and repairs. Defects found on the way are listed as *Leads for the code
reviewer*.

**Invariants held** (`CLAUDE.md`):

- *The host does not understand the user's domain* — no domain vocabulary in
  new host types or names; the working directory is a transport fact, not a
  domain one.
- *Permissive parsing, canonical representation* — no argument in argv is
  reinterpreted as a path; the anchored directory is fixed at `Config::load`,
  the one door, and nothing downstream re-derives it.
- *No narrowing of wire compatibility for the renderer* — the config's
  accepted shape is unchanged; only what a relative path means changes.
- *A backend failure never takes the host down* — a spawn failure in the new
  directory is still `BackendError::Spawn`, R-44, the configuration's side,
  and names where it looked.
- *Strata run one way* (ADR-001) — nothing enters `goad-semantics`; the clock,
  filesystem and subprocess work stays in `goad-shell` and above.

<!-- This is the audit's scope — evidence, criteria, canon. The code review's
     own lines of attack belong in `review-code.md`'s Brief, not here. -->

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

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`
- **State:** open | resolved · outstanding blockers: none | <ids>

## Verdict

<!-- The slice's closure story, written once, here. Draws on the ledger's
     synthesis and on the evidence above; restates neither. Does this slice do
     what it set out to do, and what is being accepted knowingly? -->

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

| document | change | reason | done |
|----------|--------|--------|------|
| `specs/NNN-…md §4` | | code diverged at `path:line`; code is right | [ ] |
| `draft-spec.md` → `specs/NNN-slug.md` | promote | drafted during this slice | [ ] |

**Design drift not reconciled:** <where the implementation departs from
`design.md` and the design was left as-is, with the reason. The design is a
record of intent at a point in time; it is not retro-fitted to the code
without saying so.>

## Closure

- [ ] All findings dispositioned; no blockers outstanding
- [ ] All acceptance criteria met, or explicitly waived by the user
- [ ] Each verification criterion in `plan.md` walked against the code, or the gap measured and carried
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `notes.md` §Open swept against `slice-nnn.md` §Follow-ups; every entry dispositioned
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

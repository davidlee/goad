# Audit & reconciliation — Slice 010

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `b444c6a..f9620b6` on `main` — `b444c6a` is the slice's first
commit (design approved), `f9620b6` the hand-over to audit. The code under
audit is PHASE-01 `ab5604f`, PHASE-02 `5b23720` and PHASE-03 `c67dd9c`.

**Question:** the slice is finished when all of these hold.

1. **The gate is green on the tree as committed** — `just check` exits 0, and
   the figures are re-measured here, not taken from `notes.md`.
2. **Every acceptance criterion in `slice-010.md` is met by something in the
   tree**, named by symbol: the draft spec's phase cut and §Owns (AC-1, AC-2);
   `StartupError` no longer carrying the loop's ending (AC-3); the pure exit
   decision and its one-tier-down cases, including a real
   `slint::PlatformError` (AC-4, AC-11); `exit_codes.rs`'s existing cases
   byte-identical outside the module doc (AC-5); the *stopped running* line
   (AC-6); `nix/module.nix`'s paragraph gone with all three false claims
   (AC-7); the SPEC-003 cells, which are canon-delta only until promotion
   (AC-8); the running-host observation (AC-9) and FU-1 (AC-10), which are
   audit/close work and are reported as pending, not met.
3. **Every VT/VA/VH in `plan.md` is discharged against the tree** — each
   phase sheet's claim is checked by reading the named symbol or re-running
   the named command; a mutation is re-run only where the claim cannot be read
   off the code.
4. **Nothing was touched that no phase declared.** `git diff --stat` per
   phase commit against that phase's Surfaces; every undeclared path named.
5. **The invariants hold at the new code**: no domain vocabulary in the new
   module; `src/semantics/` untouched (ADR-001); a backend or display failure
   still never leaves the host unable to be invoked again — here, that *stopped
   running* is not suppressed by the unit's `RestartPreventExitStatus`.
6. **The record can be made true**: every document the slice must change is a
   Reconciliation row, and every finding carried from the phases has a
   recommended disposition.

**Checked here:** 1–4, 6 fully; 5 by grep and reading the unit file. **Not
checked here:** AC-9 needs a person on the running host — this audit writes
the steps, it does not observe the result. Adversarial code review is the
separate `review-code.md` ledger. Canon is drafted as rows and not applied: it
waits for the user's endorsement.

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

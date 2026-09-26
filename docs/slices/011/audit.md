# Audit & reconciliation — Slice 011

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `d2617c1..fdc2229` on `main` — `d2617c1` is the accepted plan,
`fdc2229` the hand-over to audit (PHASE-03 done, VH-1 run by the user). The code
under audit is PHASE-01 (`94251b6`, `485980b`), PHASE-02 (`ccc9d61`, `f564c29`,
`0533411`) and PHASE-03 (`64f75d4`). Written before reading the code or the diff.

**Question:** the slice is finished when all of these hold.

1. **The gate is green on the tree as committed** — `just check` run here,
   not taken from `notes.md`.
2. **Every AC in `slice-011.md` is met by something in the tree, named by
   symbol**, or reported as pending with who owes it: AC-5 (canon promotion)
   and AC-7 (FU-2 strike) are audit/close work; AC-6 is the user's run.
3. **Every EX/VT/VA/VH in `plan.md` is discharged against the tree** — each
   phase sheet's claim checked by reading the named symbol or re-running the
   command, and marked *re-verified* or *taken from `notes.md`*. PHASE-03 is
   audited against its criteria as the user narrowed them (`plan-log.md`).
4. **The mutation evidence is real** — at least two of the design §9 controls
   the orchestrator did not re-run (it re-ran M0 and M8b) are re-run here,
   each confirmed to compile and to go red on the case that claims it.
5. **Nothing was touched that no phase declared** — the diff's paths against
   each phase's Surfaces; every undeclared path named.
6. **The invariants hold at the new code.**
   - CLAUDE.md's five: no domain vocabulary in anything new; wire parsing
     untouched (nothing crosses the backend boundary); no protocol capability
     narrowed; a backend failure still never stops the host or leaves it
     unable to invoke the backend again — here, a flood of refusals must not
     starve a command, a firing or the ingress-stopped report; and
     `goad-semantics` untouched (ADR-001 — no timer or clock enters stratum 1).
   - The design's own: **I-1** consecutive refusal-arm firings at least `I`
     apart; **I-2** a refusal decided while idle reaches the surface within `I`
     unless the loop ends or it is overwritten; **I-3** only the
     `Fired::Ingested`/`None` path and the new arm change behaviour; **I-4**
     `floor_until`, `event_floor_until` and `sleep` keep their one write site
     each, and the new arm writes none of them (SPEC-002/R-4, R-12, ADR-004).
   - **R-15's amended guarantee** (`canon-delta.md`) — attacked at its
     exceptions: that every `continue`/`break` in the new `'idle` region
     targets the label it must (design R2), that the leading edge is not a
     debounce and not moved by a person's own present (D5), that nothing
     other than `ingest`'s `None` is coalesced (D6), and that a command during
     a coalesced interval presents at once with the stale fold.
7. **Tests assert the property, not a proxy** — the timed cases read the
   window, not the retained model (D12, M0), and their margins are stated
   against the direction load moves them.
8. **The record can be made true** — every divergence between code and
   `design.md`/canon classified; `canon-delta.md`'s changes drafted as
   Reconciliation rows for the user's endorsement. Canon is not edited in this
   session.

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

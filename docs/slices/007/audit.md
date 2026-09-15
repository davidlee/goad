# Audit & reconciliation — Slice NNN

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** <commit range / branch under audit>
**Question:** <what would have to be true for this slice to be finished, and
which of those the audit intends to actually check. Write it before looking, so
the audit is not shaped by what turned out to be easy to find.>

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

<!-- Everything above is the audit's to gather. The two entries below are
     PHASE-06/EX-5: the *human* evidence, which exists only at the moment a
     person ran the software and which a later agent cannot reproduce. They are
     written here rather than left in `notes.md` because `docs/AGENTS.md`
     §Tiers requires it, and they are the user's account and not the
     implementer's. `notes.md`'s PHASE-06 sheet carries the fuller record,
     including what the user said that was not actioned. -->

### AC-7 — a person filled the form and read the record

**Observed by the user, 2026-09-15**, on `just demo` against
`examples/shell/backend.sh`, on a niri (Wayland) session. The implementing agent
did not see the window; the account below is the user's.

The demo's form carries six fields on one option: five `boolean`, drawn, in
three blocks — one ungrouped, then two `group` values — and one `text` field,
`note`, of a kind this renderer does not draw. The second option carries none.

1. Before answering, the user opened the diagnostic surface from the tray and
   confirmed the undrawn report: *"diagnostics - TIL it has a right click menu.
   confirmed"*. That is **AC-3's human half** — the field is reported, the view
   is still shown, and the option still answers.
2. The user ticked three of the five boxes by mouse, left two alone, and
   pressed the option's button **once**.
3. Reopening the diagnostic surface, the record read — verbatim, from the
   user's screenshot:

   ```
   stderr: answered 2026-09-15T09:26:36.425909399Z#1: option yes, values {"desk":true,"focused":false,"outside":false,"started":true,"tired":true}
   ```

   and in the user's own words: *"the bools are correct, no note"*.

**Five keys from one exchange**: one for each field the renderer drew of the
option answered, `true` for the three ticked and `false` for the two left alone,
and **no key for the undrawn `text` field**. AC-7 met, and with it a person's
sight of AC-1, AC-3 and `canon-delta.md`'s R-58.

### AC-10 — a person judged the look, and the bound held

**Observed by the user, 2026-09-15**, over two builds.

The first build was reported unusable in one specific way: the layouts
distributed the window's full height between the fields, so two members of one
block sat further apart than two blocks did. Actioned **inside AC-10's bound**
— `alignment: start` on the block container and on each block, so the declared
spacing states grouping rather than a minimum. The user on the second build:
*"form spacing looks a lot saner now"*.

Asked the criterion's two questions directly — whether the fields visibly belong
to the option that answers rather than to the one below, and whether each
heading visibly covers its own fields and not the ungrouped one above — the
user's acceptance was:

> *"i'll call it legible enough for now, nothing that can't survive until
> holistic design work"*

**AC-10 met, and the qualifier is part of it:** this is legibility sufficient to
close 007, deferring explicitly to the holistic pass that is 008. Quoting the
acceptance without the qualifier would overstate it.

What the user said that fell **outside** the bound was recorded verbatim and not
actioned, per PHASE-06/EX-4 and `plan.md` S-8 — no word wrap and no text
selection on the diagnostic surface, the vertical distribution above the block
container, and the title clipped at the top of the window. All four are in
`notes.md`'s PHASE-06 sheet and are 008's brief. The bound was on what could be
changed, never on what could be said.

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
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

# Audit & reconciliation — Slice 004

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `b6ca5f7..93abab3` on `main`. `b6ca5f7` opened the slice; the code
phases begin at `9cfb679`; `93abab3` is HEAD, the tree is clean, and all eight
phases are `done`.

**Question:** slice 004 is finished when a watcher outside the host can make it
ask its backend something, on the path slice 003 built, without the host having
learned what the event meant — and when the record says so truthfully. This
audit checks the following, and says plainly where the evidence does not carry.

**Lines of attack, chosen before looking:**

1. **The gate, run rather than cited.** `just check` executed here, transcript
   saved, the real exit code reported. A phase that reports green is a claim;
   the gate is the evidence. Slices 001–003 all closed green on a binary that
   could not open a window, so a green gate is the floor and not the argument.
2. **Each of AC-1..AC-13 against a named test function or a code citation.**
   `plan.md`'s AC-to-phase table is a map to check, not the answer: the question
   is whether an assertion exists that would fail if the criterion were false,
   not whether a phase claimed the criterion. Four ACs took readings during
   design (AC-1's *verbatim*, AC-3's *every envelope*, AC-6's *anchors*, AC-9's
   *exit code*); each is held to the reading as written, not to a looser one.
   AC-9's exit-code clause is held by review by construction — the audit checks
   the argument at `main.rs`, and checks that no instrument was quietly claimed
   in its place.
3. **Each VT/VA/VH in `plan.md`, systematically.** Discharged or not, with the
   test that discharges it. A verification criterion with no assertion behind it
   is the same defect as an unmet AC wearing different clothes.
4. **The surface delta, walked from `git log` myself.** PHASE-07's sweep already
   claims 26 files and no undeclared paths; an audit that reads a sweep's
   conclusion has audited the sweep. Undeclared paths are the highest-signal
   lead; declared-but-untouched is checked in the other direction.
5. **The five invariants, as invariants and not as slogans.**
   - *The host does not understand the domain.* The envelope is carried, not
     read. `source`, `kind` and `data` reach the backend unexamined; no new host
     type or module name carries domain vocabulary; the scan passes because
     there is nothing to find, not because the scan is narrow.
   - *Permissive wire, canonical internals.* The envelope's normalization is the
     only door, and past it nothing is unvalidated. An ambiguous envelope is
     refused rather than guessed at.
   - *Wire compatibility is not narrowed to the renderer.* The ingress contract
     is what `draft-spec.md` states, not what the current loop happens to
     consume.
   - *A backend — or here, a writer — failure never takes the host down*, and
     never leaves it unable to invoke the backend again. AC-12 is the test of
     it; the audit also looks for the paths where a panic or an early return
     could reach the loop.
   - *Strata run one way.* Stratum 1 gains no dependency, `src/semantics/`
     names no shell, and the envelope's normalization sitting in stratum 2 is
     the deliberate ADR-001 §Consequences call the slice says it is — the audit
     checks that the new ADR is actually owed and named, not assumed written.
6. **Canon and drafts, as obligations rather than as done work.** CD-1, CD-2,
   CD-3 and the new ADR are unapplied by design until reconciliation; the audit
   records what endorsement each needs and writes the Reconciliation rows
   unchecked. `SPEC-002/R-12` and `SPEC-003/R-12` are two different
   requirements: any citation that drops the prefix is a defect.
7. **AC-13, the human observation.** Lifted from `notes.md`, with an honest
   account of which runbook steps a person actually performed.

**What this audit does not do.** It does not write the code review — that is
`review-code.md`, a fresh adversarial agent, after this. It does not amend
canon, promote `draft-spec.md`, or apply `canon-delta.md`: those are user gates.
It does not repair findings. It records, disposition-ready, and stops. The
Verdict is written from the evidence and says so where the evidence runs out;
the Closure checklist stays unticked while the code review has not run.

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
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

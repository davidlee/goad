# Methodology

## Documentation

Documentation in `./docs/`
`brief.md` is the initial project brief
`adr/` contains sequentially numbered decision records.
`policy/` contains sequentially numbered policies.
`memory/` contains noteworthy facts or processes.
`slices/` contain sequentially numbered coherent changes.
`specs/` contain evergreen specifications; they are normative truth.

`specs`, `policy` and `adr` are **governing canon**. They must be abided by, or amended (with explicit user endorsement). Do not fail to read any which may be relevant:
```zsh
ls ./docs/{specs,policy,adr}/*
```

Templates for all of these live in `./docs/templates/`. Create one by copying, never by writing from memory:
```zsh
cp ./docs/templates/{spec,adr,policy}.md ./docs/{specs,adr,policy}/NNN-slug.md   # pick one
```
Adversarial review runs on a ledger, copied the same way:
```zsh
cp ./docs/templates/review-ledger.md ./docs/slices/123/review-design.md   # or review-plan / review-code
```

Canon is normative and evergreen: it states what is true now. No changelogs, no revision history. In an `adr` it is the **decision** that is fixed, not the document — keep the record accurate as consequences are learned, references accumulate, or status changes. What you may not do is quietly alter what was decided: a decision that no longer holds is superseded by a new ADR.

### Canon that does not exist yet, or must change

Two cases: the rule a slice needs was never written down, or the slice changes a rule that was. In both, do **not** edit canon mid-slice and do **not** proceed on an unwritten rule you are holding in your head. Draft it in the slice folder instead:

- **New canon** — `draft-spec.md` in the slice folder, from the spec template. Number it `SPEC-NNN` only at promotion; until then it is a draft and says so.
- **Changing existing canon** — `canon-delta.md` in the slice folder: one entry per affected document, naming the document, the section, the change as it will be stated, and why. A short delta beats a whole draft spec for a small change.

While the slice runs, the draft is its working authority: design, plan and execution cite it exactly as they would the real thing, and keep it current as the shape of the work changes. It answers "what rule are we following?" for the duration. Nothing outside the slice may cite it — it is not canon yet.

**Promote during audit and reconciliation**, with explicit user endorsement: move `draft-spec.md` to `docs/specs/NNN-slug.md`, or apply `canon-delta.md`'s stated changes to the documents it names. Record each move in the Reconciliation table in `audit.md`. Any decision that shaped the draft and could later be reversed by accident gets its own ADR.

A slice does not close holding an unpromoted draft. Either it lands, or it is abandoned with the reason written down.

---

## Workflow

This is not optional fluff. Follow it closely. Do **not** deviate from it without **explicit user instruction**.

### Where it goes

Four kinds of file, four jobs. If you are about to write the same thing twice, one of them is the wrong home.

| file | holds | shape |
|---|---|---|
| the artefact — `slice-nnn.md`, `design.md`, `plan.md`, canon | **current truth.** What is so, now. | No history, no findings, no progress. Edited in place. |
| `*-log.md` | **the conversation.** What was asked, what the user decided, why. | Append-only, time-ordered. Never rewritten; superseded. |
| `review-*.md` | **one adversarial review,** end to end: its brief, its findings, their evidence and fate, its synthesis. | Append-only, immutable finding ids. |
| `notes.md` | **the work.** Per-phase progress, tasks, local decisions, things noticed in passing, and the harvest. | Disposable detail. Anything durable is lifted out before close. |

The two that get confused: a **decision** and a **finding**. A finding is an
observation by a reviewer that something is wrong — it lives in a ledger and
ends `verified` or `withdrawn`. A decision is a choice the user made — it lives
in a log. A finding that prompts a decision produces one of each, cited to one
another; that is not double handling.

`audit.md` is the exception that proves the rule: it is the slice's closing
argument, and it draws on all four without copying any of them.

### Slice

- User begins a design conversation about new work sufficient to scope a new slice.
- Agent declares intent and creates a new numbered slice folder:
```zsh
cp -r ./docs/templates/slice ./docs/slices/123/
mv ./docs/slices/123/slice-nnn.md ./docs/slices/123/slice-123.md
```
- Agent edits templated `slice-123.md`, interviewing the user as required, to establish purpose, scope, goals / non-goals, acceptance criteria, and open questions to be explored during design. 
- If the canon this slice needs is missing, or the slice will change canon, start the draft now — see *Canon that does not exist yet, or must change* above.
- Proceed to design, with a fresh agent when appropriate. Read the templated `design.md`. 
  - Research existing documentation and code. Keep verified research output in `research.md`, in the slice folder. This may need to be repeated later as new details emerge.
  - Interview the user, one question at a time, to ensure mutual understanding and agreement about first the intent, and then the implementation. 
  - Present options, with your recommendation where appropriate. Record user decisions after each answer in `design-log.md` in the slice folder, in case of compaction or interruption.

### Design 

- Once all the questions worth asking have been answered, draft the design. Present each section to the user for confirmation or adjustment; then before proceeding to the next
- Write the design document exactly as presented.
- Suggest that an adversarial review (conducted by a fresh agent) check the design's assumptions against the documentation and code. 
  - When agreed, spawn a review agent if possible; otherwise provide a prompt for a fresh session. 
  - Record the reviewer's findings in a ledger — `review-design.md`, copied from `docs/templates/review-ledger.md`. The ledger's own Protocol section is self-contained; follow it. Findings are append-only with immutable ids; a second round appends to the same file.
  - Disposition each finding, confirm your intended response with the user, and then integrate any changes required.
    - Apply a high level of rigour; do not introduce new flaws as you address the old. Fix the class, not the instance.
    - Repeat until all of your repairs have been reviewed, and no serious findings remain.
  - The ledger owns the review end to end — brief, findings, synthesis. `design-log.md` records only what the *user* decided in response, citing the finding id.
- Revise the `slice-nnn.md` doc for consistency with the updated design.
- If the design has changed since their approval, ask the user for it again now.

### Plan

- This is likely to require a fresh agent.
- First, perform research again if necessary, covering anything not yet covered adequately in `research.md`.
- Read the design closely; trace the dependencies. Examine your assumptions and the approach laid out in the design, then verify them against the code. If any unresolved design issues emerge, go back to the appropriate stage of design and work forward from there.
- Fill in the `plan.md`, carefully choosing entry / exit criteria for each phase such that if they are completed, the intent of the slice and the design will be observed. Use multiple phases also to ensure each phase is reasonable for a single agent to complete within a session, including bookkeeping.
- Ensure that `plan.md` (and, at your option, `notes.md`) capture all the detail necessary for agents, having read the design, to attend to just their own phase's implementation and that the combined result will operate as intended.
- Present the choice to subject the plan to adversarial review (as described above) to the user. The ledger is `review-plan.md`; user decisions taken during planning go in `plan-log.md`. Neither accumulates in `plan.md`.
- Ask the user for their acceptance of the plan.

### Phase plan

- Do this **immediately before executing a phase**, not up front for all of them — a phase sheet written three phases early is fiction.
- Expand the phase's `plan.md` entry into a phase sheet under `## Phase sheets` in `notes.md`: reading list (`path:line`, the binding design sections, prior art), assumptions, STOP conditions, and a task breakdown.
- Verify the phase's entry criteria are actually met before starting. If they are not, the previous phase is not done.
- If expanding the phase shows the plan is wrong, go back to plan (or design) rather than quietly repairing it in the sheet.

### Execute

- One phase, one agent, one session. Set the phase to `in progress` in the `notes.md` status table.
- Red / green / **refactor**. The refactor step is not optional; it is where the design survives contact.
- Stay inside the phase's declared surfaces. Touching anything else is either a design change or scope creep — in both cases, stop and ask.
- STOP and consult the user on: an unanticipated obstacle, a tradeoff the design did not settle, a dependency addition, or any concession you are tempted to make on your own. Do not improvise past a decision that was not yours.
- Keep the phase sheet current as you go — tasks, decisions taken, findings. Do not save the bookkeeping for the end; it will be lost.
- End green: the phase's exit and verification criteria discharged, tests passing, nothing half-applied. Set the phase to `done`.
- Update the `## Harvest` section of `notes.md` in place before handing off.

### Audit & reconcile

- Do this once, after the last phase. Fresh agent; a worktree if the tree must stay clean. Fill in `audit.md`.
- Write the **Brief** before looking — what you intend to attack, and the invariants you will hold the slice to. Writing it afterwards means auditing only what was easy to find.
- Gather **evidence**: run the tests and checks; walk each acceptance criterion in `slice-nnn.md` and each verification criterion in `plan.md`; diff the paths actually touched against the surfaces each phase declared. Undeclared paths are the strongest lead. Evidence is the basis for a verdict, not the verdict.
- **Code review** the whole slice adversarially — a fresh agent where possible, otherwise a prompt for a fresh session. Same ledger again, as `review-code.md`, subject `implementation`. Repeat rounds until the repairs are themselves reviewed and nothing serious remains. `audit.md` links the ledger and states the outstanding-blocker count; it never copies findings.
  - Confirm each disposition with the user before acting on it. Fix the class, not the instance.
  - Do not downgrade a blocker to clear the gate, and do not defer a fix merely because it is large.
- **Reconcile the record.** The code is what shipped; the documentation must now be true about it. For each divergence decide which side is wrong:
  - document stale, code right → amend the spec / policy / ADR. This is **canon**: get explicit user endorsement before writing.
  - code wrong → it is a finding, fix it in the slice.
  - neither cleanly → it is a decision, so take it to the user.
- **Promote the slice's drafts.** `draft-spec.md` moves into `docs/specs/`; `canon-delta.md`'s changes are applied to the documents it names. Both need explicit user endorsement, and both are recorded in the Reconciliation table. A draft that should not land is abandoned in writing, not left in the slice folder.
- `design.md` is a record of intent at a point in time. Do not retro-fit it to the code silently; where the implementation departed and the design stands as written, say so under **Design drift not reconciled**.

### Close

- Work the Closure checklist at the foot of `audit.md`.
- Write the `## Summary` and `## Follow-ups` sections of `slice-nnn.md`. Follow-ups become future slices; do not leave them only in the audit.
- Lift durable facts from `notes.md` Harvest into `docs/memory/` — anything a future agent would otherwise rediscover the hard way.
- Set the slice stage to `done`.

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

---

## Workflow

This is not optional fluff. Follow it closely. Do **not** deviate from it without **explicit user instruction**.

### Slice

- User begins a design conversation about new work sufficient to scope a new slice.
- Agent declares intent and creates a new numbered slice folder:
```zsh
cp -r ./docs/templates/slice ./docs/slices/123/
mv ./docs/slices/123/slice-nnn.md ./docs/slices/123/slice-123.md
```
- Agent edits templated `slice-123.md`, interviewing the user as required, to establish purpose, scope, goals / non-goals, acceptance criteria, and open questions to be explored during design. 
- Proceed to design, with a fresh agent when appropriate. Read the templated `design.md`. 
  - Research existing documentation and code. Keep verified research output in `research.md`, in the slice folder. This may need to be repeated later as new details emerge.
  - Interview the user, one question at a time, to ensure mutual understanding and agreement about first the intent, and then the implementation. 
  - Present options, with your recommendation where appropriate. Record user decisions after each answer in `design-log.md` in the slice folder, in case of compaction or interruption.

### Design 

- Once all the questions worth asking have been answered, draft the design. Present each section to the user for confirmation or adjustment; then before proceeding to the next
- Write the design document exactly as presented.
- Suggest that an adversarial review (conducted by a fresh agent) check the design's assumptions against the documentation and code. 
  - When agreed, spawn a review agent if possible; otherwise provide a prompt for a fresh session. 
  - Record the reviewer's findings in `design-log.md`. Disposition each of them, confirm your intended response with the user, and then integrate any changes required. 
    - Apply a high level of rigour; do not introduce new flaws as you address the old. Fix the class, not the instance.
    - Repeat until all of your repairs have been reviewed, and no serious findings remain.
- Revise the `slice-nnn.md` doc for consistency with the updated design.
- If the design has changed since their approval, ask the user for it again now.

### Plan

- This is likely to require a fresh agent.
- First, perform research again if necessary, covering anything not yet covered adequately in `research.md`.
- Read the design closely; trace the dependencies. Examine your assumptions and the approach laid out in the design, then verify them against the code. If any unresolved design issues emerge, go back to the appropriate stage of design and work forward from there.
- Fill in the `plan.md`, carefully choosing entry / exit criteria for each phase such that if they are completed, the intent of the slice and the design will be observed. Use multiple phases also to ensure each phase is reasonable for a single agent to complete within a session, including bookkeeping.
- Ensure that `plan.md` (and, at your option, `notes.md`) capture all the detail necessary for agents, having read the design, to attend to just their own phase's implementation and that the combined result will operate as intended.
- Present the choice to subject the plan to adversarial review (as described above) to the user. Create `plan-log.md` if necessary to record findings and dispositions, rather than accumulating revision history in `plan.md`.
- Ask the user for their acceptance of the plan.

## 




# Don't feed the raiser your finding — prompt the surface, not the conclusion

Slice 009, where an orchestrator that already suspected a defect had to hand the
question to a fresh agent without handing it the answer.

## The fact

When you ask an agent to check something you already suspect, how you phrase the
ask determines what you get back. Give it the conclusion and it will find
support for the conclusion — including support that is not there. What comes
back then looks like independent confirmation and is an echo.

**Two witnesses beat an echo.** A finding is worth acting on when two agents
that were not told each other's conclusions reach it from different directions.

## How to apply

- **Name the surface, not the defect.** *"Check whether the sibling guard is
  held by any case"* — not *"I think the sibling guard is unheld."*
- **Give the leads as questions**, and say they may be wrong. Slice 009's
  round-4 brief listed three "where this session is most likely to be wrong"
  leads; one produced a finding, one produced nothing, and the third was
  **closed as a non-finding** — the reviewer measured that the thing the
  orchestrator suspected was actually held, by six targets.
- **Withhold your own reasoning until the report is in.** Then compare.
- **Verify what comes back at the source anyway.** An agent told the right
  surface can still be wrong about it — see
  `verify-the-enumeration-not-the-conclusion.md`.

## The related failure: raising on half a mechanism

The converse trap is raising something *yourself* on partial evidence because a
mechanism is confirmed and the cause is not.

Slice 009 had one: a tray icon disappeared and came back while neither host
process died. The **non-recovery** mechanism was real and confirmed — nothing
re-registers an icon the platform has dropped. The **cause of the
disappearance** was unknown, and the human witness was hedged (*"I think only
one"*).

It was recorded as a follow-up and not raised as a finding, deliberately.
Raising it would have substantiated a mechanism on half its evidence and put a
confident sentence into the record that nobody could later check.

Related: `verify-the-enumeration-not-the-conclusion.md`,
`a-reviewers-example-is-not-evidence.md`,
`verify-the-proposed-instrument.md`.

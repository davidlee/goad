# A directive keyed on an exit status inherits every cause that status covers, including ones added later

Found at slice 006's audit, from the journal; the status it was about was
re-cut by slice 010. What each of the host's statuses means now is SPEC-004 —
this file does not restate it, and keeps only the lesson.

## The fact

A supervisor's directive keyed on a status — systemd's
`RestartPreventExitStatus`, a script's `case $?` — is a claim about **every**
path in the program that reaches that status, not about the paths the author
had in mind. When the status is chosen by mapping a whole error type to one
number, the directive also covers every variant added to that type afterwards.

## Why the enumeration was convincing and still wrong

The directive was argued from three variants and applied to ten. Every named
one supported the conclusion; the one that did not was never named, and it was
the only one that had ever happened on the daily-driver machine. This is the
standing shape of `verify-the-enumeration-not-the-conclusion.md`: a correct
conclusion about the members you listed, applied to a set you did not walk.

## How to apply

- Before writing or reviewing a directive keyed on a status, find what chooses
  the status and walk **every** path into it — not the variants the comment
  names.
- Prefer a status whose meaning is a fact the process observes (how far it
  got) over one whose meaning is a prediction (whether a retry would work).
  A prediction has to be re-judged per cause; an observation does not, which
  is why SPEC-004 cuts on phase.
- When adding a cause that reaches an existing status, ask at the cause whether
  each consumer of that status is still right about it.

Related: `the-journal-is-the-audit-instrument-for-a-shipped-unit.md`.

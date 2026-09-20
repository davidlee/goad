# A present that disturbs nothing is invisible, so settle "is it firing?" from outside

Slice 009, VH-2. The slice's own repairs removed the last two ways of telling
that the host was still working.

## The fact

The goal of the re-present work is that a present which changes nothing
**changes nothing** — no destroyed element, no moved caret, no interrupted
drag, no rebuilt row. When that goal is met, a present is by construction
undetectable from inside the window.

Two things had accidentally been serving as the liveness indicator, and both
were repaired away in the same slice:

- the tray icon re-pushed to the desktop's tray service on every present
  (`F-R5`), and
- the diagnostics repeater rebuilding every line on every present (`F-B9`).

So during a human run the honest question *"is the pulse still firing?"* could
not be answered from the screen at all.

## How to settle it

**From outside the process.** The backend is a subprocess and is spawned once
per firing, so watching the pid answers the question without touching the host:

```
16:57:14  pid 45633  ┐ 3s alive = the configured delay
16:57:18  pid 45726  ┘
16:57:22  pid 45775    a new pid every 4.05s, five firings, no drift
16:57:26  pid 45824
16:57:30  pid 45874
```

That measurement also **identifies which rule is binding**, which no amount of
reading would have: the interval was four seconds, not the three that
`SPEC-002/R-4`'s floor would give. The backend had instructed a check one second
after a three-second reply, so the instructed instant was later than the floor
and the backend's own instruction was what bound. `ADR-004`'s anchor — spacing
measured from the previous *firing*, not from its reply — is what puts it at
four and not seven.

## How to apply

- **When a repair removes a redundant write, ask what that redundancy was
  accidentally providing.** This is the general shape and it bit twice in one
  slice: the tray icon's per-present push was also the only thing re-registering
  an icon the platform had dropped, so removing it removed the self-healing
  (recorded as a follow-up; the repair was still right).
- Before a human run, decide **in advance** how liveness will be observed, and
  put it in the observation table. "It looked like it stopped" is otherwise
  indistinguishable from "it stopped".
- Prefer an observation the host cannot influence: a subprocess pid, a log the
  backend writes, a socket connection count.
- **A diagnostic surface that shows only the current frame is not a history.**
  Slice 009's shows one frame's lines, so "a new refusal line every N seconds"
  was never the available evidence — what says the host carried on is that it
  reports a resolved next check at all.

Related: `getting-eyes-on-the-running-host.md`,
`settle-a-usability-question-by-running-it.md`,
`a-log-line-written-before-the-work-is-not-an-observable-of-it.md`.

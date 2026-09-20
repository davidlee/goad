# A backend exchange has no useful duration — the floor is a process spawn

Measured in slice 009.

## The fact

| backend | measured |
|---|---|
| `examples/shell/backend.sh` | ~2.4 ms over 50 spawns |
| `examples/typescript/backend.ts` | ~12 ms over 10 spawns |
| ceiling | the configured `backend.timeout` — 5 s in `examples/demo.toml` |

That is **three orders of magnitude**, and which end you land on is chosen by
the backend author, not by the host.

## Why it matters

Anything whose lifetime is *one exchange* has no duration you can design
around. Two consequences that shaped this slice:

- **A host behaviour that only appears during a slow exchange is invisible to a
  fast one.** An acceptance criterion failed on a person and passed in every
  test, because the rig's exchanges completed in milliseconds and production's
  took seconds. The repair was measurable only after the demo backend gained a
  knob to hold its reply.
- **A window that "cannot be typed into while the host is busy" is unnoticeable
  at 2 ms and unusable at 5 s.** Design for the ceiling; demo at the ceiling.

## How to apply

- Give the demo backend a **delay knob**, and document what each setting is for.
  Slice 009's `GOAD_DEMO_DELAY=3` is "a window long enough to type into";
  `=6` is "past the timeout, watch the host refuse and carry on."
- When a rig's timings differ from production's by orders of magnitude, say so
  in the case's own doc. A green case under a millisecond rig is not evidence
  about a second-scale exchange.
- Never treat a measured exchange time as a property of the host. It is a
  property of somebody else's program.

Related: `a-bound-is-not-tested-at-the-bound.md`,
`settle-a-usability-question-by-running-it.md`,
`timed-test-margins-are-measured-at-the-bound.md`.

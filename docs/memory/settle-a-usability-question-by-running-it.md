# Settle a usability question by running the software, not by reasoning about it

The user's own words when asked to choose between two refusal surfaces during
slice 009's design, and then confirmed twice by what the human runs actually
found.

## The fact

Presented with two options and an argument for each, the answer was the lean
**plus the method**:

> I'm also inclined to make these usability decisions based on interaction with
> actual software instead of based on a leaky theoretical model.

This is `spike-beats-the-argument.md` applied to interaction rather than to
mechanism. The right answer to *"which of these two should the design say?"* is
often *"build the smaller one and look at it."*

## What the runs then proved

Slice 009 had two human runs, and **both** found things no tier could.

- **VH-1** found AC-4 and AC-5 unmet. The defect — a slider drag dying
  mid-gesture, a form deaf to typing while the host talked to the backend — was
  invisible to the entire suite, which at the time contained no case that
  delivered a real key event to a text field and no case that operated any
  control while the host was busy. The whole of VH-1's value was in **one** of
  its three observations.
- **VH-2**, run against the repairs, gave a caret and a drag their **first
  positive readings** — and turned up a criterion passing that was nearly
  written off as a defect. A footer notice reading *"still working on the last
  request"* looked like the repaired behaviour failing; it was back-pressure on
  a capacity-1 channel, and the user's report of *which* fields reverted and
  which did not was AC-6 being read in the only tier that can see both halves at
  once.

A run reported as *"looked fine"* would have carried none of this.

## How to apply

- `docs/AGENTS.md` §Tiers: **a slice does not close until a person has run the
  software and seen the new behaviour.** A green gate is not that evidence —
  slices 001–003 all closed green on a binary that could not open a window.
- **Write the observation table before the run**, one row per thing to do, with
  *what it settles* and *what failure looks like* beside it. Ten rows took two
  short sessions here and discharged five acceptance criteria.
- **Record what was seen, in the runner's own words** — not that the run
  happened. Specific wrong-sounding observations are the valuable ones, and they
  are exactly what a summary destroys.
- **Commit the rig.** VH-1 used a scratch backend in a session scratchpad, so
  nothing about it survived and VH-2 had to be built again. Put the demo backend
  and its knobs in the repository, and document what each knob's output should
  look like — including the lines that look like defects and are not.
- Check the environment before believing a run: a stray enabled `systemd --user`
  unit put a **second** host and a second tray icon into VH-2's first run and
  nearly produced a phantom finding.

Related: `spike-beats-the-argument`, `getting-eyes-on-the-running-host.md`,
`a-green-test-can-assert-a-proxy.md`.

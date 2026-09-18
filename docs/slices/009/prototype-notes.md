# Prototype notes — slice 009

**This is not the slice.** It is a throwaway end-to-end build of `design.md`
as it currently stands, made on branch `slice-009-prototype` in a worktree at
`/home/david/dev/goad-009-proto`, while the design's adversarial review is
still open.

## Why it exists

Two aims, given by the user:

1. **Make progress.** The design has been through three review rounds and the
   plan has not started. Building the thing surfaces what reading it does not.
2. **Short-circuit design conversations by testing the assumptions.** Several
   of the remaining arguments are measurable (`slice-009.md` §Before design
   starts says so in as many words, and D-19 already made that trade once —
   the spike overturned the framing it was built to confirm).

The code is **referenced, not used**. Nothing here is promoted; the real slice
re-derives from the plan. What travels back into the slice is the *findings*
section of this file, and any canon or design change the user endorses.

## What "the current design" means here

`design.md` is current as of round 2's integration. Thirteen round-3 findings
are dispositioned and **user-confirmed** (D-23, D-24) but not yet integrated.
This prototype builds **design.md plus those thirteen**, because they are
decisions the user has already taken and building the superseded shape would
test nothing. `prototype-delta.md` is the implementer's statement of the
thirteen, extracted from `review-design.md`.

Where the delta and `design.md` conflict, the delta wins and the conflict is
recorded under Findings below.

## Rigour, deliberately uneven

The user set the standard: "as rigorous or loose as you think makes sense
given the aims." The split taken:

| area | standard | why |
|---|---|---|
| the mechanism the design turns on — two channels, the epoch, the guard, `resolve`, `pending`, the `Choose` flush | **full**: made to work, exercised by hand and by a probe | this is what the prototype exists to test |
| the pure functions the design specifies by contract — `slider_bounds`, the number parse rule, `compose`, `as_drawn` | **full unit coverage** | cheap, and each encodes a design claim that can be wrong |
| `cargo build --workspace` and `cargo clippy` | **green, always** | a red build is hostile to the next agent |
| the existing `tests/renderer/` suite | **allowed to rot** | §5.1's table says ~20 sites need rewriting for the `FieldForm`/`Reported` changes. That is real work for the slice and tests nothing about the design. Broken modules are disabled in `tests/renderer/main.rs` and listed below |
| the full §9 validation table — injection passes, tiering, negative controls | **not done** | it is the plan's job and it is most of the slice's cost |
| canon, `canon-delta.md`, the ledger | **untouched** | a prototype amends nothing |

## Disabled legacy test modules

<!-- Filled in as phases land. One line each: module, why, what it asserted. -->

## Phases

| phase | what | state |
|---|---|---|
| P1 | the split: two channels, epoch, guard, `pending`, and `text` end to end | pending |
| P2 | `number` — `Finite`, `slider_bounds`, the parse rule, both controls | pending |
| P3 | `choice` — `ComboBox`, `Chosen(AlternativeId)`, index resolution | pending |
| P4 | `datetime` — `instant.rs`, the jiff features, the two pickers | pending |
| P5 | demo backend of all five kinds, run it, and the harvest | pending |

Vertical rather than by layer, because every horizontal cut leaves a build
that cannot be run and therefore cannot be looked at, which is the one thing
`docs/AGENTS.md` §Tiers says a slice may not close without.

## Findings

<!-- The output that matters. One entry per thing the build taught that
     reading the design did not. Each says: what the design assumes, what was
     observed, and what it would cost the design to be wrong.
     P-n ids, immutable. -->

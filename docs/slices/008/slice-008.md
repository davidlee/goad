# Slice 008: the renderer gets a look

**Stage:** done
**Tier:** — see *How this slice is being run* below.
**Depends on:** 007 (closed). 006 is independent and still unopened.

## How this slice is being run

**The `docs/AGENTS.md` lifecycle is suspended for this slice by user
instruction (2026-09-16):** *"Ditch the methodology. Treat this as an
interactive pairing session. keep notes — what's on our list, what we've done —
in the slice notes."*

So there is no `design.md`, no `plan.md`, no review ledger and no phase sheet,
and the unused templates were removed rather than committed empty.
**`notes.md` is the whole record**: the list, the decisions with their
reasoning, what was measured, and what is still open. A reader wanting to know
what happened here reads that file and nothing else in this folder.

The scoping interview was abandoned after one question. Its answer stands and
is the scope: *everything listed* — window sizing, the prompt window's visual
pass, the idle surface, and the diagnostic pane's text behaviour.

Nothing here writes or amends canon, and nothing changes the wire contract. If
a visual decision turns out to need a protocol affordance, that is the signal
it belongs in a different slice.

## Purpose

007 drew the form and deliberately did not lay it out. The window it left
behind was, in the user's words, ugly — and carried a defect no gate and no
human observer had caught: **no content-derived preferred size at all**, 50×65
whatever was on it, at which one of two option controls is reachable.

Once this lands the window is worth looking at every two hours, which is how
often it appears.

## Scope

`crates/goad/ui/app.slint`, `crates/goad/build.rs` (the style default),
`crates/goad/src/diagnostics.rs` (host-side rendering of captured text),
`crates/goad/tests/renderer/`, and `assets/` for an embedded face.

## Summary

**Closed 2026-09-16.** The window is worth looking at, and the gate is green.

Six of the nine items on the list landed. The window now asks the compositor for
a size derived from its content — 420×159 for two plain options, 420×639 for a
form of forty fields, against **50×65 for everything** before — and holds
together at any height a tiler gives it. The surface has a style (`fluent`, after
all four built-ins were rendered and compared), a card per option, a panel per
block of fields, a three-step tonal ramp measured off the screenshot rather than
argued, a header card the schedule line now sits in, and an embedded face. An
option's control sits after the fields it carries: fill, then commit. A captured
stderr line's trailing newline no longer reaches the screen.

The diagnostic pane was **seen for the first time** and had five defects, four of
them on no list — no padding, lines distributed down the whole pane, none of the
visual pass reached, a full-width `Close` banner, and the known absence of word
wrap, which turned out not to be a `wrap:` property at all but an unconstrained
width inside a scroller. It is now aligned to the same 40px column as the form's
labels, with its report on the same tone.

Magnification was not inherited and was asked for this session. It is landed on
the tray, driven by the compositor's own scale factor, and **it survives a
present** — the question that decided whether it was worth building.

Two things about how this slice ran are worth carrying. The `docs/AGENTS.md`
lifecycle was suspended by user instruction and `notes.md` is the whole record;
that worked for a slice whose unit of work is *look at it and decide*, and it
would not have for one with a contract in it. And **the work was done with eyes
on the running host**, a screenshot per change — which is how six of the ten
defects repaired here were found at all, none of them by a test.

Five durable facts are lifted into `docs/memory/`:
`a-scroll-view-is-a-size-barrier-both-ways.md`,
`a-negative-control-that-does-not-compile.md`,
`getting-eyes-on-the-running-host.md`,
`window-zoom-is-the-scale-factor-event.md`, and `slint-styling-facts.md`.

## Follow-ups

> **Whether these are still open is `docs/follow-ups.md`'s**, not this
> section's. What is below is what slice 008 *raised* — the reasoning, and the
> price it was deferred against — and it stands as written, because it is a
> claim about what was decided then. The ledger carries the part that goes
> stale. This slice's rows: FU-4, FU-17, FU-20, FU-21, FU-22, FU-23, FU-24.
>
> Swept 2026-09-23 at `3ecaa11`. Anything below that the ledger does not list is
> struck in its §Closed table, with what killed it.

Live detail is in `notes.md` §*Still on the list*; these are the durable items.

- **L-4 — the diagnostic surface cannot be selected or copied.** The pane has
  had its visual pass; selection is the remainder, and it is the one part that
  changes the element tree — a read-only `TextEdit` per line in place of a
  `Text`, against a list `wiring.rs` selects by accessible label and item count.
  Do it with those two selectors in hand.
- **The diagnostic pane has no content-derived height** — L-1's cousin, left
  alone deliberately. The obvious binding is on the wrapped height, which depends
  on the width just pinned to the scroller: a plausible loop, and worth its own
  look rather than a guess.
- **L-6 — the idle surface**, undecided and a behaviour question rather than a
  layout one. With no view the surface is `Hidden` and the window with it, so
  there is nothing to lay out. Adjacent: closing the window quits the host
  (`install.rs:49`, a deliberate earlier decision), which makes the window
  something you cannot dismiss and get back.
- **L-7 — backend-authored strings reaching the screen are unbounded.** `title`,
  `option.label`, `block.heading`, while every diagnostic line in the same binary
  passes `finish(.., 1024)`. Untouched. Inherited from 007 `review-code.md` F-16,
  and it travels with `Diagnostics::of`'s unbounded *lists* (007 F-4): one
  decision about what bound, taken beside the three the module already states.
- **L-8 — the void below the form** under a tiling compositor. Stable and
  coherent now rather than scattered, but not designed. The same space L-6 might
  occupy.
- **L-9 — magnification: two things open by choice.** Persistence across a
  restart, deferred until the user has lived with it; and whether the tray is the
  shipping affordance or scaffolding towards `Ctrl +/-`. The second is gated on
  007's keyboard-focus follow-up either way — the costing is in
  `docs/memory/window-zoom-is-the-scale-factor-event.md`.
- **The standing schedule is shown in raw UTC** to a user on +10:00. Reading it
  costs a mental subtraction. A different kind of change from the rest of these
  and one to decide on its own.

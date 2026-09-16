# Slice 008: the renderer gets a look

**Stage:** in progress
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

## Where it stands

Landed: the window sizes to content; the content holds together at any window
height; the style, the option cards, the block panels, the tonal ramp, the
header card and the embedded face; and a captured stderr line's trailing
newline no longer reaches the screen.

Open: the diagnostic surface (no word wrap, no text selection), the idle
surface, the unbounded backend-authored strings, and what the space below the
form is for. `notes.md` §*Still on the list* is the live version of this
paragraph and the one to trust.

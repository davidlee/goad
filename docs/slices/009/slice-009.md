# Slice 009: the form grows the rest of its field kinds

**Stage:** scoping
**Tier:** 2 (full) — see *Why tier 2* below.
**Depends on:** 007 (closed, the form) and 008 (closed, the look). 006 is
independent and still unopened.

## Purpose

`SPEC-001/R-16` admits five field kinds and `R-57` types what each one submits.
The renderer draws **one** of them. `text`, `number`, `choice` and `datetime`
are reported through `Undrawn::FieldForm` (`view_model.rs:230`) — correct
behaviour under `R-55`, and a renderer subset that has now stood for two
slices.

This slice draws the other four, so a backend can ask for a note, a quantity,
a selection and an instant, and get back what `R-57` says it will. It adds no
protocol. It discharges the standing hazard slice 002 recorded and 007 halved.

It also settles something 007 left open and typed input makes urgent: **a
present rebuilds the whole form**, which costs a checkbox its focus ring and
costs a text field every keystroke after the first.

## Scope

- `crates/goad/ui/app.slint` — the field repeater, `FieldRow`, the `edited`
  callback.
- `crates/goad/src/view_model.rs` — the mapper's `FieldKind` match and
  `FieldForm`.
- `crates/goad/src/draft.rs` — `Edited`, `submitted`, `state_of`.
- `crates/goad/src/glass.rs` — how a present writes the options model.
- `crates/goad/src/controller.rs` — the `Edit` path and the text debounce.
- `crates/goad/src/wire.rs`, `src/install.rs` — the callback's two ends.
- `crates/goad/tests/` — including, if the design calls for it, a new
  event-loop-backed target.

## Non-goals

- **No protocol change.** Two amendment candidates were identified at scoping
  and both are deliberately out; `research.md` §Amendment candidates records the
  argument for each.
  - **`step` on a `number`** stays a hint at most. A slider without a step is
    continuous and `R-57` requires no integrality.
  - **`SPEC-001/OQ-4`** — a date without a time — stays shut. `docs/roadmap.md`
    names this slice as its answerer; drawing the control is what produces the
    evidence, and the residue is additive either way. If design finds a
    date-only field cannot be expressed at all, that reopens it at no tier cost.
- **No `field.value` prefill and no per-field errors** (`SPEC-001/OQ-2`). Still
  unlanded, still the trigger 007 named: when use says a form must *reject* an
  answer. Its absence is what forces AC-6.
- **No answer to `SPEC-002/OQ-4`** — a scheduled firing superseding a view
  mid-answer. More visible once a person is typing into the form; still a
  protocol question and still not this slice's.
- **Not 008's remaining list** — the idle surface, unbounded backend strings,
  selection in the diagnostic pane. 007 and 008 were split apart so a layout
  regression and a behaviour regression would not look alike in one diff; that
  reason has not expired.

## Why tier 2

Not canon — the non-goals above keep the protocol untouched. **Size.** The
design must settle the widened `FieldRow` and its callback, a per-kind as-drawn
value, when a present may write in place rather than rebuild, the epoch and its
guard, a text debounce with a flush point, five widget mappings, the
composition of a conforming `datetime`, and a test tier for a mechanism the
existing one cannot observe. Compressing that under tier 1's 300-line cap is
the one way `docs/AGENTS.md` §Tiers names as failing the rule dishonestly.

## Acceptance criteria

- [ ] AC-1 — A view whose option carries a `text`, a `number`, a `choice` and a
      `datetime` field draws all four, in declared order, alongside a `boolean`.
- [ ] AC-2 — Answering that option sends a `respond` whose `values` carry the
      JSON type `SPEC-001/R-57` names for each kind: a string, a number, the
      chosen alternative's id as a string, and an RFC 3339 `date-time` with an
      explicit offset.
- [ ] AC-3 — `R-58` still holds: the map carries a value for exactly the fields
      the host drew of the option answered, and no others.
- [ ] AC-4 — Typing into a `text` field records every character. Stated as an
      observable because the defect it excludes — one character per click —
      is what the re-present work exists to prevent.
- [ ] AC-5 — A present that changes nothing about a field does not disturb it:
      no destroyed element, no moved caret, no interrupted drag. Asserted in a
      tier that can actually observe a `changed` handler (`research.md` Thread 3).
- [ ] AC-6 — A-2 still holds with the element preserved: a widget whose edit the
      host refused or dropped is corrected by the next present. Negative-controlled.
- [ ] AC-7 — A field kind the renderer cannot draw is still reported through
      `Undrawn`, and the mechanism that makes a sixth kind a compile error
      survives — `R-55` is discharged for five kinds, not deleted.
- [ ] AC-8 — A `choice` field submits an **alternative** id, and nothing in the
      host confuses that namespace with an option id (`R-52`, `R-53`).
- [ ] AC-9 — A `number` whose `min`/`max` are absent draws something that
      submits a number, without the host inventing a range.
- [ ] AC-10 — `just check` exits 0, and a person has run the software and
      answered a form containing all five kinds (`docs/AGENTS.md` §Tiers).

## Governing canon

Binding: **SPEC-001** R-16, R-17, R-18, R-35, R-52, R-53, R-55, R-57, R-58.
**ADR-001** and **POL-001** — all of this slice is stratum 3; nothing reaches
`goad-semantics`.

Checked and not applicable: **SPEC-002** and **ADR-004** (scheduling — one
adjacency recorded at OQ-3 below, not acted on); **SPEC-003** and **ADR-005**
(event ingress — untouched); **ADR-002**/**ADR-003** (no new workspace member;
the spike is a standalone workspace, deleted at `a698217`'s successor).

`research.md` Thread 1 carries the clause-by-clause reading.

## Open questions

- OQ-1 — **What does each kind submit when the person has not touched it?**
  `Draft::state_of` answers `Checked(false)` as-drawn, which is the protocol's
  answer for a boolean and has no counterpart for a bounded number, a choice
  with alternatives, or a datetime. `R-58` forbids omitting the value and
  `OQ-2`'s prefill is unlanded, so the design must state a per-kind as-drawn
  value and own the consequence. The sharpest undesigned question in the slice.
- OQ-2 — **What does a conforming `datetime` control compose into?** Both
  pickers are popups and neither can be repeated, so there is no inline control;
  `R-57` wants an RFC 3339 instant with an offset while `DatePickerPopup` yields
  a bare `{year, month, day}`. This is the kind most likely to reopen
  `SPEC-001/OQ-4`.
- OQ-3 — **Where does the text debounce flush?** 150 ms is the agreed starting
  value (user, 2026-09-16), configurable per backend later. A debounce means the
  draft lags the widget, so a person who types and immediately clicks the option
  button could submit the previous text. On `accepted`, on focus loss, or before
  building the `respond` — with a data-loss failure behind the wrong answer.
- OQ-4 — **Which tier tests the re-assert, and what is knowingly left to a
  person?** `changed` fires nowhere under `init_no_event_loop`, so the whole
  `tests/renderer/` tier would be vacuously green on it. Either a new
  one-test-per-binary target in the `event_loop_schedule` shape, or a stated gap.
- OQ-5 — **When may a present write in place rather than rebuild?** The shape
  changing (a new view, a different option or field set) plainly needs a
  rebuild; a value-only change plainly does not. The boundary between them, and
  what happens on a refused command, is design.

## Summary

<!-- Written at close. -->

## Follow-ups

<!-- Written at close. -->

# Slice 009: the form grows the rest of its field kinds

**Stage:** design
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

- `crates/goad/ui/app.slint` — the field repeater, `FieldRow` and `FieldValue`
  as two channels, the `edited` callback, the epoch and each control's guard,
  and the two picker popups.
- `crates/goad/src/view_model.rs` — the mapper's `FieldKind` match, `DrawnKind`,
  `FieldForm`, and the kind-directed `as_drawn` / `resolve`.
- `crates/goad/src/draft.rs` — `Edited`, `Reported`, `Finite`, `submitted`,
  `state_of`.
- `crates/goad/src/glass.rs` — how a present writes the two models, including
  the value channel's overlay of `pending.rs` (`design.md` §5.3, §7 D26).
- `crates/goad/src/controller.rs` — the `Edit` and `Choose` paths, including the
  per-edit identity check on the edits a `Choose` carries.
- `crates/goad/src/wire.rs`, `src/install.rs` — the callback's two ends;
  `Command::Choose`'s new shape, and `Wire::send` reporting whether the command
  was enqueued.
- `crates/goad/src/main.rs` — one `Rc` for the pending map, created before the
  callback table and cloned into `install` and `SlintGlass::new` both.
- `crates/goad/src/pending.rs`, `src/instant.rs` — new: the debounce, keyed by
  (option, field) with the view each edit was made on and a timer that re-arms
  while the map is not empty; and the clock and system-zone reads a `datetime`
  needs.
- `crates/goad/Cargo.toml` — `jiff`'s `tz-system` and `tzdb-zoneinfo`, on the
  one member that reads the system zone (`design.md` §10, §7 D18).
- `crates/goad/tests/` — including one or more new event-loop-backed targets.

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
      host refused or dropped is corrected by the next present.
      Negative-controlled. *Dropped* means the host holds the value in neither
      the draft nor `pending.rs`: an edit still waiting on the debounce is shown
      rather than corrected, which is what makes a present landing inside that
      window harmless (`design.md` §5.5 I-H, §7 D26).
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
`goad-semantics`. `POL-001` §Verification also names the residue this slice
triggers, a feature switched on in a dependency stratum 1 shares; `design.md`
§10 carries the argument it requires.

Amended, not merely read: `canon-delta.md` holds **CD-1** (`SPEC-001` §7 gains
what an untouched field submits per kind, the `datetime` epoch and the `max`-only
consequence) and **CD-2** (`SPEC-001` §Verification's `R-55`, `R-57` and `R-58`
rows). Both are the slice's working authority while it runs and are promoted at
audit with explicit endorsement. Nothing outside this slice may cite them.

Checked and not applicable: **SPEC-002** and **ADR-004** (scheduling — the one
adjacency is the `SPEC-002/OQ-4` non-goal above, not acted on); **SPEC-003** and **ADR-005**
(event ingress — untouched); **ADR-002**/**ADR-003** (no new workspace member;
`spike-fields/` is a standalone cargo project rather than a member, committed at
`4f93d41` and deleted when the design closes — D-19).

`research.md` Thread 1 carries the clause-by-clause reading.

## Open questions

All five opened at scoping are closed. Each was a user decision; the argument is
in `design-log.md` and the answer is in `design.md`, and neither is repeated
here.

| | question | closed by |
|---|---|---|
| OQ-1 | what each kind submits untouched | D-6 — as-drawn is what the widget shows; `datetime` is the epoch (`design.md` §5.2) |
| OQ-2 | what a conforming `datetime` composes into | D-7 — the offset the person picked in, chained pickers, cancel abandons (§5.2, §5.4) |
| OQ-3 | where the text debounce flushes | D-8 — on answer, and nowhere else; the flush travels inside `Command::Choose` (§5.4) |
| OQ-4 | which tier tests the re-assert | D-10, widened at D-14 — the loop tier takes whatever targets its rows need, one arrangement each; the caret is a person's (§9) |
| OQ-5 | when a present may write in place | D-9 — same `view_id`; two channels rather than a retained tree (§5.3) |

One question is open and deliberately deferred: **whether the `datetime` epoch
is stated normatively or descriptively in `SPEC-001`** (`canon-delta.md` CD-1).
Deferred to promotion at audit by explicit user decision, on the ground that it
is a question about how canon should read rather than about what the code does.

`SPEC-001/OQ-4` — a date without a time — **stays shut**, and the non-goal above
set the condition under which it would have reopened. It was not met: a date-only
field can be expressed as a `datetime` at 00:00 local. `design.md` §10 states the
evidence, which is about the affordance rather than the expressiveness.

## Before design starts

`research.md` Thread 4 is the part that is easiest to skip and most expensive to
rediscover: four mechanisms already considered and rejected with the reason, one
available but untaken, and one claim flagged as unverified precisely because it
would be attractive to assume. Read it before proposing an approach.

Two process notes from scoping, offered rather than imposed:

- **The load-bearing questions here were measurable, not arguable.** Scoping
  produced a spike instead of a design section and the spike overturned the
  framing it was built to confirm — the re-present problem was priced as a
  second design surface and is one decision. Expect the same of the remaining
  questions: OQ-1 and OQ-2 in particular are likely cheaper to settle by drawing
  the thing than by reasoning about it.
- **Some of this is only observable with a real window.** `docs/memory/
  getting-eyes-on-the-running-host.md` has the launch and screenshot mechanics,
  including two surfaces the loop cannot reach at all.

## Summary

<!-- Written at close. -->

## Follow-ups

<!-- Written at close. -->

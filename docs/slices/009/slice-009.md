# Slice 009: the form grows the rest of its field kinds

**Stage:** done
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
  `FieldForm`, and the kind-directed `as_drawn` / `interpret` (`design.md` §5.2
  states why the name may not be `resolve`).
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
- `crates/goad-shell/src/clock.rs` — one doc-comment amendment and no code
  change: the rationale it carries for avoiding `jiff::Timestamp::now()` stops
  being true of the workspace build once that feature lands (`design.md` §10,
  D-35).
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

- [x] AC-1 — A view whose option carries a `text`, a `number`, a `choice` and a
      `datetime` field draws all four, in declared order, alongside a `boolean`.
- [x] AC-2 — Answering that option sends a `respond` whose `values` carry the
      JSON type `SPEC-001/R-57` names for each kind: a string, a number, the
      chosen alternative's id as a string, and an RFC 3339 `date-time` with an
      explicit offset.
- [x] AC-3 — `R-58` still holds: the map carries a value for exactly the fields
      the host drew of the option answered, and no others.
- [x] AC-4 — Typing into a `text` field records every character. Stated as an
      observable because the defect it excludes — one character per click —
      is what the re-present work exists to prevent.
- [x] AC-5 — A present that changes nothing about a field does not disturb it:
      no destroyed element, no moved caret, no interrupted drag. Asserted in a
      tier that can actually observe a `changed` handler (`research.md` Thread 3).
- [x] AC-6 — A-2 still holds with the element preserved: a widget whose edit the
      host refused or dropped is corrected by the next present.
      Negative-controlled. *Dropped* means the host holds the value in neither
      the draft nor `pending.rs`: an edit still waiting on the debounce is shown
      rather than corrected, which is what makes a present landing inside that
      window harmless (`design.md` §5.5 I-H, §7 D26).
- [x] AC-7 — A field kind the renderer cannot draw is still reported through
      `Undrawn`, and the mechanism that makes a sixth kind a compile error
      survives — `R-55` is discharged for five kinds, not deleted.
- [x] AC-8 — A `choice` field submits an **alternative** id, and nothing in the
      host confuses that namespace with an option id (`R-52`, `R-53`).
- [x] AC-9 — A `number` whose `min`/`max` are absent draws something that
      submits a number, without the host inventing a range.
- [x] AC-10 — `just check` exits 0, and a person has run the software and
      answered a form containing all five kinds (`docs/AGENTS.md` §Tiers).

## Governing canon

Binding: **SPEC-001** R-16, R-17, R-18, R-35, R-52, R-53, R-55, R-57, R-58.
**ADR-001** and **POL-001** — this slice is stratum 3 with **one declared
exception**: PHASE-09 adds `Alternatives::first` to
`crates/goad-semantics/src/protocol/canonical.rs`, with explicit user
endorsement (2026-09-19), declared in that phase's Surfaces and carried in
`plan-log.md`. The addition is pure — it names no `src/shell/`, reads no clock,
filesystem or subprocess, and adds no dependency — so ADR-001 holds; what would
have been false is the sentence this replaces, *"nothing reaches
`goad-semantics`"*. `POL-001` §Verification also names the residue this slice
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

The renderer draws all five field kinds. A backend can now ask for a note, a
quantity, a selection and an instant, and gets back what `R-57` says it will —
verified per kind against the child process's own request log rather than by
review. No protocol was added. The standing hazard slice 002 recorded and 007
halved is discharged.

**The larger change is underneath.** A present used to rebuild the whole form,
which cost a checkbox its focus ring and a text field every keystroke after the
first. It now writes through two channels behind a guard, so a present that
changes nothing changes nothing — no destroyed element, no moved caret, no
interrupted drag.

**Three things this slice learned the hard way, all at audit.**

- **`busy` was deaf, not slow.** It meant *the host is talking to the backend*,
  and a disabled Slint item **discards** input rather than queueing it, so every
  character typed during a routine poll was lost. AC-4 and AC-5 were both unmet
  from that one cause. It was invisible to the entire suite — nothing delivered
  a real key event to a text field, and nothing operated a control while `busy`
  was true — and it took a person running the software to find it. Both criteria
  were repaired rather than waived.
- **A repair is where the next defect is.** Four review rounds, each one over
  the previous round's repairs; rounds 2 and 3 each found a major *in a repair*,
  and twice a repair re-created the very defect its commit was closing.
- **The code is held by 565 cases and the prose is held by nobody.** Rounds 3
  and 4 found almost nothing but sentences: counts, a step number, a cost priced
  by a mechanism that does not exist, and a citation class that had to be
  measured before it could be believed — 53 in-repo line citations, 27 pointing
  at the wrong thing, thirteen of them from one file's drift. `CLAUDE.md` now
  carries the rule, and the class is down to 17 citations with none wrong.

**Evidence:** `just check` exits 0 — 30 `test result: ok` lines summing to 600,
which is 565 distinct cases across 22 targets with `goad-semantics`' 35 run twice
under two feature configurations. Two human runs, VH-1 and VH-2, thirteen
observations between them. `review-code.md` holds 47 findings across four rounds,
all closed. `audit.md` holds the reconciliation: sixteen documents changed, two
canon deltas promoted.

## Follow-ups

Each is owned work, not a wish. The first five were dispositioned `follow-up`
against a stated price during the audit; the rest were found at close.

| # | what | why it is not in this slice |
|---|---|---|
| 1 | **`F-R4`** — one full present per **refused** ingress arrival, including `show()`'s instantiation pass, at a rate an untrusted writer sets | The question underneath it is canon's. `SPEC-003/R-15` requires a refusal decided **while idle** to reach the diagnostics surface, so suppressing the present defers that to the next scheduled firing — and R-15's own verification case reads the retained model rather than the window, so canon's instrument would not report the change. That is a spec amendment with its own verification, not a repair. Splitting `option_models`' single walk would also reintroduce the second counter invariant **I-B** forbids |
| 2 | **Slider quantisation** — snap the reported value to `step` | Makes `step` normative for the **value**, which §Non-goals above declined. The readout landed in this slice instead, so screen and wire agree by construction; it displays the full unrounded spelling, which is ugly, and that ugliness is the argument for this follow-up in a form a person can see rather than one that has to be explained |
| 3 | **`F-S5`** — *every write to a guarded widget goes through the counter* is real and held by nothing. Hoisting an assignment out of its comparison leaves every `-p goad` target green with `reasserts` at `0` | Priced against its **canon** cost, not its code cost: the markup scan that would hold it is a fifth boundary instrument, and `POL-001` §Verification enumerates its instruments while `CLAUDE.md` forbids compressing them into one count. A policy amendment taken mid-audit for a `minor` was the wrong trade |
| 4 | **Four of six `enabled: !root.busy` bindings are held by nothing** — the `number` `Slider`, the `number` `LineEdit`, the `choice` `ComboBox` and the `datetime` `Button`. Measured at close: removing all four leaves every target green | Same class as #3 and found the same way. The two that *are* held are held by the two cases this audit added. Not a defect — the bindings are present and correct, and production disables all six |
| 5 | **`F-B4`'s harness half** — all three new loop targets fail at roughly **6x** CPU oversubscription, and the failure is the **liveness backstop** rather than any assertion: the slint-timer stepper stalls for tens of seconds and `LIVENESS_BOUND` turns the stall into a red that reads like a defect | A 40x nominal margin was not enough, so widening the bounds is not the repair — the harness is. Two independent witnesses, and a load-sensitive `just check` sits against `POL-001`'s *the gate exits 0* |
| 6 | **The tray icon has no re-assertion path.** `glass.rs` calls `set_image` on every present, but `tray_icon` returns a stable-address clone and slint's `ChangeTracker` fires only on `!=` — so nothing re-registers an icon the platform has dropped | Raised as a follow-up and **not** as a finding, deliberately: the non-recovery mechanism is confirmed, the **cause of the disappearance** is not, and the witness was hedged. The durable class is the part worth keeping — *a repair that removes a redundant write also removes the self-healing that redundancy was accidentally providing*. `F-R5` was right; this is its unpriced half |
| 7 | **`next check (instructed)` is false whenever nothing was ever instructed.** The value is `now + default_poll` — `R-26`'s third branch, which R-26 distinguishes from an instruction in as many words. A backend that never sends `next_check` is legitimate, so for it the label is **always** wrong | Outside this review's subject: the string is slice **003**'s. And it is not a relabel — `schedule.rs::resolve` returns a bare `Timestamp`, discarding the branch at the moment it takes it, so making the line honest means `resolve` reporting its branch through `State` to the diagnostics line: a change to a **pure stratum-1 function**, its callers and its verification. It also carries a spec question — what the line should say for each of R-26's three branches — that belongs in `SPEC-002` §6 |
| 8 | **Land the citation resolver as a gate instrument.** Twenty lines: resolve every in-repo `file.rs:NNN` in a comment and fail when one points at the wrong thing | The discipline is in `CLAUDE.md` and nothing enforces it. The class regenerated **twice inside the commits that repaired it**, and both times a script caught what two careful readings had missed. Not landed here on the standing decision to close this slice rather than spike an instrument |
| 9 | **`SPEC-001` OQ-4's wording** — deliberately unamended, and an open discussion rather than an omission | The audit's position is recorded in `audit-log.md`: the fork is **asymmetric**, because `R-18` already permits a renderer and only a renderer to branch on a hint, so the hint half needs no protocol change at all. The clause *"no evidence asks for one yet"* is true on its own terms and is now the wrong sentence — what 009 found is an **affordance cost**, not an inexpressibility. `roadmap.md` carries the narrative |

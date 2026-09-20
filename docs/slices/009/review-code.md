# Review — implementation — Slice 009

**Subject:** implementation — `a698217..HEAD` on `main`, 107 commits.
35 files and +7535/-595 under `crates/`. The subject is the code, not the
record: document-truth divergences are the audit's reconciliation, and appear
here only where a doc comment or a declared surface makes a *code* claim that
is false.
**Reviewer:** fresh agents, three dimensions, round 1 — Opus 5
**Opened:** 2026-09-20
**State:** open

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

Written before the review. This slice took a renderer that drew one of five
field kinds and made it draw all five, and in doing so replaced the mechanism
by which a present reaches the form. Three surfaces carry that, and each has a
different way of being wrong.

**Where the bodies are likely buried.**

- **The pure layer decides what reaches the wire.** `R-57` types what each kind
  submits and `R-58` says which fields carry a value at all. Everything between
  a widget and the wire is `draft.rs`, `view_model.rs`, `instant.rs` and
  `controller.rs`, and every one of those is total by construction *on the
  author's account*. Totality claimed in a doc comment is not totality.

- **Two channels and a guard replaced a rebuild.** `glass.rs`, `app.slint`,
  `install.rs`, `wire.rs`, `pending.rs` and `main.rs` now share mutable host
  state across a Slint callback boundary, an `Rc`, a timer and an async serve
  loop. The design names re-entrancy, write order and handle-splitting as risks
  and argues each away in prose.

- **A suite that grew by 1770 lines in one file.** The project's recorded
  failure mode is a green case that asserts a proxy for the thing it names
  (`docs/memory/a-green-test-can-assert-a-proxy.md`; slice 004 shipped four).
  Every new case here claims an injection pass. An injection pass proves a case
  can go red — not that it goes red for the defect the criterion is about.

**Invariants the subject is held to.** `CLAUDE.md`'s five, in full. In
particular: **no backend input may take the host down or leave a refusal
unattributed**, and **wire compatibility is never narrowed to fit the
renderer** — which in this slice runs the other way, since the renderer subset
is what was deleted. Plus `SPEC-001` R-16, R-17, R-18, R-35, R-52, R-53, R-55,
R-57, R-58; `ADR-001` and `POL-001` (stratum 1 stays pure, and the gate's four
instruments do not reach the dependency-feature residue).

**Round 1** — 2026-09-20 — three dimensions in parallel: the pure layer and the
wire contract; the renderer, its shared state and its markup; the suite and
what each case would survive. Each reviewer was given a surface and no
conclusion.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-A1 | blocker | fix-now | verified |
| F-S2 | major | fix-now | verified |
| F-S1 | major | fix-now | verified |
| F-P1 | minor | doc-wrong | verified |
| F-P2 | minor | doc-wrong | verified |
| F-R1 | major | fix-now | verified |
| F-R2 | major | fix-now | verified |
| F-S3 | major | fix-now | verified |
| F-R3 | major | fix-now | verified |
| F-S4 | minor | fix-now | verified |
| F-S5 | minor | fix-now | **contested** → re-open |
| F-R4 | minor | fix-now → **follow-up** | verified |
| F-R5 | minor | fix-now | verified |
| F-R6 | minor | doc-wrong | verified |
| F-R7 | minor | doc-wrong | verified |
| F-P3 | nit | fix-now | verified |
| F-P4 | nit | fix-now | verified |
| F-S6 | nit | fix-now | verified |
| F-S7 | nit | *settle first* → **doc-wrong** | verified |
| F-R8 | nit | fix-now | verified |
| F-R9 | nit | fix-now | verified |

**Round 2 opened** — 2026-09-20, over the repairs. Two dimensions, each a fresh
agent in its own worktree, each told not to read `audit.md` so that the
orchestrator's own unraised leads could not become an echo.

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-T1 | minor | | |
| F-T2 | minor | | |
| F-T3 | minor | | |
| F-T4 | nit | | |
| F-B1 | major | | |
| F-B2 | major | | |
| F-B3 | minor | | |
| F-B4 | minor | | |
| F-B5 | minor | | |
| F-B6 | minor | | |
| F-B7 | minor | | |
| F-B8 | nit | | |
| F-B9 | nit | | |

**The behaviour dimension** held the six findings whose repair produced
behaviour — `F-A1`, `F-R2`, `F-R3`, `F-R1`, `F-R5`, `F-R8` — and applied every
mutation each Response names to production code rather than reading the account.
**All six are `verified`; none is contested.** Two Responses are *understated* by
their own evidence: F-A1's reversion reddens three targets in two tiers rather
than the two claimed, and F-R2's markup claim — run rather than taken — makes
`event_loop_busy` **the first case in the project that can see an `enabled`
binding at all**. Where a repair had no case, the reviewer built a probe rather
than accepting the Response (F-R5), and the probe is what exposed **F-B6**.
**Eight new findings, `F-B1`–`F-B8`**, two of them `major`; both majors are about
what *holds* a repair or what a repair did not reach, not about a line that is
wrong. It also settled, on its own evidence rather than inheriting it, that
`wiring.rs`'s first `busy` case is now unreachable — **F-B7**.

**`F-B9` is the orchestrator's**, raised as declared raiser after both dimensions
closed. It was held back from both briefs; the behaviour dimension reached the
surface and excluded it on a test that does not cover a repeater.

**The instruments dimension** re-ran every mutation named in all six Responses
it held, applied to production code and restored from a copy, and every one
reproduced its reported reading — F-S4's four included, each confirmed to have
*compiled*. Five verified. **F-S5 is `contested` on a measured line** and returns
to open. It also ran two claims the Responses had reported as *checked* rather
than run — F-S6's whole-map `tick`, and F-S3's surviving shape — and both hold.

**The Outcome column, and who sets it.** Round 1's three raisers were agents
that no longer exist, so the protocol's *"Outcome — set by the raiser"* has no
author to return to. The user's decision (`audit.md`, *The Outcome split*)
divides it by what the repair produced rather than by severity:

- **Round 2's reviewers**, as raisers, set the Outcome for the twelve findings
  whose repair produced **code** — `F-A1`, `F-R2`, `F-R3`, `F-R1`, `F-R5`,
  `F-R8`, and `F-S1`–`F-S6`. A fresh adversarial eye over the work each
  Response describes writes `contested` on evidence rather than an opinion.
- **The orchestrator**, as **declared raiser**, sets it for the nine whose
  repair is a **document** and where there is no code to attack — `F-P1`–`F-P4`,
  `F-R4`, `F-R6`, `F-R7`, `F-R9`, `F-S7`. Each of those Outcomes says so in as
  many words, and each was checked against the tree rather than against the
  Response's account of itself.

**Round 1, completed.** **Twenty-one** findings: one blocker, six majors,
eight minors and six nits.

The renderer dimension reported `F-R1` and `F-R2` and then stopped, leaving two
of its briefed areas unattacked — shared mutable state with handle splitting
and re-entrancy, and resource behaviour. Audit session 2 ran those two as a
fresh agent rather than reading the absence of findings as a clean surface;
`F-R3`–`F-R9` are what it found, and they close round 1. Every one is reasoned
from the tree and the vendored `slint 1.17.1` sources rather than run, and each
severity line says so.

(Clerical, audit session 2: the table above carried `F-P1` and `F-P2` twice and
the count read *eleven*. No finding was added, removed or altered.) F-S1 and F-S2 were
mutation-confirmed by the audit; F-R1 is reasoned from locked sources and
**not run**, and says so.

### F-A1 — every field control is deaf for the whole backend round trip, so a keystroke typed during an exchange is discarded, not deferred

**Severity:** blocker
**Location:** `crates/goad/src/controller.rs:921-922` and `:195`;
`crates/goad/ui/app.slint:128`, and the seven `enabled: !root.busy` sites at
`:423`, `:476`, `:547`, `:610`, `:683`, `:743`, `:784`

**Expected.** `slice-009.md` **AC-4**: *"Typing into a `text` field records
every character. Stated as an observable because the defect it excludes — one
character per click — is what the re-present work exists to prevent."* And
**AC-5**: *"A present that changes nothing about a field does not disturb it:
no destroyed element, no moved caret, no interrupted drag."*

**Observed.** `Controller::engage` sets `engaged` and `serve` presents with
`busy = true` before **every** exchange (`controller.rs:921-922`), a routine
scheduled poll included. `engaged` is not cleared until `absorb` folds the
completed exchange (`controller.rs:195`), so `busy` is true for the **entire
backend round trip**, not for a frame. `frame.busy` disables all seven field
controls.

Slint does not queue input for a disabled item — it drops it. `TextInput::
key_event` returns `EventIgnored` when `!enabled` before any handling
(`i-slint-core-1.17.1/items/text.rs:954`), and `TextInput::input_event` does
the same for pointer events (`:848`). `LineEdit` binds `enabled <=>
text-input.enabled` (`widgets/common/lineedit-base.slint:11`), and a `Slider`
reaches `enabled <=> touch-area.enabled`.

So the class is one sentence: **any key or pointer event delivered to any field
control while an exchange is in flight is discarded.** Three distinct costs,
one cause:

- a `Slider` mid-drag loses its grab — VH-1's measured failure, AC-5;
- a `LineEdit` mid-word **loses the characters typed during the exchange** —
  AC-4, and the one defect AC-4 was written to exclude;
- every other control flashes disabled — cosmetic.

**What VH-1's run could not see, and why it is not evidence against this.** The
caret *is* preserved: nothing issues a focus-out on disable, and `text.rs:1990`
only stops *rendering* the cursor (`cursor_visible() && self.enabled()`). So
*"the caret stayed where it was put"* is true and is a different claim from
*"every character was recorded"* — the same proxy shape the drag finding turned
on. The rig's scratch `backend.sh` answers in milliseconds, so the deaf window
was too short to type into. It is the exchange's duration, not the present
interval, and a backend that takes 300 ms makes this routine.

**Evidence.** Concrete and checkable without a person: hold a backend's reply
for 500 ms, present, type during the wait, and the characters are absent from
the draft and from the wire. No case in any tier delivers an input event to a
control while `busy` is true — `tests/renderer/fields.rs` drives
`set_accessible_value`, which assigns `text` and calls `edited` directly
(`fluent/lineedit.slint:16`) and therefore bypasses `key_event` and its
`enabled` guard entirely. That is why the suite is green over a criterion it
does not reach.

**Evidence, completed by the audit (session 2) — the accessibility surface
bypasses `enabled` on *every* control, not only on a `LineEdit`.** F-A1 names
one road (`set_accessible_value`). The other is
`ElementHandle::invoke_accessible_default_action`
(`i-slint-backend-testing-1.17.1/search_api.rs:606-613`), which calls
`item.accessible_action(&AccessibilityAction::Default)` with **no
`accessible-enabled` check**, and `fluent/button.slint:34` implements that
action as `i-touch-area.clicked()` — the callback, not the event. So a
disabled `Button` still raises `root.clicked()` through the test surface. That
is the mechanism behind the audit's enumerated fact that *deleting `enabled:
!root.busy` from any of this slice's five new controls leaves all 592 green*:
**no `enabled` binding in this markup is observable through the surface the
suite drives.** It bears directly on the repair — a case that claims the
narrowing works must deliver a real pointer or key event, never an accessible
action.

**Relation to the record.** `design.md` **A-6** — *"Disabling a widget while an
exchange is in flight does not destroy it … being wrong costs focus, not
data"* — is the assumption this falsifies. `notes.md`'s VH-1 block already
records the drag half and prices the class as *"a one-frame flash of the whole
form when idle, a dead drag when a gesture is in flight."* The flash is not one
frame and the loss is not only a drag: it is an exchange-long deafness, and
AC-4 is inside its blast radius.

**Disposition:** `fix-now`
**Response:** Applied. `Controller::engage` takes the
`Exchanged` and sets `engaged = exchanged == Exchanged::Answer`
(`controller.rs`); `serve`'s one call site passes the `exchanged` it already
computes. `busy` now means *your answer is in flight*, so the form stays live
through a scheduled poll, a tray check and an ingested event, and the option
`Button`'s slice-003 double-submit guard fires exactly when it was written to.
Corrected with it: `engage`'s doc and its F-21 argument, `Frame::busy`'s, the
`Controller` struct's, the inline comment at the busy present, and
`app.slint`'s `busy` comment.

Measured where it can be: `tests/event_loop_busy/` delivers a **real** key
event to a focused `LineEdit` in a laid-out window under three frames —
nothing engaged (the control), an evaluation in flight (the claim), an answer
in flight (the contract kept) — because, as this finding establishes, no case
driven through the accessibility surface can see an `enabled` binding at all.
`tests/event_loop_drain/` makes the same claim through the production `serve`
with a real exchange outstanding. Injection: `engage` reverted to
`self.engaged = true` reddens both. `notes.md` §*Audit session 2* has the
readings.

**Outcome:** `verified` — set by round 2's behaviour dimension as raiser.

**Evidence.** The mutation the Response names, run rather than read:
`controller.rs:418-420` reverted to `let _ = exchanged; self.engaged = true;`,
then `cargo test -p goad --no-fail-fast`:

| target | result |
|---|---|
| `event_loop_busy` | **FAILED** at `busy.rs:296` — *"a key typed while the host is polling the backend must be recorded"*, `left ["v1/morning/noted=a"]` / `right […, "…noted=b"]` |
| `event_loop_drain` | **FAILED** at `drain.rs:462` — `left "x"` / `right "xy"` |
| `renderer` | **FAILED** — `table::busy::an_evaluation_does_not_engage_and_an_answer_does` |
| every other `-p goad` target | green |

**The Response's *"reddens both"* is understated: three targets redden, in two
tiers.** The slint half of the claim was re-derived from the vendored
`i-slint-core-1.17.1` sources and is unchanged.

**The class, re-enumerated at the markup.** After the narrowing, six of
`app.slint`'s seven `enabled: !root.busy` sites are no longer disabled by an
exchange the person did not start, and the seventh — the option `Button` — is
disabled exactly for the answer it guards: `controller.rs:312` is
`self.shown.as_ref().ok_or(Refused::SupersededView)?`, so `choose` is the only
road to `Pending::Respond` and it needs a shown view.

**What the repair does not reach, carried forward rather than held against it.**
The `engage` **call site** inside `serve` is held by nothing — **F-B1**. The
production line is correct as written; that is a coverage finding.

### F-S2 — the enqueue rule has no case in the direction that loses a person's typing

**Severity:** major
**Location:** `crates/goad/src/pending.rs:212-214` (`if enqueued { … remove }`)

**Expected.** `plan.md` PHASE-05/EX-4: *"An entry leaves the map when the send
that carries it is enqueued, not when it is accepted."* VA-1: *"confirm a
`Full` send clears nothing."* This asymmetry is the whole reason `Wire::send`
returns a `bool` at all (PHASE-05/EX-5).

**Observed.** No case anywhere produces a `Full` send from `Debounce::tick`.
All three debounce-bearing loop targets drain the capacity-1 channel on every
step and say so (`overlay.rs:236`, `numeric_guard.rs:241`, `debounce.rs:213`);
`tests/renderer` never fires the timer. `wire.rs:331` unit-tests that `send`
*reports* `false`, which is a different claim from *the caller acts on it*.

**Evidence — mutation-confirmed by the audit, not taken from the reviewer's
report.** `pending.rs:212-214` replaced with an unconditional
`self.held.borrow_mut().remove(&(option, field));`:

```
cargo test --workspace  →  exit 0, all 592 green
```

The reverted tree is `git status` clean and green at 592. So the branch that
keeps a person's typing when the channel is full is held by nothing. In
production it is reachable whenever a tick lands while `serve` is awaiting an
exchange with a command already queued — the channel is capacity 1
(`main.rs:86`) and `serve` does not drain while it is in `select!`. A second
tick 150 ms into any exchange is the ordinary case.

**The code is right and the coverage is absent.** This is not a defect in
`tick`; it is the one rule in the debounce that nothing would report breaking.

**Disposition:** `fix-now`
**Response:** Repaired with the driver the finding asks for: **`crates/goad/tests/event_loop_full/`**,
a new loop target. `event_loop_debounce`'s arrangement with the stepper's
every-step drain removed — that drain being exactly why no case anywhere produced
a `Full` send from `tick`.

One `text` field, so `held` counts the entry under test and nothing else. The
channel is occupied by a `Command::Evaluate` inside the same debounce window and
left occupied across the tick. Three readings: **A** the entry is held and
nothing delivered; **B** a tick has fired against a full channel and the entry
*still stands*; **C** the channel drained and the re-armed tick delivered the
same entry, read back through `answer`.

B is PHASE-05/VA-1. C is what stops B passing because no tick ever fired.

**Injection pass**, each applied to the tree, run, read, and restored from a
copy: **I1** `pending.rs:212-214`'s `if enqueued` removed so `tick` always
removes → **red at B**, `held: 0` against `1`. **I2** the re-arm neutered →
**red at C alone**, `held: 1, handled: 0` against `(0, 1)`, with B passing, so
the two readings discriminate independently. **I3** the occupancy drained again
immediately so the channel is empty at tick time → **red at B**, which is what
makes B's reading a fact about `Full` rather than about anything else. The
negative control compiles (`docs/memory/negative-control-must-compile.md`): I3 is
a drain, not a deleted line.

**Outcome:** `verified`

**Evidence.**

All three named injections re-run against `crates/goad/tests/event_loop_full/`,
each applied, run, read, and restored from a copy. Every one reproduces the
reported reading exactly.

| # | mutation, by `file:line` | reading I got |
|---|---|---|
| I1 | `pending.rs:263-265`, `if enqueued { … remove }` → `let _ = enqueued;` + unconditional remove | **red at B**, `full.rs:298`: `Reading { at: "B …", held: 0, handled: 0 }`, `left: (0, 0)` `right: (1, 0)` |
| I2 | `pending.rs:269`, `if !self.held.borrow().is_empty()` → `if false && …` | **red at C alone**, `full.rs:306`: `held: 1, handled: 0`, `left: (1, 0)` `right: (0, 1)` — B passed, so the readings discriminate independently |
| I3 | `full.rs:228`, `let _ = rx.try_recv();` added after the step-3 occupying send | **red at B**, `full.rs:299`: `held: 0, handled: 0` against `(1, 0)` |

I3 compiled and ran (`docs/memory/negative-control-must-compile.md`) — it is an
added statement, not a deleted line.

**The unnamed mutation I went looking for.** I enumerated what `tick` could do
wrong and each is caught: always-remove → I1, red at B; never-remove
(`if enqueued` → `if false`) → red at C, since `handled` reaches 1 while `held`
stays 1; a `Wire::send` that always reports `true` → same reading as I1. The
`Full` branch is genuinely driven by this target as it stands.

**What I found instead is a durability defect, not a coverage one**, and it is
raised as **`F-T1`**: B's reading is only non-vacuous because of an unstated
arithmetic relation between the test's step schedule and `DEBOUNCE`, a private
constant in `pending.rs`. Raising `DEBOUNCE` from 150 ms to 400 ms — a tuning
change, nothing more — leaves `event_loop_full` **green with the enqueue rule
deleted**. That does not make the repair wrong today; it is a separate finding
about how long the repair lasts.

`pending.rs` and `full.rs` restored; `git status` clean.

### F-S1 — AC-4's named element-half case cannot observe a present, so its `inits` comparison is vacuous

**Severity:** major
**Location:** `crates/goad/tests/renderer/fields.rs:1320-1362`,
`two_text_fields_typed_into_inside_one_window_both_reach_the_wire_and_neither_is_rebuilt`,
readings at `:1324` and `:1331`

**Expected.** `plan.md` §Coverage names PHASE-05/VT-3 for **AC-4**;
`design.md` §9's AC-4 row asserts *"`inits` is unchanged, so the element was
not destroyed while it was"*. The case's own doc comment claims *"the element
was not destroyed while it was being typed into, which is the whole of what
AC-4 is about."*

**Observed.** Between the two `get_inits()` readings there is no `.await`, so
`serve` — a `spawn_local` task on the same `LocalSet` — cannot be scheduled;
and nothing has been enqueued for it anyway, because `install.rs:70` routes a
`Reported::Typed` into `Debounce::hold` rather than sending a command, and
`tests/renderer` runs under `init_no_event_loop` (`harness.rs:54`) so the timer
never fires. **No present occurs between the two readings.** The equality is
guaranteed by the executor.

**Evidence — mutation-confirmed by the audit.** The whole of D8 removed: the
`if self.shown != showing` guard at `glass.rs:189` deleted so every present
calls `set_vec` and destroys every field element.

```
cargo test -p goad --test renderer two_text_fields_… → ok. 1 passed
```

**Scope, checked rather than assumed.** The criterion is *not* lost. Under the
same mutation, `cargo test --workspace --no-fail-fast` fails in four places,
one in each relevant target:

```
numeric_guard::a_present_inside_the_window_does_not_write_a_zero_back_over_a_cleared_field
overlay::a_present_shows_a_held_edit_and_corrects_one_the_host_never_recorded
reassert::a_second_present_corrects_nothing_and_a_widget_the_host_never_heard_from_is_corrected
fields::a_present_of_the_same_view_rewrites_the_values_and_destroys_no_element
```

So D8 is well held, including by AC-5's own instrument. What is wrong is
narrower and is a record defect: `plan.md` §Coverage and `design.md` §9 both
name a case for AC-4's element half that does not do that job, and the case's
doc comment states a claim it does not make.

**Disposition:** `fix-now`
**Response:** Repaired where the defect is — the case's own doc — and the record half is left
to Reconciliation rather than done twice.

The doc comment now states that the `inits` comparison **is** vacuous and why:
no `.await` between the two readings, so `serve` cannot be scheduled; nothing
enqueued for it in any case, because `install.rs:70` routes a `Reported::Typed`
into `Debounce::hold`; and `init_no_event_loop`, so no timer fires. No present
occurs between the readings and the equality is the executor's. The
mutation is named in the doc so the next reader can re-run it.

**The case is kept**: its draft-and-wire half is real and is its actual subject.
The four cases that *do* redden under the D8 mutation are named in the doc, so a
reader arriving at AC-4's element half is sent somewhere true. `plan.md`
§Coverage and `design.md` §9's AC-4 row are the remaining half and are already a
Reconciliation row.

**Outcome:** `verified`

**Evidence.**

The repair is the case's own doc comment at
`crates/goad/tests/renderer/fields.rs:1302-1323`. Two things had to be true of
it: that what it now says about the vacuity is *true*, and that the four cases
it redirects to *do* hold the element half.

**The vacuity claim is true, and re-measured.** D8 applied — `glass.rs:223`'s
`if self.shown != showing {` rewritten to `if true {`, so every present runs
`invoke_dismiss_pickers()`, `set_vec(rows)` and `set_options(…)` and destroys
every field element:

```
cargo test --workspace --no-fail-fast   →   exit 101
test fields::two_text_fields_typed_into_inside_one_window_both_reach_the_wire_and_neither_is_rebuilt ... ok
```

The case named for AC-4's element half passes with every element destroyed on
every present. The doc's own account of *why* — no `.await` between the two
`get_inits()` readings, `install.rs:70` routing a `Reported::Typed` into
`Debounce::hold` rather than sending, and `init_no_event_loop` so no timer
fires — matches the code as read.

**The four named cases all redden** under the same mutation, and so do two the
doc does not name:

```
busy::a_key_is_recorded_while_the_host_polls_and_dropped_while_the_answer_is_in_flight   (not named)
drain::a_tick_enqueued_during_an_exchange_survives_the_present_that_follows_it           (not named)
numeric_guard::a_present_inside_the_window_does_not_write_a_zero_back_over_a_cleared_field
overlay::a_present_shows_a_held_edit_and_corrects_one_the_host_never_recorded
reassert::a_second_present_corrects_nothing_and_a_widget_the_host_never_heard_from_is_corrected
fields::a_present_of_the_same_view_rewrites_the_values_and_destroys_no_element
```

Under-claiming, not over-claiming: the doc says the half is held "by four other
cases" and six hold it. Nothing to contest.

`glass.rs` restored from `$SCRATCH/orig/glass.rs`; `git status` clean.

**One residue, raised separately as `F-T4`**: the doc's locating citation
`glass.rs:189` does not point at the guard, which is at `:223`. The symbol it
quotes (`if self.shown != showing`) is unique, so a reader still finds it; the
line number does not. Two more citations in `event_loop_picker/picker.rs` carry
the same stale range.

### F-P1 — a `datetime` picked at a sub-minute zone offset submits a string denoting an instant up to 30 s from the one the host retains

**Severity:** minor
**Location:** `crates/goad/src/draft.rs:211-213`;
`crates/goad/src/instant.rs:122-140`; `crates/goad/src/glass.rs:520-531`

**Expected.** `instant.rs:56-57`, quoting §5.2 / §7 D19: *"the instant reaches
the backend carrying the offset it was resolved in."* §5.5 I-H names three
screen/wire divergences and this is not among them.

**Observed.** `compose` resolves through `TimeZone::system()`, and a tzdb
zone's pre-standardisation LMT offset carries **seconds** — `Australia/
Melbourne` is `+09:39:52` before 1895-02-01, which is the zone `instant.rs:345`
records this machine as being in. `Timestamp::display_with_offset` computes the
civil datetime at the **exact** offset and then prints the offset **rounded to
the nearest minute**.

**Evidence — the mechanism read from the locked source, independently of the
reviewer's measurement.** `jiff-0.2.35/src/fmt/temporal/printer.rs:310-318`:
`print_timestamp_with_offset_buf` calls `offset.to_datetime(*timestamp)` and
then `print_offset_rounded_buf`, whose own doc at `:772-775` reads *"If the
given offset has non-zero seconds, then they are rounded to the nearest
minute."* Measured deltas: `Europe/Amsterdam` −30 s, `Europe/Paris` +21 s,
`Australia/Melbourne` −8 s, composing 1880-06-15T12:00:00 in each.

**Why this is minor, and what it actually falsifies.** **RFC 3339 cannot
express a sub-minute offset at all** — its `time-numoffset` is
`("+" / "-") time-hour ":" time-minute`. So `R-57`'s own required format has no
room for one, and rounding is the only conforming behaviour available; `R-57`
is not breached. Nothing a person sees disagrees either: the button carries the
same rounded string (`glass.rs:523-527`) and `decompose` reopens the pickers on
the exact civil value, so the round trip through the host is lossless. What is
false is the fidelity claim in `instant.rs`'s doc and the completeness of
I-H's divergence list. PHASE-04/VT-1 cannot see it — it composes a 2024 date,
where every zone's offset is a whole number of minutes.

**Disposition:** `doc-wrong`
**Response:** Dispositioned `doc-wrong` and repaired there, because **there is no repair
available on the other side**. RFC 3339's `time-numoffset` is
`("+" / "-") time-hour ":" time-minute` and cannot express a sub-minute offset at
all, so rounding is the only conforming behaviour and `R-57` is not breached. The
finding's mechanism was re-read from the locked source and stands
(`printer.rs:310-318`, `:772-775`). `instant.rs`'s doc now states the rounding,
the measured deltas, why `R-57` survives it, that the host's own round trip is
lossless because the button carries the same rounded string and `decompose`
reopens on the exact civil value, and why PHASE-04/VT-1 cannot see it. I-H's
divergence list is `design.md`'s and is a Reconciliation row, not this.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**. The original raiser no longer exists, and this finding's repair is a document rather than code, so there is nothing for round 2's adversarial eye to attack; the user's decision on the Outcome split assigns it here. Verified against the tree rather than the Response: `instant.rs:54-80` now states the rounding in the sentence that used to claim the offset was carried intact, gives the mechanism and the three measured deltas, says why `R-57` survives — RFC 3339's `time-numoffset` has no seconds field, so rounding is the only conforming behaviour — says the host's own round trip is lossless, and says why PHASE-04/VT-1 cannot see it. The disposition is right for the reason given: there is no repair on the code side to prefer. I-H's divergence list was the finding's other half and is repaired under F-P2.

### F-P2 — `design.md` says in three places that a cleared numeric field submits `0`; it submits the number the field already held

**Severity:** minor — a **document** defect, and the document is internally
inconsistent rather than merely behind the code
**Location:** `crates/goad/src/view_model.rs:793-802` against
`docs/slices/009/design.md:356`, `:1323`, `:1345`

**Expected.** `design.md:1345` (§5.5 edges): *"numeric field cleared to `""` |
records the empty text, **and `0`**."* `:356` (§5.2): *"Empty text is zero,
which is what Slint's own `to-float` reads an empty field as."* `:1323` (I-H):
*"a cleared numeric field shows `""` and submits `0`."*

**Observed.** The host does not use Slint's `to-float`. `interpret`'s
`AdjustedText` arm applies `f64::from_str` to the raw text; `"".parse::<f64>()`
is an `Err`, so a cleared field falls through `.or_else(|| held_number(held))`
and keeps the number it had. `0` results only where the field was as-drawn
zero — no `min`, or `min: 0`.

**Evidence.** The slice's own unit asserts the implemented rule against a held
`7.25` with `""` in the input set (`view_model.rs:1248-1261`). On the wire: a
field declared `{"kind":"number","min":2.5}`, cleared and then answered,
submits `2.5` with an empty box on screen.

**Which side is wrong, and why this is not retro-fitting.** The code implements
§5.2's *stated rule* — *"the text is recorded verbatim, always; the number is
replaced only where the parse yields a finite `f64`"* — which is D-23 as
reversed at D-33. The three sentences are the residue of the **superseded**
`to-float` reading (D-16), left behind when that reversal was applied. Removing
them completes a revision the design already took; it does not fit the design
to the code. `:356`'s purpose has also gone: it justified the guard's
cleared-field exception, which PHASE-08/EX-7 measured out.

**What no document states correctly.** A cleared bounded `number` shows `""`
and submits its minimum. That is a fourth screen/wire divergence, it is not in
I-H's list, and `canon-delta.md` CD-1 does not reach it either — CD-1 is about
*untouched*, not *cleared*. A backend author cannot discover it anywhere.

**Disposition:** `doc-wrong`
**Response:** Repaired in the document, with the **explicit user endorsement** an edit to
`design.md` requires and did not have when this was written (`audit-log.md`,
2026-09-20, third entry). Four sites, and the fourth is the one that carries new
information rather than correcting old:

- **`:356`** now says empty text is *not* zero, gives the mechanism —
  `"".parse::<f64>()` is an `Err` like any other unaccepted text, so `interpret`
  falls through to `held_number` (`view_model.rs:793-802`) — and says `0` results
  only where the field was drawn at zero.
- **`:1345`**, the edges row, now states the held number and, with it, the thing
  the old row got right for the wrong reason: the clear survives in **both**
  windows by the same mechanism, because once recorded the draft's text is `""`
  and until it is recorded the overlay carries the pending `""` (D26). The strings
  agree either way, which is why no exception is needed.
- **§5.5 I-H** now lists the cleared field as submitting the number it held, and
  gains **F-P1's** sub-minute-offset divergence in the same list — the fourth and
  fifth entries on a list that had three.
- **§9 A-2 and §7 D13**, which both still stated the guard's exception as live.
  This is the third row of `notes.md`'s own reconciliation list and was endorsed
  2026-09-19: PHASE-08/EX-7 ran the exception four ways and removed it, not as
  dead weight but because carrying it suppresses the convergence AC-6 requires.
  A-2 asked whether the exception survived the overlay and the measurement
  answered; the row now says what was measured.

**Why this is a revision the design already took, and not a retro-fit.** The
three sentences are residue of D-16's `to-float` reading, which **D-33 reversed**
when the host stopped parsing through the control; the code implements §5.2's
own stated rule. `:356`'s purpose had gone independently of the reversal — it
justified the guard exception PHASE-08/EX-7 measured out, and that exception is
now gone from the markup at all five sites (`app.slint:531`, `:605`, `:682`, and
the two the `counted-*` repair touched), with `tests/event_loop_numeric_guard/`'s
second reading failing if anybody restores it (`app.slint:677-679`).

`canon-delta.md` CD-1 gains the matching clause, so a backend author reading an
empty box and a `2.5` on the wire can discover why — that is the decision of
`audit-log.md`'s first entry and is a Reconciliation row, not this.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**. The
original raiser no longer exists and this finding produced no code, so there is
nothing for round 2's adversarial eye to attack; the user's decision on the
Outcome split (`audit.md`) assigns it here. Verified by reading the four sites
against the tree rather than against the Response: `"".parse::<f64>()` is `Err`
and `interpret`'s fallback is `.or_else(|| held_number(held))`
(`view_model.rs:793-802`); the five guards carry no exception; and the wire claim
— a field declared `{"kind":"number","min":2.5}`, cleared, submits `2.5` — is the
finding's own, unchanged and still true.

### F-P3 — `Alternatives::first`'s locating citation points the wrong way, and its `reason` string carries mangled whitespace

**Severity:** nit
**Location:** `crates/goad-semantics/src/protocol/canonical.rs:382`, `:395-400`

**Observed.** The doc at `:382` says the guarantee is at *"`:362-364`, ten
lines above"* — `:382` is 20 lines **below** `:362`, so the direction is
inverted as well as the count; the `reason` at `:399` repeats *"ten lines above
this line"* from 37 lines below it. The `:361` / `:362-364` references
themselves are accurate. The `reason` string also carries two runs of collapsed
whitespace where a multi-line literal was joined without `\`; compare
`view_model.rs:536-541`, which uses the continued form.

**The substance holds** and was checked independently: `Alternatives` is a
tuple struct with a private field, `new` is its only constructor, there is no
`Default` or `Deserialize`, and no in-module site constructs an empty one.
`first()` cannot panic.

**Disposition:** `fix-now`
**Response:** Repaired. The `reason` literal's two runs of collapsed whitespace are a
continued (`\`) literal now, matching `view_model.rs:536-541`; *"ten lines
above"* became *"twenty lines above"* in the doc, and the `reason`'s own copy of
the phrase — which sat 37 lines *below* what it pointed at — now says only
*"above this line"*, since a count from inside an attribute goes stale the next
time anything above it moves. The `:361` and `:362-364` references were accurate
and are untouched. The substance was re-checked and stands: `Alternatives` is a
tuple struct with a private field, `new` is its only constructor, there is no
`Default` or `Deserialize`, and `first()` cannot panic.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**. The original raiser no longer exists, and this finding's repair is a document rather than code, so there is nothing for round 2's adversarial eye to attack; the user's decision on the Outcome split assigns it here. Verified by reading `canonical.rs:378-404`: the `reason` is a continued (`\`) literal with no collapsed runs, and its copy of the phrase now says only *"above this line"* rather than carrying a count from inside an attribute. The doc's locator reads *"twenty lines above"* and is correct — `Alternatives::new` is at `:361` and its empty-list refusal at `:362-364`, read from a doc sentence at `:383`. The substance was re-checked independently and stands: `new` is the only constructor, the field is private, there is no `Default` or `Deserialize`, and `first()` cannot panic.

### F-P4 — `group: ""` opens a new block; an unreadable `group` does not

**Severity:** nit
**Location:** `crates/goad/src/view_model.rs:245-266`, `:70-76`

**Observed.** `Run::Unreadable` and `Run::Ungrouped` both key `None` and merge,
which is stated and argued at `:261-266`. `Run::Named("")` keys `Some("")`, so
it does not merge with an adjacent ungrouped field and `blocks_from` opens a
second block whose `heading` is also `None`. `FieldBlock`'s doc — *"`heading:
None` is a block with no heading — an ungrouped run, or a `group` the backend
sent empty"* — reads as though the two cases are the same. Layout only; `R-18`
leaves it to the renderer and the behaviour is arguably right. Raised because
the type's doc does not distinguish them.

**Disposition:** `fix-now`
**Response:** Repaired in the doc; the behaviour stands, as the finding allows. `FieldBlock`'s
doc now says that **three** runs produce `heading: None` and that they do not all
merge — `Unreadable` and `Ungrouped` key `None` and join, `Named("")` keys
`Some("")` and opens a second headingless block beside them. `R-18` leaves
grouping to the renderer, so this is admitted rather than wrong; what was missing
was any statement that the three read identically once they arrive.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**. The original raiser no longer exists, and this finding's repair is a document rather than code, so there is nothing for round 2's adversarial eye to attack; the user's decision on the Outcome split assigns it here. Verified at `view_model.rs:68-82`: the doc now names all three runs that produce `heading: None`, says `Unreadable` and `Ungrouped` key `None` and join while `Named("")` keys `Some("")` and opens a second headingless block, and says `R-18` leaves grouping to the renderer so the behaviour is admitted rather than wrong. That is exactly what the finding asked for — it raised the type's doc, not the behaviour, and allowed the behaviour to stand.


### F-R1 — an open picker survives a view replacement and locks the person out of the whole form

**Severity:** major — **reasoned from the locked sources and not run.** The
four citations below are what make it checkable; settle it before dispositioning.
**Location:** `crates/goad/ui/app.slint:946-973`; `crates/goad/src/glass.rs:189-195`, `:225-231`

**Expected.** `design.md` §5.4 *Picking a datetime* and §5.5's edges table end a
pick in exactly one of `accepted`, `canceled`, or `compose` failing. §8 **R5**
prices a view replacement as *the field clears under the caret*. Nothing in
canon or the design contemplates a picker still on screen after the view it
belongs to is gone.

**Observed.** Nothing in `goad` ever closes an active popup. Both pickers are
root singletons declared **outside** the `if root.mode == WindowMode.prompt`
block, so neither the mode switch nor `set_vec` reaches them, and `hide()` does
not either.

**Evidence**, four facts from `slint 1.17.1` as vendored:

1. `fluent/datepicker.slint:23`, `fluent/time-picker.slint:24` — both bind
   `close-policy: PopupClosePolicy.no-auto-close`. A click outside does not
   close them.
2. `i-slint-core-1.17.1/window.rs:1979` — `close_all_popups` has exactly one
   caller, `window.rs:608` inside `set_component`, and `goad` calls
   `set_component` once, at construction. No host action closes an open picker.
3. `window.rs:843-871` — for a `ChildWindow` popup with the pointer outside its
   geometry, `item_tree` is set to `None` and the loop `break`s (`Menu` is the
   only continuing kind). The mouse event is then delivered **nowhere** — not to
   the popup, and not to the window beneath.
4. `window.rs:1158-1166` — Escape closes the top popup only for `CloseOnClick`
   and `CloseOnClickOutside`. Neither picker is either.

**The sequence.** A person opens a `datetime` picker; a scheduled firing
completes; `absorb` takes `Shift::Replaced` or `Shift::Closed`; `present`
rebuilds the rows or hides the window. **The picker is still up, every control
beneath it is unreachable by pointer and by keyboard, and the only exit is the
picker's own Cancel.** Completing the pick instead sends `Command::Edit` under a
now-stale `root.picking-view` and is refused `SupersededView`.

**To settle it**, one loop-tier case: open the picker, present a new `view_id`,
assert the popup is still visible or that a click on an option button raises
nothing.

**Settled by measurement (audit session 2) — confirmed, and wider than the
finding states.** A new loop-tier target,
`crates/goad/tests/event_loop_picker/{main.rs,picker.rs}`, drives it with
controls on both sides of every claim: the same click and the same keystroke,
made once with no picker up and once with a picker up, so a silent no-op cannot
be read as a driver that missed. All four slint citations were independently
re-verified in the vendored sources. Renderer tier was rejected for a stated
mechanical reason — it runs `init_no_event_loop` (`tests/renderer/harness.rs:54`)
so nothing is laid out and `absolute_position` has nothing behind it, which is
why the existing picker cases drive `invoke_accessible_default_action`, the one
API that cannot answer the question.

| | measured |
|---|---|
| survives a view replacement | **yes.** The rows rebuilt (`morning` → `evening`) and all 34 of the date picker's elements — `Next month`, `Previous month`, 30 day cells, `Cancel`, `OK` — were still in the tree |
| survives `hide()` | **yes, and this is the worse half.** On `Shift::Closed` the *form's* buttons left the tree and the picker's 34 stayed. A picker over nothing |
| the form beneath, by pointer | **unreachable.** The same click that raised `chosen v1/morning` with no picker up raised **nothing** with one up; `invoke_accessible_default_action` on that same option raised `chosen v2/evening`, so **the widget is alive and only input routing is blocked** |
| the form beneath, by keyboard | **unreachable.** `x` into the `LineEdit` raised `edited … text="x"` as a control; `y` with the picker up raised nothing |
| the exit | `Escape` does **not** dismiss (citation 4 confirmed). A real pointer click on the picker's own `Cancel` **does** close it, even after the view was replaced — so it is not a permanent lockout, which is what caps this below `blocker`. Completing the pick raised `edited v1/morning/when` — the **replaced** view's token, which `Controller::edit` refuses `SupersededView`. F-R1's prediction, exact |
| `busy` | irrelevant — `false` throughout. **F-R2's row 6 confirmed as written** |

**The case is RED by design**: it asserts the contract (`design.md` §5.4, §5.5,
§8 **R5**), so it lands *with* the repair and not before. Every other `-p goad`
target stays green beside it.

**The injection pass also settles the repair's shape.**

| # | injection | expected | read |
|---|---|---|---|
| baseline | none | red | red, on *the picker must not still be on screen* |
| A | a `dismiss-pickers()` in the markup, invoked from `present`'s `self.shown != showing` branch | green | **green** |
| B1 | A narrowed to `showing.is_some()` — replacement only | red on the hide claim alone | **red, hide claim only** |
| B2 | A narrowed to `showing.is_none()` — hide only | red on the replacement claim | **red, replacement claim** |
| B3 | A intact, the driver's press/release removed | red on the first control | **red, first control** |

Injection A says **one call site suffices**: the hide present is itself a change
of `shown`, so the `self.shown != showing` branch covers `Shift::Replaced` and
`Shift::Closed` together. B1's first run exposed a proxy defect in the case's
own draft — with the replacement repaired, the hide step was handed a window
with nothing open and read green on a claim it never exercised; the committed
case reopens the picker before the hide and its comment records why.

**Not settled, and stated rather than glossed.** Whether the blocked click was
outside the popup's geometry or absorbed by its interior — popup element
positions are popup-local (`item_tree.rs:626-629`) — though both branches of
`window.rs:843-871` end the same way, so the verdict is unaffected. What a real
compositor paints after `hide()`, which needs a `just demo` reproduction.
Which element held focus while the picker was up. And the `SupersededView`
refusal end to end: the stale token was measured, the refusal reaching the
diagnostics surface was not, because asserting it would contradict the contract
the case holds.

**Severity: stays `major`**, moved from *reasoned* to *measured*, with the scope
widened — it is not the whole *form* but the whole *window*, and it outlives the
window being hidden.

**Disposition:** `fix-now`
**Response:** Repaired at injection A's shape, unchanged. `ui/app.slint` gains
`public function dismiss-pickers()` — `date-picker.close(); time-picker.close();`
— and `glass.rs` calls `invoke_dismiss_pickers()` as the first statement of
`present`'s `self.shown != showing` branch. One call site, and the reasoning was
re-checked against the code rather than carried over: `showing` is
`frame.shown.map(…)`, so `Shift::Closed` makes it `None` against a `self.shown`
that still holds the answered view, and the hide present is itself a change of
`shown`. A `public function` rather than a callback because the direction is
host→markup, which nothing else in this file does; no `is-open` guard, because a
generated `close()` is `Option<NonZeroU32>::take().map(…)`
(`i-slint-compiler-1.17.1/generator/rust.rs:3792-3804`) and closing a picker
that is not open is a no-op.

The loop-tier case landed as written, cherry-picked from `feceab6` with no
assertion weakened: **red** at `picker.rs:398` before the repair with both
controls holding, **green** after. The injection pass was re-run against the
committed tree — B1 (`showing.is_some()`) red on the hide claim alone at `:428`,
B2 (`showing.is_none()`) red on the replacement claim at `:398`, B3 (press and
release removed) red on the first control at `:372`. `just check` exits 0 at
**596**. Readings and the injection table are in `notes.md` §*Audit session 2 —
repairs*.

**Not repaired, and recorded rather than folded in:** a third path leaves a
picker up. `Focus::Diagnostics` over a shown view derives `Surface::Diagnostics`
with `frame.shown` still `Some` (`controller.rs:162-167`), so `present` writes
`WindowMode::Diagnostic` — the prompt block leaves the tree — without changing
`shown`, and the dismiss branch is not taken. That is outside this finding's two
measured shifts and wants a finding of its own.

**Outcome:** `verified` — set by round 2's behaviour dimension as raiser.

**Evidence.** All four citations re-derived from the vendored `1.17.1` sources.
The generated `close()` is `popup_id.take().map(…)`, so closing a picker that is
not open is a no-op and no `is-open` guard is needed, as the Response argues.

*The full injection table, re-run* against `crates/goad/src/glass.rs`, restored
from a scratch copy between each:

| injection | applied at | read |
|---|---|---|
| the `invoke_dismiss_pickers()` line deleted | `glass.rs:235` | **red at `picker.rs:398`**, `picker: true` at reading D |
| `if showing.is_some() { … }` — replacement only | `glass.rs:235` | **red at `picker.rs:428`**, the hide claim alone, `picker: true` at reading H |
| `if showing.is_none() { … }` — hide only | `glass.rs:235` | **red at `picker.rs:398`**, the replacement claim alone |

**Every line number the Response quotes is the line the run produced**, and the
case is not a proxy: four controls, failing independently under the injections.

**The class, enumerated.** *A popup outlives the surface it belongs to.* Both
pickers across `Shift::Replaced` and across `Shift::Closed` — reached, measured.
Both across a `Focus::Diagnostics` mode switch — **not reached: F-B2**, measured
with a positive control. The `ComboBox` dropdown across a view replacement —
not reached; F-B2's scope note.

### F-R2 — the `busy` class, enumerated: two of the seven sites are worse than F-A1 priced, and one is not gated at all

**Severity:** major — this is **F-A1's class completed**, not a separate defect
**Location:** `crates/goad/ui/app.slint:423`, `:476`, `:547`, `:610`, `:683`, `:743`, `:784`

| # | site | line | cost when `busy` goes true mid-interaction |
|---|---|---|---|
| 1 | `CheckBox` | `:423` | focus ring; the click is discarded |
| 2 | text `LineEdit` | `:476` | **dropped keystrokes** (F-A1) |
| 3 | numeric `LineEdit` | `:610` | **dropped keystrokes** (F-A1) |
| 4 | `Slider` | `:547` | **pointer grab lost** (VH-1) |
| 5 | `ComboBox` with its popup open | `:683` | **the selection is silently swallowed and the dropdown closes.** `common/combobox-base.slint:20-23`: `select()` opens `if !root.enabled { return; }`, so `current-index` is unchanged and `selected` is never raised — but the popup item's `TouchArea` (`fluent/combobox.slint:138-143`) carries **no** `enabled` gate and still runs `popup.close()`. The click lands, does nothing, and closes the list. **Strictly worse than the `CheckBox`:** the person watches the dropdown respond and has no signal that nothing was chosen |
| 6 | `datetime` `Button` with a picker open | `:743` | **`busy` does not reach it at all.** The `Button`'s `enabled` gates only *opening*; the popups carry no `enabled` and no conditional (`:946`, `:960`), so an open picker stays fully operative through the whole round trip while every other control is inert. Nothing is lost at the widget — the loss is downstream, as F-R1 describes |
| 7 | option `Button` | `:784` | the answering click is discarded *(the reviewer's row was truncated in transit — recover it)* |

**Row 7, recovered by the audit (session 2).** The reviewer's truncated cell,
re-derived from the vendored sources rather than reconstructed from the
summary:

> **the answering press is cancelled mid-press and no `clicked` is raised.**
> `fluent/button.slint:12` binds `enabled <=> i-touch-area.enabled`. When a
> `TouchArea` holding the grab is disabled,
> `i-slint-core-1.17.1/items/input_items.rs:81-93` clears `has_hover`,
> releases the grab, sets `pressed` false, delivers `PointerEventKind::Cancel`
> and returns `ForwardAndIgnore` — so the release raises nothing. Same
> mechanism as the `Slider`'s lost grab (VH-1); `slider-base.slint` binds its
> `TouchArea` the same way.

**This site is different in kind from the other six and the difference is the
whole of its disposition.** Here the disable is *deliberate*: it is slice 003's
double-submit guard (`03138da`), and during the exchange it guards — the
person's own answer — it is correct. The defect is only that it also fires for
an exchange the person did not start. `Command::Choose` has exactly one origin
(`install.rs:40`, the option `Button`'s `clicked`) and is the only road to
`Pending::Respond` (`controller.rs:746-760`), so after the narrowing this site
disables **exactly** when its own guard wants it to and at no other time. It
needs no second flag.

**Why this matters to the repair the user chose.** Narrowing `busy` fixes sites
1–4 and 7. It does **not** fix 5 or 6 on its own: 5 is a missing `enabled` gate
on a popup item inside `std-widgets`, and 6 is a binding this project never
wrote. Both have to be answered explicitly or the class is fixed in name only.

**Disposition:** `fix-now`
**Response:** Applied with F-A1 — they are one repair and
one line. Sites 1-4 and 7 are answered by the narrowing: 1-4 are no longer
disabled by an exchange the person did not start, and 7 disables exactly for
the answer it guards, which is what it was written for. Its `enabled:
!root.busy` is untouched, as this finding argues it should be.

Sites 5 and 6 are **not** answered here and are not claimed to be:
`audit-log.md` records 5 as unreachable after the narrowing (a click cannot
reach a button beneath an open dropdown) and 6 as F-R1's, which lands with
F-R1's repair.

Held by a new case in each tier: `renderer/table.rs`'s
`an_evaluation_does_not_engage_and_an_answer_does` for the flag, and
`tests/event_loop_busy/` for what it costs a person — the latter reddens both
when `engage` is reverted **and** when the text `LineEdit`'s `enabled` binding
is deleted, so it holds the markup as well as the controller.

**Outcome:** `verified` — set by round 2's behaviour dimension as raiser.

**Evidence.** Both named instruments redden under the `engage` reversion
recorded at F-A1's Outcome above.

*The Response's second claim, run rather than taken.* Deleting the text
`LineEdit`'s `enabled: !root.busy` at `app.slint:517` — `event_loop_busy`
**FAILED** at `busy.rs:321`, `left [a, b, c]` / `right [a, b]`, every other
target green. **This is the first case in the project that can see an `enabled`
binding at all**; F-A1's own evidence records that before this slice, deleting
any of them left all 592 green.

*Sites 5 and 6, checked at the markup and the routing rather than accepted.*
Site 5 (`ComboBox`): `fluent/combobox.slint:112` binds
`close-policy: close-on-click-outside`, and `i-slint-core-1.17.1/window.rs:824`
is the arm that closes on an outside press and `:843-871` then delivers it
nowhere — so the click that would reach the option `Button` beneath the dropdown
closes the dropdown instead. `audit-log.md`'s *unreachable after the narrowing*
holds. Site 6 (the `datetime` `Button`): answered by F-R1's repair for the two
shifts it measured, and **not** for a third — **F-B2**.

### F-S3 — AC-7's compile-error mechanism is held by nothing in the gate

**Severity:** major
**Location:** `crates/goad/src/view_model.rs:317` (`drawn_form`); the case at `:889`

**Expected.** AC-7: *"the mechanism that makes a sixth kind a compile error
survives — `R-55` is discharged for five kinds, not deleted."* `plan.md`
PHASE-09/VT-7: *"`drawn_form` still matches `FieldKind` exhaustively."*

**Observed.** The case maps `drawn_form(field.kind()).is_ok()` over five fields
and asserts `[true; 5]`. Its own doc concedes *"No test can assert a match's
exhaustiveness."* VA-2 is an agent read, not a check.
`crates/goad-boundary/tests/checks/` contains no instrument naming
`drawn_form`, `FieldKind` or a wildcard arm, and `clippy::wildcard_enum_match_arm`
is not in the workspace lint set — `Cargo.toml:185` denies only
`wildcard_imports`.

**Evidence.** Add `_ => Ok(DrawnKind::Boolean),` as a final arm of
`drawn_form`. It compiles, lints clean, VT-7 stays green and `just check` stays
green — and a sixth protocol kind then renders **silently as a checkbox**
instead of forcing the draw-or-report fork. `FieldKind`
(`canonical.rs:246`) is not `#[non_exhaustive]`, so the property is real today;
nothing in the gate keeps it real. A compile-fail case (`trybuild`) or a
boundary scan for a wildcard arm in `drawn_form` would close it.

**Evidence, re-derived by the audit (session 2) — the stated mutation is wrong,
and the hole is narrower than the finding claims.** Three shapes were run
against `cargo clippy -p goad --all-targets -- -D warnings`:

| mutation to `drawn_form` | gate |
|---|---|
| add `_ => Ok(DrawnKind::Boolean),` beneath all five arms — **F-S3's own mutation** | **red.** `unreachable_patterns` (denied via `-D unused`) *and* `clippy::match_same_arms` (pedantic) |
| delete the `Boolean` arm, add `_ => Ok(DrawnKind::Boolean),` — a wildcard covering **one** variant | **red.** `clippy::match_wildcard_for_single_variants` (pedantic) |
| delete the `Boolean` and `Text` arms, add `_ => Ok(DrawnKind::Boolean),` — a wildcard covering **two** | **green.** Clippy clean |

So the lint set already holds the property in two of its three shapes, and
F-S3's *"it compiles, lints clean"* is false as written. Enumerating what a
sixth `FieldKind` variant could actually do:

- wildcard covering the new variant **alone** → `match_wildcard_for_single_variants`, gate red;
- wildcard covering the new variant **and an existing one, with a body that
  disagrees with that existing kind** → the suite reddens (`fields.rs:2120`
  asserts all five kinds draw in declared order);
- wildcard covering the new variant **and an existing one, with a body that
  agrees with that existing kind** — e.g. absorbing `Boolean` into
  `_ => Ok(DrawnKind::Boolean)` → **lints clean, suite green, sixth kind draws
  silently as a checkbox.**

**That last row is the whole hole, and it is the only one.** The finding's
substance survives and its recommended closers (a `trybuild` compile-fail case,
or a boundary scan for a wildcard arm in `drawn_form`) are still the right
ones — but *"held by nothing in the gate"* overstates it. The severity was set
at raise time and is not renegotiated here; the disposition should be taken
against this table rather than against the headline.

**Note for disposition.** AC-7 says the mechanism must *survive*. It does. What
F-S3 establishes is that nothing would report its removal — which is the same
shape as F-S2, one level up.

**Disposition:** `fix-now`
**Response:** Repaired with the instrument the finding's own re-derivation points at, which is
neither of the two closers it names.

**`trybuild` cannot express this property.** It proves that code *fails* to
compile; the hole is a wildcard arm that compiles, lints clean and leaves the
suite green. A fixture could not add a sixth `FieldKind` either — the enum is
`goad-semantics`'. The dependency was authorised and then not taken, because it
would have bought nothing.

**`clippy::wildcard_enum_match_arm`, denied for the `goad` crate**
(`lib.rs`). It is a clippy *restriction* lint, off by default, and F-S3 itself
notes it is absent from the workspace lint set. It holds exactly the surviving
shape, and the three shapes now stand as: a wildcard over the new variant alone
→ `match_wildcard_for_single_variants`; one whose body disagrees with an
absorbed kind → `fields.rs:2120`; one whose body agrees → **this lint**.

**Verified, not assumed.** With the finding's surviving mutation applied —
`Boolean` and `Text` arms deleted, `_ => Ok(DrawnKind::Boolean)` added —
`cargo clippy -p goad --all-targets -- -D warnings` fails. Baseline clean, and
`just check` exits 0 at 597.

**No exceptions, and no canon.** `goad`'s production code has no wildcard over
an enum; the two in its unit tests, destructuring the `a_choice()` fixture, are
`let … else` now. Crate-scoped rather than workspace-wide — **and the reason is
not cost**: the other four such matches (`ProtocolError::source`, and
`goad-shell/src/ingress/envelope.rs:106`, `:118`, `mod.rs:752`) each choose *no
behaviour from the variant*, so the wildcard's body is the right answer for a
variant that does not exist yet and denying it there buys four `#[expect]`s and
no property.

Because this is a lint-table entry inside the gate's existing clippy pass rather
than a new boundary instrument, **POL-001 is untouched** and the gate's
instrument count is unchanged. That was the alternative's price and it is not
paid.

**Outcome:** `verified`

**Evidence.**

The repair is `#![deny(clippy::wildcard_enum_match_arm)]` at
`crates/goad/src/lib.rs:35`. Four shapes run against
`cargo clippy -p goad --all-targets -- -D warnings`, each applied to
`view_model.rs`'s `drawn_form` (`:325-347`) and restored from a copy. Baseline
clippy clean.

| shape | reading |
|---|---|
| `_ => Ok(DrawnKind::Boolean),` beneath all five arms — F-S3's own mutation | **red**: `unreachable pattern` *and* `these match arms have identical bodies` |
| `Boolean` arm deleted, `_ => Ok(DrawnKind::Boolean),` added — wildcard over **one** variant | **red**: `wildcard matches only a single variant and will also match any future added variants` |
| `Boolean` and `Text` arms deleted, `_ => Ok(DrawnKind::Boolean),` added — the shape that used to survive | **red**: `wildcard match will also match any future added variants`, cited to `lib.rs:35` |
| **the fourth shape I went looking for** — same, but a *named binding* catch-all `other => { let _ = other; Ok(DrawnKind::Boolean) }` rather than `_` | **red**, same lint. The lint is not spelling-sensitive |

So the three shapes the Response enumerates are each held by the instrument it
names, and the obvious evasion of the new one is held too.

**The remaining shapes, enumerated rather than asserted.** A sixth `FieldKind`
can reach `drawn_form` silently only through a catch-all. An *explicit*
`FieldKind::NewKind => …` arm is not a hole — it is AC-7's fork being taken
deliberately, which is what AC-7 asks for. `#[non_exhaustive]` on `FieldKind`
would force a catch-all and so redden the gate, which is the right outcome. The
only remaining evasion is rewriting `drawn_form` as an `if let` chain with a
fallback `else`, which no lint reaches and which is a visible rewrite rather
than an omission.

**The deliberate exceptions check out as a decision, and not as an
enumeration.** Denying the lint at `ProtocolError::source`,
`goad-shell/src/ingress/envelope.rs:106`, `:118` and `mod.rs:752` would buy
`#[expect]`s and no property — I read each and the conclusion holds. But the
comment's *count* of them is wrong and its characterisation of one of them is
wrong, which is raised separately as **`F-T3`**. That does not move this
Outcome: F-S3's own subject is AC-7's mechanism, and the mechanism is held.

**One thing the Response states wrongly about its own instrument's reach**, and
it is raised as **`F-T2`**: `#![deny(…)]` in `lib.rs` governs the *library*
compilation unit. `crates/goad/src/main.rs` is a second crate root and is not
covered. That does not touch AC-7 — `drawn_form` is in the library — so the
finding's own subject is repaired, and `F-T2` is about the boundary statement,
not about AC-7.

`view_model.rs` restored; `git status` clean.

### F-S4 — PHASE-05 is the one phase of this slice with no injection table, and it is AC-4's phase

**Severity:** minor
**Location:** `notes.md` — injection tables at `:719`, `:1032`, `:1388`,
`:1869`, `:2726`, `:3211`, `:3680`, `:4525`. **None between `:1992` and `:2444`.**

**Expected.** `design.md` §9: *"Every new case gets an injection pass … the one
phase of slice 005 without it produced all three weak cases."* `plan.md`
PHASE-05/T-8 is ticked *"VT-1, VT-3 in `fields.rs`; VT-2, VT-4 in `wiring.rs`,
each with an injection pass."*

**Observed.** The sheet records that the phase agent overran and left the tree
not compiling, so *"none of its four cases had ever been run"* (`notes.md:2410`).
One injection is recorded afterwards, for the orchestrator's repair of
`wiring.rs` VT-2 alone (`:2427`). `two_text_fields_typed_into_…` is named
nowhere in `notes.md`, and no per-case red/green reading exists for PHASE-05's
`fields.rs` rows. PHASE-05's `event_loop_debounce` negative control **is**
recorded as compiled and run.

**Evidence.** **F-S1 is the defect this missing pass would have found.** An
injection aimed at PHASE-05/VT-3's stated claim cannot redden it — which is
exactly what an injection pass reports and a green run does not. `design.md`
§9's own sentence predicted this outcome for the phase that skips it, and it
came true in the same slice that wrote it down.

**Disposition:** `fix-now`
**Response:** Repaired by running the pass, now, against the tree as it stands — rather than
reconstructing what PHASE-05 would have found. The table is in `notes.md`
§*Audit session 3*; each mutation was applied to production code, the target
run, the message read, and the file restored from a copy.

| # | case | mutation | reading |
|---|---|---|---|
| P1 | VT-1 | `drawn_form`'s `Text` arm to `DrawnKind::Boolean` | **red** — *morning/also declares no accessible-value* |
| P2 | VT-2 | `choose`'s edit loop to `.take(0)` | **red** at `wiring.rs:1500` |
| P3 | VT-3 | `carried()` to `.take(1)` | **red** at `fields.rs:1380`, `noted: ""` against `"walked before breakfast"` |
| P4 | VT-4 | the `SupersededView` arm to `superseded = false` | **red** — `0` reports against `1` |

All four reach red, so none of PHASE-05's cases is wholly vacuous. **P3 is the
reading that matters**: it reddens VT-3's *wire* half — the half F-S1 found to
be real — and nothing here reddens VT-3's `inits` half, because that half
cannot be reddened. So the finding's own claim is confirmed from the other
side: F-S1 is exactly what this pass would have caught, and it took a separate
finding to catch it because the pass was skipped.

P4 needed reshaping to keep the negative control compiling
(`docs/memory/negative-control-must-compile.md`): `=> {}` leaves `superseded`
never written and trips `unused_mut` under `-D warnings`, so it assigns `false`
instead. An uncompiled control greps the same as a passing one.

`plan.md` PHASE-05/T-8's tick is now true of the record rather than of an
intention.

**Outcome:** `verified`

**Evidence.**

All four mutations re-run against the tree as it stands, each applied to
production code, the target run, the message read, and the file restored from a
copy. **Every one compiled** — each produced a test run rather than a compile
error, which is the check
`docs/memory/negative-control-must-compile.md` asks for.

| # | mutation, by `file:line` | reading I got |
|---|---|---|
| P1 | `view_model.rs:328`, `FieldKind::Text => Ok(DrawnKind::Text)` → `Ok(DrawnKind::Boolean)` | **red**, 6 cases; `fields.rs:500`: *"morning/also declares no accessible-value"* — the message the table names |
| P2 | `controller.rs:317`, `for edit in edits` → `for edit in edits.iter().take(0)` | **red**, 6 cases; first at `wiring.rs:1500` — the line the table names |
| P3 | `pending.rs:172`, `.take(1)` inserted into `carried()`'s iterator | **red**, 2 cases; `fields.rs:1380`, `"noted": String("")` against `"walked before breakfast"` — the reading the table names |
| P4 | `controller.rs:321`, `Err(Refused::SupersededView) => superseded = true` → `= false` | **red**, 1 case; `left: 0` `right: 1` — and it compiled, which is why the table's reshaping from `=> {}` was needed |

The table in `notes.md` §*Audit session 3* is accurate in every column I can
check, and P3's claim — that the pass reddens VT-3's *wire* half and nothing
reddens its `inits` half — is consistent with what I measured independently
under F-S1's D8 run.

`view_model.rs`, `controller.rs`, `pending.rs` restored; `git status` clean.

### F-S5 — `reasserts` is a self-report: decoupling the count from the write survives every case

**Severity:** minor
**Location:** `crates/goad/ui/app.slint:436-440`, `:489-493`, `:564-568`, `:642-646`, `:699-703`

**Expected.** AC-5, measured in `reassert.rs:293` and `overlay.rs:337` as
`reasserts == 0`.

**Observed.** `root.reasserts += 1` sits *inside* the
`if (self.x != root.values[…].x)` block, beside the write. Every recorded
injection mutates the two together, so each is discriminating — but the
instrument measures what the markup **says** it did, not what it did.

**Confirmed by the audit (session 2), not taken on report.** The `CheckBox`
guard at `app.slint:436-440` rewritten so the assignment runs unconditionally
and only the increment stays inside the comparison:

```
cargo test --workspace --no-fail-fast  →  exit 0, all 592 green
```

The widget is now written on every present and `reasserts` still reads `0`.
The tree was restored from a copy and `git status` checked clean afterwards.

**Evidence.** In any one guard, hoist the assignment out of the conditional and
leave the increment inside. Every widget is then written on every present — the
caret destroyed on every tray check, which is AC-5's second clause — while
`reasserts` stays `0`, so `reassert.rs`, `overlay.rs` and `numeric_guard.rs` all
stay green. No tier observes the caret (D-10 assigns it to AC-10's human half),
so nothing else catches it. **Exposure grows with each control a future slice
adds.**

**Disposition:** `fix-now`
**Response:** Repaired by making the counter a **consequence** of the write rather than a
sibling of it. All five guards now assign through a counting function —
`root.counted-bool(…)`, `-string`, `-float`, `-int`, declared beside `reasserts`
— so the count happens because the assignment evaluated its right-hand side. A
write that should not have happened is now a count that should not have
happened.

The approved shape was an observable-state case, and it is **not reachable**:
the observable F-S5's mutation changes is the caret, and no tier can see one
(D-10 assigns it to AC-10's human half; `reassert.rs` already reads the
`ComboBox`'s own `chosen`, and writing the same value back does not move it).
So the instrument was fixed instead of a case being added around it.

**Measured as a pair**, applying the finding's own mutation — the `CheckBox`
guard's assignment hoisted out of its comparison — to each shape in turn:

| shape | `event_loop_reassert`, `_overlay`, `_numeric_guard` |
|---|---|
| `root.reasserts += 1` beside the write (as shipped) | **all three green** — the defect is invisible, confirming the finding |
| the counting call (as repaired) | **red at `reassert.rs:298`**, `reasserts: 2` against `0` |

One function per slot type because slint has no generics here. The exposure
grew with every control a future slice added, which is why this is the class
rather than the instance.

**Outcome:** `contested`

**Evidence.**

The Response's own measurement reproduces exactly. `app.slint:479-481`, the
`CheckBox` guard's assignment hoisted out of its comparison with the counting
call kept:

```
reassert.rs:298 — FAILED
  Reading { inits: 3, reasserts: 0, epoch: 1, … } then Reading { inits: 3, reasserts: 2, epoch: 2, … }
  left: 2   right: 0
```

`event_loop_overlay` and `event_loop_numeric_guard` stay green, so `reassert.rs`
is the one that discriminates — as reported.

**But the property the finding was about is not held.** F-S5's claim is:
*"`reasserts` is a self-report: decoupling the count from the write survives
every case."* The repair couples the count to a write **that goes through the
counting function**. Nothing requires a write to go through it. Dropping the
call is one line, and it is the shape a future author who does not know why
`counted-bool` exists would write by default:

```slint
// crates/goad/ui/app.slint:479-481
changed tick => {
  self.checked = root.values[field.slot].checked;   // unconditional, uncounted
}
```

Measured, on the repaired tree:

```
cargo test --workspace --no-fail-fast   →   SUITE GREEN
just check                              →   exit 0, 29 `test result: ok` lines summing to 597
```

The `CheckBox` is now written on **every** present — the caret destroyed on
every tray check, AC-5's second clause — and `reasserts` reads `0`. That is
F-S5's original sentence, still true of the repaired tree. What changed is the
spelling of the mutation, not whether the instrument can miss it.

`reasserts` still reports the markup's account of itself. It reports *how many
counted writes happened*, which is the markup's own statement, rather than *how
many writes happened*.

**What must be re-dispositioned, and why.** Two claims in the Response go beyond
what landed and should be withdrawn or instrumented:

1. *"a write that should not have happened is now a count that should not have
   happened"* — true only of a write routed through `counted-*`.
2. *"The exposure grew with every control a future slice added, which is why
   this is the class rather than the instance"* — the class is *completeness*:
   every write to a guarded widget goes through a counter. Five call sites hold
   that by convention, and nothing checks the convention. A sixth control, or an
   edit to any of the five, restores the original exposure in full.

**The instrument that would hold it, named because the protocol asks the raiser
not to contest without one.** `crates/goad-boundary/tests/checks/` already runs
a line-based scan over every member's **sources and markup** — the
domain-vocabulary scan, whose reach `POL-001` §Verification states in those
words. A sibling scan over `ui/app.slint` asserting that every assignment of the
form `self.<prop> = …` inside a `changed tick` handler has `root.counted-` on
its right-hand side is the same mechanism, the same file tree, and would have
reddened under the mutation above. That is a claim about what would hold the
property, and I ran the mutation it is derived from rather than reasoning to it
(`docs/memory/verify-the-proposed-instrument.md` — it is stated as a candidate,
not as a verified closer; whoever takes it should injection-pass it).

**What I also checked, and found sound**, so the re-disposition is not wider
than it needs to be:

- **The five guards are consistent with their own counters.** Every call pairs
  the right slot type with the right function: `counted-bool`↔`.checked`
  (`:480`), `counted-string`↔`.text` (`:532`, `:683`), `counted-float`↔`.number`
  (`:606`), `counted-int`↔`.index` (`:739`). No guard reads one slot and writes
  another. No `counted-*` call sits anywhere its result is discarded.
- **The set of five is complete for the controls that need one.** Six
  interactive controls are declared in the field repeater
  (`app.slint:455`, `:511`, `:568`, `:631`, `:709`, `:770`); five self-assign
  and carry a guard. The sixth, the `datetime` `Button` at `:770`, never
  assigns its own `text`, so its declarative binding is never broken and a
  present corrects it without a guard. I read the whole picker path
  (`:787-798`, and `dismiss-pickers`) to confirm nothing writes that button's
  `text` imperatively. The comment at `:774-778` states this correctly.

`app.slint` restored from `$SCRATCH/orig/app.slint`; `git status` clean.

### F-S6 — `event_loop_debounce`'s "one edit per tick" cannot fail for its own claim

**Severity:** nit
**Location:** `crates/goad/tests/event_loop_debounce/debounce.rs:270-275`

**Observed.** Asserts `after_one.handled == 1`. The case's own comment states
the limitation and `notes.md:2361` records it as a deliberate non-control.

**Evidence.** Make `tick` iterate the whole map and `try_send` each entry: the
second `try_send` returns `Full`, the entry stands by the enqueue rule, the
re-arm offers it next tick — `handled` is still `1` then `2` and both readings
pass. Raised because it **interacts with F-S2**: one case that fills the
channel gives both claims a driver.

**Disposition:** `fix-now`
**Response:** **The finding is right that the assertion cannot fail for its own claim, and its
suggested repair does not work.** Checked before acting on it: at capacity 1 —
`main.rs:87`, and the harness the same — a `tick` that iterated the whole map is
*indistinguishable* from one-per-tick. Give the mutant two entries and an empty
channel and it sends the first, which succeeds and is removed, then the second,
which comes back `Full` and stands by the enqueue rule: one command queued and
one entry left, identical to production. `tick` is synchronous, so no drain can
interleave between the two sends. Filling the channel first is worse — then
neither shape delivers anything. So "one case that fills the channel gives both
claims a driver" gives F-S2 a driver and not this one.

Repaired by making the case say what is true of it. The assertion now reads *at
most one command per tick reaches the controller* — a fact about the channel's
capacity as much as about `tick` — and the doc states in full why the stronger
claim is unobservable rather than untested, which is the same shape as F-S7. The
rule this case cannot reach is held by a real driver in
`tests/event_loop_full/`, and the doc points there.

**Disposition unchanged** (`fix-now`, repaired in this slice); what changed is
the repair, from *add a driver* to *stop claiming what cannot be driven*.

**Outcome:** `verified`

**Evidence.**

The Response makes one load-bearing factual claim — that at capacity 1 a `tick`
which iterated the whole map is *indistinguishable* from one-per-tick — and
reports it as "checked before acting on it" without a run. I ran it.

`pending.rs:249-267`, `tick`'s single-entry `next` replaced with a `Vec` of the
whole map and the body turned into a `for` loop, `if enqueued { remove }` kept:

```
cargo test --workspace --no-fail-fast   →   exit 0, no failures
```

The whole suite is green under a whole-map `tick`, `event_loop_debounce`
included. The claim is correct, the finding's suggested repair would not have
produced a driver, and the weakened assertion at `debounce.rs:271-276` — *"at
most one command per tick reaches the controller"* — is what is true of the run.

The doc at `debounce.rs:127-147` states the unobservability and points at
`tests/event_loop_full/` for the rule this case cannot reach, which I verified
independently under F-S2. Nothing to contest.

`pending.rs` restored; `git status` clean.

---

## § New findings

### F-S7 — the I-F write order has no case that could see it wrong, and may be unobservable

**Severity:** nit
**Location:** `crates/goad/src/glass.rs:187-195`

**Observed.** PHASE-01/VA-2 assigned this to an agent read, correctly — but the
failure mode needs a view replacement to a form with **more** fields than the
one it replaces, and no case does that. `fields.rs`'s only replacement is
`THREE_FIELDS` (3) → `ANOTHER_FORM` (1), and the three other loop targets never
change `view_id`.

**Evidence.** Swap `set_values` with the `if self.shown != showing` block: the
whole suite stays green. The reviewer could identify **no observable
consequence** of the swap either, on the ground that `root.values[field.slot]`
is a declarative binding that re-evaluates when `values` is written. **If that
is right, I-F is unobservable rather than merely untested and VA-2's premise
concerns a transient nothing reads.** Worth settling one way, because I-F is
stated as a load-bearing invariant in `glass.rs:168-186` and in §5.5.

**Disposition:** *settle first* — one mutation
decides whether I-F is unobservable or merely untested; dispositioned once it
is answered.
**Response:** **Settled by measurement, then dispositioned `doc-wrong`.** The mutation the
finding asks for — `set_values` swapped below the `if self.shown != showing`
block — was applied and `cargo test --workspace --no-fail-fast` came back exit 0,
zero failures. So the green reading reproduces.

**And the reviewer's suspicion is right: I-F is unobservable, not merely
untested.** Enumerated rather than reasoned from the headline. Every
`root.values[…]` read in `app.slint` is either a lazy, dependency-tracked binding
(`:426`, `:479`, `:550`, `:613`, `:686`, `:746`), which the `set_values` write
invalidates so it re-evaluates before anything reads it, or a click-time read
(`:762-763`). The five guards do break their binding by self-assigning, but they
fire on `changed epoch`, and the epoch bump is the **last** statement in either
order — so a guard never runs against stale values. `present` is synchronous, so
neither a case nor a person can observe a state between its statements; only a
statement that **latches** the transient could make the order matter, and the one
candidate is `set_vec`'s instantiation pass, where all six `init` handlers are
`root.inits += 1`.

**The order is kept and the comment is rewritten as the forward constraint that
actually holds it: no `init` handler may read `root.values`.** One that seeded
itself at init — the shape `seed-date`/`seed-time` use at click time — would
latch the stale read and make this load-bearing for real. Nothing enforces that;
the order costs nothing and is the shape that stays correct if the markup
acquires one. `design.md` §5.5's statement of I-F as a live invariant is a
Reconciliation row.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**. The original raiser no longer exists, and the repair is a comment rather than code, so there is nothing for round 2 to attack; the Outcome split assigns it here.

Verified by re-running the finding's own mutation rather than reading the Response's account of it, and by reading the comment that landed. The comment at `glass.rs:185-212` no longer states I-F as an invariant the current markup depends on. It says the transient is unobservable, enumerates *why* — every `root.values[…]` read is either a dependency-tracked binding the `set_values` write invalidates or a click-time read, and the five guards fire on `changed epoch`, which is the last statement in either order — and then states the constraint that actually holds the order: **no `init` handler may read `root.values`**.

**That is the right shape, and it is worth saying why**, because a finding of this kind usually ends in a case. There is nothing to write a case against: the property is a prohibition on markup that does not exist. The comment is the only instrument available, nothing enforces it, and the Response says so rather than implying the order is held. `design.md` §5.5's statement of I-F as a live invariant is the remaining half and is a Reconciliation row.

### F-R3 — the entry leaves the map on the enqueue, but a present can land before the command is served, and that present writes the draft's stale value over the widget a person is typing into

**Severity:** major — **reasoned from the tree and the locked sources, not
run.** The recipe at the end is what settles it.
**Location:** `crates/goad/src/pending.rs:212-214` (the removal);
`crates/goad/src/controller.rs:839` (the present at the top of every outer
iteration), `:944-979` (the inner loop, which does not drain `commands`);
`crates/goad/src/glass.rs:283`, `:302-308`, `:326` (the overlay lookup);
`crates/goad/ui/app.slint:489-494`, `:642-647` (the guards that do the writing)

**Expected.** `design.md` §5.3, quoted at `glass.rs:320-326`: a control shows
*"what a person has just done and the host has not recorded yet, then what the
draft holds"*. The overlay exists to cover exactly the interval between a
keystroke and the host recording it. **AC-5**: *"A present that changes nothing
about a field does not disturb it: no destroyed element, no moved caret, no
interrupted drag."*

**Observed.** The overlay's cover ends one step too early. `Debounce::tick`
removes the entry the instant `Wire::send` reports the command **enqueued**
(`pending.rs:212-214`), and `pending.rs:189-193` prices that as safe because
*"the guard corrects the widget on the next present"*. That reasoning assumes
the next present happens **after** `serve` has folded the edit. It does not
have to, and in the ordinary case it does not:

```
t=0     person types "abc"           hold() → map{(opt,fld) → "abc"}, timer armed 150 ms
t=0     serve is parked in select!   (outer loop, controller.rs:840)
t=20    sleep arm fires              Fired::Scheduled → engage() → present(busy=true)
                                     → serve now awaits the backend (controller.rs:928-943)
t=150   the debounce tick fires      update_timers_and_animations() runs it from the event
                                     loop, independently of serve's polling
                                     wire.send(Command::Edit{ "abc" }) → channel EMPTY
                                     (serve took Scheduled from the timer arm, not the
                                     channel) → try_send Ok → ENTRY REMOVED
t=380   the backend replies          absorb() → inner loop breaks (controller.rs:953)
                                     → OUTER LOOP TOP → glass.present(...)  ← controller.rs:839
                                        pending.carried()  is EMPTY        (removed at t=150)
                                        the draft           has NEVER SEEN "abc" (still queued)
                                        ⇒ values[slot].text = the pre-typing value
                                        ⇒ set_epoch → the guard fires → self.text = old value
t=381   serve reaches select!        recv() yields the queued Edit → controller.edit(…)
                                     → present again → "abc" restored
```

The interleave needs nothing unusual. The channel is capacity 1
(`main.rs:86`), `serve` does not drain it while it is inside the inner
`select!` (`controller.rs:944-978`), and `reduce`'s
`(Exchanged::Evaluation, false, _) → Shift::Retained` (`controller.rs:454-456`)
is the ordinary result of a scheduled poll — the view and the draft are both
retained, so the stale value the guard writes is a real previous value rather
than a cleared field.

**This is not only a flicker.** The present at `controller.rs:839` carries
`busy = false`, so the control is re-enabled at exactly the moment it is
holding the wrong text. A person typing on from there builds on the reverted
string, and the characters typed before the revert are gone from the widget and
from the next report. The same window reverts a `Slider` mid-drag and a numeric
`LineEdit` mid-number.

**Evidence.** Four checkable facts, no person required:

1. `pending.rs:212-214` — `if enqueued { self.held.borrow_mut().remove(&(option, field)); }`.
   The entry is gone at enqueue.
2. `controller.rs:839` — `glass.present(controller.frame(notice.raised()));` is
   the **first** statement of the outer loop body, before the `select!` that
   would dequeue the command.
3. `controller.rs:944-978` — the inner loop's three arms are `cancel.stopped()`,
   `&mut call` and `ingress.arrival()`. `commands` is not among them, so a
   command enqueued during an exchange waits for the outer loop.
4. `i-slint-core-1.17.1/platform.rs:289-293` —
   `update_timers_and_animations()` runs `maybe_activate_timers` from the event
   loop, so the debounce tick fires on its own schedule and not on `serve`'s.

**Why nothing reports it.** All three debounce-bearing loop targets drain the
channel and apply the edit **before** any present:
`tests/event_loop_overlay/overlay.rs:238-249` does
`while let Ok(command) = rx.try_recv() { … controller.edit(…) }` at the top of
every step, and the schedule comment at `:265-266` reads *"present between the
ticks: one value off the draft"* — off the draft precisely because the harness
has already applied what production leaves queued. The one interleave that
matters is structurally unreachable in the rig.

**Checked by the audit (session 2): the mechanism holds, one sub-claim does
not yet.** Facts 2 and 3 were read directly — `controller.rs:839` is the first
statement of the outer loop body and carries the comment `// busy = false
here`; the inner `select!` at `:945-978` has exactly three arms and `commands`
is not one of them. Fact 1 is already mutation-confirmed under F-S2. So **the
reverting present is real**.

What is **not** established is the cost F-R3 puts on it — *"a person typing on
from there builds on the reverted string"*. That needs the event loop to turn
between the reverting present and the correcting one, and on the trace as
written it does not: `glass.present` is synchronous, the following `select!`
finds `commands.recv()` already ready, and `Command::Edit` resolves through
`controller.edit` to `None` without an await, so both presents happen inside a
single poll of `serve` and the stale text is written and overwritten before any
frame is painted. On that reading the cost is **AC-5, not AC-4**: the guard
writes twice across a present that should have disturbed nothing, `reasserts`
is non-zero, and the caret moves — which is the criterion, and enough.

Two things could still make it AC-4. A `Shift::Replaced` at that absorb
refuses the queued edit `SupersededView` and the characters are genuinely gone
— though §8 **R5** already prices a view replacement that way. And any await
introduced between those two presents turns it back into lost typing. Both are
reasons to settle it with the case rather than to argue it down.

**It bears directly on the F-A1 repair, in the wrong direction.** Today the
controls are deaf for the whole exchange, so a tick landing mid-exchange comes
from typing that finished before the exchange began. Narrowing `busy` makes
typing *during* an `Evaluate` the ordinary case, which is precisely when this
interleave fires. Repairing F-A1 without answering F-R3 trades an exchange-long
deafness for a per-poll revert.

**To settle it**, one loop-tier case: hold the backend's reply, type, let the
tick enqueue while the exchange is in flight, complete the exchange, and read
the `LineEdit`'s text at the present that follows — before the step that drains
the channel. Or, cheaper, assert `reasserts == 0` across that present, which is
AC-5's own instrument.

**Disposition:** `fix-now`
**Response:** Applied. `serve`'s outer loop now drains
`commands` through the existing `dispatch` **before** `glass.present`,
applying every queued command that resolves without an exchange and stopping
at the first that needs one. `refusal_re_arms` is `false` for a drained
command by construction; the drain bypasses the outer `biased`
`cancel.stopped()` arm for one command, which is harmless because the inner
`select!` a drained `Choose` lands in is `biased` on `cancel.stopped()` too
and so drops `call` before the backend is polled — written into the comment
rather than left to the reader. `pending.rs`'s *"the guard corrects the widget
on the next present"* is corrected in place: it is true again, and it is the
drain that makes it so.

**The sub-claim this finding left open is settled, and against the finding's
headline.** Both presents happen inside one poll of `serve`, so the widget's
text ends up correct either way and the cost is AC-5's, not AC-4's — exactly
as the audit's re-reading predicted. What makes it observable at all is that
`glass.present` ends in `window.show()`, whose `ensure_tree_instantiated` runs
the change handlers (`i-slint-core-1.17.1/window.rs:648-663`), so the guard
runs synchronously at each present: the un-drained loop writes the pre-typing
value back and then writes the typed value back, and `reasserts` counts both.

`tests/event_loop_drain/` is the case, under the production topology with the
backend held by a `oneshot` rather than by a `sleep`. It reads the debounce
map's size on both sides of the tick, so it cannot pass while measuring an
edit the overlay still covered. Injection: the drain neutered gives
`reasserts: 2` at the reading after the fold, with the text still correct —
which is both the defect and the reason the text is not the instrument.

**Outcome:** `verified` — set by round 2's behaviour dimension as raiser.

**Evidence.** The claim re-derived at the tree: `controller.rs:890-897` is the
drain, `:898` is still the first present of the iteration, the inner `select!`
at `:1017-1045` still has exactly three arms and `commands` is not among them,
and `Wire::send` is `try_send` only (`wire.rs:193-205`) at capacity 1 — so the
interleave the finding describes is the one the drain now covers.

*The mutation the Response names, run.* The drain body wrapped in `if false`:
`event_loop_drain` **FAILED** at `drain.rs:496` with `reasserts: 2` against `0`,
exactly as claimed, and the widget's text correct at both readings — which is
also why the Response is right that the text is not the instrument. Every other
`-p goad` target green, so **`event_loop_drain` is the sole instrument**.

**What the case would survive, recorded rather than raised.** It queues exactly
one command, so it cannot tell the committed `while drained.is_none()` loop from
a single `try_recv`. Harmless at the current capacity of 1, and no mutation of
the loop body changes behaviour there — but the loop's *iteration* is untested
and its correctness rests on two facts in two other files.

**The new comment the repair added is not true of all three present sites** —
**F-B3**. The timing of the case that holds the repair is **F-B4**.

### F-R4 — one full present, `window.show()`'s instantiation pass included, per refused ingress arrival, at a rate an untrusted writer sets

**Severity:** minor — **reasoned from the tree and the locked sources, not run.**
**Location:** `crates/goad/src/controller.rs:856-866`, `:877-894` and `:839`;
`crates/goad/src/glass.rs:225-231`

**Expected.** `design.md` §5.4 puts the ingress arm last *"so that a watcher
emitting at machine rate cannot starve a scheduled firing"* (`controller.rs:852-855`),
and SPEC-002/R-12's spacing floors the rate at which an arrival may cause an
**exchange**. Nothing floors the rate at which a refused arrival causes a
**present**.

**Observed.** In the outer loop, `Some(arrival) => Fired::Ingested(arrival)`
(`controller.rs:865`) goes to `ingest`, which returns `None` for all three
refusal shapes — a malformed envelope (`:676-679`), inside the spacing
(`:683-687`), an unreadable clock (`:697-701`). `serve` then takes
`let Some(attempted) = attempted else { continue; }` (`:892-894`) back to the
top of the loop, whose first statement is `glass.present(...)` (`:839`).
`ingest`'s `TooSoon` branch returns **before** the anchor is written
(`:683-687` precedes `:692`), so being refused for arriving too soon does not
itself slow the next refusal.

One present is not cheap. `option_models` rebuilds the entire row-model tree —
every `OptionRow`, `FieldBlock`, `FieldRow` and alternatives `ModelRc` — and
`present` then discards it whenever the view is unchanged (`glass.rs:146`
against `:189-195`). A fresh `VecModel` is allocated for `values` and another
for `diagnostic_lines`. `epoch` is bumped, which queues the `changed tick`
ChangeTracker of **every drawn control**. And `self.window.show()`
(`glass.rs:227`) is not a no-op on an already-visible window: it runs
`ensure_tree_instantiated()`, `update_window_properties()`, `set_visible(true)`
and `renderer().resize()` every time
(`i-slint-core-1.17.1/window.rs:1626-1653`), and `ensure_tree_instantiated`
runs the instantiation-plus-change-handler loop up to ten times
(`window.rs:648-665`).

**Evidence.** The control-flow chain above is four line citations in one file.
The cost of `show()` is `window.rs:1634-1648`. The claim that a refused arrival
presents nothing belongs to the **inner** loop's arm and is accurate about
itself (`controller.rs:966-971`); the outer loop is a second site the comment
does not reach.

**Why it is not merely cosmetic.** It is the amplifier for F-R3 and for F-R5: a
writer on the ingress socket sets how often the guard pass runs against a form
a person is typing into, and how often F-R5's tray push goes out.

**Disposition:** `fix-now` → **re-dispositioned `follow-up`** (`audit-log.md`,
2026-09-20, third entry). The finding stands in full; what changed is where it
is answered.
**Response:** **There is no repair inside this slice that is not a design
change, and that was established rather than assumed.**

`option_models` builds `rows` and `values` in **one** walk in which the slot
*is* `values.len()` (`glass.rs:343`) — invariant **I-B**, *"there is no second
counter that could fall out of step with the vector's own length"*. `values` is
written on every present (`glass.rs:221`) and only `rows` is conditional
(`:223-241`), so splitting the walk to skip the half that is discarded
reintroduces precisely the second counter I-B exists to forbid, and is the
parallel implementation `CLAUDE.md` forbids. `show()` on an already-visible
window is the *totality* argument at `glass.rs:33-35`, not an oversight.
Flooring the rate instead is worse: `ingest`'s `TooSoon` branch returns before
the anchor is written, and writing it there would let a flood push the anchor
forward indefinitely and starve the events SPEC-002/R-12 spaces.

**What is left of the cost, measured against the tree as it now stands rather
than as the finding found it.** The two amplifiers this finding was raised as
the amplifier *for* are repaired: **F-R5** removed the per-present tray
re-rasterisation and push, and **F-R3** removed the per-present guard revert.
What remains per refused arrival is one discarded `rows` build, two `VecModel`
allocations, an epoch bump whose guards then compare equal and write nothing,
and `show()`'s instantiation pass — **CPU on the UI thread, with no
user-visible disturbance**, at a rate a local writer sets.

**And the question underneath it belongs to canon.** The *inner* loop already
holds the opposite position deliberately: an arrival refused during an exchange
presents nothing (`controller.rs:1029-1048`), cited to SPEC-003/R-15 and
`review-design.md` F-15. R-15 requires a refusal decided **while idle** to reach
the diagnostics surface, so suppressing the outer loop's present defers that to
the next scheduled firing — and R-15's own verification case reads
`served.controller.frame(false).diagnostics`
(`tests/renderer/ingress.rs:1230`), the retained model rather than the window,
so **canon's instrument would not report the change**. Deciding when an idle
refusal must become visible is a spec amendment with its own verification, and
it is what the follow-up carries.

Not deferred for being large — `AGENTS.md` forbids that, and this is not large.
Deferred because it is a different unit of work. Landed in `slice-009.md`
§Follow-ups, which is where a `follow-up` disposition is required to land.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**, per
the Outcome split (`audit.md`). The finding produced no code, so there is
nothing for round 2 to attack. The finding's own control-flow claim was
re-derived from the tree this session and is unchanged:
`controller.rs:945-947` → `:961-963` → `continue` → `:898` `glass.present`.

### F-R5 — the tray icon is re-rasterised and pushed to the desktop's tray service on every present, because slint compares an `Image` by buffer pointer

**Severity:** minor — **reasoned from the locked sources, not run.**
**Location:** `crates/goad/src/glass.rs:220`;
`crates/goad/src/diagnostics.rs:452-467`

**Expected.** `glass.rs:33-35` — every property is written every call, which is
the answer to a display server failing partway through an update. Writing a
property whose value has not changed is expected to cost the write and nothing
downstream; `SharedString` behaves that way, which is why the sibling
`set_hover_text` at `glass.rs:221-223` is not raised here.

**Observed.** `tray_icon` builds a **new** `SharedPixelBuffer` on every call
(`diagnostics.rs:458`) and fills it by evaluating `pixel_at` for each of
`ICON_EDGE² = 32 × 32` pixels, each of which integrates
`SAMPLES_PER_EDGE² = 16` coverage samples (`diagnostics.rs:418`, `:441`,
`:459-464`, `:473-477`) — ~16 000 sample tests and a 4 KiB allocation per
present, for an icon with exactly two possible values.

The downstream cost is the one worth reporting. `SystemTrayIcon::init`
installs an `icon_tracker` whose notify calls `handle.set_icon(icon)` on the
platform handle (`i-slint-core-1.17.1/items/system_tray.rs:326-341`), and a
`ChangeTracker` fires only when the new value is `!=` the old
(`properties/change_tracker.rs:131-134`). `Image` derives `PartialEq` over
`ImageInner` (`graphics/image.rs:774`), whose `EmbeddedImage` arm compares
`l_buffer == r_buffer` (`:643`) — and `SharedImageBuffer::eq` compares
**`data.as_ptr()`**, not contents (`graphics/image.rs:211-223`). A freshly
allocated buffer never shares an address with the one it replaces, so two
byte-identical idle icons compare unequal and the tracker fires. The platform
tray icon is therefore re-set on **every** present: every command, every
scheduled poll, and — via F-R4 — every refused arrival.

**Evidence.** `graphics/image.rs:211-223` is the whole finding; the rest is the
call chain. The negative control is in the same file: the tooltip travels as a
`SharedString`, which compares by content, so `tooltip_tracker`
(`system_tray.rs:143`) fires only on a real change. The two properties are
written side by side at `glass.rs:220-223` and behave differently.

**The cheap repair exists.** `tray_icon` has two possible results; rasterising
each once and handing out clones keeps the buffer pointer stable, which makes
the tracker fire exactly when the state changes and costs nothing at the
boundary.

**Disposition:** `fix-now`
**Response:** Repaired, and it is the one behaviour change in this group. `tray_icon` now
rasterises the two icons once per thread and hands out clones; the rule itself
moved unchanged into `rasterise`. The mechanism is recorded on `tray_icon` so the
next author does not rediscover it: slint compares an `Image` by buffer
*address* (`graphics/image.rs:211-223`, reached through `:643` and `:774`), so a
freshly allocated buffer never compared equal to the one it replaced and
`icon_tracker` (`items/system_tray.rs:326-341`) fired on every present.
Thread-local rather than a global because `slint::Image` is neither `Send` nor
`Sync`; stability within a thread is what a `ChangeTracker` reads. The sibling
`set_hover_text` needed nothing: a `SharedString` compares by content.

**Outcome:** `verified` — set by round 2's behaviour dimension as raiser — and
see **F-B6**, which is about what *holds* the repair, not whether it is right.

**Evidence.** The finding's single load-bearing citation read directly rather
than taken: `i-slint-core-1.17.1/graphics/image.rs:211-223` is
`impl PartialEq for SharedImageBuffer`, and all three arms compare
`data.as_ptr()` — address, not contents.

*The repair, measured.* No case in the tree asserts image identity, so the
reviewer built one, appended temporarily to `tests/renderer/tray.rs`:

```rust
let a = tray_icon(TrayState::Idle);
let b = tray_icon(TrayState::Idle);
assert!(a == b);
assert!(a != tray_icon(TrayState::Fault));
```

Green at `5227ec1`. With `tray_icon`'s body reverted to the pre-repair
`rasterise(state)`, the probe **FAILED** and all 203 other `renderer` cases
passed. Probe removed with the reversion left in place: **every `-p goad` target
green** — which is **F-B6**. Both files restored.

**The class, enumerated.** *A property written on every present whose slint
comparison is by identity rather than content.* `set_image` — repaired.
`set_hover_text` — `SharedString`, content, correctly excluded. The reviewer
excluded `set_values` and `set_diagnostic_lines` on the ground that neither has
a `ChangeTracker` behind it, which is true and is the right test **for this
class**; a repeater is a second consumer of model identity that the test does
not reach, and that is **F-B9**, raised separately and not against this
Outcome.

### F-R6 — once armed, the `Debounce` is a reference cycle through slint's thread-local timer list and is never dropped

**Severity:** minor — **reasoned from the locked sources, not run.**
**Location:** `crates/goad/src/pending.rs:170-176` (`arm`)

**Expected.** `pending.rs:53-56` — *"Two fields and no more … this module holds
no way to reach the loop of its own accord"*, and `pending.rs:57-59` explains
`&Rc<Self>` as *"so the timer's callback can hold the map it will read"*. The
lifetime consequence of that choice is not stated anywhere.

**Observed.** `arm` builds `let holding = Rc::clone(self);` and moves it into
the callback. `Timer::start` does **not** store the closure in the `Timer`
struct — the `Timer` holds only a `Cell<Option<NonZeroUsize>>` id
(`i-slint-core-1.17.1/timers.rs:61-66`) — it boxes the closure into the
thread-local `CURRENT_TIMERS` slab (`timers.rs:84-93`). So the graph is

```
Rc<Debounce> ──► Debounce.timer: slint::Timer ──(id)──► CURRENT_TIMERS[id].callback
      ▲                                                            │
      └────────────────────────────────────────────────────────────┘
                         the closure holds Rc<Debounce>
```

The only thing that removes the slab entry is `Timer::drop`
(`timers.rs:188-203`), which cannot run while the strong count is held up by
the entry it would remove. After one `arm`, the `Debounce`, its `BTreeMap` and
the `Wire` clone inside the closure are retained for the life of the thread.
`tick` re-arms rather than stopping (`pending.rs:218-220`), and a final tick
that empties the map leaves the last closure registered.

**Evidence.** `timers.rs:61-66` (the `Timer` holds an id, not a callback),
`:84-93` (the closure goes to the thread-local), `:188-203` (`Drop` is the only
deregistration). The cycle is closed by `pending.rs:171` plus `:174`.

**What it does and does not cost today.** In production, nothing: `main.rs:95`
creates one `Debounce` for the process. It costs one leaked map and one
retained `mpsc::Sender<Command>` clone per test target that arms the timer, and
it is the reason `Ending::Closed` can never be reached from the callback side
(F-R9). It becomes real the moment a future slice makes a `Debounce` per view
or per window, which is what the raise is for: the type's doc argues its field
count and says nothing about its lifetime, so the next author has nothing to
read. `Weak::upgrade` inside the callback, or a `timer.stop()` on the empty
tick, closes it.

**Disposition:** `doc-wrong`
**Response:** Dispositioned `doc-wrong` and repaired there: the cycle is real, and in
production it costs nothing, so what was missing was the statement rather than a
different shape. `Debounce`'s doc now carries the retention argument in full —
`arm` moves an `Rc<Self>` into a closure that `Timer::start` boxes into the
thread-local slab rather than into the `Timer` (`timers.rs:61-66`, `:84-93`),
and only `Timer::drop` (`:188-203`) removes that entry, which cannot run while
the entry holds the strong count up — with the diagram, the per-test-target
cost, the link to F-R9, and the two closers (`Weak::upgrade` in the callback, or
`timer.stop()` on the empty tick). The type argued its field count and said
nothing about its lifetime; it does now.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**. The original raiser no longer exists, and this finding's repair is a document rather than code, so there is nothing for round 2's adversarial eye to attack; the user's decision on the Outcome split assigns it here. Verified at `pending.rs:62-91`, and the cycle re-derived from the vendored source rather than from the finding: `timers.rs:84-93` boxes the callback into the thread-local slab as `CallbackVariant::MultiFire` — read directly — and `:188-203` is `Drop`, the only deregistration. The doc carries the graph, the diagram, the per-test-target cost, the link to F-R9 and both closers. `doc-wrong` is the right disposition because the cost in production is zero (`main.rs:95` creates one `Debounce` per process) and what was missing was the statement, not a different shape.

### F-R7 — `Glass::present`'s doc says a `show()` failure is reported and the process keeps running; one failure inside `show()` panics before it can return

**Severity:** minor — **reasoned from the locked sources, not run.**
**Location:** `crates/goad/src/glass.rs:131-134` and `:225-231`

**Expected.** The method's own doc: *"Infallible: every property setter returns
`()`. A `show()` or `hide()` failure is reported on stderr through
`diagnostics::report_platform` and `present` returns; the process keeps
running."* That is a claim about code, and it is what the handling at
`glass.rs:229-231` is built to deliver.

**Observed.** `WindowInner::show` returns `Result<(), PlatformError>` and
propagates `set_visible(true)?` (`i-slint-core-1.17.1/window.rs:1636`), which
is the failure the glass handles. Twelve lines later the same function calls
`self.window_adapter().renderer().resize(size).unwrap()`
(`window.rs:1648`). A renderer resize failure therefore aborts the process from
inside `show()` and never reaches `report_platform`.

**Evidence.** `window.rs:1626-1653`, one `?` and one `.unwrap()` in the same
function. `present` calls `show()` on every non-hidden frame
(`glass.rs:225-228`), so the exposure is every present, not startup only.

**Scope, stated honestly.** This is not a backend input taking the host down —
no protocol message reaches it — so the fourth invariant is not breached. What
is wrong is narrower: a doc comment makes a code claim about `show()`'s failure
surface that the vendored source contradicts, and a reader planning the
display-server-fails-partway story will take the claim at face value.

**Disposition:** `doc-wrong`
**Response:** Repaired in the doc. `Glass::present`'s contract now separates the failure it
*does* handle — `set_visible(true)?`, propagated out of `WindowInner::show`
(`window.rs:1636`) — from `renderer().resize(size).unwrap()` twelve lines later
(`:1648`), which aborts from inside `show()` and never reaches
`report_platform`. The doc says so, says the exposure is every present rather
than startup only, and says explicitly that no protocol message reaches it, so
the fourth invariant is intact. Nothing about the handling changed: the claim was
the defect.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**. The original raiser no longer exists, and this finding's repair is a document rather than code, so there is nothing for round 2's adversarial eye to attack; the user's decision on the Outcome split assigns it here. Verified at `glass.rs:130-146`: the contract now separates the failure it handles — `set_visible(true)?`, propagated out of `WindowInner::show` (`window.rs:1636`) — from `renderer().resize(size).unwrap()` twelve lines later (`:1648`), says the exposure is every present rather than startup only, and says no protocol message reaches it so the fourth invariant is intact. The handling is unchanged, which is right: the claim was the defect.

### F-R8 — `rescale` swallows the one weak-handle upgrade in the crate with no report

**Severity:** nit
**Location:** `crates/goad/src/install.rs:242-245`

**Observed.** `let Some(window) = window.upgrade() else { return; };` is the
crate's only `Weak::upgrade`. It is the correct shape, and the failure is
unreachable today — the three zoom callbacks live in the tray's callback table,
which is held by the `Tray` handle `SlintGlass` clones strongly
(`main.rs:96-98`), so the `PromptWindow` outlives every caller. What the site
does not do is say anything when it fails: no `report_platform`, no
`debug_log`, no diagnostics line. `CLAUDE.md`'s *"Every refusal is reported and
says which side was wrong"* is about backend refusals rather than this, so this
is taste rather than canon — raised because it is the one place in the renderer
where an action a person took can vanish without a trace, and because the
`else` branch reads as deliberate handling rather than as an unreachable arm.

**Evidence.** `install.rs:117`, `:120`, `:123` are the three `as_weak()` sites;
`install.rs:243` is the only upgrade. `grep -n "upgrade" crates/goad/src/*.rs`
returns that one line.

**Disposition:** `fix-now`
**Response:** Repaired. The upgrade now calls `report_platform` before returning, so the one
place in the renderer where an action a person took could vanish without a trace
says something. The finding's own reading is kept in the comment: the failure is
unreachable today, because the three zoom callbacks live in the tray's callback
table that `SlintGlass` holds strongly (`main.rs:96-98`), so the `PromptWindow`
outlives every caller. Reported rather than made unreachable-by-construction,
because the `else` branch reading as deliberate handling was half the finding.

**Outcome:** `verified` — set by round 2's behaviour dimension as raiser.

**Evidence.** `install.rs:245-252` now reports before returning. The branch is
unreachable today for the reason the finding and the new comment both give, so
no mutation can exercise it and none is claimed. What *is* checkable is the
enumeration, and it holds:
`grep -rn "report_platform(" crates/goad/src crates/goad/tests` returns exactly
`install.rs:251` and `glass.rs:276`, and `grep -n "upgrade" crates/goad/src/*.rs`
still returns one line. The repair is what the finding asked for.

**Raised against the same three lines:** `report_platform`'s own doc comment
still says *"The only caller is `SlintGlass::present`"*, and the string it emits
is prefixed *"the window could not be drawn"*. Neither is true of the new call
site. **F-B5**.

### F-R9 — `Ending::Closed` says "only reachable at teardown"; in production it is reachable at no time at all

**Severity:** nit
**Location:** `crates/goad/src/controller.rs:76-77` and `:842-844`

**Observed.** `Ending::Closed` is taken when `commands.recv()` yields `None`,
which requires every `mpsc::Sender<Command>` to have been dropped. In
production none of them ever is: `main.rs:86` binds `tx` for the whole of
`start`, which outlives `run_event_loop_until_quit`; `Wire::new` takes a clone
(`main.rs:89`) and `install` clones it into seven callbacks
(`install.rs:39`, `:65`, `:82`, `:93`, `:99`, `:104`, `:109`) that live in the
window's and the tray's callback tables for the life of the process; and
F-R6's cycle retains an eighth inside the armed timer's closure. The real
shutdown path is `Ending::Stopped` via `Cancel`, which is what `install.rs:95`
and `:110` trip.

**Evidence.** The eight retention sites above, plus `main.rs:86`. The variant is
reachable from the test tiers, which drop their senders, so it is not dead code
— only the doc's account of *when* is wrong.

**Disposition:** `fix-now`
**Response:** Repaired in the doc. `Ending::Closed` no longer says *"only reachable at
teardown"*: it says reachable from the test tiers, which drop their senders, and
in production from nowhere at all — with the eight retention sites enumerated
(`main.rs:86`, `main.rs:89`, `install.rs`'s seven callbacks, and F-R6's armed
timer closure) and the real shutdown path named as `Stopped` via `Cancel`. The
variant is not dead code and was not removed.

**Outcome:** `verified` — set by the orchestrator as **declared raiser**. The original raiser no longer exists, and this finding's repair is a document rather than code, so there is nothing for round 2's adversarial eye to attack; the user's decision on the Outcome split assigns it here. Verified at `controller.rs:76-87`: `Ending::Closed` no longer says *"only reachable at teardown"*. It says reachable from the test tiers, which drop their senders, and from nowhere at all in production, with the eight retention sites enumerated and `Stopped` via `Cancel` named as the real shutdown path. The variant was not removed, which is right — it is reachable, just not there.

### Note — `pending.rs`'s re-arm argument cites the wrong `timers.rs` arm

Not raised as a finding: the conclusion is right and the behaviour holds. But
`pending.rs:161-168` rests its re-arm argument on `timers.rs`'s `SingleShot`
path, and that is not the path this code takes — `Timer::start` boxes every
callback as `MultiFire` regardless of the `TimerMode` it is given
(`timers.rs:86-91`). The re-emplacement logic the argument depends on is the
`MultiFire` one, and it holds. Recorded here so the citation can be corrected
with the rest of the record rather than re-derived by the next reader.

### F-T1 — `event_loop_full`'s central reading is non-vacuous only by arithmetic on a literal copy of a private constant, and the copy going stale is silent

**Severity:** `minor` — **run.**

**Location:** `crates/goad/tests/event_loop_full/full.rs:64` (`STEP`), `:73`
(`STEPS`), `:229` (the comment that does the arithmetic), `:233` (reading B);
against `crates/goad/src/pending.rs:32` (`const DEBOUNCE`).

**Expected.** `full.rs:14-19` states what reading B is for: *"**B is the
claim.** The entry must still be there: the send came back `Full` … **C is what
stops B passing for the wrong reason**."* The target exists because F-S2 found
that the `Full` branch was held by nothing; it is the only driver of that branch
in the workspace. `design.md` §9: *"Every new case gets an injection pass"* — and
an injection pass establishes that a case reddens *now*, not that it goes on
doing so.

**Observed.** B is a reading taken at step 11, and it is a fact about a `Full`
send only while the debounce deadline falls between step 3 and step 11. Nothing
asserts that. The relation is carried by one comment — `full.rs:229`, *"Before
any tick can have fired: 150 ms after step 2 is step 8"* — which restates
`DEBOUNCE` as a literal. `DEBOUNCE` is a **private** `const` at `pending.rs:32`,
not `pub`, so no test can read it and no compiler keeps the two in step.

The window is `STEP * 1 < DEBOUNCE < STEP * 9`, i.e. 25 ms < `DEBOUNCE` < 225 ms
against the shipped 150 ms. **The two bounds fail differently:**

- **Below**: the debounce fires before step 3 occupies the channel, the tick's
  send succeeds into an empty channel, and `assert!(enqueued)` at `:224-227`
  fails — **loudly**, with a message naming the precondition. Good.
- **Above**: the debounce fires *after* reading B. The entry then stands at B
  because no tick has fired, not because a tick found the channel full; the
  drain at step 12 empties the channel; the re-armed tick fires into an empty
  channel and delivers; C passes. **The case is green and measures nothing.**

**Evidence.** Both bounds run, each applied to the tree and restored from a copy.

Upper bound, and it is the one that matters — a production tuning change alone:

```
# crates/goad/src/pending.rs:32
-const DEBOUNCE: Duration = Duration::from_millis(150);
+const DEBOUNCE: Duration = Duration::from_millis(400);

cargo test -p goad --test event_loop_full   →   test result: ok. 1 passed
```

Then, on top of that, F-S2's own injection I1 — `pending.rs:263-265`'s
`if enqueued` guard deleted, which is the rule this whole target exists to hold:

```
cargo test -p goad --test event_loop_full   →   test result: ok. 1 passed
```

**The enqueue rule is deleted and the only case that holds it is green.**

Lower bound, reproduced by making the stepper slow instead (`full.rs:64`,
`STEP` 25 ms → 200 ms), which is what a stalled machine does to it:

```
panicked at crates/goad/tests/event_loop_full/full.rs:224:9:
the occupying send must succeed into an empty channel, or the case measures nothing
```

That panic is raised from inside a slint timer callback and produces a clean
test failure rather than a wedge — measured, not assumed. It is also the one
place `full.rs:66-68`'s *"nothing here panics from inside the loop"* is untrue of
the file; the consequence is nil, which is why that is not raised separately.

**Scope, stated honestly.** This is **not** a flake under load. Both timers are
wall-clock; a slow machine delays the *stepper*, which moves the debounce
deadline to an *earlier* step index — away from the silent bound and towards the
loud one. The lower bound's margin is a 150 ms stall inside one 25 ms inter-step
interval: six-fold, and its failure is attributable. What this finding is about
is a **code** change to `DEBOUNCE`, and the fact that the resulting loss of
coverage is invisible.

**The sibling precedent is in the same slice.**
`crates/goad/tests/event_loop_drain/drain.rs:98-101` declares
`const TICK_STEPS: u8 = 3` with *"The debounce window is 150 ms (`pending.rs`),
so a tick lands three steps after the keystroke that armed it. Stated here
because every step number below is read against it."* — the same literal copy,
but named, and used in the failure message at `:478`. `full.rs` has the relation
in a comment only. Neither derives it from the constant, because the constant is
private; `drain.rs` at least makes the dependency greppable.

**What would close it**, stated as a candidate rather than a verified closer
(`docs/memory/verify-the-proposed-instrument.md`): make `DEBOUNCE` `pub`, or
expose a `Debounce::window()`, and have each loop target compute its step
schedule from it — so a change to the production constant either keeps the
schedule right or fails to compile. A target timed against a copy of a constant
it cannot see is timed against nothing.

### F-T2 — the `wildcard_enum_match_arm` deny does not reach `crates/goad/src/main.rs`, and `lib.rs` says it covers "this crate"

**Severity:** `minor` — **run.**

**Location:** `crates/goad/src/lib.rs:35` (the attribute), `:18-22` (the claim);
`crates/goad/src/main.rs` (the uncovered compilation unit).

**Expected.** `POL-001` §Verification sets the standard this comment is written
against: *"What each command holds, and what it does not, is the part that must
not be overstated."* The comment claims a boundary — *"it is denied for **this
crate only**, at no cost in exceptions: `goad`'s production code has no wildcard
over an enum at all"* — and the F-S3 Response repeats it as *"a lint-table entry
inside the gate's existing clippy pass"*.

**Observed.** `#![deny(…)]` is an inner attribute on a **crate root**, and
`crates/goad` has two: `src/lib.rs` and `src/main.rs`. Cargo compiles the binary
as a separate crate that merely *depends* on the library, so the attribute does
not apply to it. `crates/goad/Cargo.toml:50-51` is `[lints] workspace = true`,
which is exclusive — a package-local `[lints.clippy]` table cannot sit beside
it — so a crate-root attribute was a reasonable mechanism; it was simply put in
one of the two roots.

It is also not a lint-table entry. The workspace lint table is
`Cargo.toml:[workspace.lints.clippy]` and reaches every target of every member;
a crate-root attribute reaches one compilation unit. The two have different
boundaries, and the Response names the first while having written the second.

**Evidence.** A wildcard over a project enum appended to `main.rs`:

```rust
#[expect(dead_code, reason = "review probe")]
fn probe(kind: &goad_semantics::protocol::canonical::FieldKind) -> u8 {
  use goad_semantics::protocol::canonical::FieldKind;
  match kind {
    FieldKind::Choice { .. } => 1,
    _ => 0,
  }
}
```

```
cargo clippy -p goad --all-targets -- -D warnings   →   exit 0, clean
```

The identical wildcard inside `lib.rs`'s module tree is `error: wildcard match
will also match any future added variants`, cited to `lib.rs:35` — measured
under F-S3 above. `main.rs` restored; `git status` clean.

**Scope, stated honestly.** **AC-7 is not affected.** `drawn_form` is in the
library, so the mechanism F-S3 is about is held. `main.rs` carries no `_ =>` arm
today, so nothing is being absorbed. This is a finding about a boundary stated
wrongly in the place a future reader will look, on a crate root that holds
`serve` — the crate's largest match statements — and it costs one line to close:
the same `#![deny(clippy::wildcard_enum_match_arm)]` at the top of `main.rs`.

### F-T3 — `lib.rs`'s enumeration of the workspace's other wildcard matches undercounts them, and mischaracterises one

**Severity:** `minor` — **run.**

**Location:** `crates/goad/src/lib.rs:24-30`.

**Expected.** The comment is the written reason the deny is crate-scoped rather
than workspace-wide, and it is explicit that the reason is a fact about the
codebase rather than a budget: *"**Not workspace-wide, and the reason is not
cost.** The other four such matches are the opposite case: `ProtocolError::source`,
and three in `goad-shell/src/ingress/` (`envelope.rs:106`, `:118`,
`mod.rs:752`). Each is a match that chooses *no behaviour from the variant* …
so the wildcard's body is the right answer for a variant that does not exist
yet, and denying it there would buy four `#[expect]`s and no property."*
`docs/memory/check-precedent-before-calling-it-coupling.md` and
`docs/memory/verify-the-enumeration-not-the-conclusion.md` both bear: a claim
about the codebase is checkable, and a correct finding can carry a wrong
sub-claim into the artefact.

**Observed.** Two defects in one sentence.

**The count is not four.** Catch-all arms over an enum outside `crates/goad`:

| site | scrutinee | in the comment's list? |
|---|---|---|
| `goad-semantics/src/error.rs:238` (`ProtocolError::source`) | `ProtocolError` | yes |
| `goad-shell/src/ingress/envelope.rs:106` | `ProtocolError` | yes |
| `goad-shell/src/ingress/envelope.rs:118` | `serde_json::Value` | yes |
| `goad-shell/src/ingress/mod.rs:752` | `EnvelopeFault` | yes |
| `goad-shell/src/ingress/mod.rs:585` | `Option<&Refusal>` | **no** |
| `goad-shell/src/config.rs:47` | `Option<OsString>` | **no** |
| `goad-shell/src/state.rs:171`, `:186` (unit tests) | `Result<_, StateError>` | **no** |

Six to eight rather than four, depending on whether the two unit-test arms
count — and they do, because the deny in `lib.rs` demonstrably reaches `goad`'s
own lib **test** target: F-S3's shape-3 run reports `could not compile goad (lib
test)`. The three arms at `normalize.rs:182`, `:246` and `:381` are correctly
excluded — they match on `wire.kind.as_str()`, a `&str`, which this lint does
not see.

**And one of the four is the opposite of what the comment says it is.**
`envelope.rs:116-122` reads the variant; that is the entire point of the arm:

```rust
match value {
  serde_json::Value::Object(object) => Ok(object),
  other => Err(EnvelopeFault::NotAnObject {
    found: json_type_name(&other),      // behaviour chosen from the variant
  }),
}
```

*"Each is a match that chooses no behaviour from the variant"* is false of this
one.

**Evidence.** The enumeration, then each site read rather than counted off the
grep:

```
grep -rn '^ *_[a-z_]* =>| *other =>| *_ =>' crates/goad-shell/src crates/goad-semantics/src --include=*.rs
sed -n '116,122p' crates/goad-shell/src/ingress/envelope.rs
sed -n '583,586p' crates/goad-shell/src/ingress/mod.rs
sed -n  '43,49p'  crates/goad-shell/src/config.rs
```

**Scope, stated honestly.** The *conclusion* — deny for `goad`, widening is a
decision for another slice — survives, and is if anything better supported: more
sites means a larger cost, not a smaller one. What does not survive is the
stated reason, *"and the reason is not cost"*, which rests on the count and on
the characterisation. Both are now written into production source as fact. The
repair is to correct the sentence, not to widen the lint.

### F-T4 — three citations of `glass.rs:189` point at a comment, not at the guard they name

**Severity:** `nit` — **run.**

**Location:** `crates/goad/tests/renderer/fields.rs:1310`;
`crates/goad/tests/event_loop_picker/picker.rs:6`, `:45`.

**Expected.** Each is a locating citation in a doc comment, offered so a reader
can re-run a mutation or find a mechanism: *"delete the `if self.shown !=
showing` guard at `glass.rs:189` so every present destroys every field element"*
(`fields.rs:1310`), and *"`present` writes the row model only where the `ViewId`
changed (`glass.rs:189-195`)"* (`picker.rs:45`).

**Observed.** `if self.shown != showing {` is at `crates/goad/src/glass.rs:223`
and its block runs to `:241`. `glass.rs:189` is the middle of the I-F ordering
comment — a line of prose.

**Evidence.**

```
grep -n 'if self.shown != showing' crates/goad/src/glass.rs   →   223
sed -n '186,191p' crates/goad/src/glass.rs                    →   prose, no code
grep -rn 'glass.rs:189' crates/                               →   the three sites above
```

**Scope, stated honestly.** A nit, and it does not touch F-S1's substance: the
`fields.rs` citation quotes the guard's source text, which is unique in the
file, so the mutation is still reproducible — I reproduced it from that quote.
Raised because the line moved *inside this slice* (`d9fe587` inserted the F-S7
comment above it) and all three citations were written or touched after that, so
nothing caught the drift.

---

### F-B1 — `serve` can stop engaging altogether and the whole suite stays green: slice 003's double-submit guard is held by no case through the production loop

**Severity:** major — **run**
**Location:** `crates/goad/src/controller.rs:993` (`controller.engage(exchanged);`)

**Expected.** F-A1's Response: *"the option `Button`'s slice-003 double-submit
guard fires exactly when it was written to"*, and `Controller::engage`'s own doc
(`controller.rs:405-412`). The claim is about `serve`, and `serve` is the only
place `engage` is called in production.

**Observed.** `engage` maps `Exchanged::Answer` to `engaged`; the **call site**
supplies the `Exchanged`, and nothing checks that it supplies the right one.
Replacing `controller.engage(exchanged)` with
`controller.engage(Exchanged::Evaluation)` — the production loop never engaging
at all, so the option `Button` is never disabled and slice 003's double-submit
guard is dead — leaves **every target of `cargo test -p goad --no-fail-fast`
green**, all three new loop targets and all 203 `renderer` cases included.

Every case that holds `busy` calls `Controller::engage` **itself**:
`event_loop_busy/busy.rs:246,254`; `renderer/table.rs:927,932,942,960`;
`renderer/wiring.rs:476,506,1842`. The mapping inside `engage` is held nine
times over; the argument that produces it is held nowhere.

`event_loop_drain` *does* run the production `serve` and catches the **opposite**
mutation — hardcoding `Exchanged::Answer` reddens it at `drain.rs:462` — because
its whole arrangement is a key typed during an evaluation. There is no
counterpart case for an answer in flight under `serve`.

**Evidence.** Three runs, each restored from a scratch copy:

| mutation at `controller.rs:993` | `cargo test -p goad --no-fail-fast` |
|---|---|
| `controller.engage(Exchanged::Answer)` | `event_loop_drain` FAILED at `drain.rs:462`; 13 other targets green |
| `controller.engage(Exchanged::Evaluation)` | **all targets green**, 0 failures |
| unmutated | all targets green |

**Scope, stated honestly.** The production line is **correct** as written; this
is a coverage finding, not a live defect, and it contests neither F-A1's nor
F-R2's repair. What it says is that the half of the repair that *keeps* a
behaviour is unheld — the half a future refactor is most likely to drop, on the
exact line this slice narrowed. The shape of the missing case is
`event_loop_drain`'s with `Command::Choose` in place of `Command::Evaluate`.

**Disposition:**
**Response:**
**Outcome:**

### F-B2 — a picker survives `open_diagnostics()`, and the diagnostics pane's only exit button is then unreachable by pointer

**Severity:** major — **run**, with a positive control
**Location:** `crates/goad/src/glass.rs:224-241` (the dismiss is inside
`if self.shown != showing`); `crates/goad/src/controller.rs:172-178`
(`surface()`); `crates/goad/ui/app.slint:952-956` (the pane's
`close-diagnostics` button)

**Expected.** F-R1: *"Nothing in canon or the design contemplates a picker still
on screen after the view it belongs to is gone"*, and `CLAUDE.md`'s fourth
invariant in its host form — no host action may leave the window unanswerable.
`design.md` §5.4/§5.5 end a pick in `accepted`, `canceled`, or `compose`
failing.

**Observed.** `surface()` maps `(Focus::Diagnostics, _)` to
`Surface::Diagnostics` while `frame.shown` stays `Some` — `frame()`
(`controller.rs:426-435`) passes `self.shown.as_ref()` unconditionally — so
`present` writes `WindowMode::Diagnostic`, taking the whole prompt block out of
the tree, with `showing == self.shown`. **The dismiss branch is not taken** and
the picker stays up over the diagnostics pane, whose every control is then
unreachable by pointer, the one button that leaves it included.

**Evidence.** Measured by temporarily extending
`tests/event_loop_picker/picker.rs` (restored; byte-identical to `5227ec1`
afterwards). Step 11's `absorb(…)` replaced with `controller.open_diagnostics()`,
then `close-diagnostics` clicked with the picker up, then the picker dismissed
by a real pointer click on its own `Cancel` and `close-diagnostics` clicked
again:

```
H  after open_diagnostics() and a click on close-diagnostics
   picker: true,  chosen: ["v1/morning", "v2/evening"]

I  the same click with the picker dismissed - the control
   picker: false, chosen: ["v1/morning", "v2/evening", "close-diagnostics"]
```

**Reading I is the positive control**: same driver, same element query, same
click, and it raises the callback once the picker is gone. So H is not a driver
that missed.

Reachable in production: `Command::OpenDiagnostics` comes from the tray menu
(`app.slint:1074`), a native surface a person can reach while a picker is up.

**Scope, stated honestly.** `major` and not `blocker` for F-R1's own reason: the
picker's `Cancel` still closes it, so it is not a permanent lockout. A second,
weaker member of the class is recorded rather than glossed — the `ComboBox`
dropdown is opened from inside a repeated row and `active_popups` holds the
popup's item tree strongly (`window.rs:1955-1962`), so a dropdown should outlive
a view replacement too; `dismiss-pickers()` does not close it, but its
`close-on-click-outside` policy means one swallowed click dismisses it, which is
why it is a footnote.

**Disposition:**
**Response:**
**Outcome:**

### F-B3 — `pending.rs`'s new claim that the drain makes *the next present* safe is false for one of `serve`'s three present sites

**Severity:** minor — **reasoned from the tree, not run**
**Location:** `crates/goad/src/pending.rs:236-244`;
`crates/goad/src/controller.rs:1046`

**Expected.** The comment `665dcf3` added at `pending.rs:236-244`: *"`serve`
closes the interval from its end: it applies every queued command that resolves
without an exchange **before** it presents, so **the next present is never one
that has not yet served this send**."* A universally quantified claim about
`serve`'s presents, written by this repair, in the file the repair corrects.

**Observed.** `serve` has three present sites — `controller.rs:898` (the outer,
drained), `:994` (the busy present, reached synchronously from `:898`'s
`select!` with nothing able to enqueue in between), and **`:1046`**, inside the
inner `select!`'s `ingress.arrival() => None` arm. That third present is reached
**after an await**, is not preceded by a drain, and lands precisely while an
exchange is outstanding — F-R3's own window. A debounce tick that enqueued
during that exchange has left the map (`pending.rs:212-214`) and has not been
served, so this present writes the pre-typing value back over the widget: F-R3's
exact defect, at a site the repair does not reach.

**Evidence.**
- `grep -n "glass.present" crates/goad/src/controller.rs` → `898`, `994`,
  `1046`. Only `898` is preceded by the drain.
- `controller.rs:1040-1047` is the arm, and its own comment says it *"costs one
  presentation per **process**"* — so the exposure is bounded at one, and only
  once the ingress accept task has ended (`ingress_stopped()`,
  `controller.rs:674-682`, SPEC-003/R-15).
- `event_loop_drain` cannot see it: it builds `Ingress::none()`, so that arm
  never fires.

**Scope, stated honestly.** The behavioural cost is small — at most one widget
revert per process, and only after ingress has died. What is wrong without
qualification is **the sentence**. The cheap repair is to drain before that
present too, or to narrow the sentence to the outer loop's present and say which
site is excluded and why.

**Disposition:**
**Response:**
**Outcome:**

### F-B4 — all three new loop targets fail under CPU oversubscription, and the failure is the liveness bound rather than any assertion: the stepper harness stalls

**Severity:** minor — **run**, instrumented at the bound
**Location:** `crates/goad/tests/event_loop_drain/drain.rs:96-106`,
`crates/goad/tests/event_loop_busy/busy.rs:54,59`,
`crates/goad/tests/event_loop_picker/picker.rs:52,57` — the `STEP` /
`LIVENESS_BOUND` pairs, and each file's step schedule

**Expected.** `docs/memory/timed-test-margins-are-measured-at-the-bound.md`:
measure at the bound the assertion governs, under real oversubscription. Slice
003's recorded worst margin *anywhere in the suite* is **10.8x**, with 13/13
full-suite runs green at loadavg 164-170 on 32 cores. `POL-001`: the gate exits
0.

**Observed.** One sustained batch, 192 spin loops on 32 cores, each target run 8
times in sequence, uninstrumented, at a sustained loadavg of 195-198:

| target | result |
|---|---|
| `event_loop_drain` | pass 6, **fail 2** |
| `event_loop_busy` | pass 7, **fail 1** |
| `event_loop_picker` | pass 4, **fail 4** |

**Every captured failure is the liveness bound, not a claim** — the stepper did
not reach its last reading inside 10 s / 20 s, against nominal runs of
240 ms / 850 ms:

```
busy.rs:276 — the stepper must have taken all three readings within 10s:
  [ Reading { at: "A a key with nothing engaged", busy: false, shown: "a",
              edits: ["v1/morning/noted=a"] } ]
```

So the mechanism is not a tight assertion margin; it is the whole slint-timer
stepper stalling for tens of seconds under load, with the liveness bound then
converting a stall into a red gate rather than a hang. **A 40x nominal margin
(`busy`) was not enough.**

`event_loop_drain` carries three genuinely tight quantities the other two do
not. Instrumented at the bound, the instrumentation reversed afterwards:

| bound | assertion it governs | idle | margin |
|---|---|---|---|
| step 9 → step 10 must be **< 150 ms** | `typed.held == 1`, `drain.rs:470` | 50.4 ms | **3.0x** |
| step 9 → step 13 must be **> 150 ms** | `enqueued.held == 0`, `drain.rs:482` | 200.2 ms | **1.33x** |
| step 1 → step 17 must be **< 3 s** | nothing asserts it; past `MINIMUM_SPACING` (`controller.rs:554`) `serve`'s standing timer fires an unplanned evaluation | 799 ms | 3.75x |

**Evidence.** Loaded, same instrumentation, 192-256 spinners:

```
BOUND step9->step10 = 153.604776ms  ← over its bound, and the case still passed
BOUND step9->step10 = 741.998273ms  ← 5x over its bound, and the case still passed
BOUND step1->step17 = 6.44441056s   ← over MINIMUM_SPACING; that run FAILED
```

**The two passes over a violated bound are worth as much as the failures**: at
742 ms between step 9 and step 10 the debounce *had* elapsed and `typed.held ==
1` held anyway, because the stepper and the debounce timer happened to be
dispatched in that order inside one `update_timers_and_animations` pass. The
assertion passed for a reason its message does not name.

Reproduce: 192 `(while :; do :; done) &` on a 32-core box, then
`cargo test -p goad --test event_loop_picker` eight times.

**Scope, stated honestly.** A repeat batch at a *lower* sustained load passed
6/6 `picker` and 5/6 `busy`, so the threshold is somewhere between roughly 3x
and 6x oversubscription on this machine and the failure rate is not a stable
number; the loadavg figures are approximate. What is not approximate: **at the
load the project's own memory document uses as its standard, this suite does not
stay green, and slice 003's did.**

**Independently witnessed.** Session 4's own `just check` failed here on
`event_loop_busy` at loadavg 198, reporting *"the stepper must have taken all
three readings within 10s: []"* — zero readings in 64 s, then one in 38 s on a
re-run. That witness and this finding were produced without contact; the
reviewer was not told of it. The instruments dimension's *"timing margins are on
the safe side of load"* is **not** a contradiction: it is about the deadlines
*under test*, where slowness moves a deadline to an earlier step index. This is
about liveness, which is the other direction.

**Disposition:**
**Response:**
**Outcome:**

### F-B5 — F-R8's repair falsifies `report_platform`'s own doc comment and emits a message that says the wrong thing

**Severity:** minor — **read**
**Location:** `crates/goad/src/diagnostics.rs:404-413`;
`crates/goad/src/install.rs:251`

**Expected.** This ledger's Subject: *"document-truth divergences … appear here
only where a doc comment or a declared surface makes a code claim that is
false."* `d9fe587`'s own commit message: *"five doc claims the code
contradicts"*.

**Observed.** `diagnostics.rs:408-410` still reads *"…and the process keeps
running. **The only caller is `SlintGlass::present`**, when `show()` or `hide()`
fails after the loop has started."* `d9fe587` added a second caller **in the
same commit** and did not amend it. `report_platform_line` (`:404-406`) renders
`"goad: the window could not be drawn: {detail}"`, so the new call site emits
*"goad: the window could not be drawn: zoom: the window was gone before the
scale could be set"* — a window that was gone, reported as one that could not be
drawn.

**Evidence.** `grep -rn "report_platform(" crates/goad/src crates/goad/tests`
returns `install.rs:251` and `glass.rs:276`; the doc comment at
`diagnostics.rs:409` is unchanged from before `d9fe587`
(`git show d9fe587 -- crates/goad/src/diagnostics.rs` touches only
`tray_icon`/`rasterise`).

**Scope, stated honestly.** Nothing behaves wrongly; the branch is unreachable.
The finding is that **a commit whose stated purpose was to correct five false
doc claims created a sixth**, and the nearest fix is one sentence.

**Disposition:**
**Response:**
**Outcome:**

### F-B6 — F-R5's repair, the one behaviour change in `d9fe587`, is held by nothing in the gate

**Severity:** minor — **run**
**Location:** `crates/goad/src/diagnostics.rs:473-485` (`tray_icon`'s
`thread_local!`); `crates/goad/tests/renderer/tray.rs`

**Expected.** `AGENTS.md` §*Execute*: red / green / refactor. F-R5's own Response
calls this *"the one behaviour change in this group"*. The Brief's standing
warning: *"Every new case here claims an injection pass."* Here there is no case
at all.

**Observed.** Reverting `tray_icon`'s body to the pre-repair `rasterise(state)`
— restoring exactly the defect F-R5 raised — leaves **every target of
`cargo test -p goad --no-fail-fast` green**, all 203 `renderer` cases included.
`tests/renderer/tray.rs` asserts the icon's *pixel contents*, which are
identical either way; nothing asserts its **identity**, which is the whole of
the finding.

**Evidence.**

| tree | probe appended to `tray.rs` | result |
|---|---|---|
| `5227ec1` | `assert!(tray_icon(Idle) == tray_icon(Idle))` | green |
| `tray_icon` body → `rasterise(state)` | same probe | **FAILED**; the other 203 `renderer` cases passed |
| `tray_icon` body → `rasterise(state)` | probe removed | **all `-p goad` targets green** |

**Scope, stated honestly.** The repair is correct — F-R5's Outcome is `verified`
on the strength of the probe, not on the Response's account. This finding is only
that **the gate would not notice its removal**, and that the case which closes it
is the four lines of the probe.

**Disposition:**
**Response:**
**Outcome:**

### F-B7 — `wiring.rs::busy_clears_and_controls_re_enable_after_a_success` now arranges a frame `serve` cannot produce, and all three narrowed cases pair an `engage(Answer)` with an `absorb(Evaluation)`

**Severity:** minor — **read, with the reachability argument run against the code**
**Location:** `crates/goad/tests/renderer/wiring.rs:468-492`, and `:495-527`,
`:1824-1866` for the pairing

**Expected.** `docs/memory/tests-asserting-proxies.md`: a case's arrangement must
be one the production path can reach, or what it measures is not what its name
says.

**Observed, two things.**

1. **The first case's busy present is now unreachable.** It runs
   `controller.engage(Exchanged::Answer)` on a fresh `Controller::new()`, so
   `shown == None`, then asserts `window.get_busy()`. In production an
   `Exchanged::Answer` exists only as a `Pending::Respond`, which only
   `Command::Choose` produces, and `Controller::choose` begins
   `self.shown.as_ref().ok_or(Refused::SupersededView)?` (`controller.rs:312`).
   So `engaged == true` with `shown == None` is a frame `serve` cannot build.
   **Before the narrowing it *was* reachable** — the startup evaluation engaged
   with nothing shown — so this is a state the repair removed and the case kept.
2. **All three narrowed cases fold the exchange with the wrong `Exchanged`.**
   Each does `engage(Exchanged::Answer)` and later
   `absorb(Exchanged::Evaluation, …)`. `serve` computes
   `exchanged = pending.exchanged()` **once** (`controller.rs:992`) and hands the
   same value to both, so the pair never diverges in production.

**Evidence.** `controller.rs:301-314`, `:992-993`, `:1020`. `reduce`
(`controller.rs:482-490`) was checked for whether the mismatch changes the fold,
and in all three cases it does not: `(Evaluation, true, false)` and
`(Answer, true, false)` are both `Replaced`; `(Evaluation, false, true)` and
`(Answer, false, true)` are both `Retained`. **So the cases still assert
something true, and nothing they assert is wrong.**

**Scope, stated honestly.** No assertion is false and none regressed at
`665dcf3` — the change was mechanical. What the narrowing did is make one
arrangement unreachable without anything noticing, which is worth one comment or
a reshaped case, not a rewrite; the honest minimal repair for (1) is the one the
sibling case already uses, absorb a view before engaging.
`renderer/table.rs`'s `busy_is_false_after_absorbing_a_success` and
`..._a_failure` also engage an `Answer` with nothing shown and are deliberately
**excluded**: they are pure `Controller` cases about `absorb` clearing a flag,
assert nothing about a screen, and their minimality is the point.

**Disposition:**
**Response:**
**Outcome:**

### F-B8 — `invoke_dismiss_pickers()` runs markup code inside the I-F transient, and the rule the I-F comment states covers only `init` handlers

**Severity:** nit — **read**
**Location:** `crates/goad/src/glass.rs:180-241`;
`i-slint-core-1.17.1/window.rs:1949-1951`

**Expected.** `glass.rs:180-212`'s I-F comment: *"Writing the new view's values
while the old rows still index them costs nothing, **because the next statement
destroys those rows**. … **So the rule this comment exists to state is
forward-looking: no `init` handler may read `root.values`.**"*

**Observed.** `set_values` is no longer immediately followed by
`self.options.set_vec(rows)`. `db1d702` inserted
`self.window.invoke_dismiss_pickers()` between them, and that call is not inert
markup: `WindowInner::close_popup_impl` ends with

```rust
if let Some(focus) = current_popup.focus_item_in_parent.upgrade() {
    self.set_focus_item(&focus, true, FocusReason::PopupActivation);
}
```

so closing a picker restores focus to the `datetime` `Button` inside the row that
is about to be destroyed, running whatever focus handling that element and its
`fluent` ancestors declare — **inside the window where the old rows index the new
view's `values`**.

**Evidence.** Benign today, and checked:
`grep -n "changed \|focus" crates/goad/ui/app.slint` returns five `changed tick`
guards and no `focus-changed`/`focus-gained` handler in this project's markup, so
nothing of the host's runs there. The generated `close()` is a no-op when the
popup is not open, so the ordinary present reaches none of this.

**Scope, stated honestly.** A nit, and only forward-looking: the I-F comment's
stated rule names `init` handlers, and there is now a second way to latch the
transient that it does not name. One clause — *and nothing between these two
statements may read `root.values`* — closes it.

**Disposition:**
**Response:**
**Outcome:**

### F-B9 — the diagnostics repeater is handed a fresh `ModelRc` on every present, so every line element is destroyed and rebuilt each time the pane is up

**Severity:** nit — **read, at the vendored source**
**Raised by the orchestrator as declared raiser**, having been held back from
both round-2 briefs so that a reviewer could find it independently
(`docs/memory/dont-feed-the-raiser-your-finding.md`). The behaviour dimension
reached the surface and excluded it on a test that does not cover this consumer
— see below — so this is raised rather than confirmed, and F-R5's Outcome
records the same thing.

**Location:** `crates/goad/src/glass.rs:250-258`; `crates/goad/ui/app.slint:934`
(the repeater), `:837` (the mode gate); against
`i-slint-core-1.17.1/model.rs:719-729` and `model/repeater.rs:559-573`.

**Expected.** F-R5's class as this ledger states it: *"a property written on
every present whose slint comparison is by identity rather than content."* And
the sibling in the same function: `self.options.set_vec(rows)` (`glass.rs:236`)
is **guarded** by `if self.shown != showing` and writes into a **retained**
`VecModel`.

**Observed.** `glass.rs:256-258` runs on **every** present, unguarded:

```rust
self.window.set_diagnostic_lines(ModelRc::new(VecModel::from(lines)));
```

A fresh `ModelRc` each time. `ModelRc`'s `PartialEq` is
`core::ptr::eq` (`model.rs:719-729`) — **address, not contents**, the same
mechanism as the `SharedImageBuffer` comparison F-R5 is about. `Repeater::model`
(`model/repeater.rs:559-573`) reads:

```rust
if model.is_dirty() {
    let old_model = model.get_internal();
    let m = model.get();
    if old_model != m {
        *self.data().inner.borrow_mut() = RepeaterInner::default();   // every instance dropped
        self.data().is_dirty.set(true);
        …
    }
}
```

So while the diagnostics pane is up, each present destroys and re-instantiates
**every** line `Text`.

**Evidence.** Read at the vendored sources above, and the repeaters enumerated
rather than assumed — `grep -n "in root\.\|in field\.\|in option\." crates/goad/ui/app.slint`
returns exactly three: `root.options` (`:357`), `option.blocks` (`:401`) and
`root.diagnostic-lines` (`:934`). Of the three, **`diagnostic-lines` is the only
one handed a fresh `ModelRc`**; `options` is fed by `set_vec` into a retained
model *and* is guarded.

**Why the behaviour dimension's enumeration did not reach it, stated precisely.**
F-R5's Outcome excludes `set_values` and `set_diagnostic_lines` because
*"neither has a `ChangeTracker` behind it"*. That is **true, and it is the right
test for a `changed` handler** — which is what F-R5's tray push was. A repeater
is a second consumer of model identity and has no `ChangeTracker`; the exclusion
test does not cover it. A correct enumeration carrying a sub-claim that does not
reach one member — `docs/memory/verify-the-enumeration-not-the-conclusion.md`.

**Scope, stated honestly, and it is narrow.** The repeater is inside
`if root.mode == WindowMode.diagnostic` (`app.slint:837`), so **while the prompt
is up it has no instances** and the cost is the `Vec<SharedString>` and the
`ModelRc` alone. The rebuild is real only while a person is looking at the
diagnostics pane, and a `Text` carries no caret, selection or focus, so **no
user-visible state is destroyed** — this is not F-R1's class and it is not
claimed to be. It is CPU on the UI thread, proportional to the line count, at
every scheduled firing and every arrival, and **the fix is the sibling's own
pattern**: retain a `VecModel` and `set_vec` into it, which also removes the
parallel implementation of a thing `options` already does one way.

**Not run.** No case observes it and none was built. What would settle it is an
`init` counter on the repeated `Text` read across two presents in diagnostic
mode — the shape `inits` already uses on the prompt side.

**Disposition:**
**Response:**
**Outcome:**


## What was checked and found clean

A bare "no findings" is unusable, so this is what was checked and what would
have shown it dirty.

### Every `RefCell` borrow, and why none can be re-entered

`grep -n "RefCell\|borrow" crates/goad/src/*.rs` returns exactly one `RefCell`
in the crate: `Debounce::held` (`pending.rs:74`). Seven borrow sites:

| site | borrow | live across | verdict |
|---|---|---|---|
| `pending.rs:89` | `borrow()` | `Debug::fmt` | no site formats a `Debounce`; `SlintGlass`'s own `Debug` is `finish_non_exhaustive` (`glass.rs:96-100`) |
| `pending.rs:118` | `borrow_mut()` | `insert` of a `String` pair and a `Held` | the `RefMut` is a statement temporary, dropped at the `;` before `self.arm(wire)` on the next line |
| `pending.rs:139` | `borrow()` | `.iter().map(…).collect()` | the closure clones `String` and `Reported` only; no call into Slint, no user code |
| `pending.rs:157` | `borrow_mut()` | `clear()` | drops `String`/`Reported`; neither has a `Drop` that can re-enter |
| `pending.rs:201` | `borrow()` | the `let next = …` initialiser | **not** live across `wire.send` at `:207`: the temporary dies at the end of the `let` statement. The doc at `:195-197` claims this and it is true |
| `pending.rs:214` | `borrow_mut()` | `remove` | statement temporary inside `if enqueued` |
| `pending.rs:218` | `borrow()` | an `if` **condition** | condition temporaries are dropped before the block, so it is not live across `self.arm(wire)` |

The re-entrancy question that matters is whether any `install.rs` callback can
run while one of those is live. It cannot, for a reason that is worth writing
down because it is not obvious: **`present` does execute markup code.**
`glass.rs:227` calls `window.show()`, and `WindowInner::show` runs
`ensure_tree_instantiated()` (`window.rs:1634`), which materialises repeaters —
firing `init => { root.inits += 1; }` — and runs queued change handlers,
firing every `changed tick` guard, in a loop of up to ten passes
(`window.rs:648-665`). Slint's `changed` handlers are otherwise deferred: a
property write only queues the `ChangeTracker` on a thread-local list
(`properties/change_tracker.rs:13`, `:29-30`), which `update_timers_and_animations`
drains at `platform.rs:292`. So the guards run inside `present`, at the `show()`
line — and none of them can raise an edit callback:

- **`LineEdit`** — all six `TextInput::edited` raise sites are internal edit
  operations: key insertion (`items/text.rs:1102`), android preedit commit
  (`:1214`), selection delete (`:1707`), paste (`:1827`), undo (`:2144`), redo
  (`:2190`). An external write to `text` reaches none of them, so
  `app.slint:491` and `:644` cannot re-enter `on_edited`.
- **`CheckBox`** — `toggled` is raised only from the touch area
  (`fluent/checkbox.slint:96-101`), the space/enter key handler (`:109-115`)
  and `accessible-action-default` (`:25-30`). Assigning `checked` raises
  nothing, so `app.slint:438` is safe.
- **`Slider`** and **`ComboBox`** — argued in the markup at `app.slint:557-562`
  and `:694-697` with citations, and both hold.
- The `datetime` `Button` carries no guard (`app.slint:736-741`).

Had any of those raised its callback, the consequence would not have been a
panic — no borrow is live at `show()` — but a **phantom edit**: the host's own
correction re-entering `Debounce::hold` and being sent to the backend as
something a person did. That is what was being looked for and it is not there.

`Rc` handles: one `Rc<Debounce>` (`main.rs:95`), cloned into the two callbacks
that use it (`install.rs:39`, `:65`) and into the glass (`main.rs:107`) — R10
holds, one value and no second map. One `Rc<VecModel<OptionRow>>`
(`main.rs:106`), held only by the glass. One `Rc<Cell<Zoom>>`
(`install.rs:115`), shared by the three zoom callbacks. No `Rc` is downgraded
anywhere; the crate's only `Weak` is F-R8's.

### Growth, and where it was looked for

- **The retained `options` model.** `set_vec` replaces the vector and calls
  `notify.reset()` (`model.rs:404-407`), which clears `tracked_rows`
  (`model/model_peer.rs:87-96`) — nothing accumulates per view. Re-handing the
  same `Rc` through `ModelRc::from` on every view change (`glass.rs:191-193`)
  re-attaches the repeater's peer, and `attach_peer` →
  `DependencyListHead::append` begins with `node.remove()`
  (`properties.rs:189-191`), so the peer list does not grow. The node itself is
  a per-repeater `OnceCell` (`model_peer.rs:137`, `:158-170`), one per
  repeater, not one per attach.
- **The per-present models.** `values`, `diagnostic_lines`, the block/field
  models and each `choice`'s alternatives are allocated fresh each present
  (`glass.rs:187`, `:210-212`, `:345-351`, `:359`, `:366`, `:604-606`) and the
  previous ones are dropped by the property write. Churn, not growth — see
  F-R4 for the churn.
- **Timers.** There is exactly one `slint::Timer` in the crate
  (`pending.rs:75`). `arm` re-uses its id through `start_or_restart_timer`
  (`timers.rs:86-92`, `:347-370`), so a burst of keystrokes produces one slab
  entry, not one per keystroke. Re-arming from inside the tick is sound and for
  the reason `pending.rs:161-168` gives — with one correction to the citation:
  the `SingleShot` removal path at `timers.rs:320-325` is **not** the path this
  code takes, because `Timer::start` boxes every callback as
  `CallbackVariant::MultiFire` whatever the `TimerMode` (`timers.rs:86-91`);
  only the free `Timer::single_shot` uses `SingleShot` (`:111-121`). The
  MultiFire re-emplacement logic the doc quotes is the right one, and it holds.
  The cleanup gap is F-R6, not a rearm bug.
- **Popups.** `active_popups` does not accumulate through the ordinary path:
  `show_popup` closes sibling popups under the same parent item before creating
  a new one (`window.rs:1781-1786`), and both pickers call `root.close()`
  **before** raising `accepted` or `canceled`
  (`fluent/datepicker.slint:84-85`, `:94-95`;
  `fluent/time-picker.slint:91-92`, `:101-102`), so the date picker is gone
  before `app.slint:950` shows the time picker. The popup that is *never*
  closed is F-R1's, and it is F-R1's.
- **Diagnostics.** `Diagnostics::lines` is a `Vec<String>` rebuilt per exchange
  and replaced wholesale by `absorb` (`controller.rs:194`), not appended to;
  every push site is bounded by one report's content
  (`diagnostics.rs:127-151`) and each line is length-capped by `finish`.
- **The window after `hide()`.** `SlintGlass` holds `window` and `tray` as
  strong clones for the life of the process by design (`glass.rs:70-71`,
  `main.rs:103-108`), and `hide()` releases the platform keepalive it took
  (`window.rs:1656-1663` against `:1626-1632`), so the pair is balanced. Hiding
  with nothing shown clears the row model (`glass.rs:188-195` with
  `showing = None`), so a hidden window is not holding a dead view's elements.

### Ordering between the three schedulers

The three cannot interleave *within* a step: the Slint callbacks and the
`Debounce` tick both run to completion on the UI thread with no await, and
`serve` is a `spawn_local` task on the same thread. Within one event-loop
iteration the order is fixed and readable —
`update_timers_and_animations` runs `maybe_activate_timers` and then
`run_change_handlers` (`platform.rs:289-293`), so a tick always precedes the
guard pass it could affect. `delivered()` clearing the whole map
(`pending.rs:157`) is sound for the reason its doc gives: `carried()` and
`delivered()` are two statements of one synchronous callback
(`install.rs:41-48`), and nothing can be inserted between them.

What is **not** ordered is the gap between the tick's enqueue and `serve`'s
dequeue, which is F-R3 — the one place where a present, a tick and the serve
loop see three different answers for the same field.

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

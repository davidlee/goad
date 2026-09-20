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
| F-A1 | blocker | fix-now | |
| F-S2 | major | fix-now | |
| F-S1 | major | fix-now | |
| F-P1 | minor | doc-wrong | |
| F-P2 | minor | doc-wrong | |
| F-R1 | major | fix-now | |
| F-R2 | major | fix-now | |
| F-S3 | major | fix-now | |
| F-R3 | major | fix-now | |
| F-S4 | minor | fix-now | |
| F-S5 | minor | fix-now | |
| F-R4 | minor | fix-now | |
| F-R5 | minor | fix-now | |
| F-R6 | minor | doc-wrong | |
| F-R7 | minor | doc-wrong | |
| F-P3 | nit | fix-now | |
| F-P4 | nit | fix-now | |
| F-S6 | nit | fix-now | |
| F-S7 | nit | *settle first* | |
| F-R8 | nit | fix-now | |
| F-R9 | nit | fix-now | |

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

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**


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

**Outcome:**

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

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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

**Outcome:**

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

**Disposition:** `fix-now`
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

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
**Response:**

**Outcome:**

### Note — `pending.rs`'s re-arm argument cites the wrong `timers.rs` arm

Not raised as a finding: the conclusion is right and the behaviour holds. But
`pending.rs:161-168` rests its re-arm argument on `timers.rs`'s `SingleShot`
path, and that is not the path this code takes — `Timer::start` boxes every
callback as `MultiFire` regardless of the `TimerMode` it is given
(`timers.rs:86-91`). The re-emplacement logic the argument depends on is the
`MultiFire` one, and it holds. Recorded here so the citation can be corrected
with the rest of the record rather than re-derived by the next reader.

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

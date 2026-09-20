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
| F-A1 | blocker | | |
| F-S2 | major | | |
| F-S1 | major | | |
| F-P1 | minor | | |
| F-P2 | minor | | |
| F-R1 | major | | |
| F-R2 | major | | |
| F-S3 | major | | |
| F-P1 | minor | | |
| F-P2 | minor | | |
| F-S4 | minor | | |
| F-S5 | minor | | |
| F-P3 | nit | | |
| F-P4 | nit | | |
| F-S6 | nit | | |
| F-S7 | nit | | |

**Round 1 closed with three dimensions reported.** Eleven findings: one
blocker, five majors, four minors — plus four nits. F-S1 and F-S2 were
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

**Relation to the record.** `design.md` **A-6** — *"Disabling a widget while an
exchange is in flight does not destroy it … being wrong costs focus, not
data"* — is the assumption this falsifies. `notes.md`'s VH-1 block already
records the drag half and prices the class as *"a one-frame flash of the whole
form when idle, a dead drag when a gesture is in flight."* The flash is not one
frame and the loss is not only a drag: it is an exchange-long deafness, and
AC-4 is inside its blast radius.

**Disposition:**
**Response:**

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

**Disposition:**
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

**Disposition:**
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

**Disposition:**
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

**Disposition:**
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

**Disposition:**
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

**Disposition:**
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

**Disposition:**
**Response:**

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

**Why this matters to the repair the user chose.** Narrowing `busy` fixes sites
1–4 and 7. It does **not** fix 5 or 6 on its own: 5 is a missing `enabled` gate
on a popup item inside `std-widgets`, and 6 is a binding this project never
wrote. Both have to be answered explicitly or the class is fixed in name only.

**Disposition:**
**Response:**

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

**Note for disposition.** AC-7 says the mechanism must *survive*. It does. What
F-S3 establishes is that nothing would report its removal — which is the same
shape as F-S2, one level up.

**Disposition:**
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

**Disposition:**
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

**Evidence.** In any one guard, hoist the assignment out of the conditional and
leave the increment inside. Every widget is then written on every present — the
caret destroyed on every tray check, which is AC-5's second clause — while
`reasserts` stays `0`, so `reassert.rs`, `overlay.rs` and `numeric_guard.rs` all
stay green. No tier observes the caret (D-10 assigns it to AC-10's human half),
so nothing else catches it. **Exposure grows with each control a future slice
adds.**

**Disposition:**
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

**Disposition:**
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

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

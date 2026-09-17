# Design log — Slice 009

Append-only, time-ordered. What was asked, what the user decided, why. Never
rewritten; superseded. Findings live in a ledger, not here.

## 2026-09-16 — scoping

**D-1 — Close 008 before opening this.** Asked because 008 was still
`Stage: in progress` with an open list. Decision: close it, carry the remainder
into its follow-ups and the roadmap, and open 009 against a clean record.
Done at `239e0c5`.

**D-2 — Draw all four undrawn kinds, not three.** The ask named datetime,
number/slider and dropdown; `text` is the fourth. Decision: all four, which
discharges the standing hazard slice 002 recorded rather than leaving a gap with
no argument behind it. `Undrawn::FieldForm` stays as the guard for a sixth kind.

**D-3 — Tier 2, on size rather than canon.** Two canon candidates were offered
and the question was declined at the time it was asked, on the ground that the
approach was not yet clear enough to size — *"I don't think we have enough to
know the tier size yet."* Reaffirmed after the spike. Neither `step` nor
SPEC-001/OQ-4 is taken; the tier comes from the design surface. `slice-009.md`
§Why tier 2.

**D-4 — A configurable per-backend text debounce, 150 ms for now.** User:
*"I expect we ultimately want a configurable debounce for text fields per
backend, but we can roll with eg 150ms for now."* So the value is a starting
point and not a constant to be argued over, and the configuration surface is
explicitly deferred rather than forgotten. What design still owes is the flush
point — OQ-3 — because a debounce means the draft lags the widget.

**D-5 — Spike the unknowns rather than design against them.** User: *"let's
spike out the unknowns rather than dive into design nitpicking."* Taken at the
point where the sizing question could not be answered from reading alone. It
overturned the framing it was built to confirm: the re-present problem had been
priced as a second design surface and is one decision. `research.md` Thread 3
carries what it measured, Thread 4 what it ruled out.

### A correction recorded, because it changed the slice's shape

The scoping agent framed "the form must survive a present" as a problem of
**value** persistence across four field kinds. The user rejected the premise —
*"it doesn't survive. It gets regenerated. If it's supposed to persist, the
backend passes it back as a value. What am I missing?"* — and was right:
`glass.rs::option_rows` builds every row from the draft, so values already
survive, and persistence across a round trip is `field.value`/OQ-2 and the
backend's.

What is actually lost is **interaction state** — caret, drag grab — and only for
`text` and `number`. The reframing is what made the problem small enough to
spike, and the spike is what made it one decision.

## 2026-09-16 — design

**D-6 — As-drawn is what the widget shows, and `datetime`'s is the epoch.**
OQ-1. Asked because `R-58` forbids omitting a value for a drawn field and
`SPEC-001/OQ-2` (`field.value`) is unlanded, so the host must invent one for
four kinds it has never drawn. Criterion put and accepted: **the as-drawn value
is whatever the widget shows before anyone touches it**, so a submitted value
never contradicts the screen. That settles four kinds by consequence —
`boolean` `false`, `text` `""`, `number` its minimum when bounded and `0` when
not, `choice` its first alternative's id.

`datetime` has no neutral: a picker is a button carrying a value and there is
no "shows nothing" instant. Two answers were offered — the epoch, or the
instant the view arrived. User: *"makes sense, and better a sentinel value
(epoch) than a misleading one"*. So an untouched `datetime` submits
`1970-01-01T00:00:00Z` and the button reads *not set*; it is the one kind whose
display distinguishes as-drawn from picked, because the epoch is not a point
anyone would pick.

Recorded against a rule the backend was already told: `R-58` says a backend
needing *unanswered* distinguishable from *false* must not send the field.

**D-7 — A `datetime` submits the offset the person picked in, and cancel
cancels.** OQ-2. Both pickers are popups, so neither can be repeated
(`research.md` Thread 3, measured); both are root singletons and the row
records which field opened one. The wire fork put was the picked local offset
against always-UTC, on the ground that `R-57` says "carrying **an offset**" and
a bare instant would not have needed the words. User: *"local time zone from
system, abandon, don't lie."*

So: the offset is resolved from `jiff::tz::TimeZone::system()` at pick time —
infallible, it falls back to UTC (`tz/timezone.rs:325`) — and `Edited` carries
the instant **and** the offset, because `draft.rs::submitted` is pure and may
not read the zone itself. `Timestamp::display_with_offset` is the one call that
renders it (`timestamp.rs:2212`).

One button per field, reading the composed value or *not set*. Date picker,
then time picker, then **one** `edited` — the draft never holds half a
datetime. **Cancel at either step abandons the whole edit**; "commit the date
at local midnight" was offered and refused as a lie about what cancel means.

**Evidence for `SPEC-001/OQ-4`, which `docs/roadmap.md` asked this slice for.**
A date-only field *can* be expressed — a `datetime` at 00:00 local — so OQ-4
stays shut, as scoping held. The cost this slice measures is the affordance: a
backend wanting only a date makes the person walk a time picker to get there.

**D-8 — The debounce flushes when the draft becomes an answer, and nowhere
else.** OQ-3. User: *"on answer is fine"*, against `accepted` and focus loss
also carrying weight.

The argument accepted was generative rather than a list: `accepted` and focus
loss are guesses about how a person leaves a field and neither is guaranteed to
happen before the option button is clicked, which is the loss `D-4` named. The
`chosen` callback flushing a pending edit before `Choose` goes down the wire is
by construction after everything the person did.

Two facts reported at the same time, neither reopening `D-4`:

- **The spike moved the debounce's reason.** It existed because a keystroke
  rebuilt the form and ate the caret; with a guarded re-assert that is no
  longer so. What remains is the capacity-1 channel — fast typing can hit
  `Full` and raise the back-pressure notice at a person mid-word — and a row
  rebuild per keystroke. The debounce stays, for the right reason.
- **`number` needs no debounce.** `Slider` fires `released` once at the end of
  a drag as well as `changed` throughout (`widgets/fluent/slider.slint:16-17`),
  so binding `released` alone removes the flood with no data loss. This is not
  Thread 4's rejected commit-only binding: that failed because
  `LineEdit::accepted` fires on Enter only. The debounce is `text`-only.

Two costs stated rather than discovered: the capacity-1 channel can still
refuse the `Choose` that follows a flush — visible through the back-pressure
notice, with the text already recorded, so a second click answers correctly —
and a pending edit landing after its view was replaced is refused
`SupersededView` and reported, which is right, because the typing really was
thrown away.

**D-9 — Structure and values become two channels; a present rebuilds only on a
new view.** OQ-5. User: *"yeah, accept"*.

Three parts, taken together:

1. **Thread 4's unverified claim is false, and on mechanism.** The claim —
   that divergence is always observable to the host — dies on `wire.rs:128-132`:
   `send` raises the back-pressure notice on `Full` and **lowers it on the next
   successful send**, and `serve` samples it once at present time
   (`controller.rs:739`). So it is a level signal about the last send, not a
   record of which edit was lost: refuse field A, land field B, and the next
   present sees a clean notice while A's widget is permanently wrong. A second,
   independent kill: even knowing *that* something was refused, the host does
   not know *which* widget, so its only response is a full rebuild — destroying
   the caret this slice exists to protect. Verified generatively rather than by
   completing the enumeration `docs/memory/enumerate-the-class-not-the-instances.md`
   warns about. **The measured guard stays.**

2. **The rule: write in place exactly when the present shows the same
   `view_id` as the one before it; rebuild otherwise.** Not Thread 4's rejected
   comparison, which compared row values and *skipped the write* — this always
   writes and only chooses the vehicle; correcting a diverged widget stays the
   re-assert's job. `view_id` is the right key by a property of the types:
   `State::issue` mints a fresh id with a strictly increasing sequence for every
   view (`goad-shell/src/state.rs:76`), and `Prepared.presentation` is built
   once (`reception.rs:75`) and never mutated afterwards (`controller.rs:277`
   writes only the draft). Same view id ⇒ identical structure, so no structural
   diff is needed or wanted.

3. **Two channels rather than one retained tree.** Writing a value into
   today's nested model (`VecModel<OptionRow>` → `blocks` → `fields`) in place
   would need the glass to retain the whole tree of handles — a nested cache
   with an invalidation rule, which `option_rows`'s own doc boasts of not
   having. Instead the row model carries **structure only** and is written only
   when the view changes, and the draft travels in a flat `values` property
   indexed by a slot each row carries. Nothing repeats over `values`, so
   rewriting it wholesale every present destroys no element, and
   `Glass::present` stays total.

   The gain beyond mechanism: it draws the same line at the markup boundary
   that `view_model.rs` and `draft.rs` already draw. `PresentationField`'s doc
   says the person's state "is **not** here and never will be", and
   `glass.rs::field_block` folds the draft into the row anyway. This restores
   the separation instead of caching around its absence.

Two costs accepted: the models must stay in step, held by building both in one
pass so the slot *is* the index by construction; and `root.values[field.slot]`
indexing from inside a repeater is to be **measured**, not assumed — the spike
is `git checkout a698217 -- spike-fields` away.

**D-10 — One new event-loop binary for the re-assert; the caret is a person's.**
OQ-4. User: *"yeah, seems right"*.

`D-8` has a side effect that shrinks the loop tier: flush-on-answer makes the
typed path **synchronously** observable with no event loop, because `chosen`
drains the pending edit itself. So `set_accessible_value` then a click on the
option button produces `Edit` then `Choose` with no timer
(`i-slint-backend-testing/search_api.rs:637`).

The split accepted:

| tier | holds |
|---|---|
| `tests/renderer/`, no loop | AC-1 structure and declared order, AC-7 `Undrawn`, AC-2 / AC-3 wire types and `R-58`, AC-8 alternative ids, AC-9 unbounded number, and the answer flush carrying pending text |
| **one new `[[test]]` binary**, one `#[test]` fn, the `event_loop_schedule` shape | AC-5 and AC-6 — both halves of the epoch re-assert, which is one mechanism — and the debounce timer firing, which nothing else observes |
| a person, AC-10 | the caret surviving a present mid-word; both pickers; a slider drag across a present |

The loop test builds the real `PromptWindow` and `SlintGlass` directly, as
`tests/renderer/` already does, rather than driving `serve`: **clicking a
checkbox with no wire installed is the dropped-edit case exactly**, with no
channel to wedge. Negative-controlled by deleting the re-assert, and the
control is compiled and run before the red is believed
(`docs/memory/a-negative-control-that-does-not-compile.md`).

**The instrument costs two properties in production markup** — an `init` count
per field widget and a second count inside the re-assert's guard — because
element identity is not observable from outside and the pair is what separates
*not rebuilt*, *no write needed* and *written back*. Accepted against the
alternative, a test-only copy of the field markup, which would be a parallel
implementation of the thing under test. Precedent: `Tray::shown` exists solely
so `visible` is a binding rather than a folded constant, and says so.

**The limit, written down rather than discovered:** `LineEdit` exposes no
readable cursor offset, so **no tier can assert the caret.** The loop test
asserts the element was not destroyed, which is the *cause*; the caret itself
is a person's observation under AC-10.

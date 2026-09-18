# Design log — Slice 009

Append-only, time-ordered. What was asked, what the user decided, why. Never
rewritten; superseded. Findings live in a ledger, not here.

**`D-n` here is not `Dn` in `design.md` §7.** Two sequences, one hyphen apart,
and from D-18 they overlap on adjacent subject matter — this file's D-21 is
about handover, §7's D21 is the picker seed. Cite the file with the id, always.
§7 is current truth and rewrites an entry in place under its own id; this file
never rewrites one and supersedes instead.

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

## 2026-09-17 — design review, round 1

Findings and their dispositions live in `review-design.md`. Only what the user
decided is here, each citing the finding that prompted it.

**D-11 — The host will read the system time zone, and the slice argues the
manifest change that makes it possible.** F-1. The reviewer found that `jiff`
is declared `default-features = false` across the workspace and no member adds
anything back, so it resolves with **no features at all**: `TimeZone::try_system`
compiles to `Err(CrateFeatureError::TzSystem)` and `TimeZone::system` silently
returns `Etc/Unknown`, which behaves as UTC. The `warn!` it emits on that path
is suppressed too, because `logging` is off. So D-7's *"local time zone from
system"* would have submitted `+00:00` for every pick, everywhere — the lie D-7
refused, arrived at by a manifest rather than by a decision.

Four answers were put. The user took the first: **enable `tz-system` (and
`tzdb-zoneinfo`, which it needs to resolve a zone) on the `jiff` entry
`crates/goad` inherits, and argue it.** `tz-system = ["std", "dep:windows-link"]`,
so this switches `std` and `alloc` on in a dependency `goad-semantics` shares.
`docs/policy/001-the-phase-gate.md` §Verification names exactly this as **the
residue**: a feature switched on by stratum 2 or 3 unifies into stratum 1's
build under `--workspace`, no gate command rejects it, and it is therefore a
design decision argued in the slice that takes it. `design.md` §10 carries the
argument. `cargo test -p goad-semantics` is unaffected: it builds stratum 1 with
stratum 1's own feature set, which does not change.

Rejected: always-UTC (D-7's lie, and D-7 is settled); resolving the offset
outside jiff (a second time implementation beside the one already depended on);
deferring `datetime` (leaves `R-55`'s subset undischarged and moves
`SPEC-001/OQ-4`'s evidence to a later slice).

**D-12 — A number crosses the markup boundary as a string, and the control is
a decision the host carries rather than one the markup infers.** F-2, and the
user's own question on top of it.

The finding: Slint's `float` is `f32` and `NumberRange` holds `f64`, so §5.2's
first draft ran the bounds, the displayed value **and the submitted value**
through a 32-bit channel. A legal bound of `1e100` becomes infinity there. That
is `CLAUDE.md`'s third invariant — *do not narrow wire compatibility because the
current renderer implements a subset* — failing through a type rather than
through a decision.

Accepted: the submitted number crosses as a **string**, parsed host-side to
`f64`; `FieldValue`'s `float` slot survives only as the guard's comparand and
the `Slider`'s own value, neither of which reaches the wire.

Then the user's question, which improved the repair: *"it does beg the question
whether a non-slider (text) float is compatible with the approach here? I'd
like to support slider vs text field as an independent-ish decision from the
field type."* It is not only compatible — the text control is the **lossless**
one, and the `Slider` is the control with an admissibility condition, because
Slint gives its `value`, `minimum` and `maximum` no more than `f32`.

That inverted the framing. The first draft had the markup infer the control
from a fact about the field (`bounded: bool, // bounded = draw a Slider`), and
the proposed repair would have stacked a second derivation on top of it.
Decided instead: **the row carries the decision, not the fact.** `slider: bool`
replaces `bounded: bool`, the markup obeys it rather than reasoning from it, and
one named function in `view_model.rs` decides it. Today that function reads only
the bounds — both present, and both round-tripping `f64`→`f32`→`f64` exactly.
Later it may read a hint (`R-18` permits the renderer and only the renderer to
branch on one, and `multiline` is the existing precedent), or a configuration,
or nothing; only its body changes.

Two things stay welded, deliberately. `FieldEdit.kind` carries the **protocol**
kind and never the control, so `draft.rs::submitted` remains the single
application of `R-57` (I-C). And both controls send the same string-valued
edit, so there is one parse rather than one per widget.

Reading a control hint **now** was offered and declined: it is scope
`slice-009.md` excludes, and no evidence has asked for it.

**D-13 — A DST fold or gap resolves under jiff's `Compatible` disambiguation,
and the design says so.** F-4. `DateTime::to_zoned` does not fail on an
ambiguous or nonexistent civil time: a fold takes the earlier occurrence and a
gap shifts forward. So `2024-03-10 02:30` in New York composes to `03:30-04:00`
and **succeeds**, which the `None`/visible-no-op treatment does not cover.

Accepted: keep `Compatible`, state it as a named edge, and rely on the button
showing the composed value — the person sees the shifted time rather than being
deceived by it, and the instant reaches the backend with its offset either way.
Rejected: refusing an ambiguous pick, which leaves a person inside a fold
unable to express 01:30 at all and has no affordance to explain the refusal;
and splitting fold from gap, which is two behaviours to explain with no evidence
asking for the distinction.

**D-14 — The remaining fifteen findings, taken as dispositioned.** F-3, F-5,
F-6, F-9, F-10, F-11, F-13, F-14, F-15, F-16, F-17 and F-18 `fix-now`; F-7, F-8
and F-12 `doc-wrong`. User: *"Take them as proposed."* Nothing tolerated,
nothing deferred, no follow-up. Two consequences are worth naming here because
they touch earlier decisions:

- **D-8's flush point stands; one fact reported beside it does not.** D-8
  decided *where* the debounce flushes — on answer, and nowhere else — and that
  is untouched. Reported alongside it was that *"`number` needs no debounce"*,
  on the ground that `Slider` fires `released` once at the end of a drag. F-9
  falsifies that ground's sufficiency, not its accuracy: `released` is raised by
  the pointer and keyboard paths, but Slint's accessibility `set-value`,
  `increment` and `decrement` actions call `set-value`, which raises **`changed`
  only**. A slider bound to `released` alone therefore never hears an assistive
  technology, and — F-13 — cannot be operated by the test tier either. So the
  `Slider` binds `changed` as well, which reintroduces the flood the debounce
  exists to stop, and the debounce stops being text-only. F-5 was the same
  defect already visible as a contradiction inside §5.2.
- **D-10's "one new binary, one test fn" does not survive F-11 and F-13.** The
  constraint behind it is real and unchanged —
  `docs/memory/slint-testing-backend-initialises-once-per-process.md`: one
  event-loop *arrangement*, one `[[test]]` target. What F-13 adds is that the
  no-loop tier cannot operate a `ComboBox` or reach inside a picker popup at
  all, so cases D-10 placed there have nowhere to run. §9 now names a driver
  per row, which is the check D-10 was missing rather than a reversal of it.

## 2026-09-17 — design review, round 2

Round 2 set a terminal outcome on F-1 … F-18 and raised F-19 … F-27; the
responder added F-28 and F-29 against its own repairs. Twelve verified, five
contested, one withdrawn. What the user decided:

**D-15 — `Command::Choose` carries the pending edits.** F-6, and it is the
finding that mattered most, because it falsifies a mechanism *every* version of
this design has described.

The command channel is capacity 1 (`main.rs:86`) and `serve` shares the UI
thread through `slint::spawn_local`. A Slint callback is synchronous and
contains no await, so `serve` cannot drain between two sends made inside one.
Flushing *N* pending edits and then `Choose` needs *N+1* slots and gets one:
**the second `try_send` of any flush always fails.** §5.4 described that as the
case where "the second `try_send` comes back `Full`", as though it were an edge;
it is the only case. This was already true before round 1's repairs — the
pre-repair design flushed one edit and then `Choose`, which is two sends.

Accepted: `Command::Choose` carries the pending edits with it. One send, no
race, and FIFO stops being load-bearing. The controller applies them to the
draft and then answers, so D-8's decision — *the debounce flushes when the draft
becomes an answer* — is held **by construction** rather than by sequencing,
which is what it always meant. It costs a change to `Choose`'s shape and to the
tests that build one.

Rejected: raising the channel capacity, because capacity 1 is load-bearing
elsewhere — it is what produces the back-pressure notice — and changing it is a
decision about a different subsystem taken for this one's convenience. And
reopening D-8's flush point, whose original argument against focus-loss stands
untouched.

**D-16 — The host parses a number the way the control validated it.** F-26.
`input-type: decimal` validates through Slint's **locale-aware** `string_to_float`,
which substitutes the locale decimal separator before parsing, while the host
was specified to call `f64::from_str`, which accepts `.` and not `,`. In a
comma-decimal locale the control approves `1,5` and the host refuses it, then
re-asserts over the person.

Accepted: parse with the rule the text was validated under — a single comma
taken as the decimal separator where no dot is present, then `f64::from_str`.
Host-side, testable without a locale fixture, and numeric formatting rather than
anything domain-shaped. Rejected: sending Slint's parsed float alongside the
text, which reintroduces as a fallback the `f32` path F-2 exists to remove; and
`settle-in-code`, which the ledger's protocol allows but which is for questions
that are unsettled, not for correctness questions with a known answer.

**D-17 — The remaining thirteen, taken as dispositioned.** F-10, F-11, F-14,
F-15, F-19 … F-25, F-27, F-28 and F-29, all `fix-now`. User: *"Take them as
proposed."* Three are worth naming because they are not text edits:

- **F-21 makes an invariant unrepresentable instead of disciplined.**
  `Edited::Adjusted` comes to hold a checked finite value rather than a bare
  `f64`, so a non-finite number — which `serde_json` would serialise as `null`,
  and `R-57` does not admit — stops being something a convention prevents.
  `CLAUDE.md`'s *"internal representations are canonical"* read literally.
- **F-13 was wrong, and §9 was built on it.** Popups *are* reachable under
  `init_no_event_loop`: the testing crate's own `test_popups` does it, and
  `ElementQuery::from_root` traverses `active_popups`. The finding inferred a
  missing capability from no case in this repository using one. §9 is rebuilt on
  what the API actually reaches, naming a concrete driver per row (F-25).
- **Two of the responder's own arguments were wrong** and are corrected rather
  than defended. D8's *"the host does not know which widget diverged"* is false —
  `try_send` returns the refused `Command::Edit`, which names the field, and
  `wire.rs` discards it deliberately (F-23). And §10's claim that
  `cargo test -p goad-semantics` would catch a purity regression contradicts
  `POL-001`, which says that command **rejects nothing** (F-24). Neither
  decision changes; both arguments do.

## 2026-09-17 — round 2's integration, dispositioned

**D-18 — The host holds the text a person typed, beside the number it means.**
F-30, and the guard's comparand at its third attempt. The first two were both
reasoned and both wrong — `f32`-collapsing (F-19), then text against a
re-format of the `f64` (F-30) — so this one was put up as options and then
**measured** before being taken, which was the user's condition: *"that's on
the assumption B has been priced with enough care to rule it out. A pet peeve
is discarding more correct solutions on the basis of assumed cost."*

Two candidates were priced against the code rather than estimated.

- **A, taken.** `Edited::Adjusted` carries the as-typed text beside the
  `Finite`; the guard compares string against string, which is an identity.
- **B, rejected on evidence, not on cost.** Converging on recency — a per-slot
  revision the host bumps — deletes the comparand rather than correcting it,
  and is the better shape in the abstract. The first pricing of it was too
  casual and was corrected: a canonical `Field` carries no value
  (`canonical.rs:220-225`), so a drawn field's value moves only through
  `record()`, and B needs neither retained values in `glass` nor a
  just-edited field on the `Frame` — one edge-held signal in the shape
  `wire::Notice` already has. It was then measured available
  (`spike-fields/tests/revision.rs`). What rules it out is its own behaviour:
  `-` and `1e400` are edits the host **cannot record**, which is exactly when a
  revision guard converges, so it writes over them unless the host holds the
  text anyway. Once the host holds the text there is no comparand left to get
  wrong.

So A is not the cheaper option; it is the one the control's own validation
forces. B stays available if the cleared-field exception ever grows.

**D-19 — Spike first, and keep the spike in history.** User: *"spike anything
that's likely to answer questions more economically than design nitpicking"*,
and *"keep (commit) the spike, we can delete it once we're done with it but
keep it in history."* Committed at `4f93d41` rather than left on disk. Three
tests, each injection-passed. It refuted a blocker (F-31), found two defects
nobody had raised (F-35, F-36), and shrank a third (F-33) — more than it was
asked.

**D-20 — F-30 … F-33 taken as dispositioned, and F-31 withdrawn.** User:
*"confirm those dispositions."* F-31 was a blocker and is wrong: no
`PopupWindow` state survives a close, so the stale-picker case it describes
cannot arise. F-34 … F-36 were raised in the same pass and carry the same
`fix-now`.

**D-21 — Hand over before integrating, rather than delegate.** User:
*"rather than use a subagent for integration it's probably about finding the
right time to handover to a fresh agent."* Same rule as round 2 — the session
that wrote a disposition does not integrate it — discharged by ending the
session at the ledger rather than by spawning a subagent underneath it.

## 2026-09-18 — round 2's integration, applied

**D-22 — What a widget reported and what the draft holds are two types.** F-37,
raised while integrating F-30: `Command::Edit` and `PendingEdit` carry an
`Edited`, and two of its five variants cannot be built where the command is
built. `Chosen` holds an `AlternativeId`, which can only be cloned off a retained
view — the mechanism §7 D12 rests AC-8 on — and F-30's repair adds the second,
because a number's text that does not parse finitely must keep the number the
field already holds and only the draft and the drawn kind know what that is.

Three answers were priced against the code. Taken: a `Reported` enum for what a
widget can say, one kind-directed `resolve` beside `as_drawn`, and `Edited` left
as exactly what the draft holds. Rejected: partial payloads inside `Edited`,
which give `submitted` arms for states the draft is promised never to hold; and
resolving at submit time in `answer`, which loses F-34's rule because `1e400`
reparses to infinity and falls back to as-drawn. User: *"Split the type
(recommended)"*.

## 2026-09-18 — design review, round 3

**D-23 — The value channel shows the draft overlaid with what `pending.rs`
holds.** F-40, the blocker round 3 found underneath round 2's pending map. The
guard cannot tell *the host did not record this edit* from *the host has not
recorded it yet*, and the debounce is what created the second; the serve loop
presents before every command, so any present inside the window writes the stale
draft value back over a person mid-type.

Three answers were put up. Taken: `glass.rs` takes a handle to the same `Rc` the
callbacks hold and prefers a pending entry over the draft's value, so a present
inside the window writes back what the person typed and the guard is quiet.
AC-6 keeps its meaning and gains precision — a widget converges exactly when the
host holds neither a draft value nor a pending one. Rejected: a per-field
suppression flag in `FieldValue`, which spells the same information as *do not
converge* rather than as *this is the value* and leaves the channel and the
widget disagreeing on purpose. Also rejected, and offered explicitly because it
deletes the class rather than answering it: dropping the debounce, which is D-4
and the user's own standing commitment. User: *"Overlay pending on the draft
(recommended)"*.

Two things the user's own framing pinned: the overlay probably subsumes §5.2's
cleared-field exception, and the exception stays until `numeric_guard.rs` is
re-run against it — this comparand has been wrong three times and twice on
reasoning. And §5.3's "no cache and therefore no invalidation rule" has to be
restated rather than left quietly false.

**D-24 — The other twelve of round 3's findings taken as dispositioned.** User:
*"Confirm all twelve (recommended)"*. F-6, F-20, F-26, F-29, F-32, F-38, F-39,
F-41, F-42, F-43, F-44, F-45, each with its repair and its rejected alternatives
written out in `review-design.md`. Four of the five contests were upheld on
evidence verified by hand this session; two of them, F-32 and F-6, were the
previous integration's own errors.

## 2026-09-18 — round 3's integration, applied

**D-25 — F-46 … F-49 taken as dispositioned.** User: *"yes, confirm them."* The
four the integration raised by writing round 3's repairs down. Three are
corrections to round 3's own dispositions: F-46, that F-43's replacement ordering
cannot exist because `answer` is `&self` over one option; F-47, that F-40's
overlay stops AC-6's stated driver reaching the dropped-edit case, so the row
would have been written green; F-48, that a float inside `Command` costs the `Eq`
derive and the repair that compiles is unsound. F-49 is the overlay's own
vacuous-pass hazard — two halves holding different `Rc`s — and is carried by §5.3,
§8 R10 and §9 together.

The confirmation ran **after** the integration rather than before it, which is the
opposite of the protocol's order. Recorded rather than smoothed over: all four
were found by writing the repair, so there was no moment at which they existed
and the repair did not.

**D-26 — round 4 waits on a prototype.** User: *"A fresh agent will run round 4
after reviewing feedback from a prototype worktree."* So the design does not go
back to a reviewer on the strength of the text alone. Round 4's brief is not
written, and is not to be written until that feedback is in hand — it is the
brief's job to name the surfaces the prototype found, and a brief written now
would name only the ones the integration did. This is D-19's rule (spike anything
a spike can answer) applied to a whole review round rather than to one question.

## 2026-09-18 — the prototype's first report

**D-27 — the prototype's findings are raised in one batch, after P1b.** User:
*"Batch after P1b (recommended)"*. P1a reported eight — P-1 … P-8 in
`prototype-notes.md` on `slice-009-prototype` at `dc30a2a`, which is the artefact;
four of them reached this session as a summary. Five land in §5.2, and P1b is
running the overlay and the timer, so its findings will land in §5.3 and §5.4
instead. Raising P1a's now would rewrite §5.2 once for P1a and again for P1b, and
pay the integration-is-review tax — the one D-21 exists for — twice over the same
text. Nothing is at risk of being lost while they wait: the prototype's record is
committed on its own branch.

The ids are the prototype's, not the ledger's. They become `F-50` onward when the
raise happens, each citing its `P-n`, and the `P-n` entries stay where they are —
they carry what the build observed, which a ledger finding states but does not
hold.

**D-28 — round 4 is raised by a fresh Claude agent.** User: *"Fresh Claude agent
(recommended)"*, asked because Codex is out of credits. The ledger's protocol asks
for a fresh **raiser**, not a fresh model, and an agent that has never seen this
slice satisfies it. Rounds 1-3 were all `gpt-5.6-sol`, so this cuts both ways and
the ledger should say so when round 4 opens: a different model has a different set
of blind spots, and may find a class the Codex rounds never looked at as easily as
it may miss one they would have caught. Waiting for credits was rejected — the
slice would sit on a billing cycle — and so was running both in sequence, because
a fifth round needs its own justification against a measured trend rather than a
second opinion's availability (`docs/memory/review-rounds-stop-on-a-measured-trend.md`).

## 2026-09-18 — the prototype's handback

The prototype stopped after `text` and handed back
`prototype-handback.md` on `slice-009-prototype` (`a1171b3`), indexing fifteen
findings; `number`, `choice` and `datetime` are not built, and §9's validation
table was never attempted. Three decisions came out of reading it.

**D-29 — the prototype's findings enter through this log, not the ledger.**
User: *"design-log decisions citing each P-n (recommended)"*. This **supersedes
D-27's second paragraph**, which said they would become `F-50` onward. The
handback's argument is `AGENTS.md`'s own: a finding is a reviewer's observation
and ends `verified` or `withdrawn`; a decision is the user's. A prototype
measurement is neither — it is evidence. Pasting `P-n` into `review-design.md`
would have the Probed-and-sound lists and the Synthesis describe a review that
did not happen that way. D-27's first paragraph stands: they were batched, and
the batch is this one.

The `P-n` ids stay where they are and are cited from here. Round 4 is **not**
shown the list (`docs/memory/dont-feed-the-raiser-your-finding.md`); anything it
finds independently is a second witness.

**D-30 — the design's `resolve` is renamed, and the instrument is not touched**
(P-10). User: *"Rename the design's function (recommended)"*.
`crates/goad-boundary/tests/checks/structure.rs:308` asserts that no production
line under `crates/goad/src` names the identifier `resolve`, and
`scan::mentions` (`scan.rs:225-234`) splits a line on every non-alphanumeric
byte and then on camel boundaries, word-matching each segment singular-or-plural
— so `resolve`, `resolves`, `resolve_index` and `Resolve` all trip it, and
`code_of` keeps string literals, so a diagnostic message carrying the word trips
it too. The design's function is red from the first line that lands, and
`just check` runs `cargo test --workspace`.

**The name taken is `interpret`**, which pairs with `as_drawn` and keeps the
"interpret an index against the drawn alternatives" reading. §5.2 states the
constraint alongside it, because it binds the whole renderer permanently and not
just this function.

Rejected: narrowing the instrument to match the import path, which is exactly the
brace-grouped-`use` evasion F-3 raised and this instrument was written to defeat —
a real reduction in what `ADR-001` holds, to buy a name. And exempting
`view_model.rs`, which is the same reduction with a smaller blast radius and is
the kind of carve-out a later agent widens rather than argues with. A third route
was checked and does not exist: moving the needle from `resolve` to `schedule`
would hold the same requirement without weakening it, but `controller.rs:16`
imports `goad_semantics::schedule::wait_for`, so the renderer legitimately names
that module.

**D-31 — a report whose kind is not the field's is refused** (P-1). User:
*"Refuse — a third `None` case"*. `interpret`'s surface becomes three cases, not
two: a choice index no alternative has, a non-finite `AdjustedValue`, and a
report whose variant does not match the drawn kind. All three are renderer bugs
and take the `Refused::UnknownField` posture — reported, nothing recorded, the
draft's value stays on screen.

Rejected: the prototype's own choice, that a mismatch never yields `None` and
takes the most conservative in-kind answer. Its ground is that `AdjustedText` is
*recorded verbatim, always* — but that rule is about an in-kind `AdjustedText`,
and on a mismatch there is no in-kind rule left to honour. Recording a value in
response to a renderer bug is what D-6 refuses in its own words: a value nobody
gave, held as though someone gave it. Also rejected: narrowing the signature so
the thirty pairs cannot be formed, which reshapes `Reported` to answer a question
one sentence answers.

What P-1 was actually about survives either way — two implementers reading §5.2
as it stands produce different `None` surfaces and both pass review. Three stated
cases is what closes that.

**D-32 — a number's spelling switches on length, not on magnitude** (P-3). User:
*"Switch on length (recommended)"*. `Edited::Adjusted` carries a text beside the
number, so `as_drawn` cannot answer for a `number` without choosing a format, and
§5.2's as-drawn bullet named only the number. The text is what the widget is
drawn showing and what the guard compares, so the choice is not cosmetic.

The rule: format with `f64`'s `Display` — the shortest decimal that reads back as
itself, which never uses scientific notation — and where that spelling exceeds
**24 characters**, use `{:e}` instead. The trigger is length because the defect
measured is length: `min: f64::MAX` is a legal `R-17` bound and draws a
309-character `LineEdit`, and the smallest normal draws 326. 24 leaves alone
every number a person would type — `f64` round-trips in at most 17 significant
digits, so 17 digits, a sign and a point is 19 — and catches the spellings that
are long only because the exponent is large. Both forms re-parse under the
grammar P-5 pinned, which admits `e` and `E`, so the guard's comparand
round-trips either way.

Rejected: switching on magnitude, the spreadsheet rule — two constants, and it
sends `1e16` to scientific when its plain spelling is 17 characters. And leaving
`Display` alone and stating the consequence the way CD-1 states the max-only one,
which is defensible on `R-35` grounds but makes the host's own screen the place
the backend's legal declaration is paid for.

§5.2 names the constant beside the parse rule it is the inverse of, and §9 pins
it with a case: a bound that spells long is drawn `{:e}`, and the number survives
the round trip.

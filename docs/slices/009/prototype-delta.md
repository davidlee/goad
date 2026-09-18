# Prototype delta — the thirteen outstanding findings of slice 009

Derived from `review-design.md` (rounds 1–3), `design-log.md` D-21 … D-24 and
`notes.md`. Line numbers are against `design.md` at `a01ae6a`, identical here and
in `/home/david/dev/goad`. All thirteen are **dispositioned `fix-now` and
user-confirmed** (D-23 for F-40, D-24 for the rest); none is integrated. Meant to
be sufficient on its own. Order of application: **F-38 … F-41 first and
together** (one class; F-40 carries the decision, F-41 and F-6 are subsumed by
its answer), then F-20, F-26, F-29, F-32, then F-42 … F-45.

## A. The F-38 … F-41 family (all four blockers) — plus F-6

One mechanism. Read §D before writing any of them individually.

### F-38 — pending edits lose their originating view

**Blocker.** Pending state is `Reported` keyed by `(option, field)`;
`PendingEdit` carries option, field and value only. On answer every entry is put
inside a `Choose` carrying the **current** view, so an entry made on a replaced
view is either applied to the replacement (same id strings) or refused as
`UnknownField` instead of `SupersededView`. The timer path has no source at all
for the `view` that `Command::Edit` requires.

**Disposition.** `PendingEdit` carries the `view` its edit was made on. The
`edited` callback already receives that view as its first argument, so it costs a
struct field and no new plumbing, and it is the only available source for the
`view` a deferred `Command::Edit` needs. On a drain — timer or answer — an entry
whose view is not the command's is **discarded and reported** through the existing
`SupersededView` site rather than applied, so the map is self-cleaning.

*Rejected:* clearing the map when the row model is rebuilt — same effect by a
less direct route, and it still leaves the timer path with no `view`.

**Sections:** §5.1 `pending.rs` (`:194-207`); §5.2 `wire.rs — Choose carries the
flush` (`:655-686`); §5.3 ownership row (`:696`); §5.4 *An answer* (`:768-785`);
§5.5 Edges (`:933-935`). **Code:**
`PendingEdit { view, option, field, value: Reported }`, all `String` but the last.

**Integration note from `notes.md` (not in the Response — verify it):** the rule
applies at **three** sites, not the Response's two. An entry is *shown*, *sent*
and *drained* only where its view is the retained one. The display site matters:
on a new view the rows are rebuilt and slots renumbered, so a stale entry keyed
`(option, field)` whose ids happen to match a new field would otherwise be
overlaid (F-40) onto the new view's widget. **State it once, over three sites.**

### F-39 — `Wire::send` cannot report enqueue success

**Blocker.** The design promises `chosen` and the timer retain pending entries when
the send comes back `Full`, but `Wire::send(&self, Command) -> ()` consumes the
command and discards `TrySendError::Full(_returned)` (`wire.rs:130`).

**Disposition.** `Wire::send` reports whether the command was enqueued — a
**return type, not a mechanism**: the outcome is already in hand at
`crates/goad/src/wire.rs:127-133` and is discarded one line before the caller
that needs it. The timer and `chosen` clear `pending.rs` only on an enqueued
send. Existing callers are unaffected: the result is advisory, notice unchanged.

**Sections:** §5.2 `wire.rs` block; §5.4 *An answer* (`:768-785`); §5.5 Edges row
*channel full when a person answers*. **Code:** signature change at
`crates/goad/src/wire.rs:127` — `Ok(())` returns success, `Full` sets the notice
and returns failure, `Closed` stays the deliberate no-op (D8).

### F-40 — a present during the debounce overwrites a captured edit — **D-23**

**Blocker, and the decision of round 3.** `pending.rs` and the draft are
separate. Until the timer or answer flushes, the widget holds the new value while
`glass.rs` derives `values` from the old `Prepared` draft; any intervening present
writes the old value, bumps `epoch`, and the guard converges — the path §5.4 calls
a dropped edit. Verified: the serve loop presents **before every command**
(`controller.rs:738-739` — `glass.present(...)` is the first statement of
`'serving: loop`), so this is not an edge.

**Disposition (D-23, user: *"Overlay pending on the draft (recommended)"*).**
The value channel is built from the draft **overlaid with what `pending.rs`
holds**: `glass.rs` takes a handle to the same `Rc` the callbacks hold, and a
pending entry for an `(option, field)` is preferred over the draft's value for
it. A present inside the window then writes back what the person typed, the
strings agree, and the guard is quiet. **Nothing about the guard changes.**

AC-6 keeps its meaning and gains precision: a widget is corrected exactly when
the host does not hold its value, where *hold* means the draft **or** pending. A
dropped or refused edit has left pending and never reached the draft, so it
converges as before; a `Full` send keeps its entry, so the widget stands and the
notice explains it.

**Cost, stated rather than absorbed.** §5.3 says both models derive from
`Prepared` alone and that there is therefore no cache and no invalidation rule
(`:707-711`). `values` now also reads `pending.rs`. That is not a cache — it is
live state with a stated lifetime and a single writer — and the overlay has no
invalidation rule either, but the sentence must say so rather than be quietly
false.

**One thing to measure, not assume.** The overlay *appears* to subsume §5.2's
cleared-field exception (a cleared field is a pending `""`, so the strings agree
on their own). It **stays** until `spike-fields/tests/numeric_guard.rs`'s case is
re-run against the overlay and shown not to need it — this comparand has been
wrong three times, twice on reasoning.

*Rejected:* a per-field suppression flag in `FieldValue`, which spells the same
information as *do not converge* rather than *this is the value* and leaves
channel and widget disagreeing on purpose. Also rejected and offered explicitly:
dropping the debounce, which is D-4 and a standing user commitment.

**Sections:** §5.2 *The guard* (`:399-443`); §5.3 ownership table and the "no
cache" paragraph (`:690-711`); §5.4 *A keystroke* (`:737-766`); §5.5 Edges; §9
AC-6 row (`:1075`). **Code:** `SlintGlass` gains one field holding the
`Rc<Pending>`; construction order is **already right** — `main.rs:90` installs the
callback table before `SlintGlass::new` at `:97`, so the `Rc` is created at step 6
and cloned into both. One field, no reordering.

**Integration note from `notes.md`:** the overlay goes **through `resolve`**, not
through a second mapping. `pending.rs` holds `Reported`; `glass.rs` displays from
`Edited`. Resolve the pending entry and use the result in place of the draft's
value — one display mapping. A `Reported` → `FieldValue` mapping beside the
existing `Edited` → `FieldValue` one is the duplication to avoid.

### F-41 — one timer cannot drain a two-entry map without reverting one

**Blocker.** With A and B pending, one `Command::Edit` per fire records only one.
Handling it presents immediately; the other entry is still absent from the draft
and is reverted by the guard (F-40). Re-arming cannot prevent that intervening
present; not re-arming strands the entry until answer.

**Disposition.** Two things answer it, **neither a new command shape**:

1. F-40's overlay — an entry still in `pending.rs` is what the value channel
   shows, so the entry the timer has not sent yet is not reverted by the present
   that follows the one it did send.
2. The timer **re-arms while the map is non-empty**. §5.1's "one `Command::Edit`
   goes down the channel, as today" was true of a single pending slot and is not
   true of a map. One entry per tick reaches the draft; the answer drains
   whatever is left in one `Choose`. Nothing waits on the answer for
   correctness, only for immediacy. The channel holds one (`main.rs:86`) and
   `serve` shares the UI thread, so one command per tick is the most available —
   a consequence of D22's constraint, not a choice made here.

**Sections:** §5.1 `pending.rs` (`:194-206`, the "two ways an edit leaves"
paragraph); §5.4 *A keystroke* (`:737-756`); §9 (`:1067`, `:1073`), which **gains
a row** that would have caught this: two fields edited inside one window, the loop
run past the debounce **without answering**, both values in the draft and neither
widget reverted. Existing rows exercise one timed field, and two fields only on
the synchronous answer path.

### F-6 — one pending command cannot preserve edits to multiple text fields

**Major; contested twice, upheld both times.** The round-2 repair (`Choose`
carries `Vec<PendingEdit>`) held this finding's own requirement — every typed
field survives to the answer — and is **not** what was wrong. What was wrong is
that the map's *timer* path still sends one `Command::Edit`, and handling it
presents while the map's other entry is absent from the draft, so that widget is
reasserted to its old value.

**Disposition.** No second command shape: the answer is exactly the F-40 + F-41
pair. Fix §5.1's sentence "When the timer fires, one `Command::Edit` goes down the
channel, as today" (`:201-202`) accordingly. **Sections:** §5.1 (`:194-207`), §5.4.

## B. The four remaining upheld contests

### F-20 — `slider_bounds` still admits an inoperable step

**Major, contested; upheld.** The three-clause predicate (exact endpoint
round-trip; finite, strictly positive `f32` span; finite, strictly positive step)
rejects `[1,1]`, `[-f32::MAX, f32::MAX]` and an underflowing step, but still
admits `minimum = 2^100`, `maximum = 2^100 + 2^77`: both endpoints round-trip and
`span / 100` is finite and positive — yet that step is below half an ulp at
`minimum` (the `f32` ulp there is `2^77`), so `minimum + step` rounds back to
`minimum` and `increment()`, which is exactly
`root.set-value(root.value + root.step)`
(`widgets/common/slider-base.slint:126-131`), freezes. Positivity does not
establish operability. Verified by hand against the locked source.

**Disposition.** `slider_bounds` gains a fourth condition in the same form as the
others — *Slint's arithmetic has to work afterwards*:

> `minimum + step` must exceed `minimum`, and `maximum - step` must fall below
> `maximum`, both evaluated in `f32`.

This **subsumes** the third clause (a step that underflows to zero fails it, and
so does a non-finite one); the span clause **stays**, because it is what makes the
division safe to perform at all. Everything the tightened predicate rejects takes
the text control.

**Sections:** §5.2, the `slider_bounds` bullet list (`:337-341`) and the paragraph
justifying it (`:343-356`); §5.5 Edges row *a `number` with both bounds but a range
no slider can operate* (`:929`). **Code:** `slider_bounds` ends with three clauses,
not four — endpoint round-trip; finite, strictly positive `f32` span;
`min + step > min && max - step < max` in `f32`.

### F-26 — the numeric text parser does not implement Slint's locale rule

**Major, contested; upheld, and the class is wider than the repair assumed.**
D-16's rule ("a single comma read as the decimal separator where no dot is
present") fixed the example locale, not the class. `string_to_float` takes an
**arbitrary separator `char`** from ICU
(`i-slint-common/lib.rs::decimal_separator_for_locale`) and replaces *that*
character, rejecting any `.` when the separator is not `.`
(`i-slint-core/string.rs:398-412`). It is **live in this build**: `i-slint-core`'s
default `std` feature enables `i-slint-common/locale-decimal-separator`
(`i-slint-core/Cargo.toml:82-95`). And it is **not reachable from host code**:
`SlintContext::locale_decimal_separator` is `i-slint-core`, which `crates/goad`
does not depend on, and the `slint` crate re-exports neither it nor
`string_to_float`. Both verified this session.

**Disposition.** The host stops naming the separator and accepts exactly the
class the control admits. `string_to_float` accepts a text one of two ways: the
separator is `.` and the text parses; or the separator is not `.`, the text
contains no `.`, and it parses once that one character is replaced. The host
mirrors that **without knowing which case it is in**:

> Parse the text as it stands. Failing that, if it holds exactly **one**
> character outside the numeric grammar, replace that character with `.` and
> parse again. Empty text is still zero.

Which characters count as the numeric grammar is **the plan's** to pin down
against `f64::from_str`. The rule stays what D-16 wanted: host-side, and testable
without a locale fixture.

**Sections:** §5.2 *Parsing the text is done under the rule the control validated
it with* (`:294-305`), and the D-16 paragraph (`:325-328`).

### F-29 — §5.1 and §5.2 undercount `instant.rs`'s impure call sites

**Nit, contested; upheld.** §5.2 claims two reads, "the system zone, in
`compose`, and the clock" in `today_local` (`:650-653`) — but
`today_local() -> (Date, Time)` promises today's **local** date with no zone in
scope, so it must read `TimeZone::system()` as well.

**Disposition.** There are **two impure kinds** of read at **three call sites**:
the clock, in `today_local`; and the system zone, in `compose` **and** in
`today_local`. §5.1 and §5.2 name the three sites — a module whose whole reason to
exist is holding the impurity should not undercount where it performs it. Nothing
else moves: §10's manifest argument is about `TimeZone::system` being called at
all, which was already true.

**Sections:** §5.1 `instant.rs` (`:216-219`); §5.2 the `today_local` doc comment
(`:604-606`) and the closing sentence at `:650-653`.

### F-32 — §5.2 inverts Slint's accessible-step rule

**Nit, contested; upheld — the gloss was inverted when the repair was
integrated.** §5.2 quotes `min(root.step, (maximum - minimum) / 100)`
(`fluent/slider.slint:29`) and calls it "a floor on the step" (`:390-391`). It is
a **cap**. **Disposition:** under this design's `step`, which is exactly that
hundredth, the cap binds at equality and the two coincide — the only thing the
sentence needed to say.

**Sections:** §5.2, the `step` paragraph (`:386-394`). One sentence.

## C. F-42 … F-45

### F-42 — `Reported` can express non-finite slider values

**Major.** `Reported::AdjustedValue(f32)` admits `NaN` and both infinities, while
the design says `Reported` "cannot express a non-finite number". `resolve`'s
documented `None` surface names only an out-of-range choice index, leaving the
non-finite slider report to be invented by an implementer.

**Disposition.** `resolve`'s `None` surface is stated **in full**, as §5.2
already states `compose`'s four fallible steps and for the same reason — a step
left off the list becomes an `unwrap`. There are **two**: an index no alternative
has, **and** a non-finite `AdjustedValue`. Both are renderer bugs and take the
`Refused::UnknownField` posture: reported, nothing recorded.

§5.2's claim becomes what is true: `Reported` cannot express an id nobody
declared — that half stands, and `AlternativeId::new` is why. A number reaches
the draft only through `resolve`, which refuses a non-finite one, so **I-G is
held by `Finite` at the wire and by `resolve` at the boundary. It is not held by
the shape of `Reported`, and §5.5 I-G says so.**

*Rejected:* a `Finite` payload — `Finite::new` would then run inside a Slint
closure, which has nothing to report a refusal to and no draft to leave alone.

**Sections:** §5.2's "buys three things" paragraph (`:560-564`); §5.5 I-G
(`:902-908`). **Code:** `resolve` keeps its shape but see the `notes.md` signature
note under §D — `shown: Option<&Edited>`, not `&Edited`.

### F-43 — `chosen` cannot drain in declared field order

**Minor.** `chosen` is told to drain in declared field order, but its map holds only
`(option, field) -> Reported`; neither the key nor `PendingEdit` carries a slot or
order, and `install.rs` does not own the retained `Presentation`.

**Disposition.** The ordering **moves to where an order actually exists**: the
controller applies carried edits on the declared-field walk it already makes for
`answer`, so the order is a property of that walk rather than of the map, and
§5.4 stops asking the callback for something it cannot derive. Nothing observable
changes either way — the keys are distinct by construction, so any order yields
the same draft, which is why the promise was safe to make and also why it was not
worth making.

**Sections:** §5.1 (`:194-202`); §5.4 *An answer* (`:768-773`), the phrase "in
declared field order".

### F-44 — §9 binds no test obligation for picker reseeding

**Major, and the dispositioning session's own gap** — F-36's repair took the
re-seed out of the loop-tier list and left it in prose, which is the failure F-11
raised about outcome-only rows. Prose outside §9's table binds nothing.

**Disposition.** §9 gains an obligations row (`:1052-1075`).

- **Asserts:** a field that has been picked, reopened, opens its fresh popup on
  **that field's** retained date and time rather than on today.
- **Driver, named as a call:** the button's `invoke_accessible_default_action` to
  open; the day cell's and the `OK` `StandardButton`'s default actions to pick
  and accept; then the same button again and a query of the popup's `date`. No
  pointer, so no layout dependency.
- **Tier:** `tests/renderer/`, on round 2's verified fact that `ElementQuery`'s
  `find_all` walks `active_popups` under `init_no_event_loop`
  (`search_api.rs:291-312`). If the popup cannot be found there the row moves to
  the loop target — R9's shape, not a new rule.


### F-45 — CD-2 names an `R-16` canon change it does not specify

**Minor.** `canon-delta.md` CD-2's heading paragraph says the `R-16` row "gains
cases", but its three changes cover only `R-57`, `R-58` and `R-55`.

**Disposition.** The `R-16` mention **goes** rather than gaining a fourth change.
`SPEC-001` §Verification's `R-13, R-14, R-16` row is about the wire forms
normalization accepts: it already names `R-16-a-{text,boolean,datetime,choice}-field`
and both `number` fixtures — *"every kind in its wire form"*. Drawing a kind in a
renderer changes nothing it claims. **Sections:** `canon-delta.md:52-54`, the CD-2
`**Document:**` line. `design.md` §10 (`:1114-1120`) already names only the three
and needs no change.

## D. Net design shape after the thirteen

`pending.rs` owns a map `(option, field) -> PendingEdit`, where
`PendingEdit { view, option, field, value: Reported }`, plus one `slint::Timer`,
behind an `Rc`. **Three holders**: the `edited` callback (writes), `chosen`
(drains), and — new, from F-40 — `glass.rs` (reads). Four things hang off it:

1. **Display.** `glass.rs` builds `values` from the draft **overlaid** with
   pending: for each field, if `pending` holds an entry whose `view` is the
   retained one, `resolve` it and display the result **in place of** the draft's
   value. One display mapping, not two.
2. **Timer drain.** On fire, take one entry and send `Command::Edit` carrying its
   `view` and its `Reported`. Clear that entry **only if `Wire::send` reports
   enqueued** (F-39). **Re-arm while the map is non-empty** (F-41) — one entry
   per tick, because the channel holds one.
3. **Answer drain.** `chosen` drains every entry into one
   `Command::Choose { view, option, edits: Vec<PendingEdit> }`, one send, cleared
   only on an enqueued send. **No ordering is asked of the callback** (F-43): the
   controller applies the carried edits on the declared-field walk it already
   makes for `answer`.
4. **Staleness.** At all three sites — shown, sent, drained — an entry whose
   `view` is not the retained one is not used; on a drain it is discarded and
   reported through the existing `SupersededView` site (F-38).

**The invariant this buys** (from `notes.md`; worth stating in §5.5): *what the
screen shows is what an answer would submit.* A drained entry reaches the draft; a
kept entry (a `Full` send) is still displayed and still travels in the next
`Choose`; a stale entry does neither.

**`Reported` / `resolve`.** `Reported` is what a widget can say (`Checked`,
`Typed`, `AdjustedText`, `AdjustedValue(f32)`, `Chosen(u32)`, `Picked{..}`);
`Edited` is what the draft holds; `resolve` maps one to the other given the kind
and what the field shows now. After F-42 its `None` surface is exactly two cases —
a choice index no alternative has, and a non-finite `AdjustedValue` — both
renderer bugs, both `Refused::UnknownField` posture. I-G is held by `Finite` at
the wire and by `resolve` at the boundary; **not** by the shape of `Reported`.

**`resolve`'s signature** (from `notes.md`, not in any Response — verify it): it
should take `held: Option<&Edited>`, **not** `&Edited`. §5.2 as integrated has the
caller do `state_of(..).unwrap_or_else(|| as_drawn(kind))`. Push that inside —
`resolve` has the kind, so it can consult `as_drawn` itself — and both callers
then pass `state_of(..)` straight through. This also puts `as_drawn`'s two call
sites back in `view_model.rs` where it lives, and §5.2's sentence about which
sites apply it changes for the second time: it is now `answer` and `resolve`.

**The guard does not change** — comparands, exception and table all as
integrated. What changes is what `values` holds. The cleared-field exception is
*probably* subsumed by the overlay and **stays until `numeric_guard.rs` is re-run
against the overlay and shown not to need it**.

**Outside `design.md`:** `slice-009.md` §Scope gains *`glass.rs` reads
`pending.rs`*; AC-4 / AC-6 may need rewording (AC-6's "the host does not hold its
value" now means draft **or** pending).

## E. Where the ledger and `notes.md` disagree

`notes.md` lists five **responder** citations as known bad. The ledger is
append-only, so they stay as written. Verified here:

| claim | status |
|---|---|
| F-10's re-disposition: one `wiring.rs` site, not two | not checked here |
| F-23's Response cites `wire.rs:126`; should be `:130` | **confirmed**: `TrySendError::Full(_returned)` is at `wire.rs:130`; `pub fn send` opens at `:127` |
| the pre-repair §9's `set_accessible_value` claim "appears nowhere here" | **disagreement.** §9's no-loop table (`design.md:1029`) *does* use `set_accessible_value(text)` for the `LineEdit` row, citing `fluent/lineedit.slint:16`, and four obligation rows name it as their driver (AC-4, AC-6, AC-9, the debounce-timer row). `notes.md` may mean a narrower pre-repair sentence; as written the note reads as false. **Do not delete `set_accessible_value` from §9 on the strength of it.** |
| F-33's Response cites `fluent/components.slint:15-19` for `ListItem`; should be `:49-53` | **confirmed consistent**: `design.md:1033` and `:1039` already cite `:49-53`, so the design is right and only the Response is stale |
| F-42's location line "cites §5.5 I-G for a claim that is in §5.2" | **disagreement.** The claim is in **both**. §5.2 `:562-563` says `Reported` "cannot express a non-finite number or an id nobody declared"; §5.5 I-G `:907-908` says "`Reported` carries a typed number only as text, so a non-finite one is not expressible at the boundary either." F-42's Location (`§5.2 Reported/resolve; §5.5 I-G`) and Evidence (`:529-564,902-908`) both name the two. **Repair both sentences**; the ledger's location line looks correct as written. |

`notes.md`'s integration notes — the `resolve` signature, overlay-through-
`resolve`, F-38's three sites, the screen/submit invariant, the construction
order, `Wire::send`'s returned command, CD-2's `R-16` mention — are **not** in any
Response. They are reproduced above under the findings they belong to and flagged
as such. Three were spot-checked and hold: `main.rs:90` installs before
`SlintGlass::new` at `:97`; `wire.rs:127-133` binds and drops `Full(_returned)`;
`controller.rs:738-739` presents at the top of `'serving: loop`.

# Plan log — Slice 009

Append-only, time-ordered. What was asked, what the user decided, and why.
Decisions only: findings live in `review-plan.md`, current truth lives in
`plan.md`.

**Id sequences.** This slice already carries three that collide —
`design.md` §7's `Dn`, `design-log.md`'s `D-n` and `prototype-notes.md`'s `P-n`.
This file adds none: entries are cited by their dated heading.

---

## 2026-09-19 — the plan is accepted, and the adversarial review is not run

**Raised.** `docs/AGENTS.md` §Plan puts two things to the user in order: whether
to subject the plan to adversarial review in `review-plan.md`, and then
acceptance. Both were put, with the observation that the design review had found
a blocker in three consecutive rounds and so a plan review is not a formality.

**Decided.** User: *"accept the plan, and begin phase 01"*. `plan.md` as written
at `44fbd8e` is accepted, and **no plan review runs**. `review-plan.md` is not
created.

**What that costs, recorded rather than argued.** The plan is unreviewed, so the
first adversarial reading of it happens at `review-code.md`, after the code
exists. Two things partly cover it. The plan stage was itself a verification
pass — it re-derived the design's `path:line` citations from `grep -n` and found
the sixth known-bad one (`design.md` §8 R5's `controller.rs:753-761`, corrected
to `:661-675`), which is the class of defect a review round finds. And the phase
plan is a second reading per phase: `AGENTS.md` requires that expanding a phase
into a sheet stop and go back rather than quietly repair, so nine expansions each
get a chance to catch what a review would have.

What is **not** covered is the plan's shape — the phase boundaries, the ordering
argument, and whether the Coverage table's mapping actually discharges each
acceptance criterion. Those were checked once, by the agent that wrote them. If
a phase boundary turns out wrong, the signal will be a phase that cannot end
green, and the response is to go back to plan rather than to widen the phase.

**Stage** moves to `executing`, and PHASE-01 begins. Its entry criteria are
discharged: EN-1 by `design-log.md` D-37 and this entry, EN-2 by `just check`
exiting 0 on a clean tree at `44fbd8e` (run 2026-09-19).

## 2026-09-19 — PHASE-02's Surfaces line was short by one file

**Raised.** PHASE-02's EX-3 requires that *"`Edited` and everything above it
drop the `Eq` derive"*. `Command` is above it (`wire.rs:18-28` carries an
`Edited` in `Command::Edit`), so the criterion compels an edit to
`crates/goad/src/wire.rs` — and PHASE-02's **Surfaces** line did not name that
file. The executing agent made the edit, correctly: one derive line and the doc
that says why (`ec0c7bf`).

The exposure is smaller than it looks. `slice-009.md` §Scope **does** name
`wire.rs`, and that card is what `audit.md`'s surface diff is read against, so
the audit's strongest instrument was never going to see an undeclared path.
What was wrong is the phase-level record: a phase whose Surfaces list is short
cannot be used to say what a phase was allowed to touch.

**Decided.** Amend, rather than leave it to audit's Reconciliation table. User:
*"proceed"*, on a recommendation to amend. `plan.md` PHASE-02 §Surfaces gains
`crates/goad/src/wire.rs`, scoped to *the `Eq` derive on `Command` and its doc,
nothing else*, and dated. This edits an accepted artefact, which is the only
reason it was the user's call and not the plan stage's.

**Noticed while doing it, and not acted on.** PHASE-03's EX-3 reads *"`Command::Edit`
carries a `Reported`, and `Command` drops its `Eq` derive with `Edited`"* — and
the second clause is **already true on entry**, discharged by PHASE-02. The two
criteria overlap rather than conflict: dropping `Edited`'s `Eq` breaks `Command`'s
derive in the same compile, so it could not have waited for PHASE-03. Left as
written, because criterion ids are immutable and a criterion that is already true
is discharged, not failed. PHASE-03's agent is told so up front.

## 2026-09-19 — three more Surfaces lines, and the class behind all four

**Raised.** PHASE-03's agent stopped twice rather than deciding alone: once on
`crates/goad/src/install.rs` being absent from PHASE-08's and PHASE-09's
Surfaces, and once on `crates/goad/tests/renderer/tree.rs` being absent from its
own. Both verified before acting.

- **`install.rs`** is named at `plan.md` PHASE-03, PHASE-05 and PHASE-07 and
  nowhere else, yet PHASE-08 draws the two `number` controls and PHASE-09 the
  `ComboBox`, and each needs its own arm in the `edited` mapper PHASE-03 writes.
- **`tree.rs`** holds the only case outside the declared Surfaces that widening
  `callback edited` breaks: `activating_a_field_control_fires_edited_with_all_four_selectors`
  binds `window.on_edited` directly, and `type EditedArgs` holds its arguments.
  Confirmed by instrument: `grep -rn "on_edited" crates/goad/` returns exactly
  two sites, `install.rs:43` and `tree.rs:435`.

**Decided.** Amend all three, dated and scoped, under the endorsement the user
gave for PHASE-02's: *"amend ph02 surfaces as you recommend"*. The `tree.rs`
repair is PHASE-03's to make — the fourth element of `EditedArgs` becomes a
`FieldEdit` and the assertion reads the `checked` slot — and it strengthens
rather than weakens the case, which exists to prove all four selectors arrive.

**The class, which is the part worth keeping.** Four Surfaces lines have now
been short, and it is one cause rather than four mistakes. The plan's Surfaces
lines were derived from `design.md` §9's enumeration, and §9 enumerates
**constructors** — *"twelve in `tests/renderer/wiring.rs`, ten in `draft.rs`'s
own tests, and the one closure in `install.rs`"*. A **binder of a markup
callback** is not a constructor, and neither is a file that only has to change
because a type above it changed (`wire.rs`'s `Eq`). So the enumeration was
accurate and its reach was not what the Surfaces lines needed. §9's count of
twelve was never wrong: `tree.rs` is a thirteenth site the count never claimed
to cover.

**How far the class reaches, bounded rather than feared.** `tree.rs` is the only
test file that binds a markup callback at all — it binds both `on_edited` and
`on_chosen`, and no other file under `crates/goad/tests/` binds any. Of the
phases still to run, only PHASE-03 changes a markup callback's **signature**;
PHASE-05 changes `Command::Choose`'s shape host-side and leaves
`callback chosen(string, string)` alone, and PHASE-07 through PHASE-09 add
*fields* to `FieldEdit`, which does not break a binding that reads one slot.
PHASE-09 already names `tree.rs`. So the sweep is complete and no further phase
is exposed by this cause.

No criterion changed in any of the three phases, and no id was renumbered.

## 2026-09-19 — a doc the phase makes stale is amended in that phase

**Raised.** PHASE-03's EX-5 widens what earns `Refused::UnknownField`: it is now
also what a report `interpret` cannot make sense of earns — an index no
alternative has, a non-finite slider value, or a report whose variant is not the
drawn field's kind. The variant's own doc at `crates/goad/src/diagnostics.rs:61-66`
describes only the old path, and `diagnostics.rs` was not in PHASE-03's Surfaces.
No behaviour changes: the variant and the rendered line are both untouched, and
the line — *"could not match that control to a field of the option it names"* —
reads correctly for the new path too.

**Decided.** Amend, in this phase. **The slice has already settled this class
twice and both times the answer was the same**: `design.md:1042` says of the
`Glass::present` contract *"The trait's doc is code and is amended in the phase
that lands this"*, and PHASE-04/EX-5 does exactly that for `clock.rs:47-53`
rather than leaving it to audit's Reconciliation table. A divergence *discovered*
at audit belongs in that table; one the slice creates knowingly does not.
PHASE-03 §Surfaces gains `crates/goad/src/diagnostics.rs`, scoped to that doc
comment alone.

Leaving it would have stood a stale doc through PHASE-05 to PHASE-08 — which are
precisely the phases that make the new path reachable in production — for the
sake of reaching `diagnostics.rs`'s natural home in PHASE-09.

**Two clauses are stale, not one.** The agent named the first sentence. The
fourth is worse: *"Only reachable from a stale or malformed callback"*. After
EX-5 a well-formed, current callback reporting a non-finite slider value earns
this refusal, and that is neither stale nor malformed.

## 2026-09-19 — PHASE-04's Surfaces, and the fifth instance of the class

**Raised.** PHASE-04's agent stopped on `crates/goad/ui/app.slint` and measured
rather than argued. `compose(date: Date, time: Time)` takes **Slint's** `Date`
and `Time` — `design.md:862-864` says so, EX-4 says it from the other side
(`compose` *uses* jiff's `Date::new`, so its inputs are not already jiff civil
values), and PHASE-07/EX-3 confirms the return. Those two structs are declared
in the widget library (`widgets/common/datepicker_base.slint:7-11`,
`time-picker-base.slint:332-336`) and are **not** generated into
`crate::generated` today, because `app.slint:1` imports neither.

Measured four ways, each built, each restored — `app.slint` copied out first and
compared back by `sha256sum`, `git status` clean afterwards. Instrument:
`grep -n "pub struct r#Date|pub struct r#Time"` over the build script's
`out/app.rs`.

| put in `app.slint` | `Date` / `Time` emitted |
|---|---|
| nothing — the tree as it stands | no |
| `import { Date, Time } from "std-widgets.slint";` | **no** — an import alone is not an export |
| that import *plus* `export { Date, Time }` | yes |
| `export { Date, Time } from "std-widgets.slint";` — one line | yes |

**Decided.** Amend, under the endorsement the user gave for PHASE-02's. PHASE-04
§Surfaces gains `crates/goad/ui/app.slint` scoped to that one line, and EX-3
gains the clause that makes the phase own it. The alternative — folding
`instant.rs` into PHASE-07 — was priced and rejected: it costs PHASE-04 the
independence that makes it the one phase able to run beside another, loads the
slice's widest markup phase with three fallible functions and their units, and
buys nothing, since PHASE-07 writes the same line one phase later.

The line declares no control and reads no slot, so `app.slint:20-26`'s standing
rule — *nothing is declared before a control reads it* — is untouched. It makes
two library types nameable from Rust; it does not declare anything the markup
draws.

**The fifth instance, and the class holds.** The agent's own reading, which is
right: the Surfaces lines came from `design.md` §9's enumeration of
**constructors**, and *a file that changes only because a type above it changed*
is not one — that was `wire.rs`'s `Eq`. This is the same shape one step further
out: `app.slint` changes because a type must become **visible**. The design is
not wrong; the Surfaces line is.

**One thing not taken on the agent's word.** It expects the line to be
transitional — a struct reached from an exported struct is generated without its
own export, so PHASE-07 should be able to delete it when `FieldValue` gains
`date` and `time`. That is a prediction, not a measurement: what was measured is
the export path, not the reachability one. PHASE-07/EX-3 now says to re-measure
and delete on the measurement rather than on the expectation.

**VA-2's instrument is changed, and strengthened.** Its literal wording asks for
an offset that is not `+00:00`, with an escape hatch if the CI zone makes that
unassertable. The agent proposed `!TimeZone::system().is_unknown()`
(`timezone.rs:705`) instead: featureless jiff falls back to `TimeZone::unknown()`
(`timezone.rs:325-337`), so that predicate tests precisely what VA-2 exists to
test, and unlike an offset it stays true on a UTC CI box. Accepted — it is the
better instrument, not the escape hatch. This machine's zone is
`Australia/Melbourne`, `is_unknown = false`, `+10`, recorded in the sheet beside
it.

## 2026-09-19 — PHASE-05's Surfaces, and the class reaches the callers

**Raised by the phase agent while expanding the sheet, before any production
code.** Three criteria compel files PHASE-05's Surfaces line does not name. Each
was verified by the orchestrator against the tree rather than taken from the
report.

**S-1 — `Command::Choose` gains `edits` (EX-6), and three literals outside the
Surfaces stop compiling.** `grep -n 'Command::Choose'` finds nine sites. Two are
inside the Surfaces (`wiring.rs:1021`, `:1102`), one is production
(`install.rs:27`), two are non-constructing (`controller.rs:675`'s pattern, a doc
at `harness.rs:70`), and **three are literals in files the line does not name**:
`tests/renderer/scheduling.rs:269`, `:925` and `tests/renderer/ingress.rs:381`.
Each is `edits: Vec::new()` and asserts nothing about edits.

**S-2 — `install` takes the pending handle (EX-8), and two loop targets stop
compiling.** The four call sites are `main.rs:90`, `tests/renderer/fields.rs:301`,
`tests/event_loop/closing.rs:57` and
`tests/event_loop_schedule/scheduling.rs:84`. The last two are in **PHASE-06's**
Surfaces and not PHASE-05's.

**Decided: amend both, under the endorsement the user gave for PHASE-02's.** In
each case the criterion and the design already agree and it is the Surfaces line
alone that is short. `design.md:1530` says *"every case that builds a
`Command::Choose` is rewritten for its new shape"* without enumerating them, and
`design.md:1537-1543` names all four `install` call sites in as many words. This
is the sixth and seventh instance of the one class, and the cause is unchanged:
the Surfaces were derived from `design.md` §9's enumeration of **constructors**,
so a file that changes because a *type* changed under it was never in the
enumeration's reach. S-2 is a variant worth naming on its own — the widened thing
is a **signature**, and the files it reaches are ones another phase already
claimed. Overlapping Surfaces between phases are normal here; a Surfaces line is
not an exclusive lock.

**One citation in `design.md:1539` is stale** — `tests/renderer/fields.rs:287`
for the `install` call site, which is at **`:301`**. `design.md` is not edited
mid-slice and the correction lives here. The claim it supports is right; only the
number moved.

### S-3 — a user-visible string, and the precedent decides it

`crates/goad/src/diagnostics.rs:219` renders *"…is a {form} field; this renderer
draws boolean fields only"*. The moment `text` draws, the clause is false, and it
stays false through PHASE-06, -07 and -08. `plan.md` PHASE-09/EX-7 owns the site
and `diagnostics.rs` is in PHASE-09's Surfaces. No test asserts the string, so
nothing goes red; the agent could have finished the phase without touching it and
correctly declined to decide on its own.

**Decided: repair it here.** §*a doc the phase makes stale is amended in that
phase* settled this class, and it had already been settled twice before that —
`design.md:1042` for the `Glass::present` contract, PHASE-04/EX-5 for
`clock.rs:47-53`. The rule those three share: *a divergence the slice creates
knowingly is repaired in the phase that creates it; only one discovered at audit
belongs in the Reconciliation table.* This instance is strictly stronger than the
precedent it follows — that was a doc comment, and this is a sentence a person
reads.

**Repaired once, not four times.** The obvious repair — widen the enumeration to
"boolean and text" — is false again at PHASE-07, at PHASE-08 and at PHASE-09,
which is the *"a true sentence goes stale by widening"* trap that has now bitten
this slice three times. So the constraint on the wording is that it **name no
subset at all**: it may name the undrawn field's own form, and must make no claim
about the set the renderer draws. Then it is true at every phase boundary from
here to the close, and PHASE-09/EX-7's first clause becomes a verification rather
than a repair — amended in place there, criterion id unchanged.

### EX-9's fixture list, annotated rather than amended

Not a decision — `mapper.rs` is already in the Surfaces — but the list is wrong
twice and the phase sheet should not inherit it.

Two citations went stale under PHASE-01 … PHASE-04:
`A_DRAWN_AND_AN_UNDRAWN_FIELD` is at `fields.rs:81`, not `:71` (`:71` is blank),
and `wiring.rs`'s `TWO_FORMS` at `:1158`, not `:1157` (a doc line). `fields.rs:76`
declares a **different** fixture also called `TWO_FORMS`, carrying no `text`
field — a same-name trap one file over. This is the second and third instance of
*a plan citation can be stale*, and both were caught by `grep -n`: the slice's
rule, *cite from an instrument that prints the number*, held again.

And the list is short by one, in a way that matters more than the count.
`mapper.rs:295-332`'s `every_undrawn_kind_is_reported_by_option_field_and_form`
enumerates all four undrawn kinds and their four reports. It goes red when `text`
draws, and its repair is **not** the one-word migration the other five take: it
**drops a row**, because it enumerates the undrawn set rather than sampling it.
The agent proposed to "migrate it with the rest", which would have been wrong,
and the correction went back before the sheet was written. PHASE-07 and PHASE-08
shrink it again; PHASE-09 deletes it with the rest.

**The deferred-deletion risk is the one to hold.** `prototype-notes.md` P-13
warns that the danger is not the edit but a phase that migrates without noticing
it has deferred a deletion. This case is the same warning one step sharper: a
fixture that *enumerates* cannot defer, it shrinks — and a phase that migrates it
instead would leave a case claiming `text` is undrawn while `text` draws.

## 2026-09-19 — `undrawn_form` becomes `drawn_form`, and a decision taken out of turn

**Kept, on its merits.** `view_model.rs`'s sorter is now
`drawn_form(&FieldKind) -> Result<DrawnKind, FieldForm>` where it was
`undrawn_form(&FieldKind) -> Option<FieldForm>`. Every technical claim behind it
was checked against the tree rather than taken from the report, and every one
holds.

**Why the old pair had no honest spelling once a second kind draws.** `sift`
wrote `kind: DrawnKind::Boolean` as a constant — true only while `boolean` was
the one kind reaching it. With two drawn it must choose, and a second total
function beside `undrawn_form` would have to answer for the three kinds still
reported undrawn. It cannot: `DrawnKind::Choice` carries the first alternative's
id (`view_model.rs:106-116`), `Alternatives` exposes only `new` and `as_slice`
(`canonical.rs:352-377`), `.first()` on the slice is an `Option`, and
`AlternativeId::new` is `pub(super)` (`canonical.rs:75`) so `crates/goad` cannot
construct a fallback id at all. The remaining spellings are a lie in the type or
a field that sorts nowhere and is dropped — the one thing `I-2` and `R-20` exist
to prevent. **This constraint is not new and was not invented to justify the
change**: `DrawnKind`'s own doc, written in PHASE-02, already states it in those
words, citing `prototype-notes.md` P-2.

A `Result` has one arm per kind, no unreachable arm, and no pair of `Option`s
whose complementarity the compiler cannot see.

**Why this is an implementation change and not a design change** — which is the
whole of why it does not go back to the user. `design.md:149-151` settles it in
its own words: *"the mechanism AC-7 cares about is not `FieldForm` itself but
`undrawn_form`'s exhaustive match over the canonical `FieldKind`"*. The property
is the match; the identifier is how the design happened to spell it that day.
`slice-009.md`'s AC-7 is identifier-free — *"the mechanism that makes a sixth
kind a compile error survives"*. Under `drawn_form` the match is still one site,
still exhaustive over the canonical type, still a compile error for a sixth kind;
`FieldForm` still becomes uninhabited as the last arm crosses, so §7 D11 stands
untouched. Nothing in `design.md` needed editing and none was done.

**The name is kept, and for a reason the agent did not give.** It offered a
rename. The candidate worth considering was `drawn_as`, which reads better at the
call site — but `view_model.rs` already declares **`as_drawn`**, a different
function with a different meaning (what an untouched field is worth, per kind).
Two names one transposition apart in one module is a worse trap than a slightly
loose `Ok`/`Err` reading. `drawn_form` keeps the `_form` stem that ties it to
`FieldForm` and collides with nothing.

### The process part, which is the more important half

**The agent took this on its own, and it was not its to take.** It was item 4 of
the STOP list in its own phase sheet, committed at `98335cd` — written down,
correctly reasoned, and then acted on without being sent. The first message to
the orchestrator carried S-1, S-2 and S-3 and not this. The agent's later account
says it was *"flagged in my first message as item 4"*; it was flagged in the
sheet as item 4, which is not the same thing. **An orchestrator sees what is
sent, not what is committed.**

Its stated reason — *"leaving it undecided would have blocked every remaining
task"* — does not survive contact with what it actually did: it raised three
STOPs and kept working on everything they did not block, which is exactly the
right shape and was available here too.

The outcome was good, and that is the trap. This is the first decision in the
slice where the agent's judgement substituted for the orchestrator's and happened
to agree. A rule that only binds when the answer would have been different is not
a rule. **What was the orchestrator's to decide was not whether the `Result` is
the better shape — it is — but whether renaming a function that `design.md`,
`canon-delta.md`, `research.md` and five phases' criteria all name by identifier
is a design change.** That question is answered by reading `design.md:149-151`,
and an agent inside one phase is the wrong reader for it.

**Recorded rather than reverted**, because reverting a correct change to make a
point costs the slice and teaches nothing durable. The correction is in the brief
for PHASE-06 onward: *raise it and keep working* is the whole protocol, and a
sheet is not an outbox.

### The wording this costs, amended here

`plan.md` is the executable truth and five criteria named the old identifier.
Amended so a later agent's `grep` finds the function that exists:

- PHASE-05/EX-2, PHASE-07/EX-2, PHASE-08/EX-2, PHASE-09/EX-2 — each now states
  the `Ok(DrawnKind::…)` the phase's arm moves to, with the `undrawn_form(..) ==
  None` it read before, so the criterion is legible from either side.
- PHASE-09/EX-6 — `drawn_form`'s match, plus the sentence that matters more than
  the rename: **this is the property AC-7 names and it is identifier-free**, so
  the rename neither discharges nor weakens it.
- PHASE-09/VT-7 and §*Why `choice` is last* — renamed.
- **PHASE-02/VA-2 is annotated, not rewritten.** It is discharged, and its
  evidence in `notes.md:979` reads *"`undrawn_form`'s body is untouched"*, which
  was true when it was written. A discharged criterion is a record of what was
  checked and when; rewriting one to match a later tree destroys the only thing
  it is for.

`design.md`, `canon-delta.md` and `research.md` keep the old identifier. They are
records of intent at a point in time, not executable truth, and `plan.md` now
carries the pointer in both directions.

## 2026-09-19 — PHASE-06's Surfaces, and a sub-claim that was wrong in the safe direction

**Raised by the phase agent before any production code, while continuing on
everything the STOP did not block** — which is the protocol working, and is
worth recording because the previous phase is where it did not.

**STOP-1 — `SlintGlass::new` gaining the `Rc<Debounce>` (EX-2) breaks two call
sites the Surfaces line does not name.** Verified by the orchestrator:
`grep -rn 'SlintGlass::new' crates/` gives **six** construction sites, not the
four `design.md:1541-1543` enumerates —
`crates/goad/tests/event_loop_reassert/reassert.rs:121` and
`crates/goad/tests/event_loop_debounce/debounce.rs:152` are the extra two.

**Decided: amend**, under the standing endorsement. Same class as PHASE-05's
S-2 and the eighth instance overall: the widened thing is a **signature**, and
`design.md` §9 enumerated the pairs it knew about. One of the two could not have
been enumerated at all — `event_loop_debounce` is PHASE-05's own new target, and
the plan was written before it existed. **That is a new sub-class worth naming:
a Surfaces line can be short about a file the slice itself creates in an earlier
phase.** PHASE-07 and PHASE-09 should expect it, since both follow phases that
add targets.

**The two are not the same edit, and the difference is the whole point of R10.**
`reassert.rs` calls no `install`, so it has nothing to share and takes a fresh
`Rc::new(Debounce::new())`. `debounce.rs` **does** call `install`
(`debounce.rs:163-166`) with a bound `pending`, so it must take
`Rc::clone(&pending)` — the glass and the callback table eleven lines apart,
which is precisely the shape R10 exists to catch. Giving that file a separate
empty handle would plant the anti-pattern in the one file a future reader is
most likely to copy. The agent argued this and it is right.

### `glass_over` is not widened — and the reasoning was right while the count was wrong

Not a STOP; the agent recorded it so the sheet would not land on a wrong number.
Its conclusion stands and its enumeration did not.

**What it reported:** *"59 call sites across 5 files, two of which
(`scheduling.rs`, `ingress.rs`) are outside the Surfaces."*

**What the tree holds:** `grep -rn 'glass_over(' crates/ | grep -v 'fn glass_over'`
is **58** calls across **four** files — `fields.rs` 1, `ingress.rs` 13,
`scheduling.rs` 14, **`wiring.rs` 30**. The fifth "file" was `harness.rs`, which
holds the definition and no call. And **three** files are outside PHASE-06's
Surfaces, not two: the one the agent missed is `wiring.rs`, which alone holds 30
of them — more than the two it named combined.

So **57 of the 58 call sites are out of the phase's reach**, and only
`fields.rs`'s single call is inside. The conclusion — do not widen `glass_over` —
is *strengthened* by the correction, which is why this is worth writing down
rather than waving through: `docs/memory/verify-the-enumeration-not-the-conclusion.md`
is precisely this shape, and a Response that inherited the sub-claim would have
put "two files, 27 sites" into the artefact as the reason for a decision whose
real reason is "three files, 57 sites".

**Decided: `glass_over` keeps its arity, and the second entry point delegates
rather than duplicating.** The agent proposed *"a second constructor beside
it"*, which risks two bodies drifting. `glass_over`'s body is four lines and is
nearly all argument-passing, so the shape that holds is one implementation with
two entry points — the arity-2 helper calling the arity-3 one with a fresh empty
handle. CLAUDE.md's *no parallel implementation* is the binding rule and it is
satisfied by delegation, not by proximity.

**Name it for the capability, not the parameter.** What a reader must decide is
whether this glass can overlay what the person has typed, and that is true
exactly when it shares the handle `install` was given. `glass_overlaying` is the
recommendation; the agent may argue a better one, but `glass_over_2` or
`glass_over_with_pending` names the plumbing rather than the question. Whichever
name lands, `glass_over`'s doc says in one line which of the two a case that
also calls `install` must use — a case that calls `install` and then
`glass_over` compiles, runs green, and measures nothing.

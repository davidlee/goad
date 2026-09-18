# Plan — Slice 009: the form grows the rest of its field kinds

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

<!-- Phase ids (PHASE-NN) and criterion ids (EN-/EX-/VT-/VA-/VH-N) are
     immutable: edits append, never renumber, so the sequence goes
     non-monotonic after a split and that is expected. Criterion ids are local
     to their phase — cite another phase's phase-qualified (PHASE-03/EX-2).
     Verification modes — VT: automated test. VA: agent check. VH: human
     acceptance.
     Progress is NOT recorded here. Status lives in `notes.md`. -->

## Overview

Nine phases. Three reshape the mechanism while `boolean` stays the only drawn
kind, one lands the datetime arithmetic off to the side, two build the debounce
and the overlay that continuous editing needs, and the last three draw one kind
each. Every phase ends green on `just check`.

The shape is forced by one fact: `Edited`, `Command` and the markup's field
structs are load-bearing for every existing test, so a change to any of them
breaks the suite until every consumer is visited. A plan that landed one kind at
a time from the outside in would therefore have no green step between the first
kind and the last. So the reshaping comes first and is kind-agnostic — the three
pure functions `as_drawn`, `interpret` and `submitted` are written for **all
five kinds with unit coverage** in PHASE-02, while the mapper still draws only
`boolean` — and each later kind phase is then one mapper arm, one markup arm,
one value-builder arm, one fixture migration and that kind's cases.

What the phases add up to:

- **PHASE-01** stops a present from rebuilding the form. That is `design.md`
  §7 D8 and D9, and it is what AC-5 asserts.
- **PHASE-02 and PHASE-03** replace one boolean with five typed values and a
  typed report, in that order: the pure layer with its units first, then the
  wiring that rewrites ~22 existing cases.
- **PHASE-04** lands `instant.rs`, the `jiff` feature and `clock.rs`'s doc
  amendment as a self-contained unit. It is the only phase that can run beside
  another.
- **PHASE-05 and PHASE-06** draw `text` and build the two halves of what
  continuous editing needs: the debounce's *delivery* (the draft gets what was
  typed) and then its *display* (the widget keeps what the host has not
  recorded yet). AC-4 and AC-6.
- **PHASE-07, PHASE-08, PHASE-09** draw `datetime`, `number` and `choice`.
  The last of them inherits the `FieldForm` retirement, because the moment the
  fifth kind draws there is no undrawn field left to build — `prototype-notes.md`
  P-13 — and with it AC-1, AC-2, AC-3, AC-7 and the human run under AC-10.

Two documents bind alongside `design.md`: `canon-delta.md` **CD-1** and **CD-2**
are the slice's working authority on two `SPEC-001` changes, cited here exactly
as canon would be. Neither is promoted by any phase; both go to audit.

## Sequencing & rationale

**Why the reshaping comes before any new kind.** `glass.rs::field_block` reads
the draft through an irrefutable `let Edited::Checked(checked) = …`
(`glass.rs:203`), whose doc says in as many words that a second `Edited` variant
is meant to be a compile error there. `Command::Edit` carries an `Edited`
(`wire.rs:38-43`) and twelve cases in `tests/renderer/wiring.rs` construct one.
Drawing a kind therefore cannot be the first move: the first move is making the
types able to carry five values, which is PHASE-02 and PHASE-03.

**Why the value channel comes before the value types.** PHASE-01 changes only
what crosses into the markup and how often, and it can do that while `Edited`
still has one variant — `state_of` is still infallible and `field_block`'s
irrefutable `let` still holds. Doing it first means the highest-risk mechanism in
the slice (D8, D9, the guard, the write order I-F) lands against a suite that is
otherwise unchanged, so a regression there is unambiguous. It also builds the
first event-loop target and the two instrument counters, which four later phases
reuse.

**Why `text` before the other three.** `pending.rs`, the overlay and
`Command::Choose`'s flush exist only for controls a person changes continuously,
and `text` is the cheapest of those. It is also the only one the prototype built,
so PHASE-05 is the phase with the most evidence behind it
(`prototype-handback.md` §3, P-11). `number` needs the same machinery and adds
two controls and a parse rule on top; starting there would buy nothing and risk
more.

**Why `datetime` before `number` and `choice`, and where that departs from the
handback.** `prototype-handback.md` §Recommendation says that if the prototype is
resumed, `datetime` goes first, because `number` mostly exercises pure functions
that already have coverage and `choice` is small. The slice is not resuming the
prototype — it re-derives from this plan — but the risk argument survives
translation: `datetime` carries the mechanisms nothing has measured (the popup
seed, the picker chain's tier, `compose`'s failure surface, §8 R6), and two of
the three `number` mechanisms it was compared against are discharged by
PHASE-02's units rather than by drawing anything. So `datetime` goes first
**among the three kinds that remain after `text`**, not first overall. Putting it
ahead of `text` would mean building the debounce after the kind that does not
need it, and revisiting `glass.rs`'s value builder for a second time.

**Why `choice` is last.** `undrawn_form` sorts a kind into drawn or
`Undrawn::FieldForm`. The moment the fifth kind draws, `FieldForm` has no
constructible variant, `dead_code` fires under `-D warnings`, and every fixture
that carries an undrawn field — five of them, `prototype-notes.md` P-13 — becomes
unsatisfiable. That retirement cannot be deferred to a tenth phase, so it lands
on whichever kind is last, and the last kind should therefore be the cheapest.
`choice` is one mapper arm, one `ComboBox` and one `interpret` arm already
written and unit-tested in PHASE-02.

**Parallelism.** **PHASE-04 is the only phase that can run beside another.** Its
surfaces are `crates/goad/src/instant.rs` (new), `crates/goad/src/lib.rs`,
`crates/goad/Cargo.toml` and `crates/goad-shell/src/clock.rs`; nothing else in
the slice touches `instant.rs` or `clock.rs`, and `lib.rs` and `Cargo.toml` are
one-line additions in disjoint regions. It may run concurrently with PHASE-01,
PHASE-02 or PHASE-03, and must land before PHASE-07. Every other phase touches at
least two of `app.slint`, `view_model.rs`, `glass.rs` and `draft.rs`, and no two
of them are disjoint. One writer per worktree: if PHASE-04 runs beside another
phase it runs in its own worktree and lands as its own commit, and neither
rewrites the other's history.

**What could be reordered or dropped.** `number` and `choice` (PHASE-08,
PHASE-09) may swap, at the cost of moving the `FieldForm` retirement onto the
heavier of the two; there is no other reason to prefer one order. Nothing can be
dropped: each of AC-1, AC-2 and AC-3 names all five kinds, so the slice has no
partial landing that satisfies its own acceptance criteria.

**Sizing.** PHASE-05 is the largest and PHASE-09 the widest; both are sized for
one session only because the work either side of them was taken out. PHASE-02
and PHASE-03 are deliberately two phases rather than one: PHASE-02 is pure code
with unit tests and PHASE-03 is a rewrite of roughly twenty-two existing call
sites, and merging them would make the rewrite the tired tail of a long session.
If a phase runs long, write a PARTIAL note into its sheet naming exactly what is
done and what is not, and hand back: a truncated phase reported as complete is
worse than a partial one.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 — five kinds drawn, in declared order | PHASE-09/EX-3, PHASE-09/VT-3 |
| AC-2 — each kind submits the JSON type `R-57` names, untouched and operated | PHASE-09/EX-4, PHASE-09/VT-4 and PHASE-09/VT-5 |
| AC-3 — `R-58`: a value for exactly the drawn fields of the answered option | PHASE-09/EX-5, PHASE-09/VT-6 |
| AC-4 — typing records every character | PHASE-05/EX-4, PHASE-05/VT-3 and PHASE-05/VT-5 |
| AC-5 — a present that changes nothing disturbs nothing | PHASE-01/EX-4, PHASE-01/VT-4 |
| AC-6 — a refused or dropped edit is corrected, element preserved, negative-controlled | PHASE-06/EX-3, PHASE-06/VT-1 |
| AC-7 — an undrawn kind is still reported; the sixth-kind compile error survives | PHASE-09/EX-6, PHASE-09/VT-7 and PHASE-09/VA-2 |
| AC-8 — a `choice` submits an **alternative** id, and no namespace is confused | PHASE-09/EX-2, PHASE-09/VT-2 |
| AC-9 — an unbounded `number` submits a number with no invented range | PHASE-08/EX-3, PHASE-08/VT-3 |
| AC-10 — `just check` exits 0, and a person has answered a five-kind form | every phase's EX-1 for the gate; PHASE-09/VH-1 for the run |

---

## PHASE-01 — the value channel and the epoch

**Objective:** a present writes the form's *values* on every call and rebuilds
its *structure* only when the `view_id` changes, so a widget is corrected by a
guarded write instead of being destroyed and rebuilt.

**Surfaces:** `crates/goad/ui/app.slint`, `crates/goad/src/glass.rs`,
`crates/goad/Cargo.toml` (one `[[test]]` entry), `crates/goad/tests/renderer/`
(`harness.rs`, `fields.rs`, `tree.rs`, `table.rs` and any case reading
`FieldRow.checked`), `crates/goad/tests/event_loop_reassert/` (new),
`tests/support/` (a shared loop-arrangement helper, if one is extracted).

**Entry**
- EN-1 — `design.md` is accepted and closed (`design-log.md` D-37) and this plan
  is accepted.
- EN-2 — `just check` exits 0 on a clean tree.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `app.slint` declares `Kind` with all five variants, `FieldRow
  { id, label, kind, slot }` and `FieldValue { checked, text, number, index }`,
  plus `in property <[FieldValue]> values` and `in property <int> epoch`. The
  `date` and `time` slots and `FieldRow`'s control fields are **not** added here
  — see the Notes.
- EX-3 — `SlintGlass` retains the last presented `ViewId`; `present` writes
  `values`, then the rows only where `frame.shown`'s `view_id` differs from the
  retained one (or where nothing is shown), then the `epoch` — in that order,
  §5.5 I-F. `option_rows` is two functions run in one pass, so a field's slot is
  its index into `values` (I-B).
- EX-4 — the `CheckBox` carries
  `property <int> tick: root.epoch; changed tick => { … }` comparing
  `self.checked` against `root.values[field.slot].checked` and writing itself
  back only where they differ.
- EX-5 — `app.slint` carries two counters as `out` properties on the window
  root, incremented from production markup (§7 D15): one from a field row's
  `init`, one from the guard's convergence write.
- EX-6 — `Glass::present`'s trait doc names the row model as its second
  deliberate exception to totality, with §5.3's argument — retained state whose
  only writer is `present`, written on exactly the frames that can change it.
- EX-7 — a new `[[test]]` target exists for the loop tier's first arrangement,
  with its own `main.rs` and its own `init_integration_test_with_system_time()`.

**Verification**
- VT-1 — `tests/renderer/fields.rs`: a present that replaces `values` wholesale
  destroys no element. Driver: two presents of the same view, then the `init`
  counter. `init` runs under `init_no_event_loop` — measured,
  `prototype-handback.md` P-12 — so this is the cheap tier and not the loop one.
- VT-2 — `tests/renderer/fields.rs`: a present carrying a **new** `view_id`
  rebuilds the rows, and the `init` counter moves. This is the control that shows
  VT-1's counter is capable of moving at all.
- VT-3 — `tests/renderer/fields.rs`: existing field cases still pass with the
  value read out of `values[slot]` rather than off the row.
- VT-4 — **AC-5.** The new loop target: two `present` calls carrying the same
  frame leave the convergence counter and the `init` counter unchanged.
- VT-5 — the same target, negative control: with the guard's difference test
  removed so it always writes, the convergence counter moves and VT-4 fails.
  Compiled and run before the red is believed
  (`docs/memory/a-negative-control-that-does-not-compile.md`).
- VA-1 — every existing `tests/renderer/` case that read a value off `FieldRow`
  reads it off `values[slot]`, and none was deleted to make the phase green.
- VA-2 — the three writes in `present` are in the I-F order, checked by reading
  the function rather than by a test: a row evaluates `root.values[field.slot]`
  while it is being instantiated, and Slint answers an out-of-range index with a
  default-initialised struct rather than an error, so the wrong order is silent.

**Notes for the implementer**

Read `design.md` §5.1 (the two channels), §5.3 (ownership, and the
`Glass::present` contract change), §5.4 (*the order of the three writes*) and
§5.5 I-B / I-F. `research.md` Thread 3 §*The split channel* is the measurement
this phase is built on: a flat value property indexed by a slot tracks, and
replacing it wholesale costs no element construction.

`glass.rs:155-213` is what you are splitting. `option_rows` nests
`OptionRow → FieldBlock → FieldRow`; `values` is **flat across the whole
presentation**, so the slot counter runs across options and blocks alike. Build
both in one pass — that is what makes I-B a property rather than a rule someone
has to remember.

**Do not add `FieldValue`'s `date` and `time` slots, or `FieldRow`'s `slider` /
`minimum` / `maximum` / `step` / `alternatives`.** §5.2 states the struct's end
state, not a landing order, and those fields mean nothing until the kind that
reads them is drawn. PHASE-07 adds the seed slots, PHASE-08 the slider fields,
PHASE-09 the alternatives.

The counters are production markup on purpose (§7 D15); a test-only copy of the
field markup is a parallel implementation of the thing under test. Expose them
as `out property <int>` on the window root so a test can read them without a
callback.

The loop target is the first of four. Name it for the **arrangement**, not the
feature (`docs/memory/slint-testing-backend-initialises-once-per-process.md`),
and expect one `#[test]` fn in it: the platform initialises once per process and
two concurrent real loops in one binary is not a thing. If you extract a shared
arrangement helper into `tests/support/`, every `pub(crate)` symbol in it must be
reachable from each includer or `dead_code` under `-D warnings` will stop the
gate (`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`).

A test that must fail from inside the Slint loop records a value, stops the loop,
and asserts on the test thread. Panicking inside the loop turns a broken
predicate into a hang.

`crates/goad/Cargo.toml` sets `autotests = false`, so a new target needs an
explicit `[[test]]` block. PHASE-04 may be editing the same file concurrently —
its change is the `jiff` line in `[dependencies]`, yours is a `[[test]]` block at
the foot.

---

## PHASE-02 — the draft's five values, and the kind-directed pure functions

**Objective:** the draft can hold what any of the five kinds is worth, and the
three pure functions that decide what a field shows and submits exist, are total
over all five kinds, and are unit-tested — while `boolean` is still the only kind
the mapper draws.

**Surfaces:** `crates/goad/src/draft.rs`, `crates/goad/src/view_model.rs`,
`crates/goad/src/glass.rs`, `crates/goad/src/controller.rs` (`answer`'s
as-drawn call only).

**Entry**
- EN-1 — PHASE-01/EX-1 … EX-7 discharged.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `draft.rs` carries `Finite` — private field, fallible constructor,
  `PartialEq` and `PartialOrd` and **no `Eq`** — and `Edited` with its five
  variants exactly as §5.2 gives them.
- EX-3 — `draft.rs` carries `Reported` with its six variants, and `Edited` and
  everything above it drop the `Eq` derive.
- EX-4 — `state_of` returns `Option<Edited>`; `submitted` has five arms, one per
  `Edited` variant, and is still the only application of `R-57` (I-C).
- EX-5 — `view_model.rs` carries `DrawnKind`, a host-local enum whose `Choice`
  variant carries the first alternative's id beside the list; `PresentationField`
  carries a `DrawnKind`.
- EX-6 — `view_model.rs` carries `as_drawn(&DrawnKind) -> Edited`, total over the
  five kinds and following §5.2's as-drawn bullets exactly, including the
  `datetime` epoch and the `number` minimum-or-zero rule with its spelling.
- EX-7 — `view_model.rs` carries
  `interpret(&Reported, Option<&Edited>, &DrawnKind) -> Option<Edited>`, which
  applies `as_drawn` itself where `held` is `None`, and whose `None` surface is
  **all three** of §5.2's cases and no more.
- EX-8 — `view_model.rs` carries the number formatter: `f64`'s `Display`, and
  `{:e}` where that spelling exceeds 24 characters.
- EX-9 — `glass.rs` builds a `FieldValue` from an `Option<Edited>` for all five
  kinds, and `controller::answer` supplies an untouched field's value through
  `interpret` rather than through a second `as_drawn` call site.

**Verification**
- VT-1 — `draft.rs` units: `Finite::new` refuses `NaN` and both infinities, and
  there is no other way to construct one.
- VT-2 — `draft.rs` units: `submitted` maps each of the five variants to the JSON
  type `R-57` names — a bool, a string, a number, the alternative id as a string,
  and an RFC 3339 datetime with an explicit offset.
- VT-3 — `view_model.rs` units: `as_drawn` over all five kinds, including
  `datetime`'s `1970-01-01T00:00:00+00:00` and a `number` whose range carries only
  a `max`, which submits `0`. This is the case `canon-delta.md` CD-1 promotes.
- VT-4 — `view_model.rs` units: `interpret`'s three `None` cases — a `Chosen`
  index no alternative of the drawn field has, a non-finite `AdjustedValue`, and
  a report whose variant does not match the drawn kind — and the in-kind cases
  beside them, including an `AdjustedText` no finite parse accepts keeping the
  number the field already held.
- VT-5 — `view_model.rs` units: the formatter on `f64::MAX` and on a number that
  spells inside the bound, and each spelling re-parsing under `f64::from_str` to
  the `f64` it came from.
- VA-1 — every new name in this phase is checked against the boundary suite's
  needles before it is written: `structure.rs`'s identifier-word match on
  `resolve` (`crates/goad-boundary/tests/checks/structure.rs:308`, subject
  `crates/goad/src`), the domain list (`vocabulary.rs:18-25`) and the purity path
  list (`purity.rs:17-27`).
- VA-2 — `boolean` is still the only kind the mapper draws: `undrawn_form` still
  returns a `FieldForm` for the other four, and no fixture has moved.

**Notes for the implementer**

Read `design.md` §5.2 end to end. It is the phase's specification and most of it
is about this one file pair. §5.5 I-C, I-D and I-G bind.

**The function is `interpret`, and no production line under `crates/goad/src` may
name the identifier `resolve`.** The instrument is
`no_production_line_in_the_renderer_names_the_identifier_resolve`
(`crates/goad-boundary/tests/checks/structure.rs:308`), verified live: it asserts
absence of the word over `crates/goad/src`, matching singular-or-plural on every
camel and non-alphanumeric boundary (`scan.rs:225-234`), with comments cut but
string literals kept. `just check` runs `cargo test --workspace`, so it is
already in the gate and needs no criterion of its own. This is
`prototype-handback.md` P-10, still true of the tree as it stands.

Three things §5.2 argues at length and an implementer will be tempted to shorten:

- **`Finite` carries no `Eq`.** `impl Eq for Finite {}` is *sound* and on its own
  restores the derives on `Edited` and `Command`, with nothing to warn anybody.
  It was written, compiled and deleted in the prototype. The rule is stated at
  the leaf so the trap cannot be reached from either door.
- **`interpret`'s `None` surface is three cases.** A `_ => None` arm satisfies a
  two-case reading and adds a failure the design never sanctioned; a mismatched
  `AdjustedText` is not an exception to *the text is recorded verbatim, always*,
  because that rule is about an in-kind report.
- **`as_drawn`'s `choice` arm is total without a lint exception**, which is why
  `DrawnKind::Choice` carries the first id beside the list. `.first()` is an
  `Option`, `unwrap_used` / `expect_used` / `indexing_slicing` are `deny`
  crate-wide, and `AlternativeId::new` is `pub(super)` so there is no fallback id
  to construct.

`slider_bounds` is **not** this phase's. It is private and has no caller until
`glass.rs` builds a `number` row, and a private fn landing a phase early is
`dead_code` at the gate. It lands in PHASE-08.

`glass.rs:203`'s irrefutable `let Edited::Checked(checked) = …` is the compile
error this phase is meant to produce. Its doc says so; replace it with the
kind-directed build rather than pattern-matching one variant out.

`draft.rs`'s own `#[cfg(test)] mod tests` is the shape for units here — the
precedent is `controller.rs`, `goad-shell/src/state.rs` and
`goad-semantics/src/schedule.rs`. Ids can only be *read* off a normalized view
(`OptionId::new` and friends are `pub(super)`), so a unit that needs an id parses
a fixture rather than minting one.

---

## PHASE-03 — the edit channel

**Objective:** the markup hands back a typed report of what one widget did, and
the host interprets it against the drawn field before anything reaches the draft.

**Surfaces:** `crates/goad/ui/app.slint`, `crates/goad/src/wire.rs`,
`crates/goad/src/install.rs`, `crates/goad/src/controller.rs`,
`crates/goad/tests/renderer/wiring.rs`, `crates/goad/tests/renderer/fields.rs`.

**Entry**
- EN-1 — PHASE-02/EX-1 … EX-9 discharged.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `app.slint` declares
  `struct FieldEdit { kind, checked, text, number, index }` and
  `callback edited(string, string, string, FieldEdit)`; the `CheckBox`'s
  `toggled` sends a `FieldEdit` literal naming only the fields it means.
- EX-3 — `Command::Edit` carries a `Reported`, and `Command` drops its `Eq`
  derive with `Edited`.
- EX-4 — `install.rs`'s `edited` closure maps a `FieldEdit` to a `Reported` and
  nothing else: no parsing, no fallback value, no refusal decided there.
- EX-5 — `controller::edit` interprets the report against the drawn field on the
  walk it already makes, and a `None` from `interpret` takes the existing
  `Refused::UnknownField` posture — reported, nothing recorded.
- EX-6 — every existing case that built a `Command::Edit` or an `Edited` is
  rewritten for the split: twelve in `tests/renderer/wiring.rs`, and the
  `install.rs` closure they exercise.

**Verification**
- VT-1 — `tests/renderer/wiring.rs`: the existing edit-refusal case still refuses
  each selector that fails and records nothing, now through a `Reported`.
- VT-2 — `tests/renderer/wiring.rs`: a report whose variant is not the drawn
  field's kind is refused `UnknownField` and records nothing — `interpret`'s
  third `None` case, asserted through the controller rather than only as a unit.
- VT-3 — `tests/renderer/fields.rs`: a `CheckBox` toggled through
  `invoke_accessible_default_action` still reaches the draft and still answers.
- VA-1 — no case was deleted to make the phase green, and none of the twelve
  rewritten cases lost an assertion in the move. Diff them one by one.

**Notes for the implementer**

Read `design.md` §5.2 from **What a widget reported is not yet what the draft
holds** to the end of the `Reported` discussion, and §5.5 I-D and I-G.

The asymmetry is the point: `Reported` is what a callback can honestly say, and
`Edited` is what the draft holds. A Slint callback cannot mint an
`AlternativeId` and cannot know the last representable number a numeric field
held, so both are interpreted where the retained presentation is. Keep the
closure dumb.

`Command` and `Edited` losing `Eq` is written down in §5.2 because the
alternative is meeting a derive error and hand-writing an `Eq` — which over
`AdjustedValue(NaN)` would claim a reflexivity the type does not have. The tests
that compare commands need `PartialEq` only.

`wiring.rs`'s own `TWO_FORMS` (`wiring.rs:1157`) is **not** `fields.rs`'s: it
carries a third field, `noted`, declared `text`. Four of the twelve sites read
that fixture. Leave it alone this phase — it still carries an undrawn field,
which is what `wiring.rs:1245`'s last refusal and `wiring.rs:1340`'s guard
assertion rest on. PHASE-05 migrates it.

`install.rs` is `pub` and in the library rather than in `main.rs` because a
`tests/` target cannot reach a binary crate. That constraint is unchanged.

---

## PHASE-04 — the instant, the `jiff` feature, and `clock.rs`'s doc

**Objective:** the host can turn a picked date and time into an instant and an
offset in the person's own zone, and the manifest change that makes the zone
readable is landed with the argument `POL-001` requires.

**Surfaces:** `crates/goad/src/instant.rs` (new), `crates/goad/src/lib.rs`,
`crates/goad/Cargo.toml`, `crates/goad-shell/src/clock.rs`.

**Entry**
- EN-1 — this plan is accepted. **No other phase is a prerequisite**; see
  §*Parallelism*.
- EN-2 — the phase runs in its own worktree if it is running beside another
  phase, and lands as its own commit.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `crates/goad/Cargo.toml` takes
  `jiff = { workspace = true, features = ["tz-system", "tzdb-zoneinfo"] }`. The
  feature goes on that member and **not** on `[workspace.dependencies]`.
- EX-3 — `src/instant.rs` carries `compose`, `decompose` and `today_local` with
  the signatures §5.2 gives, and is the only module in this crate that reads the
  clock or the system time zone.
- EX-4 — `compose` uses `Date::new` and `Time::new` and `DateTime::to_zoned`, and
  converts every integer with `i16::try_from` / `i8::try_from`. Neither
  `civil::date` nor `Date::at` appears: both panic out of range.
- EX-5 — `clock.rs`'s doc comment (`clock.rs:47-53`) is amended to state the
  three reaches §10 gives — the workspace build, where its rationale expires;
  the builds that exclude `crates/goad`, where the workaround still binds; and
  `cargo test -p goad-semantics`, which builds neither stratum above it. **No
  code change in that file.**

**Verification**
- VT-1 — `instant.rs` units: `compose` of an ordinary date and time yields the
  instant and the offset the system zone gives, and `decompose` inverts it.
- VT-2 — `instant.rs` units: each of `compose`'s four fallible steps answers
  `None` rather than panicking — an out-of-range integer into `Date`, one into
  `Time`, a civil date `Date::new` refuses, and a `DateTime` near the timestamp
  range's edge that `to_zoned` refuses.
- VT-3 — `instant.rs` units: a civil time inside a DST fold and one inside a gap
  both **succeed** under jiff's `Compatible` disambiguation, the fold taking the
  earlier occurrence and the gap shifting forward.
- VT-4 — the existing `goad-shell` clock cases still pass, and
  `cargo test -p goad-semantics` is still green — the gate's third command.
- VA-1 — the residue is argued, not merely taken: `design.md` §10 is the
  argument `POL-001` §Verification requires, and this phase cites it rather than
  restating it. Confirm §10 still says what the manifest now does.
- VA-2 — `TimeZone::system` no longer falls back to `Etc/Unknown` in a build of
  `crates/goad`: assert an offset that is not `+00:00` for a zone that has one,
  or state in the phase sheet why the CI zone makes that unassertable and what
  was checked instead.

**Notes for the implementer**

Read `design.md` §10 entire, `design-log.md` **D-35**, and `slice-009.md`
§*Governing canon*. `docs/policy/001-the-phase-gate.md` §Verification is the
canon that names this residue: no gate command rejects a feature switched on in a
dependency stratum 1 shares, which is why the decision must be argued instead of
checked.

Verified against the tree as it stands: `[workspace.dependencies]` declares
`jiff = { version = "0.2", default-features = false }` (`Cargo.toml:36`), and
`goad-semantics`, `goad-shell` and `goad` all take it unchanged. So it resolves
with no features today, exactly as §10 says.

`prototype-handback.md` P-4 is measured and narrows this phase: under today's
featureless `jiff`, `Offset`, `Offset::UTC`, `Offset::constant`,
`Timestamp::UNIX_EPOCH` and `display_with_offset` all compile and run. So
PHASE-02's `Edited::Picked` and `submitted`'s datetime arm needed no manifest
change, and what this feature gates is exactly `compose` and `today_local`.

`clock.rs`'s comment cites a `D25` that is **slice 005's**, not this design's §7
D25. Do not "correct" it to point here. Note also that `design-log.md` D-35 and
`design.md` §10 once carried the range `clock.rs:46-52` for that comment; `:46`
is blank and the comment is at `:47-53`. `design.md` is corrected; the log is
append-only and is not.

`draft.rs` declares no clock and no filesystem. Keep both reads in this module:
the clock in `today_local`, and the system zone in **both** `compose` and
`today_local` — three sites, two kinds. `today_local` cannot answer a *local*
date from the clock alone.

---

## PHASE-05 — `text`, and the debounce's delivery

**Objective:** a `text` field draws, a person can type into it continuously, and
everything they typed reaches the draft — on a timer while they keep typing, and
in the command that answers.

**Surfaces:** `crates/goad/ui/app.slint`, `crates/goad/src/pending.rs` (new),
`crates/goad/src/lib.rs`, `crates/goad/src/install.rs`,
`crates/goad/src/main.rs`, `crates/goad/src/wire.rs`,
`crates/goad/src/controller.rs`, `crates/goad/src/view_model.rs` (the mapper's
`Text` arm), `crates/goad/src/glass.rs` (the `Text` value arm),
`crates/goad/tests/renderer/{fields.rs,wiring.rs,mapper.rs,reception.rs}`,
`crates/goad/tests/event_loop_debounce/` (new), `crates/goad/Cargo.toml`
(one `[[test]]` entry).

**Entry**
- EN-1 — PHASE-03/EX-1 … EX-6 discharged.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `undrawn_form` returns `None` for `FieldKind::Text`; a `text` field
  draws a `LineEdit` bound to `values[field.slot].text`, carrying
  `accessible-description: field.id` and `enabled: !root.busy`, with the
  string-against-string guard on the epoch.
- EX-3 — `src/pending.rs` holds a map of pending edits keyed by (option, field),
  one `slint::Timer`, and nothing else. **Every entry carries the view it was
  made on.** The map does not branch on kind.
- EX-4 — the timer sends **one** `Command::Edit` per tick, carrying the entry's
  own view, and **re-arms while the map is not empty**. An entry leaves the map
  when the send that carries it is *enqueued*, not when it is accepted.
- EX-5 — `Wire::send` returns `bool`; every existing caller is unaffected and the
  back-pressure notice is raised and lowered exactly where it is today.
- EX-6 — `wire.rs` carries `PendingEdit { view, option, field, value }` and
  `Command::Choose { view, option, edits }`; the `chosen` callback drains the map
  into `edits` and sends **one** command.
- EX-7 — `controller` checks the `Choose`'s own identity first, then applies each
  carried edit through the walk `edit` already uses: a carried edit whose view is
  not the retained one is refused `SupersededView` and **the answer still goes**;
  one naming an option or field the retained view does not declare takes the
  `UnknownField` posture and **no answer is sent**.
- EX-8 — `main.rs` creates one `Rc` for the pending map before the callback table
  and clones it into `install`; `install`'s signature takes it, and so do the four
  existing call sites.
- EX-9 — the undrawn fixtures migrate off `text`: `A_DRAWN_AND_AN_UNDRAWN_FIELD`
  (`fields.rs:71`), `wiring.rs`'s `TWO_FORMS` (`wiring.rs:1157`), the two
  `mapper.rs` block cases (`mapper.rs:237`, `:255-256`, `:377`, `:423`) and the
  `reception.rs` diagnostic case (`reception.rs:762`) each name a kind that is
  still undrawn.

**Verification**
- VT-1 — `tests/renderer/fields.rs`: a `text` field draws a `LineEdit` findable
  by `field.id` as its accessible description, and the option still answers.
- VT-2 — `tests/renderer/wiring.rs`: `Command::Choose` carrying a pending edit
  applies it to the draft and then answers from a draft that includes it. At
  least one case carries an edit, so the single-send flush is asserted rather
  than assumed.
- VT-3 — **AC-4, the element half.** `tests/renderer/fields.rs`:
  `set_accessible_value` on each of two text fields, then the option control's
  default action; the draft holds both texts, and the `init` counter is unchanged
  — the element was not destroyed while it was being typed into.
- VT-4 — `tests/renderer/wiring.rs`: a carried edit naming a superseded view is
  refused `SupersededView`, is reported once rather than per edit, and the answer
  still goes. A carried edit naming a field the retained view does not declare is
  refused and **no answer is sent**.
- VT-5 — **AC-4, the timer half.** The new `event_loop_debounce` target: one
  `set_accessible_value` on a text `LineEdit`, then let the loop run past 150 ms
  **without answering**; exactly one `Command::Edit` reaches the controller and
  the draft holds the text.
- VT-6 — the same target, negative control: with the timer's re-arm removed, a
  second entry made in the same window never arrives.
- VA-1 — the entry leaves `pending.rs` on the *enqueue*. Read the two exit paths
  and confirm a `Full` send clears nothing: a refusal has already been reported
  and the guard corrects the widget, whereas a `Full` send delivered nothing.
- VA-2 — the migrated fixtures still assert what they asserted: each one's
  undrawn field is still undrawn, and no case's guard assertion became vacuous in
  the move. `prototype-notes.md` P-13.

**Notes for the implementer**

This is the largest phase. Read `design.md` §5.1 (`pending.rs`, and the two
asymmetrical exits), §5.2 (`wire.rs` — `Choose` carries the flush, and `send`
reports), §5.4 (*A keystroke*, *An answer*, *A new view*) and §5.5 I-H. Attack it
in that order: the map and its delivery rule first, then the two exits, then the
markup, then the fixture migration, then the cases.

**Why the flush travels inside `Choose` rather than as sends before it.** The
command channel holds one (`main.rs:86`, verified) and `serve` shares the UI
thread through `spawn_local`, so a Slint callback — synchronous, no await —
cannot let `serve` drain between two sends. The second `try_send` of any flush
does not merely risk `Full`, it always gets it. This is not a preference and not
a cost of keying the map; even a single held edit plus `Choose` is two sends.

**Why every entry carries its view.** The map is keyed by strings a replacement
view is free to reuse, and an entry outlives the view that produced it. The
`edited` callback already receives the view as its first argument. One rule at
three sites (I-H): shown only against its own view, sent by the timer under its
own view, drained into a `Choose` carrying that view alongside. PHASE-06 adds the
*shown* site; this phase owns the other two.

The timer's re-arm is measured, not only read: `start_or_restart_timer` preserves
the `being_activated` flag and replaces the callback
(`i-slint-core-1.17.1/timers.rs:348-372`), and the prototype reproduced it under a
real loop (`prototype-handback.md` P-11). VT-6 is the negative control that keeps
that honest.

`set_accessible_value` on a `LineEdit` assigns `text` and calls `edited` from
inside the markup (`widgets/fluent/lineedit.slint:16`), so it reaches no
`TextInput` insertion logic. That is §9's principal driver and it bypasses
`input-type` exactly as a paste does — which matters more in PHASE-08 than here,
but the mechanism is the same one.

**The fixture migration is per-phase, not once.** `prototype-notes.md` P-13:
five fixtures carry a `text` field *because `text` was the undrawn kind*. Each is
repaired by one word here, again in PHASE-07, again in PHASE-08, and in PHASE-09
there is no kind left to move to and the cases are deleted rather than repaired.
The risk is not the edit; it is a phase that migrates without noticing it has
deferred a deletion. Record the migration in the phase sheet each time.

**`Pending` is taken.** `controller.rs:385` already declares a private
`enum Pending` — the exchange a command turns into. §8 R10 calls the pending
map's type `Pending` in passing; it is in a different module so nothing fails to
compile, but two private types of that name in one crate is a readability trap
with an `Rc` around one of them. Name the new one for what it holds.

No order is promised over the carried edits, and none should be: the keys are
distinct by construction so any order yields the same draft, and the callback
holds `(option, field)` keys with no declaration order to derive one from.

---

## PHASE-06 — the overlay

**Objective:** a present that lands inside the debounce window shows what the
person typed rather than the draft's older value, so *the host did not record
this* and *the host has not recorded this yet* stop being the same thing to the
guard.

**Surfaces:** `crates/goad/src/glass.rs`, `crates/goad/src/main.rs`,
`crates/goad/tests/renderer/harness.rs`,
`crates/goad/tests/renderer/fields.rs`, `crates/goad/tests/event_loop/closing.rs`,
`crates/goad/tests/event_loop_schedule/scheduling.rs`,
`crates/goad/tests/event_loop_overlay/` (new), `crates/goad/Cargo.toml` (one
`[[test]]` entry).

**Entry**
- EN-1 — PHASE-05/EX-1 … EX-9 discharged.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `SlintGlass::new` takes a clone of **the same** `Rc` `install` was
  given; `main.rs` creates it once, before both. The four call-site pairs —
  `main.rs`, `tests/renderer/harness.rs`, `tests/event_loop/closing.rs`,
  `tests/event_loop_schedule/scheduling.rs` — are each given one value, not two.
- EX-3 — a field's displayed value is the draft's, overlaid with `pending.rs`'s
  entry for that (option, field) **where the entry was made on the view being
  presented**. The overlay goes through `interpret`, not through a second
  `Reported → FieldValue` mapping. Where `interpret` refuses the entry the
  draft's value stands.
- EX-4 — `glass.rs`'s doc stops claiming the value channel has one source: it is
  derived from `Prepared` and `pending.rs`, and there is still no cache and
  therefore still no invalidation rule.

**Verification**
- VT-1 — **AC-6.** The new `event_loop_overlay` target:
  `set_accessible_value` on a `LineEdit` whose `Wire` reaches no controller, then
  let the loop run past the debounce so the entry is sent and **leaves**
  `pending.rs`, then a present. The convergence counter increments, the widget
  holds the draft's value again, and the `init` counter is unchanged. Both halves
  are needed: while the entry is still held the host *does* hold the value and the
  widget correctly stands.
- VT-2 — the same target, negative control: with the overlay removed, a present
  landing **inside** the window reverts the widget and the case goes red.
- VT-3 — the same target: two text fields edited inside one window, then two
  ticks without answering. Both values reach the draft, and **neither widget is
  reverted at any point** — the convergence counter stays at zero across both
  ticks.
- VA-1 — the overlay is wired to one `Rc` and not two. §8 R10 is the trap: two
  `Pending` values leave every case green while measuring nothing. Check by
  reading the construction, and confirm VT-1 fails when `install` is not called.
- VA-2 — nothing re-enters: `present` reads the map, the `edited` callback
  writes it, and the timer and `chosen` take from it. `present` runs inside
  `serve`'s task and never from inside a widget callback; neither the timer nor
  `chosen` presents. Confirm no borrow is held across another.

**Notes for the implementer**

Read `design.md` §5.3 (*A field's value is the draft's, overlaid*, and *Nothing
re-enters*), §5.5 I-H, and §7 D26 and D27.

§8 **R10** is this phase's whole risk and it is silent: a test that constructs
`install`'s half and `SlintGlass`'s half separately gets an overlay that never
overlays anything, and every case stays green. `main.rs:85-101` already has the
construction order — the callback table at step 6, the glass at step 7 — so the
`Rc` is created once before both and cloned into each. No reordering.

The present in the middle of a keystroke is not exotic: `serve` presents at the
top of every iteration (`controller.rs:738-739`, verified), so *any* command
handled inside the window produces one — a tray check, a diagnostics toggle, an
evaluation finishing. That is what makes this phase necessary rather than
defensive.

The overlay adds the third of I-H's sites. A pending entry from a **replaced**
view is not overlaid, because the ids it is keyed by are strings the new view is
free to reuse. Nothing else clears the map: the timer sends each stale entry under
its own view, the controller refuses it `SupersededView`, and the entry leaves
because the send was enqueued.

Two claims in one loop target is the working limit. Order the assertions so an
injection aimed at the second is not masked by the first, and if an injection pass
cannot discriminate them, split the target rather than accepting a case that
cannot go red for its own reason.

---

## PHASE-07 — `datetime` and the two pickers

**Objective:** a `datetime` field draws a button showing its value or *not set*,
a person picks a date and then a time, and one conforming instant with the offset
they picked in reaches the draft.

**Surfaces:** `crates/goad/ui/app.slint`, `crates/goad/src/view_model.rs` (the
mapper's `DateTime` arm), `crates/goad/src/glass.rs` (the `DateTime` value arm
and the seed slots), `crates/goad/src/install.rs`,
`crates/goad/tests/renderer/{fields.rs,mapper.rs,wiring.rs,reception.rs}`.

**Entry**
- EN-1 — PHASE-06/EX-1 … EX-4 discharged.
- EN-2 — PHASE-04/EX-1 … EX-5 discharged. `compose`, `decompose` and
  `today_local` exist and the `jiff` feature is landed.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `undrawn_form` returns `None` for `FieldKind::DateTime`; the field draws
  a `Button` whose text is `values[field.slot].text` — the composed value, or
  *not set* for a field nobody has picked. The button needs **no** guard: it never
  assigns its own text, so its binding is never destroyed.
- EX-3 — `FieldValue` gains `date: Date` and `time: Time`, written every present
  from `instant::decompose` of the draft's `Picked`, or from
  `instant::today_local()` where the field has not been picked.
- EX-4 — the window root carries one `Date` and one `Time` seed property and a
  `picking` record of (view, option, field). The button's handler writes the two
  seed properties from `values[field.slot]` and calls `show()`; each popup
  **binds** to a seed at its own declaration site. A popup's properties cannot be
  assigned from an enclosing handler — that is a compile error, measured.
- EX-5 — exactly one `edited` per completed pick: the date picker's `accepted`
  stashes the date and opens the time picker, the time picker's `accepted`
  composes and reports once. `canceled` at either picker abandons the whole edit
  and records nothing. Both popups carry `close-policy: no-auto-close`.
- EX-6 — where `compose` answers `None`, nothing is recorded and the button still
  shows what it showed.
- EX-7 — the undrawn fixtures migrate off `datetime` where they had moved to it;
  see PHASE-05/EX-9.

**Verification**
- VT-1 — `tests/renderer/fields.rs`: an untouched `datetime` field's button reads
  *not set*, and answering the option submits `1970-01-01T00:00:00+00:00`. The
  screen and the wire disagree here on purpose (§7 D1, D2) and this is the case
  that fixes it.
- VT-2 — `tests/renderer/fields.rs`: the picker chain, driven by
  `invoke_accessible_default_action` throughout — the button to open, a calendar
  day cell (`accessible-role: button`, the day number as its label), then the
  dialog's `OK` `StandardButton`, then the same again on the time popup. The
  draft holds one `Picked` and the button shows it.
- VT-3 — **the re-seed.** `tests/renderer/fields.rs`: a field that has been
  picked, reopened, opens its fresh popup on **that field's** retained date and
  time rather than on today. No pointer, so no layout dependency.
- VT-4 — `tests/renderer/fields.rs`: cancelling either picker records nothing and
  leaves the button reading what it read.
- VA-1 — if VT-2 or VT-3 cannot find the popup under `init_no_event_loop`, the
  row moves to a loop target rather than being weakened. `find_first` / `find_all`
  walk `active_popups` (`search_api.rs:291-312`) and the testing backend's own
  `test_popups` runs there, so the capability is expected — but no case in this
  repository has needed it yet. Record which way it went.
- VA-2 — no `datetime` field holds half a pick at any point. Read the state
  machine and confirm the only `edited` call is on the time picker's `accepted`.

**Notes for the implementer**

Read `design.md` §5.4 (*Picking a datetime*, and the three paragraphs after it),
§5.2's `datetime` rows and as-drawn bullet, and §7 D4, D5, D19, D21.

**A popup does not live between opens.** `show-popup` compiles to a fresh
`::new()` on every show and the closed instance is dropped from `active_popups`;
both measured. Three things follow: a rewritten seed is picked up by a **fresh
binding**, not by a `changed` handler inside the widget; an in-popup selection
cannot survive to be seen again; and no pick can leak into the next field's
picker. The seed is justified by the field that *has* been picked, not by
leakage — the first draft of the design had that backwards.

A `PopupWindow` cannot be repeated or conditional, so both pickers are root
singletons and a `datetime` field has no inline control. That is a measured
compiler constraint, not a choice.

A DST fold or gap **succeeds** under jiff's `Compatible` disambiguation — the
fold takes the earlier occurrence, the gap shifts forward — so
`2024-03-10 02:30` in New York composes to `03:30-04:00`. The button then shows
the composed value, so a person sees the shift rather than being deceived by it
(§7 D19). PHASE-04/VT-3 already asserts the arithmetic; this phase's obligation
is that the button shows the result.

`glass.rs` calls `instant::today_local()` once per present for the seed slots of
any unpicked `datetime` field. That is a clock read inside `present`, and it is
deliberate: threading it through `Frame` was rejected because `Frame` is built
where a clock read has a failure path a present cannot report, and what a picker
opens on is a presentational default rather than a fact the controller retains.

Seeding from `as_drawn` opens an untouched field's picker at 1970 — D-6's
sentinel leaking into the one place D-6 chose it to keep out of. Seed from
`today_local`.

`tests/renderer/tree.rs::labels_in_option` scopes by the groupbox role and one
case asserts exactly one groupbox per option with fields. Nothing this phase
draws may bind `accessible-role: groupbox`.

---

## PHASE-08 — `number` and its two controls

**Objective:** a `number` field draws a slider where a slider can actually be
operated over its declared range and a numeric text field otherwise; both submit
a finite number, and neither writes over a person mid-entry.

**Surfaces:** `crates/goad/ui/app.slint`, `crates/goad/src/view_model.rs` (the
mapper's `Number` arm and `slider_bounds`), `crates/goad/src/glass.rs` (the
`Number` value arm and the row's control fields),
`crates/goad/tests/renderer/{fields.rs,mapper.rs,wiring.rs,reception.rs}`,
`crates/goad/tests/event_loop_numeric_guard/` (new), `crates/goad/Cargo.toml`
(one `[[test]]` entry).

**Entry**
- EN-1 — PHASE-07/EX-1 … EX-7 discharged.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `undrawn_form` returns `None` for `FieldKind::Number`; `FieldRow` gains
  `slider: bool` and `minimum` / `maximum` / `step`, read only where `slider`.
- EX-3 — `slider_bounds(&NumberRange) -> Option<(f32, f32)>` is the **only** site
  that chooses a `number`'s control, and answers `Some` only where all three of
  §5.2's clauses hold: both bounds present and exactly `f64` → `f32` → `f64`
  round-tripping; a finite, strictly positive span; and a step of
  `(maximum - minimum) / 100` that **moves the value**.
- EX-4 — the numeric `LineEdit` is `input-type: decimal`, sends its edit as
  **text**, and the host parses it with `f64::from_str` and repairs nothing. The
  text is recorded verbatim, always; the number is replaced only where the parse
  yields a finite `f64`.
- EX-5 — the `Slider` binds `changed`, debounced, and nothing else. `released` is
  not bound. Its `step` is `(maximum - minimum) / 100`.
- EX-6 — the numeric `LineEdit`'s guard compares `self.text` against
  `values[field.slot].text` — string against string — and converges unless they
  are equal, **or** the widget is empty and the held number is zero.
- EX-7 — the exception in EX-6 is re-measured against the overlay and either kept
  with the measurement recorded, or removed with the measurement recorded. It is
  not removed by argument.
- EX-8 — the undrawn fixtures migrate off `number` where they had moved to it;
  see PHASE-05/EX-9.

**Verification**
- VT-1 — `view_model.rs` units: `slider_bounds` refuses equal bounds, an `f32`
  span of infinity, and the `2^100` ulp case where a finite positive step is
  still too small to move the value; and accepts an ordinary range.
- VT-2 — `tests/renderer/fields.rs`: a `number` whose range admits a slider draws
  one, and a `number` whose range does not draws the text control. Both submit a
  number.
- VT-3 — **AC-9.** `tests/renderer/fields.rs`: a `number` with no bounds draws
  the text control, `set_accessible_value` on it reaches the draft, and the
  submitted value is a JSON number. No range appears anywhere that the backend
  did not send.
- VT-4 — `tests/renderer/fields.rs`: a numeric text the parse refuses.
  `set_accessible_value("12/25")` — a driver that bypasses `input-type` exactly as
  a paste does — leaves `12/25` on screen and the draft's number the one it
  already held.
- VT-5 — `tests/renderer/fields.rs`: a `number` declaring `f64::MAX` as its `min`
  draws a `LineEdit` carrying the `{:e}` spelling rather than 309 characters, and
  submits the `f64` it came from. PHASE-02/VT-5 is the formatter's own unit; this
  is the element half.
- VT-6 — the new `event_loop_numeric_guard` target, negative-controlled:
  `set_accessible_value("")` on a numeric `LineEdit` the host holds as `0`, then a
  present **inside** the window. The widget stays empty. Run again with the
  exception removed — if it still passes, the overlay subsumes the exception and
  it goes; if it fails, A-2 keeps it.
- VA-1 — nothing non-finite can reach the wire: the only constructor of a
  submitted number is `Finite::new`, and `interpret` refuses a non-finite
  `AdjustedValue` before that. Two places, neither of them the boundary type.
- VA-2 — no `f64` crosses the markup boundary as a `float` except a `Slider`'s
  own value and the bounds it is drawn over, all of which are `f32`-exact by
  EX-3's predicate.

**Notes for the implementer**

Read `design.md` §5.2 from **A number at the markup boundary** to the end of
**Converging on recency**, and §7 D13, D16, D17, D23. This is the longest
stretch of the design and most of it is about one control.

**The guard's comparand has been wrong three times**, twice on reasoning and once
corrected by measurement. That is why EX-7 exists: the exception stays in until VT-6 has been run against the overlay *with the exception removed*, and is
removed only on that result. Do not remove it because the argument says the overlay
subsumes it — the argument is probably right and has been probably right before.

`input-type: decimal` gates typing and nothing else. The paste path performs no
validation at all, and neither does `set_accessible_value`, which is §9's
principal driver. So every numeric case this phase writes drives the *unvalidated*
path by default, and the host may never reason from a class the control appears to
enforce. VT-4 is the case that keeps the absence of a repair rule honest: a rule
that replaced one foreign character with `.` would read `12/25` as `12.25`.

The decimal separator is `.` for the life of every process this workspace builds
— four write sites, none of them reachable here — so the parse is `f64::from_str`
on the raw text and nothing else. §8 R11 records the configuration that would arm
the hazard; this phase does not mitigate it.

`slider_bounds`'s third clause states operability directly rather than
approximating it: `increment()` is exactly `set-value(value + step)` and
`set-value` returns immediately when the result equals the value it already
holds. A step below half an ulp freezes the slider even though the bounds
round-trip exactly and the span is finite and positive. The clause subsumes the
positivity test it replaces.

A `Slider` binds `changed` and not `released` because Slint's accessibility
`set-value`, `increment` and `decrement` all route through `set-value`, which
raises `changed` only. A `released`-only binding is deaf to an assistive
technology and to every test tier. `released` is not bound as a flush either: it
has no interface to travel on.

---

## PHASE-09 — `choice`, and the retirement of `FieldForm`

**Objective:** the fifth kind draws, `Undrawn::FieldForm` becomes
unconstructible, every consumer that assumed otherwise is rewritten, and the
slice's five-kind acceptance criteria are discharged.

**Surfaces:** `crates/goad/ui/app.slint`, `crates/goad/src/view_model.rs`,
`crates/goad/src/diagnostics.rs`, `crates/goad/src/draft.rs` (one doc comment),
`crates/goad/src/glass.rs` (the `Choice` value arm),
`crates/goad/tests/renderer/{fields.rs,mapper.rs,wiring.rs,reception.rs,tree.rs}`,
`crates/goad/tests/event_loop_reassert/` (a second claim).

**Entry**
- EN-1 — PHASE-08/EX-1 … EX-8 discharged. All four other kinds draw.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `undrawn_form` returns `None` for `FieldKind::Choice`; the field draws a
  `ComboBox` over `FieldRow.alternatives` (labels, in declared order), reporting
  its `current-index`, with the guard comparing `self.current-index` against
  `values[field.slot].index`. An index out of range takes the
  `Refused::UnknownField` posture.
- EX-3 — **AC-1.** A view whose option carries a `text`, a `number`, a `choice`
  and a `datetime` field draws all four in declared order alongside a `boolean`.
- EX-4 — **AC-2.** Answering that option sends a `respond` whose `values` carry
  the JSON type `R-57` names for each kind.
- EX-5 — **AC-3.** The map carries a value for exactly the fields the host drew of
  the option answered, and no others.
- EX-6 — **AC-7.** `FieldForm` is an empty enum rather than deleted;
  `undrawn_form`'s exhaustive match over the canonical `FieldKind` still makes a
  sixth kind a compile error; `Undrawn::GroupHint` and the two content forms still
  report.
- EX-7 — every site §5.1's table names is rewritten, none by deletion-of-
  convenience: `diagnostics.rs`'s undrawn line stops naming a subset that no
  longer exists; `draft.rs:82`'s and `view_model.rs:31`'s docs stop describing a
  destination that has to be re-created before it can be reached;
  `mapper.rs:195-203` and `:290-327, 375-399`; `wiring.rs:1245`, `:1304`,
  `:1340`; `fields.rs:67-71` and `:597-632`.
- EX-8 — `wiring.rs:1245`'s `Refused::UnknownField` is still asserted, reached
  from a field id no view declared rather than from a real field's kind.
- EX-9 — `canon-delta.md` CD-2's three changes are *true of the tree*: the `R-57`
  row's cases exist, the `R-58` row's first half is genuinely unobservable with no
  substitute construction, and no option field goes undrawn on account of its
  kind. **CD-2 is not applied to `SPEC-001` here** — promotion is audit's, with
  explicit endorsement.

**Verification**
- VT-1 — `tests/renderer/fields.rs`: a `choice` field draws a `ComboBox` whose
  model is the alternatives' labels in declared order.
- VT-2 — **AC-8.** `tests/renderer/fields.rs`: `invoke_accessible_expand_action`,
  then `mock_single_click` on the `ListItem` whose accessible label is the
  alternative's; the submitted value is the **alternative's id**, and a view whose
  field id equals an option id still answers correctly.
- VT-3 — **AC-1.** `tests/renderer/fields.rs`, element queries only, nothing
  operated: all five kinds draw, in declared order, and a `number` outside
  `slider_bounds` draws the text control.
- VT-4 — **AC-2, untouched.** `tests/renderer/fields.rs`, reading the child
  process's own request log: the option control's default action alone, no control
  operated. The five as-drawn values arrive with the JSON type `R-57` names,
  including the `datetime` epoch's exact spelling. This is the only case that
  asserts what `canon-delta.md` CD-1 promotes.
- VT-5 — **AC-2, operated.** The same log, each control driven by its own driver
  first. Same per-kind typing for values a person produced.
- VT-6 — **AC-3.** `tests/renderer/wiring.rs`, the existing `R-58` cases extended
  to a form of five kinds.
- VT-7 — **AC-7.** A `view_model.rs` unit: `undrawn_form` still matches
  `FieldKind` exhaustively, and `Undrawn` still reports a `group` hint it cannot
  read.
- VT-8 — the `choice` re-assert, appended to the `event_loop_reassert` target:
  `invoke_accessible_expand_action` + `mock_single_click` with no `Wire`
  installed, then a present. The convergence counter increments and
  `current-index` returns to the draft's. This is the only measurement of A-5.
- VA-1 — §8 **R9**: if `mock_single_click` cannot be landed on a laid-out popup
  under `init_no_event_loop`, VT-2 moves to the loop target where VT-8 already
  sits — same driver, same assertion — and nothing else changes. The injection
  pass is what proves the row can go red rather than passing vacuously. Record
  which way it went.
- VA-2 — §8 **R3**: `FieldForm` was not deleted for being empty. Deleting it also
  deletes the sixth-kind compile error.
- VA-3 — every rewritten case still asserts something the defect it guards would
  survive (`docs/memory/a-green-test-can-assert-a-proxy.md`). The two cases
  `canon-delta.md` CD-2 names lose their premise; say so in the phase sheet rather
  than renaming them into something that looks equivalent.
- VH-1 — **AC-10.** A person runs the software and answers a form containing all
  five kinds: the caret mid-word, both pickers, and a slider drag across a
  present. `docs/memory/getting-eyes-on-the-running-host.md` has the launch and
  screenshot mechanics. Recorded in `audit.md` under Evidence, naming what was
  observed.

**Notes for the implementer**

Read `design.md` §5.1's consumer table entire, `canon-delta.md` **CD-2**, and
`prototype-notes.md` **P-13**. This phase is where P-13 falls due: the fixture
that has been migrating since PHASE-05 has nowhere left to go, and the cases
built on it are deleted rather than repaired.

**What is lost is written down, not worked around.** `R-58`'s MUST NOT prohibits
two things — submitting a value for a field the host did not draw, and submitting
a field of an option it is not answering. After this phase the **first is
unobservable**: no view can carry an undrawn field, so no case can watch its value
stay off the wire. There is no substitute construction — a `group`-hint field is
still *drawn*, and every surviving `Undrawn` variant is body-level. The second is
untouched and stays asserted by the same fixture's two options sharing the field
id `read`. Do not invent a case that looks like the first one.

`AlternativeId::new` is `pub(super)` in `goad-semantics`, so this crate cannot
mint one — it can only clone one off a view it drew. That is how AC-8 and `R-52`
become facts about the types rather than rules someone follows. The `ComboBox`
reports an index and `interpret` maps it against the drawn field's alternatives,
on the walk `controller::edit` already makes. That arm was written and unit-tested
in PHASE-02.

A view may legally carry a field id equal to an option id (`R-52`), and both the
field widget and the option `Button` then answer to the same accessible
description. Existing cases filter by element type; VT-2's must too.

VT-8 appends a second claim to a target PHASE-01 created. Two claims in one loop
`#[test]` is the working limit and they should be contrasting — AC-5 asserts the
convergence counter stays at zero, VT-8 asserts it moves. Order them so an
injection aimed at one is not masked by the other, and split the target if an
injection pass cannot discriminate them.

`canon-delta.md` is the slice's working authority and stays in the slice folder.
This phase makes it *true*; audit promotes it.

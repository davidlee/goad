# Prototype notes — slice 009

**This is not the slice.** It is a throwaway end-to-end build of `design.md`
as it currently stands, made on branch `slice-009-prototype` in a worktree at
`/home/david/dev/goad-009-proto`, while the design's adversarial review is
still open.

## Why it exists

Two aims, given by the user:

1. **Make progress.** The design has been through three review rounds and the
   plan has not started. Building the thing surfaces what reading it does not.
2. **Short-circuit design conversations by testing the assumptions.** Several
   of the remaining arguments are measurable (`slice-009.md` §Before design
   starts says so in as many words, and D-19 already made that trade once —
   the spike overturned the framing it was built to confirm).

The code is **referenced, not used**. Nothing here is promoted; the real slice
re-derives from the plan. What travels back into the slice is the *findings*
section of this file, and any canon or design change the user endorses.

## What "the current design" means here

**It moves, and the prototype re-synced once already.** This branch was cut at
`a01ae6a`, where `design.md` was current as of round 2's integration and the
thirteen round-3 findings were dispositioned, user-confirmed (D-23, D-24) and
**not** integrated. P1a was therefore built against `design.md` *plus* those
thirteen, stated in `prototype-delta.md`, on the ground that they are decisions
the user had already taken and building the superseded shape would test nothing.

Round 3's integration then landed in the main repository at `c065a60`, and it is
merged into this branch. So from **P1b onward, `design.md` in this worktree is
the authority and `prototype-delta.md` is a historical record** — the thirteen
are in the design now, and reading them twice is how a stale one gets applied.

Two things the delta still carries that `design.md` does not, and both are worth
keeping:

- §E's two **disagreements with `notes.md`'s "citations known bad"** — the claim
  that §9's `set_accessible_value` "appears nowhere here" is false as written,
  and F-42's location line looks correct rather than inverted. Neither was
  reconciled silently, and both are live against the slice.
- the record of which rules P1a implemented in their delta form rather than
  §5.2's, which is finding P-5.

The integration also raised **F-46 … F-49**, dispositioned and integrated but not
yet user-confirmed. The prototype builds them, for the same reason it built the
thirteen. F-48 in particular reached P1a mid-phase: `Command` and `Edited` keep
`PartialEq` and drop `Eq`, and the hand-written `Eq` that compiles is unsound
over `AdjustedValue(NaN)`.

## Rigour, deliberately uneven

The user set the standard: "as rigorous or loose as you think makes sense
given the aims." The split taken:

| area | standard | why |
|---|---|---|
| the mechanism the design turns on — two channels, the epoch, the guard, `resolve`, `pending`, the `Choose` flush | **full**: made to work, exercised by hand and by a probe | this is what the prototype exists to test |
| the pure functions the design specifies by contract — `slider_bounds`, the number parse rule, `compose`, `as_drawn` | **full unit coverage** | cheap, and each encodes a design claim that can be wrong |
| `cargo build --workspace` and `cargo clippy` | **green, always** | a red build is hostile to the next agent |
| the existing `tests/renderer/` suite | **allowed to rot** | §5.1's table says ~20 sites need rewriting for the `FieldForm`/`Reported` changes. That is real work for the slice and tests nothing about the design. Broken modules are disabled in `tests/renderer/main.rs` and listed below |
| the full §9 validation table — injection passes, tiering, negative controls | **not done** | it is the plan's job and it is most of the slice's cost |
| canon, `canon-delta.md`, the ledger | **untouched** | a prototype amends nothing |

## Disabled legacy test modules

<!-- Filled in as phases land. One line each: module, why, what it asserted. -->

| module | state | why |
|---|---|---|
| — | — | **None, through P1b.** |

**The suite did not need to rot at P1b, and that is itself worth recording.**
§5.1 priced ~20 sites as needing rewriting and the charter allowed the modules
that could not be repaired mechanically to be disabled. In the event every one
of the 187 tests is green and none is disabled; the repairs were of three
kinds, and all three are mechanical:

- **`Edited` → `Reported` at the `controller::edit` call sites** (twelve in
  `wiring.rs`) — a token substitution and a borrow.
- **The two channels at the fixture sites** (`tree.rs`, `sizing.rs`,
  `fields.rs`, `wiring.rs`) — a `FieldRow { checked }` becomes a row carrying
  `kind`/`slot` plus a `FieldValue` written to `set_values`, and a reader that
  was `row.checked` becomes a join through the slot. `tree.rs` grew one
  fixture helper for the value channel; nothing else needed new shape.
- **`text` leaving the undrawn set** — the five fixtures that used a `text`
  field *because it was undrawn* now say `"kind": "number"`.

That third one is a stop-gap and P-13 says so: the fixture has to migrate to a
kind the renderer has not reached yet at every phase, and after P4 there is no
kind left to migrate it to.

## Phases

| phase | what | state |
|---|---|---|
| P1a | the pure core: `Finite`, `Edited`'s five variants, `Reported`, `DrawnKind`, `as_drawn`, `resolve`, `slider_bounds`, the number parse rule — unit-tested, nothing drawn | **done** |
| P1b | the split: two channels, epoch, guard, `pending`, and `text` end to end — the first phase built against the **integrated** design | **done** |
| P2 | `number` — both controls, over P1a's `slider_bounds` and parse rule | pending |
| P3 | `choice` — `ComboBox`, `Chosen(AlternativeId)`, index resolution | pending |
| P4 | `datetime` — `instant.rs`, the jiff features, the two pickers | pending |
| P5 | demo backend of all five kinds, run it, and the harvest | pending |

Vertical rather than by layer, because every horizontal cut leaves a build
that cannot be run and therefore cannot be looked at, which is the one thing
`docs/AGENTS.md` §Tiers says a slice may not close without.

## Probes

Two targets, added at P1b. Neither is the §9 validation table and neither
tries to be.

| target | tier | what it watches |
|---|---|---|
| `crates/goad/tests/prototype/` | `init_no_event_loop` | the two channels and I-B's slot numbering; I-F's epoch; the overlay; I-H's staleness rule at all three sites; the answer's one-send flush and §5.2's held-or-sent table. 14 cases |
| `crates/goad/tests/event_loop_debounce/` | `init_integration_test_with_system_time` | the one claim only a real loop can show: the timer fires, delivers **one** entry per tick, and re-arms from inside its own callback. 1 case, one target, per `docs/memory/slint-testing-backend-initialises-once-per-process.md` |

**Every one of the 15 was driven red before it was kept.** Sixteen breaks were
applied to `src/` one at a time, each reverted and the revert confirmed by
`git diff`: rows written on every present, rows never written, the overlay
removed, `Pending::shown` ignoring the view, `flush` clearing whatever the send
did, every kind debounced, no kind debounced, `chosen` sending the edits
separately, the epoch not bumped, every slot numbered zero, a stale carried
edit returning `Err`, a carried edit applied under the command's view, an
undeclared carried field tolerated, the command's identity checked last, the
timer not re-arming, and `text` sent where it is raised. Every probe went red
under at least one of them, and no break left the target uncompiled
(`docs/memory/negative-control-must-compile.md`).

## Findings

<!-- The output that matters. One entry per thing the build taught that
     reading the design did not. Each says: what the design assumes, what was
     observed, and what it would cost the design to be wrong.
     P-n ids, immutable. -->

### P-1 — `resolve` has a third failure class, and F-42 named it without placing it

*From P1a.*

**What the design assumes.** F-42's disposition: "`resolve`'s `None` surface is
stated **in full** … There are **two**: an index no alternative has, **and** a
non-finite `AdjustedValue`."

**What the build observed.** F-42's own *Blocker* paragraph identifies a third
class in passing — "leaving the non-finite slider report **(and other
report/kind mismatches)** to be invented by an implementer" — and then the
disposition places two of them and not that one. `resolve`'s signature admits
every (`Reported`, `DrawnKind`) pair, so writing it means answering for
`AdjustedText` against a `DrawnKind::Boolean`, `Chosen` against a
`DrawnKind::Text`, and so on. Six reports against five kinds is thirty pairs,
of which six are in-kind.

Two of those arms cannot be `None` without contradicting a rule the design
states elsewhere. `AdjustedText`'s rule is "the text is recorded **verbatim,
always**" (§5.2), so a mismatched `AdjustedText` still has to produce an
`Edited::Adjusted`; the only open question is which number it keeps. And
`Chosen` against a non-`choice` kind lands in case 1 — "an index no
alternative has" — **only if** the implementation answers *no alternatives*
for the other four kinds. That is a coincidence of spelling, not something the
disposition guarantees: an implementation that matched on `DrawnKind::Choice`
and left the rest to a `_ => None` arm would satisfy the same two-case
sentence while adding a failure the design never sanctioned.

**What was taken here.** A report whose kind is not the field's never produces
`None`. `Chosen` resolves against a slice that is empty for every kind but
`choice`, so it falls into case 1 by construction rather than by luck;
mismatched `AdjustedText` keeps `Finite::ZERO`, the value an untouched
unbounded `number` would have held. Both are named in the code.

**What it costs the design to be wrong.** F-42's whole argument is that "a step
left off the list becomes an `unwrap`" — and the list is one class short of the
surface the signature admits. The cost is not a crash; it is that two
implementers writing §5.2 as it stands produce different `None` surfaces and
both pass the review. One sentence in §5.2 fixes it: *a report whose kind is
not the field's is not a third `None` case — it takes the most conservative
in-kind answer*. Or `resolve`'s signature narrows so the pairs cannot be
formed, which is a larger change and probably not worth it.

### P-2 — `as_drawn`'s `choice` arm is not total under this crate's lint table

*From P1a.*

**What the design assumes.** §5.2's as-drawn bullet: "`choice` → `Chosen`(first
alternative's id). Always defined: `Alternatives::new` rejects an empty list
(`canonical.rs:362`)."

**What the build observed.** True of the protocol, and invisible to the
compiler. `Alternatives::as_slice().first()` is an `Option`, `unwrap_used`,
`expect_used` and `indexing_slicing` are all `deny` crate-wide
(`clippy.toml` exempts tests only), and there is no fallback value to
construct: `AlternativeId::new` is `pub(super)`, so stratum 3 cannot mint the
id it would need. The design's own preference rules out the obvious escape —
§5.2's `number` bullet says "a total expression is cheaper than an argument
about why an `expect` is unreachable" — so the justification and the lint table
together leave exactly three ways out, and the design picks none of them:
a `#[expect(clippy::expect_used, reason = …)]`; the host-local kind carrying
the first id beside the list; or `drawn_kind` reporting an alternative-less
`choice` as `Undrawn` rather than drawing it.

**What was taken here.** `DrawnKind::Choice { first, alternatives }` — the id is
cloned once where the kind is built, which makes `as_drawn` and every display
site total with no lint exception anywhere.

**What it costs the design to be wrong.** Nothing on the wire; it is a place
where the plan will otherwise spend a review round discovering that a
one-sentence guarantee does not compile. Worth one line in §5.2 saying which
of the three it takes.

### P-3 — a `number`'s **spelling** is load-bearing and §5.2 never names it

*From P1a.*

**What the design assumes.** §5.2: `FieldValue.text` carries "the formatted
number, formatted host-side from the `f64`", and a `Slider`'s `AdjustedValue`
"resolves to an `Adjusted` whose text is the host's format of the number".

**What the build observed.** `Edited::Adjusted` carries a text as well as a
number, so **`as_drawn` cannot answer for a `number` without choosing a
format** — and §5.2's as-drawn bullet names only the number. That text is what
the widget is drawn showing and what the guard compares against, so the choice
is not cosmetic.

Measured, with `f64`'s own `Display` (the shortest decimal that reads back as
itself, which is the natural pick and the one taken here): it never uses
scientific notation. `R-17` admits any finite bound, so a field declared
`min: 1.7976931348623157e308` — `f64::MAX`, a legal bound — is drawn showing a
**309-character** string in its `LineEdit`, and `min: 2.2250738585072014e-308`
shows 326. `1e40` shows 41. The round-trip property survives all of them; the
display does not.

**What it costs the design to be wrong.** Not the wire and not the guard: a
person typing `1e308` back has their text recorded verbatim, the strings
agree, and nothing fights them. The cost is a legal `R-17` range drawing an
unreadable field, and the fix is a decision — `Display` below some magnitude
and `{:e}` above it — that belongs in §5.2 beside the parse rule it is the
inverse of, not in whichever function an implementer writes first.

### P-4 — the datetime **wire** behaviour needs no manifest change

*From P1a. Confirmation rather than a defect, recorded because it moves risk
off P4.*

**What the design assumes.** §10 argues the manifest must gain `tz-system` /
`tzdb-zoneinfo`, because `TimeZone::system()` otherwise always falls back
(`tz/timezone.rs:325-338`).

**What the build observed.** That argument is about `instant.rs` alone. Under
today's `jiff = { version = "0.2", default-features = false }`, reached from
`crates/goad` as it stands, all of `jiff::tz::Offset`, `Offset::UTC`,
`Offset::constant`, `jiff::Timestamp::UNIX_EPOCH` and
`Timestamp::display_with_offset` compile and run. So `Edited::Picked`,
`Reported::Picked` and `submitted`'s `R-57` datetime arm all land with no
manifest change, and the epoch's spelling is now measured rather than
predicted: `1970-01-01T00:00:00+00:00`, and `1969-12-31T19:00:00-05:00` for
the same instant at `-05:00`. Both are asserted in `draft.rs`.

**What it costs the design to be wrong.** Nothing — but it means P4's manifest
question gates `compose` and `today_local` and nothing else, and the
`canon-delta.md` CD-1 epoch question can be settled against a green test
before the feature argument is had.

### P-5 — where `prototype-delta.md` and `design.md` conflict (charter clause)

*From P1a. Recorded because the charter says to, not because the build found
it.*

Two of the thirteen replace a rule §5.2 states, and the lead's P1a brief
quoted §5.2's version. The delta wins per the charter, and **both rules were
implemented in their delta form**:

- **F-20, `slider_bounds`.** Third clause is `min + step > min && max - step <
  max` in `f32`, not "the step is finite and strictly positive". It subsumes
  the clause it replaces, so every case §5.2 names is still rejected;
  `[2^100, 2^100 + 2^77]` is additionally rejected and has its own test.
- **F-26, the parse rule.** "Parse as it stands; failing that, if the text
  holds exactly one character outside the numeric grammar, replace it with `.`
  and parse again", not "no `.` and exactly one `,`". The two agree on every
  case §5.2 and the brief name — `""` → 0, `"1,5"` → 1.5, `"1.5"` → 1.5,
  `"1,000.5"` → no finite parse — and diverge on texts like `"1 5"`, which the
  delta admits as 1.5. That divergence is unreachable in practice: a numeric
  `LineEdit` validates each insertion through `string_to_float`, which admits
  exactly one non-`.` character and only as *the* separator, so neither rule
  can be handed a text the other would read differently. The host is the more
  permissive of the two, which is the safe direction.

F-26 leaves "which characters count as the numeric grammar" to the plan. Pinned
here to ASCII digits and `+ - . e E`, deliberately **excluding** the letters of
`inf`, `infinity` and `nan`: those are spellings `f64::from_str` accepts, so
they parse at the first attempt, and admitting their letters would only widen
what counts as a separator. Reading taken for "exactly one character outside
the grammar": one **occurrence**, not one distinct character — so `"1,000,5"`
is refused.

The grammar is now pinned **and tested**: a test asserts `"1e5"` reads as
`100000`, which is the one mistake the rule invites — leave `e` out of the
grammar and `1e5` becomes a text with exactly one foreign character, so the
rule substitutes a `.` and silently reads `1.5`. See also P-6 and P-7, which
are what pinning it turned up.

### P-6 — the host now over-accepts relative to the control, and the design states only the other direction

*From P1a.*

**What the design assumes.** §5.2's argument for a host-side parse rule is
one-directional throughout: "A host that called `f64::from_str` on the raw text
would refuse `1,5` in a comma-decimal locale — text the control had just
approved — record nothing, and let the next guard write over the person." The
whole case for D-16, and for F-26 widening it, is that the host must not
**under-accept** relative to the control.

**What the build observed.** The rule F-26 leaves behind over-accepts, and it
does so by construction rather than by oversight. It cannot ask which character
is this locale's separator — that is exactly what F-26 took away — so it treats
*any* single character outside the numeric grammar as one. `"1 5"` and `"1x5"`
both read as `1.5`. `string_to_float` would refuse both outright: it
substitutes *the* separator and nothing else (`string.rs:398-412`).

Checked in both directions against the control's actual source, and the
asymmetry is total: **there is no text the control admits and parses finitely
that this rule reads differently or refuses.** Under a `.` separator it is
`parse::<f32>` and the host's first attempt matches it; under any other, the
control replaces every occurrence of one character and the host replaces the
one occurrence of one character, which can only differ where the control
already fails to parse. So the host under-accepts nowhere and over-accepts on
exactly the texts holding one foreign character that is not the locale's
separator.

**What it costs the design to be wrong.** Nothing through the widget:
`accept_text_input` gates every insertion, so those texts cannot be entered.
`Controller::edit` is public and is not the widget, so a caller that is not the
control can hand the host a number the control would never have produced —
and `SPEC-001/R-35` puts the judgement of whether an answer is acceptable in
the backend either way. The cost is a missing sentence rather than a defect:
§5.2 argues one direction at length and does not say that the other is
accepted deliberately. Say it, and the rule is complete.

### P-7 — §5.2's "three texts the control admits that no parse accepts" is wrong twice

*From P1a, reading `i-slint-core-1.17.1` directly.*

**What the design assumes.** §5.2: "`-`, `.` and `-.` … are exactly the three
texts the control admits that no parse accepts, allowed as len≤2 starts so a
person can begin typing a negative or a fractional number at all."

**What the build observed.** `accept_text_input`'s decimal arm is two rules,
not one (`items/text.rs:2205-2230`). A candidate of **two bytes or fewer** is
admitted when it is `-`, the separator, or `-` followed by the separator — the
separator, **not** a dot, so the enumerated set is `-`, `.` and `-.` only in a
dot locale, and is `-`, `,` and `-,` in a comma one. Under a comma separator
`"."` is *rejected* outright. The design's own F-26 removed the named separator
from the parse rule one paragraph earlier and left it in this sentence.

Any longer candidate is admitted when `string_to_float` parses it — and that
is `parse::<f32>`, which accepts `inf`, `infinity` and `nan`
case-insensitively, and reads `1e400` as an infinity. So there is a **second
set** the sentence does not name: texts the control admits that a parse
*accepts non-finitely*. They are reachable. Typing `inf` fails at the first
`i` (a one-character candidate must be `-` or the separator), but **pasting it
succeeds**, because the three-character candidate goes straight to
`string_to_float`. A person can put `inf` or `nan` into a numeric field.

The behaviour is right and needs no code: those texts parse, `Finite::new`
refuses them, the last representable number stands and the text is displayed
verbatim — the same path `1e400` already takes, and it is now asserted. What
is wrong is only the prose, and the two sets sit one sentence apart in it: one
is *admitted, no parse accepts*, the other is *admitted, a parse accepts
non-finitely*.

(A third, smaller thing, noted rather than raised: the `len <= 2` test is over
**bytes**. U+066B, the Arabic decimal separator, is two bytes, so `"٫"` alone
is admitted and `"-٫"` — three bytes — is not. A person in that locale cannot
begin a negative fraction with the separator. That is Slint's, not this
host's, and the host parses `-1٫5` correctly either way.)

**What it costs the design to be wrong.** No code, and the cost is that the
sentence is load-bearing in two places: it is the justification for "the last
representable number stands", and §5.5's Edges table repeats it. An implementer
who takes the enumeration as closed writes a three-case test and misses the
paste path entirely.

### P-8 — F-48's "`Eq` stops deriving" is load-bearing at `Finite`, not only at `Edited`

*From P1a. Small, and it actually happened.*

**What the design assumes.** F-48: `Edited::Adjusted` gains a `Finite(f64)` and
`Reported::AdjustedValue` an `f32`, "so `Eq` stops deriving on all three" —
`Reported`, `Edited` and `Command`. The instruction is not to write the impl by
hand, because it would be unsound over `AdjustedValue(NaN)`.

**What the build observed.** `Eq` does not stop deriving unless nobody writes
it on the **leaf**. P1a first shipped `impl Eq for Finite {}` — which is
*sound*, since `Finite` excludes `NaN`, the only `f64` that makes `PartialEq`
less than an equivalence relation — and with it in place `Edited` and `Command`
both kept their `Eq` derives and the whole workspace compiled clean under
`-D warnings`. F-48's trap was reached by a route F-48 does not name: not a
hand-written impl on the enum it warns about, but a defensible one on the
newtype under it.

**What was taken here.** Dropped. `Finite` carries `PartialEq` and
`PartialOrd` and no `Eq`, on the ground that the only thing the impl could buy
is an `Eq` on `Edited` that F-48 removes — an impl asserting a subtle property
that nothing consumes is a claim nobody checks. Nothing needed it: dropping
`Eq` from `Finite`, `Edited` and `Command` together broke no call site in the
workspace.

**What it costs the design to be wrong.** One line: F-48 should say the
newtype carries no `Eq` either, and why — not because it would be unsound, but
because a sound `Eq` there silently restores the derive above it.

### P-9 — `PendingEdit` carries its own `view`, and the design settles it

*From P1b. Recorded because the brief left the choice open and said to say
which was taken.*

**What the design assumes.** §5.2's struct gives `PendingEdit` a `view` field.
The brief raised the alternative: the drain filters on the presented view
before building the command, so the carried struct might not need one, and
"two readings exist".

**What the build observed.** The two readings are not equivalent, and the
design decides between them in a place the brief did not cite. §5.5's I-H says
an entry is "**drained** into a `Choose` carrying that view alongside — the
last two refused as `SupersededView` when the view has been replaced", and the
edge table carries a row for exactly that: *a stale pending entry is drained
into a `Choose`* → refused, reported, **the answer still goes**. A drain that
filtered on the presented view would leave that row unreachable: the stale
entry would stay in the map, and the person would learn of it one debounce tick
later through the timer's own `Command::Edit` instead.

**What was taken here.** The `view` is carried, the drain takes everything, and
`controller::choose` refuses a stale carried edit while still answering. Both
halves are probed
(`tests/prototype/staleness.rs::a_stale_carried_edit_is_refused_and_the_answer_still_goes`),
and the probe is negative-controlled against both ways of getting it wrong —
returning `Err` on a stale edit, and applying the carried edit under the
command's view rather than its own.

**What it costs the design to be wrong.** Nothing on the wire, and one struct
field. What it would have cost to take the other reading is one row of the edge
table quietly becoming dead prose, which is the class of defect §5.2's *stated
in full* discipline exists to prevent.

### P-10 — the design's `resolve` collides with an ADR-001 instrument, and the instrument is red

*From P1b. The collision arrived at P1a; P1b widened it from one site to five.*

**What the design assumes.** §5.2 names the new pure function `resolve`, in
`view_model.rs`, called from `controller::edit` and from `glass.rs`.

**What the build observed.** `crates/goad-boundary`'s
`structure::no_production_line_in_the_renderer_names_the_identifier_resolve`
asserts that **no production line under `crates/goad/src` names the identifier
`resolve` at all**. Its subject is `goad_semantics::schedule::resolve` — AC-6
(a), design.md §5.5 I-1a, *the timer never resolves a schedule itself* — and it
is deliberately an identifier-word match rather than a path grep, "because a
brace-grouped `use goad_semantics::schedule::{resolve, wait_for};` would defeat
a path grep (F-3) but not this". A different function of the same name defeats
it in the other direction.

The test is red at `HEAD` of this branch, on `view_model.rs:345` alone; this
phase takes it to five lines across three files. Nothing was changed outside
`crates/goad/` to repair it, per the brief.

**What it costs the design to be wrong.** The slice cannot close with that
instrument red, so the plan owes one of two amendments and neither is free:
narrow the instrument — it would have to match the *import* rather than the
word, which is the F-3 evasion it was written to close — or rename the design's
function. A rename is the cheaper of the two and touches §5.2, §5.3, §5.5 I-G
and §9. **This is a canon-adjacent decision and belongs to the user**, which is
why it is a finding rather than a repair.

### P-11 — the timer's re-arm reproduces, measured

*From P1b. Confirmation rather than a defect, and the brief asked for it
explicitly.*

**What the design assumes.** §5.1: a `slint::Timer` may be re-armed from inside
its own callback, cited to `i-slint-core-1.17.1/timers.rs:348-372` and
`:330-334`, "read from the locked source rather than assumed, because the whole
re-arm rests on it".

**What the build observed.** The source reads as stated at the cited lines:
`start_or_restart_timer` copies the old `being_activated` flag onto the
replacement (`:365-370`), and `maybe_activate_timers` puts the previous
callback back only where the permanent store is still `CallbackVariant::Empty`
— *"if not, it means the invoked callback has restarted its own timer with a
new callback"* (`:328-334`).

And it behaves as stated under a real loop.
`tests/event_loop_debounce/debounce.rs` holds two entries from one window,
drives the **real** `edited` callback, and both `Command::Edit`s arrive down a
capacity-one channel — the second only reachable through the re-arm. Removing
the re-arm delivers one and the case fails; sending `text` where it is raised
rather than debouncing it fails the elapsed-time assertion. Measured ~0.42 s for
the whole case, two ticks of 150 ms included.

**What it costs the design to be wrong.** Nothing now. It moves a *cited*
claim to a *measured* one, which is what §9's tiering asks of the claims the
design turns on.

### P-12 — `init` runs under `init_no_event_loop`, so the destruction probe is a cheap-tier case

*From P1b. A tiering fact, not a defect.*

**What the design assumes.** §9 and A-3: `changed` handlers and `slint::Timer`
do not run under `init_no_event_loop`, measured — which is why the guard and
the debounce need the loop tier.

**What the build observed.** An element's `init` handler **does** run there.
`tests/prototype/split.rs` reads a non-zero `inits` off a window presented under
`init_no_event_loop`, and the count moves when a view is replaced and does not
move when the same view is presented again.

That is what makes A-1's property — *replacing `values` wholesale destroys no
element* — a cheap-tier assertion rather than a loop-tier one, and it is worth
saying because §9's placement table decides tiers per row. The two are
independent: `init` is a construction hook and fires when the repeater
instantiates, while `changed` is driven by the property evaluator the loop
runs.

**What it costs the design to be wrong.** Nothing, but it moves the largest
probe in the set out of the tier that costs a `[[test]]` target and a
once-per-process initialiser. §9's rows for A-1 and for AC-4's *no element is
destroyed* can be written at the cheap tier.

### P-13 — the undrawn fixture has to migrate once per phase, and after P4 it cannot

*From P1b.*

**What the design assumes.** §5.1 enumerates the sites that assume
`Undrawn::FieldForm` is constructible and says the cost "is an enumeration, not
a rename"; `canon-delta.md` CD-2 records `R-58`'s MUST NOT half becoming
unobservable after this slice.

**What the build observed.** The enumeration is right about *which* sites, and
understates *how often*. Five fixtures used a `text` field **because text was
the undrawn kind**: `A_DRAWN_AND_AN_UNDRAWN_FIELD` (`fields.rs`), `wiring.rs`'s
`TWO_FORMS`, two `mapper.rs` block cases, one `reception.rs` diagnostic case.
Each was repaired by one word — `"kind": "number"` — and each will need the same
one-word repair at P2, again at P3, and again at P4, at which point there is no
undrawn kind left and the cases are not repairable at all.

So the enumeration is not a list of ~20 edits made once. It is a list of sites
that must be revisited at **every** phase that draws a kind, and the last visit
deletes rather than repairs. The mechanical cost is trivial; the risk is that a
phase repairs by migration without noticing it has deferred a deletion, and the
suite ends P4 with a handful of cases asserting nothing.

**What it costs the design to be wrong.** Not correctness — the migration is
visible and green at every step. It is a **plan** cost the design does not
price: the phase that draws the last kind inherits the whole of CD-2's
deletion, and the three phases before it each look cheaper than they are. One
line in §9 assigning the deletion to a named phase would fix it.

### P-14 — a refusal that does not stop an exchange has nowhere durable to be reported

*From P1b.*

**What the design assumes.** §5.2: a carried edit whose `view` is not the
retained one is "`Refused::SupersededView`, reported — the typing really was
discarded … The **answer still goes**."

**What the build observed.** *Reported* and *the answer still goes* are in
tension in this controller, and the design does not notice. `serve` has exactly
one refusal site, and it `continue`s to the top of the loop: a refusal and an
exchange are alternatives in `dispatch`'s `Option<Result<Pending, Refused>>`,
so there is no value that says *both*. What P1b does is have
`Controller::choose` call `self.refuse(&Refused::SupersededView)` itself and
then return `Ok`, which is the shape `refuse_during_exchange` already uses for
the ingress arm. The consequence is the same one that arm's own comment names:
`absorb` replaces the whole retained `Diagnostics` when the exchange folds, so
the line is on screen for the duration of the exchange and then gone.

Two smaller things came with it. The line a person reads is
`Refused::SupersededView`'s existing one — *"no action taken: that answer
belongs to a question that has since been replaced"* — which is about an
**answer**, and here it is about discarded typing on an answer that did go.
And a `Choose` carrying two stale edits reports once, not twice, because
`Diagnostics::refused` replaces rather than accumulates.

**What it costs the design to be wrong.** A person who answers a replaced view
and then answers the new one is told their typing was discarded only for as
long as the backend takes to reply. The design promises they are told; it does
not promise for how long, and nothing in the host today can promise longer
without a second diagnostics channel. Worth one sentence in §5.2 either
accepting that — *reported for the life of the exchange* — or moving the row
to a class of refusal the fold does not overwrite.

### P-15 — **withdrawn.** Written from a stale read of `draft.rs`

*From P1b; withdrawn by the lead on checking the code against it.*

As written, P-15 claimed `Edited` keeps `Eq` through a hand-written
`impl Eq for Finite`, and that `Command` cannot because it carries a bare
`f32`. The second half is true. The first is not: `dc30a2a` deleted that impl
and dropped `Eq` from `Finite`, `Edited`, `Reported` and `Command` alike, and
`draft.rs`'s own derive comment now says so in as many words.

**The code is right and the finding was wrong**, which is the less common way
round and worth the id it is spending. `dc30a2a` landed in the worktree while
P1b was mid-phase — two writers, briefly, because the lead launched P1b before
P1a had released the worktree. P1b was told to re-read and did so for the files
it was editing; `draft.rs` was not one of them, so the stale view survived into
a finding about `draft.rs`.

Two things are worth keeping out of it.

**The distinction P-15 drew is real even though its premise was not.** `Finite`
*can* soundly carry `Eq` and `Reported` cannot, and that asymmetry is exactly
F-48's reasoning. P1a's own P-8 records the decision that settles it: the only
thing `Finite: Eq` could buy is `Edited: Eq`, which F-48 removes, so an impl
asserting a subtle property nothing consumes is a claim nobody checks.

**A finding that describes the code rather than the design is a finding that
can go stale.** P-1 … P-14 are claims about what the *design* does not say, and
those survive a commit landing underneath them. This one was a claim about what
the *code* holds, and it did not. Worth remembering when the slice's own
`review-code.md` runs against a moving tree.


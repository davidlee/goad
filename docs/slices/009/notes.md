# Notes — Slice 009

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Handover — four phases are done; you are orchestrating the other five

Written 2026-09-19 for a fresh agent taking over execution. The design and plan
stages are closed and their handover text is gone; what is below is what running
PHASE-05 … PHASE-09 actually needs. Delete at the close.

**Your job is to orchestrate, not to implement.** One phase, one agent, one
session (`docs/AGENTS.md` §Execute). You hold the thread: you mark a phase in
progress, brief its agent, verify what comes back against the tree rather than
against the report, take the decisions that are yours, escalate the ones that
are the user's, and commit the bookkeeping. Four phases have run this way and
the pattern holds — it is described under *How to brief a phase agent* below.

### Where the slice is

Design closed (`design-log.md` D-37), plan accepted (`plan-log.md`), **PHASE-01
through PHASE-04 done**, PHASE-05 … PHASE-09 pending. §Status is the record.
`just check` has exited 0 at every phase boundary, each figure re-run by the
orchestrator rather than taken from a report: **551, 563, 564, 573** — and those
are **gate** totals. **Always quote a count with its denominator.** The gate runs
`cargo test -p goad-semantics` as a command of its own, so `goad-semantics`'s
30 + 5 are counted twice and the gate total is always exactly 35 above
`cargo test --workspace`, which is **538** now. PHASE-03's sheet named one
denominator over a list enumerating the other; both its numbers were right and
the sentence was not. Annotated in place, and §Open carries the arithmetic.

Three decisions landed *during* execution and bind what is left: **D-36** (the
refusal is not durable), **D-37** (the design is accepted), **D-38**
(`FieldEdit` carries `slider`). `plan-log.md` carries five more, all of them
amendments to `plan.md` rather than to the design.

### How to brief a phase agent

What has worked, and each element is there because something went wrong without
it:

- **Two steps, explicitly not merged.** The sheet first — reading list with
  re-derived `path:line`, assumptions, STOP conditions, tasks — then execute.
  Say so, because an agent told only to "do PHASE-0n" writes the sheet last.
- **Tell it to verify its own entry criteria** rather than inheriting your word
  for them. All four did; none found a lie, but the reading is how they learned
  the previous phase.
- **Lift the phase's traps out of `plan.md` into the brief.** The Notes for the
  implementer are load-bearing and long; an agent that skims them writes
  `resolve`, or `impl Eq for Finite {}`, or a `_ => None` arm.
- **Name the STOP conditions in your own words**, and say that a criterion
  compelling a file the Surfaces line does not name is one of them. That is what
  produced every one of the five amendments.
- **Ask for one thing you cannot verify yourself.** "Did the negative control
  actually go red, and what did the counter read?" got a number; "is it
  covered?" would have got a yes.
- **Forbid**: `git stash`, `git checkout`, `reset`, `rebase`, `push`, editing
  `design.md` / `design-log.md` / `plan.md` / `plan-log.md` / `canon-delta.md` /
  any `review-*.md`, amending canon, and deleting or weakening a test to go
  green. Allow `add` and `commit` on `main`, with the session trailers.
- **Give it the budget rule**: at ~200k tokens, stop, write a `PARTIAL` note
  naming what is and is not done, leave the tree green, hand back.

### Five things that have actually bitten

1. **The Surfaces lines are short, and it is one cause.** Five times now.
   `plan.md`'s Surfaces were derived from `design.md` §9's enumeration of
   **constructors**, so a file that changes because a type above it changed
   (`wire.rs`), because it *binds* a markup callback (`tree.rs`), because a doc
   in it went stale (`diagnostics.rs`), or because a type must become *visible*
   (`app.slint`) was never in the enumeration's reach. §9 is not wrong. Expect a
   sixth; the user has endorsed amending, and `plan-log.md` is where it goes,
   dated and scoped. **The remaining phases most exposed are PHASE-07 and
   PHASE-09**, which add markup and retire a type respectively.
2. **A plan citation can be stale.** PHASE-01 moved the line PHASE-02's notes
   cited (`glass.rs:203` → `:299`). Re-derive with `grep -n`; the slice's rule
   is *cite from an instrument that prints the number*.
3. **"Done" is not idle.** A phase agent reports the phase green and then makes
   a further *record* commit minutes later. Wait for the idle notification
   before committing anything or spawning the next agent. If you must write in
   that window, `git add <explicit paths>` — never `-A`.
4. **A true sentence goes stale by widening.** Twice in PHASE-03: §9's *twelve*
   was accurate about constructors and silent about binders;
   `Refused::UnknownField`'s doc was accurate about the only path that then
   existed. Neither is a bad citation. The working rule is in §Harvest: when a
   phase widens what a type or rule means, re-read every sentence that
   enumerated it, including the ones still true.
5. **Four id sequences collide**, not three: `design.md` §7's `Dn`,
   `design-log.md`'s `D-n`, `prototype-notes.md`'s `P-n`, and slice 008's `VT-n`
   doc comments already in `wiring.rs`. Cite the file with the id, and
   phase-qualify a new `VT-n` in a test doc (`PHASE-03/VT-2` is the precedent).

### What the remaining phases must honour

- **PHASE-05** is the largest. `pending.rs`, the timer's one-per-tick re-arm,
  `Wire::send -> bool`, `Command::Choose` carrying the flush. Its notes name the
  channel-capacity argument for why the flush travels *inside* `Choose`; that is
  measured, not a preference.
- **PHASE-07/EX-3** carries an instruction added after the plan was accepted:
  **re-measure** whether PHASE-04's one-line `export { Date, Time }` is still
  needed once `FieldValue` gains those fields, and delete it on the measurement
  rather than on the expectation. Only the export path was measured; the
  reachability one was not.
- **PHASE-08/EX-9** is D-38 and was added after acceptance: `FieldEdit` carries
  `slider: bool`, the mapper selects on it and nothing else, and **each control
  writes its own literal** — never `field.slider`. Deriving the report from the
  row costs the host its only witness that the control drawn is the control that
  reported. `design.md` §5.2 carries the argument.
- **PHASE-09** retires `Undrawn::FieldForm` and deletes the fixtures that have
  been migrating one kind at a time since PHASE-05 (`prototype-notes.md` P-13).
  There is no kind left to move them to.

### One thing the user has not been asked

`install.rs::reported`'s four undrawn arms answer `None`, so **a phase that
draws a control and forgets its mapper arm drops every edit from that control.**
It fails that phase's own first case, so it is caught — but by *absence of an
effect* rather than by a compile error, which is weaker than everything else in
this tower, where a sixth kind is a compile error in four places. PHASE-03 could
not make it stronger without a panic (denied crate-wide) or declaring markup
fields before a control reads them (against the standing rule), so the
mitigation is the schedule in the function's doc naming each arm's owning phase.
**Making it stronger is a decision, not a repair.** Raise it if a phase trips on
it; otherwise it belongs in the audit.

### Useful facts nobody should rediscover

- **`wiring.rs` and `fields.rs` measure disjoint halves of the edit path**,
  proven by injection: breaking `install.rs`'s closure reddens four `fields.rs`
  cases and nothing in `wiring.rs`; breaking `Controller::edit` reddens
  `wiring.rs` and nothing in `fields.rs`. **A new arm in `reported` is measured
  by a `fields.rs` case and by nothing else.** PHASE-05, -07, -08 and -09 each
  need this.
- **An import is not an export.** Slint generates a widget-library struct into
  `crate::generated` only if `app.slint` *exports* it; importing it does
  nothing. Measured four ways in PHASE-04.
- **`cargo test -p goad-semantics` never links the `jiff` the workspace build
  links.** Measured with `cargo tree -e features` either side of PHASE-04's
  manifest change: `--workspace` resolves `alloc, std, tz-system,
  tzdb-zoneinfo`; `-p goad-semantics`, `-p goad-shell` and `-p goad-emit` each
  resolve none.

### What holds the truth

| file | state |
|---|---|
| `plan.md` | **the executable truth.** Nine phases, criterion ids immutable. Amended six times since acceptance — five Surfaces lines and PHASE-08/EX-9 — every one dated and logged. Read a phase's Notes for the implementer in full; they are long and load-bearing |
| `design.md` | **current truth**, including D-38's `slider` discriminant in §5.2. Do not edit it; a design change goes back to the user |
| `design-log.md` | D-1 … D-38. Append-only. `D-n` here is **not** `Dn` in `design.md` §7 |
| `plan-log.md` | five dated entries, all amendments to `plan.md`. This is where a sixth Surfaces amendment goes |
| `canon-delta.md` | CD-1, CD-2 — the slice's working authority on two `SPEC-001` changes. **Promoted at audit with explicit user endorsement, by nobody else** |
| `notes.md` §Phase sheets | one per completed phase, left in place. PHASE-01's and PHASE-02's are the shape to match |
| `notes.md` §Harvest | lifted into `docs/memory/` at the close. Keep it current *in* each phase, not after |
| `review-design.md` | closed, 2563 lines, `**State:** resolved`. Read one finding if you need it; do not read it through |

### What is owed, in order

1. **PHASE-05 … PHASE-09**, one agent each. PHASE-05 is the largest and
   PHASE-09 the widest.
2. **Audit** — a fresh agent, `audit.md`, its Brief written *before* looking.
   `review-code.md` is full strength at both tiers and is where the defects
   actually are. Budget two sessions: the review rounds on the *repairs* are
   half the cost.

   **One reconciliation row is already known and endorsed.** PHASE-05 renamed
   `undrawn_form` to `drawn_form` and widened its return to
   `Result<DrawnKind, FieldForm>` (`plan-log.md`, *a decision taken out of
   turn*). `plan.md` follows the code; three records still name the old
   identifier — `design.md:151`, `:1398` (D11) and `:1513` (AC-7's row),
   `canon-delta.md:112`, `research.md:95`. The user endorsed patching
   `design.md` at reconcile rather than carrying it as drift (2026-09-19), so it
   is a **Reconciliation** row and not a *Design drift not reconciled* one.
   **`canon-delta.md:112` is the one that matters**: CD-2 is promoted into
   `SPEC-001`, so a stale identifier there would land in canon. Check it before
   promoting, not after.

   **A second row, and this one is a hole rather than a rename.** PHASE-06/EX-3
   ends *"where `interpret` refuses the entry the draft's value stands"*. That
   clause is **discharged by construction and measured by nothing**: `text` is
   the only drawn debounced kind, and `Typed` against `DrawnKind::Text` always
   succeeds, so no case in the tree can reach the refusing branch. PHASE-08 draws
   the two `number` controls and is the first phase where a pending report can be
   refused — a case belongs there. Until it exists the clause is an untested
   path, and the audit should record it as one rather than as a discharged
   criterion. Raised by PHASE-06's agent, verified at `glass.rs:378-382`.

   **A third row, and this one is a measured outcome rather than an oversight.**
   PHASE-08/EX-7 ran the numeric guard's exception four ways and **removed** it —
   not as dead weight but because carrying it suppresses exactly the convergence
   AC-6 requires (`plan-log.md`, *the exception was not dead, it was wrong*).
   `design.md` §5.2's comparand table and §7 **D13** both still state the
   exception as part of the guard. **This is reconciliation, not drift**: §9's
   **A-2** framed the exception as an obligation whose outcome was *open*, so the
   design asked for the measurement and the measurement answered it. The user has
   endorsed patching `design.md` at reconcile (2026-09-19). Both sites, and A-2's
   own row, need to say what was measured.
3. **Promote the drafts.** `canon-delta.md` CD-1 and CD-2 apply to `SPEC-001`,
   with explicit user endorsement, recorded in `audit.md`'s Reconciliation
   table. A slice does not close holding an unpromoted draft.
4. **At the close**, three documentation repairs this slice has earned and
   deferred, all in `docs/memory/`:
   - `a-present-destroys-the-widget-it-writes.md` is **half stale** — it says
     `present` ends in `self.options.set_vec(rows)` and that every row is
     rebuilt on every present. PHASE-01 made both false. The rest of the note
     is exactly what PHASE-01 implemented and is still right.
   - `a-popup-is-rebuilt-on-every-show.md` cites
     `widgets/fluent/components.slint:15-19`; the correct range is `:49-53`.
   - `glass.rs:1-3` claims to be the only file in the crate naming a generated
     type. `install.rs:15` names four and `instant.rs` two more. **PHASE-06 and
     PHASE-07 both have `glass.rs` in their Surfaces** — whichever runs first
     should fix it rather than leaving it to the close.
5. **Lift §Harvest into `slice-009.md` §Follow-ups and `docs/memory/`**, write
   §Summary, set the stage to `done`.

### Durability

**`main` is 19+ commits ahead of `origin` and nothing since `44fbd8e` is
pushed.** 33 commits and a whole branch once lived on one disk in this project.
Check `git log origin/main..main` before you finish, and ask the user — pushing
is theirs to authorise, not yours.

## Reference — carried from the design and review stages

Everything below was written while the design was being built and reviewed. It
is kept because four of its sections are still live — §*Citations known bad*,
§*Traps worth naming*, §*Facts verified by hand* and §*Integration notes* — and
the rest is the argument behind decisions the remaining phases inherit. None of
it describes work that is still owed.

### P-14 — settled, and the recipe that was here could not have worked

Formerly §*Waiting on the user*, which is how `review-design.md`'s Synthesis
cites it.

**Settled 2026-09-18 as `design-log.md` D-36**: the refusal is reported for the
life of the exchange and no longer, stated in `design.md` §5.2. No new
mechanism. This section is kept rather than deleted because the recipe it used
to carry was wrong in a way worth recording.

**The recipe could not produce the race.** It said *"the bash backend re-prompts
on every evaluate, so the view is replaced every 3 s"*, and raising `DEBOUNCE`
would make the window hittable. `MINIMUM_SPACING` (`controller.rs:509`) is a
**floor** on how often an evaluate may begin, not a cadence. What actually
schedules one in the demo: `examples/demo.toml:11` `default_poll = "30m"`, a
backend answering `next_check: 45 minutes` to everything, and a `respond` arm
returning `{"view":null}` (`examples/shell/backend.sh:88`) — so answering closes
a view rather than issuing a new one. After the first prompt nothing supersedes
anything for 45 minutes. The user ran it with the knob in place and saw nothing,
correctly.

**The drivers that do exist**, if a later slice needs this race on screen:
ingress (`just emit <source> <kind>`, floored to one per 3 s by
`controller.rs:678`), or a backend that returns a view on a short `next_check`.
`--source host` is refused — `reserved_source`, `SPEC-003/R-13` — so an emitted
event takes `backend.sh:126`'s branch and draws a **fieldless** view, which
replaces the form visibly and drowns out the thing being observed. Re-issuing
the same form needs a one-line spike edit to `backend.sh:92` (`host)` →
`host|nudge)`).

**What the pricing found, which observation would not have.** The refusal is the
smaller half of the event. `Command::Edit` never reaches the backend — it
mutates the retained draft (`controller.rs:661-675`) and yields no exchange — so
the draft dies with the view, the field clears under the caret, and everything
typed into it is lost. `SupersededView` names only the burst since the last
delivery, and the debounce restarts on every keystroke (`pending.rs:82`), so
that burst is *since your last pause*, not 150 ms flat. A notice that survived
the exchange fold would have been a durable report of the lesser loss. §8 R5 is
restated accordingly, and the follow-up on `SPEC-002/OQ-4` is in §Harvest.

**The spike knob is reverted.** `DEBOUNCE` is back to
`Duration::from_millis(150)` at `pending.rs:37` in `/home/david/dev/goad-009-proto`.

**The method still stands** even though this one was settled by argument. The
user's rule — *"I'm also inclined to make these usability decisions based on
interaction with actual software instead of based on a leaky theoretical
model"* — is in §Harvest as a memory candidate, and what changed here is that
the model stopped being leaky: reading what an `Edit` does made the observation
unnecessary. Check whether the mechanism does what the note claims **before**
sending anyone to look at it.

### Carried forward, outside this slice

`docs/memory/a-popup-is-rebuilt-on-every-show.md` cites
`widgets/fluent/components.slint:15-19` for `ListItem`'s accessible properties.
They are at `:49-53` — the same bad citation F-33 found in the design, and the
memory doc has it too. Not fixed mid-slice; lift it at close.

### Integration notes the ledger did not carry — now applied

Worked out while dispositioning, and applied during the integration. Kept because
each one records *why* the text reads as it does. **One of them was wrong**: the
first bullet's claim that pushing `as_drawn` inside `resolve` "puts `as_drawn`'s
two call sites back in `view_model.rs`" is false — `answer` is in
`controller.rs`. Its own next clause has it right, and §5.2 says `answer` and
`resolve`.

- **`resolve`'s signature should take `held: Option<&Edited>`, not `&Edited`.**
  §5.2 as integrated has the caller do `state_of(..).unwrap_or_else(|| as_drawn(kind))`.
  Push that inside: `resolve` already has the kind, so it can consult `as_drawn`
  itself, and then both callers pass `state_of(..)` straight through. This also
  puts `as_drawn`'s two call sites back in `view_model.rs` where it lives, and
  §5.2's sentence about which sites apply it changes for the second time — it is
  now `answer` and `resolve`.
- **The overlay should go through `resolve` rather than through a second
  mapping.** `pending.rs` holds `Reported`, and `glass.rs` displays from `Edited`.
  Resolving the pending entry and using the result in place of the draft's value
  keeps **one** display mapping; writing a `Reported` → `FieldValue` mapping
  beside the existing `Edited` → `FieldValue` one is the duplication to avoid.
- **F-38's rule applies at three sites, not the two its Response names.** An
  entry is *shown*, *sent* and *drained* only where its view is the retained one.
  The display site matters: on a new view the rows are rebuilt and slots
  renumbered, so a stale entry keyed `(option, field)` whose ids happen to match a
  new field would otherwise be overlaid onto the new view's widget. State it once,
  as one rule over three sites.
- ~~**The overlay creates a property worth stating as an invariant:** what the
  screen shows is what an answer would submit.~~ **Struck by F-53**, which is
  the third finding of that class (F-21, F-42): the generalisation is false —
  a numeric text no finite parse accepts, a cleared numeric field and an
  untouched `datetime` each show one thing and submit another, and the list
  cannot be closed. What survives, and is what I-H now says, is the part that
  was always checkable: a drained entry reaches the draft; a kept entry is still
  displayed and still travels in the next `Choose`; a stale entry does neither.
- **Construction order is already right.** `main.rs:85-101` installs the callback
  table before building `SlintGlass`, so the `Rc<Pending>` is created at step 6
  and cloned into `install` and `SlintGlass::new` both. One field on
  `SlintGlass`, no reordering.
- **`Wire::send`'s result is already in hand.** `wire.rs:127-133` binds
  `TrySendError::Full(_returned)` and drops it deliberately (D8). F-39 is a return
  type, not a mechanism.
- **CD-2's `R-16` mention (F-45)** is in its `**Document:**` line only; the three
  changes below it cover `R-57`, `R-58`, `R-55`. `SPEC-001`'s `R-13, R-14, R-16`
  row is about wire forms and is untouched by drawing.

### Facts verified by hand, because they overturn things

1. **The serve loop presents before every command** (`controller.rs:738-739`:
   `glass.present(...)` is the first statement of `'serving: loop`). That is what
   makes F-40 real: any handled command inside a debounce window repaints from a
   draft that does not yet hold the person's typing.
2. **`Wire::send` returns `()`** and swallows `Full` (`wire.rs:127-133`). F-39.
3. **`increment()` is `set-value(value + step)`**
   (`common/slider-base.slint:126-131`), so F-20's ulp case freezes the slider:
   at `minimum = 2^100` the `f32` ulp is `2^77` and a one-ulp span gives a step
   below half an ulp.
4. **The ICU decimal separator lookup is compiled in, and its value is never
   set.** The first half was established before round 4 and stands:
   `i-slint-core`'s default `std` feature enables
   `i-slint-common/locale-decimal-separator` (`i-slint-core/Cargo.toml:82-95`),
   and `string_to_float` replaces *that* character, rejecting `.` outright when
   the separator is not `.` (`i-slint-core/string.rs:398-412`); it is not
   reachable from host code, because `SlintContext::locale_decimal_separator` is
   `i-slint-core`, which `crates/goad` does not depend on, and `slint` re-exports
   neither it nor `string_to_float`. F-26.

   **What this entry used to say and should not have**: *"live in this build"*.
   A feature being compiled in is not the same as the value being populated.
   `locale_decimal_separator` is a plain `Property<char>` initialised to `'.'`
   with no binding (`context.rs:122-125`), and the only writers are `set_locale`
   — *"testing only"*, called from `i-slint-backend-testing` and nowhere else —
   and two arms of `select_bundled_translation`, which needs bundled translations
   compiled in and an explicit call. `crates/goad/build.rs` bundles none and
   nothing in the crate calls either. So the separator is `'.'` for the life of
   every process this workspace builds. **F-50.**

5. **`input-type: decimal` admits exactly three texts no parse accepts** — `-`,
   the locale separator alone, and `-` followed by it
   (`i-slint-core/items/text.rs:2202-2229`). `--` is not among them.
6. **No `PopupWindow` state survives a close**, and a popup's properties cannot be
   assigned from an enclosing handler. Both measured (F-31 withdrawn, F-35).
7. **Popups are reachable under `init_no_event_loop`** — `find_all` walks
   `active_popups` (`search_api.rs:291-312`) — but no case here has yet needed one
   **laid out**, which `mock_single_click` depends on (§8 R9).
8. **The command channel is capacity 1** (`main.rs:86`) and `serve` shares the UI
   thread, so one command per timer tick is the most that is available.

### Citations known bad

The ledger is append-only, so a bad citation inside a Response stays as written.
**Six** are known. Four were known before round 4's outcomes were set, and
that list held five until the prototype checked it
(`prototype-handback.md` §5): F-10's re-disposition (one `wiring.rs` site, not
two); F-23's Response (`wire.rs:130`, not `:126`); F-33's Response
(`fluent/components.slint:15-19` for `ListItem`; they are at `:49-53`); and
F-54 (`clock.rs:46-52` for the doc comment; it is at `:47-53`, and `:46` is
blank) — found by the integration, which is where three of the four came from.

**F-54's bad range has a third copy**, in `design-log.md` **D-35**, which is
append-only too — so `clock.rs:47-53` in `design.md` §10 is the whole of the
correction available. Found by the raiser setting round 4's outcomes.

**A fifth, and it is a new class: a raiser's, in an Outcome line.** Setting
round 4's outcomes, the raiser wrote `items/text.rs:2230` for the
`string_to_float(&candidate)` call inside `accept_text_input` (it is at
**`:2228`**; `:2230` is a different match arm) and `string.rs:404-406` for the
`contains('.')` refusal (it is at **`:406-408`**). Both are struck in place in
the ledger and the wrong claim they supported — that §5.2's `:2202-2229` and §8
R11's `:2208-2229` stop short of the call — is struck with them: both ranges
contain it, and both `design.md` citations are right. **The cause is worth more
than the entry.** Both were counted by hand off a `sed -n 'a,bp'` window; every
line number this slice has taken from `grep -n` or `awk NR` has held. Other
hand counts in the same pass happened to survive, which is luck rather than
method. So the rule is not *verify the responder's first*, nor even *the
reviewer's too* — it is **cite from an instrument that prints the number**.

**The fourth breaks the pattern the first three set.** F-54's range was written
by the **reviewer**, in the finding's Evidence line, and the Response then
repeated it — so the rule *verify the responder's first* is a priority, not a
sufficient check. The other three are responders' alone, and rounds 2 and 3's
reviewers' citations checked out. Round 4's *"no bad citation was found this
round"* (item 2) is a claim about `design.md`'s citations and stands; it was
never a claim about the round's own.

**A sixth, found by the plan stage, and it was in `design.md` itself —
corrected.** §8 R5 cited `controller.rs:753-761` for *"`Command::Edit` never
reaches the backend — it mutates the retained draft"*. That range is `serve`'s
**ingress** arm. The `Command::Edit` arm is at `controller.rs:667-675`
(`grep -n 'Command::Edit {'` → `:667`; the arm ends `.map(Err),` at `:675`,
and `:676` is the `match`'s closing brace), and the comment that says what R5
is claiming sits at `:661-666` — *"an edit is not an exchange: it writes
retained state and the loop continues to the top"*. The claim was true; only
the range was wrong.

**Corrected 2026-09-19 to `controller.rs:661-675`** — the comment included,
because it is the line that carries the evidence. `design.md` §8 R5 is an
artefact and was edited in place; `design-log.md` D-36 is append-only and the
range is struck there with the correction beside it, per `aeef6ab`'s precedent.
This entry stays as the record of the error.

**The cause, which is worth more than the entry.** The range came from reading
a `sed -n` window and naming the arm it happened to contain, which is the same
cause as the fifth. The rule is unchanged and was not followed: *cite from an
instrument that prints the number.* The class it belongs to is new, though —
the first bad citation found in `design.md` rather than in a ledger or a note,
and it survived four review rounds because it was written after the last one.

**Two entries were struck, and one of them was dangerous.** Both were written
here rather than in the ledger, so striking them costs nothing:

- *"the pre-repair §9's `set_accessible_value` claim appears nowhere here"* —
  false as written and not worth rescuing. `set_accessible_value` is §9's
  principal driver: the `LineEdit` and `Slider` rows both name it
  (`design.md:1285-1286`) and six obligation rows drive through it — AC-4, AC-6,
  AC-9, the debounce timer, the two-field window and the guard exception. A
  future agent acting on the note as it stood would delete a live driver.
- *"F-42's location line cites §5.5 I-G for a claim that is in §5.2"* — F-42's
  Location line cites **both** sections, and the claim was in both: §5.2 said
  `Reported` *"cannot express a non-finite number"* and §5.5 I-G said the same
  of the boundary. Both were repaired in the integration. The citation was never
  wrong.

### How this review has been run, and why

- Rounds 1 and 2: one Codex (`gpt-5.6-sol`) thread,
  `01a0ad06-ba40-7821-8d33-016b8cc4b0ee`. Round 3: a fresh thread,
  `01a0b212-239e-70d3-9a99-09729c82b971`. Use a **new** thread to raise; reuse a
  round's own thread only to set that round's outcomes. **Round 4 is not a Codex
  round** — credits ran out, and D-28 put it on a fresh Claude agent instead.
- Prompt the reviewer with **surfaces, not conclusions**
  (`docs/memory/dont-feed-the-raiser-your-finding.md`) and have it write to a
  file — reports truncate, and this one has truncated twice.
- **Spike anything a spike can answer** (D-19).
- The user asked for plainer prose: fewer punchy one-liners, more the way an
  engineer explains something to a colleague. §5.1-§5.3 are the model.

### Traps worth naming

- `design-log.md` is append-only; `design.md` §7 is current truth and rewrites an
  entry in place under its own immutable id. Two rules, two id sequences one
  hyphen apart.
- **`P-n` is a third id sequence**, and it overlaps the other way. `design.md`
  §4's guiding principles are `P-1`, `P-2`, `P-3`; `prototype-notes.md`'s
  findings are `P-1 … P-15`. The three collide, and all three of the
  prototype's land in §5.2 — which already cites §4's `P-3`. `design.md` now
  carries no prototype `P-n` at all and spells the survivor **§4's P-3**. Cite
  the file with the id, always.
- **A Slint `changed <property>` handler fires on a *change*, not on a write**,
  compared at flush time against the last value the tracker stored
  (`i-slint-core/properties/change_tracker.rs:138-141`). Writing a perturbation
  and then the real value inside one handler fires nothing.
- **Reading a widget's source tells you what an instance does, never how long the
  instance lives** (F-31, F-13's failure mirrored).
- One event-loop **arrangement**, one `[[test]]` target
  (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
- Two 64-to-32-bit narrowings were found in one round (§8 R7). Treat any
  host↔markup conversion as guilty until checked — and F-20 is the same class one
  level down: exact endpoints are not an operable range.
- **The guard's comparand has been wrong three times**, twice on reasoning and
  once corrected by measurement. D-23 touches it again. Re-run
  `numeric_guard.rs` rather than arguing about the exception.
- **A type only the controller can construct cannot be built in a Slint
  callback** (F-37). `AlternativeId` is not the only such type in
  `goad-semantics`.
- **Prose outside §9's obligations table binds nothing** (F-11, and then F-44 for
  exactly the same reason one round later).
- **A feature being compiled in is not the same as a value being populated**
  (F-50). Three rounds read `string_to_float`'s two branches correctly and none
  asked who writes the separator it branches on. When a mechanism's behaviour
  depends on configuration, find the **write site**, not just the read.
- **`input-type` gates typing, and nothing else** (F-52). `TextInput::insert` —
  the paste path — performs no validation at all, so a numeric `LineEdit` admits
  every string. An `input-type` is a typing aid, never a class the host may
  reason from.
- **`set_accessible_value` bypasses `input-type` too**, and it is §9's principal
  driver. It assigns `text` and calls `edited` from inside the markup
  (`widgets/fluent/lineedit.slint:16`), reaching no `TextInput` insertion logic —
  the same shape as the paste path. So the tier that looks like it exercises a
  validated control exercises the unvalidated one.
- **Find the write site, and then find *all* of them.** F-50 named three writers
  of the decimal separator and there are four; the fourth
  (`i-slint-core-1.17.1/translations.rs:304-310`) is the only one that reads the
  system locale, and it is compiled out by a feature rather than absent. A
  mechanism that is off is not a mechanism that cannot be on: the repair records
  the configuration (§8 R11) instead of only deleting the code path.
- **`{:e}` does not always carry a decimal point.** `format!("{:e}", 1e300)` is
  `1e300`. Measured, against a round-4 finding that generalised the other way.
- **A feature enabled by stratum 3 does not reach a build that excludes stratum
  3.** `goad-emit` takes `goad-shell` without `crates/goad`, so `-p goad-emit`
  and `-p goad-shell` resolve a shared dependency with stratum 2's features and
  not stratum 3's. Feature unification is per *build*, and "the workspace build"
  is one of several (F-54, D-35).

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the value channel and the epoch | **done** | 2026-09-19 |
| PHASE-02 — the draft's five values, and the kind-directed pure functions | **done** | 2026-09-19 |
| PHASE-03 — the edit channel | **done** | 2026-09-19 |
| PHASE-04 — the instant, the `jiff` feature, and `clock.rs`'s doc | **done** | 2026-09-19 |
| PHASE-05 — `text`, and the debounce's delivery | **done** | 2026-09-19 |
| PHASE-06 — the overlay | **done** | 2026-09-19 |
| PHASE-07 — `datetime` and the two pickers | **done** | 2026-09-19 |
| PHASE-08 — `number` and its two controls | **done** | 2026-09-19 |
| PHASE-09 — `choice`, and the retirement of `FieldForm` | pending | |

PHASE-04 is the only phase that can run beside another (`plan.md`
§*Sequencing & rationale*); every other pair overlaps on `app.slint`,
`view_model.rs`, `glass.rs` or `draft.rs`.

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — the value channel and the epoch

**Objective:** a present writes the form's *values* on every call and rebuilds
its *structure* only when the `view_id` changes, so a widget is corrected by a
guarded write instead of being destroyed and rebuilt. Discharges **AC-5**.

**Entry criteria, verified rather than assumed**

- **EN-1 — discharged.** `design-log.md:795` carries D-37 (design accepted and
  closed); `plan-log.md` §*2026-09-19 — the plan is accepted* accepts `plan.md`
  as written at `44fbd8e` and records that no plan review runs.
- **EN-2 — discharged.** `just check` run by this agent at `5849313`,
  `git status` clean: **exit 0**.

**Reading list**

*What is being changed*

- `crates/goad/src/glass.rs:155-213` — `option_rows` → `field_block`, the two
  functions that become one pass. `:203` is the irrefutable
  `let Edited::Checked(checked) = …`, which stays (PHASE-02 is the phase meant
  to meet it as a compile error).
- `crates/goad/src/glass.rs:23-35` — the `Glass::present` trait doc whose one
  deliberate exception becomes two (EX-6).
- `crates/goad/ui/app.slint:10-18` — `FieldRow` / `FieldBlock` / `OptionRow`;
  `:42-61` — the root's property and callback block; `:274-289` — the field
  repeater and the `CheckBox`.
- `crates/goad/Cargo.toml:8` — `autotests = false`; `:35-45` — the three
  existing `[[test]]` blocks.

*What reads a value off `FieldRow` today, and so is VA-1's list*

- `tests/renderer/fields.rs:200-208` — `drafted`, a synchronisation point.
- `tests/renderer/tree.rs:62-68` — `field(id, label, checked)`, the fixture
  builder; `:308-328` and `:349-382` read it back.
- `tests/renderer/wiring.rs:1617-1624` — `checked_in_row_model`.
- `tests/renderer/sizing.rs:38-44` — builds a `FieldRow` for a size probe.
- `tests/renderer/table.rs` — **nothing**: no `FieldRow` occurrence. It is in
  the plan's Surfaces defensively and needs no edit.

*Design sections that bind*

- §5.1 *the two channels* (`design.md:110-274`), §5.3 *ownership and the
  `Glass::present` contract change* (`:996-1073`), §5.4 *the order of the three
  writes* (`:1166-1180`), §5.5 **I-B** (`:1258-1259`) and **I-F**
  (`:1268-1275`), §7 **D8**, **D9**, **D15** (`:1371`, `:1372`, `:1378`), §9's
  AC-5 row (`:1487`) and the *what still needs a real loop* paragraph
  (`:1451-1463`), whose correction is that `init` is **not** among them.
- `research.md:183-191` Thread 3 §*The split channel* — the measurement.
- `plan.md:146-253` — PHASE-01 in full. PHASE-02 (`:255-364`) and PHASE-03
  (`:366-431`) for what is deliberately **not** done here.

*Prior art, read from the `slice-009-prototype` branch*

- `crates/goad/src/glass.rs` there — `option_models`, the three writes, the
  `shown: Option<ViewId>` field. Its overlay, `Pending` and `resolve` are
  PHASE-02/05/06's and are not taken.
- `crates/goad/ui/app.slint` there — the guard's exact spelling, and the
  counters.
- `crates/goad/tests/prototype/split.rs` there — the four probes VT-1/VT-2
  descend from, and `harness.rs`'s `retaining` (a hand-built `Outcome`, no
  backend), which the loop target reuses in spirit.
- `spike-fields/tests/with_loop.rs` there — the only measured arrangement for
  driving a `changed` handler: `ui.show()`, a **repeated** `slint::Timer`
  stepping a tick counter, `quit_event_loop` from the timer, assertions on the
  test thread.

*Memory*

- `a-present-destroys-the-widget-it-writes.md` — why `set_vec` is a rebuild,
  and the `inits` / guard-counter instrument.
- `change-handlers-need-an-event-loop.md` — `changed` fires nowhere under
  `init_no_event_loop`; `init` does (`design.md:1456-1463`, measured).
- `slint-testing-backend-initialises-once-per-process.md` — one arrangement,
  one `[[test]]`, one `#[test]` fn.
- `a-negative-control-that-does-not-compile.md` — read the test count, not the
  absence of `FAILED`.
- `shared-test-helper-lives-at-workspace-root-via-path.md` — every symbol of a
  `tests/support/` file must be reachable from every includer.

**Assumptions & STOP conditions**

Taken on faith, each with what makes it cheap to be wrong about:

- **A-a.** `root.values[field.slot]` tracks from inside a nested repeater, and
  replacing `values` wholesale destroys no element. Measured
  (`research.md:183-191`), doubly negative-controlled. VT-1 re-measures it in
  this markup.
- **A-b.** An `out property <int>` on the window root may be assigned from
  markup inside the component, including from inside a repeater. The prototype
  used `in-out`; the plan says `out`. If `out` does not compile, `in-out` is the
  fallback and is a local decision, not a design change.
- **A-c.** A `changed tick` handler fires under
  `init_integration_test_with_system_time()` **with the window shown**.
  Measured by `with_loop.rs`, which calls `ui.show()`; `SlintGlass::present`
  shows the window itself for a `Prompt` surface, so the arrangement gets it
  for free.
- **A-d.** Two counters are enough, where the prototype carried three. The
  third (`fires`) existed to tell *the handler never ran* from *the guard found
  agreement*. **VT-5 discharges that**: a negative control that must make the
  convergence counter *move* cannot pass unless the handler runs. VT-4 also
  asserts the epoch moved, so an epoch frozen at zero fails there rather than
  passing vacuously.

STOP and consult — do not improvise past any of these:

- **S-1.** The plan's EX-2 turns out to need `view_model.rs` (a `DrawnKind` on
  `PresentationField`) to give `FieldRow.kind` an honest value. `view_model.rs`
  is **not** in this phase's Surfaces and is PHASE-02's. If `Kind::Boolean`
  written at the one drawn kind is not acceptable, that is a plan question.
- **S-2.** The guard cannot be made to fire in the loop target at all, or VT-4
  cannot be made to go red under VT-5's injection. That is §8 R1 happening, and
  the row moves rather than being written where it is green.
- **S-3.** Any temptation to weaken, delete or `#[ignore]` an existing case to
  get green. VA-1 exists to catch exactly that.
- **S-4.** A dependency addition of any kind.

**Tasks**

- [x] T-1 markup: `Kind`, `FieldRow { id, label, kind, slot }`,
      `FieldValue { checked, text, number, index }`, `values`, `epoch`, the two
      counters, the `CheckBox`'s `init` and guard (EX-2, EX-4, EX-5)
- [x] T-2 `glass.rs`: `shown: Option<ViewId>`, `option_models` in one pass, the
      three writes in I-F order, `field_value` (EX-3, VA-2)
- [x] T-3 `glass.rs`: the `Glass::present` trait doc's second exception (EX-6)
- [x] T-4 VA-1: migrate every existing reader of `FieldRow.checked` to
      `values[slot]` — `fields.rs`, `tree.rs`, `wiring.rs`, `sizing.rs`
- [x] T-5 VT-1 and VT-2 in `tests/renderer/fields.rs`, each with an injection
      pass
- [x] T-6 the `[[test]]` target `event_loop_reassert` with its `main.rs` and
      one `#[test]` fn (EX-7), carrying VT-4
- [x] T-7 VT-5: the negative control, compiled and run, red confirmed, reverted
      and the revert confirmed by `git diff`
- [x] T-8 `just check` exits 0 (EX-1); sheet, Status and Harvest updated

**What landed, criterion by criterion**

| | discharged by | how it was checked |
|---|---|---|
| EX-1 | the gate | `just check` exit 0 |
| EX-2 | `ui/app.slint:27-29`, `:64-65` | read; `date`/`time` and the slider fields are **not** there, and the file says why |
| EX-3 | `src/glass.rs:104-171`, `:237-296` | read, and see VA-2 |
| EX-4 | `ui/app.slint:334-340` | VT-4 and its control |
| EX-5 | `ui/app.slint:87-88`, `:324`, `:338` | both counters are `out property <int>` on the window root, written from the field markup |
| EX-6 | `src/glass.rs:39-56` | read |
| EX-7 | `Cargo.toml:47-49`, `tests/event_loop_reassert/` | one `[[test]]`, one `main.rs`, one `#[test]` fn, `init_integration_test_with_system_time()` |
| VT-1 | `tests/renderer/fields.rs:652-703` | **red before the change** (`inits` 3 → 9) and after injection A |
| VT-2 | `tests/renderer/fields.rs:705-736` | red under injection B |
| VT-3 | the 187 pre-existing cases | unchanged and green, reading through `values[slot]` |
| VT-4 | `tests/event_loop_reassert/reassert.rs` | red under injection A (`inits` 2 → 4) and under VT-5 |
| VT-5 | the injection below | **compiled and ran**: `1 test … 0 passed; 1 failed`, `reasserts` 0 → 2 |
| VA-1 | four sites, the counts | 548 → 551 passing, exactly the three added; every other target's count identical; nothing `ignored` |
| VA-2 | `src/glass.rs:156`, `:159-162`, `:171` | the only writer of each of the three, in I-F order, with nothing between them that touches any of the three (`grep -n` over `src/`) |

**The injection passes** (`design.md` §9), each applied, run, read, reverted,
and the revert confirmed by `git diff`:

- **A — the rows are written on every present** (`if self.shown != showing` →
  `if true`). VT-1 red, `inits` 3 → 9; VT-4 red, `inits` 2 → 4. VT-2 stays
  green, which is what makes it a control rather than a second copy of VT-1.
- **B — the rows are written only for the first view ever shown**
  (→ `if self.shown.is_none()`). VT-2 red; VT-1 stays green. The two
  injections separate the two halves of D8: *writes too often* and *never
  writes again*.
- **VT-5 — the guard always writes** (the difference test removed from
  `changed tick`). VT-4 red on the `reasserts` clause, 0 → 2 — one write per
  field. This is the pass that proves the handler **fires**, which is what the
  prototype's third counter (`fires`) existed for and why two counters are
  enough here.

**Decisions taken during execution**

- **Two counters, not the prototype's three.** `fires` is not landed. The plan
  says two (EX-5) and VT-5 covers what the third was for: a control that must
  make `reasserts` *move* cannot pass unless the handler runs. VT-4 also reads
  the **epoch** and asserts it moved, so an epoch frozen at zero fails there
  rather than passing vacuously. Recorded because a later phase adding a text
  or numeric guard may want `fires` back, and this is the argument it has to
  beat.
- **`out property`, not `in-out`.** The prototype used `in-out`; the plan says
  `out`. `out` compiles and is assignable from a repeated child — checked
  first, because being wrong was assumption A-b.
- **VT-1 and VT-2 drive the production `serve`**, through `fields.rs`'s own
  `driving!` rig and a scripted backend, rather than presenting a hand-built
  frame. Two presents of one view is what a second exchange answering
  `view: null` already produces, and a replacement view is what a second
  answer carrying a view produces — so both cases are a person asking for
  another check, and the file needed no new helper. The loop target cannot do
  that (no runtime, no channel) and builds its `Controller` from a hand-made
  `Outcome`, which is the prototype's `retaining`.
- **No `tests/support/` file was extracted.** One loop target has one
  arrangement; a second one is PHASE-05's or PHASE-06's, and *that* is when
  the shape is known well enough to share.
- **`slot`'s overflow fallback is `i32::MAX`, not `0`.** The prototype used
  `unwrap_or(0)`, which would alias an impossible 2³¹-th field onto slot 0 and
  show it another field's value; `i32::MAX` indexes past the end, which Slint
  answers with a default. Unreachable either way — the `as` conversion is
  denied crate-wide, so *some* fallback has to be spelled.
- **`sizing.rs`'s fixture builds both channels in one pass**, numbering slots
  across options exactly as `glass.rs` does, rather than giving each option
  its own numbering and letting the surplus slots index past the end. The
  values are all defaults, but the vector is the right *length*: a fixture
  resting on the out-of-range default would be resting on the one failure I-F
  exists to prevent.

**Findings**

- **`docs/memory/a-present-destroys-the-widget-it-writes.md` is now half
  stale**, and a close-time fix rather than this phase's. It says
  *"`SlintGlass::present` ends in `self.options.set_vec(rows)`"* and
  *"every row element in the form is dropped and rebuilt on every present"* —
  both were true of `main` when it was written and neither is true now: the
  `set_vec` is guarded by the `view_id` and happens on a replacement view
  only. Everything else in the note — why the destruction was load-bearing,
  the repair, and how to measure it — is exactly what this phase implemented
  and is still right. It is `docs/memory/`, not canon, and out of this phase's
  Surfaces.
- **`design.md` §5.3's ownership table says `epoch` is written by `present`,
  every call**, and it is — but the row model's own line in that table is
  *"one view, immutable"* for `Presentation` and says nothing about the
  `Option<ViewId>` the glass now retains. The table lists *last presented
  `ViewId`* already (`design.md:1002`), so nothing is missing; noted only
  because it was checked.
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

### PHASE-02 — the draft's five values, and the kind-directed pure functions

**Objective:** the draft can hold what any of the five kinds is worth, and the
three pure functions that decide what a field shows and submits exist, are total
over all five kinds, and are unit-tested — **while `boolean` is still the only
kind the mapper draws**. Discharges no AC on its own; it is what PHASE-03 and the
four kind phases are written against.

**Entry criteria, verified rather than assumed**

- **EN-1 — discharged.** PHASE-01/EX-1 … EX-7 checked against the code, not
  inherited from the hand-over:

| PHASE-01 | claimed at | checked here |
|---|---|---|
| EX-1 | the gate | `just check` re-run by this agent at `3769095`, clean tree: **exit 0**. Target counts recorded below |
| EX-2 | `ui/app.slint:27-29`, `:64-65` | read: `Kind` with five variants, `FieldRow { id, label, kind, slot }`, `FieldValue { checked, text, number, index }`, `in property <[FieldValue]> values`, `in property <int> epoch`. `date` / `time` and the slider fields are **absent**, as EX-2 requires |
| EX-3 | `src/glass.rs:69`, `:156-171` | read: `shown: Option<ViewId>` retained; `set_values` (`:156`) → `set_vec` under `if self.shown != showing` (`:158-164`) → `set_epoch` (`:169-171`). Nothing between them touches any of the three |
| EX-4 | `ui/app.slint:332-339` | read: `property <int> tick: root.epoch; changed tick => { if (self.checked != root.values[field.slot].checked) { … } }` |
| EX-5 | `ui/app.slint:87-88`, `:324`, `:338` | read: `out property <int> inits` / `reasserts` on the window root, incremented from the `CheckBox`'s `init` and from the guard's convergence write — production markup, not a test copy |
| EX-6 | `src/glass.rs:39-48` | read: the row model is named as the second deliberate exception, with §5.3's argument |
| EX-7 | `Cargo.toml:47-49`, `tests/event_loop_reassert/main.rs` | read: a fourth `[[test]]`, its own `main.rs`, `init_integration_test_with_system_time()` |

  Baseline target counts at `3769095`, for the VA-1-shaped comparison at the
  end: `goad` lib **30**, `tests/renderer` **189**, `goad-boundary`
  `tests/checks` **43**, the three loop targets **1** each, `goad-shell` lib
  **71** / `tests/integration` **96**, `goad-semantics` lib **30**.

**Reading list**

*What is being changed*

- `crates/goad/src/draft.rs:26-29` — `Edited`, one variant, `Eq` derived;
  `:51-57` — `state_of`, infallible, answering `Checked(false)` for an absent
  key; `:84-88` — `submitted`, one arm; `:95-203` — the `#[cfg(test)] mod tests`
  that is the shape for this phase's units, and the `ids` fixture at `:107-138`
  that shows how an id is read off a normalized view.
- `crates/goad/src/view_model.rs:74-78` — `PresentationField`, which gains a
  `DrawnKind`; `:219-227` — `undrawn_form`, which **does not change** (VA-2);
  `:243-273` — `sift`, which builds the `PresentationField`.
- `crates/goad/src/glass.rs:298-304` — `field_value` and the irrefutable
  `let Edited::Checked(checked) = *state;` at **`:299`**, which this phase is
  meant to meet as a compile error. (`plan.md` PHASE-02's Notes cite it as
  `glass.rs:203`; that was its line before PHASE-01 moved it. The plan is an
  accepted artefact and is not edited from a phase sheet — recorded here so the
  next reader is not sent to the wrong line.) `:253` is its one caller.
- `crates/goad/src/controller.rs:215-242` — `answer`, whose `submitted(...)`
  call at `:230` is this phase's only edit outside the three files above;
  `:345` — `drawn_fields`, the walk it uses.
- `crates/goad/src/wire.rs:21-46` — `Command`'s derives and `Command::Edit`,
  which carries an `Edited` and so loses `Eq` with it.
- `crates/goad/src/install.rs:38-45` — the `edited` closure. Untouched this
  phase: `Edited::Checked` survives, and the `Reported` split is PHASE-03.

*Canonical types the new ones are built from*

- `crates/goad-semantics/src/protocol/canonical.rs:72-80` — `AlternativeId`,
  `new` is `pub(super)`; `:246-258` — `FieldKind`; `:261-275` — `Alternative`;
  `:350-377` — `Alternatives`, whose `new` rejects an empty list at `:362`;
  `:411-459` — `NumberRange` with `min()` / `max()`; `:103-110` — `Timestamp`.

*Design sections that bind*

- `design.md:275-994` — §5.2 end to end. §5.2 is this phase's specification;
  the parts that decide code here are the `draft.rs` block (`:623-665`), the
  callers-of-the-`Option` paragraph (`:666-686`), `Reported` and `interpret`
  (`:687-756`), the `resolve` prohibition (`:757-778`), the two `Eq` paragraphs
  (`:779-807`), the as-drawn bullets (`:808-848`) and the formatting rule
  (`:420-443`).
- `design.md:1260-1261` **I-C** (one site applies `R-57`), `:1262-1264` **I-D**
  (every id came off a view), `:1275-1285` **I-G** (a submitted number is
  finite, held at two places and at neither boundary type).
- `design.md` §7 — **D24** (`:1387`, `Finite` rather than a bare `f64`),
  **D25** (`:1388`, two types joined by one kind-directed `interpret`), **D10**
  (`:1373`, `DrawnKind` host-local), **D12** (`:1375`,
  `Chosen(AlternativeId)`).
- `plan.md:255-364` — PHASE-02 in full. `plan.md:366-431` (PHASE-03),
  `:517-645` (PHASE-05), `:731-831` (PHASE-07), `:833-940` (PHASE-08),
  `:942-1065` (PHASE-09) for what is deliberately **not** done here — in
  particular each of those four names *its own kind's value arm in `glass.rs`*
  in its Surfaces.
- `canon-delta.md:14-55` — **CD-1**, which VT-3 exercises: what an untouched
  field submits per kind, the `datetime` epoch's spelling, and the `max`-only
  consequence. Working authority, **not promoted here**.

*Prior art and measurement*

- `prototype-notes.md:182-211` **P-2** — `as_drawn`'s `choice` arm is not total
  under this crate's lint table; `DrawnKind::Choice { first, alternatives }` is
  what was taken.
- `prototype-notes.md:213-243` **P-3** — a `number`'s spelling is load-bearing;
  `f64`'s `Display` never goes scientific and `f64::MAX` spells 309 characters.
- `prototype-notes.md:340-382` **P-7** — two sets, not one trio: texts the
  control admits that *no* parse accepts, and texts a parse accepts
  *non-finitely*. `inf` is reachable by pasting. No code; the behaviour is
  already what `Finite` gives.
- `prototype-notes.md:384-412` **P-8** — `impl Eq for Finite {}` is *sound* and
  silently restores the derives above it. Written, compiled and deleted in the
  prototype.
- `prototype-handback.md:68-118` **P-10** — the `resolve` instrument, verified
  live against the tree.
- `prototype-handback.md:137-147` **P-4** — `jiff::tz::Offset`, `Offset::UTC`,
  `Timestamp::UNIX_EPOCH` and `display_with_offset` all compile and run under
  today's **featureless** `jiff` as `crates/goad` already takes it
  (`crates/goad/Cargo.toml:20`). So `Edited::Picked` and `submitted`'s datetime
  arm need no manifest change, and PHASE-04 is not an entry condition here.

*The instruments VA-1 checks a name against, read before any is written*

- `crates/goad-boundary/tests/checks/structure.rs:301-315` — the `resolve`
  identifier-word match over `crates/goad/src`; `src/scan.rs:225-235` —
  `mentions`, the word rule it uses.
- `crates/goad-boundary/tests/checks/vocabulary.rs:18-26` — the domain list.
- `crates/goad-boundary/tests/checks/purity.rs:17-27` — the purity path list.
  Its subject is stratum 1 only, so it does not reach this phase's files; read
  anyway, because VA-1 names it.

*Lint table facts this phase is shaped by* (`Cargo.toml:124-200`, the
`[workspace.lints.clippy]` block)

- `unwrap_used`, `expect_used`, `panic`, `unreachable`, `indexing_slicing` —
  all `deny`. Every new expression is total or it does not land.
- `as_conversions` and the four `cast_*` lints — `deny`. There is no `as` in
  this phase, which is why `FieldValue.number` stays unwritten (below).
- `pedantic = deny` brings `must_use_candidate`, so every new `pub` function
  carries `#[must_use]`.
- `dead_code` is `warn` in the manifest and an error under the gate's
  `-D warnings`. Everything this phase adds is `pub` in a `pub mod`
  (`lib.rs:6-16`), so the four not-yet-drawn `DrawnKind` variants and all six
  `Reported` variants are reachable and do not trip it. `slider_bounds` would
  be private with no caller and is **PHASE-08's**.

*Memory*

- `docs/memory/a-green-test-can-assert-a-proxy.md` — why every new case gets an
  injection pass (`design.md:1408-1414`).
- `docs/memory/a-negative-control-that-does-not-compile.md` — read the test
  count, not the absence of `FAILED`.
- `docs/memory/enumerate-the-class-not-the-instances.md` — the shape of
  `interpret`'s `None` surface argument.

**Assumptions & STOP conditions**

Taken on faith, each with what makes it cheap to be wrong about:

- **A-a.** `jiff::tz::Offset` is reachable and `Offset::UTC` /
  `Timestamp::display_with_offset` compile with `jiff`'s default features off.
  Measured by the prototype (P-4) and re-measured here by VT-2 going green. If
  wrong, the phase stops: adding a feature is a dependency change and PHASE-04's.
- **A-b.** Nothing in the workspace needs `Eq` on `Edited`, `Command`,
  `PresentationField`, `FieldBlock` or `PresentationOption`. The prototype
  dropped the first two and broke no call site (P-8); the last three are new
  losses this phase causes, because `DrawnKind` carries a `NumberRange` and an
  `Alternatives`, neither of which is `Eq`. Cheap: a `HashSet`/`BTreeSet` of one
  of them would be a compile error naming the site.
- **A-c.** `clippy::float_cmp` (pedantic, therefore `deny`) does not fire on an
  `assert_eq!` over `f64` in a unit test, because the `==` is inside a `core`
  macro expansion. If it does, the assertion is written as a bit comparison and
  that is a local decision, not a design change.

STOP and consult — do not improvise past any of these:

- **S-1.** Any temptation to write `impl Eq for Finite {}`. It is sound, it
  compiles, it restores the derives above it, and it is the trap §5.2 states at
  the leaf. This is not a stop so much as a standing prohibition.
- **S-2.** Any temptation to close `interpret`'s match with `_ => None`. The
  `None` surface is three cases and the match is written so that a sixth
  `Reported` variant or a sixth `DrawnKind` variant is a compile error.
- **S-3.** A kind's *control* wanting to be drawn, or `FieldRow` / `FieldValue`
  wanting a new slot. VA-2 is the line: `undrawn_form` does not change and no
  fixture moves.
- **S-4.** `slider_bounds`, or anything else whose caller is a later phase.
- **S-5.** Weakening, deleting or `#[ignore]`-ing an existing case to go green.
- **S-6.** A dependency or feature addition of any kind.

**Findings raised while expanding the phase**

- **EX-9's second clause cannot be implemented as written, and the design
  settles it.** EX-9 says *"`controller::answer` supplies an untouched field's
  value through `interpret` rather than through a second `as_drawn` call
  site."* `interpret`'s first parameter is a `&Reported` — what a **widget**
  reported — and `answer` has no widget and no report; it is submitting a field
  nobody touched. There is no call to make. `design.md:666-676` states the rule
  the plan was compressing, and states it the other way round: *"There are two
  sites that apply `as_drawn`, and only one of them is in `controller.rs`:
  `answer`, because `R-58` forbids omitting a value for a drawn field; and
  `interpret` (below), to supply the number a numeric text falls back to."*
  The two sites are `answer` and `interpret`; what `glass.rs` must stay out of
  is `as_drawn`, and that is the constraint the clause was reaching for.
  PHASE-02's own Surfaces line agrees with the design — it scopes `controller.rs`
  to *"(`answer`'s as-drawn call only)"*. Taken: `answer` calls `as_drawn`,
  `glass.rs` does not, per `plan.md:3-4` (*the plan never overrides the design;
  if it seems to, the plan is wrong*). Not handed back as a plan defect because
  the two readings do not produce different work — one of them produces no code
  at all — and `plan.md` is not editable from here. Reported to the team lead.

**Tasks**

- [x] T-1 `draft.rs`: `Finite` — private field, fallible constructor, no `Eq`
      (EX-2) — and VT-1, with its injection pass
- [x] T-2 `draft.rs`: `Edited`'s five variants and `Reported`'s six (EX-2,
      EX-3); `Eq` dropped on `Edited` and on `wire.rs`'s `Command`
- [x] T-3 `draft.rs`: `state_of -> Option<Edited>` and `submitted`'s five arms
      (EX-4), and VT-2 with its injection pass; the four existing units rewritten
      for the `Option`, none deleted
- [x] T-4 `view_model.rs`: `DrawnKind` with `Choice` carrying the first id
      beside the list, and `PresentationField.kind` (EX-5); `undrawn_form`
      untouched (VA-2)
- [x] T-5 `view_model.rs`: the number formatter (EX-8), and VT-5 with its
      injection pass
- [x] T-6 `view_model.rs`: `as_drawn` (EX-6), and VT-3 with its injection pass
- [x] T-7 `view_model.rs`: `interpret` (EX-7), and VT-4 with its injection pass
- [x] T-8 `glass.rs`: `field_value` over `Option<Edited>`, total across the five
      variants, replacing the irrefutable `let` (EX-9, first half)
- [x] T-9 `controller.rs`: `answer`'s as-drawn call (EX-9, second half)
- [x] T-10 VA-1 and VA-2 walked; `just check` exits 0 (EX-1); sheet, Status and
      Harvest updated

**What landed, criterion by criterion**

| | discharged by | how it was checked |
|---|---|---|
| EX-1 | the gate | `just check` **exit 0**. Every target's count identical to the baseline except `goad` lib, 30 → 42 |
| EX-2 | `src/draft.rs:18-62` (`Finite`), `:64-97` (`Edited`) | read: private field, `new` fallible, `ZERO`, `get`; derives are `Debug, Clone, Copy, PartialEq, PartialOrd` and **no `Eq`**. `Edited`'s five variants are §5.2's, payload for payload |
| EX-3 | `src/draft.rs:99-126` (`Reported`), `:64-74`, `src/wire.rs:18-28` | read: six variants; `Edited` derives `PartialEq` without `Eq`, and `Command` with it. The compiler found `Command`: `E0277: the trait bound Edited: Eq is not satisfied` at `wire.rs:42` |
| EX-4 | `src/draft.rs:145-182` (`state_of`), `:184-212` (`submitted`) | read: `-> Option<Edited>`; five arms, one per variant, still the only `pub(crate)` application of `R-57` and still the only site in the workspace that names a JSON type for a field value (`grep -rn "serde_json::Value::" crates/goad/src`) |
| EX-5 | `src/view_model.rs:78-122` (`PresentationField` at `:83`, `DrawnKind` at `:102`) | read: `DrawnKind` with five variants, `Choice { first, alternatives }` carrying the first id beside the list; `PresentationField` carries a `DrawnKind`. `PresentationOption`, `FieldBlock` and `PresentationField` lose `Eq` with it — `NumberRange` and `Alternatives` are not `Eq` |
| EX-6 | `src/view_model.rs:498-538` (`as_drawn` at `:511`) | read against §5.2's as-drawn bullets one by one, and measured by VT-3: `false`, `""`, the declared minimum or zero **with its spelling**, the first alternative's id, and `1970-01-01T00:00:00+00:00` |
| EX-7 | `src/view_model.rs:530-641` (`interpret` at `:569`) | read: signature as §5.2 gives it; the `None` surface argued below; `as_drawn`'s number rule reached through the shared `drawn_number` where `held` is `None`, so no caller writes the fallback |
| EX-8 | `src/view_model.rs:427-455` (`spelled` at `:446`) | read, and measured by VT-5 including the threshold from both sides |
| EX-9 | `src/glass.rs:299-341` (`field_value` at `:322`, `markup_kind` at `:289`, its doc at `:282`), `src/controller.rs:226-240` (`answer`'s as-drawn call at `:237`) | read: `field_value` takes `Option<&Edited>` and is total over the five variants and the `None`; `answer` applies `as_drawn` where the draft holds nothing. See *Findings* on EX-9's second clause |
| VT-1 | `draft.rs::a_finite_refuses_every_number_json_cannot_carry` | injection **A**, red |
| VT-2 | `draft.rs::each_kind_submits_the_json_type_r_57_names` and `::a_picked_datetime_submits_the_offset_it_was_picked_in` | injections **B** and **C**, red |
| VT-3 | `view_model.rs::as_drawn_answers_every_kind` and `::an_untouched_field_submits_what_canon_delta_cd_1_states` | injections **C**, **E**, **F** and **L**, red. The CD-1 case carries the `datetime` epoch's exact spelling and the `max`-only range that submits `0` |
| VT-4 | `view_model.rs::an_in_kind_report_becomes_what_the_draft_holds`, `::a_chosen_index_no_alternative_has_is_refused`, `::a_slider_reporting_a_non_finite_number_is_refused`, `::a_report_whose_variant_is_not_the_drawn_kind_is_refused`, `::a_numeric_text_no_finite_parse_accepts_keeps_the_number_the_field_held`, `::an_untouched_numeric_field_falls_back_to_what_it_was_drawn_showing` | injections **A**, **G**, **H**, **I**, **J**, **L**, red. The mismatch case enumerates **all twenty-four** pairs and asserts the count, so no pair can pass by being silently in-kind |
| VT-5 | `view_model.rs::a_number_spells_short_and_re_parses_to_the_number_it_came_from` | injections **K** and **L**, red — the two halves of the rule, *never scientific* and *always scientific* |
| VA-1 | the three needle lists, walked before each name was written | `grep -rniE '(^\|[^a-z])resolves?([^a-z]\|$)'` over `crates/goad/src` returns only comments and pre-existing lines; the domain list returns only the English word *site* in comments, which `code_of` cuts; the purity list is stratum 1's and returns only pre-existing `main.rs` / `diagnostics.rs` / `startup.rs` lines. The instruments themselves are in the gate and are green — `goad-boundary` 43 passing, unchanged |
| VA-2 | `git diff 3769095 -- crates/goad/src/view_model.rs`, and `git diff --stat 3769095 -- crates/goad/tests/ examples/` | `undrawn_form`'s body is untouched — the only `+` lines naming it are two comments — and **no test or example file was touched at all**. `tests/renderer` is 189 before and after |

**The injection passes** (`design.md` §9), each applied, run, read, reverted, and
the revert confirmed by `git diff` returning empty. The runner is
`scratchpad/inject.py`; it patches, runs, restores from the string it read, and
prints `git diff --stat`.

| | the defect planted | what went red |
|---|---|---|
| A | `Finite::new` accepts every `f64` | VT-1, and both `interpret` cases that rest on the refusal |
| B | `submitted`'s `Adjusted` arm sends the **text** | VT-2, VT-3's wire half |
| C | `submitted`'s `Picked` arm spells `Z` (jiff's `Display`) instead of the offset | VT-2 both cases, VT-3's wire half |
| D | `state_of` answers `Checked(false)` for an absent key — the behaviour it had | the two draft cases about absence |
| E | `drawn_number` ignores the declared minimum | VT-3 both halves, and the untouched-fallback case |
| F | `as_drawn`'s `choice` arm takes the **last** alternative | VT-3 both halves |
| G | a `number` field accepts an out-of-kind `Typed` report | the twenty-four-pair case |
| H | a `Chosen` index falls back to the first alternative instead of being refused | the out-of-range case |
| I | a refused numeric text forgets the number the field held | the verbatim case |
| J | a refused numeric text is **repaired** to the number's spelling — the substitution D-33 refuses | the verbatim case, and the untouched fallback |
| K | `spelled` never reaches for `{:e}` (threshold 400) | VT-5 |
| L | `spelled` always uses `{:e}` (threshold 0) | VT-5, VT-3, and the in-kind `AdjustedValue` case |
| M | `field_value` inverts the draft's boolean | **6** renderer cases |
| O | an untouched field is drawn ticked | **5** renderer cases |
| P | `answer` invents `Checked(true)` instead of applying `as_drawn` | **5** renderer cases, including the two `R-58` ones |
| N | `markup_kind` asserts `Kind::Boolean` for every drawn field | **nothing.** See *Findings* |

**Decisions taken during execution**

- **`Finite` derives `Clone` and `Copy` as well as `PartialEq` and
  `PartialOrd`.** §5.2 says *"`PartialEq` and `PartialOrd` and nothing else"*,
  which is an argument about **equality** and reads as an exhaustive derive list
  only by accident: `Edited` is `Clone`, so `Finite` must be, and §5.2's own
  signature `pub fn get(self) -> f64` takes the value. Neither is a route back
  to the `Eq` trap, and the type's doc says so at the leaf where the trap is.
- **`interpret` is a nested match on the kind, then the report, with no `_` in
  either position.** Five outer arms over `DrawnKind`, each with exactly two
  inner arms over `Reported` — the in-kind one, and an or-pattern naming the
  other five variants. Thirty pairs, thirty explicit destinations. A sixth
  `Reported` variant is a compile error in all five arms and a sixth `DrawnKind`
  is a compile error in the outer match. The tuple form with one
  `(A | B | C | D | E | F, _) => None` arm is shorter and was rejected: it
  makes a sixth `DrawnKind` answer `None` by silence.
- **`drawn_number(&NumberRange) -> Finite` is where `interpret` reaches
  `as_drawn`'s number rule**, rather than destructuring `as_drawn(kind)`. EX-7
  says `interpret` *"applies `as_drawn` itself where `held` is `None`"*, and the
  obligation that clause carries is that **no caller** writes
  `state_of(…).unwrap_or_else(|| as_drawn(kind))` — which is held. Destructuring
  an `Edited` back out of `as_drawn` inside the `Number` arm would need a
  `_ => Finite::ZERO` fallback that nothing can reach, which is the
  `expect`-is-unreachable shape §5.2 declines. Both `as_drawn` and `interpret`
  call `drawn_number`, so the rule still has exactly one statement.
- **`FieldRow.kind` is now `markup_kind(&field.kind)`.** PHASE-01 left
  `Kind::Boolean` as an honest constant with a comment saying it stops being one
  at PHASE-02 (`plan.md` PHASE-02/EX-5); the constant moved one level in, to
  `sift`, where `undrawn_form` is the thing that makes it honest. `markup_kind`
  is total over `DrawnKind`, so a sixth kind is a compile error in `glass.rs`
  too.
- **`field_value`'s `None` and its four undrawn arms are one arm**, not two.
  Splitting them — which reads better, and which is what PHASE-07 will do when
  an unpicked `datetime` starts reading *not set* — is a `clippy::match_same_arms`
  error while the two answers agree. Tried, measured, reverted, and the reason
  is in the comment so the next phase does not re-derive it.
- **`field_value` writes only `checked`.** The other four arms answer the
  default, because `undrawn_form` draws only `boolean` and no control reads
  another slot yet. Each remaining arm is owned by the phase that draws its
  control and says so in that phase's own Surfaces (`plan.md` PHASE-05, -07,
  -08, -09), and two of them cannot be written here at all: `index` needs the
  drawn alternatives to locate the held id, and `number` needs an `f64` → `f32`
  narrowing, which is `as_conversions = "deny"` and is `slider_bounds`'
  business. Writing a slot no control reads would be untested code landing a
  phase early — the same objection the plan makes to `slider_bounds`.
- **The test fixtures are per-module and duplicated across `draft.rs` and
  `view_model.rs`.** Both need an id read off a normalized view, and neither can
  reach the other's `#[cfg(test)] mod tests`. A shared `#[cfg(test)]` module
  would need `lib.rs`, which is not in this phase's Surfaces. Recorded rather
  than done.

**Findings**

- **`markup_kind`'s four non-`boolean` arms are unmeasured, and correctly so
  (injection N).** Planting `Kind::Boolean` for every drawn kind leaves the whole
  suite green, because `undrawn_form` still sends the other four kinds to
  `Undrawn::FieldForm` and nothing can construct a drawn field of another kind.
  It is the first thing PHASE-05 measures — its EX-2 draws a `LineEdit` for a
  `text` field, which the markup's `if` chain selects on exactly this value — so
  no case is owed here. Recorded because a green injection is the shape of a
  test asserting a proxy, and this one is green for a reason rather than by
  accident.
- **The plan's PHASE-02 Notes cite `glass.rs:203` for the irrefutable `let`;
  it was at `:299`.** PHASE-01 moved it. The line's *doc* said what the plan
  said it said, and the compile error arrived exactly as predicted
  (`E0005: refutable pattern in local binding`). `plan.md` is an accepted
  artefact and was not edited.
- **`design.md` §5.2's `Reported::Chosen(u32)` reaches the alternatives through
  `usize::try_from`, which is a fourth way to answer `None` on paper.** It
  cannot fire on any platform this workspace builds for, and where it could it
  would mean *an index no alternative has*, which **is** case 1. Noted because
  the `None` surface being three cases is a claim the phase is asked to defend,
  and this is the one mechanism inside the function that is not one of the three
  by inspection.
- **`SPEC-001`'s wire key for a `choice` field's alternatives is `options`, not
  `alternatives`.** `normalize_alternatives` says so in as many words — *"The
  path says `options`, because that is the key a backend author wrote"* — and
  both new fixtures were written the other way first and failed with
  `EmptyAlternatives { at: "view.options[0].fields[0].options" }`. The canonical
  **type** is `Alternatives`; only the wire key is `options` (`R-16`, `R-53`).

### PHASE-03 — the edit channel

**Objective:** the markup hands back a typed report of what one widget did, and
the host interprets it against the drawn field before anything reaches the
draft. Discharges no AC on its own; it is the second half of the reshaping
PHASE-02 began, and it is what PHASE-05 onwards send their edits through.

**Entry criteria, verified rather than assumed**

- **EN-1 — discharged.** PHASE-02/EX-1 … EX-9 checked against the code rather
  than inherited from the hand-over:

| PHASE-02 | claimed at | checked here |
|---|---|---|
| EX-1 | the gate | `just check` re-run by this agent at `0d7f553`, clean tree: **exit 0**. Counts below |
| EX-2 | `src/draft.rs:18-62`, `:64-97` | read: `Finite` at `:42`, private field, `new` at `:54` returning `Option`, `ZERO` at `:49`; derives `Debug, Clone, Copy, PartialEq, PartialOrd` (`:41`) and **no `Eq`**. `Edited` at `:75`, five variants |
| EX-3 | `src/draft.rs:99-126`, `src/wire.rs:27-28` | read: `Reported` at `draft.rs:113`, six variants — `Checked`, `Typed`, `AdjustedText`, `AdjustedValue`, `Chosen`, `Picked`. `Edited` derives `PartialEq` without `Eq` (`draft.rs:74`); `Command` likewise (`wire.rs:27`). **This is EX-3's second clause, and it is already true on entry** — see the Findings note below |
| EX-4 | `src/draft.rs:160`, `:193` | read: `state_of -> Option<Edited>`; `submitted` is `pub(crate)`, five arms |
| EX-5 | `src/view_model.rs:83`, `:102` | read: `PresentationField` carries a `DrawnKind`; `DrawnKind::Choice { first, alternatives }` |
| EX-6 | `src/view_model.rs:511` | read: `as_drawn(&DrawnKind) -> Edited`, five arms |
| EX-7 | `src/view_model.rs:569` | read: `interpret(&Reported, Option<&Edited>, &DrawnKind) -> Option<Edited>`; nested match, no `_` in either position, thirty explicit destinations |
| EX-8 | `src/view_model.rs:446` | read: `spelled`, `{:e}` past 24 characters |
| EX-9 | `src/glass.rs:322`, `src/controller.rs:237` | read: `field_value(Option<&Edited>)`; `answer` applies `as_drawn` where the draft holds nothing |

  Baseline target counts at `0d7f553`, for the comparison at the end: `goad`
  lib **42**, `tests/renderer` **189**, `goad-boundary` `tests/checks` **43**,
  the three loop targets **1** each, `goad-shell` lib **71** /
  `tests/integration` **96**, `goad-semantics` lib **30**. 563 in all.

**Reading list**

*What is being changed*

- `crates/goad/ui/app.slint:27-29` — `Kind`, `FieldRow`, `FieldValue`; `:98` —
  `callback edited(string, string, string, bool)`, which gains a `FieldEdit`
  in place of the bare `bool`; `:314-345` — the `CheckBox`, whose `toggled` is
  at `:341-343`.
- `crates/goad/src/wire.rs:27-28` — `Command`'s derives, already `PartialEq`
  without `Eq`; `:38-51` — `Command::Edit`, whose `value: Edited` becomes a
  `Reported`.
- `crates/goad/src/install.rs:37-45` — the `edited` closure, the whole of EX-4.
  `:14` — the `use crate::draft::Edited` that becomes `Reported`.
- `crates/goad/src/controller.rs:257-288` — `edit`, which gains the
  interpretation on the walk it already makes (`drawn_fields` at `:352`,
  `selected` at `:327`); `:674-680` — `dispatch`'s `Command::Edit` arm.
- `crates/goad/tests/renderer/wiring.rs` — **the twelve sites**, in five cases:
  `an_edit_is_refused_by_each_selector_that_fails_and_records_nothing`
  (`:1246`; sites at `:1247`, `:1260`, `:1264`, `:1268`, `:1273`, `:1293`),
  `an_answer_carries_a_value_for_every_drawn_field_of_the_option_it_names`
  (`:1305`; site at `:1309`),
  `an_answer_carries_no_value_for_another_option_or_for_an_undrawn_field`
  (`:1341`; sites at `:1354`, `:1357`),
  `the_next_present_writes_every_control_back_from_the_draft` (`:1396`; site at
  `:1403`), and
  `an_edit_starts_no_exchange_and_a_refused_one_reports_where_a_refused_click_does`
  (`:1531`; the two `Command::Edit` literals at `:1564` and `:1580`).
- `crates/goad/tests/renderer/fields.rs:234-245` — `click`, which drives the
  `CheckBox`'s default action through `install`'s closure; `:220-222` —
  `drafted`, the synchronisation point. VT-3's vehicle.

*Left alone this phase, deliberately*

- `wiring.rs`'s own `TWO_FORMS` (`:1158`) is **not** `fields.rs`'s: it carries
  a third field, `noted`, declared `text`. Four of the twelve sites read it
  (`:1268`, `:1273`, `:1354`, `:1357`), and `:1273`'s refusal and `:1343-1352`'s
  guard assertion both rest on `noted` still being undrawn. PHASE-05 migrates
  it (PHASE-05/EX-9).
- `draft.rs` — PHASE-02 already rewrote its ten unit sites for the split.
- `glass.rs` — the value channel is untouched; this phase is the edit channel.

*Design sections that bind*

- `design.md:687-756` — `Reported`, `interpret`, and the `None` surface's three
  cases. `:757-778` — the `resolve` prohibition, which binds every new name
  here. `:779-807` — the two `Eq` paragraphs.
- `design.md:296-309` — the Slint block: `FieldEdit`'s fields and
  `callback edited(string, string, string, FieldEdit)`.
- `design.md:1262-1264` **I-D** — every id on the wire came off a view;
  `:1275-1285` **I-G** — a submitted number is finite, held at two places and
  at neither boundary type.
- `design.md:1500-1511` — §9's three obligations with no widget: *every case
  that builds a `Command::Edit` or an `Edited` is rewritten for the `Reported`
  split — twelve in `tests/renderer/wiring.rs`, ten in `draft.rs`'s own tests,
  and the one closure in `install.rs` they exercise.*
- `plan.md:367-432` — PHASE-03 in full. `plan.md:518-646` (PHASE-05) for what
  is deliberately **not** done here: `pending.rs`, the debounce,
  `Wire::send -> bool`, `Command::Choose`'s new shape and the `TWO_FORMS`
  migration are all PHASE-05's.
- `docs/specs/001-host-backend-protocol.md` — `R-52` (a field id is unique
  within its option), and the refusal taxonomy `Refused::UnknownField` belongs
  to. EX-5 adds no class to it.

*Prior art*

- `crates/goad/src/glass.rs:282-321` — `markup_kind` and `field_value`'s docs.
  They are the shape for a host-side map that is total over five kinds while
  only one is drawn, and they say which phase owns each remaining arm. The
  closure here is the same shape pointing the other way.
- `notes.md` §*Traps worth naming* — *a type only the controller can construct
  cannot be built in a Slint callback* (F-37), which is the whole reason
  `Reported` exists.

**Assumptions & STOP conditions**

- **A-a.** A Slint struct literal may name a subset of a struct's fields, and
  the rest default. Measured by the design's spike (`design.md:277-279`) and
  re-measured here the moment `app.slint` compiles.
- **A-b.** `Kind` and `FieldEdit` reach Rust as a plain enum and a plain struct
  through `generated.rs`'s `include_modules!()`, exactly as `Kind`,
  `FieldRow` and `FieldValue` already do. Cheap to be wrong about: a compile
  error naming the item.
- **A-c.** Nothing outside `install.rs` and `controller.rs` names
  `Command::Edit`'s payload type. Verified by `grep -rn "Command::Edit"` over
  the workspace before the change: `install.rs:39`, `controller.rs:674`, and
  `wiring.rs:1564`/`:1580`.

STOP and consult — do not improvise past any of these:

- **S-1.** Any judgement moving *into* the Slint closure: a parse, a fallback
  value, a refusal, a clamp, a `try_from` whose failure is absorbed. EX-4 is
  the criterion and it is the one the plan says is most likely to erode.
- **S-2.** A new refusal class. A `None` from `interpret` takes the **existing**
  `Refused::UnknownField` posture (EX-5).
- **S-3.** Hand-writing an `Eq` anywhere in the `Finite` / `Edited` / `Command`
  tower. Standing prohibition, `design.md:779-807`.
- **S-4.** Weakening, deleting or `#[ignore]`-ing an existing case to go green.
  VA-1 exists to catch it.
- **S-5.** Touching a file the Surfaces line does not name — in particular
  `glass.rs`, `view_model.rs`, `draft.rs`, `main.rs` or `pending.rs`.
- **S-6.** Migrating `wiring.rs`'s `TWO_FORMS` off `text`. PHASE-05's.

**Findings raised while expanding the phase**

- **EX-3's second clause was already true on entry, and could not have
  waited.** *"`Command` drops its `Eq` derive with `Edited`"* — `Edited` lost
  its `Eq` in PHASE-02/EX-3, and `Command` carries an `Edited`, so the two
  losses are one compile: `wire.rs:42` failed with
  `E0277: the trait bound Edited: Eq is not satisfied` the moment `draft.rs`
  changed. PHASE-02's Surfaces were amended for it (`plan-log.md`,
  2026-09-19). Verified here at `wire.rs:27` and treated as discharged; not
  undone and redone. What is left of EX-3 is `Command::Edit` carrying a
  `Reported`.
- **The closure cannot be total over `Kind` in this phase, and the design's own
  `datetime` path is why.** `Reported::Picked` needs a `Timestamp` and an
  `Offset`, composed by `instant.rs::compose` from a `date` and a `time` that
  `FieldEdit` does not carry until PHASE-07 — EX-2 lists five fields and the
  markup's standing rule is that nothing is declared before a control reads it.
  §5.4's picking diagram already gives that arm an `Option` shape in as many
  words: *"Composing --> Idle: one `edited()`, or nothing if `compose` fails"*.
  So the mapper here answers `Option<Reported>` and the closure sends only on
  `Some`; the four kinds no control draws are one grouped arm naming the phase
  that fills each, which is `glass.rs::field_value`'s shape pointing the other
  way. This is **not** a refusal decided in the closure: `undrawn_form` draws
  only `boolean`, so no `FieldEdit` of another kind is constructible today.
- **Two later phases will need `install.rs` in their Surfaces, and it is not
  there.** `install.rs` appears in PHASE-03, PHASE-05 and PHASE-07's Surfaces
  and **not** in PHASE-08's or PHASE-09's — yet PHASE-08 draws the two `number`
  controls and PHASE-09 draws the `ComboBox`, and each needs its own arm in the
  mapper this phase writes. Same shape as the gap PHASE-02 found. Reported to
  the team lead; `plan.md` is an accepted artefact and is not edited from here.
- **The design does not settle how the closure tells a numeric `LineEdit`'s
  report from a `Slider`'s, and it is a hole in the *design*, not in the
  plan.** Three citations hold it, each verified rather than recalled:
  `design.md:500-502` — the controls table puts both controls under the one
  protocol kind, the `LineEdit` *"Sends `text`"* and the `Slider` *"Sends
  `number`"* (§7 D16); `design.md:697-698` — `Reported` carries
  `AdjustedText(String)` and `AdjustedValue(f32)`, two variants for that one
  kind; and `design-log.md` **D-12** — *"`FieldEdit.kind` carries the
  **protocol** kind and never the control"*. So six report variants meet a
  five-valued discriminant, and the two that collide are exactly the two a
  `number` can raise.

  **No slot value separates them, and the one that looks as though it might is
  a measured case going the other way.** *Empty `text` means a `Slider`* would
  silently eat a cleared numeric field, which A-2 measured and which must reach
  the draft as `AdjustedText("")` — `design.md`'s edges table, *numeric field
  cleared to `""`*. That is `CLAUDE.md`'s second invariant in miniature: an
  ambiguous message fails rather than being guessed at, and this one is
  ambiguous at the boundary rather than on the wire.

  Nothing in this phase turns on it — `number` has no arm here — so it is
  recorded and **not** resolved, and nothing in the phase is shaped around a
  resolution that does not exist. It goes to the user as a design question
  before PHASE-08, not during it. What this phase owes it is a price, below.

**Tasks**

- [x] T-1 `app.slint`: `FieldEdit`, the widened `edited` callback, and the
      `CheckBox`'s literal (EX-2)
- [x] T-2 `wire.rs`: `Command::Edit` carries a `Reported` (EX-3)
- [x] T-3 `install.rs`: the closure maps a `FieldEdit` to a `Reported` and
      nothing else (EX-4)
- [x] T-4 `controller.rs`: `edit` interprets on the walk it already makes, and
      a `None` takes the existing `UnknownField` posture (EX-5)
- [x] T-5 `wiring.rs`: the twelve sites rewritten (EX-6), VT-1 and VT-2 with
      their injection passes
- [x] T-6 `fields.rs`: VT-3 — the toggled `CheckBox` still reaches the draft and
      still answers. **No edit to the file was needed**; see the criterion table
- [x] T-7 VA-1's one-by-one diff of the twelve; `just check` exits 0 (EX-1);
      sheet, Status and Harvest updated
- [x] T-8 `tests/renderer/tree.rs` — a **thirteenth** site, outside the
      Surfaces as written. Unblocked: the user amended PHASE-03's Surfaces to
      name it (`plan.md`, 2026-09-19), and PHASE-08's and PHASE-09's to name
      `install.rs`
- [x] T-9 `diagnostics.rs`: `Refused::UnknownField`'s doc comment, two stale
      clauses, no code. Added to the Surfaces by the user after this phase
      raised it (`plan-log.md`, 2026-09-19)

**A thirteenth site, and it is outside the phase's Surfaces.** Widening
`callback edited` to carry a `FieldEdit` breaks exactly one case the Surfaces
line does not name: `tests/renderer/tree.rs:424`
`activating_a_field_control_fires_edited_with_all_four_selectors`, which binds
`window.on_edited` directly to capture the callback's arguments, and
`tree.rs:33`'s `type EditedArgs = (String, String, String, bool)` that holds
them. Verified to be the only one, by counting compile-error locations per file
across `--all-targets`: `wiring.rs` **12**, `tree.rs` **1**, and the ten
reported in `controller.rs` are one line — `:278`, the *arguments to this
method are incorrect* note repeated per caller.

The count of twelve is **not** wrong. `plan.md` PHASE-03/EX-6 and `design.md`
§9 both enumerate *constructors of a `Command::Edit` or an `Edited`*, and this
case constructs neither — it binds the markup callback. The enumeration's
reach is what missed it, not its arithmetic.

STOP taken rather than the edit: `docs/AGENTS.md` §Execute, and PHASE-02 found
the same shape one phase ago. Reported to the team lead.

**What landed, criterion by criterion**

| | discharged by | how it was checked |
|---|---|---|
| EX-1 | the gate | `just check` **exit 0**. Every target's count identical to the baseline except `tests/renderer`, 189 → **190** — the one case PHASE-03/VT-2 adds. 564 in all — *and that is the **gate** total, while the per-target list beside it enumerates `cargo test --workspace`, which is 529. Both numbers are right and the sentence names one denominator for two quantities; annotated 2026-09-19 by the team lead, see §Open* |
| EX-2 | `ui/app.slint:47` (`FieldEdit`), `:112` (the callback), `:355-369` (the `CheckBox`'s `toggled`) | read: `struct FieldEdit { kind: Kind, checked: bool, text: string, number: float, index: int }` and `callback edited(string, string, string, FieldEdit)`. The literal names **two** fields, `kind` and `checked`. `date` / `time` are absent, for the reason `FieldValue`'s are. Measured by injection **G**: writing one further slot in the literal turns `tree.rs`'s case red, so *naming only the fields it means* is asserted and not merely intended |
| EX-3 | `src/wire.rs:51-56`, `:27` | read: `Command::Edit { view, option, field, reported: Reported }`. The `Eq` half was already true on entry — PHASE-02's compile forced it — and was verified, not redone |
| EX-4 | `src/install.rs:109-114` (`reported`), `:42-54` (the closure) | read: one `match` on `edit.kind`, one slot read, no parse, no fallback, no refusal. The closure's own body is a `let … else` and a `send`. Measured by injection **A** |
| EX-5 | `src/controller.rs:278-303` (`edit`; the `interpret` call at `:299`) | read: the report is interpreted on the walk `declared` already made, and `None` takes the **existing** `Refused::UnknownField` — no variant added to `diagnostics.rs`'s enum (`git diff` touches no file under `src/` but `wire.rs`, `install.rs` and `controller.rs`). Measured by injections **C** and **D** |
| EX-6 | `git diff -- crates/goad/tests/renderer/wiring.rs` | the twelve, rewritten. See VA-1 |
| VT-1 | `wiring.rs::an_edit_is_refused_by_each_selector_that_fails_and_records_nothing` | still five refusals through a `Reported`, each naming which selector failed, and the answer still carries `false` for all of them. Injection **E**, red |
| VT-2 | `wiring.rs:1319::a_report_that_is_not_the_drawn_fields_kind_is_refused_and_records_nothing` | new. Injections **C** and **D**, red — it catches the mismatch being *recorded* and the mismatch being *accepted in silence*, which are the two ways EX-5 can be got wrong |
| VT-3 | `tests/renderer/fields.rs`'s four `install`-driven cases, unchanged | **no edit to `fields.rs` was needed and none was made.** `click` (`fields.rs:234-245`) already drives `invoke_accessible_default_action` on the `CheckBox` through `install`'s closure, and four cases answer afterwards. That they are not proxies for the new channel is injection **A**: breaking `reported`'s `boolean` arm turns exactly those four red |
| VA-1 | `git diff -U3 -- crates/goad/tests/renderer/wiring.rs`, read hunk by hunk | **no case was deleted and no assertion was lost.** Every one of the twelve hunks is a one-token substitution — `Edited::Checked(true)` → `&Reported::Checked(true)` at ten `edit(…)` call sites, `value:` → `reported:` at the two `Command::Edit` literals. No `Err(…)`, no `.expect(…)`, no message string, no surrounding `assert_eq!` and no part of the `answer.values` walk appears in the diff at all. The case count rose by one and fell by none: 189 → 190 |

**A thirteenth site, and the Surfaces were amended for it.** `tree.rs:432`
`activating_a_field_control_fires_edited_with_all_four_selectors` binds
`window.on_edited` directly, and `tree.rs:33`'s `EditedArgs` alias holds what it
captures. The fourth element is now a `FieldEdit` compared **whole** rather
than a `bool`, which is what makes the case hold EX-2's *naming only the fields
it means*; the three selectors it was already asserting are untouched. The user
amended PHASE-03's Surfaces to name the file, and PHASE-08's and PHASE-09's to
name `install.rs` for the same reason a phase later.

**The injection passes** (`design.md` §9), each applied, run, restored, and the
restore confirmed by comparing the file back to the string that was read —
`git diff` is the wrong instrument here, because it compares against `HEAD` and
the phase is in flight. The runner is `scratchpad/inject.py`.

| | the defect planted | what went red |
|---|---|---|
| A | `reported()` answers `None` for `Kind::Boolean` — the edit channel is dead | **4** `fields.rs` cases: the two AC-1/AC-2 wire cases, the shared-id case, and the undrawn-field case. Nothing in `wiring.rs`, which drives `Controller::edit` directly and never through the closure — which is the honest division of labour between the two files |
| B | the `CheckBox` reports someone else's kind (`Kind.text`) | **5**: the same four, through `interpret`'s third `None` case, **and** `tree.rs`'s callback case at the boundary the report is raised from. The discriminant travels and is checked, end to end |
| C | a `None` from `interpret` is recorded as-drawn instead of refused | VT-2 alone |
| D | a `None` from `interpret` is accepted in silence — nothing recorded, nothing reported | VT-2 alone. Note it is the **same** case for C and D: the accepted edit above the mismatch is what makes it catch both |
| E | the walk admits any field id — the membership refusal is gone | VT-1, and the `serve`-driven refusal case. The refusal PHASE-03 widened is still the refusal PHASE-01 left |
| G | the `CheckBox`'s literal writes a slot it does not mean (`index: 1`) | `tree.rs`'s case alone — the whole-report comparison is what asserts EX-2's third clause |

**Decisions taken during execution**

- **The mapper answers `Option<Reported>`, and the four kinds no control draws
  are one grouped arm.** Argued in the Findings above; the short of it is that
  `Reported::Picked` needs slots `FieldEdit` does not carry until PHASE-07, so
  no total map exists to write. `install.rs:92-108` carries the argument at the
  function, names the phase that owns each remaining arm, and says why the
  `None` is *unreachable* rather than *declined* — `undrawn_form` draws
  `boolean` alone. The shape is `glass.rs::field_value`'s, pointing the other
  way, and the doc says so.
- **The `CheckBox`'s literal names `Kind.boolean`, not `field.kind`.** They are
  equal today and will stay equal, so this is not about correctness of the
  value — it is about what can be measured. `field.kind` makes the report agree
  with the row **by construction**, and `interpret`'s third `None` case could
  then never fire from production markup at all. Injection **B** is the case
  that exists because of this choice, and it goes red in five places.
- **`Command::Edit`'s field is `reported`, not `value`.** The plan does not
  name it; `design.md` §5.2's whole argument is that what a widget reported is
  **not** yet what the draft holds, and calling the payload `value` re-welds
  exactly what this phase separates. `plan.md` PHASE-05/EX-6 spells
  `PendingEdit`'s own field `value`; that struct is PHASE-05's to name.
- **`Controller::edit` takes `&Reported` rather than a `Reported`.**
  `interpret` wants a reference, `dispatch` has the value in hand, and the
  three selectors beside it are already `&str`. No clone at any call site.
- **`tree.rs` compares the whole `FieldEdit`, not its `checked` slot.** A
  slot-by-slot read would have been the smaller edit and would have measured
  strictly less: the whole-value comparison is what holds the literal to naming
  only the fields it means, and injection **G** is what says so.

**What the numeric ambiguity would cost this phase's work** — asked for by the
team lead, because the mapper is the code in hand. Not a recommendation: three
ways out were named, and this is only which of them this phase's work already
fits.

Each of the three lands on the **same site** — `install.rs`'s grouped `None`
arm, whose doc already names PHASE-08 as its owner. None of them reopens this
phase's structure; they differ in whose *committed* work they disturb.

| the way out | what of PHASE-03's would change | what it lands on instead |
|---|---|---|
| **A second `FieldEdit` field** — say a `slider: bool` the `number` arm reads beside a slot | **Nothing.** The change is entirely inside the arm this phase did not write, and `tree.rs`'s whole-report comparison survives it untouched: a new field defaults, and `..FieldEdit::default()` still spells the expectation. Reading two fields to select a slot is what `kind` already does, so EX-4 is unthreatened | `app.slint`'s `FieldEdit` declaration, and PHASE-08's own arm |
| **The host re-derives the control** — one `Reported` variant carrying both slots, split inside `interpret` from `slider_bounds` | **Nothing of the mapper's shape**; it would still have one `number` arm, and `Command`'s missing `Eq` is unaffected either way, because the payload is a float on both readings | **PHASE-02's committed work**: `draft.rs::Reported` loses a variant, `view_model::interpret` loses an arm, the twenty-four-pair mismatch unit's count changes, and `design.md` §5.2's `Reported` block is rewritten. Note also that `slider_bounds` is PHASE-08/EX-3's **only** site that chooses a control, so this either calls it twice per edit or moves the decision onto `DrawnKind` — PHASE-02's again |
| **A sixth discriminant value** — `FieldEdit.kind` stops being `Kind` and becomes a report discriminant of its own | **Four edits, every one a token.** `app.slint:47` (the field's type), `app.slint:364` (the literal's `Kind.boolean` → its report spelling), `install.rs:110-112` (the match's scrutinee type and its arm names; the shape — one arm per report plus a grouped rest — is unchanged), and `tree.rs`'s expected literal. **This phase's `Kind.boolean` decision makes it cheaper rather than dearer**: the literal already names what the control *is* rather than what the row *says*, and a report discriminant is that same idea one step on | `design-log.md` D-12's weld, which is the decision being revisited |

Two things fall out that are worth having before the question is put. The
cheapest by this phase's measure is the second `FieldEdit` field and the
dearest is the host re-deriving — and the dearest is dear against **PHASE-02**,
not against PHASE-03, so its price is re-opening a phase that is committed,
reviewed and unit-covered. And `tree.rs`'s whole-report comparison holds under
all three, which is a second reason it was worth taking over a slot-by-slot
read.

**Findings**

- **`Refused::UnknownField`'s doc was narrower than the truth, and it is
  amended here.** Raised rather than edited, because `diagnostics.rs` was in no
  phase's Surfaces before PHASE-09; the user amended PHASE-03's Surfaces to
  take it, doc comment only (`plan-log.md`, 2026-09-19 — *a doc the phase makes
  stale is amended in that phase*, the same call `design.md:1042` and
  PHASE-04/EX-5 already make). **Two clauses were stale, not the one that was
  raised.** The first said the refusal is *"a field that option does not
  declare"*; the fourth said *"Only reachable from a stale or malformed
  callback"*, which after EX-5 is worse — a well-formed, current callback
  reporting a non-finite slider value earns this refusal. Both rewritten at
  `diagnostics.rs:55-67`. No behaviour: the variant and the rendered line are
  untouched, and the line reads correctly for the new path.
- **`mod editing`'s existing `VT-n` doc comments are slice 008's, and they
  collide with this plan's.** The cases either side of the new one are headed
  *VT-1*, *VT-2*, *VT-3*, *VT-5*, *VT-4*, none of them this slice's. The new
  case is headed **PHASE-03/VT-2** and says why in its own doc. This is the
  same hazard `notes.md` §*Traps worth naming* records for `Dn` / `D-n` / `P-n`,
  one sequence further on: **four** id sequences now, not three, and the fourth
  lives in test doc comments rather than in a document.
- **`wiring.rs` never exercises `install.rs`'s closure, and `fields.rs` never
  exercises `Controller::edit` directly.** Injection **A** turned four
  `fields.rs` cases red and nothing in `wiring.rs`; injections **C**, **D** and
  **E** turned `wiring.rs` cases red and nothing in `fields.rs`. That is the
  division the two files were built for, and it is worth knowing for the phases
  that add a control: a new arm in `reported` is measured by a `fields.rs` case
  and by nothing else.
- **The plan's `wiring.rs` citations are one line off, consistently and
  harmlessly.** `design.md` §5.1's table and `plan.md` cite `:1157`, `:1245`,
  `:1304` and `:1340`. Re-derived with `grep -n`: `TWO_FORMS` is at `:1158`
  (`:1157` is the last line of its doc), and the other three are the
  `#[tokio::test]` attribute line of the case each one names. Not bad
  citations — they point at the right case — but a reader jumping to the line
  lands one row above the name. Recorded so the next phase does not re-derive
  it. The line numbers have since moved anyway: the new case at `:1319` sits
  above three of them.

### PHASE-04 — the instant, the `jiff` feature, and `clock.rs`'s doc

**Objective:** the host can turn a picked date and time into an instant and an
offset in the person's own zone, and the manifest change that makes the zone
readable is landed with the argument `POL-001` requires. Discharges no AC on
its own: nothing it builds is drawn or wired this phase. PHASE-07 is its only
consumer.

**Entry criteria, verified rather than assumed**

- **EN-1 — discharged, and it is unusual.** The criterion is *"this plan is
  accepted. **No other phase is a prerequisite**"*. Both halves checked:
  `plan-log.md` records the acceptance (2026-09-19, against `plan.md` as
  written at `44fbd8e`), and `plan.md:103-112` §*Parallelism* says PHASE-04 is
  the only phase that can run beside another, because nothing else in the slice
  touches `instant.rs` or `clock.rs` and its two shared files are one-line
  additions in disjoint regions. Verified against the tree rather than read:
  `grep -rn "instant::|mod instant|instant\.rs" crates/` returns **one** hit and
  it is a doc comment — `install.rs:112`, written by PHASE-03 — so no
  production line depends on this module yet.
- **EN-2 — discharged by the fact rather than by the action.** The criterion is
  conditional: *the phase runs in its own worktree **if** it is running beside
  another phase*. It is not. PHASE-01, PHASE-02 and PHASE-03 are all `done`
  (§Status), PHASE-05 has not begun, and this agent is running alone on `main`.
  So no worktree, and the second half stands: this phase lands as its own
  commit.
  **One thing worth recording.** The tree was not clean when this phase opened:
  `crates/goad/src/install.rs` carried an uncommitted doc-comment hunk, which
  landed under this agent as `52cb96f` (*009: PHASE-03's record*) while the
  reading was in progress. It is PHASE-03's record, not this phase's work, and
  it touches no file in these Surfaces. Noted because *one writer per worktree*
  (`docs/memory/one-writer-per-worktree.md`) was momentarily not true.

**Reading list**

*What is being changed*

- `crates/goad/Cargo.toml:20` — `jiff = { workspace = true }`, the line that
  gains the two features (EX-2). `Cargo.toml:36` — `[workspace.dependencies]`'s
  `jiff = { version = "0.2", default-features = false }`, which does **not**
  change. `crates/goad/Cargo.toml:10-16` — the comment already explaining why
  this member names `jiff` directly.
- `crates/goad/src/lib.rs` — one `pub mod instant;` line, alphabetical between
  `glass` and `install`.
- `crates/goad-shell/src/clock.rs:47-53` — the doc comment EX-5 amends.
  Re-derived here with `grep -n` rather than taken from the plan: `:45` is
  `impl std::error::Error for ClockError {}`, `:46` is blank, the comment runs
  `:47-53` and ends at the `///` carrying *"The manifest test cannot see a
  feature, only a name"*. **The `D25` it cites is slice 005's**, not this
  design's §7 D25 — do not repoint it. No code change in that file.

*Design sections that bind*

- `design.md:1534-1660` — §10 **entire**. It is the argument this phase lands,
  not background: `:1554-1660` is *The `jiff` feature, and why it is here rather
  than in `canon-delta.md`*, and `:1608-1625` is the three-reaches paragraph
  EX-5 turns into `clock.rs`'s comment.
- `design.md:850-908` — `instant.rs`: the three signatures (`:854`, `:856`,
  `:859`), the *checked at every step* paragraph, the four fallible steps, the
  `Compatible` disambiguation, and why the module exists at all.
- `design.md:258-266` — the three read sites, two kinds: the clock in
  `today_local`, and the system zone in **both** `compose` and `today_local`.
- `design.md:1313` A-4 — the four fallible steps are the whole failure surface,
  *not measured*; `:1323` — a fold or a gap **succeeds**.
- `plan.md:438-520` — PHASE-04 in full.
- `plan.md:736-836` — PHASE-07, the consumer. Read for what this phase must
  hand it, and for what it must **not** do: EX-3 there is where `FieldValue`
  gains `date` and `time`.

*Canon*

- `docs/policy/001-the-phase-gate.md:110-143` §Verification — the counting rule,
  the four ADR-001 instruments, and **the residue** at `:139-143`: a feature
  switched on in a shared dependency by stratum 2 or 3 unifies into stratum 1's
  build under `--workspace`, no command in the gate rejects it, and adding one
  *"is therefore a design decision, and is argued in the slice that takes it"*.
  The row at `:128` is the one this phase can break: `cargo test -p goad-semantics`
  builds stratum 1 with **its own** feature set, and it *rejects nothing*.
- `slice-009.md:119-139` §*Governing canon* — `POL-001` named as the canon this
  slice's residue answers to; `:49` carries `clock.rs` in §Scope for one
  doc-comment amendment and no code change.

*The conversation, and the measurements*

- `design-log.md:729-750` **D-35** — the user's *"Amend it in this slice"*, and
  the three reaches. Append-only, and it still carries the bad range
  `clock.rs:46-52`; `design.md` §10 is the corrected copy.
- `canon-delta.md:14-54` **CD-1** — the open question §10's *what the feature
  does not gate* paragraph bears on. Nothing this phase does settles it; what
  P-4 already did is make it settleable against a green test.
- `prototype-handback.md:137-148` **P-4**, measured — under today's featureless
  `jiff`, `Offset`, `Offset::UTC`, `Offset::constant`, `Timestamp::UNIX_EPOCH`
  and `display_with_offset` all compile and run. So the feature gates exactly
  `compose` and `today_local`, and nothing already committed depends on it.
- `notes.md` §*Traps worth naming*, last entry — *a feature enabled by stratum 3
  does not reach a build that excludes stratum 3* (F-54, D-35). That is the
  second of EX-5's three reaches.
- `notes.md` §*Citations known bad* — six, and **F-54's is this phase's**:
  `clock.rs:46-52` appears in the ledger and in `design-log.md` D-35, both
  append-only. Re-derived above rather than trusted.

*Prior art*

- `crates/goad/src/draft.rs:16-17` — `Timestamp` here is
  `goad_semantics::protocol::canonical::Timestamp`, a newtype over
  `jiff::Timestamp` (`canonical.rs:103-113`), and `Offset` is `jiff::tz::Offset`
  unwrapped. `Edited::Picked { instant: Timestamp, offset: Offset }` fixes what
  `compose` must return.
- `crates/goad/src/generated.rs` — the `include_modules!()` quarantine. Any
  Slint-generated type this phase names comes through here.

**Four id sequences collide, not three.** `design.md` §7's `Dn`,
`design-log.md`'s `D-n`, `prototype-notes.md`'s `P-n`, and slice 008's `VT-n`
doc comments already in `wiring.rs`. Cite the file with the id, and
phase-qualify any new `VT-n` written into a test doc.

**Assumptions & STOP conditions**

- **A-a — `compose`'s and `decompose`'s `Date` and `Time` are Slint's, not
  jiff's.** `design.md:862-864` settles it in as many words: *"Slint's `Date`
  and `Time` carry `int` fields, which is `i32`, while jiff wants `(i16, i8, i8)`
  … so the conversion is `i16::try_from` / `i8::try_from`"*. EX-4 says the same
  from the other side — `compose` **uses** jiff's `Date::new` and `Time::new`,
  so its inputs cannot already be jiff civil values. PHASE-07/EX-3 confirms the
  return: `FieldValue` gains `date: Date` and `time: Time` *written from*
  `instant::decompose`.
- **A-b — `TimeZone::system()` cannot fail; what it falls back to is the whole
  question.** `jiff-0.2.35/src/tz/timezone.rs:325-337`: `try_system` or
  `TimeZone::unknown()`. Read, not recalled.
- **A-c — a fold and a gap both succeed.** `design.md:892-898` cites
  `src/civil/datetime.rs:1327-1336` and `src/tz/ambiguous.rs:33-49`. **This is
  a behaviour claim about a dependency and VT-3 measures it rather than
  inheriting it.**

STOP and consult — do not improvise past any of these:

- **S-1.** A file the Surfaces line does not name. Four Surfaces lines have
  already been found short this slice, one cause (`plan-log.md`, 2026-09-19,
  *the class behind all four*). **This one is short too — see the Findings
  below, raised rather than edited.**
- **S-2.** Anything beyond a feature flag on an existing dependency. A new
  dependency is a STOP even if it is only a dev-dependency.
- **S-3.** `civil::date` or `Date::at` anywhere, or an `as` cast in place of a
  `try_from`. Both panic paths; EX-4 is the criterion.
- **S-4.** A fifth fallible step discovered in `compose` that A-4 does not name
  — A-4 is explicitly *not measured*, so finding one is a design finding, not a
  repair.
- **S-5.** Repointing `clock.rs`'s `D25`, or making any code change in that
  file.
- **S-6.** Putting the feature on `[workspace.dependencies]`. §10's *what is
  deliberately not done*.

**Findings raised while expanding the phase**

- **The Surfaces line is short by `crates/goad/ui/app.slint`, and it is a
  fifth instance of the class `plan-log.md` already named.** `Date` and `Time`
  are not builtin Slint types: they are declared in the widget library
  (`i-slint-compiler-1.17.1/widgets/common/datepicker_base.slint:7-11` and
  `widgets/common/time-picker-base.slint:332-336`) and re-exported by
  `widgets/fluent/std-widgets.slint:24-25`. `app.slint` imports four names from
  `std-widgets.slint` and neither is among them, so **neither struct is
  generated into `crate::generated` today**. Verified by instrument, not by
  reading: `grep -n "pub struct r#Date|pub struct r#Time"` over the build
  script's `out/app.rs` at `ddd686e` returns nothing, while the same grep finds
  `r#FieldEdit`, `r#FieldValue` and the rest.

  **Measured, three ways, each built and each restored** (the file was copied
  to the scratchpad first and compared back by `sha256sum` afterwards —
  `7980903c…a42eac` before and after, `git status` clean):

  | what was put in `app.slint` | `Date` / `Time` in `out/app.rs` |
  |---|---|
  | nothing — the tree as it stands | **no** |
  | `import { Date, Time } from "std-widgets.slint";` | **no** — an import alone is not an export |
  | `import { … }` *and* `export { Date, Time }` | **yes**, both |
  | `export { Date, Time } from "std-widgets.slint";` — one line | **yes**, both |

  The emitted shapes are `pub struct r#Date { day: i32, month: i32, year: i32 }`
  and `pub struct r#Time { hour: i32, minute: i32, second: i32 }`, which is
  exactly what `design.md:862-864` predicts of them.

  So EX-3 and EX-4 — and VT-1, VT-2 and VT-3, which construct the inputs —
  cannot be written at all without one line in a file this phase may not touch.
  **STOP taken rather than the edit**, per `docs/AGENTS.md` §Execute and the
  precedent of PHASE-02's and PHASE-03's four.

  **Why it is the same class and not a new one.** `plan-log.md` (2026-09-19,
  *the class behind all four*) found that the Surfaces lines were derived from
  `design.md` §9's enumeration, which enumerates **constructors**, and that *a
  file that only has to change because a type above it changed* is not one.
  This is that, one level further out: `app.slint` has to change because a type
  must become **visible**, not because anything in it is constructed or drawn.
  The design is not wrong — `design.md:862-864` names the Slint types plainly.
  The plan's Surfaces line is.

  **What it would cost, and what it would not.** One line,
  `export { Date, Time } from "std-widgets.slint";`. It declares no control,
  reads no slot and adds no field to `FieldValue` or `FieldEdit`, so
  `app.slint:20-26`'s standing rule — *nothing is declared before a control
  reads it* — is untouched: the line makes two library types nameable from
  Rust, which is not a declaration of anything the markup draws. It is also
  **transitional**: at PHASE-07/EX-3 `FieldValue` gains `date: Date` and
  `time: Time`, and a struct reached from an exported struct is generated
  without an explicit export — so PHASE-07 should delete this line as it adds
  those fields, and that is worth writing into PHASE-07 rather than leaving to
  be noticed.

  **The alternative, priced and not recommended.** Move `instant.rs` wholesale
  into PHASE-07. That costs PHASE-04 its independence — it is the one phase the
  plan marks as able to run beside another — and it loads the slice's widest
  markup phase with three fallible functions and their nine or so units. It
  also buys nothing: PHASE-07 would write the same line one phase later.

**Tasks**

- [x] T-0 expand the phase sheet; re-derive every citation; verify EN-1 and
      EN-2 against the tree
- [x] T-1 `app.slint`'s one-line export of `Date` and `Time`. Was blocked;
      the user amended the Surfaces for it (`plan.md`, `plan-log.md`,
      2026-09-19)
- [x] T-2 `crates/goad/Cargo.toml`: the two features (EX-2), with
      `cargo test -p goad-semantics` run deliberately either side of it
- [x] T-3 `clock.rs:47-53`: the three reaches, doc comment only (EX-5)
- [x] T-4 `src/instant.rs` + `lib.rs`: `compose`, `decompose`, `today_local`
      (EX-3, EX-4)
- [x] T-5 VT-1, VT-2, VT-3, and the injection pass that says they are not
      proxies
- [x] T-6 VT-4, VA-1, VA-2; `just check` exits 0 (EX-1); sheet, §Status and
      §Harvest updated

**What landed while T-1 was blocked**

Both halves that do not depend on the answer, and the gate is green with them:
`just check` **exit 0** at this point, every count unchanged from PHASE-03's
564.

| | discharged by | how it was checked |
|---|---|---|
| EX-2 | `crates/goad/Cargo.toml:38` | `jiff = { workspace = true, features = ["tz-system", "tzdb-zoneinfo"] }`. `[workspace.dependencies]`'s line is untouched at `Cargo.toml:36` — still `{ version = "0.2", default-features = false }` — and `goad-semantics`, `goad-shell` and `goad-emit` still take it bare (`crates/goad-semantics/Cargo.toml:17`, `crates/goad-shell/Cargo.toml:14`, and `goad-emit` names no `jiff` at all). `:17-34` carries the argument at the manifest and points at `design.md` §10 rather than restating it |
| EX-5 | `crates/goad-shell/src/clock.rs:47-72` | the three reaches, in the order §10 gives them. **No code change**: `git diff -- crates/goad-shell/src/clock.rs`, filtered to lines that are not `///`, is empty. The `D25` is slice 005's and was left pointing where it pointed |
| VA-1 | `design.md:1534-1660` re-read against the manifest as landed | §10 still says what the manifest now does, clause by clause: the feature list (`tz-system` + `tzdb-zoneinfo`), the member it goes on, the two it does not, and the three reaches. Nothing in §10 needed a correction, and this phase wrote none — it is the argument, and the manifest cites it |

**The residue, measured rather than asserted.** `POL-001` §Verification's claim
is that no gate command rejects this. It can be shown positively with
`cargo tree -e features`, which is not a gate command and rejects nothing
either — it just prints what each build resolves. Instrument:
`cargo tree -p <member> -e features | grep -oE 'jiff feature "[a-z-]+"'`.

| build | jiff features, before | after |
|---|---|---|
| `--workspace` | none | `alloc`, `std`, `tz-system`, `tzdb-zoneinfo` |
| `-p goad-semantics` | none | **none** |
| `-p goad-shell` | none | **none** |
| `-p goad-emit` | none | **none** |

That is §10's three reaches as a table, and the second row is the one that
matters twice over: it is why `cargo test -p goad-semantics` **holds** (stratum
1 builds and passes with its own feature set, so the three instruments beside
it check a configuration that stands alone) and why it **rejects nothing** here
(it never links the `jiff` the workspace build links, so there is nothing for
it to notice). The command's result is identical either side of the manifest
change — 30 + 5 + 0 passing, exit 0, and it did not so much as rebuild `jiff`.
A future stratum 1 source depending on a capability this feature switches on
would keep the workspace build green and would not turn this command red. §8 R8
carries that as a standing risk with review named as its only mitigation.

**What it costs the graph: nothing new.** `Cargo.lock` gains four names under
`jiff` — `defmt`, `log`, `serde_core`, `windows-link` — and **no
`[[package]]` stanza**: all four were already in the lock at `ddd686e`, and
the diff is four insertions in jiff's own dependency list. They are optional
edges the resolver now records, not crates that build: `cargo tree --workspace`
still shows `jiff v0.2.35 └── jiff-core v0.1.0` and nothing else beneath it,
and `windows-link` is target-gated to Windows in any case. So §10's *what it
costs* — `std` pulling `alloc` into stratum 1's workspace build — is the whole
of the cost, and there is no second, unpriced one.

**The dependency claims VT-2, VT-3 and VA-2 rest on, measured while T-1 was
blocked.** All three are claims about `jiff`, not about the Slint types, so
none of them had to wait. Probe: a standalone cargo project in the scratchpad
taking `jiff = { version = "0.2", default-features = false, features = ["tz-system", "tzdb-zoneinfo"] }`
— the same resolution `crates/goad` now gets — so these are measurements of the
configuration this phase lands and not of some other one.

*VT-3 — the fold and the gap both succeed, and they behave as the plan
predicted.* Measured in `America/New_York`, under jiff's default
`Disambiguation::Compatible`:

| | civil in | resolved to | instant |
|---|---|---|---|
| **gap** | `2024-03-10T02:30` — does not exist, 02:00 → 03:00 | `2024-03-10T03:30:00-04:00` | `2024-03-10T07:30:00Z` |
| **fold** | `2024-11-03T01:30` — happens twice, 02:00 → 01:00 | `2024-11-03T01:30:00-04:00` | `2024-11-03T05:30:00Z` |

The gap shifts **forward**, which is `design.md:892-893` verbatim. The fold
takes the **earlier** occurrence, and the offset is what says so: `-04:00` is
EDT, before the fall back to `-05:00` EST, and the two candidate instants are
`05:30:00Z` and `06:30:00Z` — it took the former. Neither refused.

*VT-2 — the four fallible steps all have witnesses, and the fourth's boundary
is not where reading `DateTime::MAX` would put it.*

- steps 1 and 2, the integer conversions: `i16::try_from`/`i8::try_from` of any
  Slint `int` outside the field's width. No jiff involved.
- step 3, the checked constructors: `Date::new(2024, 2, 30)` →
  `Err(parameter 'day' for 2024-02 is invalid, must be in range 1..=29)`;
  `Date::new(2024, 13, 1)` → `Err(… 'month' … 1..=12)`;
  `Time::new(24, 0, 0, 0)` → `Err(… 'hour' … 0..=23)`. All `Result`, no panic.
- step 4, `DateTime::to_zoned`: **`Timestamp::MAX` is
  `9999-12-30T22:00:00.999999999Z`, a whole day below `DateTime::MAX`
  (`9999-12-31T23:59:59.999999999`)**, so the top of the civil range does not
  fit the timestamp range in any zone. `9999-12-31T23:59:59` was refused in all
  five zones tried, the widest being `Pacific/Kiritimati` at `+14`. That makes
  it a **zone-independent** witness, which is what VT-2 needs, because `compose`
  can only ever resolve in the system zone.

  And `design.md:880-883`'s reason for the fourth step — *whether a civil
  datetime fits the timestamp range depends on the offset it is resolved in* —
  now has a witness of its own rather than an argument: `9999-12-31T12:00:00`
  **succeeds** at `Pacific/Kiritimati` (`→ 9999-12-30T22:00:00Z`, the range's
  last instant) and is **refused** at `Australia/Melbourne`, `UTC`,
  `America/New_York` and `Pacific/Midway`. One civil value, two answers, and
  the offset is the only difference.

*VA-2 — assertable here, but the number is not what should be asserted.*
`TimeZone::system()` on this machine answers `iana_name = Some("Australia/Melbourne")`,
`is_unknown = false`, offset `+10` (`/etc/localtime -> /etc/zoneinfo/Australia/Melbourne`,
`TZ` unset), so *an offset that is not `+00:00`* is assertable. It should not be
asserted: the number is a property of this machine, and a CI box would answer
something else or the same by coincidence. `TimeZone::is_unknown()`
(`jiff-0.2.35/src/tz/timezone.rs:705`) is the machine-independent form of the
same claim — featureless `jiff` falls back to `TimeZone::unknown()`
(`timezone.rs:325-337`), and a box set to UTC still answers a *known* zone, so
`!TimeZone::system().is_unknown()` separates *the feature is on* from *the
feature is off* on every machine and separates nothing else. The Melbourne
offset is recorded here as what was checked beside it.
**What landed, criterion by criterion**

`just check` **exit 0**. `cargo test --workspace` totals **538**; every target's
count is identical to PHASE-03's except `goad`'s lib, **42 → 51**, which is this
phase's nine units and nothing else.

| | discharged by | how it was checked |
|---|---|---|
| EX-1 | the gate | `just check` exit 0, second run (the first failed `cargo fmt --all --check` alone, on three `Time` literals and one `let`; `cargo fmt --all` and re-run) |
| EX-2 | `crates/goad/Cargo.toml:38` | landed earlier; see *What landed while T-1 was blocked* |
| EX-3 | `src/instant.rs`, `src/lib.rs:12` | `compose`, `decompose` and `today_local` with §5.2's signatures, one departure below. `grep -rn "TimeZone\|wall_clock" crates/goad/src/` names this module and nothing else in the crate for either read, so *the only module in this crate that reads the clock or the system time zone* holds |
| EX-4 | `instant.rs`'s `composed_in` | three `i16::try_from` / `i8::try_from`, three more for the time, then `Date::new`, `Time::new` and `DateTime::to_zoned`. `grep -n "civil::date\|Date::at\| as "` over the file: **nothing**. Measured by injections **B**, **C** and **D** |
| EX-5 | `crates/goad-shell/src/clock.rs:47-72` | landed earlier; doc only |
| VT-1 | `an_ordinary_date_and_time_compose_to_the_instant_the_system_zone_gives`, `decompose_reads_the_instant_at_the_offset_it_is_given_and_nowhere_else` | the first names no number this machine chose: the offset is compared against `TimeZone::system().to_offset(instant)`, which is a different route to it than `compose` takes, and the instant is compared by reading it back. The second pins `decompose` outright — the epoch reads `1970-01-01T00:00` at `+00:00` and `1969-12-31T19:00` at `-05:00`, which is `prototype-handback.md` P-4's measured pair arriving from the other side. Injections **A** and **F** |
| VT-2 | `each_of_composes_four_fallible_steps_answers_none_rather_than_panicking`, `whether_a_civil_datetime_fits_the_timestamp_range_depends_on_its_offset` | one case per step, each naming the step it reaches: a year no `i16` holds, an hour no `i8` holds, `2024-02-30`, and `9999-12-31T23:59:59`. The second case is the fourth step's *reason* rather than its effect. Injections **B**, **C**, **D** |
| VT-3 | `a_civil_time_inside_a_dst_gap_succeeds_by_shifting_forward`, `a_civil_time_inside_a_dst_fold_succeeds_by_taking_the_earlier_occurrence` | both **succeed**, as the plan predicted and as §9's measurement pass had already found. The fold asserts the **instant** as well as the offset, because `05:30:00Z` and `06:30:00Z` are the same clock face and only the instant separates them. Injection **E** |
| VT-4 | the gate | `goad-shell` lib **71** and `tests/integration` **96**, both unchanged. `wall_clock`'s own cases are `controller.rs:986`, `goad-emit/src/main.rs:299` and the two event-loop targets, all green. `cargo test -p goad-semantics` run deliberately, before and after the manifest change and again at the end: 30 + 5 + 0, exit 0, identical every time |
| VA-1 | `design.md:1534-1660` | discharged earlier and re-read at the close: §10 still says what the manifest does |
| VA-2 | `the_system_time_zone_is_read_rather_than_fallen_back_from` | **assertable here, and asserted as a predicate rather than a number.** See below |

**The one departure from §5.2's signatures, and why.** `compose` takes
`&Date, &Time` rather than `Date, Time`. Every field read out of them is an
`i32` and neither is consumed, so by-value trips `clippy::needless_pass_by_value`
at `deny`; the caller — `install.rs`'s mapper — holds a `&FieldEdit` and would
have to clone to satisfy the other spelling. Exactly PHASE-03's reasoning for
`Controller::edit(&Reported)`, and it changes nothing about what the function
means. `decompose` and `today_local` are as written.

**Two private helpers the design does not name, and both exist for the tests to
be able to say something true.**

- `composed_in(zone, date, time)` — `compose` with the zone named rather than
  read. `compose` is this applied to `TimeZone::system()`. Without it, VT-3
  would have to drive DST through whatever zone the machine is in, and a CI box
  set to UTC has no DST at all: the case would pass by being vacuous.
- `local_midnight(zone, now)` — the pure half of `today_local`, for the same
  reason. **This one was added on the evidence of a green injection**, not up
  front; see below.

`EX-3`'s *only module that reads the clock or the system zone* is unaffected by
either: both are private, both take what they need, and `compose` and
`today_local` remain the two functions that read.

**The injection pass.** Ten planted defects, each applied, built, run,
restored, and the restore confirmed by comparing the file back to the string
that was read (`git diff` is the wrong instrument mid-phase — it compares
against `HEAD`). Runner: `scratchpad/inject.py`. **Three of the ten did not
compile on the first attempt and were repaired before being counted** — an
uncompiled control greps the same as a passing one
(`docs/memory/negative-control-must-compile.md`).

| | the defect planted | what went red |
|---|---|---|
| A | `compose` ignores the system zone and resolves at UTC | VT-1's first case |
| B | step 1's conversion falls back instead of refusing | VT-2's first case |
| C | step 2's conversion falls back instead of refusing | VT-2's second case |
| D | step 4 is an `expect` rather than a refusal | VT-2's both cases, by panicking |
| E | the fold takes the **later** occurrence | VT-3's fold case — and only that one, which is what says the fold assertion is about disambiguation and not about arithmetic |
| F | `split` swaps month and day | **6** cases, every one that reads a civil value back |
| H | **the manifest's two `jiff` features are gone** | **5** cases, VA-2's among them |
| I | `today_local` resolves at UTC rather than the system zone | **GREEN — nothing caught it.** The one residue; see below |
| I2 | `local_midnight` ignores the zone it is handed | the two-zone case |
| J | the midnight the pickers open on is not midnight | the two-zone case and the `today_local` case |
| K | `today_local` hands `local_midnight` the epoch rather than the clock | the `today_local` case |

**H is the evidence the residue argument could not produce.** `POL-001` says no
gate command rejects the feature, and that is true — but with the feature
removed, **five units go red**, VA-2's among them, and the other four because
`TimeZone::get` cannot resolve a name without `tzdb-zoneinfo`. So the feature is
load-bearing in a way a reader can check rather than take on the argument's
word. It remains true that nothing *in the gate* rejects **adding** it; what H
shows is the other direction — that removing it is not silent, once this phase's
units exist. That is narrower than enforcement and worth exactly that much.

**I is the residue, and it is recorded rather than papered over.**
`today_local` passing `TimeZone::UTC` where it passes `TimeZone::system()` is a
one-token defect no case catches **today**. It is not caught because Melbourne
(`+10:00`) and UTC are on the same date at the hour this ran; between 00:00 and
10:00 local the same injection would go red. A test cannot make them disagree:
`std::env::set_var` is in `clippy.toml`'s `disallowed-methods` — *"Global
process mutation is test-hostile"* — so the zone the machine is in is not
something a unit may change.

**What was done about it, which is not nothing.** The first draft of
`today_local` was a single opaque body and injection **I** was green against it.
The body was split into `local_midnight(zone, now)` and a one-expression
`today_local`, and a new case — `one_instant_is_two_different_local_dates_in_two_different_zones`
— now asserts the **class**: `2024-06-15T06:00:00Z` is the 15th at `+14:00` and
the 14th at `-11:00`, so a `today_local` that resolved at any single fixed
offset is wrong for somebody. Injections **I2**, **J** and **K** are all red
against it. What is left uncaught is one token: that the zone `today_local`
reads is the **system's**. That is the same shape as VA-2's residue and is held
by review, which is what `design.md` §8 R8 already names as the mitigation of
last resort.

**VA-2, and why it is a predicate.** The system zone here is
`Australia/Melbourne` (`/etc/localtime -> /etc/zoneinfo/Australia/Melbourne`,
`TZ` unset), offset `+10:00`, so *an offset that is not `+00:00`* **was**
assertable — it was checked by hand and is recorded in the case's doc. It is not
what the case asserts. `!TimeZone::system().is_unknown()` tests precisely what
VA-2 exists to test — featureless `jiff` falls back to `TimeZone::unknown()`
(`jiff-0.2.35/src/tz/timezone.rs:325-337`), and `is_unknown` is true exactly
then — while an offset would assert a property of whichever machine ran it, and
a CI box set to UTC would make the literal criterion unassertable while leaving
the predicate perfectly sharp. Injection **H** confirms the predicate is live.

**Decisions taken during execution**

- **`today_local` reads `goad_shell::clock::wall_clock`, not
  `jiff::Timestamp::now()`.** One clock in the host rather than two, and the
  jiff call *panics* on an unrepresentable system clock where `wall_clock`
  refuses — which is the entire reason `clock.rs` exists (005/D-9, and that
  module's own doc). The design does not name a clock API; it names the read.
- **`today_local` absorbs the clock's failure at the epoch, and says so at the
  function.** The signature §5.2 gives is total, and `design.md:1048` is
  explicit that a present has no way to report a failure — that is why
  threading the instant through `Frame` was rejected. So the refusal has to go
  somewhere, and the epoch is the only fixed point available. This is **not**
  the 1970 PHASE-07 is warned off: that warning is about seeding an untouched
  field from `as_drawn` on the ordinary path, and this is a system clock
  reading before 1970 or outside jiff's range. Recorded here because the design
  did not settle it and no signature changes either way.
- **The `app.slint` comment promises a re-measurement, not a deletion.** The
  team lead's instruction, and it is right: that the export becomes redundant
  once `FieldValue` carries `date` and `time` is a prediction about the
  *reachability* path, and what was measured was the *export* path. A comment
  promising a deletion that then does not happen is a stale doc of exactly the
  kind PHASE-03 spent its last hour repairing.

**Findings**

- **`glass.rs:1-3` claims to be *"the only file in the crate that names a
  generated type outside `generated.rs` itself"*, and it is not.**
  `install.rs:15` names `FieldEdit`, `Kind`, `PromptWindow` and `Tray`, and has
  since before this phase; `instant.rs` now names `Date` and `Time` and makes
  it one file more false. **Not repaired**: `glass.rs` is in no Surfaces line
  this phase, the claim was already false on entry, and PHASE-06 and PHASE-07
  both name the file. Raised to the team lead.
- **PHASE-03's sheet records *"564 in all"* and the per-target numbers beside
  it do not sum to it.** Its own baseline list — `goad` lib 42, `tests/renderer`
  189, `goad-boundary` `tests/checks` 43, three loop targets at 1, `goad-shell`
  71 / 96, `goad-semantics` 30 — omits four targets that exist and run
  (`goad-emit` 34, its `tests/binary` 9, `goad-shell`'s `tests/shape` 6,
  `goad-semantics`'s `tests/protocol` 5), and with those it comes to **529**,
  not 563. This phase's total is **538**, measured by summing every
  `test result: ok. N` line `cargo test --workspace` prints, which is 529 + this
  phase's 9. **Every per-target number PHASE-03 recorded is correct**; only the
  sum and the enumeration behind it are. Not edited — another phase's record is
  not this phase's to rewrite — and reported to the team lead.

  **Team lead, 2026-09-19: the sum is not wrong either, and that matters.**
  Measured both ways on a clean tree at `e27d4cd`: `just check` totals **573**
  and `cargo test --workspace` totals **538**, and the difference is exactly 35
  — the gate runs `cargo test -p goad-semantics` as its own command, so
  `goad-semantics`'s 30 + 5 are counted twice. The same arithmetic holds at
  PHASE-03's boundary: 564 gate, 529 workspace, difference 35. So **564 and 529
  are both correct and are different quantities**, and PHASE-03's sheet names
  one denominator over a list enumerating the other. This is
  `docs/memory/verify-the-enumeration-not-the-conclusion.md` in both directions
  — the finding is real and its sub-claim was not. **Quote a count with its
  denominator from here on**: *gate* or *workspace*. It is the same
  cause as §*Citations known bad*'s fifth and sixth: a number arrived at by
  hand rather than from an instrument that prints it.


### PHASE-05 — `text`, and the debounce's delivery

**Objective:** a `text` field draws, a person can type into it continuously, and
everything they typed reaches the draft — on a timer while they keep typing, and
in the command that answers. The largest phase in the slice.

**Entry criteria, verified rather than assumed**

- **EN-1 — discharged.** PHASE-03/EX-1 … EX-6 read off the tree at `f5e20f8`,
  not inherited from the hand-over:

| PHASE-03 | claimed at | checked here |
|---|---|---|
| EX-1 | the gate | the team lead re-ran `just check` at `f20564f`, clean tree: **exit 0**, **573** — a *gate* total. `cargo test --workspace` is **538**; the gate runs `cargo test -p goad-semantics` as a command of its own, so that crate's 30 + 5 are counted twice and the gate total is always exactly 35 above the workspace one. Both denominators are named wherever a count appears below |
| EX-2 | `ui/app.slint:57`, `:122`, `:338-377` | read: `export struct FieldEdit { kind: Kind, checked: bool, text: string, number: float, index: int }` at `:57`; `callback edited(string, string, string, FieldEdit)` at `:122`; the `CheckBox`'s `toggled` literal names `kind` and `checked` only (`:372-377`), and `Kind.boolean` rather than `field.kind` |
| EX-3 | `src/wire.rs:51-56`, `:28` | read: `Command::Edit { view, option, field, reported: Reported }`; `#[derive(Debug, Clone, PartialEq)]` with no `Eq` |
| EX-4 | `src/install.rs:120-125`, `:43-53` | read: `reported(&FieldEdit) -> Option<Reported>`, one `match` on `edit.kind`, one slot read, no parse and no refusal; `Kind::Text` is in the grouped `None` arm at `:123`, which is the arm this phase fills |
| EX-5 | `src/controller.rs:278-301` | read: `edit` interprets on the walk `drawn_fields` already makes (`:290`), `interpret(...).ok_or(Refused::UnknownField)` at `:299`. No class added to `diagnostics.rs`'s enum |
| EX-6 | `tests/renderer/wiring.rs`, `mod editing` at `:1131` | read: every `edit(…)` call site passes `&Reported::…`; the two `Command::Edit` literals at `:1612` and `:1628` carry `reported:` |

  **Baseline, measured at T-0** on a clean tree at `98335cd`: `just check`
  **exit 0**, gate total **573**; `cargo test --workspace` **538**. The
  difference is exactly 35 — `goad-semantics`'s 30 + 5 counted twice, because
  the gate runs `cargo test -p goad-semantics` as a command of its own.
  Per target, from `cargo test --workspace` (the denominator VA-1 compares
  against): `goad` lib **51**, `goad` `tests/renderer` **190**, the three loop
  targets **1** each, `goad-boundary` `tests/checks` **43**, `goad-emit` bin
  **34** / `tests/binary` **9**, `goad-semantics` lib **30** /
  `tests/protocol` **5**, `goad-shell` lib **71** / `tests/integration` **96** /
  `tests/shape` **6**. Every doc-test target is 0.

**Reading list**

Every `path:line` below was re-derived with `grep -n` in this session — the
slice's rule is *cite from an instrument that prints the number*, and
`plan.md`'s own `Pending` citation (`controller.rs:385`) is already one line
family off: it is at **`:408`**.

*What is being changed*

- `crates/goad/src/pending.rs` — **new**. The map, the timer, and nothing else.
- `crates/goad/src/lib.rs:6-17` — the module list, one line.
- `crates/goad/src/wire.rs:29-59` — `Command`; `Choose` at `:35`, `Edit` at
  `:51`. `:140-146` — `send`, whose `Err(TrySendError::Full(_returned))` arm at
  `:143` binds the returned command and drops it deliberately (D8).
- `crates/goad/src/install.rs:24` — `install`'s signature; `:26-31` — `chosen`;
  `:43-53` — the `edited` closure; `:120-125` — `reported`, whose `Kind::Text`
  sits in the grouped arm at `:123`.
- `crates/goad/src/controller.rs:215` — `answer`; `:278` — `edit`; `:343` —
  `selected`, the one statement of identity-before-membership; `:675` —
  `dispatch`'s `Choose` arm; `:203` — `refuse`, the fold of a refusal the
  renderer made itself.
- `crates/goad/src/view_model.rs:263-271` — `undrawn_form`; `:287-324` —
  `sift`, whose `kind: DrawnKind::Boolean` at **`:317`** is the honest constant
  PHASE-01 left and whose own comment at `:311-316` says the phase that draws a
  kind replaces it with a match. `:511-528` — `as_drawn`, whose `Text` arm is
  written; `:569-636` — `interpret`, whose `DrawnKind::Text` arm is written.
- `crates/goad/src/glass.rs:322-339` — `field_value`, whose grouped arm names
  `text` PHASE-05. `:289-296` — `markup_kind`, already total and unchanged.
- `crates/goad/ui/app.slint:1` — the widget import line, which gains `LineEdit`;
  `:336-379` — the field repeater and the `CheckBox`, which becomes one arm of a
  kind test; `:358-364` — the guard's exact spelling.
- `crates/goad/src/main.rs:86` — the capacity-1 channel; `:89-90` — the `Wire`
  and the callback table; `:97` — `SlintGlass::new`, **not** touched here
  (PHASE-06's).
- `crates/goad/Cargo.toml:47-65` — the four `[[test]]` blocks; a fifth is added.

*What breaks by compile, and where each one is*

- `Command::Choose`'s new field — `install.rs:27`, `wiring.rs:1021`, `:1102`,
  and **outside the Surfaces**: `scheduling.rs:269`, `:925`, `ingress.rs:381`.
- `install`'s new parameter — `main.rs:90`, `fields.rs:301`, and **outside the
  Surfaces**: `event_loop/closing.rs:57`,
  `event_loop_schedule/scheduling.rs:84`.

Both are STOP conditions and were reported before any production code was
written; see §*STOP conditions raised* below.

*The fixtures that migrate (EX-9), each re-derived*

- `tests/renderer/fields.rs:81` — `A_DRAWN_AND_AN_UNDRAWN_FIELD` (plan says
  `:71`, which is `THREE_FIELDS`'s doc).
- `tests/renderer/wiring.rs:1158` — `mod editing`'s own `TWO_FORMS` (plan says
  `:1157`, the last line of its doc).
- `tests/renderer/mapper.rs:237`, `:255-256`, `:377`, `:423` — the four the plan
  names, **and `:297`/`:321`**, which it does not: `every_undrawn_kind_is_-
  reported_by_option_field_and_form` enumerates all four undrawn kinds and their
  four reports and goes red the moment `text` draws. Same file, already in the
  Surfaces, so nothing to decide — but the enumeration is short and this sheet
  records it.
- `tests/renderer/reception.rs:762` — the diagnostic case.
- `tests/renderer/table.rs:151-157` carry `"kind":"text"` and are **not** in
  this list, checked rather than assumed: all four are `retained(…)` rows
  asserting a **normalizer** diagnostic (`table.rs:288-334`, rows `P6` at `:289` to `P12` at `:329`), refused before
  `present` is ever called, so drawing changes nothing about them.

*Design sections that bind*

- §5.1 (`design.md:199-226`) — `pending.rs`, keyed not singular, kind-agnostic,
  every entry carrying its view; (`:227-256`) — the two asymmetrical exits and
  the enqueue rule.
- §5.2 (`design.md:933-1018`) — the `wire.rs` block: `PendingEdit`,
  `Choose { view, option, edits }`, `send -> bool`, the two ways a carried edit
  fails and which of them still answers, *reported for the life of the exchange*,
  and *no order is promised*.
- §5.2 (`design.md:500-506`) — the controls table: `text` → `LineEdit`, sends
  `text`, `edited`, debounced. (`:545-551`) — the guard's comparand table: the
  text `LineEdit` compares `self.text` against `text` and converges unless they
  are equal.
- §5.4 (`design.md:1100-1128` *A keystroke*, `:1154-1172` *An answer*,
  `:1174-1188` *A new view*).
- §5.5 **I-H** (`design.md:1310-1317`) — one rule, three sites. PHASE-06 owns
  the *shown* site; this phase owns *sent* and *drained*.
- §9 (`design.md:1525-1543`) — the three obligations with no widget, and the two
  signatures that widen with their four call sites named.
- `plan.md:527-655` — PHASE-05 in full, Notes included.

*Prior art*

- `crates/goad/tests/event_loop_reassert/main.rs:1-28` and `reassert.rs:1-192` —
  the arrangement the new target copies: one `[[test]]`, one `#[test]` fn,
  `init_integration_test_with_system_time()`, a repeated `slint::Timer` stepping
  the loop, a liveness bound that quits rather than wedges, every assertion made
  on the test thread, and `retaining()` building a `Controller` from a hand-made
  `Outcome`.
- `crates/goad/tests/renderer/fields.rs:251-342` — `Rig` / `rigged`, the
  `install` call site this phase widens, and `tick!`'s note on why the wait is
  not optional.
- `crates/goad/src/glass.rs:322-339` — the shape for a host-side map total over
  five kinds while fewer are drawn, naming the phase that owns each arm.
  `install.rs:91-119` is the same shape pointing the other way.

*Memory*

- `slint-testing-backend-initialises-once-per-process.md` — one arrangement, one
  `[[test]]`, one `#[test]` fn.
- `a-negative-control-that-does-not-compile.md` — read the **test count**, not
  the absence of `FAILED`.
- `change-handlers-need-an-event-loop.md` — why the debounce claims cannot live
  in `tests/renderer/`.
- `tests-asserting-proxies.md` — VA-2's whole subject.
- `verify-the-enumeration-not-the-conclusion.md` — EX-9's fixture list is the
  live instance.

**Assumptions & STOP conditions**

Taken on faith, each with what makes it cheap to be wrong about:

- **A-a.** A `slint::Timer` may be restarted from inside its own callback, so
  the re-arm works. Read from the locked source
  (`i-slint-core-1.17.1/timers.rs:348-372`) and measured by the prototype
  (`prototype-handback.md` P-11). **VT-6 is the negative control that keeps it
  honest**, so being wrong is a red test rather than a silent loss.
- **A-b.** `tokio::sync::mpsc::channel`, `try_send` and `try_recv` work with no
  runtime entered. The new loop target has no runtime, and `Wire` is the only
  way `pending.rs` delivers, so the target must hold a channel even though it
  holds no `serve`. Cheap to be wrong about: the target panics on its first
  step, immediately.
- **A-c.** `ElementQuery` / `set_accessible_value` reach a `LineEdit` under
  `init_integration_test_with_system_time()` with the window shown and sized.
  `search_api.rs:637` is the setter; `event_loop_reassert` shows the window
  through `present` already and the renderer tier sizes the window to 600×600
  for the same clipping reason. Wrong ⇒ the query answers `None` at the first
  step.
- **A-d.** Adding a field to `Command::Choose` and a parameter to `install` are
  the only compile breaks outside this phase's own edits. Derived by grepping
  every construction site rather than by expectation; the build says so either
  way, and the two lists are above.
- **A-e.** A `bool` return on `Wire::send` disturbs no existing caller.
  `clippy::must_use_candidate` is `allow` workspace-wide (`Cargo.toml`), so no
  `#[must_use]` is owed and a discarded `bool` in statement position is not a
  lint.

STOP and consult — do not improvise past any of these:

- **S-1.** A criterion compelling a file the Surfaces line does not name. Hit
  three times before a line of production code; see below.
- **S-2.** Any judgement moving into a Slint callback — a parse, a fallback, a
  refusal. The `edited` closure gains a map write and nothing else.
- **S-3.** A new refusal class. A carried edit that fails takes the **existing**
  `SupersededView` or `UnknownField` posture.
- **S-4.** Clearing an entry on anything but an enqueued send. VA-1 is the check
  and the asymmetry is the design's, not a convenience.
- **S-5.** Weakening, deleting or `#[ignore]`-ing an existing case to go green.
  VA-2 exists to catch the softer version of it.
- **S-6.** A dependency addition, or a second event-loop arrangement.

**STOP conditions raised** — reported to the team lead before any production
code, with the criterion id and the file, per `docs/AGENTS.md` §Execute:

1. **EX-6 → `tests/renderer/scheduling.rs:269`, `:925`,
   `tests/renderer/ingress.rs:381`.** `Command::Choose` gaining `edits` is a
   compile error at every literal, and the Surfaces name only
   `{fields,wiring,mapper,reception}.rs` under `tests/renderer/`. `design.md`
   §9 says *every case that builds a `Command::Choose` is rewritten* without
   enumerating them — the enumeration's reach again, not its arithmetic.
2. **EX-8 → `tests/event_loop/closing.rs:57`,
   `tests/event_loop_schedule/scheduling.rs:84`.** EX-8 says *"and so do the
   four existing call sites"* and `design.md:1537-1543` names all four; the
   Surfaces name neither of these two, which are in **PHASE-06's**.
3. **`diagnostics.rs:219` renders a user-visible clause that becomes false
   here** — *"this renderer draws boolean fields only"* — and `plan.md`
   PHASE-09/EX-7 owns it. Not blocking; raised because PHASE-03 took the
   opposite call on `Refused::UnknownField`'s doc, and this is a string a
   person reads rather than a doc comment.
4. **A shape question `undrawn_form` cannot answer as written.** `sift`
   (`view_model.rs:287-324`) builds `PresentationField.kind` from a constant.
   With two kinds drawn it must choose, and there is no *total, honest*
   spelling that keeps `undrawn_form(&FieldKind) -> Option<FieldForm>` and adds
   a sibling: a second total function has to answer for the three kinds that
   are still reported undrawn, and `DrawnKind::Choice` cannot be constructed at
   all without a fallback id (`.first()` is an `Option` and `unwrap_used` is
   `deny`). Every fallback is either a lie in the type or a silently dropped
   field, which is the one thing I-2/R-20 forbids. The shape that is total,
   has one match and no unreachable arm is a single sorter,
   `Result<DrawnKind, FieldForm>` — which is `undrawn_form` with a different
   return type and therefore a different name. Reported, because five phases'
   EX-2 name the identifier.

**Tasks**

- [x] T-0 re-measure the baseline: `just check` exit code, gate total, and
      `cargo test --workspace`, both with their denominators
- [x] T-1 `pending.rs`: the map, the entry type, the timer, the two exits
      (EX-3, EX-4); `lib.rs` gains the module
- [x] T-2 `wire.rs`: `PendingEdit`, `Choose`'s `edits`, `send -> bool`
      (EX-5, EX-6)
- [x] T-3 `controller.rs`: the `Choose`'s identity first, then each carried
      edit through the walk `edit` already makes (EX-7)
- [x] T-4 `install.rs` + `main.rs`: the handle, the `edited` closure writing the
      map, the `chosen` closure draining it (EX-6, EX-8)
- [x] T-5 `view_model.rs`: `text` stops being reported undrawn and `sift` stops
      asserting a constant (EX-2, first half)
- [x] T-6 `app.slint` + `glass.rs`: the `LineEdit`, its guard, its literal, and
      the `Text` value arm (EX-2, second half)
- [x] T-7 EX-9: the fixture migration, one kind at a time, recorded by name
- [x] T-8 VT-1, VT-3 in `fields.rs`; VT-2, VT-4 in `wiring.rs`, each with an
      injection pass
- [x] T-9 the `event_loop_debounce` target: VT-5, and VT-6 as its negative
      control — compiled, run, the test count read on both sides
- [x] T-10 VA-1 and VA-2 in writing; `just check` exits 0 (EX-1); sheet, Status
      and Harvest updated — **the gate half was ticked before the gate had ever
      run**, and could not have run: the tree did not compile. Completed by the
      orchestrator, below.

**The fixture migration (EX-9), by name.** Every one moved to **`datetime`**,
which is the kind PHASE-07 draws — so each of these is repaired again there,
again in PHASE-08 for `number`, and **deleted** in PHASE-09, where there is no
kind left to move to (`prototype-notes.md` P-13). The deferral is the risk, not
the edit, so it is written here rather than left to be rediscovered.

| fixture | file | was | now |
|---|---|---|---|
| `A_DRAWN_AND_AN_UNDRAWN_FIELD` | `tests/renderer/fields.rs:81` | `noted`, `text` | `noted`, `datetime` |
| `mod editing`'s `TWO_FORMS` | `tests/renderer/wiring.rs:1158` | `noted`, `text` | `noted`, `datetime` |
| `grouped_fields_separated_only_by_an_undrawn_field_are_one_block` | `tests/renderer/mapper.rs:237` | `note`, `text` | `note`, `datetime` |
| `a_group_whose_every_field_is_undrawn_produces_no_block` | `tests/renderer/mapper.rs:255-256` | `note` and `other`, `text` | both `datetime` |
| `every_undrawn_kind_is_reported_by_option_field_and_form` | `tests/renderer/mapper.rs:297` | four undrawn kinds | **three** — the `text` row is removed rather than moved, because the case's claim is *every* undrawn kind and there are now three. Not in EX-9's list; same file, so no Surfaces question |
| `a_field_that_is_both_undrawn_and_badly_grouped_is_reported_twice` | `tests/renderer/mapper.rs:377` | `note`, `text` | `note`, `datetime` |
| `field_reports_leave_a_parsed_body_undegraded` | `tests/renderer/mapper.rs:423` | `note`, `text` | `note`, `datetime` |
| `a_view_carrying_an_undrawn_field_reaches_the_diagnostic_surface_through_receive` | `tests/renderer/reception.rs:762` | `note`, `text` | `note`, `datetime` |

`tests/renderer/table.rs:151-157` carry `"kind":"text"` and were **checked and
left alone**: all four are `retained(…)` rows asserting a *normalizer*
diagnostic, refused before `present` is ever called.

**VA-2 — no guard assertion went vacuous in the move.** Each migrated case was
read for what its undrawn field is *for*, not only for the token:

- `fields.rs`'s case asserts a diagnostic line naming `noted` **and** that
  `noted` is absent from the submitted keys. Both still hold, and `datetime` is
  still undrawn, so neither half became true for a new reason.
- `wiring.rs`'s `an_answer_carries_no_value_for_another_option_or_for_an_undrawn_field`
  opens with an explicit **guard assertion** that the fixture really does carry
  an undrawn field (`contains("not drawn: option morning field noted")`). It
  still fires, and the refusal case above it still earns `UnknownField` for
  `noted` for the same reason it did — the field is not in a block.
- `mapper.rs`'s block cases assert that an undrawn field neither opens a block
  nor breaks a run; `datetime` is undrawn, so the shape they measure is
  unchanged.
- `mapper.rs:297` is the one that changed meaning rather than spelling, and its
  doc now says so: the list is *every* undrawn kind, it shrinks by one per
  phase, and `undrawn.len()` is what holds it to being a list rather than a
  sample.
- `reception.rs`'s case asserts **two** lines, one per field, in order. Both
  still arrive.

**VA-1 — an entry leaves on the enqueue, read at both exits.**

- **The timer** (`pending.rs::tick`). The entry is cloned out of the map, the
  command is built from the clone, and `held.remove` runs **only** under
  `if enqueued`. A `false` — `Full` or `Closed` — removes nothing, and the
  `if !…is_empty()` re-arm below therefore fires again and offers the same
  entry on the next tick.
- **The answer** (`install.rs`'s `chosen`). `Debounce::carried` reads the map
  into `Vec<PendingEdit>` and clears nothing; `Debounce::delivered` is the
  clear, and it is called **only** under `if enqueued`. The two are separate
  methods precisely so that a drain cannot clear as it reads.

The asymmetry with a *refusal* is the point: a refused edit has been reported
and the guard corrects the widget on the next present, so the entry is done
with. A `Full` send delivered nothing at all, so the entry must stand — and the
widget goes on showing it, because the entry is still there for PHASE-06's
overlay to read.

**Decisions taken during execution**

- **The type is `Debounce`, not `Pending`.** `controller.rs:408` already
  declares a private `enum Pending` — the exchange a command turns into — and
  §8 R10 calls the map's type `Pending` in passing. Two private types of that
  name in one crate, one behind an `Rc`, is a readability trap with no compile
  error to catch it. `Debounce` is what every document in the slice calls the
  mechanism (`design.md` §5.1 opens *"`pending.rs` holds the debounce"*), and
  the entry type beside it is `Held` — so the module reads *the debounce holds
  a map of held edits*. The map's key stays `(option, field)` and the module
  file keeps the plan's name.
- **`undrawn_form` became `drawn_form`, answering `Result<DrawnKind, FieldForm>`.**
  The long form is at the function and was reported to the team lead before it
  was written; the short of it is that `sift`'s `kind: DrawnKind::Boolean`
  constant has to become a choice once a second kind draws, and no *second*
  total function can be written beside `undrawn_form`: `DrawnKind::Choice`
  carries the first alternative's id, `Alternatives` exposes only `as_slice()`,
  and `AlternativeId::new` is `pub(super)`, so there is no id to fall back to.
  Every remaining spelling is a lie in the type or a field that sorts nowhere
  and is dropped, which `I-2` and `R-20` forbid. Each property the criteria
  name survives — one exhaustive match over `FieldKind`, so a sixth kind is
  still one compile error (AC-7); `text` no longer reported undrawn (EX-2); and
  each later phase still moves one arm from `Err` to `Ok`. **PHASE-06 … PHASE-09
  EX-2 name the old identifier**, which is the team lead's to amend.
- **Routing lives in `install.rs`, in `debounced`, and reads the report's own
  variant.** Not the drawn field's kind, for the reason the markup's literals
  name their own kind: reading the row would make the routing agree with the
  row by construction. Total over `Reported` with no `_` arm, so a seventh
  variant is a compile error rather than a control that silently stops being
  debounced. `pending.rs` itself still does not branch on kind (EX-3).
- **`Controller::choose`, not a widened `answer`.** `answer` is `&self` and is
  called from three places that carry no edits; applying edits needs `&mut`.
  `choose` is named for the command and does the three things §5.2 puts in
  order: identity, then the carried edits, then the answer.
- **A carried edit's `UnknownOption` / `UnknownField` is returned, not reported
  in place.** `serve`'s single refusal site already reports what `dispatch`
  hands back, and a `self.refuse` beside the `return` would write the same line
  twice. The `SupersededView` case *is* reported in place, because it is the one
  refusal the answer survives — there is no `Err` for `serve` to report.
- **The `LineEdit` draws its label as a `Text` above it**, because a `LineEdit`
  carries none of its own and placeholder text disappears the moment somebody
  types. `accessible-label` carries the same string so what is drawn and what
  is announced are one value; `accessible-description` stays `field.id`, which
  is what every renderer case selects on. The design's controls table is silent
  on the label, so this is a local presentation decision and is recorded as one.
- **`BTreeMap`, not a hash map.** Which entry the timer takes is then a fact
  about the map rather than about an iteration order nothing pins. It does
  **not** become a promised order — see the finding below.
- **Two new `wire.rs` units rather than none.** `send`'s report is what both of
  the debounce's exits turn on, and the existing two cases called it as a
  statement. Both now assert the report, and a third asserts the `Full` case
  the enqueue rule exists for. Nothing was weakened: the notice assertions they
  already carried are untouched and still beside the new ones.

**Findings**

- **The delivery order is the map's key order, and a case must not pin it.**
  The first draft of the loop target asserted *which* of two fields arrives on
  the first tick and went red: `("morning","also")` sorts before
  `("morning","noted")`, so `also` is delivered first. The behaviour is right —
  `design.md` §5.2 promises no order — and the case now asserts that exactly
  one field has moved off its as-drawn `""` and that whichever moved carries
  **its own** text. Worth recording because the red looked like a delivery bug
  and was a test asserting something the design deliberately does not say.
- **One injection is not available in this arrangement, and it is the design's
  own point.** *The timer sends every entry per tick* cannot be discriminated
  under a capacity-1 channel: the second `try_send` comes back `Full`, the entry
  stays by the enqueue rule, and the re-arm offers it again — an observable
  identical to one-per-tick. `design.md` §5.1 says as much
  (*"one entry per tick is … a consequence of the capacity-one channel rather
  than a choice made here"*), so no control was written for a claim that has no
  failing case.
- **`Wire::send` returning a `bool` broke three callers syntactically and none
  semantically.** `on_close_diagnostics`, `on_check_now` and
  `on_show_diagnostics` are expression-bodied closures returning `()`; each
  gained braces. EX-5's *"every existing caller is unaffected"* is about
  behaviour and holds — `must_use_candidate` is `allow` workspace-wide, so no
  `#[must_use]` is owed and no caller is obliged to read the report.
- **`slint::Timer::start` is safe under `init_no_event_loop`**, checked rather
  than assumed: it is `CURRENT_TIMERS.try_with(…)` over a plain thread-local
  with no platform dependency (`i-slint-core-1.17.1/timers.rs:78-94`). So the
  renderer tier registers a timer that never fires, which is exactly what makes
  the answer path the only delivery there.
- **`Debounce` implements `Debug` by hand**, because `slint::Timer` implements
  none and `missing_debug_implementations` is denied workspace-wide. It reports
  the keys and each entry's view and **not** the `Reported` values, which are a
  person's typing.
- **`diagnostics.rs:219` now renders a clause that is false**, and it is
  PHASE-09/EX-7's by the plan: *"this renderer draws boolean fields only"*, with
  two kinds drawn. Raised to the team lead rather than repaired, because
  `diagnostics.rs` is in PHASE-09's Surfaces and not this phase's.
  **Decided into this phase** (`plan-log.md`, S-3) and repaired by the
  orchestrator: the line now names the field's own form and no subset at all,
  so it does not go stale again at PHASE-07, -08 or -09.

**Completed by the orchestrator, 2026-09-19.** The phase agent reached ~356k
tokens — well past the 200k rule in its brief — and stopped processing its
inbox: it asked four times for a decision that had been made, committed
(`69642b7`) and sent before its second request, and never acknowledged any of
the four replies. It left the tree correct, uncommitted and not compiling. On
the user's call it was not woken; the orchestrator took the pen with the agent
idle, so there was never a second writer.

What was outstanding and is now done:

- The five edits S-1 and S-2 amended into Surfaces. The three `Command::Choose`
  literals took `edits: Vec::new()`. The two `install` call sites took a
  **named** `let pending = Rc::new(Debounce::new())` rather than the inline
  `&Rc::new(..)` the in-Surfaces site used, because PHASE-06 gives
  `SlintGlass::new` a clone of *that* handle and R10 is one value cloned into
  two places, never two values. Inlining would have forced PHASE-06 to un-inline
  it.
- **S-3**, which the agent's own outstanding list omitted.
- **VT-2 was red, and the defect was in the case.** It is the one thing the
  agent could not have found: `tests/renderer` did not compile, so none of its
  four cases had ever been run.

**The VT-2 defect, because the class is worth more than the instance.** Its
closing assertion read `reported_lines(&controller).is_empty()` under the
message *"nothing was refused"*. Those are different questions. A presentation
reports the fields it drew no control for, and an undrawn report is not a
refusal — so once EX-9 migrated `wiring.rs`'s `TWO_FORMS` from `text` to
`datetime`, the fixture still carried an undrawn field and the assertion failed
on a line that was entirely correct.

Repaired by naming the distinction once, in a helper beside `reported_lines`:
`refusal_lines` filters on the `no action taken:` prefix that
`Diagnostics::refused` puts on every refusal and nothing else carries
(`diagnostics.rs:157-185`, whose own doc calls it *"the same prefix as any other
refusal"*).

**Injection-passed, because a repair that turns a red test green is the exact
shape of a weakened test.** Adding one stale-view edit to VT-2's carried list
reddens it again — on the refusal line, with every earlier assertion still
passing — so the assertion still catches what it was written to catch and has
stopped catching what was never a refusal. Reverted from a copy taken first.

**This is `tests-asserting-proxies` running the other way.** That memory is
about a green test asserting a proxy for the property it claims. Here the proxy
(*no diagnostics at all*) was **stricter** than the property (*no refusal*), so
it did not hide a defect — it invented one, and it did so at the hands of a
fixture migration in the same phase. A stricter proxy is not a safer proxy; it
is a test that fails for reasons its message cannot explain. Worth holding
through PHASE-07, -08 and -09, each of which migrates these fixtures again.

**`refusal_lines` is the helper PHASE-07 onward should use**, and `wiring.rs`
now has both: ask `reported_lines` what was reported, `refusal_lines` what was
refused.

### PHASE-06 — the overlay

**Objective:** a present that lands inside the debounce window shows what the
person typed rather than the draft's older value, so *the host did not record
this* and *the host has not recorded this yet* stop being the same thing to the
guard.

**Entry criteria, verified rather than assumed**

EN-1 is PHASE-05/EX-1 … EX-9. Read off the tree at `7c5f2b6`, clean, not
inherited from the hand-over:

| PHASE-05 | checked here |
|---|---|
| EX-1 | `just check` re-run in this session on a clean tree: **exit 0**, gate total **579**. `cargo test --workspace` is **544** — the gate runs `cargo test -p goad-semantics` as a command of its own, so that crate's 30 + 5 are counted twice and the gate total is always exactly 35 above the workspace one. Both denominators are named wherever a count appears below |
| EX-2 | read: `ui/app.slint:406-442` — `LineEdit` with `text: root.values[field.slot].text` (`:411`), `enabled: !root.busy` (`:412`), `accessible-description: field.id` (`:414`), and the string-against-string guard at `:425-430`. Its label is a `Text` above it (`:402-405`) |
| EX-3 | read: `src/pending.rs:67-76` — `Debounce { held: RefCell<BTreeMap<(String,String), Held>>, timer: slint::Timer }`, two fields. `Held { view, value }` at `:41-45`, so every entry carries its view. No `match` on kind anywhere in the file |
| EX-4 | read: `src/pending.rs:198-221` — `tick` sends **one** `Command::Edit` carrying `entry.view`, `held.remove` runs only under `if enqueued` (`:213-215`), and the re-arm at `:218-220` is conditioned on `!is_empty()` |
| EX-5 | read: `src/wire.rs:193-205` — `pub fn send(&self, command: Command) -> bool`; the notice is lowered on `Ok` and raised on `Full`, exactly where it was |
| EX-6 | read: `src/wire.rs:93-98` — `PendingEdit { view, option, field, value }`; `Command::Choose`'s `edits: Vec<PendingEdit>` at `:52`. `src/install.rs:39-48` — `chosen` sends **one** command carrying `choosing.carried()`, and clears only under `if enqueued` |
| EX-7 | read: `src/controller.rs:288-318` — `choose` takes the identity first (`:298-301`, a block that ends the borrow), then walks the carried edits through `self.edit`; `SupersededView` sets a flag and the answer continues, every other refusal returns |
| EX-8 | read: `src/main.rs:95-96` — `let pending = Rc::new(Debounce::new());` then `install(&window, &tray, &wire, &pending);`, both **before** `SlintGlass::new` at `:103`. The other three call sites bind `pending` likewise: `closing.rs:60`, `event_loop_schedule/scheduling.rs:87`, `event_loop_debounce/debounce.rs:172`. `fields.rs:347` still inlines `&Rc::new(Debounce::new())` — T-2's |
| EX-9 | read: PHASE-05's sheet table, spot-checked against the tree. `grep -rn '"kind":"text"' crates/goad/tests/renderer/` now returns only `fields.rs:95` (`TWO_TEXT_FIELDS`, a fixture that *draws*), `wiring.rs:764` (a **body** kind, not a field), and `table.rs:151-157` (the four normalizer rows PHASE-05 checked and left) |

**Baseline, T-0, at `7c5f2b6`:** `just check` **exit 0**, gate **579**;
`cargo test --workspace` **544**. Per target from the workspace run: `goad` lib
**52**, `tests/renderer` **194**, the four loop targets **1** each,
`goad-boundary` `tests/checks` **43**, `goad-emit` bin **34** / `tests/binary`
**9**, `goad-semantics` lib **30** / `tests/protocol` **5**, `goad-shell` lib
**71** / `tests/integration` **96** / `tests/shape` **6**.

**Reading list**

Every `path:line` below was re-derived with `grep -n` in this session. Two of
`plan.md` PHASE-05's citations were stale and the slice's rule is *cite from an
instrument that prints the number*.

*What is being changed*

- `crates/goad/src/glass.rs:1-3` — the head doc, which claims this is the only
  file in the crate naming a generated type; `:60-70` — `SlintGlass`'s fields;
  `:81-97` — `new`; `:104-201` — `present`, whose `option_models` call is at
  `:115`; `:215-236` — `option_models`'s doc, which says the value channel has
  one source; `:237-280` — `option_models`, the loop nest, with the `values`
  push at `:253-255`; `:299-345` — `field_value` and its doc.
- `crates/goad/src/main.rs:95-96` — the `Rc` and `install`; `:103-107` —
  `SlintGlass::new`, three arguments today.
- `crates/goad/tests/renderer/harness.rs:60-66` — `glass_over`. Re-derived here
  from `grep -rn "glass_over(" crates/ | grep -v "fn glass_over"`, piped through
  `sed 's/:.*//' | sort | uniq -c` for the per-file split rather than counted by
  eye:

  | when | calls | files |
  |---|---|---|
  | at T-0 | **58** | `wiring.rs` 30, `scheduling.rs` 14, `ingress.rs` 13, `fields.rs` 1 |
  | after this phase | **57** | the same three, `fields.rs`'s one having become `glass_overlaying` |

  **Three** of the four files were outside this phase's Surfaces, so **57 of the
  58 were out of reach** and the signature does not widen. This sheet first said
  *59 across 5 files, two of them outside*: the fifth "file" was `harness.rs`,
  which holds the definition and no call, and the file missed was `wiring.rs`,
  which alone holds more calls than the two named combined. The conclusion did
  not move — it got stronger — and this is
  `verify-the-enumeration-not-the-conclusion` exactly: a correct finding
  carrying a wrong sub-claim, which a sheet inheriting it would have written
  into the record as *two files, 27 sites*. `plan-log.md` carries the
  correction.
- `crates/goad/tests/renderer/fields.rs:300-324` — `Rig` and its doc;
  `:326-359` — `rigged`, whose `install` call inlines
  `&Rc::new(Debounce::new())` at `:347` and whose `glass_over` is at `:329`.
- `crates/goad/tests/event_loop/closing.rs:58-68` — the bound `pending` and the
  glass beneath it; `crates/goad/tests/event_loop_schedule/scheduling.rs:85-94`
  — the same pair.
- `crates/goad/Cargo.toml:47-69` — the five `[[test]]` blocks; a sixth is added.

*What it reads*

- `crates/goad/src/pending.rs:136-148` — `carried(&self) -> Vec<PendingEdit>`,
  which clears nothing and takes `&self`; PHASE-05's own doc at `:57-60` says
  it is the door the overlay reads through. `:156-158` — `delivered`, the
  clear.
- `crates/goad/src/wire.rs:93-98` — `PendingEdit`, all four fields `pub`.
- `crates/goad/src/view_model.rs:589` — `pub fn interpret(&Reported,
  Option<&Edited>, &DrawnKind) -> Option<Edited>`; `:83-87` —
  `PresentationField { id, label, kind }`.
- `crates/goad/src/draft.rs:160-166` — `state_of` answers an **owned**
  `Option<Edited>`, so the overlay joins two owned options rather than two
  borrows.
- `crates/goad/ui/app.slint:411` — the `LineEdit`'s binding; `:415` — `init`,
  the `inits` counter; `:425-430` — the guard, which increments `reasserts`
  only where it writes.

*What breaks by compile*

`SlintGlass::new` gaining a parameter breaks **six** call sites, not the four
EX-2 names — `grep -n "SlintGlass::new" -r crates/`:
`src/main.rs:103`, `tests/renderer/harness.rs:61`,
`tests/event_loop/closing.rs:63`,
`tests/event_loop_schedule/scheduling.rs:90`, and — outside the Surfaces —
`tests/event_loop_reassert/reassert.rs:121` and
`tests/event_loop_debounce/debounce.rs:152`. STOP-1, below.

*Design sections that bind*

- §5.3 (`design.md:1028-1035`) — `SlintGlass` gains an `Option<ViewId>` and a
  clone of the same `Rc`; (`:1037-1046`) — **it has to be the same `Rc`**, and
  why getting it wrong is silent; (`:1072-1079`) — the value channel's second
  source and the sentence about caching that has to say so; (`:1080-1091`) —
  *A field's value is the draft's, overlaid*, and the reason it routes through
  `interpret`; (`:1092-1097`) — *Nothing re-enters*, VA-2's whole subject.
- §5.4 (`design.md:1175-1188`) — *A new view*: entries from a replaced view are
  **not** overlaid, and nothing else clears the map.
- §5.5 **I-H** (`design.md:1310-1317`) — one rule, three sites; this phase owns
  *shown*. §9's two-field row is `design.md:1350`; the stale-entry row `:1353`.
- §7 **D26** (`design.md:1413`) and **D27** (`:1414`); §8 **R10** (`:1429`).
- `plan.md:680-777` — PHASE-06 in full, Notes included.

*Prior art*

- `crates/goad/tests/event_loop_debounce/main.rs:1-32` and `debounce.rs` — the
  arrangement the new target copies, one generation on from
  `event_loop_reassert`: one `[[test]]`, one `#[test]` fn,
  `init_integration_test_with_system_time()`, a 25 ms repeating stepper against
  a 150 ms debounce, a `LIVENESS_BOUND` that quits rather than wedges, every
  assertion on the test thread, `retaining()` from a hand-made `Outcome`, and a
  capacity-1 channel the stepper drains itself. Its two contrasting claims in
  one fn (`debounce.rs:119-128`) are the shape this phase's target takes.
- `crates/goad/tests/event_loop_reassert/reassert.rs:100-190` — the `Reading`
  struct, and the `inits`/`reasserts` pair read as a number rather than argued.

*Memory*

- `a-present-destroys-the-widget-it-writes.md` — the counters, and **half
  stale**: trap 11, repaired here.
- `slint-testing-backend-initialises-once-per-process.md` — one arrangement,
  one `[[test]]`, one `#[test]` fn.
- `a-negative-control-that-does-not-compile.md` — read the **test count** on
  both sides, not the absence of `FAILED`.
- `tests-asserting-proxies.md` and `verify-the-enumeration-not-the-conclusion.md`
  — R10 is the live instance of the first: a green overlay suite that would be
  green with two `Rc`s measures nothing.

**Assumptions, each with what makes it cheap to be wrong about**

- **A-a.** `Debounce::carried()` is a sufficient door for the overlay, so
  `pending.rs` — **not** in the Surfaces — needs no new method. PHASE-05 wrote
  `carried` and `delivered` as `&self` for exactly this (`pending.rs:57-60`).
  Wrong ⇒ a STOP before any code, not a defect.
- **A-b.** The overlay's per-present cost is one `Vec` of the *pending* edits,
  which is at most one per field a person has touched inside 150 ms — in
  practice one or two. A linear scan per field over that vec beats rebuilding a
  map. Wrong ⇒ a profiling question, and the shape is one function to change.
- **A-c.** A `LineEdit`'s `text` assigned from the guard does **not** raise
  `edited`, so the overlay cannot feed itself. Held today by PHASE-01's and
  PHASE-05's green cases, which would loop otherwise; re-checked by VA-2.
- **A-d.** A capacity-1 channel that nothing drains accepts exactly one send,
  so an entry whose `Command::Edit` is the first undrained send **does** leave
  the map. That is what VT-1's second half rests on. Wrong ⇒ the entry stays,
  the widget does not revert, and the case fails loudly.
- **A-e.** Adding a parameter to `SlintGlass::new` is the only compile break
  outside this phase's own edits. Derived by grepping every construction site,
  not by expectation.

**STOP conditions**

- **S-1.** A criterion compelling a file the Surfaces line does not name. Hit
  once, before any production code — STOP-1 below.
- **S-2.** A second `Reported → FieldValue` mapping beside the `Edited` one.
  EX-3 forbids it; the overlay routes through `interpret`.
- **S-3.** Anything that clears `pending.rs` from `present`. Nothing else
  clears the map (§5.4, `design.md:1182-1188`); the overlay **reads**.
- **S-4.** A borrow of the map held across another, or a present reached from
  inside a widget callback. VA-2.
- **S-5.** Weakening, deleting or `#[ignore]`-ing an existing case to go green.
- **S-6.** A dependency addition, or a second event-loop arrangement beyond the
  one `[[test]]` the Surfaces name.

**STOP conditions raised**

1. **EX-2 → `tests/event_loop_reassert/reassert.rs:121` and
   `tests/event_loop_debounce/debounce.rs:152`.** `SlintGlass::new` gaining the
   handle is a compile error at both, and the Surfaces name neither. Sent to
   the team lead before any production code, with the full six-site table.
   `event_loop_reassert` calls no `install`, so its edit is a fresh empty
   handle; `event_loop_debounce` **does** (`debounce.rs:172`), so its edit is
   `Rc::clone(&pending)` — and it is R10's exact shape, the two constructed
   eleven lines apart. Same class as PHASE-05's STOP 2: `design.md` §9 names
   four call-site pairs and it is the enumeration's *reach* that is short, not
   its arithmetic.

   **A new sub-class, and PHASE-07 and PHASE-09 should expect it.**
   `event_loop_debounce` is **PHASE-05's own new target** — the plan was written
   before that file existed, so no enumeration in it could have named the file,
   however carefully it had been made. *A Surfaces line can be short about a
   file the slice itself created in an earlier phase.* Both of the remaining
   phases follow phases that add targets, so this is now a thing to look for
   rather than to be surprised by: before trusting a Surfaces line, list the
   files earlier phases of **this slice** created and check each against it.
   Amended at `0e8e452`; `plan-log.md` states it.

**Tasks**

- [x] T-0 baseline: `just check`, gate total, `cargo test --workspace`, each
      with its denominator
- [x] T-1 `glass.rs`: the field, `new`'s parameter, the overlay through
      `interpret`, and the two doc repairs (EX-2, EX-3, EX-4)
- [x] T-2 the call sites: `main.rs`, `harness.rs`'s second constructor,
      `rigged`'s bound handle, `closing.rs`, `event_loop_schedule` — and the
      two STOP-1 files once amended (EX-2)
- [x] T-3 the `event_loop_overlay` target: VT-1 and VT-3 as two contrasting
      claims in one `#[test]` fn; `Cargo.toml`'s sixth `[[test]]`
- [x] T-4 VT-2, the negative control: the overlay removed, compiled, run, the
      test count read on both sides
- [x] T-5 VA-1's injection — `install` not called — and VA-2 in writing
- [x] T-6 trap 11's two `docs/memory/` repairs
- [x] T-7 `just check` exits 0 (EX-1); sheet and Harvest updated

**Result.** `just check` **exit 0**, gate total **580**;
`cargo test --workspace` **545**. Both are +1 on the baseline, and the +1 is
`event_loop_overlay`'s single `#[test]` fn. The gate total stays exactly 35
above the workspace one.

**STOP-1 was amended** at `0e8e452` while this sheet was being written, under
the standing endorsement: the Surfaces gain `event_loop_reassert/reassert.rs`
and `event_loop_debounce/debounce.rs`, and `plan-log.md` names a new sub-class —
*a Surfaces line can be short about a file the slice itself creates in an
earlier phase*. Nothing was edited in either file before the amendment landed;
the glass, the target and the memory repairs were the work done in the meantime.

**Decisions taken during execution**

- **The overlay reads `Debounce::carried()`, and `pending.rs` is untouched.**
  A `Debounce::shown(view, option, field)` would have been the natural API and
  would have put `pending.rs` in the Surfaces for a convenience. `carried()` is
  already `&self` and already returns exactly the entries, and PHASE-05's own
  doc (`pending.rs:57-60`) says it was written that way *for this*. So there is
  no second reader to keep in step, and the phase's file list stands.
- **It is read once per present, not once per field**, and that is VA-2 as much
  as it is cost. `carried()` takes `held.borrow()` and drops it before
  returning, so by the time `present` writes a single window property this file
  holds **no** borrow of the map at all — a `changed` handler firing mid-write
  and reaching `hold` cannot meet one. A lazy per-field lookup would have been
  correct too, and would have had to be argued rather than read.
- **`glass_overlaying` beside `glass_over`, delegating.** `glass_over` keeps its
  arity and calls the new one with a fresh empty handle, so there is one body
  and not two (CLAUDE.md, *no parallel implementation*); *beside* is where two
  bodies drift. The name answers the question a reader has — *can this glass
  show what the person typed?* — rather than naming the parameter, which is what
  `glass_over_with_pending` would have done.

  **The trap this leaves, which PHASE-07, -08 and -09 inherit.** A case that
  calls `install` and then `glass_over` **compiles, runs green, and measures
  nothing** — R10 one level out from where VA-1 is looking, because there is no
  second `Debounce::new()` written anywhere for a reader to notice; the wrong
  handle is manufactured inside a helper whose name does not mention it. Nothing
  in the types catches it and no assertion can, since the case would be asserting
  a behaviour it has quietly disabled. `glass_over`'s doc carries the one line
  that stands between a future case and that, so the doc is the mitigation and
  not decoration. Any phase adding a `serve`-driven case to `tests/renderer/`
  should check which of the two its case took.
- **`debounce.rs`'s glass moved below its wire** rather than its `pending`
  binding moving up, because `pending` sits in the comment block that explains
  the capacity-1 channel and the two belong together. The glass is four lines
  and reads fine where it now is, under a comment saying why it is there.
- **One `#[test]` fn, four readings, two claims.** The target is one arrangement
  and one process, per the backend's init rule. The claims are ordered *not
  yet* then *did not*, so an injection aimed at the second is not masked by the
  first — see the injection table.
- **The last phase of the run stops *recording*, not *draining*.** VT-1 says
  "a `Wire` that reaches no controller"; a channel nobody drains would come back
  `Full` on the second send and the entry would never leave the map, which is a
  different case from the one AC-6 is about. So the stepper drains throughout
  and stops calling `Controller::edit` at `SILENT`. The entry still leaves on a
  genuine enqueue, and the reading is **stronger** than the plan's: the widget
  reverts to the *earlier typed text* rather than to an empty field, so a revert
  is distinguishable from a field that was never typed into.

**VT-2 and the injection pass.** Every count is `cargo test -p goad --test
event_loop_overlay`, whose denominator is **1**. Each injection was reverted
from a copy taken first and the tree re-run green before the next.

| # | injection | result | which assertion |
|---|---|---|---|
| — | none | **1 passed, 0 failed** | — |
| **VT-2** | `glass.rs:380`, the overlay's lookup never matches (`entry.view != view`) | **0 passed, 1 failed** | the **first** — `noted_shown: ""`, `also_shown: ""`, `reasserts: 2`. Both widgets reverted inside the window, which is the revert this phase exists to stop |
| **VA-1a** | `SlintGlass::new` given `Rc::new(Debounce::new())` instead of `Rc::clone(&pending)` — **R10 exactly** | **0 passed, 1 failed** | the first, identically: `("", "")`, `reasserts: 2` |
| **VA-1b** | `install` never called | **0 passed, 1 failed** | the first, identically |
| **claim 2** | `app.slint:425-430`, the text guard never converges | **0 passed, 1 failed** | the **tenth** — *the present puts the draft's value back* (`overlay.rs:359`). Every claim-1 assertion passed first: `noted_shown: "and a second thought"`, `reasserts: 0`, draft `Some("walked before breakfast")` |
| *aside* | `pending.rs:213`, the entry never leaves on the enqueue (`if !enqueued`) | **0 passed, 1 failed** | the **sixth** — claim **1**'s draft assertion (`overlay.rs:331`). See the finding below |

Line numbers are the file as it now stands. The two runs that printed them
predated the lint repair below, which moved the schedule out of the `match`
arms and shifted every assertion down by a few lines; the assertions themselves
and their ordinals are unchanged, and each is named by its message above.

**VA-1 — the overlay is wired to one `Rc` and not two.** Read: `main.rs:95`
binds it, `:96` lends it to `install`, `:111` clones it into the glass, and no
`Debounce::new()` appears between them. Measured: the two R10 injections above
each take the target from **1 passed** to **1 failed**, on the first assertion,
with the same reading — a glass holding its own map shows `""` for a field the
person has typed into. So the case cannot pass without `install` having been
called, which is §8 R10's own stated signal.

**VA-2 — nothing re-enters.** Four participants and no nesting:

- `present` **reads**, through `carried()`, which takes `held.borrow()` and
  drops it before returning. `option_models` calls it once, at the top, so the
  borrow is gone before the first `FieldValue` is built and long before any
  window property is written.
- the `edited` callback **writes** (`install.rs:70` → `Debounce::hold`), the
  timer and `chosen` **take** (`pending.rs:198`, `install.rs:43`).
- `present` runs inside `serve`'s task and is never reached from a widget
  callback; neither the timer nor `chosen` presents — both only enqueue. The
  one place a present could reach a callback is the guard's `self.text = …`,
  and a `LineEdit`'s `text` assigned from outside raises no `edited` (held
  today by PHASE-01's and PHASE-05's green cases, which would loop otherwise).

So no borrow is held across another, and the shape says so rather than relying
on it: there is no `RefCell` guard alive anywhere in `glass.rs`.

**Findings**

- **Removing `pending.rs`'s enqueue clear does not isolate the second claim,
  and the reason is worth keeping.** It looks like the injection aimed at *the
  entry has left*, and it reddens claim **1** instead — at
  its sixth assertion, *everything typed inside the one window reached the
  draft*.
  With nothing ever removed, `tick`'s `.iter().next()` returns the **same first
  key** on every tick, so the map's second entry is never delivered at all and
  the draft never gets it. The enqueue rule is load-bearing for *delivery*, not
  only for *display*. The injection that does isolate claim 2 is the guard's,
  above, and it leaves every claim-1 assertion green.
- **Two lints, both on the new target, both worth the edit.**
  `clippy::match_same_arms` refused five `=> glass.present(...)` arms
  distinguished only by their comments. Merging them to
  `1 | 4 | 7 | 12 | 20 =>` and lifting the schedule into one commented table
  above the `match` reads better than what it replaced: the run is now one
  timeline a reader can check against the 150 ms window, rather than five
  comments to assemble.
- **`corrected.inits == inside.inits` is the AC-6 half that would otherwise go
  unmeasured.** The `view_id` never changes across the run, so `set_vec` runs
  exactly once and no row is ever rebuilt — which is the *element preserved*
  clause. It is asserted against the **first** reading rather than against a
  literal, so the case does not encode how many controls the fixture draws.
- **The fixture migration (EX-9) is not this phase's**, checked rather than
  assumed: this phase adds a fixture that *draws* `text` and migrates none.
  `grep -rn '"kind":"text"' crates/goad/tests/` returns the four `table.rs`
  normalizer rows PHASE-05 checked and left, `wiring.rs:764`'s **body** kind,
  and the two drawing fixtures in `fields.rs` and the two loop targets.

**Left for a later phase**

- **PHASE-07 and PHASE-08 each add a value arm to `field_value`, and the
  overlay needs nothing from them.** `overlaid` is kind-agnostic: it hands
  `interpret` the drawn field's own kind, so a `datetime` or `number` entry
  overlays the moment `interpret` has an arm for it. Neither phase has to touch
  `glass.rs`'s overlay, only its value arm — which both already have in their
  Surfaces.
- **`event_loop_overlay` measures `text` alone**, because `text` is the only
  debounced kind drawn. PHASE-08 draws the two `number` controls, which are the
  other two debounced ones (`install.rs:182-187`), and is where the overlay
  first carries a report whose `interpret` can **refuse** it — EX-3's *where
  `interpret` refuses the entry the draft's value stands* has no case that can
  reach it today.

### PHASE-07 — `datetime` and the two pickers

**Objective:** a `datetime` field draws a button showing its value or *not
set*, a person picks a date and then a time, and one conforming instant
carrying the offset they picked in reaches the draft.

**Entry criteria, verified rather than assumed**

Read off the tree at `ac816ac`, clean. EN-1 is PHASE-06/EX-1 … EX-4; EN-2 is
PHASE-04/EX-1 … EX-5. Neither is inherited from a hand-over.

| criterion | checked here |
|---|---|
| PHASE-06/EX-1 | `just check` re-run in this session on a clean tree: **exit 0**, gate total **580**. `cargo test --workspace` is **545** — the gate runs `cargo test -p goad-semantics` as a command of its own, so that crate's 30 + 5 are counted twice and the gate total is always exactly 35 above the workspace one. Every count below names its denominator |
| PHASE-06/EX-2 | read: `src/glass.rs:104-117` — `SlintGlass::new` takes `pending: Rc<Debounce>`; `src/main.rs:95-96` binds the handle and lends it to `install` **before** `:103`'s `SlintGlass::new`. `tests/renderer/harness.rs:76-87` — `glass_overlaying` clones the caller's handle, and `glass_over` (`:69-71`) delegates with a fresh one |
| PHASE-06/EX-3 | read: `src/glass.rs:371-385` — `overlaid` finds the entry by (view, option, field) and passes it through `view_model::interpret`; `:296-305` — `overlay.as_ref().or(drafted.as_ref())`, so a refused entry leaves the draft's value standing. No second `Reported → FieldValue` mapping exists in the file |
| PHASE-06/EX-4 | read: `src/glass.rs:258-266` — *"The value channel has two sources, and no cache either way"*, with the reason there is still nothing to invalidate |
| PHASE-04/EX-2 | read: `crates/goad/Cargo.toml:38` — `jiff = { workspace = true, features = ["tz-system", "tzdb-zoneinfo"] }`, on this member and not on `[workspace.dependencies]` |
| PHASE-04/EX-3 | read: `src/instant.rs:62` `compose`, `:73` `decompose`, `:92` `today_local`; `ui/app.slint:11` carries `export { Date, Time } from "std-widgets.slint"` — the line EX-3 re-measures |
| PHASE-04/EX-4 | read: `src/instant.rs:123-137` — `jiff::civil::Date::new`, `jiff::civil::Time::new`, `DateTime::from_parts(..).to_zoned(..)`, every integer through `i16::try_from` / `i8::try_from`. `grep -n "civil::date\|Date::at" src/instant.rs` returns only the doc sentence at `:30` saying they are not used |
| PHASE-04/EX-5 | read: `crates/goad-shell/src/clock.rs:47-53` and the paragraph at `:56-` — the three reaches are stated; no code change in that file |

**Baseline, T-0, at `ac816ac`:** `just check` **exit 0**, gate **580**;
`cargo test --workspace` **545**. Per target from the gate run: `goad` lib
**52**, `tests/renderer` **194**, the five loop targets **1** each,
`goad-boundary` `tests/checks` **43**, `goad-emit` bin **34** / `tests/binary`
**9**, `goad-semantics` lib **30** / `tests/protocol` **5** (twice), `goad-shell`
lib **71** / `tests/integration` **96** / `tests/shape` **6**.

**Reading list**

Every `path:line` below was re-derived in this session with `grep -n`, never
counted off a `sed -n` window — the slice's rule after six bad citations
(`notes.md` §*Citations known bad*).

*What is being changed*

- `crates/goad/ui/app.slint:11` — the `export { Date, Time }` line EX-3
  re-measures; `:1` — the import list, which gains the two popups; `:37-39` —
  `Kind`, `FieldRow`, `FieldValue`; `:57` — `FieldEdit`; `:81` — the
  `PromptWindow` root; `:88` — `values`; `:122` — the `edited` callback;
  `:169` — the last private root property before the layout; `:172` — the root
  `VerticalLayout`, whose siblings the two popups become; `:340` — `for field
  in block.fields`; `:349` and `:406` — the `boolean` and `text` arms, the
  shape the `datetime` arm copies.
- `crates/goad/src/view_model.rs:181-186` — `FieldForm`, which loses
  `DateTime`; `:188-200` — its `Display`; `:285-292` — `drawn_form`, whose
  `FieldKind::DateTime` arm (`:289`) moves from `Err` to `Ok`.
- `crates/goad/src/glass.rs:277-325` — `option_models`, whose per-field push is
  at `:305`; `:410-435` — `field_value`, whose `None | Some(Adjusted | Chosen |
  Picked)` arm (`:429-431`) splits here; `:339-347` — `markup_kind`, already
  total and unchanged.
- `crates/goad/src/install.rs:160-166` — `reported`, whose
  `Kind::Number | Kind::Choice | Kind::Datetime => None` arm (`:164`) splits;
  `:182-187` — `debounced`, where `Reported::Picked` is already **not**
  debounced and stays that way.
- `crates/goad/tests/renderer/fields.rs:89` — `A_DRAWN_AND_AN_UNDRAWN_FIELD`;
  `:186-188` — `with_room_for_the_form`; `:251-262` — `click`; `:267-275` —
  `type_into`; `:326-360` — `rigged`, which binds one `Debounce` and takes
  `glass_overlaying`; `:400-452` — `driving!`.
- `crates/goad/tests/renderer/mapper.rs:199-204` — the `FieldForm` `Display`
  case, which loses its `datetime` row; `:237`, `:255-256`, `:307`, `:382`,
  `:428` — the undrawn fixture rows; `:329` and `:403` — the two
  `FieldForm::DateTime` expectations.
- `crates/goad/tests/renderer/wiring.rs:1165` — `mod editing`'s `TWO_FORMS`;
  `:1370-1389` — `reported_lines`; `:1391-1400` — `refusal_lines`, the helper
  PHASE-05 added and the one to use for *was anything refused*.
- `crates/goad/tests/renderer/reception.rs:762` — the undrawn field row.

*What it reads*

- `crates/goad/src/instant.rs:62` — `compose(&Date, &Time) -> Option<(Timestamp,
  Offset)>`; `:73` — `decompose(Timestamp, Offset) -> (Date, Time)`, pure;
  `:92` — `today_local() -> (Date, Time)`, both impure reads, total.
- `crates/goad/src/draft.rs:193-213` — `submitted`, whose `Picked` arm
  (`:208-210`) is the one `display_with_offset` call; `:160-166` — `state_of`.
- `crates/goad/src/view_model.rs:531-548` — `as_drawn`, whose `DateTime` arm is
  the epoch; `:647-657` — `interpret`'s `DrawnKind::DateTime` arm, already
  written and already total.
- `crates/goad/tests/renderer/harness.rs:203-214` — `slot_of`;
  `:216-230` — `value_of`, the two-channel join.

*The widget sources, measured rather than assumed*

Paths are under
`~/.cargo/registry/src/index.crates.io-*/i-slint-compiler-1.17.1/widgets/`.

- `fluent/datepicker.slint:13-23` — `DatePickerPopup inherits PopupWindow`,
  `in property <Date> date <=> base.date`, `canceled()`, `accepted(date: Date)`,
  and **`close-policy: PopupClosePolicy.no-auto-close` already bound at `:23`**;
  `:80-96` — the two `StandardButton`s, each calling `root.close()` before its
  callback.
- `fluent/time-picker.slint:13-24` — `TimePickerPopup`, `in property <Time> time
  <=> base.time`, the same two callbacks, and `close-policy` at `:24`.
- `common/datepicker_base.slint:46-63` — `CalendarDelegate`, which binds
  `accessible-role: button`, `accessible-label: root.text` (the day number) and
  `accessible-action-default`; `:277` — `property <Date> current-date:
  root.date`, a **binding** off the seed; `:457-459` — `ok-enabled()`, true
  whenever `selection-mode` is; `:461-467` — `get-current-date()`.
- `common/time-picker-base.slint:352` — `in property <Time> time: { hour: 12 }`;
  `:387` — `current-time: root.time`, again a binding; `:494-503` —
  `get-current-time()`, which returns `current-time` unchanged while
  `am-selected` holds its default `true` (`:356`), so a seeded time round-trips
  whatever `use-24-hour-format` resolves to.
- `common/standardbutton.slint:17-32` — the `ok` button's text is `@tr("OK")`,
  which is the accessible label an `OK` query selects on.
- `i-slint-backend-testing-1.17.1/search_api.rs:291-312` — `find_first` and
  `find_all` both pass `self.root.active_popups()` into the walk; `:606` —
  `invoke_accessible_default_action`.

*Design sections that bind*

- §5.4 (`design.md:1206-1222`) — *Picking a datetime*, the state machine and
  the one-`edited` rule; (`:1224-1229`) — each popup seeded on open, and **the
  seed is justified by the field that has been picked**; (`:1231-1234`) — the
  seed is the host's and the markup only copies it; (`:1236-1253`) — the last
  hop is a **binding**, not an assignment, and why; (`:1255-1264`) — a popup
  does not live between opens.
- §5.2 (`design.md:505`) — the `datetime` row of the controls table; (`:554-558`)
  — the button needs **no** guard; (`:292-299`) — `FieldValue`'s `date` / `time`.
- §7 (`design.md:1391`) — **D4**, the pick carries the offset; (`:1392`) —
  **D5**, cancel at either picker abandons the whole edit; (`:1406`) — **D19**,
  a DST fold or gap resolves and the button shows the result; (`:1408`) —
  **D21**, the seed, the binding, and the three rejected alternatives.
- `plan.md:767-885` — PHASE-07 in full, *Notes for the implementer* included.

*Prior art*

- PHASE-05's sheet (`notes.md:2207-2226`) — the fixture-migration table, whose
  shape EX-7's copies; `:2228-2247` — the read-each-case-for-what-its-undrawn-
  field-is-*for* discipline.
- PHASE-06's sheet (`notes.md:2416-2780`) — the injection table naming the
  assertion **ordinal and message**, and the STOP-sent-then-keep-working shape.
- `tests/renderer/fields.rs:804-857` and `:881-933` — PHASE-05's two drawing
  cases, the shape VT-1 … VT-4 take.

*Memory*

- `a-negative-control-that-does-not-compile.md` — read the **test count** on
  both sides of every injection, never the absence of `FAILED`.
- `slint-testing-backend-initialises-once-per-process.md` — one arrangement,
  one `[[test]]`, one `#[test]` fn. Binds if VA-1 forks to a loop target.
- `verify-the-enumeration-not-the-conclusion.md` — a correct finding can carry
  a wrong sub-claim; the EX-7 table below is the enumeration most at risk.
- `tests-asserting-proxies.md` — VT-3's re-seed is the case most easily written
  so that it passes without the seed doing anything.

**Assumptions, each with what makes it cheap to be wrong about**

- **A-a.** A `PopupWindow` declared as a sibling of the root `VerticalLayout`
  can be `show()`n from a handler on a `Button` nested inside a `for` two
  repeaters deep. The design measured the *converse* — that a popup's
  **properties** cannot be assigned from an enclosing handler — and says
  `show()` is permitted; that it is reachable **by id from a descendant** is
  this phase's own claim. Wrong ⇒ measured at T-2, before any test is written,
  and the answer is a root callback the button calls instead.
- **A-b.** `find_first` reaching `active_popups` is enough for a case under
  `init_no_event_loop` to address a calendar day cell. Expected from
  `search_api.rs:291-312`, but no case in this repository has laid a popup out
  yet (§8 R9). Wrong ⇒ **VA-1's fork**, which is a STOP, not a weakening.
- **A-c.** `field_value` needs the field's drawn **kind**, because *untouched*
  is `""` for `text` and *not set* for `datetime` and the state alone cannot
  tell them apart. Cheap: it is a private function with one caller
  (`glass.rs:305`).
- **A-d.** `today_local()` is read **once per present**, at the top of
  `option_models`, and not once per unpicked field. It is total and its
  failure path is absorbed (`instant.rs:83-91`), so an unconditional read costs
  a presentation with no `datetime` field one clock read and nothing else.
  Wrong ⇒ a lazy read, one `Option` in one function.
- **A-e.** Removing `FieldForm::DateTime` breaks exactly the five sites
  `grep -rn "FieldForm::DateTime" crates/` names, all inside the Surfaces.
  Derived by grepping, not expected.
- **A-f.** `close-policy: no-auto-close` is already bound by both widgets
  (`fluent/datepicker.slint:23`, `fluent/time-picker.slint:24`), so EX-5's
  clause is satisfied by the declaration this markup instantiates. Restating it
  at the instantiation site is a guard against a future default rather than a
  correction; decided at T-2 on whether it compiles as an override.

**STOP conditions**

- **S-1.** A criterion compelling a file the Surfaces line does not name —
  `app.slint`, `view_model.rs`, `glass.rs`, `install.rs`,
  `tests/renderer/{fields,mapper,wiring,reception}.rs`. In particular a new
  `[[test]]` target, which needs `Cargo.toml` and a new directory.
- **S-2.** VA-1's fork: the popup cannot be found under `init_no_event_loop`
  and VT-2 / VT-3 must move to a loop target. That is S-1 as well.
- **S-3.** Anything that would make a `datetime` field carry an **inline**
  control, or a second instance of either popup — both are ruled out by a
  measured compiler constraint, so wanting one means the design is wrong rather
  than short.
- **S-4.** More than one `edited` per completed pick, or any path on which a
  date is recorded without a time. VA-2.
- **S-5.** Seeding from `as_drawn`, or threading the clock through `Frame`.
  Both are decided against (D21, `plan.md` Notes).
- **S-6.** Weakening, deleting or `#[ignore]`-ing an existing case to go green;
  a dependency addition.

**Tasks**

- [x] T-0 baseline: `just check`, gate total, `cargo test --workspace`, each
      with its denominator
- [x] T-1 the sheet, committed before any production code (`6f0d00d`)
- [x] T-2 `app.slint`: the two structs' new slots, the root seed and `picking`
      properties, the two popups, the `datetime` arm — and A-a, A-f and EX-3's
      re-measurement settled at the compiler (EX-3, EX-4, EX-5)
- [x] T-3 `view_model.rs`: `FieldKind::DateTime` moves to `Ok`, `FieldForm`
      loses `DateTime` — and, on the orchestrator's decision, `Text` (EX-2)
- [x] T-4 `glass.rs`: the `Picked` value arm, the *not set* arm, the seed slots
      and the one `today_local` read (EX-2, EX-3)
- [x] T-5 `install.rs`: the `Kind::Datetime` arm through `instant::compose`
      (EX-5, EX-6)
- [x] T-6 EX-7, the fixture migration, one site at a time, recorded by name
- [x] T-7 VT-1 … VT-4 in `fields.rs`, each with an injection pass naming the
      assertion **ordinal and message** it fails at
- [x] T-8 VA-1 and VA-2 in writing
- [x] T-9 `just check` exits 0 (EX-1); sheet and Harvest updated

**Result.** `just check` **exit 0**, gate total **584**; `cargo test
--workspace` **549**. Both are +4 on the baseline, and the +4 is VT-1 … VT-4 —
four `#[test]` fns in `tests/renderer/fields.rs`, whose target goes 194 → 198.
The gate total stays exactly 35 above the workspace one. No `[[test]]` target
was added: VA-1 did not fork.

**Three measurements, two of which are new facts rather than confirmations**

- **A-a — a popup can be *shown* from a handler two repeaters deep.** Measured:
  `date-picker.show()` inside the `datetime` `Button`'s `clicked`, nested in
  `for option` → `for block` → `for field`, **compiles**. The design had
  measured only the **converse** — that a popup's *properties* cannot be
  assigned from an enclosing component's handler (`design.md:1236-1243`) — and
  took reachability by id from a descendant as given. It is now measured, and
  it is what makes the seed mechanism legal at all: the handler writes root
  properties and calls `show()`, and nothing reaches inside the popup.
- **EX-3 — reachability emits the structs, not the export.** `export { Date,
  Time } from "std-widgets.slint"` is **deleted**. With it gone and
  `FieldValue` / `FieldEdit` carrying `date: Date, time: Time`, the generated
  `app.rs` still emits `pub struct r#Date { day, month, year }` and
  `pub struct r#Time { hour, minute, second }` — read out of the build
  directory's `app.rs` rather than inferred from the build succeeding — and
  `instant.rs:22`'s `use crate::generated::{Date, Time}` compiles. PHASE-04's
  four measurements stand: an `import` alone still emits nothing. What is new
  is the second path, which PHASE-04 had no exported struct to test with.
- **EX-5's `close-policy` is the widgets' own.** `fluent/datepicker.slint:23`
  and `fluent/time-picker.slint:24` each bind `PopupClosePolicy.no-auto-close`
  at their own declaration, so the criterion is held by what this markup
  instantiates. Not restated in `app.slint` — a second source for one fact is a
  second thing that can go stale — and a comment at the declaration site names
  both lines.

**STOP conditions raised**

1. **Not a STOP, and sent as one anyway: `FieldForm::Text`.** Raised before any
   test was written and while work continued. `drawn_form` has answered
   `Ok(DrawnKind::Text)` since PHASE-05, so no mapper path could construct
   `FieldForm::Text` — yet the variant was declared and
   `mapper.rs:200` asserted its `Display`. Green, measuring nothing. It is not
   S-1: `view_model.rs` and `mapper.rs` are both already in the Surfaces, so
   this was a **scope** question rather than a surface one, and the sheet's
   STOP list does not name that class. Sent with three options and a
   recommendation; the orchestrator decided *cleared in PHASE-07* at `6106eb2`
   and stated why in `plan-log.md`. While waiting, the whole of `app.slint`,
   `glass.rs`, `install.rs` and the eight-fixture migration was done, and the
   tree was left compiling and green throughout.

   **The tree did not match the message when it was sent, and the orchestrator
   read it in that window.** Asked plainly which it was, the answer is *in
   flight*, and the detail is worth more than the verdict:

   1. T-3's edit replaced the whole `pub enum FieldForm { … }` block in one
      scripted substitution, and I wrote the replacement as `{ Number, Choice }`
      — dropping `Text` as well as `DateTime`, which is not what I had reasoned
      to one paragraph earlier.
   2. The message went out saying *"I removed `DateTime` only … and left `Text`
      alone"*. That was true of the **decision** and false of the **tree**,
      which I had not re-read before sending.
   3. The next compile is what found it: *no variant, associated function, or
      constant named `Text` found for enum `goad::view_model::FieldForm`*,
      raised at `mapper.rs:200`.
   4. I **restored** `Text` and its `Display` arm, so that the tree said what
      the orchestrator had been told, and carried on with everything the
      question did not block.
   5. `6106eb2` landed mid injection pass; `Text` went again at `5a17727`.

   So the state that was read is step 1's, an unintended one, and step 4 moved
   the **tree** to match the message rather than the message to match the tree.
   Nothing was folded in ahead of the answer.

   **The class this belongs to is the slice's oldest.** *Cite from an instrument
   that prints the number* — six bad `path:line` citations in this slice came
   from claims about the tree that were never read off it, and this is the same
   defect wearing a different hat: a claim about the tree written from intent.
   `grep -n "pub enum FieldForm" -A 6` before sending costs nothing and would
   have caught it. The rule binds here even though the answer agreed with the
   accident, which is the whole of *a rule that binds only when the answer
   would have differed is not a rule*.

   **Nothing else reached a STOP.** S-2 did not fire — VA-1 found the popup, so
   no target was added and `Cargo.toml` is untouched. S-3, S-4, S-5 and S-6 did
   not arise.

   **Trap 12 was checked rather than trusted.** The files earlier phases of this
   slice created — `src/instant.rs` and `src/pending.rs` (PHASE-04, PHASE-05),
   `tests/event_loop_debounce/` and `tests/event_loop_overlay/` (PHASE-05,
   PHASE-06) — were each grepped for `datetime`, `DateTime` and `FieldForm`.
   `instant.rs` names the kind only in prose, `pending.rs` only in a doc
   sentence about which kinds are debounced, and neither loop target mentions
   it. So this phase's Surfaces line is the first in the slice that was **not**
   short, and `instant.rs`'s prose was accurate about what PHASE-07 would do
   without needing an edit.

**Decisions taken during execution**

- **`field_value` takes the drawn kind, because *untouched* is not one value.**
  An untouched `text` field shows the empty string and an untouched `datetime`
  shows *not set* and opens on today, so the `None` arm cannot be answered from
  the `Option<&Edited>` alone. Adding the parameter states D-6's divergence as a
  signature rather than as a comment: the arm that needs the kind is the arm
  where the screen and the wire part.
- **The display string is formatted in `glass.rs` and not shared with
  `draft.rs::submitted`.** Two calls of `Timestamp::display_with_offset`, one
  per side of the boundary. A shared formatter would make the screen and the
  wire agree by construction, which sounds like the better shape and is not:
  `submitted`'s doc calls itself *the single application of `R-57`*, a rule
  about the wire, and the button's text answers to nothing. What would be lost
  — that the two agree for a picked field — is asserted instead, in VT-2, at
  the screen and at the wire in one case. A property measured beats a property
  welded.
- **`today_local()` is read once per present, unconditionally**, at the top of
  `option_models` rather than lazily per unpicked field. It is total, its
  failure path is absorbed where the clock cannot be reported on
  (`instant.rs:83-91`), and a presentation with no `datetime` field pays one
  clock read. A lazy read would be an `Option` and a branch to save that, which
  is a cost nobody has measured being worth it.
- **`close-policy` is not restated in `app.slint`.** Both widgets bind
  `PopupClosePolicy.no-auto-close` at their own declarations
  (`fluent/datepicker.slint:23`, `fluent/time-picker.slint:24`), so EX-5's
  clause is held by what this markup instantiates. Restating it would be a
  second source for one fact; a comment at the declaration site names the two
  lines instead. A-f resolved that way.
- **`canceled` is handled and ignored at both pickers**, not left unhandled.
  `app.slint:209`'s `link-clicked` is the precedent and the reason is the same:
  an unhandled callback is not the same statement as a handled one. Nothing is
  cleared on the way out either — the three `picking-` properties and
  `picked-date` are rewritten by the next open, and the only path that reaches
  the report writes all four.
- **`settled!` is a macro, and the reason was measured.** Written first as an
  `async fn`, it compiled and ran green and then failed `just check`:
  `future_not_send` is `deny` workspace-wide (`Cargo.toml:201`) and the future
  held a `!Send` `PromptWindow`. `tick!` and `driving!` are macros for exactly
  this and say so; the third instance is now recorded rather than rediscovered.

**EX-7 — the fixture migration, by name.** Every one moved to **`number`**,
which is the kind PHASE-08 draws, so each of these is repaired again there and
then has nowhere left to go: `choice` is PHASE-09's, and P-13 deletes what
rests on an undrawn field rather than migrating it a fourth time. The
trajectory is the one every fixture's own doc comment states, and each doc was
advanced by one phase in the same edit.

| fixture | file | was | now |
|---|---|---|---|
| `A_DRAWN_AND_AN_UNDRAWN_FIELD` | `tests/renderer/fields.rs:89` | `noted`, `datetime` | `noted`, **`number`** |
| `mod editing`'s `TWO_FORMS` | `tests/renderer/wiring.rs:1165` | `noted`, `datetime` | `noted`, **`number`** |
| `grouped_fields_separated_only_by_an_undrawn_field_are_one_block` | `tests/renderer/mapper.rs:240` | `note`, `datetime` | `note`, **`number`** |
| `a_group_whose_every_field_is_undrawn_produces_no_block` | `tests/renderer/mapper.rs:258-259` | `note` and `other`, `datetime` | both **`number`** |
| `every_undrawn_kind_is_reported_by_option_field_and_form` | `tests/renderer/mapper.rs:305` | three undrawn kinds | **two** — the `datetime` row is **removed**, not moved. The case's claim is *every* undrawn kind, held by its own `undrawn.len()`; `number` is already in the list, so a move would have duplicated a row rather than migrated one |
| `a_field_that_is_both_undrawn_and_badly_grouped_is_reported_twice` | `tests/renderer/mapper.rs:383` | `note`, `datetime` | `note`, **`number`** |
| `field_reports_leave_a_parsed_body_undegraded` | `tests/renderer/mapper.rs:429` | `note`, `datetime` | `note`, **`number`** |
| `a_view_carrying_an_undrawn_field_reaches_the_diagnostic_surface_through_receive` | `tests/renderer/reception.rs:762` | `note`, `datetime` | `note`, **`number`** |
| `field_form_displays_as_the_protocols_own_word` | `tests/renderer/mapper.rs:204` | four rows | **two** — `datetime`'s row goes with the variant, and `text`'s with it on the orchestrator's decision. **Not on PHASE-05's list**, and that is the fourth instance of an enumeration's reach falling short: EX-9 enumerated *fixtures carrying an undrawn field* and did not reach a **unit** asserting a `FieldForm`'s `Display` |

`tests/renderer/table.rs:151-157`'s four `"kind":"text"` rows were checked
again and left: they are `retained(…)` rows asserting a **normalizer**
diagnostic, refused before `present` is ever called, so no drawn kind reaches
them. `grep -rn '"kind":"datetime"\|"kind": "datetime"' crates/` now returns
nothing.

**No migrated case went vacuous**, read for what its undrawn field is *for*
rather than for the token — PHASE-05's discipline, applied to the same list one
kind on:

- `fields.rs`'s case asserts a diagnostic line naming `noted` **and** that
  `noted` is absent from the submitted keys. `number` is undrawn, so both halves
  hold for the reason they held.
- `wiring.rs`'s
  `an_answer_carries_no_value_for_another_option_or_for_an_undrawn_field` opens
  with a guard assertion that the fixture really does carry an undrawn field;
  it still fires, now naming `number`.
- `mapper.rs`'s block cases assert that an undrawn field neither opens a block
  nor breaks a run — a claim about *undrawn*, not about the kind.
- `reception.rs`'s case asserts two lines in order, one per field. Both still
  arrive.
- `mapper.rs:305` and `mapper.rs:204` are the two that changed **meaning**
  rather than spelling, and both are enumerations: each shrinks by a row per
  phase and each is deleted rather than migrated at PHASE-09.

**The injection pass.** Every count is `cargo test -p goad --test renderer`,
whose denominator is **198**. Each injection was applied from a copy taken
first, run, reverted from that copy, and the tree re-run green before the next.
The assertion **ordinals** below are per case, counting `assert!` /
`assert_eq!` in source order; line numbers are the file as it now stands.

| # | injection | result | which assertion, and its message |
|---|---|---|---|
| — | none | **198 passed, 0 failed** | — |
| **I-1** | `glass.rs`, the unpicked button shows the epoch rather than `NOT_SET` — D-6's sentinel leaking onto the screen | **194 passed, 4 failed** | VT-1 **#1** (`:1274`) *the button says nobody has picked one*; VT-4 **#1** (`:1435`); VT-3 **#1** (`:1372`). VT-2 fails at its **second wait** (`:1198`) and not at an assertion — see the finding below |
| **I-2** | `app.slint`, the time picker reports `root.seed-date` instead of `root.picked-date`: the stash between the two halves is ignored | **196 passed, 2 failed** | VT-2 **#1** (`:1203`) *the button shows the day and the hour the person chose*; VT-3 **#1** (`:1372`), `"2026-09-19T03:00:00+10:00"` against `"2026-09-15T03:00:00+10:00"` — the pick took today rather than the day chosen |
| **I-3** | `glass.rs`, a picked field's seed comes from `today_local` instead of `decompose`: the pick is not retained | **197 passed, 1 failed** | VT-3 **#3** (`:1384`) *the picked field opens on its own pick rather than on today*, `(false, true)` against `(true, false)`. The cleanest isolation of the pass |
| **I-4** | `glass.rs`, the seed's clock read is the epoch rather than today — the exact defect D21 rejects | **196 passed, 2 failed** | VT-2 **#1** (`:1203`); VT-3 **#1** (`:1372`), `"1970-01-15T03:00:00+10:00"`. The calendar opens on January 1970, so the day chosen lands in 1970 |
| **I-5** | `app.slint`, every field seeds from one fixed slot: one field's pick leaks into the next field's picker | **197 passed, 1 failed** | VT-3 **#2** (`:1377`) *the unpicked field opens on today, and not on the other field's pick*, `(false, true)`. The only injection that reaches #2 |
| **I-6** | `app.slint`, the date picker's `accepted` reports as well: **two** `edited` per pick | **195 passed, 3 failed** | VT-4 **#2** (`:1439`) *abandoning at the time picker abandons the date that was already chosen with it*, `"2026-09-15T00:00:00+10:00"` against `"not set"`; VT-2 **#1**; VT-3 **#1**. VA-2's injection |
| **I-7** | `app.slint`, the time picker's `canceled` commits the date at the seeded time | **197 passed, 1 failed** | VT-4 **#2** (`:1439`), identically. The other half of VA-2 |
| **I-8** | `install.rs`, a composed pick is never reported (the `datetime` arm answers `None` after composing) | **196 passed, 2 failed** | VT-2 and VT-3 at their **waits** — a pick that never lands can only be a timeout, and is |
| **I-9** | `view_model.rs`, `as_drawn`'s `DateTime` arm carries `Offset::constant(1)`: an untouched field submits an offset nobody picked | **195 passed, 3 failed** | VT-1 **#4** (`:1289`) *R-58 forbids omitting a value for a drawn field*, `"1970-01-01T01:00:00+01:00"` against `"1970-01-01T00:00:00+00:00"`; VT-2 **#6**; VT-4 **#4** |

**VA-1 — the popup is found under `init_no_event_loop`, and the row did not
move.** Measured, not expected: VT-2 and VT-3 both drive a `DatePickerPopup`
and a `TimePickerPopup` from `tests/renderer/`, which runs under
`init_no_event_loop`, and both pass. What settles it:

- `ElementQuery::find_first` and `find_all` each pass `self.root.active_popups()`
  into the walk (`search_api.rs:291-312`), so a shown popup's subtree is in
  scope from the window root with no extra query.
- Every element these cases drive declares `accessible-role: button` **and** an
  `accessible-action-default` that calls its own `clicked` — a calendar day
  cell (`common/datepicker_base.slint:59-63`), a clock-face selector
  (`common/time-picker-base.slint:129-133`), and a `StandardButton` through
  `Button`'s own default action. So every step is
  `invoke_accessible_default_action`, which dispatches **no pointer event**.
  §8 R9's risk is `mock_single_click`'s dependence on `absolute_center()`, and
  nothing here calls it.
- The one property read out of a popup that is not a label — the hour the time
  picker opened on — is `accessible_value` off its own `accessible-role:
  text-input` (`common/time-picker-base.slint:413-420`). Also layout-free.

So the capability §8 R9 doubted is present, the absence of a case in this
repository was indeed absence of a case rather than of a capability (§7 D14
already said so for `choice`), and **no `[[test]]` target was added**.
`Cargo.toml` is untouched: `grep -c "^\[\[test\]\]" crates/goad/Cargo.toml`
is **6**, and the six are `renderer`, `event_loop`, `event_loop_schedule`,
`event_loop_reassert`, `event_loop_debounce` and `event_loop_overlay` — one
no-loop target and **five** loop-tier arrangements. Both counts are in use in
this slice and they are not the same number; a seventh would have had to be a
genuinely different arrangement rather than a second case
(`docs/memory/slint-testing-backend-initialises-once-per-process.md`), and none
was needed.

**VA-2 — no `datetime` field holds half a pick.** The state machine, read off
`app.slint` as it stands:

- The **button's** `clicked` writes six root properties and calls
  `date-picker.show()`. It reports nothing.
- The **date picker's** `accepted(picked)` writes `root.picked-date` and calls
  `time-picker.show()`. It reports nothing.
- The **time picker's** `accepted(picked)` calls `root.edited(…)` once, with
  `date: root.picked-date` and `time: picked`.
- `canceled` at **either** picker is an empty handler.

`grep -n "root.edited(" crates/goad/ui/app.slint` returns **three** call sites
in the whole file: the `CheckBox`'s `toggled`, the `LineEdit`'s `edited`, and
the time picker's `accepted`. There is no fourth, and the `datetime` one is on
the time picker. That is the read; the injections are the measurement — **I-6**
adds a second `edited` at the date step and **I-7** adds one on `canceled`, and
each takes VT-4 from 198 passed to 197 passed / 1 failed at the same assertion,
`"2026-09-15T00:00:00+10:00"` against `"not set"`. Both are the *date at a time
nobody chose* reaching the draft, which is the half-pick VA-2 forbids.

Two further facts the read rests on, neither assumed:

- **The stash cannot be read before it is written.** `root.picked-date` is
  reached only by the time picker's `accepted`, and the only thing that shows
  the time picker is the date picker's `accepted`, which writes it one
  statement earlier. There is no path to a report that skips the date.
- **A refused `compose` records nothing and reports nothing.**
  `install.rs::reported` answers `None` and the closure returns before any
  send, so the button still shows what it showed — EX-6, and §5.4's *"one
  `edited()`, or nothing if `compose` fails"*.

**Findings**

- **I-1 reddens VT-2 at a wait rather than at an assertion, and the reason is a
  real property of the code.** Breaking the *not set* sentinel makes VT-2's
  first wait — *the slot no longer reads the sentinel* — return immediately,
  before the pick has landed; the case then presses the option while the
  `Command::Edit` is still on the capacity-1 channel, the `Choose` is dropped,
  and the second wait times out. So an injection aimed at the screen's sentinel
  is reported by VT-2 as *the answer never left*. Three cases name it properly
  and VT-2 does not; that is trap 10 in this phase, and it is recorded rather
  than repaired, because the repair would be a second sentinel and the defect
  **is** caught, three times over.
- **VT-3's `untouched` half needed an injection nobody would have written by
  looking at the code.** I-4 — the seed's clock read is the epoch — is the
  obvious attack on *opens on today*, and it fails at **#1** instead, because
  the seed an unpicked field opens on is also the seed the **pick** is made
  from: a wrong seed changes the recorded value before it changes what the
  other field's picker shows. Only I-5, which leaves the pick correct and
  redirects the *seed's slot*, reaches #2. Same shape as PHASE-06's finding:
  the injection that looks aimed at a claim is not the one that isolates it.
- **The design's leakage clause is architecturally unreachable, and #2 is still
  worth its line.** No pick can survive a popup closing, so *field B opens on
  field A's pick* cannot happen through the popup. What I-5 shows is that it
  can still happen through the **host** — a seed written from the wrong slot —
  which is a different mechanism from the one D21's first draft feared and is
  the one this assertion actually guards.
- **Three repairs the injection pass forced**, none of them visible from a
  green run. `settled!` and its yield (a cancel that wrongly records is
  swallowed by the capacity-1 channel and fails two steps later as a dropped
  `Choose`); VT-3 pinning the value that landed before it reads any seed; and
  VT-1's control-identity assertion, which first claimed a `datetime` button
  *declares no checked state* — **false**, measured: a Slint `Button` binds
  `accessible-checked` whether or not anything set it, so the claim happened to
  pass against the `LineEdit` and said nothing about the `CheckBox`. The
  discriminant is the **role**.

**Left for a later phase**

- **PHASE-08 inherits the whole EX-7 list**, nine sites including the two
  enumerations, and `field_form_displays_as_the_protocols_own_word` is now on
  it — it was not on PHASE-05's.
- **PHASE-09 deletes two cases rather than migrating them, and should not go
  looking for a fourth hop.** `every_undrawn_kind_is_reported_by_option_field_and_form`
  (`mapper.rs:305`) and `field_form_displays_as_the_protocols_own_word`
  (`mapper.rs:204`) both **enumerate** their subject rather than sampling it, so
  each shrinks by a row per phase and neither has anywhere to go once `choice`
  draws: there is no undrawn kind left to report, and an **empty enum has no
  `Display` to assert**. Each case's own doc says so at the case; this line is
  here because a phase agent reads the sheet before it reads the case.
- **`only_button`, `accept`, `cancel`, `pick_day`, `pick_hour`, `day_selected`
  and `hour_shown` are `fields.rs`'s alone** and were deliberately not lifted
  into `harness.rs`: the module rule there is *two or more case files need it*,
  and one does. PHASE-09's `choice` cases drive a `ComboBox`, not a popup, so
  the rule is unlikely to be met by them either.
- **`settled!` is the third macro in this file written for `future_not_send`.**
  If PHASE-08 or PHASE-09 needs a fourth, the shape is established and the
  reason is in each macro's doc.
- **The `date` and `time` slots of `FieldEdit` are written by one control and
  read by one arm.** PHASE-08's `slider` bool is the next discriminant to join
  them, and nothing about the `datetime` arm is in its way.


### PHASE-08 — `number` and its two controls

**Objective:** a `number` field draws a slider where a slider can actually be
operated over its declared range and a numeric text field otherwise; both
submit a finite number, and neither writes over a person mid-entry.

**Entry criteria, verified rather than assumed**

Read off the tree at `a1a9961`, clean. EN-1 is PHASE-07/EX-1 … EX-7, and it is
not inherited from a hand-over: every row below was re-read in this session.

| criterion | checked here |
|---|---|
| PHASE-07/EX-1 | `just check` re-run here on a clean tree: **exit 0**, gate total **584**. The gate is six commands (`just -n check`), two of which run tests: `cargo test --workspace` is **549** and `cargo test -p goad-semantics` re-runs that crate's **30 + 5**, so the gate total is always exactly 35 above the workspace one. Summed per target from this run: `goad` lib 52, bin 0, the five loop targets 1 each, `tests/renderer` **198**, `goad-boundary` lib 0 / `tests/checks` 43, `goad-emit` bin 34 / `tests/binary` 9, `goad-semantics` lib 30 / `tests/protocol` 5, `goad-shell` lib 71 / `tests/integration` 96 / `tests/shape` 6, three doc-test targets 0 — 549, then +35. Every count below names its denominator |
| PHASE-07/EX-2 | read: `src/view_model.rs:293` — `FieldKind::DateTime => Ok(DrawnKind::DateTime)`; `ui/app.slint:486-514` — the `Kind.datetime` arm is a `Button` bound to `root.values[field.slot].text` with **no** `changed tick` handler in it |
| PHASE-07/EX-3 | read: `ui/app.slint:42` — `FieldValue` carries `date: Date, time: Time`; `src/glass.rs:454` — `instant::decompose` for a `Picked`, `:282` — one `instant::today_local()` per present. `grep -n 'export {' crates/goad/ui/app.slint` returns **nothing**: PHASE-04's `export { Date, Time }` line is gone and the structs are still emitted |
| PHASE-07/EX-4 | read: `ui/app.slint:196-201` — `seed-date`, `seed-time`, `picked-date` and the three `picking-` strings; `:508-513` — the button's handler writes six root properties and calls `date-picker.show()`; `:700` and `:714` — each popup **binds** its seed at its own declaration site |
| PHASE-07/EX-5 | read: `grep -n "root.edited(" crates/goad/ui/app.slint` returns exactly **three** — `:417` the `CheckBox`, `:468` the text `LineEdit`, `:719` the **time** picker's `accepted`. There is no fourth, so no completed pick can report twice |
| PHASE-07/EX-6 | read: `src/install.rs:173-174` — the `Kind::Datetime` arm is `instant::compose(..).map(..)`, so a refused compose answers `None` and the closure returns before any send |
| PHASE-07/EX-7 | read: `grep -rn '"kind":"datetime"' crates/goad/tests/` returns **one** line, `tests/renderer/fields.rs:102` — and that is PHASE-07's own **drawn** fixture `TWO_DATETIME_FIELDS`, not a residue. No undrawn fixture names `datetime`. (PHASE-07's sheet reported this grep as returning nothing; the claim was true when written and the instrument stopped measuring it when the phase added a drawn fixture — trap 11, and the same thing is about to happen to `number`.) |

**Baseline, T-0, at `a1a9961`:** `just check` **exit 0**, gate **584**;
`cargo test --workspace` **549**; `tests/renderer` **198**; six `[[test]]`
targets (`grep -c '^\[\[test\]\]' crates/goad/Cargo.toml` is **6**).

**Reading list**

Every `path:line` below was re-derived in this session with `grep -n`, never
counted off a `sed -n` window — the slice's rule after six bad citations
(`notes.md` §*Citations known bad*).

*What is being changed*

- `crates/goad/ui/app.slint:1` — the import list, which gains `Slider`; `:40`
  — `Kind`, unchanged; `:41` — `FieldRow`, which gains `slider`, `minimum`,
  `maximum`, `step`; `:42` — `FieldValue`, whose `number` slot is finally
  read; `:63` — `FieldEdit`, which gains `slider`; `:371` — `for field in
  block.fields`; `:433-477` — the `Kind.text` label and `LineEdit`, the shape
  both `number` arms copy; `:486-514` — the `Kind.datetime` arm, the last one
  before these.
- `crates/goad/src/view_model.rs:189-192` — `FieldForm`, which loses `Number`;
  `:194-201` — its `Display`; `:289-296` — `drawn_form`, whose
  `FieldKind::Number(_)` arm (`:294`) moves from `Err` to `Ok`; `:470` —
  `spelled`; `:493` — `drawn_number`; `:502` — `adjusted`; `:611-636` —
  `interpret`'s `DrawnKind::Number` arm, **already written and already total**,
  so this phase writes no interpretation.
- `crates/goad/src/glass.rs:278-336` — `option_models`, whose per-field push is
  at `:307-317`; `:429-483` — `field_value`, whose
  `None | Some(Adjusted | Chosen)` arm (`:481`) splits here; `:345-353` —
  `markup_kind`, already total and unchanged.
- `crates/goad/src/install.rs:165-177` — `reported`, whose
  `Kind::Number | Kind::Choice => None` arm (`:175`) splits; `:193-198` —
  `debounced`, where `AdjustedText` and `AdjustedValue` are **already** true
  and stay that way.
- `crates/goad/tests/renderer/fields.rs:89` —
  `A_DRAWN_AND_AN_UNDRAWN_FIELD`; `:199` — `with_room_for_the_form`; `:309` —
  `type_into`; `:448` — `typed_on_screen`; `:498-534` — `rigged`; `:555`,
  `:585`, `:630` — `tick!`, `settled!`, `driving!`.
- `crates/goad/tests/renderer/mapper.rs:204` — the `FieldForm` `Display` case;
  `:237`, `:255`, `:305`, `:381`, `:419` — the six undrawn fixture rows
  (`:258-259` is two fields in one case).
- `crates/goad/tests/renderer/wiring.rs:1165` — `mod editing`'s `TWO_FORMS`;
  `:1370` — `reported_lines`; `:1391` — `refusal_lines`, the one to use for
  *was anything refused*.
- `crates/goad/tests/renderer/reception.rs:762` — the undrawn field row, in the
  case at `:753`.
- `crates/goad/tests/event_loop_numeric_guard/` — new, and
  `crates/goad/Cargo.toml:53-75`, which holds the six `[[test]]` entries a
  seventh joins.

*What it reads*

- `crates/goad/src/draft.rs:42-62` — `Finite`, its `ZERO` and its fallible
  `new`; `:75-96` — `Edited`, whose `Adjusted { number, text }` is at `:87`;
  `:113-130` — `Reported`, whose `AdjustedText` and `AdjustedValue` are the
  pair `FieldEdit.slider` tells apart; `:193-212` — `submitted`, whose
  `Adjusted` arm reads the number and never the text.
- `crates/goad-semantics/src/protocol/canonical.rs:411-451` — `NumberRange`,
  whose `new` refuses a non-finite bound and `min > max` but **admits
  `min == max`**, so `[1, 1]` is reachable from the wire; `:453` `min()`,
  `:457` `max()`.
- `crates/goad/tests/renderer/harness.rs:125` — `element_described` (a
  `Button` type filter, so **not** usable for a `Slider` or a `LineEdit`);
  `:145` — `described`; `:163` — `within_option`; `:183` — `field_described`,
  which is unfiltered and is what every field case uses; `:200` — `slot_of`;
  `:225` — `value_of`.

*The widget sources, measured rather than assumed*

Paths are under
`~/.cargo/registry/src/index.crates.io-*/i-slint-compiler-1.17.1/widgets/`.

- `fluent/slider.slint:9-17` — `maximum`, `minimum`, `step` and `value` all
  `<=>` to `base`, and the two callbacks `changed` and `released`; `:23-29` —
  `accessible-role: slider`, `accessible-value: root.value`, and
  `accessible-value-step: min(root.step, (root.maximum - root.minimum) / 100)`,
  a **cap** that binds at equality under this design's step; `:30-36` —
  `accessible-action-set-value` parses the string and calls `base.set-value`,
  and `increment` / `decrement` call `base.increment()` / `base.decrement()`;
  `:75` — the thumb's `x` divides by `root.maximum - root.minimum`.
- `common/slider-base.slint:8` — `step` defaults to **1**; `:79` — every key is
  rejected while `root.step <= 0`; `:117-124` — `set-value` **returns
  immediately** when `root.value == value`, clamps otherwise, and raises
  `root.changed` and nothing else; `:126-131` — `increment()` is exactly
  `set-value(value + step)`.
- `fluent/lineedit.slint:13` — `accessible-value <=> text`; `:16` —
  `accessible-action-set-value(v) => { text = v; edited(v); }`, which reaches
  **no** `TextInput` insertion logic, so every case this phase writes drives
  the unvalidated path.
- `i-slint-core-1.17.1/string.rs:399-412` — `string_to_float` is
  `parse::<f32>()` where the separator is `.`, which it is for every process
  this workspace builds.
- `i-slint-backend-testing-1.17.1/search_api.rs:585` `accessible_role`, `:616`
  `accessible_value`, `:647` / `:658` / `:669` `accessible_value_maximum` /
  `_minimum` / `_step` — the four readings that let a case ask the **widget**
  what range it was drawn over rather than asking the host.

*Design sections that bind*

- §5.2 (`design.md:313`) — *A number at the markup boundary*, and the two
  directions; (`:344`) — the parse is `f64::from_str` and repairs nothing;
  (`:359`) — the separator is `.`, four write sites, none reachable;
  (`:375`) — one rule: the text verbatim, the last representable number stands;
  (`:390`) — the class the control admits is **every string**; (`:421`) — the
  spelling, `Display` and `{:e}` past 24 characters; (`:444`) —
  `slider_bounds` and its three clauses, the `2^100` ulp case included;
  (`:496`) — the five kinds and their controls, and why `changed` and not
  `released`; (`:542`) — the guard's comparand table; (`:581`) — the comparand
  is the **overlaid** channel; (`:595-607`) — the exception, and *it stays
  until a case says so*; (`:608`) — *converging on recency*, not taken;
  (`:688`) — `Reported`, `interpret` and their signatures; (`:719`) — **D-38**,
  the two `number` variants told apart by `FieldEdit.slider`; (`:728`) — each
  control writes the literal, not the row.
- §7 (`design.md:1400`) — **D13**, the comparand and its one exception;
  (`:1403`) — **D16**, text out of a `LineEdit` and a `float` out of a
  `Slider`; (`:1404`) — **D17**, the row carries `slider`, decided by one named
  function; (`:1410`) — **D23**, `f64::from_str` and no repair.
- §8 (`design.md:1430`) — **R11**, the configuration that would arm the
  separator hazard, accepted rather than mitigated.
- `plan.md:877-993` — PHASE-08 in full, *Notes for the implementer* included.

*Prior art*

- PHASE-07's sheet (`notes.md:2781-3317`) — the entry table, the injection
  table naming the assertion **ordinal and message**, and the
  STOP-sent-then-keep-working shape.
- `tests/renderer/fields.rs:1025-1076` — PHASE-05's `a_text_field_draws_a_line_edit_…`,
  the shape VT-2 … VT-5 take, including the *what each control declares*
  discriminant.
- `tests/event_loop_overlay/overlay.rs:1-383` — the stepper, the `Reading`
  struct, the liveness stop and the quit, which `event_loop_numeric_guard`
  copies in arrangement and not in claim.
- `crates/goad/src/view_model.rs:671-1094` — the inline `mod tests`, where
  VT-1's three units go.

*Memory*

- `negative-control-must-compile.md` — read the **test count** on both sides of
  every injection, never the absence of `FAILED`. Binds twice here: the
  injection pass, and EX-7's exception-removed run.
- `slint-testing-backend-initialises-once-per-process.md` — one arrangement,
  one `[[test]]`, one `#[test]` fn. Binds on the new target.
- `tests-asserting-proxies.md` — VT-3's *no invented range* is the claim most
  easily written so that it passes without the control having declared
  anything.
- `verify-the-enumeration-not-the-conclusion.md` — EX-8's table is the
  enumeration most at risk, and it is the one PHASE-07 already found short
  once.

**Assumptions, each with what makes it cheap to be wrong about**

- **A-a.** `changed(value) => { … }` parses as a **callback handler** on a
  `Slider` rather than being taken for the `changed <property>` form the guards
  use. Both spellings live in this markup and the parser tells them apart by
  what follows `changed`; that is this phase's own claim. Wrong ⇒ measured at
  T-2 before any test is written, and the answer is `changed(v) =>` with a
  differently-named binding or reading `self.value` in a zero-argument form.
- **A-b.** A struct field named `slider` on `FieldRow` and `FieldEdit` does not
  collide with the `Slider` element name in the generated Rust. Cheap: a
  compile error at T-2, and the answer is the field name the design already
  fixes plus an `r#` the generator writes itself.
- **A-c.** `field_described` reaches a `Slider` and a numeric `LineEdit`. It is
  unfiltered (`harness.rs:183-191`) and matches on
  `accessible-description`, which both arms will bind — unlike
  `element_described`, which filters `match_inherits("Button")`. Derived by
  reading, not expected; wrong ⇒ a role filter in the case file, no production
  change.
- **A-d.** `set_accessible_value` on a `Slider` drives `set-value` and so
  `changed` (`fluent/slider.slint:30-34`), which is what makes the `Slider`
  operable from `tests/renderer/` with no pointer and no layout. Wrong ⇒ VT-2's
  slider half moves to the loop tier, which is a **STOP** (S-2).
- **A-e.** Removing `FieldForm::Number` breaks exactly the sites
  `grep -rn "FieldForm::Number" crates/` names. Derived by grepping at T-3, not
  expected.
- **A-f.** `FieldValue.number` has to be written for a **numeric text** field
  as well as for a `Slider`, because EX-6's exception asks whether *the held
  number is zero* and that is the slot it asks. That is in tension with VA-2
  and with §5.2's *`FieldValue.number` … exist for the `Slider` alone*; the
  tension is **dissolved by EX-7 if the exception goes**, and is a STOP (S-5)
  if it stays. Raised with the orchestrator at T-2 rather than decided here.

**STOP conditions**

- **S-1.** A criterion compelling a file the Surfaces line does not name —
  `app.slint`, `view_model.rs`, `glass.rs`, `install.rs`,
  `tests/renderer/{fields,mapper,wiring,reception}.rs`,
  `tests/event_loop_numeric_guard/`, `Cargo.toml`'s one `[[test]]` entry.
- **S-2.** A `Slider` or a numeric `LineEdit` that cannot be operated under
  `init_no_event_loop`, moving VT-2 … VT-5 to a loop target. That is S-1 as
  well, and a second new arrangement.
- **S-3.** Any pressure to make the host reason from a class
  `input-type: decimal` appears to enforce — a repair rule, a pre-parse, a
  validation in the callback. Ruled out by D23 and measured wrong by VT-4.
- **S-4.** A second site that chooses a `number`'s control, or a markup `if`
  that branches on a fact about the field's bounds rather than on
  `field.slider`. D17.
- **S-5.** EX-7 keeping the exception, which leaves a non-`Slider` `f64` → 
  `float` crossing that VA-2's sentence does not admit (A-f).
- **S-6.** Weakening, deleting or `#[ignore]`-ing an existing case to go green;
  a dependency addition.

**Tasks**

- [x] T-0 baseline: `just check`, gate total, `cargo test --workspace`, each
      with its denominator
- [x] T-1 the sheet, committed before any production code
- [x] T-2 `app.slint`: the two structs' new slots, the `Slider` arm, the
      numeric `LineEdit` arm and its guard — and A-a, A-b settled at the
      compiler; A-f raised (EX-4, EX-5, EX-6, EX-9)
- [x] T-3 `view_model.rs`: `FieldKind::Number` moves to `Ok`, `FieldForm` loses
      `Number`, `slider_bounds` and the one narrowing site — with VT-1's units
      written red first (EX-2, EX-3)
- [x] T-4 `glass.rs`: the `Adjusted` value arm and the row's four new slots
      (EX-2, EX-3)
- [x] T-5 `install.rs`: the `Kind::Number` arm, selecting on `edit.slider` and
      on nothing else (EX-9)
- [x] T-6 EX-8, the fixture migration, one site at a time, recorded by name
- [x] T-7 VT-2 … VT-5 in `fields.rs`, each with an injection pass naming the
      assertion **ordinal and message** it fails at
- [x] T-8 VT-6: `event_loop_numeric_guard`, negative-controlled — then EX-7,
      the same case re-run with the exception removed, both counts recorded
- [x] T-9 VA-1 and VA-2 in writing
- [x] T-10 `just check` exits 0 (EX-1); sheet and Harvest updated

**Result.** `just check` **exit 0**, gate total **591**; `cargo test
--workspace` **556**. Both are +7 on the baseline, and the +7 is three targets:
`goad` lib **52 → 54** (VT-1's two units), `tests/renderer` **198 → 202**
(VT-2 … VT-5), and one new loop target at **1** (VT-6). The gate total stays
exactly 35 above the workspace one. `grep -c '^\[\[test\]\]'
crates/goad/Cargo.toml` is **7** — one no-loop target and **six** loop-tier
arrangements.

**EX-7 — the exception is removed, and the measurement says more than *dead***

Four corners, `cargo test -p goad --test event_loop_numeric_guard`, denominator
**1** each. The negative control compiles and runs on both sides.

| overlay | exception | result | where |
|---|---|---|---|
| on | **off** — the shipped tree | **1 passed** | — |
| on | on | 1 failed | assertion **#9**, *an edit the host never recorded is corrected on the next present, even though the widget is empty and the held number is zero* — `Reading { shown: "", also_shown: "", drafted: Some(0.0), inits: 2, reasserts: 0 }` |
| off | off | 1 failed | assertion **#3**, *a present inside the debounce window leaves the cleared field cleared* — `shown: "0"`, `reasserts: 1` |
| off | on | 1 failed | assertion **#9**, identically to row 2 |

Row 3 is the negative control: it reproduces the exact defect the exception was
licensed by, so the case can see a write-back at all.

**Row 2 is the finding, and it is not the one the design predicted.** The design
expected the overlay to make the exception *unnecessary* — a cleared field is a
pending `AdjustedText("")`, `interpret` reads it as the text `""` beside the
number the field already held, the channel carries `""`, and the strings agree
on their own. That is true, and it is not the whole story. The one state the
exception still **fires** in is the state the guard exists for: a cleared entry
the host *never recorded*, where the channel holds the field's own zero and the
widget is empty. Carrying it suppresses exactly the convergence AC-6 requires.
Row 4 shows that is true with or without the overlay — pre-overlay the price was
simply invisible, because nothing in this project measured AC-6 for a **numeric**
field.

**The first draft of VT-6 could not have found that.** It carried only the
cleared-field claim, and it passed in row 1 **and** row 2 — it could not tell
the exception's presence from its absence, and would have licensed removing it
on no evidence. The second claim, on a second field cleared after the stepper
goes silent, is what separates them.

**Two corrections to my own injections, both caught by running rather than by
reading.** The first *overlay off* injection was `drafted.or(overlay)`, which
falls back to the overlay precisely when `drafted` is `None` — the only case
that matters. It reported *1 passed*, and *the defect does not reproduce* was
one keystroke from being written down. The first *restore the exception*
injection matched **two** guards, the text `LineEdit`'s included; it asserted
its match count rather than editing both silently.

**EX-3's three clauses, and what subsumes what**

`slider_bounds` is four assertions in `view_model.rs`'s inline `mod tests`, one
per clause, and each isolates:

| injection | counts | which assertion, and its message |
|---|---|---|
| — none | 54 passed | — |
| `exact_f32` narrows without the round trip | 52 passed, **2** failed | `slider_bounds_…` **#3** *`0.1` is not an `f32`…* — `Some((0.1, 10.0))` against `None`; and `only_an_f64_…` **#4** *a legal `R-17` bound `f32` reads as infinity* — `Some(inf)` |
| the span clause deleted | 53 passed, **1** failed | `slider_bounds_…` **#5** *both endpoints round-trip exactly and the span is still an `f32` infinity* — `Some((-3.4028235e38, 3.4028235e38))` |
| the move clause replaced by `step > 0.0` | 53 passed, **1** failed | `slider_bounds_…` **#6** *a finite, strictly positive step can still be too small to move the value* — `Some((1.2676506e30, 1.2676508e30))` |

**The third clause subsumes the positivity test and not the span test**, and the
distinction is measured rather than reasoned. Replacing *moves the value* with
`step > 0.0` admits the `2^100` range, so the move test is strictly stronger
than the positivity test it replaces. But deleting the **span** clause admits
`[-f32::MAX, f32::MAX]`, because an infinite step *does* move the value in both
directions — so clause 2 is not subsumed, and it is evaluated first for the
separate reason that it is what makes the division safe to perform at all.

The `2^100` case is real and its arithmetic was measured before the code was
written: `2^100` is exact in `f32`, the next `f32` above it is `2^100 + 2^77`,
so a one-ulp span gives a step of about `2^70.3` — below half an ulp, and
`minimum + step` rounds back to `minimum`.

**One clause the *element* tier could not see, and now can.** Deleting clauses 2
and 3 outright first left `tests/renderer` **202 passed, 0 failed**: no fixture
had a range they refuse. The fixture gained `frozen`, a one-ulp span at `2^100`
whose bounds round-trip exactly and whose span is finite and positive, and the
same injection now reddens VT-2 **#1** with `(Slider, TextInput, Slider,
Checkbox)`.

**The injection pass, `tests/renderer`**

Denominator **202**. Each injection was applied from a copy taken first, run,
reverted from that copy, and the tree re-run green before the next. Assertion
**ordinals** are per case, counting `assert!` / `assert_eq!` in source order.

| # | injection | result | which assertion, and its message |
|---|---|---|---|
| — | none | **202 passed, 0 failed** | — |
| **I-1** | `install.rs` ignores `FieldEdit.slider` — every `number` reports as text | 201 / **1** | VT-2 **#8** *R-57: … which only reaches the draft if the control reported itself as a slider*, `Number(0.0)` against `Number(7.0)` |
| **I-2** | the `Slider` writes `slider: false` | 201 / **1** | VT-2 **#8**, identically. D-38's own injection |
| **I-3** | the numeric `LineEdit` writes `slider: true` | 200 / **2** | VT-4 **#2** *the text is recorded verbatim…*, `"0"` against `"12/25"`; VT-3 **#3** *R-57: a JSON number, unclamped and unrounded*, `Number(0.0)` against `Number(-4.5)` |
| **I-4** | `slider_bounds` drops clauses 2 and 3 | 201 / **1** | VT-2 **#1** *one kind, two controls*, `(Slider, TextInput, Slider, Checkbox)`. Green before `frozen` existed — see above |
| **I-4b** | a missing bound is filled in by the host | 199 / **3** | VT-3 **#1** *a `number` the backend sent no bound for acquires none on the way to the screen*, `(Some(0.0), Some(100.0))`; VT-4 **#2**; VT-2 **#1** |
| **I-5** | `interpret` repairs one foreign character with a dot | 201 / **1** | VT-4 **#4** *not `12.25`, which is what a one-character repair would have read*, `Number(12.25)` against `Number(3.0)`. The cleanest isolation of the pass |
| **I-6** | `interpret` discards the text it cannot parse | 201 / **1** | VT-4 **#2**, `"3"` against `"12/25"` |
| **I-7** | `spelled` never switches to `{:e}` | 201 / **1** | VT-5 **#1** *the `{:e}` spelling, not the 309 characters `Display` would have written* — and the 309 characters are in the failure output |
| **I-8** | every untouched field is shown as an untouched **text** field | 196 / **6** | VT-4 **#1** *the field is drawn showing its declared minimum*, `""` against `"3"`; VT-5 **#1**, `""`. Plus PHASE-07's four `datetime` cases — the broadest injection of the pass, and the one that says `untouched` is one rule and not five |
| **I-9** | the markup computes its own step, a tenth of the span | 201 / **1** | VT-2 **#4** *one keyboard step is a hundredth of the declared span*, `"1"` against `"0.1"` |
| **I-10** | the row ships Slint's own default bounds | 201 / **1** | VT-2 **#2** *the slider declares the range the backend sent, and neither bound is one the host chose*, `(Some(0.0), Some(100.0))` |

**Trap 10 in this phase: `accessible-value-step` cannot measure the step this
host ships.** Slint binds it to `min(root.step, (maximum - minimum) / 100)`
(`fluent/slider.slint:29`) — a **cap** — and this design's step is exactly that
hundredth, so the reading stays `0.1` whatever the markup ships. VT-2's first
draft asserted it and would have passed against a slider left at Slint's default
step of `1`. I-9 is the measurement: with a step of a *tenth* of the span, the
reported step is still `0.1` and the value moves by `1`. VT-2 now takes one
`invoke_accessible_increment_action` and reads where the value lands, which
measures the shipped step and the third clause of `slider_bounds` in the same
call.

**EX-9 / D-38 — the evidence that each control writes its own literal**

Three readings and two injections, and none of them is the grep alone:

- `grep -n "slider:" crates/goad/ui/app.slint` returns **four** lines: `:54`
  and `:89`, the two struct declarations, then **`:581` `slider: true`** inside
  the `Slider`'s `changed` handler and **`:650` `slider: false`** inside the
  numeric `LineEdit`'s `edited` handler. There is no fifth, so no third value
  is written anywhere.
- `grep -n "field.slider" crates/goad/ui/app.slint` returns **five** lines, of
  which `:38`, `:85` and `:501` are prose. The two code lines are `:521` and
  `:585`, and both are the `if` that selects which control is **drawn**.
  Neither *report* reads the row.
- `grep -n "edit.slider" crates/goad/src/install.rs` returns **one** line,
  `:193`, and it is the guard on the `Kind::Number` arm. Nothing else in the
  host reads it.
- I-1 and I-2 are the measurement. Both take VT-2 from 202 to 201 at **#8**,
  with the slider's `7` never reaching the draft.

**What no test can measure, stated rather than claimed.** Writing
`slider: field.slider` at each control would leave every case in this phase
green, because for a correct renderer the row and the control agree. That is
D-38's whole point: the literal is a *witness*, and its value is that it can
disagree. The injections above are of the literal being **wrong**, which is the
only form of the defect a test can see.

**EX-8 — the fixture migration, by name.** Every one moved to **`choice`**,
which is the last undrawn kind: PHASE-09 draws it, so nothing on this list can
be repaired a fourth time and each case is deleted there instead. Each
fixture's own doc comment was advanced by one phase in the same edit.

| site | file | was | now |
|---|---|---|---|
| `A_DRAWN_AND_AN_UNDRAWN_FIELD` | `tests/renderer/fields.rs:90` | `noted`, `number` | `noted`, **`choice`** |
| `mod editing`'s `TWO_FORMS` | `tests/renderer/wiring.rs:1165` | `noted`, `number` | `noted`, **`choice`** |
| `grouped_fields_separated_only_by_an_undrawn_field_are_one_block` | `tests/renderer/mapper.rs:240` | `note`, `number` | `note`, **`choice`** |
| `a_group_whose_every_field_is_undrawn_produces_no_block` | `tests/renderer/mapper.rs:259`, `:261` | `note` and `other`, `number` | both **`choice`** |
| `every_undrawn_kind_is_reported_by_option_field_and_form` | `tests/renderer/mapper.rs:314`, its row at `:316` | two undrawn kinds | **one** — the `number` row is **removed**, not moved; `choice` was already in the list, so a move would have duplicated a row. The `undrawn.len()` assertion goes 2 → 1 with it, which is what keeps *every* a claim rather than a word |
| `a_field_that_is_both_undrawn_and_badly_grouped_is_reported_twice` | `tests/renderer/mapper.rs:385` | `note`, `number` | `note`, **`choice`** |
| `field_reports_leave_a_parsed_body_undegraded` | `tests/renderer/mapper.rs:432` | `note`, `number` | `note`, **`choice`** |
| `a_view_carrying_an_undrawn_field_reaches_the_diagnostic_surface_through_receive` | `tests/renderer/reception.rs:762` | `note`, `number` | `note`, **`choice`** |
| `field_form_displays_as_the_protocols_own_word` | `tests/renderer/mapper.rs:205` | two rows | **one** — `number`'s row goes with the variant |

Each case was read for what its undrawn field is *for* rather than for the
token, and none went vacuous: `fields.rs`'s asserts a diagnostic line naming the
field **and** its absence from the submitted keys; `wiring.rs`'s opens with a
guard assertion that the fixture really does carry an undrawn field;
`mapper.rs`'s block cases assert a claim about *undrawn*, not about the kind;
`reception.rs`'s asserts two lines in order, one per field. The two that changed
**meaning** rather than spelling are the two enumerations, and both are down to
one row.

**Trap 11 — and this time the instrument died during the phase, not after it.**
`grep -rn '"kind":"number"' crates/` now returns **eight** lines and not one of
them is an undrawn fixture: two normalizer-refusal fixtures in
`tests/renderer/table.rs` (`OPTIONS_ON_A_NUMBER_FIELD`, `INVERTED_BOUNDS`), the
same two in `crates/goad-shell/tests/integration/failure_matrix.rs`, three
**drawn** fixtures this phase added to `fields.rs`, and one in
`event_loop_numeric_guard`. The four refusal fixtures were checked again and
left: they are `retained(…)` rows asserting a normalizer diagnostic, refused
before `present` is ever called, so no drawn kind reaches them.

The instrument that replaces the grep is not a grep. `FieldForm` now declares
**`Choice` and nothing else**, so a `number` field *cannot be reported undrawn
at all* — `grep -n "pub enum FieldForm" -A 3 crates/goad/src/view_model.rs` is
the reading, and it is a property of the type rather than of a search string.

**VA-1 — nothing non-finite can reach the wire**

`draft.rs:202` is the only site that maps a number onto the wire:
`Edited::Adjusted { number, .. } => serde_json::Value::from(number.get())`. The
field is private and the constructor fallible, so `Value::from(f64)` — which
answers JSON `null` for a non-finite, and `R-57` admits no `null` — can never
see one.

`grep -rn "Finite::new\|Finite::ZERO" crates/goad/src/` returns **three**
production sites and the rest are test fixtures:

- `view_model.rs:594`, `drawn_number` — `R-17` already guarantees a declared
  bound is finite, so this cannot refuse one; the `unwrap_or(Finite::ZERO)` is a
  total expression rather than an argument about an unreachable `expect`.
- `view_model.rs:753`, `interpret`'s `AdjustedText` arm — `Finite::new` on the
  parse, which is what makes `1e400` and `inf` leave the number alone.
- `view_model.rs:762`, `interpret`'s `AdjustedValue` arm —
  `Finite::new(f64::from(*value)).map(adjusted)`, refusing a non-finite
  **before** any `Edited` exists.

Neither boundary type holds the invariant and neither is asked to:
`Reported::AdjustedValue` is a bare `f32` and `FieldEdit.number` is a Slint
`float`. That is the design's own division — the refusal belongs where the other
renderer-bug refusals are, not inside a Slint closure with nothing to report to.

**A `Slider` cannot in fact produce a non-finite, and that was measured rather
than assumed.** `set_accessible_value("NaN")` on the drawn slider leaves the
widget showing `"10"` and the channel holding `10.0`: Slint's
`accessible-action-set-value` parses, and `set-value` clamps with
`max(minimum, min(maximum, v))` before raising `changed`, so the host is handed
the maximum. `"inf"` clamps the same way. So `interpret`'s non-finite refusal is
a guard against a renderer bug and not against this renderer — which is what
`VA-1`'s *neither of them the boundary type* is for.

**VA-2 — no `f64` crosses as a `float` except a `Slider`'s value and its bounds**

Held **by construction**, not by argument, and the reading is four greps:

- `grep -rn " as f32\| as f64" crates/*/src/` returns **one** line,
  `view_model.rs:502`, inside `exact_f32`, whose next line is the round-trip
  check. `as_conversions`, `cast_possible_truncation` and `float_cmp` are all
  `deny` workspace-wide and one `#[expect]` names why exact equality is the
  whole of the check.
- The markup declares five `float` slots: `FieldRow.minimum` / `maximum` /
  `step` (`app.slint:54`), `FieldValue.number` (`:55`) and `FieldEdit.number`
  (`:89`).
- Every **read** of one is inside the `Slider` arm:
  `grep -n "values\[field.slot\].number" crates/goad/ui/app.slint` returns
  `:539`, `:558`, `:559` — the binding and the guard — and `field.minimum` /
  `maximum` / `step` are read at `:532-534`. The numeric `LineEdit` reads
  `text` and nothing else.
- Every **write** of one is in `glass.rs`: `:333-335` for the three row slots,
  each from `slider_bounds`'s already-`f32` answer and `slider_step`'s pure
  `f32` arithmetic, and `:539` for `FieldValue.number`, which is
  `slider.and_then(|_| exact_f32(number.get()))` — so it is written **only**
  where a slider is drawn, and only through the checked narrowing.

Inbound, `FieldEdit.number` is an `f32` by nature — the `Slider`'s own value —
and `f64::from` widens it losslessly at `view_model.rs:762`. The two `LineEdit`
controls send `text`, which is lossless for every finite `f64`.

**That the `number` slot is the `Slider`'s alone is EX-7's doing.** While the
guard's exception stood, the numeric text control's guard read
`values[field.slot].number` to ask whether the held number was zero — a
non-`Slider` read of a `float` slot that VA-2's sentence does not admit, and a
narrowing that would have turned a legal `min: 1e100` into an `f32` infinity.
That was raised as **A-f** before any test was written and it is gone with the
exception; nothing had to be decided.

**PHASE-06/EX-3's unmeasured branch — not closed, and now measured as
unreachable**

The overlay's clause *"where `interpret` refuses the entry the draft's value
stands"* is still discharged by construction and measured by nothing.
`number` was the best candidate and it does not reach it either. The three ways
`interpret` can answer `None` for a `number`, each checked:

1. **a non-finite `AdjustedValue`** — `set-value` clamps into the drawn range
   before raising `changed`, measured above, and `slider_bounds` guarantees that
   range is finite. Unreachable from the markup.
2. **a `Chosen` index** — belongs to `choice`, which does not draw yet.
3. **a variant that does not match the drawn kind** — both `number` controls
   write `kind: Kind.number`, and `install::reported` selects `AdjustedText` or
   `AdjustedValue` on `edit.slider`. Both are in-kind for `DrawnKind::Number`.
   A stale entry cannot supply one either: `overlaid` tests `entry.view == view`
   before it interprets, so an entry made on a replaced view is never shown.

So the branch stays open for the audit, and what has changed is that it is now
*measured* unreachable through the markup for four of the five drawn kinds
rather than merely unexercised. `choice` is the last candidate and PHASE-09
owns it: a `ComboBox` index out of range is case 1 of the `None` surface and is
the one a renderer could plausibly produce.

**Trap 14 — the Surfaces check, and it was not short**

Files earlier phases of **this slice** created, each grepped for
`number|slider|FieldForm`: `src/instant.rs`, `src/pending.rs`,
`tests/event_loop_debounce/`, `tests/event_loop_overlay/` and
`tests/event_loop_reassert/`. Three hits, all prose — `pending.rs:11`, a doc
sentence naming which kinds are debounced and already correct for this phase;
`instant.rs:195` and `reassert.rs:42`, the English word *number*. PHASE-07's new
fixtures were grepped too and `TWO_DATETIME_FIELDS` names no numeric field.
`goad-boundary` names none of `FieldRow`, `slider`, `markup_kind` or
`app.slint`.

What the check did **not** catch, and what did: `grep -rn "FieldRow {" crates/`
returns **four** lines — the declaration at `ui/app.slint:54` and three
constructors, `src/glass.rs:323`, `tests/renderer/tree.rs:72` and
`tests/renderer/sizing.rs:63`. Two of the three are outside the Surfaces line. They are
compelled by `FieldRow` gaining four slots, not by any kind, so a grep for the
phase's own vocabulary could never have found them. The lesson is the type, not
the token: **a phase that widens a shared struct should grep for its
constructors, not for its subject.**

**STOP conditions raised**

1. **S-1, `sizing.rs` and `tree.rs`.** Sent as soon as the compile named them,
   with the edit already made and the reason given: without it the renderer
   target does not build at all, so every remaining task is blocked behind the
   reply and the fallback is handing back a tree that does not compile. The
   orchestrator amended the Surfaces at `660943e`, took
   `..FieldRow::default()`, recorded its cost, and wrote the carve-out into the
   brief. While waiting, `view_model.rs`, `glass.rs`, `install.rs` and the
   nine-site migration were all done.
2. **A-f, raised before any production code and resolved by measurement.** Not
   a STOP in the end: EX-7 removed the exception and the tension went with it.
   It was raised anyway, because if the measurement had gone the other way the
   decision would have been the orchestrator's and not mine.

**S-2 … S-6 did not arise.** Both controls are operable under
`init_no_event_loop` — `set_accessible_value` reaches `set-value` on a `Slider`
and `edited` on a `LineEdit`, neither through a pointer — so no row moved tier.
No repair rule was written. `slider_bounds` is the only site that chooses, and
the markup's two `if`s branch on `field.slider` alone. No dependency was added
and no case was weakened.

**Trap 17 — checked before the seventh target was added, and the honest answer
is not the flattering one.** `grep -c '^\[\[test\]\]' crates/goad/Cargo.toml`
was **6** and `grep -c '^#\[test\]' crates/goad/tests/event_loop*/*.rs` returns
exactly **one** per loop target. `event_loop_reassert` presents a glass directly
with no `install`, no channel and no debounce, so it cannot hold a pending entry
at all. But `event_loop_overlay` **does** have the arrangement VT-6 needs, and
saying otherwise would be false. What makes this a target rather than a case is
that EX-7 needs one `#[test]` fn run four times with production changes between
it, and `event_loop_overlay`'s single function also carries PHASE-06's claims
and deliberately goes silent partway to measure a *dropped* edit — merged, a
failure on any corner could not be attributed to one phase's claim. That
reasoning is in the new target's module doc rather than left implicit.

**Decisions taken during execution**

- **`view_model::untouched`, and `glass.rs` still never calls `as_drawn`.** The
  glass's untouched-field arm was wrong for `number` the moment `number` drew:
  its doc claimed a defaulted slot is what a field was drawn showing, true for
  four kinds and false for one, so a field declaring `min: 2.5` would have drawn
  an empty box and submitted `2.5`. The repair states the screen's half of the
  rule as a function beside the wire's half, and it **deletes** the `kind`
  parameter PHASE-07 added to `field_value`. Named `untouched` on the
  orchestrator's decision: `drawn` would have been a prefix of `as_drawn` and
  the sixth `drawn`-rooted identifier in the module, and `view_model.rs` already
  binds a local `drawn` in `sift` and `present`.
- **`slider_step` is a named function, not an expression written twice.**
  `slider_bounds` *checks* the step and `glass.rs` *ships* it. A second spelling
  is a second thing that can drift from the one that did the proving, and I-9
  shows the drift is silent at the accessibility surface.
- **`exact_f32` is `pub` and is the crate's only cast.** Two callers —
  `slider_bounds`'s first clause and `field_value`'s `number` slot — and one
  `#[expect]` covering three denied lints. Making the narrowing a *function with
  a fallible return* is what turns VA-2 from a claim into a signature: a caller
  has to say what it does with `None`.
- **VT-4 reads the value channel as well as the wire.** A host that discarded
  the refused entry and one that repaired it are each invisible to one of the
  two, measured as I-5 and I-6 reddening different assertions. It is drawn at
  its declared minimum rather than given a delivered edit because an answer with
  no new view is `Shift::Closed` and takes the form down, so a second press is
  not available in this target.

**Left for PHASE-09 and the audit**

- **Design drift, and it is real rather than cosmetic.** `design.md` §5.2's
  comparand table and §7 D13 both still state the numeric guard's exception,
  which EX-7 removed. §9 A-2 framed the outcome as open, so this is the audit's
  to reconcile; no design file was touched.
- **PHASE-09 deletes rather than migrates, and should not go looking for a
  fourth hop.** `every_undrawn_kind_is_reported_by_option_field_and_form`
  (`mapper.rs:314`) and `field_form_displays_as_the_protocols_own_word`
  (`mapper.rs:205`) are both down to **one** row, and `FieldForm` to one
  variant. There is no undrawn kind left to report and an empty enum has no
  `Display` to assert. The other seven sites on EX-8's list carry `choice` and
  have nowhere to go either.
- **`FieldRow` gains `alternatives` at PHASE-09**, which compels
  `tests/renderer/sizing.rs` and `tests/renderer/tree.rs` again — except that
  both now take `..FieldRow::default()`, so they will **not** break. That is the
  cost the orchestrator recorded at `660943e`, and it is the phase agent's to
  remember rather than the compiler's to raise.
- **PHASE-06/EX-3's refusal branch is `choice`'s last chance.** A `ComboBox`
  index no alternative has is case 1 of `interpret`'s `None` surface and is the
  one a renderer could plausibly produce. If PHASE-09 does not reach it, nothing
  will.
- **`role_of`, `range_on_screen`, `slide_to` and `step_once` are `fields.rs`'s
  alone** and were deliberately not lifted into `harness.rs`: the module rule
  there is *two or more case files need it*, and one does.


## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-19 · PHASE-08 done · see §Status

### Produced
<!-- What now exists: modules, contracts, docs. -->

- **`number` draws two controls and one function chooses between them.**
  `view_model::slider_bounds(&NumberRange) -> Option<(f32, f32)>` is the only
  site that picks a `number`'s control, on three clauses evaluated in `f32`:
  both bounds `f32`-exact, a finite strictly positive span, and a step that
  **moves the value**. `FieldRow` carries `slider` plus `minimum` / `maximum` /
  `step`, and the markup obeys the bool rather than reasoning from the bounds
  (§7 D17). The text control is the one that always works; the `Slider` is the
  one with an admissibility condition.
- **`view_model::exact_f32(f64) -> Option<f32>`** — the crate's **only** cast
  (`grep -rn " as f32\| as f64" crates/*/src/` returns one line), checked on
  the next line by the round trip, with one `#[expect]` covering
  `as_conversions`, `cast_possible_truncation` and `float_cmp`. Two callers:
  `slider_bounds`'s first clause, and the one `float` slot that leaves the
  host. That is what makes VA-2 a signature rather than a comment — a caller
  has to say what it does with `None`.
- **`view_model::untouched(&DrawnKind) -> Option<Edited>`** — what an untouched
  field **shows**, beside `as_drawn`, which is what it is **worth**. `Some` for
  four kinds, `None` for `datetime` alone. D-6's divergence now has one
  statement, in the module that owns every other kind-directed rule, and
  `glass.rs` still never calls `as_drawn`. It **deleted** the `kind` parameter
  PHASE-07 added to `field_value`.
- **`view_model::slider_step(f32, f32) -> f32`** — a hundredth of the span,
  named because `slider_bounds` *checks* this step and `glass.rs` *ships* it.
- **`crates/goad/tests/event_loop_numeric_guard/`** — the **seventh**
  `[[test]]` target. Two contrasting claims in one fn: a cleared field the host
  has not recorded *yet*, and a cleared field it did *not* record. The second is
  what keeps the guard's exception deleted rather than merely absent.
- **`fields.rs`'s numeric vocabulary** — `role_of` (the control's identity, by
  role), `range_on_screen` (the bounds the **widget** declares), `slide_to` and
  `step_once`. All `fields.rs`'s: the `harness.rs` rule is *two or more case
  files need it*, and one does.
- **`FieldForm` is down to `Choice`.** A `number` field can no longer be
  reported undrawn at all — which is a better instrument than any grep for what
  used to rest on one.

- **The value channel is the draft overlaid with `pending.rs`** —
  `glass.rs::overlaid`, I-H's third site: an entry is shown only against the
  view it was made on, and only through `interpret`, so there is no second
  `Reported` → `FieldValue` mapping. Where `interpret` refuses, the draft
  stands. Kind-agnostic, so PHASE-07 and PHASE-08 gain it by writing their
  value arm and nothing else.
- **`SlintGlass` holds a clone of `install`'s `Debounce`**, and six call sites
  each hold one value. `harness.rs::glass_overlaying` is the entry point that
  shares it; `glass_over` delegates to it with a fresh empty handle and its doc
  says which a case that calls `install` must take.
- **`crates/goad/tests/event_loop_overlay/`** — the sixth `[[test]]` target.
  Two contrasting claims in one fn, with the run's schedule written as one
  commented timeline above the `match`. Its four readings each carry screen,
  draft and both counters, because neither half implies the other.
- **`src/pending.rs`** — `Debounce`: a `BTreeMap<(option, field), Held>`, one
  `slint::Timer`, and nothing else. Every entry carries the view it was made on
  (I-H). Two exits, deliberately asymmetrical: an entry leaves on the
  **enqueue**, so a `Full` send clears nothing. `Debug` by hand — `slint::Timer`
  has none — reporting keys and views but never a person's typing.
- **`Wire::send -> bool`**, and `Command::Choose { view, option, edits }`
  carrying the flush in **one** send, because a capacity-1 channel plus a
  synchronous Slint callback makes the second `try_send` of any flush `Full`
  always rather than sometimes.
- **`drawn_form(&FieldKind) -> Result<DrawnKind, FieldForm>`**, replacing
  `undrawn_form`. One match, one arm per kind, no unreachable arm; each later
  phase moves one arm `Err` → `Ok`. AC-7's mechanism is the exhaustive match and
  is identifier-free (`design.md:149-151`). `plan-log.md` carries the argument
  and the process correction.
- **`crates/goad/tests/event_loop_debounce/`** — the fifth `[[test]]` target and
  the arrangement that proves the timer re-arms. Its stepper drains on a 25 ms
  timer against a 150 ms debounce, which is why nothing drains *inside* a tick
  callback.
- **`wiring.rs::refusal_lines`** — the refusals among the reported lines, which
  is not the question `reported_lines` answers. Use it from PHASE-07 on.
- **`diagnostics.rs`'s undrawn line names no subset** of the drawn kinds, so it
  does not go stale again as PHASE-07, -08 and -09 each draw one more.

- **`datetime` draws.** `app.slint` carries one `DatePickerPopup` and one
  `TimePickerPopup` as **root singletons** — a `PopupWindow` can be neither
  repeated nor made conditional — plus six root properties: two seeds each
  popup **binds** to at its own declaration site, `picked-date` (the stash
  between the two halves of one pick), and the three `picking-` strings the
  completed pick is reported under. A `datetime` field draws a `Button` and no
  inline control, and the button needs no guard: it never assigns its own text.
- **`FieldValue` and `FieldEdit` carry `date` and `time`.** On the value side
  they are the **seed** — what this field's picker opens on, written every
  present from `instant::decompose` of the draft's `Picked` or from one
  `instant::today_local()` read per present for a field nobody has picked. On
  the edit side they are the civil pair the **time** picker's `accepted`
  reports, which `install.rs` resolves through `instant::compose`.
- **`glass.rs::NOT_SET`** — the screen's half of D-6. A private constant with
  one reader; `fields.rs` restates the literal deliberately, so a case cannot
  agree with the markup by construction.
- **`fields.rs`'s picker vocabulary** — `only_button` (exactly one match,
  asserted, across the window *and* its `active_popups`), `accept`, `cancel`,
  `pick_day`, `pick_hour`, `day_selected`, `hour_shown`, and `settled!`, the
  round trip a case asserting an **absence** needs. All `fields.rs`'s: the
  `harness.rs` rule is *two or more case files need it*, and one does.

- **`src/instant.rs`** — `compose(&Date, &Time) -> Option<(Timestamp, Offset)>`,
  `decompose(Timestamp, Offset) -> (Date, Time)`, `today_local() -> (Date, Time)`,
  plus two private zone-parameterised halves, `composed_in` and
  `local_midnight`, that exist so the units can name a zone this machine is not
  in. The crate's only reader of the clock or the system zone; nine units.
- **`jiff`'s `tz-system` and `tzdb-zoneinfo`**, on `crates/goad` alone
  (`Cargo.toml:38`), with `POL-001`'s residue argument at the manifest pointing
  at `design.md` §10 rather than restating it.
- **`Date` and `Time` reach Rust by *reachability*, and the export line is
  gone.** PHASE-07 re-measured what PHASE-04 could not: with `FieldValue` and
  `FieldEdit` carrying `date: Date, time: Time`, the generated `app.rs` emits
  `r#Date` and `r#Time` with no `export { … } from "std-widgets.slint"` line at
  all. An `import` alone still does not emit them (PHASE-04's four
  measurements); a struct reached from an exported struct does.
- **`clock.rs`'s doc states its three reaches** (`:47-72`) — the workspace
  build, where its rationale expires; the builds that exclude `crates/goad`,
  where the workaround still binds; and `cargo test -p goad-semantics`, which
  builds neither stratum above stratum 1.

- **The two channels.** `ui/app.slint` carries `Kind`, `FieldRow` and
  `FieldValue`, with `values` and `epoch` on the window root. Both structs have
  grown a phase at a time — `FieldRow` gained `slider`, `minimum`, `maximum`
  and `step` at PHASE-08 and gains `alternatives` at PHASE-09; `FieldValue`
  gained `date` and `time` at PHASE-07 — so read the declarations rather than a
  list written here. `glass.rs::option_models` builds both in one pass, so I-B is
  a property of the construction; `present` writes values → rows (on a changed
  `view_id` only) → epoch, which is I-F.
- **Two instrument counters in production markup**, `inits` and `reasserts`,
  `out property <int>` on the window root (§7 D15).
- **The `CheckBox`'s guard**, the first of the five §5.2's comparand table
  names.
- **`Glass::present`'s second deliberate exception**, stated in the trait doc
  with §5.3's argument.
- **`crates/goad/tests/event_loop_reassert/`** — the fourth `[[test]]` target
  and the **first loop-tier arrangement that presents a glass directly**: a
  real loop, `init_integration_test_with_system_time()`, no runtime, no
  channel, no `serve`, and a repeated `slint::Timer` stepping present → read →
  present → read. Three later phases need this shape (AC-6, the `choice`
  re-assert, the guard exception); it is the thing to copy, and a fifth target
  is what a *different* arrangement costs, not a second case.
- **`harness::slot_of` / `harness::value_of`** — the join across the two
  channels, which is now the only honest way to ask the window what a field
  holds without going to the screen.
- **The five values.** `draft.rs` carries `Finite` (private field, fallible
  constructor, **no `Eq`**), `Edited`'s five variants and `Reported`'s six.
  `state_of` answers an `Option`; `submitted` has five arms and is still the
  one application of `R-57` (I-C). `wire.rs`'s `Command` drops `Eq` with them.
- **The kind-directed pure functions**, in `view_model.rs` beside the mapper:
  `spelled` (the number format rule), `as_drawn` (what an untouched field is
  **worth**), `untouched` (what it **shows**, PHASE-08) and `interpret` (what a
  widget's report becomes, against the drawn field and what the host already
  holds). Every one is total over `DrawnKind` with no `_` arm anywhere, and
  every one is unit covered with an injection pass.
- **`DrawnKind`** — the host-local drawn half of the canonical `FieldKind`,
  carried on `PresentationField`. `Choice` carries the first alternative's id
  beside the list, which is what makes `as_drawn` total under a lint table that
  denies `unwrap_used`, `expect_used` and `indexing_slicing`.
- **`glass.rs::markup_kind`** — the drawn kind as the markup's discriminant,
  total over `DrawnKind`. `FieldRow.kind` is now read off the field the mapper
  drew rather than asserted as a constant, which was PHASE-01's noted debt.
- **The edit channel.** `ui/app.slint` carries
  `FieldEdit { kind, checked, text, number, index }` and
  `callback edited(string, string, string, FieldEdit)`; `install.rs::reported`
  maps a `FieldEdit` to a `Reported` and does nothing else; `Command::Edit`
  carries the `Reported`; and `Controller::edit` interprets it against the
  drawn field on the walk it already makes, refusing an uninterpretable report
  with the `Refused::UnknownField` the taxonomy already had. Five hops, one
  judgement, and the judgement is in the only place that holds the
  presentation.

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

- **Slint's `accessible-value-step` is a cap, so it cannot measure the step you
  ship.** `fluent/slider.slint:29` binds it to
  `min(root.step, (maximum - minimum) / 100)`. Any design whose step *is* that
  hundredth gets the hundredth back whatever the markup holds — measured: a
  slider shipping a step of a **tenth** of the span still reports `0.1` and
  still moves the value by `1`. What measures the shipped step is
  `invoke_accessible_increment_action`, because `increment()` is exactly
  `set-value(value + step)`. The general rule: **a reading Slint derives from
  your value is not a reading of your value.**
- **A `Slider` cannot report a non-finite.** `set_accessible_value("NaN")`
  leaves the widget on its **maximum**: `set-value` clamps with
  `max(minimum, min(maximum, v))` before raising `changed`, and `"inf"` clamps
  the same way. So `interpret`'s non-finite refusal guards against a renderer
  bug and not against this renderer — which is worth knowing before writing a
  case that tries to reach it.
- **An injection that only *reorders* a fallback is not an injection.** The
  first *remove the overlay* injection here was `drafted.or(overlay)`, which
  falls back to the overlay precisely when the draft is empty — the only case
  that matters. It reported **1 passed** and looked like evidence that the
  defect does not reproduce. Injecting into an `or` chain means **dropping** a
  term, not moving it.
- **A dead mitigation and a harmful one are not the same finding, and only a
  case can tell them apart.** The numeric guard's exception was expected to be
  *unnecessary* once the value channel was overlaid. It is worse: the one state
  it still fires in is the state the guard exists for, so it suppresses the very
  convergence AC-6 requires. A case asserting only *the thing the mitigation
  protected still works* passes with the mitigation **and** without it and
  licenses nothing; the case has to assert what the mitigation would **break**.
- **A phase that widens a shared struct should grep for its constructors, not
  for its subject.** Trap 14's per-file scan for this phase's vocabulary
  (`number`, `slider`, `FieldForm`) was clean, and two files outside the
  Surfaces still broke — they build a `FieldRow` **by hand** and are compelled
  by the type gaining fields, which no search for a kind could find.
  `grep -rn "FieldRow {" crates/` is the instrument.
- **Taking `..Default::default()` in a hand-built fixture buys silence now and
  costs a phase later.** It is the right trade for a fixture whose subject is
  neither values nor kinds, and the cost is real: the *next* field added to the
  struct will not break those files, so a phase that ought to look at them will
  not be made to. Record it where the next phase reads.
- **An untouched field's display and its submitted value are two rules, and
  four kinds agreeing hides the fifth.** `glass.rs` read `None` as *the default
  slot is what the field was drawn showing* — true for `boolean`, `text`,
  `choice` and `datetime`'s own sentinel, false for `number`, where a field
  declaring `min: 2.5` would have drawn an empty box and submitted `2.5`. A
  coincidence that holds for every case in front of you is not a rule.

- **A test tier with no event loop can still reach inside a popup.**
  `ElementQuery::find_first` / `find_all` pass `active_popups()` into the walk
  (`search_api.rs:291-312`), so a shown `PopupWindow`'s subtree is in scope from
  the window root under `init_no_event_loop` — measured by driving a
  `DatePickerPopup` and a `TimePickerPopup` end to end from
  `tests/renderer/fields.rs`. What §8 R9 doubted was never the *visibility* of
  a popup but `mock_single_click`'s dependence on `absolute_center()`: a popup
  with no geometry is clicked nowhere near. Everything the pickers need —
  a calendar day cell, a clock-face selector, a `StandardButton` — declares
  `accessible-role: button` and an `accessible-action-default` that calls its
  own `clicked`, so `invoke_accessible_default_action` dispatches no pointer
  event and nothing depends on layout. **Absence of a case was again mistaken
  for absence of a capability** (§7 D14 had already found this once for
  `choice`).
- **A case asserting an *absence* has to drive something the host must present
  before it reads.** *Nothing was recorded* cannot be waited for, so reading
  the screen straight after the abandonment asserts only that nothing had
  arrived **yet** — and a defect that did record something then fails two steps
  later as a dropped `Choose` and a timeout, which names nothing. Driving a
  `view: null` round trip first (it folds as `Shift::Retained` and changes
  nothing) turns the same defect into a comparison naming both values. The
  yield before it is the other half: `serve` shares the test thread and the
  command channel holds one, so a wrongly recorded `Command::Edit` is still on
  it when the next send is made.
- **A Slint `Button` declares `accessible-checked`.** It has a `checkable`
  property and binds the attribute whether or not anything set it — so
  *declares no checked state* is a false discriminant that happens to pass
  against a `LineEdit` and says nothing about a `CheckBox`. What tells the three
  drawn controls apart in the tree is the **role**: `button`, `checkbox`,
  `text-input`.
- **`export` and *reachability* are two different paths into the generated
  code, and only the second survives.** PHASE-04 measured that an `import`
  alone does not emit a library struct and added an `export … from` line for
  `Date` and `Time`. Once an **exported** struct carried them as members the
  line became dead: the generator emits a struct reached from an exported one.
  Delete such a line on a measurement of the path you are actually on, not on
  the path someone else measured.
- **A seed and the value it produces cannot be attacked separately.** An
  injection to the clock read an unpicked field's picker opens on looked like
  the control for *opens on today* and failed a different assertion instead —
  because the seed the picker opens on is also the seed the **pick** is made
  from, so a wrong seed changes the recorded value before it changes what
  another field's picker shows. The injection that isolates *this field's seed,
  not that one's* leaves the pick correct and redirects the **slot** the
  handler reads.

- **An injection can redden the wrong claim for a reason worth keeping.**
  Removing `pending.rs`'s enqueue clear looks like the control for *the entry
  has left*, and it fails the **delivery** assertion instead: with nothing
  removed, `tick`'s `.iter().next()` returns the same first key forever and the
  map's second entry is never sent at all. The enqueue rule is load-bearing for
  delivery, not only for display — and an injection that reddens *a* claim is
  not evidence for *the* claim it was aimed at. The one that isolates the
  correction is the guard's own write.
- **A test that stops *recording* is not a test that stops *draining*.** A
  channel nobody drains comes back `Full`, and under the enqueue rule the entry
  then never leaves the map — a different case from *the host did not record
  this*. To reach the second, drain and discard.

- **A Slint `import` is not an `export`, and Rust only sees the exports.**
  `Date` and `Time` are library structs, not builtins; importing them into
  `app.slint` emits nothing into the generated `app.rs`, and
  `export { Date, Time } from "std-widgets.slint";` emits both. Measured four
  ways against `out/app.rs`. The general rule: **a type is nameable from Rust
  only if the root `.slint` file exports it**, directly or by carrying it in an
  exported struct — and the second half of that is a prediction here, not a
  measurement.
- **`jiff::Timestamp::MAX` is `9999-12-30T22:00:00.999999999Z`, a whole day
  below `DateTime::MAX`.** So `DateTime::to_zoned` refuses the top of the civil
  range in **every** zone, `+14:00` included. That makes
  `9999-12-31T23:59:59` a zone-independent witness for the fourth fallible
  step — which matters, because a function that reads the *system* zone cannot
  be tested with a zone-dependent one. The offset-dependence is real one notch
  lower: `9999-12-31T12:00:00` resolves at `Pacific/Kiritimati` and is refused
  at every smaller offset.
- **Zone-parameterise the pure half before asserting anything about a zone.**
  Two functions here read `TimeZone::system()`, and neither behaviour worth
  asserting — DST disambiguation, and a local date differing from UTC's — can
  be reached through the machine's own zone: a CI box set to UTC has no DST and
  no disagreement, and `std::env::set_var` is in `clippy.toml`'s
  `disallowed-methods`. Splitting `composed_in(zone, …)` and
  `local_midnight(zone, …)` out is what makes the units say something true
  rather than something that happened to hold. What is left over is one token
  per function — *that the zone is the system's* — and review is the only thing
  that holds it.
- **An assertion about a machine property should be a predicate, not a value.**
  VA-2 asked for *an offset that is not `+00:00`*; the offset was assertable
  here (`Australia/Melbourne`, `+10:00`) and asserting it would have encoded
  this machine. `!TimeZone::system().is_unknown()` separates exactly *the
  feature is on* from *the feature is off* — featureless `jiff` falls back to
  `TimeZone::unknown()` — and separates nothing else, so a UTC box passes it
  and still means something.
- **Removing a feature is a louder instrument than adding one.** `POL-001`'s
  residue is that no gate command rejects a feature switched on in a shared
  dependency, and that stands. But an injection that *deletes* the two `jiff`
  features turns five units red. The asymmetry is worth knowing when arguing a
  residue: the argument is owed on the way in, and once the units exist the way
  out is checkable.

- **An injection-pass harness must revert against a commit, not the working
  tree.** The prototype's runner did `git checkout -- crates/goad/src` against an
  uncommitted tree and reverted a whole phase. Replayed, nothing lost — but §9
  commissions an injection pass for every new case, so this is the foot-gun
  inside the discipline the slice depends on most. (`prototype-handback.md` §6.)
- **A code review's findings go stale in a way a design review's do not.** The
  prototype lost P-15 to it: written from a read of `draft.rs` that a commit
  landing mid-phase had invalidated, it described the code wrongly, while every
  finding about the *design* survived the same commit untouched. `review-code.md`
  will run against a moving tree.
- **The user settles usability questions by running the software, not by
  reasoning about it.** Asked to choose between two refusal surfaces, the answer
  was the lean plus the method: *"I'm also inclined to make these usability
  decisions based on interaction with actual software instead of based on a
  leaky theoretical model."* That is `docs/memory/spike-beats-the-argument.md`
  applied to interaction rather than to mechanism, and it should be its own
  memory at close — the shape of the right answer to *which of these two should
  the design say?* is often *build the smaller one and look at it*.
- **A refusal reaches retained state, not the screen.** `Diagnostics` is
  rendered only under `WindowMode.diagnostic` (`app.slint:325`), and
  `Controller::surface()` answers `Diagnostics` only when `focus ==
  Focus::Diagnostics`, which the Diagnostics menu item alone sets
  (`controller.rs:159-165`). `refuse()` writes `self.diagnostics` and never
  touches `self.focus` (`:203-205`). So *reported* means *recorded*, not *shown*,
  for anyone in prompt mode — which is everyone who is answering a form. Slice
  008 recorded the same thing from the other direction in
  `getting-eyes-on-the-running-host.md`. This outlives slice 009 and is a
  follow-up, not a repair inside it.
- **A backend exchange is 2 ms to 5 s, and the floor is a process spawn.**
  Measured this session: `examples/shell/backend.sh` ~2.4 ms over 50 spawns,
  `examples/typescript/backend.ts` ~12 ms over 10, ceiling the configured
  `backend.timeout` (5 s in `examples/demo.toml`). Anything whose lifetime is
  *one exchange* therefore has no useful duration — it is three orders of
  magnitude, chosen by the backend author.
- **Cite from an instrument that prints the number.** Five bad citations in this
  slice, and the fifth was written by the raiser verifying the fourth. Every one
  counted by hand off a `sed -n 'a,bp'` window; every one taken from `grep -n` or
  `awk NR` has held. A hand count is not checkable at a glance, so its being
  right is luck. This supersedes *verify the responder's first* as the operative
  rule — that was a pattern in who made the mistake, not in what caused it.
- **A negative control can stand in for an instrument.** The prototype carried
  three counters; `fires` existed only to tell *the handler never ran* from
  *the guard found agreement*. Two counters plus an injection that makes the
  convergence counter **move** discriminate the same two states, because a
  control that cannot pass unless the handler fires *is* the measurement
  `fires` was taken for. The rule generalises: before adding an instrument to
  separate two states, ask whether a control that must go red already
  separates them.
- **An `out property <int>` on a Slint window root is assignable from inside a
  repeater**, so an instrument counter does not need `in-out` and does not
  widen the component's input surface. Measured at PHASE-01, against the
  prototype's `in-out`.
- **A slot fallback should index past the end, never at zero.** A
  `i32::try_from(len)` that fell back to `0` would alias an over-long form's
  field onto slot 0 and show it *another field's* value; falling back to
  `i32::MAX` indexes out of range, which Slint answers with a
  default-initialised struct. Both unreachable; only one is wrong quietly.
- **`docs/memory/a-present-destroys-the-widget-it-writes.md` needs its first
  two sentences amended at close.** `present` no longer ends in
  `self.options.set_vec(rows)` and no longer rebuilds every row on every
  present — the `set_vec` is guarded by the `view_id`. The rest of the note,
  including the repair it recommends, is what PHASE-01 implemented and stands.
- **A `deny` lint can settle a style question the design left open.**
  `field_value`'s untouched arm and its four not-yet-drawn arms read better as
  two arms and are one, because `clippy::match_same_arms` is an error while the
  two answers agree. The general shape: before arguing about how a match should
  be cut, try the cut — the lint table has an opinion and it is cheaper to read
  than to predict.
- **A green injection can be the right answer, and it has to be said out loud.**
  Planting `Kind::Boolean` for every drawn kind in `glass.rs::markup_kind` left
  all 189 renderer cases green, because nothing can yet construct a drawn field
  of another kind. That is a correct state of the world and not a weak case —
  but it is indistinguishable at a glance from a test asserting a proxy, so a
  phase that runs an injection and gets green owes the record a sentence saying
  which of the two it is, and which phase measures it first.
- **`interpret`'s totality is a shape, not a comment.** Matching
  `(kind, report)` as a nested match with an explicit or-pattern over the
  five out-of-kind variants — rather than the shorter tuple match with one
  `(A | B | …, _) => None` arm — is what makes a sixth variant of **either**
  enum a compile error. The short form answers a sixth kind by silence, which
  is exactly the failure `DrawnKind` exists to prevent.
- **Enumerate the refusal surface in the case, and count it.** The mismatch
  case walks all six reports against all five kinds, skips the six in-kind
  pairs and asserts `refused == 24`. Sampling three pairs would have passed
  under an injection that made one whole kind permissive; the count would not.
- **A wire key and a canonical type can disagree on purpose.** A `choice`
  field's alternatives arrive under the key `options` (`R-16`, `R-53`) and
  normalize to `Alternatives`, because an alternative is a **value** and a
  view's option is an **action**. Two fixtures were written the other way first
  and failed with `EmptyAlternatives`. The rule: read `normalize.rs` for the
  wire spelling, never `canonical.rs`.
- **A doc the phase makes stale is amended in that phase, not at audit.**
  Settled three times in this slice now: `design.md:1042` for the
  `Glass::present` contract, PHASE-04/EX-5 for `clock.rs`, and PHASE-03 for
  `Refused::UnknownField`. The line is the *discovery*: a divergence found at
  audit belongs in the Reconciliation table; one the slice creates knowingly
  does not. The corollary is that a phase whose criterion widens what a type
  means should check its Surfaces for the type's own file before it starts.
- **A true statement can go stale by *widening*, and that is not a bad
  citation.** Twice in one phase: `design.md` §9's *twelve* was accurate about
  **constructors** of `Command::Edit` and `Edited` and silent about a
  **binder** of the markup callback, which is why `tree.rs` was a thirteenth
  site the count never claimed to cover; and `Refused::UnknownField`'s doc was
  accurate about the only path into it that then existed. Neither was wrong
  when written and neither is a miscount — the world got larger than a sentence
  whose scope was right. It is a different animal from §*Citations known bad*,
  which is about numbers that never pointed where they said, and it does not
  belong on that list. The working rule is what the two cases share: when a
  phase widens what a type or a rule **means**, re-read every sentence that
  enumerated it, including the ones that are still true.
- **When you raise a stale doc, name every clause of it, not the one that
  caught your eye.** `Refused::UnknownField` had two: the sentence describing
  the refusal, and a later *"Only reachable from a stale or malformed
  callback"* which after this phase is the more wrong of the two — the new path
  is a well-formed, current callback. The first was reported and the second was
  found by the person reading the report.
- **A boundary instrument can forbid a word the next slice wants** (P-10).
  `scan::mentions` word-matches after splitting on non-alphanumerics and camel
  boundaries, and keeps string literals, so an identifier or a diagnostic message
  can red a purity instrument that has no view on either. Check the boundary
  suite's needles before naming a new function.
- **A markup literal should name the control's own kind, never the row's.**
  The `CheckBox` reports `Kind.boolean` rather than `field.kind`. The two are
  equal and will stay equal, so this is not about the value — it is about what
  can be measured: `field.kind` makes the report agree with the row *by
  construction*, and `interpret`'s *report whose variant is not the drawn
  field's kind* could then never fire from production markup at all. The
  general shape: a check that compares two things is worth nothing if one of
  them is derived from the other. Measured — planting `Kind.text` in the
  literal turns five cases red across two files.
- **Comparing a whole generated struct is how you assert what a literal did
  *not* say.** Slint struct literals may name a subset of fields and the rest
  default, which is what keeps `FieldEdit` honest per control — but a case that
  reads one slot back cannot tell a two-field literal from a five-field one.
  `tree.rs` compares the whole `FieldEdit` against
  `FieldEdit { kind, checked, ..FieldEdit::default() }`, and an injection that
  writes one further slot turns it red.
- **An injection runner must confirm its revert against the string it read, not
  against `git`.** `git diff --stat` compares to `HEAD`, which is the wrong
  baseline in the middle of a phase — it reports the phase's own work as an
  unreverted injection, every time. Compare the file's bytes back to what was
  read. This is the same foot-gun as the prototype's `git checkout` one door
  along: the revert check, not the revert.
- **`tests/renderer/wiring.rs` and `tests/renderer/fields.rs` measure disjoint
  halves of the edit path, and the injections prove it.** Breaking
  `install.rs`'s closure turns four `fields.rs` cases red and nothing in
  `wiring.rs`; breaking `Controller::edit` turns `wiring.rs` cases red and
  nothing in `fields.rs`. `wiring.rs` drives the controller directly and never
  reaches the markup callback. So a phase adding a control's arm to `reported`
  is measured by a `fields.rs` case and by nothing else.

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

- **One token in `today_local` is held by review alone** — that the zone it
  reads is the **system's**. The class is covered
  (`one_instant_is_two_different_local_dates_in_two_different_zones`, and
  injections I2/J/K), and the token is not: a unit cannot change the zone the
  machine is in. Same shape as VA-2's residue, and `design.md` §8 R8 already
  names review as the mitigation.
- **PHASE-03's sheet totals 564 and its own per-target list comes to 529.**
  Every per-target number it recorded is right; the sum is not. This phase's
  538 was taken by summing the `test result: ok. N` lines the gate prints.
  Left as written — another phase's record — and reported.

- **Follow-up: `SPEC-002/OQ-4` has lost the reason it stayed open.** OQ-4 asks
  whether a host should suppress or defer a firing while a presentation is
  outstanding. It is open partly because suppression *"asks the host to judge
  that a view is worth protecting, which is domain meaning it does not hold"*.
  After this slice the host retains a draft and a keyed pending map, so *typed
  into and not yet answered* is interaction state, available without
  understanding anything about the domain. The other half of OQ-4's reason —
  that deferral needs a second pending state and a second writer of the deadline
  — is untouched, and so is the observation that the answer may belong to the
  backend. Not this slice's to answer (`slice-009.md` §Non-goals) and not
  reopened here; recorded so a later slice does not re-derive it. `design-log.md`
  D-36, `design.md` §8 R5.
- **The `#[cfg(test)]` fixtures that read an id off a normalized view are
  duplicated** between `draft.rs` and `view_model.rs`, because neither module
  can reach the other's test module and a shared one would need `lib.rs`, which
  was outside PHASE-02's Surfaces. PHASE-03 onwards will want the same fixture
  again. A `#[cfg(test)] pub(crate) mod fixtures` under `lib.rs` is the obvious
  home; it is a decision, not a repair, because `lib.rs`'s own doc says it is
  the module tree *and nothing else*.
- **`markup_kind`'s four non-`boolean` arms have no case behind them** until
  PHASE-05 draws a `LineEdit`. Not a gap this phase could close — nothing can
  construct a drawn field of another kind while `undrawn_form` is unchanged —
  and recorded so the audit does not read it as one.
- **`install.rs::reported`'s four undrawn arms answer `None`, and nothing
  measures them** — the same shape as `markup_kind`'s, and for the same reason.
  What is worth watching is that the hole is *silent by construction*: a phase
  that draws a control and forgets its arm here drops every edit from it. It
  fails that phase's own first case, so it is caught — but it is caught by
  absence of an effect rather than by a compile error, which is weaker than
  everything else in this tower. The function's doc names the phase that owns
  each arm for that reason.
- **The design does not settle how the closure tells a numeric `LineEdit`'s
  report from a `Slider`'s**, and PHASE-08 must settle it before it draws the
  second control. `Reported` has six variants for five kinds — `AdjustedText`
  and `AdjustedValue` both under `number` — while the markup's only
  discriminant is `FieldEdit.kind`, which `design-log.md` D-12 welds to the
  protocol kind and never the control. §5.2's table has the `LineEdit` sending
  `text` and the `Slider` sending `number` (D16), both under `Kind.number`, and
  no slot value separates them: an empty `text` is the measured cleared-field
  case, not a slider at rest. Three shapes are available — a sixth discriminant
  value, a second `FieldEdit` field, or `interpret` doing the split from
  `slider_bounds` — and choosing between them is a decision, not a repair.
- **Four id sequences collide now, not three.** `design.md` §7's `Dn`,
  `design-log.md`'s `D-n`, `prototype-notes.md`'s `P-n` — and the `VT-n` doc
  comments on the cases in `tests/renderer/wiring.rs`, which are **slice
  008's**. A case added to `mod editing` for this slice has to head itself
  `PHASE-03/VT-2` to avoid claiming a neighbour's id.
- **Follow-up: what a supersession costs is not what R5 said it was.** A
  superseded view clears the field under the caret and loses everything typed
  into it, because `Command::Edit` mutates the retained draft and never reaches
  the backend (`controller.rs:661-675`). The row said *widens the window* and
  named diagnostic-pane noise as the signal. Restated in place. Nothing in the
  slice changes; the audit should read the row as written now.

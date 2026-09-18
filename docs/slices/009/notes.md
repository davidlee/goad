# Notes — Slice 009

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Handover — design review, after the prototype's handback

Written 2026-09-18 for a fresh agent, and rewritten the same day when the
prototype handed back. Delete once the design closes.

### Where the slice is

Design accepted by the user at draft, then rewritten across three review rounds.
**The review loop is open.** Round 3's thirteen findings — F-6, F-20, F-26, F-29,
F-32 and F-38 … F-45 — are **integrated** into `design.md`, `slice-009.md` and
`canon-delta.md`. Rounds 1 and 2's are integrated too, and round 3 verified 18 of
them. Plan not started.

Integrating raised four more, F-46 … F-49, dispositioned, integrated and
**confirmed by the user** (D-25). None is a blocker. So all seventeen read
`_pending round 4_` and every one of them is awaiting nothing but a terminal
outcome from a raiser.

**Then the prototype ran, and stopped after `text`.** It handed back fifteen
findings (`prototype-handback.md`, `a1171b3`), one of which — **P-10** — stops
the slice closing as designed: a live `ADR-001` instrument asserts that no
production line under `crates/goad/src` names the identifier `resolve`, and §5.2
names the new function `resolve`. Four decisions came out of it, D-29 … D-32,
and **none of the fifteen is in the ledger**: D-29 routes them through the log
instead, because a measurement is neither a reviewer's finding nor a user's
choice. `number`, `choice` and `datetime` were never built and §9's validation
table was never attempted, so nothing the prototype says is evidence about them.

**D-29 … D-32 and the prototype's repairs are now integrated** (2026-09-18, a
fresh agent per D-21). §*What is owed* item 1 says where each one landed and
what the pass found doing it. **One thing was not applied: P-14**, which is a
decision rather than a repair — §*Waiting on the user*. `design.md` is otherwise
current truth again, and the design's function is now **`interpret`**.

**What changed in the integration**, in one line each, because the ledger's
Responses do not all say where the text landed:

- the F-40 family became one mechanism: a pending entry carries the view it was
  made on (§7 D27), the value channel is the draft overlaid with it (§7 D26), the
  timer delivers one entry per tick and re-arms, and `Wire::send` reports whether
  the command was enqueued. §5.5 states it as **I-H**, which is new.
- `resolve` takes `held: Option<&Edited>` and applies `as_drawn` itself, so the
  two `as_drawn` sites are now `answer` and `resolve` rather than `answer` and
  `edit`. (That function is `interpret` from D-30 onward; this paragraph records
  round 3's integration and is left as it was written.)
- §9 gained four rows — two fields in one window, the numeric exception measured
  against the overlay, the picker re-seed (F-44), and a rewritten AC-6 driver
  (F-47) — and §8 gained **R10**.
- `slice-009.md` §Scope gained `main.rs` and the overlay; AC-6 gained one
  sentence of precision.

### Why a fresh agent, and not the session that dispositioned these

D-21. The session that writes a disposition does not integrate it, and this has
now paid for itself three times: round 1's responder was wrong about four of its
own repairs, round 2's integrator found four more defects, and the session that
integrated round 2 found F-37 — a blocker — by trying to write the repair down.
Round 3 then found that same session had inverted F-32's citation and overclaimed
a type property (F-42). **Expect to find something. Integrating is a form of
review.**

### What holds the truth

| file | state |
|---|---|
| `review-design.md` | **the ledger.** F-1 … F-49, with round 3's outcomes set and its probed-and-sound list. Every Response is written to be complete without the session that wrote it: they are your brief |
| `design.md` | **current truth**, as of the integration of D-29 … D-32 and the prototype's repairs. Nothing is outstanding against it but P-14, which is a decision and is not applied |
| `design-log.md` | D-1 … D-32. Append-only. D-29 supersedes D-27's second paragraph. Note the header: `D-n` here is **not** `Dn` in `design.md` §7 |
| `research.md` | Thread 3 is everything measured |
| `canon-delta.md` | CD-1, CD-2. F-45 touches CD-2 |
| `spike-fields/` | committed at `4f93d41`. Delete when the design closes |
| `prototype-notes.md` | on `slice-009-prototype` only. **The artefact**: P-1 … P-15 in full, with the reasoning. P-15 is withdrawn in place |
| `prototype-handback.md` | on `slice-009-prototype` only. The index and the recommendation, and §What it did not test — read that before citing any of it |

### What is owed, in order

1. ~~**Integrate D-29 … D-32 and the prototype's repairs into the design.**~~
   **Done, 2026-09-18**, by a fresh agent per D-21. Every row below landed;
   `design.md`, `slice-009.md` and `canon-delta.md` carry them. The decisions
   were in `design-log.md` and the evidence in `prototype-notes.md` on
   `slice-009-prototype`, which is the artefact.

   | | change | where it actually landed |
   |---|---|---|
   | ~~D-30~~ | `resolve` → **`interpret`**, and the constraint stated | §5.1 (diagram and the `Reported` sentence), §5.2 (the function, and the constraint beside it), §5.3, §5.5 I-G and the edges table, §7 D12 / D25 / D26, `slice-009.md` §Scope. **Not §9** — the table said §9 and §9 never named the function |
   | ~~D-31~~ | the `None` surface becomes **three** cases | §5.2, and §5.5's edges row, which had enumerated exactly the two D-31 supersedes |
   | ~~D-32~~ | `Display`, and `{:e}` beyond **24 characters** | §5.2 (a new paragraph beside the parse rule, and the `number` as-drawn bullet), §9 (a new row, `tests/renderer/fields.rs` over a `view_model.rs` unit) |
   | ~~P-2~~ | the live way out: `DrawnKind::Choice` carries the first id beside the list | §5.2's `choice` as-drawn bullet, with the two dead routes named; and §5.1's one-line description of `DrawnKind`, which the repair also changes |
   | ~~P-7~~ | two sets, not one trio | §5.2 (the sentence corrected in place, then the two sets stated), **and** §5.5's edges row, which repeated the claim verbatim |
   | ~~P-8~~ | `Finite` carries no `Eq` either, and why | §5.2, beside the `Eq` paragraph F-48 wrote |
   | ~~P-12~~ | A-1's row re-tiers: `init` runs under `init_no_event_loop` | §9 — the *what still needs a real loop* paragraph, and AC-4's tier, which was split across both tiers and is now wholly `tests/renderer/` |
   | ~~P-11~~ | the timer re-arm is measured, not only read | §5.1's citation |
   | ~~P-4~~ | §10's argument gates `compose` and `today_local` and nothing else | §10 (a new *what the feature does not gate* paragraph), `canon-delta.md` CD-1's open question |

   **What this pass found**, written up rather than applied where it is a
   decision:

   - **`P-n` is a *third* id sequence, and it collides where it hurts.**
     `design.md` §4's guiding principles are `P-1`, `P-2`, `P-3`; the
     prototype's findings are `P-1 … P-15`. All three of the prototype's
     colliding ids land in §5.2, and §5.2 already cited §4's `P-3` in the very
     sentence the as-drawn repairs sit under. Two mitigations applied: no
     prototype `P-n` appears in `design.md` at all — the design states what is
     so, and the provenance lives here — and the surviving citation now reads
     **§4's P-3**. §*Traps worth naming* now carries it beside the
     `D-n` / `Dn` pair.
   - **P-7 and D-31 were each owed in §5.5's edges table as well**, and the
     table above named only §5.2. Both edges rows restated the claim being
     repaired, word for word. This is `docs/memory/`'s *prose outside §9's
     obligations table binds nothing* seen from the other side: a claim
     repeated in two places is repaired in one.
   - **D-32 puts a constraint on the grammar the plan pins.** §5.2 leaves
     *which characters make up the numeric grammar* to the plan; D-32's own
     rationale asserts that both spellings re-parse "under the grammar P-5
     pinned, **which admits `e` and `E`**". Without `e` in the grammar, `1e5`
     is a text with exactly one foreign character and the parse rule reads it
     as `1.5`. Stated in §5.2 as a constraint the plan inherits, not left to be
     rediscovered.
   - **The rename needs no §9 row.** The instrument is
     `crates/goad-boundary/tests/checks/structure.rs:308` and `just check` runs
     `cargo test --workspace`, so the constraint is already in the gate. §5.2
     says so, so nobody writes a row for it.
   - **`interpret` was checked against the rest of the boundary suite** before
     it was written in: the domain-vocabulary list (`vocabulary.rs:18-25`), the
     purity path list (`purity.rs:17-27`) and `structure.rs`'s three call-form
     greps. Clear of all of them. This is the harvest's *check the boundary
     suite's needles before naming a new function*, performed.

   **Open, and waiting on the user: P-14.** See §*Waiting on the user* below.
   Nothing in the design was reshaped to accommodate either answer.

   **Not touched, deliberately:** `review-design.md` (D-29 — the seventeen stay
   `_pending round 4_`); canon; `design-log.md` (no user decision was taken
   this pass); §5.1's pricing of the `FieldForm` consumers (P-13 is the
   plan's).

2. **Round 4**, raised by a **fresh Claude agent** (D-28), after the integration.
   Codex is out of credits; the protocol asks for a fresh raiser, not a fresh
   model. Rounds 1-3 were all `gpt-5.6-sol`, so round 4's Brief should say that
   the blind spots change with the model and that this cuts both ways. Thread
   `01a0b212-239e-70d3-9a99-09729c82b971` remains round 3's own and is still the
   right place to set round 3's outcomes if credits return — and the wrong place
   to raise. **Write round 4's Brief before the reviewer runs**, against what the
   prototype found as well as what the integration changed. **Do not hand round 4
   the `P-n` list** (D-29): anything it finds independently is a second witness.

3. **Bring the prototype's record back.** `prototype-notes.md`,
   `prototype-delta.md` and `prototype-handback.md` exist only on
   `slice-009-prototype`. They are slice documents and belong on `main` before the
   branch is retired. The code is **referenced, not promoted** — the slice
   re-derives from its own plan.

4. **Re-ask the user for acceptance.** The design has changed twice since theirs,
   and this integration is the third.

5. **Plan**, with a fresh agent. P-13 and P-14 are inputs to it, and so is the
   handback's §Recommendation: if the prototype is resumed, **`datetime` first** —
   `number` mostly exercises pure functions that already have coverage, `choice`
   is small, and `datetime` is where the unmeasured mechanisms are.

6. **Delete `spike-fields/`** when the design closes.

### Waiting on the user

**P-14 — a refusal raised beside an answer that proceeds does not survive the
fold.** Not applied, and not reshaped to fit: it is a decision, and D-21 is what
that rule exists for.

The design (§5.2, §5.5's edges table, I-H) says that a carried edit whose `view`
is not the retained one is `Refused::SupersededView`, **reported**, and *the
answer still goes*. The prototype built exactly that and found the second half
undermines the first. `serve` has one refusal site and it `continue`s: a refusal
and an exchange are alternatives in `dispatch`'s `Option<Result<Pending,
Refused>>`, so there is no value that says both. The shape that works is the one
`refuse_during_exchange` already uses — `Controller::choose` calls `self.refuse`
itself and returns `Ok` — and it inherits that arm's own stated consequence:
`absorb` replaces the whole retained `Diagnostics` when the exchange folds, so
the line is on screen for the duration of the exchange and then gone.

Two smaller things came with it. The line a person reads is
`Refused::SupersededView`'s existing one — *"no action taken: that answer belongs
to a question that has since been replaced"* — which is about an **answer**, and
here it is about discarded typing on an answer that did go. And a `Choose`
carrying two stale edits reports **once**, not twice, because
`Diagnostics::refused` replaces rather than accumulates.

So the choice is:

- **§9 gets a surface that survives the fold** — a second diagnostics channel,
  or a class of refusal `absorb` does not replace. New mechanism, and it is host
  functionality, so the first invariant's question applies to it.
- **or the design says the refusal is not durable** — one sentence in §5.2:
  reported for the life of the exchange, and no longer. Nothing in the host today
  can promise longer. The promise §5.2 makes now is *reported*, which is true;
  what it does not say is *for how long*.

This also reaches §5.5's edges table, which has a row for it, and R5 in §8, which
already accepts the wider window as a cost rather than mitigating it.

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
- **The overlay creates a property worth stating as an invariant:** what the
  screen shows is what an answer would submit. A drained entry reaches the draft;
  a kept entry is still displayed and still travels in the next `Choose`; a stale
  entry does neither. That is new and it is better than what the design had.
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
4. **The ICU decimal separator is live in this build.** `i-slint-core`'s default
   `std` feature enables `i-slint-common/locale-decimal-separator`
   (`i-slint-core/Cargo.toml:82-95`), and `string_to_float` replaces *that*
   character, rejecting `.` outright when the separator is not `.`
   (`i-slint-core/string.rs:398-412`). It is **not** reachable from host code:
   `SlintContext::locale_decimal_separator` is `i-slint-core`, which `crates/goad`
   does not depend on, and `slint` re-exports neither it nor `string_to_float`.
   F-26.
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
**Three** are known, and the list held five until the prototype checked it
(`prototype-handback.md` §5): F-10's re-disposition (one `wiring.rs` site, not
two); F-23's Response (`wire.rs:130`, not `:126`); and F-33's Response
(`fluent/components.slint:15-19` for `ListItem`; they are at `:49-53`).

All three were written by a **responder**, not by a reviewer. Rounds 2 and 3's
own citations checked out. **Verify the responder's first** — including this
list's own, which is where the two below came from.

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

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | pending / in progress / done / blocked | |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — <name>

**Objective:** <copied from plan.md>

**Reading list**
<!-- path:line references, the design sections that bind, prior art. -->

**Assumptions & STOP conditions**
<!-- What is being taken on faith, and the specific conditions under which the
     agent must stop and consult the user rather than improvise. -->

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [ ]

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** <yyyy-mm-dd> · <phase or stage> · <commit>

### Produced
<!-- What now exists: modules, contracts, docs. -->

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

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
- **A boundary instrument can forbid a word the next slice wants** (P-10).
  `scan::mentions` word-matches after splitting on non-alphanumerics and camel
  boundaries, and keeps string literals, so an identifier or a diagnostic message
  can red a purity instrument that has no view on either. Check the boundary
  suite's needles before naming a new function.

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

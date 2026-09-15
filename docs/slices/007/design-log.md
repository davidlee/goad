# Design log — Slice 007

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### 2026-09-14 — where does a half-filled answer live? (OQ-1)

- **Asked:** `serve` presents at the top of every loop iteration and
  `Glass::present` is contracted to write every property, every time. A field
  holds user state between presents, so the two collide. Three homes for a
  draft: **(a)** Slint owns it, and the glass writes the field model only when
  the `ViewId` changes; **(b)** the controller owns it, an edit becomes a
  `Command`, and `present` writes the model from the draft every time;
  **(c)** no draft at all — the click carries the values out with it.
- **Recommended:** (b).
- **Decided:** "OQ-1 - let's go with the controller."
- **Consequence:** `Controller` retains the draft, inside `Prepared` so that a
  draft and the view it answers are one value and `shown = None` cannot leave
  one behind. `Glass::present` stays total — no exception to carve, no
  write-only-on-change rule to test around. `answer()` assembles
  `UserResponse.values` from retained state, which also means every submitted
  key came from a field the retained view declared, reusing the check that
  already refuses an unknown option id. Design owns the shape of the edit
  command and where the draft sits in `Prepared`; AC-5 is what proves the
  choice held.

### 2026-09-14 — the number for the look slice

- **Asked:** the visual pass is split out of 007 and needs a number; 008 and
  009 were taken by stubs (socket transport, starter experience).
- **Decided:** "I'd suggest the next number in the sequence, renumber the stubs
  if necessary."
- **Consequence:** the look is **008**, immediately after the form that makes it
  necessary. Socket transport 008 → **009**; starter experience 009 → **010**.
  Neither stub had a folder. Forward pointers in the closed cards
  `slice-004.md` and `slice-005.md` were corrected in place — they are
  artefacts, which hold current truth and are edited in place; no log was
  touched.

### 2026-09-14 — does R-57 cover kinds this renderer does not draw?

- **Asked:** CD-1's scope. Either the new requirement names the submitted JSON
  type for all four non-`datetime` kinds regardless of what 007's renderer
  draws, or its table carries a row only per drawn kind and the rest stay
  undefined alongside `datetime`.
- **Recommended:** all four. `boolean`/`text`/`number`/`choice` have no degrees
  of freedom in their JSON type; `datetime`'s string form genuinely does, which
  is why it stays out (`canon-delta.md` §Deliberately out).
- **Decided:** all four kinds.
- **Consequence:** the wire contract stops tracking the renderer, which is
  SPEC-001/R-55 and CLAUDE.md's third invariant applied to canon itself. It also
  demotes OQ-2 from a protocol question to a testability one: whatever 007 draws
  narrows nothing, because the spec already admits all four. Research F2/F4's
  split stands — **R-57 type by kind**, **R-58 completeness** (a value for every
  field drawn, nothing for a field not drawn) — and R-58 is what makes a partial
  drawn set safe to state.

### 2026-09-14 — which field kinds does the renderer draw? (OQ-2)

- **Asked:** `slice-007.md` OQ-2 recommended four kinds (`boolean`, `text`,
  `number`, `choice`), leaving `datetime` undrawn. Research contradicted it on
  two of the three additions: `SpinBoxBase` is `int` with `maximum: 100` invented
  when bounds are absent while canonical `number` carries `Option<f64>`
  (research F11, re-verified at `widgets/common/spinbox-base.slint:5-17`), and
  `ComboBox` declares only `accessible-action-expand`, so `choice`'s submitted
  value is unreachable from the headless tier (F12, re-verified at
  `widgets/fluent/combobox.slint:28-33`). Three positions: `boolean` only;
  `boolean` + `text`; the card's four.
- **Recommended:** `boolean` only.
- **Decided:** `boolean` only.
- **Consequence:** every kind this renderer draws has its submitted value
  asserted by an automated test, with no proxy standing in for it —
  `docs/memory/` records the slice 004 case where four green tests each asserted
  something their regression would have survived. `text`, `number`, `choice` and
  `datetime` are all reported undrawn, so AC-3 is exercised by kinds a backend
  really sends rather than by a synthetic one, and `Undrawn` stays reachable by
  construction. Nothing narrows: R-57 types all four regardless (decision above),
  and R-55 names this a renderer subset.
  Two problems leave the slice with it. **F10**, the edit cadence — `LineEdit`'s
  `edited` fires per keystroke into the capacity-1 channel `serve` drains only at
  the top of the outer loop (`main.rs:86`, `wire.rs:126`, `controller.rs:611`) —
  is deferred to the slice that has evidence for a text field, where OQ-1's own
  note says it should be settled with a measurement. **F11**, `number`'s
  presentation and its absent-bounds behaviour, goes with it. Both belong in
  Follow-ups at close.
  A checkbox needs no edit cadence: one click, one message, and fourteen of them
  cannot fill a channel the loop drains between clicks.

### 2026-09-14 — several options each carrying fields (OQ-6)

- **Asked:** a block per option — that option's fields, then that option's
  button — or reveal the selected option's fields on demand.
- **Recommended:** a block per option.
- **Decided:** a block per option.
- **Consequence:** AC-4 holds by construction rather than by a check: a button
  can only reach the values sitting in its own block, which is the literal
  reading of SPEC-001/R-8 (one option, one flat map). No selection state is
  introduced, so `Glass::present`'s totality has exactly one new thing to write
  back — the draft — and not two. An option carrying zero fields stays the bare
  button it is today, which is what AC-6 asks for.
  The keying consequence is design's to state and is not optional: R-52 makes a
  field id unique **within an option**, so two options may both declare `done`.
  The draft is therefore keyed by (option id, field id), never by field id
  alone, and `answer()` filters the draft to the answered option before
  assembling `values`.

### 2026-09-14 — the grouping rule, and what a non-string `group` means (OQ-4)

- **Asked:** two rules. **Runs** — one pass in declared order, a heading drawn
  wherever the `group` value differs from the previous field's, so screen order
  is always wire order. Or **merge** — AC-2 as currently written: groups in order
  of first appearance, each collecting every field that names it, which moves a
  field up past an intervening one.
- **Recommended:** runs.
- **Decided:** runs.
- **Consequence:** **AC-2 is amended.** Its "groups appear in order of first
  appearance" clause described merge and no longer describes what is built; the
  criterion becomes *a heading is drawn where the group value changes, and no
  field is ever reordered*. That makes "no sorting — sorting would be the host
  imposing meaning" (AC-2's own last line) hold absolutely rather than nearly:
  the host never decides a field's position, so a hint it cannot read costs
  nothing but a heading.
  Three sub-rules settle with it, and the third only became visible once runs was
  chosen:
  - A `group` whose value is not a JSON string is **ungrouped and reported**
    (`Hints::as_map` is `BTreeMap<String, serde_json::Value>`,
    `canonical.rs:127`). Coercion would invent a heading the backend did not
    author. Only the key the renderer branches on is reported this way — an
    unhonoured hint the renderer never reads is not a finding.
  - A `group` of `""` is ungrouped and **not** reported: the backend sent a
    string, and there is nothing to say beyond that it was empty.
  - A run is a **block**, and `Some(name) → None` starts a new untitled block
    rather than continuing the last titled one. Otherwise an ungrouped field
    following a group is drawn under a heading that does not claim it.

### 2026-09-14 — the widget style, and the environment override (OQ-5)

- **Asked:** whether 007 selects a widget style in `build.rs`, given that 008 is
  the look slice. Verified first, because it changes the question: the style is
  *already* switchable with no code —
  `slint_build::CompilerConfiguration::new()` reads `SLINT_STYLE` from the
  environment (`i-slint-compiler-1.17.1/lib.rs:264`) and the compiler defaults to
  `fluent` (`typeloader.rs:937`), while `with_style` **overwrites** whatever the
  environment said (`slint-build-1.17.1/lib.rs:153-155`).
- **Recommended:** no — leave `build.rs` alone and let 008 choose with the whole
  surface in view.
- **Decided:** yes, select a style, as `slice-007.md` OQ-5 recommended.
- **Consequence:** `crates/goad/build.rs` is in scope after all, and the slice
  carries a visual change no headless test observes. Design states that plainly
  rather than letting it arrive as a surprise in review, and 008 inherits the
  choice rather than making it.

### 2026-09-14 — the style is a default, not a fixture (OQ-5, second half)

- **Asked:** having taken the style, whether `SLINT_STYLE` still overrides it.
  Hard-coding gives an environment-independent build; reading the variable keeps
  the comparison 008 needs available without a code edit.
- **Recommended:** keep the override, default to `material`.
- **Decided:** keep the override, default to `material`.
- **Consequence:** `build.rs` reads `SLINT_STYLE` and falls back to `material`,
  so the slice takes the style as a *default* and gives up nothing that exists
  today. `material` comes from the spike `slice-007.md` OQ-5 records, not from a
  fresh comparison. The three lines are the whole of 007's visual change; every
  other look question is 008's, per §Non-goals.

### 2026-09-14 — a firing that would discard a half-filled form (OQ-3)

- **Asked:** first as "do we report the loss?", which the user rejected as the
  wrong question — *"I reckon not tossing user work is the priority here - less
  fussed about eg. displaying that there was another firing for now."* Asked
  again as three ways to protect the work: **hold** the scheduled firing while
  the draft is dirty and run it when the draft resolves; **defer once** and then
  proceed, reporting the loss; or leave it to the **backend**, which already
  knows what it last presented and whether a `respond` arrived for it.
- **Recommended:** hold.
- **Decided:** hold.
- **Consequence, and it is the largest scope change design has taken:**
  **007 now answers SPEC-002/OQ-4.** A second canon amendment joins CD-1, and an
  ADR goes with it — the rule is reversible by accident otherwise, which is
  `docs/AGENTS.md`'s own test for when a decision earns one. `canon-delta.md`
  grows a CD-2 entry, and `slice-007.md` §Non-goals loses the line excluding
  SPEC-002/OQ-4. The slice was already tier 2; this does not raise the tier, it
  widens it.
  Three boundaries settle with the decision:
  - **Held, not skipped.** The firing runs the instant the draft resolves — a
    submission, or the view closing — so nothing is lost on either side. This is
    why nothing needs reporting: the reporting half existed to describe a
    discard, and there is now no discard to describe.
  - **Scheduled firings only.** `Check now` from the tray and an ingress event
    are someone asking, and are honoured immediately. Holding those would be the
    host overruling a person in the name of protecting them.
  - **Dirty means at least one field edited away from what was drawn.** Nothing
    ticked is not dirty, so an untouched form never holds a firing. That
    predicate is pure interaction state — the brief gives the host interaction
    outright — and it reads no field's meaning, which is what keeps the rule
    clear of domain vocabulary.
  The failure mode is on the record because a reviewer will find it: a person
  who ticks one box and walks away holds the schedule indefinitely. It is
  distinguishable from a broken host in the way SPEC-002 cares about — the
  window is on screen with their own tick in it — but the design states it
  rather than discovering it at audit, and ADR-004's spacing rule has to say
  what the held firing does to the next one.

### 2026-09-14 — does the hold cover ingested events? (OQ-3, boundary)

- **Asked:** the previous answer's preview bundled `Check now` and an ingress
  event together as "someone asking". They are not the same thing: `Check now`
  is the person, and an ingested event is another program on someone else's
  schedule — the case SPEC-002 §5 singles out as the one a second stimulus
  reaches more often. Either the hold covers scheduled firings only, or it
  covers ingested events too, refusing them at arrival the way SPEC-003/R-12
  already refuses an event it cannot take.
- **Recommended:** scheduled firings only, on scope: the alternative makes 007
  amend a third spec.
- **Decided:** scheduled firings only.
- **Consequence:** the protection is real but not total, and the design says so
  rather than implying a guarantee it does not give. An ingested event still
  supersedes a dirty form, exactly as it supersedes an unanswered view today.
  SPEC-003 is untouched and stays on the checked-and-not-applicable list.
  **Follow-up at close:** ingress against a dirty form, with SPEC-003/R-12's
  refusal-at-arrival as the shape it would take if evidence asks for it.
  Canon this slice now touches: SPEC-001 (CD-1, R-57/R-58), SPEC-002 (CD-2,
  OQ-4 answered), and one new ADR for the hold.

### 2026-09-14 — the hold is deferred (supersedes both OQ-3 entries above)

- **Asked:** by the user, on reading design §1 — *"agreed in principle. if the
  scheduled firing interrupt thing is a huge scope increase I could just as
  easily defer it."* Sized honestly: the **code** is about five lines, and the
  mechanism is cheap (below). The **canon** is not — a SPEC-002 amendment (a new
  requirement, a §7 row, and the §5 paragraph at line 115 that currently says
  OQ-4 stands), an ADR, a second `canon-delta.md` entry to reconcile, and review
  rounds on both. Roughly a third more design, review and audit surface on a
  slice already amending SPEC-001 twice.
- **Recommended:** defer, and write the mechanism down so it is not re-derived.
  The argument is the one that split the look out of 007 in the first place: a
  scheduling change inside a rendering slice yields a diff in which a scheduling
  regression and a protocol regression look alike.
- **Decided:** defer.
- **Consequence:** 007 amends **SPEC-001 only** and carries **no ADR**.
  `canon-delta.md` keeps CD-1 alone; no CD-2 is written. SPEC-002 returns to the
  checked-and-not-applicable list, and `slice-007.md` §Non-goals keeps its
  SPEC-002/OQ-4 line rather than losing it. The two OQ-3 entries above are
  superseded, not deleted: the *reasoning* in them stands and is what the
  follow-up inherits.
- **What the follow-up inherits, so no one re-derives it:**
  - **The mechanism.** `held` is *derived* per loop iteration
    (`let held = controller.draft_is_dirty();`), never stored, and the timer arm
    carries tokio's own precondition — `() = &mut sleep, if !held => …`. Because
    an elapsed `Sleep` stays elapsed while its arm is disabled, the firing lands
    the instant `held` goes false. No second pending state and no second writer
    of the deadline — which is precisely the objection SPEC-002/OQ-4 raises
    against deferral, and it does not apply to this shape.
  - **The ADR-004 interaction.** A held firing is *not* a firing, so it must not
    write `floor_until`. With the precondition the arm does not run at all while
    held, so the one and only write site (`controller.rs:621`) is never reached
    — correct by construction rather than by a rule someone must remember.
  - **The boundary already ruled on**: scheduled firings only. `Check now` is a
    person asking. An ingested event is another program on someone else's
    schedule and would need SPEC-003/R-12's refusal-at-arrival shape — a third
    spec, and out.
  - **The exposure while it waits.** Clobbering needs a firing **and** a backend
    that returns a new view; `view: null` folds to `Shift::Retained` and leaves
    the draft untouched (`controller.rs:171-176`). The cost of the gap is
    re-ticking a form, once — which is the evidence the scheduling slice should
    be cut from.

### 2026-09-14 — the extension seam for a second field kind (design §5.2)

- **Asked:** by the user, on reading §5.2 — *"When Command::Edit needs to
  accommodate eg mutations to a string or number type, what's the plan for
  extension look like."* `Command::Edit { …, checked: bool }` accommodates one
  kind only, and `number` and `choice` are discrete single-event edits that
  would fit the same door with a different payload.
- **Recommended:** land the seam now — an `Edited` value enum owned by
  `draft.rs`, one variant today, with `submitted(&Edited) -> serde_json::Value`
  as a total match.
- **Decided:** land it now.
- **Consequence:** `Draft` stores `Edited`, not `bool`; `Command::Edit` carries
  `value: Edited`; `wire.rs` depends on `draft.rs` and not the reverse, because
  the value's meaning belongs with what stores it rather than what carries it.
  **R-57 acquires a single enforcement site** — `submitted` — so a kind added to
  SPEC-001/R-16 without a decided submitted type is a compile error naming that
  function, which is the device `present()` already uses for `View`'s variants
  (`view_model.rs:103-106`). Adding a kind then touches five places the compiler
  finds: the `Edited` variant, the `submitted` arm, the `FieldForm` entry it
  stops being reported under, the widget and callback in `app.slint`, and R-57's
  table if the kind is new to the protocol rather than newly drawn.
  **What it does not promise:** that `text` arrives through this door. F-8 —
  `LineEdit`'s `edited` fires per keystroke into the one-slot channel — means a
  text field likely needs a commit event or a different channel, not another
  `Edited` variant. The seam fixes the shape of a *value*, not the transport
  that carries it, and the design says so to stop a later reader treating
  `Edited::Typed` as settled here.

### 2026-09-14 — a criterion for reviewing how it looks (design §9, AC-10)

- **Asked:** by the user, on reading §9 — *"I want to review the way it looks,
  and provide arbitrary, iterative feedback until it looks acceptable to me.
  That might make sense as the final phase of the slice; or as a separate follow
  up activity."* AC-7 asks a person to observe the **behaviour**, not to judge
  the **appearance**; those are different acts and the second had no criterion.
  Three shapes offered: **A** an unbounded loop as 007's final phase; **B** the
  same review session with changes bounded to legibility, everything else
  captured verbatim as 008's brief; **C** the loop as 008's opening act, with
  007 closing on AC-7.
- **Recommended:** B. An unbounded loop *is* 008 — it has no exit criterion a
  plan can state, it puts layout and protocol changes in one diff, and it gives
  the slice a way not to close. But 007 draws a construct nobody has seen, and
  some of what it produces may be illegible rather than merely ugly, which is
  correctness-adjacent: if a person cannot tell which fields belong to which
  option, AC-4's structural guarantee is invisible to them.
- **Decided:** B.
- **Consequence:** **AC-10** joins the slice card, and the plan gains a final
  phase for it. The bound is on what may be *changed*, never on what may be
  *said*: any feedback is welcome, and feedback beyond legibility is recorded
  verbatim rather than actioned, becoming 008's brief. That is the same path by
  which `~/satan/goad/field-notes.md` produced this slice. B and C compose —
  B's capture is C's input — so choosing B forecloses nothing about 008.
  The phase's exit criterion is statable, which is why it can be planned: *which
  fields belong to which option, and which heading covers which fields, are both
  apparent without reading the protocol, and the person says so.*

### 2026-09-15 — who verifies round 1's repairs (F-1..F-19)

- **Asked:** by the user, on reading the round-2 hand-over — *"Every outcome
  column is blank, and the protocol says `Done` requires each finding `verified`
  or `withdrawn` by **the raiser**."* All nineteen round-1 findings were
  disposed and seventeen changed an artefact, but no outcome was set. Round 1's
  raiser was a session that has ended; the responder setting its own outcomes is
  the both-hats failure the ledger's Protocol names.
- **Decided:** round 2's codex review stands as the verification of round 1. It
  inherits the raiser's role over F-1..F-19 as well as raising its own findings.
- **Consequence:** the ledger carries the rule under *Outcome, and who sets it*
  and `review-design-brief-2.md` makes it job 1's second half. A repair round 2
  accepts is `verified` with round 2 as raiser; one it finds wrong makes the
  round-1 finding `contested` and its substance a new `F-20`-onward finding; one
  it judges was never a defect is `withdrawn`. A repair round 2 does not reach
  keeps an **unset** outcome and is named in *Depth of the round*, so an
  unverified repair stays visible rather than passing as a blank cell. The
  ledger is `Done` only when the column is full — this decides who fills it, not
  that it is full.

### 2026-09-15 — the `Edited` seam: what the compiler actually finds (supersedes the entry of 2026-09-14)

- **Supersedes:** *the extension seam for a second field kind (design §5.2)*,
  above. The **decision** stands unchanged — the `Edited` value enum landed now,
  one variant, with `submitted` as a total match. What was wrong was the entry's
  account of what that buys.
- **Wrong as stated:** that a kind added to SPEC-001/R-16 "is a compile error
  naming that function", and that adding a kind "touches five places the
  compiler finds".
- **Correct:** `submitted` matches `Edited`, which is host-local, so **protocol
  growth leaves it exhaustive** — adding a kind to R-16 breaks nothing. Of the
  five places named, the compiler finds the `Edited` variant and the `submitted`
  arm once a variant is added by hand; it cannot find the `FieldForm` entry, the
  widget and callback, or R-57's table. R-57's enforcement site is real but it
  guards *host-local* growth, not the protocol's.
- **Why this entry exists:** the log is append-only, so the original entry keeps
  its wording; without this, it remains the hand-off's latest account of the
  decision and reintroduces exactly the misunderstanding that was repaired.
  Raised as F-11 in `review-design.md`, and again as **F-27** when the repair
  reached `design.md` and the slice card but not this log — round 1's *Depth of
  the round* had named the log as a second home for the claim.

### 2026-09-15 — `datetime` gets a form (F-21; reverses D3, supersedes the entry of 2026-09-14 on R-57's coverage)

- **Asked:** round 2 raised **F-21**, a blocker: F-5's repair — no host may
  submit a `datetime` value, and MUST report the field undrawn under R-55 —
  makes a conforming renderer that draws the kind R-16 admits impossible. That
  is the protocol taking the shape of the one renderer this slice builds, which
  is `CLAUDE.md`'s third invariant and R-55's "or produce the effect of" clause.
  A second defect stands whichever way the first is read: R-57 cited **R-55 for
  a report R-55 cannot license** — its undrawn report is for a capability *the
  protocol admits and a renderer does not implement*, so where the protocol
  itself forbids the answer, a renderer not drawing it is complying rather than
  exercising a subset.
- **Offered:** **A** define the form now; **B** state a general rule about kinds
  whose form the spec has not defined, so the cause sits in the spec's
  incompleteness rather than in this renderer; **C** contest F-21 and keep the
  prohibition as `tolerated`.
- **Recommended:** A.
- **Decided:** A. `datetime` submits an **RFC 3339 `date-time` string carrying
  an offset**.
- **Consequence:** R-57 types all five kinds; the prohibition, the R-55 citation
  and the "no defined submitted form" clause are gone. OQ-4 survives with a
  smaller question — whether a date without a time wants its own kind. 007 still
  draws no `datetime`, but now reports it undrawn as an ordinary **renderer**
  subset, which is the case R-55 was written for. P-1's obstacle for the kind
  changes from *contract* to *presentation*: the stock offering is two modal
  popups, `DatePickerPopup` and `TimePickerPopup`, with no combined control
  (`i-slint-compiler-1.17.1/widgets/fluent/{datepicker,time-picker}.slint:13`).
  **The reasoning to disagree with**, since the format was picked on no evidence:
  *where a constraint must be chosen blind, choose the one that admits more* — a
  format a backend can work against beats a prohibition nobody can lift without
  amending canon.
- **Supersedes** the entry of 2026-09-14, *does R-57 cover kinds this renderer
  does not draw?*, in one detail only: its decision — cover kinds the renderer
  does not draw — stands and is now stronger, but it is stated there as "all four
  non-`datetime` kinds" and the count is five. D3's own reversal risk also
  changes shape rather than disappearing, from an apparent omission to an
  apparent derivation; §10 states it.

### 2026-09-15 — how wide the §5.1 repair goes (F-32)

- **Asked:** F-32 found §5.1's opening still claiming "nothing new is retained
  outside `Controller`" after F-31 had established that `notice` is. The
  disposition is `doc-wrong` either way; the question was scope.
- **Offered:** **A** the class fix — stop §5.1's opening restating §5.3's
  inventory at all, correct the round-trip count, and put the `Notice` edge in
  the system-model diagram; **B** the false sentence only; **C** A, plus
  rewording P-2's "the controller holds it".
- **Recommended:** A.
- **Decided:** A.
- **Consequence:** §5.3 is now the single home of the state inventory and §5.1
  points at it. The count of window→loop paths goes from one to two. P-2 is
  untouched: its subject is what the person does, and a back-pressure notice is
  not that.
- **Why it was worth the wider edit:** F-32 is the third time in this review that
  a repair corrected the paragraph a finding cited and left the same claim
  standing in another live home (F-22..F-27 established the class; F-30 recorded
  it; F-32 is it again). The instance is one sentence. The cause is that the
  claim had two homes, which `docs/AGENTS.md` warns against directly.

### 2026-09-15 — the design is approved

- **Asked:** `design.md` changed after the last approval — thirty-one repairs
  across two rounds, plus F-32's class repair to §5.1 in the confirmation pass.
  `docs/AGENTS.md` requires the acceptance again.
- **Decided:** approved as it stands, with `review-design.md` resolved at 32
  findings verified, none withdrawn, no blocker outstanding.
- **Consequence:** the slice moves to **plan**. `slice-007.md` Stage is `plan`;
  `plan.md` and `notes.md` are still templates and are the next stage's work,
  with their own ledger (`review-plan.md`) if the user commissions one.

### 2026-09-15 — the design is approved again, after the §5.2 amendment

- **Asked:** `design.md` changed after the 2026-09-15 approval. Planning found
  that §5.2's *The diagnostic lines* named two `Undrawn` lines and no refusal
  line, while §5.4 routes a control that names a field the option does not carry
  to the existing refusal site. `docs/AGENTS.md` §Design requires the acceptance
  again when the design moves after it was given.
- **Decided:** approved as it stands.
- **Consequence:** §5.2 now states the fourth `Refused` variant beside the two
  `Undrawn` lines, with its wording and the reason it lives in `diagnostics.rs`:
  `Diagnostics::refused` is an exhaustive match, so a variant without a line is
  a compile error rather than an omission. It is the one diagnostic in this
  slice a test asserts verbatim —
  `every_refused_variant_renders_one_line_with_the_failure_prefix` — and
  `canon-delta.md` CD-1 carries the R-18 row it implies.
- **Both gates are now clear.** `plan.md` is accepted (`plan-log.md`), and the
  slice moves to **execute**, PHASE-01.

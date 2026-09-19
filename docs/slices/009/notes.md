# Notes — Slice 009

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Handover — the plan is accepted; PHASE-01 is executing

Written 2026-09-18 for a fresh agent, rewritten when the prototype handed back,
again when D-29 … D-32 landed, again when round 4's seven were dispositioned,
again when they were integrated, again when the ledger resolved, again when
the design closed, and again when the plan was accepted. Most of it is now
history: what a phase agent needs is `plan.md`, its own phase sheet, and
§*Traps worth naming*. Delete at the close.

**Durability, as of this handover.** `slice-009-prototype` is pushed through
`a1171b3`. **`main` is not**: everything from `be49327` onward — P-14's
settlement, the design close, the plan, the acceptance — is local only. 33
commits and the whole prototype branch once lived on one disk. Check
`git log origin/main..main` before you finish, and push if the user agrees.

### Where the slice is

Design accepted by the user at draft, then rewritten across four review rounds.
**The review loop is closed.** `review-design.md` reads `**State:** resolved`:
all 56 findings are terminal — 54 verified, 2 withdrawn, no blocker outstanding
— and the Synthesis is written. **P-14 is settled** (D-36, 2026-09-18): the
refusal is not durable, and the question behind it is `SPEC-002/OQ-4`, which
this slice declines. **The design is accepted and closed** — D-37, 2026-09-19,
against `design.md` as it stands at `be49327`. **The plan is accepted**
— `plan-log.md`, 2026-09-19, against `plan.md` as written at `44fbd8e`. Nine
phases; PHASE-04 is the only one that can run beside another. **No plan review
was run**, and `plan-log.md` records what that costs. `slice-009.md` **Stage**
reads `executing`, and **PHASE-01 is in progress**.

The paragraphs below are the arc that produced that, kept because the next agent
needs to know which surfaces have been rewritten and how often. Round 3's
thirteen findings — F-6, F-20, F-26, F-29,
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

**The review loop is still open, and what it is waiting on is outcomes rather
than repairs.** D-29 … D-32 and the prototype's repairs were integrated first
(2026-09-18, a fresh agent per D-21); §*What is owed* item 1 says where each one
landed and what the pass found doing it. **One thing was not applied: P-14**, which is a
decision rather than a repair — settled later as D-36, §*P-14 — settled*. `design.md` was
current truth again after that pass, and the design's function is now
**`interpret`**.

**Round 4 then raised seven, a fresh agent dispositioned them, and a third
session integrated them** (items 2, 2a and 2b). All seven repairs are applied;
the largest retired §5.2's locale account (D-33), reversing D-16 as a net
deletion. **`design.md` is current truth again**, and what is owed on those
seven is one thing: round 4's raiser setting their outcomes.

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
| `review-design.md` | **the ledger.** F-1 … F-56. The first forty-nine are terminal; F-50 … F-56 carry a disposition and a Response and are awaiting an outcome from round 4's raiser. Every Response is written to be complete without the session that wrote it: they are your brief |
| `design.md` | **current truth, through round 4's integration and D-36.** All seven of round 4's repairs are applied (item 2b); §5.2 and §5.5 carry P-14's answer and §8 R5 is restated |
| `design-log.md` | D-1 … D-36. Append-only. D-29 supersedes D-27's second paragraph and **D-33 reverses D-16**. Note the header: `D-n` here is **not** `Dn` in `design.md` §7 |
| `research.md` | Thread 3 is everything measured |
| `canon-delta.md` | CD-1, CD-2. F-45 touches CD-2 |
| `spike-fields/` | **deleted from the tree** at the design close (D-37). In history at `4f93d41`; `git checkout 4f93d41 -- spike-fields` restores all thirteen files |
| `prototype-notes.md` | **the artefact**: P-1 … P-15 in full, with the reasoning. P-15 is withdrawn in place. Lifted onto `main` 2026-09-18 |
| `prototype-handback.md` | the index and the recommendation, and §What it did not test — read that before citing any of it. Lifted onto `main` 2026-09-18 |
| `prototype-delta.md` | the thirteen round-3 findings as P1a was briefed on them. **Historical** from P1b onward — `design.md` is the authority. Lifted onto `main` 2026-09-18 |

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

   **P-14: settled as D-36**, 2026-09-18. See §*P-14 — settled* below; the
   recipe this section used to carry could not have produced the race, and
   working out why is what answered the question.

   **Not touched, deliberately:** `review-design.md` (D-29 — the seventeen stay
   `_pending round 4_`); canon; `design-log.md` (no user decision was taken
   this pass); §5.1's pricing of the `FieldForm` consumers (P-13 is the
   plan's).

2. **Round 4 — raised, and the seventeen closed.** Done 2026-09-18 by a fresh
   Claude agent (D-28); Codex is out of credits and the protocol asks for a fresh
   raiser, not a fresh model. The Brief was written into the ledger **before** the
   review and says what a change of model buys and costs.

   - **All seventeen carry a terminal outcome: `verified`.** None contested, none
     withdrawn. Each Outcome line says what discharges it, and several re-derive
     the Response's citation from the locked source rather than trusting it.
     **F-29 is verified with a recorded residue**: §5.1 and §5.2's `today_local`
     doc comment both name the three impurity sites, so the finding's requirement
     is met, but §5.2's closing prose still reads *"Those two reads — the system
     zone, in `compose`, and the clock, here"*, which is the sentence round 3
     quoted. True of the two *kinds*; misleading as an enumeration of sites. One
     sentence, not worth a third contest.
   - **Seven new findings, `F-50` … `F-56`** — five `major`, two `minor`, **no
     blocker**. They are raised and indexed; **none is dispositioned**. What is
     owed on them, in order: disposition, confirm each with the user, then
     integrate **with a different session** (D-21).
   - **A `## Probed and sound — round 4` list** records eleven things re-derived
     from the locked source this round, including the two that would have been
     blockers had they gone the other way: the guard's convergence write does not
     re-enter `pending.rs`, and the chained pickers never have two popups open at
     once.

   **The shape of what round 4 found.** Three rounds of `gpt-5.6-sol` had checked
   `string_to_float`'s *logic* repeatedly and correctly; none asked **who writes
   its input**. Four of the seven are that class — a claim about a dependency
   that is true of the code and false of the configuration this application
   actually runs in, or an enumeration that is closed on paper and open in the
   source. That is the model change paying off, and it is also the warning: a
   fourth `gpt-5.6-sol` round would probably not have found F-50 or F-52, and this
   round found no citation error at all, which those rounds were very good at.

   **No bad citation was found this round.** Every `design.md` and `canon-delta.md`
   citation checked — `structure.rs:308`, `scan.rs:225-234`, `timers.rs:348-372`,
   `slider-base.slint:117-131`, `fluent/slider.slint:29`, `lineedit.slint:16`,
   `string.rs:398-412`, `items/text.rs:2205-2230`, `canonical.rs:362`,
   `controller.rs:738-739`, `wire.rs:127-133`, `main.rs:85-101`,
   `view_model.rs:31` — is right. §*Citations known bad* still lists three and
   they are all still responders'.

2a. ~~**Disposition `F-50` … `F-56`, with a fresh agent.**~~ **Done,
   2026-09-18**, by a fresh Claude agent (D-21, and the user's call when offered
   the choice between that and a hat-switch). All seven carry a disposition and a
   Response written to be read without the session that wrote it: **six
   `fix-now`, one `doc-wrong`** (F-53). None became a blocker. Every disposition
   was confirmed with the user before it was written down; three of them reverse
   something already decided and are in `design-log.md` as **D-33, D-34 and
   D-35**, cited to the finding ids.

   **F-50, F-51 and F-52 took one repair, not three**, which is what the handover
   said to expect and is the whole difficulty of that surface. The repair
   **retires** §5.2's locale account rather than completing it: the parse rule
   becomes `f64::from_str` and nothing else. See D-33 for the argument. It is a
   net deletion — two paragraphs of §5.2, the *two sets* framing, and the plan's
   numeric grammar with D-32's `e` / `E` constraint, which the rule now discharges
   by itself.

   **What writing the repairs down found**, four things, which is the third time
   this stage has paid for itself:

   - **F-50 was one write site short, and the missing one is the dangerous
     direction.** `mark_all_translations_dirty`
     (`i-slint-core-1.17.1/translations.rs:304-310`) reads
     `sys_locale::get_locale()` and sets the separator from it, under
     `cfg(all(feature = "gettext-rs", target_family = "unix"))`. Compiled out
     here — `gettext` is not among `slint`'s defaults and `gettextrs` is absent
     from `Cargo.lock` — so F-50's conclusion stands. What changes is the shape of
     the hazard: it is one manifest feature away on unix rather than unreachable,
     which is why the repair records a risk (§8 R11) instead of only deleting.
   - **§9's own principal driver bypasses `input-type` too.** A
     `set_accessible_value` assigns `text` and calls `edited` from inside the
     markup (`widgets/fluent/lineedit.slint:16`), so it reaches no `TextInput`
     insertion logic — exactly like the paste path F-52 found. Every numeric case
     the plan writes therefore drives the unvalidated path by default, and the
     design had been reasoning from a class its own cases never exercise. It also
     means F-52's new §9 row costs nothing but a case.
   - **F-51 carries one false generalisation** and the repair must not inherit
     it: `format!("{:e}", 1e300)` is `1e300`, with no separator. `{:e}` emits one
     only for a mantissa that needs one. The finding's worked example (`min: 2.5`)
     is non-integral and stands.
   - **F-54's third consequence does not hold.** `goad-emit` takes `goad-shell`
     without `crates/goad` (`crates/goad-emit/Cargo.toml`), so `-p goad-emit` and
     `-p goad-shell` resolve `jiff` without `std` today and would not if stratum 2
     asked for it. `clock.rs`'s workaround is not dead code kept for an expired
     reason; its reach is narrower than its own comment claims. §10 states three
     reaches, not one.

   **Two things the dispositions found that the design owed anyway**, and both
   land as part of a repair rather than as new findings:

   - **`canon-delta.md` CD-1 has no driver.** Nothing in §9 asserts what an
     untouched field submits per kind — the thing CD-1 promotes to canon. F-56's
     repair gives it one, because splitting AC-2's row produces exactly that case
     as the cheap half.
   - **F-53's exception list cannot be closed.** Four instances now and a fifth on
     a strict reading (a touched field of an option nobody answers is displayed
     and submitted by nothing). That is why the repair deletes the claim rather
     than lengthening the list — third time for this class, after F-21 and F-42.

2b. ~~**Integrate the seven, with a different session.**~~ **Done, 2026-09-18**,
   by a fresh session (D-21). All five repairs are applied to `design.md` and
   `slice-009.md`; `canon-delta.md` needed nothing and canon is untouched. The
   table below is left as it was written, as the index to what landed where.

   **What this pass found**, all of it verification rather than repair — this is
   the first integration in the slice that turned up no defect in what it was
   handed:

   - **One citation was one line off, and this one is the reviewer's.** F-54's
     Evidence line cites `clock.rs:46-52` for the doc comment that refuses
     `jiff::Timestamp::now()`, and the Response repeated it. The comment is at
     **`:47-53`**; `:46` is blank. §10 carries the corrected range. It is the
     fourth entry in §*Citations known bad* and the first not written by a
     responder — so *verify the responder's first* is a priority, not a
     sufficient check.
   - **`clock.rs`'s comment cites a `D25` that is not this design's.** It means
     slice 005's decision; §7 D25 in this design is `interpret`. §10 says which
     one it means rather than reproducing the collision — the `D-n` / `Dn` /
     `P-n` trap, now with a fourth sequence reaching in from another slice.
   - **F-55's "check they are, rather than assuming" checked out.** §5.1's *two
     ways an edit leaves `pending.rs`* enumerates the timer and the `Choose`
     drain, and §5.3's ownership row gives the same two. Neither named
     `released`, so removing the flush leaves both true as written and nothing
     was owed in either place.
   - **Every absence claim the repairs rest on was re-derived from the locked
     source**, because F-50's whole shape is an absence: `set_locale` has
     exactly one caller in the registry (`i-slint-backend-testing`);
     `crates/` names none of `set_locale`, `select_bundled_translation` or
     `with_bundled_translations`; `build.rs` passes only `with_debug_info` and
     `with_style`; `gettext` appears **zero** times in `Cargo.lock`;
     `accept_text_input` has exactly two call sites (`:1067`, `:1117`) with
     `StandardShortcut::Paste` dispatched ahead of both at `:1034`; and
     `goad-emit` takes `goad-semantics`, `goad-shell` and `serde_json` and not
     `crates/goad`. The four separator write sites are at the lines the
     Response gives.
   - **One historical note was struck**, in §*Integration notes the ledger did
     not carry*: round 3's *the overlay creates a property worth stating as an
     invariant* is exactly the generalisation F-53 retires, and it was sitting
     in this file as a reason to put it back.
   - **The repair for F-53 made the same mistake once, in its own first
     draft.** The replacement paragraph said each divergence *"has its own edges
     row below"*; the untouched `datetime` has none — it lives in §5.2, D-6 and
     CD-1. Caught before it was committed, and it is the fourth instance of the
     class in this slice: writing *what is not claimed* is itself an
     opportunity to claim something unchecked.

   The Responses are the brief and are complete; this table is an index to them,
   not a substitute.

   | | lands in |
   |---|---|
   | F-50 / F-51 / F-52 — one repair | §5.2: delete *Parsing the text is done under the rule…*, *The host cannot read that separator…*, *Two sets of texts…* and the `e` / `E` constraint paragraph; add the separator-fact paragraph (four write sites) and the every-string paragraph. §5.5's *numeric text that is not a number* edge row loses its locale variants. §7 **D23** rewritten in place. §8 gains **R11**. §9 gains one row: a numeric text the parse refuses, `set_accessible_value("12/25")`, `tests/renderer/fields.rs` |
   | F-53 | §5.5 **I-H** only. Keep the three-site rule and the drained / kept / stale clauses; delete the *what the screen shows is what an answer would submit* generalisation and the *one place* sentence; add the paragraph saying what is not claimed |
   | F-54 | §10 gains the three-reaches paragraph. `slice-009.md` §Scope gains `crates/goad-shell/src/clock.rs`. §10 states the doc-comment amendment the way §5.3 states `Glass::present`'s |
   | F-55 | §5.2's controls table row and `Slider` paragraph; §7 **D7** rewritten in place. §5.1's *two ways an edit leaves `pending.rs`* and §5.3's ownership row become true as written — **check they are, rather than assuming** |
   | F-56 | §9: AC-2's row becomes two. Nothing in `slice-009.md` AC-2 changes |

   **Watch for**, because each has caught a session in this slice already:

   - **A claim repeated in two places is repaired in one.** P-7 and D-31 were both
     owed in §5.5's edges table as well as in §5.2. F-50's repair touches §5.2 and
     §5.5; grep for *separator*, *locale*, *two sets*, *grammar* across
     `design.md` before calling it done, and for *released* on F-55's.
   - **No production line under `crates/goad/src` may name `resolve`**
     (`goad-boundary/tests/checks/structure.rs:308`). Nothing in these repairs
     introduces a name, but check anything you do introduce against
     `vocabulary.rs:18-25`, `purity.rs:17-27` and `structure.rs`'s three call-form
     greps.
   - **§7 rewrites an entry in place under its own id; `design-log.md` never
     does.** Two entries are rewritten here, D23 and D7.
   - **`design.md` carries no prototype `P-n`.** D-32's rationale in the log cites
     *the grammar P-5 pinned*; there is no longer a grammar, so nothing in
     `design.md` should cite one.

2c. ~~**Round 4's raiser sets the outcomes on F-50 … F-56.**~~ **Done,
   2026-09-18** (`68946ef`), by a fresh agent: a bounded verification pass on the
   repairs, not a round 5. **All seven `verified`**, none contested, no new
   findings. Corrected in `aeef6ab` — the raiser's own two line numbers were
   wrong and are struck in place; no outcome changed. That is the fifth entry in
   §*Citations known bad* and the rule it produces is in the Harvest.

2d. ~~**Write the Synthesis and resolve the ledger.**~~ **Done, 2026-09-18**
   (`f985acc`). `review-design.md` §Synthesis carries the closure story and the
   header reads `**State:** resolved`. Item 4 onward follows.


3. ~~**Bring the prototype's record back.**~~ **Done, 2026-09-18.**
   `prototype-notes.md`, `prototype-delta.md` and `prototype-handback.md` are on
   `main` in this folder, copied verbatim from `slice-009-prototype`. The code is
   **referenced, not promoted** — the slice re-derives from its own plan. The
   branch may now be retired; nothing on it is needed but the twelve commits of
   code the handback indexes, and those are referenced by hash.

4. ~~**Re-ask the user for acceptance.**~~ **Done, 2026-09-19** (D-37). The
   design had changed four times since theirs — rounds 2, 3 and 4's integrations
   and D-36. Accepted as it stands at `be49327`. `slice-009.md` **Stage** now
   reads `plan`.

5. ~~**Plan**, with a fresh agent.~~ **Drafted 2026-09-19.** `plan.md` carries
   nine phases; the Coverage table discharges all ten acceptance criteria.

   Three of its inputs, and what each did to it:

   - **P-13** — the undrawn fixture migrates once per phase. Priced: each kind
     phase carries a migration exit criterion (PHASE-05/EX-9 and its references),
     and PHASE-09 owns the deletion, which is why `choice` is last.
   - **D-36** — the refusal is not durable. Absorbed into PHASE-05/VT-4: a
     `Choose` carrying two stale edits reports **once**, not twice, and the answer
     still goes.
   - **The handback's §Recommendation** — `datetime` first. Taken, but narrowed
     to *first among the kinds that remain after `text`*, and the disagreement is
     argued in `plan.md` §*Sequencing & rationale*: `text` is what forces
     `pending.rs` and the overlay, which every other kind's display depends on.

   **Accepted 2026-09-19**, and the adversarial plan review was declined —
   `plan-log.md` carries both, and what the second one costs. `review-plan.md`
   does not exist and is not owed.

   **The live item is execution**, phase by phase. The status table below is the
   record, and a phase sheet is written immediately before its phase runs, never
   earlier (`docs/AGENTS.md` §*Phase plan*: a sheet written three phases early is
   fiction).

   **Handed back rather than repaired** (`docs/AGENTS.md` — the plan stage does
   not repair the design): one bad citation in `design.md` §8 R5, in
   §*Citations known bad* as the sixth entry. Nothing else in the design failed
   verification against the code; the spot-check is in the plan-stage report.

6. ~~**Delete `spike-fields/`**~~ **Done, 2026-09-19**, with the design close
   (D-37). It is in history at `4f93d41` and nothing that cites it loses its
   warrant: every citation is to a measurement already written down.

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
| PHASE-02 — the draft's five values, and the kind-directed pure functions | pending | |
| PHASE-03 — the edit channel | pending | |
| PHASE-04 — the instant, the `jiff` feature, and `clock.rs`'s doc | pending | |
| PHASE-05 — `text`, and the debounce's delivery | pending | |
| PHASE-06 — the overlay | pending | |
| PHASE-07 — `datetime` and the two pickers | pending | |
| PHASE-08 — `number` and its two controls | pending | |
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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-19 · PHASE-01 done · see §Status

### Produced
<!-- What now exists: modules, contracts, docs. -->

- **The two channels.** `ui/app.slint` carries `Kind`,
  `FieldRow { id, label, kind, slot }` and
  `FieldValue { checked, text, number, index }`, with `values` and `epoch` on
  the window root. `glass.rs::option_models` builds both in one pass, so I-B is
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
- **A boundary instrument can forbid a word the next slice wants** (P-10).
  `scan::mentions` word-matches after splitting on non-alphanumerics and camel
  boundaries, and keeps string literals, so an identifier or a diagnostic message
  can red a purity instrument that has no view on either. Check the boundary
  suite's needles before naming a new function.

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

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
- **Follow-up: what a supersession costs is not what R5 said it was.** A
  superseded view clears the field under the caret and loses everything typed
  into it, because `Command::Edit` mutates the retained draft and never reaches
  the backend (`controller.rs:661-675`). The row said *widens the window* and
  named diagnostic-pane noise as the signal. Restated in place. Nothing in the
  slice changes; the audit should read the row as written now.

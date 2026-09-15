# Review — plan — Slice 007

**Subject:** plan — `docs/slices/007/plan.md`, read against `design.md`,
`canon-delta.md`, `slice-007.md` and the code it plans to change.
**Reviewer:** fresh Claude agent (Opus 5), spawned in-session
**Opened:** 2026-09-15
**State:** resolved

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

Written before the review. One round, by user decision (`plan-log.md`,
2026-09-15).

**What this review is for.** `design.md` is approved and `review-design.md` is
resolved at 32/32 verified; the design is **not** under review here. What is
under review is whether `plan.md` would, if executed exactly as written, produce
the design — and whether any phase could be declared done while the thing it
claims to establish is false.

**The invariants the plan is held to.**

1. **Coverage is total and honest.** Every one of `slice-007.md`'s ten
   acceptance criteria is discharged by a named criterion in a named phase, and
   the named criterion actually reaches the observable `design.md` §9 gives for
   that AC. A criterion that could be met without the behaviour existing is the
   defect.
2. **No criterion is a proxy.** `design.md` §9: every field test either reads
   the wire or asserts something about the screen — except AC-5, which needs
   both. A `VT-` that reads the draft through `Controller` and stops there is
   the slice-004 scar repeating
   (`docs/memory/a-green-test-can-assert-a-proxy.md`).
3. **Entry criteria prove the previous phase done**, and exit criteria make the
   phase's objective true. A phase whose exit is weaker than its objective can
   be closed green on nothing.
4. **Surfaces are complete.** Undeclared paths are what the audit hunts for. A
   phase that must touch a file its Surfaces list does not name is a defect in
   the plan, not in the phase.
5. **Sizing.** One phase, one agent, one session, bookkeeping included.
6. **The plan takes no decision that is not its to take.** AC-8's promotion is
   audit's; the look beyond what drawing fields forces is 008's; the six open
   questions and D1–D13 are closed; SPEC-002/OQ-4 stays deferred.

**Where the bodies are likely buried.**

- **PHASE-01/EX-7's enumeration of the notice rule's live homes.** The design
  review's own synthesis names this class: a repair lands where it was cited and
  the claim stays live somewhere else
  (`docs/memory/a-repair-sweep-misses-the-binding-site.md`). Eight sites are
  listed. Is the list complete, and is any entry on it wrong?
- **PHASE-01/EX-4**, which removes `Wire`'s window handle. That consequence is
  not stated in `design.md`; the plan argues it is forced by `dead_code` under
  the gate's `-D warnings`. Is that true, and is its blast radius correctly
  bounded?
- **The 02/03/04 split.** Does anything in PHASE-02 or PHASE-03 actually require
  something only PHASE-04 provides, so that the phase cannot end green?
- **AC-6.** The claim is that the only existing assertions that change in the
  whole slice are `mapper.rs:156-194`'s (PHASE-03/VA-2) and `wiring.rs:346-385`'s
  (PHASE-01/VT-1). Is any other existing assertion or fixture forced to change
  by a phase's exit criteria?
- **AC-5 (PHASE-05/VT-3).** Both halves are required and neither implies the
  other. Does the criterion as written actually reach both?
- **PHASE-05's helper lift** of `logging_scripted` out of `scheduling.rs`. Does
  it breach S-2, and is `harness.rs` the right home under that file's own stated
  rule?
- **What no instrument reaches.** S-6 and PHASE-06/VA-1 both claim stratum 3's
  purity is held by review alone. Does any criterion anywhere in the plan imply
  an instrument checks something it does not?

**Round 1** — 2026-09-15 — the whole of `plan.md`, against the six invariants
above.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | doc-wrong | verified |
| F-2 | major | doc-wrong | verified |
| F-3 | major | doc-wrong | verified |
| F-4 | minor | doc-wrong | verified |
| F-5 | minor | doc-wrong | verified |
| F-6 | minor | doc-wrong | verified |
| F-7 | minor | doc-wrong | verified |
| F-8 | minor | doc-wrong | verified |
| F-9 | minor | doc-wrong | verified |
| F-10 | minor | doc-wrong | verified |
| F-11 | nit | doc-wrong | verified |
| F-12 | nit | doc-wrong | verified |
| F-13 | nit | doc-wrong | verified |

<!-- Round 1's findings are appended here as they are raised. -->

**Environment note.** Run unjailed at `/home/david/dev/goad`. Slint facts below
are cited from the pinned crates at
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/i-slint-{core,compiler,backend-testing}-1.17.1/`.
Every `path:line` the plan relies on for a claim I tested was opened; where a
citation is off by a line or two but names the right construct I have not raised
it, and F-13 collects the two that mislead.

### F-1 — PHASE-04 cannot be executed inside its declared Surfaces: `Refused::UnknownField` forces `diagnostics.rs`, whose wording nothing states and whose own test claims to cover every variant

**Severity:** blocker
**Location:** `plan.md` PHASE-04 *Surfaces*, EX-2, VA-1; and the Coverage table's AC-6 row

**Expected:** PHASE-04's Surfaces are
`crates/goad/src/{reception.rs, controller.rs, wire.rs, install.rs, glass.rs}`
and `crates/goad/tests/renderer/wiring.rs`. VA-1 states that `git diff` over
`table.rs`, `wiring.rs`, `scheduling.rs`, `ingress.rs` and **`reception.rs`**
shows no changed assertion and no changed fixture. `docs/AGENTS.md` §Execute:
"Stay inside the phase's declared surfaces. Touching anything else is either a
design change or scope creep — in both cases, stop and ask."

**Observed:** EX-2 requires `Refused::UnknownField`. Three things follow that
the phase cannot do within those surfaces:

1. `Refused` is declared in `crates/goad/src/diagnostics.rs:55-68`, not in
   `controller.rs` — deliberately, and the enum's own doc says so: *"It lives
   here rather than beside the controller … every user-visible string in this
   renderer is in one file. The controller names it; it does not own it."*
   `diagnostics.rs` is **not** in PHASE-04's Surfaces (it is in PHASE-03's,
   where nothing needs it).
2. `Diagnostics::refused` (`diagnostics.rs:148-163`) is an exhaustive match over
   `Refused` with no `_` arm, so a fifth variant is a compile error there. The
   phase must therefore author a new **user-visible string**. No design section
   states it: `design.md` §5.2's *diagnostic lines* block gives verbatim wording
   for the two `undrawn_line` cases only, and §5.5's edge-case table says no more
   than "`Refused::UnknownField`, reported". PHASE-03/EX-5's stated principle —
   "the wording is stated in the design so it is reviewed once rather than
   discovered in a diff" — is exactly what fails here.
3. `crates/goad/tests/renderer/reception.rs:624-652` is
   `every_refused_variant_renders_one_line_with_the_failure_prefix`, which
   asserts the rendered line of each variant it names. Adding `UnknownField`
   either leaves that test's name false and its coverage silently incomplete, or
   forces a change to `reception.rs` — which VA-1 asserts does not happen, and
   which S-2 forbids outside the two places the plan names. AC-6's claim (the
   Coverage table, and `slice-007.md` AC-6) that the only existing assertions
   changed in the whole slice are `mapper.rs:156-194`'s and `wiring.rs:346-385`'s
   is false either way.

**Evidence:**
- `crates/goad/src/diagnostics.rs:47-68` — `Refused`'s declaration and its
  "lives here rather than beside the controller" doc.
- `crates/goad/src/diagnostics.rs:148-163` — `Diagnostics::refused`'s exhaustive
  match, four arms, no `_`.
- `crates/goad/tests/renderer/reception.rs:624-652` — the test, asserting
  `SupersededView`, `UnknownOption` and `NoClock` by their exact strings under a
  name that says *every* variant (`Ingress` is already outside it; a fifth
  variant widens the gap).
- `plan.md` PHASE-04 *Surfaces* (no `diagnostics.rs`), EX-2, VA-1.
- `design.md` §5.2 *The diagnostic lines*, §5.5 *Edge cases* row
  "edit naming a field the option does not declare".

**Disposition:** doc-wrong
**Response:** Confirmed at every leg before dispositioning, not taken on the
report: `Refused` is declared at `diagnostics.rs:54-68` with the "every
user-visible string in this renderer is in one file" doc; `Diagnostics::refused`
(`:148-163`) is a four-arm exhaustive match with no `_`; and
`reception.rs:624-652` asserts three variants by exact string under a name that
says every one.

The plan was wrong in three places and all three are repaired. PHASE-04's
**Surfaces** gain `crates/goad/src/diagnostics.rs` and
`crates/goad/tests/renderer/reception.rs`, with the reason `Refused` lives in
that file quoted at the point of declaration. **EX-8** adds the variant and its
`Diagnostics::refused` arm. **EX-9** extends
`every_refused_variant_renders_one_line_with_the_failure_prefix` with a fourth
case, and **VT-6** asserts the line. **VA-1** stops claiming `reception.rs` is
untouched.

The wording was the part that was not the plan's to settle. `design.md` §5.2
states every other diagnostic wording verbatim so it is "reviewed once rather
than discovered in a diff", and this one is the same class. **`design.md` §5.2
is amended** with the line — `no action taken: the host could not match that
control to a field of the option it names`, parallel to `UnknownOption`'s and
distinguishable from it — by explicit user decision, 2026-09-15
(`plan-log.md`). That is a change to an approved design and is flagged for
re-approval.

AC-6 is **not** breached: `design.md` §9 scopes its "no change to what they
assert" to the option tests — `tree.rs`, `table.rs`, `wiring.rs`. S-2 gains this
as its third named allowance, and the Coverage table's AC-6 row says so.

**Outcome:** verified

---

### F-2 — PHASE-02/EX-3 gives the per-option container the same `accessible-description` as the option's button, so `element_described`'s "only unambiguous" identity stops being unambiguous, and two existing assertions become order-dependent

**Severity:** major
**Location:** `plan.md` PHASE-02/EX-3, EX-6, VA-1; PHASE-01/EX-7 (the class); the Coverage table's AC-6 row

**Expected:** AC-6 and PHASE-02/VA-1: the five existing cases in `tree.rs`
change call shape only, with no changed assertion. EX-6 adds an option-scoped
query *beside* `element_described` (`tree.rs:42-49`) and leaves it alone.

**Observed:** EX-3 requires the per-option container to carry
`accessible-description: option.id`. The option's `Button` already carries
`accessible-description: option.id` (`ui/app.slint:51`). After the change two
elements in the tree answer to the same description, and
`element_described`'s `find_first()` takes whichever the walk reaches first —
which no criterion, and no line of markup the plan specifies, pins.

Two existing assertions rest on it returning the **Button**:

- `tree.rs:96-99` — `element_described(&window, id)` then
  `assert_eq!(button.accessible_item_index(), Some(index))`. The container
  declares no `accessible-item-index`, so if it is reached first this is
  `None != Some(index)` and the case goes red. Its sibling assertion on
  `accessible_label` would *not* catch the swap, because EX-3 gives the
  container `accessible-label: option.label` too.
- `tree.rs:124-125` — `element_described(&window, "opt-b")` then
  `invoke_accessible_default_action()`. A groupbox container has no default
  action, so if it is reached first `chosen` never fires and the case goes red.

This is the `a-repair-sweep-misses-the-binding-site` class at the level the plan
is supposed to close: `tree.rs:40-41`'s doc — *"The identity the tests select
on, and the only unambiguous one (D10, R-14): never by label, which two options
may share"* — becomes false, and it is not on any phase's list of live homes.
`ui/app.slint:49-51`'s comment carries the same claim and is likewise unnamed.
The plan's own *Notes for the implementer* under PHASE-02 argue at length that
an unscoped query "takes whichever comes first and reports no ambiguity" — for
field ids — and then creates that condition for option ids without noticing.

The option-**scoped** query of EX-6 survives this, which is worth stating so the
repair is not aimed at the wrong place: `find_first` passes
`ControlFlow::Break` only after a *complete* stack match, so a `Button` that
matches `option.id` but has no matching descendant yields `Continue` and the
walk carries on to the container
(`i-slint-backend-testing-1.17.1/search_api.rs:178-213`, `:288-300`). The damage
is confined to the unscoped helper and the two cases that use it.

**Evidence:**
- `crates/goad/ui/app.slint:49-51` — the Button's `accessible-description: option.id`
  and the comment claiming it is the unambiguous identity.
- `crates/goad/tests/renderer/tree.rs:40-49` — `element_described` and its
  "only unambiguous one" doc.
- `crates/goad/tests/renderer/tree.rs:95-100`, `:124-130` — the two dependent cases.
- `plan.md` PHASE-02/EX-3 ("carrying `accessible-role: groupbox`,
  `accessible-description: option.id` and `accessible-label: option.label`"),
  VA-1.
- `i-slint-backend-testing-1.17.1/search_api.rs:166-213` (`match_recursively`),
  `:288-300` (`find_first` / `find_all`).

**Disposition:** doc-wrong
**Response:** Verified independently: `app.slint:51` carries
`accessible-description: option.id` on the option's `Button`, and `design.md`
§5.2 puts the same description on the per-option container. Two elements, one
description, and `find_first()` unpinned between them.

The finding is right that the repair belongs on the **unscoped helper** and not
on the scoping scheme, and right that the scoped query of EX-6 survives — so the
design's identity scheme is not amended. PHASE-02 gains **EX-7**:
`element_described` takes a `match_inherits("Button")` filter before its
predicate, the shape `tree.rs:81-83` already uses, and the two prose homes of the
old uniqueness claim move with it — `tree.rs:40-41` and `app.slint:49-51`, which
the compiler reaches neither of.

A test **helper's implementation** is neither an assertion nor a fixture, so S-2
holds; the allowance is now stated there rather than left to be argued.

The finding's closing observation is the one worth keeping: PHASE-02's own Notes
argued that an unscoped query "takes whichever comes first and reports no
ambiguity" for field ids, and then created that condition for option ids. The
argument was present and was not applied one level up.

**Outcome:** verified

---

### F-3 — AC-2's on-screen half is discharged in PHASE-02 against a model the test builds, so the host code that actually puts declared order and headings on the screen has no verification criterion in any phase

**Severity:** major
**Location:** `plan.md` Coverage table, AC-2 row; PHASE-02/VT-4; PHASE-04/EX-6

**Expected:** invariant 1 — the named criterion reaches the observable
`design.md` §9 gives for that AC. §9's AC-2 observable is "a heading is drawn
where the `group` value changes, and the drawn order equals the declared order",
where *declared* means declared by the backend.

**Observed:** the Coverage table maps AC-2 to PHASE-03/VT-3 (the rule, as a pure
`present()` test) and PHASE-02/VT-4 ("declared order as it reaches the screen,
read from tree order"). PHASE-02 runs before `Presentation` has blocks at all —
`view_model.rs`'s `FieldBlock` does not exist until PHASE-03 and
`glass.rs::option_rows` does not build one until PHASE-04/EX-6 — so VT-4's
"declared order" is the order of a `ModelRc<OptionRow>` the test constructs by
hand in `tree.rs`. The chain is verified in two halves that do not meet:
`View → present()` by VT-3, `row model → screen` by VT-4, and
`present() → row model` — `option_rows`, the only host code that puts a
backend's declared order and its headings on the screen — by nothing.

Two specific behaviours PHASE-04/EX-6 states have no criterion anywhere:

- that `option_rows` emits one `FieldBlock` per presentation block **in order**,
  and each block's `FieldRow`s in declared order. A transposition here is caught
  by no phase; PHASE-04/VT-5 asserts one field's `checked`, PHASE-05/VT-1
  asserts three keys on the wire without asserting order, and VT-2 asserts
  option scoping.
- that `heading: None` renders as `""` — the distinction on which "an ungrouped
  field is not drawn under a heading that does not claim it" (`design.md` §5.2)
  rests. A bug that renders `None` as a literal, or drops the block, reaches
  only PHASE-06's human run.

**Evidence:**
- `plan.md` Coverage, AC-2 row; PHASE-02/VT-4; PHASE-02 *Surfaces*
  (`build.rs`, `app.slint`, `glass.rs` mechanically, `tree.rs`).
- `plan.md` PHASE-02/EX-5 — in PHASE-02 `option_rows` gets
  `blocks: ModelRc::default()` "and nothing else", so it builds no block.
- `plan.md` PHASE-04/EX-6 — the mapping, stated as an exit criterion with no
  matching VT.
- `crates/goad/src/glass.rs:138-149` — `option_rows` today.
- `design.md` §9, AC-2 row.

**Disposition:** doc-wrong
**Response:** Correct, and the sharpest coverage defect in the ledger. The chain
has three links — `View → present()`, `present() → row model`, `row model →
screen` — and the Coverage table named two.

PHASE-04 gains **VT-7** over `option_rows` with no window: two blocks, one titled
and one `heading: None`, over five fields in declared order, asserting block
order, field order within each block, and `""` for the untitled heading. The
Coverage table's AC-2 row now cites all three links and says why three.

**Outcome:** verified

---

### F-4 — PHASE-01/EX-3 falsifies `wire.rs:120-124`, which is not on EX-7's list and which VA-2's `grep -rin notice` cannot reach

**Severity:** minor
**Location:** `plan.md` PHASE-01/EX-3, EX-7, VA-2

**Expected:** EX-7 enumerates "**every live home** of the old rule … so the
sweep cannot stop early", and VA-2 reads
`grep -rin notice crates/goad/src crates/goad/ui crates/goad/tests` against that
list with "the residue accounted for line by line".

**Observed:** EX-3 splits the arm `Closed` shares with `Ok(())`. The paragraph
that explains why they share one is `wire.rs:120-124`: *"`Closed` does nothing,
deliberately (F-20) … **It shares one arm with `Ok(())`** — `clippy::match_same_arms` —
so the pattern stays named rather than swept into a wildcard."* After EX-3 that
sentence is false. It is not on EX-7's list, and it contains no occurrence of
"notice", so VA-2's instrument cannot surface it — which is precisely the
binding-site failure EX-7 exists to prevent
(`docs/memory/a-repair-sweep-misses-the-binding-site.md`).

**Evidence:**
- `crates/goad/src/wire.rs:120-124` — the paragraph.
- `crates/goad/src/wire.rs:125-134` — the two-arm match EX-3 splits.
- `plan.md` PHASE-01/EX-3, EX-7 (eight entries, none of them this one), VA-2.
- `grep -rin notice crates/goad/src crates/goad/ui crates/goad/tests` returns no
  line in `wire.rs:119-124`.

**Disposition:** doc-wrong
**Response:** Verified: `wire.rs:120-124` states the shared arm as a deliberate
choice, EX-3 splits it, and `grep -rin notice` returns nothing in that span.
Appended to EX-7 as a ninth home, with the note that VA-2's instrument cannot
reach it — which is the whole reason EX-7 is a list.

**Outcome:** verified

---

### F-5 — PHASE-01/EX-4's blast radius misses `glass.rs:36-38`, whose doc cross-references the `Wire` `Debug` impl EX-4 deletes

**Severity:** minor
**Location:** `plan.md` PHASE-01/EX-4

**Expected:** EX-4 bounds the consequence of removing `Wire`'s window handle
explicitly — the field, `Wire::new`'s parameter, the `crate::generated` import,
the hand-written `Debug`, the doc paragraph, and six `Wire::new` call sites. The
brief asks whether that blast radius is correctly bounded.

**Observed:** `SlintGlass`'s own hand-written `Debug` is documented by reference
to `Wire`'s: *"Hand-written for the same reason `Wire`'s is: the generated
component handles carry no `Debug` and `missing_debug_implementations` is
`deny`"* (`glass.rs:36-38`). Once `Wire` carries a derived `Debug` and names no
generated type, that reference points at nothing. `glass.rs` is in PHASE-01's
Surfaces, so the fix is in scope — but the site is not enumerated, and like F-4
it contains no "notice", so VA-2 cannot find it.

The rest of EX-4 checks out. `Wire::new` has exactly six call sites
(`src/main.rs:88`, `src/wire.rs:203`, `:212`, `tests/renderer/wiring.rs:364`,
`tests/event_loop/closing.rs:55`, `tests/event_loop_schedule/scheduling.rs:82`);
`crate::generated` is named in `wire.rs` only at `:17` for that field; and the
gate's `lint` recipe is `cargo clippy --workspace --all-targets -- -D warnings`,
so a never-read private field is an error rather than a warning.

**Evidence:**
- `crates/goad/src/glass.rs:36-38`.
- `crates/goad/src/wire.rs:72-89` — `Wire`'s doc paragraph and hand-written `Debug`.
- `justfile` `lint` recipe; `docs/policy/001-the-phase-gate.md` §Verification.
- `grep -rn "Wire::new" crates/` — six sites, as EX-4 says.

**Disposition:** doc-wrong
**Response:** Verified: `glass.rs:36-38` documents `SlintGlass`'s hand-written
`Debug` by reference to `Wire`'s, and EX-4 deletes the referent. Appended to EX-7
as a tenth home, scoped precisely — `SlintGlass`'s own `Debug` stays
hand-written and the reason stays true of it; what goes is the "same reason
`Wire`'s is" clause.

The rest of EX-4's blast radius was checked and confirmed, and is now enumerated
in PHASE-01's Notes rather than left as a claim.

**Outcome:** verified

---

### F-6 — PHASE-02/EX-3 and EX-4 describe incompatible markup, and the guard that reconciles them is named nowhere

**Severity:** minor
**Location:** `plan.md` PHASE-02/EX-3, EX-4, VT-5

**Expected:** an implementer can satisfy EX-3 and EX-4 from the plan without
taking a decision of their own.

**Observed:** EX-3 says "**one container per option** inside the option's row,
carrying `accessible-role: groupbox`, `accessible-description: option.id` and
`accessible-label: option.label`". EX-4 says "an option with no fields has
`blocks: []` and **the `for` produces nothing: no container**, no separator, no
stray control". A container that is per-option is not produced by the `for` over
`blocks` and therefore exists whether `blocks` is empty or not; a container the
`for` produces is per-**block**, and then `option.id` and `option.label` are
carried once per block rather than once per option. Only an explicit
`if option.blocks.length > 0` guard makes both true, and neither criterion nor
the design names one. Written literally, EX-3 yields an empty groupbox for a
zero-field option, which VT-5 then fails and which `slice-007.md` AC-6 names
outright ("no empty container").

**Evidence:**
- `plan.md` PHASE-02/EX-3, EX-4, VT-5.
- `design.md` §5.2 — "the **per-option container** carries
  `accessible-description: option.id`" and, ten lines later, "An option with no
  fields has `blocks: []`, so the `for` produces nothing: no container".
- `slice-007.md` AC-6.

**Disposition:** doc-wrong
**Response:** Correct: a per-option container is not produced by the `for` over
`blocks`, so written literally EX-3 yields an empty groupbox for a zero-field
option and EX-4 and VT-5 both fail. `design.md` §5.5's edge-case table settles
the intent — "a bare button, exactly as today" — so this is the plan failing to
carry a decision the design already took, not an open question.

EX-3 gains the explicit `if option.blocks.length > 0` guard and EX-4 names the
guard as what makes the container absent rather than empty, with the per-option
versus per-block reasoning stated so the next reader does not re-derive it.

**Outcome:** verified

---

### F-7 — the Coverage table says every phase's exit criterion is `just check` exit 0; no phase's Exit list contains it

**Severity:** minor
**Location:** `plan.md` Coverage table, AC-9 row; PHASE-01..PHASE-06 *Exit*

**Expected:** invariant 3 — exit criteria make the phase's objective true, and a
phase is not done until the gate is green (`docs/AGENTS.md` §Execute: "End
green: the phase's exit and verification criteria discharged").

**Observed:** the Coverage table discharges AC-9 with "every phase's **exit**
criterion is `just check` exit 0". Reading the six Exit lists, none of the
thirty-eight `EX-` criteria says anything about `just check`. Green appears only
as the *entry* criterion of the following phase (`EN-1` of PHASE-02..PHASE-06)
and, for PHASE-01, as a green on the base commit before anything is edited. Two
consequences: phases 01–05 can each be declared done with every EX and VT
discharged and the gate red, the failure surfacing only when the next agent runs
its entry check; and PHASE-06, having no successor, is covered only by VA-1.

**Evidence:**
- `plan.md` Coverage, AC-9 row.
- `plan.md` PHASE-01 EX-1..EX-8, PHASE-02 EX-1..EX-6, PHASE-03 EX-1..EX-7,
  PHASE-04 EX-1..EX-7, PHASE-05 EX-1..EX-4, PHASE-06 EX-1..EX-6 — no occurrence
  of "just check" in any of them.
- `plan.md` PHASE-01/EN-1 and PHASE-02..06/EN-1 — where green actually appears.
- `docs/AGENTS.md` §Execute.

**Disposition:** doc-wrong
**Response:** Correct and consequential: the Coverage table claimed a criterion
that appeared in no Exit list, so phases 01–05 could each be declared done with
the gate red and the failure would surface only at the next agent's entry check.

Each phase's Exit list now ends with `just check` exits 0 — PHASE-01/EX-9,
02/EX-8, 03/EX-8, 04/EX-10, 05/EX-5, 06/EX-7 — and the Coverage row names them
individually rather than asserting a property of "every phase".

**Outcome:** contested — the repair was right in the ledger and wrong in the
artefact. The Response named the six criteria correctly (`02/EX-8`), but the
rewritten AC-9 cell at `plan.md:139` still read
*"(PHASE-01/EX-9, **02/EX-7**, 03/EX-8, 04/EX-10, 05/EX-5, 06/EX-7)"*.
PHASE-02/EX-7 is the `element_described` disambiguation; its `just check`
criterion is **EX-8**. The cell therefore went on misstating the plan's own Exit
lists — one citation of six rather than a property of "every phase", but the
same false claim in the same cell. Returned to open for that one token; see the
re-disposition below and the terminal outcome under it.

**Re-disposition (responder, 2026-09-15):** `doc-wrong`, upheld and now
actually applied. The contest is correct: the Response named `02/EX-8` while
`plan.md`'s AC-9 cell still read `02/EX-7`, so the cell went on asserting
something false about the plan's own Exit lists — the defect F-7 raised,
surviving its own repair.

Fixed, together with two more of the same kind that the verification pass
surfaced and did not raise separately (one round, by user decision). All three
are casualties of the PHASE-02 EX-9/EX-10 → EX-7/EX-8 renumber, and none was
re-read against the text that cites it:

- AC-9's cell: `02/EX-7` → `02/EX-8`.
- S-2's helper allowance: `(PHASE-02/EX-6)` → `(PHASE-02/EX-7)`. EX-6 adds a new
  scoped query and needs no allowance; EX-7 is the criterion that changes an
  existing helper, and it cites S-2 back.
- PHASE-01's *Notes*: "EX-7's **eight** prose sites" → "**ten**", four lines
  below the list the F-4/F-5 repairs extended to ten.

Worth recording plainly rather than fixing quietly: this ledger's dominant class
is a claim staying live where the instrument that checks it cannot see it, and
the repairs for that class introduced three more instances of it. A renumber is
a claim change, and every citation of a renumbered id is one of its live homes
(`docs/memory/a-repair-sweep-misses-the-binding-site.md`).

**Outcome:** verified. All three fixes checked at their sites —
`plan.md:139` now reads `02/EX-8`, `plan.md:100` now reads `(PHASE-02/EX-7)`,
and `plan.md:263` now reads "EX-7's **ten** prose sites".

Each referent re-derived from the artefact rather than from the re-disposition,
and the whole id space swept mechanically rather than spot-checked, because a
renumber is exactly the edit a spot-check survives:

- Every phase's last `EX-` is the one AC-9's cell names: PHASE-01/EX-9,
  02/EX-8, 03/EX-8, 04/EX-10, 05/EX-5, 06/EX-7. Six for six.
- PHASE-02/EX-7 is the `element_described` change, which is the criterion that
  needs S-2's helper allowance; PHASE-02/EX-6 adds a new query and needs none.
- PHASE-01/EX-7 carries exactly ten sub-bullets.
- Every `PHASE-0N/<id>` cross-reference in `plan.md` resolves to a criterion
  that exists in that phase — no unresolvable reference anywhere in the file —
  and no phase's id sequence has a gap or a duplicate.

The re-disposition's closing paragraph is the part worth keeping: a renumber is
a claim change, and every citation of a renumbered id is one of its live homes.
That is the ledger's own dominant class, and it reached the repairs for it.
---

### F-8 — PHASE-03 falsifies SPEC-001 §7's R-18 verification row, and neither `canon-delta.md` nor any phase names it

**Severity:** minor
**Location:** `plan.md` PHASE-03/EX-6; `canon-delta.md` CD-1 *Sections*

**Expected:** EX-6 cites R-18 as permitting the read — "It reads the `group`
hint, which R-18 permits the renderer and only the renderer to do" — and the
slice's canon impact is carried by `canon-delta.md`, promoted at audit.

**Observed:** R-18 is not only a requirement; it has a verification row in
SPEC-001 §7 that states two facts about this codebase, both of which PHASE-03
makes false:

> **R-18** | **review, not a test** … `hints` is read in `src/` only by
> `normalize.rs::normalize_field`, where the remaining keys are collected and
> passed through. … **The renderer, the one component that may, does not exist
> yet.**

After PHASE-03, `hints` is read in `crates/goad/src/view_model.rs` as well, and
the renderer exists. `canon-delta.md` CD-1 names §4, §6.1, §6.2, §7 (*"one row
per new requirement"*) and §8 — the §7 work is scoped to **adding** R-57's and
R-58's rows, not to amending R-18's. `design.md` §10's canon-impact table does
not name it either. Nothing in the plan makes the discrepancy reachable for
audit's reconciliation.

**Evidence:**
- `docs/specs/001-host-backend-protocol.md:382` — the R-18 verification row,
  verbatim.
- `plan.md` PHASE-03/EX-6.
- `canon-delta.md` CD-1, **Sections**.
- `design.md` §10, SPEC-001 §7 row.

**Disposition:** doc-wrong
**Response:** Verified against `docs/specs/001-host-backend-protocol.md:382`: the
R-18 verification row states that `hints` is read in `src/` only by
`normalize.rs::normalize_field` and that "the renderer, the one component that
may, does not exist yet". PHASE-03 falsifies both, and neither `canon-delta.md`
nor `design.md` §10 reached it.

This is canon, so it goes to `canon-delta.md` rather than into a phase.
CD-1's **Sections** now names an amendment to R-18's existing row alongside the
two new ones, and a new subsection states exactly what moves: the requirement is
unamended and the verdict stays *review, not a test*; only the two sentences of
evidence beneath it change. PHASE-03's Notes say outright that this is
`canon-delta.md`'s to carry and that no phase edits `docs/specs/`.

Endorsement is not owed now — it is owed at promotion, during audit, like
everything else in that file.

**Outcome:** verified

---

### F-9 — `FieldBlock` names two different types that meet in `glass.rs`, and PHASE-04/EX-6 does not say how

**Severity:** minor
**Location:** `plan.md` PHASE-02/EX-2, PHASE-03/EX-3, PHASE-04/EX-6

**Expected:** a phase's exit criteria can be written without the implementer
taking a naming decision of their own.

**Observed:** PHASE-02/EX-2 declares the generated Slint struct
`FieldBlock { heading, fields }`; PHASE-03/EX-3 declares the `view_model.rs`
struct `FieldBlock { heading: Option<String>, fields }`. PHASE-04/EX-6 then
requires `glass.rs` to build "one `FieldBlock` per presentation block" — so both
types are in scope in one file, under one name, and one of them has to be
aliased or path-qualified at the import. `glass.rs` already imports the
generated types by name (`use crate::generated::{OptionRow, PromptWindow, Tray,
WindowMode};`, `glass.rs:14`), so this is a real collision rather than a
hypothetical one. The design avoided the clash for the two neighbouring pairs —
`OptionRow`/`PresentationOption`, `FieldRow`/`PresentationField` — and did not
for this one.

**Evidence:**
- `plan.md` PHASE-02/EX-2, PHASE-03/EX-3, PHASE-04/EX-6.
- `design.md` §5.2 — *The window* block (`export struct FieldBlock …`) and *The
  mapper* block (`pub struct FieldBlock …`).
- `crates/goad/src/glass.rs:10-16` — the import block that would carry both.

**Disposition:** doc-wrong
**Response:** Correct and concrete rather than hypothetical: `glass.rs:14` already
imports generated types by bare name, and PHASE-04/EX-6 puts both `FieldBlock`s
in that file.

EX-6 now names the import shape: generated types keep their bare names, as
`OptionRow` does, and the mapper's is path-qualified `view_model::FieldBlock` at
its use sites. Neither type is renamed — `design.md` §5.2 names both, and a
rename would be a design change taken to avoid an import line.

**Outcome:** verified

---

### F-10 — PHASE-05/EX-2 lifts `logging_scripted` into `harness.rs` without updating `harness.rs`'s own roll-call of who it serves

**Severity:** minor
**Location:** `plan.md` PHASE-05/EX-2, *Surfaces*

**Expected:** EX-2 names the lift and its consequences — "its body does not
change, its callers in `scheduling.rs` follow the import, and no assertion in
`scheduling.rs` changes". PHASE-05's Surfaces name `harness.rs`,
`scheduling.rs`, `fields.rs` and `renderer/main.rs` (explicitly including "the
module roll-call in its doc").

**Observed:** `harness.rs`'s module doc states the rule the lift is justified by
*and* enumerates its consumers: "Anything two or more of
**`wiring.rs`/`table.rs`/`scheduling.rs`** need lives here; anything one of them
needs stays where it is" (`harness.rs:1-8`). `fields.rs` becomes a fourth
consumer and the enumeration does not admit it. The plan is careful to name the
equivalent roll-call in `renderer/main.rs` and silent about this one, so the
same class of staleness is closed in one file and left open in the file the
phase's central change lands in.

The lift itself is sound and does not breach S-2: `logging_scripted`'s body
calls `logging_backend`, which comes from the shared `crate::scripting` module
(`scheduling.rs:26`), not from `scheduling.rs`, so it compiles unchanged in its
new home; and `harness.rs`'s stated rule is "what two or more … need", which
`scheduling.rs` plus `fields.rs` satisfies.

**Evidence:**
- `crates/goad/tests/renderer/harness.rs:1-8`.
- `crates/goad/tests/renderer/scheduling.rs:26` — `use crate::scripting::{invocations, logging_backend, scripted};`
- `crates/goad/tests/renderer/main.rs:10-23` — the roll-call the plan *does* name.
- `plan.md` PHASE-05 *Surfaces*, EX-2.

**Disposition:** doc-wrong
**Response:** Correct, and it is the same class as F-4 and F-5 one file over:
`harness.rs:1-8` enumerates `wiring.rs`/`table.rs`/`scheduling.rs` as its
consumers and `fields.rs` becomes a fourth. The plan named the equivalent
roll-call in `renderer/main.rs` and was silent about this one.

EX-2 now carries it, along with the finding's own confirmation that the lift is
sound — `logging_scripted`'s body calls `logging_backend` from the shared
`crate::scripting`, so it compiles unchanged in its new home.

**Outcome:** verified

---

### F-11 — PHASE-02/VT-4 uses `find_all()` against a standing in-file warning that the list virtualises, and reconciles the two nowhere

**Severity:** nit
**Location:** `plan.md` PHASE-02/VT-4

**Expected:** a criterion that contradicts a comment in the file it lands in
says why.

**Observed:** VT-4 collects the option's field descriptions "with `find_all()` —
which returns matches in tree order, depth-first pre-order". The file it is
written into carries the opposite instruction three tests above:
*"counted through `accessible-item-count` — never `find_all().len()`, because
the list virtualises"* (`tree.rs:66-68`). An implementer who reads that comment
has an unanswered STOP-shaped question at the moment they write VT-4.

Checked, and VT-4 is fine: virtualisation is the `ListView` path only —
`update_all_instances` creates every instance for a plain repeater
(`i-slint-core-1.17.1/model/repeater.rs:143-154`), and the comment at
`:156-158` scopes the lazy algorithm to "the instances visible in the ListView
viewport". `ui/app.slint:40-46` is a `ScrollView` wrapping a `VerticalLayout`
with a plain `for`, not a `ListView`. The defect is that the plan does not say
so, and `tree.rs:68` is left standing as a claim that is wrong for this markup.

**Evidence:**
- `crates/goad/tests/renderer/tree.rs:66-68`.
- `crates/goad/ui/app.slint:40-46`.
- `i-slint-core-1.17.1/model/repeater.rs:143-154`, `:156-158`.

**Disposition:** doc-wrong
**Response:** Accepted as stated, including the finding's own verification that
VT-4 is sound: virtualisation is the `ListView` path, `app.slint:40-46` is a
`ScrollView` around a plain `for`, and `update_all_instances` creates every
instance.

VT-4 now says so with the citation, and instructs that `tree.rs:68`'s claim be
corrected to name the markup it is true of. Leaving a standing comment that
contradicts a criterion in the same file is how an implementer acquires an
unanswered STOP-shaped question at the worst moment.

**Outcome:** verified

---

### F-12 — PHASE-05's Notes state the logging backend writes its log line before it reads the request; it reads the request first

**Severity:** nit
**Location:** `plan.md` PHASE-05 *Notes for the implementer*

**Expected:** the reasoning an implementer is handed about what the invocation
log proves is accurate about the script it describes.

**Observed:** the Notes say "**The invocation log proves an exchange *began*,
not that it was absorbed.** `scheduling.rs:126-140` records why at length: the
script writes its line before it reads the request."
`tests/backends/logs-the-request-then-answers.sh` does the opposite order:
`request="$(cat)"` on line 19, the log append on line 26, the answer on line 32.
The conclusion the Notes draw is right for a different reason — the line is
written before the *answer*, and long before the host folds it — but the stated
mechanism is not what the script does, and `scheduling.rs:130-131` carries the
same wording.

**Evidence:**
- `tests/backends/logs-the-request-then-answers.sh:19`, `:26`, `:32`.
- `crates/goad/tests/renderer/scheduling.rs:127-142`.
- `plan.md` PHASE-05 *Notes for the implementer*.

**Disposition:** doc-wrong
**Response:** Verified against the script:
`tests/backends/logs-the-request-then-answers.sh` reads at `:19`, logs at `:26`,
answers at `:32`. The Notes stated the mechanism backwards, having inherited it
from `scheduling.rs:127-142`, which states it backwards too.

PHASE-05's Notes now give the actual order and the reason the conclusion still
holds — the line is written before the *answer* and long before the fold.
`scheduling.rs`'s own comment is **not** this phase's to fix and is directed to
`notes.md` Findings for the audit; a phase that repairs a comment outside its
subject is scope creep, and the audit is where a stale comment with no defect
behind it belongs.

**Outcome:** verified

---

### F-13 — two citations in the plan name the wrong span, and `submitted`'s visibility is under-specified across the 03/04 boundary

**Severity:** nit
**Location:** `plan.md` PHASE-01/EX-1 and *Notes*; PHASE-05/EX-2; PHASE-03/EX-2 with PHASE-04/EX-3

**Expected:** a `path:line` names the construct it is cited for, and a signature
given in one phase is callable from the phase that needs it.

**Observed:** three small things, none of which changes a conclusion:

1. PHASE-01/EX-1 and its Notes cite `Cancel` as `wire.rs:137-180`. `Cancel` is
   `wire.rs:142-181`; `:137-140` is `Wire::stop`, and `:181` — the closing brace
   of the `impl` — falls outside the span.
2. PHASE-05/EX-2 cites `logging_scripted` as `scheduling.rs:95-105`. `:95-100`
   is its doc-comment and the function is `:101-109`, so the cited span covers
   neither end.
3. PHASE-03/EX-2 gives `submitted` "exactly as `design.md` §5.2 gives them",
   and §5.2 gives `fn submitted(edited: &Edited) -> serde_json::Value` — private
   to `draft.rs`. PHASE-04/EX-3 requires `controller.rs` to call it, and
   `slice-007.md` AC-8 names `draft.rs::submitted` as a SPEC-001 §7 verification
   vehicle that a test must reach. The visibility that satisfies all three is
   taken by the implementer, not by the plan.

**Evidence:**
- `crates/goad/src/wire.rs:136-140` (`stop`), `:142-181` (`Cancel`).
- `crates/goad/tests/renderer/scheduling.rs:95-109`.
- `design.md` §5.2 *The draft*; `plan.md` PHASE-03/EX-2, PHASE-04/EX-3;
  `slice-007.md` AC-8.

**Disposition:** doc-wrong
**Response:** All three accepted. The two citations are corrected — `Cancel` is
`wire.rs:142-181` and `logging_scripted` is `scheduling.rs:101-109`.

The third is the substantive one and was under-specified rather than mis-cited:
PHASE-03/EX-2 now states `submitted` is `pub(crate)`, with the reasoning that
makes all three constraints true at once — `controller.rs` calls it,
`slice-007.md` AC-8 names `draft.rs::submitted` as a SPEC-001 §7 vehicle a test
must reach, and a `#[cfg(test)] mod tests` inside `draft.rs` reaches a private
item either way. `design.md` §5.2 writes the signature bare and is not amended;
a visibility is the plan's to choose.

**Outcome:** verified


## Synthesis

The plan review closes with **13 findings raised, 13 verified, none withdrawn,
no blocker outstanding**. One round, by user decision, plus a repair
verification pass and one contest that sent a finding back to the responder.
Every finding was dispositioned `doc-wrong`: in each case the plan was the
defect, not the design it plans or the code it plans to change.

**What it changed.** One blocker: PHASE-04 could not be executed inside its
declared surfaces, because `Refused::UnknownField` forces `diagnostics.rs` —
where `Refused` lives deliberately, so that every user-visible string in this
renderer is in one file — and forces a new string that no document stated. The
phase now declares that surface and `tests/renderer/reception.rs`, adds the
variant and its rendered line, and extends the one existing case whose name
claims every variant. The wording was the part the plan had no authority to
settle, and `design.md` §5.2 was amended with it by explicit user decision; AC-6
is unbreached, because `design.md` §9 scopes it to the option tests.

Two majors. The per-option container was to carry the same
`accessible-description` the option's button already carries, leaving
`element_described`'s `find_first()` unpinned between two matching elements and
two existing cases silently order-dependent — repaired on the unscoped helper
rather than on the identity scheme, and verified to fail closed: a wrong filter
returns `None` and both cases fail loudly rather than selecting the wrong
element. And AC-2's chain has three links — `View → present()`,
`present() → row model`, `row model → screen` — of which the Coverage table
named two, leaving `option_rows`, the only host code that puts a backend's
declared order and headings on screen, verified by nothing.

The rest closed real gaps rather than tidying: `just check` appeared in no
phase's Exit list despite the Coverage table claiming it, so five phases could
have been declared done with the gate red; two more live homes of the notice
rule existed that VA-2's grep cannot reach, because neither line contains the
word "notice"; `FieldBlock` names two types that meet in one file; SPEC-001 §7's
existing R-18 row states two facts about this codebase that PHASE-03 falsifies,
which is now `canon-delta.md`'s to carry.

**What it confirmed**, by evidence rather than by silence: PHASE-01/EX-4's claim
that `Wire` losing its window handle is forced rather than chosen; the 02/03/04
split, with nothing in the earlier phases needing what a later one provides;
PHASE-05's helper lift, which does not breach S-2; AC-5's two-halves criterion
and its ordering; and the plan's counts — 69 `.frame()` sites, six `Wire::new`
sites, three present sites in `serve`, five `#[test]`s in `tree.rs`, six
installations in `install.rs`. One count was wrong and is corrected: `serve` has
36 call sites, not the "~18" the plan estimated.

**The process defect this ledger records is its own.** Nine of the thirteen
findings are one class — a claim the plan changes staying live where the
instrument that checks it cannot see it. The repairs for that class then
produced three more instances of it, all casualties of a single renumber in
PHASE-02, and all three were caught by the verification pass rather than by the
responder. The lesson is narrower than "the plan is unsound", and worth stating
precisely: the plan's instrument for the class *worked* — EX-7's enumerated list
plus VA-2's grep found and now documents both sites the grep cannot reach. What
had no instrument at all was the artefact's **internal** consistency: its own
ids, counts and cross-references, which every repair edits and nothing checks.
That surface is now swept clean — every cross-reference resolves, no id sequence
has a gap or a duplicate — and the plan's Overview carries the standing rule
that every phase sheet re-derives its ids from `plan.md` rather than trusting
the copy, because a phase sheet is the next renumber-shaped edit in this slice's
future and there are six of them coming.

**Risks knowingly left standing.** The plan draws a line it is worth not
mistaking for an inconsistency: PHASE-02 corrects `tree.rs:68`'s over-broad
virtualisation claim in-phase, while PHASE-05 routes `scheduling.rs:127-142`'s
backwards mechanism to `notes.md` Findings for the audit. The difference is that
PHASE-05's Surfaces qualify `scheduling.rs` as a helper lift only; PHASE-02's do
not qualify `tree.rs`. Deliberate, not an oversight.

The canon side has one residue with a named owner. F-8 found one SPEC-001
verification row that this slice falsifies; the search that found it was keyed to
suspicion rather than systematic, and the systematic version is audit's
reconciliation walk, which `docs/AGENTS.md` already mandates. A residue with an
owner, not an open hole.

And the design review's own standing risks are unchanged by this review, because
none of them was its subject: keyboard focus dropped on every present; `text`,
`number`, `choice` and `datetime` undrawn; a scheduled firing still able to
replace a dirty form; the `groupbox`-versus-`list` announcement question open;
the RFC 3339 `datetime` form chosen without product demand; and stratum 3's
purity held by review alone, which no phase's criterion may imply otherwise.

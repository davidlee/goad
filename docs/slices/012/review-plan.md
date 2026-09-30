# Review — plan — Slice 012

**Subject:** plan — `docs/slices/012/plan.md` at 7517093, read against
`design.md` and `canon-delta.md` at the same commit
**Reviewer:** fresh agent (Opus), raiser; the orchestrator responds
**Opened:** 2026-10-01
**State:** open

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

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-10-01 — the plan as a whole.

The design is approved and its review is resolved (`review-design.md`
§Synthesis); this review does not reopen it, except where the plan shows the
design cannot be built as written. What it attacks is whether executing the
plan, phase by phase as written, produces the slice the design and
`slice-012.md` describe. The invariants it holds the plan to:

1. **Criteria that would fail.** Each acceptance criterion, and each
   exit/verification criterion, is one whose failure the named check would
   actually detect — not a proxy the regression it guards would survive.
2. **Completeness.** Every `design.md` §9 test, mutation check, *outside the
   gate* check and person-run; every witness `canon-delta.md` names; every
   `slice-012.md` §Surfaces entry and every `design.md` §5.2.7 site — each has
   a phase and a criterion.
3. **The chain holds.** Each phase's entry criteria are established by the
   phases before it; claimed parallelism (PHASE-01..03; PHASE-11 beside
   PHASE-06..08) really has disjoint surfaces, including `Cargo.lock`,
   `flake.nix`, `justfile` and `canon-delta.md`; every phase can end with
   `just check` green, red steps included.
4. **Fidelity.** The plan does not override the design or canon, and does not
   settle a question the design left open without saying so.
5. **Instruments reach.** Where a phase relies on an existing lint, scan or
   test to reach new code, the phase proves it; every negative control and
   mutation compiles.
6. **Nothing deferred to prose.** Everything owed later — to a later phase,
   to audit, to close — is a checklist item with an owner.
7. **Size.** Each phase fits one agent-session (~200k tokens), bookkeeping
   included.
8. **Project rules** in the plan's own text: cite by symbol, never line
   number; name, never count.

Where the bodies are likely buried: PHASE-02's delegation (`design-log.md`
2026-10-01, G1, added after design review); PHASE-04's size; the walk
procedure (PHASE-05, PHASE-11, PHASE-09, PHASE-10) against `design.md`
§5.2.8–§5.2.9; the kit's gate tests reaching files written in later phases;
the person-runs.

## Findings

**Round 1** — raised 2026-10-01 against `plan.md` at 046d492 (unchanged
since 7517093). Raiser: fresh agent (Opus). Code claims were read in the tree
at that commit.

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | | |
| F-2 | major | | |
| F-3 | major | | |
| F-4 | major | | |
| F-5 | major | | |
| F-6 | major | | |
| F-7 | major | | |
| F-8 | minor | | |
| F-9 | minor | | |
| F-10 | minor | | |
| F-11 | minor | | |
| F-12 | minor | | |
| F-13 | minor | | |
| F-14 | minor | | |
| F-15 | minor | | |
| F-16 | minor | | |
| F-17 | minor | | |
| F-18 | minor | | |
| F-19 | nit | | |
| F-20 | nit | | |
| F-21 | nit | | |

### F-1 — PHASE-05/EX-8 asks two agents to answer a prompt from a fresh home, and nothing gives them credentials there

**Severity:** major
**Location:** `plan.md` PHASE-05/EX-8, PHASE-05 §Entry; compare PHASE-11/EN-3

**Expected:** Each exit criterion can be met by executing the phase as written
(Brief invariant 3). R1 is probed on the host before any prose, so a failure
there is R1's, and the STOP condition reads it that way.
**Observed:** EX-8 needs `claude -p … --plugin-dir "$KIT/kit"` and Codex to
*answer* a headless prompt "in a fresh home that has held no session". A fresh
home is logged out. PHASE-05 has no entry criterion and no step for
authentication; only PHASE-11 has one (EN-3, credentials as environment
variables for the capsule's ssh command). The STOP note ("if either plugin load
fails in a way the kit cannot fix: that is R1 firing") would read an
authentication failure as R1.
**Evidence:** `research.md` §"Spike: R1 and R2", *Not reached*: "A fresh home
is logged out: Claude answers *'Not logged in · Please run /login'*, Codex
`401 Unauthorized`. Nothing in the design says how a walk authenticates …
Open." `spike/run.sh`'s last line: `claude -p hi --plugin-dir "$GOAD_KIT" ||
true   # fresh home: "Not logged in"`. The spike ran the same step EX-8 asks for.

**Disposition:**
**Response:**

**Outcome:**

### F-2 — "naming the goad-backend skill" proves the skill is listed, not that the model can read it; R1's open half is not reached

**Severity:** major
**Location:** `plan.md` PHASE-05/EX-8, PHASE-11/EX-2, PHASE-05 §Surfaces
(`SKILL.md` "a placeholder: frontmatter and a line"); `design.md` §8 R1

**Expected:** R1's residual is "the model reading the skill is not
[verified]", and its mitigation is to stand the loads up first so a blocker
costs a manifest (`plan.md` §Sequencing, *Why the plugin loads come early*).
A criterion guarding that must fail when the model cannot read the skill's
body (Brief invariant 1).
**Observed:** Both criteria pass when the agent names the skill. A skill's
name and description reach the model through the harness's skill listing,
from the frontmatter. The body is read only when the skill is used. A load
that lists the skill but whose body cannot be read from the store path is the
case R1 leaves open, and it passes both EX-8 and EX-2. The placeholder is
specified as "frontmatter and a line", with no marker that only the body
carries.
**Evidence:** The spike's stub did carry one:
`spike/stub/kit/skills/goad-backend/SKILL.md` — "The spike marker is
PERIWINKLE-HALIBUT-42. If asked for the goad spike marker, answer with it
verbatim." The spike never got as far as asking for it (`research.md`, *Not
reached: the model seeing the skill*). The plan drops the marker and asks for
the name instead.

**Disposition:**
**Response:**

**Outcome:**

### F-3 — PHASE-09/EN-1's lock bump to PHASE-08's revision is owned by no phase

**Severity:** major
**Location:** `plan.md` PHASE-09/EN-1 and §Surfaces; PHASE-05/EX-4;
PHASE-11/VA-1 and §Surfaces

**Expected:** Each entry criterion is established by an earlier phase's exit
(Brief invariant 3). A write to `goad-walk` sits in some phase's surfaces, as
PHASE-05 and PHASE-10 declare theirs.
**Observed:** PHASE-09/EN-1 needs "`goad-walk`'s lock pins that revision
[PHASE-08's, on `main`] (the script's lock bump, committed in `goad-walk`)".
PHASE-05/EX-4 pins "this phase's goad commit". No later phase moves the lock:
- PHASE-06..PHASE-08 do not touch `goad-walk`.
- PHASE-11's only surface is `docs/slices/012/walk/`, and VA-1's list of what
  the walk script does includes no lock bump.
- PHASE-09's surfaces name `docs/slices/012/walk/` "(fixes to the script
  only)" and not `~/dev/goad-walk`.

Consequence: a first walk provisioned at `goad-walk`'s `main` (PHASE-11/EX-1)
sees PHASE-05's kit, whose `SKILL.md` is the placeholder, and nothing before
the walk refuses it. `walks.md` records the revision (EX-2), but only the
re-walk rule says an old revision does not count (`design.md` §5.2.9,
PHASE-10/EX-3).
**Evidence:** `plan.md` PHASE-05/EX-4 ("its lock pinned to this phase's goad
commit"). PHASE-10 §Surfaces lists `~/dev/goad-walk/flake.lock`; PHASE-09
§Surfaces does not. PHASE-11/VA-1's enumeration. `~/dev/goad-walk` at
99cc374 has `inputs.goad.url = "git+file:///home/david/dev/goad"`, and its lock
is staged but not committed (`git status --short` → `A flake.lock`).

**Disposition:**
**Response:**

**Outcome:**

### F-4 — Nothing gives `AtFault` a printed form, so PHASE-04 must either map sides itself or edit stratum 1 outside its surfaces

**Severity:** major
**Location:** `plan.md` PHASE-01/EX-1, PHASE-04/EX-3, PHASE-04/VA-1,
PHASE-04 §Surfaces; `design.md` §5.1 *Who owns what*, §5.2.3

**Expected:** The checker "prints them and never maps them" (`design.md` §5.1,
first row). PHASE-04 starts with what it needs from PHASE-01..PHASE-03 (Brief
invariant 3).
**Observed:** PHASE-01/EX-1 gives `Requirement` a display ("displays as
`R-N`") and gives `AtFault` none. The report prints each refusal's side as a word
(`design.md` §5.2.5's example: `REFUSED  backend  SPEC-001/R-40`). `AtFault`
derives only `Debug`, which prints `Backend`, capitalised. So PHASE-04 must
either:
- write a `match` from `AtFault` to a word in `goad-check`, which is a second
  encoding of the side names and a hit for VA-1's own grep `AtFault::`; or
- add a `Display` to `goad_semantics::error`, which is not in PHASE-04's
  surfaces.
**Evidence:** `design.md` §5.2.3's code block gives `#[derive(Debug, Clone,
Copy, PartialEq, Eq)] pub enum AtFault { … }`. Only `Requirement`'s doc
mentions display: "Displays as `R-44`". `plan.md` PHASE-04/VA-1: "`grep -nE
'R-[0-9]+|AtFault::'` over `crates/goad-check/src` finds only the R-56 probe's
claim".

**Disposition:**
**Response:**

**Outcome:**

### F-5 — The kit's standing guards are shown to reach only the files that exist when they are written; later phases add files and nothing re-proves reach

**Severity:** major
**Location:** `plan.md` PHASE-07/VT-1, VA-1, VA-2; PHASE-08/EX-1, VT-1;
PHASE-06/VT-1; PHASE-04/VA-1, VA-2; PHASE-10/EX-2

**Expected:** "Where a phase relies on an existing lint, scan or test to reach
new code, the phase proves it" (Brief invariant 5).
**Observed:**
- The fence test's reach is proven once, at PHASE-07 (VA-1's hand count, and
  VA-2 untagging a `protocol.md` fence). PHASE-08 writes `scheduling.md`,
  `events.md`, `checking.md` and the finished `SKILL.md`. Its EX-1 is only
  "VT-1 of PHASE-07 is green over the finished kit", which is green whether or
  not the new files were read. PHASE-10's kit fixes need only `just check`
  green (EX-2).
- The coverage test's file set is not stated: the reference directory, a
  list, or the whole kit. PHASE-10 can add reference files.
- PHASE-06/VT-1 does not say whether it enumerates `examples/*` or a hand list
  of three, though I-4 is about "every shipped example".
- I-1 and I-2 are read once, at PHASE-04 (VA-1, VA-2). I-2 "binds tests too"
  (`plan.md` PHASE-07 notes), but PHASE-07 and PHASE-08 later add
  `goad-check`'s kit tests, including the respond oracle and the matches
  beside the coverage builders. PHASE-10 edits `crates/goad-check/src`.
  Neither invariant is read again after PHASE-04.
**Evidence:** `plan.md` PHASE-08 §Exit and §Verification: no count, and no
planted fence in a file PHASE-08 writes. PHASE-10 §Exit and §Verification: no VA
names I-1, I-2, I-3 or I-5. This is the class the project's own auto-memory
names *a standing guard may not reach a new file* ("green proves the lint did
not fire, not that it looked; make the phase prove reach").

**Disposition:**
**Response:**

**Outcome:**

### F-6 — I-5's test has no stated rule and no negative control, so a vacuous version passes

**Severity:** major
**Location:** `plan.md` PHASE-07/VT-5; `design.md` §5.5 I-5, §9

**Expected:** A criterion whose failure the named check would detect (Brief
invariant 1). I-5 is what makes the kit stand alone, which AC-1 depends on
("nothing else from this repository").
**Observed:** VT-5 is a test name only. Neither the plan nor the design says
what counts as "a path in this repository outside `kit/`": `../`, a `crates/`,
`docs/` or `exercisers/` prefix, a Markdown link, an `include`. Unlike
VT-1..VT-4, it has no negative control over an inline string, and no mutation.
A test that scans for nothing passes.
**Evidence:** `plan.md` PHASE-07 §Verification: VT-2, VT-3 and VT-4 are
negative controls for VT-1, VT-5 has none, and VA-1 and VA-2 concern VT-1
only. `design.md` §9 lists `nothing_in_the_kit_names_a_path_outside_it (I-5)`
and states no rule for it.

**Disposition:**
**Response:**

**Outcome:**

### F-7 — The discard witness has no red step and no mutation, so nothing shows it can fail

**Severity:** major
**Location:** `plan.md` PHASE-01/VT-3, EX-5, VA-2; `canon-delta.md` SPEC-001
Change 2

**Expected:** Each named witness is one whose failure the check would detect
(Brief invariant 1). `canon-delta.md` Change 2 names
`::every_discard_fixture_names_a_requirement_in_its_own_list` as the corpus
witness of R-59's answers "over the discards and the `schedule` corpus".
**Observed:** EX-5's red is "on exactly those three fixtures": two
`protocol-text` `Json` fixtures and one `protocol` `EmptyAlternatives`
fixture, all read by the refusal witness. VA-2's mutation (`NestedHints` →
R-3) reds the refusal witness only. The discard witness is green from its
first run, and no criterion ever turns it red. A version that reads the wrong
root, or matches no `error` tag, passes. The schedule corpus's runner in
`runner.rs` (`SCHEDULE`, `check_schedule`) is private, so the witness has to
read that corpus itself — the part most likely to be vacuous.
**Evidence:** `crates/goad-semantics/tests/protocol/runner.rs`: `const
SCHEDULE: Corpus` and `fn check_schedule` are private, and
`assert_corpus`'s vacuity guard ("`run` cannot return `Ok(0)`") holds only for
a `Corpus` run. `jq` over `tests/fixtures/protocol*/*.json`
(`[input_filename, .requirement, .expect]`) confirms that no discard fixture's
list is corrected, so the corpus gives the discard witness no red.

**Disposition:**
**Response:**

**Outcome:**

### F-8 — PHASE-04/VA-1's grep matches requirement ids in comments, which house style requires

**Severity:** minor
**Location:** `plan.md` PHASE-04/VA-1; `design.md` §5.5 I-1

**Expected:** I-1 is about *literals* ("no requirement id or side literal").
The check should tell a literal the checker prints apart from a comment that
cites a reason.
**Observed:** `grep -nE 'R-[0-9]+|AtFault::'` over `src` "finds only the R-56
probe's claim". `docs/memory/cite-requirements-not-finding-ids.md` requires a
comment that says why to cite a requirement id, so a conforming crate has
many hits. The criterion either fails on correct code or pushes the
implementer to strip citations.
**Evidence:** In the sibling crate, `grep -nE 'R-[0-9]+'
crates/goad-emit/src/*.rs` finds:
- `main.rs:47`: `SPEC-003/R-10`, in a doc comment;
- `render.rs:62` and `render.rs:253`: `SPEC-003/R-14`, in comments;
- `render.rs:109`: in a string.

**Disposition:**
**Response:**

**Outcome:**

### F-9 — PHASE-02/EX-6's "no other `"host"` source literal remains in `crates/*/src`" is false by construction, and the rule is not carried to the probe

**Severity:** minor
**Location:** `plan.md` PHASE-02/EX-6, VT-1; PHASE-04/EX-2

**Expected:** An exit criterion the phase can meet without contradicting its
own verification.
**Observed:**
- Tests inside `src` spell `"host"`, and they are witnesses, not encodings.
  `canonical.rs`'s `an_evaluate_serializes_to_the_spec_s_wire_form` and
  `every_request_kind_carries_the_version_and_a_discriminant` build events with
  `source: "host".to_owned()`. `envelope.rs`'s reserved-source test
  (`// ---- VT-9: source == "host" ----`) substitutes `"host"` for the source.
- VT-1 moves `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`
  into `canonical.rs` "verbatim", and it asserts `event.source == "host"`.
- PHASE-04's probe builds an `Event` with the host source (`design.md`
  §5.2.2, step 4), and no PHASE-04 criterion says it must name
  `HOST_SOURCE`. The literal PL-2 removes can come back in the new crate.
**Evidence:** `grep -rn '"host"' crates --include=*.rs` finds:
- three lines in `canonical.rs`, all in `mod tests`;
- the test lines in `envelope.rs`;
- the test line `assert_eq!(event.source, "host");` in `wire.rs`.

**Disposition:**
**Response:**

**Outcome:**

### F-10 — PHASE-02/EX-4 says `view_model.rs`' `as_drawn` tests stay unchanged; EX-7 forces them to change

**Severity:** minor
**Location:** `plan.md` PHASE-02/EX-4, EX-7; `design.md` §8 R5

**Expected:** R5's guard is that the tests move or stay *verbatim*, so a green
run means the behaviour did not change.
**Observed:** EX-7 removes `DrawnKind::Choice`'s `first`. Two tests use it,
and both must change:
- The helper `a_choice` constructs `DrawnKind::Choice { first: …,
  alternatives: … }`.
- `as_drawn_answers_every_kind` destructures `let DrawnKind::Choice { first,
  .. } = a_choice()` to get its expected value.

The obvious replacement expected value, `alternatives.first().id().clone()`,
is the expression the delegated production code uses. The test's choice case
would then check the code against itself.
**Evidence:** `crates/goad/src/view_model.rs`, `mod tests`: `fn a_choice`
(`first: alternatives.as_slice()[0].id().clone()`) and `fn
as_drawn_answers_every_kind`.

**Disposition:**
**Response:**

**Outcome:**

### F-11 — G1's `From<Submitted> for Edited` in `draft.rs` "through the existing `adjusted`" cannot reach it; the plan does not say which moves

**Severity:** minor
**Location:** `plan.md` PHASE-02/EN-2, EX-3; `design.md` §5.2.4

**Expected:** Where the design's route cannot be built as written, the plan
says how it is built (Brief invariant 4).
**Observed:** `adjusted` and `spelled` are private functions in
`view_model.rs`. `draft.rs` imports nothing from `view_model`: the dependency
runs one way, `view_model` → `draft`. Building the conversion in `draft.rs`
on top of `adjusted` needs one of two changes:
- make both `pub(crate)` and add a `draft` → `view_model` edge, which is a
  module cycle; or
- move both into `draft.rs`.

Moving them falsifies `adjusted`'s own doc: "every site that produces one
without a person having typed comes through here — `as_drawn`, and a
`Slider`'s `AdjustedValue`".
**Evidence:** `crates/goad/src/view_model.rs` has `fn spelled(number: f64) ->
String` and `fn adjusted(number: Finite) -> Edited`, neither `pub`, and
imports `use crate::draft::{Edited, Finite, Reported};`.
`crates/goad/src/draft.rs` has no `use crate::view_model`.

**Disposition:**
**Response:**

**Outcome:**

### F-12 — "Their surfaces are disjoint by file" is false for PHASE-01..PHASE-03, and for PHASE-11 beside PHASE-06..PHASE-08

**Severity:** minor
**Location:** `plan.md` §Sequencing (*PHASE-01, PHASE-02 and PHASE-03 may run
in parallel*); PHASE-01 and PHASE-02 §Surfaces; PHASE-11 notes

**Expected:** Claimed parallelism has disjoint surfaces, `canon-delta.md`
included (Brief invariant 3).
**Observed:**
- PHASE-01 and PHASE-02 both list `canon-delta.md` ("test names only"), for
  Change 2 and Changes 3–4 respectively.
- Every phase writes `notes.md`: the §Status table and its own phase sheet
  (`docs/AGENTS.md` §Execute).
- PHASE-11 "may run beside them, in its own worktree", and it shares
  `notes.md` with whichever of PHASE-06..PHASE-08 is running.
**Evidence:** `plan.md` PHASE-01 §Surfaces ends "`canon-delta.md` (test names
only)", and PHASE-02 §Surfaces ends the same way. `notes.md` §Status is a single table.

**Disposition:**
**Response:**

**Outcome:**

### F-13 — PHASE-04 is the slice's largest code phase, and it is not split or given a checkpoint

**Severity:** minor
**Location:** `plan.md` PHASE-04; §Sequencing *Size*

**Expected:** Each phase fits one agent session, ~200k tokens, bookkeeping
included (Brief invariant 7).
**Observed:** PHASE-04 holds all of this:
- a new crate's arguments, and its run loop with chained answers;
- the R-56 condition, the report and the status cut;
- an argument table and fourteen named binary cases with their `bash`
  fixtures;
- the probe's unit test, three reach proofs and two invariant reads;
- the re-pointing of `canon-delta.md`.

The precedent, for a smaller binary, was one phase for the crate alone: slice
005's PHASE-03, *the crate, and the binary*, after two phases of lifts. The
plan calls PHASE-04 "the largest in code", but names no point at which a
PARTIAL checkpoint would leave a compiling, green tree.
**Evidence:** `wc -l crates/goad-emit/src/*.rs crates/goad-emit/tests/binary/*`
→ 1515 lines in total, with five binary cases in `exchange.rs`.
`docs/slices/005/plan.md`: `## PHASE-03 — the crate, and the binary`.

**Disposition:**
**Response:**

**Outcome:**

### F-14 — PHASE-06/VT-1 accepts an example that never asks anything

**Severity:** minor
**Location:** `plan.md` PHASE-06/VT-1, VA-2; `design.md` §5.2.5, §5.5 I-4

**Expected:** The regression a test guards makes it fail (Brief invariant 1).
Status 0 with no view returned by any exchange is exactly the case §5.2.5
makes visible, and AC-1 refuses it for walks.
**Observed:** VT-1 asserts only acceptance (status 0). It accepts an example
that has stopped speaking — every exchange answered `view: null` — and whose
respond path therefore goes unexercised. VT-2 covers this for triage only,
through its side effect. VA-2's mutation (exit 1 on the probe) does not reach
it.
**Evidence:** `design.md` §5.2.5: "A backend that is silent at the hour it is
checked is accepted, and the report is what shows it was not seen to ask
anything." `plan.md` PHASE-09/EX-3 requires "status 0 with at least one view
answered" of a walk; VT-1 does not require it of an example.

**Disposition:**
**Response:**

**Outcome:**

### F-15 — Run-sequence and structural properties of the checker have no criterion

**Severity:** minor
**Location:** `plan.md` PHASE-04 §Exit and §Verification; `design.md` §5.2.1,
§5.3, §5.4; `canon-delta.md` SPEC-004 Change 5

**Expected:** Each design property the plan delivers has a criterion (Brief
invariant 2).
**Observed:** No named case or VA holds any of these:
- §5.4: "every planned exchange runs to completion, whatever an earlier
  exchange did". No case asserts that the remaining exchanges still run after
  a failure at `startup`.
- `--event` envelopes are sent "in the order given". VT-1 holds only the order
  they are parsed in.
- §5.2.1: the checker "ignores `[ingress]`: it opens no socket".
- §5.3: "The checker never alters the environment it passes on."
- `canon-delta.md` SPEC-004 Change 5 holds R-13's *whatever its cause* and
  R-15 "structurally": one `ExitCode::from(2)` that reads no cause, and `main`
  returning one literal per class. EX-4 states the first, but no VA reads
  `main` for either.
**Evidence:** `plan.md` PHASE-04/VT-2's list of cases. `canon-delta.md` SPEC-004
Change 5: the R-11..R-13 row ("held structurally: every cause reaches the
status through one `ExitCode::from(2)` that reads no cause") and the R-15 row
("the compiler and review").

**Disposition:**
**Response:**

**Outcome:**

### F-16 — Parts of the reference's contract are stated in exits but held by nothing

**Severity:** minor
**Location:** `plan.md` PHASE-07/EX-3, PHASE-08/VT-1, PHASE-08/VA-1;
`design.md` §5.2.6 *The reference's structure*

**Expected:** Each design rule for the reference has a check that would fail
(Brief invariants 1, 2).
**Observed:**
- "Each id at an anchor of its own" (F-32) is in PHASE-07/EX-3, and nothing
  checks it. The coverage test holds that an id *appears*. It does not hold
  that the id heads its own anchor, or that it appears in `reference/` rather
  than in an example README.
- The statement that the specs are not shipped has no criterion.
- PHASE-07/VA-3 reads its two files for "nothing host-internal is taught".
  PHASE-08/VA-1 reads `scheduling.md`, `events.md` and `checking.md` only for
  contradictions. `design.md` §5.2.6 names bounds and cleanup as
  host-internal, and both border on scheduling.
**Evidence:** `design.md` §5.2.6, the bullets under *The reference's
structure*. `plan.md` PHASE-08/VT-1's only rule is id matching with a
non-digit boundary.

**Disposition:**
**Response:**

**Outcome:**

### F-17 — The recipe changes before the policy, against POL-001's letter, and the plan does not say it overrides it

**Severity:** minor
**Location:** `plan.md` PHASE-03/EX-3, VA-1; PHASE-06/EX-3; `plan-log.md` PL-4;
`docs/policy/001-the-phase-gate.md` §Statement

**Expected:** The plan does not override canon without saying so (Brief
invariant 4).
**Observed:** POL-001 says "the `justfile` mirrors it, and a change goes into
this policy first and into the recipe second". PL-4 changes the recipe at
PHASE-03 and PHASE-06, and the policy at audit, on the strength of
`docs/AGENTS.md`'s working-authority rule. That may be right, but the plan and
PL-4 cite only the working authority. Neither names the POL-001 sentence they
depart from. The consequences:
- From PHASE-03 to audit, POL-001's block names
  `examples/typescript/backend.ts`, which no longer exists.
- PHASE-03/VA-1 compares `just -n check` with "the POL-001 sequence with
  EX-3's line", which is not POL-001.
- EX-3 rewrites the recipe's comment "as §5.2.7 says": "one exerciser and one
  kit example, both typechecked". That is false until PHASE-06 adds the kit
  path.
**Evidence:** `docs/policy/001-the-phase-gate.md` §Statement, and §Compliance
(`deno check examples/typescript/backend.ts`). The `justfile`'s header comment:
"Change the policy first, then mirror". `design.md` §5.2.7, the `justfile`
`typecheck` row.

**Disposition:**
**Response:**

**Outcome:**

### F-18 — `goad-check`'s tests grow FU-5's class beyond the fence scanner, and the plan names only the scanner

**Severity:** minor
**Location:** `plan.md` §Owed to audit and close (*Extend FU-5*), PHASE-07/VA-4,
PHASE-04 notes; `design.md` §5.2.6 (the scanner "is a second one, knowingly")

**Expected:** "No parallel implementation" (user standards), and FU-5's
extension names every member of its class that this slice adds.
**Observed:** FU-5 covers two kinds of helper. One is the stratum-3
binary-tier helpers transcribed into both `goad-emit` and `goad` ("the spawn,
`code_of`, `stderr_of` and `stdout_of`"). The other is every helper that
promises a unique temp path. PHASE-04's binary tier and PHASE-06's kit tier
need both kinds, and a temp-dir crate is a STOP, so a third transcription is
what happens by default. The plan's FU-5 extension names only the fence
scanner.

The design's reason for a second scanner, "one crate's test targets cannot
reach another's helpers", understates what exists. Workspace members already
share test helpers from the workspace root through `#[path]`. FU-5's measured
obstacle is `dead_code` when a target includes a file whose symbols it does
not all use, and splitting the file answers that.
**Evidence:** `docs/follow-ups.md` FU-5: the obstacle, and *Dead when* "every
helper promising a unique path goes through `claim`, from a support file a
target can include without the scripted-backend helpers".
`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`.
`tests/support/{driving.rs, scripting.rs, waiting.rs}`.

**Disposition:**
**Response:**

**Outcome:**

### F-19 — Phase surfaces omit files the phase's own rules may have it edit

**Severity:** nit
**Location:** `plan.md` PHASE-03 §Surfaces; PHASE-02 §Surfaces; §Overview
*Test names are commitments*

**Expected:** Surfaces name every file a phase may touch.
**Observed:**
- PHASE-03 ships `goad-emit`'s `an_answer_that_cannot_be_written_exits_2`,
  which `canon-delta.md` SPEC-004 Change 5 places in `exchange.rs`. If it ships
  anywhere else, *Test names are commitments* requires `canon-delta.md` to
  change, but PHASE-03's surfaces omit `canon-delta.md`.
- A comment in `crates/goad/Cargo.toml` names "`view_model.rs`'s
  `Undrawn`/`Stimulus::event` path" as a reason `goad` needs `serde_json`.
  PHASE-02 moves `Stimulus::event` out of the crate, and its surfaces omit the
  manifest.
**Evidence:** `canon-delta.md` SPEC-004 Change 5, the R-8..R-10 row. The
comment above `[dependencies]` in `crates/goad/Cargo.toml`.

**Disposition:**
**Response:**

**Outcome:**

### F-20 — Counts of open lists in the plan's own text

**Severity:** nit
**Location:** `plan.md` §Sequencing (*Why the reference is two phases*);
PHASE-08/VT-1; PHASE-02 §Surfaces; §Overview PHASE-06

**Expected:** Name, never count, unless the list cannot grow (`CLAUDE.md`).
**Observed:** The plan counts open lists: "writing five reference files", "the
two files most of the requirement ids live in", "Seen red before the three
files are written", "The three kit examples". "(imports and the delegation
only, in the last four)" is a positional count over a brace list. The
reference and the examples are open lists, and PHASE-10 may add to either.
**Evidence:** `grep -noiE '\b(two|three|four|five)\b' docs/slices/012/plan.md`.

**Disposition:**
**Response:**

**Outcome:**

### F-21 — Small inaccuracies in the plugin and package criteria

**Severity:** nit
**Location:** `plan.md` PHASE-05/VA-1, PHASE-11/EX-2, PHASE-05/EX-1

**Expected:** Criteria that name the right path and command.
**Observed:**
- PHASE-05/VA-1 installs "in a fresh `CODEX_HOME`" and then checks
  `~/.codex/plugins/cache/`. The fresh install writes its cache under
  `$CODEX_HOME`, so the check reads the wrong directory.
- PHASE-11/EX-2 spells Claude's load as `claude --plugin-dir "$KIT/kit"`,
  without the `-p` that PHASE-05/EX-8 has, yet still calls the prompt headless.
- `design.md` §5.2.8 builds `packages.goad-check` "as `goad-emit` has", and
  `goad-emit`'s derivation sets `GOAD_REVISION = revision`. PHASE-05/EX-1
  checks only that the package builds, so a package whose `--version` prints
  no revision passes.
**Evidence:** `flake.nix` `goadPackages.goad-emit` (`GOAD_REVISION =
revision;`); `crates/goad-emit/src/main.rs`
(`option_env!("GOAD_REVISION")`).

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

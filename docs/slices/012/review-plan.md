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

**Round 2** — 2026-10-01 — the round 1 repairs (58777b9), and anything they
introduced. Each F-1..F-21 repair is held against its Response and the user's
decision (`plan-log.md` and `design-log.md` 2026-10-01, *plan review round 1*),
and checked across the class, not only the instance. The same invariants as
round 1 bind the repaired text. Where the bodies are likely buried: the
PHASE-04 / PHASE-12 split and its renumbered citations; the new I-5 rule;
`main` between PHASE-04 and PHASE-12; PHASE-08/VT-1's anchor rule.

## Findings

**Round 1** — raised 2026-10-01 against `plan.md` at 046d492 (unchanged
since 7517093). Raiser: fresh agent (Opus). Code claims were read in the tree
at that commit.

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | doc-wrong | verified |
| F-2 | major | doc-wrong | verified |
| F-3 | major | doc-wrong | verified |
| F-4 | major | doc-wrong | verified |
| F-5 | major | doc-wrong | verified |
| F-6 | major | doc-wrong | verified |
| F-7 | major | doc-wrong | verified |
| F-8 | minor | doc-wrong | verified |
| F-9 | minor | doc-wrong | verified |
| F-10 | minor | doc-wrong | verified |
| F-11 | minor | doc-wrong | verified |
| F-12 | minor | doc-wrong | verified |
| F-13 | minor | doc-wrong | verified |
| F-14 | minor | doc-wrong | verified |
| F-15 | minor | doc-wrong | verified |
| F-16 | minor | doc-wrong | verified |
| F-17 | minor | doc-wrong | verified |
| F-18 | minor | doc-wrong | verified |
| F-19 | nit | doc-wrong | verified |
| F-20 | nit | doc-wrong | verified |
| F-21 | nit | doc-wrong | verified |

**Round 2** — raised 2026-10-01 against `plan.md` and `design.md` at 58777b9.

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-22 | minor | | |
| F-23 | minor | | |
| F-24 | minor | | |
| F-25 | minor | | |
| F-26 | minor | | |
| F-27 | nit | | |

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

**Disposition:** doc-wrong
**Response:** PHASE-05 gains a credentials entry (API keys as environment variables in the fresh home); its STOP separates an authentication failure from R1. *(Repair: PHASE-05 gains EN-2 (API keys in the fresh home); EX-8 runs a bare `claude -p`/`codex exec` prompt before any plugin load, and a new STOP reads its failure as authentication, not R1. PHASE-11/EX-2 and its notes take the same bare-prompt step.)*

**Outcome:** verified — PHASE-05/EN-2 (API keys in the fresh home), EX-8's bare prompts before any load, and a separate authentication STOP; the class reaches PHASE-11 (EX-2's bare prompts, the matching STOP).

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

**Disposition:** doc-wrong
**Response:** The placeholder `SKILL.md` carries a body-only marker; PHASE-05/EX-8 and PHASE-11/EX-2 ask for a fact only the body states at the pinned revision. *(Repair: PHASE-05 §Surfaces specifies a body-only marker in the placeholder `SKILL.md`; PHASE-05/EX-8 and PHASE-11/EX-2 ask for it verbatim from a prompt that does not contain it, PHASE-11's at the revision `goad-walk`'s lock pins; PHASE-05's notes say why the name is not enough.)*

**Outcome:** verified — PHASE-05 §Surfaces puts a body-only marker in the placeholder; EX-8 and PHASE-11/EX-2 ask for it verbatim, and the prompt does not contain it.

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

**Disposition:** doc-wrong
**Response:** PHASE-09 owns the lock bump: `~/dev/goad-walk/flake.lock` joins its surfaces, and an exit criterion bumps it to PHASE-08's revision and commits it before either walk. *(Repair: PHASE-09 §Surfaces gains `~/dev/goad-walk/flake.lock`; EN-1 no longer assumes the bump, and new PHASE-09/EX-5 bumps and commits it before either walk. PHASE-05/EX-4 now commits the lock it pins.)*

**Outcome:** verified — PHASE-09/EX-5 owns the bump before either walk, and PHASE-09 §Surfaces lists `~/dev/goad-walk/flake.lock`; PHASE-11/EX-2 names PHASE-05's revision until then.

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

**Disposition:** doc-wrong
**Response:** `AtFault` gains `Display` in stratum 1 (`design-log.md` 2026-10-01, *plan review round 1*); PHASE-01 delivers it. *(Repair: `design.md` §5.2.3 gives `AtFault` a total-match `Display` printing the four side words; PHASE-01/EX-1 delivers it, and PHASE-01/VT-1 adds `every_side_displays_as_the_word_a_report_prints`. I-1's command (§Sequencing, *Invariant reads*) now also greps the side words as string literals.)*

**Outcome:** verified — `design.md` §5.2.3 gives `AtFault` a total `Display`; PHASE-01/EX-1 and VT-1 (`every_side_displays_as_the_word_a_report_prints`); PHASE-12/EX-3 prints through it; I-1's command greps the four side words.

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

**Disposition:** doc-wrong
**Response:** PHASE-08 plants an untagged fence in a file it writes; the coverage test reads every `*.md` under `reference/` by enumeration; PHASE-06's example test enumerates the example directories and refuses an empty set; PHASE-10 re-reads I-1, I-2 and fence reach for any file it adds. *(Repair: PHASE-08/VA-2 plants an untagged fence in `scheduling.md`; PHASE-07/EX-1 and PHASE-08/VT-1 enumerate every `*.md` under `kit/` and `reference/`, and PHASE-06/VT-1 every example directory, each refusing an empty set; PHASE-10/VA-5 re-reads I-1, I-2 and fence reach. Siblings: I-2 is re-read where kit tests are added (PHASE-07/VA-6, PHASE-08/VA-3), and I-1/I-2 over the finished checker (PHASE-12/VA-1, VA-2).)*

**Outcome:** verified — Fence reach re-proven in a PHASE-08 file (PHASE-08/VA-2) and for PHASE-10's (VA-5); the coverage, example and fence tests enumerate and refuse an empty set; I-2 re-read at PHASE-07/VA-6, PHASE-08/VA-3, PHASE-12/VA-2; I-1 at PHASE-12/VA-1 and PHASE-10/VA-5.

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

**Disposition:** doc-wrong
**Response:** I-5's rule stated in the design (`design-log.md` 2026-10-01): an escaping relative path, or a mention of any other top-level repository entry, read at test time; negative control over an inline string. *(Repair: `design.md` §5.2.6 *The kit stands alone* and §5.5 I-5 state the rule, §9 adds `a_path_outside_the_kit_is_refused`; PHASE-07 gains EX-5, VT-5's enumeration and root guard, VT-6 (the negative control) and VA-5 (a planted escaping path). The rule adds a boundary so `kit/.claude-plugin/` is not a mention of the root's `.claude-plugin`.)*

**Outcome:** verified — `design.md` §5.2.6 *The kit stands alone* states the rule; PHASE-07/VT-6 is its negative control and VA-5 its reach proof. The rule itself raises F-22.

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

**Disposition:** doc-wrong
**Response:** A mutation flipping a `ScheduleError` arm outside its fixtures' lists reds the discard witness; the witness asserts it read a non-zero number of fixtures. *(Repair: PHASE-01/VA-4 flips `ScheduleError::NotAString` R-25 → R-3, reddening the discard witness on one fixture from each half; PHASE-01/VT-3 makes each witness refuse an empty set per corpus. PHASE-01's notes say the discard witness has no red step of its own.)*

**Outcome:** verified — PHASE-01/VA-4's `NotAString` R-25 → R-3 is outside both fixtures' lists ([R-21, R-25] and [R-25, R-51], checked against the tree), and VT-3 adds a per-corpus non-vacuity guard.

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

**Disposition:** doc-wrong
**Response:** I-1's check reads non-comment code only. *(Repair: I-1's command, stated once under §Sequencing *Invariant reads*, drops whole-line comments and has each remaining hit read; PHASE-04/VA-1 and PHASE-12/VA-1 cite it.)*

**Outcome:** verified — *Invariant reads* drops whole-line comments, `///` and `//!` included, and records trailing comments. The R-56 condition's own `AtFault::` hit is F-27.

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

**Disposition:** doc-wrong
**Response:** The rule is scoped to non-test code; PHASE-04's probe names `HOST_SOURCE`. *(Repair: PHASE-02/EX-6 scopes the rule to code outside comments and `#[cfg(test)]` modules, with each grep hit read and recorded; PHASE-12/EX-1 has the probe's event source be `HOST_SOURCE`, and I-1's command greps for a `"host"` literal.)*

**Outcome:** verified — PHASE-02/EX-6 is scoped to non-test code with each hit read; PHASE-12/EX-1 has the probe name `HOST_SOURCE`, and I-1's command catches a `"host"` literal.

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

**Disposition:** doc-wrong
**Response:** EX-4 names the tests that change; the choice expectation is the fixture's literal id, not `alternatives.first()`. *(Repair: PHASE-02/EX-4 names the two tests EX-7 reaches, `a_choice` and `as_drawn_answers_every_kind`, and makes the choice expectation the fixture's literal id `"first"`.)*

**Outcome:** verified — PHASE-02/EX-4 uses the fixture's literal id; `view_model.rs`' `a_choice` fixture's first alternative is `{ "id": "first" }`, and `AlternativeId::as_str` makes the comparison writable from `goad`.

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

**Disposition:** doc-wrong
**Response:** The conversion is a private `as_edited` in `view_model.rs` beside `adjusted` (`design-log.md` 2026-10-01). *(Repair: `design.md` §5.2.4 places a private `as_edited` in `view_model.rs` beside `adjusted` and moves the round trip to `view_model.rs`' tests, §9 naming it; PHASE-02/EN-2 and EX-3 follow it, and new PHASE-02/VT-4 holds the round trip, VT-3 keeping only the projection.)*

**Outcome:** verified — `design.md` §5.2.4 and PHASE-02/EX-3 put a private `as_edited` beside `adjusted`, keep `draft.rs` free of `view_model`, and move the round trip to `view_model.rs` (VT-4).

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

**Disposition:** doc-wrong
**Response:** Parallel running requires the orchestrator to own `notes.md` §Status and to make `canon-delta.md` test-name edits at merge; sequential stays the default. *(Repair: §Sequencing's parallel paragraph says `notes.md` and `canon-delta.md` are shared, and that a phase run in parallel writes neither §Status nor `canon-delta.md`, the orchestrator applying both at merge; PHASE-11's note cites the rule.)*

**Outcome:** verified — §Sequencing now says which files are shared and gives the orchestrator the §Status and `canon-delta.md` edits at merge; PHASE-11's note points at the same rule. What the rule leaves is F-26.

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

**Disposition:** doc-wrong
**Response:** Split: the run itself becomes PHASE-12 (`design-log.md` 2026-10-01). *(Repair: PHASE-04 keeps the crate, both command forms, the steps before the first exchange (new EX-7), the report writer (EX-3 narrowed), the statuses and the status-2 cases (VT-2 narrowed); the run moves to new PHASE-12. PHASE-04/EX-2 and VT-3 are removed, now PHASE-12/EX-1 and VT-3; the diagram, rationale, Coverage, §9 table and PHASE-05/EN-1 are updated.)*

**Outcome:** verified — The split resolves: every `PHASE-04/…` and `PHASE-12/…` citation in `plan.md`, `notes.md` and `design-log.md` names a criterion that exists (grep of `PHASE-(04|12)/[A-Z]{2}-[0-9]+`), and the §9 table splits the binary cases by phase. The interim state on `main` raises F-23.

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

**Disposition:** doc-wrong
**Response:** Each example's gate test also asserts at least one view answered. *(Repair: PHASE-06/VT-1 also asserts each report shows a view answered, and PHASE-06/VA-5 makes an example silent to show it reds; PHASE-06's notes say why each example speaks at any hour.)*

**Outcome:** verified — PHASE-06/VT-1 asserts a respond line and no no-view line; VA-5's silent-example mutation reds it.

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

**Disposition:** doc-wrong
**Response:** Cases or VA reads for each named property. *(Repair: PHASE-12/VT-1 adds `a_backend_failing_at_startup_is_still_asked_the_rest` and `event_files_are_sent_in_the_order_given`; PHASE-12/EX-5 and VA-3 hold no socket and no environment change; PHASE-04/VA-6 and PHASE-12/VA-4 read `main` for R-13 and R-15.)*

**Outcome:** verified — PHASE-12/EX-2, EX-5, VT-1's two added cases, VA-3's grep; PHASE-04/EX-4 and VA-6, re-read at PHASE-12/VA-4.

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

**Disposition:** doc-wrong
**Response:** The coverage test requires each id in a heading line under `reference/`; PHASE-08 criteria for "the specs are not shipped" and "nothing host-internal". *(Repair: PHASE-08/VT-1 counts an id only in a heading line under `reference/`, and VT-2's negative control adds a body-only id; PHASE-07/EX-3 puts each id in a heading; PHASE-08/EX-5 and VA-1 hold the specs-not-shipped statement and nothing host-internal across every reference file.)*

**Outcome:** verified — PHASE-07/EX-3 puts each id in a heading line, PHASE-08/VT-1 counts only heading lines under `reference/`, VT-2 controls it; PHASE-08/EX-5 and VA-1 hold the not-shipped statement and host-internal reading. The heading rule raises F-24, and its id set F-25.

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

**Disposition:** doc-wrong
**Response:** `plan-log.md` 2026-10-01, *plan review round 1: dispositions*, supersedes PL-4's rationale and names the POL-001 sentence departed from; the recipe comment is true at each step; VA-1 wording corrected. *(Repair: PHASE-03/EX-3 names the POL-001 §Statement sentence departed from and cites `plan-log.md` 2026-10-01, *plan review round 1: dispositions*; its comment and PHASE-06/EX-3's are true at each step; PHASE-03/VA-1 and PHASE-06/VA-1 compare against POL-001's block with one line replaced. §Owed to audit notes Change 1 ends the departure.)*

**Outcome:** verified — PHASE-03/EX-3 names the departure and its reason (`plan-log.md` supersedes PL-4's rationale); VA-1 in PHASE-03 and PHASE-06 compares against POL-001's block with one line replaced; each step's comment is true of that step.

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

**Disposition:** doc-wrong
**Response:** The scanner moves to `tests/support/`, shared with `round_trip.rs`; the binary tier includes existing support files where they fit; any forced copy named in FU-5's extension (`design-log.md` 2026-10-01). *(Repair: `design.md` §5.2.6 moves the scanner to `tests/support/`, shared with `round_trip.rs`, and states the binary tier's include rule; PHASE-07 §Surfaces, EX-4, VT-7 and VA-4, and PHASE-04/VA-7 and PHASE-12/VA-7 carry it; `notes.md` §Open's FU-5 bullet is corrected.)*

**Outcome:** verified — `design.md` §5.2.6 shares the scanner from `tests/support/`; PHASE-07/EX-4, VT-7, VA-4; PHASE-04/VA-7 and PHASE-12/VA-7 name any copied helper in `notes.md` §Open's FU-5 row.

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

**Disposition:** doc-wrong
**Response:** Surfaces added. *(Repair: PHASE-03 §Surfaces gains `canon-delta.md` (VT-1 says when it changes); PHASE-02 §Surfaces gains `crates/goad/Cargo.toml`, and EX-5 its comment. Sibling: PHASE-12 and PHASE-07 list their shared and `canon-delta.md` surfaces.)*

**Outcome:** verified — PHASE-03 §Surfaces gains `canon-delta.md` with VT-1's re-point clause; PHASE-02 §Surfaces gains the manifest comment, and EX-5 holds it.

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

**Disposition:** doc-wrong
**Response:** Counts replaced by names. *(Repair: the counts named in the finding are replaced by names, and a sweep replaced the rest over open lists — the fixture lists, the example directories, the marketplace manifests, the pending packages, the reference files and the statuses.)*

**Outcome:** verified — A rescan finds only closed or finished counts (`two phases`, `two --event files`, the four sides, two for `InapplicableKey`).

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

**Disposition:** doc-wrong
**Response:** All three corrected. *(Repair: PHASE-05/VA-1 checks `$CODEX_HOME/plugins/cache/`; PHASE-11/EX-2 spells `claude -p`; PHASE-05/EX-1 checks the built `--version` prints the revision.)*

**Outcome:** verified — PHASE-05/VA-1 reads `$CODEX_HOME/plugins/cache/`; PHASE-11/EX-2 has `-p`; PHASE-05/EX-1 and PHASE-04/EX-8 carry `GOAD_REVISION`.

### F-22 — I-5's rule reads whatever the checkout holds, and refuses consumer paths that name nothing in this repository

**Severity:** minor
**Location:** `design.md` §5.2.6 *The kit stands alone*; `plan.md` PHASE-07/VT-5, VT-6

**Expected:** A gate test's verdict is a property of the committed tree, and
I-5 refuses a reference to *this repository's* paths outside `kit/`.
**Observed:** The repair was flagged for a second look, and the evidence
supports two problems.
- **The verdict depends on the checkout.** "The root's entries are read when
  the test runs" means the entries of the working directory, and that
  includes untracked and ignored ones. The same commit can therefore pass on
  a clean clone and fail here, or the reverse, depending on what local
  scratch directories exist.
- **It refuses generic paths.** The rule refuses any `<name>/` whose `<name>`
  is a root entry. Several root entries share names with paths a backend
  author's own project uses, and these name nothing in this repository:
  - `.claude/`, e.g. a project's `.claude/skills/`, is an entry here, since
    the checkout holds `.claude/worktrees/`.
  - `tests/` and `docs/`.
  - `.claude-plugin/`, which the kit itself contains as
    `kit/.claude-plugin/`. The rule exempts that spelling only when a path
    character precedes it. SKILL.md or an install line that writes
    `` `.claude-plugin/plugin.json` `` in backticks is refused.

  None of these is a reference to a path outside `kit/`.
**Evidence:** `comm -23 <(ls -A | sort) <(git ls-files | cut -d/ -f1 | sort -u)`
at the repository root → `.claude .direnv .envrc .git goad-demo.sock
goad-demo.sock.lock spike-fields target`. Any of these, and any directory a
developer adds, joins the refused set on that machine only. `flake.nix` sets
`doCheck = false` on every derivation, so the test runs only in a checkout.

**Disposition:** doc-wrong
**Response:** I-5's mention rule reads the tracked tree: the test lists tracked paths with `git ls-files` and refuses a mention `<name>/<segment>` only if that path prefix is tracked outside `kit/`; a missing `git` or `.git` fails the test, never skips it. The escaping-`../` half stands. The negative control gains a generic consumer path (`.claude/skills/`) and a backticked `` `.claude-plugin/plugin.json` ``, both accepted. `design-log.md` 2026-10-01, *plan review round 2: I-5 reads the tracked tree*.

**Outcome:**

### F-23 — Between PHASE-04 and PHASE-12, `main` carries a checker that accepts every backend without running it

**Severity:** minor
**Location:** `plan.md` PHASE-04/EX-3, EX-4; §Sequencing *Why the checker is two phases*

**Expected:** Every commit on `main` is green, and green means something true
about the binary. SPEC-004's draft R-11 reads 0 as "the host reported
nothing" of what the checker sent (`canon-delta.md` SPEC-004 Change 4:
"would have reported nothing for the requests the checker sent").
**Observed:** PHASE-04 ends with "a run that makes no exchange". Its report
is the no-view line and a verdict, so `goad-check --config <anything
loadable>` exits **0, accepted** without spawning the command. That includes a
command that does not exist, which PHASE-12 reports as `Spawn`, status 1.
Nothing in PHASE-04 refuses this state or marks it: EX-3 says "no test asserts
a status-0 or status-1 verdict". The plan's own PARTIAL rule (§Sequencing
*Size*) lets PHASE-12 span sessions, and this binary is what `main` ships
while it does. An interim run that ends with no verdict, status 2, would be
true and would need no rework later.
**Evidence:** `plan.md` PHASE-04/EX-3 ("Until PHASE-12 a run makes no
exchange, so its report is the no-view line and the verdict"); §Sequencing
("It ends green, with a run that makes no exchange"); PHASE-12/VT-1's
`an_unspawnable_command_is_reported_against_the_configuration`.

**Disposition:** doc-wrong
**Response:** Until PHASE-12 a run ends with no verdict: the report's no-view line, a stderr line saying the run is not yet implemented, and status 2. PHASE-12 replaces it with the verdict. Nothing on `main` reports acceptance it did not judge.

**Outcome:**

### F-24 — PHASE-08/VT-1 counts an id in "a Markdown heading line", and a `#` comment inside a fenced block reads as one

**Severity:** minor
**Location:** `plan.md` PHASE-08/VT-1, VT-2; PHASE-07/EX-3

**Expected:** The coverage test finds an id only where a reader finds its
explanation: at a heading (F-16's repair).
**Observed:** The repair was flagged for a second look. The reference's
fences are full of lines that begin `#`: `toml goad:config` blocks, and the
examples' shell and Python code that the reference shows. A `# SPEC-001/R-36`
comment in a TOML fence is a "line beginning `#`". A line-based reading
counts it as a heading, and no explanation exists there. VT-1 does not say
that fenced lines are skipped, although the shared scanner (PHASE-07/EX-4)
already knows where fences are. VT-2's control holds body text and not a
fenced comment, so an implementation that counts fenced `#` lines passes it.
There is also a smaller gap. PHASE-07/EX-3 asks for "each id in a heading
line **of its own**", but VT-1 counts every id in any heading, so one heading
naming five ids satisfies the test.
**Evidence:** `plan.md` PHASE-08/VT-2's inline string: a heading, an absent
id, and body text, with no fence. `design.md` §5.2.6's role table: the
`toml goad:config` role, whose TOML comments start with `#`.

**Disposition:** doc-wrong
**Response:** PHASE-08/VT-1 reads headings through the shared scanner (PHASE-07/EX-4), skipping fenced lines; a heading counts for an id only if it names exactly that one requirement id, matching PHASE-07/EX-3. VT-2's control gains a fenced `#` line naming an id that appears nowhere else, which must be reported absent, and a heading naming two ids.

**Outcome:**

### F-25 — The coverage test's id set omits R-56, which the checker's own report prints

**Severity:** minor
**Location:** `plan.md` PHASE-08/VT-1, EX-5; `design.md` §5.2.2, §5.2.6

**Expected:** PHASE-08/EX-5 has the reference say "that every id a report
prints is explained in the reference itself", and F-32's point is that a
report line leads a reader straight to its explanation.
**Observed:** VT-1's set is "every requirement a refusal can name", built from
one instance per taxonomy variant. The checker also prints `SPEC-001/R-56`
on the probe's condition, and that id is the checker's own claim, not any
variant's answer. No variant answers R-56, so the gate never requires it in
the reference. EX-5's statement is then held for R-56 only by VA-1's reading.
**Evidence:** `design.md` §5.2.3's table has no row answering R-56.
`design.md` §5.2.5's sample report prints `backend  SPEC-001/R-56  a backend
MUST tolerate a kind it does not recognise`. PHASE-12/EX-3 prints "the R-56
line only on its condition".

**Disposition:** doc-wrong
**Response:** The checker's own R-56 claim id joins VT-1's coverage set, taken from the constant the report prints, not respelled.

**Outcome:**

### F-26 — The parallel rule still has every parallel phase append a phase sheet at one place in `notes.md`

**Severity:** minor
**Location:** `plan.md` §Sequencing (*PHASE-01, PHASE-02 and PHASE-03 may run in parallel*); PHASE-11 notes

**Expected:** F-12's repair: a phase run in parallel leaves the shared files
to the orchestrator at merge.
**Observed:** The rule moves each parallel phase's status and test-name
changes "in its own phase sheet", and phase sheets live in `notes.md` under
`## Phase sheets` (`docs/AGENTS.md` §Phase plan). Two or three worktrees
therefore each append a sheet after the same template block. Git then reports
overlapping hunks at the same location in the same file, and the orchestrator
has to resolve them by hand. The rule claims `notes.md` is taken out of the
parallel phases' hands, but it moves the writes rather than removing them.
**Evidence:** `notes.md` §Phase sheets holds one template block, `### PHASE-01
— <name>`, and nothing after it but `## Harvest`, so every new sheet lands
there.

**Disposition:** doc-wrong
**Response:** PHASE-01..PHASE-03 run sequentially; the parallel rule for them is dropped. PHASE-11 alone may run beside PHASE-06..PHASE-08, and the orchestrator writes its phase sheet and its `notes.md` §Status row on `main`, so the worktree touches neither.

**Outcome:**

### F-27 — Loose ends in the repaired criteria

**Severity:** nit
**Location:** `plan.md` PHASE-12/VA-1, PHASE-12 §Surfaces, PHASE-05 §Surfaces, PHASE-08/EX-2

**Expected:** Criteria that the intended code meets as written.
**Observed:**
- PHASE-12/VA-1 says "the only hit outside a comment is the R-56 probe's
  claim". The R-56 *condition* ("only when its `fault()` is backend",
  `design.md` §5.2.2) compares with `AtFault::Backend` and is a second hit on
  I-1's grep. Its wording should name the condition as well.
- PHASE-12's binary cases read a backend's log of request kinds
  (`a_backend_failing_at_startup_is_still_asked_the_rest`,
  `event_files_are_sent_in_the_order_given`). If parsing that log needs a
  dev-dependency, `crates/goad-check/Cargo.toml` is not in PHASE-12's
  surfaces.
- PHASE-05's marker line has no stated fate. PHASE-08/EX-2's finished
  `SKILL.md` "routes and does not teach", which implies the marker goes, but
  neither PHASE-08 nor PHASE-11 says whether it stays for any later load
  check.
**Evidence:** `plan.md` PHASE-12 §Surfaces (`src/`, `tests/binary/`,
`canon-delta.md`); PHASE-05 §Surfaces (the marker); PHASE-08/EX-2.

**Disposition:** doc-wrong
**Response:** PHASE-12/VA-1 names both expected `AtFault::Backend` hits: the R-56 condition and the probe's claim. PHASE-12 §Surfaces gains `crates/goad-check/Cargo.toml`. PHASE-08 removes the PHASE-05 marker as an exit criterion; PHASE-11's load check, if entered after PHASE-08, asks for a fact only the finished `SKILL.md` body states.

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

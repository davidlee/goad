# Plan log — Slice 012

Append-only record of the plan *conversation* — what was asked, what was
decided, in time order. Never rewrite an entry; supersede it with a later one.

Only decisions live here. An adversarial review of the plan is owned by its
ledger (`review-plan.md`).

## Decisions

### 2026-10-01 — placements the plan draft put to the user

- **Asked:** the planner's draft (05e017c) put seven placements to the user;
  the orchestrator amended one (PL-7's phase).
  - **PL-1 — the `config::Command` doc contradiction** (`design.md` §5.2.1,
    F-40): repair the doc in PHASE-03 to name the routes that hold the empty
    command out (`Config::parse`, `from_argv`); leave `Command::new` and the
    public fields open, since tests in three crates build `Command` directly.
  - **PL-2 — `HOST_SOURCE`** (§5.2.4 *opportunity*): in PHASE-02, which then
    touches `envelope.rs`.
  - **PL-3 — `DrawnKind::Choice.first` removed** in PHASE-02's refactor step;
    `as_drawn` is its only user.
  - **PL-4 — the `justfile` and POL-001.** Mid-slice the recipe follows
    `canon-delta.md` POL-001 Change 1, the working authority: PHASE-03 changes
    the exerciser path, PHASE-06 adds the kit path. POL-001 is amended at
    audit.
  - **PL-5 — the walk script** lives in `docs/slices/012/walk/`, not
    `goad-walk`: the capsule clones `goad-walk`, and its agent would read the
    prompt and the control's patterns.
  - **PL-6 — `just package` and `just install`** gain `goad-check` (and
    `package`, `goad-kit`) in PHASE-05. The design did not name them.
  - **PL-7 — `version_line`** is lifted into `goad-shell`'s `report` module,
    so `goad-check` does not add a third copy beside `goad`'s and
    `goad-emit`'s. The planner put it in PHASE-04; the orchestrator
    recommended PHASE-03, which already touches `goad-emit`, keeping PHASE-04
    to the new crate.
- **Also asked:** an adversarial plan review, bounded — a fresh raiser,
  rounds stopping on a measured trend (two rounds with no plan defect).
- **Recommended:** each as listed; PL-7 in PHASE-03; the review.
- **Decided:** *"i'll take your recommendations"*.
- **Consequence:** `plan.md` folds PL-1..PL-7; the `config::positive_duration`
  change (`design-log.md` 2026-10-01, G2) joins PHASE-03 beside `from_argv`'s,
  since both open `config.rs` for the checker. `review-plan.md` is opened.

### 2026-10-01 — the capsule splits out of PHASE-05

- **Asked:** the user said oubliette needs a flake exporting the walk's
  environment (`goad-walk`'s tool set, whose stub at 99cc374 already exists)
  and one or two slices of its own work to support more than one target
  repository. PHASE-05 bundled host-side work with the capsule, so that wait
  would hold PHASE-06..PHASE-08, which do not need it. Proposed: PHASE-05
  keeps the packages, manifests, validation, `goad-walk`'s full tool set, and
  both agents loading the plugin on the host in a fresh home (R1 as far as
  the host reaches it); a new PHASE-11 takes the walk script, the negative
  control, the plugin loads in the capsule and `capsule-collect`, entered when
  oubliette can take `goad-walk`, done before PHASE-09, and free to run beside
  PHASE-06..PHASE-08. The cost: R7's mitigation, "before any kit prose",
  becomes "as early as oubliette allows"; the kit's prose does not depend on
  the walk's venue, so R7 firing late changes the walk, not the reference.
- **Recommended:** split.
- **Decided:** *"yes, that's right. sure, let's split it."*
- **Consequence:** `plan.md` PHASE-05 narrowed, PHASE-11 added, sequencing,
  coverage and PHASE-09/EN-2 updated; `notes.md` §Status gains PHASE-11.

### 2026-10-01 — plan review round 1: dispositions

- **Asked:** `review-plan.md` round 1 (b407b8b): no blocker, F-1..F-21.
  Every finding proposed `doc-wrong`. The design-touching five (F-4, F-6,
  F-11, F-13, F-18) are recorded in `design-log.md` 2026-10-01, *plan review
  round 1: design-touching dispositions*. The rest, as proposed:
  - F-1 PHASE-05 gains a credentials entry for the fresh home; its STOP
    separates an authentication failure from R1.
  - F-2 the placeholder `SKILL.md` carries a body-only marker; PHASE-05/EX-8
    and PHASE-11/EX-2 ask for a fact only the body states at the pinned
    revision.
  - F-3 PHASE-09 owns the lock bump to PHASE-08's revision.
  - F-5 reach re-proven where later phases add kit files; the coverage test
    and the example test enumerate their sets, refusing an empty one;
    PHASE-10 re-reads I-1, I-2 and fence reach.
  - F-7 a mutation reds the discard witness; a non-vacuity guard.
  - F-8 I-1's grep reads non-comment code. F-9 the `"host"` rule scoped to
    non-test code; the probe names `HOST_SOURCE`. F-10 the choice
    expectation is the fixture's literal id.
  - F-12 parallel phases need the orchestrator to own `notes.md` §Status and
    the `canon-delta.md` test-name edits at merge; sequential stays default.
  - F-14 each example's gate test asserts a view answered. F-15 the named
    run-sequence and structural properties get cases or VA reads. F-16 ids
    sit in a heading under `reference/`; "the specs are not shipped" and
    "nothing host-internal" get criteria.
  - F-17 **supersedes PL-4's rationale**: the recipe departs from POL-001
    §Statement's "policy first, recipe second" mid-slice because canon is
    not edited mid-slice (`docs/AGENTS.md` §Canon that does not exist yet)
    and the rename would otherwise break the gate; POL-001's block names a
    path that no longer exists from PHASE-03 until audit promotes the delta.
    The recipe's comment is true at each step.
  - F-19..F-21 mechanical.
- **Recommended:** as listed.
- **Decided:** *"yeah go ahead"*.
- **Consequence:** dispositions in the ledger; a fresh agent repairs
  `plan.md` and `design.md`; the raiser runs round 2.

### 2026-10-01 — plan review round 2: dispositions

- **Asked:** `review-plan.md` round 2 (4981d5f): F-1..F-21 verified; new
  F-22..F-27, all proposed `doc-wrong`. F-22 changes `design.md` and is
  recorded in `design-log.md` 2026-10-01, *plan review round 2: I-5 reads the
  tracked tree*. The rest:
  - F-23 until PHASE-12 a run ends with no verdict — status 2, a stderr line —
    so `main` never reports an acceptance it did not judge.
  - F-24 the coverage test skips fenced lines through the shared scanner; a
    heading counts only when it names exactly one requirement id; the control
    gains a fenced `#` case and a two-id heading.
  - F-25 the checker's R-56 claim id joins the coverage set.
  - F-26 **supersedes F-12's parallel rule for PHASE-01..PHASE-03**: they run
    in sequence. PHASE-11 alone may run beside PHASE-06..PHASE-08; the
    orchestrator writes its phase sheet and status row on `main`.
  - F-27 PHASE-12/VA-1 names the R-56 condition's `AtFault::Backend` hit;
    PHASE-12 surfaces gain `crates/goad-check/Cargo.toml`; PHASE-08 removes
    the PHASE-05 marker, and PHASE-11's load check after that asks for a fact
    of the finished `SKILL.md` body.
- **Recommended:** as listed; F-22 option A.
- **Decided:** *"yes, go ahead"*; F-22 *"A."*
- **Consequence:** dispositions in the ledger; a fresh agent repairs
  `plan.md` and `design.md`; round 3 verifies these six repairs only.

### 2026-10-01 — plan review round 3: close

- **Asked:** round 3 (e4e3dc1) verified F-22..F-27 and raised two minors,
  both introduced by round 2's repairs: F-28 (PHASE-08/VT-1 allowed a new
  scanner symbol outside its surfaces) and F-29 (PHASE-12/VA-1 left out the
  R-56 constant's own hit). Proposed: both `doc-wrong`; the orchestrator
  repairs them and verifies them mechanically instead of a round 4, though
  the agreed stop (two rounds with no plan defect) was not reached — the
  trend runs seven majors, then none, then two minors made by the repairs.
- **Recommended:** as proposed.
- **Decided:** *"yeah"*.
- **Consequence:** `plan.md` PHASE-08/VT-1 and PHASE-12/VA-1 repaired;
  `review-plan.md` resolved with its Synthesis.

### 2026-10-01 — plan accepted

The user accepted `plan.md` as committed at 717e57f, with PL-1..PL-7 standing
as amended by the plan review, and `review-plan.md` resolved. The risks the
review's Synthesis leaves standing are accepted with it. The user's global
`core.hooksPath` was raised and checked against I-5: `git ls-files` runs no
hook, so the rule does not depend on it. PHASE-11 stays gated on oubliette's
multi-target work and `goad-walk`'s registration, both outside this slice.

### 2026-10-01 — PHASE-01's red-first order (PHASE-01 sheet)

- **Asked:** the PHASE-01 sheet (aba6509, PLAN QUESTION 2): the plan's Notes
  put the fixture witnesses first, but they call `requirement()` and do not
  compile without it — a compile failure is not EX-5's red. Proposed order:
  `Requirement` and stratum 1's `requirement()`, red by VT-1's tables; then
  the witnesses, red on exactly EX-4's fixtures (EX-5); then the list
  corrections; then `fault()`, stratum 2 and `AtFault`'s `Display`.
- **Recommended:** the proposed order; EX-5 holds under it.
- **Decided:** *"ill take it"*.
- **Consequence:** PHASE-01's Notes corrected; the sheet's tasks follow.
  `Requirement`'s constants: `design-log.md` 2026-10-01, *`Requirement` is
  built from named constants*.

### 2026-10-01 — PHASE-02 sheet questions

- **Asked:** the PHASE-02 sheet (ccefdaa) raised three PLAN QUESTIONs.
  - **1 — EX-6's `"host"` rule.** Two non-test literals remain by
    construction: `HOST_SOURCE`'s own definition, and `AtFault::Host`'s
    printed word, which PHASE-01 added after F-9 scoped the rule. Same
    spelling, different meanings. Proposed: EX-6 admits exactly those two,
    each read and recorded; `AtFault` is not routed through `HOST_SOURCE`.
  - **2 — the drawn number.** `design-log.md` 2026-10-01, *the drawn number
    has one home: `NumberRange::drawn`*.
  - **3 — VT-2's overlapping names.** The moved
    `each_kind_submits_the_json_type_r_57_names` becomes
    `every_submitted_kind_writes_the_json_type_r57_names` (the name
    `canon-delta.md` already cites) and gains the boolean clause; no
    second case asserting the same types.
- **Recommended:** 1 as proposed; 2 (b); 3 as proposed.
- **Decided:** *"yes"*.
- **Consequence:** `plan.md` PHASE-02 EX-3, EX-6, VT-2 amended; the sheet's
  blocked tasks unblocked.

### 2026-10-01 — PHASE-02's `glass.rs` comment; PHASE-03 sheet questions; the push before a lock bump

- **Asked:**
  - **PHASE-02's `glass.rs` edit.** PHASE-02 (1e86bde) changed one comment in
    `glass.rs` — whose surface was imports and the delegation only — from
    `view_model::drawn_number` to `NumberRange::drawn`, because EX-3 required
    `grep -rn 'drawn_number' crates` to come back empty. Proposed: accept it.
  - **PHASE-03 sheet (1b5639f), PLAN QUESTION 1.** EX-8 deletes
    `diagnostics::version_line`; `startup.rs`' doc table on `arguments`
    links to it and `goad`'s `tests/binary/main.rs` module doc names it.
    Proposed: both join PHASE-03's surfaces, doc-only.
  - **PLAN QUESTION 2.** The exercisers carry present-tense "copy this"
    claims beyond EX-1's two (`round_trip.rs`'
    `the_readme_s_own_config_loads_and_runs_the_example` doc, `backend.sh`'s
    header, the TypeScript README's opening). Proposed: EX-1 read as its
    Objective reads — every present-tense claim rewritten, the grep recorded.
  - **PL-7's home** (orchestrator, from the sheet's finding):
    `goad_shell::report`'s module doc says it is "not a formatter" — what a
    line says belongs to its composer. Proposed: `version_line(version,
    revision)` lives in a module of its own, `goad_shell::version`, and
    `report` stays as its doc says. Supersedes PL-7's placement only.
  - **The push before a lock bump.** `goad-walk` now pins goad from
    `github:davidlee/goad`, so a lock bump can pin only a pushed commit.
    Proposed: PHASE-05, PHASE-09 and PHASE-10 each gain an entry checklist
    item, "goad's `main` pushed to `origin`", before their lock bump.
- **Recommended:** each as proposed.
- **Decided:** *"yeah I'll take your recommendations on all the above"*.
- **Consequence:** `plan.md` PHASE-03 (surfaces, EX-1, EX-8), PHASE-04/EX-8,
  PHASE-05, PHASE-09 and PHASE-10 entry criteria amended; the PHASE-03 sheet
  follows.

### 2026-10-01 — oubliette ready for `goad-walk` (PHASE-11/EN-2)

The user reports oubliette's prerequisites done: `goad-walk` is on GitHub,
pinning goad from `github:davidlee/goad`, and oubliette can take it as a
target. PHASE-11/EN-2 is met on the user's word; the phase sheet re-reads it
against oubliette's `docs/contract-target.md`. PHASE-11 still waits on
EN-1 (PHASE-05 done) and EN-3 (credentials for the capsule session).

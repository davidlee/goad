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

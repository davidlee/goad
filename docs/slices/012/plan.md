# Plan — Slice 012: the backend author's kit

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

<!-- Phase ids (PHASE-NN) and criterion ids (EN-/EX-/VT-/VA-/VH-N) are
     immutable: edits append, never renumber, so the sequence goes
     non-monotonic after a split and that is expected. Criterion ids are local
     to their phase — cite another phase's phase-qualified (PHASE-03/EX-2).
     Verification modes — VT: automated test. VA: agent check. VH: human
     acceptance.
     Progress is NOT recorded here. Status lives in `notes.md`. -->

## Overview

**Every phase's last exit criterion, unwritten below, is `just check` exiting
0** on the phase's final commit. The host learns to say what it
already knows first; then the checker is built on it; then the plugin is stood
up in a capsule before any prose is written for it; then the kit is written
and gated; then it is walked.

- **PHASE-01 — refusals name a requirement and a side.** `Requirement` and
  its named constants, `AtFault` (each with its `Display`), and total
  `requirement()`/`fault()` on every taxonomy in strata 1 and 2 (`design.md` §5.2.3). The corpus
  witnesses, red on the fixtures whose lists §5.2.3 corrects and green after.
- **PHASE-02 — the host's kinds and R-57 values live in stratum 1.**
  `Stimulus`, `Submitted`, `Finite` and `Submitted::as_drawn` move or are
  lifted to `goad_semantics::protocol::canonical`, with
  `NumberRange::drawn` the one home of the drawn number; `goad` delegates
  (`design.md` §5.2.4).
- **PHASE-03 — the ground the checker stands on.** `examples/` becomes
  `exercisers/` (`design.md` §5.2.7); `goad-emit`'s unwritten answer exits 2
  (§5.2.5); `config::Command::from_argv` and `config::positive_duration`
  are public (§5.2.1); `version_line` has one home, `goad_shell::version`
  (`plan-log.md` PL-7, its placement amended 2026-10-01).
- **PHASE-04 — `goad-check`: the crate and its edges.** The binary, its
  arguments in both forms, the steps before the first exchange, the report
  writer and status 2, with every status-2 case in the binary tier; a run
  ends with no verdict, status 2, until PHASE-12 (`design.md` §5.2.1,
  §5.2.5).
- **PHASE-12 — `goad-check`: the run.** The request plan, answering and
  chains, the report of every channel with its refusal lines, the R-56
  condition, and the verdict with statuses 0 and 1, with the binary cases
  that need a run (`design.md` §5.2.2, §5.2.5, §5.4).
- **PHASE-05 — packages and the plugin's shell.** The flake exports
  `goad-check` and `goad-kit`; the manifests and a placeholder `SKILL.md`
  exist and validate; both agents load the plugin from its store path on the
  host; `goad-walk` builds the full tool set (`design.md` §5.2.6, §5.2.8; R1).
- **PHASE-11 — the capsule.** The walk script; one capsule stood up, its
  negative control passed, both agents loading the plugin in it, and a tree
  collected (`design.md` §5.2.8; R1, R7). Waits on oubliette.
- **PHASE-06 — the examples.** The kit examples — focus check, Downloads
  triage and breadcrumbs — each accepted by the checker in the gate having
  answered a view; `python3` and `jq` in the devshell; the kit example's
  `deno check` path (`design.md` §5.2.6 *The examples*).
- **PHASE-07 — the fence gate, and the protocol and transport reference.** The
  shared fence scanner and every role's check; `protocol.md` and
  `running.md`; I-5's path test.
- **PHASE-08 — the rest of the reference, and its coverage.** `scheduling.md`,
  `events.md`, `checking.md`, the finished `SKILL.md`; the coverage test over
  every requirement a refusal can name, and the checker's R-56 claim.
- **PHASE-09 — the first walks.** One Claude Code walk and one Codex walk,
  measured, verdict-checked, run by a person, read, and every friction item
  dispositioned (`design.md` §5.2.9).
- **PHASE-10 — kit fixes, and the re-walks.** The kit fixes land, `goad-walk`'s
  lock moves to them, and each agent walks once more.

### Owed to audit and close

No phase can do these: they are audit's or close's by `docs/AGENTS.md`. Each is
also a checklist item — PHASE-10/VA-4 checks that `notes.md` §Open carries
every one, so none is left to prose.

- **Promote `canon-delta.md`** — every change it holds, for SPEC-001,
  SPEC-004, POL-001 and ADR-003 — with the user's endorsement, recording each
  in `audit.md`'s Reconciliation table. AC-4 and AC-7's canon half land only
  here. POL-001 Change 1 also ends the recipe's departure from POL-001
  §Statement (PHASE-03/EX-3).
- **Record the person-runs** in `audit.md` §Evidence, citing the VH criteria
  below by id.
- **Extend FU-5 and FU-7** in `docs/follow-ups.md` from `notes.md` §Open's
  rows, which name the shipped symbols (PHASE-04/VA-5, VA-7;
  PHASE-12/VA-7; PHASE-07/VA-4).
- **Disposition every other §Open candidate** at close.

**Test names are commitments.** `design.md` §9 and `canon-delta.md` name the
cases. A phase that ships a case under another name or in another file updates
`canon-delta.md` in the same commit and says so in its phase sheet (the draft
is the working authority, `docs/AGENTS.md` §Canon that does not exist yet).
Canon keeps its old citations until audit promotes the delta.

## Sequencing & rationale

```
PHASE-01 ─► PHASE-02 ─► PHASE-03 ─► PHASE-04 ─► PHASE-12 ─┐
(refusals)  (kinds)     (ground)    (crate,     (the run)  │
                                    status 2)              │
┌──────────────────────────────────────────────────────────┘
└─► PHASE-05 ─► PHASE-06 ─► PHASE-07 ─► PHASE-08 ─┬─► PHASE-09 ─► PHASE-10
    (packages)  (examples)  (fences)    (coverage) │   (walks)     (fix, re-walk)
        │                                          │
        └─► PHASE-11 (capsule; waits on oubliette)─┘
```

**Why the host changes come first.** The checker only prints what the host
knows (`design.md` §4, principle 1). It cannot be written until each taxonomy
answers `requirement()` and `fault()` and each answer prints itself
(PHASE-01), until the kinds and R-57 values it sends have a stratum-1 home
(PHASE-02), and until it can build a command by the host's own rule
(PHASE-03).

**PHASE-01, PHASE-02 and PHASE-03 run in sequence.** Their code surfaces are
disjoint by file, but each writes `notes.md` — its §Status row and its own
phase sheet, both under one heading — and each may edit `canon-delta.md`'s
test names, so in parallel they would collide in the shared files.

**PHASE-11 alone may run in parallel**, beside PHASE-06..PHASE-08, in its own
worktree with one writer. The orchestrator writes PHASE-11's phase sheet and
its `notes.md` §Status row on `main`, from the worktree agent's reports, so
the worktree touches neither; PHASE-11's surfaces hold no `canon-delta.md`
edit.

**Why the checker is two phases.** One phase holding the crate, its edges and
its run does not fit one session with its binary tier. The split is at the
first exchange. PHASE-04 builds everything decided before it: the command
line, configuration and event files, the report writer and every way to
status 2. It ends green, with a run that makes no exchange
and so ends with no verdict, status 2 (PHASE-04/EX-3): `main` never reports
an acceptance it did not judge, however many sessions PHASE-12 spans.
PHASE-12 fills the run: the request plan, the answers, the channel lines, the
R-56 condition, and the verdict with its cut to 0 or 1.

**Why the plugin loads come early, and the capsule splits out** (`design.md`
§8 R1 and R7, and their mitigations; `plan-log.md` 2026-10-01, *the capsule
splits out*). Whether each model reads a skill loaded from a store path has
never been run; PHASE-05 runs it on the host with a placeholder skill, before
any prose, so a blocker costs a manifest and not a reference. PHASE-05 needs
`goad-check` to exist, because `goad-walk`'s tool set holds it; it follows
PHASE-12 rather than running beside it, which costs nothing R1 needs, since
the loads still precede every line of kit prose. The capsule (PHASE-11)
waits on oubliette work outside this slice, so it runs as early as oubliette
allows, beside PHASE-06..PHASE-08, and must be done before PHASE-09. The
kit's prose does not depend on where the walk runs, so R7 firing late changes
the walk's shape, not the reference.

**Why the examples come before the reference.** The reference's examples cite
real backends, and the examples are the kit's highest-value content for the
walk. The examples' gate test also creates `goad-check`'s `kit` test target,
which PHASE-07 extends.

**Why the reference is two phases.** Reading SPEC-001..003 and writing every
reference file with tagged fences does not fit one session with its tests.
The split is at the coverage test: PHASE-07 builds the fence gate and writes
`protocol.md` and `running.md`, where most of the requirement ids live;
PHASE-08 writes the coverage test red, and turns it green with
`scheduling.md`, `events.md` and `checking.md`.

**Why the walks are two phases.** The first walks produce the friction the
kit fixes answer. The re-walk rule (`design.md` §5.2.9) requires the fixes on
`main` and `goad-walk`'s lock moved to that revision before either re-walk.

**Size.** Each phase fits one session, bookkeeping included. PHASE-04 and
PHASE-12 are the largest in code; PHASE-07 and PHASE-08 in prose. PHASE-09,
PHASE-10 and PHASE-11 are long in wall time and in person time, not in
tokens. A phase that reaches its budget unfinished checkpoints PARTIAL in its
sheet and hands over.

**Mutation evidence** goes in the phase's sheet in `notes.md`, under a
**Mutation evidence** heading, one row per mutation: the edit (quoted), the
command, that the mutated build **compiled**, the cases that went red **by
name**, and that the restore is green. A mutation that does not compile is not
evidence (`docs/memory/negative-control-must-compile.md`). Runs use `--no-fail-fast`.
A mutation is applied by copying the file to the scratchpad and copying it
back — never `git checkout` or `git stash` — and `git status` is clean after
each restore. The same rules bind every **reach proof** below: a planted
breach that an instrument is shown to catch. A mutation of a script or a
Markdown file has no build; its row records that the mutated file was the
one the test read.

**Invariant reads** name their command, and every hit is read and recorded.
I-1's command reads non-comment code only: `grep -rnE
'R-?[0-9]+|AtFault::|"(backend|host|configuration|environment)"'
crates/goad-check/src | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'`, which
drops whole-line comments; a hit that remains is read, and a trailing comment
on a code line is recorded as one. `R-?[0-9]+` matches an id in every
spelling I-1 counts: `R-56` in a literal, a bare `R56` constant, and
`Requirement::R56` (`design.md` §5.5 I-1; `design-log.md` 2026-10-01,
*`Requirement` is built from named constants*). I-2's is every `FieldKind`
match in `crates/goad-check`, `src` and `tests`, each read for a kind mapped
to a JSON type or a value.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-11/EX-1, EX-2 (the capsule, its negative control, the plugin in it); PHASE-09/EX-3 (each first walk: control passed, verdict 0 with a view answered, no goad source read) and PHASE-09/VH-1 (a person ran each walk's backend); PHASE-10/EX-4 and PHASE-10/VH-1 (each re-walk, the same) |
| AC-2 | PHASE-07/VT-1 (every json/toml fence in the kit is tagged and checked) with VT-2..VT-4 (its negative controls) and VA-2 (the untag mutation); PHASE-08/EX-1 and VA-2 (the same test over the finished reference, reach proven in a file PHASE-08 writes); PHASE-10/VA-5 (reach for any file the fixes add) |
| AC-3 | PHASE-12/VT-1 (each refusal reported with side and requirement, from the host's `Outcome`) and PHASE-12/VA-1, VA-2 (I-1, I-2 over the finished crate); PHASE-04/VA-1, VA-2 (the same, before the run); PHASE-01/VT-1, VT-2 (the data it prints, its printed form, and each `Requirement` constant's value) |
| AC-4 | PHASE-03/VT-1 (`goad-emit`); PHASE-04/VT-2 (`goad-check`'s status 2) and PHASE-12/VT-1, VT-2 (its statuses 0 and 1, and status 2 mid-run); the statement itself is `canon-delta.md` SPEC-004, **promoted at audit** |
| AC-5 | PHASE-06/VT-1, VT-2 (each example accepted, having answered a view; the triage side effect) and PHASE-06/VA-3 (each example read against the reference and SPEC-001); PHASE-03/EX-1 (the exercisers renamed and no longer presented as the file to copy) |
| AC-6 | PHASE-06/VH-1 |
| AC-7 | PHASE-01/VT-1, VT-2, VT-3 (total methods for every refusal kind; the corpus witnesses) with VA-2 and VA-4 (a mutation that reds each witness); the statement is `canon-delta.md` SPEC-001 Changes 1–2, **promoted at audit** |
| AC-8 | PHASE-09/EX-2; PHASE-10/EX-3, EX-4 and VH-2 (the re-walk does not regress) |
| AC-9 | PHASE-09/EX-4, VA-2; PHASE-10/EX-1, EX-5 |

`design.md` §9's tests, by phase:

| test | phase |
|---|---|
| `every_protocol_error_names_a_requirement_and_a_side`, and its bounds and schedule siblings; `every_side_displays_as_the_word_a_report_prints` | 01 |
| `every_backend_error_names_a_requirement_and_a_side`, and its cleanup and state siblings | 01 |
| `every_refusal_fixture_names_a_requirement_in_its_own_list`, `every_discard_fixture_names_a_requirement_in_its_own_list` | 01 |
| the moved `Stimulus` tests; `every_submitted_kind_writes_the_json_type_r57_names`; the moved `draft.rs` value tests; `an_as_drawn_choice_submits_the_first_alternative` and siblings | 02 |
| `the_projection_to_submitted_is_the_identity_on_each_kind`; `as_edited_projects_back_to_the_submitted_it_was_given_on_each_kind` | 02 |
| `goad-emit`: `an_answer_that_cannot_be_written_exits_2` | 03 |
| `goad-check` `args.rs` invocation table; `an_unreadable_config_exits_2_and_says_who_spoke`, `a_reserved_source_event_file_exits_2`, `an_empty_argv_is_a_usage_error`, `a_report_that_cannot_be_written_exits_2` (its `--help` half) | 04 |
| every other `goad-check` binary-tier case in §9; `a_report_that_cannot_be_written_exits_2`'s run half; `the_probe_kind_is_none_of_the_host_s_own` | 12 |
| `each_shipped_example_is_accepted_by_the_checker`, `downloads_triage_moves_the_file_it_was_asked_about` | 06 |
| `every_json_and_toml_fence_in_the_kit_is_tagged_and_checked`, `an_untagged_json_fence_is_refused`, `a_jsonc_fence_is_refused`, `a_respond_fence_with_a_value_of_the_wrong_json_type_is_refused`, `nothing_in_the_kit_names_a_path_outside_it`, `a_path_outside_the_kit_is_refused`; `round_trip.rs`' README case through the shared scanner | 07 |
| `every_requirement_a_refusal_can_name_is_explained_in_the_reference`, and its R-32/R-3 negative control | 08 |

`design.md` §9's mutation checks: `NestedHints` flipped to R-3 — PHASE-01/VA-2;
an example exits 1 on the probe — PHASE-06/VA-2; one fence untagged —
PHASE-07/VA-2. The plan adds: a `ScheduleError` arm flipped outside its
fixtures' lists — PHASE-01/VA-4; the R-56 condition dropped —
PHASE-12/VA-5; an example made silent — PHASE-06/VA-5. Its *outside the
gate* checks: `claude plugin validate` — PHASE-05/EX-3, PHASE-08/EX-4,
PHASE-10/EX-2; `nix build` and `goad-walk`'s tool set — PHASE-05/EX-1, EX-4;
the negative control — PHASE-11/EX-1 and before every walk. Its *observed by
a person* runs: PHASE-03/VH-1 (`just demo`), PHASE-06/VH-1..VH-3,
PHASE-09/VH-1, PHASE-10/VH-1; the capsule session — PHASE-11/VH-1.

---

## PHASE-01 — refusals name a requirement and a side

**Objective:** every refusal kind the host can report answers, by a total
match, the requirement and the side SPEC-001/R-59 (`canon-delta.md` SPEC-001
Change 1) assigns it, each prints itself, and the fixture corpus witnesses the
protocol and schedule answers.

**Surfaces:** `crates/goad-semantics/src/error.rs`;
`crates/goad-semantics/tests/protocol/normalize.rs` (and `runner.rs`, only if
the discard witness needs the schedule corpus's envelope);
`tests/fixtures/protocol-text/R-17-a-nan-literal-for-a-bound.json`,
`tests/fixtures/protocol-text/R-17-an-infinite-literal-for-a-bound.json`,
`tests/fixtures/protocol/R-52-a-choice-field-with-no-alternatives.json` (their
`requirement` arrays only); `crates/goad-shell/src/error.rs`;
`canon-delta.md` (test names only).

**Entry**
- EN-1 — `design.md` and `canon-delta.md` approved (`design-log.md`
  2026-09-30, *the design as a whole, after review*); `just check` green at
  HEAD.

**Exit**
- EX-1 — `goad_semantics::error` holds `Requirement` (displays as `R-N`) and
  `AtFault { Backend, Host, Configuration, Environment }`, whose `Display` is
  a total match printing `backend`, `host`, `configuration`, `environment`,
  as `design.md` §5.2.3 gives them. `Requirement`'s field is private, it has
  no constructor, and it has exactly the associated constants §5.2.3 lists:
  `R3`, `R10`, `R12`, `R13`, `R14`, `R16`, `R17`, `R18`, `R21`, `R22`,
  `R23`, `R25`, `R32`, `R40`, `R41`, `R43`, `R44`, `R45`, `R48`, `R50`,
  `R52`, `R53` — one per id the table answers — and `R56`, for the checker's
  claim (`design-log.md` 2026-10-01, *`Requirement` is built from named
  constants*). Every `requirement()` arm, in either stratum, names one.
- EX-2 — `ProtocolError`, `BoundsError` and `ScheduleError` in stratum 1, and
  `BackendError`, `CleanupFailure` and `StateError` in stratum 2, each have
  `requirement()` and `fault()`: total matches with no `_` arm, answering
  exactly §5.2.3's table — `InapplicableKey` split on `key`, `Bounds`,
  `Schedule` and `Protocol` delegating.
- EX-3 — `ConfigError`, `EnvelopeFault` and `SpanFault` have neither method.
- EX-4 — the fixture lists §5.2.3 corrects read as it corrects them — the
  R-17 `Json` fixtures `R-17-a-nan-literal-for-a-bound` and
  `R-17-an-infinite-literal-for-a-bound` [R-17, R-44], and
  `R-52-a-choice-field-with-no-alternatives` [R-52, R-53, R-16] — and no
  other fixture list is edited.
- EX-5 — the refusal witness's red on exactly those fixtures is recorded in
  the phase sheet, before the list corrections.

**Verification**
- VT-1 — `goad-semantics` `error.rs` tests:
  `every_protocol_error_names_a_requirement_and_a_side` and its bounds and
  schedule siblings, beside `must_name`, each an exhaustive table of §5.2.3's
  rows, each expected id spelled as the table spells it (`"R-44"`) and
  compared with `Requirement`'s `Display`, so a constant whose value
  disagrees with its name reds it; `every_side_displays_as_the_word_a_report_prints`,
  a table over each `AtFault` variant beside an exhaustive match.
- VT-2 — `goad-shell` `error.rs` tests:
  `every_backend_error_names_a_requirement_and_a_side` and its cleanup and
  state siblings, expected ids spelled as VT-1's.
- VT-3 — `normalize.rs`:
  `every_refusal_fixture_names_a_requirement_in_its_own_list` over the
  `protocol` and `protocol-text` corpora, and
  `every_discard_fixture_names_a_requirement_in_its_own_list` over the
  `Discarded` fixtures and the schedule corpus. Each witness refuses an empty
  set: it asserts it read at least one fixture from each corpus it covers,
  counted per corpus, and for the discard witness the `Discarded` fixtures and
  the schedule error fixtures separately. The schedule corpus's runner
  (`SCHEDULE`, `check_schedule` in `runner.rs`) is private, and
  `assert_corpus`'s vacuity guard holds only for a `Corpus` run, so this guard
  is the witness's own.
- VA-1 — the §5.2.3 table and the code agree row for row, read side by side
  and recorded in the phase sheet; this is the review the witness's stated
  reach leaves (F-23). The same read holds `Requirement`'s constants to
  §5.2.3's list: none missing, none surplus — nothing else holds a surplus
  constant (§5.2.3).
- VA-2 — mutation: `NestedHints`' `requirement()` arm R-18 → R-3 reds
  `every_refusal_fixture_names_a_requirement_in_its_own_list`. Recorded.
- VA-3 — `canon-delta.md` SPEC-001 Change 2's test names resolve to the
  shipped cases.
- VA-4 — mutation: `ScheduleError::NotAString`'s `requirement()` arm R-25 →
  R-3 reds `every_discard_fixture_names_a_requirement_in_its_own_list`, whose
  failure names both `protocol/R-25-next-check-of-the-wrong-type` (a
  `Discarded` fixture, [R-25, R-51]) and `schedule/R-25-not-a-string`
  ([R-21, R-25]) — one fixture from each half. Recorded.

**Notes for the implementer**
- Red first, in this order (`plan-log.md` 2026-10-01, *PHASE-01's red-first
  order*). The witnesses call `requirement()` and compare its printed form,
  so they do not compile without it, and a compile failure is not EX-5's
  red.
  1. `Requirement` with its constants (EX-1), its `Display`, and stratum 1's
     `requirement()` on `ProtocolError`, `BoundsError` and `ScheduleError`,
     red by VT-1's tables first — their requirement column; `fault()` does
     not exist yet.
  2. The witnesses (VT-3), red on exactly EX-4's fixtures (EX-5).
  3. The list corrections (EX-4); the refusal witness goes green.
  4. `fault()`, stratum 2's `requirement()` and `fault()` (VT-2), and
     `AtFault`'s `Display`; VT-1's tables gain their side column.

  The discard witness has no list to correct, so it is green from its first
  run; VA-4 is what shows it can fail. `every_protocol_error` in `error.rs`'
  tests already builds one of each variant; extend it, do not write a second
  builder.
- VA-2's and VA-4's mutations flip an arm to `Requirement::R3`, a constant
  that exists for `UnsupportedProtocolVersion`, so the mutated build
  compiles.
- The `requirement` array is the fixture's claim, not the code's: the
  corrections EX-4 names are the only lists edited to agree with the code
  (`design.md` §5.2.3, *Its reach*).
- STOP if any other fixture's produced error falls outside its own list: that
  is a §5.2.3 row the design got wrong, not a list to edit.

---

## PHASE-02 — the host's kinds and R-57 values live in stratum 1

**Objective:** `Stimulus`, `Submitted`, `Finite` and the as-drawn value per
kind each have one encoding, in `goad_semantics::protocol::canonical`, and
`goad` names no kind string and decides no submitted value's JSON type.

**Surfaces:** `crates/goad-semantics/src/protocol/canonical.rs`;
`crates/goad/src/{wire.rs, draft.rs, view_model.rs}`;
`crates/goad/src/{controller.rs, install.rs, main.rs, glass.rs}` (imports and
the delegation only); `crates/goad/tests/` (imports of `Stimulus`);
`crates/goad/Cargo.toml` (its dependency comment only);
`crates/goad-shell/src/ingress/envelope.rs` (`HOST_SOURCE`, `plan-log.md`
PL-2); `canon-delta.md` (test names only). `NumberRange::drawn` and its test
land in `canonical.rs`, and `drawn_number` leaves `view_model.rs`, both
listed. `goad_semantics::error` (`AtFault::Host`'s word) is read for EX-6,
not edited.

**Entry**
- EN-1 — PHASE-01 done (PHASE-01..PHASE-03 run in sequence, §Sequencing).
- EN-2 — `design.md` §5.2.4 says how `view_model::as_drawn` delegates to
  `Submitted::as_drawn`: a `FieldKind` rebuilt from the `DrawnKind`, and a
  private `as_edited` in `view_model.rs` back (`design-log.md` 2026-10-01,
  *plan review round 1: design-touching dispositions*, superseding G1's
  placement).

**Exit**
- EX-1 — `Stimulus` is in `canonical.rs` beside `Event`, `kind` and `event`
  unchanged; `crates/goad/src` holds no `"startup"`, `"requested"` or
  `"scheduled"` literal.
- EX-2 — `Submitted` (one variant per `FieldKind`), `Submitted::to_json` (the
  single R-57 site) and `Finite` (with its doc) are in `canonical.rs`;
  `draft::submitted` is `edited.submitted().to_json()`, and
  `Edited::submitted` decides no type.
- EX-3 — `Submitted::as_drawn` is the one statement of the untouched-value
  policy; `view_model::as_drawn` delegates to it by EN-2's route, through a
  private `as_edited` beside `adjusted` that spells a number through
  `adjusted`, so `adjusted`'s doc stays true; `draft.rs` still imports nothing
  from `view_model`; and `view_model::untouched` still shows what `as_drawn`
  submits (P-3). `NumberRange::drawn` is the one statement of the drawn
  number — its minimum, or zero — called by `Submitted::as_drawn`'s number arm
  and by `view_model::interpret`'s untouched fallback; `view_model::drawn_number`
  is gone, and its doc (the `max: -10` consequence, the pointer to slice 007's CD-1) is on
  `NumberRange::drawn` (`design.md` §5.2.4; `design-log.md` 2026-10-01, *the
  drawn number has one home: `NumberRange::drawn`*). `grep -rn 'drawn_number'
  crates` finds nothing; `grep -rn 'min()' crates/*/src`, each hit read and
  recorded, finds no second minimum-or-zero rule — outside tests and
  `NumberRange`'s own `impl`, the only reader of the minimum is
  `view_model::slider_bounds`, which defaults nothing.
- EX-6 — `HOST_SOURCE` is a `pub const` beside `Stimulus`; `Stimulus::event`
  and `envelope.rs`' `ReservedSource` check both name it. Outside comments and
  `#[cfg(test)]` modules, exactly two `"host"` string literals remain in
  `crates/*/src`: `HOST_SOURCE`'s own definition, and `AtFault::Host`'s
  printed word in `AtFault`'s `Display` — a side, not the reserved source, so
  it is not routed through `HOST_SOURCE`. `grep -rn '"host"' crates/*/src`,
  each hit read and recorded as a comment, a test, `HOST_SOURCE`'s definition
  or `AtFault::Host`'s word, and nothing else (PL-2; `plan-log.md` 2026-10-01,
  *PHASE-02 sheet questions*, 1). Tests keep the literal: they witness the
  wire spelling, and are not a second encoding of it.
- EX-7 — `DrawnKind::Choice` has no `first`; its stale doc goes with it
  (PL-3).
- EX-4 — `goad`'s renderer tier is green unchanged: `fields.rs`'s R-57/R-58
  cases. `view_model.rs`' `as_drawn` and `untouched` tests are unchanged save
  where EX-7 reaches them (R5): the helper `a_choice` drops `first`, and
  `as_drawn_answers_every_kind`'s choice expectation becomes the fixture's
  literal alternative id, `"first"` — not `alternatives.first()`, which is the
  expression the delegated code computes, so the case would check the code
  against itself.
- EX-5 — the doc citations that move are by symbol (`Stimulus::event`'s doc
  cites `canonical.rs` by line today); `crates/goad/Cargo.toml`'s comment no
  longer names `Stimulus::event` among `goad`'s reasons for `serde_json`.

**Verification**
- VT-1 — `canonical.rs` tests: `a_scheduled_stimulus_names_itself_scheduled`
  and `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`,
  moved verbatim.
- VT-2 — `canonical.rs`: `draft.rs`' value tests moved against `Submitted`
  (`a_boolean_field_submits_a_json_boolean`,
  `a_finite_refuses_every_number_json_cannot_carry`,
  `a_picked_datetime_submits_the_offset_it_was_picked_in`), and
  `each_kind_submits_the_json_type_r_57_names` moved as
  `every_submitted_kind_writes_the_json_type_r57_names`, gaining the
  `boolean` clause so it covers every `Submitted` variant — one case, not a
  second asserting the same types (`plan-log.md` 2026-10-01, *PHASE-02 sheet
  questions*, 3); `an_untouched_number_is_drawn_at_its_minimum_or_zero`, over
  `NumberRange::drawn` (a declared minimum, no bounds, and `max: -10` with no
  `min`); `an_as_drawn_choice_submits_the_first_alternative` and a sibling per
  kind, including the `number` case (the minimum, or zero, through
  `NumberRange::drawn`) and the `datetime` epoch case.
- VT-3 — `draft.rs`: `the_projection_to_submitted_is_the_identity_on_each_kind`.
- VT-4 — `view_model.rs`:
  `as_edited_projects_back_to_the_submitted_it_was_given_on_each_kind`, the
  round trip `Submitted` → `Edited` → `Submitted` over each kind.
- VA-1 — `cargo test -p goad-semantics` (the gate's stratum-1 command) builds
  the moved code with stratum 1's own features; no feature was added to a
  dependency shared with stratum 1 (POL-001's residue).
- VA-2 — reach: if the moves create a new file under
  `crates/goad-semantics/src`, a planted `std::fs` call in it, in code that
  compiles, reds the purity scan; restored. If they land in `canonical.rs`,
  record that no new file was created.
- VA-3 — `canon-delta.md` SPEC-001 Changes 3–4's test names resolve.

**Notes for the implementer**
- `clippy::pub_use` is denied: `goad` imports from `goad_semantics`, no
  re-export (`design.md` §5.2.4).
- `DrawnKind::Choice.first` goes in the refactor step, after the delegation
  is green (PL-3).
- `DrawnKind` is deliberately not `FieldKind` (its own doc, D10). Do not make
  it one.

---

## PHASE-03 — the ground the checker stands on

**Objective:** the exercisers are renamed and no longer read as the file to
copy, `goad-emit` exits 2 when its answer cannot be written, and
`config::Command::from_argv` is public.

**Surfaces:** `examples/` → `exercisers/` (`git mv`); every site in
`design.md` §5.2.7's rename table except POL-001 (audit) and README's kit line
(PHASE-05): `exercisers/demo.toml`, `exercisers/typescript/README.md`,
`justfile` (`typecheck`, `demo`), `crates/goad-shell/tests/integration/{harness.rs,
round_trip.rs}`, `README.md` (the `just demo` paragraph), `.gitignore` and
`flake.nix` (comments), `docs/roadmap.md`, the `docs/memory/` files the
table names; `crates/goad-emit/src/main.rs`,
`crates/goad-emit/tests/binary/exchange.rs`;
`crates/goad-shell/src/config.rs` (`from_argv`'s and `unsigned`'s
visibility, and the `Command` doc, PL-1); `version_line`'s homes
(`crates/goad-shell/src/version.rs`, new, and `crates/goad-shell/src/lib.rs`,
its `mod` line only; `crates/goad/src/diagnostics.rs`,
`crates/goad-emit/src/render.rs` and their callers; PL-7, its placement
amended by `plan-log.md` 2026-10-01, *PHASE-02's `glass.rs` comment;
PHASE-03 sheet questions; the push before a lock bump*);
`crates/goad/src/startup.rs` and
`crates/goad/tests/binary/main.rs`, doc-only, for `version_line`'s old home
(the same entry); `canon-delta.md` (test names only).

**Entry**
- EN-1 — PHASE-02 done (PHASE-01..PHASE-03 run in sequence, §Sequencing).

**Exit**
- EX-1 — `exercisers/` holds `shell/backend.sh`, `typescript/{backend.ts,
  README.md}` and `demo.toml`; each header says it is a host exerciser and
  points at `kit/`; every present-tense claim in the exercisers and their
  tests that they are the thing to copy is rewritten — `backend.ts`' "Copy
  this file", `round_trip.rs`' "the file a person copies" and its
  `the_readme_s_own_config_loads_and_runs_the_example` doc, `backend.sh`'s
  header, and the TypeScript README's opening among them — and `git grep -n
  -i 'cop\(y\|ies\)' -- exercisers
  crates/goad-shell/tests/integration/round_trip.rs README.md` is recorded,
  each remaining hit classed (`plan-log.md` 2026-10-01, *PHASE-02's
  `glass.rs` comment; PHASE-03 sheet questions; the push before a lock
  bump*).
- EX-2 — `git grep -n 'examples/' -- ':!docs/slices' ':!docs/brief.md'` finds
  only POL-001's command block, which audit amends.
- EX-3 — the `justfile`'s `typecheck` is `deno check
  exercisers/typescript/backend.ts`: `canon-delta.md` POL-001 Change 1's line
  less the kit path, which PHASE-06 adds. Its comment is true of this step:
  it names the one exerciser it typechecks and why (`deno run` does not), and
  says the recipe departs from POL-001's command block until audit promotes
  POL-001 Change 1. That departs from POL-001 §Statement's "a change goes into
  this policy first and into the recipe second", because canon is not edited
  mid-slice and the rename would otherwise break the gate (`plan-log.md`
  2026-10-01, *plan review round 1: dispositions*, superseding PL-4's
  rationale).
- EX-4 — `goad-emit`'s `--help` and `--version` write through
  `report::try_line_to`; an unwritten answer exits 2 with a `goad-emit: …` line
  on stderr.
- EX-5 — `config::Command::from_argv` is `pub`, with its doc naming the
  checker's argv form as its second caller.
- EX-6 — `config.rs`' `unsigned` is `pub` as `config::positive_duration`,
  its doc naming `goad-check`'s `--timeout` as its second caller
  (`design-log.md` 2026-10-01, G2).
- EX-7 — `config::Command`'s doc names the routes that hold the empty command
  out (`Config::parse`, `from_argv`) and no longer claims it is
  unrepresentable (PL-1).
- EX-8 — `version_line(version, revision)` is defined once, in a new module
  `goad_shell::version`, taking the package version as a parameter:
  `env!("CARGO_PKG_VERSION")` expands in the crate that compiles it, so each
  binary passes its own. `goad` and `goad-emit` call it, and their
  `--version` output is unchanged. `goad_shell::report` is unchanged, its
  "not a formatter" module doc still true (PL-7, its placement amended by
  `plan-log.md` 2026-10-01, *PHASE-02's `glass.rs` comment; PHASE-03
  sheet questions; the push before a lock bump*).

**Verification**
- VT-1 — `goad-emit` binary tier: `an_answer_that_cannot_be_written_exits_2`
  (`--help` and `--version`, stdout on `/dev/full`; status 2; the last stderr
  line begins `goad-emit: `), modelled on `goad`'s `exit_codes.rs` case of the
  same name; seen red before `main.rs` changes. If it ships outside
  `exchange.rs`, `canon-delta.md` SPEC-004 Change 5's R-8..R-10 row is
  re-pointed in the same commit.
- VT-2 — `round_trip.rs`'s `the_readme_s_own_config_loads_and_runs_the_example`
  and `harness.rs`' deno cases are green on the new paths.
- VT-3 — `config.rs` unit tests: `positive_duration` refuses `0s` and `-1s`
  under the key it is given.
- VT-4 — `goad`'s and `goad-emit`'s existing `--version` cases stay green
  across the lift.
- VA-1 — `just -n check` prints POL-001 §Compliance's command block with its
  `deno check` line replaced by EX-3's, and no other difference; recorded.
- VA-2 — the README's counts touched here are replaced by names ("the
  ten-line shell backend").

**Verification (human)**
- VH-1 — a person runs `just demo` on the renamed exerciser and sees the
  window prompt, as before.

**Notes for the implementer**
- `docs/brief.md` and closed slices' docs are not edited (§5.2.7).
- The TypeScript exerciser stays; `harness.rs` and `round_trip.rs` drive it.

---

## PHASE-04 — `goad-check`: the crate and its edges

**Objective:** a headless `goad-check` binary parses both command forms,
loads its configuration and event files by the host's rules, writes its
report through the report writer, and exits 2 on every failure as
`canon-delta.md` SPEC-004 R-11..R-15 state, with every way to status 2 in its
binary tier. It makes no
exchange yet, so it judges nothing and delivers no verdict: until PHASE-12 a
run ends with status 2. The exchange, the verdict and statuses 0 and 1 are
PHASE-12's.

**Surfaces:** `crates/goad-check/` (new: `Cargo.toml`, `src/`,
`tests/binary/` and its bash fixtures); the root `Cargo.toml`'s `members`
(appended after `goad-emit`, before `goad-boundary`) and `Cargo.lock`;
`crates/goad-boundary/tests/checks/allowlist.rs` (its module doc's member
list only); `canon-delta.md` (test paths only). `tests/support/` is read and
may be included, not edited.

**Entry**
- EN-1 — PHASE-01, PHASE-02 and PHASE-03 done.
- EN-2 — `design.md` §5.2.1 judges `--timeout` by
  `config::positive_duration` (`design-log.md` 2026-10-01, G2), public since
  PHASE-03/EX-6.

**Exit**
- EX-1 — the command line of `design.md` §5.2.1: config form (`--config`,
  else `config::default_path`), argv form after `--` through
  `Command::from_argv`, `--timeout` (default `5s`, a usage error with
  `--config`), repeatable `--event FILE` through `envelope::normalize`, `-h`,
  `--help`, `--version`. `args.rs` is pure and returns one `Invocation`.
- EX-3 — the report writer: a `render`-style module owns every line's text,
  and every stdout line goes through `report::try_line_to`. The no-view line
  is written here. Until PHASE-12 a run makes no exchange, so it ends with no
  verdict: its report is the no-view line, a stderr line says the run is not
  yet implemented, and it exits 2. Nothing on `main` reports an acceptance it
  did not judge. PHASE-12/EX-6 replaces this end with the verdict.
- EX-4 — status 2 as §5.2.5: every cause of 2, the interim end of EX-3
  included, reaches one `ExitCode::from(2)` that reads no cause; `main`
  returns an `ExitCode` built from literals, one per class it can reach; the
  last stderr line on 2 begins `goad-check: `. The verdict line and the cut
  to 0 or 1 are PHASE-12's (EX-6): written here, nothing would feed them, and
  the gate's lint refuses dead code.
- EX-5 — `crates/goad-check/Cargo.toml` names only strata 1 and 2 and
  workspace dependencies already in the lockfile, with a comment arguing it
  links no renderer, as `goad-emit`'s does (I-6).
- EX-6 — `allowlist.rs`' module doc names `goad-check` among the stratum-3
  members, by name and without a count.
- EX-7 — the steps before the first exchange, each ending the run with no
  verdict, status 2, on failure: the configuration loaded, each `--event`
  file normalized in the order given, the clock read, a current-thread
  runtime and a `Host` built the way `goad`'s `main.rs` `start` builds them.
- EX-8 — `--version` prints `goad_shell::version::version_line` with this
  crate's package version and its compilation's `GOAD_REVISION`, as
  `goad-emit`'s does (PHASE-03/EX-8).

**Verification**
- VT-1 — `args.rs` unit tests: the invocation table (config form, argv form,
  `--event` order, `--timeout` with `--config` refused, `--timeout 0s` and
  `-1s` refused, an empty argv and an empty program refused, help, version).
- VT-2 — binary tier, `tests/binary/`, the status-2 cases §9 names:
  `an_unreadable_config_exits_2_and_says_who_spoke`,
  `a_reserved_source_event_file_exits_2`, `an_empty_argv_is_a_usage_error`,
  and `a_report_that_cannot_be_written_exits_2` with its `--help` half
  (PHASE-12/VT-2 adds the run half). Each asserts status 2 and the
  `goad-check: ` prefix on the **last** stderr line.
- VT-3 — binary tier, the plan's own interim case:
  `a_run_with_no_exchange_exits_2_with_no_verdict` — a loadable
  configuration; status 2, stdout the no-view line and no verdict line, the
  last stderr line beginning `goad-check: `. PHASE-12/EX-6 deletes it.
- VA-1 — I-1 by the command under *Invariant reads*: no hit outside a comment
  at this phase, in any of the spellings that command matches; the R-56
  claim and its `Requirement::R56` are PHASE-12's. Recorded.
- VA-2 — I-2 by the command under *Invariant reads*; each match found is
  read and recorded.
- VA-3 — reach: `goad-boundary`'s
  `no_workspace_member_names_the_users_domain` reads `crates/goad-check/src`
  — a planted domain word in a string literal, in code that compiles, reds it;
  restored. Clippy reaches the crate's `src` and `tests` — a planted
  `.unwrap()` in each reds `cargo clippy --workspace --all-targets -- -D
  warnings`; restored. Both recorded.
- VA-4 — `canon-delta.md` SPEC-004 Change 5's `goad-check` test paths, for
  the cases this phase ships, are re-pointed from `…` to the shipped files.
- VA-5 — `notes.md` §Open's FU-7 row names `crates/goad-check/Cargo.toml`'s
  comment as what holds I-6.
- VA-6 — R-13 and R-15, structurally: `grep -n 'ExitCode' crates/goad-check/src`
  shows the one `ExitCode::from(2)`, reached by every status-2 path without
  reading its cause, and one literal per class; read and recorded.
- VA-7 — shared helpers: the binary tier reads each `tests/support/` file's
  whole exported surface, and includes each file whose every symbol it uses
  (`design.md` §5.2.6). Each helper it copies instead is named by symbol in
  `notes.md` §Open's FU-5 row, with the file it could not include.

**Notes for the implementer**
- Build the runtime, backend and `Host` the way `goad`'s `main.rs` `start` does; read that site first.
- Mirror `goad-emit`'s shape: pure `args.rs`, a `render`-style module that
  owns every line's text, `main` alone reading the environment, files and the
  clock. `autotests = false` with `[[test]]` targets, as every member does.
- Test backends are small `bash` scripts under `tests/binary/`. Nothing
  asserts a duration (R3). Read `tests/support/scripting.rs`' `scripted` and
  `tests/backends/answers-as-instructed.sh` before writing one: a backend
  told what to do per invocation may serve PHASE-12's cases too.
- A new external dependency (a temp-dir crate, an argument parser) is a STOP.
- `Failure::State` cannot be provoked by a cooperating test; the path to 2 is
  structural (SPEC-004 Change 5's R-11..R-13 row).

---

## PHASE-12 — `goad-check`: the run

**Objective:** `goad-check` sends the request plan, answers every view and
follows chains, reports every channel of every `Outcome` with the side and
requirement its kind answers, and charges R-56 only on its condition
(`design.md` §5.2.2, §5.4).

**Surfaces:** `crates/goad-check/src/`; `crates/goad-check/tests/binary/`
and its bash fixtures; `crates/goad-check/Cargo.toml` (`[dev-dependencies]`
only, should reading a test backend's request log need one; a new external
dependency is still a STOP); `canon-delta.md` (test paths only). `tests/support/`
is read and may be included, not edited.

**Entry**
- EN-1 — PHASE-04 done.

**Exit**
- EX-1 — the request plan of §5.2.2 in order: `Stimulus::Startup`,
  `Requested`, `Scheduled`, the R-56 probe, then each `--event` envelope in
  the order given. The probe kind is a `goad-check` constant, and its event's
  source is `HOST_SOURCE`, not a literal. `event.timestamp` and `now` are the
  wall clock at each step. Each view is answered through `Host::respond` with
  the minted `view_id`, its first option, and `Submitted::as_drawn` for each
  of that option's fields; chains are followed to `view: null` or a failure,
  up to the bound of 8 per request.
- EX-2 — every planned exchange is made whatever an earlier exchange did
  (§5.4); only a clock unreadable mid-run, a report line stdout refuses, or
  `Failure::State` ends the run early, with no verdict, status 2.
- EX-3 — the report: every channel of every `Outcome` as §5.2.2's table
  gives it; each refusal line prints its side and `SPEC-001/R-N` through
  `AtFault`'s and `Requirement`'s `Display`; the R-56 line only on its
  condition; stderr verbatim with truncation flagged; the values sent; the
  chain-bound observation. The R-56 line's id is `Requirement::R56`, from
  stratum 1, printed through `Requirement`'s `Display` as every refusal
  line's is; its text, "a backend MUST tolerate a kind it does not
  recognise", is the checker's (`design.md` §5.2.2; `design-log.md`
  2026-10-01, *`Requirement` is built from named constants*).
- EX-4 — the run feeds the status cut (EX-6): at least one refusal on any
  channel, a cleanup failure alone included, is 1; none is 0; the chain bound
  changes no status.
- EX-5 — the checker opens no socket and alters none of the environment it
  passes on (§5.2.1, §5.3): it names no `ingress` item but
  `envelope::normalize`, and sets, removes or clears no environment variable.
- EX-6 — the verdict replaces PHASE-04/EX-3's interim end: the verdict line,
  and statuses as §5.2.5 — a delivered verdict is 0 with no refusal and 1
  with at least one, cut by one function; `main` gains a literal for each, and
  the last stderr line on 1 begins `goad-check: `. The not-yet-implemented
  stderr line and its path to 2 are gone, and PHASE-04/VT-3's case is deleted
  in the commit that turns `a_conforming_backend_is_accepted_and_exits_0`
  green.

**Verification**
- VT-1 — binary tier, the run cases §9 names:
  `a_conforming_backend_is_accepted_and_exits_0`,
  `a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56`,
  `a_refused_view_is_reported_with_its_requirement_and_the_backend_side`,
  `a_discarded_next_check_is_reported_and_exits_1`,
  `an_unspawnable_command_is_reported_against_the_configuration`,
  `a_backend_failing_identically_on_every_kind_is_not_charged_with_r56`,
  `a_backend_that_returns_no_view_is_accepted_and_says_respond_was_not_exercised`,
  `a_chained_view_is_answered_until_null`,
  `a_chain_past_its_bound_is_reported_and_does_not_change_the_status`,
  `a_view_answered_carries_exactly_its_options_fields`. The R-56 case
  asserts the line's `SPEC-001/R-56`, which holds `Requirement::R56`'s value
  (`design.md` §5.2.3). And the plan's own:
  `a_backend_failing_at_startup_is_still_asked_the_rest` (a backend that
  logs each request's kind and fails the first: the log holds every planned
  kind), and `event_files_are_sent_in_the_order_given` (two `--event` files,
  the backend's log holds their kinds in the order given). The non-zero cases
  SPEC-004/R-14's row names assert the `goad-check: ` prefix on the **last**
  stderr line.
- VT-2 — `a_report_that_cannot_be_written_exits_2` gains its run half: a run
  with stdout on `/dev/full` exits 2, the last stderr line beginning
  `goad-check: `.
- VT-3 — `the_probe_kind_is_none_of_the_host_s_own`, a unit test beside the
  probe constant (a binary-only crate's constant is not reachable from
  `tests/binary/`), asserting it is none of `Stimulus`'s kinds.
- VA-1 — I-1 over the finished crate, by the command under *Invariant reads*:
  the only hits outside a comment are the R-56 probe's own — the claim's
  `Requirement::R56` (EX-3), and the two `AtFault::Backend` uses R-56 needs:
  the condition's comparison of a probe failure's `fault()` (§5.2.2) and the
  probe's claim. Any other spelling of an id, `R-56` and `R56` included, is a
  hit outside that set. Recorded. This read is also what holds that the
  claim PHASE-08/VT-1 reads, `Requirement::R56`, is the one the report
  prints.
- VA-2 — I-2 over the finished crate, `src` and `tests`. Recorded.
- VA-3 — EX-5: `grep -rnE 'ingress::|set_var|remove_var|env_clear|env_remove|\.env\('
  crates/goad-check/src` finds only the `envelope` import and its call.
  Recorded.
- VA-4 — PHASE-04/VA-6's structural read, over `main` as this phase leaves
  it.
- VA-5 — mutation: the R-56 condition's "at least one of the known-kind evaluates
  made no failure" dropped, so any backend-side failure on the
  probe is charged, reds
  `a_backend_failing_identically_on_every_kind_is_not_charged_with_r56`.
  Recorded.
- VA-6 — `canon-delta.md` SPEC-001 Change 3's and SPEC-004 Change 5's
  `goad-check` test paths, for the cases this phase ships, are re-pointed
  from `…` to the shipped files.
- VA-7 — as PHASE-04/VA-7, for any helper this phase adds.

**Notes for the implementer**
- The probe names `HOST_SOURCE` (PHASE-02/EX-6); I-1's command greps for a
  `"host"` literal.
- Test backends as PHASE-04's notes say. Nothing asserts a duration (R3).
- A new external dependency is a STOP.

---

## PHASE-05 — packages and the plugin's shell

**Objective:** the flake exports `goad-check` and `goad-kit`; the plugin's
manifests exist and validate; both agents, on the host in a fresh home, load
the plugin from its store path and read the skill's body; and `goad-walk`
builds the full tool set.

**Surfaces:** `flake.nix` (`packages.goad-check`, `packages.goad-kit`);
`.claude-plugin/marketplace.json`; `.agents/plugins/marketplace.json`;
`kit/.claude-plugin/plugin.json`; `kit/.codex-plugin/plugin.json`;
`kit/skills/goad-backend/SKILL.md` (a placeholder: frontmatter, and a body
line stating a marker that neither the frontmatter nor any other file under
`kit/` states); `README.md` (the kit and plugin-install line); `justfile`
(`package` gains `goad-check` and `goad-kit`, `install` gains `goad-check`,
PL-6); `~/dev/goad-walk/{flake.nix, flake.lock, README.md}`, committed in
that repository.

**Entry**
- EN-1 — PHASE-12 done.
- EN-2 — Claude and Codex credentials are available as API keys in
  environment variables to a session started in a fresh home. Nothing
  credential-bearing is written into the home, a committed file or
  `goad-walk`.
- [ ] EN-3 — goad's `main` pushed to `origin` at or past the revision the lock
  will pin (`plan-log.md` 2026-10-01, *PHASE-02's `glass.rs` comment;
  PHASE-03 sheet questions; the push before a lock bump*). The revision is
  this phase's own (EX-4), so the item is checked before the bump, not at
  entry.

**Exit**
- EX-1 — `nix build --no-link .#goad-check .#goad-kit` succeeds from the bare
  git form (new files `git add`ed first). The built `goad-check --version`
  prints the flake's revision beside the version, as `goad-emit`'s does
  (`GOAD_REVISION`). `goad-kit` holds `.claude-plugin/marketplace.json`,
  `.agents/plugins/marketplace.json` and `kit/`, and nothing else.
- EX-2 — `kit/.claude-plugin/plugin.json` and `kit/.codex-plugin/plugin.json`
  carry `workspace.package.version`, checked against `Cargo.toml` and
  recorded.
- EX-3 — `claude plugin validate kit/` passes.
- EX-4 — `goad-walk`'s tool set is `goad-check`, `goad-emit`, `goad`,
  `goad-kit`, `ruby` and `jq`, with `goad-kit` re-exported, its stubs for
  `goad-check` and `goad-kit` gone, its README current, its `flake.lock`
  pinned to this phase's goad commit and committed to its `main`, and it
  builds.
- EX-7 — `just package` builds `goad-check` and `goad-kit`, and `just
  install` installs `goad-check` (PL-6).
- EX-8 — on the host, in a fresh home that has held no session, from an
  empty working directory, `$KIT` being `goad-walk#goad-kit`'s store path:
  first `claude -p` and `codex exec`, with no plugin, each answer a trivial
  prompt, which shows the credentials reach the session; then `claude -p
  --plugin-dir "$KIT/kit"`, and Codex after `codex plugin marketplace add
  "$KIT"; codex plugin add goad@goad`, each answer a headless prompt asking
  for the goad skill's marker with the marker `SKILL.md`'s body states,
  verbatim. The prompt does not contain the marker. (R1, as far as the host
  reaches it; `plan-log.md` 2026-10-01, *the capsule splits out*.)

**Verification**
- VA-1 — Codex's manifest, `interface` block included, is accepted:
  `codex plugin marketplace add` and `codex plugin add goad@goad` against the
  `goad-kit` store path, with `CODEX_HOME` a fresh directory on the host,
  succeed, and `$CODEX_HOME/plugins/cache/` holds only `kit/`'s contents.
- VA-4 — nothing credential-bearing is in `goad-walk`'s tool set.

**Notes for the implementer**
- The spike (`spike/`, `research.md` §"Spike: R1 and R2") is prior art for
  the plugin loads; `goad-kit` is a marketplace root, and Claude loads
  `"$KIT/kit"`. Its stub `SKILL.md` carried a body-only marker for the same
  purpose.
- A skill's name and description reach the model from the frontmatter,
  through the harness's skill listing; only the body shows the skill was
  read. So EX-8 asks for the body's marker, not the skill's name.
- STOP if a bare prompt fails in the fresh home: that is authentication, not
  R1 — fix the credentials (EN-2), and do not read a plugin load's failure
  until the bare prompt answers.
- STOP if either plugin load fails, once the bare prompt answers, in a way
  the kit cannot fix: that is R1 firing, and the walk's shape is the user's.

---

## PHASE-06 — the examples

**Objective:** the kit ships the focus check, Downloads triage and breadcrumbs
examples of `design.md` §5.2.6, each accepted by `goad-check` in the gate
with its own config and event files, having answered a view, and a person
has seen each run.

**Surfaces:** `kit/skills/goad-backend/examples/` (new: `focus-check/`,
`downloads-triage/`, `breadcrumbs/`, as §5.2.6's tree lists them);
`crates/goad-check/tests/kit/` (new target) and `crates/goad-check/Cargo.toml`
(its `[[test]]`); `flake.nix` (`python3` and `jq` in `projectPkgs`);
`justfile` (`typecheck`'s kit path, and its comment); `canon-delta.md` (test
paths only).

**Entry**
- EN-1 — PHASE-05 done. The devshell is reloaded after `flake.nix` changes.

**Exit**
- EX-1 — each example follows §5.2.6's rules: silent unless it has a reason
  to speak; an unrecognised host kind treated as `scheduled`; state under
  `$XDG_STATE_HOME/<example>/`; diagnostics on stderr; standard library only,
  plus `jq` for the shell example; `command` relative to its own directory.
- EX-2 — each README says how to see the example in under a minute from its
  directory, and how to run it from anywhere else.
- EX-3 — the `justfile`'s `typecheck` is exactly `canon-delta.md` POL-001
  Change 1's line. Its comment is true of this step: one exerciser and one
  kit example, both typechecked because `deno run` does not, as §5.2.7 says,
  and the departure from POL-001's block still named until audit.

**Verification**
- VT-1 — `kit` tier: `each_shipped_example_is_accepted_by_the_checker` —
  every directory under `kit/skills/goad-backend/examples/`, enumerated and
  not listed, refusing an empty set; each copied to a temporary directory,
  the checker started there in the config form with the example's event
  files, `HOME` and `XDG_*` temporary. Each run exits 0 **and** its report
  shows at least one view answered: it holds a respond line and not the
  no-view line.
- VT-2 — `downloads_triage_moves_the_file_it_was_asked_about`: the file the
  event names, created under the temporary `XDG_DOWNLOAD_DIR`, is moved to
  the as-drawn target.
- VA-1 — `just -n check` prints POL-001 §Compliance's command block with its
  `deno check` line replaced by EX-3's, and no other difference.
- VA-2 — mutation: one example made to exit 1 on the probe's kind reds VT-1.
  Recorded.
- VA-3 — each example is read against SPEC-001 R-56, R-57, R-58 and R-33 and
  against its README; findings go in the phase sheet. (No review reached the
  examples' correctness.)
- VA-4 — `canon-delta.md` SPEC-001 Change 3's kit-tier path for
  `each_shipped_example_is_accepted_by_the_checker` is re-pointed to the
  shipped file.
- VA-5 — mutation: one example made silent, answering `view: null` to every
  request, reds VT-1 while it still exits 0. Recorded.

**Verification (human)**
- VH-1 — AC-6: a person runs `goad-check --config config.toml` in each
  example's directory, and against a broken backend (one of PHASE-12's
  refusing fixtures), and sees each report — sides, requirements, the verdict
  line and the status.
- VH-2 — a person runs `goad config.toml` in each example's directory and sees
  each example ask its question, from **Check now** or with `goad-emit` and
  the example's event file.
- VH-3 — A-4: a person runs `watch.sh` (with `inotify-tools` from `nix shell
  nixpkgs#inotify-tools`, since the devshell has none) against a running host,
  drops a file into the watched directory, and sees the triage view.

**Notes for the implementer**
- A README's `json` and `toml` blocks follow the tagged-fence convention
  (§5.2.6) now; PHASE-07's test will hold them.
- Example configs carry a generous timeout; no test asserts a duration (R3).
- Each example speaks in the gate at any hour: the focus check on
  `requested`, Downloads triage and breadcrumbs on their event files. VT-1's view assertion
  depends on that; an example that speaks only at some hours needs a request
  that reaches it.

---

## PHASE-07 — the fence gate, and the protocol and transport reference

**Objective:** every `json` and `toml` fence in the kit is tagged and checked
by the gate through the host's own doors, by a fence scanner the workspace
shares; the kit names no path outside itself; and the reference's
`protocol.md` and `running.md` exist with their examples checked.

**Surfaces:** `crates/goad-check/tests/kit/`; `tests/support/` (new: the
fence scanner's file); `crates/goad-shell/tests/integration/{main.rs,
round_trip.rs}` (the scanner's include, and `fenced_block` removed);
`kit/skills/goad-backend/reference/{protocol.md, running.md}`; the example
READMEs (fence tags only).

**Entry**
- EN-1 — PHASE-06 done.

**Exit**
- EX-1 — the extractor is §5.2.6's: a CommonMark fence scanner for backtick
  and tilde fences, info string split on whitespace; a block whose first info
  word, lowercased, begins `json` or `toml` fails unless that word is exactly
  `json` or `toml` with a known `goad:` role. VT-1 reads every `*.md` under
  `kit/`, enumerated and not listed, refusing an empty set.
- EX-2 — each role in §5.2.6's table is checked as the table says: responses
  through `read_response` (accepted, refused by id, discarded by id); evaluate
  requests by framing; respond requests by framing, R-58's field set and the
  JSON-type oracle over `Submitted::as_drawn`; envelopes through
  `envelope::normalize`; configs through `Config::parse`.
- EX-3 — `protocol.md` and `running.md` state only what an author must do or
  may rely on, organised by task, each rule citing its id as a report prints
  it (`SPEC-001/R-13`), each id in a heading line of its own — the anchor
  PHASE-08's coverage test reads.
- EX-4 — the scanner lives in `tests/support/`, included by `goad-check`'s
  `kit` target and `goad-shell`'s `integration` target, every symbol used by
  both (§5.2.6, *It is shared, not second*); `round_trip.rs`' `fenced_block`
  is gone.
- EX-5 — I-5 holds by §5.2.6's rule, *The kit stands alone*.

**Verification**
- VT-1 — `every_json_and_toml_fence_in_the_kit_is_tagged_and_checked`.
- VT-2 — `an_untagged_json_fence_is_refused`, over an inline string.
- VT-3 — `a_jsonc_fence_is_refused`, over an inline string.
- VT-4 — `a_respond_fence_with_a_value_of_the_wrong_json_type_is_refused`.
- VT-5 — `nothing_in_the_kit_names_a_path_outside_it` (I-5): every file under
  `kit/`, enumerated, refusing an empty set; the tracked paths read at test
  time by `git ls-files` at the repository root, refusing a list with nothing
  under `kit/` or under `crates/`. A missing `git` or `.git` fails the test;
  it never skips.
- VT-6 — `a_path_outside_the_kit_is_refused`, over inline strings against
  the tracked list VT-5 reads: an escaping relative path, and a mention of a
  tracked path outside `kit/` (`crates/goad-shell/src`), each refused; a
  consumer path that names nothing here (`.claude/skills/`) and a backticked
  `` `.claude-plugin/plugin.json` ``, each accepted.
- VT-7 — `round_trip.rs`' `the_readme_s_own_config_loads_and_runs_the_example`
  is green, reading the README's config through the shared scanner.
- VA-1 — the extractor's count of checked fences is non-zero and equals a
  count taken by hand over the kit, recorded.
- VA-2 — mutation: one fence in `protocol.md` untagged reds VT-1. Recorded.
- VA-3 — `protocol.md` and `running.md` read against SPEC-001 §4 and §6:
  nothing contradicts it, and nothing host-internal is taught. Recorded.
- VA-4 — `notes.md` §Open's FU-5 row names the shared scanner's file and
  symbol, and says it replaced `round_trip.rs`' `fenced_block` rather than
  copying it.
- VA-5 — reach: a relative path escaping `kit/`, planted in an example
  README, reds VT-5; restored. Recorded.
- VA-6 — I-2 over `crates/goad-check`'s tests as this phase leaves them (the
  respond oracle), by the command under *Invariant reads*. Recorded.

**Notes for the implementer**
- The respond oracle writes no reader of `Submitted` (U5); I-2 binds tests too.
- Fragments are shown whole or as `text` (§5.2.6).
- Read `docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`
  before adding the shared file: an includer that leaves a symbol unused
  fails the gate at its own build.

---

## PHASE-08 — the rest of the reference, and its coverage

**Objective:** the reference is complete — `scheduling.md`, `events.md`,
`checking.md` and the finished `SKILL.md` — and the gate holds that every
requirement any refusal kind can name, and the R-56 the checker claims, is
explained in it, at an anchor of its own.

**Surfaces:** `kit/skills/goad-backend/reference/{scheduling.md, events.md,
checking.md}` (and `protocol.md`, `running.md` for the statement EX-5 names);
`kit/skills/goad-backend/SKILL.md`; `crates/goad-check/tests/kit/`.

**Entry**
- EN-1 — PHASE-07 done.

**Exit**
- EX-1 — PHASE-07/VT-1 is green over the finished kit, and VA-2 shows it
  read a file this phase wrote.
- EX-2 — `SKILL.md` routes and does not teach: when to use it, the loop, how
  to get the binaries, one pointer per reference file by the question it
  answers, one line per example; no wire example of its own.
- EX-3 — `checking.md` explains the checker's forms and what it sends, the
  four sides in R-59's terms, where else to look for each kind with a declared
  imprecision, that a run mutates real state (and the `XDG_*` advice), that a
  relative `command` resolves against the starting directory, and statuses
  0, 1 and 2.
- EX-4 — `claude plugin validate kit/` passes, and the manifests' versions
  still match.
- EX-5 — the reference says the specs are not shipped, and that every id a
  report prints is explained in the reference itself (§5.2.6, *The
  reference's structure*).
- EX-6 — the marker line PHASE-05 put in the placeholder `SKILL.md` is gone;
  no load check after this phase asks for it (PHASE-11/EX-2).

**Verification**
- VT-1 — `every_requirement_a_refusal_can_name_is_explained_in_the_reference`:
  its id set is one instance per variant (two for `InapplicableKey`), each
  builder beside an exhaustive match with no `_` arm, and the checker's own
  R-56 claim as `Requirement::R56`, read through `goad-semantics`, not
  respelled. `goad-semantics` is already an ordinary dependency of
  `goad-check` (`design.md` §5.1; PHASE-04/EX-5), so its test targets reach the constant with
  no `[dev-dependencies]` entry. That the report's claim names the same
  constant is PHASE-12/VA-1's expected hit. It reads every `*.md` under
  `kit/skills/goad-backend/reference/`, enumerated and not listed, refusing
  an empty set, through the shared scanner (PHASE-07/EX-4), skipping every
  fenced line. An id counts only in a Markdown heading line that names
  exactly that one requirement id and no other (PHASE-07/EX-3); an id
  matches only when followed by a non-digit or the end. Seen red before
  `scheduling.md`, `events.md` and `checking.md` are written. It adds no
  symbol to the scanner: fenced lines are found from what the scanner already
  returns — for instance, each fence's text removed before headings are read.
  If PHASE-07's scanner returns nothing that allows this, STOP: a new symbol
  must be used by both includers (PHASE-07/EX-4), and neither
  `tests/support/` nor `round_trip.rs` is this phase's surface.
- VT-2 — its negative control over an inline string where `SPEC-001/R-32`
  appears in a heading of its own, `SPEC-001/R-3` does not appear,
  `SPEC-001/R-40` appears only in body text, `SPEC-001/R-36` appears only on
  a `#` line inside a fenced block, and `SPEC-001/R-41` and `SPEC-001/R-42`
  appear only together in one heading: R-32 is found, and R-3, R-40, R-36,
  R-41 and R-42 are not.
- VA-1 — every reference file read: `scheduling.md` and `events.md` against
  SPEC-001 R-21..R-29, SPEC-002 and SPEC-003; `checking.md` against
  `design.md` §5.2.5 and `canon-delta.md` SPEC-004; and all of them, with
  `protocol.md` and `running.md`, for nothing host-internal taught — bounds,
  cleanup and renderer subsets appear at most as one line saying what the
  author observes — and for EX-5's statement. Recorded.
- VA-2 — reach: an untagged `json` fence planted in `scheduling.md` reds
  PHASE-07/VT-1; restored. Recorded.
- VA-3 — I-2 over `crates/goad-check`'s tests as this phase leaves them (the
  matches beside the coverage builders), by the command under *Invariant
  reads*. Recorded.

---

## PHASE-09 — the first walks

**Objective:** one Claude Code walk and one Codex walk have run from the fixed
prompt in fresh capsules; each is measured, verdict-checked, run by a person
and read; and every friction item is dispositioned.

**Surfaces:** `docs/slices/012/walks.md` (new);
`docs/slices/012/walks/<agent>-1-ISSUES.md` (new);
`docs/slices/012/walk/` (fixes to the script only);
`~/dev/goad-walk/flake.lock`, committed in that repository. Walk trees and
transcripts stay in the quarantine and `goad-walk`, outside this repository.

**Entry**
- EN-1 — PHASE-08 done, on `main`.
- EN-2 — PHASE-11 done.
- [ ] EN-3 — goad's `main` pushed to `origin` at or past the revision the lock
  will pin (`plan-log.md` 2026-10-01, *PHASE-02's `glass.rs` comment;
  PHASE-03 sheet questions; the push before a lock bump*).

**Exit**
- EX-1 — before each walk, in its capsule, the negative control passed.
- EX-2 — `walks.md` has a row per walk with every column `design.md` §5.2.9
  lists: agent, walk, model; turns; wall time; tokens (input uncached, cache
  read, cache write, output, thinking) — Codex's raw fields as reported until
  the first walk shows how its cache fields nest; cost (Claude); fetch
  attempts and their witness; the pinned goad revision; the verdict; the
  person-run.
- EX-3 — each walk's verdict is `goad-check --config <the agent's config>`,
  run in the guest from `/work/goad-walk` at an hour the backend speaks:
  status 0 with at least one view answered. A walk whose agent read goad's
  source by any route fails and is re-run.
- EX-4 — every friction item — each `ISSUES.md` entry and each tagged
  transcript item — is a `walks.md` row dispositioned **kit fix** or
  **follow-up** with its reason. A fetch attempt beyond the model API is an
  item.
- EX-5 — before either walk, `goad-walk`'s `flake.lock` is bumped to
  PHASE-08's revision on `main` and committed to `goad-walk`'s `main`; each
  walk row's pinned revision is that one.

**Verification**
- VA-1 — the verdict and the negative control are re-read from the collected
  commit, not from an agent's report.
- VA-2 — a fresh agent reads each transcript and tags each friction item
  *retried*, *guessed*, *read outside the kit*, *network fetch* or *checker
  confusion*, citing the event index.

**Verification (human)**
- VH-1 — for each walk, a person runs goad on the host against the collected
  tree, with `ruby` from `nix shell nixpkgs#ruby`, the config's paths made
  absolute (the diff recorded in `walks.md`), and sees the wrap-up ask its
  questions.

---

## PHASE-10 — kit fixes, and the re-walks

**Objective:** every friction item dispositioned *kit fix* is fixed on `main`;
`goad-walk`'s lock moves to that revision; each agent walks once more; the
re-walks pass AC-1 and do not regress.

**Surfaces:** `kit/`; `crates/goad-check/src` (report wording only);
`crates/goad-check/tests/`; `docs/slices/012/{walks.md, walks/,
walk/}`; `~/dev/goad-walk/flake.lock`, committed in that repository.

**Entry**
- EN-1 — PHASE-09 done; its kit-fix rows are the work list.
- [ ] EN-2 — goad's `main` pushed to `origin` at or past the revision the lock
  will pin (`plan-log.md` 2026-10-01, *PHASE-02's `glass.rs` comment;
  PHASE-03 sheet questions; the push before a lock bump*). The revision is
  the fixed one (EX-3), so the item is checked before the bump, not at
  entry.

**Exit**
- EX-1 — each kit-fix row cites the commit that fixed it; no fix touches the
  wire or canon (§5.2.9's *kit fix*). A fix that would is re-dispositioned
  *follow-up* with the user.
- EX-2 — `just check` green; `claude plugin validate kit/` passes; the
  manifests' versions match.
- EX-3 — `goad-walk`'s `flake.lock` pins the fixed revision and is committed
  to its `main` before either re-walk; each re-walk row records that revision.
- EX-4 — each agent's re-walk meets PHASE-09/EX-1, EX-2 and EX-3, from a new
  capsule with the same prompt.
- EX-5 — re-walk friction is dispositioned as in PHASE-09/EX-4. A second
  re-walk happens only by the user's decision.

**Verification**
- VA-1 — PHASE-09/VA-1 and VA-2 for the re-walks.
- VA-2 — every PHASE-09 friction row has its disposition carried out: fixed,
  or recorded in `notes.md` §Open as a follow-up candidate.
- VA-3 — `notes.md` §Open's FU-5 and FU-7 rows name the shipped symbols.
- VA-4 — `notes.md` §Open carries each item of **Owed to audit and close**
  above.
- VA-5 — the standing guards reach what the fixes changed: I-1 and I-2 by
  the commands under *Invariant reads*, if `crates/goad-check` changed; and
  for each `*.md` the fixes add under `kit/`, an untagged `json` fence
  planted in it reds PHASE-07/VT-1, restored. Recorded, or recorded as not
  arising.

**Verification (human)**
- VH-1 — as PHASE-09/VH-1, for each re-walk.
- VH-2 — the person judges, from `walks.md`, that neither re-walk regressed in
  turns or total tokens (AC-8; indicative, no threshold).

---

## PHASE-11 — the capsule

**Objective:** the walk script exists, and a capsule provisioned from
`goad-walk` has passed its negative control, had the plugin loaded and the
skill's body read by both agents, and brought a committed tree back through
`capsule-collect`.

**Surfaces:** `docs/slices/012/walk/` (new: the walk script, the negative
control, the fixed prompt; PL-5).

**Entry**
- EN-1 — PHASE-05 done.
- EN-2 — oubliette can take `goad-walk` as a target, and the user has
  registered it (oubliette's `docs/contract-target.md`). Oubliette-side work,
  including support for more than one target repository, and not this
  slice's.
- EN-3 — Claude and Codex credentials are available as environment variables
  for the ssh command that starts a capsule session.

**Exit**
- EX-1 — in a fresh capsule provisioned at `goad-walk`'s `main`, the negative
  control passes: each source pattern and the session probe finds nothing
  there, and on the host each finds something, each source pattern
  separately; `goad-check --version`, `ls "$KIT/kit/skills"` and `ruby -e
  'require "json"'` succeed.
- EX-2 — in that capsule, first `claude -p` and `codex exec`, with no plugin,
  each answer a trivial prompt; then `claude -p --plugin-dir "$KIT/kit"`, and
  Codex after `codex plugin marketplace add "$KIT"; codex plugin add
  goad@goad`, each answer a headless prompt asking for a fact `SKILL.md`'s
  body states, verbatim, at the goad revision `goad-walk`'s lock pins —
  PHASE-05's until PHASE-09 moves it. At a revision before PHASE-08 the fact
  is PHASE-05's marker. If this phase is entered after PHASE-08 and the
  revision it loads holds the finished `SKILL.md`, whose marker PHASE-08/EX-6
  removed, the fact is one only that finished body states, in neither the
  frontmatter nor the prompt. And the walk script's
  collection step brings a committed tree back through `capsule-collect`.

**Verification**
- VA-1 — the walk script does what `design.md` §5.2.8–§5.2.9 say: `$KIT` from
  `nix eval` on `goad-walk#goad-kit`, the control before each walk, the
  agent invocations as §5.2.9 spells them, transcripts in `/work/walk-logs/`,
  the tree committed and collected. The fixed prompt is stored verbatim.
- VA-2 — whether the capsule proxy logs allowed requests, or only refused
  ones, is found out and recorded in the phase sheet (F-20); PHASE-09's
  fetch-attempt column depends on it.
- VA-3 — nothing credential-bearing is in the walk directory.

**Verification (human)**
- VH-1 — the user watches the capsule session of EX-1 and EX-2 and accepts
  the negative control's output.

**Notes for the implementer**
- The walk script lives in the slice folder and not in `goad-walk`, because
  the capsule clones `goad-walk` and its agent would read the script, the
  prompt and the control's patterns (PL-5).
- STOP if a bare prompt fails in the capsule: that is authentication (EN-3),
  not R1.
- STOP if either plugin load or `capsule-collect` fails in the capsule in a
  way the kit cannot fix: that is R1 or R7 firing, and the walk's shape is the
  user's.
- Its code surfaces are disjoint from PHASE-06..PHASE-08's, so it may run
  beside them, in its own worktree, whenever EN-2 holds. By §Sequencing's
  rule the worktree writes no `notes.md`: the orchestrator writes this
  phase's sheet and its §Status row on `main`, from the agent's reports —
  VA-2's finding included.

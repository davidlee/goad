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

- **PHASE-01 — refusals name a requirement and a side.** `Requirement`,
  `AtFault`, and total `requirement()`/`fault()` on every taxonomy in strata 1
  and 2 (`design.md` §5.2.3). The corpus witness, red on three fixtures and
  green after their lists are corrected.
- **PHASE-02 — the host's kinds and R-57 values live in stratum 1.**
  `Stimulus`, `Submitted`, `Finite` and `Submitted::as_drawn` move or are
  lifted to `goad_semantics::protocol::canonical`; `goad` delegates
  (`design.md` §5.2.4).
- **PHASE-03 — the ground the checker stands on.** `examples/` becomes
  `exercisers/` (`design.md` §5.2.7); `goad-emit`'s unwritten answer exits 2
  (§5.2.5); `config::Command::from_argv` and `config::positive_duration`
  are public (§5.2.1); `version_line` has one home (`plan-log.md` PL-7).
- **PHASE-04 — `goad-check`.** The binary, its arguments, its run sequence,
  its report and its statuses, with the binary tier (`design.md` §5.2.1,
  §5.2.2, §5.2.5).
- **PHASE-05 — packages and the plugin's shell.** The flake exports
  `goad-check` and `goad-kit`; the manifests and a minimal `SKILL.md` exist
  and validate; both agents load the plugin from its store path on the host;
  `goad-walk` builds the full tool set (`design.md` §5.2.6, §5.2.8; R1).
- **PHASE-11 — the capsule.** The walk script; one capsule stood up, its
  negative control passed, both agents loading the plugin in it, and a tree
  collected (`design.md` §5.2.8; R1, R7). Waits on oubliette.
- **PHASE-06 — the examples.** The three kit examples, each accepted by the
  checker in the gate; `python3` and `jq` in the devshell; the second
  `deno check` path (`design.md` §5.2.6 *The examples*).
- **PHASE-07 — the fence gate, and the protocol and transport reference.** The
  tagged-fence extractor and every role's check; `protocol.md` and
  `running.md`; I-5's path test.
- **PHASE-08 — the rest of the reference, and its coverage.** `scheduling.md`,
  `events.md`, `checking.md`, the finished `SKILL.md`; the coverage test over
  every requirement a refusal can name.
- **PHASE-09 — the first walks.** One Claude Code walk and one Codex walk,
  measured, verdict-checked, run by a person, read, and every friction item
  dispositioned (`design.md` §5.2.9).
- **PHASE-10 — kit fixes, and the re-walks.** The kit fixes land, `goad-walk`'s
  lock moves to them, and each agent walks once more.

### Owed to audit and close

No phase can do these: they are audit's or close's by `docs/AGENTS.md`. Each is
also a checklist item — PHASE-10/VA-4 checks that `notes.md` §Open carries
every one, so none is left to prose.

- **Promote `canon-delta.md`** — SPEC-001 Changes 1–7, SPEC-004 Changes 1–7,
  POL-001 Change 1, ADR-003 Change 1 — with the user's endorsement, recording
  each in `audit.md`'s Reconciliation table. AC-4 and AC-7's canon half land
  only here.
- **Record the person-runs** in `audit.md` §Evidence, citing the VH criteria
  below by id.
- **Extend FU-5 and FU-7** in `docs/follow-ups.md` (`notes.md` §Open names the
  shipped symbols, PHASE-04/VA-5 and PHASE-07/VA-4).
- **Disposition every other §Open candidate** at close.

**Test names are commitments.** `design.md` §9 and `canon-delta.md` name the
cases. A phase that ships a case under another name or in another file updates
`canon-delta.md` in the same commit and says so in its phase sheet (the draft
is the working authority, `docs/AGENTS.md` §Canon that does not exist yet).
Canon keeps its old citations until audit promotes the delta.

## Sequencing & rationale

```
PHASE-01 ─┐
PHASE-02 ─┼─► PHASE-04 ─► PHASE-05 ─► PHASE-06 ─► PHASE-07 ─► PHASE-08 ─┬─► PHASE-09 ─► PHASE-10
PHASE-03 ─┘               (packages)  (examples)  (fences)    (coverage) │   (walks)     (fix, re-walk)
                              │                                          │
                              └─► PHASE-11 (capsule; waits on oubliette) ┘
```

**Why the host changes come first.** The checker only prints what the host
knows (`design.md` §4, principle 1). It cannot be written until each taxonomy
answers `requirement()` and `fault()` (PHASE-01), until the kinds and R-57
values it sends have a stratum-1 home (PHASE-02), and until it can build a
command by the host's own rule (PHASE-03).

**PHASE-01, PHASE-02 and PHASE-03 may run in parallel**, each in its own
worktree with one writer, merged in order. Their surfaces are disjoint by
file. The default is sequential; parallel is the user's call at phase-plan
time.

**Why the plugin loads come early, and the capsule splits out** (`design.md`
§8 R1 and R7, and their mitigations; `plan-log.md` 2026-10-01, *the capsule
splits out*). Whether each model reads a skill loaded from a store path has
never been run; PHASE-05 runs it on the host with a placeholder skill, before
any prose, so a blocker costs a manifest and not a reference. The capsule
(PHASE-11) waits on oubliette work outside this slice, so it runs as early as
oubliette allows, beside PHASE-06..PHASE-08, and must be done before
PHASE-09. The kit's prose does not depend on where the walk runs, so R7
firing late changes the walk's shape, not the reference. PHASE-05 needs
`goad-check` to exist, because `goad-walk`'s tool set holds it.

**Why the examples come before the reference.** The reference's examples cite
real backends, and the examples are the kit's highest-value content for the
walk. The examples' gate test also creates `goad-check`'s `kit` test target,
which PHASE-07 extends.

**Why the reference is two phases.** Reading SPEC-001..003 and writing five
reference files with tagged fences does not fit one session with its tests.
The split is at the coverage test: PHASE-07 builds the fence gate and the two
files most of the requirement ids live in; PHASE-08 writes the coverage test
red, and turns it green with the remaining three files.

**Why the walks are two phases.** The first walks produce the friction the
kit fixes answer. The re-walk rule (`design.md` §5.2.9) requires the fixes on
`main` and `goad-walk`'s lock moved to that revision before either re-walk.

**Size.** Each phase fits one session, bookkeeping included. PHASE-04 is the
largest in code; PHASE-07 and PHASE-08 in prose. PHASE-09, PHASE-10 and
PHASE-11 are long in wall time and in person time, not in tokens. A phase that
reaches its budget unfinished checkpoints PARTIAL in its sheet and hands over.

**Mutation evidence** goes in the phase's sheet in `notes.md`, under a
**Mutation evidence** heading, one row per mutation: the edit (quoted), the
command, that the mutated build **compiled**, the cases that went red **by
name**, and that the restore is green. A mutation that does not compile is not
evidence (`docs/memory/negative-control-must-compile.md`). Runs use `--no-fail-fast`.
A mutation is applied by copying the file to the scratchpad and copying it
back — never `git checkout` or `git stash` — and `git status` is clean after
each restore. The same rules bind every **reach proof** below: a planted
breach that an instrument is shown to catch.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-11/EX-1, EX-2 (the capsule, its negative control, the plugin in it); PHASE-09/EX-3 (each first walk: control passed, verdict 0 with a view answered, no goad source read) and PHASE-09/VH-1 (a person ran each walk's backend); PHASE-10/EX-4 and PHASE-10/VH-1 (each re-walk, the same) |
| AC-2 | PHASE-07/VT-1 (every json/toml fence in the kit is tagged and checked) with VT-2..VT-4 (its negative controls) and VA-2 (the untag mutation); PHASE-08/EX-1 (the same test over the finished reference) |
| AC-3 | PHASE-04/VT-2 (each refusal reported with side and requirement, from the host's `Outcome`) and PHASE-04/VA-1, VA-2 (I-1, I-2); PHASE-01/VT-1, VT-2 (the data it prints) |
| AC-4 | PHASE-03/VT-1 (`goad-emit`); PHASE-04/VT-2 (`goad-check`'s three statuses); the statement itself is `canon-delta.md` SPEC-004, **promoted at audit** |
| AC-5 | PHASE-06/VT-1, VT-2 (each example accepted; the triage side effect) and PHASE-06/VA-3 (each example read against the reference and SPEC-001); PHASE-03/EX-1 (the exercisers renamed and no longer presented as the file to copy) |
| AC-6 | PHASE-06/VH-1 |
| AC-7 | PHASE-01/VT-1, VT-2, VT-3 (total methods for every refusal kind; the corpus witness); the statement is `canon-delta.md` SPEC-001 Changes 1–2, **promoted at audit** |
| AC-8 | PHASE-09/EX-2; PHASE-10/EX-3, EX-4 and VH-2 (the re-walk does not regress) |
| AC-9 | PHASE-09/EX-4, VA-2; PHASE-10/EX-1, EX-5 |

`design.md` §9's tests, by phase:

| test | phase |
|---|---|
| `every_protocol_error_names_a_requirement_and_a_side`, and its bounds and schedule siblings | 01 |
| `every_backend_error_names_a_requirement_and_a_side`, and its cleanup and state siblings | 01 |
| `every_refusal_fixture_names_a_requirement_in_its_own_list`, `every_discard_fixture_names_a_requirement_in_its_own_list` | 01 |
| the moved `Stimulus` tests; `every_submitted_kind_writes_the_json_type_r57_names`; the moved `draft.rs` value tests; `an_as_drawn_choice_submits_the_first_alternative` and siblings | 02 |
| `the_projection_to_submitted_is_the_identity_on_each_kind` | 02 |
| `goad-emit`: `an_answer_that_cannot_be_written_exits_2` | 03 |
| `goad-check` `args.rs` invocation table; every binary-tier case in §9; `the_probe_kind_is_none_of_the_host_s_own` | 04 |
| `each_shipped_example_is_accepted_by_the_checker`, `downloads_triage_moves_the_file_it_was_asked_about` | 06 |
| `every_json_and_toml_fence_in_the_kit_is_tagged_and_checked`, `an_untagged_json_fence_is_refused`, `a_jsonc_fence_is_refused`, `a_respond_fence_with_a_value_of_the_wrong_json_type_is_refused`, `nothing_in_the_kit_names_a_path_outside_it` | 07 |
| `every_requirement_a_refusal_can_name_is_explained_in_the_reference`, and its R-32/R-3 negative control | 08 |

`design.md` §9's mutation checks: `NestedHints` flipped to R-3 — PHASE-01/VA-2;
an example exits 1 on the probe — PHASE-06/VA-2; one fence untagged —
PHASE-07/VA-2. Its *outside the gate* checks: `claude plugin validate` —
PHASE-05/EX-3, PHASE-08/EX-4, PHASE-10/EX-2; `nix build` and `goad-walk`'s
tool set — PHASE-05/EX-1, EX-4; the negative control — PHASE-11/EX-1 and
before every walk. Its *observed by a person* runs: PHASE-03/VH-1 (`just
demo`), PHASE-06/VH-1..VH-3, PHASE-09/VH-1, PHASE-10/VH-1; the capsule session — PHASE-11/VH-1.

---

## PHASE-01 — refusals name a requirement and a side

**Objective:** every refusal kind the host can report answers, by a total
match, the requirement and the side SPEC-001/R-59 (`canon-delta.md` SPEC-001
Change 1) assigns it, and the fixture corpus witnesses the protocol and
schedule answers.

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
  `AtFault { Backend, Host, Configuration, Environment }`, as `design.md`
  §5.2.3 gives them.
- EX-2 — `ProtocolError`, `BoundsError` and `ScheduleError` in stratum 1, and
  `BackendError`, `CleanupFailure` and `StateError` in stratum 2, each have
  `requirement()` and `fault()`: total matches with no `_` arm, answering
  exactly §5.2.3's table — `InapplicableKey` split on `key`, `Bounds`,
  `Schedule` and `Protocol` delegating.
- EX-3 — `ConfigError`, `EnvelopeFault` and `SpanFault` have neither method.
- EX-4 — the three fixture lists read as §5.2.3 corrects them — the two R-17
  `Json` fixtures [R-17, R-44], `R-52-a-choice-field-with-no-alternatives`
  [R-52, R-53, R-16] — and no other fixture list is edited.
- EX-5 — the witness's red on exactly those three fixtures is recorded in the
  phase sheet, before the list corrections.

**Verification**
- VT-1 — `goad-semantics` `error.rs` tests:
  `every_protocol_error_names_a_requirement_and_a_side` and its bounds and
  schedule siblings, beside `must_name`, each an exhaustive table of §5.2.3's
  rows.
- VT-2 — `goad-shell` `error.rs` tests:
  `every_backend_error_names_a_requirement_and_a_side` and its cleanup and
  state siblings.
- VT-3 — `normalize.rs`:
  `every_refusal_fixture_names_a_requirement_in_its_own_list` over the
  `protocol` and `protocol-text` corpora, and
  `every_discard_fixture_names_a_requirement_in_its_own_list` over the
  `Discarded` fixtures and the schedule corpus.
- VA-1 — the §5.2.3 table and the code agree row for row, read side by side
  and recorded in the phase sheet; this is the review the witness's stated
  reach leaves (F-23).
- VA-2 — mutation: `NestedHints`' `requirement()` arm R-18 → R-3 reds
  `every_refusal_fixture_names_a_requirement_in_its_own_list`. Recorded.
- VA-3 — `canon-delta.md` SPEC-001 Change 2's test names resolve to the
  shipped cases.

**Notes for the implementer**
- Red first: write the witness tests, see them fail on the three fixtures
  (EX-5), correct the lists, then the methods. `every_protocol_error` in
  `error.rs`' tests already builds one of each variant; extend it, do not
  write a second builder.
- The `requirement` array is the fixture's claim, not the code's: the three
  corrections are the only lists edited to agree with the code (`design.md`
  §5.2.3, *Its reach*).
- STOP if any other fixture's produced error falls outside its own list: that
  is a §5.2.3 row the design got wrong, not a list to edit.

---

## PHASE-02 — the host's kinds and R-57 values live in stratum 1

**Objective:** `Stimulus`, `Submitted`, `Finite` and the as-drawn value per
kind each have one encoding, in `goad_semantics::protocol::canonical`, and
`goad` names no kind string and decides no submitted value's JSON type.

**Surfaces:** `crates/goad-semantics/src/protocol/canonical.rs`;
`crates/goad/src/{wire.rs, draft.rs, view_model.rs, controller.rs, install.rs,
main.rs, glass.rs}` (imports and the delegation only, in the last four);
`crates/goad/tests/` (imports of `Stimulus`);
`crates/goad-shell/src/ingress/envelope.rs` (`HOST_SOURCE`, `plan-log.md`
PL-2); `canon-delta.md` (test names only).

**Entry**
- EN-1 — PHASE-01's entry holds (this phase does not depend on PHASE-01).
- EN-2 — `design.md` §5.2.4 says how `view_model::as_drawn` delegates to
  `Submitted::as_drawn`: a `FieldKind` rebuilt from the `DrawnKind`, and
  `impl From<Submitted> for Edited` back (`design-log.md` 2026-10-01, G1).

**Exit**
- EX-1 — `Stimulus` is in `canonical.rs` beside `Event`, `kind` and `event`
  unchanged; `crates/goad/src` holds no `"startup"`, `"requested"` or
  `"scheduled"` literal.
- EX-2 — `Submitted` (one variant per `FieldKind`), `Submitted::to_json` (the
  single R-57 site) and `Finite` (with its doc) are in `canonical.rs`;
  `draft::submitted` is `edited.submitted().to_json()`, and
  `Edited::submitted` decides no type.
- EX-3 — `Submitted::as_drawn` is the one statement of the untouched-value
  policy; `view_model::as_drawn` delegates to it by EN-2's route, and
  `view_model::untouched` still shows what `as_drawn` submits (P-3).
- EX-6 — `HOST_SOURCE` is a `pub const` beside `Stimulus`; `Stimulus::event`
  and `envelope.rs`' `ReservedSource` check both name it, and no other
  `"host"` source literal remains in `crates/*/src` (PL-2).
- EX-7 — `DrawnKind::Choice` has no `first`; its stale doc goes with it
  (PL-3).
- EX-4 — `goad`'s renderer tier is green unchanged: `fields.rs`'s R-57/R-58
  cases, and `view_model.rs`'s `as_drawn` tests (R5).
- EX-5 — the doc citations that move are by symbol (`Stimulus::event`'s doc
  cites `canonical.rs` by line today).

**Verification**
- VT-1 — `canonical.rs` tests: `a_scheduled_stimulus_names_itself_scheduled`
  and `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`,
  moved verbatim.
- VT-2 — `canonical.rs`: `every_submitted_kind_writes_the_json_type_r57_names`;
  `draft.rs`' value tests moved against `Submitted`
  (`a_boolean_field_submits_a_json_boolean`,
  `a_finite_refuses_every_number_json_cannot_carry`,
  `a_picked_datetime_submits_the_offset_it_was_picked_in` and the rest of that
  group); `an_as_drawn_choice_submits_the_first_alternative` and a sibling per
  kind, including the `number` min-or-zero and `datetime` epoch cases.
- VT-3 — `draft.rs`: `the_projection_to_submitted_is_the_identity_on_each_kind`,
  holding also the round trip `Submitted` → `Edited` → `Submitted` (G1).
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
`flake.nix` (comments), `docs/roadmap.md`, the four `docs/memory/` files the
table names; `crates/goad-emit/src/main.rs`,
`crates/goad-emit/tests/binary/exchange.rs`;
`crates/goad-shell/src/config.rs` (`from_argv`'s and `unsigned`'s
visibility, and the `Command` doc, PL-1); `version_line`'s homes
(`crates/goad-shell/src/report.rs`, `crates/goad/src/diagnostics.rs`,
`crates/goad-emit/src/render.rs` and their callers, PL-7).

**Entry**
- EN-1 — PHASE-01's entry holds (independent of PHASE-01 and PHASE-02).

**Exit**
- EX-1 — `exercisers/` holds `shell/backend.sh`, `typescript/{backend.ts,
  README.md}` and `demo.toml`; each header says it is a host exerciser and
  points at `kit/`; `backend.ts`' "Copy this file" and `round_trip.rs`'s "the
  file a person copies" are gone.
- EX-2 — `git grep -n 'examples/' -- ':!docs/slices' ':!docs/brief.md'` finds
  only POL-001's command block, which audit amends.
- EX-3 — the `justfile`'s `typecheck` is `deno check
  exercisers/typescript/backend.ts`, and its comment is rewritten as §5.2.7
  says. This is `canon-delta.md` POL-001 Change 1's line less the kit path,
  which PHASE-06 adds.
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
- EX-8 — `version_line` is defined once, in `goad_shell::report`, taking the
  package version as a parameter: `env!("CARGO_PKG_VERSION")` expands in the
  crate that compiles it, so each binary passes its own. `goad` and
  `goad-emit` call it, and their `--version` output is unchanged (PL-7).

**Verification**
- VT-1 — `goad-emit` binary tier: `an_answer_that_cannot_be_written_exits_2`
  (`--help` and `--version`, stdout on `/dev/full`; status 2; the last stderr
  line begins `goad-emit: `), modelled on `goad`'s `exit_codes.rs` case of the
  same name; seen red before `main.rs` changes.
- VT-2 — `round_trip.rs`'s `the_readme_s_own_config_loads_and_runs_the_example`
  and `harness.rs`' deno cases are green on the new paths.
- VT-3 — `config.rs` unit tests: `positive_duration` refuses `0s` and `-1s`
  under the key it is given.
- VT-4 — `goad`'s and `goad-emit`'s existing `--version` cases stay green
  across the lift.
- VA-1 — `just -n check` prints the POL-001 sequence with EX-3's line in
  fourth place.
- VA-2 — the README's counts touched here are replaced by names ("the
  ten-line shell backend").

**Verification (human)**
- VH-1 — a person runs `just demo` on the renamed exerciser and sees the
  window prompt, as before.

**Notes for the implementer**
- `docs/brief.md` and closed slices' docs are not edited (§5.2.7).
- The TypeScript exerciser stays; `harness.rs` and `round_trip.rs` drive it.

---

## PHASE-04 — `goad-check`

**Objective:** a headless `goad-check` binary drives a backend command through
`goad_shell::host::Host`, reports each refusal with the side and requirement
its kind answers, and exits 0, 1 or 2 as `canon-delta.md` SPEC-004 R-11..R-15
state.

**Surfaces:** `crates/goad-check/` (new: `Cargo.toml`, `src/`,
`tests/binary/` and its bash fixtures); the root `Cargo.toml`'s `members`
(appended after `goad-emit`, before `goad-boundary`) and `Cargo.lock`;
`crates/goad-boundary/tests/checks/allowlist.rs` (its module doc's member
list only); `canon-delta.md` (test paths only).

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
- EX-2 — the request plan of §5.2.2 in order, the R-56 probe kind a
  `goad-check` constant, each view answered with its first option and
  `Submitted::as_drawn` per field, chains followed to `view: null` or a
  failure up to the bound of 8.
- EX-3 — the report on stdout through `report::try_line_to`: every channel of
  every `Outcome` as §5.2.2's table gives it, each refusal line with side and
  `SPEC-001/R-N`, the R-56 line only on its condition, stderr verbatim with
  truncation flagged, the no-view line when no exchange returned a view, and
  the values sent.
- EX-4 — statuses as §5.2.5: every cause of 2 through one
  `ExitCode::from(2)` that reads no cause; the last stderr line on 1 and 2
  begins `goad-check: `.
- EX-5 — `crates/goad-check/Cargo.toml` names only strata 1 and 2 and
  workspace dependencies already in the lockfile, with a comment arguing it
  links no renderer, as `goad-emit`'s does (I-6).
- EX-6 — `allowlist.rs`' module doc names `goad-check` among the stratum-3
  members, by name and without a count.

**Verification**
- VT-1 — `args.rs` unit tests: the invocation table (config form, argv form,
  `--event` order, `--timeout` with `--config` refused, `--timeout 0s`
  refused, an empty argv and an empty program refused, help, version).
- VT-2 — binary tier, `tests/binary/`, each case §9 names:
  `a_conforming_backend_is_accepted_and_exits_0`,
  `a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56`,
  `a_refused_view_is_reported_with_its_requirement_and_the_backend_side`,
  `a_discarded_next_check_is_reported_and_exits_1`,
  `an_unspawnable_command_is_reported_against_the_configuration`,
  `a_backend_failing_identically_on_every_kind_is_not_charged_with_r56`,
  `a_backend_that_returns_no_view_is_accepted_and_says_respond_was_not_exercised`,
  `an_unreadable_config_exits_2_and_says_who_spoke`,
  `a_reserved_source_event_file_exits_2`, `an_empty_argv_is_a_usage_error`,
  `a_report_that_cannot_be_written_exits_2` (the run and `--help`),
  `a_chained_view_is_answered_until_null`,
  `a_chain_past_its_bound_is_reported_and_does_not_change_the_status`,
  `a_view_answered_carries_exactly_its_options_fields`. The non-zero cases
  SPEC-004/R-14's row names assert the `goad-check: ` prefix on the **last**
  stderr line.
- VT-3 — `the_probe_kind_is_none_of_the_host_s_own`, a unit test beside the
  probe constant (a binary-only crate's constant is not reachable from
  `tests/binary/`), asserting it is none of `Stimulus`'s kinds.
- VA-1 — I-1: `grep -nE 'R-[0-9]+|AtFault::'` over `crates/goad-check/src`
  finds only the R-56 probe's claim.
- VA-2 — I-2: no `FieldKind` match in `crates/goad-check` (src or tests) maps a
  kind to a JSON type or a value; each match found is read and recorded.
- VA-3 — reach: `goad-boundary`'s
  `no_workspace_member_names_the_users_domain` reads `crates/goad-check/src`
  — a planted domain word in a string literal, in code that compiles, reds it;
  restored. Clippy reaches the crate's `src` and `tests` — a planted
  `.unwrap()` in each reds `cargo clippy --workspace --all-targets -- -D
  warnings`; restored. Both recorded.
- VA-4 — `canon-delta.md` SPEC-001 Change 3's and SPEC-004 Change 5's
  `goad-check` test paths are re-pointed from `…` to the shipped files.
- VA-5 — `notes.md` §Open's FU-7 row names `crates/goad-check/Cargo.toml`'s
  comment as what holds I-6.

**Notes for the implementer**
- Build the runtime, backend and `Host` the way `goad`'s `main.rs` `start` does; read that site first.
- `--version` calls `goad_shell::report::version_line` (PHASE-03/EX-8); no
  third copy.
- Mirror `goad-emit`'s shape: pure `args.rs`, a `render`-style module that
  owns every line's text, `main` alone reading the environment, files and the
  clock. `autotests = false` with `[[test]]` targets, as every member does.
- Test backends are small `bash` scripts under `tests/binary/`. Nothing
  asserts a duration (R3).
- A new external dependency (a temp-dir crate, an argument parser) is a STOP.
- `Failure::State` cannot be provoked by a cooperating test; the path to 2 is
  structural (SPEC-004 Change 5's R-11..R-13 row).

---

## PHASE-05 — packages and the plugin's shell

**Objective:** the flake exports `goad-check` and `goad-kit`; the plugin's
manifests exist and validate; both agents, on the host in a fresh home, load
the plugin from its store path and see the skill; and `goad-walk` builds the
full tool set.

**Surfaces:** `flake.nix` (`packages.goad-check`, `packages.goad-kit`);
`.claude-plugin/marketplace.json`; `.agents/plugins/marketplace.json`;
`kit/.claude-plugin/plugin.json`; `kit/.codex-plugin/plugin.json`;
`kit/skills/goad-backend/SKILL.md` (a placeholder: frontmatter and a line);
`README.md` (the kit and plugin-install line); `justfile` (`package` gains
`goad-check` and `goad-kit`, `install` gains `goad-check`, PL-6);
`~/dev/goad-walk/{flake.nix, flake.lock, README.md}`, committed in that
repository.

**Entry**
- EN-1 — PHASE-04 done.

**Exit**
- EX-1 — `nix build --no-link .#goad-check .#goad-kit` succeeds from the bare
  git form (new files `git add`ed first). `goad-kit` holds the two marketplace
  manifests and `kit/`, and nothing else.
- EX-2 — both plugin manifests carry `workspace.package.version`, checked
  against `Cargo.toml` and recorded.
- EX-3 — `claude plugin validate kit/` passes.
- EX-4 — `goad-walk`'s tool set is `goad-check`, `goad-emit`, `goad`,
  `goad-kit`, `ruby` and `jq`, with `goad-kit` re-exported, its stubs for the
  two pending packages gone, its README current, its lock pinned to this
  phase's goad commit, and it builds.
- EX-7 — `just package` builds `goad-check` and `goad-kit`, and `just
  install` installs `goad-check` (PL-6).
- EX-8 — on the host, in a fresh home that has held no session,
  `claude -p --plugin-dir "$KIT/kit"` and Codex after `codex plugin
  marketplace add "$KIT"; codex plugin add goad@goad` each answer a trivial
  headless prompt by naming the `goad-backend` skill, `$KIT` being
  `goad-walk#goad-kit`'s store path (R1, as far as the host reaches it;
  `plan-log.md` 2026-10-01, *the capsule splits out*).

**Verification**
- VA-1 — Codex's manifest, `interface` block included, is accepted:
  `codex plugin marketplace add` and `codex plugin add goad@goad` against the
  `goad-kit` store path in a fresh `CODEX_HOME` on the host succeed, and
  `~/.codex/plugins/cache/` holds only `kit/`'s contents.
- VA-4 — nothing credential-bearing is in `goad-walk`'s tool set.

**Notes for the implementer**
- The spike (`spike/`, `research.md` §"Spike: R1 and R2") is prior art for
  the plugin loads; `goad-kit` is a marketplace root, and Claude loads
  `"$KIT/kit"`.
- STOP if either plugin load fails in a way the kit cannot fix: that is R1
  firing, and the walk's shape is the user's.

---

## PHASE-06 — the examples

**Objective:** the kit ships the focus check, Downloads triage and breadcrumbs
examples of `design.md` §5.2.6, each accepted by `goad-check` in the gate
with its own config and event files, and a person has seen each run.

**Surfaces:** `kit/skills/goad-backend/examples/` (new: `focus-check/`,
`downloads-triage/`, `breadcrumbs/`, as §5.2.6's tree lists them);
`crates/goad-check/tests/kit/` (new target) and `crates/goad-check/Cargo.toml`
(its `[[test]]`); `flake.nix` (`python3` and `jq` in `projectPkgs`);
`justfile` (`typecheck`'s second path); `canon-delta.md` (test paths only).

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
  Change 1's line.

**Verification**
- VT-1 — `kit` tier: `each_shipped_example_is_accepted_by_the_checker` — each
  example copied to a temporary directory, the checker started there in the
  config form with the example's event files, `HOME` and `XDG_*` temporary.
- VT-2 — `downloads_triage_moves_the_file_it_was_asked_about`: the file the
  event names, created under the temporary `XDG_DOWNLOAD_DIR`, is moved to
  the as-drawn target.
- VA-1 — `just -n check` prints EX-3's line in fourth place.
- VA-2 — mutation: one example made to exit 1 on the probe's kind reds VT-1.
  Recorded.
- VA-3 — each example is read against SPEC-001 R-56, R-57, R-58 and R-33 and
  against its README; findings go in the phase sheet. (No review reached the
  examples' correctness.)
- VA-4 — `canon-delta.md` SPEC-001 Change 3's kit-tier path for
  `each_shipped_example_is_accepted_by_the_checker` is re-pointed to the
  shipped file.

**Verification (human)**
- VH-1 — AC-6: a person runs `goad-check --config config.toml` in each
  example's directory, and against a broken backend (one of PHASE-04's
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

---

## PHASE-07 — the fence gate, and the protocol and transport reference

**Objective:** every `json` and `toml` fence in the kit is tagged and checked
by the gate through the host's own doors, and the reference's `protocol.md`
and `running.md` exist with their examples checked.

**Surfaces:** `crates/goad-check/tests/kit/`;
`kit/skills/goad-backend/reference/{protocol.md, running.md}`; the example
READMEs (fence tags only).

**Entry**
- EN-1 — PHASE-06 done.

**Exit**
- EX-1 — the extractor is §5.2.6's: a CommonMark fence scanner for backtick
  and tilde fences, info string split on whitespace; a block whose first info
  word, lowercased, begins `json` or `toml` fails unless that word is exactly
  `json` or `toml` with a known `goad:` role.
- EX-2 — each role in §5.2.6's table is checked as the table says: responses
  through `read_response` (accepted, refused by id, discarded by id); evaluate
  requests by framing; respond requests by framing, R-58's field set and the
  JSON-type oracle over `Submitted::as_drawn`; envelopes through
  `envelope::normalize`; configs through `Config::parse`.
- EX-3 — `protocol.md` and `running.md` state only what an author must do or
  may rely on, organised by task, each rule citing its id as a report prints
  it (`SPEC-001/R-13`), each id at an anchor of its own.

**Verification**
- VT-1 — `every_json_and_toml_fence_in_the_kit_is_tagged_and_checked`.
- VT-2 — `an_untagged_json_fence_is_refused`, over an inline string.
- VT-3 — `a_jsonc_fence_is_refused`, over an inline string.
- VT-4 — `a_respond_fence_with_a_value_of_the_wrong_json_type_is_refused`.
- VT-5 — `nothing_in_the_kit_names_a_path_outside_it` (I-5).
- VA-1 — the extractor's count of checked fences is non-zero and equals a
  count taken by hand over the kit, recorded.
- VA-2 — mutation: one fence in `protocol.md` untagged reds VT-1. Recorded.
- VA-3 — `protocol.md` and `running.md` read against SPEC-001 §4 and §6:
  nothing contradicts it, and nothing host-internal is taught. Recorded.
- VA-4 — `notes.md` §Open's FU-5 row names the kit fence scanner's symbol
  beside `round_trip.rs`' `fenced_block`.

**Notes for the implementer**
- The respond oracle writes no reader of `Submitted` (U5); I-2 binds tests too.
- Fragments are shown whole or as `text` (§5.2.6).

---

## PHASE-08 — the rest of the reference, and its coverage

**Objective:** the reference is complete — `scheduling.md`, `events.md`,
`checking.md` and the finished `SKILL.md` — and the gate holds that every
requirement any refusal kind can name is explained in it.

**Surfaces:** `kit/skills/goad-backend/reference/{scheduling.md, events.md,
checking.md}`; `kit/skills/goad-backend/SKILL.md`;
`crates/goad-check/tests/kit/`.

**Entry**
- EN-1 — PHASE-07 done.

**Exit**
- EX-1 — VT-1 of PHASE-07 is green over the finished kit.
- EX-2 — `SKILL.md` routes and does not teach: when to use it, the loop, how
  to get the binaries, one pointer per reference file by the question it
  answers, one line per example; no wire example of its own.
- EX-3 — `checking.md` explains the checker's forms and what it sends, the
  four sides in R-59's terms, where else to look for each kind with a declared
  imprecision, that a run mutates real state (and the `XDG_*` advice), that a
  relative `command` resolves against the starting directory, and the three
  statuses.
- EX-4 — `claude plugin validate kit/` passes, and the manifests' versions
  still match.

**Verification**
- VT-1 — `every_requirement_a_refusal_can_name_is_explained_in_the_reference`:
  one instance per variant (two for `InapplicableKey`), each builder beside an
  exhaustive match with no `_` arm; an id matches only when followed by a
  non-digit or the end. Seen red before the three files are written.
- VT-2 — its negative control over an inline string where `SPEC-001/R-32`
  appears and `SPEC-001/R-3` does not.
- VA-1 — `scheduling.md` and `events.md` read against SPEC-001 R-21..R-29,
  SPEC-002 and SPEC-003; `checking.md` against `design.md` §5.2.5 and
  `canon-delta.md` SPEC-004. Recorded.

---

## PHASE-09 — the first walks

**Objective:** one Claude Code walk and one Codex walk have run from the fixed
prompt in fresh capsules; each is measured, verdict-checked, run by a person
and read; and every friction item is dispositioned.

**Surfaces:** `docs/slices/012/walks.md` (new);
`docs/slices/012/walks/<agent>-1-ISSUES.md` (new);
`docs/slices/012/walk/` (fixes to the script only). Walk trees and transcripts
stay in the quarantine and `goad-walk`, outside this repository.

**Entry**
- EN-1 — PHASE-08 done, on `main`; `goad-walk`'s lock pins that revision
  (the script's lock bump, committed in `goad-walk`).
- EN-2 — PHASE-11 done.

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

**Verification (human)**
- VH-1 — as PHASE-09/VH-1, for each re-walk.
- VH-2 — the person judges, from `walks.md`, that neither re-walk regressed in
  turns or total tokens (AC-8; indicative, no threshold).

---

## PHASE-11 — the capsule

**Objective:** the walk script exists, and a capsule provisioned from
`goad-walk` has passed its negative control, had the plugin loaded and the
skill seen by both agents, and brought a committed tree back through
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
- EX-2 — in that capsule, `claude --plugin-dir "$KIT/kit"` and Codex after
  `codex plugin marketplace add "$KIT"; codex plugin add goad@goad` each
  answer a trivial headless prompt by naming the `goad-backend` skill; and the
  walk script's collection step brings a committed tree back through
  `capsule-collect`.

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
- STOP if either plugin load or `capsule-collect` fails in the capsule in a
  way the kit cannot fix: that is R1 or R7 firing, and the walk's shape is the
  user's.
- Its surfaces are disjoint from PHASE-06..PHASE-08's, so it may run beside
  them, in its own worktree, whenever EN-2 holds.

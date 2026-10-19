# Audit & reconciliation — Slice 015

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `c68a624..53d140a` on branch `slice-015` (worktree
`.claude/worktrees/slice-015`): PHASE-01 through PHASE-04, ending
`015: PHASE-04 — VH-1 run by the person; phase done`.

**Question:** the slice is finished when a config whose `command` and
`ingress.path` are relative works however goad, `goad-check --config` and
`goad-emit` were started — through a symbolic link too — and the record says
so: the kit with no *make every path absolute* advice, and SPEC-001 /
SPEC-003 carrying the rule once `canon-delta.md` is promoted.

What this audit checks, decided before looking:

1. **The gate.** `just check` exits 0 on `53d140a`, run by the auditor, with a
   `TMPDIR` short enough for the 107-byte socket control.
2. **Each AC** (AC-1 … AC-5) against the code and the tests, not the phase
   sheets' claims. AC-4 and AC-5's person half from PHASE-04's *VH-1* record.
   AC-3's canon half is promotion, which is not yet done.
3. **Each VT / VA / VH** in `plan.md`, walked against the tree: the named
   test exists, asserts what the criterion says, and is green. Natural-red and
   mutation tables checked for missing rows, uncompiled mutants, and reds for a
   reason other than the one claimed; a sample of mutations re-run by the
   auditor — at least one per phase judged weakest — by scratchpad
   copy-and-restore, `--no-fail-fast`.
4. **Surface delta.** Each phase commit range's touched paths against that
   phase's declared Surfaces; every undeclared path classified.
5. **Reconciliation.** One row per `canon-delta.md` change; each §7 test
   citation resolves to a `fn` in the tree; any other canon / doc / code
   divergence classified. Drafted only: canon is not edited here.
6. **Design drift.** Where shipped code departs from `design.md`.

Out of scope here: in-depth code review (a fresh reviewer, `review-code.md`)
and repairs. Defects found on the way are listed as *Leads for the code
reviewer*.

**Invariants held** (`CLAUDE.md`):

- *The host does not understand the user's domain* — no domain vocabulary in
  new host types or names; the working directory is a transport fact, not a
  domain one.
- *Permissive parsing, canonical representation* — no argument in argv is
  reinterpreted as a path; the anchored directory is fixed at `Config::load`,
  the one door, and nothing downstream re-derives it.
- *No narrowing of wire compatibility for the renderer* — the config's
  accepted shape is unchanged; only what a relative path means changes.
- *A backend failure never takes the host down* — a spawn failure in the new
  directory is still `BackendError::Spawn`, R-44, the configuration's side,
  and names where it looked.
- *Strata run one way* (ADR-001) — nothing enters `goad-semantics`; the clock,
  filesystem and subprocess work stays in `goad-shell` and above.

<!-- This is the audit's scope — evidence, criteria, canon. The code review's
     own lines of attack belong in `review-code.md`'s Brief, not here. -->

## Evidence

<!-- What was run and what it said. Not a claim of correctness — the basis for
     one. -->

Gathered by the auditor on `slice-015` at `53d140a` (code) / `82462df` (the
Brief), 2026-10-19. Every claim below was checked against the tree or re-run;
where it rests on a phase sheet instead, it says so.

### Tests / checks

- **`just check`**: exit 0, run by the auditor in the worktree. `TMPDIR`
  unset, so `temp_dir()` is `/tmp` and the 107-byte socket control has room
  (`ingress.rs::a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path`
  green). Every test target passed, none failed, none ignored; clippy
  `-D warnings` and `cargo fmt --all --check` clean.
- Each case this slice wrote or changed was confirmed `ok` by name in that run:
  the `working_directory.rs` cases, the `config.rs` unit cases, the
  `startup.rs::listener` and `goad-emit` AC-5 cases, the length pin, both
  `run.rs` pins, `examples.rs`' two cases, `error.rs`'s VT-9,
  `transport.rs::a_command_that_does_not_exist_fails_to_spawn`,
  `round_trip.rs::the_readme_s_own_config_loads_and_runs_the_example`, and
  `failure_matrix.rs::a_command_that_cannot_be_spawned_reaches_the_caller_as_a_spawn_failure`.
- `crates/goad-semantics` is untouched by the range; the `justfile`'s `check`
  recipe is untouched (only `emit` and the `package` comment changed), so
  POL-001's sequence is unchanged.
- **After code review round 2** (`90013b3`, the repairs; then the promotion
  commit): `just check` exit 0 on each tree, run by the repairer. The
  promotion touches documents only.
- `main` is still at `c68a624`: nothing has landed there since the plan, so the
  worktree needs no `git merge main` before the merge yet.

### Acceptance criteria

| AC | state | evidence |
|---|---|---|
| AC-1 — relative `command` works from another directory, through a link | **met** | `Config::load` canonicalizes and anchors; `ProcessBackend::exchange` runs `Command::executable()` with `current_dir` and `PWD`. Held by `working_directory.rs`: `a_relative_argument_resolves_beside_the_config_from_another_directory` (`pwd -P` canonical, `$0` as written), `a_linked_config_runs_its_backend_beside_the_target` (a compiled, answering decoy beside the link), `a_relative_program_with_a_slash_is_executed_as_the_anchored_path`, `a_native_program_receives_its_anchored_path_as_its_name`, `the_backend_s_pwd_names_its_working_directory`, `a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for`, `a_config_reached_through_a_linked_directory_is_anchored_at_the_target`; `config.rs::tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash`; `run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config`. Through a link by a person: PHASE-04 *VH-1* step 2. `goad`'s own `main.rs::start` is held by review and AC-4 only (D11, a stated residue) |
| AC-2 — `goad-check --config` behaves the same | **met** | `examples.rs::run_checker` starts the checker in the hand-out root with `--config example/config.toml`; `each_shipped_example_is_accepted_by_the_checker` and `downloads_triage_moves_the_file_it_was_asked_about` green, and M6 reds them. Argv form from the caller's directory: `run.rs::the_argv_form_runs_a_relative_program_from_the_caller_s_directory` (M12 re-run below). `checking.md` states both forms |
| AC-3 — SPEC-001 states the rule with a §Verification row; the kit drops *make every path absolute* | **met** | Kit half met: `running.md` *Name your command* and its `ingress.path` row, `checking.md`, `events.md`, each example's `config.toml` comment and README. The auditor's re-run of PHASE-03's VA-1 grep over `kit/` finds no hit advising an absolute path, or a start directory, to work around where goad started; the one *absolute path* advice left is the socket-length remedy (D8), and `chpwd.zsh`'s is about `goad-emit`'s own directory (PHASE-03 VA-2). Each README's `goad:config` fence is byte-identical to its `config.toml` (re-diffed). **Canon half met** by promotion: SPEC-001 R-60 (§4 *Process transport*, with its §7 row) and §6.4 *Where the backend runs*; SPEC-003 R-17 (§4, with its §7 row), §6.1 and §9; see Reconciliation |
| AC-4 — a person ran goad with a relative-path config from another directory and saw a view | **met** | `notes.md` PHASE-04 *VH-1*: run by the person 2026-10-19 on the worktree's build at `bdc93f2`, in the dev shell. From `/tmp`, goad on `<worktree>/exercisers/demo.toml` by absolute path showed a view; from `/tmp`, through `/tmp/goad-015-demo.toml → <worktree>/exercisers/demo.toml`, a view; the person: *"yeah that was all expected results"*. The code is unchanged from `bdc93f2` to `53d140a` (`53d140a` touches `notes.md` only). **If a code-review repair touches `goad_shell::config`, `backend::process` or `BackendError::Spawn`, VH-1 is run again on the repaired build and recorded here instead (`plan.md` §Owed)**. Code review round 1's repairs touch `config.rs` and `error.rs` in doc comments only; the person waived the re-run on that condition (`design-log.md` 2026-10-19, *code review round 1 dispositions*). Recorded by diff: `git diff 024643d -- crates/goad-shell/src/ \| grep '^[-+]' \| grep -vE '^(\+\+\+\|---)' \| grep -vE '^[-+][[:space:]]*//'` prints nothing (`review-code.md` F-2's Response). Code review round 2's repairs (`review-code.md` F-11) are doc comments only too, and the condition is now checked against VH-1's own build: `git diff --name-only bdc93f2 -- 'crates/*/src/*'` lists only `goad-shell`'s `config.rs` and `error.rs`, and `git diff bdc93f2 -- 'crates/*/src/*' \| grep '^[-+]' \| grep -vE '^(\+\+\+\|---)' \| grep -vE '^[-+][[:space:]]*//'` prints nothing. No production line has changed since the build the person ran, so VH-1 at `bdc93f2` stands for the shipped code |
| AC-5 — a relative `ingress.path` names one socket for goad and `goad-emit` | **met** | `config.rs::tests::a_relative_path_resolves_against_the_directory_given` (byte-exact, `OsStr`); `startup.rs::listener::a_relative_configured_path_binds_beside_the_config` (socket at the canonical config directory, none in the crate directory); `goad-emit`'s `exchange.rs::a_relative_configured_socket_is_found_beside_the_configuration` (started in `<home>/start`, a directory the case owns and not the configuration's; no `--socket`) and `main.rs::tests::a_configured_ingress_path_is_what_is_written_to`. By a person: VH-1 step 3, `just emit reddit-watcher reddit-opened` from the worktree root replaced the step-2 host's view, and step 4 found `exercisers/goad-demo.sock` and its lock with none at the root. VH-1 ran at `bdc93f2`; production source has changed since only in comment lines (AC-4's check) |

### Verification criteria

Walked against the code; *sheet* marks a criterion whose evidence is the
phase sheet's record, checked for completeness but not re-run.

**PHASE-01.** EX-1 `Command::working_directory` and its doc, `None` from
`new` / `from_argv` — in code. EX-2 `executable` pure, the `/` guard on the
string — in code. EX-3 `anchored` is the one join, used by `executable` and
`ingress_config` — in code; `.` alone anchors to the directory (Decision,
pinned in VT-1's table). EX-4 `load` canonicalizes, reads the canonical path,
A-1's arm is `ConfigError::Read` over `InvalidInput`, no panic — in code (see
*Leads* on the doc's `/dev/stdin` claim). EX-5 struct update, anchored
`ingress.path` — in code. EX-6 every `parse` caller passes a directory; the
two meaning-changed cases assert as §9 says — in code.
VT-1, VT-2, VT-3, VT-4, VT-5, VT-6, VT-7 — each exists, asserts what the
criterion says, green. VA-1 — gate green (the *unaffected* list is in it).
VA-2 — `git status` clean in the auditor's runs too. **Discharged.**

**PHASE-02.** EX-1 `Command::new(&executable)`, `current_dir` and
`env("PWD", …)` under `Some`, no `arg0`; `command.spawn()` still the file's
only `spawn` (`transport_shape.rs` green) — in code. EX-2 `Spawn { executable,
working_directory, source }`, `Display` with the parenthesis only under
`Some`, R-44 and the configuration's side unchanged, `table.rs`' prefix
unchanged — in code. EX-4 (D10) README is `["deno", "run", "-A",
"backend.ts"]`, `rooted_at_the_workspace` is gone (the one §9 name that no
longer resolves, by design). EX-5 `run_checker` from the root — in code.
VT-1 … VT-10 exist and assert as stated; VT-9 is an addition to §9, recorded.
VA-1 — `transport.rs` still asserts `NotFound`, `failure_matrix.rs` still
matches `Spawn`. **Discharged.**

**PHASE-03.** VT-1, VT-2 pins exist and assert as stated (VT-2 asserts exit 0,
stronger than *judged*: a spawn failure is judged too). VT-3 kit tier green.
VA-1 grep re-run by the auditor, same hits, same classes. VA-2 `chpwd.zsh`
left, true. VA-3 fences byte-identical, re-diffed. EX-2 … EX-6 prose read
against their criteria; EX-5's socket reason is stated in the two
event-driven READMEs only, `focus-check` keeping *copy* for the read-only kit
(a recorded Decision; see *Design drift*). **Discharged.**

**PHASE-04.** EX-1 `demo.toml` `["bash", "shell/backend.sh"]`, `path =
"./goad-demo.sock"`, the `socat` / `nc` lines say they run from the
repository root. EX-2 `emit` passes `--socket exercisers/goad-demo.sock`; the
`package` comment names it and the kit examples' `goad.sock`, no count. EX-3
`.gitignore` swaps the root pair for `/exercisers/…`. EX-4 `backend.sh`'s
header quotes the new argv. EX-5 the memory note names the new place. EX-7
`shell_example`'s doc states what it shares with `demo.toml` and what not.
VA-1 both greps re-run by the auditor: every hit true under the new rule.
VA-2 re-done independently — every `canon-delta.md` §7 citation resolves to a
`fn` in its named file (Reconciliation below), and every §9 name in
`design.md` resolves except `rooted_at_the_workspace`, deleted by D10. VA-3
the main checkout still holds `goad-demo.sock` and `goad-demo.sock.lock` at
its root; the worktree holds neither. VA-4 `notes.md` §Open carries each
§Owed bullet as a box. VH-1 — see AC-4. **Discharged.**

**Natural-red and mutation tables.** Every row the plan's M-table names is
present: M1–M5 (PHASE-01), M3's second half, M6–M10, M13, M14 (PHASE-02),
M11, M12 (PHASE-03); each records *compiled*, the cases redded by name and
assertion, and a clean restore. Respellings forced by `unused` are recorded
(M1, M2). Each natural red names the assertion and matches §9's reason —
notably VT-5 (PHASE-02) reds on the `PWD` assertion with the backend
answering, not on a spawn failure. No row is missing or uncompiled.
Sheet discrepancies, neither changing a result: PHASE-01 names M5's file
`ingress.rs` (it is `ingress/mod.rs`, which the sheet's Findings correct),
and the orchestrator's re-measures cover M3, M7, M11.

**Mutations re-run by the auditor** — scratchpad copy, edit, `cargo test
--no-fail-fast`, copy back; `git status` clean after each restore.

| id | edit | command | compiled | redded | matches sheet |
|---|---|---|---|---|---|
| M5 (PHASE-01) | `ingress::bind`'s `UnixListener::bind` failure filed as `BindFault::Unprobeable` | `-p goad-shell -p goad` | yes | `ingress::a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path` alone · *"an over-long path is R-4's unusable location, got: could not be inspected: path must be shorter than SUN_LEN"* | yes |
| M9 (PHASE-02) | `.env("PWD", directory)` dropped | `-p goad-shell` | yes | `working_directory::the_backend_s_pwd_names_its_working_directory` alone · `PWD` left the worktree root (inherited), right `/tmp/goad-wd-pwd-<pid>.d` | yes |
| M14 (PHASE-02) | `Spawn`'s `working_directory` filled by `.or_else(\|\| std::env::current_dir().ok())` | `-p goad-shell -p goad -p goad-check` | yes | `transport::a_command_that_does_not_exist_fails_to_spawn` alone · *"a command with no file claims no directory"*, left `Some(<crate dir>)` | yes |
| M8 (PHASE-02) | `command.arg0(&self.command.program)` after `Command::new(&executable)` | `-p goad-shell --test integration` | yes (a first spelling through `std::os::unix::process::CommandExt` did **not** compile against tokio's `Command` and was discarded, not counted) | `working_directory::a_native_program_receives_its_anchored_path_as_its_name` alone · left `./shell`, right `<anchor>/shell`; `a_relative_program_with_a_slash_is_executed_as_the_anchored_path` stayed **green** — D5's premise | yes |
| M12 (PHASE-03) | `args.rs::program` gives the argv form `working_directory: Some(temp_dir())` | `-p goad-check` | yes | `run::the_argv_form_runs_a_relative_program_from_the_caller_s_directory` (stdout: `backend could not be spawned: /tmp/backend.sh (in /tmp)`) and `args::tests::{every_token_after_the_separator_is_the_commands, the_command_after_the_separator_is_the_argv_form_with_a_five_second_timeout, timeout_sets_the_argv_forms_timeout}` | yes |

### Surface delta

Per phase commit range (sheet commits touch `notes.md` only):

| phase | range | paths touched | undeclared |
|---|---|---|---|
| PHASE-01 | `c24c9c8..571787b` | `config.rs`, `error.rs` (`ConfigError::Read`'s doc only), `host.rs`, `round_trip.rs` (the `parse` call only), `startup.rs` (`unparseable`; in `listener`, one new case and its `directory` helper), `tagged.rs`, `exchange.rs`, `goad-emit/src/main.rs` (the one test), `ingress.rs`, `working_directory.rs` and its `mod` line, `scripting.rs` (doc only — no non-comment line changed), `notes.md` | none |
| PHASE-02 | `fe81391..934933f` | `process.rs`, `error.rs`, `transport.rs`, `failure_matrix.rs`, `table.rs`, `coverage.rs`, `working_directory.rs`, `round_trip.rs`, `exercisers/typescript/README.md`, `examples.rs`, `kit/main.rs`, `tree.rs`, `notes.md` | none. The sheet's conditional `scripting.rs` was not touched |
| PHASE-03 | `7fd8cb9..d14d75b` | `run.rs`, `binary/process.rs`, `scripting.rs` (doc only), `running.md`, `checking.md`, `events.md`, each example's `config.toml` and `README.md`, `watch.sh`, `notes.md` | none. `chpwd.zsh` untouched, as VA-2 allows |
| PHASE-04 | `33d8de4..53d140a` | `.gitignore`, `round_trip.rs` (`shell_example`'s doc only), the memory note, `demo.toml`, `backend.sh` (header only), `justfile` (`emit` and the `package` comment only), `notes.md` | none |

Declared but untouched: `chpwd.zsh` (conditional, VA-2 kept it) and PHASE-02's
conditional `scripting.rs`. Nothing dropped. No undeclared path in the slice.

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`, subject `implementation`. Two rounds, each by
  a fresh reviewer; the leads at the foot of this file seeded the first.
  Rounds stopped on the measured trend — two rounds, no code defect — in
  favour of mechanical verification (`design-log.md` 2026-10-19, *code review round 2, residues, and canon promotion*).
- **State:** resolved · outstanding blockers: none. F-11, F-12 and F-13 are
  repaired and their Outcomes are pending mechanical verification by the
  orchestrator.

## Verdict

<!-- The slice's closure story, written once, here. Draws on the ledger's
     synthesis and on the evidence above; restates neither. Does this slice do
     what it set out to do, and what is being accepted knowingly? -->

The slice does what it set out to do. A configuration whose `command` and
`[ingress] path` are relative now means the same thing however goad,
`goad-check --config` and `goad-emit` are started, and through a link to the
file or to a directory above it: the backend runs in the configuration's
canonical directory, a relative program is executed by its joined path, and
the socket is bound beside the file. The rule is canon — SPEC-001/R-60 and
SPEC-003/R-17 — and each clause is held by a case that is red under the spawn
it replaced, or by a pin that says it is one. A person ran it (VH-1). Code
review found no behavioural defect; what it found was the record saying more,
less, or something staler than the code, and that is repaired.

Accepted knowingly:

- **No re-run of VH-1 after the repairs.** The person waived it on condition
  that every repair to production source is a comment; *Evidence* (AC-4)
  records that condition by diff against VH-1's own build.
- **`goad`'s `main.rs::start` is held by review and VH-1, not a test** (D11).
- **A configuration named by a descriptor that is not a file** — a pipe, a
  socket, a process substitution — is refused rather than anchored (D4).
  `design.md` names more cases than are refused, and is left as written
  (*Design drift not reconciled*).
- **A relative socket path in a deep directory can exceed a socket address.**
  That is R-4's failure to bind, and SPEC-003 §6.1 states the remedy.
- **SPEC-001 carries R-60 with no R-59** until slice 012's audit promotes it;
  012's plan owes the design-drift row for this slice's rule change.
- **R-60's §7 row does not cite the `PWD` case or VT-9, and the kit does not
  cite R-60 or R-17** — the person's decisions (*Reconciliation*).

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

Drafted by the auditor. `canon-delta.md`'s wording was endorsed before
design review (its **Status** line) and amended after each code review round
in the person's words; promotion was endorsed separately (`design-log.md` 2026-10-19, *code review round 2, residues, and canon promotion*) and is
applied. Each canon row below is applied as `canon-delta.md` states it,
carrying the F-1, F-3 and F-13 amendments, in evergreen form.

| document | change | reason | done |
|----------|--------|--------|------|
| `specs/001-host-backend-protocol.md` §4 *Process transport* | **Change 1** — add R-60 after R-36, as `canon-delta.md` states it. Amended after code review round 1 (`review-code.md` F-1): the join now states *each `.` component dropped and each `..` kept as written*, in the person's words | R-36 says nothing about where a command runs; the code now runs it in the config file's canonical directory (`Config::load`, `ProcessBackend::exchange`). Code is right; canon is silent. R-60 skips R-59 (slice 012's unpromoted id); 012's plan holds the hand-off row (`docs/slices/012/plan.md` §Owed, *Record slice 015's rule change as design drift*) | [x] applied: R-60 in §4 *Process transport*, the row after R-36's, verbatim. No R-59: SPEC-001 has no convention for marking an unassigned id, so none is marked |
| `specs/001-…` §6.4 *Process transport* | **Change 2** — the *Where the backend runs* paragraph after the first | §6.4's prose carries each transport rule's reason | [x] applied after §6.4's first paragraph, verbatim, with F-13's *is resolved against the backend's working directory* |
| `specs/001-…` §7 | **Change 3** — R-60's row after R-36's. Amended after code review round 1 (`review-code.md` F-3): the directory-link sentence now says a link *leaves the kernel's working directory the same either way*, and the loaded value is what `PWD`, a relative program's name and the socket path are taken from, in the person's words | every requirement names its tests. Each citation resolves (below) | [x] applied after R-36's §7 row, verbatim |
| `specs/003-host-event-ingress.md` §4 | **Change 1** — add R-17 after R-16 (R-17 is SPEC-003's next free id; SPEC-001's own R-17 is a different document's). Amended after code review round 1 (`review-code.md` F-1): R-17 now joins as R-60 does, each `.` dropped and each `..` kept, in the person's words | `ingress_config` anchors `ingress.path` at the config's directory; §6.1 is silent on what a relative path means | [x] applied after R-16, verbatim |
| `specs/003-…` §6.1 | **Change 2** — replace the paragraph's first sentence with the delta's text (it keeps that sentence and extends it) | the rule and the 107-byte limit beside the path's description | [x] applied: the delta's text replaces the first sentence, and the rest of the paragraph (*The mode is owner-only…*) follows it unchanged in wording, reflowed into the one paragraph |
| `specs/003-…` §7 | **Change 3** — R-17's row after R-16's | as Change 3 above. Each citation resolves | [x] applied after R-16's §7 row, verbatim |
| `specs/003-…` §9 | **Change 4** — append R-60 to the SPEC-001 bullet | R-17 leans on R-60's directory | [x] applied: the bullet's closing full stop becomes the delta's *; R-60, …* clause |
| `canon-delta.md` §7 citations (both rows) | **no re-point needed** — verified by the auditor: every cited `fn` exists in the file the row names. SPEC-001 R-60: `working_directory.rs::{a_relative_argument_resolves_beside_the_config_from_another_directory, a_linked_config_runs_its_backend_beside_the_target, a_relative_program_with_a_slash_is_executed_as_the_anchored_path, a_native_program_receives_its_anchored_path_as_its_name, a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for, a_config_reached_through_a_linked_directory_is_anchored_at_the_target}`, `run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config`, `config.rs::tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash`, `examples.rs::each_shipped_example_is_accepted_by_the_checker`. SPEC-003 R-17: `startup.rs::listener::a_relative_configured_path_binds_beside_the_config`, `config.rs::tests::a_relative_path_resolves_against_the_directory_given`, `ingress.rs::a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path`. No phase recorded a rename | `canon-delta.md` preamble: names are `design.md` §9's until audit | [x] re-checked after code review round 2's repairs: every cited `fn` is still defined in the file its row names (`listener::…` inside `startup.rs`' `mod listener`); none re-pointed |
| `canon-delta.md` §7, R-60 row — cases not cited | **decision for the person**: whether R-60's row should also cite `working_directory.rs::the_backend_s_pwd_names_its_working_directory` and `error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable` (VT-9, an addition to §9). Auditor's recommendation: **no** — `PWD` is design, not canon (D12), and VT-9 is the argv form, which R-60 does not govern | completeness of the row against what shipped | [x] the person: **no** extra citations (`design-log.md` 2026-10-19, *code review round 2, residues, and canon promotion*) |
| `kit/skills/goad-backend/reference/running.md`, `events.md` | **decision for the person** (`plan.md` §Owed): once R-60 / R-17 are promoted, cite them in the kit? Today the rule is in prose under the existing `SPEC-001/R-36` heading and the config table; a new `### SPEC-…` heading would change `coverage.rs`' heading set | the kit cites no draft id (PHASE-03 Notes); after promotion the ids exist | [x] the person: **no** kit citation (`design-log.md` 2026-10-19, *code review round 2, residues, and canon promotion*); the kit is unchanged |
| code and repository comments citing `SPEC-001/R-60` / `SPEC-003/R-17` | **becomes true on promotion; no edit**. Cited ahead of promotion in `process.rs` (`ProcessBackend::exchange`), `error.rs` (`BackendError::Spawn`'s doc), `working_directory.rs`' module doc, `round_trip.rs` (`the_readme_s_own_config_loads_and_runs_the_example`'s doc), `examples.rs`' module doc, `exercisers/demo.toml`, `.gitignore`. If promotion is declined, each is a citation of nothing and must change before the merge | `docs/AGENTS.md`: nothing outside the slice cites a draft. These are on `slice-015`, which reaches `main` only in the merge that carries the promotion (`plan.md` §Branch and worktree) | [x] verified on promotion: a tree-wide grep for `R-60` and `SPEC-003/R-17` outside `docs/slices/015/` finds those comments, the two spec rows and their prose, SPEC-003 §9, and `docs/slices/012/plan.md`'s hand-off row — each now names canon text that says what the citation claims |
| `ConfigError::Read`'s doc (`crates/goad-shell/src/error.rs`) and `design.md` D4 / §5.2's error table | **code doc stale → a finding** (code-side, fixable in the slice): both say canonicalization refuses `/dev/stdin` (and `/proc/self/fd/N`). Measured by the auditor: `/dev/stdin` redirected from a regular file canonicalizes to that file, which is then read and anchored at the file's directory; only a pipe, socket or process substitution (`pipe:[…]`) is refused. `design.md` is not edited (*Design drift* below) | the doc states a refusal that does not happen in the common redirect case | [x] repaired, `review-code.md` F-2 (doc comments only) |
| `Config::parse`'s doc | nit: *"is the only caller that need"* → *needs* | PHASE-01 re-measure | [x] repaired, `review-code.md` F-9 |
| `startup.rs::listener::socket_path`'s doc | *"the third such helper in this test binary"* — a count, and now false or at risk with `listener::directory` beside it. Pre-existing text, outside PHASE-01's edit of `listener`; a candidate repair for code review, or a follow-up | *never count* (`CLAUDE.md`) | [x] repaired with the class, `review-code.md` F-4 |
| `running.md` *Name your command* | nit: *"the file and your backend can move together"* — *the file* is the config file, implicit | PHASE-03 re-measure | [x] repaired, `review-code.md` F-9 |

**Checked, not affected** (re-confirmed): SPEC-002; SPEC-004 — a config-read
failure is still `ConfigError::Read` and exit 2 (M4 reds
`exit_codes::an_unreadable_configuration_exits_2_and_says_only_what_its_own_arm_says`),
a spawn failure's status is unchanged; POL-001 — the `check` recipe is
untouched; ADR-001 — `goad-semantics` untouched; ADR-003, ADR-004, ADR-005.
`docs/brief.md`, `README.md` and `docs/roadmap.md` name the demo's files only
and remain true.

**Design drift not reconciled:** `design.md` is left as written. Where the
shipped work departs from it:

- **`ProcessBackend::exchange`'s order.** §5.2's snippet applies `args` and
  then the working-directory block; the code applies the working-directory
  block first, then the existing `args` / stdio chain. Same substance
  (PHASE-02 *Decisions*); the design stands.
- **D4 / §5.2's error table overstate the `/dev/stdin` refusal.** The design
  says a config named by `/dev/stdin`, `<(…)` or `/proc/self/fd/N` is refused
  as `NotFound`. In fact only one that is not a file (a pipe, a socket, a
  process substitution) is; `/dev/stdin` redirected from a file loads and is
  anchored at that file's directory. The code does what D4 chose (no
  fallback); the design's enumeration of the tolerated case is wider than the
  behaviour. The residue carried at close should be phrased by the behaviour.
  The code docs that repeated it are repaired (`review-code.md` F-2);
  `design.md` is not edited.
- **EX-5's copy reason is not uniform.** The design (§5.5, F-8) gives *copy
  this directory out* the reason *the socket and its lock are written beside
  the config, and an installed kit is read-only*. `focus-check` has no
  `[ingress]`, so its README keeps the read-only reason alone (PHASE-03
  *Decisions*). The design's reason is true of the event-driven examples; the
  design stands.
- **VT-9** (`error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable`)
  is not in §9; it was added by the plan to hold `review-design.md` F-19's
  directory-less `Display`.
- **Each README's section heading** *Run it from anywhere else* became *Run
  your own copy* (PHASE-03 *Decisions*); not a design matter, recorded for
  completeness.

## Closure

- [x] All findings dispositioned; no blockers outstanding
- [x] All acceptance criteria met, or explicitly waived by the user
- [x] Each verification criterion in `plan.md` walked against the code, or the gap measured and carried
- [x] Tests and checks green
- [x] Specs / policy / ADRs reconciled, with user endorsement where amended
- [x] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `notes.md` §Open swept against `slice-nnn.md` §Follow-ups; every entry dispositioned
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

## Leads for the code reviewer

Not fixes; places to look. File, symbol, concern.

- `crates/goad-shell/src/error.rs` — `ConfigError::Read`'s doc: claims
  canonicalization refuses `/dev/stdin`; it refuses only a non-file stdin
  (see Reconciliation). Also check `Config::load`'s doc, which repeats *names
  no file in a directory* as a `Read` cause — true only for the A-1 arm and
  the pipe case.
- `crates/goad-shell/src/config.rs` — `Config::parse`'s doc grammar (*need*);
  `tests::a_relative_path_resolves_against_the_directory_given`'s doc has one
  unwrapped over-long line (rustfmt does not wrap comments, so the gate does
  not see it). Same in `startup.rs::listener::directory`'s doc.
- `crates/goad/tests/renderer/startup.rs` — `listener::socket_path`'s doc
  counts helpers (*"the third such helper"*), now beside a new `directory`
  helper in the same module.
- `crates/goad-check/tests/binary/run.rs` — `root`'s directories are never
  removed at the end of either case, where every other new directory helper
  in the slice's cases removes its directory; they accumulate under
  `temp_dir()` (cleared only at the next hand-out of the same case).
- `crates/goad/tests/renderer/startup.rs` —
  `listener::a_relative_configured_path_binds_beside_the_config` checks and
  removes `goad.sock` / `goad.sock.lock` in the test binary's working
  directory (the crate); judge whether any other case in the `renderer`
  binary could bind a relative `goad.sock` there concurrently.
- `crates/goad-emit/tests/binary/exchange.rs` —
  `a_relative_configured_socket_is_found_beside_the_configuration` starts
  `goad-emit` in `temp_dir()` itself; a live `goad.sock` left at the root of
  `temp_dir()` by anything else would be connected to in a regression,
  masking the red. Consider a case-owned start directory.
- `crates/goad-shell/src/backend/process.rs` — `exchange` sets `PWD` to
  `working_directory` as given. Through `Config::load` it is canonical; a
  `Command` built by `Config::parse` with a non-canonical directory (tests do
  this: `round_trip.rs`' `CARGO_MANIFEST_DIR/../../exercisers/typescript`)
  gets a `PWD` with `..` in it. Probably harmless; confirm nothing relies on
  `PWD` being canonical outside `load`.
- `kit/skills/goad-backend/examples/{breadcrumbs,downloads-triage}/README.md`
  — *"goad and `goad-check` find both wherever you start them"*: `goad-check`
  does not use the socket, so *both* overstates for it.
- Kit and canon: a relative `PATH` entry now means a directory under the
  config's directory (`running.md` states it); no test holds it (a stated
  §5.5 residue). Confirm the prose's claim that `PATH` lookup happens after the
  change of directory against what std / tokio actually do.

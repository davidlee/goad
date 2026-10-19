# Plan — Slice 015: a config travels with its backend

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
0** on the phase's final commit, in the slice's worktree. That includes
`cargo test -p goad-semantics`, which this slice does not touch and which
stays in the gate (POL-001).

The anchor comes first, then the spawn that uses it, then the checker's pins
and the kit's prose, then the exercisers and the person's run.

- **PHASE-01 — the anchor.** `Config::load` canonicalizes and anchors;
  `Config::parse(text, directory)`; `Command::working_directory`,
  `Command::executable`, `anchored` (`design.md` §5.2). A relative
  `ingress.path` resolves beside the config, held at every tier that reads it:
  `goad`'s listener, `goad-emit`'s binary and its unit case. The R-4 length
  pin. The directory-link case, which is held on the loaded value.
- **PHASE-02 — the spawn.** `ProcessBackend::exchange` runs the executable in
  the working directory with `PWD` set; `BackendError::Spawn` names the
  executable and the directory (D13). The spawn cases in
  `working_directory.rs`; the TypeScript README's config (D10); the kit
  examples checked from outside their directory (AC-2), with the test docs
  that change makes false.
- **PHASE-03 — the checker's pins, and the kit's prose.** The `PATH` pin and
  the argv-form pin on `goad-check`'s built-binary tier; `running.md`,
  `checking.md`, `events.md`, the example configs' comments, the example
  READMEs and `watch.sh` state the new rule (AC-3's kit half).
- **PHASE-04 — the exercisers, the repository, and the person's run.**
  `exercisers/demo.toml`, the `justfile`, `.gitignore`, `backend.sh`'s header,
  the memory note; the stale root sockets listed for the person; a person runs
  goad from another directory and sees a view (AC-4).

### Owed to audit and close

No phase can do these: they are audit's or close's by `docs/AGENTS.md`.
PHASE-04/VA-4 checks that `notes.md` §Open carries each one as a box.

- **Promote `canon-delta.md`** — SPEC-001 Changes 1–3 (R-60, §6.4, R-60's §7
  row) and SPEC-003 Changes 1–4 (R-17, §6.1, R-17's §7 row, §9) — with the
  person's endorsement, recorded in `audit.md`'s Reconciliation table. After
  slice 012's promotion **by preference, not as a gate**: if 015's audit comes
  first, it promotes anyway, and SPEC-001 carries R-60 with no R-59 until
  012's audit (`design-log.md` 2026-10-19, *design review round 1
  dispositions*). Promotion is a commit on `slice-015`, so canon reaches `main`
  in the same merge as the code.
- **Re-point the §7 test citations** in both rows to the symbols that
  shipped, from the renames the phase sheets record, and confirm each
  resolves (`canon-delta.md` says names are `design.md` §9's until audit).
- **The kit's citation of the new ids.** No phase writes `R-60` or `R-17`
  into `kit/` (PHASE-03 Notes). Once promoted, audit decides with the person
  whether `running.md` and `events.md` cite them.
- **Record the person's run** (PHASE-04/VH-1) in `audit.md` §Evidence, naming
  what was observed. **If any repair from the audit's code review touches
  `goad_shell::config` or the spawn** (`backend::process`, or
  `BackendError::Spawn`), PHASE-04/VH-1 is run again on the repaired build
  before the merge, and §Evidence records that run, not the earlier one.
- **Slice 012's drift row** is already in `docs/slices/012/plan.md` §Owed
  (*Record slice 015's rule change as design drift*). 015's audit names the
  hand-off (`design.md` R2); it does not edit 012's record.
- **The design's residues**, carried as follow-up candidates: `goad`'s own
  `main.rs::start` is held by review and AC-4 only (D11); a config named by
  `/dev/stdin` or a process-substitution path is refused (D4, tolerated); a
  relative `PATH` entry now means the config's directory (§5.5).
- **The Owed box in `notes.md`** — deleting the stale root
  `goad-demo.sock` pair — ticked by the person after the merge.
- **Disposition every other §Open candidate** at close.

**Test names are commitments, and no phase edits `canon-delta.md`.**
`design.md` §9 and `canon-delta.md` name the cases. The delta's wording is the
person's endorsed text, and any change to it returns to them (its **Status**
line); its preamble already assigns re-pointing to audit. So a phase that
ships a case under another name or in another file records the rename in its
phase sheet in `notes.md` — the name §9 gives, the symbol shipped, the file —
and edits neither document. Audit re-points the citations from those records
(§Owed). `design.md` is a record of intent and is not edited either.

### Branch and worktree

The slice runs on its own branch, `slice-015`, in its own worktree, and
reaches `main` in **one merge** carrying code, prose and promoted canon
together (`design.md` R1; `design-log.md` 2026-10-19, *design review round 1
dispositions*). `main` never holds the new rule with the old text, or the
reverse.

```
main ──●(plan accepted)──────────────────────────────────●(merge --no-ff)──►
        \                                               /
slice-015 ●─PHASE-01─PHASE-02─PHASE-03─PHASE-04─audit─close
            (worktree .claude/worktrees/slice-015, one writer at a time)
```

- **Create**, from the main checkout, once the plan is accepted:
  `git worktree add .claude/worktrees/slice-015 -b slice-015`. Every 015
  commit after that — phase work, phase sheets, `notes.md`, audit,
  promotion, close — is made in that worktree. Nothing of 015's is committed on `main` until the
  merge.
- **Each agent prints `git -C .claude/worktrees/slice-015 log -1 --oneline`
  first** and checks it is the previous phase's last commit
  (`docs/memory/subagent-worktrees-can-be-stale.md`). One writer per
  worktree; nobody commits beside a running agent
  (`docs/memory/one-writer-per-worktree.md`).
- **Never `git stash`, never `git checkout`** (the person's rule). To change
  what a checkout has checked out, `git switch`; to work on another branch,
  `git worktree add`. A mutation is applied by copying the file to the
  scratchpad and copying it back, and `git status` is clean after each
  restore.
- **Slice 012's PHASE-10 writes what 015 writes.** Its Surfaces are `kit/`,
  `crates/goad-check/src` and `crates/goad-check/tests/`; 015 writes
  `crates/goad-check/tests/` in PHASE-01 (`tagged.rs`), PHASE-02
  (`examples.rs`, `main.rs`, `tree.rs`, `coverage.rs`) and PHASE-03
  (`run.rs`, `process.rs`), and `kit/` in PHASE-03. So **PHASE-01, PHASE-02
  and PHASE-03 each start by reading slice 012's `notes.md` §Status on
  `main`**; if PHASE-10 is `in progress`, STOP and consult the person.
- **Staying current.** Before PHASE-03 and before the merge, `git merge main`
  inside the worktree, resolve, and run the gate there. A conflict in kit
  prose is resolved in favour of 012's text with 015's rule applied to it. A
  conflict in a test file keeps both sides' cases; where both sides changed
  the same function, STOP and consult the person.
- **The merge**, at close, after the audit's code review holds no open
  blocker, the delta is promoted and AC-4 is recorded on the build being
  merged (§Owed). First read slice 012's
  `notes.md` §Status on `main`: if PHASE-09 or PHASE-10 is `in progress`,
  stop and consult the person — a walk in flight reads the kit and runs the
  binaries, and PHASE-10 also writes the kit (`review-design.md` F-20).
  Otherwise, from the main checkout: `git merge --no-ff slice-015`, then
  `just check` on `main`, then `git worktree remove .claude/worktrees/slice-015`
  and `git branch -d slice-015`. The main checkout's uncommitted `flake.lock`
  is not 015's and is not staged; the merge does not touch it, because
  `slice-015` does not.

## Sequencing & rationale

```
PHASE-01 ──► PHASE-02 ──► PHASE-03 ──► PHASE-04
(anchor:     (spawn:      (checker     (exercisers,
 config,      process,     pins, kit    repo, person's
 ingress)     error,       prose)       run: AC-4)
              AC-2)
```

**Why the anchor is its own phase.** Every later change consumes what
`Config::load` computes. The ingress half (AC-5) needs nothing else, so it is
red-then-green here, against the loader alone, before the spawn exists to
confuse a result. The phase changes `Config::parse`'s signature, which reaches
callers in the goad-shell `integration`, goad `renderer` and goad-check `kit`
targets; doing that once, with nothing else moving, keeps the diff readable.

**Why every case that is red today sits in the phase whose code turns it
green.** Every phase ends with `just check` exiting 0 (`docs/AGENTS.md`
§Execute), so a red case cannot be committed apart from its code; and a case
written after its code never shows its natural red, only a mutation's. So:

- the anchor's reds (the resolution unit case, the executable table, the
  directory-link value case, the listener case, both `goad-emit` cases) are in
  PHASE-01;
- the spawn's reds (the `working_directory.rs` spawn cases, D10's rewritten
  README case, and `examples.rs`' checker started outside each example) are in
  PHASE-02;
- PHASE-03's cases are **pins** — green on arrival — and their evidence is a
  mutation (`design.md` §9 says so for both);
- PHASE-04 has no automated case of its own; its evidence is a grep and a
  person.

**Why the kit's prose waits for PHASE-03, and the exercisers' for PHASE-04.**
The kit is gated by its own tests — every `toml` fence goes through
`Config::parse`, and nothing in it may name a path outside it — so its rewrite
is a phase with a gate, not a tail. The exercisers and the `justfile` are what
the person runs at AC-4, so they land in the phase that runs them. The one
exerciser file that is a test's *input*, the TypeScript README, moves with its
test in PHASE-02 (D10).

**Nothing runs in parallel.** One worktree, one writer. PHASE-03 and PHASE-04
are disjoint by file but share `notes.md`, and PHASE-03 may merge `main`.

**Size.** PHASE-02 is the largest: a small production change, the most new
cases, and most of the mutation table, each mutation an edit, a scoped
test run, a record and a restore. It stays one phase because splitting the
spawn from its error report would commit a spawn whose failures name the wrong
path. If its session runs long, the checkpoint is after EX-3 (the goad-shell
tier green) with `examples.rs` and D10 left; that is a `PARTIAL` commit, not a
re-plan. PHASE-04 is light in tokens and long in wall time (it waits on a
person).

**Natural-red evidence** goes in the phase's sheet too, one row per case
seen red: the case by name, the command, and **the assertion it failed on**.
A case that reds for another reason than the one `design.md` §9's *red today
because* column gives — a missing script where an inherited `PWD` was
claimed — has not shown its red; fix the case's form and run it again.

**Mutation evidence** goes in the phase's sheet in `notes.md`, under a
**Mutation evidence** heading, one row per mutation: the edit (quoted), the
command, that the mutated build **compiled**, the cases that went red **by
name** and on which assertion, and that the restore is green. A mutation that
did not compile is not evidence
(`docs/memory/a-negative-control-that-does-not-compile.md`). Runs use
`--no-fail-fast`. `unused` is denied workspace-wide, so a deletion that
leaves a binding unused does not compile: respell it so the behaviour named
is the one removed, and record the respelling.

The mutations, by the row they hold. Each must red the cases named.

| id | edit (behaviour removed) | must red | phase |
|---|---|---|---|
| M1 | `ingress_config` ignores the directory: the path as written | `config.rs::tests::a_relative_path_resolves_against_the_directory_given`; `startup.rs::listener::a_relative_configured_path_binds_beside_the_config`; `exchange.rs::a_relative_configured_socket_is_found_beside_the_configuration`; `main.rs::tests::a_configured_ingress_path_is_what_is_written_to` | 01 |
| M2 | `anchored` is `directory.join(path)` alone (`.` kept) | `a_relative_path_resolves_against_the_directory_given` (byte-exact) | 01 |
| M3 | `load` anchors at the path's parent as given, not the canonical file's | `a_config_reached_through_a_linked_directory_is_anchored_at_the_target`; in PHASE-02 also `a_linked_config_runs_its_backend_beside_the_target` (the decoy answers) | 01, 02 |
| M4 | `canonicalize`'s failure is a `ConfigError` other than `Read` | `a_dangling_link_to_a_configuration_is_unreadable` | 01 |
| M5 | `ingress::bind` files `UnixListener::bind`'s failure as a `BindFault` other than `Unbindable` | `a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path` (and only the over-long half) | 01 |
| M6 | no `current_dir`: the working directory is not applied (`PWD` still set) | `a_relative_argument_resolves_beside_the_config_from_another_directory`; `a_linked_config_runs_its_backend_beside_the_target`; `examples.rs::each_shipped_example_is_accepted_by_the_checker` | 02 |
| M7 | the platform's route: `Command::new(&self.command.program)` with `current_dir` | `a_relative_program_with_a_slash_is_executed_as_the_anchored_path` (`$0` relative); `a_native_program_receives_its_anchored_path_as_its_name` | 02 |
| M8 | `arg0(&self.command.program)` added to the anchored spawn | `a_native_program_receives_its_anchored_path_as_its_name` only — the script case stays green, which is D5's point | 02 |
| M9 | no `PWD` | `the_backend_s_pwd_names_its_working_directory` | 02 |
| M10 | `Spawn`'s `working_directory` is `None` for a configured command | `a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for` | 02 |
| M13 | `Spawn`'s `Display` prints the parenthesis whatever the directory | `error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable` | 02 |
| M11 | `executable` anchors every relative program, `/` or not | `run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config` (`beside` lines); also `the_executable_is_anchored_only_for_a_relative_path_with_a_slash` | 03 |
| M12 | the argv form is given a working directory other than the caller's | `run.rs::the_argv_form_runs_a_relative_program_from_the_caller_s_directory` | 03 |
| M14 | the spawn fills `Spawn`'s `working_directory` from `current_dir()` for a command with none | `transport.rs::a_command_that_does_not_exist_fails_to_spawn` | 02 |

M11 and M12 mutate PHASE-01 and PHASE-02 code from PHASE-03; that is
permitted under the scratchpad copy-and-restore rule and touches nothing
committed.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-01/VT-1, VT-2, VT-3, VT-6 (the resolution, the executable table, the dangling-link pin, the directory-link value); PHASE-02/VT-1..VT-6 (every spawn case); PHASE-03/VT-1 (`PATH` pin); PHASE-04/VH-1 (through a link, by a person) |
| AC-2 | PHASE-02/VT-8 (the kit examples, checker started outside them); PHASE-02/VT-9 (a `Spawn` with no working directory names none) and VT-10 (the argv form's spawn failure carries none); PHASE-03/VT-2 (argv form) and VA-1 (`checking.md` states both) |
| AC-3 | kit half: PHASE-03/EX-6, VA-1, VA-2, VA-3; canon half: **audit**, by promoting `canon-delta.md` (§Owed) |
| AC-4 | PHASE-04/VH-1; **recorded in `audit.md` §Evidence** |
| AC-5 | PHASE-01/VT-1, VT-4, VT-5 (goad's listener; `goad-emit`'s binary and unit cases); PHASE-04/VH-1 (`just emit` reaches the demo) |

---

## PHASE-01 — the anchor

**Objective:** a config loaded from a file carries its canonical directory,
and a relative `ingress.path` names a socket there for goad and `goad-emit`
alike, wherever each was started.

**Surfaces:**
- `crates/goad-shell/src/config.rs` — `Command` (field, `executable`, its
  doc), `anchored`, `Config::load`, `Config::parse`, `ingress_config`, the
  module's tests.
- `Config::parse`'s callers, the directory argument only:
  `crates/goad-shell/tests/integration/host.rs` (`host`),
  `crates/goad-shell/tests/integration/round_trip.rs`
  (`the_readme_s_own_config_loads_and_runs_the_example` — its `parse` call
  only; D10 is PHASE-02's), `crates/goad/tests/renderer/startup.rs`
  (`unparseable`), `crates/goad-check/tests/kit/tagged.rs` (`check` and its
  callers).
- `crates/goad/tests/renderer/startup.rs` — `listener`, one new case.
- `crates/goad-emit/tests/binary/exchange.rs` — one new case, and a spawn
  helper if one is needed.
- `crates/goad-emit/src/main.rs` — `tests::a_configured_ingress_path_is_what_is_written_to`
  only.
- `crates/goad-shell/tests/integration/ingress.rs` — the length pin.
- `crates/goad-shell/tests/integration/working_directory.rs` (new) and its
  `mod` line in `crates/goad-shell/tests/integration/main.rs`.
- `crates/goad-shell/src/error.rs` — `ConfigError::Read`'s doc only (EX-4).
  PHASE-02 writes this file again for `BackendError::Spawn`; the phases run
  in sequence.
- `tests/support/scripting.rs` — `claim`'s doc only, if a helper this phase
  adds is held by `claim` (Notes).
- `docs/slices/015/notes.md`.

**Entry**
- EN-1 — `plan.md` accepted by the person; the worktree exists on
  `slice-015` at or after the acceptance commit (§Branch and worktree), and
  `git log -1` there was printed and checked.
- EN-2 — `just check` exits 0 in the worktree.
- EN-3 — slice 012's `notes.md` §Status on `main` read; PHASE-10 is not `in
  progress` (§Branch and worktree).

**Exit**
- EX-1 — `Command` has `pub working_directory: Option<PathBuf>`;
  `Command::new` and `Command::from_argv` leave it `None`; its doc says what
  `Some` and `None` mean (`design.md` §5.2).
- EX-2 — `Command::executable` is pure: `anchored(directory, program)` when
  there is a directory and `program` is relative and contains `/`; otherwise
  `program`.
- EX-3 — `anchored` is the one join, used by `executable` and
  `ingress_config`: interior `.` dropped, `..` kept, an absolute path returned
  as is (D7).
- EX-4 — `Config::load` canonicalizes, reads the **canonical** path, and
  calls `Config::parse(&text, parent)`; a `canonicalize` failure is
  `ConfigError::Read`. A-1's impossible arm (no parent) is spelled without a
  panic, as Notes say. `ConfigError::Read`'s doc says what it now carries: a
  file missing or unreadable, a dangling link, and a path that names no file
  in a directory (D4's tolerated case and A-1's arm).
- EX-5 — `Config::parse(text, directory)` sets `working_directory:
  Some(directory)` by struct update over `from_argv`'s value, and anchors
  `ingress.path`. No other path in `Config` is start-directory relative.
- EX-6 — every `Config::parse` caller passes a directory; the config tests
  whose **meaning** changes do so as §9 says
  (`the_design_s_own_example_loads` asserts the working directory;
  `an_ingress_section_loads_with_its_path` asserts the anchored path).
- EX-7 — M1–M5 run and recorded (§Sequencing, *Mutation evidence*); M3's
  PHASE-02 half is PHASE-02's.

**Verification**
- VT-1 — `config.rs::tests::a_relative_path_resolves_against_the_directory_given`:
  `parse(text, "/anchor")` gives `ingress.path` byte-exact `/anchor/goad.sock`
  for `./goad.sock`, keeps an absolute one, and `working_directory` is
  `/anchor`. Seen red first.
- VT-2 — `config.rs::tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash`:
  the table `./b.sh`, `bin/b`, `../b`, `/abs/b`, `python3`, and `./b.sh` with
  `None`. Seen red first (no `executable`).
- VT-3 — `config.rs::tests::a_dangling_link_to_a_configuration_is_unreadable`:
  a pin, green on arrival; M4 reds it.
- VT-4 — `startup.rs::listener::a_relative_configured_path_binds_beside_the_config`:
  `Config::load` of a temp config naming `./goad.sock`, then `listener`; a
  socket at the canonical config directory, **none** in the test's working
  directory. Seen red first.
- VT-5 — `exchange.rs::a_relative_configured_socket_is_found_beside_the_configuration`:
  `XDG_CONFIG_HOME` set on the `goad-emit` child to a temp root whose
  `goad/config.toml` names `./goad.sock`; a fake listener at the config's
  canonical directory; `goad-emit` started in another directory without
  `--socket`; exit 0 and the envelope received. And
  `main.rs::tests::a_configured_ingress_path_is_what_is_written_to` expects
  the canonical config directory joined with `from-config.sock`. Both seen
  red first.
- VT-6 — `working_directory.rs::a_config_reached_through_a_linked_directory_is_anchored_at_the_target`:
  `<t>/link → <t>/real`; `Config::load(<t>/link/config.toml)` has
  `working_directory == Some(<t>/real)` (canonical) and `ingress.path` under
  it. Seen red first.
- VT-7 — `ingress.rs::a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path`:
  a 108-byte path is `BindFault::Unbindable` over `ErrorKind::InvalidInput`
  and the error names the path; a 107-byte path in the same directory binds.
  std's wording is not asserted. A pin; M5 reds it.
- VA-1 — `transport_shape.rs`'s guards and every case in §9's *unaffected*
  list are untouched and green.
- VA-2 — `git status` in the worktree after the red runs shows no socket,
  lock or config written into a crate directory (R5).

**Notes for the implementer**
- **Red first, and record it.** VT-1, VT-2 and VT-6 cannot compile before
  the API exists: their red is the first compile that *does* — with
  `executable` returning `program` unchanged, `parse` ignoring the directory,
  `load` not canonicalizing — run and seen failing on the assertion, before
  the real body. VT-4 and VT-5 are red against today's code once
  `Config::parse` has its new signature. Record each red, with the assertion
  it failed on (§Sequencing, *Natural-red evidence*).
- **A-1's spelling.** `canonical.parent()` is `Some` for any readable file.
  Spell the `None` arm as `ConfigError::Read` over an `io::Error` of kind
  `InvalidInput` saying the path names no file in a directory: total, no
  panic (the no-panic lints are crate-wide), no new variant, and the same
  exit 2 as an unreadable file. Not `unwrap_or(&canonical)`, which would
  anchor at the file.
- **`anchored` and the empty remainder.** `Path::join("")` appends a
  separator. Decide what `"."` alone becomes and pin it in VT-1's table or
  say in the sheet why it cannot arise.
- **Socket paths live under `std::env::temp_dir()`**, never a crate
  directory, and carry the pid last. **VT-7's precondition:** compute
  `temp_dir()`'s length; if a 107-byte path will not fit under it, the case
  **fails** naming the precondition and the length found — it never skips,
  and never `#[ignore]`s (`review-design.md` F-14; Synthesis says a jail with
  a long `TMPDIR` gates red, by design). Clean up the socket **and** its lock
  for both halves: R-5 leaves both (`goad_shell::ingress::lock_path`).
- **Canonical, not as built.** `temp_dir()` may itself sit behind a link.
  Every "at the config's directory" assertion compares against
  `std::fs::canonicalize` of that directory, never the path the test built.
- **Temp directories, and `claim`.** No `tempfile` (not on the manifest
  allowlist; a dependency addition is a STOP). A case needs a directory of
  its own: write one local helper per test file, pid-qualified, cleared at
  handout, and held by `scripting::claim` under a kind of its own in the
  targets that include `tests/support/scripting.rs` (goad-shell
  `integration`, goad `renderer`). A shared helper in `scripting.rs` itself
  would be dead code in `event_loop_schedule`, which includes the file and
  would not use it. `claim`'s doc enumerates the helpers of its class and
  **counts** them, and the count is already short: run `claim`'s own grep
  (`grep -rn 'process::id()' --include=*.rs . | grep -v '^./target'`) and rewrite the sentence to
  **name** every helper of the class it finds, held and unheld — among them
  `crates/goad-check/tests/kit/examples.rs::handout` and
  `crates/goad/tests/binary/exit_codes.rs::scratch_config`, which it misses
  today, and this slice's new ones — with no number (`CLAUDE.md`, *never
  count*). Keep `claim`'s *One kind per helper* rule: a new kind's path is
  checked against `marker`'s bare `goad-<name>-<pid>`, so a directory helper
  ends its name with a non-digit suffix after the pid (as a socket's `.sock`
  does) and can never equal a marker path. `goad-emit`'s target does not
  include the file; its new helper follows `exchange.rs`'s own `socket_path`
  precedent.
- **The listener case and R5.** Before the red run, the case asserts no
  `goad.sock` and no `goad.sock.lock` is in the test's working directory and
  removes any it finds, and it removes both again at its end: the red run
  *will* bind there, and `ingress::bind` opens the lock before it binds
  (T2-8), so the lock — the file `git status` lists (T2-S7) — is written even
  when the bind is what the case is about. `listener::cleanup` removes both;
  use it.
- **`goad-emit`'s binary case asserts the exit status before it joins the
  fake listener.** `exchange.rs::fake_listener` blocks in `accept`; in the
  red run nothing connects at the config's directory, so a `join()` first
  would hang the case instead of failing it. Join only after the status is 0.
- **`goad-emit`'s binary case** sets `XDG_CONFIG_HOME` and `current_dir` on
  the child only (`Command::env`, `Command::current_dir`). `std::env::set_var`
  is disallowed by `clippy.toml`, and process-global mutation is the reason.
  Set `XDG_CONFIG_HOME` on the child explicitly; the inherited one would
  otherwise name the person's own config.
- **`tagged.rs::check`** gains the Markdown file's directory; its synthetic
  cases pass any absolute directory — `parse` never touches the filesystem.
- **`config.rs`'s tests module stays last** in the file: an item after it
  fails clippy's `items_after_test_module` under the gate's `-D warnings`.
- STOP if the anchored `ingress.path` changes any case in §9's *unaffected*
  list, or if any caller outside T2-6's list turns up.

---

## PHASE-02 — the spawn

**Objective:** a configured backend runs in its config's directory with `PWD`
naming it, a relative program with a `/` is executed by its anchored path,
and a spawn failure names where the program was looked for.

**Surfaces:**
- `crates/goad-shell/src/backend/process.rs` — `ProcessBackend::exchange`
  only.
- `crates/goad-shell/src/error.rs` — `BackendError::Spawn`'s shape, its
  `Display`, `source`, `requirement`, `fault` arms, and its tests.
- `BackendError::Spawn`'s other sites:
  `crates/goad-shell/tests/integration/transport.rs`
  (`a_command_that_does_not_exist_fails_to_spawn`),
  `crates/goad-shell/tests/integration/failure_matrix.rs`
  (`a_command_that_cannot_be_spawned_reaches_the_caller_as_a_spawn_failure`),
  `crates/goad/tests/renderer/table.rs`,
  `crates/goad-check/tests/kit/coverage.rs`.
- `crates/goad-shell/tests/integration/working_directory.rs` — the spawn
  cases.
- `crates/goad-shell/tests/integration/round_trip.rs` —
  `the_readme_s_own_config_loads_and_runs_the_example`, its doc, and
  `rooted_at_the_workspace` (deleted); `exercisers/typescript/README.md`
  (D10).
- `crates/goad-check/tests/kit/examples.rs` (`run_checker` and its doc, the
  module doc), `crates/goad-check/tests/kit/main.rs` (module doc),
  `crates/goad-check/tests/kit/tree.rs` (`files`' doc).
- `docs/slices/015/notes.md`.

**Entry**
- EN-1 — PHASE-01 done; `git log -1` in the worktree is its last commit.
- EN-2 — slice 012's `notes.md` §Status on `main` read; PHASE-10 is not `in
  progress` (§Branch and worktree).

**Exit**
- EX-1 — `exchange` builds `Command::new(&executable)` from
  `self.command.executable()`, passes `arguments` as written, and when a
  working directory is set applies `current_dir` and `env("PWD", …)`. No
  `arg0`. The line `let mut command = Command::new(` survives, and
  `command.spawn()` is still the file's only `spawn` (`transport_shape.rs`).
- EX-2 — `BackendError::Spawn { executable, working_directory, source }`;
  `Display` is `backend could not be spawned: {executable} (in
  {working_directory}): {source}`, the parenthesis only when a directory is
  set; `source()` returns `source`; `requirement()` is still R-44 and
  `fault()` still `AtFault::Configuration`. Every site in Surfaces matches or
  builds the new shape, and the prefix `table.rs` asserts is unchanged.
- EX-3 — the `working_directory.rs` spawn cases (VT-1..VT-6) green, each seen
  red first.
- EX-4 — D10: the README's config is `["deno", "run", "-A", "backend.ts"]`,
  its prose says the file is saved beside `backend.ts` and that a relative
  path resolves against the config file's directory;
  `rooted_at_the_workspace` is gone; the case parses with
  `exercisers/typescript` as the directory and runs the README's config
  unrewritten.
- EX-5 — AC-2: `run_checker` starts the checker in the hand-out **root**
  with `--config example/config.toml` and `--event example/events/…`; the
  test docs that described starting in the copy are true of the new start.
- EX-6 — M3's PHASE-02 half, M6–M10 and M13 run and recorded.

**Verification**
- VT-1 — `a_relative_argument_resolves_beside_the_config_from_another_directory`:
  `["bash", "backend.sh"]` loaded by `Config::load` answers; its logged
  `pwd -P` is the canonical config directory and its `$0` is `backend.sh`.
- VT-2 — `a_linked_config_runs_its_backend_beside_the_target`: `Config::load`
  over a link to the file; the target directory's backend answers; a decoy
  `backend.sh` beside the link answers differently and is not the one run.
- VT-3 — `a_relative_program_with_a_slash_is_executed_as_the_anchored_path`:
  `["./backend.sh"]` (shebang) answers; its `$0` is `<canonical
  anchor>/backend.sh`.
- VT-4 — `a_native_program_receives_its_anchored_path_as_its_name`: a copy of
  the devshell's `sh` beside the config as `shell`; `["./shell", "-c", …]`
  logs `$0`, which is `<canonical anchor>/shell`.
- VT-5 — `the_backend_s_pwd_names_its_working_directory`: a config loaded
  with `Config::load` whose `command` is `["python3", "-c", <body>, <log>]` —
  no relative path in it, so against PHASE-01's tree the backend runs, and
  the case reds on the `PWD` assertion because `PWD` is inherited from the
  test process (`design.md` §9). The body answers conformingly and logs
  `os.environ["PWD"]`; it is the canonical config directory.
- VT-6 — `a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for`:
  `["./missing.sh"]` gives `Spawn` with the anchored `executable`, the anchor
  as `working_directory`, and a `Display` containing both.
- VT-7 — `round_trip.rs::the_readme_s_own_config_loads_and_runs_the_example`
  green on the README's own text; seen red with `rooted_at_the_workspace`
  deleted and the spawn unchanged.
- VT-8 — `examples.rs::each_shipped_example_is_accepted_by_the_checker` and
  `::downloads_triage_moves_the_file_it_was_asked_about` green started from
  the root; seen red against the spawn before EX-1.
- VT-9 — `error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable`:
  a `Spawn` with `working_directory: None` (the argv form's) has a `Display`
  naming the executable and containing no `(in `. Seen red against a
  `Display` that prints the parenthesis whatever the directory; M13 reds it.
  Not a §9 name: recorded in the sheet as an addition, for audit.
- VT-10 — `transport.rs::a_command_that_does_not_exist_fails_to_spawn`
  also asserts the `Spawn` it reaches carries `working_directory: None`, for
  a command with none (the argv form's shape): the spawn fills nothing in from
  the caller. M14 reds it. Settles `review-plan.md` F-15.
- VA-1 — `transport.rs`'s and `failure_matrix.rs`' spawn cases still assert
  what they asserted (`NotFound`; the configuration's side); only the match
  shape changed.

**Notes for the implementer**
- **Order.** Write VT-1..VT-6 and change VT-7's and VT-8's tests first; run
  them against PHASE-01's tree, where the spawn ignores the working
  directory, and record each red with the assertion it failed on
  (§Sequencing, *Natural-red evidence*). VT-6 needs the new `Spawn` shape to
  compile: change `error.rs` and its sites first with `process.rs` filling
  `executable` from the program as written and `working_directory: None`,
  see VT-6 red, then do EX-1.
- **Every spawn case loads its config from a file with `Config::load`**, in a
  temp directory of its own, and the test process's working directory (the
  crate) is never the config's (canon-delta R-60 row's opening).
- **The native-program case** (VT-4): resolve `sh` at test time by running
  `sh -c 'command -v sh'` and canonicalizing the answer, so the copy is the
  real binary, not a link; `std::fs::copy` keeps the mode. For `sh -c` with no
  operand, `$0` is argv[0] (`research.md` T2-S2).
- **The `PWD` case uses `python3`**, which exposes an inherited `PWD` as is;
  a shell would repair a stale one and hide the regression
  (`review-design.md` F-7).
- **Fixtures are written by the case**, not committed: a script body in the
  test file, written beside the config and made executable. Logging scripts
  write to a marker path passed as an argument, as `scripting::logging_backend`
  does; a backend that must answer conformingly answers as
  `tests/backends/` scripts do.
- **Negative controls compile.** The decoy in VT-2 is a real script in the
  same case, run-able, answering differently — never a commented-out variant.
- **D10's README** is the test's input: the prose and the fence change in
  the commit that changes the case. Keep the README's *host exerciser, not a
  template* framing.
- **`examples.rs`**: `downloads_triage_moves_the_file_it_was_asked_about`
  calls `run_checker` too. Events are read by `goad-check` relative to its own
  working directory, so `--event` paths gain the `example/` prefix; only
  `--config`'s directory moves the backend.
- **`coverage.rs`** builds a `Spawn` to enumerate kinds; give it a
  directory-less value unless the case needs one.
- **No new identifier in `process.rs`'s code contains `spawn`** in lower case
  (`transport_shape.rs::the_only_spawn_is_the_child` counts the token; strings
  and comments are stripped first).
- STOP if `ProcessBackend::new`'s signature or either binary's `main.rs`
  would need to change (`design.md` §5.2, T2-4), or if any test in §9's
  *unaffected* list reds.

---

## PHASE-03 — the checker's pins, and the kit's prose

**Objective:** `goad-check`'s built binary is pinned to find a bare program on
`PATH` and to run the argv form from the caller's directory, and the kit
states the new rule with no *make every path absolute* advice.

**Surfaces:**
- `crates/goad-check/tests/binary/run.rs` — the pins VT-1 and VT-2;
  `crates/goad-check/tests/binary/process.rs` — a spawn helper taking an
  environment and a working directory, if the cases need one.
- `tests/support/scripting.rs` — `claim`'s doc only, if a helper here is
  held by `claim`.
- `kit/skills/goad-backend/reference/running.md` (*Name your command* —
  SPEC-001/R-36's section — and the config table's `ingress.path` row),
  `reference/checking.md` (the relative-path paragraph, and the argv form's
  rule, OQ-5), `reference/events.md` (`goad-emit`'s socket rule and its
  `--socket ./goad.sock` example).
- `kit/skills/goad-backend/examples/{breadcrumbs,downloads-triage,focus-check}/config.toml`
  — the opening comment only (*relative to this directory: run goad and
  goad-check from here*); no value changes (`research.md` T2-11).
- `kit/skills/goad-backend/examples/{breadcrumbs,downloads-triage,focus-check}/README.md`,
  including each README's `toml goad:config` fence, which quotes its
  `config.toml` and its comment verbatim; `examples/downloads-triage/watch.sh`
  (header), and
  `examples/breadcrumbs/chpwd.zsh` (header) only if VA-2 classes it stale.
- `docs/slices/015/notes.md`.

**Entry**
- EN-1 — PHASE-02 done; `git log -1` checked.
- EN-2 — `main` merged into the worktree and the gate green (§Branch and
  worktree), so the kit is edited on top of whatever 012 has landed. If 012's
  PHASE-10 is `in progress` on `main`, STOP and consult the person: it is a
  writer of these files.

**Exit**
- EX-1 — `run.rs` holds the pins VT-1 and VT-2; M11 and M12 run and
  recorded.
- EX-2 — `running.md` says: the backend runs in the config file's directory,
  through any links; arguments are passed as written; a relative program with
  a `/` is found beside the config and receives that path as its name; a
  bare name is looked up on `PATH`, after the change of directory, so a
  relative `PATH` entry names a directory under it; a spawn failure names the
  path tried and the directory. Its `ingress.path` row says relative to the
  config file's directory, and states the socket-path limit (107 bytes on
  Linux) and the remedy (an absolute path somewhere short, such as
  `$XDG_RUNTIME_DIR`). It says the rule changed.
- EX-3 — `checking.md` says `--config` runs the backend as goad will, from
  any directory; the argv form runs from the caller's directory (OQ-5).
- EX-4 — `events.md`: `goad-emit` without `--socket` resolves a relative
  `ingress.path` as goad does; its `--socket` path is used as given, relative
  to where `goad-emit` runs.
- EX-5 — each example README keeps *copy this directory out*, now for its
  reason: the socket and its lock are written beside the config, and an
  installed kit is read-only (`design.md` §5.5, F-8). *Run goad from this
  directory* and *make both absolute* are gone; a copy needs no edit to its
  config. `watch.sh`'s header is true from any directory.
- EX-6 — each example `config.toml`'s opening comment states the new rule —
  `command` and the socket are relative to this file's directory, from
  wherever goad or `goad-check` is started — and no longer says *run … from
  here*; each README's `goad:config` fence quotes its `config.toml` as the
  file now reads, comment included.

**Verification**
- VT-1 — `run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config`,
  per `design.md` §9's row: under the case's temp root, `config/config.toml`
  with `command = ["goad-decoy", <log>]`; `config/goad-decoy` and
  `path/goad-decoy`, each `#!/bin/sh` using builtins only, each appending its
  own label and answering conformingly; `goad-check --config
  config/config.toml` run with `PATH` = `<root>/path` on that child alone and
  its working directory `<root>`. The log has at least one line and every
  line is `from-path`. A pin; M11 reds it.
- VT-2 — `run.rs::the_argv_form_runs_a_relative_program_from_the_caller_s_directory`:
  `goad-check -- ./backend.sh` started in a directory holding it is judged. A
  pin; M12 reds it.
- VT-3 — the kit tier green: `tagged.rs::every_json_and_toml_fence_in_the_kit_is_tagged_and_checked`
  over every edited fence, `standalone.rs::nothing_in_the_kit_names_a_path_outside_it`,
  and `examples.rs`.
- VA-1 — AC-3's kit half: `git grep -n -i 'absolute\|started in\|working
  directory\|from this directory\|from here\|relative to this directory\|run
  goad' -- kit` recorded in the sheet, each hit classed as true under the new
  rule or repaired. Checked against today's text, the pattern matches each
  example `config.toml`'s comment and each README fence's copy of it (`relative
  to this directory`, `from here`). No hit advises making a path absolute,
  or running from a directory, to work around where goad was started.
- VA-3 — each README's `goad:config` fence is byte-identical to its
  `config.toml` (a `diff` of the fence body against the file, recorded).
- VA-2 — `chpwd.zsh`'s header (*add `--socket` with the absolute path*) is
  classed: `goad-emit`'s `--socket` is used as given, so the advice may
  still be true; recorded either way.

**Notes for the implementer**
- **Cite no draft id in the kit.** `R-60` and `R-17` are not canon until
  audit, and nothing outside the slice may cite a draft (`docs/AGENTS.md`).
  State the rule in prose under the existing `SPEC-001/R-36` heading and the
  config table; audit decides on citing the ids after promotion (§Owed). No
  new `### SPEC-…` heading: `coverage.rs` counts headings by id.
- **The `PATH` case is on the built-binary tier on purpose**: `PATH` reaches
  the backend only through `Command::env` on the `goad-check` child, so the
  test process's environment cannot decide it (`review-design.md` F-5). The
  decoys use builtins only (`read`, `printf`, redirection) because the `PATH`
  they run under holds nothing else; `#!/bin/sh` is resolved by the kernel,
  not by `PATH`. "Answering conformingly" means each answers every kind the
  checker sends with a response it accepts, as `tests/backends/` scripts do;
  the assertion is on the log, not on the verdict.
- **The temp root** for both cases: one local helper in `run.rs`, held by
  `scripting::claim` (this target includes `scripting.rs`), and `claim`'s doc
  updated by name, as PHASE-01 Notes say.
- **Kit prose is product text.** Short, second person, as the reference
  already reads. Every `toml` fence keeps its `goad:config` tag and must
  parse; nothing may name a path outside `kit/`.
- STOP if a kit file's merge with 012's text leaves a rule ambiguous; the
  person decides.

---

## PHASE-04 — the exercisers, the repository, and the person's run

**Objective:** the demo runs from any directory with its socket in
`exercisers/`, nothing in the repository states the old rule, and a person has
seen it.

**Surfaces:**
- `exercisers/demo.toml` — `command`, `[ingress] path`, and the comments
  naming the socket or the old rule (D9).
- `exercisers/shell/backend.sh` — header only.
- `justfile` — `emit`'s `--socket`, and the `package` comment's sentence
  on what a `path:` copy meets in the checkout.
- `.gitignore` — the demo entries (`/exercisers/goad-demo.sock`,
  `/exercisers/goad-demo.sock.lock` replace the root pair) and the
  kit-example paragraph's reason (F-9).
- `docs/memory/path-flake-ref-breaks-on-demo-socket.md` — the socket's
  location and the general form's wording; its conclusion stands (X-4).
- `crates/goad-shell/tests/integration/round_trip.rs` — written by
  PHASE-01 and PHASE-02 as well; here `shell_example`'s doc only, which says its argv is the one `demo.toml` names and runs the
  exerciser exactly as the demo does; after EX-1 neither holds as written.
- `docs/slices/015/notes.md` (including §Owed and §Open).

**Entry**
- EN-1 — PHASE-03 done; `git log -1` checked.

**Exit**
- EX-1 — `demo.toml`: `command = ["bash", "shell/backend.sh"]`, `path =
  "./goad-demo.sock"`, every comment true from any directory (the `socat`
  and `nc` lines name the socket by a path that works from the repository
  root, or say where they are run from).
- EX-2 — `justfile` `emit` passes `--socket exercisers/goad-demo.sock`; the
  `package` comment names `exercisers/goad-demo.sock` and the kit examples'
  `goad.sock` files, which break a `path:` copy the same way, and says
  `.claude/worktrees/` holds gitignored worktrees, with no number. `just -n
  check` prints the same sequence as before (the gate recipe is not touched).
- EX-3 — `.gitignore` as Surfaces say; its comments give the new rule's
  reason.
- EX-4 — `backend.sh`'s header quotes the demo's new argv.
- EX-5 — the memory note says where the socket now lives and that a relative
  socket path resolves against the config file's directory.
- EX-6 — `notes.md` §Owed's box names which checkouts hold a stale root
  `goad-demo.sock` pair (VA-3); the person deletes them after the merge.
- EX-7 — `round_trip.rs::shell_example`'s doc is true of the new demo: what
  it shares with `demo.toml` (`bash` and the script) and what it does not
  (the working directory, an absolute script path).

**Verification**
- VA-1 — `git grep -n 'demo.toml\|goad-demo.sock\|shell/backend.sh' --
  ':!docs/slices'`, which reaches `crates/` for every claim about the demo's
  argv or socket, and `git grep -n -i 'working directory\|started in\|repository
  root' -- exercisers justfile .gitignore
  docs/memory/path-flake-ref-breaks-on-demo-socket.md`, recorded, each hit
  classed. (`crates/goad-check/src/args.rs` `DEFAULT_TIMEOUT`'s doc cites
  `demo.toml`'s timeout and stays true.)
- VA-2 — every case named in `design.md` §9 and in `canon-delta.md`'s §7
  rows resolves to a test in the tree (`cargo test -- --list` per target, or
  a grep for each `fn` name), or to a rename a phase sheet records; the list
  of renames is gathered in the sheet for audit's re-pointing.
- VA-3 — the checkouts holding a root `goad-demo.sock` or
  `goad-demo.sock.lock` listed in §Owed: the main checkout, and each path in
  `git worktree list`.
- VA-4 — `notes.md` §Open carries every item of §Owed to audit and close
  above as a box.
- VH-1 — AC-4, a person, from the worktree's build, in the dev shell (the
  person's shell is nushell; the hand-over gives the commands in its syntax,
  pasted in full, not as a pointer):
  1. from `/tmp`, start goad on `<worktree>/exercisers/demo.toml` by its
     absolute path; a view appears; **quit it** — both runs bind the same
     anchored `exercisers/goad-demo.sock`, and a second host on a live one is
     refused (SPEC-003/R-3);
  2. from `/tmp`, start goad through a link to it (`/tmp/goad-015-demo.toml →
     <worktree>/exercisers/demo.toml`); a view appears; leave it running;
  3. from the worktree root, `just emit reddit-watcher reddit-opened` reaches
     the host started in step 2 (AC-5);
  4. `exercisers/goad-demo.sock` exists and the root has no new socket; then
     quit the host.
  What was seen is recorded in the sheet, for audit's §Evidence.

**Notes for the implementer**
- `cargo run` keeps the caller's working directory, so `cargo run
  --manifest-path <worktree>/Cargo.toml -p goad -- <worktree>/exercisers/demo.toml`
  from `/tmp` is the run; the GUI needs the dev shell's `LD_LIBRARY_PATH` and
  `FONTCONFIG_FILE`. The person launches it; an agent does not.
- Do not edit `docs/brief.md`, closed slices, or slice 012's record.
- STOP if the person's run shows anything §9 did not predict.

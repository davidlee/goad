# Notes — Slice 015

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | done | 2026-10-19 |
| PHASE-02 | done | 2026-10-19 |
| PHASE-03 | done | 2026-10-19 |
| PHASE-04 | done | 2026-10-19 |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — the anchor

Written by the orchestrator, 2026-10-19, at `c68a624`; executed by a separate
agent. The sheet is the plan's second reading: where it restates a plan
criterion it quotes it; where it narrows one it says so.

**Objective** (`plan.md` PHASE-01, quoted): *"a config loaded from a file
carries its canonical directory, and a relative `ingress.path` names a socket
there for goad and `goad-emit` alike, wherever each was started."*

**Reading list**
- `plan.md` §Overview, §Sequencing & rationale (*Natural-red evidence*,
  *Mutation evidence*, the M-table), §PHASE-01 entire.
- `design.md` §5.2 (interfaces — the `Command`, `executable`, `anchored`,
  `load`, `parse` signatures are binding), §5.5 (I-1, I-3, A-1, *Edge — long
  socket path*), §7 D3, D4, D7, D8, §9 (the PHASE-01 rows, *Existing tests
  changed*, the *unaffected* list via `research.md` T2-9).
- `research.md` T2-1, T2-6, T2-7, T2-8, T2-9, T2-S5 (107-byte `sun_path`),
  T2-S7 (the lock shows in `git status`).
- Code: `crates/goad-shell/src/config.rs` (`Command`, `Config::load`,
  `Config::parse`, `ingress_config`, `mod tests`); `crates/goad-shell/src/error.rs`
  (`ConfigError::Read`); `crates/goad/tests/renderer/startup.rs`
  (`unparseable`, `mod listener`, `listener::cleanup`);
  `crates/goad-emit/tests/binary/exchange.rs` (`socket_path`, `cleanup`,
  `fake_listener`, `emit`); `crates/goad-emit/src/main.rs`
  (`socket_path`, `tests::a_configured_ingress_path_is_what_is_written_to`);
  `crates/goad-shell/tests/integration/ingress.rs`;
  `tests/support/scripting.rs` (`claim` and its doc).
- Memory: `docs/memory/` — negative controls must compile; tests asserting
  proxies; a standing guard may not reach a new file.

**Surfaces — a closed list.** Anything outside it is a STOP.
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
- `crates/goad-emit/src/main.rs` —
  `tests::a_configured_ingress_path_is_what_is_written_to` only.
- `crates/goad-shell/tests/integration/ingress.rs` — the length pin.
- `crates/goad-shell/tests/integration/working_directory.rs` (new) and its
  `mod` line in `crates/goad-shell/tests/integration/main.rs`.
- `crates/goad-shell/src/error.rs` — `ConfigError::Read`'s doc only.
- `tests/support/scripting.rs` — `claim`'s doc only.
- `docs/slices/015/notes.md` — this sheet, §Status, §Harvest.

**Entry** — checked by the orchestrator before hand-over:
- EN-1 ✔ plan accepted (`plan-log.md` 2026-10-19, *the last findings, and
  acceptance*); worktree on `slice-015` at `c68a624`.
- EN-2 ✔ `just check` in the worktree at `c68a624`: exit 0 (orchestrator,
  2026-10-19; `TMPDIR` unset, so `temp_dir()` is `/tmp`).
- EN-3 ✔ slice 012 `notes.md` §Status on `main`: PHASE-10 `pending`.

**Assumptions**
- *Verified at plan* (`research.md`, re-checked by the orchestrator at
  `c68a624`): `Config::parse`'s callers outside `config.rs` are exactly
  T2-6's four (`host.rs::host`, `round_trip.rs`, `startup.rs::unparseable`,
  `tagged.rs::check`); `Config::load`'s callers are `goad`, `goad-check` and
  `goad-emit` `main.rs`, none of which this phase edits — they get the anchor
  through `load`. `ingress::bind` opens the lock before it binds (T2-8).
  `BindFault::Unbindable` already documents the over-long path.
- *First tested here:* that a 108-byte path reaches `UnixListener::bind` and
  comes back `InvalidInput` (not truncated, not another kind) on this
  platform's std; that `temp_dir()` leaves room for a 107-byte control in the
  gate's environment (a jail's long `TMPDIR` would turn VT-7 red **by
  design** — record the length found either way); that the anchored
  `ingress.path` changes no case in the *unaffected* list.

**STOP conditions** — stop, write *where I stopped* here, and return:
- a file or symbol outside Surfaces needs to change;
- a `Config::parse` caller outside T2-6's list turns up;
- the anchored `ingress.path` changes any case in T2-9's *unaffected* list;
- a dependency would be added (no `tempfile`: not on the manifest allowlist);
- anything in `design.md` §5.2 does not fit the code as found.

**Exit criteria** (quoted from `plan.md`; none narrowed)
- EX-1 — *"`Command` has `pub working_directory: Option<PathBuf>`;
  `Command::new` and `Command::from_argv` leave it `None`; its doc says what
  `Some` and `None` mean (`design.md` §5.2)."*
- EX-2 — *"`Command::executable` is pure: `anchored(directory, program)` when
  there is a directory and `program` is relative and contains `/`; otherwise
  `program`."*
- EX-3 — *"`anchored` is the one join, used by `executable` and
  `ingress_config`: interior `.` dropped, `..` kept, an absolute path returned
  as is (D7)."*
- EX-4 — *"`Config::load` canonicalizes, reads the **canonical** path, and
  calls `Config::parse(&text, parent)`; a `canonicalize` failure is
  `ConfigError::Read`. A-1's impossible arm (no parent) is spelled without a
  panic, as Notes say. `ConfigError::Read`'s doc says what it now carries: a
  file missing or unreadable, a dangling link, and a path that names no file
  in a directory (D4's tolerated case and A-1's arm)."*
- EX-5 — *"`Config::parse(text, directory)` sets `working_directory:
  Some(directory)` by struct update over `from_argv`'s value, and anchors
  `ingress.path`. No other path in `Config` is start-directory relative."*
- EX-6 — *"every `Config::parse` caller passes a directory; the config tests
  whose **meaning** changes do so as §9 says
  (`the_design_s_own_example_loads` asserts the working directory;
  `an_ingress_section_loads_with_its_path` asserts the anchored path)."*
- EX-7 — *"M1–M5 run and recorded … M3's PHASE-02 half is PHASE-02's."*
- And `just check` exits 0 on the phase's final commit.

**Notes for the implementer** — `plan.md`'s, lifted in; read every one.
1. **Red first, and record it.** VT-1, VT-2, VT-6 cannot compile before the
   API exists: add the API as **stubs** first — `executable` returning
   `program` unchanged, `parse` ignoring the directory (`working_directory:
   None`, `ingress.path` as written), `load` not canonicalizing — compile,
   run, and see each fail **on its assertion**. Then the real bodies. VT-4 and
   VT-5 are red against today's code once `parse` has its new signature.
2. **A-1's spelling.** `canonical.parent()` `None` arm →
   `ConfigError::Read(io::Error::new(ErrorKind::InvalidInput, "… names no
   file in a directory"))`. Total, no panic (no-panic lints are crate-wide),
   no new variant, exit 2. **Not** `unwrap_or(&canonical)`.
3. **`anchored` and the empty remainder.** `Path::join("")` appends a
   separator. Decide what `"."` alone (and `"./"`) becomes and pin it in
   VT-1's table, or record here why it cannot arise. `executable` never sees
   it (no `/` in `"."`); `ingress.path = "."` is non-empty and passes
   `ingress_config`, so it *can* arise there — the decision is yours, within
   D7; record it under *Decisions*.
4. **Socket paths under `std::env::temp_dir()`**, never a crate directory,
   pid last. **VT-7's precondition:** compute `temp_dir()`'s length; if a
   107-byte path will not fit, the case **fails** naming the precondition and
   the length found — never skips, never `#[ignore]`. Clean up socket **and**
   lock for both halves (`goad_shell::ingress::lock_path`).
5. **Canonical, not as built.** Every "at the config's directory" assertion
   compares against `std::fs::canonicalize` of that directory.
6. **Temp directories, and `claim`.** One local helper per test file,
   pid-qualified, cleared at handout, held by `scripting::claim` under a kind
   of its own in targets that include `tests/support/scripting.rs` (goad-shell
   `integration`, goad `renderer`). Not a shared helper in `scripting.rs`
   (dead code in `event_loop_schedule`). A directory helper's name ends with a
   non-digit suffix after the pid (e.g. `goad-wd-<case>-<pid>.d`), so it can
   never equal a `marker` path. **`claim`'s doc counts its helpers ("three of
   the four", "the four mint", "A fifth kind") and the count is already
   short.** Run `grep -rn 'process::id()' --include=*.rs . | grep -v '^./target'`
   and rewrite the sentence to **name** every helper of the class, held and
   unheld — including `crates/goad-check/tests/kit/examples.rs::handout`,
   `crates/goad/tests/binary/exit_codes.rs::scratch_config`, and this
   phase's — with **no number** (`CLAUDE.md`, *never count*). Keep the *One
   kind per helper* rule. `goad-emit`'s target does not include the file; its
   new helper follows `exchange.rs::socket_path`'s precedent.
7. **The listener case and R5.** Before the red run the case asserts no
   `goad.sock` and no `goad.sock.lock` in the test's working directory and
   removes any it finds, and removes both again at its end (the red run binds
   there; the lock is opened before the bind). Use `listener::cleanup`.
8. **`goad-emit`'s binary case asserts the exit status before it joins**
   `fake_listener` (which blocks in `accept`): in the red run nothing
   connects, so a `join()` first hangs instead of failing.
9. **`goad-emit`'s binary case** sets `XDG_CONFIG_HOME` and `current_dir` on
   the **child only** (`Command::env`, `Command::current_dir`).
   `std::env::set_var` is disallowed by `clippy.toml`. Set `XDG_CONFIG_HOME`
   explicitly — the inherited one names the person's own config.
10. **`tagged.rs::check`** gains the Markdown file's directory; its synthetic
    cases pass any absolute directory — `parse` never touches the filesystem.
11. **`config.rs`'s tests module stays last** (`items_after_test_module` under
    `-D warnings`).
12. **Cite by symbol, never by line number; name, never count** — in code
    docs and in this sheet.

**Tasks** — `[ ]` todo · `[~]` in progress · `[x]` done · `[!]` blocked
- [x] Print `git -C /home/david/dev/goad/.claude/worktrees/slice-015 log -1
      --oneline`; record it here. Set PHASE-01 `in progress` in §Status.
      (`c24c9c8 015: PHASE-01 sheet`.)
- [x] API stubs + `Config::parse(text, directory)` signature through every
      caller (Surfaces); gate compiles.
- [x] Write VT-1, VT-2 (config.rs tests), VT-6 (`working_directory.rs`, new,
      with its `mod` line), VT-4 (startup.rs `listener`), VT-5 (both
      `goad-emit` cases); run against the stubs; record each red below.
- [x] Real bodies: `anchored`, `executable`, `load` (canonicalize, A-1 arm),
      `parse` (struct update, anchored ingress); EX-6's two meaning changes;
      `ConfigError::Read`'s doc.
- [x] VT-3 and VT-7 pins, green on arrival.
- [x] Refactor step: one join, docs true, no dead helper.
- [x] `claim`'s doc rewritten by name (Note 6).
- [x] M1–M5, each recorded below; `git status` clean after each restore.
- [x] VA-1, VA-2 recorded below.
- [x] `just check` exits 0; §Status PHASE-01 `done`; §Harvest updated; commit.

**Natural-red evidence** — one row per case: name · command · the assertion it
failed on. A red for another reason than `design.md` §9's *red today because*
column does not count.

| case | command | failed on (assertion, and why it matches §9) |
|---|---|---|
| `config.rs::tests::a_relative_path_resolves_against_the_directory_given` | `cargo test -p goad-shell --lib config::` (against the stubs) | `assertion left == right`, `ingress.path = "./goad.sock"`: left `./goad.sock`, right `/anchor/goad.sock` — `parse` ignores the directory (§9: no such parameter) |
| `config.rs::tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash` | same | `program = "./b.sh"`: left `./b.sh`, right `/anchor/b.sh` — `executable` returns `program` unchanged (§9: no `executable`) |
| `working_directory.rs::a_config_reached_through_a_linked_directory_is_anchored_at_the_target` | `cargo test -p goad-shell --test integration working_directory` | `working_directory` left `None`, right `Some(<t>/real)` — `load` does not set one (§9: no `working_directory`) |
| `startup.rs::listener::a_relative_configured_path_binds_beside_the_config` | `cargo test -p goad --test renderer a_relative_configured` | `no socket at "<tmp>/goad-wd-relative-<pid>.d/goad.sock"` (the `listener` call itself was `Ok`; the socket went to the crate directory — strays cleared, `git status` clean after) |
| `exchange.rs::a_relative_configured_socket_is_found_beside_the_configuration` | `cargo test -p goad-emit --test binary a_relative_configured` | exit status `left: 2, right: 0`: `goad-emit: nothing is listening at ./goad.sock` — connects in its own directory (asserted before the join; no hang) |
| `main.rs::tests::a_configured_ingress_path_is_what_is_written_to` (goad-emit) | `cargo test -p goad-emit --bin goad-emit a_configured` | `left: "./from-config.sock"`, `right: "/tmp/goad-emit-configured-<pid>/goad/from-config.sock"` |

**Mutation evidence** — runs use `--no-fail-fast`. Copy the file to the
scratchpad, edit, run, copy back; `git status` clean after each restore. A
mutation that does not compile is not evidence; if `unused` denies a deletion,
respell so the named behaviour is the one removed, and record the respelling.

| id | edit (behaviour removed) | must red | command | compiled? | redded (name · assertion) | restore green? |
|---|---|---|---|---|---|---|
| M1 | `ingress_config` ignores the directory: the path as written | `config.rs::tests::a_relative_path_resolves_against_the_directory_given`; `startup.rs::listener::a_relative_configured_path_binds_beside_the_config`; `exchange.rs::a_relative_configured_socket_is_found_beside_the_configuration`; goad-emit `main.rs::tests::a_configured_ingress_path_is_what_is_written_to` | `cargo test --no-fail-fast -p goad-shell -p goad -p goad-emit` | yes (`let _ = directory;` + `PathBuf::from(&file.path)`: respelled, since `unused` denies an unused `directory`) | 6: the four named, plus `config.rs::tests::an_ingress_section_loads_with_its_path` and `working_directory.rs::a_config_reached_through_a_linked_directory_is_anchored_at_the_target` (`left`/`right` on the path: `./goad.sock` vs the anchored one) | yes |
| M2 | `anchored` is `directory.join(path)` alone (`.` kept) | `a_relative_path_resolves_against_the_directory_given` (byte-exact) | `cargo test --no-fail-fast -p goad-shell -p goad -p goad-emit` | yes (`let _ = Component::CurDir;` + `directory.join(path)`: respelled to keep the import used) | `a_relative_path_resolves_against_the_directory_given`: `left "/anchor/./goad.sock"`, `right "/anchor/goad.sock"`; also `the_executable_is_anchored_only_for_a_relative_path_with_a_slash`: `"/anchor/./b.sh"` vs `"/anchor/b.sh"` | yes |
| M3 (01 half) | `load` anchors at the path's parent as given, not the canonical file's | `a_config_reached_through_a_linked_directory_is_anchored_at_the_target` | `cargo test --no-fail-fast -p goad-shell -p goad -p goad-emit` | yes (`path.parent()` for `canonical.parent()`) | `working_directory.rs::a_config_reached_through_a_linked_directory_is_anchored_at_the_target`: `working_directory` `Some(<t>/link)` vs `Some(<t>/real)` | yes |
| M4 | `canonicalize`'s failure is a `ConfigError` other than `Read` | `a_dangling_link_to_a_configuration_is_unreadable` | `cargo test --no-fail-fast -p goad-shell -p goad -p goad-emit` | yes (`.map_err(|_| ConfigError::EmptyCommand)` on `canonicalize`) | `a_dangling_link_to_a_configuration_is_unreadable` (panic: refused as another variant); also `a_configuration_is_read_from_a_file_and_a_missing_one_says_so`, `goad-emit` `main.rs::tests::an_absent_configuration_file_is_a_named_fault`, `goad` `exit_codes::an_unreadable_configuration_exits_2_and_says_only_what_its_own_arm_says` | yes |
| M5 | `ingress::bind` files `UnixListener::bind`'s failure as a `BindFault` other than `Unbindable` | `a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path` (and only the over-long half) | `cargo test --no-fail-fast -p goad-shell -p goad` | yes (`BindFault::Unprobeable` for `Unbindable` at `UnixListener::bind`) | `ingress.rs::a_path_longer_than_a_socket_address_holds_is_refused_naming_the_path`, alone: "an over-long path is R-4's unusable location, got: could not be inspected: path must be shorter than SUN_LEN" | yes |

M5 mutates `crates/goad-shell/src/ingress.rs`, outside Surfaces: permitted
under copy-and-restore, nothing committed.

**Renames** — any case shipped under a name or file other than `design.md`
§9's: §9's name · symbol shipped · file. (Audit re-points the citations; this
phase edits neither `design.md` nor `canon-delta.md`.)

None: every case ships under §9's name in §9's file.

**Decisions taken during execution**
- *`anchored` and the empty remainder (Note 3).* `.` and `./` anchor to the
  directory itself, with no trailing separator (`PathBuf::extend` over the
  non-`.` components pushes nothing). It can arise, via `ingress.path = "."`;
  the bind then fails on a directory, which is the honest report. Pinned in
  `a_relative_path_resolves_against_the_directory_given`'s table. `./` is
  the same: `Path::components` drops the trailing separator.
- *A-1's arm (Note 2).* `ConfigError::Read` over `io::Error::new(InvalidInput,
  "<canonical> names no file in a directory")`. No panic, no new variant.
- *Two new directory helpers, one per file, each claimed under its own kind.*
  `startup.rs::listener::directory` (kind `startup directory`) and
  `working_directory.rs::directory` (kind `working directory`), both minting
  `goad-wd-<case>-<pid>.d`; `exchange.rs::config_home`, unheld, minting
  `goad-emit-<case>-<pid>.d`. The `.d` suffix keeps them off every `marker`
  path.
- *Byte-exact comparisons use `OsStr`* (see Findings).
- `Command::executable` carries `#[must_use]`.
- `host.rs` and `round_trip.rs` pass `CARGO_MANIFEST_DIR` as the directory:
  neither depends on it (`round_trip.rs` rebases the command itself; D10 is
  PHASE-02's).

**Findings**
- **`Path` equality ignores an interior `.`.** `Path::new("/a/./b") ==
  Path::new("/a/b")`. The first VT-1 and VT-2 compared `PathBuf`s, and M2
  (`directory.join(path)`, `.` kept) went **green**. Both now compare
  `as_os_str()` against `OsStr::new(..)` (bytewise), and M2 reds both. Found
  by running the mutation, not by reading the test. Candidate memory:
  "a path assertion about bytes must compare `OsStr`".
- M5's file is `crates/goad-shell/src/ingress/mod.rs` (the sheet says
  `ingress.rs`). Restored; `git status` clean.
- `temp_dir()` was `/tmp` in this run; the length case's directory is 34
  bytes, leaving 72 for the file name of the 107-byte control.
- `startup.rs`'s existing `socket_path` doc (inside `mod listener`, untouched
  here) says "the third such helper in this test binary" — a count the
  *never count* rule forbids. Left for audit (outside this phase's edit of
  `listener`: one new case).
- VA-1: `just check` exits 0, so `transport_shape.rs`'s guards and every
  *unaffected* case are green. VA-2: `git status` after the red runs showed no
  socket, lock or config in a crate directory (the listener case clears and
  re-checks its strays).

**Orchestrator re-measure at `571787b`** (2026-10-19): `just check` exit 0;
M3 (01 half) re-run — `load` anchored at `path.parent()` — compiled, redded
`working_directory::a_config_reached_through_a_linked_directory_is_anchored_at_the_target`
only, restore clean. One doc nit for audit: `Config::parse`'s doc reads
*"is the only caller that need"*.

### PHASE-02 — the spawn

Written by the orchestrator, 2026-10-19, at `571787b`; executed by a separate
agent. Quotes the plan where it restates it; says so where it narrows.

**Objective** (`plan.md` PHASE-02, quoted): *"a configured backend runs in its
config's directory with `PWD` naming it, a relative program with a `/` is
executed by its anchored path, and a spawn failure names where the program
was looked for."*

**Reading list**
- `plan.md` §Sequencing & rationale (*Size* — the PARTIAL checkpoint;
  *Natural-red evidence*; *Mutation evidence*; the M-table), §PHASE-02 entire.
- `design.md` §5.2 (the `exchange` substance, **Errors**, the error table),
  §5.5 (I-2, I-4), §7 D5, D10, D12, D13, §9 (the spawn rows, AC-2's row,
  *Existing tests changed*).
- `research.md` T2-3, T2-4, T2-9, T2-S1, T2-S2 (a script's `$0` is the path
  `exec` received; `sh -c` with no operand gives argv[0]).
- `review-design.md` F-7 (why `python3` for `PWD`), F-2 and R1-S2/F-18 (the
  anchored-name witnesses); `review-plan.md` F-15 (VT-10, `settle-in-code`).
- Code: `crates/goad-shell/src/backend/process.rs` (`ProcessBackend::exchange`);
  `crates/goad-shell/src/error.rs` (`BackendError::Spawn`, `Display`,
  `source`, `requirement`, `fault`, its tests);
  `crates/goad-shell/tests/shape/transport_shape.rs`
  (`the_only_spawn_is_the_child`); the Spawn sites named in Surfaces;
  `crates/goad-shell/tests/integration/working_directory.rs` (PHASE-01's
  file and `directory` helper — extend it); `tests/support/scripting.rs`
  (`logging_backend`, `claim`); `tests/backends/answers-a-round-trip.sh` (a
  conforming answer); `crates/goad-check/tests/kit/examples.rs`
  (`handout`, `run_checker`); `round_trip.rs`
  (`the_readme_s_own_config_loads_and_runs_the_example`,
  `rooted_at_the_workspace`); `exercisers/typescript/README.md`.
- PHASE-01's sheet above: its Findings (byte-exact path comparisons need
  `OsStr`; `Path` equality ignores an interior `.`) apply here.

**Surfaces — a closed list.** Anything outside it is a STOP.
- `crates/goad-shell/src/backend/process.rs` — `ProcessBackend::exchange`
  only.
- `crates/goad-shell/src/error.rs` — `BackendError::Spawn`'s shape, its
  `Display`, `source`, `requirement`, `fault` arms, and its tests (and the
  enum's doc where it describes `Spawn`'s payload).
- `BackendError::Spawn`'s other sites:
  `crates/goad-shell/tests/integration/transport.rs`
  (`a_command_that_does_not_exist_fails_to_spawn`),
  `crates/goad-shell/tests/integration/failure_matrix.rs`
  (`a_command_that_cannot_be_spawned_reaches_the_caller_as_a_spawn_failure`),
  `crates/goad/tests/renderer/table.rs`,
  `crates/goad-check/tests/kit/coverage.rs`.
- `crates/goad-shell/tests/integration/working_directory.rs` — the spawn
  cases (and its module doc).
- `crates/goad-shell/tests/integration/round_trip.rs` —
  `the_readme_s_own_config_loads_and_runs_the_example`, its doc, and
  `rooted_at_the_workspace` (deleted, with any import it alone used);
  `exercisers/typescript/README.md` (D10).
- `crates/goad-check/tests/kit/examples.rs` (`run_checker` and its doc, the
  module doc), `crates/goad-check/tests/kit/main.rs` (module doc),
  `crates/goad-check/tests/kit/tree.rs` (`files`' doc).
- `tests/support/scripting.rs` — `claim`'s doc only, and only if this phase
  adds a helper of `claim`'s class. *(Narrowing note: `plan.md` PHASE-02
  does not list this file. The sheet adds it on the same terms PHASE-01 and
  PHASE-03 have, because extending `working_directory.rs`' existing
  `directory` helper needs no edit; a new helper would. Prefer reusing
  `directory`.)*
- `docs/slices/015/notes.md` — this sheet, §Status, §Harvest.

**Entry** — checked by the orchestrator before hand-over:
- EN-1 ✔ PHASE-01 done; `git log -1` = `571787b` before this sheet's commit
  (the sheet's commit follows it; your `log -1` will show the sheet commit).
- EN-2 ✔ slice 012 `notes.md` §Status on `main`: PHASE-10 `pending`.

**Assumptions**
- *Verified at plan / by the orchestrator at `571787b`:* `ProcessBackend::new`
  callers pass `config.backend.command` (T2-4), so the working directory
  arrives with no `main.rs` edit. `exchange` builds `let mut command =
  Command::new(&self.command.program);` and `command.spawn()` is the file's
  only `spawn`. `BackendError::Spawn(io::Error)` is matched or built at
  exactly: `error.rs` (`Display`, `source`, `requirement`, `fault`, its
  tests), `process.rs`, `transport.rs`, `failure_matrix.rs`, `table.rs`,
  `coverage.rs` (grep `Spawn` over `crates/`). Devshell has `python3`,
  `bash`, `deno`; `sh` resolves to bash-interactive's `sh`.
  `scripting::logging_backend` exists.
- *First tested here:* that the copied devshell `sh` runs from a temp
  directory (it is dynamically linked against absolute store paths — should
  hold); that `$0` for `["./shell", "-c", …]` is the anchored path once
  `Command::new(&executable)` is used; that `goad-check` reads `--event`
  relative to its own working directory (so `examples.rs` passes
  `example/events/…`); that VT-10's assertion (`working_directory: None`)
  is red under M14 — **if M14 survives, `review-plan.md` F-15 returns
  `contested`: record it under Findings and report it.**

**STOP conditions**
- a file or symbol outside Surfaces needs to change;
- `ProcessBackend::new`'s signature or either binary's `main.rs` would need to
  change (`design.md` §5.2, T2-4);
- any test in `research.md` T2-9's *unaffected* list reds;
- the prefix `backend could not be spawned: ` that `table.rs` asserts would
  have to change;
- a dependency would be added;
- M14 survives (report, do not repair VT-10 into something else silently).

**Exit criteria** (quoted from `plan.md`; none narrowed)
- EX-1 — *"`exchange` builds `Command::new(&executable)` from
  `self.command.executable()`, passes `arguments` as written, and when a
  working directory is set applies `current_dir` and `env("PWD", …)`. No
  `arg0`. The line `let mut command = Command::new(` survives, and
  `command.spawn()` is still the file's only `spawn` (`transport_shape.rs`)."*
- EX-2 — *"`BackendError::Spawn { executable, working_directory, source }`;
  `Display` is `backend could not be spawned: {executable} (in
  {working_directory}): {source}`, the parenthesis only when a directory is
  set; `source()` returns `source`; `requirement()` is still R-44 and
  `fault()` still `AtFault::Configuration`. Every site in Surfaces matches or
  builds the new shape, and the prefix `table.rs` asserts is unchanged."*
- EX-3 — *"the `working_directory.rs` spawn cases (VT-1..VT-6) green, each
  seen red first."*
- EX-4 — *"D10: the README's config is `["deno", "run", "-A", "backend.ts"]`,
  its prose says the file is saved beside `backend.ts` and that a relative
  path resolves against the config file's directory;
  `rooted_at_the_workspace` is gone; the case parses with
  `exercisers/typescript` as the directory and runs the README's config
  unrewritten."*
- EX-5 — *"AC-2: `run_checker` starts the checker in the hand-out **root**
  with `--config example/config.toml` and `--event example/events/…`; the
  test docs that described starting in the copy are true of the new start."*
- EX-6 — *"M3's PHASE-02 half, M6–M10 and M13 run and recorded."* — plus
  **M14** (VT-10's mutation; the plan's M-table assigns it to phase 02).
- And `just check` exits 0 on the phase's final commit.

**Verification** — the cases, quoted in substance from `plan.md` VT-1..VT-10,
VA-1. Names are commitments (`design.md` §9); a rename goes in *Renames*.
- VT-1 `a_relative_argument_resolves_beside_the_config_from_another_directory`
- VT-2 `a_linked_config_runs_its_backend_beside_the_target` (decoy beside the
  link answers differently and is not the one run)
- VT-3 `a_relative_program_with_a_slash_is_executed_as_the_anchored_path`
- VT-4 `a_native_program_receives_its_anchored_path_as_its_name`
- VT-5 `the_backend_s_pwd_names_its_working_directory` (`python3`; reds on
  the `PWD` assertion against PHASE-01's tree — not on a spawn failure)
- VT-6 `a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for`
- VT-7 `round_trip.rs::the_readme_s_own_config_loads_and_runs_the_example`
- VT-8 `examples.rs::each_shipped_example_is_accepted_by_the_checker` and
  `::downloads_triage_moves_the_file_it_was_asked_about`
- VT-9 `error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable`
  (not a §9 name: record it under *Renames/additions* for audit)
- VT-10 `transport.rs::a_command_that_does_not_exist_fails_to_spawn` gains
  `working_directory: None` (settles `review-plan.md` F-15)
- VA-1 `transport.rs`' and `failure_matrix.rs`' spawn cases still assert
  what they asserted (`NotFound`; the configuration's side); only the match
  shape changed.

**Notes for the implementer** — `plan.md`'s, lifted in; read every one.
1. **Order.** Write VT-1..VT-6, change VT-7's and VT-8's tests first; run
   against PHASE-01's tree (spawn ignores the working directory); record
   each red and the assertion it failed on. VT-6 needs the new `Spawn` shape
   to compile: change `error.rs` and its sites first with `process.rs`
   filling `executable` from the program **as written** and
   `working_directory: None`; see VT-6 red; then EX-1.
2. **Every spawn case loads its config from a file with `Config::load`**, in
   a temp directory of its own; the test process's working directory (the
   crate) is never the config's.
3. **VT-4:** resolve `sh` at test time by running `sh -c 'command -v sh'` and
   canonicalizing the answer, so the copy is the real binary, not a link;
   `std::fs::copy` keeps the mode. For `sh -c` with no operand, `$0` is
   argv[0].
4. **VT-5 uses `python3`**, which exposes an inherited `PWD` as is; a shell
   would repair a stale one and hide the regression.
5. **Fixtures are written by the case**, not committed: a script body in the
   test file, written beside the config and made executable. Logging scripts
   write to a marker path passed as an argument, as
   `scripting::logging_backend` does; a backend that must answer
   conformingly answers as `tests/backends/` scripts do.
6. **Negative controls compile.** VT-2's decoy is a real script in the same
   case, runnable, answering differently — never a commented-out variant.
7. **D10's README** is the test's input: prose and fence change in the commit
   that changes the case. Keep the README's *host exerciser, not a template*
   framing.
8. **`examples.rs`**: `downloads_triage_moves_the_file_it_was_asked_about`
   calls `run_checker` too. `--event` paths gain the `example/` prefix; only
   `--config`'s directory moves the backend.
9. **`coverage.rs`** builds a `Spawn` to enumerate kinds; give it a
   directory-less value unless the case needs one.
10. **No new identifier in `process.rs`'s code contains `spawn`** in lower
    case (`transport_shape.rs::the_only_spawn_is_the_child` counts the token;
    strings and comments are stripped first).
11. **Byte-exact path assertions compare `OsStr`** (PHASE-01 Finding).
    Compare against `std::fs::canonicalize` of the directory, never the path
    as built.
12. **Checkpoint.** If the session runs long, stop after EX-3 (goad-shell
    tier green), commit `PARTIAL` with *where I stopped*, leaving
    `examples.rs` and D10. That is the plan's named checkpoint, not a
    re-plan.
13. Cite by symbol, never line number; name, never count.

**Tasks**
- [x] Print `git log -1 --oneline`; record it. §Status PHASE-02 `in progress`.
      (`fe81391 015: PHASE-01 re-measured; PHASE-02 sheet`.)
- [x] `Spawn` new shape through `error.rs` + every site; `process.rs` fills
      program-as-written / `None`; gate compiles.
- [x] VT-1..VT-6 written; VT-7 (D10 test side, `rooted_at_the_workspace`
      deleted) and VT-8 (`run_checker` from the root) changed; red runs
      recorded.
- [x] EX-1 in `exchange`; VT-1..VT-6 green.  ← PARTIAL checkpoint if needed
- [x] D10's README; VT-7 green. VT-8 green; test docs true (EX-5).
- [x] VT-9, VT-10 (each seen red as the plan says).
- [x] Refactor step.
- [x] M3 (02 half), M6–M10, M13, M14 recorded; `git status` clean after each.
- [x] VA-1 recorded.
- [x] `just check` exits 0; §Status PHASE-02 `done`; §Harvest; commit.

**Natural-red evidence**

| case | command | failed on (assertion, and why it matches §9) |
|---|---|---|
| VT-1 `a_relative_argument_resolves_beside_the_config_from_another_directory` | `cargo test -p goad-shell --test integration --no-fail-fast -- working_directory::` | `answered`'s *the backend must answer*: `backend exited with status 127` — bash could not open `backend.sh` in the crate dir (§9) |
| VT-2 `a_linked_config_runs_its_backend_beside_the_target` | same | *the backend must answer*: `backend exited with status 127` — old rule, nothing in the crate dir (§9's first clause; the decoy clause is M3's) |
| VT-3 `a_relative_program_with_a_slash_is_executed_as_the_anchored_path` | same | *the backend must answer*: `backend could not be spawned: ./backend.sh: No such file or directory` — spawn `NotFound` (§9) |
| VT-4 `a_native_program_receives_its_anchored_path_as_its_name` | same | *the backend must answer*: `backend could not be spawned: ./shell: No such file or directory` — spawn `NotFound` (§9) |
| VT-5 `the_backend_s_pwd_names_its_working_directory` | same | the `PWD` assertion: left `/home/david/dev/goad/.claude/worktrees/slice-015` (inherited), right `/tmp/goad-wd-pwd-<pid>.d` — the backend ran and answered; `PWD` inherited from the test process (§9), not a spawn failure |
| VT-6 `a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for` | same, with the new `Spawn` shape and `process.rs` filling the program as written / `None` | the `executable` assertion: left `./missing.sh`, right `/tmp/goad-wd-missing-<pid>.d/missing.sh` — `Spawn` carried neither path (§9) |
| VT-7 `the_readme_s_own_config_loads_and_runs_the_example` | `cargo test -p goad-shell --test integration --no-fail-fast -- round_trip::the_readme` (README and case rewritten, `rooted_at_the_workspace` deleted, spawn unchanged) | `quiet.failure.is_none()`: `backend exited with status 1` — deno could not find `backend.ts` in the crate dir |
| VT-8 `each_shipped_example_is_accepted_by_the_checker`, `downloads_triage_moves_the_file_it_was_asked_about` | `cargo test -p goad-check --test kit --no-fail-fast -- examples::` (checker started in the root, spawn unchanged) | accepted: `examples not accepted` — breadcrumbs, downloads-triage, focus-check each `exit 1`, breadcrumbs' stderr `Module not found "file:///tmp/goad-kit-accepted-breadcrumbs-<pid>/backend.ts"`; triage: `code_of` left 1, right 0 — each backend is not in the root (§9) |
| VT-9 `a_spawn_failure_with_no_working_directory_names_only_the_executable` (red against a `Display` printing the parenthesis always) | `cargo test -p goad-shell --lib --no-fail-fast -- error::tests`, with `Display` writing ` (in {})` over `working_directory` or an empty path, then restored | `assert_eq!` on `to_string()`: left `backend could not be spawned: ./backend.sh (in ): refused`, right `… ./backend.sh: refused` |

**Mutation evidence** — `--no-fail-fast`; scratchpad copy, edit, run, copy
back; `git status` clean after each. Not compiled = not evidence.

| id | edit (behaviour removed) | must red | command | compiled? | redded (name · assertion) | restore green? |
|---|---|---|---|---|---|---|
| M3 (02 half) | `load` anchors at the path's parent as given | `a_linked_config_runs_its_backend_beside_the_target` (the decoy answers) | scratchpad `mutate.sh`: copy, edit, `cargo test -p goad-shell --no-fail-fast` + `cargo test -p goad-check --test kit --no-fail-fast -- examples::`, copy back; `config.rs` `canonical.parent()` → `path.parent()` | yes | `a_linked_config_runs_its_backend_beside_the_target` · `answered` left `{"view":null,"next_check":"1 minute"}` (the decoy's) vs `{"view":null}`; also PHASE-01's `a_config_reached_through_a_linked_directory_is_anchored_at_the_target` · `working_directory` `…/link` vs `…/real` | yes; restore byte-identical, `git status` clean |
| M6 | no `current_dir` (`PWD` still set) | `a_relative_argument_resolves_beside_the_config_from_another_directory`; `a_linked_config_runs_its_backend_beside_the_target`; `examples.rs::each_shipped_example_is_accepted_by_the_checker` | same; `command.current_dir(directory).env("PWD", directory)` → `command.env("PWD", directory)` | yes | VT-1, VT-2 · *the backend must answer*: `exited with status 127`; `each_shipped_example_is_accepted_by_the_checker` · breadcrumbs, downloads-triage, focus-check `exit 1`; beyond the required: `downloads_triage_moves_the_file_it_was_asked_about` (`code_of` 1), VT-3 (`pwd -P` line is the crate dir), VT-7 (`quiet.failure`) | yes; restore byte-identical, `git status` clean |
| M7 | the platform's route: `Command::new(&self.command.program)` with `current_dir` | `a_relative_program_with_a_slash_is_executed_as_the_anchored_path` (`$0` relative); `a_native_program_receives_its_anchored_path_as_its_name` | same; `Command::new(&executable)` → `Command::new(&self.command.program)` | yes | VT-3 · log's `$0` line `./backend.sh` vs `<anchor>/backend.sh`; VT-4 · `./shell` vs `<anchor>/shell`. Nothing else | yes; restore byte-identical, `git status` clean |
| M8 | `arg0(&self.command.program)` added to the anchored spawn | `a_native_program_receives_its_anchored_path_as_its_name` **only** — the script case stays green (D5's point; record that it stayed green) | same; `command.arg0(&self.command.program);` after `Command::new(&executable)` | yes | VT-4 only · `./shell` vs `<anchor>/shell`. **VT-3 (the script) stayed green**: 103 passed, 1 failed | yes; restore byte-identical, `git status` clean |
| M9 | no `PWD` | `the_backend_s_pwd_names_its_working_directory` | same; `.env("PWD", directory)` dropped | yes | VT-5 only · `PWD` left the worktree root (inherited) vs `<anchor>` | yes; restore byte-identical, `git status` clean |
| M10 | `Spawn`'s `working_directory` is `None` for a configured command | `a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for` | same; `working_directory: self.command.working_directory.clone()` → `None` | yes | VT-6 only · `working_directory` left `None` vs `Some(<anchor>)` | yes; restore byte-identical, `git status` clean |
| M13 | `Spawn`'s `Display` prints the parenthesis whatever the directory | `error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable` | same; the `if let Some` replaced by an unconditional ` (in {})` over `working_directory` or an empty path | yes | VT-9 only · `… ./backend.sh (in ): refused` vs `… ./backend.sh: refused` | yes; restore byte-identical, `git status` clean |
| M14 | the spawn fills `Spawn`'s `working_directory` from `current_dir()` for a command with none | `transport.rs::a_command_that_does_not_exist_fails_to_spawn` | same; `.or_else(\|\| std::env::current_dir().ok())` on the clone | yes | VT-10 only · `working_directory` left `Some(<crate dir>)` vs `None`. **Red: `review-plan.md` F-15 holds** | yes; restore byte-identical, `git status` clean |

**Renames / additions** — §9's name · symbol shipped · file.
- VT-9 is an addition (not in §9): `error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable`.

**Decisions taken during execution**
- *VT-9 asserts the whole `Display`* (`backend could not be spawned:
  ./backend.sh: refused`), which holds both halves of the plan's wording —
  names the executable, contains no `(in ` — in one equality.
- *Each spawn case logs to a file inside its own `directory`*, passed as an
  argument (the `logging_backend` form), not to a `marker`: the directory is
  already claimed, so no second kind is minted, and one `remove_dir_all`
  clears the case.
- *The case-written backend* (`working_directory.rs::logging_script`) is one
  `#!/bin/sh` script with the executable bit, so the same file serves
  `["bash", "backend.sh", log]` (VT-1, VT-2 and its decoy) and
  `["./backend.sh", log]` (VT-3). It logs `pwd -P` then `$0`.
- *Command arguments are quoted by `toml::Value`'s `Display`*
  (`working_directory.rs::configuration`) — `toml` is already a `goad-shell`
  dependency, so no escaping of our own and no new dependency.
- *VT-5's answer is `{"view": null}`* (`json.dumps`' spacing); the case
  asserts it, so the body is seen to come from the python backend.
- *VT-7's anchor is `CARGO_MANIFEST_DIR/../../exercisers/typescript` as
  given*, not canonicalized: the tier's own rooting rule (`harness.rs::example`,
  `shell_example`), and `parse` takes the directory as given (I-1 is
  `load`'s).
- *`exchange`'s order*: `Command::new(&executable)`, then the
  working-directory block, then the existing `args`/stdio chain. Same
  substance as `design.md` §5.2's snippet; keeps the chain unbroken.
  `executable` moves into `Spawn` on failure (no clone).
- *Comments in `process.rs` cite "slice 015's D5 / D12"*: the file already
  cites slice 001's D-ids (D34, D44, D48, D53), so a bare `D5` would be
  ambiguous there. Code and test docs cite `SPEC-001/R-60`, which is in
  `canon-delta.md` and promoted at audit.
- *Directory-less `Spawn` values* in `table.rs` and `coverage.rs`
  (`executable: "no-such-program"`, `working_directory: None`); `error.rs`'s
  `every_backend_error` carries a configured shape (`/anchor/backend.sh` in
  `/anchor`), so both shapes are in that file's tests.

**Findings**
- **M14 reds VT-10**: `review-plan.md` F-15 is settled in code, not
  contested.
- **VA-1**: `transport.rs::a_command_that_does_not_exist_fails_to_spawn`
  still asserts `NotFound` on the `Spawn`'s `source`;
  `failure_matrix.rs::a_command_that_cannot_be_spawned_reaches_the_caller_as_a_spawn_failure`
  still matches `Spawn` and the seed `next_check`; `error.rs`'s
  `every_backend_error_names_a_requirement_and_a_side` holds R-44 and the
  configuration's side; `table.rs`' T1 row still asserts the prefix
  `backend could not be spawned: `. Only the match shape changed. Gate green.
- M6 reds more than its row requires (VT-3's `pwd -P` line, VT-7, the triage
  case): all consistent with *no `current_dir`*. M7–M10, M13, M14 each red
  their named cases and nothing else.
- No crate-directory litter after the red runs (`git status` clean; the
  kit's hand-outs and the cases' directories are under `temp_dir()`).

**Orchestrator re-measure at `934933f`** (2026-10-19): `just check` exit 0;
M7 re-run — `Command::new(&self.command.program)` — compiled, redded
`a_relative_program_with_a_slash_is_executed_as_the_anchored_path` and
`a_native_program_receives_its_anchored_path_as_its_name` only (integration
target), restore clean. For audit: `process.rs` and `error.rs` docs cite
`SPEC-001/R-60` ahead of promotion (as `design.md` §5.2's own doc does); the
merge carries the promotion, but audit confirms none of it reaches `main`
unpromoted.

### PHASE-03 — the checker's pins, and the kit's prose

Written by the orchestrator, 2026-10-19, at `934933f`; executed by a separate
agent. Quotes the plan where it restates it; says so where it narrows.

**Objective** (`plan.md` PHASE-03, quoted): *"`goad-check`'s built binary is
pinned to find a bare program on `PATH` and to run the argv form from the
caller's directory, and the kit states the new rule with no *make every path
absolute* advice."*

**Reading list**
- `plan.md` §Sequencing & rationale (*Mutation evidence*, M11, M12),
  §PHASE-03 entire.
- `design.md` §5.5 (*Edge — bare program*, *read-only config directory*,
  *long socket path*, *the argv form*), §7 D6, D8, §9 (the two `run.rs` rows;
  AC-3's row), §10 (*Not canon, repaired in the slice*).
- `canon-delta.md` — R-60 and R-17's wording, **to state the rule
  accurately in prose; never cite the ids** (Note 1).
- `research.md` T2-11 (what the kit says today).
- `review-design.md` F-5 (why the `PATH` case is on the built-binary tier),
  F-8 (why READMEs keep *copy this directory out*).
- Code: `crates/goad-check/tests/binary/run.rs` (its `logging` helper,
  `scripting` use), `crates/goad-check/tests/binary/process.rs` (`check`,
  `fixture` — no helper takes an env or a working directory today);
  `tests/support/scripting.rs` (`claim`); `tests/backends/` (conforming
  answers); `crates/goad-check/tests/kit/{tagged,standalone,examples}.rs`
  (the kit's own gate).
- Kit: `kit/skills/goad-backend/reference/{running,checking,events}.md`; the
  three examples' `config.toml`, `README.md`; `downloads-triage/watch.sh`;
  `breadcrumbs/chpwd.zsh`.
- PHASE-01's and PHASE-02's Decisions and Findings above.

**Surfaces — a closed list.** Anything outside it is a STOP.
- `crates/goad-check/tests/binary/run.rs` — the pins VT-1 and VT-2 (and a
  local temp-root helper); `crates/goad-check/tests/binary/process.rs` — a
  spawn helper taking an environment and a working directory, if the cases
  need one.
- `tests/support/scripting.rs` — `claim`'s doc only, if a helper here is
  held by `claim`.
- `kit/skills/goad-backend/reference/running.md` (*Name your command* —
  SPEC-001/R-36's section — and the config table's `ingress.path` row),
  `reference/checking.md` (the relative-path paragraph, and the argv form's
  rule, OQ-5), `reference/events.md` (`goad-emit`'s socket rule and its
  `--socket ./goad.sock` example).
- `kit/skills/goad-backend/examples/{breadcrumbs,downloads-triage,focus-check}/config.toml`
  — the opening comment only; no value changes (`research.md` T2-11).
- `kit/skills/goad-backend/examples/{breadcrumbs,downloads-triage,focus-check}/README.md`,
  including each README's `toml goad:config` fence, which quotes its
  `config.toml` and its comment verbatim; `examples/downloads-triage/watch.sh`
  (header); `examples/breadcrumbs/chpwd.zsh` (header) only if VA-2 classes it
  stale.
- `docs/slices/015/notes.md` — this sheet, §Status, §Harvest.

**Entry** — checked by the orchestrator before hand-over:
- EN-1 ✔ PHASE-02 done (`934933f`), re-measured above.
- EN-2 ✔ `git merge main` in the worktree: *Already up to date* — `main`
  has no commit past `c68a624`, so the kit is 015's base text and the gate
  result above stands. Slice 012 `notes.md` §Status on `main`: PHASE-10
  `pending`.

**Assumptions**
- *Verified at plan / by the orchestrator:* the example configs' values are
  already config-relative (T2-11) — only their comments and the READMEs'
  prose state the old rule. Today's comments: breadcrumbs and
  downloads-triage *"`command` and the socket are relative to this directory:
  run goad and goad-check from here."*; focus-check *"`command` is relative to
  this directory: run goad and goad-check from here."* VA-1's grep today has
  hits in `kit/` (the orchestrator counted them only to confirm the grep
  reaches; classify each). `run.rs` already uses `crate::scripting`, so
  `claim` reaches a helper there. `process.rs::check` takes arguments only.
  Every `toml` fence in the kit goes through `Config::parse`
  (`tagged.rs::every_json_and_toml_fence_in_the_kit_is_tagged_and_checked`),
  and `standalone.rs::nothing_in_the_kit_names_a_path_outside_it` forbids a
  path outside `kit/`.
- *First tested here:* that `#!/bin/sh` decoys using builtins only run with
  `PATH` = `<root>/path` alone (the kernel resolves the shebang); that
  `goad-check -- ./backend.sh` started in a directory holding it is judged
  (PHASE-02 left the argv form's `None` untouched, so it should be green on
  arrival).

**STOP conditions**
- a file or symbol outside Surfaces needs to change;
- either pin is red on arrival (it is a pin: red means PHASE-01/02 code is
  wrong — report it, do not adjust the pin);
- a kit rule would be left ambiguous (no merge happened, but the same applies
  if the new rule cannot be stated without contradicting surrounding kit text);
- a kit edit would need a new `### SPEC-…` heading or a citation of `R-60` /
  `R-17`;
- a dependency would be added.

**Exit criteria** (quoted from `plan.md`; none narrowed)
- EX-1 — *"`run.rs` holds the pins VT-1 and VT-2; M11 and M12 run and
  recorded."*
- EX-2 — *"`running.md` says: the backend runs in the config file's
  directory, through any links; arguments are passed as written; a relative
  program with a `/` is found beside the config and receives that path as
  its name; a bare name is looked up on `PATH`, after the change of
  directory, so a relative `PATH` entry names a directory under it; a spawn
  failure names the path tried and the directory. Its `ingress.path` row
  says relative to the config file's directory, and states the socket-path
  limit (107 bytes on Linux) and the remedy (an absolute path somewhere
  short, such as `$XDG_RUNTIME_DIR`). It says the rule changed."*
- EX-3 — *"`checking.md` says `--config` runs the backend as goad will, from
  any directory; the argv form runs from the caller's directory (OQ-5)."*
- EX-4 — *"`events.md`: `goad-emit` without `--socket` resolves a relative
  `ingress.path` as goad does; its `--socket` path is used as given,
  relative to where `goad-emit` runs."*
- EX-5 — *"each example README keeps *copy this directory out*, now for its
  reason: the socket and its lock are written beside the config, and an
  installed kit is read-only (`design.md` §5.5, F-8). *Run goad from this
  directory* and *make both absolute* are gone; a copy needs no edit to its
  config. `watch.sh`'s header is true from any directory."*
- EX-6 — *"each example `config.toml`'s opening comment states the new rule
  — `command` and the socket are relative to this file's directory, from
  wherever goad or `goad-check` is started — and no longer says *run … from
  here*; each README's `goad:config` fence quotes its `config.toml` as the
  file now reads, comment included."* (focus-check has no socket: its comment
  names `command` only, as today.)
- And `just check` exits 0 on the phase's final commit.

**Verification** (quoted in substance from `plan.md`)
- VT-1 — `run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config`:
  under the case's temp root, `config/config.toml` with `command =
  ["goad-decoy", <log>]`; `config/goad-decoy` and `path/goad-decoy`, each
  `#!/bin/sh` using builtins only, each appending its own label (`beside`,
  `from-path`) and answering conformingly; `goad-check --config
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
  goad' -- kit` recorded below, each hit classed as true under the new rule
  or repaired. No hit advises making a path absolute, or running from a
  directory, to work around where goad was started.
- VA-2 — `chpwd.zsh`'s header (*add `--socket` with the absolute path*)
  classed: `goad-emit`'s `--socket` is used as given, so the advice may still
  be true; recorded either way.
- VA-3 — each README's `goad:config` fence byte-identical to its
  `config.toml` (a `diff` of the fence body against the file, recorded).

**Notes for the implementer** — `plan.md`'s, lifted in.
1. **Cite no draft id in the kit.** `R-60` and `R-17` are not canon until
   audit. State the rule in prose under the existing `SPEC-001/R-36` heading
   and the config table. No new `### SPEC-…` heading: `coverage.rs` counts
   headings by id.
2. **The `PATH` case is on the built-binary tier on purpose**: `PATH`
   reaches the backend only through `Command::env` on the `goad-check`
   child. Decoys use builtins only (`read`, `printf`, redirection); `#!/bin/sh`
   is resolved by the kernel, not `PATH`. "Answering conformingly" means each
   answers every kind the checker sends with a response it accepts, as
   `tests/backends/` scripts do; the assertion is on the log, not on the
   verdict. Set `PATH` (and the working directory) on that child only; never
   `std::env::set_var`.
3. **The temp root** for both cases: one local helper in `run.rs`, held by
   `scripting::claim` under a kind of its own, a name ending in a non-digit
   suffix after the pid (PHASE-01's `.d` convention), and `claim`'s doc
   updated **by name** (PHASE-01 rewrote it to name every helper; add yours,
   no count).
4. **Kit prose is product text.** Short, second person, as the reference
   already reads. Every `toml` fence keeps its `goad:config` tag and must
   parse; nothing may name a path outside `kit/`. Write it for a person who
   has never read this slice: say what happens, not what changed in which
   slice (EX-2's *"It says the rule changed"* is one plain sentence for
   readers of older configs, not a history).
5. **M11 and M12 mutate PHASE-01/02 code** (`config.rs::Command::executable`;
   the argv form's `working_directory` in `goad-check`'s `from_argv` path, or
   `process.rs`) under copy-and-restore; nothing committed outside Surfaces.
6. Cite by symbol, never line number; name, never count — in kit prose too
   (*"each example"*, not *"the three examples"*).

**Tasks**
- [x] Print `git log -1 --oneline`; record it. §Status PHASE-03 `in progress`.
      `7fd8cb9 015: PHASE-02 re-measured; PHASE-03 sheet`.
- [x] VT-1, VT-2 written; green on arrival (pins); recorded.
- [x] M11, M12 recorded.
- [x] `running.md` (EX-2), `checking.md` (EX-3), `events.md` (EX-4).
- [x] Example `config.toml` comments (EX-6) and README fences + prose (EX-5),
      `watch.sh` header; VA-2 on `chpwd.zsh`.
- [x] VT-3 (kit tier green), VA-1 grep classified, VA-3 diffs recorded.
- [x] Refactor/re-read pass over every edited kit paragraph.
- [x] `just check` exits 0; §Status PHASE-03 `done`; §Harvest; commit.

**Pin evidence** (green on arrival)

| case | command | result |
|---|---|---|
| VT-1 `a_program_with_no_slash_is_found_on_path_not_beside_the_config` | `cargo test -p goad-check --test binary -- run::a_program_with_no_slash run::the_argv_form` | green on first run; the log read `from-path` ×4 (each host kind and the probe) |
| VT-2 `the_argv_form_runs_a_relative_program_from_the_caller_s_directory` | same run | green on first run; exit 0, log `ran` ×4 |

**Mutation evidence**

| id | edit (behaviour removed) | must red | command | compiled? | redded (name · assertion) | restore green? |
|---|---|---|---|---|---|---|
| M11 | `executable` anchors every relative program, `/` or not (guard's `&& self.program.contains('/')` dropped) | `run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config` (`beside` lines); also `config.rs::tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash` | `cargo test -p goad-check --test binary --no-fail-fast -- run::`; `cargo test -p goad-shell --lib --no-fail-fast -- config::` | yes | VT-1 · `labels.iter().all(..)`, log `["beside", "beside", "beside", "beside"]`; the unit case · its table assertion. Nothing else in either filter | yes; `git status` clean of `config.rs` |
| M12 | the argv form is given a working directory other than the caller's (`args.rs`'s `from_argv` result given `Some(temp_dir())`) | `run.rs::the_argv_form_runs_a_relative_program_from_the_caller_s_directory` | `cargo test -p goad-check --no-fail-fast` | yes | VT-2 · `code_of == 0`; also `args::tests::{every_token_after_the_separator_is_the_commands, the_command_after_the_separator_is_the_argv_form_with_a_five_second_timeout, timeout_sets_the_argv_forms_timeout}`, which compare the whole `Command` (`working_directory: None`) | yes; `git status` clean of `args.rs` |

**VA-1 — grep hits, classed**

Before the edits the grep hit each example `config.toml`'s comment and its
README fence copy (*relative to this directory*, *from here*), each README's
*Run it from anywhere else* prose (*started in*, *make both absolute*,
`/absolute/path/to/…`), `checking.md`'s relative-path paragraph and
`running.md`'s *Name your command* paragraph and `ingress.path` row: all
repaired. After them, on the committed text:

| hit | class |
|---|---|
| each README's *From this directory, with `goad-check` … on `PATH`:* | true: the `goad-check --config config.toml [--event events/…]` beneath it names paths relative to the caller, which the new rule does not touch |
| each README's *run `goad config.toml` from this directory* | true: `config.toml` is goad's argument, relative to the caller; `goad-emit --socket ./goad.sock` beside it is run from the same directory |
| `chpwd.zsh` header, *add `--socket` with the absolute path* | true: VA-2 |
| `focus-check/README.md`, *as an absolute instant* | unrelated: `next_check`'s form |
| `running.md`, *an absolute path is used as written* | true: the new rule's third case |
| `running.md`, *Relative paths used to mean the directory goad was started in* | true: EX-2's one sentence that the rule changed |
| `running.md`, *not absolute* (`$XDG_CONFIG_HOME`) | unrelated: the default config file's lookup |
| `running.md`'s `ingress.path` row, *use an absolute path somewhere short* | true: the socket-length remedy (D8), not a workaround for where goad started |

No hit advises making a path absolute, or running from a directory, to work
around where goad was started.

**VA-2 — `chpwd.zsh`**

True, left as is. The hook runs `goad-emit` after every `cd`, from whatever
directory the shell has just entered, and `--socket` is used as given, so
only an absolute path names one socket from every directory. The reason is
`goad-emit`'s working directory, not goad's.

**VA-3 — fence vs file**

`awk '/^```toml goad:config/{f=1;next} /^```/{if(f){exit}} f' README.md |
diff - config.toml` in each example directory: no output for breadcrumbs,
downloads-triage and focus-check.

**Renames / additions**
- `process.rs::check_in(directory, environment, arguments)` — the spawn
  helper the sheet allowed: working directory and environment on the
  checker's child alone.
- `run.rs::root` (kind `checker root`, `goad-check-<case>-<pid>.d`),
  `run.rs::labelled_backend`, `run.rs::labels`. `claim`'s doc names `root`
  and its path shape.

**Decisions taken during execution**
- *VT-2 asserts exit 0, not just "judged".* A judged run is 0 or 1, and
  M12's spawn failure is judged too (exit 1, R-44), so "judged" alone would
  stay green under M12. The backend is conforming, so the pin is
  "accepted", plus a non-empty log.
- *One labelled backend for both cases* (`labelled_backend(path, label)`):
  VT-1's two decoys and VT-2's backend differ only in their label. It reads
  stdin with `while read`, a builtin, not `cat`, which VT-1's `PATH` lacks.
- *VT-1's config quotes the log path through `serde_json::Value`*: a JSON
  string is a TOML basic string for an ASCII path, and `toml` is not a
  dev-dependency of `goad-check` (`serde_json` is).
- *Each README's section is renamed* *Run it from anywhere else* → *Run your
  own copy*: every example now runs from anywhere as shipped, so the old
  heading promised what no longer needs a copy. Nothing links to it (grep).
- *focus-check's reason for a copy is editing, not the socket.* It has no
  `[ingress]` and keeps its state under `$XDG_STATE_HOME`, so it runs from an
  installed kit as it is; its README keeps *copy this directory out* for the
  one reason that applies to it — an installed kit is read-only, so a change
  needs a copy. EX-5's socket reason is stated in the two event-driven
  READMEs only.
- *The README fences stay as the copy's config*, introduced with *Its config
  needs no edit:*; the *becomes, in your copy, …* sentences are deleted, and
  `goad-emit` (and `watch.sh`) are told to take the copy's `goad.sock` with
  `--socket`, since `goad-emit` reads only the default config file.
- *`watch.sh`'s example keeps `--socket ./goad.sock`*, now introduced as
  *from this directory*, so the header holds wherever it is read from.
- *`events.md`'s `--socket ./goad.sock` example is kept*: the new sentence
  says a relative `--socket` is relative to where `goad-emit` runs.

**Findings**
- M12 also reds three `args.rs` unit cases that compare the whole parsed
  `Command`: the argv form's `working_directory: None` is already held at
  the unit tier as a value, and VT-2 adds the behaviour.
- Both pins were green on first run with no adjustment: the `#!/bin/sh`
  decoys run under `PATH` = `<root>/path` alone (the plan's *first tested
  here* assumption holds), and `goad-check -- ./backend.sh` from the
  backend's directory is accepted.
- No socket, lock or log in any crate or kit directory after the runs
  (`git status` lists only Surfaces); the cases' roots are under
  `temp_dir()`.

**Orchestrator re-measure at `d14d75b`** (2026-10-19): `just check` exit 0;
M11 re-run — `executable` anchoring every relative program — compiled,
redded `run::a_program_with_no_slash_is_found_on_path_not_beside_the_config`
(goad-check `binary` target), restore clean. Kit prose read; one wording nit
for audit in `running.md` (*"the file and your backend can move together"* —
"the file" is the config file, which the sentence leaves implicit).

### PHASE-04 — the exercisers, the repository, and the person's run

Written by the orchestrator, 2026-10-19, at `d14d75b`; executed by a separate
agent up to VH-1, which is the person's. Quotes the plan where it restates it;
says so where it narrows.

**Objective** (`plan.md` PHASE-04, quoted): *"the demo runs from any directory
with its socket in `exercisers/`, nothing in the repository states the old
rule, and a person has seen it."*

**Split of this phase** (the plan's own: *"VH-1 is a person, not an
agent"*): the executor does EX-1..EX-7 and VA-1..VA-4, commits with §Status
PHASE-04 `awaiting VH-1`, and stops. The orchestrator hands the person VH-1's
commands, records what they saw under *VH-1* below, and sets `done`.

**Reading list**
- `plan.md` §Overview *Owed to audit and close* (VA-4 checks it), §PHASE-04
  entire.
- `design.md` §5.3 (repo state: the socket moves), §7 D9, §10 (*Not canon,
  repaired in the slice*: the exerciser and repository rows).
- `research.md` T2-10, T2-S7, X-4.
- `review-design.md` F-9 (`.gitignore`'s kit paragraph), F-17 (the Owed box).
- Files: `exercisers/demo.toml`, `exercisers/shell/backend.sh` (header),
  `justfile` (`emit`, the `package` comment), `.gitignore`,
  `docs/memory/path-flake-ref-breaks-on-demo-socket.md`,
  `crates/goad-shell/tests/integration/round_trip.rs` (`shell_example`'s doc).
- Every earlier phase sheet's *Renames / additions* (for VA-2).

**Surfaces — a closed list.** Anything outside it is a STOP.
- `exercisers/demo.toml` — `command`, `[ingress] path`, and the comments
  naming the socket or the old rule (D9).
- `exercisers/shell/backend.sh` — header only.
- `justfile` — `emit`'s `--socket`, the comment above `emit` if it names the
  socket's location, and the `package` comment's sentence on what a `path:`
  copy meets in the checkout. *(Narrowing note: the plan names "`emit`'s
  `--socket`" and the `package` comment; the comment block above `emit` is
  part of that recipe and is included only where it states the socket path
  or the old rule.)*
- `.gitignore` — the demo entries (`/exercisers/goad-demo.sock`,
  `/exercisers/goad-demo.sock.lock` replace the root pair) and the
  kit-example paragraph's reason (F-9).
- `docs/memory/path-flake-ref-breaks-on-demo-socket.md` — the socket's
  location and the general form's wording; its conclusion stands (X-4).
- `crates/goad-shell/tests/integration/round_trip.rs` — `shell_example`'s
  doc only.
- `docs/slices/015/notes.md` (this sheet, §Status, §Owed, §Open, §Harvest).

**Entry** — checked by the orchestrator:
- EN-1 ✔ PHASE-03 done (`d14d75b`), re-measured above.

**Assumptions**
- *Verified by the orchestrator at `d14d75b`:* `demo.toml` today has
  `command = ["bash", "exercisers/shell/backend.sh"]` and `path =
  "./goad-demo.sock"` — the path's **text** stays, its meaning moves to
  `exercisers/` (D9); the comment *"Relative paths resolve against goad's
  working directory, which `just` sets to the repository root"* and the
  `socat` / `nc` lines (`./goad-demo.sock`) and *"The socket lives in the
  checkout deliberately (D-17)"* state or depend on the old rule. `just demo`
  is `run "exercisers/demo.toml"` and is not touched. `justfile` `emit` passes
  `--socket ./goad-demo.sock`. The `package` comment says
  *"`.claude/worktrees/`, six gitignored worktrees"* — a count; EX-2 asks for
  no number. `.gitignore` lists `/goad-demo.sock`, `/goad-demo.sock.lock`,
  and its kit paragraph says *"relative to where goad is run"*.
- *Root sockets today* (orchestrator, `ls`): the main checkout
  `/home/david/dev/goad` holds `goad-demo.sock` and `goad-demo.sock.lock`;
  the worktree `.claude/worktrees/slice-015` holds neither. `git worktree
  list` shows only these two checkouts. VA-3 re-checks at execution.
- *First tested here:* VH-1 — that a person's launch from `/tmp`, directly and
  through a link, opens a view, binds `exercisers/goad-demo.sock`, and that
  `just emit` reaches it.

**STOP conditions**
- a file outside Surfaces needs to change (in particular `docs/brief.md`,
  closed slices, slice 012's record — never edit those);
- `just -n check` would print a different sequence;
- VA-1 finds a claim about the demo's argv or socket in a file outside
  Surfaces that is now false;
- VA-2 finds a §9 or §7-row case with no test and no recorded rename.

**Exit criteria** (quoted from `plan.md`; none narrowed)
- EX-1 — *"`demo.toml`: `command = ["bash", "shell/backend.sh"]`, `path =
  "./goad-demo.sock"`, every comment true from any directory (the `socat`
  and `nc` lines name the socket by a path that works from the repository
  root, or say where they are run from)."*
- EX-2 — *"`justfile` `emit` passes `--socket exercisers/goad-demo.sock`; the
  `package` comment names `exercisers/goad-demo.sock` and the kit examples'
  `goad.sock` files, which break a `path:` copy the same way, and says
  `.claude/worktrees/` holds gitignored worktrees, with no number. `just -n
  check` prints the same sequence as before (the gate recipe is not
  touched)."*
- EX-3 — *"`.gitignore` as Surfaces say; its comments give the new rule's
  reason."*
- EX-4 — *"`backend.sh`'s header quotes the demo's new argv."*
- EX-5 — *"the memory note says where the socket now lives and that a
  relative socket path resolves against the config file's directory."*
- EX-6 — *"`notes.md` §Owed's box names which checkouts hold a stale root
  `goad-demo.sock` pair (VA-3); the person deletes them after the merge."*
- EX-7 — *"`round_trip.rs::shell_example`'s doc is true of the new demo: what
  it shares with `demo.toml` (`bash` and the script) and what it does not
  (the working directory, an absolute script path)."*
- And `just check` exits 0 on the phase's final commit.

**Verification**
- VA-1 — record both greps and class each hit:
  `git grep -n 'demo.toml\|goad-demo.sock\|shell/backend.sh' -- ':!docs/slices'`
  and `git grep -n -i 'working directory\|started in\|repository root' --
  exercisers justfile .gitignore docs/memory/path-flake-ref-breaks-on-demo-socket.md`.
  (`crates/goad-check/src/args.rs` `DEFAULT_TIMEOUT`'s doc cites
  `demo.toml`'s timeout and stays true.)
- VA-2 — every case named in `design.md` §9 and in `canon-delta.md`'s §7 rows
  resolves to a test in the tree (a grep for each `fn` name, or `cargo test
  -- --list` per target), or to a rename a phase sheet records; gather the
  renames/additions list below for audit's re-pointing.
- VA-3 — the checkouts holding a root `goad-demo.sock` or
  `goad-demo.sock.lock`, listed in §Owed: the main checkout, and each path in
  `git worktree list`.
- VA-4 — `notes.md` §Open carries every item of `plan.md` §Overview *Owed to
  audit and close* as a box (one box per bullet, citing it by its bold lead).
  Add PHASE-01..03's audit-bound Findings as boxes too (the `startup.rs`
  "third such helper" count; the `Config::parse` doc nit; the `running.md`
  wording nit; the `SPEC-001/R-60` citations in code ahead of promotion; the
  `OsStr` finding as a memory candidate).
- VH-1 — the person's (below). Not the executor's.

**Notes for the implementer**
1. `just demo` and every other recipe that is not `emit` or `package`'s
   comment stays as is. `just -n check` before and after: record both are
   identical.
2. `demo.toml` comments are read by a person running the demo: keep them as
   long as they need to be and no longer; say where the `socat`/`nc` lines
   are run from (the repository root, `exercisers/goad-demo.sock`) or make
   them path-independent. Drop *"D-17"* only if it becomes false; if the
   socket still lives in the checkout deliberately, it stays true.
3. The memory note's conclusion (use the bare git form) stands; only the
   location and the general wording change.
4. Do not edit `docs/brief.md`, closed slices, or slice 012's record.
5. Never delete the root sockets yourself: the Owed box is the person's.
6. Cite by symbol, never line number; name, never count.

**Tasks**
- [x] Print `git log -1 --oneline`; record it. §Status PHASE-04 `in progress`.
- [x] `just -n check` recorded (before).
- [x] EX-1 `demo.toml`; EX-4 `backend.sh` header.
- [x] EX-2 `justfile`; EX-3 `.gitignore`; EX-5 memory note; EX-7
      `shell_example`'s doc.
- [x] VA-1, VA-2, VA-3 recorded; EX-6 §Owed updated; VA-4 §Open boxes.
- [x] `just -n check` recorded (after; identical); `just check` exits 0.
- [x] §Status PHASE-04 `awaiting VH-1`; §Harvest; commit; stop.
- [x] *(orchestrator)* VH-1 handed to the person; result recorded; `done`.

**VA-1 — hits, classed** (at the PHASE-04 commit, before it)

First grep (`demo.toml`, `goad-demo.sock`, `shell/backend.sh`, outside
`docs/slices`):
- *Edited here, now true:* `.gitignore` (the two socket entries and their
  comment), `exercisers/demo.toml` (`command`, `path`, the `just emit` / `socat`
  / `nc` lines), `exercisers/shell/backend.sh` header, `justfile` (`demo`
  unchanged; `emit`'s `--socket`; the `package` comment),
  `round_trip.rs::shell_example`'s doc and path,
  `docs/memory/path-flake-ref-breaks-on-demo-socket.md`.
- *Name the file only, still true:* `README.md` (the demo runs against
  `exercisers/demo.toml` and `exercisers/shell/backend.sh`), `docs/roadmap.md`
  (same), `crates/goad-check/src/args.rs` `DEFAULT_TIMEOUT`'s doc,
  `docs/memory/a-backend-exchange-has-no-useful-duration.md` (the 5 s timeout),
  `flake.nix`'s `goad-shot` comment (`cargo run -- exercisers/demo.toml`: a
  path to the config; the socket's place is the config's business now).
- *Claims about argv or the socket in a file outside Surfaces:* none.

Second grep (`working directory`, `started in`, `repository root`, in
`exercisers`, `justfile`, `.gitignore`, the memory note): `demo.toml` lines 6
and 66 and the memory note's parenthesis state the new rule or name the old one
as history; `justfile`'s `path:` sentence is about nix's copy of the working
directory, true. No hit states the old rule as current.

**VA-2 — §9 / §7 cases resolved; renames and additions for audit**

Every test path named in `design.md` §9 and in `canon-delta.md`'s §7 rows
(extracted by script: each backticked or `::`-qualified snake_case name of 13+
characters) resolves to a `fn` of that name under `crates/`. The two that did
not match are not tests: `read_to_string` (a method named in a row's prose) and
`working_directory` (a field). Unresolved cases: none.

Renames: none. Additions, for audit's re-pointing:
- PHASE-01: none (every case ships under §9's name in §9's file).
- PHASE-02: `error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable`
  (VT-9, an addition to §9); `working_directory.rs::logging_script`,
  `config_in` (case-written backend and config helpers).
- PHASE-03: `process.rs::check_in`; `run.rs::root`, `labelled_backend`,
  `labels` (helpers, not cases).

**VA-3 — root sockets**

`git worktree list` shows two checkouts. `/home/david/dev/goad` holds
`goad-demo.sock` and `goad-demo.sock.lock`;
`/home/david/dev/goad/.claude/worktrees/slice-015` holds neither.

**VH-1 — the person's run** (recorded by the orchestrator)

Run by the person on 2026-10-19, on the worktree's build at `bdc93f2`, in
the dev shell (`nix develop . --command …`), from the commands handed over in
nushell syntax:

1. From `/tmp`, goad on `<worktree>/exercisers/demo.toml` by its absolute
   path: a view appeared; quit.
2. From `/tmp`, goad through `/tmp/goad-015-demo.toml →
   <worktree>/exercisers/demo.toml`: a view appeared; left running.
3. From the worktree root, `just emit reddit-watcher reddit-opened`: the
   running host's view was replaced by the *event arrived* view (AC-5).
4. `exercisers/goad-demo.sock` and its lock existed; the root held no socket.

The person's words: *"yeah that was all expected results. the #3 replaced the
#2 window with an event arrived, etc"*. Nothing §9 did not predict. For
`audit.md` §Evidence (AC-4, AC-5). If an audit repair touches
`goad_shell::config` or the spawn, this run is repeated on the repaired build
(`plan.md` §Owed).

**Decisions taken during execution**
- *`demo.toml` says where the raw forms are run from* (the repository root,
  socket `exercisers/goad-demo.sock`) rather than going path-independent: the
  worked wire examples stay short, and `just emit` runs from the root anyway.
- *D-17 stays:* the socket still lives in the checkout deliberately.
- *The `package` comment names the kit examples' `goad.sock` files* as
  "once an example has run": the kit's event-driven examples bind them beside
  their configs (PHASE-03's rule), where the `.gitignore` already lists them.
- *`.gitignore` and `demo.toml` cite `SPEC-003/R-17` / `SPEC-001/R-60`* ahead
  of promotion, as the code already does (PHASE-01/02 Findings).

**Findings**
- `flake.nix`'s `goad-shot` comment still reads true; no edit (outside
  Surfaces, and nothing false).
- `just -n check` is byte-identical before and after.

## Owed

Manual steps no phase's code reaches; each a box, ticked with who did it and
when.

- [ ] Delete the stale `goad-demo.sock` and `goad-demo.sock.lock` at the
  repository root of **every checkout** — the person's main checkout, and each
  worktree under `.claude/worktrees/` — once `.gitignore` drops their entries
  (the `.lock` shows as untracked otherwise: `research.md` T2-S7;
  `review-design.md` F-17). The person deletes them; an agent lists which
  checkouts hold them.
  **Held (VA-3, PHASE-04):** `/home/david/dev/goad` (the main checkout) holds
  `goad-demo.sock` and `goad-demo.sock.lock`; the `slice-015` worktree holds
  neither, and `git worktree list` shows no other checkout. Delete after the
  merge, in the main checkout only.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-10-19 · PHASE-04 (to VH-1) · see the PHASE-04 commit

### Produced
<!-- What now exists: modules, contracts, docs. -->
- `Command::working_directory`, `Command::executable`, `config::anchored`;
  `Config::load` canonicalizes and anchors, `Config::parse(text, directory)`.
- Cases: `config.rs` VT-1/2/3, `working_directory.rs` (new target module),
  `startup.rs::listener` AC-5, `goad-emit` AC-5 (binary and unit),
  `ingress.rs` length pin. `claim`'s doc names its helpers, no count.
- `ProcessBackend::exchange` spawns `Command::executable()` in the working
  directory with `PWD` set; `BackendError::Spawn { executable,
  working_directory, source }`, parenthesis only with a directory.
- Cases: `working_directory.rs` spawn cases (VT-1..VT-6) and their
  case-written backends (`logging_script`, `config_in`); `error.rs` VT-9
  (an addition to §9); `transport.rs` VT-10. D10: the TypeScript README's
  config is relative to its directory, `rooted_at_the_workspace` gone. AC-2:
  `examples.rs::run_checker` starts in the hand-out root.
- Pins `run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config`
  and `run.rs::the_argv_form_runs_a_relative_program_from_the_caller_s_directory`
  (M11, M12 red them); `process.rs::check_in`; `run.rs::root` held by
  `claim` (kind `checker root`).
- Kit states the new rule: `running.md` *Name your command* and the
  `ingress.path` row (107-byte limit, `$XDG_RUNTIME_DIR` remedy),
  `checking.md` (`--config` vs the argv form), `events.md` (`goad-emit`'s
  socket); each example's config comment and README *Run your own copy*;
  `watch.sh` header. No draft id cited in `kit/`.

- `demo.toml` runs from any directory: `["bash", "shell/backend.sh"]`,
  socket `exercisers/goad-demo.sock` (`.gitignore` follows); `just emit` passes
  that path. Memory note `path-flake-ref-breaks-on-demo-socket` updated.

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->
- `Path` equality ignores interior `.`; compare `OsStr` for byte claims
  (PHASE-01 Findings).
- `arg0` reaches a native program and not a script — M8 reds VT-4 alone, VT-3
  stays green (PHASE-02 mutation table): D5's premise, measured.
- A spawn case's log lives inside its claimed `directory`; no second
  claimed kind needed (PHASE-02 Decisions).
- "Judged" is too weak a pin for a spawn location: a spawn failure is judged
  too (exit 1). Pin on accepted (PHASE-03 Decisions, VT-2).

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

*From `plan.md` §Overview, Owed to audit and close (VA-4), one box per bullet:*
- [ ] **Promote `canon-delta.md`** — SPEC-001 Changes 1-3 and SPEC-003 Changes
  1-4, with the person's endorsement, recorded in `audit.md`'s Reconciliation
  table; after 012's promotion by preference, not as a gate.
- [ ] **Re-point the §7 test citations** in both rows to the symbols that
  shipped (see VA-2's additions) and confirm each resolves.
- [ ] **The kit's citation of the new ids** — once promoted, decide with the
  person whether `running.md` and `events.md` cite R-60 / R-17.
- [ ] **Record the person's run** (VH-1) in `audit.md` §Evidence; re-run VH-1
  on the repaired build if an audit repair touches `goad_shell::config` or the
  spawn.
- [ ] **Slice 012's drift row** — already in `docs/slices/012/plan.md` §Owed;
  audit names the hand-off, does not edit 012's record.
- [ ] **The design's residues**, as follow-up candidates: `main.rs::start`
  held by review and AC-4 only (D11); `/dev/stdin` or process-substitution
  config paths refused (D4); a relative `PATH` entry now means the config's
  directory (§5.5).
- [ ] **The Owed box** (stale root `goad-demo.sock` pair) — ticked by the
  person after the merge.
- [ ] **Disposition every other §Open candidate** at close.

*Audit-bound findings from PHASE-01..03:*
- [ ] `startup.rs::listener::socket_path`'s doc says "the third such helper in
  this test binary" — a count the never-count rule forbids.
- [ ] `Config::parse`'s doc reads "is the only caller that need" (nit).
- [ ] `running.md` *"the file and your backend can move together"* — "the file"
  is the config file, left implicit (nit).
- [ ] `SPEC-001/R-60` is cited in `process.rs`, `error.rs` and now
  `exercisers/demo.toml` (and `SPEC-003/R-17` in `.gitignore`) ahead of
  promotion; confirm none reaches `main` unpromoted.
- [ ] Memory candidate: a path assertion about bytes must compare `OsStr`, not
  `Path` (`Path` equality ignores an interior `.`).

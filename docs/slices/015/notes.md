# Notes — Slice 015

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | done | 2026-10-19 |
| PHASE-02 | pending | 2026-10-19 |
| PHASE-03 | pending | 2026-10-19 |
| PHASE-04 | pending | 2026-10-19 |

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

## Owed

Manual steps no phase's code reaches; each a box, ticked with who did it and
when.

- [ ] Delete the stale `goad-demo.sock` and `goad-demo.sock.lock` at the
  repository root of **every checkout** — the person's main checkout, and each
  worktree under `.claude/worktrees/` — once `.gitignore` drops their entries
  (the `.lock` shows as untracked otherwise: `research.md` T2-S7;
  `review-design.md` F-17). The person deletes them; an agent lists which
  checkouts hold them.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-10-19 · PHASE-01 · see the PHASE-01 commit

### Produced
<!-- What now exists: modules, contracts, docs. -->
- `Command::working_directory`, `Command::executable`, `config::anchored`;
  `Config::load` canonicalizes and anchors, `Config::parse(text, directory)`.
- Cases: `config.rs` VT-1/2/3, `working_directory.rs` (new target module),
  `startup.rs::listener` AC-5, `goad-emit` AC-5 (binary and unit),
  `ingress.rs` length pin. `claim`'s doc names its helpers, no count.

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->
- `Path` equality ignores interior `.`; compare `OsStr` for byte claims
  (PHASE-01 Findings).

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

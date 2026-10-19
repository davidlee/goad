# Research — Slice 015

**Producers:** the design agent, by reading and grepping the tree, plus one
spike in its scratchpad (outside the repo, not committed; its results are
quoted as T2-S rows).
**As of:** 2026-10-19 · `b194bac`

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- unmarked — researcher claim: cited, not checked.

Every row below was read at its site by the design agent; ✓ marks those the
design load-bears.

## Citation forms

Canon by id and section. Code by `path::symbol`, never by line number.

## Thread 1 — governing canon

### Binding

- T1-1 ✓ **SPEC-001/R-36**: the command is an argv vector, no shell; an empty
  vector or empty program is refused at load. Says nothing about where the
  command runs or what a relative path means. §7's R-36 row names
  `round_trip.rs::the_bash_backend_completes_the_same_round_trip` and
  `config.rs::an_empty_command_is_rejected_because_there_is_nothing_to_spawn`.
- T1-2 ✓ **SPEC-001/R-44**: *command not spawnable* is its own error.
  `BackendError::Spawn` carries it; `BackendError::fault` files it under
  `AtFault::Configuration` (that method cites 012's unpromoted R-59).
- T1-3 ✓ **SPEC-001 §6.4** states the transport (one process per exchange,
  argv, no warm process) and nothing about a working directory. SPEC-001 never
  uses the word *configuration*; its only config value is *the configured
  timeout* (R-41).
- T1-4 ✓ **SPEC-001 ids**: the highest in the document is R-58. Slice 012's
  unpromoted `canon-delta.md` claims **R-59** for SPEC-001, and code already
  cites it. The next free id is therefore **R-60**.
- T1-5 ✓ **SPEC-003 §6.1**: *the socket's path comes from the host's
  configuration and has no default*. Nothing says what a relative path is
  relative to. Highest SPEC-003 id: R-16. No pending delta (010, 011 closed;
  012's names SPEC-001, SPEC-004, POL-001, ADR-003) claims a SPEC-003 id.
- T1-6 ✓ **SPEC-003/R-4**: any failure to bind is a startup failure naming the
  path and what was found. **R-3**: the lock file beside the socket is opened
  before the bind. **R-5**: neither is ever unlinked.
- T1-7 ✓ **ADR-001**: configuration and the transport are stratum 2
  (`goad-shell`). Nothing here touches `goad-semantics`.

### Checked, not applicable

- **SPEC-002** (scheduling): no path, no spawn.
- **SPEC-004** (exit status): a configuration that cannot be read is exit 2
  under R-3/R-4's *never started*; a spawn failure is an exchange outcome, not
  an exit. Both unchanged by this slice. 012's SPEC-004 drafts (`goad-emit`
  R-10: *no configuration or one it cannot read*) agree and are not cited.
- **POL-001**: the gate command block is unchanged.
- **ADR-003/004/005**: no crate added, no scheduling or envelope change.

### Amendment candidates

- SPEC-001: a requirement for where the backend runs (new R-60), §6.4 prose,
  §7 row.
- SPEC-003: a requirement for what a relative `ingress.path` is relative to
  (new R-17), §6.1 prose, §7 row.

## Thread 2 — code map

### Hotspots

- `crates/goad-shell/src/config.rs` — `Config::load`, `Config::parse`,
  `Command`, `ingress_config`.
- `crates/goad-shell/src/backend/process.rs` — `ProcessBackend::exchange`,
  the one `tokio::process::Command::new`.
- Callers of `Config::parse`, whose signature changes (T2-6).
- Tests that encode the old rule (T2-9).
- Prose: `kit/skills/goad-backend/reference/{running,checking}.md`, the
  examples' `config.toml` and README, `exercisers/demo.toml`,
  `exercisers/typescript/README.md`, `exercisers/shell/backend.sh`'s header,
  `justfile` (`emit`, the `package` comment), `.gitignore`,
  `docs/memory/path-flake-ref-breaks-on-demo-socket.md`.

### Cited facts

- T2-1 ✓ `Config::load` reads the file with `read_to_string` and calls
  `Config::parse(&text)`; the path is not kept. Read failure is
  `ConfigError::Read`. Its callers split `Read` from the rest where the path is
  in hand: `goad` `main.rs::start`, `goad-emit` `main.rs::socket_path`,
  `goad-check` `main.rs::configuration`.
- T2-2 ✓ `Command { program, arguments }`, `Command::new`,
  `Command::from_argv` (its callers: `Config::parse`, and `goad-check`'s argv
  form in `args.rs`). Derives `PartialEq`; `goad-check`'s `args.rs` tests
  compare parsed commands with `Command::new` values.
- T2-3 ✓ `ProcessBackend { command, timeout }`; `ProcessBackend::new(command,
  timeout)`. `exchange` builds `Command::new(&self.command.program)`, `.args`,
  piped stdio, `kill_on_drop`. No `current_dir`.
  `transport_shape.rs::the_only_spawn_is_the_child` greps
  process.rs for the text `let mut command = Command::new(`.
- T2-4 ✓ Every config-driven `ProcessBackend` is built the same way,
  `ProcessBackend::new(config.backend.command.clone(), config.backend.timeout)`:
  `goad` `main.rs::start`, `goad-check` `main.rs::prepare`,
  `event_loop_schedule/scheduling.rs`, `event_loop/closing.rs`. So a working
  directory carried **inside `Command`** reaches every one of them unedited.
- T2-5 ✓ `goad-check`'s argv form builds `Config { backend: BackendConfig {
  command, timeout }, … ingress: None }` in `main.rs::configuration`, with a
  `Command` from `from_argv`. It has no file.
- T2-6 ✓ `Config::parse` callers outside `config.rs`:
  `goad-shell/tests/integration/round_trip.rs::the_readme_s_own_config_loads_and_runs_the_example`,
  `goad-shell/tests/integration/host.rs::host`,
  `goad/tests/renderer/startup.rs` (`unparseable`, a syntax error only),
  `goad-check/tests/kit/tagged.rs::check` (`goad:config` fences).
- T2-7 ✓ `ingress_config` turns `ingress.path` into `PathBuf::from` as
  written. `goad` `startup::listener` binds `config.ingress` as given;
  `goad-emit` `socket_path` returns `ingress.path` as given; `--socket` is used
  as given.
- T2-8 ✓ `ingress::bind` opens the lock (`hold`) before `UnixListener::bind`;
  `BindFault::Unbindable` already documents *a path longer than `sun_path`
  admits*, and its `Display` is `could not be bound: {inner}`. No test reaches
  the length case (grep `sun_path|SUN_LEN` over `crates/`: the doc comment
  only).
- T2-9 ✓ Tests that encode or depend on the old rule:
  - `round_trip.rs::the_readme_s_own_config_loads_and_runs_the_example`
    rebases `./` arguments onto the workspace with `rooted_at_the_workspace`,
    because the README's command is repo-root relative.
  - `goad-emit` `main.rs::tests::a_configured_ingress_path_is_what_is_written_to`
    asserts `./from-config.sock` comes back as written.
  - `goad-check/tests/kit/examples.rs::run_checker` starts the checker **in**
    the example's copy, which is the only place the old rule worked.
  - Unaffected, checked: `transport.rs::a_command_that_does_not_exist_fails_to_spawn`
    (`./no-such-backend-exists` through `Command::new`, no directory: still
    `NotFound`); `harness.rs::example`, `round_trip.rs::shell_example`,
    `failure_matrix.rs` (absolute paths); `host.rs` (never spawns);
    `goad-check/tests/binary/*` (argv form, absolute programs;
    `loadable.toml` is never spawned); `exit_codes.rs` (absolute socket,
    `command = ["true"]`); `standalone.rs` (runs `git` only);
    `goad/tests/renderer/startup.rs` `listener::*` (build `IngressConfig`
    directly, absolute paths).
- T2-10 ✓ `exercisers/demo.toml`: `command = ["bash",
  "exercisers/shell/backend.sh"]`, `[ingress] path = "./goad-demo.sock"`, both
  repo-root relative; comments show `./goad-demo.sock` for `socat`/`nc`.
  `justfile` `emit` passes `--socket ./goad-demo.sock`; its `package` comment
  names `goad-demo.sock` at the root. `.gitignore` anchors `/goad-demo.sock`
  and `/goad-demo.sock.lock`. `exercisers/typescript/README.md` shows `command
  = ["deno", "run", "-A", "./exercisers/typescript/backend.ts"]` and says *a
  relative path is resolved against goad's working directory*.
  `exercisers/shell/backend.sh`'s header quotes the demo's argv.
- T2-11 ✓ Kit: `running.md` *SPEC-001/R-36* says `command` is **not** resolved
  against the config's directory and to make paths absolute; its config table
  says `ingress.path` is *relative to the directory goad is started in*.
  `checking.md` says the same for `goad-check`. Example configs and READMEs
  (`breadcrumbs`, `downloads-triage`, `focus-check`) say *run goad from this
  directory* and *copy out and make absolute*. Their `command`s are relative
  arguments (`backend.ts`, `backend.sh`, `backend.py`) and their sockets
  `./goad.sock` — **already config-relative in meaning**, so no config value
  changes, only prose.
- T2-12 ✓ 012's `design.md` §5.2.1 and its kit section (F-36) state the old
  rule as a design fact. 012 is at stage *phase*, PHASE-10 pending.

### Spike (scratchpad, Rust 1.100 beta, Linux 7.2)

- T2-S1 ✓ `std::process::Command::new("./b.sh").current_dir(d)` runs
  `d/b.sh` on Linux (the child changes directory before `exec`). Rust's
  `Command::current_dir` documentation calls this case platform-specific and
  unstable; tokio's `Command` wraps std's.
- T2-S2 ✓ A shebang script's `$0` is the path given to `exec` — `./b.sh` or
  the absolute one — whatever `arg0` says. `arg0` is visible only to a native
  program (for example `sh -c '…'` with no operand, whose `$0` is argv[0]).
  `tokio::process::Command::arg0` exists (tokio 1.53, unix).
- T2-S3 ✓ A bare program (`sh`) is still found on `PATH` with `current_dir`
  set.
- T2-S4 ✓ A `current_dir` that does not exist fails the **spawn** with
  `NotFound`.
- T2-S5 ✓ `UnixListener::bind` and `UnixStream::connect`: 107 bytes succeed;
  108 and over fail with `InvalidInput`, *path must be shorter than SUN_LEN*.
- T2-S6 ✓ `std::fs::canonicalize` of a missing file is `NotFound`; it resolves
  every link, file and directory components alike.
- T2-S7 ✓ `git status` lists no socket file, but does list the regular
  `.lock` file beside it.

## Cross-thread findings

- X-1 Carrying the directory **in `Command`** reaches every config-driven spawn
  without touching `main.rs` in either binary (T2-4), and leaves the argv form
  (T2-5) and every test-built `Command::new` (T2-9) on today's behaviour.
- X-2 The kit examples already *mean* config-relative (T2-11); the rule change
  makes their configs true from anywhere and their prose false. Only
  `exercisers/demo.toml` and the TypeScript README change meaning (T2-10).
- X-3 The id 012 claims (T1-4) is the slice-015 collision to avoid: R-60, not
  R-59.
- X-4 Moving the demo socket into `exercisers/` keeps it inside the working
  tree, so a `path:` flake ref still copies it and still fails (T2-S7: the
  socket is invisible to git either way). The memory note's *conclusion*
  holds; its *location* text and the `justfile` comment change.

## Design-input deltas

- Resolve a relative program containing `/` in the host (T2-S1), and keep
  argv[0] as written with `arg0` (T2-S2), so *argv is never rewritten* is true
  of what the child receives.
- No new length check: R-4's `Unbindable` already refuses it (T2-8, T2-S5);
  the slice pins it with a test.
- `canonicalize` failure is a `ConfigError::Read`: the same subjects (missing,
  unreadable, dangling link) as `read_to_string`'s, same message.

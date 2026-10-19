# Review — design — Slice 015

**Subject:** design — `docs/slices/015/design.md`, `research.md` and
`canon-delta.md` at `e231462` or later
**Reviewer:** fresh agent, Opus
**Opened:** 2026-10-19
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

**Round 1** — 2026-10-19 — the whole design, against canon and the tree.

Written by the orchestrator before the review. The invariants (`CLAUDE.md`)
hold throughout; these are the surfaces to attack:

- **The resolution door.** Where the config's directory is found and carried
  in the canonical types, and whether anything past that door still depends
  on the directory goad was started in. Every caller of the changed loading
  and parsing functions, across all three binaries and the tests.
- **Links and failures.** What resolving the config path does to every path a
  message names (SPEC-004's startup lines, SPEC-003/R-3 and R-4's), and every
  way resolution can fail.
- **The spawn.** How the program is located once the working directory
  changes: relative, absolute, bare, and `PATH` itself. What the child sees
  as its own name. Whether the stated mechanism is the one the platform
  actually runs.
- **The socket.** The resolved path, its lock sidecar, its length bound, and
  whether host and `goad-emit` provably agree.
- **The canon wording.** Each MUST is testable and has a verification row
  that would fail if the rule broke. Ids, placement, and the CLAUDE.md
  citation rules (by symbol, never count). Whether R-60 and R-17 say more or
  less than the design does.
- **The test plan.** Each AC is held by a case that the regression it guards
  would not survive (no proxies); negative controls compile; tests that
  relied on the old rule are all found.
- **Collateral.** The demo, `justfile`, `.gitignore`, the kit, the
  exercisers, slice 012's record, and anything the design's scope missed.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | doc-wrong | verified |
| F-2 | major | fix-now | verified |
| F-3 | major | doc-wrong | verified |
| F-4 | minor | doc-wrong | verified |
| F-5 | minor | fix-now | verified |
| F-6 | minor | tolerated | verified |
| F-7 | minor | fix-now | verified |
| F-8 | minor | doc-wrong | verified |
| F-9 | minor | doc-wrong | verified |
| F-10 | minor | doc-wrong | verified |
| F-11 | minor | fix-now | verified |
| F-12 | nit | doc-wrong | verified |
| F-13 | nit | doc-wrong | verified |
| F-14 | nit | fix-now | verified |
| F-15 | nit | doc-wrong | verified |
| F-16 | nit | doc-wrong | verified |
| F-17 | nit | fix-now | verified |
| F-18 | minor | fix-now | verified |
| F-19 | nit | doc-wrong | verified |
| F-20 | nit | doc-wrong | verified |
| F-21 | nit | doc-wrong | |

### Round 1 — raiser's notes

Raiser: fresh agent, Opus, at `4d53458`. Spike `R1-S` (scratchpad, not
committed; Rust 1.100 beta, tokio 1.53.1, Linux 7.2, `sh` → bash 5.3): a
directory `anchor/` holding a shebang `b.sh` that prints `$0` and `pwd -P`, a
link `shell` → `sh`, and `sub/goad-decoy`; spawned with `std::process::Command`
from a different working directory. Its rows, cited below:

- R1-S1 `Command::new(<anchor>/b.sh).arg0("./b.sh").current_dir(anchor)` →
  the script's `$0` is the **absolute** path. `Command::new("./b.sh")
  .current_dir(anchor)` → `$0` is `./b.sh`.
- R1-S2 `<anchor>/shell -c 'echo $0'` with `arg0("./shell")` → `./shell`;
  without `arg0` → the absolute path.
- R1-S3 `Command::new("goad-decoy").env("PATH", "sub:/usr/bin:/bin")
  .current_dir(anchor)` runs `anchor/sub/goad-decoy`. With `PATH=":/usr/bin"`,
  `Command::new("b.sh")` runs `anchor/b.sh`.
- R1-S4 `anchor.join(p).components().collect::<PathBuf>()`: `./goad.sock` →
  `<anchor>/goad.sock`; `a/./b` → `a/b`; `../x` kept; `a//b/` → `a/b`.
- R1-S5 `fs::canonicalize("/dev/stdin")` and `("/proc/self/fd/0")`, stdin a
  pipe → `NotFound`.
- R1-S6 `current_dir("/nonexistent")` → spawn `NotFound` (T2-S4 reproduced).
- R1-S7 bash `pwd` with a stale `PWD` in its environment reports the real
  directory; python's `os.environ["PWD"]` keeps the stale value.

**Checked and sound** (each read at its site, or re-run):

- T2-S1, T2-S2, T2-S4 reproduce (R1-S1, R1-S2, R1-S6). T2-S3 holds: a bare
  name is found on `PATH` with `current_dir` set.
- `tokio::process::Command::arg0` exists in the locked tokio (1.53.1, unix),
  so D5's three lines compile against what is pinned.
- T2-S5 for both sides: tokio's `UnixListener::bind` builds its address with
  std's `SocketAddr::from_pathname` (tokio `net/unix/listener.rs`
  `UnixListener::bind`), and `ingress::client` uses std's `UnixStream`, so
  the host and `goad-emit` meet the same length rule and the same error.
- `anchored` as specified (join, then `Path::components`) drops interior `.`
  and keeps `..` (R1-S4). D7's byte argument holds.
- `ingress::bind`'s order — `reclaim` → `probe` → `hold` (opens the lock) →
  `UnixListener::bind` — means an over-long path reaches
  `BindFault::Unbindable` after the lock file exists, which §5.5's long-path
  edge states correctly; the lock path itself has no `sun_path` limit
  (`ingress::lock_path`'s doc).
- `goad-emit` on an over-long path: `client::send` → `SendFault::Unreachable`
  → `render::fault_line` names the path, exit 2. §5.2's table row holds (the
  line reads *nothing is listening at*, which is imprecise but pre-existing).
- T2-4: every config-driven `ProcessBackend::new` passes
  `config.backend.command` (`goad` `main.rs::start`, `goad-check`
  `main.rs::prepare`, `event_loop/closing.rs`, `event_loop_schedule/
  scheduling.rs`). No struct-literal `Command { … }` exists outside
  `config.rs`, and only `process.rs` reads `program` / `arguments`, so D1's
  *no edit to either `main.rs`* holds and adding a field breaks no caller.
- T2-6: the `Config::parse` callers outside `config.rs` are exactly those
  named; `Config::load` callers are exactly `goad` `main.rs::start`,
  `goad-emit` `main.rs::socket_path`, `goad-check` `main.rs::configuration`,
  and `config.rs`'s own test.
- `transport_shape.rs::the_only_spawn_is_the_child` (in
  `crates/goad-shell/tests/shape/`) still matches §5.2's spawn code, and
  `::nothing_returns_between_the_spawn_and_the_cleanup_budget`'s region is
  untouched by it.
- I-3 by construction: both binaries take the socket path from
  `Config::load` → `ingress_config` → `anchored`, one function.
- Ids: SPEC-001's highest is R-58 and SPEC-003's R-16; SPEC-003 §6.1's first
  sentence is verbatim as Change 2 quotes it. R-60 and R-17 are free.
- AC-2's red claim: `examples.rs::run_checker` starts in the copy today, and
  each example's command names a sibling file, so starting at the hand-out
  root is red under the old rule.
- AC-5's `goad-emit` unit expectation must be canonical (§9 says so), which
  matters on a platform whose `temp_dir` is a link.

### F-1 — R-60 and I-2 promise that a script program receives its name as written; the design gives it the absolute path

**Severity:** major (touches the person's endorsed canon wording)
**Location:** `canon-delta.md` SPEC-001 Change 1 (R-60, *the program receives
its own name as written*); `design.md` §5.5 I-2, D5

**Expected:** a MUST-level statement in R-60, and I-2 (*the child's argv is
exactly `[program, arguments…]` as written*), true of what the backend
observes for every program R-60 governs.
**Observed:** for the commonest relative program with a `/` — a shebang
script, `["./backend.sh"]` — the kernel hands the interpreter the path given to
`exec`, not argv[0]. D5 executes `anchored(dir, program)`, so the script's `$0`
is `/<anchor>/backend.sh`; under today's rule it is `./backend.sh`. D5 itself
says so (*a script's `$0` becomes the absolute path either way*), but the canon
sentence and I-2 are unconditional. The clause is true only of a native
program, which is why §9 has to observe it through `sh -c` with no operand.
**Evidence:** R1-S1 (`$0` absolute with `arg0("./b.sh")` set), R1-S2 (a native
program sees `arg0`); research T2-S2; `design.md` D5's last sentence.

**Disposition:** doc-wrong
**Response:** Correct, and the claim was canon's. `arg0` is dropped. R-60 now says a relative program with a `/` is executed as the path joined onto the configuration's directory and that the joined path is the name it receives, a script and a native program alike; §6.4's paragraph gives the reason (a script's interpreter is handed the exec path whatever argv[0] says). I-2 now says the arguments are as written and the name is the executed path. D5 rewritten with `arg0` among the rejected options; research's design-input delta corrected. Person confirmed (`design-log.md` 2026-10-19, *design review round 1 dispositions*).

**Outcome:** verified — round 2. R-60 now says the joined path is the name the program receives, a script and a native program alike, with F-1 cited in its *Why*; I-2 is restated; D5's `arg0` is gone from §5.2. Read in `canon-delta.md` Change 1, `design.md` §5.2 and §5.5. The new wording's native half is unheld: F-18.

### F-2 — Nothing verifies R-60's *by the host's own resolution*; every §9 case passes with the platform's resolution

**Severity:** major
**Location:** `canon-delta.md` SPEC-001 Change 1 (R-60's program clause) and
Change 3 (R-60's §7 row); `design.md` §9 AC-1 rows, D5

**Expected:** the brief's rule that each MUST has a verification row that fails
if the rule breaks. R-60 requires a relative program with a `/` to be executed
*by the host's own resolution and not by whatever a platform does*.
**Observed:** on Linux the platform's route — `Command::new(&program)` with
`current_dir` — runs the same file (T2-S1). Revert `exchange` to that, keeping
`arg0` and `current_dir`, and every §9 case stays green:
`a_relative_program_with_a_slash_resolves_beside_the_config` asserts only that
the backend answers; `the_backend_receives_its_program_as_written` sets argv[0]
to `program` either way; `config.rs::tests::the_executable_is_anchored_only_…`
tests the pure function, not that `exchange` calls it; `transport_shape`'s
guard matches `let mut command = Command::new(` with any argument. The clause
is held by review alone, and the row does not say so.
**Evidence:** R1-S1 gives a behavioural witness the plan can use: a shebang
script's `$0` is the path `exec` received, absolute under D5 and `./b.sh` under
the platform route. Today's §9 does not assert it, and F-1's wording would
forbid asserting it.

**Disposition:** fix-now
**Response:** `working_directory.rs::a_relative_program_with_a_slash_is_executed_as_the_anchored_path` (renamed from `…_resolves_beside_the_config`) asserts the script's `$0` is `<canonical anchor>/backend.sh`, which reds under `Command::new(relative)` + `current_dir` (R1-S1). Named in `design.md` §9 and in R-60's §7 row, which now says this is what tells the host's resolution from the platform's. `the_backend_receives_its_program_as_written` is removed with `arg0`.

**Outcome:** verified — round 2. `a_relative_program_with_a_slash_is_executed_as_the_anchored_path` asserts the script's `$0` is the anchored path, which R1-S1 shows is `./b.sh` under the platform's route, so the case is red under the regression it guards. R-60's §7 row says so.

### F-3 — The directory-component half of the linked-config case cannot discriminate, and canon's §7 row claims it does

**Severity:** major
**Location:** `design.md` §9,
`working_directory.rs::a_linked_config_runs_its_backend_beside_the_target`;
`canon-delta.md` SPEC-001 Change 3 (*through a link in a directory component,
each with a decoy backend beside the link that answers differently*)

**Expected:** a decoy that a non-canonicalizing implementation would run, so
the case is red under it (*a link-directory implementation: decoy*).
**Observed:** a process's working directory is an inode, not a string, so
`chdir` through a directory link lands in the target directory. Load
`/t/a/link/config.toml` with `link → /t/real`: an implementation that uses
`path.parent()` as given spawns in `/t/a/link`, which is `/t/real` — the real
backend runs, `pwd -P` prints `/t/real`, and the case is green. There is no
place *beside the link* that such an implementation would look: the
directory holding the link (`/t/a`) is nobody's candidate. Joined paths fare
the same (`/t/a/link/../x` resolves physically). What differs is only the
**string** — `working_directory`, the anchored `ingress.path` and its byte
length — so the observable witness is the loaded `Config` value, not the
spawn. The file-link half is sound.
**Evidence:** `design.md` §5.4 (*cwd = /real/dir*); T2-S6; reasoning above,
checkable with one `ln -s` and `pwd -P`.

**Disposition:** doc-wrong
**Response:** Correct. The spawn case keeps only the file-link half, which discriminates. The directory-link half moves to `working_directory.rs::a_config_reached_through_a_linked_directory_is_anchored_at_the_target`, asserting the loaded `Command`'s `working_directory` (and the anchored `ingress.path`) is the target's — the string anchor `PWD`, the socket path and its length see; red under `path.parent()` as given. R-60's §7 row says the directory-link half is held on the loaded value and why it cannot be held at the spawn.

**Outcome:** verified — round 2. The directory-link half is now `a_config_reached_through_a_linked_directory_is_anchored_at_the_target`, held on the loaded value (`working_directory`, anchored `ingress.path`), red under `path.parent()` as given; the §7 row says why it is not observable at the spawn. The file-link case keeps its decoy.

### F-4 — R-60's §7 row says the replaced behaviour fails *each* case; the `PATH` case is a pin

**Severity:** minor
**Location:** `canon-delta.md` SPEC-001 Change 3 (*so the behaviour R-60
replaces fails each one*); `design.md` §9 row for
`a_program_with_no_slash_is_never_looked_for_beside_the_config`

**Expected:** a canon verification row whose description of its cases is true.
**Observed:** §9 marks the `PATH` case *pin*: under today's spawn (no
`current_dir`) a bare `goad-decoy` is `NotFound` too, so the old behaviour
passes it. F-3's directory-component half is the same. The row's quantifier
covers both.
**Evidence:** `design.md` §9 (*pin; red under an implementation that anchors
bare names*).

**Disposition:** doc-wrong
**Response:** R-60's §7 row no longer quantifies over every case: it lists the cases red under the old spawn, then names the `PATH` case as a pin and says what reds it. The directory-link case is described separately (F-3).

**Outcome:** verified — round 2. R-60's §7 row now separates *red under the spawn R-60 replaces* from *a pin*, and names the pin.

### F-5 — A relative or empty `PATH` entry finds a bare program beside the config; the `PATH` test depends on the inherited environment

**Severity:** minor
**Location:** `design.md` §5.5 *Edge — bare program*, §9
`a_program_with_no_slash_is_never_looked_for_beside_the_config`;
`canon-delta.md` SPEC-001 Change 2 (*a bare name on `PATH`, as a shell would
find it*)

**Expected:** a test named *never looked for beside the config* that cannot
fail for reasons outside the host.
**Observed:** the lookup happens after the child's `chdir`, so `PATH=.:…`,
`PATH=…:` (a trailing colon is a common shell-profile slip) or any relative
entry resolves against the anchor. Under the old rule the same entry meant
goad's start directory. The design accepts this (*not defended*) but the test
inherits `PATH` and so passes or fails by the developer's profile; nothing in
§9 says it pins `PATH`. The test name and §6.4's *as a shell would find it*
state more than the mechanism does.
**Evidence:** R1-S3 (both a relative entry and an empty entry resolve in the
anchor).

**Disposition:** fix-now
**Response:** The case is renamed `a_program_with_no_slash_is_found_on_path_not_beside_the_config` and sets `PATH` itself to one absolute directory holding a `goad-decoy` that answers *from PATH*, with another beside the config answering *beside*; the inherited `PATH` is never consulted. §5.5's edge and §6.4's paragraph now state that the lookup follows the change of directory, so a relative or empty `PATH` entry means the config's directory; *as a shell would find it* is gone. The kit's `running.md` says so (§10).

**Outcome:** contested — round 2. **Contested.** The rename and the prose hold (§5.5 edge, §6.4, `running.md` in §10). The case cannot be built as placed: it sits in `goad-shell`'s `integration` target and "sets `PATH` itself", but neither `ProcessBackend` nor `config::Command` offers an environment, so the only way to give the child a `PATH` is to change the test process's own. `clippy.toml` disallows `std::env::set_var`, edition 2024 makes it `unsafe`, and it is process-global: sibling cases in the same binary spawn `bash` and `python3` by bare name (this file's own `a_relative_argument_…` and `the_backend_s_pwd_…`), and the case's `PATH` holds only `goad-decoy`, so they would fail when run in parallel. A mechanism that works: a test target of its own holding the one case (the `event_loop_*` precedent), with the `set_var` under a reasoned `#[expect]`; or the built-binary tier, setting `PATH` on the `goad-check --config` child (`Command::env` on a process, no global mutation). Either moves the case, and R-60's §7 row cites it by its `working_directory.rs` path, so the row moves with it.

**Response (round 2):** The case moves to the built-binary tier
(`design-log.md` 2026-10-19, *design review round 2 dispositions*):
`crates/goad-check/tests/binary/run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config`
runs `goad-check --config` with `PATH` set by `Command::env` on that child
only — no `set_var`, no new target, no `#[expect]`. Layout under the case's
temp root: `config/config.toml` naming `["goad-decoy", <log>]`;
`config/goad-decoy` and `path/goad-decoy`, each `#!/bin/sh` (present on
NixOS, as the existing `deletes-itself-on-its-third-run.sh` fixture relies
on) and using shell builtins only, so neither depends on the restricted
`PATH`; each appends its own label to the log and answers conformingly. The
checker runs with `PATH` = `<root>/path`, from `<root>`. It asserts at least
one log line and every line `from-path`, so an implementation that looks
beside the config for a bare name reds it with `beside`. `design.md` §9 and
R-60's §7 row cite it at its new path; the `working_directory.rs` row is gone.

**Outcome (round 3):** verified. `design.md` §9 and R-60's §7 row put the case at `crates/goad-check/tests/binary/run.rs::a_program_with_no_slash_is_found_on_path_not_beside_the_config`, with `PATH` set by `Command::env` on the `goad-check --config` child: no global mutation, no new target. `goad-check` is located by `CARGO_BIN_EXE_goad-check`, not by `PATH`. The decoys use `#!/bin/sh`, which this tier already uses (`deletes-itself-on-its-third-run.sh`) and NixOS provides; a builtins-only decoy (`read`, `printf`, `>>`) reads its request and answers under `env -i PATH=/nonexistent` (R3-S2). Pin under the old spawn (no working directory, so only `<root>/path` can be found); red under a beside-the-config lookup, which writes `beside`.

### F-6 — `canonicalize` refuses a configuration that is not a file in a directory; D4's *same subjects* is false

**Severity:** minor
**Location:** `design.md` D4, §5.2 errors table row 1; research T2-S6 and its
*Design-input deltas*

**Expected:** D4: `canonicalize`'s failures have *the same subjects (missing,
dangling, unreadable) as `read_to_string`'s*.
**Observed:** `goad-check --config <(…)`, `--config /dev/stdin`, or any
`/proc/self/fd/N` reads today and will fail as `ConfigError::Read` with
*No such file or directory*, naming a path that exists. Nothing in the tree
uses that form (grep for `/dev/stdin`, `/dev/fd`, `<(` over `crates`, `kit`,
`exercisers`, `justfile`, canon: no hit), so the cost is a misleading message
and a closed door, not a broken test. The design neither states it nor picks:
refuse it with a message that says why, or fall back for a path with no
directory.
**Evidence:** R1-S5.

**Disposition:** tolerated
**Response:** D4's *same subjects* is now qualified to a file in a directory, and the exception is stated and tolerated: `/dev/stdin`, `<(…)`, `/proc/self/fd/N` read today and are refused as `ConfigError::Read` `NotFound` naming a path that exists. Rationale: no user in the tree, kit or canon, and a fallback would be a second rule for where a backend runs. Added to §5.2's errors table and research T2-S6 and deltas.

**Outcome:** verified — round 2. Tolerated with a reason in D4 and §5.2's table; the reason (a fallback would be a second rule for where a backend runs) is a real one.

### F-7 — The backend inherits a `PWD` naming goad's start directory

**Severity:** minor
**Location:** `design.md` §5.2 (`ProcessBackend::exchange`'s new lines), §5.5

**Expected:** R-60's point — a backend's relative paths mean its config's
directory however goad was started — to hold for what the backend can read of
its own location, or the design to say where it does not.
**Observed:** the child inherits goad's environment, `PWD` included, and
`current_dir` does not touch it. A shell repairs a stale `PWD`; other runtimes
expose it as is. §5.5 does not mention it; the design neither sets it to the
anchor nor states it.
**Evidence:** R1-S7; `demo.toml`'s comment that the backend inherits the
host's environment.

**Disposition:** fix-now
**Response:** Design, not canon (D12): when a working directory is set, the spawn also sets `PWD` to it (§5.2). I-4 states it; `working_directory.rs::the_backend_s_pwd_names_its_working_directory` asserts it with a `python3` backend, which exposes `PWD` as inherited (R1-S7) where a shell would repair it and hide the defect.

**Outcome:** verified — round 2. D12, I-4, §5.2's `.env("PWD", directory)` and `the_backend_s_pwd_names_its_working_directory` (python3, red today because `PWD` is inherited). Design, not canon, which is right: R-60 is about the working directory.

### F-8 — A kit example used in place now writes its socket into the kit, and from a read-only kit cannot bind at all

**Severity:** minor
**Location:** `design.md` §5.5 *Edge — `/nix/store` config*; §10 kit rows

**Expected:** §5.5 to state every consequence of OQ-2's read-only directory.
**Observed:** the edge names only the backend's working directory. The
`breadcrumbs` and `downloads-triage` configs write `path = "./goad.sock"`;
under R-17 that socket and its lock are created beside the config. From an
installed kit (`goad-kit` is a store path; Claude loads `$out/kit`) `hold`
cannot open the lock, so goad exits 2 (R-4) where today it binds in the
caller's directory. From the checkout, every run from anywhere now writes
`goad.sock` + `.lock` into `kit/…/examples/*/`, which makes `.gitignore`'s kit
entries load-bearing and the kit READMEs' *copy this directory out* advice a
requirement rather than a preference. The kit prose the slice rewrites needs to
say so.
**Evidence:** `flake.nix` `goad-kit`; `ingress::hold`; `.gitignore` kit
paragraph; the two example `config.toml`s.

**Disposition:** doc-wrong
**Response:** §5.5's read-only edge now covers both consequences: a read-only working directory, and a relative `ingress.path` created beside the config — from an installed kit `ingress::hold` cannot open the lock and goad exits 2 (R-4); from the checkout each run writes the socket and lock into `kit/…/examples/*/`, which `.gitignore` covers. The kit READMEs keep *copy this directory out*, now with that reason; listed in §10's collateral.

**Outcome:** verified — round 2. §5.5's read-only edge now names the socket consequence; §10 keeps *copy this directory out* with its reason.

### F-9 — Prose that states the old rule, outside §10's list

**Severity:** minor
**Location:** `design.md` §10 *Not canon, repaired in the slice*; §5.3

**Expected:** every artefact that states or depends on the old rule is in
scope (the brief's *anything the design's scope missed*).
**Observed:** not listed:
- `kit/skills/goad-backend/reference/events.md` — the one place the kit states
  `goad-emit`'s socket rule (*Without `--socket` it reads `[ingress] path` from
  goad's default config file*) and shows `--socket ./goad.sock`. AC-5 changes
  what that means.
- `.gitignore`'s kit-example paragraph (*relative to where goad is run*); §5.3
  names only the demo entries.
- `kit/…/downloads-triage/watch.sh` header (*a host started on this example's
  config.toml is reached with `--socket ./goad.sock`*), true now only from that
  directory.
- `crates/goad-check/tests/kit/tree.rs::files` doc (*a host run in an
  example's directory leaves one there*), and the module docs of
  `tests/kit/main.rs` and `tests/kit/examples.rs` (*started in a copy of its
  own directory*), which AC-2's change makes false.
**Evidence:** grep for `started in|working directory|relative to where|
goad.sock` over `kit`, `crates`, `.gitignore`.

**Disposition:** doc-wrong
**Response:** All added to §10's collateral and `slice-015.md` §Scope: `events.md`, `.gitignore`'s kit-example paragraph, `downloads-triage/watch.sh`'s header, and the docs of `tests/kit/main.rs`, `examples.rs::run_checker` and `tree.rs::files`. Research gains them as a delta row.

**Outcome:** verified — round 2. Every artefact F-9 listed is in §10 and `slice-015.md` §Scope.

### F-10 — 012's walks are still pending; R1 covers file contention, not which rule those walks run under

**Severity:** minor
**Location:** `design.md` §8 R1; `slice-015.md` *Depends on*

**Expected:** a stated order between 015's code landing and 012's walks.
**Observed:** 012's notes show PHASE-09 (first walks) and PHASE-10 (re-walks)
pending. R1 says code phases do not wait. If 015's code reaches `main` first,
012's walkers run a host with the new rule while reading kit text
(`running.md` *SPEC-001/R-36*, `checking.md`) that states the old one — the
kit edits wait for PHASE-10 — so a walker who writes cwd-relative paths for a
config elsewhere now fails, and 012's walk evidence is taken against a rule
its kit does not describe.
**Evidence:** `docs/slices/012/notes.md` §Status; `design.md` §8 R1.

**Disposition:** doc-wrong
**Response:** Decided differently from a wait (`design-log.md` 2026-10-19, round 1 dispositions): 015's code and its kit and exerciser prose land together in one merge from 015's own branch and worktree, so `main` never holds one rule with the other's text; that merge goes to `main` when 015 ships, without waiting on 012's PHASE-10 if 012 has not moved, and only a PHASE-10 in flight is coordinated with. §8 R1 and `slice-015.md` *Depends on* rewritten. `canon-delta.md`'s promotion paragraph is now a preference: if 015 closes first it promotes its own delta and SPEC-001 carries R-60 with no R-59 until 012 promotes.

**Outcome:** verified — round 2. `slice-015.md` *Depends on*, §8 R1, `canon-delta.md`'s promotion note and the design-log entry agree: one merge, code with prose. A walk in flight across that merge is not covered: F-20.

### F-11 — *Not spawnable* gains a second subject and still names no path

**Severity:** minor
**Location:** `design.md` §5.2 errors table (rows 2 and 3)

**Expected:** a refusal a person can act on (CLAUDE.md: every refusal says
which side was wrong — it does — and brief §13's *different mistakes with
different fixes*, cited by `BackendError`'s doc).
**Observed:** a missing anchor and a program not beside the config both render
`backend could not be spawned: No such file or directory (os error 2)`
(`BackendError`'s `Display`). Before, `NotFound` had one subject and the place
it looked was the directory the person started goad in. Now the place is a
directory the person may never have typed (a link's target), and the message
says neither it nor the executed path. *No new variant* is defensible; the
design should say whether the message carries the resolved path.
**Evidence:** `error.rs` `impl Display for BackendError` (`Spawn` arm);
R1-S6.

**Disposition:** fix-now
**Response:** D13: `BackendError::Spawn` becomes `{ executable, working_directory, source }`, its `Display` naming both paths; `requirement()`, `fault()` and the asserted prefix are unchanged. R-44's wording is unchanged (it names the error's kind, not its text) and no SPEC-004 line quotes the message. Canon does change: R-60 gains *its report MUST name the path the host tried to execute and the working directory it tried to execute it in*, held by `working_directory.rs::a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for`. The tests that match or build `Spawn` are listed in §9.

**Outcome:** verified — round 2. D13, §5.2's `Spawn { executable, working_directory, source }` and its `Display`; R-60's new clause; `a_program_that_cannot_be_spawned_is_reported_with_where_it_was_looked_for`. §9's list of changed tests matches every site that matches or builds `Spawn` (grep `BackendError::Spawn` over `crates`: `error.rs` and its tests, `process.rs`, `transport.rs`, `failure_matrix.rs`, `renderer/table.rs`, `kit/coverage.rs`). The clause's reach for a command with no directory: F-19.

### F-12 — I-3 holds only when goad runs on the default configuration

**Severity:** nit
**Location:** `design.md` §5.5 I-3

**Expected:** *goad and `goad-emit` … agree wherever each starts.*
**Observed:** `goad-emit`'s `socket_path` reads only `default_path`; goad
started on another file (the demo) agrees with nothing — hence `just emit`'s
`--socket`. True for the same file, which AC-5's test exercises; I-3 should say
so.
**Evidence:** `goad-emit` `main.rs::socket_path`; `justfile` `emit` comment.

**Disposition:** doc-wrong
**Response:** I-3 now says *reading the same file*, and that `goad-emit` reads only the default file, so a goad started on another (the demo) is reached with `--socket`.

**Outcome:** verified — round 2. I-3 is conditioned on the same file, naming `socket_path`.

### F-13 — R-17's §7 row: tier label and *every startup failure*

**Severity:** nit
**Location:** `canon-delta.md` SPEC-003 Change 1 and Change 3

**Expected:** the row's tier label to match the tests it names; R-17's scope
unambiguous.
**Observed:** Change 3 says *renderer and unit* and names
`crates/goad-shell/tests/integration/ingress.rs`, an integration test. R-17's
*the one every startup failure names* reads, out of context, as every startup
failure — the unreadable-config one names the config path.
**Evidence:** Change 3 text; `StartupError::ConfigUnreadable`.

**Disposition:** doc-wrong
**Response:** R-17's §7 row is labelled *renderer, integration and unit*. R-17 now says the resolved path is the one *each of their* (R-1 to R-5's) startup failures names.

**Outcome:** verified — round 2. R-17's row reads *renderer, integration and unit*; R-17 says *each of their startup failures*.

### F-14 — The length pin asserts std's wording and needs a short temp directory

**Severity:** nit
**Location:** `design.md` §9 R-4 pin row; `canon-delta.md` SPEC-003 Change 3

**Expected:** a pin that turns red only if the length case is filed elsewhere.
**Observed:** *message names … `SUN_LEN`* is std's
`SocketAddr::from_pathname` text, not the host's; a toolchain rewording reds
the case for no behaviour change. `ErrorKind::InvalidInput` under
`BindFault::Unbindable` is the host's own claim. The 107-byte control needs
`temp_dir()` of at most 105 bytes; a jail's `TMPDIR` (§8 R3) can exceed that,
and the plan should say what the case does then.
**Evidence:** tokio `UnixListener::bind` → std `SocketAddr::from_pathname`;
T2-S5.

**Disposition:** fix-now
**Response:** The pin asserts `BindFault::Unbindable` over `ErrorKind::InvalidInput` and that the message names the path; std's wording is not asserted (§9 and R-17's §7 row). Precondition stated: when `std::env::temp_dir()` leaves no room for the 107-byte control, the case fails naming the precondition and the length found — it never skips. §8 R3's signal updated.

**Outcome:** verified — round 2. The pin asserts `InvalidInput` under `Unbindable` and the path, not std's wording, and states its `temp_dir` precondition (fails, never skips).

### F-15 — `Command::in_directory` duplicates a public field

**Severity:** nit
**Location:** `design.md` §5.2 `impl Command`

**Expected:** one way to set the directory (CLAUDE.md: write less code).
**Observed:** `working_directory` is `pub`, and `in_directory` sets it too;
its one caller is `Config::parse`.
**Evidence:** §5.2.

**Disposition:** doc-wrong
**Response:** `Command::in_directory` is removed; `Config::parse` sets the public `working_directory` by struct update over `from_argv`'s value. D1 says the field is the one way to set it.

**Outcome:** verified — round 2. No builder; §5.2 says the public field is the one way.

### F-16 — The *existing tests changed* list omits `config.rs`'s own tests

**Severity:** nit
**Location:** `design.md` §9, *Existing tests changed, not added*

**Expected:** every caller of `Config::parse` whose call changes.
**Observed:** `config.rs::tests` calls `Config::parse(…)` directly in
`rejection` and several cases (`the_design_s_own_example_loads`,
`an_ingress_section_loads_with_its_path`, which also asserts
`./goad.sock` as written, `with_no_ingress_section_ingress_is_none`). The
first two change meaning, not only signature.
**Evidence:** `crates/goad-shell/src/config.rs` `mod tests`.

**Disposition:** doc-wrong
**Response:** §9's *existing tests changed* list now names `config.rs::tests` — `rejection` and every case calling `parse` — and the two that change meaning: `the_design_s_own_example_loads` (asserts `working_directory`) and `an_ingress_section_loads_with_its_path` (asserts the anchored path, not `./goad.sock`).

**Outcome:** verified — round 2. §9 lists `config.rs::tests` and the two cases that change meaning.

### F-17 — The stale root socket pair is *deleted by hand*, with no box to catch it

**Severity:** nit
**Location:** `design.md` §5.3 *Repo state*

**Expected:** a deferred manual step carried as a checklist item, naming every
checkout it applies to.
**Observed:** the person's checkout holds both `goad-demo.sock` and
`goad-demo.sock.lock` at the root now; agent worktrees under
`.claude/worktrees/` may too. Once `.gitignore` drops the entries, the `.lock`
shows as untracked (T2-S7). The design leaves it to prose.
**Evidence:** `ls` at the repository root; §5.3.

**Disposition:** fix-now
**Response:** A box in `notes.md` §Owed: delete the root `goad-demo.sock` and `.lock` in every checkout — the person's main checkout and each `.claude/worktrees/` worktree — once `.gitignore` drops their entries. §5.3 points to it.

**Outcome:** verified — round 2. `notes.md` §Owed carries the box, naming every checkout including worktrees.

### Round 2 — raiser's notes

Raiser: the round-1 raiser, at `2ebb41c`. Read the artefacts, not the
Responses: `design.md` §5.2, §5.3, §5.5, §7 (D4, D12, D13), §8 R1, §9, §10;
`canon-delta.md` SPEC-001 Changes 1–3 and SPEC-003 Changes 1 and 3;
`slice-015.md` *Depends on* and §Scope; `notes.md` §Owed; `design-log.md`'s two
round-1 entries. No new spike: R1-S1 and R1-S2 already answer every
behavioural question the repairs raise.

**Checked and complete:**

- R-60's reworded text, §6.4's paragraph and R-60's §7 row agree with §5.2's
  spawn code and with I-2: no `arg0`, the joined path as the name, `PWD` set,
  the report naming executable and directory. No line number and no count in
  any of them; every test is cited by path and symbol.
- The script-`$0` case is the discriminating witness for *the host's own
  resolution* (R1-S1: `./b.sh` under the platform route, absolute under the
  join). Red today (`NotFound`) and red under the regression F-2 named.
- The directory-link case on the loaded value is red under `path.parent()` as
  given, and the row explains why the spawn cannot see it.
- `BackendError::Spawn`'s new shape: §9's list is every site that matches or
  builds it (`error.rs` — `requirement`, `fault`, `Display`, `source`, and its
  tests; `process.rs`; `transport.rs`; `failure_matrix.rs`;
  `renderer/table.rs`, whose prefix assertion survives; `kit/coverage.rs`).
  `goad-check` renders the failure through `Display`, so its report carries
  the paths too. `goad-check/tests/binary/run.rs`'s probe case asserts the
  requirement and side, not the text, and is unaffected.
- `transport_shape.rs`: the new spawn code still holds `let mut command =
  Command::new(`; the error's construction contains `Spawn`, not `spawn` (the
  guard is case-sensitive), so `the_only_spawn_is_the_child` stays at one hit.
- `PWD` (D12): set only beside `current_dir`, to the same canonical path, so
  `pwd -P` and `PWD` agree. Setting one variable does not change how std
  finds a bare program (that changes only when `PATH` itself is set).
- The python3 `PWD` case is red today: the test process's `PWD` is the
  invocation directory, never the temp config's.
- F-10's one-merge decision is consistent across four artefacts, and the
  promotion note no longer makes 012's audit a gate.
- The length pin's precondition: it fails rather than skips when `temp_dir()`
  is too long. In a jail whose `TMPDIR` exceeds 105 bytes the gate goes red,
  by design and with a message saying why; the existing `ingress.rs` cases
  that bind under `temp_dir()` already carry the same exposure. Recorded,
  not raised.

**Not checked:** whether the larger `Spawn` variant moves `BackendError` past
clippy's `result_large_err` threshold. It is two `PathBuf`s and an
`io::Error`, likely under it; the first `just check` of the phase answers it.

**For round 3:** F-5's mechanism (where the case lives and how `PATH` reaches
the child) and F-18–F-20's repairs. Nothing else in the repairs is open.

### F-18 — Nothing holds R-60's *a native program alike*

**Severity:** minor
**Location:** `canon-delta.md` SPEC-001 Change 1 (*the joined path is the name
the program receives, a script and a native program alike*) and Change 3;
`design.md` §5.5 I-2, §9

**Expected:** each clause of R-60 held by a case that a regression would
turn red.
**Observed:** round 1's `the_backend_receives_its_program_as_written`, the one
case that observed a native program's argv[0], was dropped with `arg0`. Now
only a script's `$0` is asserted, and a script's `$0` is the executed path
whatever argv[0] is (R1-S1). So the regression *add `arg0(&program)` back*
leaves every §9 case green while a native program receives `./shell` rather
than the joined path the canon promises.
**Evidence:** R1-S1 (script `$0` absolute with `arg0("./b.sh")`), R1-S2
(native `$0` follows `arg0`); `design.md` §9 has no native-program row. The
same construction as before — `["./shell", "-c", 'echo "$0"']`, `shell` a
link to `sh` — now asserting the anchored path, holds it.

**Disposition:** fix-now
**Response:** `working_directory.rs::a_native_program_receives_its_anchored_path_as_its_name`: the devshell's `sh`, resolved at test time (`sh -c 'command -v sh'`, then canonicalized to the real binary), is **copied** beside the config as `shell`; `["./shell", "-c", …]` logs `$0`, which is argv[0] for `sh -c` with no operand, and the case asserts `<canonical anchor>/shell`. Red today (`NotFound`) and red if `arg0(&program)` is put back (`./shell`, R1-S2). Cited in `design.md` §9 and R-60's §7 row.

**Outcome:** verified — round 3. `working_directory.rs::a_native_program_receives_its_anchored_path_as_its_name` in §9 and R-60's §7 row. The devshell's `sh` is bash (`bash-interactive-5.3p15/bin/sh` → `bash`, an ELF, not a multicall or wrapper binary), so `sh -c 'command -v sh'` + canonicalize + copy gives a real bash that behaves the same under the name `shell`; the store paths it was linked against are absolute, so the copy runs. R3-S1: the copy's `$0` is the joined path under D5, and `./shell` both with `arg0("./shell")` and under the platform's route, so the case is red under both regressions.### F-19 — R-60's report clause is not scoped to commands with a working directory

**Severity:** nit
**Location:** `canon-delta.md` SPEC-001 Change 1 (*its report MUST name the
path the host tried to execute and the working directory it tried to execute
it in*); `design.md` §5.2 `Display` (*the parenthesis only when a working
directory is set*)

**Expected:** a MUST whose letter the design meets in every case it covers.
**Observed:** the sentence follows *A program that cannot be started is R-44's
…* and reads as applying to every spawn failure. For a command with no file —
`goad-check`'s argv form — the design's `Display` names no directory, which
breaks the literal clause. Either scope the clause (*a command loaded from a
configuration file*, as R-60's first sentence is) or name the caller's
directory too.
**Evidence:** the two texts cited.

**Disposition:** doc-wrong
**Response:** R-60's clause is narrowed: *for a command loaded from a configuration file, its report MUST name the path the host tried to execute and the working directory …*. §6.4's sentence likewise says *a configured command*. `design.md` §5.2 states that such a command always has a working directory (I-1), and that `goad-check`'s argv form reports the executable only, its directory being the caller's own. Endorsed by the person (`design-log.md` 2026-10-19, round 2 dispositions).

**Outcome:** verified — round 3. R-60's clause now reads *for a command loaded from a configuration file*; §6.4 says *a configured command*; §5.2 says the argv form names the executable only and why; the §7 row's case loads a configuration. The four agree.### F-20 — R1 coordinates with a PHASE-10 in flight, not a PHASE-09

**Severity:** nit
**Location:** `design.md` §8 R1; `slice-015.md` *Depends on*

**Expected:** the merge rule to cover every 012 phase whose walks run the
binaries and read the kit.
**Observed:** R1's risk names PHASE-09 and PHASE-10; its mitigation coordinates
only with *a PHASE-10 actually in flight*. A PHASE-09 walk in progress when 015
merges sees the rule change mid-walk, the case F-10 was about. One word fixes
it, or a sentence saying why PHASE-09 cannot be in flight.
**Evidence:** `docs/slices/012/notes.md` §Status (PHASE-09 pending).

**Disposition:** doc-wrong
**Response:** The merge coordinates with any 012 walk in flight, PHASE-09 or PHASE-10, and with PHASE-10 also as a writer of the same files: `design.md` §8 R1 and `slice-015.md` *Depends on* corrected. `design-log.md`'s round 2 entry already states it; F-10's Response stands as written in round 1 and this Response supersedes its *only a PHASE-10* phrase; `canon-delta.md`'s promotion paragraph names no phase.

**Outcome:** verified — round 3. §8 R1 and `slice-015.md` *Depends on* both coordinate with a 012 walk actually in flight, PHASE-09 or PHASE-10.### Round 3 — raiser's notes

Raiser: the round-1 raiser, at `66ad96a`, over `b7b1756` and `66ad96a` only.
Spike `R3-S` (scratchpad, not committed):

- R3-S1 the devshell's `sh`, canonicalized and **copied** beside a directory
  as `shell`, run with `-c 'echo $0'`: `Command::new(<dir>/shell)` with
  `current_dir` → `<dir>/shell`; the same with `arg0("./shell")` → `./shell`;
  `Command::new("./shell")` with `current_dir` (the platform's route) →
  `./shell`.
- R3-S2 a `#!/bin/sh` decoy that loops on `read`, appends a label with
  `printf … >> "$1"` and prints `{}` runs to exit 0 under
  `env -i PATH=/nonexistent`.

**Complete:** F-5, F-18, F-19 and F-20's repairs, and R-60's narrowed clause
against §5.2, §6.4 and the §7 row (F-19's Outcome). The §7 row now cites every
case by full path or under its one stated default, and no case is counted.
One wording point in the same row is F-21.

### F-21 — R-60's §7 row opens with a tier and a quantifier two of its cases do not meet

**Severity:** nit
**Location:** `canon-delta.md` SPEC-001 Change 3

**Expected:** the row's opening to describe the cases it names. Round 1's F-13
is the same class for R-17's row.
**Observed:** *integration, every case loading its configuration and driving a
real backend from a working directory that is not the configuration's* —
but the row also names a built-binary case (`goad-check/tests/binary/run.rs`),
a case that drives no backend (`::a_config_reached_through_a_linked_directory_
is_anchored_at_the_target` asserts the loaded value), and a unit test. The body
says which is which, so nothing is false in substance; the opening is.
**Evidence:** Change 3's text.

**Disposition:** doc-wrong
**Response:** (orchestrator, as responder.) R-60's §7 row in `canon-delta.md` now opens "integration, built-binary and unit. Every case that drives a real backend loads its configuration and does so from a working directory that is **not** the configuration's." The body is unchanged. The person approved the phrase (`design-log.md` 2026-10-19, *F-21*).

**Outcome:**

## Synthesis

Three rounds, findings F-1 to F-21; no blocker was raised.

**What the review changed.** Most of it landed on R-60:
- It no longer promises a program its name as written. A script cannot
  receive that, so a relative program with a `/` now receives the joined path,
  scripts and native programs alike, and `arg0` is gone (F-1).
- The host's own resolution now has a witness, the script's `$0` (F-2), and
  the native half has one too (F-18).
- The directory-link half of the linked-config case moved from the spawn,
  where it is unobservable, to the loaded value (F-3).
- Its verification row separates red cases from the pin (F-4).
- The `PATH` case moved to the built-binary tier so the environment cannot
  decide it (F-5).
- A spawn failure for a configured command names the executed path and the
  working directory (F-11, scoped by F-19).

Elsewhere:
- The child's `PWD` is set to its working directory (F-7).
- The read-only-directory edge now covers the socket, so the kit keeps *copy
  this directory out*, with the reason (F-8).
- Missed collateral joined the scope (F-9).
- 015 merges code and prose together, coordinated with any 012 walk in flight
  (F-10, F-20).

The smaller ones: I-3 is conditioned on the same file (F-12); R-17's row and
phrase were tidied (F-13); the length test no longer asserts std's wording and
states its precondition (F-14); the builder went (F-15); the changed-tests
list is complete (F-16); the stale root socket became a checklist box (F-17).

**What it confirmed.** The design's spikes reproduce. `tokio`'s `arg0` and
`UnixListener::bind` behave as stated, so host and `goad-emit` meet one
length rule. `anchored` has the byte behaviour D7 needs. The lock is opened
before the bind. Every config-driven spawn reaches the new field without
editing a `main.rs`, and every caller of the changed functions and every site
of `BackendError::Spawn` is in §9. The spawn-shape guards still hold.

**Risks knowingly left standing:**
- A config named by `/dev/stdin`, `<(…)` or `/proc/self/fd/N` is refused as
  unreadable (F-6, tolerated: nothing uses the form).
- A relative or empty `PATH` entry now means the config's directory (F-5's
  edge, stated, not defended).
- The length test fails rather than skips where `temp_dir()` exceeds 105
  bytes, so a jail with a long `TMPDIR` gates red, with a message saying why.
- `goad`'s own `main.rs::start` is held by review and AC-4 only (D11).
- If 015 closes before 012, SPEC-001 carries R-60 with no R-59 until 012's
  audit.


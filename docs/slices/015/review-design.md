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
| F-1 | major | | |
| F-2 | major | | |
| F-3 | major | | |
| F-4 | minor | | |
| F-5 | minor | | |
| F-6 | minor | | |
| F-7 | minor | | |
| F-8 | minor | | |
| F-9 | minor | | |
| F-10 | minor | | |
| F-11 | minor | | |
| F-12 | nit | | |
| F-13 | nit | | |
| F-14 | nit | | |
| F-15 | nit | | |
| F-16 | nit | | |
| F-17 | nit | | |

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

### F-12 — I-3 holds only when goad runs on the default configuration

**Severity:** nit
**Location:** `design.md` §5.5 I-3

**Expected:** *goad and `goad-emit` … agree wherever each starts.*
**Observed:** `goad-emit`'s `socket_path` reads only `default_path`; goad
started on another file (the demo) agrees with nothing — hence `just emit`'s
`--socket`. True for the same file, which AC-5's test exercises; I-3 should say
so.
**Evidence:** `goad-emit` `main.rs::socket_path`; `justfile` `emit` comment.

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

### F-15 — `Command::in_directory` duplicates a public field

**Severity:** nit
**Location:** `design.md` §5.2 `impl Command`

**Expected:** one way to set the directory (CLAUDE.md: write less code).
**Observed:** `working_directory` is `pub`, and `in_directory` sets it too;
its one caller is `Config::parse`.
**Evidence:** §5.2.

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

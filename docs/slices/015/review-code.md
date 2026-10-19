# Review — implementation — Slice 015

**Subject:** implementation — commit range `c68a624..6818bc7` on branch
`slice-015` (code, tests, kit prose, exercisers, repo files)
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

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-10-19 — the whole range, first pass from the diff and
`design.md` / `canon-delta.md`, before reading `audit.md`.

Invariants held: the five in `CLAUDE.md` (no domain vocabulary; permissive
wire, canonical inside; no narrowing; a backend failure never takes the host
down and every refusal says which side; strata one way); *name, never count*;
*cite by symbol*; the endorsed R-60 / R-17 wording in `canon-delta.md` as the
working authority.

Lines of attack:

1. **R-60 against the code.** `Config::load` (canonicalize, read the
   canonical path, `parent`), `anchored` (drops `.`, keeps `..`),
   `Command::executable` (the `/` rule; "contains a `/` and does not begin
   with one" vs `is_relative() && contains('/')`), `ProcessBackend::exchange`
   (no `arg0`, `current_dir`, `PWD`). Degenerate programs: `./`, `dir/`, a
   trailing slash, `.` alone.
2. **What std / tokio actually do** with `current_dir` + a bare program + a
   captured environment (the `PWD` override makes std capture env): is the
   `PATH` lookup after the `chdir`, as R-60 §6.4 prose and the kit claim, on
   both the `posix_spawn` and the fork/exec route? Read the pinned std source.
3. **Error reporting.** `BackendError::Spawn` — every producer, every
   consumer (renderer, `goad-check` report), whether the Display names both
   paths wherever R-60 says it MUST; a spawn failure caused by the working
   directory, not the program. `ConfigError::Read` — does the reported
   message still name the path the person typed, and is every doc claim
   about which inputs `canonicalize` refuses true?
4. **Tests that assert a proxy.** For each new test, would the regression it
   guards survive? Especially the platform-route witness (`$0`), the link
   decoy, the `PATH` pin, the `PWD` case, the socket length pin, and the
   kit examples started from the root.
5. **Test hygiene.** Temp paths unique per case (pid alone collides between
   cases in one binary), sockets and locks cleaned up, parallel safety,
   `PATH` / env mutation, R5 (no litter in crate directories), the
   length-pin precondition.
6. **Prose truthfulness.** Kit `running.md`, `checking.md`, `events.md`,
   examples' READMEs/configs, `watch.sh`, exerciser README and headers,
   `justfile`, `.gitignore`, the memory file: does each say what the code
   does now, and nothing it does not?
7. **Invariants.** Domain vocabulary in new host code; stratum 1 untouched;
   doc comments that count or cite line numbers.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | minor | doc-wrong | verified |
| F-2 | minor | fix-now | verified |
| F-3 | minor | doc-wrong | verified |
| F-4 | minor | fix-now | verified |
| F-5 | nit | fix-now | verified |
| F-6 | nit | fix-now | verified |
| F-7 | nit | fix-now | verified |
| F-8 | nit | fix-now | verified |
| F-9 | nit | fix-now | verified |
| F-10 | nit | fix-now | verified |
| F-11 | nit | fix-now | |
| F-12 | nit | fix-now | |
| F-13 | nit | doc-wrong | |

### F-1 — R-60 and R-17 promise "the joined path", but the join drops `.` and no rule says so

**Severity:** minor
**Location:** `canon-delta.md` SPEC-001 Change 1 (R-60) and SPEC-003 Change 1
(R-17); `kit/skills/goad-backend/reference/running.md` *Name your command*;
`crates/goad-shell/src/config.rs::anchored`

**Expected:** canon that a backend author can read to predict the name their
program receives and the socket path the host binds. R-60: a relative program
"MUST be executed as that path joined onto the same directory … the joined
path is the name the program receives". `running.md`: "your program receives
that joined path as its name".
**Observed:** `anchored` filters every `Component::CurDir`, so `./backend.sh`
under `/dir` is executed as, and receives, `/dir/backend.sh`; `a/./b` becomes
`/dir/a/b`. The ordinary meaning of "joined" (and `Path::join`, and a shell's
`"$dir/$program"`) is `/dir/./backend.sh`. Neither R-60's nor R-17's rule
text states the normalization (drop interior `.`, keep `..`); it appears only
in R-17's §7 row ("byte for byte so that no `./` survives") and in
`design.md` D7, which is not canon. R-17 says only "the resolved path".
**Evidence:** `config.rs::tests::a_relative_path_resolves_against_the_directory_given`
pins `("a/./b/./goad.sock", "/anchor/a/b/goad.sock")`;
`working_directory.rs::a_relative_program_with_a_slash_is_executed_as_the_anchored_path`
asserts `$0` = `{anchor}/backend.sh` for `["./backend.sh"]`. Failure scenario:
a second implementation held to SPEC-001 that joins literally passes every
stated MUST and hands a script `$0` = `/dir/./backend.sh`, and binds a socket
path two bytes longer — the 107-byte budget R-17's own length clause is about.

**Disposition:** doc-wrong — confirmed by the person (`design-log.md` 2026-10-19, *code review round 1 dispositions*).
**Response:** `canon-delta.md` SPEC-001 Change 1 (R-60): the join gains *"each `.`
component dropped and each `..` kept as written"*, after *"joined onto the
same directory"*. SPEC-003 Change 1 (R-17): *"It is joined onto that
directory as SPEC-001/R-60 joins a relative program: each `.` component
dropped, each `..` kept."*, after its first sentence. Both in the endorsed
words, unparaphrased; the **Status** line records the amendment.
`running.md` *Name your command*: the program receives *"the config file's
directory joined with that path as its name, with each `.` part dropped and
each `..` kept: `./backend.sh` arrives as `<directory>/backend.sh`"*. Class
check in code: `config.rs` — `anchored`'s doc said *interior* `.` components
(a leading one is dropped too) and *"`.` alone"*; now *each* `.` and *"a path
of `.` components only"*; the same *"the one spelling"* claim in
`tests::a_relative_path_resolves_against_the_directory_given`'s doc was false
beside its own `./` row, now reworded. Left for the person, not edited:
`canon-delta.md` §6.4's *"is executed by its path beside the file and
receives that path as its name"* and Change 1's **Why** *"receives the joined
path"* — both now lean on R-60's amended definition of the join and read
true by it; §6.4 does not restate the `.` rule. Whether it should is wording
beyond the endorsed text.

**Outcome:** verified — `canon-delta.md` R-60 and R-17 carry the person's endorsed words exactly (compared against `design-log.md` 2026-10-19, *code review round 1 dispositions*); `anchored`'s and its test's docs now say *each* `.` and *a path of `.` components only*, which the `./` row bears out; `running.md` gives the worked example. §6.4's *receives that path as its name* reads true under the amended join.

### F-2 — `ConfigError::Read`'s and `Config::load`'s docs name refusals that do not happen

**Severity:** minor
**Location:** `crates/goad-shell/src/error.rs` — `ConfigError::Read`'s doc;
`crates/goad-shell/src/config.rs` — `Config::load`'s `# Errors`

**Expected:** a doc that states which inputs `load` refuses.
**Observed:** `ConfigError::Read`: "a path that names no file in a directory
— which canonicalization refuses for `/dev/stdin` and `/proc/self/fd/N`".
`Config::load`: "`ConfigError::Read` if the file … names no file in a
directory". `canonicalize` is `realpath(3)`: `/dev/stdin` and
`/proc/self/fd/N` are refused only when the descriptor is not a file (a pipe,
a socket — the `pipe:[…]` link target does not resolve). When the descriptor
is a regular file they canonicalize to that file, which is then read and
anchored at its real directory. The `parent() == None` arm the phrase "names
no file in a directory" describes is reachable only for `/`.
**Evidence:** run by the reviewer, scratchpad:
`realpath("/dev/stdin") < f.txt` → `…/scratchpad/f.txt`;
`echo x | realpath /dev/stdin` → `FileNotFoundError … /proc/<pid>/fd/pipe:[…]`;
`realpath("/proc/self/fd/7") 7<f.txt` → `…/scratchpad/f.txt` (Python's
`os.path.realpath(strict=True)`, which follows the same kernel links as
libc `realpath`). Confirms the audit's lead. Failure scenario: a person
reading the doc concludes `goad /dev/stdin < config.toml` is refused, when it
loads and runs the backend in the config's directory.

**Disposition:** fix-now — confirmed by the person. The `design.md` D4 / §5.2 overstatement is recorded in `audit.md` *Design drift not reconciled* (already there; reworded to cite this finding).
**Response:** `error.rs` — `ConfigError::Read`'s doc: missing or unreadable, a link to
nothing, `/`, or a descriptor that is not a file (`/dev/stdin` or
`/proc/self/fd/N` on a pipe or a socket, which canonicalization refuses); on
a regular file either path canonicalizes to that file and loads.
`config.rs` — `Config::load`'s `# Errors`: `Read` if the path does not
canonicalize (missing, dangling link, a non-file descriptor such as
`/dev/stdin` on a pipe), if it is `/` (no parent directory — the only input
the `parent()` arm reaches), or if the file is unreadable; and
`/dev/stdin` redirected from a regular file is that file, read and anchored
at its directory. Doc lines only: `git diff 024643d -- crates/goad-shell/src/ | grep '^[-+]' | grep -vE '^(\+\+\+|---)' | grep -vE '^[-+][[:space:]]*//'`
prints nothing (exit 1): every changed line under `crates/goad-shell/src/`
is a `///`, `//!` or `//` line, so the person's no-VH-1-re-run condition
holds (`config.rs`, `error.rs`; nothing under `backend/` changed).

**Outcome:** verified — re-checked against the round-1 measurements: a pipe or socket descriptor is refused, a regular file loads, `/` is the only input the `parent()` arm reaches. Doc-only, verified independently (round 2 Synthesis).

### F-3 — "Not observable at the spawn" is false for a directory link: `PWD` and `$0` observe it

**Severity:** minor
**Location:** `canon-delta.md` SPEC-001 Change 3 (R-60's §7 row: "A link in a
**directory** component of the configuration's path is not observable at the
spawn — a working directory reached through a directory link is the target
directory"); `crates/goad-shell/tests/integration/working_directory.rs` —
`a_config_reached_through_a_linked_directory_is_anchored_at_the_target`'s doc

**Expected:** the stated reason a requirement is held on a value rather than
a process is true, because it is promoted into SPEC-001 verbatim.
**Observed:** the kernel's working directory is the same either way, but two
things the host hands the child are not. `ProcessBackend::exchange` sets
`PWD` from `working_directory` as given (D12), and executes a relative program
with a `/` as `anchored(working_directory, program)`, which is its `$0` (D5).
An implementation that anchored at `path.parent()` as given would give a child
`PWD = <t>/link` and `$0 = <t>/link/backend.sh` — both observable at the
spawn, by the same two techniques
`the_backend_s_pwd_names_its_working_directory` and
`a_relative_program_with_a_slash_is_executed_as_the_anchored_path` already
use. The test's own doc says as much one sentence earlier ("`PWD`, the socket
path and its byte length all see the **target** directory"). The value-level
instrument is sound; the claim that nothing else could hold it is not.
**Evidence:** `process.rs` — `ProcessBackend::exchange`:
`command.current_dir(directory).env("PWD", directory)` and
`Command::new(&executable)`. Failure scenario: SPEC-001 §7 records a reason
that a reader can falsify in one python line, and a future reviewer asking
"why no spawn case?" is told something untrue.

**Disposition:** doc-wrong — confirmed by the person.
**Response:** `canon-delta.md` SPEC-001 Change 3 (R-60's §7 row): *"is not observable at
the spawn — … — so it is held on the loaded value:"* replaced by the endorsed
*"leaves the kernel's working directory the same either way — a directory
reached through a link is the target directory — so it is held on the loaded
value, from which `PWD`, a relative program's name and the socket path are
all taken:"*. `working_directory.rs` —
`a_config_reached_through_a_linked_directory_is_anchored_at_the_target`'s
doc: the same reason in the same terms, and it now names what the case
asserts (the command's working directory and the socket path) instead of
*"`PWD`, the socket path and its byte length"*, which the case does not
assert directly.

**Outcome:** verified — the §7 row's sentence is the endorsed text exactly, and the test's doc matches it; it now names `PWD`, a relative program's name and the socket path as taken from the loaded value, which is what `ProcessBackend::exchange` and `ingress_config` do.

### F-4 — The slice added helpers and left counts of them standing elsewhere

**Severity:** minor
**Location:** `crates/goad-shell/tests/integration/ingress.rs` —
`socket_path`'s doc; `crates/goad-check/tests/kit/examples.rs` — `handout`'s
doc; `crates/goad/tests/renderer/startup.rs` — `listener::socket_path`'s doc

**Expected:** *name, never count* (`CLAUDE.md`). The slice rewrote
`tests/support/scripting.rs` `claim`'s doc to name every helper of the class
rather than count them; the satellites that count the same class should move
with it (fix the class).
**Observed:** three counts this slice made false or riskier, none edited:
- `ingress.rs::socket_path`: "the other helpers of this class mint
  `goad-serve-…`, `goad-startup-…` and — unheld, both of them … —
  `goad-emit`'s two". The class now also holds the `goad-wd-…` and
  `goad-check-…` directory helpers, and `goad-emit` has three unheld helpers
  (`exchange.rs::socket_path`, `exchange.rs::config_home` — new in this slice
  — and `main.rs`'s `config_home`), as `claim`'s rewritten doc itself lists.
- `examples.rs::handout`: "unheld by it for the reason `claim`'s doc gives
  `goad-emit`'s two" — same stale count.
- `startup.rs::listener::socket_path`: "the third such helper in this test
  binary, and held by the same instrument as the other two" — the slice added
  `listener::directory`, held by `claim`, in the same module.
**Evidence:** `grep -rn "goad-emit\`'s two\|third such helper" crates tests`
→ the three docs above; `git show c68a624:tests/support/scripting.rs` names
`goad-emit`'s two, `HEAD`'s names three. Failure scenario: the next helper
author trusts "goad-emit's two" when checking a new kind against what the
others mint — the check `claim`'s doc says must be done by reading, not by
name.

**Disposition:** fix-now — confirmed by the person.
**Response:** Class grep (`grep -rn 'helper' crates tests --include=*.rs` for counted or
ordinal references to `claim`'s class) found the three named and a fourth:
`crates/goad/tests/renderer/ingress.rs` — `socket_path`'s doc, *"made by
four more helpers — … and two in `goad-emit`"*. Each now points at `claim`'s
doc, which names every helper held and unheld, instead of counting:
`crates/goad-shell/tests/integration/ingress.rs::socket_path` (*"the other
helpers of this class mint paths of their own — `claim`'s doc names each"*),
`crates/goad-check/tests/kit/examples.rs::handout` (*"the reason `claim`'s
doc gives every unheld one"*), `crates/goad/tests/renderer/startup.rs` —
`listener::socket_path` (*"a helper of `claim`'s class, held by it as every
such helper in this test binary is"*), and
`crates/goad/tests/renderer/ingress.rs::socket_path` (*"made by every helper
of `claim`'s class … `claim`'s own doc names each"*).

**Outcome:** verified — the three counts are gone and the sweep found a fourth (`crates/goad/tests/renderer/ingress.rs::socket_path`, *four more helpers … two in `goad-emit`*), the class this finding was about. A re-grep for counted or ordinal references to `claim`'s class finds none. The repair's own reflow is F-11.

### F-5 — Three citations name the wrong source

**Severity:** nit
**Location:** `crates/goad-shell/src/config.rs` —
`tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash`'s
doc; `working_directory.rs` —
`a_config_reached_through_a_linked_directory_is_anchored_at_the_target`'s doc;
`crates/goad-shell/src/error.rs` — `BackendError::Spawn`'s doc

**Expected:** a citation that says what it is cited for.
**Observed:**
- "D3: only a relative program **with a slash** is the config's; a bare name
  is the environment's" — `design.md` D3 is `Config::parse(text, directory)`;
  the `/` rule is D5 and the `PATH` rule D6.
- "`design.md` §5.2's I-1" — I-1 is in §5.5 *Invariants*.
- "`None` is a command with no file, which runs where its caller runs
  (SPEC-001/R-60)" — R-60 governs only "a command loaded from a configuration
  file"; the `None` case is D2 (and `design.md` §5.2 *Errors* says R-60 "does
  not govern a command with no file").
**Evidence:** `design.md` §7 rows D2, D3, D5, D6; §5.5 I-1; `canon-delta.md`
R-60's first sentence.

**Disposition:** fix-now — confirmed by the person.
**Response:** `config.rs` —
`tests::the_executable_is_anchored_only_for_a_relative_path_with_a_slash`'s
doc cites D5 for the `/` rule and D6 for `PATH`. `working_directory.rs` —
`a_config_reached_through_a_linked_directory_is_anchored_at_the_target`'s
doc cites `design.md` §5.5's I-1. `error.rs` — `BackendError::Spawn`'s doc
cites `design.md` D2 for `None` and says R-60 governs only a command loaded
from a file. Doc lines only (see F-2's check).

**Outcome:** verified — D5/D6, §5.5's I-1 and D2 are the right sources (`design.md` §7, §5.5).

### F-6 — Two `claim` kinds mint the same `goad-wd-<case>-<pid>.d` shape

**Severity:** nit
**Location:** `crates/goad/tests/renderer/startup.rs` — `listener::directory`;
`crates/goad-shell/tests/integration/working_directory.rs` — `directory`

**Expected:** `claim`'s doc: "A new kind must therefore be checked against the
paths the others mint, not assumed distinct because its name is."
**Observed:** `listener::directory` claims kind `"startup directory"`,
`working_directory.rs::directory` claims `"working directory"`, and both mint
`temp_dir()/goad-wd-{case}-{pid}.d`. `claim`'s rewritten doc lists
`goad-wd-<name>-<pid>.d` once, as if one helper minted it. No live collision —
the two are in different test binaries, so the pids differ — but were they
ever in one binary, `directory("x")` from each would share a path that
neither kind's key detects. `wd` in a startup-listener helper is residue of
the copy. The helper exists three times (these two and `run.rs::root`); a
shared one in `scripting.rs` is priced by the `dead_code` cost `claim`'s doc
already names (the file is also included by `event_loop_schedule`, which
would not use it), so the copies are defensible — the shared shape is not.
**Evidence:** `grep -n 'goad-wd-' crates -r` → both helpers.

**Disposition:** fix-now — confirmed by the person.
**Response:** `crates/goad/tests/renderer/startup.rs` — `listener::directory` now mints
`goad-startup-<case>-<pid>.d` (the `wd` residue gone); its doc says the
`.d` keeps it apart from a `marker` path and from `listener::socket_path`'s
`.sock`, which shares the prefix. `working_directory.rs::directory` keeps
`goad-wd-<case>-<pid>.d`. `tests/support/scripting.rs` — `claim`'s doc lists
both shapes, `goad-startup-<name>-<pid>.d` and `goad-wd-<name>-<pid>.d`,
among the held helpers' paths; it names each helper and counts none.

**Outcome:** verified — `listener::directory` mints `goad-startup-<case>-<pid>.d`; its doc's claim that the `.d` keeps it apart from `socket_path`'s `goad-startup-<case>-<pid>.sock` in the same binary is true, and the shared prefix is what `claim`'s doc allows (one kind per helper, not per prefix). No two kinds now mint one shape.

### F-7 — `run.rs::root`'s directories are never removed

**Severity:** nit
**Location:** `crates/goad-check/tests/binary/run.rs` — `root`,
`a_program_with_no_slash_is_found_on_path_not_beside_the_config`,
`the_argv_form_runs_a_relative_program_from_the_caller_s_directory`

**Expected:** the slice's other new directory helpers' cases remove their
directory on success (`working_directory.rs`, `startup.rs`, `exchange.rs`,
the length pin); where one is kept on purpose, its doc says so
(`examples.rs::handout`: "left after the run so a failure can be read").
**Observed:** neither case calls `remove_dir_all`, and `root`'s doc says only
"emptied before it is handed out". Every run leaves two pid-named directories
(scripts, config, log) under `temp_dir()`; a later run clears only its own
pid's. Confirms the audit's lead.
**Evidence:** `grep -n remove_dir_all crates/goad-check/tests/binary/run.rs`
→ only the clear inside `root`.

**Disposition:** fix-now — confirmed by the person.
**Response:** `crates/goad-check/tests/binary/run.rs` —
`a_program_with_no_slash_is_found_on_path_not_beside_the_config` and
`the_argv_form_runs_a_relative_program_from_the_caller_s_directory` each end
with `remove_dir_all(&root)`, as the slice's other directory helpers' cases
do (`working_directory.rs`, `startup.rs`, `exchange.rs`); `root`'s doc says
its case removes it on success and a failing case leaves it to be read.
This file's older cases log to `scripting` markers, a different (single-file)
class cleared at hand-out; the directory helpers were the closer precedent.
Observed after the gate run: no `goad-check-*.d` directory for that run's
pid remained under `/tmp` (older runs' remain; not removed by this repair).

**Outcome:** verified — both cases remove their root on success; the doc says a failing case leaves it, as `handout`'s does.

### F-8 — The `goad-emit` AC-5 case starts in the shared `temp_dir()`

**Severity:** nit
**Location:** `crates/goad-emit/tests/binary/exchange.rs` —
`a_relative_configured_socket_is_found_beside_the_configuration`

**Expected:** the regression it guards (connecting to `./goad.sock` in the
start directory) fails the case, as its doc promises: "a `goad-emit` that
connected elsewhere would hang the join instead of failing the case" — hence
the status is asserted first.
**Observed:** the start directory is `std::env::temp_dir()` itself, not a
directory the case owns. Under the regression, if anything listens at
`<temp_dir>/goad.sock` and accepts, `goad-emit` exits 0, the status assertion
passes, and `listener.join()` blocks forever — the hang the ordering was
meant to prevent — after sending the case's envelope to a listener the test
does not own. Nothing in the workspace binds there today (checked:
no `temp_dir().join("goad.sock")`), so this is latent. A start directory
inside `home` (the case already owns one) removes it. Confirms the audit's
lead, with the consequence corrected: a hang, not a masked green.
**Evidence:** `emit_from(&std::env::temp_dir(), &home, …)`.

**Disposition:** fix-now — confirmed by the person.
**Response:** `crates/goad-emit/tests/binary/exchange.rs` —
`a_relative_configured_socket_is_found_beside_the_configuration` starts
`goad-emit` in `<home>/start`, a directory the case creates inside its own
`config_home` and removes with it; the doc says nothing listens there, so a
`goad-emit` that connected to `./goad.sock` is refused rather than hanging
the join. **Natural red** under the guarded regression, scratchpad
copy-and-restore: `config.rs::ingress_config`'s `path` set to
`{ let _ = directory; Path::new(&file.path).to_path_buf() }` (a first
spelling without `let _ = directory` did **not** compile — `unused variable`
under `-D warnings` — and was discarded, not counted); `cargo test -p
goad-emit --no-fail-fast`, compiled; red:
`exchange::a_relative_configured_socket_is_found_beside_the_configuration` —
*"goad-emit: nothing is listening at ./goad.sock: No such file or directory"*,
status 2, not 0, no hang — and
`main.rs::tests::a_configured_ingress_path_is_what_is_written_to` (left
`./from-config.sock`). Restored by copy; `config.rs` afterwards differs from
`024643d` in comment lines only, `git status` shows no untracked file.

**Outcome:** verified — the case starts in `<home>/start`, its own, and the repairer's natural red (compiled, `--no-fail-fast`) shows the regression fails with status 2 rather than hanging. Green in the reviewer's gate run. The AC-5 row in `audit.md` that described the old start directory was not updated (F-12).

### F-9 — Prose that says slightly more, or less, than the code

**Severity:** nit
**Location:** as listed

**Expected:** prose true of the code.
**Observed:**
- `kit/…/examples/breadcrumbs/README.md` and `downloads-triage/README.md`:
  "`command` and `[ingress] path` are relative to the config file's
  directory, so goad and `goad-check` find both" — `goad-check` opens no
  socket (`goad-check` `main.rs`: "the checker opens no socket"). Confirms the
  audit's lead.
- `running.md`: "a relative `PATH` entry names a directory under it" — a
  `..`-led entry names one above it. The behaviour itself is as stated
  (verified below, Synthesis); only *under* is loose.
- `config.rs` — `Command`'s doc still opens "What to spawn: a program and its
  arguments", though the type now also says where.
- `config.rs` — `Config::parse`'s doc: "is the only caller that need"
  (*needs*). Confirms the audit's lead.
- `working_directory.rs` module doc: "Each backend is written by its case,
  beside the configuration" — `the_backend_s_pwd_names_its_working_directory`
  runs `python3 -c` with no file, the spawn-failure case writes none, and the
  linked case writes its decoy beside the link, not the configuration.
**Evidence:** the quoted lines, by symbol or heading.

**Disposition:** fix-now — confirmed by the person.
**Response:** Kit `breadcrumbs/README.md` and `downloads-triage/README.md` *Run your own
copy*: *"so goad finds both, and `goad-check` the backend, wherever you start
them"*; fences untouched. `running.md` *Name your command*: a relative
`PATH` entry *"is relative to that directory"*; and *"the config file and
your backend can move together"* (the audit's *the file* nit, same class).
`config.rs` — `Command`'s doc opens *"What to spawn and where: a program,
its arguments and its working directory."*; `Config::parse`'s doc *"the only
caller that needs to"*. `working_directory.rs` module doc: *"Whatever a case
puts on disk — a backend, a decoy, a log — it writes into that directory"*
(the case's own). Left for the person: `canon-delta.md` §6.4's *"a relative
entry in `PATH` names a directory under the backend's working directory"*
carries the same *under* looseness as the kit line did; endorsed canon
wording, so not edited.

**Outcome:** verified — each item is true of the code: `goad-check` is credited with the backend only; a relative `PATH` entry *is relative to that directory*, as the reviewer's round-1 run showed; `Command`'s doc names the working directory; the grammar; the module doc. The same *under* in `canon-delta.md` §6.4 was left for the person without a row (F-13).

### F-10 — Doc comments left unwrapped

**Severity:** nit
**Location:** `config.rs` —
`tests::a_relative_path_resolves_against_the_directory_given`'s doc (one line
of 117 columns); `crates/goad-check/tests/kit/tree.rs` — `files`' doc (87);
`startup.rs` — `listener::directory`'s doc (93, in a file wrapped near 80)

**Expected:** the surrounding wrap; rustfmt does not wrap comments, so the
gate does not see it.
**Observed:** each was edited in place and not reflowed.
**Evidence:** `awk 'length > 85'` over the three files. Confirms the audit's
lead for `config.rs` and `startup.rs`; `tree.rs` is the reviewer's.

**Disposition:** fix-now — confirmed by the person.
**Response:** Reflowed: `config.rs` —
`tests::a_relative_path_resolves_against_the_directory_given`'s doc;
`crates/goad-check/tests/kit/tree.rs` — `files`' doc; `startup.rs` —
`listener::directory`'s doc. Class check over every comment line this slice
added (`git diff -U0 c68a624 -- '*.rs'`, added `//` lines over 80 columns):
those three, and one 81-column line in `round_trip.rs`, which matches that
file's own wrap (it has lines to 84) and was left.

**Outcome:** verified — the three named docs are reflowed. A new over-long line was introduced by F-4's repair, not missed here (F-11).

### F-11 — F-4's repair left one doc line unwrapped and two paragraphs half-reflowed

**Severity:** nit
**Location:** `crates/goad/tests/renderer/ingress.rs` — `socket_path`'s doc;
`crates/goad/tests/renderer/startup.rs` — `listener::socket_path`'s doc;
`tests/support/scripting.rs` — `claim`'s doc

**Expected:** F-10's rule, applied to the lines the round-1 repairs wrote.
**Observed:** `renderer/ingress.rs::socket_path`'s doc now has a 97-column
line (*"F-17, F-22). It matters more here than for a marker: two cases
sharing a socket path share a"*), written by `c44ee07`. The edited paragraphs
in `listener::socket_path` (*"binary is (`review-code.md` F-11). The stake is
the"*) and `claim` (*"that all of them share (F-21). What holds a"*) end
mid-paragraph on short lines. F-10's class check covered lines the slice
added before the repairs, not the repairs' own.
**Evidence:** `git diff 024643d c44ee07 -- crates tests | grep '^+' | awk
'length > 86'` → the `ingress.rs` line.

**Disposition:** fix-now — confirmed by the person (`design-log.md` 2026-10-19, *code review round 2, residues, and canon promotion*).
**Response:** Reflowed: `renderer/ingress.rs` — `socket_path`'s doc;
`startup.rs` — `listener::socket_path`'s doc; `scripting.rs` — `claim`'s doc.
Class check over every doc line the slice's repairs wrote (`git diff -U1
024643d -- '*.rs'`: an added `///` line under 60 columns followed by a
non-blank `///` line, and added lines over 81 columns): one more,
`config.rs` — `anchored`'s doc, written by F-1's repair, reflowed with the
three (a doc comment, so the doc-only condition still holds). The same
commit replaces three line-number citations with citations by section or
symbol, as the person decided at this round: `error.rs` — `ConfigError`'s
doc and `config.rs` — `tests::GOOD`'s doc cited `design.md` by line, which is
slice 001's design (introduced in `bc06d6d`, *phase 001.11*); they now cite
slice 001's `design.md` §5.4 *Startup* and §5.2 *Config*. goad-emit
`exchange.rs` — `fake_listener`'s doc cited `tests/integration/ingress.rs`
by line; it now names `goad-shell`'s `fake_listener` there. Doc-only, by
diff: `git diff 3287846 -- crates/goad-shell/src/ | grep '^[-+]' | grep -vE
'^(\+\+\+|---)' | grep -vE '^[-+][[:space:]]*//'` prints nothing.
Line-number citations of moving files remain in files this slice did not
touch (`canonical.rs`, `view_model.rs`, `lib.rs` in `goad`); not this
slice's, not edited.

**Outcome:**

### F-12 — `audit.md`'s AC-5 row still says the `goad-emit` case starts in `temp_dir()`

**Severity:** nit
**Location:** `docs/slices/015/audit.md` — *Acceptance criteria*, AC-5 row

**Expected:** evidence that describes the test as it now is.
**Observed:** the row cites
`exchange.rs::a_relative_configured_socket_is_found_beside_the_configuration`
as *"(started in `temp_dir()`, no `--socket`)"*. F-8's repair moved the start
directory to `<home>/start`, a directory the case owns, so the evidence now
describes the test as it was before the repair. (*Leads for the code
reviewer* also says `temp_dir()`, but that list records what was raised, and
reads true as history.)
**Evidence:** `grep -n 'started in \`temp_dir()\`' docs/slices/015/audit.md`
→ the AC-5 row; `exchange.rs`: `emit_from(&start, &home, …)`.

**Disposition:** fix-now — confirmed by the person (`design-log.md` 2026-10-19, *code review round 2, residues, and canon promotion*).
**Response:** `audit.md` AC-5 row: the `goad-emit` case is now described as
*"started in `<home>/start`, a directory the case owns and not the
configuration's; no `--socket`"*, as `exchange.rs` —
`a_relative_configured_socket_is_found_beside_the_configuration` does
(`emit_from(&start, &home, …)`). *Leads for the code reviewer* is left as
history, as the finding says.

**Outcome:**

### F-13 — `canon-delta.md` §6.4 keeps the *under* that F-9 repaired in the kit, with no row

**Severity:** nit
**Location:** `canon-delta.md` SPEC-001 Change 2 (§6.4): *"a relative entry in
`PATH` names a directory under the backend's working directory"*

**Expected:** a looseness found in the kit and repaired there is either
repaired in canon too or carried in the ledger so the person decides it.
F-9's Response says *"Left for the person … not edited"*, but no finding
or row records it, so promotion would copy it into SPEC-001 unnoticed.
**Observed:** a `PATH` entry such as `../bin` names a directory *above* the
working directory. `running.md` now says *"is relative to that directory"*;
the canon it paraphrases still says *under*. Endorsed wording, so changing it
is the person's call.
**Evidence:** the quoted §6.4 sentence; `running.md` *Name your command*
after `c44ee07`.

**Disposition:** doc-wrong — confirmed by the person, with the wording below (`design-log.md` 2026-10-19, *code review round 2, residues, and canon promotion*).
**Response:** `canon-delta.md` SPEC-001 Change 2 (§6.4): *"a relative entry in `PATH`
names a directory under the backend's working directory"* is now *"a
relative entry in `PATH` is resolved against the backend's working
directory"*, the person's words exactly. The **Status** line records the
amendment. §6.4 does not restate R-60's `.`-dropping rule, by the person's
decision.

**Outcome:**

## Synthesis

**Round 1 (raiser)** — 2026-10-19. Ten findings: four minor, six nit; no
blocker, no major. Three of the minors are about the record (F-1, F-3 in the
draft canon; F-2 in a code doc), one about counted prose (F-4). No finding is
a behavioural defect in the shipped code.

**Checked and found complete:**

- **R-60 behaviour.** `Config::load` canonicalizes, reads the canonical path
  and anchors at its parent; `anchored` is the one join for the executable
  and the socket; `Command::executable`'s guard `is_relative() &&
  contains('/')` is R-60's "contains a `/` and does not begin with one" on
  unix; `ProcessBackend::exchange` executes `executable()` with no `arg0`,
  sets `current_dir` and `PWD` only under `Some`, and reports
  `Spawn { executable, working_directory, source }` with both paths under
  `Some`. Every production spawn reaches it through `config.backend.command`
  (`goad` `main.rs`, `goad-check` `main.rs`); the argv form gets `None` by
  `from_argv`. Every displayed spawn failure goes through `Display`
  (`goad` `diagnostics.rs`, `goad-check` `render.rs`).
- **What std actually does** (rust-src 1.100.0-beta.5, the nearest vendored
  source to the beta.7 toolchain; `library/std/src/sys/process/unix/unix.rs`):
  the fork/exec route `chdir`s (lines 342–343) before `execvp` (413); the
  `posix_spawn` route bails when the environment changed `PATH` and the
  program is bare (471), and otherwise adds the `chdir` as a file action
  (657–673, 737–738) before `posix_spawnp`, which searches `PATH` after the
  file actions. The `PWD` override captures the environment but does not
  touch `PATH`. tokio's `Command` wraps std's. So a bare program is looked up
  after the move on both routes, as §6.4's draft paragraph and `running.md`
  say. **Verified empirically** with the built `goad-check`: `PATH=bin`,
  `bin/goad-decoy` at both the start directory and the config's directory —
  only the config directory's ran.
- **Mutation re-run** (scratchpad copy-and-restore, `--no-fail-fast`,
  compiled, `git status` clean): `Command::new(&self.command.program)` (the
  platform route) reds `a_relative_program_with_a_slash_is_executed_as_the_anchored_path`
  (`$0` `./backend.sh`) and `a_native_program_receives_its_anchored_path_as_its_name`
  (`./shell`), nothing else — the `$0` witness distinguishes the host's
  resolution from the platform's, as claimed.
- **Proxy check, by reasoning over each new case:** the link decoy answers
  differently and logs; the directory-link case compares against the
  canonical target; the `PATH` pin sets `PATH` on one child and would red
  under anchoring every relative program; the `PWD` case uses a non-shell
  runtime; the length pin has a same-directory control and fails rather than
  skips on its precondition; the kit examples start outside the copy. None
  asserts a proxy the guarded regression would survive (F-8 is a hang, not a
  false green).
- **Gate:** `just check` exit 0 at `6818bc7`, run by the reviewer.
- **Invariants:** no domain vocabulary in added host lines; `goad-semantics`
  untouched; a spawn failure in the new directory stays R-44, the
  configuration's side. No added doc cites a line number of a moving file.
- **Prose:** kit `running.md`, `checking.md`, `events.md`, the examples'
  configs and READMEs (fences match the files), `watch.sh`, `demo.toml`,
  `backend.sh`, the TypeScript README, `justfile`, `.gitignore` and the
  memory note are true of the code apart from F-9. Neither the demo backend
  nor any kit backend does cwd-relative I/O, so moving their working
  directory changes nothing else.
- **`canon-delta.md` §7 citations** each resolve to a `fn` in the named file.

**Audit leads:** confirmed — `ConfigError::Read` / `Config::load` docs (F-2);
`Config::parse` grammar and the unwrapped doc lines (F-9, F-10);
`listener::socket_path`'s count (F-4, widened to the class); `run.rs::root`
not removed (F-7); `goad-emit` started in `temp_dir()` (F-8, consequence
corrected to a hang); README *both* (F-9); the `PATH`-after-`chdir` claim —
checked and **true** (no finding). Not confirmed — another `renderer` case
binding a relative `goad.sock` in the crate directory (none does; the only
relative `goad.sock` in that binary is the AC-5 case's own); `PWD` with `..`
from `round_trip.rs`' non-canonical anchor (nothing in the TypeScript
exerciser or any backend reads `PWD`; only `parse`-built test commands can
carry it).

**Left standing, not raised:** the AC-5 startup case removes
`crates/goad/goad.sock` and its lock before running whether or not it made
them — harmless unless someone keeps a config in `crates/goad`.

**Round 2 (raiser)** — 2026-10-19 — the repairs, `024643d..c44ee07`.

**Outcomes:** F-1 to F-10 all `verified`. Three new nits, F-11 to F-13. No
blocker, no major, nothing contested.

**Checked and found complete:**

- **Canon wording.** `canon-delta.md`'s amended R-60 (*"each `.` component
  dropped and each `..` kept as written"*), R-17 (*"It is joined onto that
  directory as SPEC-001/R-60 joins a relative program: each `.` component
  dropped, each `..` kept."*) and the R-60 §7 directory-link sentence are,
  word for word, the text the person endorsed in `design-log.md` 2026-10-19,
  *code review round 1 dispositions*. Nothing else in the canon rows changed.
  The **Status** line records the amendment.
- **The doc-only condition, checked independently.** For each file
  `c44ee07` changed under `crates/goad-shell/src/` (`config.rs`,
  `error.rs`), the reviewer stripped every line that is a `//`-family
  comment from both `024643d` and `c44ee07` and diffed what was left: it is
  identical. No other production source changed (`git diff --name-only
  024643d c44ee07 -- crates/*/src` lists only those two). The person's
  condition for not re-running VH-1 holds.
- **Gate:** `just check` exit 0 at `c44ee07`, run by the reviewer;
  `exchange::a_relative_configured_socket_is_found_beside_the_configuration`
  green from its new start directory.
- **Class sweeps.** No counted or ordinal reference to `claim`'s class is
  left (re-grep). `goad-startup-…` is shared by a `.sock` and a `.d` helper in
  one binary, which their docs and `claim`'s state correctly. No added doc
  line cites a line number of a moving file.

**Did any repair make anything worse?** Nothing in code or behaviour. Two
records came out slightly worse. F-4's repair introduced an unwrapped line
(F-11). F-8's repair left `audit.md`'s AC-5 evidence describing the test's
old start directory (F-12). F-13 is not a regression: a kit repair that was
deliberately not carried into endorsed canon, now on the ledger so it
reaches the person before promotion.

**Noted, not raised:** the new `ConfigError::Read` / `Config::load` docs name
*a descriptor that is not a file* and give a pipe and a socket as examples.
A tty or `/dev/null` behind `/dev/stdin` canonicalizes too, and fails later or
as a parse error rather than as `Read`. The examples bound the claim, and
the docs say nothing false about those cases.

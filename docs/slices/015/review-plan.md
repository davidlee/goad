# Review — plan — Slice 015

**Subject:** plan — `docs/slices/015/plan.md` at `3190648`, read with
`design.md` (resolved, `review-design.md`)
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

**Round 1** — 2026-10-19 — the whole plan, against the design and the tree.
Bounded: at most two rounds (`plan-log.md` 2026-10-19; the person asked for a look
over).

Written by the orchestrator before the review. Surfaces to attack:

- **Coverage.** Every AC, every design §9 case and every canon-delta §7
  citation lands in a phase with a criterion that would fail if it were
  missing.
- **Red first.** Each case said to red today does red at its phase's start,
  for the reason claimed, and each mutation in the table compiles and reds the
  cases it names (no proxies).
- **Surfaces.** Each phase's closed list holds every file its tasks touch,
  and no two phases write the same file without saying so. Undeclared paths
  are the strongest lead.
- **Entry and exit.** Each phase's entry criteria are what the previous
  phase's exit criteria make true; each exit is checkable by an agent who did
  not write the code.
- **Size.** Each phase fits one agent in one session, bookkeeping included.
- **Mechanics.** The branch and worktree steps, the merges from `main`, the
  coordination with 012's walks, and the person's rule against `git stash`
  and `git checkout`.
- **The plan's own calls** (the kit not citing draft ids; the impossible
  arm as `ConfigError::Read`; per-file temp helpers; `chpwd.zsh`; the
  `justfile` comment) and anything the plan says that the design does not.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | doc-wrong | |
| F-2 | minor | doc-wrong | |
| F-3 | minor | doc-wrong | |
| F-4 | minor | doc-wrong | |
| F-5 | minor | doc-wrong | |
| F-6 | minor | doc-wrong | |
| F-7 | nit | doc-wrong | |
| F-8 | nit | doc-wrong | |
| F-9 | nit | doc-wrong | |
| F-10 | nit | doc-wrong | |
| F-11 | nit | doc-wrong | |
| F-12 | nit | doc-wrong | |
| F-13 | nit | doc-wrong | |
| F-14 | nit | doc-wrong | |

Round 1 raised by a fresh Opus agent acting as **raiser** only: severity set,
Disposition, Response and Outcome left blank. Plan read at `3190648`, tree at
`2b0ed70`.

### F-1 — the three kit example `config.toml` files state the old rule, and no phase writes them

**Severity:** major
**Location:** `plan.md` PHASE-03 Surfaces, EX-5, VA-1;
`kit/skills/goad-backend/examples/{breadcrumbs,downloads-triage,focus-check}/config.toml`

**Expected:** the design repairs "examples' configs and READMEs" (`design.md`
§10, *Not canon, repaired in the slice*), and the slice's scope names "the
examples' configs and READMEs that say to run goad from the example's
directory" (`slice-015.md` §Scope; `research.md` T2-11: "Example configs and
READMEs … say *run goad from this directory*"). A phase's closed Surfaces list
holds every file its tasks must touch.

**Observed:** each example's `config.toml` opens with the old rule as a
comment — `` # `command` and the socket are relative to this directory: run
goad and goad-check from here. `` (focus-check: `` `command` is relative to
this directory … ``). Under R-60 the instruction *run from here* is false.
PHASE-03 Surfaces lists the three READMEs, `watch.sh` and `chpwd.zsh`, and no
`config.toml`; EX-5 speaks only of the READMEs; no other phase names them.
PHASE-03/VA-1's grep (`absolute\|started in\|working directory\|from this
directory`) matches neither comment line (`relative to this directory`, `from
here`), so the agent check meant to catch survivors passes over them. Each
README's `toml goad:config` fence also reproduces the same comment verbatim,
so an agent repairing the README fence and leaving `config.toml` alone ships
a README that no longer quotes the file it shows — or edits `config.toml`
outside its Surfaces.

**Evidence:** `git grep -n 'from here' -- kit` (three `config.toml` hits, three
README fence hits); `plan.md` PHASE-03 **Surfaces**; `design.md` §10, last
bullet.

**Disposition:** doc-wrong
**Response:** PHASE-03 Surfaces now name the three example `config.toml` files (opening comment only) and each README's `goad:config` fence that quotes it; new PHASE-03/EX-6 requires the comment to state the new rule and the fence to quote the file as it now reads; new VA-3 diffs each fence against its file. VA-1's pattern gains `from here`, `relative to this directory` and `run goad`, checked against today's comment text, which it now matches in all three configs and all three fences. Overview and Coverage (AC-3) updated.

**Outcome:**

### F-2 — `round_trip.rs::shell_example`'s doc becomes false at PHASE-04, and no phase or grep reaches it

**Severity:** minor
**Location:** `plan.md` PHASE-04 Surfaces and VA-1;
`crates/goad-shell/tests/integration/round_trip.rs::shell_example`

**Expected:** after D9 nothing in the repository states the demo's old argv
(PHASE-04 Objective: "nothing in the repository states the old rule").

**Observed:** `shell_example`'s doc says *"`["bash", script]` is the argument
vector `exercisers/demo.toml` names, so this runs the exerciser exactly as the
demo does."* After PHASE-04/EX-1 the demo names `["bash", "shell/backend.sh"]`
and runs in `exercisers/`; the helper runs an absolute script from the crate
directory. `round_trip.rs` is in PHASE-01's and PHASE-02's Surfaces for other
functions only. PHASE-04/VA-1's greps are `goad-demo.sock` (not in that doc)
and a term grep scoped to `exercisers justfile .gitignore docs/memory/…`,
which excludes `crates/`. The stale sentence survives every instrument the
plan names. (`crates/goad-check/src/args.rs` `DEFAULT_TIMEOUT`'s doc also
cites `demo.toml`, and stays true.)

**Evidence:** `git grep -n 'demo.toml' -- crates`; `plan.md` PHASE-04/VA-1's
pathspecs.

**Disposition:** doc-wrong
**Response:** `round_trip.rs::shell_example`'s doc is in PHASE-04's Surfaces (doc only) with EX-7 saying what it must state. PHASE-04/VA-1's first grep is now `demo.toml\|goad-demo.sock\|shell/backend.sh` over the whole tree less `docs/slices`, so it reaches `crates/`; `args.rs` `DEFAULT_TIMEOUT`'s doc is named as staying true.

**Outcome:**

### F-3 — a phase may edit endorsed `canon-delta.md` for a renamed case without returning to the person

**Severity:** minor
**Location:** `plan.md` §Overview, *Test names are commitments*; every phase's
Surfaces ("`design.md` §9 and `canon-delta.md` only for a renamed case")

**Expected:** `canon-delta.md` **Status**: "wording endorsed by the person …
Review may still change it; any change returns to the person." Its preamble:
"Test names are `design.md` §9's; at audit each is re-pointed to the symbol
that shipped."

**Observed:** the plan lets an executing agent change test names inside the
endorsed §7 rows "in the same commit" and record it in its phase sheet, with
no consult. That changes endorsed wording mid-execution, and duplicates the
audit's re-pointing step that `canon-delta.md` already assigns (and §Owed
lists again).

**Evidence:** `canon-delta.md` **Status** line and preamble; `plan.md`
§Overview, the paragraph beginning *Test names are commitments*.

**Disposition:** doc-wrong
**Response:** The class: every place the plan let a phase edit `canon-delta.md` (and `design.md` §9). §Overview's *Test names are commitments* now says no phase edits either document: a rename is recorded in the phase sheet (§9's name, shipped symbol, file) and audit re-points from those records, as the delta's preamble assigns. Removed from every phase's Surfaces and from §Branch and worktree's commit list; §Owed's re-point bullet and PHASE-04/VA-2 now read from the recorded renames.

**Outcome:**

### F-4 — the `PWD` case's natural red is not pinned to the reason design §9 gives

**Severity:** minor
**Location:** `plan.md` PHASE-02/VT-5 and Notes *Order*; `design.md` §9 row
`the_backend_s_pwd_names_its_working_directory`

**Expected:** design §9: red today "because `PWD` is inherited from the test
process" — the backend runs and the case fails on the `PWD` assertion.
`plan.md` §Sequencing treats the natural red as evidence ("a case written
after its code never shows its natural red, only a mutation's").

**Observed:** VT-5 says only "a `python3` backend logs `os.environ["PWD"]`".
Written like the file's other cases — `["python3", "backend.py", <log>]`,
relative, as the R-60 row's opening asks — then against PHASE-01's tree
(spawn ignores the working directory) python cannot open `backend.py`, the
exchange fails, and the recorded red is a missing script, not an inherited
`PWD`. PHASE-02 Notes *Order* says "record the reds" without the assertion,
unlike the mutation-evidence rule, which requires it. M9 still holds the
behaviour; what is wrong is the natural red the sheet would record
(`docs/memory/a-mutation-can-red-the-right-case-on-the-wrong-assertion.md`
is the same class, for mutations). A `python3 -c` body, or recording each
natural red's failing assertion, would close it.

**Evidence:** `plan.md` PHASE-02/VT-5; `design.md` §9, *red today because*
column of that row.

**Disposition:** doc-wrong
**Response:** PHASE-02/VT-5's command is now `["python3", "-c", <body>, <log>]`, loaded with `Config::load`, so it names no relative path and against PHASE-01's tree it reds on the `PWD` assertion, inherited from the test process, as §9 says. The class: §Sequencing gains *Natural-red evidence* — each natural red is recorded with the assertion it failed on, and a red for another reason than §9's is not the red claimed. PHASE-01 and PHASE-02 Notes cite it.

**Outcome:**

### F-5 — 012's PHASE-10 writes `crates/goad-check/tests/` as well as `kit/`; the plan guards only the kit, and only from PHASE-03

**Severity:** minor
**Location:** `plan.md` §Branch and worktree, *Staying current*; PHASE-03/EN-2;
`docs/slices/012/plan.md` PHASE-10 Surfaces

**Expected:** design R1 and `review-design.md` F-20: coordinate with PHASE-10
"also as a writer of the same files".

**Observed:** 012 PHASE-10's Surfaces are `kit/`, `crates/goad-check/src`
(report wording) and `crates/goad-check/tests/`. 015 writes
`crates/goad-check/tests/kit/tagged.rs` (PHASE-01),
`tests/kit/{examples,main,tree,coverage}.rs` (PHASE-02) and
`tests/binary/{run,process}.rs` (PHASE-03). The plan calls PHASE-10 a writer
of "the same kit files as PHASE-03", gives a conflict rule for kit text only
("012's text with 015's rule applied to it"), and checks *PHASE-10 in
progress → STOP* at PHASE-03/EN-2 and at the merge only. PHASE-01 and
PHASE-02 start with no such check, and a conflict in a test file has no
stated resolution rule.

**Evidence:** `docs/slices/012/plan.md` `## PHASE-10` **Surfaces**; `plan.md`
PHASE-01 and PHASE-02 Surfaces and Entry.

**Disposition:** doc-wrong
**Response:** §Branch and worktree now states 012 PHASE-10's Surfaces (`kit/`, `crates/goad-check/src`, `crates/goad-check/tests/`) against the 015 phases that write the same trees, and new entry criteria PHASE-01/EN-3 and PHASE-02/EN-2 join PHASE-03/EN-2: read 012's §Status on `main`, STOP and consult if PHASE-10 is in progress. A conflict rule for test files is added: keep both sides' cases; same function changed on both sides → STOP and consult.

**Outcome:**

### F-6 — `Spawn`'s directory-less `Display` (the argv form) has no named case

**Severity:** minor
**Location:** `plan.md` PHASE-02/EX-2, VT-6; `design.md` §5.2 *Errors*

**Expected:** EX-2: the parenthesis "only when a directory is set"; design
§5.2: the argv form's report "names the executable only". Each behaviour the
design states has a case that fails without it.

**Observed:** VT-6 holds the `Some` arm only. The `None` arm is left to
"`error.rs`'s own tests", which the plan neither names nor describes. A
`Display` that always prints `(in …)`, with an empty or `None`-shaped
directory, is red under no case the plan names: `table.rs` row `T1` asserts
the prefix, and `run.rs::an_unspawnable_command_is_reported_against_the_configuration`
asserts a side word before the id.

**Evidence:** `plan.md` PHASE-02 **Verification**; `crates/goad/tests/renderer/table.rs`
row `T1` (`Prefixed("backend could not be spawned: ")`).

**Disposition:** doc-wrong
**Response:** New PHASE-02/VT-9 `error.rs::tests::a_spawn_failure_with_no_working_directory_names_only_the_executable`: `working_directory: None` gives a `Display` naming the executable with no `(in `; seen red against an always-parenthesised `Display`, and new mutation M13 reds it. EX-6 and Coverage (AC-2) cite it; recorded as an addition to §9's names for audit.

**Outcome:**

### F-7 — the listener case's red run leaves a lock file the R5 note does not clean

**Severity:** nit
**Location:** `plan.md` PHASE-01 Notes, *The listener case and R5*; VA-2

**Expected:** VA-2: "`git status` … shows no socket, lock or config written
into a crate directory".

**Observed:** the note says the case "asserts no `goad.sock` is in the test's
working directory and removes any it finds". `ingress::bind` opens the lock
(`hold`) before `UnixListener::bind` (T2-8), so the red run also writes
`crates/goad/goad.sock.lock` — the file `git status` *does* list (T2-S7), and
one that would block `git worktree remove` at close. The VT-7 note says "the
socket **and** its lock"; the listener note does not.

**Evidence:** `research.md` T2-8, T2-S7; `crates/goad/tests/renderer/startup.rs`
`listener::cleanup`, which removes both for this reason.

**Disposition:** doc-wrong
**Response:** PHASE-01 Notes *The listener case and R5* now covers `goad.sock.lock` as well as the socket, before and after the case, gives the reason (the lock is opened before the bind, T2-8, and is the file `git status` lists, T2-S7), and points at `listener::cleanup`.

**Outcome:**

### F-8 — `goad-emit`'s binary case hangs instead of failing if it joins the listener first

**Severity:** nit
**Location:** `plan.md` PHASE-01/VT-5

**Expected:** a red case fails.

**Observed:** `exchange.rs::fake_listener` blocks in `accept` on its thread. In
VT-5's red run nothing connects at the config's directory, so a case that
calls `join()` before asserting the exit status waits forever. The file's
existing cases assert first (`an_accepted_envelope_exits_0_and_says_nothing`),
but VT-5 is the first case there whose red run is expected, and the plan does
not state the order.

**Evidence:** `crates/goad-emit/tests/binary/exchange.rs::fake_listener` and
`::an_accepted_envelope_exits_0_and_says_nothing`.

**Disposition:** doc-wrong
**Response:** PHASE-01 Notes add: `goad-emit`'s binary case asserts the exit status before joining the fake listener, and joins only after a 0, so its red run fails instead of hanging in `accept`.

**Outcome:**

### F-9 — `claim`'s enumeration is already short before this slice

**Severity:** nit
**Location:** `plan.md` PHASE-01 Notes, *Temp directories, and `claim`*;
`tests/support/scripting.rs::claim` doc

**Expected:** the plan's repair: "rewrite that sentence to name the held and
unheld helpers".

**Observed:** "Six helpers … this holds four" is false today, not only after
this slice: `crates/goad-check/tests/kit/examples.rs::handout` calls itself
"a helper of `claim`'s class, unheld by it", and
`crates/goad/tests/binary/exit_codes.rs::scratch_config` writes over its path
at handout; neither is named. The rewrite has to start from `claim`'s own
grep, not from the current list. The plan also does not carry `claim`'s rule
that a new kind's prefix be checked against `marker`'s bare
`goad-<name>-<pid>`.

**Evidence:** `grep -rn 'process::id()' --include=*.rs crates tests`;
`claim`'s doc paragraphs *Six helpers make that promise* and *One kind per
helper*.

**Disposition:** doc-wrong
**Response:** PHASE-01 Notes now require rewriting `claim`'s doc from `claim`'s own grep, naming every helper of the class held and unheld — `examples.rs::handout` and `exit_codes.rs::scratch_config` among them — with no number, and carry *One kind per helper*: a new directory helper ends its name with a non-digit suffix after the pid so it can never equal a `marker` path.

**Outcome:**

### F-10 — the reason given for keeping `config.rs`'s tests module last names the wrong instrument

**Severity:** nit
**Location:** `plan.md` PHASE-01 Notes, "`config.rs`'s tests module stays last
in the file (the boundary structure check)"

**Expected:** a cited instrument holds what it is cited for.

**Observed:** `crates/goad-boundary/tests/checks/structure.rs` skips each
`#[cfg(test)]` item and resumes after it; its module doc says
`crates/goad-shell/src` has files whose test module starts before their
midpoint. It does not require a test module to be last. What fails on an
item after the test module is clippy's `items_after_test_module` under the
gate's `-D warnings`. The instruction is right; its reason sends a reader to
the wrong file.

**Evidence:** `structure.rs` module doc and `production_lines`.

**Disposition:** doc-wrong
**Response:** The reason now names clippy's `items_after_test_module` under the gate's `-D warnings`, not the boundary structure check.

**Outcome:**

### F-11 — `ConfigError::Read`'s doc becomes incomplete, and `error.rs` is not in PHASE-01's Surfaces

**Severity:** nit
**Location:** `plan.md` PHASE-01 Surfaces and Notes, *A-1's spelling*;
`crates/goad-shell/src/error.rs` `ConfigError::Read`

**Expected:** a variant's doc says what it carries.

**Observed:** `Read`'s doc is "The file is missing or could not be read."
After PHASE-01 it also carries A-1's `InvalidInput` arm ("names no file in a
directory") and D4's tolerated `/dev/stdin`-style `NotFound`, neither of
which is "missing". PHASE-01 does not list `error.rs`; PHASE-02 lists it for
`BackendError::Spawn` only.

**Evidence:** `error.rs` `enum ConfigError`; `design.md` D4.

**Disposition:** doc-wrong
**Response:** `crates/goad-shell/src/error.rs` is in PHASE-01's Surfaces for `ConfigError::Read`'s doc only (noting PHASE-02 writes it again, in sequence); EX-4 requires the doc to name what `Read` now carries: missing or unreadable, a dangling link, a path naming no file in a directory (D4, A-1).

**Outcome:**

### F-12 — the `justfile` `package` comment's count cannot be repaired by naming, and the sentence omits the kit sockets

**Severity:** nit
**Location:** `plan.md` PHASE-04 Notes, "name them or drop the number"

**Expected:** the edited sentence is true.

**Observed:** the comment says the checkout holds "`.claude/worktrees/`, six
gitignored worktrees". Today `git worktree list` shows the main checkout only
and `.claude/worktrees/` does not exist, so there is nothing to name; only
dropping the number is open. The same sentence lists the checkout's sockets
as `goad-demo.sock` alone, while the kit examples' `goad.sock` files, present
in this checkout, break a `path:` copy the same way.

**Evidence:** `git worktree list`; `ls kit/skills/goad-backend/examples/*/goad.sock`.

**Disposition:** doc-wrong
**Response:** PHASE-04 Surfaces and EX-2: the `package` comment drops the worktree count (says `.claude/worktrees/` holds gitignored worktrees, no number) and names the kit examples' `goad.sock` files beside `exercisers/goad-demo.sock`. The Notes line offering to name the worktrees is removed.

**Outcome:**

### F-13 — VH-1's steps 1 and 2 start two hosts on one socket

**Severity:** nit
**Location:** `plan.md` PHASE-04/VH-1

**Expected:** a person's script that runs as written.

**Observed:** step 1 starts goad on `demo.toml`; step 2 starts it through a
link to the same file, so on the same anchored `exercisers/goad-demo.sock`.
With the step-1 window still open, step 2 is refused *in use by a live host*
(SPEC-003/R-3) and exits 2, which a person could read as the link case
failing. Step 3 needs one host running. The plan does not say to close the
first before the second, or which host step 3 reaches.

**Evidence:** `plan.md` PHASE-04/VH-1 steps 1–3; SPEC-003/R-3.

**Disposition:** doc-wrong
**Response:** VH-1 now quits the step-1 host before step 2 (both bind the same anchored socket; SPEC-003/R-3 refuses a second), leaves the step-2 host running, and step 3 says it reaches that host; step 4 ends by quitting it.

**Outcome:**

### F-14 — AC-4's run precedes the audit's code-review repairs, and the plan does not say when it is re-run

**Severity:** nit
**Location:** `plan.md` PHASE-04/VH-1; §Owed, *Record the person's run*;
§Branch and worktree, *The merge*

**Expected:** `docs/AGENTS.md`: a slice does not close until a person has run
the software and seen the new behaviour.

**Observed:** VH-1 runs in PHASE-04, before audit's `review-code.md` rounds. A
repair to `goad_shell::config` or `backend::process` after it leaves
`audit.md` §Evidence describing a build that no longer exists. The plan names
no condition for re-running VH-1.

**Evidence:** `plan.md` §Owed and *The merge*.

**Disposition:** doc-wrong
**Response:** §Owed: if any repair from audit's code review touches `goad_shell::config` or the spawn (`backend::process`, `BackendError::Spawn`), PHASE-04/VH-1 is re-run on the repaired build before the merge and §Evidence records that run. *The merge* now requires AC-4 recorded on the build being merged.

**Outcome:**

## Round 1 — raiser's notes

Checked and found sound, so a second round can narrow.

- **Callers and sites.** `Config::parse`'s callers outside `config.rs` are
  exactly T2-6's four; `Config::load`'s are the three binaries. No struct
  literal of `config::Command` exists outside `Command::new`, so the new field
  breaks nothing at construction; the `Config` literals in `event_loop_drain`
  and `event_loop_answer` use `Command::new` and get `None`. Every
  `BackendError::Spawn` site is in PHASE-02's Surfaces (`process.rs`,
  `error.rs` and its tests, `transport.rs`, `failure_matrix.rs`, `table.rs`,
  `coverage.rs`); no `src/` outside `goad-shell` matches it. The only text
  assertion on the spawn message is `table.rs` row `T1`'s prefix. Both
  config-driven `ProcessBackend::new` calls take
  `config.backend.command.clone()`, so neither `main.rs` changes.
- **Shape guards.** `transport_shape.rs::the_only_spawn_is_the_child` counts
  lowercase `spawn` on stripped lines and requires `let mut command =
  Command::new(`; `nothing_returns_between_the_spawn_and_the_cleanup_budget`
  starts at `.spawn()`. The designed `exchange` and a struct-shaped `Spawn`
  arm satisfy both; the plan's note on lowercase identifiers is the right one.
- **Red today.** Reasoned from the code: the resolution unit case, the
  executable table and the directory-link case red on the stub the Notes
  describe; the listener case binds in `crates/goad` today; `goad-emit`'s unit
  case asserts `./from-config.sock` today and its binary case gets exit 2;
  `examples.rs` started in the hand-out root finds none of the three
  backends; the README case reds once `rooted_at_the_workspace` is gone;
  PHASE-02's VT-1..VT-4 and VT-6 red against PHASE-01's spawn (VT-5: F-4).
- **Mutations.** M1–M12 each compile under the plan's respelling rule
  (`unused = deny`, `dead_code = warn` in `Cargo.toml`, verified) and each
  reds the cases it names. M8 keeps the script case green and reds the native
  one, as T2-S2 predicts. M11's beside-decoy and M12's non-caller directory
  red their pins. M5 touches only the `UnixListener::bind` line, so lock-open
  failures stay `Unbindable`.
- **The `PATH` pin.** The built `goad-check` runs from `CARGO_BIN_EXE_…`, so
  `PATH` on its child reaches only the backend lookup; `#!/bin/sh` decoys
  using builtins need nothing on that `PATH`. The goad-check `binary` target
  `#[path]`-includes `scripting.rs`, so the plan's `claim` route is available.
- **Test-support constraints.** `std::env::set_var` and `std::env::var` are
  disallowed (`clippy.toml`); `event_loop_schedule` includes `scripting.rs`,
  so a shared helper there would be dead code; `goad-emit`'s targets do not
  include it. As the Notes say.
- **Coverage.** Every AC, every §9 row and every name in both §7 rows lands in
  a phase with a VT/VA/VH that fails without it, except F-6's arm.
- **Mechanics.** `git worktree add … -b slice-015` from the main checkout,
  `git merge main` inside the worktree, `git merge --no-ff` from main, then
  `git worktree remove` and `git branch -d`: no `stash`, no `checkout`. The
  main checkout's modified `flake.lock` is untouched by a merge that does not
  change it. Every `docs/memory/` note the plan cites exists.
- **The plan's own calls.** Not citing draft ids in `kit/` follows
  `docs/AGENTS.md` (nothing outside the slice may cite a draft). A-1 as
  `ConfigError::Read` over `InvalidInput` is total, adds no variant and keeps
  exit 2, consistent with D4. Per-file temp helpers are forced by the
  `dead_code` constraint above. `chpwd.zsh`: its advice (an absolute
  `--socket`) stays true, because `goad-emit`'s `--socket` is used as given
  and a `chpwd` hook runs from every directory; classing it under VA-2 is
  right. The `justfile` call: F-12.
- **Not re-argued.** D1–D13, D4's tolerated case and D11's residue were taken
  as settled.

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

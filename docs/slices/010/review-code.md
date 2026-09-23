# Review — implementation — Slice 010

**Subject:** implementation — `b444c6a^..f9620b6` on `main`, restricted to
`crates/` and `nix/`: the new `exit` module, the rewired `main`/`start`,
`Cancel::is_stopped`, the `report_exit` / `report_exit_line` pair, the
`structure` scan, the binary- and renderer-tier cases, and the unit comment in
`nix/module.nix`. Held to `draft-spec.md` (the slice's working canon) R-1…R-7,
`slice-010.md` AC-1…AC-11, and `CLAUDE.md`'s rules.
**Reviewer:** fresh agent (Claude Opus 5.5), round 1, own worktree
**Opened:** 2026-09-23
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

**Round 1** — 2026-09-23 — the whole slice's code at `f9620b6`, read
independently: `notes.md` §Findings / §Handover and the phase sheets' Findings
were not read before the findings below were written.

Lines of attack, written before reading the diff:

1. **Every way the process ends.** Walk `main` → `run` → `start` for `--help`,
   `--version`, each `StartupError` arm, the requested stop, an unrequested loop
   end with `Err` and with `Ok`, and a panic. For each: the status, and what
   standard error holds last (R-1…R-4, R-7).
2. **The seam.** Does `start` hand `exit::ended` the call's own result and a
   stop read taken *after* the call? Can anything but a request trip `Cancel`,
   and can anything re-file the loop's end into `Err` (R-1, R-2, A5)?
3. **Tests as proxies.** For each new case, name the regression it guards and
   check the case would red on it; mutation-run the one whose claim rests on
   process behaviour (the ingress case and the display removal) with a control
   that compiles.
4. **The line, as a line.** R-4 and P-B say *a line*, naming the binary, and
   the *last* one. Check what the process actually writes for a platform error,
   not what the pure half returns for a literal.
5. **Comment truth.** Every doc comment and comment the slice wrote or left in
   a touched file, against the code *and* against `draft-spec.md` — in
   particular against §3 P-D and §6 *What may not be inferred*, which forbid
   reading a retry prediction off a number.
6. **Project rules.** Domain vocabulary, cite-by-symbol, name-never-count,
   strata. And whether `just check` reaches what the docs say it reaches.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | |
| F-2 | major | fix-now | |
| F-3 | minor | fix-now | |
| F-4 | minor | fix-now | |
| F-5 | minor | fix-now | |
| F-6 | minor | fix-now | |
| F-7 | nit | fix-now | |
| F-8 | nit | fix-now | |

### F-1 — Three comments read a retry prediction off status 2, which the draft spec forbids and AC-7 names

**Severity:** major
**Location:** `nix/module.nix`, the comment above `Restart` in the `Service`
block; `crates/goad/src/exit.rs`, module doc; `crates/goad/tests/binary/exit_codes.rs`,
module doc (first paragraph).

**Expected:** `draft-spec.md` §3 P-D: *a status says what happened, never what
to do about it*. §6 *What may not be inferred*: *"never started does **not**
mean that trying again would fail: a socket a live host holds is released when
that host exits, and a display that was not there at one moment may be there at
the next."* AC-7: the unit comment *"states the phase rule rather than a
retryability claim that is false of `Runtime` and of `Ingress` in-use"*.
`design.md` §5.4 itself says a restart at +2 s after the compositor went away
exits 2 and is later recovered by the session target — so a retry after 2 can
succeed.

**Observed:**
- `nix/module.nix`: *"2 is a host that never started — whatever `StartupError`
  variant, a bad configuration, an unreadable clock, an ingress socket already
  held — **so a restart changes nothing a person has not changed first**"*. That
  is a retryability claim, and it lists *an ingress socket already held* — the
  exact case AC-7 says it is false of. It also opens *"The directives argue from
  **phase**, not from cause"* and then argues from predicted retry outcome.
  Smaller: *"0 is the window being closed"* omits the tray's quit, the other
  route `install` wires (`tray.on_quit`).
- `exit.rs` module doc: *"A supervisor may restart 1 and **gains nothing by
  restarting 2**"* — a recommendation and a prediction, in the one module that
  owns the numbers, under a paragraph headed *"the rule the numbers follow,
  stated here rather than cited"*.
- `exit_codes.rs` module doc: the consequence the directive prevents is
  *"restarting a host that cannot start, on a bad configuration or an ingress
  socket already held"* — the same prediction for the same held-socket case.
  (This paragraph predates the slice, but the slice rewrote this module doc and
  AC-5 names it as the one permitted change.)

**Evidence:** the three quoted sentences against `draft-spec.md` §3 P-D, §6,
and `slice-010.md` AC-7; `startup::listener` → `ingress::bind` → `hold` answers
`BindFault::InUse` only while another process holds the lock, which is released
when it exits (`draft-spec.md` §6 says so in terms). The class is *a document
restating the retryability axis the spec rejects*; fixing only the unit comment
would leave the module that owns the numbers teaching it.

**Disposition:** fix-now
**Response:** Accepted as raised, and as a class: every sentence in the slice's diff that reads a retry outcome off a status is repaired, not only the three quoted. `nix/module.nix` states the unit's policy (restart 1, do not restart 2) as the unit's choice, arguing from phase, predicting nothing; it names both routes into 0 (the window's close request and the tray's quit). `exit.rs`'s module doc and `exit_codes.rs`'s module doc state what each number means and leave restart to the supervisor (`draft-spec.md` §3 P-D). Carries audit A-1 (user decision, `design-log.md` *at audit*). The repair greps the diff for the class (*restart*, *retry*, *gains nothing*, *changes nothing*) and says in the commit what it found.

**Outcome:**

### F-2 — The "line" beside a platform failure is several lines, and the last one names neither the binary nor what happened

**Severity:** major
**Location:** `diagnostics::report_startup_line`, `diagnostics::report_exit_line`
(`crates/goad/src/diagnostics.rs`); `draft-spec.md` R-4's §7 row.

**Expected:** R-4: every non-zero exit is accompanied by *a line* on standard
error *naming the binary that wrote it and what happened*, and *it MUST be the
last line the process writes there*. P-B the same. R-6: *stopped running*'s line
says the host had been running.

**Observed:** both pure halves interpolate `{error}` raw
(`format!("goad: {error}")`, `format!("goad: the host was running and stopped:
{error}")`). Slint's backend selector builds its error as
`PlatformError::Other(format!("Could not initialize backend.\n{}", …))` with one
line per backend tried (`i-slint-backend-selector-1.17.1/lib.rs`,
`create_default_backend`). Run headless, the host writes **three** lines, and
the last is `No backends configured.` — no `goad: `, no phase:

```
$ env -u WAYLAND_DISPLAY -u WAYLAND_SOCKET -u DISPLAY target/debug/goad c.toml; echo status=$?
goad: the display could not be opened: Could not initialize backend.$
Error from Winit backend: Error initializing winit event loop: os error at …/winit-0.30.13/src/platform_impl/linux/mod.rs:765: neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set.$
No backends configured.$
status=2
```

(`c.toml`: a valid `[backend]` and `[schedule]`, no `[ingress]`; `cat -A`
output.) This is the *display could not be opened* line AC-6 sets against the
*stopped running* line, and the same raw interpolation is on
`Ended::StoppedRunning(Some(error))`'s arm, so any multi-line `PlatformError`
at the seam leaves a last line that does not say the host had been running.

**Evidence:** the transcript above, at `f9620b6`. The §7 R-4 row says *last
line the process writes* is held by `too_many_arguments_exits_2_and_says_who_spoke`
and `no_argument_and_no_configuration_home_exits_2`, which read the whole
stream — but both are variants whose `Display` is a single literal line, so they
hold *last line* only for errors that cannot contain a newline: a proxy for the
property. The `diagnostics` module doc claims *"the escape/bound pipeline every
line on this surface goes through"*; these two outlets do not go through it.

**Disposition:** fix-now
**Response:** Verified at `f9620b6`: `finish` (escape, then bound) is applied by `Diagnostics::of`, `Diagnostics::refused` and `next_check_line`; the three stderr outlets — `report_startup_line`, `report_exit_line`, `report_platform_line` — interpolate raw. No recorded reason for the bypass. **User decision (2026-09-23): route all three through `finish(…, LINE_LIMIT)`** — one line, the platform's cause kept, escaped as `\n`. Rejected: first line only (loses the winit cause on line 2); `goad: ` on every line (the last still says neither phase nor cause). A renderer-tier case builds a multi-line `PlatformError` through `From<String>` and asserts each outlet's answer contains no line terminator and begins with its fixed prefix; `draft-spec.md` §7's R-4 row cites it for *last line* in place of the two single-literal cases' proxy. The module doc's *every line on this surface* becomes true rather than reworded.

**Outcome:**

### F-3 — `--help` and `--version` exit 0 when the answer could not be written

**Severity:** minor
**Location:** `run` (`crates/goad/src/main.rs`), the `Launch::Help` and
`Launch::Version` arms; `goad_shell::report::line_to`.

**Expected:** R-1: exit 0 *if, and only if,* the process ended doing what it was
asked — *an invocation that was a question, answered*. The §7 R-1 row says the
*only if* half is held by `exit::status` answering 0 for `Ended::AsAsked` alone.

**Observed:** `run` answers `Ok(Ended::AsAsked)` unconditionally after
`print_usage` / `print_version`, which go through `line_to`, best effort by
design (*"the exit code still carries the fact"* — but here the exit code
carries the opposite). A question whose answer never reached standard output
exits 0 with nothing on standard error.

**Evidence:** `target/debug/goad --help > /dev/full; echo $?` prints `0`
(measured at `f9620b6`). `exit::status`'s tests hold the *shape* → number map;
nothing holds that `AsAsked` was earned. Either the code or R-1's wording
("answered") is wrong; which is a disposition.

**Disposition:** fix-now
**Response:** Accepted: the code is wrong, not R-1. **User decision (2026-09-23): fix-now in `goad`, follow-up for `goad-emit`.** `print_usage` and `print_version` report whether the write succeeded; a failed write is a failure before the loop call, so exit 2 (R-3) through a new `StartupError` variant with an R-4 line on stderr (`goad: the answer could not be written to standard output: {error}`, through the F-2 pipeline). `line_to` stays best-effort for its other callers; its doc's *the exit code still carries the fact* is corrected where it is false. Held by a binary-tier case spawning `--help` with stdout on `/dev/full`, asserting 2 and the prefix, cited in §7's R-1 row for the *only if* half. Cost recorded: a reader that closes the pipe before reading (`| head -0`) now gets a broken-pipe line and 2. `goad-emit` shares the pattern and is a row in `slice-010.md` §Follow-ups.

**Outcome:**

### F-4 — `exit::status`'s stated reason for `u8` is false on the pinned toolchain

**Severity:** minor
**Location:** `exit::status` doc (`crates/goad/src/exit.rs`).

**Expected:** a doc comment true of the code and toolchain it builds with.

**Observed:** *"`u8` and not `ExitCode`: `ExitCode` carries no `PartialEq`, so a
function answering one could not be asserted by any test"*. On the flake's
toolchain (`rustc 1.99.0-beta.7`), `std::process::ExitCode` implements
`PartialEq`.

**Evidence:** a scratch program
`fn main(){ let a=std::process::ExitCode::from(1); let b=std::process::ExitCode::from(1); println!("{}", a==b); }`
compiled with the devshell's `rustc` prints `true`. The choice of `u8` may
still be right (a number reads better in an assertion failure); the argument
given for it is not.

**Disposition:** fix-now
**Response:** Reword; keep `u8`. The true reason is the one the raiser names: a bare number is what an assertion failure prints and what a supervisor reads. The false claim about `ExitCode` goes.

**Outcome:**

### F-5 — `report_exit_line` says each sentence is true of exactly one situation; the seam makes that false

**Severity:** minor
**Location:** `diagnostics::report_exit_line` doc, and `diagnostics::report_exit`
doc (`crates/goad/src/diagnostics.rs`).

**Expected:** `draft-spec.md` §2 adopts SPEC-003 P-D for absolute clauses: a
clause that says a mechanism always holds names its exception in the sentence.
§5 *What the seam costs* and R-6: where the event-loop call fails on entry, the
*stopped running* line says the host had been running, *"for that shape it is
wrong"*.

**Observed:** *"Each sentence is true of exactly one situation, which is what
makes the line worth reading"* — with no mention of the on-entry failure, for
which *"the host was running and stopped"* is false. `exit.rs`'s `Ended` doc
does carry the seam; this doc, which is about the sentence itself, does not.
Smaller: `report_exit`'s doc, *"stderr, once, last."*, is untrue for
`Ended::AsAsked`, which writes nothing.

**Evidence:** the doc text against `draft-spec.md` §5 and R-6's own exception
clause; `report_exit_line`'s `Ok(Ended::AsAsked) => None` arm.

**Disposition:** fix-now
**Response:** `report_exit_line`'s *exactly one situation* names its exception in the sentence (the call failing on entry, `draft-spec.md` §5 *What the seam costs*). `report_exit`'s *once, last* says `Ended::AsAsked` writes nothing.

**Outcome:**

### F-6 — Binary-tier docs still describe the pre-slice seam, and one counts

**Severity:** minor
**Location:** `crates/goad/tests/binary/exit_codes.rs` (module doc first line;
`help_prints_the_usage_block_on_stdout_and_exits_0` doc);
`crates/goad/tests/binary/main.rs` (module doc).

**Expected:** comments in touched files true of the code now; `CLAUDE.md`
*name, never count*.

**Observed:**
- `exit_codes.rs`: *"The exit code `main` chooses"* — `main` no longer chooses;
  `exit::status` does, and the same module doc two paragraphs down says so.
- `help_prints_the_usage_block_on_stdout_and_exits_0`: *"`Ok(())` is exit 0"* —
  `run` answers `Ok(Ended::AsAsked)` now. (AC-5 forbids modifying the case; its
  doc is a separate question for the responder.)
- `tests/binary/main.rs`: *"every answer it asserts is reached before the first
  Slint call — the two zero-exits inside `run`, and the startup failures `start`
  settles in its **first step**"*. `an_unbindable_ingress_path_exits_2`, added
  by this slice, settles at `start` step 3 (`startup::listener`). And *"the two
  zero-exits"* counts `Launch`'s answering variants — the enum whose own doc
  records being hand-incremented from two to three (`Launch`,
  `crates/goad/src/startup.rs`).

**Evidence:** the quoted text against `main`, `run` and `start` at `f9620b6`,
and against `an_unbindable_ingress_path_exits_2`'s own doc (*"`startup::listener`
runs at `start` step 3"*).

**Disposition:** fix-now
**Response:** Under the AC-5 waiver for doc comments (`design-log.md` *at audit*). `exit_codes.rs` says `exit::status` chooses; the `help_…` case's doc says `Ok(Ended::AsAsked)`; `tests/binary/main.rs` names the rule (*before the first Slint call*) instead of *first step* and *the two zero-exits*. Carries P3-a and P3-c.

**Outcome:**

### F-7 — `process::command`'s doc conflates what the removal holds, and leaves out what it depends on

**Severity:** nit
**Location:** `command` doc (`crates/goad/tests/binary/process.rs`).

**Expected:** a doc that says what the mechanism holds and what it rests on.

**Observed:** *"Every case here settles before the first Slint call, and nothing
but this removal keeps it that way"* — the removal does not keep a case settling
before Slint; it keeps a case that does not settle from hanging (the doc's next
sentences say exactly that). And *"removing all three leaves no backend to fall
back to"* is true because `goad` is built with `backend-winit` alone
(`cargo tree -p goad -i i-slint-backend-linuxkms` prints nothing; the selector's
default is winit only) — a Slint feature change that admits `linuxkms` or `qt`
changes the fallback list, and the doc names the winit version but not this.

**Evidence:** the doc text; `i-slint-backend-selector-1.17.1/lib.rs`
`create_default_backend`; `cargo tree` above.

**Disposition:** fix-now
**Response:** The doc says what the removal holds — a case that does not settle before Slint fails fast rather than hanging — and what it rests on: `goad` builds with Slint's winit backend alone, so a feature that admits another backend changes the fallback.

**Outcome:**

### F-8 — `diagnostics`' module inventory omits the pure half the slice added

**Severity:** nit
**Location:** module doc (`crates/goad/src/diagnostics.rs`).

**Expected:** the inventory this module doc keeps, phase by phase, lists what is
in the module.

**Observed:** the slice rewrote the sentence to name `report_exit` (*"the impure
outlet renamed `report_exit` at 010/PHASE-02"*) and did not add
`report_exit_line`, the pure half and the one the tests assert.

**Evidence:** the module doc against `diagnostics::report_exit_line`.

**Disposition:** fix-now
**Response:** Add `report_exit_line` to the inventory, beside `report_exit`. Carries PHASE-02's *renamed* finding on the same sentence.

**Outcome:**

### Checked and found complete (round 1)

- **The gate.** `just check` exits 0 at `f9620b6` (build, `cargo test
  --workspace`, `cargo test -p goad-semantics`, `deno check`, clippy
  `-D warnings`, `fmt --check`); it runs `goad-boundary`'s `structure` scan and
  both goad tiers, so the cases §7 cites are in the gate.
- **The decision and the number.** `exit::ended` reads only `stop_requested`
  to choose the variant and `call` only for the carried error; `exit::status`
  is 0/1/2 over `AsAsked` / `StoppedRunning(_)` / `Err(_)`, reading no
  `StartupError` variant. The four `ended` cases hold one result against both
  requests (the error pair over one value), so a classifier reading `call` in
  either direction reds one of them; the `exit_status` cases pin each shape's
  number. Not proxies.
- **The wiring.** `start` binds `stop_signal = cancel.clone()` before `cancel`
  moves into `serve`; `Cancel` derives `Clone` over one `watch` channel, so the
  clone's `is_stopped` sees a trip through any other clone. The only `Wire::stop`
  callers are `install`'s `on_close_requested` and `tray.on_quit`; nothing else
  in `crates/goad/src` trips `Cancel`. The read is taken after
  `run_event_loop_until_quit` returns and passed with the call's own result.
  `Ended::StoppedRunning` is constructed only in `exit::ended`; nothing
  downstream turns an `Ended` into an `Err`. `StartupError::Platform`'s doc no
  longer lists the loop call (AC-3).
- **The scan.** `structure::the_loop_s_ending_is_never_a_startup_failure`
  counts one production site and requires the line to end at the call; a
  rustfmt-split `….map_err(…)?` leaves a first line not ending in `();` and is
  caught. Its stated limit (a re-filing off the call's line) is declared.
- **Mutation, compiled.** `startup::listener`'s `Some` arm replaced with
  `Ok(Ingress::none())` (first attempt did not compile — unused import — and
  was discarded; second kept the import live): the binary tier ran,
  `an_unbindable_ingress_path_exits_2` failed with
  `goad: the display could not be opened: …` and the other six passed. So the
  prefix is what discriminates the case, and a spawn past the socket fails fast
  rather than hanging. Restored by byte copy from the scratch directory, `diff`
  clean, `git status` clean.
- **No litter.** The ingress case refuses at `reclaim`'s `NotASocket` before
  `hold` creates a lock file; nothing is left in the temp directory.
- **Citations.** Every test and symbol `draft-spec.md` §7 names resolves to
  exactly one definition (`help_prints_…` resolves twice only because
  `goad-emit` has a namesake). No added line cites a line number.
- **Rules.** No domain vocabulary in the new identifiers (`Ended`, `AsAsked`,
  `StoppedRunning`, `ended`, `status`, `is_stopped`); `exit` is stratum 3 and
  names Slint, `goad-semantics` untouched. `lib.rs`'s module count was replaced
  by the rule.
- **AC-5.** The pre-existing `exit_codes` cases are byte-unchanged; the diff
  touches the module doc and appends one case and one helper.
- **SPEC-003/R-4's stale sentence** (*"`main`'s single `match run()` … maps
  every `Err` to exit 2"*) is still in canon, and `canon-delta.md` Change 1
  carries its repair — correct to leave for promotion.

**Not reached.** Whether anything writes to standard error after `report_exit`
on the *stopped running* path — the design's *"nothing runs after it"*. If the
loop ends before `serve` returns, the `spawn_local` future is still pending and
is dropped whenever its last `Arc` goes, possibly at thread-local teardown after
`main` returns. Not observable without a display; AC-9's run is where it would
show.

**Cross-check, written after the findings above** (`notes.md` §Handover
*Findings carried to audit*, and the phase sheets' mutation tables, read only
now).
- **Agree, independently found:** F-6's *"`Ok(())` is exit 0"* and *"first
  step"* are PHASE-03's; the *"0 is the window being closed"* aside in F-1 is
  PHASE-03's; F-8 is adjacent to PHASE-02's *"renamed"* finding on the same
  module doc. The listener mutation above repeats M-14 and agrees with it.
- **Not in the carried list:** F-1's substance (three retry predictions off 2,
  one of them in the unit comment AC-7 governs), F-2 (multi-line line, last
  line not naming the binary), F-3, F-4, F-5, F-7. M-13's recorded panic text
  (*"Could not initialize backend. … neither WAYLAND_DISPLAY …"*) is the
  multi-line line of F-2, seen and read as one line.
- **Carried there and not re-raised here:** the crate-root
  `wildcard_enum_match_arm` deny not reaching `exit::status`, and `lib.rs`'s
  `path:line` citations — PHASE-01's, outside this round's diff-line scan;
  no disagreement.

## Synthesis

<!-- Written when the ledger resolves. -->

# Review — implementation — Slice 010

**Subject:** implementation — `b444c6a^..f9620b6` on `main`, restricted to
`crates/` and `nix/`: the new `exit` module, the rewired `main`/`start`,
`Cancel::is_stopped`, the `report_exit` / `report_exit_line` pair, the
`structure` scan, the binary- and renderer-tier cases, and the unit comment in
`nix/module.nix`. Held to `draft-spec.md` (the slice's working canon) R-1…R-7,
`slice-010.md` AC-1…AC-11, and `CLAUDE.md`'s rules.
**Reviewer:** fresh agent (Claude Opus 5.5), round 1, own worktree; a second
fresh agent, round 2, own worktree; a third fresh agent, round 3, own worktree
**Opened:** 2026-09-23
**State:** closed

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

**Round 2** — 2026-09-23 — the round-1 repairs, `e412953..3434b76` on `main`
(`aede3df`, `33c4654`, `ebaba86`, `cc0db76`): code, tests, the
`nix/module.nix` comment, and the promoted canon (`docs/specs/004-process-exit-status.md`,
the `docs/specs/003-host-event-ingress.md` amendments) held to
`canon-delta.md` and `audit.md` §Reconciliation C-1…C-6. Fresh agent, own
worktree. Written before the diff was read; `notes.md` §Handover *Repairs,
round 1* not read until the round-2 findings and outcomes below were written.

Lines of attack:

1. **Outcomes as a class.** For each of F-1…F-8, re-derive the class the
   Response names and grep the whole tree for it, not the three quoted sites —
   in particular F-1's retry vocabulary across `crates/`, `nix/` and canon,
   and F-2's raw `{error}` interpolation into any stderr outlet.
2. **Tests red on their regression.** Mutation-run each new case (the
   multi-line outlet case, the `/dev/full` case) with a mutation that compiles;
   record the failing assertion.
3. **`line_to` / `try_line_to`.** The added flush and fallibility: every
   caller in `goad`'s host stderr outlets and in `goad-emit`; whether a
   caller's behaviour changed; whether *one way to write a line* is true
   (count the write paths to stdout/stderr in the binaries).
4. **`StartupError::AnswerUnwritten`.** Its line through the escape pipeline;
   its status (2, through `exit::status`, reading no variant); its `Display`;
   whether every doc, comment and spec clause that enumerates `StartupError`'s
   variants or the edges into 0 and 2 is still true — and whether 0 is still
   *only if* earned once `--help` can fail.
5. **F-2's escape, downstream.** Whether any binary-tier case asserts an exact
   stderr line that the escape or bound now changes; what the journal reader
   now sees (a `\n` escape, a bound truncation marker) and whether the spec
   says so.
6. **Promoted canon against the delta.** SPEC-004 against `draft-spec.md` as
   endorsed and `canon-delta.md` exactly; SPEC-003's amended cells and §9;
   every `module::case` citation in canon resolves to exactly one definition;
   no line-number citation added; no `DRAFT` / `draft-spec` / `SPEC-NNN`
   residue outside the slice folder; name-never-count in the new canon.
7. **The gate.** `just check` exits 0 at `3434b76` before any mutation; the
   total recorded.

**Round 3** — 2026-09-23 — the round-2 repairs, `5b51c90..1c1fe9d` on `main`
(`e0f1488`, `07c71f3`, `1c1fe9d`), and the `docs/follow-ups.md` FU-4 edit
the repair made beyond its brief. Fresh agent, own worktree (fast-forwarded
from `5b51c90`). Written before the diff was read; `notes.md` §Handover
*Repairs, round 2* not read until the findings and outcomes below were
written.

Lines of attack:

1. **Outcomes as a class.** F-1 (re-disposed) and F-9…F-15, each against
   the class its Response names: F-1's retry vocabulary over `crates/`,
   `nix/` *and* `docs/specs/`; F-9's present-tense history over all of
   SPEC-004, not §1 alone; F-10/F-11/F-15 over every stderr outlet.
2. **One escape or two.** Whether `diagnostics`' stderr path and `finish`
   share one escape function or each has its own. Whether dropping the bound
   is safe for every `{error}` that reaches a stderr outlet: who controls
   each one's length (the user's file, the platform, a backend, a peer on
   the socket).
3. **Tests red on their regression.** Mutation-run each new or changed case
   (the long-configuration tail case, the terminator case, the flush case),
   with mutations that compile: restore the bound, drop the terminator step,
   remove the flush.
4. **Stale claims of the old shape.** Grep the repo (outside closed slices'
   records) for any sentence that still says a stderr line is bounded, that
   *every line* passes `finish`, or that names `LINE_LIMIT` for stderr.
5. **The amended canon.** SPEC-004 §1, §5's diagram, the R-1 and R-4 §7
   rows: true of the code; inside the endorsed scope (`design-log.md`,
   *code review round 2*); no counts; no line-number citations; every cited
   symbol resolves; formatted like the surrounding canon (and the diagram
   parses).
6. **The *today*s left standing.** SPEC-004 §2 and §8: each *today* judged
   as true of the tree, a history, or a retry prediction.
7. **Beyond the brief.** The FU-4 edit in `docs/follow-ups.md`: what it
   changed, whether it was asked for, whether it is true.
8. **The gate.** `just check` exits 0 at `1c1fe9d` before any mutation; the
   total recorded.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | verified |
| F-2 | major | fix-now | verified |
| F-3 | minor | fix-now | verified |
| F-4 | minor | fix-now | verified |
| F-5 | minor | fix-now | verified |
| F-6 | minor | fix-now | verified |
| F-7 | nit | fix-now | verified |
| F-8 | nit | fix-now | verified |
| F-9 | minor | fix-now | verified |
| F-10 | minor | fix-now | verified |
| F-11 | nit | fix-now | verified |
| F-12 | minor | doc-wrong | verified |
| F-13 | minor | fix-now | verified |
| F-14 | nit | fix-now | verified |
| F-15 | nit | fix-now | verified |
| F-16 | minor | fix-now | verified |
| F-17 | nit | fix-now | verified |
| F-18 | nit | fix-now | verified |
| F-19 | minor | fix-now | verified |
| F-20 | nit | fix-now | verified |
| F-21 | nit | fix-now | verified |
| F-22 | nit | fix-now | verified |
| F-23 | nit | fix-now | verified |

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

**Outcome:** contested (round 2). The three quoted sites are repaired, and a
grep of `crates/` and `nix/` for the class finds no surviving prediction. But
the Response's scope was *every sentence in the slice's diff*, and the slice's
diff included the draft spec, now `docs/specs/004-process-exit-status.md`. Its
§1 *Intent*, second paragraph, promoted unchanged:

> A supervisor told 2 is told *a person must change something first*, and so
> does nothing; which is right for the configuration and wrong for the
> display, **where the host would have come back on the next try**.

That sentence reads a retry outcome off a cause, and it is exactly what the
same document's §6 *What may not be inferred* forbids: *"a display that was
not there at one moment **may** be there at the next … Neither number is a
prediction."* `notes.md` shows why the grep missed it: it ran over
`git diff b444c6a^..HEAD -- crates nix`, not over the draft, though its own
pattern list includes *next try* and *comes back*. The class is now in
canon, where the Response's own citation (P-D) lives. Evidence:
`grep -n 'next try' docs/specs/004-process-exit-status.md` returns §1's line.
(The same paragraph's present-tense history is a separate defect, F-9.)

**Re-disposition (round 2):** fix-now. The contest is correct: the class grep stopped at `crates` and `nix`. SPEC-004 §1's paragraph is rewritten with F-9 (user decision and canon endorsement, 2026-09-23), and the class grep (*restart*, *retry*, *next try*, *gains nothing*, *changes nothing*, *would have come back*) is re-run over `docs/specs/` as well as `crates` and `nix`, its result stated in the commit.

**Outcome (round 3):** verified. §1's paragraph is gone, and with it
*next try*. The class grep (*next try*, *come(s) back*, *gains nothing*,
*changes nothing*, *would have come*, *restart … changes/gains/would*,
*retry will/would*, *would fail/succeed again*) over `crates/`, `nix/`,
`docs/specs/`, `docs/policy/`, `docs/adr/`, `docs/brief.md` and
`docs/follow-ups.md` finds no retry outcome read off a status. Every hit is
unrelated (a present that *changes nothing*, a value that *comes back*).
`07c71f3`'s message states the grep and its result. Outside that scope,
`docs/memory/exit-2-means-two-different-failures.md` still says *"Ask, at
the variant, whether it would succeed on a retry"*. It is not canon, and
`audit.md`'s reconciliation row R-2 already carries its rewrite, so it is
not raised here. Two retry-axis sentences the repair judged true of the tree
are F-18 and F-19, which are new findings and not a contest. Neither reads
an outcome off a status.

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

**Outcome:** verified (round 2). All three stderr outlets now call
`finish(…, LINE_LIMIT)`. The `Err` arm of `report_exit_line` inherits it
through `report_startup_line`. A grep of `crates/*/src` for
stdout/stderr writes finds six sites. All six are in `goad-emit`'s
`to_stdout`/`to_stderr` or `diagnostics`, and no other outlet interpolates
raw. Mutation, compiled: taking `finish` off `report_exit_line`'s
`StoppedRunning(Some(_))` arm (the middle of the case's three outlets) reds
`stderr_outlets::a_multi_line_platform_error_is_one_line_from_every_outlet`
at its per-outlet assertion, so the loop does not hide an outlet. The case's
`ends_with("\\nNo backends configured.")` also reds first-line-only and
replace-with-space. Measured on the process: `goad bad.toml` writes exactly
one line. What the repair costs a person reading that line is F-10 and F-11.
*Every line on this surface* is true save one arm (F-15).

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

**Outcome:** verified (round 2), for the class the Response names: a write
that the device *refuses*. Measured: `goad --version > /dev/full` writes
`goad: the answer could not be written to standard output: No space left on
device (os error 28)` and exits 2. Mutations, both compiled (the arm's `?`
replaced with `match … { Ok(()) | Err(_) => () }`): swallowing `--help`'s
result reds `exit_codes::an_answer_that_cannot_be_written_exits_2` with
`--help: left: 0, right: 2`, and swallowing `--version`'s reds it too, so the
case's loop holds both questions. `exit::status` still reads no variant. The
`AnswerUnwritten` `Display` goes through `finish`. The `StartupError` doc's
sources sentence names the new variant. `goad-emit`'s row is in
`slice-010.md` §Follow-ups. The Response does not reach one edge: a
*closed* standard output still exits 0 (F-12).

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

**Outcome:** verified (round 2). The `PartialEq` claim is gone, and the new
reason is true. On the devshell's `rustc`, `println!("{:?}",
ExitCode::from(2))` prints `ExitCode(unix_exit_status(2))`, the *`Debug`
wrapper* the doc names.

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

**Outcome:** verified (round 2). `report_exit_line`'s doc puts the on-entry
exception inside the *exactly one situation* sentence and cites SPEC-004 §5
*What the seam costs*, which exists under that heading. `report_exit`'s doc
reads *or nothing at all, for `Ended::AsAsked`*. No other doc in
`diagnostics` claims one-situation-per-sentence.

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

**Outcome:** verified (round 2). All three sites are repaired. The rule in
`tests/binary/main.rs` is *the questions `run` answers, and the startup
failures that settle before `start` constructs its first component*. It
covers `an_answer_that_cannot_be_written_exits_2`, which settles in `run`,
and `an_unbindable_ingress_path_exits_2` at `start` step 3. It also agrees
with `exit_codes.rs`'s *past step 4 fails fast at `PromptWindow::new`*.
No count survives in either module doc.

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

**Outcome:** verified (round 2). The doc now says what the removal holds: a
case that does not settle fails fast rather than hanging. It names the winit
pin and the winit-only build, and it names the features that would change
the fallback (`backend-linuxkms`, `backend-qt`).

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

**Outcome:** verified (round 2). The module doc reads *the impure outlet that
010/PHASE-02 replaced with `report_exit` and its pure half,
`report_exit_line`*. The PHASE-02 *renamed* wording is gone.

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

### F-9 — SPEC-004 §1 states the pre-slice host as the present

**Severity:** minor
**Location:** `docs/specs/004-process-exit-status.md` §1 *Intent*, second
paragraph.

**Expected:** `docs/AGENTS.md` §Documentation: *"Canon is normative and
evergreen: it states what is true now. No changelogs, no revision history."*
The spec's own header comment says the same (*"No changelog, no revision
history, no 'we used to'"*). The repair applied this rule to R-3's §7 cell,
dropping *"All but one are unchanged by this document's arrival"* as *"a
count and a history in canon"*, and the user endorsed that
(`design-log.md`, 2026-09-23, *two edits the repair made beyond its brief*).

**Observed:**

> The host's number **today** says less than it appears to. **Every failure
> exits 2**, whether the host could not read its configuration before it
> opened anything or ran for hours and then lost its display. … **This
> document is the repair**: …

At `3434b76` this is false of the tree. Under R-2 and `exit::status`, a host
that lost its display exits 1. The paragraph was true while the document was
a draft, and the promotion did not re-read it.

**Evidence:** the quoted text against SPEC-004 R-2 and against
`exit_status::stopped_running_is_1`. The design-log entry above shows the
promotion already treated a history clause in canon as a defect. This is a
second instance of the same class, three sections earlier. F-1's contest is
the prediction in the same paragraph. This finding is its tense.

**Disposition:** fix-now
**Response:** **User decision (2026-09-23), with F-1's re-disposition:** SPEC-004 §1's *today* paragraph is rewritten to state the rule alone — the status names the phase the process ended in, a fact it observes, not a judgement about whether trying again would work. The history lives in `slice-010.md` and FU-1. Endorsed canon edit.
**Outcome:** verified (round 3). §1 now states the rule alone. It has no
*today*, no *every failure exits 2*, and no *this document is the repair*.
The next paragraph's *"Once this exists"* is now *"With the status
stated"*, the same tense class inside §1, so it was caught too. A grep of
SPEC-004 for *today*, *repair*, *arrival*, *used to*, *no longer* and
*previously* leaves §2's and §8's *today*s. Those are judged under the
round-3 Brief item 6. One of them is false (F-18). The sentence the repair
wrote keeps an absolute that §5 qualifies (F-16).

### F-10 — The line bound cuts the cause off a configuration error

**Severity:** minor
**Location:** `diagnostics::report_startup_line` (the `finish(…, LINE_LIMIT)`
F-2 added), for `StartupError::ConfigUnparseable`.

**Expected:** F-2's Response: *"one line, **the platform's cause kept**"*.
SPEC-004 §6: *"Why is in the line R-4 requires, and not in the number."*
The bound was chosen for a line *"enough for a serde message quoting a
document, an OS error"* (`LINE_LIMIT`'s doc). That line is a message, and it
was never meant to be one that carries a source excerpt.

**Observed:** `toml`'s `Display` puts the message **last**. It comes after
the position, the quoted source line and a caret line padded with spaces to
the error's column. So the excerpt costs about twice the column in
characters before the message begins. Once the escaped line passes 1024
characters, `bound` keeps the first 1024 and the message is the part it cuts.
Measured, on a config whose sixth line has an error at column 1511
(`note = "aaa…" x`):

```
goad: …/long.toml: configuration is not valid: TOML parse error at line 6, column 1511\n  |\n6 | note = "aaaa…aaaa [2235 more characters not shown]
```

The person gets a position, then a run of the source line, then the marker.
What was wrong with the file is gone. Before F-2 the whole message reached
the stream. The threshold is roughly (1024 − prefix) / 2 — a column of about
450 with a short path. That is rare, but the cost falls on the one arm
whose text comes from a parser (§6: *why is in the line*). The F-2 decision
was taken on a platform error, and its evidence did not include this arm.

**Evidence:** the transcript above (`target/debug/goad` at `3434b76`,
`long.toml` built by a scratch script). `bound` keeps a prefix by design,
and it knows nothing about which part of a composed line is the cause. No
case holds a configuration line's tail: the binary tier asserts
`an_unparseable_configuration_…`'s prefix only, by design.

**Disposition:** fix-now
**Response:** **User decision:** the stderr outlets (`report_startup_line`, `report_exit_line`, `report_platform_line`) escape and do **not** bound. The bound exists for lengths a backend or transport chose (D53); these lines carry the user's own configuration's parse error or the platform's, and journald's own limit is far above `LINE_LIMIT`. Rejected: raising the bound to `STDERR_LIMIT` (moves the threshold, keeps the class). A case holds that a long configuration error keeps its message tail.
**Outcome:** verified (round 3). `report_startup_line`,
`report_exit_line`'s `StoppedRunning(Some(_))` arm and
`report_platform_line` now take `one_line`. `one_line` is
`Escaped(without_one_terminator(composed))`, and it has no bound.
`finish` and `LINE_LIMIT` are left with in-window callers only. Mutations,
all compiled:
- A bound restored inside `one_line` (`bound(…, LINE_LIMIT)`) reds
  `a_configuration_error_far_along_a_long_line_keeps_the_parser_s_message`
  on the *more characters not shown* assertion. It also reds
  `no_stderr_outlet_bounds_its_line`.
- Reverting `report_platform_line` alone to `finish(…, LINE_LIMIT)` reds
  `no_stderr_outlet_bounds_its_line` and
  `no_stderr_outlet_ends_in_a_visible_terminator`.
- Reverting `report_startup_line` alone reds all three new cases.

So the per-outlet loop does not hide an outlet. Measured on the process: a
configuration whose line has 100,000 characters writes one 200,260-byte line
ending *`` unexpected key or value, expected newline, `#` ``*, which is the
message kept. What that length costs is F-20.

### F-11 — Every configuration-parse line now ends in a visible `\n`

**Severity:** nit
**Location:** `diagnostics::report_startup_line`, via `finish`, for
`StartupError::ConfigUnparseable`.

**Expected:** the precedent in the same module. `without_one_terminator`
exists because a backend's stderr ending in its own newline *"rendered … as a
visible `\n` at the end of the record"*, and it strips one terminator before
`finish` for exactly that reason.

**Observed:** `toml`'s message ends in a newline. Escaped, that newline
becomes a literal `\n` at the end of the line, before the real terminator:

```
$ goad bad.toml     # contents: this is not toml {{{
goad: …/bad.toml: configuration is not valid: TOML parse error at line 1, column 6\n  |\n1 | this is not toml {{{\n  |      ^\nkey with no value, expected `=`\n
```

**Evidence:** the transcript above, at `3434b76`. F-2 routed the startup
line through `finish` without the terminator step that the backend's stderr
line has.

**Disposition:** fix-now
**Response:** With F-10: the stderr outlets drop at most one trailing terminator through the existing `without_one_terminator` before escaping — the rule this module already follows for captures, reused, not restated.
**Outcome:** verified (round 3). The terminator step reuses
`without_one_terminator`. There is one escape, `Escaped`, composed two
ways: `finish` escapes and bounds, and `one_line` drops a terminator and
escapes. So nothing is implemented twice. Two mutations, both compiled:
- Dropping `without_one_terminator` from `one_line` reds
  `no_stderr_outlet_ends_in_a_visible_terminator`, and the long-line case
  as well.
- Replacing only the `StoppedRunning(Some(_))` arm with a bare `Escaped`
  reds `no_stderr_outlet_ends_in_a_visible_terminator` alone. That case's
  loop covers the arm.

Measured: `goad bad.toml` now ends ``expected `=`$`` under `cat -A`. The
visible `\n` is gone.

### F-12 — A closed standard output still answers a question with 0

**Severity:** minor
**Location:** `run`'s `Launch::Help` / `Launch::Version` arms
(`crates/goad/src/main.rs`). The docs that overstate this edge are
`StartupError::AnswerUnwritten`'s doc (`crates/goad/src/startup.rs`) and
SPEC-004 §7 R-1's row.

**Expected:** what the docs claim.
- `AnswerUnwritten`: *"A question whose answer **reached nobody** was not
  answered, so it is not an end *as asked*."*
- SPEC-004 §7 R-1: *"That `Ended::AsAsked` is earned — a question answered
  only if its **answer arrived** — is binary tier."*

**Observed:** `goad --help >&-` (standard output closed) exits **0** and
writes nothing to standard error. Measured at `3434b76`: `closed status=0`.
Rust's standard library treats a write to a closed standard stream (`EBADF`)
as a success. This is measured, not read from source here: no rust-src in
the devshell. So `try_line_to` answers `Ok(())`. The repair holds a write the
device *refused* (`/dev/full`, a broken pipe). It does not hold one that
*arrived nowhere*. The canon row and the variant's doc state the stronger
property.

**Evidence:** the measurement above. The one case,
`an_answer_that_cannot_be_written_exits_2`, uses `/dev/full`, which is the
edge the repair does reach. Whether to close this gap in code, or to narrow
both sentences to *a write that failed*, is a disposition.

**Disposition:** doc-wrong
**Response:** Verified by `strace`: before `main`, Rust's runtime polls fds 0–2, finds fd 1 `POLLNVAL`, and reopens it on `/dev/null`; the answer is written there, as `> /dev/null` would. Exit 0 is right. **User decision:** the `AnswerUnwritten` doc and SPEC-004 R-1's §7 row say *a write the stream refused*, and state the runtime's reopening once, as the reason a closed handle is not that. Endorsed canon edit.
**Outcome:** verified (round 3). Re-traced at `1c1fe9d` with `strace -e
trace=poll,openat,write goad --version >&-`. The trace shows `poll` on fds
0–2 answering `fd=1, revents=POLLNVAL`, then `openat("/dev/null", O_RDWR) =
1`, then `write(1, "0.1.0\n", 6) = 6`, and the process exits 0. `--version
> /dev/full` still writes the `AnswerUnwritten` line and exits 2. Both
sentences now say *refused*. `AnswerUnwritten`'s doc points to the R-1 row
and does not restate it. No test holds the closed-handle sentence (F-21).

### F-13 — `try_line_to`'s flush is unheld, and unreachable from its callers

**Severity:** minor
**Location:** `goad_shell::report::try_line_to`
(`crates/goad-shell/src/report.rs`).

**Expected:** a mechanism its doc gives a reason for is held by a case that
reds without it (`docs/memory/tests-asserting-proxies.md`).

**Observed:** the doc says *"Flushed before answering: a buffered sink can
report its failure only at the flush, and a flush left to the process's exit
reports it to nobody."* Mutation, compiled: `sink.flush()` replaced with
`Ok(())`. Then `cargo test -p goad-shell --lib report` passes 4/4 and
`cargo test -p goad --test binary` passes 8/8, including
`an_answer_that_cannot_be_written_exits_2`. Nothing reds. Both production
callers pass a sink the flush never matters for. One is `StdoutLock`, which
is line-buffered, and `writeln!`'s terminator already forces the write, so
`ENOSPC` surfaces from `writeln!` itself. The other is `StderrLock`, which
is unbuffered. The `Broken` test sink fails on `write`, so it cannot tell
the flush's presence from its absence.

**Evidence:** the mutation above, restored by byte copy, `git status` clean.
The flush may be right as defence against a future buffered caller. Nothing
holds it, and nothing reds on its removal.

**Disposition:** fix-now
**Response:** Keep the flush: `try_line_to` takes any `Write`, and its contract — whether the line arrived — is false without it for a buffered sink. Add a `report` unit case with a sink whose write succeeds and whose flush fails, asserting `try_line_to` answers the error.
**Outcome:** verified (round 3). `RefusesFlush` accepts every write and
fails the flush with `StorageFull`. Mutation, compiled: `sink.flush()`
replaced with `{ let _ = &mut sink; Ok(()) }` reds
`a_refused_flush_is_reported_to_a_caller_whose_line_is_the_answer` at its
`expect_err`, and the other four `report` cases pass. Restored by byte copy.

### F-14 — SPEC-004 §5's diagram still sends every question to 0

**Severity:** nit
**Location:** `docs/specs/004-process-exit-status.md` §5, the `stateDiagram`.

**Expected:** R-1 as the repair reads it: 0 for *a question, answered*, and
answered only if the answer was written (§7 R-1's row).

**Observed:** the edge `Invoked --> Answered: the invocation was a question`
makes *being a question* sufficient for `Answered` → 0. A question whose
answer was refused now exits 2. `NeverStarted`'s edge, *a step before the
event-loop call failed*, can be read to cover that, but only because the
reader already knows it does. The diagram was drawn before F-3's repair, and
the promotion did not revisit it.

**Evidence:** the diagram text against `an_answer_that_cannot_be_written_exits_2`.

**Disposition:** fix-now
**Response:** SPEC-004 §5's diagram gains the edge from a question whose answer could not be written to 2. Endorsed canon edit.
**Outcome:** verified (round 3). The diagram now splits the question:
`Invoked --> Question`, then `Question --> Answered: the answer was written`
and `Question --> NeverStarted: the stream refused the answer`. The syntax is
valid `stateDiagram-v2`, and `NeverStarted --> [*]: 2` already existed.
§5's prose under the diagram still describes the question without the
refusal (F-17).

### F-15 — *Every line on this surface goes through the pipeline* has one arm that does not

**Severity:** nit
**Location:** `diagnostics` module doc (*"the escape/bound pipeline every
line on this surface goes through"*); SPEC-004 §7 R-4's row (*"every line on
that surface goes through `diagnostics`' one escape-and-bound pipeline"*);
`report_exit_line`'s `Ok(Ended::StoppedRunning(None))` arm.

**Observed:** that arm answers
`"goad: the host was running and stopped, and no error was reported".to_owned()`
without `finish`. The output is the same, because a fixed ASCII literal
escapes and bounds to itself. But the universal claim, now written into
canon, is false as a statement about the code. F-2's Response said the
module doc *"becomes true rather than reworded"*.

**Evidence:** the arm's text in `report_exit_line`, against the two quoted
sentences.

**Disposition:** fix-now
**Response:** Carried by F-10's repair: the module doc and the canon sentence say which step each surface takes (escape and bound for the in-window surface; escape alone for stderr), and no longer claim *every line* for a fixed literal. Endorsed canon edit.
**Outcome:** verified (round 3). SPEC-004's R-4 row now reads *"Each line
of this report that **interpolates a value** takes `diagnostics`' one
escape"*, which is true: the `StoppedRunning(None)` literal interpolates
nothing. `report_startup_line`'s doc says *every **composed** line on
standard error*. The module doc says *every composed line on the in-window
surface goes through* `finish`, and *the host's own standard error takes
`one_line` instead*. Read alone, that second sentence would cover the
literal too. Read after the first, it means composed lines. This is left as
taste, not raised.
A grep of `crates/`, `nix/` and `docs/` for *escape/bound*,
*escape-and-bound*, *every line on this/that surface*, `LINE_LIMIT` and
`finish` finds no claim that a host stderr line is bounded. Closed slices'
records (001–009) and this ledger were excluded. One test name,
`the_line_goes_through_the_escape_and_bound_pipeline`, remains. It is about
`next_check_line`, which is in-window, so it is correct.

### Checked and found complete (round 2)

- **Worktree.** It was at `40caa4a`, 26 commits behind. `git merge
  --ff-only main` brought it to `3434b76` before any reading.
- **The gate.** `just check` exits 0 at `3434b76` before any mutation:
  **638 passed**, 0 failed, 0 ignored, over 31 `test result` lines. This
  matches the repair's reported total.
- **Mutations, all compiled, all restored by byte copy from the scratch
  directory, `diff` and `git status` clean after each.**
  - `--help` arm swallowed: reds `an_answer_that_cannot_be_written_exits_2`.
  - `--version` arm swallowed: reds `an_answer_that_cannot_be_written_exits_2`.
  - `finish` off `StoppedRunning(Some(_))`: reds
    `a_multi_line_platform_error_is_one_line_from_every_outlet`.
  - `try_line_to`'s flush removed: nothing reds (F-13).
- **`line_to` callers.** `goad-emit`'s `to_stdout` and `to_stderr`, and
  `goad`'s `report_exit` and `report_platform`. The added flush discards
  its result with the write's. It moves no byte or status on a
  line-buffered stdout or an unbuffered stderr. A grep of `crates/*/src`
  finds no stdout/stderr write outside `report`'s two functions, so *one
  way to write a line* holds: one implementation, with two policies over it.
- **`AnswerUnwritten`.** Its status is 2 through `exit::status`'s single
  `Err` arm, and it appears in `every_startup_failure_is_2`. Its `Display`
  is pinned by `display_text::answer_unwritten`, and `source()` is `None`
  (`source_walk`). The line goes through `finish`. The docs that enumerate
  where `StartupError`s come from are still true, and so are the docs that
  enumerate the edges into 0 or 2. Those are `StartupError`'s type doc, the
  `nix/module.nix` comment (0 names the tray quit, the window close and
  the answered question), SPEC-004 §6's instance list (named as instances)
  and §7 R-3's list of unreached causes (`Clock`, `Runtime`, `Platform`,
  `EventLoop`, `Enqueue`, which is complete against the enum). The one
  exception is §5's diagram (F-14).
- **F-2's escape against the binary tier.** No binary-tier case changed
  meaning. `too_many_arguments_…` and `no_argument_…` compare the stream
  with `report_startup_line` itself, so they track the escape. The prefix
  cases stop before any text the escape could touch. The scratch paths
  contain no `\` or control character.
- **Promotion against the delta, exactly.** Change 1's replacement
  sentence and Change 2's bullet appear verbatim in SPEC-003 once
  whitespace is normalised and `SPEC-00N` → `SPEC-004` is applied.
  Change 3's old phrase is gone and its new one is present. SPEC-004
  differs from the draft only in its header (Status `active`, the
  `SPEC-NNN` references), the removed `DRAFT-ONLY` comment, and the §7
  rows the repair amended.
- **Citations.** Every backticked `module::case` and case name in SPEC-004,
  and in SPEC-003's R-3 and R-4 rows, resolves to exactly one `fn`
  definition in `crates/`. The one exception is
  `help_prints_the_usage_block_on_stdout_and_exits_0`, which resolves twice
  because `goad-emit` has a namesake; SPEC-004 path-qualifies it, as round
  1 recorded. Neither spec has a `file:NN` line citation.
- **Residue.** `SPEC-NNN`, `SPEC-00N`, `draft-spec` and `DRAFT` appear
  nowhere outside `docs/slices/` except `docs/AGENTS.md`'s method text and
  `docs/follow-ups.md`'s generic `SPEC-NNN/R-N` form. `crates/` and
  `nix/` cite no draft.
- **Nix.** The directives are unchanged. The comment states policy per
  number and predicts nothing.

**Cross-check, round 2, written after the findings and outcomes above**
(`notes.md` §Handover *Repairs, round 1*, read only now).
- **Agree:** the F-2 and F-3 red-first reports match the mutations above.
  The 638 total matches. The notes also claim C-2's precondition held, and
  a citation re-check agrees.
- **Explains F-1's contest:** the class grep ran over `-- crates nix`
  only. Its pattern list includes *comes back* and *next try*, and run over
  the draft it would have hit §1.
- **Not in the notes:** F-9 (the notes record removing R-3's history
  clause, not §1's), F-10, F-11, F-12, F-14, F-15. **F-13's premise is in
  the notes, and read the other way:** the notes say the flush *"writes
  nothing further"* for `goad-emit`'s sinks. That is the same fact that
  makes it unheld for `goad`'s.
- **Seen there and not raised here:** `audit.md`'s AC-8 row still reads
  *pending — not applied*. That verdict belongs to audit.

### F-16 — SPEC-004 §1 now says the phase *is* a fact the process observes; §5 says *very nearly* and names where it is not

**Severity:** minor
**Location:** `docs/specs/004-process-exit-status.md` §1 *Intent*, second
paragraph, the sentence the repair wrote.

**Expected:** SPEC-004 §2 adopts SPEC-003 P-D for absolute clauses: *"a clause
here that says a mechanism always holds or never fails names its exception and
bounds it"*. §5 is where the exception is: *"Which phase the process ended in
is **very nearly** a fact it observes … The phase axis has one imprecision, it
is at the seam named below"*. *What the seam costs* then says that for a call
failing on entry the host reports *stopped running* and *"the host cannot know
it"*.

**Observed:** §1 now reads *"The status names **the phase the process ended
in**: how far the process got, which **is a fact it observes**, and not a
judgement about whether trying again would work."* The sentence makes the
absolute claim. It names no exception and points nowhere. That is the claim
§5 qualifies two sections later, for the one shape where the host does
*not* observe how far it got. The old paragraph had the same clause. The
repair rewrote the paragraph around the clause and kept it.

**Evidence:** the two quoted sentences side by side; §5 *What the seam costs*.
This is the first sentence a reader meets that states the rule. So it is
the one where the exception most needs to be in the sentence, or pointed to
from it.

**Disposition:** fix-now
**Response:** SPEC-004 §1 names the seam in the sentence: the phase is observed, except that a loop call failing on entry is reported as *stopped running* (§5 *What the seam costs*). Endorsed canon edit.
**Outcome:** verified (orchestrator site check at `cb7d2bc`, in place of a round 4 — `design-log.md`, *round 3: prose only*).

### F-17 — §5's prose still describes every question as answered

**Severity:** nit
**Location:** `docs/specs/004-process-exit-status.md` §5, *Both edges that
reach 0 are the same class.*

**Observed:** *"An invocation that is a question answers on standard output
and has nothing further to do; a running host that is asked to stop ends
because it was. Both did what was asked."* Since F-14, the diagram above
it has a question that did not answer: `Question --> NeverStarted: the
stream refused the answer`. The paragraph is about the edges into 0, so
context narrows it to the answered question. But the sentence itself states
the unconditional, which is exactly what F-14 raised about the diagram.

**Evidence:** the paragraph against the diagram directly above it, and
against `exit_codes::an_answer_that_cannot_be_written_exits_2`.

**Disposition:** fix-now
**Response:** §5's prose names the refused answer beside the answered question, as the diagram does. Endorsed canon edit.
**Outcome:** verified (orchestrator site check at `cb7d2bc`, in place of a round 4 — `design-log.md`, *round 3: prose only*).

### F-18 — SPEC-004 §2 says `goad-emit`'s statuses *have no owner today*; the same section says they are owned

**Severity:** nit
**Location:** `docs/specs/004-process-exit-status.md` §2 *Boundaries*, the
**SPEC-001 §2** bullet.

**Observed:** *"SPEC-001 §2 puts the `goad emit` command line out of its own
scope, which is why that binary's statuses **have no owner today** and why this
document claims the boundary."* The first bullet of the same list reads
*"`goad-emit` is **nominally owned** and not yet governed. §Owns is stated at
the wider boundary deliberately"*. OQ-2 also says *nominally owned*. Once
this document claims the boundary, the statuses have an owner, and the
*today* is the draft's tense. `07c71f3`'s message judged this *today* true
of the tree. Read against its own section, it is not.

**Evidence:** the two bullets; §8 OQ-2. This is F-9's class, *canon stating
the pre-promotion state as the present*, in the section after §1.

**Disposition:** fix-now
**Response:** §2 drops *today*: `goad-emit`'s statuses are nominally owned and not governed. Endorsed canon edit.
**Outcome:** verified (orchestrator site check at `cb7d2bc`, in place of a round 4 — `design-log.md`, *round 3: prose only*).

### F-19 — OQ-1 judges each cause's restart policy, the axis the document puts out of scope

**Severity:** minor
**Location:** `docs/specs/004-process-exit-status.md` §8, OQ-1.

**Expected:** §2: restart, retry and backoff **policy** *"is never asserted
here (§3 P-D)"*. §6 *What may not be inferred*: *"a display that was not
there at one moment may be there at the next"*. §6 *A stated consequence*: a
consumer that suppresses restarts on 2 suppresses both the display and the
configuration, and *"subdividing the class means deciding, per cause, whether
a retry could succeed"*.

**Observed:** OQ-1 closes the question with *"Not today (§6's stated
consequence): **the candidate causes either want suppression or are indifferent
to it**"*. That sentence decides, per cause, what a restart policy should do.
Take §6's own example cause, a display absent at startup. It does not *want*
suppression, since it may be there at the next attempt. It is *indifferent*
only under one supervisor's arrangement (the unit's session target restarts
it; `design.md` §5.4). §6 argues from the axis being rejected. OQ-1 argues
from a verdict on that axis.

**Evidence:** the quoted clause against §2's out-of-scope sentence and §6's
example. Brief item 6 asked for §8's *today*s to be judged. This is the
clause that *Not today* rests on. It was promoted unchanged in round 1, and
`07c71f3` left it as *true of the tree*. It is not a *today*, so the
repair's grep could not reach it.

**Disposition:** fix-now
**Response:** OQ-1 keeps only the reason that holds — subdividing reintroduces the per-cause judgement §5 rejects — and drops the per-cause restart verdict. Endorsed canon edit.
**Outcome:** verified (orchestrator site check at `cb7d2bc`, in place of a round 4 — `design-log.md`, *round 3: prose only*).

### F-20 — An unbounded stderr line is linear in the configuration line's length, and nothing says what that costs

**Severity:** nit
**Location:** `diagnostics::one_line` doc (`crates/goad/src/diagnostics.rs`);
SPEC-004 §7 R-4's row (*"and no bound"*).

**Observed:** measured at `1c1fe9d`, a configuration with one 100,000-character
line and an error at its end makes `goad` write **one 200,260-byte line**.
That is about twice the column: the source excerpt, then a caret line padded to
the column. The stream still holds one line, and it ends in the message, so
R-4 holds on the stream. The shipped supervisor reads it through journald,
and journald's `LineMax=` (48K by default) splits a longer line into several
records. The last record the journal shows for that run is then a fragment.
It begins partway through the caret padding and does not begin `goad: `.
`one_line`'s doc gives the reason for no bound and says nothing of what the
lack of one costs. The R-4 row says the same, and so does F-10's Response.
F-10's Response also cited *"journald's own limit is far above
`LINE_LIMIT`"*: true, but the relevant comparison is now against an
unbounded line.

**Evidence:** the measurement (`huge.toml`, `note = "a…a" x` with 100,000
`a`s, `wc -lc` on standard error: `1 200260`). Who controls the length of
each `{error}` that reaches a stderr outlet: the configuration file and
the paths in it (the person), OS error text, and Slint/winit text. The last
is reached through `StartupError::Platform`/`EventLoop`,
`Ended::StoppedRunning` and `report_platform`'s callers
(`SlintGlass::present`, and `install`'s `rescale` literal). **No backend- or
peer-authored text reaches any stderr outlet**, so the missing bound is not
a length a backend chose. D53's reason does not apply, and the user's
decision stands on its own terms. This finding is about the unstated cost,
not the decision.

**Disposition:** fix-now
**Response:** The decision stands; its cost is written down in `one_line`'s doc and R-4's §7 row: a line past journald's `LineMax` is split, and the last record then does not begin `goad: `. Endorsed canon edit.
**Outcome:** verified (orchestrator site check at `cb7d2bc`, in place of a round 4 — `design-log.md`, *round 3: prose only*).

### F-21 — R-1's row states the closed-descriptor behaviour in a *verified by* cell and names nothing that holds it

**Severity:** nit
**Location:** `docs/specs/004-process-exit-status.md` §7, R-1's row, *"A
closed standard output is not a refusal: before `main`, Rust's runtime …
reopens it on `/dev/null` … and the question exits 0."*

**Observed:** every other clause in the row names a case, or says it is
review or evidence and what that holds. This one states a property of the
standard library's startup, and through it an exit status, with no holder.
A binary-tier case could reach it: spawn `--help` with fd 1 closed, then
assert 0 and an empty standard error. It is not a clause *no cooperating
test can reach* (§2's rule for §7), so it is neither held nor declared
unheld. If a toolchain or target stops sanitising the descriptors, the
sentence becomes false, and nothing in the gate sees it happen.

**Evidence:** the row text; the `strace` in F-12's round-3 Outcome.

**Disposition:** fix-now
**Response:** R-1's row says the closed-stdout behaviour is the Rust runtime's, evidenced by the traced run, and held by no test — a test would pin the toolchain, not this code. Endorsed canon edit.
**Outcome:** verified (orchestrator site check at `cb7d2bc`, in place of a round 4 — `design-log.md`, *round 3: prose only*).

### F-22 — Two in-place substitutions left their lines unwrapped

**Severity:** nit
**Location:** `docs/specs/004-process-exit-status.md` §1, the line beginning
*"With the status stated, a supervisor — `nix/module.nix`'s systemd unit is
the one this"*, at 86 columns, where the surrounding prose wraps at 80;
`diagnostics::finish`'s doc, the line *"in-window surface passes through
this — decoding (step 2) happens only for stderr, before"*, about 95
columns, beside lines wrapped at 80.

**Evidence:** `awk 'length > 80'` over SPEC-004's prose (table rows
excluded). Three other lines are over 80, all pre-existing and all 87
columns or fewer: one each in §3, §8 and §9. The repair's diff shows both
lines as word substitutions that were not rewrapped. On its own this is
taste, and the pre-existing lines show the document is not strict about it.

**Disposition:** fix-now
**Response:** Rewrap both.
**Outcome:** verified (orchestrator site check at `cb7d2bc`, in place of a round 4 — `design-log.md`, *round 3: prose only*).

### F-23 — *stderr* now names two streams in `diagnostics`, and two docs beside the repair use it unqualified

**Severity:** nit
**Location:** `diagnostics` (`crates/goad/src/diagnostics.rs`): `STDERR_LIMIT`'s
doc, *"The stderr line's own bound: **the only line whose whole value is
diagnostic prose a person reads**"*; `finish`'s doc, *"decoding (step 2)
happens only for stderr"*.

**Observed:** both mean a **backend's** captured stderr, one line on the
in-window surface. The repair added *"the host's own standard error"* to the
module doc, `LINE_LIMIT`'s doc and `one_line`'s doc, the stream
`report_startup_line` writes to. Beside those, `STDERR_LIMIT`'s *the only
line whose whole value is diagnostic prose a person reads* now reads as
false. The host's own stderr lines are that too, and they are the unbounded
ones.

**Evidence:** the quoted docs against `one_line`'s doc and the module doc.

**Disposition:** fix-now
**Response:** `STDERR_LIMIT`'s doc says it bounds a captured backend's stderr, distinct from the host's own stderr lines.
**Outcome:** verified (orchestrator site check at `cb7d2bc`, in place of a round 4 — `design-log.md`, *round 3: prose only*).

### Checked and found complete (round 3)

- **Worktree.** It was at `5b51c90`. `git merge --ff-only main` brought it
  to `1c1fe9d` before any reading.
- **The gate.** `just check` exits 0 at `1c1fe9d` before any mutation:
  **642 passed**, 0 failed, 0 ignored, over 31 `test result` lines.
- **Mutations, all compiled, all restored by byte copy from the scratch
  directory, `git status` clean after each.** One attempt at the
  `StoppedRunning` arm did not compile (a misplaced `.to_string()`). It was
  discarded, and the recompiled version is the one recorded under F-11.
  - A bound restored in `one_line` reds the long-line case and the no-bound
    case.
  - Dropping the terminator step from `one_line` reds the terminator case
    and the long-line case.
  - `report_platform_line` alone on `finish` reds the no-bound case and the
    terminator case.
  - `report_startup_line` alone on `finish` reds all three new cases.
  - The `StoppedRunning(Some(_))` arm without the terminator step reds the
    terminator case.
  - `try_line_to`'s flush removed reds the new `report` case.
- **One escape.** `Escaped` is the only escape in `diagnostics`. `finish`
  and `one_line` compose it with `bound` and with `without_one_terminator`
  respectively. `without_one_terminator`'s own doc describes the capture use
  alone, and `one_line`'s doc states the reuse and the reason for it.
- **Citations.** The six case names the repair cited or added each resolve
  to exactly one `fn` in `crates/`: the three new `stderr_outlets` cases,
  `a_multi_line_platform_error_is_one_line_from_every_outlet`,
  `an_answer_that_cannot_be_written_exits_2` and
  `a_refused_flush_is_reported_to_a_caller_whose_line_is_the_answer`.
  SPEC-004 has no `file:NN` citation. The amended rows count nothing.
- **Scope.** The canon edits are the ones `design-log.md` *code review round
  2* endorsed: §1, §5's diagram, R-1's row and R-4's pipeline sentence.
  Nothing else in SPEC-004 moved.
- **FU-4, beyond the brief.** The old sentence, *"every diagnostic line in
  the same binary passes `finish(.., LINE_LIMIT)`"*, became false when the
  stderr outlets left `finish`. The new sentence, *"every composed line on
  the in-window diagnostic surface passes `finish`, which bounds it"*, is
  true: every `finish` caller is in-window. `tooltip` bounds lines that were
  already finished. The edit keeps a follow-up ledger row true, touches no
  canon, and the notes record it. `docs/slices/007/slice-007.md`'s copy of
  the sentence was left as a closed record, correctly.
- **SPEC-004 §2/§8 *today*s, judged** (Brief item 6). §2's *"What
  `goad-emit` does today is in its own source"*: true, and not a
  prediction. §2's *"no owner today"*: false (F-18). OQ-1's *"Not today"*:
  a decision, not a prediction, but the clause it rests on is F-19. OQ-2's
  *"ungoverned today"* and OQ-3's *"Nothing consumes such a thing today"*:
  true of the tree. No *today* reads a retry outcome off a status.

**Cross-check, round 3, written after the findings and outcomes above**
(`notes.md` §Handover *Repairs, round 2*, read only now).
- **Agree:** the red-first reports for the three `stderr_outlets` cases
  and the flush case match the mutations above. The 642 total matches. The
  FU-4 edit and the `slice-007.md` exclusion are recorded there.
- **Disagree:** the notes put §2's *"no owner today"* with the *today*s that
  are *true of the tree*. Against §2's own first bullet it is not (F-18).
- **Not in the notes:** F-16, F-17, F-19, F-20, F-21, F-22, F-23.

## Synthesis

Closed 2026-09-23 after three rounds and a site check. Round 1 found the
slice's two majors — retry predictions read off status 2 (F-1), and a final
stderr "line" that was several lines (F-2) — and a question that exited 0
unanswered (F-3). Round 2's repairs of those raised two code defects (F-10, the
bound cutting a parser's message; F-13, an untested flush) and moved F-1's class
into canon it had not been grepped over. Round 3 found prose only (F-16…F-23),
so the review closed on the orchestrator's site check rather than a fourth
round. Every finding `verified`; no blocker raised. Class lessons: a class grep
must cover canon as well as code; a bound suits lengths a peer chose, not a
line whose cause comes last.

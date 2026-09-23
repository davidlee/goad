# Design log — Slice 010

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### 2026-09-23 — how far into canon the exit-code repair goes

- **Asked:** narrow (repair the conflation in code, fix SPEC-003/R-4's stale
  verification sentence, drop `nix/module.nix`'s exception paragraph, leaving
  the numerals stated by no normative document) or broad (state the host's
  process-exit contract normatively, with the code and the module as its
  consumers) — and if broad, where it lives, given that SPEC-003 §6.3's
  "failure vocabulary" is the ingress *refusal reason* set and says nothing
  about process exit.
- **Decided:** "broad. a new spec I'd say"
- **Consequence:** the slice drafts new canon. `draft-spec.md` in the slice
  folder per `docs/AGENTS.md` §Canon that does not exist yet, numbered only at
  promotion. FU-1's stated cost — "SPEC-003's failure vocabulary" — was wrong
  about the home; the ledger row is corrected at close, not now.

### 2026-09-23 — what the new spec owns

- **Asked:** A (the host's exit statuses alone), B (how the host process ends
  and what it reports — statuses, the rule that classes are distinguished by
  status and not only by message, what a supervisor may infer, and the stderr
  line beside a failure), or D (B for both binaries, adding `goad-emit`'s
  0/1/2, which SPEC-001 §2 puts out of its own scope).
- **Recommended:** B, with `goad-emit` named in §2 Boundaries as a different
  contract — it reports a refusal *as an answer*, and a host that never started
  has no answer to give (005/D-4's cut).
- **Decided:** "the spec can (arguably should) nominally own more than is
  written in this slice. scope it as B, write it to accommodate D in future."
- **Consequence:** §Owns is stated at the wider boundary — the exit status of
  this project's binaries — while §4 writes requirements for the host only.
  §2 must say that plainly: `goad-emit` is **nominally owned and not yet
  governed**, so no reader takes silence for a rule. The principles in §3 are
  written to hold of both binaries, so adding `goad-emit`'s column later is an
  append rather than a restructure.

### 2026-09-23 — the axis is phase, not retryability

- **Asked:** does the status name the **phase** the process ended in (a fact it
  observes) or its **retryability** (a judgement per variant)? With the
  numerals that follow: 0 asked to stop, 2 never started, 1 stopped running.
- **Recommended:** phase. Retryability is wrong today for at least
  `Runtime(io::Error)` and `Ingress` in-use, both exit 2 while a retry might
  work; and keeping 2 where it is leaves `tests/binary/exit_codes.rs` true as
  written and makes `RestartPreventExitStatus=2` correct without changing the
  numeral.
- **Decided:** accepted, with the question raised of whether *never started,
  transient* deserves separating from *never started, permanent*.
- **Consequence:** three classes, cut on phase. The transient question is
  answered in the entry below and recorded as a non-goal with its reason, so a
  later reader does not re-derive it.

### 2026-09-23 — no transient/permanent split inside "never started"

- **Asked (by the user):** is it worth separating a startup failure that may
  clear from one that will not?
- **Answer:** the candidate set collapses on inspection. `Ingress` in-use
  *wants* suppression — a second host retrying against a socket a live host
  holds is a loop that should not run. `Runtime` and `Clock` are "the machine
  is broken", where a restart is harmless and pointless. The one case with
  measured evidence is the loop's end, and it is already its own class. A
  "transient" class would also reintroduce the per-variant judgement the phase
  axis was chosen to avoid, and a misfiled variant fails **silently** in both
  directions: a restart loop until systemd's limiter, or a host that stays down.
- **Decided:** not split. Recorded as a non-goal in `slice-010.md`.
- **Consequence:** the extension path is stated in the draft spec rather than
  left implicit — subdividing *never started* later is a spec amendment plus a
  second directive in `nix/module.nix`, which is cheap precisely because the
  module lives in this repository (006/OQ-1's own argument).

### 2026-09-23 — how the new class is held

- **Asked:** a pure seam (`run` returns an outcome value; `main`'s numeral
  becomes a pure function over it, tested exhaustively one tier down, with the
  real `PlatformError` constructed via `From<String>`), a headless compositor
  in the gate, or evidence at audit.
- **Recommended:** the pure seam plus audit evidence, with the single call site
  declared review-only in the draft spec's §7 — SPEC-003/R-3's and R-4's own
  position for clauses no cooperating test can reach. No new tooling, which
  `CLAUDE.md` §Environment requires be asked for anyway.
- **Decided:** agreed.
- **Consequence:** the seam is the slice's structural change, not a rider on
  it: `StartupError` stops carrying the loop's ending, and its doc comment —
  "every way `run` can fail **to reach the event loop**" — becomes true again.

### 2026-09-23 — the journal corrects the diagnosis

- **Not a decision; evidence read during scoping**, recorded here because it
  changes what the slice claims. Full extract in `research.md` §Thread 3.
- Six exits, not FU-1's four. All six suppressed by
  `RestartPreventExitStatus=2`; the four fast recoveries came from
  `Install.WantedBy = graphical-session.target`, not from systemd's restart
  logic.
- **In both two-hour outages the compositor never went away** — no session
  target cycle. Three `Io error: Broken pipe` lines precede each exit: goad's
  own Wayland connection broke while the display stayed up. So *a compositor
  going away*, the cause named by FU-1, the roadmap and `nix/module.nix`
  alike, is wrong for exactly the cases that did the damage — and the repair is
  better than advertised, converting a two-hour outage into a two-second one.

### 2026-09-23 — leanings on the two design-stage gates

Recorded while the design agent was running, and relayed to it. **Leanings,
not yet the gate** — confirmed or overturned when the design is presented.

- **OQ-6 — does the draft spec bind a supervisor?** Leaning **no**: the spec
  defines what each status means and leaves restart policy to the consumer.
  `nix/module.nix` is then a consumer that happens to live in this repository,
  not a party the spec commands. The design agent is told to stop arguing this
  one and design against it.
- **OQ-5 — how `goad-emit` appears.** Recorded as a named boundary, and
  **kept in view as a possible follow-up** rather than settled by silence: a
  row in `docs/follow-ups.md` at close, carrying its own kill condition, so
  the second binary's statuses being ungoverned is a tracked gap rather than
  an omission a reader has to notice.

### 2026-09-23 — the design gate: OQ-4 and the two scope deltas

- **Asked:** (1) OQ-4 — should a startup platform failure also be restartable?
  (2) a new production file `crates/goad/src/exit.rs`, one beyond
  `slice-010.md` §Scope. (3) `diagnostics::report_startup` — the impure outlet,
  not `report_startup_line` — removed and replaced by `report_exit`.
- **Recommended:** no; yes; yes.
- **Decided:** "1. yeah / 2. yup / 3. yep" — all three as recommended.
- **Consequence:** OQ-4 is closed *no*: a startup platform failure stays
  *never started*, and the consequence is stated in the draft spec §6 rather
  than left for a reader to rediscover. `exit.rs` joins §Scope. `report_startup`
  goes; `report_startup_line` keeps its name, signature and text, so AC-5 holds.
  Design stage complete; the design goes to adversarial review.

### 2026-09-23 — the three gates the design review would not take alone

- **Asked:** presented with round 1's dispositions (`review-design.md`), three
  sub-decisions the responder declined to take on its own. (1) **F-1** — rename
  `Ended::AskedToStop`, whose label is false of `--help`, to something true of
  both edges into 0? (2) **F-2** — add the missing binary-tier ingress case in
  this slice, or defer it? (3) **F-4** — add a gate case for
  `run_event_loop_until_quit`, and if so which: (a) a uniqueness scan, which
  does **not** catch §8 R2's regression because a re-filing keeps the call-site
  count at one, or (b) a scan for the re-filing spelling itself, which does.
- **Decided:** "1. yep / 2. defer / 3. b)".
- **Consequence:** the variant is `Ended::AsAsked`, the draft spec's §6 class is
  *as asked*, and the test names follow. The uncovered ingress case becomes a
  row in `slice-010.md` §Follow-ups with its own kill condition — one that also
  falls due on narrowing SPEC-003/R-3's *"same position"* analogy, since after
  F-2's repair R-4's exit is **uncovered** rather than unreachable.
  `structure::the_loop_s_ending_is_never_a_startup_failure` joins §Scope as a
  third test target, and `design.md` §8 R2 stops being a residue the gate does
  not hold: what is left is the line scan's own limit, stated in the case.


### 2026-09-23 — F-11, and a repair that reached for its own unobserved fact

- **Asked:** F-11's disposition — §6's status-1 row tells a supervisor the
  host's backend was *reachable*, which nothing in `start` ever asks. The
  responder proposed `doc-wrong`: drop the backend half and say what the process
  observed.
- **Decided:** "responder's disposition accepted, unless you see it
  differently".
- **Consequence:** accepted, and the repair is wider than the proposal in two
  places, both the class the finding names rather than new ground.

  The proposal's own replacement wording said the host's window *opened*. It
  was not adopted: `PromptWindow::new` **constructs** a window, and the crate's
  only `window.show()` is in `SlintGlass::present`, inside the loop and only for
  `Surface::Prompt | Surface::Diagnostics`. A tray-resident host sits at
  `Surface::Hidden` for hours and exits 1 having never shown one. The finding
  was right and its repair would have swapped one unobserved fact for another.

  The adjacent *what a reader may infer* cell said the host was *working*; same
  overclaim, one column over, since a host whose configured backend command does
  not exist runs and exits 1 identically. It says *running*, which is what the
  host's own stderr line already says (`goad: the host was running and
  stopped: {error}`), so the row and the line now agree.

  Round 1 is closed — F-1…F-11 all `verified`. Round 2 is fired against the
  repairs.

### 2026-09-23 — the design review, round 2: the seam and the instrument

- **Asked:** presented with round 2's eleven findings against round 1's repairs.
  (1) **F-12** — `run_event_loop_until_quit`'s `Err` does not establish that the
  loop began, so R-2, R-6 and §6's status-1 row assert an unobserved fact:
  narrow what 1 claims, or keep the sentence and bound the exception per P-D?
  (2) **F-16** — the new gate case misses a one-line, unaliased re-filing via
  `?` through a `From` impl: widen the stated limit, or close it in the case?
  (3) **F-13…F-22** — take the mechanical dispositions as a block?
- **Decided:** "1. agreed / 2. also agreed / 3. yes" — narrow; close it; block.
- **Consequence:** R-2 and R-3 now cut at **the call that runs the event loop**
  rather than at the loop having begun, which is the fact the process can
  observe; this is also F-14's blocker repair, since the old *at or before*
  wording had R-3 and R-2 admitting the same end. §5 gains *What the seam
  costs*, declaring that a loop failing on entry is reported as *stopped
  running* and the host cannot tell — with a normative *a consumer MUST NOT read
  1 as evidence that the host did any work*. AC-6 and the host's line text are
  unchanged: the common case is what they describe. A1 argues both directions
  and cites `set_event_loop_quit_on_last_window_closed(false)`; a new A4 carries
  the `Err` half.

  `the_loop_s_ending_is_never_a_startup_failure` now reds on two needles — the
  named spelling, and `?` applied directly to the call — each with its own
  compiled fixture. The two documented limits, multi-line and alias, stand.

  Round 2's own repairs are unreviewed. Round 3 is owed, against a narrower
  subject: the recut requirements, the new §5 paragraph, the instrument's
  changed specification, and §7's promotion gate.

### 2026-09-23 — the design review, round 3: the other conjunct, and the case that stops being deferred

- **Asked:** presented with round 3's twelve findings against round 2's repairs.
  (1) **F-23** — `run_event_loop_until_quit` answers `Err` for an end a person
  *asked* for (a `loop_error` latched in `about_to_wait`, which does not exit,
  then a user quit), so R-1's *if* half is violated: declare it as residue, or
  close it in code? (2) **F-26** — SPEC-003 §7's preamble forbids amending that
  spec to hold a no-test row for a clause the row itself calls reachable, which
  is what Change 1 now does: land the deferred ingress case now, amend the
  preamble, or drop Change 1? (3) **F-28** — the line scan leaks a third
  one-line spelling (`map_err(Into::into)?`): replace the needle set with a scan
  for the `From` impl every silent route needs? (4) the remaining nine as a
  block.
- **Decided:** "1. (2) / 2. land it now, (1) / 3. yes, let's / 4. yes".
- **Consequence, and two of these are design changes rather than repairs:**

  **F-23 closes in code.** `Cancel` already wraps a `watch::Sender<bool>` and
  holds its own receiver, so the fact *a stop was asked for* is existing state
  and needs no new channel. It gains a synchronous read; `start` keeps a clone
  the way it already keeps one of `pending`, and the loop call's `Err` arm
  answers `Ended::AsAsked` when a stop was asked for. The status stops declaring
  a residue and starts reporting an observed fact, which is the slice's thesis.
  `crates/goad/src/wire.rs` joins §Scope.

  **F-26 lands the ingress case.** The deferral recorded at round 1 was sound on
  its own terms and is overtaken: the R-4 cell repair the user endorsed cannot
  be applied to SPEC-003 without it. `crates/goad/tests/binary/exit_codes.rs`
  joins §Scope for an added case and an extended `scratch_config`; AC-5's
  *existing cases unmodified* is untouched by an addition. **It falls due on
  SPEC-003/R-3's cell in the same movement**, as `slice-010.md` §Follow-ups
  always said it would, so `canon-delta.md` gains Change 3 and the follow-up row
  is struck.

  **F-28 swaps the instrument** rather than widening it a third time: one needle
  for the explicit spelling, one for `impl From<…> for StartupError`, which is
  the precondition every silent route requires. The limit becomes a property of
  the matcher instead of a list maintained by hand.

  The design has changed materially since the user's approval of it, so it is
  re-presented before the plan (`docs/AGENTS.md` §Design).

### 2026-09-23 — the design review, round 4: the volition arm made holdable, and the scan re-cut

- **Asked:** presented with round 4's sixteen findings against round 3's
  repairs, plus **F-51**, raised by the responder while disposing (R-1's new
  MUST NOT, read with its pronoun, forbids the design's own `Ok` arm).
  (1) **F-37** — the case §9 names for the volition arm cannot observe it: lift
  the decision into a pure function in `exit.rs`, or declare it review?
  (2) **F-43 / F-44** — the scan's stated limit misses a variant import, an
  alias or a helper on the call's own line, and the `impl From` needle bans
  every conversion onto `StartupError` on a false premise: replace the named
  needle with a shape rule on the call's line, and keep or drop the `From`
  needle? (3) **F-46** — winit raises `CloseRequested` only for a message
  received (a compositor's close request, a client-side-decoration click, an
  X11 `WM_DELETE_WINDOW`), so a broken connection cannot trip `Cancel`, but a
  compositor can: should R-1 say *asked* rather than *a person asked*?
  (4) the rest, F-51 included, as a block.
- **Decided:** "pure fn in exit.rs / shape needle, drop From / say 'asked', not
  'a person' / yes, as a block".
- **Consequence:** `exit::ended(call, stop_requested) -> Ended` decides the
  volition arm, pure, asserted one tier down over one error value; `start`
  reads `Cancel::is_stopped` after the call has returned and passes it. What
  stays review is that `start` passes the real read.

  `the_loop_s_ending_is_never_a_startup_failure` becomes one rule: exactly one
  production line of `crates/goad/src` names `run_event_loop_until_quit`, and
  that line ends at the call. It also holds the call's uniqueness, which
  nothing in the gate held. The `impl From` needle is dropped, so
  `StartupError` carries no standing constraint. Its limit: a later statement
  that re-files the call's binding — `call?` included — is invisible.

  R-1 and every site that followed it say the host was **asked** to stop — by
  its quit control, or a close request its window received — and not that a
  person asked, which the host never observes.

  The design has changed again since its approval; the re-approval gate stays
  open, and round 5 is owed against these repairs.

### 2026-09-23 — F-52, the case name that still said the loop started

- **Asked:** F-52, raised by the responder in round 4. The case name
  `exit_status::a_platform_error_after_the_loop_started_is_1` asserts a fact
  F-12 established the process does not observe. Rename it to
  `exit_status::stopped_running_is_1`?
- **Decided:** "I'll take your recommendation."
- **Consequence:** renamed in `design.md` §9 and `draft-spec.md` §7 R-2's cell.
  Round 5 is to be run by a fresh agent.

### 2026-09-23 — the design review, round 5: every end decided on the request

- **Asked:** F-53. The event-loop call can answer `Ok` with no stop requested:
  Slint's winit backend assigns `loop_error` on each window event and calls
  `exit()` when one is set, and winit delivers the rest of the iteration first,
  so a later success clears the error and the loop ends `Ok`. `exit::ended`
  answers `AsAsked` for that, and the host exits 0, silently and unrestarted.
  Verified by reading the vendored source (orchestrator and raiser, apart).
  (a) Decide every end on whether a stop was requested; or (b) keep the `Ok`
  arm and declare the route beside `Closed`. Under (a), what does
  `StoppedRunning` carry for an `Ok` — a `PlatformError` built from a string,
  or `Option<PlatformError>`?
- **Decided:** "take route a) and the stronger typed honest #2 Option".
- **Consequence:** `exit::ended` reads the request alone; the call is read only
  for the error `StoppedRunning` carries, which is `None` when the call
  answered `Ok`. A1's *`Ok` means asked* is withdrawn, and `Ending::Closed`
  stops being R-1's residue: it now exits 1, which R-2, §5's diagram and §6 row
  1 already required (F-54). The host never builds an error the platform did
  not raise. F-51's Response priced (a) as a change to the value type, which
  was correct; it rejected it for a path no production code reached, which
  F-53 shows was false. The design has changed again, so round 6 is owed on
  this repair.

### 2026-09-23 — the design review, round 6: a case for the new shape, and no round 7

- **Asked:** round 6 found one major, a design defect (F-63): no case feeds
  `exit::status` `Ended::StoppedRunning(None)`, so a classifier answering 0 for
  it passes every named case and restores F-53's exit 0. Plus three minors and
  two nits, all prose. Proposed: add
  `exit_status::stopped_running_with_no_error_is_1`; extend the distinctness
  case to both stopped lines; the rest as the raiser routed; no round 7, the
  repairs closed by a site check.
- **Decided:** "yes, go ahead".
- **Consequence:** one case added and cited; R-1's normative sentence scoped to
  a host that reached the loop call (F-67). No design change. The review
  closes on the orchestrator's site check, and the design goes to the user for
  re-approval.

### 2026-09-23 — the design re-approved

- **Asked:** the design has changed since its approval (every end decided on
  the request, `StoppedRunning` carrying an `Option`, `Cancel::is_stopped`, the
  scan re-cut, R-1's *asked* and its scope, AC-11, Change 3, the ingress case
  and its struck follow-up). Approve it as it stands, before the plan?
- **Decided:** "yeah accept".
- **Consequence:** the slice moves to plan. `plan.md` is written by a fresh
  agent against `design.md` and `draft-spec.md` as committed.

### 2026-09-23 — P-1, raised at plan: the ingress case could not tell its failure from a display's

- **Asked:** the planner, verifying the design against the tree, measured that
  a configuration past the ingress step also exits 2 headlessly, at
  `PromptWindow::new` — so `exit_codes::an_unbindable_ingress_path_exits_2`,
  asserting the status alone, is green for any startup failure, and §9's
  bindable-path mutation is green headless and a hang on a machine with a
  display (`notes.md` Handover). (a) The case also asserts the ingress arm's
  stderr prefix; (b) the binary tier's spawn removes `WAYLAND_DISPLAY`,
  `WAYLAND_SOCKET` and `DISPLAY`, widening §Scope by `tests/binary/process.rs`
  and `tests/binary/main.rs`'s module doc. And: a seventh review round, or a
  site check?
- **Decided:** "take both" / "compare the sites".
- **Consequence:** `design.md` §5.2 gains the display-free spawn and §9 the
  prefix and a second mutation (`start` handing `listener` `None`);
  `draft-spec.md` §7's R-4 row says the case holds a prefix of standard error;
  `canon-delta.md` Change 1 says so too; `slice-010.md` §Scope names the two
  binary-tier files. Closed by the planner's site check, not a review round.

### 2026-09-23 — at audit: canon endorsed, AC-5 waived for doc comments, the nix retry claim reworded

- **Asked** (`audit.md` §Reconciliation): (1) endorse the canon rows C-1…C-6 as
  drafted — `draft-spec.md` to `docs/specs/004-process-exit-status.md`, its
  `DRAFT-ONLY` comment removed, `canon-delta.md` Changes 1–3 applied to
  SPEC-003 (1 and 3 together), and the `SPEC-004` citations in
  `nix/module.nix`, `exit.rs` and `StartupError`'s doc with the code repairs;
  (2) `help_prints_the_usage_block_on_stdout_and_exits_0`'s doc says
  *"`Ok(())` is exit 0"*, false since PHASE-02, and AC-5 forbids touching an
  existing case — waive or hold; (3) audit's A-1: the new nix comment's
  *"a restart changes nothing a person has not changed first"* is a
  retryability claim — reword as the unit's policy, or keep.
- **Decided:** endorse all; waive AC-5 for doc comments (it protects the cases'
  behaviour) and repair the sentence; reword as policy.
- **Consequence:** promotion proceeds; the AC-5 waiver is recorded in
  `audit.md`. (3) is the same class as `review-code.md` round 1 F-1, raised
  independently — the repair of F-1 carries it.

### 2026-09-23 — code review round 1: the stderr line escaped, and a question answered only when written

- **Asked** (`review-code.md` round 1): (1) **F-2** — the three stderr outlets
  interpolate `{error}` raw, so a multi-line `PlatformError` writes several
  lines and the last names neither binary nor phase. Route them through the
  module's existing `finish` (escape, bound); or keep the first line; or prefix
  every line. (2) **F-3** — `goad --help > /dev/full` exits 0: fix the code, or
  reword R-1's *answered*. (3) F-1, F-4…F-8 as fix-now.
- **Recommended:** `finish`; fix the code, `goad-emit` as a follow-up; yes.
- **Decided:** "F-2 - A"; F-3 fix-now + follow-up, after clarifying that plain
  `--help` still exits 0 — only a failed write exits 2, which is coreutils'
  convention (`ls --help > /dev/full` exits 1). The rest as recommended.
- **Consequence:** one pipeline for every line the host writes, and R-4's §7 row
  cites a multi-line case for *last line* instead of the single-literal proxy.
  A failed `--help`/`--version` write is a startup failure: a new
  `StartupError` variant, status 2 under R-3, and a binary-tier case on
  `/dev/full` in R-1's row. Both are amendments to the endorsed draft spec's §7,
  taken with these decisions. Accepted cost: a reader that closes the pipe
  before reading gets a broken-pipe line and 2. `goad-emit` shares the pattern
  through `line_to` and is a follow-up row.

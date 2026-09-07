# Design log — Slice 003

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### YYYY-MM-DD — <question in one line>

- **Asked:** <the question and the options presented>
- **Recommended:** <agent's recommendation, if any>
- **Decided:** <the user's answer, verbatim where it matters>
- **Consequence:** <what changes in the design; D-ref if it became a §7 decision>

### 2026-09-07 — Gate autonomy: the slice 002 grant, renewed for slice 003

- **Asked:** how much autonomy at the gates for slice 003 — the same grant as
  slice 002 (decide everything except canon); that grant extended to plan
  acceptance; or hold every gate as `docs/AGENTS.md` writes them.
- **Recommended:** the same grant as slice 002.
- **Decided:** **the same grant as slice 002 — decide everything except canon.**
- **Consequence:** a standing deviation from `docs/AGENTS.md`, recorded here
  because the methodology requires explicit user instruction to depart from
  the workflow and this is that instruction. For slice 003 only:
  - Design decisions that the methodology would put to the user one question
    at a time are taken by the agent and recorded in this log with the
    reasoning and the alternatives rejected, in the same shape as a decision
    the user took.
  - Adversarial-review findings are dispositioned by the agent without
    confirmation, and the disposition is recorded in the ledger.
  - **Reserved to the user:** plan acceptance; any change to canon
    (`docs/specs/`, `docs/policy/`, `docs/adr/`), including promotion of a
    draft or a delta at audit and the reopening of SPEC-001 OQ-3; and the
    close.
- **Also decided:** model mix — Opus for research, design, plan, adversarial
  review and audit; Sonnet for phase execution and verification, escalating
  to Opus on a failed phase. The orchestrator (this session) holds no stage
  itself; every stage is a fresh agent briefed from the slice folder.

### 2026-09-07 — OQ-1: does schedule state persist across restarts?

- **Asked:** whether this slice writes the resolved next check (or anything
  else) to disk, which is the decision `docs/roadmap.md` §Open decisions names
  as the one that either reopens SPEC-001 OQ-3 or leaves it shut.
- **Recommended:** nothing persists.
- **Decided:** **nothing persists.** SPEC-001 OQ-3 stays shut.
- **Consequence:** the host's state space stays one process lifetime wide
  (`crates/goad-shell/src/state.rs:3-4` is still true). A restarted host seeds a
  fresh resolved check through `schedule::resolve`'s `(None, None)` arm, exactly
  as it does today, and no `view_id` can outlive a process. No `canon-delta.md`
  entry: the decision is that canon stays as written. D-1 (§7).

### 2026-09-07 — OQ-2: is there a floor on how often the host evaluates?

- **Asked:** whether the host imposes a minimum interval between its own
  evaluations. Without one, a backend returning a past `next_check` on every
  response drives an exchange loop bounded only by how long a backend
  invocation takes (`research.md` Thread 4, H-1). With one, a backend asking for
  a very fast cadence is refused something the protocol admits.
- **Recommended:** a floor.
- **Decided:** **a fixed host-side floor, a few seconds.** The backend's
  instruction is untouched — R-28 stands, and the instant is stored and
  reported exactly as sent; only the *firing* is spaced. The design picks the
  number and states the rule precisely.
- **Consequence:** D-2 (§7) picks **3 seconds** and D-3 (§7) states the rule as
  a floor on *scheduled-after-scheduled* firings only. SPEC-001 needs a sentence
  saying that R-28 governs what is stored and not when the host fires, which is
  what `draft-spec.md` and `canon-delta.md` carry.

### 2026-09-07 — OQ-3: is a pending check observable to the person?

- **Asked:** nothing on the glass or in the tray mentions the schedule today.
  Options ran from nothing at all, through a line in the diagnostic surface, to
  a live countdown in the tray tooltip.
- **Recommended:** a line in the diagnostic surface.
- **Decided:** **one line in the diagnostic surface naming the next check. No
  countdown.**
- **Consequence:** `Frame` carries the instant, `diagnostics.rs` renders the
  sentence (every user-visible string in this renderer is in one file), and the
  tray tooltip is untouched. The line is **not** part of `Diagnostics`: that
  type is an exchange's product and carries a fault bit, and a standing
  schedule is neither. D-9 (§7).

### 2026-09-07 — OQ-4: is a scheduled evaluation distinguishable on the wire?

- **Asked:** `Stimulus` today is `Startup` and `Requested`, emitted as
  `event.kind` of `"startup"` and `"requested"` (`wire.rs:37-67`). Brief §8.1
  illustrates `"source": "scheduler", "kind": "poll"`; SPEC-001 §6.1 illustrates
  `"source": "timer", "kind": "scheduled"`; the code emits `"source": "host"`
  for both existing stimuli.
- **Recommended:** a third variant.
- **Decided:** **a scheduled stimulus is distinguishable on the wire — a third
  `Stimulus` variant.** The design chooses the strings and reconciles the two
  illustrations against the contract.
- **Consequence:** `Stimulus::Scheduled`, `kind` = `"scheduled"`, `source`
  unchanged at `"host"` — SPEC-001 §6.1's `"scheduled"` is adopted and its
  `"timer"` source is corrected to what the host emits; the brief's `"scheduler"`
  / `"poll"` is an illustration the spec supersedes. Because a backend can only
  branch on strings the contract fixes, the three kinds become a requirement
  (R-56). D-10 (§7), `canon-delta.md` CD-1 and CD-2.

### 2026-09-07 — OQ-5: suspend and large clock jumps *(agent decision under the standing grant)*

- **Asked:** a laptop closed for six hours wakes with a resolved instant long
  past. Is the mechanical answer — fire once, return to cadence — also the
  product answer, or should a wake be its own stimulus?
- **Alternatives rejected:** (b) a distinct wake stimulus, which needs a
  platform signal the host does not have and a fourth `event.kind` nothing can
  yet produce; (c) detecting a discontinuity between the wall clock and the
  monotonic deadline and re-resolving, which is a second resolution site and
  the drift F-48 was raised about; (d) arming the wait in bounded slices —
  never sleeping longer than some maximum and re-reading the wall clock at each
  slice — which anchors the wait to wall time at the cost of a clock read per
  slice and a `NoClock` path that recurs while a clock is broken.
- **Decided (agent):** **(a) — a resolved instant at or before `now` fires
  once and returns to cadence. No wake stimulus, no jump detection.** It is
  what `schedule::resolve` already guarantees (`schedule.rs:196-201`), it adds
  no state, and it keeps the one resolution site.
- **Consequence:** the wait is a monotonic deadline, and on Linux
  `CLOCK_MONOTONIC` does not advance across a suspend. So a host suspended for
  six hours with a thirty-minute wait pending does **not** fire on wake with a
  long-past instant; it fires thirty minutes of *awake* time after it was
  armed. The design states this as a known limitation (§5.5, R3) rather than
  pretending the wall-clock reading holds, and it is a named follow-up.
  Alternative (d) is the cheapest route to fixing it if the limitation proves
  to matter. D-8 (§7).

### 2026-09-07 — OQ-6: which stratum owns the wait? *(agent decision)*

- **Asked:** stratum 3 beside `serve`, where the loop and the clock already
  are, or stratum 2 beside `Host`, where the resolved instant lives. ADR-001
  permits either and forbids stratum 1.
- **Alternatives rejected:** stratum 2 — a `Host` that waits would own an async
  runtime dependency it does not have today, would need the cancellation signal
  passed into it, and would make `Host::evaluate` reentrant in a way R-31's
  single outstanding interaction has no answer for. A new stratum 2 module that
  only *computes* a wait is stratum 1's work misfiled.
- **Decided (agent):** **stratum 3 owns the waiting; stratum 1 owns the
  arithmetic.** The wait is a `select!` arm inside `serve`; the duration is
  `goad_semantics::schedule::wait_for`, a pure total function. Stratum 2 gains
  nothing at all — no accessor, no field, no method.
- **Consequence:** the slice's stratum 2 surface is empty, which the scope in
  `slice-003.md` is revised to say. D-4, D-5 (§7).

### 2026-09-07 — OQ-7: which timer facility? *(agent decision, gated on spike S-1)*

- **Asked:** `tokio::time`, already available with no manifest change, or
  `slint::Timer`, polled by the executor that already owns `serve` and
  advanced by the mock clock already in the tree. `research.md` Thread 5 named
  spike S-1 as the gate: nobody had shown a `tokio::time::sleep` completing
  when polled by Slint's executor under the `EnterGuard`.
- **Evidence:** S-1 was run (`research.md` Thread 5, *Spike S-1 result*;
  source preserved at `docs/slices/003/timer-probe.local.rs`). Four cases, one
  process, real headless Slint event loop, multi-thread runtime, guard held:
  a bare `sleep(300ms)` completed in 301.5 ms; a pinned `Sleep` in a `biased`
  `select!` against an idle receiver fired via the timer arm in 201.3 ms after
  `reset()`, and again in 150.4 ms after a second `reset()` on an
  already-fired sleep; a real `Host::evaluate` over the real process transport
  completed in 2.9 ms with no failure and no cleanup failure; an
  already-elapsed deadline completed in 2.1 µs. Total 656 ms.
- **Alternatives rejected:** `slint::Timer` — it does not run at all under
  `init_no_event_loop()`, which is the harness the entire cheap test tier uses
  (`wiring.rs:22`), so choosing it would put every scheduling assertion in the
  one-test-per-process event-loop tier. It is also a second time source in a
  program that has one, and the transport's own budgets are already tokio's.
- **Decided (agent):** **`tokio::time`.** A single `tokio::time::Sleep`, pinned
  and held across loop iterations, re-armed with `reset`.
- **Consequence:** no manifest change for production. D-6 (§7).

### 2026-09-07 — OQ-8: how is behaviour over real time checked? *(agent decision)*

- **Asked:** real short waits with polled predicates (slice 002's `until`,
  `wiring.rs:135-148`) against a mock clock (tokio `test-util`, or Slint's).
- **Alternatives rejected:** tokio's mock clock — `pause()`'s auto-advance
  jumps to the next timer whenever the runtime is idle, and every `serve` test
  awaits a real child process, so the transport's own `tokio::time::timeout`
  (`process.rs:106`) would fire immediately (`research.md` Thread 3). It also
  requires a current-thread runtime, so it is by construction not the
  production topology. Slint's mock clock advances Slint's timers and not
  tokio's, so it cannot advance the timer D-6 chose.
- **Decided (agent):** **three tiers, no mock clock.** (1) Stratum 1 unit tests
  for `wait_for` — every edge, including the past instant and the floor, at
  zero wall-clock cost. (2) The cheap renderer tier drives the production
  `serve` with real waits in the tens of milliseconds, made cheap by the floor
  rule D-3 chose: every assertion needs at most **one** unspaced scheduled
  firing. (3) One new `[[test]]` target for AC-10, under
  `init_integration_test_with_system_time()`.
- **Consequence:** AC-12's no-flakiness clause is held by making every timing
  assertion one-sided in the direction load pushes it — liveness bounded at
  2 s, and the anti-spin assertions counting invocations inside a window far
  shorter than the 3 s floor, where load can only *reduce* the count. D-7,
  D-11, D-12 (§7).

### 2026-09-07 — OQ-9: where does the initial wait come from? *(agent decision)*

- **Asked:** `Host` seeds a resolved check at construction and publishes no
  accessor (`host.rs:127-135`), so at process start stratum 3 knows no instant.
  Research named three options: add `Host::resolved_check()`; take the first
  `Outcome::next_check` from the startup evaluation `main.rs:91` already
  enqueues; or recompute `now + default_poll` in stratum 3.
- **Alternatives rejected:** an accessor on `Host` — it exists only to be read
  once, at a moment when the value is about to be superseded milliseconds
  later by the startup evaluation's own outcome, and it widens stratum 2's
  surface for nothing. Recomputing `now + default_poll` restates a rule
  stratum 1 states once, deliberately (`schedule.rs:209-213`).
- **Decided (agent):** **a fourth option: the timer starts armed at the
  floor, and the startup evaluation's outcome supersedes it before it can
  fire.** The loop is serial, so the initial arm can only elapse if the host is
  idle at the top of the loop for the whole floor — which happens only when the
  startup evaluation was never dispatched or was refused. In that case firing
  is the correct behaviour, not a defect.
- **Consequence:** the sleep is **always armed** — there is no disarmed state
  and no `Option` around it — which is what makes a broken clock at startup
  self-healing rather than a permanently silent host. `Host` is unchanged and
  `serve`'s signature is unchanged. D-5 (§7).

### 2026-09-07 — Canon shape: a delta on SPEC-001 and a draft SPEC-002 *(agent decision)*

- **Asked:** whether the timer's rules belong in SPEC-001 or in their own spec.
  SPEC-001 §2 puts *"the timer that decides when to evaluate"* out of scope and
  names the seam it abuts; `research.md` Thread 1 flags that if the timer's
  rules become normative, a new spec beats an edit to that scope line.
- **Alternatives rejected:** folding the floor and the fire-once rule into
  SPEC-001 — it would require reopening a scope sentence that is correct as
  written, and it would put host behaviour into a document whose subject is a
  wire contract a second implementation must satisfy.
- **Decided (agent):** **both, split by subject.** What crosses the wire is
  SPEC-001's and goes in `canon-delta.md`: the three event kinds become a
  requirement (R-56), and §6.1's illustration is corrected to the source the
  host actually emits. What the host does with an instant it already holds is a
  new document: `draft-spec.md`, *SPEC-002 — the host's scheduling behaviour*,
  covering no re-resolution, fire-once-on-past, and the floor.
- **Consequence:** two artefacts to promote at audit, both reserved to the
  user. Nothing outside this slice may cite either. D-13 (§7).

### 2026-09-07 — Acceptance criteria revised *(agent decision)*

- **Asked:** whether every AC in `slice-003.md` is discharged by a design
  section, per the design stage's own obligation.
- **Decided (agent):** three ACs are revised, and the rest stand.
  - **AC-4** said a past instant fires *"immediately"*. With D-3's floor, a past
    instant whose predecessor was itself a scheduled evaluation fires after the
    floor — which is the whole mechanism that stops the busy loop AC-4 is about.
    "Immediately" becomes "promptly", and the criterion names both cases.
  - **AC-5** claimed the timer would be observed *"on the default cadence once
    [the retained instant] has passed"*. That half is `schedule::resolve`'s
    behaviour, already held by stratum 2's own tests, and observing it through
    the timer costs a full floor interval of gate time for no additional
    evidence. AC-5 is narrowed to what the timer owns: a failing backend is
    re-invoked unprompted at the instant it last asked for, and never faster
    than the floor.
  - **AC-9** said a transient clock failure leaves *"the retained resolved
    instant"* surviving. The design retains the instant in the controller and
    the *deadline* in the loop, and the observable property is that neither is
    disturbed and the loop does not spin. AC-9 is restated in those terms.
- **Consequence:** `slice-003.md` is edited in place; this entry is the record
  of why.

### 2026-09-07 — Adversarial review round 1: D-3 revised, and three new decisions

- **Asked:** how to disposition `review-design.md` round 1's sixteen findings.
  Taken by the agent under the standing autonomy grant recorded above;
  `canon-delta.md` and `draft-spec.md` are drafts, so editing them is the
  agent's, and no canon document was touched. Dispositions and reasoning are in
  the ledger, one per finding.
- **Decided (F-2, and it supersedes the earlier shape of D-3):** the minimum
  spacing is anchored to **the previous scheduled firing**, held in the loop as
  one monotonic `tokio::time::Instant` (`floor_until`) that nothing clears.
  Rejected: (a) the earlier rule — space a scheduled firing only when its
  *predecessor* was itself scheduled, a `bool` — because its whole
  justification is that every other stimulus is a person, and this slice's own
  Non-goals hand slice 004 a stimulus that is not; a rule written into canon on
  a premise its own roadmap retires will be inherited unread. (b) spacing every
  evaluation, including a person's, which the user's OQ-2 answer rules out.
  (c) a *wall-clock* retained anchor, which is what the earlier draft rejected
  and rightly: it exposes the floor to a backwards clock jump. A monotonic
  anchor is the clock the deadline already uses, so (c)'s objection does not
  reach it.
- **Consequence:** the rule now makes no claim about what the other stimuli
  are; draft SPEC-002 R-5 says in the requirement itself that it bounds due-check
  firings only and that a host adding another stimulus must bound that stimulus
  separately. `design.md` §5.4, §7 D-3, `draft-spec.md` R-4/R-5, and
  `slice-003.md` §Non-goals all say this. It also closes F-6: the anchor has
  exactly one write site where the bit had two plausible ones.
- **Also decided (F-12), D-14:** `MINIMUM_SPACING` moves out of
  `goad-semantics` into `crates/goad/src/controller.rs`, and `wait_for` takes
  no floor argument. Rejected: keeping the constant in stratum 1 and passing
  it, which puts a host operational budget beside protocol arithmetic where
  none of ADR-001's four instruments can see it. D-3's anchor makes the move
  forced as well as tidy — the floor is now a `max` against a monotonic instant
  whose type stratum 1 cannot name.
- **Also decided (F-4), D-15:** `Controller::absorb` returns
  `Absorbed { shift, next_check }`. Rejected: `.expect` at the re-arm site,
  which `expect_used = "deny"` refuses outside tests and POL-001 forbids
  suppressing; and an `Option`-returning accessor, which only moves the unwrap.
  `Outcome::next_check` is concrete on every outcome, so the totality is real.
- **Also decided (F-3), D-16:** AC-6 becomes two instruments — the *identifier*
  `resolve` absent from `crates/goad/src` code, and the path `schedule::resolve`
  occurring exactly twice in `crates/goad-shell`'s production code, both in
  `host.rs` — neither pinned to a line number. Rejected: one grep for the path
  across the tree, which this slice's own first import from
  `goad_semantics::schedule` makes defeatable by brace-grouping `resolve` into
  it.

### 2026-09-07 — Adversarial review round 2: D-16 refined, two test shapes settled

- **Asked:** how to disposition `review-design.md` round 2's F-17 to F-20.
  Taken by the agent under the standing grant; all four accepted `fix-now`,
  none rejected. No canon document touched.
- **Decided (F-17), D-16 refined:** both AC-6 instruments are built on
  `structure.rs`'s production-code walk — cut at each file's own
  `#[cfg(test)]`, comments stripped — rather than on `goad_boundary::scan::Scan`,
  which reads every line of every file and has no such cutoff. Rejected:
  renaming `wire.rs`'s two test functions, which would make the gate green
  while leaving an instrument that forbids an ordinary English word in test
  code. Round 1's stated counts were wrong; both are now re-run and written
  into `design.md` where the claim is made — 0 production occurrences of
  `resolve` over 12 files under `crates/goad/src`, and 2 of `schedule::resolve`
  over 8 files under `crates/goad-shell/src`, at `host.rs:128` and `:259`.
- **Decided (F-18):** AC-9's fixture is a clock that **succeeds once and then
  fails**. Rejected: an always-failing clock, which never completes an exchange
  and so never arms a deadline the timer arm can win; and widening the window
  past the three-second spacing, which would cost more than the whole
  unavoidable test budget F-7's repair accounts for.
- **Consequence:** `design.md` §2, §5.5 I-1/I-1a, §7 D-16 and §9 (the AC-3,
  AC-6, AC-7 and AC-9 rows, and the margin table) restate the instruments and
  the tests; `slice-003.md` AC-6 and AC-9 match. F-19 and F-20 are corrections
  inside §9 alone: AC-3's row now describes the test the margin table describes,
  its margin is 3x rather than 200x, and AC-7's timed assertion is in the table.

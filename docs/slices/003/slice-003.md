# Slice 003: Scheduling — the timer that turns a resolved instant into an evaluation

**Stage:** design
**Depends on:** slice 001 (closed) — SPEC-001, `schedule::resolve`, `Host` and
the resolved next check. Slice 002 (closed) — the renderer, the `serve` loop,
the wall clock, and the one observable surface a scheduled evaluation can show
its work on.
**Research:** `research.md` — five threads, and spike S-1 run and recorded
(Thread 5, *Spike S-1 result*).
**Decisions:** `design-log.md`. Design: `design.md`. Canon: `canon-delta.md`
(SPEC-001) and `draft-spec.md` (SPEC-002, the host's scheduling behaviour).

## Purpose

goad keeps time on paper and not in fact. Every exchange resolves a next check
and stores it; nothing ever waits for one. The host asks the backend a question
when the process starts and when a person picks *Check now* from the tray, and
at no other moment. A backend that says *ask me again in forty-five minutes* is
answered with silence.

This slice makes the resolved instant do something. When it arrives, the host
evaluates. When a backend supplies a new one — from an evaluation or from a
response — the wait changes to match, in both directions: sooner as readily as
later. When no backend supplies one, the configured default poll is the cadence.
When the backend fails, the host keeps asking on the cadence it already had,
rather than falling silent or spinning.

Once this lands, brief §21's AC-3 has an observable for the first time, and AC-8
— *a later valid `next_check` supersedes an earlier one* — stops being a
property of a pure function and becomes a property of a running program.

## Scope

Surfaces this slice may touch.

**The wait itself:**

- `crates/goad/src/controller.rs` — `serve`'s loop gains a third `select!` arm
  that wakes on time, a monotonic `floor_until` anchor, and the
  `MINIMUM_SPACING` constant; `Controller` retains the next check and `Frame`
  carries it. `Controller::absorb` currently destructures `Received::next_check`
  into `_` (`:140-143`); that seam is the one this slice fills, and `absorb`
  now returns `Absorbed { shift, next_check }` so the loop's read is total
  (design D-15). `Pending` gains a `now()` accessor beside `exchanged()`.
- **No new module.** The wait is a third `select!` arm inside `serve`, holding
  a pinned `tokio::time::Sleep` — design D-4, D-6.
- `crates/goad-semantics/src/schedule.rs` — gains `wait_for`, the pure function
  answering *how long from this instant to that one*, floored at zero. It takes
  no minimum spacing: the floor is a host operational budget, so
  `MINIMUM_SPACING` is a constant in `crates/goad/src/controller.rs` beside the
  loop that applies it (design D-14). No change to the *behaviour* of
  `resolve`, `parse` or `parse_span`; one would be a design change, not a
  refactor. One documentation correction: `resolve`'s doc comment
  (`schedule.rs:198-201`) states the one-off past instant as though it were the
  general case, which is review finding F-1 written into the tree.

**The plumbing:**

- `crates/goad/src/reception.rs` — **unchanged.** `Received::next_check` already
  exists and is already carried, and `receive` stays the one place an `Outcome`
  is destructured (slice 002 D16). `Controller::absorb` stops discarding the
  field and retains it (design §5.3).
- `crates/goad-shell/src/host.rs` — **unchanged.** The design takes a fourth
  route to the initial wait: the timer starts armed at the floor and the startup
  evaluation's outcome supersedes it (D-5). **Stratum 2 gains nothing at all.**
- `crates/goad/src/main.rs` — **unchanged.** `serve`'s signature does not
  change; the sleep and the floor are internal to the loop.
- `crates/goad/src/wire.rs` — `Stimulus` gains `Scheduled`, `kind` =
  `"scheduled"` (D-10). `Command` gains nothing; see Non-goals.
- `crates/goad/src/diagnostics.rs` — one new pure function rendering the
  next-check line, and `crates/goad/src/glass.rs` appends it to the diagnostic
  model it already builds (D-9).

**Configuration:**

- `crates/goad-shell/src/config.rs` — **documentation only.** The floor is a
  host constant, not configuration (design D-2), so no second configured
  duration lands here and `ScheduleConfig::default_poll` is read exactly as it
  is today. One doc comment on `default_poll` records that a value below the
  floor is accepted and honoured for the process's first scheduled firing, then
  floored — which is where risk R2's mitigation lives.

**Tests:**

- `crates/goad/tests/renderer/` — the cheap tier, where scheduled behaviour over
  simulated or short real time is asserted.
- **A new `[[test]]` target** for AC-10 — `crates/goad/tests/event_loop_schedule/`,
  its own process because the Slint testing backend initialises once per
  process, which is why `tests/event_loop/` already holds exactly one test
  (D-12). `crates/goad/tests/event_loop/` itself is unchanged.
- `tests/support/driving.rs` and `tests/backends/` — helpers and scripted
  backends extended, not duplicated.
- `crates/goad-boundary/tests/checks/` — AC-6's two instruments, which is where
  every scan in the workspace lives (ADR-003 §Decision). No change to
  `crates/goad-boundary/src/`: both are configurations of machinery that
  already exists.
- `crates/goad/Cargo.toml` — the new `[[test]]` target only. **No new feature
  and no new dependency:** the design takes no mock clock (D-7), so tokio's
  `test-util` is not added, and POL-001's feature-unification residue is
  untouched in both directions.

## Non-goals

- **Re-resolving the schedule.** `schedule::resolve` has exactly two call
  sites, both inside `Host`: the seed in `Host::new` (`host.rs:128`) and
  `Host::resolve_from` (`host.rs:259`), the latter reached from the accept path
  and the failure path, both of which write what they report. This slice adds
  no third. The timer consumes
  `Outcome::next_check` and computes nothing of its own beyond the wait
  duration. A second resolution would produce an instant the host never stored,
  which is the drift slice 001's F-48 was raised about.
- **Adjusting a backend's instruction.** SPEC-001 R-28 requires a past
  `next_check` be stored as given. The host does not clamp it forward, does not
  round it, and does not substitute a value it prefers. Whether the host may
  impose a floor on its **own** evaluation rate is a separate question and is
  OQ-2 — a floor on the host's cadence is not an adjustment to the backend's
  stored instruction.
- **Retry.** A failed exchange is not retried sooner than the schedule says.
  `host.rs:89-91` gives the reason: the host cannot know what a failed exchange
  already did, so a retry may repeat a side effect. The cadence is the retry.
- **The timer as a queued command.** The command channel has capacity 1 and its
  producers use `try_send`, reporting a full channel as a UI notice
  (`wire.rs:122-129`, `main.rs:72`). A person losing a duplicate click and being
  told so is acceptable; a scheduled check evaporating because a click was in
  flight is a defect with no symptom. The wait belongs beside the channel, not
  inside it.
- **Event ingress** (slice 004) and **the socket transport** (slice 005). A
  scheduled evaluation and an external event are two stimuli into one path; this
  slice builds the path, and 004 adds the second stimulus. The minimum spacing
  this slice introduces bounds the host's own due-check firings and nothing
  else, and it cannot be cleared by another stimulus — but it does **not**
  bound evaluation driven by an ingested event. How that stimulus is bounded is
  slice 004's question, and draft SPEC-002 R-5 says so in the rule itself
  rather than leaving it to be inherited unread.
- **Catch-up.** If the host was not running when a check was due, it does not
  evaluate once per missed interval on waking. One check is due, and one
  evaluation discharges it.
- **Wall-clock policy beyond the resolved instant.** Calendar-aware scheduling,
  time zones, and "tomorrow morning" are SPEC-001 R-23's refusals and brief
  §9.1's deferred forms. Nothing here parses a new schedule form.

## Acceptance criteria

- [ ] AC-1 — **The host evaluates without being asked.** With a backend that
      supplies no `next_check`, a running host performs a second `evaluate`
      approximately one configured `default_poll` after the first, and the
      backend's own invocation record is what shows it. Discharges brief §21
      AC-3.
- [ ] AC-2 — **`next_check` from either direction changes the wait.** An
      instruction returned by an `evaluate`, and an instruction returned by a
      `respond`, each move the next scheduled evaluation to the instructed
      instant. Discharges the observable half of brief §21 AC-7.
- [ ] AC-3 — **A later valid instruction supersedes an earlier one, in both
      directions, observably.** A second instruction that is *earlier* than the
      pending one shortens the wait; a second instruction that is *later*
      lengthens it. The earlier-wins case is the one a naive implementation gets
      wrong, because latest-valid-wins is issue order and not `max`
      (`schedule.rs:204-208`). Discharges brief §21 AC-8's "observable over
      time" half.
- [ ] AC-4 — **A resolved instant at or before `now` fires as soon as the floor
      permits, and each such instant fires at most once.** A backend-supplied
      past instruction is stored as given (R-28), so the timer must handle a
      non-positive wait without underflow and without spinning. Firing never
      re-fires an instant on account of its having elapsed. What follows
      depends on the backend, and both cases are asserted: if it stops
      instructing the past, the next resolution consumes the elapsed value and
      cadence resumes (`schedule.rs:196-201`); if it instructs the past on
      **every** response, `resolve`'s first arm returns the instruction
      verbatim and the retained value is replaced rather than consumed, so
      cadence never resumes and the minimum spacing is the only bound. Asserted
      as a bounded number of exchanges over a bounded interval, not as a timing
      measurement.
- [ ] AC-5 — **A failing backend is polled on its existing cadence, and no
      faster.** A backend that fails every invocation is invoked again,
      unprompted, at the instant it last asked for while that is still ahead,
      and never faster than the minimum spacing. Asserted by counting invocations over a
      bounded interval, so that a busy-loop fails the assertion rather than
      merely being slower. *Revised at design:* the original also claimed the
      transition to the default cadence once the instant has passed. That is
      `schedule::resolve`'s behaviour, already held by stratum 2's own tests,
      and observing it through the timer costs a whole floor interval of gate
      time for no new evidence. This is slice 001's F-1/F-34/F-48 carried
      forward, and SPEC-001 R-29 and §5's *"A broken backend is polled on its
      existing cadence"*.
- [ ] AC-6 — **The timer never resolves a schedule.** Structural, and held by
      two scans in `crates/goad-boundary`, both reading **production code
      only** — cut at each file's own `#[cfg(test)]`, comments stripped — and
      neither pinned to a line number (design D-16): **(a)** the identifier
      `resolve` appears in no production line under `crates/goad/src` — the
      identifier, not the path, so that a brace-grouped `use` cannot hide a
      call; and **(b)** the path `schedule::resolve` occurs exactly twice in
      `crates/goad-shell`'s production code, both in `host.rs`. Both counts
      are measured against the tree, not assumed: 0 over 12 files and 2 over 8
      respectively. The timer's input is
      `Outcome::next_check` alone. In the manner of slice 001's
      `transport_shape.rs` and slice 002's boundary scans, rather than by
      review alone; each instrument's stated residue is in design §9.
- [ ] AC-7 — **A timer does not defeat cancellation.** A stop request while the
      host is waiting for a scheduled check ends `serve` promptly, and a stop
      request while a scheduled exchange is in flight drops that exchange, as
      slice 002 AC-12 already requires. The renderer still leaves behind no task
      or handle a drop would fail to cancel (SPEC-001 R-48).
- [ ] AC-8 — **A backend failure does not stop the clock.** After any failure in
      SPEC-001's taxonomy, the host is still scheduled and still invokes the
      backend again unprompted. SPEC-001 R-45, and the failure mode SPEC-001 §5
      calls *"the failure mode the user notices last"*.
- [ ] AC-9 — **A clock that cannot be read does not lose the schedule, and does
      not spin.** A `ClockError` inside the loop is reported as a refusal
      (`controller.rs:249-253`); the retained instant is untouched; a pending
      future deadline is untouched; and a clock that fails on a *scheduled*
      firing produces one report per minimum spacing rather than a loop. The
      scheduled half needs a clock that **succeeds once and then fails**: an
      always-failing clock never completes an exchange, so nothing arms a
      deadline the timer arm could win (design §9).
      *Revised at design:* the design splits the instant (retained by the
      controller) from the deadline (held by the loop), so the criterion names
      both.
- [ ] AC-10 — **The waiting mechanism is proved in the arrangement it will
      actually run in, save for one component that no headless test can
      reach.** Slice 002 left this as its first follow-up (F-5): no test drives
      an exchange, or any timer, through the production topology of Slint's
      executor polling under a tokio `EnterGuard`. This slice is the first that
      cannot argue around it, because the timer *is* that mechanism. The
      multi-thread runtime, the `EnterGuard`, `spawn_local`, a real window and
      tray, the real transport and production `serve` are all production's; the
      Slint **platform** is the testing backend's, because there is no headless
      way to install the production one. The criterion is met when every other
      component is production's and the substitution is stated (design §5.5
      A-1).
- [ ] AC-11 — **No domain vocabulary** appears in any crate name, module name,
      type, markup component, accessible label, or user-visible string. The
      standing criterion — brief §21 AC-16, `CLAUDE.md` invariant 1 — held by
      the vocabulary scan in `crates/goad-boundary`.
- [ ] AC-12 — **`just check` exits 0**, six commands, no command weakened,
      conditioned or `#[ignore]`d, and no test whose passing depends on machine
      load. POL-001 §Compliance. A timing-sensitive test that can be flaky under
      a loaded gate is a design defect, not a tolerated cost.

## Governing canon

**Binding:**

- **SPEC-001** — the host/backend interaction protocol. §2 puts the timer out of
  scope and names the seam it abuts: *"it consumes [a resolved next-check
  instant] and calls `evaluate`."* R-26 (how the instant was resolved), R-27
  (always concrete, never absent), R-28 (a past instruction is stored as given),
  R-29 (a failure accepts no instruction), R-21 (the host's one duration
  grammar), R-45 (a failure leaves the host invocable) and R-48 (cancellation)
  all constrain this slice at one remove. §5's *"A broken backend is polled on
  its existing cadence"* states the intended behaviour in prose already.
- **ADR-001** — one-way strata. A timer is I/O against real time. Stratum 1
  stays pure; `schedule::resolve` keeps taking `now` as a parameter.
- **ADR-003** — the workspace of strata. The four ADR-001 instruments and the
  vocabulary scan apply unchanged; a new member is not anticipated.
- **POL-001** — the phase gate. Six commands, and the review obligation about
  feature unification into stratum 1's build.

**Checked, not applicable:**

- **ADR-002** — superseded by ADR-003. Its T2 trigger (a second binary) is slice
  004's, not this slice's.

## Open questions

**All nine are closed.** OQ-1 to OQ-4 by the user, OQ-5 to OQ-9 by the agent
under the standing autonomy grant. Each answer is recorded in `design-log.md`
with its alternatives, and appears as a decision D-N in `design.md` §7.

- ~~**OQ-1 — Does schedule state persist across process restarts?**~~
  **Answered (user): nothing persists.** SPEC-001 OQ-3 stays shut. D-1.
- ~~**OQ-2 — Is there a floor on how often the host will evaluate?**~~
  **Answered (user): yes, a fixed host-side floor of a few seconds; the
  backend's instruction is untouched and only the firing is spaced.** The design
  picks **3 seconds** (D-2) and spaces **a scheduled firing from the previous
  scheduled firing**, anchored to a monotonic instant that nothing clears
  (D-3), so a person's own actions are never delayed and no other stimulus can
  clear the floor.
- ~~**OQ-3 — Is a pending check observable to the person?**~~
  **Answered (user): one line in the diagnostic surface naming the next check.
  No countdown, and the tray tooltip is untouched.** D-9.
- ~~**OQ-4 — Is a scheduled evaluation distinguishable from a requested one on
  the wire?**~~ **Answered (user): yes — a third `Stimulus` variant.** The
  design chooses `kind` = `"scheduled"`, adopting SPEC-001 §6.1's own string,
  with `source` unchanged at `"host"`; the three kinds become normative as
  R-56. D-10, `canon-delta.md` CD-1 and CD-2.
- ~~**OQ-5 — What happens across a suspend or a large clock jump?**~~
  **Answered (agent): a resolved instant at or before `now` fires once and
  returns to cadence. No wake stimulus, no jump detection.** The wait is
  measured monotonically, so a suspend does not consume it — stated as a known
  limitation, not hidden. D-8, design §5.5 E-4, risk R3.
- ~~**OQ-6 — Which stratum owns the wait?**~~ **Answered (agent): stratum 3
  owns the waiting, stratum 1 owns the arithmetic, stratum 2 gains nothing.**
  D-4.
- ~~**OQ-7 — Which timer facility?**~~ **Answered (agent): `tokio::time`.**
  It is already available with no manifest change and is what the transport's
  own budgets use, whereas `slint::Timer` does not run under
  `init_no_event_loop()`, the harness the cheap tier uses, and would be a
  second time source. Spike S-1 gated this and was run on 2026-09-07: a
  `tokio::time::Sleep` polled by Slint's executor under the `EnterGuard`
  completes on time, and a pinned `Sleep` can be `reset` repeatedly.
  `research.md` Thread 5 *Spike S-1 result*; D-6.
- ~~**OQ-8 — How is "observable over real time" checked without wall-clock
  sleeps?**~~ **Answered (agent): no mock clock.** Exhaustive stratum 1 unit
  tests for `wait_for` at zero cost; real waits in the tens of milliseconds in
  the cheap tier, kept cheap by D-3's floor rule; and one new event-loop target
  for AC-10. D-7, D-11, D-12.
- ~~**OQ-9 — Where does the initial wait come from?**~~ **Answered (agent): the
  sleep is always armed, initially at the floor, and the startup evaluation's
  outcome supersedes it before it can fire.** No accessor on `Host`, no
  recomputation of `now + default_poll`. D-5.

## Summary

<!-- Written at close. -->

## Follow-ups

<!-- Written at close. -->

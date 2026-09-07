# Notes — Slice 003

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — The arithmetic, and the third stimulus | done | 2026-09-07 |
| PHASE-02 — The wait, and the cadence it keeps | done | 2026-09-07 |
| PHASE-03 — What the floor bounds, and what a failure does not stop | done | 2026-09-07 |
| PHASE-04 — What the person sees, and what the scan holds | done | 2026-09-07 |
| PHASE-05 — The topology | done | 2026-09-07 |
| PHASE-06 — Restatement, re-measurement, and the gate | done | 2026-09-07 |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — The arithmetic, and the third stimulus

**Objective:** stratum 1 can answer *how long from this instant to that one*,
totally and at zero wall cost, and stratum 3 can name a scheduled evaluation on
the wire. Nothing waits yet.

**Reading list**
- `plan.md:397-486` — the PHASE-01 entry (binding: EN-1, EX-1..EX-5, VT-1..VT-3,
  VA-1..VA-2, S-1..S-2, implementer notes).
- `design.md:471-506` (§5.5 E-1) — the two successor cases for a past instant,
  which EX-3's doc-comment repair must state.
- `design.md:151-292` (§5.2) — the exact `wait_for` body and doc comment to
  land, and the `Stimulus` enum shape (`Startup, Requested, Scheduled`), the
  wire form, and CD-1/CD-2 (source stays `"host"`, kind `"scheduled"`; SPEC-001
  §6.1's `"timer"` is not adopted here).
- `draft-spec.md:73-89` (§4 R-3, R-6) — the requirements EX-1..EX-3 discharge.
- `docs/policy/001-the-phase-gate.md` — the six-command gate, "never `allow`",
  the four ADR-001 instruments + vocabulary scan + residue.
- `docs/adr/001-one-way-strata.md` — stratum 1 purity (no I/O, no async, no
  reach into stratum 2/3).
- `docs/memory/cite-requirements-not-finding-ids.md` — EX-3's doc comment cites
  `SPEC-001/R-26`/`R-28`, not `F-1`.
- `docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` — bears on
  S-1 if `wait_for` needs a suppression (it does not, per the design body).
- `crates/goad-semantics/src/schedule.rs` (whole, 338 lines) — `resolve`'s doc
  comment at `:198-201` to amend; existing `#[cfg(test)] mod tests` helpers
  `instant()`/`now()` at `:244-250` to reuse.
- `crates/goad/src/wire.rs` (whole, 226 lines) — `Stimulus` at `:37-67`;
  `Command`/`Wire`/`Cancel` are out of scope (untouched); existing test names
  at `:208`/`:215` (must not be renamed, per implementer note).

**Assumptions & STOP conditions**
- `jiff::Timestamp::duration_since` and `TryFrom<SignedDuration> for
  std::time::Duration` behave exactly as the design's implementer notes state
  (jiff 0.2.35). Verified by VT-1 rather than trusted blind.
- S-1 — stop if `wait_for` cannot be written without an arithmetic operator or
  without an `#[expect]`.
- S-2 — stop if correcting `resolve`'s doc comment turns out to need a
  behaviour change to `resolve` itself.
- Surfaces: `crates/goad-semantics/src/schedule.rs`, `crates/goad/src/wire.rs`,
  `docs/slices/003/notes.md`. Must not touch `controller.rs`, `glass.rs`,
  `diagnostics.rs`, `reception.rs`, anything under `crates/goad-shell/src`, or
  any manifest.

**Tasks**
- [x] Verify EN-1 (tree at 572049f-descendant with no source change; `just
      check` green) before editing.
- [x] Red/green: `wait_for` unit tests in `schedule.rs` (VT-1, VT-2), then the
      function (EX-1, EX-2).
- [x] Amend `resolve`'s doc comment (EX-3).
- [x] Red/green: `Stimulus::Scheduled` tests in `wire.rs` (VT-3), then the
      variant, `kind()`, doc comment (EX-4).
- [x] `just check`; `cargo test -p goad-semantics` named separately (VA-1,
      VA-2).

**Decisions taken during execution**
- `wait_for`'s edge tests (VT-1) use `jiff::Timestamp::MIN`/`MAX` directly,
  matching the implementer note's citation of jiff 0.2.35's `SignedDuration`
  range guarantee (A-2, `design.md:539-540`); no new test helper needed.
- `resolve`'s amended doc comment states both E-1 successor cases as a
  two-item list rather than prose, matching `design.md` §5.5's own structure,
  and drops the `(F-1)` citation in favour of `SPEC-001/R-26`/`SPEC-001/R-28`
  (`docs/memory/cite-requirements-not-finding-ids.md`).

**Findings**
- A second `(F-1)` citation survives at `schedule.rs:327` (now, after the
  edit), inside the `#[cfg(test)] mod tests` doc comment for
  `an_elapsed_retained_check_is_consumed_and_the_default_poll_applies`. It
  predates this slice (commit `ad811c6d`, slice 002) and is outside EX-3's
  named location (`schedule.rs:198-201`, the function's own doc comment, not
  a test's). Left as found — EX-3 does not name it and repairing it would
  widen this phase's exit criterion past what was written. Worth a citation
  sweep at a later phase or at close.

**Criteria discharged**
- EN-1 — tree at `0b2e50f` (a `572049f` descendant, no source changed since);
  `just check` exit 0 before editing, confirmed twice
  (`/tmp/.../scratchpad/gate-entry.log`, `gate-entry2.log`, both exit 0).
- EX-1 — `wait_for(next_check, now) -> std::time::Duration` lands at
  `schedule.rs`, computing `max(next_check - now, 0)` via
  `duration_since`/`try_from`/`unwrap_or`, no `+`/`-` operator, under the
  module's existing `#![deny(clippy::arithmetic_side_effects)]`.
- EX-2 — no spacing parameter, no stratum-3 constant named; `resolve`,
  `parse`, `parse_span` unchanged in behaviour (only `resolve`'s doc comment
  edited).
- EX-3 — `resolve`'s doc comment restates both E-1 successor cases, cites
  `SPEC-001/R-26`/`SPEC-001/R-28`.
- EX-4 — `Stimulus` has three variants (`Startup, Requested, Scheduled`);
  `kind()` returns `"scheduled"` for the new one; `event()` unmodified and
  still total over `source`/`data` for all three; doc comment at `wire.rs`
  lists all three.
- EX-5 — no manifest touched (`git status --short` confirms).
- VT-1/VT-2 — five `wait_for` tests: future (exact), at-`now` (zero), past
  (zero, not underflow), and both `jiff::Timestamp` range edges (total, no
  panic); the zero cases assert `Duration::ZERO` by value.
- VT-3 — two `wire.rs` tests: `Stimulus::Scheduled.kind() == "scheduled"`,
  and `.event(now)` carries `source == "host"`, `kind == "scheduled"`,
  `timestamp == now`, `data == Value::Null`.
- VA-1 — `just check` exit 0, 10.253s wall (`gate-final.log`); tail pasted
  above in this session's tool output (58 `goad` unit/renderer/shape tests,
  30 `goad-semantics` tests including the five new `wait_for` cases, clippy
  and fmt clean).
- VA-2 — `cargo test -p goad-semantics` run standalone: 30 passed, 0 failed
  (includes the five `wait_for` tests and the amended `resolve` suite
  unchanged in count/behaviour beyond the new additions).

**STOP conditions encountered:** none. S-1 and S-2 did not trigger — the
design's exact `wait_for` body compiled clean under the module's arithmetic
deny with no suppression, and `resolve`'s doc comment was reworded without
touching its behaviour.

### PHASE-02 — The wait, and the cadence it keeps

**Objective:** `serve` wakes on time — the `select!` timer arm, the floor
anchor, `Absorbed`, and the six timed assertions that drive every branch but
the refusal one (PHASE-03's).

**Reading list**
- `plan.md:487-777` — the whole PHASE-02 entry: EN-1/EN-2; EX-1..EX-12; VT-1..
  VT-8, VA-1..VA-4; S-3..S-6, S-20, S-21; implementer notes.
- `design.md:151-292` (§5.2) — `wait_for` (already landed), `MINIMUM_SPACING`,
  `Absorbed { shift, next_check }`, `Pending::now()`, D-14/D-15's "no
  `Controller::next_check()` accessor".
- `design.md:322-470` (§5.4) — the loop's mermaid, the written-out `select!`,
  the floor rule stated exactly, D-3/D-5/D-6/D-11.
- `design.md:632-654` (§7) — D-1..D-18, all read; D-3/D-5/D-6/D-11/D-14/D-15
  bind this phase directly.
- `design.md:692-720` (§9 margin table) — the six rows this phase measures
  (AC-1, AC-2×2, AC-3×2, AC-7); VA-2 pastes observed against these.
- `draft-spec.md:73-166` — R-1..R-11, and §5's stated behaviour/mermaid.
- `docs/policy/001-the-phase-gate.md`, `docs/adr/001-one-way-strata.md`.
- `docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md`,
  `shared-test-helper-lives-at-workspace-root-via-path.md`,
  `cite-requirements-not-finding-ids.md`, `a-bound-is-not-tested-at-the-bound.md`,
  `stop-letter-vs-purpose-is-a-plan-log-adjudication.md`,
  `git-stash-forbidden-recover-read-only.md`.
- `crates/goad/src/controller.rs` (whole, 424 lines, read) — `absorb`
  (`:136-159`), `stamp` (`:249-253`), `Pending` (`:261-282`), `serve`
  (`:298-393`), the refusal `match` (`:325-361`).
- `crates/goad-semantics/src/schedule.rs:254-257` — `wait_for`, landed.
- `crates/goad/src/clock.rs` (whole) — `Clock = fn() -> Result<Timestamp,
  ClockError>`.
- `crates/goad/tests/renderer/main.rs` (whole) — module declarations.
- `crates/goad/tests/renderer/wiring.rs:1-148` (fixtures to move), `:280-330`,
  `:500-680`, `:820-910`, `:1000-1231` (the 22 `.absorb` read sites and the six
  VT-1 tests, confirmed by grep against every `.absorb(` call in the file).
- `crates/goad/tests/renderer/table.rs:660-870` (the 9 `.absorb` read sites).
- `tests/support/driving.rs` (whole) — `host_from`, `config`, `scripted`,
  `invocations`; `answers-as-instructed.sh` and `answers-a-round-trip.sh` (the
  only backend script that logs the raw request).
- `crates/goad-shell/src/config.rs:1-73` — `Config`/`BackendConfig`/
  `ScheduleConfig` field shapes (PL-3).
- `docs/slices/003/slice-003.md:95-122` (Scope) — `tests/backends/` "extended,
  not duplicated"; the AC-10 target and `scripting.rs` split are **not** this
  phase's (PHASE-05's), read only to confirm they are out of scope here.

**Verified before starting (EN-1, EN-2):**
`just check` exit 0, 5.318s wall
(`/tmp/.../scratchpad/gate-entry.log`), tree at `c53d0e2`. `wait_for` and
`Stimulus::Scheduled` exist, covered by PHASE-01/VT-1..VT-3 (`notes.md` above).

**Assumptions**
- `absorb`'s 22 read sites and the fixture/`use`-line bill match EX-3/EX-11/
  EX-12 exactly — reconfirmed by grep against the tree at `c53d0e2` (all 22
  line numbers match; the discard sites are the remainder).
- Timestamps in this phase's tests use `stub_clock` (fixed instant), so a
  relative instruction ("100 milliseconds") yields the same wait every cycle
  and an absolute past instant yields zero every cycle — the implementer note's
  point, taken as given.

**Finding against the plan, resolved rather than a STOP** — the phase's own
"Must not touch" list names `tests/backends/` verbatim, but its own
"Notes for the implementer" section authorises exactly the case this phase
needs: VT-3 requires the second request's `event.kind` to be observed, and
`answers-as-instructed.sh` (`cat >/dev/null`) never echoes the request, so no
existing script can discharge it. The note says so directly — *"a new script
under `tests/backends/` is permitted by `slice-003.md`'s Scope, PL-7
notwithstanding"* — and `slice-003.md:103-104`'s Scope states `tests/backends/`
is "extended, not duplicated" for the whole slice. Read together: "must not
touch" bars editing the ten existing scripts (which several other tiers
depend on); adding one new, unreferenced-elsewhere script is the extension the
Scope and the note both name. Resolution: add exactly one new script,
`tests/backends/logs-the-request-then-answers.sh` — reads stdin, appends the
raw request verbatim to the invocation log (argv[2]), prints `{"view":null}`.
Used by VT-3 only. If this reading is wrong, PHASE-03's reviewer or the audit
should say so; recorded here rather than silently assumed.

**STOP conditions** (from the plan, watched throughout): S-3 (refusal re-arm
needs a redesign), S-4 (an existing test goes red), S-5 (tokio `test-util` /
mock clock tempted), S-6 (flaky across 3 runs), S-20 (a VA-2 margin under 5x),
S-21 (`wiring.rs`/`table.rs` needs an edit outside the three named classes).

**Tasks**
- [x] (a) `Absorbed` + `.shift` at the 22 sites; gate green (6.606s).
- [x] (b) `harness.rs` fixture move + `wiring.rs`'s `use`-line bill (EX-11,
      EX-12); gate green (6.606s).
- [x] (c) the `select!` third arm, `Fired`, the floor anchor, `Pending::now()`,
      the two re-arm sites; VT-1's six existing tests stay green unchanged
      (gate green, 6.928s).
- [x] (d) `scheduling.rs`: VT-2..VT-8, each measured (VA-2); the one new
      backend script (gate green, 5.732s).
- [x] (e) refactor; `cargo fmt`; final `just check`; VA-3 break-and-revert;
      VA-4 `git diff` capture (gate green, 9.377s).

**Decisions taken during execution**
- The conditional re-arm on a timer-arm refusal (EX-7) is written as an
  `if refusal_re_arms { sleep.as_mut().reset(floor_until); }` guard inserted
  at all three `Err(refused) => { controller.refuse(&refused); …; continue; }`
  sites in the `match command` block, rather than singled out to the one site
  the implementer note says is reachable (`Command::Evaluate`'s `stamp`
  failure). `refusal_re_arms = matches!(fired, Fired::Scheduled)` is computed
  once, before the match. Chosen because design.md §5.4's own written-out
  loop states the rule once, generally ("on a refusal: … re-arm iff this
  iteration came from the timer arm … every other refusal leaves the deadline
  exactly as it was"), not as a special case of one arm; the other two sites
  guard against a boolean that is always `false` there today, so behaviour is
  identical to singling it out, and the code does not need revisiting if a
  future slice routes another command through the timer arm.
- `Fired` derives only `Debug` — `matches!` needs no trait, and nothing else
  compares or clones a `Fired`.
- `MINIMUM_SPACING`'s and `Fired`'s doc comments cite `SPEC-002/R-4` and
  `design.md §5.4`/`F-12` respectively, not finding ids
  (`docs/memory/cite-requirements-not-finding-ids.md`).

**Findings**
- **Against EX-12's bill (resolved, not a STOP — recorded per
  `docs/memory/stop-letter-vs-purpose-is-a-plan-log-adjudication.md`'s
  evidence-first style, for the orchestrator to confirm).** EX-12 states
  `wiring.rs`'s import bill as **eight** `use` lines — four deleted, four
  narrowed, one added — and does not name `use std::time::Duration;`
  (originally `wiring.rs:13`). But `Duration`'s only two consumers in the
  file were `TIMEOUT`'s type (`const TIMEOUT: Duration = …`) and `until`'s
  signature/body (`bound: Duration`, `Duration::from_millis(5)`) — both of
  EX-11's seven moved fixtures. Confirmed by grep before editing: no other
  top-level use of `Duration` exists in the file (the two child-module
  `use std::time::Duration;`/`use std::time::{Duration, Instant};` at former
  `:794`/`:1102` are local to `mod interaction`/`mod cancellation` and
  independent of the top-level import). Leaving the top-level import in
  place after the move would not compile — `cargo build -p goad --tests`
  after the fixture move (with this line deleted) produced **zero**
  warnings, confirming both that the deletion was required and that nothing
  else regressed. Treated as within EX-11's fixture move rather than a ninth
  EX-12 line needing a separate STOP: it renames no symbol, changes no test
  body, and is the same class of change as EX-12's other eight lines (an
  import whose sole consumers are fixtures EX-11 already authorises moving),
  just one the plan's own bill-count missed. If this reading is wrong, it is
  a one-line fix to `wiring.rs`'s import block, not a redesign.
- **Against the "Must not touch" list (resolved, not a STOP).** Recorded
  above under Assumptions — `tests/backends/` is named in this phase's
  "Must not touch" list, but the phase's own "Notes for the implementer"
  section pre-authorises exactly the one new script VT-3 needs, citing
  `slice-003.md`'s Scope. Acted on in task (d): one new script,
  `tests/backends/logs-the-request-then-answers.sh` (mirrors
  `answers-as-instructed.sh`'s instruction-list mechanism, but logs each raw
  request the way `answers-a-round-trip.sh` already does, which
  `answers-as-instructed.sh` — `cat >/dev/null` — cannot). Used by VT-3 and
  VT-7 (the second needs it too: FD-4's "the second invocation's `event.kind`
  is `\"requested\"`, not `\"scheduled\"`" is the same observation VT-3
  needs). No existing script edited.
- **EX-12's own prose does not match its own table (noted for completeness,
  not acted on further).** The bill's lead sentence says "Four lines go, four
  narrow, one is added"; the table beneath it names three lines **deleted**
  (`Rc`, `ClockError`, `Timestamp`) and five **narrowed** (`generated`,
  `glass`, `i_slint_backend_testing`, `slint`, `crate::driving`). Followed the
  table (the literal enumeration) rather than the prose count; the final
  `git diff` (VA-4, below) matches the table plus the one `Duration` line
  above.

**Criteria discharged**
- EX-1 — `MINIMUM_SPACING: std::time::Duration = Duration::from_secs(3)`,
  private, doc comment cites `SPEC-002/R-4`, states not configurable, not
  visible to a backend, never applied to a person's own request.
- EX-2 — `Controller.next_check: Option<Timestamp>`, written only by
  `absorb`, read only by `frame()`; `Frame.next_check: Option<Timestamp>`
  (`pub`); no `Controller::next_check()` accessor exists.
- EX-3 — `Absorbed { pub shift: Shift, pub next_check: Timestamp }`; one
  `absorb`, returning `Absorbed`; all 22 named read sites take `.shift`
  (confirmed by grep against the plan's own line numbers before editing —
  exact match); the 19 discard sites (`;`-terminated, value unused) compile
  unchanged.
- EX-4 — `Pending::now(&self) -> Timestamp`, beside `exchanged()`.
- EX-5 — `sleep` pinned (`Box::pin(tokio::time::sleep_until(started +
  MINIMUM_SPACING))`), always armed; `floor_until: tokio::time::Instant`
  initialised to `started`.
- EX-6 — first `select!` gains a third arm, last in `biased` order; binds
  `Fired::{Command(Command), Scheduled}`; the timer arm's body is exactly
  the one write to `floor_until`, then yields `Fired::Scheduled`;
  `Command::Evaluate(Stimulus::Scheduled)` built after the match, through
  the same `stamp`.
- EX-7 — two re-arm sites: after `absorb`,
  `sleep.as_mut().reset(max(Instant::now() + wait_for(…), floor_until))`;
  after a refusal with `refusal_re_arms` true, `sleep.as_mut().reset(floor_until)`.
- EX-8 — no `expect`/`unwrap`/`panic` added to `crates/goad/src`; no
  `#[expect]` added there (`git diff` shows none).
- EX-9 — `use goad_semantics::schedule::wait_for;`, no brace group;
  `resolve`/`schedule::resolve` named nowhere in `crates/goad/src`
  (unchanged from before this phase).
- EX-10 — `main.rs` declares `mod harness;` and `mod scheduling;`, both
  `#[cfg(test)]` at the declaration.
- EX-11 — `harness.rs` holds the seven fixtures, `pub(crate)`, bodies and
  signatures byte-identical to their prior form (diffed by eye against the
  pre-move read); nine child modules in `wiring.rs` unedited, resolving
  through the new top-level `use crate::harness::{…}` exactly as
  `wiring.rs`'s existing `crate::driving` re-export pattern already does.
- EX-12 — the eight named lines land as specified (see Findings above for
  the one the bill did not name).
- VT-1 — all six named tests pass with the shape unchanged (confirmed via
  `cargo test -p goad --test renderer` after tasks (a)-(c), 123 passed, 0
  failed; the six confirmed present by name via `-- --list`).
- VT-2/VT-3 —
  `scheduling::a_short_default_poll_is_honoured_unfloored_for_the_first_scheduled_check`:
  `default_poll = 100ms`, backend returns `{"view":null}` every call;
  invocation count reaches 2 inside `until(2s)`; second request's
  `event.kind == "scheduled"` (via `logging_scripted`/`request_kind`).
- VT-4 —
  `scheduling::an_instruction_from_an_evaluate_shortens_the_wait_past_a_far_default_poll`:
  `default_poll = DEFAULT_POLL` (30 min), first response instructs
  `"100 milliseconds"`; second invocation inside `until(2s)`.
- VT-5 —
  `scheduling::an_instruction_from_a_respond_shortens_the_wait_past_a_far_default_poll`:
  first response carries a view; `Choose` naming it (view token read off the
  window, via `harness::current_view_token`) answered with `"100
  milliseconds"`; third invocation inside `until(2s)`.
- VT-6 —
  `scheduling::an_earlier_instruction_supersedes_a_pending_far_deadline`:
  first instruction `"60 seconds"`, then a person-driven evaluate answering
  `"100 milliseconds"`; third (scheduled) invocation inside `until(2s)`.
- VT-7 —
  `scheduling::a_later_instruction_supersedes_and_the_earlier_deadline_does_not_fire`:
  first instruction `"100 milliseconds"`, second evaluate — enqueued on a
  capacity-1 channel, `.await`-blocked until the first is dequeued, so
  enqueued before that exchange completes by construction (FD-4) — instructs
  `"60 seconds"`; second invocation's `event.kind == "requested"`; no third
  invocation inside a 300 ms window after.
- VT-8 —
  `scheduling::a_stop_issued_while_parked_on_the_timer_arm_ends_serve_well_inside_the_timeout`:
  far `default_poll`, channel drained after one exchange, loop parked on the
  timer arm; `Cancel::stop()` from the driving task; `serve` returns
  `Ending::Stopped`, measured elapsed 99.991 µs against `TIMEOUT` (2 s).
- VA-1 — `just check` exit 0 after every task: (a) 6.606s, (b) 6.606s, (c)
  6.928s, (d) 5.732s, (e, final) 9.377s. Tails pasted in this session's tool
  output; `/tmp/.../scratchpad/gate-task-{a,b,c,d}.log`, `gate-final.log`.
- VA-2 — margins measured (all six timed rows `design.md` §9 names for this
  phase), against `cargo test -p goad --test renderer scheduling::` observed
  per-test (`nth` run, no `--nocapture` instrumentation left in the tree):

  | assertion | kind | design expected | bound | observed | margin (obs. vs. bound) |
  |---|---|---|---|---|---|
  | AC-1 (VT-2/VT-3) | liveness | ~105 ms | `until(2s)` | ~270 ms | ~7.4x |
  | AC-2, from an evaluate (VT-4) | liveness | ~105 ms | `until(2s)` | ~250 ms | ~8x |
  | AC-2, from a respond (VT-5) | liveness | ~105 ms | `until(2s)` | ~290 ms | ~6.9x |
  | AC-3, earlier supersedes (VT-6) | liveness | ~105 ms | `until(2s)` | ~260 ms | ~7.7x |
  | AC-3, later supersedes (VT-7) | anti-fire | no firing | 300 ms window | ~480 ms total (incl. the 300 ms deliberate wait) | not a race — structural |
  | AC-7 (VT-8) | liveness | at once | `TIMEOUT` (2s) | 99.991 µs (the measured `elapsed`, not the whole test's wall time) | ~20 000x |

  No row is below the 5x floor (S-20 not triggered); the closest is VT-5 at
  ~6.9x. `finished in Xs` figures are the test harness's own per-test time
  (includes real subprocess spawns for `bash`/the backend script, not just
  the instructed wait — the design's "~105 ms" figures are the idealised
  wait alone). No flakiness across 3 consecutive full runs of `scheduling::`
  (`/tmp/.../scratchpad/scheduling-flake-check.log`: 1.870s/1.887s/1.895s
  real, all 6 passed each time) — S-6 not triggered.
- VA-3 — `MINIMUM_SPACING` set to `Duration::from_secs(0)`, `cargo test -p
  goad --test renderer` run: 129 passed, 0 failed (123 existing + 6 new).
  Reverted; confirmed no stray probe artifact remains (`grep -n "VA-3\|
  from_secs(0)"` empty). Recorded here per the plan; `design.md` not edited.
- VA-4 — `git diff` over `wiring.rs`/`table.rs`, captured at
  `/tmp/.../scratchpad/va4-diff-final.patch` (327 lines; `--stat`: table.rs
  +24/-→net, wiring.rs +39/-100 total across both). Read in full: 22 `.shift`
  additions (13 wiring, 9 table, all at the plan's own line numbers); seven
  fixture bodies deleted from `wiring.rs` (`Rc`/`Duration`/`ClockError`
  import block, `now`, `stub_clock`, `window_and_tray`, `glass_over`,
  `current_view_token`, `until` — byte-identical to their moved form in
  `harness.rs` bar the `pub(crate)` EX-11 permits); the import-block bill
  (see Findings for the one extra line, `Duration`, beyond EX-12's eight). No
  renamed symbol, no changed test body, no touched `use super::` line, no
  `use` line outside EX-12's set plus the one documented exception.
- S-3, S-4, S-5, S-6, S-20, S-21 — none triggered as a **halt**. Two
  boundary tensions (`tests/backends/`, the ninth `use` line) were resolved
  and documented above rather than halted on; see Findings for the
  reasoning, evidence, and the explicit flag for the orchestrator to confirm
  or overrule.

**`just check` (final, task e):** exit 0, 9.377s real
(`/tmp/.../scratchpad/gate-final.log`). `cargo test --workspace` alone (a
separate, later measurement, not a controlled before/after — PHASE-01's
`gate-entry` logs only recorded the six-command total, not this command in
isolation): 4.757s real
(`/tmp/.../scratchpad/cargo-test-workspace-after.log`). `git status --short`:
`crates/goad/src/controller.rs`, `crates/goad/tests/renderer/{main.rs,
table.rs, wiring.rs}` modified; `crates/goad/tests/renderer/{harness.rs,
scheduling.rs}`, `tests/backends/logs-the-request-then-answers.sh` new;
`flake.lock` modified but pre-dates this session (untouched by this phase).

### PHASE-03 — What the floor bounds, and what a failure does not stop

**Objective:** the floor and the refusal path are exercised, not just
implemented — a past instant on every response, once, a failing backend, a
clock that fails after one success, and a person acting mid-cadence.

**Reading list**
- `plan.md:779-958` — the whole PHASE-03 entry: EN-1/EN-2; EX-1..EX-3;
  VT-1..VT-6, VA-1..VA-3; S-7..S-9, S-22, S-23; implementer notes.
- `design.md:557-575` (E-1) — the past-instant successor cases, one-off vs.
  every-response; `:632-654` D-2, D-3, D-14 (the floor); `:679`, `:711-712`
  (AC-9's succeed-once clock trace and margin rows); `:665-720` §9 (validation
  table, margins).
- `draft-spec.md` R-3..R-6, R-8 (`:79-84`), and the verification table
  `:174-179`.
- `docs/policy/001-the-phase-gate.md`, `docs/memory/*.md` (all 16 files read).
- `docs/slices/003/notes.md` PHASE-02 sheet (above) — the three-site
  `refusal_re_arms` guard (EN-2, confirmed present by grep before starting:
  `controller.rs:406`, `:415`, `:428`); PHASE-02's fixtures and test style in
  `harness.rs`/`scheduling.rs`; the `logs-the-request-then-answers.sh` script
  already landed for `event.kind` reads.
- Code, read whole or by grep: `crates/goad/src/controller.rs` (`serve`
  `:341-473`, `absorb` `:154-176`, `stamp` `:267-271`, `Pending` `:279-309`,
  the three refusal sites); `crates/goad/src/diagnostics.rs` (`Refused`,
  `Diagnostics::refused`, `:140-153`); `crates/goad/src/clock.rs` (`Clock`,
  `ClockError`, whole); `crates/goad-semantics/src/schedule.rs::resolve`
  (`:232-247`) and `wait_for` (`:254-257`); `crates/goad-shell/src/host.rs`
  (`:65-76`, `:128`, `:228-259`, `:289-293` — `resolve` is called on every
  outcome including a failure); `crates/goad/tests/renderer/scheduling.rs`
  and `harness.rs` (whole, both); `tests/support/driving.rs` (whole);
  `tests/backends/answers-as-instructed.sh` (sentinels, `@garbage`),
  `logs-the-request-then-answers.sh`; `crates/goad-shell/src/config.rs`
  (`ScheduleConfig`, `:72-73`, current state: no doc comment).

**Verified before starting (EN-1, EN-2):** `just check` exit 0
(`/tmp/.../scratchpad/gate-entry.log`), tree at `8d1b0ec`. `controller.rs`'s
three refusal sites each carry `if refusal_re_arms { sleep.as_mut()
.reset(floor_until); }` (grep, `:406`, `:415`, `:428`) — the branch exists,
undriven until this phase's VT-3, per EN-2's own wording (a document check,
not a coverage claim).

**Key fact governing every timed assertion here:** `harness::stub_clock` is
fixed at `2026-01-01T00:00:00Z` on **every** call — `stamp`'s `now` argument
never advances in wall-clock terms even though real time passes between
invocations. `schedule::resolve`'s two "no incoming" branches — "retained
stands" and "now + default_poll" — therefore land on the *same numeric
value* whenever the retained instant equals `now + default_poll` exactly,
which VT-2 (AC-5/AC-8) exercises: the test cannot distinguish "retained held"
from "recomputed" by the final value alone, only that neither path produced
something else (a defect this design does not need to distinguish from — the
observable claim is R-29, "unaffected by the failure", and a wrong branch
would silently agree with it here). Recorded so a future reader does not
mistake the test for weaker than it looks by accident.

**Assumptions**
- `answers-as-instructed.sh`'s `@garbage` sentinel discharges VT-2 (AC-5/
  AC-8) unmodified — confirmed by reading the script: `exits-zero-with-
  unparseable-stdout.sh`'s behaviour inline, no new script needed for this
  case (only VT-3/VT-6, below, reuse the existing
  `logs-the-request-then-answers.sh` PHASE-02 already added).
- VT-3/VT-4's fixture is a single `static AtomicUsize` `fn`, used by exactly
  one test (`docs/memory` note on `Clock`'s `fn`-pointer type; the plan's own
  warning about `cargo test`'s parallel threads sharing one binary's
  statics).
- EX-2's citation of `SPEC-001/R-21` (not a slice-003 requirement) is
  design.md E-5's own point verbatim: `default_poll` goes through the same
  duration grammar R-21 states, and refusing a value below the floor would
  invent a rule that grammar does not carry. Cited exactly as the plan's
  criterion states it.

**STOP conditions** (watched throughout): S-7 (a test needs `#[ignore]`,
retry, an oversized sleep or a widened tolerance to pass — POL-001), S-8
(VT-1's count is not 2), S-9 (the phase needs to change the loop's shape, not
repair a defect — back to plan), S-22 (a VA-2 margin below 5x — consult the
orchestrator, do not ship as a follow-up), S-23 (VT-5's `frame().next_check`
is neither the past instruction nor `now + default_poll`).

**Tasks**
- [x] (a) `config.rs`: `ScheduleConfig::default_poll` doc comment (EX-2),
      citing SPEC-002/R-5 and SPEC-001/R-21.
- [x] (b) `scheduling.rs`: VT-1 (AC-4, past instant every response) and VT-2
      (AC-5/AC-8, failing backend) — both against the mechanism as it
      stands, no repair anticipated.
- [x] (c) `scheduling.rs`: VT-3/VT-4 (AC-9, the succeed-once clock and its
      vacuity control).
- [x] (d) `scheduling.rs`: VT-5 (the one-off past instruction, R-3) and VT-6
      (a person mid-cadence, R-4/R-5).
- [x] (e) VA-2 margins measured per test, 3 runs each for flake-check on the
      new module; VA-3 break-and-revert; refactor; `just check` final.

**Findings**
- **VT-3's own stated assertions (one `NoClock` line, retained `next_check`
  unchanged, invocation count still 1) do not by themselves make VA-3's
  break-and-revert meaningful for AC-9.** With `MINIMUM_SPACING` zeroed, the
  clock still fails on every read after the first, so the backend is never
  reached either way — `invocations(&log) == 1` holds whether or not the
  floor is doing anything, and `Diagnostics::refused` overwrites rather than
  accumulates, so the line count stays 1 too. Neither assertion is capable of
  going red under VA-3, which the plan requires ("VT-3's count assertion goes
  red"). Resolution: added one more assertion the plan does not name —
  `CLOCK_READS.load(Ordering::SeqCst) == 2` — reading the test's own fixture
  counter (already present for VT-3/VT-4's mechanics) as the direct witness
  of a spin. Confirmed by VA-3, below: 2 with the floor, 381 in the same
  500 ms window with it zeroed. This is the assertion that actually holds
  AC-9's "does not spin" clause; without it the criterion was checking
  everything a floor failure leaves unchanged and nothing it would change.
  If this reading is wrong, it is a one-assertion addition to revisit, not a
  redesign.
- **VA-2's own stated expectation for VT-5 and VT-6 (~105 ms liveness) does
  not match either test's trace, though neither criterion depends on the
  number.** The plan's VA-2 note predicts "~105 ms liveness against
  `until(2 s)`… the same shape as VT-1's" for both. VT-1's own liveness is
  ~12 ms, not ~105 ms (its second invocation is the *unfloored* scheduled
  firing a past instruction produces — zero computed wait, not a
  `default_poll`-gated one), and VT-5's second invocation and VT-6's third
  are the same two shapes (an unfloored past-instant firing, and a person's
  directly-dispatched evaluate) — neither gated by `default_poll` the way
  AC-1/AC-2's ~105 ms figure (the template this note was evidently copied
  from) is. Observed liveness for both is ~6-12 ms, not a defect — the
  margin against `until(2 s)` is far wider than predicted (~160-330x instead
  of ~19x) — but the design table's own arithmetic assumption is off for
  these two rows. Recorded for the audit to correct in `design.md` §9 if it
  promotes these rows, rather than silently matched to the wrong number.
- No `controller.rs` repair was needed (EX-3): all six VT cases, and VA-3's
  break-and-revert, passed against the mechanism PHASE-02 landed, unmodified
  — confirmed by `git diff crates/goad/src/controller.rs` being empty after
  the VA-3 edit-and-revert (below). This is itself further evidence for
  PHASE-02/EN-2's claim that the branch, though undriven before this phase,
  was written correctly.

**Criteria discharged**
- EX-1 — all six cases below pair an anti-spin window with a liveness
  assertion (or, for VT-4, a liveness-only vacuity control) over the same
  run; SPEC-002/R-4's second half and R-5's second half are VT-6's.
- EX-2 — `crates/goad-shell/src/config.rs`:
  `ScheduleConfig::default_poll`'s doc comment states the below-floor case,
  citing SPEC-002/R-5 and SPEC-001/R-21 (E-5's own point: refusing it would
  invent a rule the grammar does not carry). The only edit to `config.rs`
  this phase makes (confirmed by `git diff --stat`, below).
- EX-3 — `crates/goad/src/controller.rs` has a zero-line diff (`git status
  --short`, `git diff`, both below); no production file outside it was
  touched.
- VT-1 — `a_past_instant_on_every_response_fires_once_and_then_holds_at_the_floor`:
  `default_poll` far (30 min), backend instructs `2020-01-01T00:00:00Z` on
  every response. Second invocation inside `until(2 s)`; count holds at 2
  across a 500 ms window; `Served::controller.frame().next_check ==
  Some(2020-01-01T00:00:00Z)` after the loop stops (SPEC-002/R-6).
- VT-2 — `a_failing_backend_is_retried_unprompted_never_faster_than_the_floor`:
  `default_poll` 100 ms, backend `@garbage` on every response. Second
  invocation inside `until(2 s)` (AC-8); count holds at 2 across a 500 ms
  window (AC-5); retained `next_check == Some(2026-01-01T00:00:00.100Z)`
  after the loop stops, unaffected by the failure (SPEC-001/R-29).
- VT-3 — `a_clock_that_fails_after_the_startup_exchange_refuses_and_holds`:
  `succeeds_once_then_fails`, a `fn` over a private `static AtomicUsize`
  (`CLOCK_READS`), used by this test only. `default_poll` 100 ms, honest
  backend. Startup exchange succeeds; the scheduled firing's `stamp` fails;
  inside a 500 ms window: invocation count stays 1, exactly one diagnostic
  line containing "system clock could not be read", `CLOCK_READS == 2` (the
  added spin witness, see Findings), retained `next_check ==
  Some(2026-01-01T00:00:00.100Z)` unaffected by the refusal (SPEC-001/R-8).
- VT-4 — `the_same_shape_with_a_working_clock_reaches_a_second_invocation`:
  same shape, `stub_clock` (never fails) — second invocation inside
  `until(2 s)`, confirming VT-3's count-still-one is not vacuous.
- VT-5 — `a_one_off_past_instruction_is_consumed_and_cadence_resumes`:
  instruction 1 the past instant, instruction 2 none, `default_poll` 100 ms.
  Second invocation inside `until(2 s)`; count holds at 2 across a 500 ms
  window (SPEC-002/R-3); `next_check == Some(2026-01-01T00:00:00.100Z)` —
  `now + default_poll`, neither the past instruction nor a third value
  (S-23 not triggered).
- VT-6 — `a_person_acting_mid_cadence_does_not_clear_the_floor`: VT-1's
  backend and `default_poll`. After the second (scheduled) invocation lands,
  a person's `Command::Evaluate(Stimulus::Requested)` is sent; the third
  invocation lands inside `until(2 s)` of that send (R-5) and the count then
  holds at exactly 3 across a 500 ms window from the send (R-4).
- VA-1 — `just check` exit 0, entry (`gate-entry.log`, 8d1b0ec) and final
  (`gate-final.log`), both pasted below.
- VA-2 — margins (3 runs each, `--exact`, temporary `eprintln!`
  instrumentation added, measured, then removed — none left in the tree,
  confirmed by `git diff` against this phase's own scheduling.rs):

  | assertion | kind | design/plan expected | bound | observed (3 runs) | margin |
  |---|---|---|---|---|---|
  | VT-1 (AC-4) liveness, 2nd invocation | liveness | "at once" (D-5, unfloored) | `until(2s)` | ~11.2/13.5/11.8 ms | ~150-180x |
  | VT-1 (AC-4) anti-spin, count holds | anti-spin | 2 invocations | 500 ms window | count stayed 2, all 3 runs | 6x (window vs. 3s floor) |
  | VT-2 (AC-5/AC-8) liveness, 2nd invocation | liveness | ~105 ms | `until(2s)` | ~108.8/113.2/110.8 ms | ~18x |
  | VT-2 (AC-5) anti-spin, count holds | anti-spin | count unchanged | 500 ms window | count stayed 2, all 3 runs | 6x |
  | VT-3 (AC-9) startup invocation | liveness | ~5-10 ms | `until(2s)` | ~6.2/6.3/5.5 ms | ~300x |
  | VT-3 (AC-9) refusal-window hold | anti-spin | window ≈ 500 ms wall | (sanity check) | 501.4/500.7/501.6 ms | n/a — confirms the window itself, not a margin |
  | VT-3 (AC-9) spin witness, `CLOCK_READS` | anti-spin | 2 reads | 500 ms window | 2, all 3 runs (381 under VA-3, see below) | the assertion VA-3 needed |
  | VT-4 (vacuity control) liveness | liveness | ~105 ms | `until(2s)` | ~108.6/110.1/109.5 ms | ~18x |
  | VT-5 (R-3) liveness, 2nd invocation | liveness | ~105 ms (VA-2's own note — see Findings, does not match the trace) | `until(2s)` | ~11.5/11.7/12.2 ms | ~166x (not ~19x) |
  | VT-5 (R-3) anti-spin, count holds | anti-spin | 2 invocations | 500 ms window | count stayed 2, all 3 runs | 6x |
  | VT-6 (R-4/R-5) liveness, 3rd invocation | liveness | ~105 ms (VA-2's own note — see Findings, does not match the trace) | `until(2s)` | ~6.3/6.2/5.4 ms | ~330x (not ~19x) |
  | VT-6 (R-4/R-5) anti-spin, count holds | anti-spin | count unchanged | 500 ms window from send | count stayed 3, all 3 runs | 6x |

  No row is below the 5x floor (S-22 not triggered) — every row is at or
  above design's own worst case. No flakiness across 3 consecutive full runs
  of `scheduling::` (all 12 tests, `finished in 0.78s` each run, 3/3 green).
- VA-3 — `MINIMUM_SPACING` set to `Duration::from_secs(0)` in
  `crates/goad/src/controller.rs`. `cargo test -p goad --test renderer
  scheduling::`: VT-1, VT-2, VT-5, VT-6 all failed as predicted (each landed
  a 5th invocation instead of holding at 2 or 3); VT-3's added `CLOCK_READS`
  assertion failed too (381 reads in the 500 ms window instead of 2) — this
  is the assertion the Findings entry above added specifically so this line
  would fire. All other tests (the eight PHASE-02 cases plus VT-4) stayed
  green, as expected — they do not depend on the floor. Reverted; `cargo
  test -p goad --test renderer scheduling::` green again, 12/12, confirmed
  by re-running (above). `git diff crates/goad/src/controller.rs` after the
  revert: empty.
- S-7, S-8, S-9, S-22, S-23 — none triggered. S-8 and S-23 confirmed
  affirmatively above (VT-1's count is exactly 2; VT-5's `next_check` is
  `now + default_poll`, no third value).

**`just check` (final):** exit 0, 8.932s real (`gate-final.log`,
`/tmp/.../scratchpad/gate-final.log`) — up from PHASE-02's final 9.377s
(this run measured slightly faster overall, within normal machine-load
variance; not a regression). `cargo test --workspace` in isolation: 4.768s
real (`/tmp/.../scratchpad/cargo-test-workspace-after-p3.log`), against
PHASE-02's 4.757s — negligible growth from six added tests, each a real
subprocess-backed `serve` run. Re-confirmed after this sheet's own final
edits: exit 0, 5.276s real (`gate-final-confirm.log`).

**`git status --short`:** `crates/goad-shell/src/config.rs`,
`crates/goad/tests/renderer/scheduling.rs`, `docs/slices/003/notes.md`
modified; `flake.lock` modified but pre-dates this session (untouched by
this phase, as PHASE-02 also noted). No file outside this phase's declared
surfaces; `crates/goad/src/controller.rs` unmodified;
`tests/backends/`/`tests/support/`/`Cargo.toml` untouched.

### PHASE-04 — What the person sees, and what the scan holds

**Objective:** the host reports the next check it holds, saying which of the
two instants it is; two instruments in `goad-boundary` hold the claim that
the timer resolves nothing.

**Reading list**
- `plan.md:958-1084` — the whole PHASE-04 entry (EN-1/2, EX-1..EX-5,
  VT-1..VT-5, VA-1..VA-3, S-10..S-12, S-24, implementer notes); `plan.md`
  FD-2 (`:170-192` approx) — the two mechanical gaps in `structure.rs`;
  Coverage tables' AC-6/R-2 rows.
- `design.md` §5.2 `:263-291` (D-9, D-17, the next-check line and its own
  window property); §5.5 I-1/I-1a `:471-500` (AC-6's two resolution sites,
  the identifier-not-path reasoning); §9 AC-6 row `:676` (the two
  instruments, measured counts).
- `draft-spec.md` `:150-166` — R-2, and the closing clause forbidding the
  line from presenting the instruction as a prediction.
- `docs/policy/001-the-phase-gate.md` (instruments; never `allow`).
  `docs/memory/slint-build-mechanics.md` (build.rs/generated setter
  mechanics), `clippy-toml-test-exemptions-are-a-hidden-boundary.md`,
  `cargo-test-cwd-is-package-root-not-workspace-root.md`,
  `expect-dead-code-ahead-of-caller-needs-cfg-attr.md` — read, none bite
  this phase (no relocation, no cwd-relative path, no ahead-of-caller
  helper).
- Code read whole or by grep: `crates/goad-boundary/tests/checks/
  structure.rs` (whole — the file plan/team-lead call `src/structure.rs`
  does not exist; this is the one FD-2/EX-2 describe); `crates/goad-boundary/
  src/scan.rs` `mentions` (`:208-232`) and `workspace_root`; `crates/goad/
  ui/app.slint` (whole); `crates/goad/src/diagnostics.rs` (whole — the
  escape/bound `finish` pipeline); `crates/goad/src/glass.rs` (whole);
  `crates/goad/src/controller.rs` `Frame`/`Absorbed`/`next_check` (grep —
  `Frame::next_check` already `Option<Timestamp>` from PHASE-02, untouched
  here); `crates/goad/tests/renderer/wiring.rs` `mod transitions` and the
  DT-5 test (`:147-201`); `crates/goad-semantics/src/protocol/canonical.rs`
  `Timestamp` (`:103-117`, `Display` via jiff, `instant()`/`new()`).

**Assumptions**
- The team-lead brief's surface path `crates/goad-boundary/src/structure.rs`
  is a misstatement; the real file is `crates/goad-boundary/tests/checks/
  structure.rs`, matching `plan.md`'s own Surfaces line and FD-2. Proceeding
  on the file that exists.
- VT-2 (the renderer test) goes in `wiring.rs`'s `mod transitions`, beside
  DT-1/DT-5, per the plan's own note that its fixtures already live there.
  `tree.rs` (also listed as a surface) is read but not touched unless VT-2
  needs a headless-tier assertion it turns out `wiring.rs` cannot make —
  not anticipated, since `window.get_next_check()` is a direct generated
  getter exactly like `window.get_notice()`.
- AC-6 (b)'s matcher is the same `mentions`-based function as (a)'s: FD-2
  says "two matchers" total (the old `str::contains` one, kept for the
  three existing needles, and one built on `goad_boundary::scan::mentions`)
  — `mentions` itself already branches on `::` to become a path-substring
  match for `"schedule::resolve"`, so instrument (b) reuses the same
  function as instrument (a) rather than needing a third.

**STOP conditions watched:** S-10 (markup beyond one property/one line),
S-11 (renaming a `wire.rs` test fn), S-12 (either instrument needs
`goad-boundary/src/` to change), S-24 (VT-2 needs a `harness.rs` fixture).

**Tasks**
- [x] (a) `structure.rs`: directory-parameterise `subject_files`/
      `occurrences_of`; add `mentions_occurrences_of`; two new tests
      (VT-3, VT-4) plus vacuity guard over both directories (EX-3) plus
      `counting_itself` controls (VT-5).
- [x] (b) `diagnostics.rs`: `next_check_line` + VT-1 unit tests.
- [x] (c) `app.slint`: `next-check` property + one markup line (EX-4).
- [x] (d) `glass.rs`: write `next-check` on every `present`.
- [x] (e) `wiring.rs`: VT-2.
- [x] (f) refactor, lint, fmt, `just check` final.

**Findings**
- None against the design or plan. FD-2's own resolution (one
  `mentions`-based matcher covering both AC-6 instruments, since `mentions`
  itself branches on `::`) held exactly as the plan's Notes for the
  implementer anticipated — no third matcher was needed.
- The team-lead brief's stated surface `crates/goad-boundary/src/
  structure.rs` does not exist; the real file, matching `plan.md`'s own
  Surfaces line, is `crates/goad-boundary/tests/checks/structure.rs`. Noted
  as an assumption above; no design/plan defect, just an imprecise brief.

**Criteria discharged**
- EX-1 — `diagnostics.rs::next_check_line(at: Timestamp) -> String` renders
  second precision via `jiff::Timestamp::round(jiff::Unit::Second)`, falls
  back to the unrounded instant on a rounding error (`unwrap_or`, proven at
  `jiff::Timestamp::MIN`/`MAX`), names the instruction not the deadline
  ("next check (instructed): …"), and goes through `finish`
  (escape/bound).
- EX-2 — `structure.rs` is directory-parameterised
  (`subject_files(dir)`/`occurrences_of(dir, needle)`); two matchers, named:
  `occurrences_of` (substring, `str::contains`, the three existing needles
  unchanged in what they assert) and `mentions_occurrences_of`
  (`goad_boundary::scan::mentions`, both AC-6 instruments — see FD-2 note
  above).
- EX-3 — `the_subject_directories_are_found_and_are_not_empty` runs the
  vacuity guard for both `SUBJECT_DIR` and `SHELL_SUBJECT_DIR`;
  `no_production_line_in_the_renderer_names_the_identifier_resolve` and
  `schedule_resolve_is_called_only_from_host` each assert their own
  directory's file count is non-zero before asserting on occurrences.
- EX-4 — `PromptWindow` gains `in property <string> next-check`; one
  markup line (`Text { text: root.next-check; }`) renders it below the
  diagnostic `ScrollView`, above the Close button; `glass.rs`'s `present`
  writes it from `Frame::next_check` on every call
  (`frame.next_check.map(next_check_line).unwrap_or_default()`), `""` when
  `None`. Not appended to `diagnostic-lines`; the empty-state sentinel's
  condition (`app.slint`'s `if root.diagnostic-lines.length == 0`) is
  byte-for-byte unchanged.
- EX-5 — neither new instrument pins a line number: VT-4 asserts a count
  and a `BTreeSet` of file names.
- VT-1 — four unit tests in `diagnostics.rs`'s own `#[cfg(test)] mod tests`:
  an ordinary instant to second precision (verified against the exact
  string); `jiff::Timestamp::MIN` and `::MAX` both render with no panic;
  the pipeline assertion (`LINE_LIMIT` respected).
- VT-2 — `wiring.rs::vt2_the_next_check_line_has_its_own_property_and_
  leaves_the_sentinel_and_the_tray_alone`, in `mod transitions` beside DT-1:
  `""` before any exchange, non-empty after one; `Diagnostics::state() ==
  Idle`; `nothing_to_report_shown` still true. `just check` confirms DT-5
  (`dt1_a_...`) unchanged and green in the same run.
- VT-3 — `no_production_line_in_the_renderer_names_the_identifier_resolve`:
  0 occurrences over 12 `.rs` files under `crates/goad/src` (file count
  confirmed non-zero via `find … | wc -l` = 12, matching design.md §9).
- VT-4 — `schedule_resolve_is_called_only_from_host`: 2 occurrences over 8
  `.rs` files under `crates/goad-shell/src`, both `host.rs` (file count
  confirmed non-zero, `find … | wc -l` = 8).
- VT-5 — three new `counting_itself` tests: a comment-only `resolve` is not
  counted by the word matcher; a brace-grouped
  `use goad_semantics::schedule::{resolve, wait_for};` **is** counted (I-1a,
  F-3); `resolved` (the participle) is **not** counted — asserted, per
  `mentions`'s own contract (token or its plural only).
- VA-1 — `just check` exit 0, 14.716s real
  (`/tmp/…/scratchpad/gate-final-phase04.log`).
- VA-2 — break-and-revert on VT-3: added
  `use goad_semantics::schedule::{resolve, wait_for};` to `controller.rs`,
  confirmed `no_production_line_in_the_renderer_names_the_identifier_resolve`
  failed naming `controller.rs:15`, reverted (`git diff` empty after).
- VA-3 — break-and-revert on VT-4: added a third
  `schedule::resolve(None, None, …)` call site to `host.rs::new`, confirmed
  the count assertion failed `left: 3, right: 2` naming `host.rs:128`,
  `:129`, `:260`, reverted (`git diff` empty after).
- S-10, S-11, S-12, S-24 — none triggered. `app.slint` carries exactly one
  new property and one new markup line; no `wire.rs` test fn was renamed;
  `goad-boundary/src/` is untouched (`mentions`, `code_of`, `workspace_root`
  were already `pub`); VT-2 needed no `harness.rs` fixture —
  `window.get_next_check()` is a direct generated getter, the same pattern
  `window.get_notice()` already uses.

**`just check` (final):** exit 0, 14.716s real
(`/tmp/…/scratchpad/gate-final-phase04.log`).

**`git status --short`:** `crates/goad-boundary/tests/checks/structure.rs`,
`crates/goad/src/{diagnostics.rs,glass.rs}`, `crates/goad/tests/renderer/
wiring.rs`, `crates/goad/ui/app.slint`, `docs/slices/003/notes.md` modified;
`flake.lock` modified but pre-dates this session (untouched by this phase,
as PHASE-02/03 also noted). `controller.rs`, `goad-boundary/src/`,
`harness.rs`/`scheduling.rs`/`table.rs`, `tests/support/`, every manifest —
all confirmed untouched (`git diff --stat` empty for each).

### PHASE-05 — The topology

**Objective:** the waiting mechanism proved in the arrangement production
uses — a real Slint event loop (testing backend substituted, nothing else), a
multi-thread tokio runtime, the `EnterGuard`, `serve` polled by Slint's
executor — with exactly one component substituted and the substitution
stated.

**Reading list**
- `plan.md:1084-1211` — the whole PHASE-05 entry: Surfaces, Must-not-touch,
  EN-1, EX-1..EX-6, VT-1/VT-2, VA-1..VA-3, S-13..S-15, S-25..S-27,
  implementer notes.
- `design.md` §5.5 A-1 (`:519-537`, what S-1 substituted, F-8, what remains
  unproven — the production platform itself); D-12 (`:647`, one `[[test]]`
  target because the testing backend's init is once-per-process); D-18
  (`:652`, the `tests/support/` split, the refused `#[allow]`/`#[expect]`
  alternatives, POL-001's carve-outs); §9 AC-10 row (`:680`) and its margin
  row (`:713`, ~105 ms expected, `until(2 s)`, 19x).
- `research.md` Thread 5 (`:423-511`) — the Slint/tokio coexistence
  arrangement `main.rs` uses (three parts: `EnterGuard`, `spawn_local`,
  one `quit_event_loop` site) and spike S-1's four measured cases (A-D);
  case B2 (`reset` on an already-fired pinned `Sleep`) and case D (an
  already-elapsed `sleep_until`) bracket the loop's own AC-4 path per F-15.
- `docs/policy/001-the-phase-gate.md` — six commands, never `allow`, the
  site-local `#[expect]` carve-out.
- `docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`
  (PL-4's rule: every symbol in a shared file reachable from every
  includer — the rule FD-3/D-18 apply to the new split);
  `docs/memory/cargo-test-cwd-is-package-root-not-workspace-root.md` (the
  `#[path]` depth, `CARGO_MANIFEST_DIR` + `../..`);
  `docs/memory/slint-build-mechanics.md` (read; no build.rs change here);
  `docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` (read;
  not the shape here — nothing lands ahead of its caller).
- `docs/slices/003/notes.md` PHASE-02 sheet (the six timed `serve` tests'
  shape — `LocalSet` + `tokio::task::spawn_local`, `until`, `stub_clock`,
  `scripted`/`logging_backend`); PHASE-04 sheet (nothing relevant touches
  this phase's surfaces).
- Code read whole: `crates/goad/tests/event_loop/{main.rs,closing.rs}` (the
  template — runtime, `EnterGuard`, real window/tray, `install`, real
  channel/`Cancel`, `SlintGlass`, `spawn_local`, the one
  `quit_event_loop` site, `Rc<RefCell<Option<Ending>>>` read-back);
  `tests/support/driving.rs` (whole, 255 lines); `crates/goad/tests/
  renderer/scheduling.rs` (whole, 698 lines — VT-2/VT-3's shape is VT-1's
  template: `logging_scripted`/`scripted`, `until`, invocation counting);
  `crates/goad/tests/renderer/harness.rs` (confirms it only takes
  `driving::instant`, so it is outside the split's blast radius per the
  plan).
- Code read by grep: `#[path` across `crates/*/tests` and `tests/support`
  (confirms exactly two includers today: `crates/goad/tests/renderer/
  main.rs:38`, `crates/goad-shell/tests/integration/main.rs:10`); every
  `driving::{…}` import site in both targets' files (confirms the moving
  six — `backend`, `marker`, `clear`, `logging_backend`, `invocations`,
  `scripted` — are used by: `wiring.rs`, `table.rs`, `scheduling.rs`
  (renderer); `harness.rs` (re-export), `round_trip.rs`, `failure_matrix.rs`
  (integration); `host.rs` and `transport.rs` use only non-moving symbols
  (`instant`/`presented`, `CLEANUP_LIMIT`) and so need no import edit,
  confirmed by grep, not assumption); `crates/goad/src/controller.rs`
  `serve`'s signature and `Ending`/`Served` (`:341-350`, `:70-84`);
  `crates/goad/src/wire.rs` `Wire::new`/`Cancel::new` signatures;
  `crates/goad/src/install.rs` (whole, `pub fn install`);
  `tests/backends/answers-as-instructed.sh` (whole — past-the-list default
  response `{"view":null,"next_check":"45 minutes"}`, the positive
  control).

**Assumptions**
- `crates/goad-shell/tests/integration/transport.rs` and `host.rs` are
  declared surfaces (plan's Surfaces list) but need no edit: grep confirms
  neither imports a moving symbol. Left untouched; `git diff --stat` will
  show them empty, which is the expected shape, not a shortfall.
- The new target's `Config`/`Host` are built inline exactly as `closing.rs`
  builds them, using `scripting::scripted` for the backend half only — no
  new symbol added to either shared file (FD-3's whole point, per the
  plan's implementer notes).
- `Command::Evaluate(Stimulus::Requested)` (not `Startup`) is the dispatch
  the driving task sends, matching every PHASE-02/03 `serve` test's own
  pattern in `scheduling.rs`, rather than replaying `main.rs`'s
  `Stimulus::Startup` — the criterion is about the scheduled *second*
  invocation, not about which stimulus produced the first.

**STOP conditions watched:** S-13 (new dependency/feature), S-14 (split
needs a body change), S-15 (`init_integration_test_with_system_time()`
unavailable or diverges from S-1), S-25 (`harness.rs` imports a moved
symbol), S-26 (VA-3 margin below 5x), S-27 (an unused-import site other
than the one EX-1 names).

**Tasks**
- [x] (a) `tests/support/scripting.rs`: move `backend`, `marker`, `clear`,
      `logging_backend`, `invocations`, `scripted` out of `driving.rs`
      unchanged; `driving.rs` drops the `Path`/`PathBuf` import; both
      existing targets' `main.rs` gain a `#[path]` include, and their
      import sites split `driving::{…}`/`scripting::{…}`. `just check`
      green with no new target yet (EX-1, EX-2).
- [x] (b) `crates/goad/Cargo.toml`: the `event_loop_schedule` `[[test]]`
      target; `tests/event_loop_schedule/{main.rs,scheduling.rs}` skeleton
      proving the topology runs at all (EX-3..EX-6).
- [x] (c) VT-1's assertion: the watcher task, the AC-10 liveness bound,
      three measured runs.
- [x] (d) the break-and-revert (VA-3's negative-control cousin — the STOP
      list's own liveness proof that the test would fail if the timer arm
      were not polled).
- [x] (e) refactor, lint, fmt, `just check` final; VA-1..VA-3 pasted.

**Findings**
- The plan's VA-1 text says "it now runs seven test binaries in
  `crates/goad`". Measured (`cargo test --workspace` under `just check`,
  the `crates/goad` section only): `unittests src/lib.rs`,
  `unittests src/main.rs`, `tests/event_loop/main.rs`,
  `tests/event_loop_schedule/main.rs`, `tests/renderer/main.rs`, and
  `Doc-tests goad` (0 tests, still an invoked binary) — **six**, not seven.
  This is the same class of small planning-estimate drift the Harvest
  already records for PHASE-03's VA-2 note (a predicted shape that turned
  out to be a fixed offset from the true one); it does not change any
  criterion's substance — the new binary is present and green, which is
  what EX-3/VA-1 actually require — and is recorded rather than silently
  waved through.
- `eprintln!`-based margin instrumentation was drafted directly into VT-1
  (printing the watcher's observed elapsed time) and removed once
  `cargo clippy` reported it against `print_stderr`/`use_debug`
  (`Cargo.toml:150-152`, both `deny`, no test exemption — unlike the four
  keys `clippy.toml`'s `allow-*-in-tests` covers,
  `docs/memory/clippy-toml-test-exemptions-are-a-hidden-boundary.md`). The
  three measured runs below were taken with `--nocapture` before the print
  statement was removed, which is sufficient for VA-3 — the assertion
  itself needs no built-in instrumentation to be a liveness proof.

**Criteria discharged**
- EX-1 — `tests/support/scripting.rs` holds exactly `backend`, `marker`,
  `clear`, `logging_backend`, `invocations`, `scripted`, moved unchanged in
  body and signature. `driving.rs`'s diff is exactly the six functions
  leaving plus the one forced header edit, `use std::path::{Path,
  PathBuf};` at its old `:16` (confirmed: `git diff -- tests/support/
  driving.rs` shows no other line touched). Both existing targets' `main.rs`
  gained a second `#[path]` include; the new target's `main.rs` includes
  `scripting.rs` only, never `driving.rs`.
- EX-2 — `cargo clippy --workspace --all-targets -- -D warnings` exits 0
  (dead_code included); every `pub(crate)` symbol in both shared files is
  reachable from every target that includes that file.
- EX-3 — `crates/goad/Cargo.toml` gained one `[[test]]` target, `name =
  "event_loop_schedule"`, `path = "tests/event_loop_schedule/main.rs"`. No
  dependency, feature, or `dev-dependency` line touched (confirmed:
  `git diff crates/goad/Cargo.toml` shows only the new `[[test]]` block).
- EX-4 — the target carries exactly one `#[test]` fn,
  `scheduling::a_scheduled_evaluation_fires_under_the_production_topology`.
- EX-5 — the test's own composition (`tests/event_loop_schedule/
  scheduling.rs`) mirrors `start`/`closing.rs`: a multi-thread `tokio`
  runtime built with `enable_all()`, its `EnterGuard` held for the loop's
  life, a real `PromptWindow`/`Tray`, `install`'s callback table, a real
  `mpsc` channel and `Cancel`, `SlintGlass`, `ProcessBackend` against the
  real `answers-as-instructed.sh` child, the production `serve`, and
  `slint::spawn_local`. Init is `init_integration_test_with_system_time()`.
- EX-6 — the module doc on `tests/event_loop_schedule/main.rs` states the
  one substitution (the Slint platform / testing backend) and what it does
  not reach, citing A-1 and F-8.

**Verification**
- VT-1 — `a_scheduled_evaluation_fires_under_the_production_topology`:
  dispatches `Command::Evaluate(Stimulus::Requested)` from a watcher
  `spawn_local` task, waits for the invocation log to show a second
  invocation, trips `Cancel`, and asserts `served.ending ==
  Some(Ending::Stopped)` and `invocations(&log) == 2`. Green.
- VT-2 — the negative control is not code (the test body cannot poll while
  `run_event_loop_until_quit` is running — there is nothing to call it
  from), and is recorded here per the plan's own instruction: the
  watcher-task shape is required because `run_event_loop_until_quit` blocks
  the thread that owns both the Slint executor and any code that could
  observe the invocation log from the test body; `research.md` Thread 5's
  spike S-1 is the prior art (same shape, same reason, `timer-probe.local.rs`
  cases B1/B2).
- VA-1 — `just check` under the dev shell, exit 0, **6.886s real** then
  **5.606s real** on a repeat run (both from a warm `target/`; PHASE-04's
  cold-er final run was 14.716s, so this is not a regression), pasted at
  `/tmp/claude-1000/-home-david-dev-goad/302a7bbc-4adf-436d-ae25-a8c85c96ad29/
  scratchpad/gate-final-phase05.log`. The new binary appears: `Running
  tests/event_loop_schedule/main.rs
  (target/debug/deps/event_loop_schedule-…)`, `running 1 test … ok`. Six
  binaries run in `crates/goad`'s section, not seven — see Findings.
- VA-2 — `git diff --stat` over `tests/support/` and both existing
  targets:
  ```
  crates/goad-shell/tests/integration/failure_matrix.rs |  5 ++---
  crates/goad-shell/tests/integration/harness.rs        |  5 +++--
  crates/goad-shell/tests/integration/main.rs           |  7 +++++++
  crates/goad-shell/tests/integration/round_trip.rs     |  5 +++--
  crates/goad/tests/renderer/main.rs                    | 11 +++++++++--
  crates/goad/tests/renderer/scheduling.rs              |  3 ++-
  crates/goad/tests/renderer/table.rs                   |  5 ++---
  crates/goad/tests/renderer/wiring.rs                  |  3 ++-
  tests/support/driving.rs                              | 87 --------------
  ```
  No symbol renamed, no body changed; `transport.rs` and `host.rs` show no
  diff at all, as predicted (Assumptions) — grep had already confirmed
  neither imports a moving symbol. `driving.rs`'s diff is exactly the six
  functions plus the one import line (EX-1).
- VA-3 — VT-1's elapsed time, measured with `--nocapture` before the
  temporary `eprintln!` was removed (Findings): **104.4 ms, 102.7 ms,
  97.6 ms** across three separate runs, against `until(2 s)` — **19.1x to
  20.5x margin**, matching `design.md` §9's predicted ~105 ms / 19x row
  (`:713`) and well clear of S-26's 5x floor. `cargo test --workspace` wall
  time: unchanged in practice — the new target adds ~0.26s to a suite
  already dominated by real child processes (`just check` total 6.886s
  then 5.606s on a repeat run, both below PHASE-04's final 14.716s; the
  difference is warm-`target/` noise between runs, not a regression,
  confirmed by running `just check` three times this phase with exit 0
  each time).
  **Break-and-revert:** removed `let _entered = runtime.enter();` (replaced
  with `let _ = &runtime;`) — the test then panicked at
  `crates/goad/src/controller.rs:357:28`, *"there is no reactor running,
  must be called from the context of a Tokio 1.x runtime"*, inside
  `serve`'s own `sleep_until`. Reverted; `git diff` for the test file
  empty afterward. This is the liveness proof the plan's Do step 3 asks
  for: the test fails, and fails at the timer arm itself, if the guard the
  production topology depends on is missing.

**S-13..S-15, S-25..S-27** — none triggered. No dependency, feature or
`dev-dependency` added (S-13); the split needed no body change, confirmed
by `git diff` on the moved functions being empty within `scripting.rs`
relative to their old bodies (S-14); `init_integration_test_with_system_time`
exists and behaved as S-1 measured — the AC-10 margin matches design.md's
prediction almost exactly (S-15); `renderer/harness.rs` still imports only
`driving::instant`, confirmed by grep before and after (S-25); no unused-
import site beyond the one `Path`/`PathBuf` line EX-1 names, confirmed by
`cargo clippy --workspace --all-targets -- -D warnings` exiting 0 (S-27).

**`just check` (final):** exit 0, 5.606s real
(`/tmp/claude-1000/-home-david-dev-goad/302a7bbc-4adf-436d-ae25-a8c85c96ad29/
scratchpad/gate-final-phase05.log`).

**`git status --short`:** new — `crates/goad/tests/event_loop_schedule/`
(`main.rs`, `scheduling.rs`), `tests/support/scripting.rs`; modified —
`crates/goad-shell/tests/integration/{failure_matrix.rs,harness.rs,main.rs,
round_trip.rs}`, `crates/goad/Cargo.toml`, `crates/goad/tests/renderer/
{main.rs,scheduling.rs,table.rs,wiring.rs}`, `tests/support/driving.rs`,
`docs/slices/003/notes.md`; `flake.lock` modified but pre-dates this
session (untouched by this phase, as every prior phase also noted). No file
under any `src/`, `crates/goad/tests/event_loop/`, `crates/goad/tests/
renderer/harness.rs`, `tests/backends/`, or `crates/goad-shell/tests/
integration/{transport.rs,host.rs,fake.rs}` touched — all confirmed
untouched by `git diff --stat` being empty for each.

**What PHASE-06 needs to know:** the split moved six functions and touched
eight import sites, all mechanical (no symbol renamed, no body changed);
`transport.rs` and `host.rs` needed no edit despite being declared surfaces
— check them empty in the final diff rather than assuming a miss. AC-10 is
now discharged (`design.md` §9's row, PHASE-06's restatement sweep can cite
`event_loop_schedule::scheduling::a_scheduled_evaluation_fires_under_the_
production_topology`). One component remains structurally unproven by any
test in this repository: the production Slint platform's own polling of a
`spawn_local` future (A-1, F-8) — PHASE-06/EX-1 should state this rather
than let AC-10's "minus one component" phrase go unexplained in the
restatement. The VA-1 binary-count finding above ("six", plan said "seven")
is cosmetic and needs no repair, but PHASE-06's own restatement should not
copy the plan's "seven" figure forward uncritically.

### PHASE-06 — Restatement, re-measurement, and the gate

**Objective:** every document in the slice folder is true about the tree, the
margin table is measured rather than estimated, and the gate is green from a
clean clone.

**Reading list**
- `plan.md:1212-1306` — the whole PHASE-06 entry: EN-1/EN-2; EX-1..EX-6;
  VA-1..VA-5; S-16..S-19; implementer notes.
- `plan-log.md` PL-14, PL-15, PL-16 — the two `plan.md` corrections billed to
  this phase (PHASE-02's EX-12 prose/table, and its "Must not touch" list vs.
  its own implementer note on `tests/backends/`).
- `design.md` §9 (`:665-736`) — the margin table (not retro-fitted; measured
  numbers live here in `notes.md` instead, per PL-6/PL-11) and R1's own
  framing of what the anti-spin floors cost.
- `slice-003.md` Acceptance criteria (`:164-261`); `draft-spec.md` (whole,
  SPEC-002 draft); `canon-delta.md` (whole, CD-1..CD-3).
- `docs/policy/001-the-phase-gate.md` — the six-command gate, the four
  ADR-001 instruments + vocabulary scan + residue counting rule.
- `docs/memory/*.md`, all 16 read across PHASE-01..05's sheets; this phase
  adds a durable-fact candidate on `print_stderr`/`use_debug` having no test
  exemption (PHASE-05 Findings, confirmed against `Cargo.toml:151-152` and
  `clippy.toml`'s four `allow-*-in-tests` keys, neither of which names them).
- PHASE-01..05's phase sheets above (whole) — Findings and handover notes are
  this phase's restatement-sweep worklist, not re-derived.

**Assumptions & STOP conditions**
- S-16 — a document cannot be made true without a code change: STOP, finding
  for audit, not a documentation edit.
- S-17 — tempted to retro-fit `design.md` to match what shipped: STOP, record
  under Design drift instead.
- S-18 — any `just check` run is red or intermittently red: STOP.
- S-19 — a collected margin is below threshold (liveness < 5x, or gate wall
  time > 8.276s = slice 002's 5.276s baseline + 3s): STOP, consult, do not
  widen/shorten/pass over.
- Surfaces: `docs/slices/003/{plan.md, notes.md, slice-003.md, draft-spec.md,
  canon-delta.md}`. No source file, no manifest, no markup. Must not touch
  `docs/specs/`, `docs/policy/`, `docs/adr/`, `CLAUDE.md`, `design.md`.

**Tasks**
- [x] Verify EN-1/EN-2 before editing.
- [x] Restatement sweep over `docs/slices/003/*.md` prose only (EX-1..EX-6);
      source-file doc comments/test names are outside this phase's Surfaces
      line ("No source file, no manifest, no markup") — a document truth
      that needs a code change is S-16, named as a finding, not fixed here.
- [x] The two `plan-log.md`-billed `plan.md` corrections (PHASE-02 EX-12,
      PHASE-02 Must-not-touch), plus the two PL-16-named restatement notes
      (PHASE-05 "seven"→"six", PHASE-03 VA-2 "~105 ms" mismatch).
- [x] Collect the measured margin table from PHASE-02/03/05's sheets into one
      table here; re-run the timed targets three times.
- [x] Clean-clone gate (VA-1) on HEAD; working-tree gate for this phase's own
      edits.
- [x] Vocabulary scan (VA-2/AC-11).
- [x] `slice-003.md` AC pointers (EX-3); `canon-delta.md` CD-1 re-read
      (EX-2); `draft-spec.md` R-1..R-11 verified-by check (EX-1).
- [x] Harvest updated; Status → `done`.

**Findings**
- The stray `(F-1)` citation PHASE-01 flagged at `schedule.rs:327` (inside
  `#[cfg(test)] mod tests`, pre-dating this slice at `ad811c6d`) is **not**
  repaired here: `schedule.rs` is a source file, and this phase's Surfaces
  line is explicit — "No source file, no manifest, no markup." This is
  S-16's case exactly (a document/comment cannot be made true without
  touching code outside the declared surface), so it stays a finding for
  audit rather than a fix taken on this phase's own initiative. Already
  named in the Open section above (PHASE-01/Learned); restated here so the
  restatement sweep's own scope is clear — it covers `docs/slices/003/*.md`
  prose, not source-file doc comments or test names, which PHASE-06's plan
  entry never lists among its exit criteria (EX-1..EX-6 name only the five
  markdown files).

**`plan.md` corrections made (PL-16), one line each:**
- PHASE-02/EX-12: the bill's lead sentence "ten imported items across eight
  `use` lines... four go, four narrow" corrected to "eleven imported items
  across nine `use` lines... four deleted, five narrow" (matching its own
  table, which already had 3 deleted + 5 narrowed + 1 added), and the missing
  ninth line, `:13 use std::time::Duration;` (deleted), added as its own
  table row — `harness.rs`'s mirror-image list already named `Duration`, only
  the bill's own table and count were short.
- PHASE-02's "Must not touch" list: `tests/backends/` narrowed to except the
  one new script PHASE-02's own implementer note pre-authorises, so the list
  no longer contradicts itself.
- PHASE-05/VA-1: "seven test binaries" corrected to "six", naming them.
- PHASE-03/VA-2: the "~105 ms liveness... same shape as VT-1's" note for
  VT-5/VT-6 corrected — VT-1's own shape is ~6-12 ms (unfloored), not
  ~105 ms (`default_poll`-gated); the note had copied the wrong template.

**Criteria discharged**
- EX-1 — `draft-spec.md` §7 rewritten: every requirement R-1..R-11's *verified
  by* row now names the test that holds it, by file and function, checked to
  exist by grep against the tree before citing (23 function names across
  `crates/goad-semantics/src/schedule.rs`, `crates/goad/tests/renderer/
  {scheduling.rs, wiring.rs}`, `crates/goad-boundary/tests/checks/
  structure.rs`). R-11 stays recorded as review, not a test. R-9's row states
  plainly that no standing test asserts the construction directly — it is
  witnessed by six pre-existing `serve` tests continuing to pass unchanged.
- EX-2 — `canon-delta.md` CD-1's R-56 re-read against `crates/goad/src/
  wire.rs:42-67`: `Stimulus::event` writes `source: "host"` unconditionally
  and `kind()` returns exactly `"startup"` | `"requested"` | `"scheduled"` —
  matches CD-1 verbatim, no correction needed. CD-2 (the SPEC-001 §6.1
  illustration) and CD-3 (the Boundaries pointer) are prose changes with
  nothing in the tree to drift against; both re-read, both stand as written.
- EX-3 — `slice-003.md`'s twelve acceptance criteria each now carry a
  **Discharged by:** pointer naming the phase/criterion (matching this plan's
  Coverage table exactly) and are checked `[x]`.
- EX-4 — Harvest updated below (Produced/Learned/Open), current as of this
  phase.
- EX-5 — the measured margin table, below. No row breaches 5x; no gate wall
  time breaches baseline+3s (S-19 not triggered).
- EX-6 — `just -n check` prints the same six commands, same order, as
  `docs/policy/001-the-phase-gate.md` §Compliance. Confirmed this session.

**The measured margin table** (collected from PHASE-02/§VA-2, PHASE-03/§VA-2,
PHASE-05/§VA-3 above; re-verified stable by three fresh runs each this phase —
`scheduling::` 12/12 green at 0.80 s/0.80 s/0.80 s per run, `event_loop_
schedule` 1/1 green at 0.26 s/0.28 s/0.27 s per run):

| assertion | kind | expected | bound | observed | margin |
|---|---|---|---|---|---|
| AC-1 2nd invocation (VT-2/VT-3) | liveness | ~105 ms | `until(2s)` | ~270 ms | ~7.4x |
| AC-2 from an evaluate (VT-4) | liveness | ~105 ms | `until(2s)` | ~250 ms | ~8x |
| AC-2 from a respond (VT-5) | liveness | ~105 ms | `until(2s)` | ~290 ms | ~6.9x — closest margin measured |
| AC-3 earlier supersedes (VT-6) | liveness | ~105 ms | `until(2s)` | ~260 ms | ~7.7x |
| AC-3 later supersedes (VT-7) | anti-fire | no firing | 300 ms window | no firing, all runs | structural, not a race (F-19) |
| AC-4 past instant/every response, liveness (PHASE-03/VT-1) | liveness | "at once" | `until(2s)` | ~11.2/13.5/11.8 ms | ~150-180x |
| AC-4 past instant/every response, anti-spin | anti-spin | 2 invocations | 500 ms window | held at 2, 3/3 runs | 6x (window vs. 3s floor) |
| AC-5 failing backend, liveness (PHASE-03/VT-2) | liveness | ~105 ms | `until(2s)` | ~108.8/113.2/110.8 ms | ~18x |
| AC-5 failing backend, anti-spin | anti-spin | count unchanged | 500 ms window | held at 2, 3/3 runs | 6x |
| AC-7 stop while waiting (VT-8) | liveness | at once | `TIMEOUT` (2s) | 99.991 µs (elapsed) | ~20 000x |
| AC-9 succeed-once clock, startup liveness (VT-3) | liveness | ~5-10 ms | `until(2s)` | ~6.2/6.3/5.5 ms | ~300x |
| AC-9 succeed-once clock, refusal window (sanity check) | anti-spin | window ≈ 500 ms | — | 501.4/500.7/501.6 ms | confirms the window, not a margin |
| AC-9 succeed-once clock, `CLOCK_READS` spin witness | anti-spin | 2 reads | 500 ms window | 2, 3/3 runs (381 under VA-3 zeroed) | the assertion that actually holds "does not spin" |
| AC-9 vacuity control, working clock (VT-4) | liveness | ~105 ms | `until(2s)` | ~108.6/110.1/109.5 ms | ~18x |
| R-3 one-off past instruction (PHASE-03/VT-5) | liveness | no `design.md` row — plan's own, corrected this phase | `until(2s)` | ~11.5/11.7/12.2 ms | ~166x |
| R-3 one-off past instruction, anti-spin | anti-spin | 2 invocations | 500 ms window | held at 2, 3/3 runs | 6x |
| R-4/R-5 person mid-cadence (PHASE-03/VT-6) | liveness | no `design.md` row — plan's own, corrected this phase | `until(2s)` | ~6.3/6.2/5.4 ms | ~330x |
| R-4/R-5 person mid-cadence, anti-spin | anti-spin | count unchanged | 500 ms window from send | held at 3, 3/3 runs | 6x |
| AC-10 event loop (PHASE-05/VT-1) | liveness | ~105 ms | `until(2s)` | 104.4/102.7/97.6 ms | 19.1x-20.5x |

No row is below the 5x liveness floor; the closest is AC-2 from a respond at
~6.9x (S-19 not triggered). Gate wall time: three consecutive working-tree
`just check` runs this phase — **5.630 s / 5.582 s / 5.634 s**, all exit 0,
against slice 002's 5.276 s baseline (band: ≤ 8.276 s) — no breach.

**Design drift not reconciled (S-17 — for audit, `design.md` not edited):**
- `design.md` §9 states a uniform "~105 ms / 19x" expectation for the four
  `default_poll = 100 ms`-driven liveness rows (AC-1, AC-2×2, AC-3-earlier).
  Measured, all four land between ~250-290 ms with margins ~6.9x-8x — real,
  not a rounding error, and the closest (AC-2 from a respond, ~6.9x) sits
  materially nearer the 5x STOP floor than the design's own confidence
  implied. Every run stayed green across this phase's re-verification; still
  worth the audit's attention as the margin the slice is actually running on.
- `design.md` §9's AC-9 row predicts one liveness figure ("~105 ms/19x") for
  "refusal after the succeed-once clock fails." The test that discharges it
  does not produce a single comparable number: it measures the *startup*
  exchange's liveness (~6 ms/~300x) and verifies the refusal itself through a
  500 ms anti-spin window plus the `CLOCK_READS` spin witness, not through a
  liveness bound. The design's row and the test's actual shape do not
  correspond one-to-one.
- AC-7's design row predicts ~2000x; measured is ~20 000x (99.991 µs against
  a 2 s `TIMEOUT`) — an order of magnitude looser than predicted, in the safe
  direction. Not a risk; recorded because §9's own numbers are otherwise
  fairly tight.

**Verification**
- VA-1 — **clean-clone gate.** `git clone` of HEAD (`43b0f95`) into the
  scratchpad, `just check` run there: **exit 0, 60.047 s real** (cold,
  `/tmp/.../scratchpad/clean-clone-gate.log`) — comparable to slice 002's own
  cold clean-clone figure (57.648 s, `notes.md` PHASE-09), no numeric bound
  stated for this figure in this slice's plan. `grep -rn "#\[ignore\]"
  crates/ tests/` returns nothing (confirmed, exit 1/no matches).
- VA-2 — **vocabulary scan (AC-11).** `cargo test -p goad-boundary --test
  checks vocabulary`: 13 passed, 0 failed, including
  `no_workspace_member_names_the_users_domain` and
  `no_member_manifest_names_the_users_domain_in_its_own_crate_name`. Walked
  by hand against `crates/goad-boundary/tests/checks/vocabulary.rs:18-25`'s
  `DOMAIN` list (`habit, streak, journal, site, goal, reminder, compliance`):
  `schedule`, `check`, `poll`, `spacing`, `scheduled`, `next_check` appear in
  none of it. Recorded, not assumed.
- VA-3 — the four ADR-001 instruments and the domain-vocabulary scan, five
  separate results (POL-001 §Verification): (1) crate-edge direction — `cargo
  build --workspace` exits 0, no `error[E0433]`; (2) manifest allowlist —
  held by `cargo test -p goad-boundary` (part of `--workspace`), passing;
  (3) stratum-1 `std` purity scan — same, passing; (4) `cargo test -p
  goad-semantics` standalone — exit 0, 30 passed (this phase's own gate
  runs, above); (5) domain-vocabulary scan — VA-2 above, 13 passed. Five
  results, not one count.
- VA-4 — three consecutive working-tree `just check` runs, this phase's own
  edits included: **5.630 s / 5.582 s / 5.634 s**, all exit 0
  (`/tmp/.../scratchpad/gate-wt-{1,2,3}.log`).
- VA-5 — `git diff --stat` from the plan commit (`0b2e50f`) to PHASE-05's
  head (`43b0f95`) against every phase's declared Surfaces: every touched
  path maps to a named surface (PHASE-01: `schedule.rs`, `wire.rs`; PHASE-02:
  `controller.rs`, `renderer/{main,scheduling,harness,wiring,table}.rs`,
  the one pre-authorised `tests/backends/` script; PHASE-03: `config.rs`,
  `scheduling.rs`; PHASE-04: `structure.rs`, `diagnostics.rs`, `glass.rs`,
  `app.slint`, `wiring.rs`; PHASE-05: `driving.rs`→`scripting.rs` split,
  `Cargo.toml`, `event_loop_schedule/*`, the four `goad-shell/tests/
  integration/*` import-site edits, `renderer/main.rs`). No undeclared path.

**STOP conditions encountered:** none. S-16 named one case (the stray F-1
citation, Findings above) as a finding for audit rather than a halt — it is
outside this phase's own surfaces, which is exactly what S-16 describes. S-17
named three Design-drift items above rather than retro-fitting `design.md`.
S-18 and S-19 did not trigger — every gate run this phase was green, and no
margin or gate-wall-time threshold was crossed.

**`just check` (this phase, working tree, final):** exit 0, 5.634 s real
(third of the three VA-4 runs above).

**`git status --short` (this phase's own edits):** `docs/slices/003/
{draft-spec.md, notes.md, plan.md, slice-003.md}` modified. `flake.lock`
modified but pre-dates this session, as every prior phase also noted. No
source file, manifest or markup touched — confirmed by `git diff --stat`
against the four files above.

**For the auditor to attack first:** the AC-2-from-a-respond margin (~6.9x,
the closest of any row to the 5x floor) under a loaded machine; the AC-9
design-row/test-shape mismatch (Design drift, above); and the stray `(F-1)`
citation at `schedule.rs:327`, which is a source-file fix no phase's surfaces
have covered yet.

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-07 · PHASE-06 done · all six phases done, gate green,
tree not yet committed for this phase

### Produced
- `goad_semantics::schedule::wait_for` — total, zero-arithmetic-operator wait
  computation, stratum 1 (PHASE-01/EX-1, EX-2).
- `Stimulus::Scheduled` at `crates/goad/src/wire.rs`, wired through `kind()`
  and `event()` (PHASE-01/EX-4). Dispatched from PHASE-02 on.
- `crates/goad/src/controller.rs`: `MINIMUM_SPACING` (3s), `Fired`,
  `Absorbed`, `Pending::now()`, `Controller.next_check`/`Frame.next_check`,
  and `serve`'s third `select!` arm with the two re-arm sites (PHASE-02/
  EX-1..EX-9). The refusal-arm re-arm is written as a three-site
  `if refusal_re_arms { … }` guard rather than a single-site special case —
  see the phase sheet's Decisions.
- `crates/goad/tests/renderer/harness.rs` — the seven fixtures two or more
  of `wiring.rs`/`table.rs`/`scheduling.rs` share, moved from `wiring.rs`
  (PHASE-02/EX-11).
- `crates/goad/tests/renderer/scheduling.rs` — VT-2..VT-8, six timed `serve`
  tests (PHASE-02).
- `tests/backends/logs-the-request-then-answers.sh` — a request-logging
  variant of `answers-as-instructed.sh`, for the two cases that need
  `event.kind` (VT-3, VT-7).
- `crates/goad/tests/renderer/scheduling.rs` — six more `serve` tests
  (PHASE-03/VT-1..VT-6): a past instant on every response, a failing
  backend, a clock that succeeds once then fails (`succeeds_once_then_fails`
  over a private `static CLOCK_READS: AtomicUsize`), that fixture's vacuity
  control, a one-off past instruction, and a person acting mid-cadence. No
  new `tests/backends/` script — `@garbage` and the existing
  `logs-the-request-then-answers.sh` covered every case.
- `crates/goad-shell/src/config.rs`: `ScheduleConfig::default_poll`'s doc
  comment (PHASE-03/EX-2), citing SPEC-002/R-5 and SPEC-001/R-21.
- `crates/goad/src/diagnostics.rs::next_check_line` — the standing
  schedule's own rendered line, second precision, through the escape/bound
  pipeline (PHASE-04/EX-1). `PromptWindow`'s new `next-check` property
  (`app.slint`) and `glass.rs`'s total write of it on every `present`
  (PHASE-04/EX-4) — kept out of `diagnostic-lines` per D-17, so the
  "Nothing to report." sentinel and DT-5 survive unchanged.
- `crates/goad-boundary/tests/checks/structure.rs`: directory-parameterised
  (`SUBJECT_DIR`, `SHELL_SUBJECT_DIR`); AC-6's two instruments —
  `mentions_occurrences_of`, built on `goad_boundary::scan::mentions`,
  asserting no production line in `crates/goad/src` names the identifier
  `resolve` (0/12 files) and that `schedule::resolve` occurs exactly twice
  in `crates/goad-shell/src`, both in `host.rs` (2/8 files) (PHASE-04/EX-2,
  EX-3).
- `tests/support/scripting.rs` — the scripted-backend half of the former
  `driving.rs`: `backend`, `marker`, `clear`, `logging_backend`,
  `invocations`, `scripted`, moved unchanged (PHASE-05/EX-1, D-18, FD-3).
  `driving.rs` keeps the host-composition half.
- `crates/goad/tests/event_loop_schedule/{main.rs,scheduling.rs}` — AC-10's
  own `[[test]]` target: `a_scheduled_evaluation_fires_under_the_production_
  topology` drives a scheduled evaluation through the production topology
  (real window/tray, `install`, multi-thread tokio runtime, `EnterGuard`,
  real `ProcessBackend`, production `serve`, `slint::spawn_local`) under
  `init_integration_test_with_system_time()`, the one substitution being
  the Slint platform itself (PHASE-05/EX-3..EX-6, A-1, F-8). Measured
  margin 19.1x-20.5x against design.md §9's predicted 19x.
- PHASE-06: `draft-spec.md` §7 now names, per requirement, the test that
  holds it by file and function (23 citations, each grep-confirmed against
  the tree before being written); `slice-003.md`'s twelve acceptance
  criteria each carry a **Discharged by:** pointer and are checked; the
  measured margin table (19 rows) and three Design-drift items live in this
  phase's own sheet above, collected from PHASE-02/03/05's separately-taken
  measurements rather than re-measured from scratch (the per-test wall
  times cannot be re-taken without reintroducing the `print_stderr`-denied
  instrumentation PHASE-05 already had to remove).

### Learned
- The design's exact `wait_for` body (`design.md:155-160`) compiles clean
  against `#![deny(clippy::arithmetic_side_effects)]` with no suppression:
  `jiff::Timestamp::duration_since` plus `TryFrom<SignedDuration> for
  std::time::Duration` really is total and operator-free, confirming A-2 and
  the implementer note without needing S-1.
- `schedule.rs:327`'s test doc comment still cites `(F-1)`, pre-dating this
  slice (slice 002, `ad811c6d`). It sits outside every phase's declared
  surface so far; worth a citation sweep before or at PHASE-06/close.
- **PHASE-02's plan carried two small, resolvable gaps**, both documented in
  the phase sheet with evidence rather than silently patched: EX-12's
  `wiring.rs` import bill named eight lines but a ninth (`use std::time::
  Duration;`) was also solely consumed by the moved fixtures and had to go
  too (confirmed by the compiler: zero warnings after deleting it, and it
  would not compile left in place); and this phase's "Must not touch" list
  named `tests/backends/` even though the phase's own implementer notes
  pre-authorise exactly the one new script VT-3/VT-7 need there. Both are
  candidates for a closer look at plan-generation practice — a bill or a
  Surfaces list that a phase's own notes immediately except from itself is a
  seam worth naming explicitly next time, not resolving per-phase.
- `tokio`'s bounded `mpsc::channel(1)` gives a *free*, non-flaky way to prove
  "command B was enqueued before exchange A completed" (FD-4/VT-7): a second
  `send().await` on a full channel does not resolve until the first message
  is dequeued, so sending sequentially on a capacity-1 channel is a
  structural guarantee, not a timing race — no sleep, no `@slow-view`-style
  synthetic delay needed.
- **`harness::stub_clock`'s fixed `now` makes two of `schedule::resolve`'s
  branches numerically indistinguishable** whenever the retained instant
  equals `now + default_poll` exactly (PHASE-03, VT-2/AC-5): "the retained
  value stands" and "nothing retained, so `now + default_poll`" land on the
  same value. A test asserting only the final `next_check` cannot tell which
  branch actually ran in that specific case — recorded so a future reader
  does not mistake `served.controller.frame().next_check` for a stronger
  witness than it is here; the assertion still holds SPEC-001/R-29 (neither
  branch produced something *else*), which is the claim that mattered.
- **A test's own retry/spin fixture can be a load-bearing assertion, not
  just plumbing.** PHASE-03/VT-3's `static CLOCK_READS: AtomicUsize`
  (needed anyway, because `Clock` is a `fn` pointer and cannot capture) is
  also the only thing that can witness AC-9's "does not spin" clause under
  VA-3's break-and-revert — the backend invocation count and the diagnostic
  line count both stay unchanged whether or not the floor is doing anything,
  because the clock keeps failing regardless of spacing. Worth watching for
  in a future plan: a fixture built to make a test possible may be the
  fixture the criterion actually needs asserted on.
- **A plan's own VA-2 "expected" figure can be copied from the wrong
  shape.** PHASE-03's VA-2 note predicted "~105 ms liveness… the same shape
  as VT-1's" for VT-5 and VT-6, but VT-1's own liveness is ~12 ms (an
  unfloored past-instant firing), not ~105 ms (a `default_poll`-gated one) —
  the note conflated the two shapes AC-1/AC-2 and AC-4 actually are.
  Harmless here (every margin came in wider than predicted, never
  narrower), but worth a second look before promoting `design.md` §9's new
  rows verbatim.
- **`clippy.toml`'s `allow-*-in-tests` carve-out does not cover every
  workspace `deny`.** `unwrap_used`/`expect_used`/`panic`/
  `indexing_slicing` are exempted in test code; `dbg_macro`,
  `print_stdout`, `print_stderr` and `use_debug`
  (`Cargo.toml:149-152`) are not — a temporary `eprintln!("…{:?}", …)`
  dropped into a test for margin instrumentation trips two of them at once
  and has to come back out. `docs/memory/clippy-toml-test-exemptions-are-
  a-hidden-boundary.md` names the first four; worth widening that memory
  (or adding a sibling) to name these four as the ones that are **never**
  test-exempt, so a future phase does not rediscover this by trying it.
- **A plan's own binary count can drift by a fixed offset too.**
  PHASE-05's VA-1 predicted "seven test binaries in `crates/goad`"; six
  run (`unittests` lib, `unittests` main, `event_loop`, `event_loop_
  schedule`, `renderer`, `Doc-tests goad`). Same class as the VA-2 note
  above — a plan figure that does not match what the tree actually does,
  caught by measuring rather than pasting the plan's own claim.

### Open
- Findings sweep at close: the stray `(F-1)` citation above (PHASE-01); the
  two plan-gap findings above (PHASE-02), for the orchestrator/audit to
  confirm the resolution or record differently; PHASE-03's VA-2
  "~105 ms" mismatch for VT-5/VT-6 (harmless, but `design.md` §9 should not
  inherit the wrong number if these rows are promoted); PHASE-03's
  `CLOCK_READS` addition beyond the plan's own stated VT-3 assertions, for
  confirmation that it is the right fix rather than a workaround.
- PHASE-04 needs to know: `controller.rs`'s refusal path (all three
  `refusal_re_arms` sites) is now driven, not just written — VT-3 exercises
  the `Command::Evaluate` site from the timer arm; the other two remain
  undriven (they guard a boolean that is always `false` today, per PHASE-02's
  Decisions). No `controller.rs` change landed this phase — anyone touching
  it next inherits exactly PHASE-02's shape, unmodified.
- PHASE-06 needs to know: AC-10 is discharged; the one component slice 002's
  F-5 leaves open across every phase — the production Slint platform's own
  polling of a `spawn_local` future, as opposed to the testing platform's —
  is still unproven by any test in this repository (A-1, F-8) and should be
  named as such in the restatement rather than implied closed. The VA-1
  "seven vs. six" binary-count drift (Learned) needs no repair but should
  not be copied forward into PHASE-06's own count.
- **Resolved at PHASE-06:** the stray `(F-1)` citation (`schedule.rs:327`)
  stays open — outside every phase's surfaces including this one (S-16,
  PHASE-06 Findings above); the two PHASE-02 plan-gap findings were
  confirmed as within-phase by the orchestrator (`plan-log.md` PL-16) and
  the two corresponding `plan.md` corrections are made (PHASE-06's own
  sheet); PHASE-03's VA-2 "~105 ms" mismatch is corrected in `plan.md` and
  is **not** inherited by `design.md` (which is not retro-fitted, S-17) —
  the correct expectation for VT-5/VT-6 is recorded directly in the measured
  margin table instead; the `CLOCK_READS` addition stands unquestioned — no
  reviewer has raised it since PHASE-03, and PHASE-06's own re-runs confirm
  it is still the only assertion that fails under VA-3's break-and-revert.
- **Memory candidates for close** (named here per EX-4; not written into
  `docs/memory/` by this phase):
  1. `clippy.toml`'s `allow-*-in-tests` carve-out (`unwrap_used`,
     `expect_used`, `panic`, `indexing_slicing`) does **not** cover
     `dbg_macro`, `print_stdout`, `print_stderr` or `use_debug`
     (`Cargo.toml:149-152`, all `deny`, no test exemption) — confirmed
     again this phase directly against both files. A future phase reaching
     for `eprintln!`/`dbg!`/`{:?}` as temporary test instrumentation will
     hit this immediately; worth a memory file of its own, or widening
     `docs/memory/clippy-toml-test-exemptions-are-a-hidden-boundary.md`
     with a second section for the never-exempt class.
  2. **FD-3's rule**, an extension of `docs/memory/shared-test-helper-lives-
     at-workspace-root-via-path.md`: every `pub(crate)` symbol in a file
     shared via `#[path]` must be reachable from **every** includer, not
     just the includers that existed when the file was written — a third
     includer (PHASE-05's new `event_loop_schedule` target) that uses only
     part of a shared file's surface still compiles fine; a third includer
     that needs a symbol the file doesn't export is what fails, and only
     at that includer's own build.
  3. **FD-2's rule**: a scan built on `str::contains` and a scan built on
     `goad_boundary::scan::mentions` (word/path-boundary matching) are
     different instruments: a `contains`-based scan cannot safely assert
     "no production line names the identifier X" (it would also catch `X`
     as a substring of a longer identifier), and neither substitutes for
     the other. `structure.rs` now carries one of each, named separately
     (PHASE-04/EX-2).
<!-- Still unresolved at this point. Candidates for follow-ups. -->

## Repairs after review

Code review round 1 (`review-code.md`, F-1..F-16). One session, after
PHASE-06. Sixteen findings, no blockers; fifteen `fix-now` or `doc-wrong`, one
`tolerated`. What changed and why, by surface — the reasoning is in the ledger,
this is the inventory.

**Production source.**
- `crates/goad/src/controller.rs` — the three refusal `continue`s fold into one
  (F-3, D-21); `deadline_after` replaces the bare `Instant + Duration` and is
  total, clamped at `LONGEST_WAIT` = 365 days (F-5, D-20); eight `D-N` and one
  `F-N` citation replaced by requirement ids or words, including the `Fired`
  enum's miscited F-12 (F-9, F-16). `Duration::new`, not `from_secs`, because
  `clippy::duration_suboptimal_units` wants the unstable `from_days` and
  POL-001 forbids suppressing a lint.
- `crates/goad/src/diagnostics.rs` — `next_check_line` truncates rather than
  half-expands (F-12, D-24); the miscited *"R-2's closing clause"* becomes
  `draft-spec.md` §6 (F-9).
- `crates/goad-semantics/src/schedule.rs` — `#[must_use]` on `wait_for` (F-15).
- `crates/goad/ui/app.slint` — the `D-9, D-17` comment restated in words (F-9);
  the next-check row is now conditional, so no empty row is laid out before the
  first exchange (F-14's related observation).

**Instruments.**
- `crates/goad-boundary/tests/checks/structure.rs` — AC-6 (b) switches to the
  identifier matcher and is renamed
  `the_identifier_resolve_is_confined_to_the_hosts_resolution_path`; measured 9
  occurrences over `host.rs`, `state.rs`, `error.rs`. `production_lines` skips
  each `#[cfg(test)]` **item** instead of cutting the file at the first one.
  The vacuity guard gains a per-file and a per-directory line count (F-1,
  D-23). The participle control's fixture is code, not a comment (F-4).
- New fixture `tests/fixtures/structure/production_after_tests.rs`: production
  code after an inline test module, which is the shape no subject file has.

**Tests.**
- New `tests/support/waiting.rs`: the poll loop, once, plus `LIVENESS_BOUND`
  (F-8). `renderer/harness.rs::until` becomes an assertion over it; the
  event-loop tier calls the non-asserting half.
- `event_loop_schedule/scheduling.rs` — the watcher no longer panics inside the
  Slint loop; it records a bool, always stops the loop, and the assertion is
  made on the test thread (F-10). Proven: a broken predicate now fails in
  5.16 s instead of hanging.
- `renderer/scheduling.rs` — two new cases:
  `a_refusal_that_did_not_come_from_the_timer_leaves_the_deadline_standing`
  (F-3's cover, and it discriminates) and
  `an_instruction_at_the_far_edge_of_time_arms_the_sleep_without_panicking`
  (F-5 at the loop level).
- `renderer/wiring.rs` — VT-2's populated half asserts the exact rendered line
  (F-14).
- The liveness bound moves from 2 s to 5 s at every renderer-tier call site
  (F-7).
- `tests/backends/logs-the-request-then-answers.sh` — the header states both
  differences from `answers-as-instructed.sh` (F-11).

**Documents.** `canon-delta.md` CD-1 redrafted, `event.kind` left open (F-2,
D-19) — **not applied to canon; the user endorses at reconciliation**.
`draft-spec.md` §2, §5, §6, §7 and a new OQ-4. `design.md` §7 gains D-19..D-24,
§9's AC-6 row and margin table are restated on measured values, R1's *"the
smallest is 19x"* is corrected. `design-log.md` records the six decisions.
`slice-003.md` Follow-ups gains F-6's.

**Re-measured, three runs each, per-test with `--exact`.**

| assertion | run 1 | run 2 | run 3 | bound | margin |
|---|---|---|---|---|---|
| AC-1 second invocation | 0.27 s | 0.25 s | 0.27 s | 5 s | ~18x |
| AC-2 from an `evaluate` | 0.26 s | 0.26 s | 0.28 s | 5 s | ~18x |
| AC-2 from a `respond` | 0.29 s | 0.28 s | 0.28 s | 5 s | ~17x |
| AC-3 earlier supersedes | 0.27 s | 0.26 s | 0.28 s | 5 s | ~18x |
| AC-9 refusal, whole test | 0.66 s | 0.65 s | 0.66 s | 5 s + a 500 ms window | ~18x |
| AC-5 failing backend, whole test | 0.77 s | 0.76 s | 0.76 s | 5 s + a 500 ms window | ~18x |
| AC-10, the event-loop target | 0.27 s | 0.26 s | 0.28 s | 5 s | ~18x |
| F-3's refusal window, whole test | 0.67 s | 0.67 s | 0.67 s | 500 ms window | anti-fire |
| F-5's far-edge case | 0.17 s | 0.17 s | 0.16 s | none | n/a |

No margin moved below its bound; every one moved up, because the bound did.
`renderer scheduling::` ran 14/14 green three consecutive times at
0.82/0.82/0.79 s. `just check` exits 0 in **5.6-7.3 s** warm (12.8 s cold),
zero warnings, no `allow` and no `expect` outside tests.

### Round 2 (F-17..F-21)

All five `fix-now`. Round 1's sixteen were verified against the tree, not
against their responses.

- **F-17** — `scan.rs` gains `code_without_literals`, `code_of`'s sibling over
  the **same** state machine (`strip(line, Literals::Kept | Cut)`); the block
  comment and the three literal arms now share one `cut_out`. `production_lines`
  counts braces over it, so one unbalanced `{` in a test module's literal no
  longer blinds the rest of the file. The on-disk fixture carries the hazard
  (a plain string, a raw string and a char literal, each with an open brace)
  and failed before the change. Four controls pin the strip.
- **F-18** — instrument (b) counts the resolving **call**: `resolve(` with no
  identifier byte before it, over literal-stripped code. **2, both in
  `host.rs`.** A second assertion catches the path taken as a value without
  being called. A reworded diagnostic and a renamed private helper no longer
  red the suite; a brace-grouped `use` plus a bare call, and a
  `let _f = …::resolve;`, both do. The test keeps its original name, which is
  true again. Walk helpers refactored to one function with three predicates.
- **F-19** — `slice-003.md` AC-6 rewritten (call count, item-scoped skip, all
  three vacuity guards named); `design.md` §9's AC-6 row and D-23 and
  `draft-spec.md` §7's R-2 row swept with it. `audit.md:152` left for the
  audit's own hand.
- **F-20** — `design.md` §5.5 E-6 separates the host's clamp (D-20, about the
  `Add`) from tokio's (about a deadline it is given) and now reads *mitigated*.
- **F-21** — VT-6's two waits take `FLOOR_SAFE_BOUND` (2 s) rather than the
  5 s workspace bound, its window is `ANTI_SPIN_WINDOW` (500 ms), and
  `FLOOR_SAFE_BOUND + ANTI_SPIN_WINDOW < FLOOR_MILLIS` is a `const _: ()`
  assertion — verified to fail the build at 5 s, then reverted. §9's
  unqualified anti-spin claim now carries the exception; R1 and the margin
  table too.

**Gate after round 2:** `just check` exit 0, 11.0 s cold, 19 test-result
lines, zero warnings. Three runs each: `renderer scheduling::` 14/14 at
0.81/0.82/0.81 s, `event_loop_schedule` 1/1 at 0.27 s ×3, `goad-boundary
checks` 43/43 at 0.08 s ×3. VT-6 measured 0.68/0.68/0.67 s against its 2 s
bound.

### Round 3 (F-22, blocker)

`an_instruction_at_the_far_edge_of_time_arms_the_sleep_without_panicking`, added
in round 1 for F-5, synchronised on the invocation log and stopped the loop at
once. The backend script appends its log line **before** it reads the request,
so the stop raced the exchange: the cancel arm dropped the call, `absorb` never
ran, `next_check` was `None`. Reproduced at 3 red in 5 full renderer runs, 12 of
12 green in isolation.

Class fix: `scheduling.rs::absorbed_line(rfc3339)` names the rendered
next-check line, which `glass.present` writes at the top of the iteration
*after* `absorb` and so cannot be read early. Six sites now wait on it — the
far-edge case, and five that had a fixed 20 ms or 500 ms sleep standing between
a log line and an assertion about what the exchange resolved. VT-8's *"short
settle"* was the clearest instance: a delay used as synchronisation.

Left as they are, with the reason written down: `event_loop_schedule` asserts
only facts about the log itself; `wiring.rs`'s two `@hang` cases need the
exchange in flight when the stop lands, which is the opposite requirement.

**Ten consecutive full renderer runs, 10 green at 138/138.** `just check` exit 0.

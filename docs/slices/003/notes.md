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
| PHASE-04 — What the person sees, and what the scan holds | pending | |
| PHASE-05 — The topology | pending | |
| PHASE-06 — Restatement, re-measurement, and the gate | pending | |

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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-07 · PHASE-03 done · tree not yet committed for this
phase

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
<!-- Still unresolved at this point. Candidates for follow-ups. -->

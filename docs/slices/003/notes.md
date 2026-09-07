# Notes — Slice 003

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — The arithmetic, and the third stimulus | done | 2026-09-07 |
| PHASE-02 — The wait, and the cadence it keeps | done | 2026-09-07 |
| PHASE-03 — What the floor bounds, and what a failure does not stop | pending | |
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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-07 · PHASE-02 done · tree not yet committed for this
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

### Open
- Findings sweep at close: the stray `(F-1)` citation above (PHASE-01); the
  two plan-gap findings above, for the orchestrator/audit to confirm the
  resolution or record differently.
- PHASE-03 inherits the `if refusal_re_arms { sleep.as_mut().reset(floor_until);
  }` guard at all three refusal sites in `controller.rs::serve` (only the
  `Command::Evaluate` one is reachable from the timer arm today, per the
  implementer note) — worth knowing before touching that function again.
<!-- Still unresolved at this point. Candidates for follow-ups. -->

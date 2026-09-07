# Plan — Slice 003: Scheduling — the timer that turns a resolved instant into an evaluation

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

<!-- Phase ids (PHASE-NN) and criterion ids (EN-/EX-/VT-/VA-/VH-N) are
     immutable: edits append, never renumber, so the sequence goes
     non-monotonic after a split and that is expected. Criterion ids are local
     to their phase — cite another phase's phase-qualified (PHASE-03/EX-2).
     Verification modes — VT: automated test. VA: agent check. VH: human
     acceptance.
     Progress is NOT recorded here. Status lives in `notes.md`. -->

## Overview

Six phases turn a resolved instant into an evaluation. They run **01, 02, 03,
04, 05, 06** — no reordering, no parallelism, because every phase from 02
onward reads or edits `crates/goad/src/controller.rs`.

The spine is the design's own three-layer claim (§4, P-3): **prove the
arithmetic where it is free, prove the loop where it is cheap, prove the
topology once.**

- **PHASE-01** lands the two leaf changes nothing else can start without: the
  pure `wait_for` in stratum 1, and the third `Stimulus` variant in stratum 3.
  Neither depends on the other; both are prerequisites of PHASE-02, and neither
  fills a session alone.
- **PHASE-02** is the slice. One `select!` arm, one monotonic floor anchor, one
  `Absorbed` return value, two re-arm sites — and the six timed assertions that
  drive them: the cadence, both supersession directions, an instruction from
  each entry point, and a stop issued while the loop is waiting on the timer.
  It also pays the mechanical price of `Absorbed`: **22 call sites** in
  `renderer/{wiring.rs, table.rs}` read `absorb`'s return value and stop
  compiling when its type changes, and the seven fixtures the new module needs
  are private to `wiring.rs` and move to a target-local `harness.rs`. Both are
  declared surfaces, not trespass (F-1, F-2).
- **PHASE-03** attacks the same mechanism from the failure side: a backend that
  instructs the past on every response, one that instructs it **once**, a
  backend that fails every invocation, a clock that succeeds once and then
  fails, and a person acting in the middle of a scheduled cadence. These are the
  anti-spin windows and they are where the slice's unavoidable wall time is
  spent.
- **PHASE-04** is what a person sees and what a scan holds: one diagnostic line,
  and AC-6's two instruments in `goad-boundary`.
- **PHASE-05** is AC-10 — the new `[[test]]` target that runs the timer under a
  real Slint event loop, a multi-thread runtime and an `EnterGuard`, with only
  the Slint platform substituted.
- **PHASE-06** is the restatement sweep, the re-measured margin table, and the
  clean-clone gate, as slice 002's PHASE-09 was.

Four things hold for every phase.

1. **The gate is POL-001's six commands.** A phase is not green until `just
   check` exits 0. Every phase's VA-1 is the gate's pasted output, not a claim
   that it would pass.
2. **`draft-spec.md` (SPEC-002) and `canon-delta.md` are the slice's working
   authority** (`docs/AGENTS.md:36`). Phases cite them exactly as they would
   canon. **No phase writes into `docs/specs/`, `docs/policy/` or `docs/adr/`,
   and no phase edits `CLAUDE.md`.** CD-1, CD-2, CD-3 and SPEC-002's promotion
   all land at audit, with user endorsement.
3. **Code cites requirement ids, not finding ids**
   (`docs/memory/cite-requirements-not-finding-ids.md`). A comment that needs to
   say why cites `SPEC-001/R-28` or `SPEC-002/R-4`, never `F-12` or `D-3`.
4. **Absence is never asserted alone.** Every anti-spin and anti-fire window in
   this slice is paired with a liveness assertion over the same mechanism, and
   AC-6's two scans each carry a vacuity guard. A window that would pass against
   a timer that never fires at all is not evidence
   (`docs/memory/a-bound-is-not-tested-at-the-bound.md`).

## Sequencing & rationale

**Why the arithmetic and the stimulus come first, and together.** Stratum 1's
`wait_for` is the only thing the loop computes with, and stratum 1 has its own
gate column (`cargo test -p goad-semantics`); landing it alone gives PHASE-02 a
dependency that is already green in both columns. `Stimulus::Scheduled` is three
lines and one match arm and is what the timer arm dispatches; it cannot land
with the loop without the loop's first commit also being the commit that changes
the wire vocabulary. Neither is a session's work. Together they are one small,
fast phase whose exit is a clean entry criterion for the heavy one.

**Why the whole loop mechanism is one phase.** `serve`'s first `select!`, the
floor anchor, the two re-arm sites and the conditional re-arm after a refused
scheduled firing are one function body. Splitting them across phases means
editing the same twenty lines twice and leaving a branch nothing drives in
between — the failure
`docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` records. So
PHASE-02 lands all of it, and lands the assertions that drive every branch of
it except the refusal branch, which PHASE-03 drives.

**Why the refusal branch waits for PHASE-03 and the mechanism does not.** The
conditional re-arm is two lines inside PHASE-02's own `match`; writing it in
PHASE-03 would reopen PHASE-02's surface for a two-line edit. What PHASE-03
adds is the evidence: AC-9's succeed-once clock is the only fixture that reaches
that branch, and it belongs with AC-4 and AC-5 because all three are anti-spin
windows and all three are about what the floor bounds. PHASE-02's exit therefore
carries the branch as **written and reviewed but not yet driven**, and says so;
PHASE-03/EN-2 is the criterion that closes it.

**Why the timed tests split 6 / 6 rather than 12 / 0.** Twelve `serve` tests with
a `LocalSet`, a real child process and a wall-clock bound is more than one Sonnet
session with bookkeeping, and slice 002's PHASE-10 — `serve` plus six such
tests — is the calibration. The split falls where the fixtures differ: PHASE-02's
six all use a well-behaved scripted backend and a working clock; PHASE-03's six
each need a misbehaving backend or a broken clock, and one of them needs a
`static`-backed clock fixture that does not exist yet. PHASE-03's six are three
fixtures, not six: VT-1, VT-5 and VT-6 are three instruction lists against one
past-instant shape, and VT-3/VT-4 are one clock fixture read twice.

**Why the diagnostic line and the boundary scans share a phase.** Neither is
large; both are the slice's *structural* obligations rather than its behavioural
ones; and neither touches `serve`. PHASE-04 is the only phase after PHASE-01
that does not read the loop, which makes it the one that could in principle be
reordered. It is kept fourth because it renders `Frame::next_check`, which
PHASE-02 lands.

**Why AC-10 is last but one.** It composes everything: the loop, the stimulus,
the real transport, a real window and tray, `spawn_local` under an `EnterGuard`.
Running it before PHASE-03 would mean debugging the topology and the mechanism
at once. It is also the phase that pays FD-3's cost — the shared test helper
splits so the new target can include only what it uses — and that is work best
done against a tree where nothing else is in flight.

**Reordering and parallelism.** PHASE-04 could run any time after PHASE-02.
Nothing else moves: 01 → 02 → 03 is a dependency chain, 05 needs 02 and 03, and
06 needs everything. **No two phases may run in parallel:** 02, 03 and 05 all
read or write `crates/goad/src/controller.rs`, and 03, 05 and 06 all write
`docs/slices/003/notes.md`.

**Size.** PHASE-02 is the heavy one and is heavy for a stated reason. PHASE-01 is
the lightest. PHASE-03 and PHASE-05 are medium; PHASE-04 and PHASE-06 are
light-to-medium. **PHASE-02 is not split**, and the call-site migration F-1
uncovered is why the question was reopened rather than why it changed: the 22
edits are uniform — `.shift` appended, no import added, `Shift` already in
scope at every assertion — and the fixture lift is a seven-item move plus one
`use` line, because a child module's `super::X` already resolves through a
private parent import (`wiring.rs:706` proves it in the tree today). Neither is
re-reading, which is what a session's budget is actually spent on. Splitting
after EX-9 would buy a phase whose exit is "a mechanism with one test", which
PL-1 rejected for good reason. If a phase overruns a session, that is a finding for `notes.md`
and a `PARTIAL` checkpoint, not a reason to skip the sheet
(`docs/memory/stop-letter-vs-purpose-is-a-plan-log-adjudication.md`).

## Decisions taken during planning

Recorded in `plan-log.md` with reasoning and rejected alternatives, under the
standing autonomy grant (`design-log.md`, 2026-09-07). Summarised here because
each one decides where a phase boundary falls or what a phase may touch.

- **PL-1** — six phases, in the order and split above.
- **PL-2** — the renderer tier's scheduling tests get their **own module**,
  `crates/goad/tests/renderer/scheduling.rs`, not a ninth `mod` inside
  `wiring.rs` (already 1232 lines).
- **PL-3** — the per-test `default_poll` is built in the renderer target from
  `goad_shell::config::{Config, BackendConfig, ScheduleConfig}` and handed to
  `driving::host_from`, which already exists and is already reachable from both
  including targets. **Nothing is added to `tests/support/driving.rs`** for it.
- **PL-4** — AC-6's two instruments are two new `#[test]`s in the **existing**
  `crates/goad-boundary/tests/checks/structure.rs`, which becomes
  directory-parameterised. No new module, no change to
  `crates/goad-boundary/src/`.
- **PL-5** — `tests/support/` splits into `driving.rs` (host composition) and a
  new `scripting.rs` (the scripted-backend and invocation-log helpers) at
  PHASE-05, so AC-10's target can include only what it uses. See FD-3.
- ~~**PL-6**~~ — **superseded by PL-11.** The measurement discipline is
  unchanged; the consequence of a breached margin is now a STOP rather than a
  follow-up (F-6).
- **PL-7** — no new `tests/backends/` script and no new sentinel. Every fixture
  this slice needs is a response body the existing `answers-as-instructed.sh`
  already passes through, or one of its existing sentinels.
- **PL-10** — `#[allow(dead_code)]` on the AC-10 target's own `#[path]` module
  declaration is the cheap alternative to FD-3's split, and it is **refused by
  canon**: POL-001 §Compliance authorises a site-local `#[expect(…, reason)]`
  and says *"never `allow`"*. PL-5's split stands, and PL-10 is the argument
  PL-5 owed (F-3).
- **PL-11** — supersedes PL-6. Margins are measured by the phase that lands each
  test and collected by PHASE-06, and `design.md` §9 is still not retro-fitted;
  a **breached** margin is a STOP at the phase that measured it, not a note
  carried forward (F-6).
- **PL-12** — supersedes PL-2's Consequence. The seven fixtures two renderer
  modules now share move to `crates/goad/tests/renderer/harness.rs`, a
  target-local module on the pattern
  `crates/goad-shell/tests/integration/harness.rs` already sets, and PHASE-02
  declares `wiring.rs` and `table.rs` as surfaces bounded to that move and to
  the `absorb` migration (F-1, F-2).

## Findings against the design

Recorded rather than patched (`docs/AGENTS.md:101`). None blocks the plan.
**FD-1 was escalated and is now ruled** (`design-log.md` 2026-09-07, D-17); the
other three are settled inside the plan.

### FD-1 — the diagnostic line as designed deletes the "Nothing to report." sentinel, and repairing it needs the markup

`design.md` §5.2 has `glass.rs` append the next-check line to the model it
already builds from `Diagnostics::lines()`. The markup renders its
empty-surface sentinel on exactly that model's length:

```
crates/goad/ui/app.slint:60
      if root.diagnostic-lines.length == 0: Text { text: "Nothing to report."; }
```

Once any exchange has completed, `Frame::next_check` is `Some` and the model is
never empty again, so "Nothing to report." can never render. That is a
user-visible behaviour slice 002 holds deliberately — DT-1 and DT-5, *"the one
case the tray (now) and the window (what happened) are allowed to disagree"* —
and the assertion that fails is
`crates/goad/tests/renderer/wiring.rs::dt1_a_clean_outcome_under_diagnostic_mode_clears_lines_but_leaves_the_window_open`,
at its `nothing_to_report_shown(&window)` call.

**Ruled, 2026-09-07** (`design-log.md`, D-17): the line gets its **own**
`in property <string> next-check` on `PromptWindow`, rendered under the
diagnostic list; `diagnostic-lines` stays the exchange's record. This is §5.2's
own argument for keeping the line outside `Diagnostics` — a standing schedule is
not an exchange's product — applied to the glass as well as to the types. It
costs two lines of markup and one setter.

**Rejected:** accepting the loss and rewriting `dt1`'s assertion, which retires
a user-visible behaviour by side effect.

**Consequence:** `crates/goad/ui/app.slint` is now in `slice-003.md`'s Scope,
bounded to that property and its one markup line. PHASE-04 declares it
unconditionally, and a second markup change there is a STOP.

### FD-2 — `structure.rs` cannot express AC-6 as it stands, in two ways

Both are mechanical, and both would be discovered late by an agent reading only
the design.

- **`SUBJECT_DIR` is a module constant** (`structure.rs:20`), and every helper
  — `subject_files`, `occurrences_of`, the vacuity guard — closes over it.
  AC-6's instrument (b) scans `crates/goad-shell/src`. The file has to become
  directory-parameterised, and the vacuity guard has to run for both directories.
- **`occurrences_of` matches with `str::contains`** (`structure.rs:78`).
  `design.md` §9 asks instrument (a) for the identifier `resolve` *"matched as a
  word"*, which `contains` is not: it also matches `resolved`, an ordinary
  English participle. `goad_boundary::scan::mentions` is the word matcher and is
  already public, but it **cannot replace `contains` wholesale** — `mentions`
  splits on every non-alphanumeric byte, so the three existing needles
  (`quit_event_loop(`, `tokio::spawn`, `slint::spawn_local(`) would stop
  matching. Two matchers, named, are the answer; PHASE-04/EX-2 states it.

Measured against the tree at HEAD 572049f, confirming `design.md` §2's rows:
`crates/goad/src` has **0** production lines naming `resolve` over **12** `.rs`
files; `crates/goad-shell/src` has **2** naming `schedule::resolve` over **8**,
at `host.rs:128` and `host.rs:259`.

### FD-3 — a third includer of `tests/support/driving.rs` fails the gate

`design.md` §5.5 and `slice-003.md`'s Scope say the AC-10 target reuses
`tests/support/driving.rs` rather than duplicating it. It cannot, as things
stand. `dead_code` is `warn` in the workspace table (`Cargo.toml:103`) and the
gate's clippy line promotes it back to an error with `-D warnings`, so **every
`pub(crate)` symbol in a `#[path]`-included helper must be reachable from every
target that includes it** — the rule
`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md` records as
PL-4 of slice 002, and which the tree currently satisfies exactly (`CLEANUP_LIMIT`
is used by `integration/transport.rs` *and* by `renderer/table.rs`).

An AC-10 target that includes the file whole would leave thirteen of the file's
nineteen `pub(crate)` symbols unreachable. PL-5 is the answer: move the six
scripted-backend helpers (`backend`, `marker`, `clear`, `logging_backend`,
`invocations`, `scripted`) into `tests/support/scripting.rs`; AC-10's target
includes that file and nothing else, and builds its `Config` and `Host` locally
exactly as `crates/goad/tests/event_loop/closing.rs:63-74` already does. The
closure was re-traced at HEAD: `scripted` (`driving.rs:171`) →
`logging_backend` (`:139`) → `backend` (`:38`) and `marker` (`:50`) → `clear`
(`:59`), with `invocations` (`:148`) called directly. Six move, thirteen stay.

**The cheaper alternative, and why it is refused** (F-3, PL-10).
`#[allow(dead_code)]` on the new target's own `mod` declaration is one line
against ten files, and it does work mechanically — a lint attribute on a module
covers the items of the file it names, and only in the target that carries it,
so the other two targets' `dead_code` coverage is untouched. It is refused
because POL-001 §Compliance authorises *"a **site-local** `#[expect(lint, reason
= …)]` at the narrowest scope that works … **never `allow`**, which is silent
when it stops being true"*, and
`docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` says the same:
*"treat any other spelling (a plain `#[allow]`, a bare `#[expect]`) as a
defect."* Substituting `#[expect]` does not rescue it — a module-wide
expectation over a hand-written shared helper is not site-local, and POL-001's
one module-scoped carve-out is the generated-code quarantine. The positive rule
the split obeys is
`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`: *"When a
later change makes a shared symbol unused by one includer, move it out
immediately … this re-settlement is expected maintenance."*

**This widens the slice's Scope, and is recorded as one.** FD-1's one-file
widening was escalated (PL-8); this is eight files and a test target of another
crate, so it gets the same treatment rather than less: **D-18** in
`design-log.md` and `design.md` §7, and a Scope entry in `slice-003.md` naming
`tests/support/scripting.rs` and the include-and-import lines of both existing
targets.

### FD-4 — AC-3's "later supersedes" test has a second, unstated margin

`design.md` §9's margin table gives that test one ratio: a 300 ms anti-fire
window against the 100 ms deadline it proves was cancelled. There is a second
window it does not name — the interval between the first exchange completing and
the second instruction being enqueued, which must be *shorter* than the 100 ms
deadline or the scheduled firing happens and the test fails for the wrong reason.

It is closable rather than merely measurable: the second command is enqueued
**before** the first exchange completes, on a channel that holds one. `biased`
then serves it ahead of the timer arm on the very next iteration, and there is no
race at all. PHASE-02/VT-7 requires that shape.

### FD-5 — `design.md` §9's AC-4 row discharges only one of AC-4's two successor cases

`slice-003.md` AC-4 requires **both** successor cases to be asserted: a past
instruction given *once*, after which the elapsed value is consumed and cadence
resumes; and one given on *every* response, after which cadence never resumes.
`design.md` §9's AC-4 row names one backend — *"a `serve` test whose backend
returns a past absolute instant on **every** response"* — which is the second
case only. The plan inherited the gap and its own Coverage table then claimed
both (F-4).

`design.md` is a record of intent and is not retro-fitted
(`docs/AGENTS.md:137`), so this is recorded rather than patched. The plan
repairs it: **PHASE-03/VT-5** drives the one-off case, and the pure half is
already held at stratum 1 by
`schedule.rs::an_elapsed_retained_check_is_consumed_and_the_default_poll_applies`
(`:311`) — the arm AC-4's own `schedule.rs:196-201` citation points at.

The repair deliberately does **not** assert the third invocation landing. Under
the floor it lands three seconds after the second, which is the whole floor
interval of gate time AC-5's own revision refused to pay. What VT-5 asserts
instead is the retained value: after the loop stops, `Frame::next_check` is
`now + default_poll` rather than the past instant, which is *the elapsed value
was consumed* observed through the loop, at no wall cost.

### FD-6 — the draft spec's requirements were never walked

`draft-spec.md` is the slice's working authority (`docs/AGENTS.md:36`), and
`design.md` §9 and this plan's first Coverage table both walk `slice-003.md`'s
AC-1..AC-12 and stop. Nothing walked R-1..R-11. Two requirements fell through:
R-4's second half (*"a person acting in the middle of a scheduled cadence does
not raise that count"*) and R-5's second (*"an evaluation a person asks for is
dispatched without waiting for it"*), which is the property `design.md` §5.4
argues hardest for (F-5).

Repaired by construction rather than by vigilance: the **second Coverage table**
below maps every requirement to a criterion, and PHASE-03/VT-6 is the criterion
the walk showed was missing. PHASE-06/EX-1 remains the last check, but it is no
longer the first.

## Coverage

Two tables, because the slice answers to two documents. The first walks
`slice-003.md`'s acceptance criteria; the second walks `draft-spec.md`
(SPEC-002)'s requirements, which are the slice's working authority and were
previously walked nowhere before PHASE-06 (FD-6).

### Acceptance criteria

Every acceptance criterion in `slice-003.md`, mapped to the phase and criterion
that discharges it.

| AC | discharged by |
|----|---------------|
| AC-1 — evaluates unprompted on the default poll | PHASE-02/VT-3 |
| AC-2 — `next_check` from either direction changes the wait | PHASE-02/VT-4 (from an `evaluate`), PHASE-02/VT-5 (from a `respond`) |
| AC-3 — a later valid instruction supersedes, both directions | PHASE-02/VT-6 (earlier wins), PHASE-02/VT-7 (later wins) |
| AC-4 — a past instant fires as the floor permits, once per instant, no underflow, no spin | PHASE-01/VT-2 (the arithmetic, at zero cost); PHASE-03/VT-1 (the loop, a past instruction on **every** response — the retained value is replaced verbatim and cadence never resumes); PHASE-03/VT-5 (the loop, a **one-off** past instruction — the elapsed value is consumed and the retained check returns to cadence). The pure half of the one-off case is already held by `schedule.rs::an_elapsed_retained_check_is_consumed_and_the_default_poll_applies` (`:311`), which is what AC-4's own `schedule.rs:196-201` citation names (FD-5) |
| AC-5 — a failing backend is polled on its existing cadence and no faster | PHASE-03/VT-2 |
| AC-6 — the timer never resolves a schedule | PHASE-04/VT-3 (absence over stratum 3), PHASE-04/VT-4 (count over stratum 2) |
| AC-7 — a timer does not defeat cancellation | PHASE-02/VT-8 |
| AC-8 — a backend failure does not stop the clock | PHASE-03/VT-2, read for liveness rather than for rate |
| AC-9 — a clock that cannot be read loses neither the instant nor the deadline, and does not spin | PHASE-03/VT-3 |
| AC-10 — proved in the arrangement it will run in, minus the platform | PHASE-05/VT-1 |
| AC-11 — no domain vocabulary | PHASE-06/VA-2, over the finished tree; the standing scan runs at every phase's VA-1 |
| AC-12 — `just check` exits 0, nothing weakened | PHASE-06/VA-1, over a clean clone; every phase's VA-1 in the working tree |

### Draft SPEC-002's requirements

`draft-spec.md` §7 states a *verified by* row per requirement in prose. This
table names the criterion that is that row. PHASE-06/EX-1 checks the names
against the tree at the end; this table is what keeps a hole from surviving to
be found there (FD-6).

| R | discharged by |
|---|---|
| R-1 — evaluates when a check comes due, unasked | PHASE-02/VT-3 |
| R-2 — no resolution outside SPEC-001/R-26's one | PHASE-04/VT-3 (absence over stratum 3), PHASE-04/VT-4 (count over stratum 2) |
| R-3 — an elapsed instant fires once, no underflow | PHASE-01/VT-1 and VT-2 (the arithmetic); PHASE-03/VT-1 (repeated) and VT-5 (one-off) |
| R-4 — minimum spacing, **whatever else the host did in between** | PHASE-03/VT-1 (the bound itself), PHASE-03/VT-6 (a person acting mid-cadence does not raise the count), PHASE-03/VA-3 (break-and-revert: the floor is what bounds it) |
| R-5 — spacing binds scheduled firings only | PHASE-02/VT-3 (a `default_poll` under the spacing is honoured for the process's first scheduled firing), PHASE-03/VT-6 (a person's own evaluation is dispatched without waiting for it) |
| R-6 — spacing alters nothing stored or reported | PHASE-03/VT-1's retained-value assertion: after a past instruction the reported next check is that instruction, unchanged |
| R-7 — supersession works in both directions | PHASE-02/VT-6 (earlier), PHASE-02/VT-7 (later) |
| R-8 — a clock failure loses nothing and does not loop | PHASE-03/VT-3, with PHASE-03/VT-4 as its vacuity control |
| R-9 — no scheduled evaluation while an exchange is in flight | PHASE-02/EX-6 by construction — the timer arm exists in the **first** `select!` only — witnessed by PHASE-02/VT-1's six unchanged `serve` tests |
| R-10 — cancellation takes precedence; nothing left to cancel | PHASE-02/VT-8, and VT-1's three existing cancellation tests |
| R-11 — no catch-up for intervals missed while not running | **review, not a test** (`draft-spec.md` §7 says so: nothing persists, so there is no record to catch up from). PHASE-06/EX-1 records it as such rather than letting it look discharged |

---

## PHASE-01 — The arithmetic, and the third stimulus

**Objective:** stratum 1 can answer *how long from this instant to that one*,
totally and at zero wall cost, and stratum 3 can name a scheduled evaluation on
the wire. Nothing waits yet.

**Surfaces:** `crates/goad-semantics/src/schedule.rs`,
`crates/goad/src/wire.rs`, `docs/slices/003/notes.md`.

**Must not touch:** `crates/goad/src/controller.rs`, `glass.rs`,
`diagnostics.rs`, `reception.rs`; any file under `crates/goad-shell/src`;
`crates/goad/Cargo.toml`; any manifest at all.

**Entry**
- EN-1 — the tree is at the accepted design (HEAD 572049f or a descendant that
  changes no source), and `just check` exits 0 before anything is edited.

**Exit**
- EX-1 — `schedule.rs` carries `pub fn wait_for(next_check: Timestamp, now:
  Timestamp) -> std::time::Duration`, computing `max(next_check - now, 0)`
  through `jiff::Timestamp::duration_since` and
  `std::time::Duration::try_from(..).unwrap_or(ZERO)`. **No `+` or `-` operator
  appears** — the module carries `#![deny(clippy::arithmetic_side_effects)]`
  (`schedule.rs:14`).
- EX-2 — `wait_for` takes **no** minimum spacing and names no constant of stratum
  3's (D-14). `resolve`, `parse` and `parse_span` are unchanged in behaviour.
- EX-3 — `resolve`'s doc comment at `schedule.rs:198-201` no longer states the
  one-off past instant as the general case. It states both successor cases, as
  `design.md` §5.5 E-1 does: a one-off instruction is consumed and cadence
  resumes; an instruction repeated on every response is *replaced* by
  `resolve`'s first arm and cadence never resumes. It cites `SPEC-001/R-26` and
  `SPEC-001/R-28`, not a finding id.
- EX-4 — `wire.rs`'s `Stimulus` has three variants; `kind()` returns
  `"startup" | "requested" | "scheduled"`; `event()` is unchanged and still
  writes `source: "host"` and `data: Value::Null` for all three. The doc comment
  at `wire.rs:37-38` lists all three.
- EX-5 — no manifest changes, no new dependency, no new feature.

**Verification**
- VT-1 — `wait_for` unit tests in `schedule.rs`'s existing `#[cfg(test)] mod
  tests`, for a future instant (the exact difference), an instant **at** `now`
  (zero), an instant before `now` (zero, not an underflow), and instants at both
  edges of `jiff::Timestamp`'s representable range in both directions (total,
  no panic).
- VT-2 — AC-4's arithmetic half: the past-instant and at-`now` cases above
  assert `Duration::ZERO` **by value**, not merely "small".
- VT-3 — `wire.rs`'s inline test module asserts `Stimulus::Scheduled.kind() ==
  "scheduled"`, and that `Stimulus::Scheduled.event(now)` carries `source ==
  "host"`, `kind == "scheduled"`, `timestamp == now` and `data ==
  Value::Null` — the three fields CD-1's R-56 makes normative.
- VA-1 — `just check` under `nix develop`, output pasted into the phase sheet.
- VA-2 — `cargo test -p goad-semantics` named separately in the sheet: it is the
  gate's own third command and the only one that builds stratum 1 with exactly
  its own features (POL-001 §Compliance).

**STOP**
- S-1 — `wait_for` cannot be written without an arithmetic operator, or without
  a `#[expect]`. Stop: D-14 and the module deny are both design, and a
  suppression here is POL-001's prohibited motive.
- S-2 — correcting `resolve`'s doc comment turns out to require a change to
  `resolve`'s *behaviour*. Stop: `slice-003.md` Scope says a behaviour change
  there is a design change, not a refactor.

**Notes for the implementer**

- The two halves are independent. Do the stratum 1 half first: it has its own
  gate column and a failure there is unambiguous.
- `jiff::Timestamp::duration_since(self, other) -> SignedDuration` exists at
  jiff 0.2.35 `timestamp.rs:1919` and returns `self - other`, so the argument
  order is `next_check.instant().duration_since(now.instant())`.
  `impl TryFrom<SignedDuration> for core::time::Duration` is
  `signed_duration.rs:2574`; it fails **only** on a negative value, which is
  exactly the elapsed case, so `unwrap_or(Duration::ZERO)` is total and not a
  swallowed error.
- `Timestamp` is `Copy` (the derive is `canonical.rs:102`, the struct `:103`)
  and its only accessor is `instant()` (`:110`). There is no `PartialOrd`, so do
  not reach for one.
- The `#[cfg(test)] mod tests` in `schedule.rs` already has `fn instant(rfc3339:
  &str)` and `fn now()` helpers — use them; do not mint a third spelling.
- `wire.rs` line 208 and line 215 are test function names containing the word
  *resolve*. They sit **after** the file's `#[cfg(test)]` at line 185 and must
  **not** be renamed: PHASE-04's instrument cuts there deliberately, and
  renaming them to buy a green scan is the thing `design.md` D-16 rejects.
- The wire form CD-1 pins is
  `{"source":"host","kind":"scheduled","timestamp":<now>,"data":null}`. SPEC-001
  §6.1's illustration still says `"source":"timer"`; that is CD-2 and it is
  **not** applied in this slice.

---

## PHASE-02 — The wait, and the cadence it keeps

**Objective:** `serve` wakes on time. A resolved instant becomes an
`evaluate` without a person asking; an instruction from either entry point moves
the next firing; an earlier instruction shortens the wait and a later one
lengthens it; and a stop issued while the loop is waiting still ends it at once.

**Surfaces:** `crates/goad/src/controller.rs`;
`crates/goad/tests/renderer/{main.rs, scheduling.rs, harness.rs}` —
`harness.rs` is **new**, and `scheduling.rs` is **new**;
`crates/goad/tests/renderer/{wiring.rs, table.rs}` — **bounded to three
mechanical changes and nothing else**: appending `.shift` at the 22 call sites
that read `absorb`'s return value (EX-3); deleting, in `wiring.rs` only, the
seven fixture definitions that move to `harness.rs` (EX-11); and editing, in
`wiring.rs` only, the eight `use` lines EX-12 names one by one — the imports
those seven fixtures were the sole consumers of, which `unused_imports` (denied
at `Cargo.toml:99-102`) will not let the phase leave behind;
`docs/slices/003/notes.md`.

The two bounded files are declared because the phase **cannot compile without
them** (F-1, F-2, F-13), not because it may work in them. No test body in either
file changes; VA-4 is the instrument that says so. The third class is stated as
the compiler's bill in EX-12 rather than as *"whatever the compiler demands"*,
which would be no bound at all.

**Must not touch:** `crates/goad/src/{glass.rs, diagnostics.rs, reception.rs,
wire.rs, install.rs, main.rs}`; `crates/goad/ui/app.slint`; anything under
`crates/goad-shell/src` or `crates/goad-semantics/src`; `tests/support/`;
`tests/backends/`; `crates/goad/tests/event_loop/`;
`crates/goad/tests/renderer/{tree.rs, mapper.rs, tray.rs, reception.rs,
startup.rs}`; any manifest. Within `wiring.rs` and `table.rs`, **any edit that
is not one of the two named above** — including a change to a test body, an
assertion, a fixture that stays, or a module's `use super::` line.

**Entry**
- EN-1 — PHASE-01's exit criteria are discharged and `just check` exits 0.
- EN-2 — `goad_semantics::schedule::wait_for` and `Stimulus::Scheduled` both
  exist and are covered by PHASE-01/VT-1..VT-3.

**Exit**
- EX-1 — `controller.rs` declares `const MINIMUM_SPACING: std::time::Duration =
  Duration::from_secs(3)`, private to the module, with a doc comment citing
  `SPEC-002/R-4` and stating that it is not configurable, not visible to a
  backend, and never applied to anything a person asked for.
- EX-2 — `Controller` gains a private `next_check: Option<Timestamp>`, written
  only by `absorb` and read only by `frame()`. `Frame` gains
  `pub next_check: Option<Timestamp>`. **There is no
  `Controller::next_check()` accessor** (D-15).
- EX-3 — `absorb` returns `Absorbed { pub shift: Shift, pub next_check:
  Timestamp }` — `next_check` **not** an `Option`, because `Outcome::next_check`
  is concrete on every outcome including failures (`host.rs:76`). There is **one**
  `absorb`, not a new one beside the old: `CLAUDE.md` forbids a parallel
  implementation, and a migrate-then-delete pair moves the same 22 call sites in
  the same phase for the price of a second name. All 22 read sites are updated
  to `.shift` in this phase — `wiring.rs` nine `let shift = …` (`:290`, `:323`,
  `:508`, `:526`, `:552`, `:570`, `:599`, `:631`, `:671`) and four inside an
  `assert_eq!` (`:830`, `:853`, `:880`, `:901`); `table.rs` two `let shift = …`
  (`:671`, `:720`) and seven inside an `assert_eq!` (`:792`, `:801`, `:809`,
  `:823`, `:837`, `:849`, `:866`). The remaining 20 call sites discard the value
  and do not change. Nothing else about either file changes, and `Shift` is
  already imported wherever it is asserted, so no import moves.
- EX-4 — `Pending` gains `fn now(&self) -> Timestamp` beside `exchanged()`.
- EX-5 — `serve` holds a pinned `tokio::time::Sleep`, **always armed**, initially
  at `started + MINIMUM_SPACING`, and a `floor_until: tokio::time::Instant`
  initialised to `started` — that is, already in the past, so the first
  scheduled firing of the process is not spaced (A-3).
- EX-6 — the first `select!` has a third arm, **last** in `biased` order after
  cancel and commands, and the `select!` **binds a discriminant**: a private
  `Fired` with `Command(Command)` and `Scheduled` variants, exactly as
  `design.md` §5.4's written-out loop has it. Winning the timer arm does two
  things and only two: sets `floor_until = tokio::time::Instant::now() +
  MINIMUM_SPACING` — the one and only write site — and yields `Fired::Scheduled`.
  The `Command::Evaluate(Stimulus::Scheduled)` it dispatches is built after the
  match and goes through the same `stamp` as every other command. The
  discriminant is a criterion and not a note because a bare `Command` carries no
  provenance, and EX-7 has nothing to be true of without it (F-12).
- EX-7 — there are exactly **two** re-arm sites. After `absorb`:
  `sleep.as_mut().reset(max(Instant::now() + wait_for(absorbed.next_check,
  requested_at), floor_until))`. After a refusal **that came from the timer
  arm**: `sleep.as_mut().reset(floor_until)`. Every other refusal path leaves
  the deadline untouched and still `continue`s.
- EX-8 — **no `expect`, `unwrap` or `panic` is added to production code**, and no
  `#[expect]` is added anywhere in `crates/goad/src`. `expect_used` and
  `unwrap_used` are `deny` workspace-wide and carved out for tests only
  (`clippy.toml:20-23`).
- EX-9 — the identifier `resolve` still appears **nowhere** in
  `crates/goad/src`'s production code. The import is
  `use goad_semantics::schedule::wait_for;` — no brace group (I-1a).
- EX-10 — `crates/goad/tests/renderer/main.rs` declares `mod scheduling;` and
  `mod harness;`, both `#[cfg(test)]` at the declaration like every other module
  there.
- EX-11 — `crates/goad/tests/renderer/harness.rs` exists and holds exactly the
  seven fixtures two modules of this target now share, **moved from `wiring.rs`
  unchanged in body and signature and `pub(crate)` in their new home** — the
  visibility is the one permitted difference, because a sibling module reaches
  nothing private, and it is how
  `crates/goad-shell/tests/integration/harness.rs` already spells its own.
  `stub_clock`'s `#[expect(clippy::unnecessary_wraps, reason = …)]` travels with
  it. The seven are `TIMEOUT` (`wiring.rs:28`), `now` (`:30`), `stub_clock`
  (`:45`),
  `window_and_tray` (`:65`), `glass_over` (`:73`), `current_view_token` (`:123`)
  and `until` (`:135`). Nothing else moves — `in_diagnostic_mode`,
  `nothing_to_report_shown`, `window_shown`, `accessible_enabled_of` and the
  response-body constants have one consumer and stay. `wiring.rs` gains one
  `use crate::harness::{…}` line and its nine child modules are **not edited**:
  a child's `super::X` resolves through a private parent import, which the tree
  already relies on at `wiring.rs:706`. `harness.rs`'s module doc states the
  rule it inherits from `crates/goad-shell/tests/integration/harness.rs:5-10` —
  two or more consumers in this target live here, one consumer stays where it
  is, and what both *tiers* need lives in `tests/support/`.
- EX-12 — `wiring.rs`'s import block is the compiler's bill for EX-11 and
  nothing more. Measured at HEAD: the seven moved fixtures are the sole
  consumers of **ten** imported items across **eight** `use` lines. Four lines
  go, four narrow, one is added:

  | `wiring.rs` | after |
  |---|---|
  | `:12` `use std::rc::Rc;` | **deleted** — only use is `glass_over` (`:77`) |
  | `:15` `use goad::clock::ClockError;` | **deleted** — only use is `stub_clock` (`:45`) |
  | `:21` `use goad_semantics::protocol::canonical::Timestamp;` | **deleted** — `now` (`:30`) and `stub_clock` (`:45`) |
  | `:18` `use goad::generated::{OptionRow, PromptWindow, Tray};` | `use goad::generated::PromptWindow;` — loses `OptionRow` (`:77`) and `Tray` (`:65`, `:69`, `:73`) |
  | `:19` `use goad::glass::{Glass, SlintGlass};` | `use goad::glass::Glass;` — `SlintGlass`'s only code uses are `:73`-`:74`; `:682` and `:808` are doc comments |
  | `:22` `use i_slint_backend_testing::{ElementHandle, ElementQuery, init_no_event_loop};` | loses `init_no_event_loop` (`:66`) |
  | `:23` `use slint::{ComponentHandle, Model, VecModel};` | `use slint::ComponentHandle;` — loses `VecModel` (`:77`) **and `Model`**, the trait behind `.row_data` at `:126`; `ComponentHandle` stays on `as_weak` (`:115`, `:387`) |
  | `:26` `use crate::driving::{host, instant, invocations, quiet_event, scripted};` | loses `instant` (`:31`) |
  | — | **added:** `use crate::harness::{TIMEOUT, current_view_token, glass_over, now, stub_clock, until, window_and_tray};` |

  `harness.rs` carries the mirror image: `Rc`, `Duration`, `ClockError`,
  `OptionRow`, `PromptWindow`, `Tray`, `SlintGlass`, `Timestamp`,
  `init_no_event_loop`, `ComponentHandle`, `Model`, `VecModel`, and
  `crate::driving::instant`. `table.rs` takes only EX-3's migration and its
  imports do not move.

**Verification**
- VT-1 — `a_click_naming_a_superseded_view_is_refused_with_no_backend_contact`
  (`wiring.rs:923`),
  `the_negative_control_with_no_intervening_evaluate_the_click_is_answered`
  (`:1003`), `serve_drives_one_exchange_through_the_production_loop` (`:1070`),
  `tripping_cancel_mid_exchange_ends_serve_well_under_the_timeout` (`:1119`),
  `a_stop_tripped_before_the_first_poll_wins_over_a_ready_command` (`:1170`) and
  `on_stop_a_command_queued_behind_the_exchange_is_left_unread` (`:1196`) all
  still pass with **their bodies unchanged**. That is decidable rather than
  aspirational: all six begin at or after `:923`, past the last `absorb` read
  site (`:901`), so none of them is touched by EX-3's migration, and EX-11's
  lift edits no child module. VA-4 is the instrument. Risk R4 is discharged here
  or it is a finding.
- VT-2 — a `serve` test proving the timer arm is reachable at all: with a
  `default_poll` of 100 ms and a backend that returns `{"view":null}`, a second
  invocation appears. This is the positive control every absence assertion below
  is paired with.
- VT-3 — **AC-1, and SPEC-002/R-1 and R-5's first half.** The same run, read as
  the criterion: the backend's own invocation log advances to 2 within
  `until(2 s)`, and the second request's `event.kind` is `"scheduled"`. It is
  also R-5's first half — *"the first scheduled evaluation of the process
  honours a default poll shorter than the spacing"* — and the criterion says so:
  `default_poll` is 100 ms against a 3 s floor, and the firing lands at ~105 ms
  because `floor_until` starts in the past. That is discharge, not a side
  effect. *Liveness. Expected ~105 ms against a 2 s bound.*
- VT-4 — **AC-2, from an `evaluate`.** A backend whose first response instructs
  `"100 milliseconds"` while `default_poll` is far (30 minutes): the second
  invocation lands inside `until(2 s)`. Without the instruction being consumed
  it would not land for half an hour. *Liveness.*
- VT-5 — **AC-2, from a `respond`.** The first response carries a view; a
  `Command::Choose` naming it is answered with `"100 milliseconds"`; the third
  invocation — the scheduled one — lands inside `until(2 s)`. The view token is
  read off the window's own options model, as `wiring.rs::current_view_token`
  does; do not mint it independently.
- VT-6 — **AC-3, earlier supersedes.** First instruction `"60 seconds"`, then a
  person-driven `evaluate` answering `"100 milliseconds"`. The scheduled firing
  lands inside `until(2 s)` — which it cannot do if the pending 60 s deadline
  survived. *Liveness. This is the case `max(retained, incoming)` gets wrong
  (`schedule.rs:204-208`).*
- VT-7 — **AC-3, later supersedes.** First instruction `"100 milliseconds"`,
  then `"60 seconds"`. **No** invocation beyond the second inside a 300 ms
  window — a window deliberately longer than the 100 ms deadline the test proves
  was cancelled. *Anti-fire.* The second command must be enqueued **before** the
  first exchange completes (FD-4); assert that it was, by asserting the second
  invocation's `event.kind` is `"requested"` and not `"scheduled"`.
- VT-8 — **AC-7.** With a far `default_poll` and no command pending, the loop is
  parked on the timer arm; `Cancel::stop()` from the driving task ends `serve`
  well inside `TIMEOUT` (2 s), returning `Ending::Stopped`. Its failure mode is a
  hang, not a late value.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — the wall time of `cargo test --workspace` recorded before and after the
  phase, and the per-test elapsed time of VT-3..VT-8 recorded against
  `design.md` §9's margin table rows for AC-1, AC-2 (×2), AC-3 (×2) and AC-7
  (PL-11). Written into the phase sheet; **`design.md` is not edited**.
- VA-3 — break-and-revert on the floor: temporarily set `MINIMUM_SPACING` to
  zero, confirm nothing in this phase's tests goes red (they are all above the
  floor), revert, and record it. The floor's own evidence is PHASE-03's; this
  records that PHASE-02's tests do not silently depend on it.
- VA-4 — **the two bounded files are bounded.** `git diff` over
  `crates/goad/tests/renderer/{wiring.rs, table.rs}`, pasted into the sheet,
  showing only: 22 lines gaining `.shift`; seven fixture definitions deleted
  from `wiring.rs`; and EX-12's eight `use` lines — four deleted, four narrowed,
  one added. No renamed symbol, no changed test body, no touched `use super::`
  line, and no `use` line EX-12 does not name. **Widened visibility on the seven
  moved items is the one permitted difference** (EX-11); everything else about
  them is byte-identical. Borrowed from PHASE-05/VA-2, which has the right
  instrument for the analogous move. A mechanical churn of this size across a
  1232-line file is exactly where a behaviour change hides.

**STOP**
- S-3 — the conditional re-arm after a refused scheduled firing cannot be
  written without restructuring the `match command` block beyond recognition.
  Stop and consult: the shape is the design's, and a second `select!` or a
  second loop is a design change.
- S-4 — any existing test in `crates/goad/tests/renderer/` or
  `crates/goad-shell/tests/` goes red. This is R4 firing. Record what changed
  and stop; do **not** adjust the existing test to accommodate the timer.
- S-5 — the phase is tempted to add `tokio`'s `test-util` feature, a mock clock,
  or any manifest change. D-7 rejected all three, and POL-001's feature-unification
  residue is a design decision reserved to the slice's design.
- S-6 — a timed assertion is flaky across three consecutive runs. POL-001 calls
  that a design defect, not a tolerated cost; record the measurement and stop.
- S-20 — a liveness margin measured in VA-2 is **below 5x**. Record the
  measurement and stop; the orchestrator is consulted (PL-11). Do not widen a
  bound, shorten an expectation, or carry it forward as a note: AC-12 calls a
  load-sensitive test a design defect, and this is the phase that can still fix
  it.
- S-21 — `wiring.rs` or `table.rs` needs an edit that is not one of EX-3's 22
  `.shift` sites, EX-11's fixture move, or EX-12's eight `use` lines. Stop: the
  surface is bounded to those three changes, and anything else in those files is
  a scope change. **An import EX-12 does not name that the compiler nonetheless
  reports as unused is itself the STOP** — it means the move is not the one the
  plan measured, and the bill needs re-deriving rather than extending on the
  spot.

**Notes for the implementer**

- **The one shape the design leaves to you.** The refusal paths today `continue`
  from inside the `let pending = match command { … }` expression — it opens at
  `controller.rs:325` and `let exchanged = pending.exchanged();` is `:362` — so
  `Fired::Scheduled` is not in scope where the refusal is handled. Hoist it:
  bind what the arm produced *before* the match (EX-6's `Fired`), and handle the
  refusal after it. The narrow fact that makes this easy is that
  the **only** refusal reachable from the timer arm is `stamp` failing — the
  timer arm builds `Command::Evaluate`, and `OpenDiagnostics`,
  `CloseDiagnostics` and `Choose` can only come from the channel.
- **The clock in the cheap tier is fixed.** `wiring.rs::stub_clock` returns
  `2026-01-01T00:00:00Z` on every call. That is not a problem — it is what makes
  these tests deterministic. `wait_for` is computed from the `now` the *request*
  carried, so a relative instruction of `"100 milliseconds"` yields a 100 ms wait
  on every cycle, forever, and an absolute past instant yields zero on every
  cycle. Reuse `stub_clock`'s shape; `Clock` is a `fn` pointer and cannot
  capture.
- **Building a host with a short poll.** `driving::host_from(config, now)` takes
  a `Config` you build yourself. `Config`, `BackendConfig` and `ScheduleConfig`
  all have `pub` fields (`crates/goad-shell/src/config.rs:28-73`). Do **not**
  add a helper to `tests/support/driving.rs` (PL-3, FD-3).
- **The scripted backend past the end of its list** answers
  `{"view":null,"next_check":"45 minutes"}`
  (`tests/backends/answers-as-instructed.sh:30`). Give every case one more
  instruction than it expects invocations, or expect that default.
- **`"100ms"` and `"100 milliseconds"` both parse**; `"0:00:00.1"` does not —
  `parse_span` refuses anything shaped like a time of day
  (`schedule.rs:looks_like_a_time_of_day`).
- **Reading the request's kind.** `answers-as-instructed.sh` does `cat
  >/dev/null` and does not echo the request. If a case needs to assert
  `event.kind`, the cheapest route is a case-local script that appends the
  request to the log; a new script under `tests/backends/` is permitted by
  `slice-003.md`'s Scope, PL-7 notwithstanding — PL-7 says none is *needed* for
  the behaviour, not that none may exist for the wire. Prefer asserting the
  invocation count where the kind is not the point.
- **`until` is `wiring.rs`'s** (`wiring.rs:135-148`) and polls every 5 ms,
  panicking when the bound passes. `scheduling.rs` needs it, and so do six other
  fixtures: EX-11 moves all seven to `harness.rs`. Do the move **first**, before
  writing a line of `scheduling.rs`, and run the gate on the move alone — a
  seven-item lift that compiles is a much cheaper thing to debug than a
  seven-item lift plus a new `select!` arm.
- **The move is smaller than it looks.** `wiring.rs`'s child modules import
  through `super::`, and `super::X` resolves through a private `use` in the
  parent — `mod body_content` (`wiring.rs:706`) already reaches `host`,
  `quiet_event` and `scripted` that way, and they are `crate::driving`'s. So
  `wiring.rs` gains `use crate::harness::{TIMEOUT, current_view_token,
  glass_over, now, stub_clock, until, window_and_tray};` at the top and its nine
  `use super::{…}` lines are left alone. If `unused_imports` fires on that line,
  the fallback is to point the affected child module at `crate::harness`
  directly — not to copy a fixture.
- **`harness.rs` needs `use crate::driving::instant;`** for `now`, and it is the
  only thing it takes from the shared helper. That is why PHASE-05's split of
  `tests/support/` does not reach it.
- `tokio::time::Instant` implements `Ord` and `Add<Duration>`, so
  `std::cmp::max(Instant::now() + wait, floor_until)` is one expression with no
  arithmetic lint exposure — `controller.rs` carries no
  `arithmetic_side_effects` deny.
- Slice 002's `serve` tests bound at 2 s and dispatch immediately, so the
  always-armed 3 s initial sleep should not reach any of them. VT-1 is the
  criterion that says so out loud.

---

## PHASE-03 — What the floor bounds, and what a failure does not stop

**Objective:** every way a backend, a person or the machine can try to make the
host spin is bounded and proved bounded — a past instruction on every response,
a past instruction once, a failure on every invocation, a clock that stops
working after the first read, and a person acting in the middle of a scheduled
cadence.

**Surfaces:** `crates/goad/tests/renderer/scheduling.rs`,
`crates/goad-shell/src/config.rs` (**documentation only**),
`docs/slices/003/notes.md`. `crates/goad/src/controller.rs` is declared as a
**repair surface**: this phase may fix a defect its own tests expose in
PHASE-02's mechanism, and may not extend it.

**Must not touch:** any other production file; `tests/backends/`;
`tests/support/`; `crates/goad/Cargo.toml`.

**Entry**
- EN-1 — PHASE-02's exit criteria are discharged and `just check` exits 0.
- EN-2 — PHASE-02/EX-7's second re-arm site (the refused scheduled firing)
  exists in `crates/goad/src/controller.rs`, decided by reading the file: if the
  branch is absent, PHASE-02 is not done. **That it is not yet driven is a
  document check, not a coverage claim** — PHASE-02's phase sheet records the
  branch as written, reviewed and not yet driven, which its own exit already
  obliges it to say. The gate carries no coverage instrument (`justfile:19` —
  six commands, none of them a coverage run), so an executor cannot decide a
  coverage claim and must not be asked to (F-8). VT-3 below is what closes the
  branch.

**Exit**
- EX-1 — AC-4, AC-5, AC-8 and AC-9 each have a `serve` test, and each pairs an
  anti-spin window with a liveness assertion over the same run. SPEC-002/R-4's
  second half and R-5's second half have one too (VT-6, FD-6).
- EX-2 — `ScheduleConfig::default_poll` carries a doc comment stating that a
  value below the host's minimum spacing is accepted and honoured for the
  process's **first** scheduled firing, then floored — citing `SPEC-002/R-5` and
  `SPEC-001/R-21`. This is risk R2's whole mitigation and the **only** edit this
  slice makes to `config.rs`.
- EX-3 — **no production file outside `crates/goad/src/controller.rs` is
  touched, and any `controller.rs` diff is quoted in the phase sheet and named
  as a finding.** Both halves are decided mechanically: the first is a path
  check the audit's own surface diff repeats, the second is a document check.
  The earlier wording — *"no production behaviour changed"* — was discharged by
  either branch and constrained neither, since a repair to the mechanism **is**
  a behaviour change; that is what makes it a repair (F-7). S-9 remains the
  guard that separates a repair from an extension.

**Verification**
- VT-1 — **AC-4.** A backend that instructs an **absolute past instant** on every
  response — e.g. `{"view":null,"next_check":"2020-01-01T00:00:00Z"}` — with
  `default_poll` far. Assert: exactly **two** invocations (the startup one, then
  one unfloored scheduled firing, because `floor_until` starts in the past), and
  that the count does not move across a **500 ms** window. *Anti-spin: 6x under
  the 3 s floor.* Paired liveness: the second invocation must arrive inside
  `until(2 s)`, so a timer that never fires fails too. **Also SPEC-002/R-6:**
  after the loop stops, `Served::controller`'s `frame().next_check` is
  `2020-01-01T00:00:00Z` — the instruction, verbatim and unadjusted by the
  floor. This run is also the measurement `design.md` §5.5 A-1's residue (F-15)
  names — a `reset` to an already-elapsed deadline on the long-lived pinned
  sleep.
- VT-2 — **AC-5 and AC-8.** A backend failing every invocation (`@garbage`
  repeated), `default_poll` 100 ms. Read twice: the second invocation arrives
  inside `until(2 s)` — the clock did not stop (AC-8) — and the count then holds
  across a **500 ms** window — the host is not spinning (AC-5). Assert also that
  the retained instant the controller holds is unchanged across the failure
  (`SPEC-001/R-29`).
- VT-3 — **AC-9.** A clock that **succeeds once and then fails**, as a
  top-level `fn` over a private `static AtomicUsize` **in
  `crates/goad/tests/renderer/scheduling.rs`** — written *in the manner of*
  `harness::stub_clock`, not beside it. It is one test's fixture, and
  `harness.rs` carries what two or more modules share
  (`crates/goad-shell/tests/integration/harness.rs:5-10` states the rule);
  `wiring.rs` is not a surface of this phase and does not become one (F-9).
  `Clock` is a `fn` pointer (`clock.rs:16`) and cannot capture. With `default_poll` at
  100 ms the trace is: the startup exchange completes and arms ~100 ms; the timer
  arm wins and advances `floor_until`; `stamp` refuses; the deadline re-arms to
  `floor_until`, 3 s out. Assert inside a **500 ms** window: exactly one
  `NoClock` refusal line on the diagnostic surface, the retained `next_check`
  unchanged, and the invocation count still **one**. An always-failing clock does
  **not** reach this criterion and must not be substituted (F-18).
- VT-4 — the vacuity control for VT-3: the same fixture with a clock that never
  fails produces a second invocation inside `until(2 s)`, so VT-3's "count still
  one" is not passing because nothing was ever scheduled.
- VT-5 — **AC-4's other successor case, and SPEC-002/R-3** (FD-5). VT-1's
  backend shape, instructed **once**: instruction 1 is
  `{"view":null,"next_check":"2020-01-01T00:00:00Z"}`, instruction 2 is
  `{"view":null}`, and `default_poll` is **100 ms**. Trace: the startup exchange
  is invocation 1 and retains the past instant; the unfloored scheduled firing
  is invocation 2 and advances `floor_until` 3 s out; its response carries no
  instruction, so `resolve` finds a retained value at or before `now`, consumes
  it, and applies the default poll. Assert, inside a **500 ms** window: exactly
  **two** invocations, and — the criterion — that `frame().next_check`, read off
  `Served::controller` after the loop stops, is **`2026-01-01T00:00:00.100Z`**,
  by value. That instant is `now + default_poll` against the fixed `stub_clock`
  and is not the past instruction, so the elapsed value was consumed and cadence
  resumed. Paired liveness: invocation 2 inside `until(2 s)`. The third
  invocation is deliberately **not** awaited — the floor puts it 3 s out, a
  whole floor interval of gate time for evidence
  `schedule.rs::an_elapsed_retained_check_is_consumed_and_the_default_poll_applies`
  (`:311`) already holds at zero cost.
- VT-6 — **SPEC-002/R-4's second half and R-5's second half** (FD-6). VT-1's
  backend and `default_poll`, plus a person. Once the second invocation has
  landed, the driving task sends `Command::Evaluate(Stimulus::Requested)`.
  Assert, over a **500 ms** window from that command: invocation 3 arrives
  inside `until(2 s)` of it being sent — a person's evaluation is **not**
  delayed by the floor (R-5); and the count is then exactly **3** and does not
  move — the person's action did not clear or reset the floor (R-4). A floor
  that a person's exchange cleared would fire invocation 4 at once and fail the
  window, because that exchange also returns a past instant. This is
  `design.md` §5.4's *"a person acting does not clear the floor"*, the property
  the design argues hardest for and the one nothing asserted.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — the per-test elapsed times of VT-1..VT-6 against `design.md` §9's rows
  for AC-4, AC-5 (×2) and AC-9 (×2), and `cargo test --workspace` wall time
  before and after (PL-11). Into the phase sheet, not into `design.md`. VT-5 and
  VT-6 have no §9 row — they are this plan's, from FD-5 and FD-6 — so their
  measured times are recorded as new rows with the expectation stated here:
  ~105 ms liveness against `until(2 s)`, and a 500 ms anti-spin window 6x under
  the floor, the same shape as VT-1's.
- VA-3 — break-and-revert on the floor: set `MINIMUM_SPACING` to zero and
  confirm **VT-1, VT-2 and VT-6 go red** and VT-3's count assertion goes red;
  revert; paste the output. This is the only direct evidence the floor does
  anything, and without it the windows pass against a host that simply never
  fires. VT-6 is the row that matters most here: with no floor, a person's
  exchange re-arms at once and invocation 4 lands inside the window, which is
  precisely SPEC-002/R-4's second half failing.

**STOP**
- S-7 — a test needs an `#[ignore]`, a retry, a sleep longer than the design's
  stated window, or a tolerance widened to make it pass. POL-001 forbids all
  four when the motive is a green gate.
- S-8 — VT-1's invocation count is not 2. That is either the floor not applying,
  the elapsed-deadline `reset` not completing (A-1's open residue), or
  `resolve`'s first arm not behaving as `design.md` §5.5 E-1 says. All three are
  findings; none is a number to adjust.
- S-9 — the phase finds it needs to change the loop's *shape* rather than repair
  a defect in it. That is PHASE-02 being wrong, and it goes back to plan.
- S-22 — a liveness margin measured in VA-2 is **below 5x**. Record the
  measurement and stop; the orchestrator is consulted (PL-11). AC-12 calls a
  load-sensitive test a design defect, and this is the phase that can still fix
  the tests it lands.
- S-23 — VT-5's `frame().next_check` is neither the past instruction nor
  `now + default_poll`. A third value means the loop is resolving something of
  its own, which is SPEC-002/R-2 breached. That is a finding, not a value to
  match the assertion to.

**Notes for the implementer**

- **Why an absolute past instant and not a negative span.** `parse_span` accepts
  a negative span and `resolve` stores it (`SPEC-001/R-28`), but an absolute
  instant is unambiguous against the fixed `stub_clock` and reads as what it is.
- **Why the count is 2 and not 1 or 3.** The startup evaluation is invocation 1.
  `floor_until` starts at the loop's start instant, already in the past, so the
  first scheduled firing is unfloored and lands at once — invocation 2. That
  firing advances `floor_until` by 3 s, so invocation 3 cannot happen inside a
  500 ms window. This is `design.md` §5.4's *"at most one unfloored immediate
  firing"* in a single assertion.
- **`@garbage`** (`answers-as-instructed.sh:51`) exits zero with unparseable
  stdout — a `Failure` with no view, which is what AC-5 needs. `@exit1` also
  works. `@hang` does **not**: it burns the 2 s backend timeout per invocation
  and would make the anti-spin window meaningless.
- **The retained instant is readable** from `Served::controller`'s
  `frame().next_check` after the loop returns (PHASE-02/EX-2), which is how
  VT-1, VT-2, VT-3 and VT-5 assert it without peeking mid-flight. VT-5 is the
  only one that asserts a *computed* value (`now + default_poll`), and it is
  exact rather than approximate because `harness::stub_clock` is fixed at
  `2026-01-01T00:00:00Z` on every call.
- **VT-1, VT-5 and VT-6 are one fixture with three instruction lists.** Write
  VT-1 first and derive the other two from it; the scripted backend takes the
  list as argv, so the difference between them is a `&[&str]` and, for VT-6, one
  `try_send` from the driving task.
- A `static AtomicUsize` in a test module is shared across the whole test binary,
  and `cargo test` runs cases in parallel threads. Give the fixture its own
  `static` and make sure only one test uses it, or the count is not yours.
- `config.rs`'s edit is a doc comment and nothing else. If the phase finds itself
  changing a type or a validation there, it has left its surface.

---

## PHASE-04 — What the person sees, and what the scan holds

**Objective:** the host reports the next check it holds, saying which of the two
instants it is; and two instruments in `goad-boundary` hold the claim that the
timer resolves nothing.

**Surfaces:** `crates/goad/src/{diagnostics.rs, glass.rs}`,
`crates/goad-boundary/tests/checks/structure.rs`,
`crates/goad/tests/renderer/{wiring.rs, tree.rs}` (assertions about the
diagnostic surface only), `crates/goad/ui/app.slint` (**one property and one
markup line**, per D-17), `docs/slices/003/notes.md`.

**Must not touch:** `crates/goad/src/controller.rs`;
`crates/goad-boundary/src/`; `crates/goad/tests/renderer/{harness.rs,
scheduling.rs, table.rs}`; `tests/support/`; any manifest.

VT-2's fixtures are already `wiring.rs`'s (`nothing_to_report_shown`,
`in_diagnostic_mode`), which stayed there under PHASE-02/EX-11 because they have
one consumer. If VT-2 turns out to need a `harness.rs` fixture, that is S-24.

**Entry**
- EN-1 — PHASE-03's exit criteria are discharged and `just check` exits 0.
- EN-2 — `crates/goad/ui/app.slint` still carries its empty-state sentinel at
  the diagnostic surface (`if root.diagnostic-lines.length == 0`), and
  `wiring.rs::dt1_a_clean_outcome_under_diagnostic_mode_clears_lines_but_leaves_the_window_open`
  is green. D-17's whole point is that both stay true through this phase.

**Exit**
- EX-1 — `diagnostics.rs` carries `pub fn next_check_line(at: Timestamp) ->
  String`, rendering second precision via
  `jiff::Timestamp::round(jiff::Unit::Second)` and falling back to the unrounded
  instant if rounding errors at the edge of representable time. The string names
  what it is: the **instruction**, not the deadline. It goes through the same
  `finish`/`bound` pipeline every other line on this surface goes through.
- EX-2 — `structure.rs` is directory-parameterised and carries **two** matchers,
  named: a substring matcher for the three existing needles, and an
  identifier-word matcher over `goad_boundary::scan::mentions` for AC-6's
  instrument (a). The existing three tests are unchanged in what they assert.
  See FD-2.
- EX-3 — the vacuity guard runs for **both** subject directories, and each new
  instrument asserts a non-zero inspected file count before asserting anything
  about occurrences.
- EX-4 — `PromptWindow` gains `in property <string> next-check` and one markup
  line rendering it below the diagnostic list (D-17). `glass.rs` writes it from
  `Frame::next_check` on **every** `present`, and writes `""` when the field is
  `None` — total, like every other property `present` writes. The line is
  **not** added to `diagnostic-lines`, and the empty-state sentinel's condition
  is untouched.
- EX-5 — no line number is pinned by either new instrument (D-16).

**Verification**
- VT-1 — `next_check_line` unit tests: an ordinary instant rendered to second
  precision; an instant at the edge of representable time that does not panic;
  the escape/bound pipeline applied.
- VT-2 — a renderer test asserting the line reaches the window's `next-check`
  property after an exchange and is `""` before one; that `Diagnostics::state()`
  is **unaffected** by it (a standing schedule is not a fault); and — the
  regression D-17 exists to prevent — that "Nothing to report." **still**
  renders on a clean outcome with a next check standing.
- VT-3 — **AC-6 (a).** No production line under `crates/goad/src` names the
  identifier `resolve`, matched as a word. **Measured on the tree at plan time:
  0 occurrences over 12 `.rs` files.** Assert the file count is non-zero in the
  same test.
- VT-4 — **AC-6 (b).** The path `schedule::resolve` occurs exactly **twice** in
  `crates/goad-shell/src`'s production code, both in `host.rs`. **Measured: 2
  over 8 `.rs` files**, at `host.rs:128` and `host.rs:259` — asserted as a count
  and a set of file names, never as line numbers.
- VT-5 — the counting controls for both, in `structure.rs`'s existing
  `mod counting_itself`: a line naming `resolve` only in a comment is not
  counted; a real `use goad_semantics::schedule::resolve;` **is** counted by the
  word matcher; and `resolved` is or is not counted, according to what EX-2
  settles — asserted either way, so the choice is on the page.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — break-and-revert on VT-3: add `use goad_semantics::schedule::{resolve,
  wait_for};` to `controller.rs`, confirm VT-3 goes red, revert, paste. Then the
  brace-grouped form specifically, which is the import F-3 says defeats a path
  grep. An absence assertion with no demonstrated failure is not evidence.
- VA-3 — break-and-revert on VT-4: add a third `schedule::resolve(` call site to
  `host.rs`, confirm the count goes to 3 and the test names the file, revert,
  paste.

**STOP**
- S-10 — the phase finds it needs any markup change beyond the one property and
  the one line D-17 authorises. `app.slint` is in scope for exactly that;
  anything more is a design change.
- S-11 — making VT-3 pass requires renaming a test function in
  `crates/goad/src/wire.rs`. D-16 rejects that explicitly: the cut at
  `#[cfg(test)]` is what removes those names, and if it is not working the cut
  is the defect.
- S-12 — either instrument needs `crates/goad-boundary/src/` to change.
  `design.md` §9 says both are configurations of machinery that already exists;
  if that is false it is a finding.
- S-24 — VT-2 needs a fixture that lives in `crates/goad/tests/renderer/
  harness.rs`. Stop and consult: `harness.rs` is PHASE-02's and is not a surface
  here, and moving a second fixture into it is a change to a file two phases
  now depend on.

**Notes for the implementer**

- `structure.rs:20`'s `SUBJECT_DIR` is closed over by `subject_files`,
  `occurrences_of` and the vacuity guard. Thread the directory through as a
  parameter rather than adding a second constant and a second copy of the walk.
- `goad_boundary::scan::mentions(line, token)` — the `fn` is `scan.rs:225`, its
  doc comment `:208-224` — already branches: a token containing `::` is matched
  as a substring, a bare word is split on every non-alphanumeric byte and at
  case boundaries and matched against the token or its plural. So `mentions(code, "schedule::resolve")` is
  instrument (b) exactly, and `mentions(code, "resolve")` is instrument (a) —
  but `mentions(code, "quit_event_loop(")` matches **nothing**, which is why the
  existing three needles keep the substring matcher.
- `mentions` calls `code_of` itself; `production_lines` has already applied it.
  Applying it twice is harmless on already-stripped text, but say so in a comment
  rather than leaving the next reader to work it out.
- Word-matching means `resolves` matches (plural) and `resolve_from` matches
  (split on `_`), which `design.md` §5.5 I-1a intends. `resolved` does **not**
  match, being neither the token nor its plural. Decide and assert.
- `Diagnostics` is **not** where the line goes (D-9), and `diagnostic-lines` is
  **not** where it is rendered (D-17): the first carries a fault bit, and the
  second is what the markup's empty-state sentinel is computed from. The line is
  `Frame`'s, and `Frame::next_check` already exists from PHASE-02/EX-2.
- `SlintGlass::present` is total by design — every property, every call
  (`glass.rs:16-24`). Writing `next-check` only when the field is `Some` would
  break that; write `""` for `None`.
- The tray tooltip is untouched (OQ-3's answer). Do not add the line to it.

---

## PHASE-05 — The topology

**Objective:** the waiting mechanism is proved in the arrangement production
uses, with exactly one component substituted and the substitution stated.

**Surfaces:** `crates/goad/Cargo.toml` (the `[[test]]` target only),
`crates/goad/tests/event_loop_schedule/{main.rs, scheduling.rs}`,
`tests/support/{driving.rs, scripting.rs}`,
`crates/goad/tests/renderer/{main.rs, wiring.rs, table.rs, scheduling.rs}`
(**include and import lines only**),
`crates/goad-shell/tests/integration/{main.rs, harness.rs, round_trip.rs,
failure_matrix.rs, transport.rs, host.rs}` (**include and import lines only**),
`docs/slices/003/notes.md`.

**Must not touch:** any file under any `src/`; `crates/goad/tests/event_loop/`;
`crates/goad/tests/renderer/harness.rs`; `tests/backends/`.

`harness.rs` is deliberately outside the split's blast radius: the only symbol
it takes from `tests/support/` is `instant`, which stays in `driving.rs`
(PHASE-02's note says so, and EX-1 keeps it there). The six renderer and
integration files listed above are the complete set that import a moved symbol;
`crates/goad-shell/tests/integration/fake.rs` is correctly excluded.

**Entry**
- EN-1 — PHASE-04's exit criteria are discharged and `just check` exits 0.

**Exit**
- EX-1 — `tests/support/scripting.rs` exists and holds exactly `backend`,
  `marker`, `clear`, `logging_backend`, `invocations` and `scripted`, moved from
  `driving.rs` **unchanged in body and signature**. `driving.rs` keeps the
  host-composition half. Two header edits are forced by the move and are the
  only changes to either file beyond it, both measured at HEAD: `driving.rs`
  **deletes `use std::path::{Path, PathBuf};`** at `:16`, whose only consumers
  are the five moving functions (`Path` at `:39`, `:59`, `:148`; `PathBuf` at
  `:50`, `:139`, `:171`), and `scripting.rs` carries that line plus
  `use goad_shell::config::Command;`. `driving.rs`'s other fourteen imports all
  keep a consumer and do not move — `Command` included, which stays for `config`
  (`:79`) and `host` (`:96`). This is F-13's class in its second instance
  (PL-14). Both
  files are included by the `renderer` and `integration` targets; the new target
  includes `scripting.rs` only. FD-3 is the reason and the phase sheet says so.
- EX-2 — every `pub(crate)` symbol in each shared file is reachable from every
  target that includes that file. This is not a style preference: `dead_code` is
  `warn` (`Cargo.toml:103`) and the gate's `-D warnings` promotes it to an error.
- EX-3 — `crates/goad/Cargo.toml` gains one `[[test]]` target, `name =
  "event_loop_schedule"`, `path = "tests/event_loop_schedule/main.rs"`. **No new
  dependency, no new feature, no `dev-dependency` change.**
- EX-4 — the new target carries exactly **one** `#[test]` fn, because
  `i_slint_backend_testing`'s init is once per process — the same reason
  `tests/event_loop/` holds one (D-12).
- EX-5 — the topology is production's in every component but the Slint platform:
  a multi-thread `tokio` runtime with `enable_all`, its `EnterGuard` held for the
  loop's life, a real `PromptWindow` and `Tray`, `install`'s callback table, a
  real `mpsc` channel and `Cancel`, `SlintGlass`, `ProcessBackend` against a real
  child, the production `serve`, and `slint::spawn_local`. The init is
  `init_integration_test_with_system_time()` — **not** `_with_mock_time`, which
  is `tests/event_loop/`'s and would stop tokio's timers being what is measured.
- EX-6 — the substitution is stated in the target's own module doc: the Slint
  platform is the testing backend's, no headless test can install the production
  one, and that is the single component AC-10 does not reach (A-1, F-8).

**Verification**
- VT-1 — **AC-10.** With `default_poll` at 100 ms and a scripted backend, a
  second invocation — unprompted, scheduled — is observed from inside a second
  `slint::spawn_local` task that awaits `tokio::time::sleep` between polls, and
  the watcher then trips `Cancel`; `serve` returns `Ending::Stopped` and
  `quit_event_loop` ends the loop. *Liveness: ~105 ms expected, `until(2 s)`.*
- VT-2 — the negative control: the test body cannot poll while
  `run_event_loop_until_quit` is running, so a version of VT-1 that observes from
  the test body would hang. Record, in the sheet, that the watcher-task shape is
  required and why — the shape spike S-1 ran (`research.md` Thread 5).
- VA-1 — `just check` under `nix develop`, pasted. It now runs seven test
  binaries in `crates/goad`; confirm the new one appears in the output.
- VA-2 — `git diff --stat` over `tests/support/` and both existing targets,
  confirming the move is a move: no symbol renamed, no body changed, only
  `#[path]` and `use` lines edited outside the two shared files. The expected
  diff inside `driving.rs` is the six functions leaving and **exactly one**
  further line, `use std::path::{Path, PathBuf};` at `:16` (EX-1). A second
  orphaned import there means the cut is not the one the plan measured — that is
  S-27, not a line to delete on the spot.
- VA-3 — VT-1's elapsed time, and `cargo test --workspace` wall time before and
  after, against `design.md` §9's AC-10 row (PL-11).

**STOP**
- S-13 — the new target needs a dependency, a feature, or a `dev-dependency` the
  workspace does not already carry. `slice-003.md` Scope forbids all three.
- S-14 — the split of `tests/support/` cannot be made without changing a helper's
  body. Then it is not a move, and PL-5's justification does not hold; stop and
  consult.
- S-15 — `init_integration_test_with_system_time()` is unavailable or behaves
  differently from what spike S-1 measured. A-1 is the assumption the whole
  design rests on; a surprise here is a finding, not something to work around.
- S-25 — `crates/goad/tests/renderer/harness.rs` turns out to import a symbol
  the split moves. Then this phase's surface list is wrong; stop and correct the
  plan rather than editing a file it does not declare.
- S-26 — a liveness margin measured in VA-3 is **below 5x**. Record and stop
  (PL-11).
- S-27 — the compiler reports an unused import in `tests/support/driving.rs`
  other than `use std::path::{Path, PathBuf};`, or an unused import in either
  existing target's files. Stop: the split is not the one EX-1 measured, and the
  bill is re-derived at plan rather than extended at the keyboard (F-13,
  PL-14).

**Notes for the implementer**

- `crates/goad/tests/event_loop/closing.rs:33-108` is the template for the whole
  arrangement, including the `Rc<RefCell<Option<Ending>>>` read-back across
  `spawn_local` and the body's shared thread. Copy the shape; change the init
  function, the backend, the `default_poll`, and add the watcher task.
- `closing.rs` builds its `Config` and `Host` **inline** and includes no shared
  helper. The new target does the same for the host half and includes only
  `scripting.rs` for the log half. That is FD-3's whole point.
- `#[path = "../../../../tests/support/scripting.rs"]` — four levels up from
  `crates/goad/tests/<target>/`, which is the repository root
  (`driving.rs`'s own header, and
  `docs/memory/cargo-test-cwd-is-package-root-not-workspace-root.md`).
- `harness.rs:29` re-exports `backend`, `clear` and `marker` from `driving`;
  that line moves to `scripting`.
- Mixing `init_no_event_loop()` and an integration-test backend in one binary is
  not an option (`crates/goad/tests/event_loop/main.rs:5-8`). The new target
  must not include any module that calls `init_no_event_loop()`.
- The watcher task and the serving task both run on the one Slint thread. Do not
  reach for `tokio::spawn` — `structure.rs`'s existing instrument forbids a
  `tokio::spawn` handle in `crates/goad/src`, and the same reasoning applies here
  by convention rather than by scan.

---

## PHASE-06 — Restatement, re-measurement, and the gate

**Objective:** every document in the slice folder is true about the tree, the
margin table is measured rather than estimated, and the gate is green from a
clean clone.

**Surfaces:** `docs/slices/003/{plan.md, notes.md, slice-003.md, draft-spec.md,
canon-delta.md}`. **No source file, no manifest, no markup.**

**Must not touch:** `docs/specs/`, `docs/policy/`, `docs/adr/`, `CLAUDE.md` —
all four are audit's, with user endorsement. `docs/slices/003/design.md` is a
record of intent and is **not** retro-fitted (`docs/AGENTS.md:137`).

**Entry**
- EN-1 — PHASE-05's exit criteria are discharged and `just check` exits 0.
- EN-2 — every phase from 01 to 05 is `done` in `notes.md`'s status table, each
  with its VA-1 output pasted.

**Exit**
- EX-1 — `draft-spec.md` (SPEC-002) is true about the tree: every requirement
  R-1..R-11's *verified by* row names a test that exists, by file and function
  name. Any requirement nothing holds is named as such rather than left to look
  discharged. This is now a **check** rather than a discovery: the plan's second
  Coverage table already maps each requirement to a criterion, and R-11 is
  already recorded as review rather than test. A requirement that reaches this
  phase unheld is a finding against the plan, not a hole to describe.
- EX-2 — `canon-delta.md` CD-1, CD-2 and CD-3 are re-read against what shipped.
  CD-1's R-56 must state the three kinds the tree actually emits.
- EX-3 — `slice-003.md`'s acceptance criteria each carry a pointer to the
  criterion that discharged them, matching this plan's Coverage table.
- EX-4 — `notes.md`'s Harvest is written: what now exists, what was learned, what
  is still open. Candidates for `docs/memory/` are named there; **the files
  themselves are written at close**, not here.
- EX-5 — the **measured** margin table is in `notes.md`, one row per timed
  assertion, with the expected time, the bound, the measured time and the
  measured ratio. It carries rows for PHASE-03/VT-5 and VT-6, which have no
  `design.md` §9 estimate and state their expectation from the plan instead.
  A **breached** margin — any liveness margin below 5x, or gate wall time more
  than 3 s above slice 002's 5.276 s baseline — is **S-19**, a STOP, not a note
  (PL-11, superseding PL-6). The measurement is recorded in `notes.md` and named
  there as a candidate follow-up; **nothing is written into `slice-003.md`**,
  whose Summary and Follow-ups are `docs/AGENTS.md` §Close's and are written
  once, after audit (F-10) — the same rule the note below already applies to
  `docs/memory/`.
- EX-6 — the `justfile` still mirrors POL-001's command block exactly, verified
  with `just -n check`.

**Verification**
- VA-1 — **AC-12.** `just check` exits 0 from a **clean clone** of the branch in
  a fresh `nix develop`, with the full output pasted. Six commands, none
  weakened, none conditioned, no `#[ignore]` anywhere in the slice's tests —
  asserted by `grep -rn "#\[ignore\]" crates/ tests/` returning nothing.
- VA-2 — **AC-11.** The vocabulary scan passes over the finished tree, and the
  new identifiers are walked by hand against it: `schedule`, `check`, `poll`,
  `spacing`, `scheduled`, `next_check` are host vocabulary, not domain
  vocabulary. Recorded, not assumed.
- VA-3 — the four ADR-001 instruments and the domain-vocabulary scan are each
  named in the sheet with their result, as five separate results and not one
  count (POL-001 §Verification, `CLAUDE.md`).
- VA-4 — three consecutive `just check` runs, all green, with the wall time of
  each recorded. This is R1's evidence: a timing test that is flaky under load is
  a design defect, and one green run does not distinguish the two.
- VA-5 — the paths actually touched across the slice, diffed against the surfaces
  each phase declared. Any undeclared path is written into `notes.md` as a
  finding for audit.

**STOP**
- S-16 — a document cannot be made true without changing code. That is a finding
  for audit, not a documentation edit.
- S-17 — the phase is tempted to edit `design.md` to match what shipped. It is a
  record of intent; divergence goes under *Design drift not reconciled* in
  `audit.md`.
- S-18 — a `just check` run is red or intermittently red. The slice is not
  closable; record and stop.
- S-19 — a collected margin is below its threshold: a liveness margin under 5x,
  or gate wall time more than 3 s above slice 002's 5.276 s baseline. **Stop and
  consult the orchestrator.** Do not widen a bound, shorten an expectation, or
  record it and pass over. `slice-003.md` AC-12 says a test whose passing
  depends on machine load is a design defect and not a tolerated cost, and
  PHASE-06 is the phase after which no phase can act, so a note here ships the
  defect (PL-11, F-6).

**Notes for the implementer**

- Slice 002's PHASE-09 is the precedent for this phase's shape.
- The clean clone matters because `flake.lock` is modified in the working tree at
  the slice's start; a green gate in a warm shell is not the same claim.
- `docs/memory/` files are lifted at **close**, from Harvest, per
  `docs/AGENTS.md:143`. This phase names the candidates and writes nothing into
  `docs/memory/`.
- Two candidates are already visible: FD-3's rule (a `#[path]`-shared test helper
  must have every symbol reachable from every includer, or `-D warnings` fails
  the gate — an extension of the existing memory file) and FD-2's (a scan built
  on `contains` and a scan built on `mentions` are different instruments and
  neither substitutes for the other).

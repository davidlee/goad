# Notes — Slice 003

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — The arithmetic, and the third stimulus | done | 2026-09-07 |
| PHASE-02 — The wait, and the cadence it keeps | pending | |
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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-07 · PHASE-01 done · `0b2e50f` (pre-commit; not yet
staged for this phase)

### Produced
- `goad_semantics::schedule::wait_for` — total, zero-arithmetic-operator wait
  computation, stratum 1 (PHASE-01/EX-1, EX-2).
- `Stimulus::Scheduled` at `crates/goad/src/wire.rs`, wired through `kind()`
  and `event()` (PHASE-01/EX-4). Nothing dispatches it yet — that is
  PHASE-02.

### Learned
- The design's exact `wait_for` body (`design.md:155-160`) compiles clean
  against `#![deny(clippy::arithmetic_side_effects)]` with no suppression:
  `jiff::Timestamp::duration_since` plus `TryFrom<SignedDuration> for
  std::time::Duration` really is total and operator-free, confirming A-2 and
  the implementer note without needing S-1.
- `schedule.rs:327`'s test doc comment still cites `(F-1)`, pre-dating this
  slice (slice 002, `ad811c6d`). It sits outside every phase's declared
  surface so far; worth a citation sweep before or at PHASE-06/close.

### Open
- Findings sweep at close: the stray `(F-1)` citation above.
<!-- Still unresolved at this point. Candidates for follow-ups. -->

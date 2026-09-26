# Slice 011: the refused arrival's present

**Stage:** done — closed 2026-09-26; audit and reconciliation in `audit.md`
**Tier:** 2 (full) — the slice amends canon: SPEC-003/R-15, and the
verification rows of R-15 and R-12, which record the present a refusal costs.
**Depends on:** —

## Purpose

An ingress arrival the host refuses while idle costs one full present. `ingest`
answers the writer and folds the refusal, `serve`'s outer loop `continue`s, and
the loop's first statement is `glass.present`. That present rebuilds the whole
row-model tree in `option_models` and discards it, allocates two `VecModel`s,
bumps the epoch, and — with the window up — runs `window.show()`'s
instantiation pass. What the refusal actually changed is the diagnostics lines
and the tray state.

The rate is the writer's. Slice 004 recorded ~1690 presents/s against a writer
emitting flat out, one per refusal. The writer is untrusted and need not
misbehave to reach this: arriving inside the spacing is enough.

What that costs has never been **run**. 009's F-R4 was reasoned from the tree,
and the two amplifiers it named — the per-present tray push (F-R5) and guard
revert (F-R3) — are gone. So this slice measures first, and repairs only what
the measurement shows is worth repairing.

Once it lands: refused arrivals, at any rate, cost the UI thread a bounded
number of presents per interval, and the last refusal decided while idle still
reaches the window within a stated bound — so a person debugging a watcher
still sees why it is being refused.

## Scope

**First, a measurement** — UI-thread cost per refused arrival on the running
host, against a writer flooding refusals, with the window up and hidden. Its
instrument and figures go in `research.md`. The user decides on it before
design proceeds (OQ-1).

**Code, if the measurement warrants it.**

- `serve` (`crates/goad/src/controller.rs`) — the outer loop's refusal path, and
  whatever bounds the presents it causes. It coalesces: a
  refusal-only change presents at once after quiet, then at most once per
  interval, trailing edge. After a refusal decided while idle, the surface is
  updated within the interval and shows what the diagnostics then hold. A
  refusal overwritten before that update is never shown (`design-log.md`,
  2026-09-26, and F-1/F-2).
- `crates/goad/tests/renderer/ingress.rs` — R-15's positive case, rewritten to
  read the window; new cases for the bound and the latest refusal under a flood,
  and for a command during a coalesced interval, all timed from a recording
  glass (`design.md` D12); one added assertion in `a_dead_accept_task_…`; and
  `ingress::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`,
  whose recorded ratio is the thing this slice changes. It keeps its R-12 half,
  renamed (`design.md` §9, D13).

**Canon.** `canon-delta.md` for SPEC-003: R-15's requirement (what a refusal
decided while idle is owed, and by when), its verification row — which reads
the retained model, not the window, so it would not report the change — and
R-12's verification row, which records *one presentation per refusal* as a
measured fact. SPEC-003 §6.3 has two statements that R-15's exceptions make
false, and its counts of causes are replaced. SPEC-002's R-12 row cites the same case, and only that citation
changes, to the renamed case.

**At close.** `docs/follow-ups.md` FU-2 — struck with what killed it, or
re-priced with the measurement if the slice concludes it is not worth doing.

## Non-goals

- **What the diagnostics slot retains (FU-3).** Ingress refusals replacing the
  whole slot, so a flood buries a backend failure, is a retention question.
  Coalescing slows the presents; it does not change what is kept. Its own
  slice unless the design shows the two cannot be separated.
- **Splitting `option_models`' single walk.** It reintroduces the second counter
  invariant **I-B** forbids (009 F-R4's Response).
- **A cheaper present in general** — diffing in `Glass::present`, or skipping
  `show()` on a visible window. It re-argues I-B and the `show()` totality
  argument 009 defended, and it bounds cost per present, not presents per
  second. Recorded as the rejected option in `design-log.md`.
- **The inner loop.** An arrival refused *during* an exchange already presents
  nothing (SPEC-003/R-15's second sentence, 004 `review-design.md` F-15). It
  does not change.
- **Accepted arrivals.** Their exchanges are already floored by the spacing
  (SPEC-002/R-12).
- **Deferral to the next present of any kind** (FU-2's own framing). With a
  `next_check` hours out, a person debugging a watcher would see nothing
  (`design-log.md`, 2026-09-26).

## Acceptance criteria

- [x] AC-1 — The UI-thread cost of one refused arrival is **measured on the
      running host** against a flooding writer, with the window up and hidden.
      The instrument, its denominator and what it does not reach are in
      `research.md`, and the user has decided on it before design proceeds.
- [x] AC-2 — Refused arrivals decided while idle, at any rate, cause at most a
      bounded number of presents per interval. A renderer case asserts the
      bound against a writer emitting flat out, and fails if every refusal
      presents. A second asserts that a lone refusal after quiet still
      presents at once (the leading edge).
- [x] AC-3 — The last refusal decided while idle reaches the **window** — not
      only the retained model — within the stated bound, and a case reads the
      window to show it.
- [x] AC-4 — Nothing else about the loop changes: an accepted arrival, a
      command, a scheduled firing and a refusal during an exchange present
      exactly as they did. Every existing case in
      `crates/goad/tests/renderer/ingress.rs` other than those AC-2 and AC-3
      supersede keeps every assertion it has. The one exception is
      `a_dead_accept_task_…`: it moves mechanically to the recording glass and
      gains one assertion, which guards the ingress-stopped path's loop label
      (`design.md` §9, R2; review F-10).
- [x] AC-5 — SPEC-003/R-15 states what a refusal decided while idle is owed and
      by when; its verification row names cases that read the window; R-12's
      row no longer records one presentation per refusal. Promoted from
      `canon-delta.md` with the user's endorsement.
- [x] AC-6 — A person floods refusals at the running host and sees the window
      stay responsive with the latest refusal on the diagnostics surface.
      Recorded in `audit.md` §Evidence (`docs/AGENTS.md` §Tiers).
- [x] AC-7 — `docs/follow-ups.md` FU-2 is struck with what killed it, or
      re-priced with AC-1's figures if the slice ends without a repair. The
      strike restates FU-2's *Dead when*, which a lone refusal after quiet
      still does not literally meet. The restated condition: refused arrivals
      cost at most one present per interval, and R-15 states the update
      guarantee (`design.md` §10; review F-14).

## Governing canon

**Binding.**

- **SPEC-003 (host event ingress)** — R-15 is amended; R-12's and R-15's
  verification rows change.
- **SPEC-002 (scheduling)** — R-12's spacing and ADR-004's floor must be
  untouched by whatever bounds the presents: a presentation deadline is not an
  evaluation and must not become one.
- **ADR-001 (one-way strata)** — the change is stratum 3 (`crates/goad`). No
  clock or timer reaches `goad-semantics`.
- **`docs/AGENTS.md` §Tiers** — tier 2, and a person runs the software before
  close.
- **`CLAUDE.md` §Working here** — name, never count; cite by symbol. R-15's
  amendment must not count the refusal kinds.

**Checked, not applicable.**

- **SPEC-001 (host/backend protocol)** — nothing crosses the backend boundary.
- **SPEC-004 (process exit status)** — no exit path changes.
- **ADR-003, ADR-005** — workspace shape and envelope normalization are
  untouched.
- **POL-001 (the phase gate)** — no new command or instrument.

## Open questions

- ~~OQ-1 — What does AC-1's measurement show, and is it worth repairing?~~
  **Repair.** The present is ~99 % of a refused arrival's UI-thread cost with
  the form up; one local writer pins the UI thread and delays a tray activation
  by 6.8–8.8 s (`research.md` at `c57b670`; `design-log.md`, 2026-09-26).
- ~~OQ-2 — The interval, and does R-15 state the number or the rule?~~
  **Both edges, 1 s.** The first refusal after quiet presents at once; a burst
  causes at most one further refusal-only present per interval, always the
  latest. R-15 states the rule; the number lives in code
  (`design-log.md`, 2026-09-26).
- ~~OQ-3 — Where the coalescing deadline lives.~~ **A second pinned `Sleep`,
  with its own arm just above ingress, inside a new `'idle` wait loop.** A
  refused arrival resumes waiting, and the top present stays unconditional.
  Neither anchor is touched (`design.md` D1–D4).
- ~~OQ-4 — The tray.~~ **The same bound.** One present writes both the window
  and the tray, and updating the tray on its own would need a partial present
  (`design.md` D9).
- ~~OQ-5 — An owed present when the loop leaves idle.~~ **What is owed is an
  update of a stale surface, not a particular refusal.** Any present that
  happens first shows whatever the diagnostics then hold: a diagnostics
  command, an edit, or the engage present. A refusal-site refusal or the
  ingress-stopped fold overwrites the stale fold before it is shown, and R-15
  names that exception. When the loop ends, the pending update is not made
  (`design.md` §5.4, D8; `design-log.md`, F-1/F-2).

## Summary

A refused arrival decided while idle no longer costs a present of its own.
AC-1's measurement on the running host showed the present was ~99 % of a
refused arrival's UI-thread cost, and one local writer pinned the UI thread
(`research.md`). `serve` now coalesces refusal-only updates of the surface on
both edges: the first refusal after quiet is shown at once, and a burst causes
at most one further update per `REFUSAL_PRESENT_INTERVAL` (1 s), always showing
the latest fold. It does this with a second pinned deadline,
`next_refusal_present`, and a `surface_stale` flag inside a new `'idle` wait
loop. No reply, command, scheduled firing or evaluation waits for it, and
neither scheduling anchor is touched.

The renderer cases now read the **window** through `RecordingGlass`, not the
retained `Diagnostics`, so R-15's verification observes what a person sees. T2
bounds a flood's updates by count and by gap and checks the last one names the
last refusal. T3 holds the leading edge, and T4 holds a command during a
coalesced interval. Each timed bound is held by a named mutation (`notes.md`
PHASE-02). The user ran a flood against the host and saw the window stay
responsive with the latest refusal shown (AC-6).

Canon: SPEC-003's R-15 states the update guarantee, the per-interval limit
and its exceptions (overwrite, loop end, clock overflow). §6.3 is made
consistent with it, and R-15's and R-12's verification rows cite the new
cases. SPEC-002's R-12 row follows the rename and names its independence
cases rather than counting them. The code review ran four rounds: 14 findings,
all verified, none blocking (`review-code.md`).

## Follow-ups

Promoted to `docs/follow-ups.md`:

- **FU-2 — struck.** This slice was FU-2. Its *Dead when*, restated as the
  design required (`design.md` §10), is met: refused arrivals cost at most one
  present per interval, and R-15 states the update guarantee.
- **FU-3 — citation extended** (`design.md` R4). Coalescing slows updates; it
  does not change retention, so a flood now shows fewer of its refusals and a
  refusal-site refusal can overwrite a stale fold before it is shown. That is
  the user's choice (`design-log.md`, F-1/F-2), and it leaves FU-3's question
  as it was. Not a repair here, because retention is a non-goal of this slice.
- **FU-45 — new** (`review-code.md` F-8). Two `Instant + MINIMUM_SPACING`
  additions in `serve` still panic on clock overflow, so R-15's clock-overflow
  clause describes a state the host cannot stay in. Pre-existing, practically
  unreachable, and the initial arm's fallback is a design choice rather than a
  repair — priced at tier 1.
- **FU-10 — citation extended** (`review-code.md` F-12). The workspace-internal
  `file.rs:NNN` citations this slice did not move remain outside the slice's
  surfaces. FU-10's *Dead when* already covers them.
- **FU-11 — citation extended.** `ingress.rs`'s `REFUSAL_PRESENT_INTERVAL` is a
  hand-copy of the private production constant, checked by nothing but its
  comment, as `FLOOR_MILLIS` is. T2(a) reds an interval shorter than about
  0.85 s, but between that and 1 s the value is held by review
  (`review-code.md`, *Risks knowingly left standing*).

`notes.md` §Open, each entry:

- **Design drift, `design.md` §9 T3's "M8 → R2"** — settled: recorded under
  `audit.md` *Design drift not reconciled*, with M8b as the control for R2.
- **FU-3's added citation** — promoted: FU-3's row cites 011.
- **Memory `a-refusal-is-recorded-not-shown`** — settled at close: amended so
  it is true of the promoted R-15 row (`docs/memory/`).

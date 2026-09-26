# Design log — Slice 011

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### 2026-09-26 — what shape the repair takes, and whether to measure first

- **Asked:** three shapes. (1) **Defer** — a refusal presents nothing, and is
  seen at the next present of any kind (FU-2's framing; R-15 weakened to "by
  the next present"). (2) **Coalesce** — a refusal-only change presents at most
  once per interval, trailing edge, so the last refusal is always shown; a
  second deadline in `serve`, R-15 amended to state the bound. (3) **Cheapen**
  — present only diagnostics and tray for a refusal, keeping it immediate; it
  re-argues invariant I-B and `show()`'s totality, and bounds cost per present
  rather than presents per second. And: measure the UI-thread cost on the
  running host first, since 009's F-R4 was reasoned and never run, and its two
  amplifiers (F-R3, F-R5) are gone.
- **Recommended:** measure first; lean (2). (1) leaves a person debugging a
  watcher blind until a `next_check` that may be hours out.
- **Decided:** "yes" — measure first, lean coalesce.
- **Consequence:** AC-1 is a measurement on the running host, and OQ-1 is a user
  gate on it before design proceeds. Deferral and a general cheaper present are
  non-goals in `slice-011.md`.

### 2026-09-26 — the autonomy grant

- **Asked:** the grant slices 003 and 005 ran under — the agent decides
  everything except canon. User gates: product questions touching canon, plan
  acceptance, and endorsement of the R-15 amendment (and any other canon edit).
- **Decided:** "yes".
- **Consequence:** the orchestrator decides design and plan questions that do
  not touch canon, and records them in this log as its own, marked as such.

### 2026-09-26 — OQ-1: is it worth repairing (the measurement gate)

- **Asked:** `research.md` Thread 3 — with the form up, the present is ~99 % of
  a refused arrival's UI-thread cost (~330 µs against ~4 µs with the present
  skipped); one local writer pins the UI thread at a full core, and a tray
  activation took 0.5–8.3 s to reach the window during a flood (≤0.02 s with
  the present skipped). Repair, or re-price FU-2 and stop?
- **Recommended:** repair; the control supports coalescing.
- **Decided:** repair (implicit in choosing the bound, next entry).
- **Consequence:** OQ-1 closed. FU-2's "no user-visible disturbance" is wrong
  on the real platform and is corrected at close.

### 2026-09-26 — OQ-2: the bound R-15 states

- **Asked:** (a) trailing edge only, at `MINIMUM_SPACING` (3 s) — even a lone
  refusal waits up to 3 s; or (b) leading and trailing edge at a short interval
  (1 s suggested) — the first refusal after quiet presents at once, as today,
  and a burst causes at most one further present per interval, always the
  latest.
- **Recommended:** (b) at 1 s; R-15 states the rule (a refusal decided while
  idle reaches the diagnostics surface within the interval; at most one
  refusal-only present per interval), the number lives in code as ADR-004's
  floor does.
- **Decided:** "b)"
- **Consequence:** the design coalesces on both edges at 1 s. R-15's amendment
  in `canon-delta.md` states the rule; its wording is endorsed at audit.

### 2026-09-26 — F-1, F-2: what R-15 promises when a refusal is overwritten

- **Asked:** `review-design.md` F-1 and F-2 — `Controller::refuse` replaces the
  whole retained `Diagnostics`, so an owed refusal is overwritten, unshown, by
  the next refusal (flood, refused command, ingress-stopped fold). The drafted
  R-15 promised every idle refusal reaches the surface within the interval,
  which the design cannot keep. Word R-15 around what the code guarantees — an
  update of the surface within the interval, showing the latest refusal — or
  widen the slice into retention (FU-3)?
- **Recommended:** the update guarantee; name the overwrite as the exception;
  retention stays FU-3's.
- **Decided:** "word R-15 around that update guarantee, name the overwrite as
  its exception, and leave retention to FU-3."
- **Consequence:** the owed flag means *the surface is stale*. R-15's draft in
  `canon-delta.md` is rewritten; FU-3's row gains this slice's citation at
  close (a flood now shows fewer refusals than before, by design).

### 2026-09-26 — M8's red lands on R1; M8b added (orchestrator, under the grant)

- **Found at PHASE-02:** M8 (F reset at every top present) reds T3 on R1, not
  R2 as `design.md` §9 predicted — T3's pin exchange presents at the top and
  moves F before R1 arrives.
- **Decided:** M8 stands as evidence, its label corrected in `notes.md`; M8b
  (F reset only on the no-exchange command exit) is added and must red T3 on
  R2, which is D5's rejected alternative exactly. `design.md` is not
  retro-fitted; audit lists the label under *Design drift not reconciled*.

### 2026-09-26 — audit: dispositions and canon endorsement (user)
- **Asked:** the dispositions for `review-code.md` round 1 (F-1..F-9, and the
  audit's A-1..A-3, entered as F-3, F-5 and F-10), and endorsement of
  `canon-delta.md` Changes 1–5, with one open question: SPEC-002's R-12 row
  counts "all three directions".
- **Decided:** "Approve all". F-8 is a follow-up, every other finding
  `fix-now`. Changes 1–5 endorsed, Change 3 as amended by F-1, and Change 5
  **extended** to name SPEC-002's independence cases rather than count them.
  Canon is applied at audit once round 2 has settled the wording.

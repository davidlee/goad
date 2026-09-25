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

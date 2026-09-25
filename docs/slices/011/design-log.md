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

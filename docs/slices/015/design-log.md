# Design log — Slice 015

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### 2026-10-19 — open a slice for W0-6 now, apart from 012

- **Asked:** whether 012's walk follow-ups become a 012 amendment or new
  slices, and in what order (012 `handover-satan-followups.md`).
- **Recommended:** new slices. 012's groups A and B (the checker's run,
  `goad-emit --config`) reverse decisions 012's design settled, and edit
  SPEC-004 drafts nothing outside 012 may cite, so they wait for 012's audit.
  W0-6 amends SPEC-001, which is canon, so it can start now, alone. Leave
  router conveniences (W1-6, W1-7, W1-9) to backends.
- **Decided:** "yeah" — this slice, W0-6 first.

### 2026-10-19 — the mechanism (OQ-1)

- **Asked:** spawn the backend in the config file's directory, or resolve only
  a relative program against it and leave the working directory alone.
- **Recommended:** the working directory. Only it fixes `["python3",
  "backend.py"]`, where the relative path is an argument. Cost: a backend's
  own relative paths move with it, which is what such an author expects.
- **Decided:** the working directory ("yeah, i think it makes the most
  sense").

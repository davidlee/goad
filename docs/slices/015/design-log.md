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

### 2026-10-19 — a linked config (OQ-2)

- **Asked:** when the config path is a symbolic link, is the directory the
  link's or the target's?
- **Recommended:** the target's, with every link in the path resolved
  (`std::fs::canonicalize`). The live config links to
  `~/satan-corpus/goad/router/config.toml`, whose neighbours the backend
  needs. Cost: a config linked into `/nix/store` gets a read-only working
  directory, and already has to use absolute paths.
- **Decided:** the target's directory.

### 2026-10-19 — `ingress.path` (OQ-3)

- **Asked:** does `ingress.path` resolve against the config's directory too?
- **Recommended:** yes. The kit's examples already assume it (`./goad.sock`),
  and `goad-emit` reads the path through `Config::load`, so one resolution
  there keeps the host and `goad-emit` agreeing. Noted: the socket path
  length limit.
- **Decided:** yes.

### 2026-10-19 — compatibility (OQ-4)

- **Asked:** does anything rely on today's rule, and does it need a way back?
- **Recommended:** no way back. Pre-1.0, one user; a second rule would be
  canon forever. `exercisers/demo.toml` is the only config in reach that
  breaks (its command and socket are relative to the repo root), and this
  slice repairs it with the `justfile`'s demo socket. The memory note on the
  repo-root socket and `path:` flake refs is re-checked once the socket moves.
- **Decided:** agree.

### 2026-10-19 — `goad-check`'s argv form (OQ-5)

- **Asked:** where does `goad-check -- PROGRAM` run its backend, with no
  config file? And where is that recorded: this slice's canon-delta after
  012 promotes SPEC-004, or a reconciliation row owed to 012's audit?
- **Recommended:** the caller's working directory, as now; recorded as a row
  owed to 012's audit.
- **Decided:** the caller's working directory, and the row ("#2 works for
  me").
- **Superseded on checking:** 012's SPEC-004 draft (`canon-delta.md`, R-11
  to R-15) holds `goad-check`'s exit statuses only, and no canon holds its
  command line. So no row is owed to 012. The rule goes in the kit's
  `checking.md`, which this slice edits. Told to the person on recording.

### 2026-10-19 — the design stage's grant

- **Asked:** may the design agent decide everything except canon wording, which
  returns to the person before review? And start design now?
- **Decided:** yes to both.

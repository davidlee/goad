# Design log — Slice 006

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

### 2026-09-20 — open the slice now, with 009 still in audit

- **Asked:** whether to open 006 while 009 is mid-flight, park it until 009
  closes, or treat the packaging work as out-of-band infrastructure that skips
  the `docs/AGENTS.md` lifecycle. The third was offered because it is the user's
  to declare and not the agent's to assume.
- **Recommended:** none of the three outright; the blocker named was that 009
  held a dirty tree in the files the startup-surface half would touch.
- **Decided:** *"009 is in the final throes of audit. let's open the slice."*
- **Consequence:** the lifecycle applies in full. The slice folder is created
  from the template. Execution was to wait on 009's close, the two overlapping
  in `crates/goad/src/main.rs` (`docs/memory/one-writer-per-worktree.md`); 009
  closed at `af76b4c` during this scoping conversation, so nothing waits.

### 2026-09-20 — scope is all four of the roadmap's bullets

- **Asked:** whether the slice takes two of `docs/roadmap.md` §006's bullets —
  the crane package and the home-manager module, which were what the user
  proposed and which touch no Rust — or all four, adding `--version` with the
  git sha and startup errors that name the path they tried.
- **Recommended:** initially **1, 2 and 4**, deferring the startup-error work on
  the ground that it touched files 009 held dirty and would cost design surface
  under the tier 1 cap. The user asked whether the errors needing a path were
  already enumerated. They were not written down anywhere; enumerating them
  showed the gap is **one variant of one enum** — `StartupError::Config`, since
  `Ingress` already names its path under `SPEC-003/R-3` and `R-4` and the other
  seven variants have no path to name — and that `goad-emit` already carries the
  repair's shape. The recommendation to defer was withdrawn as priced on cost
  that had not been measured (`docs/memory/price-the-rejected-option-against-code.md`).
- **Decided:** *"yes"* — all four, tier 1, with the module-home and
  gate-membership questions carried as open questions.
- **Consequence:** `slice-006.md` as written: scope names both the nix surfaces
  and `crates/goad/src/startup.rs`; AC-4, AC-5 and AC-6 are the startup-surface
  half; OQ-1 and OQ-2 carry what was deliberately left open, and OQ-2's answer
  is one of the two things that would raise the tier.

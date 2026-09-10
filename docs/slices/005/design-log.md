# Design log — Slice 005

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

### 2026-09-11 — clean the stale worktree before opening the slice

- **Asked:** `.claude/worktrees/wf_65e5549f-4e2-8` stood at `30d834f` (slice 001
  phase 13) with the 002 crate split staged and uncommitted, zero commits ahead
  of `main`. Remove it, or leave it?
- **Recommended:** remove — its staged content is the workspace split that
  closed in 002 and is on `main`, so nothing there is unmerged work.
- **Decided:** clean it up.
- **Consequence:** worktree removed; the two fully-merged branches `slice-002`
  and `worktree-wf_65e5549f-4e2-8` deleted; the stale `goad-demo.sock` and its
  `.lock` (gitignored, 2026-09-10) unlinked. One worktree, clean tree. Nothing
  about the slice's scope.

### 2026-09-11 — where `emit` lives

- **Asked:** a new member crate with its own binary; a second `[[bin]]` inside
  `goad-shell`; or an `emit` subcommand of the existing `goad` binary. The cost
  that decides it: `crates/goad` links Slint, so a subcommand makes every cron
  invocation link and start a GUI binary.
- **Recommended:** a new member crate, `crates/goad-emit`, depending on
  `goad-shell` only.
- **Decided:** new crate, own binary.
- **Consequence:** fires ADR-002's **T2** as ADR-002 predicted, under ADR-003's
  rules for what a member is. `Cargo.toml` gains one enumerated member entry.
  Raises OQ-3 — whether the new member gets a `goad-boundary` allowlist row, and
  whether answering that amends canon and so raises the tier.

### 2026-09-11 — the input surface

- **Asked:** four flags with `--data` optional; the same plus `--data -` reading
  stdin; or positional `source` and `kind`.
- **Recommended:** flags, `--data` optional — the smallest surface covering
  SPEC-003/§6.2's four fields, and no order that becomes contract.
- **Decided:** flags, `--data` optional.
- **Consequence:** AC-1. `--data -` is a **non-goal**, revisited in 007 only if
  quoting large JSON on a command line actually hurts. `timestamp` defaults to
  the moment of invocation; whether `--timestamp` exists at all is OQ-6, and
  the default answer is no.

### 2026-09-11 — what the exit code says

- **Asked:** three codes (0 accepted / 1 refused / 2 could not send); 0 and
  non-zero; or a distinct code per SPEC-003 reason token.
- **Recommended:** three codes, with the reason token and any `retry_after_ms`
  on stderr — a retry wrapper can branch without parsing prose, which R-14
  forbids for `detail`.
- **Decided:** 0 / 1 refused / 2 can't send.
- **Consequence:** AC-2 and AC-3. The exit-code contract stays decoupled from
  SPEC-003's reason set, so a ninth token would not be an exit-code change; the
  set is closed at eight today and an unknown token prints verbatim.

### 2026-09-11 — how the socket path is found

- **Asked:** read the host's configuration with `--socket` overriding; require
  `--socket` always; or configuration only, no override.
- **Recommended:** configuration with an override — one discovery rule shared
  with the host, and a way for tests and odd setups to point elsewhere.
- **Decided:** config, `--socket` overrides.
- **Consequence:** AC-4. `--socket` consults no configuration at all, so a
  broken config file cannot break an explicit path. Raises OQ-2: the discovery
  rule is `crates/goad/src/startup.rs::config_path` today, in a crate `emit` may
  not depend on, so it moves down to `goad-shell` or gets restated.

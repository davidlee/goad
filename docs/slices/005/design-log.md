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

### 2026-09-11 — the six open questions, answered in one pass

- **Asked:** OQ-1 argument parsing; OQ-2 where configuration-path discovery
  lives; OQ-3 whether the new member gets a `goad-boundary` allowlist row;
  OQ-4 how a CLI writes output under a workspace-wide `print_stderr` deny;
  OQ-5 `data`'s default and who parses `--data`; OQ-6 whether `--timestamp`
  exists. Each was put with its recommendation and the fact that decided it.
- **Recommended:** hand-rolled parsing on `startup::arguments`' shape; extract
  the *rule* (`config::default_path`) rather than the parser; **no allowlist
  row**, because POL-001 scopes that instrument to strata 1 and 2 and
  `goad-emit` is stratum 3, so a row would extend policy rather than apply it;
  move `line_to` down to `goad-shell`; `data` defaults to `null` and `--data`
  is parsed locally; no `--timestamp`.
- **Decided:** accept all six, including no allowlist row.
- **Consequence:** the slice **stays tier 1** — nothing here writes or amends
  canon. OQ-1..OQ-6 struck in `slice-005.md` with their answers; OQ-7 raised and
  answered there in the same pass (no `tokio` in emit — a blocking
  `std::os::unix::net::UnixStream` needs no runtime). Three corrections to
  `slice-005.md` fell out of the answers and are made: **AC-7** no longer claims
  an instrument holds emit's Slint-freedom — the crate edge does, and `cargo
  tree` is the audit evidence; **ADR-001's row** in Governing canon said the new
  member sits at stratum 2's level, which is wrong and is the error OQ-3 turned
  on — it is stratum 3, which ADR-001 §Decision names as *"entry points — the
  Slint renderer, command-line binaries"*; and **ADR-005** moves from *not
  applicable* to *binding as precedent*, because its reasoning — what decides a
  normalization's stratum is which contract it holds — is what places the
  reply's normalization in `goad-shell`. The gap the allowlist row would have
  closed is recorded under Follow-ups as stratum 3's, not this crate's.

- **Asked (in passing):** AC-6 said "reaches the backend", but no test target
  links both the CLI binary and a running host.
- **Decided:** by the agent, and recorded for the reviewer to contest — AC-6 is
  rephrased to what a test can actually see (the bytes a real invocation puts on
  a real socket normalize to the `Event` sent), and the backend leg is AC-8's
  human run. The alternative — keeping the wording and asserting something
  upstream of it — is the failure mode
  `docs/memory/a-green-test-can-assert-a-proxy.md` records from slice 004.

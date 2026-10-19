# Slice 015: a config travels with its backend

**Stage:** design
**Tier:** 2 (full) — it adds a rule to SPEC-001, which states nothing today
about where a backend command runs.
**Depends on:** — for canon. Its `kit/` edits wait for slice 012's PHASE-10,
which may edit the same files (one writer per worktree).

## Purpose

A goad config cannot sit beside its backend and be used from anywhere. The
command `["python3", "backend.py"]` runs with goad's own working directory, so
`backend.py` is found only when goad is started in the config's directory.
A service, a desktop launcher, or `~/.config/goad/config.toml` linking to the
file all start goad elsewhere, so every config SATAN wrote uses absolute paths
(012 `walks.md` W0-6).

Once this lands, a config and its backend can move together as one directory,
and a relative path in `command` means the same thing however goad was
started.

SPEC-001/R-36 says only that `command` is an argv vector with no shell. Where
it resolves is unstated: today it is whatever a no-shell spawn does, which the
kit documents (`running.md`, *SPEC-001/R-36*) and 012's design relies on
(`design.md` §5.2.1, F-36).

**The relative path is usually an argument, not the program.** Rewriting argv
against the config's directory would mean guessing which arguments are paths,
which the permissive-parsing invariant forbids. The candidate mechanism is
therefore the backend's **working directory**: spawn it in the config file's
directory (OQ-1, settled).

## Scope

- SPEC-001 §6.4 *Process transport*, and a new or amended requirement beside
  R-36, through `canon-delta.md`.
- `goad_shell::backend::process` (the spawn) and `goad_shell::config` (where
  the config's directory is known, `Config::load`).
- `goad-check`'s config form, which spawns as the host does and so inherits
  the change. Its argv form (`-- PROGRAM`) has no config file.
- `goad-emit`, which reads `ingress.path` through `Config::load` when
  `--socket` is absent. Its command line does not change.
- The kit's prose that states today's rule: `running.md` (*SPEC-001/R-36*,
  and its `ingress.path` row), `checking.md`, and the examples' configs and
  READMEs that say to run goad from the example's directory.
- `exercisers/demo.toml` (its `command` and socket path become relative to
  `exercisers/`) and the `justfile` recipes that name the demo's socket.
- SATAN's configs in `~/satan-corpus/goad/` are outside this repository. They
  can drop their absolute paths once this ships; that is their owner's edit.

## Non-goals

- **Rewriting argv.** No argument is reinterpreted as a path (see Purpose).
- **Group A and B from 012's handover** (`goad-check`'s run, `goad-emit
  --config`). They change SPEC-004 drafts that 012's audit has not yet
  promoted, and wait for it.
- **Making routers easier** (W1-6, W1-7, W1-9). Left to backends. A kit
  example router may come later, with whichever slice next owns `kit/`.

## Acceptance criteria

- [ ] AC-1 — A config whose `command` holds a relative path to its backend
  works when goad is started from another directory, including when goad
  reaches the config through a symbolic link (the live setup: OQ-2).
- [ ] AC-2 — `goad-check --config` behaves the same way, so a check run from
  any directory exercises what the host will run.
- [ ] AC-3 — SPEC-001 states the rule as a requirement with a §Verification
  row, and the kit states it without the current *make every path absolute*
  advice.
- [ ] AC-4 — A person has run goad with a relative-path config, from a
  directory other than the config's, and seen a view.
- [ ] AC-5 — A relative `ingress.path` names the same socket for goad and
  for `goad-emit` without `--socket`, wherever each is started.

## Governing canon

- **SPEC-001** — host/backend protocol: R-36 (argv, no shell), R-44 (a program
  the host cannot start is the configuration's side), §6.4 process transport.
  Amended.
- **SPEC-003** — ingress: §6.1 says the socket path comes from the
  configuration. Amended to say what a relative path is relative to (OQ-3).
  R-4 covers a path that cannot be bound.
- **SPEC-004** — exit status. Checked: a spawn failure's status is unchanged.
  012's unpromoted SPEC-004 drafts are not cited (`docs/AGENTS.md`, *Canon
  that does not exist yet*).
- **ADR-001** — strata. The spawn lives in stratum 2 (`goad-shell`); nothing
  moves into `goad-semantics`.

## Open questions

- ~~OQ-1 — **The mechanism.** Spawn the backend with its working directory
  set to the config's directory, or resolve only a relative *program* against
  it and leave the working directory alone?~~ **The working directory**
  (`design-log.md` 2026-10-19, *the mechanism*): it fixes the common case,
  where the relative path is an argument. A backend's own relative paths then
  resolve against the config's directory too, which is what an author writing
  relative paths expects. Left for design: a bare program name (`python3`)
  still resolves on `PATH`, and a relative program with a slash
  (`./backend.py`) must mean the config's directory unambiguously. Rust's
  `Command::current_dir` leaves that case platform-specific, so the host
  likely resolves such a program itself.
- ~~OQ-2 — **Which directory when the config is a symbolic link?**~~ **The
  target's**: the config path is resolved, every link in it, and its parent
  is the directory (`design-log.md` 2026-10-19, *a linked config*). The live
  setup needs it: `~/.config/goad/config.toml` links to `router/config.toml`.
  Cost: a config linked into `/nix/store` gives its backend a read-only
  working directory; such a config already needs absolute paths.
- ~~OQ-3 — **`ingress.path` too?**~~ **Yes** (`design-log.md` 2026-10-19,
  *`ingress.path`*). The kit's examples already write `./goad.sock` beside the
  config, and `goad-emit` reads the path through the same `Config::load`, so
  one resolution there gives every binary one rule. Left for design: a
  socket path's length limit (about 108 bytes), which the absolute form may
  exceed where the relative one did not.
- ~~OQ-4 — **Compatibility.**~~ **No way back** (`design-log.md` 2026-10-19,
  *compatibility*). The one config in reach that relies on the old rule is
  `exercisers/demo.toml`, and this slice repairs it. SATAN's configs are
  absolute and the kit's examples are run from their own directory, so
  neither changes meaning. The kit says the rule changed. Left for design:
  which tests that spawn with relative paths relied on the old rule.
- ~~OQ-5 — **The argv form of `goad-check`.**~~ **The caller's working
  directory, as now** (`design-log.md` 2026-10-19, *`goad-check`'s argv
  form*). With no config file there is no other candidate. No canon holds
  `goad-check`'s command line, drafted or promoted (012's SPEC-004 draft is
  its exit statuses), so the rule is stated in `checking.md`, which this
  slice already edits.

## Summary

<!-- Written at close: what actually landed, in three or four lines. -->

## Follow-ups

<!-- Deferred work surfaced by this slice. Each becomes a future slice or a
     line in a spec. -->

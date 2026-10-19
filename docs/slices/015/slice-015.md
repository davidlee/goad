# Slice 015: a config travels with its backend

**Stage:** done — closed 2026-10-19; audit and reconciliation in `audit.md`
**Tier:** 2 (full) — it adds a rule to SPEC-001, which states nothing today
about where a backend command runs.
**Depends on:** nothing, to ship. Code and its kit and exerciser prose land
together in one merge from this slice's own branch and worktree, and that
merge goes to `main` when the slice ships, without waiting on slice 012's
PHASE-10 if 012 has not moved; only a 012 walk actually in flight — PHASE-09
or PHASE-10 — is coordinated with (`review-design.md` F-20), and PHASE-10
also as a writer of the same files. Promotion after 012's is a
preference: if this slice closes first it promotes its own delta, and
SPEC-001 carries R-60 with no R-59 until 012's audit (`design-log.md`
2026-10-19, *design review round 1 dispositions*, F-10).

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
  and its `ingress.path` row), `checking.md`, `events.md` (`goad-emit`'s
  socket rule), the examples' configs and READMEs that say to run goad from
  the example's directory (they keep *copy this directory out*, with its new
  reason), and `downloads-triage/watch.sh`'s header.
- `goad-shell`'s `BackendError::Spawn`, which gains the executable and the
  working directory (`design.md` D13), and the tests that match or build it.
- The `goad-check` kit tier's docs that AC-2's change makes false
  (`tests/kit/main.rs`, `examples.rs`, `tree.rs`), and `.gitignore`'s
  kit-example paragraph.
- `exercisers/demo.toml` (its `command` and socket path become relative to
  `exercisers/`), the `justfile` recipe and comment that name the demo's
  socket, and `.gitignore`'s entries for it.
- `exercisers/typescript/README.md`, whose config is repo-root relative and
  whose prose states today's rule, and `exercisers/shell/backend.sh`'s header,
  which quotes the demo's command.
- `docs/memory/path-flake-ref-breaks-on-demo-socket.md`, which names the
  socket's location.
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

- [x] AC-1 — A config whose `command` holds a relative path to its backend
  works when goad is started from another directory, including when goad
  reaches the config through a symbolic link (the live setup: OQ-2).
- [x] AC-2 — `goad-check --config` behaves the same way, so a check run from
  any directory exercises what the host will run.
- [x] AC-3 — SPEC-001 states the rule as a requirement with a §Verification
  row, and the kit states it without the current *make every path absolute*
  advice.
- [x] AC-4 — A person has run goad with a relative-path config, from a
  directory other than the config's, and seen a view.
- [x] AC-5 — A relative `ingress.path` names the same socket for goad and
  for `goad-emit` without `--socket`, wherever each is started.

## Governing canon

- **SPEC-001** — host/backend protocol: R-36 (argv, no shell), R-44 (a program
  the host cannot start is the configuration's side), §6.4 process transport.
  Amended: new R-60 (R-59 is slice 012's pending id), §6.4, §7.
- **SPEC-003** — ingress: §6.1 says the socket path comes from the
  configuration. Amended to say what a relative path is relative to (OQ-3):
  new R-17, §6.1, §7, §9. R-4 covers a path that cannot be bound, the
  too-long one included.
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

A configuration and its backend now move together as one directory.
`Config::load` canonicalizes the file's path, every link in it, and anchors
the loaded `Command` and a relative `[ingress] path` at the file's own
directory. `ProcessBackend::exchange` runs the backend there with `PWD` set to
it, and executes a relative program that contains a `/` by its joined path,
which is also the name it receives. A spawn failure names the executable and
the directory (`BackendError::Spawn`). goad, `goad-check --config` and
`goad-emit` without `--socket` all go through the same `Config::load`, so they
agree wherever each is started; `goad-check`'s argv form keeps the caller's
directory.

Canon: SPEC-001 gained R-60, §6.4 *Where the backend runs* and R-60's §7 row;
SPEC-003 gained R-17, the rule and the socket-address limit in §6.1, R-17's §7
row, and R-60 in §9. SPEC-001 carries R-60 with no R-59 until slice 012's
audit. The kit, the examples, `exercisers/demo.toml` and the `justfile` state
the new rule and drop the *start goad in this directory* advice. A person ran
goad on the demo from `/tmp`, directly and through a link, and reached it with
`just emit` (AC-4, AC-5). Code review ran two rounds: F-1 to F-13, all
verified, none a behavioural defect (`review-code.md`).

## Follow-ups

Promoted to `docs/follow-ups.md`; each one extends a row that already raised
the claim, and no new row was needed:

- **FU-11 — citation extended** (`design.md` D11). `goad`'s own `main.rs::start`
  is held by review and AC-4, not a test. Not a repair: the design rejected a
  binary-tier `goad` case because the binary reaches a window before the
  spawn, and the path it would test is the `Config::load` → `ProcessBackend`
  path the `goad-shell` integration cases and the built `goad-check` already
  hold. Priced at tier 1 plus a harness, which is FU-11's shape.
- **FU-10 — citation extended** (`review-code.md` F-11). Line-number citations
  remain in files this slice never touched (`goad`'s `canonical.rs`,
  `view_model.rs`, `lib.rs`). Every file the slice touched was swept to zero;
  the rest are outside its surfaces, and FU-10's *Dead when* covers them.
- **FU-5 — citation extended** (`notes.md` PHASE-01 *Decisions*).
  `goad-emit`'s `exchange.rs::config_home` is one more unique-path helper
  `claim` does not hold, for FU-5's mechanical reason: the target does not
  include `tests/support/scripting.rs`. Priced at FU-5's tier 1; moving `claim`
  was outside every phase's surfaces.
- **FU-18 — citation extended** (`design.md` I-3). A relative `ingress.path`
  now agrees for the default file wherever goad and `goad-emit` start, but a
  goad started on any other file is still reached only with `--socket`. A
  non-goal here (slice 012's group B, behind 012's audit), so not a repair.

Re-verified, by each row's own *Dead when*, the rows naming a file this slice
touched: FU-5, FU-6, FU-10, FU-11, FU-14, FU-18, FU-35, FU-37, FU-42, FU-43.
None was killed by this slice. **FU-43's condition is met**, by slice 012
PHASE-03 (`a02165b`), not by this slice; the row says so and is left for 012's
close to strike.

`notes.md` §Open, each entry (the boxes there cite the evidence):

- **Promote `canon-delta.md`**, **re-point the §7 citations**, **the kit's
  citation of the new ids**, **record the person's run** — settled at audit
  (`audit.md` Reconciliation and §Evidence).
- **Slice 012's drift row** — carried to 012: `docs/slices/012/plan.md` §Owed
  holds it, and 012's audit discharges it.
- **D4 — a configuration named by a descriptor that is not a file** (a pipe, a
  socket, a process substitution) is refused, not anchored — settled, not
  promoted: tolerated by design because nothing in the tree, the kit or canon
  uses the form, and a fallback would be a second rule for where a backend
  runs. `ConfigError::Read`'s doc states it (`review-code.md` F-2).
- **§5.5 — a relative `PATH` entry now means the config's directory** —
  settled, not promoted: SPEC-001 §6.4 and `running.md` state it as the
  environment's choice. Code review round 1 verified it against std's source
  and the built `goad-check`. A case would pin std's lookup order, not
  anything the host decides.
- **D11**, the **line-number citations**, **`config_home`** and **I-3** —
  promoted, above.
- **The audit-bound findings from PHASE-01..03** — settled by
  `review-code.md` F-4 and F-9, and by promotion for the ahead-of-promotion
  citations.
- **The memory candidate** — harvested
  (`docs/memory/path-equality-ignores-an-interior-dot.md`).
- **The Owed box** — still open, and the person's: after the merge, delete
  `goad-demo.sock` and `goad-demo.sock.lock` at the main checkout's root.

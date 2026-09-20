# Slice 006: packaging and the startup surface

**Stage:** planned
**Tier:** 1 (thin) — see *What would raise the tier* below. `design.md` runs
46 lines over the tier 1 cap by explicit user decision, recorded at its head and
in `design-log.md`; nothing else about the tier changes.
**Depends on:** nothing. Independent of 007, 008 and 009 by design. It was
sequenced after them because 009 held `crates/goad/src/main.rs` dirty and this
repository runs one writer per worktree; 009 closed at `af76b4c` while this
slice was being scoped, so that constraint is discharged.

## Purpose

Everything between *built* and *running daily*.

What runs daily today is whatever the working tree last compiled, installed by
a recipe that is honest about being a stopgap. Three things are wrong with it,
and each has been met in practice rather than anticipated:

- **The binary and its environment are two things that must be installed
  together.** The GUI libraries are `dlopen`'d rather than linked, so `ldd`
  resolves clean and the binary still opens no window without
  `LD_LIBRARY_PATH`; fontconfig finds fonts through a configuration file, so
  `FONTCONFIG_FILE` is the second half and its absence is a window that draws
  no text. `just install` captures both into `~/.config/goad/env` and the unit
  reads them back. Capturing one and not the other fails invisibly, found
  whenever a window next happens to draw.
- **The unit is a hand-written file outside the repository** —
  `~/satan/goad/goad.service`, symlinked into `~/.config/systemd/user/`. It is
  version-controlled in the sense that a file is, and nothing rebuilds it from
  a clean clone.
- **Two install paths can both put a `goad` on `$PATH` and nothing can say
  which ran.** `goad --version` is read as a configuration path and fails
  opening a file called `--version`.

A fourth thing is adjacent and shares the slice because it is the same
question — *which file did it mean?* — asked at startup instead of at install:
**a startup failure that has a path does not always name it.** `goad
/nonexistent/wat.toml` reports *configuration could not be read: No such file
or directory (os error 2)*: which side was wrong, but not which file it tried.
The case that most needs the path told to it is the one where nobody typed it,
because the default path is computed from the environment.

Once this lands: `nix build` produces a binary that is self-contained
anywhere, a module builds the unit from a store path, and the running host can
say what it is.

## Scope

- `flake.nix` — crane, the source filter, the package outputs, the wrapper, the
  revision, and the module export.
- `nix/module.nix` — new; the systemd user unit. OQ-1 put it here.
- `justfile` — a new `package` recipe (OQ-2b). The `install` recipe is
  unchanged; its env half is kept (OQ-6).
- `Cargo.toml` — the workspace manifest's two-line `tokio` entry, joined onto
  one line. A precondition of evaluating anything with crane, not a preference
  (`research.md` Thread 3 S-1).
- `crates/goad/src/startup.rs` — `Launch`, `arguments` and its table, and
  `StartupError::Config` replaced by two path-carrying arms.
- `crates/goad/src/main.rs` — the `--version` destination beside `--help`'s, and
  `start`'s two-arm split of `Config::load`.
- `crates/goad/src/diagnostics.rs` — `version_line`, and `USAGE`'s third form.
- `crates/goad/Cargo.toml` and `crates/goad/tests/binary/` — new; a binary test
  target, which `goad-emit` has and `goad` does not.
- `crates/goad-emit/src/main.rs`, `crates/goad-emit/src/render.rs` — emit's
  `--version` takes the same shape (OQ-7).
- `crates/goad/tests/renderer/startup.rs` — the argument table and the display
  text are both already tested there.
- `docs/roadmap.md` — at close.

Checked and **not** in scope, both of which design closed: `crates/goad/build.rs`
(OQ-4 takes `option_env!`, not a build script) and
`crates/goad-shell/src/error.rs` (OQ-3 kept the path at stratum 3, so
`ConfigError` is untouched).

Outside the repository and required as evidence rather than as a deliverable:
the `~/flakes` wiring that consumes whatever this slice exports.

## Non-goals

- **`cargo install` is not retired.** Both paths work from the same
  `Cargo.toml`, as they do in `~/dev/doctrine`. Retiring one is what makes
  `--version` pointless, and the daily-driver path must keep working while the
  nix one is proven.
- **The backend is not packaged.** `~/satan/goad` is one person's evidence and
  one person's domain. A host that packaged its backend would be a host that
  understood the domain.
- **No NixOS module, and no darwin.** A user unit on one machine. A system
  service is a different lifetime and a different question.
- **`goad-emit`'s startup diagnostics are not reopened.** It already carries
  the shape this slice gives `goad` — `StartupFault::Unparseable { path, fault
  }` — and is the prior art, not the work. Its **`--version` is in scope**:
  OQ-7 found the same *which install is running?* defect in emit, which
  `just install` installs and AC-2 packages, and `--version` is not a startup
  diagnostic.
- **No `--config PATH` flag.** It is a standing 005 follow-up and it is a
  second way to name the file this slice is about naming. One at a time.
- **No release, no CI, no crates.io.** `publish = false` is not provisional.

## Acceptance criteria

- [ ] AC-1 — `nix build .#goad` succeeds from a clean checkout, and the binary
      it produces opens a window **with text in it** when run from a shell that
      has neither `LD_LIBRARY_PATH` nor `FONTCONFIG_FILE` set. Both halves are
      named because capturing one and not the other is the defect this replaces.
- [ ] AC-2 — `nix build .#goad-emit` succeeds, and the binary emits an envelope
      into a running host's socket.
- [ ] AC-3 — Nothing a caller sets in the environment is required by either
      binary. Stated as an absence because that is what `wrapProgram` buys, and
      the env file is what it retires.
- [ ] AC-4 — `goad --version` prints, on stdout and exit 0, the package version
      and — **when the build stamped one** — the git revision it came from; and
      is not read as a configuration path. A `cargo install`ed binary stamps no
      revision and prints the bare version, which is what AC-5 rests on
      (OQ-4, OQ-7; `design.md` D3).
- [ ] AC-5 — A nix-built `goad` and a `cargo install`ed `goad` can be told
      apart from their `--version` output alone.
- [ ] AC-6 — Every `StartupError` that has a path in hand names it. Today
      exactly one does not: `Config`. `Ingress` already does and MUST continue
      to — `SPEC-003/R-3` and `R-4` require it.
- [ ] AC-7 — A home-manager module produces the unit, and the unit keeps the
      restart semantics the hand-written one argued for: `Restart=on-failure`
      with `RestartPreventExitStatus=2`, because exit 2 is every `StartupError`
      and none of them succeeds on a retry.
- [ ] AC-8 — `just check` exits 0, **and** a person has run the nix-built
      binary under systemd and seen the window (`docs/AGENTS.md` §Tiers: a green
      gate is not that evidence).
- [ ] AC-9 — `just install` still installs a working pair, or is changed by an
      explicit decision recorded in `design-log.md` rather than by attrition.

## Governing canon

Binding:

- **POL-001** — the phase gate. The slice adds a build path the gate does not
  run, and **it stays out of it**: OQ-2 was answered no, so this policy is not
  amended and the tier stays 1. What binds is the prohibition itself — no
  command may be removed, weakened or made conditional, and nothing this slice
  does to the `justfile` may touch the six the block names. The new `package`
  recipe (OQ-2b) sits outside the block and carries no standing obligation, so
  it amends nothing.
- **ADR-001** — one-way strata. The startup-surface half is stratum 3 work.
  OQ-3 asked whether the path belonged in stratum 2's error type instead and
  was **answered at stratum 3**, so `goad_shell` is untouched and nothing here
  crosses a stratum in a new direction.
- **SPEC-003** R-3, R-4 — ingress startup failures name the path, and say what
  was found. This slice touches `StartupError`'s `Display` and must not weaken
  them; AC-6 states the obligation in the direction of the change.
- **`CLAUDE.md`'s vocabulary invariant** — the module and the unit are host
  packaging and must name no domain. The boundary scan does not read `.nix`.

Checked and not applicable: **SPEC-001** and **SPEC-002** (no protocol, no
scheduling change; nothing here is on the wire); **ADR-004**, **ADR-005** (the
same); **ADR-002**, **ADR-003** (no new workspace member — `nix/` is not a
crate, and no binary is added or moved between strata).

## Open questions

**All answered.** Each was decided in the design interview and is recorded in
`design-log.md` with its argument; `design.md` §7 carries the decision and the
rejected alternative. Kept here as the slice's own record of what was open.

| | question | answer |
|---|---|---|
| OQ-1 | where the home-manager module lives | **in this repository** — `nix/module.nix`, exported as `homeManagerModules.default`; `~/flakes` gets the four-line consumer. `RestartPreventExitStatus=2` is this repository's exit-code contract. |
| OQ-2 | does `nix build` join the phase gate | **no.** POL-001 untouched, tier stays 1. |
| OQ-2b | the carried third option — a `just package` recipe | **yes**, outside the gate and with no standing obligation on future slices. An obligation would have been canon, and tier 2. |
| OQ-3 | which stratum names the configuration path | **stratum 3.** `ConfigError` is also emit's; a path inside it prints twice there, and repairing that is a non-goal. |
| OQ-4 | where the revision comes from | **the flake stamps `GOAD_REVISION`**; the `cargo install` path stamps none. Both stamping a real sha would make the two indistinguishable, failing AC-5. |
| OQ-5 | does the package build run the tests | **no** — `doCheck = false`, on three measured obstacles (`research.md` S-3). `just check` is what holds the tests. |
| OQ-6 | what happens to `~/.config/goad/env` | **kept.** goad must keep running on non-NixOS systems. Its reader becomes a person; nothing reads it automatically once the module carries no `EnvironmentFile`. |
| OQ-7 | raised in research — does `goad-emit --version` change shape too | **yes**, the same shape. The *which install is running?* defect is equally emit's. |

One question raised during the interview was **deferred rather than answered**:
whether goad's non-NixOS story needs more than `cargo install` — documentation,
and something that verifies it. It is a follow-up, held in `notes.md` §Open.

## What would raise the tier

Tier 1 as opened, and **tier 1 at design acceptance**: nothing here writes or
amends canon, and nothing changes the wire contract. Both routes to tier 2 that
were open are now closed:

- **OQ-2 answered yes** would have amended POL-001. It was answered **no**.
  OQ-2b's `package` recipe sits outside the gate block and carries no standing
  obligation, so it does not reopen this.
- **OQ-3 answered in stratum 2** would have changed a type `goad-emit` also
  consumes. It was answered **stratum 3**.

A third pressure appeared instead and was **declined**: `design.md` exceeds the
tier 1 line cap, and the user chose to run over rather than split the slice or
raise the tier (`design-log.md`, 2026-09-20). That is a deviation from
`docs/AGENTS.md` §Tiers taken on explicit instruction, not a tier change — the
two-round bound on the shared design-and-plan ledger is unchanged.

A tier may be raised mid-slice and never lowered.

## Before design starts

Facts already paid for, which design should not rediscover:

- **The flake reference must be the bare/git form, not `path:`.**
  `docs/memory/path-flake-ref-breaks-on-demo-socket.md` — the repository root
  holds `goad-demo.sock` by design (slice 004 D-17), and a `path:` reference
  copies the whole directory into the store, which refuses a socket. The cost
  is that an uncommitted `flake.nix` is invisible to a consumer, so iterate
  with `nix build` in the repository and commit before switching.
- **crane's `cleanCargoSource` keeps only `.rs`, `.toml` and `.lock`.**
  `crates/goad/build.rs` compiles `crates/goad/ui/app.slint`, which `import`s
  `../../../assets/Inter.ttf` and `Geist.ttf`. All three would be stripped.
  `~/dev/doctrine`'s flake grafts stripped assets back after the fact because
  there the loss is silent; here `build.rs` fails, so a widened source filter is
  the cheaper shape. Verify rather than assume — this is prior art, not a rule.
- **The two fonts are tracked past a gitignore.** `assets/.gitignore` says
  `*.ttf`; `Inter.ttf` and `Geist.ttf` are force-added and are exactly the two
  `app.slint` imports. `assets/static/*.ttf` is untracked and unused by the
  build. A git-input reference therefore sees what the build needs — and would
  stop doing so if a third face were added without being force-added too.
- **fontconfig needs a configuration file, not a package on a path.**
  `docs/memory/fontconfig-needs-makefontsconf-not-just-buildinputs.md`.
  `flake.nix` already has `guiLibs` and `fontsConf` to hand a wrapper.
- **Prior art, read before proposing:** `~/dev/doctrine/flake.nix` (crane over a
  workspace, the toolchain shared with the devshell, `doCheck = false`);
  `~/flakes/modules/home/linux/satan-attrd.nix` and `behaviour.nix` (the two
  module patterns OQ-1 chooses between);
  `~/dev/satan-attrd/nix/module.nix` (a module's option surface).

## Summary

<!-- Written at close: what actually landed, in three or four lines. -->

## Follow-ups

<!-- Deferred work surfaced by this slice. Each becomes a future slice or a
     line in a spec. -->

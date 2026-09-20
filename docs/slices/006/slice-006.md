# Slice 006: packaging and the startup surface

**Stage:** scoping
**Tier:** 1 (thin) — see *What would raise the tier* below.
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

- `flake.nix` — crane, the source filter, the package outputs, the wrapper, and
  the module export.
- `nix/module.nix` — new; the systemd user unit, if OQ-1 puts it here.
- `justfile` — the `install` recipe and whatever survives of its env half.
- `crates/goad/src/startup.rs` — `Launch`, `arguments` and its table,
  `StartupError::Config`.
- `crates/goad/src/main.rs` — the `--version` destination, beside `--help`'s.
- `crates/goad/src/diagnostics.rs` — only if the version line is written there.
- `crates/goad/build.rs` — only if the sha arrives through it (OQ-4).
- `crates/goad-shell/src/error.rs` — only if OQ-3 puts the path in stratum 2.
- `crates/goad/tests/renderer/startup.rs` — the argument table and the display
  text are both already tested there.
- `docs/roadmap.md` — at close.

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
  }` — and is the prior art, not the work.
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
- [ ] AC-4 — `goad --version` prints a version and the git revision it was
      built from, on stdout, exit 0 — and is not read as a configuration path.
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
  run. Whether it joins is OQ-2, and the answer decides the tier.
- **ADR-001** — one-way strata. The startup-surface half is stratum 3 work;
  OQ-3 asks whether the path belongs in stratum 2's error type instead, which
  is a question about direction and must be answered against this ADR rather
  than around it.
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

- OQ-1 — **Where the home-manager module lives.** `~/flakes` holds two house
  patterns: `modules/home/linux/satan-attrd.nix` imports a module the project
  exports; `modules/home/linux/behaviour.nix` writes panopticon's unit inline.
  The unit carries facts only goad knows — which exit code is a refusal, what
  it is `PartOf` — and the question is whether that is enough to put a
  `nix/module.nix` in this repository.
- OQ-2 — **Does `nix build` join the phase gate?** If it does, POL-001 is
  amended and this slice is tier 2. If it does not, a `flake.nix` that stops
  building is green here and broken in `~/flakes`, which is where it is noticed.
- OQ-3 — **Which stratum names the configuration path.** Both shapes are
  already in this tree: `IngressError { path, fault }` carries it in stratum 2;
  `goad-emit`'s `StartupFault::Unparseable { path, fault }` wraps it at
  stratum 3. `goad` has the value at the seam and drops it —
  `main::start(path)` calls `Config::load(path).map_err(StartupError::Config)`.
  Whichever is taken, the design says why the other was not, because the
  divergence is what a reader will ask about.
- OQ-4 — **Where the revision comes from on the `cargo install` path**, which
  has no flake and may have no git. `self.rev` / `self.dirtyRev` answers the
  nix side; the other side is a decision about what an unknown revision prints.
  AC-5 is a constraint on this answer, not a consequence of it.
- OQ-5 — **Does the package build run the tests?** crane's `doCheck` is on by
  default, and the renderer tests need a fontconfig configuration to construct
  a component at all — `flake.nix`'s jail comment records 58 of 156 cases
  panicking in fontique with `NoMatch` without one, measured when it was
  written. So the choice is *pass `fontsConf` into the check phase* or
  *`doCheck = false`, leaving verification to the gate*.
- OQ-6 — **What happens to `~/.config/goad/env`.** It exists for the
  `cargo install` path alone once the nix binary is wrapped. Deleting it breaks
  that path; keeping it leaves a file whose two values can still go stale, with
  one fewer consumer to notice.

## What would raise the tier

Tier 1 as opened: nothing here writes or amends canon, and nothing changes the
wire contract. Two things would change that, and both are open questions above
rather than surprises:

- **OQ-2 answered yes** — a command added to the gate is an amendment to
  POL-001, and the slice becomes tier 2.
- **OQ-3 answered in stratum 2** — moving a path into `ConfigError` changes a
  type `goad-emit` also consumes, and the design surface grows the seam between
  two binaries. That is a size judgement, not a canon one, but it is the shape
  that would push past the 300-line cap.

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

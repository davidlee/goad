# Plan — Slice 006: packaging and the startup surface

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

<!-- Phase ids (PHASE-NN) and criterion ids (EN-/EX-/VT-/VA-/VH-N) are
     immutable: edits append, never renumber, so the sequence goes
     non-monotonic after a split and that is expected. Criterion ids are local
     to their phase — cite another phase's phase-qualified (PHASE-03/EX-2).
     Verification modes — VT: automated test. VA: agent check. VH: human
     acceptance.
     Progress is NOT recorded here. Status lives in `notes.md`. -->

## Overview

Five phases: two packaging, two startup surface, one evidence.

`design.md` §5.2 states eight contracts, (a) to (h). PHASE-01 lands (a), (b),
(c) and (h); PHASE-02 lands (d); PHASE-03 lands (e) and (g); PHASE-04 lands (f).
PHASE-05 lands nothing — it is where the acceptance criteria no phase can
discharge alone are discharged, by running the software.

**The two halves are independent.** Nothing in the packaging work calls anything
in the startup work. The single coupling is the *name* `GOAD_REVISION`: PHASE-01
sets it on two derivations, PHASE-03 reads it with `option_env!`. Neither phase
imports, links or tests the other's surface, so the ordering below is a choice
and not a dependency — §Sequencing says why this choice.

**AC-5 cannot be discharged until both halves have landed**, and AC-1's second
half, AC-2's second half and AC-7's unit are not discharged by any green gate.
All four are PHASE-05's, deliberately.

**`just check` exits 0 at every phase exit**, the joined `tokio` entry included
(PHASE-01/EX-1). No phase ends on a gate it has not run.

**Two decisions this plan takes** that `design.md` leaves below its altitude,
recorded here rather than improvised in a phase:

- **P-1 — the derivation's `version` is read from the manifest**, not written as
  a literal: `(builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version`,
  which is `~/dev/doctrine/flake.nix`'s idiom. The spike used the literal
  `"0.1.0"` only because S-1 blocked the read, and the spike's README lists it
  as wrong for the deliverable. It is a single-source-of-truth choice and
  **nothing more**: the S-1 precondition is enforced by crane, unconditionally
  and whether or not the flake reads a manifest itself — `cleanCargoToml` parses
  every manifest to build the dummy source, and supplying `pname`/`version`
  explicitly is not a workaround (S-1). Dropping the read would loosen nothing
  and keeping it tightens nothing (F-6). What the constraint does lack is a
  record at the site it binds, which PHASE-01/EX-1 now carries.
- **P-2 — `just install`'s comment is corrected in PHASE-01.** Its recipe is
  unchanged (OQ-6, D7) but its comment predicts its own retirement — *"Slice 006
  … wraps the environment into the binary, at which point there is no pair to
  keep in sync and no env file to go stale"* — which D7 made false: the pair
  survives on the cargo path, and the file's reader becomes a person (§5.3).
  Text only, no behaviour: leaving it is shipping a false claim about the slice
  that just ran.

## Sequencing & rationale

Packaging (PHASE-01, PHASE-02), then startup (PHASE-03, PHASE-04), then evidence
(PHASE-05).

**Why packaging first, given the halves are independent.** With PHASE-01
landed, PHASE-03 can *observe* the stamped branch of `--version` —
`nix build .#goad && ./result/bin/goad --version` printing `0.1.0 (<rev>)` —
rather than reaching it only through `version_line`'s unit tests. The reverse
order leaves the stamped branch unobserved until PHASE-05. It is also the
riskiest work (the only phase that iterates against a build system rather than a
compiler) and goes first while the session budget is whole.

**What cannot be reordered.** PHASE-05 is last: it is the only phase whose entry
criterion is every other phase. Nothing else is fixed — PHASE-03 and PHASE-04
may swap, and either may precede PHASE-01/02 if a later reading of the tree
makes that better.

**What cannot run in parallel.** PHASE-01 and PHASE-02 both touch `flake.nix`;
PHASE-03 and PHASE-04 both touch `crates/goad/src/startup.rs`,
`crates/goad/src/main.rs` and `crates/goad/tests/renderer/startup.rs`. One writer
per worktree holds regardless (`docs/memory/one-writer-per-worktree.md`).

**Sizing.** PHASE-01 is the largest and is the one to watch: it carries the
precondition, the flake, two negative controls, the `justfile`, and — after
round 1's F-1 — a wrapper reading and a headless photograph. If it is going to
overrun a session, the cut is after EX-5 (the flake evaluates and both packages
build); VA-3, VA-6 and VA-7 are all observations of a build that already
exists, and a second agent can take them with the phase sheet open. PHASE-02 and
PHASE-05 are small. PHASE-03 and PHASE-04 are ordinary red/green/refactor work
across three files each.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-01/EX-2 (builds) · PHASE-05/VH-1 (a window with text in it) |
| AC-2 | PHASE-01/EX-3 (builds) · PHASE-05/VH-3 (an envelope into a running host) |
| AC-3 | PHASE-01/VA-6 (the wrapper carries both) · PHASE-01/VA-7 (a window with text, headless) · PHASE-01/VA-4 (the absence claim only) |
| AC-4 | PHASE-03/VT-2 (binary tier) · PHASE-03/VT-3 (not read as a path) · PHASE-03/VT-1 (the stamped branch, the only place `cargo test` reaches it) · PHASE-05/VA-1 (the stamped branch on a real binary) |
| AC-5 | PHASE-05/VA-1 |
| AC-6 | PHASE-04/VT-1, PHASE-04/VT-2 |
| AC-7 | PHASE-02/VA-1 (the module generates it) · PHASE-05/VH-2 (the unit systemd actually loaded) |
| AC-8 | every phase's gate — PHASE-01/EX-8, PHASE-02/EX-5, PHASE-03/EX-7, PHASE-04/EX-5 · PHASE-05/VH-1 |
| AC-9 | PHASE-05/VA-2 |

`design.md` §9's **thirteen** verification rows — twelve as accepted, plus the
one F-1 added — in its order: the manifest joins (PHASE-01/VA-1) · the source
filter (PHASE-01/VA-3) · both packages build (PHASE-01/EX-2, EX-3) · an empty
environment (PHASE-01/VA-4) · the wrapper carries both variables
(PHASE-01/VA-6, VA-7) · `--version` at
the binary tier (PHASE-03/VT-2) · `version_line` both branches (PHASE-03/VT-1) ·
the argument table (PHASE-03/VT-3) · every path-holding error names its path
(PHASE-04/VT-1, VT-2) · the unit's restart semantics (PHASE-02/VA-1,
PHASE-05/VH-2) · a window with text in it (PHASE-05/VH-1) · `just install` still
works (PHASE-05/VA-2) · the gate (every phase).

---

## PHASE-01 — the crane packages

**Objective:** `nix build .#goad` and `nix build .#goad-emit` produce wrapped
binaries that run from an empty environment, from a tree whose workspace
manifest crane can parse.

**Surfaces:** `Cargo.toml` (the `tokio` entry only) · `flake.nix` ·
`flake.lock` · `justfile` (a new `package` recipe; the `install` recipe's
comment).

**Entry**
- EN-1 — tree clean at `a0ffdbd` or later, design accepted, one writer.

**Exit**
- EX-1 — **the precondition, first:** `Cargo.toml`'s `[workspace.dependencies]`
  `tokio` entry is one line, and `just check` exits 0 with it joined. Committed
  before the flake is touched, so a later bisect separates a manifest change
  from a packaging one. **The entry carries a comment saying why it must stay on
  one line** — crane parses every manifest with `builtins.fromTOML`, which is
  TOML 1.0, and a newline inside an inline table is TOML 1.1 (S-1). `just check`
  cannot see a re-split; the comment is the only thing at that site that can
  (F-6), and the entries around it are already argued in comments.
- EX-2 — `nix build .#goad` succeeds and `./result/bin/goad` is a wrapper over
  `.goad-wrapped` (AC-1, first half).
- EX-3 — `nix build .#goad-emit` succeeds; `goad-emit` has **no** wrapper and
  **no** `guiLibs` in `buildInputs` (AC-2, first half; §5.2(a), S-4).
- EX-4 — `packages.${system}.default` is `goad`; the three jail packages and the
  devShell are unchanged.
- EX-5 — both derivations carry `GOAD_REVISION`, from
  `self.shortRev or self.dirtyShortRev or ""` (§5.2(c)). Nothing reads it yet.
- EX-6 — `just package` exists, builds both, sits outside POL-001's `check`
  chain, and its comment carries the bare-git-form caveat (§5.2(h), OQ-2b).
- EX-7 — `just install`'s comment no longer predicts its own retirement (P-2).
- EX-8 — `just check` exits 0.

**Verification**
- VT-1 — none new. This phase writes no Rust.
- VA-1 — §9 row 1: evaluation succeeds where S-1 failed. `nix eval
  .#packages.x86_64-linux.goad.drvPath` returns. What makes this an observation
  rather than a comment claiming one is **crane**, which parses every manifest
  unconditionally (S-1) — not P-1's `fromTOML` read, which adds no enforcement
  of its own (F-6).
- VA-2 — `nix derivation show .#goad` and `.#goad-emit` both name
  `GOAD_REVISION` with a plausible short revision (EX-5).
- VA-3 — §9 row 2: **S-2's two negative controls, rebuilt and observed to
  fail.** Add them temporarily to `packages`, build each, record the failure
  text in `notes.md`, then remove them. An uncompiled control greps the same as
  a passing one (`docs/memory/negative-control-must-compile.md`):
  - stock `craneLib.cleanCargoSource` → `build.rs` fails, `ui/app.slint` missing;
  - stock **+ `.slint`**, `assets/` still stripped → the Slint compiler cannot
    resolve `Inter.ttf` / `Geist.ttf`.
- VA-4 — §9 row 4, AC-3's **absence** claim and nothing more: under `env -i`,
  with neither `LD_LIBRARY_PATH` nor `FONTCONFIG_FILE` set in the caller's
  environment, `env -i ./result/bin/goad --help` prints the usage block at exit
  0, and `env -i ./result-emit/bin/goad-emit --version` exits 0 printing a
  version line. `ldd ./result/bin/.goad-wrapped` reports zero `not found`.
  **Do not assert emit's exact string here**: PHASE-01/EX-5 stamps
  `GOAD_REVISION` on emit's derivation too, so from PHASE-03 a nix-built emit
  prints `0.1.0 (<rev>)` and a criterion pinned to `0.1.0` would be false at
  audit, which walks every criterion (F-2).
- VA-5 — `just -n check` still prints POL-001 §Compliance's six commands, in
  order, unchanged. The new recipe touches none of them.
- VA-6 — **AC-3's positive half, and the one check here that can fail for the
  defect this slice replaces** (F-1). Read the *generated wrapper script* at
  `./result/bin/goad` and confirm it sets **both** `--prefix LD_LIBRARY_PATH`
  and `--set-default FONTCONFIG_FILE`. Every check in VA-4 passes on a binary
  wrapped with only one of the two — `--help` precedes any Slint call, emit is
  unwrapped by construction, and `ldd` resolving clean on a `dlopen`ing binary
  is the very fact `slice-006.md` §Purpose cites as what makes the defect
  invisible.
- VA-7 — **a window with text in it, headless, in the phase that wrote the
  wrapper.** `flake.nix`'s `goadShot` is the instrument: cage on the wlroots
  headless backend, grim for the photograph, `SLINT_BACKEND=winit-software`.
  **The invocation is the criterion** (F-9) — `goad-shot` is reachable only from
  the dev shell, which exports both `LD_LIBRARY_PATH` and `FONTCONFIG_FILE`, and
  `--set-default` is by design a no-op against a caller's value while `--prefix`
  prepends to a list already naming all five `guiLibs`. Run it from the dev
  shell unchanged and a wrapper missing **both** flags photographs identically.
  So strip exactly those two, and absolutise every path, because cage's child
  starts wherever cage does — `goadShot`'s own comment says so, which is why it
  already absolutises `GOAD_SHOT_OUT`, and `examples/demo.toml`'s `command` is
  relative again inside it:

  ```
  env -u LD_LIBRARY_PATH -u FONTCONFIG_FILE \
    goad-shot -o "$PWD/shot.png" -s 5 -- \
    "$PWD/result/bin/goad" "$PWD/examples/demo.toml"
  ```

  `PATH` is kept: the demo backend is `["bash", "examples/shell/backend.sh"]`.
  Unsetting the two costs cage and grim nothing — they are store binaries with
  their own rpath. **Open the PNG and look at it**; a blank window, or one with
  boxes where glyphs should be, fails this criterion, and so does an empty
  compositor. That a file was produced is not the criterion.
  If the run collides with a `just demo` already holding `./goad-demo.sock`,
  that is the collision and not a packaging defect — stop the other host.
  This does **not** discharge AC-1's second half — that is PHASE-05/VH-1, a
  person, under systemd, on the real compositor (A3). It is here so that a
  fontless wrapper is caught four phases earlier than it otherwise would be.

**Notes for the implementer**

Start from `docs/slices/006/spike/flake.nix`. It is a **starting point, not a
template**: its own README lists five throwaway packages and five things wrong
with it. Carry across the toolchain binding, the `lib.cleanSourceWith` shape,
`buildDepsOnly` and `wrapProgram`; do not carry the literal `workspaceVersion`
(P-1), `guiLibs` on `goad-emit`, the dropped `--locked`, or the `.ttf` suffix
filter.

- **The filter is spelled by directory, not by extension** — cargo sources, plus
  `.slint`, plus everything under `assets/` (§5.2(b), D9, and `design-log.md`,
  *the source filter is spelled by directory*). The spike's `.ttf` suffix is the
  rejected spelling.
- **`packages.${system}` is currently `jailPkgs` bare.** It becomes a merge;
  losing a jail package is a silent regression the gate does not see (EX-4).
- **`crane.mkLib pkgs |> overrideToolchain rust`**, with `rust` the same
  `pkgs.rust-bin.beta.latest.default` binding the devshell already uses —
  crane's default nixpkgs-stable rustc is older than this workspace's
  `edition = "2024"`, quite apart from doctrine's lint-verdict note.
- **`pname` and `version` must be passed explicitly** to `buildDepsOnly` and
  both `buildPackage` calls: the workspace root is a virtual manifest with no
  `[package]`, so crane's `crateNameFromCargoToml` has nothing to read.
- **`doCheck = false` on all three, and do not reintroduce `doCheck = true`.**
  crane's `checkPhaseCargoCommand` inherits `cargoExtraArgs`, so the obvious
  spelling with a `--bin` selector **reports green having run nothing** (S-3).
  OQ-5 and D5 settle this; a phase does not reopen it.
- **`nix build .#goad` from inside the repository reads the git tree, so an
  untracked file is invisible to it.** Nothing new is untracked in this phase,
  but PHASE-02's `nix/module.nix` will be: `git add` before building. Never a
  `path:` reference — the repository root holds `goad-demo.sock` and
  `.claude/worktrees/` holds six gitignored worktrees
  (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).
- **Commit before any consumer sees it.** `~/flakes` is PHASE-05's, and a
  git-input consumer cannot see an uncommitted `flake.nix`.
- Cost, so an agent does not read a slow build as a hang: 181s cold store, 22s
  for `goad`'s final layer warm, 10s for `goad-emit`, 1s no-op, 6–7s for a
  negative control (S-5).
- **`goad-shot` is already in the devshell** (`flake.nix`, `goadShot`) and this
  slice is the first thing to use it. It starts the app, sleeps `-s` seconds,
  photographs and kills; a GUI launched any other way from an agent session
  needs a background launch, not an `&`
  (`docs/memory/gui-launch-needs-a-pipe.md`). Five seconds is a starting guess
  — if the photograph is of an empty compositor, raise it before concluding
  anything about fonts.
- **Do not delete the spike.** It is a record of a measurement; `audit.md` says
  so at close.

---

## PHASE-02 — the home-manager module

**Objective:** the systemd user unit is generated from a store path by a module
this repository owns, with no `EnvironmentFile`.

**Surfaces:** `nix/module.nix` (new) · `flake.nix` (the
`homeManagerModules.default` export only).

**Entry**
- EN-1 — PHASE-01 done: there is a package for the module to take.

**Exit**
- EX-1 — `nix/module.nix` exists with exactly the option surface §5.2(d) states:
  `enable` (`mkEnableOption`), `package` (required, **no default**),
  `extraConfig` (`{}`, merged over the generated `Service` block). No
  `configFile` option and no typed environment options — both deliberately
  excluded (OQ-1).
- EX-2 — `config = mkIf cfg.enable` gives `home.packages = [cfg.package]` and
  `systemd.user.services.goad` **in home-manager's three blocks** — a flat
  attrset satisfies a field list read literally, renders an attrset VA-1's
  permissive stub finds every field in, and is then rejected or dropped by
  home-manager at the PHASE-05 cutover, which is the one place this plan says a
  module defect must not be repaired (F-11):

  | block | |
  |---|---|
  | `Unit` | `After`, `PartOf` = `graphical-session.target` |
  | `Service` | `ExecStart = "${cfg.package}/bin/goad"`, `Restart = "on-failure"`, `RestartPreventExitStatus = 2`, `RestartSec = 2`, and **no `EnvironmentFile`** — `cfg.extraConfig` merges over this block and no other |
  | `Install` | `WantedBy = graphical-session.target` |

  (AC-7, AC-3.)
- EX-3 — exported as `homeManagerModules.default`; `nix flake show` lists it.
- EX-4 — the module's comment carries R1: a tarball consumer stamps no revision,
  so AC-5 stops holding silently — the consumer is a git input by construction.
- EX-5 — `just check` exits 0. (It does not read `.nix`; run it anyway — this
  phase must not be the one that discovers a stale tree.)

**Verification**
- VA-1 — §9 row 9, AC-7: the generated unit is read, not assumed. **This
  repository has no home-manager input and is not getting one** — a new flake
  input is a dependency addition and a STOP condition (`docs/AGENTS.md`
  §Execute). The route is a throwaway `lib.evalModules` harness, in the same
  discipline as VA-3's negative controls: a stub module declaring only the two
  options this module sets (`home.packages`, `systemd.user.services`), the real
  module, and a configuration fragment enabling it with a fake package. Print
  `config.systemd.user.services.goad`, check every field in EX-2 by name, and
  check `EnvironmentFile` is **absent** rather than empty. Record the rendered
  attrset in `notes.md`, then discard the harness.
  **This was built and run at plan time against a module of the same shape**, so
  the route is known to exist rather than assumed (F-3). Two things it cost:
  `lib` comes from the flake's own locked nixpkgs, and the reference must be
  `builtins.getFlake "git+file:///home/david/dev/goad"` — the bare path form
  dies with *`goad-demo.sock` has an unsupported type*, the same trap as
  everywhere else in this slice.
  **What it does not hold:** the stub's option types are permissive, so this
  checks the module's own output, not home-manager's acceptance of it. The unit
  systemd actually loads is PHASE-05/VH-2 — this is the generator, that is the
  loader, and neither substitutes for the other.
- VA-2 — **I4, the review obligation**: no domain vocabulary in `nix/module.nix`
  or the unit text it produces. Nothing checks this — neither the
  domain-vocabulary scan nor any of ADR-001's four instruments reads `.nix`
  (a review obligation, not an enforced rule; it does not join POL-001
  §Verification's count — Cross-thread 4; `design.md` §8 R3). Read it, and say in
  `notes.md` that it was read. The scan's own word list is
  `crates/goad-boundary/tests/checks/vocabulary.rs`, `DOMAIN`.
- VA-3 — `git add nix/module.nix` before evaluating: an untracked file is
  outside the flake's git-tree source and the export will evaluate to a
  file-not-found that reads like a typo.

**Notes for the implementer**

- Prior art, in this order: `~/dev/satan-attrd/nix/module.nix` for the option
  surface; `~/satan/goad/goad.service` for the unit's fields, which come across
  as **defaults** rather than as constants a consumer cannot override;
  `~/flakes/modules/home/linux/satan-attrd.nix` for what the consumer will look
  like (four lines, PHASE-05, out of repo).
- `RestartPreventExitStatus = 2` is **this repository's exit-code contract**,
  which is the whole argument for the module living here (OQ-1): `main` maps
  every `StartupError` to exit 2, and PHASE-04 adds a tenth variant that
  inherits it with no change to the unit.
- The module asserts nothing about how its `package` was built. It takes a
  derivation and names `bin/goad` in it.

---

## PHASE-03 — `--version`, on both binaries

**Objective:** both binaries answer `--version` on stdout at exit 0, printing
the revision when the build stamped one, and `goad --version` stops being read
as a configuration path.

**Surfaces:** `crates/goad/src/startup.rs` · `crates/goad/src/main.rs` ·
`crates/goad/src/diagnostics.rs` · `crates/goad/Cargo.toml` ·
`crates/goad/tests/binary/` (new) · `crates/goad/tests/renderer/startup.rs` ·
`crates/goad-emit/src/main.rs` · `crates/goad-emit/src/render.rs`.

**Entry**
- EN-1 — nothing. Independent of PHASE-01 and PHASE-02 (§Overview); if they have
  landed, VA-1 is available and is worth having.

**Exit**
- EX-1 — `Launch` gains `Version`; `arguments`' guard sits **before** the
  catch-all `[only]` arm, and its doc table gains the row §5.2(e) states.
  `goad x --version` is still two arguments and still `StartupError::Usage` —
  the host does not guess which was meant.
- EX-2 — `diagnostics::version_line(revision: Option<&str>) -> String`:
  `Some("08528b5")` → `0.1.0 (08528b5)`, `None` → `0.1.0`. `run`'s `Version` arm
  writes it to stdout and returns `Ok(())`, beside `Help`'s.
- EX-3 — the caller passes
  `option_env!("GOAD_REVISION").filter(|revision| !revision.is_empty())` —
  **set-but-empty is unset**, the rule `crates/goad/build.rs` already states for
  `SLINT_STYLE` (§5.2(c)). No `build.rs` change; `option_env!` is a macro and is
  not on `clippy.toml`'s disallowed list.
- EX-4 — `diagnostics::USAGE` gains the third form, as `goad-emit`'s
  `render::USAGE` already lists three.
- EX-5 — `goad-emit` takes the same shape: `render::version_line`, same two
  branches, same `option_env!` filter, replacing the bare
  `env!("CARGO_PKG_VERSION")` in `main`'s `Invocation::Version` arm (OQ-7, D4).
  Nothing else about emit changes.
- EX-6 — `crates/goad/Cargo.toml` declares `[[test]] name = "binary"`, path
  `tests/binary/main.rs`. The manifest has `autotests = false`, so an undeclared
  target is silently not built.
- EX-7 — `just check` exits 0.

**Verification**
- VT-1 — §9 row 6, AC-5's rendering half: `version_line`'s **both branches**, as
  unit tests beside the other pure rendering functions — `mod tests` in
  `crates/goad/src/diagnostics.rs`, and in `crates/goad-emit/src/render.rs`.
  This is the **only** place the stamped branch is reachable under `cargo test`,
  which never sets `GOAD_REVISION` (`design-log.md`, OQ-7; and
  `docs/memory/tests-asserting-proxies.md` — the unit test is the real
  assertion, not a proxy for one).
- VT-2 — §9 row 5, AC-4, D10: a binary-tier case in
  `crates/goad/tests/binary/`, spawning `env!("CARGO_BIN_EXE_goad")` with
  `--version` and asserting stdout trims to the package version, exit 0, stderr
  empty. Transcribe `crates/goad-emit/tests/binary/exchange.rs`'s
  `version_prints_the_package_version_on_stdout_and_exits_0` and its helpers; do
  not invent a second convention.
- VT-3 — §9 row 7, AC-4's second half: the argument table's cases in
  `crates/goad/tests/renderer/startup.rs`, `mod arguments_table` — `--version`
  is `Launch::Version` and **not** `Launch::Config(PathBuf::from("--version"))`;
  `goad x --version` is still `Usage`. The usage block's own case in
  `mod usage_block` covers the third form.
- VA-1 — available only if PHASE-01 has landed, and worth taking: `nix build
  .#goad && ./result/bin/goad --version` prints `0.1.0 (<short rev>)`, and
  `~/.cargo/bin/goad --version` (or a `cargo run`) prints bare `0.1.0`. This is
  a sighting of AC-5, not its discharge — PHASE-05/VA-1 owns that.

**Notes for the implementer**

- **What each build prints** (`design-log.md`, OQ-7): nix clean tree
  `0.1.0 (08528b5)`; nix dirty tree `0.1.0 (08528b5-dirty)`; `cargo install`
  `0.1.0`. **No `(revision unknown)`** — the user dropped it explicitly. Say
  only what is known (§4, principle 2).
- **No program-name prefix.** `"goad: "` exists on `report_startup_line` because
  stderr must say who spoke; a direct answer on stdout need not, and emit's
  existing `--version` is the precedent.
- **Emit's existing binary test passes unchanged**, because `cargo test` never
  sets `GOAD_REVISION`, so emit's stdout still equals `CARGO_PKG_VERSION`. Do
  not weaken that assertion to a `starts_with` to make room for a revision it
  will never see under the gate.
- `render::startup_error_line` is `pub(crate)`; match the module's own
  visibility rather than `design.md` §5.2(g)'s illustrative `pub fn`.
- `tests/binary/main.rs` follows emit's: a doc comment saying what this tier
  holds and what it deliberately does not, and `#[cfg(test)] mod …` on the
  declaration — a `tests/` target is always built with `--test`, so the
  attribute is never off, and without it `clippy::tests_outside_test_module`
  fires (see emit's own comment).
- The binary tier is feasible without a display only because both zero-exits
  precede any Slint call (§5.4). Nothing in this target may construct a window.
- `crates/goad/src/diagnostics.rs`'s `mod tests` cut is what `goad-boundary`'s
  AC-6 (a) instrument relies on to stay clear of this file's fixtures — add
  tests inside the existing `mod tests`, do not restructure it.

---

## PHASE-04 — the configuration path, named

**Objective:** every `StartupError` that holds a path names it. `goad
/nonexistent/wat.toml` says which file it tried.

**Surfaces:** `crates/goad/src/startup.rs` · `crates/goad/src/main.rs` ·
`crates/goad/tests/renderer/startup.rs`.

**Entry**
- EN-1 — nothing. Independent of PHASE-01 and PHASE-02; orderable against
  PHASE-03, with which it shares three files and therefore cannot run
  concurrently.

**Exit**
- EX-1 — `StartupError::Config(ConfigError)` is replaced by the two arms
  §5.2(f) states: `ConfigUnreadable { path: PathBuf, fault: std::io::Error }`
  and `ConfigUnparseable { path: PathBuf, fault: ConfigError }`, rendered
  `"{path} could not be read: {fault}"` and `"{path}: {fault}"` — the two
  spellings already in this tree, not a third (§4, principle 3).
- EX-2 — `main::start` splits them at the seam, where the path is in hand, with
  the same two-arm match `goad-emit`'s `socket_path` uses.
- EX-3 — `StartupError`'s doc comment says **ten** variants. It says eight today
  and there are nine; `Ingress` went unrecorded in slice 004. Fix the class:
  check the count after the edit rather than incrementing the stale number.
- EX-4 — `goad_shell::error::ConfigError` is **untouched** (OQ-3, D1). A path
  inside it would print twice in `goad-emit`, and repairing that is a non-goal.
- EX-5 — `just check` exits 0.

**Verification**
- VT-1 — §9 row 8, AC-6, I1: in `crates/goad/tests/renderer/startup.rs`,
  `mod display_text`, replace `config_is_unwrapped_and_unprefixed` with one case
  per new arm, each asserting the rendered line **contains the path** and reads
  as §5.2(f) states.
- VT-2 — the invariant, not just the two instances: `Ingress`'s existing case,
  `ingress_is_unwrapped_and_unprefixed_and_names_the_path`, still passes
  unchanged. SPEC-003 R-3 and R-4 bind it and AC-6 must not weaken it.
- VA-1 — I1 by enumeration: after the edit, `StartupError` has ten variants;
  exactly three hold a path (`ConfigUnreadable`, `ConfigUnparseable`,
  `Ingress`), all three name it, and the other seven hold none. Walk the enum —
  the claim is about the type, not about the three cases
  (`docs/memory/verify-the-enumeration-not-the-conclusion.md`).
- VA-2 — the defect as `research.md` S-4 stated it is gone:
  `goad /nonexistent/wat.toml` and `goad --version` no longer print the same
  line. The second half is PHASE-03's; this phase owns the first.

**Notes for the implementer**

- `main.rs` carries `#![deny(clippy::wildcard_enum_match_arm)]`. Emit's
  `socket_path` shape — `Err(ConfigError::Read(fault))`, `Err(fault)`, `Ok(..)`
  as arms of a match on the `Result` — is the one to transcribe; a match on the
  `ConfigError` itself with a binding catch-all sits directly under that deny.
  `just lint` is the arbiter, not this note.
- `Config::load` returns `ConfigError::Read(std::io::Error)` for the unreadable
  case and five other variants for everything else, so the split is exactly
  `Read` against the rest — the same cut emit already makes.
- `StartupError` has no `PartialEq` (a `slint::PlatformError` inside it has
  none), so cases assert on `Display`, as every existing case in
  `mod display_text` does.
- The two new arms carry a `PathBuf`, so `start` clones the path it was handed.
  That is the cost of naming it, and it is paid once per failed startup.

---

## PHASE-05 — the cutover, and the evidence

**Objective:** the software has been run: a nix-built binary under systemd, a
window with text in it, and a `--version` that tells the two install paths
apart.

**Surfaces:** none in the repository but `notes.md`. The `~/flakes` consumer and
the retirement of `~/satan/goad/goad.service` are **out-of-repo evidence, not
deliverables** (§5.4, `slice-006.md` §Scope).

**Entry**
- EN-1 — PHASE-01 through PHASE-04 all `done` in `notes.md`, and committed:
  `~/flakes` is a git-input consumer and cannot see an uncommitted `flake.nix`.

**Exit**
- EX-1 — the `~/flakes` consumer exists: `imports =
  [inputs.goad.homeManagerModules.default]` plus `services.goad = { enable =
  true; package = …; }`, on the `satan-attrd.nix` pattern, with the goad flake
  added as an input in the **bare git form**.
- EX-2 — the user service runs from the module's unit.
- EX-3 — `~/satan/goad/goad.service` and its symlink in
  `~/.config/systemd/user/` are removed, **after** EX-2 is observed and not
  before (§5.4's sequence).
- EX-4 — `~/.config/goad/env` still exists and `just install` still writes it
  (OQ-6). It is not removed by this cutover.
- EX-5 — every observation below is written into `notes.md` naming what was
  seen, so `audit.md` can cite it rather than re-run it.

**Verification**
- VH-1 — **AC-8 and AC-1's second half, the person's, and no green gate is this
  evidence** (`docs/AGENTS.md` §Tiers): the nix-built `goad`, running under
  systemd, opens a window **with text in it**. Both halves are named because
  capturing one and not the other is the defect this slice replaces — a window
  that draws no text is `FONTCONFIG_FILE` missing, and A3 (DejaVu as fallback
  for the two compiled-in faces) is testable only here.
- VH-2 — §9 row 9's second half, AC-7: read the unit systemd actually loaded —
  `systemctl --user cat goad` — and confirm `Restart=on-failure`,
  `RestartPreventExitStatus=2`, `RestartSec=2`, an `ExecStart` that is a
  **store path**, and **no `EnvironmentFile=`**.
- VH-3 — AC-2's second half: the nix-built `goad-emit` puts an envelope into the
  running host's socket and the host reacts.
- VA-1 — **AC-5, which no single phase can discharge**: `./result/bin/goad
  --version` and `~/.cargo/bin/goad --version` are run one after the other and
  differ — `0.1.0 (<rev>)` against `0.1.0`. Record both strings verbatim in
  `notes.md`.
  **VA-2 runs first, and that ordering is the criterion's** (F-8): the
  `~/.cargo/bin/goad` on this machine today predates the slice and has no
  `--version` at all — it reads the flag as a configuration path and exits 2
  (measured, `research.md` S-4). That "differs" from a stamped version line
  while demonstrating nothing AC-5 asks for. So: **both invocations must exit 0
  and both must print a version line**, agreeing on the package version and
  differing only in the parenthetical. An exit 2 on either side fails this
  criterion rather than passing it.
- VA-2 — §9 row 11, AC-9: `just install` runs green from the dev shell and
  installs a working pair — both binaries answer `--version`, and the env file
  it wrote names two non-empty store paths.
- VA-3 — `just package` runs green from a clean checkout of the committed tree
  (AC-1 and AC-2's build halves, re-observed after every phase has landed).

**Notes for the implementer**

- **Sequence matters and is §5.4's**: stand up the consumer, move the service
  onto the module's unit, *then* remove the hand-written unit. Removing first
  leaves nothing running if the module is wrong.
- A restart loop here is the thing `RestartPreventExitStatus=2` exists to
  prevent: if the service exits 2 it has a `StartupError`, and the diagnostic is
  on its stderr — `journalctl --user -u goad`. After PHASE-04 that line names
  the configuration file it tried.
- Nothing in this phase is a deliverable of the repository. If the cutover
  surfaces a defect in the module or the package, it is a finding against
  PHASE-01 or PHASE-02 and is repaired there — not patched in `~/flakes`.
- `docs/roadmap.md` is updated **at close**, not here (`slice-006.md` §Scope).

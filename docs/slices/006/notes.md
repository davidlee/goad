# Notes — Slice 006: packaging and the startup surface

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the crane packages | done | 2026-09-21 |
| PHASE-02 — the home-manager module | done | 2026-09-21 |
| PHASE-03 — `--version`, on both binaries | done | 2026-09-21 |
| PHASE-04 — the configuration path, named | done | 2026-09-21 |
| PHASE-05 — the cutover, and the evidence | pending | |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — the crane packages

**Objective:** `nix build .#goad` and `nix build .#goad-emit` produce wrapped
binaries that run from an empty environment, from a tree whose workspace
manifest crane can parse.

**Reading list**

Cited by symbol, not line (CLAUDE.md §Working here); `plan.md` §PHASE-01 is the
contract and everything else is support.

- `docs/slices/006/plan.md` §PHASE-01 — entry, EX-1..EX-8, VA-1..VA-7, and its
  *Notes for the implementer*. Read it whole before touching anything; the
  eleven-item note list is where the spike's five known-wrong choices are named.
- `docs/slices/006/design.md` §5.2 (a) flake outputs and the selector table,
  (b) the source filter, (c) `GOAD_REVISION`, (h) `just package`; §5.3 for why
  `~/.config/goad/env` survives with a person as its reader (P-2's premise);
  §5.5; §9 rows 1–5.
- `docs/slices/006/spike/` — `flake.nix` and `README.md`. **Starting point, not
  a template.** The README lists what is wrong with it.
- `docs/slices/006/research.md` Thread 3 — S-1 (the TOML 1.0 parse, EX-1's
  reason), S-2 (the two negative controls, VA-3), S-3 (`doCheck` reports green
  having run nothing), S-4 (emit has no renderer), S-5 (build costs).
- `docs/policy/001-the-phase-gate.md` §Compliance — the six commands VA-5
  re-reads out of `just -n check`.
- `flake.nix` — `guiLibs`, `fontsConf`, `goadShot`, `jailPkgs`,
  `packages.${system}` (currently `jailPkgs` bare — EX-4), and the devShell's
  `LD_LIBRARY_PATH` / `FONTCONFIG_FILE` bindings, which are the pair the
  wrapper must reproduce and the reason VA-7 strips them.
- `Cargo.toml` `[workspace.dependencies]` — the `tokio` entry, and the
  `toml` / `slint` entries above and below it as the house style for an
  argued comment (EX-1).
- `justfile` — the `install` recipe's comment block (EX-7) and `check` (EX-8,
  VA-5).
- `crates/goad/build.rs` — the `SLINT_STYLE` set-but-empty rule, the prior art
  §5.2(c) points at. Not edited in this phase.
- `docs/memory/negative-control-must-compile.md` (VA-3),
  `path-flake-ref-breaks-on-demo-socket.md` (never a `path:` ref),
  `gui-launch-needs-a-pipe.md` (VA-7), `one-writer-per-worktree.md`.
- `docs/AGENTS.md` §Execute — red/green/**refactor**, stay inside the declared
  surfaces, STOP rather than improvise.

**Assumptions & STOP conditions**

Taken on faith:

- The spike's measurements still hold on today's tree — crane parses, the
  filter shape works, costs are S-5's. The spike ran at `b523a14`; nothing
  since has touched `Cargo.toml` or `flake.nix`.
- `goad-shot` works from this dev shell. It has never been used by a slice
  before; VA-7 is its first consumer.
- Five seconds is enough for `-s`. A photograph of an empty compositor means
  raise it, not that fonts are broken.

STOP and consult rather than improvise:

- **`doCheck`.** If the build looks like it wants `doCheck = true`, that is S-3
  and OQ-5/D5 settled it. Do not reopen it in a phase.
- **The filter.** If admitting `.slint` and `assets/` by directory does not
  work, stop. The `.ttf` suffix spelling is the *rejected* one (D9) — reaching
  for it is a design change.
- **A dependency, a nixpkgs input, or a toolchain channel change.** `rust` is
  `pkgs.rust-bin.beta.latest.default`, the binding the devShell already uses.
- **Any surface outside** `Cargo.toml` (the `tokio` entry only), `flake.nix`,
  `flake.lock`, `justfile`. Touching a Rust source file in this phase is scope
  creep: this phase writes no Rust (VT-1).
- **VA-6 or VA-7 fails.** A wrapper missing a flag, or a photograph with boxes
  where glyphs should be, is the defect the slice exists to remove. Report it;
  do not relax the criterion.
- Session budget past ~200k tokens: PARTIAL checkpoint here and hand to a fresh
  agent (`docs/memory/subagent-session-budget.md`).

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->

- [x] T-1 — EX-1, first and alone: join the `tokio` entry onto one line, add
      the comment saying why it must stay there (S-1: `builtins.fromTOML` is
      TOML 1.0; a newline inside an inline table is TOML 1.1, and `just check`
      cannot see a re-split). `just check` exits 0. **Commit before the flake
      is touched.**
      *Done.* `just check` exits 0 in 21s with the entry joined, and
      `nix eval --impure --expr '(builtins.fromTOML (builtins.readFile
      /home/david/dev/goad/Cargo.toml)).workspace.package.version'` now returns
      `"0.1.0"` where S-1 recorded a parse error. Commit `43a7a10`.
- [x] T-2 — VA-1: `nix eval .#packages.x86_64-linux.goad.drvPath` returns,
      where S-1 failed. Record the output.
- [x] T-3 — EX-2..EX-5: the flake. crane via `crane.mkLib pkgs |>
      overrideToolchain rust`; one `buildDepsOnly` over `--workspace`; explicit
      `pname`/`version` on all three (virtual manifest); `version` read from the
      manifest (P-1); `doCheck = false` on all three; the filter by directory;
      `goad` wrapped with both flags, `goad-emit` unwrapped and without
      `guiLibs`; `GOAD_REVISION` on both; `packages.${system}` a **merge** with
      `jailPkgs`, `default` = `goad`.
- [x] T-4 — EX-2, EX-3: `nix build .#goad` and `nix build .#goad-emit` succeed.
      `./result/bin/goad` is a wrapper over `.goad-wrapped`.
- [x] T-5 — VA-2: `nix derivation show` names `GOAD_REVISION` on both, with a
      plausible short revision.
- [x] T-6 — VA-3: build S-2's two negative controls, **observe each fail**,
      record the failure text here, then remove them. An uncompiled control
      greps the same as a passing one.
- [x] T-7 — VA-4: `env -i ./result/bin/goad --help` exits 0 printing usage;
      `env -i ./result-emit/bin/goad-emit --version` exits 0 printing a version
      line (**do not pin its exact string** — F-2); `ldd
      ./result/bin/.goad-wrapped` reports zero `not found`.
- [x] T-8 — VA-6: read the generated wrapper script and confirm **both**
      `--prefix LD_LIBRARY_PATH` and `--set-default FONTCONFIG_FILE`. This is
      the one check here that fails for the defect the slice replaces.
- [x] T-9 — VA-7: the headless photograph, with the two variables stripped and
      every path absolute. **Open the PNG and look at it.** A blank window,
      boxes for glyphs, or an empty compositor all fail.
- [x] T-10 — EX-6: `just package`, outside POL-001's chain, comment carrying
      the bare-git-form caveat.
- [x] T-11 — EX-7: `just install`'s comment stops predicting its own
      retirement (P-2). Text only; the recipe is unchanged.
- [x] T-12 — VA-5: `just -n check` still prints POL-001 §Compliance's six
      commands, in order, unchanged.
- [x] T-13 — EX-8: `just check` exits 0. Refactor pass, phase sheet current,
      §Status set to `done`, §Harvest updated in place, commit.

**What was observed**
<!-- Verification criteria are observations, not claims. The strings are here
     because a criterion nobody can re-read is a claim. -->

- **VA-1** — `nix eval .#packages.x86_64-linux.goad.drvPath` →
  `"/nix/store/hlq2221wm89lkrm5vsh8zpv2kp08qf39-goad-0.1.0.drv"`. S-1's
  evaluation-time `toml::parse_inline_table` failure is gone.
- **VA-2** — `GOAD_REVISION` is `43a7a10-dirty` on **both** derivations, which
  is `git rev-parse --short HEAD` plus the dirty marker the working tree earns.
  `goad-emit`'s `buildInputs` is empty; `goad`'s is the five `guiLibs` (EX-3).
- **VA-3** — both controls added to `packages`, built, and **observed to
  fail**, then removed. `nix eval` of `.#goad` returns the same `drvPath`
  before and after their removal, so nothing of theirs leaked into the
  deliverable. The failure text, verbatim:
  - stock `craneLib.cleanCargoSource` — `build.rs` exits 1 with
    *`error: Could not load /build/source/crates/goad/ui/app.slint: No such
    file or directory (os error 2)`*, then
    *`Error: CompileError([".../ui/app.slint:0: Could not load ..."])`*.
  - stock **+ `.slint`**, `assets/` stripped — `build.rs` exits 1 with
    *`error: File "../../../assets/Inter.ttf" not found`* at
    `crates/goad/ui/app.slint:14:8`, and the same for `Geist.ttf` at `15:8`,
    then `Error: CompileError([...])` naming both.
- **VA-4** — under `env -i`: `goad --help` prints the three-line usage block
  and exits 0; `goad-emit --version` prints `0.1.0` and exits 0 (**not** pinned
  as a criterion — PHASE-03 makes it `0.1.0 (<rev>)`, F-2);
  `ldd ./result/bin/.goad-wrapped | grep -c 'not found'` → `0`.
- **VA-5** — `just -n check` prints POL-001 §Compliance's six commands, in
  order, unchanged. `just -n package` prints its one command and sits outside
  the chain.
- **VA-6** — the generated wrapper at `./result/bin/goad` carries **both**: five
  `--prefix`-shaped `LD_LIBRARY_PATH` prepend blocks (wayland, libxkbcommon,
  libglvnd, fontconfig, gcc-lib), and
  `export FONTCONFIG_FILE=${FONTCONFIG_FILE-'/nix/store/…-fonts.conf'}` — the
  `--set-default` spelling, so a caller's own value still wins. It `exec`s
  `.goad-wrapped`.
- **VA-7** — passes, and the photograph was opened and looked at. 1280x720,
  the window filling the output: the title *"Fill in your interstitial
  journal?"*, the subtitle *"The last entry was a while ago."*, group headings
  *"Where you were"* and *"How it went"*, five labelled checkboxes, an *Energy*
  slider, a *Pages written* field reading `0`, a *Mood* dropdown reading
  `Good`, a *When* button reading `not set`, an *"Anything notable?"* field and
  a *Yeah* submit button. Glyphs throughout, no tofu boxes anywhere, and not an
  empty compositor. Run with `LD_LIBRARY_PATH` and `FONTCONFIG_FILE` stripped
  from the caller's environment and every path absolute, so it is the wrapper
  under test and not the dev shell. The PNG is at `va7.png` in this session's
  scratchpad and does not survive it.
- **Cost, re-measured** — 182s for the cold crane layer (S-5 said 181s), 10s
  for `goad-emit`, ~1s for a no-op `just package`, 6–7s per negative control.
  S-5 holds.

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **`--locked` is restored on all three derivations, the dependency layer
  included.** crane's default `cargoExtraArgs` is `--locked`, and setting the
  attribute replaces it rather than adding to it — which is how the spike came
  to drop it, and why its README lists that as wrong for the deliverable. So
  the dependency layer is `--locked --workspace`, not `--workspace`.
- **`just package` is `nix build --no-link --print-out-paths`.** Out-links are
  the alternative, and they land in the checkout as `result` / `result-1`.
  Neither name is in this repository's `.gitignore`; `result` alone happens to
  be covered by the *user's* global ignore file on this machine, which a clean
  clone elsewhere would not have. `.gitignore` is not this phase's surface, and
  printing the two store paths needs no ignore rule at all.
- **VA-7 ran against a copy of `examples/demo.toml`** in the session
  scratchpad, identical but for `[ingress] path`, which is an absolute
  scratchpad socket. Two `goad` hosts were already running and one held
  `./goad-demo.sock`. VA-7 names that collision and says to stop the other
  host; stopping a host the user is running is not an executing agent's call,
  and nothing in the criterion turns on which path the socket takes. The rest
  of the invocation is VA-7's, verbatim — both variables stripped, every path
  absolute, `-s 5`.
- **`assetsDir` is scoped inside `src`'s own `let`**, not as a sibling binding:
  it exists for one expression and is meaningless outside it.

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

- **`result*` out-links are not ignored by this repository.** Only the exact
  name `result`, and only via `~/.gitignore_global` on this machine. Anyone who
  runs a bare `nix build` here — PHASE-02 and PHASE-05 both will — gets an
  untracked symlink in `git status`. One line in `.gitignore` fixes the class;
  it was outside PHASE-01's declared surfaces. Candidate follow-up.
- **`goad-shot` cannot distinguish a good photograph from no photograph.** Its
  inner script runs `grim … || echo "goad-shot: grim failed" >&2` and does not
  propagate the app's exit status, so a run in which the app died immediately
  and grim photographed a blank output still exits 0 with a PNG on disk. That
  is exactly why VA-7 says to open the file, and the criterion was met here —
  but this slice is the tool's first consumer and the property is worth
  knowing. Not a packaging defect and not in scope.
- **Two `goad` hosts were running throughout**: `/home/david/.cargo/bin/goad`
  (the `just install` binary, on the default configuration path) and
  `target/debug/goad examples/demo.toml` holding `./goad-demo.sock`. The first
  is the host PHASE-05's cutover replaces; worth knowing before that phase
  moves the user service.

### PHASE-02 — the home-manager module

**Objective:** the systemd user unit is generated from a store path by a module
this repository owns, with no `EnvironmentFile`.

**Reading list**

- `docs/slices/006/plan.md` §PHASE-02 — the contract: EX-1..EX-5, VA-1..VA-3,
  and its *Notes for the implementer*. EX-2's three-row table is the unit,
  field for field.
- `docs/slices/006/design.md` §5.2(d) — the option surface; §5.4 — where this
  module sits in the cutover PHASE-05 performs; §8 R1 (the tarball consumer
  stamps no revision — EX-4's comment) and R3 (nothing scans `.nix` — VA-2).
- **Prior art, in this order, all outside this repository and all read-only:**
  `~/dev/satan-attrd/nix/module.nix` (the option surface),
  `~/satan/goad/goad.service` (the unit's fields — they come across as
  *defaults* a consumer may override, not constants),
  `~/flakes/modules/home/linux/satan-attrd.nix` (what PHASE-05's consumer will
  look like — four lines).
- `flake.nix` as PHASE-01 left it — `goadPackages`, the `packages.${system}`
  merge, and the `outputs` attrset the export joins.
- `crates/goad-boundary/tests/checks/vocabulary.rs`, `DOMAIN` — VA-2's word
  list. Read-only; this phase writes no Rust.
- `crates/goad/src/main.rs` — the exit-2 mapping every `StartupError` takes.
  It is the contract `RestartPreventExitStatus = 2` encodes, and the argument
  for the module living in this repository (OQ-1). Read-only.
- `docs/memory/path-flake-ref-breaks-on-demo-socket.md` — the harness must use
  `builtins.getFlake "git+file:///home/david/dev/goad"`; the bare path form dies
  on the socket. `negative-control-must-compile.md` — the same discipline
  applies to VA-1's harness: it must actually evaluate and actually print.
- `docs/AGENTS.md` §Execute.

**Assumptions & STOP conditions**

Taken on faith:

- The `lib.evalModules` harness route works: it was built and run at plan time
  against a module of this shape (F-3). Its two known costs are recorded in
  VA-1 — `lib` from the flake's own locked nixpkgs, and the `git+file://`
  reference.
- PHASE-01's packages evaluate, so `cfg.package` has something real to be.

STOP and consult rather than improvise:

- **A home-manager flake input.** This repository has none and is not getting
  one; that is a dependency addition (`docs/AGENTS.md` §Execute) and VA-1's
  harness exists precisely to avoid it. If the harness will not evaluate,
  stop — do not reach for the input.
- **The option surface.** No `configFile` option, no typed environment
  options. OQ-1 excluded both deliberately; a phase does not reopen it.
- **A flat attrset instead of home-manager's three blocks.** It would satisfy
  a field list read literally and be rejected or dropped at the PHASE-05
  cutover — the one place this plan says a module defect must not be repaired
  (F-11).
- **Any surface outside** `nix/module.nix`, `flake.nix` (the export only), and
  this sheet. In particular: `~/flakes` is **PHASE-05's** and is not touched
  here, and the prior-art trees are read-only.
- Session budget past ~200k tokens: PARTIAL checkpoint and hand over.

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->

- [x] T-1 — read the three prior-art files before writing anything.
      *Done.* The option surface is `~/dev/satan-attrd/nix/module.nix`'s, less
      its three typed environment options (OQ-1 excluded those); the unit's
      fields and two of its comments are `~/satan/goad/goad.service`'s.
- [x] T-2 — EX-1: `nix/module.nix` with exactly §5.2(d)'s three options —
      `enable`, `package` (required, **no default**), `extraConfig` (`{}`).
- [x] T-3 — EX-2: `config = mkIf cfg.enable` giving `home.packages` and
      `systemd.user.services.goad` in `Unit` / `Service` / `Install`, every
      field as the plan's table states, **no `EnvironmentFile`**, and
      `extraConfig` merging over `Service` and no other block.
- [x] T-4 — EX-4: the module's comment carries R1 — a tarball consumer stamps
      no revision, so AC-5 stops holding silently; the consumer is a git input
      by construction.
- [x] T-5 — VA-3, before evaluating anything: `git add nix/module.nix`. An
      untracked file is outside the flake's git-tree source.
- [x] T-6 — EX-3: exported as `homeManagerModules.default`; `nix flake show`
      lists it.
- [x] T-7 — VA-1: the throwaway `lib.evalModules` harness. Stub module
      declaring only `home.packages` and `systemd.user.services`, the real
      module, a fragment enabling it with a fake package. Print
      `config.systemd.user.services.goad`; check **every** EX-2 field by name;
      check `EnvironmentFile` is **absent**, not empty. Record the rendered
      attrset here, then discard the harness. It checks the generator, not
      home-manager's acceptance — that is PHASE-05/VH-2.
      *Done, and discarded.* It evaluated and printed; the attrset is below.
- [x] T-8 — VA-2, the review obligation I4: read `nix/module.nix` and the unit
      text it produces for domain vocabulary against `DOMAIN`, and **say here
      that it was read**. Nothing enforces this — no ADR-001 instrument and not
      the vocabulary scan reads `.nix`.
- [x] T-9 — EX-5: `just check` exits 0 (it does not read `.nix`; run it anyway).
      Refactor pass, sheet current, §Status `done`, §Harvest updated in place,
      commit.

**What was observed**
<!-- Verification criteria are observations, not claims. -->

- **VA-3** — `git add nix/module.nix` ran before the first evaluation, and
  every evaluation below read the git tree (`warning: Git tree … is dirty`,
  which is the tracked-but-modified `flake.nix`).
- **EX-3** — `nix flake show --all-systems` lists `homeManagerModules` beside
  `devShells` and `packages`, and prints it as `homeManagerModules: unknown`:
  `nix flake show` has no type for a module output and does not descend into
  one, so "lists it" is the whole of what that command can say. That the export
  is a module and not a broken path was checked separately —
  `builtins.isFunction (getFlake …).homeManagerModules.default` → `true`.
- **VA-1** — the harness evaluated, printed, and was discarded. Rendered
  `config.systemd.user.services.goad`, verbatim (the store path is the fake
  package's; `lib` came from the flake's own locked nixpkgs, and the reference
  was `git+file:///home/david/dev/goad`):

  ```json
  {
    "Install": { "WantedBy": ["graphical-session.target"] },
    "Service": {
      "ExecStart": "/nix/store/qp72s4kmlsxc4sa7jw38fnvjbjb7qwwa-goad-fake/bin/goad",
      "Restart": "on-failure",
      "RestartPreventExitStatus": 2,
      "RestartSec": 2
    },
    "Unit": {
      "After": ["graphical-session.target"],
      "PartOf": ["graphical-session.target"]
    }
  }
  ```

  Field by field, and by name, against EX-2's table:

  | EX-2 | rendered at | value |
  |---|---|---|
  | `After` | `Unit.After` | `["graphical-session.target"]` |
  | `PartOf` | `Unit.PartOf` | `["graphical-session.target"]` |
  | `ExecStart` | `Service.ExecStart` | `"${cfg.package}/bin/goad"`, interpolated |
  | `Restart` | `Service.Restart` | `"on-failure"` |
  | `RestartPreventExitStatus` | `Service.RestartPreventExitStatus` | `2` |
  | `RestartSec` | `Service.RestartSec` | `2` |
  | `WantedBy` | `Install.WantedBy` | `["graphical-session.target"]` |

  The blocks are `["Install", "Service", "Unit"]` — three, not a flat attrset
  (F-11). `Service`'s own field list is exactly
  `["ExecStart", "Restart", "RestartPreventExitStatus", "RestartSec"]`.
- **VA-1, `EnvironmentFile` absent rather than empty** — `?` tested on all four
  places it could hide, all `false`: `Service`, `Unit`, `Install`, and the
  service attrset's top level. It does not appear in `Service`'s field list
  above either.
- **VA-1, `extraConfig` merges over `Service` and no other block** — rendered a
  second time with `extraConfig = { RestartSec = 10; Nice = 9; }`:
  `Service.RestartSec` → `10` (the module's own value is a default a consumer
  overrides), `Service.Nice` → `9` (a field the module never sets), `Unit` and
  `Install` compare equal to the first render, and the block list is unchanged.
- **VA-1, two properties beyond the field list.** Both are EX-1/EX-2 claims that
  a field-by-field check of the enabled case cannot see:
  - `config = mkIf cfg.enable` — with `enable = false`, `systemd.user.services`
    renders `{}` and `home.packages` renders `[]`. Nothing leaks past the
    guard.
  - `package` is required with no default — `enable = true` and no `package`
    fails evaluation with *`error: The option 'services.goad.package' was
    accessed but has no value defined. Try setting the option.`*, and the
    option carries no `default` attribute at all (`opts.package ? default` →
    `false`).
- **The option surface is exactly three** — `builtins.attrNames` of
  `options.services.goad` → `["enable", "extraConfig", "package"]`. No
  `configFile`, no typed environment options (OQ-1). `enable`'s rendered
  description is *"Whether to enable goad, the personal intervention shell."*;
  `extraConfig`'s default is `{}`.
- **VA-2 — the module and the unit text it produces were read for domain
  vocabulary, and are clean.** Read against `DOMAIN` in
  `crates/goad-boundary/tests/checks/vocabulary.rs` — `habit`, `streak`,
  `journal`, `site`, `goal`, `reminder`, `compliance` — by word and
  case-insensitively, which is how `goad_boundary::scan::mentions` matches. I
  read `nix/module.nix` whole (comments included, where the Rust scan's
  `code_of` would strip them, so this is the stricter reading), and the unit
  text is the rendered attrset above: three `graphical-session.target`
  strings, a store path, `on-failure`, and two integers. A word-boundary grep
  agrees — zero for all seven, against `systemd` at 4 as the positive control
  that the pattern matches anything at all. **Nothing enforces this**: no
  ADR-001 instrument and not the domain-vocabulary scan reads `.nix` (design
  §8 R3, I4).
- **EX-5** — `just check` exits 0. `alejandra --check nix/module.nix flake.nix`
  also passes, which is the style the rest of the repository's nix is written
  in; it is not on the gate and `just check` never opened either file.

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **`Description` and `Type = "simple"` are not carried across from
  `~/satan/goad/goad.service`.** EX-2's table and design §5.2(d) agree on
  `Unit` = `After` + `PartOf`, and the plan calls the table "the unit, field
  for field"; `Type = "simple"` is systemd's default for an `ExecStart`-only
  unit, so it says nothing. One consequence is worth naming rather than
  discovering at the cutover: `extraConfig` merges over `Service` **only**, so
  a consumer cannot add `Unit.Description` through it either, and
  `systemctl --user status goad` will show the unit name where the old unit
  showed *"goad — personal intervention shell"*. If that description is wanted
  it is an amendment to §5.2(d), not a phase choice.
  **Superseded 2026-09-21, after PHASE-03**: it was put to the user as an
  amendment and the description was restored — §5.2(d)'s `Unit` row now names
  `Description`, and `nix/module.nix` carries it as a constant (`design-log.md`,
  *the unit keeps its `Description`*). `Type = "simple"` stays out. The call to
  raise it rather than carry it silently is what made that possible.
- **`extraConfig` is `attrsOf anything`, merged with `//`.** Shallow override,
  which is what "merged over the generated `Service` block" means for a flat
  list of systemd directives; `recursiveUpdate` would be the same thing at this
  depth while implying the block nests. `anything` rather than `str` because
  systemd directives are strings, integers, booleans and lists — the unit's own
  `RestartPreventExitStatus = 2` is an integer, so a stricter type would forbid
  overriding a field the module itself sets.
- **The export is `import ./nix/module.nix`, not the bare path.** Both work in
  a consumer's `imports`; `import` makes the file evaluate at export time, so a
  syntax error is a failure here rather than out of repo. It is also the prior
  art's spelling.
- **A `session` binding inside the module's `let`** for
  `graphical-session.target`, named once and used three times. The three
  references are the same thing by design (AC-3), not a coincidence that could
  drift.
- **The harness checked two things VA-1 did not name** — the `mkIf` guard and
  the missing-`package` error, both recorded above. They cost one evaluation
  each and close the gap between "every field is right" and "the module only
  ever produces those fields".

**Findings**
<!-- Things noticed in passing that are not this phase's job. -->

- **`nix flake show` cannot see into a module output.** It prints
  `homeManagerModules: unknown` and stops, so a flake that exported a broken
  path or a non-module would still "list it". EX-3 as written is satisfied, but
  the check that carries weight is the `builtins.isFunction` eval recorded
  above, and VA-1 past it. Worth knowing wherever a nix output is verified by
  `flake show` alone.
- **The git-input hazard (R1) is documented where the importer reads, not
  where the mistake is made.** The module's header says to fetch goad as a git
  input and why; the line that could get it wrong is the `inputs.goad.url` in
  `~/flakes`, which PHASE-05 writes. PHASE-05 already has the bare-git-form
  requirement in its EX-1 — this is the same fact arriving from the other side,
  and the two should agree.
- **PHASE-01's `result` out-link finding did not bite here.** This phase built
  nothing: `nix flake show` and `nix eval` leave no out-link, so `git status`
  stayed clean apart from the two declared surfaces. The finding still stands
  for PHASE-05.

### PHASE-03 — `--version`, on both binaries

**Objective:** both binaries answer `--version` on stdout at exit 0, printing
the revision when the build stamped one, and `goad --version` stops being read
as a configuration path.

**Reading list**

- `docs/slices/006/plan.md` §PHASE-03 — the contract: EX-1..EX-7, VT-1..VT-3,
  VA-1, and its *Notes for the implementer*, which settles what each build
  prints and forbids `(revision unknown)`.
- `docs/slices/006/design.md` §5.2(c) the revision and the set-but-empty rule,
  §5.2(e) the argument surface and its table row, §5.2(g) `version_line`;
  §5.4 for why the binary tier is feasible without a display.
- `crates/goad/src/startup.rs` — `Launch`, `arguments`, and `arguments`' doc
  table. The new guard sits **before** the catch-all `[only]` arm.
- `crates/goad/src/diagnostics.rs` — `USAGE`, `print_usage`,
  `report_startup_line` (the `"goad: "` prefix, and why stdout does not take
  one), and the existing `mod tests`. **Add inside that `mod tests`; do not
  restructure it** — its cut is what `goad-boundary`'s AC-6 (a) instrument
  relies on.
- `crates/goad/src/main.rs` — `run`'s `Help` arm, which the `Version` arm sits
  beside.
- `crates/goad/build.rs` — the `SLINT_STYLE` set-but-empty rule EX-3 mirrors.
  Read-only; **no `build.rs` change in this phase**.
- `crates/goad-emit/tests/binary/exchange.rs` —
  `version_prints_the_package_version_on_stdout_and_exits_0` and its helpers
  (`emit`, `code_of`, `stdout_of`, `stderr_of`). VT-2 **transcribes** this;
  it does not invent a second convention. Its sibling `main.rs` is the model
  for the new `tests/binary/main.rs`, `#[cfg(test)] mod …` attribute included.
- `crates/goad-emit/src/render.rs` — `USAGE` (three forms already),
  `startup_error_line`'s `pub(crate)` visibility, and `mod tests`;
  `crates/goad-emit/src/main.rs` — `Invocation::Version`'s bare
  `env!("CARGO_PKG_VERSION")`, which EX-5 replaces.
- `crates/goad/tests/renderer/startup.rs` — `mod arguments_table` and
  `mod usage_block`, where VT-3's cases go.
- `crates/goad/Cargo.toml` — `autotests = false` and the existing `[[test]]`
  block. An undeclared target is silently not built (EX-6).
- `docs/memory/tests-asserting-proxies.md` — VT-1 is the real assertion for the
  stamped branch, not a proxy: `cargo test` never sets `GOAD_REVISION`.
- `docs/AGENTS.md` §Execute — red / green / **refactor**.

**Assumptions & STOP conditions**

Taken on faith:

- PHASE-01 landed, so VA-1 is available and worth taking. It is a *sighting* of
  AC-5, not its discharge — PHASE-05/VA-1 owns that.
- Emit's existing binary test passes unchanged, because `cargo test` never sets
  `GOAD_REVISION`.

STOP and consult rather than improvise:

- **`(revision unknown)` or any other placeholder.** The user dropped it
  explicitly (OQ-7). Say only what is known.
- **Weakening emit's existing assertion** to a `starts_with` to make room for a
  revision it will never see under the gate. If it seems to need that, stop.
- **A `build.rs` change, or `std::env::var`.** `option_env!` is a macro and is
  not on `clippy.toml`'s disallowed list; that is the whole reason for the
  shape. Reaching for either means the approach has gone wrong.
- **Restructuring `diagnostics.rs`'s `mod tests`** rather than adding inside it.
- **Constructing a window anywhere in `tests/binary/`.** The tier is feasible
  only because both zero-exits precede any Slint call.
- **Any surface outside** the eight the plan declares, plus this sheet.
- Session budget past ~200k tokens: PARTIAL checkpoint and hand over.

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->

- [x] T-1 — **red first.** VT-1's unit cases for `version_line`'s both
      branches, in `diagnostics.rs`'s existing `mod tests` and in
      `render.rs`'s. `Some("08528b5")` → `0.1.0 (08528b5)`; `None` → `0.1.0`.
      *Done.* Both red with *`error[E0432]: unresolved import
      `super::version_line`*, in `goad` (lib test) and in `goad-emit`
      (bin test), before either function existed.
- [x] T-2 — EX-2, EX-5: `version_line` in both crates, matching each module's
      own visibility (`render.rs` is `pub(crate)`, not §5.2(g)'s illustrative
      `pub fn`). Green.
- [x] T-3 — VT-3 red, then EX-1: `Launch::Version`, the guard **before** the
      catch-all `[only]` arm, `arguments`' doc table row. Cases in
      `mod arguments_table` — `--version` is `Launch::Version` and **not**
      `Launch::Config(PathBuf::from("--version"))`; `goad x --version` is still
      `Usage`.
      *Done.* Red first with *`error[E0599]: no variant … named `Version`
      found for enum `Launch``*.
- [x] T-4 — EX-4 and its case in `mod usage_block`: `USAGE` gains the third
      form, as emit's already lists.
- [x] T-5 — EX-2's caller: `run`'s `Version` arm writes `version_line` to
      stdout and returns `Ok(())`, beside `Help`'s. No `"goad: "` prefix —
      stderr says who spoke, a direct answer on stdout need not.
      *Done.* Adding the variant first red `main.rs` with *`error[E0004]:
      non-exhaustive patterns: `Launch::Version` not covered`* — the
      crate-root `#![deny(clippy::wildcard_enum_match_arm)]` is why that
      match has no catch-all to absorb it silently.
- [x] T-6 — EX-3: the caller passes
      `option_env!("GOAD_REVISION").filter(|revision| !revision.is_empty())`.
      Set-but-empty is unset. Same in emit (EX-5), replacing the bare
      `env!("CARGO_PKG_VERSION")`.
- [x] T-7 — EX-6 and VT-2: `[[test]] name = "binary"`, path
      `tests/binary/main.rs`, in `crates/goad/Cargo.toml`; the target
      transcribed from emit's, doc comment and `#[cfg(test)] mod …` included.
      Spawn `env!("CARGO_BIN_EXE_goad")` with `--version`; assert stdout trims
      to the package version, exit 0, stderr empty.
      *Done*, and EX-6's premise checked rather than trusted — see **What was
      observed**.
- [x] T-8 — VA-1, if PHASE-01's packages still build: `nix build .#goad &&
      ./result/bin/goad --version` prints `0.1.0 (<short rev>)`; a `cargo run`
      prints bare `0.1.0`. Record both strings verbatim. Use
      `--no-link --print-out-paths` or clean up the out-link; `result` is not
      ignored by this repository.
- [x] T-9 — EX-7: `just check` exits 0. **Refactor pass** — the step where the
      design survives contact. Sheet current, §Status `done`, §Harvest updated
      in place, commit.

**What was observed**
<!-- Verification criteria are observations, not claims. -->

- **VA-1, the two version lines, verbatim.** Built at `4484eaf` with the
  working tree dirty, so the stamp carries the dirty marker PHASE-01 recorded:

  | build | invocation | stdout | exit |
  |---|---|---|---|
  | nix | `env -i …-goad-0.1.0/bin/goad --version` | `0.1.0 (4484eaf-dirty)` | 0 |
  | nix | `env -i …-goad-emit-0.1.0/bin/goad-emit --version` | `0.1.0 (4484eaf-dirty)` | 0 |
  | cargo | `cargo run -q -p goad --bin goad -- --version` | `0.1.0` | 0 |
  | cargo | `cargo run -q -p goad-emit -- --version` | `0.1.0` | 0 |

  That is **I3 sighted on both binaries** — a nix-built and a `cargo`-built
  `goad` differ in what `--version` says — and the bare form carries no
  placeholder. AC-5's discharge is still PHASE-05/VA-1's; this is a sighting.
  Store paths: `/nix/store/ncqvifv9rygpk2yrg0s893p3iaspfj28-goad-0.1.0` and
  `/nix/store/j3n228yvbyfgidszg6kgss9qw0fg7i2c-goad-emit-0.1.0`, built with
  `nix build --no-link --print-out-paths`, so no out-link landed in the
  checkout. `git add -A` ran first: the bare git form cannot see an untracked
  file, and `tests/binary/` was new.
- **EX-4, the third usage form, from the built binary.** `env -i …/bin/goad
  --help` prints, in full:

  ```
  usage: goad [<config-path>]
         goad -h | --help
         goad --version

  With no argument the configuration is read from
  $XDG_CONFIG_HOME/goad/config.toml, and from $HOME/.config/goad/config.toml when
  XDG_CONFIG_HOME is unset, empty, or not absolute.
  ```
- **EX-6's premise, checked rather than trusted.** With the `[[test]] name =
  "binary"` block removed, `cargo test -p goad` prints **no** `Running
  tests/binary/…` line, emits no error, and exits 0 — the target is silently
  not built, exactly as `autotests = false` implies
  (`docs/memory/negative-control-must-compile.md`). The block was restored and
  `git diff crates/goad/Cargo.toml` is empty, so nothing of the control
  survived; the case runs again and passes.
- **VT-2 at its own tier** — `cargo test -p goad --test binary` runs
  `version::version_prints_the_package_version_on_stdout_and_exits_0`, 1
  passed. Under the gate `GOAD_REVISION` is unset, so what it sees is the
  bare `0.1.0`; the stamped branch is VT-1's alone.
- **Emit's existing binary assertion passes unchanged** — `cargo test -p
  goad-emit`: 36 unit cases and 9 binary cases, all passing, with
  `version_prints_the_package_version_on_stdout_and_exits_0` still asserting
  equality against `CARGO_PKG_VERSION` rather than a `starts_with`. The
  premise held: `cargo test` never sets `GOAD_REVISION`.
- **EX-7** — `just check` exits 0, after the refactor pass and with everything
  applied.

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **`diagnostics::print_version(revision)` exists, beside `print_usage`.** EX-2
  names `version_line` and says `run`'s arm *writes it to stdout*; it does not
  say through what. `diagnostics.rs`'s own shape answers that — every
  user-visible string on this surface has a pure `*_line` half and an outlet
  beside it (`print_usage`, `report_startup`, `report_platform`), and the
  outlet is the only thing that touches a stream. The alternative was `main.rs`
  calling `goad_shell::report::line_to` itself, which would put the second
  stdout outlet outside the module whose whole claim is that it holds them all.
  Emit needs no equivalent: `to_stdout` already exists in its `main`, which is
  that crate's own answer to the same question.
- **`option_env!` sits in each `main`, not inside `version_line`.** EX-3 says
  *the caller passes* it, and that is what keeps `version_line` pure and both
  of its branches a unit case rather than a build configuration. It is also
  what makes the two crates' copies genuinely independent: each reads the
  `GOAD_REVISION` of its own compilation.
- **`dash_dash_version` asserts equality to `Launch::Version` and nothing
  else.** VT-3 states the property as *`Launch::Version` and **not**
  `Launch::Config(PathBuf::from("--version"))`*; on an enum deriving
  `PartialEq` the equality is the stronger of the two and subsumes the
  inequality, which would also pass for `Launch::Help`. The regression the
  second half names is recorded in the case's doc comment instead, where it
  says what the behaviour used to be.
- **The `[[test]]` block goes after `renderer`, not at the end**, so the eleven
  `event_loop_*` targets stay contiguous.
- **`diagnostics.rs`'s module doc gains one clause.** It names each outlet on
  this surface and said "two"; there are now three. That is a count this
  phase's change falsifies, so it was repaired rather than left. Nothing else
  in the file moved — `mod tests` was added to, not restructured.

**Findings**
<!-- Things noticed in passing that are not this phase's job. -->

- **`Launch` has no `-V`.** `--version` is the only spelling, matching emit,
  which also takes only `--version`. Nothing in the design asked for the short
  form and nothing here adds one; worth knowing only because `-h` *does* have
  a short form beside `--help`, so the two flags are not symmetric.
- **`crates/goad`'s `tests/binary/` is a second consumer of a convention that
  lives in `goad-emit`.** `goad`/`code_of`/`stdout_of`/`stderr_of` are now
  written twice, once per crate, because nothing at stratum 3 is shared and
  neither crate may depend on the other. Four four-line helpers is a cheap
  duplication and transcribing was the instruction, but a third binary tier
  would be the point to stop copying. Not this slice's call.
- **`StartupError`'s doc still says "The eight variants"** and lists nine
  (`Ingress` was added without the count moving). PHASE-04 rewrites that
  surface into ten, so it is that phase's to fix; noted here so it is not
  mistaken for drift this phase introduced.

### PHASE-04 — the configuration path, named

**Objective:** every `StartupError` that holds a path names it. `goad
/nonexistent/wat.toml` says which file it tried.

**Reading list**

- `docs/slices/006/plan.md` §PHASE-04 — the contract: EX-1..EX-5, VT-1, VT-2,
  VA-1, VA-2, and its four *Notes for the implementer*, which settle the match
  shape, the `Read`-against-the-rest cut, the absence of `PartialEq`, and the
  clone.
- `docs/slices/006/design.md` §5.2(f) — the two arms and their exact rendered
  text; §4 principle 3 (the two spellings already in the tree, not a third).
- `crates/goad/src/startup.rs` — `StartupError`, its doc comment (**it says
  eight and there are nine**; `Ingress` went unrecorded in slice 004), the
  `Config` arm, and the `Display` impl.
- `crates/goad/src/main.rs` — `start`, the seam where the path is in hand, and
  the crate-root `#![deny(clippy::wildcard_enum_match_arm)]` that rules out a
  binding catch-all.
- `crates/goad-emit/src/main.rs` — `socket_path`. **The shape to transcribe**:
  `Err(ConfigError::Read(fault))`, `Err(fault)`, `Ok(..)` as arms of a match on
  the `Result`, not on the `ConfigError`.
- `crates/goad-shell/src/error.rs` — `ConfigError`, and `Config::load`'s
  `Read(std::io::Error)` against its five other variants. **`ConfigError` is
  untouched** (EX-4, OQ-3, D1).
- `crates/goad/tests/renderer/startup.rs` — `mod display_text`, its existing
  `config_is_unwrapped_and_unprefixed` (which VT-1 replaces) and
  `ingress_is_unwrapped_and_unprefixed_and_names_the_path` (which VT-2 requires
  to pass **unchanged**).
- `docs/specs/` SPEC-003 R-3 and R-4 — what binds the `Ingress` case, and why
  AC-6 must not weaken it.
- `docs/memory/verify-the-enumeration-not-the-conclusion.md` — VA-1 is a claim
  about the type, not about the three cases. Walk the enum.
- `docs/AGENTS.md` §Execute — red / green / **refactor**.

**Assumptions & STOP conditions**

Taken on faith:

- Nine variants today, eight claimed in the doc. **Verified by the orchestrator
  at `c67262e`**: `NoConfigPath Usage Config Clock Runtime Platform EventLoop
  Enqueue Ingress`. After the split, ten. Count after the edit; do not
  increment the stale number.
- PHASE-03 landed on the same three files and is done. Nothing of it is
  half-applied — the gate was re-run independently.

STOP and consult rather than improvise:

- **Touching `goad_shell::error::ConfigError`.** Putting a path inside it would
  print twice in `goad-emit`, and repairing that is a declared non-goal
  (EX-4, OQ-3, D1).
- **A third rendering spelling.** The two are already in this tree; §4
  principle 3 says use them.
- **A binding catch-all in the match.** That sits directly under `main.rs`'s
  `wildcard_enum_match_arm` deny. `just lint` is the arbiter — if the lint and
  this sheet disagree, the lint wins and the note is the thing that was wrong.
- **Weakening `ingress_is_unwrapped_and_unprefixed_and_names_the_path`** to
  make room for anything. SPEC-003 binds it.
- **Any surface outside** `crates/goad/src/startup.rs`,
  `crates/goad/src/main.rs`, `crates/goad/tests/renderer/startup.rs`, and this
  sheet.
- Session budget past ~200k tokens: PARTIAL checkpoint and hand over.

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->

- [x] T-1 — **red first.** VT-1: in `mod display_text`, replace
      `config_is_unwrapped_and_unprefixed` with one case per new arm, each
      asserting the rendered line **contains the path** and reads as §5.2(f)
      states. Assert on `Display` — `StartupError` has no `PartialEq`.
- [x] T-2 — EX-1: `Config(ConfigError)` becomes `ConfigUnreadable { path,
      fault: std::io::Error }` and `ConfigUnparseable { path, fault:
      ConfigError }`, rendered `"{path} could not be read: {fault}"` and
      `"{path}: {fault}"`. Green.
- [x] T-3 — EX-2: `main::start` splits them at the seam, transcribing emit's
      `socket_path` match — on the `Result`, not on the `ConfigError`. The two
      new arms carry a `PathBuf`, so `start` clones the path it was handed;
      that is the cost of naming it, paid once per failed startup.
- [x] T-4 — EX-3: `StartupError`'s doc comment says **ten**. Count the variants
      after the edit and write what you counted. Fix the class, not the
      instance: the stale eight is what incrementing produces.
- [x] T-5 — VT-2: `ingress_is_unwrapped_and_unprefixed_and_names_the_path`
      still passes, unchanged. Do not touch it.
- [x] T-6 — VA-1, by enumeration: walk `StartupError`. Ten variants; exactly
      three hold a path (`ConfigUnreadable`, `ConfigUnparseable`, `Ingress`);
      all three name it; the other seven hold none. Write the walk here — the
      claim is about the type.
- [x] T-7 — VA-2: run it. `goad /nonexistent/wat.toml` names the file it tried,
      and no longer prints the line S-4 recorded. Record the string verbatim.
- [x] T-8 — EX-5: `just check` exits 0. **Refactor pass.** Sheet current,
      §Status `done`, §Harvest updated in place, commit.

**What was observed**
<!-- Verification criteria are observations, not claims. -->

**VA-1 — the enum walk, after the edit.** `StartupError` has **ten** variants,
counted off the type and not off the previous number. Each, and whether it holds
a path:

| # | variant | holds a path | names it |
|---|---|---|---|
| 1 | `NoConfigPath` | no — a unit; there *is* no path, which is the fault | n/a |
| 2 | `Usage` | no — a unit | n/a |
| 3 | `ConfigUnreadable { path, fault }` | **yes**, a `PathBuf` | yes — `"{} could not be read: {fault}"`, `path.display()` |
| 4 | `ConfigUnparseable { path, fault }` | **yes**, a `PathBuf` | yes — `"{}: {fault}"`, `path.display()` |
| 5 | `Clock(ClockError)` | no — `BeforeEpoch` and `OutOfRange` carry no path | n/a |
| 6 | `Runtime(std::io::Error)` | no — the runtime builder's error, about threads, not files | n/a |
| 7 | `Platform(slint::PlatformError)` | no | n/a |
| 8 | `EventLoop(slint::EventLoopError)` | no | n/a |
| 9 | `Enqueue` | no — a unit | n/a |
| 10 | `Ingress(IngressError)` | **yes**, inside `IngressError` | yes — `IngressError`'s own `Display` is `"{}: {}"` over `path.display()` and the fault, and `StartupError` renders it unwrapped |

Three hold a path, all three name it, the other seven hold none. The claim is
about the type: the walk is the whole enum, not the three cases.

**VA-2 — run, verbatim.** Against the debug binary, tree at this commit:

```
$ goad /nonexistent/wat.toml
goad: /nonexistent/wat.toml could not be read: No such file or directory (os error 2)
exit=2
```

The line `research.md` S-4 recorded — `goad: configuration could not be read: No
such file or directory (os error 2)` — is gone: it named no file, and this one
does. The second arm, observed against a fixture holding `[backend]\ncommand = `:

```
$ goad ./bad.toml
goad: ./bad.toml: configuration is not valid: TOML parse error at line 2, column 11
  |
2 | command =
  |           ^
string values must be quoted, expected literal string

exit=2
```

S-4's other half, which is PHASE-03's and was confirmed in passing: `goad
--version` prints `0.1.0` at exit 0, so the two invocations no longer print the
same line.

**The deny, measured rather than taken on faith.** The sheet's STOP condition
said a binding catch-all *sits directly under* `main.rs`'s
`wildcard_enum_match_arm` deny. Probed: an arm spelled `other => …` in a
`map_err(|fault| match fault { … })` over `ConfigError` fails the build —
*wildcard match will also match any future added variants* — so the lint counts
a named binding as a wildcard and the `Result`-shaped match is required, not
preferred. The probe was reverted; `cargo clippy -p goad --bin goad
--all-targets` is clean at this commit.

**VT-1 / VT-2 / EX-5.** `display_text::config_unreadable_names_the_path_and_the_fault`
and `display_text::config_unparseable_names_the_path_and_renders_the_fault_unprefixed`
were written first and failed to compile (`no variant named ConfigUnreadable`),
which is the red. `display_text::ingress_is_unwrapped_and_unprefixed_and_names_the_path`
passes **unchanged** — it does not appear in this phase's diff at all. `just
check` exits 0; `git status` shows exactly the three declared surfaces.

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **The stale count was in three doc comments, not one.** EX-3 names the enum's;
  `tests/renderer/startup.rs` carried two more — the module header's *"`Display`
  for each of its eight variants"* and `stderr_outlets`' *"a ninth variant …
  exactly as it renders the other eight"*. Fixing the class means the number
  stops being the thing maintained by hand: the enum's doc says **ten** because
  the plan requires a count there, and the two in the test file now state the
  claim without an ordinal (*every variant it has*, *every sibling*). A count
  nothing checks is what rotted twice.
- **`start` splits with early `return`s, not emit's tail expression.** The three
  arm patterns are transcribed exactly — `Err(ConfigError::Read(fault))`,
  `Err(fault)`, `Ok(..)`, matched on the `Result` — but `socket_path` *ends* at
  its match and `start` continues past it, so the two error arms return and the
  `Ok` arm binds. `clippy::wildcard_enum_match_arm` is satisfied: no top-level
  arm is a wildcard or a bare binding. `just lint` was the arbiter and passed.
- **`source_walk::startup_error_source_is_always_none` gained the two new
  variants.** Its own doc claims *every variant answers `None`* while
  enumerating a subset; naming the arms this phase introduced keeps the
  enumeration honest as the type grows. Nothing existing in it was changed.

**Findings**
<!-- Things noticed in passing that are not this phase's job. -->

- **SPEC-003's R-4 verification row now undercounts.** It says the ingress case
  is *"rendered beside its **eight** siblings"* by
  `display_text::ingress_is_unwrapped_and_unprefixed_and_names_the_path` — nine
  after this split. Canon, so not amended mid-slice: a reconciliation row for
  `audit.md`, in the *document stale, code right* column.
- **`ConfigError::Read`'s own text still reads *configuration could not be
  read*** and names no file. Untouched by design (EX-4, OQ-3, D1) — a path
  inside it would print twice in `goad-emit`. Worth knowing that the misleading
  string still exists at stratum 2; nothing at stratum 3 renders it any more.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-21 · PHASE-04 · `1bb66de`

### Produced
<!-- What now exists: modules, contracts, docs. -->

- `flake.nix` `packages.x86_64-linux` — `goad` (wrapped), `goad-emit`
  (unwrapped, no `guiLibs`), `default` = `goad`, merged over the three jail
  packages. One crane dependency layer over `--workspace`, one source filter
  spelled by directory, `doCheck = false` throughout, `GOAD_REVISION` on both
  binaries. `flake.lock` gains the `crane` entry.
- `justfile` — `just package`, outside POL-001's chain.
- `Cargo.toml` — the `tokio` entry joined, with the comment that is the only
  thing at that site able to see a re-split.
- `nix/module.nix` — the home-manager module, exported from `flake.nix` as
  `homeManagerModules.default`. Three options (`enable`, `package` with no
  default, `extraConfig`), one unit in `Unit` / `Service` / `Install`, all three
  targets `graphical-session.target`, **no `EnvironmentFile`**, and a header
  comment carrying R1 (fetch goad as a git input; a tarball prints a bare
  version and says nothing).
- **`--version`, on both binaries.** `diagnostics::version_line` /
  `print_version` in `crates/goad` and `render::version_line` in
  `crates/goad-emit`: the package version, plus the revision in parentheses
  when the build stamped one, and **no placeholder** when it did not. Each
  `main` passes
  `option_env!("GOAD_REVISION").filter(|revision| !revision.is_empty())`.
- **`Launch::Version`**, guarded before `arguments`' catch-all arm, so
  `goad --version` is answered instead of being opened as a configuration file
  of that name. `goad x --version` is still `StartupError::Usage`.
  `diagnostics::USAGE` lists the third form.
- **`crates/goad/tests/binary/`** — the crate's first binary tier, declared as
  `[[test]] name = "binary"`. One case: `--version` on stdout at exit 0, stderr
  empty. Feasible headless only because both zero-exits precede any Slint call.
- **`StartupError::ConfigUnreadable` and `ConfigUnparseable`** replace
  `Config(ConfigError)`, each carrying the `PathBuf` the host tried and
  rendering one of the two spellings already in the tree — `"{path} could not
  be read: {fault}"` and `"{path}: {fault}"`, which is `goad-emit`'s
  `startup_error_line` verbatim minus its prefix. `main::start` makes the cut
  at the seam, matching on `Config::load`'s `Result` the way emit's
  `socket_path` does. `goad_shell::error::ConfigError` is untouched.
  `StartupError` now has **ten** variants and its doc comment says ten.

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

- **crane's `filterCargoSources` admits every directory**, so a widening filter
  only has to name *files*. A filter that returns false for a directory prunes
  the subtree unvisited, which is the failure mode to expect if a widening ever
  silently admits nothing.
- **Setting `cargoExtraArgs` replaces crane's default, which is `--locked`.**
  Adding a package selector therefore drops `--locked` unless it is written
  back in. Both the spike and doctrine's flake get this wrong.
- **`nix build` from inside this checkout reads the git tree**, so an untracked
  file is invisible to it and a tracked-but-modified one is not. PHASE-02's
  `nix/module.nix` must be `git add`ed before it will build.
- **A `just demo` in the checkout blocks any second host** on
  `./goad-demo.sock`. A configuration copy with an absolute ingress path
  photographs the packaged binary without stopping the user's host.
- **A home-manager module can be rendered with no home-manager input.**
  `lib.evalModules` over three modules — a stub declaring only the options the
  module under test *sets*, the module itself, and a fragment enabling it —
  prints the generated attrset. `lib` comes from the flake's own locked
  nixpkgs (`flake.inputs.nixpkgs.lib`), and `derivation { name; system;
  builder; }` is a fake package that satisfies `types.package` without building
  anything. What it does **not** hold: the stub's types are permissive, so this
  is the module's output and not home-manager's acceptance of it.
- **`nix flake show` prints `homeManagerModules: unknown` and does not
  descend.** It has no type for a module output, so it proves the attribute
  exists and nothing more; `builtins.isFunction` on the export is the cheap
  check that it evaluates.
- **`alejandra` is on this machine's system PATH, not in the devshell**, and
  the repository's `.nix` already complies with it. Nothing on the gate reads
  `.nix` at all — neither for format nor for vocabulary.
- **`autotests = false` makes an undeclared `tests/` target silently not
  built** — measured, not assumed: with the `[[test]]` block removed,
  `cargo test -p goad` prints no `Running tests/binary/…` line, reports no
  error, and exits 0. A whole tier of cases can be added to this workspace and
  leave the gate green while never running once. Both `goad` and `goad-emit`
  carry the flag.
- **The revision is readable at compile time without touching `build.rs`.**
  `option_env!` is a macro, so `clippy.toml`'s `disallowed-methods` ban on
  `std::env::var` does not apply and no `#[expect]` is needed — unlike
  `build.rs`'s `SLINT_STYLE` read, which needed one. `.filter(|v|
  !v.is_empty())` is what makes set-but-empty read as unset, which is the case
  a flake consumed as a tarball produces (`GOAD_REVISION=""`).
- **`cargo test` never sets `GOAD_REVISION`**, so no test that spawns a binary
  can reach the stamped branch. A binary-tier `--version` case can only ever
  see the bare version; the stamped form has to be a unit case over a pure
  function taking the revision as an argument, or it is not tested at all.
- **`clippy::wildcard_enum_match_arm` counts a *named binding* as a wildcard,
  not only `_`** — measured, not assumed: an arm spelled `other => …` in a
  match on `ConfigError`, under `crates/goad/src/main.rs`'s crate-root deny,
  fails the build with *wildcard match will also match any future added
  variants*. So `map_err(|fault| match fault { … })` is not available in that
  file at all, and matching on the enclosing `Result` is a requirement rather
  than a style preference: `Err(fault)` is a tuple-struct pattern at the arm's
  top level and the binding sits inside it, which the lint does not reach.
- **A path in a stratum 3 error costs a clone and nothing else.** `start` holds
  a `&Path` and the variant holds a `PathBuf`, so the diagnostic is paid for
  with one `to_path_buf()` on a path that is already failing to start a
  process. The alternative — putting the path into `ConfigError` at stratum 2 —
  would print it twice in `goad-emit`, which renders the path itself.

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

- **A documented non-nix build path** — raised at design, 2026-09-20, and
  deliberately deferred by the user (`design-log.md`, *the non-NixOS path is
  `cargo install`, and C is a follow-up*). It is a design goal that goad runs on
  non-NixOS systems; this slice states in `design.md` that the non-nix path is
  plain `cargo install --path crates/goad --locked`, needing neither the wrapper
  nor `~/.config/goad/env`, but nothing documents or verifies it. Candidate
  scope for the follow-up slice: where that statement lives for a reader who is
  not holding this design, and whether anything checks it.
- **Four binary-tier helpers are now written twice**, once in
  `crates/goad-emit/tests/binary/exchange.rs` and once in
  `crates/goad/tests/binary/version.rs` — transcribed deliberately
  (plan §PHASE-03/VT-2: do not invent a second convention), and cheap at two
  copies. Nothing at stratum 3 is shared and neither crate may depend on the
  other, so a third binary tier is where this would need an answer rather than
  a third copy.
- **Owed at reconcile: SPEC-003's R-4 verification row undercounts the
  siblings.** It describes
  `display_text::ingress_is_unwrapped_and_unprefixed_and_names_the_path` as
  rendering the ingress error *"beside its **eight** siblings"*; after
  PHASE-04's split there are nine. Canon, so untouched mid-slice — a
  *document stale, code right* row for `audit.md`'s Reconciliation table.
- **`ConfigError::Read`'s own text still says *configuration could not be
  read*** and names no file. Deliberate (EX-4, OQ-3, D1): a path inside it
  would print twice in `goad-emit`. Nothing at stratum 3 renders that string
  any more, so this is latent rather than live — the question for a follow-up
  is whether a stratum 2 error that names no subject should carry that wording
  at all.

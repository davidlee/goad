# Notes — Slice 006: packaging and the startup surface

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the crane packages | done | 2026-09-21 |
| PHASE-02 — the home-manager module | done | 2026-09-21 |
| PHASE-03 — `--version`, on both binaries | pending | |
| PHASE-04 — the configuration path, named | pending | |
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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-21 · PHASE-02 · `7914864`

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

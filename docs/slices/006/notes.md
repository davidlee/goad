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
| PHASE-05 — the cutover, and the evidence | done | 2026-09-21 |

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

### PHASE-05 — the cutover, and the evidence

**Objective:** the software has been run: a nix-built binary under systemd, a
window with text in it, and a `--version` that tells the two install paths
apart.

**How this phase is run** — endorsed by the user, 2026-09-21: **an agent preps,
the person executes.** Nothing outside `/home/david/dev/goad` is written by an
agent. The agent takes the machine-side criteria and drafts the `~/flakes`
consumer *as text in this sheet*; the person applies it, switches, cuts the
service over, and makes the three VH observations. The orchestrator writes what
the person reports back into this sheet.

This is not a softening of the criteria. VH-1, VH-2 and VH-3 are human
acceptance in the plan and always were — `docs/AGENTS.md` §Tiers: *a slice does
not close until a person has run the software and seen the new behaviour*, and
no green gate is that evidence.

**Reading list**

- `docs/slices/006/plan.md` §PHASE-05 — the contract: EX-1..EX-5, VH-1..VH-3,
  VA-1..VA-3, and its four *Notes for the implementer*. §5.4's sequence is the
  one thing in this phase that must not be reordered.
- `docs/slices/006/design.md` §5.4 — the cutover's lifecycle.
- `docs/slices/006/slice-006.md` §Acceptance criteria — AC-1, AC-2, AC-5, AC-7,
  AC-8 and AC-9 all finish here or are re-observed here.
- `~/flakes/modules/home/linux/satan-attrd.nix` — the consumer pattern, and
  `~/flakes/flake.nix`'s `satan-attrd` input block. **Read-only.**
- `nix/module.nix` — the module's own header already carries the four-line
  consumer and the git-input warning.
- PHASE-01's §What was observed above — VA-7's `goad-shot` invocation, which is
  the nearest thing to VH-1 that an agent can reach, and is **not** VH-1.

**The one place the precedent must not be copied**

`~/flakes` wires `satan-attrd` as `url = "path:/home/david/dev/satan-attrd"`.
**goad cannot use a `path:` input.** The repository root holds `goad-demo.sock`,
a unix socket nix refuses outright, and `.claude/worktrees/` holds gitignored
worktrees it would copy anyway (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).
The input is the **bare git form**, `git+file:///home/david/dev/goad`, which
satan-attrd's own header says it should itself become. Two consequences the
person should know before switching:

- a git input reads the **committed** tree, so anything uncommitted is invisible
  to the switch — EN-1 exists for this;
- `self.shortRev` exists over a git input, so `goad --version` prints a
  revision. Over a **tarball** URL it would not, and AC-5 would fail silently
  (R1, and the module header says so).

**Assumptions & STOP conditions**

- **The agent writes nothing outside `/home/david/dev/goad`.** Not `~/flakes`,
  not `~/.config/systemd/user/`, not `~/satan/`. It drafts; it does not apply.
- **`just install` (VA-2) writes to `~/.cargo/bin` and `~/.config/goad/env`.**
  That is outside the repository, and it is the person's to run, not the
  agent's — it replaces the very binary VA-1 compares against, and ordering is
  the criterion (F-8).
- **`~/.cargo/bin/goad` is running right now** (PHASE-01 finding), as is a
  `target/debug/goad` on `./goad-demo.sock`. The cutover replaces the first.
- **§5.4's sequence is not reorderable**: stand up the consumer, move the
  service onto the module's unit, *then* remove the hand-written unit.
  Removing first leaves nothing running if the module is wrong.
- **A defect found here is repaired in PHASE-01 or PHASE-02, not patched in
  `~/flakes`.** Nothing in this phase is a deliverable of the repository.
- `docs/roadmap.md` is updated **at close**, not here.

**Tasks — agent-reachable**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->

- [x] T-1 — EN-1: confirm PHASE-01..04 are all `done` here and committed, and
      the tree is clean. A git input cannot see an uncommitted `flake.nix`.
- [x] T-2 — VA-3: `just package` runs green from the committed tree. Record
      both store paths.
- [x] T-3 — read `~/flakes/flake.nix`'s input block and
      `~/flakes/modules/home/linux/satan-attrd.nix`, then **draft, in this
      sheet, as text**: the input entry for goad in the bare git form, and the
      consumer module on the `satan-attrd.nix` pattern — header comment
      included, in that file's voice. Say which file each goes in and where.
      **Do not write either file.**
      *Done.* Three files, not two — see below. Nothing in `~/flakes` was
      written.
- [x] T-4 — draft the exact command sequence the person will run, in §5.4's
      order, with the observation to make after each. One command per line,
      copy-pasteable, absolute where it matters.
- [x] T-5 — VA-1's first half, agent-side: record what `./result/bin/goad
      --version` prints from the committed tree, verbatim. The comparison is
      the person's, after VA-2.
      *Done*, from the store path rather than `./result` — `just package` uses
      `--no-link` and builds no out-link, and the store path is the same
      artefact.

**Tasks — person-only**
<!-- These are not agent-reachable. Prose defers; a checklist box catches.
     `docs/memory/a-deferred-step-needs-a-checklist-box.md`. -->

- [ ] P-1 — EX-1: apply the drafted input and consumer to `~/flakes`.
- [ ] P-2 — EX-2: switch, and the user service runs from the module's unit.
- [ ] P-3 — VH-2: `systemctl --user cat goad` — confirm `Restart=on-failure`,
      `RestartPreventExitStatus=2`, `RestartSec=2`, an `ExecStart` that is a
      **store path**, `Description`, and **no `EnvironmentFile=`**.
- [ ] P-4 — VH-1: the window, **with text in it**. AC-8 and AC-1's second half.
      No green gate is this evidence.
- [ ] P-5 — VH-3: the nix-built `goad-emit` puts an envelope into the running
      host's socket and the host reacts.
- [ ] P-6 — EX-3: remove `~/satan/goad/goad.service` and its symlink in
      `~/.config/systemd/user/` — **after** P-2 is observed, never before.
- [ ] P-7 — VA-2: `just install` from the dev shell, then VA-1's comparison.
      **This ordering is the criterion** (F-8): today's `~/.cargo/bin/goad`
      predates the slice and answers `--version` with exit 2, which "differs"
      while demonstrating nothing. **Both invocations must exit 0 and both must
      print a version line**, agreeing on the package version and differing
      only in the parenthetical.
- [ ] P-8 — EX-4: `~/.config/goad/env` still exists and `just install` still
      wrote it. Not removed by this cutover.
- [ ] P-9 — EX-5: report each observation; the orchestrator writes it into
      §What was observed below, naming what was seen, so `audit.md` can cite it
      rather than re-run it.

#### T-3 — the drafted `~/flakes` changes

Not written. Text only, for the person to apply as P-1.

**Three files, not one.** A module file under `~/flakes/modules/home/linux/` is
inert until something imports it: `satan-attrd.nix` is named in
`~/flakes/hosts/Sleipnir/home.nix` and nowhere else. So the consumer is (a) an
input, (b) a new module file, (c) one line in the host's home config.

**And one `git add`.** `~/flakes` is **not its own git repository** —
`git -C ~/flakes rev-parse --show-toplevel` answers `/home/david`. The flake
ref `.` that `just home-switch` passes therefore resolves against the `$HOME`
git tree, and an **untracked** file is invisible to it. This is PHASE-02's VA-3
again, one directory up: `git add` the new module before switching, or
home-manager will evaluate a configuration that does not contain it. (Tracked
files that are merely *modified* are visible, so `flake.nix` and `home.nix`
need no special handling — but staging all three together is simpler than
remembering which.)

**(a) `~/flakes/flake.nix` — the input.** Inside `inputs = { … }`, after the
`satan-attrd` block and before the long `oubliette` comment:

```nix
    # goad — personal intervention shell. Local checkout at ~/dev/goad.
    #
    # A *git* input, never `path:`. The repository root holds a live unix
    # socket (`goad-demo.sock`) that nix refuses to copy outright, and
    # `.claude/worktrees/` holds gitignored agent worktrees a `path:` copy
    # would take anyway.
    #
    # The git form is also what makes `goad --version` worth anything: the
    # build stamps `self.shortRev` into the binary, and a *tarball* URL has no
    # such attribute — the version line then prints a bare `0.1.0` with no
    # revision, and nothing anywhere reports that it happened.
    #
    # A git input reads the **committed** tree: anything uncommitted in
    # ~/dev/goad is invisible to a switch. `nix flake update goad` advances the
    # pin.
    goad.url = "git+file:///home/david/dev/goad";
```

**No `follows`, deliberately** — the one judgement call in this draft, and the
plan does not settle it. goad pins `nixpkgs` and `rust-overlay` as a pair, and
crane builds the binary against the toolchain that overlay resolves out of
*that* nixpkgs; `inputs.nixpkgs.follows = "nixpkgs-home"` would build a closure
other than the one `just package` verified here, which is the artefact this
phase exists to run. The cost of not following is one more nixpkgs in
`~/flakes/flake.lock` — which this flake already accepts on purpose for
`llm-agents`. If you would rather share the copy, add the follows *after* the
cutover is observed, and re-observe VH-1 on the rebuilt binary.

**(b) `~/flakes/modules/home/linux/goad.nix` — a new file**, whole:

```nix
# goad — personal intervention shell.  A Slint window the host owns and a
# user-supplied backend drives; one JSON document per exchange over stdio,
# plus an ingress socket goad-emit writes to.  The host understands none of
# the domain — items, slots and the record format are backend.py's business.
#
# Source: ~/dev/goad (git+file:// input — a git input, never `path:`: the
# repo root holds a live unix socket nix refuses to copy, and only a git
# input carries the self.shortRev that `goad --version` prints).
#
# State + artefacts:
#   Config at ~/.config/goad/config.toml — the path `goad` with no argument
#   reads.  It names the backend command and the ingress socket.
#   Backend and its per-day records: ~/satan/goad/ (backend.py, data/).
#   ~/.config/goad/env belongs to the *cargo* install path (`just install`)
#   and is NOT read by this unit: the packaged binary is wrapped and carries
#   LD_LIBRARY_PATH and FONTCONFIG_FILE itself, so there is no second file
#   to drift against the build the unit runs.
#
# Replaces the hand-written unit at ~/satan/goad/goad.service.
#
# A refusal is exit 2 and the unit's RestartPreventExitStatus stops rather
# than loops; the diagnostic naming the configuration file is on stderr.
#
# Smoke:
#   systemctl --user status goad
#   journalctl --user -u goad -f
#   goad --version          # 0.1.0 (<rev>) — a bare 0.1.0 means a tarball
{
  inputs,
  pkgs,
  ...
}: {
  imports = [inputs.goad.homeManagerModules.default];

  services.goad = {
    enable = true;
    package = inputs.goad.packages.${pkgs.system}.goad;
  };
}
```

Exactly the `satan-attrd.nix` shape, and exactly the four-line consumer
`nix/module.nix`'s own header prescribes. `goad-emit` is deliberately **not**
installed by this: the module puts `cfg.package` on `home.packages` and nothing
else, and VH-3 runs the packaged `goad-emit` from its store path.

**(c) `~/flakes/hosts/Sleipnir/home.nix` — the import.** In the `imports` list,
under the `# machine-specific` comment, after the `satan-attrd.nix` line:

```nix
    ../../modules/home/linux/goad.nix
```

**Determined by reading, and worth knowing before you switch**

- `extraSpecialArgs` on `homeConfigurations."david"` passes `inputs`, and
  `pkgs` is the module argument home-manager always supplies, so both names the
  consumer uses are in scope — the same two `satan-attrd.nix` relies on.
- goad's flake fixes `system = "x86_64-linux"` and exports
  `packages.x86_64-linux` only; the home config's `pkgs.system` is
  `x86_64-linux`. They meet.
- `~/flakes` currently has uncommitted work in `flake.nix`, `flake.lock`,
  `hosts/Sleipnir/config.nix`, `modules/nixos/greeter.nix`,
  `modules/nixos/umbriel.nix` and `modules/shared/cli/_packages/dev.nix`. A
  switch takes all of it. Nothing to do about it here — just know that a
  failed switch may be about something other than goad.

**Not determined by reading — handled by ordering instead**

`~/.config/systemd/user/goad.service` is today a symlink to
`~/satan/goad/goad.service`, and it sits on exactly the path home-manager wants
for its generated unit. Standalone home-manager refuses to clobber a file it
does not own (*Existing file … is in the way*) and this configuration sets no
`backupFileExtension` — only `darwin/default.nix` does. Whether the switch
would abort or quietly take the path was not settled by reading, and settling
it empirically means running `home-manager build`, which writes outside this
repository. **The sequence removes the symlink before the switch instead**,
which makes the question moot and costs nothing: `~/satan/goad/goad.service`
stays on disk until P-6, so one `ln -s` puts the old service back.

#### T-4 — the sequence, for the person

§5.4's order: stand up the consumer, move the service onto the module's unit,
*then* retire the hand-written one. Each block says what to look for. Do not
run P-6 until P-2 through P-5 have all been seen.

**P-1 — apply the three fragments (EX-1)**

```sh
$EDITOR /home/david/flakes/flake.nix                       # fragment (a)
$EDITOR /home/david/flakes/modules/home/linux/goad.nix     # fragment (b), new
$EDITOR /home/david/flakes/hosts/Sleipnir/home.nix         # fragment (c)
git -C /home/david add flakes/flake.nix flakes/modules/home/linux/goad.nix flakes/hosts/Sleipnir/home.nix
git -C /home/david status --short flakes/
```

Observe: all three listed as staged. An untracked `goad.nix` is invisible to
the switch — see T-3.

```sh
cd /home/david/flakes && nix flake lock
grep -A4 '"goad"' /home/david/flakes/flake.lock | head -20
```

Observe: a `goad` node appears, `"type": "git"`, `"url":
"file:///home/david/dev/goad"`, and a `"rev"` that starts with whatever

```sh
git -C /home/david/dev/goad rev-parse --short HEAD
```

answers. **Compare against that, not against a revision written here** — this
sheet is committed to the tree it describes, so every literal revision in it is
one commit stale the moment it lands. A `"type": "tarball"` is the failure the
input comment warns about. (This step is optional: the
switch locks anyway. It is worth doing alone because it separates *the input
resolves* from *the switch works*.)

Optional, and cheap:

```sh
cd /home/david/flakes && nix fmt
```

The flake's own treefmt (alejandra + statix). Re-`git add` if it reformats.

**Before the switch — free the unit name, keep the fallback**

```sh
systemctl --user stop goad
systemctl --user is-active goad
ls -l /home/david/.config/systemd/user/goad.service
rm /home/david/.config/systemd/user/goad.service
```

Observe: `inactive`; the `ls` shows the symlink still pointing at
`/home/david/satan/goad/goad.service` before you remove it.

Two reasons, both hard:

1. home-manager writes its generated unit to exactly that path and will not
   clobber a file it does not own (T-3's last paragraph).
2. The old host holds the ingress socket `/run/user/1000/goad.sock`. A second
   host on the same socket is a `StartupError`, which is exit 2, which
   `RestartPreventExitStatus=2` then declines to retry — you would switch
   successfully into a stopped service.

This is **not** EX-3. The unit file itself is untouched; only the symlink goes.

**P-2 — switch (EX-2)**

```sh
cd /home/david/flakes && just home-switch
systemctl --user status goad
```

`home-switch` is `home-manager switch --flake '.#david'` behind a
`check-flake-root` guard, so the `cd` is not cosmetic: the `.` resolves against
the working directory.

Observe: the switch exits 0 and its activation output mentions
`goad.service`; `status` then shows `active (running)` and a `Loaded:` line
whose path is `/home/david/.config/systemd/user/goad.service` — now
home-manager's symlink into the store, not the old one into `~/satan`.

If either fails, the recovery block at the end of this section puts the old
service back. **Do not repair a module or package defect in `~/flakes`** — it
is a finding against PHASE-01 or PHASE-02 and is repaired there.

**P-3 — VH-2, the unit systemd actually loaded (AC-7)**

```sh
systemctl --user cat goad
systemctl --user cat goad | grep -c EnvironmentFile
```

Observe, by name:

| expect | value |
|---|---|
| `Description=` | `goad — personal intervention shell` |
| `ExecStart=` | a **store path** ending `/bin/goad`. What it should equal: `nix path-info /home/david/dev/goad#goad` run from a clean tree. It is a store path that matters here, not which one — a path under `/home/david` would mean the module took something other than the package |
| `Restart=` | `on-failure` |
| `RestartPreventExitStatus=` | `2` |
| `RestartSec=` | `2` |
| `EnvironmentFile=` | **absent** — the `grep -c` prints `0` |

`Description` is in the module but **not** in PHASE-02's rendered attrset
recorded above in this file: that render predates commit `c67262e`, which added
it. The unit you are reading is current; there is no discrepancy to chase.

**P-4 — VH-1, the window, with text in it (AC-8, AC-1's second half)**

Look at the screen. Both halves are the criterion: a window, **and** text
drawn in it. No command prints this and no green gate is this evidence.

If there is no window at all:

```sh
journalctl --user -u goad -n 50 --no-pager
```

A window that opens but draws **no text** is `FONTCONFIG_FILE` — a packaging
defect in the wrapper, repaired in PHASE-01, never patched in `~/flakes`.

**P-5 — VH-3, an envelope into the running host (AC-2's second half)**

```sh
EMIT=$(nix build --no-link --print-out-paths /home/david/dev/goad#goad-emit)
"$EMIT/bin/goad-emit" --source cutover --kind smoke
echo "exit=$?"
journalctl --user -u goad -n 20 --no-pager
```

The store path is deliberate: this is the **nix-built** `goad-emit` VH-3 asks
for, and the module installs only `goad` on PATH. No `--socket` — emit reads
the ingress path out of `~/.config/goad/config.toml`, which names
`/run/user/1000/goad.sock`, and the host under the new unit is the one holding
it.

Observe: **exit 0** (the host accepted the event) or **exit 1** (the host
refused it) — either is the host reacting, which is what VH-3 asks. **Exit 2 is
not**: it means no usable answer could be had — nothing listening on that
socket, or a path mismatch. The window should also move to whatever the backend
answered with.

**P-6 — EX-3, retire the hand-written unit. Only after P-2..P-5.**

```sh
rm /home/david/satan/goad/goad.service
ls -l /home/david/.config/systemd/user/goad.service
ls /home/david/satan/goad/
```

Observe: the `.config` entry is home-manager's store symlink (it was replaced
at P-2); `~/satan/goad/` keeps `backend.py`, `data/`, `justfile`, `README.md`
and `field-notes.md` — only the unit goes.

**After this the recovery block below no longer exists.** That is precisely why
it is last.

**P-7 — VA-2, then VA-1's comparison (AC-9, AC-5)**

From the repository's dev shell — direnv, or `nix develop`. `just install`
refuses to run outside it: both environment variables it writes come from
`flake.nix`, and an env file naming two empty values is the silent breakage
written down.

```sh
just install
ls -l /home/david/.cargo/bin/goad /home/david/.cargo/bin/goad-emit
```

Observe: exits 0; both binaries have just-updated mtimes.

Then VA-1, **in this order** (F-8):

```sh
/home/david/.cargo/bin/goad --version ; echo "exit=$?"
NIXGOAD=$(nix build --no-link --print-out-paths /home/david/dev/goad#goad)
"$NIXGOAD/bin/goad" --version ; echo "exit=$?"
```

Expect `0.1.0` at exit 0 from the cargo path — `just install` sets no
`GOAD_REVISION`, so there is no parenthetical — and `0.1.0 (<short rev>)` at
exit 0 from the nix path, the revision being whatever
`git -C /home/david/dev/goad rev-parse --short HEAD` answers. **Both must exit 0 and both must print a version line**,
agreeing on `0.1.0` and differing only in the parenthetical. An exit 2 on
either side fails VA-1 rather than passing it: that is today's pre-slice binary
reading `--version` as a configuration path, which "differs" while
demonstrating nothing. Record both strings verbatim.

`goad-emit` answers too, if you want the pair:

```sh
/home/david/.cargo/bin/goad-emit --version ; echo "exit=$?"
```

**P-8 — EX-4, the env file survives (OQ-6)**

```sh
ls -l /home/david/.config/goad/env
cat /home/david/.config/goad/env
```

Observe: it exists, its mtime is from the `just install` you just ran, and it
names two non-empty store paths — `LD_LIBRARY_PATH` and `FONTCONFIG_FILE`.
Nothing automatic reads it any more; the packaged binary is wrapped and the
module's unit names no `EnvironmentFile`. It belongs to the cargo path and the
cutover does not remove it.

**Recovery — valid until P-6, and only until then**

If the switch fails, or the new unit will not start and you want the old
service back now:

```sh
systemctl --user stop goad
rm -f /home/david/.config/systemd/user/goad.service
ln -s /home/david/satan/goad/goad.service /home/david/.config/systemd/user/goad.service
systemctl --user daemon-reload
systemctl --user start goad
systemctl --user status goad
```

A stopgap, not a revert: the next `just home-switch` puts home-manager's unit
back. To revert properly, undo the three P-1 edits and switch again.

This is the whole reason §5.4's sequence is what it is —
`~/satan/goad/goad.service` is still on disk, so the fallback is one `ln -s`.
After P-6 it is not.

**What was observed**
<!-- Verification criteria are observations, not claims. The VH rows are the
     person's own words about what they saw. -->

*The agent half. VH-1..VH-3, VA-2 and VA-1's comparison are the person's and
are written here when reported (P-9).*

- **T-1, EN-1** — PHASE-01 through PHASE-04 all read `done` in §Status above.
  `git status --porcelain` printed nothing; `git log --oneline -1` was
  `af01ead 006: PHASE-05's phase sheet — the cutover, and the evidence`. The
  committed tree and the working tree are the same tree, so the git input has
  everything.
- **T-2, VA-3** — `just package` exited 0 from that tree. Two store paths,
  verbatim:

  ```
  /nix/store/rgsn4p821bqmkjq0w9j6s2g98l23iq06-goad-0.1.0
  /nix/store/zmpmc6xkmyj7p7hp7dfdr7415j35y1kz-goad-emit-0.1.0
  ```

  The build was warm and re-derived both packages; `--no-link` means there is
  no `./result` and the tree stayed clean.
- **T-5, VA-1's first half** — from the store path above, verbatim, both at
  exit 0:

  ```
  $ /nix/store/rgsn4p821bqmkjq0w9j6s2g98l23iq06-goad-0.1.0/bin/goad --version
  0.1.0 (af01ead)
  $ /nix/store/zmpmc6xkmyj7p7hp7dfdr7415j35y1kz-goad-emit-0.1.0/bin/goad-emit --version
  0.1.0 (af01ead)
  ```

  `git rev-parse --short HEAD` is `af01ead`, so the parenthetical is this
  commit and `self.shortRev` reached the binary. **This is a sighting of AC-5,
  not its discharge** — the comparison is VA-1's and is the person's, after
  VA-2 (P-7).
- **The pre-slice cargo binary, for contrast, and why P-7's order is the
  criterion** — `~/.cargo/bin/goad --version` today prints
  `goad: configuration could not be read: No such file or directory (os error
  2)` and exits **2**. It reads `--version` as a configuration path: it
  predates PHASE-03, and its message predates PHASE-04's named path too. It
  "differs" from a stamped version line while demonstrating nothing AC-5 asks
  for (F-8, `research.md` S-4). After P-7's `just install` it must print
  `0.1.0` at exit 0.
- **The live service, as found** — `goad.service` is `active`, PID 4024,
  `FragmentPath=/home/david/.config/systemd/user/goad.service`, which is a
  symlink to `/home/david/satan/goad/goad.service`. Its host holds
  `/run/user/1000/goad.sock` — the ingress path `~/.config/goad/config.toml`
  names. Both facts shape the sequence: the symlink occupies the path
  home-manager wants, and the socket cannot be held by two hosts.

*The cutover itself, 2026-09-21. The boundary set at the start of this phase —
an agent prepares, the person executes — was lifted by the person partway
through ("you get it working"), so everything below from the switch onward was
run by the orchestrator except VH-1, which cannot be delegated.*

- **The sequence was not run in its stated order, and P-6 went first.** Before
  any of P-1..P-5, `rm ~/satan/goad/goad.service` and `rm
  ~/.config/systemd/user/goad.service` were both run, then `daemon-reload`. The
  symlink removal is the pre-switch step and was correct; the other is EX-3,
  and it destroyed the fallback the whole of §5.4's ordering exists to keep. No
  harm followed — the host process survived as an orphan (`LoadState=not-found`
  with `ActiveState=active`, MainPID still serving), and the unit's text was
  recovered verbatim from the terminal's own `systemctl --user cat` output and
  written back. But the recovery block named an artefact that exists in no
  repository, so its only copy was scrollback. See §Findings.
- **P-1, as found: the consumer was a stub.**
  `~/flakes/modules/home/linux/goad.nix` had been created with the `imports`
  line alone — no `services.goad.enable`, no `package`. `enable` defaults
  false and `package` carries no default by design (PHASE-02), so a switch on
  that file installs no unit and no package and reports success. Completed to
  T-3's fragment (b) before switching. Fragments (a) and (c) were already
  applied and staged.
- **P-1, the lock: the goad input had locked a dirty tree.**
  `~/flakes/flake.lock` held `"dirtyRev":
  "22f412c1afb6e07dc984b4d10f5c538d5949c406-dirty"`. Cause was in this
  repository, not that one: `flake.lock` here had been rewritten by a `nix
  flake update` at 10:58 (every input advanced) and left uncommitted, and a
  `git+file://` input reads a dirty working tree as dirty. `flake.nix`'s
  `revision` binding is `self.shortRev or self.dirtyShortRev or ""`, so the
  cutover would have installed a binary printing `0.1.0 (22f412c-dirty)` —
  honest, but not the artefact PHASE-01 verified, and VA-1's comparison would
  have been against a build nothing else describes. Repaired by restoring the
  committed lock (`git show HEAD:flake.lock > flake.lock`; the working tree
  then matched HEAD at `22f412c`) and re-locking with `nix flake update goad`,
  which resolved to `ref=refs/heads/main&rev=22f412c1…` with no dirty
  attribute and rolled goad's five transitive inputs back to the committed
  pins.
- **T-2 re-observed on the clean tree.** `just package` exited 0. The store
  paths differ from those recorded above because `GOAD_REVISION` is part of
  the derivation and the revision advanced `af01ead` → `22f412c`:

  ```
  /nix/store/225f81n4mnwxs5bgp95k3lrbbwix1881-goad-0.1.0
  /nix/store/1k83k1kas8bl8nc8fxayv5kdp0x18z3h-goad-emit-0.1.0
  ```

- **P-2, the switch.** `cd ~/flakes && just home-switch` exited 0. Six
  derivations built, `goad.service.drv` among them, and the activation's
  closing line was `Starting units: goad.service, stasis.service`.
- **VH-2 (AC-7) — the unit systemd loaded.** Every row asked for, verbatim
  from `systemctl --user cat goad`:

  ```
  # /home/david/.config/systemd/user/goad.service
  #   -> /nix/store/dlix0wqckvifqmc9xg3bskav89dbhmg8-goad.service/goad.service
  ExecStart=/nix/store/225f81n4mnwxs5bgp95k3lrbbwix1881-goad-0.1.0/bin/goad
  Description=goad — personal intervention shell
  Restart=on-failure
  RestartPreventExitStatus=2
  RestartSec=2
  ```

  `EnvironmentFile` absent — `grep -c EnvironmentFile` printed `0`. The
  `ExecStart` store path is character-for-character the one `just package`
  printed, so the module took the package and not something under `/home`.
  `systemctl --user status` then read `active (running)`, `Loaded:` naming the
  `.config` path, `Main PID: 464078 (.goad-wrapped)` — the wrapper is what
  systemd supervises, so `LD_LIBRARY_PATH` and `FONTCONFIG_FILE` are being set
  by the package rather than by any file beside it.
- **VH-1 (AC-8, AC-1's second half) — the window, with text in it. The
  person's own words: "diagnostics window shows: nothing to report."** Taken
  from the tray menu's **Diagnostics** item rather than from a scheduled
  showing, because no slot was due and `backend.py` may legitimately answer a
  forced check with nothing to display — which would have been no evidence
  either way. The diagnostics pane draws unconditionally, so a window
  containing glyphs is exactly the criterion and the backend having nothing to
  report is orthogonal to it. Fontconfig is reaching the renderer.
- **The tray icon is now created, and previously was not.** Every prior start
  in the journal carries `Slint: Failed to create system tray icon: 0` — the
  cargo binary under the hand-written unit, on 2026-09-20 16:44 and
  2026-09-21 09:04. The packaged binary's start at 13:35 logs no such line and
  the icon is drawn. Unasked-for by any criterion and worth recording: the
  wrapper repaired a GUI-stack defect nobody had attributed to packaging.
- **VH-3 (AC-2's second half) — an envelope into the running host.** The
  nix-built `goad-emit` at
  `/nix/store/1k83k1kas8bl8nc8fxayv5kdp0x18z3h-goad-emit-0.1.0/bin/goad-emit
  --source cutover --kind smoke` exited **0**: the host accepted the event.
  No `--socket`; emit read the ingress path out of
  `~/.config/goad/config.toml` and found the new host holding it.
- **The stale socket was reclaimed, not tripped over.** The old host's
  `/run/user/1000/goad.sock` was left on disk by the out-of-order teardown
  with nothing listening. The new host rebound it at 13:35. This is `reclaim`
  in `goad-shell`'s `ingress` doing what it documents — probe, take the lock
  beside the path, unlink the stale file, bind — and it is the reason the
  misordered teardown cost nothing.
- **P-6 (EX-3) — the hand-written unit retired**, after VH-1..VH-3 and not
  before, on the restored copy. `~/.config/systemd/user/goad.service` is
  home-manager's symlink into
  `/nix/store/q8zqz2pyx2sb3ymin79bdnpp0ijmx8fg-home-manager-files`;
  `~/satan/goad/` keeps `backend.py`, `data/`, `field-notes.md`, `justfile`
  and `README.md`. Only the unit went.
- **P-7, VA-2 then VA-1 (AC-9, AC-5).** `nix develop --command just install`
  by the person, exit 0; both cargo binaries and `~/.config/goad/env` rewritten
  at 13:43. The comparison, all four at exit 0:

  ```
  ~/.cargo/bin/goad            --version → 0.1.0
  ~/.cargo/bin/goad-emit       --version → 0.1.0
  <store>/bin/goad             --version → 0.1.0 (22f412c)
  <store>/bin/goad-emit        --version → 0.1.0 (22f412c)
  ```

  The parenthetical is the only difference, on both binaries, which is what
  AC-5 asks: the packaged path carries the revision and the cargo path does
  not, and neither is broken by the other existing. The cargo `goad` exiting
  **0** here is the whole of AC-9 — before this it exited 2, reading
  `--version` as a configuration path.
- **P-8, EX-4 (OQ-6) — the env file survives.** `~/.config/goad/env` exists,
  mtime 13:43 from that `just install`, and names two non-empty store paths.
  Nothing automatic reads it any more: the packaged binary is wrapped and the
  module's unit names no `EnvironmentFile`. It belongs to the cargo path and
  the cutover left it alone.
- **The running service was unaffected by P-7.** `MainPID=464078`,
  `ExecMainStartTimestamp` still 13:35:30 — `just install` writes to
  `~/.cargo/bin` and touches nothing systemd supervises.

**Decisions taken during execution**

- **The consumer is three files, not one, and the third is
  `hosts/Sleipnir/home.nix`.** A module under `~/flakes/modules/home/linux/` is
  inert until the host's `imports` names it. EX-1 describes the module's
  content and is silent on reachability; drafting only the content would have
  produced a switch that changed nothing and a cutover that appeared to fail.
- **The goad input takes no `follows`.** Not settled by the plan. Reasoning and
  the alternative are in T-3 under fragment (a).
- **The symlink at `~/.config/systemd/user/goad.service` is removed before the
  switch, not after.** §5.4's sequence is about not destroying the fallback
  before the replacement is proven; the fallback is
  `~/satan/goad/goad.service`, which stays on disk until P-6. Removing the
  symlink frees the path home-manager needs without touching the fallback, and
  it stops the host that holds the ingress socket. EX-3 is still P-6.
- **T-5 was taken from the store path, not `./result`.** `just package` uses
  `--no-link` and produces no out-link; building one would have added an
  untracked symlink for no gain. Same artefact, same bytes.
- **The phase's boundary was lifted mid-execution, by the person.** This phase
  was planned as "agent preps, you execute" precisely because it writes outside
  this repository. Partway through the person said "you get it working", which
  moved the switch, the `~/flakes` repair and the verification back to the
  orchestrator. VH-1 stayed with the person because it is an observation no
  process can make on their behalf, not because of the boundary.
- **The committed `flake.lock` was restored rather than the update
  committed.** The alternative was to accept the 10:58 `nix flake update` and
  re-run the phase gate against it. Rejected: an unrequested toolchain and
  nixpkgs advance in the middle of a cutover changes the artefact under the
  criteria that are measuring it, and every store path recorded in this phase
  would have needed re-deriving to say anything. The update is one
  `nix flake update` away whenever it is wanted deliberately — §Open carries
  it.
- **VH-1 was taken from the tray's Diagnostics pane, not a scheduled
  showing.** The criterion is that the packaged binary draws a window with
  text in it — a fontconfig property. A scheduled showing additionally depends
  on `backend.py` having something to say at that moment, which is domain
  behaviour this host deliberately knows nothing about, and "Check now" can
  answer with no window at all without that being evidence of anything. The
  `Tray` component's `Menu` in `ui/app.slint` offers `Diagnostics`, which
  draws unconditionally. Strictly stronger evidence for the criterion, and
  independent of the backend.

**Findings**

- **`~/flakes` is not its own git repository.** Its root is `/home/david`, so
  the flake ref `.` resolves against the `$HOME` git tree and an untracked file
  under `~/flakes` is invisible to a switch. Same class as PHASE-02's VA-3, one
  directory up, and the reason P-1 stages all three files.
- **Standalone home-manager on this machine sets no `backupFileExtension`** —
  only `darwin/default.nix` does — so a foreign file on a generated unit's path
  has no fallback behaviour to rely on. Not verified empirically: doing so
  means `home-manager build`, which writes outside this repository. The
  sequence sidesteps it.
- **PHASE-02's recorded render lacks `Description`, and that is not a defect.**
  The attrset recorded under PHASE-02 §What was observed was taken before
  commit `c67262e`, which added `Unit.Description`. VH-2 asks for
  `Description`; the current module has it. Worth knowing before someone reads
  the two side by side at audit.
- **`just install` stamps no revision**, so the cargo path prints a bare
  `0.1.0`. That is what makes VA-1's pair differ in the parenthetical alone —
  it is a property of the recipe, not of the comparison, and a future
  `GOAD_REVISION` in `just install` would quietly make VA-1 unfalsifiable.
- **A recovery block is only as durable as the artefact it names, and this one
  named a file under no version control.** §T-4's recovery step was `ln -s
  ~/satan/goad/goad.service …`, correct for every failure it anticipated and
  worth nothing against the one that happened: the file being removed. That
  path is not tracked — `/home/david` is a git repository and it was never
  added — and the unit's text appears verbatim in no document here;
  `research.md` paraphrases three of its directives and `design.md` describes
  the module that replaces it. Recovery worked only because the person had run
  `systemctl --user cat` in the same terminal minutes earlier. A sequence that
  is going to destroy something should carry the thing it destroys, or say
  where a copy is.
- **The consumer can be written wrong in a way that switches green.** A
  home-manager module file that imports `inputs.goad.homeManagerModules.default`
  and sets nothing installs nothing: `enable` defaults false, and the switch
  succeeds. The module's own header (`nix/module.nix`) prescribes the
  four-line consumer, but nothing detects a three-line one. Same shape as
  PHASE-02's VA-3 and T-3's `git add` caveat — a switch reporting success is
  not evidence that it took the module.
- **A dirty working tree changes what a `git+file://` consumer builds, and the
  version line is where it shows.** `flake.nix`'s
  `self.shortRev or self.dirtyShortRev or ""` means a dirty tree stamps
  `<rev>-dirty` rather than failing, so the tell is visible — but only to
  someone reading the version line, and the consumer's lock records it as
  `dirtyRev` where nobody looks. The `-dirty` suffix is the designed-in
  witness and it worked; what nothing holds is that an uncommitted change in
  this repository silently redefines what a switch elsewhere installs.
- **The stale-socket path was exercised in production for the first time.**
  `reclaim`'s documented sequence handled a socket left by a host killed out
  from under its unit. Nothing was arranged to test this; the misordered
  teardown produced the condition and the host rebound cleanly at 13:35.
- **The system tray icon creates now and did not before, and the environment
  is not the reason.** Journal lines before the cutover carry `Slint: Failed to
  create system tray icon: 0` on every start of the cargo binary under the
  hand-written unit; the packaged binary logs none and draws the icon. The
  first explanation reached for — that the wrapper carries a library
  `~/.config/goad/env` lacked — is **wrong, and was checked**: after P-7 the
  wrapper script and the env file name the same five store paths (gcc-lib,
  fontconfig-lib, libglvnd, libxkbcommon, wayland) and the same `fonts.conf`.
  Byte-identical. Whatever the difference is, it is not `guiLibs` and not
  `FONTCONFIG_FILE`. See §Open.

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
- **Owed at reconcile: SPEC-003 cites three source sites by line number, and
  all three are wrong.** Found by the orchestrator at `4ff4adb` while checking
  the row above. In R-4's verification row, `crates/goad/src/main.rs:21-29` for
  `main`'s single `match run()` — it is at `:35` today, and the citation was
  **already wrong before this slice opened**: at `4f9fb9d` lines 21-29 were
  `use` declarations. PHASE-04 moved it one further by adding a `use`. In R-3's
  row, `crates/goad-shell/src/ingress/mod.rs:99` for `BindFault::LivenessUnknown`
  (it is at `:100`) and `:235` for `TryLockError::Error` (`:236`); this slice
  touched neither file, so both were already off by one. This is the rot class
  `CLAUDE.md` §Working here names as having rotted three times in one slice —
  *cite by symbol, never by line number* — and canon is where it is worst,
  because nothing re-reads canon on the commit that moves the line. Two rows for
  `audit.md`: the citations themselves (*document stale, code right*), and the
  question of whether the rule should bind canon explicitly, which is an
  amendment and needs endorsement. Both are canon, so untouched mid-slice.
- **`ConfigError::Read`'s own text still says *configuration could not be
  read*** and names no file. Deliberate (EX-4, OQ-3, D1): a path inside it
  would print twice in `goad-emit`. Nothing at stratum 3 renders that string
  any more, so this is latent rather than live — the question for a follow-up
  is whether a stratum 2 error that names no subject should carry that wording
  at all.
- **A `nix flake update` is owed, deliberately, and is not lost.** An update of
  every input was made in this repository at 10:58 on 2026-09-21 and left
  uncommitted; it was restored to the committed lock during PHASE-05 so the
  cutover measured the artefact the phase gate had verified. Whenever it is
  wanted, it is `nix flake update` followed by `just check` and `just package`,
  and then `nix flake update goad` in `~/flakes` to carry it across. Doing it
  as its own commit is the point — an input advance that arrives inside another
  change is indistinguishable from that change.
- **`Slint: Failed to create system tray icon: 0` stopped happening, and the
  cause is not established.** It appears on every pre-cutover start of the
  cargo binary and on none of the packaged binary's. The environment is ruled
  out — the wrapper and `~/.config/goad/env` carry identical library paths and
  the identical `fonts.conf` (§Findings). Two candidates remain, and this slice
  distinguished neither: the cargo binary was built 2026-09-16 and the code has
  moved since, so it may simply be older; or it is a start-order race, since
  both failing starts are at or near session start and the packaged binary's
  observed start was 13:35, hours into a session with the status-notifier host
  certainly up. The second would mean `After=graphical-session.target` is not
  sufficient for the tray, which is a module question and would be a real
  defect in what this slice shipped. Cheap to settle: start the freshly
  installed cargo binary against a throwaway config and read its journal, then
  restart the unit at login. Worth doing before the tray is trusted.
- **AC-9 and VA-1's comparison are still open.** P-7 (`just install` from the
  dev shell) and P-8 were offered and declined during the cutover;
  `~/.cargo/bin/goad` remains the 2026-09-16 binary, which exits 2 on
  `--version`. The cutover does not depend on them — the packaged path is
  live and observed — but the criterion that the cargo path still works, and
  the comparison that distinguishes the two version lines, have not been made.
  They can be run at any time; until then this phase is not `done`.

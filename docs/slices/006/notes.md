# Notes — Slice 006: packaging and the startup surface

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the crane packages | done | 2026-09-21 |
| PHASE-02 — the home-manager module | pending | |
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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-21 · PHASE-01 · `69e3b30`

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

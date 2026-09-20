# Notes — Slice 006: packaging and the startup surface

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the crane packages | in progress | 2026-09-21 |
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

- [ ] T-1 — EX-1, first and alone: join the `tokio` entry onto one line, add
      the comment saying why it must stay there (S-1: `builtins.fromTOML` is
      TOML 1.0; a newline inside an inline table is TOML 1.1, and `just check`
      cannot see a re-split). `just check` exits 0. **Commit before the flake
      is touched.**
- [ ] T-2 — VA-1: `nix eval .#packages.x86_64-linux.goad.drvPath` returns,
      where S-1 failed. Record the output.
- [ ] T-3 — EX-2..EX-5: the flake. crane via `crane.mkLib pkgs |>
      overrideToolchain rust`; one `buildDepsOnly` over `--workspace`; explicit
      `pname`/`version` on all three (virtual manifest); `version` read from the
      manifest (P-1); `doCheck = false` on all three; the filter by directory;
      `goad` wrapped with both flags, `goad-emit` unwrapped and without
      `guiLibs`; `GOAD_REVISION` on both; `packages.${system}` a **merge** with
      `jailPkgs`, `default` = `goad`.
- [ ] T-4 — EX-2, EX-3: `nix build .#goad` and `nix build .#goad-emit` succeed.
      `./result/bin/goad` is a wrapper over `.goad-wrapped`.
- [ ] T-5 — VA-2: `nix derivation show` names `GOAD_REVISION` on both, with a
      plausible short revision.
- [ ] T-6 — VA-3: build S-2's two negative controls, **observe each fail**,
      record the failure text here, then remove them. An uncompiled control
      greps the same as a passing one.
- [ ] T-7 — VA-4: `env -i ./result/bin/goad --help` exits 0 printing usage;
      `env -i ./result-emit/bin/goad-emit --version` exits 0 printing a version
      line (**do not pin its exact string** — F-2); `ldd
      ./result/bin/.goad-wrapped` reports zero `not found`.
- [ ] T-8 — VA-6: read the generated wrapper script and confirm **both**
      `--prefix LD_LIBRARY_PATH` and `--set-default FONTCONFIG_FILE`. This is
      the one check here that fails for the defect the slice replaces.
- [ ] T-9 — VA-7: the headless photograph, with the two variables stripped and
      every path absolute. **Open the PNG and look at it.** A blank window,
      boxes for glyphs, or an empty compositor all fail.
- [ ] T-10 — EX-6: `just package`, outside POL-001's chain, comment carrying
      the bare-git-form caveat.
- [ ] T-11 — EX-7: `just install`'s comment stops predicting its own
      retirement (P-2). Text only; the recipe is unchanged.
- [ ] T-12 — VA-5: `just -n check` still prints POL-001 §Compliance's six
      commands, in order, unchanged.
- [ ] T-13 — EX-8: `just check` exits 0. Refactor pass, phase sheet current,
      §Status set to `done`, §Harvest updated in place, commit.

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** <yyyy-mm-dd> · <phase or stage> · <commit>

### Produced
<!-- What now exists: modules, contracts, docs. -->

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

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

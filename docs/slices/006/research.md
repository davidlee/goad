# Research — Slice 006

**Producers:** design-stage agent, reading canon, this tree, and the four
out-of-tree prior-art sites `slice-006.md` §Before design starts names.
**As of:** 2026-09-20 · `c658b1a` (Thread 3 measured against this tree, then
reverted — the working tree is unchanged by the spike)

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- unmarked — researcher claim: cited, not checked.

Design and plan may only load-bear ✓ rows, or rows they verify at point of use.
Verify what you lean on, not everything.

## Citation forms

Canon claims cite the document id (`SPEC-003 §4`, `ADR-007`). Code claims cite
the file and the **symbol** — `CLAUDE.md` §Working here forbids line numbers,
which rot; this overrides the template's `path:line` form for in-tree code.
Out-of-tree prior art is cited by file and symbol too.

## Thread 1 — governing canon

### Binding

- ✓ **POL-001** — the gate is **six commands**, listed in its §Compliance block,
  which is canonical; the `justfile` mirrors it, policy first and recipe second.
  Adding `nix build` to the gate is therefore an amendment to this policy, not a
  `justfile` edit (OQ-2). §Scope clause 4 also obliges the policy to say **what
  each instrument holds and does not** — a new command would have to arrive with
  that statement, not just with a line in the block.
- ✓ **POL-001 §Verification** — the counting rule: four ADR-001 instruments,
  plus the domain-vocabulary scan, plus one unenforced residue. Nothing this
  slice writes may compress that count. None of the four reads `.nix`.
- ✓ **ADR-001 / `CLAUDE.md`** — stratum 1 is pure and never names stratum 2.
  Relevant to OQ-3 only in direction: `goad_shell::error::ConfigError` is
  **stratum 2** and `crates/goad/src/startup.rs` is stratum 3, so carrying the
  path at stratum 3 is with the grain and carrying it at stratum 2 is also legal
  — this is a cohesion question, not a direction one. See Cross-thread.
- ✓ **SPEC-003 R-3, R-4** — every ingress bind failure MUST be a startup failure
  **naming the path**, R-4 additionally naming *what was found*. Held today by
  `IngressError` (Thread 2). AC-6 must not weaken this: whatever happens to
  `StartupError::Config`, the `Ingress` arm keeps naming its path.
- ✓ **`CLAUDE.md`'s vocabulary invariant** — no domain vocabulary in host
  sources. The scan runs over crate sources (`crates/goad-boundary`), so a
  `nix/module.nix` and a systemd unit are **unscanned surface**: the invariant
  binds them, nothing checks them.

### Checked, not applicable

- **SPEC-001** (host/backend protocol), **SPEC-002** (scheduling) — nothing here
  is on the wire or on a schedule; no message, no timing.
- **ADR-004** (spacing from the previous scheduled firing), **ADR-005** (the
  envelope normalizes in stratum 2) — both are about protocol values.
- **ADR-002 / ADR-003** (the workspace of strata) — no workspace member is
  added, moved or split: `nix/` is not a crate.

### Amendment candidates

- **POL-001** — only if OQ-2 is answered *yes*. That answer raises the tier.
  No other canon looks likely to need changing: the startup-surface half moves a
  value inside a stratum-3 type and changes text SPEC-003 does not fix.

## Thread 2 — code map

### Hotspots

| file | why |
|---|---|
| `flake.nix` | no `crane` input, no `packages` beyond the three jails; `guiLibs` and `fontsConf` already exist as bindings and are exactly what a wrapper needs |
| `justfile` (`install` recipe) | writes `~/.config/goad/env` from the dev shell's two variables; its own comment names slice 006 as its retirement |
| `crates/goad/src/startup.rs` | `StartupError::Config` wraps `ConfigError` and drops the path; `arguments` reads any single non-`-h` token as a path |
| `crates/goad/src/main.rs` | `start` holds the path and discards it at the `map_err`; `run` is where a `--version` arm would land beside `Launch::Help` |
| `crates/goad/src/diagnostics.rs` | `USAGE` is the one usage const; `report_startup_line` is the pure half of the stderr outlet |
| `crates/goad/Cargo.toml` | no binary test target exists here (see Cited facts) |
| `crates/goad/build.rs` | the only build script; already reads one environment variable, with an argued lint expectation |
| `nix/module.nix` | does not exist (OQ-1) |

### Cited facts

**The startup surface**

- ✓ `StartupError::Config(goad_shell::error::ConfigError)` — a newtype arm, and
  its `Display` arm is `write!(f, "{error}")`, unprefixed. `ConfigError::Read`'s
  own text is *"configuration could not be read: {inner}"*, which names no path.
  (`crates/goad-shell/src/error.rs`, `impl fmt::Display for ConfigError`.)
- ✓ The path is **in hand at the call site**: `start(path: &Path)` calls
  `Config::load(path).map_err(StartupError::Config)`
  (`crates/goad/src/main.rs`, `start`).
- ✓ `StartupError` has **nine** variants, not eight — its doc comment says
  *"The eight variants"* and the enum lists `NoConfigPath`, `Usage`, `Config`,
  `Clock`, `Runtime`, `Platform`, `EventLoop`, `Enqueue`, `Ingress`. The comment
  went stale when `Ingress` was added in slice 004. A doc-comment defect this
  slice will touch anyway.
- ✓ Of the nine, exactly one holds a path and does not name it: `Config`.
  `Ingress` names it via `IngressError`; the other seven have no path.
  (`crates/goad/src/startup.rs`, `enum StartupError`.) This is the enumeration
  `design-log.md`'s second entry priced the fourth scope bullet on.
- ✓ **Two shapes for carrying a path already exist in this tree**:
  - stratum 2 — `IngressError { path: PathBuf, fault: BindFault }`, one struct,
    `Display` is `"{path}: {fault}"`
    (`crates/goad-shell/src/ingress/mod.rs`, `struct IngressError`). Its doc
    comment argues the shape: *"One struct, not an enum of paths: every message
    names the path once, and the fault says what was found."*
  - stratum 3 — `StartupFault::Unreadable { path, fault }` /
    `Unparseable { path, fault }` / `NoIngress { path }`, which wrap a stratum 2
    `ConfigError` **at the seam where the path is known**
    (`crates/goad-emit/src/main.rs`, `enum StartupFault` and `socket_path`).
- ✓ Emit's wrapping **splits `ConfigError::Read` from every other variant** so
  the two render differently — *"{path} could not be read: {fault}"* versus
  *"{path}: {fault}"* (`crates/goad-emit/src/render.rs`,
  `startup_error_line`). Any repair to `goad` that copies this shape inherits
  that split; one that puts the path inside `ConfigError` would make emit's
  split redundant and its two arms print the path twice.
- ✓ `arguments` treats **any** single token that is not `-h`/`--help` as a
  configuration path — `[only] => Ok(Launch::Config(PathBuf::from(only)))`
  (`crates/goad/src/startup.rs`, `arguments`). This is the whole of the
  `--version`-read-as-a-path defect; its doc table states the same rule.
- ✓ `Launch` has two variants, `Help` and `Config`; `run` matches them and
  `main` maps the result to `ExitCode::SUCCESS` or `2`
  (`crates/goad/src/main.rs`, `main`, `run`).

**`--version`, and what already exists**

- ✓ **`goad-emit` already implements `--version`** and prints
  `env!("CARGO_PKG_VERSION")` alone, on stdout, exit 0
  (`crates/goad-emit/src/main.rs`, `main`, the `Invocation::Version` arm).
- ✓ Its argument scan looks for `-h`/`--help`/`--version` in **flag position
  only**, so `--data --version` is a value and not a request for the version
  (`crates/goad-emit/src/args.rs`, `parse`, `in_flag_position`). `goad` takes at
  most one positional argument and has no value positions, so it needs none of
  this machinery — but the *shape* of the decision (a third `Launch` variant,
  not an early exit) is the precedent.
- ✓ `render::USAGE` in emit lists all three invocation forms including
  `goad-emit --version`; `goad`'s `USAGE` lists two and would gain a third
  (`crates/goad/src/diagnostics.rs`, `USAGE`).
- ✓ **Emit has a binary test target and `goad` does not.** `goad-emit` declares
  `[[test]] name = "binary"`, and `version_prints_the_package_version_on_stdout_and_exits_0`
  spawns `env!("CARGO_BIN_EXE_goad-emit")` and asserts stdout trims to
  `CARGO_PKG_VERSION` (`crates/goad-emit/tests/binary/exchange.rs`). `goad`'s
  twelve `[[test]]` targets are all library-linking (`renderer`, eleven
  `event_loop*`); a `--version` assertion **at the binary tier** for `goad`
  means a new test target. Feasible without a display: `--version` returns
  before any Slint call.

**Where a revision could come from**

- ✓ `crates/goad/build.rs` already reads `SLINT_STYLE` at build time under an
  argued `#[expect(clippy::disallowed_methods, reason = …)]`, because
  `clippy.toml` disallows `std::env::var` with *"Use typed configuration loading
  instead"*. A build script that also emitted a revision would extend an
  expectation that is already written and argued (`crates/goad/build.rs`,
  `main`; `clippy.toml`, `disallowed-methods`).
- ✓ The `env!`/`option_env!` **macros** are not `std::env::var` and are not on
  the disallowed list; the tree already uses `env!` in eleven places, in
  production code (`crates/goad-emit/src/main.rs`) as well as tests.
- ✓ The version itself is `[workspace.package] version = "0.1.0"`, inherited by
  every member as `version.workspace = true` (`Cargo.toml`;
  `crates/goad/Cargo.toml`).
- ✓ `git remote -v` — `origin` is `git@github.com:davidlee/goad.git`, so a
  git-input flake reference has a real upstream to name.

**The nix surface as it stands**

- ✓ `flake.nix` declares inputs `nixpkgs`, `rust-overlay`, `pub`,
  `llm-agents`. **No `crane`.** Its outputs are `packages.${system}` (three
  jails only) and one devShell. No `homeManagerModules`, no package for either
  binary.
- ✓ `guiLibs` — `wayland`, `libxkbcommon`, `libGL`, `fontconfig`,
  `stdenv.cc.cc.lib`, with a comment stating they are `dlopen`'d and must be on
  `LD_LIBRARY_PATH` "inside and outside the jail or the window never opens".
- ✓ `fontsConf = pkgs.makeFontsConf { fontDirectories = [pkgs.dejavu_fonts]; }`,
  with the measured jail note: 58 of 156 `-p goad --test renderer` cases panic
  in fontique with `NoMatch` when `FONTCONFIG_FILE` is absent. This is OQ-5's
  evidence: the renderer tier **does** need a font configuration to run.
- ✓ The devShell sets exactly the two variables `just install` captures:
  `LD_LIBRARY_PATH = lib.makeLibraryPath guiLibs` and
  `FONTCONFIG_FILE = fontsConf`.
- ✓ `just install` runs `cargo install --path crates/goad --locked` and the same
  for `crates/goad-emit`, then writes both variables into
  `${XDG_CONFIG_HOME:-$HOME/.config}/goad/env`, guarding each with `${VAR:?…}`
  so running outside the dev shell fails loudly (`justfile`, `install`).
- ✓ The fonts are tracked, but **by a negation rule and not by a force-add** —
  `assets/.gitignore` is `*.ttf` followed by `!Inter.ttf` and `!Geist.ttf`.
  `slice-006.md`'s third *Before design starts* bullet says "force-added"; the
  conclusion it draws is unchanged (a third face would be invisible unless
  negated too) but the mechanism is a rule in the file, which a reader can see.
  `git ls-files assets` is exactly `.gitignore`, `Geist.ttf`, `Inter.ttf`,
  `OFL.txt`; `crates/goad/ui/app.slint` imports the two faces and nothing else.
  Untracked and unused by the build: `assets/Geist-Italic.ttf`,
  `assets/Inter-Italic.ttf`, and the whole of `assets/static/`.

**Prior art, out of tree**

- ✓ `~/dev/doctrine/flake.nix` — crane over a workspace: one `rust` toolchain
  binding shared by devshell and `craneLib = (crane.mkLib pkgs).overrideToolchain rust`,
  with a comment recording that crane's default nixpkgs-stable rustc **flips
  lint verdicts** against the beta toolchain the gate uses. `version` is read
  out of `Cargo.toml` with `builtins.fromTOML`. `buildDepsOnly` runs on the lean
  source and `buildPackage` on the grafted one. `doCheck = false`, because that
  project's tests need Postgres — *not* a general rule.
- ✓ `~/dev/doctrine/flake.nix`, `srcWithDist` — the **graft** pattern:
  `cleanCargoSource` output copied into a `runCommandLocal`, made writable, and
  the stripped asset roots copied back. The comment states the failure it
  repairs: the binary "ships asset-incomplete" because the embed silently
  drops. In goad the same loss is **loud** — `build.rs` fails — which is why
  `slice-006.md` expects a widened filter to be cheaper than a graft here.
- ✓ `~/dev/satan-attrd/nix/module.nix` — a module's option surface:
  `enable` (`mkEnableOption`), `package` (required, no default), and typed
  options for each environment value, plus `extraEnvironment` as an escape
  hatch. `config = lib.mkIf cfg.enable { home.packages = [cfg.package];
  systemd.user.services.<name> = { Unit; Service; Install; }; }`.
- ✓ `~/flakes/modules/home/linux/satan-attrd.nix` — the **thin consumer**: four
  lines, `imports = [inputs.satan-attrd.homeManagerModules.default]` and
  `services.satan-attrd = { enable = true; package = …; }`.
- ✓ `~/flakes/modules/home/linux/behaviour.nix` — the **inline** alternative:
  `home.packages = [panopticon]` plus `systemd.user.services.panopticon-sway`
  written out in the consumer, with `Unit`/`Service`/`Install` attrsets. No
  module in the producing repository at all.
- ✓ **`~/flakes` names goad nowhere** — `grep -rn goad ~/flakes --include=*.nix`
  is empty. Whatever this slice exports, the consumer wiring is new.
- ✓ `~/satan/goad/goad.service` — the unit as it runs today:
  `After`/`PartOf=graphical-session.target`, `ExecStart=%h/.cargo/bin/goad`,
  `EnvironmentFile=%h/.config/goad/env`, `Restart=on-failure`,
  `RestartPreventExitStatus=2`, `RestartSec=2`,
  `Install.WantedBy=graphical-session.target`. Its comment argues the exit-2
  rule in exactly AC-7's terms: exit 2 is any `StartupError`, none of which
  succeeds on a retry.

### Precedents

- **Error text that names a path**: `"{path}: {fault}"` (ingress) and
  `"{path} could not be read: {fault}"` (emit's `Read` split). A third spelling
  would be a third convention.
- **A user-visible string lives in one module.** `crates/goad`'s is
  `diagnostics.rs` (*"Everything a person reads, in one module"*); emit's is
  `render.rs` (*"Every line the binary can write, as a `String` with no sink"*).
  A version line belongs in `diagnostics.rs`, rendered pure and written by
  `main`.
- **A pure `*_line` function plus a thin writer** is the outlet shape on both
  binaries (`report_startup_line`/`report_startup`; `startup_error_line`).
- **Binary-tier assertions spawn `CARGO_BIN_EXE_*`** rather than shelling out to
  `cargo run` (`crates/goad-emit/tests/binary/exchange.rs`, `emit`).
- **An enum arm, never an early `exit`**: `std::process::exit` is disallowed, and
  `Launch::Help` exists so that `main` keeps its single exit-code decision.

## Thread 3 — the spike (measured, 2026-09-20)

A working `crane` package was built in this tree, exercised, and reverted; the
flake, its lock and `Cargo.toml` are back at `c658b1a`. **The apparatus is kept
at `docs/slices/006/spike/`** — its `README.md` says what each throwaway
package was for and lists what is deliberately wrong with it for the
deliverable, so the claims below can be checked against something runnable
rather than against this prose. Every row is ✓ by construction: it is an
observation of a build that ran, not a claim about someone else's code.

### What was built

`crane` over the existing workspace, one toolchain binding shared with the
devshell, `buildDepsOnly` for the dependency layer, a widened source filter,
and `wrapProgram` over the two runtime variables. Arms: `doCheck` off and on,
the check phase with and without an environment, and two negative controls on
the source filter.

### S-1 — `builtins.fromTOML` cannot read this workspace manifest

`nix build` fails **at evaluation**, before any compilation:

```
error: while parsing TOML: toml::parse_inline_table: missing closing bracket `}`
 37 | tokio      = { version = "1",
```

`[workspace.dependencies]`'s `tokio` entry spans two lines. A newline inside an
inline table is TOML 1.1; `fromTOML` (Lix 2.95.2) implements TOML 1.0, which
cargo does not restrict itself to. **It is not only doctrine's
read-the-version-from-Cargo.toml idiom that trips on this** — crane itself
parses every manifest: `buildDepsOnly` → `crateNameFromCargoToml` for a missing
`pname`/`version`, and unconditionally in `cleanCargoToml`, which builds the
dummy source. Supplying `pname` and `version` explicitly is **not** a
workaround; the dummy-source derivation still fails.

The whole repair is **one line** — join the `tokio` entry — and it is the only
such entry in the workspace: a scan of all six manifests for an inline table
that opens and does not close on its line finds exactly `Cargo.toml`'s line 37.
With it joined, evaluation succeeds. This is a **precondition of packaging with
crane**, not a preference, and it belongs in the plan's first phase.

### S-2 — both halves of the source widening are necessary, and both fail loudly

Two negative controls, each a real derivation that was built and observed to
fail (`docs/memory/negative-control-must-compile.md`):

| filter | outcome | time |
|---|---|---|
| `craneLib.cleanCargoSource` (stock) | `build.rs` fails: *Could not load `…/crates/goad/ui/app.slint`: No such file or directory* | 7s |
| stock **+ `.slint`**, `.ttf` still stripped | the Slint compiler fails: *File "../../../assets/Inter.ttf" not found*, and the same for `Geist.ttf` | 6s |
| stock **+ `.slint` + `.ttf`** | builds | — |

So `slice-006.md`'s *Before design starts* prediction holds and is now measured:
the loss is loud here, it costs seconds to discover, and a widened filter is
cheaper than doctrine's graft. Both extensions are load-bearing; neither alone
is enough.

Residue the spike did **not** settle: a `.ttf` suffix filter also admits
`assets/Geist-Italic.ttf`, `assets/Inter-Italic.ttf` and all of
`assets/static/`, none of which the build uses. Untracked files are outside a
git-input source anyway, but a design that names the two faces exactly is
narrower than one that names an extension.

### S-3 — `doCheck = true` needs three environmental provisions, and still fails

Run in three arms against the nix sandbox:

| arm | provisions | result |
|---|---|---|
| B1 | none | **lib tier fails**: 49 passed, 6 failed. Every failure is `instant.rs` — *"failed to find time zone `America/New_York` since there is no time zone database configured"*, and one whose own assertion message is *"the manifest's `tz-system` feature is what stops this being Etc/Unknown"*. The sandbox has no tzdb. |
| B2 | `TZDIR=${pkgs.tzdata}/share/zoneinfo` + `FONTCONFIG_FILE=${fontsConf}` | lib tier **passes** (55/55); eleven `event_loop*` targets pass; **`event_loop_schedule` fails** — `a_scheduled_evaluation_fires_under_the_production_topology` panics with *"the scheduled evaluation never landed within 5s"*. Also, non-fatally: *Fontconfig error: No writable cache directories /homeless-shelter/.cache/fontconfig*, and *Slint: Failed to create system tray icon: 0* (no session bus in the sandbox). |

Two further facts about the cost:

- **`doCheck` doubles the dependency build.** `cargoArtifacts` was built with
  `--workspace` and `doCheck = false`, so it holds no dev-dependencies; both
  check arms recompiled the Slint stack (`i-slint-compiler`, `slint`,
  `i-slint-backend-winit`, …) from source before running a case.
- **`cargoExtraArgs` is inherited by the check phase.** crane's
  `checkPhaseCargoCommand` is `${cargoTestCommand} ${cargoExtraArgs}
  ${cargoTestExtraArgs}` (`crane/lib/buildPackage.nix`). A package that selects
  its binary with `cargoExtraArgs = "-p goad --bin goad"` and sets
  `doCheck = true` runs `cargo test -p goad --bin goad` — the binary target's own
  unit tests, of which `main.rs` has none. **It reports green having run
  nothing.** Any `doCheck = true` must therefore carry a separate, argued test
  selector, or it is a false green.

### S-4 — the wrapper works, and the binary runs from an empty environment

`wrapProgram` produced `bin/goad` over `bin/.goad-wrapped`, prefixing
`LD_LIBRARY_PATH` with the five `guiLibs` store paths and setting
`FONTCONFIG_FILE` with `--set-default` (so a caller's own value still wins).
Observed under `env -i`, with neither variable set in the caller's environment:

| invocation | result |
|---|---|
| `env -i …/bin/goad --help` | the usage block on stdout, exit 0 |
| `env -i …/bin/goad --version` | `goad: configuration could not be read: No such file or directory (os error 2)`, **exit 2** |
| `env -i …/bin/goad /nonexistent/wat.toml` | the **identical** line, exit 2 |
| `ldd …/bin/.goad-wrapped` | zero `not found` |
| `env -i …/bin/goad-emit --version` | `0.1.0`, exit 0 — emit needs no wrapper |

The third row is sharper than `slice-006.md`'s statement of the defect: it is
not only that the path is unnamed, it is that **a mistyped flag and a missing
file are indistinguishable**. AC-4 and AC-6 are the same repair seen twice.

Not settled by the spike: whether a window opens **with text in it** (AC-1's
second half). That needs a compositor, and is AC-8's person-runs-it evidence.
What the spike does establish is the mechanism — the variables reach the process
— and B2 shows `FONTCONFIG_FILE` is what lets a component be constructed at all
in a bare environment.

### S-5 — what the build costs

| measurement | time |
|---|---|
| cold: vendor + dependency layer + build + wrap | 181s |
| final layer only, warm `cargoArtifacts` | 22s |
| `goad-emit`, warm | 10s |
| no-op rebuild, nothing changed | 1s |
| a negative control that fails in `build.rs` | 6–7s |

The 181s is a warm **nix store** and a cold crane layer; a genuinely cold
machine also pays the crates.io vendoring (~4,100 derivations) and the
toolchain. This is the number OQ-2 must be argued against.

## Cross-thread findings

1. **OQ-3 has a precedent that the slice doc under-states.** `slice-006.md`
   frames it as a free choice between two in-tree shapes. It is not quite free:
   `goad-emit` already wraps `ConfigError` with a path **at stratum 3**, and it
   does so *because* it holds the path at the seam — the same position
   `main::start` is in. Putting the path inside `ConfigError` instead would
   change a type emit consumes and leave emit's `Unreadable`/`Unparseable`
   split printing the path twice unless emit is also changed, which the slice's
   non-goals forbid. The stratum 3 answer is with the grain of both binaries;
   the stratum 2 answer is a wider change that the non-goals then block from
   being finished. Design must still argue it, but the asymmetry is real and is
   evidence, not preference.

2. **AC-5 creates an asymmetry with `goad-emit` that nothing in the slice doc
   mentions.** Emit's `--version` prints the bare package version; if `goad`'s
   prints version **and revision**, the two binaries of one workspace answer the
   same flag in two different shapes, and emit's binary test pins its half
   (`version_prints_the_package_version_on_stdout_and_exits_0` asserts
   equality with `CARGO_PKG_VERSION`, not a prefix). The slice's non-goals say
   emit's *startup diagnostics* are not reopened; `--version` is arguably not a
   startup diagnostic. **This is an open question the design must settle** — see
   OQ-7 in `design.md` §6.

3. **OQ-5 is answered, and the answer is not the one the slice doc framed.**
   The choice was posed as *pass `fontsConf` into the check phase* or
   *`doCheck = false`*. Thread 3 S-3 shows fonts are the **second** of at least
   three obstacles: the tzdb is the first and fails the lib tier before the
   renderer tier is reached, and a timing-sensitive `event_loop_schedule` case
   fails in the sandbox even with both provisions. Add the doubled dependency
   build and crane's inherited `cargoExtraArgs` — which makes the obvious
   spelling of `doCheck = true` a **false green** — and the package build is
   reconstructing the devshell badly to re-run what the gate already runs well.
   doctrine's `doCheck = false` turns out to be right here for a different
   reason than it is right there.

4. **The module is unscanned surface for the vocabulary invariant.** Whatever
   OQ-1 decides, neither the boundary scan nor any of ADR-001's four instruments
   reads `.nix` — so "the module names no domain" is a review obligation, in the
   same category POL-001 §Verification calls *residue*. Worth stating in the
   design rather than discovering at audit.

5. **OQ-2 and the tier interact with the `justfile`'s mirror rule.** POL-001
   says the policy changes first and the recipe second. If `nix build` joins the
   gate, the slice writes canon (tier 2, full lifecycle, a `canon-delta.md`) —
   and the gate then requires a working nix build on every phase, including the
   phases that are changing the flake. Sequencing, not just tier.

## Design-input deltas

- **A precondition nothing else in the slice names (S-1)**: `Cargo.toml`'s
  `tokio` entry must be joined onto one line before any crane package can be
  evaluated, and this is the first thing the plan's first phase does. It is a
  one-line change to a manifest and it is not optional.
- **OQ-5**: answered — `doCheck = false`, argued from S-3's three measured
  layers rather than from doctrine's precedent. The design must also say what
  *does* hold the tests, which is the gate, and note that a `doCheck = true`
  carrying a `--bin` selector reports green having run nothing.
- **The source filter (S-2)**: widen for `.slint` **and** `.ttf`; both halves
  proven necessary by controls that were built and observed to fail. Open
  design choice left: filter by extension, or name the two faces exactly.
- **AC-4 and AC-6 are one defect seen twice (S-4)**: `goad --version` and
  `goad /nonexistent/wat.toml` print the *same* line today. A design that
  repairs only one of them leaves the other indistinguishable from it.
- **OQ-2 now has a number to be argued against (S-5)**: 22s for the final layer
  warm, 181s cold-store, 1s no-op.
- **OQ-3**: the enumeration behind it is confirmed — one variant, `Config` — and
  the emit precedent makes the stratum 3 shape the one to beat. Design should
  present it as recommended-with-reasons rather than as an even choice.
- **OQ-4**: `build.rs` is a live option with an existing, argued lint
  expectation; `env!`/`option_env!` are not disallowed. The real question is
  what an **unknown** revision prints, which AC-5 constrains.
- **A new open question (OQ-7)**: does `goad-emit --version` change shape too,
  or do the two binaries diverge deliberately? Cross-thread 2.
- **A defect to fix in passing**: `StartupError`'s doc comment says *"The eight
  variants"* and there are nine.
- Nothing here contradicts `slice-006.md`'s *Before design starts* facts; the
  asset and flake-reference bullets are confirmed rather than revised.

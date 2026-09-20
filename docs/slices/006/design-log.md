# Design log — Slice 006

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

### 2026-09-20 — open the slice now, with 009 still in audit

- **Asked:** whether to open 006 while 009 is mid-flight, park it until 009
  closes, or treat the packaging work as out-of-band infrastructure that skips
  the `docs/AGENTS.md` lifecycle. The third was offered because it is the user's
  to declare and not the agent's to assume.
- **Recommended:** none of the three outright; the blocker named was that 009
  held a dirty tree in the files the startup-surface half would touch.
- **Decided:** *"009 is in the final throes of audit. let's open the slice."*
- **Consequence:** the lifecycle applies in full. The slice folder is created
  from the template. Execution was to wait on 009's close, the two overlapping
  in `crates/goad/src/main.rs` (`docs/memory/one-writer-per-worktree.md`); 009
  closed at `af76b4c` during this scoping conversation, so nothing waits.

### 2026-09-20 — scope is all four of the roadmap's bullets

- **Asked:** whether the slice takes two of `docs/roadmap.md` §006's bullets —
  the crane package and the home-manager module, which were what the user
  proposed and which touch no Rust — or all four, adding `--version` with the
  git sha and startup errors that name the path they tried.
- **Recommended:** initially **1, 2 and 4**, deferring the startup-error work on
  the ground that it touched files 009 held dirty and would cost design surface
  under the tier 1 cap. The user asked whether the errors needing a path were
  already enumerated. They were not written down anywhere; enumerating them
  showed the gap is **one variant of one enum** — `StartupError::Config`, since
  `Ingress` already names its path under `SPEC-003/R-3` and `R-4` and the other
  seven variants have no path to name — and that `goad-emit` already carries the
  repair's shape. The recommendation to defer was withdrawn as priced on cost
  that had not been measured (`docs/memory/price-the-rejected-option-against-code.md`).
- **Decided:** *"yes"* — all four, tier 1, with the module-home and
  gate-membership questions carried as open questions.
- **Consequence:** `slice-006.md` as written: scope names both the nix surfaces
  and `crates/goad/src/startup.rs`; AC-4, AC-5 and AC-6 are the startup-surface
  half; OQ-1 and OQ-2 carry what was deliberately left open, and OQ-2's answer
  is one of the two things that would raise the tier.

### 2026-09-20 — the empirical questions are spiked before the design is drafted

- **Asked:** whether to settle OQ-5 (`doCheck`), the source-filter shape and
  half of OQ-6 (whether `wrapProgram` actually retires `~/.config/goad/env`) by
  building, or to draft the design from argument and leave all three to a phase.
- **Recommended:** spike first — in this tree, uncommitted, with the finding
  lifted into `research.md` and the flake reverted before the design is drafted,
  so the design cites measurements and the implementation still happens under an
  accepted plan. The cost named on the other side: design writing `flake.nix`
  before there is a plan is the thing `docs/AGENTS.md` exists to prevent.
- **Decided:** *"yeah, spike first"*.
- **Consequence:** research gains a measured section rather than a predicted
  one; OQ-2's answer can be priced against a real build time.

### 2026-09-20 — OQ-2: `nix build` does not join the phase gate

- **Asked:** whether `nix build` joins POL-001's command block (amending canon,
  raising this slice to tier 2), stays out of it entirely, or lands as a
  `just package` recipe outside the gate that audit is obliged to run before a
  slice closes.
- **Recommended:** joining the gate. The spike had measured the cost at 22s
  warm and 1s for a no-op, against a gate that already runs two test tiers and
  a deno typecheck; and the failures the spike produced — an evaluation error
  from `fromTOML`, two source-filter regressions failing in 6–7s — are exactly
  the class a fast command catches.
- **Decided:** *"No, for now."*
- **Consequence:** POL-001 is untouched and **the slice stays tier 1** —
  `design.md` capped at 300 lines, design and plan reviewed in one ledger of at
  most two rounds. `research.md` Thread 1's amendment-candidate list is now
  empty. The residue the answer accepts, stated so it is not rediscovered: a
  `flake.nix` that stops building is green in this repository and breaks in
  `~/flakes`, which is where it is noticed.
- **Not answered, and carried:** whether the third option — a `just package`
  recipe outside the gate, run at audit — is wanted. "No, for now" settled gate
  membership, not that. It belongs in `design.md` §6 as an open question and
  must be put to the user before design is accepted.

### 2026-09-20 — OQ-3: the path is named at stratum 3

- **Asked:** whether `goad` names the configuration path by growing a
  path-carrying arm on its own stratum 3 `StartupError` (wrapping stratum 2's
  `ConfigError` at the seam, as `goad-emit`'s `StartupFault::Unreadable` /
  `Unparseable` already do), or by putting the path inside stratum 2's
  `ConfigError` so every consumer gets it.
- **Recommended:** stratum 3 — not for symmetry, but because the stratum 2
  answer cannot be finished inside this slice. `ConfigError` is also consumed by
  `goad-emit`; a path inside it makes emit's two wrapping arms print the path
  twice, and repairing that is emit's startup diagnostics, which
  `slice-006.md` §Non-goals excludes. It also spreads the surface across two
  crates and two binaries, the shape §What would raise the tier names as the
  push past the 300-line cap. Argument recorded on the other side: stratum 2
  fixes it once for all consumers — answered by the fact that `Config::load`
  takes the path as an argument, so every caller already holds it.
- **Decided:** *"stratum 3"*.
- **Consequence:** `goad_shell::error::ConfigError` is untouched; `goad-emit` is
  untouched. The change is confined to `crates/goad/src/startup.rs` and the
  `map_err` in `crates/goad/src/main.rs`'s `start`. `research.md` Cross-thread 1
  is the evidence. The tier stays 1: the route to tier 2 that
  `slice-006.md` §What would raise the tier held open is now closed, and both
  routes are settled.

### 2026-09-20 — the arm's shape: emit's split, transcribed

- **Asked:** given stratum 3, how the arm is shaped and what it prints, since
  `ConfigError::Read`'s own text is *"configuration could not be read:
  {inner}"* and a path in front of it unchanged yields
  `goad: <path>: configuration could not be read: No such file or directory`
  — two colons, and "configuration" saying what the path already said. Three
  options: **A** two arms copying `goad-emit` (`ConfigUnreadable { path, fault:
  io::Error }` / `ConfigUnparseable { path, fault: ConfigError }`, split at the
  seam by the same two-arm match `goad-emit`'s `socket_path` uses); **B** one
  arm, one spelling, shipping the doubled prefix on the commonest failure;
  **C** one arm with the split done inside `Display`.
- **Recommended:** A. Two lines at the seam, a transcription of a precedent a
  reviewer can diff against, and `ConfigError`'s variants stay out of `goad`'s
  `Display`. C saves one enum arm and pays by hiding the split where nobody
  comparing the two binaries would look. Cost of A named: a tenth variant on an
  enum whose doc comment already miscounts at eight.
- **Decided:** *"A"*.
- **Consequence:** `StartupError::Config(ConfigError)` is replaced by two
  arms; `crates/goad/src/main.rs`'s `start` gains the two-arm match on
  `Config::load`; the two `Display` arms print `"{path} could not be read:
  {fault}"` and `"{path}: {fault}"`, which are the two spellings already in the
  tree and not a third convention (`research.md` Thread 2 §Precedents). The
  stale *"The eight variants"* doc comment is corrected to ten in the same
  edit.

### 2026-09-20 — OQ-4: nix stamps the revision, `cargo install` does not

- **Asked:** where the revision comes from, given AC-5's trap — **if both build
  paths stamp a real git sha they are not distinguishable**, since the same
  clean commit yields the same line. Mechanism and discriminator are one
  decision. **A**: the flake passes `self.shortRev or self.dirtyShortRev` into
  the derivation's environment and `crates/goad` reads it with
  `option_env!("GOAD_REVISION")`; absence is the discriminator. **B**:
  `build.rs` shells out to `git rev-parse --short HEAD` so the cargo path
  stamps too, with a `(nix …)` / `(git …)` label discriminating.
- **Recommended:** A. It states only what the binary knows — a revision was
  stamped in, or it was not — where B's label is an inference the binary cannot
  justify, nothing stopping a cargo build from having the variable set by hand.
  B also costs a subprocess in the build, `cargo:rerun-if-changed` on
  `.git/HEAD` *and* the ref file (the classic way to ship a stale sha), a
  git-absent fallback, and a sha that is a lie when `cargo install --path` runs
  from a dirty tree. Cost of A stated: a `cargo install`ed `goad` can never say
  what it was built from.
- **Decided:** *"A"* — with the question raised of whether this needs
  `nix --impure`.
- **Measured, answering that:** it does not. `nix flake metadata --json .` on
  Lix 2.95.2 reports `locked.dirtyShortRev = "08528b5-dirty"` with no impure
  evaluation; `rev`/`shortRev`/`dirtyRev`/`dirtyShortRev` are attributes of the
  locked input, mirrored onto `self`.
- **Consequence, and two facts the design carries rather than re-asks:**
  - no `build.rs` change, no subprocess, no clippy expectation; `option_env!`
    is a macro and is not `std::env::var`, so nothing on `clippy.toml`'s
    `disallowed-methods` is touched.
  - **any uncommitted change yields the `-dirty` suffix, a documentation edit
    included.** It is the normal development case, not a warning sign.
  - a tarball fetch (`github:…/archive/….tar.gz`) has *neither* attribute, so
    the nix side needs its own fallback. The design takes
    `GOAD_REVISION = self.shortRev or self.dirtyShortRev or ""` with the Rust
    side treating **set-but-empty as unset** — the rule
    `crates/goad/build.rs`'s module doc already states for `SLINT_STYLE`, so it
    is a precedent rather than a new convention, and "unknown" is decided in
    exactly one place.

### 2026-09-20 — OQ-7: emit converges, and the unknown revision is silent

- **Asked:** whether `goad-emit --version` takes the same shape as `goad`'s or
  the two binaries diverge deliberately. Raised by `research.md` Cross-thread 2,
  not by the slice doc. Emit prints `env!("CARGO_PKG_VERSION")` bare today and
  its binary test asserts stdout **equals** it.
- **Recommended:** converge, kept minimal — emit gains the revision in the same
  shape and nothing else about emit changes. The argument was not symmetry: the
  slice's third defect ("two install paths can both put a `goad` on `$PATH` and
  nothing can say which ran") is equally true of emit, which `just install`
  installs and AC-2 and AC-3 package, so a diverged design leaves the pair
  half-diagnosable. The non-goal excludes emit's *startup diagnostics*, which
  `--version` is not.
- **Decided:** converge, **but the cargo-built binary prints the bare version
  with no parenthetical at all** — *"it's the primary distributable version,
  and it is likely to confuse"*. The proposed `0.1.0 (revision unknown)` is
  dropped.
- **Consequence — what each binary prints:**

  | build | output |
  |---|---|
  | nix, clean tree | `0.1.0 (08528b5)` |
  | nix, dirty tree | `0.1.0 (08528b5-dirty)` |
  | `cargo install` | `0.1.0` |

  Bare, with no program-name prefix, following emit's existing precedent —
  the `"goad: "` prefix on `report_startup_line` exists because stderr must say
  who spoke, and a direct answer on stdout need not. Both binaries render
  through a pure `version_line(revision: Option<&str>) -> String` in their own
  string module (`crates/goad/src/diagnostics.rs`,
  `crates/goad-emit/src/render.rs`), matching the `*_line` outlet shape already
  on both sides.
- **Two things this decision moves:**
  - **AC-4 becomes false as written** for the `cargo install` path, since it
    says `--version` prints *"a version and the git revision it was built
    from"*. `slice-006.md` takes a consistency edit when the design is drafted
    (`docs/AGENTS.md` §Design). AC-5 is unaffected: `0.1.0` and
    `0.1.0 (08528b5)` still tell the two apart.
  - **emit's binary test passes unchanged** —
    `version_prints_the_package_version_on_stdout_and_exits_0` asserts equality
    with `CARGO_PKG_VERSION`, and `cargo test` never sets `GOAD_REVISION`. The
    stamped branch is therefore reachable **only** through `version_line`'s
    unit tests, which is what they are for
    (`docs/memory/tests-asserting-proxies.md`).

### 2026-09-20 — OQ-5: `doCheck = false`

- **Asked:** whether the crane package runs the workspace's tests. Framed in
  `slice-006.md` as *pass `fontsConf` into the check phase* or
  *`doCheck = false`*; `research.md` Thread 3 S-3 had already shown that framing
  was too narrow.
- **Recommended:** `doCheck = false`, argued from three measured layers rather
  than from doctrine's precedent (which is false for a different reason — its
  tests need Postgres):
  1. **no tzdb in the sandbox** — the lib tier fails first, 6 of 55 cases, all
     `instant.rs`. Fonts never come into it.
  2. **with `TZDIR` + `FONTCONFIG_FILE`** the lib tier passes (55/55) and eleven
     `event_loop*` targets pass, but
     `a_scheduled_evaluation_fires_under_the_production_topology` fails —
     *"the scheduled evaluation never landed within 5s"* — in a sandbox with no
     session bus and no writable font cache.
  3. **crane's `checkPhaseCargoCommand` inherits `cargoExtraArgs`**, so a
     package selecting `-p goad --bin goad` with `doCheck = true` runs
     `cargo test -p goad --bin goad`, and `main.rs` has no unit tests: **it
     reports green having run nothing.**
  Plus: `cargoArtifacts` carries no dev-dependencies, so both check arms
  recompiled the Slint stack from source before running a case.
- **Decided:** *"doCheck = false it is"*.
- **Consequence:** the design states what *does* hold the tests — `just check`,
  POL-001's six commands, run in the devshell where the tzdb, the font
  configuration and a real session are present — and restates OQ-2's accepted
  residue: a `flake.nix` that stops building is green here and breaks in
  `~/flakes`.
- **Declined in passing, recorded so it is not rediscovered at review:** a
  separate `checks.${system}` output so `nix flake check` runs the tests
  without `nix build .#goad` paying for them. It fails on the same three layers
  — the sandbox is the obstacle, not the command it is attached to — and OQ-2
  has already declined to put a nix command on the gate, so nothing would run
  it.

### 2026-09-20 — the source filter is spelled by directory, not by extension

- **Asked:** how to spell the widened crane source filter. `research.md`
  Thread 3 S-2 had measured both halves as necessary — stock
  `cleanCargoSource` fails in `build.rs` (7s, `ui/app.slint` missing); stock
  plus `.slint` fails in the Slint compiler (6s, the two font imports
  unresolved) — and left the spelling open. **A**: by extension — cargo
  sources, plus `.slint`, plus `.ttf`. **B**: by what the build reads — cargo
  sources, plus `.slint`, plus everything under `assets/`. A third, naming the
  two faces exactly, was not recommended: it adds a second place to edit when a
  face is added, on top of the `assets/.gitignore` negation that must already
  be remembered.
- **Recommended:** B. Same size; reads as a statement of fact (*`build.rs`
  compiles `ui/app.slint`, which imports from `assets/`*) where A is a list of
  file types that happens to be complete today. A future asset that is not a
  font needs a flake edit under A and none under B. Both admit exactly the same
  files today.
- **Decided:** *"B"*.
- **Consequence, and the residue it retires:** S-2's stated worry — a `.ttf`
  suffix also admitting `assets/Geist-Italic.ttf`, `assets/Inter-Italic.ttf`
  and `assets/static/` — is **nil under either spelling**, because a git-input
  source contains only tracked files and all three are untracked. It would
  appear only if they were later tracked, and then it costs store size, not
  correctness. The design says so rather than leaving it as an open residue.

### 2026-09-20 — OQ-1: the module lives in this repository

- **Asked:** whether the home-manager module is exported from here as
  `nix/module.nix` (`~/flakes` then carrying the four-line `satan-attrd.nix`
  consumer) or written longhand in `~/flakes` (the `behaviour.nix` pattern,
  with no module in the producing repository). `grep -rn goad ~/flakes
  --include=*.nix` is empty, so the consumer wiring is new either way.
- **Recommended:** in-repo. The deciding fact is `RestartPreventExitStatus=2`:
  that number is goad's exit-code contract — `main` maps every `StartupError`
  to exit 2, and this slice adds a tenth variant to that enum. A unit encoding
  the contract belongs in the repository that owns it, so one commit changes
  both and the reason is visible in one diff. Inline in `~/flakes` repairs
  *"hand-written"* and *"nothing rebuilds it from a clean clone"* but not the
  third thing `slice-006.md` §Purpose names: the unit stays outside the
  repository and the coupling stays invisible.
- **Decided:** *"accepted."* — the module and the option surface below.
- **Consequence — the option surface**, following
  `~/dev/satan-attrd/nix/module.nix`:

  | option | |
  |---|---|
  | `enable` | `mkEnableOption` |
  | `package` | required, no default — the consumer passes it |
  | `extraConfig` | an attrset merged over the generated `Service` block |

  `config = mkIf cfg.enable` produces `home.packages = [cfg.package]` plus
  `systemd.user.services.goad`, with `ExecStart = "${cfg.package}/bin/goad"` —
  a store path, not `%h/.cargo/bin/goad` — and **no `EnvironmentFile` at all**,
  because `wrapProgram` carries both variables (AC-3).
  `After`/`PartOf`/`WantedBy=graphical-session.target`, `Restart=on-failure`,
  `RestartPreventExitStatus=2` and `RestartSec=2` come across from
  `~/satan/goad/goad.service` as defaults, which is AC-7.
- **Deliberately excluded, and accepted as excluded:** a `configFile` option
  (the default XDG path resolves inside a user unit, and a second way to name
  that file is a standing non-goal) and typed environment options (there is no
  environment left to type).
- **Carried into the design as an obligation, not a check:** `research.md`
  Cross-thread 4 — none of ADR-001's four instruments nor the
  domain-vocabulary scan reads `.nix`, so *the module names no domain* is a
  review obligation in the category POL-001 §Verification calls **residue**.

### 2026-09-20 — OQ-6: `~/.config/goad/env` is kept

- **Asked:** whether `just install`'s env half survives. Grep established the
  file has exactly one writer (`justfile`'s `install` recipe) and one reader
  (the out-of-tree `~/satan/goad/goad.service`, via `EnvironmentFile=`);
  nothing in the repository reads it. The recipe's own comment anticipates
  retirement — *"Slice 006 … wraps the environment into the binary, at which
  point there is no pair to keep in sync and no env file to go stale."*
- **Recommended:** A, delete the env half — the comment's claim holds for the
  **nix** binary and not for the `cargo install`ed one, whose only remaining
  runner is a person at a shell, and the devshell already supplies both
  variables. The recommendation was made conditional on a habit the agent
  cannot see: whether the user ever runs `~/.cargo/bin/goad` outside
  `nix develop`.
- **Decided:** *"it's a design goal to not stop goad running on non-nixOS
  systems; so I choose B"* — the env file is kept.
- **Consequence:** `just install` is unchanged. The file's consumer becomes a
  **person** rather than a unit: the module carries no `EnvironmentFile`, so
  after this slice nothing reads the file automatically. The `${VAR:?…}` guard
  is what keeps it honest, and re-running `just install` is the repair when the
  store paths in it are collected. The design states this rather than leaving a
  file with no stated reader, and carries the staleness as a named risk.
- **Raised by the decision and put to the user next:** the guard makes
  `just install` **require the devshell**, which is itself a thing that blocks
  an install on a non-NixOS machine.

### 2026-09-20 — the non-NixOS path is `cargo install`, and C is a follow-up

- **Asked:** whether the non-NixOS goal obliges a change to `just install`,
  whose `${VAR:?…}` guard makes the recipe require the devshell and therefore
  unrunnable on a non-NixOS machine. **A**: nothing to do — the recipe was
  never the non-NixOS path; a user on another distribution runs
  `cargo install --path crates/goad --locked`, their linker finds the GUI
  libraries and their fontconfig has a system configuration, so neither
  variable nor any env file is needed. **B**: the recipe degrades — install
  unconditionally, write the env file only when the devshell supplied the
  values; cost, a nix user who forgets `nix develop` gets a quietly env-less
  install, the exact failure the guard was added to make loud. **C**: the goal
  is larger than the recipe — documentation for building without nix, and
  possibly the gate.
- **Recommended:** A, with one piece of work attached — the design states out
  loud that the nix package and the env file are both *nix-path* mechanisms and
  that the non-NixOS path is plain `cargo install`, needing neither. That is
  true today and written down nowhere, which is why packaging with nix reads as
  though it narrows where goad runs.
- **Decided:** *"I'll take your suggestion for now; longer term probably C, so
  let's note as a follow up"*.
- **Consequence:** `justfile` is untouched by this slice beyond whatever the
  nix work needs; the design carries the statement above, and **C becomes a
  follow-up** — written into `slice-006.md` §Follow-ups at close and drawn from
  `notes.md` §Open, per `docs/AGENTS.md` §Close. Scope of that follow-up as
  understood now: a documented non-nix build path, and the question of whether
  anything verifies it.

### 2026-09-20 — OQ-2b: a `just package` recipe, with no standing obligation

- **Asked:** the middle option OQ-2 carried unanswered. Two halves, separated
  because they cost differently: **the recipe existing** is a `justfile` entry
  that touches no canon — POL-001's mirror rule binds the `check` recipe to the
  policy's six-command block and a different recipe touches neither — while
  **audit being obliged to run it** is an obligation on every future slice, has
  to live in POL-001 or `docs/AGENTS.md` to bind anything, and is therefore a
  canon amendment and **tier 2**. Options: **A** no recipe; **B** the recipe
  with no standing obligation; **C** recipe plus obligation, raising the tier.
- **Recommended:** B, weakly. The `justfile` is this repository's documented
  entry point and a packaging command that appears nowhere in it is a command
  nobody finds; the recipe's comment is somewhere to put the `path:` trap
  (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`) so it is not
  rediscovered a third time. The argument against was stated: a recipe nothing
  is obliged to run is a recipe that rots, and `just --list` gains an entry
  whose answer to *"when do I run this?"* is *"when you feel like it."*
- **Decided:** *"B"*.
- **Consequence:** `just package` builds both packages, sits outside POL-001's
  block, and amends no canon — **the tier stays 1 and OQ-2 is now closed
  entire**. The recipe carries the bare-git-form caveat in its comment. *This*
  slice's `audit.md` runs it as evidence for AC-1 and AC-2, which the audit
  does regardless; no future slice is bound.

### 2026-09-20 — the design runs 46 lines over the tier 1 cap, deliberately

- **Asked:** `docs/AGENTS.md` §Tiers caps a tier 1 `design.md` at 300 lines and
  prescribes splitting the slice above it, naming compression-to-fit as the one
  dishonest way to pass. The drafted design measured **337** before the note
  recording this decision (346 with it). §5.2 alone is 80 lines because it is
  eight distinct contracts — flake outputs, source filter, revision, module
  option surface, argument surface, error surface, `version_line`, the recipe —
  across four files, two crates and two languages. Three options were put:
  **split** (the halves share only `GOAD_REVISION`; packaging ≈ 230 lines and
  the startup surface ≈ 130, with AC-5 moving to the second); **raise to tier
  2**, which `docs/AGENTS.md` permits mid-slice and which removes the cap at the
  cost of separate `review-design.md` and `review-plan.md` ledgers with
  unbounded rounds; **cut surface**, which reaches 37 lines only by dropping
  whole contracts including the module and AC-7 with it.
- **Recommended:** split. The cap is a proxy for *can a two-round shared
  design-and-plan review hold this?*, and eight contracts across two languages
  is not what "thin" means; the reviewer competence also splits cleanly, nix on
  one side and Rust on the other. Cost named on that side: a second full
  lifecycle, scope through close, for work already designed.
- **Decided:** *"I say we just stick to tier 1 and let it hang 40 lines over."*
- **Consequence:** an explicit deviation from `docs/AGENTS.md` §Tiers, which its
  own §Workflow preamble admits on explicit user instruction. `design.md` says
  so at its head so that audit meets the reason before the line count.
  **Nothing else about tier 1 changes** — one shared `review-design.md` for
  design and plan, at most two rounds. The escape if round 2 leaves serious
  findings open is unchanged and still applies: `settle-in-code` with a named
  phase and a named test, or raise the tier.

### 2026-09-20 — round 1 of the joint ledger: two calls the repairs needed

- **Asked:** `review-design.md` round 1 raised eight findings, no blockers, five
  major. Six were mechanical repairs to `plan.md`. Two needed the user, because
  both amend a design that is accepted, committed and already 46 lines over the
  tier 1 cap.
- **F-1 — AC-3's only discharge cannot fail for the defect it guards.** Every
  check in PHASE-01/VA-4 passes on a binary wrapped with `LD_LIBRARY_PATH` and
  not `FONTCONFIG_FILE`, which is exactly the half-wrapped case `slice-006.md`
  §Purpose exists to retire; `design.md` §9's own row prescribes the same blind
  checks. **A**: read the generated wrapper for both variables — deterministic,
  seconds. **B**: A, plus run the packaged binary under this repository's own
  `goadShot` (cage + grim, headless) and look at the photograph.
- **Decided:** *"Wrapper text + headless run"* — B. A fontless wrapper is now
  caught in the phase that wrote it rather than at PHASE-05, four phases later.
  AC-1's second half is unmoved: still a person, under systemd (PHASE-05/VH-1).
- **F-4 and F-5 — the two design amendments.** F-4: four sites file the
  unscanned `.nix` surface under *"POL-001 §Verification's residue category"*,
  which canon does not have — it names one residue, about feature unification,
  in the one section whose subject is not blurring its own enumeration. F-5: the
  statement the user accepted **instead of** option C on 2026-09-20 — that the
  wrapper and the env file are nix-path mechanisms and a non-NixOS machine needs
  neither — was never written into `design.md`, while `notes.md` §Open's
  follow-up load-bears on it being there. Options put: both (cap grows); F-4
  only, F-5 deferred; both with ~4 lines cut elsewhere to hold at 46.
- **Decided:** *"Both, cap grows to ~50 over"*. As landed the design is 355
  lines, 55 over; the head states the number and why it moved. F-5 discharges an
  endorsed decision rather than taking a new one, and F-4 keeps the slice tier 1
  by not claiming a second entry in POL-001's count.

### 2026-09-20 — round 2: the photograph that could not fail, and the bound accepted

- **Asked:** round 2 verified seven of eight repairs, **contested F-4**, and
  raised three more. Two calls.
- **F-9 — VA-7 was F-1's proxy class in better clothes.** `goad-shot` is
  reachable only from the dev shell, which exports both `LD_LIBRARY_PATH` and
  `FONTCONFIG_FILE`; `--set-default` is a no-op against a caller's value and
  `--prefix` prepends to a list already naming all five `guiLibs`, so a wrapper
  missing both flags photographs identically. **A**: repair the invocation —
  `env -u` the two variables, absolutise the paths. **B**: drop VA-7 and rest
  PHASE-01 on VA-6's reading of the wrapper text.
- **Decided:** *"Strip the two variables"* — A. Confirmed from inside the dev
  shell before landing: the two are absent under the `env -u` prefix, `PATH`
  survives for the demo backend, and cage and grim are reached by store path.
- **The review bound.** Round 2 is the last tier 1 allows, so these four repairs
  land with no adversarial pass — and round 2 is precisely what caught a round 1
  Response that overstated its repair (F-4). **A**: accept the bound, verify
  mechanically. **B**: raise to tier 2, which buys an unbounded ledger and
  cannot be undone. **C**: land them, then one verification-only pass.
- **Decided:** *"Accept the bound; I verify mechanically"* — A. Each repair was
  checked by the check that would have failed had it not landed, and the ledger
  records the responder switching to the raiser's role to set those outcomes,
  which the ledger Protocol permits when declared.
- **Consequence:** `review-design.md` is **resolved** — eleven findings, no
  blockers, all `fix-now`, nothing outstanding. The tier stays 1. Three risks
  are recorded as knowingly standing in its Synthesis, the third being that the
  round 2 repairs carry no adversarial pass.

### 2026-09-21 — the plan is accepted; execution opens at PHASE-01

- **Asked:** acceptance of the five-phase plan, the joint design + plan ledger
  having resolved at `4f9fb9d` — eleven findings over two rounds, no blockers,
  all `fix-now`, nothing outstanding.
- **Decided:** *"plan accepted"*.
- **Consequence:** the plan stage closes and execution opens at PHASE-01, the
  crane packages. `slice-006.md` §Stage reads `planned`; `notes.md` §Status
  carries all five phases at `pending`. The tier stays 1. PHASE-01's phase sheet
  is written immediately before its agent starts, per `docs/AGENTS.md` §Phase
  plan — not now. Three risks stand knowingly, recorded in `review-design.md`
  §Synthesis: I4 has no instrument, PHASE-01's named cut has no trigger, and the
  round 2 repairs carry no adversarial pass.

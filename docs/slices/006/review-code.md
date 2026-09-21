# Review — implementation — Slice 006

**Subject:** implementation — `git diff 4f9fb9d..HEAD`, excluding
`docs/slices/006/`. Fifteen files: `.gitignore`, `Cargo.toml`,
`crates/goad-emit/src/main.rs`, `crates/goad-emit/src/render.rs`,
`crates/goad/Cargo.toml`, `crates/goad/src/diagnostics.rs`,
`crates/goad/src/main.rs`, `crates/goad/src/startup.rs`,
`crates/goad/tests/binary/main.rs`, `crates/goad/tests/binary/version.rs`,
`crates/goad/tests/renderer/startup.rs`, `flake.lock`, `flake.nix`, `justfile`,
`nix/module.nix`. HEAD is `177f383`.
**Reviewer:** fresh agent, Claude Opus 5
**Opened:** 2026-09-21
**State:** open

Per `docs/AGENTS.md` §Tiers this ledger is **full strength with rounds
unbounded at both tiers** — the tier 1 two-round bound applies to
`review-design.md` and to nothing here.

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-09-21 — Written before the diff was opened. Attacked in this
order of expected yield:

1. **The wrapper, read from the store and not inferred.** AC-3 and I2 are the
   slice's reason to exist, and `review-design.md` F-1 and F-9 are the record of
   two successive attempts to write a check for them that could fail. The
   generated script at `$out/bin/goad` is the artefact; `--prefix` against
   `--set-default` is the semantic question, and the case that matters is a
   caller who already has the variable set. What does each flag do to a caller's
   `LD_LIBRARY_PATH` that names a *different* GTK? Is `FONTCONFIG_FILE`
   deferential where deference is wrong, or right?
2. **`doCheck` against `cargoExtraArgs`.** `research.md` S-3 and the plan's
   PHASE-01 note both name the trap: crane's `checkPhaseCargoCommand` inherits
   `cargoExtraArgs`, so a `--bin` selector reports green having run nothing.
   `doCheck = false` is the settled answer (OQ-5, D5) — check that it is
   actually set on all three derivations, including `buildDepsOnly`, and that
   `--locked` survives the `cargoExtraArgs` override that the Harvest records as
   the thing both the spike and doctrine get wrong.
3. **Two copies of one rule.** `option_env!("GOAD_REVISION")` with the
   set-but-empty-is-unset filter exists in `crates/goad/src/main.rs` and in
   `crates/goad-emit/src/render.rs`. Two transcriptions of one rule is one
   opportunity for them to disagree. Check both against what `flake.nix` can
   actually pass: `self.shortRev or self.dirtyShortRev or ""` has three
   branches, and a dirty tree and a tarball fetch are two of them. A1 and R1 say
   the tarball case fails silently; confirm that is what happens rather than
   something worse.
4. **The new test tier, against the mutation it claims to catch.**
   `docs/memory/tests-asserting-proxies.md` records four green tests in this
   project that each asserted something the guarded regression would survive.
   For each new or changed case in `crates/goad/tests/binary/version.rs`,
   `crates/goad/tests/renderer/startup.rs` and the two `mod tests` blocks: what
   edit to the production code turns it red, and is that edit the one the
   criterion is about? Particular suspicion falls on VT-3's *`--version` is not
   read as a path* — a table case that asserts a variant can pass on a guard
   placed anywhere — and on the `[[test]]` declaration itself, since
   `autotests = false` means an undeclared target is silently not built and the
   Harvest records that as measured. Run `cargo test -p goad --test binary` and
   confirm the case actually executes; mutate and confirm it fails.
5. **`StartupError`'s enumeration, not its three interesting arms.**
   `docs/memory/verify-the-enumeration-not-the-conclusion.md`: AC-6 and I1 are
   claims about the *type* — every variant holding a path names it. Walk all ten
   variants, not the two the slice added. The doc comment's count is the
   instance PHASE-04/EX-3 named; the class is whether anything at all keeps a
   count honest. Also `clippy::wildcard_enum_match_arm` at every match over
   `Launch` and over `StartupError` — a lint denied at the crate root is an
   obligation the slice inherits at each new arm, and `Launch::Version`'s
   position relative to the catch-all `[only]` arm in `arguments` decides
   whether `--version` is answered or opened.
6. **`nix/module.nix` as systemd.** The three blocks are `review-design.md`
   F-11's repair, so the shape is likely right; the question is whether the
   *content* is correct systemd and correct home-manager.
   `RestartPreventExitStatus=2` encodes this repository's exit-code contract, so
   the check is not that the line is present but that `main` still maps every
   `StartupError` to exit 2 and nothing else to it — a success path that exits 2,
   or a failure that exits 1, breaks the contract from the other side.
   `extraConfig` merging over `Service` and no other block: what can a consumer
   override, and what can it not? `package` with no default, and `mkIf`'s
   interaction with a required option — a module whose `config` body is
   evaluated when `enable = false` is a different failure from one that is not.
7. **`flake.nix` as a whole, for what the slice could have broken without
   noticing.** `packages.${system}` was `jailPkgs` bare and is now a merge;
   losing a jail package is a silent regression nothing on the gate sees
   (PHASE-01/EX-4). The devShell, `goadShot` and `goadHeadless` are the
   instruments this repository photographs itself with, and PHASE-01/VA-7 used
   one of them — did using it change it? The `src` filter is spelled by
   directory (D9): check what it admits that it should not as well as what it
   drops, since a filter that admits the world still builds.
8. **`justfile` against POL-001 §Compliance.** The six-command block is
   canonical in the policy and mirrored in the recipe, policy first. `just -n
   check` must print those six, in that order, unweakened and unconditioned. The
   `package` recipe must carry no standing obligation on future slices (OQ-2b,
   D8), and `install` must still install a working pair (AC-9) — the Harvest
   says AC-9 was still open at PHASE-04 and the last commit says PHASE-05 closed
   it.
9. **The five invariants, ADR-001 and the vocabulary scan.** Nothing here should
   put a clock, a filesystem, a subprocess or an async runtime into
   `src/semantics/`, and nothing should put domain vocabulary anywhere. The
   slice's own I4 is the unenforced half — `nix/module.nix` and the unit text
   are read by no instrument, so they are read by me. `Cargo.toml`'s joined
   `tokio` entry is a workspace-manifest change and the residue POL-001
   §Verification names is about feature unification into stratum 1: did the
   join change any feature, or only whitespace?
10. **Citation rot.** `CLAUDE.md` §Working here — cite by symbol, never by line
    number. Every citation this slice *added* to code comments gets checked.
    The three pre-existing SPEC-003 violations recorded in `notes.md` §Open are
    out of scope and will not be filed.

**Invariants held to:** `CLAUDE.md`'s five, ADR-001's direction rule, POL-001's
six-command block and its counting rule, SPEC-003 R-3 and R-4, and the slice's
own I1–I4 and A1–A3.

**Not in scope and not filed:** the design's overrun of the tier 1 line cap (an
explicit user decision taken twice); the three SPEC-003 line-number citations
already recorded as owed at reconcile; anything inside `docs/slices/006/`, which
the subject excludes; and design findings already disposed in
`review-design.md`, unless the code diverged from the disposition.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major |  |  |
| F-2 | minor |  |  |
| F-3 | minor |  |  |
| F-4 | nit |  |  |
| F-5 | nit |  |  |
| F-6 | nit |  |  |

### F-1 — the exit-code contract `RestartPreventExitStatus=2` encodes is asserted by no test at any tier

**Severity:** major
**Location:** `crates/goad/src/main.rs`, `main`'s `Err` arm; `crates/goad/tests/binary/version.rs`; `nix/module.nix`, the `Service` block's comment

**Expected:** AC-7 and `nix/module.nix` both rest on one numeral. The module's
own comment states it as a contract — *"Exit 2 is any `StartupError` — `main`
in `crates/goad/src/main.rs` maps every one of them to it, and each new variant
inherits that"* — and `plan.md` PHASE-02's implementer note calls it **this
repository's exit-code contract**, which is the whole argument OQ-1 gave for
the module living in this repository rather than in `~/flakes`. A contract that
another artefact in the tree depends on by number, and that a person cannot see
break, is the kind this repository holds with a test: `docs/memory/tests-asserting-proxies.md`
is the standing warning, and this slice created the tier where the numeral is
observable.

**Observed:** nothing observes it. Changing `ExitCode::from(2)` to
`ExitCode::from(1)` in `main` leaves the whole gate green — measured, not
argued. The new binary tier asserts exit **0** for `--version` and nothing
else; `crates/goad-emit/tests/binary/exchange.rs`, the file
`plan.md` PHASE-03/VT-2 says to transcribe, asserts exit 1 and exit 2 at five
sites, so the convention being copied does hold its own exit codes and the
transcription took only the zero case.

The consequence is the one the module's comment says the directive exists to
prevent: with the failure code no longer 2, `Restart=on-failure` restarts a
service that cannot start, on a bad configuration or an ingress socket already
held, until systemd's start limiter stops it — and the gate stays green
through the change that caused it.

**Evidence:** measured in a copy of the tree at `177f383` under
`CARGO_TARGET_DIR` outside the repository, with `ExitCode::from(2)` replaced by
`ExitCode::from(1)` and nothing else changed:

```
cargo test --workspace                                  → exit 0
cargo clippy -p goad --all-targets -- -D warnings       → exit 0
```

and the tree restored afterwards. `grep -rn 'code_of' crates/goad/tests/`
returns only `tests/binary/version.rs`'s own helper and its single exit-0
assertion; `grep -n 'code_of(&output), [12]' crates/goad-emit/tests/binary/exchange.rs`
returns five lines.

The position a responder will reach for, and it is a real one:
`crates/goad/tests/renderer/startup.rs`'s module doc says *"No test here runs
the binary or asserts an exit code (§9's own rule) — `main`'s one `match` over
`run()`'s `Result` is what chooses the code, and every value that `match` sees
is already covered here."* That was argued before this slice and is sound about
the **renderer** tier. Two things this slice changed make it insufficient now:
the tier that can run the binary exists, and a second artefact in this
repository — `nix/module.nix` — now names the numeral the match chooses. The
claim *"every value that match sees is already covered"* covers the arms; it
does not cover the constant.

The behaviour is correct today, and was observed correct here at every reachable
arm: `goad /nonexistent/wat.toml`, a syntactically bad file, a file missing a
required field, and a directory all print a path-naming line on stderr and exit
2. That is what makes this a missing test and not a bug.

**Disposition:**
**Response:**

**Outcome:**

### F-2 — "set-but-empty is unset" is written twice, argued twice, and held by nothing

**Severity:** minor
**Location:** `crates/goad/src/main.rs`, `run`'s `Launch::Version` arm;
`crates/goad-emit/src/main.rs`, `Invocation::Version`'s arm

**Expected:** A1 and R1 are the slice's named risk that a tarball consumer
stamps no revision, and the mechanism that makes that case print a bare version
rather than an empty parenthetical is the filter on the caller:
`option_env!("GOAD_REVISION").filter(|revision| !revision.is_empty())`. Both
copies carry a comment arguing it. `design.md` §9's *`version_line` both
branches* row is what §Coverage gives AC-5 and I3 at the unit tier.

**Observed:** `version_line` takes `Option<&str>` and the filter sits outside
it, so the rule is in the two call sites and the two unit cases cannot reach
it. Nothing else can either: no test, fixture or gate command sets
`GOAD_REVISION` — `grep -rn GOAD_REVISION` over the tree returns the two call
sites, the two derivations in `flake.nix`, and four comments. Deleting
`.filter(…)` from **both** callers leaves the whole workspace green.

What the deletion would actually change is measured and is not nothing: built
with `GOAD_REVISION=""`, which is exactly what `self.shortRev or
self.dirtyShortRev or ""` yields for a tarball fetch, the filtered binaries
print `0.1.0` and the unfiltered ones print `0.1.0 ()`.

A rule written twice is one opportunity for the two copies to disagree, and
this one has no instrument that would notice if they did. It is testable where
it is not: moving the emptiness test inside `version_line` makes
`version_line(Some(""))` a pure case at the tier that already holds both other
branches, in both crates, at the cost of the call sites reading
`option_env!("GOAD_REVISION")` bare.

**Evidence:** measured in the same out-of-tree copy, with
`.filter(|revision| !revision.is_empty())` removed from both `main.rs` files
and nothing else changed:

```
cargo test --workspace                                       → exit 0
cargo clippy --workspace --all-targets -- -D warnings        → exit 0
GOAD_REVISION="" cargo build -p goad -p goad-emit --bins
  ./goad --version       → 0.1.0 ()
  ./goad-emit --version  → 0.1.0 ()
```

and, unmutated, the same build prints `0.1.0` from both. The three branches of
the flake's expression were checked directly —
`nix eval` over stand-in attrsets gives `shortRev` for a clean tree,
`dirtyShortRev` for a dirty one and `""` for a tarball — so the empty case the
filter exists for is reachable and is the one A1 names.

**Disposition:**
**Response:**

**Outcome:**

### F-3 — the `justfile` names a canonical block that no longer exists, and cites `AGENTS.md` by line number

**Severity:** minor
**Location:** `justfile`, the header comment

**Expected:** POL-001 §Statement — *"The gate is the command block in
§Compliance below, in that order; the `justfile` mirrors it, and a change goes
into this policy first and into the recipe second."* `CLAUDE.md` §Verifying
repeats it: *"The command block in `docs/policy/001-the-phase-gate.md` is
canonical and the `justfile` mirrors it."* `docs/AGENTS.md` §Canon that does not
exist yet: a draft is the slice's working authority *for the duration* and
**nothing outside the slice may cite it**. `CLAUDE.md` §Working here: cite by
symbol, never by line number.

**Observed:** the `justfile`'s header still points at slice 002's draft:

> `docs/slices/002/draft-policy.md`'s command block is canonical — the slice's
> working authority until it is promoted at audit (`docs/AGENTS.md:36`, `:38`)

`docs/slices/002/draft-policy.md` does not exist. It was promoted, and the
block is now POL-001 §Compliance. So the one file POL-001 names as its mirror
directs a future agent to a path that is not there, and describes canon as a
draft awaiting promotion. The parenthetical is two line-number citations into
`docs/AGENTS.md` of exactly the class `CLAUDE.md` says has rotted three times in
one slice; they happen to still land on the right sentences today, which is the
property that makes the class dangerous rather than safe.

This predates slice 006 — it is not in the diff. It is filed because the
`justfile` is one of the slice's declared surfaces, the slice edited it twice
(the `package` recipe and `install`'s comment), and `slice-006.md` §Governing
canon binds this slice to POL-001 in the direction of the `justfile`. A
reviewer who reads the file the slice changed and does not say this leaves the
next agent to find it.

The gate itself is intact: `just -n check` prints POL-001 §Compliance's six
commands, in order, and `package` is outside the `check` chain
(`check: build test test-stratum1 typecheck lint fmt-check`).

**Evidence:** `ls docs/slices/002/` — twelve files, none named
`draft-policy.md`. `sed -n '36p;38p' docs/AGENTS.md` returns the draft-authority
and promotion sentences, so the numbers are currently right and silently. `just
-n check` against POL-001 §Compliance: identical, six lines, same order.
`grep -rn draft-policy` outside `docs/slices/00{1,2}/` returns nothing else —
the `justfile` is the only surviving citation.

**Disposition:**
**Response:**

**Outcome:**

### F-4 — the slice repaired two stale counts of one class and wrote a third

**Severity:** nit
**Location:** `crates/goad/src/startup.rs`, `StartupError`'s doc comment;
`crates/goad/tests/renderer/startup.rs`, the module doc

**Expected:** PHASE-04/EX-3 states this repository's rule for the class in
terms — *"Fix the class: check the count after the edit rather than
incrementing the stale number"* — and the slice applied it twice, once to
`StartupError`'s own doc comment (eight, when there were nine) and once at
`review-design.md` F-10 to `plan.md` §Coverage's row count.

**Observed:** two files changed in the same phase took opposite strategies, and
neither is recorded as a choice.

`startup.rs` keeps a count and adds a second one to keep in step with it: *"The
ten variants … Two come from argument and environment handling, eight from the
steps after it."* Both numbers are correct today — the enum has ten variants,
two of them from argument and environment handling. Nothing holds either. A
new variant is forced by the compiler into `Display`'s exhaustive match and into
nothing else, which is precisely how the count reached the slice saying eight
with nine in the enum.

`tests/renderer/startup.rs` goes the other way and deletes its count: *"`StartupError`'s
`Display` for **every** variant it has"*, where it previously said *"each of its
eight variants"*; and `source_walk`'s case now says *"the arms added since are
named here as they arrive rather than counted."* The universal is true today —
all ten variants have a `Display` case — and it is unfalsifiable by reading,
where a count at least announces its own staleness.

Neither is wrong. Having both, decided silently in one phase, means the next
agent adding a variant has no rule to follow.

**Evidence:** `StartupError`'s variants, walked: `NoConfigPath`, `Usage`,
`ConfigUnreadable`, `ConfigUnparseable`, `Clock`, `Runtime`, `Platform`,
`EventLoop`, `Enqueue`, `Ingress` — ten, two of them from argument and
environment handling. `mod display_text` has a case naming each of the ten
(`Clock` twice), so the strengthened claim holds at `177f383`. The diff hunks
for both doc changes are in the same commit range as PHASE-04/EX-3's repair.

**Disposition:**
**Response:**

**Outcome:**

### F-5 — `extraConfig`'s type admits shapes home-manager's `Service` block rejects

**Severity:** nit
**Location:** `nix/module.nix`, `options.services.goad.extraConfig`

**Expected:** `design.md` §5.2(d) and PHASE-02/EX-1 give `extraConfig` one job:
*"Systemd directives merged over the generated `Service` block, and over no
other block."* home-manager's `systemd.user.services.<name>.Service` is an
attribute set of unit option values — strings, numbers, booleans, lists of
those — not of arbitrary attribute sets.

**Observed:** the declared type is `lib.types.attrsOf lib.types.anything`, which
accepts a nested attrset and merges it straight into `Service`. Rendered:
`extraConfig = { Unit = { Description = "hijack"; }; }` produces
`Service.Unit = { Description = "hijack"; }` — a consumer's plausible mistake
(reaching for the block this module does not let them reach) is accepted here
and rejected by home-manager's own type checker in the consumer's tree, which
is one repository away from the module that could have refused it. A tighter
type — `attrsOf (oneOf [bool int str (listOf str)])`, or home-manager's own
`unitOption` if it can be reached without an input — would refuse it at the
site that documents the restriction.

Everything else about the option surface checks out, measured rather than read.

**Evidence:** a `lib.evalModules` harness of the shape `plan.md` PHASE-02/VA-1
describes — a stub declaring only `home.packages` and `systemd.user.services`,
the real module, and an enabling fragment with a `derivation {…}` fake package,
`lib` from the flake's own locked nixpkgs and the flake reached as
`git+file:///home/david/dev/goad` — built in the scratchpad and run:

- `enable = true` renders `Unit` / `Service` / `Install` with `ExecStart`,
  `Restart = "on-failure"`, `RestartPreventExitStatus = 2`, `RestartSec = 2`,
  `After`/`PartOf`/`WantedBy` all `graphical-session.target`, and
  `Service ? EnvironmentFile` is **false**;
- `enable = false` with **no** `package` set evaluates to `{}` rather than
  erroring, so the required option costs a disabled consumer nothing;
- `extraConfig = { RestartSec = 10; Environment = "FOO=1"; }` overrides and adds
  inside `Service` as documented;
- `extraConfig = { Unit = { Description = "hijack"; }; }` lands as
  `Service.Unit`.

Independently, the unit systemd has actually loaded was read —
`systemctl --user cat goad` — and carries the three blocks, a store-path
`ExecStart`, `Restart=on-failure`, `RestartPreventExitStatus=2`, `RestartSec=2`
and no `EnvironmentFile=`; `systemctl --user show-environment` names neither
`LD_LIBRARY_PATH` nor `FONTCONFIG_FILE`, so the wrapper is what supplies them
in the deployed configuration and AC-3 holds there rather than by inheritance.

**Disposition:**
**Response:**

**Outcome:**

### F-6 — two prose slips in comments this slice added

**Severity:** nit
**Location:** `crates/goad/src/diagnostics.rs`, the module doc;
`crates/goad/src/main.rs`, `start`'s step-1 comment

**Expected:** the comments in these two files are dense and carefully wrapped,
and every one of them is argued. Both slips are in text this slice wrote.

**Observed:**

- `diagnostics.rs`'s module doc: the insertion left an orphan wrap — *"The
  module carries the arithmetic deny / because both halves / compute over
  lengths…"* — and introduces `006` as a bare slice id where every other
  reference in the paragraph is a phase (`PHASE-04`, `PHASE-05`, `PHASE-08`).
  Elsewhere the slice spells this `006/PHASE-03`, which is the form that reads
  unambiguously.
- `main.rs`, `start`: *"the same `Read` against the rest, and the one this
  file's `wildcard_enum_match_arm` deny admits"* — the cut is `Read`
  **against** the rest, so the phrase reads as a doubled preposition. The
  sentence is the one explaining why the match is on the `Result`, which is the
  non-obvious part of the change.

Neither costs anything to leave. They are noted because the surrounding prose is
the artefact a later agent reads to find out why the match has the shape it has.

**Evidence:** the `//!` block above `#![deny(clippy::arithmetic_side_effects)]`
in `diagnostics.rs`; the step-1 comment above `let config = match Config::load(path)`
in `main.rs`'s `start`. `cargo fmt --all --check` passes —
rustfmt does not rewrap doc comments, so nothing in the gate sees either.

**Disposition:**
**Response:**

**Outcome:**

## Synthesis — round 1

**Six findings: one major, two minor, three nits. No blockers.**

| | |
|---|---|
| F-1 | major — the exit-code contract `RestartPreventExitStatus=2` encodes is asserted by no test at any tier |
| F-2 | minor — "set-but-empty is unset" is written twice, argued twice, and held by nothing |
| F-3 | minor — the `justfile` names a canonical block that no longer exists, and cites `AGENTS.md` by line number |
| F-4 | nit — the slice repaired two stale counts of one class and wrote a third |
| F-5 | nit — `extraConfig`'s type admits shapes home-manager's `Service` block rejects |
| F-6 | nit — two prose slips in comments this slice added |

Both of the findings with teeth are about the same thing, which is worth saying
plainly: **the code is right and the things holding it right are thinner than
the documents claim.** Every behaviour this slice set out to produce was
observed working. What F-1 and F-2 say is that two of them would go on being
claimed after they stopped being true.

### What was attacked and found nothing in

This is the larger half of the round.

**The wrapper.** Read from the store, not inferred. `$out/bin/goad` carries
both flags — five `--prefix LD_LIBRARY_PATH` blocks over `guiLibs`, and
`export FONTCONFIG_FILE=${FONTCONFIG_FILE-'…fonts.conf'}` — so the half-wrapped
case `slice-006.md` §Purpose exists to retire is not what shipped. The
`--set-default` deference is real and deliberate (`${VAR-…}`, not `${VAR:-…}`),
and it matters less than it looks: `systemctl --user show-environment` names
neither variable, so under the unit the wrapper's values are the only ones. The
`--prefix` semantics are right for the defect — a caller's own GTK on
`LD_LIBRARY_PATH` loses to the store's, which is the direction a `dlopen`ing
binary needs.

**`doCheck` against `cargoExtraArgs`.** `doCheck = false` on all three
derivations, and the build log shows `--locked` surviving the `cargoExtraArgs`
override on both packages (`cargo build --release … --locked -p goad --bin
goad`), which the Harvest records as the thing both the spike and doctrine get
wrong. The flake's comment claiming `main.rs` has no unit tests, so the obvious
`doCheck = true` spelling would report green having run nothing, was checked
and is exactly right: `Running unittests src/main.rs … running 0 tests`.

**The source filter.** The filtered source in the store is 126 files and 3.3 MB:
`crates/goad/ui/app.slint`, `assets/{Inter,Geist}.ttf`, the manifests, the lock,
`clippy.toml`, `rustfmt.toml`, `examples/demo.toml`, the `.rs` files and
nothing else — `docs/`, `nix/`, `scratchpad/` and `tests/` survive as empty
directories and carry no content. D9's by-directory spelling admits
`assets/OFL.txt` and `assets/.gitignore`, which is what by-directory means and
is harmless. The prefix guard against `hasSuffix "assets"` is argued at the site.

**The flake as a whole.** `packages.x86_64-linux` is `default goad goad-emit
jailed-claude jailed-codex jailed-pi` — the merge did not lose a jail package
(PHASE-01/EX-4), and `default.pname` is `goad`. `crane` enters `flake.lock` with
no inputs of its own, so no second `nixpkgs`. The devShell change is a binding
extraction (`rust-bin.beta.latest.default` → `rust`) with the same value, and
`goadShot` / `goadHeadless` / `headlessEnv` are untouched. The revision
expression was exercised on stand-in attrsets for all three of its branches:
clean → `shortRev`, dirty → `dirtyShortRev`, tarball → `""`.

**The module, as generated and as loaded.** The `lib.evalModules` harness was
rebuilt from scratch rather than trusted from `notes.md`, and the results are in
F-5's Evidence: three blocks, no `EnvironmentFile`, `extraConfig` merging over
`Service` and no other, and `enable = false` with no `package` evaluating
cleanly. `systemctl --user cat goad` shows the unit systemd actually loaded,
which is the same shape from the loader's side — so F-11 of `review-design.md`
is discharged in the artefact and not only in the criterion.

**`StartupError` and `Launch`, walked rather than sampled.** Ten variants; three
hold a path (`ConfigUnreadable`, `ConfigUnparseable`, `Ingress`) and all three
name it; the other seven hold none. All ten have a `Display` case and all ten
answer `source()` `None`. `Ingress`'s SPEC-003 R-3/R-4 case is untouched and
passes. The behaviour was run, not read: a missing file, a syntactically bad
file, a file missing a required field and a directory each print a path-naming
line and exit 2, and `goad --version` no longer renders the same string as
`goad /nonexistent/wat.toml` — `research.md` S-4's defect is gone.

**`Launch::Version`'s position.** Held by the compiler, not only by the test:
moving the `[only] if only == "--version"` guard below the catch-all `[only]`
arm fails the build with *unreachable pattern* under `-D unused`. So the
regression the table case guards cannot reach a green gate by that route. The
case is still the right one to have — it pins the **value**, which the
compiler does not.

**The new tier runs.** `cargo test -p goad` executes
`tests/binary/main.rs` with one case. The `autotests = false` exposure the
manifest comment names was re-measured rather than taken from `notes.md`:
with the `[[test]]` block deleted, `cargo test -p goad` never mentions the
target and exits 0. That is a property `goad-emit` has carried since before
this slice and the comment at the site is the right answer to it, so it is not
filed.

**Dropping the path from `ConfigUnreadable`'s `Display`** fails the gate, so
AC-6's repair is held by a case that can see it.

**The gate, and the canon it mirrors.** `just check` exits 0 at `177f383`.
`just -n check` prints POL-001 §Compliance's six commands, in order; `package`
is outside the `check` chain and `install` is behaviourally unchanged. Nothing
in the diff touches `crates/goad-semantics`, so ADR-001's strata are not at
issue. The domain-vocabulary invariant holds in the surfaces no instrument
reads: `nix/module.nix`'s only word from the `DOMAIN` list is inside
`journalctl`, in a smoke-test comment. **No citation of the `path:line` class
was added anywhere in the diff** — `F-3`'s is pre-existing.

**`Cargo.toml`'s one-line `tokio` entry.** The reason was found before it was
judged, and it holds: `nix --version` reports Lix 2.95.2, which the comment
names, and the comment states the failure mode, the parser, and why `just check`
cannot see a re-split. It is the strongest comment in the diff.

**`.gitignore`** — `/result` and `/result-*` cover what a bare `nix build`
leaves; `just package` uses `--no-link` and needs neither.

**AC-5 and AC-9, which `notes.md` §Open still reports as open.** They are not:
`~/.cargo/bin/goad` and `goad-emit` are dated 2026-09-21 13:43 and both print
`0.1.0` at exit 0, against `0.1.0 (177f383)` from the packaged pair. The §Open
entry is the PHASE-04-era Harvest, which PHASE-05 did not refresh. Out of this
ledger's subject — it is inside `docs/slices/006/` — and flagged for the audit
rather than filed.

### What a round 2 should attack

1. **The repairs**, whatever they are. F-1 and F-2 both invite a fix that lands
   in the ledger's Response and not in the tree; `review-design.md` F-4 is this
   slice's own standing evidence that a Response overstating a repair survives a
   round.
2. **The rest of the binary tier.** If F-1 is fixed by adding exit-code cases,
   the next question is what else the tier should hold that the renderer tier
   cannot see — `--help`, the no-argument path with neither `XDG_CONFIG_HOME`
   nor `HOME`, stderr's `goad: ` prefix on a real process. `goad-emit`'s nine
   cases are the yardstick and `goad` has one.
3. **What I could not check, below.**

### What I could not check, and why

- **AC-1's second half — a window with text in it.** Not re-observed. It needs
  a compositor and a running host; PHASE-01/VA-7 (headless, under `goad-shot`)
  and PHASE-05/VH-1 (a person, under systemd) are the evidence, and re-running
  `goad-shot` would bind `./goad-demo.sock` and write into the checkout, both
  of which this review was told not to do. What I could verify about it I did:
  the wrapper carries `FONTCONFIG_FILE`, the store `fonts.conf` it names is the
  same path the dev shell exports, and the loaded unit supplies neither variable
  itself.
- **AC-2's second half — an envelope into a running host's socket.** Same
  reason: it binds the live host's socket.
- **A dirty-tree revision.** `0.1.0 (<rev>-dirty)` was verified as an
  *expression* (the `or` chain yields `dirtyShortRev` when `shortRev` is absent)
  but not as a build, because producing one means modifying a tracked file. An
  untracked file does not make the tree dirty — measured, and it is why writing
  this ledger did not disturb the builds above.
- **home-manager's own acceptance of the module's option types.** The harness
  stub is permissive by construction, as PHASE-02/VA-1 says. The loaded unit is
  the stronger evidence and it is in F-5; what neither reaches is what
  home-manager does with an `extraConfig` of the wrong shape, which is F-5's
  subject and is stated there as inference from the destination's type, not as a
  measurement.
- **`just install` and `just demo`** were not run: out of bounds. AC-9 was
  checked from its artefacts instead, above.

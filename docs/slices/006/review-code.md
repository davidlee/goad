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

**Round 2** — 2026-09-22 — Written before `git diff 91d46fe..HEAD` was opened.
Subject: the round-1 repair commit `9f53fe5` entire, everything the round-1
Responses claim, and the binary tier as it now stands. Attacked in this order of
expected yield:

1. **The two mutations, re-run rather than read.** Both Responses close on a
   measured claim — `ExitCode::from(2) → from(1)` reddens four of six binary
   cases, and deleting `.filter(…)` from both `version_line`s reddens exactly
   two unit cases and nothing else. Run the raiser's own mutation rather than
   reading its result, with `--no-fail-fast` or the coverage reads thinner than
   it is — `docs/memory/verify-the-proposed-instrument.md` is this repository's
   statement of why. A Response is a claim about the tree; the
   mutation is the only thing that makes it a measurement. `review-design.md`
   F-4 is this slice's standing precedent for a Response that overstated.
2. **`tests/binary/exit_codes.rs`, case by case, against what each claims to
   walk.** Five cases and a sixth pre-existing one. The Response says *one per
   exit code `main` can choose, walked by the arm that chooses it* — two claims,
   and the second is the falsifiable one. For each case: which `StartupError`
   arm does the spawned process actually reach, and would it still reach that
   arm on a machine configured differently? `no_argument_and_no_configuration_home`
   is the suspicious one — it depends on `env_remove` of two variables being
   sufficient to keep a real process off the machine's real configuration, and a
   case that silently reaches a *different* arm still asserts exit 2 and still
   passes. `docs/memory/a-green-test-can-assert-a-proxy.md` is the class.
3. **The helper extraction, as a refactor.** Four helpers moved from
   `version.rs` into `process.rs` "unchanged, plus one variant". Moves are where
   behaviour leaks: `autotests = false` means a file not reached by a `mod`
   declaration is silently not built, and round 1 measured that exposure on the
   `[[test]]` block itself. Does `cargo test -p goad --test binary` actually
   execute every case in all three files, and did `version.rs` lose anything in
   the move?
4. **F-4's sweep, against the class it declared.** The repair did not just fix
   two comments, it stated a rule — **name, never count** — and rules are swept,
   not spot-fixed (`docs/memory/a-repair-sweep-misses-the-binding-site.md`). Are
   there surviving counts of a growing list in the surfaces this slice wrote?
   The two deliberate exemptions are named in the Response and are sound; what I
   am looking for is a third instance nobody looked for, including one the
   repair commit itself wrote.
5. **What the repair commit touched beyond the repairs.** `git diff --stat` over
   `91d46fe..HEAD` first, and anything outside the six findings' declared
   locations is the strongest lead — the same rule the audit applies to phase
   surfaces. Round 1 cleared the wrapper, the flake, the module and the source
   filter at `177f383`; a repair that disturbed any of them is cleared by
   nothing.
6. **F-2's repair, for whether one home per crate is one home.** The rule moved
   inside `version_line`, in two crates. Two copies of a function are still two
   copies; the question is whether they now agree in behaviour at every input,
   not whether they read alike. Both crates' cases, and both call sites handing
   `option_env!` on unjudged.
7. **The binary tier as a whole**, which is round 1's own item 2. `goad-emit`'s
   nine cases are the yardstick and `goad` now has six. What can the tier see
   that the renderer tier cannot, and what of that is still unobserved?
8. **F-3, F-5 and F-6 as small verifications.** A citation that now resolves,
   `just -n check` still printing POL-001 §Compliance's six in order, the
   follow-up actually landed in `slice-006.md` with an accurate statement of the
   option surface, and both prose slips gone without a new one arriving.
9. **The gate, and the invariants, re-run at `9f53fe5`.** `just check` to exit
   0. No `path:line` citation added by the repair — `CLAUDE.md` §Working here,
   and the class F-3 was about. Nothing into `src/semantics/`, no domain
   vocabulary.

**Invariants held to:** the same as round 1 — `CLAUDE.md`'s five, ADR-001's
direction rule, POL-001's six-command block and its counting rule, SPEC-003 R-3
and R-4, and the slice's own I1–I4 and A1–A3 — plus the Protocol's own rule that
a disposition is discharged by the tree and not by its Response.

**Not in scope and not filed:** everything round 1 excluded, unchanged. Plus:
F-5 is `follow-up` and its *deferral* is not reopened here — only whether the
follow-up landed and says what it says it says.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | verified |
| F-2 | minor | fix-now | verified |
| F-3 | minor | fix-now | verified |
| F-4 | nit | fix-now | verified |
| F-5 | nit | follow-up | verified |
| F-6 | nit | fix-now | verified |
| F-7 | minor | fix-now |  |
| F-8 | minor | fix-now |  |
| F-9 | nit | fix-now |  |
| F-10 | nit | fix-now |  |

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

**Disposition:** fix-now
**Response:** Accepted in full, including the reading of why the renderer
tier's rule is insufficient now: it covers the arms, not the constant.

`crates/goad/tests/binary/exit_codes.rs` is new and holds five cases — one per
exit code `main` can choose, walked by the arm that chooses it:

| case | arm | asserts |
|---|---|---|
| `help_prints_the_usage_block_on_stdout_and_exits_0` | `Ok(())` | exit 0, `USAGE` verbatim on stdout, stderr empty |
| `too_many_arguments_exits_2_and_says_who_spoke` | `StartupError::Usage` | exit 2, the **whole** stderr line, stdout empty |
| `no_argument_and_no_configuration_home_exits_2` | `NoConfigPath` | exit 2, the whole line |
| `an_unreadable_configuration_exits_2_and_names_the_path` | `ConfigUnreadable` | exit 2, the `goad: ` prefix, the path |
| `an_unparseable_configuration_exits_2_and_names_the_path` | `ConfigUnparseable` | exit 2, the prefix, the path |

Two of them are compared against `report_startup_line(&StartupError::Usage)`
and `…::NoConfigPath` rather than against a literal, so the line and the
`"goad: "` prefix are pinned on a real process without restating text the
renderer tier already asserts verbatim. The two configuration arms assert the
prefix and the path only: `fault` is the OS's own words and varies.

**The mutation the finding measured now reds.** `ExitCode::from(2)` →
`from(1)`, `cargo test -p goad --test binary --no-fail-fast`: four of the six
cases fail, the two zero-exits pass. Restored, and `just check` exits 0.

Three things were written down rather than left implicit, because the finding
is as much about the claim as the test:

- `tests/renderer/startup.rs`'s module doc now says what it covers and what it
  does not — *"That covers the **arms**; it does not cover the **constant**
  either arm names"* — and names the file that holds the rest. The sentence
  the finding quoted stays, because it was never wrong.
- `StartupError`'s own doc states the contract at the type: every variant is
  exit 2, `nix/module.nix` depends on the numeral by value, and
  `tests/binary/exit_codes.rs` is what holds it.
- `exit_codes.rs`'s module doc carries the reasoning, including the measured
  consequence — a restart loop on a host that cannot start — so the next agent
  reads why the cases exist and not only what they assert.

The spawn helpers moved out of `version.rs` into `tests/binary/process.rs`
unchanged, plus one variant: `goad_with_no_config_home`, which `env_remove`s
both variables so the no-argument case cannot reach the machine's real
configuration. Copying four helpers into the second file would have been the
same drift this tier exists to catch.

Not done, and named as the boundary: nothing here reaches `Clock`, `Runtime`,
`Platform`, `EventLoop`, `Enqueue` or `Ingress`. Those need a compositor, a
broken clock or a held socket; the claim the cases support is that *the
constant* is 2, and every reachable arm agrees on it.

**Outcome:** verified — the mutation re-run in an out-of-tree copy of `9f53fe5`
reproduces the Response exactly: `ExitCode::from(2) → from(1)`,
`cargo test -p goad --test binary --no-fail-fast` → **4 failed, 2 passed**, and
the two that pass are the two zero-exits. The tier runs six cases, all six
executed under `cargo test --workspace`. Each failure's panic output carries the
stderr line of the arm its doc names, so the four exit-2 cases reach the four
arms claimed and not some other. The residue the Response declares — six arms
needing a compositor, a broken clock or a held socket — is stated accurately and
is not held against it.

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

**Disposition:** fix-now
**Response:** Accepted, and repaired the way the finding names: the emptiness
test moved **inside** `version_line`, in both crates.

`revision.filter(|revision| !revision.is_empty())` is now the scrutinee of the
`match`, and each call site hands `option_env!("GOAD_REVISION")` on unjudged.
One function decides the line; the rule has one home per crate instead of two,
and it is reachable from the tier that already holds the other two branches.

One new case in each crate,
`a_build_stamped_with_an_empty_revision_is_an_unstamped_build`, asserting
`version_line(Some(""))` is `0.1.0`. **Mutation-checked**: with the filter
removed from both, `cargo test --workspace --no-fail-fast` reds exactly those
two cases and nothing else — which is the finding's point restated, since
before the move the same deletion reddened nothing.

The reasoning moved with the rule rather than being deleted from the call
sites: `version_line`'s doc now carries the `build.rs`/`SLINT_STYLE`
precedent and the tarball case, and each call site keeps one line saying where
the decision lives. The two crates still hold two copies — stratum 3 has two
binaries and no shared crate, and each must read the `GOAD_REVISION` of its own
compilation (design.md §5.2(g)) — but each copy is now a function with a test
rather than an argument in a comment.

**Outcome:** verified — `.filter(|revision| !revision.is_empty())` deleted from
**both** `version_line`s, `cargo test --workspace --no-fail-fast`: exactly
`diagnostics::tests::a_build_stamped_with_an_empty_revision_is_an_unstamped_build`
and `render::tests::…` red, nothing else in the workspace, which is the
Response's claim to the word. Both call sites now hand `option_env!("GOAD_REVISION")`
on unjudged (`main.rs`'s `Launch::Version` arm, `goad-emit`'s
`Invocation::Version` arm). The drift this leaves in the plan and the design is
F-8 and is not held against the repair.

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

**Disposition:** fix-now
**Response:** Accepted. Pre-existing, and filed correctly: the `justfile` is a
declared surface of this slice, the slice edited it twice, and POL-001 names it
as its mirror in this direction.

The header now reads *"`docs/policy/001-the-phase-gate.md` §Compliance's
command block is canonical"*, with the derivation pointer to
`docs/slices/002/design.md` §5.6 kept — that file exists and is the record of
where the block came from. The draft-authority clause is gone, because the
draft was promoted, and both `docs/AGENTS.md` line-number citations are gone
with it: they were load-bearing only for the draft rule they explained. Nothing
replaces them, since the sentence no longer makes a claim about drafts.

`just -n check` is unchanged and still prints POL-001 §Compliance's six
commands in order.

**Outcome:** verified — `docs/policy/001-the-phase-gate.md` §Compliance exists
and its fenced block is `just -n check`'s six commands in the same order,
compared line for line; `docs/slices/002/design.md` §5.6 resolves to *The
ADR-002 trigger check, and the gate the split leaves*. Both `docs/AGENTS.md`
line numbers are gone and the header carries no `path:line` citation of any
kind.

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

**Disposition:** fix-now
**Response:** Accepted, and the rule decided rather than left to the next
agent: **name, never count.**

`StartupError`'s doc no longer opens with *"The ten variants … Two come from
argument and environment handling, eight from the steps after it."* It reads
*"Every way `run` can fail to reach the event loop, and the exact text of
each. `NoConfigPath` and `Usage` come from argument and environment handling;
the rest from the steps after it"*, and says why in one clause — a count is
stale at the next variant, nothing in the gate reads it, and that is exactly
how the number said eight with nine in the enum. `tests/renderer/startup.rs`'s
surviving count, *"`ClockError`'s for both of its"*, went the same way.

This is the same rule as `CLAUDE.md` §Working here's *cite by symbol, never by
line number*, applied to a cardinality instead of a location, and for the same
reason: the thing named survives an edit above it and the number does not. It
is recorded in `notes.md` §Audit, and Harvest §Open carries it as a candidate
row for reconcile — whether `CLAUDE.md` should state it is the user's call at
audit, not a repair's.

Two counts were deliberately **not** touched. POL-001 §Compliance's *"six
commands"* and the `justfile`'s mirror of it are canon's own count of a closed
list that a policy edit changes on purpose; and this ledger's own Synthesis
counts findings, which is a statement about a finished round and cannot go
stale.

**Outcome:** verified on both locations the finding named — `StartupError`'s
doc carries no cardinal and states the rule with its reason; the renderer tier's
*"`ClockError`'s for both of its"* is now *"for every one of its"*. The rule as
decided is sound and the two exemptions are correctly reasoned. That the sweep
stopped at the two named sites, and passed over `Launch`'s doc in the same file,
is raised as **F-10** rather than contested here: the finding asked for a rule
where there was none, and it got one.

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

**Disposition:** follow-up
**Response:** Accepted as real, deferred as a judgement rather than a defect.

The finding is right that the refusal happens one repository away from the
module that documents the restriction, and right that `attrsOf anything` is
looser than the destination. What it does not settle — and what a repair under
audit pressure would settle badly — is which shapes are legitimate.
`attrsOf (oneOf [bool int str (listOf str)])` is a guess at systemd's value
grammar, and home-manager's own `unitOption` is the right type but reaching it
means either a home-manager input this repository deliberately does not take
(OQ-1, and the `lib.evalModules` harness exists precisely because of that
choice) or copying its definition, which is a third transcription of a rule
this ledger has already found two of. A narrowing that refuses a directive a
consumer was entitled to is worse than one that accepts a shape they will be
told about by the type checker one repository over.

Landed in `slice-006.md` §Follow-ups with the option surface, the two candidate
types and the reason neither is obviously right — not put down in a
disposition.

Everything else in the Evidence is a confirmation and is taken as one: three
blocks, no `EnvironmentFile`, `extraConfig` merging over `Service` and no other
block, and `enable = false` with no `package` evaluating to `{}` — all of it
rebuilt from scratch rather than read out of `notes.md`, which is the stronger
form of the same check.

**Outcome:** verified — the follow-up is in `slice-006.md` §Follow-ups, not
left in the disposition, and it states the option surface, both candidate types
and the reason neither is obviously right. The deferral is a judgement the user
took and is not reopened.

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

**Disposition:** fix-now
**Response:** Accepted. Both repaired.

`diagnostics.rs`'s module doc is rewrapped — the orphan line is gone — and the
bare `006` is now `006/PHASE-03`, the form the rest of the slice uses.

`main.rs`'s step-1 comment now reads *"the shape `goad-emit`'s `socket_path`
already uses, cutting the same `Read` from the rest"*. The doubled preposition
was the whole slip; the sentence's claim is unchanged.

**Outcome:** verified — both named slips are gone: `diagnostics.rs`'s module
doc reads *"The module carries the arithmetic deny because both halves compute
over lengths…"* on one wrap and spells the slice `006/PHASE-03`; `main.rs`'s
step-1 comment reads *"cutting the same `Read` from the rest"*. No new slip in
the repaired text. One orphan wrap survives four lines above
(*"…the escape/bound / pipeline every line on / this surface goes through"*) and
is **not** filed: `git show 4f9fb9d:crates/goad/src/diagnostics.rs` has it
verbatim, so it is outside F-6's stated scope of *text this slice wrote*.

### F-7 — nothing holds which of the two configuration arms `start` routes to; the two binary cases assert what both satisfy

**Severity:** minor
**Location:** `crates/goad/src/main.rs`, `start`'s `match Config::load(path)`;
`crates/goad/tests/binary/exit_codes.rs`,
`an_unreadable_configuration_exits_2_and_names_the_path` and
`an_unparseable_configuration_exits_2_and_names_the_path`

**Expected:** PHASE-04/EX-1 and EX-2 are the phase's substance: one arm becomes
two, split *at the seam where the path is in hand*, rendered
`"{path} could not be read: {fault}"` and `"{path}: {fault}"` (`design.md`
§5.2(f)). D2 chose two arms over one precisely to avoid the doubled prefix that
one spelling produces on the commonest failure. A split is only a split if the
two halves stay on their own sides, and this repository holds that kind of
claim with a case that the wrong side reddens.

**Observed:** the routing is held by nothing. Both new binary cases assert exit
2, the `"goad: "` prefix and `stderr.contains(path)` — three properties
**both** arms satisfy, since both `Display` impls open with the path. The
renderer tier's `mod display_text` constructs `StartupError` values directly
and never calls `start`, so it holds the two texts and not the choice between
them; `grep` finds no other caller of `Config::load` or of `start` anywhere in
`crates/goad/tests/`.

Misrouting a read failure into the parse arm therefore leaves the whole gate
green — measured, not argued — while changing the line a person reads into
exactly the doubled-prefix spelling D2 rejected.

This is not a defect in F-1's repair: F-1 was about the constant, the constant
is now held, and `exit_codes.rs`'s own doc comments claim only what they
assert. It is that the two cases the repair added are the closest thing in the
tree to a test of PHASE-04's central change, and they are
`docs/memory/a-green-test-can-assert-a-proxy.md`'s shape — green against the
regression they sit next to. The cheap repair is one more assertion per case,
on the phrase only that arm says: `goad-emit`'s
`render.rs` already does exactly this and says why — *"a case asserting only
that stays green when two arms … Each of the three file faults is paired with
a phrase only its own arm says (F-7)"* — so the convention exists one crate
over and was not transcribed with the helpers.

**Evidence:** measured in a copy of `9f53fe5` exported with `git archive`,
outside the repository, `CARGO_TARGET_DIR` outside it as well. `start`'s first
arm, and nothing else, replaced by:

```rust
Err(ConfigError::Read(fault)) => {
  return Err(StartupError::ConfigUnparseable {
    path: path.to_path_buf(),
    fault: ConfigError::Read(fault),
  });
}
```

```
cargo test --workspace --no-fail-fast                  → 0 failed
cargo clippy --workspace --all-targets -- -D warnings  → exit 0
```

and the binary built from each, run against a missing file:

```
unmutated  goad: /nonexistent/wat.toml could not be read: No such file or directory (os error 2)
mutated    goad: /nonexistent/wat.toml: configuration could not be read: No such file or directory (os error 2)
```

Both exit 2, so the four exit-code assertions are untouched; the mutated line
is `ConfigError::Read`'s own *configuration could not be read* behind the
path — the doubled prefix `design.md` §7 D2 names as the reason there are two
arms at all. The copy was restored textually and verified byte-identical to
`HEAD` before anything else was run.

**Disposition:** fix-now
**Response:** Accepted, including the reading that this is not a defect in
F-1's repair but a gap the repair's two cases were the closest thing in the
tree to filling.

Each configuration case now asserts **its own arm's rendering, up to the part
that varies**, and nothing beyond it:

| case | asserts |
|---|---|
| `an_unreadable_configuration_exits_2_and_says_only_what_its_own_arm_says` | `starts_with("goad: /nonexistent/wat.toml could not be read: ")` |
| `an_unparseable_configuration_exits_2_and_says_only_what_its_own_arm_says` | `starts_with(&format!("goad: {}: ", path.display()))` |

The two prefixes are the arms' own contributions and cannot both be true of
one line: `ConfigUnreadable` renders the path, a **space**, then *could not be
read*; `ConfigUnparseable` renders the path and a **colon**, adding nothing
else because `ConfigError` already says what was wrong with the contents. Each
case was renamed to say what it now holds.

Both stop at the colon. The text past it is stratum 2's — `toml`'s own message,
carrying a line, a column and a caret excerpt — and pinning it here would make
a stratum-3 process case brittle against a dependency's wording for no gain,
which is the shape `docs/memory/tests-asserting-proxies.md` warns about from
the other direction.

**The finding's own mutation now reds, and reds precisely.** `start`'s `Read`
arm routed into `ConfigUnparseable`,
`cargo test -p goad --test binary --no-fail-fast`: **1 failed, 5 passed** — the
unreadable case, and only it. Restored textually; `git diff` on `main.rs` is
empty and `just check` exits 0.

The convention is `goad-emit`'s and is now cited at the site, so the next
transcription of that tier takes the assertion with the helpers rather than
after a review.

**Outcome:**

### F-8 — two round-1 repairs made five plan and design statements false, and nothing records the drift

**Severity:** minor
**Location:** `docs/slices/006/plan.md` PHASE-03/EX-3 and EX-5, PHASE-04/EX-3;
`docs/slices/006/design.md` §5.2(f) and §5.2(g); `docs/slices/006/notes.md`
§Open

**Expected:** `docs/AGENTS.md` §Audit — the audit walks *each verification
criterion in `plan.md`*, and `design.md` *is a record of intent at a point in
time. Do not retro-fit it to the code silently; where the implementation
departed and the design stands as written, say so under* **Design drift not
reconciled**. The mechanism for carrying that from a repair to the audit is a
row in `notes.md` §Open: this slice already uses it three times, each opening
*"Owed at reconcile:"*.

**Observed:** F-2's and F-4's repairs each reversed a decision the plan states
as an exit criterion, and the phases stay `done` with no row anywhere saying
so.

F-2 moved the emptiness filter from the call sites into `version_line`. That
makes false:

- PHASE-03/**EX-3** — *"the caller passes
  `option_env!("GOAD_REVISION").filter(|revision| !revision.is_empty())`"*.
  The callers now pass `option_env!("GOAD_REVISION")` bare.
- PHASE-03/**EX-5** — `goad-emit` takes *"the same `option_env!` filter"* in
  `main`'s `Invocation::Version` arm. It no longer does.
- `design.md` §5.2(g) — *"Callers pass
  `option_env!("GOAD_REVISION").filter(…)`."*

and it reverses a decision `notes.md` PHASE-03 §Decisions records with its
argument: *"**`option_env!` sits in each `main`, not inside `version_line`.**
EX-3 says *the caller passes* it, and that is what keeps `version_line` pure
and both of its branches a unit case rather than a build configuration."*
F-2 established that the argument's second half was wrong. Nothing says so
where the argument is written.

F-4 removed `StartupError`'s variant count. That makes false:

- PHASE-04/**EX-3** — *"`StartupError`'s doc comment says **ten** variants"*.
- `design.md` §5.2(f) — *"The doc comment's 'eight variants' becomes ten."*

and reverses `notes.md` PHASE-04 §Decisions, which states the opposite rule in
terms: *"the enum's doc says **ten** because the plan requires a count there"*.

Neither repair is wrong — both are improvements, and both were the user's
decision. What is missing is the record. `notes.md` §Audit's round-1 entry
describes the repairs and the rule and says nothing about the criteria they
falsify; §Open carries no row for either; `audit.md` is still the untouched
template, so its **Design drift not reconciled** slot has nothing to draw on.
An audit walking `plan.md` from the top finds three exit criteria unmet on
phases marked `done` and no explanation, which is the reading that costs the
most to undo.

**Evidence:** `plan.md` PHASE-03/EX-3 and EX-5 and PHASE-04/EX-3, quoted above
from the current file; `design.md` §5.2(f) *"The doc comment's 'eight
variants' becomes ten"* and §5.2(g) *"Callers pass
`option_env!("GOAD_REVISION").filter(|revision| !revision.is_empty())`"*, both
unchanged by `git diff 91d46fe..HEAD` — the repair commit touches neither file.
Against the tree: `grep -rn 'version_line\|print_version' crates --include=*.rs`
shows both call sites passing `option_env!("GOAD_REVISION")` unfiltered, and
`StartupError`'s doc comment contains no cardinal. `grep -rn 'drift' ` over
`notes.md`, `audit.md` and `design-log.md` returns three hits, none of them
this; `notes.md` §Open's three *"Owed at reconcile:"* rows are SPEC-003's
undercount, SPEC-003's three line-number citations, and the `CLAUDE.md`
*name, never count* candidate — the pattern this needs two more of.

**Disposition:** fix-now
**Response:** Accepted without reservation. The repairs were right and the
record of what they cost was missing, which is the whole finding.

One row added to `notes.md` §Open, in the established *"Owed at reconcile:"*
form, carrying all five statements as a table — `plan.md` PHASE-03/EX-3 and
EX-5 and PHASE-04/EX-3, `design.md` §5.2(f) and §5.2(g) — each against what the
tree says after the repair. It names `audit.md` §**Design drift not
reconciled** as its destination, and records that both repairs also reverse an
argument `notes.md` §Decisions states: the decisions stand as the record of
what was decided then, and the row is what says they were superseded.

One row rather than two, because it is one class with one cause — a
disposition taken at audit that changes code the plan described — and splitting
it would make the audit walk it twice.

**Nothing in `plan.md` or `design.md` was edited.** `docs/AGENTS.md` is
explicit that a design is a record of intent at a point in time and is not
retro-fitted to the code silently; the reconciliation row is the mechanism that
exists instead, and reaching for the file would have been the easier and wrong
repair.

**Outcome:**

### F-9 — the commit that repaired a citation to a file that does not exist wrote one

**Severity:** nit
**Location:** `docs/slices/006/notes.md`, §Audit, *"`review-code.md` round 1 —
the repairs"*

**Expected:** F-3 in this ledger is the class, one commit earlier: a citation
whose target is not there sends the next agent to a path that does not resolve,
and the `justfile`'s pointer at `docs/slices/002/draft-policy.md` was repaired
for exactly that reason. `CLAUDE.md` §Working here governs the neighbouring
form.

**Observed:** the §Audit entry attributes the re-run mutations to
`docs/memory/mutate-check-the-coverage-claim.md`. There is no such file.
`docs/memory/` holds 78 entries and none of them is it — the rule being cited
is real, but it lives in the agent's own session memory and not in this
repository, which is the failure mode: a private note cited as though it were
a repository fact a reader can open.

Three siblings predate the commit and miss for the same reason —
`docs/memory/subagent-session-budget.md` and
`docs/memory/gui-launch-needs-a-pipe.md` in PHASE-01's sheet (the latter also
in `plan.md` PHASE-01), and
`docs/memory/a-deferred-step-needs-a-checklist-box.md` in PHASE-05's. Filed as
one finding because the class is one class, and filed at all only because F-3
established that this slice files this class on its own surfaces.

**Evidence:**

```
for f in $(grep -rho 'docs/memory/[a-z0-9-]*\.md' docs/slices/006/) ; do
  [ -e "$f" ] || echo "MISSING: $f"
done | sort -u
→ MISSING: docs/memory/a-deferred-step-needs-a-checklist-box.md
  MISSING: docs/memory/gui-launch-needs-a-pipe.md
  MISSING: docs/memory/mutate-check-the-coverage-claim.md
  MISSING: docs/memory/subagent-session-budget.md
```

`git diff 91d46fe..HEAD -- docs/slices/006/notes.md` contains the
`mutate-check` line as an addition; the other three appear in no hunk of the
repair commit at all.
Every other path cited in an added line of the repair commit resolves, and the
commit adds **no** `path:line` citation anywhere — that half of `CLAUDE.md`
§Working here held.

**Disposition:** fix-now
**Response:** Accepted. The finding is exactly right about the mechanism, and
the instance is mine: `docs/memory/mutate-check-the-coverage-claim.md` is a
note in an agent's own session memory, cited in a repository artefact as though
a reader could open it.

All four are repaired by stating the rule in prose and dropping the path — the
rules are real and worth keeping, the pointers were never followable:

| site | now reads |
|---|---|
| `notes.md` §Audit | *"a reviewer's mutation is re-run by the responder or the coverage claim is the reviewer's word — with `--no-fail-fast`, or the count stops at the first failure"* |
| `notes.md` PHASE-01 sheet | *"A run that overruns finishes badly rather than finishing late."* |
| `notes.md` PHASE-05 sheet | *"a step named only in a paragraph is a step a person executing skips"* |
| `plan.md` PHASE-01 | *"a background launch, not an `&` — which exits 144, and a pipe does not fix it"* |

`plan.md` was edited and F-8's rule about not retro-fitting does not cover it:
a broken pointer is not a statement of intent, and replacing it with the
sentence it pointed at changes no criterion, no decision and no verification.
The distinction is worth naming because the two findings land in the same
commit and pull in opposite directions.

`docs/memory/` in this repository is a different thing from an agent's memory
directory and the two are easy to conflate from inside a session. The lasting
answer is the one already applied: if the rule is worth citing, write the rule.

**Outcome:**

### F-10 — *name, never count* was decided, and the counterexample eleven lines above the repair was not swept

**Severity:** nit
**Location:** `crates/goad/src/startup.rs`, `Launch`'s doc comment

**Expected:** the Protocol's guardrail — *fix the class, not the instance*.
F-4's repair did more than fix two comments: it decided a rule, wrote it into
`StartupError`'s doc as the reason the count is gone, named the two counts it
deliberately exempted, and put the rule to the user as a `CLAUDE.md` candidate.
A rule stated that widely is swept, and the first place to sweep is the file it
was written in.

**Observed:** `Launch`'s doc, eleven lines above `StartupError`'s in the same
file, carries two counts of the same growing list and was not touched:

> *What the arguments asked for. **Three outcomes**, and **the two** that
> answer and stop are outcomes here rather than an early `exit` hidden inside
> argument parsing…*

`Launch` is the enum this slice **grew** — it had two variants and gained
`Version` — and the slice's own edit changed *"Two outcomes"* to *"Three
outcomes"*. That is the act PHASE-04/EX-3 names and forbids in terms: *"Fix the
class: check the count after the edit rather than incrementing the stale
number."* The count is correct today for the same reason `StartupError`'s was
correct at eight until `Ingress` arrived.

What makes this worth a line is not the number — today it is right. It is
that the site was passed over by three separate enumerations of this class,
each of which set out to be exhaustive: PHASE-03 §Findings (*"`StartupError`'s
doc still says 'The eight variants'"*), PHASE-04 §Decisions (*"The stale count
was in **three** doc comments, not one … Fixing the class means the number
stops being the thing maintained by hand"*), and F-4's Response, whose *"Two
counts were deliberately **not** touched"* paragraph is where a reader looks
for what was considered and left. PHASE-04's sentence is literally true —
`Launch`'s number was not *stale*, because PHASE-03 had just hand-maintained
it — and that is the point: the class it names is a number maintained by hand,
and the fourth one in the same file had been maintained by hand in the same
slice, three phases earlier. `docs/memory/verify-the-enumeration-not-the-conclusion.md`
is the standing warning and this is a clean instance of it: three correct
conclusions, each resting on an enumeration that missed the same file's other
enum.

One weaker instance is noted and **not** filed as a count of a growing list,
because it reads as a statement about a finished sequence rather than a live
cardinality: `diagnostics.rs`'s module doc, *"006/PHASE-03 adds a third"*. The
module header's *"the three things `run` needs"* is the same judgement call and
is left to the responder.

**Evidence:** `git show 4f9fb9d:crates/goad/src/startup.rs` — *"What the
arguments asked for. **Two** outcomes, and `--help` is one of them"* — against
the current file's *"**Three** outcomes, and **the two** that answer and stop"*;
the enum gained `Version` in PHASE-03. `git diff 91d46fe..HEAD --
crates/goad/src/startup.rs` is a single hunk, `@@ -19,10 +19,18 @@ pub enum
Launch {` — the enum whose doc this finding is about is what git names as the
repair's enclosing context.
The three enumerations are at `notes.md` PHASE-03 §Findings, `notes.md`
PHASE-04 §Decisions, and this ledger's F-4 Response.

**Disposition:** fix-now
**Response:** Accepted, and it is the better catch of the two nits: the miss is
not the count, it is that *"Two counts were deliberately **not** touched"* is
the sentence a reader trusts to be exhaustive, and it was written eleven lines
below a third.

`Launch`'s doc now names its members instead of counting them — *"`Help` and
`Version` answer and stop, and they are outcomes here rather than an early
`exit` hidden inside argument parsing"* — and states, at the site, that this
enum's count was hand-incremented from two to three when `Version` arrived,
which is the act PHASE-04/EX-3 forbids in terms. Naming the increment rather
than quietly removing the number is deliberate: the next agent who reads the
rule gets the instance that produced it.

The two judgement calls the finding left to the responder are both **left
standing**, and the line between them and a swept count is worth stating once:
*name, never count* is about a cardinality of a list that **grows**.
`diagnostics.rs`'s *"006/PHASE-03 adds a third"* is a statement about a
finished sequence of phases, and the module header's *"the three things `run`
needs"* names a fixed tuple that a new `StartupError` variant or a new `Launch`
outcome does not change. Neither can be falsified by the edit the rule exists
to survive.

That reasoning is in `notes.md` §Audit rather than repeated at each site, so
the rule has one statement and the exemptions have one list.

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

## Synthesis — round 2

**Six outcomes, all `verified`. Four new findings: two minor, two nits. No
blockers, and none raised.**

| | |
|---|---|
| F-1 … F-6 | `verified` — every repair the round-1 Responses claim is in the tree, and both mutations they close on reproduce exactly |
| F-7 | minor — nothing holds which of the two configuration arms `start` routes to; the two binary cases assert what both satisfy |
| F-8 | minor — two round-1 repairs made five plan and design statements false, and nothing records the drift |
| F-9 | nit — the commit that repaired a citation to a file that does not exist wrote one |
| F-10 | nit — *name, never count* was decided, and the counterexample eleven lines above the repair was not swept |

**The repairs hold.** That is the round's main result and it was the thing most
worth attacking: `review-design.md` F-4 is this slice's own precedent for a
Response that survives a round while overstating, and nothing of that shape is
here. Both mutations were re-run rather than read, in an out-of-tree copy, with
`--no-fail-fast`, and both landed on the Responses' numbers to the case:
`ExitCode::from(2) → from(1)` reds four of six and passes the two zero-exits;
deleting the emptiness filter from both `version_line`s reds exactly the two
new unit cases and nothing else in the workspace. `just check` exits 0 at
`9f53fe5`.

**What the four new findings have in common is not the code.** F-7 is the only
one about behaviour, and even there the behaviour is right — what is missing is
the instrument. F-8, F-9 and F-10 are all the same shape as round 1's two with
teeth, one turn further on: *the things holding the claims true are thinner
than the claims.* Round 1 said that about the code's tests. Round 2 says it
about the repairs' own record — a plan whose exit criteria two repairs
falsified, a citation to a file that is not there, a rule declared and not
swept to the enum eleven lines above it. None of them gates anything. All of
them cost the next agent, which is the standard this repository sets.

**F-7 is the one to weigh properly**, because it sits exactly where a reviewer
is least likely to look: inside the repair that closed the round's only major.
F-1 asked for the constant and got it. The two cases that came with it look
like a test of PHASE-04's central change and are not one — both arms open their
`Display` with the path, so *exit 2, the prefix, the path* is satisfied by
either, and misrouting one into the other leaves the workspace green and clippy
clean. The convention that would close it already exists one crate over, in
`goad-emit`'s `render.rs`, written against this exact failure and cited to that
ledger's own F-7.

### What was attacked and found nothing in

**The whole repair diff, not only the six locations.** Fourteen files, and
nothing outside the findings' declared surfaces: no `flake.nix`, no
`nix/module.nix`, no `Cargo.toml`, no `flake.lock`. The `justfile` change is
the header comment F-3 named and nothing else — `just -n check` still prints
POL-001 §Compliance's six commands in order, compared line for line against the
policy's fenced block. So round 1's clearances of the wrapper, the source
filter, the packages merge and the module were not disturbed, because nothing
the repair touched reaches them.

**The binary tier, whole, and not only the cases the repair added.** Six cases
execute under `cargo test --workspace` — the `[[test]] name = "binary"` target
is reached, and all three files are reached through `main.rs`'s three `mod`
declarations. Round 1's own item 2 for this round is discharged: `--help`, the
no-argument path with neither `XDG_CONFIG_HOME` nor `HOME`, and stderr's
`goad: ` prefix on a real process are all now held, the last two against
`report_startup_line` rather than against a literal. Every case is
environment-independent in the way it claims: `goad_with_no_config_home`
`env_remove`s both variables and the mutation run proves the process really
reaches `NoConfigPath`, since the panic output carries that arm's exact line.

**`scratch_config`, for a parallel implementation.** It is not one.
`std::env::temp_dir()` qualified by case name and process id is this
repository's established spelling in seven files — `goad-shell`'s `config.rs`,
its integration `ingress.rs`, the renderer tier's `startup.rs` and
`ingress.rs`, `goad-emit`'s `main.rs` and its `exchange.rs`, and the root
`tests/support/scripting.rs` — all of them saying `tempfile` is not on the
manifest. The new helper follows it and cleans up before its assertions, so a
panicking case leaves nothing behind.

**The helper extraction.** `process.rs` is the four helpers plus
`goad_with_no_config_home`; the only change in the move is a private `command`
factored out of `goad`, which both spawners now share. `version.rs` lost
nothing but the definitions. Two copies across the two crates remain, and
`notes.md` §Open carries that as a known duplication with the reason — updated
in the repair commit to name the new file, which is the bookkeeping F-8 says
was not done for the plan.

**`version_line`'s two copies, for whether they now agree.** Identical bodies,
identical filter, one new case each with the same name and the same assertion.
Both call sites pass `option_env!` bare; `grep` finds no third caller of either
function in the workspace.

**Citation rot in the repair.** The repair commit adds **no** `path:line`
citation anywhere — checked over every added line of the diff, not sampled.
Every file path in an added line resolves except the one F-9 is about; the
crate-relative forms (`tests/binary/exit_codes.rs` cited from `startup.rs`)
resolve against their crate root and are the file's existing convention.

**The invariants.** Nothing in the repair reaches `crates/goad-semantics`, so
ADR-001's direction rule is not at issue and the four instruments are checking
the same configuration they were. The vocabulary scan does not read the two new
files — `excluded_dirs: &["tests", "target"]`, D13 and D17, an explicit design
decision and not a gap this repair opened. The new test text carries no word
from `DOMAIN`.

**`StartupError`'s new doc claim, *"Every variant is exit 2"*.** True, and held
structurally rather than by the cases: `main`'s `match` has one `Err` arm and
`clippy::wildcard_enum_match_arm` is denied at that crate root. The four
reachable arms are now asserted; the six that need a compositor, a broken clock
or a held socket are not, which is what F-1's Response says in terms.

### What I could not check, and why

- **The same three as round 1**, for the same reasons and unchanged by the
  repair: AC-1's window with text in it, AC-2's envelope into the running
  host's socket, and a dirty-tree revision build. The first two bind the live
  service's socket; the third needs a tracked file modified.
- **`just install` and `just demo`** — out of bounds, as in round 1.
- **home-manager's own acceptance of `extraConfig`'s type.** Unchanged by the
  repair and still F-5's subject, now a landed follow-up.
- **Whether the six unreachable `StartupError` arms exit 2 on a real process.**
  Stated as the boundary in F-1's Response and taken as accurate; reaching
  `Platform` needs a compositor, `Ingress` needs a held socket, and the socket
  this review may not touch is the one that would do it.
- **`nix build`, re-run.** Not re-run: the repair changes no derivation input
  that `nix build` reads differently — `flake.nix`, `Cargo.toml` and
  `flake.lock` are untouched — and round 1's store evidence therefore still
  describes the same artefact. Stated as inference, not as a measurement.

### What a round 3 should attack, if there is one

`docs/memory/review-rounds-stop-on-a-measured-trend.md` is the relevant rule
and the trend is measurable: round 1 found one major in the code, round 2 found
one minor in the code and three in the record. If F-7 is repaired, the thing to
check is the repair's own mutation — the arm-swap above, run — and after that
the honest reading is that the ledger has stopped producing code defects and
should close on mechanical verification rather than on a fourth round.

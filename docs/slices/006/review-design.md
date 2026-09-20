# Review — design + plan — Slice 006

**Subject:** design + plan — `docs/slices/006/design.md` (accepted, committed at
`a0ffdbd`) and `docs/slices/006/plan.md` (drafted, uncommitted). Tier 1: the two
share one ledger and this review runs **at most two rounds**
(`docs/AGENTS.md` §Tiers).
**Reviewer:** fresh agent, Claude Opus 5
**Opened:** 2026-09-20
**State:** resolved

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

**Round 1** — 2026-09-20 — Attacked, in this order of expected yield:

1. **Coverage closure.** `plan.md` §Coverage maps the nine ACs of `slice-006.md`
   and the twelve verification rows of `design.md` §9 onto phase criteria. For
   each row: if every entry and exit criterion of every phase were met exactly
   as written, would the AC be *true*, or only *proxied*? Hunting specifically
   for criteria that assert a stand-in the guarded regression would survive
   (`docs/memory/tests-asserting-proxies.md`), and for rows landed in a phase
   where nothing can observe them.
2. **Phase sizing.** `docs/AGENTS.md` §Plan: one phase, one agent, one session,
   bookkeeping included. PHASE-01 carries a manifest precondition, the whole
   flake, two negative controls and two `justfile` edits. If entry criteria are
   met but exit criteria are unreachable inside the session, name the cut.
3. **Plan decisions that are design decisions.** P-1 and P-2 in `plan.md`
   §Overview are taken by the plan, not stated by the design. §Plan says an
   unresolved design issue goes back to design; it is not repaired in the plan.
   P-1's self-enforcing claim about `builtins.fromTOML` gets tested against the
   actual evaluation semantics, not accepted on assertion.
4. **Load-bearing claims about the tree.** Both documents assert facts about
   files that exist: the manifest entry count, `StartupError`'s variants and the
   one that drops a path, which crate has a binary test target, what
   `packages.${system}` currently is, whether emit's binary test survives
   unchanged, and whether any ADR-001 instrument or the vocabulary scan reads
   `.nix`. Each is checked against the tree at `a0ffdbd`. A false load-bearing
   claim is a finding whatever its size.
5. **Canon.** POL-001's six-command block must not be weakened, conditioned or
   reordered; `just package` must carry no standing obligation. ADR-001's
   direction must hold with the path at stratum 3 (OQ-3). SPEC-003 R-3/R-4 must
   not be weakened by AC-6's repair. And the vocabulary invariant binds
   `nix/module.nix` and the unit text with no instrument reading either — is
   PHASE-02/VA-2 a real guard on that residue or a checkbox?
6. **The traps already paid for.** `doCheck = false` (crane's
   `checkPhaseCargoCommand` inherits `cargoExtraArgs`, so a `--bin` selector
   reports green having run nothing), the bare git flake reference and never
   `path:`, and the spike as a starting point rather than a template. Does the
   plan reintroduce one, or fail to warn a phase agent off one?

Two things are **not** in scope and will not be filed: the design's 46 lines
over the tier 1 cap (an explicit user decision, recorded at the document head,
in `slice-006.md` §What would raise the tier, and in `design-log.md`), and
anything downstream of it; and the absence of a separate `review-plan.md`, which
is correct at this tier.

**Round 2** — 2026-09-20 — Outcomes on F-1 … F-8, and the repairs themselves as
the new subject. The attack list is the one handed to this round before anything
was reopened, in its order: whether each repair landed in the **artefact** and
not only in the Response describing it; whether any fixed the instance and left
the class; VA-7, the criterion the responder flagged as the one they were least
sure of; F-3's `lib.evalModules` harness and whether its stated split leaves
AC-7's generator half discharged by something that cannot fail; F-5's paragraph
against what the 2026-09-20 decision actually attached; F-4's completeness
against a fresh grep rather than against the sites the finding listed; and
PHASE-01's size now that the plan names a cut. Round 1 declined to price that
cut; round 2 either endorses the one named or says why not.

The standing exclusions hold, and one is now larger: the design's overrun, at
53 lines, is an explicit user decision taken twice, and nothing downstream of it
is filed.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | verified |
| F-2 | major | fix-now | verified |
| F-3 | major | fix-now | verified |
| F-4 | major | fix-now · re-disposed r2 | contested → verified |
| F-5 | major | fix-now | verified |
| F-6 | minor | fix-now | verified |
| F-7 | minor | fix-now | verified |
| F-8 | minor | fix-now | verified |
| F-9 | major | fix-now | verified |
| F-10 | minor | fix-now | verified |
| F-11 | minor | fix-now | verified |

### F-1 — AC-3's only discharge cannot fail for the defect it guards

**Severity:** major
**Location:** `plan.md` PHASE-01/VA-4, and §Coverage's AC-3 row

**Expected:** AC-3 — *nothing a caller sets in the environment is required by
either binary* — is the second half of the defect `slice-006.md` §Purpose
names: the GUI libraries are `dlopen`'d, so `ldd` resolves clean on a binary
that opens no window, and fontconfig's absence is a window that draws no text.
*"Capturing one and not the other fails invisibly."* A criterion that discharges
AC-3 must therefore be able to **fail** when the wrapper carries one variable
and not the other.

**Observed:** PHASE-01/VA-4 is three checks, and all three pass on a binary with
**no wrapper at all**:

- `env -i ./result/bin/goad --help` — `design.md` §5.4 puts both zero-exits
  before any Slint call, so `--help` reaches neither a `dlopen`'d library nor
  fontconfig. This is stated in the design as the reason a binary test target is
  feasible; it is the same reason the check is blind here.
- `env -i ./result-emit/bin/goad-emit --version` — emit has no wrapper by
  construction (§5.2(a), PHASE-01/EX-3), so this observes nothing about
  wrapping.
- `ldd ./result/bin/.goad-wrapped` reports zero `not found` — this is precisely
  the fact §Purpose cites as what makes the defect invisible.

PHASE-01/EX-2 checks only that `./result/bin/goad` *is* a wrapper over
`.goad-wrapped`, not what the wrapper sets. §Coverage gives AC-3 exactly one
discharger, VA-4. The first thing in this slice that could fail on a
half-wrapped binary is PHASE-05/VH-1, four phases later and out of the phase
that wrote the wrapper.

**Evidence:** `slice-006.md` §Purpose, first bullet, and `design.md` §1 for the
`ldd`-resolves-clean fact; `design.md` §5.4, *"Both zero-exits precede any Slint
call"*; `research.md` S-4's table, where `env -i …/bin/goad --help` returns the
usage block at exit 0 and `env -i …/bin/goad-emit --version` prints `0.1.0` with
emit unwrapped. Two checks that *can* fail are available and named nowhere in
`design.md`, `plan.md` or `research.md`: reading the generated wrapper for both
`--prefix LD_LIBRARY_PATH` and `--set-default FONTCONFIG_FILE`, and running the
packaged binary under this repository's own headless instrument — `flake.nix`'s
`goadShot` and `goadHeadless` (cage + grim, `SLINT_BACKEND=winit-software`),
built for *"an agent can run the GUI and photograph it"*. `grep -rn
"goad-shot\|goadShot\|cage\|headless" docs/slices/006/*.md` is empty.

**Disposition:** fix-now
**Response:** Correct, and it reaches further than the plan: `design.md` §9's
own *an empty environment* row prescribes the same three blind checks, so the
plan inherited the defect rather than introducing it. Both are repaired.
Precision on the claim: `ldd` on an *unwrapped* build fails because
`.goad-wrapped` does not exist, which detects nothing — the case that matters
is the **half-wrapped** one, where a wrapper sets `LD_LIBRARY_PATH` and not
`FONTCONFIG_FILE`, and there all three checks pass. That is the defect
`slice-006.md` §Purpose names, and it is now guarded twice.
`design.md` §9 gains a row; `plan.md` gains PHASE-01/VA-6 — read the generated
wrapper for **both** `--prefix LD_LIBRARY_PATH` and `--set-default
FONTCONFIG_FILE` — and PHASE-01/VA-7 — run the packaged binary under this
repository's own `goadShot` (cage + grim, headless), open the PNG and look at
it. The user chose both over the wrapper reading alone, so that a fontless
wrapper is caught in the phase that wrote it rather than four phases later.
VA-4 survives, scoped in writing to AC-3's absence claim. AC-1's second half is
untouched and remains PHASE-05/VH-1, a person, under systemd.

**Outcome:** verified

### F-2 — PHASE-01/VA-4 asserts an emit output that PHASE-03 makes false

**Severity:** major
**Location:** `plan.md` PHASE-01/VA-4, against PHASE-01/EX-5 and PHASE-03/EX-5

**Expected:** every verification criterion is walked at audit — `audit.md`
§Evidence, *"each VT/VA/VH in `plan.md`, discharged or not"* — so a criterion
must still be true when the slice closes, not only in the phase that wrote it.

**Observed:** VA-4 asserts that `env -i ./result-emit/bin/goad-emit --version`
prints `0.1.0`. PHASE-01/EX-5 stamps `GOAD_REVISION` on **both** derivations,
and PHASE-03/EX-5 replaces emit's bare `env!("CARGO_PKG_VERSION")` with
`render::version_line` fed by the same `option_env!("GOAD_REVISION")` filter.
From PHASE-03 onwards the nix-built `goad-emit` prints `0.1.0 (<rev>)`, so VA-4
is true only in the window between PHASE-01 and PHASE-03. Nothing re-states it:
PHASE-05/VA-3 re-runs `just package` from the committed tree but asserts nothing
about either binary's output, and PHASE-05/VA-1 covers `goad` only.

**Evidence:** `design.md` §5.2(c), `GOAD_REVISION` *"on both derivations'
environment"*; `plan.md` PHASE-01/EX-5 and PHASE-03/EX-5. The plan is careful
about the converse case and scopes it correctly — PHASE-03's note that emit's
existing binary test *"passes unchanged, because `cargo test` never sets
`GOAD_REVISION`"* is true of `cargo test` and false of `nix build`, which is
what VA-4 runs.

**Disposition:** fix-now
**Response:** Correct. PHASE-01/EX-5 stamps `GOAD_REVISION` on both derivations
and PHASE-03/EX-5 makes emit read it, so `0.1.0` is emit's nix output only
between PHASE-01 and PHASE-03, and audit walks every criterion. VA-4 now asserts
exit 0 and *a version line* for emit, and says in terms why the exact string
must not be pinned there. The converse case the finding credits the plan with —
emit's existing binary test surviving because `cargo test` never sets the
variable — is unaffected and stands.

**Outcome:** verified

### F-3 — PHASE-02/VA-1 prescribes an evaluation this repository has no machinery for

**Severity:** major
**Location:** `plan.md` PHASE-02/VA-1

**Expected:** `docs/AGENTS.md` §Plan — `plan.md` captures the detail an agent
needs to attend to **just its own phase**; §Execute makes a dependency addition
a STOP-and-consult condition rather than a phase-time improvisation.

**Observed:** VA-1 requires the phase agent to *"evaluate the module against a
minimal home-manager configuration and print the rendered
`systemd.user.services.goad` attrset"*, and marks it as the generator half of
AC-7. `flake.nix` declares four inputs — `nixpkgs`, `rust-overlay`, `pub`,
`llm-agents` — and none of them is home-manager. A home-manager module's
`config` body sets options (`home.packages`, `systemd.user.services`) that
home-manager's own module set declares, so `lib.evalModules` over
`nix/module.nix` alone cannot render the unit. The plan offers no route: not a
new flake input (which is the STOP condition), not an impure evaluation against
`~/flakes`, and not the alternative of leaving all of AC-7 to PHASE-05/VH-2. The
phase's remaining criteria (EX-1, EX-2, EX-4) are all readings of the module's
own text, so if VA-1 is unreachable the phase discharges AC-7 by inspection of
the generator's source rather than of its output.

**Evidence:** `flake.nix`, the `inputs` attrset; `plan.md` PHASE-02/VA-1 and
PHASE-02 Exit. `design.md` §9's own row for this states the method as *"read the
module's generated unit **after `home-manager switch`**"* — PHASE-05's
machinery, in `~/flakes`, which is where home-manager actually is
(`research.md` Thread 2, prior art out of tree).

**Disposition:** fix-now
**Response:** Correct that the plan named no route, and the remedy was built
before it was written down rather than prescribed on faith
(`docs/memory/verify-the-proposed-instrument.md`). A `lib.evalModules` harness —
a stub module declaring only `home.packages` and `systemd.user.services`, the
real module, and a fragment enabling it with a fake package — renders
`config.systemd.user.services.goad` in full, `EnvironmentFile`'s absence
included. It was run at plan time against a module of this shape and printed the
expected attrset. No home-manager input, so no dependency addition and no STOP.
Two costs are now in the criterion: `lib` comes from the flake's own locked
nixpkgs, and the reference must be `builtins.getFlake "git+file://…"` — the bare
path form dies on `goad-demo.sock`, which this exercise reproduced live. The
criterion also states what the harness does **not** hold: permissive stub types
check the module's output, not home-manager's acceptance of it, and
PHASE-05/VH-2 remains the loader's half of AC-7.

**Outcome:** verified

### F-4 — "POL-001 §Verification's *residue* category" is not what POL-001 says

**Severity:** major
**Location:** `design.md` §3 (forces table), §5.5 I4, §8 R3, §10; `plan.md`
PHASE-02/VA-2

**Expected:** POL-001 §Verification's counting rule is *"four ADR-001
instruments, plus the domain-vocabulary check, plus **one residue nothing
enforces**"*, and **the** residue has one stated referent: *"a feature switched
on in a shared dependency by stratum 2 or 3 unifies into stratum 1's build under
`--workspace`, and no command in this gate rejects it."* The same section states
that no document may merge its parts into a single number; `CLAUDE.md` repeats
the rule.

**Observed:** five places in the two documents under review file the unscanned
`.nix` surface under *"POL-001 §Verification's residue category"* — as though
canon named a category of unenforced obligations that a second member may join.
It names one residue, about feature unification, which is a different fact from
an invariant no instrument scans. The underlying observation is correct and well
argued (no instrument reads `.nix` — verified below); what is wrong is the
citation, in an accepted design, of the one canon section whose subject is not
blurring its own enumeration.

**Evidence:** `docs/policy/001-the-phase-gate.md` §Verification, the counting
rule and the paragraph beginning *"**The residue** is real…"*.
`crates/goad/Cargo.toml`'s `[dependencies]` comment uses the term with exactly
that referent for `jiff`'s two features — the in-tree precedent for the word.
`research.md` Cross-thread 4 states it correctly as an **analogy** — *"in the
same category POL-001 §Verification calls residue"* — which `design.md` §10
compresses into a citation. The invariant claim itself checks out:
`crates/goad-boundary/tests/checks/vocabulary.rs`'s `domain_scan` and
`crates/goad-boundary/tests/checks/purity.rs`'s scans are all
`extensions: &["rs", "slint"]` or `&["rs"]` over `workspace.members`
directories, and `nix/` is neither.

**Disposition:** fix-now
**Response:** Correct, and the danger is larger than a mis-citation: POL-001
§Verification enumerates four instruments, the domain-vocabulary check and **one**
residue about feature unification, and forbids merging its parts into a single
number. A design that files a second obligation under that section reads as an
amendment to the enumeration — and an amendment is tier 2. All four sites now
say *a review obligation, not an enforced rule*, and §10 says explicitly that I4
does not join POL-001's count. The underlying observation, which the reviewer
verified independently, is untouched: no ADR-001 instrument and not the
vocabulary scan reads `.nix`. `research.md` Cross-thread 4 stated it as an
analogy and was right; the design compressed the analogy into a citation.

**Outcome:** contested — three of the four sites that carried the phrase are
repaired; `plan.md` PHASE-02/VA-2, which this finding's Location named, still
reads *"(POL-001 §Verification's *residue*; Cross-thread 4; R3)"*. It is the
load-bearing instance of the four: the criterion that **is** the obligation. The
Response's *"All four sites now say…"* counts the design's three and the plan's
one as repaired — `grep -n residue docs/slices/006/plan.md` returns line 298.
One correction the other way, and the responder was right not to act on it: this
finding's Location also listed `design.md` §8 R3, which carries the fact and
never carried the citation. That site needed nothing; the count of real sites
was four, not five.

### F-5 — the one piece of work the non-NixOS decision attached is not in the design

**Severity:** major
**Location:** `design.md` (absent); `design-log.md`, 2026-09-20 *"the non-NixOS
path is `cargo install`, and C is a follow-up"*; `notes.md` §Open

**Expected:** the user took option A **with work attached**: *"A, with one piece
of work attached — the design states out loud that the nix package and the env
file are both *nix-path* mechanisms and that the non-NixOS path is plain `cargo
install`, needing neither. That is true today and written down nowhere, which is
why packaging with nix reads as though it narrows where goad runs."* The entry's
Consequence records *"the design carries the statement above"*.

**Observed:** it does not. Every mention of the cargo path in `design.md` is
about the nix machine: §1 and §5.1's `cargo install --path` node with
`~/.config/goad/env` dotted onto it, §5.3's row for that file, §5.5's edge case
*"`cargo install` from a git checkout: still bare `0.1.0`"*, and D7's
rejected-alternative clause *"goad must keep running on non-NixOS systems"*.
Nothing says the wrapper and the env file are nix-path mechanisms, or that a
non-NixOS machine needs neither. `notes.md` §Open — the only home of follow-up C
until close — load-bears on the statement being there: *"this slice states in
`design.md` that the non-nix path is plain `cargo install --path crates/goad
--locked`, needing neither the wrapper nor `~/.config/goad/env`"*. The follow-up
is therefore written as though the work it was traded against had landed.

**Evidence:** `grep -n "cargo install\|non-NixOS\|NixOS" docs/slices/006/design.md`
returns six lines — §1's Boundary, §5.1's diagram node, §5.5 I3, §5.5's edge
case, and D7 — none of which is the statement. `design-log.md`, the entry's
**Recommended** and **Consequence** paragraphs; `notes.md` §Open.

**Disposition:** fix-now
**Response:** Correct, and verified against both sides: the `design-log.md`
entry of 2026-09-20 records option A taken *with one piece of work attached* and
its Consequence says *"the design carries the statement above"*, while a grep of
`design.md` for the six mentions of the cargo path returns the boundary
paragraph, the diagram node, I3, an edge case and D7 — none of them the
statement. `notes.md` §Open's follow-up C load-bears on a sentence that was not
there. §1 now carries it in five lines: the wrapper and `~/.config/goad/env` are
nix-path mechanisms, and a non-NixOS machine needs neither. This discharges an
endorsed decision rather than taking a new one. It puts the design seven lines
further over the tier 1 cap, from 46 to 53 (355 lines as landed); the overrun
was put to the user again with the two alternatives, and again accepted. The
document head now states the new number and why it moved.

**Outcome:** verified

### F-6 — P-1's "self-enforcing" claim is not what S-1 measured

**Severity:** minor
**Location:** `plan.md` §Overview P-1, and PHASE-01/VA-1

**Expected:** `research.md` S-1 measured that crane parses every manifest
whether or not the flake reads one itself: *"crane itself parses every
manifest: `buildDepsOnly` → `crateNameFromCargoToml` for a missing
`pname`/`version`, and **unconditionally in `cleanCargoToml`**, which builds the
dummy source. Supplying `pname` and `version` explicitly is **not** a
workaround; the dummy-source derivation still fails."*

**Observed:** P-1 claims the `builtins.fromTOML` read *"also makes the S-1
precondition **self-enforcing**: if the `tokio` entry is ever re-split,
evaluation fails loudly at the same place it failed at S-1"*, and VA-1 says
*"P-1's `fromTOML` read is what proves the manifest parses rather than a comment
claiming it does"*. The enforcement is crane's and is unconditional; the read
adds none of it, and removing the read would remove none of it. The conclusion
(a re-split entry fails loudly at evaluation) is right; the mechanism credited
for it is not, which matters because a later agent reading P-1 would think
dropping the read loosens the precondition, or that keeping it tightens one.

Separately, and left standing by the same false credit: nothing records the
constraint at the site it binds. `Cargo.toml` argues `publish = false`, the
enumerated `members`, and `jiff`'s two features in comments; PHASE-01/EX-1
requires the joined entry and a green gate but no note saying why the entry must
stay on one line — and `just check`, the command a later agent runs, cannot see
a re-split (OQ-2's accepted residue: it breaks in `~/flakes`).

**Evidence:** `research.md` S-1; `plan.md` §Overview P-1 and PHASE-01/EX-1,
VA-1; `Cargo.toml`'s `[workspace.dependencies]` block, where the `tokio` entry
carries no comment and the entries around it do.

**Disposition:** fix-now
**Response:** Correct on the mechanism, and the decision is unaffected. S-1
measured crane parsing every manifest *unconditionally* in `cleanCargoToml`,
with explicit `pname`/`version` not a workaround — so the enforcement is
crane's, the `fromTOML` read adds none of it, and removing the read would remove
none of it. P-1 now says the read is a single-source-of-truth choice and nothing
more, and PHASE-01/VA-1 stops claiming the read is what proves the manifest
parses. The finding's second half is the useful one and is taken: PHASE-01/EX-1
now requires a comment **at the `tokio` entry** saying why it stays on one line.
`just check` cannot see a re-split — that is OQ-2's accepted residue, which
breaks in `~/flakes` — so the comment is the only thing at that site that can,
and the entries around it are already argued the same way.

**Outcome:** verified

### F-7 — two §Coverage rows do not survive checking

**Severity:** minor
**Location:** `plan.md` §Coverage, the AC-8 and AC-4 rows

**Expected:** `plan.md`'s own header comment makes criterion ids immutable and
requires another phase's criterion to be cited phase-qualified; a §Coverage row
names criteria that can actually observe the AC.

**Observed:** two rows fail that, in one class — the map was not walked back
against the phases.

- AC-8's row reads *"every phase's `just check` (PHASE-01/EX-1 …
  PHASE-04/EX-3)"*. PHASE-04's gate criterion is **EX-5**; EX-3 is
  *"`StartupError`'s doc comment says **ten** variants."* The range's endpoint
  names a criterion that is not a gate run.
- AC-4's row names PHASE-03/VT-2 and VT-3 only. AC-4 requires the revision
  *"when the build stamped one"*, and PHASE-03/VT-1's own text says `cargo test`
  never sets `GOAD_REVISION` — so neither VT-2 (binary tier) nor VT-3 (argument
  table) can ever see a stamped revision. The only criterion that observes it on
  a real binary is PHASE-05/VA-1, which the AC-4 row does not name. The AC is
  closed by the plan; the row does not say by what.

**Evidence:** `plan.md` PHASE-04 Exit, EX-3 and EX-5; PHASE-03/VT-1, *"This is
the **only** place the stamped branch is reachable under `cargo test`"*;
PHASE-05/VA-1.

**Disposition:** fix-now
**Response:** Both correct, and in one class: the §Coverage map was written
forward from the phases and never walked back against them. AC-8's row now names
each phase's gate criterion individually (PHASE-01/EX-8, PHASE-02/EX-5,
PHASE-03/EX-7, PHASE-04/EX-5) instead of a range whose endpoint was the
doc-comment criterion. AC-4's row now names PHASE-03/VT-1 — the only place
`cargo test` reaches the stamped branch, as VT-1's own text says — and
PHASE-05/VA-1, which observes it on a real binary. AC-3's row is rewritten by
F-1 in the same pass.

**Outcome:** verified

### F-8 — AC-5's only discharge can pass against a binary from before the slice

**Severity:** minor
**Location:** `plan.md` PHASE-05/VA-1

**Expected:** AC-5 is *"a nix-built `goad` and a `cargo install`ed `goad` can be
told apart from their `--version` output alone"*, and §Overview says AC-5 cannot
be discharged until **both halves have landed**. VA-1 is its only discharger.

**Observed:** VA-1 runs `./result/bin/goad --version` and
`~/.cargo/bin/goad --version` *"one after the other"* and requires that they
differ — `0.1.0 (<rev>)` against `0.1.0`. It does not require
`~/.cargo/bin/goad` to have been built from this tree. The `~/.cargo/bin/goad`
that exists today, from the current `just install`, has no `--version` at all:
`arguments` reads the flag as a configuration path, and the binary exits 2 with
a diagnostic. That "differs" from `0.1.0 (<rev>)`, so VA-1 passes while
demonstrating nothing AC-5 asks for. PHASE-05/VA-2 re-runs `just install`, which
would refresh it, but nothing orders VA-2 before VA-1 and neither criterion
mentions the other.

**Evidence:** `crates/goad/src/startup.rs`, `arguments` — the catch-all
`[only] => Ok(Launch::Config(PathBuf::from(only)))`; `research.md` S-4's second
row, which is the measured output of exactly that invocation; `justfile`,
`install`, which is what put that binary on `$PATH`.

**Disposition:** fix-now
**Response:** Correct, and confirmed on this machine rather than from the
research note: `~/.cargo/bin/goad --version` today prints *goad: configuration
could not be read: No such file or directory (os error 2)* and exits 2, from a
binary dated before this slice. A criterion satisfied by *they differ* passes on
that. VA-1 now requires VA-2 to have run first, and requires **both** invocations
to exit 0 printing a version line, agreeing on the package version and differing
only in the parenthetical; an exit 2 on either side fails it. The ordering is
written into the criterion rather than left to the order the list happens to be
read in.

**Outcome:** verified

### F-9 — VA-7's photograph is taken in the one environment that hides what it is for

**Severity:** major
**Location:** `plan.md` PHASE-01/VA-7; `design.md` §9, the *the wrapper carries
both variables* row

**Expected:** VA-7 is the second of the two remedies for F-1, chosen by the user
over the wrapper reading alone *"so that a fontless wrapper is caught in the
phase that wrote it"*. Its whole marginal value over VA-6 is that it runs the
wrapper's values rather than reading them: VA-6 catches a wrapper **missing** a
flag, VA-7 is what would catch a wrapper whose flag is present and useless.

**Observed:** as written it can catch neither. `goad-shot` is on `PATH` only
inside the dev shell (`flake.nix`, `goadShot` is in `projectPkgs`, and
`projectPkgs` is the devShell's `packages`), so the agent running VA-7 is in the
dev shell by construction — and the dev shell exports **both** of the variables
the wrapper exists to supply: `LD_LIBRARY_PATH = lib.makeLibraryPath guiLibs`
and `FONTCONFIG_FILE = fontsConf`. `headlessEnv` does not unset them and cage's
child inherits them. Against that environment:

- `--set-default FONTCONFIG_FILE` is by design a no-op when the caller already
  has one — `design.md` §5.2(a), *"`--set-default` and not `--set`, so a
  caller's own `FONTCONFIG_FILE` still wins (S-4)"*. A wrapper that omits the
  flag **entirely** photographs identically.
- `--prefix LD_LIBRARY_PATH` prepends to a value that already names all five
  `guiLibs`. A wrapper that omits that flag too photographs identically.

So the photograph shows that *the dev shell's* environment draws text, which is
the status quo this slice replaces, and VA-4's rescoping to *"the absence claim
only"* leaves VA-6 — a reading of the wrapper's text — as the only criterion in
the phase that can fail on a half-wrapped binary. That is where F-1 started.

**Evidence:** `flake.nix`, `devShells.${system}.default`, the two variables set
on `mkShell`, and `projectPkgs`/`goadShot` for where the tool lives;
`design.md` §5.2(a) for `--set-default`'s deliberate deference to the caller;
`research.md` S-4, which measured the wrapper's effect under `env -i` precisely
because an inherited environment cannot show it. The criterion needs its
invocation pinned: strip exactly the two variables (`env -u LD_LIBRARY_PATH -u
FONTCONFIG_FILE goad-shot …` keeps `PATH`, which the demo backend needs), and —
separately — name absolute paths for the config and the output, because
`flake.nix`'s own `goadShot` comment says *"cage's child starts wherever cage
does"* and that is why it already absolutises `GOAD_SHOT_OUT`, while
`examples/demo.toml` is passed relative and its `command` is
`["bash", "examples/shell/backend.sh"]`, relative again.

**Disposition:**
**Response:**

**Outcome:**

### F-10 — the count of `design.md` §9's rows went stale in the repair that added one

**Severity:** minor
**Location:** `plan.md` §Coverage, the paragraph below the AC table

**Expected:** PHASE-04/EX-3 states this repository's rule for exactly this
class, against `StartupError`'s doc comment: *"Fix the class: check the count
after the edit rather than incrementing the stale number."*

**Observed:** F-1's repair added a row to `design.md` §9, which now has
**thirteen**. `plan.md` §Coverage still opens *"`design.md` §9's **twelve**
verification rows, in its order"*, and the enumeration that follows folds the
new row into the *an empty environment* item — *"(PHASE-01/VA-4, and the wrapper
row `design.md` §9 gained at F-1: PHASE-01/VA-6, VA-7)"* — so the list is no
longer one item per row and no longer in the design's order either. The mapping
is complete; the count and the shape that makes it checkable are not.

**Evidence:** `design.md` §9's table, thirteen rows under the header;
`plan.md` §Coverage; `plan.md` PHASE-04/EX-3 for the rule.

**Disposition:**
**Response:**

**Outcome:**

### F-11 — EX-2's field list names no unit blocks, which is the one thing the harness cannot catch

**Severity:** minor
**Location:** `plan.md` PHASE-02/EX-2, checked by PHASE-02/VA-1

**Expected:** VA-1 states its own limit honestly — *"the stub's option types are
permissive, so this checks the module's own output, not home-manager's
acceptance of it"* — and then discharges AC-7's generator half by checking
*"every field in EX-2 by name"*. For that split to hold, EX-2 must name the
thing a permissive stub cannot: **where** each field goes. home-manager's
`systemd.user.services.<name>` is three attrsets — `Unit`, `Service`, `Install`
— and `design.md` §5.2(d) knows it, defining `extraConfig` as *"merged over the
generated **`Service`** block"*.

**Observed:** EX-2 lists the fields flat: `ExecStart`, `After`/`PartOf`/
`WantedBy`, `Restart`, `RestartPreventExitStatus`, `RestartSec`, no
`EnvironmentFile`. Nothing says `Service.ExecStart`, `Unit.After`,
`Install.WantedBy`. A module that puts them at the top level, or `WantedBy`
under `Unit`, satisfies EX-2 read literally and renders an attrset in which
VA-1 finds every field by name — and home-manager either rejects it or drops it
silently. That failure then surfaces at PHASE-05, out of repo, at the cutover,
which is the one place the plan says a module defect must **not** be repaired
(*"it is a finding against PHASE-01 or PHASE-02 and is repaired there"*).

**Evidence:** `design.md` §5.2(d), *"merged over the generated `Service`
block"*; `research.md` Thread 2's prior art, which records both the shape —
`~/dev/satan-attrd/nix/module.nix`'s `systemd.user.services.<name> = { Unit;
Service; Install; }` — and the unit this one replaces,
`~/satan/goad/goad.service`, whose fields are already sorted into `[Unit]`,
`[Service]` and `[Install]` and whose `Install.WantedBy` research writes
qualified while writing the rest bare.

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

---

## Round 2 repairs — F-4 re-disposed, F-9 … F-11 disposed

Written by the **responder**. The two-round bound (`docs/AGENTS.md` §Tiers)
means there is no round 3, and the user chose to accept the bound with
mechanical verification rather than raise the tier (`design-log.md`,
2026-09-20). **The outcomes below were therefore set by the responder acting as
raiser** — the Protocol permits one agent to hold both roles provided the switch
is deliberate and declared, and this is that declaration. Each names the check
that would have failed had the repair not landed, and each check was run.

### F-4 — re-disposition, after a correct contest

**Disposition:** fix-now
**Response:** The contest is upheld in full and the error was the responder's.
The round 1 Response claimed *"All four sites now say…"* having repaired three;
`plan.md` PHASE-02/VA-2 — the criterion that **is** the obligation, and so the
load-bearing instance — still carried *"(POL-001 §Verification's residue;
Cross-thread 4; R3)"*. It now reads *a review obligation, not an enforced rule;
it does not join POL-001 §Verification's count*. The raiser's own correction is
accepted with it: `design.md` §8 R3 carries the fact and never carried the
citation, so the true count was four sites, not five.
This is the round's most useful finding, and not for its size: a Response that
overstates a repair is indistinguishable from one that describes it, and only a
second pass over the artefact separates them
(`docs/memory/writing-the-repair-down-is-the-review.md`).
**Outcome:** verified — `grep -n residue docs/slices/006/plan.md` is empty, and
the only surviving match across both documents is `design.md` §5.4's *"OQ-2's
accepted residue"*, the ordinary word used of OQ-2's own decision.

### F-9

**Disposition:** fix-now
**Response:** Correct, and confirmed from inside the shell the criterion would
have run in: `goad-shot` resolves only from the dev shell, and both
`LD_LIBRARY_PATH` and `FONTCONFIG_FILE` are set there. With `--set-default` a
no-op against a caller's value and `--prefix` prepending to a list already
naming all five `guiLibs`, a wrapper missing both flags photographs identically
— VA-7's entire marginal value over VA-6 was nullified by its own environment.
The repair is the raiser's: the invocation, not the criterion. VA-7 now runs
under `env -u LD_LIBRARY_PATH -u FONTCONFIG_FILE`, keeps `PATH` because the demo
backend is `["bash", "examples/shell/backend.sh"]`, and absolutises the binary,
config and output paths on `goadShot`'s own stated grounds. An empty compositor
now fails the criterion explicitly, and a `./goad-demo.sock` collision with a
running `just demo` is named as a collision rather than a packaging defect. The
`-s 5` settle is left as a starting guess, as the finding says it should be.
**Outcome:** verified — under the exact `env -u` prefix both variables are
absent and `PATH` is present (run); `goad-shot` reaches cage and grim by store
path, so stripping costs them nothing (read from the script).

### F-10

**Disposition:** fix-now
**Response:** Correct, and it is PHASE-04/EX-3's class exactly — a count stated
in one place and changed in another. §Coverage now says **thirteen**, says why
(twelve as accepted, plus F-1's), and gives the wrapper row its own item instead
of folding it into the empty-environment row.
**Outcome:** verified — `design.md` §9 has 13 data rows, and §Coverage lists one
item per row.

### F-11

**Disposition:** fix-now
**Response:** Correct, and the failure path is the one that matters: a flat
module satisfies EX-2 read literally, renders an attrset VA-1's permissive stub
passes, and surfaces at the PHASE-05 cutover — which this plan names as the one
place a module defect must not be repaired. EX-2 is now a three-row table,
`Unit` / `Service` / `Install`, with `extraConfig` merging over `Service` and no
other. `design.md` §5.2(d) carried the same flat list and is corrected in place,
length-neutral, since it is the contract the criterion transcribes. The raiser's
judgement that this is EX-2's to close and not VA-1's is accepted: the harness
observes content, not shape, and says so.
**Outcome:** verified — both documents name the three blocks, in the shape the
F-3 harness actually rendered at plan time.

## Synthesis

Two rounds, eleven findings, no blockers: five major and three minor in round 1,
one major and two minor in round 2. Every finding was disposed `fix-now`; none
was `tolerated`, `follow-up` or `settle-in-code`, and nothing is outstanding —
so the tier 1 bound is met without a third round and without raising the tier.

**What the review changed.** Three defects were structural rather than textual.
*AC-3 had no criterion that could fail*: every check `design.md` §9 prescribed,
and the plan inherited, passed on a binary wrapped with one of its two variables
— the exact defect `slice-006.md` §Purpose exists to retire (F-1) — and the
first repair was itself blind, because the instrument runs in a dev shell that
supplies both variables (F-9). It took both rounds to get a check that
discriminates. *A citation filed the slice's unenforced obligation under POL-001
§Verification's residue*, which names one residue about a different fact, in the
one section whose subject is not blurring its own enumeration; left standing it
reads as an amendment, and an amendment is tier 2 (F-4). *The statement the user
accepted instead of option C* on 2026-09-20 had never been written into the
design, while `notes.md` §Open load-bore on it (F-5). The rest repaired criteria
that named the wrong thing (F-2, F-7, F-8, F-10, F-11) or credited the wrong
mechanism (F-6).

**What it confirmed.** The phase decomposition, the independence of the two
halves, and the placement of the acceptance criteria no phase can discharge
alone. PHASE-01's size was examined in both rounds and left standing, with the
cut now named. P-1 and P-2 are the plan's to take. F-3's harness — built and run
before it was written into a criterion — closes the module's generator half
honestly and states in the criterion what it does not reach.

**Risks knowingly left standing.** Three. **I4 has nothing enforcing it**: no
ADR-001 instrument and not the vocabulary scan reads `.nix`, and this slice adds
none, because adding one amends POL-001 and is tier 2. **PHASE-01's named cut
has no trigger** — an agent mid-phase has no signal for *going to overrun*, so
the cut is available to a reader of the plan and not to the agent executing it.
**The round 2 repairs carry no adversarial pass**: they were verified
mechanically by the responder acting as raiser, under the bound the user chose
to accept. F-4 is the standing evidence for why that is a real cost — a Response
that overstated a repair survived a round.

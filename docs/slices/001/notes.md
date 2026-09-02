# Notes — Slice 001

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| design | **accepted** — review closed at round 5, 16 repairs unverified | 2026-08-26 |
| plan | **accepted 2026-08-29** — ten phases in `plan.md` (01…08, 10, 09); two design gaps found and closed; **four review rounds run**, fourteen findings, all repaired and all confirmed; round 4 clean, plan judged executable | 2026-08-29 |
| PHASE-01 | **done** — `just check` exits 0 in both feature columns. All six EX and all seven V criteria discharged; see `## Phase sheets` | 2026-08-29 |
| PHASE-02 | **done** — `just check` exits 0 in both feature columns; 17 unit tests, 13 of them this phase's. All four EX and both VT criteria discharged, VA-1 and VA-2 pasted. **Three** plan gaps raised by the expansion and by execution, all closed by user decision — surfaces amended, tests colocated, `ViewId`/`Timestamp` alone with public constructors, and `Fields` permitting empty. See `## Phase sheets` | 2026-08-30 |
| PHASE-03 | **done** — `just check` exits 0 in both feature columns; 22 unit tests, 5 of them this phase's, plus a 16-file fixture corpus running in **both** columns. All four EX and all three VT criteria discharged, VA-1 and VA-2 pasted. **Five** break-and-revert runs, not the two the sheet asked for, because the fixture format makes three property claims; two of them found real defects in the runner, both fixed at the refactor step. No plan gap raised during execution — the four found at expansion were all closed beforehand. See `## Phase sheets` | 2026-09-02 |
| PHASE-04 | **sheet written, not started.** Entry criteria checked and met. **Three plan gaps found at expansion, all awaiting a user decision** — the Surfaces name no Rust under `tests/`, so the corpus has nowhere to be asserted from; VA-2 names `src/semantics/normalize.rs`, which is not the file; and VT-2's `NaN` fixture cannot be written in the inherited format, because serde_json refuses the literal at *envelope* parse. The third is settled in the sheet as implementer latitude; the first two amend `plan.md`. See `## Phase sheets` | 2026-09-02 |
| PHASE-05…10 | not started; phase sheets are written one at a time, immediately before execution. Execution order is 01…08, **10**, 09 | — |

**PHASE-03 landed** `src/semantics/schedule.rs`, `tests/protocol/runner.rs` and
16 fixtures under `tests/protocol/fixtures/schedule/`, plus one line in
`src/semantics/mod.rs` and three in `tests/protocol/main.rs`. `schedule.rs` is
two pure functions — `parse`, which reads a wire `next_check` as one instant or
one of five named `ScheduleError`s, and `resolve`, which is brief §9's three
arms. `runner.rs` is the table-driven corpus runner PHASE-04 inherits, plus this
phase's own corpus below a divider. Nothing else was touched.

**PHASE-02 landed** `src/semantics/protocol/{mod,canonical}.rs` and one line in
`src/semantics/mod.rs`. `canonical.rs` is the whole of the tier's types: six
scalars, the eight inbound types, three checked collections, `NumberRange`, and
the five outbound request types with their `Serialize`. Nothing else was touched.

**Code exists as of PHASE-01, 2026-08-29.** `src/lib.rs`,
`src/semantics/{mod,error}.rs`, `src/shell/mod.rs`,
`tests/protocol/{main,boundary}.rs`, `tests/integration/main.rs` and
`rustfmt.toml`; `just check` exits 0 in both feature columns. The rest of the
scaffolding landed outside the phase flow and PHASE-01 inherited rather than
created it: `Cargo.toml`, `clippy.toml`, `justfile`, `.gitignore`, `Cargo.lock`,
`LICENSE`. PHASE-01 amended `Cargo.toml` twice — `toml` as an optional
dependency (EX-6) and `module_name_repetitions = "allow"` (user decision).

## Handover — plan review closed, plan awaiting acceptance, 2026-08-27

<!-- Written after round 4 came back clean. The 2026-08-26 handover below is
     still the map for a *phase* agent; this one says where the slice stands. -->

### The one job

~~Ask the user to accept `plan.md`.~~ **Accepted 2026-08-29**, and PHASE-01's
sheet is written under `## Phase sheets`. **Execute PHASE-01**: one phase, one
agent, one session; set it `in progress` in the Status table first. Order is
**01…08, 10, 09**, and phase sheets stay one-at-a-time — never write PHASE-02's
until PHASE-01 is done.

Nothing is outstanding before execution. The two stale `.gitignore` notes in
`plan.md` PHASE-01 were disposed 2026-08-29 (`plan-log.md`), and `[package]`
metadata plus `LICENSE` landed the same day as user-directed scaffolding.
**Use a fresh session** — `docs/AGENTS.md` §Execute: one phase, one agent, one
session.

### What just happened

Round 4 ran fresh (gpt-5.5 via MCP, thread
`01a04244-8fe2-7630-9d38-784ed05b4fa7`, spent) and came back **clean: no
findings**. F-12, F-13 and F-14 all **confirmed** with cited evidence, and the
carve-out's *reasoning* — priority 2, never previously attacked — was found to
hold. Verdict: the plan is **executable as it stands**.

`review-plan.md` is **closed** and its **Synthesis is written**. Read that rather
than the fourteen findings; it is the closure story and it names the risks the
review knowingly leaves standing.

Four rounds, fourteen findings, all `major`, none contested, all repaired, all
confirmed. This is a stronger closure than the design's, which the user closed
with sixteen repairs unverified.

### Risks the closure leaves standing — carry these into execution

- **PHASE-06/VT-6 is confirmed on paper only.** Round 4 verified the documents
  and dependency features and did not re-run the tokio metrics probe behind the
  positive control. VT-6 has been rewritten three times. **PHASE-06 should run
  it before trusting it** — `transport-probe.local.rs` is the fastest route.
- **The self-sweep's five sites, PHASE-08's split comment and PHASE-06's
  cancellation note got no per-site verdict**, only the round's overall "no new
  findings". Weakest link in the closure.
- The design's **sixteen unverified repairs** are untouched by the plan review.
  Expect two to four residual defects, most likely `design.md` §5.4 — which the
  plan splits across PHASE-05 and PHASE-06. Verify those hardest.

### The number that should still govern how you work

**Roughly four repairs in five in this slice produced a new finding**, until the
sweep was applied deliberately.

| round | repairs checked | found defective |
|---|---|---|
| 2 | 6 | 4 |
| 3 | 5 | 2 |
| author's self-sweep after round 3 | 3 | 5 stale sites |
| 4 | 3 | **0** |

One failure mode, every time: **a repair applied at the site the finding named,
and left standing at the sites that restate the same contract.** F-13 is the
purest case — a repair made in direct response to a finding *about* unswept
restatements, itself unswept at two sites. The round 4 packet, written to hunt
that defect, then carried two of them itself.

So, in code as in documents: **after you repair anything, grep for every other
statement of the contract you just changed, before you report the repair done.**
`design.md` §9's restatement sweep names §5.5, §7 and §9's AC map. Add
`draft-spec.md` §7, `plan.md`'s Overview and Coverage table, `design-log.md`,
this file, and `justfile` — every one has held a stale restatement in this slice.

The second habit: **a criterion that names a mechanism does not yet have one.**
Two criteria in this plan were written to be falsifiable and were not —
`shutdown_timeout(ZERO)`, which cannot fail, and a line-for-line justfile match a
correct justfile fails. Both were caught by a reviewer, not by their author.

### State, as of this handover

- `plan.md` — ten phases, order **01…08, 10, 09**. Drafted, **not accepted**.
- `review-plan.md` — **closed**, 14 findings across 4 rounds, Synthesis written.
- Two user decisions of 2026-08-27 in `design-log.md`: D53 amended, `just`
  adopted as canonical runner. Both swept.
- One **author** decision under them, also in `design-log.md`: the tests
  carve-out in `clippy.toml`. Reversible; its reasoning is now reviewed and
  holds. Reverting costs an `#[expect]` per asserting test.
- `canon-delta.md` — CD-1 and CD-2 still await user endorsement, at audit.
- Working tree: modified files, uncommitted. Nothing staged for a reason.

### Things that are settled and should not be re-litigated

- `just` **is** in `flake.nix` `devToolPkgs` (since `6489521`) and `deno` is in
  `projectPkgs` (since `b76b75c`). Both verified as store paths in a fresh
  `nix develop`, 2026-08-27. Earlier notes claiming otherwise were observing a
  **stale shell**. PHASE-01/VH-1 covers the reload and is not optional.
- `just -n check` prints §9's six commands in §9's order. Checked.
- The four `allow-*-in-tests` keys are accepted by this toolchain and do silence
  the five errors ordinary test code otherwise produces. Measured, twice.

## Handover — design accepted, plan drafted, 2026-08-26

<!-- Written for a planner. Kept because the material below is still what a
     phase agent needs; superseded only where the plan now answers it. Steps 1-3
     of "Do next" are done — `plan.md` exists. -->

### Where this is

**Design is accepted and the review is closed.** `review-design.md` holds 63
findings across five rounds and its **Synthesis** is written — read that first,
it is the closure story and it names the risks the review knowingly left
standing. `design-log.md` holds the user decisions, dated, each against a finding
id. This section is the map for a planner, not a summary of either.

`plan.md` is now written — ten phases (01…08, 10, 09), coverage complete, not
yet accepted.
`docs/AGENTS.md` §Plan is the process and it is binding: research first if `research.md` has gaps,
then read the design closely and **trace its dependencies against the code** —
except there is no code, so every trace lands on a design claim instead, which is
the next section.

### The one thing to know before you plan

The review was closed **by user decision, not by reaching done**. Sixteen repairs
— F-48…F-63 — are `repaired` with no independent confirmation. All seven blockers
were repaired, so nothing is outstanding by severity; what is missing is a second
opinion on the last two rounds of work.

From the ledger's own base rate, roughly 0.2 defects per repair and falling,
expect **two to four defects still resident**. The likeliest location is
`design.md` §5.4, which has been restructured three times in three rounds —
F-49 deleted the spawn, F-53 collapsed the graces, F-59 moved the wait back
inside the timeout — and whose last restructure nothing has reviewed.

This is not a reason to stop; it is a reason to plan §5.4 as the phase that gets
the most verification, and to expect it. `docs/AGENTS.md` is explicit that
unresolved design issues surfacing during planning go **back to design** rather
than getting quietly repaired in the plan. If you find one, that is the process
working, not you exceeding your brief.

### Do next, in order

1. **Read `review-design.md`'s Synthesis**, then `design.md` §5 end to end. Skip
   the findings themselves unless something looks wrong — the Synthesis exists so
   you do not have to read 63 of them.
2. **Check `research.md` for gaps** against what the plan needs. It covers Slint,
   the tokio-vs-smol dependency count, and the deno decision. It does not cover
   anything about test harness shape or fixture-corpus mechanics.
3. ~~**Write `plan.md`.**~~ Done 2026-08-26 — nine phases, since split to ten. Two design gaps
   surfaced while writing it, A-P1 and A-P2, and are with the user.
4. ~~**Offer the user adversarial review of the plan.**~~ Done — accepted, run
   over four rounds, closed clean 2026-08-27. See `review-plan.md`'s Synthesis.
5. **Ask the user to accept the plan.** Then phase sheets, one at a time,
   immediately before each phase — never up front. **Still outstanding.**

### What planning must not re-open

These are **user decisions**, each recorded in `design-log.md` against a finding
id. Reversing one needs the user, not an argument.

| decision | id |
|---|---|
| Bounded/drained pipes done in-slice, not deferred | reversal of D18/D19 |
| P1 enforced, and scoped to values the host *interprets* | D31, F-9 |
| Wire hints are flat only — not both spellings | D37, F-38 |
| `BoundsError::NotFinite` kept as a constructor guard | D39, F-36 |
| Kind-inapplicable keys rejected, not treated as hints | D43, F-45 |
| Cleanup is a second dimension, not an error precedence rule | D47, F-48 |
| tokio optional behind a `shell` feature, not a workspace split now | D49, F-51 |
| Explicit `null` ≡ omission, except `view` | D50, F-50 |
| A choice field's options are id + label only | D46, F-54 |
| Cancellation states the narrow claim; AC-5 narrowed rather than made true | D54, F-60 |
| Design accepted with 16 repairs unverified; review closed | 2026-08-26 |

### Design facts a plan has to respect

Not decisions — consequences. Each one constrains phase ordering or surfaces.

- **The `shell` feature gate is structural, not a flag.** `shell/` is
  `#[cfg(feature = "shell")]`, the integration test target declares
  `required-features = ["shell"]`, and `autotests = false`. A phase that adds a
  test target without that plumbing breaks AC-15's build gate — which is the
  mechanical form of a binding ADR, so it breaks canon compliance, not just CI.
- **Six verification commands, two feature columns.** `design.md` §9. A phase
  that ends green in one column is not green.
- **The no-panic lints are module-level `#![deny(...)]` attributes**, not a
  manifest `[lints]` section and not `-D warnings` (F-62). Whichever phase first
  creates a module handling backend-derived data owns putting them there.
- **`src/semantics/` may not name `crate::shell`, `crate::bin` or `tokio`** — I2,
  AC-15's grep. It fails vacuously if it finds no files, so it needs files.
- **Fixtures are data files walked by a table-driven runner**, not Rust literals
  (§9). That shapes the protocol tier's first phase considerably.
- **Stratum 3 is declared and empty.** No binary this slice. The integration
  tests reach the crate only through its public API, which is deliberate.

### Established empirically — do not re-derive

Each was checked by building and running it, and each changed the design. **Do
not re-derive these; do not take them on trust either where a phase depends on
one — re-run it.** The probe source is preserved at `transport-probe.local.rs`
with `transport-probe-Cargo.local.toml`: copy both into a scratch crate
(`src/main.rs` + `Cargo.toml`) and `cargo run`. It carries seven cases, A–G, and
is the fastest way to check any change to §5.4's structure before writing tests
for it. **Promoting it to a tracked spike is a user call, not an agent's** — but
PHASE-0x for the process transport should probably start by running it.

- `#[serde(default)] Option<Option<T>>` returns `None` for **both** `{}` and
  `{"view":null}`. The distinction needs a `deserialize_with` helper. (F-5)
- For a plain `Option<T>`, `{"x":null}` and `{}` are likewise indistinguishable —
  which is why D50 is a rule and not a mechanism. (F-50)
- serde_json rejects `NaN` and `1e400` before any bounds check runs, so
  `BoundsError::NotFinite` is unreachable from the wire. (F-36)
- jiff 0.2.35: `SpanRelativeTo::days_are_24_hours()` resolves days and weeks
  exactly and rejects months and years cleanly with no tzdb. (F-10)
- tokio `start_kill` and `wait` both return `Ok` against an already-exited,
  already-waited child — tokio caches the status, so cleanup is idempotent for
  free. (F-48)
- A `select!` **sub-future** is dropped the instant its parent is cancelled; a
  `tokio::spawn`ed task is not, and is still running 100 ms later. This is F-49's
  entire substance and the reason the drain is not a task.
- The §5.4 structure drains 4000 stderr lines past the 64 KiB pipe buffer while
  reading stdout, with no deadlock.
- **The two grandchild cases differ, and the design described one while measuring
  the other for four rounds (F-63).** A grandchild holding *stderr only*
  (`(sleep 30) >/dev/null &`) → `Ok(response)` + `cleanup: TimedOut`, 303 ms. A
  grandchild holding *stdout too* (`(sleep 30) &`) → `Err(Timeout)` **and**
  `cleanup: TimedOut`, 902 ms, because stdout never reaches EOF. The child exits
  and is reaped in both, which is what makes `Orphaned` the wrong name.
- **A non-zero exit after a valid response yields `ExitStatus { code: Some(1) }`
  with the body discarded** — but only if the status is read inside the timed
  region. Cleanup kills before it waits, so a status observed there does not
  exist (F-59). The success path costs 2.5 ms, so the cleanup budget is not paid
  when nothing goes wrong.
- **`body` holding `&mut child` needs an inner scope**, or the cleanup budget
  cannot take the borrow back. Compiles as written in §5.4.
- Feature gate: `cargo tree --no-default-features` contains no tokio node and
  `cargo test --no-default-features` compiles and runs. (F-51)

### What the review taught, worth carrying into execution

Not review technique — three habits that caught real defects and will catch them
in code too.

1. **A claim is held by a mechanism or it is not held.** Three separate findings
   (F-51, F-57, F-62) were claims that read as green and were enforced by
   nothing: canon no build checked, a build gate whose command could not run, an
   invariant resting on lints that were off by default. When a phase says
   "verified by", check that the thing named actually runs and actually fails.
2. **Execute any claim that can be executed.** Seven probe cases, five of which
   changed the design. Its blind spot, learned the hard way at F-63: running one
   case and describing another. A measurement is evidence for the sentence next
   to it only if that sentence names the case that was run.
3. **Three repairs to one thing is evidence about the thing.** F-27, F-40 and
   F-49 were one `tokio::spawn` mistake; the fix was a deletion. F-52, F-54,
   F-58 and F-61 were one identifier mistake. If a phase needs its third patch in
   the same place, stop and look at the decision underneath it.

### Working constraints carried over

- **Ask the user before using the codex/GPT MCP** — separate billing. Rounds 4
  and 5 each used a **fresh** reviewer with no thread history, and both reached
  past what the accumulating thread of rounds 1–3 had managed. If the plan gets
  a review, do the same. Round 5's thread was
  `01a03af5-bc55-79a1-a216-ff9c7e7ee4e1`; treat it as spent.
- **No code without an accepted plan** (`CLAUDE.md`). The plan is not accepted
  until the user says so.
- **Never `git stash`.** Commit only when asked.
- Canon (`docs/specs/`, `docs/policy/`, `docs/adr/`) is amended only with
  explicit user endorsement, and `canon-delta.md` holds CD-1 and CD-2 awaiting
  exactly that.

### The review packet, if a further round is ever wanted

Deleted as stale — it was a concatenation and is regenerable in one command:

```bash
{ cat docs/slices/001/review-packet-instructions.local.md
  for p in brief.md slices/001/slice-001.md slices/001/design.md slices/001/draft-spec.md; do
    printf '\n\n---\n\n# DOCUMENT — `docs/%s`\n\n---\n\n' "$p"; cat "docs/$p"
  done
} > docs/slices/001/review-roundN-packet.local.md
```

`review-packet-instructions.local.md` is kept — it carries the reviewer brief,
the finding format and a finding index, and it opens with a STALE banner saying
what to fix before reuse: the index stops at F-58, and the priorities still tell
a reviewer to verify F-48…F-58, which round 5 already did. `*.local.*` is
gitignored.

## Found while planning — 2026-08-26

Two things `docs/AGENTS.md` §Plan calls for going back to design over. One was a
real gap and is now a user decision; the other was a wrong premise and was
withdrawn after measurement. Both are in `plan-log.md`.

### The TOML parser was unnamed — settled

`design.md` §5.2 specifies a TOML configuration file and a parsed `Config`, and
§5.1's manifest lists serde, serde_json, jiff and tokio. Nothing named a parser,
and adding one is a dependency addition — `docs/AGENTS.md` §Execute says stop and
consult, and ADR-002's Verification section requires the triggers be checked and
recorded.

**Decided:** `toml`, optional, inside the `shell` feature, exactly as tokio is.
T1 does not fire, for D49's reason and by D49's mechanism. `design.md` §5.1 and
§3 are each a line short; that is audit's reconciliation to make, not a mid-slice
edit.

### The suspected build-gate defect was a wrong premise — withdrawn

Raised as the same class as F-51: the integration tier must drive an async API
from a `#[test]`, Cargo forbids optional dev-dependencies, so tokio looked to
need an unconditional `[dev-dependencies]` entry — which would compile tokio in
the `--no-default-features` column and make three statements false (`design.md`
§5.1's "serde, serde_json and jiff and nothing else", §9's "contains no tokio
node", and CD-1's proposed ADR-001 text, which repeats the `cargo tree` line into
canon).

**The premise was wrong.** A test target has the package's **regular**
dependencies in scope, optional ones included, whenever the feature enabling
them is on. No dev-dependency is needed. Measured on a manifest with no
`[dev-dependencies]` section at all:

- Default column: `#[tokio::test]` in `tests/integration/main.rs` compiles and
  runs; the `protocol` target runs too.
- `--no-default-features`: `tests/protocol/main.rs` naming `tokio` fails with
  `error[E0433]: cannot find module or crate 'tokio'`. The `integration` target
  is skipped, not built.
- `cargo tree --no-default-features` is clean with no `-e` filter.

So `design.md` and CD-1 are correct as written and no canon text changes.

**Carry this into execution — it is stronger than the design claimed.** The
stratum 1 test tier cannot be tested *with* an async runtime even by accident,
because the test target cannot name one. That is Cargo's own resolution rather
than review or a grep, so AC-15's boundary test does **not** need extending to
cover `tests/protocol/`. It also means a later `[dev-dependencies] tokio` entry —
added for convenience by someone who does not know this — would silently put a
runtime back in reach of the stratum 1 tier and weaken the gate without failing
anything. PHASE-01's implementer notes say so.

Reproduce in about a minute: a crate with `tokio` optional behind a `shell`
feature, `autotests = false`, two `[[test]]` targets with the integration one
carrying `required-features = ["shell"]`, then add a `tokio::` token to
`tests/protocol/main.rs` and run both columns.

## Open against the design — raised 2026-08-26, **both dispositioned 2026-08-27**

Scaffolding landed outside the phase flow (`Cargo.toml`, `clippy.toml`,
`justfile`, `.gitignore`, `Cargo.lock`; commits `b76b75c`, `4fc8637`, `dda6bf2`)
and `design.md` §9 was amended with it. Two things were open. Both are now user
decisions, recorded in `design-log.md` under 2026-08-27.

### D53 — amended to the crate-wide/per-module split. Closed.

`unwrap_used`, `expect_used` and `indexing_slicing` are crate-wide denies in
`[lints.clippy]`; `arithmetic_side_effects` stays module-level. The four sites
that still stated the superseded form — `design.md` §5.5's I9 row, §7's D53, the
§9 paragraph that contradicted the supersession note four lines above it, §9's
AC-6 row, and `draft-spec.md` §7's R-46 row — are swept. `plan.md`'s Overview
item 4 no longer reports a divergence. Reasoning in `design-log.md`.

### `just` — adopted as the canonical runner. Closed.

Recipes: `build`, `test`, `test-stratum1`, `lint`, `fmt-check`, `check` (the
gate, and `default`), `fmt` (outside it). `design.md` §9's block stays canonical
and the recipes mirror it; `just -n check` prints the list, and it must be the
same commands with the same arguments in the same order — **not** the same
characters, since §9 carries comments and a line continuation `just -n` does not
print. That is PHASE-01/VA-3, and the literal-match wording was review finding
F-13.

**The claim that `just` was missing from `flake.nix` was false.** It has been in
`devToolPkgs` since commit `6489521`; the justfile's own header said otherwise
and this file repeated it. What the claim was actually observing is a *stale
shell*: a session entered before a `flake.nix` change does not see it, which is
true of `deno` too. Checked with `nix develop --command`: `just` 1.58.0 and
`deno` 2.9.4, both store paths. No `flake.nix` change was needed and AC-1 holds.

PHASE-01/EX-1 and every phase's VA-1 are now `just check`. PHASE-09/EX-1 has
`AGENTS.md` naming the recipes (AC-10) and its VA-1 runs the gate from a clean
clone entered through `nix develop`.

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Kept in place after the phase is done. -->

### PHASE-01 — Crate skeleton, the build gate, and the stratum 1 error taxonomy

**State:** sheet written 2026-08-29 · **done** 2026-08-29 — `just check` exits 0
**Plan entry:** `docs/slices/001/plan.md:184`
**Surfaces (from the plan, nothing added):** `Cargo.toml`, `clippy.toml`,
`rustfmt.toml`, `justfile`, `.gitignore`, `src/lib.rs`, `src/semantics/mod.rs`,
`src/semantics/error.rs`, `src/shell/mod.rs`, `tests/protocol/main.rs`,
`tests/protocol/boundary.rs`, `tests/integration/main.rs`, `flake.nix`.

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | plan accepted by the user | **met** — the user asked for this sheet on 2026-08-29, which is acceptance of `plan.md`; recorded in `plan-log.md` |
| EN-2 | none | met vacuously |

#### What already exists — inspected 2026-08-29, not taken from the plan

The plan was written before some of this scaffolding landed. Checked against the
working tree, not against the plan's description of it:

| path | state | consequence for this phase |
|---|---|---|
| `Cargo.toml` | **exists**, commit `472e4d3`; `[package]` metadata added 2026-08-29 (`license = "MIT"`, `repository`, `readme`, `keywords`, `categories`) on user instruction, lifted from `~/dev/doctrine`. Carries `autotests = false`; both `[[test]]` targets declared with explicit paths, `integration` with `required-features = ["shell"]`; `[dependencies]` matches `design.md:371`'s snippet exactly; `[lints.rust]` and `[lints.clippy]` populated, with `unwrap_used`/`expect_used`/`indexing_slicing` crate-wide | VA-2 is a **read** of an existing file, and it passes as written. EX-6 is the only manifest edit this phase owes |
| `clippy.toml` | **exists**, `472e4d3` + the carve-out. All four `allow-*-in-tests` keys present | no change |
| `justfile` | **exists**, `472e4d3` + the §9 mirror. Six commands, §9's order | no change; VA-3 verifies it |
| `.gitignore` | **exists**, commit `4fc8637`, holding `*.local.md` and `target/` | **the plan says it does not exist.** See below |
| `LICENSE` | **added 2026-08-29** — MIT, © 2026 David Lee | outside the plan's declared surfaces; a user-directed scaffolding change, not phase work. Recorded in `plan-log.md` |
| `rustfmt.toml` | **missing** | this phase creates it. `tab_spaces = 2` |
| `Cargo.lock` | exists, `dda6bf2` | inherited |
| `src/`, `tests/` | **empty directories** | every file is new. Cargo cannot parse a `[[test]]` whose `path` is absent, so **no cargo command runs at all** until both `main.rs` files exist. That orders task 1 |
| `flake.nix` | `just` in `devToolPkgs:42`, `deno` in `projectPkgs:53` | EX-5 already discharged; confirm in-shell only (VH-1) |

#### Two stale implementer notes in `plan.md` — raised, and now closed

Found while expanding this phase, both in PHASE-01's *Notes for the implementer*.
Neither was a criterion, so neither blocked. Both were the slice's own recurring
defect: a statement of current state left standing after the state changed.

1. **"`.gitignore` does not exist yet."** False since commit `4fc8637`.
2. **"`*.local.*` is already covered by the user's global ignore file, so do not
   duplicate it."** The global file does cover it —
   `~/.gitignore_global:10`, confirmed with `git check-ignore -v` — but the
   repository `.gitignore` already duplicated it **narrower**, as `*.local.md`,
   so `*.local.rs` and `*.local.toml` (the transport probe's two files) rested on
   one machine's config.

**Disposed 2026-08-29, by user decision.** `.gitignore` widened to `*.local.*`
with the reason written at the site; no tracked file matched, checked before the
change; all three probe and packet files now resolve to `.gitignore:4` rather
than to the global file. `plan.md`'s note is rewritten to describe the tree as it
is. Recorded in `plan-log.md`.

#### Reading list

Read before writing anything. `path:line`.

| what | where | why |
|---|---|---|
| the phase itself | `docs/slices/001/plan.md:184` | criteria are binding as written |
| **the manifest contract** | `docs/slices/001/design.md:371` | dependency names and features; the prose at `:394` states `required-features`, `autotests` and why the gate needs them — VA-2's three load-bearing facts are in the prose, not the code block |
| **the verification block** | `docs/slices/001/design.md:1910` | canonical. EX-1 and VA-3 both compare against it. Take the second clippy line from here, never from memory |
| the error taxonomy | `docs/slices/001/design.md:885` | EX-2 transcribes it. Every variant, no additions |
| why `NotFinite` stays | `docs/slices/001/design.md:781` | D39. It is unreachable from JSON and kept anyway; do not "tidy" it |
| I9 | `docs/slices/001/design.md:1662` | what the lints are actually holding |
| D53 as amended | `docs/slices/001/design.md:1890` | crate-wide three / per-module `arithmetic_side_effects` |
| R-46 | `docs/slices/001/draft-spec.md:387` | the spec's form of the same, including the tests carve-out |
| AC-1 | `docs/slices/001/slice-001.md:73` | clean clone, dev shell, zero warnings |
| AC-11 | `docs/slices/001/slice-001.md:147` | the vocabulary list VT-2 greps for |
| AC-15 | `docs/slices/001/slice-001.md:167` | two parts, different strength — the grep and the build gate |
| AC-10 | `docs/slices/001/slice-001.md:142` | read only to know it is **not** yours: root `AGENTS.md` is PHASE-09's |
| the strata rule | `docs/adr/001-one-way-strata.md` | binding canon; EX-3 is its mechanical form |
| the crate rule | `docs/adr/002-single-crate-until-triggered.md` | binding canon; T1 does not fire here |
| the tokio dev-dependency finding | `docs/slices/001/plan-log.md:33` | measured. Explains why there is no `[dev-dependencies]` section and why adding one is a regression |
| prior art | none — `src/` and `tests/` are empty | |

#### Assumptions

Each is checkable; check it rather than proceeding on it.

- **A1** — the dev shell in use was entered after `6489521` and `b76b75c`. If not,
  `just` and `deno` resolve from the user's profile or not at all. VH-1 settles it
  and it is the first thing to do, not the last.
- **A2** — `cargo 1.99.0-beta.1` from `rust-bin.beta.latest.default`
  (`flake.nix:39`, `research.md`). Do not pin backwards: `design.md` §5.2's D11
  relies on AFIT.
- **A3** — `edition = "2024"` as already committed in `Cargo.toml`.
- **A4** — no module created in this phase handles backend-derived data **at run
  time**. `semantics/error.rs` declares error types; it parses nothing and does no
  arithmetic. So no module-level `#![deny(clippy::arithmetic_side_effects)]`
  lands this phase, and PHASE-02 is the first to owe one. The crate-wide three
  apply from the moment `[lints.clippy]` is in force, which is already.
  **Re-check this before ending the phase** — if `error.rs` acquires so much as a
  `From<serde_json::Error>` that inspects a value, the obligation moves.

#### STOP conditions

Stop and consult the user; do not improvise past any of these.

- ~~The `cargo` clippy group / `cargo_common_metadata`.~~ **Withdrawn 2026-08-29,
  by measurement — it cannot fire here.** `publish = false` silences
  `cargo_common_metadata` outright: a scratch crate carrying `cargo = "deny"` and
  no metadata at all passes `cargo clippy --all-targets -- -D warnings` with
  `publish = false` present, and fails with five errors (`license`, `repository`,
  `readme`, `keywords`, `categories`) the moment it is removed. The metadata was
  added anyway, on the user's instruction, and a **positive control** confirms it
  is complete on the lint's own terms rather than merely hidden: the same probe
  with goad's `[package]` block and **no** `publish = false` passes clean. If a
  *different* `cargo`-group lint ever fires, the STOP still stands — a lint
  carve-out is `design.md` §9's business, not this phase's.
- Any temptation to add `[dev-dependencies]`. It is unnecessary — a test target
  already sees the package's optional dependencies when their feature is on
  (`plan-log.md:33`, measured) — and an unconditional `tokio` entry there would
  put a runtime back in reach of the stratum 1 test target, silently, without
  failing anything. That is the one property this column exists to prevent.
- Any variant added to or removed from the §5.2 enums, including "obviously
  unreachable" ones. `NotFinite` is D39, a user decision.
- Any change to `design.md` §9's command block, or to the `justfile` recipes that
  mirror it. §9 changes first, by user decision, then the recipes.
- Root `AGENTS.md` content. AC-10 is PHASE-09's, and the commands it must name
  are what this phase is establishing.
- Anything outside the declared surfaces above.

#### Tasks

Red / green / **refactor**. The refactor step is not optional.

1. **VH-1 first, not last.** `nix develop`, then `deno --version` and
   `just --version`; confirm both resolve to store paths. A stale shell has
   already produced one false claim in this slice (`notes.md` Status). Paste the
   output below.
2. **Make cargo able to run at all.** `src/lib.rs` with
   `pub mod semantics;` and `#[cfg(feature = "shell")] pub mod shell;`
   (`design.md:394`), empty `src/semantics/mod.rs` and `src/shell/mod.rs`,
   `tests/protocol/main.rs` and `tests/integration/main.rs`. The integration
   `main.rs` may be an empty module until PHASE-05. Nothing else — no error types
   yet.
3. **`rustfmt.toml`**, `tab_spaces = 2`. Before there is code to churn: without
   it `cargo fmt` reformats to four and every snippet in `design.md` stops being
   copy-able. `CLAUDE.md` asks for two.
4. **RED — VT-1's vacuity guard first.** Write the empty-directory case before
   the real grep: point the helper at a directory with no files and assert it
   **fails**. A guard written after the passing case is a guard written to agree
   with it.
5. **RED/GREEN — VT-1.** No file under `src/semantics/` contains `crate::shell`,
   `crate::bin` or `tokio`. Green against the modules from task 2.
6. **RED/GREEN — VT-2.** No file under `src/` contains habit, streak, journal,
   site, goal, reminder, compliance (AC-11, `slice-001.md:147`), with the same
   empty-directory guard.
7. **REFACTOR** — the two greps are one walk-and-match helper with two
   configurations, not two copies. `boundary.rs` is where the duplication would
   be cheapest to leave and most expensive to keep, since PHASE-09's sweep
   extends it.
8. **RED — VT-3.** A case per `ScheduleError`, `BoundsError` and `ProtocolError`
   variant asserting its `Display` names the value it carries. Write these
   against the taxonomy at `design.md:885` before `error.rs` exists.
9. **GREEN — EX-2.** `src/semantics/error.rs`: the three enums exactly as §5.2
   lists them, with `Display` and `std::error::Error`. `#[derive(Debug)]` on each
   — `missing_debug_implementations = "deny"` is on.
10. **EX-6.** `toml` into `[dependencies]` as `optional = true`, pulled in by the
    `shell` feature exactly as tokio is. Then confirm `cargo tree
    --no-default-features` shows neither it nor tokio. It is an unused dependency
    for six phases and that costs nothing — `unused_crate_dependencies` is
    deliberately off (`Cargo.toml`, commented with the reason).
11. **EX-3, all three parts, by observation.** (a) `cargo tree
    --no-default-features` — no tokio node. (b) `cargo test
    --no-default-features` **skips** the `integration` target rather than failing
    to build it. (c) add a `tokio` token temporarily to a `src/semantics/` file
    **or** to `tests/protocol/`, confirm that column fails with `E0433`, revert.
    Break-and-revert, not assertion. Paste all three.
12. **VA-2.** Read `Cargo.toml` and confirm `autotests = false`, both `[[test]]`
    targets with explicit paths, and `required-features = ["shell"]` on
    `integration`. Already true; confirm rather than write.
13. **VA-3.** `just -n check` against `design.md:1913`'s block. Compare the
    **command sequence** — same commands, same arguments, same order — not the
    characters: §9 carries inline comments and wraps the second clippy line, and
    `just -n` prints neither, so a correct justfile fails a literal comparison
    (F-9, F-13). Paste both.
14. **EX-1 / VA-1.** `just check` exits 0. Paste the output. Not "they should
    pass".
15. **Bookkeeping before handing off** — Status table to `done`, this sheet kept
    current as you go, `## Harvest` updated in place.

#### Verification record

Filled in during execution, not after. Empty until then.

| id | mode | result | evidence |
|---|---|---|---|
| VH-1 | human | **pass, both halves.** Agent's in a fresh shell 2026-08-29 (A1 was false); the user's own interactive shell 2026-08-30 | Log, 2026-08-29 VH-1 and 2026-08-30 VH-1 — human half |
| VT-1 | test | **pass**, and seen to fail | `boundary.rs::stratum_1_names_neither_the_shell_a_binary_nor_the_runtime`; red by planting `crate::shell` — Log, VT-1/VT-2 |
| VT-2 | test | **pass**, and seen to fail | `boundary.rs::no_host_source_file_names_the_user_s_domain`; red by planting `habit` — Log, VT-1/VT-2 |
| VT-3 | test | **pass**, and seen to fail twice over | four cases in `error.rs`; red once as a missing type, then by break-and-revert on both gates — Log, VT-3 |
| VA-1 | agent | **pass** | `just check` exits 0; full output pasted in the Log under *the gate* |
| VA-2 | agent | **pass** — read, not written | `Cargo.toml:19` `autotests = false`; `:44–46` protocol with explicit path; `:48–51` integration with path and `required-features = ["shell"]` |
| VA-3 | agent | **pass** — same six commands, same arguments, same order | both blocks pasted in the Log, VA-3 |
| EX-1 | — | **pass** — six of six, both feature columns | Log, *the gate*. Unblocked by the user's decision on `module_name_repetitions` |
| EX-2 | — | **pass** | `src/semantics/error.rs`; 12 + 2 + 5 variants, `Display` and `std::error::Error`, `NotFinite` kept per D39 |
| EX-3 | — | **pass**, all three parts by observation | Log, EX-3 — tree, skip, and break-and-revert at two sites |
| EX-4 | — | **pass** | `tests/protocol/boundary.rs`; both greps, both with the vacuity guard, guard written first |
| EX-5 | — | **pass** — discharged 2026-08-26 (`b76b75c`), confirmed in-shell | Log, VH-1: `deno` at `/nix/store/pn1qbka…-deno-2.9.4/bin/deno` |
| EX-6 | — | **pass** | `toml v1.1.4`, `optional = true`, in `shell` via `dep:`; stratum 1 tree carries neither it nor tokio — Log, EX-6 |

#### Log

<!-- Append as you go: decisions taken, obstacles, anything noticed in passing.
     Do not save the bookkeeping for the end; it will be lost. -->

- 2026-08-29 — sheet written. Entry criteria checked. Two stale implementer
  notes in `plan.md` raised for the user (`.gitignore`); no criterion affected.
- 2026-08-29 — `[package]` metadata added and `LICENSE` written, on user
  instruction; fields lifted from `~/dev/doctrine`, licence MIT, repository
  `davidlee/goad`. The anticipated `cargo_common_metadata` obstacle was
  **wrong and is withdrawn** — `publish = false` silences that lint on its own.
  Measured both directions plus a positive control; see the STOP list above.

- 2026-08-29 — **VH-1, run first. A1 was false and the sheet was right to
  distrust it.** The shell this phase's agent inherited was entered before
  `6489521`/`b76b75c`: `just` resolved from `/home/david/.nix-profile/bin/just`
  — the user's profile, not the flake — and `deno` did not resolve at all. Had
  VH-1 been left to the end, every command in this phase would have run under
  the wrong toolchain and the phase would have closed on a false claim, which is
  the exact failure `notes.md` Status already records once.

  ```
  $ echo "IN_NIX_SHELL=$IN_NIX_SHELL"; command -v just deno cargo; just --version; deno --version
  IN_NIX_SHELL=impure
  /home/david/.nix-profile/bin/just
  /nix/store/cyn97lq74y3lx15y95gyzplnmmx451g9-rust-default-1.99.0-beta.1-2026-08-18/bin/cargo
  just 1.58.0
  bash: line 1: deno: command not found
  ```

  The **flake is correct**; only the shell was stale. Entered fresh, both
  resolve to store paths:

  ```
  $ nix develop --command bash -c 'command -v just deno cargo rustc; just --version; deno --version; cargo --version'
  /nix/store/ni2dxycnhsp34y4qy6q44nw6pp6bj0l0-just-1.58.0/bin/just
  /nix/store/pn1qbka1qfxw0wfbh1scsd2gvhv0dhj2-deno-2.9.4/bin/deno
  /nix/store/cyn97lq74y3lx15y95gyzplnmmx451g9-rust-default-1.99.0-beta.1-2026-08-18/bin/cargo
  /nix/store/cyn97lq74y3lx15y95gyzplnmmx451g9-rust-default-1.99.0-beta.1-2026-08-18/bin/rustc
  just 1.58.0
  deno 2.9.4 (stable, release, x86_64-unknown-linux-gnu)
  cargo 1.99.0-beta.1 (eb98b54bc 2026-08-11)
  ```

  EX-5 confirmed in-shell, as the plan asked. A2 confirmed at the same time:
  `cargo 1.99.0-beta.1`, matching `flake.nix:39`.

  **Consequence, and it is a working rule for the rest of this phase:** every
  command below is run as `nix develop --command bash -c '…'`. An agent's shell
  is not the user's and cannot be reloaded in place; wrapping each command is
  the only way to be sure which toolchain answered. **The human half of VH-1 is
  outstanding** — the user's own interactive shell is still the stale one and
  needs reloading before they run anything by hand.

- 2026-08-29 — tasks 2, 3 done. `src/lib.rs`, `src/semantics/mod.rs`,
  `src/shell/mod.rs`, `tests/protocol/main.rs`, `tests/integration/main.rs`,
  `rustfmt.toml` (`tab_spaces = 2`). `cargo build` runs for the first time.
  `src/` and `tests/` were empty *directories* with no subdirectories, so the
  `mkdir -p` is part of the task; the phase sheet's "every file is new" is
  accurate but understates it.

- 2026-08-29 — **VT-1, VT-2 and the vacuity guard: red observed, then green.**
  Both reds were made by planting, not by absence of code, so each assertion has
  been seen to fail for its own reason.

  *The guard's red was the instructive one.* Written against a walk with no
  `inspected == 0` check, the no-Rust-files case returned **`Ok(0)`** — the
  vacuous pass, exactly as specified. The renamed-directory case failed too, but
  only incidentally, via `Unreadable`; had the guard been written after the
  passing case, that incidental failure would have been mistaken for the guard
  working. This is why the sheet put task 4 before task 5.

  ```
  ---- a_scan_that_inspects_no_rust_files_fails ----
  a scan inspecting nothing must fail: 0
  ---- a_scan_whose_directory_was_renamed_away_fails ----
  expected a vacuity breach, got:
  …/src/semantics-renamed: could not be read, so was not inspected: No such file or directory (os error 2)
  ```

  With the guard added, both pass. Then VT-1 and VT-2, red by planting
  `use crate::shell::Backend;` in a comment in `semantics/mod.rs` and `habit` in
  a comment in `lib.rs`:

  ```
  …/src/semantics/mod.rs:4: forbidden token `crate::shell`
  …/src/lib.rs:13: forbidden token `habit`
  ```

  Plants reverted (`git diff --stat src/` clean of them), all four green.

- 2026-08-29 — **two decisions taken inside `boundary.rs`, both narrowing.**
  Neither is a design change; both make the criterion mean what it says.
  1. **Matching is case-insensitive.** AC-11 is about *type names*, which are
     `CamelCase`; a literal lower-case `contains` would miss `Habit` entirely
     and the check would be theatre. Tokens are held lower-case and each line is
     lowered before comparison.
  2. **Substring, not word-boundary.** `site` will match `composite` if one ever
     appears. That over-approximates deliberately: a false positive costs a
     rename, a false negative costs the invariant. If it ever fires falsely that
     is a decision to take then, not a loophole to pre-cut now.

  Also: `run` reports **every** breach rather than the first, and cannot return
  `Ok(0)` — so arriving at `Ok` *is* the guard discharging, which is why
  `assert_clean` needs no count assertion. Failure text goes through `Display`
  rather than `{:?}`: `clippy::use_debug` is `deny` and has no test carve-out in
  `clippy.toml`, so `{:?}` in a test would fail the gate.

- 2026-08-29 — task 7 (refactor) is satisfied by construction rather than by a
  later pass: one `Scan { root, forbidden }` walk with four configurations, one
  `assert_clean`. PHASE-09 extends it by adding a `Scan`, not a walk.

- 2026-08-29 — **`clippy::tests_outside_test_module` applies to `tests/`
  targets, not only to unit tests.** It is `deny` in `Cargo.toml`, so the four
  `#[test]` functions in `boundary.rs` failed the first clippy column outright.
  Resolved by **complying, not carving out**: `main.rs` declares
  `#[cfg(test)] mod boundary;`, and clippy reads the `cfg(test)` on the
  declaration as marking the module. A `tests/` target is always built with
  `--test`, so the attribute never actually switches anything off — verified,
  all four tests still run in both columns. No lint change was needed, which
  keeps §9 out of it.

- 2026-08-29 — **VT-3 placement: `src/semantics/error.rs`, in a
  `#[cfg(test)] mod tests`.** The natural home was a new
  `tests/protocol/errors.rs`, and that is **outside the phase's declared
  surfaces** — a STOP. Of the two surfaces that could hold it,
  `tests/protocol/main.rs` is the target root that PHASE-02 will fill with
  protocol fixtures, so putting the taxonomy's `Display` cases there is
  borrowing a room. `error.rs` is named in the surfaces, is cohesive with the
  types, satisfies `tests_outside_test_module`, and runs in both columns.

- 2026-08-29 — **VT-3: red, green, and then proved to bite.** Written before the
  types existed, so the first red was `E0432: unresolved imports
  super::BoundsError, super::ProtocolError, super::ScheduleError`. A compile
  error is a weak red, so both of the test's gates were then broken and
  reverted:

  1. **Display stops naming its value.** `MissingField`'s arm rewritten to
     `write!(f, "missing a required field")`:
     ```
     `missing a required field` never names the `protocol_version` it carries
     ```
  2. **A variant is added with no case.** `AddedWithoutACase { unformatted }`
     appended to `ProtocolError`:
     ```
     error[E0004]: non-exhaustive patterns: `&ProtocolError::AddedWithoutACase { .. }` not covered
     ```
     Twice — once for the test's `must_name` table and once for the `Display`
     impl itself.

  That second gate is the one worth keeping. VT-3's stated purpose is "what
  stops a variant being declared with a field nothing ever formats"; a table of
  hand-written cases cannot do that on its own, so `must_name` is a
  **wildcard-free match** and a new variant fails to compile until it has an
  arm. **Residual gap, stated rather than hidden:** a variant could still gain a
  `must_name` arm and no *instance* in `every_protocol_error()`. Nothing in the
  type system catches that; it is a review point, and it is the reason the three
  instance lists are ordered exactly as §5.2 lists the variants.

- 2026-08-29 — **EX-6.** `toml v1.1.4`, `optional = true`, pulled in by `shell`.
  Written as `dep:toml` rather than a bare name deliberately: a bare entry also
  mints an implicit `toml` feature, which is a second, ungated way into the
  graph. Stratum 1's tree is jiff, serde, serde_json and their transitives, and
  nothing else:

  ```
  $ cargo tree --no-default-features
  goad v0.1.0 (/home/david/dev/goad)
  ├── jiff v0.2.35
  ├── serde v1.0.229
  └── serde_json v1.0.151
  (no tokio node, no toml node)

  $ cargo tree --depth 1
  goad v0.1.0 (/home/david/dev/goad)
  ├── jiff v0.2.35
  ├── serde v1.0.229
  ├── serde_json v1.0.151
  ├── tokio v1.53.1
  └── toml v1.1.4+spec-1.1.0
  ```

- 2026-08-29 — **EX-3, all three parts observed.**

  **(a)** `cargo tree --no-default-features` — no tokio node. Above.

  **(b)** `cargo test --no-default-features` runs `lib` (4) and
  `tests/protocol/main.rs` (4) and **does not list `tests/integration/main.rs`
  at all** — skipped for unmet `required-features`, not built and not failed.
  The default column does run it. That contrast is the criterion.

  **(c) Break-and-revert, at both sites the plan offers, because they prove
  different things.**

  ```
  # tokio named in src/semantics/mod.rs, stratum 1 column
  error[E0433]: cannot find module or crate `tokio` in this scope
   --> src/semantics/mod.rs:7:41
  # the same file, same token, default column
  Finished `dev` profile [unoptimized] target(s) in 0.04s

  # tokio named in tests/protocol/main.rs, stratum 1 column
  error[E0433]: cannot find module or crate `tokio` in this scope
    --> tests/protocol/main.rs:13:5
  ```

  The middle line is the one worth reading: **a `semantics/` module that uses
  tokio compiles perfectly well in the default column.** The build gate catches
  it only in the column where the runtime is absent, which is exactly why AC-15
  keeps VT-1's grep as well and why removing either half would be a loss.

  The protocol-target result confirms `plan-log.md:33` on its own terms: the
  stratum 1 *test* target cannot name a runtime either, with no
  `[dev-dependencies]` section in the manifest — and none was added.

- 2026-08-29 — **VA-3.** `just -n check` against `design.md:1913`. Same six
  commands, same arguments, same order; the differences are §9's inline comments
  and its wrapped second clippy line, neither of which `just -n` prints (F-9,
  F-13).

  ```
  $ just -n check                        │  design.md §9
  cargo build                            │  cargo build
  cargo test                             │  cargo test
  cargo test --no-default-features       │  cargo test   --no-default-features   # stratum 1 alone
  cargo clippy --all-targets -- -D warnings
                                         │  cargo clippy --all-targets -- -D warnings
  cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub
                                         │  cargo clippy --all-targets --no-default-features -- \
                                         │    -D warnings -A dead_code -A unreachable_pub
  cargo fmt --check                      │  cargo fmt --check
  ```

- 2026-08-29 — **noticed in passing, not acted on.** `rustfmt.toml` carries only
  `tab_spaces = 2`, as the sheet specified, and that fixes indentation — but
  stock rustfmt still explodes any struct literal wider than `struct_lit_width`
  (default 18) across lines. So `design.md` §5.2's
  `UnsupportedPrimitive { kind: String, at: String },` is one line in the design
  and four in `error.rs`. Task 3's stated aim was that the design's snippets stay
  copy-able; indentation was the half it named, and this is the half it did not.
  A `struct_lit_width` setting would close it. **Not done** — the sheet fixed
  `rustfmt.toml`'s content precisely, and widening it is a decision, not an
  implementation detail. Raised for the user; carried to Harvest.

- 2026-08-29 — **STOP. `clippy::module_name_repetitions` and the §5.2 type
  names are incompatible, and the phase gate cannot go green until that is
  decided.** Raised to the user; not improvised past.

  `Cargo.toml:157` sets `module_name_repetitions = "deny"`. In
  `src/semantics/error.rs` it fires three times — `ProtocolError`, `BoundsError`
  and `ScheduleError` all end with their module's name:

  ```
  error: item name ends with its containing module's name
    --> src/semantics/error.rs:17:10  |  pub enum ProtocolError {
    --> src/semantics/error.rs:41:10  |  pub enum BoundsError {
    --> src/semantics/error.rs:56:10  |  pub enum ScheduleError {
  ```

  **This is the whole of what is failing.** Five of the gate's six commands are
  green; `lint` fails on this and nothing else, in both columns. `cargo fmt`
  applied and clean, `cargo test` green in both columns.

  **It is a class, not an instance.** §5.2 puts `BackendError`, `CleanupFailure`
  and `StateError` in `shell/error.rs`, so two more fire at PHASE-04/05. The
  design has chosen `…Error` inside an `error` module as its convention; this
  lint forbids that convention.

  **Neither escape the phase could take on its own is free:**
  - Renaming to `error::{Protocol, Bounds, Schedule}` contradicts EX-2's "exactly
    as §5.2 lists them", and reads badly — `Protocol::Bounds(Bounds)`.
  - A `pub use` re-export at `semantics::` is forbidden by
    `clippy::pub_use = "deny"` (`Cargo.toml:158`).
  - Turning the lint off, or `#[expect]`ing it, is a lint-table change, which the
    sheet's STOP list assigns to `design.md` §9 and the user, not to this phase.

  Both viable resolutions were **probed and both work**, so the decision is about
  which is right, not about whether either lands:
  `module_name_repetitions = "allow"` in `[lints.clippy]` → `just check` exits 0;
  a module-level `#![expect(clippy::module_name_repetitions, reason = …)]` →
  clippy clean.

  **Found alongside it, for audit rather than for now.** `design.md:921` writes
  the wrapped type as `Protocol(semantics::ProtocolError)` — the path
  `semantics::ProtocolError`, not `semantics::error::ProtocolError`. Reaching
  that path needs a re-export from `semantics/mod.rs`, which
  `clippy::pub_use = "deny"` forbids. So the design's own spelling of the path
  is unreachable under the design's own lint table. Nothing this phase does
  depends on it — no caller exists yet — and it is a reconciliation item, not a
  phase repair.

- 2026-08-29 — **STOP resolved, user decision: allow the lint crate-wide.**
  `module_name_repetitions = "allow"` in `[lints.clippy]`, with the argument
  written at the site: §5.2 has chosen `…Error`-inside-`error` as this crate's
  convention, so every error type in the design violates the lint by
  construction. A per-module `#[expect(…, reason = …)]` was the alternative and
  was rejected as an instance fix for a class defect — §9 built that hatch for
  rare, individually argued exceptions like `unwrap_used`, not for a convention
  broken deliberately every time. §9's lint prose should gain a sentence at
  audit; the lint table and §9 are now one line apart.

- 2026-08-29 — **the `rustfmt` question, investigated and answered *against*
  changing anything.** The user asked what works better long term rather than
  picking, so it was measured.

  `struct_variant_width` (default 35) is the option that governs enum variant
  definitions — not `struct_lit_width`, which governs construction sites. Raised
  to 60 it does exactly what was wanted:

  ```
  tab_spaces = 2 only                 |  + struct_variant_width = 60
    UnsupportedPrimitive {            |    UnsupportedPrimitive { kind: String, at: String },
      kind: String,                   |    InapplicableKey { key: usize, kind: String, at: String },
      at: String,                     |
    },                                |
  ```

  **And then it does not.** A doc comment on *any* variant makes rustfmt abandon
  the compact form for the **whole enum**, whatever the width. Same enum, same
  config, doc comments the only difference:

  ```
  ### verbatim §5.2 enum ###          ### same, doc comments stripped ###
    UnsupportedProtocolVersion {        UnsupportedProtocolVersion { found: u32 },
      found: u32,                       UnsupportedPrimitive { kind: String, at: String },
    },                                  InapplicableKey { key: &'static str, kind: String, at: String },
  ```

  So the setting cannot deliver the thing it was for. The enums that most need
  line-for-line correspondence with §5.2 are precisely the ones carrying
  per-variant rationale — `NotFinite` holds D39's argument, `MissingOffset`
  holds brief §13's. What would remain is config that looks load-bearing and is
  not, with an enum's layout flipping silently the first time someone documents
  a variant. **Reverted; `rustfmt.toml` holds `tab_spaces = 2` and nothing
  else, exactly as the sheet specified.** The observation goes to Harvest so
  this is not re-litigated.

  Two rustfmt behaviours were tripped over on the way and are worth writing
  down: `rustfmt --print-config current .` does **not** read the project's
  `rustfmt.toml` (it reported `tab_spaces = 4` against a tree that formats at
  two), so it is useless as a diagnostic here; and rustfmt **never rejoins** an
  already-split item, so a width option looks inert when tested against source
  the previous `cargo fmt` had already exploded. Both cost time.

- 2026-08-29 — **A4 re-checked before closing, as the sheet required.**
  `src/semantics/error.rs` declares types and formats them. It does no
  arithmetic, acquires no `From<serde_json::Error>`, and inspects no value at
  run time. So no module-level `#![deny(clippy::arithmetic_side_effects)]` lands
  this phase and **PHASE-02 is still the first to owe one**. The only `+` under
  `src/semantics/` is inside a doc comment (`now + span`, on
  `ScheduleError::OutOfRange`). `boundary.rs` does `offset + 1`, which is a test
  target and outside I9's scope in any case.

- 2026-08-29 — **the gate. EX-1 / VA-1.** `just check` exits 0, six of six
  commands, both feature columns. Blank lines stripped, nothing else.

```
cargo build
    Finished `dev` profile [unoptimized] target(s) in 0.03s
cargo test
    Finished `test` profile [unoptimized] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/goad-b2dc77c978f22f5c)
running 4 tests
test semantics::error::tests::the_taxonomy_implements_error_and_wrapping_variants_expose_their_source ... ok
test semantics::error::tests::every_bounds_error_display_names_what_it_carries ... ok
test semantics::error::tests::every_schedule_error_display_names_what_it_carries ... ok
test semantics::error::tests::every_protocol_error_display_names_what_it_carries ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/integration/main.rs (target/debug/deps/integration-4e1a897ae41c6ffc)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/protocol/main.rs (target/debug/deps/protocol-faa3768803807cf7)
running 4 tests
test boundary::a_scan_that_inspects_no_rust_files_fails ... ok
test boundary::a_scan_whose_directory_was_renamed_away_fails ... ok
test boundary::stratum_1_names_neither_the_shell_a_binary_nor_the_runtime ... ok
test boundary::no_host_source_file_names_the_user_s_domain ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests goad
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
cargo test --no-default-features
    Finished `test` profile [unoptimized] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/goad-8f2b0ee03f1fa567)
running 4 tests
test semantics::error::tests::every_bounds_error_display_names_what_it_carries ... ok
test semantics::error::tests::every_protocol_error_display_names_what_it_carries ... ok
test semantics::error::tests::the_taxonomy_implements_error_and_wrapping_variants_expose_their_source ... ok
test semantics::error::tests::every_schedule_error_display_names_what_it_carries ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/protocol/main.rs (target/debug/deps/protocol-ad9a7db0a1b8b441)
running 4 tests
test boundary::a_scan_that_inspects_no_rust_files_fails ... ok
test boundary::a_scan_whose_directory_was_renamed_away_fails ... ok
test boundary::stratum_1_names_neither_the_shell_a_binary_nor_the_runtime ... ok
test boundary::no_host_source_file_names_the_user_s_domain ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests goad
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized] target(s) in 0.03s
cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub
    Finished `dev` profile [unoptimized] target(s) in 0.01s
cargo fmt --check
```

- 2026-08-30 — **VH-1's human half, discharged.** Recorded here rather than in
  PHASE-02's sheet because the criterion is PHASE-01's; the phase's verification
  record now carries both halves and the Harvest item is closed. The user
  reloaded and all four tools resolve into `/nix/store/`:

  ```
  just   /nix/store/ni2dxycnhsp34y4qy6q44nw6pp6bj0l0-just-1.58.0/bin/just
  deno   /nix/store/pn1qbka1qfxw0wfbh1scsd2gvhv0dhj2-deno-2.9.4/bin/deno
  cargo  /nix/store/cyn97lq74y3lx15y95gyzplnmmx451g9-rust-default-1.99.0-beta.1-2026-08-18/bin/cargo
  rustc  /nix/store/cyn97lq74y3lx15y95gyzplnmmx451g9-rust-default-1.99.0-beta.1-2026-08-18/bin/rustc
  ```

  The `deno` store hash is **identical** to the one the agent saw on 2026-08-29,
  so the two halves are not merely both green — they agree on the same flake
  evaluation, which is what makes the human half worth running at all.

  Two things noticed in passing, neither a criterion:

  - the toolchain is `rust-default-1.99.0-beta.1-2026-08-18`, a **beta** pinned
    by `flake.lock`. Fine while pinned; it is the thing that would move under
    the slice if the lock is ever floated, so a gate failure that appears with
    no source change should suspect it first.
  - the user's interactive shell is **nu**, not bash. `&&` is not nu syntax, so
    a bash-style chain handed over for a `!` line runs as separate commands or
    not at all. Affects how commands are handed to the user; the agent's own
    `nix develop --command bash -c '…'` is unaffected.

### PHASE-02 — Canonical types and their checked constructors

**State:** **done 2026-08-30.** `just check` exits 0 in both feature columns.
Three plan gaps raised in all — two while writing the sheet, one during task 2 —
and all three closed by user decision before any code depended on them.
**Plan entry:** `docs/slices/001/plan.md:305`
**Surfaces (from the plan, as amended 2026-08-29):** `src/semantics/mod.rs` (one
line, `pub mod protocol;`), `src/semantics/protocol/mod.rs`,
`src/semantics/protocol/canonical.rs`, `src/semantics/error.rs` (extend only if a
variant proves to need a field the design named and PHASE-01 missed). This
phase's tests are colocated `#[cfg(test)]` modules in those files;
`tests/protocol/` is PHASE-03's.

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | PHASE-01/EX-1 and EX-2 discharged | **met**, re-verified 2026-08-29 rather than read off the status table. `just check` exits 0 on all six commands in a fresh `nix develop` (EX-1). `src/semantics/error.rs` carries the §5.2 taxonomy complete — `ProtocolError` 12 variants (`:17`), `BoundsError` 2 (`:65`), `ScheduleError` 5 (`:80`), each with `Display` and `std::error::Error` (EX-2) |

#### What already exists — inspected 2026-08-29

| path | state | consequence for this phase |
|---|---|---|
| `src/semantics/error.rs` | exists, 353 lines. Every variant this phase raises is already declared with the field set §5.2 gives it, including `at: String` on the five collection errors | EX-3 and EX-4 **assert against** this file; the parenthesis in the plan's Surfaces line ("extend only if…") is not expected to fire |
| `src/semantics/mod.rs` | exists, 5 lines: doc comment and `pub mod error;` | needs `pub mod protocol;` — one line. It was missing from the declared surfaces and was added to them on 2026-08-29 |
| `src/semantics/protocol/` | **does not exist** | every file in it is new |
| `tests/protocol/` | `main.rs` declaring `#[cfg(test)] mod boundary;`, plus `boundary.rs` | PHASE-03 owns extending this target (`plan.md:374`). This phase does not touch it — see the test-home decision below |
| `Cargo.toml` | `jiff = { version = "0.2", default-features = false }`, no `serde` feature | measured below; no manifest change is needed and none is authorised here |
| the colocated-test pattern | `src/semantics/error.rs:163` — `#[cfg(test)] mod tests` inside the module under test, which is what satisfies `clippy::tests_outside_test_module` | prior art for where this phase's tests go |

#### Three things the plan did not settle — raised, and **all closed**

Both surfaced by expanding the phase; neither was a repair this sheet could make
on its own (`AGENTS.md`, *Phase plan*: if expanding the phase shows the plan is
wrong, go back to the plan). Both went to the user, both were decided as
recommended, and `plan.md:311` and its EX-1 now carry the outcome. The reasoning
is kept below because it is the argument the phase will be judged against; the
decisions themselves are in `plan-log.md`.

**1. The declared surfaces have no home for this phase's tests, and no line for
`src/semantics/mod.rs`.** PHASE-02 is the only phase in `plan.md` whose Surfaces
name no test path; every other phase that writes a test names the file. It also
cannot declare `src/semantics/protocol` without editing `src/semantics/mod.rs`.

The test home is not a free choice, and this is the substantive half. **VT-1 and
VT-3 cannot live in `tests/protocol/`.** That target is an external crate, and
under D30 (`design.md:1868`) `Opt`, `Field` and `Alternative` have `pub(super)`
fields and no public constructor — so an external test cannot build the `Vec<Opt>`
that `Options::new` rejects, nor the two same-id `Field`s VT-3 needs. Making them
constructible from outside would be exactly R10 (`design.md:1903`), the risk this
phase's VA-2 exists to catch. So VT-1 and VT-3 are `#[cfg(test)]` unit tests in
`canonical.rs`, following `error.rs:163`.

VT-2 could go either way — the outbound types have public fields by design — and
the recommendation is to colocate it with the other two: one home for the phase's
tests, no collision with PHASE-03's ownership of the `tests/protocol/` scaffolding,
and PHASE-04's corpus is where the protocol tier gets exercised from outside.

**Amendment applied to `plan.md:311`:** surfaces are now
`src/semantics/mod.rs`, `src/semantics/protocol/mod.rs`,
`src/semantics/protocol/canonical.rs`, `src/semantics/error.rs` (extend only
if…), with a sentence saying this phase's tests are colocated `#[cfg(test)]`
modules and that `tests/protocol/` is PHASE-03's.

**2. Which scalar newtypes get a public constructor.** The design gives the
scalars no constructors at all, and two downstream phases need some of them:
PHASE-07 mints `view_id` as `{now RFC 3339}#{seq}` in `src/shell/state.rs`
(`plan.md:732`, D13) and its surfaces do not include `canonical.rs`, so whatever
it needs must ship here; and stratum 2 reads the clock, so it must be able to
build a `Timestamp` from a `jiff::Timestamp`.

Recommended split, on the host-authored / backend-authored line:

- **`ViewId` and `Timestamp` get public constructors.** Both name values the
  *host* authors — a minted id and a clock read. Nothing about them is a claim
  that a backend said something.
- **`OptionId`, `FieldId` and `AlternativeId` do not.** They are backend-authored
  addresses; a caller answering a view clones the one the view carries, through
  the accessor. A public constructor would let a caller mint an id no backend ever
  sent, which is the same hole D30 closes on the canonical types.
- Every scalar gets a public read accessor regardless, and `Clone` (already in
  the design's derive list), which is what makes the second bullet workable.

**Decided as recommended**, 2026-08-29. `plan.md`'s EX-1 now states the split and
its reason, so PHASE-07 inherits it rather than rediscovering it.

**3. EX-3 required `Fields` to reject empty, and it must not.** Found during
task 2, writing VT-1's rejection cases: the empty-`Fields` case had no error
variant to assert, because there is no `EmptyFields` in the taxonomy and never
was one. R-15 (`draft-spec.md:106`) says an option **MAY** carry fields; R-15's
verification row (`:364`) asks for "an option with and without fields";
`brief.md:131` and `:567` say it twice; and the spec's example response (`:232`)
carries `{ "id": "yes", "label": "Now" }` with no `fields` key. §5.5's edge table
has rows for `options: []`, duplicate option ids, duplicate field ids and empty
alternatives, and **no empty-fields row**. `Opt.fields` is a `Fields` and not an
`Option<Fields>`, so an option with no fields is a `Fields` holding none —
rejecting that would have made the spec's own example unnormalizable at PHASE-04.

The source is `design.md:704`'s comment over the three newtypes — "all three for
the same reason: >= 1 element, and ids unique" — where the F-52 paragraph beneath
it argues only duplicates for fields and never argues non-emptiness. EX-3
restated the blanket comment instead of the argument. **The slice's recurring
defect, third instance.**

**Decided 2026-08-30:** `Fields::new` checks uniqueness only; `Options::new` and
`Alternatives::new` keep both checks; no `EmptyFields` is invented. `plan.md`'s
EX-3 rewritten; `design.md:704` left as written and carried to audit
reconciliation, like the `toml` line before it (`plan-log.md`).

#### Reading list

Read before writing anything. `path:line`.

| what | where | why |
|---|---|---|
| the phase itself | `docs/slices/001/plan.md:305` | criteria are binding as written |
| **the canonical types** | `docs/slices/001/design.md:644` | EX-1 transcribes this block. Every type, every field, no additions |
| the argument behind the three collection newtypes | `docs/slices/001/design.md:717` | why `Options`, `Fields` and `Alternatives` are one rule and not three cases — the refactor in task 9 is this argument in code |
| `Alternative` is not an `Opt` | `docs/slices/001/design.md:697`, `:757` | F-61 and F-54. EX-4 is these two paragraphs |
| **the outbound types** | `docs/slices/001/design.md:841` | EX-2 transcribes this block |
| why `NotFinite` stays and what a NaN literal actually produces | `docs/slices/001/design.md:781` | D39/F-36. Do not write a test asserting `NotFinite` from JSON; there is no JSON that reaches it |
| the error taxonomy as landed | `src/semantics/error.rs:17` | the variants EX-3 asserts. Read the file, not §5.2, for field names |
| the edge-case rows this phase implements | `docs/slices/001/design.md:1716`–`1720`, `:1733` | one VT-1 case per row |
| I1, I15, I16 | `docs/slices/001/design.md:1654`, `:1668`, `:1669` | what the constructors are holding |
| D30 | `docs/slices/001/design.md:1868` | `pub(super)` plus accessors; the reason, not just the rule |
| D45, D46, D52 | `docs/slices/001/design.md:1883`–`1889` | the three newtypes, the no-recursion rule, and the separate alternative namespace |
| D4 | `docs/slices/001/design.md:1842` | jiff default features off. If something wants a time zone, stop |
| D13 | `docs/slices/001/design.md:1851` | `view_id` shape — read only to know what PHASE-07 will need of `ViewId` |
| R10 | `docs/slices/001/design.md:1903` | VA-2's reason for existing |
| R-1, R-6, R-7, R-8 | `docs/slices/001/draft-spec.md:81`, `:92`–`:94` | what a request must carry |
| R-52, R-53, R-16 | `docs/slices/001/draft-spec.md:107`–`:109` | the uniqueness rule and the shape of a `choice` field |
| **the request wire forms** | `docs/slices/001/draft-spec.md:232` | VT-2 asserts against these two documents literally |
| the spec's own verification rows | `docs/slices/001/draft-spec.md:354`, `:360` | VT-2 and VT-3 are these rows |
| the module layout | `docs/slices/001/design.md:303` | `protocol/{wire,canonical,normalize}.rs`; this phase writes one of the three |
| the lint table | `Cargo.toml:57` onwards | `dead_code` is `warn` in the manifest and the first clippy line passes `-D warnings`, so it still fails the gate. See A3 |
| D53 as amended | `docs/slices/001/design.md:1890` | the module-level `arithmetic_side_effects` obligation this phase inherits (A4 on PHASE-01's sheet) |
| prior art | `src/semantics/error.rs` | the whole file: doc-comment register, two-space indent, `#[cfg(test)] mod tests`, exhaustive matches used as compile-time guards |

#### Assumptions

Each is checkable; check it rather than proceeding on it.

- **A1 — the dev shell.** Same as PHASE-01/A1 and it was false there. Run
  `nix develop --command bash -c '…'` for every command, or verify the shell
  first. `just` must resolve to a store path, not to `~/.nix-profile/bin/just`.
- **A2 — jiff with `default-features = false` is enough for everything this phase
  needs.** **Checked 2026-08-29, by running it**, in a scratch crate carrying the
  same three dependencies:
  - `jiff::Timestamp::from_str("2026-08-23T04:12:00Z")` parses;
  - `Display` prints `2026-08-23T04:12:00Z`, which is `draft-spec.md:232`'s form
    exactly;
  - a hand-written `Serialize for Timestamp` doing `s.collect_str(&self.0)`
    produces `"2026-08-23T04:12:00Z"` in the JSON;
  - `"45 minutes".parse::<jiff::Span>()` works too, which is PHASE-03's problem
    and is recorded here because the probe was already running.

  So **no jiff `serde` feature is needed and none may be added** — jiff's `serde`
  feature is a `dep:serde_core` edge, and adding it is a dependency change, which
  is a STOP. The newtype is ours; its `Serialize` is ours to write.
- **A3 — nothing this phase lands is dead in the default-features column.**
  `dead_code` is `warn` in `Cargo.toml` but the first clippy line passes
  `-D warnings`, so a private item with no caller fails `just check` (`design.md`
  §9). Public items on public types in public modules are reachable and do not
  fire; a `pub(super)` field read by its accessor does not fire. A private helper
  written before its caller does. Order the work so nothing sits uncalled.
- **A4 — this is the phase that owes a module-level
  `#![deny(clippy::arithmetic_side_effects)]`.** PHASE-01 recorded the obligation
  as moving here (`src/semantics/error.rs:8` says so in the file). `canonical.rs`
  validates backend-derived values, so it takes the attribute. If the implementer
  concludes it does not — the file does no arithmetic today — that judgement is
  written down here and the obligation moves to PHASE-04's `normalize.rs`, which
  certainly does.
- **A5 — the checked constructors take the path they are constructing at.**
  `EmptyOptions`, `DuplicateOptionId`, `DuplicateFieldId`, `DuplicateAlternativeId`
  and `EmptyAlternatives` all carry `at: String` (`src/semantics/error.rs:39`
  onwards) and a constructor called in isolation has no path context, so the
  caller supplies it. `NumberRange::new` is the exception and stays as §5.2 writes
  it — `BoundsError` carries no path, deliberately.

#### STOP conditions

Stop and consult the user; do not improvise past any of these.

- Any dependency or dependency-feature change, jiff's `serde` feature
  specifically. A2 says it is not needed; if something appears to need it, the
  something is wrong.
- Any `impl Deserialize` in `canonical.rs`. Canonical values are reached through
  `normalize_response` (P1); a derive here is a second door and it is PHASE-04's
  work in the wrong file.
- Any `pub` field on a canonical type, or an accessor returning `&mut`. That is
  R10 and VA-2 is looking for it.
- Any type, variant or field added to or removed from `design.md:644` and `:841`.
  `Content`'s four variants and every `FieldKind` are admitted and rendered by
  nobody this slice — that is P3, not dead weight to trim.
- A time zone, a clock read, or anything that wants jiff's default features.
- `HashMap`/`HashSet` — `clippy.toml` disallows both. `Hints` is a `BTreeMap` in
  the design and the uniqueness checks want `BTreeSet`.
- Anything outside the declared surfaces, including `src/semantics/mod.rs` until
  the decision above is taken.

#### Tasks

Red / green / **refactor**. The refactor step is not optional.

1. **Entry check, then the two open items.** Confirm the shell (A1) and re-run
   `just check` green before touching anything, so a later failure is this
   phase's. The surfaces and constructor-visibility decisions are settled —
   `plan-log.md`, 2026-08-29 — so there is nothing to wait on.
2. **RED — VT-1, one case per rejection, before any constructor exists.** Empty
   `Options` and `Alternatives` — **not** `Fields`, which permits zero elements
   (item 3 above, decided 2026-08-30); a duplicate id in all three; a
   `NumberRange` that is inverted; a `NumberRange` with a non-finite bound. Each asserts the
   **variant and the `at` path it carries** — `EmptyAlternatives` and
   `DuplicateAlternativeId` for alternatives, never `EmptyOptions` /
   `DuplicateOptionId` (EX-4). A missing type is a weak red; treat it as the
   first red and plan the break-and-revert in task 8 as the real one.
3. **GREEN, part one — the scalars.** `ViewId`, `OptionId`, `AlternativeId`,
   `FieldId`, `Timestamp`, `Hints`, with the constructor visibility the user
   settled and a read accessor each. `Ord`/`Eq` on `FieldId` — it keys a
   `BTreeMap` in `UserResponse`.
4. **GREEN, part two — the canonical types.** `Response`, `View`, `Choice`,
   `Opt`, `Content`, `Field`, `FieldKind`, `Alternative`, then the three checked
   collection newtypes and `NumberRange`. `pub(super)` fields, read-only
   accessors, `Debug`/`Clone`/`PartialEq`. VT-1 goes green here.
5. **GREEN, part three — the outbound types.** `Request`, `Evaluate`, `Respond`,
   `Event`, `UserResponse`, with public fields (this is the one place VA-2
   permits them) and `Serialize`. The encoding is left to implementation by
   §5.2; a shape measured to produce the spec's bytes exactly is a private
   envelope `struct { protocol: u32, #[serde(flatten)] kind }` over an
   internally-tagged `#[serde(tag = "type", rename_all = "lowercase")]` enum,
   with `Serialize for Timestamp` doing `collect_str`.
6. **VT-2 — the two request snapshots.** Assert against the literal JSON at
   `draft-spec.md:232`, compared as parsed `serde_json::Value` so that key order
   is not asserted but a **missing `protocol` or `type` is**. A round trip would
   pass with the version field absent, which is the whole point of the criterion.
7. **VT-3 — the negative case.** The same `FieldId` used by fields in *different*
   options is accepted. This is what shows I15's scope is per-option, not
   per-view (`draft-spec.md:360`).
8. **RED again, by break-and-revert.** Weaken one uniqueness check and one
   emptiness check, confirm the specific VT-1 case fails and nothing else, revert.
   PHASE-01's standard: a criterion that names a mechanism is not yet a criterion
   that has one.
9. **REFACTOR — the three collection constructors share one rule, not two.**
   `design.md:717` states uniqueness as one rule deliberately, and three copies
   of a `BTreeSet` walk is the same restatement defect this slice keeps
   producing, in code. **Uniqueness covers all three; non-emptiness covers two**
   (item 3), so a single helper bundling both checks would be the same
   over-generalisation the plan just shed. One uniqueness helper parameterized by
   the error each collection raises, with the emptiness check layered on the two
   that have one, is the shape. Watch
   A3 while doing it: a helper with no caller yet fails the gate.
10. **VA-2.** Grep `canonical.rs` for `pub ` on a struct field. None outside the
    outbound request types. Paste the grep and its output, not a claim.
11. **VA-1.** `just check` exits 0, both feature columns. Paste it.
12. **Bookkeeping before handing off** — Status table, this sheet kept current as
    you go, `## Harvest` updated in place.

#### Verification record

| id | mode | result | evidence |
|---|---|---|---|
| VT-1 | test | **pass**, and seen to fail twice over | eight cases in `canonical.rs`: empty `Options`, empty `Alternatives`, duplicate option / field / alternative ids, an inverted range, a non-finite bound at each end, plus the one-bound and no-bound acceptances. Red once as eleven unresolved types, then by break-and-revert — see Log, task 8 |
| VT-2 | test | **pass**, first run | `an_evaluate_serializes_to_the_spec_s_wire_form` and `a_respond_serializes_to_the_spec_s_wire_form`, each against the literal JSON at `draft-spec.md:232` parsed to `serde_json::Value`, so key order is not asserted and a missing `protocol` or `type` is. A third test, `every_request_kind_carries_the_version_and_a_discriminant`, asserts R-1 and R-6 over both kinds at once so a third request kind added without an envelope fails here rather than at PHASE-04 |
| VT-3 | test | **pass** | `the_same_field_id_in_different_options_is_accepted` — two options each carrying a `minutes` field. A per-view uniqueness check would pass every VT-1 case and still be wrong; this is the case that separates them |
| VA-1 | agent | **pass** | `just check` exits 0, six commands, both feature columns. Full output in the Log under *the gate*. 17 unit tests in each column, plus the four boundary tests in the default column |
| VA-2 | agent | **pass** — grep pasted, not claimed | Log, task 10. Eleven `pub` struct fields, all four owners identified mechanically as `Evaluate`, `Respond`, `Event` and `UserResponse` — the outbound types, where `pub` is the design's own exception. Fourteen `pub(super)` fields on the inbound types, and **no `&mut` anywhere in the file** |
| EX-1 | — | **pass** | six scalars with the constructor split the user settled (`ViewId`/`Timestamp` public, the three backend-authored ids `pub(super)`), the eight inbound types, `Options`/`Fields`/`Alternatives`, `NumberRange`. All `pub(super)` fields, read accessors, `Debug`/`Clone`/`PartialEq` |
| EX-2 | — | **pass** | `Request`, `Evaluate`, `Respond`, `Event`, `UserResponse` with public fields and `Serialize`; `"protocol": 1` and a `"type"` of `evaluate`/`respond` asserted by three tests |
| EX-3 | — | **pass, as amended** | every §5.5 row this phase owns has a case. **`Fields` permits empty** — the criterion was over-general and was rewritten before code depended on it; see item 3 above and `plan-log.md`, 2026-08-30 |
| EX-4 | — | **pass, and the two halves pass differently** | the *error* half is tested: `duplicate_alternative_ids_are_rejected_as_alternatives_never_as_options` and its empty counterpart assert `DuplicateAlternativeId` / `EmptyAlternatives` and would fail on the `Options` variants. The *type* half is *structural, not tested*: `Alternative.id` is an `AlternativeId` and `Opt.id` an `OptionId`, both newtypes over a private `String` with no conversion between them, so passing one for the other does not compile. Asserting a **non**-compilation needs `trybuild`, which is a dependency addition and therefore a STOP — so this is recorded as compiler-enforced by construction rather than dressed up as a test that exists |
| EX-3 | — | — | |
| EX-4 | — | — | |

#### Log

<!-- Append as you go: decisions taken, obstacles, anything noticed in passing.
     Do not save the bookkeeping for the end; it will be lost. -->

- 2026-08-29 — sheet written. EN-1 re-verified rather than read off the status
  table: `just check` green on all six commands, taxonomy complete in
  `src/semantics/error.rs`.
- 2026-08-29 — **A2 measured before it could become an assumption.** A scratch
  crate with `jiff = { version = "0.2", default-features = false }`, serde and
  serde_json: parse, `Display`, `collect_str` and `Span` parsing all work, and
  both request forms serialize byte-identical to `draft-spec.md:232` — including
  a newtype-of-`String` `FieldId` used as a `BTreeMap` key, which serde_json
  renders as a plain JSON object key. Written up under A2; the consequence is
  that the jiff `serde` feature is unnecessary, so the dependency STOP does not
  need to fire.
- 2026-08-29 — two plan gaps raised and **both closed by user decision the same
  day**: surfaces amended to include `src/semantics/mod.rs` with the phase's
  tests colocated, and `ViewId`/`Timestamp` alone getting public constructors.
  `plan.md:311` and EX-1 rewritten; reasoning in `plan-log.md`.

---

- 2026-08-30 — **task 1, entry check.** Shell verified before anything else
  (A1's lesson from PHASE-01): `just`, `cargo` and `rustc` all resolve to store
  paths, `rustc 1.99.0-beta.1`. `just check` green before a line was written, so
  any later failure is this phase's.

- 2026-08-30 — **task 2, RED, and it found the third plan gap.** Writing the
  rejection cases before the constructors existed is what surfaced it: the
  empty-`Fields` case had no error variant to assert against, because there is
  no `EmptyFields` in the taxonomy and never was. Chased it to four sources
  saying an option may carry no fields — R-15, R-15's own verification row,
  `brief.md` twice, and the spec's example response — and to the absence of an
  empty-fields row in §5.5. Raised, decided, `plan.md` EX-3 rewritten before any
  code depended on it. **Written up as item 3 above and in `plan-log.md`.**

  The red itself: eleven unresolved types plus `AlternativeId`, which I had left
  out of the import list. Weak, as the sheet predicted — task 8 is the real one.

- 2026-08-30 — **tasks 3–7, GREEN.** Scalars, inbound types, checked
  collections, outbound types and their `Serialize`, then VT-2 and VT-3. Both
  VT-2 snapshots matched the spec's JSON on the **first run** — the envelope
  shape was not guessed at, it came out of A2's probe, which is the whole return
  on having measured it during planning.

- 2026-08-30 — **A3 fired, exactly where the sheet said it would.** Four
  `pub(super) fn new` — on `OptionId`, `AlternativeId`, `FieldId` and `Hints` —
  have no caller until PHASE-04's `normalize.rs`, so `dead_code` failed the
  first clippy column. Three things worth recording:

  1. The manifest already names this case — "a phased plan lands a type one
     phase before its caller" — and blesses `expect(dead_code, reason = …)` for
     it, self-clearing via `unfulfilled_lint_expectations`. Used as designed.
     Widening to `pub` was the alternative and would have undone EX-1's
     constructor split, which is the whole point of that decision.
  2. A plain `#[expect]` then failed the *other* way: clean under `cargo build`,
     unfulfilled under `cargo test`, because the colocated tests call those
     constructors. Scoped to `#[cfg_attr(not(test), expect(…))]`. This is a
     consequence of the colocation decision that nothing anticipated, and it is
     the kind of thing an external test target would not have produced.
  3. `unfulfilled_lint_expectations` makes the attributes self-removing: when
     PHASE-04 calls these, the gate fails until they come off. So this is a
     dated obligation with a mechanism, not a comment.

- 2026-08-30 — **`missing_errors_doc` fires, and the manifest says it does not.**
  `Cargo.toml`'s `[lints.clippy]` carries `# missing_errors_doc = "deny"`
  commented out under "Doc-comment lints paused alongside missing_docs", but
  `pedantic = "deny"` re-enables it, so the pause is real only for `missing_docs`
  itself — a rustc lint, in the other table. PHASE-01 never met this because
  `error.rs` returns no `Result`. Answered by writing the four `# Errors`
  sections, which the constructors wanted anyway; the lint table was not
  touched. **The stale comment is for audit, not a phase repair** — see Harvest.

- 2026-08-30 — **A4 discharged: `canonical.rs` takes the deny.** The file does
  no arithmetic today, so the sheet allowed the obligation to move on to
  PHASE-04's `normalize.rs` if the implementer judged it should. It should not:
  this is where backend-derived values first land and where the bounds and
  uniqueness checks live, the attribute costs nothing while there is no
  arithmetic, and PHASE-04 will add some here. `error.rs:10` said the first
  module handling backend-derived data owes one; `canonical.rs:16` now says it
  is that module, so the pointer resolves rather than dangling.

- 2026-08-30 — **task 8, the real RED, by break-and-revert.** Two mutations, one
  at a time, each reverted before the next:

  ```
  Options::new       `if !seen.insert(…)` → `… && false`
    → 1 failed: duplicate_option_ids_are_rejected_naming_the_id_and_where
      16 passed
  Alternatives::new  `if alternatives.is_empty()` → `if false && …`
    → 1 failed: an_empty_alternatives_is_rejected_as_alternatives_never_as_options
      16 passed
  reverted           → 17 passed
  ```

  Exactly one test each, and the right one — so the cases are specific, not a
  suite that happens to be red for any reason.

- 2026-08-30 — **task 9, REFACTOR, and the finding changed its shape.** The
  sheet framed this as "three copies of one rule collapse into one helper".
  After the `Fields` decision it is two rules: uniqueness over all three,
  non-emptiness over two. `ensure_unique_ids` takes the id projection and the
  error constructor and covers all three; the emptiness check stays inline in
  the two constructors that have one. **Bundling both into one helper would have
  been the same over-generalisation that got `Fields` wrong**, in code this
  time — so the plan gap improved the refactor rather than complicating it.

- 2026-08-30 — **the refactor tripped PHASE-01's AC-11 grep, on a doc comment.**
  `tests/protocol/boundary.rs` failed with ``forbidden token `site` `` against
  `canonical.rs:356` — my own phrase "call sites". `site` is in the
  domain-vocabulary list and the scan matches substrings. Reworded. Recorded
  because it is a false positive of a *canon* test, not of this phase's code:
  ordinary English can carry those tokens, and `reminder` is the next one likely
  to bite. **For audit** — see Harvest. Also worth saying plainly: PHASE-01's
  test caught text written by a later phase, unprompted, which is the test
  working.

- 2026-08-30 — **task 10, VA-2, pasted rather than claimed.**

  ```
  $ grep -nE "^[[:space:]]+pub [a-z_]+:" src/semantics/protocol/canonical.rs
  534:  pub now: Timestamp,
  535:  pub event: Event,
  540:  pub view_id: ViewId,
  541:  pub now: Timestamp,
  542:  pub response: UserResponse,
  547:  pub source: String,
  548:  pub kind: String,
  549:  pub timestamp: Timestamp,
  551:  pub data: serde_json::Value,
  556:  pub option: OptionId,
  558:  pub values: BTreeMap<FieldId, serde_json::Value>,

  $ awk '/^pub struct|^struct/ {name=$0} /^  pub [a-z_]+:/ {print NR": "name}' …
  534: pub struct Evaluate {
  540: pub struct Respond {
  547: pub struct Event {
  556: pub struct UserResponse {

  $ grep -nE "^[[:space:]]+pub\(super\) [a-z_]+:" … | wc -l
  14

  $ grep -n "&mut" src/semantics/protocol/canonical.rs
  (none)
  ```

  The owners are resolved mechanically rather than by eye: all eleven `pub`
  fields belong to the four outbound types, which is the design's own exception
  (D5 — requests are host-authored). Fourteen `pub(super)` on the inbound side,
  and no `&mut` accessor anywhere, which is R10's other signal.

- 2026-08-30 — **task 11, VA-1, the gate.** `just check` exits 0. Six commands,
  both feature columns, 17 unit tests in each plus PHASE-01's four boundary
  tests in the default column. Full output pasted below.

  ```
  cargo build            Finished `dev` profile
  cargo test             17 passed · integration 0 · protocol 4 · doc 0
  cargo test --no-default-features
                         17 passed · protocol 4 · doc 0   (integration skipped)
  cargo clippy --all-targets -- -D warnings
                         Finished
  cargo clippy --all-targets --no-default-features -- -D warnings \
        -A dead_code -A unreachable_pub
                         Finished
  cargo fmt --check      (silent)
  ```

  The `--no-default-features` column runs the same 17: nothing in this phase is
  behind `shell`, which is what stratum 1 being stratum 1 looks like.


### PHASE-03 — Schedule resolution, and the fixture runner

**State:** **done 2026-09-02.** `just check` exits 0 in both feature columns.
All four EX and all three VT criteria discharged; VA-1 and VA-2 pasted in the
Verification record. Entry criterion was checked, not assumed, and the baseline
was green before anything was touched. The four plan gaps found at expansion
were all closed before execution began, and **execution raised no new one** —
the first phase in this slice to need nothing from the user mid-flight.
Two defects were found in this phase's own runner at the refactor step and fixed
there; both are in the Log, and both now have a break-and-revert holding them.
**Plan entry:** `docs/slices/001/plan.md:387`
**Surfaces (from the plan):** `src/semantics/schedule.rs`,
`tests/protocol/main.rs`, `tests/protocol/runner.rs`,
`tests/protocol/fixtures/schedule/**`, and `src/semantics/mod.rs` (one line,
`pub mod schedule;`) — **added to the plan's list 2026-09-02**; see item 1.

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | PHASE-02/EX-1 discharged (`Timestamp` exists) | **met**, verified by reading the file rather than the status table. `src/semantics/protocol/canonical.rs:145` — `pub struct Timestamp(jiff::Timestamp)`, `Debug + Clone + Copy + PartialEq + Eq`, with the public `new(jiff::Timestamp)` and `instant() -> jiff::Timestamp` the 2026-08-29 constructor split gave it. `Serialize` at `:160` |

Baseline re-run 2026-08-30, in a fresh `nix develop`: `just check` exits 0 on all
six commands, both feature columns — 17 unit tests, 4 boundary tests, integration
skipped in the stratum 1 column. `just` and `cargo` both resolve into
`/nix/store/` (A1). Any failure from here is this phase's.

#### What already exists — inspected 2026-08-30

| path | state | consequence for this phase |
|---|---|---|
| `src/semantics/protocol/canonical.rs` | 530 lines; `Timestamp` at `:145`, `Response.schedule: Option<Timestamp>` at `:205` with a `schedule()` accessor | the type this phase's two functions return. Not a surface here — nothing in it needs changing |
| `src/semantics/error.rs` | `ScheduleError` at `:80`, all five variants — `NotAString { found: &'static str }`, `MissingOffset { raw }`, `CalendarUnit { raw }`, `OutOfRange { raw }`, `Unparseable { raw }` | EX-1 asserts **against this file**, not against §5.2. The field names are the contract |
| `src/semantics/mod.rs` | 6 lines: doc comment, `pub mod error;`, `pub mod protocol;` | needs `pub mod schedule;`. One line, and it is item 1 |
| `tests/protocol/main.rs` | 11 lines; declares `#[cfg(test)] mod boundary;` with the comment explaining why the attribute is there | gains `#[cfg(test)] mod runner;`. The same attribute for the same reason — `clippy::tests_outside_test_module` is `deny` and reaches `tests/` targets |
| `tests/protocol/boundary.rs` | the AC-11 / AC-15 scans | **prior art for the vacuity guard**, which the corpus runner needs for the same reason: a walk over a directory that has been renamed away reports success. `Breach::Vacuous` is the shape to copy |
| `tests/protocol/fixtures/` | **does not exist** | every file in it is new |
| `Cargo.toml` | `jiff = { version = "0.2", default-features = false }`; resolved to **0.2.35** in `Cargo.lock`, the version the F-10 probe measured | no manifest change is needed and none is authorised here |
| the colocated-test pattern | `src/semantics/error.rs:163`, `canonical.rs` | where VT-2 goes — see item 3 |
| `#![deny(clippy::arithmetic_side_effects)]` | `canonical.rs:17` | `schedule.rs` takes the same attribute; A4 |

#### Four things the plan did not settle — one decided, three settled here

The first needed a user decision because it changes `plan.md`; the other three
are implementer latitude and are settled here so that PHASE-04 and PHASE-07
inherit them rather than rediscover them.

**1. The declared surfaces had no line for `src/semantics/mod.rs`. Raised, and
closed by user decision 2026-09-02.** `src/semantics/schedule.rs` cannot exist without
`pub mod schedule;` in `src/semantics/mod.rs`, and the plan's Surfaces line does
not name that file. This is **the same omission PHASE-02 hit** and it was closed
the same way on 2026-08-29 by amending `plan.md:311` — the third patch in the
same place would be evidence about the decision underneath it, but two is a
pattern in phase-plan *drafting*, not in the design: a phase that adds a module
always edits its parent's `mod` list, and only PHASE-01 wrote that down.

**Decided 2026-09-02: fix the class, not the instance.** The Surfaces of
PHASE-03, **PHASE-04** and **PHASE-07** were amended in one edit —
`src/semantics/mod.rs`, `src/semantics/protocol/mod.rs` and `src/shell/mod.rs`.
The proposal named PHASE-05; on applying it, PHASE-05 already declares both its
`mod` files and **PHASE-07** did not, so the third instance moved. PHASE-01 and
PHASE-05 were the only phases that had written it down. `plan-log.md`,
2026-09-02.

The same edit found one stale sentence: `src/semantics/protocol/mod.rs`'s doc
comment says `wire` arrives at PHASE-03. It arrives at PHASE-04, which now
declares that file and can make the sentence true.

**2. `resolve`'s signature, and what type `default_poll` arrives as.** EX-2 fixes
the arguments as `(retained, incoming, default, now)` and the return as a
concrete `Timestamp`; it fixes no types, and PHASE-07 inherits whatever this
phase picks. Settled here as:

```rust
pub fn resolve(
  retained:     Option<Timestamp>,
  incoming:     Option<Timestamp>,
  default_poll: jiff::SignedDuration,
  now:          Timestamp,
) -> Timestamp
```

- **`retained` is `Option`, not `Timestamp`.** §5.3's `State::resolved_check` is
  deliberately not an `Option` and is seeded by `Host::new`, so at run time it is
  always `Some`. Making the parameter optional is what lets `Host::new` **seed
  through this same function** instead of writing `now + default_poll` a second
  time in stratum 2. Brief §9's three arms then exist in exactly one place, which
  is what R-26 and R-27 are asking for.
- **`incoming: Option<Timestamp>`, and `None` means "no usable instruction".**
  An invalid `next_check` has already become `None` plus a `Discarded::Schedule`
  inside `normalize_response` (§5.2, P2), so resolution never sees invalidity and
  must not be given a `Result`. VT-2's "invalid preserves existing" case is
  therefore `incoming = None` at this layer; name the test so it says that, or a
  later reader will think the criterion is untested.
- **Latest-valid-wins is *issue order*, not `max()`.** Brief §9 and §22's
  point 8 — "a later valid `next_check` supersedes an earlier one" — mean the
  most recent *instruction*, so a valid `incoming` wins even when it is **earlier**
  than `retained`. `max(retained, incoming)` passes the obvious tests and is
  wrong; it also contradicts R-28, since a backend that says "check me sooner"
  would be silently overridden. VT-2 owes a case where `incoming < retained` and
  `incoming` still wins.
- **`jiff::SignedDuration`, not `std::time::Duration`.** Stratum 1 is jiff-native
  and `now` is a `jiff::Timestamp` under the newtype; taking a std `Duration`
  would put a fallible conversion inside a function that must be total. §5.2's
  `ScheduleConfig { default_poll: Duration }` does not say **which** `Duration`,
  so nothing is contradicted — PHASE-07 converts at the config boundary, which is
  where the TOML string is parsed anyway.
- **Totality at the range edge.** `now + default_poll` can overflow jiff's
  representable range, and R-27 forbids an unresolved state, so the third arm
  **saturates** at `jiff::Timestamp::MAX` rather than returning a `Result`.
  Reachable only from a `now` already at the edge of representable time; written
  down because `checked_add` returning `None` needs *some* answer and the choice
  should be on the page rather than in a `.unwrap_or(…)` a reviewer has to
  reverse-engineer. Note `unwrap_or` is fine — it is `unwrap()` that is denied.

**3. Which tests are fixtures and which are Rust.** VT-1 and VT-3 are parse
cases with a JSON input and one expected outcome each: fixtures, per §9
("fixtures are data files, not Rust literals"). **VT-2 is not** — resolution
takes four typed arguments, two of them `Option`, and encoding a `None` and a
`SignedDuration` as JSON buys nothing a reader of the protocol can use. VT-2 goes
in a colocated `#[cfg(test)] mod tests` in `schedule.rs`, following
`error.rs:163` and PHASE-02's precedent. That is inside the declared surfaces and
needs no amendment.

**4. `"next_check": null` is normalization's case, not this file's.** D50/R-51
make an explicit `null` identical to omission everywhere but `view`, and F-50
requires that it produce **no** discard. If `parse` were handed a `Value::Null`
it would return `NotAString { found: "null" }` — a discard, and the defect F-50
names. The rule is a *normalization-wide* one (D50 covers every modelled field),
so it belongs in `normalize.rs` where PHASE-04/EX-7 already owns it, not
per-field here. `parse`'s doc comment says so explicitly, so PHASE-04 meets the
obligation at the call site; PHASE-04/VT-4 and EX-7 are what hold it.

**An observation for PHASE-04, not a repair here.** PHASE-04's Surfaces
(`plan.md:447`) name `tests/protocol/fixtures/**` but neither `runner.rs` nor
`main.rs`. Its corpus needs a different harness function — `normalize_response`
rather than `parse` — which no fixture file can supply, so it will touch at least
one of them. EX-3 asks that PHASE-04 inherit the **format** without inventing a
second one, not that it add zero lines of Rust; the runner below is split so that
the format and the discovery are inherited and only the per-corpus checker is
new. Raise it at PHASE-04's expansion, not now.

#### Measured 2026-09-02, before the phase starts — the parse mechanism

Run in a scratch crate on jiff 0.2.35, `default-features = false`, in the dev
shell. Three things the sheet had left open are settled by measurement rather
than by reasoning. **This does not discharge VA-2** — that criterion asks the
*phase* to re-run the jiff behaviour, and it still must. What it does is remove
the unknowns that would otherwise have been met with fixtures already written
against them.

**1. `OutOfRange` is reachable. The risk is retired.** It was the `NotFinite`
shape (F-36, D39) — a named variant whose input JSON might not admit — and it is
not. jiff bounds each `Span` unit (`days` to `±7304484`), so a big enough span
fails at *parse* and lands in `Unparseable`; but between the parse bound and the
instant range there is a live window:

| input | outcome |
|---|---|
| `"1000000 days"` | ok — `4764-07-20T04:12:00Z` |
| `"2900000 days"` | ok — `9966-07-29T04:12:00Z` |
| `"3000000 days"`, `"7304484 days"`, `"1000000 weeks"` | **`OutOfRange`** — parses, converts, `checked_add` fails |
| `"10000000 days"` | `Unparseable` — outside the `Span` unit bound, so it never reaches the add |

`"1000000 weeks"` is the fixture: legible, and on the right side of both
boundaries. `Timestamp::MAX` is `9999-12-30T22:00:00.999999999Z`.

**2. The three failure kinds separate structurally, with no error-string
matching.** This is the part that would otherwise have been guessed:

- `Timestamp::from_str` fails on an offsetless instant with *"failed to find
  offset component"* — but the discriminator is not that message.
  `"2026-08-22T18:00:00"` parses cleanly as a `jiff::civil::DateTime` and
  `"tomorrow morning"` does not, so **`MissingOffset` is "civil parse succeeds
  where timestamp parse failed"** — a mechanism rather than a string comparison.
- `Span::to_duration(SpanRelativeTo::days_are_24_hours())` fails **only** for
  calendar units: `"1 month"`, `"1 year"`, `"1mo"` and `"1y"` all reach it and
  nothing else does. So `to_duration` erroring **is** `CalendarUnit`.
- `checked_add` erroring **is** `OutOfRange`.

Which fixes the dispatch order, unambiguously, because **no string parses as both
a civil datetime and a span**: absolute → civil (⇒ `MissingOffset`) → span
(⇒ `CalendarUnit` / `OutOfRange` / an instant) → `Unparseable`. A date with no
time, `"2026-08-22"`, parses civil and so lands in `MissingOffset`, which is the
right variant for it.

Confirmed alongside: `"1 day"`, `"1 week"` and `"1d 2h"` give exactly 24h, 168h
and 26h (VT-3); `"-45 minutes"` resolves to a past instant (EX-4); and
`Value::as_str()` is `None` for `45`, `null`, `true`, `[]`, `{}` and `45.5`
alike — which is A5, and is also why item 4 matters, since `null` would otherwise
become `NotAString`.

**3. One thing nobody asked about: `"18:00:00"` parses as a span of eighteen
hours.** A bare wall-clock time is neither of R-21's two forms, but jiff reads it
as `PT18H` under **either** dispatch order — it fails the civil parse, so
reordering does not help. A backend author writing `"18:00:00"` and meaning *six
this evening* gets *eighteen hours from now*, silently and successfully.

That is `MissingOffset`'s own argument — the most likely backend mistake deserves
a name — applied to a case §5.2 did not consider, and a sixth `ScheduleError`
variant is a design question rather than a phase repair (a STOP, and
`plan.md:437` says so). **Not invented here.** The phase adds a fixture asserting
the accepted behaviour, so it is a documented accept rather than a latent
surprise, and carries the question to audit.

#### Reading list

Read before writing anything. `path:line`.

| what | where | why |
|---|---|---|
| the phase itself | `docs/slices/001/plan.md:387` | criteria are binding as written |
| **what the schedule grammar accepts, and why days resolve while months do not** | `docs/slices/001/design.md:1739` | EX-1 is this paragraph. `SpanRelativeTo::days_are_24_hours()` is named there, and VA-2 re-runs it |
| the §5.5 edge rows this phase implements | `docs/slices/001/design.md:1707`–`:1715` | one VT-1 fixture per row |
| P2, and the two-clause granularity test | `docs/slices/001/design.md:205`, `:220` | why an invalid `next_check` is a discard and not an `Err`. The table at `:263` is the short form |
| brief §9 | `docs/brief.md:472` | the three arms of resolution, in the words R-26 restates. `:1040`–`:1041` is the "later supersedes earlier" half |
| `ScheduleError` as landed | `src/semantics/error.rs:80` | the five variants and their field names. Read the file, not §5.2 |
| why `MissingOffset` is broken out | `docs/slices/001/design.md:961` | it is a debuggability decision, not a taxonomy accident (brief §13) |
| `resolved_check` is not an `Option` | `docs/slices/001/design.md:1192` | why `resolve` takes `Option` anyway — item 2 |
| `Normalized` and `Discarded` | `docs/slices/001/design.md:858` | the caller this phase's `parse` is written for. Do not build it here |
| **fixtures are data files, and why** | `docs/slices/001/design.md:2022` | the corpus is reviewable protocol documentation, which is what makes it usable to the draft spec |
| the three test tiers | `docs/slices/001/design.md:2013` | this phase writes in the protocol tier, which reaches stratum 1 only |
| R-21…R-29 | `docs/slices/001/draft-spec.md:120`–`:128` | the requirements the fixtures are named for |
| their verification rows | `docs/slices/001/draft-spec.md:366`–`:368` | VT-1, VT-2 and EX-4 are these three rows |
| R-51 | `docs/slices/001/draft-spec.md:86` | the `null` rule — item 4. Read it to know what this file must **not** do |
| D4 | `docs/slices/001/design.md:1842` | jiff default features off. If something wants a time zone, stop |
| D53 as amended | `docs/slices/001/design.md:1890` | the module-level `arithmetic_side_effects` obligation. A4 |
| the vacuity guard, in code | `tests/protocol/boundary.rs` | the whole file. `Breach::Vacuous` and the aggregate report are what the runner copies |
| prior art for the module | `src/semantics/error.rs`, `src/semantics/protocol/canonical.rs` | doc-comment register, two-space indent, `#[cfg(test)] mod tests`, module-level lint attribute at the top |

#### Assumptions

Each is checkable; check it rather than proceeding on it.

- **A1 — the dev shell.** False on PHASE-01, and still a live trap: outside
  `nix develop` `just` resolves to `~/.nix-profile/bin/just`. Run every command
  as `nix develop --command bash -c '…'`, or verify `just` and `cargo` resolve
  into `/nix/store/` first. Verified 2026-08-30 for the baseline run above.
- **A2 — jiff 0.2.35 resolves days and weeks and rejects calendar units, with no
  tzdb.** Recorded from the F-10 probe (`notes.md`, *Established empirically*).
  **VA-2 requires re-running it, not citing it**, and this phase depends on it
  entirely. `Cargo.lock` still says 0.2.35, so the re-run is a confirmation
  rather than a re-derivation — but it is the confirmation the criterion asks
  for, and "the lockfile is unchanged" is not it.
- **A3 — nothing this phase lands is dead in the default-features column.**
  `dead_code` is `warn` in the manifest and the first clippy line passes
  `-D warnings`, so a private helper written before its caller fails `just check`
  (`design.md` §9). `resolve` and `parse` are `pub` in a `pub mod` and are
  reachable; a private helper is not. Order the work so nothing sits uncalled.
- **A4 — `schedule.rs` takes `#![deny(clippy::arithmetic_side_effects)]`.** It
  handles backend-derived data *and* does time arithmetic — it is the clearest
  case in the crate, and the reason the lint is per-module rather than crate-wide
  (D53 as amended). `canonical.rs:17` is the form. Consequence to expect: bare
  `+` on a `jiff::Timestamp` will not pass, which is correct — the overflow path
  **is** `OutOfRange`, so `checked_add` is the required spelling and the lint is
  what makes forgetting it impossible.
- **A5 — `parse` takes `&serde_json::Value`, not `&str`.** `NotAString` exists to
  report `"next_check": 45`, which means the function must see the untyped value
  to name what it found. `serde_json::Value::as_str` gives the string case;
  everything else is `NotAString { found: <the JSON type name> }`. `found` is
  `&'static str`, so the name comes from a match on the `Value` variant, not
  from formatting.
- **A6 — the fixture corpus runs in both feature columns.** `tests/protocol/` is
  built with `--no-default-features` too, so nothing in `runner.rs` may reach for
  tokio, and the fixture loader must be `std` plus `serde_json` plus `jiff`. That
  is the same rule the tier exists to enforce, applied to the harness itself.

#### STOP conditions

Stop and consult the user; do not improvise past any of these.

- Any dependency or dependency-feature change. A file-walking corpus loader is
  the obvious place a `walkdir`, `glob`, `insta` or `rstest` gets reached for;
  `std::fs::read_dir` over one flat directory is enough and is what the boundary
  scan already does.
- A clock read anywhere in `semantics/` — `Timestamp::now()`, `SystemTime::now()`,
  a default `now`. `now` is a parameter (I3), and this is the file where it is
  most tempting.
- A time zone, `jiff::tz`, or anything wanting jiff's default features (D4).
- Clamping a past `next_check` to `now`, or "correcting" a resolved instant in
  any direction. EX-4 and R-28 are exactly this, and F-13 is the finding that put
  them there.
- `max(retained, incoming)` as the latest-valid-wins rule. See item 2 — it is a
  semantics change, not a refactor.
- Any new `ScheduleError` variant, or a field added to one. §5.2 fixes the five;
  a sixth is a design question (the plan says so at `:437`).
- `parse` returning `Result<Option<Timestamp>, _>` to absorb `null` — item 4.
  That is PHASE-04's rule in the wrong file.
- Making `resolve` fallible, or `retained` non-optional in a way that forces
  `Host::new` to compute `now + default_poll` itself. R-27 and item 2.
- Anything outside the declared surfaces, including `src/semantics/mod.rs` until
  item 1 is decided.
- `HashMap`/`HashSet` — `clippy.toml` disallows both.

#### The fixture format

EX-3 requires this written down here, because PHASE-04 inherits it for a much
larger corpus. Designed for a reader who knows the protocol and not the tests.

**Reconciled with the code 2026-09-02, which is what discharges EX-3.** This
section describes `tests/protocol/runner.rs` as it shipped, not as it was
proposed; where execution changed the design the change is marked *(as landed)*.

**One case per file**, in a flat directory per corpus, named for the requirement
it verifies: `tests/protocol/fixtures/schedule/R-22-absolute-without-offset.json`.
The filename is the index — `ls` over the directory is a coverage report against
`draft-spec.md` §4, which is what makes the corpus usable as spec verification
rather than only as tests.

```json
{
  "requirement":  ["R-22", "R-25"],
  "description":  "An absolute instant with no UTC offset is rejected distinctly from a general parse failure.",
  "now":          "2026-08-23T04:12:00Z",
  "input":        "2026-08-22T18:00:00",
  "expect":       { "error": "MissingOffset" }
}
```

Five keys, and every one of them is required:

| key | shape | why |
|---|---|---|
| `requirement` | array of `R-N` ids, **non-empty** | a case may verify more than one, and an empty array is a case nobody can justify. *(as landed: the runner rejects the empty array rather than only discouraging it)* |
| `description` | one sentence, present tense, about the **protocol** | this is the half that makes the corpus documentation. "rejects an offsetless instant", not "asserts MissingOffset". It is what a failure report prints above the reason |
| `now` | RFC 3339 with offset | every case is deterministic. *(as landed: the runner parses it and hands the checker a `Timestamp`, so no corpus parses it twice.)* PHASE-04 needs it too — `normalize_response` takes `now` |
| `input` | any JSON value | corpus-specific: the raw `next_check` value here, a whole wire response at PHASE-04. Untyped, so `45` and `null` are expressible without a second key |
| `expect` | object of **exactly one** key | externally tagged, so the key names the outcome kind: `{"instant": …}` or `{"error": …}` here. PHASE-04 adds its own tags **to its own checker**, not to this table |

The **envelope is shared and the payload is not**. `runner.rs` owns
`requirement`, `description`, `now` and file discovery, and hands `input` and
`expect` back as `serde_json::Value`; each corpus supplies the function that
interprets them. That is what lets PHASE-04 add a corpus whose input is a whole
response without editing a single line that PHASE-03's cases go through — and it
is the reason `expect` is not a closed Rust enum in the runner.

*(as landed)* The seam is three items, and PHASE-04 needs no more:

```rust
type Check = fn(&Fixture<'_>) -> Result<(), String>;   // a corpus reads its own payload
struct Fixture<'a> { now: Timestamp, input: &'a Value, expect: &'a Value }
struct Corpus { root: &'static str, check: Check }     // + `assert_corpus(&CORPUS)`
```

`Ok(())` is the protocol behaving as the fixture says; the `Err` string is the
sentence printed under the description. One `#[test]` per corpus calls
`assert_corpus`.

*(as landed)* **The external tag is read by the shared half, not by each
corpus** — `outcome_tag` returns the single key and its value, and rejects an
`expect` carrying two. The tagging is the format's; only the tag *names* are the
corpus's. Without this a fixture claiming both `instant` and `error` would have
had one of its two claims silently unverified, since a checker looks for its
tags in some order and stops.

*(as landed)* **Failures are three kinds, because they are three different
questions**: `Vacuous` (nothing was found to run), `Malformed` (a file is not a
fixture, or the directory could not be enumerated — nothing was asserted, so no
protocol claim broke), and `Mismatch` (a fixture was read and the protocol did
not do what it says). Only `Mismatch` prints a `description`; the others have no
protocol claim to print.

Three properties the runner must have, all of them lessons already paid for.
Each was verified by breaking it — see the Log:

1. **A vacuity guard.** An empty or missing corpus directory **fails**, the way
   `boundary.rs` fails a scan that inspected nothing. A walk that finds no files
   and reports success is the defect this slice has already met once.
   *(as landed: the guard counts fixtures **found**, not fixtures **passed**. The
   obvious spelling — `if ran == 0`, incrementing `ran` on success — makes a
   corpus whose every case failed also claim to have run nothing, which is a
   second and false accusation on top of a real one.)*
2. **Unknown keys in a fixture are rejected** — `#[serde(deny_unknown_fields)]`
   on the envelope. Fixtures are ours, so strictness is free and it catches
   `expct` on the day it is typed. This is **not** in tension with I10/R-4: that
   rule is about the inbound *protocol*, where an unknown key is a backend using
   a newer host. A fixture is not a backend.
3. **Every failing case is reported, not the first.** The report names the file
   path and the `description`, so a run tells you which protocol claim broke
   without opening anything.

*(as landed)* **Where the corpus checker lives.** `check_schedule` and its
`#[test]` are in `runner.rs`, under a divider, below the shared half. The
declared surfaces give this phase no third file in `tests/protocol/`, and adding
one would have been a surface change rather than an implementation choice. The
file therefore has two halves and says so. If PHASE-04's corpus makes that
uncomfortable — and it is the more likely outcome — splitting the per-corpus
halves out is PHASE-04's call, taken with its own surfaces in hand. See the
observation for PHASE-04 above, which anticipated touching this file.

#### Tasks

Red / green / **refactor**. The refactor step is not optional.

1. **Entry check.** Confirm the shell (A1) and re-run `just check` green before
   touching anything, so a later failure is this phase's. All four items above
   are closed; there is nothing to wait on.
2. **VA-2 first, not last — re-run the jiff behaviour.** Scratch crate, jiff
   0.2.35, `default-features = false`. The 2026-09-02 measurement above says what
   to expect; VA-2 asks the phase to confirm it, and *expected* is not *observed*.
   Re-run all of it: the 24h / 168h / 26h exactness, `"1 month"` and `"1 year"`
   rejected with no tzdb, `"45 minutes"` and `"-45 minutes"` both resolving, the
   offsetless-instant / civil-parse discrimination, and `"1000000 weeks"` reaching
   `OutOfRange` while `"10000000 days"` reaches `Unparseable`. Paste the outputs
   into the Log. Every fixture in task 4 is written against what **this** run
   measured; if it disagrees with the table above, the table is what is wrong.
3. **GREEN — `parse`, one variant at a time, red first.** `pub fn parse(value:
   &serde_json::Value, now: Timestamp) -> Result<Timestamp, ScheduleError>`, with
   `#![deny(clippy::arithmetic_side_effects)]` at the top of the file (A4) and the
   doc comment that says `Value::Null` is normalization's case and not this
   function's (item 4). `NotAString` comes first, off `Value::as_str`; the string
   cases then take the measured dispatch — absolute → civil (⇒ `MissingOffset`)
   → span (⇒ `CalendarUnit` from `to_duration`, `OutOfRange` from `checked_add`,
   else an instant) → `Unparseable`. **No branch reads an error message.**
4. **RED then GREEN — the runner and the corpus, in that order.** Write
   `runner.rs` against a corpus of one, so the vacuity guard is exercised while
   the directory is genuinely nearly empty; then the corpus. VT-1's cases are
   §5.5's rows and `draft-spec.md:366`'s list: absolute with offset, absolute
   without; spans in minutes, hours, days and weeks; `"1 month"`;
   `"1000000 weeks"` for `OutOfRange`; `45`; `"tomorrow morning"`. Two more the
   measurement earned: `"10000000 days"`, which is `Unparseable` rather than
   `OutOfRange` and so shows the two boundaries are different; and `"18:00:00"`,
   accepted as eighteen hours — asserted so the behaviour is documented rather
   than latent (item 3 of the measurement). VT-3 is
   three of those asserting the **exact** resolved instants. EX-4 is a past
   absolute and `"-45 minutes"`, each asserting the instant is stored as given.
5. **GREEN — `resolve`, with VT-2 colocated.** The signature at item 2. Cases:
   incoming wins over retained; incoming wins **even when earlier** than retained
   (the `max()` trap); no incoming falls back to retained; neither present falls
   back to `now + default_poll`; and the invalid-preserves-existing case, named so
   it says that `None` here means "discarded upstream".
6. **RED again, by break-and-revert.** Two breaks, because two mechanisms are
   being claimed. (a) Weaken `parse` so `MissingOffset` returns `Unparseable`;
   confirm exactly the offsetless fixture fails and nothing else; revert.
   (b) **Break the runner, not a fixture** — point the corpus at a directory that
   does not exist and confirm the vacuity guard fails rather than passing with
   zero cases; revert. PHASE-01's standard: a criterion that names a mechanism is
   not yet a criterion that has one, and the guard is the mechanism the whole
   corpus rests on.
7. **REFACTOR.** The span path and the absolute path share `now` and share the
   overflow check; the three arms of `resolve` are brief §9's three sentences and
   should read as them. Watch A3 — a helper extracted before its second caller
   exists is dead code and fails the gate.
8. **VA-1.** `just check` exits 0, six commands, both feature columns. Paste it.
   The `--no-default-features` column is the one that proves the corpus runner
   reaches nothing it should not.
9. **Bookkeeping before handing off.** EX-3 is discharged by *The fixture format*
   above being true of what shipped — reconcile it with the code rather than
   leaving it as a proposal. Status table, this sheet kept current as you go,
   `## Harvest` updated in place.

#### Verification record

All discharged 2026-09-02. Evidence is pasted below the table, not summarised in
it.

| id | mode | result | evidence |
|---|---|---|---|
| VT-1 | test | **pass** | the corpus: 16 fixtures under `tests/protocol/fixtures/schedule/`, one `#[test]` (`runner::every_scheduling_fixture_states_what_the_protocol_does`). Every case in the plan's list is present — absolute with offset and without; spans in minutes, hours, days and weeks; `"1 month"`; out of range; `45`; `"tomorrow morning"` — each asserting its own variant. E1, E2 |
| VT-2 | test | **pass** | five colocated tests in `schedule.rs`, all five red before `resolve` had a body and green after. Includes the `max()` trap (`incoming < retained` and `incoming` still wins) and invalid-preserves-existing. E3, E5 |
| VT-3 | test | **pass** | `R-24-relative-span-in-days.json`, `R-24-relative-span-in-weeks.json`, `R-24-compound-span-of-days-and-hours.json` assert the **exact** resolved instants — `2026-08-24T04:12:00Z`, `2026-08-30T04:12:00Z`, `2026-08-24T06:12:00Z` from `now = 2026-08-23T04:12:00Z`, i.e. exactly 24h, 168h and 26h. E1 |
| VA-1 | agent | **pass** | `just check` exits 0, six commands, both feature columns. The corpus runs in the `--no-default-features` column too, which is A6 discharged rather than asserted. E4 |
| VA-2 | agent | **pass** | re-run, not cited: scratch crate on jiff 0.2.35 with `default-features = false`, every row of *Measured 2026-09-02* reproduced. `SpanRelativeTo::days_are_24_hours()` resolves days and weeks exactly and rejects months and years, with no tzdb. E0 |
| EX-1 | test | **pass** | all five `ScheduleError` variants reachable from a fixture, plus the accepting cases. Asserted against `error.rs`'s variant names via `error_name`'s exhaustive match, so a sixth variant cannot be added without an arm. Break (a) confirms the `MissingOffset` branch is load-bearing. E1, E2 |
| EX-2 | test | **pass** | `resolve(retained, incoming, default_poll, now) -> Timestamp` — pure, total, concrete return, no clock. Three arms, `schedule.rs:146`. VT-2 covers all three plus the two traps. E3 |
| EX-3 | review | **pass** | *The fixture format* above was reconciled with the shipped `runner.rs` rather than left as a proposal; four *(as landed)* corrections are marked in it. The runner's three required properties were each verified **by breaking them** — E2 |
| EX-4 | test | **pass** | `R-28-absolute-instant-already-past.json` (a past absolute stored unchanged) and `R-28-negative-span.json` (`"-45 minutes"` → `2026-08-23T03:27:00Z`). Nothing clamps: `parse_instruction` returns the parsed instant directly and the only arithmetic is the span addition. E1 |

**E0 — VA-2, the jiff re-run.** Scratch crate, `jiff = { version = "=0.2.35",
default-features = false }`; its `Cargo.lock` and goad's both say 0.2.35.

```
Timestamp::MAX = 9999-12-30T22:00:00.999999999Z

-- 1. exactness: 1 day / 1 week / 1d 2h (VT-3, F-10, D28) --
       1 day -> 86400s  (24h) expected 24h  exact=true
      1 week -> 604800s  (168h) expected 168h  exact=true
       1d 2h -> 93600s  (26h) expected 26h  exact=true

-- 2. calendar units rejected, no tzdb (R-23, D4) --
     1 month -> to_duration Err: using unit 'month' in span or configuration requires that a relative reference time be given (`jiff::SpanRelativeTo::days_are_24_hours()` was given but this only permits using days and weeks without a relative reference time)
      1 year -> to_duration Err: using unit 'year' … (same)
         1mo -> to_duration Err: … (same)
          1y -> to_duration Err: … (same)

-- 3. minutes, both signs (EX-4, R-28) --
    45 minutes -> instant 2026-08-23T04:57:00Z
   -45 minutes -> instant 2026-08-23T03:27:00Z

-- 4. the structural discrimination (no error-string matching) --
      2026-08-23T05:00:00Z  timestamp=true  civil=false span=false -> instant 2026-08-23T05:00:00Z
  2026-08-23T05:00:00+10:00  timestamp=true  civil=true  span=false -> instant 2026-08-22T19:00:00Z
       2026-08-22T18:00:00  timestamp=false civil=true  span=false -> MissingOffset
                2026-08-22  timestamp=false civil=true  span=false -> MissingOffset
          tomorrow morning  timestamp=false civil=false span=false -> Unparseable
                  18:00:00  timestamp=false civil=false span=true  -> instant 2026-08-23T22:12:00Z

-- 5. the two range boundaries are different (OutOfRange vs Unparseable) --
      1000000 days -> instant 4764-07-20T04:12:00Z
      2900000 days -> instant 9966-07-29T04:12:00Z
      3000000 days -> OutOfRange (parameter 'Unix timestamp seconds' is not in the required range of -377705023201..=253402207200)
      7304484 days -> OutOfRange (same)
     1000000 weeks -> OutOfRange (same)
     10000000 days -> Unparseable (parameter 'days' is not in the required range of -7304484..=7304484)

-- 6. as_str is None for every non-string JSON value (A5) --
          45 -> as_str = None  (variant number)
        null -> as_str = None  (variant null)
        true -> as_str = None  (variant boolean)
          [] -> as_str = None  (variant array)
          {} -> as_str = None  (variant object)
        45.5 -> as_str = None  (variant number)
     "1 day" -> as_str = Some("1 day")  (variant string)
```

Every row matches *Measured 2026-09-02*. The table is not wrong. The one thing
it did not say is the `timestamp=true civil=true` row — see the Log.

**E1 — the corpus, as `ls` reads it.** The filename is the index, so this is the
coverage report against `draft-spec.md` §4:

```
R-21-absolute-with-offset.json
R-21-bare-wall-clock-time.json
R-21-relative-span-in-hours.json
R-21-relative-span-in-minutes.json
R-22-absolute-without-offset.json
R-23-calendar-unit-months.json
R-23-calendar-unit-years.json
R-24-compound-span-of-days-and-hours.json
R-24-relative-span-in-days.json
R-24-relative-span-in-weeks.json
R-25-not-a-string.json
R-25-span-beyond-the-grammar-s-unit-bound.json
R-25-span-leaving-the-representable-range.json
R-25-unparseable-prose.json
R-28-absolute-instant-already-past.json
R-28-negative-span.json
```

**E2 — task 6, break and revert. Five breaks, not two**, because the format
section makes three property claims and each needed one. Every break was
reverted and the gate re-run green after each.

*(a) `MissingOffset` weakened to `Unparseable`* — exactly one fixture fails, and
the report names the file and the protocol claim rather than a line number:

```
tests/protocol/fixtures/schedule/R-22-absolute-without-offset.json: An absolute instant with no UTC offset is rejected distinctly from a general parse failure.
    expected MissingOffset, got Unparseable (unparseable schedule: 2026-08-22T18:00:00)
test result: FAILED. 4 passed; 1 failed
```

*(b) the corpus root pointed at a directory that does not exist* — the vacuity
guard, in the literal shape `boundary.rs` was written for. Both faults are
reported, because a directory can fail to be read **and** yield nothing:

```
tests/protocol/fixtures/schedule-renamed-away: the corpus directory could not be read: No such file or directory (os error 2)
tests/protocol/fixtures/schedule-renamed-away: ran no fixtures — renamed, emptied, or misspelled
```

Run again against a directory that **exists and holds no fixtures**, the second
line appears alone. Both shapes fail; neither passes with zero cases.

*(c) `CalendarUnit` weakened* — two fixtures claim it, and **both** are named in
one run. That is property 3, which break (a) could not have shown:

```
…/R-23-calendar-unit-months.json: A span in months is rejected with its own error: …
    expected CalendarUnit, got Unparseable (unparseable schedule: 1 month)
…/R-23-calendar-unit-years.json: A span in years is rejected for the same reason as one in months, …
    expected CalendarUnit, got Unparseable (unparseable schedule: 1 year)
```

*(d) six deliberately malformed fixtures added* — property 2, and the envelope's
other guards. All six rejected, all six reported in one run:

```
ZZ-typo.json:           not a fixture: unknown field `expct`, expected one of `requirement`, `description`, `now`, `input`, `expect` at line 6 column 9
ZZ-no-requirement.json: not a fixture: `requirement` names no R-N id
ZZ-bad-now.json:        not a fixture: `now` is not an RFC 3339 instant: sometime
ZZ-empty-expect.json:   `expect` must be an object of exactly one key naming the outcome kind, found 0
ZZ-two-claims.json:     `expect` must be an object of exactly one key naming the outcome kind, found 2
ZZ-unknown-tag.json:    `expect` names `discard`, which this corpus does not read
```

*(e) `parse` short-circuited so that every case fails* — the guard must **not**
also claim vacuity, because the corpus did run. 14 fixtures reported (the two
that expect `Unparseable` still pass), and no `ran no fixtures` line. This break
is what found the defect the refactor fixed; see the Log.

**E3 — VT-2 red, before `resolve` had a body.** With the body replaced by
`now`, all five fail and nothing else does:

```
test …::a_valid_incoming_instruction_supersedes_the_retained_one ... FAILED
test …::a_valid_incoming_instruction_wins_even_when_it_is_earlier_than_the_retained_one ... FAILED
test …::an_instruction_discarded_upstream_arrives_as_none_and_preserves_the_retained_value ... FAILED
test …::with_no_incoming_instruction_the_retained_value_stands ... FAILED
test …::with_nothing_retained_and_nothing_incoming_the_default_poll_is_added_to_now ... FAILED
test result: FAILED. 7 passed; 5 failed
```

**E4 — VA-1, `just check`.** Six commands, both columns, exit 0:

```
cargo build
cargo test                          22 passed (lib) · 5 passed (protocol) · 0 doc-tests
cargo test --no-default-features    22 passed (lib) · 5 passed (protocol)
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub
cargo fmt --check
just check exit=0
```

22 lib tests, 5 of them this phase's (VT-2). 5 protocol tests, 1 of them this
phase's — the corpus is one `#[test]` over 16 files. The
`--no-default-features` column running the corpus is A6 discharged: the runner
reaches nothing above stratum 1.

**E5 — the surfaces actually touched**, against the plan's list:

```
M src/semantics/mod.rs        (one line, `pub mod schedule;`)
M tests/protocol/main.rs      (three lines, `#[cfg(test)] mod runner;`)
? src/semantics/schedule.rs
? tests/protocol/runner.rs
? tests/protocol/fixtures/schedule/*.json   (16 files)
```

No undeclared path. Nothing else in `src/` or `tests/` was edited.

#### Log

<!-- Append as you go: decisions taken, obstacles, anything noticed in passing.
     Do not save it for the end. -->

- **2026-08-30, sheet written.** Entry criterion EN-1 checked against
  `canonical.rs:145` rather than the status table; baseline `just check` green in
  a fresh `nix develop`, `just` and `cargo` both store paths. Four plan gaps
  found while expanding — three settled in the sheet and recorded so PHASE-04 and
  PHASE-07 inherit them, one taken to the user.
- **2026-09-02, item 1 closed.** User decided the class fix: PHASE-03, PHASE-04
  and PHASE-07 Surfaces all gained their parent `mod` file in one edit
  (`plan-log.md`). PHASE-05 needed none — it already declared both.
- **2026-09-02, the jiff unknowns measured before starting.** `OutOfRange` is
  reachable — `"1000000 weeks"` — so the F-36-shaped risk is retired rather than
  carried into the corpus. The three failure kinds separate structurally
  (`to_duration` ⇒ `CalendarUnit`, `checked_add` ⇒ `OutOfRange`, civil-parse
  success ⇒ `MissingOffset`), so no branch reads an error message. Found in
  passing: `"18:00:00"` is accepted as a span of eighteen hours. Full table under
  *Measured 2026-09-02* above; VA-2 still re-runs it in the phase. One risk pre-flagged: `OutOfRange` may be
  unreachable from the wire, in the shape F-36 already established for
  `NotFinite`, and task 2 settles it by measurement before any fixture asserts it.

- **2026-09-02, entry check (task 1).** `nix develop --command bash -c 'which
  just cargo; just check'` — `just` at
  `/nix/store/ni2dxycnhsp34y4qy6q44nw6pp6bj0l0-just-1.58.0/bin/just`, `cargo` at
  `/nix/store/cyn97lq74y3lx15y95gyzplnmmx451g9-rust-default-1.99.0-beta.1-2026-08-18/bin/cargo`.
  A1 verified. `just check` exits 0 — 17 unit tests, 4 boundary tests, both
  clippy columns clean, `cargo fmt --check` silent. Any failure from here is
  this phase's.
- **2026-09-02, VA-2 re-run (task 2), before a line of the corpus was written.**
  Scratch crate under the session scratchpad, `jiff = { version = "=0.2.35",
  default-features = false }`, lockfile confirmed at 0.2.35 — the same version
  `Cargo.lock` pins. **Every row of *Measured 2026-09-02* reproduced.** Output
  pasted under the Verification record.
  - One thing the measurement table did not say, found by printing all three
    parse results per input rather than only the classification:
    `"2026-08-23T05:00:00+10:00"` parses as a `Timestamp` **and** as a
    `civil::DateTime`. So it is not true that the three parsers partition the
    input space — only that *civil* and *span* do. The dispatch is unaffected,
    because absolute is tried first and wins, which is the order the sheet
    already fixes; but "absolute first" is therefore load-bearing rather than
    incidental, and `parse` carries a comment saying so. The sheet's claim as
    written — "no string parses as both a civil datetime and a span" — is
    accurate; it just is not the whole partition story.
- **2026-09-02, tasks 3 and 5 — the red/green record.** Seven cycles, each red
  before it was green: `NotAString`; an absolute instant with an offset (which
  killed the deliberate `Ok(now)` stub); `MissingOffset`; a span resolving
  against `now`; `CalendarUnit`; `OutOfRange` **with** its `Unparseable`
  neighbour in the same test, so the two boundaries are asserted to be
  different; then `resolve`'s five VT-2 cases.
  - **Two branches were written ahead of their tests and backed out.** The span
    cycle landed `CalendarUnit` and `OutOfRange` in one edit because they sit in
    the same dispatch. Both were reverted to `Unparseable`, driven red
    separately, and restored. `resolve` got the same treatment for the same
    reason — it arrived during a doc-comment pass, so its body was replaced by
    `now` until VT-2 was red against it (E3). Cheap, and the alternative is a
    branch nothing ever proved was load-bearing.
- **2026-09-02, AC-11's vocabulary scan fired twice, on the word "site".** Not a
  false positive worth carving out — the scan is deliberately blunt and
  substring-matched, and it reaches doc comments and ordinary comments under
  `src/`, which is where prose lives. Both hits were mine, both in the ordinary
  software sense ("the one site that can apply it", "left standing at half its
  sites"), and both were reworded. Cost about a minute each; the alternative is
  a `#[allow]` on a scan whose whole value is that it has no exceptions.
  **PHASE-04 and PHASE-09 will hit this**, because "site" is unavoidable when
  writing about where a rule is applied. Harvested.
- **2026-09-02, task 4 — the runner, and where its corpus lives.** The declared
  surfaces give this phase no third file under `tests/protocol/`, so
  `check_schedule` and its `#[test]` are in `runner.rs` below a divider, with
  the shared half above it. That is inside the surfaces; a new file would not
  have been. Recorded in *The fixture format* so PHASE-04 inherits the split
  rather than the accident, and it is PHASE-04's call whether to separate them
  once its own corpus is in hand.
- **2026-09-02, task 7 — the refactor found two real defects, not just tidying.**
  This is the step the sheet says is not optional, and it earned its place:
  1. **`expect` was documented as a single-key object and enforced as neither.**
     The checker looked for `instant`, then for `error`, and returned on the
     first it found. A fixture carrying both would have had one claim silently
     unverified. Fixed by moving the external-tag read into the shared half —
     `outcome_tag` returns the one key or refuses — which also puts the tagging
     discipline where PHASE-04 inherits it instead of reimplementing it. Break
     (d) is the proof.
  2. **The vacuity guard counted the wrong thing.** It incremented on cases that
     *passed* and fired when that count was zero, so a corpus whose every case
     failed would have been accused of running nothing as well — a false second
     finding stacked on a real one, in the report a reader trusts to tell them
     what broke. Now it counts fixtures **found**. Break (e) exists to hold this,
     and it is the break the sheet did not ask for.
  - The sheet's refactor prompt is half wrong about the code, and the note
    belongs here rather than silently ignored: "the span path and the absolute
    path share `now` and share the overflow check" — they do not. The absolute
    path does no arithmetic at all; it parses to an instant and returns it, which
    is precisely why R-28 is free rather than enforced. The only `checked_add` on
    the parse side is the span one. Nothing was extracted, because extracting a
    helper with one caller is dead code and A3 fails the gate on it.
- **2026-09-02, `-D warnings` over `pedantic` cost three fixes worth knowing.**
  `missing_errors_doc` fires on any `pub fn` returning `Result` — the manifest
  comments say the doc lints are "paused", but `pedantic = deny` re-enables this
  one, so `parse` needed an `# Errors` section (`canonical.rs:380` is the
  house form). `unused_self` on a method that never touched `self`, and
  `type_complexity` on `Corpus<fn(&Fixture<'_>) -> Result<(), String>>`. The
  second and third were fair: the method became a free function and the generic
  parameter became a `type Check` alias, which left `Corpus` non-generic and
  simpler than it started.
- **2026-09-02, observation for the audit, not a repair.** `report()` in
  `runner.rs` is identical to `report()` in `boundary.rs`, and the two
  aggregate-then-panic shapes are the same shape. `boundary.rs` is **not** a
  declared surface here, so factoring it out would have been a surface change;
  it is left duplicated deliberately. PHASE-09 extends `boundary.rs` and is the
  natural place to take it, if anyone judges two copies worth removing.
- **2026-09-02, one bookkeeping mishap, recorded because the fix is generic.**
  An edit to *The fixture format* anchored its end on `"#### Tasks"`, which
  matched **PHASE-01's** heading rather than PHASE-03's — the sheets all share
  the same subheadings, so the first match is nine hundred lines too early. The
  result was a duplicated tail, caught immediately by re-reading the heading map
  and repaired by reconstruction; verified against `git show HEAD` that no
  section was lost. In a file of repeated section names, anchor on the enclosing
  `### PHASE-NN` first, or on a unique string.


### PHASE-04 — Wire types, normalization, and the protocol corpus

**State:** **sheet written 2026-09-02; not started.** Entry criteria checked and
met. **Three plan gaps found at expansion, all needing a user decision before
execution begins** — two amend `plan.md`'s Surfaces and one corrects a path in a
verification criterion. They are items 1–3 below. Nothing else in the phase is
blocked on them.
**Plan entry:** `docs/slices/001/plan.md:449`
**Surfaces (from the plan):** `src/semantics/protocol/mod.rs` (the two `pub mod`
lines, and its doc comment), `src/semantics/protocol/wire.rs`,
`src/semantics/protocol/normalize.rs`, `tests/protocol/fixtures/**`.
**Surfaces the expansion says are missing:** `tests/protocol/runner.rs` and
`tests/protocol/main.rs` — item 1.

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | PHASE-03/EX-1 discharged (`parse` accepts both forms and rejects with the five named variants) | **met**, verified by reading `src/semantics/schedule.rs:66` (`parse`) and `:79` (`parse_instruction`), and by the 16-file corpus at `tests/protocol/fixtures/schedule/`, which names all five variants across its `expect.error` tags |
| EN-1 | PHASE-03/EX-3 discharged (the runner walks data files; its format is documented so this phase inherits rather than invents) | **met.** `tests/protocol/runner.rs` ships the shared half — `Envelope`, `Fixture`, `Corpus`, `Check`, `outcome_tag`, `assert_corpus`, the three-kind `Fault` and the vacuity guard. *The fixture format* above is reconciled with the code that shipped, with the four *(as landed)* corrections marked |

Baseline, 2026-09-02: `just check` exits 0 on all six commands, both feature
columns — 22 unit tests, 5 protocol tests, integration skipped in the stratum 1
column. PHASE-03 is committed at `2648a17`. Any failure from here is this phase's.

#### What already exists — inspected 2026-09-02

| path | state | consequence for this phase |
|---|---|---|
| `src/semantics/protocol/canonical.rs` | 863 lines. Every type EX-2 must produce. `Options::new(Vec<Opt>, at: &str)` at `:387`, `Alternatives::new` at `:417`, `Fields::new` at `:446`, `NumberRange::new(Option<f64>, Option<f64>)` at `:481` | **the checked constructors are normalization's error paths.** `at` is passed in by the caller, so path accumulation is this phase's and the constructors already take it |
| — the `pub(super)` fields | `Response:203`, `Choice:224`, `Opt:245`, `Field:276`, `Alternative:317` | `normalize.rs` is a sibling module under `semantics::protocol`, so `pub(super)` on `canonical`'s fields reaches it. Struct-literal construction works; **no new constructor is needed and none may be added** — widening is R10 |
| — the scalar minting constructors | `OptionId::new:74`, `AlternativeId::new:103`, `FieldId::new:131`, `Hints::new:186` — all `pub(super)` | the four things normalization mints. `ViewId::new:46` and `Timestamp::new:148` are `pub` because the host authors them; this phase mints neither |
| `src/semantics/error.rs` | `ProtocolError` at `:17`, twelve variants; `BoundsError` at `:65`; `ScheduleError` at `:80` | EX-3, EX-6 and VT-2 assert **against this file**, not against §5.2. `InapplicableKey { key: &'static str, kind: String, at: String }` at `:31` — `key` is `&'static str`, so the four keys it can name are a closed set: `min`, `max`, `options`, `fields` |
| `src/semantics/protocol/mod.rs` | 7 lines; `pub mod canonical;` only. Its doc comment says "`wire` and `normalize` arrive in PHASE-03 and PHASE-04" | both arrive here. The comment is in the declared Surfaces and this phase is the one that can make it true |
| `src/semantics/schedule.rs` | `parse(&Value, Timestamp) -> Result<Timestamp, ScheduleError>` at `:66` | called by `normalize_response`. **Its doc comment states an obligation this phase owes** — see item 5 |
| `tests/protocol/runner.rs` | the shared half above a divider, `check_schedule` + `const SCHEDULE` + one `#[test]` below it | the seam this phase extends. `Corpus { root, check }` is a value, so a second corpus is a second `const` and a second `#[test]` — **no change to the shared half is needed**, which is the property PHASE-03 built it for |
| `tests/protocol/fixtures/schedule/` | 16 files, flat, filename-indexed | the naming convention this phase follows: `R-N-what-it-verifies.json` |
| `#![deny(clippy::arithmetic_side_effects)]` | `canonical.rs:17`, `schedule.rs:14` | `wire.rs` and `normalize.rs` take the same attribute — EX-5 |

#### Three plan gaps — user decision needed before execution

**1. The Surfaces do not name `tests/protocol/runner.rs` or
`tests/protocol/main.rs`, and the corpus cannot exist without them.** The plan
gives this phase `tests/protocol/fixtures/**` and no Rust under `tests/`. But a
fixture file asserts nothing on its own: the protocol corpus needs a checker
function reading its own `expect` tags, a `const` naming its directory, and a
`#[test]` calling `assert_corpus` — all Rust, all in `runner.rs`, and a new file
under `tests/protocol/` would need `main.rs` to declare it.

This is **the same class as the `mod`-line omission closed on 2026-09-02**: a
phase's Surfaces listing the data it adds and not the declaration that reaches
it. PHASE-03's sheet anticipated exactly this and deferred it here rather than
patching the plan early ("*An observation for PHASE-04, not a repair here*").

Recommendation: **amend PHASE-04's Surfaces to add `tests/protocol/runner.rs`
and `tests/protocol/main.rs`**, and leave the split between them to execution.
*The fixture format* already says the one-file-two-halves arrangement is
"PHASE-04's call, taken with its own surfaces in hand" — naming both files is
what puts that call in hand. Adding a third file (`tests/protocol/normalize.rs`,
say) then needs `main.rs`, which is why it is named too.

**2. VA-2 names a file that does not exist.** It reads "an `unwrap()` in
`src/semantics/normalize.rs`". The file is `src/semantics/protocol/normalize.rs`
— `normalize` is under `protocol/`, per EX-1 and the Surfaces line three
paragraphs above it in the same phase entry. A path correction, not a change of
intent; raised rather than absorbed because it is plan text.

**3. VT-2's `NaN` fixture cannot be written in the format this phase inherits —
and this is not a wording problem.** VT-2 requires a fixture asserting
`Protocol(Json)` for a `NaN` literal, per F-36/D39. The envelope's `input` is a
`serde_json::Value`, and the whole fixture file is read with
`serde_json::from_str`. So a file containing `"input": {"min": NaN}` **fails at
envelope parse** and lands as `Fault::Malformed` — "not a fixture" — never
reaching the checker and never asserting the protocol claim. The case would pass
into the corpus looking like coverage and be none.

**Measured 2026-09-02**, scratch crate, serde_json 1, rather than reasoned:

| input | reading it as a `WireField` | reading the enclosing envelope |
|---|---|---|
| `{"min": 1e400}` | `ERR` — `number out of range at line 1 column 13` | `ERR` — same error, at the envelope |
| `{"min": -1e400}` | `ERR` — `number out of range` | — |
| `{"min": NaN}` | `ERR` — `expected value at line 1 column 9` | `ERR` — same error, at the envelope |
| `{"min": 1e308}` | `OK` — `Some(1e308)`, finite | — |

Two things this settles. The design's F-36 measurement **holds**: neither literal
reaches bounds validation, so `NotFinite` stays unreachable from the wire and
D39's "keep it anyway, and assert `Protocol(Json)` instead" is correct as
written. And the reason it holds is the same reason the fixture cannot be
structured — serde_json refuses both at *parse*, wherever the text sits.

This one is **implementer latitude, not a plan change**, and it is settled in
item 6 below. It is listed here because it changes what the corpus looks like and
because the plan's VT-2 reads as though one uniform corpus discharges it.

#### Settled here — implementer latitude PHASE-05 and PHASE-10 inherit

**4. Two corpora, not one, and the second exists only for text serde_json will
not parse.** From item 3.

- **`fixtures/protocol/` — structured.** `input` is the wire response as a JSON
  value, exactly as a backend would emit it. The checker does
  `serde_json::from_value::<WireResponse>(input)` and then
  `normalize_response(wire, now)`. This is every fixture but two, and it is the
  one that has to read as protocol documentation (AC-9, §9).
- **`fixtures/protocol-text/` — raw.** `input` is a JSON **string** carrying the
  document text verbatim; the checker does `serde_json::from_str`. Only the two
  literals JSON cannot hold live here: `NaN` and `1e400`. The escaping that makes
  this form unreadable is exactly why it is not the default.

Both are `Corpus` values over the **unchanged** shared half — a second `const`
and a second `#[test]`. That is PHASE-03's split doing the job it was built for,
and the vacuity guard then covers each directory separately, which is a small
gain: emptying either one fails on its own.

Rejected: one corpus with string inputs throughout — it destroys the reviewable-
as-documentation property AC-9 turns on. Also rejected: dropping the two cases to
colocated Rust tests — §9 says fixtures are data files, and the two cases are
protocol claims like any other.

**Not this phase's:** R-38's "empty stdout, two documents on stdout" fixtures are
about reading one document off a pipe. `normalize_response` takes a
`WireResponse`, so framing is the transport's — PHASE-05.

**5. `next_check: null` is elided before `parse` is called, and this phase owes
it.** PHASE-03 recorded the obligation and could not discharge it:
`schedule::parse` handed a `Value::Null` returns `NotAString { found: "null" }`,
which is a **discard**, and R-51 forbids one here. D50's rule is
normalization-wide, so it belongs at the one place that can apply it uniformly.
Concretely: `WireResponse.next_check` is `Option<Value>`, and serde maps both
omission and explicit `null` to `None` — verified in PHASE-03 — so the elision is
already structural for this field. **What EX-7 must prove is that it is, not that
it was intended**: the fixture pair is `next_check` omitted and `next_check: null`
producing an identical outcome with an **empty discard list**, and `next_check:
45` still discarded and reported. The assertion is the silence.

**6. The `at` path grammar.** §6 leaves accumulation to implementation and the
contract is "the named error, the retained string, the path". Settled as the
dotted/bracketed form the design's own examples use —
`view.options[1].fields[2].kind` — because §5.2 writes it that way twice and a
second spelling would make the corpus's asserted strings disagree with the
design's prose. Fixtures assert the path **literally**, which is what makes it a
contract rather than a convenience.

**7. `WireView` is named in §5.2 and defined only in §6.** `WireResponse.view` is
`Option<Option<WireView>>` and §5.2's block defines `WireChoice`, `WireOpt` and
`WireField` — not `WireView`. §6 offers the shape as implementation latitude:
`struct WireView { kind: String, #[serde(flatten)] rest: serde_json::Value }`,
dispatched on `kind`. That is the encoding EX-3 needs, since a view's unrecognised
`kind` must be `UnsupportedPrimitive` and not a serde error. Taken as offered.
Recorded rather than raised: §6 is explicit that the mechanism is not mandated, so
this is latitude being exercised, not a hole. Worth an audit note only if §9's
"every type named in §5 is defined in §5" sweep is run again — this is the same
shape as F-55's `WireOpt` and F-56's `cleanup_only`.

#### Assumptions — each is a place this phase breaks

- **A1 — the dev shell.** Same as PHASE-01/A1, PHASE-02/A1 and PHASE-03/A1, and
  it was false at PHASE-01. Confirm `just`, `cargo` and `rustc` resolve into
  `/nix/store/` before believing any gate result.
- **A2 — `#[serde(flatten)]` at depth does not disturb the `deserialize_with`
  on `view` above it.** §6 says this was run. It is the single interaction the
  whole wire shape rests on, and **this phase re-runs it** rather than citing the
  design — the PHASE-03 precedent, where re-measurement retired one risk and
  fixed the dispatch order. Cheap: one struct, one round of four inputs.
- **A3 — serde binds `min`, `max` and `options` before `kind` is dispatched, so
  a misplaced modelled key cannot fall through to `hints`.** This is F-45's whole
  premise and EX-6 is built on it. If it is false, `InapplicableKey` is
  unreachable for those three keys and the design's cost argument for D37
  changes. Re-run alongside A2.
- **A4 — a misspelled *required* key still fails after flattening.** VT-3's
  `labell` case. Design measured `missing field 'label'`; re-run with A2.
- **A5 — `serde_json::from_value` and `from_str` produce the same errors for the
  same document.** The two corpora in item 4 take different routes into
  `WireResponse`, and if they diverge, a fixture's assertion depends on which
  directory it sits in. Check once, at the start.

#### STOP conditions

Stop and consult the user, do not improvise past:

- Any of items 1–3 not yet decided when execution reaches the work they govern.
- A `ProtocolError` variant that turns out to need a field the design did not
  give it — that is a design question, as it was at PHASE-03.
- A2, A3 or A4 measuring false. Each is a design premise, not an implementation
  detail, and A3 measuring false unmakes EX-6.
- Wanting a constructor or a wider field on `canonical.rs`. It is a declared
  surface **only** via `protocol/mod.rs`; `canonical.rs` itself is not listed,
  and R10 is the named risk.
- The corpus needing a sixth envelope key. The format is PHASE-03's and shared;
  changing it changes the scheduling corpus too.
- Discovering a second `Discarded` variant is wanted. D10 makes that a
  deliberate argument against P2's eligibility test, not a local addition.

#### Tasks

Red / green / **refactor**, and the refactor step is where PHASE-03 found two
real defects. Order is by dependency, not by criterion number.

1. **Baseline.** `just check` green, tools in `/nix/store/`. A2–A5 measured in a
   scratch crate before any wire type is written.
2. **`protocol/mod.rs`** — add `pub mod normalize;` and `pub mod wire;`, and fix
   the doc comment's PHASE-03 claim. One edit, and it is the phase's smallest.
3. **`wire.rs`** — EX-1. `#![deny(clippy::arithmetic_side_effects)]` at the top
   (EX-5). `WireResponse`, the `present` helper, `WireView`, `WireChoice`,
   `WireOpt`, `WireField`. No `deny_unknown_fields` anywhere (I10).
4. **`normalize.rs`** — EX-2's signature, `Normalized<T>`, `Discarded`. Same
   module deny. Build it error-path first: EX-3's three discriminant sites, then
   EX-6's applicability check, then the checked constructors, then EX-7's null
   rule (item 5), then the happy path.
5. **The corpus** — EX-4, EX-8, VT-1…VT-4. Two `Corpus` consts (item 4), two
   `#[test]`s. Drive each fixture red before it goes green: a fixture written
   against code that already passes it has asserted nothing.
6. **Break-and-revert** — VA-2, in both forms, in **host** code, at the corrected
   path from item 2. `clippy.toml` carves the no-panic lints out of `tests/`, so
   an `unwrap()` under `tests/` proves nothing (F-14). PHASE-03 ran five cycles
   against a sheet that asked for two; the count follows the number of distinct
   property claims, not the sheet.
7. **Refactor**, then bookkeeping in place — status table, this sheet's state,
   the Verification record with pasted evidence, the Log, the Harvest.

#### Corpus inventory — what EX-4 owes, sourced from `draft-spec.md` §7

The plan says "the corpus covers `design.md` §9's protocol-tier list". §9's
enumeration is its **misbehaving-backend** list, which is written for the
integration tier and mixes transport cases in. The protocol-tier enumeration that
is actually complete is `draft-spec.md` §7's rows whose *verified by* column says
"fixtures" — those are the ones a fixture can discharge, and they are what the
corpus is checked against at audit. Cross-checked against §9's list; the
transport rows are marked as not this phase's.

| requirement | fixtures owed |
|---|---|
| R-2, R-3 | `protocol` absent, known (`1`), unknown (`2`) — the last asserting `UnsupportedProtocolVersion { found }` |
| R-4, R-5, R-51 | **six** levels, one unmodelled field each: envelope, view, option, field, content block, **alternative** (EX-8, F-11). Each asserts acceptance **and an empty discard list** |
| R-51 | `next_check` omitted / `null`, and `protocol: null` — identical outcomes, empty discard list; paired with `next_check: 45`, discarded and reported (item 5) |
| R-10, R-11 | `view` omitted → `MissingField { field: "view" }`; `view: null` → accepted, `Response.view` is `None` |
| R-12 | unknown `kind` at the **view**, at a **field**, and inside **content** — each asserting the literal `at` path (item 6) |
| R-13, R-14, R-16 | `options: []` → `EmptyOptions { at }`; duplicate option ids → `DuplicateOptionId { id, at }`; each of the five field kinds in its wire form |
| R-52 | duplicate field ids in one option → `DuplicateFieldId`; duplicate alternative ids → `DuplicateAlternativeId`, **not** `DuplicateOptionId`; a `choice` field with no alternatives → `EmptyAlternatives`; and the **negative** — the same field id in *different* options, accepted. The negative is what shows the scope is per-option |
| R-53 | a `choice` field whose option carries `fields` → `InapplicableKey { key: "fields", kind: "choice", at }`, **rejected** not ignored |
| R-50 | `min` on `text`, `options` on `number`, `min` on `choice` — each asserting key, kind and path. Plus the **negative**: an unnamed key on the same field becoming a hint, so R-50 and R-15 are shown not to collide |
| R-17 | `min: 10, max: 1` → `Bounds(Inverted)`; `NaN` and `1e400` → `Protocol(Json)` — the two that live in `fixtures/protocol-text/` (item 4) |
| R-15 | an option **with** and **without** fields — PHASE-02's amendment made empty `Fields` legal, and this is the pair that holds it |
| R-19 | the four content forms plus the bare string. Brief §10.1's `"body": "Optional context"` **verbatim** (EX-4, F-31) |
| R-18 | brief §10.2's `{"id":"notes","kind":"text","label":"Anything notable?","multiline":true}` **verbatim**, asserting `multiline` lands in `hints` (EX-4, F-38) |
| VT-3 | the misspelling pair: `minn` becomes a hint; `labell` is rejected |
| R-1, R-6, R-7, R-8 | **already discharged** at PHASE-02 by the request snapshots — not re-asserted here |
| R-21…R-28 | **already discharged** by PHASE-03's scheduling corpus |
| R-38 | **not this phase's** — framing is the transport's (PHASE-05) |

Name each file for the requirement it verifies, as PHASE-03 did: `ls` over the
directory is then a coverage report against §4.


## Harvest

**Fresh as of:** 2026-09-02 · plan accepted, **PHASE-01, PHASE-02 and PHASE-03
done** · committed through `2648a17`, the tree clean · **PHASE-04's sheet is
written and holds three plan gaps awaiting a user decision**

### Produced

- `design.md` — the design. 54 decisions (D18, D19, D21, D36, D41, D42 struck or
  superseded), 16 invariants, 11 risks.
- `draft-spec.md` — 54 requirements, `R-1`…`R-54`, several restated at round 5. Not canon; promoted at close
  with user endorsement (AC-13, AC-14).
- `canon-delta.md` — CD-1 (ADR-001 Verification, now half a build gate) and CD-2
  (ADR-002's T1 annotation). Both await endorsement.
- `review-design.md` — 63 findings across five rounds, six of them
  responder-raised. **Closed**, with a written Synthesis; read that rather than
  the findings.
- `design-log.md` — user decisions and the reasoning behind each round.
- `plan.md` — ten phases (01…08, 10, 09), coverage map complete. Drafted, not
  accepted.
- `plan-log.md` — the planning decisions and what they rested on.
- `review-plan.md` — **closed 2026-08-27**, with a written Synthesis. Four
  rounds, fourteen findings, all `major`, all repaired, **all confirmed**. Read
  the Synthesis rather than the findings.
- **The crate, from PHASE-01.** `src/lib.rs`, `src/semantics/{mod,error}.rs`,
  `src/shell/mod.rs`, `tests/protocol/{main,boundary}.rs`,
  `tests/integration/main.rs`, `rustfmt.toml`. `just check` exits 0 in both
  feature columns. The §5.2 stratum 1 taxonomy is complete — 12 + 2 + 5
  variants, `Display` and `std::error::Error`. `toml` is in the manifest,
  optional, inside `shell`.
- **The protocol tier's types, from PHASE-02.** `src/semantics/protocol/mod.rs`
  and `canonical.rs` (863 lines), plus one line in `src/semantics/mod.rs`. Six
  scalars, the eight inbound types, `Options`/`Fields`/`Alternatives`,
  `NumberRange`, and the five outbound request types with a hand-written
  `Serialize`. Inbound fields `pub(super)` with read accessors; the outbound
  types are the design's own `pub` exception (D5). 17 unit tests, 13 of them this
  phase's, and the `--no-default-features` column runs all 17.
- **Schedule resolution and the fixture runner, from PHASE-03.**
  `src/semantics/schedule.rs` (`parse` and `resolve`, both pure, `now` a
  parameter on each), `tests/protocol/runner.rs`, and 16 fixtures under
  `tests/protocol/fixtures/schedule/`, plus one line in `src/semantics/mod.rs`
  and three in `tests/protocol/main.rs`. All five `ScheduleError` variants are
  reachable from a fixture. 22 lib tests, 5 of them this phase's; the corpus is
  one `#[test]` over the 16 files and runs in **both** feature columns.
  **The fixture format is the durable artefact** — PHASE-04 inherits the
  envelope, the discovery, the external-tag read and the vacuity guard, and
  supplies only a `Check` function. It is written up under PHASE-03's sheet,
  *The fixture format*, reconciled against the code that shipped.

### Learned

Candidates for `docs/memory/` at close — all listed under **Established
empirically** above, plus:

- Slint 1.17.1 builds clean in this dev shell; `slint-build` in `build.rs`,
  `slint::include_modules!()`, and a missing `std-widgets.slint` import fails in
  the *build script* rather than rustc (`research.md`).
- tokio at 14 unique deps versus the smol family's 31, measured — the opposite of
  the intuitive reading of "smallest reasonable" (`research.md`).

**From PHASE-01, all measured here rather than assumed:**

- **`clippy::tests_outside_test_module` applies to `tests/` targets, not only to
  `#[cfg(test)]` unit modules.** With it at `deny`, every `#[test]` in an
  integration target is an error. The fix needs no lint carve-out: declare the
  module as `#[cfg(test)] mod name;` in the target root and clippy is satisfied.
  A `tests/` target is always built with `--test`, so the attribute switches
  nothing off.
- **A doc comment on any enum variant defeats `struct_variant_width` for the
  whole enum**, so no rustfmt width setting can keep a *documented* taxonomy
  line-for-line with `design.md`. This was investigated and rejected in PHASE-01;
  do not re-open it without new evidence.
- **`rustfmt --print-config current .` does not read the project's
  `rustfmt.toml`.** It reports defaults against a tree that is formatted
  otherwise, which makes it actively misleading. Use
  `rustfmt --config-path ./rustfmt.toml --emit stdout <file>` instead.
- **rustfmt never rejoins an already-split item.** Width options therefore look
  inert when tested against source a previous `cargo fmt` exploded. Test them on
  source written the compact way.
- **The feature gate catches a runtime in `semantics/` only in the column where
  the runtime is absent.** A `semantics/` module that uses tokio compiles
  perfectly well under default features — observed. That is the whole reason
  AC-15 keeps VT-1's grep alongside the build gate, and why dropping either half
  is a real loss rather than tidying.
- **`clippy::module_name_repetitions` is incompatible with §5.2's naming**, and
  `clippy::pub_use = "deny"` blocks the re-export that would dodge it. Settled
  2026-08-29 by allowing the lint crate-wide, with the argument at the site.

**From PHASE-02, all measured here rather than assumed:**

- **`#[expect(dead_code)]` and a colocated `#[cfg(test)]` module are in direct
  conflict.** The lib target sees the item as dead and the expectation is
  fulfilled; the test target sees the tests calling it and the expectation is
  *unfulfilled*, which `-D warnings` turns into an error via
  `unfulfilled_lint_expectations`. Clean under `cargo build`, failing under
  `cargo test`, from one attribute. `#[cfg_attr(not(test), expect(…))]` is the
  form that works. Anything that lands an item one phase before its caller and
  tests it in the same file will hit this.
- **Commenting out a lint in `[lints.clippy]` does not disable it if a group
  enables it.** `missing_errors_doc` is commented out with a note saying the
  doc-comment lints are paused, and `pedantic = "deny"` re-enables it anyway.
  Only `missing_docs` is genuinely paused, and only because it is a *rustc* lint
  in the other table. A commented-out entry silences nothing on its own.
- **jiff at `default-features = false` needs no `serde` feature to hit the
  wire.** It has none available, and none is wanted: `serializer.collect_str(&t)`
  over jiff's own `Display` produces `2026-08-23T04:12:00Z` — the spec's exact
  RFC 3339 form. Ten lines of hand-written `Serialize` in place of a dependency
  edge into stratum 1.
- **The envelope shape that produces `{"protocol": 1, "type": "…", …payload}`**
  is a private `struct { protocol: u32, #[serde(flatten)] body }` over an
  internally-tagged `#[serde(tag = "type", rename_all = "lowercase")]` enum. The
  payload's own keys land at the top level beside the two envelope keys. Probed
  during planning and correct on its first run in code.
- **A newtype over `String` serializes as a plain JSON object key** when used as
  a `BTreeMap` key — `BTreeMap<FieldId, Value>` renders as `{"minutes": 20}`
  with no key-serializer work.
- **A forbidden-token scan that matches substrings will fire on prose.** AC-11's
  domain list contains `site`, which caught the phrase "call sites" in a doc
  comment. `reminder` is the next one likely to bite. Worth knowing before
  writing the next such scan, in this project or another.
  **It recurred twice in PHASE-03**, on "the one site that can apply it" and "at
  half its sites" — so this is not a one-off but the predictable cost of a blunt
  scan over prose, and the word is near-unavoidable when writing about *where* a
  rule is applied. Both were reworded in about a minute each; the carve-out was
  not taken, and should not be, because the scan's value is that it has no
  exceptions. Expect it again in PHASE-04 and PHASE-09.

**From PHASE-03, all measured here rather than assumed:**

- **jiff's three parsers do not partition the input, and the dispatch order is
  load-bearing because of it.** An RFC 3339 string *with* an offset parses as a
  `jiff::Timestamp` **and** as a `jiff::civil::DateTime`. Trying civil first
  would therefore report `MissingOffset` for a value that carries one. Civil and
  span *do* partition, so below the absolute arm the order is only naming. The
  rule: absolute → civil → span → unparseable.
- **The three schedule failure kinds separate structurally, so no branch reads an
  error message.** `Span::to_duration(days_are_24_hours())` fails **only** for
  calendar units, so that error *is* `CalendarUnit`; `Timestamp::checked_add`
  failing *is* `OutOfRange`; a civil parse succeeding where the timestamp parse
  failed *is* `MissingOffset`. Verified against jiff 0.2.35.
- **jiff bounds each `Span` unit and the instant range separately, and the two
  boundaries are different.** `"1000000 weeks"` parses and then overflows
  (`OutOfRange`); `"10000000 days"` exceeds the `days` unit bound of ±7304484 and
  never reaches the addition (`Unparseable`). This is what retired the F-36-shaped
  risk that `OutOfRange` might be unreachable from the wire — it is reachable.
- **`clippy::missing_errors_doc` is live despite being commented out**, for the
  same reason PHASE-02 found for the other doc lints: `pedantic = "deny"`
  re-enables it. Any `pub fn` returning `Result` needs an `# Errors` section.
  This is the third phase to trip over the commented-out-but-grouped pattern.
- **A vacuity guard must count what was *found*, not what *passed*.** The
  obvious spelling — increment on success, fire when the count is zero — makes a
  corpus whose every case failed also report that it ran nothing: a false second
  accusation stacked on a real one, in the report a reader trusts. Found by
  breaking `parse` so that everything failed, which is a break worth running on
  any aggregate-and-report harness. `boundary.rs` counts files inspected and is
  correct; the runner's first draft was not.
- **An externally-tagged `expect` must be read by the shared half, not by each
  corpus.** Reading tags per-corpus means looking for one, then the other, and
  returning on the first hit — so a fixture claiming two outcomes has one of them
  silently unverified. Rejecting the second key is a three-line function and it
  belongs where every corpus inherits it.

### Open

**Raised by PHASE-03's expansion, 2026-09-02 — audit business, not a phase
repair.**

- **`"next_check": "18:00:00"` is accepted as a span of eighteen hours.**
  Measured on jiff 0.2.35 while retiring PHASE-03's `OutOfRange` risk. A bare
  wall-clock time is neither of R-21's two forms, and jiff reads it as `PT18H`
  under either dispatch order — it fails the civil parse, so no ordering avoids
  it. A backend author writing `"18:00:00"` and meaning *six this evening* gets
  *eighteen hours from now*, silently and successfully. This is exactly
  `MissingOffset`'s own argument (`design.md:961` — the most likely backend
  mistake deserves a name) applied to a case §5.2 did not consider. A sixth
  `ScheduleError` variant is a design question and a STOP, so PHASE-03 asserted
  the accepted behaviour in a fixture — documented rather than latent — and the
  question goes to audit. **Shipped as
  `tests/protocol/fixtures/schedule/R-21-bare-wall-clock-time.json`**, which
  states the behaviour in the description so a reader of the corpus meets it
  rather than discovering it. If audit adds the sixth variant, that fixture is
  the one to change.

**Raised by PHASE-02, 2026-08-30 — all three are audit business, none is a phase
repair.**

- **`design.md:704` over-generalises the collection rule.** The comment over
  `Options`, `Alternatives` and `Fields` reads "all three for the same reason:
  >= 1 element, and ids unique within the collection", but the F-52 paragraph
  directly beneath it argues only duplicates for fields, and R-15, R-15's
  verification row, `brief.md` twice and the spec's own example response all say
  an option may carry none. The comment is left as written and goes to
  reconciliation, on the same footing as the `toml` line: the design is a record
  of intent at a point in time. **`plan.md` EX-3 is already corrected**
  (`plan-log.md`, 2026-08-30) — the criterion had to be right before code
  depended on it; the design's prose did not.

- **`Cargo.toml`'s "doc-comment lints paused" comment is false for three of the
  four.** `missing_errors_doc`, `missing_panics_doc` and `missing_safety_doc`
  are commented out in `[lints.clippy]` under a note saying they are paused —
  but `pedantic = "deny"` enables all three regardless, so only `missing_docs`
  (a rustc lint, in the other table) is actually paused. PHASE-02 met
  `missing_errors_doc` as a hard error on its four checked constructors and
  answered it by writing the docs, which were wanted anyway. Nothing is broken;
  the comment states the opposite of what the table does, and the next phase to
  return a `Result` will re-discover it.

- **AC-11's domain-vocabulary scan matches substrings, and `site` is in the
  list.** `tests/protocol/boundary.rs` failed this phase on the phrase "call
  sites" in a doc comment. Reworded, and the test was right to be strict — but
  the token list contains ordinary English fragments (`site`, and `reminder` is
  the next likely one), so this will recur in prose rather than in code. Whether
  to anchor the scan on word boundaries is an audit call, not a phase one: the
  scan is PHASE-01's surface and tightening it silently would weaken a canon
  test to suit a comment.

- **Sixteen repairs unverified** — F-48…F-58, never re-examined, and F-59…F-63,
  round 5's own. Accepted knowingly; see the ledger's Synthesis. Expect two to
  four residual defects, most likely in §5.4.
- `plan.md` **accepted 2026-08-29**, review closed — ten phases, coverage map
  complete. PHASE-01's sheet is written; the rest are written one at a time,
  immediately before their phase.
- **Two design gaps surfaced during planning** and both are closed — the TOML
  parser by user decision, the suspected build-gate defect by measurement that
  withdrew it. See *Found while planning* above and `plan-log.md`. No canon or
  design text changes; `design.md` §5.1 and §3 owe a `toml` line at audit.
- ~~**Plan review: three rounds … F-12…F-14 unconfirmed.**~~ **Closed 2026-08-27
  on a clean round 4**, ledger `review-plan.md`, Synthesis written. Four rounds,
  fourteen findings, all `major`, none contested, all repaired, **all
  confirmed**. Each round confirmed the last one's repairs: round 2 found **4 of
  6** defective, round 3 **2 of 5**, round 4 **0 of 3**. The rate fell to zero
  only after the author applied the restatement sweep deliberately. Two risks
  the closure leaves standing, both in the Synthesis: round 4 did not re-run the
  tokio metrics probe behind PHASE-06/VT-6, and the self-sweep's five sites got
  no per-site verdict, only the round's overall silence.
- **The recurring defect in this slice is unswept restatements.** The design
  review found it three times (its F-56); round 2 found it three times more —
  a rule applied at the named site and not at the sites that restate it, and an
  enumeration inherited stale from before the decision that changed it. Anything
  written here should be swept before it is believed.
- **Two criteria have now been written to be falsifiable and were not.**
  PHASE-06/VT-6 was `shutdown_timeout(ZERO)`, which cannot fail — measured —
  and PHASE-01/VA-3 demanded a line-for-line match a correct justfile fails.
  Both are repaired — and F-8's replacement was itself vacuous until F-12, because
  a lazy future dropped before its first poll spawns nothing however the transport
  is written. The pattern is worth carrying into execution: a criterion that
  *names* a mechanism is not yet a criterion that *has* one, and a mechanism needs
  a positive control saying it would have seen the thing it is looking for.
- CD-1 and CD-2 unapplied, awaiting endorsement at audit.
- **`plan.md` was amended after acceptance, three times, all at PHASE-02.**
  2026-08-29: surfaces gained `src/semantics/mod.rs` and the colocated-test rule,
  and EX-1 gained the constructor-visibility split — both raised while expanding
  the phase sheet. 2026-08-30: EX-3 was rewritten so `Fields` checks uniqueness
  only, raised while writing that criterion's own rejection cases. All three were
  user decisions (`plan-log.md`), not agent repairs, and all three were taken
  before any code depended on them. Audit should read the PHASE-02 entry as
  amended rather than as accepted on 2026-08-29.
- **Three reconciliation items opened by PHASE-01**, none of them phase repairs:
  1. `design.md:921` writes the wrapped type as `Protocol(semantics::ProtocolError)`
     — the path `semantics::ProtocolError`, not `semantics::error::ProtocolError`.
     Reaching it needs a re-export from `semantics/mod.rs`, which
     `clippy::pub_use = "deny"` forbids. The design's own spelling is unreachable
     under the design's own lint table. No caller exists yet.
  2. §9's lint prose and `[lints.clippy]` are one line apart: `module_name_repetitions`
     is now `allow` by user decision and §9 does not say so.
  3. §5.1's manifest and §3's trigger analysis still owe the `toml` line
     (already noted at `plan-log.md`); PHASE-01 wrote it into `Cargo.toml`.
- ~~**VH-1's human half is outstanding.**~~ **Closed 2026-08-30.** The user
  reloaded; `just`, `deno`, `cargo` and `rustc` all resolve into `/nix/store/`,
  and `deno`'s store hash matches the one the agent saw on 2026-08-29 — the same
  flake evaluation, not just two green checks. PHASE-01's Log and verification
  record carry the evidence.
- Whether to promote `transport-probe.local.rs` to a tracked spike — a user call.
- ~~`flake.nix` `devToolPkgs` lacks deno.~~ **Closed.** `deno` landed in
  `projectPkgs` at commit `b76b75c`, and `just` has been in `devToolPkgs` since
  `6489521`. Both verified as store paths in a fresh `nix develop`, 2026-08-27.
  A shell entered before those commits does not see them and must be reloaded;
  **PHASE-01 found exactly that** on its first task, in its own shell. The
  reload it left owing — VH-1's human half — is also closed, 2026-08-30. See
  the phase sheet's Log for both.
- **Two design decisions taken 2026-08-27** — D53 amended, `just` adopted as the
  canonical runner. Both in `design-log.md`; the Open section above is the short
  form. Round 2 found no defect in either as stated, but found one in how
  PHASE-01 verified the `just` mirroring (F-9).
- **The crate-wide lints fire inside tests, and `clippy.toml` now carves that
  out** — `allow-unwrap-in-tests`, `allow-expect-in-tests`,
  `allow-panic-in-tests`, `allow-indexing-slicing-in-tests`. Measured: a scratch
  crate with goad's lint table fails the gate with five errors on ordinary test
  code and exits 0 with the four keys. `unwrap_in_result = "deny"` is
  deliberately not scoped away. Round 3 reviewed it and found F-14: the plan
  still told an implementer to prove the crate-wide lints by breaking them
  "anywhere", which the carve-out had made false for `tests/`. The carve-out's
  *reasoning* has still not been attacked — only its restatements.

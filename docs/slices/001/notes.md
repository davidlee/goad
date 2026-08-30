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
| PHASE-03…10 | not started; phase sheets are written one at a time, immediately before execution. Execution order is 01…08, **10**, 09 | — |

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


## Harvest

**Fresh as of:** 2026-08-29 · plan accepted, **PHASE-01 done** · working tree,
uncommitted

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

### Open

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

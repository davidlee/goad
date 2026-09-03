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
| PHASE-04 | **done** — `just check` exits 0 in both feature columns; 22 unit tests and 9 protocol tests, 4 of the latter this phase's, over a **54-file** corpus in two directories. All eight EX and all four VT criteria discharged, VA-1 and VA-2 pasted. Entry criteria checked and met. **Three plan gaps found at expansion, all closed before execution** — the Surfaces named no Rust under `tests/`, so the corpus had nowhere to be asserted from, and VA-2 named `src/semantics/normalize.rs`, which is not the file: both amended by user decision 2026-09-02. The third — VT-2's `NaN` fixture is unwritable in the inherited format, because serde_json refuses the literal at *envelope* parse — is settled in the sheet as a second corpus over raw text. **A fourth was raised during execution** — `canonical.rs` joined the Surfaces, scoped to removing four `expect(dead_code)` attributes PHASE-02 wrote as temporary, without which the lib does not compile once normalization calls the constructors. See `## Phase sheets` | 2026-09-02 |
| PHASE-05 | **done** — `just check` exits 0 in both feature columns; 22 unit tests, 7 integration and 14 protocol, 5 of the latter this phase's. All seven EX, all five VT and all three VA criteria discharged. **Three plan gaps were closed at expansion and none was raised during execution.** Two assumptions broke, both measured rather than reasoned about: **A3 is false as stated** — `Io` is deterministic, but only for a request past the 64 KiB pipe buffer, since a smaller one is accepted by the kernel and outlives the reader — and **the probe misleads about `bash`**, which drove its backends with `bash -c` and so exec'd their last command; a script *file* forks, turning the same two lines into PHASE-06's grandchild case. One departure from §5.4's sketch, recorded and argued: `body` is an `async fn` rather than an inline block, so VT-5's region check asserts F-41's rule instead of tripping over the sketch's own nested `?`s. Five break-and-revert runs, the strongest of which — holding stdin open — fails three tests at once with R-37's symptom verbatim. **A fourth gap was raised at the end and closed by user decision 2026-09-03**: `tests/integration/transport.rs`, which holds this phase's cases, was not in the Surfaces; they now read `tests/integration/**`, the form PHASE-06 already used. See `## Phase sheets` | 2026-09-03 |
| PHASE-06 | **done** — `just check` exits 0 in both feature columns; 22 unit tests, 15 integration and 15 protocol, one of the latter this phase's. All six EX, all six VT and both VA criteria discharged. **Two plan gaps found at expansion, both closed by user decision 2026-09-03** — VT-4 asked for a wedged `wait`, which no test can arrange, and VT-5's suite-level no-orphans claim is unsound under libtest's in-process parallelism. A finding against `process.rs` as shipped was closed with them and repaired here: `read_capped` borrowed rather than owned, so the stdout handle dropped when the exchange **returned** rather than at the bound — measured 500 ms apart, and the first red of the phase. **Five break-and-revert runs, and two of them found defects in this phase's own test mechanism** — a stderr fixture that passed against the reader it was meant to catch, and a `/proc` filter blind to the two backends that `exec`. Both repaired and both re-broken. See `## Phase sheets` | 2026-09-03 |
| PHASE-07 | **done** — `just check` exits 0 in both feature columns; 35 unit, 27 integration, 15 protocol. All eight EX, all six VT and VA-1 discharged. Entry criteria checked and met. **Two plan gaps found at expansion, both closed by user decision 2026-09-03** — `design.md` §5.2's taxonomy named no error type for a rejected config, which VT-2 requires (`ConfigError` added, five variants); and the config duration grammar would restate `schedule.rs:96`–`:106`, which `CLAUDE.md` forbids without a decision (restated deliberately, recorded for audit). **A third decision was taken during execution**: EX-2 names `Option<Outstanding>` and `design.md:1167` gives it an `issued_at` nothing in this slice reads — the gate refuses an unread field, so it is kept under a self-clearing `#[expect(dead_code, reason = …)]` rather than dropped. **Assumption A1 fired on the first run** and cost a fixture: serde never decodes a value it skips, so the invalid-UTF-8 case parsed cleanly against `WireResponse`; re-measured and moved into a view's title, which is the case `design.md:1052` is about. **Five break-and-revert runs, plus a sixth on the lint expectation.** See `## Phase sheets` | 2026-09-03 |
| PHASE-08 | **done** — `just check` exits 0 on all **seven** commands in both feature columns; 35 unit, 32 integration (5 of them this phase's), 15 protocol. All four EX and all four V criteria discharged. Entry criteria checked and met. **One plan gap found at expansion and closed by user decision 2026-09-03** — `deno run` does not typecheck, which is the reason OQ-9 gives for choosing deno; the gate now runs `deno check`, so `design.md` §9 and `justfile` joined the Surfaces and the plan gained **EX-6**. **A defect in this phase's own test mechanism was found by breaking it**: VT-2's plan-suggested vehicle — a config pointing at a command that cannot be spawned — is **vacuous**, because a host that spawns and then refuses still returns the refusal; the case now uses an invocation log passed as argv, which catches both reordering breaks. **clippy rejected the log's first design** (a process-wide file behind a `std::sync::Mutex` held across an await) and the argv form that replaced it is simpler. **Six break-and-revert runs.** See `## Phase sheets` | 2026-09-03 |
| PHASE-10 | **done** — `just check` exits 0 on all seven commands in both feature columns; 35 unit, **52** integration (20 of them this phase's), 15 protocol. All four EX and all three VT criteria discharged, VA-1 and VA-2 pasted. Entry criteria checked and met. **One question of scope closed by user decision 2026-09-03** — EX-2's "whole misbehaving suite" spans the transport modes as well as the protocol ones, because R-45 is a claim about host state surviving *process* failure and a protocol refusal never touches a process lifecycle. **Two of this phase's own assertions were vacuous and both were found by breaking them**: the seeded check and a re-resolved one are the same instant, so thirteen cases could not tell R-29 from a recomputation; and a one-`Host` suite that only asserts the last exchange passes against a `Host` rebuilt every time. Both repaired and both re-broken. **Six break-and-revert runs.** VA-2's walk found **two items of `design.md` §9's list with no end-to-end case** — a backend that writes nothing, and the brief's own §10.1/§10.2 examples — neither in EX-1's list, both recorded for audit. See `## Phase sheets` | 2026-09-03 |
| PHASE-09 | not started; phase sheets are written one at a time, immediately before execution. Execution order is 01…08, **10**, 09 | — |

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

**State:** **done 2026-09-02.** `just check` exits 0 in both feature columns.
All eight EX and all four VT criteria discharged; VA-1 and VA-2 pasted in the
Verification record. The three plan gaps found at expansion were closed before
execution, and execution raised **one more** — `canonical.rs` had to join the
Surfaces so four `expect(dead_code)` attributes PHASE-02 left could come off —
closed by user decision the same day. A2, A3, A4 and A5 were re-measured before
a wire type was written and all four hold. Six break-and-reverts plus two
naive-expectation reds; the refactor step found one real defect in a doc comment
that claimed the opposite of the code.
**Plan entry:** `docs/slices/001/plan.md:449`
**Surfaces (from the plan):** `src/semantics/protocol/mod.rs` (the two `pub mod`
lines, and its doc comment), `src/semantics/protocol/wire.rs`,
`src/semantics/protocol/normalize.rs`, `tests/protocol/fixtures/**`.
**Surfaces added by user decision 2026-09-02:** `tests/protocol/runner.rs` and
`tests/protocol/main.rs` — item 1. The list above is `plan.md` as amended.

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

#### Three plan gaps found at expansion — all three closed

**1. The Surfaces did not name `tests/protocol/runner.rs` or
`tests/protocol/main.rs`, and the corpus cannot exist without them. Closed by
user decision 2026-09-02 — both added.** The plan
gives this phase `tests/protocol/fixtures/**` and no Rust under `tests/`. But a
fixture file asserts nothing on its own: the protocol corpus needs a checker
function reading its own `expect` tags, a `const` naming its directory, and a
`#[test]` calling `assert_corpus` — all Rust, all in `runner.rs`, and a new file
under `tests/protocol/` would need `main.rs` to declare it.

This is **the same class as the `mod`-line omission closed on 2026-09-02**: a
phase's Surfaces listing the data it adds and not the declaration that reaches
it. PHASE-03's sheet anticipated exactly this and deferred it here rather than
patching the plan early ("*An observation for PHASE-04, not a repair here*").

**Decided: both added, and the split between them left to execution.**
*The fixture format* already says the one-file-two-halves arrangement is
"PHASE-04's call, taken with its own surfaces in hand" — naming both files is
what puts that call in hand. Adding a third file (`tests/protocol/normalize.rs`,
say) then needs `main.rs`, which is why it is named too.

**2. VA-2 named a file that does not exist. Corrected by user decision
2026-09-02.** It read "an `unwrap()` in `src/semantics/normalize.rs`". The file
is `src/semantics/protocol/normalize.rs`
— `normalize` is under `protocol/`, per EX-1 and the Surfaces line three
paragraphs above it in the same phase entry. A path correction, not a change of
intent; raised rather than absorbed because it is plan text.

**3. VT-2's `NaN` fixture cannot be written in the format this phase inherits —
and this is not a wording problem. Settled here as implementer latitude; no plan
change.** VT-2 requires a fixture asserting
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
item 4 below. It is listed here because it changes what the corpus looks like and
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

#### Verification record

All discharged 2026-09-02. Evidence is pasted below the table, not summarised
in it.

| id | mode | result | evidence |
|---|---|---|---|
| VT-1 | test | **pass** | the corpus: **52** fixtures under `tests/protocol/fixtures/protocol/` and **2** under `tests/protocol/fixtures/protocol-text/`, run by two `#[test]`s through PHASE-03's runner with the shared half unchanged. E1, E5 |
| VT-2 | test | **pass** | `normalize::every_reachable_error_in_the_taxonomy_is_named_by_a_fixture` reads the corpora back and asserts every `ProtocolError` variant is named by a fixture. **Two** documented exceptions, not one, and both are the design's — see the Log. Written down *in the test*, as the criterion asks, in the block comment above it. E3, E5 |
| VT-3 | test | **pass** | `R-50-a-misspelled-optional-key-becomes-a-hint.json` (`minn` lands in `hints`) and `R-15-a-misspelled-required-key.json` (`labell` is rejected). Both were red against the `normalize_field` stub. E1, E2 |
| VT-4 | test | **pass** | six `R-4-an-unmodelled-key-on-*.json`, one per level including `Alternative`. Each asserts acceptance and an **empty** discard list; the whole canonical value is asserted, so an unmodelled key surviving anywhere would break the case. Driven red by break (d), which put `deny_unknown_fields` on five of the six levels. VT-4's third clause does not hold at the field level and the sheet's own inventory says why — Log. E2 |
| VA-1 | agent | **pass** | `just check` exits 0, six commands, both feature columns. 22 unit tests and 9 protocol tests in each. E4 |
| VA-2 | agent | **pass** | four break-and-reverts, in **host** code at the corrected path: `unwrap()` and an unchecked `+` in `normalize.rs`, an unchecked `+` in `wire.rs`, and the same `unwrap()` under `tests/` confirming the F-14 carve-out lets it pass. E2 |
| EX-1 | review + test | **pass** | `wire.rs`: no `deny_unknown_fields` anywhere (break (d) is the proof it is load-bearing rather than merely absent); `view` present-preserving through `present`, with all three states distinguished by fixture; `next_check`, `body` and a `choice` field's `options` typed `serde_json::Value`; `hints` flattened. `clippy::option_option` had to be excepted with a written reason — Log. E1, E2 |
| EX-2 | test | **pass** | `normalize_response(wire, now) -> Result<Normalized<Response>, ProtocolError>`, `Normalized { value, discarded }`, `Discarded` closed at one variant. `now` is a parameter; nothing in the module reads a clock. `src/semantics/protocol/normalize.rs:82` |
| EX-3 | test | **pass** | `R-12-an-unknown-view-kind.json` (`view.kind`), `R-12-an-unknown-field-kind.json` (`view.options[1].fields[2].kind`) and `R-12-an-unknown-content-kind.json` (`view.body.kind`) — each asserting the path **literally**. A fourth, `R-12-a-misplaced-key-on-an-unknown-field-kind.json`, holds the ordering: the kind is reported, not the misplaced key. E2 break (f) |
| EX-4 | review | **pass** | the corpus covers every row of the sheet's inventory. Brief §10.1's `"body": "Optional context"` and brief §10.2's `{"id":…,"multiline":true}` are both carried **verbatim**, in `R-19-a-body-written-as-a-bare-string.json` and `R-18-brief-10-2-s-own-field-example.json`. E1 |
| EX-5 | agent | **pass** | `#![deny(clippy::arithmetic_side_effects)]` at the top of both `wire.rs` and `normalize.rs`, each verified **by breaking it** — E2 breaks (b) and (c). The other three restriction lints are crate-wide and break (a) confirms they reach this module |
| EX-6 | test | **pass** | `R-50-min-on-a-text-field.json`, `R-50-options-on-a-number-field.json` and `R-50-min-on-a-choice-field.json` assert `InapplicableKey` with the key, the kind and the path; `R-53-an-alternative-carrying-fields.json` asserts a `choice` field's option carrying `fields` is **rejected**, not ignored. All four were red against the stub. A3 was measured before any of it was written, and it holds |
| EX-7 | test | **pass** | `R-51-next-check-omitted.json` and `R-51-next-check-null.json` produce **identical** outcomes with an empty discard list; `R-51-protocol-null.json` likewise; `R-25-next-check-of-the-wrong-type.json` is still discarded and reported, so the two are shown to be distinguished. Three further `null` cases the rule reaches were added from the refactor review — a nulled body, a nulled modelled key on a field, a nulled `fields` on an alternative |
| EX-8 | test | **pass** | six levels, six fixtures, one unmodelled key each — envelope, view, option, field, content block, alternative. The field level is D37's stronger claim: the key is **kept as a hint**. E2 break (d) |

**E0 — task 1, the four assumptions measured before a wire type was written.**
Scratch crate under the session scratchpad, `serde = "=1.0.229"`,
`serde_json = "=1.0.151"` — the versions `Cargo.lock` pins.

```
== A2: flatten at depth vs deserialize_with on `view` above it ==
  view absent                        OK  protocol=Some(1) view=ABSENT next_check=None
  view null                          OK  protocol=None view=NULL next_check=None
  view choice, nested                OK  protocol=None view=VIEW(kind=choice) next_check=None
  view choice, body string           OK  protocol=None view=VIEW(kind=choice) next_check=None
  view choice + next_check           OK  protocol=None view=VIEW(kind=choice) next_check=Some(String("45 minutes"))
  view unknown kind                  OK  protocol=None view=VIEW(kind=slider) next_check=None
  view not an object                 ERR invalid type: integer `45`, expected struct WireView
  unmodelled at envelope             OK  protocol=None view=NULL next_check=None

  -- the nested WireView.rest, re-read as a WireChoice --
  choice title=T body=Some(String("Optional context"))
    options=Some([WireOpt { id: "a", label: "A", fields: Some([WireField {
      id: "n", kind: "text", label: "L", min: None, max: None, options: None,
      hints: {"multiline": Bool(true)} }]) }])

== A3: does serde bind min/max/options before `kind` is dispatched? ==
  text + min                         OK  kind=text min=Some(1.0) max=None options=None hints={}
  number + options                   OK  kind=number min=None max=None options=Some(Array [...]) hints={}
  choice + min                       OK  kind=choice min=Some(1.0) max=None options=None hints={}
  number + min/max                   OK  kind=number min=Some(1.0) max=Some(10.0) options=None hints={}
  text + minn (misspelt)             OK  kind=text min=None max=None options=None hints={"minn": Number(1)}
  text + multiline                   OK  kind=text min=None max=None options=None hints={"multiline": Bool(true)}

== A4: a misspelled REQUIRED key after flattening ==
  labell instead of label            ERR missing field `label` at line 1 column 40
  kindd instead of kind              ERR missing field `kind` at line 1 column 37
  idd instead of id                  ERR missing field `id` at line 1 column 37

== A5: from_str vs from_value ==
  they agree on which failure occurs for every document tried; a from_value
  error carries no line and column, and fixtures assert variant names rather
  than error strings. `1e400` fails when the Value itself is parsed.

== the null elision (item 5), measured separately ==
  all omitted        protocol=None view=None      next_check=None
  next_check null    protocol=None view=Some(false) next_check=None
  next_check 45      protocol=None view=Some(false) next_check=Some(Number(45))
  protocol null      protocol=None view=Some(false) next_check=None
  protocol 2         protocol=Some(2) view=Some(false) next_check=None

  serde_json Number equality is spelling-sensitive: parsed "10" != parsed "10.0",
  so a fixture asserting an f64 bound writes `10.0`.
```

All four hold. A3 holding is what makes EX-6 reachable; had it been false, the
design's cost argument for D37 would have changed and this would have been a
STOP.

**E1 — the corpus, as `ls` reads it.** The filename is the index, so this is the
coverage report against `draft-spec.md` §4.

```
tests/protocol/fixtures/protocol/            (52)
  R-2-protocol-omitted / -declared-as-the-version-the-host-implements
  R-3-protocol-declared-as-a-version-the-host-does-not-implement
  R-4-an-unmodelled-key-on-{the-envelope,a-view,an-option,a-field,
                            a-content-block,an-alternative}
  R-10-view-omitted
  R-11-view-null-is-nothing-to-show
  R-12-an-unknown-{view,field,content}-kind
  R-12-a-misplaced-key-on-an-unknown-field-kind
  R-13-a-choice-view / -with-no-options / -omitting-options-entirely
  R-14-duplicate-option-ids
  R-15-an-option-with-no-fields / -carrying-fields / -a-misspelled-required-key
  R-16-a-{text,boolean,datetime}-field
  R-16-a-number-field-{with,without}-bounds
  R-16-a-choice-field
  R-17-inverted-bounds
  R-18-brief-10-2-s-own-field-example
  R-19-a-body-written-as-a-bare-string
  R-19-a-body-tagged-as-{text,markdown,html,uri}
  R-21-next-check-as-a-relative-span
  R-25-next-check-of-the-wrong-type
  R-50-min-on-a-text-field / -options-on-a-number-field / -min-on-a-choice-field
  R-50-a-misspelled-optional-key-becomes-a-hint
  R-51-next-check-omitted / -next-check-null / -protocol-null
  R-51-a-nulled-body / -a-nulled-modelled-key-on-a-field
  R-51-a-nulled-fields-key-on-an-alternative
  R-52-duplicate-field-ids-within-one-option
  R-52-the-same-field-id-in-different-options
  R-52-duplicate-alternative-ids / -a-choice-field-with-no-alternatives
  R-53-an-alternative-carrying-fields

tests/protocol/fixtures/protocol-text/       (2)
  R-17-a-nan-literal-for-a-bound
  R-17-an-infinite-literal-for-a-bound
```

**E2 — the breaks. Six, plus two naive-expectation reds.** Every break was
reverted and the gate re-run green after each.

*(a) `unwrap()` in `src/semantics/protocol/normalize.rs`* — the crate-wide
restriction lint, at the path VA-2 was corrected to:

```
error: used `unwrap()` on a `Result` value
  --> src/semantics/protocol/normalize.rs:96:21
   = help: …#unwrap_used
error: docs for function which may panic missing `# Panics` section
  --> src/semantics/protocol/normalize.rs:75:1
```

The second line was not asked for and is worth keeping: `missing_panics_doc` is
commented as "paused" in the manifest, and `pedantic = deny` re-enables it — the
same surprise PHASE-03 met with `missing_errors_doc`.

*(b) an unchecked `+` in `normalize.rs`* — the module-level deny, and the trace
names the attribute:

```
error: arithmetic operation that can potentially result in unexpected side-effects
   --> src/semantics/protocol/normalize.rs:152:16
   --> src/semantics/protocol/normalize.rs:20:9
 20 | #![deny(clippy::arithmetic_side_effects)]
```

*(c) the same in `wire.rs`* — a separate claim, because EX-5 names two files:

```
error: arithmetic operation that can potentially result in unexpected side-effects
   --> src/semantics/protocol/wire.rs:191:3
   --> src/semantics/protocol/wire.rs:29:9
 29 | #![deny(clippy::arithmetic_side_effects)]
```

*(d) the identical `unwrap()` under `tests/`* — `clippy exit=0`. F-14 is real:
an `unwrap()` there proves nothing, which is why (a) is where it belongs.

*(e) I10 broken — `deny_unknown_fields` on `WireResponse`, `WireChoice`,
`WireOpt`, `WireAlternative`, `WireContent` and `WireContentValue`.* Five of the
six EX-8 levels failed, and so did every tagged-content fixture, because
`WireContent` and `WireContentValue` each see the other'"'"'s key as unknown — the
two-struct split of a content block **depends** on permissiveness rather than
merely coexisting with it. `WireView` and `WireField` cannot take the attribute
at all: `flatten` and `deny_unknown_fields` do not compose.

*(f) the corpus root of the second corpus renamed away* — the vacuity guard
covers each directory on its own, which is the small gain item 4 claimed for
splitting them:

```
…/fixtures/protocol-text-renamed-away: the corpus directory could not be read: No such file or directory (os error 2)
…/fixtures/protocol-text-renamed-away: ran no fixtures — renamed, emptied, or misspelled
```

*(g) applicability judged before the kind dispatch* — the ordering claim, and
the fixture it earned stayed in the corpus:

```
R-12-a-misplaced-key-on-an-unknown-field-kind.json: A modelled key misplaced on a
  kind the host does not implement reports the kind, not the key.
    expected { "UnsupportedPrimitive": { "at": "view.options[0].fields[0].kind", "kind": "slider" } }
    got      { "InapplicableKey": { "at": …, "key": "min", "kind": "slider" } }
```

*(h) two naive-expectation reds*, where no ordinary red exists because the claim
is about a dependency or about a `null` serde already elides. The `NaN` and
`1e400` fixtures were written expecting `Bounds(NotFinite)` — the reading F-36
refutes — and came back:

```
    expected { "Bounds": { "NotFinite": { "bound": "min", "found": null } } }
    got      { "Json": null }
    (malformed JSON: expected value at line 1 column 150)
    (malformed JSON: number out of range at line 1 column 154)
```

The nulled `min` on a text field was written expecting `InapplicableKey` and
came back accepted, which is R-51 refuting R-50 for that one case. Both were
then corrected to what the protocol does.

**E3 — VT-2, the coverage test.** `every_reachable_error_in_the_taxonomy_is_named_by_a_fixture`
derives the tag set from one instance of each `ProtocolError` variant, reads
both corpora back, and fails on a variant no fixture names. Two exemptions, each
asserted in the negative rather than merely skipped:

- `ProtocolError::Schedule` — asserted **not** to appear as an error, because an
  unusable `next_check` is a discard on an accepted message (P2, R-25). The
  companion test `a_schedule_failure_is_named_by_a_fixture_as_a_discard` asserts
  it appears on that channel instead.
- `BoundsError::NotFinite` — asserted **not** to appear, because a fixture
  claiming it would be a test that cannot fail (F-36, D39). `Inverted` is
  asserted present.

**E4 — VA-1, `just check`.** Six commands, both columns, exit 0:

```
cargo build
cargo test                          22 passed (lib) · 9 passed (protocol) · 0 doc-tests
cargo test --no-default-features    22 passed (lib) · 9 passed (protocol)
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub
cargo fmt --check
just check exit=0
```

9 protocol tests, 4 of them this phase's — two corpora and two coverage tests.
Both corpora run in the `--no-default-features` column, so the new checker
reaches nothing above stratum 1.

**E5 — the surfaces actually touched**, against the plan's list as amended:

```
M src/semantics/protocol/canonical.rs   (four lint attributes removed, nothing else)
M src/semantics/protocol/mod.rs         (two `pub mod` lines, and the stale doc comment)
M tests/protocol/main.rs                (three lines, `#[cfg(test)] mod normalize;`)
M tests/protocol/runner.rs              (five items to `pub(crate)`, and a doc paragraph)
? src/semantics/protocol/wire.rs
? src/semantics/protocol/normalize.rs
? tests/protocol/normalize.rs
? tests/protocol/fixtures/protocol/*.json         (52 files)
? tests/protocol/fixtures/protocol-text/*.json    (2 files)
```

No undeclared path. Nothing else in `src/` or `tests/` was edited.

#### Log

<!-- Append as you go: decisions taken, obstacles, anything noticed in passing. -->

- **2026-09-02, task 1 — entry check.** `nix develop --command bash -c 'which
  just cargo rustc; just check'` — `just` at
  `/nix/store/ni2dxycnhsp34y4qy6q44nw6pp6bj0l0-just-1.58.0/bin/just`, `cargo` and
  `rustc` at
  `/nix/store/cyn97lq74y3lx15y95gyzplnmmx451g9-rust-default-1.99.0-beta.1-2026-08-18/bin/`.
  A1 verified. `just check` exits 0 — 22 unit tests, 5 protocol tests, both
  clippy columns clean. Any failure from here is this phase's.
- **2026-09-02, task 1 — A2, A3, A4 and A5 measured before a wire type was
  written.** Scratch crate under the session scratchpad, `serde = "=1.0.229"`
  and `serde_json = "=1.0.151"`, the versions `Cargo.lock` pins. **All four
  hold.** Output pasted under the Verification record as E0.
  - **A2 holds.** `#[serde(flatten)]` inside `WireView` and inside `WireField`
    does not disturb the `deserialize_with = "present"` on `view` above them.
    All three states stay distinct — absent, `null`, a view — with a nested
    field carrying a hint two levels below the flatten.
  - **A3 holds, and EX-6 is therefore reachable.** `{"kind":"text","min":1}`
    binds `min: Some(1.0)` with `hints` **empty**. Serde binds the declared
    optional before `kind` is more than a string, so a misplaced modelled key
    cannot fall through to `hints`: it is raised or it is lost.
  - **A4 holds.** `labell` gives ``missing field `label` ``; so do `kindd` and
    `idd`. Required keys stay required after flattening.
  - **A5 holds where the corpora need it, with one difference worth writing
    down.** `from_str` and `from_value` agree on *which* failure occurs for
    every document tried; they differ only in that a `from_value` error carries
    no line and column. Fixtures assert a variant name and never an error
    string, so nothing depends on it. The case that is not a difference at all:
    `1e400` fails when the `serde_json::Value` is *parsed*, before any struct is
    involved — item 3's finding arriving from the other direction, and why both
    it and `NaN` need the raw-text corpus.
- **2026-09-02, one plan gap raised and closed mid-execution — `canonical.rs`
  joins the Surfaces, scoped to four lint attributes.** PHASE-02 put
  `#[cfg_attr(not(test), expect(dead_code, …))]` on `OptionId::new`,
  `AlternativeId::new`, `FieldId::new` and `Hints::new`, and each reason text
  says the attribute comes off once PHASE-04 calls it. Measured rather than
  predicted: one `OptionId::new` call from a stub `normalize.rs` gives
  `error: this lint expectation is unfulfilled` and the lib does not compile.
  The tuple fields are private, so `new` is the only construction path and there
  is no `normalize.rs` that avoids it. **Not R10** — nothing is added and nothing
  widened; four temporary attributes are deleted. User decision 2026-09-02:
  remove them and amend `plan.md`, recorded in `plan-log.md`.
- **2026-09-02, tasks 2 and 3 — `protocol/mod.rs` and `wire.rs`.** The doc
  comment's PHASE-03 claim is gone; both modules are declared. `wire.rs` is
  §5.2's shape with two additions §5.2 does not name and §6 leaves open:
  `WireAlternative` (id and label only — `fields` is deliberately **not**
  declared, so R-53's rejection is a check on the raw object rather than a field
  this type carries), and the split of a tagged content block into `WireContent`
  (its `kind`) and `WireContentValue` (its `value`), for the reason `WireView` is
  split from `WireChoice`: an unrecognised content kind must be
  `UnsupportedPrimitive` naming the string it found, which needs the
  discriminant read before anything beside it is bound.
- **2026-09-02, task 4 — normalization, in four red/green batches.** Each batch
  was driven red against a deliberate stub before it was written.
  1. **The envelope.** Ten fixtures red against a `normalize_response` returning
     `MissingField` unconditionally; green once version, `view` presence and
     `next_check` were real. Confirmed by measurement that serde maps *both* an
     omitted and an explicitly null `next_check` to `None`, and the same for
     `protocol` — so item 5's elision is structural, and the fixture pair proves
     the behaviour rather than the intention.
  2. **The two literals JSON cannot express.** Written first with the *naive*
     expectation — `Bounds(NotFinite)` — precisely so the red would be the
     design's own claim being demonstrated: both come back `Json`
     (`expected value at line 1 column 150`, `number out of range at line 1
     column 154`), F-36 executed rather than cited. Then corrected to `Json`.
     This is the one group with no ordinary red available: the claim is about
     `serde_json`'s parser, so no version of this crate's code makes it fail.
  3. **The view.** Eleven fixtures red against a `normalize_view` stub; green
     with the choice dispatch, options, body and content.
  4. **Fields.** Eighteen fixtures red against a `normalize_field` stub; green
     with kind dispatch, applicability, bounds and alternatives.
- **2026-09-02, EX-8's six levels went green on arrival, so they were driven red
  by breaking I10.** `#[serde(deny_unknown_fields)]` was added to
  `WireResponse`, `WireChoice`, `WireOpt`, `WireAlternative`, `WireContent` and
  `WireContentValue`, and reverted. Five of the six levels failed, plus two
  groups worth recording: the nulled-`fields` case, and **every tagged-content
  fixture** — because `WireContent` and `WireContentValue` each see the other's
  key as unknown, so the two-struct split of a content block *depends* on
  permissiveness rather than merely coexisting with it. The field level does not
  move under that break, and needs none: its fixtures were red in batch 4
  against the stub, and its claim is D37's stronger one — an unmodelled key
  there is **kept as a hint**, not ignored.
- **2026-09-02, VT-4's third clause does not hold at the field level, and the
  sheet's own inventory already says so.** VT-4 asks each of the six levels to
  assert "the field is absent from the canonical value". On a *field object*
  that is false by design: D37 makes every unnamed key a hint, so it survives
  into `Field.hints`. The corpus inventory row for R-4/R-5/R-51 states only the
  two clauses that hold at all six — acceptance and an empty discard list — and
  those are what every level asserts. The field-level fixture then asserts the
  **stronger** thing, that the key lands in `hints`, which is what D37 claims and
  what VT-3's `minn` case says from the other side. Recorded rather than raised:
  D37 settles it and there is no decision left in it. Same shape as
  `design.md:704`'s blanket comment over the three checked collections, which
  PHASE-02 found over-general for `Fields`.
- **2026-09-02, `clippy::option_option` refuses the shape §5.2 mandates.**
  `Option<Option<WireView>>` is a pedantic lint failure, and the lint's own
  suggestion is "a custom enum if you need to distinguish all 3 cases" — which is
  a design change made to satisfy a style lint, since §5.2 fixes both the shape
  and the `present` helper and EX-1 requires both. Taken as §9's
  reason-carrying exception instead: an `#[expect(…, reason = …)]` at the field,
  greppable and self-clearing. This is exactly the hatch D53 built for F-35's
  `child.stdin.take()`, used for the second time and for the same kind of reason.
- **2026-09-02, one local decision the design did not reach: `"fields": null`
  on an alternative is accepted.** R-53 rejects a `fields` an alternative
  carries; D50/R-51 say an explicit `null` means what omission means *for every
  modelled field*, and `fields` is a key the spec names. A null one carries
  nothing to reject and a serializer emitting `null` for an absent optional is
  doing nothing wrong — which is R-51's own argument. Applying the stated rule
  rather than inventing one, with a fixture that says so. Worth an audit note:
  the design states R-51 over "modelled fields" and R-53 over position, and this
  is where the two meet.
- **2026-09-02, two further local decisions, both applications of a stated
  rule.** An **absent** `options` on a view is `EmptyOptions` and an absent
  `options` on a `choice` field is `EmptyAlternatives`, because absent and empty
  are the same offer — nothing to pick — and R-13 rejects the second. The wire
  types make both optional precisely so the decision is normalization's rather
  than serde's. And applicability is judged **inside** each kind's arm rather
  than before the match, so `{"kind":"slider","min":1}` reports the *kind*:
  naming the key would send a backend author to fix the wrong thing.
- **2026-09-02, task 5 — the corpus's `expect` tags, for a reader who inherits
  them.** *The fixture format* says PHASE-04 adds its tags to its own checker
  rather than to the shared table, so they are recorded here. Two, both
  externally tagged like everything else in the format:
  `{"error": {"<Variant>": {<its fields>}}}` and
  `{"accepted": {"canonical": ..., "discarded": [...]}}`.
  - **An accepting fixture states the whole canonical value**, not a probe into
    one part of it. `canonical` is the entire normalized response rendered back
    to JSON, so a fixture says what a wire document *means* — which is what
    makes the corpus usable to `draft-spec.md` §7, and what discharges R-4/R-5
    without a second mechanism: a key that survived normalization would appear
    in the rendering and break the case.
  - **`discarded` is required on every accepting fixture**, never optional. For
    most cases the assertion *is* the empty list, and a corpus where silence was
    the default would assert it nowhere.
  - Rendering is externally tagged throughout — `{"markdown": ...}`,
    `{"number": {"min": ..., "max": ...}}`, `{"choice": [...]}` — so a reader
    meets one convention rather than three. Every renderer is an exhaustive
    match: a variant added to `canonical.rs` or to the taxonomy cannot reach the
    corpus without an arm.
  - One sharp edge, measured: `serde_json`'s `Number` equality is
    spelling-sensitive, so a fixture asserting an `f64` bound writes `10.0` and
    not `10`.
- **2026-09-02, task 5 — `runner.rs` split, which was PHASE-04's call to take.**
  Taken: the shared half stays in `runner.rs` with the scheduling corpus below
  its divider, exactly as PHASE-03 left it, and this phase's two corpora live in
  `tests/protocol/normalize.rs`. Five items became `pub(crate)` and one doc
  paragraph was added; **no logic above the divider changed**, which is the
  property the seam was built for. Extending `runner.rs` instead would have
  grown one file from two halves to four; moving PHASE-03's corpus out would
  have rewritten a phase that is done.
- **2026-09-02, task 7 — the refactor, and what it found.** Three extractions
  and one real defect.
  1. **`each_indexed`** replaces three near-identical loops — a view's options,
     an option's fields, a `choice` field's alternatives all walk a list, index
     it into the path, and hand the result to a checked constructor. The gain is
     not line count: `{at}[{index}]` is now written **once**, so the path
     grammar cannot drift between the three, and the corpus asserts those paths
     literally.
  2. **`normalize_alternative`** came out of `normalize_alternatives`, which is
     what makes the first extraction fit.
  3. On the test side, `render_normalized` replaced two copies of the same
     `{canonical, discarded}` shape; `sole_tag` now reads through the shared
     half's `outcome_tag` rather than walking the map a second way; and one
     directory walk serves both coverage tests.
  4. **The defect: `normalize_response`'s `# Errors` section said `Json` is not
     raised here.** It is, in three places — a view's payload, a content block
     and a `choice` field's alternatives are each deserialized *inside* this
     module, because their discriminants have to be dispatched before anything
     beside them is bound. The doc comment described the design's composition
     (transport, then `from_slice`, then normalize) and not the code under it.
     Corrected to say both. The same refactor pass added three fixtures the
     review turned up — a nulled body, a nulled modelled key on a field, a
     nulled `fields` on an alternative.
- **2026-09-02, AC-11's vocabulary scan did not fire.** PHASE-03 predicted it
  would, on the word "site", which is hard to avoid when writing about where a
  rule applies. It was avoided deliberately — "place", "position", "level" — at
  no cost to the prose. Worth carrying: the word to reach for is "place".


### PHASE-05 — Process transport: the structure and the paths that work

**State:** **done 2026-09-02.** `just check` exits 0 in both feature columns;
all seven EX, all five VT and all three VA criteria are discharged in the
Verification record below. Entry criterion checked and met. The probe was run
before any of `process.rs` existed, which is EX-6. **Three plan gaps found at
expansion, all three closed by user decision the same day** — `plan.md` and
`plan-log.md` carry them; **none was raised during execution.** Two assumptions
broke instead, both measured: A3's route to `Io`, and the probe's `bash -c`
against the harness's script files. One departure from §5.4's sketch, argued in
the Log. Nothing outstanding.
**Plan entry:** `docs/slices/001/plan.md:560`
**Surfaces (from the plan, as amended 2026-09-02 and 2026-09-03):**
`src/shell/mod.rs`, `src/shell/error.rs`, `src/shell/backend/mod.rs`,
`src/shell/backend/transport.rs`, `src/shell/backend/process.rs`,
`tests/integration/**`, `tests/backends/*.sh`, and — added by the gap 1
decision — `tests/protocol/transport_shape.rs` and `tests/protocol/main.rs`.
The integration entry was `main.rs` and `harness.rs` until the gap 4 decision
below replaced it with the glob PHASE-06 already uses.

#### Reading list

| what | where | why |
|---|---|---|
| the structure, in full | `design.md:1234` §5.4 | the sketch this phase implements. Read the prose after the code block too: nine of its bullets are repairs with findings attached, and each names a way of getting the structure wrong |
| the transport seam | `design.md:983` | `Backend`, `Exchange`, `Exchange::failed`, `cleanup_only`, `Captured` — EX-1 is this block |
| the error taxonomy | `design.md:914` (`BackendError`), `:927` (`CleanupFailure`) | what may be returned, and the argument that cleanup is a second dimension rather than a variant |
| invariants | `design.md:1648` — I9, I11, I13 | I13 is the "no `?` past the spawn" rule stated as an invariant; I11 is the no-task claim |
| the edge table | `design.md:1690`–`1730` | rows this phase owns: non-zero exit, zero exit with unparseable stdout, and the two grandchild cases (the last two are PHASE-06's to assert) |
| the misbehaving-backend list | `design.md:2044` §9 | the script inventory, minus the ones PHASE-06 and PHASE-08 own |
| the requirements | `draft-spec.md:146`–`152` (R-36…R-43), `:373`–`:383` | R-41's corrected wording — the call waits at most `timeout + CLEANUP_LIMIT`, both stated |
| the probe | `docs/slices/001/transport-probe.local.rs` | seven cases; five changed the design. Gitignored, so it is copied out to run |
| prior art — source-text tests | `tests/protocol/boundary.rs:14` (`Scan`), `:75` (`run`, holding the vacuity guard) | VT-5's shape, and the guard the plan names |
| prior art — reaching the crate root from a test binary | `tests/protocol/boundary.rs:69` | `env!("CARGO_MANIFEST_DIR")`; the harness locates `tests/backends/` the same way |

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | PHASE-02/EX-1 discharged — `Request` exists to be serialized | **met.** `Request` at `src/semantics/protocol/canonical.rs:471`, its two payloads at `:477` and `:483`, the hand-written `Serialize` at `:537`, and two unit tests asserting the exact wire form at `:727` and `:775`. All fields are `pub`, and the doc comment at `:465` says why — requests are host-authored, so stratum 2 constructs them directly |

Baseline, 2026-09-02: `just check` exits 0 on all six commands, both feature
columns — 22 unit tests, 9 protocol tests, integration skipped in the stratum 1
column because it declares `required-features = ["shell"]`. PHASE-04 is committed
at `588ccd1`. Any failure from here is this phase's.

#### What already exists — inspected 2026-09-02

| path | state | consequence for this phase |
|---|---|---|
| `src/shell/mod.rs` | 3 lines, doc comment only, no `pub mod` | every module this phase adds is declared here. It is in the Surfaces, so the PHASE-03 omission does not recur |
| `src/lib.rs:11` | `#[cfg(feature = "shell")] pub mod shell;` | already gated. Nothing here changes |
| `src/semantics/error.rs:17` | `ProtocolError`, twelve variants | `BackendError::Protocol` wraps this. See gap 3: nothing in this phase raises it |
| `tests/integration/main.rs` | 4 lines, doc comment only, "Empty until PHASE-05" | the target exists and is already feature-gated in `Cargo.toml:47`. This phase makes the comment false, which is what it is for |
| `tests/backends/` | does not exist | new directory. Not a Cargo target and needs no declaration — the scripts are data, read by argv |
| `Cargo.toml:23` | `tokio` optional, features `process`, `time`, `rt`, `io-util`, `macros` | **sufficient** — measured below. `Cargo.toml` is not in this phase's Surfaces, so a missing feature would have been a STOP |
| `clippy.toml:24`–`27` | the four `…-in-tests` keys | the integration tier may `unwrap` and index. `process.rs` may not |
| `clippy.toml:36` | `disallowed-types` bans `HashMap` and `HashSet` | `BTreeMap`/`BTreeSet` if the harness needs a map |
| `#![deny(clippy::arithmetic_side_effects)]` | `canonical.rs:17`, `schedule.rs:14`, `wire.rs:29`, `normalize.rs:20` | `process.rs` takes the same attribute — EX-7. `transport.rs` and `error.rs` are declarations and take none |

#### Measured at expansion, before any of it was written

Three questions whose answers would each have been a STOP, settled in a scratch
crate rather than reasoned about.

**1. `Cargo.toml`'s tokio feature list is sufficient, and `rt-multi-thread` is not
needed.** A crate carrying goad's exact feature list compiles the whole of §5.4's
structure — `Command`, `kill_on_drop`, `tokio::pin!`, `select!`,
`tokio::time::timeout`, `AsyncReadExt`/`AsyncWriteExt` — and `#[tokio::test]`
runs against it. `#[tokio::test]` defaults to the current-thread flavour, which
`rt` alone provides; the probe's manifest carries `rt-multi-thread` only because
`#[tokio::main]` defaults the other way. **Consequence:** `Cargo.toml` stays out
of this phase, as its Surfaces require.

**2. The AFIT future is `Send`, so EX-1's signature is honest.** `clippy::future_not_send`
is denied crate-wide, and `fn exchange(&mut self, …) -> impl Future<Output = Exchange> + Send`
is a promise the implementation has to keep. Measured by writing the trait with
`+ Send`, implementing it with §5.4's exact borrow structure, and calling it
through a generic `fn assert_send<B: Backend>(b: &mut B) -> impl Future<…> + Send`.
It compiles. Nothing in the pinned sub-future, the `select!` or the two nested
`timeout`s is non-`Send`.

**3. The probe's own code does not survive goad's lint table, in four places.**
This matters because the probe is the thing being transcribed. Under
`-D clippy::indexing_slicing`:

```
error: slicing may panic       src/lib.rs:27:58   &buf[..room]
error: slicing may panic       src/lib.rs:28:46   &buf[..n]
error: indexing may panic      src/lib.rs:36:49   &self.command[0]
error: slicing may panic       src/lib.rs:37:15   &self.command[1..]
```

`indexing_slicing` is crate-wide and carved out only for tests, so `process.rs`
must reach for `buf.get(..room)` and `command.split_first()` instead. The gate
would catch all four; they are recorded so the phase does not spend a cycle
discovering that its reference implementation is not lint-clean.

#### EX-6 — the probe, run 2026-09-02, before any of `process.rs` existed

Copied to a scratch crate with its own manifest and run with `cargo run --release`.
All seven cases reproduce, and the three timings the design records — 902 ms,
303 ms and 2.5 ms — reproduce within noise.

| case | elapsed | `result` | `stderr` | `cleanup` |
|---|---|---|---|---|
| A. normal | 2.6 ms | `Ok("{\"view\":null}")` | `"hi\n"` | `None` |
| B. stderr flood then answer | 10.3 ms | `Ok("{\"view\":null}")` | first 8 KiB, `truncated=true` | `None` |
| C. stderr then hang past timeout | 601.3 ms | `Err("timeout")` | `"about to hang\n"` | `None` |
| D. grandchild holds both pipes | 902.1 ms | `Err("timeout")` | `"parent\n"` | `Some(TimedOut { after: 300ms })` |
| E. grandchild holds stderr only | 303.7 ms | `Ok("{\"view\":null}")` | `"parent\n"` | `Some(TimedOut { after: 300ms })` |
| F. valid JSON then exit 1 | 3.2 ms | `Err("exit status Some(1)")` | `"why\n"` | `None` |
| G. valid JSON, exit 0 | 2.3 ms | `Ok("{\"view\":null}")` | `""` | `None` |

The probe's constants are `TIMEOUT = 600ms` and `CLEANUP_LIMIT = 300ms`, so its
stated bound is 900 ms, not the crate's. Four readings worth carrying:

- **No case disagrees with §5.4.** The plan's "where the sketch and a measurement
  disagree, the measurement wins and the disagreement is a finding" does not fire.
- **C is the case that separates the two bounds.** A backend that hangs with no
  grandchild pays the timeout and then cleans up *inside* the budget —
  `cleanup: None` at 601 ms, not 901. So a timeout does not imply a cleanup
  failure, and VT-2 must assert `cleanup: None` rather than ignoring the field.
- **D is 902 ms against a 900 ms bound.** VA-3's assertion is therefore a bound
  with headroom, not an equality, and not a tight ceiling either: 2 ms of it is
  scheduling. Assert `<= timeout + CLEANUP_LIMIT + slack` with slack stated, and
  on the success path assert `< timeout`, which G clears by two orders of
  magnitude.
- **B truncates at 8 KiB because the probe's `STDERR_LIMIT` is 8 KiB.** The
  crate's is 256 KiB. The flood case that asserts `truncated` against the real
  limit is PHASE-06/EX-2; B here only shows the concurrency works.

#### A3 measured at execution — the route works, but not the way the sheet said

Run before any test was written, as A3 required: 20 attempts per case, in a
scratch crate carrying the probe's structure. `write_all` of a payload of the
stated size to a backend behaving as stated, with the drain running alongside:

| case | backend | payload | result, 20 runs |
|---|---|---|---|
| H | `exit 0` — exits without reading | 64 B | **20/20 wrote successfully** |
| I | `exit 0` | 1 MiB | **20/20 `BrokenPipe` (os error 32)** |
| J | `exec 0<&-; sleep 5` — closes stdin, stays alive | 64 B | 20/20 timeout |
| K | `exec 0<&-; sleep 5` | 1 MiB | **20/20 `BrokenPipe`** |
| L | `sleep 5` — never reads, stays alive | 1 MiB | 20/20 timeout |

**A3 is right that `Io` is reachable and wrong about the route.** It is not racy:
H is a deterministic *success*. A request smaller than the pipe buffer (64 KiB on
Linux) is accepted by the kernel and sits in the buffer, which outlives the
reader — so a backend exiting before reading produces a normal exchange, not an
error. `Io` needs the write to still be in progress when the read end closes,
which means a payload past the buffer: I and K are 20/20, and Rust's
`SIGPIPE`-ignored startup means the write returns `Err` rather than killing the
process, as A3 supposed.

Consequences, all inside the phase's Surfaces:

- **VT-3's `Io` case is honest and stays.** It writes an `Evaluate` whose
  `Event.data` carries a padding string past 64 KiB — `data` is opaque and
  host-authored (R-9), so a large one is a legitimate request, and the size is
  incidental to the mechanism it exercises.
- **`Io` does need a script after all** — a backend that exits without reading
  stdin. Latitude item 5 anticipated both outcomes ("`Io` needs no script either
  *if* the EPIPE route works"), and `tests/backends/*.sh` is already a Surface,
  so this is a sixth script rather than a plan gap.
- **L is the reading worth keeping for PHASE-06.** A backend that never reads and
  never exits does not produce `Io`; it times out with the write still pending.
  Blocking on stdin is bounded by the exchange timeout like everything else,
  which is what §5.4 step 5 claims and had not been observed.

#### Three plan gaps found at expansion — all three closed 2026-09-02

**1. VT-5's source-text test has no declared home, and the shape it needs is not
the shape `boundary.rs` was built for.** VT-5 asks for three checks over
`process.rs`'s source text "in the same tier and with the same found-no-files
guard as PHASE-01's boundary checks". PHASE-01's boundary checks are
`tests/protocol/boundary.rs`, which is in the protocol target and therefore runs
in **both** feature columns. That file is not in this phase's Surfaces, and
neither is `tests/protocol/main.rs`, which declares it. Same class as the two
omissions already closed in this plan — a phase's Surfaces naming what it adds
and not the file that reaches it — and the **fifth** instance.

There is a second half. `Scan` is a *forbidden-token walk over a directory*:
`{ root, forbidden }`, matched case-insensitively per line. Only one of VT-5's
three checks is that shape:

| check | shape | fits `Scan`? |
|---|---|---|
| no `Arc`, no `Mutex` | token absent | yes |
| the only `spawn` is `Command::spawn` | token present, occurrence **shape** constrained | no |
| no `?` between the spawn and the cleanup budget | **region**-scoped | no |

`boundary.rs`'s own doc comment says "PHASE-09 extends this file — extend the
configuration, not the walk", and two of the three cannot be expressed as
configuration.

**Decided 2026-09-02:** a new `tests/protocol/transport_shape.rs`, plus
`tests/protocol/main.rs` for the `mod` line, both joining the Surfaces; nothing
lifted from `boundary.rs` beyond the idea of the guard, whose form here is
"the file was found and read" rather than "the walk inspected files". The
alternative — generalising `Scan` to carry a per-line predicate and a region
state machine — was rejected: it reworks PHASE-01's surface to fit a different
job, and would leave one type serving two unrelated questions. **PHASE-06/VT-6
re-asserts the spawn grep against the finished module and had the same
omission**, so both phases were amended at once, as the `mod`-line amendment
covered three.

**2. The harness is told to build a `Config`, and `Config` is PHASE-07's.**
The plan's notes for this phase say the harness needs "building a `Config`
pointing at one". But `src/shell/config.rs` is **PHASE-07's** surface, and
PHASE-07/EX-1 owns `Config` whole, including its rejection rules and its TOML
loading. Nothing in this phase's Surfaces can define it.

**Decided 2026-09-02:** no `Config` in this phase. `ProcessBackend` holds
`command: Vec<String>` and `timeout: Duration` directly, which is what §5.4's
sketch actually needs — `Backend::exchange` takes only `&mut self` and the
request, so the timeout must already be on the transport. PHASE-07 then
constructs a `ProcessBackend` *from* a loaded `Config`, which is the direction
"durations resolve at load, so nothing downstream carries an unparsed string"
(`design.md:1152`) already implies. Read that way the plan's sentence is loose
advice rather than a contradiction — raised because it is plan text and because
it decides the constructor's signature.

**3. `BackendError::Protocol` is not reachable from the transport this phase
builds, and VT-3 requires a case for it.** VT-3 asks for "one case per
`BackendError` variant this phase can reach: `Spawn` …, `Timeout`, `ExitStatus`,
`Protocol(Json)` via unparseable stdout, and `Io`". But EX-1 fixes
`Exchange.result` as `Result<Vec<u8>, BackendError>` — the transport returns
**bytes** and parses nothing, so it has no way to produce a `Json` error. The
design agrees where it is specific: "invalid UTF-8 then becomes a
`Protocol(Json)` error **via `from_slice`**" (`design.md:1052`), and `from_slice`
runs in `Host`, which is PHASE-07.

R-38 — exactly one JSON document, trailing content an error — is discharged by
that same `from_slice`, which refuses trailing content by itself. PHASE-04's
sheet passed R-38's two fixtures here on the grounds that framing is the
transport's; that is right about the *tier* and wrong about the *phase*, because
this phase's transport hands the bytes on unparsed.

**Decided 2026-09-02:** VT-3 drops the `Protocol(Json)` clause; PHASE-07 gains
it as **EX-8 and VT-6**, R-38's framing rule with it. The backend script stays
here regardless — EX-4 names "a zero exit with unparseable stdout" as a
**stderr** claim, and this phase asserts exactly that: `result` is `Ok(bytes)`,
those bytes do not parse, and the stderr survived. Only the variant claim moved,
and it got cheaper doing so: PHASE-07 asserts it against the fake `Backend` it
already builds, where this phase would have paid a spawn per case.

#### Settled here — implementer latitude

**4. The harness is three functions, not a framework.** `tests/integration/harness.rs`,
declared from `main.rs`:

- `backend(name: &str) -> Vec<String>` — `vec!["bash", "<manifest>/tests/backends/<name>.sh"]`,
  rooted with `env!("CARGO_MANIFEST_DIR")` exactly as `boundary.rs:69` does. No
  shebang, no executable bit: the command is an argv vector and `bash` is
  argv[0] (R-36, AC-12).
- a constructor for the transport under test, taking the script name and a
  timeout.
- `#[tokio::test]` per case, current-thread flavour. Measured above as sufficient.

Tests name their own timeout rather than sharing one constant: VT-2 wants a
short one so the suite stays fast, and VT-1 wants one long enough that a healthy
exchange cannot flake.

**5. Backend scripts are declarative, one behaviour each, named for it.** This
phase's five, from §9's list minus what PHASE-06 and PHASE-08 own:

| script | behaviour | serves |
|---|---|---|
| `reads-stdin-then-answers.sh` | `cat >/dev/null`, then a valid response, exit 0 | VT-1, R-37, AC-12 |
| `hangs-past-the-timeout.sh` | `sleep`, well past any test timeout | VT-2, R-41 |
| `writes-stderr-then-hangs.sh` | write to stderr, then `sleep` | VT-4, R-42, F-3 |
| `answers-then-exits-non-zero.sh` | valid JSON to stdout, note to stderr, `exit 1` | VT-3, EX-3, R-40 |
| `exits-zero-with-unparseable-stdout.sh` | note to stderr, garbage to stdout, `exit 0` | EX-4, R-42, F-24 |

`Spawn` needs no script — a path that does not exist. `Io` needs no script
either if the EPIPE route works; see A3.

**6. What "no `?` past the spawn" is checked as.** The region runs from the
`cmd.spawn()` match to the closing brace of the cleanup budget. The check finds
the line holding `spawn()`, finds the line holding `CLEANUP_LIMIT`, and asserts
no `?` appears in a code position between them. Comments and string literals are
excluded crudely — the file is ours, and a check that is easy to read beats one
that is hard to fool. This is the one of the three that can produce a false
positive as `process.rs` grows; the sheet's Open section is where that goes if it
does.

#### Assumptions — each a place this phase can break

- **A1 — the probe's seven cases are the whole of what §5.4 was built against.**
  Run at expansion; all seven reproduce and none disagrees. If `process.rs`
  departs from the probe's shape in any way that is not a lint repair, re-run it
  and record the output again — that is what EX-6 exists for, and PHASE-06/EX-6
  restates it.
- **A2 — timings are bounds, not values.** D measured 902 ms against a 900 ms
  bound on an unloaded machine. Under `cargo test` the suite runs cases in
  parallel by default; a tight ceiling will flake. State the slack, and if a case
  needs the machine to itself, say so rather than widening the bound until it
  passes.
- **A3 — `BackendError::Io` is reachable, and the route is EPIPE on stdin.** A
  backend that exits before reading stdin makes `write_all` fail with
  `BrokenPipe`. Unverified — Rust ignores `SIGPIPE` at startup, so the write
  should return `Err` rather than killing the test process, but this has not been
  run. **Measure it before writing the test**, and if it is racy rather than
  deterministic, `Io` has no honest case and that is a finding for VT-3 the same
  way gap 3 is.
- **A4 — `bash` is present and `tests/backends/*.sh` need no executable bit.**
  Both follow from the argv vector (R-36). The devshell provides bash.
- **A5 — the exchange future stays `Send`.** Measured. It stops being true the
  moment something non-`Send` is held across an `await`; `future_not_send` is
  denied, so the gate says so rather than the trait quietly failing to be
  implementable.

#### STOP conditions

- **The measurement and §5.4 disagree.** The plan says it in as many words: the
  measurement wins and the disagreement is a **finding for design**, not a repair
  here. This is the phase the design is least sure of.
- **Wanting `wait_with_output()`.** §5.4 refuses it twice and gives two reasons.
  It reads like a simplification and is a redesign.
- **Wanting a `tokio::spawn`, an `Arc`, a `Mutex`, or a `?` past the spawn.**
  Each was a repair with a finding attached (F-49, F-41). VT-5 exists so that a
  regression is a test failure; wanting one now is a STOP.
- **Wanting to widen `Cargo.toml`.** It is not in the Surfaces, and the feature
  set is measured sufficient. A new dependency is a STOP under `AGENTS.md`
  regardless.
- **Wanting a constructor or a field on anything under `semantics/`.** Not a
  surface here. The transport serializes a `Request` and returns bytes; it needs
  nothing else from stratum 1.
- **A fourth gap of the same kind.** The three found at expansion are closed;
  a new one is a return to plan, not a repair in this sheet.

#### Tasks

1. **EX-6 — the probe.** Done at expansion; recorded above. Re-run only if the
   structure departs from it.
2. **`src/shell/error.rs`** — `BackendError` and `CleanupFailure`, exactly
   `design.md:914`. `StateError` is PHASE-07's and is not added here.
3. **`src/shell/backend/transport.rs`** — EX-1: the trait, `Exchange`,
   `Exchange::failed`, `Captured`. `cleanup_only` lives with the thing that
   disposes of a child, so it goes in `process.rs`.
4. **`src/shell/mod.rs` and `src/shell/backend/mod.rs`** — the declarations.
5. **The harness and the first backend script** — red against no transport, then
   green: VT-1, the normal exchange. This is the case that proves stdin is closed.
6. **`process.rs`** — the structure, transcribed from the probe and repaired for
   the lint table (three sites named above, plus `command.split_first()`).
   `#![deny(clippy::arithmetic_side_effects)]` at the top (EX-7).
7. **One test per remaining VT**, each red against a deliberate stub before it is
   green: VT-2 (timeout, and `cleanup: None` — see case C), VT-3 (`Spawn`,
   `ExitStatus`, `Io`), VT-4 (stderr survives the timeout).
8. **VT-5 and VA-2** — the three source-text checks, in
   `tests/protocol/transport_shape.rs` with its `mod` line in
   `tests/protocol/main.rs`; and the read that confirms `child.wait()` sits
   inside the timed region rather than in the cleanup budget.
9. **VA-3** — the elapsed-time assertions, as bounds with stated slack.
10. **Break-and-revert** each source-text check and at least one behavioural
    claim, and record what the broken run said. A check that cannot be made to
    fail is not a check.
11. **Refactor**, then `just check`, then the sheet, the status table and the
    Harvest.

#### Verification record

| id | mode | result | evidence |
|---|---|---|---|
| EX-1 | — | **pass** | `src/shell/backend/transport.rs`: `Backend` with `-> impl Future<Output = Exchange> + Send`, `Exchange { result, stderr, cleanup }` with no outer `Result`, `Exchange::failed` (`pub(super)`), `Captured { bytes, truncated }` |
| EX-2 | — | **pass**, with one structural departure recorded in the Log | `src/shell/backend/process.rs`: `kill_on_drop(true)` at spawn; stdin moved into `body` and dropped after the write; `drain_capped` pinned as a sub-future and raced in one `select!` with the `if !drained` guard; `child.wait()` inside the `self.timeout` region; one `CLEANUP_LIMIT` budget covering kill, reap and drain completion |
| EX-3 | — | **pass**, both halves | `transport.rs::a_correct_backend_completes_an_exchange` (reads to EOF, answers, `cleanup: None`); `::a_non_zero_exit_discards_the_body_it_came_with` (`ExitStatus { code: Some(1) }`, body unreachable, stderr kept) |
| EX-4 | — | **pass**, both named paths | `::stderr_written_before_a_hang_survives_the_timeout` and `::a_zero_exit_with_an_unparseable_body_still_carries_its_stderr`. Every other case asserts stderr too — VT-1 against the serialized request verbatim, VT-2 against the pid |
| EX-5 | — | **pass**, and asserted rather than read | `transport_shape.rs::nothing_returns_between_the_spawn_and_the_cleanup_budget`; seen to fail (Log, break 3) |
| EX-6 | — | **pass** — run at expansion, before any of `process.rs` existed | the seven-case table above, plus the A3 table, both run in scratch crates |
| EX-7 | — | **pass** | `process.rs:6` `#![deny(clippy::arithmetic_side_effects)]`; the gate is green with it, and the two `saturating_sub`/`truncate` sites are why it is there |
| VT-1 | test | **pass**, and seen to fail | `::a_correct_backend_completes_an_exchange`; red as an unresolved `process` module before `process.rs` existed, then red again on break 4 |
| VT-2 | test | **pass**, three claims | `::a_backend_that_never_answers_times_out_and_is_disposed_of` — `Timeout { after }`, `cleanup: None` (case C's reading, not ignored), and the pid confirmed gone by `kill -0` |
| VT-3 | test | **pass** — four variants, one case each | `Spawn` `::a_command_that_does_not_exist_fails_to_spawn` (`NotFound`); `Timeout` VT-2's case; `ExitStatus` `::a_non_zero_exit_discards_the_body_it_came_with`; `Io` `::a_backend_that_exits_before_reading_breaks_the_pipe` (`BrokenPipe`, padded request — see the A3 table) |
| VT-4 | test | **pass**, and seen to fail | `::stderr_written_before_a_hang_survives_the_timeout`; red by break 4 (the write never reaches a backend that is not reading) |
| VT-5 | test | **pass** — three checks, both guards, all five seen to fail | `tests/protocol/transport_shape.rs`, declared from `tests/protocol/main.rs`; runs in **both** feature columns. Breaks 1–3 in the Log; the two guard tests are their own positive controls |
| VA-1 | agent | **pass** | `just check` exits 0, both columns — 22 unit, 7 integration, 14 protocol. Pasted in the Log |
| VA-2 | agent | **pass** — read, and then measured | `child.wait()` at `process.rs:180` is inside `body`, which is called at `:112` and awaited inside `tokio::time::timeout(self.timeout, …)` at `:115`; the cleanup budget's own `wait` is at `:188` in `dispose`, called at `:141` inside `timeout(CLEANUP_LIMIT, …)`. Two different `wait`s in two different budgets, which is what F-59 asked for. Break 5 turns the read into a measurement: replacing the timed `wait` with a synthesized success status fails VT-3's `ExitStatus` case and nothing else |
| VA-3 | agent | **pass** — bounds with stated slack | `transport.rs:18` `CLEANUP_LIMIT`, `:24` `SLACK`, both with the reason on the page. Success path asserted `< 500 ms` (probe measured 2.5 ms); timeout path asserted `>= timeout` and `< timeout + CLEANUP_LIMIT + SLACK`. The suite runs in 0.30 s |

#### Log

- 2026-09-02 — sheet written; entry criterion checked; EX-6 and three further
  measurements recorded above. Status set to **in progress**.

- 2026-09-02 — **A3 measured before any test was written, and it is false as
  stated.** Its own table is above, under *A3 measured at execution*. The short
  form: a backend that exits before reading does **not** fail the write when the
  request fits the pipe buffer — 20/20 successes — so `Io` needs a payload past
  64 KiB, where it is 20/20 `BrokenPipe`. Not racy in either direction. The
  fourth gap A3 anticipated does not exist; VT-3 keeps its `Io` case, and it
  gained a sixth backend script, which `tests/backends/*.sh` already covers.

- 2026-09-02 — **the probe misleads about `bash`, and it cost the first red.**
  `hangs-past-the-timeout.sh` as first written was the probe's case C verbatim —
  `echo … >&2` then `sleep 30` — and it produced `cleanup: TimedOut` where the
  probe measured `None`. The cause is not the transport: the probe drove its
  backends with `bash -c`, which **execs** the last command of the string it is
  given, so `sleep` *became* the child. Bash running a script **file** forks
  instead, so the same two lines make `sleep` a grandchild holding both pipes —
  which is PHASE-06's case, arrived at by accident, and which would also have
  left a 30-second orphan behind every run. `exec sleep 30` restores the case
  the script is named for. Both hanging scripts carry the measurement in a
  comment.

  This is the phase's most portable finding: **the probe's backends are not the
  harness's backends**, and A1's "re-run the probe if the structure departs from
  it" does not cover a divergence in the fixtures. PHASE-06 writes the two
  grandchild scripts and needs the inverse of this — a `sleep` that is *not*
  exec'd — so it is in the Harvest.

- 2026-09-02 — **`body` is its own function, and that is the one departure from
  §5.4's sketch.** The sketch inlines it as an async block, which puts three
  `?`s — the stdin write, the stdout read and `child.wait()` — inside the region
  VT-5's third check walks. Those `?`s are harmless (they return from the block,
  not from `exchange`), but no textual check distinguishes them from the one
  F-41 is about, so the check as latitude item 6 specifies it would have failed
  against the design's own structure. Moving the block to an `async fn` keeps
  every claim §5.4 makes — ends at exit not EOF, holds `&mut child`, lives in an
  inner scope that releases the borrow before the cleanup budget — and leaves
  `exchange` with no `?` at all between the spawn and the budget. The check then
  asserts exactly F-41's rule instead of approximating it. Recorded as a
  departure rather than a repair because EX-2 says "as written".

- 2026-09-02 — **the four lint sites the sheet predicted, and how each was
  repaired.** `&self.command[0]` and `&self.command[1..]` became
  `command.split_first()`, which also gives the empty case somewhere to go.
  `&buf[..room]` and `&buf[..n]` did not need a repair so much as a different
  reader: both `read_capped` and `drain_capped` use `AsyncReadExt::read_buf`
  into a `Vec`, so nothing is ever sliced — `read_capped` appends into the
  output it will return and compares `out.len()` against the limit;
  `drain_capped` reads into a reused chunk and `truncate(room)`s it, where
  `room` is a `saturating_sub`. Neither indexes, and the module's
  `arithmetic_side_effects` deny is satisfied without a single carve-out.
  `8 * 1024 * 1024` in a `const` does not trip it — const-evaluated.

- 2026-09-02 — **two constructor questions the design leaves to
  implementation.**
  - *An empty command.* `command = []` is rejected at config load, which is
    PHASE-07's, so this phase cannot see one — but `split_first()` returns
    `Option` and the lint table forbids indexing, so the case needs an answer
    anyway. It returns `Exchange::failed(BackendError::Spawn(InvalidInput))`:
    nothing was spawned, which is exactly what `Spawn` means, and no new type
    was introduced to make the state unrepresentable. Keeping
    `command: Vec<String>` is the gap-2 decision's letter.
  - *A request that will not serialize.* Structurally unreachable — a `Request`
    is host-authored and every field serializes infallibly — but `unwrap` is
    denied and something must be returned. It serializes **before** the spawn,
    so the failure has no child to dispose of, and reports
    `Protocol(ProtocolError::Json(_))`. This is not the claim gap 3 moved to
    PHASE-07: that one is about parsing what a *backend* wrote, and nothing here
    parses a response.

- 2026-09-02 — **`Debug` formatting is denied crate-wide and the test tiers
  answer that with `Display`, not with an exception.** `clippy::use_debug` is
  `deny` in `[lints.clippy]` and is *not* one of the four keys `clippy.toml`
  carves out for tests, and `boundary.rs` shows the convention: give the
  diagnostic a `Display` and format with `{}`. So `Duration` renders as
  `{}ms` via `as_millis()` in `BackendError`/`CleanupFailure`, and the harness
  grew three helpers — `describe`, `describe_cleanup` and `stderr` — which is
  also where the repeated `String::from_utf8_lossy(&exchange.stderr.bytes)`
  went. `escape_debug()` covers the one case that wanted quoting.

- 2026-09-02 — **break-and-revert, five runs.** Each break was applied, run,
  reverted, and the file's checksum compared before and after.

  1. *An `Arc<Mutex<Captured>>` type alias in `process.rs`.*
     `the_transport_shares_nothing_with_anything` failed:
     `line 34: `Arc` — the drain borrows, it does not share` and the same for
     `Mutex`. Both tokens, both located.
  2. *A real `let _leak = tokio::spawn(async {});` in `exchange`.*
     `the_only_spawn_is_the_child` failed, printing both occurrences:
     `found: let mut child = match command.spawn() { | let _leak = tokio::spawn(async {});`.
     The count is what catches it, which is F-12's point — the token is `spawn`,
     not `tokio::spawn`.
  3. *A compiling `?` between the spawn and the budget* — a closure returning
     `Option`, since `exchange` returns no `Result` and a bare `?` will not
     compile there. `nothing_returns_between_the_spawn_and_the_cleanup_budget`
     failed: `line 97: `?` between the spawn and the cleanup budget — a return
     there skips disposal (F-41)`. **Stated precisely:** this shows the check
     sees a `?` in a code position in the region. The dangerous form — a `?`
     that returns from `exchange` — cannot be written today at all, because
     `Exchange` is not a `Result`; the check is a tripwire against the
     signature changing, and break 3 is as close as a compiling break gets.
  4. *`std::mem::forget(stdin)` instead of `drop(stdin)`* — the host holds the
     request pipe open. **Three tests failed, all three of the backends that
     read stdin**: VT-1 with `a correct backend answers: Timeout { after: 5s }`,
     VT-3's exit-status case with `expected a non-zero exit, got backend did not
     respond within 5000ms`, and EX-4's unparseable case. That is R-37's symptom
     verbatim — "a timeout on every call that looks like a slow backend rather
     than a host bug" — and it is the strongest single break in the phase.
  5. *`body` ends at EOF on stdout rather than at exit* — the timed
     `child.wait()` replaced with a synthesized success status, which is F-59's
     defect exactly. **One test failed, and it is the right one**:
     `expected a non-zero exit, got a 14-byte response`. The body that parsed
     was delivered with the exit code that disclaimed it never read. This is
     VA-2 as a measurement rather than a reading.

- 2026-09-02 — **a fourth gap of the same kind, found at the end by diffing the
  paths touched against the Surfaces. Raised, not repaired — the STOP list says
  so in as many words.** The tests this phase writes live in
  `tests/integration/transport.rs`, and the Surfaces name
  `tests/integration/main.rs` and `tests/integration/harness.rs` and nothing
  else. Same class as the five before it: a phase's Surfaces naming what it adds
  and not the file that carries it — except that here the missing file is the
  tests themselves, which is the phase's own deliverable.

  Everything else this phase touched is declared. The two options are to move
  the cases into `main.rs`, against the protocol tier's own convention that a
  target root declares and does not assert; or to amend PHASE-05's Surfaces to
  `tests/integration/**`, which is **the form PHASE-06's Surfaces already use**
  and which fixes the class rather than the instance. The second is the
  recommendation. **Closed the same way it was raised: user decision 2026-09-03,
  the glob.** `plan.md` and `plan-log.md` carry it; no code moved. Sixth
  instance of the class, and the first where the undeclared file was the phase's
  own deliverable rather than the `mod` line reaching it.

- 2026-09-02 — **the gate.** `nix develop`'s toolchain confirmed first, per
  PHASE-01's working rule: `just` and `cargo` both resolve into `/nix/store`,
  `cargo 1.99.0-beta.1`, `rustc 1.99.0-beta.1 (f47d5bb13)`.

  ```
  $ just check                      # exit 0
  cargo build
  cargo test                        # 22 unit, 7 integration, 14 protocol, 0 doc
  cargo test --no-default-features  # 22 unit, 14 protocol — integration skipped
  cargo clippy --all-targets -- -D warnings
  cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub
  cargo fmt --check
  ```

  `pgrep -f '^sleep 30'` after the suite: nothing. PHASE-06/EX-5 owns that
  assertion properly; this is the informal version, and it is the reason the
  `exec` finding above mattered before PHASE-06 rather than during it.

### PHASE-06 — Process transport: bounds, disposal, and the two grandchild cases

**State:** **done 2026-09-03.** `just check` exits 0 in both feature columns —
22 unit, 15 integration, 15 protocol. All six EX, all six VT and both VA criteria
are discharged in the Verification record below. Entry criterion checked and met,
and the expansion's measurements are recorded below. **Two plan gaps were found at
expansion and both were closed by user decision the same day** — `plan.md` and
`plan-log.md` carry them. One finding against code PHASE-05 shipped was raised
with them and is *not* a gap: EX-1 requires behaviour `process.rs` does not have,
`design.md` §5.4 and R-43 already mandate that behaviour, and `process.rs` is in
this phase's Surfaces — so it is this phase's work. The user's decision was to
repair the code rather than narrow the criterion.
**Plan entry:** `docs/slices/001/plan.md:696`
**Surfaces (from the plan):** `src/shell/backend/process.rs`,
`src/shell/error.rs`, `tests/integration/**`, `tests/backends/*.sh`,
`tests/protocol/transport_shape.rs`.

#### Reading list

| what | where | why |
|---|---|---|
| the two bounds and their asymmetry | `design.md:1457`, `:1520`–`:1527` | D34 in full: "truncate" means stop **storing**, never stop **reading**. EX-2 is this bullet |
| the stdout cap's second half | `design.md:1528` | the claim EX-1 asserts — the reader closes the stream, and the backend is what observes it |
| the two grandchild cases | `design.md:1407`–`1424` | the table EX-4 asserts, and the argument for why they differ |
| cancellation, scoped | `design.md:1425`–`1440` | what VT-6 may claim and what it may not: nothing the **host** holds survives; the child is `kill_on_drop`'s, best-effort |
| the edge rows this phase owns | `design.md:1725`–`1729` | four rows: the stdout cap, the stderr cap, the two grandchild cases. VA-2 is a re-read of these against the tests |
| the two dimensions | `design.md:1380`–`1406` (the four-combination table), `:927` (`CleanupFailure`) | EX-3. `TimedOut` is named for what was observed, not for what it might mean |
| the requirements | `draft-spec.md:152` (R-43), `:153` (R-48), `:154` (R-54), `:381`–`:383` (their verification rows) | R-43's verification row is where "asserted by the backend observing the broken pipe" comes from |
| the transport as shipped | `src/shell/backend/process.rs` | `read_capped`/`drain_capped` at `:223`/`:247`, the cleanup budget at `:140`, `body` at `:167` |
| the harness this phase inherits whole | `tests/integration/harness.rs` | `backend`, `transport`, `evaluate`/`padded_evaluate`, `describe`/`describe_cleanup`/`stderr` |
| prior art — a pid asserted gone | `tests/integration/transport.rs:98`–`127` | VT-2's `alive()`, and the pattern EX-5 extends |
| prior art — source-text checks | `tests/protocol/transport_shape.rs` | `Source`, `Code`, `Breach`, and the two guard tests. VT-6's structural half re-asserts `the_only_spawn_is_the_child` here |
| the `exec` finding, and its inverse | `notes.md` PHASE-05 Log 2026-09-02, and the Harvest | **the single most important thing to read before writing a script.** PHASE-05's hanging scripts need `exec`; this phase's grandchild scripts need its absence |

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | PHASE-05 discharged — every exit criterion it carries | **met.** EX-1…EX-7 are all recorded `pass` in PHASE-05's Verification record, each with named evidence; the fourth gap raised at the end of that phase was closed by user decision 2026-09-03 (the `tests/integration/**` glob) and needed no code |

Baseline, 2026-09-03: `just check` exits 0 on all six commands, both feature
columns — 22 unit, 7 integration, 14 protocol. PHASE-05 is committed at
`b3ac51c`. Any failure from here is this phase's.

#### What already exists — inspected 2026-09-03

| path | state | consequence for this phase |
|---|---|---|
| `src/shell/backend/process.rs` | the whole §5.4 structure, 267 lines | both bounds are already implemented and neither is tested. `STDOUT_LIMIT` 8 MiB (`:22`), `STDERR_LIMIT` 256 KiB (`:26`), `CLEANUP_LIMIT` 500 ms (`:30`) |
| `read_capped` (`process.rs:223`) | takes `&mut (impl AsyncRead …)` | **does not drop the stdout handle at the bound** — see the finding below. EX-1's second clause is about this line |
| `src/shell/error.rs` | `BackendError` with `OutputTooLarge`, `CleanupFailure` with `TimedOut` | both variants exist and neither is reachable from a test yet. This phase is expected to add nothing here |
| `tests/backends/` | six scripts, PHASE-05's | four more, and the two hanging ones are the trap: they carry `exec` and this phase's grandchild scripts must not |
| `tests/integration/transport.rs` | 7 cases, `CLEANUP_LIMIT` and `SLACK` restated at `:18` and `:24` | the constants are already local to the tier; this phase reuses them rather than restating them again |
| `tests/protocol/transport_shape.rs` | 3 checks + 2 guards, runs in **both** columns | VT-6's structural half is `the_only_spawn_is_the_child`, already written and already green |

#### Measured at expansion, before any test was written

A scratch crate with a path dependency on this repo, driving the **real**
`ProcessBackend` against candidate scripts. Three runs of each; the numbers below
are stable to a millisecond or two. This is the harness's own bash-script route,
not the probe's `bash -c`, which is the divergence PHASE-05 paid for.

| case | script shape | timeout | elapsed | `result` | `stderr` | `cleanup` |
|---|---|---|---|---|---|---|
| grandchild holds stderr | `sleep 2 >/dev/null &` then answer | 5000 ms | **503 ms** | `Ok(14)` | 0 B | `TimedOut { 500ms }` |
| grandchild holds stdout too | `sleep 2 &` then answer | 300 ms | **802 ms** | `Err(Timeout)` | 0 B | `TimedOut { 500ms }` |
| stderr flood then answer | 300 KB to stderr, then answer | 5000 ms | **5 ms** | `Ok(14)` | 262144 B, `truncated` | `None` |
| stdout flood, `exec`'d | `exec yes …` | 5000 ms | **6 ms** | `Err(OutputTooLarge)` | 15 B | `None` |
| stdout flood, **not** `exec`'d | `yes …` as a grandchild | 5000 ms | 507 ms | `Err(OutputTooLarge)` | 15 B | `TimedOut { 500ms }` |

Five readings, each of which changes a test this phase would otherwise have
written wrong:

- **Both grandchild cases reproduce exactly as `design.md:1407` tabulates them,
  and the timings separate them.** The stderr-only case pays the cleanup budget
  *alone* — 503 ms against a 5-second timeout, so a test whose timeout is long
  proves the timeout was not paid. The stdout-too case pays both, 802 ms against
  300 + 500. Assert the first as `< 1 s` with a long timeout and the second as
  `>= timeout` and `< timeout + CLEANUP_LIMIT + SLACK`.
- **The `exec` inversion is real and it is silent.** The stdout flood written the
  obvious way — `yes` as the script's last command with other lines after it —
  makes the flooder a *grandchild* holding stderr, which turns `cleanup: None`
  into `TimedOut` and quietly converts the flood case into a grandchild case.
  Same defect as F-63, arrived at from the other direction. The flooder must be
  `exec`'d, or its stderr must be closed.
- **The stderr flood does not deadlock and truncates at exactly 256 KiB**, with
  the body still reading stdout — 262144 bytes, `truncated`, 5 ms. The flood is
  written past the 64 KiB pipe buffer *and* past the bound, which is what EX-2
  asks for; `yes … | head -c 300000 >&2` is enough and costs nothing.
- **`num_alive_tasks()` works under this crate's exact tokio feature list** —
  `process`, `time`, `rt`, `io-util`, `macros`; no `rt-multi-thread`, no
  `tokio_unstable`. Measured on a current-thread runtime the measurement built
  itself: 0 with nothing spawned, 1 with a detached exchange in flight, 0 after
  `abort()`. That is VT-6's positive control and its assertion, in that order.
- **Children of the test process can be enumerated without an external tool.**
  `/proc/self/task/*/children` lists them, `/proc/<pid>/cmdline` names them, and
  a grandchild whose parent has died is **not** in the list — it reparents. So
  the `sleep 2` a grandchild case leaves behind is not an orphan by EX-5's
  definition, and EX-5 does not need it to be gone.

#### The finding that is not a gap — `read_capped` does not drop the handle

EX-1 requires three things of the stdout bound: `OutputTooLarge`, **the reader
drops the handle**, and the backend observing the broken pipe. The second is
false of `process.rs` as shipped. `read_capped` borrows (`&mut`), so `stdout`
lives in `exchange`'s frame until the exchange **returns** — after the kill,
after the reap, after the drain.

This is not a reading of the source; it is measured, and the difference is
observable. A flooding backend that reports its own `EPIPE` out of band — a
grandchild that writes a marker file, so it outlives the host's kill — with a
second grandchild holding stderr so that disposal stalls for the whole 500 ms
budget and widens the window:

| `read_capped` takes | marker written, relative to the exchange returning |
|---|---|
| `&mut` — as shipped | **+1.8 ms** — the pipe closed because the call ended |
| by value — as §5.4 states | **−500.1 ms** — the pipe closed because the bound was hit |

`design.md:1520` says the reader "stops at the limit and **drops the handle**",
`design.md:1528` says that is what makes the flood stop, R-43 says the host
"stops reading and **closes the stream**", and `process.rs:220`'s own doc comment
already claims it. The code is the only thing that disagrees, `process.rs` is in
this phase's Surfaces, and EX-1 asks for exactly this. **Closed by user decision 2026-09-03: repair the code.** So it is phase work:
`read_capped` takes its reader **by value**, `body` takes `stdout` by value, and
the handle drops where the bound is hit. `drain_capped` already has that shape,
so the lint table has been shown to accept it.

Two consequences to carry into the tasks: **EX-6 fires** — `process.rs` changes,
so the probe is re-run and its output recorded — and the claim needs a
regression guard that is not a timing race, which is the fourth source-text check
in the latitude section below.

#### Two plan gaps found at expansion — **both closed 2026-09-03**

**1. VT-4's arrangement cannot be built.** It asks for "a backend wedged so
`wait` cannot return". `dispose` is `start_kill` then `wait`; `start_kill` sends
`SIGKILL`, and the only thing that defers `SIGKILL` is uninterruptible kernel
sleep, which a test cannot arrange without a device to block on. What *is*
reachable — and what §5.5's row at `design.md:1729` actually cares about — is
that **the cleanup budget elapses, `TimedOut` is reported, and the exchange
returns** rather than blocking. Three cases in this phase produce that, and in
all three the stall is the **drain**, not `wait`.

Options: (a) reword VT-4 to the reachable claim — disposal that cannot complete
within the budget — and discharge it on the case that already exists, with the
elapsed bound as its content; (b) keep VT-4 as a distinct case and give it the
stdout-flood-plus-stalled-drain backend, which stalls disposal for a different
reason than the grandchild cases do; (c) leave the wording and accept a test that
asserts something it did not arrange. **Recommendation: (a)**, with §5.5's row
wording — "wedged so `wait` cannot return" — recorded for audit as a case the
design describes and no test can build. **Closed as (a) by user decision
2026-09-03**; `plan.md` VT-4 is amended and `plan-log.md` carries the argument.

**2. VT-5's "after the whole misbehaving suite" is unsound under `cargo test`.**
libtest runs a target's tests as threads of **one** process, so a global "the
test process has no children" assertion sees the children of every case running
concurrently, and fails on other people's work. EX-5's own wording is per-case
("after every misbehaving case"), which is sound; VT-5's suite-level wording is
not.

Options: (a) read EX-5 literally — every misbehaving case asserts its own child
is gone, the way VT-2 already does with `kill -0` — and give VT-5 an aggregate
that **settles**: poll `/proc/self/task/*/children`, filtered to processes whose
`cmdline` names `tests/backends/`, until none remains or a deadline passes, so a
concurrent case cannot false-fail it and a genuine leak still does; (b) move the
aggregate into its own test target, which needs a `[[test]]` entry and so needs
`Cargo.toml`, which is **not** in this phase's Surfaces; (c) drop the aggregate
and keep only the per-case assertions. **Recommendation: (a).** **Closed as (a)
by user decision 2026-09-03**; `plan.md` VT-5 is amended in two parts.

#### Settled here — implementer latitude

1. **Scripts are declarative, named for the behaviour**, per PHASE-05's rule.
   Four were foreseen — `floods-stdout-past-the-cap.sh`,
   `floods-stderr-then-answers.sh`, `leaves-a-grandchild-holding-stderr.sh`,
   `leaves-a-grandchild-holding-stdout-too.sh` — and execution added two more,
   both inside `tests/backends/*.sh` and both argued in the Log:
   `floods-stdout-and-reports-the-broken-pipe.sh`, because EX-1's third clause
   has no in-band channel, and `hangs-without-exec.sh`, because an `exec`'d
   backend is not named by its script in `/proc`. Each carries the measurement
   that constrains its shape in a comment — for the grandchild pair, that a bare
   `sleep` is required and `exec` would destroy the case, which is the inverse of
   the note the two hanging scripts carry.
2. **Grandchild sleeps are 2 seconds**, not 30. They must outlive the 500 ms
   cleanup budget and nothing more; a 30-second sleep leaves a process about for
   half a minute after every run for no gain.
3. **The flood's marker path is an argv element, not an environment variable.**
   `ProcessBackend` takes a `Vec<String>`, so a test can pass the path as `$1`;
   `std::env::set_var` is `unsafe` in edition 2024 and `unsafe_code` is denied.
   The file goes in `std::env::temp_dir()`, is named with the process id, and is
   removed by the test.
4. **A fourth source-text check in `transport_shape.rs`**: `read_capped` takes
   its reader by value. That is the regression guard for the finding above —
   ownership is what drops the handle, and it is visible in the signature, where
   a timing assertion would be a race.
5. **EX-5's mechanism is `/proc`, not `pgrep`.** The devshell declares neither
   `procps` nor `coreutils`; `tests/integration/transport.rs:120` already shells
   out to `kill`, which resolves ambiently. Reading `/proc/self/task/*/children`
   needs no tool at all. Whether to convert `alive()` to the same mechanism is a
   refactor-step call, not a criterion.
6. **Nothing in `src/shell/error.rs` is expected to change.** It is in the
   Surfaces because `OutputTooLarge` and `CleanupFailure::TimedOut` are this
   phase's to reach; both already exist. Wanting to *rename* `TimedOut` is a STOP
   (F-48, F-63).

#### Assumptions — each a place this phase can break

- **A1 — the five measurements above hold under `cargo test`'s parallelism.**
  They were taken sequentially in one process. A loaded machine moves the
  timings, not the outcomes; the bounds are asserted with `SLACK` already stated
  at `transport.rs:24` for exactly this.
- **A2 — a grandchild reparents rather than staying a child.** Measured. If it
  did not, EX-5's aggregate would fail on cases that are behaving correctly.
- **A3 — `read_capped` taking its reader by value survives the lint table.**
  `drain_capped` already does, so `needless_pass_by_value` is not expected to
  fire. If it does, the reason goes at the site under D53's hatch — the drop *is*
  the point of the ownership.
- **A4 — 300 KB written to stderr by `yes | head -c` outruns nothing.** The flood
  has to pass the 64 KiB pipe buffer while the body is reading stdout, which is
  the deadlock the concurrency exists to prevent; measured at 5 ms.
- **A5 — the marker file is written by a process the host's kill cannot reach.**
  It is a grandchild, and its stderr is closed so it does not hold the drain open.
  If either changes, the flood case silently becomes a grandchild case.

#### STOP conditions

- **A third plan gap.** The two above are open; a new one is a return to plan.
- **Wanting to change `CLEANUP_LIMIT`, `STDOUT_LIMIT` or `STDERR_LIMIT` to make a
  test pass.** They are constants by decision (`design.md:1457`), and a test that
  needs one moved has found something the design should hear about.
- **Wanting to rename `CleanupFailure::TimedOut`.** F-48 and F-63 both.
- **Wanting a `tokio::spawn`, an `Arc`, a `Mutex`, or a `?` past the spawn** —
  unchanged from PHASE-05, and now with three green checks that will say so.
- **Wanting `Cargo.toml`.** Not in the Surfaces. A second test target, a dev
  dependency, or a tokio feature would all need it, and each is a STOP.
- **A grandchild case that needs a sleep to assert cancellation.** VT-6's
  behavioural half is a task-count assertion on a runtime the test owns; if it
  needs timing, the structure has regressed.

#### Tasks

1. **Close the two gaps** with the user, and amend `plan.md` / `plan-log.md`.
   **Done 2026-09-03** — both took the recommendation, as did the `read_capped`
   finding.
2. **`read_capped` and `body` take their readers by value** (the finding above),
   red first: the flood case's marker assertion fails against the shipped
   signature.
3. **EX-6 — re-run the probe** once `process.rs` has changed, and record its
   seven cases here.
4. **EX-1 / VT-1** — `floods-stdout-past-the-cap.sh` and its case:
   `OutputTooLarge`, and the marker written before the exchange returned.
5. **EX-2 / VT-2** — `floods-stderr-then-answers.sh`: success, `truncated`, and
   the body still reading stdout. Break it by making `drain_capped` stop at the
   bound and record the hang.
6. **EX-4 / VT-3** — the two grandchild scripts and their two cases, asserting
   both dimensions and the elapsed bounds that separate them.
7. **EX-3 / VA-2** — the four-combination table: two rows are PHASE-05's cases,
   two are this phase's. State where each row is asserted rather than adding a
   fifth test to restate them.
8. **VT-4** — as gap 1 settles it.
9. **EX-5 / VT-5** — the per-case assertions and the settling aggregate.
10. **VT-6** — the structural half re-asserted against the finished module, and
    the behavioural half on a runtime the test owns: spawn, wait for `>= 1`,
    abort, assert `0`.
11. **Break-and-revert** every new check and at least two behavioural claims;
    record what each broken run said.
12. **Refactor**, then `just check`, then this sheet, the status table and the
    Harvest.

#### EX-6 — the probe, re-run 2026-09-03 after `process.rs` changed

Copied to a scratch crate and run with `cargo run --release`, as PHASE-05 did.
All seven cases reproduce, and every timing is within noise of the run recorded
under PHASE-05's sheet — so the ownership repair changed nothing the probe can
see, which is the answer this re-run existed to get.

| case | elapsed | `result` | `cleanup` |
|---|---|---|---|
| A. normal | 2.5 ms | `Ok("{\"view\":null}")` | `None` |
| B. stderr flood then answer | 10.8 ms | `Ok("{\"view\":null}")`, `truncated` | `None` |
| C. stderr then hang past timeout | 601.3 ms | `Err("timeout")` | `None` |
| D. grandchild holds both pipes | 901.8 ms | `Err("timeout")` | `Some(TimedOut { after: 300ms })` |
| E. grandchild holds stderr only | 303.4 ms | `Ok("{\"view\":null}")` | `Some(TimedOut { after: 300ms })` |
| F. valid JSON then exit 1 | 3.0 ms | `Err("exit status Some(1)")` | `None` |
| G. valid JSON, exit 0 | 2.1 ms | `Ok("{\"view\":null}")` | `None` |

#### Verification record

| id | mode | result | evidence |
|---|---|---|---|
| EX-1 | — | **pass**, and it needed the repair | `transport.rs::a_stdout_flood_is_refused_and_the_backend_sees_the_stream_close` — `OutputTooLarge { limit: 8 MiB }`, and the backend's own out-of-band report that the stream closed, written 500 ms before the exchange returned. The "reader drops the handle" half is asserted structurally by `transport_shape.rs::the_capped_reader_owns_the_stdout_handle`, because ownership is the mechanism and a timing assertion would be a race |
| EX-2 | — | **pass**, and the fixture had to be repaired twice to make it mean anything | `::a_stderr_flood_is_truncated_and_the_exchange_still_succeeds` — `Ok(body)`, `truncated`, exactly `STDERR_LIMIT` kept, `cleanup: None`, and no deadlock with the body still reading stdout. See the Log: the first two versions of the script passed against a reader that stops at the bound |
| EX-3 | — | **pass**, all four combinations | the table under *EX-3* in `transport.rs` names the case that asserts each row: `Ok`/`None` and `Err`/`None` are PHASE-05's two, `Ok`/`Some` and `Err`/`Some` are this phase's grandchild pair. Every case asserts **both** fields, so no row is carried by a test that ignores the other dimension |
| EX-4 | — | **pass**, and asserted differently, which is F-63's whole point | `::a_grandchild_holding_stderr_costs_the_cleanup_budget_and_nothing_else` (`Ok(response)`, `TimedOut`, elapsed in `[CLEANUP_LIMIT, CLEANUP_LIMIT + SLACK)` against a 5 s timeout that is never approached) and `::a_grandchild_holding_stdout_too_fails_both_dimensions` (`Timeout { after: 300ms }`, `TimedOut`, elapsed `>= timeout + CLEANUP_LIMIT`). The scripts differ by one redirection |
| EX-5 | — | **pass**, per case and in aggregate | every misbehaving case ends with `alive(reported_pid(&exchange))` false; `::the_misbehaving_suite_leaves_no_child_behind` drives all five and then settles over `/proc`. The cancelled exchange asserts only what AC-5 claims — nothing the **host** holds — and says nothing about the child |
| EX-6 | — | **pass** — re-run because `process.rs` changed | the seven-case table above |
| VT-1 | test | **pass**, two cases | the flood with a reporter, and `::a_stdout_flood_with_nothing_behind_it_is_disposed_of_cleanly`, which is the `cleanup: None` claim the first case cannot make because it stalls disposal on purpose |
| VT-2 | test | **pass**, and seen to fail | `::a_stderr_flood_is_truncated_and_the_exchange_still_succeeds`; red under break 2 with `ExitStatus { code: None }` — the backend killed by `SIGPIPE`, which is what a bounded *reader* on that stream produces |
| VT-3 | test | **pass** | the two grandchild cases above, both dimensions and the elapsed bounds |
| VT-4 | test | **pass**, as reworded 2026-09-03 | the flood-with-a-reporter case: disposal cannot complete inside the budget, `TimedOut` is reported, and the exchange **returns** in `< CLEANUP_LIMIT + SLACK`. The grandchild pair asserts the same shape at its own two bounds |
| VT-5 | test | **pass**, both parts, and seen to fail | per-case `alive` assertions and `::the_misbehaving_suite_leaves_no_child_behind`, with `::a_backend_that_is_running_is_seen_as_a_child` as the enumerator's positive control. Red under break 3: `3 process(es) still a child of this one after 3s` |
| VT-6 | test | **pass**, both halves, both seen to fail | structural: `transport_shape.rs::the_only_spawn_is_the_child`, red under break 5c naming both occurrences. Behavioural: `::a_cancelled_exchange_leaves_nothing_of_the_host_behind` on a runtime it builds itself, red under break 5a with `left: 1, right: 0`. Break 5b shows the positive control is load-bearing — with it removed the same leak passes |
| VA-1 | agent | **pass** | `just check` exits 0, both columns — 22 unit, 15 integration, 15 protocol. Pasted in the Log |
| VA-2 | agent | **pass** — the four rows this phase owns, re-read against the tests | `design.md:1725` (8 MiB: refused, reader closes, child reaped — all three asserted, the middle one structurally); `:1726` (256 KiB: retained, flagged, **drained to EOF**, succeeds — the last clause is what the fixture's final write exists to assert); `:1727` and `:1728` (the grandchild pair, each asserting the row's own `result`/`cleanup`/timing); `:1729` (wedged `wait`, reworded — see gap 1) |

#### Log

- 2026-09-03 — sheet written. Entry criterion checked and met; baseline green at
  `b3ac51c`. Five measurements taken against the real transport, tabulated above.
  One finding recorded against `process.rs` as shipped — `read_capped` does not
  drop the stdout handle, and the marker measurement separates the two
  structures by 500 ms. Two plan gaps raised.

- 2026-09-03 — **both gaps closed by user decision, and the finding with them.**
  VT-4 reworded to disposal that cannot complete within the budget; VT-5 split
  into per-case assertions and a settling aggregate; `read_capped` repaired to
  own its reader. `plan.md` and `plan-log.md` carry all three. Status set to
  **in progress**.

- 2026-09-03 — **the ownership repair, red first.** The flood-with-a-reporter
  case was written against the shipped signature and failed with "the backend
  never saw the stream close, so the reader is holding the handle past the
  bound". `read_capped` and `body` then took their handles by value and it went
  green. The probe was re-run afterwards, as EX-6 requires; nothing it measures
  moved.

- 2026-09-03 — **break-and-revert found two defects, and both were in this
  phase's own test mechanism rather than in the transport.** That is the whole
  argument for the step: the code under test was right each time and the thing
  asserting it was not.

  1. *`drain_capped` stops reading at the bound.* The stderr fixture **passed**.
     Two reasons, and both had to be fixed. The flood was 300 KB, so the part
     left after the 256 KiB bound was ~37 KB — inside the 64 KiB pipe buffer, so
     nothing blocked. And the deeper one: a bounded reader that stops also
     **drops its handle**, so the flooder dies of `EPIPE` rather than blocking,
     and the answer still arrives. "Truncated, and it succeeded" is therefore
     true of a host that stops reading, and the design's predicted symptom — a
     deadlock — never appears. The fixture now writes 400 KB and then asks the
     pipe a question the host cannot answer for it: one more line to stderr,
     after the bound, whose success decides which body is written. Red under the
     same break with `ExitStatus { code: None }` — the backend killed by
     `SIGPIPE`.
  2. *Disposal kills nothing.* Three cases failed, and the aggregate — the one
     whose entire job is to catch a leak nothing else sees — **passed**. Its
     `/proc` filter matched `tests/backends/` against the child's command line,
     and two of the scripts `exec`, so a leaked `sleep 30` is not named by the
     script it came from. The same blindness made the enumerator's positive
     control race the `exec`. The filter is gone: the aggregate settles over
     *all* children, which is sound because a concurrent case's child leaves on
     its own and a leak does not. Re-run under the same break: `3 process(es)
     still a child of this one after 3s`.

- 2026-09-03 — **the cancellation control is load-bearing, shown rather than
  argued** (F-12). With a real `tokio::spawn` leaked into `exchange`: with the
  wait-for-the-child control in place the case fails `left: 1, right: 0`; with
  the control removed the *same leak passes*, because a future dropped before
  its first poll never spawns anything. The control waits on a marker unique to
  the case in `/proc`, so another case's backend cannot satisfy it — which
  needed a seventh script, `hangs-without-exec.sh`: an `exec`'d backend is no
  longer named by its script in `/proc`, and this is the one case that must see
  the child *while* the exchange is in flight.

- 2026-09-03 — **three lint repairs and one refactor.**
  `clippy::let_underscore_must_use` caught all three of the phase's `let _ =`
  sites. Two were marker-file removals and became `harness::clear`, whose doc
  comment says why absence is normal at both ends; the third was
  `let _ = task.await` and became an assertion worth making —
  `task.await.is_err()`, which says the exchange really was cancelled rather
  than allowed to finish. The refactor: the "first line of stderr is the pid"
  convention every misbehaving script follows moved into
  `harness::reported_pid`, which also refuses an empty pid — `alive("")` would
  otherwise ask `/proc` about itself and answer yes. PHASE-05's timeout case
  uses it too, so the convention has one statement. `alive` moved with it and
  now reads `/proc` rather than shelling out to `kill`, which removes an ambient
  tool dependency **and** the child it spawned on every call, which the
  unfiltered enumeration would have had to watch go by.

- 2026-09-03 — **the gate.**

  ```
  $ just check                      # exit 0
  cargo build
  cargo test                        # 22 unit, 15 integration, 15 protocol, 0 doc
  cargo test --no-default-features  # 22 unit, 15 protocol — integration skipped
  cargo clippy --all-targets -- -D warnings
  cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub
  cargo fmt --check
  ```

  Five consecutive suite runs at 1.43 s, no timing flake. Afterwards: no child of
  the test process, and the only processes left are `sleep 2` grandchildren
  reparented to init, which is what the two grandchild fixtures are for and what
  EX-5 explicitly does not claim. One piece of untidiness worth naming: on a
  **failing** flood run the marker file is written a millisecond or two after the
  case has already cleared it, so a stale `/tmp/goad-broken-pipe-<pid>` survives.
  It is bounded to one per failing run and only appears when the phase is red.


### PHASE-07 — Config, host state, and composition

**State:** **done 2026-09-03.** `just check` exits 0 in both feature columns —
35 unit, 27 integration, 15 protocol. All eight EX, all six VT and VA-1 are
discharged in the Verification record below. Entry criteria checked and met, and
the expansion's measurements are recorded below. **Two plan gaps were found at
expansion and both were closed by user decision the same day** — `plan.md` and
`plan-log.md` carry them. **A third decision was taken during execution**:
EX-2's `Outstanding` names a field nothing in this slice reads, and the gate
refuses an unread field — kept under a self-clearing lint expectation rather than
dropped. **Five break-and-revert runs, plus a sixth on that expectation.** One
assumption fired exactly as written (A1) and cost a fixture; one held with a
qualification (A3).
**Plan entry:** `docs/slices/001/plan.md:830`
**Surfaces (from the plan):** `src/shell/mod.rs` (three `pub mod` lines),
`src/shell/config.rs`, `src/shell/state.rs`, `src/shell/host.rs`,
`src/shell/error.rs` (add `StateError` **and `ConfigError`**, per the gap-1
decision), `tests/integration/**`. **Not** `Cargo.toml` — `toml` was declared at
PHASE-01/EX-6. **Not** `src/semantics/schedule.rs`, per the gap-2 decision.

#### Reading list

| what | where | why |
|---|---|---|
| `Host`, `Outcome`, `Presented`, `Failure` | `design.md:1060`–`1131` | the whole public surface this phase lands, and the four paragraphs arguing each field's position. EX-4 is this block |
| `Config`, and the TOML it parses | `design.md:1132`–`1158` | EX-1's three values, the argv-not-shell rule (R-36), and "durations resolve at load" |
| `State`, its three owners, and `resolved_check` | `design.md:1159`–`1210` | EX-2. The ownership table is where "no `Arc`, no `Mutex`" is argued (D14) |
| `view_id` — the value and its four reasons | `design.md:1211`–`1233` | D13. VT-5 asserts reason 3 |
| startup, and what is fatal | `design.md:1236`–`1241` | "a malformed or missing config is **fatal at construction**"; a backend that cannot spawn is not |
| the round trip, as a sequence | `design.md:1574`–`1599` | the order this phase composes: exchange → `from_slice` → `normalize_response` → resolve → state update |
| `respond` checks before it transports | `design.md:1600`–`1604` | EX-3's "before the transport is touched", and why forwarding a stale answer would be wrong |
| the state machine | `design.md:1605`–`1614` | five transitions, and EX-6 is two of them |
| failure does not move the schedule | `design.md:1615`–`1620` | EX-5, and P2 at the lifecycle level |
| the two `view: null` rows | `design.md:1725`, `:1726` (edge table, first two rows) | EX-6 exactly — `evaluate` leaves an outstanding interaction alone, `respond` closes it (F-46) |
| the framing rows | `design.md:1741`, `:1742` | EX-8's first two cases, and "framing is the transport's job and this transport's frame is one document" |
| the config rejection rows | `design.md:1743`, `:1744` | EX-1's three rejections, stated as edge cases |
| the host validates nothing but the id | `design.md:1753`–`1757` | what `respond` may **not** do. PHASE-08/VT-5 tests it; this phase must not foreclose it |
| the requirements | `draft-spec.md:125` (R-26), `:127` (R-27), `:128` (R-29), `:134` (R-30), `:135` (R-31), `:136` (R-32), `:137` (R-33), `:138` (R-34), `:139` (R-35), `:147` (R-38), `:155` (R-49) | R-30 is the one with no test anywhere in the plan — see *Noticed, not this phase's* below |
| the verification rows | `draft-spec.md:367`–`377` | what the spec says each of the above is proven by |
| schedule resolution as it shipped | `src/semantics/schedule.rs:146` and its doc comment at `:120`–`:145` | `resolve`'s four arguments, why `default_poll` is a `jiff::SignedDuration`, and the sentence "converting at the config boundary" that decides EX-1's storage types |
| the duration grammar, as it is already written | `src/semantics/schedule.rs:96`–`:106` | the two lines config must either reuse or restate. Gap 2 |
| normalization's entry point | `src/semantics/protocol/normalize.rs:80` and its doc comment | `normalize_response(wire, now) -> Result<Normalized<Response>, ProtocolError>`, and the `Json`-arrives-here-too paragraph |
| the transport seam | `src/shell/backend/transport.rs` | what a fake has to implement, and `Exchange`'s three fields — all `pub`, so a test can build one |
| the error taxonomy as it shipped | `src/shell/error.rs` | the `Display`/`Error`/doc-comment conventions `StateError` must match |
| the harness this phase extends | `tests/integration/harness.rs` | `backend`, `transport`, `evaluate`, `describe` — and the `Display`-not-`Debug` rule for panic messages |
| prior art — colocated unit tests in stratum 2 | `src/shell/backend/process.rs` (none) and `src/semantics/schedule.rs:164` | there is **no** colocated test in `shell/` yet. `schedule.rs` is the pattern to copy |
| the lint obligation | `plan.md:14` Overview item 4, and `src/shell/backend/process.rs:1`–`:6` | EX-7: `host.rs` gets the module deny, `config.rs` and `state.rs` do not, and the proof is break-it-and-revert **in host code** |

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | PHASE-06 discharged, EX-6 included | **met.** EX-1…EX-6, VT-1…VT-6 and both VA rows are all recorded `pass` with named evidence in PHASE-06's Verification record. EX-6 — the probe re-run after `process.rs` changed — is the seven-case table in that sheet |
| EN-2 | PHASE-01/EX-6 discharged: `toml` declared under `shell`, and absent from stratum 1's graph | **met.** `Cargo.toml` declares `toml = { version = "1", optional = true }` and `shell = ["dep:tokio", "dep:toml"]`. `cargo tree --no-default-features` shows five crates — jiff, serde, serde_json and their own — and neither `tokio` nor `toml`. Pasted in the Log |

Baseline, 2026-09-03: `just check` exits 0 on all six commands, both feature
columns — 22 unit, 15 integration, 15 protocol. PHASE-06 is committed at
`532c244`. Any failure from here is this phase's.

#### What already exists — inspected 2026-09-03

| path | state | consequence for this phase |
|---|---|---|
| `src/shell/mod.rs` | two `pub mod` lines — `backend`, `error` | three more: `config`, `host`, `state`. That is the whole of this file's change |
| `src/shell/error.rs` | `BackendError` (7 variants), `CleanupFailure` (2), both with `Display` and `Error::source` | `StateError` is added here in the same shape. **`BackendError::Protocol` already exists and is unreachable** — EX-8 is what reaches it |
| `src/shell/backend/transport.rs` | `Backend` trait, `Exchange { result, stderr, cleanup }`, `Captured` | all three `Exchange` fields are `pub`, so the fake constructs one literally. `Exchange::failed` is `pub(super)` and is **not** available to `tests/` |
| `src/shell/backend/process.rs:34`–`:50` | `ProcessBackend::new(command: Vec<String>, timeout: std::time::Duration)` | the construction `Config` feeds. Its doc comment already says loading and rejection are configuration's job — this phase makes that true |
| `src/semantics/schedule.rs:146` | `resolve(retained, incoming, default_poll: jiff::SignedDuration, now)` | fixes `ScheduleConfig::default_poll`'s type. Seeding is the `(None, None)` arm, so `Host::new` calls `resolve` rather than adding `now + default_poll` a second time |
| `src/semantics/protocol/canonical.rs:43`, `:103` | `ViewId::new(impl Into<String>)` and `Timestamp::new`/`instant`, both **public** | PHASE-02/EX-1 made them public for exactly this phase |
| `src/semantics/protocol/canonical.rs:147`–`:161` | `Response::view() -> Option<&View>`, `Response::schedule() -> Option<Timestamp>` | what the host reads out of a normalized response. `schedule()` is `resolve`'s `incoming` |
| `tests/integration/main.rs` | two `#[cfg(test)] mod` lines | two more: the fake and this phase's cases |
| `tests/integration/harness.rs` | nine helpers, all `pub(crate)` | `describe` takes `&Result<Vec<u8>, BackendError>` — the transport's shape, not the host's. This phase adds its own describer rather than widening that one |

#### Measured at expansion, before any test was written

A scratch crate with a path dependency on this repo, against jiff 0.2.35,
toml 1.0 and serde_json 1.0.151 — the versions in `Cargo.lock`. Every row below
decides something a test or a type would otherwise have been written wrong.

**`jiff::Timestamp`'s `Display` is already RFC 3339, and D13 needs nothing else.**

| input | `Display` |
|---|---|
| `2026-08-23T04:12:00Z` | `2026-08-23T04:12:00Z` |
| `2026-08-23T04:12:00.5+10:00` | `2026-08-22T18:12:00.5Z` |
| `Timestamp::UNIX_EPOCH` | `1970-01-01T00:00:00Z` |

So `format!("{}#{}", now.instant(), seq)` **is** D13's value, and D13's own
example is reproduced exactly. Two consequences: the id is always UTC-normalized
whatever offset the caller's instant carried, and it is **not fixed width** —
a sub-second `now` renders fractional seconds. Nothing parses a `view_id`, so
neither matters; both are stated so VT-5's fixture is chosen deliberately.

**The config duration grammar, through `Span` + `days_are_24_hours()` — the same
two lines `schedule.rs:96`–`:106` already runs.**

| raw | result |
|---|---|
| `5s`, `30m`, `500ms`, `1h 30m`, `45 minutes`, `PT45M` | accepted — `5s`, `30m`, `500ms`, `1h 30m`, `45m`, `45m` |
| `1 day` | accepted as `24h` |
| `0s` | accepted, `is_zero()` |
| `-1s` | accepted, `is_negative()` |
| `1 month` | `to_duration` fails — the calendar-unit case |
| `""`, `abc` | `Span` parse fails |

So **zero and negative both parse** and neither is refused by the grammar. EX-1's
`"0s"` rejection is therefore an explicit check, not a fallout of parsing, and
the same check covers `-1s`, which EX-1 does not name and which is the same
mistake. `std::time::Duration::try_from(SignedDuration)` errors only on the
negative — `0s` converts to `0ns` happily — so ordering the check **before** the
conversion is what makes `timeout = "0s"` reachable as its own error rather than
as a conversion failure.

**toml 1.0.**

| input | outcome |
|---|---|
| the design's own example (`design.md:1134`) | parses to the three values |
| `[schedule]` absent | `TOML parse error at line 1, column 1 … missing field \`schedule\`` |
| `command = "x"` | `TOML parse error at line 2, column 9 … invalid type: string "x", expected a sequence` |
| an unknown top-level key | **accepted silently** — see the latitude section |
| `{{{` | `TOML parse error … invalid key-value pair, expected key` |

`toml::de::Error`'s `Display` already carries line, column and a caret excerpt,
so a config error that wraps it needs to add nothing.

**`serde_json::from_slice` is exactly R-38's framing rule, with one qualification.**

| bytes | outcome |
|---|---|
| `` (empty) | `Err(EOF while parsing a value at line 1 column 0)` |
| `{"view":null}` | `Ok` |
| `{"view":null} {"view":null}` | `Err(trailing characters at line 1 column 15)` |
| `{"view":null} x` | `Err(trailing characters at line 1 column 15)` |
| `{"view":null}\n  ` | **`Ok`** — trailing whitespace is not trailing content |
| `{"a":"\xff"}` | `Err(invalid unicode code point at line 1 column 8)` |

All three of EX-8's cases are `serde_json::Error`, so all three are
`ProtocolError::Json` and none needs a new variant. The qualification is the
fifth row: R-38 says "trailing content is an error" and serde's reading is
"trailing **non-whitespace**", which is the right reading — a backend ending its
document with a newline is not sending two — and is worth stating because a
fixture built out of `echo` produces exactly that byte. Measured against
`serde_json::Value`; **re-measure against `WireResponse` at the first red**, since
the typed path is what ships.

#### Two plan gaps found at expansion — **both closed 2026-09-03**

**1. Nothing in the design names an error type for a rejected config.** EX-1
requires three rejections and VT-2 requires each to "name its error", but
`design.md` §5.2's taxonomy has five error types and none of them is about
configuration: `ProtocolError`, `BoundsError` and `ScheduleError` are stratum 1's
and are about a *backend's* message; `BackendError` is about an exchange that ran;
`CleanupFailure` is about disposal; `StateError` is about an id a *caller* named.
A config file is none of those — it is the user's own file, read before any
backend exists. The plan's Surfaces say `src/shell/error.rs` **(add `StateError`)**,
naming one addition where the phase needs two, and `draft-spec.md` R-44's list of
distinct errors does not mention configuration either. `design.md:1236` says only
that a malformed or missing config is "fatal at construction", which says what
happens to the process and nothing about what the caller is handed.

This is not a criterion that can be narrowed: without a type, `Config::load`
returns something, and the choices are a `ConfigError`, a `String`, or a panic.
The last two are refused by AC-6's own ethos and by the crate's lint table.

**Recommended:** add `ConfigError` to `src/shell/error.rs`, in the shape the two
types there already have — `Display`, `Error::source`, one variant per mistake a
user can make:

| variant | for |
|---|---|
| `Read(std::io::Error)` | the file is missing or unreadable (`design.md:1236`'s "missing") |
| `Syntax(toml::de::Error)` | not TOML, a mistyped value, a missing section or key |
| `Duration { key: &'static str, raw: String }` | a duration string jiff refuses — `"1 month"`, `"abc"` |
| `EmptyCommand` | `command = []` |
| `NonPositive { key: &'static str }` | `timeout = "0s"`, `default_poll = "0s"`, and the negatives the same check catches |

Five variants discharge EX-1's three named clauses and the two structural ones
above them. **Closed by user decision 2026-09-03: add `ConfigError`, five
variants.** `plan.md`'s Surfaces line now names both additions to `error.rs`.

**2. The duration grammar would be stated twice.** `schedule.rs:96`–`:106` already
parses a span and converts it with `SpanRelativeTo::days_are_24_hours()`, and
config needs the same two lines. `design.md:1152` is explicit that this is
deliberate — "the same grammar as `next_check`, one duration syntax across the
product" — which makes a *divergence* the defect, not the duplication as such.

Three things make sharing awkward rather than obvious. The existing function is
`parse_instruction`, which is private, tries an **absolute instant first** (a form
config must not accept), and reports `ScheduleError` — a taxonomy whose variants
are about a backend's `next_check`, so `ScheduleError::CalendarUnit` on a config
key would name the wrong subject. And `src/semantics/schedule.rs` is **not** in
this phase's Surfaces.

**Recommended: (a) restate the two lines in `config.rs`** with config's own
errors, and record it. The alternative, **(b) extract a stratum 1 helper** —
`schedule.rs` gaining a `pub fn` returning `SignedDuration` and a caller-chosen
error — is the better factoring on paper and costs a surface amendment plus a
shared error vocabulary between a backend-derived value and a user-authored one.
A third option, **(c)**, is to defer the question to audit with the duplication
recorded, which is (a) plus a note.

The reason this is a gap rather than latitude: `CLAUDE.md` forbids parallel
implementations outright, so choosing (a) is a decision about the standard, not
a coding choice. **Closed by user decision 2026-09-03: (a), restate in
`config.rs`, and record the duplication for audit.** The thing that must not
diverge is the grammar, which is jiff's rather than this crate's; the two lines
are not. `plan.md`'s Surfaces line now says `src/semantics/schedule.rs` is
**not** a surface here, so the decision cannot be quietly reversed later.

#### Settled here — implementer latitude

1. **`Config` stores two different duration types, and the design's unqualified
   `Duration` does not say which.** `BackendConfig::timeout` is a
   `std::time::Duration` because `ProcessBackend::new` takes one and tokio's
   timeout takes one; `ScheduleConfig::default_poll` is a `jiff::SignedDuration`
   because `resolve` takes one and `schedule.rs:141` says the conversion belongs
   "at the config boundary". Both are what "durations resolve at load" means for
   their one consumer.
2. **A wire/canonical split for the config file, mirroring `semantics`.** A
   `#[derive(Deserialize)]` struct with `String` durations, and a validating
   conversion into `Config`. The alternative — a custom `Deserialize` — puts
   validation inside serde's error channel, where `EmptyCommand` would have to
   be spelled as a deserialization failure.
3. **No `deny_unknown_fields` on the config types.** Measured above: an unknown
   key is accepted silently, so `default_poll` misspelled as `defualt_poll` is a
   missing-field error (caught) but a stray `[logging]` section is not (ignored).
   Strictness is arguably right for a file the user wrote — I10's no-closed-contract
   rule is about *inbound wire types* and does not reach here — but no criterion
   asks for it and P3 forbids building what nobody asked for. **Recorded for
   audit, not built.**
4. **`Host` keeps `config` and reads `default_poll` on every call.** `resolve`'s
   `default_poll` argument is only consulted on the `(None, None)` arm, so after
   seeding it can never change the answer; passing it anyway keeps R-26's three
   arms stated in one place and keeps the field read. An unread field would fail
   the gate — `dead_code` is `warn`, and `cargo clippy -- -D warnings` promotes it.
5. **Where each test lives.** VT-2 (config rejections) and VT-5 (`view_id`
   determinism) are colocated `#[cfg(test)] mod tests` in `config.rs` and
   `state.rs`: both are pure functions over host-authored input and neither needs
   a backend. VT-1, VT-3, VT-4 and VT-6 are `tests/integration/`, against the
   fake. This is `schedule.rs`'s split — fixtures for what a wire document means,
   colocated units for what a typed function computes — applied one stratum up.
   Note the colocated tests run in the `shell` column only, which is correct:
   the modules do not exist in the other one.
6. **The fake is scripted, and counts its calls.** `Vec<Exchange>` popped in
   order plus a call count, because EX-3's claim is that the backend was **not
   contacted** and a count is the only way to say so. PHASE-08/VT-2 makes the
   same claim through a real process that would fail if run; these are the same
   assertion at two costs, not a duplication.
7. **`Outcome` needs a describer of its own.** `harness::describe` takes the
   transport's `Result<Vec<u8>, BackendError>`. The host tier's panic messages
   want `Failure` and `Option<Presented>`, so this phase adds `describe_outcome`
   beside it rather than generalising the existing one.

#### Assumptions — each a place this phase can break

| id | assumption | how it breaks, and what tells us |
|---|---|---|
| A1 | `serde_json::from_slice::<WireResponse>` reports empty input, trailing content and invalid UTF-8 the same way the `Value` probe did | a typed target can fail *earlier* — a missing `view` field is `MissingField`, not `Json`. Every EX-8 fixture is therefore a *well-shaped* document plus the framing defect, never a shapeless one. Re-measured at the first red |
| A2 | `Exchange`'s three public fields are enough to build every case the fake needs | `Exchange::failed` is `pub(super)`; if a case needs a constructor `tests/` cannot reach, the fake cannot express it and that is a finding against the seam, not a reason to widen it quietly |
| A3 | a `Host<B>` generic over `Backend` compiles against a fake whose `exchange` is an `async fn` | AFIT plus the `+ Send` bound on the trait's return. If the fake needs a `Box::pin`, the seam is harder to implement than the design claims and PHASE-08's own backends inherit that |
| A4 | nothing in this phase does arithmetic on backend-derived data except through `semantics` | EX-7 puts the deny on `host.rs`. If `config.rs` or `state.rs` turns out to need one, the lint table's placement rule (D53, "about the data, not the directory") is what decides, and the sheet records why |
| A5 | `State::next_seq` incrementing is the only arithmetic in `state.rs` | EX-7 says if the counter moves into a module carrying the lint it needs a `checked_add`. It does not move; if the increment ends up in `host.rs`, it does |

#### STOP conditions

- **Either open gap answered by the agent rather than the user.** Both are
  decisions about what the design and the standard say, not about code.
- Anything requiring an edit under `src/semantics/**` — not a Surface. Gap 2's
  option (b) is exactly this and is why it is a question.
- Anything requiring an edit to `Cargo.toml` — not a Surface, and `toml` is
  already declared.
- A measurement disagreeing with `design.md` §5.3, §5.4 or the §5.5 edge table.
  PHASE-06's precedent: the measurement wins, the disagreement is a finding, and
  the finding goes to the user before any repair.
- `Outcome`, `Presented` or `Failure` turning out not to be buildable as
  `design.md:1063`–`1096` states them.
- The round trip needing anything of `respond` beyond the `view_id` check
  (`design.md:1753`). PHASE-08/VT-5 asserts the host validates nothing else.

#### Tasks

Red / green / **refactor** throughout. Break-and-revert on every claim whose
mechanism is not obvious from the assertion.

1. **Sheet, entry criteria, baseline, measurements.** Done — above.
2. ~~**Both gaps to the user.**~~ **Done 2026-09-03** — both closed on the
   recommended option, `plan-log.md`.
3. **`config.rs`** — the wire/canonical split, the three values, the three
   rejections, `ConfigError`. Red first: one rejection case per EX-1 clause,
   colocated (VT-2). EX-1.
4. **`state.rs`** — `State`, `Outstanding`, `issue`, `next_seq`. Red first:
   VT-5's exact-id assertion against a fixed `now` and counter. EX-2.
5. **`error.rs`** — `StateError`'s two variants, `Display`, `Error::source`, in
   `BackendError`'s shape. EX-3's vocabulary.
6. **The fake backend and the host tier's harness additions.** No host code yet;
   this is the vehicle everything below is red against.
7. **`host.rs`** — `Host`, `Outcome`, `Presented`, `Failure`, `new`, `evaluate`,
   `respond`. In criterion order, red each: EX-4 (the shape), EX-8 + VT-6 (the
   three framing cases), EX-3 + VT-3 (stale and unknown ids, outstanding
   survives), EX-5 + VT-4 (the schedule across three failures), EX-6 (the two
   `view: null` rows). EX-7's module deny lands with the file and is proven by
   break-and-revert **in host code**, per Overview item 4.
8. **Refactor.** The step that is not optional. Particular attention to whether
   `evaluate` and `respond` are two functions over one shared body or two bodies
   that drifted — they differ in exactly two places, the request they build and
   what they do to `outstanding`.
9. **Break-and-revert.** At minimum: the `view_id` check moved *after* the
   exchange (VT-3 must fail, and the call count is what catches it); the failure
   path allowed to write `resolved_check` (VT-4); `from_slice` replaced by a
   reader that stops at the first document (VT-6's trailing-content case); the
   `respond` success path leaving `outstanding` set (EX-6).
10. **`just check`, both columns.** VA-1. Then the Verification record, the
    Harvest, and the status table.

#### Noticed, not this phase's

All six are audit or reconciliation business, and none is a phase repair. They
are restated in the Harvest's *Open* section.

- **Invalid UTF-8 in a value serde *skips* is accepted.** `{"a":"\xff"}` parses
  as a `WireResponse`, because every field is `#[serde(default)]` and a skipped
  value is never decoded. So "the host rejects a body that is not UTF-8" is true
  only of bytes serde actually reads — keys, and values it binds.
  `design.md:1052`'s argument for `Vec<u8>` over `String` **stands** and is in
  fact vindicated: the case that matters is a *read* value, where
  `String::from_utf8_lossy` would have silently substituted U+FFFD, and that case
  is rejected. But the rejection is not total, and R-38's verification row does
  not say so. Audit should decide whether that is a qualification worth writing
  down or a case the host should refuse outright — the latter would mean
  validating the whole body before parsing it, which costs a pass over 8 MiB.
- **`design.md` §5.2's error taxonomy is now incomplete as written.**
  `ConfigError` exists and §5.2 lists five error types, not six. Straight
  reconciliation: the code is right and the document is stale.
  `draft-spec.md`'s R-44 has the same shape — its list of distinct errors names
  nothing about configuration.
- **`design.md:1167`'s `issued_at` is read by nothing.** Kept under
  `#[expect(dead_code, reason = …)]` by user decision. Audit should either give
  it a reader or remove it from the design; the expectation self-clears if a
  reader appears, so the record cannot go stale in the other direction.
- **A config file's unknown keys are ignored silently.** A stray `[logging]`
  section or a misspelled section name is accepted; a misspelled *key* is caught,
  because it presents as a missing field. `deny_unknown_fields` would close it
  and no criterion asks for it, so it was recorded rather than built (P3). I10's
  no-closed-contract rule is about *inbound wire types* and does not reach a file
  the user wrote, so nothing forbids the strictness either.
- **Half of PHASE-05's second Open item is now closed structurally.**
  `command = []` is rejected at load, so `ProcessBackend`'s synthesized
  `Spawn(InvalidInput)` is unreachable *through `Config`*. It remains reachable
  by a caller that builds a `ProcessBackend` directly, which
  `tests/integration/transport.rs` does. Audit's question is unchanged in kind
  and smaller in scope.
- **R-30's verification row has no owner in the plan.** `draft-spec.md:370` asks
  for a "source check that no inbound type has a `view_id` field", and the
  Coverage map allocates it to no phase — it is a spec row rather than an AC, and
  the map is by AC. The requirement's first half ("the host mints every
  `view_id`") is discharged here by EX-2; the source check is unwritten, and its
  natural home is `tests/protocol/boundary.rs`, which is PHASE-01's surface.
  Audit business, or PHASE-09's sweep. Recorded rather than fixed, because
  `tests/protocol/` is not this phase's surface.

#### Re-measured during execution — A1 was right, and the fixture it named was wrong

The expansion measured framing against `serde_json::Value`. A1 said a typed
target can fail *earlier* and that every EX-8 fixture must therefore be a
well-shaped document plus the framing defect. It fired on the first run: the
invalid-UTF-8 case was `{"a":"\xff"}`, and against `WireResponse` that **parses**
— every field is `#[serde(default)]`, so `"a"` is an unknown key whose value
serde skips without decoding it. The case failed on `missing required field
"view"`, which is a different claim entirely.

Re-measured against `WireResponse` itself:

| bytes | outcome |
|---|---|
| empty | `Err(EOF while parsing a value at line 1 column 0)` |
| `{"view":null}` | `Ok` |
| `{"view":null}\n  ` | `Ok` — trailing whitespace is not trailing content |
| `{"view":null} {"view":null}` | `Err(trailing characters at line 1 column 15)` |
| `{"a":"\xff"}` — bad byte in a **skipped** value | **`Ok`** |
| `{"view\xff":null}` — bad byte in a **key** | `Err(invalid unicode code point at line 1 column 8)` |
| `{"view":{…,"title":"\xff",…}}` — bad byte in a **read** value | `Err(invalid unicode code point at line 1 column 36)` |
| `{"view":null}\xff` — bad byte **after** the document | `Err(trailing characters at line 1 column 14)` |

The fixture is now the seventh row — the bad byte sits in a view's title. That is
the case `design.md:1052` is actually about: a title `String::from_utf8_lossy`
would have silently turned into U+FFFD, which is why `Exchange.result` carries
`Vec<u8>`. The fifth row is a finding and is in *Open* below.

#### Verification record

| id | mode | result | evidence |
|---|---|---|---|
| EX-1 | — | **pass** | `config.rs::the_design_s_own_example_loads` reads `design.md:1134` verbatim into the three values, with `timeout` a `std::time::Duration` and `default_poll` a `jiff::SignedDuration` — the two consumers' types, resolved at load. The three rejections are VT-2's cases below. Durations go through `jiff::Span` + `SpanRelativeTo::days_are_24_hours()`, `next_check`'s own grammar, restated deliberately per the gap-2 decision |
| EX-2 | — | **pass**, and it needed a decision | `State` holds `Option<Outstanding>`, a non-`Option` `resolved_check` and a `u64` counter. `Host::new` seeds through `schedule::resolve`'s `(None, None)` arm rather than adding `now + default_poll` a second time (I4). `view_id` is VT-5's case. `Outstanding.issued_at` is read by nothing in this slice and is kept under an `#[expect(dead_code, reason = …)]` — user decision 2026-09-03; see the Log's fourth entry, and *Open* |
| EX-3 | — | **pass** | `StateError::NoOutstandingView` and `StaleViewId` are distinct variants in `shell/error.rs`, raised by `State::verify`, which `Host::respond` calls **before** it builds a request. `state.rs`'s five colocated cases and `host.rs`'s two integration cases; the rejection-leaves-it-intact half is asserted by a *subsequent* exchange succeeding, not by inspecting state |
| EX-4 | — | **pass** | `Outcome` carries all six fields as `design.md:1063` states them, and `Presented` pairs the view with its id inseparably. `host.rs::a_returned_view_arrives_with_its_id_and_a_concrete_next_check` asserts view, id, `next_check`, empty discards and `cleanup`; `::an_unusable_next_check_is_discarded_and_the_view_still_arrives` is the only case that exercises `discarded`; `::stderr_and_the_cleanup_verdict_survive_a_failed_exchange` asserts the two fields that must survive whatever the result was (R-42, R-54) |
| EX-5 | — | **pass**, and the mechanism is the signature | `Host::no_action` takes `&self`, so a failure path *cannot* write `resolved_check` — the same move as `State::verify(&self)` for R-34, and break 3 had to change the signature to `&mut self` before it could break the rule. VT-4's three cases plus its positive control |
| EX-6 | — | **pass** | `WhenNothingToShow` is the one place the two entry points differ after the bytes are in. `::a_null_view_answering_an_evaluate_leaves_the_interaction_open` (the interaction survives, proven by a later `respond` succeeding) and `::a_null_view_answering_a_respond_closes_the_interaction` (proven by a later `respond` being refused as `NoOutstandingView`). `::a_view_returned_by_a_respond_replaces_the_one_it_answered` covers the state diagram's self-transition (R-33) |
| EX-7 | — | **pass**, and seen to fail | `host.rs:13` carries `#![deny(clippy::arithmetic_side_effects)]`; `config.rs` and `state.rs` do not, and each says why in its own module doc — the rule is about the data, not the directory. Break 1: one `discarded.len() + 1` in `host.rs` fails the gate, naming line 13. In **host code**, which is what F-14 requires: the same expression under `tests/` proves nothing |
| EX-8 | — | **pass**, with the qualification the re-measurement found | `read()` in `host.rs` is the crate's only `from_slice` over a backend's bytes. `::a_body_that_is_not_exactly_one_json_document_is_a_protocol_failure` asserts all three cases as `Protocol(Json)`, and `::a_document_followed_by_whitespace_is_still_one_document` asserts the other half of R-38 — trailing whitespace is not trailing content. The qualification is *Open*'s first item: a byte serde **skips** is never decoded |
| VT-1 | test | **pass** | twelve cases in `tests/integration/host.rs`, all against `fake::FakeBackend`. No case in this phase spawns a process |
| VT-2 | test | **pass**, one per clause, each naming its error | `::an_empty_command_is_rejected_because_there_is_nothing_to_spawn` (`EmptyCommand`), `::a_zero_timeout_is_rejected_because_it_fails_every_exchange` and `::a_zero_default_poll_is_rejected_because_it_is_a_busy_loop` (`NonPositive`, each matching its own key). Two more than the criterion asks for: `::a_missing_section_is_refused_by_the_parser_rather_than_by_a_check` (`Syntax`) and the missing-file half of `::a_configuration_is_read_from_a_file_and_a_missing_one_says_so` (`Read`), so every `ConfigError` variant but `Duration` has a case |
| VT-3 | test | **pass**, and seen to fail | `::an_answer_against_an_idle_host_is_refused_and_no_exchange_happens` asserts `NoOutstandingView` **and** `calls.count() == 0`; `::a_superseded_id_is_refused_and_the_outstanding_interaction_survives` asserts `StaleViewId` naming both ids, an unchanged call count, and that the live interaction still answers. Red under break 2 with the count doing the catching: `a stale answer must not reach the backend, left: 3, right: 2` |
| VT-4 | test | **pass**, and not vacuous | `::no_failure_moves_the_schedule` over a timeout, a non-zero exit and malformed JSON. `::a_successful_exchange_does_move_the_schedule` is its positive control and is load-bearing: without it the case would pass against a host that never updates the schedule at all, which is F-8's mistake in this phase's shape |
| VT-5 | test | **pass**, and seen to fail | `state.rs::a_fixed_now_and_counter_produce_the_id_the_design_documents` asserts four exact ids and reaches `design.md:1216`'s own worked example, `2026-08-23T04:12:00Z#3`, by the counter alone. Red under the format break with `left: "2026-08-23T04:12:00Z-0", right: "2026-08-23T04:12:00Z#0"` |
| VT-6 | test | **pass**, and seen to fail | the same case as EX-8, asserting for each of the three that `next_check` is still the seeded value — EX-5's rule applied to this failure. Red under break 4, which failed **only** this case: a reader that stops at the first document accepts two |
| VA-1 | agent | **pass** | `just check` exits 0, both feature columns — 35 unit, 27 integration, 15 protocol. Pasted in the Log |

#### Log

- 2026-09-03 — sheet written. Entry criteria checked and met; baseline green at
  `532c244`, 22 unit / 15 integration / 15 protocol. `cargo tree
  --no-default-features` shows `jiff`, `serde`, `serde_json` and their own
  dependencies, and neither `tokio` nor `toml`. Four measurement groups taken
  against the real crates and tabulated above. **Two plan gaps raised, both
  open**: no error type exists for a rejected config, and the duration grammar
  would be stated twice.

- 2026-09-03 — **both gaps closed on the recommended option** (`plan-log.md`).
  `ConfigError` joins `StateError` in `shell/error.rs`, five variants; the
  duration grammar is restated in `config.rs` and the duplication recorded.
  `plan.md`'s Surfaces line amended for both, including the explicit **not**
  `src/semantics/schedule.rs` so the second decision cannot be quietly reversed.

- 2026-09-03 — `error.rs`, `config.rs`, `state.rs`. Red first on all three of
  VT-2's clauses: a `parse` that deserialized and checked nothing failed each of
  them, and the design's own example, for the right reason each time. Two
  departures found while writing them, both measured rather than reasoned about:
  **`jiff::Error` does not implement `std::error::Error`** under
  `default-features = false` (D4), so `ConfigError::Duration` carries jiff's
  message in a `detail` field that `Display` reads and `Error::source` does not
  chain — named `detail` rather than `source` so the signature does not imply a
  chain it cannot provide. And **`std::time::Duration::try_from` accepts zero and
  refuses only the negative**, so the positivity check has to run *before* the
  conversion or `timeout = "0s"` would be reachable only as a conversion failure.
  `state.rs`'s two claims were both seen to fail: the id format break, and a
  `verify` that clears on rejection, which took `R-34: refusing an answer must
  not close the interaction it was not for` red.

- 2026-09-03 — `host.rs`, the fake, and the host tier. **A1 fired on the first
  run** and is the phase's most useful assumption: the invalid-UTF-8 fixture
  parsed cleanly against `WireResponse` because serde never decodes a value it
  skips. Re-measured, table above; the fixture is now a bad byte in a view's
  title, which is the case `design.md:1052` is about. **A3 holds with a
  qualification** — an `async fn` impl of the AFIT seam compiles, but `clippy`
  refuses an `async` with no `.await` in it, so the fake returns
  `std::future::ready`. That is the honest description of a scripted answer and
  needed no `Box::pin`, which was A3's actual risk. **A2, A4 and A5 all held.**

- 2026-09-03 — **five break-and-revert runs, plus a sixth on the lint
  expectation.** Each is recorded against the criterion it falsifies in the
  Verification record: (1) arithmetic in `host.rs` fails the gate at line 13;
  (2) the `view_id` check moved after the exchange takes three cases red, and the
  **call count** is what catches it — `left: 3, right: 2`; (3) a failure path
  that extends the schedule takes four cases red, and had to change
  `no_action(&self)` to `&mut self` before it could even be written; (4) a reader
  that stops at the first document instead of requiring exactly one fails
  **only** the framing case; (5) an accepted `respond` that leaves the
  interaction open fails only the close case. All five reverted and the gate is
  green.

- 2026-09-03 — **EX-2's `Outstanding` restored by user decision.** The first
  implementation was `Option<ViewId>`: `design.md:1167`'s `issued_at` is read by
  nothing in this slice, and an unread private field fails the gate because
  `cargo clippy -- -D warnings` promotes rustc's `dead_code`. EX-2 names
  `Option<Outstanding>` explicitly, so shipping the narrower type was a
  criterion-level divergence rather than a coding choice. The decision was to
  keep the design's shape under `#[expect(dead_code, reason = …)]` — the hatch
  `Cargo.toml` preserves for cases worth recording. **The expectation is live,
  not decoration:** adding a read of `issued_at` produces `this lint expectation
  is unfulfilled` at `state.rs:40`, so the record clears itself the moment the
  field acquires a reader.

- 2026-09-03 — `just check` exits 0 on all six commands, both feature columns:

  ```
  cargo build
  cargo test                     35 unit, 27 integration, 15 protocol, 0 doc
  cargo test --no-default-features   22 unit, 15 protocol, 0 doc
  cargo clippy --all-targets -- -D warnings
  cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub
  cargo fmt --check
  ```

  Landed: `src/shell/{config,state,host}.rs`, `ConfigError` and `StateError` in
  `src/shell/error.rs`, three `pub mod` lines in `src/shell/mod.rs`,
  `tests/integration/{fake,host}.rs` and two lines in
  `tests/integration/main.rs`. Nothing else was touched.

### PHASE-08 — The round trip and the example backends

**State:** **done 2026-09-03.** `just check` exits 0 on all seven commands in
both feature columns — 35 unit, 32 integration, 15 protocol. All four EX and all
four V criteria are discharged in the Verification record below; entry criteria
checked and met, and the expansion's measurements are recorded. **One plan gap
raised at expansion and closed the same day by user decision**: `deno run` does not typecheck, which
is the reason OQ-9 gives for choosing deno — the gate now runs `deno check`, and
the plan gained **EX-6** for it.
**Plan entry:** `docs/slices/001/plan.md:924`
**Surfaces (from the plan, as amended by the EX-6 decision):**
`examples/typescript/**`, `tests/backends/**`, `tests/integration/**`,
`justfile`, and `design.md` §9's command block. **Not** `slice-001.md` — its
OQ-9 answer still carries the wrong claim and PHASE-09's sweep owns it.

#### Reading list

| what | where | why |
|---|---|---|
| the phase | `plan.md:918`–`:975` | the three EX and the four VT this sheet expands, and the three implementer notes |
| the round trip, as a sequence | `design.md:1574`–`1599` | the order EX-1 asserts, and the one diagram in the design that names deno |
| the host validates nothing but the id | `design.md:1753`–`1757` | VT-5 exactly. `respond` checks the `view_id` and **nothing else**; field values pass through opaque |
| backends are trusted user programs | `design.md:147`–`:151`, brief §14 | EX-2's prohibition: `-A` is not a sandbox and the example's comments may not imply one |
| hermeticity | `design.md:166`–`:169` | "`cargo test` must be able to spawn a backend with no build step and no `node_modules`" — the constraint the example is written against |
| the tree the design fixes | `design.md:332`–`:333` | `examples/typescript/` — the showcase backend, deno |
| the config the design writes | `design.md:1136` | `command = ["deno", "run", "-A", "./backend.ts"]`, which EX-2 names as argv |
| the scenario the example should read like | brief §18, `docs/brief.md:866`–`:905` | the interstitial journal: evaluate → choice with `next_check`, respond → `view: null`. The example is documentation (plan's first implementer note) and this is the brief's own worked example |
| what a backend receives | `src/semantics/protocol/canonical.rs:507`–`:555`, and the two wire-form tests at `:726` and `:747` | the exact request JSON, both kinds. The example parses this, so it is written against the snapshots rather than against prose |
| what a backend may answer | `tests/protocol/fixtures/protocol/R-15-an-option-carrying-fields.json`, `R-16-a-number-field-with-bounds.json` | an accepting fixture states the whole canonical value, so these are the readable statement of the response shape the example emits |
| the requirements | `draft-spec.md:139` (R-35), `:145` (R-36), `:162` (R-45) | R-35 is VT-5, R-36 is EX-3's shape, R-45 is PHASE-10's and is why the harness must reuse a `Host` |
| the verification rows | `draft-spec.md:373` (R-35), `:374` (R-36), `:386` (R-45) | what the spec says each is proven by |
| the harness this phase extends | `tests/integration/harness.rs` | `backend`, `transport`, `evaluate`/`padded_evaluate`, `describe`, `stderr`, `marker`/`clear`, and the `Display`-not-`Debug` rule for panic messages |
| the host tier's own shape | `tests/integration/host.rs:1`–`:120` | `describe_outcome`, `presented`, `state_error`, and `an_answer()` — the precedent for harvesting an `OptionId` from a view, which VT-5 needs a foreign one of |
| the fake | `tests/integration/fake.rs` | the call count that PHASE-07 used for AC-8, and the note that says VT-2 makes the same claim through a real process |
| the transport tier's case style | `tests/integration/transport.rs:1`–`:70` | per-case timeouts rather than a shared constant, and what a discriminating assertion looks like here |
| the backend scripts | `tests/backends/reads-stdin-then-answers.sh` | the declarative one-behaviour-per-script convention, the echo-to-stderr trick, and the no-shebang rule |
| spawn, as it happens | `src/shell/backend/process.rs:53`–`:95` | no `env_clear`, no `current_dir`: the child inherits the test binary's environment and cwd, which is what makes `$PPID` and `$TMPDIR` usable below |
| what PHASE-10 will need from here | `plan.md:995`–`:1050` | EX-2 runs a **sequence** against **one** `Host`, so this phase's harness owes a reusable `Host<ProcessBackend>` |

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | PHASE-07 discharged, EX-7 included | **met.** EX-1…EX-8, VT-1…VT-6 and VA-1 are all recorded `pass` with named evidence in PHASE-07's Verification record. EX-7 is the `#![deny(clippy::arithmetic_side_effects)]` at `src/shell/host.rs:13`, seen to fail under break 1 |
| EN-2 | `deno` available in the dev shell | **met.** `deno 2.9.4 (stable, release, x86_64-unknown-linux-gnu)`, `/nix/store/pn1qbka1qfxw0wfbh1scsd2gvhv0dhj2-deno-2.9.4/bin/deno`; `flake.nix:53` puts `pkgs.deno` in `projectPkgs` |

Baseline, 2026-09-03: `just check` exits 0 on all six commands, both feature
columns — 35 unit, 27 integration, 15 protocol. PHASE-07 is committed at
`bc06d6d`. Any failure from here is this phase's.

#### What already exists — inspected 2026-09-03

| path | state | consequence for this phase |
|---|---|---|
| `examples/` | **does not exist** | the whole of EX-2 is new. `design.md:332` fixes the directory name |
| `tests/backends/` | twelve scripts, every one a *misbehaviour* | EX-3's is the first well-behaved bash backend other than `reads-stdin-then-answers.sh`, which answers `{"view":null}` and nothing else — it cannot carry a round trip |
| `tests/integration/harness.rs` | `backend(name)` → `["bash", <abs>/tests/backends/<name>.sh]`; `transport(name, timeout)`; the `/proc` cluster; `marker`/`clear` | `backend` is the pattern the deno example's argv follows. Nothing here builds a `Config` or a `Host` — that is this phase's addition, and PHASE-10 inherits it |
| `tests/integration/host.rs` | twelve cases, all against the fake; `CONFIG` is a `parse`d string with a command that cannot run | a config for a *real* process is new. The `Config::parse`-from-a-string route is the one to copy — no temp file needed |
| `tests/integration/fake.rs` | `FakeBackend`, `Calls`, `answering`/`failing`/`failing_noisily` | the cheapest source of a canonical `OptionId` that no real view offered, which is VT-5's fixture |
| `src/shell/config.rs:97` | `Config::parse(text)` is `pub` | a case builds its config from a literal, and the command can be built by the harness rather than written into the TOML |
| `src/shell/host.rs:105` | `Host::new(config, backend, now)` | `Host<ProcessBackend>` needs nothing new in host code |

#### Measured at expansion, before anything was written

deno 2.9.4, in the dev shell, against a two-line `echo.ts` that reads stdin to
EOF and writes one JSON document. Every row decides something the example or a
test would otherwise have been written wrong.

**Startup, and hermeticity.** No imports, so nothing is fetched and nothing is
cached.

| what | measured |
|---|---|
| `deno run -A ./echo.ts`, warm | ~15–20 ms per run (5 runs, 77 ms wall) |
| the same with `DENO_DIR` pointed at an empty directory | 24 ms, and no output on stderr — there is no cold-cache penalty and no download line to confuse a stderr assertion |
| the same with `DENO_DIR` **read-only** (`chmod 500`) | exit 0, correct output. The example needs no writable cache |
| `deno run -A --check ./echo.ts`, warm | ~87 ms per run (5 runs, 433 ms wall) |
| stderr on a clean run | empty. What the script writes is all that arrives |

So a 5-second timeout is four orders of magnitude of headroom, and the example
is hermetic in the sense `design.md:166` requires.

**`deno run` does not typecheck.** This is the plan gap below, and it is a
measurement rather than a reading of the docs.

| file | `deno run -A` | `deno check` |
|---|---|---|
| `const n: number = "not a number"` | **exit 0**, and the value printed as the string it is | `TS2322 [ERROR]: Type 'string' is not assignable to type 'number'`, exit 1 |

**`$PPID` inside a spawned script is the test binary, and `$TMPDIR` is what
`std::env::temp_dir()` reads.** Both measured against a direct `bash script`
spawn, which is what `ProcessBackend` performs — no shell interposes (R-36).

| what | measured |
|---|---|
| caller pid `1117701`, script's `$PPID` | `1117701` |
| `${TMPDIR:-/tmp}` with `TMPDIR` unset / set to `/tmp/foo` | `/tmp` / `/tmp/foo`, matching `std::env::temp_dir()`'s own rule |

That is what lets a script and a test agree on a filename with no environment
variable to pass and no JSON to parse in bash: `harness::marker` already names
its files after `std::process::id()`, and `$PPID` is the same number.

#### One plan gap found at expansion — **closed 2026-09-03**

**`deno run` does not typecheck, and OQ-9 says it does.** `slice-001.md:244`
answers OQ-9 with deno because "it runs `.ts` directly with no build step and
typechecks rather than stripping types — which is the point of choosing
TypeScript when brief §3.7 makes agents the authors." The first half is true and
the second is false: deno has not typechecked `deno run` by default since 1.23,
and the measurement above shows a type error running to completion. `plan.md`
PHASE-08's second implementer note repeats the claim.

EX-2 names the argv — `["deno", "run", "-A", …]` — so the phase as written ships
a backend nothing typechecks, and the stated reason for preferring TypeScript to
JavaScript does not hold. Three ways out, and the choice is the user's:

1. **Correct the record, add nothing.** EX-2's argv stands; OQ-9's answer and the
   plan's note are amended at PHASE-09 to say deno was chosen for zero-build
   execution, and that typechecking is a separate command. The example ships
   unchecked, and an agent editing it gets no type feedback from running it.
2. **`--check` in the argv.** `["deno", "run", "-A", "--check", "./backend.ts"]`
   restores the claim exactly, and costs ~70 ms per exchange — every exchange,
   for every user of the example config, not just in the suite.
3. **Typecheck in the gate.** `just check` gains `deno check
   examples/typescript/backend.ts`, so the claim holds where it is useful (at
   edit time) and costs nothing at run time. `justfile` and `design.md` §9's
   command block are **not** in this phase's Surfaces, so this is a plan
   amendment as well as a decision — and it puts deno on the critical path of
   the phase gate, which AC-1 says must work from a clean clone in the dev
   shell.

**Recommendation: 3, with 1's correction to the record made anyway.** The reason
OQ-9 gives for TypeScript is real — an agent author wants the types checked —
and option 3 is the only one that delivers it where an author is, rather than on
every exchange forever. It is one line in the `justfile`, and deno is already an
AC-1 dev-shell dependency (EN-2), so the gate acquires no new tool. Option 1
alone leaves the slice shipping a documentary example whose types are decoration.

**Decided: option 3, 2026-09-03** (`plan-log.md`, `design-log.md`). `design.md`
§9 is seven commands, `justfile` mirrors it as `typecheck`, and `plan.md`
PHASE-08 gained **EX-6** plus the two extra Surfaces. PHASE-01's discharged
criteria still enumerate six and are deliberately not restated; a comment above
PHASE-01 says so and points at the decision. The gate is red until
`examples/typescript/backend.ts` exists, which is why task 1 is task 1.

#### Settled here — implementer latitude

Four choices the design and plan leave open. None is a plan gap; each is
recorded because a later reader will ask why.

**The example decides from the request, and owns no state.** EX-1 wants one
backend to answer `view: null` and then a choice, and this transport spawns a
fresh process per exchange (`design.md` §5.4) — so a stateless backend can only
vary its answer by reading the request. The example therefore keys on
`event.data`, which is opaque to the host (R-9) and is exactly what an emitter
supplies. Brief §18's backend "checks whatever state it owns"; the example says
in a comment where that state would live and that the host never learns of it,
rather than writing a file the suite would then have to clean up. The
alternative — a state file — would make the example's behaviour depend on
filesystem residue between exchanges, which is a worse thing to hand an agent to
copy.

**The bash backend matches strings, and says so.** It exists for AC-12 — to
distinguish a transport that works for any configured command from one that
works for deno — not to be a showcase. Bash has no JSON parser, and adding one
would be a dependency the dev shell does not declare, so it selects its answer
with a `case` over the raw request text. The comment says the test controls the
request, which is what makes a string match sound here and unsound in an example.

**"The backend was not spawned" is witnessed by a file, not by the host.** VT-2's
first case points the config at a program that does not exist, so a spawn that
happened would arrive as `Failure::Backend(Spawn)` rather than
`Failure::State(_)` — the error variant is the discriminator. Its second case
cannot do that: the host must reach the backend once to obtain a view before a
stale answer can be refused. There the script appends one line per invocation to
`${TMPDIR:-/tmp}/goad-invocations-$PPID`, which the measurement above shows is
the path `harness::marker` builds, and the test asserts the count did not move
across the refusal — with a *subsequent* accepted answer as the positive
control, because a witness that never moves proves nothing. This is PHASE-06's
lesson applied: a bound, or a non-event, needs a question the host cannot answer
for itself.

**VT-5's foreign `OptionId` comes from the fake.** `OptionId` is deliberately not
publicly constructible (D30), so an answer naming an option no view offered still
has to be harvested from *some* view. The cheapest source in this tier is a
`Host<FakeBackend>` answering a canned body whose option id is spelled
`an-option-no-view-offered`, which also makes the test's intent legible at the
assertion. `host.rs::an_answer` is the precedent.

#### Assumptions — each a place this phase can break

- **A1 — the test binary's cwd is the crate root, and nothing relies on it.**
  Every path this phase builds is rooted at `CARGO_MANIFEST_DIR`, as
  `harness::backend` already does, so the assumption is only that the *example's
  own documentation* may use a relative path (`design.md:1136`'s `./backend.ts`)
  while the suite does not. If a case is written with a relative path it will
  pass from `cargo test` at the root and fail from elsewhere.
- **A2 — `$PPID` in a script spawned by tokio is the test binary.** Measured
  against a direct spawn, and `ProcessBackend` spawns directly. If tokio's
  `Command` ever forks an intermediary, the invocation witness silently stops
  matching and VT-2's second case becomes vacuous — so the case asserts the
  positive control first.
- **A3 — one `Host` can be driven through a sequence with `#[tokio::test]`'s
  single-threaded runtime.** `evaluate` and `respond` take `&mut self` (I6), so a
  sequence is sequential by construction; nothing here needs `spawn`.
- **A4 — deno reads stdin to EOF and exits.** `new Response(Deno.stdin.readable)`
  resolves when the host closes stdin, which is R-37's rule and what
  `reads-stdin-then-answers.sh` already relies on. Measured above.
- **A5 — the `next_check` the example emits is one the host accepts.** It writes
  brief §18's own `"45 minutes"`, which `R-21-next-check-as-a-relative-span.json`
  covers. If it did not, the outcome would carry a discard and EX-1 would still
  pass — so the round-trip cases assert `discarded.is_empty()`.

#### STOP conditions

- The plan gap above is **not** settled by the user. EX-2's argv is the phase's
  first line of code; do not write it, or the example's comments, either way.
- The example needs anything that is not a `.ts` file — an import map, a
  `deno.json`, a lockfile, a `node_modules` — which is EX-2's own prohibition and
  `design.md:166`'s constraint.
- A case needs the host to grow an accessor, an env-passing config key, or a cwd
  setting. That is a design change, not a test fixture.
- The bash round-trip backend needs a shebang or an executable bit (R-36, AC-12).
- Anything wants to touch `justfile`, `design.md` or `slice-001.md`. The
  documentary correction the gap asks for belongs to PHASE-09; only a plan
  amendment endorsed by the user puts the `justfile` in this phase's Surfaces.

#### Tasks

1. `examples/typescript/backend.ts` — the showcase. Reads one JSON document,
   answers one. Brief §18's shape, keyed on `event.data`, with the §14 note about
   `-A` and a comment saying where a real backend's state would live.
2. `examples/typescript/README.md` — how to point a config at it, and nothing
   else. Documentation for a copier, not a second specification.
3. `tests/backends/answers-a-round-trip.sh` — EX-3's bash backend: echoes the
   request to stderr, appends one line to the invocation witness, and selects its
   answer with a `case`.
4. `harness.rs` — `example(name)` for a deno argv, `config(command, timeout,
   default_poll)` for a `Config` built around a command, `host(...)` returning a
   reusable `Host<ProcessBackend>`, and `invocations()` for the witness. The
   `Host` constructor is what PHASE-10/EX-2 inherits.
5. `tests/integration/round_trip.rs` — VT-1, VT-2, VT-3 and VT-5, red first.
6. Break and revert each claim, one at a time, recorded against its criterion.
7. EX-6's break-and-revert: a type error in the example must take `just check`
   red at the `typecheck` recipe. A check nobody has seen fail is not a check.
8. `just check`, both columns. Update this sheet, the Status table and the
   Harvest.

#### The plan's suggested mechanism for VT-2 was vacuous — found by breaking it

VT-2 says to assert the backend was not spawned by "pointing the config at a
backend that would fail if it ran". The first draft did exactly that — a command
naming `/nonexistent/goad-must-not-spawn-this` — and the case **stayed green
under both breaks that reorder the check**, which is the class of defect it
exists to catch:

| break | what it does | the nonexistent-command case | the invocation log |
|---|---|---|---|
| 4 | `respond` verifies the id *after* the exchange | **passed** | count 1, expected 0 — red |
| 5 | `respond` verifies first, then forwards the answer anyway | **passed** | count 1, expected 0 — red |

The reason is the same both times: a host that spawns and refuses afterwards
still *returns* the refusal, so `Failure::Backend(Spawn)` never reaches the
caller and there is nothing for the variant assertion to catch. The criterion's
claim — "the backend was **not** spawned" — is about something that did not
happen, and only a witness outside the host can speak to it. The case now runs
the logging backend and asserts a count of zero, which catches both breaks. This
is PHASE-06's own lesson arriving a second time: a bound is not tested by
asserting the outcome at the bound.

#### Verification record

| id | mode | result | evidence |
|---|---|---|---|
| EX-1 | — | **pass** | `round_trip.rs::the_deno_example_completes_a_round_trip`: one `Host`, three processes. `view: null` on the quiet event; a choice on the prompting one, arriving as `Presented { view_id, view }`; a `respond` carrying that id accepted, with `view: null` and a moved schedule; and a fourth exchange refused as `NoOutstandingView`, so the interaction is seen to have closed. The id is checked **through the backend**, not through host state: the example writes `answered <view_id> with yes` to stderr, and the case asserts that string exactly (F-23, AC-7) |
| EX-2 | — | **pass** | `examples/typescript/` is two files — `backend.ts` and `README.md`. No `deno.json`, no lockfile, no `node_modules`, no build step; run as `["deno", "run", "-A", <path>]` by `harness::example`. Measured hermetic at expansion: 24 ms against an empty `DENO_DIR`, and exit 0 against a **read-only** one. Both the script's header and the README's *Trust* section say `-A` grants the user's full authority and that deno's default-deny permissions are not a security boundary here (brief §14, OQ-9) |
| EX-3 | — | **pass**, and the shape is load-bearing | `tests/backends/answers-a-round-trip.sh`, invoked as `["bash", <script>, <log>]` — no shebang, no executable bit. Break 3 removed `bash` from argv and all three bash cases went red with `backend could not be spawned: Permission denied (os error 13)` while the deno case stayed green, which is AC-12's argument stated in the negative: a suite that only ran deno would not have noticed |
| EX-6 | — | **pass**, and seen to fail | `justfile`'s `typecheck` recipe runs `deno check examples/typescript/backend.ts`, and `just -n check` prints §9's seven commands in §9's order. Break 1 is the one that matters: `const PROMPT_AFTER_MINUTES: string = 45;` is a **behaviour-preserving** type error, so `cargo test` stayed green and the gate failed at `typecheck` with `TS2322` and `TS2365` — the recipe catches what the suite cannot. Its first attempt (`: number = "forty five"`) was rejected as a break because it changed the runtime answer and the round trip caught it first |
| VT-1 | test | **pass** | EX-1's case, against the deno example. Break 2 — `respond` forwarding a freshly minted id instead of the caller's — failed **only** this case, and only on the stderr assertion: `left: "answered 2026-08-23T04:14:00Z#1 with yes"`, `right: "…04:12:00Z#0 with yes"` |
| VT-2 | test | **pass**, after the repair above | `::an_answer_no_view_asked_for_never_reaches_the_backend` (`NoOutstandingView`, invocations 0, empty stderr, `cleanup: None`, schedule still the seeded `04:42:00Z`) and `::a_superseded_answer_never_reaches_the_backend` (`StaleViewId`, invocations unmoved at 2, then 3 for the accepted answer as the positive control). Break 5 — refuse, then forward anyway — is caught by **nothing but the counts**, at both tiers: `left: 3, right: 2` here and in `host.rs`'s fake |
| VT-3 | test | **pass** | `::the_bash_backend_completes_the_same_round_trip`: the identical three-exchange sequence, the bash view's own title, the request echoed back on stderr, and `invocations == 3` — one process per exchange and no more. Red under break 3 |
| VT-5 | test | **pass**, and seen to fail | `::an_answer_the_view_did_not_offer_reaches_the_backend_unchanged`: an answer naming `an-option-no-view-offered`, with a value under `a-field-no-option-offered`, against the bash view that offered `log`/`skip`. Both reach the backend verbatim and the exchange is **accepted** — the host validates the `view_id` and nothing else (R-35, D17). Break 6 stripped `values` before serializing and failed only this case, printing the request the backend actually saw |
| VA-1 | agent | **pass** | `just check` exits 0 on all **seven** commands, both feature columns — 35 unit, 32 integration, 15 protocol. Pasted in the Log |

#### Log

- 2026-09-03 — `just check` exits 0 on all seven commands, both feature columns:

  ```
  cargo build
  cargo test                     35 unit, 32 integration, 15 protocol, 0 doc
  cargo test --no-default-features   22 unit, 15 protocol, 0 doc
  deno check examples/typescript/backend.ts
  cargo clippy --all-targets -- -D warnings
  cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub
  cargo fmt --check
  ```

  Landed: `examples/typescript/{backend.ts,README.md}`,
  `tests/backends/answers-a-round-trip.sh`, `tests/integration/round_trip.rs`,
  one line in `tests/integration/main.rs`, the additions and the five lifted
  describers in `tests/integration/harness.rs`, their removal from
  `tests/integration/host.rs`, the `typecheck` recipe in `justfile`, and
  `design.md` §9's seventh command. Nothing else was touched.

- 2026-09-03 — sheet written. Entry criteria checked and met; baseline green at
  `bc06d6d`, 35 unit / 27 integration / 15 protocol. Three measurement groups
  taken against deno 2.9.4 and a spawned script, tabulated above. **One plan gap
  raised**: `deno run` does not typecheck, which is the reason OQ-9 gives for
  choosing deno. Four latitude choices settled and recorded.

- 2026-09-03 — **PHASE-08's code landed.** `examples/typescript/{backend.ts,README.md}`,
  `tests/backends/answers-a-round-trip.sh`, `tests/integration/round_trip.rs`
  with its `main.rs` line, and additions to `harness.rs`. **A refactor moved
  five describers** — `instant`, `describe_outcome`, `presented`,
  `backend_error`, `state_error` — out of `host.rs` and into `harness.rs`, plus
  a new `stderr_of`: the second file needing them is what the harness is for,
  and a second copy would have been a parallel implementation. `host.rs` carries
  a comment saying where they went.

- 2026-09-03 — **clippy found the witness design, not a review.** The first
  invocation log was one file per *process*, named from `$PPID`, with a
  `std::sync::Mutex` serializing the cases that read it —
  `clippy::await_holding_lock` refused it, correctly. The replacement is
  strictly better and needed no lock: the log path travels as **argv[2]**, which
  is how a command is parameterized when nothing interposes a shell (R-36), so
  each case has its own log and the `$PPID` measurement is no longer load-bearing
  for anything. A2 is therefore retired rather than discharged.

- 2026-09-03 — **the assumptions, at the end.** A1 held and was never tested,
  because every path is rooted at `CARGO_MANIFEST_DIR`. **A2 is retired, not
  discharged** — the argv-passed log removed the need for `$PPID` to mean
  anything. A3 held: three exchanges through one `Host` under
  `#[tokio::test]`'s single-threaded runtime, no `spawn` anywhere. A4 held. A5
  held and its assertion is load-bearing: every accepting case asserts
  `discarded.is_empty()`, so a `next_check` the host could not use would fail
  rather than pass quietly.

- 2026-09-03 — **six break-and-revert runs.** Each is recorded against the
  criterion it falsifies: (1) a behaviour-preserving type error fails only
  `typecheck`; (2) a minted `view_id` in the `respond` request fails only VT-1's
  stderr assertion; (3) `bash` removed from argv fails all three bash cases and
  none of deno's; (4) the id verified after the exchange fails nine cases across
  both tiers; (5) verify-then-forward-anyway is caught **only** by the two
  counts; (6) `values` stripped before serialization fails only VT-5. All six
  reverted, `git diff src/` empty, and the gate green.

- 2026-09-03 — **the gap closed on the recommended option.** §9's command block
  gains `deno check examples/typescript/backend.ts` and the `justfile` mirrors
  it; `just -n check` re-checked against the block, seven commands in §9's
  order. `plan.md` PHASE-08 gained EX-6, `justfile` and `design.md` §9 joined its
  Surfaces, its second implementer note was corrected in place, and PHASE-09/EX-1
  now says seven. The restatement sweep for "six" was run across the slice:
  `plan.md`'s Overview item 1, PHASE-01's objective and PHASE-09/EX-1 amended;
  `justfile`'s header comment amended; PHASE-01's discharged criteria, the
  verification records, both review ledgers and the earlier log entries left
  alone as the historical record they are.

### PHASE-10 — The failure matrix end to end

**State:** **done 2026-09-03.** `just check` exits 0 on all seven commands in
both feature columns — 35 unit, **52** integration, 15 protocol. All four EX and
all three VT criteria are discharged in the Verification record below, VA-1 and
VA-2 pasted; entry criteria checked and met. **One question of scope was put to
the user at expansion and closed the same day** — EX-2's "whole misbehaving
suite" spans the transport modes as well as the protocol ones, which is what
R-45 claims. Four choices of test mechanism are settled below. Two of this
phase's own assertions were vacuous and both were found by breaking them; six
break-and-revert runs in all, and VA-2's walk recorded two gaps under Harvest /
Open.
**Plan entry:** `docs/slices/001/plan.md:998`
**Surfaces (from the plan):** `tests/backends/**`, `tests/integration/**`.
Nothing under `src/`, nothing in `docs/` but this sheet.

#### Reading list

| what | where | why |
|---|---|---|
| the phase | `plan.md:998`–`:1067` | the four EX, the three VT and two VA this sheet expands, and the two implementer notes |
| the misbehaving-backend list | `design.md:2056`–`:2090` | VA-2 walks this prose item by item. It is the only statement of the set, and it is a paragraph rather than a table |
| the three tiers and what each drives | `design.md:1975`–`:1985` | integration drives AC-5, AC-6, AC-7 and AC-12; the protocol tier drives the corpus. The overlap this phase creates is deliberate and this table is why |
| failure does not move the schedule | `design.md:1615`–`:1619` | EX-3 exactly, and the reason: a failed exchange that cleared the schedule turns a broken backend into a silent host |
| non-zero exit beats parseable stdout | `design.md:1625`–`:1628` | one of EX-4's six modes, and the one where the body must be discarded rather than used |
| one exchange in flight, one concrete `next_check` | `design.md:1663` (I6), `:1666` (I12) | I6 is what makes EX-2's sequence sequential by construction; I12 is why every case below can assert a `next_check` even on a failure |
| the stratum 1 taxonomy | `src/semantics/error.rs:17`–`:93` | the exact variants and field names VT-1 asserts. `ScheduleError` is the one that never arrives as an `Err` |
| the stratum 2 taxonomy | `src/shell/error.rs:23`–`:100` | `BackendError`'s seven variants and `StateError`'s two — what VT-3 asserts a caller receives |
| where each failure becomes an `Outcome` | `src/shell/host.rs:167`–`:230` | `exchange`'s four steps and `no_action`. Reading this is how a case knows which variant to expect before it is written |
| the thirteen bodies, verbatim | `tests/protocol/fixtures/protocol/` — `R-3-…`, `R-13-a-choice-with-no-options`, `R-14-…`, `R-52-duplicate-field-ids-within-one-option`, `R-12-an-unknown-field-kind`, `R-10-view-omitted`, `R-25-next-check-of-the-wrong-type`, `R-17-inverted-bounds`, `R-50-min-on-a-text-field`, `R-50-options-on-a-number-field`, `R-51-next-check-null`, `R-51-protocol-null`, and `schedule/R-23-calendar-unit-months` | each fixture's `input` is the body to emit and its `expect` is the variant and path to assert. Copying them keeps the two tiers making the same claim about the same bytes |
| the harness | `tests/integration/harness.rs:186`–`:300` | `host`, `config`, `logging_backend`, `invocations`, and the five describers. This phase adds to it and rewrites none of it |
| what EX-4 already has | `tests/integration/round_trip.rs:220`, `:257` | the two `StateError` modes, through `Host`, over the real transport, asserting the variant a caller sees — PHASE-08 wrote them for AC-8 |
| the same claim against the fake | `tests/integration/host.rs:298`, `:326` | `no_failure_moves_the_schedule` and its positive control. EX-3 is this through the real transport (R-29), not a second copy of it |
| the parameterization precedent | `tests/backends/answers-a-round-trip.sh` | argv[2] as the invocation log, and why not an environment variable |
| the requirements | `draft-spec.md:124` (R-25), `:128` (R-29), `:162` (R-45) | R-45 is EX-2's whole substance |
| the verification rows | `draft-spec.md:369` (R-29), `:386` (R-45) | R-45's row still describes PHASE-08's pre-split scope. Recorded Open by PHASE-08; **PHASE-09/EX-3 owns the correction**, not this phase |

#### Entry criteria — checked, not assumed

| id | criterion | state |
|---|---|---|
| EN-1 | PHASE-08 discharged; the harness can run a sequence of exchanges against one `Host` | **met.** PHASE-08's Verification record has all four EX and all four V criteria `pass`. `harness::host(command, timeout, now)` at `tests/integration/harness.rs:246` returns a `Host<ProcessBackend>` by value, and `round_trip.rs:257` already drives four exchanges through one |

Baseline, 2026-09-03: `just check` exits 0 on all seven commands, both feature
columns — 35 unit, 32 integration, 15 protocol. PHASE-08 is committed at
`d7312aa`. Any failure from here is this phase's.

#### What already exists — inspected 2026-09-03

| path | state | consequence for this phase |
|---|---|---|
| `tests/backends/` | thirteen scripts. Every one names a *transport-level* behaviour — hang, flood, exit non-zero, unparseable stdout, grandchild — and none emits a protocol-level misbehaviour | all thirteen EX-1 bodies are new. Four of EX-4's six modes are already scripted and this phase writes no new script for them |
| `tests/integration/round_trip.rs` | five cases, all well-behaved or refused | EX-4's two `StateError` modes are here and discharged; the other four are not |
| `tests/integration/transport.rs` | the same four transport modes, asserted as `BackendError` at the transport | EX-4 is the other end of these: not the transport's error, but the `Outcome` a caller receives. Not a duplicate — a different subject |
| `tests/protocol/fixtures/protocol/` | 52 fixtures; every EX-1 mode is among them, with its expected variant and path stated | the bodies and the assertions are copied from here rather than invented |
| `src/shell/backend/process.rs:41` | `ProcessBackend` holds a command and a timeout and nothing else | EX-2's one-`Host` reuse needs no transport change: there is no per-exchange state to corrupt. That the *host* survives is the claim, and it is not free — `State` and the schedule are mutable |

#### Settled here — implementer latitude

Four choices the design and plan leave open. None is a plan gap.

**One parameterized script, not thirteen.** EX-2 requires a *single* command to
emit a different misbehaviour on each of thirteen consecutive exchanges: a
`Host` is built around one command at construction, and the transport spawns a
fresh process per exchange with no state between them. So a backend that varies
its answer by invocation index is not a convenience, it is the only shape EX-2
admits. Giving VT-1 a *second* mechanism — thirteen one-line scripts — would then
be two mechanisms for one claim, and the individual cases would no longer be
running what the suite case runs. So:
`tests/backends/answers-as-instructed.sh`, argv[2] the invocation log and
argv[3…] the instructions, one per invocation, falling back to a well-behaved
response once the list is exhausted. The index is the line count of the log,
which is state on disk because the process cannot hold it. Its behaviour — "do
what you were told to, in the order you were told" — is one behaviour, so the
one-behaviour-per-script convention survives.

An instruction is a **response body**, or one of four sentinels naming a
transport-level misbehaviour, which is the scope decision below:
`@hang` (never answer), `@exit1` (a valid body, then exit 1), `@flood` (`exec
yes` past the stdout cap) and `@garbage` (exit 0 with a body that will not
parse). Each is the behaviour an existing script already has, quoted from it,
because those scripts are where the `exec` reasoning is written down — a bare
`yes` would fork and become PHASE-06's grandchild case instead of this one.

**The bodies are Rust literals beside their assertions, not fixture files.** A
fixture is an envelope with the body nested under `input`, and bash has no JSON
parser to lift it out. Reading the file whole would also couple the two tiers the
wrong way round: an edit to a protocol fixture would silently change what an
integration case asserts, and the two tiers exist to make the *same* claim
*independently* — normalization refuses these bytes; the refusal survives the
transport, the host and the `Outcome`. The fixtures are the source the literals
are copied from, named per case in a comment.

**EX-3 is discharged inside VT-3's cases, not by a fourth test.** EX-3 asks that
the schedule be unchanged across a timeout, a non-zero exit and a
malformed-JSON exchange, through the real transport. VT-3 already runs exactly
those three exchanges through a `Host` to assert the `Outcome` variant, so each
of them also asserts `next_check` is the seeded instant. A separate test would
spawn the same three processes to assert half as much. `host.rs:298` makes the
same claim against the fake; this is R-29 through a real fork and a real pipe.

**EX-4's stale and unknown `view_id` modes are already discharged, and this
phase writes no third case.** `round_trip.rs:220` and `:257` run them through
`Host` over the real transport, assert `NoOutstandingView` and `StaleViewId`
respectively, and carry an invocation-log witness that no process was spawned —
which is more than EX-4 asks for. The plan's first implementer note governs:
where a case adds nothing, say so here rather than assert the same call twice.
The Verification record cites them by name.

#### One question of scope, put to the user at expansion — **closed 2026-09-03**

**EX-2's "whole misbehaving suite" is ambiguous, and one `Host` is built around
one command.** So what the suite can contain is bounded by what a single command
can be made to do. Two readings: EX-1's thirteen protocol bodies (EX-2 sits
directly after EX-1, so that is the literal one), or every misbehaving mode
including the transport ones.

The literal reading is the weaker half of R-45. "No backend failure may leave the
host unable to invoke the backend again" is a claim about *host* state surviving
*process* failure, and a protocol-level refusal never touches a process
lifecycle — the transport spawned, read and reaped cleanly, and a pure function
declined the bytes. The failures that could plausibly wedge a host are the ones
that leave a child, a reader or a descriptor behind.

**Decision, user, 2026-09-03: span the transport modes too.** The emitter honours
the four sentinels above, so the one-`Host` sequence runs a timeout, a non-zero
exit, an output flood and malformed stdout alongside the thirteen bodies. VT-3
still gives each transport mode its own case and its own `Host`, because VT-3
asserts a *variant* and the suite case asserts *survival*; those are different
claims and the suite case is a poor place to read either. No plan text changes —
this settles which reading EX-2 has, and the Verification record will say so.

#### Assumptions — each a place this phase can break

- **A1 — a JSON body survives argv verbatim.** Nothing interposes a shell
  (R-36), so the body is one argv element and `printf '%s\n' "$body"` writes it
  back byte for byte. If any quoting or word-splitting appears, every VT-1 case
  fails at once and loudly — a mangled body is a `Json` error, not the variant
  the case expects.
- **A2 — the invocation log is a reliable sequence counter.** PHASE-08 measured
  the mechanism; what is new is reading it as an *index* rather than a count.
  Each case names its own log through `harness::marker`, so concurrent cases
  cannot advance each other's index.
- **A3 — thirteen failed exchanges leave the host able to spawn.** This is EX-2's
  claim rather than a background assumption, and it is stated here because the
  case is worthless if a fresh `Host` is used by accident. The final well-behaved
  exchange is the positive control, and it asserts the schedule *moved* — a
  success that changes nothing is indistinguishable from a fourteenth failure.
- **A4 — every EX-1 mode reaches the caller as `Failure::Backend(Protocol(_))`,
  except the three schedule ones.** `read` maps `from_slice` and
  `normalize_response` alike into `BackendError::Protocol`. An invalid
  `next_check` is not an error at all: it travels in `Outcome::discarded` on an
  accepted message (P2, R-25), so those cases assert `failure.is_none()` and
  inspect `discarded`. If this is wrong for any one mode, the design's list and
  the taxonomy disagree — a finding, not a repair.
- **A5 — a body with a trailing newline is one document.** `host.rs:280` says
  trailing whitespace is not trailing content. The script writes `printf '%s\n'`,
  so every case depends on it.

#### STOP conditions

- Any EX-1 mode turns out not to reach the caller as the design's list says it
  does. That is a disagreement between the taxonomy and §9's prose, and it is the
  user's to settle — not a case rewritten until it passes.
- EX-2 will not pass without a fresh `Host` per case. The requirement is reuse;
  a suite that satisfies it by construction has stopped testing it.
- A case needs `Host`, `State` or the transport to grow anything — an accessor, a
  reset, a constructor. That is a design change.
- Anything wants to touch `src/`, `design.md`, `plan.md`, `draft-spec.md` or
  `slice-001.md`. R-45's stale verification row in particular belongs to PHASE-09.
- VA-2's walk finds an item in §9's list that nothing anywhere tests. Record it
  and stop; whether it is this phase's to build is a scope decision.

#### Tasks

1. `tests/backends/answers-as-instructed.sh` — the emitter: a body, or one of
   the four sentinels. Bash builtins for the index, `cat >/dev/null` to drain
   stdin as its siblings do, and the `exec` note carried across from
   `floods-stdout-past-the-cap.sh`.
2. `harness.rs` — `scripted(case, bodies)` returning the argv and the log, on
   `logging_backend`'s shape; and two describers, one for the `ProtocolError`
   inside an `Outcome` and one for a `Discarded::Schedule`, so a failing case
   says what came instead. `Display`, not `Debug`, as the tier requires.
3. `tests/integration/failure_matrix.rs` — VT-1's thirteen cases, red first, each
   naming the fixture its body came from.
4. VT-2 — one `Host`; the thirteen bodies and the four sentinels in sequence,
   then a well-behaved eighteenth asserting the schedule moved. The timeout
   sentinel wants a short per-case timeout, as `transport.rs` does it, so the
   sequence stays fast.
5. VT-3 — the five remaining EX-4 modes — a command that cannot be spawned, a
   timeout, a non-zero exit, malformed stdout and output past the cap — each
   asserting the `Outcome` variant and that `next_check` did not move. Three of
   them are EX-3.
6. Break and revert each claim, one at a time, recorded against its criterion.
   At minimum: the index made constant (VT-1 passes, VT-2 must not); the
   final body made a failure (VT-2's positive control must fire); a `Host`
   rebuilt per case in VT-2 (must not be detectable — if it is not, the case is
   vacuous and needs a witness).
7. VA-2 — walk `design.md:2056`'s list item by item, tabulate item → test,
   record every gap.
8. `just check`, both columns. Update this sheet, the Status table and the
   Harvest.

#### Two of this phase's own assertions were vacuous — both found by breaking

**The seeded check and a re-resolved one are the same instant.** Every case
started at `now` and asserted `next_check == seeded_check()`, which is `now`
plus the default poll — and a host that *re-resolved* the schedule on a failure
would compute exactly that. Break 3 (`no_action` resolving fresh instead of
reporting the retained value) left thirteen of this file's cases green,
including all three of EX-3's. The fix is one line of setup: `instructed` now
runs a well-behaved exchange first, so the schedule is at `04:57` before the
case's own body arrives and "unchanged" is distinguishable from "recomputed".
Re-run, break 3 fails seven cases here. `host.rs:326` had already found this
against the fake — its `a_successful_exchange_does_move_the_schedule` exists for
exactly this reason, and this tier had to learn it again.

**A one-`Host` suite that only asserts the last exchange is not a test of
reuse.** The plan warns that a fresh `Host` per case satisfies EX-2 by
construction. Asserting that the final exchange succeeds does not catch it,
because a fresh host succeeds too — break 2 (a `Host` rebuilt inside the loop)
passed against the first draft. VT-2 now begins by putting a view outstanding
and moving the schedule, and reads both back after the seventeen failures: a
rebuilt host reports the seeded check and refuses the answer with
`NoOutstandingView`. Break 2 now fails on the first misbehaving exchange.

That is the third and fourth time in this slice a case has passed for the wrong
reason. Both were found the same way, and neither by reading.

#### VA-2 — `design.md:2056`'s list, item by item

Every item, and the test that holds it. "end to end" means through `Host` over a
real process; the protocol tier means a fixture through `normalize_response`.

| § 9's item | where |
|---|---|
| sleeps past the timeout | `transport.rs::a_backend_that_never_answers_times_out_and_is_disposed_of`; **end to end** `failure_matrix::a_backend_that_never_answers_reaches_the_caller_as_a_timeout`, and `@hang` in VT-2 |
| sleeps past the timeout **after writing to stderr** (F-3) | `transport.rs::stderr_written_before_a_hang_survives_the_timeout` — the transport tier's, and the assertion is about the capture |
| floods stdout past the cap (F-2) | `transport.rs::a_stdout_flood_is_refused_and_the_backend_sees_the_stream_close`; **end to end** `failure_matrix::output_past_the_cap_reaches_the_caller_as_output_too_large`, and `@flood` in VT-2 |
| exits non-zero after writing valid JSON | `transport.rs`; **end to end** `failure_matrix::a_non_zero_exit…`, and `@exit1` in VT-2. Same item as "writes a valid response and then exits non-zero" below — §9 names it twice |
| writes malformed JSON | `host.rs` (fake); **end to end** `failure_matrix::a_body_that_will_not_parse…`, and `@garbage` in VT-2 |
| writes nothing | `host.rs:167` against the fake — empty stdout is an unexpected EOF, so `Protocol(Json)`. **Not end to end.** Not in EX-1's list; recorded below |
| declares an unknown protocol version | protocol tier; **end to end** VT-1 |
| returns `options: []` | protocol tier; **end to end** VT-1 |
| returns duplicate option ids | protocol tier; **end to end** VT-1 |
| returns an unknown `kind` nested in a field, asserting `at` (F-6) | protocol tier; **end to end** VT-1, path asserted |
| omits `view` entirely (F-5) | protocol tier; **end to end** VT-1 |
| `"next_check": 45` | protocol tier; **end to end** VT-1, as a discard |
| `"next_check": "1 month"` (F-10) | schedule corpus; **end to end** VT-1, as a discard |
| `min: 10, max: 1` (F-9) | protocol tier; **end to end** VT-1 |
| floods stderr past its cap **and then succeeds** (F-25) | `transport.rs::a_stderr_flood_is_truncated_and_the_exchange_still_succeeds` |
| a text field carrying `min`, a number field carrying `options` (F-45) | protocol tier; **end to end** VT-1, both, key/kind/path asserted |
| two fields in one option sharing an id (F-52) | protocol tier; **end to end** VT-1, path asserted |
| `"next_check": null` and `"protocol": null`, nothing discarded (F-50) | protocol tier; **end to end** VT-1, both |
| grandchild holding **stderr only** (F-48, F-53, F-63) | `transport.rs::a_grandchild_holding_stderr_costs_the_cleanup_budget_and_nothing_else` |
| the same leaving a grandchild holding **stdout too** (F-63) | `transport.rs::a_grandchild_holding_stdout_too_fails_both_dimensions` |
| a valid response then `exit 1` (D15, R-40, F-59) | as above — **end to end** in `failure_matrix`, body discarded and stderr kept |
| exits 0 with unparseable stdout after writing to stderr (F-24) | `transport.rs::a_zero_exit_with_an_unparseable_body_still_carries_its_stderr`; **end to end** in `failure_matrix`, stderr asserted |
| brief §10.1's and §10.2's own examples, accepted verbatim (F-31, F-38) | protocol tier: `R-19-a-body-written-as-a-bare-string.json` and `R-18-brief-10-2-s-own-field-example.json`. **Not end to end.** Not in EX-1's list; recorded below |
| command not found | `transport.rs::a_command_that_does_not_exist_fails_to_spawn`; **end to end** `failure_matrix::a_command_that_cannot_be_spawned_reaches_the_caller_as_a_spawn_failure`. No fixture, as §9 says |

**Two items have no end-to-end case**, and neither is in EX-1's list, so neither
was built here: a backend that **writes nothing**, and the brief's **§10.1 /
§10.2 examples**. Both are tested — the first against the fake, the second as
fixtures — so nothing is untested; what is missing is the journey through a real
process. §9 introduces the list as backends "the integration tier needs", which
is a stronger claim than EX-1 makes. Three cases would close it, and the
instructed backend already emits arbitrary bodies, so the cost is the cases
themselves. **Audit or a plan amendment owns the scope decision.**

#### Verification record

| id | mode | result | evidence |
|---|---|---|---|
| EX-1 | — | **pass** — thirteen bodies, each its own case | `failure_matrix.rs`, VT-1's thirteen `#[tokio::test]`s plus a well-behaved control. Each names the fixture its body came from; the three schedule modes assert a discard rather than a failure, which is what the design says they are |
| EX-2 | — | **pass** — one `Host`, nineteen exchanges | `one_host_survives_every_misbehaving_backend_and_still_works`: a view, then all thirteen bodies and all four sentinels, then the answer to that view. Reuse is witnessed by state the failures did not touch, not by the last exchange working — break 2 |
| EX-3 | — | **pass** — through the real transport | the timeout, non-zero-exit and malformed-JSON cases each assert `next_check` is the instant the *previous* exchange set. Seen to fail: break 3 |
| EX-4 | — | **pass** — six modes, five new | spawn, timeout, non-zero exit, malformed stdout and output past the cap in `failure_matrix.rs`; the stale and unknown `view_id` at `round_trip.rs:220` and `:257`, which already assert the `StateError` a caller sees and carry an invocation witness. No third copy written |
| VT-1 | test | **pass**, and seen to fail | thirteen cases; red before the emitter script existed, then break 1 (a constant instruction index) and break 4 (a non-zero exit ignored) |
| VT-2 | test | **pass**, and seen to fail three times | break 1 (no sequencing: 0 refusals against 13), break 2 (a rebuilt `Host`: the schedule back at its seed), break 6 (a failed exchange closing the outstanding interaction — **caught here and nowhere else**, so R-34 across a real backend failure is this case's alone) |
| VT-3 | test | **pass**, and seen to fail | five cases; break 3 (the schedule re-resolved), break 4 (the exit status ignored), break 5 (the stderr dropped on a failure) |
| VA-1 | agent | **pass** | `just check` exits 0 on all seven commands, both feature columns — 35 unit, **52** integration, 15 protocol |
| VA-2 | agent | **pass** — walked, and two gaps recorded | the table above. Twenty-four items; twenty-two have an end-to-end case or belong to a tier that owns them, two have no end-to-end case and are named |

#### Log

- 2026-09-03 — sheet written. Entry criterion checked against PHASE-08's record
  and against the code; baseline gate green at `d7312aa`.
- 2026-09-03 — **EX-2's scope settled by the user**: the one-`Host` sequence
  spans the transport modes as well as the protocol ones. Recorded above.
- 2026-09-03 — **red, then green.** The thirteen VT-1 cases were written against
  a script that did not exist; fourteen red. `answers-as-instructed.sh` turned
  them green with no change to any assertion.
- 2026-09-03 — **refactor.** The bodies moved out of the assertions into named
  constants, because VT-2 sends the same thirteen and a second copy is a place
  for the two claims to drift. `choice` and `answer_first_option` moved from
  `round_trip.rs` into `harness.rs` when VT-2 needed to answer a view — the
  harness's own rule is that what two case files need lives there.
- 2026-09-03 — **six break-and-revert runs**, each against the criterion it
  falsifies. (1) the instruction index made constant: VT-2 alone, 0 refusals
  against 13 — VT-1 cannot see it, since each sends one instruction. (2) a
  `Host` rebuilt inside VT-2's loop: caught only after VT-2 acquired a state
  witness; see above. (3) `no_action` re-resolving the schedule: 7 cases here
  plus `host.rs`'s control — and it was this break that exposed the vacuous
  assertion. (4) a non-zero exit ignored: VT-3's case, VT-2's refusal count, and
  `transport.rs`. (5) the stderr dropped on a failure: VT-3's two stderr cases
  and `host.rs`. (6) a failed exchange closing the outstanding interaction: VT-2
  alone. All six reverted; `git diff src/` empty and the gate green.
- 2026-09-03 — **the gate.** `just check` exits 0 on all seven commands in both
  feature columns. 35 unit, 52 integration (20 of them this phase's), 15
  protocol. `src/` untouched: this phase landed
  `tests/backends/answers-as-instructed.sh`,
  `tests/integration/failure_matrix.rs`, three helpers and two moved ones in
  `harness.rs`, one line in `main.rs`, and the two helpers' removal from
  `round_trip.rs`.

## Harvest

**Fresh as of:** 2026-09-03 · plan accepted, **PHASE-01 through PHASE-08 and
PHASE-10 done — every code phase in the slice** · `just check` exits 0 in both
feature columns, and it is now **seven** commands — the seventh typechecks the
example, because `deno run` does not · stratum 1 is complete — the wire types,
the canonical types, normalization and schedule resolution, with a 70-file
fixture corpus across three directories — **the process transport is complete
with it**: every bound, both grandchild cases, disposal on its own channel, and
cancellation — **the host composes the two**, **the round trip runs end to end
against two real backends**, one TypeScript and one bash, and **every failure
mode the design names now reaches a caller as the `Outcome` it should**, with
R-45's one-`Host` reuse witnessed by state that seventeen failures did not
touch. **PHASE-09 is next and last**: `AGENTS.md`, the restatement sweep, and
reconciliation of the draft. Then audit

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
- **The wire types, normalization and the protocol corpus, from PHASE-04.**
  `src/semantics/protocol/wire.rs` and `normalize.rs`, `tests/protocol/normalize.rs`,
  and **54 fixtures** across `tests/protocol/fixtures/protocol/` (52) and
  `fixtures/protocol-text/` (2), plus two `pub mod` lines in
  `src/semantics/protocol/mod.rs`, three in `tests/protocol/main.rs`, five
  `pub(crate)`s in `runner.rs`, and the removal of four spent `expect(dead_code)`
  attributes in `canonical.rs`.
  `normalize_response(wire, now)` is the only path from wire to canonical;
  `now` is a parameter and nothing reads a clock. Every `ProtocolError` variant
  the wire can reach is named by a fixture, held by a coverage test that reads
  the corpus back — with two exemptions asserted **in the negative**:
  `ProtocolError::Schedule`, which is a discard rather than an error, and
  `BoundsError::NotFinite`, which JSON cannot express.
  9 protocol tests, 4 of them this phase's, and both new corpora run in **both**
  feature columns.
  **The durable artefact beyond the code is the corpus's `expect` shape** — an
  accepting fixture states the *whole* canonical value rendered back to JSON,
  not a probe into part of it, which is what makes `ls` over the directory a
  coverage report and the files themselves readable as protocol documentation.
  PHASE-05 and PHASE-10 inherit it, written up under PHASE-04's sheet.
- **The process transport and the integration tier, from PHASE-05.**
  `src/shell/error.rs` (`BackendError`, `CleanupFailure`),
  `src/shell/backend/transport.rs` (`Backend`, `Exchange`, `Captured`),
  `src/shell/backend/process.rs` (`ProcessBackend`, and §5.4's structure),
  two `pub mod` lines in `src/shell/mod.rs` and two in `backend/mod.rs`;
  `tests/integration/harness.rs` and `tests/integration/transport.rs` with the
  `main.rs` that declares them; **six backend scripts** under `tests/backends/`;
  and `tests/protocol/transport_shape.rs` with its `mod` line.
  7 integration tests and 5 protocol tests, the latter running in **both**
  feature columns because a source-text check needs no runtime.
  **Two durable artefacts beyond the code.** The **harness** is three functions
  and two request builders, and PHASE-06 and PHASE-08 inherit it whole:
  `backend(name)` builds the argv vector rooted at `CARGO_MANIFEST_DIR`,
  `transport(name, timeout)` points a `ProcessBackend` at it, and `describe` /
  `describe_cleanup` / `stderr` render a failure as a sentence, which is what
  keeps `clippy::use_debug` satisfied without an exception. The **backend
  scripts** are declarative — one behaviour each, named for it, each carrying
  the measurement that constrains its shape in a comment; the `exec` note in
  `hangs-past-the-timeout.sh` is the one PHASE-06 must read before writing the
  grandchild cases.

- **The bounds, disposal and the two grandchild cases, from PHASE-06.** Seven
  cases in `tests/integration/transport.rs` and a fourth check in
  `tests/protocol/transport_shape.rs`, over **five new backend scripts** plus
  `hangs-without-exec.sh`; `harness.rs` gained the process-inspection cluster —
  `children`, `children_running`, `alive`, `reported_pid`, `marker`, `clear`.
  One change to `process.rs`: `read_capped` and `body` take their stdout handle
  **by value**, so the bound closes the pipe rather than the exchange returning
  doing it. 15 integration tests and 15 protocol.
  **The durable artefact beyond the code is what makes each of these cases
  discriminating**, because three of them passed for the wrong reason first: a
  bound is not tested by asserting the outcome at the bound, since a bounded
  reader that *stops* produces the same outcome as one that *keeps draining*.
  Each case now carries a question the host cannot answer for it — a marker file
  written by a grandchild that outlives the kill, a stderr write after the bound
  whose success picks the response body, a `/proc` enumeration that settles.

- **Configuration, host state and the composition point, from PHASE-07.**
  `src/shell/{config,state,host}.rs`, `ConfigError` and `StateError` in
  `src/shell/error.rs`, and `tests/integration/{fake,host}.rs`. `config.rs` is
  the design's three values with a wire/canonical split and five rejections;
  `state.rs` is `Option<Outstanding>`, a concrete `resolved_check` and the
  `view_id` mint; `host.rs` is `Host<B>`, `Outcome`, `Presented`, `Failure`, and
  the four steps that follow an exchange — `from_slice`, `normalize_response`,
  schedule resolution, state update. 35 unit tests and 27 integration.
  **Two structural facts are worth carrying rather than rediscovering.** The
  failure rules are held by *signatures*, not by discipline: `Host::no_action`
  takes `&self` and `State::verify` takes `&self`, so neither a failed exchange
  nor a refused answer can move the schedule or close an interaction — break 3
  had to change a signature before it could break R-29. And `Host` is generic
  over `Backend` for a reason that paid immediately: twelve host-level cases run
  with no process at all, and the fake's **call count** is the only thing that
  can assert AC-8's "the backend is not contacted".

- **The round trip and the example backends, from PHASE-08.**
  `examples/typescript/backend.ts` (a whole backend in about eighty lines, no
  build step and no `node_modules`) with its `README.md`;
  `tests/backends/answers-a-round-trip.sh`, the first well-behaved bash backend
  that can carry a sequence; `tests/integration/round_trip.rs`, five cases; the
  harness's host-tier half — `example`, `config`, `host`, `quiet_event`,
  `prompting_event`, `logging_backend`, `invocations`, and the five describers
  lifted out of `host.rs`; and the `typecheck` recipe with §9's seventh command.
  32 integration tests.
  **Three things worth carrying.** `harness::host(command, timeout, now)` is the
  composition stratum 3 will perform — the transport is built *from* the
  config's own command, so a case cannot point the two at different backends —
  and it is what PHASE-10/EX-2 drives a sequence through. **A backend is
  parameterized through argv, not through the environment**: `cargo test` runs a
  target's cases in one process, so `std::env::set_var` is both unsafe and
  racy, while an extra argv element costs nothing and is exactly what R-36's
  argument vector is for. And the **example decides from the request** rather
  than owning state, because this transport spawns per exchange: a stateless
  backend can only vary its answer by reading what it was asked, and the example
  says in a comment where a real backend's state would live.

- **The failure matrix, from PHASE-10.**
  `tests/backends/answers-as-instructed.sh` — one backend that emits the body it
  was handed, or one of four sentinels naming a transport-level misbehaviour —
  and `tests/integration/failure_matrix.rs`, twenty cases: thirteen protocol
  modes each to its own variant and path, five transport modes as the `Outcome`
  a caller receives, one nineteen-exchange suite through a single `Host`, and a
  control. Plus three harness helpers (`scripted`, `protocol_error`,
  `only_discard`) and two lifted out of `round_trip.rs` (`choice`,
  `answer_first_option`). 52 integration tests. **Two things worth carrying.**
  **One parameterized backend beats N scripts** when a criterion needs a single
  command to misbehave differently on consecutive exchanges — which any
  one-`Host` requirement does, since a `Host` is built around one command. And
  **a suite that proves reuse has to read back state the failures did not
  touch**: an outstanding view and a moved schedule, both established before the
  failures and both checked after. Asserting only that the last exchange
  succeeded passes against a `Host` rebuilt every time.

### Learned

**Two instants that happen to be equal hide a whole rule — PHASE-10.** Every
case in the new tier started at `now` and asserted the schedule was still
`seeded_check()`, which is `now` plus the default poll — the exact value a host
that *re-resolved* the schedule on failure would produce. Thirteen cases,
including all three the plan pointed at R-29, could not tell the rule from its
negation, and the gate could not either. The fix is setup rather than assertion:
move the schedule off its seed before the case begins. The general form is worth
keeping — **an assertion against a value the defect would also produce is not an
assertion** — and it is why the fake tier's positive control at `host.rs:326`
exists. This tier had to rediscover it.

**A criterion can name a mechanism that cannot hold it, and only breaking it
says so — PHASE-08.** VT-2 asked for "the backend was not spawned" to be shown
by pointing the config at a command that would fail if it ran. It cannot: a host
that spawns and *then* refuses still returns the refusal, so the spawn failure
never reaches the caller, and the case stayed green under both breaks that
reorder the check. The claim is about something that did not happen, and only a
witness outside the host — here a log the backend appends to, one line per
invocation — can speak to it. This is the third time in this slice that a case
passed for the wrong reason and a break found it; the pattern is now explicit:
**an assertion about a non-event is vacuous unless something other than the code
under test records the event.**

**A gate command is worth more than an argv flag when the check is about the
source — PHASE-08.** `deno run` does not typecheck, and the two ways to restore
OQ-9's reason for TypeScript are not equivalent: `--check` in the backend's argv
pays ~70 ms on **every exchange for every user of the example**, while
`deno check` in `just check` pays it once per edit, where the author is. The
general form: put a check where the thing it checks is *written*, not where it
is *run*.

**A typed deserialization target fails differently from an untyped one, and the
difference decides fixtures — PHASE-07.** `serde_json::from_slice::<Value>` and
`from_slice::<WireResponse>` disagree about `{"a":"\xff"}`: the first reports an
invalid code point, the second returns `Ok`, because every field is
`#[serde(default)]` and **serde never decodes a value it skips**. A framing
fixture must therefore be a well-shaped document plus exactly the defect it
names — anything else is caught by a different rule and asserts a different
claim. The measurement that matters is against the type that ships.

**A lint expectation is a self-clearing record, and that is what makes it
better than a comment — PHASE-07.** `#[expect(dead_code, reason = …)]` on a field
the design names and nothing reads holds the design's shape *and* fails the
build the moment the situation changes: adding a read produces `this lint
expectation is unfulfilled`. Verified by adding one. A stale `#[allow]` is
invisible; a stale `#[expect]` is a warning.


Candidates for `docs/memory/` at close — all listed under **Established
empirically** above, plus:

- Slint 1.17.1 builds clean in this dev shell; `slint-build` in `build.rs`,
  `slint::include_modules!()`, and a missing `std-widgets.slint` import fails in
  the *build script* rather than rustc (`research.md`).
- tokio at 14 unique deps versus the smol family's 31, measured — the opposite of
  the intuitive reading of "smallest reasonable" (`research.md`).

**From PHASE-05, all measured:**

- **`bash -c` execs the last command of the string it is given; `bash script.sh`
  forks it.** So the same two lines are two different process trees. A fixture
  ending in `sleep 30` is *the child* under `-c` and *a grandchild* as a script
  file — where it outlives the kill, holds both pipes open, turns a plain hang
  into a cleanup timeout, and leaves a 30-second orphan behind every run.
  `exec sleep 30` in a script file restores the `-c` behaviour. This is why the
  transport probe and the integration harness can disagree while the transport
  is correct, and it is the first thing to check when a backend fixture behaves
  unlike the probe case it was copied from.
- **A write smaller than the pipe buffer (64 KiB on Linux) succeeds even when
  the reader has already exited.** The kernel accepts it into the buffer, which
  outlives the process. `EPIPE` requires the write to still be *in flight* when
  the read end closes, so a broken pipe on stdin is reachable deterministically
  only with a payload past the buffer — 20/20 either way, both directions.
  Rust ignores `SIGPIPE` at startup, so the write returns `Err` rather than
  killing the process.
- **`clippy::use_debug` is `deny` crate-wide and is *not* one of the four keys
  `clippy.toml` carves out for tests.** So `{:?}` fails the gate in test code
  too. The convention this repo already had — `boundary.rs`'s `Breach` — is to
  give the diagnostic a `Display` and format with `{}`; `Duration` renders via
  `as_millis()`, and `str::escape_debug()` covers the one case that wants
  quoting. Reach for a helper, not an `#[expect]`.
- **`AsyncReadExt::read_buf` into a `Vec` removes every slicing site a capped
  reader would otherwise need.** The obvious loop — read into `[u8; N]`, then
  `&buf[..n]` — is four `indexing_slicing` errors under this lint table.
  Appending straight into the output (`out.reserve(chunk); read_buf(&mut out)`)
  and comparing `out.len()` against the bound, or reading into a reused chunk
  and `truncate(room)`ing it, needs no indexing and no arithmetic beyond a
  `saturating_sub`.
- **A textual "no `?` in this region" check cannot tell a nested block's `?`
  from a function-level one.** §5.4's sketch inlines the exchange body as an
  async block, which puts three harmless `?`s inside the region F-41's rule is
  about; the check would have failed against the design's own structure.
  Extracting the block to an `async fn` makes the region honest and the check
  exact. The general form: when a source-text check and a structure disagree,
  the structure is usually the cheaper thing to move — provided every claim it
  makes survives the move.

**From PHASE-06, all measured:**

- **A bounded reader that stops also closes the pipe, so "truncated and it
  succeeded" does not distinguish draining from stopping.** The writer takes
  `EPIPE` and carries on instead of blocking, and the answer still arrives. The
  design predicts a *deadlock* from collapsing the two readers; that prediction
  holds only for a reader that stops **without** dropping its handle, which is
  not the shape anyone writes. A truncation test therefore needs the backend to
  attempt one more write after the bound and to say, in the body the host does
  read, whether it succeeded.
- **What must exceed the pipe buffer is the remainder after the bound, not the
  flood.** At 300 KB against a 256 KiB bound the leftover is ~37 KB, fits the
  64 KiB buffer, and nothing blocks. 400 KB is the smallest honest fixture.
- **The `exec` finding runs in both directions, and the second one is quieter.**
  A backend that must be the child needs `exec`; a backend that must leave a
  grandchild must not have it. Two of `tests/backends/`'s scripts `exec` — and
  so **lose their script name from `/proc/<pid>/cmdline`**, because the argv is
  the exec'd program's. Any check that identifies our processes by command line
  is therefore blind to exactly the leaks that matter. Found by breaking the
  kill and watching the leak detector pass.
- **`cargo test` runs a target's cases as threads of one process**, so any global
  assertion about the process — children, task counts, open descriptors — sees
  every concurrently running case. Two mechanisms make such a claim assertable:
  **settling** (poll until quiet, so a concurrent case's transient child clears
  and a leak does not) and **owning the thing measured** (build the runtime the
  task count is about).
- **A grandchild whose parent has died reparents to init**, so it is not a child
  of the test process. That is what lets a grandchild fixture and a no-orphans
  assertion coexist, and it is why R-48's claim is about *children*.
- **`num_alive_tasks()` needs no `tokio_unstable` and no `rt-multi-thread`** —
  it works on a current-thread runtime under this crate's exact feature list.
  Its positive control must wait for the future to have done real work: the
  spawned task registers at `spawn`, so a count of 1 proves nothing about the
  exchange. Waiting until the **child process exists** does.
- **`clippy::let_underscore_must_use` is denied and is not carved out for
  tests.** Every `let _ = fallible()` in a test is an error. Two of the three
  sites became a named helper whose doc comment says why the failure is
  uninteresting; the third became an assertion worth making.
- **`/proc` beats shelling out for process questions in tests**, and not only for
  the missing tool: `kill -0` spawns a child of its own on every call, which any
  enumeration of children then has to watch go past.

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

**From PHASE-04, all measured here rather than assumed:**

- **`#[serde(flatten)]` at depth does not disturb a `deserialize_with` above
  it.** The whole wire shape rests on this: `WireResponse.view` is
  `Option<Option<WireView>>` with a presence-preserving deserializer, and both
  `WireView` and `WireField` flatten beneath it. All three states stay distinct
  with a hint two levels down. Re-measured rather than cited, as the design's §6
  says it was.
- **Serde binds a declared optional *before* the discriminant beside it is
  read.** `{"kind":"text","min":1}` yields `min: Some(1.0)` with an **empty**
  `hints` map. There is no encoding in which a misplaced modelled key falls
  through to a flattened catch-all, so normalization must reject it or lose it —
  F-45's premise, and it holds.
- **A misspelled *required* key still fails after flattening.** `labell` gives
  ``missing field `label` ``; so do `kindd` and `idd`. D37's stated cost really
  is bounded to the optional keys.
- **`serde_json::Number` equality is spelling-sensitive.** Parsed `10` does not
  equal parsed `10.0`. Any fixture comparing an `f64` by JSON equality has to
  write the float spelling.
- **`from_str` and `from_value` agree on which failure occurs, and differ only
  in that `from_value` errors carry no line and column.** `1e400` is the case
  that looks like a difference and is not: it fails when the `Value` itself is
  parsed, before any struct is involved.
- **`clippy::option_option` (pedantic) forbids the shape §5.2 mandates**, and
  its own suggested alternative is a custom enum — a design change to satisfy a
  style lint. Taken as an `#[expect(…, reason = …)]` at the field. Second use of
  D53's reason-carrying hatch; expect a third.
- **`clippy::missing_panics_doc` is live, for the same reason
  `missing_errors_doc` is.** The manifest comments the doc lints out and says
  they are paused; `pedantic = "deny"` re-enables both. An `unwrap()` in host
  code therefore fails the gate *twice* — once for the unwrap and once for the
  missing `# Panics` section.
- **`deny_unknown_fields` and `flatten` do not compose**, so I10's permissiveness
  cannot be broken uniformly to test it. More useful: splitting a tagged object
  into a discriminant struct and a payload struct — the encoding §6 offers, and
  the one that makes a named error possible at all — **depends** on
  permissiveness, because each struct sees the other's key as unknown.
- **A corpus that asserts the whole canonical value discharges "unknown fields
  are ignored" for free.** No second mechanism, no probe language: a key that
  survived normalization appears in the rendering and breaks the case.
- **Where no ordinary red exists, write the naive expectation first.** Two
  groups had no failing state reachable from this crate's code — the `NaN` and
  `1e400` literals, whose claim is about `serde_json`'s parser, and the `null`
  cases serde elides structurally. Asserting the *wrong* reading first and
  watching it be refuted is the available red, and it puts the design's own
  argument in the run log.
- **The AC-11 scan can be lived with by word choice.** "Place", "position" and
  "level" carry everything "site" would have, so the predicted recurrence did
  not happen.

### Open

**Raised by PHASE-10, 2026-09-03 — both are scope decisions for audit or a plan
amendment, neither is a phase repair.**

- **Two items of `design.md` §9's misbehaving-backend list have no end-to-end
  case.** A backend that **writes nothing** is asserted against the fake
  (`host.rs:167`, empty stdout as an unexpected EOF) and the brief's **§10.1 and
  §10.2 examples** are asserted as protocol fixtures — so nothing is untested,
  but neither travels through a real process. Neither is in PHASE-10/EX-1's
  list, which is why neither was built. §9 introduces the list as backends "the
  integration tier needs", which claims more than EX-1 does. Three cases would
  close it and the instructed backend already emits arbitrary bodies.
- **R-34 across a *backend* failure is asserted in exactly one place.** Break 6
  — a failed exchange closing the outstanding interaction — is caught only by
  PHASE-10/VT-2, and there only incidentally, because VT-2 needs an outstanding
  view as its reuse witness. `host.rs` asserts R-34 for a *refusal*, which is a
  different path. If VT-2 is ever simplified, the rule loses its only test.

**Raised by PHASE-08, 2026-09-03 — one belongs to PHASE-09, two to audit.**

- **`slice-001.md`'s OQ-9 answer is false as written.** It says deno
  "typechecks rather than stripping types". It does not, and the correction is
  PHASE-09's restatement sweep — PHASE-08's Surfaces do not reach that file.
  `design.md` §9 and `plan.md` are already corrected, and the *decision* OQ-9
  records (deno, `-A`) is unaffected: only its stated reason was wrong.
- **The example's README config is not exercised.** It uses a relative path,
  `["deno", "run", "-A", "./examples/typescript/backend.ts"]`, which is right
  for a user's own config and is why no test uses it — the suite roots every
  path at `CARGO_MANIFEST_DIR`. So the one config a reader will copy is the one
  nothing runs. Audit should decide whether that is worth a case that sets a
  working directory, or whether `config.rs`'s existing parse cases cover it.
- **`draft-spec.md` R-45's verification row still describes PHASE-08's original
  scope** — "the whole integration tier runs every misbehaving backend against
  one host instance". That is PHASE-10/EX-2 after the F-6 split. The row is
  accurate about the requirement and stale about where it is proven; PHASE-09/EX-3
  owns pointing it at the test that exists.

**Raised by PHASE-07, 2026-09-03 — six are audit or reconciliation business,
none is a phase repair.** The phase sheet's *Noticed, not this phase's* section
states each in full; in short:

- **Invalid UTF-8 is rejected only where serde reads it.** A skipped value's
  bytes are never decoded, so `{"a":"\xff"}` parses. `design.md:1052`'s argument
  for `Vec<u8>` stands and is vindicated by the case that matters — a *read*
  value, where lossy conversion would have substituted U+FFFD silently — but
  R-38's verification row claims more than the implementation does.
- **`design.md` §5.2 lists five error types and there are six.** `ConfigError`
  is new by user decision. `draft-spec.md`'s R-44 has the same omission.
  Reconciliation: the code is right, the documents are stale.
- **`design.md:1167`'s `issued_at` is read by nothing** and is kept under a
  self-clearing `#[expect]`. Audit gives it a reader or removes it.
- **A config file's unknown keys and section names are ignored silently.**
  `deny_unknown_fields` would close it; no criterion asks for it, so it was
  recorded rather than built.
- **Half of PHASE-05's synthesized-`Spawn` item closes structurally**, since
  `command = []` is now rejected at load. It stays reachable for a caller that
  builds a `ProcessBackend` without a `Config`.
- **R-30's verification row has no owner in the plan.** The Coverage map is by
  AC and this is a spec row; the source check it asks for is unwritten, and its
  home is PHASE-01's surface.

**Raised by PHASE-06, 2026-09-03 — three are audit business, none is a phase
repair.**

- **`design.md:1729`'s row — "backend wedged so `wait` cannot return" — names a
  mechanism no test can arrange.** `dispose` is `start_kill` then `wait`, and
  only uninterruptible kernel sleep defers `SIGKILL`. Every case that does make
  the cleanup budget elapse stalls on the **drain**. VT-4 was reworded to the
  reachable claim by user decision 2026-09-03 and is discharged; the design's row
  stands as written, and audit should decide whether it says something the
  implementation cannot honour or simply describes a case the tier cannot build.
- **The stdout cap's second effect is now true and still not the one the design
  describes.** `design.md:1528` says the cap kills the backend "by itself" —
  the reader drops the handle, the flooder takes `SIGPIPE`, and `wait()` returns
  with a signal status. With the ownership repair the pipe does close at the
  bound, but disposal still `start_kill`s before observing anything, so what the
  host actually sees is its own kill. Both mechanisms now agree on the outcome
  and the sentence describes the one that does not fire. Reconciliation business,
  not a defect.
- **A failing flood run leaves a stale marker in the temp directory.** The
  backend writes it a millisecond or two after the case has cleared it, so
  `/tmp/goad-broken-pipe-<pid>` survives — one per failing run, never on a green
  one. Cheap to live with; audit may prefer the case to clear on the way in only,
  which it already does, and to say so rather than clearing twice.

**Raised by PHASE-05, 2026-09-02 — three are audit business, none is a phase
repair.**

- **`BackendError::PipeMissing` has no test anywhere in the slice.** VT-3 asks
  for one case per variant "this phase can reach", and this one is not reachable
  from outside: it fires when a stdio handle the host itself asked for is absent
  after a successful spawn (F-35), which no backend can arrange. `cleanup_only`
  exists solely to serve it and is therefore also untested. Audit should decide
  whether that is acceptable — the alternatives are a unit test that fabricates
  the state, or an argument on the page that the variant is a guard rather than
  a path.
- **Two synthesized failures on paths configuration will close.** An empty
  `command` returns `Spawn(io::ErrorKind::InvalidInput)`, and a `Request` that
  will not serialize returns `Protocol(ProtocolError::Json(_))`. Both are
  structurally unreachable — PHASE-07 rejects `command = []` at load, and a
  host-authored `Request` serializes infallibly — but the lint table forbids
  `unwrap`, so something must be returned. The gap-3 decision said nothing in
  this phase raises `Protocol`; the reading taken here is that the decision is
  about parsing what a *backend* wrote, which this transport never does. Audit
  should confirm that reading, or make one or both states unrepresentable.
- **VT-5's `?`-region check is a tripwire, not a proof.** The dangerous form —
  a `?` that returns from `exchange` and skips disposal — cannot be written
  today at all, because `Exchange` is not a `Result`. What the check actually
  catches is a `?` in a code position in the region, which is why break 3 had
  to use a closure. Latitude item 6 predicted this is the check that can produce
  a false positive as `process.rs` grows; it is worth re-reading if PHASE-06's
  changes make it fire on something harmless.

**Raised by PHASE-04, 2026-09-02 — all four are audit business, none is a phase
repair.**

- **R-51 and R-53 meet, and the design does not say what happens.** R-51 states
  that an explicit `null` means what omission means *for every modelled field*;
  R-53 rejects a `fields` key on a `choice` field's alternative, and R-50 rejects
  a modelled key on a kind that does not admit it. So what does
  `{"id": "red", "label": "Red", "fields": null}` mean? PHASE-04 applied R-51 —
  the sender asserted nothing, so nothing is lost by reading it as omission, and
  a serializer emitting `null` for an absent optional is doing nothing wrong,
  which is R-51's own argument. **Shipped as
  `R-51-a-nulled-fields-key-on-an-alternative.json` and
  `R-51-a-nulled-modelled-key-on-a-field.json`**, both stating the reading in
  their descriptions. If audit prefers strictness, those two fixtures are what
  change. Note the second case is structural rather than chosen: serde maps
  `"min": null` to `None` before normalization sees it, so rejecting it would
  need a wire-type change, not a normalization one.
- **VT-4's third clause is false at one of its six levels.** It asks each
  unmodelled-key fixture to assert "the field is absent from the canonical
  value". On a *field object* D37 makes every unnamed key a hint, so it survives
  into `Field.hints` deliberately. The sheet's own corpus inventory states only
  the two clauses that hold at all six — acceptance and an empty discard list —
  and those are what the fixtures assert, with the field level asserting the
  stronger D37 claim instead. Same shape as `design.md:704`'s blanket comment
  over the three checked collections, which PHASE-02 raised.
- **VT-2 names one documented exception and there are two.**
  `BoundsError::NotFinite` is the one it names. The second is
  `ProtocolError::Schedule`, which by the design's own P2 rule never arrives as
  an `Err` at all — `design.md` §5.2 says so under AC-6. Both are asserted **in
  the negative** by the coverage test rather than skipped, so neither can quietly
  become reachable without the corpus noticing.
- **`design.md` §5.2 names `WireView` and defines it only in §6**, alongside
  `WireOpt` (F-55) and `cleanup_only` (F-56). PHASE-04 also added two wire types
  §5 names nowhere — `WireAlternative`, and the split of a content block into
  `WireContent` and `WireContentValue`. All are §6 latitude being exercised, not
  holes; they are listed because §9's "every type named in §5 is defined in §5"
  sweep would otherwise find `WireView` a third time and stop there.

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

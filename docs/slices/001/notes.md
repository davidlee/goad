# Notes — Slice 001

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| design | **accepted** — review closed at round 5, 16 repairs unverified | 2026-08-26 |
| plan | **drafted, not accepted** — nine phases in `plan.md`; two design gaps found and closed; adversarial review agreed, not yet run | 2026-08-26 |
| PHASE-01…09 | not started; phase sheets are written one at a time, immediately before execution | — |

**No code exists.** No `Cargo.toml`, `src/` and `tests/` empty. Every file this
slice touches is a new file.

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

`plan.md` is now written — nine phases, coverage complete, not yet accepted.
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
3. ~~**Write `plan.md`.**~~ Done 2026-08-26 — nine phases. Two design gaps
   surfaced while writing it, A-P1 and A-P2, and are with the user.
4. **Offer the user adversarial review of the plan** — `review-plan.md`,
   decisions in `plan-log.md`. Their call, not yours. Given how this design's
   review went, say what it would cost and what it has historically caught.
5. **Ask the user to accept the plan.** Then phase sheets, one at a time,
   immediately before each phase — never up front.

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

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Nothing here yet: the slice has no accepted plan. -->

## Harvest

**Fresh as of:** 2026-08-26 · design accepted, review closed, planning not
started · working tree, uncommitted (7 files)

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
- `plan.md` — **empty template.** This is the next artefact.

### Learned

Candidates for `docs/memory/` at close — all listed under **Established
empirically** above, plus:

- Slint 1.17.1 builds clean in this dev shell; `slint-build` in `build.rs`,
  `slint::include_modules!()`, and a missing `std-widgets.slint` import fails in
  the *build script* rather than rustc (`research.md`).
- tokio at 14 unique deps versus the smol family's 31, measured — the opposite of
  the intuitive reading of "smallest reasonable" (`research.md`).

### Open

- **Sixteen repairs unverified** — F-48…F-58, never re-examined, and F-59…F-63,
  round 5's own. Accepted knowingly; see the ledger's Synthesis. Expect two to
  four residual defects, most likely in §5.4.
- `plan.md` **drafted** — nine phases, coverage map complete. Not accepted;
  adversarial review of it not yet offered. No phase sheets.
- **Two design gaps surfaced during planning** and both are closed — the TOML
  parser by user decision, the suspected build-gate defect by measurement that
  withdrew it. See *Found while planning* above and `plan-log.md`. No canon or
  design text changes; `design.md` §5.1 and §3 owe a `toml` line at audit.
- **Plan review agreed** — one round, fresh reviewer, ledger `review-plan.md`.
  Not yet run.
- CD-1 and CD-2 unapplied, awaiting endorsement at audit.
- Whether to promote `transport-probe.local.rs` to a tracked spike — a user call.
- `flake.nix` `devToolPkgs` still lacks deno. **Raised with the user 2026-08-26**
  and assigned to PHASE-01 (EX-5, VH-1) — it was already in the slice's Scope per
  OQ-9, so it needed a phase, not a decision. The dev shell must be reloaded
  after the change or a running session will not see it.

# Notes — Slice 002

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| design | **accepted** — five review rounds, F-1…F-40, all terminal. Discharged by the replacement criterion in §5 of the Handover below: *every claim is either built, or its failure mode is loud at first compile* | 2026-09-05 |
| plan | **accepted 2026-09-05, revised after review** — ten phases in `plan.md` (PHASE-07 split at its own seam, so the execution order is 01…07, 10, 08, 09); twelve planning decisions in `plan-log.md`; seven findings against the design (DF-1…DF-7). One adversarial round has now run — `review-plan.md`, F-1…F-33, nine blockers, all terminal — and it repaired §5.1's artifact map against the real tree rather than reading it a sixth time | 2026-09-05 |
| PHASE-01 — the workspace split | **done** — gate green, 130 paths relocated, four map defects found and repaired (`review-plan.md` F-34…F-37) | 2026-09-05 |
| PHASE-02 — the workspace invariant checks | todo | 2026-09-05 |
| PHASE-03 — `crates/goad`, Slint, the markup and the element tree | todo | 2026-09-05 |
| PHASE-04 — the mapper and the tray rasteriser | todo | 2026-09-05 |
| PHASE-05 — the diagnostic surface and the reception seam | todo | 2026-09-05 |
| PHASE-06 — the controller, the fold, and the failure case table | todo | 2026-09-05 |
| PHASE-07 — the glass, the wiring, and back-pressure | todo | 2026-09-05 |
| PHASE-10 — `serve`, and the stop that drops the exchange | todo — **executes between PHASE-07 and PHASE-08**; ids are immutable, so the sequence is non-monotonic (PL-10) | 2026-09-05 |
| PHASE-08 — startup, the entry point, and the event-loop tier | todo | 2026-09-05 |
| PHASE-09 — the drafts, the restatement sweep, and the clean-clone gate | todo | 2026-09-05 |
| audit | todo — CD-1…CD-7 and `draft-policy.md` are promoted here, with explicit endorsement, and nowhere earlier (`docs/AGENTS.md:38`) | 2026-09-05 |

**Execution order is 01, 02, 03, 04, 05, 06, 07, 10, 08, 09.** No two phases have disjoint surfaces:
every renderer phase touches `crates/goad/src/lib.rs` and
`crates/goad/tests/renderer/main.rs`. One agent, one phase, one session.

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — The workspace split: three members, the relocation, and the six-command gate

**Objective:** the single crate is a workspace of three members, every file
§5.1's artifact map moves has moved, the gate is §5.6's six commands, and the map
has been walked in both directions against what actually happened.

**Anchors (EN-5), bound before anything moved**

| | |
|---|---|
| `<pre-split>` | `54a76aaa24ad32a04f12daabc7036445171a17ea` |
| `<slice base>` | `a6ae61764b80f843b53b642e598ea71b69d43a94` (`git merge-base main HEAD`; PHASE-09/VA-3 reads it from here) |
| commit protocol | **one commit for the whole phase**, made after the gate is green, so `<pre-split>` cannot drift and EX-4's backward walk covers the whole relocation in one diff |

**Entry criteria, run rather than read**

| # | evidence |
|---|---|
| EN-1 | `git rev-parse --abbrev-ref HEAD` → `slice-002`; `git status --porcelain` → exactly ` M flake.lock` |
| EN-2 | `cargo metadata --no-deps --format-version 1` → one package, `goad` |
| EN-3 | `just check` exit 0. Warm wall-clock, three consecutive runs, no source change between: **1.828 s / 1.809 s / 1.808 s → median 1.808 s**. This is the pre-split baseline A-4 is measured against at PHASE-03/EX-1 |
| EN-4 | this sheet |
| EN-5 | the table above |

Also captured before anything moved, for VT-1: `cargo test -- --list` (116 test
lines) and `cargo test --no-default-features -- --list` (41), module prefixes
stripped and unioned → **116 distinct test names**.

**Reading list**

- `docs/AGENTS.md:107-123` — phase plan and execute. `:36` — the drafts are the
  slice's working authority; `:38` — promotion is at audit.
- `design.md` §5.1 whole (`:142-467`), and *The artifact map* (`:291-467`) in
  particular: the split's source→destination table, the member table, the
  test-target table, the `#[path]` literal.
- `design.md` §5.5 (`:2700-2928`) — the STOP table, transcribed verbatim below.
- `design.md` §5.6 (`:2929-3160`) — the six-command gate, the three enforcement
  residues, `goad-boundary`'s public API (PHASE-02's shape).
- `design.md` §12.8 (`:4473-4515`) — where the driving helpers live, and how a
  target is declared. Its two lists do **not** partition `harness.rs` (DF-5), so
  PHASE-01/EX-7's table is the specification and §12.8 is its source.
- `plan.md` PHASE-01 (`:315-705`) — EN-1…EN-5, EX-1…EX-14, VT-1…VT-2, VA-1…VA-4,
  PS-1, PS-2.
- `plan-log.md` PL-1 (three members, not four), PL-2 (`goad-boundary`'s rewrite
  is PHASE-02), PL-4 (the `driving.rs` cut is made here and re-settled at
  PHASE-06), PL-9 (`boundary.rs`'s PHASE-01 destination), PL-11 (the member
  manifest skeleton), PL-12 (three `Scan`s, one `#[test]`).
- `research.md`'s dry-run section — read as **what the dry run did**, never as
  what this tree contains. Its 111 / 91 / 77 / 14 were measured on a tree this
  branch has never had; §5.1 is repaired and is the authority.
- Prior art in the tree: `tests/protocol/main.rs:5-11` (why `#[cfg(test)]` sits
  on a `tests/` target's module declarations), `tests/protocol/boundary.rs:69-91`
  (`CARGO_MANIFEST_DIR`, and the vacuity guard), `clippy.toml:20-23` (the four
  allow-in-tests keys that make `panic!` legal in a test and illegal in a
  library).

**Assumptions**

- A-map — §5.1's artifact map is correct about this tree. It is the least-audited
  passage in the design and **executing this phase is its audit**. A wrong row is
  the most valuable thing this phase can return; it is corrected, recorded in the
  ledger and in `design-log.md`, and named in the result — never routed around.
- A-path — `#[path]` on a `mod` in `crates/<member>/tests/<target>/main.rs`
  resolves relative to that directory, so `../../../../` reaches the repository
  root. Verified on paper by the plan; verified by compilation here.
- A-4 — untouched by this phase. No `slint` in the graph, so ADR-002's T3 cannot
  be measured here; the baseline above is what PHASE-03 measures against.

**STOP conditions — `design.md` §5.5, verbatim**

| # | condition | why it is not a phase's to decide |
|---|---|---|
| S-1 | a **third** distinct lint needs an `#[expect]` outside the generated-code quarantine | the table is wrong for this stratum (A-2). Two remain unspent |
| S-2 | a lint suppression outside the quarantine module, a lint the workspace table does not set, or a `[lints]` table in a member manifest | D8 is wrong for generated code (A-1) |
| S-3 | `CompilerConfiguration::with_debug_info` is gone, or item 6's guard test fails | every element-tree assertion rests on it (A-3) |
| S-4 | median warm `just check` **> 300 s** | ADR-002 T3 has fired hard (A-4) |
| S-5 | item 14a measures shutdown at **> 250 ms** against a 2 s timeout | shutdown is awaiting the exchange, which AC-12 forbids |
| S-6 | a file has to move that §5.1's artifact map does not name, or a content change beyond that table's "change permitted" column | it is a redesign, and AC-2 says so (R4) |
| S-7 | a `.slint` compile error the markup in §5.2 did not have | A-7's evidence no longer covers the markup |
| S-8 | any dependency beyond `slint`, `slint-build`, the Slint testing dev-dependency and the named font package | `CLAUDE.md` requires a dependency be asked about |

Plus this phase's own two, from `plan.md`:

- **PS-1** — a `+` or `-` line in EX-13's hunks, in a production source under
  `crates/goad-semantics/src` or `crates/goad-shell/src`, falls outside EX-5a's
  vocabulary. The trigger is EX-13's output, not the agent's judgement.
- **PS-2** — `just check` still red after five distinct repair attempts, or 45
  minutes from the first attempt, whichever comes first; or, at any attempt, the
  failure names a file the map marks change-forbidden. Paste the failing output
  **before** deciding anything.

And the four hard stops standing over the whole session: no canon written or
promoted; no dependency beyond the endorsed set; the gate is never weakened; no
work outside the declared surfaces.

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [x] EN-1…EN-5 run, anchors bound, baseline and pre-split test-name set captured
- [x] this sheet, before anything moves
- [x] T1 — `crates/goad-semantics/`: `git mv` `src/semantics/**`, the three
      protocol test files, the 88 fixtures to `tests/fixtures/**`
- [x] T2 — `crates/goad-shell/`: `git mv` `src/shell/**`, the six
      `tests/integration/*.rs`, `transport_shape.rs` into the new `shape` target
- [x] T3 — `tests/support/driving.rs`: EX-7's cut, item by item
- [x] T4 — `crates/goad-boundary/`: PL-9's layout, EX-5c's changes
- [x] T5 — the root `Cargo.toml` and three member manifests to EX-8a's skeleton
- [x] T6 — the `justfile`: six commands, header comment repointed
- [x] T7 — green the gate; `cargo fmt --all` last, then re-run the whole gate
- [x] T8 — EX-3/EX-4/EX-5/EX-11/EX-13 walks; EX-10 and EX-14 controls
- [x] T9 — bookkeeping: sheet, ledger, `design-log.md`, `plan-log.md`; one commit

**Exit criteria, discharged**

| # | evidence |
|---|---|
| EX-1 | `cargo metadata --no-deps` → `['goad-boundary', 'goad-semantics', 'goad-shell']`. Root has `[workspace]` at line 1 and **zero** `[package]` tables. `members` is a literal three-entry list; `grep -c '\*'` over it → 0 |
| EX-2 | `just check` exit **0**. `just -n check` prints §5.6's six commands in §5.6's order — pasted at VA-3 |
| EX-3 | the forward walk, four predicates, below |
| EX-4 | the backward walk, below. **No path outside §5.1 plus the exclusion set `docs/slices/002/**`.** S-6's path half did not fire |
| EX-5 | amended to a containment by PL-13 / F-37: all 88 files under `tests/fixtures/` are `R100`; the `R100` set is **92**, and the four extras are named. `grep -c 'tests/backends'` over the walk → **0** |
| EX-5a | the vocabulary, plus F-34's seventh entry. Checked mechanically at EX-13 |
| EX-5b | the named non-identical changes, below — the plan's eight rows, plus F-35's seventh integration change and F-34's nine comment lines |
| EX-5c | five things, not four (F-36). All five below |
| EX-6 | four `[[test]]` targets: `goad-semantics`/`protocol`, `goad-shell`/`integration`, `goad-shell`/`shape`, `goad-boundary`/`checks`. `checks` declares `{direction, vocabulary}` per PL-1's deferral. `main.rs` files carry `#[cfg(test)] mod` declarations and nothing else |
| EX-7 | the §12.8 cut, item by item, below. Nothing in `harness.rs` unaccounted for |
| EX-8 | the lint **lines and levels** diff clean against `git show 54a76aa:Cargo.toml` — 69 non-comment lines, **identical**. The carve-out comment is rewritten; diff below |
| EX-8a | the skeleton transcribed key for key. Five keys inherited, `publish = false` stated once, no member carries `description`/`keywords`/`categories`/`readme` |
| EX-9 | `grep -rn 'feature *= *"shell"'` → nothing. `grep -n optional Cargo.toml crates/*/Cargo.toml` → nothing. `grep -rn 'required-features'` → nothing. `grep -rn '\[features\]'` → nothing |
| EX-10 | two break-and-revert `E0433` controls, pasted below |
| EX-11 | the walk pasted below, with a hand-written line per non-`R100` file |
| EX-12 | the `justfile` header cites `draft-policy.md` and §5.6, and no longer cites `docs/slices/001/design.md` §9. `CLAUDE.md` untouched — `git diff 54a76aa -- CLAUDE.md docs/specs docs/policy docs/adr` is **empty** |
| EX-13 | full hunks, below, every `+`/`-` line matched against EX-5a |
| EX-14 | three demonstrations, each with a live negative control, below |

**Verification**

| # | evidence |
|---|---|
| VT-1 | `cargo test --workspace -- --list` after, diffed against `cargo test -- --list` + `cargo test --no-default-features -- --list` before, module prefixes stripped: **identical, 116 names.** The first run renamed one test function and the diff caught it; the name was restored rather than absorbed, which is what PL-12 settled the shape for |
| VT-2 | `checks` runs the vocabulary scan over all three members' `src/` and carries **both** vacuity controls, each failing for its own reason — EX-14(c) |
| VA-1 | `just check` exit 0. Warm wall-clock, three runs: **1.789 / 1.805 / 1.797 s → median 1.797 s**, against the EN-3 pre-split baseline of **1.808 s**. No `slint` in the graph, so this measures the split alone: **the split cost nothing.** A-4 and ADR-002 T3 are PHASE-03's |
| VA-2 | both walks and EX-13's hunks, below |
| VA-3 | `just -n check` pasted below beside §5.6's block — compared as a **command sequence** |
| VA-4 | `cargo tree -p goad-semantics --edges normal,build,dev`: `jiff`, `serde`, `serde_json` and their transitives. **0 `tokio` nodes, 0 `toml` nodes** |

**VA-3 — `just -n check`, beside §5.6**

```
cargo build --workspace
cargo test --workspace
cargo test -p goad-semantics
deno check examples/typescript/backend.ts
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

Six commands, in §5.6's order, with §5.6's arguments. Recipe names unchanged —
`build`, `test`, `test-stratum1`, `typecheck`, `lint`, `fmt-check` — and `lint`
has one line.

**EX-8 — the carve-out comment, diffed**

```diff
-# Dead/unused things should not survive agent iterations — with two carve-outs.
+# Dead/unused things should not survive agent iterations — with one carve-out.
 #
-# `dead_code` and `unreachable_pub` are transient *by construction* here, and
-# both cases are structural rather than sloppiness:
-#   - the `--no-default-features` column drops `shell`, so every stratum 1 item
-#     whose only caller lives in stratum 2 is dead in that column. That is
-#     ADR-001's feature gate working, not a defect.
-#   - a phased plan lands a type one phase before its caller.
-# At `deny` the only escape is peppering `#[cfg_attr(…, expect(dead_code, …))]`
-# across `semantics/`. At `warn` the noise stays visible and the build still
-# goes. `expect(dead_code, reason = …)` remains available for the cases worth
-# *recording*; it self-clears via `unfulfilled_lint_expectations`.
+# `dead_code` and `unreachable_pub` are transient *by construction* here: a
+# phased plan lands a type one phase before its caller. At `deny` the only
+# escape is peppering `#[cfg_attr(…, expect(dead_code, …))]` across a member.
+# At `warn` the noise stays visible and the build still goes.
+# `expect(dead_code, reason = …)` remains available for the cases worth
+# *recording*; it self-clears via `unfulfilled_lint_expectations`.
 #
 # Both are left at their default `warn` explicitly rather than by omission, so
-# that the carve-out is a decision on the page and not an oversight. The phase
-# gate still refuses dead code in the default-features column; only the
-# `--no-default-features` line allows it, because there it is structural.
+# that the carve-out is a decision on the page and not an oversight. It buys
+# **nothing at the gate**: the gate is one column now, `cargo clippy --workspace
+# --all-targets -- -D warnings`, and `-D warnings` promotes both back to errors
+# there. The second column and its `-A dead_code -A unreachable_pub` are retired
+# with the `shell` feature (`design.md` §5.6); what `warn` still buys is a
+# `cargo check` in the inner loop that reports rather than stops.
```

**EX-3 — the map walked forward, four predicates keyed to row kind**

*moved* — destination exists **and** source does not. 27 rows, all `ok`:
`src/semantics/{mod,error,schedule}.rs`, `src/semantics/protocol/{mod,canonical,normalize,wire}.rs`,
`src/shell/{mod,config,error,host,state}.rs`, `src/shell/backend/{mod,process,transport}.rs`,
`tests/protocol/{main,normalize,runner,transport_shape,boundary}.rs`,
`tests/integration/{main,fake,host,round_trip,transport,failure_matrix,harness}.rs`.
Fixtures: 88 at `tests/fixtures/`, `tests/protocol/fixtures/` gone.

*deleted* — `src/lib.rs` is gone and nothing claims to be its destination. `ok`.

*rewritten in place* — path exists and `git diff <pre-split> HEAD -- <path>` is
non-empty: `Cargo.toml` (140 lines), `Cargo.lock` (29), `justfile` (96). `ok`.

*unchanged in place* — path exists and the diff is **empty**: all 15
`tests/backends/*.sh`, `clippy.toml`, `rustfmt.toml`, `flake.nix`,
`examples/typescript/{backend.ts,README.md}`. Zero diff lines each. `ok`.

**EX-4 / EX-11 — the map walked backward, and the AC-2 evidence**

`git diff --find-renames --name-status -M 54a76aa HEAD` — commit to commit, so
the pre-existing unstaged `flake.lock` edit is out of it by construction. **135
rows, of which 5 are the exclusion set `docs/slices/002/**` — 130 relocated
paths.** `grep -c '^R100'` → **92**; of those, 88 are fixtures and the four named
below are not. `grep -c 'tests/backends'` → **0**. The walk below was taken
against the index before the commit and is identical to the committed one outside
the exclusion set (`diff` of the two, empty).

```
  # 88 fixture rows elided — all R100, all `tests/protocol/fixtures/X` -> `tests/fixtures/X`.
  # The four non-fixture R100 rows are printed in full (F-37):
  R100	src/semantics/error.rs	crates/goad-semantics/src/error.rs
  R100	src/semantics/mod.rs	crates/goad-semantics/src/lib.rs
  R100	src/semantics/protocol/mod.rs	crates/goad-semantics/src/protocol/mod.rs
  R100	src/shell/backend/mod.rs	crates/goad-shell/src/backend/mod.rs
  # every remaining row, R100 or not:
  M	Cargo.lock
  M	Cargo.toml
  A	crates/goad-boundary/Cargo.toml
  A	crates/goad-boundary/src/lib.rs
  R057	tests/protocol/boundary.rs	crates/goad-boundary/src/scan.rs
  A	crates/goad-boundary/tests/checks/direction.rs
  A	crates/goad-boundary/tests/checks/main.rs
  A	crates/goad-boundary/tests/checks/vocabulary.rs
  A	crates/goad-semantics/Cargo.toml
  R099	src/semantics/protocol/canonical.rs	crates/goad-semantics/src/protocol/canonical.rs
  R098	src/semantics/protocol/normalize.rs	crates/goad-semantics/src/protocol/normalize.rs
  R099	src/semantics/protocol/wire.rs	crates/goad-semantics/src/protocol/wire.rs
  R098	src/semantics/schedule.rs	crates/goad-semantics/src/schedule.rs
  R058	tests/protocol/main.rs	crates/goad-semantics/tests/protocol/main.rs
  R098	tests/protocol/normalize.rs	crates/goad-semantics/tests/protocol/normalize.rs
  R098	tests/protocol/runner.rs	crates/goad-semantics/tests/protocol/runner.rs
  A	crates/goad-shell/Cargo.toml
  R097	src/shell/backend/process.rs	crates/goad-shell/src/backend/process.rs
  R096	src/shell/backend/transport.rs	crates/goad-shell/src/backend/transport.rs
  R098	src/shell/config.rs	crates/goad-shell/src/config.rs
  R098	src/shell/error.rs	crates/goad-shell/src/error.rs
  R096	src/shell/host.rs	crates/goad-shell/src/host.rs
  R051	src/shell/mod.rs	crates/goad-shell/src/lib.rs
  R097	src/shell/state.rs	crates/goad-shell/src/state.rs
  R097	tests/integration/failure_matrix.rs	crates/goad-shell/tests/integration/failure_matrix.rs
  R094	tests/integration/fake.rs	crates/goad-shell/tests/integration/fake.rs
  A	crates/goad-shell/tests/integration/harness.rs
  R097	tests/integration/host.rs	crates/goad-shell/tests/integration/host.rs
  A	crates/goad-shell/tests/integration/main.rs
  R087	tests/integration/round_trip.rs	crates/goad-shell/tests/integration/round_trip.rs
  R097	tests/integration/transport.rs	crates/goad-shell/tests/integration/transport.rs
  A	crates/goad-shell/tests/shape/main.rs
  R098	tests/protocol/transport_shape.rs	crates/goad-shell/tests/shape/transport_shape.rs
  M	docs/slices/002/notes.md
  M	justfile
  D	src/lib.rs
  D	tests/integration/harness.rs
  D	tests/integration/main.rs
  A	tests/support/driving.rs```

**Every row against §5.1.** Nothing in the walk is a path the map does not name:
`Cargo.toml` and `Cargo.lock` are their own rows; the three member manifests are
the `Cargo.toml` row's "plus four member manifests" (three of four — PL-1);
`crates/goad-boundary/{src/lib.rs,tests/checks/*}` are the `boundary.rs` row's
"split across `src/` and `tests/`", shaped by PL-9;
`crates/goad-shell/tests/shape/main.rs` is named by the map as "a **new**
`main.rs` … an added file, not this rename's destination";
`tests/support/driving.rs` is the `harness.rs` row's destination; `src/lib.rs` is
the *deleted* row. `docs/slices/002/notes.md` is EX-4's exclusion set.

**The two `D`+`A` pairs are the map's two split rows, not lost renames.** Git's
default 50% threshold does not pair them; at `-M20%` it does:

```
R048  tests/integration/harness.rs  -> crates/goad-shell/tests/integration/harness.rs
R043  tests/integration/main.rs     -> crates/goad-shell/tests/integration/main.rs
```

`harness.rs` is below the threshold because §12.8's cut moved 60% of it into
`driving.rs` — which is the map's own "splits" row working. `main.rs` is a
28-line file that gained the `#[path]` include.

**EX-5b — the named non-identical changes, file by file**

| file (post-split) | change | source |
|---|---|---|
| `crates/goad-semantics/tests/protocol/normalize.rs` | `:266` → `../../tests/fixtures/protocol`; `:273` → `…/protocol-text` | EX-5b |
| `crates/goad-semantics/tests/protocol/runner.rs` | `:332` → `../../tests/fixtures/schedule` | EX-5b |
| `crates/goad-shell/tests/shape/transport_shape.rs` | three subject paths: `src/backend/process.rs`, `src/backend/process-renamed.rs`, `src/error.rs` | EX-5b |
| `crates/goad-shell/tests/integration/round_trip.rs` | `include_str!("../../../../examples/typescript/README.md")` | EX-5b |
| `crates/goad-shell/tests/integration/round_trip.rs` | **`rooted_at_the_workspace`** — the README's relative script path rebased | **F-35, new** |
| `crates/goad-shell/tests/integration/harness.rs` | `example()` → `../../examples/typescript/backend.ts`; the §12.8 cut | EX-5b |
| `crates/goad-shell/tests/integration/transport.rs` | gains `use crate::driving::CLEANUP_LIMIT;` | EX-5b |
| `tests/support/driving.rs` | `backend()` → `../../tests/backends` | EX-5b |
| `crates/goad-boundary/**` | EX-5c, five items | EX-5c + F-36 |
| `crates/goad-shell/tests/integration/{host,failure_matrix}.rs` | `use crate::harness::{…}` splits into `crate::driving::{…}` + `crate::harness::{…}` | EX-5a `use` line |
| `crates/goad-shell/tests/integration/harness.rs` | `pub(crate) use crate::driving::{backend, clear, marker};` — a re-export, so `transport.rs`'s `harness::backend(…)` call sites are untouched | EX-5a `use` line |
| seven files, nine lines | comments the split falsifies | **F-34, new** |

**EX-5c — `boundary.rs`'s change is five things, not four**

1. the `src/` ÷ `tests/` division, in PL-9's shape: `src/lib.rs` declares
   `pub mod scan;`; `src/scan.rs` carries `Scan`, `Breach`, `report`, `mentions`,
   `code_of`, `camel_segments` and the walk; `tests/checks/` carries `main.rs`,
   `direction.rs` and `vocabulary.rs`.
2. `#[derive(Debug)]` on `Scan` — `missing_debug_implementations` is `deny`.
3. a `# Errors` section on `Scan::run` — `clippy::missing_errors_doc` is
   pedantic-at-deny on a public fallible fn.
4. the workspace-root rebase: `Scan::root()` is `CARGO_MANIFEST_DIR` joined with
   `../..` then `self.root`; the four configured roots become
   `crates/goad-semantics/src` (direction), the three member `src/` directories
   (vocabulary), `docs/adr` (unchanged — it is workspace-relative and really
   there), and `crates/goad-semantics-renamed/src`.
5. **`camel_segments` loses two `bytes[i]` reads** (F-36). `indexing_slicing` is
   `deny` and `clippy.toml:23`'s `allow-indexing-slicing-in-tests` stopped
   applying when the function became library code. A self-zip and one `.get`, the
   same two bytes, **no `#[expect]`** — the S-1 budget is untouched.

Four items are `pub` for the division: `Scan` (with both fields), `Breach`,
`report`, `mentions`. `assert_clean` did **not** move into the library: it
`panic!`s, and `clippy.toml`'s `allow-panic-in-tests` covers a test target and
not a lib. It lives in `tests/checks/vocabulary.rs` and `direction.rs` imports
it — a phase-local placement, recorded under *Decisions* below.

**EX-7 — the §12.8 cut, item by item. Nothing in `harness.rs` unaccounted for.**

| item | goes to | why |
|---|---|---|
| `scripted`, `logging_backend`, `backend`, `marker` | `driving.rs` | §12.8 |
| `clear` | `driving.rs` | closure — `marker` calls it |
| `invocations`, `config`, `host`, `instant` | `driving.rs` | §12.8 |
| `DEFAULT_POLL` | `driving.rs` | closure — `config` reads it |
| `host_from` | `driving.rs` | closure — `host` calls it |
| `quiet_event` | `driving.rs` | §12.8 |
| `event` | `driving.rs`, as `pub(crate)` | closure — `quiet_event` calls it; `prompting_event` stayed and calls `crate::driving::event` |
| `describe_outcome`, `choice`, `answer_first_option`, `presented` | `driving.rs` | §12.8; the last three call `describe_outcome` |
| `CLEANUP_LIMIT` | `driving.rs`, with its keep-in-sync note | §12.8 — and it was at `tests/integration/transport.rs:22`, not in `harness.rs` (DF-5) |
| `transport`, `describe`, `describe_cleanup`, `stderr`, `children`, `alive`, `reported_pid`, `padded_evaluate`, `example` | `harness.rs` | §12.8's transport list |
| `backend_error`, `state_error`, `protocol_error`, `only_discard`, `stderr_of` | `harness.rs` | §12.8's `Outcome` accessors |
| `evaluate`, `children_running`, `prompting_event`, `command_line` | `harness.rs` | named explicitly as staying; each transport- or integration-local |

All 18 `driving.rs` items have a live caller in the `integration` target, so
`dead_code` under `-D warnings` does not fire — PL-4's condition, checked by the
gate rather than asserted.

**EX-10 — instrument 1, observed**

```
== control 1: a stratum 1 source names goad_shell ==
error[E0433]: cannot find module or crate `goad_shell` in this scope
  --> crates/goad-semantics/src/error.rs:12:5
   |
12 | use goad_shell::host::Host;
   |     ^^^^^^^^^^ use of unresolved module or unlinked crate `goad_shell`
error: could not compile `goad-semantics` (lib) due to 1 previous error

== control 2: a stratum 1 source names tokio ==
error[E0433]: cannot find module or crate `tokio` in this scope
  --> crates/goad-semantics/src/error.rs:12:5
   |
12 | use tokio::process::Command;
   |     ^^^^^ use of unresolved module or unlinked crate `tokio`
error: could not compile `goad-semantics` (lib) due to 1 previous error

== reverted: 0 diff lines ==
```

**EX-14 — the relocated checks demonstrated non-vacuous**

*(a) all three fixture corpora resolve and are non-empty.* `tests/fixtures/`:
protocol **60**, protocol-text **4**, schedule **24** — 88. The five
`protocol` tests pass, so `runner.rs:98`'s *"ran no fixtures — renamed, emptied,
or misspelled"* appears for none of the three. Control: point `PROTOCOL` at
`../../tests/fixtures/protocol-renamed` →
`…/tests/fixtures/protocol-renamed: ran no fixtures — renamed, emptied, or
misspelled`, `3 failed`. Reverted.

*(b) the `shape` target's negative control still fails for its own reason.*
`crates/goad-shell/src/backend/` holds `mod.rs`, `process.rs`, `transport.rs` —
**no `process-renamed.rs`**. Control: point `RENAMED_AWAY` at
`src/backend/process.rs`, which is there →
`a_check_whose_subject_is_not_there_fails ... FAILED`. Reverted. So the
re-rooting of the three subject paths is witnessed, not assumed.

*(c) both boundary vacuity controls fail for their own **distinct** reasons.*
`docs/adr` exists at the workspace root, holds 2 entries and **0** `.rs` files —
`NOTHING_TO_INSPECT` fails because there is nothing to inspect.
`crates/goad-semantics-renamed` does **not** exist — `RENAMED_AWAY` fails because
its root is missing. The `../..` rebase is what keeps them distinct: without it
`docs/adr` would resolve to `crates/goad-boundary/docs/adr`, which does not
exist, and the two controls would collapse into one (F-24's warning, avoided).

**EX-13 — the content check, mechanical**

Full hunks for every non-`R100` renamed file **except** the three the map itself
marks rewritten or split — `boundary.rs → scan.rs` (170 changed lines, the one
file AC-2 obliges an argument for; the argument is EX-5c above),
`harness.rs` (232, the §12.8 cut inventoried in EX-7), and `round_trip.rs` (51,
the imports plus F-35's helper). For those three the map's own column is
"substantively rewritten" / "splits", so an inventory is the evidence and a hunk
is noise; for the other twenty, the hunks are the evidence and they are here in
full.

**Result: every `+`/`-` line below is a `use` line, a `mod` line, a
`#[cfg]`/`#[path]` attribute, or a path string literal — EX-5a's vocabulary —
except nine comment lines, which are F-34's class and are listed in EX-5b.
Three of the nine are in production sources; those three are the only lines in
`crates/goad-semantics/src` or `crates/goad-shell/src` that PS-1's letter
touches, and each is a sentence the split itself makes false.**

```
  src/semantics/protocol/canonical.rs -> crates/goad-semantics/src/protocol/canonical.rs
    -use crate::semantics::error::{BoundsError, ProtocolError};
    +use crate::error::{BoundsError, ProtocolError};
    -  use crate::semantics::error::{BoundsError, ProtocolError};
    +  use crate::error::{BoundsError, ProtocolError};

  src/semantics/protocol/normalize.rs -> crates/goad-semantics/src/protocol/normalize.rs
    -use crate::semantics::error::{ProtocolError, ScheduleError};
    -use crate::semantics::protocol::canonical::{
    +use crate::error::{ProtocolError, ScheduleError};
    +use crate::protocol::canonical::{
    -use crate::semantics::protocol::wire::{
    +use crate::protocol::wire::{
    -use crate::semantics::schedule;
    +use crate::schedule;

  src/semantics/protocol/wire.rs -> crates/goad-semantics/src/protocol/wire.rs
    -use crate::semantics::error::{ProtocolError, json_type_name};
    +use crate::error::{ProtocolError, json_type_name};

  src/semantics/schedule.rs -> crates/goad-semantics/src/schedule.rs
    -use crate::semantics::error::{ScheduleError, SpanFault, json_type_name};
    -use crate::semantics::protocol::canonical::Timestamp;
    +use crate::error::{ScheduleError, SpanFault, json_type_name};
    +use crate::protocol::canonical::Timestamp;
    -  use crate::semantics::protocol::canonical::Timestamp;
    -  use crate::semantics::schedule::{parse, resolve};
    +  use crate::protocol::canonical::Timestamp;
    +  use crate::schedule::{parse, resolve};
    -  // `tests/protocol/fixtures/schedule/`. They were written here first, as the
    +  // `tests/fixtures/schedule/`. They were written here first, as the

  tests/protocol/main.rs -> crates/goad-semantics/tests/protocol/main.rs
    -//! Stratum 1 test target. Runs in both feature columns, and in the
    -//! `--no-default-features` column it runs with no async runtime resolvable at
    -//! all — naming `tokio` here is a compile error, which is the point.
    +//! Stratum 1 test target. `goad-semantics` has no async runtime anywhere in its
    +//! dependency graph, so naming `tokio` here is a compile error, which is the
    +//! point.
    -#[cfg(test)]
    -mod boundary;
    -#[cfg(test)]
    -mod transport_shape;

  tests/protocol/normalize.rs -> crates/goad-semantics/tests/protocol/normalize.rs
    -use goad::semantics::error::{BoundsError, ProtocolError, ScheduleError};
    -use goad::semantics::protocol::canonical::{
    +use goad_semantics::error::{BoundsError, ProtocolError, ScheduleError};
    +use goad_semantics::protocol::canonical::{
    -use goad::semantics::protocol::normalize::{Discarded, Normalized, read_response};
    +use goad_semantics::protocol::normalize::{Discarded, Normalized, read_response};
    -  root: "tests/protocol/fixtures/protocol",
    +  root: "../../tests/fixtures/protocol",
    -  root: "tests/protocol/fixtures/protocol-text",
    +  root: "../../tests/fixtures/protocol-text",

  tests/protocol/runner.rs -> crates/goad-semantics/tests/protocol/runner.rs
    -use goad::semantics::error::ScheduleError;
    -use goad::semantics::protocol::canonical::Timestamp;
    -use goad::semantics::schedule::parse;
    +use goad_semantics::error::ScheduleError;
    +use goad_semantics::protocol::canonical::Timestamp;
    +use goad_semantics::schedule::parse;
    -  root: "tests/protocol/fixtures/schedule",
    +  root: "../../tests/fixtures/schedule",

  src/shell/backend/process.rs -> crates/goad-shell/src/backend/process.rs
    -use crate::semantics::protocol::canonical::Request;
    -use crate::shell::backend::transport::{Backend, Captured, Exchange};
    -use crate::shell::config;
    -use crate::shell::error::{BackendError, CleanupFailure};
    +use crate::backend::transport::{Backend, Captured, Exchange};
    +use crate::config;
    +use crate::error::{BackendError, CleanupFailure};
    +use goad_semantics::protocol::canonical::Request;
    -  use crate::shell::error::BackendError;
    +  use crate::error::BackendError;
    -  /// cases in `tests/integration/transport.rs` pass against that
    +  /// cases in `crates/goad-shell/tests/integration/transport.rs` pass against that

  src/shell/backend/transport.rs -> crates/goad-shell/src/backend/transport.rs
    -use crate::semantics::protocol::canonical::Request;
    -use crate::shell::error::{BackendError, CleanupFailure};
    +use crate::error::{BackendError, CleanupFailure};
    +use goad_semantics::protocol::canonical::Request;

  src/shell/config.rs -> crates/goad-shell/src/config.rs
    -use crate::semantics::schedule::parse_span;
    -use crate::shell::error::ConfigError;
    +use crate::error::ConfigError;
    +use goad_semantics::schedule::parse_span;
    -  use crate::shell::error::ConfigError;
    +  use crate::error::ConfigError;

  src/shell/error.rs -> crates/goad-shell/src/error.rs
    -use crate::semantics::error::{ProtocolError, SpanFault};
    -use crate::semantics::protocol::canonical::ViewId;
    +use goad_semantics::error::{ProtocolError, SpanFault};
    +use goad_semantics::protocol::canonical::ViewId;

  src/shell/host.rs -> crates/goad-shell/src/host.rs
    -use crate::semantics::protocol::canonical::{
    +use crate::backend::transport::{Backend, Captured, Exchange};
    +use crate::config::Config;
    +use crate::error::{BackendError, CleanupFailure, StateError};
    +use crate::state::State;
    +use goad_semantics::protocol::canonical::{
    -use crate::semantics::protocol::normalize::{Discarded, Normalized, read_response};
    -use crate::semantics::schedule;
    -use crate::shell::backend::transport::{Backend, Captured, Exchange};
    -use crate::shell::config::Config;
    -use crate::shell::error::{BackendError, CleanupFailure, StateError};
    -use crate::shell::state::State;
    +use goad_semantics::protocol::normalize::{Discarded, Normalized, read_response};
    +use goad_semantics::schedule;

  src/shell/mod.rs -> crates/goad-shell/src/lib.rs
    -//! event ingress. Compiled only with the `shell` feature, which is the whole of
    -//! what keeps the runtime out of stratum 1's graph (ADR-001, D49).
    +//! event ingress. A crate of its own, which is what keeps the runtime out of
    +//! stratum 1's graph (ADR-001, D49).

  src/shell/state.rs -> crates/goad-shell/src/state.rs
    -use crate::semantics::protocol::canonical::{Timestamp, ViewId};
    -use crate::shell::error::StateError;
    +use crate::error::StateError;
    +use goad_semantics::protocol::canonical::{Timestamp, ViewId};
    -  use crate::semantics::protocol::canonical::{Timestamp, ViewId};
    -  use crate::shell::error::StateError;
    +  use crate::error::StateError;
    +  use goad_semantics::protocol::canonical::{Timestamp, ViewId};

  tests/integration/failure_matrix.rs -> crates/goad-shell/tests/integration/failure_matrix.rs
    -//! same bodies — `tests/protocol/fixtures/protocol/` is where each one below was
    +//! same bodies — `tests/fixtures/protocol/` is where each one below was
    -use crate::harness::{
    -  answer_first_option, backend_error, describe_outcome, host, instant, invocations, only_discard,
    -  presented, protocol_error, quiet_event, scripted, stderr_of,
    +use crate::driving::{
    +  answer_first_option, describe_outcome, host, instant, invocations, presented, quiet_event,
    +  scripted,
    -use goad::semantics::error::{BoundsError, ProtocolError, ScheduleError};
    -use goad::semantics::protocol::canonical::Timestamp;
    -use goad::semantics::protocol::normalize::Discarded;
    -use goad::shell::config::Command;
    -use goad::shell::error::BackendError;
    -use goad::shell::host::Outcome;
    +use crate::harness::{backend_error, only_discard, protocol_error, stderr_of};
    +use goad_semantics::error::{BoundsError, ProtocolError, ScheduleError};
    +use goad_semantics::protocol::canonical::Timestamp;
    +use goad_semantics::protocol::normalize::Discarded;
    +use goad_shell::config::Command;
    +use goad_shell::error::BackendError;
    +use goad_shell::host::Outcome;

  tests/integration/fake.rs -> crates/goad-shell/tests/integration/fake.rs
    -use goad::semantics::protocol::canonical::Request;
    -use goad::shell::backend::transport::{Backend, Captured, Exchange};
    -use goad::shell::error::{BackendError, CleanupFailure};
    +use goad_semantics::protocol::canonical::Request;
    +use goad_shell::backend::transport::{Backend, Captured, Exchange};
    +use goad_shell::error::{BackendError, CleanupFailure};

  tests/integration/host.rs -> crates/goad-shell/tests/integration/host.rs
    +use crate::driving::{describe_outcome, instant, presented};
    -use crate::harness::{
    -  backend_error, describe_outcome, instant, only_discard, presented, state_error,
    -};
    -use goad::semantics::protocol::canonical::{Event, Timestamp, UserResponse, View, ViewId};
    -use goad::shell::config::Config;
    -use goad::shell::error::{BackendError, CleanupFailure, StateError};
    -use goad::shell::host::Host;
    +use crate::harness::{backend_error, only_discard, state_error};
    +use goad_semantics::protocol::canonical::{Event, Timestamp, UserResponse, View, ViewId};
    +use goad_shell::config::Config;
    +use goad_shell::error::{BackendError, CleanupFailure, StateError};
    +use goad_shell::host::Host;
    -fn host(scripted: Vec<goad::shell::backend::transport::Exchange>) -> (Host<FakeBackend>, Calls) {
    +fn host(scripted: Vec<goad_shell::backend::transport::Exchange>) -> (Host<FakeBackend>, Calls) {
    -        BackendError::Protocol(goad::semantics::error::ProtocolError::Json(_))
    +        BackendError::Protocol(goad_semantics::error::ProtocolError::Json(_))

  tests/integration/main.rs -> crates/goad-shell/tests/integration/main.rs
    -//! `required-features = ["shell"]` in `Cargo.toml` is what makes
    -//! `cargo test --no-default-features` skip this target rather than fail to
    -//! build it.
    +//! It is a target of `goad-shell`, so `cargo test -p goad-semantics` does not
    +//! build it at all; nothing gates it by feature.
    -// as `tests/protocol/main.rs` explains: a `tests/` target is always built with
    -// `--test`, so this is never off.
    +// as `crates/goad-semantics/tests/protocol/main.rs` explains: a `tests/` target
    +// is always built with `--test`, so this is never off. The `#[path]` include
    +// carries the attribute at its declaration site for the same reason.
    +#[cfg(test)]
    +#[path = "../../../../tests/support/driving.rs"]
    +mod driving;

  tests/integration/transport.rs -> crates/goad-shell/tests/integration/transport.rs
    +use crate::driving::CLEANUP_LIMIT;
    -use goad::shell::backend::process::ProcessBackend;
    -use goad::shell::backend::transport::Backend;
    -use goad::shell::config::Command;
    -use goad::shell::error::{BackendError, CleanupFailure};
    -/// The transport's own cleanup budget, restated because it is private to
    -/// `process.rs` and these bounds are about it. If it changes there, VA-3's
    -/// assertions below are wrong until this does too.
    -const CLEANUP_LIMIT: Duration = Duration::from_millis(500);
    +use goad_shell::backend::process::ProcessBackend;
    +use goad_shell::backend::transport::Backend;
    +use goad_shell::config::Command;
    +use goad_shell::error::{BackendError, CleanupFailure};
    -/// `tests/protocol/boundary.rs` and `transport_shape.rs` both carry one of
    +/// `crates/goad-boundary/src/scan.rs` and `transport_shape.rs` both carry one of
    -// `tests/protocol/transport_shape.rs`, which asserts the transport spawns
    +// `crates/goad-shell/tests/shape/transport_shape.rs`, which asserts the transport spawns

  tests/protocol/transport_shape.rs -> crates/goad-shell/tests/shape/transport_shape.rs
    -  path: "src/shell/backend/process.rs",
    +  path: "src/backend/process.rs",
    -  path: "src/shell/backend/process-renamed.rs",
    +  path: "src/backend/process-renamed.rs",
    -/// `src/shell/backend/process.rs` renamed, the check left pointing at where it
    +/// `src/backend/process.rs` renamed, the check left pointing at where it
    -    path: "src/shell/error.rs",
    +    path: "src/error.rs",
```

**Decisions taken during execution**

Small and local, inside what the design already settled. The three that amend a
plan criterion are **PL-13** in `plan-log.md`; the four design repairs are the
`design-log.md` entry of 2026-09-05, *"§5.1's artifact map, repaired again"*.

- **`assert_clean` lives in `tests/checks/vocabulary.rs`, not in the library.**
  It `panic!`s, and `clippy.toml`'s `allow-panic-in-tests` covers a test target
  and not a lib, so moving it into `scan.rs` would need an `#[expect]`.
  `direction.rs` imports it. *Rejected:* duplicating three lines into both
  modules (DRY, and two statements of one rule), and a third `checks` module for
  two helpers (EX-6 fixes the module list at two, and a module per function is
  not a shape).
- **`vocabulary.rs` also holds the two vacuity controls and the word-matching
  test.** Both belong to `goad_boundary::scan` rather than to either
  configuration of it; `direction.rs` has one scan and no controls of its own.
  The word test is kept **verbatim**, including its three `crate::shell` string
  cases, because it tests `mentions`'s substring rule and not the tree.
- **`harness.rs` re-exports `backend`, `marker` and `clear` from `driving`**
  rather than `transport.rs` re-spelling its call sites. A `pub(crate) use` is
  inside EX-5a's vocabulary and a call-site path change is not, so this is the
  repair that keeps `transport.rs`'s diff to the one line EX-5b names. Measured:
  `clippy::pub_use` (deny) does **not** fire on `pub(crate) use`.
- **`resolver = "3"` in `[workspace]`.** Not in EX-8a's skeleton; a virtual
  manifest with edition-2024 members needs it stated. It is a manifest key, which
  is EX-5a vocabulary.
- **`fmt` becomes `cargo fmt --all`.** Not in the gate, so not in §5.6's six —
  but a `fmt` that formats one member is a trap next to a `fmt-check` that checks
  all three.
- **`workspace.members` is in stratum order** — semantics, shell, boundary — and
  `vocabulary.rs`'s three hand-written scans are in the same order, so the two
  lists read as one thing until PHASE-02 enumerates them.
- **`slint` and `slint-build` are not added to `[workspace.dependencies]`.** The
  map lists them because it describes the slice's end state; D1 says the split
  lands before Slint enters the tree, and PL-1 defers `crates/goad` to PHASE-03.
  An unused workspace dependency would make "before Slint enters the tree" a
  claim about a manifest rather than about a graph.
- **The test name `no_host_source_file_names_the_user_s_domain` was restored.**
  The first VT-1 run showed it renamed to `no_member_…`; the name-set diff caught
  it, and the name was put back rather than absorbed. That is the criterion doing
  exactly what PL-12 settled the shape for.

**Findings**

Four are in `review-plan.md` round 2 — **F-34** (the comment class EX-5a has no
entry for), **F-35** (the README case rests on cargo's working directory; a
blocker, and the only thing that stood between the relocation and a green gate),
**F-36** (EX-5c's four is five, and the class is `clippy.toml`'s four
`allow-*-in-tests` keys), **F-37** (the `R100` set is 92, not 88). All four are
`verified`; `design.md` §5.1 is repaired for all four.

Carried forward, not this phase's to fix:

- **DF-6 is now half-discharged in code and the design has not caught up.**
  `Scan` carries the `#[derive(Debug)]` that §5.6's block omits, so §5.6 and the
  tree disagree from here on. Audit's *Design drift not reconciled*.
- **`plan-log.md` PL-3's Consequence still says "PHASE-01/EX-5 asserts exactly
  **91** byte-identical renames".** Superseded twice over — by F-1/F-17 (88) and
  now by F-37 (92). The log is append-only, so it stands as written; a reader who
  reaches it out of order will be misled.
- **PHASE-02 inherits F-36's class.** It moves `code_of` and adds `members.rs`
  and `manifest.rs` to the same library. Every item that crosses from
  `tests/checks/` into `crates/goad-boundary/src/` loses `unwrap_used`,
  `expect_used`, `panic` and `indexing_slicing` exemptions at once.
- **PHASE-06 inherits the `harness.rs` re-export.** When the `renderer` target
  first includes `driving.rs` and PL-4's re-settlement runs, the three re-exported
  names are the seam to look at first.
- **`tests/support/` has no consumer but a `#[path]` include.** Nothing enforces
  that: a future member could add it as a module the ordinary way and nothing
  would object. Noted, not instrumented.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-05 · PHASE-01 · the split commit on `slice-002`

### Produced

- Three members — `goad-semantics`, `goad-shell`, `goad-boundary` — and four
  `[[test]]` targets: `protocol`, `integration`, `shape`, `checks`. `crates/goad`
  and the `renderer` / `event_loop` targets are PHASE-03's and PHASE-08's (PL-1).
- `tests/support/driving.rs` at the workspace root, §12.8's cut made and
  inventoried (PHASE-01/EX-7), included by one literal `#[path]`.
- `tests/fixtures/**` at the workspace root — 88 files, byte-identical. CD-4.
- The six-command gate in the `justfile`, header repointed to `draft-policy.md`
  and §5.6. `CLAUDE.md` untouched; CD-5 is still audit's.
- `crates/goad-boundary/src/{lib,scan}.rs` and `tests/checks/{main,direction,
  vocabulary}.rs` — PL-9's layout, PHASE-02's starting point.
- `review-plan.md` round 2, F-34…F-37; `design.md` §5.1 repaired at four rows;
  `plan-log.md` PL-13.

### Learned

Durable enough for `docs/memory/`, and none of it reachable by reading:

- **`clippy.toml`'s four `allow-*-in-tests` keys are a hidden boundary.**
  `unwrap_used`, `expect_used`, `panic` and `indexing_slicing` are all `deny`
  here and all exempted in test code, so **every item relocated from a test
  target into a library crosses four lint boundaries at once** — silently, until
  the gate says so. F-36 is the instance; the class costs a phase a compile every
  time it moves test-shaped code into a crate. PHASE-02 does it again.
- **Cargo runs a test binary with the *package* root as its working directory.**
  Before a workspace split that is the repository root; after it, it is
  `crates/<member>`. Anything resting on the two being the same — a config with a
  relative path, a fixture found by cwd — breaks at the split and no import change
  reaches it. F-35. The general rule the tree already states is
  `CARGO_MANIFEST_DIR` + `../..`, and it is stated for exactly this reason.
- **A `git mv`-only split leaves more files byte-identical than a reading
  predicts, not fewer.** 92 `R100` rows against a predicted 88: `error.rs` and
  three `mod.rs` files needed none of the change their map row permitted (F-37).
  Assert **⊇** on such a set, never `=`; an equality punishes the split for being
  cleaner than forecast.
- **`clippy::pub_use` does not fire on `pub(crate) use`** — measured, so a
  re-export is available for keeping a call site's module path stable across a
  helper's move.
- **A workspace split of this shape costs nothing at the gate.** Warm `just
  check`: **1.808 s** pre-split, **1.797 s** post-split, three runs each. The
  number A-4 and ADR-002 T3 turn on is PHASE-03's, when `slint` enters the graph.
- **The name-set diff is the right shape for VT-1.** It caught a test function
  this phase renamed in passing, which a count equality would have absorbed
  (PL-12).

### Open

- **CD-1…CD-7 and `draft-policy.md`** — unchanged, unpromoted, audit's, and the
  user's alone. The `justfile` now cites the draft as the slice's working
  authority (`docs/AGENTS.md:36`), which is what CD-5 will make permanent.
- **DF-6 has diverged in code:** `Scan` carries a `#[derive(Debug)]` §5.6's block
  omits. Audit's *Design drift not reconciled*.
- **`plan-log.md` PL-3's Consequence carries the twice-superseded 91.**
  Append-only, so it stands; a reader arriving there out of order is misled.
- **A-1, A-2, A-3, A-4 all still stand** and all still need `slint` in the graph.
  The A-2 expectation budget is **unspent** — PHASE-01 added no `#[expect]`
  anywhere.

---

## Handover

**Written:** 2026-09-05, end of session 3. This section **replaces** the session-1
handover and the session-2 and session-3 addenda. It is the whole state, not a
delta; nothing earlier in this file needs reading first.

| | |
|---|---|
| branch | `slice-002`, 9 commits ahead of `a6ae617` (slice 001's close) |
| head | `d880801` — *review round 5: seven findings raised, six reopened, all repaired* |
| gate | **verified this session:** `just check` exits 0, 1.836 s warm |
| tree | clean apart from the pre-existing unstaged `flake.lock` edit (a `bun2nix` input repointed to a `Mic92` fork). Untouched for three sessions. Leave it alone |
| canon | **untouched.** `git diff a6ae617..HEAD -- docs/specs docs/policy docs/adr CLAUDE.md` is empty |
| stage | the design gate. `plan.md` is still the template. **No code has changed in three sessions** |

The slice has produced 12,569 lines of documentation and 0 lines of code. That
is the fact this handover has to justify or condemn, and §5 does the arithmetic.

---

### 1. What rounds 4 and 5 built, and what the compiler said

Round 3 ended by writing itself a lesson: *an assumption a scratch crate can
reach should be reached before a phase starts, not listed as a risk.* Rounds 4
and 5 spent themselves discharging it. Six scratch measurements, all recorded in
`research.md` as numbered threads.

**Round 4 — three passages built, three defective.** This is the round where the
compiler became the raiser rather than the reviewer.

| built | thread | what the compiler said |
|---|---|---|
| F-9's failure-matrix `Case`/`Observed`/`Cohort` schema | Thread 9 | **14 corrections**, two of them expected strings that disagree with the fixtures on disk |
| F-26's startup surface — `StartupError`'s eight variants, `arguments(argv, env)`, the `writeln!` outlet | Thread 10 | **5 corrections**, including an `arguments` call site that does not compile |
| A-2's two named lint instances — the stderr outlet, six `Wire` clone bindings against `shadow_unrelated` | Thread 11 | **13 errors across 9 lints** on the design's own text, plus 8 more in the rasteriser |

Five findings followed (F-29…F-33), two of them blockers: `serve` carried an
`#[expect]` for a lint that does not fire (F-29), and **every `pub` item in the
renderer was a lint error because the crate's shape had never been stated**
(F-30). F-30's repair is D28 — a library plus a thin binary — and it is load
bearing for the whole test strategy.

One judgement call from that round is worth a reader's attention. **Thread 10's
prescription was rejected and its measurement kept.** It measured
`unreachable_pub` correctly and concluded *"`pub(crate)`, not a lib target"*,
which is right for a crate with no integration tests and wrong for this one:
§12.8 runs the cheap tier in a `tests/` target that `pub(crate)` locks out. A
measurement is evidence about the shape it was taken on, not about a shape it
never saw.

**Round 5 — three artefacts built or validated, one corrected.** The first round
in which measurement mostly *confirmed*.

| measured | result |
|---|---|
| round 4's `f9-schema` scratch crate, re-run | **still green.** `--test table` 8/8, clippy `-D warnings` exit 0. The 33-row `CASES` array is now copied into `design.md` §9 item 12.9 **verbatim from the crate that compiled it**, not paraphrased; its `<A>` placeholder is documented as a placeholder |
| `makeFontsConf { fontDirectories = [ dejavu_fonts ]; }` evaluated, built, `fc-list`'d | 39 DejaVu faces; the conf carries the store path as an explicit `<dir>`. **Found the trap: adding the font to `buildInputs` alone does nothing** — this is the one correction |
| `niri validate` on the proposed `window-rule`, niri 26.04 | *config is valid* |

Round 5 raised seven findings (F-34…F-40) and reopened six under their existing
ids (F-6, F-8, F-9, F-16, F-17, F-30). All thirteen `fix-now`, all `verified`.
Two were blockers, and the second is the one that matters:

- **F-30 regressed under integration.** Round 4 raised it with a compiler,
  repaired it correctly, and made §5.1 say *"everything a test can reach,
  `install` included, lives in the library."* Nine hundred lines later the
  prescription still read `fn install(…)` — private, reachable from neither the
  binary crate nor a `tests/` target. A rule stated and not applied to the site
  the rule came from.
- **F-37 was never raised by four rounds.** The design could state the exact
  `Display` of thirty-three diagnostic lines and could not say what its two test
  targets were called, where `build.rs`'s input lived, or which modules `lib.rs`
  declares. Four rounds asked *does this work?*; none asked *can this be typed?*

**What is now specified that was not.** F-37's repair, `design.md` §5.1 *The
artifact map* (`design.md`:291–445, ~155 lines), is the largest single addition
this slice has made and is what PHASE-01 executes directly against.

| what | where |
|---|---|
| the split's source→destination table, 111 files, with a "change permitted" column AC-2 reads against | `design.md` §5.1, the artifact map |
| four member manifests, dependency by dependency, with per-member feature sets | same |
| six `[[test]]` targets by name, path and `main.rs` module list | same |
| the shared helper `tests/support/driving.rs` and its literal `#[path = "../../../../tests/support/driving.rs"]` | same |
| `crates/goad/src/lib.rs`, ten `pub mod` lines | same |
| which of §9's seventeen validation items runs in which target | same |
| `SlintGlass` — module, fields, constructor, `impl Glass` — and what a `show`/`hide` failure does | `design.md` §5.3 |
| `pub fn install`, in `install.rs`; `StartupError`'s module and derives | `design.md` §5.4 |
| `code_of -> Cow<'_, str>`, and `goad-boundary`'s whole public API | `design.md` D13, §5.6 |
| four numeric thresholds and eight STOP conditions, S-1…S-8 | `design.md` §5.5, §8 R2, §9 item 14a |
| the font — `pkgs.dejavu_fonts` + `makeFontsConf` + `FONTCONFIG_FILE` | `design.md` D12 |
| the validated niri `window-rule` | `design.md` §5.4 |
| the counting rule — four ADR-001 instruments, the vocabulary scan, one named residue — with **one** home, cited by six documents | `design.md` §5.1 |

Writing the artifact map forced two design decisions: **one** `ui/app.slint`
(the shape `research.md` Thread 8 actually compiled), and `slint`/`slint-build`
pinned `= 1.17.1`, so A-1 and A-3 change on a deliberate upgrade rather than on
resolver drift.

---

### 2. The previous handover's four open items — one closed, three not

Session 2's handover left four. Checked against the tree, not against the report:

| # | item | state |
|---|---|---|
| 1 | **`draft-policy.md` and `canon-delta.md` CD-5 read against `design.md` §10 C-5** — the last unreviewed artefact pair | **CLOSED.** Round 5 did exactly this reading and it produced two findings. F-36: the draft legislated repository-wide rules the design never derived — three of four clauses are now derived clause by clause in a new block under §10 C-5, and the fourth (lint discipline) is **cut**, which is also F-16's structural fix. F-35 corrected a context-dependent evidence path in `canon-delta.md`, and the class fix went into its preamble |
| 2 | **A-4** — `just check` wall-clock with 411 crates in the tree (ADR-002 T3) | **OPEN, and not closable by a spike.** It needs `slint` in the graph. It now has a protocol and three numbered bands instead of the word "tolerable" (S-4: ≤ 120 s local, ≤ 300 s, stop). Today's baseline, measured this session: **1.836 s warm, pre-split** |
| 3 | **The canon decisions, CD-1…CD-7 and `draft-policy.md`** | **OPEN, for the third session running — and larger than it was.** A debt discharged at audit, not a gate on the work in front of it. See §3 |
| 4 | **`plan.md` is not begun** | **OPEN, and blocked by nothing.** Session 3 recorded it as waiting on item 3. That was a misreading of the methodology: `docs/AGENTS.md:36` — *"while the slice runs, the draft is its working authority: design, plan and execution cite it exactly as they would the real thing"* — and promotion happens at audit (`:38`). `plan.md` therefore cites `canon-delta.md` and `draft-policy.md` as binding, and keeps them current |

Round 5 **enlarged** item 3 rather than shrinking it. F-36's repair rewrote
`draft-policy.md`'s Scope and Compliance and added a derived scope block to §10
C-5; F-6's repair changed the wording that CD-1 and CD-7 will transcribe into a
new ADR and into `CLAUDE.md`. The user must endorse the movements as they now
read, not as session 1 described them.

---

### 3. What is open, and what only the user can decide

#### 3a. Canon — the one thing the autonomy grant withholds

Nothing under `docs/specs/`, `docs/policy/` or `docs/adr/` has been created or
edited on this branch. Verified: the diff is empty. Every movement below is
drafted in the slice folder and waits **for audit** — `docs/AGENTS.md:38`. None
of them gates `plan.md` or a phase: for the duration of the slice the drafts are
the working authority and are cited as canon would be (`docs/AGENTS.md:36`).

| # | movement | vehicle | needed |
|---|---|---|---|
| CD-1 | a new ADR **superseding** ADR-002 (the split) | `canon-delta.md` | the *decision* is endorsed (`design-log.md`, 2026-09-05); the **wording** is not, and F-6 changed it |
| CD-2 | ADR-002's stated reason for expecting T1 is measurably false; the superseding ADR states the real ground | `canon-delta.md` | with CD-1 |
| CD-3 | SPEC-001 has no rule at the glass — R-20's no-silent-dropping stops before the renderer | `canon-delta.md` | at audit |
| CD-4 | SPEC-001 §7 names the fixture directory normatively; moving it is a canon change | `canon-delta.md` | at audit |
| CD-5 | `CLAUDE.md`'s gate pointer moves off a **closed slice's design** (`docs/slices/001/design.md` §9) | `canon-delta.md` | **lands with the policy below, or neither lands** |
| CD-6 | `CLAUDE.md` invariant 1 — the boundary test must grow to grep `.slint` | `canon-delta.md` | at audit |
| CD-7 | `CLAUDE.md`'s "both feature columns" becomes false the day the split lands | `canon-delta.md` | at audit; F-6 changed the wording |
| — | **new** policy: the phase gate — six commands, four enforcement instruments, one named residue, and a scope now derived clause by clause | `draft-policy.md` | **paired with CD-5** |

**Two things to decide, and the first is a pair by construction.** Applying CD-5
alone leaves `CLAUDE.md` pointing at nothing; promoting the draft alone leaves
two claimants to the gate.

1. **Endorse the canon movements**, or their timing.
2. **Decide what discharges the design gate** — §5 argues that "no reviewer
   objects" has no fixed point here and proposes a replacement.

Already granted and needing nothing further: the crate split, the
tray-plus-window shape, the font package in `flake.nix`, and the dependency set
(`slint`, `slint-build`, the Slint testing dev-dependency).

#### 3b. Unbuilt claims the design still rests on

Four of round 5's thirteen repairs rest on **reading by their own author**, and
the ledger says so rather than letting five rounds of accumulated rigour imply
otherwise:

| repair | what it is | why it is a smaller bet than round 3's |
|---|---|---|
| **F-37**, the artifact map | ~155 lines: 111 paths, four manifests, six target names, one literal `#[path]`, `lib.rs` | a table of file paths fails **loudly** on the first `cargo build` |
| **F-38**, four thresholds | four numbers and the S-1…S-8 STOP table | wrong only if a measurement disagrees, and the measurement is the first thing PHASE-01 does |
| **F-8**, `code_of -> Cow<'_, str>` + `goad-boundary`'s public API | two signatures | the smallest surface a wrong repair can have |
| **F-17**, `SlintGlass` + the third stderr outlet | one module declaration | same |

None is a *behaviour* specified from summaries, which is what rounds 1–4 kept
finding. That is the honest case for them, and it is written down as a bet.

#### 3c. Standing assumptions

- **A-1, A-3** — standing; A-1 now has a local/stop line, A-3 is now a **stop**
  rather than a decision (F-38, S-2 and S-3). Both need `slint` in the graph.
- **A-2** — ~75 unproven lints against hand-written renderer code. Largely
  discharged by Thread 11. **Expectation budget unspent; three remain.** F-27's
  spend was refunded by F-29. The stop rule stands: the third distinct `expect`
  outside the generated-code quarantine stops the phase.
- **A-4** — see §2. The only assumption the first renderer commit is genuinely
  for.
- **A-5, A-6, A-7** — discharged. A-5 measured twice; `serve` is an `async fn`
  with no attribute. A-6's residual fallback deleted at F-34.
- **D25 is an admission, not a risk.** No instrument in the gate rejects a
  feature switched on by stratum 2 or 3 in a dependency shared with stratum 1.
  The design states the rule and states that nothing enforces it. That shape is
  deliberate — it is what F-6 was raised four times to get right.

---

### 4. Every decision taken under the autonomy grant

The grant is `design-log.md`, 2026-09-04, *"Gate autonomy: an explicit deviation
from `docs/AGENTS.md`"* — decide everything except canon, and record it as a
user decision would be recorded. Four vehicles were used and nothing was decided
outside them.

| vehicle | count | holds |
|---|---|---|
| `design-log.md` | **55** design decisions, each marked *Autonomy grant* with Asked / Decided / Why / Rejected / Consequence | the decisions themselves |
| `review-design.md` | **59** dispositions over five rounds — 40 findings, 19 of them re-dispositions — ids immutable, each with its reason | finding dispositions |
| `canon-delta.md` + `draft-policy.md` | **7** movements + 1 new policy, **drafted, not applied** | canon consequences |
| `notes.md` | 0 | phase-local decisions — there are none, because there are no phases |

`design-log.md` holds 61 entries. Six are **not** the agent's: three frame the
grant itself (*How the slice is driven*, *Gate autonomy*, *Scope extended*) and
three are the **user's own** and marked as such — the ADR-002 T1 split, the
tray-not-window empty state, and the devshell font. No review round has
challenged any of the three on its merits, and no finding has touched the
guiding principles.

The 55, by heading. `grep -n '^### ' docs/slices/002/design-log.md` gives line
numbers.

*The split and the gate* — the gate is six commands and `-p goad-semantics`
earns its place · the workspace invariant checks get their own member · members
are enumerated, not listed; R7 is retired · the renderer inherits the workspace
lint table unchanged · R3: four instruments for stratum 1, and the claim
narrowed to fit · R3: the new gate policy is drafted from the policy template.

*The state machine* — the presentation transition is a total function, and
cleanup is not one of its inputs · `(view: Some, failure: Some)` is unreachable
and is still written total · `Command::Choose` carries the view token · the
window has one derived surface value, and a new question outranks a record · one
consumption point for an `Outcome`, and it runs the mapper.

*The runtime seam* — shutdown leaves the command channel; `serve` is one
function both tiers call · the queue policy is four mechanisms, and only one is
the safety mechanism · a callback holds a `Wire`, and `busy` and `notice` are
two properties · four shutdown sources, one path; `dismissed()` is deleted ·
`serve` is a plain fn returning `impl Future` **(superseded)** · R3: the loop
was built, and it changed `serve`'s signature · R3: three seams closed, one
shape each.

*The glass* — the glass is one total method, and the component is never
recreated · Markdown is parsed once and the parse is retained · `ContentForm` is
two variants and no payload · the tray icon is a rule with no artefact · R3: the
icon has numbers and the startup surface has strings · R3: the Slint API is read
from the compiler, not inferred · R3: the markup was compiled, and the tray
could not be written.

*The diagnostic surface* — the display bound is applied last, and counted in
characters · two truncations, two statements · stderr alone reports without
raising fault; a renderer refusal does raise it.

*Startup and the clock* — the clock is a `fn` pointer returning `Result` ·
config discovery, exactly; and one startup exit code · the xdg app id is
`"goad"`, and the window rule lives beside the binary · R3: the entry point is
Rust, not a numbered list · R3: the slice document was wrong about the clock,
not the design.

*Validation* — the failure case table is written into the design, not delegated
· rows assert the rendered text, not the Rust variant · four rows read the
element tree, one per channel · where the exemptions, the refusals and the F-1
coda sit in the sequence · the failure table drives the `Host`, and item 11
drives the channel · the driving helpers are shared by one included file, cut at
the intersection · integration: one vocabulary, one pair type, one home for
`Refused` · AC-12 asserts what the host holds, not that the child is gone.

*Round 4 — what the compiler decided* — the three unbuilt passages were built,
and all three were wrong · the renderer crate is a library plus a thin binary
(D28) · `serve` carries no attribute, and A-2's budget is unspent · the escape
step is a `Display` adapter, and the outlet is not · `HOME` as given, XDG
absoluteness as one test, and the `argv[0]` skip · the failure table's instants,
its sentinel body, and a total channel partition · the house test standard
yields to `unnecessary_wraps`, on the standard's own terms.

*Round 5* — the count has one home, and every other document cites it · a
display failure goes to stderr, and the glass stays total · the artifact map,
and the two decisions writing it forced · every threshold is a number, and the
stops are one table · the font is `dejavu_fonts`, and `buildInputs` alone does
nothing · the window rule is validated KDL, not remembered KDL · the phase-gate
policy's scope is derived, clause by clause.

---

### 5. Is this converging? — the numbers, and the honest answer

The question a reader is owed after five rounds and no code: **is the design
converging, or is the review finding new work as fast as it closes old work?**

#### The raise rate

| round | new | reopened | total dispositions | new blockers | rested on built evidence |
|---|---|---|---|---|---|
| 1 | 12 (F-1…F-12) | — | 12 | 4 | 0 of 12 |
| 2 | 7 (F-13…F-19) | 7 | 14 | 1 | 0 of 14 |
| 3 | 9 (F-20…F-28) | 4 | 13 | 0 | 2 of 13 |
| 4 | 5 (F-29…F-33) | 2 | 7 | 2 | **7 of 7** |
| 5 | 7 (F-34…F-40) | 6 | 13 | 1 | 3 of 13 |
| **total** | **40** | **19** | **59** | **8** | **12 of 59** |

New findings per round: **12, 7, 9, 5, 7.** That is flat, not falling.
Re-dispositions per round: **0, 7, 4, 2, 6.** Also flat. A **blocker** was raised
at round 5 (F-37) that four earlier rounds did not see. Nineteen of fifty-nine
dispositions — **32%** — were reopenings of findings a previous round had
already marked `verified`. `F-9` has been disposed five separate times; `F-6`
four; `F-8`, `F-16` and `F-17` three each.

**On the raw counts the answer is: the review is finding new work about as fast
as it closes old work, and has been for four rounds. As a reading process it is
not terminating, and there is no number in the table that predicts round 6 would
be the last.**

#### The one series that does fall

| session | measurements taken | wrong |
|---|---|---|
| 1 (round 3) | 3 assumptions | 2 |
| 2 (round 4) | 3 passages built | 3 |
| 3 (round 5) | 3 artefacts built/validated | 1 |

Six of nine, then 1 of 3. **Building converges. Reading does not.** That is the
whole finding, and it is consistent with the other thing the record shows: round
4, the only round where every disposition rested on built evidence, is also the
only round whose raise count dropped.

#### Why the raise rate stayed flat — and why that is not an excuse

Each round asked a different question. Rounds 1–3 asked *is this right?*; round 4
asked *does this compile?*; round 5 asked *can this be typed?* Every new question
opened a fresh seam, which explains a flat rate without redeeming it — because
the corollary is that **there is no evidence the current question set is
complete.** A sixth question would plausibly find a sixth seam. That is precisely
why "run another round until it comes back clean" cannot be the exit criterion:
the criterion has no fixed point, and five rounds is the evidence.

#### What must change

**1. Stop reviewing by reading; execute instead.** The four unbuilt repairs
(§3b) are paths, two signatures and four numbers — the class that fails loudly at
first compile. Do not spend a session reading them. **Execute the artifact map in
a worktree.** `research.md`'s dry run put the split at about six minutes to a
green gate; `git reset --hard` reverses it; and performing it proves every path,
every manifest, every target name and the `#[path]` arithmetic at once. It is
also PHASE-01's actual work, so the cost is not a review round — it is the first
phase, done where a mistake is free.

**2. Replace the design gate's exit criterion.** Not *"no reviewer objects"* —
that has no fixed point here. Instead: **every claim in the design is either
built, or its failure mode is loud at first compile.** By that criterion the
design is met on everything except the four repairs in §3b, and step 1
discharges all four.

**3. Carry the canon debt; do not wait behind it.** CD-1…CD-7 and
`draft-policy.md` are real, they are the user's alone to endorse, and they are
endorsed **at audit** (`docs/AGENTS.md:38`). They are not a blocker on planning
or on execution, because `docs/AGENTS.md:36` makes the drafts the slice's
working authority for precisely this interval. Session 3 called the endorsement
*"the only genuine blocker"* and let three sessions pass behind it; the debt was
real and the blockage was not.

**The risk of the status quo, stated plainly.** An endless design gate is a
failure mode of the same family as a half-built tree. Five rounds, three
sessions, 5,049 lines of design and zero lines of code is what it looks like from
outside, and the review's own numbers do not promise a sixth round would end it.

---

### 6. What the next session does first

1. **Do not wait on canon.** §3a's movements are the one thing the autonomy grant
   withholds, and they are three sessions old — but they are put to the user **at
   audit** (`docs/AGENTS.md:38`), not before. Until then the drafts *are* this
   slice's working authority (`docs/AGENTS.md:36`): `plan.md` and every phase
   cite `canon-delta.md` and `draft-policy.md` exactly as they would cite canon,
   and keep them current as the work changes. CD-5 and `draft-policy.md` still
   go together or not at all — at audit.
2. **Execute the artifact map in a worktree**, per §5's recommendation — *not* a
   round 6 reading pass. Entry: `design.md` §5.1 *The artifact map*, top to
   bottom. Exit: `just check` at 0, and AC-2's content-change list matching the
   map's "change permitted" column. If it comes back green, the last unbuilt
   repair of consequence is built rather than read, and nothing stands between
   the slice and `plan.md`.
3. **Then `plan.md`.** PHASE-01 is the split, because it moves 111 files and
   nothing else should be moving at the same time. Two constraints the builds
   added to phase planning, both of which decide where a boundary can fall:
   `dead_code` is fatal under `-D warnings`, so the phase that lands
   `StartupError` must land a construction site for all eight variants in the
   same commit; and §5.4's *shapes* table plus §9's preamble are the two lists a
   phase reads before writing renderer code or a test target.
4. **The first thing PHASE-03 does after `slint` lands is A-4's timing
   protocol** — `plan.md` PHASE-03/EX-1, run before any renderer content. (This
   item said *PHASE-02* when it was written, before the phase numbering existed;
   PHASE-02 is the boundary rewrite and has no `slint` in it. Corrected
   2026-09-05, `review-plan.md` F-32.) That number decides whether ADR-002's T3
   has fired. Baseline measured 2026-09-05, pre-split: `just check` 1.836 s
   warm.
5. **Read §5.5's STOP table (S-1…S-8) into every phase sheet.** It is the list a
   phase agent needs in order to recognise a condition it is not allowed to
   improvise past.

**This Handover section predates `plan.md` and is superseded by it wherever the
two disagree.** It is kept as the record of what was known at the design gate;
`plan.md` is the executable authority for phase numbering, ordering and criteria.

**Reading list for whoever picks this up:** `docs/AGENTS.md`;
`docs/slices/002/slice-002.md` (15 acceptance criteria, Stage: design);
`design.md` §5.1 *The artifact map* (`:291–445`), §5.4, §5.5's STOP table, §9
item 12, §10 C-5; `review-design.md` Brief and the round-4 and round-5
syntheses; `design-log.md` from 2026-09-04; `research.md` Threads 7–11;
`canon-delta.md` and `draft-policy.md`.

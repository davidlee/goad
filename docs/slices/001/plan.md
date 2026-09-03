# Plan — Slice 001: Protocol core and process backend transport

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

<!-- Phase ids (PHASE-NN) and criterion ids (EN-/EX-/VT-/VA-/VH-N) are
     immutable: edits append, never renumber, so the sequence goes
     non-monotonic after a split and that is expected. Criterion ids are local
     to their phase — cite another phase's phase-qualified (PHASE-03/EX-2).
     Verification modes — VT: automated test. VA: agent check. VH: human
     acceptance.
     Progress is NOT recorded here. Status lives in `notes.md`. -->

## Overview

Ten phases take the repository from no Rust code to a library that completes an
`evaluate`/`respond` round trip against a real user-written backend, headless,
with every failure mode in `design.md` §5.5 reachable and asserted.

The spine is the stratum boundary. Phases 01–04 build stratum 1 — the pure
semantic core — and nothing in them may name tokio or touch a process. Phases
05–08 and 10 build stratum 2 on top of it. Phase 09 is the documentary and
consistency work the slice owes before audit, and it runs last.

**Execution order is 01, 02, 03, 04, 05, 06, 07, 08, 10, 09.** PHASE-10 was split
out of PHASE-08 after review; ids are immutable and edits append, so the sequence
is non-monotonic and that is expected.

Three things are established in PHASE-01 and hold for every phase after it:

1. **The build gate exists before the code it gates.** The `shell` feature, the
   two declared test targets and the six verification commands are the first
   thing that lands, so every subsequent phase is checked by them rather than
   retrofitted into them. This is the design's own lesson from F-51 — a claim is
   held by a mechanism or it is not held — applied to the plan's ordering.
2. **The two greps are tests from the start.** AC-11 (domain vocabulary) and
   AC-15's direction half (`src/semantics/` names no `crate::shell`,
   `crate::bin` or `tokio`) both fail vacuously against an empty tree, so both
   land with a guard that fails when they find nothing to inspect, and both run
   in every phase's `cargo test` from PHASE-01 onward.
3. **Both feature columns, every phase.** A phase is not green until all six
   commands pass. `design.md` §9 is explicit that a matrix checked in one column
   is unchecked, and the cost of discovering that at PHASE-08 is a rework of
   everything between.
4. **The no-panic lints follow the data, not the phase.** I9 and R-46 are held by
   `#![deny(...)]` on **every** module that handles backend-derived data — not
   on the first one to do so. Any phase creating such a module carries the
   obligation, and proves it the same way: add a temporary violation, confirm
   clippy fails, revert. Restriction lints are allow-by-default, so `-D warnings`
   never enabled them and a module that quietly lacks one looks identical to a
   module that has it and passes (F-62). Raised as review finding F-2 against
   this plan, whose first draft gave the obligation to PHASE-04 alone.

   **What is left per-module is `clippy::arithmetic_side_effects`.**
   `unwrap_used`, `expect_used` and `indexing_slicing` are crate-wide denies in
   `Cargo.toml`'s `[lints.clippy]`, with
   `clippy::allow_attributes_without_reason = "deny"` making a written reason the
   price of allowing one back — which is the drift argument D53 rested on,
   answered by a mechanism. `arithmetic_side_effects` stays per-module because
   crate-wide it fires on every loop counter. D53 was **amended to say this on
   2026-08-27** by user decision, and I9, D53, `design.md` §9 and
   `draft-spec.md` §7's R-46 row now all state the split form.

   **Crate-wide stops at the test targets.** `clippy.toml` sets
   `allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests` and
   `allow-indexing-slicing-in-tests`, because crate-wide otherwise fires on
   ordinary test code — measured, `design.md` §9. So a break-it-and-revert proof
   must break it in **host code**: an `unwrap()` under `tests/` passes and proves
   nothing (review finding F-14). `unwrap_in_result = "deny"` is not carved out
   and still reaches tests.

`design.md` §5.4 — the process transport — is split across two phases (05, 06)
rather than one. It has been restructured three times across review rounds 3, 4
and 5 and the last restructure has had no independent review (`notes.md`, and
`review-design.md`'s Synthesis, risk 1). Splitting it puts the working path and
the failing paths in separate sessions with separate exit criteria, and both
phases begin by re-running `transport-probe.local.rs` rather than trusting the
sketch. That is where the plan spends its verification budget.

## Sequencing & rationale

**Why stratum 1 first.** It has no dependencies pointing upward by construction,
so it can be finished and frozen. Building stratum 2 first would mean writing the
transport against types that do not exist yet and then editing it when they do.

**Why the errors land in PHASE-01.** `semantics/error.rs` is the one stratum 1
file with no dependency on any other, and the design fixes its every variant in
§5.2 — writing it is transcription, not speculation. It also gives the two grep
tests real files to inspect on the day they are written, which is what keeps
their vacuity guards honest rather than theoretical.

**Why canonical before wire.** Normalization's output type has to exist before
its input can be dispatched into it, and the checked constructors
(`Options`, `Fields`, `Alternatives`, `NumberRange`) are what several of
normalization's error paths *are*.

**Why schedule before normalize.** `normalize_response` discards an invalid
`next_check` into `Discarded::Schedule { raw, reason: ScheduleError }`, so the
parse half of `schedule.rs` is a dependency of normalization, not a peer.
PHASE-03 also introduces the fixture runner, because the scheduling corpus is
the smallest corpus that exercises it and PHASE-04's much larger one should
inherit a runner that already works rather than co-evolve with it.

**Why the transport splits at "works" versus "fails".** PHASE-05 owns the
structure and the paths a correct backend takes. PHASE-06 owns the bounds, the
cleanup dimension and the two grandchild cases — the material F-2, F-3, F-25,
F-43, F-48, F-53, F-59 and F-63 have all rewritten at least once. Each is a
session's work on its own; together they are not.

**Why Host comes after the transport and before the round trip.** `Host`
composes a transport that exists; and AC-8's rejection paths must be provable
against a backend that would fail if it ran, which needs `Host` in place before
PHASE-08 can assert "and the backend was not contacted".

**Why PHASE-08 splits, and why the id is 10.** The integration tier divides at the
same seam PHASE-05 and PHASE-06 divide at: a backend that works, versus backends
that fail. PHASE-08 keeps the round trip and the two example backends; **PHASE-10**
takes the protocol-level failure matrix and R-45's one-`Host` reuse. Phase ids are
immutable and edits append, so the new phase is 10 and the **execution order is
01…08, 10, 09** — PHASE-09 is the documentary close-out and stays last. Raised as
review finding F-6.

**Reordering.** PHASE-02 and PHASE-03 could swap only if the fixture runner moved
with the schedule work; nothing else in the sequence can be reordered without
breaking a dependency. PHASE-09 could in principle be folded into audit — it is
not, because AC-10 is a deliverable of this slice and audit is not the place to
first write one.

**Parallelism.** None of these phases have disjoint surfaces except 02 and 03,
and their dependency ordering removes even that. One agent, one phase, in
sequence.

## Decisions taken during planning

Recorded in `plan-log.md` with their reasoning. Both touch PHASE-01's manifest.

- **The TOML parser is `toml`, optional, inside the `shell` feature.**
  `design.md` §5.2 specifies TOML configuration and names no crate; §5.1's
  manifest lists serde, serde_json, jiff and tokio only. Config lives in
  `shell/config.rs`, so the parser is a stratum 2 dependency and is gated exactly
  as tokio is (D49). ADR-002's T1 does not fire, for D49's reason and by D49's
  mechanism. User decision, 2026-08-26.

- **No `[dev-dependencies]` entry for tokio, and none is needed.** Raised as a
  suspected gap in the build gate and **withdrawn after measurement**: a test
  target has the package's *regular* dependencies in scope, optional ones
  included, whenever the feature enabling them is on. So `tests/integration/`
  uses `#[tokio::test]` directly in the default column, while in the
  `--no-default-features` column `tests/protocol/` **cannot name tokio at all**
  — `error[E0433]: cannot find module or crate 'tokio'`. `design.md` §5.1 and §9
  and `canon-delta.md` CD-1 are correct as written, and plain `cargo tree
  --no-default-features` stays clean.

  The consequence is stronger than the design claimed and is worth carrying into
  PHASE-01: the stratum 1 tier cannot be tested *with* an async runtime even by
  accident. That is Cargo's resolution, not a grep and not review, so AC-15's
  boundary test does not need extending to `tests/protocol/`.

## Coverage

Every acceptance criterion in `slice-001.md`, mapped to the phase and criterion
that discharges it. A gap here is a gap in the plan.

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-01/EX-1 establishes both columns; every phase's VA-1 re-checks; PHASE-09/VA-1 is the final run |
| AC-2 | PHASE-02/EX-1 (types, version on the envelope), PHASE-04/EX-8 and VT-4 (unknown optional ignored, at all six inbound levels — `Alternative` included, per F-11), PHASE-04/EX-3 (unknown required primitive rejected) |
| AC-3 | PHASE-03/EX-1, VT-1 |
| AC-4 | PHASE-03/EX-2, VT-2 |
| AC-5 | PHASE-05/EX-1…EX-4, PHASE-06/EX-1…EX-5; the cancellation clause specifically by PHASE-06/VT-6 |
| AC-6 | PHASE-01/EX-2 (the stratum 1 taxonomy exists), PHASE-04/VT-2 (every `ProtocolError` reachable), PHASE-05/VT-3 and PHASE-06/VT-1…VT-4 (`BackendError`, `CleanupFailure` — at the transport), PHASE-07/VT-3 (`StateError`), PHASE-07/EX-8 and VT-6 (`Protocol(Json)` and R-38's framing, at the one place `from_slice` runs — moved from PHASE-05 by user decision 2026-09-02), PHASE-10/VT-1 (each **protocol-level** mode end to end) and PHASE-10/VT-3 (the transport and lifecycle modes as the `Outcome` a caller receives). The split is deliberate and was an overclaim before review finding F-10: PHASE-10 does not re-assert at the host what PHASE-05 and PHASE-06 assert at the transport, it asserts what reaches the caller. Its **no-panic** clause is the lint rule in Overview item 4, placed and proven by PHASE-04/EX-5, PHASE-05/EX-7 and PHASE-07/EX-7 |
| AC-7 | PHASE-08/EX-1, VT-1 |
| AC-8 | PHASE-07/EX-3, VT-3; PHASE-08/VT-2 adds the no-spawn assertion |
| AC-9 | PHASE-03/EX-3 (the runner and the scheduling corpus), PHASE-04/EX-4 (the protocol corpus, including the brief's verbatim examples) |
| AC-10 | PHASE-09/EX-1, VH-1 |
| AC-11 | PHASE-01/EX-4, VT-2 — a test from the first phase, re-run by every phase after |
| AC-12 | PHASE-08/EX-3, VT-3 |
| AC-13 | already satisfied by `draft-spec.md` as it stands; PHASE-09/VA-2 re-checks it against what shipped |
| AC-14 | **not in this plan.** It is discharged at audit and close, per `docs/AGENTS.md`, and needs explicit user endorsement |
| AC-15 | PHASE-01/EX-3 (dependency-graph half, the build gate) and EX-4 (direction half, the boundary test) |

---

## PHASE-01 — Crate skeleton, the build gate, and the stratum 1 error taxonomy

**Objective:** the crate exists, the six verification commands run green in both
feature columns, the two boundary greps are tests rather than intentions, and
every error type stratum 1 can raise is declared.

**Surfaces:** `Cargo.toml`, `clippy.toml`, `rustfmt.toml`, `justfile`,
`.gitignore`, `src/lib.rs`,
`src/semantics/mod.rs`, `src/semantics/error.rs`, `src/shell/mod.rs`,
`tests/protocol/main.rs`, `tests/protocol/boundary.rs`,
`tests/integration/main.rs`, `flake.nix`.

**Entry**
- EN-1 — the plan is accepted by the user (`docs/AGENTS.md` §Plan; `CLAUDE.md`,
  no code without an accepted plan).
- EN-2 — none. Both planning decisions above are settled; the manifest content
  they govern is written by this phase.

**Exit**
- EX-1 — the phase gate passes: **`just check` exits 0**. It runs `design.md`
  §9's six commands in §9's order — `cargo build`, `cargo test`, `cargo test
  --no-default-features`, `cargo clippy --all-targets -- -D warnings`, `cargo
  clippy --all-targets --no-default-features -- -D warnings -A dead_code
  -A unreachable_pub`, `cargo fmt --check`. `just` is the canonical runner (§9,
  user decision 2026-08-27) and is already in `flake.nix` `devToolPkgs`, so
  AC-1's clean-clone claim holds off this machine — verified in a fresh
  `nix develop` on 2026-08-27. Take the second clippy line from §9 rather than
  from memory: the two `-A`s are load-bearing and the reason they are on that
  line only is recorded there.
- EX-2 — `src/semantics/error.rs` declares `ProtocolError`, `BoundsError` and
  `ScheduleError` exactly as `design.md` §5.2 lists them, with `Display` and
  `std::error::Error`. No variant is added that the design does not name, and
  none is omitted — including `BoundsError::NotFinite`, which D39 keeps
  deliberately as a constructor guard despite being unreachable from JSON.
- EX-3 — the feature gate holds and is observed to, in three parts: `cargo tree
  --no-default-features` shows no tokio node; `cargo test --no-default-features`
  **skips** the `integration` target rather than failing to build it; and a
  `tokio` token added temporarily to a `src/semantics/` file **or** to
  `tests/protocol/` fails that column to compile with `E0433`. Observe the third
  by breaking it and reverting, not by asserting it.
- EX-4 — `tests/protocol/boundary.rs` carries both greps — AC-15's direction
  check and AC-11's domain-vocabulary check — each with a guard that fails when
  it inspects zero files.
- EX-5 — **already discharged, 2026-08-26**: the user added `pkgs.deno` to
  `flake.nix`. Note it went into `projectPkgs` rather than the `devToolPkgs`
  attribute `slice-001.md`'s Scope names; the effect is identical, since
  `projectPkgs` is `devToolPkgs` plus the GUI libs and the agents, and both the
  dev shell and the agent jails take `projectPkgs`. The jails now get deno too,
  which is what a jailed agent running the integration tier needs. This phase
  confirms it in the shell rather than making the change. `just` needs no change
  either: it has been in `devToolPkgs` since commit `6489521`, and both it and
  `deno` resolve to store paths in a fresh `nix develop` (checked 2026-08-27).
  A shell entered before either landed will not see them — hence VH-1.

- EX-6 — `toml` is declared in `[dependencies]` as `optional = true` and pulled
  in by the `shell` feature, exactly as tokio is, per the planning decision
  above. `cargo tree --no-default-features` shows neither it nor tokio. It lands
  here rather than at PHASE-07 because the manifest is this phase's business and
  because an entry criterion may not require work the entering phase owns —
  PHASE-07/EN-2 did, which is review finding F-7. It is an unused dependency for
  six phases and that costs nothing: `unused_crate_dependencies` is deliberately
  off in `[lints.rust]`, for this class of reason.
**Verification**
- VT-1 — `boundary.rs`: no file under `src/semantics/` contains `crate::shell`,
  `crate::bin` or `tokio`; plus a case pointing the same helper at an empty
  directory and asserting it **fails**. The vacuity guard is the point — without
  it a renamed directory turns the test green.
- VT-2 — `boundary.rs`: no file under `src/` contains any of habit, streak,
  journal, site, goal, reminder, compliance (AC-11, brief §21.16), with the same
  empty-directory guard.
- VT-3 — a case per `ScheduleError`, `BoundsError` and `ProtocolError` variant
  asserting its `Display` names the value it carries. Cheap, and it is what stops
  a variant being declared with a field nothing ever formats.
- VA-1 — run `just check` and paste the output into the phase sheet. Not "they
  should pass".
- VA-2 — confirm by reading `Cargo.toml` that `autotests = false` is present in
  `[package]`, that both `[[test]]` targets are declared with explicit paths, and
  that `integration` carries `required-features = ["shell"]`. `design.md` §5.1's
  manifest snippet states these in prose beneath the code block rather than
  inside it; all three are load-bearing.
- VA-3 — `just -n check` prints §9's six commands in §9's order: same commands,
  same arguments, same sequence. Compare the **command sequence**, not the
  characters — §9's block carries inline comments and wraps the second clippy
  line across two physical lines, and `just -n` prints neither, so a correct
  justfile fails a literal comparison (review finding F-9, against this
  criterion's own first wording). Paste both into the phase sheet. This is the
  whole of what "the justfile mirrors §9" means, and it is a command rather than
  a read (F-5).
- VH-1 — the user reloads the dev shell and confirms `deno --version` and
  `just --version` resolve to store paths. A `flake.nix` change does not reach a
  running shell on its own.

**Notes for the implementer**

- `rustfmt.toml` with `tab_spaces = 2`. Every Rust snippet in `design.md` is
  written at two spaces and `CLAUDE.md` asks for it; without the file
  `cargo fmt` will reformat to four and the design's sketches stop being
  copy-able. Settle it here, before there is code to churn.
- `.gitignore` **exists** (commit `4fc8637`) and needs no work. It carries
  `target/` and `*.local.*`. The `*.local.*` line is deliberately **not** left to
  the user's global ignore file, which also covers it: a rule that lives only in
  one machine's global config is not a property of the repository, and a clean
  clone elsewhere would track the probes and review packets. Widened from
  `*.local.md` on 2026-08-29; no tracked file matched, checked before the change.
- Cargo will not parse a `[[test]]` target whose `path` does not exist, so both
  `main.rs` files must be created in this phase even though they hold almost
  nothing. `tests/integration/main.rs` may be an empty module until PHASE-05.
- Do **not** write root `AGENTS.md` content here. AC-10 is PHASE-09's, and the
  verification commands it must name are the thing this phase is establishing.
- `edition` and the exact toolchain: `cargo 1.99.0-beta.1` from
  `rust-bin.beta.latest.default` (`research.md`, verified). AFIT is relied on by
  `design.md` §5.2 (D11), so do not pin backwards.
- The design's §5.1 manifest is the contract for dependency *names and features*,
  plus `toml` per the decision above. Do not add a `[dev-dependencies]` section:
  it is unnecessary (a test target already sees the package's optional
  dependencies when their feature is on) and an unconditional tokio entry there
  would put a runtime back in reach of the stratum 1 test target, which is the
  one thing this column exists to prevent.

---

## PHASE-02 — Canonical types and their checked constructors

**Objective:** every canonical type in `design.md` §5.2 exists, cannot be
constructed in an invalid state from outside `semantics::protocol`, and the two
request kinds serialize to the wire form the draft spec requires.

**Surfaces:** `src/semantics/mod.rs` (one line, `pub mod protocol;`),
`src/semantics/protocol/mod.rs`,
`src/semantics/protocol/canonical.rs`, `src/semantics/error.rs` (extend only if a
variant proves to need a field the design named and PHASE-01 missed).

This phase's tests are **colocated `#[cfg(test)]` modules** in the files above,
not a `tests/protocol/` target, and that is forced rather than preferred: under
D30 `Opt`, `Field` and `Alternative` have `pub(super)` fields and no public
constructor, so an external test crate cannot build the values VT-1 and VT-3
reject and accept. Making it able to would be R10. `tests/protocol/` remains
PHASE-03's surface.

**Entry**
- EN-1 — PHASE-01/EX-1 and EX-2 discharged.

**Exit**
- EX-1 — the scalar newtypes (`ViewId`, `OptionId`, `AlternativeId`, `FieldId`,
  `Timestamp`, `Hints`) — `ViewId` and `Timestamp` with a **public** constructor
  because the host authors both (PHASE-07 mints `view_id`, and stratum 2 reads
  the clock), the other three constructible only within `semantics::protocol`
  because a backend authors them and a caller obtains one by cloning it out of a
  canonical value — the response types (`Response`, `View`, `Choice`, `Opt`,
  `Content`, `Field`, `FieldKind`, `Alternative`), the checked collection
  newtypes (`Options`, `Fields`, `Alternatives`) and `NumberRange` all exist as
  §5.2 states them, with `pub(super)` fields and read-only accessors (D30).
- EX-2 — the outbound types (`Request`, `Evaluate`, `Respond`, `Event`,
  `UserResponse`) exist and serialize with `"protocol": 1` and a `"type"`
  discriminant of `evaluate` or `respond` (R-1, R-6).
- EX-3 — each checked constructor rejects what §5.5's edge-case table says it
  rejects: empty `Options` and `Alternatives`, duplicate ids within all three
  collections, and a `NumberRange` that is inverted or non-finite. **`Fields`
  checks uniqueness only and permits zero elements**, amended 2026-08-30: R-15
  says an option MAY carry fields and its verification row (`draft-spec.md:364`)
  asks for fixtures with and without; §5.5's table has no empty-fields row and
  the taxonomy no `EmptyFields`; and `Opt.fields` is a `Fields`, not an
  `Option<Fields>`, so an option with no fields is a `Fields` holding none. The
  earlier wording generalised `design.md:704`'s blanket comment over the three
  newtypes, which the F-52 paragraph beneath it does not support — F-52 argues
  duplicates for fields and never argues non-emptiness.
- EX-4 — `AlternativeId` and `OptionId` are distinct types and neither can be
  passed for the other (F-61, D52), and `DuplicateAlternativeId` /
  `EmptyAlternatives` are the errors raised for alternatives — never
  `DuplicateOptionId` / `EmptyOptions`.

**Verification**
- VT-1 — one case per rejection in EX-3, asserting the specific error variant and
  the path it carries.
- VT-2 — request serialization snapshots for both kinds (draft spec §7, R-1,
  R-6, R-7, R-8), asserted against literal expected JSON rather than a
  round trip. A round trip would pass with the version field missing.
- VT-3 — the same field id used by fields in **different** options is accepted.
  The negative case is what shows I15's scope is per-option and not per-view
  (draft spec §7, R-52).
- VA-1 — `just check`.
- VA-2 — grep `canonical.rs` for `pub ` on a struct field. There should be none
  outside the outbound request types, whose fields are host-authored and
  deliberately public. R10 in `design.md` §8 is the risk this check exists for.

**Notes for the implementer**

- The accessors are boilerplate and the temptation to widen the fields to `pub`
  is the named risk R10. D30 is the reason not to: outside
  `semantics::protocol`, a canonical value must be provably the output of
  normalization.
- `Timestamp` wraps `jiff::Timestamp` and jiff is `default-features = false`
  (D4). If something here wants a time zone, stop — that is stratum 1 acquiring
  I/O, and the answer is a parameter, not a dependency feature.
- Do not implement `Deserialize` for anything in this file. Canonical types are
  reached only through `normalize_response` (P1); a derive here would open a
  second door.
- `Content` has four variants and a v0 renderer implements none of them. That is
  P3 with brief §11.1 naming the future implementor — do not trim it to what
  slice 002 will draw.

---

## PHASE-03 — Schedule resolution, and the fixture runner

**Objective:** `next_check` parses to one canonical instant or to a named
`ScheduleError`, resolution is a pure function implementing latest-valid-wins,
and the protocol tier has a table-driven runner over data files that PHASE-04
can extend without changing.

**Surfaces:** `src/semantics/mod.rs` (one line, `pub mod schedule;`),
`src/semantics/schedule.rs`, `tests/protocol/main.rs`,
`tests/protocol/runner.rs`, `tests/protocol/fixtures/schedule/**`.

<!-- The parent `mod` file was added to this phase, to PHASE-04 and to PHASE-07
     by user decision 2026-09-02: a phase that adds a module edits its parent's
     `mod` list, and only PHASE-01 and PHASE-05 wrote that down. PHASE-02 had
     the same omission amended on 2026-08-29; this closes the class rather than
     the third instance. `plan-log.md`. -->

**Entry**
- EN-1 — PHASE-02/EX-1 discharged (`Timestamp` exists).

**Exit**
- EX-1 — parsing accepts an RFC 3339 instant with an offset and a jiff friendly
  span, and rejects with the right variant: `NotAString`, `MissingOffset`,
  `CalendarUnit`, `OutOfRange`, `Unparseable` (AC-3, R-21…R-25).
- EX-2 — resolution is a pure function over (retained, incoming, default, now)
  returning a concrete `Timestamp`: the latest **valid** instruction, else the
  retained value, else `now + default_poll`. An invalid instruction preserves
  rather than disables (AC-4, R-26, R-27).
- EX-3 — the runner walks data files, and its fixture format is documented in
  `notes.md` so PHASE-04 inherits it rather than inventing a second one.
- EX-4 — a past instant, including a negative span, is stored as given and not
  clamped (R-28, D29, F-13).

**Verification**
- VT-1 — the scheduling corpus: absolute with and without offset; spans in
  minutes, hours, days and weeks; `"1 month"`; an out-of-range span; `45` (a
  number, not a string); `"tomorrow morning"`. Each asserts its own variant.
- VT-2 — resolution over the triple, including latest-valid-wins and
  invalid-preserves-existing.
- VT-3 — `"1 day"`, `"1 week"` and `"1d 2h"` resolve to exactly 24h, 168h and 26h
  (F-10, D28).
- VA-1 — `just check`.
- VA-2 — **re-run** the jiff behaviour rather than trusting `notes.md`:
  `SpanRelativeTo::days_are_24_hours()` resolving days and weeks and rejecting
  months and years, with no tzdb present. `notes.md` records it as established at
  jiff 0.2.35 and instructs re-running where a phase depends on it. This phase
  depends on it entirely.

**Notes for the implementer**

- `now` is a parameter everywhere. Stratum 1 reads no clock (I3), and this is the
  file where that is most tempting.
- The fixture format is this phase's most consequential decision because PHASE-04
  inherits it for a much larger corpus. Design it for a reader who knows the
  protocol and not the tests — `design.md` §9 says the corpus must be reviewable
  as protocol documentation, which is what makes it usable to the draft spec.
- `ScheduleError` variants already exist from PHASE-01. If one needs a field the
  design did not give it, that is a design question, not a local repair.

---

## PHASE-04 — Wire types, normalization, and the protocol corpus

**Objective:** a permissive wire response normalizes to a canonical one — total,
with a discard list — and every `ProtocolError` the design names is reachable
from a fixture.

**Surfaces:** `src/semantics/protocol/mod.rs` (the two `pub mod` lines, and
its doc comment, which says `wire` arrives at PHASE-03 and does not),
`src/semantics/protocol/wire.rs`, `src/semantics/protocol/normalize.rs`,
`tests/protocol/runner.rs`, `tests/protocol/main.rs`,
`tests/protocol/fixtures/**`, and `src/semantics/protocol/canonical.rs` —
**for the removal of four lint attributes and nothing else**.

<!-- `canonical.rs` was added by user decision 2026-09-02, scoped in the line
     above. PHASE-02 put `#[cfg_attr(not(test), expect(dead_code, …))]` on
     `OptionId::new`, `AlternativeId::new`, `FieldId::new` and `Hints::new`, and
     each reason text says the attribute comes off once PHASE-04 calls it —
     `unfulfilled_lint_expectations` then fails the gate until it does. The
     tuple fields are private, so `new` is the only construction path and the
     phase cannot complete without removing all four. Neither a constructor nor
     a widened field, so R10 is untouched, and the STOP condition in the phase
     sheet stands unchanged. `plan-log.md`. -->

<!-- `runner.rs` and `main.rs` were added by user decision 2026-09-02: a
     fixture file asserts nothing on its own, so the corpus needs a checker, a
     `Corpus` const and a `#[test]`, and a new file under `tests/protocol/`
     needs `main.rs` to declare it. Whether the per-corpus halves split out of
     `runner.rs` is left to execution — `notes.md`, *The fixture format*.
     Same class as the parent-`mod` omission closed the same day.
     `plan-log.md`. -->

**Entry**
- EN-1 — PHASE-03/EX-1 and EX-3 discharged.

**Exit**
- EX-1 — the wire types are as §5.2 states: no `deny_unknown_fields` anywhere
  inbound (I10, R-4, R-5); `view` present-preserving via the `present` helper so
  omission and explicit `null` stay distinct (D25, F-5); `next_check` and `body`
  typed `serde_json::Value` (D6, D38); `hints` `#[serde(flatten)]` (D37, F-38).
- EX-2 — `normalize_response(wire, now) -> Result<Normalized<Response>,
  ProtocolError>` with `Normalized { value, discarded }` and `Discarded` a closed
  enum of one variant (D10).
- EX-3 — an unrecognised `kind` at **any** of the three discriminant sites — the
  view, a field, a content block — yields `UnsupportedPrimitive { kind, at }`
  carrying the path (D8, F-6, R-12).
- EX-4 — the corpus covers `design.md` §9's protocol-tier list, and includes
  brief §10.1's bare-string `body` and §10.2's flat `multiline` **verbatim**
  (AC-9, F-31, F-38).
- EX-5 — `wire.rs` and `normalize.rs` carry
  `#![deny(clippy::arithmetic_side_effects)]` per Overview item 4 (I9, F-62,
  R-46). The other three restriction lints are crate-wide and need no per-module
  attribute. This phase is the first to handle backend-derived data; it is
  **not** the only one, and PHASE-05 and PHASE-07 carry the same obligation for
  the modules they add.
- EX-6 — `InapplicableKey` is raised for a modelled key its kind does not admit,
  and a `choice` field's option carrying `fields` is **rejected** rather than
  ignored (D43, D46, F-45, F-55, R-50, R-53).
- EX-7 — `null` behaves per D50 and R-51: identical to omission everywhere except
  `view`, reported as nothing, with `"next_check": 45` still discarded and
  reported so the two are shown to be distinguished.
- EX-8 — unmodelled fields are ignored **at every inbound level**, not merely
  permitted by the absence of `deny_unknown_fields`: the envelope, the view, an
  option, a field, a content block **and an `Alternative`** — a `choice` field's
  options — each accept one and normalize unchanged (AC-2, R-4, R-5, R-53).
  `Alternative` is a level because D52/F-61 made it one: an alternative is not an
  option and does not share its type. The first repair enumerated the five levels
  that predated D52 (review finding F-11). Raised as review finding F-4 — the first draft mapped AC-2 to
  a structural serde condition, which is not the behaviour R-5's own §7 row asks
  to be verified.

**Verification**
- VT-1 — the corpus, table-driven through PHASE-03's runner.
- VT-2 — a case constructing every `ProtocolError` variant from a fixture, with
  exactly one documented exception: `BoundsError::NotFinite`, which JSON cannot
  express, and whose fixture asserts `Protocol(Json)` instead (F-36, D39). Write
  the exception down in the test, not only in the phase sheet.
- VT-3 — the misspelling pair: an optional key (`minn`) becomes a hint; a
  required key (`labell`) is rejected. Both are D37's stated cost, and asserting
  them is what keeps the cost the size the design claimed.
- VT-4 — one fixture per level in EX-8 — six, `Alternative` included — each
  carrying an unmodelled field and
  asserting the message is accepted, the field is absent from the canonical
  value, and **nothing is discarded** — the assertion is the silence, since a
  discard here would be the defect (R-4, R-5, and R-51's §7 row for the shape of
  the assertion).
- VA-1 — `just check`. Note that clippy's second column is what proves EX-5's
  lints are on: they are restriction lints and `-D warnings` never enabled them.
- VA-2 — verify EX-5 by breaking it, in **both** forms: an `unwrap()` in
  `src/semantics/protocol/normalize.rs` (the crate-wide deny) and an unchecked `+` in the
  same file (the per-module one), confirming clippy fails on each, then
  reverting. **Not "anywhere"** — `clippy.toml` carves the no-panic lints out of
  both test targets, so an `unwrap()` in `tests/` is expected to pass and would
  disprove nothing (review finding F-14). Break it in host code. An
  allow-by-default lint that was never switched on is indistinguishable from one
  that is switched on and passing (F-62).

**Notes for the implementer**

- `#[serde(flatten)]` alongside declared optional fields is the shape D37 chose
  and F-45 then constrained: serde binds `min`, `max` and `options` before `kind`
  is dispatched, so they cannot fall through to `hints` and an inapplicable one
  must be raised rather than lost. Get EX-6 red first.
- The `at` path accumulation is explicitly left to implementation
  (`design.md` §6). One encoding is offered there and was run — read the
  discriminant with a `#[serde(flatten)] rest: Value` struct and dispatch — but
  it is offered, not mandated. Whatever you choose, the contract is the named
  error, the retained string and the path.
- The corpus is a deliverable in its own right (AC-9) and it is what the draft
  spec's §7 rows point at. Name the files for the requirement they verify.

---

## PHASE-05 — Process transport: the structure and the paths that work

**Objective:** `Backend::exchange` exists and the §5.4 structure is implemented
as written, with a correct backend completing an exchange and a non-zero exit
discarding the body it came with.

**Surfaces:** `src/shell/mod.rs`, `src/shell/error.rs`,
`src/shell/backend/mod.rs`, `src/shell/backend/transport.rs`,
`src/shell/backend/process.rs`, `tests/integration/**`, `tests/backends/*.sh`,
`tests/protocol/transport_shape.rs`, `tests/protocol/main.rs`.

<!-- The two `tests/protocol/` paths were added by user decision 2026-09-02,
     from PHASE-05's expansion: VT-5's source-text checks belong to the boundary
     tier, and neither the file that holds them nor the `main.rs` that declares
     it was named. Fifth instance of a phase's Surfaces naming what it adds and
     not the file that reaches it; `plan-log.md` carries the argument.

     `tests/integration/**` replaced `main.rs` and `harness.rs` by user decision
     2026-09-03, raised at the end of PHASE-05's execution: the cases themselves
     live in `tests/integration/transport.rs` and nothing declared it. The glob
     is the form PHASE-06's Surfaces already use, so this makes one convention
     of two rather than widening anything. Sixth instance, fixed as a class. -->

**Entry**
- EN-1 — PHASE-02/EX-1 discharged (`Request` exists to be serialized).

<!-- The probe run was EN-2 in the first draft. Review finding F-1: an entry
     criterion means "the previous phase is not done" if unmet, and no previous
     phase owes this. It is this phase's own first task, and EX-6 is where it
     is discharged. -->

**Exit**
- EX-1 — `trait Backend { fn exchange(&mut self, request: &Request) -> impl
  Future<Output = Exchange> + Send; }` with `Exchange { result, stderr, cleanup }`
  and no outer `Result` (D22, D33, D40), and `Captured { bytes, truncated }`.
- EX-2 — `process.rs` implements §5.4's sketch: spawn with `kill_on_drop(true)`;
  write stdin and **drop the handle**; drain stdout and stderr concurrently via a
  `select!` sub-future, never a `tokio::spawn` (D44, F-49); observe the exit
  status **inside** `config.timeout` (D51, F-59); one `CLEANUP_LIMIT` budget
  covering kill, reap and drain completion (D48).
- EX-3 — a backend that reads stdin to EOF completes an exchange (R-37), and one
  that writes a valid response and exits **non-zero** yields
  `ExitStatus { code: Some(1) }` with the body discarded and the stderr kept
  (D15, R-40, F-59).
- EX-4 — stderr is carried on every path this phase reaches, including a timeout
  and a zero exit with unparseable stdout (R-42, F-24).
- EX-5 — there is no `?` between the spawn and the cleanup budget (F-41). The
  only return that skips cleanup is the spawn failure itself, where no child
  exists.
- EX-6 — `transport-probe.local.rs` has been run **before any of `process.rs` was
  written**, and its output is recorded in the phase sheet. Copy it and
  `transport-probe-Cargo.local.toml` into a scratch crate and `cargo run`; its
  seven cases are the fastest check on §5.4 and five of them changed the design.
  This is the phase's first task, not a gate on the previous one.
- EX-7 — `process.rs` carries `#![deny(clippy::arithmetic_side_effects)]` per
  Overview item 4. It is the module that reads a backend's bytes and computes
  over their lengths against `STDOUT_LIMIT` and `STDERR_LIMIT`, which is exactly
  the arithmetic the lint exists for; if any module in this crate needs it, it
  does (review finding F-2).

**Verification**
- VT-1 — a normal exchange against a bash backend: request in, response out,
  exit 0, `cleanup: None`.
- VT-2 — a backend sleeping past the timeout: `BackendError::Timeout`, and the
  process confirmed gone afterwards (R-41).
- VT-3 — one case per `BackendError` variant this phase can reach: `Spawn` (a
  path that does not exist — no fixture needed), `Timeout`, `ExitStatus`, and
  `Io`.

  <!-- `Protocol(Json)` was in this list and is not reachable here: EX-1 fixes
       `Exchange.result` as `Result<Vec<u8>, _>`, so the transport parses
       nothing. It moves to PHASE-07/EX-8, where `from_slice` runs, and R-38's
       framing goes with it. User decision 2026-09-02. The backend script for
       a zero exit with unparseable stdout stays here, because EX-4's claim
       about that case is that the **stderr** survives. -->
- VT-4 — a backend that writes to stderr and **then** sleeps past the timeout,
  asserting the stderr survives (F-3). This is the case D18's reversal exists
  for; it is not covered by VT-2.
- VA-1 — `just check`.
- VT-5 — three of the four §5.4 regressions are greps and are asserted as a test
  over `process.rs`'s source text, in the same tier and with the same
  found-no-files guard as PHASE-01's boundary checks: **no task is spawned**, no
  `Arc`/`Mutex`, and no `?` between the spawn and the cleanup budget. Each was a
  repair in the design review (F-49, F-36's lock deletion at D44, F-41), so each
  is a regression with a name. Review finding F-5.

  **In `tests/protocol/transport_shape.rs`, not in `boundary.rs`** — user
  decision 2026-09-02. `Scan` is a forbidden-token walk over a *directory*, and
  only the `Arc`/`Mutex` check is that shape: the first constrains an occurrence
  *shape* and the third a *region*. Generalising `Scan` to carry a per-line
  predicate and a region state would rework PHASE-01's surface to serve an
  unrelated question, against that file's own instruction to extend the
  configuration and not the walk. The guard is kept in spirit and changes form
  with the subject: here it is that the file was found and read.

  The first check greps the token `spawn` and permits exactly one occurrence
  shape, `Command::spawn` — the child. `tokio::spawn` alone would let
  `Handle::spawn`, `spawn_blocking`, `spawn_local` and `JoinSet::spawn` through,
  and F-49's leak needs only one of them (review finding F-12). PHASE-06/VT-6
  re-asserts this against the finished module.
- VA-2 — the fourth is genuinely a read and stays one: `child.wait()` sits
  **inside** the timed region rather than in the cleanup budget (F-59, D51). No
  string distinguishes the two placements; the scope structure does.
- VA-3 — assert the elapsed time on the timeout path is bounded by
  `config.timeout + CLEANUP_LIMIT` and, on a prompt success, by neither (R-41).
  The probe measured 2.5 ms for the success path; if this phase's success path
  costs the cleanup budget, the structure is wrong.

**Notes for the implementer**

- This is the phase the design is least sure of. §5.4 is its fourth structure and
  the last restructure has had no independent review. Where the sketch and a
  measurement disagree, the measurement wins and the disagreement is a finding —
  take it back to design rather than repairing it here.
- The harness is this phase's other deliverable and PHASE-06 and PHASE-08 both
  inherit it. It needs: locating `tests/backends/` from the test binary,
  constructing the transport against one, and running an async body from a
  `#[test]`. Keep the backend scripts declarative — one behaviour each, named
  for it.
- **No `Config` in this phase** — user decision 2026-09-02. `config.rs` is
  PHASE-07's surface and PHASE-07/EX-1 owns the type whole, loading and
  rejection rules included. The transport holds `command: Vec<String>` and
  `timeout: Duration` directly, which is what §5.4 needs anyway: `exchange`
  takes only `&mut self` and the request, so the timeout is already the
  transport's. PHASE-07 constructs one *from* a loaded `Config`, which is the
  direction "durations resolve at load" (`design.md:1152`) implies.
- A backend script needs no shebang: `command` is an argv vector and the design's
  own AC-12 case is `["bash", "./backend.sh"]` (R-36).
- Do not reach for `wait_with_output()`. §5.4 says why at length, twice, and it
  reads like a simplification.
- Run the probe first (EX-6). Not after writing `process.rs` to check it, and not
  instead of the tests — before, so the structure you write is the structure that
  was observed rather than the one that was read.

---

## PHASE-06 — Process transport: bounds, disposal, and the two grandchild cases

**Objective:** every bound in AC-5 behaves as the design distinguishes them, the
cleanup dimension is reachable and reported independently of the exchange
result, and both grandchild cases are observed rather than described.

**Surfaces:** `src/shell/backend/process.rs`, `src/shell/error.rs`,
`tests/integration/**`, `tests/backends/*.sh`,
`tests/protocol/transport_shape.rs`.

**Entry**
- EN-1 — **PHASE-05 discharged** — every exit criterion it carries, not a
  range. Ranges go stale the moment a phase gains a criterion, which is how
  PHASE-07 and PHASE-08 came to cite ones that no longer covered their
  predecessor (review finding F-7).

<!-- The conditional probe re-run was EN-2 in the first draft; review finding F-1
     applies to it as it does to PHASE-05's. It is EX-6 below. -->

**Exit**
- EX-1 — the stdout bound **fails** the exchange with `OutputTooLarge`, the
  reader drops the handle, and the backend observes the broken pipe (D27, D34,
  R-43).
- EX-2 — the stderr bound **truncates and keeps draining**: a backend flooding
  stderr past 256 KiB and then answering correctly **succeeds**, with
  `truncated` set (D34, F-25, F-43). This is the case that deadlocks if the two
  readers are collapsed into one.
- EX-3 — `CleanupFailure` is reported on `Exchange::cleanup`, independent of
  `result`, and all four combinations in §5.4's table are distinguishable
  (D47, R-54).
- EX-4 — both grandchild cases are asserted, and asserted **differently** (F-63):
  a grandchild holding stderr only gives `Ok(response)` with
  `cleanup: TimedOut`; a grandchild holding stdout as well gives `Err(Timeout)`
  **and** `cleanup: TimedOut`.
- EX-5 — after every misbehaving case, no child of the test process remains
  (R-48). A cancelled exchange asserts the narrow claim AC-5 makes: nothing the
  **host** holds survives it. It does not assert the child is gone (D54, F-60).
- EX-6 — if `process.rs` changed at all in this phase, the probe was re-run and
  its output recorded in the phase sheet, as PHASE-05/EX-6.

**Verification**
- VT-1 — the stdout flood, asserting `OutputTooLarge` and the backend's own
  observation of the closed pipe.
- VT-2 — the stderr flood past the pipe buffer **and** past the bound, asserting
  success, `truncated`, and no deadlock. Run it with the body still reading
  stdout — the deadlock this exists to prevent needs both.
- VT-3 — the two grandchild cases, each asserting both dimensions and the elapsed
  time (the probe measured 303 ms and 902 ms against a 900 ms bound).
- VT-4 — **disposal that cannot complete within the budget**: the cleanup budget
  elapses, `CleanupFailure::TimedOut` is reported, and the exchange **returns**,
  within `timeout + CLEANUP_LIMIT` plus stated slack. The elapsed bound is the
  content — a host that blocks is the host going down.

<!-- Amended by user decision 2026-09-03, from PHASE-06's expansion. The first
     wording asked for "a backend wedged so `wait` cannot return promptly", and
     no test can build one: `dispose` is `start_kill` then `wait`, `start_kill`
     sends `SIGKILL`, and only uninterruptible kernel sleep defers `SIGKILL`.
     Every case that does make the budget elapse stalls on the **drain**, not on
     `wait`. §5.5's row at `design.md:1729` keeps its wording and goes to audit
     as a case the design describes and no test can arrange. -->

- VT-5 — no-orphans, in **two** parts:
  - **per case** — every misbehaving case asserts that the child it spawned is
    gone, which is EX-5's own wording and the pattern PHASE-05/VT-2 established.
  - **aggregate** — a check that **settles**: poll the test process's children,
    filtered to those whose command line names `tests/backends/`, until none
    remains or a deadline passes.

<!-- Amended by user decision 2026-09-03, from PHASE-06's expansion. The first
     wording — "after the whole misbehaving suite, the test process has no
     children" — is unsound under `cargo test`: libtest runs a target's tests as
     threads of one process, so a global instantaneous check sees the children of
     every case running concurrently and fails on their work. Settling rather
     than sampling keeps the claim that catches a leaked child no test knows
     about, which is the one R-48 is about. A separate test target would also
     have worked and needs `Cargo.toml`, which is not in this phase's
     Surfaces. -->
- VT-6 — the cancellation claim, in two mechanisms, because EX-5 had none in the
  first draft (review finding F-3):
  - **structural** — PHASE-05/VT-5's spawn grep already fails if the transport
    acquires a task. Assert it here too against the finished module, so the
    claim and its mechanism sit in the same phase as the criterion. The grep is
    over the token `spawn`, with `Command::spawn` the only permitted occurrence —
    not over `tokio::spawn` alone, which leaves `Handle::spawn`, `spawn_blocking`,
    `spawn_local` and `JoinSet::spawn` through (review finding F-12).
  - **behavioural** — the exchange must be **driven far enough to have started
    work** before it is dropped, or the assertion is vacuous: Rust futures are
    lazy, and a future dropped before its first poll leaves the count at zero
    however the transport is written (review finding F-12). So: spawn the
    exchange as its own task against a backend that will not answer, wait until
    the count is **≥ 1** — which is the positive control, proving the metric is
    live and would see a leak — then abort that task, let the runtime settle, and
    assert `handle.metrics().num_alive_tasks() == 0`. It asserts nothing about
    the child, which is what D54 concedes and AC-5 states.

    The first repair asserted `shutdown_timeout(Duration::ZERO)` returns
    promptly, and **that assertion could not fail** (review finding F-8):
    `shutdown_timeout` bounds the wait for *blocking* tasks, while async tasks
    are aborted at shutdown and never delay it. Measured on this toolchain — with
    a live detached async task it returned in 271 µs, with nothing spawned in
    144 µs. `num_alive_tasks()` on the same probe read **1** and **0**, needs no
    `tokio_unstable`, and is therefore the mechanism. The test must own the
    runtime it measures, since the count is per-runtime.
- VA-1 — `just check`.
- VA-2 — re-read `design.md` §5.5's edge-case table and confirm every row this
  phase owns has a test, and that the test asserts what the row says rather than
  something adjacent. F-63 was exactly a test that measured one case while the
  prose described another.

**Notes for the implementer**

- The asymmetry between the two readers is the whole of D34: "truncate" means
  stop **storing**, never stop **reading**. If a refactor makes them one function
  with a flag, EX-2 will hang rather than fail, which is the worst available
  symptom.
- `CleanupFailure::TimedOut` is deliberately not named `Orphaned` (F-48, F-63):
  in the case that actually fires, the child has exited and been reaped. Do not
  rename it to something that reads better and asserts more.
- The cancellation assertion is structural, not timed. There is no task to
  outlive the exchange because the transport spawns none — no `tokio::spawn`, and
  none of the other spawn APIs VT-5's grep covers. If asserting it needs a sleep,
  the structure has regressed.
- **`read_capped` owns the stdout handle**, so it drops where the bound is hit.
  Measured at expansion 2026-09-03: as PHASE-05 shipped it the reader *borrowed*,
  which drops the handle when the exchange **returns** — 500 ms later on a case
  whose disposal stalls, and observable from the backend. `design.md:1520` and
  `:1528`, R-43 and `process.rs`'s own doc comment all state the close happens at
  the bound, and EX-1 requires it; user decision 2026-09-03 repairs the code
  rather than narrowing the criterion. Ownership is the mechanism, so a
  source-text check over the signature is the regression guard — a timing
  assertion would be a race.

---

## PHASE-07 — Config, host state, and composition

**Objective:** `Host` composes the transport, normalization, schedule resolution
and interaction state; configuration loads from TOML; a stale or unknown
`view_id` is rejected before the backend is reached.

**Surfaces:** `src/shell/mod.rs` (three `pub mod` lines), `src/shell/config.rs`,
`src/shell/state.rs`, `src/shell/host.rs`,
`src/shell/error.rs` (add `StateError`), `tests/integration/**`. **Not**
`Cargo.toml` — `toml` is declared at PHASE-01/EX-6 (F-7).

**Entry**
- EN-1 — **PHASE-06 discharged**, EX-6 included — the probe re-run the F-1
  repair added. Stated as the phase and not as a range (review finding F-7).
- EN-2 — PHASE-01/EX-6 discharged: `toml` is already declared under the `shell`
  feature and `cargo tree --no-default-features` shows neither it nor tokio.
  This phase *uses* the parser; it does not add it (F-7).

**Exit**
- EX-1 — `Config` loads brief §5's three values and rejects at load: `command =
  []`, `timeout = "0s"`, `default_poll = "0s"`. Durations parse with jiff — one
  duration grammar across the product.
- EX-2 — `State` holds `Option<Outstanding>`, a non-`Option` `resolved_check` and
  a sequence counter; `view_id` is `{now RFC 3339}#{seq}` (D13); `Host::new`
  seeds `resolved_check` from `now + default_poll` (I4).
- EX-3 — `StateError::NoOutstandingView` and `StaleViewId` are distinct variants,
  raised before the transport is touched, and a rejection leaves the outstanding
  interaction intact (AC-8, R-32, R-34, D24).
- EX-4 — `Outcome` is a struct carrying `view: Option<Presented>`, a concrete
  `next_check`, `discarded`, `stderr`, `failure` and `cleanup` — with `Presented`
  pairing the view and its id inseparably (D23, D32, F-23, I12, I14).
- EX-5 — a failed exchange leaves `resolved_check` exactly as it was (R-29, P2).
- EX-6 — `view: null` answering an `evaluate` leaves an outstanding interaction
  alone; answering an accepted `respond` closes it (F-46, §5.5's edge table).
- EX-7 — `host.rs` carries `#![deny(clippy::arithmetic_side_effects)]` per
  Overview item 4: it is where a backend's bytes are parsed, normalized and
  composed into an `Outcome`, so it handles backend-derived data throughout
  (review finding F-2). `config.rs` and `state.rs` do not — a config file is the
  user's own and a `view_id` is host-minted — and the rule is about the data, not
  the directory. `State::next_seq` increments, so if that counter moves into a
  module carrying the lint it needs a checked add rather than an `#[expect]`.
- EX-8 — `Host` reads the transport's bytes with `serde_json::from_slice`, and
  that is where `BackendError::Protocol` arises and where R-38's framing rule is
  enforced: a body that is not **exactly one** JSON document is a `Json` error
  and reaches normalization not at all. Three cases — empty stdout, trailing
  content after one document, and invalid UTF-8, the last per `design.md:1052`,
  which is why `Exchange.result` carries `Vec<u8>` rather than `String`. Moved
  here from PHASE-05/VT-3 by user decision 2026-09-02: the transport returns
  bytes and parses nothing, so the variant was unreachable in the phase that
  claimed it.

**Verification**
- VT-1 — host-level tests against a **fake** `Backend`, not a process: the
  generic `Host<B>` exists so this is cheap, and it keeps these cases out of the
  slow tier.
- VT-2 — config rejection cases, one per EX-1 clause, each naming its error.
- VT-3 — stale and unknown `view_id`, asserting the variant and that the
  outstanding interaction survives.
- VT-4 — schedule unchanged across a timeout, a non-zero exit and a
  malformed-JSON exchange (R-29).
- VT-5 — `view_id` determinism: a fixed `now` and counter produce the exact
  documented id (D13's third reason).
- VT-6 — EX-8's three framing cases, against the **fake** backend rather than a
  process: handing `Host` a byte string needs no spawn, and the fake is already
  VT-1's vehicle. Each asserts `Protocol(Json)` and that the schedule did not
  move (R-29, and EX-5's rule applied to this failure).
- VA-1 — `just check`.

**Notes for the implementer**

- `State` is a plain struct behind `&mut self`. No `Arc`, no `Mutex` — D14, and
  the same argument F-49 eventually applied to the drain. Brief §12 gives this
  host no concurrency to protect against.
- The fake backend belongs in the integration tier because `Host` is stratum 2.
  It is still the right vehicle for EX-3, EX-5 and EX-6 — the process transport
  adds nothing to those and costs a spawn each.
- `Outcome::failure` says the **host** took no action. It is not a claim about
  the backend's side effects (F-32, R-49). Whatever documentation this phase
  writes on that field must say so.

---

## PHASE-08 — The round trip and the example backends

**Objective:** a backend written in TypeScript and a backend written in bash both
complete a full `evaluate` → view → `respond` round trip against the real
process transport.

<!-- Split per review finding F-6: this phase also owned the end-to-end failure
     matrix and R-45's one-`Host` reuse, which is not one session's work. Those
     are PHASE-10, which runs immediately after this phase and before PHASE-09.
     EX-4 and EX-5 below are struck and re-stated there. PHASE-10's own scope was
     later narrowed at F-10 — it owns the protocol-level matrix plus the
     caller-visible `Outcome` for the transport and lifecycle modes, not a
     re-assertion of everything PHASE-05 and PHASE-06 hold. -->

**Surfaces:** `examples/typescript/**`, `tests/backends/**`,
`tests/integration/**`.

**Entry**
- EN-1 — **PHASE-07 discharged**, EX-7 included — the `host.rs` lint attribute
  the F-2 repair added. Stated as the phase and not as a range (F-7).
- EN-2 — `deno` available in the dev shell (PHASE-01/EX-5, VH-1).

**Exit**
- EX-1 — the AC-7 round trip: the example backend returns `view: null`; then
  returns a choice; the host mints a `view_id` and hands it to the caller inside
  `Presented`; a `respond` carrying that id reaches the backend; the reply is
  accepted (F-23).
- EX-2 — `examples/typescript/` is a minimal showcase backend run as
  `["deno", "run", "-A", …]`, with no build step and no `node_modules`. Its
  comments must **not** present deno's default-deny permissions as a security
  boundary — brief §14 makes backends trusted user programs, and `-A` is there
  for that reason (OQ-9's answer).
- EX-3 — at least one backend the suite exercises is **not** TypeScript: a bash
  script invoked as `["bash", "./backend.sh"]`, with no shebang (AC-12, R-36).
- ~~EX-4~~ — moved to PHASE-10/EX-1. F-6.
- ~~EX-5~~ — moved to PHASE-10/EX-2. F-6.

**Verification**
- VT-1 — the AC-7 round trip against the deno example.
- VT-2 — AC-8 through the real transport, asserting the backend was **not**
  spawned: point the config at a backend that would fail if it ran.
- VT-3 — the same round trip against the bash backend (AC-12).
- ~~VT-4~~ — moved to PHASE-10/VT-1. F-6.
- VT-5 — an answer naming an option the view did not offer reaches the backend
  unchanged (R-35, D17). The host validates `view_id` and nothing else, and this
  is the test that says so.
- VA-1 — `just check`.
- ~~VA-2~~ — moved to PHASE-10/VA-2. F-6.

**Notes for the implementer**

- The example is documentation as much as it is a test fixture — brief §3.7 makes
  agents the intended authors of backends. Write it to be copied.
- `deno` is chosen because it runs `.ts` with no build step and typechecks rather
  than stripping types (OQ-9). If a fixture needs to compile-fail, that is a
  different fixture, not a change of runtime.
- Leave the harness able to run a **sequence** of exchanges against one `Host`.
  PHASE-10/EX-2 needs it and retrofitting it is worse than allowing for it.

---

## PHASE-10 — The failure matrix end to end

<!-- Split out of PHASE-08 per review finding F-6. Runs immediately after
     PHASE-08 and before PHASE-09; the id is 10 rather than 09 because phase ids
     are immutable and edits append. -->

**Objective:** every **protocol-level** failure mode `design.md` §9 names is
asserted through the whole stack, the transport and lifecycle modes are asserted
end to end where nothing else asserts them through a full `Outcome`, and a host
that has seen all of them can still complete an exchange.

<!-- Narrowed per review finding F-10. The first wording claimed every AC-6 mode
     end to end while EX-1 listed only the protocol-level ones; the transport and
     lifecycle modes are held at PHASE-05 and PHASE-06, and re-asserting all of
     them here would re-inflate the phase F-6 split for size. What was a genuine
     gap — a mode nothing carries through the full `Outcome` — is EX-4. -->

**Surfaces:** `tests/backends/**`, `tests/integration/**`.

**Entry**
- EN-1 — **PHASE-08 discharged.** The harness can run a sequence of
  exchanges against one `Host` (PHASE-08's implementer notes).

**Exit**
- EX-1 — the protocol-level misbehaving backends from `design.md` §9's list run
  end to end and each asserts its own error: unknown protocol version,
  `options: []`, duplicate option ids, duplicate field ids in one option, an
  unknown `kind` nested inside a field with its path, `view` omitted,
  `"next_check": 45`, `"next_check": "1 month"`, `min: 10, max: 1`, a text field
  carrying `min`, a number field carrying `options`, and `"next_check": null` /
  `"protocol": null` asserting **nothing** is discarded. Was PHASE-08/EX-4.
- EX-2 — R-45: the whole misbehaving suite runs against **one** `Host` instance
  and a later exchange still succeeds. A backend failure may not leave the host
  unable to invoke a backend again. Was PHASE-08/EX-5.
- EX-3 — the schedule is unchanged across a timeout, a non-zero exit and a
  malformed-JSON exchange, asserted here through the real transport rather than
  through PHASE-07's fake (R-29).
- EX-4 — the transport and lifecycle modes reach the caller as the right
  `Outcome`, which PHASE-05 and PHASE-06 do not assert because they test the
  transport rather than the host: a command that cannot be spawned, a timeout, a
  non-zero exit, malformed stdout, output past the cap, and a stale or unknown
  `view_id`. One exchange each, through `Host`, asserting the variant — not the
  transport-level error, but what a caller receives. Added per review finding
  F-10; EX-3 already runs three of these through the real transport but asserts
  only that the schedule did not move.

**Verification**
- VT-1 — one test per failure mode in EX-1, each asserting the specific variant
  and, where the design gives one, the path. Was PHASE-08/VT-4.
- VT-2 — EX-2 as a single test: construct one `Host`, run the whole suite through
  it, then assert a well-behaved exchange still succeeds.
- VT-3 — one test per mode in EX-4, each asserting the `Outcome` variant the
  caller sees.
- VA-1 — `just check`.
- VA-2 — walk `design.md` §9's misbehaving-backend list item by item against the
  tests that now exist and record any item with no test in the phase sheet. The
  list is long and prose-shaped; a gap in it is invisible without this pass. Was
  PHASE-08/VA-2.

**Notes for the implementer**

- Most of these cases already exist as **fixtures** in the protocol tier from
  PHASE-04. That is not duplication: the protocol tier proves normalization
  refuses them, this tier proves the refusal survives the transport, the `Host`
  and the `Outcome`. Where a case adds nothing beyond the fixture, say so in the
  phase sheet rather than writing a test that asserts the same call twice.
- EX-2 is easy to satisfy accidentally by using a fresh `Host` per case. Do not:
  the requirement is about reuse.

---

## PHASE-09 — `AGENTS.md`, the restatement sweep, and reconciliation of the draft

**Objective:** the repository's own map is true, the slice's documents agree with
each other and with the code, and the slice is in a state audit can start from.

**Surfaces:** root `AGENTS.md`, `docs/slices/001/draft-spec.md`,
`docs/slices/001/notes.md`, `docs/slices/001/slice-001.md`.

**Entry**
- EN-1 — **PHASE-08 and PHASE-10 discharged.** This phase runs
  last, after PHASE-10.

**Exit**
- EX-1 — root `AGENTS.md` carries what brief §15.1 asks and it currently lacks:
  that the host does not understand the user's domain, the permissive-wire /
  canonical-internal rule, the warning against narrowing the protocol to the
  current renderer, pointers to the authoritative documents, and the
  verification commands (AC-10) — named as **`just` recipes**, `just check` for
  the gate, with `design.md` §9's block cited as where the underlying six live
  (user decision 2026-08-27). It is **additive** — the existing pointer,
  canon rule, dev-shell facts and working principles stay.
- EX-2 — the restatement sweep from `design.md` §9 has been run over this slice's
  batch of work: §5.5's invariant and edge tables, §7's decision index, §8's
  risks, §9's AC map and misbehaving-backend list, `draft-spec.md` §4 and §6, and
  the AC text in `slice-001.md`. Divergences are recorded, not silently fixed —
  a design that departed from the code is `audit.md`'s **Design drift**, and code
  that departed from the design is a finding.
- EX-3 — `draft-spec.md` §7's rows point at tests that exist, by name. A row
  pointing at nothing is either a missing test or a requirement that shipped
  unverified, and the second must say so explicitly with a reason (AC-13).
- EX-4 — `notes.md`'s Harvest is current: what was produced, what was learned,
  what is open. The `docs/memory/` candidates are listed but **not** moved —
  lifting them is a close-stage act.
- EX-5 — every phase in `notes.md`'s status table is `done` with a date, and each
  phase sheet records what it actually did rather than what it planned to.

**Verification**
- VT-1 — the full suite, both columns, from a **clean clone** — not the working
  tree. AC-1 says "from a clean clone in the nix dev shell" and a working tree
  can pass on a file nobody committed.
- VA-1 — `just check`, on that clean clone, entered via `nix develop` so that
  `just` comes from `devToolPkgs` and not from a user profile.
- VT-2 — the two mechanical halves of the sweep, run as commands with their
  output recorded rather than as reading (review finding F-5): every struck or
  superseded decision id in `design.md` §7 grepped for elsewhere in the slice's
  documents, and every type or function named in §5 grepped for a definition in
  §5. Both have produced findings before — F-56 found D41 and D42 still cited as
  holding invariants, F-55 found `WireOpt` named and undefined, F-56 found
  `cleanup_only`. A non-empty result is a gap, not a pass.
- VA-2 — AC-by-AC walk of `slice-001.md`, recording for each the test or check
  that discharges it. This is the input audit's evidence-gathering starts from,
  not a substitute for it. It stays a read: `design.md` §9 says outright that no
  test can observe that two English sentences disagree, which is the reason the
  restatement sweep belongs to review and not to CI.
- VH-1 — the user accepts `AGENTS.md`'s content. It is the document every future
  agent reads first, and its wording is a judgement call the plan should not make
  alone.

**Notes for the implementer**

- AC-14 is **not** yours. Promoting `draft-spec.md` to `docs/specs/` and applying
  `canon-delta.md`'s CD-1 and CD-2 happen at audit and close, with explicit user
  endorsement. Leave both in the slice folder.
- The sweep is a review step and not a test, for the reason §9 gives: no test can
  observe that two English sentences disagree. Its trigger is the **batch** —
  which this slice is, in its entirety — and it has one owner and one moment
  (F-56).
- The two mechanical halves of the sweep are VT-2 and are commands. What is left
  for VA-2 is the part that is irreducibly a read — whether two statements of the
  same contract, in different sections, still say the same thing.

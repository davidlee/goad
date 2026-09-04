# Plan — Slice 002: The workspace split, and the first renderer

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

Ten phases take the repository from one crate with no renderer to a workspace of
four members whose stratum 3 draws a `choice` view, answers it, reports every
failure in SPEC-001's taxonomy, and stops without waiting for an exchange it has
abandoned. **They run in the order 01, 02, 03, 04, 05, 06, 07, 10, 08, 09.**
PHASE-10 is PHASE-07 split in two at the seam its own objective states
(review-plan F-12, PL-10); phase ids are immutable, so the sequence goes
non-monotonic and that is expected.

The spine is D1: **the split lands first and alone, on a tree with no renderer in
it.** PHASE-01 is that split and nothing else. PHASE-02 rebuilds the
workspace-wide invariant checks the split displaces. PHASE-03 puts `slint` in the
dependency graph for the first time. PHASE-04 through PHASE-08 build stratum 3
inward-out — the pure functions first, the glass, then the loop, and the process
entry point last. PHASE-09 is the documentary close-out the slice owes before
audit.

Six things are established early and hold for every phase after.

1. **The gate is six commands from PHASE-01 onward** (`design.md` §5.6). A phase
   is not green until `just check` exits 0. There is no feature matrix any more:
   the split retires the `shell` feature and the second clippy column with it,
   and the renderer adds no column back. Every phase's VA-1 is the gate's output
   pasted into its sheet — not "they should pass".
2. **The working authority for the gate is `draft-policy.md`, not
   `docs/slices/001/design.md` §9.** `docs/AGENTS.md:36` makes the slice's drafts
   its working authority for the duration; promotion is at audit (`:38`). So
   `plan.md`, every phase, and the `justfile`'s own header comment cite
   `draft-policy.md` and `canon-delta.md` exactly as they would cite canon, and
   every phase keeps them current. **No phase writes into `docs/specs/`,
   `docs/policy/` or `docs/adr/`, and no phase edits `CLAUDE.md`** — CD-5, CD-6
   and CD-7 all target `CLAUDE.md` and all land at audit.
3. **`design.md` §5.5's STOP table, S-1…S-8, binds every phase.** It is copied
   verbatim into every phase sheet at phase-plan time. Each phase below names the
   subset that can actually fire in it, and adds a phase-local condition only
   where the design left one unnamed. On any of them: stop, write what happened
   into `notes.md`, return a stop status. None is a threshold to relax.
4. **No phase has a VH criterion.** These phases run with no human present, so
   every entry, exit and verification criterion below is a command an agent can
   run or a file it can read. Where slice 001 used a VH — the dev shell being
   reloaded after a `flake.nix` change — this plan uses
   `nix develop --command …` instead (PHASE-03/EX-2).
5. **Absence is never asserted alone.** Guiding principle 3 and E-1: every
   absence-shaped assertion is paired with a presence-shaped one and demonstrated
   against a deliberately broken implementation, with the break-and-revert output
   pasted into the sheet. Three separate mechanisms in this stack make "nothing is
   showing" pass against a broken UI.
6. **The lint shapes are applied, not rediscovered.** `design.md` §5.4's *The
   shapes the lint table requires* is nine rules measured against the real table,
   plus §9's two rules for test targets. A phase reads that table before it writes
   renderer code. **A-2's expectation budget is two spendable**, stated
   arithmetically because the design states it twice and differently: §5.5's S-1
   fires on the **third** distinct `#[expect]` outside the generated-code
   quarantine and says "Two remain unspent", while §5.5's A-2 prose says "three
   remain". The stop's trigger governs — two may be spent, the third stops the
   phase (DF-7).

## Sequencing & rationale

**Why the split is alone.** D1. Every measurement behind it was taken on a tree
with no renderer; putting the split's failures and the renderer's failures in one
diff leaves no way to tell them apart. It is also the cheapest audit available of
the least-audited passage in the design: §5.1's artifact map states ~115 renames
file by file, and executing it either succeeds or fails loudly on the first
`cargo build`. Round 1 of `review-plan.md` walked that map against
`git ls-files` and found seven rows wrong; the map is repaired, and the totals it
now carries are derived by command rather than quoted (F-1, F-33). PHASE-01's exit criteria are written to **catch a wrong map**
rather than route around one — EX-3 walks the map row by row in both directions,
and a file that has to move and is not in it is S-6.

**Why the boundary rewrite is its own phase.** `tests/protocol/boundary.rs` is
345 lines today and is the one file the map marks *substantively rewritten*.
PHASE-01 relocates it with the minimum change that keeps it running; PHASE-02
rewrites it into `goad-boundary`'s three modules, adds the manifest allowlist and
the stratum 1 purity scan, and retires the `tokio` source grep **in the same
change** that lands its replacement, which is what §5.6 asks for. Doing both in
one phase would put a 600-line rewrite inside the phase whose value is being a
pure relocation, and would spoil AC-2's evidence.

**Why `slint` enters on its own commit.** A-4 is the only assumption the first
renderer commit genuinely exists for, and its measurement protocol says *before
anything else in that phase is done*. PHASE-03 adds the member, the font and the
markup, measures, and then builds the element-tree tier. It is also where A-1
(the twelve-lint list), A-3 (`with_debug_info`) and A-7's regression check all
settle, and where S-2, S-3, S-4 and S-7 can first fire.

**Why the pure functions come before the loop.** `serve` consumes the mapper, the
reception seam, the diagnostic reducer and the controller. Building it first would
mean writing it against types that do not exist and editing it when they do.
PHASE-04 and PHASE-05 are testable with no component and no runtime; PHASE-06's
controller is testable with no glass; only PHASE-07 needs all three at once.

**Why the failure table sits at PHASE-06 and the loop at PHASE-10.** §12.1 folds
every row through `Controller::absorb` and reads
`controller.frame().diagnostics.lines()`. It needs no glass and no event loop, so
it lands with the controller it drives. Item 11 needs the element tree and the
production `serve`, so it lands with them. Splitting there puts thirty-three rows
of transcription in one session and nine loop observables in another. Item 11
splits again at the PHASE-07 / PHASE-10 seam (PL-10): 11e, 11f, 11g and 11i need
a glass and a wire but no loop; 11a–d and 11h need `serve`.

**Why startup lands with the entry point.** `design.md` §5.4: all eight
`StartupError` variants must be *constructed* in the phase that lands them, and
their only construction sites are in `run` and `start`. So `startup.rs`,
`main.rs`, the two remaining diagnostic outlets and item 17 are one phase. It is
the heaviest of the renderer phases and it is heaviest for a stated reason.

**Reordering.** PHASE-04 and PHASE-05 could swap only if `Prepared` and the
mapper moved with them; nothing else can be reordered without breaking a
dependency. PHASE-09 could in principle fold into audit — it is not, because
`docs/AGENTS.md:40` forbids closing a slice on a stale draft, and audit is not the
place to first make one true.

**Parallelism.** None. Every renderer phase touches `crates/goad/src/lib.rs` and
`crates/goad/tests/renderer/main.rs`. One agent, one phase, in sequence — and
the sequence is 01…07, **10**, 08, 09.

**Size.** PHASE-01, PHASE-06, PHASE-08 and PHASE-10 are the four heavy
sessions — a ~115-file relocation, a 33-row table, the entry point plus the
event-loop tier, and `serve` plus cancellation. PHASE-04 is the lightest. The
fourth is why PHASE-07 was split: as originally written it landed five source
modules and discharged thirteen verification groups, among them a real-process
reducer walk, R-33 staleness with a negative control, back-pressure, two
simultaneous-ready races and a 250 ms latency measurement — more than PHASE-06,
which the plan already called heavy, and it was not on the list (review-plan
F-12). None is now expected to exceed one session including bookkeeping; if one
does, that is a finding for `notes.md`, not a reason to skip the sheet.

## Decisions taken during planning

Recorded in `plan-log.md` with their reasoning and rejected alternatives, under
the standing autonomy grant (`design-log.md`, 2026-09-05). Summarised here
because each one decides where a phase boundary falls.

- **PL-1 — the split lands three members, not four.** §5.1's map is the *slice's*
  end state; D1 forbids `crates/goad` existing before Slint does. PHASE-01
  creates `goad-semantics`, `goad-shell` and `goad-boundary`, and four of the six
  `[[test]]` targets. The `goad` member row and the `renderer` / `event_loop`
  target rows are PHASE-03's and PHASE-08's, and PHASE-01/EX-6 names them as
  deliberately deferred rather than leaving a reader to infer it.
- **PL-2 — `goad-boundary`'s rewrite is PHASE-02.** PHASE-01 moves the machinery
  into `crates/goad-boundary/src/` and the token lists and controls into
  `tests/checks/`, which is D17's shape, and changes nothing else about it. The
  three direction tokens survive one phase and are retired at PHASE-02/EX-9,
  together with the allowlist that replaces them.
- **PL-3 — `answers-as-instructed.sh` gains its three `@lingers*` arms at
  PHASE-06,** not at the split. The map permits the change in the split row; it
  does not require it there, and item 12 is its only consumer. **The original
  rationale is void and is not the reason:** it read "PHASE-01 therefore exits
  with 91 byte-identical renames", and the backend scripts are not renames at
  all — §5.1 sends them to the path they already occupy, so the byte-identical
  set is the 88 fixtures whenever the arms land (review-plan F-1, F-17). What
  stands is the reason that never depended on a count: the arms exist for §12.1's
  table, the table is PHASE-06's, and the phase whose whole value is being a pure
  relocation should not also be authoring test fixtures.
- **PL-4 — the `driving.rs` cut is made at PHASE-01 and re-settled at PHASE-06.**
  §12.8's cut is *the intersection of what the two tiers use*, and the second tier
  does not exist until PHASE-06. At PHASE-01 the file has one consumer and every
  item in it is live. At PHASE-06 the `goad` target includes the same file, and
  any helper that target does not call is dead code under `-D warnings`: moving
  it back into `crates/goad-shell/tests/integration/harness.rs`, or moving one the
  other way, is §12.8's own rule being applied, not a design change.
- **PL-5 — three file placements are decided where the design states two
  different things.** See *Findings against the design* below; also recorded in
  `design-log.md`.
- **PL-6 — item 14f gets a fourth module in `goad-boundary`'s `checks` target,
  `structure.rs`.** §5.1 places item 14f in that target but its module list names
  three, one per ADR-001 instrument, and a `quit_event_loop` call-site count is
  none of them.
- **PL-9 — `boundary.rs`'s PHASE-01 destination is stated, not discovered.**
  §5.1's row gives it as "`crates/goad-boundary/`, split across `src/` and
  `tests/` — below", and "below" is §5.6's three-module API, which is PHASE-02's
  shape. So the forward map walk had nothing to check for the one row it matters
  most for, and PHASE-01 would have invented a file layout — the class of
  decision §5.1 exists to prevent (F-37). *Decided:* PHASE-02's shape **minus
  `manifest.rs`**, which needs a `toml` this phase does not have.
  `crates/goad-boundary/src/lib.rs` declares `pub mod scan;` and nothing else;
  `src/scan.rs` carries today's `Scan`, `Breach`, `mentions`, `report` and
  `Scan::run`, changed only as EX-5c permits; `tests/checks/` carries `main.rs`,
  `vocabulary.rs` and `direction.rs` — the token lists and the controls.
  PHASE-02/EX-1 is then a restructure of a known starting point, adding
  `members.rs` and `manifest.rs`, rather than a discovery.
- **PL-10 — PHASE-07 splits at the seam its own objective states, and the second
  half is PHASE-10.** PHASE-07 keeps `glass.rs`, `install.rs` and `wire.rs`'s
  `Wire` / `Cancel` (items 11e, 11f, 11g, 11i); PHASE-10 takes `serve` and
  cancellation (items 11a–d, 11h, 14a–d). Ids are immutable and never renumbered,
  so PHASE-10 executes between PHASE-07 and PHASE-08 and PHASE-08's EN-1 names
  it. PHASE-10's entry criterion is mechanical, which is what makes the seam
  usable: `Glass`, `SlintGlass`, `install`, `Wire` and `Cancel` all exist and the
  gate is green.
- **PL-11 — the member manifest skeleton is written out once (EX-8a).** The plan
  previously *offered* `publish = false` two ways and called four inherited keys
  "worth inheriting", in the one place `clippy::cargo` at `deny` bites; and it
  said nothing about the root's `description`, `keywords`, `categories` and
  `readme`, which die with the root package. *Decided:* inherit five keys from
  `[workspace.package]`, carry no metadata keys at all in members. A member
  `description` is also a place `CLAUDE.md` invariant 1 could be breached by a
  string no scan reads — PHASE-02/EX-11 scans sources, not manifests.
- **PL-12 — `boundary.rs`'s three member scans are one `#[test]` over three
  `Scan`s.** PHASE-01/VT-1 asks whether the test population survived the split,
  and the answer depends on a shape the plan had not chosen. One test iterating
  three `Scan`s keeps `boundary.rs` at five test functions, so VT-1's name-set
  diff has nothing legitimate to absorb; and it keeps the hand-written R7 shape
  PHASE-02 retires visible as one thing rather than three. `Breach` carries the
  path, so a failure still names which member.

## Findings against the design

Raised during planning, not repaired in `design.md` (`docs/AGENTS.md:137` — the
design is a record of intent). Each is resolved here so no phase has to invent an
answer, and each is a candidate for the audit's *Design drift not reconciled*.

**The one exception is §5.1's artifact map, which *was* repaired**, on
2026-09-05, against `git ls-files` on this branch: seven rows and one paragraph
(the rename totals, `Cargo.lock`, three fixture-path constants,
`transport_shape.rs`'s destination and subject paths, `round_trip.rs`'s
`include_str!`, `harness.rs`'s `example()`, and the `goad` member's `jiff` and
`serde_json`). A DF entry says "the design stands as written"; these did not
stand, and PHASE-01 would have executed against numbers it could not meet. The
reasoning and the rejected alternative are in `design-log.md`; the findings are
`review-plan.md` F-1, F-4, F-5, F-6, F-8, F-9, F-13, F-17, F-33. That is
measurement, not a review round, and it belongs in the audit's *Reconciliation*
rather than its *Design drift*.

- **DF-1 — the design states two different homes for the tray rasteriser.**
  §5.4's block is headed `// crates/goad/src/tray_icon.rs`; §5.1's map puts
  `tray_icon` in `diagnostics.rs`, and §5.1's `lib.rs` — quoted as "the whole
  file" — declares ten modules with no `tray_icon` among them. *Resolved:* the
  rasteriser, `ICON_EDGE`, `IDLE`, `FAULT` and `TrayState` live in
  `diagnostics.rs`, which already carries the
  `#![deny(clippy::arithmetic_side_effects)]` the rasteriser needs. Adding an
  eleventh `pub mod` would contradict §5.1's `lib.rs` and is a design change.
- **DF-2 — the design states two different homes for `Prepared`.** §5.2 declares
  it in a block headed `// crates/goad/src/reception.rs`, beside the `Received`
  that owns it; §5.1's map lists it under `controller.rs`. *Resolved:*
  `reception.rs`, where its shape is written. The map's reading would make
  PHASE-05's `Received` depend on a type PHASE-06 lands, inverting the phase
  order for no gain. **The converse case goes the other way:** `Wire`, `Cancel`,
  `Command` and `Stimulus` appear under a `// crates/goad/src/controller.rs`
  header in §5.3 and under `wire.rs` in the map; the map wins there, because
  otherwise `lib.rs` declares an empty `wire` module.
- **DF-3 — "glass.rs is the ONLY file in the crate that names a generated type"
  (§5.3) is false as the design itself writes it.** `install.rs` takes
  `&PromptWindow` and `&Tray`; `Wire` holds `slint::Weak<PromptWindow>`; and
  under DF-1 `diagnostics.rs` returns a `slint::Image`. *Resolved:* the
  load-bearing statement, and the one phases are held to, is that **`glass.rs` is
  the only file that reads or writes a generated component's properties**. No
  phase may treat the wider claim as a constraint.
- **DF-4 — §5.4's `dead_code` constraint on `StartupError` may be void under
  D28.** It was measured on a scratch crate (`research.md` Thread 10) before F-30
  made `crates/goad` a library, and `dead_code` does not fire on `pub` items in a
  library target. *Not resolved by reading:* PHASE-08 lands `startup.rs` and
  `main.rs` together, which satisfies the constraint whether or not it still
  binds, so nothing in the plan rests on the answer.
- **DF-5 — §12.8's two helper lists neither partition `harness.rs` nor close
  over their own dependencies.** Eight of the file's 35 items are in neither list
  (`evaluate`, `clear`, `children_running`, `command_line`, `DEFAULT_POLL`,
  `host_from`, `prompting_event`, `event`), four of those are called by helpers
  the host-driving list moves, and `CLEANUP_LIMIT` — which §12.8 sends to
  `driving.rs` — is not in `harness.rs` at all; it is
  `tests/integration/transport.rs:22`. Read literally, §12.8 deletes four live
  helpers and fails to compile. *Resolved:* PHASE-01/EX-7 states the cut item by
  item, dependency-closed, with §12.8 as its source rather than its
  specification. The design stands as written.
- **DF-6 — §5.6's `pub struct Scan` has no `#[derive(Debug)]`, and
  `missing_debug_implementations` is `deny`.** The block derives `Debug` on
  `Breach` only. It is a test-local private struct today, so the lint does not
  fire; the moment it leaves a test target it is compile-stopping.
  *Resolved:* PHASE-01/EX-5c names the derive as one of four permitted changes.
- **DF-7 — the design states A-2's expectation budget twice and differently.**
  §5.5's A-2 prose says "the budget is therefore unspent: three remain"; §5.5's
  S-1 row says the **third** distinct `#[expect]` fires the stop and "Two remain
  unspent". *Resolved arithmetically:* the stop fires on the third, so two are
  spendable. The Overview and PHASE-10/VA-3 say two; S-1's trigger is unchanged.

## Coverage

Every acceptance criterion in `slice-002.md`, mapped to the phase and criterion
that discharges it. A gap here is a gap in the plan.

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-01/EX-2 establishes the six-command gate at the split commit; every phase's VA-1 re-runs it; PHASE-09/EX-5 is the clean-clone run under `nix develop` |
| AC-2 | PHASE-01/EX-3 and EX-4 (the map walked in both directions, a predicate per row kind), EX-5a/EX-5b/EX-5c (the permitted-change vocabulary and every named exception), **EX-13** (the hunks checked against that vocabulary — the only criterion that looks at content) and EX-11 (the evidence pasted, not transcribed); PHASE-02/EX-12 supplies the argument for the one substantively rewritten file |
| AC-3 | instrument 1 — PHASE-01/EX-10 (negative control, broken and reverted); instrument 4 — PHASE-01/EX-2 (the gate's third command); instruments 2 and 3 — PHASE-02/EX-4, EX-7, EX-8, VT-1, VT-2; the counting rule and the unenforced residue — PHASE-02/VA-3 |
| AC-4 | PHASE-03/VT-2 (item 7, at the element tree); PHASE-10/VT-1 (item 11a, end to end through `serve`) |
| AC-5 | PHASE-03/VT-3 (item 8, the markup half: the right `OptionId` and the right view token, two options sharing a label); PHASE-10/VT-4 (item 11d, R-33 staleness and its negative control) |
| AC-6 | PHASE-03/VT-4 (item 9, the empty state); PHASE-10/VT-2 (item 11b, both `view: null` readings in one test) |
| AC-7 | PHASE-06/VT-1 (item 12, the whole taxonomy through one retained `Host`, with §12.6's stated exemption); PHASE-10/VT-3 (item 11c, a failed `respond` keeps the question and a retry succeeds) |
| AC-8 | PHASE-05/VT-4, VT-6, VT-7, VT-8 (items 13d, 13f, 13g, 13h — the bound at the bound, the ordering of decode/escape/bound, once-exactly, two distinguishable truncations); PHASE-08/VT-1 (item 17's `source()` clauses) |
| AC-9 | PHASE-04/VT-1, VT-2 (items 4 and 5); PHASE-05/VT-11, VT-12 (items 13k, 13l) |
| AC-10 | PHASE-03/VT-1 (item 6, the guard test) and EX-8 (the cheap tier runs with no display server) |
| AC-11 | PHASE-03/VT-4 and VT-5 (items 9 and 10 — presence *and* absence, each demonstrated against a deliberately broken implementation) |
| AC-12 | PHASE-10/VT-10…VT-13 (items 14a–d, cheap tier); PHASE-08/VT-2 (item 14e, the event-loop tier) and VT-3 (item 14f, the structural count) |
| AC-13 | PHASE-02/EX-6 and VT-3 (members enumerated, `.slint` and `.rs`, the string-aware cut, all six positive controls) |
| AC-14 | PHASE-02/VT-3 places the scan; every phase's VA-1 re-runs it, and PHASE-04/EX-3's "no file under `crates/goad/` is an image" is its neighbour (PHASE-04/VT-3) |
| AC-15 | **not fully in this plan.** PHASE-09/EX-1 makes `canon-delta.md` and `draft-policy.md` true about what shipped and records every divergence; promotion and endorsement are audit's, per `docs/AGENTS.md:38` and HARD STOP 1. PHASE-09's Objective says so too, so that a reader of the status table does not read "all phases green" as "all acceptance criteria discharged" |

---

## PHASE-01 — The workspace split: three members, the relocation, and the six-command gate

**Objective:** the single crate is a workspace of three members, every file
§5.1's artifact map moves has moved, the gate is §5.6's six commands, and the map
has been walked in both directions against what actually happened.

**Surfaces:** `Cargo.toml` (becomes the workspace root), `Cargo.lock` (rewritten
by cargo, not by hand), `crates/goad-semantics/**`, `crates/goad-shell/**`,
`crates/goad-boundary/**`, `src/**` (removed), `tests/**`, `justfile`,
`docs/slices/002/notes.md`, `docs/slices/002/plan-log.md`.

**Not touched:** `flake.nix` (PHASE-03), `clippy.toml`, `rustfmt.toml`,
`examples/**`, `README.md`, `CLAUDE.md`, `docs/specs/`, `docs/policy/`,
`docs/adr/`, and the unstaged `flake.lock` edit.

**Entry**
- EN-1 — `git rev-parse --abbrev-ref HEAD` is `slice-002`, and `git status
  --porcelain` shows exactly one entry, ` M flake.lock`.
- EN-2 — `cargo metadata --no-deps --format-version 1` reports exactly one
  package, `goad`. The split has not begun.
- EN-3 — `just check` exits 0 on the tree as it stands, and its warm wall-clock is
  recorded in the phase sheet as the pre-split baseline. (`notes.md` records
  1.836 s warm on 2026-09-05; re-measure rather than quote it.)
- EN-4 — the PHASE-01 sheet exists under `## Phase sheets` in `notes.md`, carrying
  `design.md` §5.5's S-1…S-8 table verbatim and a reading list of `design.md`
  §5.1 (whole), §5.6, §12.8, and `research.md`'s dry-run section — the last of
  those read as *what the dry run did*, never as *what this tree contains*
  (`research.md`'s counts are wrong for this branch; §5.1 is repaired and is the
  authority).
- EN-5 — **the git anchors are bound before anything moves.**
  `git rev-parse HEAD` is recorded in the sheet as `<pre-split>`; every
  `<pre-split>` below is that sha. `git merge-base main HEAD` is recorded as
  `<slice base>`, which PHASE-09/VA-3 uses. **The commit protocol is one commit
  for the whole phase**, made after the gate is green, so `<pre-split>` cannot
  drift as the phase makes commits of its own and EX-4's backward walk covers
  the whole relocation in one diff.

**Exit**
- EX-1 — `cargo metadata --no-deps --format-version 1` lists exactly three
  packages: `goad-boundary`, `goad-semantics`, `goad-shell`. The root
  `Cargo.toml` has a `[workspace]` table and **no** `[package]` table, and
  `members` is a literal three-entry list — **no glob**, because `goad-boundary`
  fails on one by construction (`Breach::GlobMember`).
- EX-2 — `just check` exits 0, and `just -n check` prints §5.6's six commands, in
  §5.6's order, with §5.6's arguments: `cargo build --workspace`, `cargo test
  --workspace`, `cargo test -p goad-semantics`, `deno check
  examples/typescript/backend.ts`, `cargo clippy --workspace --all-targets --
  -D warnings`, `cargo fmt --all --check`. The recipe names are unchanged —
  `build`, `test`, `test-stratum1`, `typecheck`, `lint`, `fmt-check` — and `lint`
  has one line.
- EX-3 — **the map, walked forward, with a predicate per row kind.** §5.1's
  split table has four kinds of row and one predicate does not fit them all;
  classify each row by its own columns and check the matching predicate,
  recording the class beside the row.
  - **moved** — `src/semantics/**`, `src/shell/**`,
    `tests/protocol/{main,normalize,runner}.rs`, `tests/protocol/fixtures/**`,
    `tests/protocol/transport_shape.rs`, `tests/protocol/boundary.rs`,
    `tests/integration/*.rs`: the destination path exists **and** the source path
    does not.
  - **deleted** — `src/lib.rs`: the path is gone, and nothing claims to be its
    destination.
  - **rewritten in place** — `Cargo.toml`, `Cargo.lock`, `justfile`: the path
    exists and `git diff <pre-split> HEAD -- <path>` is non-empty.
  - **unchanged in place** — `tests/backends/*.sh` (all 15), `clippy.toml`,
    `rustfmt.toml`, `flake.nix`, `examples/**`: the path exists and
    `git diff <pre-split> HEAD -- <path>` is **empty**.
  The last two kinds are why the old single predicate could not be discharged:
  four rows send a file to the path it already occupies, and one row has no
  destination at all.
- EX-4 — **the map, walked backward, from a pinned baseline.**
  `git diff --find-renames --name-status <pre-split> HEAD`, minus the exclusion
  set below, names no path §5.1 does not. The walk is **commit to commit**,
  never against the working tree, which is what keeps the pre-existing
  ` M flake.lock` edit out of it. The exclusion set is exactly
  `docs/slices/002/**` — this phase's own bookkeeping, which is in its Surfaces
  and in no map row, and which the map is not the place to enumerate. `Cargo.lock`
  is **not** excluded: it is now a map row of its own, because it is tracked, the
  split rewrites it, and a walk that meets it without a row fires S-6 on cargo's
  own output. A file that had to move, is outside the exclusion set, and is not a
  row of the map is **S-6**, not an improvisation.
- EX-5 — **the rename ledger, derived rather than quoted.**
  `git diff --find-renames --name-status -M <pre-split> HEAD | grep -c '^R100'`
  equals the number of files under `tests/protocol/fixtures/` at entry —
  `git ls-files tests/protocol/fixtures | wc -l`, **88** on 2026-09-05.
  Re-measure it; do not quote it. Those 88 are the fixtures, and they are the
  only byte-identical renames the split produces. Separately:
  **`tests/backends/**` appears in the walk not at all** — the map sends the 15
  scripts to the path they already occupy, so a `tests/backends` line of any
  status is a defect. (The dry run's *91 = 77 fixtures + 14 scripts*
  (`research.md:775-781`) is wrong on all three terms and was measured on a tree
  this branch never had; §5.1 is repaired, and review-plan F-1 and F-17 carry the
  measurement.) Every other renamed file's change is confined to EX-5a's
  vocabulary, and EX-13 is what confirms it.
- EX-5a — **the permitted-change vocabulary, stated once** so a diff can be
  checked against it instead of argued about. A `+` or `-` line in a renamed,
  non-identical file must be one of: a `use` line; a `mod` line; a `#[path]` or
  `#[cfg…]` attribute; a **path string literal** — a fixture root, a script root,
  an `examples/` path, or a `transport_shape.rs` subject path; an `include_str!`
  argument; or a manifest key. Anything else is **PS-1 / S-6**.
- EX-5b — **the named non-identical changes**, listed here so that none is
  discovered by a red test and EX-11's evidence line already exists for each.

  | file (post-split path) | change |
  |---|---|
  | `crates/goad-semantics/tests/protocol/normalize.rs` | `:266` → `../../tests/fixtures/protocol`; `:273` → `../../tests/fixtures/protocol-text` |
  | `crates/goad-semantics/tests/protocol/runner.rs` | `:332` → `../../tests/fixtures/schedule` |
  | `crates/goad-shell/tests/shape/transport_shape.rs` | `:32` `src/shell/backend/process.rs` → `src/backend/process.rs`; `:257` `…/process-renamed.rs` → `src/backend/process-renamed.rs`; `:276` `src/shell/error.rs` → `src/error.rs`. `CARGO_MANIFEST_DIR` is now `crates/goad-shell`, so the `shell/` segment is gone |
  | `crates/goad-shell/tests/integration/round_trip.rs` | `:49` `include_str!("../../examples/typescript/README.md")` → `"../../../../examples/typescript/README.md"`. `include_str!` resolves against the **source file**, not the manifest |
  | `crates/goad-shell/tests/integration/harness.rs` | `:207` `examples/typescript/backend.ts` → `../../examples/typescript/backend.ts`; plus the §12.8 cut of EX-7 |
  | `crates/goad-shell/tests/integration/transport.rs` | gains `use crate::driving::CLEANUP_LIMIT;` — `CLEANUP_LIMIT` lives here today (`:22`, six further sites) and EX-7 moves it |
  | `tests/support/driving.rs` | `backend()`'s `tests/backends` → `../../tests/backends` (§12.8's rule for the shared file) |
  | `crates/goad-boundary/**` | EX-5c |

- EX-5c — **`boundary.rs`'s change at PHASE-01 is exactly four things**, and a
  fifth is S-6. "The `src/` ÷ `tests/` division and nothing else" is not
  achievable: two of the four are compile-stopping under the gate.
  1. the `src/` ÷ `tests/` division D17 requires. PL-9 states the file layout, so
     the phase does not invent one.
  2. **`#[derive(Debug)]` on `Scan`.** It becomes public API and
     `missing_debug_implementations` is `deny` (`Cargo.toml:76`); §5.6's block
     derives `Debug` on `Breach` only (DF-6).
  3. **a `# Errors` section on `Scan::run`.** `clippy::missing_errors_doc` is
     pedantic-at-deny and fires on a public fallible fn. An `#[expect]` here is
     S-1 budget spent for nothing.
  4. **the workspace-root rebase.** `Scan::root()` is
     `Path::new(env!("CARGO_MANIFEST_DIR")).join(self.root)`
     (`boundary.rs:69-71`), and from `crates/goad-boundary` that resolves
     *inside the member*. It becomes `CARGO_MANIFEST_DIR` joined with `../..`
     and then `self.root` — §5.6's own rule, applied one phase earlier than §5.6
     states it — and the four configured roots (`:236` `src/semantics`, `:243`
     `src`, `:269` `docs/adr`, `:275` `src/semantics-renamed`) are rewritten for
     the new tree: three member `src/` directories (PL-2), and the two controls
     per EX-14. Without this every scan resolves to a directory that does not
     exist, all four return `Breach::Vacuous`, and the gate is red.
- EX-6 — four `[[test]]` targets exist, with the map's names and paths:
  `goad-semantics` / `protocol` / `tests/protocol/main.rs`; `goad-shell` /
  `integration` / `tests/integration/main.rs`; `goad-shell` / `shape` /
  `tests/shape/main.rs`; `goad-boundary` / `checks` / `tests/checks/main.rs`.
  Each is a `main.rs` of `#[cfg(test)] mod` declarations and nothing else. Three
  carry the map's module lists:
  - `protocol` — `#[cfg(test)] mod {normalize, runner};`
  - `integration` — `#[cfg(test)] mod {harness, fake, host, round_trip,
    transport, failure_matrix};` and the literal
    `#[cfg(test)] #[path = "../../../../tests/support/driving.rs"] mod driving;`
  - `shape` — `#[cfg(test)] mod transport_shape;`, beside
    `crates/goad-shell/tests/shape/transport_shape.rs`. That `main.rs` is an
    **added file**, not the rename's destination: the 6-test body lands at
    `transport_shape.rs`, which is what makes EX-3, EX-4 and EX-6 satisfiable at
    once. §5.1 states both paths, so neither is a path the map does not name.

  `checks` is the exception, and **PL-1's deferral extends to modules as well as
  members and targets**: at PHASE-01 it declares
  `#[cfg(test)] mod {vocabulary, direction};` — today's two configured scans and
  their controls, relocated. The map's list, `#[cfg(test)] mod {vocabulary,
  purity, allowlist};`, is **PHASE-02/EX-5's**: `allowlist` reads TOML and this
  phase's `goad-boundary` has no dependencies, and `purity` is new code
  PHASE-02/EX-8 lands. Declaring either here means an empty module, which is a
  vacuous test with no guard. The map's other two targets — `renderer` and
  `event_loop` — are **deliberately deferred to PHASE-03 and PHASE-08**, because
  D1 forbids `crates/goad` existing before Slint does (PL-1).
- EX-7 — **the shared helper exists and the §12.8 cut is made item by item.**
  `tests/support/driving.rs` exists at the workspace root and is included by
  `crates/goad-shell/tests/integration/main.rs` through the literal
  `#[path = "../../../../tests/support/driving.rs"]`, with `#[cfg(test)]` at the
  declaration site. §12.8's two lists are neither exhaustive of `harness.rs` nor
  dependency-closed — eight of its 35 items are in neither list, four of those
  are called by helpers the host-driving list moves, and `CLEANUP_LIMIT` is not
  in the file at all — so the cut is stated here in full and §12.8 is its source
  rather than its specification (DF-5).

  | item | goes to | why |
  |---|---|---|
  | `scripted`, `logging_backend`, `backend`, `marker` | `driving.rs` | §12.8 |
  | `clear` | `driving.rs` | **closure** — `marker` calls it (`harness.rs:178-182`) |
  | `invocations`, `config`, `host`, `instant` | `driving.rs` | §12.8 |
  | `DEFAULT_POLL` | `driving.rs` | **closure** — `config` reads it (`:228-237`; const at `:220`) |
  | `host_from` | `driving.rs` | **closure** — `host` calls it (`:245-251`) |
  | `quiet_event` | `driving.rs` | §12.8 |
  | `event` (private today) | `driving.rs`, as **`pub(crate)`** | **closure** — `quiet_event` calls it (`:262-271`). `prompting_event` stays behind and will call `crate::driving::event`, which is why the visibility changes |
  | `describe_outcome`, `choice`, `answer_first_option`, `presented` | `driving.rs` | §12.8. The last three all call `describe_outcome`, so the four are closed |
  | `CLEANUP_LIMIT` | `driving.rs`, with its keep-in-sync note | §12.8 — but it is **not in `harness.rs`.** It is `tests/integration/transport.rs:22`, with six further sites there. `transport.rs` gains a `use` line (EX-5b), which is inside EX-5a's vocabulary |
  | `transport`, `describe`, `describe_cleanup`, `stderr`, `children`, `alive`, `reported_pid`, `padded_evaluate`, `example` | `harness.rs` | §12.8's transport list |
  | `backend_error`, `state_error`, `protocol_error`, `only_discard`, `stderr_of` | `harness.rs` | §12.8's "the `Outcome` accessors" |
  | `evaluate` (56 sites), `children_running` (1), `prompting_event` (6), `command_line` (private; `children` calls it) | `harness.rs` | **named explicitly as staying.** In neither §12.8 list, and each is transport- or integration-local |

  Nothing in `harness.rs` is unaccounted for after this table, which is the
  property "exactly §12.8's list" did not have.
- EX-8 — `[workspace.lints]` carries the pre-split `[lints.rust]` and
  `[lints.clippy]` **levels** verbatim: diff the lint *lines* against
  `git show <pre-split>:Cargo.toml` and expect no difference. The **comments are
  not verbatim, and must not be**: `Cargo.toml:78-104`'s `dead_code` /
  `unreachable_pub` carve-out is justified in-comment by "the
  `--no-default-features` column drops `shell`", and EX-9 retires that column in
  this same commit. Rewrite that block to match the one-column gate and paste its
  diff into the sheet; shipping the old rationale is shipping a false statement
  about the gate. Every member manifest carries `lints.workspace = true` and no
  other `[lints]` content (D8). Every member sets `autotests = false` and declares
  its test targets explicitly.
- EX-8a — **the member manifest skeleton, written out once** so no phase invents
  a key, the way §5.6 writes `goad-boundary`'s API out once so no phase invents a
  signature. In the root:

  ```toml
  [workspace.package]
  version    = "0.1.0"
  edition    = "2024"
  license    = "MIT"
  repository = "https://github.com/davidlee/goad"
  publish    = false
  ```

  and in every member, identically but for the name and the dependency block:

  ```toml
  [package]
  name              = "goad-…"        # or "goad"
  version.workspace = true
  edition.workspace = true
  license.workspace = true
  repository.workspace = true
  publish.workspace = true
  autotests         = false

  [dependencies]      # §5.1's member table; every entry `{ workspace = true }`
  [lints]
  workspace = true
  [[test]]            # §5.1's test-target table
  ```

  Two things this settles rather than offers. **`publish = false` is inherited
  once from `[workspace.package]`**, not repeated per member — it is what
  silences `clippy::cargo_common_metadata`, which is in the `cargo` group at
  `deny` (the root manifest's own comment, `Cargo.toml:12-15`, carried forward).
  And **no member carries `description`, `keywords`, `categories` or `readme`**:
  the root's metadata (`Cargo.toml:5-10`) dies with the root package, `publish =
  false` already discharges the lint, and a member `description` is a place
  `CLAUDE.md` invariant 1 could be breached by a string no scan reads —
  PHASE-02/EX-11's vocabulary scan reads sources, not manifests (PL-11).
- EX-9 — the `shell` feature is gone, not relocated: no member declares a
  `[features]` table, `grep -rn 'feature *= *"shell"'` over `Cargo.toml`,
  `crates/` and `tests/` returns nothing, and no `[[test]]` carries
  `required-features`. **`tokio` and `toml` lose `optional = true`** as they move
  into `[workspace.dependencies]` (`Cargo.toml:25`, `:36`): the key belonged to
  the feature, `optional` is not meaningful in `[workspace.dependencies]`, and a
  member inheriting it would carry an optional dependency with no feature to
  enable it. `grep -n optional Cargo.toml crates/*/Cargo.toml` returns nothing.
- EX-10 — **instrument 1, observed rather than asserted.** Add
  `use goad_shell::host::Host;` to a `crates/goad-semantics/src/` file and confirm
  `cargo build -p goad-semantics` fails with `error[E0433]`; revert. Repeat with
  `use tokio::process::Command;`. Both outputs pasted into the sheet.
- EX-11 — the AC-2 evidence list is the **pasted output** of
  `git diff --find-renames --name-status -M <pre-split> HEAD`, plus one
  hand-written line per file whose status is not `R100`, naming the map row that
  permits its change and the EX-5b entry it matches. The command is the evidence;
  the prose is only for the exceptions. Hand-transcribing ~115 similarity indices
  is the largest single cost in a phase whose mechanical work the dry run put at
  ~6 minutes, and it is where a wrong number gets copied forward — which is
  exactly how the dry run's 77 and 14 survived five review rounds.
- EX-12 — the `justfile` header comment cites `docs/slices/002/draft-policy.md`
  (the slice's working authority, `docs/AGENTS.md:36`) and `design.md` §5.6, and
  no longer cites `docs/slices/001/design.md` §9. `CLAUDE.md` is **not** edited:
  repointing it is CD-5 and lands at audit, paired with the policy's promotion.
- EX-13 — **the content check, mechanical.** This is the criterion AC-2 actually
  turns on, and until now nothing in the phase compared the *content* of a moved
  file against its source: EX-3 and EX-4 check paths, EX-5 checks byte-identity
  only for the `R100` set, and `--name-status` prints a similarity index and no
  hunks — `R091` looks the same whether the 9% is an import block or a rewritten
  function body. The gate cannot tell either: a semantically equivalent but
  different body passes every test.
  So: for every renamed file whose status is not `R100`, run
  `git diff --find-renames -M <pre-split> HEAD -- '<old path>' '<new path>'` —
  **full hunks**, not `--name-status` — and confirm every `+` and `-` line
  matches EX-5a's vocabulary. Paste the hunks into the sheet. A line outside the
  vocabulary is **PS-1 / S-6**, detected rather than self-reported.
- EX-14 — **the relocated checks are demonstrated non-vacuous, not assumed so.**
  Three of them are guarded only by a vacuity check, and a re-rooting that breaks
  them leaves the guard passing for the wrong reason.
  - **All three fixture corpora report a non-zero inspected count.**
    `runner.rs:98` fails with *"ran no fixtures — renamed, emptied, or
    misspelled"* when one does not; that message appearing for none of the three
    is the evidence, and it is why EX-5b lists three constants and not one.
  - **The `shape` target's negative control still fails for its own reason.** The
    `process-renamed.rs` row (`transport_shape.rs:257`) must still fail when
    pointed at a path that does not exist. Otherwise the re-rooting of the three
    subject paths is unwitnessed, and the file's vacuity guard is all that stands
    between the split and a silently-passing shape check.
  - **Both of `boundary.rs`'s vacuity controls still fail for the reason each was
    written for.** `RENAMED_AWAY` (const `:274-277`, test `:293-305`) fails
    because its root is missing. `NOTHING_TO_INSPECT` (const `:265-270`, test
    `:279-291`) must fail because `docs/adr` **exists and holds no Rust** — its doc comment says so. Rebased member-relative it would
    resolve to `crates/goad-boundary/docs/adr`, which does not exist: the test
    would still pass, as an exact duplicate of `RENAMED_AWAY`, and the only
    control for the *no-scannable-files* case would be silently gone. Assert on
    the `Breach::Vacuous` payload's `root`, or point the control at a directory
    that provably exists post-split.

**Verification**
- VT-1 — **the test *names* survive the split, not a total.**
  `cargo test --workspace -- --list` after, diffed against
  `cargo test -- --list` plus `cargo test --no-default-features -- --list`
  before, is equal as a **set of test names with module prefixes stripped**. A
  test file that fails to be re-declared in its new `main.rs` disappears
  silently, and this is what catches it. An equality of *counts* does not, in
  either direction: PL-2 turns `boundary.rs`'s two configured scans into three,
  so the count moves legitimately, and a real loss that happens to balance
  passes. **Decided so the phase does not have to (PL-12): three `Scan`s, one
  `#[test]` iterating them** — `boundary.rs`'s five test functions stay five, and
  the R7 shape PHASE-02 retires stays visible as one thing rather than three.
- VT-2 — `goad-boundary`'s `checks` target runs the domain-vocabulary scan over
  the `src/` of all three members and still carries **both** vacuity controls —
  `RENAMED_AWAY` (`boundary.rs:293-305`) *and* `NOTHING_TO_INSPECT`
  (`:279-291`) — each failing for its own distinct reason, per EX-14.
- VA-1 — `just check` run and its output pasted into the phase sheet, with the
  warm wall-clock beside the EN-3 baseline.
- VA-2 — the two map walks (EX-3, EX-4) pasted, **together with EX-13's full
  hunk output**. `--name-status` alone is a list of names and a similarity index;
  it is not evidence about content, and content is what AC-2 is a claim about.
- VA-3 — `just -n check` output pasted beside §5.6's block. Compare the **command
  sequence**, not the characters: §5.6 is a fenced block and `just -n` prints
  neither comments nor line wrapping (slice 001's plan-review F-9, against this
  criterion's own earlier wording).
- VA-4 — `cargo tree -p goad-semantics --edges normal,build,dev` contains no
  `tokio` node and no `toml` node. Pasted.

**STOP**
S-6 and S-8 from `design.md` §5.5, plus two the design does not name because they
are local to this phase:
- **PS-1** — a `+` or `-` line in **EX-13's hunks**, in a production source under
  `crates/goad-semantics/src` or `crates/goad-shell/src`, falls outside EX-5a's
  vocabulary. The trigger is EX-13's output, not the agent's judgement about what
  a file "needs" — a self-report with no detection procedure is not a stop
  condition. The dry run says no production file needed a change beyond its
  import block; a file that does is R4's signal that the split is a redesign, and
  AC-2 says so.
- **PS-2** — `just check` is still red after **five** distinct repair attempts,
  or **45 minutes** from the first attempt, whichever comes first; or, at any
  attempt, the failure names a file the map marks change-forbidden. On reaching
  the count or the clock, paste the failing output into the sheet **before**
  deciding anything. The earlier wording — "the remaining failure is not an
  import path, a manifest entry, or a test-target declaration" — is not a trigger
  an agent can apply to itself: almost any compile failure in a 115-file
  relocation can be narrated as one of those three, and the agent doing the
  narrating is the one who wants to keep going.

**Notes for the implementer**

- Move with `git mv`, one row of the map at a time, so rename detection is exact
  and EX-5's similarity indices mean something. A copy-then-delete makes the
  whole map unauditable.
- The `#[path]` arithmetic checks out and has been verified on paper: `#[path]`
  on a `mod` in `crates/<member>/tests/<target>/main.rs` resolves relative to
  that directory, so four `../` reaches the repository root. Every member sits at
  depth two, which is what makes the one literal uniform.
- Cargo will not parse a `[[test]]` whose `path` does not exist, so all four
  `main.rs` files must exist before the manifests do.
- The member manifests are **EX-8a's skeleton**, key for key. It is written out
  so the phase transcribes rather than chooses; `publish`, `edition`, `version`,
  `license` and `repository` are inherited from `[workspace.package]`, and the
  four metadata keys the root package carries today are not carried anywhere.
- `clippy.toml` and `rustfmt.toml` stay at the workspace root and are read from
  there for every member. Do not copy them into members.
- `crates/goad-boundary` has **no dependencies** in this phase. `toml` arrives at
  PHASE-02 with the allowlist that needs it; it is already in
  `[workspace.dependencies]`.
- **The workspace root is `CARGO_MANIFEST_DIR` joined with `../..`** — §5.6's
  rule, and it binds *this* phase, not just PHASE-02, because `Scan::root()`
  resolves against `CARGO_MANIFEST_DIR` today and after the move that is
  `crates/goad-boundary/`. EX-5c item 4 is the change; missing it turns all four
  scans vacuous and the gate red. A test binary's working directory is not
  something to rely on.
- `boundary.rs`'s two configured scans become three, one per member's `src/`,
  written out by hand, as **one `#[test]` over three `Scan`s** (PL-12). That is
  the R7 shape the design retires — PHASE-02/EX-6 replaces it with enumeration,
  and this phase does not attempt it.
- Moving `Scan`, `Breach`, `mentions` and `report` into a library makes them
  public API, so `clippy::missing_errors_doc` fires on `Scan::run` and
  `missing_debug_implementations` fires on `Scan`. A `# Errors` section and a
  `#[derive(Debug)]` are the answers; an `#[expect]` for either is S-1 budget
  spent for nothing. Both are in EX-5c.
- `tests/protocol/transport_shape.rs` cannot stay in a stratum 1 target: it names
  a stratum 2 source. The 6-test body becomes
  `crates/goad-shell/tests/shape/transport_shape.rs`, with a **new**
  `crates/goad-shell/tests/shape/main.rs` carrying
  `#[cfg(test)] mod transport_shape;` beside it — an added file, not the rename's
  destination, so EX-5's count is not polluted. Its three subject-path constants
  move with it (EX-5b). This is the upward reach the single crate hid, and
  finding it is ADR-002's Verification section working.
- Run `cargo fmt --all` last, and re-run the whole gate after it.

---

## PHASE-02 — The workspace invariant checks: `goad-boundary`'s three instruments

**Objective:** ADR-001's stratum 1 rule is held by four instruments with four
stated boundaries, `CLAUDE.md` invariant 1 is held by a fifth that enumerates its
own members and reads `.slint`, and the `tokio` source grep is retired in the
same change that lands what replaces it.

**Surfaces:** `crates/goad-boundary/**`, root `Cargo.toml`
(`[workspace.dependencies]` only, if `toml`'s entry needs adjusting),
`docs/slices/002/notes.md`.

**Entry**
- EN-1 — every PHASE-01 exit criterion is discharged and recorded in its sheet,
  and `just check` exits 0.
- EN-2 — `crates/goad-boundary` exists as a member depending on no other member.

**Exit**
- EX-1 — `crates/goad-boundary/src/lib.rs` declares exactly `pub mod members;`,
  `pub mod scan;`, `pub mod manifest;` and nothing else.
- EX-2 — `scan.rs` carries `Scan { root, extensions, excluded_dirs, forbidden }`,
  `Breach` with its four variants (`Token`, `Vacuous`, `Unreadable`,
  `GlobMember`), `Scan::run -> Result<usize, Vec<Breach>>` reporting **every**
  breach rather than the first, and `pub fn code_of(line: &str) -> Cow<'_, str>`
  implementing D13's four-state per-line cut. `mentions`' signature is unchanged.
- EX-3 — `members.rs` carries
  `pub fn members(root_manifest: &Path) -> Result<Vec<PathBuf>, Vec<Breach>>`,
  returning entries in manifest order and failing on a glob entry, an unreadable
  manifest, and an empty list.
- EX-4 — `manifest.rs` carries
  `pub fn unpermitted(manifest: &Path, text: &str, permitted: &[&str]) -> Result<usize, Vec<Breach>>`,
  reading every table named `dependencies`, `dev-dependencies` or
  `build-dependencies` **at any depth**, checking a renamed entry by its
  `package` value as well as its key, and treating zero entries across all tables
  as `Breach::Vacuous`.
- EX-5 — `tests/checks/main.rs` declares `#[cfg(test)] mod {vocabulary, purity,
  allowlist};`.
- EX-6 — the vocabulary scan is **one** configured template applied to every entry
  `members()` returns, with `extensions = ["rs", "slint"]` and
  `excluded_dirs = ["tests", "target"]`. There is no hand-written per-member list.
- EX-7 — the allowlist runs against `crates/goad-semantics/Cargo.toml` with
  `["jiff", "serde", "serde_json"]` and against `crates/goad-shell/Cargo.toml`
  with that list plus `["goad-semantics", "tokio", "toml"]`. `goad` and
  `goad-boundary` have no allowlist, and the test says why.
- EX-8 — the purity scan runs over `crates/goad-semantics/src` with §5.6's nine
  forbidden tokens. `std::time::Duration` is **not** among them, and the test
  says why.
- EX-9 — the three retired direction tokens are gone: no test greps a stratum 1
  source for `tokio`, `crate::shell` or `crate::bin`. The fact they checked now
  lives in a manifest, where EX-7 reads it.
- EX-10 — `cargo tree -p goad-boundary` shows `toml` and **no** `goad-*` node.
- EX-11 — `crates/goad-boundary/src/` contains no domain token: it is scanned by
  the check it implements, and the token list lives under `tests/`.
- EX-12 — the AC-2 argument for `boundary.rs` is written into the phase sheet:
  what was rewritten, and why D13, D17, the allowlist and the purity scan are the
  argument.

**Verification**
- VT-1 — the allowlist's seven controls (§9 item 3), each a literal manifest
  through `unpermitted`: `tokio` in `[dependencies]`; in `[dev-dependencies]`; in
  `[build-dependencies]`; in `[target.'cfg(unix)'.dependencies]`; renamed behind
  `package`; a manifest with no dependency table at all (`Vacuous`); and the real
  stratum 1 manifest, clean.
- VT-2 — the purity scan's controls: one per forbidden token planted in a stratum
  1 source; one planted **inside a comment**, to prove the cut still applies and
  the control is therefore not vacuous; and the real `crates/goad-semantics/src`,
  clean.
- VT-3 — the vocabulary controls of §9 item 15, in full. *Positive:* a forbidden
  word in a `.slint` component name; in an `accessible-label`; in an ordinary
  `.rs` string; in a string after a URL on the same line; after an escaped quote;
  after a raw string. *Negative:* `// the call sites`; `let x: &'static str =
  "ok"; // habit`; `/* habit */ let x = 1;`. *Vacuity:* a member directory with no
  `.rs` or `.slint` file fails naming itself; a glob in `workspace.members` fails;
  a scan pointed at a renamed-away root fails.
- VT-4 — `code_of` returns `Borrowed` when nothing was cut or the cut ran to end
  of line, and `Owned` with **one space** where an interior `/* … */` closed and
  left code behind it: `Site/*x*/View` must not become `SiteView`.
- VA-1 — `just check` output pasted.
- VA-2 — break-and-revert on each of the three instruments: plant a violation,
  confirm that instrument alone fails and names the file and line, revert. All
  three outputs pasted.
- VA-3 — the counting rule is stated in exactly one place. Confirm by reading that
  `design.md` §5.1 is the only statement of *four ADR-001 instruments, plus the
  domain-vocabulary scan, plus one residue nothing enforces*, and that this
  phase's test module docs cite it rather than restating a number. Confirm also
  that the D25 feature residue is written down as unenforced and that **no test in
  this phase claims to reject it**.

**STOP**
S-8, plus:
- **PS-3** — the root `Cargo.toml` cannot list members without a glob. `members()`
  fails on a glob by design; if the workspace genuinely needs one, that is a
  design question about D13, not a phase's.

**Notes for the implementer**

- The `.slint` positive controls need a `.slint` file to scan and no member has
  one yet. Point a control `Scan` at a fixture directory under
  `crates/goad-boundary/tests/`, which the production walk excludes by
  `excluded_dirs` and a control reaches by naming it as `root`.
- D13 names four costs of the line-based cut — a multi-line string literal, a
  multi-line block comment, `r"` being recognised in `.slint` where it does not
  exist, and all-caps compounds. Record them beside the code; do not close them.
  A scan trusted past its reach is worse than one that is not trusted.
- §5.6 names the purity scan's three misses — `use std::{fs, process};`, a later
  alias, and I/O performed on stratum 1's behalf by a permitted dependency. It is
  a **regression tripwire**, and every place that cites it must say so.
- `Breach` is shared by all three modules so one `report(&[Breach])` names every
  failure of every check. That is `boundary.rs:63-70`, kept.
- The workspace root is `CARGO_MANIFEST_DIR` joined with `../..`, the same rule
  every other target uses. A test binary's working directory is not something to
  rely on.

---

## PHASE-03 — `crates/goad`: Slint in the graph, the markup, and the element tree

**Objective:** the renderer crate exists, `ui/app.slint` compiles, the generated
tree is quarantined in one module, the cheap test tier runs headless with a guard
test that proves the query API is live, and A-4 has a number.

**Surfaces:** `flake.nix`, root `Cargo.toml` (`[workspace.dependencies]`,
`workspace.members`), `crates/goad/**`, `docs/slices/002/notes.md`.

**Entry**
- EN-1 — PHASE-02's exit criteria are discharged and `just check` exits 0.
- EN-2 — the dependency set this phase may add is exactly `slint`, `slint-build`,
  the Slint testing dev-dependency and `pkgs.dejavu_fonts`. Anything else is S-8.

**Exit**
- EX-1 — **A-4's measurement protocol, run first, before any renderer content.**
  Add the member with its `slint` dependency and a trivial `lib.rs`, then: one
  cold `time just check` in a worktree after `cargo clean`, recorded and **not**
  thresholded; then three consecutive warm `time just check` runs with no source
  change between them, and the **median wall-clock** recorded. Apply §5.5's
  bands — ≤ 120 s continue; > 120 s and ≤ 300 s continue and raise a follow-up in
  `slice-002.md`; **> 300 s is S-4**.
- EX-2 — `flake.nix` carries
  `fontsConf = pkgs.makeFontsConf { fontDirectories = [ pkgs.dejavu_fonts ]; };`
  and `FONTCONFIG_FILE = fontsConf;` in the devshell environment, and changes
  nothing else. Verified in-shell, not by reading:
  `nix develop --command sh -c 'fc-list | grep -c DejaVu'` prints a non-zero
  count. Adding the package to `buildInputs` alone does nothing (D12).
- EX-3 — `crates/goad/Cargo.toml` matches §5.1's member row exactly, and it is
  EX-8a's skeleton: `[dependencies]` `goad-semantics`, `goad-shell`, **`jiff`**,
  **`serde_json`**, `slint`, `tokio` with `rt-multi-thread` and `sync`;
  `[dev-dependencies]` `slint` with its testing feature; `[build-dependencies]`
  `slint-build`. Every entry is `{ workspace = true }`. `slint` and `slint-build`
  are pinned **exactly** `= 1.17.1` in `[workspace.dependencies]`.
  `lints.workspace = true` and nothing else; `autotests = false`;
  `[[test]] name = "renderer" path = "tests/renderer/main.rs"`.

  `jiff` and `serde_json` are named because `crates/goad` names them directly and
  **a transitive dependency is not in the extern prelude**: `clock.rs` writes
  `jiff::Timestamp::from_nanosecond` and `ClockError::OutOfRange(jiff::Error)`
  (PHASE-06/EX-1); `Stimulus::event` builds `Event { … data: Value::Null }`, and
  `canonical.rs:495` types `data` as `serde_json::Value`; and
  `tests/support/driving.rs`, which PHASE-06/EX-9 includes into this target,
  carries `DEFAULT_POLL: jiff::SignedDuration`, `instant() -> jiff::Timestamp`
  and `serde_json::json!`. Without them PHASE-06 does not compile, and no
  manifest is in PHASE-06's Surfaces. **This is not S-8:** both are already in
  `[workspace.dependencies]`, so a `{ workspace = true }` entry adds nothing to
  the graph. §5.1's member row and this criterion both say so (review-plan F-4).
- EX-4 — `crates/goad/build.rs` passes exactly `ui/app.slint` to
  `slint_build::compile_with_config(path, CompilerConfiguration::new().with_debug_info(true))`,
  returns `Result`, and contains no `.unwrap()` and no `.expect()`.
- EX-5 — `crates/goad/ui/app.slint` is §5.2's block — one file, `OptionRow`,
  `WindowMode`, `PromptWindow`, `Tray` — and it compiles. `Tray` declares
  `image`, `hover-text` and `shown` and **binds** the inherited `icon`, `tooltip`
  and `visible` to them, because redeclaring is an error and a literal binding is
  folded to a constant with no setter generated (F-28, E-4).
- EX-6 — `src/generated.rs` is the only module wrapping `include_modules!()`,
  carrying one `#![expect(…, reason = …)]` inner attribute. The lint list is
  corrected to what this build actually emits, and every addition or removal is
  recorded in the sheet with the diagnostic that forced it (A-1, an authorised
  local adjustment). A suppression **outside** that module, a lint the workspace
  table does not set, or a `[lints]` table in `crates/goad/Cargo.toml` is **S-2**.
- EX-7 — `src/lib.rs` declares `pub mod generated;` and grows one `pub mod` line
  per later phase, reaching §5.1's ten at PHASE-08. There is no `main.rs` yet.
- EX-8 — `tests/renderer/main.rs` exists as `#[cfg(test)] mod tree;`, the cheap
  tier is initialised with `init_no_event_loop()`, and `cargo test -p goad` runs
  with **no display server** and opens no socket.
- EX-9 — items 6, 7, 8, 9 and 10 pass.

**Verification**
- VT-1 — item 6: a guard test asserting a known element **is** found. Every
  element-tree assertion in this slice rests on the query API being live; without
  debug info they all pass vacuously.
- VT-2 — item 7: title, body and one activatable control per option, in the order
  they were given, with `accessible-item-count` matching the model — never
  `find_all().len()`, because the list virtualises (E-2).
- VT-3 — item 8: activating a control fires `chosen` with the right `OptionId`
  **and** the right view token, including the case where two options share a label
  (R-14, D10). Selection is by `accessible_description`, never by label.
- VT-4 — item 9: the empty state, asserted by presence **and** absence.
- VT-5 — item 10: every absence assertion demonstrated against a deliberately
  broken implementation. Each break-and-revert output pasted.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — A-7's negative control: a bad `accessible-role` in `ui/app.slint` fails
  the build script. Broken, output pasted, reverted. Without it EX-5 proves
  nothing.
- VA-3 — the timing record pasted: the cold run, the three warm runs, the median,
  and which band it fell in.

**STOP**
S-1, S-2, S-3, S-4, S-7, S-8 — this is the phase in which five of the eight can
first fire. In particular: `CompilerConfiguration::with_debug_info` missing, or
VT-1 failing against a build that used it, is **S-3** and not a matter for the
environment-variable fallback A-3 rejects.

**Notes for the implementer**

- Do the timing first. Once the renderer content is in, the number measures
  something else.
- A `flake.nix` change does not reach a running shell. Every command after EX-2
  runs through `nix develop --command …` or in a shell re-entered after the
  change.
- The real proof of the font is not `fc-list`; it is the cheap test tier not
  panicking inside the font stack at component construction. With an empty
  fontconfig every test panics there, naming nothing useful.
- §9's two test-target rules are not optional: a `tests/…` target is a `main.rs`
  with `#[cfg(test)]` module declarations (`clippy::tests_outside_test_module`,
  nine diagnostics from one file), and a `#[test]` returning `Result` must
  actually use `?` (`clippy::unnecessary_wraps`, six from one file). A test with
  nothing to unwrap returns `()`, and that is the house standard's own intent.
- `StyledText` declares no accessible role, so the body's **content** is asserted
  on `Presentation` in the mapper tier (PHASE-04) and the element tree asserts
  only that a `StyledText` is present in prompt mode.
- Debug info stays on in release. It costs +0.05%, and without it the query API
  returns empty.

---

## PHASE-04 — The mapper and the tray rasteriser

**Objective:** a canonical `View` becomes a `Presentation` through one exhaustive
match that degrades and reports but never refuses, and the tray icon is a rule
with numbers rather than an asset.

**Surfaces:** `crates/goad/src/{lib.rs, view_model.rs, diagnostics.rs}`,
`crates/goad/tests/renderer/{main.rs, mapper.rs, tray.rs}`,
`docs/slices/002/notes.md`.

**Entry**
- EN-1 — PHASE-03's exit criteria are discharged and `just check` exits 0.
- EN-2 — `crates/goad`'s cheap test tier runs and PHASE-03/VT-1's guard test
  passes.

**Exit**
- EX-1 — `view_model.rs` declares `present`, `Presentation`, `PresentationOption`,
  `Body`, `Undrawn`, `ContentForm` and `Presentation::body_is_degraded` exactly as
  §5.2 writes them. The `match` over `View` has **no** `_` arm, so a second
  `View` variant is a compile error naming this file.
- EX-2 — §5.2's six-row content table is implemented and is the only statement of
  the rule: a rejected markdown body reaches `Body::Plain` carrying the source
  **and** yields `Undrawn::MarkdownUnsupported`; an accepted one is retained as
  `Body::Rich`, parsed once; `Html` and `Uri` reach `Body::Plain` and yield
  `Undrawn::ContentForm`. Nothing the backend authored is omitted.
- EX-3 — `diagnostics.rs` exists carrying `#![deny(clippy::arithmetic_side_effects)]`,
  `TrayState`, `ICON_EDGE`, `IDLE`, `FAULT` and
  `tray_icon(TrayState) -> slint::Image` with §5.4's pinned geometry — centre
  `128`, `OUTER_SQ` `14_400`, `INNER_SQ` `5_184` for `Idle` and `0` for `Fault`,
  the 4×4 sample grid at `(8x + 2i + 1, 8y + 2j + 1)`, and both boundary
  comparisons inclusive (DF-1 places this here).
- EX-4 — **not one bare arithmetic operator** appears in the rasteriser: every
  product and sum is `saturating_*`, and the one division is
  `checked_div(16).unwrap_or(0)`. No expression uses `as`.
- EX-5 — `lib.rs` gains `pub mod diagnostics;` and `pub mod view_model;`.

**Verification**
- VT-1 — item 4: `present` over every row of §5.2's content table, asserting both
  what is rendered and what `undrawn` names, plus a non-empty `Opt::fields()`
  yielding `Undrawn::OptionFields` with the right count.
- VT-2 — item 5: a markdown corpus covering the parse/reject boundary, asserting
  that a reject degrades rather than refuses, that a rejected body still reaches
  the glass, and that an accepted parse is **retained** rather than re-run.
- VT-3 — item 16: `tray_icon(Idle)` and `tray_icon(Fault)` are both 32×32; the
  centre pixel `(16, 16)` has `alpha == 0` for `Idle` and `alpha == 255` for
  `Fault`; a ring pixel such as `(16, 4)` is fully opaque in **both**; the corner
  `(0, 0)` has `alpha == 0` in both; and no file under `crates/goad/` is an image.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — §5.4's *shapes* table applied, not rediscovered: rule 5 (`# Errors` on
  every exported fn returning `Result`), rule 7 (a loop binding never reuses the
  name of the thing it iterates) and rule 9 (the rasteriser's arithmetic) are
  confirmed by reading before the first clippy run, and the run pasted.

**STOP**
S-1, S-8.

**Notes for the implementer**

- `slint::StyledText` derives `Debug, PartialEq, Clone, Default`, so retaining the
  parse costs `Presentation` none of its derives. It is a value type: holding one
  needs no event loop, which is what keeps this tier cheap.
- `Content::Text` goes through `from_plain_text` and `Content::Markdown` through
  `from_markdown` **explicitly**. The implicit `.slint` string coercion is
  `from_plain_text`, which is right for one and silently wrong for the other.
- `ContentForm` carries no payload on purpose: the bytes already reach the glass
  as the body, and carrying them here too would render one value twice.
- `Opt::fields()` returns `&Fields`, so the mapper counts without an accessor
  `canonical.rs` does not grant. Nothing mints an `OptionId` — its constructor is
  `pub(super)` and the design does not want the one it does not have.
- The two icon states differ in **form as well as hue** — an annulus and a filled
  disc — so the pair survives a monochrome panel and a colour-blind viewer. The
  assertions are values because §5.4 pinned the numbers; do not restate the
  numbers in the test.

---

## PHASE-05 — The diagnostic surface and the reception seam

**Objective:** every fact an exchange produces has exactly one renderer and one
place, the three display bounds are applied last and counted in characters, and
`receive` is the only consumer of an `Outcome` in the process.

**Surfaces:** `crates/goad/src/{lib.rs, diagnostics.rs, reception.rs}`,
`crates/goad/tests/renderer/{main.rs, reception.rs}`,
`docs/slices/002/notes.md`.

**Entry**
- EN-1 — PHASE-04's exit criteria are discharged and `just check` exits 0.
- EN-2 — `view_model.rs` declares `Undrawn` and `Presentation`, which
  `Diagnostics::of` and `receive` both consume.

**Exit**
- EX-1 — `diagnostics.rs` carries `Reported`, `Refused` (three variants),
  `Diagnostics` with private `lines` and `fault`, `of`, `refused`, `is_clear`,
  `lines`, `state`, `tooltip`, `BUSY_NOTICE`, `line_to`, `report_platform`, and
  the escaping **`Display` adapter** — not a `String` accumulator, which is two
  denied lints each suggesting the other (shapes rule 8).
- EX-2 — the pipeline runs in §5.4's order and the order is the contract: compose,
  decode (stderr only, `from_utf8_lossy`), escape, **bound last**. The three
  bounds are 4096 characters for the stderr line, 1024 for every other line, and
  120 for the tooltip, counted with `chars().count()` after escaping.
- EX-3 — every string in §5.4's *The exact strings* is present verbatim, and
  §5.4's *Once, exactly* table holds: no `source()` walk, no marker that repeats
  the reason it marks, no second statement of a number another module owns.
- EX-4 — ordering, severity and retention are as §5.4 states: failure, cleanup,
  undrawn, discarded, then the stderr capture line and the stderr line;
  `TrayState::Fault` iff anything other than stderr alone; a later outcome
  replaces the diagnostics wholesale.
- EX-5 — `reception.rs` carries `receive`, `Received` and `Prepared` (DF-2), and
  `receive` is total, pure, panic-free, and the **only** place an `Outcome` is
  destructured in `crates/goad/src/` — confirmed by grep and recorded.
- EX-6 — `lib.rs` gains `pub mod reception;`.

**Verification**
- VT-1 — item 13a: an `Outcome` carrying a view whose body is rejected markdown
  yields a `Received` whose diagnostics are **not** clear and whose `prepared` is
  `Some`; and its converse, a clean view yielding clear diagnostics.
- VT-2 — item 13b: one `Outcome` carrying a failure, a cleanup failure, an undrawn
  body, a discard and stderr at once, asserting all six lines in the stated order.
- VT-3 — item 13c: stderr alone leaves `state()` at `Idle` and `is_clear()` false;
  each of failure, cleanup, discard, undrawn and a `Refused` alone raises `Fault`.
- VT-4 — item 13d: for each of the three limits, *limit − 1* untouched, *limit*
  untouched, *limit + 1* truncated with the marker naming exactly one elided
  character (`docs/memory/a-bound-is-not-tested-at-the-bound.md`).
- VT-5 — item 13e: non-UTF-8 stderr decodes lossily; a newline becomes `\n` and a
  backslash `\\`; a multi-line `StyledTextFromMarkdownError` becomes one line;
  non-Latin text and combining marks pass verbatim.
- VT-6 — item 13f: stderr whose **escaped** form exceeds the limit while its raw
  byte length does not is truncated. This is the assertion that fails if the bound
  is applied to bytes.
- VT-7 — item 13g: a `Discarded::Schedule` whose raw is `"18:00:00"` renders that
  substring exactly once, for both the `NotAString` arm and a raw-carrying arm;
  a `Failure::Backend(BackendError::Io(_))` renders the OS message once and not
  again from `source()`.
- VT-8 — item 13h: a `Captured` that is both transport-truncated and
  display-truncated produces the capture line **and** the marker, with different
  text, capture line first.
- VT-9 — item 13i: all four tooltip forms, including the `(+n more)` plural and
  the 120-character projection of a long line 0.
- VT-10 — item 13j: `Diagnostics::refused` produces one line per `Refused`
  variant, with the same prefix as an `Outcome` failure.
- VT-11 — item 13k: `ContentForm` renders "HTML" and "a URI", and an HTML body
  reaches `Body::Plain` with the value the backend sent.
- VT-12 — item 13l: a rejected body yields `Body::Plain` plus one
  `MarkdownUnsupported`; an accepted one yields `Body::Rich`; and
  `body_is_degraded()` is false for `Body::Rich` and `Content::Text`, true for
  rejected markdown, HTML and URI.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — break-and-revert on VT-7: add a `source()` walk to the reducer, confirm
  VT-7 goes red, revert. A rule about what code must *not* do needs a test that
  fails when someone adds it back.

**STOP**
S-1, S-8.

**Notes for the implementer**

- `print_usage`, `report_startup` and the `USAGE` const are **PHASE-08's**: they
  need `StartupError`, which lands with its construction sites. `line_to` and
  `report_platform` land here because `SlintGlass` (PHASE-07) is
  `report_platform`'s only caller and a `pub` item in a library is not dead code.
- `report_platform` takes the **rendered** detail, `&str`, not
  `&slint::PlatformError`. The `Display` happens at the one site that has the
  value, which keeps this module testable with a literal.
- `std::io::Write` is **not** imported: the `impl std::io::Write` bound supplies
  `write_fmt`, and the import trips `unused_imports` at `deny`.
- The outlet spelling is `match writeln!(sink, "{line}") { Ok(()) | Err(_) => () }`.
  `.ok();` and `drop(..)` also pass the table — the design says so and says why
  the `match` is chosen anyway. Do not "simplify" it.
- `Diagnostics::of` takes `Reported` **by value and destructures it in the first
  statement**: a body that only reads a by-value parameter is
  `clippy::needless_pass_by_value` at `deny`.
- `undrawn` is a separate argument rather than a field of `Reported`, because it
  is the mapper's fact and `Outcome` has no field for it. That is what makes I-2
  structural rather than remembered.

---

## PHASE-06 — The controller, the fold, and the failure case table

**Objective:** the presentation transition is a total function of
`(entry point, view, failure)` with cleanup in none of its rows, and every failure
in SPEC-001's taxonomy has been driven through one retained `Host` and read off
the diagnostics the production reducer produced.

**Surfaces:** `crates/goad/src/{lib.rs, clock.rs, wire.rs, controller.rs}`,
`crates/goad/tests/renderer/{main.rs, table.rs}`, `tests/support/driving.rs`,
`crates/goad-shell/tests/integration/harness.rs`,
`tests/backends/answers-as-instructed.sh`, `docs/slices/002/notes.md`.

**Entry**
- EN-1 — PHASE-05's exit criteria are discharged and `just check` exits 0.
- EN-2 — `receive`, `Received`, `Prepared`, `Diagnostics` and `Refused` all exist,
  because `absorb` calls `receive` and folds what it returns.

**Exit**
- EX-1 — `clock.rs` carries `Clock` (a `fn` pointer), `ClockError` with its two
  variants and their exact `Display`, `std::error::Error` with the **default**
  `source()`, and `wall_clock` with a `# Errors` section. It builds its instant
  from `SystemTime` and `jiff::Timestamp::from_nanosecond`, **not**
  `jiff::Timestamp::now()`, which would unify jiff's `std` feature into stratum
  1's build (D25's known residue, already avoided).
- EX-2 — `wire.rs` carries `Command` — `Evaluate`, `Choose { view, option }`,
  `OpenDiagnostics`, `CloseDiagnostics`, and **no** `Shutdown` — and `Stimulus`
  with `kind()` and `event(now)`. `Wire` and `Cancel` are PHASE-07's.
- EX-3 — `controller.rs` carries `Surface`, `Focus`, `Shift`, `Exchanged`,
  `Frame`, `Controller` with exactly the four retained fields, `new` **with an
  `impl Default` beside it**, `absorb`, `refuse`, `answer` (with `# Errors`),
  `open_diagnostics`, `close_diagnostics`, `engage`, `frame`, and the free `stamp`
  helper.
- EX-4 — `surface()` is **derived** from `(focus, shown)`, never stored: three
  inputs, three outputs, no combination unnamed.
- EX-5 — the seven rows of §5.4's reducer are implemented as a total match on
  `(Exchanged, prepared.is_some(), refused)` with no `_` arm and no
  `unreachable!()`. Row 7 is `Replaced`. `cleanup` appears in no row.
- EX-6 — `engage()` is the only setter of `engaged` and `absorb` clears it
  unconditionally, whatever the `Shift`.
- EX-7 — `answer` returns `UserResponse { option, values: BTreeMap::new() }` and
  answers with the **retained** `ViewId`; `ViewId::new` is called from nowhere in
  `crates/goad`.
- EX-8 — `answers-as-instructed.sh` gains the three `@lingers*` arms exactly as
  §12.1 writes them, including `@lingers-with-a-view`'s **pinned** body: a title,
  exactly one option, no fields, no body content, and `"next_check": "120
  minutes"`.
- EX-9 — `tests/renderer/main.rs` gains `#[cfg(test)] mod table;` and the literal
  `#[cfg(test)] #[path = "../../../../tests/support/driving.rs"] mod driving;`,
  and the §12.8 cut is re-settled so that **every** item in `driving.rs` is called
  by both including targets. Anything the `goad` target does not call moves back
  into `crates/goad-shell/tests/integration/harness.rs`; anything it needs that
  stayed there moves in (PL-4).
- EX-10 — item 12 runs: §12.9's array in §12.2's sequence, through **one**
  retained `Host`, with §12.6's stated exemption for the `Spawn` cohort, ending on
  `invocations(&log) == instructions.len()`.
- EX-11 — `lib.rs` gains `pub mod clock;`, `pub mod controller;` and
  `pub mod wire;`.

**Verification**
- VT-1 — item 12 in full: every row's exact `Display` text on its channel, every
  row's `shift`, `refused` bit, schedule instant and invocation count, and the
  trailing `respond(A)` / `evaluate(@lingers-with-a-view)` / `respond(B)`
  exchanges as rows of the array like any other.
- VT-2 — the seven reducer rows asserted as `Shift` values directly, including
  row 5 (reached by handing `absorb` an outcome carrying `Failure::State`) and
  row 7 (asserted on a constructed `Outcome`, which is what makes the arm total
  rather than a panic).
- VT-3 — `Frame::busy` is `false` after `absorb`, on a successful outcome and on
  a failed one. The negative control is the one that matters: an `absorb` that
  does not clear `engaged` disables every control for the rest of the process.
  Break, confirm red, revert, paste.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — the `driving.rs` / `harness.rs` cut as it now stands, listed item by
  item, with any item moved in either direction named and the reason given.

**STOP**
S-1, S-8.

**Notes for the implementer**

- §12.1 is the vehicle and it is settled: one `Host<ProcessBackend>` over
  `bash <backends>/answers-as-instructed.sh <log> <instruction…>`, folding each
  row through **`Controller::absorb`** — the production reduction — and reading
  `controller.frame().diagnostics.lines()`. `absorb` calls `receive` and an
  `Outcome` is not `Clone`, so calling both is not available.
- Instruction accounting is the trap: the script hands out instructions by
  *invocation* index, and the two state refusals never invoke it. Order the list
  by the exchanges that reach a process.
- `now` is `2026-08-23T04:12:00Z` for every `evaluate` and `04:14:00Z` for every
  `respond`. The schedule expectation is **per row**, not one rule, which is why
  §12.4 gives it a field.
- Five sentinels write the child's pid to stderr, so five rows carry a `stderr:`
  line whose text is a pid. `Expect::Unpinned` exists for exactly those.
- Row 5 is not reachable *through the controller* — the renderer's token and
  `Host`'s state are written in the same fold — and reaching it by construction is
  the design's own instruction, not a shortcut.
- §12.9 is the array, preserved verbatim, and it is there so nobody derives
  thirty-three ids from prose. Transcribe it; do not re-derive it.

---

## PHASE-07 — The glass, the wiring, and back-pressure

**Objective:** one total `present`, one `Wire` that refuses to block, and a
`Cancel` that is level-held — everything `serve` composes, before `serve`
exists.

**Split from the original PHASE-07 at the seam its own objective stated
(PL-10).** `serve`, cancellation and items 11a–d, 11h and 14a–d are **PHASE-10**,
which executes next. This phase writes no loop.

**Surfaces:** `crates/goad/src/{lib.rs, wire.rs, glass.rs, install.rs}`,
`crates/goad/tests/renderer/{main.rs, wiring.rs}`,
`docs/slices/002/notes.md`.

**Entry**
- EN-1 — PHASE-06's exit criteria are discharged and `just check` exits 0.
- EN-2 — `Controller`, `Frame`, `Command`, `Stimulus`, `Clock` and `Diagnostics`
  all exist; this phase composes them into a glass and a wire and adds nothing to
  them.

**Exit**
- EX-1 — `glass.rs` carries the `Glass` trait with one **infallible, total**
  `present`, and `SlintGlass` as its only implementation, holding `window`, `tray`
  and `Rc<VecModel<OptionRow>>`. `SlintGlass::new` writes the tray's `image` and
  `hover-text` **before returning**, because the tray registers nothing until a
  non-empty image is assigned.
- EX-2 — `present` writes every property from the frame on every call — `set_vec`
  on the process-lifetime `VecModel`, the `ModelRc` re-handed, heading, body,
  degradation, busy, mode, the diagnostic lines, the tray's `image` and
  `hover-text` — then shows or hides. `notice` is written `""` here and set from
  nowhere else in the trait. A `show()` or `hide()` `Err` goes to
  `report_platform(&error.to_string())` and `present` returns; the process keeps
  running, and there is no de-duplication.
- EX-3 — `wire.rs` gains `Wire` with a **hand-written** `Debug` (`slint::Weak`
  implements none, and `missing_debug_implementations` is `deny`), `Wire::new` as
  the only constructor, `send` using `try_send` with `Full` writing
  `BUSY_NOTICE` to `notice` and `Ok(()) | Err(TrySendError::Closed(_)) => ()` as
  **one** arm, and `stop`. `Cancel` is level-held over
  `tokio::sync::watch::<bool>`, with `new` **plus an `impl Default`**, `stop`, and
  `stopped(&self)`.
- EX-4 — `install.rs` carries `pub fn install(&PromptWindow, &Tray, &Wire)` with
  §5.4's six installations, each owning its own named `Wire` clone. It is `pub`
  and in the library, because item 14e drives it from a `tests/` target.
- EX-6 — `lib.rs` gains `pub mod glass;` and `pub mod install;`.
- EX-7 — items **11e, 11f, 11g and 11i** pass. Items 11a–d and 11h need `serve`
  and are PHASE-10/EX-7's.

**Verification**
- VT-5 — item 11e: `Refused::UnknownOption` and `Refused::NoClock`, each with no
  backend contact, the presentation retained, and one diagnostic line.
- VT-6 — item 11f: the five DT transitions, read as `Surface` from the frame
  **and** as the element tree.
- VT-7 — item 11g: back-pressure. A second command sent while the channel is full
  sets `notice` to `BUSY_NOTICE`, does not enter `Diagnostics`, and the next
  `present` clears it.
- VT-9 — item 11i: `busy` returns to `false` after a successful exchange and after
  a failed one, with the option controls reading `accessible_enabled == true`.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — break-and-revert on VT-9's negative control: an `absorb` that does not
  clear `engaged` leaves every control disabled. Pasted.
- VA-3 — the expectation budget: `crates/goad/src/` contains **no** `#[expect]`
  outside `generated.rs`. If it contains one, the sheet argues it and the count is
  recorded against S-1, **which leaves two spendable** — the stop fires on the
  third (DF-7).

**STOP**
S-1, S-8.

**Notes for the implementer**

- `Cancel::stopped` keeps its `-> impl Future` shape and trips
  `manual_async_fn` on nothing, because it clones its receiver before the async
  block.
- The six `install` clones get six distinct binding names for **readability**, not
  because a lint requires it: `let wire = wire.clone();` compiles clean under this
  table. That is written down so the next reader does not "simplify" it and be
  right.
- Nothing is read back out of a Slint property to build a response. The two option
  strings are the only values that travel outward and back, and both are matched
  against retained state.
- `Cancel` lands here with **no consumer in production code** until PHASE-10.
  `crates/goad` is a library (D28), so `dead_code` does not fire on `pub` items;
  but item 11g exercises `Wire` and nothing this phase exercises `Cancel` beyond
  its own construction, so `Cancel` has unit coverage only until PHASE-10's
  items 14a–d arrive. That is the price of the seam, and it is stated rather than
  discovered.

---

## PHASE-10 — `serve`, and the stop that drops the exchange

**Objective:** one loop both tiers call, and a stop request that drops the
exchange it interrupts rather than waiting for it.

**PHASE-07 split in two (PL-10).** This phase executes **after PHASE-07 and
before PHASE-08**; the id is 10 because ids are immutable and are never
renumbered. PHASE-07 landed everything `serve` composes; this phase writes the
loop and nothing else.

**Surfaces:** `crates/goad/src/{controller.rs, wire.rs}`,
`crates/goad/tests/renderer/{main.rs, wiring.rs}`, `docs/slices/002/notes.md`.

**On the criterion ids below.** They are the ones these criteria carried in the
original PHASE-07, kept rather than renumbered so that anything citing
PHASE-07/VT-10 needs only its phase corrected. The gaps — no EX-1…EX-4, no
EX-6, no VT-5…VT-7, no VT-9, no VA-2 — are PHASE-07's criteria, not missing ones.

**Entry**
- EN-1 — PHASE-07's exit criteria are discharged and `just check` exits 0.
- EN-2 — **mechanical, which is what makes the seam usable:** `Glass`,
  `SlintGlass`, `install`, `Wire` and `Cancel` all exist and are reachable from a
  `tests/` target; `Controller`, `Frame`, `Command`, `Stimulus`, `Clock` and
  `Diagnostics` exist from PHASE-06. `serve` composes them and adds nothing to
  them.

**Exit**
- EX-5 — `controller.rs` gains `Pending`, `Ending`, `Served` and `serve` as an
  ordinary `async fn` carrying **no attribute at all**, with §5.4's loop body:
  `select! { biased; … }` in both places, `Pending` built before the borrow, the
  exchange future built from it, and `Served { ending, host, controller, glass }`
  after the loop.
- EX-7 — items **11a–d, 11h and 14a–d** pass.

**Verification**
- VT-1 — item 11a: the seven reducer rows folded through `Controller::absorb` and
  then read from the element tree in the same `block_on`, with rows 1–4 and 6
  driven by a real `tokio::process::Command` backend.
- VT-2 — item 11b, the row AC-6 turns on: a successful `respond` with `view: None`
  closes the interaction and the window goes away; a successful `evaluate` with
  `view: None` while an interaction is outstanding leaves the question on screen.
  **Both, in one test**, or AC-6 is being asserted in the shape F-14 showed wrong.
- VT-3 — item 11c: a failed `respond` keeps the window, and a retry on the same
  `ViewId` then succeeds.
- VT-4 — item 11d: R-33 staleness, not merely the redraw. A `Choose` bearing view
  **A**'s token queued behind a slow exchange, an intervening `evaluate` returning
  view **B**, then the queued `Choose` refused as `Refused::SupersededView` with
  the invocation log not advancing. The negative control is the same sequence
  without the intervening evaluate, where the click is answered.
- VT-8 — item 11h: the test calls `serve` — the same function `main` wraps — so a
  loop-body change cannot pass here and fail in production.
- VT-10 — item 14a: with an exchange in flight against `@hang` and a **2 s**
  configured timeout, tripping `Cancel` ends the task in **under 250 ms**,
  measured from `Cancel::stop()` to `serve` returning.
- VT-11 — item 14b: `serve` **returns** a `Served`, so the exchange future was
  dropped rather than abandoned unpolled.
- VT-12 — item 14c: a stop request arriving in the same poll as a ready command
  wins, and one arriving *before* `stopped()` is first awaited still ends the loop.
- VT-13 — item 14d: on `Ending::Stopped` the receiver's buffer is left unread — a
  command queued behind the exchange produces no further invocation.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-3 — the expectation budget: `crates/goad/src/` contains **no** `#[expect]`
  outside `generated.rs`. If it contains one, the sheet argues it and the count is
  recorded against S-1, **which leaves two spendable** — the stop fires on the
  third (DF-7).

**STOP**
S-1, S-5, S-8. **S-5 is this phase's**: item 14a measuring above 250 ms against a
2 s timeout means shutdown is awaiting the exchange, which AC-12 forbids, and it
is not a threshold to relax.

**Notes for the implementer**

- `serve` carries **no** `#[expect(clippy::future_not_send)]`. The lint does not
  reach this signature — it drops `Send` obligations that mention a type parameter
  at the top level, and `serve` is generic over `B` and `G` — so the attribute
  would be *unfulfilled*, and `unfulfilled_lint_expectations` is an error under
  `-D warnings`. The plain-`fn`-returning-`impl Future` shape does not dodge the
  lint either and costs `clippy::manual_async_fn` as well. Both measured.
- One future, not two branches. Duplicating the cancellation `select!` per entry
  point states the contract twice; boxing it puts a wrapper between `select!` and
  the exchange, which weakens "the exchange future is dropped" into a claim about
  the box.
- What AC-12 can honestly observe is what the host holds. That the child is gone
  is **not** asserted: it would be a race, and a flaky gate is worse than an
  honest one.

---

## PHASE-08 — Startup, the entry point, and the event-loop tier

**Objective:** goad is a process a person can run: it finds its configuration,
reports every startup failure in its own voice on stderr and exits 2, and a
window-close gesture ends the loop through the one path the design allows.

**Surfaces:** `crates/goad/src/{lib.rs, startup.rs, diagnostics.rs, main.rs}`,
`crates/goad/Cargo.toml` (the `event_loop` test target),
`crates/goad/tests/renderer/{main.rs, startup.rs}`,
`crates/goad/tests/event_loop/{main.rs, closing.rs}`, `crates/goad/README.md`,
`crates/goad-boundary/tests/checks/{main.rs, structure.rs}`,
`docs/slices/002/notes.md`.

**Entry**
- EN-1 — **PHASE-10's** exit criteria are discharged and `just check` exits 0.
  PHASE-10 executes between PHASE-07 and PHASE-08 (PL-10).
- EN-2 — `serve`, `install`, `SlintGlass`, `Wire`, `Cancel` and `wall_clock` all
  exist, because `start` constructs every one of them and constructs nothing else.

**Exit**
- EX-1 — `startup.rs` carries `Launch`, `StartupError` and `arguments`, and
  nothing else. `StartupError` has the eight variants §5.4 names, `Display` text
  byte-identical to §5.4's table, `std::error::Error` with the **default**
  `source()`, and no `PartialEq`.
- EX-2 — `arguments(argv, env)` is pure over both, skips `argv[0]` **itself**, and
  implements §5.4's four-row table, with `XDG_CONFIG_HOME` honoured only when set
  and **absolute** and `HOME` used as given.
- EX-3 — `diagnostics.rs` gains `USAGE` (one `const`, no trailing newline),
  `print_usage` (stdout) and `report_startup` (stderr), with §5.4's exact text.
  A usage error names the flag and does **not** reprint the usage block.
- EX-4 — `main.rs` holds `main`, `run` and `start` and **nothing else**, exactly
  as §5.4 writes them, and constructs all eight `StartupError` variants (DF-4).
  `main` returns `ExitCode` and is the one place an exit code is chosen; every
  startup failure exits **2**.
- EX-5 — `quit_event_loop` has exactly **one** call site in `crates/goad/src/`,
  with no exception carved out for `Wire::send`'s `Closed` arm.
- EX-6 — `crates/goad/Cargo.toml` declares
  `[[test]] name = "event_loop" path = "tests/event_loop/main.rs"`, whose
  `main.rs` is `#[cfg(test)] mod closing;` and which uses
  `init_integration_test_*`.
- EX-7 — `crates/goad/README.md` carries §5.4's **validated** `window-rule` block
  under a heading naming niri, with §5.4's two sentences and nothing else.
- EX-8 — `crates/goad/src/lib.rs` now reads exactly §5.1's ten `pub mod` lines, in
  that order, and nothing else.
- EX-9 — `crates/goad-boundary/tests/checks/main.rs` gains
  `#[cfg(test)] mod structure;` (PL-6), and items 14e, 14f and 17 pass.

**Verification**
- VT-1 — item 17: `StartupError`'s `Display` for each of its eight variants and
  `ClockError`'s for both of its, asserted **verbatim** against §5.4's table; the
  usage block produced by one `const` and byte-identical wherever it appears; a
  usage error's text not containing the usage block; `StartupError::source()` and
  `ClockError::source()` both `None`; and the argument table's rows — zero
  arguments with `XDG_CONFIG_HOME` set/unset/empty/relative/absolute, `HOME`
  unset/empty/relative/absolute, one argument, `-h`, `--help`, two arguments.
  Every row writes `argv` the way `std::env::args_os()` yields it, **program name
  first**.
- VT-2 — item 14e: a real close request runs `Wire::stop` and returns
  `KeepWindowShown`, `serve` returns, then `quit_event_loop` runs and
  `run_event_loop_until_quit` returns. This is the only case that needs the
  integration-test platform, and it is why the target exists.
- VT-3 — item 14f: a source scan asserting `quit_event_loop` has exactly one call
  site in `crates/goad/src/`, and that the renderer holds no `tokio::spawn`
  handle — mirroring slice 001's `the_only_spawn_is_the_child`.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — the binary, once, by hand: `cargo run -p goad -- --help` exits 0 and
  prints `USAGE` byte-identically; `cargo run -p goad -- a b` exits 2 and prints
  `goad: too many arguments: …`. Neither reaches a Slint call, so neither needs a
  display. **No test** asserts an exit code by running the binary — that is what
  §9 item 17 forbids, and this is an agent check, not a test.
- VA-3 — the two `StartupError` sites that discard an error do so with a **named**
  binding: `.map_err(|_returned| StartupError::Enqueue)` and
  `.map_err(|_negative| ClockError::BeforeEpoch)`. `clippy::map_err_ignore` is
  `deny` and fires on the wildcard.

**STOP**
S-1, S-2, S-8. Additionally:
- **PS-4** — `dead_code` fires on a `StartupError` variant despite `start`
  constructing it. That means DF-4 is wrong in a direction nothing here
  anticipated, and the shape of the fix — an `#[expect]`, a restructure, or a
  design change — is not a phase's to choose.

**Notes for the implementer**

- `run` cannot use `?` in `main`, so the fallible half is `run`/`start` and the
  exit code is chosen once. `--help` returns `Ok(())` and therefore exits 0 with
  no second exit path.
- The environment is **passed in**, not reached for:
  `arguments(std::env::args_os(), &|name| std::env::var_os(name))`. The closure is
  not redundant — `var_os` is generic over `K: AsRef<OsStr>` and a generic fn item
  does not coerce to `&dyn Fn(&str) -> Option<OsString>`. Measured, as was the
  arity.
- `slint::set_xdg_app_id("goad")` runs **before** any component is constructed:
  the app icon comes from the app id and the `icon` property is silently dropped.
- `_entered` is a named binding with a leading underscore, not `_`. `let _ = …`
  drops the `EnterGuard` immediately and `let_underscore_must_use` refuses it
  anyway. Without the guard the first poll of a `tokio::process` future on the
  Slint thread panics with *there is no reactor running*.
- The `JoinHandle` from `spawn_local` is bound and dropped: dropping it does not
  drop the future, which is why nothing is retained.
- `run_event_loop_until_quit`'s `Err` is the one `Platform` site that is not a
  startup failure, and it still exits 2. That is recorded in §5.4 as a case
  sitting in the wrong bucket by decision; do not add a ninth variant to fix it.
- The README's `open-floating` and `open-focused` are a **recommendation**, not a
  requirement. Only the app id is goad's to state.

---

## PHASE-09 — The drafts, the restatement sweep, and the clean-clone gate

**Objective:** every document this slice owns is true about what shipped, the
harvest is written, and `just check` exits 0 from a clean clone under
`nix develop` — the second half of AC-1.

**The slice is not closeable at PHASE-09/EX-6.** AC-15 is discharged only by the
audit's Reconciliation table: promoting `canon-delta.md` and `draft-policy.md`
needs explicit user endorsement and happens at audit (`docs/AGENTS.md:38`, HARD
STOP 1). Ten phases green is not the same event as fifteen acceptance criteria
discharged, and the status table must not be read as though it were.

**Surfaces:** `docs/slices/002/{slice-002.md, canon-delta.md, draft-policy.md,
notes.md, plan.md, plan-log.md, design-log.md}`, `crates/goad/README.md` if it
has drifted.

**Not touched:** `docs/slices/002/design.md` — it is a record of intent at a point
in time, and where the implementation departed and the design stands as written,
that is *Design drift not reconciled* for `audit.md` (`docs/AGENTS.md:137`).
`docs/specs/`, `docs/policy/`, `docs/adr/` and `CLAUDE.md` are **not** touched and
nothing is promoted: that is audit's, with explicit endorsement (HARD STOP 1,
`docs/AGENTS.md:38`).

**Entry**
- EN-1 — PHASE-08's exit criteria are discharged and `just check` exits 0.
- EN-2 — every phase's sheet in `notes.md` records its exit and verification
  criteria as discharged, or records a stop.

**Exit**
- EX-1 — `canon-delta.md`'s seven entries and `draft-policy.md` read true against
  the tree as it shipped. Every divergence is either repaired in the draft or
  written into `notes.md` Findings for audit, with the id of the entry it touches.
  Neither document is promoted.
- EX-2 — `slice-002.md` is consistent with what shipped: its Scope paths, its
  Governing canon section, and every acceptance criterion's wording. Its Stage is
  advanced.
- EX-3 — the restatement sweep, with its mechanical halves run as commands rather
  than read: `cargo metadata --no-deps` names the four members; the six `[[test]]`
  target names match §5.1's table; `just -n check` matches
  `draft-policy.md`'s command block; `crates/goad/src/lib.rs` matches §5.1's ten
  lines. Every count, path, target name and command named in `plan.md` and
  `slice-002.md` is checked against the tree, and each divergence is recorded.
- EX-4 — `notes.md`'s `## Harvest` is written **in place**: Produced, Learned,
  Open, with the `docs/memory/` candidates named — at minimum the compositor
  window rule, the `makeFontsConf` trap, and the `#[path]`-into-workspace-root
  helper pattern.
- EX-5 — `just check` exits 0 **from a clean clone under `nix develop`**: a fresh
  `git clone` or `git worktree add` of the branch, entered with `nix develop`,
  with the command sequence and the wall-clock pasted.
- EX-6 — A-4's warm median re-measured on the finished tree and recorded against
  §5.5's bands, beside PHASE-03/EX-1's number. A move across a band boundary is
  recorded; **> 300 s is S-4** even here.

**Verification**
- VA-1 — EX-5's clean-clone run pasted in full.
- VA-2 — the acceptance-criterion walk: every AC in `slice-002.md` walked against
  the criterion this plan's Coverage table names for it, with the evidence
  recorded. An AC whose named criterion did not in fact discharge it is a finding
  for `audit.md`, not a repair here.
- VA-3 — the surfaces diff: `git diff --name-only <slice base> HEAD` against the
  union of every phase's declared Surfaces. `<slice base>` is the sha
  PHASE-01/EN-5 recorded — `git merge-base main HEAD`, taken before the split —
  and is read from the PHASE-01 sheet, not re-derived. Undeclared paths are the audit's
  strongest lead and this phase hands them over rather than tidying them away.

**STOP**
S-4, S-8. And the standing one: **nothing under `docs/specs/`, `docs/policy/` or
`docs/adr/` is created or edited, and no draft is promoted.** If reconciliation
appears to require it, that is audit's work and it needs the user.

**Notes for the implementer**

- `docs/AGENTS.md:40` — a slice does not close holding an unpromoted draft. This
  phase's job is to make the drafts *promotable*, not to promote them.
- CD-5 and `draft-policy.md` land together or not at all: applying CD-5 alone
  leaves `CLAUDE.md` pointing at nothing, and promoting the policy alone leaves
  two claimants to the gate. Say so again in the handover.
- The harvest is ids and one-line hooks only. Never restate content that lives
  elsewhere.

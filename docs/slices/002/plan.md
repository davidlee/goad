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

Nine phases take the repository from one crate with no renderer to a workspace of
four members whose stratum 3 draws a `choice` view, answers it, reports every
failure in SPEC-001's taxonomy, and stops without waiting for an exchange it has
abandoned.

The spine is D1: **the split lands first and alone, on a tree with no renderer in
it.** PHASE-01 is that split and nothing else. PHASE-02 rebuilds the
workspace-wide invariant checks the split displaces. PHASE-03 puts `slint` in the
dependency graph for the first time. PHASE-04 through PHASE-08 build stratum 3
inward-out — the pure functions first, the loop next, the process entry point
last. PHASE-09 is the documentary close-out the slice owes before audit.

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
   renderer code; A-2's expectation budget is **three, all unspent**, and the
   third distinct `#[expect]` outside the generated-code quarantine is S-1.

## Sequencing & rationale

**Why the split is alone.** D1. Every measurement behind it was taken on a tree
with no renderer; putting the split's failures and the renderer's failures in one
diff leaves no way to tell them apart. It is also the cheapest audit available of
the least-audited passage in the design: §5.1's artifact map states 111 renames
file by file, and executing it either succeeds or fails loudly on the first
`cargo build`. PHASE-01's exit criteria are written to **catch a wrong map**
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

**Why the failure table sits at PHASE-06 and the loop at PHASE-07.** §12.1 folds
every row through `Controller::absorb` and reads
`controller.frame().diagnostics.lines()`. It needs no glass and no event loop, so
it lands with the controller it drives. Item 11 needs the element tree and the
production `serve`, so it lands with them. Splitting there puts thirty-three rows
of transcription in one session and nine loop observables in another.

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
`crates/goad/tests/renderer/main.rs`. One agent, one phase, in sequence.

**Size.** PHASE-01, PHASE-06 and PHASE-08 are the three heavy sessions — a
111-file relocation, a 33-row table, and the entry point plus the event-loop
tier. PHASE-04 is the lightest. None is expected to exceed one session including
bookkeeping; if one does, that is a finding for `notes.md`, not a reason to skip
the sheet.

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
  does not require it there, and item 12 is its only consumer. PHASE-01 therefore
  exits with **91** byte-identical renames, which is the dry run's own number and
  the cleanest AC-2 evidence available.
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

## Findings against the design

Raised during planning, not repaired in `design.md` (`docs/AGENTS.md:137` — the
design is a record of intent). Each is resolved here so no phase has to invent an
answer, and each is a candidate for the audit's *Design drift not reconciled*.

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

## Coverage

Every acceptance criterion in `slice-002.md`, mapped to the phase and criterion
that discharges it. A gap here is a gap in the plan.

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-01/EX-2 establishes the six-command gate at the split commit; every phase's VA-1 re-runs it; PHASE-09/EX-5 is the clean-clone run under `nix develop` |
| AC-2 | PHASE-01/EX-3, EX-4 and EX-11 (the map walked in both directions, the evidence list written); PHASE-02/EX-12 supplies the argument for the one substantively rewritten file |
| AC-3 | instrument 1 — PHASE-01/EX-10 (negative control, broken and reverted); instrument 4 — PHASE-01/EX-2 (the gate's third command); instruments 2 and 3 — PHASE-02/EX-4, EX-7, EX-8, VT-1, VT-2; the counting rule and the unenforced residue — PHASE-02/VA-3 |
| AC-4 | PHASE-03/VT-2 (item 7, at the element tree); PHASE-07/VT-1 (item 11a, end to end through `serve`) |
| AC-5 | PHASE-03/VT-3 (item 8, the markup half: the right `OptionId` and the right view token, two options sharing a label); PHASE-07/VT-4 (item 11d, R-33 staleness and its negative control) |
| AC-6 | PHASE-03/VT-4 (item 9, the empty state); PHASE-07/VT-2 (item 11b, both `view: null` readings in one test) |
| AC-7 | PHASE-06/VT-1 (item 12, the whole taxonomy through one retained `Host`, with §12.6's stated exemption); PHASE-07/VT-3 (item 11c, a failed `respond` keeps the question and a retry succeeds) |
| AC-8 | PHASE-05/VT-4, VT-6, VT-7, VT-8 (items 13d, 13f, 13g, 13h — the bound at the bound, the ordering of decode/escape/bound, once-exactly, two distinguishable truncations); PHASE-08/VT-1 (item 17's `source()` clauses) |
| AC-9 | PHASE-04/VT-1, VT-2 (items 4 and 5); PHASE-05/VT-11, VT-12 (items 13k, 13l) |
| AC-10 | PHASE-03/VT-1 (item 6, the guard test) and EX-8 (the cheap tier runs with no display server) |
| AC-11 | PHASE-03/VT-4 and VT-5 (items 9 and 10 — presence *and* absence, each demonstrated against a deliberately broken implementation) |
| AC-12 | PHASE-07/VT-10…VT-13 (items 14a–d, cheap tier); PHASE-08/VT-2 (item 14e, the event-loop tier) and VT-3 (item 14f, the structural count) |
| AC-13 | PHASE-02/EX-6 and VT-3 (members enumerated, `.slint` and `.rs`, the string-aware cut, all six positive controls) |
| AC-14 | PHASE-02/VT-3 places the scan; every phase's VA-1 re-runs it, and PHASE-04/EX-3's "no file under `crates/goad/` is an image" is its neighbour (PHASE-04/VT-3) |
| AC-15 | **not fully in this plan.** PHASE-09/EX-1 makes `canon-delta.md` and `draft-policy.md` true about what shipped and records every divergence; promotion and endorsement are audit's, per `docs/AGENTS.md:38` and HARD STOP 1 |

---

## PHASE-01 — The workspace split: three members, the relocation, and the six-command gate

**Objective:** the single crate is a workspace of three members, every file
§5.1's artifact map moves has moved, the gate is §5.6's six commands, and the map
has been walked in both directions against what actually happened.

**Surfaces:** `Cargo.toml` (becomes the workspace root), `crates/goad-semantics/**`,
`crates/goad-shell/**`, `crates/goad-boundary/**`, `src/**` (removed),
`tests/**`, `justfile`, `docs/slices/002/notes.md`, `docs/slices/002/plan-log.md`.

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
  §5.1 (whole), §5.6, §12.8, and `research.md`'s dry-run section.

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
- EX-3 — **the map, walked forward.** For every row of §5.1's split table, the
  destination path exists and the source path does not. Recorded row by row.
- EX-4 — **the map, walked backward.** `git diff --find-renames --name-status
  <pre-split commit> HEAD` names no path the map does not. A file that had to
  move and is not a row of the map is **S-6**, not an improvisation.
- EX-5 — exactly **91** of the renames are byte-identical (`R100`): the 77
  protocol fixtures and the 14 backend scripts, `answers-as-instructed.sh`
  included (PL-3). Every other changed file's change is confined to `use`/`mod`
  lines, a path constant, or a manifest entry — the map's "change permitted"
  column — with the sole exception of `boundary.rs`, whose change here is the
  `src/` ÷ `tests/` division D17 requires and nothing else.
- EX-6 — four `[[test]]` targets exist, with the map's names and paths:
  `goad-semantics` / `protocol` / `tests/protocol/main.rs`; `goad-shell` /
  `integration` / `tests/integration/main.rs`; `goad-shell` / `shape` /
  `tests/shape/main.rs`; `goad-boundary` / `checks` / `tests/checks/main.rs`.
  Each is a `main.rs` of `#[cfg(test)] mod` declarations and nothing else, with
  the map's module lists. The map's other two targets — `renderer` and
  `event_loop` — are **deliberately deferred to PHASE-03 and PHASE-08**, because
  D1 forbids `crates/goad` existing before Slint does (PL-1).
- EX-7 — `tests/support/driving.rs` exists at the workspace root and is included
  by `crates/goad-shell/tests/integration/main.rs` through the literal
  `#[path = "../../../../tests/support/driving.rs"]`, with `#[cfg(test)]` at the
  declaration site. It holds exactly §12.8's host-driving list — `scripted` and
  what it rests on (`backend`, `marker`, `logging_backend`), `invocations`,
  `config`, `host`, `instant`, `quiet_event`, `describe_outcome`, `choice`,
  `answer_first_option`, `presented`, and the single restated `CLEANUP_LIMIT`
  with its keep-in-sync note — and `crates/goad-shell/tests/integration/harness.rs`
  holds exactly §12.8's transport list.
- EX-8 — `[workspace.lints]` carries the pre-split `[lints.rust]` and
  `[lints.clippy]` blocks **verbatim** (diff them against `git show
  <pre-split>:Cargo.toml`), and every member manifest carries
  `lints.workspace = true` and no other `[lints]` content (D8). Every member sets
  `autotests = false` and declares its test targets explicitly.
- EX-9 — the `shell` feature is gone, not relocated: no member declares a
  `[features]` table, `grep -rn 'feature *= *"shell"'` over `Cargo.toml`,
  `crates/` and `tests/` returns nothing, and no `[[test]]` carries
  `required-features`.
- EX-10 — **instrument 1, observed rather than asserted.** Add
  `use goad_shell::host::Host;` to a `crates/goad-semantics/src/` file and confirm
  `cargo build -p goad-semantics` fails with `error[E0433]`; revert. Repeat with
  `use tokio::process::Command;`. Both outputs pasted into the sheet.
- EX-11 — the AC-2 evidence list is written into the phase sheet: every moved
  path with its similarity index, and one line per non-identical file naming the
  map row that permits its change.
- EX-12 — the `justfile` header comment cites `docs/slices/002/draft-policy.md`
  (the slice's working authority, `docs/AGENTS.md:36`) and `design.md` §5.6, and
  no longer cites `docs/slices/001/design.md` §9. `CLAUDE.md` is **not** edited:
  repointing it is CD-5 and lands at audit, paired with the policy's promotion.

**Verification**
- VT-1 — `cargo test --workspace` runs the same total number of tests as the
  pre-split tree. Record both numbers. A test file that fails to be re-declared
  in its new `main.rs` disappears silently, and this is what catches it.
- VT-2 — `goad-boundary`'s `checks` target runs the domain-vocabulary scan over
  the `src/` of all three members and still carries the vacuity guard that fails
  when pointed at a renamed-away root (`boundary.rs:293-305`, retained).
- VA-1 — `just check` run and its output pasted into the phase sheet, with the
  warm wall-clock beside the EN-3 baseline.
- VA-2 — the two map walks (EX-3, EX-4) and the `git diff --find-renames
  --name-status` output pasted.
- VA-3 — `just -n check` output pasted beside §5.6's block. Compare the **command
  sequence**, not the characters: §5.6 is a fenced block and `just -n` prints
  neither comments nor line wrapping (slice 001's plan-review F-9, against this
  criterion's own earlier wording).
- VA-4 — `cargo tree -p goad-semantics --edges normal,build,dev` contains no
  `tokio` node and no `toml` node. Pasted.

**STOP**
S-6 and S-8 from `design.md` §5.5, plus two the design does not name because they
are local to this phase:
- **PS-1** — a production source under `crates/goad-semantics/src` or
  `crates/goad-shell/src` needs a change beyond its `use`/`mod` lines or a path
  constant. The dry run says no production file needed one; a file that does is
  R4's signal that the split is a redesign, and AC-2 says so.
- **PS-2** — `just check` will not reach 0 and the remaining failure is not an
  import path, a manifest entry, or a test-target declaration.

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
- **`publish = false` on every member**, or once in `[workspace.package]` with
  `publish.workspace = true` in each. `clippy::cargo_common_metadata` is in the
  `cargo` group at `deny`, and `publish = false` is what silences it — the
  existing root manifest says so in a comment worth carrying forward. `edition`,
  `version`, `license` and `repository` are worth inheriting the same way.
- `clippy.toml` and `rustfmt.toml` stay at the workspace root and are read from
  there for every member. Do not copy them into members.
- `crates/goad-boundary` has **no dependencies** in this phase. `toml` arrives at
  PHASE-02 with the allowlist that needs it; it is already in
  `[workspace.dependencies]`.
- `boundary.rs`'s two configured scans become three, one per member's `src/`,
  written out by hand. That is the R7 shape the design retires — PHASE-02/EX-6
  replaces it with enumeration, and this phase does not attempt it.
- Moving `Scan`, `Breach`, `mentions`, `code_of` and `report` into a library
  makes them public API, so `clippy::missing_errors_doc` fires on `Scan::run`.
  A `# Errors` section is the answer; an `#[expect]` is S-1 budget spent for
  nothing.
- `tests/protocol/transport_shape.rs` cannot stay in a stratum 1 target: it names
  a stratum 2 source. It becomes `crates/goad-shell/tests/shape/main.rs` with
  `#[cfg(test)] mod transport_shape;` beside it. This is the upward reach the
  single crate hid, and finding it is ADR-002's Verification section working.
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
- EX-3 — `crates/goad/Cargo.toml` matches §5.1's member row exactly:
  `[dependencies]` `goad-semantics`, `goad-shell`, `slint`, `tokio` with
  `rt-multi-thread` and `sync`; `[dev-dependencies]` `slint` with its testing
  feature; `[build-dependencies]` `slint-build`. `slint` and `slint-build` are
  pinned **exactly** `= 1.17.1` in `[workspace.dependencies]`. `lints.workspace =
  true` and nothing else; `autotests = false`; `[[test]] name = "renderer" path =
  "tests/renderer/main.rs"`.
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

## PHASE-07 — The glass, `serve`, and the wiring

**Objective:** one total `present`, one loop both tiers call, and a stop request
that drops the exchange it interrupts rather than waiting for it.

**Surfaces:** `crates/goad/src/{lib.rs, wire.rs, controller.rs, glass.rs,
install.rs}`, `crates/goad/tests/renderer/{main.rs, wiring.rs}`,
`docs/slices/002/notes.md`.

**Entry**
- EN-1 — PHASE-06's exit criteria are discharged and `just check` exits 0.
- EN-2 — `Controller`, `Frame`, `Command`, `Stimulus`, `Clock` and `Diagnostics`
  all exist; `serve` composes them and adds nothing to them.

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
- EX-5 — `controller.rs` gains `Pending`, `Ending`, `Served` and `serve` as an
  ordinary `async fn` carrying **no attribute at all**, with §5.4's loop body:
  `select! { biased; … }` in both places, `Pending` built before the borrow, the
  exchange future built from it, and `Served { ending, host, controller, glass }`
  after the loop.
- EX-6 — `lib.rs` gains `pub mod glass;` and `pub mod install;`.
- EX-7 — items 11a–i and 14a–d pass.

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
- VT-5 — item 11e: `Refused::UnknownOption` and `Refused::NoClock`, each with no
  backend contact, the presentation retained, and one diagnostic line.
- VT-6 — item 11f: the five DT transitions, read as `Surface` from the frame
  **and** as the element tree.
- VT-7 — item 11g: back-pressure. A second command sent while the channel is full
  sets `notice` to `BUSY_NOTICE`, does not enter `Diagnostics`, and the next
  `present` clears it.
- VT-8 — item 11h: the test calls `serve` — the same function `main` wraps — so a
  loop-body change cannot pass here and fail in production.
- VT-9 — item 11i: `busy` returns to `false` after a successful exchange and after
  a failed one, with the option controls reading `accessible_enabled == true`.
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
- VA-2 — break-and-revert on VT-9's negative control: an `absorb` that does not
  clear `engaged` leaves every control disabled. Pasted.
- VA-3 — the expectation budget: `crates/goad/src/` contains **no** `#[expect]`
  outside `generated.rs`. If it contains one, the sheet argues it and the count is
  recorded against S-1's budget of three.

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
- `Cancel::stopped` keeps its `-> impl Future` shape and trips
  `manual_async_fn` on nothing, because it clones its receiver before the async
  block.
- The six `install` clones get six distinct binding names for **readability**, not
  because a lint requires it: `let wire = wire.clone();` compiles clean under this
  table. That is written down so the next reader does not "simplify" it and be
  right.
- One future, not two branches. Duplicating the cancellation `select!` per entry
  point states the contract twice; boxing it puts a wrapper between `select!` and
  the exchange, which weakens "the exchange future is dropped" into a claim about
  the box.
- Nothing is read back out of a Slint property to build a response. The two option
  strings are the only values that travel outward and back, and both are matched
  against retained state.
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
- EN-1 — PHASE-07's exit criteria are discharged and `just check` exits 0.
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
  union of every phase's declared Surfaces. Undeclared paths are the audit's
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

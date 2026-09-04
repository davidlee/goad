# Plan log — Slice 002

Append-only working record for the plan stage. Survives compaction and
interruption; `plan.md` itself stays clean. Never rewrite an entry — supersede
it with a later one.

Decisions here are taken under the **standing autonomy grant** (`design-log.md`,
2026-09-04 *Gate autonomy* and 2026-09-05), and are recorded as a user's would
be: Asked / Decided / Why / Rejected / Consequence. Findings from an adversarial
review of the plan would live in `review-plan.md`, not here. Design-shaped
decisions taken while planning are cross-posted to `design-log.md`.

## Decisions

### 2026-09-05 — PL-1: the split lands three members, not four

- **Asked:** `design.md` §5.1's artifact map states four member manifests and six
  `[[test]]` targets. D1 states that the split lands *first, alone, before Slint
  enters the tree*. `crates/goad` carries `slint`, `slint-build` and the Slint
  testing dev-dependency. Both cannot be true of one commit.
- **Decided:** the artifact map is the **slice's** end state, not PHASE-01's.
  PHASE-01 creates `goad-semantics`, `goad-shell` and `goad-boundary`, and four
  of the six test targets. `crates/goad` and the `renderer` / `event_loop`
  targets arrive at PHASE-03 and PHASE-08.
- **Why:** D1's grounds are evidence, not taste — every measurement behind the
  split was taken on a tree with no renderer, and a combined diff has no way to
  separate the split's failures from the renderer's. Nothing in the map depends
  on all four members existing at once; `workspace.members` is enumerated by
  `goad-boundary` precisely so a fourth can arrive later and be scanned without
  an edit.
- **Rejected:** creating `crates/goad` at PHASE-01 as a stub with no `slint`
  dependency — it buys nothing, adds a member with no content for two phases, and
  makes "before Slint enters the tree" a claim about a manifest rather than about
  a dependency graph. Also rejected: treating the map's four-manifest line as an
  error in the design; it is correct about the end state and says so.
- **Consequence:** PHASE-01/EX-6 names the two deferred targets explicitly, so a
  reader is not left to infer that four-of-six is deliberate. PHASE-09/EX-3
  re-checks the full six against the map on the finished tree.

### 2026-09-05 — PL-2: `goad-boundary`'s rewrite is PHASE-02, not PHASE-01

- **Asked:** the map marks `tests/protocol/boundary.rs` (345 lines) the one file
  *substantively rewritten*, and §5.6 says the `tokio` source grep is retired
  "in the same change" as the manifest allowlist that replaces it. Does that
  change belong in the split?
- **Decided:** PHASE-01 relocates the machinery into
  `crates/goad-boundary/src/` and the token lists and controls into
  `tests/checks/` — D17's shape — and changes nothing else. PHASE-02 rewrites it:
  `members`, `scan` with `extensions` and the string-aware `code_of`, `manifest`
  with the allowlist, the stratum 1 purity scan, and the retirement of the three
  direction tokens in the same change as their replacement.
- **Why:** two reasons, and the second is the load-bearing one. A 600-line
  rewrite inside a phase whose whole value is being a *pure relocation* destroys
  the evidence AC-2 and R4 depend on — "a content change beyond import paths and
  manifest entries is evidence of redesign" is only a signal if the split commit
  has almost none. And §5.6's "in the same change" is a constraint on the
  *retirement*, not on the *relocation*: PHASE-02 satisfies it exactly.
- **Rejected:** doing both in PHASE-01 (one session, two unrelated risks, and the
  map's audit contaminated by the rewrite's noise); and retiring the three
  direction tokens at PHASE-01 with nothing yet replacing them, which would leave
  one phase in which no instrument holds "no runtime in stratum 1's manifest".
- **Consequence:** for one phase the vocabulary scan is configured by hand, once
  per member — the R7 shape §8 retires. PHASE-02/EX-6 replaces it with
  enumeration, and PHASE-01's notes say so rather than leaving it to look like an
  oversight.

### 2026-09-05 — PL-3: `answers-as-instructed.sh` gains its `@lingers*` arms at PHASE-06

- **Asked:** the map's split row permits `answers-as-instructed.sh` to change,
  naming the three `@lingers*` arms §12.1 writes out. Item 12, at PHASE-06, is
  their only consumer. Where do they land?
- **Decided:** PHASE-06. PHASE-01 moves all fourteen backend scripts
  byte-identical.
- **Why:** the map's column *permits* a change; it does not require it in the
  split. Landing them at PHASE-01 puts a content change unrelated to relocation
  into the one commit whose AC-2 argument should be "nothing changed but import
  paths", and drops the byte-identical count from the dry run's 91 to 90 for no
  gain. The arms are inert until item 12 exists.
- **Rejected:** landing them at PHASE-01 to match the map's row literally, and
  landing them at PHASE-08 with the event-loop tier, which separates them from
  the only test that reads them.
- **Consequence:** PHASE-01/EX-5 asserts exactly **91** byte-identical renames,
  which is the dry run's own number and therefore a check with a known answer.

### 2026-09-05 — PL-4: the `driving.rs` cut is made at PHASE-01 and re-settled at PHASE-06

- **Asked:** §12.8 defines `tests/support/driving.rs` as *the intersection of what
  the two tiers use*, and says an included helper neither tier calls is dead code
  that fails `clippy --workspace --all-targets -- -D warnings`. The second tier —
  `crates/goad`'s `renderer` target — does not exist until PHASE-06. An
  intersection cannot be computed against a set that does not exist.
- **Decided:** PHASE-01 makes the cut against §12.8's enumerated host-driving
  list, with `crates/goad-shell/tests/integration` as its only consumer — every
  item is live there, because every one of them is a helper that target already
  calls. PHASE-06, when the `renderer` target first includes the same file,
  re-settles the boundary: any item the `goad` target does not call moves back
  into `harness.rs`, and any item it needs that stayed in `harness.rs` moves in.
- **Why:** §12.8 states the *rule*; the rule cannot be fully applied until both
  consumers exist, and applying it in two steps is the rule working rather than a
  deviation from it. The alternative — deferring the whole cut to PHASE-06 —
  removes a row from the map's split table and would make PHASE-01's forward walk
  incomplete.
- **Rejected:** carrying dead helpers behind `#[expect(dead_code)]` in the
  `renderer` target, which spends S-1 budget on a bookkeeping problem; and
  duplicating the helpers per tier, which is the second implementation §12.8
  exists to prevent.
- **Consequence:** PHASE-06/EX-9 and VA-2 make the re-settlement an explicit,
  recorded act with the moved items listed in both directions, rather than a
  silent edit inside a test-support file nobody reviews.

### 2026-09-05 — PL-5: three file placements, where the design states two different things

Cross-posted to `design-log.md`, because module placement is a design decision.
The findings themselves are `plan.md`'s DF-1, DF-2 and DF-3.

- **Asked:** §5.1's artifact map and the `// crates/goad/src/….rs` headers on
  §5.2–§5.4's code blocks disagree three times: the tray rasteriser
  (`tray_icon.rs` vs `diagnostics.rs`), `Prepared` (`reception.rs` vs
  `controller.rs`), and `Wire`/`Cancel`/`Command`/`Stimulus` (`controller.rs` vs
  `wire.rs`).
- **Decided, item by item rather than by a blanket rule:**
  - the rasteriser, `ICON_EDGE`, `IDLE`, `FAULT` and `TrayState` → **`diagnostics.rs`**;
  - `Prepared` → **`reception.rs`**;
  - `Wire`, `Cancel`, `Command`, `Stimulus` → **`wire.rs`**.
- **Why:** no blanket rule is honest, because each of the three disagreements is
  settled by a different fact. §5.1 quotes `lib.rs` as *the whole file* with ten
  `pub mod` lines, so an eleventh `tray_icon` module contradicts it, and
  `diagnostics.rs` already carries the `#![deny(clippy::arithmetic_side_effects)]`
  the rasteriser needs. `Prepared`'s full shape is written in §5.2's
  `reception.rs` block beside the `Received` that owns it, and the map's reading
  would make PHASE-05 depend on a type PHASE-06 lands — the phase order inverts
  for no gain. And if `Wire` and friends lived in `controller.rs`, `lib.rs` would
  declare an empty `wire` module.
- **Rejected:** "the map always wins" and "the declaring block always wins" —
  each is right twice and wrong once; and repairing `design.md`, which
  `docs/AGENTS.md:137` forbids mid-slice and which would rewrite intent to match
  a plan.
- **Consequence:** a fourth statement falls with them. §5.3's *"glass.rs is the
  ONLY file in the crate that names a generated type"* is false as the design
  itself writes it — `install.rs` takes `&PromptWindow` and `&Tray`, `Wire` holds
  `slint::Weak<PromptWindow>`, and under this decision `diagnostics.rs` returns a
  `slint::Image`. The load-bearing statement, and the one phases are held to, is
  that **glass.rs is the only file that reads or writes a generated component's
  properties.** All four are candidates for the audit's *Design drift not
  reconciled*.

### 2026-09-05 — PL-6: item 14f gets a fourth module in `goad-boundary`'s `checks` target

- **Asked:** §5.1 places validation item 14f — `quit_event_loop` has exactly one
  call site, and the renderer holds no `tokio::spawn` handle — in
  `goad-boundary::checks`, whose module list in the same table is
  `{vocabulary, purity, allowlist}`: one module per instrument, and 14f is none
  of them.
- **Decided:** `crates/goad-boundary/tests/checks/structure.rs`, declared
  alongside the other three, landing at PHASE-08 with the code it counts.
- **Why:** 14f is a source **count**, not a forbidden-token scan, so it is not a
  configured `Scan` and does not belong inside `vocabulary` or `purity`. Putting
  it in a fourth named module keeps `Scan`'s contract — fail on presence — intact,
  and keeps the three instrument modules readable as one instrument each.
- **Rejected:** folding it into `vocabulary` (a scan that fails on presence cannot
  express "exactly one"); and moving it into `crates/goad`'s `renderer` target,
  which contradicts §5.1's placement and would put a workspace-wide structural
  claim inside the member it is a claim about.
- **Consequence:** a one-line deviation from §5.1's module list for `checks`,
  recorded here and re-checked at PHASE-09/EX-3.

### 2026-09-05 — PL-7: no phase carries a VH criterion

- **Asked:** slice 001's plan used a VH criterion for the one thing an agent could
  not do — reloading the dev shell after a `flake.nix` change, so `deno` and
  `just` resolved. Slice 002 adds a font to `flake.nix` at PHASE-03 and has the
  same problem.
- **Decided:** no VH criteria anywhere. PHASE-03 runs every command after its
  `flake.nix` change through `nix develop --command …`, and EX-2 verifies the font
  in-shell — `nix develop --command sh -c 'fc-list | grep -c DejaVu'` — rather
  than by asking a person to confirm it.
- **Why:** these phases execute with no human present. A criterion nobody can
  discharge is not a criterion; it is a phase that cannot end green.
- **Rejected:** keeping a VH and letting the executing agent mark it "assumed" —
  which is the shape of concession `docs/AGENTS.md` §Execute forbids.
- **Consequence:** every entry, exit and verification criterion in `plan.md` is a
  command an agent can run or a file it can read.

### 2026-09-05 — PL-8: `startup.rs` lands with `main.rs`, whether or not the constraint that forces it still binds

- **Asked:** §5.4 states that all eight `StartupError` variants must be
  *constructed* in the phase that lands them, because `dead_code` is fatal under
  `-D warnings`. That was measured (`research.md` Thread 10) on a scratch crate
  built **before** F-30 made `crates/goad` a library plus a thin binary — and
  `dead_code` does not fire on `pub` items in a library target.
- **Decided:** PHASE-08 lands `startup.rs`, `main.rs`, the two remaining
  diagnostic outlets and item 17 together, which satisfies the constraint whether
  or not it still binds. `plan.md`'s DF-4 records the doubt; nothing in the plan
  rests on resolving it.
- **Why:** the standing lesson of this slice is that reading is not evidence. I
  can argue `dead_code` is now void; I have not measured it, and a phase boundary
  placed on an unmeasured argument is exactly the failure rounds 3 and 4 kept
  finding. Ordering the phase so the question cannot arise costs nothing.
- **Rejected:** splitting `startup.rs` out as an early, cheap, pure phase on the
  strength of the library-target reading; and adding `#[expect(dead_code, reason
  = …)]` pre-emptively, which spends S-1 budget against a lint that may never
  fire.
- **Consequence:** PHASE-08 is the heaviest renderer phase, and it is heaviest for
  a stated reason. If `dead_code` fires anyway despite `start` constructing every
  variant, that is PHASE-08's **PS-4** stop, because the fix would be a design
  question rather than a phase's.

### 2026-09-05 — PL-9: `boundary.rs`'s PHASE-01 destination is stated, not discovered

- **Asked:** `review-plan.md` F-10. §5.1's map row for the one file it marks
  *substantively rewritten* gives its destination as "`crates/goad-boundary/`,
  split across `src/` and `tests/` — below". "Below" is §5.6's three-module API,
  which is PHASE-02's shape. So PHASE-01/EX-3's forward walk had nothing to check
  for the row it matters most for, and PHASE-01 would have invented a file layout
  inside `crates/goad-boundary/src/` — the class of decision §5.1 exists to
  prevent (F-37).
- **Decided:** PHASE-02's shape **minus `manifest.rs`**, which needs a `toml`
  PHASE-01 does not have. `src/lib.rs` declares `pub mod scan;` and nothing else.
  `src/scan.rs` carries today's `Scan`, `Breach`, `mentions`, `report` and
  `Scan::run`, changed only as PHASE-01/EX-5c permits. `tests/checks/` carries
  `main.rs`, `vocabulary.rs` and `direction.rs`.
- **Why:** PHASE-02/EX-1 is then a restructure of a known starting point — add
  `members.rs`, add `manifest.rs`, move `code_of` in — rather than a discovery.
  And a two-module `src/` at PHASE-01 is honest about what exists: there is no
  allowlist yet, so there is no `manifest.rs` to put an empty file in.
- **Rejected:** a single `src/lib.rs` that PHASE-02 splits — it makes PHASE-02's
  first act a file split rather than a feature, and it puts the D17 division the
  map calls for at PHASE-02 instead of PHASE-01. Also rejected: creating an empty
  `manifest.rs` to match §5.6 exactly, which is a module with no content and no
  test, i.e. a vacuous shape.
- **Consequence:** PHASE-01/EX-3 has a destination to check for this row, and
  PHASE-02/EX-1's "declares exactly three modules" is a change to a known two.

### 2026-09-05 — PL-10: PHASE-07 splits at the seam its own objective states; the second half is PHASE-10

- **Asked:** `review-plan.md` F-12. PHASE-07 as written landed `glass.rs`,
  `install.rs`, the second half of `wire.rs` (`Wire`, `Cancel`, a hand-written
  `Debug`, back-pressure) **and** the whole `serve` loop with `select! { biased; }`
  in two places, then discharged thirteen verification groups including a
  real-process reducer walk, R-33 staleness with a negative control, back-pressure,
  two simultaneous-ready races, a 250 ms latency measurement and two
  break-and-revert transcripts. The plan's own Size paragraph named PHASE-01,
  PHASE-06 and PHASE-08 as the heavy sessions and did not name it.
- **Decided:** split at the seam the objective already stated. PHASE-07 keeps
  `glass.rs`, `install.rs` and `wire.rs`'s `Wire` / `Cancel` — items 11e, 11f,
  11g, 11i. **PHASE-10** takes `serve` and cancellation — items 11a–d, 11h,
  14a–d. Ids are immutable and are never renumbered (`plan.md`'s own preamble
  anticipates this), so PHASE-10 executes between PHASE-07 and PHASE-08 and
  PHASE-08/EN-1 names it.
- **Why:** a phase that overruns is a phase whose bookkeeping is done badly at
  the end, and this one had the highest verification density in the plan. The
  seam is real rather than arbitrary: everything in PHASE-07 is what `serve`
  composes, and PHASE-10's entry criterion is mechanical — those five items exist
  and the gate is green.
- **Rejected:** naming PHASE-07 a fourth heavy session and saying what gets
  dropped if it overruns. Nothing in it is droppable: every VT is an acceptance
  criterion's only evidence, and "drop something" is a concession
  `docs/AGENTS.md` §Execute forbids an agent to make alone.
- **Consequence:** ten phases, non-monotonic order, and one stated cost — `Wire`
  and `Cancel` land at PHASE-07 with no production consumer until PHASE-10.
  `crates/goad` is a library (D28), so `dead_code` does not fire on `pub` items;
  PHASE-07's notes say so rather than leaving it to be discovered.

### 2026-09-05 — PL-11: the member manifest skeleton is written out once

- **Asked:** `review-plan.md` F-28. PHASE-01's notes *offered* `publish = false`
  two ways ("on every member, or once in `[workspace.package]`"), called
  `edition`, `version`, `license` and `repository` "worth inheriting", and said
  nothing at all about the root's `description`, `keywords`, `categories` and
  `readme` — in the one place `clippy::cargo` at `deny` bites, and where under
  `CLAUDE.md` invariant 1 a member `description` is a place domain vocabulary
  could enter a file no scan reads.
- **Decided:** PHASE-01/EX-8a writes the skeleton out key for key, the way §5.6
  writes `goad-boundary`'s API out once so no phase invents a signature. Five
  keys inherited from `[workspace.package]` — `version`, `edition`, `license`,
  `repository`, `publish` — with `publish = false` stated **once**. No member
  carries `description`, `keywords`, `categories` or `readme`.
- **Why:** `publish = false` is what silences `cargo_common_metadata`
  (`Cargo.toml:12-15`, measured in slice 001), so the metadata keys buy nothing
  and cost an unscanned surface. Inheriting once beats repeating four times
  across four members.
- **Rejected:** carrying the root's metadata into `goad` alone as "the real
  package". Nothing is published; it would be the only member shaped differently,
  for no gain.
- **Consequence:** PHASE-03/EX-3 and PHASE-08's manifest work both cite EX-8a
  rather than restating keys.

### 2026-09-05 — PL-12: `boundary.rs`'s three member scans are one `#[test]` over three `Scan`s

- **Asked:** `review-plan.md` F-26. PHASE-01/VT-1 asked that
  `cargo test --workspace` run "the same total number of tests as the pre-split
  tree", while PHASE-01's own notes had `boundary.rs`'s two configured scans
  become three. Whether the count moves depends on a shape nobody had chosen, and
  a count equality is defeated by any legitimate change and satisfied by an
  illegitimate one that balances.
- **Decided:** two things. The criterion becomes a **set equality over test
  names** — `cargo test --workspace -- --list` after, diffed against the two
  pre-split `--list` runs, module prefixes stripped. And the shape is settled:
  **one `#[test]` iterating three `Scan`s**, so `boundary.rs` stays at five test
  functions.
- **Why:** the property VT-1 is proxying is "no test file silently failed to be
  re-declared in its new `main.rs`". A name-set diff states that directly,
  catches a missing `mod`, and tolerates a deliberate split. Settling the shape as
  well means the diff has nothing legitimate to absorb at PHASE-01 — the cheapest
  possible first run of the check.
- **Rejected:** three `#[test]` functions, one per member, which reads better in
  a failure line but multiplies the R7 shape PHASE-02 immediately retires;
  `Breach` carries the path, so a failure names the member either way.
- **Consequence:** PHASE-01/VT-1 is a diff of two command outputs rather than two
  numbers, and PHASE-02/EX-6 replaces the whole thing with enumeration anyway.

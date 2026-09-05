# Audit & reconciliation — Slice 002

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `a6ae617..af4c6f2` on branch `slice-002`. Ten phases: PHASE-01
(the split) through PHASE-09 (the documentary close-out), run in declared order
01, 02, 03, 04, 05, 06, 07, **10**, 08, 09.

**Question:** what would have to be true for this slice to be finished, and
which of it this audit checks.

For the slice to be finished: the workspace builds and the six-command gate is
green (AC-1); every relocated file moved unchanged or its change is argued
(AC-2); the four ADR-001 instruments plus the vocabulary scan plus the named
residue hold stratum 1's purity as `design.md` §5.1's counting rule states it,
not as a claimed sum (AC-3); a `choice` view is drawn and answered correctly
(AC-4, AC-5); `view: null` is read per interaction rather than per message
(AC-6); every row of SPEC-001's failure taxonomy leaves the host up and
answerable, with the one stated `BackendError::Spawn` exemption (AC-7); the two
arbitrary values are bounded at the glass, once each (AC-8); a `markdown` body
the parser rejects degrades rather than refuses the view (AC-9); the renderer's
tests run headless with a live guard against a `build.rs` regression (AC-10);
the empty state is asserted by presence and absence, against a broken
implementation (AC-11); a stop drops the in-flight exchange rather than
awaiting it (AC-12); the vocabulary scan is member-discovering and
string-literal aware (AC-13); no domain vocabulary anywhere (AC-14); and
`canon-delta.md` accounts for every canon movement the slice obliges, promoted
or abandoned in writing (AC-15) — which this document is the promotion of. Five
CLAUDE.md invariants hold: the host understands no domain; wire parsing is
permissive, internal representations canonical; the protocol is not narrowed
to what this renderer draws; a backend failure never takes the host down; and
strata run one way, `goad-semantics` never naming `goad-shell`.

What this audit checks, by the method `docs/AGENTS.md` states: all five
invariants, all fifteen ACs against evidence (a command run, a test name, a
`file:line`), every VT/VA in `plan.md` PHASE-01…10 (discharged or not, from
each phase's sheet in `notes.md`, with five spot-checked against the tree
across phases), the surface delta (paths touched vs. every phase's declared
Surfaces, redoing PHASE-09's VA-3 rather than trusting it), and the canon
movements CD-1…CD-7 plus `draft-policy.md` against what is actually in the
tree today (not what the drafts say should be there).

What this audit does **not** check: the code review's own adversarial line —
that is `review-code.md`'s brief, run in parallel by a separate reviewer, and
this document only points at its ledger and state. It also does not re-derive
PHASE-09's own sheet from scratch line by line; where a phase's discharge
record in `notes.md` is itself the evidence (a pasted command's output, a named
test), this audit takes the paste as the evidence and re-runs the command
itself rather than re-deriving the phase's bookkeeping. It does not touch
`review-code.md`, does not write the Verdict, and does not tick Closure.

<!-- This is the audit's scope — evidence, criteria, canon. The code review's
     own lines of attack belong in `review-code.md`'s Brief, not here. -->

## Evidence

<!-- What was run and what it said. Not a claim of correctness — the basis for
     one. -->

### Tests / checks

`nix develop --command just check` (warm, this session): **exit 0**, wall-clock
**8.697 s** real. Per-target `test result:` lines, `cargo test --workspace`:
`goad` lib 6, `goad` bin 0, `goad`/`event_loop` 1, `goad`/`renderer` 117,
`goad-boundary` lib 0, `goad-boundary`/`checks` 28, `goad-semantics` lib 25,
`goad-semantics`/`protocol` 5, `goad-shell` lib 17, `goad-shell`/`integration`
58, `goad-shell`/`shape` 6 — **263 passed, 0 failed** across eleven targets.
`cargo test -p goad-semantics` (the gate's third command, run again with only
its own manifest's features): lib 25 + `protocol` 5 = **30 passed, 0 failed**.
`deno check examples/typescript/backend.ts`: clean. `cargo clippy --workspace
--all-targets -- -D warnings`: clean. `cargo fmt --all --check`: clean.
`justfile:50` carries exactly one `cargo clippy` line — the second
feature-column line CD-7 describes as retired is in fact gone from the recipe.

`cargo tree -p goad-semantics -e normal,build,dev`: `jiff`, `serde`,
`serde_json` and their transitives only — **0 `tokio` nodes, 0 `toml` nodes,
0 `slint` nodes**. `cargo tree -p goad-boundary`: no `goad-*` member in its
tree — it depends on none of the three it scans, as D17 requires.

### The four instruments, and the vocabulary scan — break-and-revert

`cargo test -p goad-boundary` lists 28 tests across `allowlist` (8),
`purity` (3), `structure` (7) and `vocabulary` (10, including two vacuity
controls), naming both source location and behaviour rather than a bare
assertion.

Three plants, each applied, run, and reverted this session (`git diff` empty
and the suite green again after each revert):

1. **Domain word in `.slint` markup.** `crates/goad/ui/app.slint`, `"Check
   now"` → `"Check habit now"`. `vocabulary::no_workspace_member_names_the_
   users_domain` **FAILED**, naming
   `crates/goad/ui/app.slint:103: forbidden token \`habit\``. Reverted;
   10/10 vocabulary tests green.
2. **Direct `std::fs` reach in a stratum 1 source.** Appended a function
   calling `std::fs::read_to_string` to `crates/goad-semantics/src/lib.rs`.
   `purity::the_real_stratum_1_source_names_none_of_the_nine` **FAILED**,
   naming `crates/goad-semantics/src/lib.rs:11: forbidden token
   \`std::fs\``. Reverted; 3/3 purity tests green.
3. **`tokio` in stratum 1's manifest.** Added `tokio = { workspace = true }`
   to `crates/goad-semantics/Cargo.toml`. `allowlist::the_real_stratum_1_
   manifest_is_clean` **FAILED**, naming
   `crates/goad-semantics/Cargo.toml:0: forbidden token \`tokio\`` — file
   named, line **0**, and that is the instrument's own documented limit, not
   a defect: `manifest.rs:48` comments "not tracked: the parser this reads
   through carries no span" (also recorded at `notes.md:1031-1033`,
   `:3254-3255`). Reverted; 8/8 allowlist tests green.

Instrument 4 (`cargo test -p goad-semantics`) is verified above by running it,
not by breaking it — AC-3 states outright that it "rejects nothing". Instrument
1 (Cargo resolution at crate edges) is PHASE-01/EX-10's own negative control,
not repeated here; its result (`error[E0433]` on a planted `goad_shell`/`tokio`
import) is taken from that sheet.

### Acceptance criteria

Evidence gathered directly against the tree, not copied from PHASE-09's own
VA-2 walk (`notes.md:2996-3020`), which is treated here as a lead. Both walks
reach the same disposition.

| AC | met? | evidence |
|----|------|----------|
| AC-1 | met | `just check` exit 0 above; six commands, order matching `draft-policy.md` §Compliance and `justfile`; `just -n check` prints the same sequence (checked) |
| AC-2 | met | `git diff --name-status a6ae617 e3170b1`: **113** renames (`git diff -M` default threshold), **92** `R100` byte-identical, 24 additions, 3 deletions, 4 modifications — reproduces `canon-delta.md` CD-1's repaired figures exactly (command re-run this session, not trusted from the doc) |
| AC-3 | met | four instruments confirmed live and boundary-true by break-and-revert above (three of four) plus PHASE-01/EX-10 (Cargo resolution); vocabulary scan confirmed separately not one of the four; residue is stated as a review obligation in `draft-policy.md` §Verification, not enforced anywhere in the gate — consistent with the claim |
| AC-4 | met | `crates/goad/tests/renderer/wiring.rs:481` `mod rows` exercises the seven-row reducer table through a real `SlintGlass`; `tree.rs` (PHASE-03) asserts the element-tree half at the mapper boundary |
| AC-5 | met | `wiring.rs`'s `interaction` module: `a_click_naming_a_superseded_view_is_refused_with_no_backend_contact` and its negative control — R-33 staleness, no backend contact on a stale token |
| AC-6 | met | `wiring.rs:712` `view_null_follows_the_interaction_not_the_message` |
| AC-7 | met | `table.rs`'s 33-row driver (PHASE-06) folds the whole taxonomy through one retained `Host`, with the `BackendError::Spawn` exemption stated in `slice-002.md` AC-7 and built as its own row; `wiring.rs`'s item 11c (a failed `respond` keeps the question, a retry succeeds) |
| AC-8 | met | `reception.rs`'s VT-4/6/7/8 (PHASE-05) — bound at the bound, decode/escape/bound ordering, once-exactly, two distinct truncations; `startup.rs`'s `source_walk` (PHASE-08) |
| AC-9 | met | `mapper.rs:93` `rejected_markdown_degrades_to_plain_and_is_reported_undrawn`, asserting `Undrawn::MarkdownUnsupported`; `mapper.rs:204` corpus test straddling the parse/reject boundary |
| AC-10 | met | renderer tests run in this session's `just check` with no display server (headless CI-shaped invocation); PHASE-03's guard test on the query API (`notes.md:1238`) |
| AC-11 | met | PHASE-03 VT-4/VT-5 — presence and absence, each demonstrated against a deliberately broken build (`notes.md:1256,1260`) |
| AC-12 | met | `controller.rs:318,388` `select! { biased; … }`; `wiring.rs`'s cancellation module measures `Cancel::stop()` to `serve` returning at ~79-106 µs (four runs), three orders of magnitude under the 250 ms bound; `served.ending == Ending::Stopped` asserted at four sites |
| AC-13 | met | `vocabulary.rs` reads `workspace.members` (`a_glob_in_workspace_members_fails`, `a_scan_whose_directory_was_renamed_away_fails`) and carries the six positive controls plus two vacuity controls listed above by name |
| AC-14 | met | `vocabulary::no_workspace_member_names_the_users_domain` scans every member found from `workspace.members`; confirmed live by break-and-revert above |
| AC-15 | open, by design | not fully discharged by the plan (`plan.md:311`); this document's Reconciliation table is the promotion/abandonment record `docs/AGENTS.md:38` reserves for audit |

### Verification criteria

73 VT/VA discharge lines counted across the ten phase sheets in `notes.md`
(`grep -c '^| VT-\|^| VA-\|^- \*\*VT-\|^- \*\*VA-\|^\*\*VA-\|^\*\*VT-\|^- VT-\|^-
VA-'`), one row per phase's own table or bullet style — PHASE-01/02/03/06/07/08
use `| VT-n |` tables, PHASE-04/10 use bold bullets, PHASE-05/09 use plain
bullets. Every phase's Status-table entry reads `done`, none records a stop,
and each phase's own sheet pairs every VT/VA it declares with a result. No
undischarged VT or VA was found in any sheet.

Five spot-checked directly against the tree, chosen across phases rather than
re-reading the sheets that name them:

- PHASE-10/VT-1 (item 11a): `crates/goad/tests/renderer/wiring.rs:481`,
  `mod rows` — present.
- PHASE-10/VT-2 (item 11b): `wiring.rs:712`,
  `view_null_follows_the_interaction_not_the_message` — present.
- PHASE-04/VT-1,2 (markdown degradation, AC-9): `crates/goad/tests/renderer/
  mapper.rs:93,204` — present, and read above to confirm the assertion shape
  matches AC-9's wording, not merely the function's name.
- PHASE-02/VT-3 (AC-13's six positive controls): `crates/goad-boundary/tests/
  checks/vocabulary.rs` — all six named functions present, plus two vacuity
  controls (`a_member_directory_with_no_rust_or_slint_file_fails_naming_
  itself`, `a_glob_in_workspace_members_fails`) beyond the six AC-13 itself
  names.
- PHASE-10 cancellation (AC-12, items 14a-d): `controller.rs:318,388`,
  `select! { biased; … }` — present, matching the design's stated mechanism
  (drop, not await) rather than only the test's name.

### Surface delta

`git diff --name-status a6ae617 af4c6f2` (this session, not PHASE-09's cached
figures): 80 additions, 24 deletions, 6 modifications, 92 `R100` renames — 202
paths total. Every `crates/**`, `tests/**`, `Cargo.toml`, `Cargo.lock`,
`flake.nix`, `justfile`, `clippy.toml` path falls inside some phase's declared
Surfaces (`plan.md` PHASE-01…10, PL-17's one-arm extension of PHASE-10's
included).

Outside those directories, the diff carries fifteen paths. Six are documents
the slice's own Scope and every phase's bookkeeping duty declare —
`canon-delta.md`, `draft-policy.md` (`slice-002.md` Scope: "drafted here"),
`notes.md`, `plan.md`, `plan-log.md` (the plan and its log, kept current every
phase per `docs/AGENTS.md`), `slice-002.md` itself (PHASE-09/EX-2's stage
advance) — not undeclared. One is `src/lib.rs`, **deleted**: it held the
pre-split single-crate module tree (`pub mod semantics; #[cfg(feature =
"shell")] pub mod shell;` at `a6ae617:src/lib.rs`) and its removal is squarely
inside PHASE-01's declared Surfaces ("`src/**` → relocated"), confirmed by
`git log --follow -- src/lib.rs` showing no history after `e3170b1` (PHASE-01).

The remaining eight are the same set PHASE-09's own VA-3 found and classified
(`notes.md:3020-3040`), reproduced here independently and matching exactly:
`.gitignore`, `docs/roadmap.md`, `docs/slices/002/audit.md`,
`docs/slices/002/design.md`, `research.md`, `review-design.md`,
`review-plan.md`, `design-log.md` — all added in the design-stage commit
`e5aff57`, before `<slice base>`'s split successor, or (for `design-log.md`
and `audit.md`) declared by the slice's own Surfaces. No difference from
PHASE-09's finding: same eight paths, same classification, reached by an
independent `git diff` rather than by re-reading their table.

### Canon inconsistencies live in the tree now

- `CLAUDE.md:71-72` — "clippy in **both** feature columns, and a format check.
  Nothing is green until it exits 0, and a matrix checked in one column is
  unchecked." False today: `justfile:50` carries one `cargo clippy` line, and
  `cargo tree -p goad-semantics` above shows no feature gating `tokio`/`toml`
  out of stratum 1's build. This is CD-7's subject.
- `CLAUDE.md:75` — "The command block in `docs/slices/001/design.md` §9 is
  canonical and the `justfile` mirrors it: change §9 first, then the recipe."
  Points at a closed slice's design as the gate's canonical source; `docs/
  AGENTS.md` forbids retro-fitting a closed design, so this pointer has been
  stale since the split changed the gate's command scope. This is CD-5's
  subject; `draft-policy.md` is the promotable replacement.
- `docs/adr/002-single-crate-until-triggered.md:3` — `**Status:** accepted`.
  The workspace it describes as not-yet-existing (`crates/goad-semantics`,
  `crates/goad-shell`, `crates/goad`, `crates/goad-boundary`) exists in the
  tree today, and the ADR's own Verification section calls for it to be
  superseded, not amended, the day the split happens (confirmed: `Cargo.toml`
  `[workspace] members` lists all four). CD-1/CD-2 name the superseding ADR;
  it is drafted content in `canon-delta.md`, not yet written as `docs/
  adr/003-*.md` — endorsement is required first.

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`
- **State:** open — round 2 verified · outstanding blockers: none · awaiting endorsement: F-5 (follow-up), F-6 (doc-wrong, drop `named`), F-7 (contested: the raiser's route runs the binary, which `design.md` §9 item 17 forbids; a sink seam for `report_platform` needs endorsement)

## Verdict

The slice does what it set out to do. The split landed first and alone, and
it was a relocation: 113 renames, 92 byte-identical, every content change
argued (AC-2). Slint entered on its own commit at a warm gate cost of 2.1 s
(5.3 s on the finished tree), nowhere near S-4's band. A `choice` view is
drawn, answered, and its stale click refused locally; `view: null` follows the
interaction; every row of SPEC-001's failure taxonomy has been driven through
one retained `Host` and read off the production reducer; a stop drops the
in-flight exchange in ~100 µs against a 2 s timeout. Fourteen acceptance
criteria are met on evidence re-run here rather than copied. AC-15 is open
because promotion is this document's own last act.

Two adversarial rounds raised no blocker. The two majors were the same shape:
code that worked and a suite that did not hold it — the drawn body and the
degradation marker, and one 100 ms sleep standing in for an observed
precondition. Both are repaired and re-verified, with a named residual:
Slint's testing API exposes no accessor for a `StyledText`'s content, so the
"still shown" half of AC-9 is held to the `Presentation` boundary and argued
for the last hop.

Accepted knowingly, pending the user's word:

- **F-5** — the production runtime topology is instantiated by the
  `event_loop` tier and never drives an exchange. A follow-up slice, not this
  one.
- **F-6** — `Refused`'s `named` field is written and never read; the design
  declares it and its exact strings do not render it. Drop it (`doc-wrong`).
- **F-7** — the two stderr outlets' exact strings are asserted nowhere. The
  raiser's route runs the binary, which `design.md` §9 item 17 forbids; the
  alternative is a sink seam for `report_platform`. Either is a decision.
- **Two `git stash` incidents** by executors, both recovered read-only and
  verified byte-for-byte; `stash@{0}` and `stash@{1}` are redundant with HEAD
  and are the user's to drop.

The five invariants hold, and four of them hold by instruments inside the
gate rather than by argument. The residue is stated: D25's feature
unification, one feature wide, and the last hop from property to pixel.

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

| document | change | reason | done |
|----------|--------|--------|------|
| `docs/adr/002-single-crate-until-triggered.md` → **new** `docs/adr/003-the-host-splits-into-a-workspace-of-strata.md` (CD-1) | superseded: recorded the decision as taken (four crates along the strata, plus `goad-boundary`), the four things ADR-002 left open now decided, what the split bought and exactly where that claim stops, what it cost (measured: 113 renames, 92 byte-identical, 24 additions, 3 deletions, 4 modifications), and where ADR-001's discipline actually slipped | ADR-002's own Verification section requires supersession, not amendment, the day the split lands; `docs/adr/002-…md:3` still read `accepted` against a workspace that exists | [x] |
| same new ADR (CD-2) | folded in, as the trigger record: ADR-002's stated reason for T1 ("a build-dependency with a conditional `build.rs` cannot be gated as cleanly") is false, measurably so — an optional `[build-dependencies]` entry plus `#[cfg(feature = "ui")]` resolves to one node. States the two real reasons T1 fired instead (the dev-dependency; the lint collision) | a superseded ADR that hides its own false premise invites a future slice to reason from it | [x] |
| `docs/specs/001-host-backend-protocol.md` §2 and a new renderer-facing requirement (CD-3) | closed the gap: a new requirement R-55, in a new "Renderers" subsection of §4, states a renderer MUST NOT refuse a view because it cannot draw part of it — it draws what it can and reports what it did not. §2's out-of-scope wording gained one sentence: drawing stays out of scope, what a renderer owes the protocol having received a view does not. §7's Verification table gained R-55's row, citing `crates/goad/tests/renderer/mapper.rs::rejected_markdown_degrades_to_plain_and_is_reported_undrawn` and its `reception.rs` counterpart | R-20's guarantee stops at normalization; §2 left nothing binding the renderer, and this slice is the first renderer — CLAUDE.md invariant 3's failure mode, closed before it could occur in code | [x] |
| `docs/specs/001-host-backend-protocol.md` §7 (CD-4) | restated the fixture path as `tests/fixtures/`, not `tests/protocol/fixtures/` | the split moved the directory so both crates can reach it; a normative path went stale by relocation | [x] |
| `CLAUDE.md` Verifying section (CD-5) | repointed "the command block in `docs/slices/001/design.md` §9 is canonical" at `docs/policy/001-the-phase-gate.md`; left `docs/slices/001/design.md` §9 untouched as slice 001's record. (`CLAUDE.md` is a symlink to the repository-root `AGENTS.md`; the edit lands there) | a closed slice's design cannot be the gate's live definition without being retro-fitted, which `docs/AGENTS.md` forbids; the split changed the gate's scope, forcing the choice | [x] |
| `CLAUDE.md` invariant 1 (CD-6) | **no wording change**, contingent on D13/D17 as implemented: confirmed live in `crates/goad-boundary/src/{scan.rs,members.rs}` — the scan reads `workspace.members` from the root manifest (`members.rs`) and is configured for both `.rs` and `.slint` extensions, string-literal-aware in both (`scan.rs:25-26,134-138,264`). If this were ever dropped, this becomes a `CLAUDE.md` amendment instead | `CLAUDE.md`'s claim that "a boundary test greps for it" would be false the day markup existed and the test still scanned `.rs` only; it does not, so the claim stands as written | [x] |
| `CLAUDE.md` Verifying section (CD-7) | replaced "clippy in **both** feature columns" and "a matrix checked in one column is unchecked" with a pointer to `docs/policy/001-the-phase-gate.md` and the three-part count (four ADR-001 instruments, the vocabulary scan, one unenforced residue), attributed to `design.md` §5.1's counting rule | confirmed false today: `justfile:50` carries one clippy line, no feature gates `tokio`/`toml` out of stratum 1 | [x] |
| `draft-policy.md` → `docs/policy/001-the-phase-gate.md` | promoted: `git mv`, retitled `POL-001`, the `DRAFT` callout and every "draft"/`POL-NNN` self-reference stripped, the Verification section's count reworded to cite `design.md` §5.1's counting rule by reference rather than asserting it as this document's own number | drafted this slice as the phase gate's new canon (`docs/templates/policy.md`); CD-5 and this promotion land together or not at all — applying CD-5 alone leaves `CLAUDE.md` pointing at nothing, promoting the policy alone leaves two claimants to the gate | [x] |
| `docs/adr/002-single-crate-until-triggered.md` status line and Consequences (not itemised as its own CD entry, but implied by CD-1's supersession) | `Status:` changed from `accepted` to `superseded by ADR-003`; one sentence added under Consequences → Neutral pointing at ADR-003 and naming the corrected premise, per `docs/AGENTS.md`'s rule that a superseded ADR is not otherwise rewritten | CD-1 says "supersede" but does not itself instruct editing ADR-002's own file; superseding a document means marking it so, which the draft table left implicit | [x] |
| `docs/policy/001-the-phase-gate.md` numbering (not itemised as its own CD entry) | numbered `POL-001` — the first policy in `docs/policy/`, which did not exist before this promotion | the draft table's promotion row named the destination path but not the POL number the promoted file would carry | [x] |
| `CLAUDE.md:64` (the authoritative-documents table) | `single crate until triggered` → the workspace of strata, ADR-003 superseding ADR-002 | same class as CD-1: a line naming a superseded ADR by its old decision; not itemised by any delta, applied under PL-18's endorsement of the class | [x] |
| `docs/specs/001-host-backend-protocol.md` §7, R-46's verification row | "clippy in **both** feature columns" → the gate's one clippy line, citing POL-001 | same class as CD-7: the feature matrix no longer exists; the delta named only `CLAUDE.md`'s copy of the claim, applied under PL-18's endorsement of the class | [x] |

**Design drift not reconciled:**

- `design.md:2538`'s artifact-map header comment reads `// crates/goad/src/
  tray_icon.rs — stratum 3`; DF-1 resolved the tray rasteriser's actual home as
  `diagnostics.rs` at plan time, and the code was built to that resolution
  (confirmed: no `tray_icon.rs` file exists in the tree). The design text
  itself was left saying the wrong file.
- `crates/goad-boundary/src/scan.rs:22,40` carry `#[derive(Debug)]` on `Scan`
  and `Breach`, resolving DF-6's "the block derives `Debug` on `Breach` only"
  as one of four permitted changes (PHASE-01/EX-5c) — the design's own
  narrower wording was not updated to say both types.
- `Breach::Token`'s field at `scan.rs:48` is `token: Cow<'static, str>` (F-38);
  the design's snippet for this variant did not carry that type.
- `design.md:367`'s member table cell for `goad` reads `` `slint` with its
  testing feature `` — confirmed present, unreconciled against whatever the
  as-built Cargo feature name for Slint's test harness actually is.
- `design.md:918`'s code-block header comment reads `// crates/goad/src/
  controller.rs — stratum 3, and it names no Slint type.` above the `Command`
  enum; `Command` is defined at `crates/goad/src/wire.rs:23`, not
  `controller.rs`. The artifact map's own worked example points at the wrong
  file.
- Item 14e (the real-close-request test) was planned against PHASE-10 in
  `plan.md`'s original text and reassigned to PHASE-08 during execution
  (`notes.md`, commit `69348ce`, "item 14e is PHASE-08's, not PHASE-10's");
  confirmed as built: `crates/goad/tests/event_loop/closing.rs` landed in
  PHASE-08 (`notes.md:2537-2833`), not PHASE-10.
- `design.md:389-396`'s test-target table lists `goad-boundary`/`checks` as
  declaring `{vocabulary, purity, allowlist}` only; the real
  `crates/goad-boundary/tests/checks/main.rs:22-26` also declares `structure`,
  a sixth instrument the module's own header comment says holds neither
  ADR-001's rule nor `CLAUDE.md`'s (PHASE-09 F-2, confirmed live in the tree).
- `crates/goad/src/clock.rs:13`'s doc comment reads "PHASE-07's `serve` takes
  one of these"; `serve` landed in PHASE-10 (PL-10's split of PHASE-07).
  `clock.rs` is production code in no phase's declared Surfaces
  (`plan.md:1515-1517` names only `docs/slices/002/*` and conditionally
  `crates/goad/README.md`), so no phase's own discipline touched it. A code
  nit rather than a canon or design-drift item properly speaking — raised
  here because PHASE-09 carried it forward, but it is `review-code.md`'s to
  dispose, not this table's.

## Closure

- [ ] All findings dispositioned; no blockers outstanding
- [ ] All acceptance criteria met, or explicitly waived by the user
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

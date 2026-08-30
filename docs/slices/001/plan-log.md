# Plan log — Slice 001

Append-only working record for the plan stage. Survives compaction and
interruption; `plan.md` itself stays clean. Never rewrite an entry — supersede
it with a later one.

Decisions here are the **user's**. Findings from an adversarial review of the
plan live in `review-plan.md`, not here; a finding that prompts a decision
produces one of each, cited to one another.

## Decisions

### 2026-08-26 — Which TOML parser, and how is it gated?

- **Asked:** `design.md` §5.2 specifies a TOML configuration file and a parsed
  `Config`, and §5.1's manifest lists serde, serde_json, jiff and tokio. No TOML
  parser is named anywhere in the design, the draft spec or the design log.
  Adding one is a dependency addition — which `docs/AGENTS.md` §Execute says to
  stop and consult on, and which ADR-002's Verification section requires be
  checked against the triggers and recorded in the slice's design.
- **Recommended:** `toml`, **optional, inside the `shell` feature**, exactly as
  tokio is. `Config` lives in `shell/config.rs`, so the parser is a stratum 2
  dependency; a non-optional one would put a dependency stratum 1 does not need
  back into stratum 1's graph and undo half of D49.
- **Decided:** `toml`, optional, inside the `shell` feature.
- **Consequence:** ADR-002's T1 does not fire, for D49's reason and by D49's
  mechanism — nothing stratum 1 must not need is required in order to build
  stratum 1. PHASE-01 writes it into the manifest; PHASE-07 is the phase that
  uses it. `design.md` §5.1's manifest and §3's trigger analysis are both a line
  short and should gain one during audit's reconciliation, since the design is a
  record of intent at a point in time and this decision post-dates it.

### 2026-08-26 — Does the integration tier need a tokio dev-dependency, and does that weaken the build gate?

- **Asked:** raised by the planner as a suspected defect of the same class as
  F-51. The integration tier must drive an async API from a `#[test]`, which
  appeared to require tokio under `[dev-dependencies]` — and Cargo forbids
  optional dev-dependencies, so that entry would be unconditional. Measured in a
  scratch crate reproducing §5.1's manifest: with such an entry present,
  `cargo test --no-default-features` still skips the `integration` target and
  still refuses to compile a `semantics/` file naming tokio, but it *does*
  compile tokio, and plain `cargo tree --no-default-features` lists it under a
  `[dev-dependencies]` heading. On that reading three statements in
  `design.md` §5.1, §9 and `canon-delta.md` CD-1 were false.
- **User's question:** how does that interact with the requirement to test the
  application logic without async?
- **Answered by measurement, and the finding was withdrawn.** The premise was
  wrong. A test target has the package's **regular** dependencies in scope,
  optional ones included, whenever the feature enabling them is on — so no
  dev-dependency is needed at all. Verified on a manifest with no
  `[dev-dependencies]` section: `#[tokio::test]` compiles and runs in
  `tests/integration/` in the default column, while in the
  `--no-default-features` column `tests/protocol/` naming `tokio` fails with
  `error[E0433]: cannot find module or crate 'tokio'`. `cargo tree
  --no-default-features` is clean with no `-e` filter.
- **Decided:** no `[dev-dependencies]` section. No correction is owed to
  `design.md` or to CD-1 — all three claims are true as written.
- **Consequence:** the answer to the user's question is stronger than the design
  claimed for it. Stratum 1 cannot be tested *with* an async runtime even by
  accident, because the test target cannot name one; that is Cargo's resolution
  rather than review or a grep, so AC-15's boundary test does not need extending
  to cover `tests/protocol/`. PHASE-01/EX-3 asserts it by breaking it and
  reverting, and PHASE-01's implementer notes forbid adding a
  `[dev-dependencies]` section later, since an unconditional tokio entry there
  is precisely what would put a runtime back in reach of the stratum 1 tier.

### 2026-08-26 — Adversarial review of the plan?

- **Asked:** whether `plan.md` goes to a fresh adversarial reviewer before
  acceptance, as the design did over five rounds.
- **Decided:** yes — one round, fresh reviewer with no thread history.
- **Consequence:** ledger at `review-plan.md`, copied from
  `docs/templates/review-ledger.md`, subject `plan`. Rounds 4 and 5 of the design
  review both used a fresh reviewer and both reached past what the accumulating
  thread of rounds 1-3 managed; round 5's thread
  (`01a03af5-bc55-79a1-a216-ff9c7e7ee4e1`) is spent and is not reused.

### 2026-08-27 — Closing the plan review: four rounds, closed on a clean one

- **Asked:** whether round 4 closes `review-plan.md`, or whether the residual
  rate justifies a fifth.
- **Decided:** closed. Round 4 raised **no findings**, confirmed F-12, F-13 and
  F-14 individually with cited evidence, found the tests carve-out's *reasoning*
  sound on its first attack, and judged `plan.md` **executable as it stands**.
- **Why not a fifth round.** The defect rate per repair fell 4/6 → 2/5 → 0/3 and
  the round that produced the zero was the first run against text the author had
  already restatement-swept. A fifth round would be testing the sweep, not the
  plan, and the review's stated close condition — no blocker, nothing a repair
  cannot close — was met. The one review decision that cost this slice most was
  the *design* review's opposite: closing with sixteen repairs unverified.
- **Consequence:** `review-plan.md` `State: closed`, Synthesis written. Two risks
  are recorded there and carried into `notes.md`'s handover rather than resolved:
  round 4 verified PHASE-06/VT-6's mechanism on the documents and did **not**
  re-run the tokio metrics probe, and priority 3's sites — the author's five
  self-sweep repairs, PHASE-08's split comment, PHASE-06's cancellation note —
  got no per-site verdict, only the round's overall silence.
- **Also recorded, not as a finding:** the round 4 packet itself carried two
  stale restatements, found at dispatch — its reading list still named rounds 1
  and 2 and pointed the reviewer at F-7…F-11, and its finding-format example was
  headed `F-7` against an instruction to number from F-15. Author-found,
  author-repaired before dispatch. The document written to hunt the restatement
  defect had it. That is the strongest argument in this slice for making the
  sweep mechanical rather than intentional.
- **Next:** `plan.md` to the user for acceptance. No code before it.

### 2026-08-29 — Plan accepted

- **Asked:** acceptance of `plan.md`, per `docs/AGENTS.md` §Plan and `CLAUDE.md`'s
  "no code without an accepted plan".
- **Decided:** accepted. The user's instruction to write PHASE-01's phase sheet
  is the acceptance; there was no separate ceremony and none is owed.
- **Consequence:** PHASE-01/EN-1 is discharged. The phase sheet is in `notes.md`
  under `## Phase sheets`; execution may begin. Sheets stay one at a time,
  immediately before each phase — `docs/AGENTS.md` §Phase plan, and a sheet
  written three phases early is fiction.
- **Raised while expanding the phase, and owed back to the plan:** two of
  PHASE-01's *Notes for the implementer* describe a tree that has since changed.
  The plan says `.gitignore` does not exist — it has since commit `4fc8637` —
  and says not to duplicate the user's global `*.local.*` ignore, which that
  commit already did, narrower, as `*.local.md`. `docs/AGENTS.md` §Phase plan
  forbids repairing this in the sheet, so it is recorded and left to the user.
  Neither is a criterion and neither blocks PHASE-01.

### 2026-08-29 — Package metadata and licence

- **Asked:** whether to carry `[package]` metadata now, and under what licence.
- **Decided:** yes, now. **MIT**, repository `https://github.com/davidlee/goad`,
  fields lifted from `~/dev/doctrine`'s manifest. `LICENSE` written at the
  repository root, © 2026 David Lee. `publish = false` stays and is not
  provisional.
- **Consequence:** `Cargo.toml` `[package]` gains `license`, `repository`,
  `readme`, `keywords` and `categories`. `LICENSE` is a new root file, outside
  PHASE-01's declared surfaces — a user-directed scaffolding change of the same
  kind as `Cargo.toml`, `clippy.toml` and `justfile` before it, not phase work.
- **The reason it was raised was wrong, and the correction matters more than the
  change.** PHASE-01's sheet flagged `clippy::cargo_common_metadata` as an
  anticipated gate failure. It cannot fire: **`publish = false` silences it
  outright.** Measured in a scratch crate carrying `cargo = "deny"` — clean with
  `publish = false`, five errors without it (`license`, `repository`, `readme`,
  `keywords`, `categories`). The metadata is therefore carried for correct
  attribution, not to pass the gate, and the STOP condition is withdrawn.
- **A positive control was run rather than assumed**, per the review's own
  lesson that a mechanism needs one: the same probe with goad's new `[package]`
  block and `publish = false` **removed** also passes clean. So the metadata is
  complete on the lint's terms, not merely hidden behind `publish = false`.

### 2026-08-29 — The two stale `.gitignore` notes in PHASE-01

- **Asked:** disposition of two stale *Notes for the implementer* surfaced while
  expanding PHASE-01 — `plan.md` claimed `.gitignore` did not exist (false since
  `4fc8637`) and told the implementer not to duplicate the user's global
  `*.local.*` ignore, which that commit had already done, narrower, as
  `*.local.md`.
- **Decided:** widen the repository `.gitignore` to `*.local.*`, and rewrite the
  plan's note to describe the tree as it is.
- **Why widen rather than defer to the global file.** A rule that lives only in
  one machine's global config is not a property of the repository. Under
  `*.local.md` alone, `transport-probe.local.rs` and
  `transport-probe-Cargo.local.toml` were ignored by `~/.gitignore_global` and by
  nothing in the repo, so a clean clone elsewhere would track them — and AC-1 is
  a claim about a clean clone.
- **Consequence:** `.gitignore` now carries `*.local.*` with the reason written
  at the site. `git ls-files` was checked first: no tracked file matched, so
  nothing was silently un-tracked. `git check-ignore -v` now resolves all three
  probe and packet files to `.gitignore:4` rather than to the global file.
  `plan.md:282` rewritten; PHASE-01's sheet records the closure.

### 2026-08-29 — Two gaps in PHASE-02, surfaced by expanding its phase sheet

- **Asked:** disposition of two things `plan.md`'s PHASE-02 entry did not settle.
  (1) Its declared surfaces name no test file and no line for
  `src/semantics/mod.rs`, which the phase must edit to declare
  `semantics::protocol` at all — it is the only phase in the plan that writes
  tests and names none. (2) The design gives the scalar newtypes no constructors,
  while PHASE-07 mints `view_id` in `shell/state.rs` (D13) and its surfaces
  exclude `canonical.rs`, so whatever it needs must ship in PHASE-02.
- **Decided:** (1) surfaces gain `src/semantics/mod.rs`, and the phase's tests are
  **colocated `#[cfg(test)]` modules**; `tests/protocol/` stays PHASE-03's.
  (2) **`ViewId` and `Timestamp` get public constructors; `OptionId`, `FieldId`
  and `AlternativeId` do not.**
- **Why the test home is forced rather than chosen.** `tests/protocol/` is an
  external crate. Under D30, `Opt`, `Field` and `Alternative` have `pub(super)`
  fields and no public constructor, so an external test cannot build the
  `Vec<Opt>` that `Options::new` must reject or the two same-id `Field`s VT-3
  must accept. The only way to write VT-1 and VT-3 there is to widen the
  canonical types, which is R10 — the exact risk the same phase's VA-2 exists to
  detect. VT-2 could have gone either way and is colocated with them: one home
  per phase, and no early touch on the target PHASE-03 owns.
- **Why the constructor split falls on host-authored versus backend-authored.**
  A `ViewId` is minted by the host and a `Timestamp` is a clock read; neither
  asserts that a backend said anything, and stratum 2 must be able to build both.
  An `OptionId`, `FieldId` or `AlternativeId` is an address a backend chose, so a
  public constructor would let a caller mint an id no backend ever sent — the
  same hole D30 closes one level up on the canonical types. A caller answering a
  view clones the id out of the view through its accessor, which is what the
  design's `Clone` derive is already for.
- **Consequence:** `plan.md:311` rewritten — surfaces amended and the colocation
  rule stated with its reason; EX-1 amended to name which scalars carry a public
  constructor and why. PHASE-02's sheet in `notes.md` records both as closed.

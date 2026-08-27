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

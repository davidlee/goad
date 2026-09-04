# Audit & reconciliation — Slice 001

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `6489521..578bf84` — everything after the accepted design, on
`main`. Fourteen execution commits across ten phases (01–08, 10, 09). Written
2026-09-04, before opening `src/`, `tests/`, `notes.md` or either review ledger.

**Standing instruction from the user, and the audit's stance:** decisions taken
during execution were accepted without much scrutiny and are **not** treated as
settled. Every `plan-log.md` entry dated after 2026-08-26 is re-examined here as
if it were a proposal, and the question asked of each is whether the brief and
the accepted design would have produced it. The user's own framing of what
matters: *is the thing holding up coherently against the brief and the design?*
That is the audit's first line, and the others serve it.

**Lines of attack.**

1. **Coherence with the brief.** Brief §1 and §3.1: the host understands
   interaction, not intent. For every behaviour the host has, ask brief §22.1 —
   could it live in the backend? — and brief §22.3 — has the protocol been
   narrowed to what a renderer would find convenient? The brief's own worked
   examples (§8, §10.1, §10.2, §18) must be accepted verbatim by what shipped,
   not merely by a fixture claiming so. Brief §14: nothing may imply a sandbox.
   Brief §17's *required* list is checked for what this slice claimed and what
   it did not, so the slice's scope is confirmed as the brief's phases 1 and 2
   and nothing more.
2. **Conformance to the accepted design.** `design.md` §5.2's types, §5.3's
   state and ownership table, §5.4's transport structure, and §5.5's sixteen
   invariants. For each invariant: is it held by the mechanism the table names,
   or by convention wearing a mechanism's name? The suspects: I1 (a second door
   into canonical types — `pub` fields, a `Deserialize` impl, a public
   constructor), I3 (a clock, `std::fs`, `std::env` or `std::process` under
   `src/semantics/`), I6 (`&mut self` actually threaded), I7 (a `hints` key read
   anywhere), I11 and I13 (the reader ownership and the no-`?` region), I15
   (uniqueness checked in every constructor, not two of three).
3. **Canon.** ADR-001's direction half is a review gate — this audit is that
   review. Grep `src/semantics/` for anything the AC-15 test cannot see: `std::`
   I/O modules, re-exports that flatten the boundary, stratum 2 types leaking
   downward. ADR-002's triggers: confirm none fired, including T3 in its gradual
   form (test wall-clock). CD-1 and CD-2 are checked against the code before
   they are applied, not after.
4. **The draft spec against the code.** Every `R-N` walked against `src/`, not
   against its §7 row. The five review-held rows (R-9/R-19, R-18, R-20, R-30,
   R-49) are performed here rather than believed. A row naming a test is
   verified by reading the test's assertion, since a test that exists and a test
   that asserts the requirement are different things.
5. **Test honesty.** For each verification claim, can the test fail? Vacuity
   guards present and themselves tested; timing assertions with slack that makes
   them meaningless; fixtures that assert acceptance of a document without
   asserting the value it normalizes to; the R-45 one-`Host` test actually
   reusing one host; the cancellation test actually driving the future.
6. **Execution-time decisions, one by one.** Enumerate `plan-log.md`'s entries
   after plan acceptance. For each: what changed, whether the design already
   settled it the other way, whether it widened a surface, and whether the
   brief would have produced it. Particular suspects, named before looking:
   `ConfigError` (not in §5.2's taxonomy); the config duration grammar restated
   rather than shared with `schedule.rs` (a DRY breach the user's own standards
   forbid); the `deno check` seventh command; the `Fields`-may-be-empty
   amendment; the canonical.rs comment edits at PHASE-09; every Surfaces
   widening.
7. **Surface delta.** `git diff --stat 6489521..578bf84` against the union of
   every phase's declared Surfaces. Undeclared paths are the lead. Declared and
   untouched is the other lead.
8. **Simplicity.** The user's standard is *as simple as possible, but no
   simpler*, and *write less code*. The transport in particular has been
   restructured five times under review; the audit asks whether what shipped is
   the smallest structure that holds AC-5, or one that accreted. Also: the ratio
   of documentation to code, and whether a future agent can find the truth
   without reading 6,000 lines of notes.
9. **The items PHASE-09 handed forward.** The two §9 list items with no
   end-to-end case; R-34 across a real backend failure resting on one test; the
   README config nothing runs; §5.2's stale taxonomy block; the wedged-`wait`
   edge-case row no test can arrange; the stdout-cap sentence; `PipeMissing`
   untested. Each gets a disposition, none is inherited as "known".
10. **Closure mechanics.** AC-14 and the two canon deltas need explicit user
    endorsement and are not applied by this audit on its own authority. The
    audit's job is to make the endorsement decision well-posed: what exactly
    would be promoted, what it says that the code does not do, and what the
    code does that it does not say.

**Invariants held to.** The five in root `AGENTS.md` and the sixteen in
`design.md` §5.5. Where the two disagree with each other, that is a finding
against the slice's own restatement sweep.

**What this audit will not do.** It will not retro-fit `design.md` to the code;
departures go under *Design drift not reconciled*. It will not downgrade a
blocker to clear the gate. It will not promote or amend canon without the user's
explicit endorsement, recorded in the Reconciliation table.

## Evidence

Gathered 2026-09-04 by the audit agent, session 1. Nothing here is a verdict.

- **Tests / checks:** `just check` on the working tree at `578bf84` (only
  `flake.lock` dirty), exit 0. Seven commands, both columns:
  35 unit + 52 integration + 15 protocol with `shell`; 22 unit + 15 protocol
  without. Matches PHASE-09's clean-clone run of `30d834f` (`notes.md`, PHASE-09
  Verification record, VT-1/VA-1); the audit did not repeat the clean clone.
  Integration tier wall-clock 1.46 s, so ADR-002's T3 is nowhere near firing.
- **Grep-based invariant checks, all clean:** no `std::{fs,env,process,net,
  time,thread,io}`, `SystemTime`, `Instant::now`, `tokio`, `crate::shell` or
  `async` under `src/semantics/` (I3, ADR-001 direction); `hints` read nowhere
  but collected in `normalize.rs:231,239` (I7); `Content::Uri` constructed and
  never read; `.data` / `.values` never read (R-9); no `Deserialize` and no
  `pub use` in `canonical.rs`, and the only `pub` fields are the five outbound
  request types (I1, D5); no domain vocabulary under `src/` (the two hits are
  the scan's own token list and a backend script, which is the backend's);
  every `?` in `process.rs` is inside `body`, `dispose`, `read_capped` or the
  cleanup budget, none between the spawn and the budget (I13).
- **Acceptance criteria:** the audit re-walked every AC against the tests it
  names rather than against PHASE-09's VA-2 table, and concurs with that table:
  AC-1…AC-13 and AC-15 **met** with the tests named there; AC-14 **open** until
  close. Two qualifications the table does not carry: AC-5's cancellation
  clause is held by a test that drives the future and asserts a positive
  control (`transport.rs:602`), which is the honest form; AC-9's "verbatim"
  claim holds for §10.1's body and §10.2's field but the corpus's *envelope*
  differs from the brief's (§8.2's examples carry `next_check` beside `view`,
  and the corpus's do too — checked, `R-21-next-check-as-a-relative-span`).
- **Verification criteria:** every VT/VA/VH in `plan.md` is recorded
  discharged in its phase sheet with evidence pasted, and the audit spot-read
  the discharge of the criteria most likely to be vacuous: PHASE-06/VT-6
  (cancellation — positive control present), PHASE-10/VT-2 (one `Host` — reuse
  witnessed by an outstanding view and a moved schedule, not by the last
  exchange), PHASE-08/VT-2 (no spawn — invocation-log witness, after a vacuous
  first draft the sheet records), PHASE-04/VT-2 (every variant named by a
  fixture — `normalize.rs:426`, with the two exemptions asserted in the
  negative). None vacuous.
- **Surface delta:** `git diff --name-only 6489521..578bf84` against the union
  of every phase's declared Surfaces. **Undeclared and touched:** `LICENSE`
  (user-directed, `plan-log.md` 2026-08-29), `Cargo.lock` (implied by
  `Cargo.toml`), `docs/slices/001/review-plan.md` and `plan-log.md` (planning
  stage, before execution), `docs/slices/001/design-log.md` and the I9 / D53 /
  §9 lint-prose edits to `design.md` (user decisions of 2026-08-27, before plan
  acceptance on 08-29 — recorded, not phase work). `design.md` §9's command
  block was declared by PHASE-08. **Declared and untouched:** none.
  **Verdict:** no scope creep; every undeclared path has a recorded decision.
- **Canon deltas checked against the code before application:** CD-1's claims
  hold — tokio and toml are `optional`, `shell = ["dep:tokio", "dep:toml"]`,
  `cargo test --no-default-features` runs in the gate, and the direction test
  greps exactly the three tokens CD-1 names. CD-2's T1 wording matches
  `Cargo.toml`. Both are ready to apply on endorsement.
- **Execution-time decisions re-examined** (`plan-log.md`, 2026-08-29 →
  2026-09-04, eleven entries). Nine are plan bookkeeping — Surfaces widened to
  name a `mod` line, a test file, or a comment edit — and each names its class;
  none changed behaviour. Two changed the shipped shape and are re-opened as
  findings below rather than inherited: `ConfigError` (2026-09-03, a fifth
  error type the design lacks — sound, and a drift entry) and the restated
  config duration grammar (2026-09-03 — F-4 in `review-code.md`). The
  `Fields`-may-be-empty amendment (2026-08-30) is correct: the brief, the spec
  and the spec's own example all admit an option with no fields. The `deno
  check` seventh command (2026-09-03) is correct and cheap.

## Code review

<!-- Adversarial, by a fresh agent where possible. Findings are append-only and
     keep their ids across rounds. -->

Ledger: `docs/slices/001/review-code.md`, subject `implementation`. Findings
live there and are not copied here.

**Round 1** — 2026-09-04 — two raisers: the audit agent (F-1…F-16, raised while
gathering evidence, before any fresh-reviewer output was seen) and a fresh
reviewer agent whose findings append after. Outstanding blockers: **0** raised
by the audit agent; the fresh reviewer's count is recorded in the ledger when
it lands.

<!-- severity — blocker: must not ship. major: real defect or design breach.
       minor: worth fixing, not urgent. question: needs an answer before it
       can be graded.
     disposition — aligned: observation correct, nothing to change.
       fix-now: code fix, inside this slice. spec-wrong: the code is right and
       the document is stale — goes to Reconciliation below. tolerated:
       accepted drift, with a written rationale. deferred: becomes a follow-up
       in `slice-nnn.md`.
     No finding may be left undispositioned at close. Do not downgrade a
     blocker to dodge the gate, and do not defer merely because the fix is
     large. -->

**Synthesis:** <the closure story: what the audit found, what it changed, and
the risks it knowingly leaves standing.>

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

| document | change | reason | done |
|----------|--------|--------|------|
| `specs/NNN-…md §4` | | code diverged at `path:line`; code is right | [ ] |

**Design drift not reconciled:** <where the implementation departs from
`design.md` and the design was left as-is, with the reason. The design is a
record of intent at a point in time; it is not retro-fitted to the code
without saying so.>

## Closure

- [ ] All findings dispositioned; no blockers outstanding
- [ ] All acceptance criteria met, or explicitly waived by the user
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

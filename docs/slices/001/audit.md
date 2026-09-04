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
reviewer subagent (F-17…F-33). Thirty-three findings, **0 blockers**, 4 major
(F-1, F-2 and their independent confirmations F-17, F-18), 20 minor, 9 nit.
Six of the reviewer's findings confirm audit-agent findings independently and
with probe output. **All dispositioned by user decision 2026-09-04** (session
2): 29 fix-now, F-13 and F-15 tolerated, six duplicates following their
primaries. **All 29 repaired** in session 2; `just check` exits 0 in both
columns.

**Round 2** — 2026-09-04 — a fresh reviewer over the repairs only. Eleven
findings, F-34…F-44: **0 blockers**, 1 major (F-34 — F-1's guard bypassed on
the failure path), 6 minor, 4 nit; every round-1 repair but F-9's confirmed
red-on-revert. All eleven dispositioned by user decision 2026-09-04 (9
fix-now, F-44 tolerated); all nine repaired in session 2b.

**Rounds 3–6** — 2026-09-04, session 2b — each a fresh reviewer over the
previous round's repairs only. Round 3: F-45…F-51 (7, none above minor);
round 4: F-52…F-54 (3); round 5: F-55…F-56 (2); round 6: **no findings**, its
one aside self-raised as F-57, a doc nit, and repaired. Every finding
dispositioned by user decision in one interview per round and repaired
red/green before the next round opened. Rounds 3–5 each found the time-of-day
seam one shape further along; the rule is now two conjuncts and a trim, and
twelve schedule fixtures pin it.

**Round 7** — 2026-09-04, session 3 — opened because reconciliation changed
code: the six comment lines under `src/` and `tests/` that cite the spec by
path, renamed at promotion. A fresh reviewer over that diff only confirmed all
six and found one nit of the same class beside them, F-58, dispositioned
fix-now by the user and repaired.

**Totals.** Fifty-eight findings over seven rounds: 0 blocker, 5 major, 33
minor, 20 nit. Fifty-five fix-now and repaired; three tolerated by user
decision (F-13, F-15, F-44). Every Outcome set. **Outstanding blockers: 0.
Outstanding fix-now: 0.** `just check` exits 0 in both columns at every
round's commit and on the closing tree.

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

**Synthesis:** the ledger's own Synthesis is the closure story for the review
and is not copied here. What the audit adds is the verdict on its ten lines of
attack. **Coherence with the brief** was the line that produced the findings
that mattered, and every one of them went the brief's way against an
execution-time decision or an unexamined default: an elapsed check is consumed
(brief §9's "existing *valid* check"), a time of day is refused (brief §3.3), a
failure accepts no instruction, shape refusals are typed and total. **The
design** holds: all sixteen §5.5 invariants are held by the mechanism the table
names, and the departures — listed under *Design drift not reconciled* — are
each a case where the code went further than the design or the design guessed
a mechanism that measurement replaced. **Canon**: ADR-001's direction half was
reviewed here as the ADR asks, both compile columns and the scans clean; no
ADR-002 trigger fired. **The draft spec** was walked requirement by
requirement and is reworded where the code is now right and the draft was
behind; §7 names a test or fixture for all 54 requirements, checked by script
in both directions. **Test honesty**: two vacuous assertions were found by the
phases themselves and one by the review (F-43); none survives. **Execution-time
decisions**: two changed the shipped shape and both were re-opened as findings
rather than inherited — one stood (`ConfigError`, a drift entry) and one fell
(the restated duration grammar, F-4). **Surface delta**: clean. **Simplicity**:
the transport is the smallest structure found that holds AC-5's five clauses;
what accreted under review was tests, not mechanism. **PHASE-09's items**: each
dispositioned in the Reconciliation section. **Closure mechanics**: the
endorsement decisions are posed below.

Risks knowingly left standing are the ledger Synthesis's list, unchanged:
a text-scan boundary test; a handful of colon strings the shape rule names as
a time of day that no author writes on purpose; an unbounded `raw` in a
discard line; a doubled chain in one `Display`; `PipeMissing` reachable by no
test; and this slice's code citing its own ledger ids.

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

Two lists were worked: the session-2 and session-2b handovers in `notes.md`,
and the ledger Synthesis's "For reconciliation" paragraph. Each item below is
one of **document stale** (code right, document amended), **code wrong** (a
finding — none remained; every one had been raised in the ledger already), or
**decision** (neither cleanly; taken to the user).

**Canon — each needs the user's explicit endorsement, recorded in the row.**

| document | change | reason | done |
|----------|--------|--------|------|
| `docs/specs/001-host-backend-protocol.md` | `draft-spec.md` promoted as **SPEC-001**, `Status: active`, the not-canon preamble removed. Six revision-history sentences struck (R-41, R-43, R-47, R-48 "Corrected/Restated/Scoped per…", the R-53 and R-45 rows' "earlier draft"/"previously described"), because canon carries no changelog (`docs/AGENTS.md`). The `F-N` ids that remain as rationale are qualified once in §9 as `review-design.md`'s | AC-14; promotion is what makes it normative | [x] endorsed by the user 2026-09-04 (session 3 interview); `git mv`, so history follows the file |
| `docs/adr/001-one-way-strata.md` §Verification | CD-1 applied as stated in `canon-delta.md` | record accuracy; checked against the code in Evidence | [x] endorsed 2026-09-04, applied as stated |
| `docs/adr/002-single-crate-until-triggered.md` §Decision, T1 | CD-2 applied as stated in `canon-delta.md` | record accuracy; checked against the code in Evidence | [x] endorsed 2026-09-04, applied as stated |

**The draft, reconciled before promotion — document stale, code right.** All
done this session; endorsed as part of the promotion above.

| document | change | reason | done |
|----------|--------|--------|------|
| `draft-spec.md` R-26 | the retained instant stands only while still ahead of `now`; an elapsed one is consumed | `schedule::resolve` consumes `retained <= now` (F-1, user decision) | [x] |
| `draft-spec.md` R-29 | "MUST NOT accept a new instruction"; resolves as if none arrived; reports what it retains | `Host::no_action` resolves through `resolve_from` and writes state (F-34, F-48) | [x] |
| `draft-spec.md` R-21 | trim on both forms, errors quote the value as sent; the time of day stated as a shape (colon groups, optional fraction) or a clock-grammar read behind a colon/`T` gate; signed colon forms are spans, unitless integers unparseable; one duration grammar | `schedule::parse_instruction`, `parse_span`, `looks_like_a_time_of_day` (F-2, F-37, F-46, F-50, F-52, F-53, F-55) | [x] |
| `draft-spec.md` R-25 | names the six distinct ways a value fails | `ScheduleError` has six variants; `TimeOfDay` was unnamed | [x] |
| `draft-spec.md` R-44 | adds stdout past its bound; splits malformed from protocol-invalid with the line stated; adds duplicate key at any depth and the nested `hints` object | `ProtocolError::{Shape, DuplicateKey, NestedHints}`, one `From` door (F-19, F-22, F-25) | [x] |
| `draft-spec.md` R-18 | nested `hints` object rejected with its path; `null` there is omission | `normalize_field` (F-25, F-38) | [x] |
| `draft-spec.md` R-19 | a content block neither string nor object is a shape error | `Object<WireContent>` (F-35) | [x] |
| `draft-spec.md` R-51 | nulled `hints` on a field and nulled `fields` on an alternative named as omission | fixtures `R-51-a-nulled-{hints-key-on-a-field,fields-key-on-an-alternative}` (F-38) | [x] |
| `draft-spec.md` R-36 | the vector's first element names the program; empty vector or empty program refused at load | `Command::from_argv` (F-3, F-41) | [x] |
| `draft-spec.md` §5 "A broken backend is polled on its existing cadence" | restated in R-29's terms, with why an elapsed instant is not reported | same as R-26/R-29 | [x] |
| `draft-spec.md` §7 | PHASE-09's two-direction check re-run (script in the session-3 handover, `notes.md`): 22 fixtures no row cited, 2 tests renamed (`what_a_json_value_cannot_carry_is_refused_from_the_document_text`; the never-reads case), 6 tests added in review; intro names the `protocol-text` runner; the `R-21-*` glob replaced by names. Result **CLEAN** — 54 of 54, no id in two rows, every fn and fixture present, every fixture cited | rows written before rounds 1–6 | [x] |

**Code — comments only, pending the promotion.**

| document | change | reason | done |
|----------|--------|--------|------|
| `src/semantics/schedule.rs:2`, `src/semantics/protocol/canonical.rs:117`, `:511`, `:721`, `tests/protocol/normalize.rs:21`, `tests/protocol/runner.rs:48` | `draft-spec.md` → `SPEC-001` | promotion moves the file, so the path citations dangle. A round-7 fresh reviewer over this diff only, per the standing rule | [x] endorsed 2026-09-04; `just check` exit 0 both columns; round 7 below |
| root `AGENTS.md`, document table | `docs/specs/` no longer "empty until slice 001 promotes its draft" | promotion | [x] |

**Decisions — neither side cleanly wrong; taken to the user.**

- **`design.md` §9's two misbehaving-backend items with no end-to-end case**
  (a backend that writes nothing; brief §10.1/§10.2's examples through a real
  process). *Tolerated, no case added:* both are held at other tiers
  (`host.rs::a_body_that_is_not_exactly_one_json_document_is_a_protocol_failure`
  over empty stdout; the two brief fixtures), and since F-22 both corpora and
  the host read through the one door `normalize::read_response`, so a process
  vehicle would exercise the transport, which its own suite already covers. The
  §9 claim "the integration tier needs" is a drift entry below. User decision 2026-09-04.
- **`BackendError::PipeMissing` and `cleanup_only` reachable by no test.**
  *Tolerated* (F-15): a guard for a handle the host itself requested, which no
  backend can arrange; the design states it as a guard.
- **`design.md`'s departures.** *Left as written* (below), not amended — user decision 2026-09-04: the
  design is a record of intent at a point in time, the promoted spec is now the
  living truth for every item here, and each departure is one the code
  documents at its site.
- **`transport-probe.local.rs`** stays untracked under `.gitignore`'s
  `*.local.*` (user decision 2026-09-04): the measurement it made (`design.md` §9) is recorded; the code
  was a probe.

**PHASE-09's handed-forward items**, each dispositioned rather than inherited:
the two §9 list items — above; R-34 across a backend failure — now two tests
(F-7 added `host.rs::a_backend_failure_during_respond_leaves_the_interaction_answerable`);
the README config — run by `round_trip.rs::the_readme_s_own_config_loads_and_runs_the_example`
(F-16); §5.2's taxonomy — drift, below; the wedged-`wait` row — drift, below;
the stdout-cap sentence — drift, below; `PipeMissing` — above; `issued_at` —
removed (F-6); the `Cargo.toml` lint comment — corrected (F-12); the AC-11 scan
— matches words (F-14); config unknown keys — refused (F-5); `design.md:1052`
— drift, below.

**Design drift not reconciled.** `design.md` stands as written at each of
these; the code went elsewhere, and the promoted spec or the code's own
comments say so.

- **§5.2 error taxonomy.** `ProtocolError` gained `Shape`, `DuplicateKey`,
  `NestedHints` (F-19, F-22, F-25); `ScheduleError` gained `TimeOfDay` (F-2);
  `SpanFault` is new, the shared duration grammar's refusal (F-4);
  `ConfigError` is a seventh type the design lacks (user decision 2026-09-03);
  `:921` spells `semantics::ProtocolError`, unreachable under
  `pub_use = "deny"` — the path is `semantics::error::ProtocolError`.
- **§5.2 `Command`.** The config boundary yields `Command { program,
  arguments }`, not `Vec<String>`, so an empty command cannot reach the
  transport (F-3, F-41). `Fields` and `Hints` no longer derive `Default`
  (F-11); the protocol version is one `pub const` (F-27).
- **§5.3 `resolved_check`.** Still not an `Option`, as the prose says; but
  "else the retained one" holds only while the retained instant is ahead of
  `now` (F-1). `Outstanding.issued_at` (`:1172`) was removed — nothing read it
  (F-6).
- **§5.4 "Failure does not move the schedule"** (`:1615`–`:1619`) and the
  state diagram's `respond(stale id) — state untouched` (`:1611`). A failure
  resolves through the same arm as success with no instruction and writes what
  it reports; the outstanding interaction is untouched, the resolved check may
  move from an elapsed instant to the default poll (F-34, F-48, F-54). R-29
  and §5 of the spec state the rule as it is.
- **§5.4 `read_capped(r: &mut impl AsyncRead)`** (`:1294`, `:1330`). The
  reader owns the handle, as the prose and
  `transport_shape.rs::the_capped_reader_owns_the_stdout_handle` require.
- **§5.4 "the cap kills the backend by itself"** (`:1528`). The pipe does
  close at the bound, but disposal `start_kill`s before observing anything, so
  the host sees its own kill; the sentence names the mechanism that does not
  fire.
- **§5.4 step 3.** The stdin write and the stdout read are concurrent and a
  broken pipe on the write is tolerated (F-10, F-24); the sketch writes then
  reads.
- **§5.5 edge table, `backend wedged so wait cannot return`** (`:1729`).
  Names a mechanism no test can arrange — only uninterruptible sleep defers
  `SIGKILL`; every observed cleanup timeout stalls on the drain.
- **§5.2 `:704`** "all three ≥ 1": `Fields` may be empty (user decision
  2026-08-30; brief and spec agree).
- **§5.1 manifest and §3 trigger analysis** omit `toml`; **§9's lint prose**
  omits `module_name_repetitions = "allow"` (user decision 2026-08-27).
- **`:1052`** "invalid UTF-8 becomes a `Protocol(Json)` error": true only for
  a value serde decodes; a skipped value's bytes are never read. The spec's
  R-38 row states the narrower claim.
- **§9's misbehaving-backend list** says "the integration tier needs" where
  two items are held at other tiers — the decision above.
- **Not drift:** `WireAlternative`, `WireContent`, `WireContentValue` are named
  nowhere in §5 — §6 latitude exercised, as PHASE-04 recorded.

## Closure

- [x] All findings dispositioned; no blockers outstanding — F-1…F-58, ledger Synthesis
- [x] All acceptance criteria met — AC-1…AC-15 checked in `slice-001.md`; AC-14 by the promotion above
- [x] Tests and checks green — `just check` exit 0, both columns, closing tree
- [x] Specs / policy / ADRs reconciled, with user endorsement where amended — SPEC-001, CD-1, CD-2; `docs/policy/` empty
- [x] `slice-001.md` Summary and Follow-ups written
- [x] `notes.md` Harvest current; five facts lifted to `docs/memory/`
- [x] `slice-001.md` stage set to `done`

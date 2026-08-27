# Review — plan — Slice 001

**Subject:** plan — `docs/slices/001/plan.md`, as at 2026-08-26, read against
`docs/slices/001/design.md`, `docs/slices/001/slice-001.md`,
`docs/slices/001/draft-spec.md`, `docs/adr/001-one-way-strata.md`,
`docs/adr/002-single-crate-until-triggered.md` and `docs/AGENTS.md`.
**Reviewer:** fresh agent, no thread history (gpt-5.5 via MCP)
**Opened:** 2026-08-26
**State:** open

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. Reject a finding on **evidence**, never on assertion. Confirm each
disposition with the user before acting on it. Fix the class, not the instance,
and do not introduce new defects repairing old ones.

## Brief

**Round 1** — 2026-08-26 — the plan's sequencing, phase sizing, and whether its
criteria are held by mechanisms.

The design under this plan has already had five review rounds and 63 findings,
and was closed **by user decision with 16 repairs unverified**. This review is
not a sixth round on the design. It reviews the **plan**: whether nine phases in
the stated order can actually be executed, and whether a phase that reports
itself green has demonstrated what its exit criteria claim.

What this round is probing, in order of expected yield:

1. **Dependency errors in the sequencing.** Each phase's entry criteria must be
   discharged by an earlier phase's exit criteria, and each phase must be able to
   produce the artefacts its own exit criteria name using only what precedes it.
   A phase that cannot be finished without reaching forward is the defect.
2. **Criteria held by nothing.** The design review's single most transferable
   finding was that a claim is held by a mechanism or it is not held (F-51, F-57,
   F-62 — canon no build checked, a build gate whose command could not run, an
   invariant resting on allow-by-default lints). Applied here: does each `EX-`
   and `VT-` name something that actually runs and can actually fail? A `VA-`
   that amounts to "check carefully" is the same defect in plan clothing.
3. **Phase sizing.** `docs/AGENTS.md` requires each phase be finishable by one
   agent in one session, bookkeeping included. PHASE-04 (wire, normalization and
   the protocol corpus), PHASE-06 (bounds, disposal, both grandchild cases) and
   PHASE-08 (round trip plus the full failure matrix) are the three most likely
   to be over-packed. PHASE-01 carries the manifest, the module tree, the error
   taxonomy, two boundary tests and a `flake.nix` change.
4. **Coverage that is nominal rather than real.** Every AC in `slice-001.md` is
   mapped in the plan's Coverage table. A row that points at a criterion which
   does not in fact discharge the AC is worse than a gap, because it reads as
   covered. AC-5 and AC-6 are the long ones and the likeliest to have a clause
   nothing lands on.
5. **The §5.4 split.** The transport is split across PHASE-05 and PHASE-06 on the
   argument that its last restructure is unreviewed. Is the seam in the right
   place — does PHASE-05 leave the transport in a state PHASE-06 can extend
   without rewriting, and can PHASE-05's exit criteria be met without the bounds
   PHASE-06 owns?

Where the bodies are likely buried, stated so a green report on these is worth
something:

- **PHASE-01 does a lot and everything depends on it.** If its build gate is
  wrong, eight phases are checked by a broken check.
- **The fixture-runner format** is decided in PHASE-03 and inherited by PHASE-04,
  which has a much larger corpus. A format that suits scheduling and not the
  protocol tier is a rewrite discovered late.
- **PHASE-07 introduces a dependency** (`toml`) after six phases of manifest
  stability. Its entry criteria must actually re-prove the gate.
- **PHASE-09 is where AC-10 first gets written** and where the restatement sweep
  runs. If it is really three phases of work, the plan hides its own tail.
- The plan asserts stratum 1 cannot be tested with an async runtime because a
  test target cannot name tokio with the feature off. That was measured, and the
  measurement is in `plan-log.md`. Attack it if the reasoning does not hold.

**Out of scope for this round.** The design's own decisions — D1…D54 — and the
sixteen unverified repairs. A defect found in `design.md` is still worth raising,
but as a design finding routed back to design per `docs/AGENTS.md`, not as a
reason to restructure the plan.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | |
| F-2 | major | fix-now | |
| F-3 | major | fix-now | |
| F-4 | major | fix-now | |
| F-5 | major | fix-now / aligned | |
| F-6 | major | fix-now | |

### F-1 — Probe execution is used as an entry criterion without an earlier phase discharging it

**Severity:** major
**Location:** `plan.md` PHASE-05/EN-2 and PHASE-06/EN-2

**Expected:** Each phase entry criterion is discharged by an earlier phase's exit
criterion; `docs/AGENTS.md` §Phase plan says entry criteria are verified before
starting and that if they are not met, *the previous phase is not done*.
**Observed:** PHASE-05/EN-2 requires `transport-probe.local.rs` to have been run
"in this session"; PHASE-06/EN-2 conditionally requires re-running it. No earlier
phase exit criterion produces either fact. These are useful preflight actions but
they are not prior-phase gates.
**Evidence:** `docs/slices/001/plan.md` PHASE-05 Entry, PHASE-06 Entry;
`docs/AGENTS.md` §Phase plan; `review-plan.md` §Brief axis 1.

**Disposition:** fix-now
**Response:** Correct, and it matters for the reason the finding gives rather
than as bookkeeping: `docs/AGENTS.md` gives an unmet entry criterion one meaning
— *the previous phase is not done* — and neither PHASE-04 nor PHASE-05 owes a
probe run. The probe stays, and stays first; it moves from Entry to PHASE-05/EX-6
and PHASE-06/EX-6, where discharging it is this phase's job and the phase sheet
records the output. PHASE-05's implementer notes now say to run it *before*
writing `process.rs`, which the Entry placement left ambiguous.

**Outcome:**

### F-2 — AC-6's no-panic mechanism is only owned for PHASE-04-era modules

**Severity:** major
**Location:** `plan.md` PHASE-04/EX-5; Coverage row AC-6

**Expected:** AC-6 requires "No path panics"; design I9 and draft spec R-46 hold
that with module-level restriction lints on the modules handling backend-derived
data. Later shell modules that parse process output and compose host outcomes
also handle backend-derived data.
**Observed:** PHASE-04/EX-5 places the lints because that phase is "the first to
handle such data", but PHASE-05 through PHASE-08 add `process.rs`, `host.rs` and
integration paths handling backend output with no criterion requiring the same
attributes there. The AC-6 coverage row points at protocol and transport tests,
not at the R-46 lint mechanism for the later modules.
**Evidence:** `slice-001.md` AC-6; `design.md` §5.5 I9 and §9's AC-6 map;
`draft-spec.md` R-46 and its §7 row; `plan.md` PHASE-04/EX-5, PHASE-05 Surfaces,
PHASE-07 Surfaces, Coverage AC-6.

**Disposition:** fix-now
**Response:** Correct, and the most consequential of the six. The first draft's
wording — PHASE-04 "is the first to handle such data and therefore owns placing
them" — states an ownership rule that expires the moment a second module handles
that data, and `process.rs` and `host.rs` both do. The class fix, not the
instance: the rule is now stated once in the plan's Overview as item 4, over the
*data* rather than over a phase or a directory, with the break-it-and-revert
proof attached to it. PHASE-04/EX-5 is narrowed to the two modules it actually
writes; PHASE-05/EX-7 and PHASE-07/EX-7 carry the same obligation for
`process.rs`, `shell/error.rs` and `host.rs`. `config.rs` and `state.rs` are
excluded and the exclusion is argued: a config file is the user's own and a
`view_id` is host-minted. AC-6's coverage row now names the lint rule as its
no-panic mechanism rather than leaving that clause pointing at tests.

**Outcome:**

### F-3 — AC-5's cancellation clause is asserted but not falsifiably verified

**Severity:** major
**Location:** `plan.md` PHASE-06/EX-5 and VT-1…VT-5

**Expected:** AC-5 requires the cancelled-exchange claim to be stated narrowly
and held structurally: no host task, buffer or descriptor survives cancellation,
while the child falls to `kill_on_drop`. A plan criterion must name something
that can fail.
**Observed:** PHASE-06/EX-5 says a cancelled exchange asserts the narrow claim,
but the verification criteria cover the stdout flood, the stderr flood, the
grandchild cases, wedged cleanup, and no children after the misbehaving suite.
None names a cancellation test or source check that would fail if the transport
retained a task, buffer, descriptor or handle after drop.
**Evidence:** `slice-001.md` AC-5; `draft-spec.md` §7's R-48/R-54 row; `plan.md`
PHASE-06/EX-5 and VT-1…VT-5.

**Disposition:** fix-now
**Response:** Correct. EX-5 asserted the clause and nothing could falsify it,
which is the design review's own F-51/F-57/F-62 defect wearing plan clothing.
PHASE-06/VT-6 now holds it in two ways. Structural: the `tokio::spawn` grep from
PHASE-05/VT-5 is asserted again here against the finished module, so the
criterion and its mechanism live in the same phase. Behavioural: start an
exchange against a backend that will not answer, drop the future, then drop the
runtime with `shutdown_timeout(Duration::ZERO)` and assert it returns promptly —
a detached task blocks that call, so the test fails if the structure regresses.
Neither asserts anything about the child, which is what D54 concedes and AC-5
states.

**Outcome:**

### F-4 — AC-2's unknown-optional-field coverage is nominal

**Severity:** major
**Location:** `plan.md` Coverage row AC-2; PHASE-04/EX-1, EX-3, VT-3

**Expected:** AC-2 requires unknown optional fields to be ignored. Draft spec
R-4/R-5 verify this with protocol-tier fixtures carrying unmodelled fields **at
each level**.
**Observed:** the coverage row maps this to PHASE-04/EX-1 and EX-3. EX-1 requires
no `deny_unknown_fields`, which is a structural serde condition rather than the
required behavioural fixture coverage at each level. EX-3 is about unrecognised
required `kind` discriminants. VT-3 covers one optional misspelling pair, not
unmodelled fields at each inbound level.
**Evidence:** `slice-001.md` AC-2; `draft-spec.md` R-4/R-5 and their §7 row;
`plan.md` Coverage AC-2, PHASE-04/EX-1, EX-3, VT-3.

**Disposition:** fix-now
**Response:** Correct. "No `deny_unknown_fields`" is a property of the type
declaration; R-5's §7 row asks for fixtures carrying unmodelled fields **at each
level**, which is a property of the behaviour, and the two are not the same
claim — a `#[serde(flatten)]` map or a dispatch site could swallow or reject one
without the structural condition changing. PHASE-04/EX-8 now requires the
behaviour at envelope, view, option, field and content level, and VT-4 asserts
each: accepted, absent from the canonical value, and **nothing discarded** — the
silence being the assertion, borrowed from R-51's own §7 row. The Coverage row
for AC-2 points at EX-8 and VT-4 rather than at EX-1.

**Outcome:**

### F-5 — Several VA criteria are manual confirmation rather than failing mechanisms

**Severity:** major
**Location:** `plan.md` PHASE-05/VA-2, PHASE-06/VA-2, PHASE-08/VA-2, PHASE-09/VA-2

**Expected:** the review brief asks whether every `EX-`, `VT-`, `VA-` and `VH-`
criterion can actually fail, and states that a `VA-` amounting to "check
carefully" is the same defect as a claim held by nothing.
**Observed:** PHASE-05/VA-2 says to read `process.rs` line by line and confirm
four facts; PHASE-06/VA-2 says to re-read an edge-case table and confirm
coverage; PHASE-08/VA-2 says to walk a prose list and record gaps; PHASE-09/VA-2
says to perform an AC walk. These are review activities, but the plan does not
make the resulting gaps fail a command or a named test, so each can be satisfied
by recording the walk rather than by a falsifying mechanism.
**Evidence:** `review-plan.md` §Brief axis 2; `plan.md` PHASE-05/VA-2,
PHASE-06/VA-2, PHASE-08/VA-2, PHASE-09/VA-2.

**Disposition:** fix-now on the mechanisable half; aligned on the residue, with
the reason recorded.
**Response:** The finding is half right and the half it is right about was worth
raising. Three of PHASE-05/VA-2's four checks are string searches being done by
eye — no `tokio::spawn`, no `Arc`/`Mutex`, no `?` past the spawn — and each names
a regression that a design review round actually had to repair (F-49, D44's lock
deletion, F-41). They are now PHASE-05/VT-5, a source-text test in the same tier
and with the same found-no-files guard as PHASE-01's boundary checks. Two of
PHASE-09/VA-2's halves are likewise greps and are now PHASE-09/VT-2: struck or
superseded decision ids still cited elsewhere, and types named in `design.md` §5
with no definition in §5. Both have produced findings before — F-56 found D41 and
D42 still cited, F-55 found `WireOpt` named and undefined.

What stays a `VA` stays deliberately. `VA` is a verification mode the plan
template defines — "agent check" — not an absence of one, and the residue is
irreducibly a read: whether `child.wait()` sits inside the timed region or in the
cleanup budget is a scope structure that no string distinguishes (F-59), and
whether two statements of one contract in different sections still agree is what
`design.md` §9 says outright no test can observe. Mechanising those would replace
judgement with box-ticking, which is the failure mode F-56 already found in this
slice once. The plan now says which is which and why, so a later reader does not
read a `VA` as a `VT` that someone gave up on.

**Outcome:**

### F-6 — PHASE-08 is too large for the methodology's one-session phase size

**Severity:** major
**Location:** `plan.md` PHASE-08

**Expected:** `docs/AGENTS.md` requires phases be reasonable for one agent to
complete within one session, including bookkeeping.
**Observed:** PHASE-08 owns the TypeScript showcase backend, a bash round trip,
the full AC-7 round trip, AC-8 through the real transport, R-35's
host-validation behaviour, R-45's reuse of one `Host`, and an end-to-end failure
matrix over the protocol-level misbehaving-backend list. That is not wiring
existing pieces: it is example authoring, multiple backend fixtures, full-stack
assertions and a checklist over a long prose list, in one phase.
**Evidence:** `docs/AGENTS.md` §Plan and §Execute; `review-plan.md` §Brief axis 3
naming PHASE-08 as likely over-packed; `plan.md` PHASE-08/EX-1…EX-5, VT-1…VT-5,
VA-2.

**Disposition:** fix-now
**Response:** Correct. Split at the seam PHASE-05 and PHASE-06 already use — a
backend that works, versus backends that fail. PHASE-08 keeps the AC-7 round
trip, the deno showcase example and the bash backend; the new **PHASE-10** takes
the protocol-level failure matrix, R-45's one-`Host` reuse and the prose-list
walk, plus R-29's schedule-unchanged assertions moved up from PHASE-07's fake
transport to the real one. Phase ids are immutable and edits append, so the new
phase is 10 and the execution order is 01…08, 10, 09; PHASE-08's EX-4, EX-5, VT-4
and VA-2 are struck in place with pointers rather than deleted. PHASE-08's
implementer notes now require the harness to support a sequence of exchanges
against one `Host`, which is what PHASE-10/EX-2 needs and what would otherwise be
a retrofit.

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

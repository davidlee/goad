# Review — plan — Slice 001

**Subject:** plan — `docs/slices/001/plan.md`, as at 2026-08-26, read against
`docs/slices/001/design.md`, `docs/slices/001/slice-001.md`,
`docs/slices/001/draft-spec.md`, `docs/adr/001-one-way-strata.md`,
`docs/adr/002-single-crate-until-triggered.md` and `docs/AGENTS.md`.
**Reviewer:** fresh agent, no thread history (gpt-5.5 via MCP)
**Opened:** 2026-08-26
**Closed:** 2026-08-27
**State:** closed

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

**Round 4** — 2026-08-27 — **clean. No findings raised.** Confirmation of round
3's three repairs (F-12…F-14), the author's self-sweep sites treated as
unverified, and the first attack on the tests carve-out's *reasoning* rather than
its restatements. Same reviewer profile, fresh thread (gpt-5.5 via MCP); packet
in `review-packet-plan-r4.local.md`. Findings number from F-15. Intended as the
final round: no blocker and nothing a repair cannot close means the Synthesis is
written and `plan.md` goes to the user for acceptance.

Two stale sites were found **in the packet itself** while dispatching it, and
repaired before it was sent: its reading list still named "the ledger, rounds 1
and 2" and told the reviewer that "F-7…F-11 and their Responses are what this
round must verify" — round 3's text left standing — and its finding-format
example was headed `### F-7` while the instruction above it said to number from
F-15. Same failure mode as F-13, in the document written to hunt for it. Not
given a finding id — author-found, author-repaired — but recorded here as
further evidence for the rate.

**Result.** No findings. **F-12, F-13 and F-14 all confirmed**, each with cited
evidence: F-12 on the widened definition-site grep at PHASE-05/VT-5 plus VT-6's
positive control; F-13 on all five sites now stating command-sequence
equivalence rather than literal text, with no sixth active restatement; F-14 on
the carve-out being stated at all seven sites and on the break-it-and-revert
instructions at PHASE-04/VA-2, PHASE-05/EX-7 and PHASE-07/EX-7 all pointing at
host modules. On priority 2 — the carve-out's reasoning, never previously
attacked — the reviewer found the argument holds: R-46 and I9 constrain host
run-time paths handling backend-derived data, test code including
`tests/protocol/` is assertion code, and `unwrap_in_result` remains uncarved.
Verdict on the plan: **executable as it stands.**

The reviewer stated one limit on its own coverage, as the packet asked: it did
not re-run the tokio metrics probe behind F-12's positive control, and verified
the documents and the current dependency features instead. Thread
`01a04244-8fe2-7630-9d38-784ed05b4fa7`; treat as spent.

**Round 3** — 2026-08-27 — confirmation of round 2's five repairs, plus first
review of the clippy tests carve-out. Same reviewer profile, fresh thread; packet
in `review-packet-plan-r3.local.md`. Three findings, F-12…F-14, all `major`, no
blocker. **F-7, F-10 and F-11 confirmed; F-8 and F-9 incomplete.**

The defect rate is falling — 4 defects per 6 repairs at round 2, 2 per 5 at round
3 — and the failure mode has not changed. F-13 is the restatement defect again,
and this time in text written the same day by the author who had just described
that defect twice in this ledger: `plan.md` PHASE-01/VA-3 was repaired while
`design.md` §9 and the `justfile` header went on asserting the line-for-line
claim the repair existed to delete. F-12 is the falsifiability defect again: the
replacement mechanism was sound but the test could reach it without the exchange
having started, and the accompanying grep named one spawn API out of five.

F-14 is the round's own new material and it is the carve-out's first review: it
found that `plan.md` still told an implementer to prove the crate-wide lints by
breaking them "anywhere", which the carve-out had just made false for `tests/`.

**Author's self-sweep after round 3, before round 4 was packeted.** Prompted by
the observation that roughly four in five repairs in this slice have produced a
new finding. It found **five more stale sites** that round 3's repairs had not
reached, in the same two contracts round 3 had just found stale:

| contract | site the repair missed |
|---|---|
| F-13, the mirroring | `notes.md`'s Open section — "must match it line for line" |
| F-13 | `design-log.md`'s 2026-08-27 entry — the same claim, in the record of the decision |
| F-14, the tests carve-out | `draft-spec.md` §7's R-46 row — "crate-wide" with no carve-out |
| F-14 | `design.md` §5.5's I9 row — same |
| F-14 | `design-log.md` had no entry for the carve-out at all; it was in `clippy.toml`, `design.md` §9 and this ledger, and nowhere in the decision record |

None is raised as a finding — they were found and repaired by the author, and
this ledger records raised findings. They are recorded here because they are the
strongest available evidence for the rate: F-13's own repair, made in direct
response to a finding *about* unswept restatements, was itself unswept at two
sites. **Round 4 should assume the same of everything repaired since round 3**,
and should treat the five sites above as repaired-but-unverified.

**Round 2** — 2026-08-27 — confirmation of round 1's six repairs, plus the two
user decisions of 2026-08-27 (D53 amended; `just` adopted as the canonical
runner), which nothing had reviewed.

Fresh reviewer, no thread history (gpt-5.5 via MCP), packet in
`review-packet-plan-r2.local.md`. Five findings, F-7…F-11, all `major`, no
blocker. Of the six repairs: **F-2 and F-5 confirmed; F-1, F-3, F-4 and F-6
defective** — 4 defects per 6 repairs, above the design review's 0.9/0.6/0.2 per
round and vindicating the decision to run this round rather than accept.

Three of the four defective repairs failed the same way: the repair was applied
at the site the finding named and not at the sites that restate it. F-1 moved the
probe out of two `EN-` lists and did not re-check the other `EN-` lists against
the rule it invoked; F-4 enumerated the inbound levels and missed one; F-6 split
the phase and left the new phase's objective claiming what only the old phase's
scope would have covered. That is `design.md` §9's restatement sweep, unrun
against this batch, for the third time in this slice.

Nothing in the round found a defect in the D53 amendment or the `just` adoption
as *stated* — but see F-9, which is a defect in how PHASE-01 verifies the `just`
mirroring, and the tests carve-out below, which the packet could not have
covered because it was found after the packet was sent.

**Found by the author, after the packet was dispatched and outside the round:**
the crate-wide form of D53 fires inside both test targets. A scratch crate
carrying goad's lint table fails `cargo clippy --all-targets -- -D warnings` with
five errors on ordinary test code — `unwrap()` on a fixture, `v[0]` on a known
vector — and exits 0 with `allow-unwrap-in-tests`, `allow-expect-in-tests`,
`allow-panic-in-tests` and `allow-indexing-slicing-in-tests` in `clippy.toml`.
All four are accepted by this toolchain. Repaired in `clippy.toml` and recorded
in `design.md` §9; `unwrap_in_result = "deny"` is deliberately not scoped away.
Not given a finding id: it was raised and repaired by the author, and the ledger
records raised findings, but a round 3 should attack the carve-out.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | repair **incomplete** at round 2 → F-7 |
| F-2 | major | fix-now | `verified` (round 2) |
| F-3 | major | fix-now | repair **defective** at round 2 → F-8 |
| F-4 | major | fix-now | repair **incomplete** at round 2 → F-11 |
| F-5 | major | fix-now / aligned | `verified` (round 2) |
| F-6 | major | fix-now | repair **incomplete** at round 2 → F-10 |
| F-7 | major | fix-now | `verified` (round 3) |
| F-8 | major | fix-now | repair **incomplete** at round 3 → F-12 |
| F-9 | major | fix-now | repair **incomplete** at round 3 → F-13 |
| F-10 | major | fix-now + doc-wrong | `verified` (round 3) |
| F-11 | major | fix-now | `verified` (round 3) |
| F-12 | major | fix-now | |
| F-13 | major | fix-now | |
| F-14 | major | fix-now | |

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

**Outcome:** **Round 2: repair incomplete.** The probe moved, but the rule the repair
invoked was not applied to the other `EN-` lists. Superseded by F-7.


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

**Outcome:** **Round 2: `verified`.** The obligation is stated over the data in Overview
item 4 and carried by PHASE-04, PHASE-05 and PHASE-07; the `config.rs` /
`state.rs` exclusion holds on the stated rule.


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

**Outcome:** **Round 2: repair defective.** The behavioural half could not fail —
`shutdown_timeout` never waits on async tasks. Superseded by F-8, where it is
measured.


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

**Outcome:** **Round 2: repair incomplete.** The enumeration of inbound levels predated
D52 and omitted `Alternative`. Superseded by F-11.


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

**Outcome:** **Round 2: `verified`.** The mechanisable half is mechanised; the residue is
judgement over scope structure and English agreement, and the plan says which is
which.


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

**Outcome:** **Round 2: repair incomplete.** The split is right; PHASE-10's objective
claimed more than its criteria specified. Superseded by F-10.


### F-7 — Phase entries omit prior exit criteria and still use entry criteria for same-phase work

**Severity:** major
**Location:** `plan.md` PHASE-07/Entry, PHASE-08/Entry; `docs/AGENTS.md` §Phase plan

**Expected:** after F-1, every `EN-` obeys `docs/AGENTS.md` §Phase plan: entry
criteria are verified before starting, and if unmet, *the previous phase is not
done*. A phase entry must not omit exit criteria the preceding phase owes, and
must not require work owned by the phase being entered.
**Observed:** PHASE-07/EN-1 requires PHASE-06/EX-1…EX-5, omitting PHASE-06/EX-6.
PHASE-08/EN-1 requires PHASE-07/EX-1…EX-6, omitting PHASE-07/EX-7.
PHASE-07/EN-2 requires the TOML parser to be added under the `shell` feature,
but PHASE-07's own Surfaces name `Cargo.toml` "(the TOML parser)" and no earlier
exit criterion discharges that addition.
**Evidence:** `docs/AGENTS.md` §Phase plan; `plan.md` PHASE-06/EX-6 and
PHASE-07/EN-1; PHASE-07/EX-7 and PHASE-08/EN-1; PHASE-07 Surfaces and EN-2;
`plan.md` "Decisions taken during planning", which says PHASE-01's manifest is
touched by the TOML decision, while PHASE-01's Exit has no `toml` criterion.

**Author's check:** confirmed by reading. The two omitted ranges are exactly the
criteria the F-1 and F-2 repairs appended — `EX-6` to PHASE-06, `EX-7` to
PHASE-07 — and the `EN-` lists that cite them by range were not re-read. This is
the F-56 class again, and the third instance in this slice.

**Disposition:** fix-now
**Response:** Correct, and the class fix is not to widen the two stale ranges but to
stop citing ranges. An `EN-` that names `EX-1…EX-n` is a statement about how many
criteria the previous phase had *when the line was written*, and both stale
citations are exactly the ranges the F-1 and F-2 repairs extended. PHASE-06,
PHASE-07, PHASE-08, PHASE-09 and PHASE-10 now enter on **"PHASE-nn discharged"**,
which cannot go stale. The selective entries — PHASE-02, PHASE-03, PHASE-04,
PHASE-05 — keep naming specific criteria, because there the subset is a
deliberate claim with a reason attached, not an accident of counting.

The TOML half is a separate defect and the finding is right about it too: an
entry criterion may not require work the entering phase owns. `toml` moves to
**PHASE-01/EX-6**, where the manifest already lives; PHASE-07's Surfaces lose
`Cargo.toml` and its EN-2 now points at PHASE-01/EX-6. It is an unused dependency
for six phases, which costs nothing because `unused_crate_dependencies` is off in
`[lints.rust]` for this class of reason.

**Outcome:** **Round 3: `verified`.** Whole-phase entry criteria where a range would go
stale, specific criteria where the subset is deliberate; `toml` at PHASE-01/EX-6
and out of PHASE-07's surfaces.


### F-8 — The cancellation test can pass even with detached spawned work

**Severity:** major
**Location:** `plan.md` PHASE-06/VT-6

**Expected:** F-3 required a cancellation verification that fails if the
transport retains a task, buffer, descriptor or handle after the exchange future
is dropped.
**Observed:** VT-6's behavioural half drops the future, then calls
`shutdown_timeout(Duration::ZERO)` and asserts it returns promptly. A zero
timeout returns promptly whether or not spawned work is still alive, so the
assertion cannot fail.
**Evidence:** `plan.md` PHASE-06/VT-6; `review-plan.md` F-3 Response;
`slice-001.md` AC-5; tokio's `Runtime::shutdown_timeout` documentation.

**Author's check: confirmed, by measurement, and the mechanism is worse than the
finding states.** `shutdown_timeout` bounds the wait for *blocking* tasks; async
tasks are aborted at shutdown and never delay it, so no timeout value makes this
assertion sensitive to a detached async drain. Measured on this toolchain
(scratch crate, tokio multi-thread runtime): with a live detached async task
`shutdown_timeout(ZERO)` returned in **271 µs**; with nothing spawned, **144 µs**.
Both prompt.

The same probe found a mechanism that does discriminate:
`Handle::metrics().num_alive_tasks()` read **1** with the detached task alive and
**0** with nothing spawned, and compiles with no `tokio_unstable` cfg. That is
the shape a repair should take — assert the count is zero after the future is
dropped — and it fails for exactly the reason F-49 existed.

**Disposition:** fix-now
**Response:** Correct, and worse than raised — confirmed by measurement rather than by
reading tokio's docs. `shutdown_timeout` bounds the wait for *blocking* tasks;
async tasks are aborted at shutdown and never delay it, so **no** timeout value
makes that assertion sensitive to a detached drain. On this toolchain: 271 µs
with a live detached async task, 144 µs with nothing spawned.

The same probe supplied the replacement. `Handle::metrics().num_alive_tasks()`
read 1 and 0 on those two cases and needs no `tokio_unstable`. VT-6's behavioural
half is now that assertion, with the note that the test must own the runtime it
measures. This is the second time in this slice a criterion was written to be
falsifiable and was not — F-3 raised it, and the repair reproduced it.

**Outcome:** **Round 3: repair incomplete.** The mechanism is right; the test could reach
it without the exchange having been polled, and the companion grep covered one
spawn API of five. Superseded by F-12.


### F-9 — `just -n check` cannot match the canonical §9 command block line for line

**Severity:** major
**Location:** `plan.md` PHASE-01/VA-3; `design.md` §9; `justfile`

**Expected:** PHASE-01/VA-3 requires `just -n check` to print §9's six commands
in §9's order, matching that block line for line.
**Observed:** §9's block carries inline comments and a physical line
continuation on the second clippy command; `just -n check` prints six executable
lines with neither. The recipes mirror the commands, but the criterion as
written is false and would fail on a correct justfile.
**Evidence:** `design.md` §9's block; `plan.md` PHASE-01/VA-3; `justfile`;
observed `just -n check` output.

**Author's check:** confirmed, and it is the author's own wording from
2026-08-27, written the same day. A criterion that a correct implementation
fails is worse than no criterion: the first implementer either weakens it or
mangles §9 to satisfy it.

**Disposition:** fix-now
**Response:** Correct, and it is the author's own wording from the same day. A
criterion a correct implementation fails is worse than none: the first
implementer either weakens it or mangles §9 to satisfy it. VA-3 now compares the
**command sequence** — same commands, same arguments, same order — and says why
a literal comparison is wrong, since §9's block carries comments and a line
continuation that `just -n` does not print.

**Outcome:** **Round 3: repair incomplete.** Repaired at PHASE-01/VA-3 and left standing in
`design.md` §9 and the `justfile` header. Superseded by F-13.


### F-10 — PHASE-10 claims the AC-6 failure matrix but specifies only the protocol-level subset

**Severity:** major
**Location:** `plan.md` PHASE-10 Objective, EX-1, VT-1; Coverage row AC-6

**Expected:** PHASE-10's objective says every failure mode AC-6 names is asserted
through the whole stack, and the Coverage row for AC-6 says PHASE-10/VT-1 covers
"each mode end to end".
**Observed:** EX-1 lists only protocol-level misbehaving responses — version,
empty options, duplicate ids, unknown kind, omitted view, bad `next_check`, bad
bounds, inapplicable keys, `null` cases. The transport and lifecycle modes —
command not spawnable, timeout, non-zero exit, malformed stdout, output too
large, cleanup-failure combinations, stale or unknown interaction through the
full `Outcome` — are absent from the end-to-end matrix. Several are tested
earlier at transport or fake-backend level, which does not satisfy this phase's
own "whole stack" claim.
**Evidence:** `slice-001.md` AC-6; `design.md` §9's AC-6 row and its
misbehaving-backend list; `plan.md` Coverage AC-6, PHASE-10 Objective, EX-1…EX-3,
VT-1.

**Author's check:** confirmed. PHASE-10/EX-3 does bring timeout, non-zero exit
and malformed JSON through the real transport, but only to assert the *schedule*
is unchanged, not the variant. The gap between the objective and EX-1 is real,
and the Coverage row is the overclaim that makes it dangerous — it reads as
covered.

**Disposition:** fix-now + doc-wrong
**Response:** Correct, and both halves needed answering. The objective is narrowed to
what PHASE-10 actually owns: the protocol-level matrix. The transport and
lifecycle modes stay where they are held, at PHASE-05 and PHASE-06 — re-asserting
them here would re-inflate the phase F-6 split for size, which is how the
overclaim got in.

But there was a real gap inside the overclaim, and narrowing alone would have
buried it: PHASE-05 and PHASE-06 assert those modes **at the transport**, and
nothing asserted what a caller receives. **PHASE-10/EX-4 and VT-3** now do —
unspawnable command, timeout, non-zero exit, malformed stdout, output past the
cap, stale or unknown `view_id`, one exchange each through `Host`, asserting the
`Outcome`. The AC-6 coverage row states the split rather than implying PHASE-10
covers everything.

**Outcome:** **Round 3: `verified`.** The objective is narrowed, EX-4/VT-3 carry the
caller-visible modes, and no AC-6 clause is left behind the narrowed EX-1.


### F-11 — Unknown-field coverage omits choice-field alternatives as an inbound level

**Severity:** major
**Location:** `plan.md` PHASE-04/EX-8 and VT-4

**Expected:** AC-2 and R-4/R-5 require unmodelled inbound fields to be ignored on
any inbound message; F-4's repair claims coverage at every inbound level.
**Observed:** EX-8 and VT-4 enumerate envelope, view, option, field and content
block. A `choice` field's `options` are read as `Alternative`, an inbound object
with its own shape distinct from a view's option. The plan tests that a
modelled-but-inapplicable key is rejected there, but not that an unmodelled key
is ignored.
**Evidence:** `plan.md` PHASE-04/EX-8, VT-4; `design.md` §5.2 on `Alternative`;
`draft-spec.md` R-53, R-4/R-5 and their §7 row; `slice-001.md` AC-2.

**Author's check:** confirmed. `Alternative` was created by D52/F-61 precisely
because an alternative is not an option, and the enumeration that F-4's repair
produced is the pre-D52 list of levels. The repair inherited a stale enumeration.

**Disposition:** fix-now
**Response:** Correct, and the mechanism of the miss is worth recording: `Alternative`
exists *because* D52/F-61 ruled that an alternative is not an option, and the
repair enumerated the five levels that predated that decision. A repair that
inherits a stale enumeration is the same defect as one that inherits a stale
restatement. EX-8 and VT-4 now carry six levels.

**Outcome:** **Round 3: `verified`.** Six inbound levels, `Alternative` included, and no
seventh inbound object the design defines is missing.


### F-12 — Cancellation metric can pass without exercising the exchange

**Severity:** major
**Location:** `plan.md` PHASE-06/VT-6

**Expected:** F-8's repair must make the cancellation test fail if dropping a
live exchange leaves detached host work behind.
**Observed:** VT-6 says to start an exchange, drop the future, let the runtime
settle, and assert `num_alive_tasks() == 0`, but does not require the future to
be polled to the point where any work has begun. Rust futures are lazy: a future
dropped before first poll leaves the count at zero however the transport is
written. The structural half also names only `tokio::spawn`, not every
task-spawning API.
**Evidence:** `plan.md` PHASE-06/VT-6; `design.md` §5.4 on cancellation;
`draft-spec.md` R-48/R-54's verification row.

**Disposition:** fix-now
**Response:** Correct on both halves, and the first is the more serious: an
assertion that is vacuous when the code is *right* is indistinguishable from one
that is vacuous when the code is wrong. VT-6 now spawns the exchange as its own
task, waits until the count is **≥ 1** — a positive control proving the metric is
live and would see a leak — then aborts, settles, and asserts zero. That is the
same shape as the found-no-files guard on PHASE-01's boundary greps, and for the
same reason.

The grep is widened at its **definition** site, PHASE-05/VT-5, not only where
PHASE-06 re-asserts it: the token `spawn`, with `Command::spawn` the sole
permitted occurrence. `tokio::spawn` alone let `Handle::spawn`, `spawn_blocking`,
`spawn_local` and `JoinSet::spawn` through, and F-49's leak needs one of them.
Repairing only the re-assertion would have been F-13's defect in the same breath.

**Outcome:**

### F-13 — The `just` mirroring repair was not swept through its restatements

**Severity:** major
**Location:** `plan.md` PHASE-01/VA-3; `design.md` §9; `justfile` header

**Expected:** F-9's repair replaces literal line matching with same commands,
same arguments, same sequence, because §9 carries comments and a line
continuation `just -n` cannot print.
**Observed:** PHASE-01/VA-3 compares the command sequence. `design.md` §9 still
says `just -n check` "must match this block line for line", and the `justfile`
header still says each recipe mirrors a line of §9 "verbatim". The repair is
present at the finding site and stale at the primary reference and the
implementation-facing header.
**Evidence:** `plan.md` PHASE-01/VA-3; `design.md` §9; `justfile` header;
`review-plan.md` F-9 Response.

**Disposition:** fix-now
**Response:** Correct, and it is the sharpest instance of this slice's recurring
defect: the stale sites are text the author wrote the same day, in the same
session in which this ledger twice described that exact failure. Both repaired —
§9 now states the sequence comparison and why a literal one is wrong, and the
`justfile` header says the same. §9 remains the canonical statement; what changed
is what "mirrors" means.

The transferable lesson is that a repair's blast radius is not the finding's
`Location:` line. F-9's location named `plan.md`; the contract lived in three
files.

**Outcome:**

### F-14 — The test carve-out makes the lint proof's "anywhere" criterion false

**Severity:** major
**Location:** `plan.md` Overview item 4 and PHASE-04/VA-2; `clippy.toml`;
`design.md` §9

**Expected:** after the explicit test carve-out, any break-it-and-revert proof
must name a location where the lint is still active.
**Observed:** `plan.md` says the three no-panic restriction lints are crate-wide
and PHASE-04/VA-2 proves that with "an `unwrap()` anywhere", while `clippy.toml`
sets the four `allow-*-in-tests` keys and `design.md` §9 says tests are carved
out. An `unwrap()` in `tests/protocol/` or `tests/integration/` is "anywhere" and
is expected to pass, so the criterion can fail to prove the mechanism it claims.
**Evidence:** `plan.md` Overview item 4, PHASE-04/VA-2; `clippy.toml`;
`design.md` §9's "Tests are carved out" paragraph; `draft-spec.md` R-46's row.

**Disposition:** fix-now
**Response:** Correct, and the carve-out's first review earned its round. The
carve-out was landed in `clippy.toml` and `design.md` §9 and not carried into the
plan's statement of the same contract — the restatement defect once more, on
material less than a day old. PHASE-04/VA-2 now names `src/semantics/normalize.rs`
for both halves of the proof and says why "anywhere" is wrong; Overview item 4
carries the carve-out and the same instruction.

The finding does not claim the carve-out itself is unsound and neither does this
response: what it gives up is `unwrap()` in test code, `unwrap_in_result` is not
carved out and still reaches tests, and I9 is about paths handling
backend-derived data at run time. A round 4 should attack that reasoning rather
than the wording.

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

**Closed 2026-08-27, on a clean round.** Four rounds, fourteen findings, all
`major`, none contested, all repaired, and every repair independently confirmed.
The plan is judged executable as it stands. This is a different closure from the
design review's, which was closed by user decision with sixteen repairs
unverified; here nothing is outstanding by severity *and* nothing is outstanding
by confirmation.

### What the review changed

Not the plan's shape. Ten phases in the order 01…08, 10, 09 survived four rounds
intact — no phase was resequenced, and PHASE-04 and PHASE-06, named as
over-packed in round 1, were left whole after argument rather than by neglect.
The one structural change was round 1's F-6 split, which produced PHASE-10.

What it changed was **whether the plan's claims are held by anything.** Two
classes account for almost all fourteen findings:

1. **Criteria that named a mechanism without having one.** PHASE-06/VT-6 was
   `shutdown_timeout(ZERO)`, which cannot fail — measured. PHASE-01/VA-3 demanded
   a line-for-line justfile match that a *correct* justfile fails. F-8's
   replacement for VT-6 was itself vacuous until F-12, because a lazy future
   dropped before its first poll spawns nothing however the transport is written.
   Three iterations on one criterion before it could fail for the right reason.
2. **Repairs applied at the named site and left standing at every site that
   restates the same contract.** F-1, F-4, F-6, F-13, F-14 — and the design
   review's F-56 before them.

### What it confirmed

- Both user decisions of 2026-08-27 — D53 amended to the crate-wide/per-module
  split, `just` adopted as canonical runner — are sound *as stated*. Round 2
  found no defect in either; what it found (F-9) was a defect in how PHASE-01
  verified the `just` mirroring.
- The tests carve-out in `clippy.toml` — an **author** decision taken under D53,
  because D53's amended form is unimplementable without it. Round 4 attacked its
  reasoning, not just its wording, and found it holds. Flagged reversible in
  `design-log.md`; reverting costs one `#[expect]` per asserting test.
- The plan's coverage of AC-1…AC-15 is real, not nominal, after F-4 and F-11.

### The rate, and what it says about the process

| round | repairs checked | found defective |
|---|---|---|
| 2 | 6 | **4** |
| 3 | 5 | **2** |
| author's self-sweep after round 3 | 3 | 5 stale sites |
| 4 | 3 | **0** |

Roughly four repairs in five produced a new finding, until round 4. The rate
fell to zero only after the author's self-sweep applied the restatement sweep
deliberately — and the strongest single datum in this ledger is that the sweep
found five sites that round 3's repairs had missed, including F-13's own repair,
which was made in direct response to a finding *about* unswept restatements. The
packet for round 4 then had two stale sites of its own, found at dispatch.

The lesson is mechanical, not moral: **a repair is not done when the named site
is fixed; it is done when every site that states the same contract has been
grepped.** `design.md` §9's restatement sweep exists for this and names §5.5, §7
and §9's AC map. `draft-spec.md` §7, `plan.md`'s Overview and Coverage table,
`design-log.md`, `notes.md` and the `justfile` each held a stale restatement at
least once in this slice.

### Risk this closure knowingly leaves standing

- **Round 4 did not re-run the tokio metrics probe** behind PHASE-06/VT-6's
  positive control; it verified the documents and the dependency features. VT-6
  has now been rewritten three times, and its mechanism is confirmed on paper
  only. PHASE-06 should run it before trusting it — `transport-probe.local.rs`
  is the fastest route.
- **Priority 3 — the self-sweep's five sites, PHASE-08's split comment and
  PHASE-06's cancellation note — got no per-site verdict**, only the round's
  overall "no new findings". Weaker evidence than the explicit F-12…F-14
  confirmations, and the weakest link in this closure.
- **The design's sixteen unverified repairs are untouched by this review** and
  remain as `review-design.md`'s Synthesis left them: expect two to four residual
  defects, most likely in `design.md` §5.4. The plan puts §5.4 across PHASE-05
  and PHASE-06, which are the phases that should get the most verification.
- **No phase sheets exist.** Each is written immediately before its phase, so
  every phase still carries the risk that its detail does not survive contact.

### Handover

`plan.md` is ready for user acceptance. **No code before that acceptance**
(`CLAUDE.md`). After it: phase sheets one at a time, PHASE-01 first, and its
VH-1 reload of the dev shell is not optional — `just` and `deno` entered
`flake.nix` after some working shells were started.

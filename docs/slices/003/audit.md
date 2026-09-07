# Audit & reconciliation — Slice 003

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** branch `slice-002`, commit range `0b2e50f..3777c22` — the six
execution phases of slice 003, plus the slice's documents and its two drafts
(`canon-delta.md` against SPEC-001, `draft-spec.md` for the host's scheduling
behaviour).

**Question:** for this slice to be finished, all of the following would have to
be true. The audit checks each of them directly, by running the thing rather
than by reading the sheet that says it was run.

1. **The timer does what the slice said it would.** All twelve acceptance
   criteria in `slice-003.md` hold against the tree as committed, and every
   VT-/VA- criterion named in `plan.md` discharges when the audit runs the
   command itself. A criterion marked `[x]` by an execution agent is a claim,
   not evidence; each is re-run here. AC-1 through AC-5 and AC-9 are the
   behavioural core — the wait exists, both directions move it, the earlier
   instruction wins, a past instant fires once without spinning, a failing
   backend is polled on cadence and no faster, and an unreadable clock neither
   loses the schedule nor loops.

2. **The five invariants of `CLAUDE.md` are intact.**
   - *No domain vocabulary.* AC-11's scan, run explicitly over the finished
     tree, not inherited from a phase's own pass.
   - *Permissive wire, canonical interior.* `Stimulus::Scheduled` is a new wire
     value (D-10); the audit checks that it entered through normalization like
     its siblings and that nothing downstream of the door is unvalidated.
   - *No narrowing of wire compatibility to match the renderer.* The new
     `kind` = `"scheduled"` must not have been made the renderer's business.
   - *A backend failure never takes the host down, and never leaves it unable
     to invoke the backend again.* AC-5, AC-8 and AC-9 are this invariant in
     three forms; the audit reads them together, not one at a time.
   - *Strata run one way (ADR-001).* `cargo test --no-default-features` is the
     instrument. `wait_for` lands in stratum 1 and must take no clock; the
     `MINIMUM_SPACING` floor must live in stratum 3 beside the loop that
     applies it (D-14). Stratum 2 was declared to gain **nothing at all** —
     that is a checkable claim about `crates/goad-shell`.

3. **The surfaces the phases declared are the surfaces they touched.** Each
   phase commit is diffed against its `## PHASE-NN` surfaces list in
   `plan.md`. Undeclared paths are the audit's strongest lead and every one is
   listed, whether or not it turns out to be benign. Declared-but-untouched is
   read the other way: dropped work, or a design that moved and did not say so.
   Three surfaces were declared **unchanged** by name —
   `crates/goad/src/reception.rs`, `crates/goad-shell/src/host.rs`,
   `crates/goad/src/main.rs` — and a diff hit on any of them is a finding
   before it is anything else.

4. **The timing evidence survives a loaded machine.** AC-12 states outright
   that a test whose passing depends on machine load is a design defect, not a
   tolerated cost. The audit re-runs the timed targets under concurrent load
   and records the closest observed margin. The 6.9x row — AC-2's `respond`
   case — is the one under suspicion. A margin that narrows under load is
   reported as a defect even if the run passed.

5. **The gate is genuinely green, in its own terms.** `just check` exits 0 over
   six commands, none weakened, conditioned or `#[ignore]`d. POL-001 holds four
   ADR-001 instruments, the vocabulary scan, and one residue nothing enforces;
   the audit does not compress them into a single count, and checks that no
   document written this slice does either.

6. **The record is true about the code.** The two drafts are checked against
   what shipped and against `docs/templates/spec.md` and SPEC-001's own style
   before promotion is proposed. Draft SPEC-002 R-11 is marked "review, not
   test"; the audit performs that review, since nothing else will. Any decision
   that could be reversed by accident — the floor's monotonic anchor, "nothing
   persists" — is assessed for whether it needs an ADR rather than a spec line.
   Slice 002's follow-up F-5 (the production topology) was claimed by AC-10;
   the audit says with evidence whether it is discharged.

7. **Nothing is carried silently.** PHASE-06 recorded three design-drift items;
   the audit reconciles those and whatever else it finds, and the stray `(F-1)`
   citation at `crates/goad-semantics/src/schedule.rs` — which predates the
   slice and sits outside every phase's declared surfaces — gets a stated
   disposition rather than another slice's worth of inheritance.

**Out of scope for this document.** Defects in the implementation are the code
review's, and belong in `review-code.md`; a running reviewer owns that ledger
and this audit does not write to it. The audit does not fix code, does not
commit, and does not edit canon — promotion is proposed here and endorsed by
the user elsewhere.

## Evidence

Everything below was run by the audit against the working tree at `3777c22`,
in the dev shell. Nothing is taken from a phase sheet.

### The gate

| run | condition | exit | wall |
|---|---|---|---|
| `just check` | idle tree | 0 | (untimed, first run) |
| `just check` | idle tree | 0 | 5.365 s |
| `just check` | 32-core box at loadavg 164 | 0 | 21.464 s |

`just -n check` prints the six commands of `docs/policy/001-the-phase-gate.md`
§Compliance, in that order, character for character. No command is weakened,
conditioned, or removed.

`cargo test --workspace`: **303 passed, 0 failed, 0 ignored**, 5.056 s.
`grep -rn "#\[ignore\]" crates/ tests/` returns nothing. Sixteen test binaries.

### The instruments POL-001 counts separately

Run individually rather than inferred from a green gate, and reported as five
results because POL-001 §Verification forbids merging them into one:

1. **Crate-edge direction** — `cargo build --workspace` exits 0; no `E0433`.
2. **Manifest allowlist** — `cargo test -p goad-boundary --test checks --
   allowlist purity`: 11 passed.
3. **Stratum 1 `std` purity scan** — same binary, same result.
4. **Stratum 1 on its own features** — `cargo test -p goad-semantics`: 30
   passed, exit 0.
5. **The domain-vocabulary scan** (not one of the four) — `cargo test -p
   goad-boundary --test checks vocabulary`: 13 passed, including
   `no_workspace_member_names_the_users_domain` and
   `no_member_manifest_names_the_users_domain_in_its_own_crate_name`.

**A sixth instrument named by `CLAUDE.md` no longer exists.** `CLAUDE.md` says
*"`cargo test --no-default-features` is the compiler enforcing it, not a
convention."* Run: it compiles and passes the identical 303 tests, because no
manifest in the workspace declares a `[features]` table at all. POL-001
§Compliance already records why — the crate split left the feature with nothing
to gate, so `--no-default-features` "stops being a distinct column". The command
rejects nothing. `CLAUDE.md` is stale against its own canon; see Reconciliation.

### Acceptance criteria

Each criterion's own named test, run with `--exact` by the audit.

| AC | criterion | run | result |
|----|-----------|-----|--------|
| AC-1 | evaluates unprompted on the default poll | `renderer scheduling::a_short_default_poll_is_honoured_unfloored_for_the_first_scheduled_check` | **met**, 0.27 s |
| AC-2 | `next_check` from an `evaluate` | `renderer scheduling::an_instruction_from_an_evaluate_shortens_the_wait_past_a_far_default_poll` | **met**, 0.25 s |
| AC-2 | `next_check` from a `respond` | `renderer scheduling::an_instruction_from_a_respond_shortens_the_wait_past_a_far_default_poll` | **met**, 0.28 s |
| AC-3 | earlier supersedes | `renderer scheduling::an_earlier_instruction_supersedes_a_pending_far_deadline` | **met**, 0.26 s |
| AC-3 | later supersedes | `renderer scheduling::a_later_instruction_supersedes_and_the_earlier_deadline_does_not_fire` | **met**, 0.47 s |
| AC-4 | the arithmetic | five `wait_for` tests in `crates/goad-semantics/src/schedule.rs` | **met**, inside the 30 |
| AC-4 | a past instruction on every response | `renderer scheduling::a_past_instant_on_every_response_fires_once_and_then_holds_at_the_floor` | **met**, 0.67 s |
| AC-4 | a one-off past instruction | `renderer scheduling::a_one_off_past_instruction_is_consumed_and_cadence_resumes` | **met**, 0.68 s |
| AC-5 | a failing backend keeps its cadence, no faster | `renderer scheduling::a_failing_backend_is_retried_unprompted_never_faster_than_the_floor` | **met**, 0.76 s |
| AC-6 (a) | `resolve` names no production line under `crates/goad/src` | `checks structure::no_production_line_in_the_renderer_names_the_identifier_resolve` | **met** |
| AC-6 (b) | `schedule::resolve` occurs exactly twice, both in `host.rs` | `checks structure::schedule_resolve_is_called_only_from_host` | **met** |
| AC-7 | a timer does not defeat cancellation | `renderer scheduling::a_stop_issued_while_parked_on_the_timer_arm_ends_serve_well_inside_the_timeout` | **met**, 0.19 s |
| AC-8 | a failure does not stop the clock | the AC-5 test, read for liveness | **met** |
| AC-9 | an unreadable clock loses nothing and does not spin | `renderer scheduling::a_clock_that_fails_after_the_startup_exchange_refuses_and_holds`, with `::the_same_shape_with_a_working_clock_reaches_a_second_invocation` as its vacuity control | **met**, 0.67 s / 0.26 s |
| AC-10 | proved in the production topology, minus the platform | `event_loop_schedule scheduling::a_scheduled_evaluation_fires_under_the_production_topology` | **met**, 0.27 s |
| AC-11 | no domain vocabulary | the vocabulary scan, above | **met** |
| AC-12 | `just check` exits 0, nothing weakened, nothing load-sensitive | the gate table and the load evidence below | **met** |

Both AC-6 instruments assert a non-empty file walk before asserting the
absence or the count, so neither can pass on a walk that inspected nothing.
AC-6 (b) asserts `len == 2` before comparing the file-name set, so the set
equality is safe rather than the trap `docs/memory/assert-superset-not-equality-on-a-rename-set.md`
describes.

### Verification criteria in `plan.md`

Every VT- and VA- criterion the plan names was walked. All discharge. The
twelve `renderer::scheduling` tests and the one `event_loop_schedule` test
cover PHASE-02/VT-2..VT-8 and PHASE-03/VT-1..VT-6; PHASE-01/VT-1..VT-3 are
inside `goad-semantics`'s 30 and `wire.rs`'s inline module; PHASE-04/VT-1..VT-5
are inside the renderer's 136 and the boundary crate's 37; PHASE-05/VT-1..VT-2
are the event-loop target. The break-and-revert criteria (PHASE-02/VA-3,
PHASE-03/VA-3, PHASE-04/VA-2, PHASE-04/VA-3) are one-shot experiments recorded
in the phase sheets, not standing tests; the audit did not re-run them, and
says so rather than implying it did.

Draft SPEC-002 §7 names **22 test functions by name**. The audit resolved every
one of them to a file: all 22 exist. No requirement's *verified by* row points
at a test that is not there.

### The load evidence (AC-12's load clause)

`notes.md` records a closest margin of ~6.9x for AC-2 from a `respond` and
flags it for the audit. The audit re-measured it directly.

The margin the tests actually run on is the elapsed time inside `until`, not
the test's total wall time. To read it without touching the tree, the audit
built a detached `git worktree` at `3777c22`, instrumented the two `until`
helpers to print their elapsed time, measured, and removed the worktree. The
tree under audit was never modified.

Load was 32 busy loops plus a concurrent `cargo build --release`, then 128
busy loops: **loadavg 164-170 on a 32-core box**, roughly five times
oversubscribed.

| assertion | bound | worst elapsed under load | margin |
|---|---|---|---|
| AC-2 from a `respond` (the 6.9x row) | 2 s | 131.8 ms | **15.2x** |
| AC-1 first scheduled firing | 2 s | 130.2 ms | 15.4x |
| AC-2 from an `evaluate` | 2 s | 140.2 ms | 14.3x |
| AC-3 earlier supersedes | 2 s | 176.2 ms | 11.4x |
| AC-10 production topology | 2 s | 132.9 ms | 15.0x |
| worst of all `until` calls in the suite | 2 s | **184.9 ms** | **10.8x** |

**The suite was run 10 further times at loadavg ~170: 13/13 green every time,
130 test results, zero failures.** `just check` itself exits 0 at loadavg 164.

**The recorded 6.9x is a measurement artefact, not the margin.** PHASE-02
measured a span wider than the `until` bound covers — it included the child
process spawn that precedes the wait. The bounded quantity is ~2x safer than
the sheet claims, and it does not degrade under five-fold oversubscription: the
figures above are the loaded ones. AC-12's clause that no test's passing depends
on machine load holds on evidence. The number in `notes.md` is wrong in the safe
direction and should be corrected.

### Draft SPEC-002 R-11 — the review the requirement asks for

R-11 says the host must not evaluate once per interval that elapsed while it
was not running, and its *verified by* row says **review, not a test**. The
audit performed it.

- **Nothing persists.** No production path in any crate writes schedule state.
  The only `fs::write` in the workspace's source is a test helper in
  `crates/goad-shell/src/config.rs`, writing a temporary config file.
- **The seed cannot look backwards.** `Host::new` resolves through
  `schedule::resolve(None, None, default_poll, now)`, so a freshly started host
  holds `now + default_poll` and no earlier instant.
- **The loop holds no missed-interval concept.** `serve` arms one `Sleep` at
  `started + MINIMUM_SPACING`, which the startup exchange's outcome supersedes.
  There is no queue, no counter, and no iteration over elapsed intervals.

R-11 holds by construction. It is a requirement recorded for the slice that
adds persistence, and marking it *review* rather than *test* is honest — there
is no state a test could set up to falsify it.

### Surface delta

`git diff --stat 0b2e50f..3777c22`: 28 files, +3172 / -312. One commit per
phase. Each phase commit was diffed against its own `## PHASE-NN` **Surfaces**
list.

**Three files the slice declared it would not touch were not touched:**
`crates/goad/src/reception.rs`, `crates/goad-shell/src/host.rs`,
`crates/goad/src/main.rs`. Stratum 2 gained nothing, as designed.

**Undeclared paths — three touches, all bookkeeping, none in code:**

| phase | path | what it was |
|---|---|---|
| PHASE-01 | `docs/slices/003/plan-log.md` | PL-15, the user's acceptance of the plan |
| PHASE-01 | `docs/slices/003/slice-003.md` | stage `planned` → `executing`, one line |
| PHASE-02 | `docs/slices/003/plan-log.md` | PL-16, the orchestrator's two calls on EX-12's import bill and the pre-authorised backend script |

No phase declared its own log as a surface, and each of these is the log doing
its documented job. They are named here because PHASE-06/VA-5 concluded "no
undeclared path", which is not quite true, and because the audit's rule is to
list every one rather than to pre-filter for benignity. None is scope creep and
none touches code.

**Declared-but-untouched — four:**

| phase | path | reading |
|---|---|---|
| PHASE-04 | `crates/goad/tests/renderer/tree.rs` | the file exists; PHASE-04's diagnostic-surface assertions all landed in `wiring.rs`, so the second declared file was simply not needed |
| PHASE-05 | `crates/goad-shell/tests/integration/transport.rs` | neither imports a symbol that moved to `scripting.rs`, so the include-line edit the surface anticipated had nothing to do |
| PHASE-05 | `crates/goad-shell/tests/integration/host.rs` | as above |
| PHASE-06 | `docs/slices/003/canon-delta.md` | EX-2 required CD-1..CD-3 be *re-read* against what shipped, not rewritten; the audit re-read them and found them accurate |

`crates/goad/src/controller.rs` was declared a **repair surface** for PHASE-03
and was not touched — PHASE-03's own commit subject says "no production
repair", and the diff agrees. PHASE-02's one new file under `tests/backends/`
was pre-authorised in the phase's implementer notes.

The pre-authorised script aside, **no undeclared source, test, manifest or
markup path exists anywhere in the slice.**

### Canon-delta, checked against the documents it names

- **CD-1** — R-55 is the highest requirement id in SPEC-001; R-56 is free.
  The three kinds it fixes are the three the tree emits: `wire.rs`'s `kind()`
  returns exactly `"startup"`, `"requested"`, `"scheduled"`.
- **CD-2** — SPEC-001 §6.1's illustration does read `"source": "timer"`
  (line 244), and `Stimulus::event` does write `source: "host"` for every
  stimulus. The divergence is real and CD-2 states it correctly, including its
  deliberate decision to leave `"data": {}` alone.
- **CD-3** — §2's Boundaries paragraph reads as CD-3 quotes it, and its
  pointer is conditional on SPEC-002 landing, which is the right coupling.

### The wire invariants

`Stimulus` is outbound only: it names the `event.kind` the host emits, and is
never parsed. No normalization door is owed and none was cut. The renderer
gained one Slint property and one markup line; the new `kind` string is not the
renderer's business and nothing narrowed to match it. `crates/goad/ui/app.slint`
grew exactly two lines, and the empty-state sentinel that D-17 exists to protect
still renders, held by its own test.

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`
- **State:** open · **round count: pending** — a reviewer is working the ledger
  in parallel with this audit and had not reported when the audit was written.
- **Outstanding blockers:** *pending the code review.* The audit itself raises
  no blocker; see the Verdict.

## Verdict

**All twelve acceptance criteria are met, each on evidence the audit gathered
itself.** The slice does what it set out to do: the resolved instant now makes
the host act, in both directions, at a bounded rate, without losing the
schedule when the backend or the clock fails, and without a resolution the host
never told anyone about. Brief §21 AC-3 has its first observable, and AC-8 has
stopped being a property of a pure function.

**The floor is the load-bearing part, and it holds.** A backend that instructs
the past on every response is the case that turns a scheduler into a busy loop,
and `MINIMUM_SPACING` is the only thing between the host and that outcome. It
is a stratum 3 constant beside the loop that applies it, anchored to the
previous scheduled firing on a monotonic instant that nothing clears, and it
adjusts nothing the host stores or reports. The design's separation of *what is
stored* from *when the host fires* is what lets the floor coexist with
SPEC-001/R-28 rather than contradict it, and the code expresses that separation.

**The one number the slice flagged for the audit was wrong, in the safe
direction.** `notes.md` hands the auditor a 6.9x margin as the thing to attack.
Measured at the bound that actually governs, under five-fold CPU
oversubscription, it is 15.2x, and the worst margin anywhere in the suite is
10.8x. Ten further runs at that load were 13/13 green. AC-12's clause that a
load-sensitive test is a design defect is satisfied on evidence rather than on
absence of observed flakes. The recorded figure should be corrected so that a
future slice does not inherit a false sense of how tight this is.

**Slice 002's follow-up F-5 is discharged.** F-5 asked for one exchange driven
through the production runtime topology — Slint's executor polling a
`tokio::process` future under a tokio `EnterGuard` — which slice 002 had
instantiated but never driven. `event_loop_schedule` drives a scheduled
evaluation through it: the multi-thread runtime, the `EnterGuard` held for the
loop's life, a real window and tray, `install`'s callback table, a real channel
and `Cancel`, `SlintGlass`, `ProcessBackend` against a real child, the
production `serve`, and `slint::spawn_local`. The one substitution is the Slint
**platform**, which is the testing backend's because no headless test can
install the production one. That residue is AC-10's own stated limit and slice
002's F-8; it is not closable in this repository and the audit does not treat
it as outstanding.

**What is accepted knowingly.**

- **The suspend limitation.** A wait measured monotonically is not consumed by
  time spent suspended, so a check due during a suspend fires after the host
  has been awake for the remainder of the wait. Stated in draft SPEC-002 §5 and
  its OQ-1, decided at D-8, and correctly deferred rather than hidden.
- **The production Slint platform.** Structurally unproven by any test here.
- **The break-and-revert criteria** are one-shot experiments recorded in phase
  sheets, not standing tests. If the floor were deleted tomorrow, the standing
  suite would catch it through the anti-spin windows, but the *necessity*
  argument itself lives in prose.

**The audit raises no blocker.** It raises one code-hygiene finding for the
code reviewer (below), and a set of record corrections that need the user's
endorsement.

**For the code reviewer.** The slice introduced **eight `D-N` design-decision
citations and one new `F-N` review-finding citation into production source** —
in `crates/goad/src/controller.rs` and `crates/goad/src/diagnostics.rs`, and
more again in test files. `docs/memory/cite-requirements-not-finding-ids.md`
records a user decision, settled at slice 001's audit and *"in force from slice
002"*, that code comments cite a spec requirement, a spec section, an ADR or the
brief — never a slice-local `F-N` or `D-N` — and it says in terms: *"Do not
extend the practice."* Measured: production source under `crates/*/src` carried
**zero** `D-N` citations at `0b2e50f` and carries **eight** at `3777c22`. This
is a class, not an instance, and now that draft SPEC-002 exists most of these
comments have a requirement id available to cite instead. Handed to the ledger,
not fixed here.

## Reconciliation

Nothing in this table is applied. Each row needs the user's explicit
endorsement, and the audit does not write canon.

| document | change | reason | done |
|----------|--------|--------|------|
| `draft-spec.md` → `docs/specs/002-host-scheduling-behaviour.md` | promote | drafted during this slice as its working authority; §7's 22 named tests all exist in the tree | [ ] |
| `docs/specs/001-host-backend-protocol.md` §4 *Requests* | apply CD-1 — add R-56 fixing `event.source` at `"host"` and `event.kind` at exactly `"startup"` / `"requested"` / `"scheduled"` | the host now emits three kinds and a backend can only branch on strings the contract fixes; R-55 verified as the current highest id | [ ] |
| `docs/specs/001-host-backend-protocol.md` §7 | apply CD-1's second half — a verification row for R-56 | every requirement needs a §7 row | [ ] |
| `docs/specs/001-host-backend-protocol.md` §6.1 | apply CD-2 — the illustration's `"source": "timer"` becomes `"host"` | no host build has ever emitted `"timer"`; CD-1 makes the field normative, so the example must show what the host sends | [ ] |
| `docs/specs/001-host-backend-protocol.md` §2 *Boundaries* | apply CD-3 — one sentence pointing at SPEC-002 for the minimum spacing | without it, R-28 reads as forbidding a floor it does not govern. Conditional on SPEC-002 landing | [ ] |
| `CLAUDE.md` §Strata run one way | replace `cargo test --no-default-features` with `cargo test -p goad-semantics` as the named instrument | no workspace manifest declares `[features]`; the command passes the identical 303 tests and rejects nothing. POL-001 §Compliance already records that the feature column was retired at the crate split. `CLAUDE.md` is stale against its own canon | [ ] |
| `docs/roadmap.md` §003, §*v0.1.0 acceptance coverage* rows 3 and 8, §*Open decisions*, §*Where this stands* | mark 003 closed; coverage rows 3 and 8 discharged; close the "whether 003 persists schedule state" decision; refresh the standing paragraph | the roadmap is stale at 2026-09-04 and still describes the gate as "green in both feature columns" | [ ] |
| `docs/memory/` | lift the durable facts from `notes.md` Harvest | Close step; the load-margin correction in particular is a fact a future slice would otherwise rediscover | [ ] |

**Edits promotion needs, beyond moving the file.** The draft matches
`docs/templates/spec.md` section for section (1-9), and its cross-references are
already qualified as `SPEC-001/R-N`, which is the house form. What must change:

1. **Title** — `# SPEC-NNN (draft): The host's scheduling behaviour` becomes
   `# SPEC-002: The host's scheduling behaviour`.
2. **Status** — the four-line draft disclaimer ("not canon… nothing outside
   `docs/slices/003/` may cite it") becomes `**Status:** active`.
3. **The template's standing comment** — SPEC-001 carries the evergreen /
   immutable-ids HTML comment under its front matter; the draft omits it.
   Add it, in SPEC-001's wording, with the citation form `SPEC-002/R-N`.
4. **Filename** — `002-host-scheduling-behaviour.md`, matching
   `001-host-backend-protocol.md`'s habit of dropping the leading article
   rather than transliterating the title.
5. **Requirement ids** stay R-1..R-11. They are spec-local and immutable from
   promotion; nothing renumbers.
6. **Ordering against CD-1.** The draft's §2, §6 and §9 all cite SPEC-001/R-56,
   which does not exist until CD-1 is applied. CD-1 must land with or before
   the promotion, or the new spec ships with a dangling reference.
7. **One citation must move.** R-4's *verified by* row cites `notes.md`
   PHASE-03's VA-3 for the floor's break-and-revert. `notes.md` is disposable
   by `docs/AGENTS.md`'s own table, and canon may not depend on a file the
   Close step empties. Either lift that experiment into `docs/memory/` and cite
   it there, or state the row as *review, not a test* the way R-11 already is.
   Citing `design.md` and `review-design.md` is fine — SPEC-001 §9 does it.
8. **The margin correction** — if the corrected load figures go anywhere in
   canon, they belong in `docs/memory/`, not in the spec.

**Is an ADR warranted?** Two decisions could be reversed by accident and are
currently held only in a spec requirement or a design log:

- **The floor's anchor.** Draft R-4 says the spacing is measured from the
  previous *scheduled firing*, on a monotonic instant that nothing clears —
  never from "the last thing the host did". D-3 records why, and
  `review-design.md` F-2 records that the justification (every other stimulus
  is a person) **expires in slice 004**, which adds a non-human stimulus. A
  requirement states the rule; it does not state that the rule's premise is
  about to move. **Recommend an ADR.** This is exactly the shape ADRs exist
  for: a choice whose rationale a later slice will be tempted to reverse
  without noticing.
- **"Nothing persists."** OQ-1's answer, taken by the user, keeps SPEC-001's
  own OQ-3 shut and is the sole reason draft R-11 is reviewable rather than
  testable. It is currently recorded in `design-log.md` and implied by R-11.
  **Recommend an ADR** only if the user wants it defended at that weight;
  otherwise `docs/roadmap.md` §Open decisions closing the question explicitly
  is enough. The audit's recommendation is the lighter option — R-11 already
  states the consequence normatively, which the anchor decision does not.

**Design drift not reconciled.** `design.md` is a record of intent and was not
retro-fitted. Four departures stand:

1. **§9's uniform "~105 ms / 19x" row** for the four `default_poll = 100 ms`
   liveness assertions. PHASE-06 measured 250-290 ms and 6.9x-8x and recorded
   the gap. The audit's own measurement at the governing bound gives 130-176 ms
   and 11x-15x, under heavy load. So the design's expectation is closer to
   right than PHASE-06's correction of it, and **both recorded numbers are
   wrong in different directions**. The design stands; `notes.md`'s margin
   table is what should be corrected at close.
2. **§9's AC-9 row** predicts one liveness figure for "refusal after the
   succeed-once clock fails". The test that discharges it does not produce a
   comparable number: it measures the startup exchange's liveness and verifies
   the refusal through a 500 ms anti-spin window plus a clock-read counter. The
   row and the test do not correspond one to one. The test's shape is the
   better one — a spin is not a liveness failure — and the design stands as the
   record that this was not foreseen.
3. **AC-7's design row** predicts ~2000x; measured is ~20 000x, in the safe
   direction. Recorded because §9's other numbers are tight.
4. **PHASE-04's `tree.rs` surface** was declared and not needed; the diagnostic
   assertions landed in `wiring.rs`. Not a drift in behaviour, only in the
   plan's guess about where a test would go.

**The stray `(F-1)` citation at `crates/goad-semantics/src/schedule.rs:327`.**
Disposition: **leave it, and treat the class as the finding.** It is slice
001-era residue on a test doc comment, and
`docs/memory/cite-requirements-not-finding-ids.md` records the user's decision
to tolerate slice 001's existing `F-N` citations because they grep to that
slice's own ledgers and a sweep would trade a pointer for a paraphrase. Eighty-
four such citations predate this slice; removing one is cosmetic. What matters
is the same memory's next sentence — *"Do not extend the practice"* — which this
slice did, eight times in production source. That is the code reviewer's, and
it is recorded in the Verdict above rather than dressed up as a fix to line 327.

## Closure

- [ ] All findings dispositioned; no blockers outstanding — *pending the code review*
- [x] All acceptance criteria met, or explicitly waived by the user — twelve of twelve, evidenced above
- [x] Tests and checks green — `just check` exits 0 idle and under loadavg 164; 303 tests, 0 ignored
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended — the Reconciliation table is proposed, not applied
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

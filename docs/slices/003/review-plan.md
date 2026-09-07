# Review — plan — Slice 003

**Subject:** plan — `docs/slices/003/plan.md` (934 lines, six phases), read
against `design.md`, `slice-003.md`, `draft-spec.md`, `canon-delta.md`, the
canon it cites, and the tree it declares as surfaces
**Reviewer:** fresh agent (Opus 5), no authorship of any slice 003 artefact
**Opened:** 2026-09-07
**State:** resolved — 13 findings, all `verified`, none withdrawn, 0 outstanding at any severity

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

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-09-07 — the plan as an instruction to six Sonnet-class
executors, one phase per session including bookkeeping.

Written after reading `CLAUDE.md`, `docs/AGENTS.md`, `slice-003.md`,
`design.md`, `draft-spec.md`, `canon-delta.md`, `design-log.md` (the standing
grant and D-17) and `review-design.md`'s three syntheses — and **before**
opening `plan.md`. Three rounds of design review have already run, all twenty
findings verified, so the design's *shape* is not this review's subject. The
plan is.

**What a plan is answerable for here.** `docs/AGENTS.md` §Plan gives four
obligations, and each is an attack surface:

1. *"verify [the design's assumptions and approach] against the code"* — every
   `path:line` the plan load-bears must still say what the plan says it says.
   The design was caught twice by its own reviewers stating a checked-sounding
   fact about the tree without running the check (F-3, F-17). A plan built on
   that design is the third chance to repeat it.
2. *"entry / exit criteria … such that if they are completed, the intent of the
   slice and the design will be observed"* — a criterion an executor cannot
   decide mechanically is not a criterion, it is an opinion, and it will be
   decided in the executor's favour at 2 a.m. of a long session.
3. *"each phase is reasonable for a single agent to complete within a session,
   including bookkeeping"* — a Sonnet-class executor, ~200k tokens, one phase.
4. *"If any unresolved design issues emerge, go back to the appropriate stage of
   design"* — and §Phase plan repeats it: *"go back to plan (or design) rather
   than quietly repairing it"*. FD-1 was taken back to design and produced D-17.
   Whether FD-2..FD-n got that same treatment, or a stated reason not to, is a
   process question with a documented right answer.

**Where the bodies are likely buried.**

- **PHASE-02.** The `select!` restructure is the slice. If sizing was done by
  counting acceptance criteria rather than by counting the re-reading a fresh
  session must pay for, this is where it shows. A phase that must hold the loop
  sketch, `wait_for`, the floor rule, `Absorbed`, `Pending::now`, and six timed
  tests in one head is the candidate for a split — and the split has its own
  cost, because phase two's second half re-reads what its first half wrote.
- **Declared surfaces as a loophole.** Surfaces are the only thing standing
  between an executor and scope creep (§Execute: *"Stay inside the phase's
  declared surfaces"*). Two failures are possible and opposite: a surface
  declared too wide lets a later phase grow the loop it was not asked to grow,
  and a surface not declared at all forces an executor to either stop or
  trespass. `controller.rs` declared as repair-only in a later phase is the
  exact shape of the first.
- **The union of surfaces against Scope.** `slice-003.md` §Scope is explicit to
  the file, including two files declared **unchanged** (`reception.rs`,
  `host.rs`, `main.rs`) and one declared bounded (`app.slint`, D-17's property
  and one markup line, *"a second markup change is a STOP"*). A phase surface
  that quietly admits one of those is a scope change wearing a plan's clothes.
- **Coverage as mention.** Twelve acceptance criteria and twelve draft SPEC-002
  requirements. A coverage table that names AC-n beside a phase is not
  discharge; the cited criterion has to actually decide the AC. AC-4, AC-6,
  AC-9 and AC-10 were each made *harder* during design review, so a plan
  written from the earlier reading would under-discharge exactly those four.
- **Invented instruments.** PL-6's divergence threshold is the planner's, not
  the design's and not canon's. An instrument with no consequence on breach is
  decoration; one with a consequence nobody is told to act on is worse, because
  it looks like control.
- **The gate's known traps.** `docs/memory/` exists precisely because executors
  walk into them. Any phase whose work touches cargo's test cwd, a `#[path]`
  shared helper, clippy's test carve-outs, or the `expect_dead_code` cfg-attr
  and does not say so is a phase that will burn a session discovering it.
- **Cohesion by sizing.** A phase pairing a stratum 1 change with a stratum 3
  one because neither fills a session is defensible; a phase doing it while
  claiming a theme is not. ADR-001's strata run one way, and a phase that
  straddles them must say which direction its dependency runs.

**The invariants the plan is held to.** `CLAUDE.md`'s five: no domain
vocabulary; permissive wire, canonical internals; no narrowing of wire
compatibility to suit the renderer; a backend failure never takes the host down;
strata run one way and `cargo test --no-default-features` is the compiler
enforcing it. Plus POL-001's six-command gate, unweakened — no `#[ignore]`, no
conditioning, no lint suppression to make a phase green — and `docs/AGENTS.md`'s
own file-placement rule, that current truth, conversation, findings and work
each have exactly one home.

**Not this review's subject:** the design's shape, the number 3 seconds, and
whether the slice is worth doing. Those are settled. Disposition of every
finding below belongs to the responder, not to me.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | `fix-now` | `verified` |
| F-2 | major | `fix-now` | `verified` |
| F-3 | major | `fix-now` | `verified` |
| F-4 | major | `fix-now` | `verified` |
| F-5 | major | `fix-now` | `verified` |
| F-6 | major | `fix-now` | `verified` |
| F-7 | minor | `fix-now` | `verified` |
| F-8 | minor | `fix-now` | `verified` |
| F-9 | minor | `fix-now` | `verified` |
| F-10 | minor | `fix-now` | `verified` |
| F-11 | nit | `fix-now` | `verified` |
| F-12 | nit | `fix-now` | `verified` |
| F-13 | major | `fix-now` | |

**Outcomes are the raiser's and are set in round 2.** Every finding is
dispositioned `fix-now` and every repair is integrated; none was deferred,
downgraded or tolerated. F-11's first row is the one place the responder
disagrees on measurement, and it is argued there rather than re-severitied.

### F-1 — PHASE-02 cannot compile inside its own declared surfaces: `absorb`'s new return type breaks 22 call sites in two files it may not touch

**Severity:** blocker
**Location:** `plan.md` PHASE-02 *Surfaces* / *Must not touch* / EX-3

**Expected:** `docs/AGENTS.md` §Execute — *"Stay inside the phase's declared
surfaces. Touching anything else is either a design change or scope creep — in
both cases, stop and ask."* A phase's declared surfaces must therefore contain
every path the phase must edit to end green, since its exit is `just check`
exiting 0.

**Observed:** PHASE-02/EX-3 requires `absorb` to return `Absorbed { shift,
next_check }` and says *"Every existing caller of `absorb` is updated to read
`.shift`."* PHASE-02's surfaces are `crates/goad/src/controller.rs`,
`crates/goad/tests/renderer/{main.rs, scheduling.rs}` and
`docs/slices/003/notes.md`. Its *Must not touch* list names
`crates/goad/tests/renderer/wiring.rs` explicitly; `table.rs` is neither
declared nor forbidden, which under the same rule means it may not be touched.

**Evidence:** measured at HEAD 572049f. `absorb` has 45 call sites; **22 read
the return value** and stop compiling the moment its type changes from `Shift`
to `Absorbed`:

- `wiring.rs` — nine `let shift = controller.absorb(…)` (`:290`, `:323`,
  `:508`, `:526`, `:552`, `:570`, `:599`, `:631`, `:671`) and four
  `assert_eq!(…absorb(…), Shift::…)` (`:829`, `:853`, `:879`, `:900`).
- `table.rs` — two `let shift = …` (`:671`, `:720`) and seven `assert_eq!`
  (`:791`, `:800`, `:808`, `:822`, `:836`, `:848`, `:865`).

The production call site (`controller.rs:384`) discards the value, so it is not
the constraint; the tests are. PHASE-04 is the first phase that may open
`wiring.rs`, and no phase ever declares `table.rs` for anything but PHASE-05's
*"include and import lines only"*.

PHASE-02/VT-1 compounds it: it requires six named `wiring.rs` tests to pass
**unmodified**, in a phase that must edit that same file for a signature change.

**Disposition:** `fix-now`

**Response:**

Counted at HEAD with the compiler's eye, not the design's: 42 `absorb`
calls in the tree, of which **22 read the return value** —
`wiring.rs` nine `let shift = …` (`:290`, `:323`, `:508`, `:526`, `:552`,
`:570`, `:599`, `:631`, `:671`) and four inside an `assert_eq!` (`:830`,
`:853`, `:880`, `:901`); `table.rs` two `let shift = …` (`:671`, `:720`) and
seven inside an `assert_eq!` (`:792`, `:801`, `:809`, `:823`, `:837`, `:849`,
`:866`). The finding's count is exactly right; its line numbers for the
`assert_eq!` sites name the macro's opening line rather than the call, which
changes nothing.

**Chosen: declare the two files as PHASE-02 surfaces, bounded to the
migration.** Rejected the alternative the brief offered — a new fn beside the
old, with the old removed after migration — on two grounds. `CLAUDE.md` forbids
a parallel implementation, and the alternative does not avoid a single edit:
removing the old `absorb` in the same phase still requires all 22 call sites to
move, so it buys a second name and no reduction in work. The migration itself is
uniform: `.shift` appended at 22 sites, no import added, because `Shift` is
already in scope wherever it is asserted.

VT-1's six named tests all sit at or after `wiring.rs:923`, past the last changed
call site (`:901`), so *unmodified* is preservable as a claim about those six
function bodies. VT-1 is restated to say that, and PHASE-02 gains a `git diff`
instrument borrowed from PHASE-05/VA-2 to hold it. PHASE-02's *Must not touch*
now names `wire.rs`, `main.rs`, `tree.rs`, `mapper.rs`, `tray.rs`,
`reception.rs` and `startup.rs` explicitly rather than relying on omission.

Sizing re-checked and **not split**: 22 uniform one-token edits plus a
seven-item fixture move is cheap in a session's budget, and the reviewer's own
calibration against slice 002's PHASE-10 stands.

**Outcome:** `verified`

### F-2 — PHASE-02's new test module needs seven fixtures that are private to the file PHASE-02 may not touch, and its own notes instruct the edit its surfaces forbid

**Severity:** major
**Location:** `plan.md` PHASE-02 *Must not touch* vs its *Notes for the
implementer*; `plan-log.md` PL-2 *Consequence*

**Expected:** the surfaces and the implementer notes of one phase say the same
thing. `CLAUDE.md` also forbids parallel implementations, so "copy the fixtures
into `scheduling.rs`" is not the escape.

**Observed:** PL-2's Consequence reads *"PHASE-02 lifts `until` to a
`super`-visible helper rather than copying it, **and updates `wiring.rs`'s uses
in the same change**"*, and PHASE-02's notes repeat it. PHASE-02's *Must not
touch* names `crates/goad/tests/renderer/wiring.rs`.

**Evidence:** every fixture a `serve`-driven scheduling test needs is a private
item of the `wiring` module, measured at HEAD 572049f:
`TIMEOUT` (`wiring.rs:28`), `now` (`:30`), `stub_clock` (`:45`),
`window_and_tray` (`:65`), `glass_over` (`:73`), `current_view_token` (`:123`,
named by PHASE-02/VT-5) and `until` (`:135`). The existing `mod serving`
imports six of them from `super` (`wiring.rs:1064-1067`). None is reachable
from a sibling module. PL-2 saw the problem for `until` and for *"the
window/tray fixtures"* but the surface list was not updated to match, and the
other five are never named at all.

**Disposition:** `fix-now`

**Response:**

Confirmed: all seven are private items of the `wiring` module —
`TIMEOUT` (`wiring.rs:28`), `now` (`:30`), `stub_clock` (`:45`),
`window_and_tray` (`:65`), `glass_over` (`:73`), `current_view_token` (`:123`)
and `until` (`:135`) — and a sibling module reaches none of them.

**Where they live: `crates/goad/tests/renderer/harness.rs`, a new module of the
renderer target**, declared `#[cfg(test)] mod harness;` in `renderer/main.rs`.
This is not a new pattern: `crates/goad-shell/tests/integration/harness.rs`
already *is* this, and its own header states the rule — *"Anything two of the
three case files need lives here; anything one of them needs stays there. What
both tiers need lives in `tests/support/driving.rs`"*. Exactly seven fixtures
now have two consumers in the renderer target, so exactly seven move; nothing
else does. `tests/support/` is the wrong home — three of the seven name Slint
types the `integration` target cannot see, and every symbol in a
`#[path]`-shared file must be reachable from every includer (FD-3's own rule).

The migration is small and mechanical. `wiring.rs` loses seven definitions and
gains one `use crate::harness::{…}` line; its nine child modules are
**untouched**, because a child's `super::X` already resolves through a private
parent import — the tree proves it today, `mod body_content`
(`wiring.rs:706`) reaching `host`, `quiet_event` and `scripted` that way. Three
files change plus one new one.

PL-2's Consequence is superseded by PL-12, which names the destination the
surface lists now match.

**Outcome:** `verified`

### F-3 — FD-3's remedy widens the slice's declared Scope by a whole test target and was decided inside the plan, while FD-1's one-file widening was escalated; and the cheapest alternative was never evaluated

**Severity:** major
**Location:** `plan.md` FD-3, PHASE-05 *Surfaces*; `plan-log.md` PL-5, PL-8

**Expected:** two things. First, PL-8's own stated reason for escalating FD-1
rather than deciding it: *"Either answer also widens the slice's declared scope
by one file."* The same test applied to FD-3. Second, `CLAUDE.md` — *"DRY —
find out whether existing code can be adapted"* and *"write less code"* — which
is an argument for the cheapest correct answer, not for the tidiest one.

**Observed:** FD-3 widens the slice by **eight** files that
`slice-003.md` §Scope does not name, and PL-5 decided it under the grant.
PHASE-05's surfaces include `crates/goad-shell/tests/integration/{main.rs,
harness.rs, round_trip.rs, failure_matrix.rs, transport.rs, host.rs}` — an
entire test target of another crate — plus the new `tests/support/scripting.rs`.
`slice-003.md` §Scope names only *"`tests/support/driving.rs` and
`tests/backends/` — helpers and scripted backends extended, not duplicated"*.

Separately, PL-5's rejected alternatives are *(a)* restating the helpers,
*(b)* `#[expect(dead_code)]` **per symbol inside the shared file**, and *(c)* a
second `#[test]` in `tests/event_loop/`. The obvious fourth — `#[allow(dead_code)]`
on the new target's own `mod` declaration — is not among them, and PL-5's stated
objection to (b) does not touch it: `allow` never raises
`unfulfilled_lint_expectations`, and an attribute on the includer is invisible
to the two targets that do use the symbols, so their `dead_code` coverage is
unchanged.

**Evidence:** the six symbols PL-5 moves are all transitively reachable from
`scripted`, so the split does work — `scripted` → `logging_backend`
(`driving.rs:139`) → `backend` (`:38`) and `marker` (`:50`) → `clear` (`:59`);
`invocations` (`:148`) is called directly. That is not in dispute. What is in
dispute is its price: the move edits `#[path]` and `use` lines in ten files
across two existing targets and relocates a re-export at
`crates/goad-shell/tests/integration/harness.rs:29`, against one attribute on
one line. `dead_code` is `warn` in `Cargo.toml:103` and the gate's `-D warnings`
promotes it, exactly as FD-3 states — which is what makes a target-local `allow`
sufficient.

The counter-argument that deserves a written answer, not silence: POL-001
forbids suppressing a lint to make a phase green. Whether declaring "this
includer uses a subset" is that, or is the honest statement of a fact, is the
decision PL-5 should have taken on the page.

**Disposition:** `fix-now`

**Response:**

Accepted in both limbs, and they resolve in opposite directions.

**The cheap alternative, evaluated on evidence.** It is refused by canon, not by
taste. POL-001 §Compliance authorises *"a **site-local** `#[expect(lint, reason
= …)]` at the narrowest scope that works … **never `allow`**, which is silent
when it stops being true"*. `#[allow(dead_code)]` on the `#[path]` module
declaration is the exact shape that sentence names and forbids, and
`docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` says the same
in its own words: *"treat any other spelling (a plain `#[allow]`, a bare
`#[expect]`) as a defect."* Substituting `#[expect]` does not rescue it: a
module-wide expectation over a hand-written shared helper is not *site-local*,
and POL-001's only module-scoped carve-out is the generated-code quarantine.
Against that, `docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`
records the positive rule the split obeys — *"When a later change makes a shared
symbol unused by one includer, move it out immediately … this re-settlement is
expected maintenance, not a sign the split was wrong."* **PL-5's decision
stands.** PL-10 records the fourth alternative and this reasoning, which is the
argument the finding correctly says PL-5 owed.

**The escalation asymmetry is a real defect and is repaired.** FD-3's remedy
widens the slice by a test target of another crate, and PL-8 escalated FD-1 for
strictly less. The split is therefore recorded the way FD-1 was: **D-18** in
`design-log.md` and `design.md` §7, and a new Scope entry in `slice-003.md`
naming `tests/support/scripting.rs` and the include-and-import lines of both
existing targets. Traced the closure myself: `scripted` → `logging_backend`
(`driving.rs:139`) → `backend` (`:38`) and `marker` (`:50`) → `clear` (`:59`),
with `invocations` (`:148`) called directly — six symbols, all live in the new
target, thirteen left in `driving.rs`.

**Outcome:** `verified`

### F-4 — the Coverage table claims PHASE-03/VT-1 discharges both of AC-4's successor cases; VT-1 describes only one, and the other is asserted nowhere

**Severity:** major
**Location:** `plan.md` §Coverage, AC-4 row; PHASE-03/VT-1

**Expected:** `docs/AGENTS.md` §Plan — entry and exit criteria such that *"if
they are completed, the intent of the slice and the design will be observed"*.
`slice-003.md` AC-4 states the intent: *"What follows depends on the backend,
and **both cases are asserted**: if it stops instructing the past, the next
resolution consumes the elapsed value and cadence resumes; if it instructs the
past on **every** response … cadence never resumes."* That sentence is F-1 of
`review-design.md`, the finding three rounds of design review spent the most
effort on.

**Observed:** the Coverage table reads *"AC-4 … PHASE-03/VT-1 (the loop, **both
successor cases**)"*. PHASE-03/VT-1 specifies one backend: *"a backend that
instructs an **absolute past instant** on every response … Assert: exactly two
invocations."* That is the second case only. Nothing in PHASE-01, PHASE-02 or
PHASE-03 drives a backend that instructs the past **once** and then stops, which
is the first case and the one whose behaviour AC-4 cites `schedule.rs:196-201`
for.

**Evidence:** `plan.md` PHASE-03/VT-1 in full; `slice-003.md` AC-4;
`design.md` §9's AC-4 row, which has the same single-backend discharge — so the
gap originates in the design and the plan inherited it rather than catching it.
`docs/AGENTS.md` §Plan requires the plan to *"examine your assumptions and the
approach laid out in the design, then verify them against the code"*, and §Phase
plan requires an unresolved design issue to *"go back to plan (or design) rather
than quietly repairing it"*. This one was neither caught nor repaired: the
Coverage table asserts the discharge exists.

Either AC-4's first case gets a criterion, or AC-4 is amended to say the case is
`resolve`'s and is held by stratum 1 — which is exactly the argument AC-5 already
makes for its own analogous half (*"already held by stratum 2's own tests, and
observing it through the timer costs a whole floor interval of gate time for no
new evidence"*). Both are defensible. Claiming coverage that is not there is not.

**Disposition:** `fix-now`

**Response:**

The Coverage claim is false as written. AC-4's first successor case — a past
instruction *once*, then none — is driven by no phase.

**Repaired in two places, at no wall-clock cost.** The arithmetic half is
already held at stratum 1 by an existing test,
`schedule.rs::an_elapsed_retained_check_is_consumed_and_the_default_poll_applies`
(`:311`), which is what AC-4's own `schedule.rs:196-201` citation points at.
What was missing is the loop-level evidence, and it is cheap: **PHASE-03/VT-5**
drives a backend that instructs an absolute past instant on the first response
and no `next_check` on the second, with `default_poll` at 100 ms, and asserts
that after the loop stops the retained `Frame::next_check` is
`2026-01-01T00:00:00.100Z` — `now + default_poll`, by value against the fixed
`stub_clock` — rather than the past instant. That is *the elapsed value was
consumed and cadence resumed*, observed through the loop, and it distinguishes
this case from VT-1's, where the retained value stays the past instant because
`resolve`'s first arm replaces it verbatim.

Deliberately **not** asserted: the third invocation actually landing. The floor
pushes it three seconds out, which is the whole floor interval of gate time
AC-5's own revision refused to pay for no new evidence.

The Coverage row is rewritten to name all three sources honestly. The gap
originates in `design.md` §9's AC-4 row, which carries the same single-backend
discharge; the design is a record of intent and is **not** retro-fitted, so it
is recorded as **FD-5** in the plan's *Findings against the design*.

**Outcome:** `verified`

### F-5 — no phase criterion discharges draft SPEC-002 R-4's second half or R-5's, and PHASE-06/EX-1 is the first place the gap becomes visible

**Severity:** major
**Location:** `plan.md` §Coverage; PHASE-06/EX-1

**Expected:** `docs/AGENTS.md` — a draft spec *"is the slice's working
authority: design, plan and execution cite it exactly as they would the real
thing"*, and `plan.md`'s own point 2 repeats it. `draft-spec.md` §7 states a
*verified by* row for each of R-1..R-11.

**Observed:** the Coverage table maps AC-1..AC-12 to phases and stops. No table
maps R-1..R-11, and `design.md` §9 has none either. Walking the two that are not
implied by an AC:

- **R-4's second half** — §7: *"a person acting in the middle of a scheduled
  cadence does not raise that count"*. PHASE-03/VT-1 has no person acting;
  PHASE-02/VT-6 has a person-driven `evaluate` but asserts liveness, not an
  invocation count against the floor. Nothing asserts it.
- **R-5** — §7: *"the first scheduled evaluation of the process honours a
  default poll shorter than the spacing; an evaluation a person asks for is
  dispatched without waiting for it"*. The first half falls out of PHASE-02/VT-3
  incidentally (`default_poll` 100 ms, floor 3 s); the second is asserted
  nowhere as a criterion.

**Evidence:** `draft-spec.md:80-81` (R-4, R-5) and `:175-176` (their §7 rows);
`plan.md` §Coverage in full. PHASE-06/EX-1 — *"every requirement R-1..R-11's
*verified by* row names a test that exists"* — is where this surfaces, five
phases after PHASE-03, the only phase whose fixtures could cheaply have carried
it. That is precisely the failure PL-6 names for margins (*"by then a bad margin
is five phases old and the phase that could have fixed it is closed"*) and does
not apply to the draft spec.

R-4's second half is also the property `design.md` §5.4 argues hardest for — *"a
person acting does **not** clear the floor"* — and the one `review-design.md`
round 2 says it attacked and could not break. It is the slice's central claim
and it has no test.

**Disposition:** `fix-now`

**Response:**

Both gaps confirmed, and the requirement table that would have caught them did
not exist.

**A second Coverage table now maps R-1..R-11** to the criterion that discharges
each, beside the AC table. PHASE-06/EX-1 stops being the first place a hole
becomes visible.

**R-4's second half gets PHASE-03/VT-6**, in the phase that owns the floor: the
past-instant-every-response backend, `default_poll` far. Startup is invocation 1;
the unfloored scheduled firing is invocation 2 and sets `floor_until` three
seconds out; the driving task then sends `Command::Evaluate(Stimulus::Requested)`
— a person acting mid-cadence — and invocation 3 lands at once. The assertion is
that the count is **exactly 3 and holds across a 500 ms window**. A floor that a
person's action cleared or reset would fire invocation 4 immediately and fail it.
This is `design.md` §5.4's *"a person acting does not clear the floor"*, which
the review is right to call the slice's central claim.

**R-5's second half falls out of the same run** and is asserted in it: invocation
3 arrives inside `until(2 s)` of the command being sent, so a person's evaluation
was not delayed by the floor. R-5's first half is PHASE-02/VT-3, which now names
R-5 rather than discharging it incidentally.

**Outcome:** `verified`

### F-6 — PL-6's divergence threshold admits what AC-12 forbids: a below-margin liveness test is recorded as a follow-up and ships

**Severity:** major
**Location:** `plan-log.md` PL-6 *Consequence*; `plan.md` PHASE-06/EX-5

**Expected:** `slice-003.md` AC-12 — *"no test whose passing depends on machine
load. … A timing-sensitive test that can be flaky under a loaded gate is a
design defect, not a tolerated cost."* `design.md` §8 R1 says the same, and
`review-plan.md`'s Protocol says *"Do not reach for `follow-up` because the fix
is large."*

**Observed:** PHASE-06/EX-5 states the consequence of a breach: *"the divergence
is written up under this plan's *Findings against the design* and carried into
`slice-003.md`'s Follow-ups."* PL-6 adds *"Anything else is recorded and passed
over."* So a liveness assertion measured at, say, 4x margin — against design §9's
estimated 19x — is documented and shipped, inside a slice whose own acceptance
criterion calls that a design defect. Nothing stops the phase, nothing reopens
the design, and PHASE-06 is the phase after which no phase can act.

The instrument is otherwise well chosen: measuring at the phase that lands the
test is right (`docs/AGENTS.md:137` forbids retro-fitting `design.md`, and PL-6
honours it), the 5x and 3-second numbers are defensible as a tripwire, and VA-4's
three consecutive green runs is real flakiness evidence. What is missing is that
a breach must be a STOP, not a note. PHASE-02/S-6 already has the right shape
for a different trigger (*"a timed assertion is flaky across three consecutive
runs … record the measurement and stop"*); EX-5 has no counterpart.

**Evidence:** `plan.md` PHASE-06/EX-5 and PHASE-06/VA-4; `plan-log.md` PL-6;
`slice-003.md` AC-12; `docs/policy/001-the-phase-gate.md` §Compliance.

**Disposition:** `fix-now`

**Response:**

Accepted without qualification. A margin measured below its threshold is AC-12's
*"design defect, not a tolerated cost"*, and recording it as a follow-up in the
last phase that can act is how the defect ships.

**PL-6 is superseded by PL-11.** The measurement discipline survives unchanged —
the phase that lands a test measures it, PHASE-06 collects, `design.md` is not
retro-fitted. What changes is the consequence of a breach: **PHASE-06/S-19**,
a STOP. The executor records the measurement, does not repair the margin on its
own authority, and consults the orchestrator; the slice does not close on a
breached margin. The threshold numbers themselves (5x liveness, 3 s of gate wall
time) are kept — the finding says they are defensible and offers no better ones.

The same STOP shape is given to PHASE-02 and PHASE-03, which are the phases that
actually take the measurements: a breach there is a STOP at the phase that could
still fix it, not a note carried five phases forward. PHASE-02/S-6's flakiness
trigger stays as it is.

**Outcome:** `verified`

### F-7 — PHASE-03/EX-3 asserts two things that cannot both be true

**Severity:** minor
**Location:** `plan.md` PHASE-03/EX-3

**Expected:** an exit criterion an executor can decide mechanically.

**Observed:** *"EX-3 — no production behaviour changed. If PHASE-02's mechanism
needed a repair, the diff to `controller.rs` is quoted in the phase sheet and
named as a finding."* A repair to the mechanism **is** a change in production
behaviour; that is what makes it a repair. As written the criterion is
discharged by either branch and so constrains neither.

**Evidence:** PHASE-03 *Surfaces* declares `controller.rs` a repair surface —
*"may fix a defect its own tests expose in PHASE-02's mechanism, and may not
extend it"*. The guard is otherwise sound: the diff is quoted, it is named as a
finding, S-9 stops a shape change, and PHASE-06/VA-5 diffs touched paths against
declared surfaces. Only EX-3's first clause is wrong. Suggested reading: *no
production file outside `controller.rs` is touched, and any `controller.rs` diff
is quoted and named as a finding.*

**Disposition:** `fix-now`

**Response:**

Correct: a repair to the mechanism is a production behaviour change, so the
criterion is discharged by either branch and constrains nothing.

The reviewer's suggested reading is adopted almost verbatim. EX-3 now reads: **no
production file outside `crates/goad/src/controller.rs` is touched, and any
`controller.rs` diff is quoted in the phase sheet and named as a finding.** Both
halves are mechanical — the first is a path check the audit's own surface diff
repeats, the second is a document check. S-9's stop on a shape change is
untouched and remains the guard that distinguishes a repair from an extension.

**Outcome:** `verified`

### F-8 — PHASE-03/EN-2 is not mechanically decidable

**Severity:** minor
**Location:** `plan.md` PHASE-03/EN-2

**Expected:** `docs/AGENTS.md` §Phase plan — *"Verify the phase's entry criteria
are actually met before starting."* That requires a criterion an agent can
check, not estimate.

**Observed:** *"PHASE-02/EX-7's second re-arm site … exists in the tree and is
**still undriven by any test**."* The first half is grep. The second is a
coverage claim over seven new tests and roughly forty existing ones, with no
coverage tooling in the gate. An executor will read the branch, believe it, and
tick the box.

**Evidence:** `justfile:19` — the gate is build, test, test-stratum1, typecheck,
lint, fmt-check. No coverage instrument exists. The criterion's purpose is
served more cheaply by its own last clause (*"if the branch is absent, PHASE-02
is not done"*), which is decidable; the undriven half could be dropped or
re-stated as "PHASE-02's sheet records the branch as written and not yet
driven", which is a document check.

**Disposition:** `fix-now`

**Response:**

Correct. There is no coverage instrument in the gate (`justfile:19` — six
commands, none of them a coverage run), so *"still undriven by any test"* is an
unfalsifiable claim an executor will tick.

EN-2 is split into the two halves the finding identifies. The grep half stays and
is decidable: the second re-arm site exists in `controller.rs`, and if it is
absent PHASE-02 is not done. The coverage half becomes a **document** check —
PHASE-02's phase sheet records the branch as written, reviewed and not yet
driven, which is what PHASE-02's exit already obliges it to say. Nothing is lost:
PHASE-03/VT-3 is what actually closes the branch, and it is a test.

**Outcome:** `verified`

### F-9 — PHASE-03/VT-3 places its clock fixture in a file PHASE-03 does not declare

**Severity:** minor
**Location:** `plan.md` PHASE-03/VT-3

**Expected:** a phase's tests live in that phase's surfaces.

**Observed:** VT-3 says the succeed-once clock is *"a top-level `fn` over a
`static AtomicUsize` **beside `wiring.rs:44`'s existing `stub_clock`**"*.
PHASE-03's surfaces are `renderer/scheduling.rs`, `config.rs` and `notes.md`,
with `controller.rs` as a repair surface. `wiring.rs` is not among them.

**Evidence:** `stub_clock` is at `crates/goad/tests/renderer/wiring.rs:45`,
inside the `wiring` module. If "beside" means *in the manner of*, the sentence
should say so; if it means *in that file*, it is F-2 again in another phase. The
same ambiguity sits in PHASE-03's own note about `Clock` being a `fn` pointer,
which is correct (`crates/goad/src/clock.rs`) and does not settle where the
fixture lives.

**Disposition:** `fix-now`

**Response:**

Ambiguous as written, and under F-2's repair the ambiguity now has a third
possible reading, so it is settled explicitly.

The succeed-once clock is a fixture of **one test**, so it does not go to
`harness.rs` — the renderer harness carries what two or more modules need, which
is the rule `crates/goad-shell/tests/integration/harness.rs:5-10` states. VT-3's
fixture is a top-level `fn` in
`crates/goad/tests/renderer/scheduling.rs`, written **in the manner of**
`harness::stub_clock` — a plain `fn` over a private `static AtomicUsize`, because
`Clock` is a `fn` pointer (`clock.rs:16`) and cannot capture. The plan now says
*in the manner of* rather than *beside*, and PHASE-03's surfaces are unchanged:
`wiring.rs` is not one of them and does not become one.

**Outcome:** `verified`

### F-10 — PHASE-06/EX-5 writes into a section `docs/AGENTS.md` reserves for close

**Severity:** minor
**Location:** `plan.md` PHASE-06/EX-5

**Expected:** `docs/AGENTS.md` §Close — *"Write the `## Summary` and
`## Follow-ups` sections of `slice-nnn.md`"* — after audit, at close.
`slice-003.md`'s own Follow-ups section carries `<!-- Written at close. -->`.

**Observed:** EX-5 requires a margin divergence to be *"carried into
`slice-003.md`'s Follow-ups"* at PHASE-06, which is the last execution phase and
runs before audit. PHASE-06's *Must not touch* correctly fences `docs/specs/`,
`docs/policy/`, `docs/adr/` and `design.md`, and correctly leaves
`docs/memory/` to close — so the file-placement discipline is otherwise observed
and this is the one slip.

**Evidence:** `docs/AGENTS.md` §Close; `slice-003.md` §Follow-ups; `plan.md`
PHASE-06/EX-5 and its own note *"`docs/memory/` files are lifted at **close**
… This phase names the candidates and writes nothing into `docs/memory/`."* The
same rule applies one line up.

**Disposition:** `fix-now`

**Response:**

Correct, and F-6's repair removes the occasion. With a breached margin now a
STOP rather than a follow-up, EX-5 no longer writes anything into
`slice-003.md`.

EX-5 is restated: the measured table goes into `notes.md`, and a divergence is
recorded there and named as a **candidate** follow-up, in the same words
PHASE-06's own note already uses for `docs/memory/` — the candidates are named at
PHASE-06 and the files are written at close. `slice-003.md`'s Summary and
Follow-ups stay `docs/AGENTS.md` §Close's, written once, after audit.

**Outcome:** `verified`

### F-11 — four load-bearing citations have drifted by a few lines

**Severity:** nit
**Location:** `plan.md` PHASE-02 notes, PHASE-04 notes

**Expected:** a `path:line` an executor can open.

**Observed and evidence**, all measured at HEAD 572049f:

| plan says | tree says |
|---|---|
| the refusal paths `continue` from inside `controller.rs:325-361` | the `let pending = match command` expression spans `:324-358` |
| `goad_boundary::scan::mentions` … `scan.rs:208-234` | `pub fn mentions` is at `scan.rs:225`; `:208` is the start of its doc comment |
| `Timestamp` is `Copy` (`canonical.rs:103`) | the derive is at `canonical.rs:102`; `pub fn new` is `:106` and `instant()` `:110` |
| `wiring.rs:44`'s `stub_clock` | the `fn` is at `wiring.rs:45`; `:44` closes its `#[expect]` |

Everything else checked out exactly, including the two counts AC-6 turns on
(`crates/goad/src`: 3 raw `resolve` lines at `controller.rs:255`, `wire.rs:208`
and `:215`, 0 production occurrences over 12 `.rs` files;
`crates/goad-shell/src`: 2 production occurrences of `schedule::resolve` over 8
files, at `host.rs:128` and `:259`), `app.slint:60`, `wire.rs:185`,
`schedule.rs:14` and `:198-201`, `Cargo.toml:103`, `clippy.toml:20-23`,
`answers-as-instructed.sh` lines 30 / 51 / 83-86, `host.rs:76`,
`i-slint-backend-testing-1.17.1/lib.rs:72` for
`init_integration_test_with_system_time`, and all nine named test functions.

**Disposition:** `fix-now`

**Response:**

All four re-measured and corrected in `plan.md`:

| was | now |
|---|---|
| the refusal paths `continue` from inside `controller.rs:325-361` | `:325-361` is right for the `let pending = match command` expression — it opens at `:325` and `let exchanged = pending.exchanged();` is `:362`. **Kept.** |
| `goad_boundary::scan::mentions` … `scan.rs:208-234` | `pub fn mentions` is `scan.rs:225`; `:208` opens its doc comment. Now cited as `scan.rs:225`, with the doc at `:208-224`. |
| `Timestamp` is `Copy` (`canonical.rs:103`) | the derive is `:102`, the struct `:103`. Now `canonical.rs:102-103`. |
| `wiring.rs:44`'s `stub_clock` | the `fn` is `:45`. Now `harness.rs`, unpinned — F-2's lift moves it, and pinning a line in a file this slice is about to rewrite is how the next citation drifts. |

The first row is the one place the finding is wrong, and it is withdrawn on
measurement rather than argued: the responder re-ran it and `plan.md` was right.
The rest of the finding's audit — the two AC-6 counts, `app.slint:60`,
`wire.rs:185`, `schedule.rs:14` and `:198-201`, `Cargo.toml:103`,
`clippy.toml:20-23`, `answers-as-instructed.sh` and the nine test names — was
spot-checked against the tree and agrees.

**Outcome:** `verified`

### F-12 — no criterion names the discriminant PHASE-02/EX-7's conditional re-arm needs

**Severity:** nit
**Location:** `plan.md` PHASE-02/EX-6 and EX-7

**Expected:** the exit criteria describe the same mechanism the design sketches.

**Observed:** EX-6 says winning the timer arm *"produces
`Command::Evaluate(Stimulus::Scheduled)`"*. EX-7 then requires a re-arm *"after
a refusal **that came from the timer arm**"*. A bare `Command` carries no such
provenance — `Command::Evaluate(Stimulus::Scheduled)` is also what a
hypothetical channel producer would send. `design.md` §5.4's sketch solves it
with a `Fired` discriminant returned by the `select!`; PHASE-02's notes reach
the same place informally (*"bind what the arm produced before the match"*), but
no exit criterion names it, so the phase can satisfy EX-6 literally and then
have nothing to test in EX-7.

**Evidence:** `design.md` §5.4's written-out loop, `Fired::Command(command)` /
`Fired::Scheduled`; `plan.md` PHASE-02/EX-6, EX-7 and *Notes for the
implementer*. Low stakes — the notes get the implementer there — but EX-6's
wording is what an executor checks itself against.

**Disposition:** `fix-now`

**Response:**

Correct: `Command::Evaluate(Stimulus::Scheduled)` carries no provenance, and
EX-6 as written can be satisfied by a shape EX-7 then cannot test.

EX-6 now names the discriminant. The first `select!` binds a `Fired` — the
enum `design.md` §5.4's written-out loop already uses — with `Fired::Command`
and `Fired::Scheduled` arms; winning the timer arm sets `floor_until` and
yields `Fired::Scheduled`, and the `Command::Evaluate(Stimulus::Scheduled)` it
dispatches is built after the match, on the same `stamp` path as every other
command. EX-7's *"a refusal that came from the timer arm"* then has something in
scope to be true of. The implementer note that reached the same place informally
stays, and now restates rather than substitutes for the criterion.

**Outcome:** `verified`

### F-13 — PHASE-02's bound on `wiring.rs` admits two edit classes; the fixture lift forces a third, and S-21 stops the phase for making it

**Severity:** major
**Location:** `plan.md` PHASE-02 *Surfaces*, EX-11, VA-4, S-21

**Expected:** the bound on a surface admits every edit the phase's own exit
criteria force. PHASE-02 states it exactly: *"bounded to two mechanical changes
and nothing else"* — appending `.shift` at 22 sites, and deleting seven fixture
definitions and adding one `use crate::harness::…` line. S-21 then stops the
phase for *"an edit that is not one of EX-3's 22 `.shift` sites or EX-11's
fixture move"*, and VA-4 requires the pasted `git diff` to show only those.

**Observed:** the seven moved fixtures are the sole consumers of nine imported
items, spread over eight `use` lines at the top of `wiring.rs`. `unused` is
`deny` at the workspace root with `unused_imports` explicitly kept there
(`Cargo.toml:99-102`), so leaving them is not an option: the phase cannot end
with `just check` at 0 without a third class of edit that its own bound forbids
and its own STOP catches.

**Evidence:** measured at HEAD 572049f. Each row is an item whose only
occurrences outside its `use` line are inside a fixture EX-11 moves:

| `wiring.rs` line | becomes |
|---|---|
| `:12` `use std::rc::Rc;` | deleted — only use is `glass_over` (`:77`) |
| `:15` `use goad::clock::ClockError;` | deleted — only use is `stub_clock` (`:45`) |
| `:21` `use goad_semantics::protocol::canonical::Timestamp;` | deleted — only uses are `now` (`:30`) and `stub_clock` (`:45`) |
| `:18` `use goad::generated::{OptionRow, PromptWindow, Tray};` | loses `OptionRow` (`:77`) and `Tray` (`:65`, `:69`, `:73`); `PromptWindow` stays |
| `:19` `use goad::glass::{Glass, SlintGlass};` | loses `SlintGlass` (`:73`, `:74`; `:682` and `:808` are comments); `Glass` stays |
| `:22` `use i_slint_backend_testing::{…, init_no_event_loop};` | loses `init_no_event_loop` (`:66`) |
| `:23` `use slint::{ComponentHandle, Model, VecModel};` | loses `VecModel` (`:77`); `Model` stays, used at `:992` |
| `:26` `use crate::driving::{host, instant, …};` | loses `instant` (`:31`) |

`table.rs` is unaffected — it takes only the `.shift` migration and moves
nothing.

This is the round-1 class one level down: the bound was written from the
author's account of the move rather than from the compiler's bill for it. The
repair is small — name a third admitted class, *"removing the imports the moved
fixtures were the only consumers of, listed above"*, add it to VA-4's expected
diff, and exclude it from S-21 — but it must be named, because S-21 is a STOP
and an executor obeying it stops with a red gate.

`harness.rs` needs the matching imports, and EX-11's *"moved from `wiring.rs`
unchanged"* is not literally achievable for a second reason: the seven items are
private in `wiring.rs` and must become `pub(crate)` to be reachable from a
sibling module, as `crates/goad-shell/tests/integration/harness.rs` already
spells them. VA-4 tolerates that — it forbids a renamed symbol and a changed
body, not a widened visibility — but EX-11's word does not.

**Disposition:** `fix-now`

**Response:**

Accepted, and the bill is **one item larger** than raised: **ten items across
eight `use` lines**, not nine. `Model` drops too. It never appears by name in
`wiring.rs` — it is the trait behind `.row_data`, whose only call site is
`current_view_token` (`:126`). The `:992` occurrence the finding reads as a
retained use is `.iter()` on `Diagnostics::lines()`, a slice iterator and not
`Model::iter`; an exhaustive scan for `row_data`, `row_count`, `model_tracker`
and `iter` over the file with comments and string literals stripped finds no
other. So `:23` becomes `use slint::ComponentHandle;`, losing both `Model` and
`VecModel`. `ComponentHandle` genuinely stays, on `as_weak` at `:115` and
`:387`.

Two of the finding's own rows also needed the comment strip to be right, and
they are: `SlintGlass`'s surviving occurrences at `:682` and `:808` are doc
comments, which is why it drops — the finding says so and is correct.

**The repair.** PHASE-02's bound now admits a **third** edit class, stated as
the compiler's bill rather than as a description: the eight `use` lines, named
one by one in EX-12, with what each becomes. Four lines are deleted, four are
narrowed, and one `use crate::harness::…` line is added. S-21 excludes the third
class by citing EX-12, so an executor obeying the STOP no longer stops with a red
gate. VA-4's expected diff gains the same eight lines, which keeps the bound
checkable rather than merely wider — a bound that admits "whatever the compiler
demands" would admit anything.

**The visibility point is right and EX-11's word is corrected.** The seven items
become `pub(crate)` in `harness.rs`, exactly as
`crates/goad-shell/tests/integration/harness.rs` spells its own. EX-11 now says
*"moved unchanged in body and signature, `pub(crate)` in their new home"*, and
VA-4 says visibility is the one permitted difference. `stub_clock`'s
`#[expect(clippy::unnecessary_wraps)]` travels with it.

**The class, swept.** *An edit a lint forces that the design did not ask for* is
now a thing the plan looks for rather than a thing it discovers. One other
instance exists and is repaired: PHASE-05's split of `tests/support/driving.rs`
orphans `use std::path::{Path, PathBuf};` — `Path` and `PathBuf` occur only in
`marker`, `clear`, `logging_backend`, `invocations` and `scripted`, all of which
move — so that line is deleted and `scripting.rs`'s own header carries it.
PHASE-05/EX-1's *"moved unchanged"* is qualified the same way, and VA-2's
expected diff names the line. No other phase is exposed: PHASE-01 and PHASE-03
are additive; PHASE-02's `table.rs` takes only the `.shift` migration and
orphans nothing; PHASE-04's `structure.rs` and PHASE-06's documents are declared
whole, so a forced edit inside them is already in bounds. PL-14 records the
class so a later phase looks for it before an executor does.

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

**Round 1, raiser's synthesis — 2026-09-07. Written before any disposition, so
it is a verdict on the artefact rather than on the repairs.**

**Verdict: the plan is right in shape and wrong at one seam. One blocker, five
majors, four minors, two nits.** The phase decomposition, the sequencing
argument, the fixture-driven seams and the STOP conditions are the work of
someone who read the design closely. What failed is the mechanical part the
plan is uniquely answerable for: the surface lists were written from the
design's account of the change rather than from the compiler's.

**The blocker is one class with two limbs.** PHASE-02 declares
`crates/goad/tests/renderer/wiring.rs` as *must not touch* and never declares
`table.rs`, then requires a change to `absorb`'s return type that breaks 22 call
sites in exactly those two files (F-1), and a new test module that needs seven
fixtures private to that same file (F-2). PL-2 saw half of the second limb and
the surface list was not updated to match. Nothing here threatens the design or
the phase split — the repair is to declare `wiring.rs` and `table.rs` as
PHASE-02 surfaces, bounded to the signature update and the fixture lift, and to
say so in VT-1 rather than requiring six tests to pass *unmodified* in a file
the phase must edit.

**Is PHASE-02 one Sonnet session? Yes, and the plan's calibration is honest.**
Slice 002's PHASE-10 wrote `serve`, `Pending`, `Ending` and `Served` from
nothing plus nine `serve` tests, in one session. Slice 003's PHASE-02 modifies
that loop — one arm, one anchor, two re-arm sites, one return type — and adds
seven tests using a harness that already exists (`LocalSet::run_until` +
`spawn_local`, `wiring.rs:1130-1150`). It is smaller than its own precedent. I
would **not** split it. If forced, the only seam that does not cost re-reading
the whole loop body is after EX-9 with VT-2 as the positive control, leaving
VT-3..VT-8 to a follow-on — and that seam buys a phase whose exit is "a
mechanism with one test", which is the shape PL-1 rejected for good reason. The
honest repair to sizing is F-1's: the phase is a session's work *plus* 22
mechanical call-site edits, and the plan should say so rather than forbid them.

**Can the loop grow outside PHASE-02?** Only in PHASE-03, and it is adequately
fenced: `controller.rs` is a repair surface, the diff must be quoted in the
sheet and named as a finding, S-9 stops a shape change, and PHASE-06/VA-5 diffs
touched paths against declared surfaces at the end. PHASE-04, PHASE-05 and
PHASE-06 each forbid it explicitly or forbid all of `src/`. The one defect is
EX-3's wording, which asserts "no production behaviour changed" and then
describes the behaviour change it permits (F-7).

**Are the surfaces complete and tight?** Tight, yes — every *Must not touch*
list I checked is real, and PHASE-05's integration-target list matches exactly
the six files that import from `driving.rs`, with `fake.rs` correctly excluded.
Complete, no: F-1, F-2 and F-9 are three instances of the same omission, in
three phases.

**FD-3 is the plan's one piece of scope creep, and it is DRY doing the
creeping** (F-3). The split works — I traced the six moved symbols and all are
reachable from `scripted`, so `dead_code` is genuinely satisfied — but it costs
eight files the slice's charter does not name, including a whole test target of
another crate, and it was decided under the grant while FD-1's *one*-file
widening was escalated on the explicit reasoning that widening scope is not a
plan question. The cheaper answer, `#[allow(dead_code)]` on the new target's own
`mod` declaration, is not in PL-5's rejected list; PL-5 rejected only the
per-symbol `#[expect]` inside the shared file, which is a different thing and
fails for a reason that does not apply. Cheaper is not automatically right —
POL-001's prohibition on suppressing a lint is a real counter-argument — but it
is the argument PL-5 owed and did not make.

**PHASE-01's stratum 1 + stratum 3 pairing is acceptable.** Both halves are
leaves, neither depends on the other, both are PHASE-02's prerequisites, and the
plan says plainly that neither fills a session. The *Must not touch* list is the
tightest in the plan and stops either half growing. Cohesion by sizing, stated
as sizing, is not a defect.

**PL-6's instrument is the right one, aimed correctly, with no trigger
attached** (F-6). Measuring at the phase that lands the test is right;
`design.md` §9 is correctly not retro-fitted; VA-4's three green runs is real
evidence. But a liveness margin measured below 5x becomes a finding and a
follow-up, and ships — inside a slice whose AC-12 says a load-sensitive test is
a design defect and not a tolerated cost. A breach needs a STOP, in the shape
PHASE-02/S-6 already has for a neighbouring trigger.

**Criteria decidability.** Almost all of them are mechanical, which is unusual
and worth saying. The exceptions are PHASE-03/EN-2's coverage claim (F-8), which
no instrument in the gate can settle, and PHASE-03/EX-3 (F-7). PHASE-04/EN-2 is
the model the rest should copy: it names a test and a markup condition, and both
are run rather than judged.

**Coverage is where the plan is thinnest.** AC-1, AC-2, AC-3, AC-5, AC-6, AC-7,
AC-8, AC-9, AC-10, AC-11 and AC-12 are each genuinely discharged by the cited
criterion, several of them with a break-and-revert control the design did not
ask for and should have (PHASE-04/VA-2 and VA-3 are the best criteria in the
plan). AC-4 is claimed and not delivered (F-4). And the draft spec — the slice's
own working authority — is never walked at all: R-4's second half, the *"a
person acting does not clear the floor"* property that is the whole point of
D-3's re-anchoring, has no test in any phase, and PHASE-06/EX-1 is the first
place anyone will notice (F-5).

**FD-1 through FD-4, against `docs/AGENTS.md`'s rule.** FD-1 was escalated,
ruled at design as D-17, and cross-posted correctly to both logs — exemplary.
FD-2 and FD-4 are mechanical discoveries about how to express what the design
already decided, and settling them in the plan is right. FD-3 changed the
slice's scope and should have gone back the way FD-1 did (F-3).

**Gate and memory gotchas: the plan walks into none of them.** Every one is
cited where it bites — the four-level `#[path]` and the cargo cwd rule at
PHASE-05, the `-D warnings` promotion of `dead_code` at FD-3, `clippy.toml`'s
test carve-outs at PHASE-02/EX-8, the `expect_dead_code` cfg-attr trap as the
stated reason PHASE-02 does not land a branch nothing drives, and the
`#[cfg(test)]` cut as the reason PHASE-01 must **not** rename `wire.rs`'s two
test functions. I re-ran the two counts AC-6 turns on and both are as the plan
states. `tokio`'s `time` feature is already unified in from
`[workspace.dependencies]`, so PHASE-02 needs no manifest change and POL-001's
residue really is untouched.

**What this review did not test:** the implementation, which does not exist; the
number 3 seconds and the six-phase count, both decisions rather than defects;
and whether the measured margins will hold, which is what PL-6 exists to find
out.

**Risks left standing if every finding is fixed as raised.** PHASE-03's
`controller.rs` repair surface stays a judgement call about "repair" versus
"extend", mitigated but not closed. The margin table remains estimates until
PHASE-06. AC-10's one substituted component cannot be closed by anything. And
the `until`-plus-fixtures lift out of a 1232-line `wiring.rs` is the kind of
mechanical churn that hides a behaviour change; PHASE-05's VA-2 has the right
instrument for the analogous move (`git diff --stat`, no renamed symbol, no
changed body) and PHASE-02 should borrow it.


**Round 1, responder's synthesis — 2026-09-07. Written after dispositioning all
twelve and integrating the repairs, and it is a report on what changed, not a
second verdict.**

**Every finding is `fix-now`. Nothing was deferred, downgraded or tolerated,
and no `follow-up` was reached for.** The raiser's diagnosis is accepted whole:
the surface lists were written from the design's account of the change rather
than from the compiler's, and it recurred in three phases. So the repair was not
to patch three surfaces but to re-derive every phase's list from what the change
actually touches, and then declare exactly that.

**The blocker's repair is a declaration, not a redesign.** PHASE-02 declares
`wiring.rs` and `table.rs`, bounded to two mechanical changes: `.shift` appended
at the 22 sites that read `absorb`'s return value, and the seven-fixture move
out of `wiring.rs`. The alternative the brief offered — a new fn beside the old,
the old removed after migration — was rejected on evidence rather than taste: it
moves the same 22 call sites in the same phase and adds a second name,
which `CLAUDE.md` forbids. VT-1's *unmodified* claim survives intact because all
six named tests begin at or after `wiring.rs:923`, past the last read site
(`:901`); VA-4 is the new `git diff` instrument that holds it, borrowed from
PHASE-05 as the raiser suggested.

**PHASE-02 is not split.** The question was reopened and answered the same way,
with the reasons now on the page: the 22 edits are uniform and add no import,
and the fixture move is seven definitions plus one `use` line because a child
module's `super::X` already resolves through a private parent import — the tree
proves it today at `wiring.rs:706`. Neither is re-reading, which is what a
session's budget actually buys.

**The fixture lift has a home with a precedent.**
`crates/goad/tests/renderer/harness.rs`, on the pattern
`crates/goad-shell/tests/integration/harness.rs:5-10` already states in its own
words. Seven fixtures have two consumers and move; the five that have one stay.
`tests/support/` is refused twice over — three of the seven name Slint types the
`integration` target cannot see, and FD-3's own rule forbids it.

**F-3 splits in two and resolves in opposite directions.** The cheap alternative
is refused by canon, not by preference: POL-001 §Compliance says *"never
`allow`"* in the sentence that authorises `#[expect]`, and the memory file says
the same. But the escalation asymmetry the finding names is real and is
repaired — the split is now D-18 in `design-log.md` and `design.md` §7, and
`slice-003.md`'s Scope names the eight files, the treatment PL-8 gave FD-1 for
strictly less.

**Two coverage holes are closed by a table rather than by vigilance.** AC-4's
first successor case gets PHASE-03/VT-5, which asserts the retained value
returns to `now + default_poll` after a one-off past instruction — observable
through the loop, exact against the fixed clock, and costing no floor interval,
which is the trade AC-5's own revision already made. Draft SPEC-002's R-1..R-11
now have their own Coverage table, and it is what surfaced the missing
criterion: PHASE-03/VT-6, a person acting mid-cadence, which discharges R-4's
second half and R-5's second half in one run. The raiser is right that this is
the slice's central claim and had no test.

**PL-6 is superseded rather than patched.** PL-11 keeps the threshold and the
measurement discipline and changes the consequence: a breached margin is a STOP
at the phase that measured it — S-20, S-22, S-26 and S-19 — and the orchestrator
is consulted. F-10 then dissolves: with no follow-up to write, PHASE-06/EX-5 no
longer reaches into a section `docs/AGENTS.md` reserves for close.

**Where the responder disagrees.** One row of F-11, re-measured: the `let
pending = match command` expression does open at `controller.rs:325`, and
`:362` is `let exchanged`, so the plan's `:325-361` was right. The other three
citations drifted as stated and are corrected; `stub_clock`'s is now unpinned,
because F-2's lift moves the file it was pinned in.

**What a round-2 verifier should attack first.** PHASE-02's bounded surfaces are
the new risk this repair introduces: *bounded to two changes* is a sentence, and
VA-4 is the only thing between it and a third change in a 1232-line file. After
that, PHASE-03's size — it went from four criteria to six — and whether VT-5's
asserted instant (`2026-01-01T00:00:00.100Z`) is actually what `resolve` yields
against `harness::stub_clock` and a 100 ms `default_poll`. Both are traced in
the plan and neither has been run.

---

**Round 2, raiser's synthesis — 2026-09-07. All twelve round-1 findings
`verified`; one new, `major`.**

**Verdict: the repairs are good and the plan is close. One outstanding finding,
0 blocker.** The blocker is discharged properly rather than argued away:
PHASE-02 now declares `wiring.rs` and `table.rs` as surfaces bounded to two
mechanical changes, cites all 22 `absorb` read sites by line, and I confirmed
every one of them against the tree. The seven fixtures move to a new
target-local `crates/goad/tests/renderer/harness.rs` on the pattern the
integration target already sets, and the claim that carries the move —
`super::X` resolving through a private parent import — is real at
`wiring.rs:706`, which reaches five of the seven that way today.

**The three the responder asked to be attacked first.**

- **The bound on `wiring.rs`, with only VA-4 behind it.** The instrument is
  adequate — a pasted `git diff` plus S-21 plus the audit's own surface diff is
  as much as a bounded surface can carry — but the *bound itself* is short by
  one edit class, and that is F-13. The seven moved fixtures are the sole
  consumers of nine imported items across eight `use` lines; `unused_imports` is
  denied at the workspace root, so the phase cannot end green without touching
  them, and S-21 stops it for doing so. Same class as round 1: written from the
  author's account of the move, not the compiler's bill.
- **PHASE-03 at six criteria.** It still fits. The six are four fixtures —
  VT-1/VT-5/VT-6 are three instruction lists against one past-instant shape,
  VT-3/VT-4 are one clock fixture read twice, VT-2 is `@garbage` — the phase
  writes no production code beyond a doc comment and a possible repair, and the
  plan's own arithmetic for it is now on the page and checks out. It is smaller
  than slice 002's PHASE-10. PHASE-02 is the one now at the ceiling, and its
  answer to that (the 22 edits are uniform, the lift is a move, neither is
  re-reading) is the right argument.
- **VT-5's asserted instant is correct.** I traced it: the one-off past
  instruction is returned verbatim by `resolve`'s first arm; the next response
  carries no instruction, the retained value is not `> now`, so the third arm
  yields `now + default_poll`, and against `stub_clock`'s fixed
  `2026-01-01T00:00:00Z` with a 100 ms poll that is exactly
  `2026-01-01T00:00:00.100Z`. `schedule.rs:311`'s
  `an_elapsed_retained_check_is_consumed_and_the_default_poll_applies` exists and
  asserts that arm at zero cost, so VT-5's refusal to await the third invocation
  is paid for. S-23 is the right guard.

**F-3's canon citation is real.** POL-001 §Compliance, lines 74-78, authorises a
site-local `#[expect(lint, reason)]` and says *"never `allow`, which is silent
when it stops being true"*, with one module-scoped carve-out for generated code.
`docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md:42` says the
same. PL-10 quotes both accurately, and the widening is escalated to design as
D-18 rather than decided in the plan — which is FD-1's treatment applied to a
larger widening, as it should be.

**What the other repairs did.** F-4 and F-5 are the two that added tests rather
than words: PHASE-03/VT-5 drives AC-4's missing successor case, and
PHASE-03/VT-6 drives the *"a person acting does not clear the floor"* property
that nothing asserted, with VA-3's break-and-revert making it evidence rather
than an assumption. A second Coverage table now walks draft SPEC-002's R-1..R-11,
so the hole F-5 found cannot recur by omission. F-6's repair is a STOP at all
three measuring phases (S-19, S-20, S-22) rather than a follow-up. F-7, F-8, F-9,
F-10 and F-12 are each restated so an executor decides them mechanically, and
three of F-11's four citations are corrected in place while the fourth is moot.
No acceptance criterion was weakened and no severity was negotiated down.

**What a round 3 should attack first.** F-13's repair, and specifically whether
naming a third admitted class leaves the bound still bounded — the point of the
bound is that a 1232-line file cannot be opened for general work, and each
admitted class is a crack. Then `harness.rs`'s visibility: the seven items must
become `pub(crate)`, which EX-11's *"unchanged"* does not admit and VA-4 does
not forbid. Then PHASE-03/VT-6's window arithmetic, which is the only assertion
in the slice whose failure depends on a floor *not* being cleared by an event
the test itself injects.

---

**Round 3, raiser's synthesis — 2026-09-07. Ledger resolved.**

**Verdict: all thirteen findings `verified`. Nothing outstanding at any
severity — 0 blocker, 0 major, 0 minor, 0 nit. Nothing withdrawn, nothing
dispositioned `aligned` or `tolerated`, no severity negotiated down.**

Round 3 verified F-13 alone, by re-running the compiler's bill rather than
reading the argument for it. **EX-12's bill is right and mine was not.** I had
`Model` staying; it goes. Its only method use is `.row_data(0)` at
`wiring.rs:126`, inside `current_view_token`, which moves — the `.iter()` at
`:992` is on the `&[String]` `Diagnostics::lines()` returns
(`diagnostics.rs:165`), not on a model. So `:23` becomes
`use slint::ComponentHandle;`, and the count is ten items over eight lines, not
nine. Every other row checks out: `Rc`, `ClockError`, `Timestamp`,
`OptionRow`, `Tray`, `SlintGlass`, `init_no_event_loop`, `VecModel` and
`instant` have no consumer left, and `PromptWindow`, `Glass`, `ElementHandle`,
`ElementQuery`, `Duration`, `host`, `invocations`, `quiet_event` and `scripted`
all do. `ComponentHandle` stays for the reason EX-12 gives: `window_shown`
(`:114-116`) calls `window.window()`, a trait method, and that fixture is one
of the stayers. `harness.rs`'s mirror list is complete, and correctly omits
`Glass`. PHASE-05's orphan is real too — after the six helpers leave,
`driving.rs:16`'s `use std::path::{Path, PathBuf};` has no consumer at all.

**What three rounds changed.** A phase that could not compile inside its own
surfaces now declares them, at line-level precision, with a pasted `git diff`
and a STOP behind the bound. A DRY-shaped scope widening was escalated to
design as D-18 on the same reasoning that escalated FD-1, after the cheap
alternative was evaluated and refused against POL-001's actual words. Two
acceptance claims that were mentions became tests — AC-4's other successor case
and, more importantly, the *"a person acting does not clear the floor"*
property the design argues hardest for and nothing asserted. A margin breach
became a STOP at the phase that can still fix it, instead of a follow-up that
ships. And five criteria that asked an executor for a judgement now ask for a
grep, a document check, or a value.

**What it confirmed.** The six-phase split, the fixture-driven seams, PHASE-02
as one session and PHASE-03 as another, and the calibration against slice 002's
PHASE-10. None of that moved under attack.

**The risks it knowingly leaves standing.** PHASE-03's `controller.rs` repair
surface is still a judgement about repair versus extension, fenced by a quoted
diff and S-9 rather than closed. The margin table is estimates until PHASE-06
measures it. AC-10's one substituted component nothing can close. And the
`wiring.rs` bound now admits three classes where it began with two — each one
argued and each one billed in advance, which is the right way to widen a bound,
but a fourth would be worth resisting hard.

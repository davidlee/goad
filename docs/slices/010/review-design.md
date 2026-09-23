# Review — design — Slice 010

**Subject:** design — `draft-spec.md`, `design.md` and `canon-delta.md` in
`docs/slices/010/`, as they stand at `40caa4a` plus this slice's uncommitted
documents. The plan does not exist yet and is not under review: this is tier 2,
so the plan gets its own ledger.
**Reviewer:** fresh agent, no part in writing the artefacts
**Opened:** 2026-09-23
**State:** open — F-1…F-62 `verified`. Round 6's F-63…F-68 are disposed with
the user and integrated. **No round 7**: round 6's one design-level finding,
F-63, was repaired by a case and its citations, not a design change, so its
repairs closed by a site check (the user's call, and round 6's synthesis).
**Closed** — F-1…F-68 `verified`, no blocker outstanding.

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
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

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
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-09-23 — the design and the canon it proposes.

**What this review is for.** The slice writes a **new spec**. Canon is the
thing nothing re-reads once written, so a defect admitted here is the most
expensive kind this project has: `canon-delta.md` exists in this very slice
because a sentence written into SPEC-003 went stale and no instrument noticed
for three slices. The draft spec is held to the standard canon is held to, not
the standard a design document is held to.

**The invariants the subject is held to.**

- `CLAUDE.md` §Working here — **name, never count**, and **cite by symbol**. A
  count of statuses, requirements, variants or call sites is a defect wherever
  it appears, including in prose that reads as incidental.
- `CLAUDE.md` §Code Standards — no parallel implementation; find whether
  existing code can be adapted before new code is added; coupling and cohesion;
  naming.
- `docs/AGENTS.md` §"Canon that does not exist yet" — the draft is the slice's
  working authority and nothing outside the slice folder may cite it.
- ADR-001 (one-way strata) and POL-001 (the phase gate).
- The project's own invariant that a clause no cooperating test can reach is
  **declared in terms**, saying what review holds *and what it does not* —
  SPEC-003/R-3's `LivenessUnknown` cell is the form.

**Surfaces where a defect would be expensive, named without prejudging what is
there.** The requirement set's completeness and its edges; the verification
table's honesty, case by case, against what the cited tests can actually
observe; the boundary the draft spec claims against SPEC-001 and SPEC-003; the
seam between the two channels `run` answers and what each admits; the names
this design introduces into a crate that already has neighbours for them; and
the `canon-delta.md` replacement text, which will be applied to a live spec
verbatim.

**Two decisions are closed and are not the review's to reopen** — the phase
axis, and no transient split inside *never started* (`design-log.md`,
2026-09-23, both endorsed by the user). A finding that the design implements
either of them *badly* is in scope. A finding that the decision was wrong is
not, unless the reviewer has evidence the user did not have, in which case it
is raised as a finding against the decision's stated reasoning and says so.

**Round 2** — 2026-09-23 — the repairs round 1 made.

**What this review is for.** Round 1's repairs are unreviewed, and this
project's own history is that the commit repairing a false claim is where the
next one gets written — `canon-delta.md` exists because of exactly that, and
F-11 demonstrated it inside round 1: the finding was correct and its proposed
repair wording reached for a second unobserved fact. **The subject is the
repaired text, not the design.** A finding that re-litigates a decision round 1
already disposed is out of scope unless it carries evidence round 1 did not
have.

**The standard is unchanged and applies harder here.** Canon is the thing
nothing re-reads once written. `canon-delta.md`'s replacement text will be
applied to a live SPEC-003 **verbatim** at audit, and the draft spec is promoted
to a numbered spec. Both are held to the standard canon is held to.

**Surfaces, named without prejudging what is there.**

- **Every sentence round 1 rewrote.** F-1…F-11's repairs, against the artefacts
  as they now stand — not against the finding text that motivated them. Read
  the current documents first and the ledger second.
- **Claims the repairs introduced that nothing has checked.** A repair may
  assert something true of the tree, false of it, or not yet true of it. Two are
  known to be new and are named here as *surfaces to check*, not as defects:
  `canon-delta.md`'s assertion about where R-4's failures settle, and the draft
  spec's R-2 cell, which cites a test by a name canon will carry and that does
  not exist in the tree yet. There may be others; the named two are a starting
  point, not the list.
- **The class F-11 names, swept across the whole spec.** F-11 was one row
  telling a consumer something the process never observed. §6 is billed as *"the
  whole of what may be inferred from each"*, and §3 P-A says a status carries no
  more than which outcome happened. Every cell of §6, and every requirement in
  §4 that describes what a status or a line means, is in scope for the same
  question: *did the process watch this happen?*
- **The gate case `structure::the_loop_s_ending_is_never_a_startup_failure`,**
  which round 1 added: whether it holds what `design.md` §8 R2 and the draft
  spec's R-2 cell say it holds, and whether the limit those sites state is the
  limit it actually has.
- **Cross-document integrity after eleven edits.** Ids, symbol citations and
  cross-references that resolved before the repairs and may not now — including
  between `design.md`, `draft-spec.md`, `canon-delta.md` and `slice-010.md`'s
  acceptance criteria. Whether any repair silently moved, weakened or orphaned
  an acceptance criterion.
- **`CLAUDE.md` §Working here, over the repaired text specifically.** Name,
  never count; cite by symbol. Round 1's repairs added prose, and prose is where
  a count gets written.

**Out of scope, as in round 1.** The phase axis and the absence of a
transient/permanent split inside *never started* are closed by the user
(`design-log.md`, 2026-09-23). `plan.md` does not exist yet and is not under
review.

**Round 3** — 2026-09-23 — the repairs round 2 made.

**What this review is for.** Round 2's subject was round 1's repairs, and it
found **both its blockers inside them** — F-14 in the phrase F-7's repair
introduced, F-15 inside `canon-delta.md` itself. That is a measured rate, not a
worry: this project's repairs have now written a blocker in each of two
consecutive rounds. Round 3's subject is round 2's repairs, and the brief is
narrower than round 2's because round 2's clean sweeps are recorded in its
synthesis and are **not** to be re-billed.

**The narrow subject — the four places round 2 wrote new normative text.**

- **F-12's recut.** R-2 and R-3 were re-cut onto *the call that runs the event
  loop* rather than the loop having begun; R-6 gained an exception clause; §5
  gained a new paragraph, *What the seam costs*, ending in a normative **a
  consumer MUST NOT read 1 as evidence that the host did any work**; §5's
  diagram transitions, §6's rows and the *cut is phase, not retryability*
  paragraph all moved with it. The questions this raises and does not answer:
  whether R-1, R-2 and R-3 now partition every end the host can reach, with none
  admitted by two of them and none by none of them; whether *reached the call*
  is observable at the point the classifier runs; whether the new MUST NOT is a
  requirement anything can hold, or prose wearing a MUST; and whether §5's
  residue paragraph and R-6's exception say the same thing as each other.
- **F-16's instrument.** The gate case now reds on two needles. Check the second
  one against the **passing** form the design specifies, not only against the
  defect: a needle that the correct code also contains is a case that cannot go
  green. Check that each needle has a compiled fixture, and that the two stated
  limits are still the limits.
- **F-18's promotion gate.** §7's preamble now carries a MUST forbidding
  promotion while any case it names does not resolve. Whether that is
  enforceable, whether anything at audit is positioned to hold it, and whether
  it agrees with `docs/AGENTS.md` on how a draft is promoted.
- **F-13's exception rewrite.** R-4 took an exception into its own clause and §2
  stopped quantifying over the requirement set. Whether R-4's new wording
  changes what R-4 requires of a conforming host, which is not what the finding
  asked for.

**Two further surfaces, because eleven edits landed across four documents.**

- **`canon-delta.md`'s replacement block**, again, and for the same reason as in
  round 2: it is applied to a live SPEC-003 **verbatim**. It was rewritten by
  F-15.
- **Cross-document integrity and the acceptance criteria.** Whether any of
  `slice-010.md`'s acceptance criteria is now unmet, orphaned or made false by
  the recut — AC-4 and AC-6 in particular, which were deliberately left
  unchanged — and whether `design.md` §9's and §10's reconciliation rows still
  name every obligation the documents now carry.

**And, over round 2's new prose specifically**, `CLAUDE.md` §Working here: name,
never count; cite by symbol.

**Out of scope.** The phase axis and the absence of a transient split inside
*never started* remain closed by the user. The decisions taken at round 2's gate
— narrowing what 1 claims rather than bounding an exception, and closing F-16's
route rather than documenting it — are the user's and are not to be reopened; a
finding that either is *implemented* badly is in scope. `plan.md` still does not
exist.

**Round 4** — the repairs round 3 made, **two of which changed the design**.

**What this review is for, and why its subject differs in kind.** Rounds 1–3
each found a blocker inside the previous round's repairs; that is measured, and
round 4 exists because of it. But rounds 2 and 3 reviewed *prose*, and round 3's
repairs include a **code change** and a **reversed deferral**. A design change
carries risks prose does not: a race, a wrong observation point, an instrument
that forbids more than intended. Weight the reading accordingly.

**The narrow subject.**

- **F-23's code change** — `Cancel::is_stopped` (`crates/goad/src/wire.rs`), the
  clone `start` retains, and the loop call's `Err` arm answering
  `Ended::AsAsked` when a stop was asked for and `Ended::StoppedRunning`
  otherwise. With it: R-1's new normative clause about deciding volition from
  the request, R-2's *and no stop had been requested*, §5's opening sentence
  that volition is not part of the seam's cost, `design.md` §5.1's paragraph,
  A5, §9's assertion row and mutation, and `Ended`/`StoppedRunning`'s doc
  comments (F-29). Nothing here has been compiled. Questions worth asking and
  not answered here: **when** `is_stopped` is read relative to everything that
  can set it and everything that can end the loop; whether every reachable
  combination of (stop requested, call result) lands in the class R-1 and R-2
  require; and whether `Cancel` is the right owner of the fact.
- **F-26's landed case and the three-way delta** — `slice-010.md` §Scope's new
  test bullet, AC-8's added sentence, the struck §Follow-ups row, and
  `canon-delta.md`'s Changes 1, 2 and **3**. Change 3 is new normative text that
  will be applied to SPEC-003 verbatim; check its replacement against **the
  live R-3 cell**, not against what this ledger says the cell contains.
- **F-28's swapped instrument** — the second needle is now `impl From<…> for
  StartupError` rather than a call-site spelling. Check what it catches, what it
  misses, **and what it forbids**: a needle stated that broadly is a standing
  constraint on a type, not only a guard against one regression. Check that
  `design.md` §5.2, §8 R2, §9's row, §9's mutation list and the draft spec's R-2
  cell now say the same thing as each other, since round 3 raised F-27 because
  they did not.

**Two further surfaces.**

- **Whether the repairs completed each other.** Thirty-four findings have edited
  four documents. Is any acceptance criterion now unmet, orphaned or falsified?
  Do `design.md` §9's and §10's lists still name every obligation the documents
  carry? Round 3's F-27 and F-30 were both *a repair that did not reach all its
  sites*, and that class has now appeared in two consecutive rounds.
- **`CLAUDE.md` §Working here over round 3's new prose**: name, never count;
  cite by symbol.

**Out of scope.** The phase axis, and no transient split inside *never started*.
Round 3's gate decisions — closing F-23 in code, landing F-26's case, swapping
F-28's needle — are the user's and are not to be reopened; a finding that one is
*implemented* badly is in scope. Round 3's clean sweeps are in its synthesis and
are not re-billed. `plan.md` still does not exist.

**Round 5** — the repairs round 4 made, **three of which changed the design**.

**The narrow subject.**

- **`exit::ended`** (F-37) — the function, its guard, the arms' cases over one
  error value, the two mutations, and what §9 now says no mutation measures.
  Check **when** `start` reads `is_stopped` against everything that can trip
  `Cancel`, and whether the `Ok` arm's reliance on A1 is stated wherever the
  arm is described.
- **The re-cut scan** (F-43, F-44) — *exactly one production line names
  `run_event_loop_until_quit`, and it ends at the call*. Check what it catches,
  what it misses and what it forbids, against `code_of`, `production_lines` and
  `occurrences_where` as they are. Check its derived limit (a later statement
  re-filing the binding) against the matcher. Check that `design.md` §5.2, §8
  R2, §9's rows and mutations, and the draft spec's R-2 cell all say the same
  thing. That the call's uniqueness is now held is a new claim.
- **R-1 says *asked*** (F-46, F-51) — the definition in R-1, the reworded MUST
  NOT, and every site that followed them. Check the winit reading in `design.md`
  §5.1 and A5 at the symbol (`winit-0.30.13`, `i-slint-core-1.17.1`). A claim
  about what a compositor sends is exactly the kind this review has found
  overstated before.
- **`canon-delta.md` Change 3**, now a removal from the analogy. Check it
  against the live SPEC-003 R-3 and R-5 cells.

**Two further surfaces.** Whether the sweeps reached every site (F-42's list was
wider than the finding named), AC-11 against R-1's clause, and F-52's rename
of a case name at every site outside this ledger.
The orchestrator's pre-round consistency pass, listed at the foot of
`notes.md` §Handover, is subject too.
`CLAUDE.md` §Working here over round 4's prose.

**Out of scope.** The phase axis; no transient split inside *never started*;
rounds 1–4's user decisions, including the pure function, dropping the `From`
needle, and *asked*. A finding that one is *implemented* badly is in scope.
`plan.md` is still the template.

**Round 6** — the repairs round 5 made, **one of which changed the design**.

**The narrow subject.**

- **Every end decided on the request** (F-53, F-54). `exit::ended` as an `if`
  on `stop_requested`, `Ended::StoppedRunning(Option<slint::PlatformError>)`,
  `report_exit_line`'s `StoppedRunning(None)` arm and its sentence, the split
  `Ok` cases, and §9's mutations for the decision. Check that nothing still
  reads the call's result as whether a stop was asked for, in either direction,
  at any site: A1 was rewritten, A5 was not; R-1's requirement, its cell, R-2's
  cell, §5 *What the seam costs*, §6, AC-11, `slice-010.md` §Scope. Check that
  `Ending::Closed` now exiting 1 is stated consistently wherever it was a
  residue. Check the new sentence against R-4 and R-6, and whether the `None`
  line needs the pair treatment R-6's row gives the other.
- **The scan's limit stated from the matcher** (F-55) at `design.md` §5.2, §8
  R2 and R-2's cell, and the review claim added beside it.
- **`canon-delta.md`** Change 1's new lead (F-58) and Change 3's R-5 sentence
  (F-59), against live SPEC-003.

**Also.** F-56, F-57, F-60, F-61 and F-62 at their sites. `CLAUDE.md` §Working
here over round 5's prose — §9's mutation list was rewritten as a list.

**Severity rule for this round (user, 2026-09-23).** `blocker` and `major` are
reserved for **design** defects — a behaviour, type, decision or verification
claim that is wrong. Prose that is imprecise, inconsistent or incompletely swept,
but states no wrong design, is `minor` or `nit` however many sites it touches.
The subject is round 5's repairs only; surfaces they did not touch are not
re-reviewed.

**Out of scope.** The phase axis; no transient split inside *never started*;
rounds 1–5's user decisions, including route (a) and `Option` over a
constructed error. A finding that one is *implemented* badly is in scope.
`plan.md` is still the template.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | doc-wrong | verified |
| F-2 | blocker | doc-wrong | verified |
| F-3 | blocker | doc-wrong | verified |
| F-4 | major | fix-now | verified |
| F-5 | major | fix-now | verified |
| F-6 | major | doc-wrong | verified |
| F-7 | minor | doc-wrong | verified |
| F-8 | minor | doc-wrong | verified |
| F-9 | minor | fix-now | verified |
| F-10 | nit | doc-wrong | verified |
| F-11 | major | doc-wrong | verified |
| F-12 | major | doc-wrong | verified |
| F-13 | major | doc-wrong | verified |
| F-14 | blocker | doc-wrong | verified |
| F-15 | blocker | doc-wrong | verified |
| F-16 | major | fix-now | verified |
| F-17 | minor | fix-now | verified |
| F-18 | minor | doc-wrong | verified |
| F-19 | minor | fix-now | verified |
| F-20 | minor | fix-now | verified |
| F-21 | minor | fix-now | verified |
| F-22 | nit | fix-now | verified |
| F-23 | blocker | fix-now | verified |
| F-24 | blocker | doc-wrong | verified |
| F-25 | major | doc-wrong | verified |
| F-26 | major | fix-now | verified |
| F-27 | major | doc-wrong | verified |
| F-28 | major | fix-now | verified |
| F-29 | major | doc-wrong | verified |
| F-30 | minor | doc-wrong | verified |
| F-31 | minor | doc-wrong | verified |
| F-32 | minor | doc-wrong | verified |
| F-33 | minor | doc-wrong | verified |
| F-34 | minor | fix-now | verified |
| F-35 | blocker | doc-wrong | verified |
| F-36 | blocker | doc-wrong | verified |
| F-37 | blocker | fix-now | verified |
| F-38 | blocker | doc-wrong | verified |
| F-39 | major | doc-wrong | verified |
| F-40 | major | doc-wrong | verified |
| F-41 | major | doc-wrong | verified |
| F-42 | major | doc-wrong | verified |
| F-43 | major | fix-now | verified |
| F-44 | major | fix-now | verified |
| F-45 | major | doc-wrong | verified |
| F-46 | major | fix-now | verified |
| F-47 | major | fix-now | verified |
| F-48 | minor | fix-now | verified |
| F-49 | minor | fix-now | verified |
| F-50 | minor | fix-now | verified |
| F-51 | major | doc-wrong | verified |
| F-52 | minor | fix-now | verified |
| F-53 | blocker | fix-now | verified |
| F-54 | major | doc-wrong | verified |
| F-55 | major | doc-wrong | verified |
| F-56 | minor | doc-wrong | verified |
| F-57 | minor | doc-wrong | verified |
| F-58 | minor | doc-wrong | verified |
| F-59 | minor | doc-wrong | verified |
| F-60 | minor | doc-wrong | verified |
| F-61 | nit | doc-wrong | verified |
| F-62 | nit | doc-wrong | verified |
| F-63 | major | doc-wrong | verified |
| F-64 | minor | doc-wrong | verified |
| F-65 | minor | doc-wrong | verified |
| F-66 | nit | doc-wrong | verified |
| F-67 | minor | doc-wrong | verified |
| F-68 | nit | doc-wrong | verified |

### F-1 — `Ended`'s doc comment and the "`Ok` means the host started" seam are false of the type the design specifies

**Severity:** blocker
**Location:** `design.md §5.2` (the `Ended` doc comment; the paragraph beginning *A failure travelling in the `Ok` channel*), `design.md §7 D1`

**Expected:** the repair this slice exists to make is a type whose doc comment is
true of it. `design.md §1` states the defect as `StartupError`'s doc claiming
*every way `run` can fail to reach the event loop* while `start` routed the
loop's ending through it; AC-3 is that the claim becomes true again. The
replacement type must not repeat the form.

**Observed:** `Ended`'s doc comment, as written in §5.2, says the type is *"How
a host that started ended. The `Ok` channel of what `run` answers, and so, by
construction, only reachable once the event loop has begun."* The same section's
`run` signature comments the Help and Version arms as `Ok(Ended::AskedToStop)`,
and §5.5's edge table gives `--help` / `--version` → `Ok(Ended::AskedToStop)`.
Those arms are reached from `run`'s `match` over `Launch` before `start` is
called at all, so the event loop has not begun and no host has started. The
prose beside it — *"`Ok` here means *the host started*, which is the phase
seam"* — is false for the same two arms, and D1 leans on it: *"a reader checking
the code against the spec reads the `Result`'s channels as the phase seam.
§5.2's doc comment is where that is said, and it is load-bearing."* The
`Result`'s channels are not the phase seam. `Ok` spans *answered a question,
never having started* and *started, then ended*; only `Err` is a phase.

The variant's **name** carries the same claim: the draft spec's §6 labels the
class *asked to stop* and R-1 admits *an invocation that was a question,
answered* into it, so `Ended::AskedToStop` is the value `--version` produces.

**Evidence:** `run`'s current arms are `Launch::Help => { diagnostics::print_usage(); Ok(()) }`
and `Launch::Version => { … Ok(()) }` (`run`, `crates/goad/src/main.rs`), both
returning before `start` and therefore before `slint::run_event_loop_until_quit`.
`design.md §5.2`'s own `run` line documents them as the `Ok(Ended::AskedToStop)`
arms. Draft spec R-1 and §5's state diagram both make *Answered* a state
distinct from *Running*, reaching 0 without passing through it.

Note the rest of D1 survives: OQ-1's other argument against the flat enum — that
it takes `?` away from `start` — is independent of the seam claim and is not
touched by this finding. What is touched is the second argument, *"the `Result`
shape has no value in the wrong channel"*: `Ok(Ended::AskedToStop)` produced
before the loop is exactly a value whose type-level claim the shape cannot keep.

**Disposition:** doc-wrong
**Response:** the design was the defect. `Ended`'s doc comment now says the
type is *how the process ended when it did not fail to start*, and names both
ends inside `Ok` — a question answered with the loop never having begun, and a
loop that began and ended. The seam paragraph in §5.2 says `Err` is *never
started* and `Ok` is *did not fail to start*, so the channel bounds one class
rather than being a three-way phase seam. D1's consequence is rewritten to the
same shape: the reader takes one boundary from the `Result` and the other from
`Ended`, and D1 now states in terms that a version of the comment claiming `Ok`
means the host started would be false of `--help`. OQ-1's answer is unchanged —
its `?`-in-`start` argument never depended on the seam claim — but OQ-1's second
argument is corrected rather than left standing.

**The name went with it, endorsed by the user** (`design-log.md`, 2026-09-23):
`Ended::AskedToStop` is `Ended::AsAsked`, the draft spec's §6 class is *as
asked*, and the two test names that carried the old label follow —
`exit_status::as_asked_is_0` and
`stderr_outlets::report_exit_line_says_nothing_when_the_end_was_as_asked`.
`slice-010.md` AC-1 is brought into line.

**Outcome:** verified

### F-2 — `canon-delta.md`'s replacement text attributes SPEC-003/R-4's failure to R-3's in-use case, and gives a false reason for it being unreachable

**Severity:** blocker
**Location:** `canon-delta.md` §SPEC-003 → Change 1 → *What it will say*; the same belief in `draft-spec.md §7`, R-3's cell

**Expected:** the replacement is applied to a live spec **verbatim** at audit
(`canon-delta.md` head). It must be true of the requirement whose cell it sits
in. SPEC-003/R-4 is *a path occupied by anything that is not a socket, and any
other failure to bind or to set the mode*, and it says in its own text that a
socket a live host holds is R-4's **only** as a symlink, *"rather than as R-3's
in-use case"*. The in-use case is R-3's.

**Observed:** the replacement says the status is not held at the binary tier
*"for **this** requirement's failure: an ingress bind refused because a live host
holds the path needs that live host, and no case in that target has one."* That
is R-3's failure, not R-4's. R-4's own failures — a regular file at the path, a
directory with no write permission, a symlink at the path — need no live host at
all, and are reachable by `crates/goad/tests/binary/` headlessly: `start`
(`crates/goad/src/main.rs`) calls `startup::listener` at step 3, before
`PromptWindow::new` at step 5, so an ingress bind failure settles before the
first Slint call — the precise condition `crates/goad/tests/binary/main.rs`
names for a case being feasible at that tier. `exit_codes.rs` already owns a
`scratch_config` helper that writes a configuration file for one case.

So the conclusion (*no case in that target covers it today*) is true and the
stated reason is false, which is the worse of the two failures: a reader given a
false impossibility does not go and write the case.

`draft-spec.md §7`'s R-3 cell carries the same belief — *"The causes those cases
do **not** reach — a clock, a runtime, a bound socket, a display — are
headless-unreachable **or need a second live host**"* — where *a bound socket*
covers `StartupError::Ingress` whole, most of whose faults need neither.

**Evidence:** SPEC-003/R-4's requirement text, `docs/specs/003-host-event-ingress.md`
§4: *"including a socket a live host holds, which is a startup failure as a
symlink rather than as R-3's in-use case"*. R-3's requirement text carries the
lock-held clause. SPEC-003 §7's R-4 cell names
`ingress::a_regular_file_at_the_path_is_refused_naming_what_was_found` and
`ingress::a_directory_with_no_write_permission_is_refused_naming_the_path` as
R-4's cases — neither involves a live host. Step order in `start`:
`startup::listener(config.ingress.as_ref())?` precedes
`PromptWindow::new().map_err(StartupError::Platform)?`.

**Disposition:** doc-wrong
**Response:** the replacement text was the defect, and it was about to be
applied to a live spec verbatim. It now says the binary tier covers none of
R-4's failures **and that nothing prevents it**: `startup::listener` runs before
the first Slint call, so a regular file, an unwritable directory or a symlink at
the configured path settles headlessly exactly as that target's existing cases
do. *Uncovered, not unreachable* is said in those words, because the false
impossibility was the worse half — a reader given one does not go and write the
case.

Two consequences followed and are taken rather than left. The delta's
cross-reference paragraph no longer claims SPEC-003/R-3's *"same position"*
analogy survives intact: it survives **weakened**, since `LivenessUnknown` is a
value no cooperating test *can* produce while R-4's exit is one no case has
*yet* been written for, and narrowing R-3's cell is carried as a follow-up on
the same kill condition. `draft-spec.md §7`'s R-3 cell loses *"a bound socket …
need a second live host"* and separates `Ingress` from the causes that really
are unreachable. `design.md §4` principle 3 stops citing R-3 and R-4 as one
position.

**The uncovered case itself is deferred, endorsed by the user**
(`design-log.md`, 2026-09-23) — not for size but for ownership: it verifies a
requirement of SPEC-003, and every acceptance criterion here is about the host's
exit taxonomy. It is a row in `slice-010.md` §Follow-ups with its price and its
kill condition, and that row is what stops the corrected sentence becoming the
next thing nothing re-reads.

**Outcome:** verified

### F-3 — the draft spec's R-4 cell misdescribes what the four binary-tier cases assert

**Severity:** blocker
**Location:** `draft-spec.md §7`, R-4's cell

**Expected:** §7's own preamble — *"Each row names the kind of verification and
what discharges it, so the claim is checkable rather than asserted"* — and the
`review-design.md` §Brief's standard, the verification table's honesty case by
case against what the cited tests can actually observe. This is the defect class
`canon-delta.md` exists to repair in SPEC-003: a verification cell claiming a
mechanism that is not there.

**Observed:** the cell states *"each R-3 case above asserts the exact line on
**standard error** and that standard output is empty"*, and later that *"*last
line the process writes* is held for the headless cases by those cases reading
the whole stream"*. Of the four cases R-3's cell names
(`crates/goad/tests/binary/exit_codes.rs`):

- `too_many_arguments_exits_2_and_says_who_spoke` — asserts the whole stream
  equals the exact line, and asserts stdout empty. The claim holds here alone.
- `no_argument_and_no_configuration_home_exits_2` — asserts the whole stream
  equals the exact line; asserts **nothing** about stdout.
- `an_unreadable_configuration_exits_2_and_says_only_what_its_own_arm_says` —
  asserts `stderr.starts_with("goad: /nonexistent/wat.toml could not be read: ")`.
  Not the exact line, not the whole stream, nothing about stdout.
- `an_unparseable_configuration_exits_2_and_says_only_what_its_own_arm_says` —
  asserts `stderr.starts_with(&format!("goad: {}: ", path.display()))`. Same
  three gaps.

Two of four assert a prefix rather than a line, three of four are silent about
standard output, and the *last line* clause is held by two, not by "the headless
cases". The two prefix cases are deliberately so — their doc comments explain
that the fault text past the prefix is the OS's and `toml`'s — which makes this
a defect in the cell's description and not in the cases.

**Evidence:** the four case bodies in `crates/goad/tests/binary/exit_codes.rs`,
by name above. `too_many_arguments_…` is the only one carrying
`assert!(output.stdout.is_empty(), …)`.

**Disposition:** doc-wrong
**Response:** the cell was the defect; the cases are right as they stand,
and AC-5 forbids touching them, so the row was rewritten to describe what is
actually there. It now takes the five cases one at a time: `too_many_arguments_…`
holds the whole claim and is the one that pins the `goad: ` prefix on a real
process; `no_argument_…` holds the entire stream and says nothing about standard
output; the two `…says_only_what_its_own_arm_says` cases hold a **prefix** and no
more, deliberately, because the text past it is the operating system's and
`toml`'s; `help_…` holds the converse. The *last line the process writes* clause
is now attributed to the two whole-stream cases and to no other headless case.

Naming them one at a time rather than as a set is the class fix: a row that
quantifies over four cases is a row that goes stale when one of them changes, and
that is how the sentence this slice is repairing in SPEC-003 came to be
there.

**Outcome:** verified

### F-4 — uniqueness claims are stated at a scope where they are false, and one of them is held by the gate rather than by review

**Severity:** major
**Location:** `draft-spec.md §7` R-1's cell and R-2's cell; `design.md §5.5` (the `Ended::StoppedRunning` invariant, and A1)

**Expected:** `CLAUDE.md` §Working here — cite by symbol, and a claim a reader
can check. A uniqueness claim is only checkable if it names the tree it
quantifies over; the repository already states this one correctly in two places.

**Observed:** R-1's cell says *"`slint::quit_event_loop` has exactly one call
site **in this workspace**"*. There are call sites in
`crates/goad/tests/event_loop/closing.rs`,
`crates/goad/tests/event_loop_schedule/scheduling.rs`,
`crates/goad/tests/event_loop_drain/drain.rs`,
`crates/goad/tests/event_loop_answer/answer.rs` and their siblings — a reader
who checks the sentence as written finds it false on the first grep. The true
claim is *in `crates/goad/src`*.

The same cell attributes it to **review**. It is held by a test the phase gate
runs: `structure::quit_event_loop_has_exactly_one_call_site`
(`crates/goad-boundary/tests/checks/structure.rs`), over
`SUBJECT_DIR = "crates/goad/src"`. So the cell overstates the scope and
understates the verification at once, in the one table whose job is to be exact
about which is which.

R-2's cell and `design.md §5.5` repeat the unqualified form:
*"`slint::run_event_loop_until_quit` has exactly one call site, in `start`"*, and
*"the `Err` arm of **the workspace's only** `run_event_loop_until_quit` call"* —
the second flatly false, with call sites in the same `event_loop_*` targets.
Unlike `quit_event_loop`, that one has **no** gate instrument, in `src` or
anywhere, which is a real difference between the two claims that the wording
currently hides.

**Evidence:** `grep -rn "quit_event_loop" crates/ --include=*.rs` and the same
for `run_event_loop_until_quit`. `SUBJECT_DIR` and
`quit_event_loop_has_exactly_one_call_site` in
`crates/goad-boundary/tests/checks/structure.rs`. `main.rs`'s own comment at the
call site gets the scope right: *"The crate's ONLY `quit_event_loop` call
site"*.

**Disposition:** fix-now
**Response:** both halves taken. **Scope:** R-1's cell says `crates/goad/src`
and says why — the `event_loop_*` targets each have a call of their own — and
`design.md §5.5`'s invariant and A1 are corrected the same way, since the
unqualified form was the class and the cells were two instances of it.
**Attribution:** R-1's cell now separates *what the gate holds* from *what
review holds*, citing `structure::quit_event_loop_has_exactly_one_call_site`
(`crates/goad-boundary/tests/checks/structure.rs`) for the first. R-2's cell
says plainly that `run_event_loop_until_quit`'s uniqueness has **no** instrument,
which is the real difference between the two claims and was the thing the old
wording hid.

**And the gap it exposed is closed, endorsed by the user** (`design-log.md`,
2026-09-23). The offered uniqueness scan was checked before being recommended
and rejected on evidence: a re-filing keeps the call-site count at one, so it
would not catch §8 R2's regression at all. What lands instead is
`structure::the_loop_s_ending_is_never_a_startup_failure` — no line of
`crates/goad/src` names both `run_event_loop_until_quit` and `StartupError`,
which is the spelling the defect has today and the one it would return in. The
line scan's limits (a multi-line or aliased re-filing) are named in the case, in
`design.md §5.2` and in the draft spec's R-2 cell rather than discovered later,
and the control is a compiled fixture beside `a_real_call_site_is_counted`. §8
R2 stops being a residue the gate does not hold; what is left is the scan's own
limit. POL-001 is not amended — its §Verification enumerates the four ADR-001
instruments and the domain-vocabulary scan, and this file's cases are none of
them — and `slice-010.md` §Scope and §Governing canon carry the third
target.

**Outcome:** verified

### F-5 — the design moves the exit constants to the renderer tier and does not reconcile the two module docs that state the opposite cut

**Severity:** major
**Location:** `design.md §5.2` (the `u8` rationale), `design.md §9`, `design.md §10`; `draft-spec.md §7` R-5's cell

**Expected:** a design that changes a stated rule says so, and names every doc
the change falsifies. `design.md §5.2` does this for `startup.rs`, listing each
doc-comment repair as *"a claim the type currently falsifies"*. The same
standard applies to the two-tier cut, which is not an incidental comment but a
rule two module docs state and a prior ledger settled.

**Observed:** the cut today is that the renderer tier holds the **arms** and the
binary tier holds the **constant**:

- `crates/goad/tests/renderer/startup.rs` module doc: *"No test here runs the
  binary or asserts an exit code (§9's own rule) … That covers the **arms**; it
  does not cover the **constant** either arm names, which `nix/module.nix`
  depends on by value."*
- `crates/goad/tests/binary/exit_codes.rs` module doc: *"What it cannot see is
  the constant each arm names, because no pure test runs a process. That is the
  cut between the two tiers and the reason these cases are here and not there."*

The design puts `exit_status::asked_to_stop_is_0`,
`a_platform_error_after_the_loop_started_is_1` and
`every_startup_failure_is_2` into `crates/goad/tests/renderer/startup.rs`, all
asserting the numerals. That is a change to the cut — a defensible one, and
§5.2's `u8`-over-`ExitCode` rationale depends on it — but the design nowhere
says the rule has moved. `exit_codes.rs`'s module doc is on the change list
(AC-5's one permitted edit); `renderer/startup.rs`'s is on no list in §5.2, §9
or §10, and §9's table says only *"same file"*.

The draft spec's R-5 cell then rests the whole requirement on those renderer
cases — *"`exit_status`'s cases answer 0, 1 and 2 … which is the requirement's
own falsifier"* — while a module doc in the tree says that tier cannot see the
constant.

**Evidence:** the two module docs quoted above, at the head of each file. The
new case names are `design.md §9`'s own table and `draft-spec.md §7`'s R-1, R-2,
R-3 and R-5 cells.

**Disposition:** fix-now
**Response:** the cut does move, and the design now says so instead of
leaving two module docs asserting the old one. `design.md §5.2` gains **The
two-tier cut moves, and both module docs say so**: the numerals were the binary
tier's while they lived in `main`; as a pure function's answers they are the
renderer tier's, and what the binary tier holds — that the **process** answers
them to a caller — is still the half no pure test can reach, which is why that
target still exists. `crates/goad/tests/renderer/startup.rs`'s module doc joins
§10's surfaces and `slice-010.md` §Scope; `exit_codes.rs`'s remains AC-5's one
permitted edit. The draft spec's R-5 cell states the same cut, so the spec no
longer rests on renderer-tier assertions that a module doc in the tree calls
impossible.

**Outcome:** verified

### F-6 — the draft spec adopts SPEC-003's P-D and then discharges it in §7 instead of in the clause

**Severity:** major
**Location:** `draft-spec.md §2` (the closing paragraph), R-1, R-7

**Expected:** SPEC-003 P-D: *"Where the mechanism has an exception, the **clause**
names it and bounds it; a clause that cannot say where its exception would be is
a clause that has not been checked."* The draft binds itself to this: *"The
criterion **SPEC-003 P-D** states for absolute clauses binds this document
too."*

**Observed:** the sentence that adopts it also substitutes for it — *"§7's cells
for R-1 and R-2 are where that criterion is met rather than asserted."* A §7 cell
says what a test holds; P-D is about what the normative clause says. A reader of
R-1 alone is told an unqualified *if, and only if*, and the exception — `serve`
returning `controller::Ending::Closed` reaches 0 with nobody having asked — is
two sections away in a verification cell.

R-7 is the sharper instance, and §2 concedes it by naming only R-1 and R-2:
*"The host MUST NOT exit with a status this document does not define."* A panic
exits 101, which the document does not define. The exemption is in §5's *An end
this document does not assign a status to* and in R-7's own cell (*"the process
does not choose those numbers"*), and nowhere in R-7. Under the criterion the
document adopts, R-7 is an absolute clause that does not name its own exception.

**Evidence:** `docs/specs/003-host-event-ingress.md` §3 P-D, including *"Amending
this spec means meeting the criterion for any absolute clause added, and a clause
that cannot meet it is not ready to be written down."* `draft-spec.md` R-1, R-7,
§5's abnormal-end paragraph, §7's R-1 and R-7 cells.

**Disposition:** doc-wrong
**Response:** the two rules are separated in §2. P-D binds the **clause**
and is now said to be met in §4; §7's rule is about a clause no cooperating test
reaches and is met in R-1's and R-2's rows. The paragraph says a clause may need
both, one or neither, and that discharging the first in the second meets
neither — which is what the old sentence did.

R-7 carries its own exception now: *save for an end it does not choose — a
signal, or a panic in its own runtime — whose number the runtime picks and §5
accounts for*. R-1 gains none, and the cell says why: `Ending::Closed` reaching
0 would be a **violation** of R-1, not an exception to it, so naming it in the
clause would be writing a permission the document does not mean to give. That
distinction is the part worth having — it was invisible while both sat under one
sentence.

**Outcome:** verified

### F-7 — two enumerations of startup causes read as exhaustive and omit `EventLoop` and `Enqueue`

**Severity:** minor
**Location:** `draft-spec.md §6` (the status-2 row, *what happened* column), `draft-spec.md §7` R-3's cell

**Expected:** `CLAUDE.md` §Working here — name the members, or name the rule they
share. An enumeration offered as the set is read as the set.

**Observed:** §6's status-2 row says *"Every failure before the event loop is
here: an argument it cannot use, a configuration it cannot find, read or parse,
a clock it cannot read, a runtime it cannot build, a socket its configuration
named that it cannot bind, a display it cannot open."* Two `StartupError`
variants are absent: `EventLoop` (*the event loop would not accept the host
task*) and `Enqueue` (*the first request could not be enqueued*). §7's R-3 cell
names the uncovered causes as *"a clock, a runtime, a bound socket, a display"*
and omits the same two.

Both are true of the rule R-3 states — *whatever its cause* — so nothing
normative is wrong; what is wrong is that two lists presented as the causes are
not, in a document whose §7 turns on which causes a test reaches.

**Evidence:** `StartupError`'s variants, `crates/goad/src/startup.rs`:
`NoConfigPath`, `Usage`, `ConfigUnreadable`, `ConfigUnparseable`, `Clock`,
`Runtime`, `Platform`, `EventLoop`, `Enqueue`, `Ingress`. `EventLoop` is raised
from `slint::spawn_local(…).map_err(StartupError::EventLoop)?` and `Enqueue`
from `tx.try_send(…).map_err(…)`, both in `start`.

**Disposition:** doc-wrong
**Response:** both enumerations named the rule instead. §6's status-2 row
says *every failure at or before the step that begins the event loop, whatever
its cause and however many causes there come to be*, and the examples that
follow are marked as instances and not as the set — which also stops a normative
document enumerating `StartupError`'s variants, a code fact with nothing
re-reading it. §7's R-3 cell splits the uncovered causes by kind, which is the
distinction that was missing rather than a longer list: `Clock`, `Runtime`,
`Platform`, `EventLoop` and `Enqueue` are unreachable from a test; `Ingress` is
merely uncovered, and F-2's follow-up is cited there.

**Outcome:** verified

### F-8 — R-1's cell claims the `exit_status` cases are "the whole of what `exit::status` can see"; R-5's cell has the accurate word

**Severity:** minor
**Location:** `draft-spec.md §7`, R-1's cell

**Expected:** the *only if* half of R-1 is the load-bearing half, and the
sentence that discharges it should be exactly as strong as the cases are.

**Observed:** R-1's cell says *"`exit::status` answers 0 for
`Ended::AskedToStop` and for nothing else, and the cases in `exit_status` are
the whole of what it can see."* `exit::status` takes
`&Result<Ended, StartupError>`, so what it can see includes every `StartupError`
variant; the cases are three, and R-3's own cell says the third *"names
representative variants rather than counting them"*. The claim is true of the
**shapes** and false of the values. R-5's cell says exactly that — *"the shapes
`exit::status` can see"* — so the accurate phrasing already exists one row
below.

**Evidence:** `design.md §5.2`'s `status` signature and its three arms;
`draft-spec.md §7` R-3's and R-5's cells.

**Disposition:** doc-wrong
**Response:** R-1's cell now says *the whole of the **shapes** it can see —
not of the values*, and points at R-3's row for how the `Err` half is held.
R-5's wording was already right and is untouched. `slice-010.md` AC-4 carried
the same overclaim — *every value that function can see* — and is corrected with
it, since leaving the card saying one thing and the spec another is how the two
drift.

**Outcome:** verified

### F-9 — two doc sites this slice falsifies are on no change list

**Severity:** minor
**Location:** `design.md §5.2` (`crates/goad/src/diagnostics.rs`), `design.md §10` (*Beyond `slice-010.md` §Scope*)

**Expected:** §5.2 sets the standard for `startup.rs`, enumerating each doc
repair as *"a claim the type currently falsifies"*. The same sweep is owed
wherever this slice falsifies one.

**Observed:**

- `crates/goad/src/lib.rs`'s header keeps a running module count — *"ten at
  PHASE-08, nine after 005 lifted `clock` … and ten again with `draft` … Slice
  009 adds two more"*. §10 names the `pub mod` line `exit.rs` needs and not the
  count above it, which the same edit makes stale. This is the *never count*
  rot in the file the slice is editing.
- `crates/goad/src/diagnostics.rs`'s module doc names the outlet being removed:
  *"PHASE-08 adds the startup surface's own two outlets, `USAGE`, `print_usage`
  and `report_startup`"*. §5.2 removes `report_startup` and does not name the
  module doc that lists it.

**Evidence:** the header comment of `crates/goad/src/lib.rs`; the `//!` block at
the head of `crates/goad/src/diagnostics.rs`; `design.md §5.2`'s `diagnostics.rs`
subsection and §10's *Beyond §Scope* paragraph.

**Disposition:** fix-now
**Response:** both are on the change list. `diagnostics.rs`'s module doc is
named in §5.2 beside the removal that falsifies it. `lib.rs`'s header is in
§10's surfaces, and the instruction is to **replace the running count with the
rule it was counting**, not to increment it — a count nothing re-reads is the rot
`CLAUDE.md` names, and incrementing it in the very file this slice edits would
be adding to it.

**Outcome:** verified

### F-10 — `design.md §10` counts the canon-delta's replacement as two sentences; `canon-delta.md` replaces one

**Severity:** nit
**Location:** `design.md §10` (the SPEC-003 §7 row), `design.md §7 D2`

**Expected:** two artefacts in one slice describing the same edit describe it the
same way, and a count is not used where a name will do.

**Observed:** §10's row reads *"two sentences replaced: the stale *no test target
links the binary*, and the one recording the conflation as a fact"*.
`canon-delta.md` says *"One sentence in the cell"* carrying *"two defects of
different kinds"*, and quotes it: a single sentence with two clauses. D2's
consequence line carries a second incidental count — *"beside the two it already
renders"* — of `diagnostics.rs`'s stderr sentences, which is stale at the third.

**Evidence:** the quoted block in `canon-delta.md` §Change 1 *What is wrong*;
`docs/specs/003-host-event-ingress.md` §7, R-4's cell, where the text is one
sentence.

**Disposition:** doc-wrong
**Response:** §10's row says *one sentence replaced, carrying two
defects*, which is what `canon-delta.md` quotes and what SPEC-003 §7 contains.
D2's consequence line says *beside the ones it already renders*.

**Outcome:** verified

### F-11 — §6's *stopped running* row says the backend was reachable; nothing ever asked it

**Severity:** major
**Location:** `draft-spec.md §6`, the status-1 row, *what happened* column

*Raised after round 1's repairs were integrated, while sweeping them. Same
round, same ledger — the id is appended rather than the round restarted.*

**Expected:** §6 is introduced as *"the whole of what may be inferred from
each"*, so its *what happened* column is load-bearing for a consumer: a status
may not carry a fact the process never observed. §3 P-A says the same from the
other side — the number says which of a closed set of outcomes happened, and
nothing more.

**Observed:** the row reads *"The process started — its event loop began, its
window and its backend were **reachable** — and then stopped for a reason nobody
asked for."* The window half is true: `PromptWindow::new`, `slint::set_xdg_app_id`
and `Tray::new` all ran. The backend half is not. Nothing in `start` contacts
the backend. `ProcessBackend::new(config.backend.command.clone(), config.backend.timeout)`
stores a command and a timeout and does nothing else; the first exchange is
enqueued as `Command::Evaluate(Stimulus::Startup)` and is not served until
`serve` runs, which is **after** `run_event_loop_until_quit` has begun. A host
whose configured backend command does not exist reaches exit 1 exactly as one
whose backend answered for hours does.

So a supervisor told 1 is told, by this document, that a backend was reachable —
which is a fact about the user's configuration that the process has no standing
to report, and the one inference §6 must not license by accident.

**Evidence:** `ProcessBackend::new` (`crates/goad-shell/src/backend/process.rs`)
— a two-field constructor; the work is in `Backend::exchange`. Step order in
`start` (`crates/goad/src/main.rs`): the backend is constructed at step 1, the
first evaluation is *enqueued* at step 8, and `serve` consumes it inside the
loop spawned at step 9.

**Disposition:** `doc-wrong`
**Response:** The row is the defect; the design and the code are right. §6's
*what happened* cell now reads *"The process started — it reached the display,
built its window and its tray, and its event loop began — and then stopped for a
reason nobody asked for."*

**The repair is wider than the finding asked for, in two places, and both are
the same class the finding names.**

First, the finding's own sub-claim — *"The window half is true"* — is true of
what ran and false of the word the proposed replacement reached for. *Opened*
was not adopted. `PromptWindow::new` **constructs** a window; the only
`window.show()` in the crate is in `SlintGlass::present` (`glass.rs`), it runs
inside the loop, and it runs only for `Surface::Prompt | Surface::Diagnostics`.
A tray-resident host sits at `Surface::Hidden` for hours and exits 1 having
never shown a window. Writing *its window opened* would have replaced one
unobserved fact with another — which is the defect this finding is about, and
exactly the reason the repair is the place the next false claim gets written.
What the process has actually observed by the time it can answer 1 is that it
reached the display, that its components were built, and that its loop began.

Second, the adjacent *what a reader may infer* cell said *"The host was
**working**"*. Same class, one column over: *working* is unobserved for the same
reason *backend reachable* is — a host whose configured backend command does not
exist runs and exits 1 identically. It now reads *"The host was **running**"*,
which is also the vocabulary the host's own stderr line already uses
(`design.md`'s line table: `goad: the host was running and stopped: {error}`),
so the row and the line now say the same thing.

**Checked and clean:** no other site carries the claim. `reachable` appears
nowhere else in the slice's documents in this sense, and the R-6 requirement,
the line table and `stderr_outlets::a_host_that_stopped_running_says_it_had_been_running`
all say *running* already. The repair is one row.

**Outcome:** `verified`

### F-12 — *stopped running* asserts the loop began; the value it is derived from does not say that

**Severity:** major
**Location:** `draft-spec.md §4` R-2 and R-6; `draft-spec.md §6`, the status-1 row, both columns; `design.md §5.5` A1

**Expected:** §6 is introduced as *"the whole of what may be inferred from each"*
and §3 P-A says a status carries which outcome happened and no more. F-11 settled
the standard in terms: a cell may say only what the process watched happen. R-2
and R-6 are MUSTs and are held to it at least as hard, because §5 rests the whole
axis on phase being *"a fact it observes and cannot get wrong"*.

**Observed:** the process derives *stopped running* from one thing —
`slint::run_event_loop_until_quit()` answering `Err` — and that answer does not
establish that the loop began. In the version this workspace pins, the call
reaches `EventLoopState::run`, whose first act is to take the not-running loop
instance and, failing that, answer
`PlatformError::from("Nested event loops are not supported")` **before**
`run_app_on_demand` is called at all; and `run_app_on_demand` can itself fail at
entry, wrapped as `Error running winit event loop: {e}`. In either shape the loop
never begins, the design files the error as `Ended::StoppedRunning`,
`exit::status` answers 1, and the host writes *the host was running and stopped*.

Four sites state the unobserved fact. R-2: *"its event loop began and then ended
for a reason nobody requested"*. R-6: the line *"MUST say that the host had been
running"*. §6's status-1 *what happened*: *"…and its event loop began…"*. §6's
status-1 *what a reader may infer*: *"The host was running and is now gone."*
Each is inferred from an `Err` that a host whose loop never started produces too.
The direction is the reverse of the defect this slice repairs, and the
conflation is the same one: a phase read off a channel that does not carry it.

The `Ok` half has the mirror gap, and only part of it is declared. A1 is headed
*"The loop returning `Ok` means someone asked"*, and its body argues entirely
about where `quit_event_loop` is called from — which establishes *quit ⇒ the loop
returns*, not the converse R-1's *if, and only if* needs. Whether
`run_event_loop_until_quit` can answer `Ok` for a reason nobody asked for is not
addressed at A1, at R-1's cell, or anywhere else; both name `Ending::Closed` and
stop.

**Evidence:** `run_event_loop_until_quit` (`slint-1.17.1/lib.rs:265`) delegates to
`Platform::run_event_loop`; the winit backend's implementation
(`i-slint-backend-winit-1.17.1/lib.rs:796`) calls `EventLoopState::run`, whose
body (`i-slint-backend-winit-1.17.1/event_loop.rs:689-717`) carries
`.ok_or_else(|| PlatformError::from("Nested event loops are not supported"))?`
ahead of
`run_app_on_demand(&mut self).map_err(|e| format!("Error running winit event loop: {e}"))?`.
Vendored citations, pinned: `Cargo.lock` names `slint` and
`i-slint-backend-winit` at 1.17.1, the same version `design.md §2` cites
`i-slint-core` at. `start` (`crates/goad/src/main.rs`) calls the function once, so
the nested-loop arm is not a production path today — which is an argument about
the current code, not a property of the value the classifier reads, and the
requirement is written about the value.

No replacement wording is proposed here. What the process has observed when it
can answer 1 is that it reached the display, built its components, and that the
loop call answered an error rather than a requested stop; whether R-2 narrows to
that or keeps its sentence with an exception is the responder's, and either way
the proposal needs checking against the same standard as the defect.

**Disposition:** `doc-wrong`
**Response:** Verified at the pinned source before disposing: `EventLoopState::run`
(`i-slint-backend-winit-1.17.1/event_loop.rs`) answers
`PlatformError::from("Nested event loops are not supported")` ahead of
`run_app_on_demand`, and `run_event_loop_until_quit` (`slint-1.17.1/lib.rs`)
runs through `with_platform`, which can fail to select a platform at all. The
finding is right, and it is the slice's own defect pointing the other way.

**The user chose to narrow what 1 claims rather than bound an exception**
(`design-log.md`, 2026-09-23). The repair cuts R-2 and R-3 at **the call**
rather than at the loop, because reaching the call is the last thing the process
can observe without asking the loop about itself:

- R-2 — *"reached the call that runs its event loop and that call ended for a
  reason nobody requested"*, with the seam named in its own sentence.
- R-3 — *"every failure **before** it reached the call"*, which is also F-14's
  repair; the two findings share one edit and it removes the overlap rather than
  moving it.
- §5's diagram transitions, §6's status-1 row, and §5's *the cut is phase, not
  retryability* paragraph, which claimed phase is *"a fact it observes and
  cannot get wrong"*. It now claims the thing that is actually true and is the
  better argument: the axis has **one** imprecision, it is at a named seam, it
  is declared, and it does not grow when a new cause arrives.
- §5 gains *What the seam costs*, which states the residue and closes with a
  normative *a consumer MUST NOT read 1 as evidence that the host did any work*.
- R-6 keeps its sentence and carries its exception **in that sentence**, which
  is F-13's lesson applied rather than restated: where the call fails on entry
  the line still says the host had been running, because the host has nothing by
  which to know otherwise.

**The `Ok` half is repaired as the finding describes and not further.** A1 now
argues both directions and names what makes the converse hold:
`run_event_loop_until_quit` sets `set_event_loop_quit_on_last_window_closed(false)`,
so the loop does not end of its own accord when the last window closes — which
for a tray-resident host that hides its window is exactly the shape that would
otherwise return `Ok` with nobody having asked. It is stated as a property of
the pinned version, not of the API's contract.

A new assumption **A4** carries the `Err` half. AC-6 and the line text are
unchanged: the common case — a display lost after hours — is what they describe,
and the rare entry failure is the declared residue rather than a rewrite of the
host's vocabulary.

**Outcome:** `verified`

### F-13 — §2 says no requirement but R-7 has an exception to carry; R-4 has one, and it is stated in §5

**Severity:** major
**Location:** `draft-spec.md §2` (the paragraph adopting SPEC-003 P-D), R-4

**Expected:** F-6's repair adopted SPEC-003 P-D for the **clause**: *"a clause
here that says a mechanism always holds or never fails names its exception and
bounds it, or it is not ready to be written down"*, and the paragraph beneath it
says that a document which discharges that rule in §7's rule *"has met neither"*.

**Observed:** the adopting paragraph closes *"That is a rule about the **clause**,
and it is met in §4 — R-7 carries its own exception in its own sentence, and no
other requirement here has one to carry."* R-4 has one. R-4 reads *"**Every**
non-zero exit MUST be accompanied by a line on standard error naming the binary
that wrote it and what happened"*, and §5's *An end this document does not assign
a status to* names the exception out loud: *"A process killed by a signal, or
ended by a panic in its own runtime, carries neither a status this document
defines **nor the line R-4 requires**."*

So the document knows R-4's exception, states it two sections away from the
clause, and §2 asserts there is none. That is the shape F-6 found, re-committed
inside F-6's own repair: the exception lives outside the clause, and the sentence
written to close the pattern is what licenses this instance of it.

**Evidence:** `draft-spec.md §2`, closing paragraphs; R-4's text in §4; §5's
abnormal-end paragraph, which cites R-4 by id. `docs/specs/003-host-event-ingress.md`
§3 P-D for the criterion, including *"a clause that cannot meet it is not ready
to be written down"*.

**Disposition:** `doc-wrong`
**Response:** §2 asserted a universal the document does not hold, in the sentence written to
close F-6's pattern. Two edits, and the second is the one that matters:

- R-4 carries its exception in its own clause — *"Every non-zero exit **this
  document assigns**…"*, with the signal-or-panic end named there and bounded by
  §5.
- §2 stops quantifying. It now says the rule is met *wherever a clause has an
  exception to carry*, and names R-7's and R-4's rather than asserting that no
  other exists. A universal about the requirement set is what went wrong here;
  it is not replaced with a corrected count.

**Outcome:** `verified`

### F-14 — R-3 admits the loop's own failure into *never started*, which R-2 and §5 put in *stopped running*

**Severity:** blocker
**Location:** `draft-spec.md §4` R-3; `draft-spec.md §6`, the status-2 row

**Expected:** R-5 requires the classes to be told apart by the number alone and
says *"No two classes may share a number"*. §1 says the status *"names **the phase
the process ended in**"*. Two requirements in one normative table may not both
admit the same end, and the boundary between 1 and 2 is the one this document
exists to draw.

**Observed:** R-3 reads *"every failure **at or before** the step that begins the
event loop, whatever its cause"*. The step that begins the event loop is
`slint::run_event_loop_until_quit`. A failure *at* that step is the whole of R-2's
class. Read as written, R-3 requires 2 for exactly the end R-2 requires 1 for, and
a second implementation held to this document has no way to decide between them.

§5's state diagram uses the narrower word and does not have the problem —
*"Invoked --> NeverStarted: a step **before** the event loop failed"*. §6's
status-2 row repeats R-3's wider form, so the ambiguity is in two of the three
places and absent from the third.

The phrase is round 1's. F-7's repair replaced *before the event loop* with *at or
before the step that begins the event loop*, to stop the enumeration omitting
`EventLoop` and `Enqueue` — both of which are raised before
`run_event_loop_until_quit` is reached and were already inside the narrower word,
so the widening bought nothing and cost the boundary.

**Evidence:** R-2, R-3, R-5 and §5's state diagram in `draft-spec.md`; §6's
status-2 *what happened* column. F-7's Response in this ledger for the phrase's
provenance. In `start` (`crates/goad/src/main.rs`),
`slint::spawn_local(…).map_err(StartupError::EventLoop)?` and
`tx.try_send(…).map_err(|_returned| StartupError::Enqueue)?` both precede
`slint::run_event_loop_until_quit()`.

**Disposition:** `doc-wrong`
**Response:** Confirmed against the table: R-3's *at or before the step that begins the event
loop* and R-2's class admit the same end, and R-5 forbids exactly that. The
phrase is F-7's repair and bought nothing — `EventLoop` and `Enqueue` are both
raised before the call in `start`, so the narrower word already covered them.

Repaired together with F-12, since one cut fixes both: R-3 is *"every failure
before it reached the call that runs its event loop"*, R-2 is *"reached the call
… and that call ended"*, and the seam is *did it reach the call*, which is always
decidable and leaves no end in both classes. §6's status-2 row takes the same
wording. §5's diagram, which had the narrower word and was right, is also
re-cut so all three sites now say the same thing rather than two of three.

**Outcome:** `verified`

### F-15 — the canon-delta's replacement credits a named case with a universal the draft spec says that case does not hold

**Severity:** blocker
**Location:** `canon-delta.md` §SPEC-003 → Change 1 → *What it will say*

**Expected:** the block is applied to a live SPEC-003 **verbatim** at audit
(`canon-delta.md` head), into the one cell whose defect was recording a mechanism
that was not there. A verification cell may credit a case with what that case
asserts and no more — F-3's standard, and the reason Change 1 exists at all.

**Observed:** the replacement says *"What holds the status meanwhile is
`exit::status` (`crates/goad/src/exit.rs`), whose single `Err` arm answers one
number for **every** `StartupError` variant and reads none of them, **asserted by**
`exit_status::every_startup_failure_is_2` (`crates/goad/tests/renderer/startup.rs`)"*.

The draft spec's own R-3 cell says the opposite about that case: *"the *whatever
its cause* clause is held **structurally** rather than by that enumeration:
`exit::status` answers 2 from a single arm over `Err(_)` that reads no
`StartupError` variant … `exit_status::every_startup_failure_is_2` names
representative variants rather than counting them and asserts the number for
each."* The case asserts a sample. The universal is held by the arm's shape, which
no test reads.

So the sentence about to enter canon attributes to a named test a claim the
slice's other document explicitly denies it holds — and the accurate wording
already exists one document over, which is the shape F-8 found inside the draft
spec and this is inside the delta.

**Evidence:** `canon-delta.md` Change 1's replacement block; `draft-spec.md §7`'s
R-3 cell; `design.md §9`'s row *"`exit::status` answers 2 for a startup failure,
named variants rather than counted"*. `crates/goad/tests/renderer/startup.rs` has
no `exit_status` module today, so the case's content is what the design and the
draft spec say it will be and nothing else.

**Disposition:** `doc-wrong`
**Response:** The delta's block goes into SPEC-003 verbatim, and it credited a named case with
a universal the draft spec says that case does not hold. The accurate wording
existed one document over; the replacement now mirrors it and puts the
attribution where it belongs:

> What holds the status meanwhile is the **shape** of `exit::status` … a single
> `Err` arm that reads no `StartupError` variant … **No test holds that
> universal.** What `exit_status::every_startup_failure_is_2` holds is a sample
> — it names representative variants rather than counting them…

The *no test holds that universal* sentence is added rather than inherited: the
draft spec's R-3 cell says the universal is held structurally, but it does not
say in terms that no test reaches it, and the cell this lands in is one whose
original defect was recording a mechanism that was not there.

**Outcome:** `verified`

### F-16 — the new gate case's stated limit is narrower than its actual limit: `?` through a `From` impl is a one-line, unaliased miss

**Severity:** major
**Location:** `design.md §5.2` (*What the case does not reach*), `design.md §8` R2, `draft-spec.md §7` R-2's cell

**Expected:** all three sites name the scan's limit *at the point it is
introduced rather than discovered later* (§5.2's own words), and R-2's cell says
the limit is stated *"because a line scan's limit is the whole of what it costs"*.
A stated limit that is not the limit is worse than none: it is what a reader
stops looking past.

**Observed:** all three give the same two — *"a re-filing spread over more than
one line, or one routed through an alias"*. There is a third, and it is neither.
`StartupError` carries no `From` impl today; adding
`impl From<slint::PlatformError> for StartupError` — an ordinary convenience —
makes

```rust
  slint::run_event_loop_until_quit()?;
```

compile inside a function answering `Result<_, StartupError>`, re-file the loop's
ending as a startup failure, and name `StartupError` on no line at all. One line,
no alias, no rename-import. `the_loop_s_ending_is_never_a_startup_failure` stays
green, and so does everything else — which is the property §8 R2 says the case
exists to remove.

It is a likelier route than either limit that is named, because `?` is the idiom
`start` is built on: §5.2's own argument for the `Result` shape is that *"Every
`?` in `start` keeps working"*, and OQ-1 rejects the flat enum precisely because
it *"takes `?` away from `start`"*. The mutation §9 asks the plan to confirm is
caught — `map_err(StartupError::Platform)` restored — is the spelled form, so
nothing in the slice's own validation would find this one either.

**Evidence:** `crates/goad/src/startup.rs` declares no `impl From<…> for
StartupError` (the variants are constructed at their call sites in `start`,
`crates/goad/src/main.rs`). The matcher the case is specified to use —
`occurrences_where` over `production_lines`
(`crates/goad-boundary/tests/checks/structure.rs`) — applies a predicate to one
`code_of`-stripped line at a time, so a needle absent from the text is absent
from the scan. `design.md §9`'s mutation list.

**Disposition:** `fix-now`
**Response:** The route is real and was verified before disposing: `StartupError` declares no
`From` impl (`crates/goad/src/startup.rs`), and `occurrences_where` applies its
predicate to one `code_of`-stripped line at a time
(`crates/goad-boundary/tests/checks/structure.rs`), so a needle absent from the
text is absent from the scan.

**The user chose to close it rather than document it** (`design-log.md`,
2026-09-23), and the reason is the finding's own: a stated limit is what a reader
stops looking past, and this route is likelier than either limit that was named,
because `?` is the idiom OQ-1 chose the `Result` shape to preserve.

`the_loop_s_ending_is_never_a_startup_failure` now reds on either spelling one
line admits — a line naming both `run_event_loop_until_quit` and `StartupError`,
and a line applying `?` directly to the call (`run_event_loop_until_quit()?`).
The second needle is precise rather than a ban on `?`: the arm the design
specifies wraps the call before the `?`, so the passing form does not contain
it. Each needle gets its own compiled fixture, per the existing rule that an
uncompiled control greps the same as a passing one. `design.md` §5.2, `design.md`
§8 R2 and the draft spec's R-2 cell all state both needles; the two documented
limits — multi-line, and alias — are unchanged and still stated, because they
are still real.

**Outcome:** `verified`

### F-17 — three surfaces are described as beyond `slice-010.md` §Scope; §Scope names all three

**Severity:** minor
**Location:** `design.md §5.1` (the `exit.rs` paragraph), `design.md §5.2` (the `structure.rs` paragraph), `design.md §10` (*Beyond `slice-010.md` §Scope*)

**Expected:** §10's *Beyond §Scope* list is what a reader reads to find what still
needs the user's endorsement. It works only if everything on it is actually
outside §Scope; an entry that is not camouflages the one that is.

**Observed:** round 1's repairs moved three surfaces into `slice-010.md` §Scope
and left the design describing them as outside it.

- §5.1: *"This is one file beyond the surfaces `slice-010.md` §Scope names"* —
  §Scope's Code list carries `crates/goad/src/exit.rs`, marked
  *(endorsed 2026-09-23, `design-log.md`)*.
- §5.2: *"This is a third target beyond the two `slice-010.md` §Scope names"* —
  §Scope names `crates/goad-boundary/tests/checks/structure.rs` with the case by
  name, and names `crates/goad/tests/renderer/startup.rs` as well. §Scope names
  three test targets, not two. The sentence is also a count where a name would
  do.
- §10's list carries four bullets; three of them — `exit.rs` with its `pub mod`
  line, `crates/goad/tests/renderer/startup.rs`'s module doc, and
  `structure.rs` — are in §Scope. Only `lib.rs`'s header comment is genuinely
  outside it, and it is the one a reader would now skim past.

**Evidence:** `slice-010.md` §Scope → **Code**, bullets for `crates/goad/src/exit.rs`,
`crates/goad/tests/renderer/startup.rs` and
`crates/goad-boundary/tests/checks/structure.rs`; F-4's and F-5's Responses in
this ledger, which put them there.

**Disposition:** `fix-now`
**Response:** Confirmed against `slice-010.md` §Scope, which names all three. §5.1 and §5.2
now cite §Scope and the endorsement instead of claiming to exceed it, and §10's
list says in terms that the three were endorsed **into** §Scope and states what
remains: `lib.rs`'s header comment, which is the one thing genuinely outside it
and the reason the list exists. The three consequences of in-scope surfaces are
pointed at §5.1 and §5.2 rather than re-listed, so nothing is lost and nothing
is camouflaged.

§5.2's *"a third target beyond the two §Scope names"* was also a count, as the
finding notes. It is gone rather than corrected to three.

**Outcome:** `verified`

### F-18 — §7 names its cases in the present indicative; most of them do not exist, and nothing says which

**Severity:** minor
**Location:** `draft-spec.md §7` (the preamble and every cell), `draft-spec.md §9` (the SPEC-003 bullet)

**Expected:** §7's preamble is *"Each row names the kind of verification and what
discharges it, **so the claim is checkable** rather than asserted."* The document
is promoted into `docs/specs/` at audit and is thereafter the thing nothing
re-reads. A reader — including whoever promotes it — needs to be able to tell a
citation that resolves today from one this slice still owes.

**Observed:** the cells read as statements about the tree, and most of the cases
they name are not in it. Absent: `exit_status::as_asked_is_0`,
`exit_status::a_platform_error_after_the_loop_started_is_1`,
`exit_status::every_startup_failure_is_2`,
`stderr_outlets::report_exit_line_says_nothing_when_the_end_was_as_asked`,
`stderr_outlets::report_exit_line_for_a_startup_failure_is_the_startup_line`,
`stderr_outlets::a_host_that_stopped_running_says_it_had_been_running`,
`stderr_outlets::the_stopped_line_is_not_the_line_a_host_that_never_started_writes`,
and `structure::the_loop_s_ending_is_never_a_startup_failure`. Present: the four
`exit_codes::` failure cases, `help_prints_the_usage_block_on_stdout_and_exits_0`,
and `structure::quit_event_loop_has_exactly_one_call_site`.

R-3's cell mixes the two under one heading — *"binary tier, every case it can
reach headlessly, **unchanged by this document's arrival**"* introduces four cases
that exist, and the same cell closes on `exit_status::every_startup_failure_is_2`,
which does not. §9's SPEC-003 bullet has the same shape one step out: *"R-4's cell
also defers the *meaning* of the status it mentions to this document"* is true
only once `canon-delta.md` Change 1 has been applied, and reads as a fact about
SPEC-003 as it stands.

`design.md §9` carries the rule that closes this — *"**Test names are commitments.**
The draft spec's §7 cites them, and the draft is kept current as the shape of the
work changes"* — but `design.md` is not promoted and nothing outside the slice
folder may cite it, so the obligation travels only as long as someone is reading
both files.

**Evidence:** `crates/goad/tests/renderer/startup.rs`'s modules are
`display_text`, `source_walk`, `usage_block`, `stderr_outlets`,
`usage_error_does_not_reprint_the_block`, `arguments_table` and `listener`; there
is no `exit_status` module and no `report_exit_line` case in `stderr_outlets`.
`crates/goad-boundary/tests/checks/structure.rs` has no
`the_loop_s_ending_is_never_a_startup_failure`. `crates/goad/src/exit.rs` does not
exist. `git status --short` at `8c1fabb` shows `docs/slices/010/` as the only
change in the tree.

**Disposition:** `doc-wrong`
**Response:** §7's preamble now says that while the document is a draft its rows are written
in the present indicative and some of the cases they name do not exist yet, and
makes the consequence normative and mechanical rather than a matter of care:
**the document MUST NOT be promoted into `docs/specs/` until every case named in
§7 resolves in the tree.** That is the same mechanism the delta already uses for
its `SPEC-00N` placeholder — an unfinished promotion is detectable — and unlike
`design.md §9`'s commitment rule it travels with the promoted document, which is
the half the finding identified as missing.

The cases are not marked individually. A per-row marker is a second thing to
keep true, and the promotion gate makes every row's tense correct at the moment
it stops being a draft.

**Outcome:** `verified`

### F-19 — R-4's cell opens with a count of the cases R-3 names, inside the repair that stopped quantifying over them

**Severity:** minor
**Location:** `draft-spec.md §7`, R-4's cell (first clause)

**Expected:** `CLAUDE.md` §Working here — *never count*, exempting only *"a count
of something that cannot grow"*. F-3's own Response states the rule for this cell:
*"a row that quantifies over four cases is a row that goes stale when one of them
changes, and that is how the sentence this slice is repairing in SPEC-003 came to
be there."*

**Observed:** the repaired cell opens *"binary tier, where the **four cases** R-3
names do **not** all hold the same thing, so this row takes them one at a time
rather than as a set."* The body does take them one at a time; the opening clause
still quantifies. The list can grow: `slice-010.md` §Follow-ups carries a row
whose kill condition is a fifth case landing in
`crates/goad/tests/binary/exit_codes.rs` and being named by SPEC-003/R-4's cell,
and R-3's cell is where it would be named. The count is stale on the commit that
closes the slice's own follow-up.

**Evidence:** `draft-spec.md §7` R-4's cell; R-3's cell, which names four cases;
`slice-010.md` §Follow-ups, third entry, *Dead when* clause. F-3's Response in
this ledger.

**Disposition:** `fix-now`
**Response:** *"the four cases R-3 names"* → *"the cases R-3 names"*. The body already took
them one at a time; only the opening clause quantified. The list can grow —
`slice-010.md` §Follow-ups has a row whose kill condition is a fifth case — so
this is not the closed-list exemption.

**Outcome:** `verified`

### F-20 — `structure.rs` has no `tests` module; the control the design sites its fixture in is called something else

**Severity:** minor
**Location:** `design.md §5.2` (the `crates/goad-boundary/tests/checks/structure.rs` paragraph)

**Expected:** `CLAUDE.md` §Working here — cite by symbol, and a symbol that
resolves. The paragraph is the instruction a phase agent implements from.

**Observed:** *"with a fixture control in the file's own `tests` module beside
`a_real_call_site_is_counted`"*. There is no `mod tests` in that file. The module
holding `a_real_call_site_is_counted` is `mod counting_itself`, whose own doc
states what belongs in it — *"Controls on this file's own counting and its
test-scope cut"* — which is the thing the new control has to satisfy, and the
thing a reader looking for `tests` will not find.

`notes.md` §Harvest carries the same wrong name, in an entry marked checked and
clean. The entry's substance holds — `occurrences_where` does take an arbitrary
predicate and `a_real_call_site_is_counted` is a compiled fixture — so this is the
name and not the conclusion.

**Evidence:** `crates/goad-boundary/tests/checks/structure.rs`: `mod counting_itself`
contains `a_real_call_site_is_counted`; the file declares no module named `tests`.

**Disposition:** `fix-now`
**Response:** `mod counting_itself` is the module, and its doc states what belongs in it —
*"Controls on this file's own counting and its test-scope cut"* — which is what
the new control has to satisfy. `design.md` §5.2 now names it and cites that doc.

`notes.md` §Harvest is corrected too, and says the name was wrong rather than
quietly swapping it: the entry was in a *checked and clean* list, and a wrong
symbol in that list is worse than no entry, because the next stage is told not
to look. The entry's substance held — only the symbol was false.

**Outcome:** `verified`

### F-21 — §6's status-2 inference cell licenses a missing-resource reading that `Enqueue` and `Usage` do not support

**Severity:** minor
**Location:** `draft-spec.md §6`, the status-2 row, *what a reader may infer* column

**Expected:** the same standard F-11 set, applied to the column F-11 did not
reach on that row. §6 is *"the whole of what may be inferred from each"*, and the
*what a reader may infer* column is the one a consumer acts on.

**Observed:** the cell reads *"The host has not begun and did not begin this time.
**What it needs was not there or was not usable.**"* The second sentence is an
inference about the host's environment, and `StartupError` carries variants for
which nothing was missing or unusable. `Enqueue` — *the first request could not be
enqueued* — is a fault in the host's own channel; nothing external was absent.
`Usage` — two or more positional arguments — is a wrong invocation, not an absent
resource. A person or a supervisor told *what it needs was not there* goes looking
at the machine, and for those causes there is nothing there to find.

The *what happened* column on the same row is careful about this — F-7's repair
made its list *"instances of the rule, named as instances and not as the set"* —
so the row states the rule in one column and a cause-shaped inference in the next.

**Evidence:** `StartupError`'s variants (`crates/goad/src/startup.rs`), and their
`Display` text asserted in `display_text` (`crates/goad/tests/renderer/startup.rs`):
`enqueue` pins *"the first request could not be enqueued"* and `usage` pins *"too
many arguments…"*. `tx.try_send(Command::Evaluate(Stimulus::Startup)).map_err(|_returned| StartupError::Enqueue)?`
in `start` (`crates/goad/src/main.rs`).

**Disposition:** `fix-now`
**Response:** Confirmed: `Enqueue` is a fault in the host's own channel and `Usage` is a wrong
invocation; nothing external was absent in either. The cell now reads *"The host
has not begun and did not begin this time. Why is in the line R-4 requires, and
not in the number."* — which is also what P-A and R-3 already say, so the
inference column stops contradicting the requirement it serves.

**Outcome:** `verified`

### F-22 — P-C names only the host's binary-tier target as what holds the numbers, in a section that binds both binaries

**Severity:** nit
**Location:** `draft-spec.md §3` P-C

**Expected:** §3's own preamble: *"These hold of every binary this project ships,
including the one §4 does not yet govern."* A principle stated for both binaries
that illustrates with one of them should say which it is illustrating with, or the
illustration reads as the whole mechanism.

**Observed:** P-C reads *"A consumer may depend on a number by value.
`nix/module.nix` does, and `crates/goad/tests/binary/` is what holds it."* Read
under the section's preamble, that says the host's target is what holds the
numbers this principle is about. `goad-emit`'s are held by
`crates/goad-emit/tests/binary/exchange.rs`, which asserts 0, 1 and 2 against the
spawned binary. The principle's closing claim — *"something in the gate fails
first"* — is true of both; only the named holder is partial.

**Evidence:** `crates/goad-emit/tests/binary/exchange.rs` asserts `code_of(&output)`
equal to 0, 1 and 2 across its cases; `slice-010.md` §Follow-ups already names that
file as what a future §7 row for `goad-emit` would cite.

**Disposition:** `fix-now`
**Response:** P-C now names the binary tier of each binary the document owns —
`crates/goad/tests/binary/exit_codes.rs` and
`crates/goad-emit/tests/binary/exchange.rs` — and says in the same sentence that
§4 writes no requirement for the second yet. That keeps §3's stated scope honest
without admitting `goad-emit` into §4, which is a follow-up and not this
slice's.

**Outcome:** `verified`


### F-23 — the loop's call answers `Err` for an end a person asked for, so R-1 and R-2 both admit it and the host answers 1

**Severity:** blocker
**Location:** `draft-spec.md §4` R-1 and R-2; `draft-spec.md §5` (*What the seam
costs*); `draft-spec.md §6`, the status-1 row; `draft-spec.md §7`, R-1's cell;
`design.md §5.5` A1

**Expected:** round 2 re-cut R-2 and R-3 onto *the call that runs the event
loop* because that is the fact the process observes. R-5 requires the classes to
be told apart by the number alone and says *"No two classes may share a
number"*; R-1 is an *if, and only if*. Every end the host can reach is admitted
by exactly one of R-1, R-2 and R-3, or the cut is not a partition.

**Observed:** the recut moved the *first* conjunct of R-2 onto something
observable and left the *second* asserting something the process never watched.
R-2 requires 1 *"when it reached the call that runs its event loop and **that
call ended for a reason nobody requested**"*. The value the classifier reads is
`run_event_loop_until_quit()` answering `Err`, and in the pinned version that
answer does not establish that nobody requested the end.

`EventLoopState::run` latches a `loop_error` and returns it **after**
`run_app_on_demand` has returned normally. `about_to_wait` sets `loop_error`
from a failed `create_inactive_windows` and does **not** call
`event_loop.exit()`, so the error sits latched while the loop keeps running. A
person then quits: `quit_event_loop` sends `CustomEvent::Exit`, `user_event`
calls `event_loop.exit()`, `run_app_on_demand` returns `Ok`, and
`EventLoopState::run`'s closing `if let Some(error) = self.loop_error { return
Err(error); }` answers `Err`. The host reports *stopped running* and exits 1 for
an end a person asked for.

That end is admitted by R-1's **if** half — *"a running host that a person asked
to stop"* — which requires 0, and is **not** admitted by R-2, whose second
conjunct it fails. So one reachable end is claimed by one requirement, refused by
the other, and answered 1 by the design. This is the same defect F-12 found, on
the conjunct F-12 did not touch.

Three further sites inherit it:

- **`design.md §5.5` A1**, rewritten by F-12's repair to argue both directions,
  opens *"*Asked ⇒ `Ok`*: `run_event_loop_until_quit` returns `Ok` when
  `quit_event_loop` was called"*. That implication is false at the pinned
  version. A1's body argues only about where `quit_event_loop` is called from,
  which establishes nothing about the return value.
- **`draft-spec.md §7`, R-1's cell** holds the *only if* half and names what
  review holds — *"that `Ended::AsAsked` is what `start` answers when
  `run_event_loop_until_quit` returns `Ok`"*. Nothing in the cell holds the
  **if** half, and nothing can, because it is false.
- **§6's status-1 row** repeats *"that call ended for a reason nobody asked
  for"* in the column billed as what happened.

The shape is not the declared residue. §5's *What the seam costs* declares a
loop that never began being reported as *stopped running*; this is the reverse —
a loop that ran, was asked to stop, did stop, and is reported as having stopped
unasked. Nothing in the document names it.

**Evidence:** `EventLoopState::run`
(`i-slint-backend-winit-1.17.1/event_loop.rs:689-717`) — the
`run_app_on_demand(&mut self).map_err(…)?` call, then
`self.shared_backend_data.not_running_event_loop.replace(Some(winit_loop))`,
then `if let Some(error) = self.loop_error { return Err(error); }`.
`about_to_wait` (same file, `:625-627`) — `if let Err(err) =
self.shared_backend_data.create_inactive_windows(event_loop) { self.loop_error =
Some(err); }`, with no `exit()` beside it; the only `if self.loop_error.is_some()
{ event_loop.exit(); }` is at the foot of `window_event` (same file, `:541`).
`user_event`'s `CustomEvent::Exit` arm (same file, `:556-568`) calls
`event_loop.exit()`, and that is the arm `Proxy::quit_event_loop`
(`i-slint-backend-winit-1.17.1/lib.rs:841`) sends to. `Platform::run_event_loop`
(same file, `:796`) is what `run_event_loop_until_quit`
(`slint-1.17.1/lib.rs:265`) reaches through `with_platform`. Vendored citations,
pinned at 1.17.1. In `crates/goad/src/main.rs`, `start`'s
`match slint::quit_event_loop() { Ok(()) | Err(_) => () }` discards the quit's
own result, so nothing in the host records that a quit was requested either.

**Disposition:** `fix-now`
**Response:** Verified in the pinned backend before disposing, and the finding is exact:
`window_event` closes with `if self.loop_error.is_some() { event_loop.exit(); }`
while `about_to_wait` latches `loop_error` from `create_inactive_windows` and
does **not** exit; `EventLoopState::run` then checks `loop_error` after
`run_app_on_demand` returns normally. A latched error plus a requested quit
answers `Err` for an end R-1 requires 0 for.

It is F-12's defect in the other conjunct of the sentence F-12 repaired. That
sentence asserted two things the channel does not carry — *the loop began* and
*nobody asked* — and round 2 removed one of them.

**The user chose to close it in code rather than declare it** (`design-log.md`,
2026-09-23), and the reason the option existed is that the host already holds
the fact: `Cancel` (`crates/goad/src/wire.rs`) wraps a `watch::Sender<bool>` and
keeps its own receiver, so *a stop was asked for* is existing state. It gains
`Cancel::is_stopped`, a synchronous read — named around `Cancel::stopped`, which
is the future and must not be confused with it at a call site. `start` retains a
clone the way it already retains one of `pending`, and the loop call's `Err` arm
answers `Ended::AsAsked` when a stop was asked for.

`crates/goad/src/wire.rs` joins §Scope. R-1 gains the clause that makes this a
requirement rather than an implementation detail — *a host MUST decide this on
whether a stop was requested, which it observes, and MUST NOT infer it from the
call's success* — and R-2 takes *and no stop had been requested*. §5's residue
paragraph now opens by saying volition is **not** part of the cost, so the two
conjuncts are visibly separated: one is closed, one is declared.

`exit::status` stays pure over `Ended` and AC-4 is untouched; the classification
is in `start`, which is impure and holds the handle. A5 carries the slint
behaviour this rests on, and says that a future version which exits on latching
makes the check redundant rather than wrong. §9 gains the assertion and the
mutation.

**Outcome:** `verified`

### F-24 — §6's status-1 inference cell licenses exactly the inference §5's new MUST NOT forbids

**Severity:** blocker
**Location:** `draft-spec.md §6`, the status-1 row, *what a reader may infer* and
*what happened*; `draft-spec.md §5` (*What the seam costs*, closing sentence)

**Expected:** §6 is introduced as *"the whole of what may be inferred from
each"*, and §3 P-A says the number is *"the whole of what a consumer can branch
on"*. A normative sentence and the table that states what a reader may infer
cannot contradict one another; F-11 and F-12 both settled that a §6 cell may say
only what the process watched happen.

**Observed:** F-12's repair closed §5 with *"**A consumer MUST NOT read 1 as
evidence that the host did any work.**"* §6's status-1 row, one section later,
says a reader may infer *"The host got as far as running and is now gone."*, and
opens its *what happened* column with *"The process started —"*.

*Got as far as running* is evidence that the host did work. The two sentences
cannot both stand: one is a MUST NOT addressed to a consumer, the other is the
document's own statement of what that consumer may infer, in the cell that is
billed as the whole of it. A second implementation reading §6 draws the
inference §5 forbids, and a reviewer citing §5 against a consumer is
contradicted by §6.

The rest of the *what happened* column is observable and was correctly re-cut —
*"it reached the display, built its window and its tray, and reached the call
that runs its event loop"*. The leading *"The process started"* and the whole of
the inference cell were not.

**Evidence:** `draft-spec.md §5`, final sentence of *What the seam costs*;
`draft-spec.md §6`, status-1 row, both columns; `draft-spec.md §3` P-A. F-12's
Response in this ledger lists §6's status-1 row among the sites the recut moved.

**Disposition:** `doc-wrong`
**Response:** Both cells were mine, in round 2, and the contradiction is as stated. §6's
status-1 row now reads *"The process reached the display, built its window and
its tray, and reached the call that runs its event loop; that call then ended,
and no stop had been asked for"*, and infers *"The host reached the point of
running and is now gone. Whether it ever served anything is not in the
number."*

*The process started* is gone from the leading clause: it was the same
unobserved claim in shorter words. The inference cell no longer asserts work was
done, which is what §5 disclaims — and it now says so positively rather than
relying on a reader to notice an absence.

**Outcome:** `verified`

### F-25 — the new MUST NOT is addressed to a consumer, which D5 and OQ-6 settled the document does not do, and nothing holds it

**Severity:** major
**Location:** `draft-spec.md §5` (*What the seam costs*, closing sentence);
`design.md §6` OQ-6; `design.md §7` D5

**Expected:** D5 is a user-gated decision: *"the spec defines the statuses and
binds no supervisor"*. Its stated reason is precise — *"a MUST addressed to one
would be a requirement §7 could not verify past this repository's own single
consumer — a row naming no test, in a document whose §7 exists to forbid exactly
that"*. OQ-6 records how the decision is kept: *"every mention of a consumer is
a statement about what the **status** carries rather than an instruction about
what to do with it — that distinction is the whole of the decision, so it is
held in the wording and not only in the absence of a MUST."*

**Observed:** F-12's repair wrote *"**A consumer MUST NOT read 1 as evidence that
the host did any work.**"* into §5. That is an instruction to a consumer wearing
a MUST NOT, which is the one form OQ-6 says the decision is held against. It
carries no requirement id, so nothing can cite it; it is outside §4, so §7 has no
row for it; and no test, review clause or audit observation is named anywhere as
holding it. It is the row naming no test that D5 rejected binding a supervisor in
order to avoid.

`design.md §6` OQ-6 now states a falsehood about its own subject: *"Nothing in
the draft spec is addressed to a supervisor"*, and *"The draft spec has no
requirement addressed to a supervisor."* §2 puts restart and retry policy with
*"whatever supervises the process"*, and §3 P-D makes the consumer the holder of
that policy, so a consumer here is the supervisor OQ-6 names.

The repair may well be right that the residue needs saying. What it may not do
is say it in a form two closed decisions exclude, and leave the document that
records those decisions asserting the opposite.

**Evidence:** `draft-spec.md §5`, closing sentence of *What the seam costs*;
`draft-spec.md §4`, which carries no such requirement; `draft-spec.md §7`, whose
table has a row for R-1…R-7 and nothing else; `design.md §6` OQ-6 and `§7` D5;
`design-log.md`, 2026-09-23, OQ-6's gate. `draft-spec.md §2` (*Out of scope*) and
`§3` P-D for consumer/supervisor.

**Disposition:** `doc-wrong`
**Response:** The finding is right and the breach was mine. OQ-6 and D5 are user-gated, and
`design-log.md` records the decision as held *"in the wording and not only in
the absence of a MUST"* — so a MUST NOT addressed to a consumer is the one form
the decision exists to exclude, and it carried no id, no §4 home and no §7 row.

§5 now says **so 1 is not evidence that the host did any work** — a statement
about what the number carries, closing with *that is a statement about what the
number carries, not an instruction to whoever reads it; policy is the
consumer's (§3 P-D)*.

`design.md §6` OQ-6 no longer asserts something false about its own subject, and
does not simply revert to asserting the decision holds: it records that the
decision was **breached once and restored**, and names the shape. The decision
is held in wording, and a record of the wording that broke it is worth more to
the next repair than a restored assertion.

**Outcome:** `verified`

### F-26 — the canon-delta's replacement leaves SPEC-003/R-4 holding a clause with no test that the cell itself says is reachable, which SPEC-003 §7's preamble forbids

**Severity:** major
**Location:** `canon-delta.md` §SPEC-003 → Change 1 → *What it will say*

**Expected:** the block is applied to a live SPEC-003 **verbatim**, and the
amended document must satisfy SPEC-003's own rules. SPEC-003 §7's preamble is
one of them: *"A row naming no test is a row this spec may not be amended
holding: **where a clause cannot be reached by a test**, the row says so in terms
and says what review holds instead, rather than passing over it."* The escape is
available to a clause that **cannot** be reached; it is not available to one
that can.

**Observed:** the replacement's own argument is that the clause **can** be
reached and simply is not: *"No case in that target covers *this* requirement's
failures … and nothing prevents one: `startup::listener` … runs before the first
Slint call, so those settle headlessly exactly as the cases that target already
holds do. **They are uncovered, not unreachable.**"* It then names no test for
the clause and offers shape plus review in place of one.

So the amended R-4 row would hold a clause that names no test and is, by the
row's own words, reachable by one — precisely the shape §7's preamble says the
spec may not be amended holding. The sentence being replaced did fit the escape,
because it claimed unreachability (*"no test target links the binary"*); the
repair correctly removes the false unreachability claim and, in doing so, removes
the ground the cell was standing on. Nothing in `canon-delta.md` amends the
preamble, records an exception to it, or notes the tension.

The *uncovered, not unreachable* distinction is itself load-bearing elsewhere —
the delta's own cross-reference note narrows R-3's analogy on it, and the draft
spec's R-2 cell cites it — so it cannot simply be dropped.

**Evidence:** `docs/specs/003-host-event-ingress.md` §7, preamble paragraph;
`canon-delta.md` §Change 1, *What it will say*, and the paragraph *One
cross-reference checked rather than assumed*; `draft-spec.md §7` R-2's cell,
which cites the same distinction. SPEC-003 R-4's text in §4, whose *"The host
MUST NOT start without the listener its configuration asked for"* is the clause
the exit status speaks for.

**Disposition:** `fix-now`
**Response:** SPEC-003 §7's preamble is as quoted — verified in the live document: *"A row
naming no test is a row this spec may not be amended holding: where a clause
**cannot** be reached by a test, the row says so in terms."* The escape is for
unreachable clauses. F-2's repair correctly removed the false unreachability
claim and, in doing so, removed the ground the no-test row stood on. The
amendment could not have landed at audit.

**The user chose to land the deferred case** (`design-log.md`, 2026-09-23).
`exit_codes::an_unbindable_ingress_path_exits_2` joins §Scope with
`scratch_config` extended to write a configuration that loads; AC-5 is
unaffected, since every existing case still passes unmodified and an addition is
not a modification. AC-8 now states in terms why the cell must name a case, so
the constraint is recorded where a promoter reads it rather than only here.

**The companion obligation fell due in the same movement**, exactly as
`slice-010.md` §Follow-ups always said it would: with R-4's exit held by a case,
R-3's *"same position"* analogy is false, so `canon-delta.md` gains **Change 3**
narrowing it, marked *not separable from Change 1* — applying Change 1 alone
would leave SPEC-003 asserting an equivalence its own amended cell contradicts.
The follow-up row is struck with what killed it.

Round 1's deferral was sound on its own terms. What overturned it was a
constraint in SPEC-003 that neither round 1 nor round 2 read.

**Outcome:** `verified`

### F-27 — F-16's second needle never reached `design.md §9`: the validation row and the mutation list still describe a one-needle case

**Severity:** major
**Location:** `design.md §9` (the `structure::the_loop_s_ending_is_never_a_startup_failure`
row; the *Mutations the plan should confirm are caught* paragraph)

**Expected:** §9 is headed *"What the plan must produce"* and is the
implementation view a phase agent works from; its mutation list exists *"so the
coverage claim is measured rather than asserted"*. F-16's own finding closed on
exactly this: *"The mutation §9 asks the plan to confirm is caught —
`map_err(StartupError::Platform)` restored — is the spelled form, so nothing in
the slice's own validation would find this one either."*

**Observed:** F-16's Response names the three sites it updated — *"`design.md`
§5.2, `design.md` §8 R2 and the draft spec's R-2 cell all state both needles"*.
§9 is not among them, and §9 still carries the case as a one-needle instrument:

- the validation row's *what* column reads *"no line of `crates/goad/src` names
  both `run_event_loop_until_quit` and `StartupError`"* — needle 1 alone;
- the mutation list names one mutation for the case — *"restoring
  `run_event_loop_until_quit().map_err(StartupError::Platform)` must red
  `the_loop_s_ending_is_never_a_startup_failure`"* — the spelled form, again
  needle 1 alone.

So the document the plan is built from specifies half the instrument, and the
slice's own measured-coverage step confirms only the half that already worked. A
phase agent implementing §9 faithfully produces the case F-16 found inadequate;
nothing in §9 would notice.

**Evidence:** `design.md §9`, both sites; `design.md §5.2` and `§8` R2 and
`draft-spec.md §7` R-2's cell, which do state both needles; F-16's Response in
this ledger.

**Disposition:** `doc-wrong`
**Response:** Correct, and the omission is the one F-16's own finding predicted: its closing
sentence observed that §9's mutation would not catch the new route, and the
repair updated three sites without updating §9. A design whose implementation
view specifies half an instrument produces half an instrument.

§9's validation row and mutation list now both state the instrument as §5.2
specifies it, and the mutation list gains the one that measures the second
needle on its own: **adding `impl From<slint::PlatformError> for StartupError`
with no call site changed must red the case.** That is the whole claim of a
precondition scan — that it fires before anything uses the thing — and it is
worth nothing unmeasured. §9 also gains the mutations for F-23's arm and F-26's
case.

**Outcome:** `verified`

### F-28 — the stated limit is still not the limit: a one-line, unaliased re-filing through `Into::into` names neither needle

**Severity:** major
**Location:** `design.md §5.2` (*What the case does not reach*), `design.md §8`
R2, `draft-spec.md §7` R-2's cell

**Expected:** F-16's standard, adopted by its repair and unchanged: *"A stated
limit that is not the limit is worse than none: it is what a reader stops looking
past."* R-2's cell states the limit *"because a line scan's limit is the whole of
what it costs"*, and says the case reds on the re-filing *"in either spelling one
line admits"*.

**Observed:** the same premise F-16 used — `StartupError` gains
`impl From<slint::PlatformError>` — admits a third one-line spelling that neither
needle catches:

```rust
  slint::run_event_loop_until_quit().map_err(Into::into)?;
```

and its twin `.map_err(|error| error.into())?`. Each re-files the loop's ending
as a startup failure, on one line, with no alias and no rename-import. Needle 1
misses it because the line names `StartupError` nowhere. Needle 2 misses it
because the `?` is not adjacent to the call: the matcher is specified as the
substring `run_event_loop_until_quit()?`, and the text here is
`run_event_loop_until_quit().map_err(…)?`.

The three sites still declare the limit as exactly two shapes — *"a re-filing
spread over more than one line, or one routed through an alias"* — and R-2's cell
calls the two needles *"either spelling one line admits"*. Both statements are
false of the tree the moment the `From` impl F-16 postulates exists, which is the
same postulate the second needle is built on.

This is the class F-16 named, not a new one: the needles were widened and the
statement of what they do not reach was not re-derived from the matcher.

**Evidence:** `occurrences_where` over `production_lines`
(`crates/goad-boundary/tests/checks/structure.rs`) applies its predicate to one
`code_of`-stripped line at a time, so a needle absent from the text is absent
from the scan. `crates/goad/src/startup.rs` declares no `impl From<…> for
StartupError`. `design.md §5.2`'s two bullets, `design.md §8` R2's sentence, and
`draft-spec.md §7` R-2's cell, all of which state the two limits and the two
needles.

**Disposition:** `fix-now`
**Response:** Confirmed: `map_err(Into::into)?` and `map_err(|error| error.into())?` each
re-file on one line, unaliased, naming neither needle. I have now widened this
instrument twice and it has leaked twice, which is the signal that the shape is
wrong rather than the list short.

**The user endorsed swapping the needle set** (`design-log.md`, 2026-09-23). The
second needle is no longer a call-site spelling but the precondition every
silent re-filing requires: **`impl From<…> for StartupError`**. `?`,
`map_err(Into::into)`, `map_err(|e| e.into())` and every sibling differ only in
spelling and all of them need that impl to exist. `StartupError` declares none
today and has no reason to — its variants are constructed at their call sites in
`start`.

The stated limit is now **derived from the matcher** rather than enumerated:
`occurrences_where` tests one `code_of`-stripped line at a time, so what escapes
is a named re-filing split across lines, or an `impl From` written across lines
or for an alias. What it no longer depends on is which conversion spelling a
call site uses. A limit that is a property of the instrument stays true; a limit
that is a list goes stale the next time someone writes a spelling nobody
listed — twice, here.

**Outcome:** `verified`

### F-29 — `Ended`'s and `StoppedRunning`'s doc comments still assert the loop began, in the doc D1 calls load-bearing

**Severity:** major
**Location:** `design.md §5.2` (the `crates/goad/src/exit.rs` code block, the
`Ended` type doc and the `StoppedRunning` variant doc); `design.md §7` D1

**Expected:** F-12's recut removed *the loop began* from R-2, R-6, §6's status-1
row, §5's diagram and §5's *cut is phase* paragraph, on the ground that the
process cannot observe it. §7 D1 says of this very doc comment: *"§5.2's doc
comment is what says so, and it is load-bearing for exactly that reason"* — and
F-1, a round-1 blocker, was raised against an earlier version of the same
comment. It is also the text a phase agent copies into `exit.rs` verbatim, where
it becomes the type's own doc and outlives the slice.

**Observed:** both doc comments still state the fact the recut removed.

- `Ended`: *"either an invocation that was a question and has been answered —
  the event loop never having begun — or a host whose loop **began** and then
  ended."*
- `Ended::StoppedRunning`: *"The loop ended and nobody asked it to."*

The first asserts the loop began, which the draft spec now says in terms the host
cannot observe (R-2: *"a host cannot observe whether the loop itself began"*;
§7 R-2's cell: *"What no test and no review holds is that the loop began"*). The
second asserts that nobody asked, which F-23 shows is false of a reachable end.
A reader checking the code against the spec reads this comment for the `Ok`
boundary — D1 says so — and finds it claiming what the spec disclaims.

`design.md §5.5`'s A4 was added to carry exactly this and is stated two sections
away from the type it is about, which is the arrangement §2 of the draft spec
rejects for clauses.

**Evidence:** `design.md §5.2`, the `exit.rs` block; `design.md §7` D1;
`draft-spec.md §4` R-2 and `§7` R-2's cell; F-1 and F-12 in this ledger.

**Disposition:** `doc-wrong`
**Response:** The sharpest instance of the class, because §7 D1 calls this comment
load-bearing and a phase agent copies it into `exit.rs` verbatim, where it
outlives the slice. F-1 was a round-1 blocker against an earlier version of the
same comment; this is its third pass.

`Ended`'s doc no longer says the loop began — it says the process *reached the
call* — and now carries the residue **at the type**, in two sentences, rather
than leaving A4 to say it two sections away. That placement is the point: §2 of
the draft spec rejects exactly the arrangement where a clause's exception lives
elsewhere, and a type doc is a clause.

`StoppedRunning`'s doc no longer says nobody asked it to. It states the
condition the variant is now built under and names the trap in terms: *the call
also answers `Err` for a stop a person asked for … a call site that classifies
on the error alone is wrong even though it compiles.*

**Outcome:** `verified`

### F-30 — §2's two repaired enumerations are each incomplete, and one was made incomplete by the sibling repair in the same round

**Severity:** minor
**Location:** `draft-spec.md §2`, the two closing paragraphs

**Expected:** F-13's repair replaced a universal about the requirement set with
named members, and its Response states the intent: *"A universal about the
requirement set is what went wrong here; it is not replaced with a corrected
count."* A named list a reader uses as an index is only worth the members it
names.

**Observed:** both paragraphs name two members, and both lists are short by one.

- *"it is met in §4 wherever a clause has an exception to carry: R-7's is the
  status it does not choose, R-4's is that same end writing no line"*. **R-6 has
  an exception and carries it in its own sentence** — *"Its exception is the seam
  §5 names: where the event-loop call fails on entry the line still says the host
  had been running"* — added by F-12's repair, in the same round, and not named
  here. A reader using §2 as the index of sanctioned exceptions finds R-6's
  unaccounted for.
- *"§7 governs a clause **no cooperating test reaches** … R-1's and R-2's rows
  are where that one is met."* R-4's cell also holds such a clause and says so:
  *"*last line the process writes* … for *stopped running* it is review — the
  report is written by `main` after `run` has returned"*. No cooperating test
  reaches that either.

Neither omission is fatal to the rule — R-6's exception is in its clause, R-4's
cell does say what review holds — so this is the index and not the requirements.
It is raised because §2's two lists are the repair F-13 asked for, and one of
them went stale inside its own round.

**Evidence:** `draft-spec.md §2`, closing paragraphs; R-6's text in §4; §7's R-4
cell, closing clause. F-13's Response in this ledger.

**Disposition:** `doc-wrong`
**Response:** Both lists were the repair F-13 asked for, and one went stale inside its own
round — R-6's exception was added by F-12, in the same round, and F-13's list
did not learn of it.

§2's exception index now names R-7's, R-4's and R-6's. Its §7 index now names
R-1's, R-2's and R-4's rows. Neither is a count and both are indexes of a closed
set that only grows when a requirement gains an exception — at which point the
requirement's own sentence is being edited, which is the moment to edit the
index.

**Outcome:** `verified`

### F-31 — §7's promotion gate travels into canon with nothing to remove it, and it is not mechanical

**Severity:** minor
**Location:** `draft-spec.md §7`, the preamble paragraph beginning *While this
document is a draft*; `design.md §10` (*Obligations promotion must discharge*);
`design.md §8` R4

**Expected:** `docs/AGENTS.md` §Documentation: *"Canon is normative and
evergreen: it states what is true now. No changelogs, no revision history."*
`design.md §8` R4's mitigation is *"§10's reconciliation rows name **every**
promotion obligation, including D6's three citation sites."* F-18's Response
justifies the gate by where it lives: *"unlike `design.md §9`'s commitment rule
it **travels with the promoted document**, which is the half the finding
identified as missing."*

**Observed:** two halves.

*Nothing removes it.* The paragraph is about the document's own draft status and
is false the moment the document is canon. F-18's Response makes travelling with
the promoted document the reason the gate is better than `design.md §9`'s rule —
so it is meant to survive — yet a promoted spec carrying *"While this document is
a draft, its rows are written in the present indicative and some of the cases
they name do not exist yet"* and a MUST about its own promotion is exactly the
revision history AGENTS forbids. Either reading leaves an obligation: if it goes,
§10 must say so and does not; if it stays, canon holds a paragraph that is false
of itself. §10's obligation list names three citation sites and nothing about
§7's preamble, so `design.md §8` R4's *every promotion obligation* is now false.

*It is not mechanical.* The Response claims the gate *"makes the consequence
normative and mechanical rather than a matter of care"*, and grounds that on
*"the same mechanism the delta already uses for its `SPEC-00N` placeholder — an
unfinished promotion is detectable"*. The placeholder is detectable: it is a
literal string a grep finds. *Every case named in §7 resolves in the tree* is
not — it is a hand walk of every symbol §7 cites across four files, with nothing
naming the set to walk. The two are not the same mechanism, and the one the
finding was answered with is the weaker of them.

**Evidence:** `draft-spec.md §7`, preamble; `design.md §10`, *Obligations
promotion must discharge*; `design.md §8` R4; `docs/AGENTS.md` §Documentation and
§Canon that does not exist yet; `canon-delta.md`'s `SPEC-00N` paragraph. F-18's
Response in this ledger.

**Disposition:** `doc-wrong`
**Response:** Right on both halves, and the second is the one that matters: `docs/AGENTS.md`
§Documentation says canon states what is true now, so a promoted spec carrying a
paragraph about its own draft status is a revision history.

The paragraph is now an HTML comment marked **`DRAFT-ONLY, removed at
promotion`**, and the obligation moved to where obligations live: `design.md`
§10 carries removing it and checking every citation resolves, alongside D6's
three citation sites and the `SPEC-00N` substitutions. §8 R4's *every promotion
obligation* claim is true again.

The MUST is gone with the paragraph. F-18's repair was right that the commitment
needed to travel with the document and wrong that a normative sentence inside
the document was how — a promotion obligation is the mechanism this project
already has, and it is the one `canon-delta.md`'s placeholder note uses.

**Outcome:** `verified`

### F-32 — §5 names two entry-failure shapes; `design.md` A4 names three, and one of the two cannot occur in this host at that point

**Severity:** minor
**Location:** `draft-spec.md §5` (*What the seam costs*, second sentence);
`design.md §5.5` A4

**Expected:** §5's residue paragraph is the whole of what the recut declares, and
R-6's exception and §7's R-2 cell both defer to it. `CLAUDE.md` §Working here:
name the members, or name the rule they share.

**Observed:** §5 gives a two-member list — *"The call can fail on entry — no
platform could be selected, or a loop instance is already running"*. `design.md`
A4 gives three for the same seam: the nested-loop arm, *"`run_app_on_demand` can
itself fail at entry"*, and *"`with_platform` can fail to select a platform at
all"*. §5 drops the middle one, which is the only one of the three with a
production route.

The one §5 leads with has no production route. `with_platform` reaches
`i_slint_backend_selector::with_global_context`, which calls its backend factory
only when no context exists; `i_slint_core::context::with_global_context` returns
`Ok(f(ctx))` directly once `GLOBAL_CONTEXT` is set. In `start`,
`PromptWindow::new()` has already succeeded before the loop call, and
`crates/goad/src/main.rs`'s own step-5 comment records why that is what selects
the backend. So by the time `start` reaches the call, the platform is selected
and *no platform could be selected* cannot arise. A4's nested-loop arm is
likewise declared not a production path — *"`start` calls the function once"* —
which leaves `run_app_on_demand` failing at entry as the only live shape, and it
is the one §5 does not name.

The residue is real and worth declaring; what is stated is not the shape it
would take.

**Evidence:** `draft-spec.md §5`, *What the seam costs*; `design.md §5.5` A4;
`with_platform` and `with_global_context`
(`i-slint-backend-selector-1.17.1/lib.rs:181` and the `with_global_context`
beneath it); `i_slint_core::context::with_global_context`
(`i-slint-core-1.17.1/context.rs:349`), whose `Some(ctx) => Ok(f(ctx))` arm skips
the factory; `EventLoopState::run`
(`i-slint-backend-winit-1.17.1/event_loop.rs:689-717`) for the two arms A4 names.
Vendored citations, pinned at 1.17.1. `crates/goad/src/main.rs`, `start` step 5.

**Disposition:** `doc-wrong`
**Response:** Verified: `PromptWindow::new()` succeeds before the loop call and is what selects
the backend (`start`'s own step-5 comment says so), so the global context is set
and *no platform could be selected* cannot arise at that point. §5 led with the
one shape that has no route and omitted the one that does.

§5 now names the shape rather than the mechanism: *the windowing system can
refuse to start the loop at the moment it is asked, which is the shape with a
live route once a window already exists*. A spec states what a reader must know;
which of three internal arms produces it is A4's, and A4 keeps all three because
a design may cite a mechanism a spec should not.

**Outcome:** `verified`

### F-33 — §5's diagram names the post-call state `Running`, which is the word the recut removed everywhere else

**Severity:** minor
**Location:** `draft-spec.md §5`, the `stateDiagram-v2` block

**Expected:** F-12's Response lists *"§5's diagram transitions"* among the sites
the recut moved, and the transitions did move — *"a step before the event-loop
call failed"* and *"the event-loop call was reached"* are both observable. The
state names are part of the same diagram and say the same kind of thing.

**Observed:** the state a host enters on reaching the call is named `Running`,
and every path to 0 or 1 runs through it. For the shape §5's own next paragraph
declares — the call failing on entry — the host never ran, so the diagram routes
that end through a state named for something that did not happen, three lines
above the paragraph saying it did not. A diagram is read before the prose beneath
it, and this one still carries the pre-recut vocabulary.

`Unasked` is not part of this: nobody did request an entry failure, so that state
name survives the recut intact.

**Evidence:** `draft-spec.md §5`, the diagram's `Invoked --> Running: the
event-loop call was reached`, `Running --> Asked`, `Running --> Unasked`; the
*What the seam costs* paragraph below it; R-2's *"a host cannot observe whether
the loop itself began"*.

**Disposition:** `doc-wrong`
**Response:** The transitions were re-cut by F-12 and the state they point at was not. It is
`Reached` now — the process reached the event-loop call — and the two
transitions out of it read *a person asked it to stop* and *the call ended, none
asked*, which is the same cut R-1 and R-2 now make.

**Outcome:** `verified`

### F-34 — `report_startup_line`'s own doc names `report_startup`, which this slice removes, and it is on no change list

**Severity:** minor
**Location:** `design.md §5.2` (the `crates/goad/src/diagnostics.rs`
subsection); `crates/goad/src/diagnostics.rs`, `report_startup_line`'s doc
comment

**Expected:** F-9's repair put every doc site this slice falsifies on a change
list, and `design.md §5.2` sets the standard for `startup.rs` by enumerating each
repair as *"a claim the type currently falsifies"*. §5.2 says explicitly that
`report_startup_line` *"keeps its name, its signature and its text"* — its doc is
neither of those three and is not mentioned.

**Observed:** `report_startup_line`'s doc opens *"The exact string
`report_startup` writes, with no destination — the pure half, so a test can
assert it with no sink to fake (F-7)."* §5.2 removes `report_startup` — *"the
impure outlet with one caller … is removed rather than left as a second door onto
the same stream"* — so after this slice the surviving function's doc defines it
by a function that does not exist. §5.2 names the module's `//!` doc as the site
to repair and stops there; F-9's sweep named the same two sites and missed this
one, one function down in the same file.

The intra-doc link in `report_platform_line`'s neighbour is unaffected; this is
the one site, and the fix is a sentence.

**Evidence:** `crates/goad/src/diagnostics.rs`, `report_startup_line`'s doc
comment immediately above `pub fn report_startup_line`, and `report_startup`
beneath it with its single caller at `crates/goad/src/main.rs`'s `main`.
`design.md §5.2`, the `diagnostics.rs` subsection. F-9 in this ledger.

**Disposition:** `fix-now`
**Response:** F-9's class, one function down in the same file, and missed by F-9's own sweep.
`report_startup_line`'s doc defines the function by `report_startup`, which this
slice removes, so the survivor would have been documented in terms of something
that does not exist.

§5.2 said the function *"keeps its name, its signature and its text"* — three
things, and the doc comment is a fourth. It now says so and re-anchors the doc to
the outlet that survives. The text the function answers is unchanged, so AC-5
and the binary tier's assertions are untouched.

**Outcome:** `verified`


### F-35 — §5.2's seam block still classifies on the error alone, which is the code its own neighbouring doc comment calls wrong

**Severity:** blocker
**Location:** `design.md §5.2` (the `crates/goad/src/main.rs` subsection, the
fenced block introduced by *"`start`'s last statement … becomes the seam"*)

**Expected:** §5.2 is *Interfaces & contracts* — the section a phase agent
implements from, and the one place this design writes the seam as code. F-23's
repair made the `Err` arm conditional on `Cancel::is_stopped`, and R-1 now
carries a MUST for it. The block that shows the arm must show the arm the
design requires.

**Observed:** the block is unchanged from before F-23:

```rust
  match slint::run_event_loop_until_quit() {
    Ok(()) => Ok(Ended::AsAsked),
    Err(error) => Ok(Ended::StoppedRunning(error)),
  }
```

That is the classification-on-the-error-alone the same section's
`Ended::StoppedRunning` doc comment explicitly forbids, three fenced blocks
above it: *"this variant is built only after consulting `Cancel::is_stopped`,
and a call site that classifies on the error alone is wrong even though it
compiles."* §5.1 states the arms the design wants — *"`Err` → `Ended::AsAsked`
**if a stop was asked for**, and `Ended::StoppedRunning` otherwise"* — and §5.2
shows the other thing.

The two are not a redundancy that happens to disagree: §5.1 is prose and §5.2 is
the code. An agent expanding a phase sheet from §5.2's fenced block writes
exactly the defect F-23 was raised on, passes every case §9 names (see F-37),
and leaves the gate green.

This is round 3's own recurring class — a repair that did not reach all its
sites (F-27, F-30) — landing on the one site that is copied rather than read.

**Evidence:** `docs/slices/010/design.md §5.2`, the fenced block beginning
`match slint::run_event_loop_until_quit() {`; the `Ended::StoppedRunning` doc
comment in the `exit.rs` block of the same section; `design.md §5.1`, the
paragraph beginning *"`start` keeps a clone of `cancel`"*; `draft-spec.md §4`
R-1's closing clause; `design.md §5.5` A5.

**Disposition:** `doc-wrong`
**Response:** Verified: §5.2's block was the pre-F-23 arm. Repaired
by F-37's route rather than by writing the `is_stopped` branch into the block:
the block now binds the call in a statement of its own and hands the result and
`stop_requested.is_stopped()` to `exit::ended`, whose body is in the `exit.rs`
block above it. `Ended::StoppedRunning`'s doc names `ended` as its builder. The
sites that described the old arm were swept together — §5.1, §5.5's first
invariant, A1, A5, the edges table (a new row for a requested stop answered
`Err`), and the draft spec's R-1 and R-2 cells.
**Outcome:** `verified` (round 5).

### F-36 — A1 still asserts *Asked ⇒ `Ok`*, the implication F-23 proved false, and now contradicts A5 four bullets below it

**Severity:** blocker
**Location:** `design.md §5.5`, assumption A1 (first direction)

**Expected:** F-23 named three sites that inherited its defect, and `design.md
§5.5` A1 was the first of them: *"A1, rewritten by F-12's repair to argue both
directions, opens 'Asked ⇒ `Ok`: `run_event_loop_until_quit` returns `Ok` when
`quit_event_loop` was called'. That implication is false at the pinned
version."* An assumption list is billed in the design as *"each one a place this
design can break"*; an assumption known to be false is not an assumption.

**Observed:** A1 is verbatim as F-23 quoted it. It still reads *"Two directions,
and R-1's *if and only if* needs both. *Asked ⇒ `Ok`*:
`run_event_loop_until_quit` returns `Ok` when `quit_event_loop` was called"*,
and its body argues only about where `quit_event_loop` is called from — which,
as F-23 established, settles nothing about the return value.

F-23's Response repaired R-1, R-2, §5, §9 and `Ended`'s doc comment, and added
A5. It did not touch A1. The result is that two adjacent entries in one list
contradict each other: A1 says a requested stop yields `Ok`, and A5 says *"a
latched error followed by a requested quit answers `Err` for an end a person
asked for."*

The consequence is not cosmetic. A1 is the whole of the design's argument that
R-1's *if and only if* holds, and `draft-spec.md §7` R-1's cell rests on the
same reasoning (see F-40). A reader checking R-1 against the design finds an
argument the ledger already recorded as false.

**Evidence:** `docs/slices/010/design.md §5.5`, assumption A1's first direction,
and A5 in the same list; F-23 in this ledger, third bullet of its *Observed*
section and its *Evidence* citations into
`i-slint-backend-winit-1.17.1/event_loop.rs` (vendored, pinned). Re-verified in
the vendored source: `about_to_wait` sets `loop_error` from
`create_inactive_windows` with no `event_loop.exit()` beside it, the only
`if self.loop_error.is_some() { event_loop.exit(); }` is at the foot of
`window_event`, and `EventLoopState::run` returns `Err(error)` from its latched
`loop_error` after `run_app_on_demand` has returned normally.

**Disposition:** `doc-wrong`
**Response:** Verified: A1 still opened *Asked ⇒ `Ok`*, which F-23
proved false. A1 now argues only *`Ok` ⇒ asked* — the one direction the design
still reads off the call, and what lets `exit::ended` answer `Ok` without
consulting the request — and says in terms that the other direction is false
(A5) and that R-1's *if* half rests on the request read, not on the call. The
quit-route argument and the `Closed` residue moved inside that direction, where
they belong.
**Outcome:** `verified` (round 5). A1's remaining direction, *`Ok` ⇒ asked*, is itself false at the pinned version; that is F-53's, not a defect in this repair.

### F-37 — the case §9 names for the volition arm cannot observe volition, and the mutation it is paired with reds nothing

**Severity:** blocker
**Location:** `design.md §9`, the validation row *"a loop `Err` is *as asked*
when a stop was asked for, and *stopped running* when none was — both arms, over
the same error value"*, and the mutation *"Taking the `is_stopped` check out of
the loop's `Err` arm must red `a_loop_error_after_a_requested_stop_is_0`"*;
`design.md §5.1`, closing sentence

**Expected:** §9 is *"what the plan must produce"*, and its mutation list exists
*"so the coverage claim is measured rather than asserted"*. F-27 was raised in
round 3 because a mutation list exercised only half an instrument; the standard
it set is that a named mutation must red the named case.

**Observed:** the named case is in `exit_status`, the module of
`crates/goad/tests/renderer/startup.rs` that drives `exit::status`. That
function's whole argument is `&Result<Ended, StartupError>`, and §5.2 fixes its
arms as `Ok(Ended::AsAsked) => 0`, `Ok(Ended::StoppedRunning(_)) => 1`,
`Err(_) => 2`. Each of the following is fatal to the row:

- **`Ended::AsAsked` carries no payload.** The row says the two arms are
  asserted *"over the same error value"*. There is no error value in the
  as-asked arm to be the same one — the design's own `Ended` declaration gives
  `AsAsked` no field. The row describes a case that cannot be written.
- **The case is a duplicate of one already in the list.**
  `a_loop_error_after_a_requested_stop_is_0` can only construct
  `Ok(Ended::AsAsked)` and assert 0, which is
  `exit_status::as_asked_is_0`, two rows above it, under a name that claims
  something about a loop error and a stop. A case named for a fact it does not
  see is the failure mode `docs/memory/` records as *tests asserting proxies*.
- **The mutation cannot red it.** The `is_stopped` check lives in `start`
  (`crates/goad/src/main.rs`), which is the binary crate — §4's second guiding
  principle says so in terms: *"the binary tier spawns the process … nothing
  reaches inside `main` to assert a branch of it."* Deleting the check changes
  which `Ended` `start` builds; it changes nothing any renderer-tier test
  constructs. Every case in `exit_status` stays green.

`design.md §5.1` carries the same claim — *"what is held one tier down is
`Cancel::is_stopped`'s own behaviour **and the classification over both arms**"*
— and it is false for the same reason: `exit::status`'s arms are over `Ended`,
and the classification F-23 added is the step that chooses the `Ended`.

So the design commits the plan to a case that cannot hold what the row says,
and records a mutation that measures nothing. The requirement the pair is
supposed to discharge — R-1's *"a host MUST decide this on whether a stop was
requested"* — ends up held by nothing at all.

**A repair exists and is not obviously free.** The decision could be lifted into
a pure function in `exit.rs` — something of the shape
`fn ended(result: Result<(), slint::PlatformError>, stopped: bool) -> Ended` —
after which both arms are assertable at the renderer tier over one error value
and the mutation reds. `exit.rs` and `crates/goad/tests/renderer/startup.rs` are
both already in `slice-010.md` §Scope, so this adds no surface. It does add a
second public item to `exit.rs` and a second thing `main`/`start` calls, and it
would need its own row in §9 and in `draft-spec.md §7`; this is raised as a
route, not as a settled repair.

**Evidence:** `design.md §9`, the two rows and the mutation sentence naming
`a_loop_error_after_a_requested_stop_is_0`; `design.md §5.2`, the `Ended`
declaration and `exit::status`'s body; `design.md §5.1`, closing paragraph;
`design.md §4`, second guiding principle; `crates/goad/src/main.rs`, `start`;
`crates/goad/tests/renderer/startup.rs`, whose modules drive library functions
only. `draft-spec.md §7` R-2's cell, which calls the same fact *review* and
names no case for it (F-40).

**Disposition:** `fix-now`
**Response:** Verified at the symbol: `exit::status`'s argument is
`&Result<Ended, StartupError>`, `Ended::AsAsked` carries no payload, and the
check sat in `start` in the binary crate, so no renderer-tier case could see it
and the named mutation reddened nothing. **The user chose the pure function**
(`design-log.md`, 2026-09-23, round 4). Before proposing it the responder
checked the lint risk: `exit::ended` as written in §5.2 — `Err(error) if
!stop_requested => StoppedRunning(error)`, `Ok(()) | Err(_) => AsAsked` — passes
`clippy::pedantic` and `wildcard_enum_match_arm` at deny on the repo's clippy,
over stand-in types, with a top-level `_ =>` confirmed red as the negative
control.

`exit::ended(call, stop_requested) -> Ended` is in `exit.rs`, which already owns
the value, so no surface is added. Cases, `ended::` in
`crates/goad/tests/renderer/startup.rs`: both `Err` arms over **one** error
value, and the `Ok` arm. Mutations (§9): dropping the guard reds
`a_loop_error_after_a_requested_stop_is_as_asked`; dropping the arm reds
`a_loop_error_with_no_stop_requested_is_stopped_running`. **What stays review is
stated, not implied**: `start` passing a constant, or a read taken before the
call, is green everywhere, and §9 now says so under its mutation list. The read's
timing is fixed on the page — the call is bound in its own statement and the
read is in the next, after every callback that could trip `Cancel` has run.
Reached: §5.1, §5.2, §5.5, §9's rows and mutations, the draft spec's R-1 and R-2
cells, and AC-11 (F-47).
**Outcome:** `verified` (round 5).

### F-38 — Change 3's replacement text contradicts Change 1 inside one sentence, and it is applied to SPEC-003 verbatim

**Severity:** blocker
**Location:** `canon-delta.md` §SPEC-003 → Change 3 → *What will be added*

**Expected:** Change 3 exists to remove a contradiction Change 1 creates: R-3's
cell says `LivenessUnknown` holds *"the same position as R-4's exit code below
and R-5's process exit"*, and after Change 1 that is false. Both blocks are
applied to a live SPEC-003 verbatim, and the amended document must not assert
something its own sibling cell denies. F-15 and F-26 were both raised against
this file for exactly this standard.

**Observed:** the replacement reads:

> the same position as R-5's process exit — and **not** the same as R-4's, whose
> exit is now held by a case at the binary tier. What this arm shares with them
> is that review, not a test, is what stands behind the clause here; what it
> does not share is reachability.

The second sentence contradicts the first. *Them* is R-4's exit and R-5's
process exit — the two the first sentence just distinguished between. Of R-4 it
asserts that *review, not a test,* stands behind the clause; the clause before
it says R-4's exit *"is now held by a case at the binary tier"*, and Change 1's
own replacement opens **The non-zero exit is held one tier down, plus review**
and names `exit_codes::an_unbindable_ingress_path_exits_2`. A test stands behind
R-4 after Change 1. That is the whole point of Change 1.

So the narrowing lands on the wrong axis. Change 3 correctly removes the
*reachability* equivalence and then re-asserts a *verification-kind*
equivalence that Change 1 has just falsified — leaving SPEC-003 holding the
same class of false cross-reference the delta was written to remove, in the
amendment written to remove it.

The sentence is right about R-5: the live R-5 cell concedes that *"process exit
is not `Drop`, so nothing here speaks for a killed host"*, so review is what
stands behind that clause. It is R-4 the sentence cannot carry.

**Evidence:** `canon-delta.md` Change 3, *What will be added*, both sentences;
`canon-delta.md` Change 1, *What it will say*, opening sentence and the case it
names; `docs/specs/003-host-event-ingress.md` §7, R-3's cell (the phrase *"the
same position as R-4's exit code below and R-5's process exit"*), R-4's cell,
and R-5's cell (*"What the case holds is narrower than this requirement's
sentence"*).

**Disposition:** `doc-wrong`
**Response:** Verified against the live R-3 cell and the live R-5
cell. The second sentence asserted a verification kind of R-4 that Change 1
falsifies. Rather than repair the contrast, Change 3 now removes R-4 from the
analogy: the phrase *"the same position as R-4's exit code below and R-5's
process exit"* becomes *"the same position as R-5's process exit"*, and nothing
else in the cell changes. Nothing new is asserted in canon, so there is nothing
new to be false. R-5's half is untouched, and R-5's own cell supports it
(*process exit is not `Drop`, so nothing here speaks for a killed host*). The
phrase occurs once in `docs/specs/003-host-event-ingress.md`, so the
replacement is unambiguous.
**Outcome:** `verified` (round 5).

### F-39 — Change 3 is headed *What will be added* and supplies replacement text, in a file whose other change says *replaced, in place*

**Severity:** major
**Location:** `canon-delta.md` §SPEC-003 → Change 3 → *What will be added*

**Expected:** `docs/AGENTS.md` §*Canon that does not exist yet, or must change*
requires each entry to name *"the document, the section, the change as it will be
stated"*, and the file's own preamble says the changes are *"applied at audit"*.
The application is mechanical and is performed by an agent who will not have
this review in hand. Change 1 shows the form: it quotes the sentence it removes
under *What is wrong*, then says *"The sentence above is replaced, in place,
by:"*.

**Observed:** Change 3's instruction is *What will be **added**", and the block
that follows is not an addition — it opens *"the same position as R-5's process
exit"*, which is a rewrite of the live clause *"the same position as R-4's exit
code below and R-5's process exit"*. The prose above it says the intent is
*"narrowing the analogy **in place**"*, but the instruction the promoter reads
at the point of acting says *added*, and the block carries no *replaces* verb
and no quotation of the text it displaces at the point of replacement.

An agent applying it as written appends, and R-3's cell then reads *"… the same
position as R-4's exit code below and R-5's process exit. the same position as
R-5's process exit — and **not** the same as R-4's …"* — the original false
equivalence retained beside its own correction, in canon, with the delta
consumed and nothing left to re-read.

Change 2 is unambiguous because an addition is genuinely what it is (*"as a
bullet in the existing list"*). Change 3 uses Change 2's verb for Change 1's
operation.

**Evidence:** `canon-delta.md` Change 1 (*"The sentence above is replaced, in
place, by:"*), Change 2 (*"What will be added, as a bullet in the existing
list"*), Change 3 (*"What will be added, narrowing the analogy in place rather
than removing the cross-reference"*); `docs/specs/003-host-event-ingress.md` §7,
R-3's cell, the clause beginning *"no cooperating test in this workspace can
produce one"*.

**Disposition:** `doc-wrong`
**Response:** Verified. Change 3 now takes Change 1's form: it names
the sentence (the one beginning *"`BindFault::LivenessUnknown`"*), quotes the
phrase it displaces, and says *"is replaced, in place, by:"*. The verb
*added* is gone from it.
**Outcome:** `verified` (round 5).

### F-40 — R-1's new clause has no verification anywhere in §7, and R-2's cell calls *review* the fact `design.md §9` calls a test

**Severity:** major
**Location:** `draft-spec.md §7`, R-1's cell and R-2's cell; `draft-spec.md §4`
R-1

**Expected:** §7's own preamble says each row *"names the kind of verification
and what discharges it, so the claim is checkable rather than asserted"*, and
that where a clause cannot be reached by a cooperating test the row says so **in
terms**, saying what review holds and what it does not. F-23 added a normative
clause to R-1; a requirement gaining a clause gains a verification obligation in
the same movement, which is the class F-27 and F-30 were both raised on.

**Observed:** R-1 now closes with *"**A stop that was asked for reaches 0 however
the event-loop call reports it** — a host MUST decide this on whether a stop was
requested, which it observes, and MUST NOT infer it from the call's success."*
R-1's §7 cell is unchanged by F-23 and says nothing about it. What it does say
about the quit edge is the pre-F-23 statement — *"that `Ended::AsAsked` is what
`start` answers when `run_event_loop_until_quit` returns `Ok`"* — which is
exactly the `Ok`-only reading the new clause forbids relying on. The clause is
neither discharged by a named case, nor declared unreachable in terms, nor
mentioned.

R-2's cell does carry the fact, under **What review holds**: *"its `Err` becomes
`Ended::AsAsked` when a stop had been requested and `Ended::StoppedRunning`
otherwise"*. So the spec's position is that this is held by review. `design.md
§9`'s row says it is held by a test in both arms. Two documents in one slice
state different verification kinds for one fact — the disagreement F-27 was
raised on, in the opposite direction.

Which of the two is right is F-37's subject. What this finding holds is that
they do not agree, and that the requirement carrying the MUST has no row of its
own for it.

**Evidence:** `draft-spec.md §4`, R-1's third sentence; `draft-spec.md §7`, R-1's
cell in full and R-2's cell's *What review holds* clause; `design.md §9`, the
row naming `exit_status::a_loop_error_after_a_requested_stop_is_0`; F-23 and
F-27 in this ledger.

**Disposition:** `doc-wrong`
**Response:** Verified: R-1's cell said nothing about its new clause,
and R-2's called *review* what §9 called a test. After F-37 both are true of one
thing. R-1's cell now holds the clause at the renderer tier — `exit::ended`'s
three `ended::` cases, and `Cancel::is_stopped`'s case in `wire.rs` — and names
what review holds on top: that `start` hands `exit::ended` the call's own result
and a read taken after it. R-2's cell names
`ended::a_loop_error_with_no_stop_requested_is_stopped_running` beside the
`exit::status` case, and its review clause is now only the call site's
placement and that nothing else constructs the variant. The pre-F-23 sentence
about `start` answering `AsAsked` on `Ok` is gone. §9 and the draft spec now
state the same verification kind.
**Outcome:** `verified` (round 5).

### F-41 — R-4's cell states its method as *the cases R-3 names, one at a time* and then skips the case round 3 added

**Severity:** major
**Location:** `draft-spec.md §7`, R-4's cell

**Expected:** R-4 is *"Every non-zero exit this document assigns MUST be
accompanied by a line on standard error …"*, and its cell declares its own
method: *"binary tier, where the cases R-3 names do **not** all hold the same
thing, so this row takes them one at a time rather than as a set."* A cell that
says it walks a set must walk the set, or say which member it does not and why —
this document's own standard for an unheld clause.

**Observed:** after F-26, R-3's cell names five binary-tier cases:
`too_many_arguments_exits_2_and_says_who_spoke`,
`no_argument_and_no_configuration_home_exits_2`,
`an_unreadable_configuration_exits_2_and_says_only_what_its_own_arm_says`,
`an_unparseable_configuration_exits_2_and_says_only_what_its_own_arm_says`, and
`an_unbindable_ingress_path_exits_2`. R-4's cell walks the first four and
`help_prints_the_usage_block_on_stdout_and_exits_0`. The fifth is absent.

It is absent silently, which is the part that matters. `an_unbindable_ingress_path_exits_2`
is a **non-zero** exit this document assigns, so R-4 quantifies over it; nothing
in §Scope, `design.md §9` or the case's own description says whether it asserts
a line on standard error at all. A reader of the promoted spec cannot tell
whether the omission means *this case holds nothing for R-4* or *this cell was
not swept when the case landed*.

The same sweep reached R-3's cell (the case is named there) and did not reach
R-4's, one row below it — round 3's own recurring class.

**Evidence:** `draft-spec.md §7`, R-3's cell and R-4's cell; `draft-spec.md §4`
R-4; `slice-010.md` §Scope, the `exit_codes.rs` bullet; `design.md §9`, the row
*"a configured ingress path the host cannot bind exits 2, read from the process
by a caller"*, which claims the status and nothing about standard error;
`canon-delta.md` Change 1, whose replacement text also describes the case as
asserting *"the status a caller reads"* and no line.

**Disposition:** `doc-wrong`
**Response:** Verified: R-4's cell walked four of R-3's five cases and
skipped the fifth without saying so. It now says so:
`an_unbindable_ingress_path_exits_2` holds the status and nothing about standard
error, and is listed because R-3 names it. The `Ingress` line is held one tier
down by `stderr_outlets::report_startup_line_renders_ingress_like_its_siblings`,
and that it reaches a real process's standard error is review. The case is not
strengthened. That would widen an added case for a claim no finding asked it to
carry.
**Outcome:** `verified` (round 5).

### F-42 — Change 3 exists in `canon-delta.md` and on no change list outside it; the design and the slice still say SPEC-003 takes one amendment, in one cell

**Severity:** major
**Location:** `design.md §10` (the canon-impact table); `design.md` §Authority
block; `slice-010.md` §Tier; `slice-010.md` §Scope → **Canon**

**Expected:** `design.md §10` is titled *Canon impact* and is the design's
statement of what this slice does to canon; §10 itself says *"§10's
reconciliation rows name every promotion obligation"* (R4's mitigation in §8).
Round 3's brief named this surface in terms: *do `design.md` §9's and §10's
lists still name every obligation the documents carry?*

**Observed:** Change 3 is a new amendment to a second cell of SPEC-003 §7,
marked in `canon-delta.md` as **not separable** from Change 1. The sites
outside that file still describe the SPEC-003 amendment as a single cell:

- **`design.md §10`'s table** has a row for *SPEC-003 §7, R-4's cell* and a row
  for *SPEC-003 §9 References*. There is no row for R-3's cell. The table is
  what a reconciliation walks; an amendment with no row is an amendment nobody
  is asked to apply.
- **`design.md`'s Authority block** says *"`canon-delta.md` carries **the one
  change** to SPEC-003."* Three changes. It is also a count, which
  `CLAUDE.md` §Working here forbids for a set that can grow — and this one grew.
- **`slice-010.md` §Tier** says the slice writes *"a spec … and an amendment to
  SPEC-003/R-4's verification cell"*.
- **`slice-010.md` §Scope → Canon** says *"`canon-delta.md` for SPEC-003/R-4's
  verification cell, which is stale in one sentence and asserts the defect as a
  fact in another"* — a description of Change 1 alone, offered as the whole of
  the canon scope.

AC-8 is the only place outside `canon-delta.md` that mentions Change 3, and it
mentions it in a subordinate clause of a criterion whose subject is R-4's cell.
An acceptance criterion is not a change list.

**Evidence:** `canon-delta.md`, Change 3's heading and its *Not separable from
Change 1* paragraph; `design.md §10`'s table, all four rows; `design.md`'s
Authority block, final sentence; `slice-010.md` lines under **Tier** and under
§Scope → **Canon**; `slice-010.md` AC-8's final sentence.

**Disposition:** `doc-wrong`
**Response:** Verified, and the sweep is wider than the finding's
list. Also stale were `design.md` §3 (named R-4's cell alone); `design.md` §4's
third principle, which still called R-4's exit *uncovered* after F-26 landed
the case; `slice-010.md` §Governing canon; and `draft-spec.md` §9, which cited
R-4's cell as the form for clauses no cooperating test reaches, false once
Change 1 lands. Now: the Authority block names the changes (§7's R-4 and R-3
cells, §9 References) without counting them; §10 has a row for R-3's cell,
marked not separable, and cites each Change by number; the Tier, §Scope → Canon
and §Governing canon name all three; the draft spec cites R-3's cell for the
form and R-4's for the deferral.
**Outcome:** `verified` (round 5).

### F-43 — the second needle's stated limit misses a single-line re-filing: a variant import defeats both needles at once

**Severity:** major
**Location:** `design.md §5.2` (*What the case does not reach*) and `design.md
§8` R2; `draft-spec.md §7`, R-2's cell (*What that scan does not reach*)

**Expected:** F-28 swapped the second needle to `impl From<…> for StartupError`
on the argument that it is *"upstream of the call site rather than chasing
it"*, and all three sites derive the instrument's limit from the matcher rather
than listing it — *"a limit derived from the matcher stays true; a limit
enumerated by hand goes stale the next time someone writes a spelling nobody
listed."* A derived limit must actually follow from the matcher. The limit is
stated in canon: it goes into the promoted spec verbatim.

**Observed:** the stated limit is multi-line spellings and an aliased
`StartupError`. There is a **single-line** shape it does not cover:

```rust
use crate::startup::StartupError::Platform;   // an import, not an alias
…
slint::run_event_loop_until_quit().map_err(Platform)?;
```

The call line names `run_event_loop_until_quit` and does **not** name
`StartupError`, so needle one does not fire. No conversion is involved, so
needle two has nothing to find — the design's own argument for needle two is
that *"every silent re-filing needs a conversion the compiler can apply"*, and
this one needs none. The `use` line names `StartupError` but not
`run_event_loop_until_quit`, so it fires neither.

The design's taxonomy is *named* versus *enabled*. This shape is named, and the
name it wears is the variant's, not the type's — a third category neither needle
covers and the stated limit does not admit. It is one line, it compiles, it
lints clean under `wildcard_enum_match_arm`, and it re-files the loop's ending
as a startup failure exactly as `map_err(StartupError::Platform)` does today.

So the sentence *"What it does **not** depend on is which conversion spelling
the call site uses, because the second needle is upstream of all of them"* is
true and insufficient: the evasion that matters here is not a conversion
spelling at all.

**Evidence:** `design.md §5.2`, the **named** and **enabled** bullets and the
paragraph *What the case does not reach*; `design.md §8` R2's closing sentence;
`draft-spec.md §7` R-2's cell, the clause beginning *"What that scan does not
reach"*. `crates/goad-boundary/tests/checks/structure.rs`:
`occurrences_where` tests one `code_of`-stripped line at a time and
`occurrences_of` is `str::contains`, so both needles are per-line substring
tests over the line's own text. `crates/goad/src/startup.rs` declares
`StartupError::Platform` as a tuple variant, so `map_err(Platform)` is a valid
constructor reference once imported.

**Disposition:** `fix-now`
**Response:** Verified. `occurrences_where` tests one `code_of`-stripped
line at a time, so needle one needed the literal `StartupError` on the call's
line. A variant import, an alias or a helper each evade it, and none needs a
conversion. The stated limit missed the whole class, not one spelling. **The
user chose to re-cut the instrument** (`design-log.md`, 2026-09-23, round 4).
It no longer lists spellings: exactly one production line of
`crates/goad/src` names `run_event_loop_until_quit`, and that line ends at the
call. Wrapping or chaining puts tokens between the call's `()` and the `;`
however it is spelled. A chain moved to the next line leaves the call's line
without the `;`. So every same-line re-filing reds, and the count half holds the
call's uniqueness, which nothing in the gate held (R-2's cell and §5.5 said so).
The limit, derived from the matcher: a later statement that re-files the call's
binding. §9's mutation list includes the variant-import spelling this finding
raised.
**Outcome:** `verified` (round 5). The same-line class is closed. The derived limit that replaced the old one is still narrower than the matcher's; that is F-55's.

### F-44 — the second needle forbids every `impl From` onto `StartupError`, and the reason given for that being free is false of the variants `startup.rs` builds

**Severity:** major
**Location:** `design.md §5.2` (the **enabled** bullet); `design.md §8` R2;
`draft-spec.md §7`, R-2's cell

**Expected:** round 3's brief named this surface: *"a needle stated that broadly
is a standing constraint on a type, not only a guard against one regression."*
`structure.rs` already carries this project's own ruling on instruments of that
shape, in `schedule_resolve_is_called_only_from_host`'s doc: an instrument that
*"reds for reasons its requirement has no view on invites the repair bump the
number — after which nobody reads its report again."*

**Observed:** the needle reds on any production line in `crates/goad/src`
declaring `impl From<…> for StartupError`, for any source type. R2's requirement
is about one thing — the loop's ending travelling as a startup failure — and the
needle's reach is every conversion into the type, forever.

The design prices that at nothing, on a stated reason: *"`StartupError` declares
none today, and it has no reason to: its variants are constructed at their call
sites in `start`."* Verified at the symbol, and the second half is false for
the variants `startup.rs` builds for itself:

- `StartupError::Ingress` is constructed in `startup::listener`
  (`crates/goad/src/startup.rs`), via `ingress::bind(&config.path).map_err(StartupError::Ingress)`.
- `StartupError::NoConfigPath` and `StartupError::Usage` are constructed in
  `startup::arguments` (same file).

`listener` is the plainest `From` candidate in the crate: an `impl
From<IngressError> for StartupError` would let its body be
`Ok(ingress::bind(&config.path)?)`, which is an ordinary tidying with no view on
the event loop at all — and it would red
`the_loop_s_ending_is_never_a_startup_failure`, in a file that reports itself as
guarding the loop's ending. `Clock(ClockError)` and `Runtime(io::Error)` are the
same shape one step behind.

Whether the constraint is worth its cost is a decision, not a defect. What this
finding holds is that the decision was taken on a false premise, that no site
records it as a standing constraint rather than a regression guard, and that
`structure.rs` already documents where instruments of this shape end up.

**Evidence:** `crates/goad/src/startup.rs`, `listener` and `arguments`, each
constructing `StartupError` variants outside `start`; `crates/goad/src/main.rs`,
`start`, which constructs the remaining variants; `grep -rn "impl From" crates/goad/src/`
returns nothing, so the needle is vacuous on today's tree;
`crates/goad-boundary/tests/checks/structure.rs`,
`schedule_resolve_is_called_only_from_host`'s doc comment, section *What it is
deliberately not about*; `design.md §5.2`, the **enabled** bullet.

**Disposition:** `fix-now`
**Response:** Verified: `startup::listener` constructs
`StartupError::Ingress`, and `startup::arguments` constructs `NoConfigPath` and
`Usage`, both outside `start`. The premise for the needle being free was false.
**The user chose to drop the `impl From` needle** (`design-log.md`, 2026-09-23,
round 4). F-43's shape rule catches every same-line spelling that needle
covered. What it gave up is a `call?` on a later line, and that sits inside the
stated limit. `StartupError` carries no standing constraint, and §5.2 says the
case constrains one line and no type.
**Outcome:** `verified` (round 5).

### F-45 — *a compiled fixture* has no precedent in the file it cites and no target that would compile one

**Severity:** major
**Location:** `design.md §5.2` (the `structure.rs` subsection, the sentence
*"A control that does not compile greps the same as a passing one, so each
needle gets a compiled fixture and not a string in a comment"*); `notes.md`
§Harvest → Learned, the `structure.rs`'s machinery entry

**Expected:** the sentence states a requirement on the case's controls and
justifies it by citing the file's existing practice: *"with a fixture control in
the file's `counting_itself` module beside `a_real_call_site_is_counted`, whose
own doc states what belongs there."* A requirement stated by analogy has to have
the analogy hold, and a phase agent reads this as the shape to copy.

**Observed:** each of the sentence's claims is false at the symbol.

- **`a_real_call_site_is_counted` has no doc comment.** Its whole body is
  `let line = "    match slint::quit_event_loop() {"; assert!(code_of(line).contains("quit_event_loop("));`,
  with `#[test]` directly above `fn`. The doc that *"states what belongs there"*
  is the enclosing `mod counting_itself`'s, one level up.
- **Its fixture is a string literal**, which is the thing the sentence contrasts
  a compiled fixture against. It is compiled as a `&str`; the construct it
  stands for is not compiled as Rust at all, so it greps the same as a passing
  one in exactly the sense the sentence warns about.
- **`goad-boundary` compiles no fixture.** Its `Cargo.toml` sets
  `autotests = false` and declares one target, `[[test]] name = "checks", path =
  "tests/checks/main.rs"`. `tests/fixtures/structure/production_after_tests.rs`
  and `tests/fixtures/structure/nested/marker.rs` are in no target; they are read
  with `std::fs::read_to_string`. Neither `cargo build --workspace` nor
  `cargo clippy --workspace --all-targets` compiles them.

So §5.2 requires something the crate has no mechanism for, and cites as
precedent the one practice it is telling the phase agent not to follow. A phase
agent either writes a string fixture — the thing forbidden — or invents a
compilation mechanism, which is a surface `slice-010.md` §Scope does not name
and a STOP condition nothing records.

`notes.md`'s *checked and clean* list carries the same characterisation
(*"the `counting_itself` module already controls the matcher with a compiled
fixture (`a_real_call_site_is_counted`)"*), which is the third time that list
has been found holding a false symbol-level claim (F-16, F-20).

**Evidence:** `crates/goad-boundary/tests/checks/structure.rs`, `mod
counting_itself` and `a_real_call_site_is_counted` within it;
`crates/goad-boundary/Cargo.toml`, `autotests = false` and its single `[[test]]`
stanza; `ls crates/goad-boundary/tests/fixtures/structure/`;
`production_after_an_inline_test_module_is_still_read`, which reads its fixture
as text. `design.md §5.2`; `notes.md` §Harvest → Learned.

**Disposition:** `doc-wrong`
**Response:** Verified: `goad-boundary` sets `autotests = false` and
declares one `[[test]]` target, its `tests/fixtures/` files are read as text,
and `a_real_call_site_is_counted` is a string literal with no doc comment. The
requirement was a mistransfer of the *negative control must compile* lesson,
which is about a **mutated build**, not a fixture. §5.2 now says the predicate's
controls are string literals in `counting_itself`, the file's own practice.
Those prove the predicate reads a line as intended and nothing about the tree.
The tree is proved by §9's mutations of real source, which count only once the
mutated build compiles and the case reds. `notes.md`'s *checked and clean*
entry is corrected and records that it has now been wrong twice.
**Outcome:** `verified` (round 5).

### F-46 — nothing establishes that losing the display cannot trip the stop signal, and under F-23's arm that turns the slice's target failure into exit 0

**Severity:** major
**Location:** `design.md §5.1` (the `Cancel` paragraph); `design.md §5.5` A5 and
§8 R1; `notes.md` §Harvest → Learned (*A1's quit wiring is as the design states
it*)

**Expected:** F-23's arm reads *a stop was asked for* off `Cancel` and answers 0
for it. The arm is only sound if `Cancel` is tripped by a person and by nothing
else — which is what `notes.md` records as checked and clean: *"`Cancel::stop`
reaches the loop from exactly two places — `window.on_close_requested` and
`tray.on_quit`."* The two places are named; what is not established is what can
*drive* them.

**Observed:** `on_close_requested` is not a host-initiated callback. It is
invoked when the windowing system delivers a close, and in the pinned backend
`window_event`'s `WindowEvent::CloseRequested` arm calls
`window.try_dispatch_event(corelib::platform::WindowEvent::CloseRequested)`,
which is what reaches the Slint callback the host installs
(`crates/goad/src/install.rs`, `window.window().on_close_requested(move || {
quitting.stop(); CloseRequestResponse::KeepWindowShown })`). So *the display
sent us something* and *a person asked us to stop* arrive at `Cancel` through
the same door.

Nothing in `design.md`, `notes.md` or the vendored reading rules out a display
teardown producing a `CloseRequested` before or alongside the `loop_error` that
ends the loop. If one can, the sequence is: compositor goes, `CloseRequested`
dispatched, `quitting.stop()` trips `Cancel`, `serve` returns, the loop ends
`Err`, `is_stopped` reads true, and the host exits **0** — under
`Restart = "on-failure"`, not restarted at all. That is strictly worse than the
defect this slice exists to repair, which at least exited non-zero.

Two related gaps, whichever way the question resolves:

- **§8 R1's failure signals were not swept.** They are *"the status is 2 after a
  lost display, or the line is the *never started* one"* — written before F-23
  made 0 reachable from that arm. AC-9 (*"the journal shows `status=1`"*) does
  still catch it, so the slice is not blind; the risk register is.
- **A5 argues the check is safe in one direction only.** It establishes that a
  latched error plus a quit must not read as *nobody asked*. It does not ask the
  converse — whether anything but a person can set the flag the check now trusts.

This is raised as an unestablished premise, not as a demonstrated race: what a
Wayland or X11 teardown actually delivers is not settled by reading
`event_loop.rs`, and settling it may need the running host (AC-9's territory).
The finding is that F-23's arm was designed without the question being asked.

**Evidence:** `crates/goad/src/install.rs`, the `on_close_requested` closure and
`tray.on_quit`; `crates/goad/src/wire.rs`, `Wire::stop` → `Cancel::stop`;
`i-slint-backend-winit-1.17.1/event_loop.rs`, `window_event`'s
`WindowEvent::CloseRequested` arm and the `if self.loop_error.is_some() {
event_loop.exit(); }` at that function's foot (vendored, pinned at 1.17.1);
`design.md §5.5` A5 and §8 R1; `notes.md` §Harvest → Learned, the A1 entry;
`slice-010.md` AC-9.

**Disposition:** `fix-now`
**Response:** Settled by reading, as far as reading goes. In winit
0.30.13 on Linux, `CloseRequested` comes only from a message received:
`WinitState::request_close` (a Wayland `xdg_toplevel` close),
`FrameAction::Close` (a client-side decoration's button), and the X11
`WM_DELETE_WINDOW` client message. Slint dispatches it to
`on_close_requested` from `window_event`. The only other `request_close` caller
is a `.slint` root `Window`'s `close()`, and `crates/goad/ui/` calls `close()`
only on popups. A broken connection delivers no message, so the measured
failure cannot reach 0 through F-23's arm. A compositor that closes its clients
as it goes does trip `Cancel`, and the host already exits 0 for that by the
`Ok` route. So the premise F-23 relied on, *a person asked*, was never
observed. **The user chose to say *asked*** (`design-log.md`, 2026-09-23, round
4): R-1 defines *asked* as a request the host received — its quit control, or a
close request delivered to its window — and disclaims who sent it. Every site
that followed was swept: the draft spec's §5 diagram and prose and §6's row 0,
`Ended`'s doc, §5.1, A1, A5, and `slice-010.md` §Scope. A5 gains the
converse direction, and §8 R1 gains *status 0 with no line* as a failure
signal. What reading cannot settle — what a real teardown delivers — stays
AC-9's.
**Outcome:** `verified` (round 5).

### F-47 — R-1's new normative clause is reachable by no acceptance criterion

**Severity:** major
**Location:** `slice-010.md` §Acceptance criteria (AC-1…AC-10)

**Expected:** `docs/AGENTS.md` §Slice has the acceptance criteria establish that
the intent of the slice was met, and the audit *"walks each acceptance criterion
in `slice-nnn.md`"*. F-23 added a MUST to R-1 and a code surface (`wire.rs`) to
§Scope; a new normative requirement that the slice is the first implementation
of needs a criterion, or the audit has no instrument that looks for it.

**Observed:** no criterion mentions a requested stop, volition, `Cancel` or
`is_stopped`. The nearest are:

- **AC-4**, which requires every **shape** `exit::status` can see to be asserted
  one tier down. The shapes are `Ok(AsAsked)`, `Ok(StoppedRunning)` and `Err`;
  the `is_stopped` decision is the step that *chooses* a shape, and is not one.
  AC-4 is satisfied whether or not the check exists.
- **AC-6**, which is about exit 1 and its stderr line.
- **AC-9**, which observes a lost display on the running host — the arm where no
  stop was requested.

So the slice can close with every criterion green and F-23's repair absent from
the code: `exit::status` is untouched by it (as F-23's own Response says), §9's
named case cannot see it (F-37), and no criterion asks for it. `slice-010.md`
§Scope records the surface, which is a statement of where work goes, not a
statement of what must be true at the end.

**Evidence:** `slice-010.md` §Acceptance criteria, AC-1…AC-10 in full;
`slice-010.md` §Scope, the `crates/goad/src/wire.rs` bullet; `draft-spec.md §4`
R-1's closing clause; F-23's Response in this ledger, which lists the sites it
changed and names no acceptance criterion.

**Disposition:** `fix-now`
**Response:** Verified: no criterion reached R-1's new clause. AC-11
added: a requested stop reaches 0 however the call reports it, the decision is a
pure function whose two `Err` arms are asserted one tier down over one error
value, and `start` feeds it a read taken after the call returns. AC-4 is left
alone. It is about `exit::status`'s shapes, and `exit::ended` is a different
function.
**Outcome:** `verified` (round 5).

### F-48 — §9's `Cancel::is_stopped` row names the wrong location and no case, in the table that says test names are commitments

**Severity:** minor
**Location:** `design.md §9`, the row *"`Cancel::is_stopped` answers false before
`stop` and true after it, and stays true | `wire`'s existing `Cancel` cases,
`crates/goad/tests/renderer/`"*

**Expected:** every other row of §9 names a case by symbol and a file by path,
and the table closes with **Test names are commitments** — *"a phase that names
a case differently updates the draft in the same commit."* `CLAUDE.md` §Working
here: cite by symbol.

**Observed:** two defects in one cell.

- **The location is wrong.** `Cancel`'s cases are
  `stopped_resolves_immediately_when_already_tripped` and
  `stopped_does_not_resolve_until_stop_is_called`, in `crates/goad/src/wire.rs`'s
  own `#[cfg(test)] mod tests`. `crates/goad/tests/renderer/` contains no case
  naming `Cancel`; the module comment above `wire.rs`'s test module says so
  directly — *"`Cancel`'s level-held property and `Notice`'s retention are
  unit-tested here"*. A phase agent reads this row and looks in the wrong crate
  target.
- **The existing cases cannot hold the row.** Both drive `Cancel::stopped()`,
  the future. `is_stopped` is a different function that does not exist yet, so
  *"`wire`'s existing `Cancel` cases"* names cases that assert nothing about the
  behaviour in the left-hand column. The row is the only one in §9 that names no
  case, in the table that calls names commitments.

The remedy is mechanical, and `Notice::raised` — the same `*self.rx.borrow()`
read, with `a_raised_notice_stays_raised_until_it_is_lowered` beside it — is the
precedent for both the function and the case.

**Evidence:** `crates/goad/src/wire.rs`, `mod tests` and the comment above it,
`Cancel::stopped`, `Notice::raised`,
`a_raised_notice_stays_raised_until_it_is_lowered`;
`ls crates/goad/tests/renderer/`; `design.md §9`, the row and the closing
**Test names are commitments** paragraph.

**Disposition:** `fix-now`
**Response:** Verified: `Cancel`'s cases are in `wire.rs`'s own
`#[cfg(test)] mod tests`, and both drive `Cancel::stopped`. §9's row now names
`tests::is_stopped_is_false_until_stop_and_stays_true` in `crates/goad/src/wire.rs`,
beside `a_raised_notice_stays_raised_until_it_is_lowered`. §5.1 names
`Notice::raised` as the precedent for the function and the case.
**Outcome:** `verified` (round 5).

### F-49 — §5.1's `(§9, R10)` resolves to nothing in this document

**Severity:** minor
**Location:** `design.md §5.1`, the sentence *"`start` keeps a clone of `cancel`
before the rest moves into `serve`, exactly as it already keeps one of `pending`
for the glass (§9, R10)"*

**Expected:** `design.md`'s own header fixes the citation forms: *"canon by id
(`SPEC-003 §4`, `ADR-007`, `POL-002`); doc-local refs bare — OQ-1 (§6), D1 (§7),
R1 (§8)."* A bare `§9, R10` is therefore a reference into this document.

**Observed:** this document's §9 is the validation table and carries no ids at
all; its `R`-prefixed ids are §8's risks, R1 through R5. There is no R10
anywhere in `design.md`. The citation is slice 009's — `crates/goad/src/main.rs`
carries *"(`design.md` §9, R10)"* beside the `Debounce` it describes, meaning
that slice's design — and it was copied into this one without the qualifier that
made it resolve.

The claim it supports is true: `main.rs` does construct `pending` in `start`,
hand `&pending` to `install` and `Rc::clone(&pending)` to `SlintGlass::new`. Only
the citation is dead.

**Evidence:** `design.md`'s header comment, *Reference forms*; `design.md §9`
and §8; `crates/goad/src/main.rs`, step 6's comment (*"the callbacks and the
glass must share one map, never hold two (`design.md` §9, R10)"*) and step 7's.

**Disposition:** `fix-now`
**Response:** Verified: design.md has no R10; `main.rs`'s citation is
slice 009's. Replaced by the symbol: `start` hands `Rc::clone(&pending)` to
`SlintGlass::new`.
**Outcome:** `verified` (round 5).

### F-50 — `canon-delta.md`'s SPEC-003 preamble counts, and the count has been outgrown by the changes beneath it

**Severity:** minor
**Location:** `canon-delta.md` §SPEC-003, the preamble line *"Two sections, one
of which is separable from the other."*; the ordering of the three change
subsections

**Expected:** `CLAUDE.md` §Working here — name, never count, and it *"binds
canon too, which is where it rots worst"*. The exemption is a count of something
that cannot grow. The set of sections this delta touches grew once already, in
round 3.

**Observed:** the line reads *"Two sections, one of which is separable from the
other."* It was written when the delta held Change 1 (§7) and Change 2 (§9).
Change 3 landed in §7 and is marked **not separable**, so the sentence's second
half now describes one of three changes rather than one of two, and a reader
meets *one of which is separable* above a file in which two of three are not.
Naming them — §7's two cells, and §9's list, of which only the §9 entry stands
alone — costs a clause and cannot go stale.

Beside it, and for the same reader: the subsections are ordered Change 1,
Change 3, Change 2. The placement is defensible (Change 3 falls due with
Change 1) and the heading says so, but nothing warns a promoter working the file
top to bottom that the numbering is not the reading order.

**Evidence:** `canon-delta.md`, the paragraph under `## SPEC-003 (host event
ingress)` and the three `### Change …` headings in file order; `CLAUDE.md`
§Working here, *Never count*.

**Disposition:** `fix-now`
**Response:** Verified. The preamble now names the changes and does
not count them: §7's R-4 cell (Change 1) and R-3 cell (Change 3), which land
together, and §9 References (Change 2), which stands alone. It also says the
file is in application order, not numbering order. The numbers are not
changed, because this ledger cites them.
**Outcome:** `verified` (round 5).

### F-51 — R-1's new MUST NOT, read with its pronoun, forbids the `Ok` arm the design answers on every ordinary quit

*Raised by the responder while disposing round 4, acting as raiser (Protocol
§Roles), and disposed in the same pass as responder. So its Outcome belongs to
round 5, not to the agent that wrote both halves.*

**Severity:** major
**Location:** `draft-spec.md §4` R-1, closing clause

**Expected:** a normative clause that is applied verbatim at promotion reads
one way, and the design meets that reading.

**Observed:** R-1 closed *"a host MUST decide this on whether a stop was
requested, which it observes, and MUST NOT infer **it** from the call's
success."* *It* can resolve to *that a stop was asked for*. On that reading the
design's `Ok` arm — `Ok` → `Ended::AsAsked` with no request consulted, which is
§5.1's arm and A1's direction — infers volition from the call's success, on
every ordinary quit. What F-23 found is narrower: the call's **error** must not
be read as *nobody asked*. The clause says more than the finding it came from,
and the design does not meet the extra.

**Evidence:** `draft-spec.md §4` R-1 as it stood after round 3; `design.md §5.1`
(the loop call's arms); `design.md §5.5` A1; F-23's Response in this ledger.

**Disposition:** `doc-wrong`
**Response:** Confirmed with the user in round 4's block. The clause now reads
*"MUST NOT read the call's error as evidence that none was"*, which is what F-23
established and what `exit::ended` does. The `Ok` arm's reliance on the quit
route is A1's argument. Its one unrequested path, `controller::Ending::Closed`,
stays R-1's declared residue in R-1's own cell. The alternative was to close
that residue as well by deciding every end on the request alone. It was not
taken: an `Ok` with no request has no error for `Ended::StoppedRunning` to
carry, so the value type would have to change, for a path no production code
reaches.
**Outcome:** `verified` (round 5). The pronoun is repaired. The premise the Response gives for not taking the alternative — *a path no production code reaches* — is F-53's subject.

### F-52 — a case name still asserts that the loop started, the fact F-12 established the process does not observe

*Raised by the responder while disposing round 4, acting as raiser; not yet
disposed.*

**Severity:** minor
**Location:** `design.md §9`, the row naming
`exit_status::a_platform_error_after_the_loop_started_is_1`, and its mutation
list; `draft-spec.md §7` R-2's cell

**Expected:** §9 says test names are commitments, and F-12's repair, with F-29's
after it, removed *the loop began* from every sentence about
`Ended::StoppedRunning`. The draft spec's §5 declares that the host cannot
observe it.

**Observed:** the case builds `Ended::StoppedRunning` around a
`slint::PlatformError` made through `From<String>` and asserts 1. Nothing about a
loop starting is in it, and nothing could be. Its name says *after the loop
started*, in the one kind of place round 2's sweep did not reach: a test name.
The name will be cited from canon once the draft is promoted.

**Evidence:** `design.md §9` (the row and the mutation sentence);
`draft-spec.md §7` R-2's cell; `draft-spec.md §5` *What the seam costs*;
`design.md §5.5` A4; F-12 and F-29 in this ledger.

**Disposition:** `fix-now`
**Response:** Confirmed by the user. Renamed to
`exit_status::stopped_running_is_1` at every site outside this ledger that
names it: `design.md §9` (the row and the mutation list) and `draft-spec.md §7`
R-2's cell. The row and the cell say the error is real, so the name does not
have to. Earlier findings in this ledger keep the old name, because findings
are immutable. The case is not yet written, so no code changes.
**Outcome:** `verified` (round 5).

### F-53 — the loop call answers `Ok` for an end nobody requested, by a route in Slint itself, so A1 is false and `exit::ended` exits 0 for a failure

**Severity:** blocker
**Location:** `design.md §5.5` A1 (*"What does end it with `Ok` is
`quit_event_loop`"*) and the edges table's `Ending::Closed` row; `design.md
§5.2`, `exit::ended`'s doc comment (*"the call returns `Ok` only through the
host's one quit route"*); `design.md §8` R1's failure signals;
`draft-spec.md §7` R-1's cell, *What neither holds*; F-51's Response (*"for a
path no production code reaches"*)

**Expected:** `exit::ended` answers `Ended::AsAsked` for `Ok` without
consulting the request. The design says that is sound because `Ok` means a
stop was asked for (A1), with one declared exception, `controller::Ending::Closed`,
which no production path reaches. R-1's cell declares that exception and no
other. F-51's Response declined the alternative — deciding every end on the
request — on the same premise. For the arm to be sound, `Ok` from
`run_event_loop_until_quit` must be reachable only through `quit_event_loop`.

**Observed:** at the pinned versions it is not. Slint's winit backend ends the
loop with `event_loop.exit()` on its own error path, and can then clear the
error before the loop returns:

- Every `window_event` arm **assigns** `self.loop_error` rather than
  accumulating into it: `RedrawRequested` → `window.draw().err()`, `Resized` →
  `window.resize_event(size).err()`, `CloseRequested`, `Focused`,
  `ScaleFactorChanged` likewise. A success writes `None`.
- At the foot of `window_event`, `if self.loop_error.is_some() {
  event_loop.exit(); }`. `resumed` does the same after a failed
  `create_inactive_windows`.
- winit's `exit()` sets the exit code to `Some(0)`. Wayland's `single_iteration`
  never checks `exiting()`: it delivers every buffered window event, then every
  `RedrawRequested`, then `AboutToWait`, and only `pump_events` looks at the
  exit code afterwards.
- So an event that fails (`loop_error = Some`, `exit()`) followed **in the same
  iteration** by one that succeeds (`loop_error = None`) ends the iteration
  with the exit latched and the error gone. `pump_events` answers
  `PumpStatus::Exit(0)`, `run_on_demand` answers `Ok(())`, and
  `EventLoopState::run` finds `loop_error` empty and answers `Ok`.

No stop was requested, so `Cancel` is untripped. `exit::ended(Ok(()), false)`
answers `Ended::AsAsked`. The host exits **0** and writes nothing (status 0 has
no line), under `Restart = "on-failure"`, which does not restart it. That is
F-46's outcome by a different door: a failure reported as a success, silently.
When the route is `resumed`'s, it is also the loop failing **on entry**, which
draft spec §5 says reports 1.

This is not the slice's measured failure. `research.md` §Thread 3 records
`Error running winit event loop: Exit Failure: 1`, which is winit's own
`set_exit_code` on a dispatch error and still reaches `Err`. It is a second
class of loop ending — a window-level platform error — that the design reads as
a request.

Every site that states the `Ok` arm's premise states it as a single-exception
fact:

- A1: *"What does end it with `Ok` is `quit_event_loop`"*.
- `exit::ended`'s doc: *"the call returns `Ok` only through the host's one quit
  route; the one way that route is reached with no request is the spec's
  declared residue"*.
- R-1's cell: *What neither holds* names `Ending::Closed` alone, and calls it
  production-unreachable.
- §8 R1's signal *"status 0 with no line"* is attributed only to the display's
  loss tripping `Cancel`.

**Established by reading the vendored source, not by running it.** Reaching the
route needs a real backend. What is established is that the code admits it; the
design's claim is that the code does not.

**Routes, not settled repairs** (none verified):
(a) decide `Ok` on the request as well — F-51's declined alternative. It needs a
value for an `Ok` with no request, since `StoppedRunning` carries a
`PlatformError` and there is none.
(b) keep the arm, and declare this route beside `Closed` in A1, `exit::ended`'s
doc, R-1's cell and §8 R1, as a violation of R-1 the host cannot observe. That
leaves exit 0 for a failure, and the residue is no longer an argument about this
repository's code.
The choice is the user's, as F-51's was.

**Evidence:** `i-slint-backend-winit-1.17.1/event_loop.rs` (vendored, pinned):
`window_event`, the arms at lines 219, 222, 241–244 and 253 and the exit at
541–542; `resumed`, 155–157; `EventLoopState::run`, 689–717, the `loop_error`
check at 714. `i-slint-backend-winit-1.17.1/lib.rs` `run_event_loop`, 796–811,
adds no check. `winit-0.30.13/src/platform_impl/linux/wayland/event_loop/mod.rs`:
`run_on_demand`, 185–195 (`Exit(0)` → `Ok(())`); `pump_events`, 225–233;
`single_iteration` from 331, with no `exiting()` check before `AboutToWait` at
515; `exit()`, 667–669 (`Some(0)`). `slint-1.17.1/lib.rs`,
`run_event_loop_until_quit`. `design.md §5.2`, §5.5 A1 and edges, §8 R1;
`draft-spec.md §7` R-1's cell; F-51's Response.

**Disposition:** `fix-now`
**Response:** Confirmed by the responder at the symbol, apart from the raiser:
`window_event`'s arms assign `self.loop_error = window.draw().err()` and its
siblings, the foot calls `event_loop.exit()`, winit's `exit()` sets code 0, and
the Wayland loop checks `exiting()` only between iterations. **The user took
route (a), with `Option`** (`design-log.md`, round 5). `exit::ended` decides on
the request alone — `if stop_requested { AsAsked } else {
StoppedRunning(call.err()) }` — and `Ended::StoppedRunning` carries
`Option<slint::PlatformError>`, `None` when the call answered `Ok`. The host
builds no error the platform did not raise; `report_exit_line` gains a
`StoppedRunning(None)` arm with its own sentence, and the edges table a row for
it. A1 is rewritten: the call's result is not the volition signal in either
direction, and what A1 assumes is only that *requested* is `Cancel` tripped.
`Ending::Closed` stops being R-1's residue — it exits 1 — and R-1's cell loses
its *What neither holds* paragraph and the `quit_event_loop` argument that
served only A1. R-1's MUST NOT now reads *"MUST NOT read the call's result as
evidence either way"*. `exit::ended`'s doc, §5.1's direction list, §5.5's
invariant, §8 R1, §9's rows and mutations, AC-11 and the draft spec's §5 *What
the seam costs* were swept. F-51's Response was right that the value type
changes, and wrong that the path was unreachable. The route stays established
by reading only; no gate command reaches it.
**Outcome:** `verified` (round 6). `exit::ended` as §5.2 writes it, with `status` and `report_exit_line`, compiles under `clippy::pedantic` and `clippy::wildcard_enum_match_arm` over stand-in types, and the four `ended` cases pass. No site reads the call's result as the request, in either direction. The repair's gap one function downstream is F-63.

### F-54 — R-2, §5's diagram and §6 row 1 require 1 for an unrequested `Ok`; `exit::ended` answers 0, and only R-1's cell says so

**Severity:** major
**Location:** `draft-spec.md §4` R-2; `§5` state diagram (`Reached --> Unasked:
the call ended, none asked`, then `Unasked --> [*]: 1`); `§6` row 1; `§7` R-2's
cell; `design.md §9`, the row naming `ended::a_loop_that_returned_ok_is_as_asked`

**Expected:** F-51's Response settled that an `Ok` with no request answers 0,
and R-1's cell calls that a *violation* of R-1 rather than an exception. The
normative text around it has to say one thing about that end, and §7's preamble
requires each row to say what it does not hold.

**Observed:** the requirement text partitions on the request alone; the design
partitions on the request only for `Err`.

- R-2: *"MUST exit **1** when it reached the call … and that call **ended** and
  no stop had been requested"* — *ended*, not *ended in error*. `Ok` with no
  request satisfies R-2's antecedent, so R-2 requires 1; `exit::ended` answers
  0.
- §5's diagram routes *"the call ended, none asked"* to 1 and has no edge for
  `Ok` with no request going to 0.
- §6 row 1: *"that call then ended, and no stop had been asked for"* → 1.
- `Ended::StoppedRunning`'s doc (§5.2) says *"ended **in error**"*, so the type
  and the spec describe different sets.

R-1's cell declares the `Closed` route a violation **of R-1**. It also violates
R-2, and R-2's cell is silent, so a reader of R-2 and its row cannot learn that
the host answers 0 where R-2 requires 1. Before F-53 this was a text mismatch
over an unreachable path; after F-53 it is a reachable one.

The case `ended::a_loop_that_returned_ok_is_as_asked` inherits the ambiguity.
§9 does not say which `stop_requested` it passes, and neither choice is sound:

- with `true`, it holds nothing the `Err` cases do not, and no mutation
  distinguishes the arm's one property, *`Ok` is not consulted*;
- with `false`, it pins as **required behaviour** the end R-1's cell calls a
  violation and R-2 requires to be 1 — a test asserting a spec breach.

**Evidence:** `draft-spec.md §4` R-1 and R-2; `§5` diagram; `§6` row 1; `§7`
R-1's cell (*a **violation** of this requirement rather than an exception to
it*) and R-2's cell; `design.md §5.2`, `Ended::StoppedRunning`'s doc and
`exit::ended`'s body; `design.md §9`, the `ended::a_loop_that_returned_ok_is_as_asked`
row; F-51's Response.

**Disposition:** `doc-wrong`
**Response:** Closed by F-53's route, which makes the design meet the spec as
written rather than amending the spec. R-2, §5's diagram and §6 row 1 already
said *the call ended and no stop was requested → 1*, and now the design answers
1 there. `Ended::StoppedRunning`'s doc no longer says *ended in error*. The
`Ok` case is split in two, each naming its request:
`ended::a_loop_that_returned_ok_with_no_stop_requested_is_stopped_running` and
`ended::a_loop_that_returned_ok_after_a_requested_stop_is_as_asked`. R-2's cell
cites the first beside the `Err` case, and §9's mutation list names the F-53
shape as what the first reds.
**Outcome:** `verified` (round 6). R-2, §5's diagram, §6 row 1 and R-2's cell now agree with `exit::ended`; the split `Ok` cases name their request at every site.

### F-55 — the re-cut scan's "derived" limit is still an enumeration: any re-filing off the call's line is invisible, not only a re-filing of the binding

**Severity:** major
**Location:** `design.md §5.2`, *What the case does not reach* (*"That is the
whole of its limit"*); `design.md §8` R2; `draft-spec.md §7` R-2's cell, *What
that scan does not reach*, *"derived from the matcher because a line scan's
limit is the whole of what it costs"*

**Expected:** F-43's Response replaced a hand-listed limit with one *"derived
from the matcher"*. The matcher reads exactly one thing: the `code_of`-stripped
text of lines that name `run_event_loop_until_quit`. Derived from that, the
limit is **everything not written on the call's line**. The R-2 cell goes into
canon verbatim.

**Observed:** all three sites state the limit as one instance of that class:
*a later statement that re-files the call's **binding***. The re-filing R2
guards against is the loop's ending travelling as a startup failure, and that
can be written anywhere downstream of the call, mostly without touching the
binding. For example, in `start`:

```rust
  let call = slint::run_event_loop_until_quit();
  match exit::ended(call, stop_requested.is_stopped()) {
    Ended::StoppedRunning(error) => Err(StartupError::Platform(error)),
    done @ Ended::AsAsked => Ok(done),
  }
```

The call's line is the sanctioned one, so the case is green; every renderer
case is green, since none reaches `start`; the binary tier cannot reach the
loop. The host exits 2 for the loop's ending — the defect the slice exists to
repair. The same move in `run` or `main`, over the `Ended` that `start` answers,
is equally invisible. §5.5's review invariant is *"no other site constructs
`Ended::StoppedRunning`"*; this constructs `StartupError::Platform`, so that
invariant does not name it either.

So *"That is the whole of its limit"* is false, and *"derived from the matcher"*
describes an enumeration of one — F-43's class again, inside F-43's repair.

**Verified:** the snippet, over stand-in types (`PlatformError`, `StartupError
{ Platform, … }`, `Ended`, and `ended` as §5.2 writes it), compiles with
`rustc --edition 2024` under `#![deny(clippy::wildcard_enum_match_arm)]` and
answers `Err(Platform(…))`. Not compiled in the tree.

**Route, not a settled repair:** state the limit as the matcher gives it — the
case reads only the call's own line, so a re-filing anywhere else, of the
binding or of the `Ended` it becomes, is invisible — and name `start`'s, `run`'s
and `main`'s handling of `Ended` as review in R-2's cell.

**Evidence:** `crates/goad-boundary/tests/checks/structure.rs`,
`occurrences_where` and `production_lines` (one stripped line per predicate
call); `design.md §5.2`, §5.5's invariants, §8 R2; `draft-spec.md §7` R-2's
cell; F-43's Response.

**Disposition:** `doc-wrong`
**Response:** Taken as the raiser routed it. `design.md` §5.2, §8 R2 and R-2's
cell state the limit as the matcher gives it: the scan reads the call's own line
and nothing else, so a re-filing anywhere off it — of the binding, or of the
`Ended` that `exit::ended` answers, in `start`, `run` or `main` — is invisible.
*"That is the whole of its limit"* is gone. R-2's cell and §5.5's invariant add
to review that nothing downstream of `exit::ended` turns an `Ended` into an
`Err`.
**Outcome:** `verified` (round 6). The limit is stated as a class at all three sites, and the review claim is in R-2's cell and §5.5. A residual imprecision in *what the matcher reads* is F-68.

### F-56 — `exit::status`'s doc comment names `report_startup`, which this slice removes

**Severity:** minor
**Location:** `design.md §5.2`, the `exit.rs` block, `status`'s doc comment

**Expected:** F-34 was raised because `report_startup_line`'s doc named
`report_startup`, *"which this slice removes"*, and §5.2 now re-anchors that
doc *"to the outlet that survives"*. A doc comment a phase agent copies verbatim
must not name a function the same slice deletes.

**Observed:** `exit::status`'s doc ends *"`main` widens — the same pure/impure
cut `report_startup_line` and `report_startup` already make."* After this slice
`report_startup` does not exist — `report_exit` replaces it (§5.2,
`slice-010.md` §Scope) — and the pair making the cut is `report_exit_line` /
`report_exit`. F-34's class, at a second site its repair did not reach.

**Evidence:** `design.md §5.2`, `status`'s doc comment and the paragraph
beginning *"`report_startup_line` keeps its name"*; `crates/goad/src/diagnostics.rs`,
`report_startup`; F-34.

**Disposition:** `doc-wrong`
**Response:** `exit::status`'s doc now names the pair that survives:
*"the same pure/impure cut `report_exit_line` and `report_exit` make"*.
**Outcome:** `verified` (round 6).

### F-57 — §3 says *journal* may not appear in the new module's comments; the vocabulary scan does not read comments

**Severity:** minor
**Location:** `design.md §3`, the domain-vocabulary bullet (*"that word may not
appear in the new module's comments"*); `notes.md` §Harvest → Learned, *the
vocabulary scan's shape*

**Expected:** a constraint the design places on a phase agent is true of the
instrument it cites.

**Observed:** `Scan::inspect` tests each line with `goad_boundary::scan::mentions`,
whose first step is `code_of`; `mentions`'s own doc says *"Comment text is cut
off first by `code_of` — a vocabulary check is about what the code names"*. A
`//` or `///` comment in `exit.rs` saying *journal* is not a breach. What the
scan does forbid is the word in code and in **string literals**, which `code_of`
keeps — the relevant surface for the new stderr sentence, not for comments. The
rule as written forbids what the gate allows and is silent where it applies.

The *checked and clean* entry (*"forbidden in `exit.rs`"*) is true of code and
does not repeat the comment claim; it is listed because it is what a later
agent reads in place of the design.

**Evidence:** `crates/goad-boundary/src/scan.rs`, `Scan::inspect`, `mentions`
and its doc, `code_of` (*"comments removed, string literals intact"*);
`crates/goad-boundary/tests/checks/vocabulary.rs`, `DOMAIN` and `domain_scan`;
`design.md §3`.

**Disposition:** `doc-wrong`
**Response:** `design.md` §3 now forbids the word in the new module's code and
string literals, the stderr sentences included, and says a comment is not a
breach because `mentions` cuts comments through `code_of`. The *checked and
clean* entry in `notes.md` says the same and records that it missed it.
**Outcome:** `verified` (round 6). The comment rule is repaired; the sentence places the stderr sentences in the new module, which is F-66.

### F-58 — Change 1's replacement opens *"held one tier down"*, a term SPEC-003 does not have and the slice uses for the other tier, and says *meanwhile* in evergreen canon

**Severity:** minor
**Location:** `canon-delta.md` Change 1, *What it will say*

**Expected:** text applied to SPEC-003 verbatim uses terms SPEC-003 defines or
that read plainly, and states what is true now.

**Observed:**

- The bold lead is *"**The non-zero exit is held one tier down, plus review.**"*
  SPEC-003 has no tier vocabulary. In this slice *one tier down* means the
  renderer tier below the binary tier (`design.md §4` second principle, §5.1;
  `draft-spec.md §7` R-4's cell), yet the first case the sentence names,
  `exit_codes::an_unbindable_ingress_path_exits_2`, is binary tier — the tier
  that is **not** one down. A promoted reader gets a term with no referent; a
  slice reader gets the wrong one.
- *"What holds the status **meanwhile** is the shape of `exit::status`"* is a
  time word. In canon it reads as an interim arrangement, and canon states what
  is true now (`docs/AGENTS.md` §Documentation). The meaning appears to be *for
  the variants no case reaches*.

**Evidence:** `canon-delta.md` Change 1; `grep -rn "tier down" docs/specs
docs/policy docs/adr` returns nothing; `design.md §4`; `draft-spec.md §7` R-4's
cell (*"held one tier down, by `stderr_outlets::…`"*).

**Disposition:** `doc-wrong`
**Response:** Change 1's lead is now *"held by a case on the built binary, by
the shape of the classifier, and by review"*, which uses no tier term and
names each holder in the order the cell cites them. *Meanwhile* is replaced by
*"For the variants no case reaches"*.
**Outcome:** `verified` (round 6). Checked against live SPEC-003: the replaced sentence occurs once, and the lead's holders are cited in the order it names them.

### F-59 — Change 3 keeps R-5 in the analogy on a citation that supports a different claim

**Severity:** minor
**Location:** `canon-delta.md` Change 3, *What it will say*, the paragraph
beginning *"R-4 leaves the analogy"*

**Expected:** Change 3's replacement is sound against the live cell — the phrase
is unique and the result is well-formed, both checked. What remains is the claim
the retained half makes, since the delta vouches for it.

**Observed:** after Change 3, R-3's cell says `LivenessUnknown` is something
*"no cooperating test in this workspace can produce … the same position as
R-5's process exit"* — a position of **unreachability by any cooperating
test**. The delta supports it with R-5's cell: *"process exit is not `Drop`, so
nothing here speaks for a killed host"*. That sentence says what R-5's **case**
does not hold, not that no cooperating test **can** reach process exit — the
distinction this slice's own R-2 cell draws (*"a clause no cooperating test
**can** reach, as distinct from one no case has yet been written for"*). The
workspace can spawn and kill processes (`crates/goad/tests/binary/`, and the
forked child in
`ingress::a_socket_a_forked_child_still_holds_is_reclaimed_and_the_new_listener_serves`);
whether a killed host is reachable is not established either way.

The retained phrase is pre-existing canon, and the delta need not settle it. It
should not certify it with a citation that does not carry it — the pattern
Change 1 removes from R-4's cell, an unreachability claim nobody re-read.

**Evidence:** `docs/specs/003-host-event-ingress.md §7`, R-3's cell (the
`LivenessUnknown` sentence) and R-5's cell; `canon-delta.md` Change 3;
`draft-spec.md §7` R-2's cell.

**Disposition:** `doc-wrong`
**Response:** Change 3 no longer cites R-5's cell in support. It leaves R-5's
half of the analogy standing, says in terms that the delta does not vouch for
it, and names it as R-5's cell's question, unsettled by this slice. The
retained phrase is pre-existing canon, and this slice does not amend it.
**Outcome:** `verified` (round 6). The delta no longer cites R-5's cell in support. The retained phrase is still unsupported canon; the Response says so, which is what the finding asked.

### F-60 — §9's *"The first two must be spelled so the mutated build compiles"* resolves to the wrong mutations

**Severity:** minor
**Location:** `design.md §9`, *Mutations the plan should confirm are caught*

**Expected:** the pre-round-5 consistency pass added a rule for how two scan
mutations are spelled; a phase agent applies it to the mutations it names.

**Observed:** the paragraph lists, in order, the `=> 1` → `=> 2` status arm, the
`report_exit_line` arm, the `Err(_) => 2` arm, the restored
`.map_err(StartupError::Platform)?`, the imported-variant re-filing, and a
second production call. *"The first two"* are the status arm and the
`report_exit_line` arm; the spelling rule that follows is about neither. The
next sentence, *"Each of those reds nothing else"*, resolves the same way and is
false of them — `Err(_) => 1` is listed as reddening `every_startup_failure_is_2`
**and** the binary tier. The intended referents are the two re-filings of the
call's line. A positional count, which `CLAUDE.md` §Working here rules out for
a list that has grown twice in this review.

**Evidence:** `design.md §9`, the mutation paragraph; `notes.md` §Handover,
*Pre-round-5 consistency pass*, first bullet.

**Disposition:** `doc-wrong`
**Response:** §9's mutation paragraph is now a list grouped by what each
mutation targets — the classifier, the line, the scan, the decision, the new
binary case. The compile-spelling rule names *the two re-filings*, and
*"reds nothing else"* names *each scan mutation*. No positional reference is
left.
**Outcome:** `verified` (round 6). No positional reference remains; *each scan mutation reds nothing else* is true of each of the three.

### F-61 — F-46's sweep left *somebody asked* in §5.1

**Severity:** nit
**Location:** `design.md §5.1`, *What this buys and what it does not*

**Observed:** *"the status no longer reports *nobody asked* for an end
**somebody** asked for"*. F-46's Response says every site that followed R-1 was
swept to *asked*. *Somebody* attributes the request to an agent — the reading
F-46 removed; *a stop that was requested* is the observed fact. The nearby
*nobody asked* (A1, A5's heading) reads as *no request* and is arguable;
*somebody* is not.

**Evidence:** `design.md §5.1`; `draft-spec.md §4` R-1's definition of
*asked*; F-46's Response.

**Disposition:** `doc-wrong`
**Response:** §5.1's sentence now reads *"no longer reports *nobody asked* for
an end that was requested, nor *as asked* for one that was not"*, rewritten
with F-53's second direction.
**Outcome:** `verified` (round 6).

### F-62 — `stop_requested` names a `Cancel`, so the seam reads `stop_requested.is_stopped()`

**Severity:** nit
**Location:** `design.md §5.2`, the seam block (`let stop_requested =
cancel.clone();`)

**Observed:** the binding holds the signal, not the fact. At the call site it
reads as a predicate asked a predicate, and `exit::ended`'s parameter of the
same name is the `bool` — one identifier for two types, one line apart. A name
for the handle (`stop_signal`) leaves `stop_requested` to the `bool`, which is
what `exit::ended` calls it.

**Evidence:** `design.md §5.2`, the seam block and `exit::ended`'s signature;
`CLAUDE.md` §Code Standards, *naming things well*.

**Disposition:** `doc-wrong`
**Response:** The handle is `stop_signal`; `stop_requested` is left to
`exit::ended`'s `bool`.
**Outcome:** `verified` (round 6). `stop_signal` at §5.2; no other artefact names the handle.

### F-63 — `exit::status` over `Ended::StoppedRunning(None)` is asserted nowhere, so F-53's exit 0 can return one function downstream with every case green

**Severity:** major — a design defect: a verification claim that is wrong
**Location:** `design.md §9`, the `exit_status::stopped_running_is_1` row and
*Mutations the plan should confirm are caught* (*The classifier*, *The
decision*); `draft-spec.md §7` R-1's cell (*the cases in `exit_status` are the
whole of the **shapes** it can see*) and R-5's cell (*the requirement's own
falsifier*); `slice-010.md` AC-4 (*every **shape** that function can see is
asserted one tier down*)

**Expected:** F-53's repair gave `Ended::StoppedRunning` an `Option`, so
`StoppedRunning(None)` is a shape a pattern in `exit::status` can see and
distinguish. It is also the exact end F-53 exists to move from 0 to 1. R-1's
cell says the *only if* half of R-1 is held one tier down because the
`exit_status` cases cover every shape `exit::status` can see, R-5's cell says
those cases are R-5's falsifier, and AC-4 requires every shape asserted.

**Observed:** the only case that feeds `exit::status` a *stopped running* value
is `exit_status::stopped_running_is_1`, and §9 and R-2's cell both say it builds
the value around a real `PlatformError` — `StoppedRunning(Some(_))`. The
`ended` cases hold that the F-53 route produces `StoppedRunning(None)`; nothing
holds what `exit::status` answers for it. This mutation:

```rust
  match outcome {
    Ok(Ended::AsAsked | Ended::StoppedRunning(None)) => 0,
    Ok(Ended::StoppedRunning(Some(_))) => 1,
    Err(_) => 2,
  }
```

compiles under the crate's deny of `clippy::wildcard_enum_match_arm` (it has no
wildcard), passes every case §9 names, and ships exit 0 with the no-error line
for the F-53 route — the outcome round 5 was reopened to remove, reached through
the classifier instead of the decision. None of §9's mutations names it, and
R-1's *only if* claim, R-5's falsifier claim and AC-4 are each false of the
design as specified.

**Verified:** over stand-in types under `/tmp` — `PlatformError` as a
`#[non_exhaustive]` enum with `From<String>` and `Display`, `StartupError`,
and `Ended`, `ended`, `status`, `report_exit_line` as §5.2 writes them. The
file passes `cargo clippy --all-targets` under
`#![deny(clippy::wildcard_enum_match_arm, clippy::pedantic)]`, including the
mutant. Analogues of `as_asked_is_0`, `stopped_running_is_1`,
`every_startup_failure_is_2` and the four `ended` cases all pass against the
mutant, and `status_mutant(&Ok(ended(Ok(()), false)))` answers 0.

**Route, verified in the same stand-in:** a case asserting `exit::status`
answers 1 for `Ok(Ended::StoppedRunning(None))` — a second
`exit_status` case, or a second assertion in `stopped_running_is_1` — passes on
§5.2's `status` and reds the mutant. §9's row, its *classifier* mutation list,
and R-1's, R-2's and R-5's cells would name it. Not checked in the tree.

**Evidence:** `design.md §5.2` (`Ended`, `status`); `design.md §9`;
`draft-spec.md §7` R-1, R-2 and R-5 cells; `slice-010.md` AC-4; F-53's Response.

**Disposition:** `doc-wrong`
**Response:** Confirmed by the responder: the only `exit_status` case over
*stopped running* builds `StoppedRunning(Some(_))`. Repaired as routed, the
user confirming. `exit_status::stopped_running_with_no_error_is_1` is added to
§9; the classifier mutations name the split-arm mutant as what reds it, and the
`=> 2` mutant as reddening both *stopped running* cases. R-1's cell names the
`exit_status` cases it calls the whole of the shapes, R-2's cell cites the new
case beside `stopped_running_is_1`, and R-5's cell names both as the *stopped
running* shapes. AC-4's text is unchanged and now true. No design change: the
repair is a case and its citations. The mutation is the plan's to run in the
tree; the raiser's stand-in is the only evidence so far.
**Outcome:** `verified` — by site check, not a seventh round (see *Close*)

### F-64 — the no-error line is outside R-6's pair, and §9's *line* mutation no longer says which arm

**Severity:** minor — no wrong behaviour; a verification row argues itself
vacuous and a mutation lost its referent
**Location:** `draft-spec.md §7` R-6's cell, its last sentence; `design.md §9`,
*The line* mutation

**Expected:** R-6 binds the line accompanying *stopped running*, and F-53's
repair gave that class two lines. R-6's cell holds the first as a **pair**
*"because either half alone is vacuous"* — a pinned sentence is a snapshot until
a second case asserts it differs from a never-started line.

**Observed:**

- R-6's cell pins the no-error sentence with
  `stderr_outlets::a_host_that_stopped_running_with_no_error_says_it_had_been_running`
  alone, and gives as its reason that the end *"carries no error to confuse with
  a startup failure's"*. R-6's MUST NOT is about the **line** a host that never
  started writes, not about the error it carries, so the reason does not address
  it; by the cell's own first sentence the no-error line is held by a snapshot.
- §9: *"Making `report_exit_line`'s *stopped running* arm answer
  `report_platform_line`'s sentence must red the pair."* There are now two such
  arms. For the `StoppedRunning(None)` arm the claim is false: that mutation
  reds the no-error pin, and neither case of the pair.

**Route, not verified:** have the pair's second case assert both stopped lines
differ from `report_startup_line` over `StartupError::Platform`, or say in R-6's
cell why a snapshot is enough for the constant line; name the arm in §9's
mutation and add the `None` arm's.

**Evidence:** `draft-spec.md §4` R-6, `§7` R-6's cell; `design.md §5.2`
`report_exit_line`; `design.md §9` rows and *The line*.

**Disposition:** `doc-wrong`
**Response:** Taken by the first route. The distinctness case now asserts that
**each** *stopped running* line differs from `report_startup_line` over
`StartupError::Platform`, so R-6's row holds both sentences as pairs, and the
cell's reason that did not address R-6's MUST NOT is gone. §9's row says the
pair makes both pins claims. §9's *line* mutation names each arm
(`StoppedRunning(Some(_))`, `StoppedRunning(None)`) and what each reds.
**Outcome:** `verified` — by site check, not a seventh round (see *Close*)

### F-65 — A1 and the `Ending::Closed` edge say `Closed` is `StoppedRunning(None)`; A5's own mechanism makes it `Some`

**Severity:** minor — the status is right either way; a stated value is not
**Location:** `design.md §5.5` A1, the paragraph on `Ending::Closed` (*"so it is
`Ended::StoppedRunning(None)`"*); the edges table's `Ending::Closed` row (*"status
1 by the row above"*, the row for a call that answers `Ok`)

**Observed:** A1 derives `None` from *ends the loop through `quit_event_loop`*.
A5 states that `about_to_wait` latches `loop_error` without exiting and that
`EventLoopState::run` checks it after `run_app_on_demand` returns. So a
`quit_event_loop` after a latched error answers `Err`, and `Closed` then is
`StoppedRunning(Some(_))`, with the other line. `exit::ended` handles both; A1
states one. `main.rs`'s `start` calls `quit_event_loop` after `serve` returns
whatever the `Ending`, so the route is the one A1 names.

**Route:** *it is `Ended::StoppedRunning`, carrying `None` unless the backend
had latched an error (A5)*; the edge row cites both call results rather than
*the row above*.

**Evidence:** `design.md §5.5` A1, A5 and edges; `crates/goad/src/main.rs`
`start`, the spawned task; `controller::Ending::Closed`.

**Disposition:** `doc-wrong`
**Response:** A1 now says `Closed` is `Ended::StoppedRunning` carrying `None`
unless the backend had latched an error before the quit (A5), when it carries
that. The edges row says `Ok(Ended::StoppedRunning(_))`, status 1, the line
for whichever result the call answered, instead of pointing at *the row
above*.
**Outcome:** `verified` — by site check, not a seventh round (see *Close*)

### F-66 — the second *stopped running* sentence is not swept: four sites still say *the new line* or *the new sentence*, and §3 places the sentences in the wrong module

**Severity:** nit
**Location:** `design.md §5.5` edges, the row *"the loop ends unasked"*;
`design.md §5.2` *"The new sentence names the phase and not the cause"*;
`design.md §5.3` *"including the new sentence"*; `slice-010.md` §Scope,
*"the stderr line the new class writes"*; `design.md §3`, the vocabulary bullet

**Observed:**

- The edges row *"the loop ends unasked | `Ok(Ended::StoppedRunning)`, status 1,
  the new line"* overlaps the row two below it (*"the loop call answers `Ok` and
  no stop was asked for"*), which is also unasked, and names one line where §5.2's
  table has two. It reads as the `Err` case written before F-53.
- §5.2, §5.3 and §Scope each speak of one new sentence.
- §3, repaired for F-57, forbids the word *"in the new module's code or string
  literals — the new stderr sentences included"*. The sentences are in
  `diagnostics.rs`, not `exit.rs`. The scan reads both, so the rule binds; the
  sentence locates them wrongly.

**Evidence:** `design.md §3`, §5.2's stderr table and `report_exit_line`, §5.3,
§5.5 edges; `slice-010.md` §Scope.

**Disposition:** `doc-wrong`
**Response:** The superseded edges row *"the loop ends unasked"* is deleted; the
two rows for the call's results cover it. §5.2 and OQ-2 say *both new
sentences*, §5.3 *the two new stopped running sentences*, and `slice-010.md`
§Scope names both lines. §3 now binds *code or string literals this slice
writes — `exit.rs`, and the new stderr sentences in `diagnostics.rs`*.
**Outcome:** `verified` — by site check, not a seventh round (see *Close*)

### F-67 — R-1's new normative sentence, read unscoped, makes the request decide 0 for an invocation that was a question

**Severity:** minor — the design's behaviour is right; canon text applied
verbatim says less precisely what it means
**Location:** `draft-spec.md §4` R-1, *"Whether a stop was asked for decides
between 0 and any other status, however the event-loop call reports its end"*

**Observed:** the sentence names no subject. Read on its own it says the request
decides 0 for every end, and `--help` exits 0 with no stop asked for — the first
edge R-1 itself defines. *"however the event-loop call reports its end"* implies
a host that reached the call, and AC-11 says *"between 0 and 1"*, which is the
scope meant. The rest of R-1 is scoped (*a running host*).

**Route:** *"For a host that reached the call that runs its event loop, whether
a stop was asked for decides between 0 and 1, however …"*.

**Evidence:** `draft-spec.md §4` R-1, R-2; `slice-010.md` AC-11;
`design.md §5.5` edges, `--help`.

**Disposition:** `doc-wrong`
**Response:** Taken as routed. R-1 now reads *"For a host that reached the call
that runs its event loop, whether a stop was asked for decides between 0 and 1,
however that call reports its end"*, which is AC-11's scope.
**Outcome:** `verified` — by site check, not a seventh round (see *Close*)

### F-68 — *"it reads the call's own line and nothing else"* is false of the count half, which reads every production line naming the function

**Severity:** nit
**Location:** `design.md §5.2`, *What the case does not reach*; `design.md §8`
R2; `draft-spec.md §7` R-2's cell, *What that scan does not reach*

**Observed:** F-55's repair derives the limit from what the matcher reads, and
states that as *the call's own line and nothing else*. The case has two halves
(§5.2): the count reads every production line that names
`run_event_loop_until_quit`, and the shape reads that one line. A re-filing
written on another line that names the function — a second call, or
`use slint::run_event_loop_until_quit;` — reds the count; §5.2 says so in its sentence
beginning *What it does forbid*. The derived limit, *invisible anywhere off the call's line*,
is right for every re-filing that does not name the function, which is all that
matters for R2. Only the premise overstates.

**Route:** *it reads only lines that name the function, and requires there to be
one* — or *reads nothing but lines naming the call*.

**Evidence:** `design.md §5.2`, the case's rule and *What the case does not
reach*; `crates/goad-boundary/tests/checks/structure.rs`, `occurrences_where`.

**Disposition:** `doc-wrong`
**Response:** All three sites now name both halves: the count half reads every
production line naming the function, the shape half reads the call's line, so
the limit is a re-filing off that line **that does not name the function**.
**Outcome:** `verified` — by site check, not a seventh round (see *Close*)

## Synthesis

**Written at F-10's close, and extended at F-11's.** Round 1 is closed: every
finding it raised is `verified`.

**Round 1** — eleven findings disposed: three blockers, four majors, three
minors, one nit. None was withdrawn and none was contested. Every repair is a
repair to a document; no acceptance criterion was dropped, and the two fenced
decisions — the phase axis, and no transient split inside *never started* — were
not reopened and no evidence was found that bears on either.

**What the review changed, in one sentence each.**

The design's central type carried a doc comment that was false of it (F-1). `run`
answers `Ok(Ended::AskedToStop)` for `--help` before the event loop exists, so
*"only reachable once the event loop has begun"* and *"`Ok` here means the host
started"* were both wrong — in a slice whose entire purpose is a type whose doc
comment lied. The `Result`'s channels are not a three-way phase seam: `Err` is
*never started* whole and `Ok` holds the other two. The type is now
`Ended::AsAsked`, the spec's class is *as asked*, and D1 says what the shape
does and does not buy.

Canon was about to acquire two false sentences and did not. `canon-delta.md`'s
replacement text — applied verbatim to SPEC-003 at audit — attributed R-4's
failure to R-3's in-use case and called it unreachable without a second live
host (F-2); R-4's failures settle in `startup::listener` before the first Slint
call and are merely **uncovered**. The draft spec's R-4 cell claimed four binary
cases assert an exact line and an empty standard output (F-3); one of the four
does. Both are the exact defect class `canon-delta.md` exists to repair, found
inside the repair.

Three claims were stated at a scope or an attribution where they were false
(F-4, F-8, F-7), and one criterion was adopted and then discharged in the wrong
section (F-6). The two-tier cut this design moves is now said to move, instead
of leaving two module docs in the tree asserting the old one (F-5). Three doc
sites the slice falsifies joined the change list (F-9, F-10).

**What the review added.** One gate case,
`structure::the_loop_s_ending_is_never_a_startup_failure`. It came out of F-4
and is worth recording as a method note: the instrument first proposed was a
uniqueness scan mirroring `quit_event_loop`'s, and checking it before
recommending it showed it would not catch §8 R2's regression at all — a
re-filing keeps the call-site count at one. The case that landed scans for the
re-filing spelling instead. A finding can be right that nothing holds a property
and wrong about what would hold it.

**What it knowingly leaves standing.**

- **The scan's own limit.** A re-filing spread over more than one line, or
  routed through an alias, is invisible to it. Named in the case, in `design.md`
  §5.2 and in the draft spec's R-2 cell — the risk is smaller than it was and it
  is not gone.
- **R-4's uncovered exit, and R-3's analogy with it.** Deferred with the user's
  endorsement to a `slice-010.md` §Follow-ups row carrying a kill condition.
  Until it lands, SPEC-003/R-3's *"same position"* cross-reference is true only
  in the weak sense, and `canon-delta.md` says so rather than letting the
  stronger reading stand.
- **A1's `Ending::Closed` residue and A2's untested call site**, unchanged by
  this review: both were already declared in terms, which is why neither became
  a finding. AC-9 is still the only evidence for the slice's central claim.
- **D5's residue** — nothing in canon stops `nix/module.nix`'s comment drifting
  back to a retryability argument. Priced and foreseen in the design; not this
  review's to close.

**F-11, raised while sweeping round 1's repairs, is the thesis in miniature.**
§6's status-1 row told a supervisor the host's backend had been *reachable* — a
fact about the user's configuration nothing in `start` ever asks for, since the
backend is two stored fields until `serve` runs inside the loop. And the repair
had to be checked against the same standard as the defect: the proposed
replacement said the host's window *opened*, which is false of a tray-resident
host that sat at `Surface::Hidden` and never showed one. Both the finding and
its first repair reached for an unobserved fact. The row now says only what the
process watched happen.

**What this synthesis is not.** The repairs above have not themselves been
reviewed. Four of them rewrote normative text, and the project's own history is
that the commit repairing a false claim is where the next one is written. Round
2 is owed, and F-11 is now part of what it must attack rather than the reason it
cannot start.

## Synthesis — round 2

**Round 2's subject was round 1's repairs, and the thesis held.** Eleven
findings: two blockers, four majors, four minors, one nit. None withdrawn, none
contested. **Both blockers were written by round 1** — F-14's ambiguity is the
phrase F-7's repair introduced, and F-15's misattribution is inside
`canon-delta.md`, the document that exists because a sentence rotted in SPEC-003.
The project's stated reason for reviewing repairs is no longer a prediction.

**What round 2 changed, in one sentence each.**

*The seam moved from the loop to the call.* F-12 is the finding the slice will
be remembered by: the host derives *stopped running* from
`run_event_loop_until_quit` answering `Err`, and that answer does not establish
the loop ever began — `EventLoopState::run` can fail before `run_app_on_demand`
is reached, and `with_platform` can fail to select a platform at all. Four sites
asserted the loop began, including the §6 row repaired **in round 1** for the
same class of defect one clause over. R-2 and R-3 now cut at *did the process
reach the call*, which is a fact it can observe, and §5's *What the seam costs*
declares what that leaves: a loop that fails on entry is reported as *stopped
running* and the host cannot tell. The axis stopped claiming it *"cannot get
wrong"* and started claiming what is true and is the stronger argument — one
imprecision, at a named seam, that does not grow when a new cause arrives.

*Two requirements stopped admitting the same end* (F-14), and one universal
about the requirement set was replaced by naming its members rather than
correcting a count (F-13).

*Canon was again about to acquire a false sentence and again did not* (F-15).
The replacement block credited a named test with a universal the draft spec says
it does not hold; it now attributes the universal to the arm's shape and says in
terms that **no test holds it**.

*The gate case grew a second needle* (F-16). `?` applied directly to the call,
once `StartupError` gains a `From<slint::PlatformError>`, re-files the loop's
ending on one line naming `StartupError` nowhere — a likelier route than either
limit the design had named, because `?` is the idiom OQ-1 chose the `Result`
shape to preserve. The case reds on it now; the two real limits are still
stated.

*The rest were claims stated where they were false*: three surfaces described as
out of scope that §Scope names (F-17), §7's citations written in the present
indicative for cases that do not exist (F-18), a count inside the repair that
stopped counting (F-19), a module cited by a name it does not have — in a list
marked *checked and clean* (F-20), an inference about the environment that two
`StartupError` variants do not support (F-21), and a principle binding both
binaries that named one binary's tests (F-22).

**A method note, from F-16 and F-20 together.** Round 1 recorded both surfaces
in `notes.md` §Harvest as *checked and clean*, so a later stage would not pay
for them twice. Both entries were substantively right and one cited a symbol
that does not exist. A *checked and clean* list is read by agents who have been
told not to look again, which makes a wrong symbol in it cost more than no entry
at all — and its claims are not re-derived by anything downstream. The list is
worth keeping and worth auditing at the same standard as the artefact.

**What round 2 knowingly leaves standing.** The line scan's two real limits,
multi-line and alias, unchanged and still stated. The residue F-12 declares: a
loop that fails on entry reports 1, and AC-6's line text is unchanged, because
the common case is the one it describes. A1's converse now rests on
`set_event_loop_quit_on_last_window_closed(false)` — a property of the pinned
version, said to be that rather than an API guarantee.

**What this synthesis is not.** Round 2's repairs are themselves unreviewed, and
round 2 exists because that sentence was true of round 1. Round 3 is owed. Its
subject is narrower: F-12's seam is a normative recut of two requirements and a
new §5 paragraph, F-16 changed an instrument's specification, and F-18 added a
promotion gate — three places where this round wrote new normative text.

## Synthesis — round 3

**Round 3's subject was round 2's repairs. Two blockers, five majors, five
minors; none withdrawn, none contested. Four of the twelve are defects round 2's
repairs introduced.** Every round so far has found a blocker inside the previous
round's repairs, which is now a measured property of this slice and not a
prediction.

**F-23 is the round's finding, and it is F-12 in the other half of the same
sentence.** The sentence round 2 repaired asserted two things the channel does
not carry: *the loop began*, and *nobody asked*. Round 2 removed the first and
left the second. `about_to_wait` latches `loop_error` and, unlike
`window_event`, does not exit the loop; `EventLoopState::run` checks it after
`run_app_on_demand` returns. So a latched error followed by a requested quit
answers `Err`, and the host exits 1 for an end R-1 requires 0 for.

It closed in code rather than in prose, because the host already held the fact —
`Cancel` keeps its own receiver, so *a stop was asked for* needed a read, not a
channel. The residue pile got one entry shorter instead of one longer, which is
the first time in this review that a finding about an unobserved claim was
answered by observing it.

**The instrument stopped being widened and was swapped** (F-28). Round 1
proposed a scan that would not have caught the regression; round 2 added a
second needle; round 3 found a third spelling the two needles missed. Two leaks
is the signal that the shape is wrong, so the second needle is now the
precondition every silent re-filing requires — `impl From<…> for StartupError` —
and the stated limit is derived from the matcher instead of enumerated by hand.
F-27 caught that round 2 had updated three sites and not `design.md` §9, the one
a phase agent builds from, so the slice would have produced the instrument F-16
had already found inadequate.

**A user-gated decision was breached by a repair and restored** (F-25). D5 and
OQ-6 settled that the spec binds no supervisor, and `design-log.md` records that
the decision is held in the wording rather than in the absence of a MUST. Round
2's repair wrote a MUST NOT addressed to a consumer. `design.md` §6 now records
the breach and its shape rather than merely re-asserting the decision — the next
repair is better served by the wording that broke it than by the claim that it
holds.

**A deferral was overturned by a constraint nobody had read** (F-26). SPEC-003
§7's preamble forbids amending that document to hold a row naming no test for a
clause the row itself calls reachable. F-2's repair removed the false
unreachability claim, which was the escape the old sentence qualified under, so
the amendment AC-8 requires could not land. The deferred ingress case is now in
the slice, and R-3's *"same position"* analogy is narrowed with it, as
`slice-010.md` §Follow-ups always said it would be.

**The rest** were claims the recut falsified and did not reach: doc comments a
phase agent copies verbatim, still asserting both removed facts (F-29); two
indexes each short a member added in the same round (F-30); a draft-status
paragraph that would have travelled into canon as a revision history (F-31); an
entry-failure shape with no production route stated in place of the one that has
it (F-32); a diagram state the transitions had moved past (F-33); and a doc
comment defining a surviving function by the one this slice removes (F-34).

**What round 3 knowingly leaves standing.** The line scan's derived limits —
multi-line, and an aliased `impl From`. F-12's phase residue, unaffected by
F-23's volition repair and still declared at the seam. A2's untested call site
and AC-9 as the only evidence for the slice's central claim, unchanged since
round 1.

**What this synthesis is not.** Round 3 made two **design** changes — a code
change to `Cancel` and `start`, and a test case that reverses a deferral — and
they are unreviewed. A round that changes the design has a different risk
profile from one that repairs prose. Round 4 is owed, and the design is
re-presented to the user before the plan (`docs/AGENTS.md` §Design).

## Synthesis — round 4

**Round 4's subject was round 3's repairs, two of them design changes. Four
blockers, nine majors, three minors; none withdrawn. The responder raised two
more while disposing, F-51 and F-52, both disposed with the user.** Two
blockers (F-35, F-36) and two majors (F-41, F-42) were sites a round-3 repair
did not reach. That class has now appeared in three consecutive rounds.

**F-37 is the round's finding.** Round 3 closed F-23 in code, but inside `start`,
where §4's second principle says no test reaches. The case §9 named for it
could not be written, and its mutation reddened nothing, so a new MUST was held
by nothing. The decision is now `exit::ended`, a pure function one tier down,
and what stays review is written down. The same move that made F-23's arm
testable is what exposed F-51: once the arms were written as code, R-1's
pronoun could be checked against them.

**The instrument was re-cut rather than patched a fourth time** (F-43, F-44).
Each round had found a spelling the needles missed, and the round-3 needle
banned every conversion onto a type on a false premise. The case now states
one rule about the one line it guards. That also holds the call's uniqueness,
which had been declared unheld since round 1.

**A premise nobody had checked was checked** (F-46). F-23's arm trusts `Cancel`,
and nothing had asked what trips it. Reading winit shows a lost connection
cannot trip it, and a compositor's close request can. So R-1 now says what the
host observes, a request, and not who made it. This is F-11, F-12 and F-23's
class again: a sentence asserting a fact the process never sees.

**What round 4 knowingly leaves standing.** `start`'s wiring of `exit::ended`
is review. The scan misses a later statement that re-files the call's binding.
A compositor that closes its clients exits 0. R-1's `Closed` residue remains.
AC-9 is still the only evidence for the slice's central claim.

**Round 5 is owed**, against the brief above, and the design is re-presented
to the user before the plan.

## Synthesis — round 5

**Round 5's subject was round 4's repairs, three of them design changes. One
blocker, two majors, five minors, two nits; F-35…F-52 all `verified`, none
contested or withdrawn.** For the fifth round running, the blocker is inside the
previous round's repairs. This time it was not a site a sweep missed. It was a
premise a repair relied on and did not check.

**F-53 is the round's finding.** Round 4 made the `Err` arm honest by reading
the request, and kept the `Ok` arm reading the call. The reason given, in A1,
`exit::ended`'s doc, R-1's cell and F-51's Response, was that `Ok` comes only
through `quit_event_loop`. Slint's winit backend has a second route. Every
`window_event` arm *assigns* `loop_error`, the arm's foot calls `exit()`, and
winit delivers the rest of the iteration before it looks at the exit. So a
failing event followed by a succeeding one ends the loop `Ok` with nothing
requested, and the host exits 0, silently and unrestarted. That is the outcome
F-46 was raised to rule out, reached by a different door. It is the fourth
instance of the class F-11, F-12, F-23 and F-46 share: a sentence asserting a
fact the process never observes. This time the fact was about a library, not
about the host. **F-54** is its textual half: R-2, §5's diagram and §6 row 1
already require 1 for that end, and the design answers 0.

**The instrument's limit was enumerated a fifth time** (F-55). The re-cut rule
holds, and every same-line spelling reds. But the limit that replaced the list
is itself a list of one. A re-filing of the `Ended` that `exit::ended` answers
compiles, keeps the call's line sanctioned, and restores the defect. The
matcher's real limit is *anything off the call's line*.

**The rest.** A second doc comment naming the function F-34's site was re-anchored
away from (F-56). A vocabulary rule stricter than the scan it cites (F-57).
Two sentences in canon text applied verbatim (F-58, F-59). A positional count
the consistency pass introduced (F-60). Two nits.

**Checked this round and clean — not to be re-billed.**
- The F-46 reading, at the symbol. winit 0.30.13's `queue_close` has two
  callers, `WindowHandler::request_close` and `FrameAction::Close`, and X11
  raises `CloseRequested` only from `WM_DELETE_WINDOW` in `client_message`.
  Slint's `request_close` is reached from the `CloseRequested` dispatch and
  from a root `Window`'s `close()`, and `crates/goad/ui/` calls `close()` only
  on popups.
- The tray's `quit`. It reaches `Cancel` through ksni's channel and a
  `spawn_local` dispatch loop, so it runs on the event loop, and *once the call
  has returned the value is final* holds.
- F-37's and F-43's mutations are sound. `cargo test` does not pass
  `-D warnings`, so the guard-dropping mutation compiles despite its
  unreachable pattern. `let call = Ok(…map_err(…)?);` type-checks against
  `exit::ended`.
- Change 3's phrase occurs once, and the result is well-formed. The F-52 rename
  is complete outside the ledger. F-42's sites all name all three changes.
  `Ended`, `status`, `ended`, `is_stopped` and `mod exit` collide with nothing.

**What round 5 knowingly leaves standing.** F-53 is established by reading and
not by running. Reaching it needs a backend no gate command has, and so does
confirming any repair of it. AC-9 observes the `Exit Failure` path, not this
one.

**Round 6 is owed only if F-53's disposition changes the design.** If the user
takes route (a), that is a change to `Ended` or to `exit::ended`'s arms, and
the measured trend says to review it. Route (b), and every other finding here,
is prose, and a prose repair is closed by checking its sites, not by a sixth
round.

## Synthesis — round 6

**Round 6's subject was round 5's repairs, one of them a design change. One
major, three minors, two nits; F-53…F-62 all `verified`, none contested or
withdrawn. No blocker.** For the first time in six rounds, the previous round's
repairs hold no blocker. The one design defect is again inside the design
change, one function past where the repair stopped looking.

**F-63 is the round's finding.** F-53 moved the decision into `exit::ended` and
proved it with four cases. It also gave `StoppedRunning` an `Option`, which made
`StoppedRunning(None)` a shape `exit::status` can match on its own. No case
feeds `exit::status` that shape. A classifier that answers 0 for it compiles
under the crate's lints, passes every named case, and restores F-53's outcome.
So R-1's *only if* claim, R-5's falsifier claim and AC-4 are each false as
specified. The repair is one assertion, and was checked against stand-ins.

**The rest is prose, per the user's severity rule.** The no-error line sits
outside R-6's pair and §9's line mutation (F-64). A1 fixes `Closed`'s payload
where A5's mechanism allows either (F-65). The second sentence was not swept to
every site (F-66). R-1's new sentence is unscoped (F-67). F-55's derived limit
rests on a premise that forgets the count half (F-68).

**Checked this round and clean — not to be re-billed.**
- `exit::ended`, `exit::status` and `report_exit_line` as §5.2 writes them
  compile clean under `clippy::pedantic` and `clippy::wildcard_enum_match_arm`,
  over stand-in types. This extends the lint spike `notes.md` §Handover records
  as covering only `exit::status`. It is still not compiled in the tree.
- No site reads the call's result as whether a stop was asked for, in either
  direction: A1, A5, §5.1, `exit::ended`'s doc, R-1's requirement and cell,
  R-2's cell, §5 *What the seam costs*, §6 row 1, §8 R1, AC-11 and §Scope.
  `Ending::Closed` is a residue nowhere, and exits 1 at every site that names
  it. In `start`, `quit_event_loop` follows `serve` whatever its `Ending`, and
  `Cancel::stop` has two callers, both in `install`.
- The split `Ok` cases are named identically in §9, its mutation list, and R-1's
  and R-2's cells. Each of §9's *decision* mutations reds the case it names.
- Canon-delta Change 1's replaced sentence and Change 3's phrase each occur once
  in live SPEC-003, and both results read well-formed.
- F-56, F-58, F-60, F-61 and F-62 at every site; `stop_signal` appears in no
  artefact but `design.md §5.2`.

**What round 6 knowingly leaves standing.** F-53's route is still established
only by reading. `start`'s wiring is still review. The retained R-5 half of
SPEC-003 R-3's analogy stays unsupported canon, as F-59's Response says.

**Round 7 is not owed on this trend if F-63 is repaired as routed.** The repair
is one case plus the sites that cite it, and it does not change the design. A
repair that is a case and its citations is closed by checking its sites and
running the mutation, not by another round.

## Close — site check for round 6's repairs

**Acting as checker, not raiser or responder.** The user waived a seventh round
(`design-log.md`, round 6): round 6's one design-level finding was repaired by
a case and its citations, and the rest is prose. Each repair was checked by
reading its sites in the artefacts, not the Responses.

- **F-63.** `exit_status::stopped_running_with_no_error_is_1` is named in
  `design.md` §9 (row and classifier mutations) and in R-1's, R-2's and R-5's
  cells. Its mutation is unrun in the tree; that is the plan's.
- **F-64.** R-6's cell holds both stopped lines as pairs; §9's row and *line*
  mutation name each arm; *"must red the pair"* is gone.
- **F-65.** A1 and the `Closed` edge row no longer say `None` unconditionally.
- **F-66.** No *new line* / *new sentence* singular, no *loop ends unasked* row,
  and §3 names `diagnostics.rs`.
- **F-67.** R-1's sentence is scoped; *"between 0 and any other status"* is
  gone.
- **F-68.** `design.md` §5.2 and R-2's cell name both halves; §8 R2 names the
  shape half and the *does not name the function* qualifier.
- **Wider.** No site reads the call's result as whether a stop was asked for
  (`Ok` means asked, `returned_ok_is_as_asked`, *only through the host's one quit
  route*: none left).

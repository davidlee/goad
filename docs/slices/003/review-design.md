# Review — design — Slice 003

**Subject:** design — `docs/slices/003/design.md`, with `canon-delta.md`,
`draft-spec.md` and `slice-003.md` as the artefacts it must be consistent with
**Reviewer:** fresh adversarial agent (Opus 5), no part in authoring the design
**Opened:** 2026-09-07
**State:** resolved — three rounds; all twenty findings verified, none
outstanding

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

**Round 1** — 2026-09-07 — the design of the scheduling timer.

Written after reading canon (`CLAUDE.md`, SPEC-001 in full, ADR-001, ADR-003,
POL-001) and before opening `design.md`, `research.md`, `design-log.md`,
`canon-delta.md`, `draft-spec.md` or `slice-003.md`.

**The invariants this review holds the design to.**

1. *The host does not understand the domain* (`CLAUDE.md`; SPEC-001 P-A). A
   timer is the most domain-tempting component in the host: "when should this
   fire" is one refusal away from "what kind of thing is being scheduled".
   Nothing in the design's types, module names or constants may encode a policy
   that belongs to a backend.
2. *Permissive wire, canonical internals* (SPEC-001 P-B). Whatever the timer
   consumes must already be canonical: SPEC-001/R-27 says the resolved next
   check is always a concrete instant with no unresolved state, so a timer that
   re-parses, re-validates or defends against a `next_check` has found a leak,
   and one that admits an `Option` has widened the canonical type.
3. *A renderer's subset is never the protocol's* (`CLAUDE.md` invariant 3;
   SPEC-001/R-55). The generalisation to attack here is the timer's: an
   implementation that fires on a `tokio` sleep must not become the reason the
   protocol admits fewer instants than SPEC-001 §4 says it does. R-28 (a past
   instant is stored as given) and R-26 (an elapsed instant is consumed for the
   default poll) are the two places a timer will be tempted to "correct" a
   backend's instruction; a floor or clamp is exactly that correction wearing a
   different name.
4. *A backend failure never takes the host down, and never leaves it unable to
   invoke the backend again* (SPEC-001 P-C, R-45, R-29). A timer adds a new
   way to fail closed: a scheduling loop that exits, wedges, or stops rearming
   after a refusal is a silent host, which R-29's Behaviour note names as the
   failure mode a user notices last.
5. *Strata run one way* (ADR-001, ADR-003, POL-001 Verification). Stratum 1 has
   no clock. Anything the design puts in `crates/goad-semantics` that needs to
   know the time, or that encodes a host operational policy, is a violation the
   four instruments may or may not catch — the purity scan sees a direct `std`
   clock reach and nothing else, so a constant or a policy smuggled downward
   passes the gate and still breaks the ADR.

**What I intend to attack, specifically.**

- The three assumptions the designer nominated: the monotonic deadline across
  suspend (R3); D-3's floor spacing scoped to scheduled-after-scheduled
  firings, which I will try to defeat with an alternating-stimulus loop; and
  computing the wait from the request's `now` rather than a fresh clock read,
  looking for compounding lateness.
- Every acceptance criterion in `slice-003.md` against a design section that
  actually discharges it, with particular weight on AC-4, AC-5 and AC-9, which
  the designer revised — a revision that makes an AC easier to pass is a
  weakening unless the original was wrong.
- The canon delta and the draft spec against SPEC-001's existing R-n, in letter
  and in spirit; and `kind: "scheduled"` / `source: "host"` against brief §8.1
  and SPEC-001 §6.1, including whether correcting the illustration changes the
  contract for a backend written against the published text.
- AC-6's grep-based structural scan: whether a line-based instrument can hold
  the property claimed, and whether its admitted set is complete and minimal —
  the failure mode POL-001 names for every other line-based scan in this repo.
- The test strategy (D-7: real waits, no mock clock) against AC-12's no-flake
  requirement, with the actual bounds and margins computed, not asserted; and
  whether the AC-10 production-topology test asserts the timer or only that a
  sleep returns.
- Cancellation under AC-7: a pinned sleep in a `select!`, a stop arriving
  concurrently with a reset, and whether any handle or task survives the drop
  R-48's final clause forbids.
- Anything the spike did not measure that the design cites as measured, and any
  path the design will touch but does not declare (or declares and will not
  touch).

**Round 2** — 2026-09-07 — the repairs, and only the repairs.

Sixteen findings dispositioned `fix-now` or `doc-wrong`, none `aligned`, none
`tolerated`. Round 2 verifies each repair against the tree rather than against
the response that describes it, and then attacks the repairs as new subjects.
Three repairs introduce mechanism rather than wording and are where a fresh
defect is most likely: **D-3's `floor_until` anchor**, which replaces a `bool`
with loop state and is now normative in draft SPEC-002 R-4; **D-16's two AC-6
instruments**, which are new checks that must pass on the tree as it stands
today, not merely be well-argued; and **§9's margin table**, which is
arithmetic from the design's own numbers and can be checked against them. The
question I hold each to is the one round 1 kept finding: is the claim true of
every path, or only of the common one.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | verified |
| F-2 | major | fix-now | verified |
| F-3 | major | fix-now | verified |
| F-4 | major | fix-now | verified |
| F-5 | minor | doc-wrong | verified |
| F-6 | minor | fix-now | verified |
| F-7 | minor | doc-wrong | verified |
| F-8 | minor | doc-wrong | verified |
| F-9 | minor | doc-wrong | verified |
| F-10 | minor | doc-wrong | verified |
| F-11 | minor | fix-now | verified |
| F-12 | minor | fix-now | verified |
| F-13 | minor | doc-wrong | verified |
| F-14 | nit | fix-now | verified |
| F-15 | nit | doc-wrong | verified |
| F-16 | nit | doc-wrong | verified |
| F-17 | major | fix-now | verified |
| F-18 | minor | fix-now | verified |
| F-19 | minor | fix-now | verified |
| F-20 | nit | fix-now | verified |

### F-1 — E-1 and AC-4 claim a past instant fires "once"; with a backend that instructs the past on every response it fires forever, and only the floor bounds it

**Severity:** major
**Location:** `design.md` §5.5 E-1; `slice-003.md` AC-4; `schedule.rs:196-201`

**Expected:** an edge case the design nominates as discharging an acceptance
criterion states what actually happens on that path.

**Observed:** E-1 reads in full: *"`wait_for` yields the floor, or zero when
unspaced. The firing happens; the following resolution consumes the elapsed
value (`schedule.rs:196-201`); cadence resumes. Once, not repeatedly."* AC-4
carries the same clause. Both are true only when the past instruction is a
one-off. `resolve`'s first arm returns a valid `incoming` verbatim
(`schedule.rs:227-228`), so a backend that answers every request with a past
instant is never in the arm that consumes anything: the retained value is
replaced, not consumed, on every exchange, and the host fires once per
`MINIMUM_SPACING` indefinitely. "Cadence resumes" never happens and "once, not
repeatedly" is false.

**Evidence:** the cited paragraph is itself the proof — `schedule.rs:196-201`
says *"A **retained** value at or before `now` does not stand"* (emphasis on
which value), and the consuming arm is `(None, Some(pending)) if pending >
now`, unreachable while `incoming` is `Some`. The design states the correct
behaviour two sections earlier, in §5.4's floor consequences: *"A backend
returning a past `next_check` on every response can cause at most one unspaced
immediate firing per human action, and every firing after that is at least
three seconds apart."* `draft-spec.md` §5 *A backend asking for the past* is
also correct. So the design holds both the right statement and the wrong one,
and the wrong one is the one attached to AC-4. This matters beyond wording: E-1
as written says the schedule self-corrects, which invites a later reader to
treat the floor as belt-and-braces rather than as the only bound on H-1
(`research.md` Thread 4).

**Disposition:** `fix-now`
**Response:**

Accepted in full, and the class swept. E-1 held both readings and the
wrong one was attached to AC-4. The repair states the true invariant —
**each resolved instant fires at most once; firing does not re-fire an
instant on account of its having elapsed** — and then splits the two
successor cases explicitly wherever the claim appears: a one-off past
instruction reaches `resolve`'s consuming arm and cadence resumes; a past
instruction on every response is returned verbatim by the first arm, the
retained value is replaced rather than consumed, cadence never resumes, and
the minimum spacing is the only bound.

Every site traced and corrected: `design.md` §5.5 E-1, §7 D-8's row is
unchanged in substance but §5.4's consequences paragraph now names the
"first firing of the process" rather than "per human action" (which D-3's
new shape makes exact), §9's AC-4 row; `slice-003.md` AC-4; `draft-spec.md`
P-C, R-3 and §5 *A backend asking for the past* — the last of which the
raiser called correct but which carried the same "the following resolution
consumes the elapsed value and the host returns to the default cadence"
sentence as an unconditional claim.

**Outcome:** `verified`. Traced to every site the response names. `design.md` §5.5 E-1 now
states the invariant (*"Each resolved instant fires at most once"*) and then
splits the two successor cases, naming the first arm as the one that replaces
rather than consumes, and saying in terms that the floor *"is not
belt-and-braces, it is the whole answer to H-1"*. `slice-003.md` AC-4 carries
the same split; `design.md` §9's AC-4 row now says "on **every** response".
`draft-spec.md` §5 *A backend asking for the past* is rewritten as two bullets
and P-C and R-3 with it. The response also found a site round 1 did not: the
same partial claim in `schedule.rs:198-201`'s own doc comment, now named in E-1
with the phase that corrects it. Repaired wider than raised.

### F-2 — D-3's floor rule is being written into canon on a premise slice 004 retires

**Severity:** major
**Location:** `draft-spec.md` R-5; `design.md` §5.4 (*The floor rule, stated
exactly*), §7 D-3; `slice-003.md` §Non-goals

**Expected:** a normative rule states a condition that stays true, or names the
premise it depends on so the slice that invalidates it must revisit it.

**Observed:** the rule spaces *scheduled-after-scheduled* firings only, and
draft SPEC-002 R-5 fixes that shape normatively: *"The minimum spacing MUST NOT
apply to an evaluation whose predecessor was not itself a scheduled
evaluation."* The design's whole argument for that shape is that the
non-scheduled stimuli are human: *"the hazard is a self-driven loop, and a
person's own actions are rate-limited by the person and by a capacity-1
channel"* (§7 D-3), and §5.4 *"A person's own actions are never delayed by the
floor."*

That premise expires inside this slice's own declared roadmap.
`slice-003.md` §Non-goals says *"Event ingress (slice 004) … A scheduled
evaluation and an external event are two stimuli into one path; this slice
builds the path, and 004 adds the second stimulus."* An external event source
is not rate-limited by a person. Once one exists, an event arriving at machine
rate clears the spacing bit before every scheduled firing, and a backend that
instructs the past (F-1's case) is again in an unbounded exchange loop — the
exact hazard the floor was introduced for. Slice 004 will inherit R-5 as canon
and has no reason to reopen it, because R-5 does not say why it is shaped that
way.

**Evidence:** `design.md` §7 D-3 rationale; `draft-spec.md` R-5;
`slice-003.md` §Non-goals (*Event ingress*); `research.md` Thread 4 H-1 and
X-2. The interaction is between two things the slice already holds, not a
speculative future.

**Disposition:** `fix-now`
**Response:**

Accepted, and the rule rewritten rather than annotated. Annotating the
premise would have left slice 004 free to read the annotation and keep the
rule; the repair removes the premise instead.

The floor is now anchored to **the previous scheduled firing**, held as one
monotonic `tokio::time::Instant` that nothing clears. It is equivalent to the
predecessor bit today and strictly stronger once a non-human stimulus exists,
because no stimulus can clear it. It makes no claim about what the other
stimuli are.

Recorded as a decision in `design-log.md` (2026-09-07, round 1) with the three
rejected alternatives, and `design.md` §7 D-3 rewritten in place — design.md
holds current truth, so the superseded shape lives in the log, not in a second
row. Draft SPEC-002 R-4 gains *"whatever else the host did in between"* and
R-5 now says in the requirement itself that it bounds due-check firings only
and that **a host adding another stimulus must bound that stimulus
separately**. `slice-003.md` §Non-goals says the same where slice 004 will
read it.

**Outcome:** `verified`, and the repair is stronger than the finding asked for. Round 1
asked for the premise to be named; the response removed it. `floor_until` is a
`tokio::time::Instant` written at one site and cleared by nothing, so no
stimulus — human, event or otherwise — can un-floor a scheduled firing.
Checked against the loop: the anchor is monotonically non-decreasing, so the
process gets exactly **one** unfloored scheduled firing rather than one per
human action, which is what the `bool` allowed. Draft SPEC-002 R-4 gains
*"whatever else the host did in between"* and R-5 now says a host adding a
stimulus *"MUST decide separately how that stimulus is bounded, and MUST NOT
read this requirement as covering it"* — which is the sentence slice 004 needs
to find. §5.4's own *Why an anchored instant and not a wall-clock one* answers
the objection the earlier draft raised against a retained instant: a monotonic
anchor has no backwards-jump exposure.

### F-3 — AC-6's scan is defeated by the brace-grouped import this design itself makes the likely one

**Severity:** major
**Location:** `slice-003.md` AC-6; `design.md` §9 (AC-6 row), §5.5 I-1;
`crates/goad-boundary/tests/checks/purity.rs:1-8`

**Expected:** a structural instrument either holds the property or states what
it does not reach, per POL-001 §Verification, which gives every instrument in
this repository a "what it does not reach" column.

**Observed:** AC-6 asserts *"`schedule::resolve` is named at exactly its two
existing sites … and nowhere in `crates/goad/src`"*, held by *"a test that
greps the tree"*. Three problems, one of them fatal.

1. **The defeat is the import this design creates.** Stratum 3 must now import
   from `goad_semantics::schedule` for the first time — `wait_for` and
   `MINIMUM_SPACING` (§5.2). The natural spelling is
   `use goad_semantics::schedule::{MINIMUM_SPACING, wait_for};`. Adding
   `resolve` to that brace group gives a stratum 3 file a bare `resolve(...)`
   call and **no line anywhere containing the token `schedule::resolve`**. The
   scan passes. This is not hypothetical: `purity.rs`'s own header concedes the
   identical class — *"`use std::{fs, process};` names neither token and
   passes"* — and before this slice, `crates/goad/src` imported nothing from
   `schedule` at all, so the grouping opportunity is new.
2. **The admitted set as written is not the set in the tree.** `schedule::
   resolve` is also named at `crates/goad-shell/src/state.rs:41`,
   `crates/goad-shell/src/host.rs:124` and `crates/goad-shell/src/config.rs:25`
   (doc comments), and `resolve(` appears seven times in `schedule.rs`'s own
   `#[cfg(test)]` module. The existing prior art
   (`crates/goad-boundary/tests/checks/structure.rs`, via `code_of`) strips
   comments and reads only to `#[cfg(test)]`, which would handle all of them —
   but the design does not say the scan is built on that machinery, and an
   agent implementing "a test that greps the tree" from the design alone gets
   five false positives.
3. **Line numbers are the wrong pin.** The design names `host.rs:128` and
   `host.rs:259`. Any edit above either shifts them; §2's own table already
   cites `resolve_from` as `:258-265` when the `schedule::resolve` call is at
   `:259` and the fn opens at `:258`. Pin by file and count, not by line.

**Evidence:** verified against the tree — `grep -rn "schedule::resolve"` over
`crates/` returns `host.rs:124`, `host.rs:128`, `host.rs:259`, `state.rs:41`,
`config.rs:25`; `crates/goad/src` currently imports only
`goad_semantics::protocol::*` (six files, no `schedule`). `purity.rs:1-8` and
POL-001 §Verification for the convention this AC does not follow. AC-6 is the
sole instrument the design offers for `draft-spec.md` R-2.

**Disposition:** `fix-now`
**Response:**

Accepted on all three points, and the instrument redesigned rather than
patched.

1. The defeat is real and this slice creates it. The stratum 3 half now
   forbids the **identifier** `resolve`, not the path: one more configured
   `goad_boundary::scan::Scan` over `crates/goad/src`, which strips comments
   through `code_of`, matches an identifier by word (so it catches `resolve`
   inside a brace-grouped `use` and does not catch `resolved`), and carries
   the vacuity guard. An item cannot be called without being named, at the
   call or in the `use`, so import style cannot defeat it. Verified it passes
   today: the token's five occurrences under `crates/goad/src` are all in
   comments (`reception.rs:41`, `wire.rs:167`, `controller.rs:140`, `:168`,
   `:255`). The one residue — a stratum 2 re-export under another name — is
   stated in `design.md` §5.5 I-1a and §9.
2. The admitted set is now stated as measured: `schedule::resolve` appears
   five times in the tree, of which `host.rs:124`, `state.rs:41` and
   `config.rs:25` are doc comments `code_of` removes, plus seven bare
   `resolve(` calls in `schedule.rs`'s own `#[cfg(test)]` module. The stratum
   2 half is explicitly built on `structure.rs`'s machinery — production code
   only, read to the file's own `#[cfg(test)]`, comments stripped — rather
   than left as "a test that greps the tree".
3. No line numbers. The stratum 2 half asserts *exactly two occurrences, both
   in `host.rs`*, pinned by file and count. `design.md` §2's row now cites the
   call at `:259` separately from the function span `:258-265`.

D-16 records the decision; `slice-003.md` AC-6 and `design.md` §9's AC-6 row
state both instruments and both residues.

**Outcome:** `verified` **as to the defect raised**. The path grep is gone; AC-6 is now two
instruments, neither pinned to a line (D-16). The brace-group defeat is closed
for stratum 3 by forbidding the identifier, and I confirmed the closure is
total against every import style I could construct: a brace-grouped `use`, an
`as` alias, a glob import with a bare call, and a fully-qualified call all name
`resolve` on some line, and `goad_boundary::scan::mentions` splits on
non-alphanumerics so each is matched. The one route out is a stratum 2
re-export under another name, which the design states as the residue. I-1's
site inventory is now correct: two calls, three doc mentions, seven in stratum
1's own tests.

The asymmetry between the two instruments is also right, for a reason the
design does not give: `mentions` splits on `_`, so an identifier scan over
stratum 2 would match `resolve_from` three further times and could not assert a
count of two. The path form is forced there, not merely chosen.

**What the repair got wrong is its factual claim about the tree, and that is
raised separately as F-17.**

### F-4 — the design's `serve` sketch calls `.expect` in production code, which the workspace lint table denies

**Severity:** major
**Location:** `design.md` §5.4 (the loop code block); `Cargo.toml:136-138`;
`clippy.toml:20-23`

**Expected:** the design's code shape passes the gate POL-001 defines, or the
design argues a site-local `#[expect]` where it needs one.

**Observed:** the re-arm line reads
`wait_for(controller.next_check().expect("absorb just stored one"), now, floor)`.
`expect_used = "deny"` is workspace-wide (`Cargo.toml:137`) and `clippy.toml`
carves it out for tests only (`allow-expect-in-tests = true`). `serve` is
production stratum 3. The gate's clippy line
(`cargo clippy --workspace --all-targets -- -D warnings`) fails on it, and
POL-001 forbids reaching for a suppression to make a phase green.

The lint is the symptom. The defect is that §5.3 makes
`Controller::next_check` an `Option` — *"`Option` for one reason: before the
first outcome there is genuinely no instant in stratum 3"* — and the design
then needs it total at the one site that reads it, immediately after `absorb`.
Two shapes remove the unwrap rather than suppress it: have `absorb` return the
resolved instant alongside the `Shift` (it already destructures `Received`,
`controller.rs:136-146`), or move the re-arm behind a `Controller` method that
owns the `Option`. Either keeps §5.3's justification for the `Option` intact
while making the loop's read infallible.

**Evidence:** `Cargo.toml:136-138`; `clippy.toml:20-23` and its own comment
that the four keys exist because a *test* that unwraps is asserting;
`controller.rs:136-160` for `absorb`'s current shape; POL-001 §Compliance
(*"Don't … suppress a lint in order to make a phase green"*).

**Disposition:** `fix-now`
**Response:**

Accepted, and the cause fixed rather than the symptom. Verified the lint
table (`Cargo.toml:137`, `expect_used = "deny"`) and the carve-out
(`clippy.toml:20-23`, tests only), so the sketch would have failed the gate.

`Controller::absorb` now returns `Absorbed { shift, next_check }`. The
`next_check` is not an `Option`: `Outcome::next_check` is a concrete instant
on every outcome, failures included (`host.rs:76`), so an exchange that
completed always resolved one. The loop reads the return value and never the
field, so the read is total by construction rather than by assertion.
`Controller.next_check` stays an `Option` for `frame()` alone, which keeps
§5.3's justification intact, and there is no accessor — an accessor would
exist only to be unwrapped. D-15.

**Outcome:** `verified`. `Absorbed { shift, next_check }` is the shape, `next_check` is not
an `Option` on it, and the loop reads the return value. No `expect` appears in
the §5.4 sketch. Checked the justification rather than accepting it:
`Outcome::next_check` is documented *"Always concrete — brief §9 resolves in
every case, including failure"* (`host.rs:76`) and `no_action` writes it on the
failure path (`host.rs:283-298`), so the totality is a property of the type and
not an assertion about it. §5.2 also removes the `Controller::next_check()`
accessor and says why — *"it would exist only to be unwrapped"* — which fixes
the class rather than the site.

### F-5 — §5.4 property 1's claim that an elapsed deadline "wins the next iteration" is false for the two arms that reach `absorb`

**Severity:** minor
**Location:** `design.md` §5.4, property 1; `controller.rs:312-360`

**Expected:** a property stated as load-bearing about the `biased` order holds
on every path.

**Observed:** property 1 says *"An elapsed deadline is not lost by losing the
race — it stays elapsed and wins the next iteration."* The loop has four
command arms. `OpenDiagnostics` and `CloseDiagnostics` `continue` without
touching the sleep, so the claim holds there. `Evaluate` and `Choose` both
reach the second `select!` and then the re-arm — `sleep.as_mut().reset(...)` —
which discards the elapsed deadline outright. On those two paths the scheduled
firing does not win the next iteration; it is superseded.

The *behaviour* is defensible: both paths run an exchange that re-resolves the
schedule, so the host is correctly armed afterwards. The statement is not, and
it is offered as one of "five properties of that shape, each load-bearing".
Restate it as: an elapsed deadline survives an arm that does not re-arm, and is
superseded by one that does.

**Evidence:** `controller.rs:326-333` (the two `continue` arms) against
`:382-385` (`absorb`, where the design inserts the reset); `design.md` §5.4
property 1.

**Disposition:** `doc-wrong`
**Response:**

Accepted; the statement was wrong, the behaviour was not. Property 1 now
reads: an elapsed deadline survives an arm that does not re-arm — the two
diagnostics arms, which `continue` without touching the sleep
(`controller.rs:326-333`) — and is **superseded**, not lost, by the two arms
that reach `absorb` and re-arm from a freshly resolved instant.

This finding named the class the synthesis identifies, and the sweep for
others of it is recorded against F-7, F-8, F-9, F-13 and F-16 individually. It
also turned up one the review did not raise, now `design.md` §5.5 E-6: an
instruction further out than tokio's `MAX_SAFE_MILLIS_DURATION` (roughly two
years, `runtime/time/source.rs:28-29`) is clamped and fires early rather than
overflowing — a claim about the common path that the design had not qualified
either.

**Outcome:** `verified`. Property 1 now distinguishes the two arms that `continue` without
touching the sleep from the two that reach `absorb` and re-arm, and uses
*superseded* rather than *lost* for the second pair. Checked against the loop:
`controller.rs:326-333` are the two `continue` arms, and they are the only ones.

### F-6 — `previous_was_scheduled` has no stated write site, and the plausible sites give different behaviour

**Severity:** minor
**Location:** `design.md` §5.3 (third row), §5.4 (the code block)

**Expected:** the one piece of loop state that selects the floor has a named
write site, because the floor is the slice's only bound on a self-driven loop.

**Observed:** §5.3's table says the bit is *"written by `serve`, once per
iteration"*. The §5.4 sketch declares
`let mut previous_was_scheduled = false;` and never assigns it. The two natural
readings differ:

- **written at the top of every iteration from the stimulus.** Then
  `Command::OpenDiagnostics` and `Command::CloseDiagnostics` — which `continue`
  without an exchange (`controller.rs:326-333`) — clear the bit, so opening the
  diagnostics window un-floors the next scheduled firing. Combined with F-1's
  backend that instructs the past, toggling the diagnostics panel becomes an
  amplifier.
- **written only in the `absorb` arm.** Then a scheduled fire refused by
  `stamp` (§5.4 property 5) never sets it, so the firing after a clock recovers
  is un-floored even though its predecessor was scheduled.

Neither is obviously wrong, and that is the point: the design settles the floor
rule in prose and leaves the mechanism that implements it to the phase.

**Evidence:** `design.md` §5.3 and §5.4; `controller.rs:326-333` for the two
arms that never reach `absorb`.

**Disposition:** `fix-now`
**Response:**

Accepted, and closed by F-2's repair rather than by naming a write site for
the bit. The bit is gone. `floor_until`, a monotonic instant, has exactly one
write site: **the timer arm sets it the moment it wins the `select!`**, before
stamping. Nothing else writes it and nothing clears it.

Both of the readings the finding names disappear with it. Opening the
diagnostics window cannot un-floor anything, because those arms write nothing.
A scheduled fire refused by `stamp` still advanced the anchor, because the
anchor is advanced by the firing rather than by the outcome — which also
turns §5.4's property 5 from a special case into a consequence of the general
rule. `design.md` §5.3's ownership table now names the site.

**Outcome:** `verified`, and the mechanism replaced rather than the write site named. The
`bool` is gone. `floor_until` has exactly one write — *"the moment the timer
arm wins the `select!`, before stamping"* — which is stated in §5.3, drawn in
§5.4's diagram, and written in the sketch inside the arm body. Both horns of
the finding are closed by that placement: a diagnostics command cannot clear an
anchor nothing clears, and a refused scheduled fire has already advanced it, so
E-2's three-second retry falls out of the general rule instead of needing one
of its own (§5.4 property 5).

### F-7 — R1's flakiness argument is one-sided in the wrong direction for the half it covers with `until`, and no margin is stated anywhere

**Severity:** minor
**Location:** `design.md` §8 R1; `slice-003.md` AC-12;
`crates/goad/tests/renderer/wiring.rs:135-148`

**Expected:** AC-12 calls a load-sensitive test *"a design defect, not a
tolerated cost"*, so the argument that no test is load-sensitive has to be
sound.

**Observed:** R1's mitigation reads *"Every timing assertion is one-sided in
the direction load pushes it: liveness through `until(2 s, …)`, anti-spin by
counting invocations inside a window far shorter than the 3 s floor, where load
can only reduce the count."* The second clause is right. The first is exactly
backwards: `until` panics when its deadline passes (`wiring.rs:141-146`), so a
liveness assertion is one-sided in the direction load pushes it *towards
failure*. Load cannot make an anti-spin count fail; it is the only thing that
can make an `until` fail.

The margins are probably adequate — AC-1's 100 ms poll against a 2 s bound is
20x, and the existing tier already spawns real child processes under the same
bound — but the design states no figure for any of them, and the whole of
AC-12's discharge is this one sentence. What is missing is the arithmetic: for
each timed test, the expected time, the bound, and the ratio; and the added
wall-clock cost of the anti-spin windows, which are floors rather than
ceilings and therefore always paid (the gate measured 5.276 s on the finished
slice 002 tree, ADR-003).

**Evidence:** `wiring.rs:135-148` (`until` asserts against a deadline);
`design.md` §8 R1; `slice-003.md` AC-12; ADR-003 §Context for the gate's
current cost.

**Disposition:** `doc-wrong`
**Response:**

Accepted on both halves. R1's mitigation is rewritten to say what is true:
anti-spin counts cannot fail under load because load only reduces a count;
`until` panics when its deadline passes (`wiring.rs:141-146`), so liveness is
one-sided **towards** failure and is held by margin, not by one-sidedness.

The arithmetic the finding asks for is now a table in `design.md` §9: for each
timed assertion, its kind, expected time, bound, margin and wall cost. The
smallest margin is 19x (a ~105 ms expectation against `until(2 s)`). The
always-paid cost is the anti-spin and anti-fire windows, about 1.8 s, plus
roughly 0.7 s of liveness waits that resolve as soon as the firing lands,
against the 5.276 s gate ADR-003 §Context measures. Stated as estimates from
the design's own numbers, with the plan re-measuring the gate before and after
— the finding is right that they cannot be measured until the tests exist.

**Outcome:** `verified`. R1 now says plainly that liveness *"is one-sided in the wrong
direction and is held by margin instead"*, and §9 carries the margin table. I
recomputed every ratio from the table's own numbers: 2000/105 = 19.05 for the
seven liveness rows, 3000/500 = 6 for the three anti-spin rows, 1.8 s of
unavoidable window time and ~0.7 s of liveness waits against ADR-003's
measured 5.276 s gate. All correct as arithmetic. Two gaps *in* the table are
raised as F-19 and F-20; the finding as raised is discharged.

### F-8 — A-1 and AC-10 both substitute the Slint platform, and neither says so

**Severity:** minor
**Location:** `design.md` §5.5 A-1, §9 (AC-10 row); `research.md` Thread 5
(*Spike S-1 result*, "The topology"); `timer-probe.local.rs:36`

**Expected:** an assumption labelled *"Measured, not reasoned"* names what the
measurement did not cover, in the manner of every other instrument in this
repository.

**Observed:** A-1 is *"Slint's executor completes tokio timers under the
`EnterGuard`"*, and AC-10 is *"proved in the arrangement it will actually run
in, not only in a test harness that substitutes for it"*. Both the spike and
the AC-10 target run under `i_slint_backend_testing`'s `TestingBackend`
(`init_integration_test_with_system_time`, whose options are `mock_time: false,
threading: true`). Production runs `start`'s real platform. Research's summary
says *"The topology. Exactly `start`'s"* and then lists the testing
initialiser as one of its components, which is a substitution rather than an
identity.

The choice is right — there is no headless way to run the production backend —
and everything else in the topology genuinely is production's: the multi-thread
runtime, the guard, `spawn_local`, a real window and tray, the real transport.
The defect is that the residual is unstated, so AC-10 reads as fully
discharging slice 002's F-5 when what it discharges is F-5 minus the platform.
One sentence in A-1 and one in §9's AC-10 row fixes it.

**Evidence:** `timer-probe.local.rs:30,36`;
`i-slint-backend-testing-1.17.1/lib.rs:67-80` (the initialiser and its
options); `research.md` Thread 5; `design.md` §5.5 A-1 and §9.

**Disposition:** `doc-wrong`
**Response:**

Accepted. Verified the initialiser and its options
(`i-slint-backend-testing-1.17.1/lib.rs:67-80`; `timer-probe.local.rs:36`).
A-1 now carries a *What S-1 substituted* paragraph naming the one component —
the Slint platform, `TestingBackend` rather than the platform `start` installs
— listing everything else as genuinely production's, and saying plainly that
no headless test can close it. §9's AC-10 row and `slice-003.md` AC-10 both
say the criterion discharges slice 002's F-5 for every component except the
platform. The AC is not weakened: it still requires every other component to
be production's, and now additionally requires the substitution to be
stated.

**Outcome:** `verified`. A-1 gains *What S-1 substituted*, which names the testing backend,
cites `timer-probe.local.rs:36` and the initialiser's own options, lists the
components that genuinely are production's, and states the one that is not:
*"that the production platform's own event loop polls a `spawn_local` future
the way the testing platform does. No headless test can close that."* §9's
AC-10 row is retitled *"minus one component"* and says AC-10 discharges slice
002's F-5 for everything except the platform. That is the whole of what was
asked.

### F-9 — draft SPEC-002 R-3 and R-4 contradict each other in letter, and the design already spotted it once

**Severity:** minor
**Location:** `draft-spec.md` R-3, R-4; `design-log.md` (*Acceptance criteria
revised*, AC-4)

**Expected:** two requirements in one document do not forbid each other.

**Observed:** R-3: *"A resolved next check at or before the current instant
MUST fire once, **promptly**"*. R-4: *"The host MUST NOT begin a scheduled
evaluation less than a fixed minimum spacing after the scheduled evaluation
that preceded it. The spacing is 3 seconds."* A past instant whose predecessor
was a scheduled evaluation fires three seconds later, which R-3 forbids as
written and R-4 requires.

The design already found this defect once, on the same words: the design-log's
AC revision entry says AC-4 said *"immediately"*, that *"with D-3's floor, a
past instant whose predecessor was itself a scheduled evaluation fires after
the floor"*, and softened it to "promptly" while naming both cases. The draft
spec did not inherit the naming-both-cases half. R-3 should be explicitly
subject to R-4, or should say "as soon as R-4 permits".

**Evidence:** `draft-spec.md` §4 R-3 and R-4; `design-log.md`, entry
*Acceptance criteria revised*; `slice-003.md` AC-4 as revised, which does name
both cases.

**Disposition:** `doc-wrong`
**Response:**

Accepted. R-3 now reads *"MUST fire as soon as R-4 permits"*, which removes
the contradiction in letter, and R-3 additionally carries F-1's correction
about what happens next. R-4 and R-5 are rewritten under F-2 and no longer
depend on a predecessor's kind, so the interaction R-3 defers to is a single
rule rather than a pair. `slice-003.md` AC-4 already named both cases and now
matches.

**Outcome:** `verified`. Draft SPEC-002 R-3 now reads *"MUST fire as soon as R-4 permits"*
and carries F-1's two-case split as well. R-4 is unchanged in substance and
still scoped by *"the scheduled evaluation that preceded it"*, so a process with
no preceding scheduled firing satisfies it vacuously — which is what makes the
first firing's exemption a consequence of R-4 rather than an exception to it.

### F-10 — the floor is invisible to the backend by decision and invisible to the person by omission, and the one observable can disagree with when the host will fire

**Severity:** minor
**Location:** `design.md` §5.2 (`next_check_line`), §7 D-9, §5.5 E-2, E-4;
`draft-spec.md` OQ-2

**Expected:** the schedule surface OQ-3 bought answers "when will the host next
ask?".

**Observed:** `next_check_line(at)` renders `Controller.next_check` — the
instruction the backend sent, retained by `absorb`. What the host actually acts
on is the monotonic deadline in the loop, and the design names three states in
which the two differ:

- **the floor** raises the deadline above the instruction whenever the
  predecessor was scheduled (§5.4). A backend instructing one second is shown
  as one second and fires at three.
- **a refused scheduled fire** re-arms the deadline at `MINIMUM_SPACING` and
  leaves the instruction untouched (§5.4 property 5, E-2). While a clock is
  broken the line shows an instant that has already passed, indefinitely.
- **a suspend** detaches the deadline from wall time entirely (E-4, R3). After
  a six-hour suspend the line shows an instant six hours gone.

`draft-spec.md` OQ-2 records deliberately that the floor is not reported to the
backend. Nothing records that it is not reported to the person either, and the
person is the one who was given a surface. Showing the deadline the host will
actually fire on — or both values when they differ — costs one more field on
`Frame`, which the design is adding to anyway.

**Evidence:** `design.md` §5.2, §5.3's ownership table (instant in the
controller, deadline in the loop), §5.4 property 5, §5.5 E-2 and E-4;
`draft-spec.md` OQ-2; `design-log.md` OQ-3's answer (*"one line naming the next
check"*).

**Disposition:** `doc-wrong`
**Response:**

Accepted in part, on evidence.

**Accepted:** the line said "next check" and rendered the instruction, in a
design that names three states where the host fires elsewhere. It now renders
`next check (instructed): …`, and `design.md` §5.2 states which of the two
instants it is and lists the three divergences. D-9's row records it.

**Rejected, with reason:** showing the deadline the host will actually fire on
is not one more field on `Frame`. The deadline is a `tokio::time::Instant`, a
monotonic value with no wall-clock meaning, so rendering it needs a second
clock read per frame and a conversion that can fail — a new failure path in
the renderer, for a surface OQ-3 deliberately kept to one line. Recorded as
draft SPEC-002 OQ-3 rather than dropped, and §6 of the draft now forbids a
host presenting the instruction *as a prediction* of when it will fire, which
is the part that actually matters to a second implementation.

**Outcome:** `verified`, and the rejected half is a reasonable refusal. The line is now
`"next check (instructed): …"`, and §5.2 names the three states in which the
instruction and the deadline differ. The refusal to surface the deadline rests
on a fact I checked: `floor_until` and the sleep's deadline are
`tokio::time::Instant`s, monotonic values with no wall-clock rendering, so
showing one needs a second clock read per frame and a fallible conversion —
a new failure path in the renderer, for a value the parenthetical already warns
the reader about. It is recorded as draft SPEC-002 OQ-3 rather than dropped,
which is the right home. Accepting the refusal.

### F-11 — two paths the design will touch are declared unchanged or not declared at all

**Severity:** minor
**Location:** `slice-003.md` §Scope; `design.md` §9 (AC-6 row), §8 R2

**Expected:** §Scope lists the surfaces the slice may touch, and
`docs/AGENTS.md` §Execute makes an undeclared path either a design change or
scope creep.

**Observed:** two omissions, in both directions.

- **`crates/goad-boundary` is not in §Scope.** AC-6's new scan is placed in the
  "boundary" tier (`design.md` §9), which is that member — it is where every
  other scan in the workspace lives (ADR-003 §Decision). §Scope's Tests block
  names `crates/goad/tests/`, the new `event_loop_schedule` target,
  `tests/support/driving.rs`, `tests/backends/` and `crates/goad/Cargo.toml`,
  and stops.
- **`crates/goad-shell/src/config.rs` is declared "unchanged"** in §Scope, on
  the ground that the floor is a constant rather than configuration. R2's
  mitigation then says the floor is *"Stated in `draft-spec.md` and in the
  config documentation"*. The config documentation is that file.

Both are small edits. The point is that the audit diffs touched paths against
declared surfaces, and undeclared paths are what it looks for first.

**Evidence:** `slice-003.md` §Scope (*Configuration* and *Tests* blocks);
`design.md` §9 AC-6 row (tier: boundary) and §8 R2; ADR-003 §Decision for what
`goad-boundary` owns; `docs/AGENTS.md` §Execute and §Audit.

**Disposition:** `fix-now`
**Response:**

Accepted, in both directions.

`crates/goad-boundary/tests/checks/` is added to `slice-003.md` §Scope's Tests
block, with the note that `crates/goad-boundary/src/` is unchanged — both AC-6
instruments are configurations of machinery that already exists.

`crates/goad-shell/src/config.rs` moves from **unchanged** to **documentation
only**: one doc comment on `default_poll` recording that a value below the
spacing is accepted, honoured for the process's first scheduled firing, and
floored thereafter. That is where R2's mitigation claimed to live, so the
choice was to make the claim true or to drop it; making it true is the smaller
lie to nobody.

**Outcome:** `verified`. `slice-003.md` §Scope now carries
`crates/goad-boundary/tests/checks/` with a note that `goad-boundary/src` is
*not* touched because both instruments are configurations of existing
machinery, and `crates/goad-shell/src/config.rs` as **documentation only**, with
R2 naming that as *"the sole reason that file is in scope"*. Both declarations
match what the design actually does.

### F-12 — `MINIMUM_SPACING` puts a host operational policy in stratum 1, where its siblings are not

**Severity:** minor
**Location:** `design.md` §5.2, §5.1 (the diagram's `s1` box); ADR-001
§Decision; `crates/goad-shell/src/config.rs`

**Expected:** stratum 1 is *"protocol types, wire-to-canonical normalization,
schedule resolution"* (ADR-001 §Decision). Host operational budgets live above
it.

**Observed:** `wait_for` takes `floor` as a parameter, so the *function* is
policy-free and belongs in stratum 1 without argument. The *constant* is a
different thing: 3 seconds is a decision about how often this host is willing
to run its own work (D-2), it is read only by stratum 3's loop, and every
sibling policy lives elsewhere — `ScheduleConfig::default_poll` and the backend
timeout in stratum 2's `config.rs`, the transport's cleanup budget in stratum
2. `draft-spec.md` §6 says so itself: *"The host owns: the pending wait, the
minimum spacing, and the decision to fire"* — and D-4 puts the host's waiting
in stratum 3.

This is spirit rather than letter, and none of POL-001's four instruments sees
it: a `const` is not a clock reach, not a manifest entry and not a crate edge.
That is precisely why it is worth naming — ADR-001's own Negative consequences
predict *"placement questions … that the three names do not settle on their
own"*. Moving the constant beside the loop that applies it costs nothing and
leaves stratum 1 holding arithmetic only.

**Evidence:** `design.md` §5.2; ADR-001 §Decision and §Consequences (Negative,
second bullet); `crates/goad-shell/src/config.rs` for where `default_poll` and
`timeout` live; POL-001 §Verification's four-instrument table for why nothing
catches it.

**Disposition:** `fix-now`
**Response:**

Accepted. `MINIMUM_SPACING` moves to `crates/goad/src/controller.rs`, beside
the loop that applies it, and `wait_for` loses its `floor` parameter — stratum
1 keeps `max(next_check - now, 0)` and nothing else.

F-2's repair makes the move forced rather than merely tidy: the floor is now a
`max` against a `tokio::time::Instant`, a type stratum 1 cannot name, so the
policy could not have stayed there even if placement were only a matter of
taste. The finding's own point stands and is recorded in `design.md` §3 —
none of ADR-001's four instruments sees a policy constant placed downward, so
this placement is held by argument, not by scan. D-14.

One consequence stated rather than hidden: the floor arithmetic leaves the
exhaustively unit-tested pure tier. `design.md` §9 says so and names AC-4's
and AC-5's anti-spin windows as what holds it instead.

**Outcome:** `verified`, and the repair went further than the finding. `MINIMUM_SPACING`
moves to `crates/goad/src/controller.rs`, and `wait_for` loses the floor
parameter entirely: it is now `wait_for(next_check, now) -> Duration`, pure
`max(next_check - now, 0)`. §5.2 states the principle — *"What stratum 1 keeps
is a difference between two instants — a quantity with no policy in it"* — and
D-14 records that D-3's monotonic anchor makes the placement forced rather than
merely tidy, since stratum 1 cannot name a `tokio::time::Instant`. §10 adds
that no ADR-001 instrument had to be extended to hold the rule.

### F-13 — `slice-003.md` says all nine open questions are closed and then leaves OQ-7 open in the same list

**Severity:** minor
**Location:** `slice-003.md` §Open questions, OQ-7

**Expected:** the artefact holds current truth (`docs/AGENTS.md` §Where it
goes).

**Observed:** the section opens **"All nine are closed."** Eight entries are
struck through and carry an *Answered* line. OQ-7 is not struck through, has no
answer, and still reads *"`research.md` Thread 5 names spike S-1, which gates
this: a `tokio::time` sleep has never been shown to complete when polled by
Slint's executor under the `EnterGuard`."* S-1 was run on 2026-09-07 and OQ-7
was decided in favour of `tokio::time` (`design-log.md`, OQ-7 entry; `design.md`
§7 D-6). A reader checking the charter is told the opposite of what happened.

**Evidence:** `slice-003.md` §Open questions header line against its OQ-7
bullet; `design-log.md` entry *OQ-7: which timer facility?*; `research.md`
Thread 5 *Spike S-1 result*; `design.md` §7 D-6.

**Disposition:** `doc-wrong`
**Response:**

Accepted; a plain contradiction in the charter. OQ-7's bullet is struck
through and carries an *Answered (agent)* line citing spike S-1's date and
result, `research.md` Thread 5, and D-6. The header's "All nine are closed" is
now true of the list beneath it.

**Outcome:** `verified`. OQ-7 is struck through and answered with `tokio::time`, and the
spike is cited. The section header's claim and its contents now agree.

### F-14 — the `now` D-11 computes the wait from is not in scope where the sketch uses it

**Severity:** nit
**Location:** `design.md` §5.4 (code block), §7 D-11; `controller.rs:335-360`

**Expected:** the design's sketch reaches the values it names.

**Observed:** the re-arm reads `wait_for(…, now, floor)`. In the real loop
`now` is bound inside each `Command` match arm and moved into `Pending`
(`controller.rs:335-360`); `Pending` exposes only `exchanged()`, and the whole
`Pending` is consumed by the `call` block. At the re-arm site there is no `now`.
`Timestamp` is `Copy` (`canonical.rs:102`) so the fix is trivial — bind it at
iteration scope, or add a `Pending::now()` — but the design declares no change
to `Pending` and the sketch reads as if none were needed.

**Evidence:** `controller.rs:335-360` and `:371-380`; `canonical.rs:102`;
`design.md` §5.4.

**Disposition:** `fix-now`
**Response:**

Accepted; verified that `now` is bound inside each `Command` match arm and
moved into `Pending`, which exposes only `exchanged()` and is consumed by the
`call` block (`controller.rs:335-380`). `Pending` gains `now()` beside
`exchanged()`, and the loop binds `let requested_at = pending.now();` exactly
as it already binds `let exchanged = pending.exchanged();`. `Timestamp` is
`Copy` (`canonical.rs:103`). Declared in `design.md` §5.2 and in
`slice-003.md` §Scope, so it is no longer an undeclared change.

**Outcome:** `verified`. §5.2 adds `Pending::now()` beside `exchanged()`, and §5.4 binds
`let requested_at = pending.now();` next to `let exchanged = pending.exchanged();`
before the value is consumed. The accessor is declared as a surface this slice
touches, which it was not before.

### F-15 — S-1 did not measure the one `reset` shape AC-4 needs

**Severity:** nit
**Location:** `research.md` Thread 5 (*Spike S-1 result*, cases B2 and D);
`timer-probe.local.rs:79-127`; `design.md` §5.5 A-1

**Expected:** where the design says a shape was measured, the measured shape is
the one the loop uses.

**Observed:** the spike covers `reset` to a future deadline on a fresh sleep
(B1), `reset` to a future deadline on an already-fired sleep (B2), and an
elapsed deadline on a **freshly constructed** `sleep_until` (D). The loop's
AC-4 path is neither: it is `sleep.as_mut().reset(tokio::time::Instant::now() +
Duration::ZERO)` on the long-lived pinned sleep. The risk is small — D shows
tokio does not hang on an elapsed deadline, B2 shows `reset` after firing works
— but A-1 is presented as covering the design's re-arm, and this is the one
combination it does not run. Cheap to add if the probe is re-run.

**Evidence:** `timer-probe.local.rs:79-92` (B2) and `:118-126` (D);
`research.md` Thread 5's result table; `design.md` §5.4's re-arm line and §5.5
E-1.

**Disposition:** `doc-wrong`
**Response:**

Accepted; the residue is now stated rather than the claim widened. A-1 gains
a *What S-1 did not run* paragraph: the spike measured `reset` to a future
deadline on a fresh sleep (B1) and on a fired sleep (B2), and an elapsed
deadline on a freshly constructed `sleep_until` (D); the loop's AC-4 path is
`reset` to an already-elapsed deadline on the long-lived pinned sleep, which
B2 and D bracket but neither runs.

Not re-run, deliberately: AC-4's own `serve` test **is** that measurement, and
it is a test that ships rather than a probe that is deleted. If tokio failed
to complete that reset, AC-4 would hang and fail its bound rather than pass
quietly. `design.md` §9's AC-4 row says so, so the evidence is attached to the
criterion that depends on it.

**Outcome:** `verified`. A-1 gains *What S-1 did not run*, which names the fourth
combination exactly — `reset` to an already-elapsed deadline on the long-lived
pinned sleep — says B2 and D bracket it, and hands the measurement to AC-4's
own `serve` test with the reason it is a real measurement (*"if tokio failed to
complete that reset, that test would hang and fail its bound rather than pass
quietly"*). §9's AC-4 row says the same. Closed by a test rather than by a
re-run of the spike, which is the better answer.

### F-16 — property 3 understates the effect for a repeating relative cadence

**Severity:** nit
**Location:** `design.md` §5.4, property 3; §7 D-11

**Expected:** the stated error bound covers the repeating case, since the
default poll is the repeating case.

**Observed:** property 3 says the wait *"errs by waiting longer than instructed
by exactly the exchange's own duration — never shorter."* For an **absolute**
instruction that is exact: the firing lands one exchange after the instant, and
the next instruction re-anchors. For a **relative** cadence there is no anchor:
`next_check = request_now + poll`, the wait is `poll`, and the firing lands at
`request_now + exchange + poll`, so the *period* is `poll + exchange` on every
cycle. Lateness against the ideal cadence therefore accumulates without bound,
even though the per-cycle error is what property 3 says it is.

Attacked and it does not break anything: the exchange measured 2.9 ms
(`research.md` Thread 5, case C) against a default poll measured in minutes,
and no absolute instant drifts. But "errs by exactly the exchange's own
duration" reads as a one-off, and the repeating case is the common one. One
clause fixes it.

**Evidence:** `design.md` §5.4 property 3; `schedule.rs:221-236` arm 3
(`now + default_poll` from the request's own `now`); `research.md` Thread 5
case C for the exchange cost.

**Disposition:** `doc-wrong`
**Response:**

Accepted. Property 3 now separates the two cases: for an **absolute**
instruction the firing lands one exchange duration late and the next
instruction re-anchors, so the error does not accumulate; for a **relative**
cadence there is no anchor, the realised period is `default_poll + exchange`
every cycle, and lateness against an ideal cadence grows without bound. The
magnitude is stated with it — 2.9 ms against a default poll in minutes, about
one part in twenty thousand — so the clause records a property rather than
implying a defect.

**Outcome:** `verified`. Property 3 now separates the absolute case (one exchange late, no
accumulation) from the relative one (realised period `default_poll + exchange`,
lateness growing without bound) and quantifies it at one part in twenty
thousand from S-1's own measurement. That is more than the finding asked for.


### F-17 — AC-6's stratum 3 instrument fails on the tree as it stands, and the design states the opposite as verified fact

**Severity:** major
**Location:** `design.md` §9 (AC-6 row, instrument (a)); `slice-003.md` AC-6;
`crates/goad/src/wire.rs:208`, `:215`; `crates/goad-boundary/src/scan.rs:225-234`

**Expected:** a new instrument passes on the tree it is introduced against, and
the design's statement of why it passes is checkable and true. `design.md` §9
says: *"It passes today: the token's five occurrences under `crates/goad/src`
are all in comments."* `slice-003.md` AC-6 says the identifier *"appears in no
code line under `crates/goad/src`."*

**Observed:** neither is true. `resolve` occurs **three** times under
`crates/goad/src`, not five, and **two of the three are code**:

```
crates/goad/src/controller.rs:255:/// One exchange, resolved but not yet started …   ← comment, and `resolved` does not match
crates/goad/src/wire.rs:208:  async fn stopped_resolves_immediately_when_already_tripped() {
crates/goad/src/wire.rs:215:  async fn stopped_does_not_resolve_until_stop_is_called() {
```

Both `wire.rs` lines trip the instrument, for two independent reasons in
`goad_boundary::scan::mentions`:

- it splits on every non-alphanumeric character (`scan.rs:231`), so
  `stopped_does_not_resolve_until_stop_is_called` yields the bare word
  `resolve`; and
- `is_singular_or_plural_of` strips a trailing `s` (`scan.rs:402-407`), so
  `stopped_resolves_immediately_when_already_tripped` yields `resolves`, which
  matches `resolve`.

`Scan` also has no `#[cfg(test)]` cutoff — it excludes directories by name
(`scan.rs:126-131`) and reads every line of every file it visits — so an
inline test module inside `crates/goad/src` is scanned as ordinary source.
The design says instrument **(b)** reads *"production code only (to the file's
own `#[cfg(test)]`, comments stripped)"* and says nothing of the kind for (a),
which is exactly where the difference bites: `wire.rs`'s test module is inline.

**Three things follow, and only the first is a naming accident.**

1. The instrument is red on arrival. Whoever implements AC-6 will land a failing
   gate and then be under pressure to weaken the check, which is the situation
   POL-001 §Compliance exists to forbid.
2. The false-positive surface is wider than the design examined. It correctly
   rules out `resolved`, but not the class: any stratum 3 identifier with a
   `resolve` segment matches, including `resolve_from`, `resolve_at`,
   `resolves`, and — through `camel_segments` — `resolveLater`. Two of those
   are names stratum 2 already uses, so a stratum 3 helper borrowing the
   vocabulary is not far-fetched.
3. The design's own evidence was not checked against the tree. Round 1's F-3
   turned on the same kind of claim, and this response replaced one unverified
   count with another.

**Two repairs, and they are not equivalent.** Cutting (a) at `#[cfg(test)]` the
way `structure.rs` already does makes the instrument true today and scopes it to
the property AC-6 actually states, which is about production code. Renaming the
two test functions makes the gate green while leaving a scan that forbids an
English word in test code, where forbidding it buys nothing. The first is the
fix to the class.

**Evidence:** `grep -rn "resolve" crates/goad/src/` returns exactly the three
lines above; `crates/goad-boundary/src/scan.rs:225-234` (`mentions`),
`:402-413` (`is_singular_or_plural_of`, `is_es_plural`), `:127-133`
(`is_excluded`, directories only), `:175-196` (`inspect`, every line of the
file); `crates/goad-boundary/tests/checks/structure.rs:9-14`, `:60-70` for the
`#[cfg(test)]` cutoff that exists and is not used here.

**Disposition:** `fix-now`
**Response:**

Accepted in full, including point 3. Verified by running it:
`grep -rn "resolve" crates/goad/src` returns exactly the three lines quoted —
`wire.rs:208`, `:215`, `controller.rs:255`. Round 1's "five occurrences, all in
comments" came from a case-insensitive word-boundary grep, which is precisely
the wrong instrument for a check about `mentions`, and it is the same failure
F-3 was about.

**The repair is the `#[cfg(test)]` cutoff, not a rename.** Instrument (a) is no
longer a `goad_boundary::scan::Scan`: `Scan` excludes directories by name and
reads every line of every file it visits (`scan.rs:127-133`, `:175-196`), so an
inline test module inside `crates/goad/src` is ordinary source to it. Both
instruments are now built on `structure.rs`'s machinery instead —
`production_lines` cuts at the file's own `#[cfg(test)]` and passes each line
through `code_of`, over a recursive `.rs` walk with its own vacuity guard
(`structure.rs:60-86`, `:97-125`). `wire.rs`'s test module opens at `:185`, so
the cutoff removes both names. The functions are not renamed: an instrument
that forbids an English word in test code holds nothing.

**Both instruments re-run, and the counts are now in the design where the claim
is made.** Simulating `production_lines` plus a word match: `crates/goad/src`
yields **0** occurrences of `resolve` over **12** `.rs` files;
`crates/goad-shell/src` yields **2** occurrences of `schedule::resolve` over
**8** files, at `host.rs:128` and `:259`. Each instrument additionally asserts
its walk inspected a non-zero file count, so neither passes vacuously.

Point 2 is accepted and reclassified rather than mitigated: the instrument
forbids the *segment*, so `resolve_from`, `resolve_at`, `resolves` in stratum 3
production code fail too. That is intended — stratum 3 has no business
resolving anything — and §5.5 I-1a now says so as a stated residue rather than
leaving it to be discovered.

**Outcome:** `verified`, and re-measured rather than read. Both instruments move off
`scan::Scan` onto `structure.rs`'s `production_lines`, which cuts at the file's
own `#[cfg(test)]` line and passes each line through `code_of`
(`structure.rs:61`, `:74`). I re-ran both walks myself rather than trusting the
figures: simulating `production_lines` plus a word match gives **0 hits over 12
files** under `crates/goad/src`, and the path form gives **2 hits over 8 files**
under `crates/goad-shell/src`, at `host.rs:128` and `host.rs:259` — the design's
numbers exactly, including the two line numbers it no longer pins the test to.
`wire.rs`'s two test function names now fall outside the walk because they sit
after that file's `#[cfg(test)]`, which is the class fix rather than the rename.
Each instrument asserts a non-zero file count, so neither can pass vacuously.
`slice-003.md` §Scope still declares no change to `crates/goad-boundary/src`,
and that holds: `code_of` is already `pub` in `scan.rs` and `production_lines`
lives in the test tree the scope does declare.

### F-18 — AC-9's scheduled-firing half is not discharged by the test §9 names, and the obvious way to discharge it breaks the margin table

**Severity:** minor
**Location:** `slice-003.md` AC-9; `design.md` §9 (AC-9 row and margin table);
`crates/goad/src/clock.rs:16`

**Expected:** every acceptance criterion maps to a design section that
discharges it, which is the design stage's own obligation
(`docs/AGENTS.md` §Design).

**Observed:** AC-9 has two halves. The first — a `ClockError` is reported as a
refusal and disturbs neither the retained instant nor a pending future deadline
— is discharged. The second is *"a clock that fails on a **scheduled** firing
produces one report per minimum spacing rather than a loop"*, and nothing in §9
reaches it.

§9's AC-9 row is *"a `serve` test with a clock that fails, asserting the refusal
line and a bounded invocation count across the anti-spin window"*, and the
margin table gives that window as 500 ms. Trace it: with a clock that always
fails, no exchange completes, so nothing ever re-arms the sleep, and the only
armed deadline is the initial one at `started + MINIMUM_SPACING` — three
seconds out. Inside 500 ms the timer arm never wins. The test observes the
*command* arm's refusal, which is the half already covered, and never enters
the path §5.4 property 5 and E-2 describe.

Widening the window to clear three seconds discharges it and costs more than
three seconds of wall time, against a table whose whole unavoidable budget is
1.8 s — so the naive repair invalidates F-7's repair. The cheap route is a
clock that succeeds once and then fails, which needs a `static` counter because
`Clock` is a `fn` pointer (`clock.rs:16`) and cannot capture: the first
exchange arms a 100 ms deadline, the timer arm wins, `stamp` refuses, and the
whole thing is observable inside the existing 500 ms window. Either way the
design has to say which, because the two differ by an order of magnitude in gate
cost.

**Evidence:** `slice-003.md` AC-9 as revised; `design.md` §9 AC-9 row and the
margin table's AC-9 line; §5.4's sketch (`floor_until = started`, initial arm at
`started + MINIMUM_SPACING`); `crates/goad/src/clock.rs:16` for the `fn` pointer;
`crates/goad/tests/renderer/wiring.rs:39-47` for the existing fixed-clock stub,
which has no failing counterpart.

**Disposition:** `fix-now`
**Response:**

Accepted, and the succeed-once clock chosen — the design now says which,
which was the finding's actual demand.

The trace is stated in §9's AC-9 row: the startup exchange completes and arms
~100 ms; the timer arm wins and advances `floor_until`; `stamp` refuses; the
deadline re-arms to `floor_until`, three seconds out. The assertion is one
`NoClock` refusal line, the retained instant unchanged, and the invocation
count still at one — all inside the existing 500 ms window, so F-7's budget is
untouched. Widening the window past three seconds is rejected for exactly the
reason the finding gives: it would cost more than the table's entire
unavoidable budget.

`Clock` is a `fn` pointer (`clock.rs:16`) and cannot capture, so the fixture is
a top-level `fn` over a `static` counter, beside `wiring.rs:44`'s existing
`stub_clock`. The margin table now carries two AC-9 rows, liveness and
anti-spin, where it carried one.

**Outcome:** `verified`. §9's AC-9 row now specifies a clock that succeeds once and then
fails, over a `static` counter because `Clock` is a `fn` pointer, and traces the
sequence that reaches the criterion's scheduled half: the startup exchange arms
~100 ms, the timer arm wins and advances `floor_until`, `stamp` refuses, the
deadline re-arms three seconds out. The row also says in terms why an
always-failing clock cannot reach it. The margin table gains two AC-9 rows and
the budget is unchanged — the anti-spin windows still total 1.8 s and the
liveness waits 0.7 s, which I recomputed against the twelve rows.

### F-19 — §9's AC-3 row and §9's margin table describe different tests, and the row's version asserts nothing

**Severity:** minor
**Location:** `design.md` §9 (AC-3 row; margin table row *AC-3 later
supersedes*)

**Expected:** the validation table and the margin table describe one test.

**Observed:** they do not, and the row is the stale half. The AC-3 row's second
case reads *"Later: a near instruction then a far one, **no** firing observed
inside a window well under the near value."* The margin table's row for the
same case is *"AC-3 later supersedes: 100 ms, then 60 s | anti-fire | no firing
| 300 ms window | 200x."*

A 300 ms window is three times **over** the near value, not well under it — and
it has to be. The test's whole content is that the 100 ms deadline did not
survive being superseded by the 60 s one. A window shorter than 100 ms would see
no firing whether superseding worked or not, so the row as written specifies a
vacuous assertion. The table is right; the row is a round-1 sentence that
survived the repair.

The stated margin measures the wrong quantity for the same reason. 200x is
60 s ÷ 300 ms, the ratio to the instruction that must *not* fire — which is not
what could go wrong. The meaningful ratio is 300 ms ÷ 100 ms = **3x**: the
window against the deadline the test is proving was cancelled. That is still
load-safe, because load can only delay a firing and so only cause a false pass,
but 3x is the number a reader needs and 200x is not.

**Evidence:** `design.md` §9, the AC-3 row against the margin table's AC-3
anti-fire row; `slice-003.md` AC-3 for what the criterion requires.

**Disposition:** `fix-now`
**Response:**

Accepted; the row was the stale half and is rewritten to the table's test.
The AC-3 row now states the 100 ms instruction, the 60 s supersession and the
300 ms window, and says why the window must be *longer* than the superseded
deadline: a shorter one would see no firing whether superseding worked or not.

The margin is corrected to **3x** — the window against the 100 ms deadline the
test proves was cancelled — with a paragraph saying why 60 s ÷ 300 ms measures
nothing that could go wrong, and why 3x is still load-safe: load can only delay
a firing, so it can only cause a false pass, never a false failure. That is the
one row in the table whose margin is small, and it now says so plainly instead
of hiding behind a large irrelevant number.

**Outcome:** `verified`. The AC-3 row now says the window must be *longer* than the
superseded 100 ms deadline and gives the reason, and it names the margin table's
row as the same test. The margin is restated as **3x the superseded 100 ms
deadline** rather than 200x against the instruction that must not fire.

### F-20 — the margin table claims to cover every timed assertion and omits AC-7's

**Severity:** nit
**Location:** `design.md` §9 (margin table preamble); `slice-003.md` AC-7

**Expected:** *"Every timed assertion, its expected time, its bound, and the
ratio"* means every one.

**Observed:** ten rows, and AC-7's is not among them. AC-7 is discharged by
*"the existing cancellation tests, extended with a stop issued while the loop is
waiting on the timer arm"*, which is a liveness assertion of exactly the kind
the table exists for: it waits for `serve` to return and fails on a bound if it
does not. It is the one new timed assertion whose failure mode is a hang rather
than a late value, so its bound is the one a reader would most want stated.

**Evidence:** `design.md` §9's margin table and its preamble; the AC-7 row of
the same section; `crates/goad/tests/renderer/wiring.rs:28` (`TIMEOUT`, 2 s) for
the bound the existing cancellation tests already use.

**Disposition:** `fix-now`
**Response:**

Accepted. AC-7's assertion is now a row: expected `serve` returns at once,
bound `TIMEOUT` (2 s, `wiring.rs:28`), margin ~2000x, wall cost ~0. The §9 AC-7
row also now names how the loop is put into the waiting state — a far
`default_poll` — and says why the assertion belongs in the table: its failure
mode is a hang rather than a late value. The preamble says "all eleven" rather
than leaving "every timed assertion" to be checked.

**Outcome:** `verified`. The margin table gains an AC-7 row: a stop while waiting on the
timer arm, expected to return at once, bounded by the existing `TIMEOUT` of two
seconds (`wiring.rs:28`).

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

**Round 1, raiser's synthesis — 2026-09-07. Written before any disposition, so
it is a verdict on the artefact rather than on the repairs.**

**Verdict: the design is sound in shape and defective in three places that
matter. No blocker.** Sixteen findings: four `major`, nine `minor`, three
`nit`. The central choices survive attack. The seam really is one wire
(`controller.rs:136-146`), the wait really does belong in stratum 3, `tokio::time`
really is the right facility and the spike that gates it was run honestly, and
the floor really is the only thing standing between a backend that instructs
the past and an unbounded exchange loop.

**What held under attack.** The three assumptions the designer nominated all
survive, two of them intact and one only after correction.

- **The monotonic deadline (R3).** Correct as stated: on Linux
  `CLOCK_MONOTONIC` does not advance across a suspend, so a wait is not
  consumed by suspended time. The design says so, records it as a limitation
  rather than a property, and names the cheapest fix. Not a defect.
- **Computing the wait from the request's `now` (D-11).** Lateness does not
  compound against any anchor: an absolute instruction lands one exchange
  duration late and the next instruction re-anchors. What does accumulate is
  the *period* of a repeating relative cadence, which becomes `poll +
  exchange`, and the design's phrasing does not cover that. F-16, a nit, at
  2.9 ms per exchange against a default poll in minutes.
- **Defeating D-3's floor by alternating stimuli.** Not defeatable inside this
  slice: every non-scheduled stimulus is a person through a capacity-1 channel.
  It is defeatable in slice 004, which this slice's own Non-goals commit to,
  and the rule is being written into canon in a form that survives that slice
  unchanged. F-2.

**The four majors, and why each is more than a wording defect.**

- **F-1** — §5.5 E-1 and AC-4 both say a past instant fires "once" and that the
  next resolution "consumes the elapsed value". That is `resolve`'s *retained*
  arm; a backend that instructs the past on every response never reaches it.
  The design states the correct behaviour elsewhere (§5.4, and `draft-spec.md`
  §5), so it holds both readings — and the wrong one is attached to the
  acceptance criterion, where it implies the schedule self-corrects and the
  floor is redundant.
- **F-2** — D-3's rule is canonized as "spacing applies only after a scheduled
  predecessor". Its entire justification is that the other stimuli are human.
  Slice 004 adds a non-human one and inherits R-5 with no note that its premise
  moved.
- **F-3** — AC-6's grep is defeated by
  `use goad_semantics::schedule::{MINIMUM_SPACING, resolve, wait_for};`, and
  this design is what makes that import line exist for the first time. Its
  admitted set is also smaller than the tree's, and it pins line numbers the
  design's own §2 already gets wrong.
- **F-4** — the loop sketch calls `.expect` in production, which
  `expect_used = "deny"` refuses. The lint is the symptom; the cause is an
  `Option` unwrapped at the one site the design argues is total.

**The pattern behind the minors.** Six of the nine are the same failure: a
statement that is true of the common path offered as true of all paths. Property
1 holds for two of four command arms (F-5). R1's one-sidedness argument is right
about anti-spin and backwards about liveness (F-7). A-1 says "measured" of a
topology with the platform substituted (F-8). SPEC-002 R-3's "promptly"
forgets R-4's three seconds, on words the design already corrected once in the
charter (F-9). The diagnostic line shows the instruction and calls it the next
check, in three states where the host will fire elsewhere (F-10). The charter
says nine questions are closed above a list where one is open (F-13). None is
individually serious. Together they say the design's prose ran slightly ahead of
its own analysis, and the analysis is otherwise good.

**What this review did not test.** The plan and the implementation, which are
not yet written; the *number* 3 seconds, which is a user decision recorded in
`design-log.md` and outside a reviewer's gift; and the actual wall-clock cost of
the new tests, which cannot be measured until they exist — F-7 asks for the
arithmetic instead.

**Risks this review would leave standing if every finding were fixed as
proposed.** R3 (suspend) is real, stated, and correctly deferred. The floor's
invisibility to the backend is a deliberate refusal (`draft-spec.md` OQ-2) and
should stay one. The residual that nothing can close is F-8's: no headless test
can run the production Slint platform, so AC-10 will always be production minus
one component, and the honest move is to say so rather than to keep looking for
a harness that does not exist.

**Round 1, responder's note — 2026-09-07. Written after integrating the
repairs, and acting as responder, not raiser: no `Outcome` column is filled
below, because a repair is not verified by the agent that made it.**

All sixteen accepted, none rejected outright, one (F-10) accepted in part with
the rejected half rejected on evidence and recorded as a draft SPEC-002 open
question rather than dropped. No acceptance criterion was weakened to clear a
finding; AC-4, AC-6 and AC-10 each became *harder* to pass — AC-4 now asserts
both successor cases, AC-6 now needs two instruments instead of one grep, and
AC-10 now requires the substitution to be stated as well as the topology to be
production's.

**Two findings changed the design rather than the prose.** F-2's repair
re-anchors the minimum spacing to the previous scheduled firing on the
monotonic clock, which removes the premise slice 004 retires and, incidentally,
closes F-6 (one write site where the bit had two) and forces F-12 (the constant
cannot live in stratum 1 once the floor is a `max` against a type stratum 1
cannot name). F-4's repair makes `absorb` return the instant it resolved, so
the loop's read is total and the `Option` survives only where it is honest.

**One finding the review did not raise, found while sweeping F-5's class:** an
instruction further out than tokio's `MAX_SAFE_MILLIS_DURATION` is clamped and
fires early rather than overflowing. Recorded as `design.md` §5.5 E-6.

**Two places outside the four artefacts carried F-1's defect and were swept
with it.** `research.md` Thread 4 H-1 said the "once" was guaranteed by
`resolve` consuming the elapsed value, which is true only of the consuming arm;
corrected in place, since research is the evidence the design leans on.
`crates/goad-semantics/src/schedule.rs:198-201`'s own doc comment says the same
thing — *"stored as given, fires once, and then falls back to cadence"* — and
is the place a future reader is most likely to meet it. No code was touched
this round: it is declared in `slice-003.md` §Scope as a documentation
correction and carried by `design.md` §5.5 E-1 for the phase that adds
`wait_for` beside it.

**What a round 2 should attack first.** The new floor rule, on the same ground
round 1 attacked the old one: whether anything can clear `floor_until`, whether
the timer arm is the right write site given a refused firing still advances it,
and whether "the first scheduled firing of the process is unfloored" is a hole
rather than the deliberate concession AC-1 depends on. Then the margin table in
§9, which is arithmetic from the design's own numbers and has not been
measured. Then AC-6's two instruments against the tree, since both are new and
one of them is a word-token scan whose false-positive surface is stratum 3's
future vocabulary.

---

**Round 2, raiser's synthesis — 2026-09-07. All sixteen round-1 findings
`verified`; four new, one of them `major`.**

**Verdict: the repairs are good, and one of them is red on the tree.** Every
round-1 finding is discharged, several of them wider than they were raised —
F-1's response found the same partial claim in `schedule.rs`'s own doc comment,
F-2 removed the premise instead of annotating it, F-12 took the floor out of
`wait_for`'s signature as well as out of stratum 1, F-16 quantified the drift.
Nothing was dispositioned `aligned` or `tolerated`, and no severity was
negotiated down.

**The three mechanism changes, attacked on their own terms.**

- **`floor_until` (D-3) holds.** It is a monotonic instant with one write site,
  monotonically non-decreasing, cleared by nothing. I tried to get a second
  unfloored scheduled firing out of it — a diagnostics toggle, a click storm, a
  refused fire, an hour of purely human traffic before the first scheduled one —
  and could not: the anchor grants exactly one unfloored firing per process,
  where the `bool` it replaced granted one per human action. **The first firing
  being unfloored is the concession, not a hole.** It is what lets AC-1 observe
  a 100 ms `default_poll`, it is stated in four places (§5.4, A-3, D-5, E-5,
  and draft SPEC-002 §5), and it falls out of R-4's own scoping — *"the
  scheduled evaluation that preceded it"* is vacuous when there is none — rather
  than needing an exception.
- **The margin table (R1) is arithmetically sound and descriptively wrong twice.**
  Every ratio recomputes from the table's own numbers, and the 1.8 s / 0.7 s
  split against ADR-003's measured 5.276 s is right. But the AC-3 row of §9 and
  the AC-3 row of the table describe different tests, and §9's version is
  vacuous (F-19); and the table claims to cover every timed assertion while
  omitting AC-7's, the one whose failure mode is a hang (F-20).
- **AC-6's instruments (D-16) are the right two, and (a) fails today.** The
  identifier form closes the brace-group defeat completely — I could not
  construct an import or aliasing style that evades it short of a stratum 2
  re-export, which the design names as its residue — and the path form is
  *forced* for stratum 2 rather than merely chosen, because `mentions` splits on
  `_` and would count `resolve_from` three extra times. What the response did
  not do is run it: `resolve` occurs three times under `crates/goad/src`, not
  five, and two are code, not comments (F-17). That is round 1's F-3 pattern
  repeating one level down — the reasoning was checked and the tree was not.

**The one refusal, and it is reasonable.** F-10's rejected half — not surfacing
the deadline beside the instruction — rests on a fact that holds: the deadline
is a `tokio::time::Instant`, and rendering it needs a second clock read per
frame and a fallible conversion, which is a new failure path in the renderer for
a value the parenthetical already warns about. Recorded as draft SPEC-002 OQ-3
rather than dropped. Accepted.

**What is left.** F-17 must be fixed before AC-6 can land, and the fix that
addresses the class is to cut instrument (a) at `#[cfg(test)]` the way
`structure.rs` already does, not to rename two test functions. F-18 is a real
gap between AC-9 and its discharge whose naive repair would cost more gate time
than the whole margin table budgets. F-19 and F-20 are corrections to §9. None
gates acceptance, and nothing found in round 2 disturbs the design's shape:
after two rounds the mechanism is sound and what keeps failing is the habit of
stating a checked-sounding fact about the tree without running the check.

**Round 2, responder's note — 2026-09-07. Responder, not raiser: no round 2
`Outcome` is filled.**

All four accepted, all `fix-now`, none rejected. F-17 is the one that mattered:
the round 1 repair replaced an unverified count with another unverified count,
and the instrument it justified was red on the tree. The repair is the
`#[cfg(test)]` cutoff — the class fix the finding names — and both instruments
are now built on `structure.rs`'s existing machinery rather than on `Scan`,
which has no such cutoff.

**Every tree claim in the design was re-run this round, and the counts are
written where the claim is made.** `crates/goad/src`: 3 raw `resolve` lines, 0
production occurrences over 12 `.rs` files. `crates/goad-shell/src`: 2
production occurrences of `schedule::resolve` over 8 files, at `host.rs:128`
and `:259`. Both instruments now also assert a non-zero inspected file count,
so neither can pass vacuously.

Nothing else changed. No acceptance criterion was weakened: AC-6 gained a
scoping rule and two measured counts, AC-9 gained the fixture that actually
reaches its second half, AC-3's row gained the assertion it was missing, and
the margin table gained a row and lost an inflated ratio.

**What a round 3 should attack first.** The margin table's AC-3 row, which is
now the only 3x in it and rests on the claim that load can only delay a firing.
Then AC-9's succeed-once trace, which is arithmetic on the loop sketch and has
not been run. Then whether instrument (a) forbidding the `resolve` segment in
stratum 3 production code is a rule anyone will later want to except, since an
excepted scan is a scan that has stopped holding anything.

---

**Round 3, raiser's synthesis — 2026-09-07. Ledger resolved.**

**Verdict: all twenty findings `verified`. Nothing outstanding at any severity —
0 blocker, 0 major, 0 minor, 0 nit. No finding was withdrawn, none was
dispositioned `aligned` or `tolerated`, and no severity was negotiated down.**

Round 3 verified the four round-2 repairs and raised nothing new. The one that
mattered, F-17, is the only repair this review has checked by re-running the
instrument rather than by reading the argument for it, and the design's figures
survived that: 0 occurrences over 12 files in stratum 3, 2 over 8 in stratum 2
at `host.rs:128` and `:259`. The repair is also the right shape — both AC-6
instruments now sit on `structure.rs`'s `production_lines`, so the
`#[cfg(test)]` cutoff is machinery they inherit rather than a rule each
restates, and the two `wire.rs` test names fall out of scope because the
property was scoped correctly, not because they were renamed.

**What three rounds changed.** The floor stopped being a `bool` resting on a
premise slice 004 retires and became a monotonic anchor that nothing clears.
`wait_for` lost its floor argument and stratum 1 went back to holding
arithmetic only. The loop lost its `expect`. AC-6 went from one defeatable grep
to two instruments with stated residues, both green on the tree. Every timed
assertion acquired an expected time, a bound and a ratio. AC-9's scheduled half
acquired a test that reaches it. And four separate places where a claim true of
the common path was written as true of every path — E-1, property 1, property 3,
A-1 — now name the paths they do not cover.

**What it confirmed.** The design's shape never moved: one wire, the wait in
stratum 3, `tokio::time` on a spike that was honestly run, and a floor that is
the whole answer to the busy-loop hazard rather than a belt-and-braces on one.

**The risks it knowingly leaves standing.** R3, the suspend limitation, stated
and deferred with the cheapest fix named. The floor's silence toward the
backend, a deliberate refusal recorded as draft SPEC-002 OQ-2, and its silence
toward the person, recorded as OQ-3 — the diagnostic line now says which of the
two instants it renders, which is the honest cheap repair rather than a fix.
AC-6's two stated residues: a stratum 2 re-export under another name, and a
third stratum 2 call spelled through a bare imported `resolve`. And AC-10's,
which nothing can close: no headless test runs the production Slint platform,
so the topology is proved minus one component and A-1 says which.

The margin table's figures are estimates from the design's own numbers and have
not been measured. The plan re-measures the gate before and after, and a phase
that finds them badly wrong is a finding rather than a rounding error — which is
the right place to have left it.

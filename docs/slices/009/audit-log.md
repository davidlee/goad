# Audit log — Slice 009

Append-only, time-ordered. What the user was asked during audit and
reconciliation, and what they decided. The reasoning that produced a finding
lives in `review-code.md`; the closing argument lives in `audit.md`. This file
holds only decisions, cited to the finding or question that prompted them.

Same shape as `design-log.md` and `plan-log.md`, and created for the same
reason: `docs/AGENTS.md` §*Where it goes* puts a user decision in a log, and
the audit stage had no log of its own.

---

## 2026-09-20 — four decisions, taken together after round 1 of `review-code.md`

### F-A1 — the `busy` class: **fix now, by narrowing `busy`**

`busy` is true for the whole backend round trip and Slint discards rather than
queues input for a disabled item, so **AC-4 and AC-5 are both unmet from one
cause**. Four options were put: narrow `busy`, suppress the busy present, waive
both criteria, or defer the class to a follow-up slice.

**Decided: narrow it** — `busy` comes to mean *your answer is in flight* rather
than *the host is talking to the backend*. Both criteria are then **met rather
than waived**, which is why this was preferred to the waiver the Closure
checklist would have admitted. The two rejected fixes are rejected for stated
reasons and not on cost: suppressing the busy present fixes the flash and
leaves the deafness, now invisible until a slow backend (VH-1's lead 1 says so
in as many words); and `AGENTS.md` forbids deferring a fix merely because it is
large, which this is not — `engaged` is read exactly once, into `frame.busy`
(`controller.rs:400`), and `engage()` has one production call site (`:921`).

### The slider gap — **readout now, aiming as a follow-up**

Two problems were priced separately once the audit found the readout is nearly
free: `glass.rs:568` already writes the `text` slot for a slider, from the
host's own `spelled(number)` — the exact string `submitted` sends.

**Decided: land the readout in this slice**, because screen and wire then agree
by construction and §4's P-3 is satisfied with no rounding and no wire change.
**Quantisation is a follow-up slice**, because snapping the reported value to
`step` makes `step` normative for the value and `slice-009.md` §Non-goals
declined exactly that. The readout will display the full unrounded spelling,
which is ugly — and is the argument for the follow-up, in a form a person can
see rather than one that has to be explained.

### CD-1 — **promote descriptively, and gain a cleared-number clause**

**Decided: the descriptive form.** A backend MAY NOT rely on the epoch to mean
*untouched*; it is what this host sends, `R-58`'s existing instruction — do not
send the field — remains the way to distinguish unanswered, and `OQ-2` stays
the real answer to the question a sentinel would half-answer. This keeps the
slice's *no protocol change* non-goal intact.

**And a clause CD-1 did not have**, exposed by **F-P2**: a cleared bounded
`number` shows an empty box and submits **its minimum**, not `0`. That is a
fourth screen/wire divergence; `design.md` §5.5 I-H does not list it, three
sentences in `design.md` state the opposite, and CD-1 as drafted does not reach
it because CD-1 is about *untouched* rather than *cleared*. A backend author
reading an empty box and a `2.5` on the wire can discover it nowhere.

### The record — **CD-2, the roadmap and the example all land; `SPEC-001` OQ-4 to be discussed**

**Decided: fix before close.**

- **CD-2, corrected and then promoted.** Not optional: `SPEC-001`
  §Verification's `R-58` row cites two test functions PHASE-09 deleted, so
  canon is untrue about the tree as it stands. Two corrections first — CD-2's
  Change 3 cites `undrawn_form`, renamed to `drawn_form` at PHASE-05, and the
  `R-57` row's closing sentence names an arm that moved to the same place.
- **`examples/shell/backend.sh`.** Outside every phase's Surfaces and now false
  in three places, one of which describes runtime behaviour a person running
  `just demo` watches the host contradict.
- **`docs/roadmap.md`.** `design.md` §10 says this is owed at close: the
  roadmap names 009 as `SPEC-001`/OQ-4's answerer and OQ-4 stays shut.

**`SPEC-001` OQ-4's own wording: opened for discussion rather than decided.**
The audit's position, put to the user and not yet answered: OQ-4's fork — *its
own kind, or a hint on `datetime`* — is **asymmetric**, because `R-18` already
permits a renderer and only a renderer to branch on a hint, so the hint half
needs no protocol change at all. The clause *"no evidence asks for one yet"* is
true on its own terms and is now the wrong sentence: what 009 found is not
inexpressibility but an affordance cost. The recommendation is an **evergreen**
replacement — no slice number, no history, since `roadmap.md:519-547` already
carries the narrative and canon carries no revision history.

---

## 2026-09-20 — round 1 completed, and every finding dispositioned

Round 1 grew from fourteen findings to **twenty-one** when the renderer
dimension's two unattacked areas were run as a fresh agent rather than read as
a clean surface (`review-code.md`, `F-R3`–`F-R9`). Two results changed what the
repairs have to do.

### F-R1 — **fix now**, and it is settled by measurement rather than argument

F-R1 was raised *reasoned, not run*. It is now run: a loop-tier target drives
it with controls on both sides of every claim, and it is **confirmed and wider
than it was written** — the picker survives `hide()` as well as a view
replacement, so a person can be left with a picker over nothing, and the form
beneath is unreachable by pointer *and* by keyboard while the widgets
themselves stay alive. The injection pass fixed the repair's shape at **one
call site**: `present`'s `self.shown != showing` branch covers `Shift::Replaced`
and `Shift::Closed` together. The case is red by design and lands with the
repair.

### F-R3 — **fix now, by draining `commands` before the present**

Narrowing `busy` (F-A1, decided last session) makes typing during an
`Evaluate` the ordinary case — which is exactly the window in which a debounce
tick enqueues an edit that `serve` has not yet served, leaving the present at
`controller.rs:839` to write the draft's stale value back over the widget.
**Repairing F-A1 without this one trades an exchange-long deafness for a
per-poll revert**, so the two land together.

Four shapes were put. **Decided: drain `commands` before the present** — apply
every queued command that resolves without an exchange, stopping at the first
that needs one. The reason it is preferred to the other three is that it is
what the three loop-tier harnesses already do (`overlay.rs:238-249`), which is
*why* the rig cannot see the defect: production is being made to do what the
rig does. Rejected on their shape, not their cost — moving the present is the
same effect in a less obvious form; holding the entry until the edit is served
reverses PHASE-05/EX-4, which is the only reason `Wire::send` returns a `bool`;
and a `commands` arm in the inner `select!` reopens the re-entrancy the design
closed.

### The `busy` narrowing needs no second flag, and no new decision

Put to the user as a design question and withdrawn as one: `Command::Edit` is
not an exchange (`controller.rs:766-776` returns `None`), and `Command::Choose`
has exactly one origin (`install.rs:40`). So `Pending::Respond` holds **iff the
person clicked an option button** — there is no `Respond` they did not
initiate, and the option `Button`'s slice-003 double-submit guard wants exactly
the narrowed meaning. One flag, one line: `controller.engage(exchanged)`.
`F-R2`'s sites 5 and 6 are answered — 5 becomes unreachable (a click cannot
reach the button beneath an open dropdown, `window.rs:843-871`), and 6 is F-R1
rather than a `busy` problem.

### Every remaining finding, dispositioned together

**fix-now:** `F-A1`, `F-R2`, `F-R1`, `F-R3`, `F-S2`, `F-S1`, `F-S3`, `F-S4`,
`F-S5`, `F-R4`, `F-R5`, `F-P3`, `F-P4`, `F-R8`, `F-R9`, `F-S6`.
**doc-wrong:** `F-P1`, `F-P2`, `F-R6`, `F-R7`.
**settle, then dispose:** `F-S7` — one mutation decides whether I-F is
unobservable or merely untested.

`F-S3` is dispositioned against the audit's re-derivation rather than the
finding's headline: the stated mutation does **not** lint clean, and the hole
is one shape only — a wildcard absorbing an existing kind with a body that
agrees with it. A boundary scan closes it.

### The budget, revised again — **four sessions**

Round 1 nearly doubled and two of the new findings want real code. Session 2
takes the three repairs that gate — `F-A1`/`F-R2`, `F-R1`, `F-R3` — and
checkpoints. Session 3 takes the remaining repairs and round 2. Session 4 takes
reconciliation and close.

---

## 2026-09-20 — the two findings round 1 left blocked on a decision

Both were put to the user in session 3 and not answered; both are answered now,
at the head of session 4, before round 2 was launched.

### F-R4 — **re-dispositioned `fix-now` → `follow-up`**

The finding is correct and there is no repair inside this slice that is not a
design change. Established, not assumed, and re-derived this session rather
than inherited: `option_models` builds `rows` and `values` in **one** walk in
which the slot **is** `values.len()` (`glass.rs:343`), which is invariant I-B —
*"there is no second counter that could fall out of step with the vector's own
length"*. `values` is written on every present and only `rows` is conditional,
so splitting the walk to skip the discarded half reintroduces exactly the
second counter I-B forbids, and `CLAUDE.md` forbids the parallel
implementation that would be.

**Two things changed since it was raised, both in its favour.** The two costs
it was raised as the amplifier *for* are repaired: **F-R5** removed the
per-present tray push and **F-R3** removed the per-present guard revert. What
remains per refused arrival is one discarded `rows` build, two `VecModel`
allocations, an epoch bump whose guards then write nothing, and `show()`'s
instantiation pass — **CPU on the UI thread, with no user-visible
disturbance**.

**And the question underneath it is canon's, not this slice's.** The *inner*
loop already holds the opposite position deliberately: an arrival refused
during an exchange presents nothing (`controller.rs:1029-1048`), cited to
SPEC-003/R-15 and `review-design.md` F-15. R-15 requires a refusal decided
**while idle** to reach the diagnostics surface, so suppressing the outer
loop's present defers that to the next scheduled firing — and R-15's own
verification case reads `served.controller.frame(false).diagnostics`
(`renderer/ingress.rs:1230`), the retained model rather than the window, so
**canon's instrument would not report the change**. Deciding when an idle
refusal must become visible is a spec amendment with its own verification, not
a repair.

Lands in `slice-009.md` §Follow-ups, where a `follow-up` disposition is
required to land. It is not deferred for being large: it is deferred because
it is a different unit of work.

### F-P2 — **endorsed: `design.md` is corrected**

Explicit endorsement given for the edit `doc-wrong` requires. Two changes,
and the second is the one that carries the new information:

- The three sentences at `design.md:356`, `:1323` and `:1345` are corrected to
  the rule the code implements. They are residue of the **superseded** `to-float`
  reading (D-16), left behind when D-33 reversed it; removing them completes a
  revision the design already took rather than fitting the design to the code.
  `:356`'s purpose had gone independently — it justified a guard exception
  PHASE-08/EX-7 measured out.
- §5.5 **I-H**'s divergence list gains the fourth divergence: a cleared bounded
  `number` shows `""` and submits **its minimum**. This is not a correction of
  anything; it is a screen/wire divergence stated in no document at all, which
  is why `canon-delta.md` CD-1 gains the matching clause (decided above, this
  file).

---

## 2026-09-20 — two canon amendments the recorded endorsement did not reach

Reconciliation surfaced two changes to `SPEC-001` §Verification that the first
entry's *"CD-2, corrected and then promoted"* does not cover. Both were put to
the user with the evidence and both are endorsed.

### CD-2 Change 3 — **amended before promotion, because F-S3 landed after it was drafted**

Change 3 says the `R-55` row should name where the sixth-kind path is held:
*"`view_model.rs::undrawn_form`'s exhaustive match, which is a compile-time
guard rather than a case."* Two things are wrong with promoting that verbatim.
The identifier is the one PHASE-05 renamed — already known, already endorsed.
The **claim** is the new problem: **F-S3** established that an exhaustive match
is a guard against a sixth kind only while nobody absorbs it into a wildcard
arm, and a `_` arm compiles, lints clean under the workspace set, and leaves
the gate green. What holds the property is the match **plus**
`clippy::wildcard_enum_match_arm`, denied for the `goad` crate.

**Decided: amend Change 3, then promote.** The `R-55` row names both. Canon
that says the match is the guard would be true of the source and false of the
gate, which is the failure `docs/memory/a-count-in-a-comment-is-a-claim-nothing-checks.md`
and this slice's own three instrument findings are all instances of. The
amendment costs one clause; **`POL-001` is still untouched and the gate's
instrument count is unchanged**, because F-S3's repair is a lint-table entry
inside the existing clippy pass rather than a fifth boundary instrument.

### The `R-18` row — **confirmed, and the site named**

The row says *"`view_model.rs::present` reads exactly one key, `group`"*. This
was carried in `audit.md`'s table as *unverified — check before touching*. It is
now checked: the read moved to `Run::of` (`view_model.rs:258`), which `present`
still reaches through `sift` (`:363-368`). **The claim is true**; what has
decayed is its locating power, since a reader grepping `present` for `group`
finds nothing.

**Decided: confirm and name the site.** `Run::of` is named alongside `present`.
Canon carries no revision history, so it reads as the evergreen statement rather
than as a correction — which is the right shape, because nothing was wrong.

---

## 2026-09-20 — round 2 closed: thirteen dispositions, taken together

Round 2's two dimensions are both complete. Every finding they left open was put
to the user with its price stated rather than estimated, and two were
**verified before they were priced** — the lesson this audit has now learned four
times over.

### F-S5 — **`follow-up`**, and the contest was re-run before it was priced

The contest holds. Session 4 could not re-run it — the machine was carrying
eight concurrent `cargo` processes — so it was re-run here on a quiet one:
`app.slint:479-481` with the `CheckBox`'s assignment unconditional and the
`counted-bool` call dropped leaves **every `-p goad` target green** with
`reasserts` at `0`. The widget is written on every present, the caret is
destroyed on every tray check, and the instrument says nothing.

**Decided: `follow-up`.** The property — *every write to a guarded widget goes
through a counter* — is real, unheld, and now owned. The markup scan was priced
honestly against the other two options and rejected on its **canon** cost, not
its code cost: `POL-001` §Verification enumerates its instruments and
`CLAUDE.md` forbids compressing them into one count, so a fifth category is a
policy amendment taken mid-audit for a `minor`. `tolerated` was rejected because
it writes off a property this slice's own reviewer found and can state exactly.

What lands now is the Response's two overreaching claims withdrawn, and the
finding's own sentence left standing as true of the repaired tree.

### F-B4 — **drain's margin now; the stepper harness is a follow-up**

Two problems arrived in one finding and they are priced separately.

**Decided: fix the margin, defer the harness.** `drain.rs`'s
step 9 → step 13 bound must exceed the 150 ms debounce and runs at 200 ms — a
**1.33x margin**, and it is tight whether or not the machine is loaded. That is
a defect in the case and it is fixed in this slice.

The other half is not: all three new loop targets fail at roughly 6x
oversubscription (picker 4/8, drain 2/8, busy 1/8), and **the failure is the
liveness backstop rather than any assertion** — the slint-timer stepper stalls
for tens of seconds and `LIVENESS_BOUND` converts the stall into a red that
reads like a defect. A 40x nominal margin was not enough, so widening the bounds
is not the repair; the harness is. That is a different unit of work, and it is
deferred for that reason and not for being large.

**Two independent witnesses.** Session 4's own `just check` failed here on
`event_loop_busy` at loadavg 198, and the reviewer reproduced it under a
controlled batch and instrumented it at the bound, without being told. Recorded
because a load-sensitive `just check` sits against `POL-001`'s *the gate exits
0*, and slice 003's suite stayed green at the same load.

### F-B2 and F-B1 — both **`fix-now`**, and both are majors

**F-B2.** A picker survives `open_diagnostics()` and the diagnostics pane's only
exit button is then unreachable by pointer, reachable in production from the
tray menu. It is F-R1's class one surface over, and F-R1's own Response
predicted it. Measured **with a positive control** — the same driver, the same
element query and the same click raise the callback once the picker is gone — so
the blocked click is not a driver that missed. It is fixed rather than deferred
because the criterion F-R1 was repaired to meet is the same one this defeats.

**F-B1.** `serve`'s `engage` call site is held by nothing: replacing
`controller.engage(exchanged)` with `engage(Exchanged::Evaluation)` kills slice
003's double-submit guard in production and **the entire suite stays green**.
All nine cases that hold `busy` call `engage` themselves. Fixed now because the
half of a repair that *keeps* a behaviour is the half a refactor drops, and this
slice narrowed that exact line. Noted against it, and accepted: the case is a
fourth loop-tier target, which F-B4 has just established is the load-fragile
tier.

### The remaining eight — **`fix-now`, all of them**

Sentences, two lint attributes and one verified `const` assertion. None changes
behaviour except F-B9, and none is deferred.

**F-T1's closer was verified before it was priced, and is cheaper than its own
author proposed.** The proposal was to derive each target's step schedule from
`DEBOUNCE`; what was built instead is `pub` on the constant plus eight lines of
`const _: () = assert!(…)` in `full.rs`. Measured: green at the shipped 150 ms,
and F-T1's own 150 → 400 mutation now **fails to compile** —
*"reading B is vacuous unless the debounce deadline falls between step 3 and
step 11"*. Derivation would have rescaled silently and kept the case green,
which is a weaker instrument than the one the finding asked for.

`F-T2` (one `#![deny]` on the second crate root), `F-T3` (the undercount — six to
eight sites, not four, and one of the four characterised backwards), `F-T4`
(three citations pointing at prose), `F-B5` (a commit that corrected five false
doc claims and created a sixth), `F-B8` (one clause on the I-F comment), and
`F-B3` — whose repair is to **narrow the sentence** to the outer loop's present
and name the excluded site, rather than to drain before `controller.rs:1046`:
that present costs one widget revert per *process* and only after ingress has
died, and a behaviour change there buys less than the honest sentence does.

`F-B6` promotes the reviewer's four-line probe into a real case — it is the case
F-R5's repair owed, and without it the gate does not notice the repair's
removal. `F-B7` absorbs a view before engaging, which is what its own sibling
case already does.

**F-B9 is the orchestrator's finding and the user disposed it like any other.**
Retain a `VecModel` and `set_vec` into it, as `options` already does: three
lines, and it removes a parallel implementation of something the file otherwise
does one way.

## 2026-09-20 — round 3 closed: two re-dispositions and six findings, taken together

Round 3 reviewed round 2's twelve repairs, which no one had reviewed. One fresh
agent, its own worktree, told to confirm the tree before reading and not to read
`audit.md`. It set thirteen Outcomes by re-running every mutation each Response
names — **eleven `verified`, two `contested`** — and raised six findings.

**Its worktree arrived 38 commits stale, at `f352124`.** Step 0 caught it; it
was a clean ancestor with no unique commits, so it fast-forwarded and took every
reading at `0b0975b`. **Second recorded instance.**
`docs/memory/subagent-worktrees-can-be-stale.md` is no longer a one-off and
should say so at harvest: the check is not a formality, and a session that
skipped it would have reviewed a tree two sessions old and reported confidently
about it.

### The two contests — both confirmed at the source before they were priced

The rule that has now held six times: **verify before pricing.** Both contests
were re-read by the orchestrator at the vendored and production sources rather
than accepted from the report, and both survived.

**`F-B9` → `fix-now`.** The repair does not close the finding. Retaining the
`VecModel` closes the *pointer* path; `VecModel::set_vec` ends in
`notify.reset()` and `RepeaterTracker::reset` clears every instance **without
consulting the pointer at all**, so the *mutation* path stayed open and the
unguarded write rebuilt every line element on every present. The sibling the
repair copied — `options.set_vec` — is inside `if self.shown != showing`; **the
guard the Response explicitly rejected was the necessary half.** Measured 1→2→3
across three presents, 1→1→1 with a content guard.

This is the second time in this audit that **a repair was wrong about what it
held**, and the first where the Response named the very case that would have
caught it and declined to write it. F-B9's Response was honest about the
residue; the residue was the defect.

**`F-T3` → `fix-now`, by dropping the axis rather than restating it.** The
repair corrected a false count and wrote a **new false characterisation** into
production source: it called `ingress/mod.rs`'s `other => InvalidEnvelope(other)`
an arm that chooses no behaviour from the variant, when `Refusal::reason` splits
`InvalidEnvelope(ReservedSource)` from every other and the variant therefore
reaches the wire. Two attempts, wrong in opposite directions.

**Decided: delete the axis, do not re-characterise.** It is not load-bearing —
the conclusion rests on cost, which the corrected count establishes — and the
comment now says in as many words that no claim is made about it, and why. **A
claim nothing checks, restated, is how this comment went wrong twice.**

### F-C2 — the user's premise was corrected, and the decision changed with it

The user's first instinct was to patch best-effort and accept, on the ground
that the citations are *"of historical interest only after the slice is done"*.
**That is true of the slice folder and false of these:** they are doc comments
in `crates/goad/src/`, shipped, and read by whoever next opens the file.

**Decided with the premise corrected: repair the class, not the instances, and
add no gate instrument.** Every citation becomes a **symbol** rather than a line
number. A symbol cannot rot; re-numbering has now failed twice, the second time
in the very commit that closed `F-T4`. A gate check was considered and rejected
**on the user's explicit decision to close the slice rather than spike one** —
not on a cost estimate, which would have been an estimate.

**The residue is stated in the finding** rather than left implicit: nothing
enforces the symbol discipline. It joins `POL-001`'s dependency-feature residue
as a real property held by nobody.

### The remaining four — `fix-now`, except `F-C5`

**`F-C3`** (`major`) is a coverage finding and the reviewer proved it both ways:
the dismiss made unconditional leaves **all fifteen targets green**, and
production behaviour is separately measured correct. One reading added.

**`F-C1`** is the project's named failure mode in its exact shape — a green
reading whose message names a defect it would not catch. The clause is deleted
and what the control does *not* hold is written beside it. **F-B1's Response is
not edited**: the ledger is append-only, and the correction lives in F-C1 and in
F-B1's Outcome.

**`F-C4`** is one sentence. **`F-C5`** is `doc-wrong`: *"at no wall-clock cost"*
was true of the intervals and false of the total. **The orchestrator re-measured
rather than transcribe a figure it had not taken** — and its first instrument
was wrong, a clock declared inside the per-tick closure that reset every tick
and read 90 ns. It was rebuilt before any number was believed: ~877 ms over
three runs, against 799 ms, margin 3.75x → 3.42x.

### Round 4 — scoped to the repairs, and that is the user's decision

**Round 3's trend is not zero**, so the stopping rule does not apply and it was
not claimed to. What changed is the **shape**: round 1 was a blocker and six
majors of live defect; round 2, two majors about what holds a repair; round 3,
**one live defect, one coverage gap with behaviour separately measured correct,
and four claims wrong in prose.** The defects are gone; what remains is the
record disagreeing with the code.

**Decided: round 4 is a verification pass over round 3's repairs, not a fresh
adversarial round.** If it returns no code defect, the slice closes. One
session, not two.

### The tray icon — a **follow-up**, not a finding, and the reason matters

VH-2 turned up an observation nothing in the gate could have: two tray icons,
one of which disappeared and returned while **neither host process died**
(`NRestarts=0`, both PIDs continuous for twenty minutes). The non-recovery
mechanism is real and was confirmed — `glass.rs` calls `set_image` on every
present, but F-R5's repair made `tray_icon` return a stable-address clone and
slint's `ChangeTracker` fires only on `!=`, so **nothing re-registers an icon
the platform has dropped.**

**It is a follow-up and not a slice finding, deliberately.** What is missing is
the cause of the disappearance, and the user's own report was hedged. Raising it
would be substantiating a mechanism on half its evidence — the failure
`docs/memory/dont-feed-the-raiser-your-finding.md` and
`docs/memory/verify-the-enumeration-not-the-conclusion.md` both describe.

**The class is worth keeping** and is the harvest item: *a repair that removes a
redundant write also removes the self-healing that redundancy was accidentally
providing.* F-R5 was right; this is its unpriced half.

## 2026-09-20 — VH-2 completed, and the two things its last observation turned up

VH-2 is discharged in full: observations 5, 6, 8 and 9 in run 2 and observation
10 in run 3, all with `goad.service` stopped so run 1's two-host confound could
not recur. `notes.md` §VH-2 holds what was seen, in the user's words. **AC-2,
AC-3, AC-8, AC-9 and CD-1 all read positive**, and `F-R1` and `F-P2` are
confirmed on a person.

Observation 10 passed on evidence the orchestrator had framed wrongly twice,
and both corrections came from the run rather than from reasoning: the cadence
after a refusal is `default_poll`, not the delay (**SPEC-001/R-29**, R-26's
third branch — read back off the screenshot as 08:24:50Z − 30 min = the failure
instant), and **the diagnostic surface does not accumulate**, so "a refusal line
every N seconds" was never the shape of the evidence. What says the host carried
on is that it reports a resolved next check at all.

### The demo's `DELAY=6` path — **fix now**, comment and interval both

The documented knob prepares a person for one line and produces two, then goes
quiet for thirty minutes. Neither surprise is a host defect: the second line is
`backend.sh`'s own `sleep` surviving as a **grandchild** holding inherited
stderr — the exact class
`goad-shell/tests/integration/transport.rs::a_grandchild_holding_stderr_costs_the_cleanup_budget_and_nothing_else`
holds — and the silence is `default_poll`.

**Decided: fix both.** `demo.toml`'s `GOAD_DEMO_DELAY=6` comment now names all
three things to expect and why two of them are not defects, and
`default_poll` drops from `30m` to **30s** so the host is seen to check again.
30 s is above the 3 s minimum spacing, so nothing here starts testing the floor
(SPEC-002 §6), and no test reads this file's value.

The file is reachable by no gate command, which is why this is a decision rather
than a finding — and is the same reason `backend.sh`'s cadence comment was
recorded rather than raised.

### `next check (instructed)` — **follow-up**, and priced rather than estimated

The line reports `now + default_poll` — R-26's third branch, which R-26
distinguishes from an instruction in as many words — under a label that says
*instructed*. Not an edge case: a backend that never sends `next_check` is
legitimate (R-26 admits it; **SPEC-002/R-1**'s own verification case is exactly
that backend), so for such a backend **every** reported check carries the wrong
attribution. The harm is misdirection at the one moment the line is
load-bearing: a person debugging *"why isn't my `next_check` taking effect?"*
is pointed at the wrong side of the boundary, which is what the failure taxonomy
exists to prevent.

**Two things were established before the disposition, not assumed.**

- **The string predates this slice** — slice 003, `21811b7`, PHASE-04 — so it is
  outside this review's subject, `a698217..HEAD`.
- **The fix is not a relabel.** `schedule.rs::resolve` returns a bare
  `Timestamp`; the branch it took is discarded at the moment it is taken. Making
  the line honest means `resolve` reporting its branch, threaded through `State`
  to the diagnostics line — a change to a **pure stratum-1 function**, its
  callers and its verification.

**And the finding is weaker than it first reads**, which is recorded because the
severity turns on it: `SPEC-002` §6 and `diagnostics.rs::next_check_line`'s doc
are both entirely about a different axis — *instruction* as against
**prediction**. On that reading the word is precise about the contrast the spec
drew and merely silent on provenance, rather than false.

**Decided: `follow-up`.** It carries a spec question — what the line should say
for each of R-26's three branches — that belongs in SPEC-002 §6 and not in this
slice's close. Lands in `slice-009.md` §Follow-ups.

## 2026-09-20 — round 4 closed, and the F-D3 decision was taken twice

Round 4 ran as decided: a verification pass over session 6's seven repairs, not
a fresh adversarial round. **It returned no behavioural defect and no
`blocker`** — seven findings, `F-D1`–`F-D7`, five `minor` and two `nit`, every
one of them a claim in prose that is false of the tree. All thirteen of round
3's mutations reproduce as recorded. That is the condition the slice closes on.

**Its worktree arrived 42 commits stale** and fast-forwarded from a clean
ancestor. **Third instance**, and a fourth followed within the hour on the
citation-conversion agent, at the same commit `f352124`. This is no longer a
subagent hazard to be checked for; it is a defect in how worktrees are
provisioned here, and the harvest should say so.

**Two of the reviewer's sub-claims were corrected at the source before any
disposition was priced**, which is the seventh time this audit has done that and
the third time it changed an answer: `F-D3`'s enumeration and `F-D5`'s count.
Both corrections **widen** the finding rather than weaken it.

### Six repairs, all `fix-now`, and two of them delete rather than correct

`F-D1` and `F-D2` are the shape worth naming: both were off-by-one claims — *"All
six `init` handlers"* when there are seven, *"step 10 reopens"* when it is step
11 — and **both were repaired by removing the number, not by fixing it**.
*"Every `init` handler in the markup is `root.inits += 1`"* and *"the same
reason as the earlier reopen"* cannot go stale. Correcting the counts would have
been the third instance of a class this slice has already paid for three times.

`F-D4` replaces the account of the 75 ms with the arithmetic the step arms
support, and **keeps both measurements with their instruments named** rather
than choosing one: 876.9 / 877.2 / 878.3 and 876.1 / 875.6 / 874.9. Two
instruments agreeing to 0.4% is a stronger statement than either alone.
`F-D5` states what holds instead of a uniqueness that was false. `F-D6` stops
pricing a guard by a mechanism `set_vec` does not have. `F-D7` rewraps three
lines — and the finding's own stated margin is corrected, because these files
carry unwrappable 113-character identifiers and cannot keep a 78-column rule.

### `F-D3` — decided, then re-decided when the residue was measured

**First decision: fix the four, state the residue, add a rule to `CLAUDE.md`.**
Taken on the understanding that the residue was *"smaller than 59 and larger
than 3, and nobody has measured it"* — the audit's own words. A gate instrument
was again declined, consistent with session 6.

`CLAUDE.md` gained the rule under §Working here: **cite by symbol, never by line
number**, with the evidence attached and the vendored exception carved out,
because a blanket rule would send the next agent converting
`i-slint-core-1.17.1/model.rs:211`, where the line number is the only useful
form.

**Then the residue was measured, and the premise did not survive it.** A script
written for the mechanical check — the one the user endorsed in place of a fifth
review round — resolves every in-repo citation and prints what is actually at
the line. **53 in-repo citations; 27 land on a comment or a blank line; about 24
of those are simply wrong.** And the cause is concentrated:

| cited as | meaning | actually at | times |
|---|---|---|---|
| `main.rs:86` / `:87` | the capacity-1 command channel | `main.rs:98` | **9** |
| `main.rs:89` | `Wire::new` | `:101` | 1 |
| `main.rs:95` | the shared `Debounce` | `:106` | 2 |
| `main.rs:96-98` | the window outliving its callers | blank | 1 |
| `pending.rs:212-214` | *"the `if enqueued`"* | a doc line | 2 |

**One file's anchors moved and took thirteen citations across seven files with
them**, two of them inside `event_loop_full/full.rs`'s injection table — the
evidence for `F-S2`, a finding this audit raised and closed.

**Re-decided: convert the ~24 now.** The option first rejected had become much
cheaper (nine of the twenty-four are the same citation) and the option first
taken had become much weaker — *"state the residue"* would have meant writing
down that half the crate's citations are wrong and closing anyway. The 26 that
currently land on code are **left alone**: they are right today, the new rule
covers them going forward, and converting them is the full enumeration pass that
was declined for its own reasons.

**The measurement is the point.** Both decisions were correct on the information
available; what changed was that the information stopped being an estimate.
`docs/memory/price-the-rejected-option-against-code.md` is about not ruling an
option out on an estimate — this is its converse, and the harvest should carry
it: **a residue nobody has measured is not a small residue, and "state the
residue" is only honest once it has a number.**

### Round 5 — **not run**, and this is the stopping decision

Repairing seven prose findings writes new prose that nobody reviews, which is
precisely how `F-T4` became `F-C2` became `F-D3`. Three options were put.

**Decided: mechanical verification, no round 5.** These seven are counts, line
numbers, a wrap width and one arithmetic claim — every one of them
script-checkable, and every one of them a thing a *reading* agent has now got
wrong at least once. A fifth agent round would spend a session re-reading what a
script settles in a minute.
`docs/memory/review-rounds-stop-on-a-measured-trend.md` names exactly this exit,
and the trend here is stronger than the one that memory was written from: round
4 found **zero** code defects.

The resolver script is **not landed**. It stays in the session scratchpad, per
the standing decision to close the slice rather than spike a gate instrument.
Landing it is a follow-up and is recorded as one.

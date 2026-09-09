# Review — implementation — Slice 004

**Subject:** implementation — `9cfb679^..93abab3` on `main`, every code phase of
slice 004 (26 non-documentation files; the ingress module, the loop's two new
arms, the config key, the startup bind, and both test tiers)
**Reviewer:** fresh agent, Claude Opus 5 (1M context)
**Opened:** 2026-09-09
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

Written before reading the diff. `audit.md` established that the slice did what
it said; this review asks whether what it did is sound. It does not re-run the
gate, re-walk the acceptance criteria, or re-check the surface delta — that
evidence stands.

**The invariants this review holds the code to** (`CLAUDE.md`), in the order it
attacked them:

1. **Permissive wire, canonical internals.** `envelope::normalize` is the only
   door. Is anything unvalidated past it? Is anything *ambiguous* guessed at
   rather than refused?
2. **Wire compatibility is not narrowed to fit a consumer.** The published
   contract is `draft-spec.md`, and the host is one implementation of it. Every
   place the code and §6.2/§6.3/§6.4 could disagree, checked in both
   directions — including where the code is right and the *contract about to be
   promoted to canon* is wrong.
3. **The host does not understand the domain.** No branch on `kind`, no read
   into `data`, no comparison of `timestamp` to now.
4. **A failure never takes the host down, and every refusal says which side was
   wrong.** Partial reads, timeouts, dropped connections, a writer that never
   writes, concurrent connections, a `oneshot` whose receiver is gone, and the
   accept task's own death.
5. **Strata run one way.** Stratum 1 unchanged but for one visibility widening.

**Where the bodies were expected.** The seams: the listener/loop split (two
authors of one reason vocabulary), the reply's bytes (the one thing a second
implementation would be held to), the accept task's error path (no test drives
it), and anything the *diagnostics surface* is the only report of — the places
where the spec makes a promise that only prose keeps.

**Two leads inherited undispositioned, both re-derived from the code rather
than accepted:** the `ingress_stopped()` reason literal (F-4 below, **confirmed
as stated**), and the `accept_loop` retry (F-2, **confirmed, and it is worse
than the audit's framing**).

**Round 1** — 2026-09-09 — the whole code diff: `crates/goad-shell/src/ingress/`
(both files), `crates/goad/src/controller.rs`'s new arms and helpers,
`config.rs`, `startup.rs`, `main.rs`, `diagnostics.rs`, both ingress test
modules, and the demo's config and example backend. Read against
`draft-spec.md` R-1..R-16 and §5/§6 clause by clause, `design.md` §5.1-§5.5,
and SPEC-001/R-7, R-9, R-45..R-47, R-56.

**Round 2** — 2026-09-09 — the repairs, `93abab3..441fa94`. Every one of the
thirteen repairs read against the finding it answers and against the condition
its outcome attached; the gate re-run here rather than cited (`just check`,
**exit 0**, 19 `test result: ok` blocks, 0 failures, all ten new cases present
and green); each new case checked for whether it would have gone red before its
own repair; and the repaired surfaces attacked afresh for defects the repairs
introduced. Two of the three findings below were confirmed by running the code,
not by reading it.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | verified |
| F-2 | major | fix-now | verified |
| F-3 | major | fix-now | verified |
| F-4 | minor | fix-now | verified |
| F-5 | minor | fix-now | verified |
| F-6 | minor | doc-wrong | verified |
| F-7 | minor | fix-now | verified |
| F-8 | minor | fix-now | verified |
| F-9 | minor | follow-up | verified |
| F-10 | minor | doc-wrong | verified |
| F-11 | minor | doc-wrong | verified |
| F-12 | minor | fix-now | verified |
| F-13 | nit | fix-now | verified |
| F-14 | minor | fix-now | verified |
| F-15 | minor | doc-wrong | verified |
| F-16 | minor | fix-now | verified |
| F-17 | minor | fix-now | verified |
| F-18 | major | fix-now | verified |
| F-19 | minor | | |
| F-20 | minor | | |
| F-21 | minor | | |

Disposition column transcribed by the raiser from each finding's own
**Disposition** line; the responder wrote those, this table only summarises
them. The outcome column is the raiser's.

### F-1 — the reply is not newline-terminated, and §6.3 says it is

**Severity:** major
**Location:** `crates/goad-shell/src/ingress/mod.rs:419-444` (`reply`),
`:496` (`write_all`); `draft-spec.md` §6.3

**Expected:** `draft-spec.md` §6.3 opens the reply's normative description with
*"One JSON object, **newline-terminated**, then the host closes."* That sentence
is the wire form a second implementation would be held to, and the framing a
watcher author writing to the published contract in any language would code
against.

**Observed:** the host writes no newline. `reply()` returns
`serde_json::to_string(&wire)` and `handle` writes exactly those bytes, then
shuts down the write side. The module's own doc comment states the choice
outright: *"No trailing newline: the connection's close is the line's
terminator."* `notes.md:657` and `:703` record it as a deliberate PHASE-03
decision, matched to the A-1 probe's harvested byte count. **The spec was never
brought into line with it.**

The contract and the host therefore disagree about the reply's framing, and the
document is the one about to be promoted to canon. A writer that reads until
`\n` — the obvious reading of "newline-terminated", and what `nc -U | head -1`
does — gets its line only because EOF happens to terminate a read too; a client
that requires the terminator, or a second host implementation that emits it, is
now on the wrong side of whichever document it read.

**Evidence:** `draft-spec.md:218`; `crates/goad-shell/src/ingress/mod.rs:419-421`
(the doc comment), `:443` (`to_string`, no terminator appended), `:496`. **No
test discriminates the two**: every reader in both tiers is
`read_to_string`-to-EOF (`crates/goad-shell/tests/integration/ingress.rs:130-137`,
`crates/goad/tests/renderer/ingress.rs:132-135`) and every assertion goes
through `serde_json::from_str`, which accepts the JSON with or without a
trailing newline. `design.md:213` says only "One line, then the host closes",
which is compatible with either — so the disagreement is spec-versus-code, with
nothing between them holding it.

**Disposition:** fix-now
**Response:** The code is wrong; §6.3 stands. Appending the terminator is one
byte and strictly widens compatibility — a reader that stops at `\n` and a
reader that reads to EOF both work against a host that emits it, while only the
second works today. PHASE-03's rationale is overturned knowingly: "the
connection's close is the line's terminator" is a true statement about this
host, not a reason for the contract to promise a byte it does not send, and the
invariant this slice exists under is that a renderer's or a host's convenience
does not narrow the wire. Repair adds an assertion that discriminates the two —
the existing readers cannot, by construction. **User's call, 2026-09-09.**

**Repaired, 2026-09-09.** `reply()`
(`crates/goad-shell/src/ingress/mod.rs`) appends `\n`; its doc comment states
the reason and cites this finding. Two instruments, and the outcome's note is
taken — neither goes through `parsed`:
`ingress::every_reply_is_newline_terminated_before_the_close`
(`crates/goad-shell/tests/integration/ingress.rs`) asserts `reply.ends_with('\n')`
on the raw string `read_reply` already returns, over **both** replies the host
can write, accepted and refused; and `tests::a_reply_is_one_newline_terminated_line`
in the module itself, which adds that there is exactly *one* newline. Confirmed
red before the change (`left: Some(125), right: Some(10)`, on a first draft that
read the line) and green after.

**Outcome:** verified. One note for the repair, not a condition: the
discriminating assertion has to read the raw bytes, not `parsed(&reply)` —
`serde_json::from_str` accepts the document with or without the terminator, so
an assertion routed through it would be the same blind instrument in a new
place. `reply.ends_with('\n')` on the string `read_reply` already returns is
enough.

### F-2 — `accept_loop` spins without bound or backoff on a persistent `accept()` error

**Severity:** major
**Location:** `crates/goad-shell/src/ingress/mod.rs:452-461`

**Expected:** every other loop in this workspace that could be driven by
something outside the host's control is bounded. `serve`'s scheduled arm re-arms
at the floor after a refusal precisely so a failing firing cannot spin
(`controller.rs:636` and `:673-677`, `refusal_re_arms`); `Ingress::arrival` drops the
receiver as it yields `None` so a closed channel parks instead of spinning
(`ingress/mod.rs:220-229`), and `PHASE-04/VT-7` exists to assert exactly that.
`drain` is bounded in bytes with the reason written down. The accept path is the
one loop with no bound at all.

**Observed:**

```rust
loop {
  let Ok((stream, _addr)) = listener.accept().await else {
    continue;
  };
  ...
}
```

Every `accept()` error is discarded, unlogged and uncounted, and the loop
retries immediately. `accept(2)` errors that persist rather than clearing on the
next call are ordinary: `EMFILE`/`ENFILE` (the process or the system is out of
descriptors — reachable here, since the host spawns a subprocess with three
pipes per exchange *and* holds a descriptor per connection), `ENOBUFS`/`ENOMEM`,
and `EBADF`/`EINVAL` if the listener is ever invalidated. Under any of them the
task busies a tokio worker at 100% until the condition clears by itself, and
`EMFILE` is a condition a spin actively prolongs.

The audit's framing — *"it is on a spawned task, so it costs no presentation"* —
is correct and incomplete. The runtime is `new_multi_thread`
(`crates/goad/src/main.rs:59`), so the Slint thread is not blocked and no
presentation is charged; what is charged is one worker pegged for the life of
the fault, on a desktop application that is expected to sit idle. Nothing
reports it, so the first symptom a person gets is a hot machine.

**Evidence:** `ingress/mod.rs:452-461`; the docstring at `:449-451` states *"An
`accept()` error does not end the task"* and says nothing about how often it
retries. `crates/goad/src/main.rs:59` for the runtime kind. **No test drives an
`accept()` error at all** — grep both ingress test modules: every case exercises
a connection that was accepted. The nearest neighbour,
`a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
(`renderer/ingress.rs:664`), asserts the no-spin property on the *receiver* side
and has no analogue on the accept side.

**Disposition:** fix-now
**Response:** Confirmed, and the reviewer's account of the cost is the one that
governs: a pegged worker on an idle desktop app, unreported. Repair is bounded
backoff between retries and, after a bound of consecutive failures, ending the
task through the **existing** ingress-stopped path — which already reports and
already parks the arm. No errno taxonomy: a persistent `accept()` fault *is*
ingress being unavailable, and the host already has a word for that. Reuses
machinery rather than adding a surface. If an `accept()` error cannot be driven
from a test without contrivance, say so in the ledger rather than asserting an
instrument that does not exist. **User's call, 2026-09-09.**

**Repaired, 2026-09-09**, with the outcome's constraint taken as written.
`accept_loop` (`crates/goad-shell/src/ingress/mod.rs`) records when the current
run of failures began and retries until it has lasted `ACCEPT_FAULT_BUDGET` —
**five seconds**, stated in elapsed time and named in the docstring beside
`ENVELOPE_LIMIT`'s and `ENVELOPE_DEADLINE`'s, because the bound is *how long a
transient fault has to clear before ingress is declared dead*. The next
connection that arrives forgets the fault. `accept_backoff` waits as long as
the fault has already lasted, clamped to 5 ms and 500 ms: sleeping the elapsed
time is what doubles it, so the schedule is 5, 10, 20 … 500 ms and the whole
budget costs a bounded handful of syscalls. Spending it `break`s the loop,
dropping the sender — the **existing** ingress-stopped path, already reported
and already parked. No errno taxonomy, no new surface.

**The honest gap, as asked.** No case drives a real `accept()` error, and none
is added. `EMFILE`/`ENFILE` are process-wide and `cargo test` runs cases in
parallel in one process — `SOCKET_MODE`'s own doc comment makes this argument
about `umask(2)` — so provoking one is either contrivance or collateral damage
to unrelated cases. What is tested is the budget, as the pure function that
decides it: `tests::a_retry_waits_as_long_as_the_fault_has_lasted_between_the_two_bounds`
and `tests::a_fault_that_outlasts_the_budget_ends_the_task_rather_than_spinning`
(`crates/goad-shell/src/ingress/mod.rs`, a new `#[cfg(test)] mod tests`). The
seam between the spent budget and the report is held by **review**, not by a
test; the far side of it — a dropped sender reaching a person — is driven end
to end by `a_dead_accept_task_is_folded_once_…` and now also by
`ingress_stopping_during_an_exchange_…` ([[F-3]]).

**Outcome:** verified, with one constraint the repair must answer explicitly.

Ending the task on a bound of consecutive failures **trades a recoverable fault
for an unrecoverable one**: today an `EMFILE` burst pegs a core and then
recovers when descriptors free; after this repair it parks ingress for the life
of the process. That trade is right — the spec already admits ingress ending
and reports it (§5), and a reported permanent stop beats an unreported hot
machine — but it means the bound is *how long the host tolerates a transient
fault before declaring ingress dead*, and a bare count does not express that.
With backoff, N consecutive failures is anywhere from milliseconds to minutes
depending on the schedule. **State the bound in elapsed time, and say the number
in the docstring**, the way `ENVELOPE_LIMIT` and `ENVELOPE_DEADLINE` already
state theirs.

On the instrument: an `accept()` error genuinely cannot be provoked here without
contrivance — `EMFILE` needs a process-global `setrlimit` and cases run in
parallel in one process, the same reason `SOCKET_MODE`'s own docstring gives for
not setting a umask. The available honest instrument is to lift the retry
decision into a **pure function** — consecutive failures and elapsed, to
retry-after or give-up — and unit-test that beside `spacing_elapsed` and
`deadline_after`, which are in that module for exactly this reason. The
observable half is already covered:
`a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
holds what happens once the task ends, whatever ended it.

### F-3 — the ingress-stopped `unavailable` is discarded, not reported, when ingress dies during an exchange

**Severity:** major
**Location:** `crates/goad/src/controller.rs:722` (the inner arm),
`:446-455` (`ingress_stopped`); `draft-spec.md` R-15, §5

**Expected:** `draft-spec.md` R-15 closes with *"The one refusal that reaches no
writer is the one that answers no envelope — the ingress-stopped `unavailable` of
§6.3, **for which this surface is the only report there is**."* §5 states it as
behaviour: *"The host reports that once, as `unavailable`, on the surface R-15
names — the only place it can, since no envelope reaches it afterwards to be
refused."* Neither sentence is conditional on what the host happened to be doing
at the moment ingress died.

**Observed:** the inner (during-exchange) arm folds it onto the controller and
nothing ever presents it:

```rust
arrival = ingress.arrival() => match arrival {
  None => controller.refuse(&ingress_stopped()),
  Some(arrival) => refuse_during_exchange(&mut controller, arrival),
},
```

The inner loop calls no `glass.present`, and its only non-cancelling exit is the
`outcome` arm, which calls `controller.absorb(..)` — and `absorb` assigns
`self.diagnostics = diagnostics` wholesale (`controller.rs:178`). So when the
accept task dies while an exchange is in flight, the fold is **guaranteed** to be
overwritten before any presentation. Not a race: there is no path from that arm
to a frame. The permanent, unrecoverable condition R-15 says the surface is the
only report of is reported nowhere at all, and `Ingress::arrival` has already
parked the arm, so it never recurs.

The design already knew: `design.md:229`'s reply table qualifies this row *"the
ingress-stopped case, **when the loop was idle**"*. The draft spec does not carry
that qualification. One of the two is wrong, and the one being promoted to canon
is the unqualified one.

**Evidence:** `controller.rs:720-724` (the inner arm; no `present`, no `break`),
`:710-717` (the `outcome` arm, `absorb` then `break`), `:178` (`absorb` replaces
`self.diagnostics`); `draft-spec.md:107` (R-15) and `:157-161` (§5);
`design.md:229`. The negative is already proven for the sibling case by a test:
`a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`
(`renderer/ingress.rs:1131`) asserts that a refusal folded in this same arm is
gone by the time `serve` returns. **No case drives the `None` arm during an
exchange** — `a_dead_accept_task_is_folded_once_…` (`:664`) kills the accept task
before `serve` has anything in flight, so it only ever exercises the outer arm at
`:626-629`.

**Disposition:** fix-now
**Response:** The spec is right and the code is wrong. A permanent,
unrecoverable condition reported *nowhere* is what "every refusal is reported"
forbids, and the arm is already parked so there is no later chance. Repair makes
the ingress-stopped `unavailable` survive the in-flight exchange — present it
before `absorb` can overwrite it — and drives the `None` arm during an exchange,
which no case does today. R-15 and §5 keep their unqualified wording; it is
`design.md:229`'s "when the loop was idle" that recorded an implementation
accident as intent, and that goes under Design drift rather than being adopted.
**User's call, 2026-09-09.**

**Repaired, 2026-09-09.** The inner arm's `None` branch calls
`controller.refuse(&ingress_stopped())` and then
`glass.present(controller.frame())`, so the report reaches a frame before
`absorb` can replace the surface (`crates/goad/src/controller.rs`). **On the
`None` branch only**, per the outcome: `Some(arrival)` still presents nothing,
so `review-design.md` F-15's measured cost and R-15's negative case are both
untouched — `a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`
and `a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`
pass unchanged. The branch is reachable at most once per process, and by no
writer.

Driven by `ingress::ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`
(`crates/goad/tests/renderer/ingress.rs`), which is the case nothing had: the
evaluate is queued on the command channel *before* `serve` starts, so the
biased outer `select!` takes it and the closed channel is first met by the
inner arm; `@slow-view`'s foreground `sleep 0.2` keeps the exchange running
while the assertion reads the **live window**, and the case also asserts the
view had not yet landed. Confirmed red without the `present` — the fold reaches
no frame and the bound times out — and green with it.

**Observation, not a finding — raised by the reviewer at round 2 and left as
one.** The case depends on the accept task's drop completing before the inner
arm first polls, and `shutdown_background` *defers* that drop, so in principle
the outer arm could observe the closed channel first. It was traced and could
not be confirmed as a defect, so it stays an observation: the race runs the
**safe** way — what precedes the inner arm's first poll is a subprocess spawn,
so load lengthens the exchange and favours the inner arm, and losing the race
fails the case rather than passing it wrongly. Measured 12/12 stable.

**Recorded in two places on purpose.** This entry is the record of *who
observed it and why it was not raised*. The durable half — why the race is safe
and where to look — is in the case's own doc comment
(`crates/goad/tests/renderer/ingress.rs`), because whoever meets a future flake
will be reading the test and not this ledger. Deliberately **not** `notes.md`
Harvest: `docs/AGENTS.md:88` makes that file disposable, so a landing spot for a
flake that has not happened yet would evaporate at close. If it ever does flake,
it joins the four pre-existing flaky tests in `slice-004.md` Follow-ups, and the
doc comment is what tells whoever gets there that it was looked at once
already.

**Outcome:** verified. Guarding one thing for the repair: present on the `None`
branch **only**, never on `Some(arrival)`. The `None` branch fires at most once
per process — `Ingress::arrival` parks the arm as it yields — so a presentation
there costs exactly one, ever, and no writer can reach it. The `Some` branch is
the one `review-design.md` F-15 measured and
`a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`
holds negative; presenting there would reopen both. The two branches sit one
line apart, which is the whole risk.

### F-4 — the ingress-stopped refusal's wire token is a string literal, and nothing reads it

**Severity:** minor
**Location:** `crates/goad/src/controller.rs:446-455`

**Expected:** the reason vocabulary is closed at eight tokens and `Refusal` owns
it. `Refusal::reason()` is written specifically so that *"the compiler — not a
reviewer — is what notices the day a ninth reason is needed"*
(`ingress/mod.rs:288-290`), and `refuse_arrival` — the only other site that puts
a reason on the diagnostics surface — reads it off the value
(`controller.rs:421`, `reason: refusal.reason().to_owned()`).

**Observed:** `ingress_stopped()` writes the token by hand:

```rust
Refused::Ingress {
  reason: "unavailable".to_owned(),
  detail: "ingress has stopped; no further events will be accepted".to_owned(),
}
```

The comment above it argues correctly that no `Refusal` value describes this
cause — `UnavailableCause` has two variants and neither is it. That justifies not
constructing a `Refusal`; it does not justify duplicating the *token*, which is
the one part of the value that is shared vocabulary rather than a description of
the cause. The token now has two authors, and the second is a literal.

**Nothing would fail if it were misspelled.** `PHASE-08/VT-8`
(`the_reason_token_set_is_closed_at_eight`, integration) reaches only
`Refusal::reason()`, and `PHASE-07/VT-7`
(`a_dead_accept_task_is_folded_once_…`, `renderer/ingress.rs:705-709`) asserts
`line.contains("ingress has stopped")` — the *detail*, never the token. Compare
`a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface`
(`:1102-1107`), which does assert `line.contains("too_soon")`: the token is
checked on the surface for every reason but this one.

Raised `minor` rather than `major` deliberately: the wire's own reason set is
untouched, and the consequence of a misspelling is a wrong token in one
person-facing line. The defect is the class — an eight-token closed set with a
second, unchecked author — not the current spelling, which is right.

**Evidence:** `controller.rs:446-455`; `controller.rs:418-424` (`refuse_arrival`,
the site that does it correctly); `ingress/mod.rs:318-335` (`reason()`);
`renderer/ingress.rs:705-709` versus `:1102-1107`. PHASE-07 reported and did not
fix it, on purpose, so this review would meet it cold — confirmed independently
here from the code, and the audit's description of it at `audit.md:264-267` is
accurate as far as it goes.

**Disposition:** fix-now
**Response:** The lead, confirmed. Read the token off `Refusal::reason()` like
`refuse_arrival` does. The current spelling is right, so this is not a live bug —
it is the class: an eight-token closed set with a second, unchecked author.
Fixing the class is one line and removes the second author entirely, which is
worth more than any assertion added around the literal. See [[F-5]] — same
closure claim, other end.

**Repaired, 2026-09-09**, in the shape the outcome prescribes rather than the
one the Response named. `UnavailableCause` gains a third variant,
`IngressStopped`, whose `Display` is the sentence the fold used to spell by
hand; nothing constructs it on the wire side, and its doc comment says so.
`ingress_stopped()` is now one line —
`folded(&Refusal::Unavailable(UnavailableCause::IngressStopped))` — where
`folded` is extracted from `refuse_arrival` and is **the only author of an
ingress diagnostics line**: token off `reason()`, prose off `Display`. No value
that misdescribes the cause is harvested for its token.

**No test is red before this**, and the finding says why: the current spelling
was right, so this is the class and not a live bug. What holds the class is the
removal of the second author (review) plus [[F-5]]'s compile gate, which
`IngressStopped` strengthens — the match there is now exhaustive over
`UnavailableCause` as well, so a fifth cause fails to compile in the test file
too. `a_dead_accept_task_is_folded_once_…` still asserts the same detail line,
unchanged, which is the regression guard on the prose.

**Outcome:** verified — the disposition. **The repair shape as written is a
trap, and I will raise it as a new finding if it lands that way.**

"Read the token off `Refusal::reason()`" cannot be done without a `Refusal`
value, and no existing one describes this cause: that is what
`ingress/mod.rs:268-272` says and it is correct. Harvesting
`Refusal::Unavailable(UnavailableCause::Stopping).reason()` for its token would
be **worse than the literal** — a value meaning *the host is stopping* standing
in for *ingress has ended permanently*, whose `Display` ("no answer was given
for this envelope") is wrong about the situation and is one refactor away from
being read.

The shape that fixes the class without that: **a third `UnavailableCause`
variant**. `draft-spec.md:243-251` is explicit that `unavailable` covers three
causes and that the third "is not a ninth token" — so a third variant is *more*
faithful to the contract than two, `Display` gets the right sentence, and
`ingress_stopped()` collapses into the ordinary `refuse_arrival` shape.
`mod.rs:268-272`'s argument against it is about the **wire**, and this value
never reaches the wire; it reaches the diagnostics surface, which is where
`refuse_arrival` sends `reason()` too. It also makes [[F-5]]'s exhaustive match
stronger by one arm. A named constant both sites read is the weaker fallback if
the variant is refused.

### F-5 — `the_reason_token_set_is_closed_at_eight` does not close the set against an added reason

**Severity:** minor
**Location:** `crates/goad-shell/tests/integration/ingress.rs:720-761`

**Expected:** `draft-spec.md` R-14's Verification row claims the test holds *"the
**exact token set**, so a reason added or renamed fails here rather than at a
client"*, and `audit.md:111` repeats it: *"so a token added, removed or renamed
fails there."*

**Observed:** the test builds its input from a **hand-written array of eight
`Refusal` values** and compares `reason()` over that array against a
hand-written array of eight strings. A ninth `Refusal` variant that is not added
to the array leaves both sets unchanged and the test green. What the test
actually holds is: (a) a *renamed* token, (b) a token *removed* from `reason()`'s
match, and (c) the eight-way mapping being correct. It does not hold (d), a
token *added* — which is the direction the requirement is about, since the
closure claim is what a client's parser depends on.

The compiler does force a change to `reason()` when a variant is added
(exhaustive match, no `_` arm) — that is real and worth keeping — but it forces
an edit, not a *review of the wire contract*, and the test is what R-14 names as
the instrument for the latter.

**Evidence:** `crates/goad-shell/tests/integration/ingress.rs:722-737` (the
literal array of eight constructions), `:739-748` (the literal expected set);
`draft-spec.md:318` (R-14's verification claim); `audit.md:111` (AC-4's, the same
claim). The `#[test]` is a plain unit test with no listener, so nothing else in
the case reaches `Refusal`'s variant list either.

**Disposition:** fix-now
**Response:** The test holds three of the four things R-14 claims for it, and
the missing one — a token *added* — is the direction a client's parser depends
on. The compiler does force an edit to `reason()`, but an edit is not a review of
the wire contract. Repair: put an exhaustive `match` over a `Refusal` value in
the test itself, all eight arms named with no `_`, so a ninth variant fails to
compile *in the test file*. That is a real instrument, costs nothing, and makes
R-14's sentence true as written rather than weakening it. Fix the class with
F-4.

**Repaired, 2026-09-09.** `the_reason_token_set_is_closed_at_eight`
(`crates/goad-shell/tests/integration/ingress.rs`) now carries its own `token`
function: an exhaustive `match` with **no `_` arm at either level** — over
`Refusal`'s variants and over `UnavailableCause`'s — and it is the **source of
the compared set**, per the outcome's condition: every member of `reasons` is
returned by one of its arms. Beside it, `witnesses()` is length-linked to the
eight-string `EXPECTED` literal, and each witness's `Refusal::reason()` is
asserted equal to the match's own token, so a rename in production alone fails
here.

**Measured, not asserted.** A ninth `Refusal` variant was added temporarily and
three outcomes recorded: (1) the test file **fails to compile** —
`error[E0004]: non-exhaustive patterns: &Refusal::Ninth not covered` — which is
the direction R-14 is about, and it fires *here* rather than at a client;
(2) adding only the arm leaves the case green; (3) adding the arm **and** a
witness fails, first on the length link and then on the set. The variant was
then reverted.

**Its boundary, stated in the case's own doc comment rather than claimed away.**
(2) is a real hole: Rust cannot force the witness list to cover a newly added
variant without a derive macro or an enumeration crate, and both are dependency
additions — a STOP condition, so neither was taken. The *compile* gate is
forced; the *assertion* gate depends on the author adding a witness one line
from the arm the compiler has just made them write. R-14's sentence — *"a reason
added or renamed fails here rather than at a client"* — is true as written,
because a compile failure in this file is a failure here. If that is judged
insufficient, this should return `contested` rather than the case claiming more
than it holds.

**Against the condition, said plainly.** The condition distinguishes a match
that is the *source of the compared set* from one that is a *ward beside a
hand-written array*. This case is the former by construction — every member of
`reasons` is returned by an arm, and there is no second path into the set — but
it still shows the ward's symptom, result (2) above, because the arms are
reached through witnesses and Rust cannot force a witness to exist. Those are
the same observable, and the case says so rather than resting on the
distinction.

**The shape that would close it is outside this finding's declared location, so
it is offered rather than taken.** Make `Refusal::reason()` return a closed
`Reason` *type* instead of a `&'static str`. A ninth `Refusal` variant then
cannot mint a token at all — its forced arm must name an existing `Reason` —
and a genuinely new token is a new variant of a type whose only purpose is the
wire vocabulary, which is a much smaller surface to hold. It is a public API
change reaching `controller.rs` and both test tiers, well past
`tests/integration/ingress.rs:720-761`, so it is a decision rather than a
repair. **This is the one thing the repair agent asks be decided.**

**Which set this case closes.** The eight **wire tokens**. `UnavailableCause`'s
four causes share one token, so the `unavailable` arm's or-pattern grows while
`EXPECTED` does not — §6.3's own arrangement, *"a fourth cause of one of them,
not a ninth token"*. R-14's sentence is not bent to fit: it is about tokens, and
tokens are what is compared. The exhaustive cause pattern buys only that a fifth
cause must be looked at here, where the question of whether it earns a token
belongs. The case's doc comment says this.

**Decided by the user, 2026-09-09: weaken R-14; the `Reason` type is a
follow-up.** The argument two paragraphs up — that a compile failure in this
file is a failure *here* — is **kept, and made explicit**, not retracted:
result (1) stands, and adding a reason does force a stop in this file. The leak
is narrower than *R-14 is false*. What the compile gate does not force is a
**correct** update: result (2) lets an author satisfy the compiler and walk away
green. So the amendment names which of the four directions is held by an
**assertion** and which by the **compile gate plus review**, and the argument
becomes the sentence's second half rather than an unstated reliance inside its
first. A second implementation then knows which guarantee it is leaning on,
which is more information than either the old sentence or a retraction. What
landed:

- **`draft-spec.md` R-14's Verification row** now claims a token *renamed*, a
  token *removed*, and the eight-way mapping by assertion, and states in terms
  that the *added* direction is a compile gate plus review — naming why Rust
  cannot force the witness list without a dependency, and telling a second
  implementation to read it that way.
- **Three documents that repeated the stronger claim are corrected in place,
  each saying it was corrected and citing this finding**: `audit.md`'s AC-4 row
  (*"added, removed or renamed"* — the reviewer's own evidence for this
  finding), `plan.md`'s PHASE-08/VT-8 entry, which said the same thing before
  the code existed, and `notes.md`'s PHASE-07 finding, which asserted VT-8
  *"correctly and completely closes `Refusal`'s own eight-token set"*. The
  audit does not keep a claim the spec has given up, and neither does the plan.
  `design.md` is untouched: it says only *"one test asserts the exact token
  set"*, which is true, and it is a record of intent (`docs/AGENTS.md`).
- **The case's doc comment** now leads with which half holds which direction and
  carries the three measured results. That is the durable part.
- **The `Reason` type is not implemented.** It is deferred as a design question
  outside this finding's declared location — the ground [[F-9]] was deferred
  on, and explicitly not a size argument — and lands in `slice-004.md`
  Follow-ups as one entry with [[F-9]] and [[F-10]]'s design half, since the
  three share a cause: what the host's surfaces and vocabularies owe now that a
  process outside the host can reach them. F-9's second-host-startup instance
  is named there.

**Outcome:** verified — **conditionally**, and the condition is the whole
answer to the question asked.

The shape holds **only if the match is the source of the compared set**. Each
arm must yield the value (or token) that is collected and compared against the
literal eight-string expectation. Then a ninth variant fails to compile; the
repairer adds an arm; the collected set becomes nine; the assertion fails
against the eight-string literal. Both gates fire, and R-14's sentence is true
as written.

It does **not** hold if the match is an exhaustiveness *ward* standing beside
the existing hand-written array. Then a ninth variant forces an arm onto the
ward, the array stays at eight, and the test goes green — an instrument that
looks like it closes the set and does not, which is worse than today's, because
today's at least does not claim to. That is the failure this finding is about,
reproduced one layer up.

So: build it as the source, not as a ward. If it lands as a ward I will contest
at re-review rather than weaken R-14. What the repair still cannot hold, and
nobody should claim it does: a new *field* on an existing variant. R-14 does not
ask for that.

### F-6 — R-11's "byte-for-byte" is false of `data`, and the test cannot see it

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/envelope.rs:140-142`;
`draft-spec.md` R-11; `crates/goad/tests/renderer/ingress.rs:271-275`

**Expected:** R-11 — *"All four fields reach the backend as the event of an
`evaluate` (SPEC-001/R-7): `source`, `kind` and `data` **byte-for-byte as
sent**."*

**Observed:** `data` is parsed into a `serde_json::Value` and re-serialized on
the way out. Two byte-level changes survive that round trip:

- **Object key order is normalized.** `serde_json` is on the manifest without
  the `preserve_order` feature (`Cargo.lock:4605-4616` lists `itoa`, `memchr`,
  `serde`, `serde_core`, `zmij` — no `indexmap`), so `serde_json::Map` is a
  `BTreeMap` and keys reach the backend sorted, not as written. `{"b":1,"a":2}`
  in, `{"a":2,"b":1}` out.
- **Numbers outside `f64`'s exact range lose precision.** Without
  `arbitrary_precision`, an integer literal larger than `u64::MAX` is parsed to
  `f64` and re-emitted in exponential form.

Neither breaks SPEC-001/R-9, which says *"carries both verbatim"* and is about
the host not *interpreting* the payload — that invariant holds, and this module
genuinely reads nothing (`envelope.rs:140-142` is a `remove` and nothing else).
The defect is R-11's stronger sentence, which is newly written by this slice and
is about to be promoted as canon, on the strength of an implementation that
cannot keep it.

**Evidence:** `envelope.rs:140-142`; `canonical.rs:490-496` (`data:
serde_json::Value`); `Cargo.lock:4605-4616`. **The test is insensitive to it by
construction**: `renderer/ingress.rs:271-275` asserts
`request["event"]["data"] == serde_json::json!({"count_last_hour": 4})`, an
equality between two already-parsed `Value`s, which compares maps as maps —
order-blind, and it would pass under any of the transformations above. The
integration tier's `a_well_formed_envelope_reaches_the_judge_as_the_event_it_wrote`
(`integration/ingress.rs:571`) asserts the same way, one key deep.

**Disposition:** doc-wrong
**Response:** The document is the defect. R-11's "byte-for-byte" is newly
written by this slice and was never designed for: `data` round-trips through
`serde_json::Value`, so key order normalizes and precision past `f64` is lost.
Restate R-11 in SPEC-001/R-9's terms — the host does not *interpret* `data` and
forwards it whole — which is the invariant it was reaching for and which the
code does hold; `envelope.rs:140-142` reads nothing. The two alternatives were
weighed and refused: `preserve_order`/`arbitrary_precision` is a serde_json
feature change **shared with stratum 1**, exactly POL-001's residue, for a
property nothing needs; carrying the raw slice reopens the canonical type's
normalization door at the end of a slice. **User's call, 2026-09-09.**

**Repaired, 2026-09-09**, on the model the outcome names and without reaching
for "verbatim". R-11 now reads: `source` and `kind` as the strings sent, `data`
as the **value** sent — carried whole and read into nowhere — and `timestamp`
as the instant sent; then, in §6.2's own voice, *"the value is preserved and its
spelling is not: `data` round-trips through the host's JSON parser, so a key
order or a numeric precision the host cannot represent is not a promise this
requirement makes."* §6.2's `data` row carries the concrete half where a reader
looks it up — key order normalized, numbers past an IEEE 754 double losing
precision — beside the `timestamp` row that already says the same thing about
spelling. §7's R-11 row no longer claims byte-for-byte either: it says `data` is
compared as a **value**, which is what the existing assertion actually does.
Documentation only; no code changed.

**Outcome:** verified, and the reason for refusing the feature route is the
stronger of the two arguments — Cargo unifies features across the graph, so
`preserve_order` would reach stratum 1 whoever declared it.

The restatement has a ready-made model in the spec's own voice, and should use
it rather than inventing one: §6.2's `timestamp` row already says *"the instant
is preserved; its **spelling** is not."* R-11's `data` clause wants the same
sentence — the **value** is preserved and forwarded whole, the spelling is not —
which is true, is checkable, and does not reproduce the overclaim by swapping
one absolute word for another. Avoid landing on "verbatim" as the replacement:
it is SPEC-001/R-9's word for *not interpreted*, and reusing it here as if it
were a claim about bytes is how this finding happened.

### F-7 — a read error on the connection is reported to the writer, and to a person, as `malformed`

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:539`

**Expected:** `CLAUDE.md` — *"Every refusal is reported and says which side was
wrong."* `malformed` means, per §6.3's own table, *"the bytes were not one JSON
document"*, and its stated *writer's fix* is *"the serializer"*.

**Observed:**

```rust
Ok(Err(_io)) => Err(Refusal::Malformed),
```

An `io::Error` from `read_until` — a connection reset mid-envelope, `EIO`, any
transport fault — is folded into the one reason that names the writer's
serializer as the thing at fault. The writer is told it sent bad bytes when it
sent none the host could read; more to the point, `refuse_arrival` puts the same
verdict on the diagnostics surface, where a person reads *"an event was refused
(malformed): the bytes are not one JSON document"* about a connection that
failed underneath. `unavailable` is the reason set's own token for *the host
cannot act on this envelope*, and it is the honest one; the error is discarded
rather than carried into `detail`.

**Evidence:** `ingress/mod.rs:534-540`; `draft-spec.md:231-232` (the
`malformed` row and its writer's fix); `controller.rs:418-424` (`refuse_arrival`
folds the same refusal onto the surface). Reachability is narrow — a peer that
sent an RST usually cannot read the reply either — but the diagnostics half is
not narrow: the fold happens whether or not the reply lands. No test drives an
I/O error on the read.

**Disposition:** fix-now
**Response:** Correct: `malformed` names the writer's serializer, and a
transport fault is not that. `unavailable` is the reason set's own word for *the
host cannot act on this envelope* and is the honest verdict. Repair maps the
read `io::Error` to `Unavailable` and carries the error into `detail` rather
than discarding it. The reachability of the *reply* half is narrow; the
diagnostics half is not, and a person reading "the bytes are not one JSON
document" about a reset connection is being told the wrong side was wrong —
which is precisely the invariant.

**Repaired, 2026-09-09**, with the coupled §6.3 edit the outcome requires — so
F-4, F-7 and §6.3 landed as one piece and agree on how many causes there are.
`read_envelope`'s `Ok(Err(io))` arm now returns `unreadable(io)`, a named
function stating the rule and carrying the error into `detail` through a fourth
`UnavailableCause` variant, `Unreadable(io::Error)`, which also becomes the
refusal's `source()`. §6.3's `unavailable` row and its causes paragraph now read
**four** causes and say in terms why a transport fault is not `malformed`; its
*which refusals a person sees* paragraph adds the faulted connection to the list
that always reaches the surface. The reason set is untouched at eight.

**Test, and its boundary.** `tests::a_connection_that_faults_mid_read_is_unavailable_and_carries_the_error`
(`crates/goad-shell/src/ingress/mod.rs`) holds the rule at the one site that
states it. It is a unit case, honestly so: a peer that closes a Unix stream
socket gives the host EOF, not an error, so no writer a test can build makes
`read_until` fail — the error kinds that would (`ECONNRESET`, `EIO`, `EBADF`)
are not reachable over AF_UNIX from a cooperating test without `libc`, which is
a dependency addition. `unreadable` exists as a function so the rule is
reachable at all; that `read_envelope` calls it is held by review.

**Outcome:** verified, with a coupled spec edit the disposition does not name.

`unavailable` is the right token, but §6.3's own row defines it by an
enumeration — *"the host cannot act on any envelope — it is stopping, its clock
is unreadable, or its ingress has stopped"* — and a transport fault on one
connection is none of the three. Mapping the read error there without touching
§6.3 puts a cause on the wire the contract does not describe, which is a smaller
copy of [[F-1]]. So the repair carries a §6.3 amendment, and it needs a
`UnavailableCause` variant to hold the detail — the same enum [[F-4]] is
already opening. F-4, F-7 and the §6.3 edit are one piece of work; done
separately they will disagree about how many causes there are.

### F-8 — a connection accepted after the judge is gone closes with no reply, while the process is still running

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:487-489`

**Expected:** R-8 — *"Every envelope MUST receive exactly one reply on the same
connection… The only case in which a connection may close unanswered is one in
which **the host process itself is gone**."*

**Observed:** when `arrivals.send(arrival)` fails — the receiver has been
dropped — `handle` returns `false` immediately. The `Arrival` comes back inside
the `SendError` and is dropped with it, taking the `Answer` and its `oneshot`
sender; nothing is written to `stream`, which is then dropped and closed. The
writer gets EOF with no reply.

The process is not gone at that moment. `main.rs:110-127` binds `_served` — and
with it the `Ingress` receiver — inside the `spawn_local` future, so the
receiver is dropped when that block ends, and only *then* does the Slint loop
unwind, `start` return, the runtime guard drop, and the runtime (with the accept
task) drop. The accept task keeps accepting for the whole of that unwind.

The window is short — milliseconds — and this is the same shape as the defined
`Unavailable(Stopping)` path, which is what the *dropped-`Answer`* case already
does correctly (`:490-495`). The difference is only which of two adjacent
failures happened first, and one of them is answered and the other is not.
`audit.md:110` asserts the exception is admitted because *"the judge is gone,
which is the host process going away"*; that is an assumption about timing, not
a property the code holds.

**Evidence:** `ingress/mod.rs:482-495` (the `send` failure returns before any
write; the `rx.await` failure two lines below writes `Unavailable(Stopping)`);
`crates/goad/src/main.rs:110-131`. No test covers a connection arriving in that
window; `a_dropped_answer_yields_unavailable_then_a_close`
(`integration/ingress.rs:424-435`) covers only the second, answered case.

**Disposition:** fix-now
**Response:** R-8 admits exactly one unanswered close — the host process being
gone — and `main.rs:110-131` shows the receiver drops well before that, so the
exception is being claimed on a timing assumption rather than a property.
`audit.md:110`'s justification is wrong on this point and the audit is amended
with it. Repair: on `arrivals.send` failure, write `Unavailable(Stopping)` and
close, exactly as the adjacent dropped-`Answer` case at `:490-495` already
does. Two adjacent failures of the same shape should not differ in whether the
writer gets an answer.

**Repaired, 2026-09-09.** On `arrivals.send` failure `handle` writes
`Unavailable(Stopping)` and closes, exactly as the adjacent dropped-`Answer`
path does — and the `Answer` is not recovered from the `SendError`, per the
outcome. The two lines are shared rather than duplicated: `stopping()` builds
the reply both moments send, and `respond()` is the write-then-shutdown both
paths end with (`crates/goad-shell/src/ingress/mod.rs`). Held by
`ingress::a_connection_accepted_after_the_judge_is_gone_is_answered_unavailable`
(`crates/goad-shell/tests/integration/ingress.rs`), which drops the `Ingress`
outright — the state `main.rs`'s `spawn_local` block leaves behind while the
accept task keeps accepting — and then sends one envelope. Confirmed red before
the change (the connection closed with no bytes at all) and green after.

**Outcome:** verified. The `Answer` need not be recovered from the
`SendError` — the adjacent path at `:490-495` does not use one either; it calls
`reply(false, Some(&Refusal::Unavailable(UnavailableCause::Stopping)))` and
writes the bytes. The same two lines, before `return false`.

### F-9 — any ingress refusal replaces the whole diagnostics surface, so an outside writer can erase the host's own fault reports

**Severity:** minor
**Location:** `crates/goad/src/controller.rs:418-424` (`refuse_arrival`),
`:188-190` (`Controller::refuse`), `crates/goad/src/diagnostics.rs:146-166`

**Expected:** the diagnostics surface is *"everything a person reads"* and is
where a backend failure, a cleanup failure and every discarded field are
reported. Slice 004 adds an input to it that is reached from **outside the
process**, at a rate the host does not bound.

**Observed:** `Diagnostics::refused` builds `lines: vec![one line]` and
`Controller::refuse` assigns it over the whole retained value. Every ingress
refusal the loop decides while idle therefore wipes whatever was on the surface.
That was already true of a superseded click, but a click is the person's own act;
an envelope is not. The measured cost is one full replacement per refusal at
~1690/s (`audit.md:112`, PHASE-07's own measurement), so any local process that
can open the socket can hold the surface at *"an event was refused (too_soon)"*
indefinitely, and a backend failure a person needs to see is gone within a
millisecond of appearing.

A concrete, non-adversarial instance already exists in the design: a second
host's startup probes the path with a bare `connect`, which the live host reads
as an empty envelope and refuses `malformed`
(`ingress/mod.rs:152-158`). So a failed `goad` start on a machine that is already
running one wipes the running host's surface and leaves a spurious *"an event
was refused (malformed)"* on it — a message about the operator's own second
process, phrased as if a watcher sent bad bytes.

The repair is probably design-level (the surface holds one thing; ingress now
has an unbounded, external author of it), which is why this is raised rather
than costed here.

**Evidence:** `diagnostics.rs:146-166` (`lines: vec![…]`, replacing);
`controller.rs:188-190`; `ingress/mod.rs:147-158` (the reclaim probe's stated
side effect); `audit.md:112` for the rate. `review-design.md` F-15 settled the
*presentation cost per refusal*; it did not consider what the surface's single
slot is worth when an outside party writes to it.

**Disposition:** follow-up
**Response:** Real, and the root it shares with F-3 is the reason F-3 is
being fixed rather than documented. But the repair here is a *surface* design —
the diagnostics slot now has an unbounded author outside the process, and
deciding what it holds (retention, precedence between host-authored faults and
externally-triggered refusals) is a design question this slice never opened.
Taking it inside a closing slice would be improvising past a decision that was
not this slice's. **Not deferred for size** — deferred because it is a design
question, which is the admitted ground. Lands in `slice-004.md` Follow-ups with
the second-host-startup instance named, since that one is non-adversarial and
will be met by an operator before any attacker. **User's call, 2026-09-09.**

**Outcome:** verified — **not contested**, and the ground given is the right
one rather than a size argument dressed up.

Checked against the guardrail directly. Every candidate repair — append rather
than replace, rank host-authored faults above externally-triggered ones,
rate-limit the ingress author — changes what `Diagnostics` means for the three
refusal paths that **predate this slice** (`SupersededView`, `UnknownOption`,
`NoClock`). It is not containable inside the ingress arm, so taking it here
would be settling a surface-wide question inside a closing slice, which
`docs/AGENTS.md` §Execute names as the thing to stop and ask about.

Fixing [[F-3]] does not make this worse or easier: F-3's repair is a
single-shot carve-out for an event that happens at most once per process, and it
establishes no precedence policy.

Two conditions on the outcome, both checkable at re-review: it lands in
`slice-004.md` Follow-ups **with the second-host-startup instance named**, since
that is the non-adversarial one an operator meets first; and it is reconciled
with [[F-10]]'s own deferred design half, which routes to Follow-ups "beside
F-9" — one entry naming both questions, or two that cite each other. Two
unlinked entries about the same surface is how a follow-up becomes a place to
put things down.

### F-10 — a writer that connects and never writes starves ingress at 500 ms per connection, and the spec does not say so

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:452-461`;
`draft-spec.md` §6.4

**Expected:** §6.4's *"What is not bounded, and why"* names exactly one
unbounded wait — the wait for judgement — and argues it is inherited from the
main thread rather than added. Both read bounds are stated so a writer knows
what it can hold.

**Observed:** the accept loop is strictly sequential: `accept` → `handle`
(read, hand over, await judgement, reply, close) → `accept`. A connection that
writes nothing holds it for the full `ENVELOPE_DEADLINE` — 500 ms — during which
no other envelope is accepted at all. Two such connections per second, from any
process on the machine that can open the socket, make ingress permanently
unavailable to every legitimate watcher, and nothing on the diagnostics surface
says why: the stalled connections are answered `timed_out` to *their own*
writers, and a watcher whose envelopes are simply never accepted sees only
latency.

The socket is `0600`, so this is confined to the user's own uid — which on a
desktop includes every application the person runs, not only code they wrote.
`draft-spec.md` §6.4 states the *per-read* bound and the *judgement* wait, and
says the host holds one arrival at a time; it never states the consequence that
the read bound is also a bound on how long one connection can deny all others.
That is the property a watcher author needs and cannot derive from the two
numbers given.

**Evidence:** `ingress/mod.rs:452-461` (sequential; `handle` is awaited before
the next `accept`), `:40` (`ENVELOPE_DEADLINE`), `:518-527` (the read is per
connection); `draft-spec.md:283-294` (§6.4's own account of what is not
bounded); `design.md:523` (I-2, which states the one-at-a-time property for the
judgement wait only). `a_connection_that_writes_nothing_times_out_and_the_listener_serves_next`
(`integration/ingress.rs:617`) drives one such connection and asserts the
listener recovers; nothing drives two.

**Disposition:** doc-wrong
**Response:** The sequential accept loop is the design's choice and stands;
what is missing is that §6.4 never states its consequence. A watcher author is
given the per-read bound and the judgement wait and cannot derive from them that
one connection denies all others for the length of a read. Repair: §6.4's "What
is not bounded, and why" gains the head-of-line property in terms — one
connection at a time, so the read bound is also the bound on how long a single
writer can hold off every other. Whether ingress *should* serve connections
concurrently is a design question and goes to Follow-ups beside [[F-9]]; the
socket's `0600` mode bounds the blast radius to the user's own uid, which is not
nothing but is also not an argument for silence.

**Repaired, 2026-09-09.** §6.4's *What is not bounded, and why* gains a
paragraph — *"One connection at a time, so the read bound is also a denial
bound"* — stating the property in the **writer's** terms per the outcome: the
per-read bound is the longest a single writer can hold off every other; two
silent connections a second make ingress effectively unavailable to every
legitimate watcher; and this is the one way an envelope fails to arrive
**without a refusal**, because the stalled connections are answered `timed_out`
to their own writers and a starved watcher sees only latency. The `0600` mode's
bound on who can do it is stated with it. Documentation only. The design
question — whether ingress should serve connections concurrently — is not
opened here; it is reconciled with [[F-9]]'s deferral, and this repair touched
no `slice-004.md` Follow-ups, which land at close.

**Outcome:** verified. The property is worth stating in the writer's terms as
well as the host's: what a watcher author needs to know is that a refusal is not
the only way an envelope fails to arrive — it can also simply wait, behind
somebody else's connection, for up to one read bound. §6.4 states two numbers
today and a reader cannot derive that from them. Its deferred half is reconciled
with [[F-9]]'s, per that finding's outcome.

### F-11 — the contract records no rule for a symlink at the socket path, and the implementation has one

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:159-196`;
`draft-spec.md` R-3, R-4

**Expected:** R-3 and R-4 divide the world in two: *"a socket no live host
holds"* is reclaimed, *"a socket a live host holds"* is a startup failure, and
*"anything that is not a socket"* is a startup failure naming what was found.
The spec is what a person configuring the host reads.

**Observed:** `reclaim` uses `symlink_metadata`, so it never follows the link,
and `describe` reports `"a symlink"`. A symlink pointing at a socket a live host
holds is therefore refused as `NotASocket { found: "a symlink" }` — with the
message *"not a socket — found a symlink"* — rather than as `InUse`. The
behaviour is deliberate and defensible (`design.md:581`: *"A symlink to a socket
is ambiguous and is refused rather than followed"*), and it is the right call:
following a link at a path the host is about to `chmod` would be worse.

What is missing is that the *spec* says none of it. R-3/R-4 admit only "socket"
and "not a socket", and by the letter of R-3 a symlink to a live socket is a
path occupied by a socket a live host holds. A person who symlinks the
configured path — a normal thing to do with a socket under `/run` — gets a
startup failure whose message asserts something that is not true of the target,
and no document to check it against. The design records the rule; the design is
not canon, and `docs/AGENTS.md` §*Canon that does not exist yet* makes the draft
spec, not `design.md`, the slice's working authority.

**Evidence:** `ingress/mod.rs:160-172` (`symlink_metadata`, then the file-type
gate), `:180-196` (`describe`'s `"a symlink"` arm); `draft-spec.md:95-96` (R-3,
R-4); `design.md:581`. The path is untested in both tiers — the integration
module covers a regular file and a directory, not a link.

**Disposition:** doc-wrong
**Response:** The implementation's rule is right — following a link at a
path the host is about to `chmod` would be worse, and `design.md:581` reasoned
it. The defect is that R-3/R-4 admit only "socket" and "not a socket", so by
their letter a symlink to a live socket is R-3's case, and `design.md` is not
canon: §*Canon that does not exist yet* makes the draft spec the slice's
authority. Repair: R-3/R-4 gain the symlink rule explicitly, unfollowed and
refused, with the reason. Add the missing case to the integration tier while
there — the path is untested in both tiers, which is how a documented rule and
an undocumented one come to look alike.

**Repaired, 2026-09-09.** R-3 now says the path is inspected **without
following symbolic links**, so it speaks of a socket *at* the path and never of
one a link points to; R-4 gains the symlink rule explicitly — not followed, a
startup failure naming it as a symlink whatever it points at, *including a
socket a live host holds* — with the reason: following a link would put R-2's
owner-only mode on a file the configuration never named.

The integration case is the one the outcome asks for, and only that one:
`ingress::a_symlink_to_a_live_socket_is_refused_unfollowed_and_the_target_keeps_serving`
(`crates/goad-shell/tests/integration/ingress.rs`) binds a live socket, links
the configured path at it, asserts `NotASocket { found: "a symlink" }` **and not
`InUse`**, and then shows the target host still serving. A link to nothing was
not added: it agrees with every reading of R-3 and would pass whether the rule
existed or not. §7's R-4 row names the case.

**Outcome:** verified. The case that earns its place is a symlink pointing at a
**live** socket, asserted `NotASocket { found: "a symlink" }` and not `InUse` —
that is the one where the rule and R-3's letter disagree, and the one the
amended text has to be true of. A symlink to nothing exercises the same arm
while agreeing with every reading, so it would pass whether the rule existed or
not.

### F-12 — the example backend interpolates envelope values into JSON without escaping

**Severity:** minor
**Location:** `examples/shell/backend.sh:39-55`

**Expected:** `examples/` is the worked example a watcher author copies, and
`examples/shell/backend.sh` is the backend the AC-13 human observation actually
ran. Its existing comment already warns that substring matching on JSON *"is not
something to imitate"*; the new code goes further than matching.

**Observed:** the new arm extracts `source` and `kind` with shell parameter
expansion and interpolates them straight into a JSON string literal:

```sh
source=${request#*'"source":"'}
source=${source%%'"'*}
…
"title": "An event arrived: '"$source"' / '"$kind"'",
```

`source` and `kind` are watcher-chosen and admit any non-empty string. A `source`
of `he said "hi"` closes the JSON string early and the backend emits a document
the host refuses; a `source` containing a backslash produces an invalid escape.
The host handles it correctly — this is a backend failure, reported, and the
host stays up — but the example demonstrates a pattern that breaks on the first
quote, in a file whose whole job is to show a watcher author what a backend
looks like.

Second, smaller point in the same file: `*'"source":"host"'*` matches the
substring anywhere in the request, so an ingested envelope whose `data` contains
the literal `"source":"host"` takes the host branch. The comment above it argues
correctly that field order makes the *extraction* read `event.source`; the
`case` pattern above it has no such protection.

**Evidence:** `examples/shell/backend.sh:39-55` (the new arm), and `:25` for the branch
pattern above it; `draft-spec.md:206` (`source` admits *"a non-empty string, not
`\"host\"`"* — nothing about quoting). `envelope.rs`'s own tests confirm the host
carries such a `source` through unexamined, which is correct and is what makes
this reachable.

**Disposition:** fix-now
**Response:** A `source` or `kind` containing `"` or `\` produces broken JSON
from the one file in this project allowed to know what an event means — and it
is the file a person copies to write their own backend, so the defect
propagates by design. Escape properly. If that cannot be done with what the
devshell already declares, **stop and ask** rather than adding a dependency:
a dependency addition is a STOP condition, not a repair decision.

**Repaired, 2026-09-09. No dependency was needed, and no STOP was reached.**
The shebang question the outcome flags is already settled by the file: it has
none, and `examples/demo.toml` names `["bash", "examples/shell/backend.sh"]`,
so bash is the program and `${value//from/to}` is available. The file now says
so in its header, to save the next reader re-deriving it.

Both halves of the finding are answered. **Escaping:** an `escaped` helper
doubles `\` and then escapes `"`, and both interpolated values go through it.
**The branch:** `source` and `kind` are extracted *before* the branch, and the
host arm matches on the extracted `$source` rather than on `*'"source":"host"'*`
anywhere in the request — so an ingested envelope whose opaque `data` carries
that literal can no longer choose which prompt the example shows. The comment
says why.

Held by `round_trip::the_shell_example_escapes_the_values_it_carries_into_a_view`
(`crates/goad-shell/tests/integration/round_trip.rs`), which runs the example
through the real transport, as `["bash", script]`, with `source` = `he said
"hi"` and `kind` = `back\slash`. The discriminator is that the example
**answers at all** — the host handles broken backend JSON correctly, which is
exactly why nothing else caught this — plus the view's title. Confirmed red
against the file at `HEAD` and green after. The remaining limit is fidelity,
not validity: parserless extraction still truncates a value at its first
escaped quote, which is what the file's own *"not something to imitate"*
already says.

**Outcome:** verified, with one half of the finding still owed. The Response
answers the escaping; it does not mention the second point — `*'"source":"host"'*`
matching the substring **anywhere** in the request, so an envelope whose `data`
carries the literal `"source":"host"` takes the host branch. That is the same
defect as the escaping one (a value the host carries opaquely changing the
example's control flow) and belongs in the same repair.

Practical note so the STOP lands early rather than late: whether escaping is
reachable at all turns on the script's shebang — `${v//\\/\\\\}` is bash, not
POSIX `sh` — so settle that before writing the fix, not after.

### F-13 — `ingest` writes the event anchor with a panicking `Instant` addition

**Severity:** nit
**Location:** `crates/goad/src/controller.rs:492`

**Expected:** `deadline_after` (`controller.rs:361-379`) exists because
`Instant + Duration` panics on overflow and *"a panic here would be a backend
failure taking the host down (SPEC-001/R-45)"*; it uses `checked_add` with a
clamp, and has a test named for exactly that.

**Observed:** the event anchor's one write site is
`*event_floor_until = arrived + MINIMUM_SPACING;` — the plain, panicking `+`.

Unreachable in practice: `arrived` is `Instant::now()` and `MINIMUM_SPACING` is
a 3-second constant, so unlike `deadline_after`'s input this addition is not
driven by anything a backend or a writer chose. Raised only because the module
states the hazard in a doc comment thirty lines above and then does not follow
its own rule at the one new site, and because `ingress/mod.rs` carries
`#![deny(clippy::arithmetic_side_effects)]` while `controller.rs` does not — so
the two halves of one slice are held to different standards by the lint, not by
a decision anyone recorded.

**Evidence:** `controller.rs:492`; `controller.rs:361-379` and `:762-777` of the
inline test module for the rule it departs from; `ingress/mod.rs:14` versus
`controller.rs:1-6` for the lint asymmetry.

**Disposition:** fix-now
**Response:** Take it — the rule is stated thirty lines above at
`deadline_after` and this is the one site that does not follow it. A nit that is
already answered by neighbouring code is cheaper to fix than to carry.

**Repaired, 2026-09-09.** `crates/goad/src/controller.rs`'s one write site is
`arrived.checked_add(MINIMUM_SPACING).unwrap_or(arrived)`, with one line saying
it follows the rule `deadline_after` states and what the fallback does — the
anchor stays at `arrived`, so the spacing degenerates rather than the process
ending. `deadline_after`'s *justification* is deliberately not imported, per the
outcome: that argument is about a wait a **backend** chose, and this addend is a
host constant.

**No test is red before this**, and there is no honest way to make one:
`arrived` is `Instant::now()` and the addend is a three-second constant, so the
overflow is unreachable without constructing an `Instant` the platform does not
offer. The finding says as much. `deadline_after`'s own
`a_wait_that_would_overflow_the_clock_is_clamped_rather_than_panicking` remains
the instrument for the rule; this site now follows it.

**Outcome:** verified. Do not import `deadline_after`'s *justification* with its
technique: that docstring argues from a wait a **backend** chose, and this
addend is a host constant, so repeating the argument here would put a claim in
the source that is not true of the site. `checked_add` with a stated fallback
and one line saying it follows the module's rule is the whole of it.

### F-14 — the example's `respond` branch still matches a substring of the whole request, which a watcher controls

**Severity:** minor
**Location:** `examples/shell/backend.sh:43-46`

**Expected:** [[F-12]]'s second half named the defect and the repair's own new
comment states the rule it followed (`backend.sh:23-26`): *"the branch reads
this value rather than the whole request: `case $request in
*'\"source\":\"host\"'*)` would also match an ingested envelope whose opaque
`data` happened to carry that literal, and hand a watcher control of which
prompt this file shows."* The ledger's guardrail is **fix the class, not the
instance**.

**Observed:** the `source` branch was fixed exactly as described — `source` and
`kind` are extracted first and the inner `case $source in host)` matches the
extracted value. **The `type` branch one line above was left as a whole-request
substring match**, and it is the same defect with the same consequence:

```sh
case $request in
  *'"type":"respond"'*)
    printf '{"view":null,"next_check":"45 minutes"}\n'
    ;;
```

`data` is opaque and reaches the backend byte-adjacent to the request's own
keys, so a watcher that sends `"data":{"type":"respond"}` matches this branch.
The example then answers `{"view":null,…}` and shows **nothing** — the watcher
has chosen which prompt the file shows, which is precisely what the repair's
comment says the fix was for. Silently showing nothing is if anything the worse
of the two outcomes, because there is no wrong prompt to notice.

The fix is the technique already in the file: extract `type` the same way
(`"type"` is the request's own second key, before any payload, so the first
occurrence is the right one) and branch on the extracted value.

**Evidence:** run against the repaired file:

```
$ printf '%s' '{"protocol":1,"type":"evaluate","now":"…","event":{…,"data":{"type":"respond"}}}' \
    | bash examples/shell/backend.sh
{"view":null,"next_check":"45 minutes"}

$ # the same request with "data":{}
{ "view": { … "title": "An event arrived: w / k", … } }
```

`examples/shell/backend.sh:43-46`; the rule it departs from at `:23-26`;
`draft-spec.md` §6.2 (`data` admits any JSON value). The new case
`round_trip::the_shell_example_escapes_the_values_it_carries_into_a_view` covers
the escaping half of F-12 and does not reach this branch.

**Disposition:** fix-now
**Response:** Confirmed by the reviewer running it, which is the right standard for
an example. F-12's repair wrote the rule into a comment at `:23-26` and then
left the branch one line above disobeying it — the worst of the two states,
because the file now documents a discipline it does not keep. The technique is
already in the file: read the extracted value, not the whole request. The
reviewer's point about severity is taken and does not change the disposition:
showing *nothing* is worse than showing the wrong prompt, because there is no
wrong prompt for a person to notice.

**Repaired, 2026-09-09.** `type` is extracted the same way `source` and `kind`
are, and the outer branch is now `case $type in respond)`. **No branch in the
file matches against `$request` any more** — that is the class, and the comment
at the head of the extractions was rewritten to state it as one rule over every
branch rather than as a remark about the `source` branch: *"a value the host
carries opaquely must not reach this file's control flow."* It also says why the
first occurrence is the right one for each of the three, which is what makes the
technique safe rather than lucky: `type` is the request's own second key and
`source` and `kind` are the event's first two, so all three precede any `data` a
watcher wrote.

Held by `round_trip::the_shell_examples_branch_is_the_hosts_to_decide_and_not_a_watchers`
(`crates/goad-shell/tests/integration/round_trip.rs`), which sends **both**
literals the file branches on inside `data` — `{"type":"respond"}` and
`{"source":"host"}` — and requires the ingested branch for each. One case for
two literals, because it is one rule. Confirmed red with the old branch restored,
failing exactly as the reviewer reported: *"`data` of {"type":"respond"} must
not decide whether the example shows anything: nothing to show, and no failure."*
The `{"source":"host"}` half was already green — F-12 fixed that branch — and it
is in the case so the rule is held whole rather than at the point it last
broke.

**Outcome:** verified, and the repair is landed and checked. `type` is now
extracted like `source` and `kind`, **no branch matches `$request` at all**, and
the comment states one rule over every branch instead of a remark about the one
that was fixed — which is the class, not the instance. Confirmed by running it:
`data` of `{"type":"respond"}` and of `{"source":"host"}` both now reach the
event branch and produce `"An event arrived: w / k"`, and a real `respond`
request still short-circuits. Its test drives both literals and asserts the
title, so it holds both branches rather than the one I reported.

The ordering note I was going to attach is discharged by the repair rather than
by argument: `source` and `kind` are still read above the branch and still hold
`{` for a `respond` request, but with `$type` driving the outer `case` no arm
that reads them is reachable for such a request. Nothing holds a meaningless
value on a path that consults it.

### F-15 — §6.3 says a faulted connection's `unavailable` always reaches a person; the code reaches it only while idle

**Severity:** minor
**Location:** `draft-spec.md:265-271`; `crates/goad/src/controller.rs:441-448`
(`refuse_during_exchange`)

**Expected:** R-15 is general and correct: a refusal decided **while no exchange
is in flight** reaches the diagnostics surface, and one decided while an
exchange *is* in flight is *"reported to the writer only."* §6.3's *"Which
refusals a person sees"* paragraph exists to spell that out per reason, and it
already gets the distinction right for shape refusals — *"a shape refusal
reaches it when the host happened to be idle and not otherwise, which is what
makes shape-before-state (§5) a claim with a negative case."*

**Observed:** [[F-7]]'s repair added the new cause to that paragraph on the
wrong side of the distinction:

> `too_soon`, the clock's `unavailable` **and a faulted connection's** always
> do

`too_soon` and the clock's `unavailable` are decided by the **loop**, at
`design.md` §5.4 steps 3 and 4, which only the outer arm reaches — so *always*
is true of them. A faulted connection's `unavailable` is decided by the
**listener**, in `read_envelope`, exactly like every shape refusal; it travels
in the `Arrival` and its fate is whichever arm receives it. Reached during an
exchange, `refuse_during_exchange` folds it and the inner loop presents nothing,
so `absorb` supersedes it before any frame — the same guaranteed loss
[[F-3]] was raised about.

So a sentence added to fix one finding restates, one row down, the error
[[F-3]] existed to remove: a spec clause unconditional where the code is
conditional. The requirement (R-15) is right; the paragraph that explains it now
contradicts it.

**Evidence:** `draft-spec.md:270-271` for the claim. The path:
`ingress/mod.rs:648` (`Ok(Err(io)) => Err(unreadable(io))`, in the accept task),
`:676-678` (`unreadable` builds the `Refusal`), carried as
`Arrival { result: Err(_) }`; then `controller.rs:441-448`
(`refuse_during_exchange` — `Err(shape) => shape`, folded via `refuse_arrival`,
no `glass.present`) versus `:481-487` (`ingest` step 1, folded and then
presented by the outer loop's `continue`). The correct placement is beside the
shape refusals: *reaches a person when the host happened to be idle, and not
otherwise*. No test asserts the claim in either direction — the negative
already exists in shape for
`a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`.

**Disposition:** doc-wrong
**Response:** The document is wrong, and it is wrong in exactly the way [[F-3]] was
raised to stop — one row down from where we just removed the same defect. `too_soon`
and the clock's `unavailable` are decided by the **loop** at §5.4 steps 3 and 4,
which only the outer arm reaches, so *always* is true of them. A faulted read is
decided by the **listener**, travels in the `Arrival`, and during an exchange is
folded without presenting and then superseded by `absorb`. F-7's new cause was
filed on the wrong side of the distinction. Move it beside the shape refusals and
state the conditionality there. R-15 stays as written — it is the paragraph
explaining R-15 that contradicts it, not R-15.

This is the second instance of one class: **a spec clause unconditional where the
code is conditional.** Repair fixes the class — sweep §6.3 and §5 for any other
clause asserting a refusal reaches a person without saying which arm decides it,
rather than patching this row alone.

**Repaired, 2026-09-09**, as the class. R-15 is untouched. §6.3's *"Which
refusals a person sees"* is no longer a list of reasons with a fate attached to
each — it is now organised by **which side decided the refusal**, which is what
actually settles it:

- decided by whatever accepts connections, and so travelling in the `Arrival`
  with the envelope — `malformed`, `invalid_envelope`, `reserved_source`,
  `too_large`, `timed_out`, **and the faulted read's `unavailable`**, now filed
  here: they reach the surface when the loop happened to be idle and not
  otherwise;
- decided by the loop and only while nothing is in flight — `too_soon` and the
  clock's `unavailable`, for which *always* is exact, because §5.4's steps 3 and
  4 are reachable in no other state;
- `engaged`, decided during an exchange by definition; the stopping
  `unavailable`, with no loop left to present it; and the ingress-stopped
  `unavailable`, which answers no envelope.

The paragraph closes with the rule that makes the next such clause checkable
rather than plausible: *"A clause of this paragraph that says a refusal always
reaches a person is making a claim about who decides it… One that cannot say
which side decides is a clause that has not been checked."*

That sentence is written as a **criterion** and is meant to be promoted as one.
`audit.md`'s Reconciliation row for promoting this draft to `SPEC-003` now says
so in terms, so it is not trimmed to a row note on the way into canon: it is
what makes the next instance of this class findable by reading, and this sweep
found two instances of it.

**The sweep found one more**, in the other direction — §6.3's causes paragraph
still said the ingress-stopped cause *"is reported to a person under R-15 or not
at all"*, which was true before [[F-3]] and under-claims after it. It now says
the surface is the only report it has and that it is reported whichever state
the loop was in, matching §5, which gained the same clause. Nothing else in §5
or §6.3 asserts a reach without naming the deciding side.

**No new test, and the reason is [[F-7]]'s stated gap.** The clause this
corrects is about a refusal no test can produce — a faulted AF_UNIX read is not
reachable from a cooperating writer. What the correction says is already held in
its class by `a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`,
which drives a listener-decided refusal through the inner arm and asserts it
reaches no frame; the faulted read takes that identical path, in the same
`Arrival`, and the spec now files it there.

**Outcome:** verified, and the sweep did what sweeping is for — it found one
I had not.

The paragraph is now organised by **which side decided the refusal**, with the
listener's six (the faulted read's `unavailable` among them, where it belongs),
the loop's two, `engaged`, the stopping `unavailable`, and the ingress-stopped
one that travels the other way. It closes with the criterion itself — *"One that
cannot say which side decides is a clause that has not been checked"* — which is
the durable part: it makes the next instance of this class findable by reading
rather than by a reviewer happening to trace the path.

**The extra one is the interesting result.** §6.3's `unavailable` prose still
said the ingress-stopped cause *"is reported to a person under R-15 or not at
all"* — obsolete since [[F-3]]'s repair made it unconditional, and
**under**-claiming rather than over-, which is the direction a sweep looking only
for the reported defect would have missed. §5 and §6.3 now agree. That is a
second instance of the class found by the class fix, which is the argument for
sweeping made in one line.

Six, two, one, one and one — my own count matches the repaired text.

### F-16 — `audit.md`'s AC-3 row still describes the code [[F-8]] repaired, and the amendment its Response promised was not made

**Severity:** minor
**Location:** `docs/slices/004/audit.md:110` (AC-3), `:12-14` (Subject)

**Expected:** [[F-8]]'s Response, which I verified, says in terms:
*"`audit.md:110`'s justification is wrong on this point and the audit is amended
with it."* `audit.md` is the slice's closing argument, and `docs/AGENTS.md:85`
classes it with the artefacts that hold **current truth**.

**Observed:** the repair landed in the code and the amendment did not. AC-3
still reads:

> The one admitted exception is visible in the code: `ingress/mod.rs:487-489`
> returns without replying only when `arrivals.send` fails — the judge is gone,
> which is the host process going away (`draft-spec.md` R-8)

Every clause of that is now false. There is no admitted exception on that path:
`handle` writes `stopping()` before returning (`ingress/mod.rs:584-586`). The
line numbers no longer point at it. And the justification — *the judge is gone,
which is the host process going away* — is the reasoning F-8 showed was a timing
assumption rather than a property, which is why the code changed. `audit.md:111`
(AC-4) was corrected for [[F-5]] in the same pass, with a dated *Corrected*
note; AC-3 was not.

Same class, and cheaper to fix together: the audit's **Subject** still says
*"`b6ca5f7..93abab3` … `93abab3` is HEAD, the tree is clean"*, and its Evidence
records `just check` run on `93abab3` and a surface delta walked over
`9cfb679^..93abab3`. HEAD is `441fa94`, seven commits later, and those commits
touch a file no phase declared and the surface walk does not list —
`crates/goad-shell/tests/integration/round_trip.rs`. The audit is arguing for a
tree that is not the one shipping. (The gate does hold on the shipping tree: I
re-ran `just check` at `441fa94` myself, exit 0.)

**Evidence:** `audit.md:110` against `crates/goad-shell/src/ingress/mod.rs:584-586`
and the new case
`ingress::a_connection_accepted_after_the_judge_is_gone_is_answered_unavailable`
(`tests/integration/ingress.rs:525-541`), which asserts the reply that row says
does not exist; `audit.md:12-14` against `git log --oneline -1` (`441fa94`);
`git diff --name-status 93abab3..441fa94` for `round_trip.rs` against
`audit.md:218-245`'s file-by-file walk.

**Disposition:** fix-now
**Response:** Correct, and my own fault: F-8's Response promised the amendment and
nothing carried it out. Every clause of AC-3's row is now false — no unanswered
close on that path, line numbers pointing elsewhere, and a justification resting
on the timing assumption F-8 disproved. AC-4 got a dated *Corrected* note in the
same pass and AC-3 got nothing.

Take the whole class in one pass, as the reviewer says: **the audit is currently
arguing for a tree that is not shipping.** Its Subject names `93abab3` as HEAD
with a clean tree; its gate evidence is that tree; and its file-by-file surface
walk does not list `round_trip.rs`, which the repairs added and which no *phase*
declared — correctly so, since it arrived at repair, but the walk has to say that
rather than omit it. Refresh Subject, Evidence and the surface walk to the
shipping tree, mark each correction dated and attributed to its finding, and do
not silently restate — an audit that quietly updates its own evidence is worth
less than one that shows what changed under it.

**Repaired, 2026-09-09**, taking both of the outcome's two things. Nothing in
`audit.md` was silently restated: every correction is dated, attributed to the
finding that drove it, and quotes what it replaces.

- **AC-3's row.** Corrected, and it says so, quoting the old justification in
  full and naming all three ways it is now false — no admitted exception on that
  path, line numbers pointing elsewhere, and a justification resting on the
  timing assumption F-8 disproved. It names
  `a_connection_accepted_after_the_judge_is_gone_is_answered_unavailable` as the
  case asserting the reply the old row said did not exist. Attributed to
  [[F-8]], *"whose Response promised this amendment and whose repair pass did
  not make it — caught by F-16"*.
- **Subject.** Names both ranges and keeps them apart: `b6ca5f7..93abab3` is the
  eight phases, `93abab3..HEAD` the repairs. The refresh note says in terms that
  the document had been arguing for a tree that is not shipping, and that the
  two ranges were made under **different disciplines** — declared surfaces
  versus a numbered finding.
- **The surface walk**, in the outcome's honest form. The existing walk now
  states that it covers `9cfb679^..93abab3` **and nothing after it**, and a
  second walk covers the repair range as its own section: six files, each
  against the findings it answers, with no phase retro-fitted to any of them.
  `round_trip.rs` is named explicitly as the file the phase walk does not list
  and correctly does not — *"its provenance is a finding, not a phase; the walk
  says so rather than omitting it, which is how it went unrecorded until
  F-16."* The section also records that `Cargo.lock` does not move in this
  range, because no repair added a dependency and two declined to.
- **The gate line, on both trees rather than collapsed into one.** And it is
  **not uniformly green, which is now recorded rather than smoothed**: a run
  here failed on
  `ingress::a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves`
  — the reclaim flake `99abac4` chased and could not reproduce. Measured before
  reporting: 1 failure in 35 sequential runs on the shipping tree, **2 in 60 on
  a worktree at `93abab3`**, which predates every repair. Same case, same
  message, comparable rate — so it is pre-existing, not the repairs'. The audit
  now says the gate passes on the shipping tree and **is not deterministic on
  either**, and `slice-004.md` Follow-ups gains the reproduction with the
  condition that finds it: repeated sequential runs of the one target,
  *unloaded*, which is the opposite of the parallel load PHASE-03's chase used
  and why that chase missed it. The gate line names the tree each run was made
  on rather than a range: the failure and the 35-run measurement at `aef04c4`,
  and three consecutive green runs at `4fb79c0`, the tree this refresh is dated
  to.

**Also in this pass, from the reviewer's caller census of
`Refusal::reason()`.** `slice-004.md`'s `Reason`-type follow-up entry now
carries the census in full, so whoever picks it up does not re-run it. It adds
the one thing the entry did not make visible: `ingress/mod.rs` holds a **test**
caller as well as the definition — the in-module case
`a_connection_that_faults_mid_read_is_unavailable_and_carries_the_error` asserts
on the token and needs the same update. The file was already named, so the scope
was complete rather than wrong. The entry also now says why the renderer tier's
many `reason(...)` calls are not callers: they are that file's own reply
parser.

**Outcome:** verified — the disposition. **The repair has not landed yet**:
`aef04c4` carries F-14 and F-15 and does not touch `audit.md`, so this one is
still owed and I will re-review it when it lands.

Taking the whole staleness rather than the row is right, and *"do not silently
restate"* is the part that matters — an audit that refreshes its own evidence
without showing what moved is the same defect as one that never refreshes it.

Two things for the pass. The surface walk's honest form is not to add
`round_trip.rs` to a phase — no phase declared it and none should be
retro-fitted — but to say the walk covers `9cfb679^..93abab3` and name the
repair range's own surfaces separately, since they arrived under a different
discipline. And the gate line should say which tree it was run on: the audit's
transcript is from `93abab3`, and I re-ran `just check` at `441fa94` myself,
exit 0. Two runs on two trees is a fact worth recording rather than collapsing
into one.

See [[F-17]] — the same promise-tracking gap, on [[F-3]] rather than [[F-8]],
and it belongs in this finding's own pass through `audit.md`.

### F-17 — [[F-3]]'s Response promised a Design drift entry for `design.md:229` and none was written

**Severity:** minor
**Location:** `docs/slices/004/audit.md:371-387` (*Design drift not reconciled*);
`docs/slices/004/design.md:229`

**Expected:** [[F-3]]'s Response, which I verified, ends: *"R-15 and §5 keep
their unqualified wording; it is `design.md:229`'s 'when the loop was idle' that
recorded an implementation accident as intent, and **that goes under Design
drift** rather than being adopted."* `docs/AGENTS.md:168` is the rule it was
honouring — *"where the implementation departed and the design stands as
written, say so under **Design drift not reconciled**."*

**Observed:** `design.md:229` still reads, of the ingress-stopped
`unavailable`, *"the ingress-stopped case, **when the loop was idle** — and that
fold is the only report of it there is."* [[F-3]]'s repair made it
unconditional — both arms present it — and the repair pass amended
`draft-spec.md` §5 to say so in terms, and §6.3 again under [[F-15]].
`design.md` was correctly left as written: `git log 93abab3..HEAD --
docs/slices/004/design.md` is empty, which is the departure `docs/AGENTS.md:168`
is about.

**What is missing is the record of it.** `audit.md`'s *Design drift not
reconciled* section opens *"one item, and it is the one the row above proposes
to close"* and describes only `design.md` §5.2's `Refusal` payload list. There is
no second item and no Reconciliation row: `design.md:229` appears nowhere in
`audit.md` at all. So the design now contradicts the code and the draft spec on
a point a finding was raised about, and the one document whose job is to record
that says there is one item of drift.

This is [[F-16]] again with a different parent — a Response promising an edit in
a document other than the one it repairs, with nothing tracking the promise.
Raised separately rather than folded in because it is a different document, a
different section and a different finding's debt; raised **now** rather than at
round 3 because F-16's repair pass is already inside `audit.md` and this belongs
in the same pass.

**Evidence:** `design.md:229` (unchanged — `git log 93abab3..HEAD --
docs/slices/004/design.md` returns nothing); `audit.md:371-372` (*"one item"*);
`grep -n "when the loop was idle\|design.md:229" docs/slices/004/audit.md` →
no match. Against `draft-spec.md` §5's amended *"This one is unconditional on
the host's state: it is reported whether the loop was idle or mid-exchange when
ingress died"*, and `controller.rs:726-741`, the branch that presents it.

**Disposition:** fix-now
**Response:** Correct, and it is mine in the same way [[F-16]] is: F-3's Response
promised the entry, I verified that Response, and nothing carried the promise
out. `design.md` staying as written is right — `docs/AGENTS.md:168` requires it,
and no repair commit touched it. **What is missing is the record of the
departure, not the departure.**

The state to fix: `design.md:229`'s *"when the loop was idle"* now contradicts
both the code and the draft spec on the exact point F-3 was raised about, and
`audit.md`'s *Design drift not reconciled* still opens *"one item"* and describes
only the `Refusal` payload list. Repair adds the second item, in the same pass as
F-16 since both are `audit.md` and both are the same class.

**Repaired, 2026-09-09**, in F-16's own pass. `audit.md`'s *Design drift not
reconciled* now opens **two items** and carries the second: `design.md:229`'s
*"when the loop was idle"*, what [[F-3]] showed it to be — an implementation
accident recorded as intent — what the repair changed, and the test that holds
the unconditional behaviour. It states that `design.md` is left as written **on
purpose**, cites `docs/AGENTS.md:168` for why, and evidences it with `git log
93abab3..HEAD -- docs/slices/004/design.md` being empty. Unlike the first item
it carries **no Reconciliation row**, and the entry says why: nothing is proposed
to the user, because nothing should change.

The sweep this came from is closed: F-3 and F-8 are the only two Responses in
the ledger promising an edit outside the file they repair, and both are now
findings. F-5's four cross-document edits and F-9's and F-10's Follow-ups
landings were all made. Nothing else is outstanding on that axis.

**Outcome:** verified. The entry does what F-3's Response promised and one
thing more: it states the qualification, says why it was an implementation
accident rather than intent, cites the case that now holds the repaired
behaviour, and then makes the *absence* of a Reconciliation row explicit —
*"nothing is proposed to the user, because nothing should change"* — with `git
log 93abab3..HEAD -- docs/slices/004/design.md` returning empty as the evidence
that `design.md` was left alone deliberately. That last part is what stops a
future reader mistaking the drift for an oversight, which is the failure mode a
drift entry exists to prevent.

### F-18 — `reclaim`'s liveness probe infers a live host from `connect()`, and that inference is false in any process that forks

**Severity:** major
**Location:** `crates/goad-shell/src/ingress/mod.rs:159-176` (`reclaim`) and its
doc comment at `:147-158`; `draft-spec.md` R-3

**Expected:** R-3 divides the world in two and the probe is what divides it: *"A
path that is occupied by a socket **no live host holds** MUST be reclaimed… A
path a **live** host holds MUST be a startup failure naming the path."* A host
that refuses to start against a socket no host holds has broken R-3, and the
person meets it as *goad will not start, naming a live host that does not
exist*, with the recovery being to delete a file the host said it would reclaim.

**Observed:** `reclaim` decides liveness with

```rust
if std::os::unix::net::UnixStream::connect(path).is_ok() {
  return Err(fault(path, BindFault::InUse));
}
```

and its doc comment argues for it in terms: *"a Unix domain socket has no atomic
'is anyone listening' query, and a failed connect to a stale socket file is the
standard idiom."* The idiom is fine; **the inference drawn from it is not.**
`connect()` succeeding means *a live listening socket is bound to that path*. It
does **not** mean a live **host** holds it, and the gap is reachable: a `fork`
duplicates the listening descriptor into the child, so between fork and exec —
where `CLOEXEC` closes it — the socket object still has a reference and remains
a live listening socket bound to that path **after its owner has closed its own
descriptor**. `connect` then succeeds correctly, and `reclaim` reports `InUse`
about a host that is gone.

This is the cause of the reclaim flake `99abac4` chased and could not
reproduce: `a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves`
failing `in use by a live host`.

**Evidence — five measured steps, three independent implementations, controls
throughout.**

1. **The kernel does not lie on its own.** `bind → listen → close → connect` in
   a loop with nothing else running: **0 spurious successes in 200,000**
   single-threaded, **0 in 320,000** across 16 concurrent threads.
2. **The case never fails alone.** VT-1 run by itself: **0 failures in 120
   runs**. It requires the rest of the suite.
3. **Caught in the act**, by instrumenting the failure path and reproducing at
   run 44 of 120 of the full target. At the moment `reclaim` said *in use by a
   live host*: `ss -xa | grep vt1-reclaim` was **empty**, an immediate reconnect
   gave `ECONNREFUSED`, and the file was mode `0755` — the case's own std
   listener, never goad's `0600`. That rules out a second listener at the path
   (case names are unique, checked), a temp-dir collision (`pid_max` is
   4194304) and a leftover file.
4. **Isolated from goad entirely**, with a probe doing only those syscalls and a
   fresh unique path per iteration, so no second listener can exist:

   | condition | spurious / 40,000 |
   |---|---|
   | alone in the binary | **0** (×3) |
   | + 400 live AF_UNIX listeners in *another* process | **0** (×3) |
   | + one live goad listener, same process, no forking | **0** (×3) |
   | + the `transport::` cases — subprocess-heavy, **no sockets** | **4**, then **7** |
   | the full suite | **18** |

   It tracks forking, not listeners.
5. **Reproduced outside Rust, and demonstrated deterministically.** Pure Python,
   one thread probing its own just-closed listener and one thread forking:
   **47 spurious in 40,000**; with the forking thread disabled, **0 in 40,000**.
   Then, with no race at all — bind, `fork` a child that sleeps, close the
   parent's descriptor, probe:

   ```
   while the forked child still holds the inherited fd:  connect SUCCEEDED
   after that child has exited:                          connect refused (111)
   ```

**Confidence.** High on the fact — three independent implementations with clean
controls. High on the mechanism, on the strength of step 5's deterministic
demonstration plus step 4's discriminators; it is **strongly-supported inference
rather than direct observation**, because the child's descriptor table was not
caught mid-window (`strace -f`), which was judged not worth the time against
evidence this consistent.

**On severity, and what the rate does *not* say.** The ~3% per full run is a
property of **the test binary's shape**, not an estimate of what a person meets:
there the probing process is itself forking constantly, so it duplicates its own
listeners' descriptors. In production the probe would have to catch a
*different* host's socket duplicated into a child that is mid-fork at the
instant that host died, and a dead host's children have already exec'd, where
`CLOEXEC` closed the descriptor. **The rate does not transfer, and a writeup
that carries it across would overstate this.** What does not depend on the rate:
the inference is unsound, and today's narrowness holds only because `reclaim`
runs before `serve`, so goad is not yet spawning backends when it probes —
**an accident of startup ordering that nothing states as load-bearing and
nothing would fail if it changed.** Raised `major` on that ground: a stated
invariant is decided by a test that does not decide it.

**Disposition:** fix-now
**Response:** R-3 is a stated invariant and it does not currently hold; that is
the ground for repairing in-slice rather than deferring. Liveness becomes an
**exclusive advisory lock a host holds for its own lifetime**, not an inference
from `connect()`: a sidecar lock file beside the socket, `open` with `O_CREAT`
then a non-blocking exclusive `flock`. Lock acquired → no live host, so unlink
the socket and bind; lock held by another → `InUse`. The guard lives as long as
the process, not as long as `reclaim`. The lock file is **not** unlinked on
exit, for the reason R-5 gives for the socket — a stale lock file is harmless,
because the lock and not the file is the signal.

Why this answers the defect where `connect()` cannot: a dead host's children
have already exec'd and `CLOEXEC` closed the inherited lock descriptor at exec,
so a dead host holds no lock. The fork window that makes `connect()` lie
produces no false *live* here. **User's call, 2026-09-09.**

**PARTIAL — raised and dispositioned here; the repair is not started.** The
repair agent that diagnosed this reached its session budget on the diagnosis and
handed over rather than beginning a fresh unit of work near its ceiling. What
remains, in full, so the next agent starts from a brief and not from chat:

1. **The repair itself**, as the Response specifies.
2. **R-3 and R-4's wording, and `reclaim`'s doc comment.** That comment argues
   for the connect idiom in terms; the argument is now known false and must be
   **replaced, not softened** — say what the lock holds and why connect could
   not.
3. **Upgrade skew, to be settled and stated rather than left implicit:** against
   a socket left by a host that predates the lock file there is no lock, so the
   socket is treated stale and unlinked. Right for a genuinely dead old host,
   wrong for a live one. Write it down wherever R-3's rule now lives.
4. **A Design drift entry in `audit.md`, beside [[F-17]]'s.** `design.md` §5.5's
   edge-case table records the probe's side effect — a live host reading the
   probe as an empty envelope and refusing it `malformed`. **That side effect
   disappears**, since the new probe never connects. `design.md` stays as
   written (`docs/AGENTS.md:168`); the record of the departure is the drift
   entry.
5. **Reconcile [[F-9]]'s Follow-ups entry.** Its second-host-startup instance —
   the one the reviewer required be named in full — is *predicated on that side
   effect* and **no longer exists**. F-9's underlying question stands on its own
   (the diagnostics slot has an author outside the process; the three pre-slice
   refusal paths are still why it is deferred), but the entry must not keep
   claiming an instance that is gone. Say what replaced it and why rather than
   deleting the sentence.
6. **The test that could not be written before**: a stale socket probed while
   the process forks. Step 5 above is the recipe — bind, fork a child that
   holds the descriptor, close the parent's, probe.
7. **Verification.** `just check` exits 0, and the integration target run
   **enough times to say something real about the flake being gone** rather than
   asserting it from one green run. The pre-repair baselines to beat are in this
   finding: 1 failure in 35 sequential runs on the repaired tree, 2 in 60 at
   `93abab3`.

**This repair reverses `design.md` D-10, and that is a design change rather
than a refactor** (`CLAUDE.md`: breaking one of the five is a design change).
`slice-004.md` Follow-ups, *Single-instance enforcement*, records the decision it
reverses: *"Closing it inside the ingress module would be a partial
single-instance guarantee under another name (`design.md` D-10, `research.md`
F15)."* A lifetime-held `flock` in the ingress module **is** that partial
guarantee.

It proceeds anyway, and the ground is stated rather than assumed: D-10 declined
a lock on **scoping** grounds, and this finding rests on **R-3 being broken in
shipping code, measured**. A stated invariant failing outranks a scoping
preference. But the reversal is recorded as a design change, not absorbed into a
repair — which is what items 8-11 below are for.

**The repair therefore delivers partial single-instance enforcement as a
*consequence*, not as a goal.** The goal is a sound liveness inference; one host
per socket path falls out of holding the lock for the process's lifetime.
Whoever picks up the *Single-instance enforcement* follow-up needs to know that
a piece of it already exists, and where.

8. **`draft-spec.md` §6.1's *"Non-normative limit — the bind race"* becomes
   false and must be rewritten to what is now true.** It says two hosts starting
   in the same instant may both find the path stale and both bind, the second's
   file winning. With the lock held for process lifetime only one can hold it,
   and the second sees `InUse`: **this repair closes that race.** Do not leave a
   limit standing that the code no longer has. The *second*, separate limit
   below it — *"the path after the bind"*, nothing re-probing once bound — is
   **not** closed by this and stays as written.
9. **The *Single-instance enforcement* Follow-ups entry loses its first face.**
   The probe/bind race it names as a symptom (OQ-6) is closed. Its second face
   survives untouched: nothing re-probes the path once bound, so a socket
   unlinked underneath a live listener leaves it holding a descriptor no
   `connect` can reach. Reconcile it the way item 5 reconciles [[F-9]]'s — say
   what closed and why, do not delete the sentence.
10. **A third Design drift entry in `audit.md`, for D-10**, beside [[F-17]]'s
    and §5.5's. It is **the most consequential of the three, and should say so**:
    the other two outdate a description, this one reverses a decision.
    `design.md` stays as written (`docs/AGENTS.md:168`).
11. **Add the TOCTOU cross-reference, now.** It needed no fourth copy while it
    was an open limit recorded in two places; it is not an open limit any more,
    and both places — §6.1 and the Follow-ups entry — are rewritten by items 8
    and 9. Link this finding's diagnosis from wherever the closure is stated, so
    the next reader sees why a documented race stopped being one.

**STOP conditions for the next agent:** a dependency addition (`flock` is
reachable through `std::os::unix::io` plus a raw call — if it is not reachable
without a new crate, that is a STOP, not a repair decision), and the lock and
the socket disagreeing in a way the design did not settle.

**Repaired, 2026-09-09 — this supersedes the PARTIAL brief above, which stands
as the record of the handover.** All eleven items landed; what follows says
where each one is.

**1 — the repair.** `reclaim` (`crates/goad-shell/src/ingress/mod.rs`) no longer
connects to anything. It settles what is *at* the path first (R-4 before R-3, so
a symlink or a regular file is refused without a lock file appearing beside it),
then calls `hold`, which opens `lock_path(path)` — the socket's own path with
`.lock` appended — with `O_CREAT` and `mode(SOCKET_MODE)`, and takes a
non-blocking exclusive `flock`. Lock taken → no live host → the socket is
unlinked and `bind` takes the path. `WouldBlock` → `InUse`. Any other lock
error, and any failure to open the file, → a new `BindFault::LivenessUnknown`:
*whether a live host holds it could not be determined*, because neither answer
is safe to assume. `reclaim` returns the `File`, `bind` puts it in
`Ingress::_lock`, and `serve` owns that for the life of the process — the guard
outlives `reclaim` by the whole run.

**No new dependency, and no `unsafe`.** `flock` is reachable as
`std::fs::File::try_lock` (stable; `TryLockError::{WouldBlock, Error}`), so the
STOP condition this finding set was not reached. The mode is set by `open(2)`
rather than by a `chmod` after it, so unlike the socket (A-5) there is no
window.

**2 — R-3, R-4 and the doc comment.** R-3 is rewritten around the lock and now
opens *"Liveness is an exclusive advisory lock, and never a connection to the
socket"*; it states the hold-for-lifetime obligation, both outcomes, and that
liveness must not be assumed either way when it cannot be determined. R-5 gains
the lock file: not unlinked on exit, because the lock and not the file is the
signal. R-4 needed no change — its symlink clause already reads *"rather than as
R-3's in-use case"*, and the R-4-before-R-3 order in `reclaim` is what makes
that true. **`reclaim`'s doc comment is replaced, not softened**: the paragraph
that argued *"a failed connect to a stale socket file is the standard idiom"* is
gone, and in its place is what the lock holds, why `connect` cannot hold it, and
the fork mechanism that breaks it.

**3 — upgrade skew, settled and stated.** In `draft-spec.md` §6.1 as a
non-normative limit, and again in `reclaim`'s doc comment: a socket left by a
host predating the lock file has no lock beside it, so it reads as stale and is
unlinked — right for a dead old host, wrong for a live one, which would go on
serving a socket the path no longer reaches. One restart of the old host closes
it; the window is the single upgrade that crosses this change.

**4, 10 — two Design drift entries in `audit.md`**, making four, with the intro
count and the ordering rewritten. The third records that the probe's side effect
is gone: §5.5's *zero bytes, then EOF → `malformed`* row stays true about
writers and loses its one host-authored instance, and two further sentences go
stale with it (§5.5's *socket unlinked underneath a live listener* row offers *a
second host reclaiming the path under R-3* as a cause, now reachable only if the
lock file has been removed first; §5.2's `IngressError` sentence lists six
faults where the tree has seven). The fourth is D-10, and says outright that it
is the most consequential of the four because the other three outdate a
description while it reverses a decision — with the ground stated: D-10 declined
on scoping grounds, F-18 rests on R-3 being broken in shipping code, and a
failing invariant outranks a scoping preference. `design.md` is left as written
in both, per `docs/AGENTS.md:168`, and neither gets a Reconciliation row.

**5 — [[F-9]]'s instance.** Reconciled in `slice-004.md`, not deleted: the entry
now states the instance it used to name, says it no longer exists because
nothing connects to the live host at all, and keeps the underlying question —
the slot still holds one thing and ingress is still an unbounded author of it
from outside the process; what is left is the adversarial instance rather than
the operator's own. The three pre-slice refusal paths remain why it is deferred.

**6 — the test that could not be written before**, and it is deterministic
rather than a race.
`ingress::a_socket_a_forked_child_still_holds_is_reclaimed_and_the_new_listener_serves`
(`crates/goad-shell/tests/integration/ingress.rs`) binds a plain listener, hands
it to a child as **stdin** — `dup2` onto fd 0 clears `CLOEXEC`, so the fork
window that is microseconds wide in a real spawn is the child's whole life —
drops this process's last descriptor for it, asserts `connect` still succeeds
against a path no host holds, and requires `bind` to reclaim it and serve. A
second case,
`ingress::a_live_host_keeps_its_path_after_the_socket_file_is_removed`, holds
the half no `connect` probe could reach at all: with the socket file removed
there is nothing to connect to, so the old code bound a second host beside the
first, and the lock refuses it. Both were **confirmed red at `4467e0f`** in a
throwaway worktree — the first with the flake's own message, *"in use by a live
host"*, the second on its `panic!` — and both are green here.
`ingress::tests::the_lock_is_the_socket_s_own_path_with_a_suffix` pins the
lock's name beside them.

**7 — verification, measured against the baselines rather than asserted.**
`just check` at `95f0a97`, **exit 0**, 19 `test result: ok` blocks, zero
failures. The flake was then measured the way this finding measured it —
repeated sequential runs of the integration target, unloaded, which is the
condition it wants — and **against a control run on this machine rather than
against the numbers alone**:

| tree | runs | failures |
|---|---|---|
| repaired | **800** | **0** |
| `4467e0f`, pre-repair, same machine, same condition | 200 | **2** (runs 73 and 190) |

**The control is what makes the zero mean anything, and the control's rate is
what bounds the claim.** At 1 in 100, the rate it measured, 0 in 800 has about
a 0.03% chance of being luck. The first 200 runs alone would have had a 13%
chance, which is why they were not the whole measurement — *one green run* is
worth less again. The prior baselines this had to beat were 1 in 35 on the
repaired-at-the-time tree and 2 in 60 at `93abab3`; the control reproduces
both, so the window still fails when the code is still wrong, and 0 in 800 is
past all three.

None of that is the whole claim, and it does not need to be: **the mechanism is
gone.** Nothing connects to anything during a bind, so there is no listening
socket for a `fork` to keep alive behind the host's back, and the case that
used to lose that race is a deterministic test of the same condition now.

**8 — §6.1's bind race.** Rewritten to what is true, not softened: two hosts
starting in the same instant **cannot** both bind, because the lock is exclusive
and only one takes it — the check and the bind are still not one atomic step,
but the lock spans both. Stated as a property of the contract rather than as a
limit on it, with the second consequence beside it (one live host per socket
path) and the caveat that the exclusion is only as durable as the lock file's
inode. **The separate limit below it — *the path after the bind* — stands**, and
one clause of it was corrected rather than left false: what would close it is
the host re-probing its own path, not the single-instance enforcement this
contract now partly does own.

**9 — the *Single-instance enforcement* follow-up.** Reconciled in
`slice-004.md`, both faces named: the first is closed, as a consequence rather
than a goal, with the D-10 reversal pointed at `audit.md`; the second is
untouched and the entry says why the lock cannot reach it — it keeps a *second*
host off the path, and that residue is about the *first* host's own socket.

**11 — the cross-reference.** F-18's diagnosis is linked from every place the
closure is now stated: `draft-spec.md` §6.1 (as the measurement behind the
mechanism), `slice-004.md` under both the flake entry and *Single-instance
enforcement*, `audit.md`'s two new drift entries, and `reclaim`'s own doc
comment. It was correctly uncopied while it was an open limit; it is not one
now.

**Two judgement calls a reviewer should check rather than take.** First, a bind
against a path that is **empty** while another host holds the lock is now
`InUse` rather than a successful bind. That is the rule R-3 states — the lock,
not the file — and it is what closes the race, but it is behaviour no case
demanded before, so it has its own test. Second, `a_directory_with_no_write_permission_is_refused_naming_the_path`
now fails at the lock file rather than at the bind, so its fault is
`LivenessUnknown` rather than `Unbindable`; the case asserts the path and not
the variant, so it passes either way, and R-3's last clause and R-4 are the same
startup failure naming the same path. Both are stated in §7 rather than left to
be found.

**Two documents this touched that no item named**, both because the repair made
them wrong: `.gitignore` gains `/goad-demo.sock.lock`, and `notes.md`'s demo
walkthrough listed *a socket file left behind after you quit* under **what would
indicate failure** — which R-5 requires, so it was a standing contradiction and
is now the opposite sentence, naming both files.

**Outcome:** verified. I attacked the repair on the seven lines the handover
named and could not break it. What follows is what I checked rather than what I
was told.

**The lock holds what the probe could not, and for a reason narrower than the
one stated.** `flock` is associated with the **open file description**, so a
`fork` duplicates it and the lock survives until *every* descriptor closes
(`flock(2)`: *"the lock is released either by an explicit `LOCK_UN` … or when
all such file descriptors have been closed"*). That is the same inheritance
that broke `connect`. What makes it harmless here is a property the repair has
and the old code did not: **the lock is never released while a host lives.** The
old flake needed a *release* followed by a probe of the same path, and a
concurrent unrelated `fork` supplied the gap between them; under the repair the
reclaim case's path has never been locked by anyone when `bind` reaches it, so
there is no release to race. That is why the mechanism is gone rather than
narrowed — see [[F-19]] for the one place the *explanation* of this claims more
than the mechanism gives.

**`LivenessUnknown` is a real third answer.** R-3's *"Liveness MUST NOT be
assumed either way when it cannot be determined"* is the requirement it
discharges, and both alternatives are unsafe in different directions: assuming
live refuses to start forever on a path nobody holds, and assuming dead unlinks
a live host's socket. It changes the *message* and not the outcome — both are
startup failures — which is exactly what R-4 asks of a fault: name the path and
what was found.

**The red-first cases test the mechanism, not a proxy, and I did not need to
run them to know it.** `a_socket_a_forked_child_still_holds_…` opens with
`assert!(connect(&path).is_ok(), "the case says nothing unless the child keeps
the socket connectable")` — a control asserting *precisely the condition the old
code branched on*. Given that assertion passes, the old `reclaim` reaches
`InUse` deterministically and the case panics with the flake's own message; the
sibling, against an empty path, reaches `Ok` and panics on its own `panic!`.
Both are red by construction at `4467e0f`, which is stronger evidence than a
worktree run because it does not depend on the run. Holding the descriptor as
**stdin** to clear `CLOEXEC` is what turns the microsecond window into a
deterministic case, and that is the difference between a test of the mechanism
and a test of the weather.

**The bind race is closed, and closed for the stated reason.** The lock is taken
in `reclaim`, before `UnixListener::bind`, and released only at process exit —
so it spans probe *and* bind, which is the gap §6.1's old limit named. Rewriting
that limit as a property rather than deleting it is right. **The second limit is
correctly left standing** and its closing mechanism is correctly restated: *"what
would close it is the host re-probing its own path after the bind"* replaces the
old *"single-instance enforcement, which this contract does not own"*, which
would have been false once the contract owned a piece of it. That substitution is
easy to miss and it was not missed.

**Item 4, not contested.** A bind against an empty path while another host holds
the lock returning `InUse` is R-3 as rewritten, and it is strictly better than
what it replaces: under the old code, `rm`ing the socket under a running host
let a second host bind and left the first holding an unreachable descriptor —
two hosts, one path, neither told. The new answer refuses the second and says
why.

**The measurement is made properly, and its own caveat is the tell.** The
control at `4467e0f` — same machine, same condition, same case, same message —
is what makes 0 in 800 mean something; a bare 800 would not. Naming the first
200 as 13% luck is the instinct that separates a measurement from a number, and
so is *"the mechanism is gone"* outranking the count. I ran the integration
target **60 times sequentially at `33b45b1`: 0 failures** — reported as an
independent execution on a different day, **not** as corroboration of the rate,
because at the control's 1-in-100 a clean 60 is about 55% likely by luck and
says nothing on its own.

**The gate, run here rather than cited:** `just check` at `33b45b1`, exit 0, 19
`test result: ok` blocks, zero failures, all three new cases present and green.

**On the D-10 record — adequate, with one observation.** It is the strongest
entry in the four: it names what was decided, that this is a reversal and not an
outdating, the ground (a scoping preference against a measured invariant
failure), that single-instance enforcement is a *consequence* rather than the
goal, and where the follow-up owner will find the piece that already exists. The
observation: it closes by invoking *"a decision that no longer holds is
superseded rather than edited"*, which is `docs/AGENTS.md:27`'s rule **for
ADRs**, and D-10 is a `design.md` decision with no such vehicle — so nothing
formally supersedes it. That is not a gap in practice, because the mechanism,
its reasoning and its consequence are all in `draft-spec.md` R-3, R-5 and §6.1,
which is what a future agent reads; canon carries the decision even though
nothing carries the supersession. Worth knowing rather than worth fixing.

### F-19 — §6.1 says a dead host's children hold no lock; between `fork` and `exec` they do

**Severity:** minor
**Location:** `draft-spec.md` §6.1 (*"The lock beside the socket…"*);
`crates/goad-shell/src/ingress/mod.rs`, `reclaim`'s doc comment

**Expected:** [[F-18]] was raised on a standard it stated itself — *"What does
not depend on the rate: the inference is unsound, and today's narrowness holds
only because `reclaim` runs before `serve` … an accident of startup ordering
that nothing states as load-bearing."* [[F-15]] promoted the general form as a
criterion this spec now carries: a clause that cannot say when it does not hold
is a clause that has not been checked.

**Observed:** §6.1 explains why the lock has no fork gap, and the explanation is
absolute:

> The lock has no such gap: an inherited descriptor is `CLOEXEC` and is gone at
> `exec`, so a dead host's children hold no lock and a dead host is not live.

`reclaim`'s doc comment says the same: *"a dead host's children have all
exec'd. A dead host holds no lock."*

**`flock` is associated with the open file description**, not the descriptor,
so a `fork` duplicates it and *"the lock is released either by an explicit
`LOCK_UN` operation on any of these duplicate file descriptors, or when all such
file descriptors have been closed"* (`flock(2)`). A child between `fork` and
`exec` therefore **does** hold the lock. `CLOEXEC` ends it at `exec` and not
before, so a host that dies while a child of its own is inside that window
leaves the lock held until that child execs — and the next host to start reads
`InUse` about a host that is gone. That is the same shape of false *live* F-18
was raised about, arriving through the same mechanism the paragraph is in the
middle of explaining.

**This does not make the repair unsound and I am not asking for a code change.**
The lock is a defensible signal — a host with an un-exec'd child has not
finished existing — and the window is microseconds, self-clearing, and reachable
only at the instant of death, where the old probe's window recurred on every
fork for the whole of a process's life. The defect is that a paragraph whose
whole subject is *fork duplicating a descriptor* asserts an absolute that fork
duplicating a descriptor is the exception to.

**And the residue rests on something unstated, which is F-18's own objection.**
Whether the window is reachable at all depends on how the host spawns: glibc's
`posix_spawn` uses `CLONE_VM | CLONE_VFORK`, which suspends the parent until the
child execs, and Rust's `Command` takes that path only when the spawn's
configuration allows it. So the margin of safety here may be an implementation
detail of the standard library's spawn selection — *an accident that nothing
states as load-bearing*, which is the sentence F-18 wrote about the code it
replaced.

**Evidence:** `flock(2)` on lock ownership by open file description and release
on last close; `crates/goad-shell/src/ingress/mod.rs` (`hold` takes the lock via
`File::try_lock`; the `File` is `O_CLOEXEC` by Rust's default, which is the half
of the claim that is true); `draft-spec.md` §6.1 and `reclaim`'s doc comment for
the absolute. The fix is a clause, not a mechanism: say that the lock is held by
any un-exec'd child of the holder, that this bounds the exception to the instant
of a host's death, and that it is self-clearing — the same shape §6.1 already
uses for *upgrade skew* two paragraphs down.

**Disposition:** doc-wrong
**Response:** The documents are the defect and the code stays as it is. **This
finding is against my brief as much as against the repair** — I told the repair
agent that `CLOEXEC` closing the inherited lock descriptor at exec is what makes
a dead host hold no lock, and that is wrong: `flock` is held by the open file
description, so a `fork` duplicates it exactly as it duplicated the listening
socket. Inheritance is not the difference.

The real reason the repair works is narrower and better: **the lock is never
released while a host lives**, so there is no release for a probe to race. The
old flake needed a release followed by a probe of the same path with an
unrelated fork supplying the gap; under the repair the reclaim case's path has
never been locked when `bind` reaches it. Fix §6.1 and `reclaim`'s doc comment to
say that, and add the clause the finding asks for — the lock is held by any
un-exec'd child of the holder, which bounds the exception to the instant of a
host's death and is self-clearing. Use the shape §6.1 already uses for upgrade
skew two paragraphs down.

Do **not** repair it by asserting `posix_spawn`'s `CLONE_VFORK` closes the
window. The finding is right that this would rest the margin on the standard
library's spawn selection, which is exactly the unstated accident F-18 was raised
about. Naming the residue is the repair; removing it is not available here.

**Outcome:**

### F-20 — the socket's filesystem must support advisory locking, and nothing says so

**Severity:** minor
**Location:** `draft-spec.md` §6.1; `crates/goad-shell/src/ingress/mod.rs`
(`hold`)

**Expected:** §6.1 is where the contract states what the socket's location must
provide. It already carries one such statement — *"The containing directory is
the user's responsibility"* — for the permission property the mode cannot reach.

**Observed:** R-3 now **requires** the lock: *"The host MUST decide whether a
live host holds the path by taking an exclusive lock on a sidecar file beside
it."* So the filesystem holding `ingress.path` must support `flock(2)`. Most
do; some do not — various FUSE mounts, and NFS depending on version and mount
options. On one that does not, `try_lock` returns `TryLockError::Error`,
`reclaim` yields `LivenessUnknown`, and **the host does not start at all**,
where before this change it started and served.

That behaviour is correct — R-3's *"Liveness MUST NOT be assumed either way when
it cannot be determined"* requires exactly it — and the message names the path
and what happened. What is missing is that **nothing tells the person
configuring `ingress.path` that this is a property their chosen location must
have**, so the failure is met as a host that will not start with a message about
a lock they did not know existed, and the recovery — put the socket somewhere
else — is not derivable from it.

This is the weaker of the two I am raising and I would not argue hard for it: a
new environmental requirement became load-bearing in this repair, and §6.1 is
the paragraph that exists to state environmental requirements. One sentence
beside the directory-permissions one closes it.

**Evidence:** `draft-spec.md` R-3's lock clause and §6.1, neither of which
mentions filesystem support; `crates/goad-shell/src/ingress/mod.rs`'s `hold`,
where `TryLockError::Error` becomes `LivenessUnknown`; `BindFault`'s own
`Display` — *"whether a live host holds it could not be determined: {inner}"* —
which reports the errno faithfully and still leaves the reader without the
remedy.

**Disposition:** doc-wrong
**Response:** Agreed, and the reviewer is right that it is the weaker of the two —
but the ground is not strength, it is that R-3 now **requires** the lock, so
`flock` support became an environmental requirement of `ingress.path` in this
repair, and §6.1 is the paragraph whose job is to state environmental
requirements. It already carries the sibling sentence about the containing
directory. One sentence beside it.

The behaviour is correct and does not change: a filesystem that cannot lock
yields `LivenessUnknown` and the host does not start, which is what R-3's
*"MUST NOT be assumed either way"* requires. What is missing is only that a
person choosing a location is never told the location must support locking, so
the failure arrives as a host that will not start over a lock they did not know
existed, with the remedy — put the socket somewhere else — underivable from the
message. See [[F-21]]: same paragraph, and both are about a person meeting a
lock they never asked for.

**Outcome:**

### F-21 — an unusable socket location is now reported as an undetermined liveness

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:177-188` (`hold`);
`draft-spec.md` §7's R-4 row

**Expected:** R-4 — *"any other failure to bind … MUST be a startup failure
naming the path **and what was found**."* R-3's new last clause is a different
case: *"Liveness MUST NOT be assumed either way **when it cannot be
determined**."* The two are different questions and `BindFault` has a variant
for each.

**Observed:** `hold` collapses them. The `open` of the lock file and the
`try_lock` on it both map to `LivenessUnknown`:

```rust
.open(lock_path(path))
.map_err(|error| fault(path, BindFault::LivenessUnknown(error)))?;
match lock.try_lock() {
  ...
  Err(std::fs::TryLockError::Error(error)) => Err(fault(path, BindFault::LivenessUnknown(error))),
}
```

Because `reclaim` now calls `hold` **before** `UnixListener::bind`, every
failure that used to surface at the bind surfaces here first — and the most
likely misconfiguration of all is one of them. A typo'd or missing directory:
`symlink_metadata` returns `NotFound`, so `occupant` is `None` and the code
proceeds to `hold`, whose `open` gets `ENOENT`. The person reads

> whether a live host holds it could not be determined: No such file or
> directory

where before this repair they read *"could not be bound: No such file or
directory"*. A read-only directory reads the same way with `EACCES`. **The
fault's name asserts a liveness question and the errno reports a filesystem
one**; the errno is the true half, and it is the half R-4 asks be named. A
person is pointed at a lock they did not know existed instead of at the
directory they mistyped.

The variant is right for what it was made for — `try_lock` failing on a
filesystem that cannot lock is genuinely *liveness could not be determined*,
which is [[F-20]]'s case. It is wrong for *the location is unusable*, which is
R-4's.

**And §7's R-4 row now asserts the collapse as intended**: *"A directory the
host cannot write is refused at the **lock file** now rather than at the bind —
R-3's last clause and R-4 are the same startup failure naming the same path,
and the case asserts the path rather than the variant."* They are the same
startup **failure** and not the same **message**, and R-4's requirement is
about the message. The test is honest — it never asserted a variant and its
name promises only the path — but the row uses that honesty to argue the
distinction away rather than to notice it had moved.

The fix is a split, not a new concept: an `open` failure means the path is
unusable and belongs with R-4's faults; a `try_lock` failure means liveness
could not be determined and is R-3's. `hold` already has the two errors in
separate arms.

**Evidence:** `crates/goad-shell/src/ingress/mod.rs:177-188`; `reclaim` calling
`hold` before `bind`, and reaching it for a `NotFound` path because `occupant`
is `None` rather than an error; `BindFault::LivenessUnknown`'s own `Display`
(*"whether a live host holds it could not be determined: {inner}"*) against
`Unbindable`'s (*"could not be bound: {inner}"*); `draft-spec.md` §7's R-4 row
for the argument that the two are the same. The case
`a_directory_with_no_write_permission_is_refused_naming_the_path` passes either
way, which is why nothing caught the move.

**Disposition:** fix-now
**Response:** The regression is real and it is the one thing in this slice that
degrades what a person sees. A mistyped or missing directory is the commonest
misconfiguration there is, and it now reads *"whether a live host holds it could
not be determined: No such file or directory"* where it read *"could not be
bound: No such file or directory"*. The fault's name asserts a liveness question
while the errno reports a filesystem one, and R-4's requirement is about the
message.

**Fix by splitting, not by adding a concept** — `hold` already has the two errors
in separate arms. The lock file's `open` failing means *the location is
unusable* and must surface as a location fault naming what was wrong with the
path; `try_lock` failing means liveness is genuinely undetermined, which is what
`LivenessUnknown` was made for and which [[F-20]] is about. Keep
`LivenessUnknown` for the second only.

**Two things go with it, and the second is the reason this was not caught:**

- §7's R-4 row currently argues the distinction away — *"R-3's last clause and
  R-4 are the same startup failure naming the same path, and the case asserts
  the path rather than the variant."* They are the same failure and not the same
  message. Correct the row.
- **The case asserts the path and not the variant, which is exactly why the fault
  could move under it and stay green.** Strengthen it to assert the variant. A
  test honest about what it checks is still a test that let a user-facing
  regression through, and the fix for that is the assertion, not the honesty.

**Outcome:**

## Handover

<!-- Written 2026-09-09 while the reviewer still held the whole review in view,
     deliberately before it was needed. If this session ends, a successor takes
     the review from here without reading the conversation that produced it. -->

The reviewer for rounds 1-4 is one agent, kept across rounds on purpose. This
section is what a successor needs and cannot get from the findings alone.

### Where every finding stands

**Closed — outcome `verified`, repair landed and re-reviewed: F-1 … F-18.**
Nothing is owed on any of them. Three carry conditions a successor should still
check at promotion or close, listed under *Standing conditions* below: [[F-5]],
[[F-15]], [[F-9]].

**Open — dispositioned at `b762db5`, repair in flight, outcome not set:**

| id | disposition | what round 4 must check |
|---|---|---|
| F-19 | doc-wrong | the clause in **all three** sites — `draft-spec.md` §6.1, `reclaim`'s doc comment, and `notes.md`'s Harvest entry. The third is the one bound for `docs/memory/`, so it is the copy that outlives the slice. **No site may close the window by invoking `posix_spawn`'s `CLONE_VFORK`** — that would replace one unstated accident with another, which is F-18's own objection |
| F-20 | doc-wrong | one sentence in §6.1 beside the directory-permissions one. Nothing else should move |
| F-21 | fix-now | the split in `hold` — an `open` failure is R-4's *the path is unusable*, a `try_lock` failure is R-3's *liveness could not be determined*; §7's R-4 row corrected with it; and the test's assertion strengthened without its name coming to disagree with what it asserts |

### Standing conditions the reviewer is holding

Not written anywhere else, and lost if this section is not read.

1. **[[F-15]]'s criterion must survive promotion as a criterion.** §6.3's closing
   sentence — *"A clause of this paragraph that says a refusal always reaches a
   person is making a claim about who decides it… One that cannot say which side
   decides is a clause that has not been checked"* — reads as commentary and is
   therefore the sentence most likely to be trimmed when `draft-spec.md` becomes
   `SPEC-003`. `audit.md`'s promotion row instructs that it be kept. **If it is
   trimmed to a row note, that is a finding and not a remark**, and the raiser
   committed to raising it as one. It is what makes the next instance of that
   class findable by reading rather than by a reviewer tracing a path, and F-15's
   own sweep found two instances with it.
2. **[[F-9]]'s Follow-ups entry must keep its shape at close.** One entry, three
   instances, shared cause stated; the second-host-startup instance stated and
   then *retired with the reason* rather than deleted. It has already been
   reconciled once ([[F-18]] removed the instance it named). A close pass that
   tidies it into a one-line follow-up loses the reason it was deferred, which is
   the only thing stopping it being deferred again on worse grounds.
3. **[[F-5]]'s boundary must not be re-strengthened.** R-14's Verification row
   claims *renamed*, *removed* and the mapping by assertion, and *added* by
   compile gate plus review. That is exact and was arrived at by measurement. A
   future editor tempted to simplify it back to *"a reason added or renamed fails
   here"* would be restoring the overclaim F-5 was raised about.
4. **[[F-2]]'s budget is stated in elapsed time on purpose.** Five seconds, in
   the docstring. A change to a retry *count* would silently un-state it.

### Lines of attack — spent, and not

**Spent, and a successor should not re-run them.** The permissive/canonical
boundary through `envelope.rs`, clause by clause against R-9 and R-10. Domain
vocabulary, including paraphrase, over `src/` and module and type names. Strata
direction; stratum 1's manifest and its dependency graph (`cargo tree -p
goad-semantics -i tokio` matches nothing). Failure handling on the whole ingress
path: partial reads, both budgets, dropped connections, a writer that never
writes, a `oneshot` whose receiver is gone, concurrent connections, the accept
task's own death. The socket's lifecycle end to end — bind, reclaim, mode,
symlink, the absence of an unlink, and now the lock. The reply's wire form. Both
anchors and the three AC-6 cases. Every author of the diagnostics surface. The
example backend. And the spec-versus-code axis, heavily — that is where most of
this ledger came from.

**Not spent. In the order this reviewer would take them:**

1. **Walk all sixteen §7 Verification rows against the tests they name, asking
   of each: does this test hold what this row claims?** This is the highest-value
   stone left and the reason is in the ledger's own record: of the four rows this
   review happened to check, **three were overclaiming** — R-14 ([[F-5]]), R-11
   ([[F-6]]) and R-4 ([[F-21]]). None was found by looking for that; each was
   found by chasing something else. Twelve rows have never been read against
   their tests.
2. **Whether five seconds is the right `ACCEPT_FAULT_BUDGET`.** [[F-2]]'s
   condition was that the bound be stated in elapsed time, and it is. Whether the
   value is long enough for an `EMFILE` burst under a backend storm was never
   asked, and spending the budget is irreversible for the life of the process.
3. **`glass.rs`, for whether a person actually sees the ingress-stopped report.**
   [[F-3]]'s repair puts it in a frame; this reviewer reasoned that `Surface`
   stays `Hidden` when nothing is shown and that the tray goes to `Fault`, and
   stopped there rather than reading `present`.
4. **`take_timestamp`'s permissiveness** (`envelope.rs`). `jiff::Timestamp`
   accepts more than RFC 3339 — an RFC 9557 zone annotation, `-00:00` as an
   offset, arguably a date alone. Considered and dropped as consistent with
   permissive-in; a successor may reasonably disagree, and R-10 says *"MUST be an
   RFC 3339 instant"*.
5. **Two hosts and one writer, interleaved**, beyond what the cases cover.
6. **`just demo`, run.** The human observation covers it and the runbook was
   read; this reviewer never ran it.

### Observations dropped rather than raised

Recorded so they are not rediscovered as new. Each was considered and judged not
to be a finding; a successor is free to disagree, but should know it was seen.

- **The arm race in
  `ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`.**
  It depends on `shutdown_background` having dropped the accept task before the
  inner arm first polls, and that drop is deferred. Measured **12/12 stable**;
  the race runs the safe way, because load lengthens the subprocess spawn and so
  favours the inner arm, and a loss fails the case rather than passing it
  wrongly. Not a finding this reviewer could confirm. **If it ever flakes, this
  is the explanation, and it belongs beside the four flakes `slice-004.md`
  already records.**
- **`reddit-watcher` and `reddit-opened` in `src/`.** The vocabulary scan covers
  `crates/goad-shell/src/ingress/envelope.rs`'s test fixtures and passes because
  `reddit` is not on the `DOMAIN` list. Judged not a breach: the invariant is
  about host **types and module names**, these are fixture *values*, and they
  come from brief §19's own scenario.
- **The TOCTOU between `bind` and `set_permissions` on the socket.** A path
  swapped for a symlink in that window would have `0600` applied to the target.
  Dropped because §6.1 puts the containing directory in the user's
  responsibility in terms, which is the same ground A-5's mode window rests on.
- **`detail` can carry ~64 KiB of watcher-chosen key name onto the wire.**
  Bounded by `ENVELOPE_LIMIT`, escaped and truncated before it reaches a person
  (`Escaped`, `bound`). Harmless.
- **`escaped()` in `examples/shell/backend.sh` cannot escape control
  characters.** Unreachable: the host serialises `source` and `kind` through
  `serde_json`, so a control character arrives at the script already escaped as
  two ASCII characters.
- **Nothing bounds what the host writes to the backend's stdin.** A 64 KiB
  envelope becomes a 64 KiB `evaluate`. Judged SPEC-001's, not this contract's.

### What this reviewer would say differently, asked at the end

Severity is set at raise time and none of these reopens a finding. They are the
things a continuous view shows and a fresh one would not.

- **[[F-4]] was `minor` by consequence and load-bearing by position.** Its
  repair — reading the token off `Refusal::reason()` — is what forced
  `UnavailableCause` to become the right shape, which [[F-7]] and then [[F-18]]
  both needed. A finding whose severity reflects its blast radius can still be
  the one whose repair unblocks the others; the ledger records the severity and
  not that.
- **[[F-9]] is the one whose severity gets weaker the longer it is deferred.**
  `minor` was right on the ground that the surface's single-slot design predates
  this slice. That ground does not survive a second deferral: each slice that
  leaves an outside-the-process author writing to a slot the host needs for its
  own faults makes *"it predates us"* a weaker sentence.
- **The class this review closed on instances is *"an absolute clause about a
  mechanism that cannot name the mechanism's exception."*** [[F-3]], [[F-15]]
  and [[F-19]] are all it. The criterion promoted from F-15 is scoped to *which
  refusals a person sees* — which is why F-15's own sweep did not reach §6.1's
  lock paragraph, and [[F-19]] had to be raised separately one round later. The
  criterion is right and narrower than the class. **The general form is worth
  stating somewhere it applies to the whole document**, not only to §6.3.

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

**Round 2 is complete; the ledger stays open** on three new findings
([[F-14]], [[F-15]], [[F-16]]), all `minor`, none blocking. **All thirteen
round-1 repairs land**, each checked against the finding it answers and the
condition its outcome attached, and each new case checked for whether it would
have gone red before its own repair — the ten that were added all would have.
The gate was re-run here rather than cited: `just check` at `441fa94`, **exit
0**, 19 `test result: ok` blocks, zero failures.

Two of the three new findings are the same shape and it is worth naming: **a
repair that fixed its instance and stopped short of its class.** [[F-14]] is
[[F-12]]'s own rule, written into a comment in the file and then not applied to
the branch one line above it — a watcher can still choose which prompt the demo
shows, by making it show none. [[F-15]] is [[F-7]]'s new cause filed on the
wrong side of the very distinction [[F-3]] was raised to enforce. [[F-16]] is an
amendment a Response promised and the pass did not make. None of the three
questions a repair that was made; all three are about where a repair stopped.

**How [[F-5]]'s condition resolved, since its outcome is terminal and reads as
though it did not.** That outcome said: build the match as the *source of the
compared set*, not as a *ward beside a hand-written array*, or it would be
contested. What landed is the source by construction — every member of `reasons`
comes off an arm, and there is no second path into the set — and it still shows
the ward's symptom, because the arms are reached through witnesses and Rust
cannot force a witness to exist. The repairer measured that rather than
asserting past it, and the ledger and `draft-spec.md` R-14 now agree on the
width of the leak: the compile gate is forced, a **correct** update is not. I
re-derived the same boundary independently, including that the deferred `Reason`
type would narrow the leak rather than close it, since a hand-written `ALL`
stays hand-written. So the condition was answered by weakening the verification
claim while leaving the requirement intact — the alternative F-5 itself named —
and there is nothing left to contest. The `Reason` follow-up's declared
surfaces were checked against the code: `Refusal::reason()` has exactly one
production caller outside its module (`folded()`), two in the integration tier,
and one in `ingress/mod.rs`'s own `#[cfg(test)]` block; the renderer tier names
`Refusal` in a doc comment only and reads reasons off the reply JSON, so the
entry's claim that it is untouched holds.

Two gaps the repairs declared rather than papered over — no test drives a real
`accept()` error ([[F-2]]), and the read-fault mapping is unit-only ([[F-7]]) —
were checked and are **honest**, not findings in disguise. `EMFILE` needs a
process-global `setrlimit` against parallel cases in one process, and a peer
closing an AF_UNIX stream gives EOF rather than an error, so neither is
reachable from a cooperating test without `libc`. In both, the rule is held
where it can be — as a pure function, at the one site that states it — and the
untested remainder is named. A third candidate was dropped rather than raised:
`ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`
looked like it might race on which arm observes the closed channel, but the race
runs the safe way (load lengthens the exchange and so favours the inner arm) and
12 consecutive runs were stable, so it is an observation and not a finding.

**Round 1's raise and disposition passes are both complete.** All thirteen outcomes are
`verified` and **nothing is contested** — including [[F-9]]'s `follow-up`, which
was checked against the guardrail rather than waved through, and [[F-5]]'s
proposed instrument, which holds under a stated condition and not otherwise.
Four outcomes carry conditions the re-review will check, and one ([[F-4]])
warns that the repair *shape* named in its Response would be worse than the
defect if taken literally. Three repairs — [[F-4]], [[F-7]] and §6.3's
`unavailable` row — are one piece of work and will disagree about how many
causes `unavailable` has if they are done apart.

Thirteen findings: three `major`, nine `minor`, one `nit`. **No `blocker`** —
nothing found makes the slice unsafe to run or breaks the invariant that a
failure never takes the host down, and the ingress path's core is sound: the envelope's normalization is a
genuine single door, nothing past it is unvalidated, no host code reads `kind`
or `data` or judges `timestamp`, the two anchors are independent in all three
directions with falsifying tests behind them, and the eight-token reason
vocabulary is one exhaustive match with the compiler behind it.

What the review did not find is worth stating, because these were the lines of
attack most likely to produce a `blocker`. **Nothing narrowed the wire contract
to fit the renderer**: the envelope admits any JSON `data`, and no field is
constrained by what the loop happens to consume. **No domain vocabulary entered
host types or module names**, and the paraphrase check comes back clean too —
`reddit-watcher` appears only as a fixture value drawn from brief §19, never as
a type, a module, or a branch. **Strata still run one way**: stratum 1's only
change is one `pub(crate)` → `pub`, and `cargo tree -p goad-semantics -i tokio` errors
*"did not match any packages"* — run here, not taken from the audit. **The malformed-input paths are safe**: `serde_json`'s own
recursion limit bounds the duplicate-key walk, attacker-chosen key names reach
the diagnostics surface only through `Escaped` and `bound`, and the reply is
built with `serde_json` rather than interpolated so a watcher-chosen key cannot
break the reply's own syntax.

The three `major`s are all one shape — **the contract about to become canon says
something the code does not do**, in a place no test discriminates: the reply's
framing (F-1), the ingress-stopped report's reach (F-3), and, at `minor`, the
opacity claim for `data` (F-6). In two of the three the *code* is defensible and
the *spec sentence* is the likely defect; F-2 is the one that is a plain code
defect with no document on either side of it.

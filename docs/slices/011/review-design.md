# Review — design — Slice 011

**Subject:** design — `docs/slices/011/design.md` and `docs/slices/011/canon-delta.md`
at `d57f503`
**Reviewer:** fresh agent, Opus
**Opened:** 2026-09-26
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

**Round 1** — 2026-09-26 — the design and canon delta at `d57f503`, checked
against the tree (`serve`, `ingest`, `dispatch`, `refuse_arrival`,
`ingress_stopped`, `Controller::refuse`, `Glass::present`, `accept_loop` /
`handle`, `Ingress::arrival`, `crates/goad/tests/renderer/ingress.rs`,
`tests/support/waiting.rs`) and against tokio 1.53.1's timer.

What this round attacks:

1. **Control flow.** Every exit from the new `'idle` loop, what each
   `continue` / `break` reaches once a second loop wraps them, and whether
   `refusal_owed` and `next_refusal_present` behave across every transition:
   command, scheduled firing, accepted arrival, refusal-site refusal,
   ingress death, stop.
2. **"Carries it."** Whether each path the design says carries an owed refusal
   to the surface actually shows *that refusal*, given what `Controller::refuse`
   and `absorb` do to the retained `Diagnostics`.
3. **Starvation and yield.** Whether the `biased` ordering argument is about a
   reachable state, and whether it protects the window or only a property
   write.
4. **Canon wording.** Whether R-15 as amended says exactly what the code will
   do — no more, no less — under P-D (every absolute names its exception), and
   whether the sections the delta marks "checked, not changed" (§6.3, §5, §6.4)
   are still true. Counts carried into canon.
5. **Tests.** For every named mutation, trace whether the named assertion goes
   red; look for assertions that hold a proxy, bounds tested only by a liveness
   backstop, vacuous passes, and timed margins whose failure direction is the
   gate's (the user runs other cargo builds on this box).
6. **AC coverage** from `slice-011.md`, and simpler designs missed.

Invariants held: the five in `CLAUDE.md`; SPEC-003/R-8, R-12, R-15, P-C, P-D;
SPEC-002/R-4, R-12 and ADR-004 (one write site per anchor); ADR-001 (stratum 3
only); name-never-count.

**Round 2** — 2026-09-26 — the repairs at `361d5a3`: each F-1…F-14 repair
checked against the tree rather than its Response, then the repairs attacked
as new surface. The rewritten R-15 and the new §6.3 entry are checked for
exactly-what-the-code-does, P-D (including which side decides each new
exception) and counts. `surface_stale` is traced across every transition,
including the drain. `RecordingGlass` is checked for what it actually observes.
Each re-timed case gets a trace of every named mutation and a load direction
for each bound. The new VT-7 assertion and the AC-4 and AC-7 edits are checked
against the tree.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | doc-wrong | verified |
| F-2 | major | doc-wrong | verified |
| F-3 | major | doc-wrong | verified |
| F-4 | minor | doc-wrong | verified |
| F-5 | minor | doc-wrong | verified |
| F-6 | minor | doc-wrong | verified |
| F-7 | minor | doc-wrong | verified |
| F-8 | minor | doc-wrong | verified |
| F-9 | minor | doc-wrong | verified |
| F-10 | minor | doc-wrong | verified |
| F-11 | minor | doc-wrong | verified |
| F-12 | minor | doc-wrong | verified |
| F-13 | nit | doc-wrong | verified |
| F-14 | nit | doc-wrong | verified |
| F-15 | minor | doc-wrong | |
| F-16 | minor | doc-wrong | |
| F-17 | minor | doc-wrong | |
| F-18 | minor | doc-wrong | |
| F-19 | nit | doc-wrong | |
| F-20 | nit | doc-wrong | |

### F-1 — A refusal-site refusal or the ingress-stopped fold does not carry an owed refusal: it erases it

**Severity:** major
**Location:** `design.md` §5.4 (*OQ-5 cases*, first bullet), §5.1 state
diagram (`Waiting --> Top: … refusal-site refusal, ingress stopped`),
`slice-011.md` OQ-5

**Expected:** §5.4: "a command, a refusal-site refusal or the ingress-stopped
fold goes to the top, which presents the owed refusal with it."
**Observed:** Both of those paths call `Controller::refuse` before reaching the
top — the refusal site in `serve` (`Err(refused) => { controller.refuse(&refused); … continue; }`)
and the outer ingress arm (`controller.refuse(&ingress_stopped()); continue;`).
`Controller::refuse` is `self.diagnostics = Diagnostics::refused(refused)`, and
`Diagnostics::refused` builds a one-line value. The owed ingress refusal is
replaced before the top presents, so it never reaches any frame. Today it would
have been presented first, at once. This is a new way a refusal decided while
idle is never shown, and neither the design (I-2, §5.4) nor the amended R-15
names it. Paths that *do* carry it, verified: a diagnostics command or
successful `Edit` (`dispatch` → `None`, diagnostics untouched), and any exchange
start (the engage present precedes `absorb`).
**Evidence:** `Controller::refuse` and `Diagnostics::refused`
(`crates/goad/src/diagnostics.rs`); the refusal site and the outer
`ingress.arrival()` arm in `serve`. Reachable while owed: a stale `Choose`
(`SupersededView`), a refused `Edit`, a scheduled firing with an unreadable
clock (`NoClock`), ingress dying.

**Disposition:** `doc-wrong` — responder: orchestrator. User decided 2026-09-26 (`design-log.md`): the owed state is *the surface is stale*, not *this refusal is owed*; an overwritten refusal is not shown, and retention stays with FU-3.
**Response:** Per the user's decision, the owed state is now *the surface is stale*. `refusal_owed` is renamed `surface_stale`. design.md §5.4 (*While the surface is stale*) now says that a refusal-site refusal or the ingress-stopped fold overwrites the stale fold unshown, while a diagnostics command, an edit or the engage present shows it. The same change is made in the §5.1 diagram, I-2, D8, §6 OQ-5, R4 (FU-3) and slice-011.md OQ-5.

**Outcome:** verified — design.md §5.4 *While the surface is stale* now splits the paths correctly: command / `Edit` / engage present show the stale fold; refusal site and ingress-stopped fold overwrite it (checked against `Controller::refuse` and the drain, which calls no `refuse`). Diagram, I-2, D8, OQ-5 agree.

### F-2 — Amended R-15 puts a per-refusal MUST the design cannot meet, and its exceptions are not all named (P-D)

**Severity:** major
**Location:** `canon-delta.md` Change 1 (R-15's amended cell); `design.md` I-2

**Expected:** P-D: an absolute clause names its exception and bounds it. The
cell should state what the code guarantees.
**Observed:** The first sentence binds every refusal: it "MUST reach that surface
**within a fixed interval** of being decided, unless the host's loop ends
first". Under the design, a refusal superseded inside its interval never
reaches the surface at all:
- by a later ingress refusal — the flood, which is the case the slice exists
  for; only the last refusal before each present is ever shown;
- by any `Controller::refuse` (F-1).

The cell's third sentence ("shown when that interval ends — as the latest
refusal decided by then") silently contradicts the first rather than being its
named exception. The loop ending is the only exception named. I-2 ("A refused
arrival decided while idle reaches the window … within `I`, unless the loop
ends first") is false in the same way.

The second MUST ("updates … MUST NOT exceed one per interval") has an
exception the design itself records and the cell does not: §5.5 *Edge — clock
overflow on the reset* falls back to one update per refusal. Unreachable in
practice, but P-D asks for it to be named or the clause restated.

What the code actually guarantees is a statement about **updates**: while
idle, an update of the surface follows every refusal within the interval
(unless the loop ends), and each update shows what the diagnostics then hold.
**Evidence:** the design's own T2(c) asserts only the *last* key; §5.4
*Refusal inside an interval* ("that present shows … the latest fold"); F-1.

**Disposition:** `doc-wrong` — responder: orchestrator. User decided 2026-09-26 (`design-log.md`): R-15 is worded around an update of the surface within the interval showing the latest refusal, with the overwrite named as its exception (P-D); the clock-overflow fallback named too.
**Response:** canon-delta Change 1 rewrites R-15 around the update guarantee: the surface is updated within the interval and shows the latest refusal decided by then. It names the overwrite and loop-end exceptions, and separately the clock-overflow exception to the one-per-interval limit. design.md I-2 is restated the same way.

**Outcome:** verified — Change 1 now binds an *update* within the interval showing the latest refusal, names overwrite and loop end, and separately the clock-overflow exception to the per-interval limit. I-2 matches. New issues in the rewritten text are raised as F-16 and F-17.

### F-3 — SPEC-003 §6.3 is marked "checked, not changed", but two of its statements become false

**Severity:** major
**Location:** `canon-delta.md` *Checked, and not changed* (§6.3, §5);
`design.md` §10

**Expected:** Every statement in §6.3 still holds after the amendment, as the
delta asserts.
**Observed:** §6.3, *Which refusals a person sees*:
- the second bullet says of `too_soon` and the clock-unreadable `unavailable`,
  "for these two *always* is exact". After this slice neither always reaches the
  surface: one decided inside an interval is dropped if the loop ends first (D8,
  which the new R-15 names), and one superseded inside its interval is never
  shown (F-2). The new exception is decided by a different side (the host's
  stop, or a later refusal), which is precisely what §6.3's P-D criterion says
  a clause must account for.
- the first bullet says shape refusals "reach the surface when it happened to
  be idle". Under a flood, all but one per interval do not.

The delta's reason ("'Reaches' in §6.3 now carries R-15's bound") does not
address either. §6.3 needs an entry in `canon-delta.md`.
**Evidence:** SPEC-003 §6.3's bullets, read against the amended R-15 and D8.

**Disposition:** `doc-wrong` — responder: orchestrator. §6.3 gains a `canon-delta.md` entry.
**Response:** New canon-delta Change 2 covers SPEC-003 §6.3. It rewrites the lead-in and the first two bullets on R-15's terms (overwrite, loop end), and it replaces the unavailable paragraph's cause counts.

**Outcome:** verified — Change 2 quotes the §6.3 lead-in, both bullets and the `unavailable` paragraph exactly as they stand in SPEC-003, and the replacements are true under Change 1.

### F-4 — T2: M2 does not turn (b) red; only (c) catches it

**Severity:** minor
**Location:** `design.md` §9, T2 row (M2: "(b) and (c) go red")

**Expected:** Under M2 (owed set only when the deadline has already passed),
(b) goes red.
**Observed:** Traced: under a sustained flood, the first refusal after each `F`
finds the deadline passed, sets owed, and the arm presents at once and re-arms.
So M2 presents at about `t0`, `t0+I`, `t0+2I` — the same cadence as the design
during the flood. (a) and (b) stay green. Only the trailing edge differs: the
flood's last refusal is almost never the first after an `F`, so (c) goes red.
M2 is caught, but by one assertion, not two. A Response that inherits "(b)
and (c)" will write a false claim about which assertion holds the trailing
edge into the plan.
**Evidence:** the mutation as written in §9, traced through the arm's
precondition and reset.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md §9 T2 row: M2 (stale set only when `F` has passed) is now claimed to turn red on (c) only.

**Outcome:** verified — T2 row claims M2 → (c) only.

### F-5 — The trailing edge after a flood is held only by `LIVENESS_BOUND`, not by the interval

**Severity:** minor
**Location:** `design.md` §9 T2(c); `slice-011.md` AC-3; `canon-delta.md`
Change 2 (the flood case's bullet)

**Expected:** AC-3: the last refusal reaches the window "within the stated
bound". R-15 as amended: shown "when that interval ends".
**Observed:** (c) waits `LIVENESS_BOUND` (5 s, `tests/support/waiting.rs`) after
the flood ends — five intervals. (b) bounds the gaps *during* the flood only,
and T3 bounds only the leading edge. So an implementation whose trailing present
lands anywhere up to 5 s after the last refusal passes every case. Memory
`a-bound-is-not-tested-at-the-bound`: the outcome (the last key is eventually
shown) is what a correct and a late implementation agree on. The fix is cheap:
bound (c) at `I` + slack from the last reply, with `LIVENESS_BOUND` kept only as
the backstop.
**Evidence:** T2(c)'s text; `LIVENESS_BOUND` in `tests/support/waiting.rs`.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md §9 T2(c) is now bounded at `at − last reply ≤ 2I`, which is `I` + slack `I`, measured from the recording glass. `LIVENESS_BOUND` is no longer the bound. The canon-delta Change 3 flood bullet says the same.

**Outcome:** verified — (c) is now `at − last reply ≤ 2I`, read from the log; the trailing present lands within `I` of the last decision, so margin `I`.

### F-6 — T3's controls M5 and M6 go red only under timing the design does not pin

**Severity:** minor
**Location:** `design.md` §9 T3 row and T3 bullet

**Expected:** M5 and M6 each turn T3 red.
**Observed:**
- **M5** (`next_refusal_present` starts at `started + I`) changes anything only
  if the first refusal is decided within `I` of `started`. T3 first runs an
  accepted envelope's exchange (a subprocess), waits for its `absorb`, then
  sends the refusal. Nothing keeps that under 1 s; on a loaded box it is not,
  and M5 is then green. The leading edge's real property — re-arming — is held
  by the second refusal, so M5 may be the wrong control.
- **M6** (reset to `now + 2 s`) reds only if the quiet gap `g` before the
  second refusal satisfies `2 s − g > I/2`, i.e. `g < 1.5 s`. The design
  gives only a lower bound ("after more than `I` of quiet") and a spacing
  ceiling (3 s). An implementer choosing `g = 2 s` — safely inside the spacing —
  makes M6 green. Sleep overshoot under load pushes `g` upward, the same way.

R2's `const _: () = assert!` ties the gap to the spacing only. It should also
tie `g` below `I + I/2`, or M6 should be restated.
**Evidence:** T3's text in §9; the arithmetic above.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md §9 T3 is re-pinned so that neither control depends on how long the priming exchange takes. M5 is now trailing-edge-only (`F = now + I` when stale is first set), which turns R1 red deterministically. R2 is a numbered shape refusal sent at R1's present + 1.25·I, so it has no spacing dependency. M6 is now `now + 3I`, which leaves `F` about 1.75 s off at R2.

**Outcome:** verified — M5 (trailing-edge only) reds R1 with no timing precondition; R2 no longer depends on the spacing; M6 at `3I` leaves `F` ≈ 1.75 s off at R2. M8 likewise deterministic.

### F-7 — T4 can pass without an owed refusal ever existing

**Severity:** minor
**Location:** `design.md` §9 T4 bullet

**Expected:** T4 holds "a command during a coalesced interval … carries the
refusal".
**Observed:** Nothing asserts that B was **owed** — not yet on the window — when
the command is sent. If B is decided more than `I` after A's present (a stall
between the two connections), B gets its own leading-edge present and T4 passes
without testing that the command carries anything. Separately, M7 is caught only
by the "within `I/2` of the send" clause, and only while the send lands within
`I/2` of A's present. The "before A's present plus `I`" clause compares two poller
observations and cannot reliably tell M7 apart (both land about `I` after A,
each with up to one poll interval of lag). Suggest a precondition assertion that
the window still shows A's line at the send, which also makes M7's red
unconditional.
**Evidence:** T4's text; `POLL_INTERVAL` in `tests/support/waiting.rs`.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md §9 T4 now has a precondition, read from the recording glass when the command is sent: the last present shows A and not B, and `sent − A.at < I/2`. M7's red therefore rests on `at − sent ≤ I/2`, where the trailing present would land about `I` after A.

**Outcome:** verified — the precondition makes the owed state explicit. A residual margin issue at the precondition's edge is raised as F-19.

### F-8 — The new timed upper bounds fail in the gate's direction under load, and R1 records only the benign direction

**Severity:** minor
**Location:** `design.md` §8 R1; §9 T2(a), T2(b), T3, T4

**Expected:** Memory `margin-size-is-not-margin-direction`: for every timed
bound, write down which way load moves it. A load-sensitive gate is a defect.
**Observed:** R1 says load "cannot make them pass wrongly". True, but it is the
failure direction that costs the gate. Every new bound is an upper bound that
load pushes toward red:
- T2(b)'s gap ≤ `I + I/2`;
- T3's and T4's "within `I/2`".

Each is observed by a 5 ms poller that shares a current-thread runtime with
`serve` and the accept task, so the poller's own scheduling lag adds to the
measured gap. T2(a)'s ceiling `1 + ceil(span / I)` takes `span` from the
**writer's** reply times, on the blocking pool. Load can shrink that span
relative to the serve-side span the arm actually runs on, for example when the
first reply is read late. That lowers the ceiling below a correct count near an
integer boundary. Suggest `CountingGlass` record `(Instant, diagnostic line)`
at each present, serve-side, and measure gaps and the span from those. That
removes the poller's lag and the writer's clock from every bound, and leaves
only `serve`'s own stalls. Also state the direction per bound, beside the
constant.
**Evidence:** `within` / `POLL_INTERVAL` (`tests/support/waiting.rs`); T2's
bound as written; memories `timed-test-margins-are-measured-at-the-bound`,
`margin-size-is-not-margin-direction`.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md D12 and §9 test support: `CountingGlass` becomes `RecordingGlass`, which logs `(Instant, Surface, lines)` inside `present`. Every gap and span is measured from that log, so there is no poller and span no longer comes from the writer. Each bound's load direction and margin is in §9's *load →* column. R1 is rewritten: lower bounds are safe under load; upper bounds have ≥ `I/2` of margin and are measured at the bound.

**Outcome:** verified — serve-side recording removes the poller and the writer-side span; every bound carries a load direction. One direction label is wrong (F-18).

### F-9 — The starvation argument defends an unreachable state, and misses what actually gives the UI thread back

**Severity:** minor
**Location:** `design.md` §3 (*`biased` starvation*), D4, R4; `canon-delta.md`
Change 2, *Review, not a test*, first bullet

**Expected:** The review-only canon row says what holds and why.
**Observed:**
1. **The premise cannot happen.** "A writer that keeps arrivals always
   ready" is excluded by the accept side's own shape. `bind` makes a
   `mpsc::channel(1)`, and `handle` awaits the `Answer` and writes the reply
   before `accept_loop` accepts the next connection (SPEC-003 §6.4, *One
   connection at a time*). After `serve` answers, the next arrival does not
   exist until the accept task has replied, accepted, read and normalized
   again, and `serve` re-polls in microseconds. The ingress arm is therefore
   Pending on the poll after each refusal, whatever the writer does. R4 half
   says this ("`serve` turns an arrival around faster than the accept loop
   does") but then presents it as an obstacle to testing rather than as the
   reason.
2. **Protecting the arm would not protect the window.** Suppose the premise
   were reachable. Nothing in `'idle` ever returns `Pending`, and under Slint's
   executor there is no tokio coop budget to force a yield. The UI thread would
   never get back to Slint's event loop, which renders the present and handles
   input. The arm's position would guarantee property writes, not a visible
   window. The canon bullet "cannot starve it" is true only of the write.

What returns the UI thread — the slice's actual purpose (`research.md` Thread 3,
the tray-latency probe) — is that `serve` yields once per arrival, because of
point 1. Nothing states it, and nothing tests it (AC-6 is the only witness).
Keeping the arm above ingress is still right, as cheap defence. The row
should give the structural reason instead of the counterfactual.
**Evidence:** `bind`, `accept_loop`, `handle` (`crates/goad-shell/src/ingress/mod.rs`);
SPEC-003 §6.4.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md §2 now states the real mechanism: `bind`'s `mpsc::channel(1)` and the sequential `accept_loop` mean `serve` yields once per arrival, and the UI thread was lost to the present's cost, not to starvation. D4 now justifies the arm's position on ordering alone: when both are ready, the owed update goes first, so I-2 rests on the timer and not on goad-shell's channel shape. R3 records the yield per arrival as untested (AC-6 is its witness). canon-delta Change 3's review bullets are rewritten to match.

**Outcome:** verified — §2 states the channel(1)/sequential-accept yield, D4 rests on ordering alone, R3 and the canon review bullets state the untested yield honestly.

### F-10 — R3's control for the ingress-stopped `continue` reds for a different reason than stated, and can go green under load

**Severity:** minor
**Location:** `design.md` §8 R3

**Expected:** R3: if the ingress-stopped `continue` ends up targeting `'idle`,
`a_dead_accept_task_…` goes red "because that fold would then never be
presented".
**Observed:** Under that mutation the fold is still presented: `serve`'s
initial arm fires at `MINIMUM_SPACING` (3 s), and the engage present carries it.
That is inside assertion 1's `until(LIVENESS_BOUND, …)` (5 s), so assertion 1
passes. The case then reds only at assertion 2 (anti-spin), because the
exchange's `absorb` present lands inside `ANTI_SPIN_WINDOW` (500 ms), with a
message that blames a spinning arm. If that exchange takes longer than 500 ms
(a slow subprocess spawn under load), assertion 2 holds, assertion 3 holds, and
the mutation survives. The control needs its own observable: the fold on the
window **before** the initial arm could fire, for example within `I` of the
fold, or well under `MINIMUM_SPACING`.
**Evidence:** `a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
(assertions 1–3, `ANTI_SPIN_WINDOW`); `LIVENESS_BOUND`; `serve`'s initial
`sleep`.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md R2 and the §9 VT-7 row: VT-7 gains one assertion, that the first present showing 'ingress has stopped' is recorded less than `MINIMUM_SPACING/2` after `serve` is spawned. The mutation (the fold's `continue` retargeted to `'idle`) is then shown only by the engage present at `MINIMUM_SPACING`, so it turns red deterministically. slice-011.md AC-4 is restated to permit this.

**Outcome:** verified — under the mutation the fold first appears at the engage present at `MINIMUM_SPACING`; the new `< MINIMUM_SPACING/2` assertion reds that deterministically. AC-4 restated to permit it.

### F-11 — `refusal_owed`'s scope is stated ambiguously, and one reading loses every owed present

**Severity:** minor
**Location:** `design.md` §5.2 (*`refusal_owed`*), §5.3, D7

**Expected:** One unambiguous scope.
**Observed:** "Local to the `'idle` loop and `false` on entry to it", "one pass
of `'idle`", "a pass of `'idle` always starts with nothing owed". Read as one
*iteration*, the flag is declared inside the loop body. Then `continue 'idle`
re-initialises it to `false` before the next `select!`, and the arm is never
enabled. The intended scope, one entry into `'idle` across all its iterations,
means declaring it immediately before `'idle: loop`. The tests would catch the
wrong reading. The design should not leave it to them.
**Evidence:** the three phrasings cited.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md §5.2 now specifies `let mut surface_stale = false;` declared immediately before `'idle: loop`, outside its body, once per entry into the idle wait and surviving that wait's iterations. §5.3 and D7 are aligned with it.

**Outcome:** verified — §5.2 pins the declaration immediately before `'idle: loop`.

### F-12 — D5's "interval runs from refusal-caused presents only" is canon, but no named control holds it

**Severity:** minor
**Location:** `design.md` D5, §9; `canon-delta.md` Change 1 ("when no **such**
update has been made within the last interval")

**Expected:** The rejected alternative D5 names (the interval runs from any
present) has a mutation that turns a case red. The canon states the
distinction.
**Observed:** No M-number covers it. Traced: resetting `F` at every top present
reds T3 only if T3's first refusal is decided within `I/2` of the accepted
exchange's `absorb` present. That coverage is incidental and depends on
timing, like M5 (F-6). The fix is either to name it as a control with a pinned
precondition, or to add one step to T4: a refusal decided shortly after the
command's present is still shown at once when `F` has passed.
**Evidence:** D5; T3's sequence.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md §9 T3 adds control M8 (`F` also reset at every top present). R2 follows a diagnostics command's present after `F` has passed, so M8 turns it red. D5 cites it. The canon-delta Change 3 T3 bullet states the rule, and 'No ADR' cites M3 and M8.

**Outcome:** verified — M8 (reset `F` at every top present) moves `F` at the command's present, so R2 waits ≈ `I`; red regardless of how long the priming exchange takes.

### F-13 — "Presents at once, with no await that parks" is stronger than A-1's evidence

**Severity:** nit
**Location:** `design.md` §5.4 (*Refusal after quiet*), A-1, I-1

**Expected:** A-1 supports §5.4's claim.
**Observed:** tokio 1.53.1 `Wheel::insert` fires on registration only when
`when <= self.elapsed`. That is the wheel's last *processed* time, not now. The
initial `sleep_until(started)` is never polled before the first refusal, so it
can be Pending on its first poll: `serve` parks for one driver turn (~1 ms)
before the leading-edge present. This is harmless, and `I/2` absorbs it, but
"no await that parks" is not what the timer guarantees. A-1's second clause
should say "once the driver has advanced past it". Separately, I-1 holds for arm
firings. Presents follow each one after the drain, so their spacing is `I` less
the drain's duration.
**Evidence:** `Wheel::insert` and `Handle::reregister` in tokio 1.53.1
(`src/runtime/time/wheel/mod.rs`, `src/runtime/time/mod.rs`); memory
`tokio-time-runs-under-slints-executor` measured a deadline 10 s past, not one
near the wheel's current tick.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** design.md A-1 now says a past deadline fires once the driver has advanced past it (`Wheel::insert`), so the leading edge may wait one driver turn. §5.4 now says 'no later than the timer driver's next turn'. I-1 is restated for arm firings, with presents at least `I` minus one drain apart. The canon wording says 'without waiting for the interval' in place of 'at once'.

**Outcome:** verified — A-1, §5.4 and I-1 now say what tokio 1.53.1's wheel does.

### F-14 — Counts and unchecked sections carried through the delta; FU-2's kill condition not literally met

**Severity:** nit
**Location:** `canon-delta.md` Change 3, *Checked, and not changed*;
`slice-011.md` AC-7

**Observed:**
- Change 3 keeps the rest of SPEC-003 R-12's cell verbatim, including "The
  anchor's independence in **three** directions". §6.3, which the delta checks,
  says "Which of the **eight**". `CLAUDE.md` says name-never-count binds canon.
  The delta touches both passages, so this is the cheap moment to fix them, or
  to say why each is a closed set.
- §6.4 is where SPEC-003 lists host constants. The delta should say in one
  line why the interval is not there (it is invisible to a writer).
- FU-2's *Dead when* reads "a refused arrival no longer costs a present". A lone
  refusal after quiet still costs one, by design, so the strike at close must
  restate the kill condition rather than claim it as written.

**Disposition:** `doc-wrong` — responder: orchestrator, under the autonomy grant. Repaired in `design.md`/`canon-delta.md`.
**Response:** canon-delta states the count rule: 'eight' is exempt because `the_reason_token_set_is_closed_at_eight` holds it. R-12's 'three directions' is replaced (Change 4.2), and §6.3's cause counts are replaced (Change 2.4). Change 1's *Why not §6.4* says why the interval is not listed there. design.md §10 and slice-011.md AC-7 note that FU-2's *Dead when* is restated at close.

**Outcome:** verified — "eight" is exempt as a closed set held by `the_reason_token_set_is_closed_at_eight` (R-14's row); "three directions" and §6.3's cause counts are replaced; *Why not §6.4* added; AC-7 restates FU-2's *Dead when*. The repair itself introduced new counts (F-16).

### F-15 — `RecordingGlass` records the frame it is handed, not the window, while the canon row says every case "observes the window"

**Severity:** minor
**Location:** `design.md` D12, §9 *Test support* ("appends `Presented { at, surface, lines }` inside `present`, before it delegates"); `canon-delta.md` Change 3's lead ("Every case below observes the **window** … the lines it wrote")

**Expected:** AC-3 requires the refusal to reach "the **window** — not only the
retained model". The R-15 row claims each case observes the window.
**Observed:** A record taken inside `present` *before* delegating can only read
the `Frame`. That is `frame.surface` and `frame.diagnostics`, and
`Frame::diagnostics` borrows the retained `Diagnostics`. The record therefore
proves *that* a present happened and *what the retained model held* at that
instant. That closes FU-2's gap, since a suppressed present now shows. But it is
not what `SlintGlass::present` wrote. A regression in the write (the
diagnostics `write_if_changed`, or `set_mode`) would leave every timed case
green. The case VT-7 already reads, `window.get_diagnostic_lines()`, is one
call away. `RecordingGlass` can hold a handle to the window it wraps, delegate
first, then record `window.get_diagnostic_lines()` and `window.get_mode()`.
The alternative is to word the canon cell as what it is: "the frame each
update was given". Memories: `a-refusal-is-recorded-not-shown`,
`a-green-test-can-assert-a-proxy`.
**Evidence:** `Frame` (`controller.rs`: `pub surface`, `pub diagnostics: &'a Diagnostics`); `SlintGlass::present` (`glass.rs`, `set_mode`, diagnostics via `write_if_changed`); the VT-7 comment "the window's own `diagnostic_lines`, written unconditionally by `glass.rs`".

**Disposition:** `doc-wrong` — responder: orchestrator. Take the stronger branch: `RecordingGlass` records what the window holds **after** delegating, so every case reads the window as AC-3 and the canon row say.
**Response:** I took the stronger branch. design.md D12 and §9 *Test support*: `RecordingGlass` now holds the window it wraps, delegates to `SlintGlass::present` first, and then records `(at, mode, lines)` from `window.get_mode()` and `window.get_diagnostic_lines()`. §9 adds a read-the-window control, M0 (drop `write_if_changed`): T2, T3 and T4 must go red. The lead of canon-delta Change 3 now says the glass delegates first and then records the window's own mode and lines.

**Outcome:**

### F-16 — The repair puts new counts into canon and the design

**Severity:** minor
**Location:** `canon-delta.md` Change 1 (R-15 cell: "**Two** exceptions, both the host's"; "**The one** exception is the host's own arithmetic"), Change 2.3 ("R-15's **pair** of exceptions"); `design.md` §10 ("**three** named exceptions"), I-3 ("**Two** changes only")

**Expected:** The delta's own count rule: "Every count in a passage this delta
touches is either replaced or justified as exempt" (`CLAUDE.md`, name never
count). An exception list is not a closed set a test holds. It is exactly the
kind of list P-D expects to grow when a clause is checked again, which is what
round 1 did.
**Observed:** Each of these counts appears in text the repair wrote, and none
is justified. §6.3's "R-15's pair of exceptions" is a count in one spec
section of a list that lives in another, and nothing re-reads it when R-15
gains an exception. Name the exceptions instead ("R-15's overwrite and
loop-end exceptions"), or name the rule they share.
**Evidence:** the quoted phrases; `canon-delta.md` *The count rule*.

**Disposition:** `doc-wrong` — responder: orchestrator. Name the exceptions, never count them, in canon and design alike.
**Response:** Counts replaced by names. canon-delta Change 1: 'Two exceptions, both the host's' becomes 'Its exceptions are the host's own', and 'The one exception' becomes 'That limit yields to the host's own arithmetic'. Change 2.3: 'R-15's pair of exceptions' becomes 'R-15's overwrite and loop-end exceptions'. design.md §10 names the overwrite, loop-end and clock-overflow exceptions, and I-3 now reads 'Only the `Fired::Ingested`/`None` path and the new arm change'.

**Outcome:**

### F-17 — The delta says two different things about which side decides the new exceptions, and "both the host's" conflicts with §6.3's own grouping

**Severity:** minor
**Location:** `canon-delta.md` Change 1 *Why* ("The host decides all three") and cell ("Two exceptions, both the host's"); Change 2 *Why* ("The new exceptions come from a different side: a later refusal, or the host's stop")

**Expected:** P-D's criterion: an absolute clause answers *which side decides
the thing it is absolute about*, and answers it once.
**Observed:** Change 1 attributes the overwrite to the host, while Change 2
attributes it to "a later refusal". §6.3 groups refusals by deciding side, and
under that grouping the later refusal that most often overwrites is a shape
refusal. §6.3 puts those under "decided by whatever accepts connections", paced
by the writer, not the host. There is a defensible reading in which the host
decides: the overwrite happens because the host retains one fold (FU-3), and
the host decides that retention. But the cell must say that, not "both the
host's" alongside a *Why* that says "a different side". As written, the
criterion P-D requires is answered two ways within one delta.
**Evidence:** the three quoted sentences; SPEC-003 §6.3's first bullet.

**Disposition:** `doc-wrong` — responder: orchestrator. The overwrite is the host's: what the surface retains is the host's choice (FU-3), whatever side the overwriting refusal blames. Change 2 is brought into line with Change 1.
**Response:** The overwrite is the host's. The R-15 cell in canon-delta Change 1 now says an overwritten refusal is never shown 'whichever side decided the later refusal, because the surface holds what the host chooses to retain'. Change 1's *Why* explains that the overwrite belongs to the host even when a shape refusal decided by the accept side does the overwriting. Change 2's *Why* now matches: 'the host's, as Change 1 states: its retention … and its stop'.

**Outcome:**

### F-18 — T2(a)'s load direction is labelled "safe", but a stall can shrink the gap it bounds, and the canon row states a stronger bound than I-1

**Severity:** minor
**Location:** `design.md` §9 T2 row ((a) "≥ `I − 50 ms`", *load →* "(a) safe"); `canon-delta.md` Change 3 flood bullet ("Consecutive updates are never closer than the interval")

**Expected:** The label says which way load moves each bound. The canon cell
states what I-1 holds.
**Observed:** The arm resets `F` from `now` in its own body. The present
it causes, and so the log's `at`, comes later: after `continue 'serving`, the
drain, and `controller.frame`. Call that delay `d1`. The next logged gap is
`I + (arm latency) + d2 − d1`. A preemption inside `d1` of the *earlier*
present shortens the following gap one-for-one. At the oversubscription the
user runs, a 50 ms descheduling is not exotic. So load moves (a) toward red,
not away from it. The chance is small, but the "safe" label is the claim R1
relies on. Separately, the canon cell says updates are "never closer than the
interval", while I-1 says "at least `I` minus one drain" and the test asserts
`I − 50 ms`. Canon should state I-1's form. The test can either stamp the arm's
own instant (a second log, written by the arm) or keep `I − 50 ms` and label
its direction honestly.
**Evidence:** §5.2's arm (`reset; continue 'serving;`); I-1's own wording; the top of `serve`'s loop (drain, then `glass.present(controller.frame(…))`).

**Disposition:** `doc-wrong` — responder: orchestrator. T2(a)'s load label corrected; the canon row states the guarantee I-1 actually holds, no stronger.
**Response:** design.md §9 T2(a) now asserts gaps of at least `I/2`, and is labelled as moving toward red under load: a stall between an arm firing and its present shortens the next gap one-for-one, with a margin of `I/2`. M1 still gives µs gaps. I-1 is restated per arm firing, with each present following after the drain. The flood bullet in canon-delta Change 3 now claims only what I-1 holds: at most one firing per interval, each update following its firing after the loop's turn, and an assertion that updates are no closer than half the interval.

**Outcome:**

### F-19 — T4's precondition admits a send so late that M7's red margin reaches zero

**Severity:** nit
**Location:** `design.md` §9 T4 row

**Observed:** Under M7 the first present after `sent` is the trailing one, at
about `A.at + I`. Its `at − sent` is therefore about `I − (sent − A.at)`. The
precondition admits `sent − A.at` up to `I/2`, so at the edge M7 lands at about
`I/2`, which is the bound itself. `d1` and arm latency then decide the colour.
In the normal run `sent − A.at` is a few milliseconds and the margin is about
`I/2`. The indeterminacy exists only after a stall. Tightening the
precondition to `< I/4` makes M7's red margin at least `I/4` without changing
the passing case.
**Evidence:** T4's precondition and bound, traced under M7.

**Disposition:** `doc-wrong` — responder: orchestrator. Tighten to I/4.
**Response:** design.md §9 T4: the precondition is tightened to `sent − A.at < I/4`. Under M7, `at − sent ≥ 3I/4`, which gives a red margin of at least `I/4`.

**Outcome:**

### F-20 — Stale figures and one stale sentence left by the repair

**Severity:** nit
**Location:** `design.md` §1 ("up to 8 s") and §2 ("~330 µs"); `slice-011.md` *Scope* ("so the latest refusal is always shown") and OQ-1 ("up to 8 s")

**Observed:** `research.md` was re-run at `c57b670`. It now gives ~318 µs per
refusal with the form up, and tray activations of 6.8–8.8 s during a flood.
"Up to 8 s" is now below the measured range, not an upper bound. "The latest
refusal is always shown" is what F-1/F-2 decided is not promised: an overwritten
refusal is never shown.
**Evidence:** `research.md` at `c57b670` (the probe table: 8.8 s, 6.8 s, 8.3 s); `design-log.md` F-1/F-2 entry.

**Disposition:** `doc-wrong` — responder: orchestrator. Figures from c57b670; Scope restated to the update guarantee.
**Response:** design.md §1 now uses the figures from `c57b670`: ~318 µs against 3.6 µs, and tray activations of 6.8–8.8 s. In slice-011.md, OQ-1 uses the same figures, and the *Scope* sentence is restated to the update guarantee, with overwritten refusals never shown. The same edit tightens design.md from 352 to 278 lines, with the mechanism unchanged and no decision, invariant, test or mutation dropped.

**Outcome:**

## Round 1 — what is sound

Checked against the tree and found complete:

- **§2 Current state** is accurate: select order, which `ingest` steps return
  `None`, every path to the top present, the inner loop's two branches,
  `Frame` borrowing the retained `Diagnostics`, and the test tier's
  current-thread runtime with the accept task on `serve`'s thread.
- **The `'idle` shape (D2) holds I-3.** The top present stays unconditional,
  and the only new ways out of `'idle` are the new arm (to the top) and the
  refused-arrival `continue 'idle`. Every other path is untouched. Unlabelled
  `break Ending::…` would fail to compile once `'idle` yields a tuple, so R3's
  only live hazard is the one `continue`.
- **Anchors (I-4, D1).** The new arm builds no `Fired`. `refusal_re_arms`,
  `floor_until`, `event_floor_until` and the schedule's `sleep` are never
  reached from it, so each keeps its one write site.
- **The bound arithmetic in §5.4** (by `r + I`, arm firings at least `I` apart,
  throttle not debounce) is correct. The arm cannot starve `commands` or `sleep`
  (D4's second half).
- **D6** (the ingress-stopped fold and refusal-site refusals are not coalesced)
  is sound. Its VT-7 reasoning is right.
- **A-2** is verified (`EnvelopeFault::Unknown` displays "unknown key `{key}`").
  A-1's "fires even if unpolled after a reset" is verified (`Sleep::reset`
  re-registers).
- **Controls M1, M3, M4 and M7** go red as stated (M7 via the `I/2` clause).
- **Change 4 (SPEC-002)** is exactly a citation swap. The words attached to the
  case are already R-12's half only. **Change 3** drops the headless figure
  rather than carrying it, which is right.
- **Vocabulary** ("update of the surface", not "presentation") and R-12's
  envelopes-vs-updates distinction in Change 1 are sound. Nothing in the delta
  counts refusal kinds.
- **No simpler design found.** A trailing edge needs a timer arm. Folding it
  into `sleep` breaks R-4's one write site, and a skip-flag breaks I-3. The
  nested loop is the smallest shape that keeps the top present unconditional.

## Round 2 — what is sound

Every F-1…F-14 repair is verified above. It was checked against the tree and
the current SPEC-003 text, not against the Response.

**What the repairs made no worse.** Traced, not taken on trust:

- **`surface_stale` across transitions.** Command, `Edit`, scheduled firing
  (clock readable or not), accepted arrival, refusal site, ingress death,
  stop and closed channel all do what §5.4 says. The drain calls no
  `Controller::refuse`: a refused `Edit` or `Choose` comes back as
  `Some(Err)` and is folded at the refusal site *after* the top present. So
  nothing overwrites a fold between `continue 'serving` and the present.
- **T2's controls** M1 → (a), M3 → (b), M4 → (b) and M2 → (c) all go red as
  claimed. (b)'s `2I` and (c)'s `2I` each carry a margin of about `I`.
- **T3's controls** M5, M6 and M8 go red independent of the priming exchange.
  The command-then-R2 order is safe because `commands` is above ingress in the
  `biased` `select!`.
- **VT-7's new assertion** reds the retargeted `continue` deterministically:
  3 s against 1.5 s. The only other way a fold reaches that window is the
  initial arm.
- **§6.3 Change 2** quotes SPEC-003 exactly. The "eight" exemption is real:
  R-14's row names the test that holds it.
- **D4's new rationale, and the review bullets in Change 3,** are
  structurally honest.
- **AC-4's and AC-7's edits** permit exactly VT-7's mechanical switch plus one
  assertion, and restate FU-2's kill condition. Nothing else in AC-4 is
  loosened.

No new major or blocker. Round 2's new findings are about how exactly the
canon cell and the test instrument are worded and labelled (F-15…F-18), plus
two nits. None of them reopens the mechanism.

## Synthesis

<!-- Written when the ledger resolves. -->

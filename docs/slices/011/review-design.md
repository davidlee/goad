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

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | | |
| F-2 | major | | |
| F-3 | major | | |
| F-4 | minor | | |
| F-5 | minor | | |
| F-6 | minor | | |
| F-7 | minor | | |
| F-8 | minor | | |
| F-9 | minor | | |
| F-10 | minor | | |
| F-11 | minor | | |
| F-12 | minor | | |
| F-13 | nit | | |
| F-14 | nit | | |

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

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

## Synthesis

<!-- Written when the ledger resolves. -->

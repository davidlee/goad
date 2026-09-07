# Research — Slice 003

**Producers:** one scoping agent, 2026-09-07, threads 1–5 in a single session.
**As of:** 2026-09-07 · `56bf622` (branch `slice-002`; working tree clean but
for `flake.lock` and this untracked folder)

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

**Baseline.** `just check` exits 0 at `56bf622`. Warm wall time **5.3 s**
(`real 0m5.298s`), which matches ADR-003's recorded 5.276 s on the finished
slice-002 tree. The gate is the six commands POL-001 §Compliance lists, and
`just -n check` prints them in that order.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- unmarked — researcher claim: cited, not checked.

Design and plan may only load-bear ✓ rows, or rows they verify at point of use.
Verify what you lean on, not everything.

Marked ✓ by the **design** agent, 2026-09-07, at the sites it load-bears. Two
researcher claims were found inaccurate while checking and are corrected in
place, marked ✗→✓: `schedule::resolve`'s call-site count (Thread 2, Fact 1) and
the forbidden-token list (Thread 1). Rows still unmarked were not leaned on by
the design.

## Citation forms

Canon claims cite the document id (`SPEC-001 §4`, `ADR-003`). Code claims cite
`path:line`. An uncited claim is unverifiable by definition.

---

## Thread 1 — governing canon

### Binding

**SPEC-001, the host/backend interaction protocol.** ✓ §2 puts *"the timer that
decides when to evaluate"* explicitly **out of scope**, and then names the seam
this slice sits on: *"The timer abuts it at the resolved next-check instant — it
consumes one and calls `evaluate`."* So the spec does not tell the timer how to
behave; it tells the timer what it is handed and what it may do with it. Six
requirements constrain the value that crosses that seam, and they bind this
slice at one remove:

| id | what it obliges of a timer |
|---|---|
| R-26 ✓ | The next check resolves to the latest **valid** instruction, else the retained instant **if still ahead of the current one**, else `now + default_poll`. *"A resolved instant at or before the current one has fired: it is consumed, not carried."* Resolution is `schedule::resolve`'s, and it has already run before the timer sees anything. |
| R-27 ✓ | The resolved next check is **always a concrete instant**. There is no unresolved state, so the timer never has to represent "no schedule". |
| R-28 ✓ | A `next_check` in the past **MUST be stored as given**. The host must not adjust a backend's instruction to a value it prefers — which forecloses clamping a past instruction forward to protect the timer's own cadence. |
| R-29 ✓ | A failed exchange **MUST NOT accept a new instruction**; it resolves as if none arrived, and *"what it reports is what it retains"*. |
| R-21 ✓ | The span grammar is *"the host's one duration grammar: anything else in the host that reads a duration reads it with the same grammar and the same refusals."* Any new configured duration this slice introduces is bound by that sentence. |
| R-45 ✓ | No backend failure may terminate the host, and none may leave it unable to invoke the backend again. A timer that stops after a failure breaks this. |

SPEC-001 §5 *"A broken backend is polled on its existing cadence"* is the
paragraph closest to this slice's subject, and it already states the intended
behaviour in prose, including its rationale: *"reporting an already-elapsed
instant would have the timer retry a backend that failed at its scheduled check
in a tight loop."*

**ADR-001 — one-way strata.** ✓ Stratum 1 (`crates/goad-semantics`) is pure: no
clock, no runtime. `schedule::resolve` already lives there and takes `now` as a
parameter (`crates/goad-semantics/src/schedule.rs:221-236`). A timer is I/O
against real time and therefore belongs to stratum 2 or 3, never stratum 1.

**ADR-003 — the workspace of strata.** Four members. Whichever member gains the
timer gains it under the manifest allowlist and the purity scan; a new
dependency in `crates/goad-semantics` is `error[E0433]` at the crate edge, and a
`std::time::Instant` reach inside stratum 1 is caught by the purity scan
(`crates/goad-boundary/tests/checks/purity.rs:17-27`). Note that
✓ `std::time::Duration` is deliberately **not** forbidden there — *"a duration is
a quantity, not a clock"* — so a duration may be named in stratum 1, and is
(`ScheduleConfig::default_poll` is a `jiff::SignedDuration`).

**POL-001 — the phase gate.** Six commands, no feature matrix. Two clauses bear
on this slice:

1. *"the residue"* — a feature switched on in a dependency **shared with stratum
   1** unifies into stratum 1's build under `--workspace`, and no gate command
   rejects it. It is a review obligation, argued in the slice that takes it.
   Stratum 1's dependencies are `jiff`, `serde`, `serde_json`
   (`crates/goad-semantics/Cargo.toml`); `tokio` is not among them, so a new
   **tokio** feature does not touch the residue. A new **jiff** feature would.
2. No command may be removed, weakened, or made conditional to get a phase
   green — which includes making a timing-sensitive test conditional or
   `#[ignore]`d.

**`CLAUDE.md` invariant 1** — no domain vocabulary. ✗→✓ The forbidden token
list is **seven** tokens, not five: `habit`, `streak`, `journal`, `site`,
`goal`, `reminder`, `compliance`
(`crates/goad-boundary/tests/checks/vocabulary.rs:17-26`). `schedule`, `poll`,
`timer`, `tick`, `check` are host vocabulary and pass. This is the standing
criterion every slice carries.

### Checked, not applicable

- **ADR-002 (single crate until triggered)** — superseded by ADR-003. Its T1 and
  T2 triggers are spent or unfired; T2 (a second binary) is slice 004's.
- **SPEC-001 §6.4 (process transport)** — unchanged by this slice. The timer
  calls `Host::evaluate`, which calls the transport; nothing about the transport
  moves.
- **SPEC-001 R-30…R-35 (interaction identity)** — a scheduled evaluation is an
  ordinary `evaluate`, so identity behaviour is `Host`'s and already settled.
  The one place it becomes live again is persistence: see OQ-3 below.

### Amendment candidates

- **SPEC-001 §2 Scope.** *"the timer that decides when to evaluate"* is out of
  scope and *"persistence of anything across host restarts"* is out of scope.
  Both are stated as facts about what the spec covers, and both stay true if the
  timer's behaviour is written down somewhere else. If the slice decides the
  timer's rules **are** protocol-adjacent enough to be normative — the
  no-re-resolution rule in particular — the choice is a new spec (`SPEC-002`,
  the host's scheduling behaviour) rather than an edit to SPEC-001's scope line.
  A `draft-spec.md` in this folder is the mechanism `docs/AGENTS.md` prescribes.
- **SPEC-001 OQ-3** — *"Whether a stale `view_id` survives a host restart. R-32's
  rejection is scoped to one process lifetime while nothing persists."* It
  reopens **only if** this slice persists schedule state. `docs/roadmap.md`
  §Open decisions names this as the decision that closes or defers OQ-3.
- **`docs/policy/001-the-phase-gate.md`** — no change expected unless the slice
  adds a gate command (it should not) or a jiff feature (see the residue above).

---

## Thread 2 — code map

### Hotspots

| file | why it is in scope |
|---|---|
| `crates/goad/src/controller.rs` | `serve` is the loop (`:298-397`). A timer is a third `select!` arm, or it is not in the loop at all. `absorb` currently drops `next_check` at `:142`. |
| `crates/goad/src/reception.rs` | `Received::next_check` (`:42`) is the value the renderer already receives on every outcome and does nothing with. |
| `crates/goad/src/clock.rs` | the wall clock, one function wide, `pub type Clock = fn() -> Result<Timestamp, ClockError>` (`:16`). Explicitly *"Not a timer: slice 003 owns the schedule."* |
| `crates/goad-shell/src/host.rs` | owns the resolved check inside `State` and reports it on every `Outcome`, but exposes **no accessor** — see Cited fact 4. |
| `crates/goad-shell/src/state.rs` | `resolved_check` and `resolve_to` (`:52`, `:60`), already `pub` on `State`, which `Host` holds privately. |
| `crates/goad-shell/src/config.rs` | `ScheduleConfig::default_poll` (`:72-73`), parsed at load through the shared duration grammar (`:155-167`). Any new configured duration lands here. |
| `crates/goad/src/wire.rs` | `Command` (`:23-35`) and `Cancel` (`:141-176`). Whether a timer produces a `Command` or is its own `select!` arm is decided against these. |
| `crates/goad/src/main.rs` | `start` composes runtime, components, wire, glass and `serve` (`:48-116`). Any new argument to `serve` is threaded here. |
| `crates/goad/Cargo.toml` | tokio features (`rt-multi-thread`, `sync` on top of the workspace set). A test-only `test-util` would go in `[dev-dependencies]`. |

### Cited facts

**1. Where `next_check` is resolved.** ✗→✓ `goad_semantics::schedule::resolve`
has **two** call sites in the tree, not one, and both are in `host.rs`:
`Host::new`'s seeding call (`crates/goad-shell/src/host.rs:128`) and
`Host::resolve_from` (`:259`). The researcher's "exactly one function, called
from exactly two sites" describes `resolve_from` alone and omits the seed. Any
scan written for AC-6 must admit both. `Host::resolve_from`
(`crates/goad-shell/src/host.rs:258-265`)
wraps `goad_semantics::schedule::resolve`; the accept path calls it with the
incoming instruction (`:228`) and the failure path calls it with `None`
(`:289`). Both then write the result into state with `State::resolve_to`
(`:229`, `:290`), so *what is reported is what is retained*. There is no third
call site.

`schedule::resolve` itself (`crates/goad-semantics/src/schedule.rs:221-236`) is
three arms:

```rust
match (incoming, retained) {
  (Some(instruction), _) => instruction,
  (None, Some(pending)) if pending.instant() > now.instant() => pending,
  (None, _) => Timestamp::new(now.instant().checked_add(default_poll)
                              .unwrap_or(jiff::Timestamp::MAX)),
}
```

**2. ✓ What `Host` returns after an exchange.** `Outcome`
(`crates/goad-shell/src/host.rs:70-96`): `view`, `next_check` (`:76`, always
concrete, *"including failure"*), `discarded`, `stderr`, `failure`, `cleanup`.
`Outcome` is not `Clone`.

**3. ✓ What the renderer does with it today: nothing.**
`reception::receive` carries `next_check` through into `Received` (`:42`,
`:86`), and `Controller::absorb` destructures it into `_` at
`crates/goad/src/controller.rs:140-143` with the comment *"Resolved on every
outcome, but nothing in this slice retains a schedule — slice 003 fills that
seam"*. Nothing else in `crates/goad/src` names `next_check`.

**4. ✓ The seeded instant is unreachable from outside `Host`.** `Host::new`
(`crates/goad-shell/src/host.rs:127-135`) seeds through `schedule::resolve`'s
`(None, None)` arm and stores the result in `State`. `State::resolved_check` is
`pub` (`crates/goad-shell/src/state.rs:52`) but `Host::state` is a private field
and `Host` publishes no accessor. **Consequence for design:** at process start,
before any exchange completes, stratum 3 has no way to learn the resolved check.
Its options are (a) add `Host::resolved_check()`, (b) take the first
`Outcome::next_check` from the startup evaluation that `main.rs:91` already
enqueues, or (c) recompute `now + default_poll` in stratum 3 — which restates a
rule stratum 1 deliberately states once (`schedule.rs:209-213`). This is a
design decision, not a discovered constraint.

**5. ✓ How the clock is abstracted.** `pub type Clock = fn() -> Result<Timestamp,
ClockError>` (`crates/goad/src/clock.rs:16`). A plain `fn` pointer: `Copy`,
`Send`, no trait, no lifetime. `serve` takes one (`controller.rs:303`) and calls
it through the private `stamp` helper (`controller.rs:249-253`), which maps a
clock error to `Refused::NoClock`. Production is `wall_clock()`
(`clock.rs:64-78`), built from `SystemTime` rather than `jiff::Timestamp::now()`
because the latter needs jiff's `std` feature, which would unify into stratum
1's build (D25). Tests substitute a one-line `fn` (`stub_clock`,
`crates/goad/tests/renderer/wiring.rs:45`).

**6. ✓ What the `serve` loop looks like.** `crates/goad/src/controller.rs:298-397`.
One `loop`, two `select!`s, both `biased` with cancellation first:

```
loop {
  glass.present(frame)                       // busy = false
  select! { biased;
    cancel.stopped()   => break Ending::Stopped,
    commands.recv()    => Some(command) | None => break Ending::Closed,
  }
  → build a `Pending` (or `continue` on a refusal)
  controller.engage(); glass.present(frame)  // busy = true
  select! { biased;
    cancel.stopped() => break Ending::Stopped,   // the call future is DROPPED
    outcome = call   => controller.absorb(exchanged, outcome),
  }
}
```

The command channel is `mpsc::channel::<Command>(1)` (`main.rs:72`) and the
Slint callbacks use `try_send`, reporting a full channel as a UI notice rather
than blocking (`wire.rs:122-129`). **Consequence for design:** a timer that
delivers through this channel can be *dropped* when the channel is full, which
would silently lose a scheduled check. A third `select!` arm cannot be dropped
that way. This is the single most consequential structural choice in the slice.

**7. ✓ There is no timer or sleep anywhere in production code.** The only
`tokio::time` uses in `crates/*/src` are the transport's two budgets:
`tokio::time::timeout(self.timeout, …)`
(`crates/goad-shell/src/backend/process.rs:106`) and the fixed
`CLEANUP_LIMIT = 500ms` (`:30`, `:131`, `:226`). No `Instant::now`, no `sleep`,
no `interval`.

**8. ✓ How tests drive time today: real, short wall-clock waits.**
`crates/goad/tests/renderer/wiring.rs:135-148` defines
`until(bound, predicate)`, which polls a predicate every 5 ms against a
`std::time::Instant` deadline and panics if the bound (2 s in every caller) is
exceeded. It exists because a PHASE-10 repair replaced fixed delays with an
observed condition. `#[tokio::test]` supplies the runtime; `LocalSet` +
`spawn_local` drives the real `serve`
(`wiring.rs:920-990`). `tokio`'s `test-util` feature is **not** enabled anywhere,
so `tokio::time::pause()` / `advance()` are unavailable today.

**9. ✓ The one real event-loop test.** `crates/goad/tests/event_loop/closing.rs` is
its own `[[test]]` target holding exactly one `#[test]`, because
`i_slint_backend_testing::init_integration_test_with_mock_time` *"can only be
called once per process"*. It builds the whole production arrangement — a
multi-thread tokio runtime with `enable_all()`, an `EnterGuard` held for the
loop's life, real window and tray, real channel, real `serve` — and drives a
close request through it. It never drives an **exchange** through that
arrangement, which is slice 002's first follow-up (F-5).

**10. ✓ A manual "check now" already exists.** `crates/goad/ui/app.slint:103` is a
tray menu item wired at `crates/goad/src/install.rs:40` to
`Command::Evaluate(Stimulus::Requested)`. This slice does not need to add one.

**11. ✓ `default_poll` is validated at load.** `config.rs:155-167` parses it with
`goad_semantics::schedule::parse_span` and rejects non-positive values;
`config.rs:326-350` are the tests for a negative and a zero value, the latter
named *"because it is a busy loop"*. The example config
(`config.rs:184-191`) is `default_poll = "30m"`, matching brief §5.

### Precedents

- **Pure rule, impure driver.** `schedule::resolve` takes `now` as a parameter
  and lives in stratum 1; `clock::wall_clock` supplies it from stratum 3. The
  same seam is available for a timer: a pure function that says *how long to
  wait given a resolved instant and a now*, and an impure arm that waits.
- **A `fn` pointer instead of a trait** (`Clock`, D15) — the established way to
  make an ambient capability substitutable in one line of test code.
- **A capability that is one function wide, and says what it is not.**
  `clock.rs`'s header states *"Not a timer"*. Whatever this slice adds should
  state its own boundary the same way.
- **Deriving rather than storing.** `Controller::surface` is derived from two
  fields with no combination unnamed (`controller.rs:117-124`, F-15). If a
  "next check" becomes controller state, the same discipline applies: one value,
  no redundant "is a check pending" flag beside it.
- **One consumption point for an `Outcome`** — `receive` (D16,
  `reception.rs:53`). A schedule plumbed out of `absorb` must not create a
  second place an `Outcome` is taken apart.
- **Test helpers are built, not repeated.** `tests/support/driving.rs` is
  included by `#[path]` into both test crates
  (`docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`).
  `DEFAULT_POLL` is stated once there (`driving.rs:71`).

---

## Thread 3 — prior art for the patterns this slice needs

**How slice 002 tested async behaviour without wall-clock sleeps: it did not
avoid them; it made them conditional.** The rule that emerged
(`wiring.rs:131-134`, a PHASE-10 repair) is *poll for the condition, bound the
wait, never assume a fixed delay covers it*. Every `serve`-driven test uses
`until(Duration::from_secs(2), predicate)` with a 5 ms poll. What that buys is a
test that fails loudly rather than flakily; what it costs is real seconds when a
condition never arrives.

**The alternative nobody has used here yet: tokio's mock clock.**
`tokio::time::pause()` / `advance()` / `#[tokio::test(start_paused = true)]`
require the `test-util` feature, which is not enabled
(`Cargo.toml:36-37`, `crates/goad/Cargo.toml:22`). Two facts make it a design
question rather than an obvious win:

1. Paused time **auto-advances** to the next timer whenever the runtime has
   nothing else to do. The transport's own `tokio::time::timeout(self.timeout,
   …)` (`process.rs:106`) is a timer. With a real child process being awaited —
   which every existing `serve` test uses — the runtime is idle waiting on
   process I/O, so the backend timeout would fire immediately. A mock-clock test
   of the schedule and a real-process test of an exchange may not be the same
   test.
2. `pause()` requires a current-thread runtime. `#[tokio::test]` is
   current-thread, so that part is satisfied; production is multi-thread
   (`main.rs:59-62`), so a mock-clock test is by construction not the production
   topology.

**Slint has its own timer facility**, and its own mock clock:
`i_slint_backend_testing::init_integration_test_with_mock_time` is already used
(`closing.rs:35`) and advances Slint's timers, not tokio's. A `slint::Timer`
would be testable under that harness and would be polled by the executor that
already owns `serve`. It would also be a second time source in a program that
has one. This is a genuine two-way design choice and should be decided, not
defaulted into.

---

## Thread 4 — the carried hazards, each at its code site

**H-1 ✓ — the busy-loop (slice 001 F-1, F-34, F-48; `docs/slices/001/notes.md`
around :266).** The failure mode as originally found: `schedule::resolve`
retained an elapsed check indefinitely, so a backend that stopped sending
`next_check` after a scheduled poll left the host reporting an instant in the
past on every exchange, and a timer firing on a past instant would spin. It was
**fixed in slice 001**: `resolve`'s second arm now requires `pending.instant() >
now.instant()` (`schedule.rs:229`), and an elapsed retained check falls to
`now + default_poll`. The boundary is `<=`, not `<`, deliberately — an exchange
run *at* the resolved instant is that check firing
(`schedule.rs:196-201`, and the unit test
`a_retained_check_equal_to_now_is_consumed_too`).

What the slice-001 follow-up asks of this slice is the consequence, and it is
worth stating precisely because the follow-up's own wording overstates it:

> *"The timer consumes a resolved instant that is always ahead of the `now` it
> was resolved at, on success and on failure alike."*

That is true on **two** of `resolve`'s three arms and false on the first. Arm
two returns a `pending` strictly greater than `now`; arm three returns
`now + default_poll` with `default_poll` validated positive at load
(`config.rs:155-167`). Arm one — `(Some(instruction), _) => instruction` —
returns a backend instruction **verbatim, past or future**, because R-28 forbids
the host adjusting it. `schedule.rs:198-201` says so in as many words: *"A
backend-supplied past instant (R-28) is therefore stored as given, fires once,
and then falls back to cadence."*

So the timer must handle an instant at or before `now`, and the correct
behaviour is to fire **immediately, once per resolved instant**. That "once" is
about the instant, not about the host: `resolve`'s *consuming* arm is
`(None, Some(pending))`, reachable only when no valid instruction arrived, so
the following exchange consumes the elapsed value **only if the backend stops
instructing the past**. Arm one returns a valid `incoming` verbatim, replacing
the retained value rather than consuming it. `schedule.rs:198-201`'s doc
comment states the one-off case as though it were the general one and should be
corrected when `wait_for` lands beside it. **The residual hazard, which nothing
in the tree currently prevents:** a backend that returns a past `next_check` on
*every* response drives an unbounded-rate exchange loop.
Each iteration costs one backend invocation, so it is a hot loop rather than an
infinite one, but there is no floor on the poll rate anywhere in the host. R-28
forbids clamping the instruction; whether a **minimum interval between
evaluations** is a legitimate host-side floor (it constrains the host's own
cadence, not the backend's stored instruction) is a design question this slice
must answer or explicitly decline.

**H-2 — no re-resolution.** The timer must not call `schedule::resolve` itself.
There are exactly two call sites, both in `Host` — the seed at `host.rs:128`
and the one inside `resolve_from` at `host.rs:259`; the latter is reached from
two callers,
and both write what they report (`host.rs:229`, `:290`). A timer that
re-resolved would compute against a different `now` and produce a value the host
never stored — which is precisely the drift F-48 was raised about
(`host.rs:275-279`: *"Otherwise a later exchange would resolve from an instant
the caller was never told, and every further failure would push the wake later
from its own `now`."*). The timer's whole input is `Outcome::next_check`.

**H-3 ✓ — latest-valid-wins is issue order, not `max`.** `schedule.rs:204-208` and
the unit test
`a_valid_incoming_instruction_wins_even_when_it_is_earlier_than_the_retained_one`.
A backend asking to be checked *sooner* is still the later instruction. For the
timer this means a pending wait must be **replaceable in both directions**: an
instruction that moves the check earlier has to shorten the current wait, not
merely be picked up at the next expiry. Roadmap AC-8 — *"a later valid
`next_check` supersedes an earlier one, observable over time"* — is exactly this
case, and it is the one a naive implementation gets wrong.

**H-4 — a failure must not accelerate retries.** R-29 and `host.rs:267-281`. The
failure path resolves with `incoming: None`, so a retained instant still ahead
stands unchanged and an elapsed one becomes `now + default_poll`. The timer
therefore gets a correct instant on the failure path already; the hazard is a
timer that adds retry logic of its own on top. `host.rs:89-91` states why there
is no retry at all: *"the host cannot know what a failed exchange already did."*

**H-5 — OQ-3 and persistence.** `docs/specs/001-host-backend-protocol.md` §8
OQ-3 is shut only while nothing persists; `crates/goad-shell/src/state.rs:3-4`
records the same scoping in the code (*"Nothing is written to disk (the OQ-6
decision), so the state space is one process lifetime wide"*). Brief §20 phase 4
leaves persistence open with *"if required"*. If this slice persists a resolved
check, a restarted host can hold a `view_id` minted by a previous process, and
R-32's rejection rule needs a stated behaviour across restarts. See OQ-1 in
`slice-003.md`.

**H-6 — a clock failure inside the loop.** `stamp` maps `ClockError` to
`Refused::NoClock` and the loop `continue`s with no exchange
(`controller.rs:249-253`, `:352-357`). A timer that needs a `now` to decide
whether to fire has the same failure mode, and it must not become "the schedule
is lost". Whatever holds the resolved instant must survive a clock read that
failed.

---

## Thread 5 — external constraints

**tokio.** ✓ Pinned at 1.53.1 (`Cargo.lock:5269-5271`). Workspace feature set:
`process`, `time`, `rt`, `io-util`, `macros` (`Cargo.toml:36-37`);
`crates/goad` adds `rt-multi-thread` and `sync`
(`crates/goad/Cargo.toml:22`). So `tokio::time::sleep`, `sleep_until`,
`Instant`, `Sleep` and `interval` are **already available** with no manifest
change. `test-util` (for `pause`/`advance`/`start_paused`) is not, and would be
a `[dev-dependencies]` addition to `crates/goad`. Because `tokio` is not a
dependency of `crates/goad-semantics`, neither addition touches POL-001's
feature-unification residue.

**jiff.** 0.2.35 (`Cargo.lock:2576-2577`), `default-features = false` at the
workspace level. `Timestamp` arithmetic is available; `Timestamp::now()` is not,
by decision (D25) — enabling jiff's `std` feature *would* hit the residue,
because jiff **is** a stratum 1 dependency. Converting a `jiff::Timestamp`
difference into a `std::time::Duration` for a tokio sleep is a stratum 2/3 job.

**Slint and tokio coexistence, as slice 002 settled it.** The arrangement is
`main.rs:59-116` and it has three parts:

1. A multi-thread tokio runtime built with `enable_all()`, whose `EnterGuard`
   (`main.rs:63`) is held for the entire life of the Slint event loop. Without
   it, *"the first poll of a `tokio::process` future on the Slint thread panics
   with **there is no reactor running**"* — measured in slice 002's research
   Thread 4.
2. `slint::spawn_local` drives the single `serve` future
   (`main.rs:96-116`); Slint's executor polls it on the UI thread, tokio's
   reactor and timer driver run on the runtime's own threads.
3. `slint::quit_event_loop` is called from exactly one place, after `serve`
   returns.

`crates/goad/tests/event_loop/closing.rs:41-47` reproduces parts 1–3 verbatim in
a headless test.

**What that arrangement has never been shown to do: complete a timer.** No test
drives an exchange — or any tokio timer — through the production topology; that
gap is slice 002's own first follow-up (F-5, `docs/slices/002/slice-002.md`
§Follow-ups). The reasoning that it works is sound (a `tokio::time::Sleep`
registers with the timer driver reachable through the `EnterGuard`, and the
driver wakes the future's waker, which for a `spawn_local` future is Slint's
cross-thread wake), but it is reasoning, not measurement, and this repository's
convention is to measure. **Spike S-1 for the design stage: put a
`tokio::time::sleep` inside a future polled by Slint's executor under the
`EnterGuard` and observe it complete.** If it does not, the timer is
`slint::Timer`'s and the design changes shape.

### Spike S-1 result — **run 2026-09-07, and it completes**

**What was run.** `docs/slices/003/timer-probe.local.rs` (gitignored under
`*.local.*`, preserved for re-running), added to `crates/goad/Cargo.toml` as a
temporary `[[test]]` target pointing outside the crate, run with
`cargo test -p goad --test timer_probe -- --nocapture`, and the manifest
reverted immediately afterwards. No spike code was left in `crates/`.

**The topology.** Exactly `start`'s: `init_integration_test_with_system_time()`
for a real headless Slint event loop on real time; a multi-thread tokio runtime
built with `enable_all()`; its `EnterGuard` held for the life of the loop; a
real `PromptWindow` and `Tray`; one `slint::spawn_local` future; the process
ended by `slint::quit_event_loop` from inside that future, with
`run_event_loop_until_quit` on the outside.

**Four cases, one process.**

| case | what it put in the spawn_local future | measured |
|---|---|---|
| A | a bare `tokio::time::sleep(300 ms)` | completed in **301.5 ms** |
| B1 | a pinned `Sleep`, `reset()` to 200 ms, in a `biased` `select!` against an idle `mpsc` receiver | fired **via the timer arm** in **201.3 ms** |
| B2 | the same `Sleep`, `reset()` again after it had already fired, to 150 ms | fired via the timer arm in **150.4 ms** |
| C | a real `Host::evaluate` over the real `ProcessBackend` against a real child process | completed in **2.9 ms**, no failure, no cleanup failure, `next_check` resolved |
| D | `sleep_until` an instant 10 s in the **past** | completed in **2.1 µs**, no spin, no underflow |

Total wall time 656 ms for the whole probe.

**What it settles.** OQ-7 falls to `tokio::time`: a `Sleep` polled by Slint's
executor under the `EnterGuard` completes on time, a pinned `Sleep` can be
`reset` repeatedly — including after it has already fired, which is exactly the
design's re-arm — and an already-elapsed deadline completes immediately rather
than hanging or underflowing. Case C is separately the first time an
**exchange** has been driven through the production topology, which is slice
002's follow-up F-5.

**One incidental observation.** The headless backend logs `Slint: Failed to
create system tray icon: Failed to create a rgba8 buffer from an icon image`
when a `Tray` is constructed under the testing backend. It is noise, not a
failure: the existing `closing.rs` builds a `Tray` the same way and passes.

---

## Cross-thread findings

**X-1 — The seam is already cut, and only one wire is missing.** SPEC-001 §2
says the timer *"consumes a resolved next-check instant and calls `evaluate`"*.
`Outcome::next_check` is that instant; `receive` already carries it into
`Received`; `absorb` already destructures it and names slice 003 in the comment
that throws it away (`controller.rs:140-143`). The slice is small because slice
001 and 002 each stopped exactly at this line. What is genuinely undecided is
**where the waiting happens** — a `select!` arm in `serve` (stratum 3), or a
component in stratum 2 — and **what the pure part of it is**.

**X-2 ✓ — The channel is the wrong road for a timer.** Fact 6: capacity 1,
`try_send`, a full channel becomes a UI notice. A person clicking twice can
legitimately lose the second click and be told so; a scheduled check that
silently evaporates because a click was in flight is a defect with no observable
symptom. This argues for a third `select!` arm over a `Command::Evaluate`
producer, and it argues independently of any preference about strata.

**X-3 — The one arm that returns an unadjusted past instant is where the
acceptance criteria bite.** H-1 and H-3 are the same arm of `resolve` seen from
two sides. Roadmap AC-8's observability requirement and the no-busy-loop
property both live there. A test that only exercises the default poll exercises
neither.

**X-4 — Two mock clocks, and they are not the same clock.** Slint's mock time
(already in the tree, `closing.rs:35`, once per process) and tokio's `test-util`
(not in the tree). If the timer is tokio's, the cheap tier can use `test-util`
but the real-event-loop tier cannot advance it; if the timer is Slint's, the
real-event-loop tier can advance it but the cheap tier gains a dependency on a
Slint backend it currently avoids. Neither is free, and the choice of timer
facility and the choice of test strategy are one decision, not two.

**X-5 — Nothing observable changes on the glass unless the slice decides it
should.** `Frame` carries `surface`, `shown`, `diagnostics`, `busy`
(`controller.rs:77-83`); the tray tooltip is a function of diagnostics and
whether a view is shown (`glass.rs:104-107`). A resolved next check has no
representation on the glass today. Making one is a product decision and is
OQ-3 in `slice-003.md`.

---

## Design-input deltas

1. **The slice-001 follow-up's premise needs correcting in the design.** "Always
   ahead of the `now` it was resolved at" is false on `resolve`'s first arm
   (Thread 4, H-1). The design must state the timer's behaviour for a resolved
   instant at or before `now` — fire immediately, once — rather than assume the
   case cannot arise. Assuming it cannot arise is how a `Duration` subtraction
   underflows.
2. **A floor on evaluation rate is an open product question**, surfaced by H-1's
   residual hazard and not settled by any canon. R-28 forbids clamping the
   *instruction*; it says nothing about the host's own minimum interval between
   evaluations.
3. **The initial resolved check has no route out of `Host`** (Fact 4). Design
   must pick one of three, and (c) restates a stratum 1 rule.
4. **The timer should not be a `Command`** (X-2).
5. **The timer facility and the test strategy are one decision** (X-4), and
   spike S-1 (Thread 5) gates it.
6. **Nothing here requires a change to `crates/goad-semantics`** unless the
   design chooses to put a pure "how long until this instant" function there.
   That would be the natural home under ADR-001 and would need no new
   dependency: `jiff::Timestamp` subtraction yields a `SignedDuration` and is
   already used in stratum 1.

# Notes — Slice 004

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the A-1 probe | done — **A-1 holds** | 2026-09-08 |
| PHASE-02 — the configuration key, and the envelope | done | 2026-09-08 |
| PHASE-03 — the socket's lifecycle, and the accepted path | done | 2026-09-08 |
| PHASE-08 — the refusal vocabulary, and the closed reason set | done | 2026-09-08 |
| PHASE-04 — `serve`'s ingress arms, the second anchor, and what a refusal costs | done | 2026-09-08 |
| PHASE-05 — the two anchors, and what a person can see | done | 2026-09-08 |
| PHASE-06 — binding at startup, and the demo a person runs | code done, **VH-1 open** — awaiting the user's run (runbook in this file, PHASE-06 sheet) | 2026-09-08 |
| PHASE-07 — the sweep, the spec's own table, and the gate | done | 2026-09-08 |

Rows are in **execution order** — PHASE-08 is the listener's second half and
runs between 03 and 04 (`plan.md` PL-10). Phase ids are immutable, so a split
appends a number rather than renumbering.

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-04 — `serve`'s ingress arms, the second anchor, and what a refusal costs

**Entry check:** EN-1 — PHASE-08's exit criteria are discharged (its sheet
below, every criterion with evidence) and `just check` **exit 0** at `9d36002`,
re-run before anything was edited (transcript:
`…/scratchpad/phase04-baseline.txt`, session-local). EN-2 — `bind`, `Ingress`,
`Arrival`, `Answer` and `Refusal` exist in
`crates/goad-shell/src/ingress/mod.rs`, all `Debug`, all `Send`; `Refusal`
already carries `Engaged` and `TooSoon { retry_after }` and the reason set is
closed at eight (PHASE-08/EX-5). Both hold: proceeding.

**Reading list**

| what | where |
|---|---|
| the phase, entire | `plan.md:1100-1482` |
| the Overview's five standing rules, and §Sequencing on why this phase is not split | `plan.md:81-112`, `:166-177`, `:201-210` |
| F-15's body — the finding this phase settles, and what returns it `contested` | `review-design.md` `### F-15` |
| the order of judgement, both arms, and the sequence diagram (position is time) | `design.md` §5.4 `:391-515` |
| the two anchors, one write site each | `design.md` §5.3 `:358-390` |
| `Fired` never sees a closed channel | `design.md` §5.2 `:283-300` |
| what a refusal costs the UI thread | `design.md` §5.5 `:530-546` |
| R2 and R6 | `design.md` §8 `:638-648` |
| the requirements | `draft-spec.md` R-12 `:104`, R-14 `:106`, R-15 `:107`, R-16 `:108`, §5 order of judgement `:134-161` |
| `serve` as it stands, both `select!`s | `crates/goad/src/controller.rs:382-513` |
| `floor_until`'s initialisation and its one write site | `controller.rs:407`, `:420` |
| `refusal_re_arms` | `controller.rs:427` |
| the inner `select!` | `controller.rs:505-514` |
| the unit test module VT-8 joins | `controller.rs:524-583` |
| `Refused` and `Diagnostics::refused` | `crates/goad/src/diagnostics.rs:45-61`, `:136-158` |
| what one `present` costs | `crates/goad/src/glass.rs:67-121` |
| the ingress surface this phase consumes | `crates/goad-shell/src/ingress/mod.rs:196-346` (`Ingress`, `Arrival`, `Answer`, `Refusal`, `reason()`, `Display`) |
| the renderer tier's shape, and the helpers not to re-mint | `crates/goad/tests/renderer/scheduling.rs:1-230`, `renderer/harness.rs` whole |
| `@slow-view`, the only sentinel that holds an exchange open in the foreground | `tests/backends/answers-as-instructed.sh` |
| the request wire form VT-1 reads | `goad-semantics/src/protocol/canonical.rs:505-557` — `{"protocol":1,"type":"evaluate","now":…,"event":{…}}` |

**Assumptions**

- **`tokio::net` is reachable from `crates/goad`'s test targets only by feature
  unification** (`goad-shell` enables `net`; `crates/goad/Cargo.toml` does not,
  and this phase may not touch a manifest). The cases therefore use
  **`std::os::unix::net::UnixStream` on `spawn_blocking`** for the writer's
  side: it depends on no feature `crates/goad` does not itself declare, and it
  keeps the blocking client off the thread `serve` runs on.
- **Every refusal the loop decides is folded onto the controller; only the
  *outer* arm presents.** R-15's bound is then emergent rather than
  special-cased: the outer arm's `continue` reaches
  `glass.present` at the top of the loop, the inner arm's does not and `absorb`
  supersedes it (`design.md` §5.2's *the inner arm folds the same refusal*, and
  §5.2's reason table for `engaged`: *`absorb` overwrites it*).
- **The four commands move into a private `dispatch`**, so that the ingested
  road and the command road meet at one `Option<Result<Pending, Refused>>`
  rather than at a `Command` an arrival cannot become (D-13). Pure extraction,
  no behaviour change; it is what keeps EX-10 true of `serve`.
- **`spacing_elapsed` is the named private free function** EX-6 requires, and
  VT-8 is its only caller besides the arm.

**STOP conditions** (`plan.md` S-1..S-5, not softened)

- S-1 — the inner `select!` cannot become a loop, or `call` cannot be pinned.
- S-2 — an ingested evaluation cannot be built without a `Stimulus` variant.
- S-3 — a margin under 10x on a case VA-2 does not exempt, or an existing test
  in the three `crates/goad` targets goes red.
- S-4 — VT-5's presentation number is bad: F-15 returns to the ledger
  `contested` and `SPEC-003/R-15` is **not** weakened to make the phase green.
- S-5 — VT-7's `None` is unreachable by the second-runtime route.

**Tasks**

- [x] sheet written; status `in progress`.
- [x] EN-1/EN-2 verified.
- [x] `diagnostics.rs` — `Refused::Ingress { reason, detail }` and its
      `Diagnostics::refused` arm (EX-2).
- [x] `controller.rs` — `Served.ingress`, `serve`'s parameter, `Fired::Ingested`,
      both arms, `event_floor_until`, `spacing_elapsed`, `ingest`,
      `refuse_arrival`, `refuse_during_exchange`, `ingress_stopped`, `dispatch`
      (EX-1..EX-10).
- [x] `controller.rs` unit tests — VT-8, the boundary.
- [x] 23 call sites (EX-7) + `renderer/main.rs`'s two-line `mod` and two doc
      sentences.
- [x] `renderer/ingress.rs` — VT-1..VT-5, VT-7.
- [x] VA-1..VA-4; harvest.

**Verification — every criterion, discharged**

| id | discharged by |
|---|---|
| EX-1 | `serve` takes `ingress: Ingress` last (`controller.rs`); `Served` gains `pub ingress: Ingress`, handed back beside `host`, `controller` and `glass`. No test destructures `Served` — re-verified, `git grep -n Served` outside `controller.rs` finds only the two doc comments in `wiring.rs` |
| EX-2 | `Fired::Ingested(Arrival)`; `Refused::Ingress { reason: String, detail: String }` (`diagnostics.rs`), both fields `Clone + PartialEq + Eq` so `Refused`'s derives are unchanged; `Diagnostics::refused` gains its arm, still exhaustive with no `_` |
| EX-3 | the outer `select!`'s fourth arm is **last** in `biased` order: cancel → commands → sleep → ingress |
| EX-4 | the inner `select!` is a `loop` with three arms in `biased` order: cancel → call → ingress. `call` is `std::pin::pin!`ed across iterations; the outer loop carries the label `'serving` so `break 'serving Ending::Stopped` still leaves the loop. The ingress arm falls through to the next iteration — the **same** exchange, nothing re-invoked (VT-3 asserts `invocations == 1`) |
| EX-5 | `ingest` is steps 1 and 3-5 in that order; `refuse_during_exchange` is steps 1 and 2 in that order. Shape precedes state in **both**: each reads `Arrival::into_parts`'s `Result` first and refuses with the shape refusal it carries before looking at any host state |
| EX-6 | `let mut event_floor_until = started;` with the comment the criterion asks for, citing `SPEC-002/R-12` and P-3. **One write site** — `ingest`'s `*event_floor_until = arrived + MINIMUM_SPACING`, below step 3 and above steps 4 and 5, so an *attempted* evaluation writes it — and **one read site**, step 3's `spacing_elapsed`. `floor_until` is not named in `ingest` and `event_floor_until` is not named in the timer arm. One `MINIMUM_SPACING`, no second constant. `refusal_re_arms` is untouched and still `matches!(fired, Fired::Scheduled)`. The comparison is the named private free function `spacing_elapsed(now, floor) -> bool { now >= floor }`, with the doc comment R-14 forces |
| EX-7 | 23 sites: `git grep -c 'Ingress::none()'` — `main.rs` 1 (full path, so no import), `renderer/wiring.rs` 6, `renderer/scheduling.rs` 14, `event_loop/closing.rs` 1, `event_loop_schedule/scheduling.rs` 1. No assertion moved — VA-3 |
| EX-8 | `Fired::Ingested` carries an `Arrival`, never an `Option`. The outer arm's `None` handler folds `ingress_stopped()` and `continue`s **inside the `select!` arm**, before any `Fired` exists; the inner arm's folds the same and falls through to the next inner iteration. Neither reaches `refusal_re_arms`, resets `sleep`, or writes an anchor |
| EX-9 | `git diff 9d36002 -- crates/goad/src \| grep '^+' \| grep -E '\.unwrap\(\|\.expect\(\|panic!\|#\[expect'` is **empty**. Nothing derived from an envelope is unwrapped: `into_parts` yields a `Result` that is matched, and every failure is a `Refusal` |
| EX-10 | the arms are three lines (outer: `match arrival`, one call) and four (inner), delegating to four private free functions of 6-30 lines each. `serve` is 167 lines against 136 before, having **given up** the four-command match to `dispatch`: the net growth is the anchor, its comment, and the two arms. R2 did not fire — see *Findings*, which records what was done to keep it from firing rather than claiming the risk away |
| EX-11 | the file's module doc states the rule and each member's doc comment names what it pinned: VT-1, VT-4, VT-5 and VT-7 pin `next_check` at 60 s (`NEXT_CHECK_A_MINUTE_OFF`); VT-3's exchange is `@slow-view`, whose body pins 45 minutes in the script itself. VT-2 states that it is **not** a member and why; VT-8 is not in the file |
| VT-1 | `a_well_formed_envelope_produces_one_evaluation_carrying_all_four_fields` — one `evaluate`; `source`, `kind` and `data` byte-identical; `timestamp` asserted the **same instant** by parsing both sides with `jiff` and separately asserted to render with `Z` (A-3 checked at the point of use); `now` asserted to be `stub_clock`'s `2026-01-01T00:00:00Z`, not the envelope's |
| VT-2 | `the_view_an_ingested_evaluation_returns_reaches_the_window_and_is_answerable` — the heading reaches the real window, the token is read off the options model with `current_view_token` (never minted), the `Choose` reaches the backend, the window closes |
| VT-3 | `an_envelope_arriving_during_an_exchange_is_refused_engaged_before_it_completes` — `engaged`, and the view had **not** landed when the reply came back, so the refusal is the inner arm's. `invocations == 1` at the end: the refusal re-invoked nothing. Connect-to-reply measured **314-490 µs** against `@slow-view`'s 200 ms foreground sleep (~400-640x) |
| VT-4 | `a_second_envelope_inside_the_spacing_is_refused_too_soon_and_says_how_long` — `too_soon` with `retry_after_ms` in `(0, 3000]`; the control waits exactly `retry_after_ms` and is **accepted**, which is R-14's rounding read from the host's own field; two accepted envelopes, two evaluations |
| VT-5 | `a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal` — see **F-15's number** below |
| VT-6 | the three `crates/goad` test targets pass with assertions unchanged: `renderer` 138 → **144** (the six new), `event_loop` and `event_loop_schedule` unchanged, `goad` lib 15 → **16** (VT-8). VA-3 is the evidence that nothing else moved |
| VT-7 | `a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating` — the second-runtime route (S-5) **works**: `bind` under a multi-thread runtime's `enter()`, then `shutdown_background()`, and `arrival()` yields `None`. One diagnostic line, naming that ingress has stopped; the presentation count does **not** advance over the 500 ms after it; a scheduled firing then lands at the backend. Assertion 1 is read on the live route — see *Findings* F-b |
| VT-8 | `a_writer_arriving_exactly_at_the_anchor_is_outside_the_spacing`, in `controller.rs`'s own `#[cfg(test)] mod tests` beside `deadline_after`'s three cases: at the floor, one nanosecond before, one nanosecond after. **Non-vacuous, measured**: with `>=` changed to `>` the first assertion fails with its own message and passes again when reverted |
| VA-1 | `just check` **exit 0** (transcript `…/scratchpad/phase04-gate.txt`, session-local). Baseline at `9d36002` also exit 0 before anything was edited (`…/phase04-baseline.txt`) |
| VA-2 | the margin table below |
| VA-3 | `…/scratchpad/va3-bounded.py` (kept): with the added argument, the four `use` lines and the one `main.rs` comment removed, all five bounded files are **token-identical to `9d36002`** — `ALL MATCH`. No renamed symbol, no changed test body, no touched assertion |
| VA-4 | **break-and-revert on the anchor.** With `*event_floor_until = arrived + MINIMUM_SPACING` removed: VT-4 **red**, VT-5 **red** (it turns on the anchor too), VT-1, VT-2, VT-3 and VT-7 **green**. Reverted; all six green again |

**F-15's number (VT-5), and the disposition**

| measured | value |
|---|---|
| window | 500 ms, flat out, after the priming exchange was absorbed — so every refusal in it is one the loop decided **while idle**, which is the state R-15 obliges the host to report |
| refusals | **845** |
| presentations | **845** |
| **presentations per refused envelope** | **1.000** |
| presentations per second | **~1690** |
| assertion | `assert_eq!(cost, replies.len())` — the cost is fixed **at one**, so a change that raised it, or that added a second route to the surface, fails here |

**F-15 settles.** The claim it was dispositioned `settle-in-code` to settle — that
one refusal costs one presentation and no more — is now a number and an
assertion rather than an argument. `SPEC-003/R-15` was not weakened.

**What the number does not say, stated rather than absorbed.** ~1690
presentations per second is the *loop's throughput* under a writer that the
host itself paces (I-2: one arrival outstanding, and the writer waits for its
reply before opening the next connection). It is **not** a measurement of
whether a person could sit in front of it: this tier is headless
(`init_no_event_loop`), so no compositing happens and `show()`/`hide()` are
cheap. Whether the window is *visibly* unresponsive under a flood is R6's other
signal and belongs to a human run. `design.md` §8 R6 is explicit that one-for-one
is not the signal — it is the design — and that the signal is a *rise* above what
this phase measured. This phase is that baseline.

**VA-2 — margins, at the bound each assertion actually governs**

Three kinds, as VA-2 requires each case be classified. Times are the whole test
binary invocation (≈40 ms of process and Slint start-up included), three runs.

| case | elapsed | the bound that governs | margin | kind |
|---|---|---|---|---|
| VT-1 | 160-170 ms | `LIVENESS_BOUND` 5 s | ~30x | liveness |
| VT-2 | 162-169 ms | `LIVENESS_BOUND` 5 s | ~30x | liveness |
| VT-3 | 368-378 ms | `@slow-view`'s 200 ms foreground sleep, against a **314-490 µs** connect-to-reply | **~400-640x** | liveness (the `engaged` reply must land inside the exchange) |
| VT-4 | 3159-3177 ms | `LIVENESS_BOUND` 5 s over a wait that **is** `MINIMUM_SPACING` | ~1.6x | **exempt** — a wait that is the bound under test. D-5 rejected a configurable spacing precisely so no test could buy time by moving a bound |
| VT-5 | 671-682 ms | the 500 ms window is chosen, not a bound; the settle wait is `LIVENESS_BOUND` 5 s and returned in <5 ms | >1000x on the settle | liveness (plus a constructed window) |
| VT-7 | 3163-3167 ms | two: the 500 ms **anti-fire window** (a chosen window, asserted as an equality, no ratio admitted); and `LIVENESS_BOUND` 5 s for the scheduled firing, which is `serve`'s initial arm at `MINIMUM_SPACING` | anti-fire: n/a; liveness: ~1.7x | **exempt** on the same ground as VT-4 — the firing it waits for *is* `MINIMUM_SPACING`. The two windows are kept apart by a compile-time assertion that the anti-spin window closes well inside the spacing |

No margin under 10x on a case VA-2 does not exempt. **S-3 not reached.**

**Decisions taken during execution**

- **The four commands moved into a private `dispatch`, returning
  `Option<Result<Pending, Refused>>`.** An arrival cannot become a `Command`
  (D-13, S-2), so the two roads have to meet at the `Pending` both produce; the
  extraction is what gives them one join point instead of a second refusal site,
  and it is why `serve` grew by the arms rather than by the arms plus a
  duplicated dispatch. Pure extraction, no behaviour change — VA-3 and the
  unchanged suite are the evidence.
- **Every refusal the loop decides is folded onto the controller; only the
  *outer* arm presents.** R-15's bound is then a consequence of the loop's
  shape rather than a special case: the outer arm's `continue` reaches
  `glass.present` at the top; the inner arm falls through to the same exchange
  and `absorb` supersedes the fold before anything is presented. This is
  `design.md` §5.2 in terms (*the inner arm folds the same refusal*) and §5.2's
  reason table for `engaged` (*`absorb` overwrites it*). **A note for
  PHASE-05:** R-15's negative case must therefore read the **live** route, not
  `Served.controller` — a case that cancels mid-exchange and reads the retained
  value will find the fold there.
- **The clock-unreadable case folds `Refused::NoClock`, not a second
  `Refused::Ingress`.** `stamp` already renders `ClockError` once at the one
  site that has it, and the resulting line names the clock exactly as EX-5 step
  4 asks — the same line a scheduled firing writes for the same fault. See
  finding F-a for what this does *not* reach.
- **The writer's side of every case is `std::os::unix::net::UnixStream` on
  `spawn_blocking`,** not `tokio::net`. See finding F-c.
- **VT-5's window opens after the priming exchange has been absorbed.** Only
  then is the loop idle, and only then is every reply `too_soon` rather than a
  mixture of `too_soon` and `engaged` — which is what the criterion asks for,
  and what makes the presentation count a count of *reportable* refusals.

**Findings**

- **F-a — `unavailable`'s wire `detail` cannot name the clock, and EX-5 step 4
  says it should.** `design.md` §5.4 says *"`unavailable` means one thing about
  the host and three about why … and `detail` says which"*, and EX-5 step 4 says
  *"`unavailable`, `detail` naming the clock"*. The wire `detail` is
  `Display for Refusal`, and `Refusal::Unavailable` is a **unit** variant whose
  `Display` is fixed at *"no answer was given for this envelope"* — PHASE-03's
  wording for the dropped-`Answer` cause. The loop cannot vary it without adding
  a payload to `Refusal`, and `crates/goad-shell/src` is a forbidden surface
  here. **Built as:** the writer gets `reason: "unavailable"` with that generic
  detail; the clock is named on the **diagnostics surface** instead, where
  `Refused::NoClock` already renders it. So R-14 is satisfied (the reason is
  from the closed set) and R-15 is satisfied (a person is told what happened);
  what is not true is §5.4's claim that the *wire's* `detail` distinguishes
  `unavailable`'s three causes. **Not repaired here** — it is a change to
  PHASE-03's type, which is a design question about what `Refusal` carries.
  Reported to the orchestrator.
  **Resolved** (bounded cross-phase repair, orchestrator-ruled, `d823739`'s
  follow-on): `Refusal::Unavailable` took the payload PHASE-03's unit variant
  was missing — a new two-variant `UnavailableCause` (`Stopping`,
  `ClockUnreadable`), naming the two wire-side causes of `draft-spec.md`
  §6.3's *writer's fix* column; the ingress-stopped third cause still gets no
  variant, because it answers no envelope and has no wire reply to carry
  (`design.md` §5.2) — `controller.rs`'s `ingress_stopped()` now names the
  `unavailable` token directly rather than borrowing it from a `Refusal`
  value that would misdescribe the cause. `Display` renders each wire cause
  distinctly (`ingress/mod.rs`); the shutdown wording stays byte-identical to
  what PHASE-03 wrote for it. `crates/goad/src/controller.rs`'s `ingest` step
  4 now constructs `Refusal::Unavailable(UnavailableCause::ClockUnreadable)`,
  so the loop names the clock exactly as EX-5 step 4 asked. Held by a new
  test, `unavailable_s_two_causes_carry_different_detail`
  (`crates/goad-shell/tests/integration/ingress.rs`, beside VT-6): it asserts
  both the clock cause and the shutdown cause (VT-6) reply `reason:
  "unavailable"`, and that their `detail` strings differ — the distinction
  §5.4 and §6.3 claimed and that nothing previously asserted. `just check`
  exits 0.
- **F-b — PHASE-04/VT-7's assertion 1 cannot read `Served.controller`, because
  assertion 3 destroys what it would read.** The plan's *Where each assertion
  reads its number* paragraph gives assertion 1 the retained diagnostics and
  assertion 2 the live route. But assertion 3 requires a scheduled exchange to
  land **and be absorbed**, and `Controller::absorb` assigns
  `self.diagnostics = diagnostics` — it replaces the whole value — so by the
  time `serve` returns, the fold is gone. **Built as:** assertion 1 reads the
  live route (`window.get_diagnostic_lines()`, written unconditionally by
  `glass.rs`), asserting one line naming that ingress has stopped; that it
  happened **once** is held by assertion 2, since a second fold would cost a
  second presentation. The case's own doc comment says so. A plan defect, not a
  code one.
- **F-c — `tokio::net` reaches `crates/goad`'s test targets only by feature
  unification.** `crates/goad/Cargo.toml` declares `tokio` with
  `rt-multi-thread` and `sync`; `net` arrives only because `goad-shell` enables
  it and cargo unifies features across the graph. A case written against
  `tokio::net::UnixStream` would compile today and break the day `goad-shell`
  stopped needing `net` — for a reason nothing in `crates/goad` states. This
  phase may not touch a manifest, so the writer's side uses
  `std::os::unix::net::UnixStream` on `spawn_blocking` instead: it depends on
  nothing this crate does not already declare, and the blocking pool keeps it
  off the thread `serve` runs on. Recorded rather than repaired.
- **F-d — EX-7's call-site line numbers are each one line later than the plan
  says.** `event_loop/closing.rs:82` → `:83`,
  `event_loop_schedule/scheduling.rs:110` → `:111`, and every
  `renderer/scheduling.rs` number `:166…:874` → `:167…:875`. Not a plan defect:
  PHASE-02 added `ingress: None` to a `Config` literal above them (FD-3), and
  the plan cited the tree at `b6ca5f7`. The **count** — 23 — is exact.
- **F-e — the 23rd argument does not fit on one line, so rustfmt wraps all 23
  call sites vertically.** `rustfmt`'s `fn_call_width` is 60; the existing
  six-argument `serve(…)` is 55 characters and the seventh takes it to 72. Every
  site therefore becomes the same nine-line vertical form, and the raw diff over
  the five bounded files is ~230 lines rather than 23. VA-3 is discharged
  **token-wise** instead (`…/scratchpad/va3-bounded.py`, `ALL MATCH`), which is
  the check the criterion is actually asking for.
- **F-f — a case that panics leaves its socket in `std::env::temp_dir()`.**
  `cleanup(&path)` runs after `local.run_until(…)` returns, so a failing
  assertion inside the block skips it — VA-4's deliberate break left two behind.
  Self-healing (`socket_path` unlinks before binding) and outside the checkout,
  so R5 is untouched; noted because a reader will meet the files.
- **The inner arm's step 1 is built but not driven here.** A *shape* refusal
  decided during an exchange is `refuse_during_exchange`'s `Err` branch;
  PHASE-05/VT-5 is the case that drives it (R-15's negative side). VT-3 drives
  the same function's step 2. Recorded so the branch is not read as untested by
  accident.

**No STOP condition was reached.** S-1: the inner `select!` became a loop with
`std::pin::pin!` and a labelled break, no second loop and no second `select!`.
S-2: no `Stimulus` variant — `ingest` builds `Pending::Evaluate` directly.
S-3: no unexempted margin under 10x, and no existing test went red. S-4: the
cost per refusal is **exactly one**, asserted; F-15 settles rather than
returning `contested`. S-5: the second-runtime route reached `None` on the first
attempt; nothing was added to `Ingress`.

### PHASE-05 — The two anchors, and what a person can see

**Entry check:** EN-1 — PHASE-04's exit criteria are discharged (its sheet
above, every criterion with evidence) and `just check` exit 0 at `c4f4796`
(per the orchestrator's brief). EN-2 — PHASE-04/VT-5's presentation number
(1.000, `845`/`845`) was good; F-15 did not return to the ledger `contested`.
Both hold: proceeding.

**Reading list**

| what | where |
|---|---|
| the phase, entire | `plan.md:1493-1585` |
| AC-6 and its reading | `slice-004.md` §Acceptance criteria, §Readings taken in design |
| the two anchors, both write sites, both starting elapsed | `design.md` §5.3 `:358-390` |
| the order of judgement, the sequence diagram | `design.md` §5.4 `:391-515` |
| §9's AC-6 row (the exact three-case argument) | `design.md` §9 `:649-681` |
| CD-3 — ADR-004's amended Verification | `canon-delta.md` `:192-242` |
| ADR-004 whole, esp. §Alternatives, §Verification | `docs/adr/004-*.md` |
| PHASE-04's sheet: decisions, F-a/F-b/F-c, the inner arm's undriven step 1 | `notes.md` PHASE-04 sheet, above |
| the loop, `ingest`, `spacing_elapsed`, both anchors, `deadline_after` | `controller.rs:360-405` (`deadline_after`, `spacing_elapsed`), `:490-537` (`ingest`), `:589-602` (both anchors' initialisation), `:606-724` (`serve`) |
| `wait_for` / R-26's re-arm | `controller.rs:507-512`, `goad-semantics/src/schedule.rs` |
| existing PHASE-04 cases and the file's own rules | `crates/goad/tests/renderer/ingress.rs` whole |
| the scheduled-only precedent for this exact shape (a person's action not clearing the *scheduled* floor) | `crates/goad/tests/renderer/scheduling.rs:830-870` (`a_person_acting_mid_cadence_does_not_clear_the_floor`) |
| `scripted`, `logging_backend`, `invocations` | `tests/support/scripting.rs` |
| `answers-as-instructed.sh` — one instruction per invocation, in order, defaults past the end | `tests/backends/answers-as-instructed.sh` |
| `within`/`until`, `LIVENESS_BOUND` | `tests/support/waiting.rs`, `renderer/harness.rs:80-85` |
| the refusal vocabulary, `Refusal::reason`/`Display`, `UnavailableCause` | `crates/goad-shell/src/ingress/mod.rs:236-370` |
| `Diagnostics::refused`/`lines`/`is_clear` | `crates/goad/src/diagnostics.rs:146-176` |

**Assumptions**

- **No new consumer for PL-8's `CountingGlass`.** None of this phase's six
  cases needs a presentation count; `harness.rs` is not touched.
- **The six cases stay in `ingress.rs`**, extending PHASE-04's file per the
  phase's own Surfaces line. Existing consts (`ENVELOPE`,
  `NEXT_CHECK_A_MINUTE_OFF`, `MINIMUM_SPACING`, `socket_path`, `cleanup`,
  `send`, `parsed`, `accepted`, `reason`) are reused; nothing is redefined.
- **`flat_out` is generalised to take the envelope text as a parameter**
  (DRY, `CLAUDE.md`) so VT-6's malformed flood and PHASE-04/VT-5's
  well-formed one share it; its window constant (`FLAT_OUT_WINDOW`) is
  reused for both, since "far shorter than the spacing" is the same
  requirement either way.
- **VT-1's construction has no priming exchange.** `floor_until` starts
  already elapsed (`started`) and nothing has fired the timer yet at test
  start, so the single ingested envelope this case sends is the *first*
  attempted firing of any kind — the only write `floor_until` could receive
  is exactly the one under test, with nothing else to disentangle it from
  (unlike VT-2, where an antecedent real scheduled firing must exist for
  "advance" to be a meaningful question).
- **VT-2's setup: priming `Requested` (100 ms) → real scheduled firing at T0
  (instructed 1 s, floored to T0+3s) → ingested firing at T0+ε (instructed
  the identical 1 s string).** Reusing the *same* instruction for the
  scheduled and the ingested exchange is what makes the two hypotheses agree
  on everything except whether the ingested firing wrote `floor_until`
  (EX-2/CD-3's load-bearing clause) — both resolve to a deadline no later
  than T0+1s, satisfying the plan's own bound on the ingested exchange
  without needing a second constant.
- **T0 is measured in the test via `Instant::now()`**, taken the moment
  `invocations(&log) >= 2` is observed (the real scheduled firing has
  started), not inferred from a rendered timestamp — the two candidate
  next-check renderings (scheduled and ingested) are textually identical
  (both "1 second" against the same fixed `stub_clock`), so they cannot be
  told apart by the surface.

**STOP conditions** (`plan.md` S-1..S-3)

- S-1 — a case cannot be written without changing production code.
- S-2 — VT-2 passes under **both** hypotheses when its anchor is broken.
- S-3 — VT-5 shows the refusal on the surface after all.

**Task breakdown**

- [x] sheet written; status `in progress`.
- [x] EN-1/EN-2 verified.
- [x] `flat_out` generalised (parameterised envelope); PHASE-04/VT-5 call site
      updated; `MALFORMED` const added.
- [x] VT-1 — `an_ingested_firing_never_writes_the_scheduled_floor`.
- [x] VT-2 — `an_ingested_firing_does_not_advance_the_scheduled_floor`.
- [x] VT-3 — `a_scheduled_firing_does_not_clear_the_event_floor`.
- [x] VT-4 — `a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface`.
- [x] VT-5 — `a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`.
- [x] VT-6 — `after_a_flood_of_malformed_envelopes_the_host_still_evaluates`.
- [x] VA-2 — margin table.
- [x] VA-3 — break-and-revert, all three breaks, recorded.
- [x] VA-1 — `just check`; harvest.

**Verification — every criterion, discharged**

| id | discharged by |
|---|---|
| EX-1 | six named `#[tokio::test]`s in `ingress.rs`; VT-1/VT-2/VT-3 each carry a doc comment naming which ADR-004 alternative they falsify (VT-1: the "last thing the host did" write-on-any-firing alternative; VT-2: the boolean-cleared-by-another-stimulus alternative, F-2; VT-3: none — ADR-004 makes no claim, it holds CD-1's own rule) |
| EX-2 | VT-2's doc comment states the pin (both the T0 exchange and the ingested exchange answered the identical `next_check`, "1 second"), cites `SPEC-001/R-26` and `controller.rs:507-512`, and states in one sentence why: so the two hypotheses disagree about nothing except whether the ingested firing wrote `floor_until` |
| EX-3 | VT-4 (positive) and VT-5 (negative); VT-5's doc comment states explicitly: *"this is what makes SPEC-003/R-15's bound a claim rather than an excuse"* |
| EX-4 | VT-2 pairs its anti-fire window (to T0+2.7s) with a liveness assertion (`invocations>=4`, `LIVENESS_BOUND`) directly below it; VT-6 pairs the flood's implicit anti-fire (`invocations==0` immediately after) with `until(LIVENESS_BOUND, invocations>=1)`. VT-1's tight bound needs no separate anti-fire window — stated in its own doc comment, EX-4 does not apply to a case with no anti-fire window |
| EX-5 | every case's doc comment states what it pinned and why: VT-1 (its one exchange, 300ms, and the resulting scheduled firing, a minute off); VT-2 (T0's exchange and the ingested exchange, both "1 second"; the fourth, a minute off); VT-3 (the first envelope, 300ms; the scheduled firing, a minute off); VT-4 (the priming exchange, a minute off); VT-5 (`@slow-view`'s own 45-minute pin, by the script); VT-6 (the one post-flood exchange, a minute off) |
| VT-1 | `an_ingested_firing_never_writes_the_scheduled_floor` — green; goes **red** under VA-3's Break 1 (`condition did not become true within 2s`) |
| VT-2 | `an_ingested_firing_does_not_advance_the_scheduled_floor` — green; goes **red** under VA-3's Break 3 (`left: 4, right: 3` — the fourth invocation landed before the anti-fire window closed) |
| VT-3 | `a_scheduled_firing_does_not_clear_the_event_floor` — green; goes **red** under VA-3's Break 2 (`a refusal must name a reason` — the second envelope was wrongly accepted). **Repaired mid-phase**: its original form sent the second envelope right after `invocations(&log) >= 2` (proves the scheduled exchange *began*, not that it was *absorbed*) and flaked `engaged`/`too_soon` twice under `just check`'s load — fixed by waiting for that exchange's own rendered `next_check` first, the same pattern PHASE-04/VT-5 already uses (*Learned*, below). Re-verified red under Break 2 after the fix |
| VT-4 | `a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface` — green; asserts `served.controller.frame().diagnostics.lines()` contains a line naming `too_soon` |
| VT-5 | `a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface` — green; asserts no retained diagnostic line contains "was refused" after `@slow-view`'s exchange absorbs |
| VT-6 | `after_a_flood_of_malformed_envelopes_the_host_still_evaluates` — green; `invocations == 0` immediately after the flood, `== 1` (not more) after the liveness wait, every reply `malformed` |
| VA-1 | `just check` **exit 0**, three consecutive clean runs after the VT-3 fix (transcripts `…/scratchpad/check-run-{1,2,3}.txt`, session-local, not durable). Two earlier runs, mid-phase, failed on the pre-existing race the *Learned* entry below describes (once in PHASE-04's own test, once in PHASE-05/VT-3 before its fix) — not a defect in the gate |
| VA-2 | the margin table below |
| VA-3 | three breaks, each isolated (the other two cases stay green under each break) — see below |

**VA-2 — margins**

Times are the whole test binary invocation for that one case (three runs,
`cargo test -- <case> --exact`), reading the harness's own "finished in" line.

| case | elapsed | the bound that governs | margin | kind |
|---|---|---|---|---|
| VT-1 | 470ms | `Duration::from_secs(2)`, a bound chosen specifically to be well inside `MINIMUM_SPACING` | ~4.3x | **exempt** — the same ground as PHASE-04/VT-4 and VT-7: the bound *is* the discriminator (a 5s `LIVENESS_BOUND` would not have caught VA-3 Break 1 — a 3s-floored firing lands comfortably inside 5s) |
| VT-2 | 3.26-3.28s | two: the anti-fire window (to T0+2.7s, exempt on the same ground as VT-1); the paired liveness wait afterward, `LIVENESS_BOUND` 5s over the remaining ~0.3-0.5s to the real firing | anti-fire: n/a; liveness: ~10-15x | mixed — anti-fire exempt, liveness comfortable |
| VT-3 | 3.16-3.17s | `LIVENESS_BOUND` 5s over a wait that *is* `retry_after_ms`, the remaining event spacing (~2.7s) | ~1.85x | **exempt** — the same ground PHASE-04/VT-4 states for its own control wait |
| VT-4 | 150-170ms | `LIVENESS_BOUND` 5s | ~29-33x | liveness |
| VT-5 | 330-360ms | `LIVENESS_BOUND` 5s against `@slow-view`'s 200ms foreground sleep | ~14-15x | liveness |
| VT-6 | 3.15-3.16s | `LIVENESS_BOUND` 5s over the remaining ~2.65s of `MINIMUM_SPACING` after the 500ms flood window | ~1.9x | **exempt** — the same ground PHASE-04/VT-7 states: the firing it waits for *is* `MINIMUM_SPACING` |

No unexempted margin under 10x. **S-3-style concern not reached** (S-3 in
this phase names a different condition; the margin rule from PHASE-04's plan
carries over by the same reasoning and is checked the same way).

**VA-3 — break-and-revert, three breaks**

All three breaks were applied to `controller.rs` **in a scratch copy**,
never committed; each was reverted by restoring the pre-phase file
(`cp`'d back) and confirmed byte-identical (`diff`) before the next break.
`git status`/`git diff --stat crates/goad/src/controller.rs` show no
production change survives this phase.

1. **Break 1 (VT-1's falsifier).** `ingest` additionally took `floor_until`
   and wrote it (`*floor_until = arrived + MINIMUM_SPACING`) alongside
   `event_floor_until` — "the ingested firing writes the scheduled floor
   too". VT-1 **red** (`condition did not become true within 2s`). VT-2
   stayed green (the write only ever *delays*, never advances, so "does not
   advance" does not turn on it). VT-3 also went red as a side effect (the
   scheduled firing this case's first envelope produces was itself delayed
   by the break, landing outside the window VT-3 measures) — recorded, not a
   defect: Break 1 is deliberately broad.
2. **Break 2 (VT-3's falsifier).** The timer arm additionally cleared
   `event_floor_until = tokio::time::Instant::now()` alongside writing
   `floor_until` — "a scheduled firing clears the event anchor". VT-3
   **red** (`a refusal must name a reason` — the second envelope was wrongly
   accepted, so `reason()`'s `unwrap`-style expect panicked for lack of one).
   VT-1 and VT-2 stayed green (isolated).
3. **Break 3 (VT-2's falsifier, S-2).** The floor applied at the exchange
   -completion re-arm site (`controller.rs:707-716`) was made conditional on
   `refusal_re_arms` (`matches!(fired, Fired::Scheduled)`) — "the floor
   applies only when the firing whose outcome is being resolved was itself
   scheduled" — the boolean alternative ADR-004 rejected (F-2). VT-2 **red**
   (`left: 4, right: 3` — the fourth invocation, the scheduled evaluation the
   floor should have been holding back, had already landed before the
   anti-fire window closed). VT-1 and VT-3 stayed green (isolated). **S-2 is
   not reached: the case does discriminate.**

### PHASE-08 — The refusal vocabulary, and the closed reason set

**Entry check:** EN-1 — PHASE-03's exit criteria discharged and `just check`
exit 0 at `48cb744` (PHASE-03's own VA-1, re-verified below before editing).
EN-2 — `bind`, `Ingress`, `Arrival`, `Answer`, `IngressError` exist
(`crates/goad-shell/src/ingress/mod.rs`); `Refusal` carries `Unavailable`,
`Malformed`, `InvalidEnvelope`, `TooLarge`, `TimedOut` with an exhaustive
`reason()`; every read is already bounded (`read_envelope`, both budgets
enforced in one function). The fake judge (`judge()`, `Verdict`) is in
`crates/goad-shell/tests/integration/ingress.rs`. Both hold: proceeding.

**Reading list**

| what | where |
|---|---|
| the phase, entire | `docs/slices/004/plan.md:960-1097` |
| the split rationale and id map (PL-10, PL-11) | `plan.md:960-986` |
| PHASE-03's sheet — reading list, assumptions, findings I inherit | `notes.md` `### PHASE-03`, whole section (this file) |
| PHASE-02's harvested finding — `EnvelopeFault::Malformed` meets `malformed` | `notes.md` Harvest, "`EnvelopeFault` carries one variant..." |
| PHASE-03's harvested finding — the non-blocking drain; this phase must not reintroduce a second blocking wait on the refusal path | `notes.md` Harvest, "Closing an `AF_UNIX SOCK_STREAM` socket..." |
| the reply wire form and reason table (all eight rows) | `design.md` §5.2 `:206-237` |
| order of judgement (shape before state) | `design.md` §5.4 `:407-423` |
| `Refusal`'s full payload list, `reason()`/`Display` split | `design.md` §5.2 `:297-305` |
| draft-spec — this phase's requirements | R-13 `:99`, R-14 `:100`, §6.3 whole `:206-237`, §6.4 bounds (context only, not touched) `:239-260` |
| the module this phase extends | `crates/goad-shell/src/ingress/mod.rs`, whole file — `Refusal`, `reason()`, `Display`, `Wire`, `reply()`, `shape_refusal` |
| `EnvelopeFault::ReservedSource` | `crates/goad-shell/src/ingress/envelope.rs:53-56` — already a distinct variant; only the wire's `reason()` needs to special-case it, per PHASE-03's own Assumptions note that `shape_refusal`'s mapping (`ReservedSource` into `InvalidEnvelope`) is not this phase's to change |
| the fake judge and its fixtures, reused not rewritten | `crates/goad-shell/tests/integration/ingress.rs`, whole file |
| workspace lints bearing on the rounding | `Cargo.toml:143` (`integer_division`, deny), `:165-169` (`as_conversions`, `cast_*`, deny), `:136-137` (`unwrap_used`, `expect_used`, deny) |
| the existing `+1`/`try_from`/`unwrap_or` idiom this phase's rounding reuses | `crates/goad-shell/src/ingress/mod.rs:448-450` (`read_envelope`'s `cap`) |

**Assumptions**

- **`Refusal` gains exactly two new variants, `Engaged` and `TooSoon { retry_after:
  Duration }`** — not a third `ReservedSource` variant. EX-5 lists seven payloads
  total (five PHASE-03's, two this phase's); the eighth wire token,
  `reserved_source`, is `reason()` special-casing
  `InvalidEnvelope(EnvelopeFault::ReservedSource)`, matching `design.md` §5.2's
  own payload list (which names four non-unit payloads, not five) and
  PHASE-03's Assumptions note that `shape_refusal`'s mapping is not this
  phase's to touch.
- **The rounding helper takes an owned `Duration` and is used by both `reply()`
  and `Display for Refusal::TooSoon`**, so the two never compute it
  differently. `checked_add(Duration::from_nanos(999_999))` then
  `Duration::as_millis()` (which truncates internally, in the standard
  library, not in this crate) is the shape the implementer notes specify;
  `u64::try_from` narrows it, `unwrap_or(u64::MAX)` on both the `checked_add`
  fallback and the `try_from` fallback rather than a panic, matching
  `read_envelope`'s own precedent for the same idiom.
- **VT-9's cases construct every non-`too_soon` `Refusal` via the fake judge's
  script**, sending only well-formed envelopes, rather than triggering
  `too_large`/`timed_out` for real. Plan's own words — "No case here waits on
  a bound" — rule out a real `ENVELOPE_DEADLINE` wait; scripting
  `Refusal::TimedOut{after: ENVELOPE_DEADLINE}` (a public constructor) proves
  the same wire fact (this field's absence) without the 500ms cost or the
  bound-wait S-6 risk stated in the plan's Verification preamble.
- **VT-7 scripts `Verdict::Drop` for its three shape-refusal sends**, not
  `Accept`. A dropped `Answer` is `unavailable`'s own trigger (PHASE-03/VT-6);
  scripting it here and getting the shape reason back anyway is a stronger
  assertion of shape-before-state than an `Accept` verdict would be, and
  costs nothing extra.

**STOP conditions** (`plan.md` S-1, S-6 — not softened)

- S-1 — a case cannot be written without a queue, a retry, or a second
  arrival outstanding.
- S-6 — one of PHASE-03's cases goes red.

**Tasks**

- [x] phase sheet written; status set to `in progress`.
- [x] EN-1/EN-2 verified (above).
- [x] `mod.rs` — `Refusal::Engaged`, `Refusal::TooSoon`, `reason()` widened to
      eight arms (`reserved_source` special-cased), `Display`, `source()`
      (EX-5).
- [x] `mod.rs` — `round_up_millis`, `Wire.retry_after_ms`, `reply()` (EX-12).
- [x] doc comments updated to drop the "PHASE-08 completes..." forward
      references now that this phase is the one doing it.
- [x] `tests/integration/ingress.rs` — `Verdict::Refuse(Refusal)`, imports
      (`Refusal`, `EnvelopeFault`, `Duration`), VT-7, VT-8, VT-9 (two cases).
- [x] lint/format after each file; `just check` green; VA-1, VA-4, VA-5.

**Verification — every criterion, discharged**

| id | discharged by |
|---|---|
| EX-5 | `Refusal` gains `Engaged` and `TooSoon { retry_after: Duration }`, beside the five PHASE-03 landed — seven variants total (`crates/goad-shell/src/ingress/mod.rs`). `reason()` widened to eight arms, still exhaustive with no `_` arm: `InvalidEnvelope(EnvelopeFault::ReservedSource)` matches ahead of the general `InvalidEnvelope(_)` arm, so `reserved_source` is a wire token, not a ninth payload. No production code in this phase constructs `Engaged` or `TooSoon` — the only two sites doing so are `tests/integration/ingress.rs`'s `Verdict::Refuse` cases |
| EX-12 | `Wire.retry_after_ms: Option<u64>`, `skip_serializing_if`; `reply()` sets it from `Some(Refusal::TooSoon { retry_after })` only, `None` otherwise. `round_up_millis`: `checked_add(Duration::from_nanos(999_999))` then `Duration::as_millis()` (the truncation is the standard library's, not this crate's arithmetic) then `u64::try_from`, both fallbacks `unwrap_or` rather than a panic — the same shape `read_envelope`'s own `cap` already uses. VT-9's `a_too_soon_reply_carries_retry_after_ms_rounded_up` proves the rounding: 1400.3ms → 1401, not 1400 |
| EX-13 | `reason()`'s `InvalidEnvelope(EnvelopeFault::ReservedSource) => "reserved_source"` arm, ahead of the general `InvalidEnvelope(_) => "invalid_envelope"` arm. `shape_refusal`'s mapping is untouched — `ReservedSource` still becomes `Refusal::InvalidEnvelope(EnvelopeFault::ReservedSource)`; only `reason()` reads it as its own token. VT-7's `reserved` case proves it end to end, off the real wire |
| VT-7 | `the_three_shape_reasons_this_phase_owns_are_read_off_the_wire` — `malformed`, `invalid_envelope` (a non-object top level), `reserved_source`, each read off the reply; scripted `Verdict::Drop` throughout (not `Accept`) so a leak into `unavailable` would have shown; `seen` asserts all three arrivals recorded `Seen::Refused`, none `Seen::Event` |
| VT-8 | `the_reason_token_set_is_closed_at_eight` — one `Refusal` per variant (both `InvalidEnvelope` faces), `.reason()` collected into a `BTreeSet`, compared against a literal eight-string set written in the test, not against any production constant |
| VT-9 | `a_too_soon_reply_carries_retry_after_ms_rounded_up` (the rounding, on a value with a non-zero sub-millisecond remainder) and `retry_after_ms_is_absent_from_every_reason_but_too_soon` (all seven other reasons, each scripted via `Verdict::Refuse` so no case waits on a bound — including `too_large` and `timed_out`, scripted rather than really triggered) |
| VA-1 | `just check` **exit 0**, transcript at `/tmp/claude-1000/-home-david-dev-goad/a10c38f4-3ff2-4c14-924e-3b2377d46bee/scratchpad/phase08-final.txt` (session-local, not durable) |
| VA-4 | `git status --short` after the full suite shows only the three source files this phase touched, no socket file; `find /tmp -maxdepth 1 -iname 'goad-ingress-*'` empty |
| VA-5 | `git diff --stat crates/goad-shell/Cargo.toml` empty — this phase adds no feature and no dependency |

**No STOP condition was reached.** S-1: every VT-9 case scripts its refusal through
the fake judge rather than needing a queue, a retry, or a second arrival
outstanding. S-6: PHASE-03's own 13 cases (VT-1..VT-6, VT-10..VT-14) all still
pass — `ingress::` filtered run showed all 17 (13 PHASE-03's + 4 this
phase's) green before the full-workspace run, and the full-workspace
`integration` binary grew from 71 to 75 tests, all passing.

**Decisions taken during execution**

- **`reserved_source` is `reason()` special-casing one `EnvelopeFault`
  variant, not a sixth `Refusal` payload.** Matches EX-5's own count (seven
  variants) and `design.md` §5.2's payload list (four non-unit payloads, not
  five). `shape_refusal` (PHASE-03's) is untouched, per that phase's own
  Assumptions note.
- **VT-9's absence case scripts all seven non-`too_soon` reasons, including
  `too_large` and `timed_out`, via `Verdict::Refuse` rather than a real byte
  overflow or a real 500ms wait.** The plan's Verification preamble states no
  case in this phase waits on a bound; scripting proves the same wire fact
  (the field's absence) without the cost or the S-6 risk of re-triggering a
  bound PHASE-03 already owns.
- **VT-7 scripts `Verdict::Drop`, not `Accept`, for its three shape-refusal
  sends.** A dropped `Answer` is `unavailable`'s own trigger; getting the
  shape reason back anyway is the stronger assertion that shape precedes
  state, at no extra cost.

**Findings**

- None against the design, draft-spec or plan. `EnvelopeFault::ReservedSource`
  was already a distinct variant from PHASE-02; this phase's whole job was
  `reason()` reading it as its own token, exactly as `design.md` §5.2 and
  `draft-spec.md` §6.3 already stated.

### PHASE-03 — The socket, the bounded read, and the accepted path

**Heading restored at PHASE-07.** This section's own `###` heading was lost in
the edit that landed PHASE-08's sheet (`9d36002` replaced the whole section
instead of inserting ahead of it) — the content below is untouched and was
never missing, only its outline entry. Found by PHASE-07's sweep; recorded here
rather than silently repaired, since it is bookkeeping on this phase's own
declared surface (`notes.md`) and changes no content.

**Entry check:** EN-1 — PHASE-02's exit criteria discharged; `just check`
**exit 0** at `bee5d2f` (transcript:
`/tmp/claude-1000/-home-david-dev-goad/a10c38f4-3ff2-4c14-924e-3b2377d46bee/scratchpad/phase03-baseline.txt`,
session-local). EN-2 — `envelope::normalize` exists
(`crates/goad-shell/src/ingress/envelope.rs:86`), covered by PHASE-02/VT-2..
VT-11. Both hold: proceeding.

**Reading list**

| what | where |
|---|---|
| the phase, entire | `docs/slices/004/plan.md:708-957` |
| PHASE-08, so the split's boundary is clear | `plan.md:960-1097` |
| the split rationale (PL-10, PL-11) | `plan.md:716-733` |
| system model — three parts | `design.md` §5.1 `:85-134` |
| stratum 2 surface — `bind`, `Ingress`, `Arrival`, `Answer`, the constants | `design.md` §5.2 `:239-277` |
| the reply wire form and reason table (this phase's five rows) | `design.md` §5.2 `:198-234` |
| lifecycle — startup order, select ordering, shutdown | `design.md` §5.4, whole section `:391-516` |
| invariants I-1..I-3, A-5 (the mode window), the edge-case table | `design.md` §5.5 `:516-587` |
| draft-spec — this phase's requirements | R-2 `:92`, R-3 `:93`, R-4 `:94`, R-5 `:95`, R-6 `:96`, R-7 `:97`, R-8 `:98`, §6.1 `:171-193`, §6.4 `:273-296` |
| **prior art — the A-1 probe's listener** | `docs/slices/004/ingress-probe.local.rs`, whole file — accept loop shape, the reply bytes (no trailing newline, confirmed by its own byte count), `Arrival`/`oneshot` shape. A stand-in cut to A-1's question: no budgets, no mode, no reclaim |
| **prior art — a byte-bounded read** | `crates/goad-shell/src/backend/process.rs:236-260` (`read_capped`) — `AsyncReadExt::take(limit + 1)` then check `len() > limit`, the exact `indexing_slicing`-clean shape this phase's read reuses for its own byte bound |
| **prior art — one-struct error naming a path** | `crates/goad-shell/src/error.rs` (`ConfigError`), `crates/goad-shell/src/config.rs::ingress_config` — the `EmptyPath` precedent for "unusable value not representable past the boundary" |
| the module this phase extends | `crates/goad-shell/src/ingress/mod.rs` (module decl only, PHASE-02) |
| `EnvelopeFault`, `normalize` | `crates/goad-shell/src/ingress/envelope.rs`, whole file — `Malformed` is PHASE-02's harvested finding, meets the wire's `malformed` reason here |
| the manifest this phase changes | `crates/goad-shell/Cargo.toml:17` (`tokio = { workspace = true }`, no features yet) |
| the allowlist test that must keep passing untouched | `crates/goad-boundary/tests/checks/allowlist.rs:19-27` (`STRATUM_2` already names `tokio`; only features change, not the manifest allowlist itself) |
| workspace lints | `Cargo.toml:74` (`unsafe_code`), `:132-133` (`allow_attributes`/`_without_reason`), `:142` (`indexing_slicing`), `:183` (`pub_use`), `:200` (`future_not_send`), `:80` (`missing_debug_implementations`) |
| test-tier conventions | `crates/goad-shell/tests/integration/main.rs`, `harness.rs` (Display-based diagnostics, not `Debug`) |
| temp-path precedent (no `tempfile`) | `crates/goad-shell/src/config.rs:226`, `tests/support/scripting.rs::marker:35-39` |

**Assumptions**

- **The reclaim's liveness probe is `std::os::unix::net::UnixStream::connect`
  (blocking, momentary, under `bind`'s own synchronous call).** Neither
  `design.md` nor `draft-spec.md` states the mechanism; connect-then-fail is
  the standard idiom for a Unix domain socket (no atomic "is anyone listening"
  syscall exists). **Side effect, accepted rather than defect:** on a **live**
  path (VT-2), this probe connection reaches the *other* host's accept task,
  which reads zero bytes then EOF — refused `malformed` on its side, per
  the edge-case table's "zero bytes, then EOF → malformed" row. VT-2's own
  wording — "the first listener is still serving afterwards — asserted by
  writing an envelope to it and reading a reply, not by inspecting the error
  alone" — anticipates exactly this: the assertion exists to prove the accept
  loop shrugs off a stray connection, which is what the probe produces. Not a
  STOP: no invariant is broken (I-3 holds on both sides), no surface is
  touched beyond this phase's own, and it costs the *other* host one
  diagnostics-surface entry only if it happens to be idle at that moment —
  the same class of stated residue as A-5.
- **`EnvelopeFault::Malformed` maps to `Refusal::Malformed`; every other
  `EnvelopeFault` variant maps to `Refusal::InvalidEnvelope`**, including
  `ReservedSource` — splitting `reserved_source` out to its own wire reason is
  PHASE-08/EX-13, explicitly not this phase's (plan.md's Surfaces note).
- **A raw I/O error mid-read** (not a timeout, not the byte cap — e.g. a genuine
  socket error) is folded into `Refusal::Malformed` rather than a new variant.
  Undocumented in design/draft-spec, and not exercised by any VT case (hard to
  trigger without fault injection); chosen because none of the five variants
  this phase owns fits better and I-3 still holds (always answered, never a
  panic).
- **The reply carries no trailing newline.** The probe's own accept loop
  writes the JSON bytes and closes with none, and P-B's harvested byte count
  (30 bytes for `{"protocol":1,"accepted":true}`) confirms it — "close" is the
  line's terminator, not `\n`.
- **`Refusal::Unavailable` is a unit variant** (no payload) in this phase,
  matching `design.md`'s own payload list, which omits it. Its `Display` text
  is written for what *this phase* constructs it for only (a dropped
  `Answer`); later phases reusing the same variant for their own causes is
  their own scope, not pre-empted here.
- **Channel capacity 1** for the arrivals `mpsc`, matching I-2's "irrelevant
  beyond 1" and the probe's own precedent.

**STOP conditions** (plan.md S-1..S-5, not softened)

- S-1 — a case needs a queue, a retry, or a second arrival outstanding.
- S-2 — the read cannot be framed newline-or-EOF and bounded in both bytes and
  time without a second concurrency dimension.
- S-3 — a VA-2 margin comes in under 10x.
- S-4 — an existing case outside this phase's own goes red.
- S-5 — `set_permissions` cannot set the mode on a bound Unix socket on this
  platform.

**Tasks**

- [x] phase sheet written; status set to `in progress`.
- [x] `Cargo.toml` — add `net`, `sync` features to `goad-shell`'s `tokio`
      entry (EX-1).
- [x] `mod.rs` — constants (`SOCKET_MODE`, and re-declare/keep `ENVELOPE_LIMIT`,
      `ENVELOPE_DEADLINE` here since PHASE-02 declared the module only) (EX-2,
      EX-11).
- [x] `mod.rs` — `IngressError`/`BindFault`, `reclaim`, `bind` (EX-3).
- [x] `mod.rs` — `Ingress`, `Arrival`, `Answer`, `Refusal` (EX-4, EX-9, EX-10).
- [x] `mod.rs` — the accept task: bounded read (EX-7, EX-11), sequential loop
      (EX-8), one reply then close (EX-6).
- [x] `tests/integration/ingress.rs` (new) + `main.rs`'s one `mod ingress;` —
      the fake judge, VT-1..VT-6, VT-10..VT-14.
- [x] lint/format after each file; `just check` green; VA-1..VA-3.

**Verification — every criterion, discharged**

| id | discharged by |
|---|---|
| EX-1 | `crates/goad-shell/Cargo.toml`: `tokio = { workspace = true, features = ["net", "sync"] }`, the only line changed in that manifest (`git diff` confirmed one line); root `Cargo.toml` untouched |
| EX-2 | `mod.rs`: `pub const SOCKET_MODE: u32 = 0o600` with its own doc comment (design.md §5.2 gives the constant no comment of its own; written to state `SPEC-003/R-2` and the A-5 window directly — see Decisions). `ENVELOPE_LIMIT`/`ENVELOPE_DEADLINE` carried over from PHASE-02's stub with their doc comments |
| EX-3 | `bind` — `reclaim(path)?` then `UnixListener::bind` then `set_permissions` then `tokio::spawn(accept_loop(...))`, synchronous; `IngressError { path, fault: BindFault }`, six fault variants naming what was found |
| EX-4 | `Ingress::none()`/`arrival()` (parks on `None` via `std::future::pending`; drops the receiver and parks on a closed channel); `Arrival::into_parts`; `Answer::accepted`/`refused` both consume `self`; a dropped `Answer` yields `unavailable` (VT-6) |
| EX-6 | `Wire` struct + `reply()`, serialized with `serde_json` (not interpolated — a watcher-chosen key name in `detail` must not break the reply's own JSON); no trailing newline (matches the A-1 probe's own harvested byte count); `retry_after_ms` not present anywhere this phase — no `Refusal` this phase constructs carries it |
| EX-7 | `read_envelope`: `read_until(b'\n', …)` over `BufReader::new(stream.take(ENVELOPE_LIMIT + 1))`; a second envelope on one connection is never read (VT-5c) |
| EX-8 | `accept_loop`: `handle(...).await` before the next `listener.accept()`; an `accept()` error `continue`s; the task ends only when `arrivals.send(...)` fails (the channel closed) |
| EX-9 | `cargo clippy --workspace --all-targets -- -D warnings` clean — `future_not_send` and `missing_debug_implementations` are both `deny` and both would have fired |
| EX-10 | `Refusal` — five variants (`Unavailable`, `Malformed`, `InvalidEnvelope`, `TooLarge`, `TimedOut`), `reason()` exhaustive with no `_` arm (confirmed: adding a sixth `PHASE-08` variant will not compile here until this match is extended, which is `PHASE-08`'s to do) |
| EX-11 | `ENVELOPE_LIMIT`/`ENVELOPE_DEADLINE` declared in `mod.rs` with `read_envelope` enforcing both in the same function; VT-13 (bytes), VT-14 (time) |
| VT-1 | `a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves` |
| VT-2 | `a_live_socket_refuses_a_second_bind_and_keeps_serving` |
| VT-3 | `a_regular_file_at_the_path_is_refused_naming_what_was_found` |
| VT-4 | `a_directory_with_no_write_permission_is_refused_naming_the_path` (chosen over "a path component that is not a directory" — see Decisions) |
| VT-5 | three tests: `an_envelope_terminated_by_a_newline_is_accepted`, `an_envelope_terminated_by_closing_the_write_side_is_accepted`, `a_second_envelope_on_the_same_connection_is_never_read` |
| VT-6 | `a_dropped_answer_yields_unavailable_then_a_close` |
| VT-10 | `the_socket_is_owner_only_after_bind`; ambient umask at run time was `0o022` (checked once, outside the test, per the plan's own prohibition on a umask call inside a case) — non-vacuous |
| VT-11 | `a_malformed_envelope_reaches_no_event_and_the_listener_stays_up` |
| VT-12 | `a_well_formed_envelope_reaches_the_judge_as_the_event_it_wrote` |
| VT-13 | `more_than_the_byte_limit_is_refused_too_large_and_the_limit_itself_is_accepted` |
| VT-14 | `a_connection_that_writes_nothing_times_out_and_the_listener_serves_next` |
| VA-1 | `just check` **exit 0**, transcript at `/tmp/claude-1000/-home-david-dev-goad/a10c38f4-3ff2-4c14-924e-3b2377d46bee/scratchpad/phase03-final-check2.txt` (session-local, not durable) |
| VA-2 | VT-14 elapsed, three runs (temporary `eprintln!`, reverted before the final `just check`): **501.87 / 501.71 / 501.82 ms** against `ENVELOPE_DEADLINE` = 500 ms — ratio ≈1.004, far inside the 10x bound. No other case in this phase waits on a bound |
| VA-3 | `git status --short` after the full suite shows no socket file; `find /tmp -iname 'goad-ingress-*'` empty; VT-4's directory removed by the case itself |

**No STOP condition was reached.** S-1: no case needed a queue, a retry, or a
second outstanding arrival — the accept loop is sequential by construction.
S-2: the framing and both bounds are one `read_until` over
`BufReader::new(stream.take(ENVELOPE_LIMIT + 1))` wrapped in one
`tokio::time::timeout`, no second concurrency dimension. S-3: VA-2's ratio is
~1.004, nowhere near 10x. S-4: the full pre-existing suite (`cargo test
--workspace`) stayed green throughout — 35+71+6 tests plus stratum 1's 30+5,
none newly failing. S-5: `set_permissions` on a bound Unix socket worked on
this platform without incident.

**Decisions taken during execution**

- **The reclaim's liveness probe is `std::os::unix::net::UnixStream::connect`.**
  Confirmed as anticipated in the phase sheet's Assumptions: VT-2's probe
  connection lands on the *first* listener as a stray, empty connection,
  refused `malformed` there — harmless, and exactly why the fake judge
  (`judge()`) answers anything past its own script with `accepted` rather than
  asserting an exact arrival count for that case.
- **VT-4 uses a directory with no write permission (`0o500`), not "a path
  component that is not a directory."** The latter makes `std::fs::
  symlink_metadata` itself fail with `ENOTDIR` (not `NotFound`), which would
  route through `BindFault::Unprobeable` rather than exercising `bind()`'s own
  failure — a real fault, but not the one the phase's `Unbindable` variant
  exists for, and not a deterministic choice across platforms. The
  no-write-permission directory reaches `symlink_metadata` = `NotFound` (search
  needs only execute permission), then `UnixListener::bind` itself fails with
  `EACCES` → `Unbindable`, deterministically. **Assumption:** the test
  environment does not run as root (permission checks would be bypassed);
  true here (a Nix devshell, unprivileged user).
- **A raw I/O error mid-`read_until`** is folded into `Refusal::Malformed`, per
  the phase sheet's stated assumption. Not exercised by any test (no fault
  injection available); I-3 still holds regardless (always answered, never a
  panic).

**Findings**

- **A defect in the phase's own design, found and fixed in this phase: an
  unconditional post-refusal drain would have silently doubled
  `ENVELOPE_DEADLINE` for the one case that has nothing to drain.** Closing an
  `AF_UNIX` `SOCK_STREAM` socket while bytes the peer sent are still unread in
  the kernel's receive buffer resets the connection (`ECONNRESET`) rather than
  delivering a graceful close — confirmed empirically: VT-13's `too_large`
  case failed with exactly that error before any drain existed, because our
  own test intentionally over-sends past `ENVELOPE_LIMIT`. Neither
  `design.md` nor `draft-spec.md` nor `plan.md` mentions this; it is a
  transport-level consequence of `SPEC-003/R-7`'s own bound (stopping a read
  early necessarily leaves a writer's excess bytes unread), not a defect in
  those documents' *requirements* — but the plan's `read_capped` prior art
  (`process.rs`) does not need to handle it, because a backend's stdout pipe
  is one-directional and never needs a reply written back on the same
  channel afterward. First fix attempt wrapped the drain in the same
  `tokio::time::timeout(ENVELOPE_DEADLINE, …)` shape `read_envelope` uses;
  measured, this **doubled** VT-14's elapsed time to ~1.0025 s, because a
  silent writer (the common `timed_out` case) has nothing queued and the
  drain then does nothing *but* wait out its own copy of the deadline before
  giving up. Fixed by making `drain` non-blocking (`UnixStream::try_read` in a
  bounded loop, stopping the instant nothing is immediately readable) rather
  than a second bounded wait — costs nothing when there is nothing queued
  (VT-14's own case, re-measured at ~501.8 ms, matching the single-deadline
  figure) and clears `too_large`'s guaranteed leftover in a handful of
  syscalls. **Recorded because a future phase touching this read (there is
  none planned — `PHASE-08` explicitly does not touch the read) should not
  reintroduce a second blocking wait on the refusal path.**
- **VT-2's own wording anticipates the reclaim probe's side effect exactly** —
  see Decisions above. No action needed; confirms the phase sheet's assumption
  rather than contradicting it.
- **A single `just check` failure of VT-1
  (`a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves`),
  reported after the fact with no captured output, could not be reproduced.**
  Chased per the orchestrator's brief (bounded diagnosis session,
  post-`b07576b`). Baseline: 200/200 quiet runs, single case, isolated. Under
  real CPU oversubscription — 128 then 191 busy loops on 32 cores, loadavg
  ramped from ~52 to ~230 (1.6x-7.2x cores, exceeding the 5x/164-170 the
  audit's own margin memory used) — the case was invoked **515** further
  times by five different methods: 60 solo runs at ~120 loadavg; 90 runs
  6-way-parallel at ~165 loadavg (whole `ingress::` module each time); 160
  runs of the full `integration` binary (all 75 cases, default in-process
  thread parallelism) at up to 230 loadavg; and 5 full `cargo test
  --workspace` runs at 210-220 loadavg. **Zero failures of this case across
  all 515 + 200 = 715 invocations.** The same load window did reproduce
  *other*, pre-existing flakes in the same binary —
  `failure_matrix::a_backend_that_never_answers_reaches_the_caller_as_a_timeout`
  (5 times), `transport::a_stdout_flood_is_refused_and_the_backend_sees_the_stream_close`
  (8 times), and two more `transport`/`failure_matrix` cases once each — none
  of them `ingress::`, all of them cases that wait on a real subprocess and a
  real timeout rather than on `bind`/`reclaim`. **No mechanism found, nothing
  changed.** `bind`/`reclaim` (`crates/goad-shell/src/ingress/mod.rs`) is
  synchronous, touches nothing shared across processes (the socket path is
  unique per test-case name and PID), and every liveness check in it —
  `symlink_metadata`, `UnixStream::connect`, `remove_file` — is a single
  syscall with no window for another process to intervene between them that
  128-191 busy loops did not already stress. The honest read is a true
  one-off (a machine hiccup — memory pressure, a scheduler anomaly, page
  cache stall — orthogonal to this test's own logic) rather than a
  reproducible race in `bind`/`reclaim`; the failure_matrix/transport flakes
  found instead are a plausible source of a "concurrent load" report that got
  attributed to the wrong test by proximity in the same `just check` run. Not
  written to `docs/memory/` — there is no durable fact here, only a negative
  result recorded so a future audit does not re-chase it from zero.

### PHASE-01 — The A-1 probe

**Objective:** A-1 is a measurement rather than an assumption. Either the design
stands as written, or the slice stops here.

**State:** **done**, 2026-09-08. **Verdict: A-1 holds.** A `tokio::spawn`ed
accept task delivers a connection to the `slint::spawn_local` future in
**128–207 µs**, and in **243–300 µs** with nothing else armed after a second of
deliberate idle — 334× under S-1's 100 ms red line. `research.md` Thread 3 has
the numbers, the topology and what was *not* measured. PHASE-02 may proceed.

**Reading list**

| what | where |
|---|---|
| the phase, entire | `docs/slices/004/plan.md:451-556` |
| why the probe is alone and first | `docs/slices/004/plan.md:39-46`, `:116-122` |
| **A-1 as stated** | `docs/slices/004/design.md:549-554` (§5.5 Assumptions) |
| **R1** | `docs/slices/004/design.md:642` (§8) |
| the arrangement being measured | `design.md` §5.1 `:85-134`, §5.2 stratum 2 `:239-277`, §5.4 startup order `:393-405` |
| I-2 — one arrival outstanding, the wait for judgement unbounded | `design.md:522` |
| the inner arm's shape (what P-C reproduces) | `design.md:427-452` (§5.4 select ordering) |
| Thread 3, reserved for this result | `docs/slices/004/research.md:243-247` |
| **prior art — the probe** | `docs/slices/003/timer-probe.local.rs`, whole file |
| **prior art — how a result is recorded** | `docs/slices/003/research.md:470-503` (Spike S-1 result) |
| the results-table shape | `docs/memory/tokio-time-runs-under-slints-executor.md` |
| the production arrangement the probe copies | `crates/goad/src/main.rs:47-121` — config → runtime → `runtime.enter()` → window/tray → `spawn_local(serve)` → `run_event_loop_until_quit` |
| the outer and inner `select!` the probe mirrors | `crates/goad/src/controller.rs:409-412`, `:503-514` |
| tokio's features for `crates/goad` (no `net`) | `crates/goad/Cargo.toml:22` |
| `Config` has two fields today (PHASE-02 adds the third) | `crates/goad-shell/src/config.rs:28-31` |
| `Host::evaluate` | `crates/goad-shell/src/host.rs:137` |

**Assumptions**

- `i_slint_backend_testing::init_integration_test_with_system_time()` gives a
  **real** headless Slint event loop on real time — established by spike S-1,
  not re-verified here.
- `crates/goad` already carries tokio's `rt-multi-thread` and `sync`; only
  `net` is missing, and the plan's implementer note permits adding it
  **temporarily** alongside the `[[test]]` stanza, reverted by EX-3.
- The client is a **std thread** running blocking `std::os::unix::net::UnixStream`.
  That is what the slice's client actually is (`slice-004.md` §Non-goals: a shell
  one-liner), and it keeps the client off the runtime being measured.
- The connect instant is written into a shared slot **before** `connect`, so
  write → connect → accept → read → send → recv orders it ahead of every read of
  it. The number therefore includes the `connect` syscall, which can only inflate
  it.
- Constructing a `Tray` under the headless backend logs *Failed to create system
  tray icon*. Noise, not a failure
  (`docs/memory/tokio-time-runs-under-slints-executor.md`).

**STOP conditions** (from `plan.md` S-1, S-2 — not softened)

- **S-1** — any of: an arrival not observed within a small multiple of the
  connect under P-D (**anything over 100 ms with nothing else armed is red**); an
  arrival observed only after an unrelated wake; a hang; a panic; or
  `UnixListener::bind` failing under the runtime guard. On any of these: record
  the measurement and the verdict, report to the orchestrator, **do not**
  improvise a fallback, **do not** proceed to PHASE-02. The two foreseeable
  repairs (a second `slint::spawn_local` accept loop; `slint::invoke_from_event_loop`)
  are **design changes** and go back to the design stage.
- **S-2** — the probe cannot be written without touching `crates/*/src`. Stop.

**Tasks**

- [x] EN-1 — tree at `b6ca5f7` + doc-only changes (`git diff --stat b6ca5f7 -- crates/ Cargo.toml justfile flake.nix examples/ tests/` is empty); `just check` exit 0 before any edit.
- [x] phase sheet written; status set to `in progress`.
- [x] write `docs/slices/004/ingress-probe.local.rs` on the shape of `timer-probe.local.rs` (EX-1).
- [x] temporary `[[test]]` stanza + `net` in `crates/goad/Cargo.toml`.
- [x] run P-A..P-D, capture output verbatim (VT-1..VT-4) — run **three times**, because one sample of a timing claim is not a margin.
- [x] revert `crates/goad/Cargo.toml`; confirm byte-identical (EX-3).
- [x] `just check` exit 0 with the manifest reverted (VA-1).
- [x] `research.md` Thread 3 — output pasted, results table, verdict (EX-2, EX-4).
- [x] verdict line here; Harvest updated; phase `done`.

**Verification — every criterion, discharged**

| id | discharged by |
|---|---|
| EN-1 | `git diff --stat b6ca5f7 -- crates/ Cargo.toml justfile flake.nix examples/ tests/` empty at `658b124`; `just check` **exit 0** before any edit |
| EX-1 | `docs/slices/004/ingress-probe.local.rs` exists on `timer-probe.local.rs`'s shape: header stating the question, the temporary `[[test]]` stanza and the command, one process, four cases, an `Rc<RefCell<Vec<String>>>` report, `quit_event_loop`, and a final `assert_eq!(report.len(), 4)` so a silently-skipped case fails |
| EX-2 | `research.md` **Thread 3 — Spike A-1**: run 1 pasted verbatim, plus a results table in `docs/memory/tokio-time-runs-under-slints-executor.md`'s shape |
| EX-3 | `crates/goad/Cargo.toml` sha256 `7aa0c09a19bb689e2994d6128a19d5e30fdf94317889e2ef0a6310f1274b5965` — identical to its state at EN-1. `git status --short` shows `docs/slices/004/{notes,research}.md` only; `--ignored` shows the probe as the one untracked file, and it is gitignored. `Cargo.lock` content is unchanged |
| EX-4 | the **State** line above, and Thread 3's heading: *A-1 holds* |
| VT-1 (P-A) | arrival observed **via the ingress arm** in 206.6 / 162.3 / 127.8 µs from `connect`, 89 bytes intact |
| VT-2 (P-B) | connect→reply 130.4 / 105.2 / 123.7 µs; **30 bytes** — `{"protocol":1,"accepted":true}` — then close |
| VT-3 (P-C) | arrival observed **before the exchange completed = true**, at 100.4 ms into a 305 ms exchange, connect→arrival 275 / 306 / 337 µs; the exchange resolved with no failure and re-armed `next_check` |
| VT-4 (P-D) | 299.7 / 278.6 / 242.8 µs after **1.0004 s** parked; the report line states in terms that cancel was untripped, the command channel live and silent, the timer parked 3600 s out and no click armed |
| VA-1 | `just check` **exit 0** with the manifest reverted — output below |

```
cargo build --workspace
cargo test --workspace
cargo test -p goad-semantics
deno check examples/typescript/backend.ts
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
EXIT=0
```

(all 16 test binaries green: 138 + 58 + 43 + 30 + 30 + 17 + 15 + 6 + 5 + 5 + 1 + 1
passed, 0 failed, across the two `cargo test` commands.)

**No STOP condition was reached.** S-1's red line is *anything over 100 ms with
nothing else armed*; the worst P-D reading is 299.7 µs, **334× under it**. S-2
did not arise — nothing under `crates/*/src` was touched.


**Decisions taken during execution**

- **The probe binds and spawns the four accept tasks *before* `spawn_local`**,
  under the `EnterGuard` and before `run_event_loop_until_quit` — because that is
  where `design.md` §5.4 puts `ingress::bind` and it is the arrangement A-1 is a
  claim about. Binding inside the future would measure something easier.
- **One socket, one accept task and one client per case**, rather than one socket
  reused. Cases must not be able to observe each other's arrivals, and P-D's
  "nothing else armed" has to be literally true.
- **P-A's and P-D's `select!` reproduce the design's outer arm ordering** —
  `biased; cancel → commands → sleep → ingress` — with the sleep parked an hour
  out and a **live but idle** command sender. A dropped sender would make the
  commands arm resolve `None` immediately and win the biased race forever, which
  would measure nothing.
- The reply line is the design's own (`design.md` §5.2), so P-B's byte count is
  the real one and not a stand-in.

**Findings**

- **The wake path A-1 needs was already exercised by spike S-1, but not the
  part that matters.** S-1's timer completed because tokio's timer driver — on a
  runtime thread — woke a `spawn_local` future's waker across threads. So the
  *wake* was cross-thread already. What A-1 adds is that a `tokio::spawn`ed
  **task** runs at all while the main thread is inside Slint's loop, and that the
  reactor polls a `UnixListener` there. The probe is a measurement of the runtime
  being driven, not of the waker.

- **Nothing surprising surfaced against the design.** P-C put an arrival into
  the inner `select!` 100 ms into a 305 ms exchange and the exchange still
  resolved, which is the shape §5.4's inner arm assumes. No STOP condition was
  reached and no design defect was found.

- **`Cargo.lock` goes stat-dirty but not content-dirty** when tokio's `net`
  feature is added and removed: `socket2` is pulled in for the build and the
  lockfile is rewritten identically. `git status` reports it modified until the
  next `git diff` refreshes the index. Worth knowing before someone reverts a
  file that never changed.

### PHASE-06 — Binding at startup, and the demo a person runs

**Entry check:** EN-1 — PHASE-05's exit criteria are discharged (its sheet
above, every criterion with evidence) and `just check` **exit 0** at `623e2f0`,
re-run before anything was edited. Holds: proceeding.

**Reading list**

| what | where |
|---|---|
| the phase, entire | `plan.md` PHASE-06 |
| the amended startup order, `Host::new` below the bind | `design.md` §5.4 "Startup" |
| AC-8, AC-9, AC-13 and AC-9's reading | `slice-004.md` §Acceptance criteria, §Readings taken in design |
| §9's AC-8/AC-9 rows | `design.md` §9 |
| R-2, R-3, the bind race, the path-after-bind residue | `draft-spec.md` §6.1 |
| `main::start`, the numbered comment blocks, `Host::new` at `:54` | `crates/goad/src/main.rs:47-125` |
| `StartupError`'s eight variants, `Display`, no `PartialEq` | `crates/goad/src/startup.rs:21-65` |
| the module's own no-exit-code rule | `crates/goad/tests/renderer/startup.rs:1-11` |
| `report_startup_line` | `crates/goad/src/diagnostics.rs:339-346` |
| `bind`, `IngressError`, `BindFault`, `Ingress::none()` | `crates/goad-shell/src/ingress/mod.rs:55-90, 127-138, 202-211` |
| `IngressConfig` | `crates/goad-shell/src/config.rs:89-92` |
| `Host::new` consumes `Config` | `crates/goad-shell/src/host.rs:127-134` |
| `serve`'s seventh parameter | `crates/goad/src/controller.rs:569-577` |
| the demo backend to modify | `examples/shell/backend.sh` |
| the demo config | `examples/demo.toml` |
| `socat`/`deno` in the dev shell | `flake.nix:56-61, 118-122` |
| `.gitignore`'s existing scratch rule | `.gitignore:1-8` |
| `Event`'s field order | `crates/goad-semantics/src/protocol/canonical.rs:490-496` |
| the host serialises compactly | `crates/goad-shell/src/backend/process.rs:63` |

**Assumptions**

- `listener(None)` returns `Ok(Ingress::none())` and touches nothing — no
  `bind` call, no directory entry (VT-2/AC-7 second half).
- `listener(Some(cfg))` calls `ingress::bind(&cfg.path)` and maps its `Err`
  through `StartupError::Ingress`.
- `main.rs`'s step 1 keeps `config`, `now` and `backend` construction but stops
  short of `Host::new`; the bind becomes its own numbered step 2, `Host::new`
  moves to (new) step 3, and the runtime/guard step is renumbered ahead of the
  bind. Comment prose for step 1 loses the "the config is then moved into the
  host" clause.
- `examples/shell/backend.sh`'s `"source":"host"` arm must precede the
  catch-all — the file's existing `case` already has the `respond` arm first,
  so the ingested-event arm slots in as the new middle case, catch-all last.
- The one-liner is `socat` per EX-6/S-2 — no `/tmp`, no second config, no
  `deno eval`, per D-17.

**STOP conditions** (`plan.md` S-1..S-4)

- S-1 — bind cannot go where EX-3 puts it.
- S-2 — the one-liner does not work from a clean clone with `socat` alone.
- S-3 — VH-1's run shows something else visibly wrong.
- S-4 — VH-1 step 3 shows no change at all.

**Task breakdown**

- [x] sheet written; status `in progress`.
- [x] `StartupError::Ingress(IngressError)` + `Display` + `source()`.
- [x] `startup::listener`.
- [x] `main.rs` re-sequenced: bind after the guard, `Host::new` below it,
      `serve` passes the real `Ingress`.
- [x] `flake.nix` gains `pkgs.socat`.
- [x] `examples/demo.toml` gains `[ingress]`, header comment carries the
      `socat`/`nc` one-liners.
- [x] `.gitignore` gains the demo socket.
- [x] `examples/shell/backend.sh` names `source`/`kind` for an ingested event.
- [x] VT-1, VT-2, VT-3 in `crates/goad/tests/renderer/startup.rs`.
- [x] `just check`.
- [x] VA-2 clean-clone run, VA-3 review argument, both pasted below.
- [x] runbook written; VH-1 handed to the user, left open.
- [x] Harvest updated; status `done` (VH-1 excepted).

**Decisions taken while executing**

- `main::start`'s numbered steps went from 7 to 9: step 1 stops short of
  `Host::new` (keeps only `Config::load`, the clock, the backend); step 2 is
  the runtime guard, unchanged; the bind is a new step 3; `Host::new` is a new
  step 4; the former steps 3-7 (components, bridge, glass, enqueue, task) are
  renumbered 5-9. No gap, per the plan's instruction.
- `StartupError::Ingress`'s `Display` is `write!(f, "{error}")`, matching
  `Config`'s and `Clock`'s arms — `IngressError`'s own `Display` already names
  the path (`"{path}: {fault}"`), so no second prefix is added.
- EX-1's "a `source()` arm": `StartupError` overrides no `source()` (its own
  doc comment already says "the **default** `source()`"), so there is no match
  to add an arm to. Read as: the ninth variant stays covered by the same
  always-`None` policy `source_walk`'s test enforces (F-47) — extended that
  test with an `Ingress` case rather than adding an override, which would have
  reopened the double-print risk the design deliberately closed.
- `backend.sh`'s ingested-event arm reads `source`/`kind` by parameter
  expansion exactly as the implementer notes describe
  (`${request#*'"source":"'}` / `%%'"'*`), verified directly against the real
  wire bytes (below) rather than assumed from the field-order argument alone.
- `.gitignore`'s entry is `/goad-demo.sock` (anchored to the repo root, where
  `examples/demo.toml`'s relative `[ingress]` path resolves once `just` sets
  goad's working directory there) rather than a path under `examples/`.

**Verification — every criterion, discharged**

| id | discharged by |
|---|---|
| EX-1 | `StartupError::Ingress(IngressError)`, `Display` arm, `source()` covered by the default and asserted in `source_walk`; `main`'s `match run()` (pasted below, VA-3) maps it to exit 2 unchanged |
| EX-2 | `startup::listener` — `None` → `Ingress::none()`, `Some` → `ingress::bind` wrapped; `main::start` is its only caller |
| EX-3 | bind sits at step 3, after `runtime.enter()` (step 2) and before `Host::new` (step 4) and everything UI (steps 5-9); `Host::new` reads `config.ingress` immediately before consuming `config` — `crates/goad/src/main.rs:47-71` |
| EX-4 | `serve(...)`'s seventh argument is `ingress`, the real value; the only change to that call site |
| EX-5 | `flake.nix:60` — `pkgs.socat` beside `pkgs.deno` in `projectPkgs`, reaching both the dev shell and the jails |
| EX-6 | `examples/demo.toml` gains `[ingress]` with `path = "./goad-demo.sock"`, and its header comment carries both the `socat` and `nc` one-liners verbatim, with a real `source`/`kind`/`timestamp`/`data` |
| EX-7 | `.gitignore` gains `/goad-demo.sock` |
| EX-8 | `examples/shell/backend.sh`'s middle `case` arm (`"source":"host"`) keeps the fixed prompt; the new catch-all extracts `source`/`kind` by parameter expansion and names them in the view's title; verified directly (below) against `evaluate` requests built the way the real host builds them |
| VT-1 | `startup::listener::some_path_binds` (positive) and `::some_path_that_is_a_regular_file_names_the_path` (negative, `StartupError::Ingress`'s `Display` asserted to contain the path) |
| VT-2 | `startup::listener::none_binds_nothing` — `Ok`, and a watched directory gains no entry across the call |
| VT-3 | `startup::display_text::ingress_is_unwrapped_and_unprefixed_and_names_the_path` (the `Display` half) and `startup::stderr_outlets::report_startup_line_renders_ingress_like_its_siblings` (the `goad: {error}` half) |
| VA-1 | `just check`, exit 0 — transcript `…/scratchpad/check-final3.txt`, session-local |
| VA-2 | the clean-clone run, below |
| VA-3 | the exit-code argument, below |
| VH-1 | **not discharged by this agent** — see the runbook. Left open. |

**Break-and-revert — retroactive red confirmation**

These tests were written after their implementation, not red-first (flagged in
the original report). Confirmed retroactively, PHASE-05's style: `startup.rs`
copied to a scratch file first (never `git stash`); each break applied,
`cargo test -p goad --test renderer startup::…` run, then the scratch copy
restored and diffed byte-identical before the next break. All four breaks
reverted; `just check` re-run **exit 0** after the last restore
(`…/scratchpad/check-post-revert.txt`, session-local) and `git status --short`
clean against the `7caedfd` commit — the restored file is exactly what shipped.

| break | changed | predicted | observed |
|---|---|---|---|
| 1 | `listener`'s `Some` arm: `Ok(Ingress::none())` instead of calling `ingress::bind` | `some_path_that_is_a_regular_file_names_the_path` red; `some_path_binds` **stays green** | confirmed both ways — `some_path_that_is_a_regular_file_names_the_path` panicked (`unwrap_err()` on `Ok(Ingress { arrivals: None })`); `some_path_binds` passed unchanged |
| 2 | `listener`'s `None` arm: `Err(StartupError::Enqueue)` instead of `Ok(Ingress::none())` | `none_binds_nothing` red | confirmed — panicked `Err(Enqueue) was not Ok` |
| 3 | `StartupError::Ingress`'s `Display`: `"ingress error: {error}"` instead of `"{error}"` | `display_text::ingress_is_unwrapped_and_unprefixed_and_names_the_path` red; `stderr_outlets::report_startup_line_renders_ingress_like_its_siblings` **stays green** (it computes its own expectation from the same live, broken `Display`, so it can only ever test that `report_startup_line` is generic over the variant — not the variant's own text; a different, correctly-insensitive claim) | confirmed both ways — the first failed `left: "ingress error: …" right: "…"`; the second passed unchanged |
| 4 | `StartupError`'s `Error` impl: overridden `source()` leaking `Ingress`'s inner error, instead of the type's documented default (`None` for every variant, deliberately — no chain walk, so a message is never rendered twice) | `source_walk::startup_error_source_is_always_none`'s new `Ingress` case red | confirmed — panicked on `.source().is_none()` |

**Finding: `some_path_binds` was vacuous under Break 1.** It asserted only
`result.is_ok()`, and `Ingress` exposes no way to distinguish a bound handle
from `Ingress::none()` from outside the crate (its `arrivals` field is
private) — so a `listener` that silently stopped binding and always returned
`Ingress::none()` passed this test. `some_path_that_is_a_regular_file_
names_the_path` and `none_binds_nothing` are not vacuous (both went red
exactly as predicted); this was specific to the one positive case.

**Patched, and re-proved.** `some_path_binds` now also asserts the filesystem
entry `bind` leaves behind: `std::fs::metadata(&path).file_type().is_socket()`
(`std::os::unix::fs::FileTypeExt`), which `Ingress::none()`'s path never
produces. Break 1 re-applied against the patched test, same procedure (scratch
copy, byte-diff restore):

| break | changed | predicted | observed |
|---|---|---|---|
| 1 (re-run, against the patched assertion) | `listener`'s `Some` arm: `Ok(Ingress::none())` instead of calling `ingress::bind` | `some_path_binds` **now red** too | confirmed — panicked `"…/goad-startup-some-….sock" was not created: No such file or directory (os error 2)`; `some_path_that_is_a_regular_file_names_the_path` red as before |

Restored from the same scratch copy, diffed byte-identical to the `7caedfd`
commit again. `just check`: one run hit a **pre-existing, unrelated** flake —
`goad-shell`'s `ingress::a_stale_socket_with_no_listener_is_reclaimed_and_the_
new_one_serves` failed under concurrent load (`in use by a live host` on a
path nothing else should be racing), passed in isolation
(`cargo test -p goad-shell --test integration
a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves`), and
passed again on a clean re-run of the whole gate. Not touched by this phase's
surfaces (`crates/goad-shell/src/ingress/` is *Must not touch*); noted rather
than chased. Final run: **exit 0**.

**VA-2 — the clean clone**

A separate clone (`git clone` of this checkout at `623e2f0`, working tree
copied over it so this phase's changes were present) in `nix develop`:

```
$ which socat
/nix/store/…-socat-1.8.1.3/bin/socat
$ cargo build -p goad --bin goad
   Compiling … (26 workspace crates)
    Finished `dev` profile [unoptimized] target(s) in 26.21s
```

`just demo`'s equivalent (`cargo run -p goad --bin goad -- examples/demo.toml`)
started, bound `goad-demo.sock` with mode `srw-------` (owner-only, AC-10), and
the documented `socat` one-liner produced the wire's real replies — not
guessed at, run:

```
$ printf '%s' '{"source":"reddit-watcher","kind":"reddit-opened","timestamp":"2026-08-22T17:10:00+10:00","data":{}}' \
    | socat - UNIX-CONNECT:./goad-demo.sock
{"protocol":1,"accepted":true}

# a second envelope while the first exchange (the demo backend's own process)
# was still in flight:
{"protocol":1,"accepted":false,"reason":"engaged","detail":"an exchange was already in flight"}

# malformed:
$ printf '%s' 'not json' | socat - UNIX-CONNECT:./goad-demo.sock
{"protocol":1,"accepted":false,"reason":"malformed","detail":"the bytes are not one JSON document"}
```

The process was then killed and the socket removed. **This is not VH-1's
evidence** — it is a headless check of the socket's plumbing (bind, mode,
accept, reply), run to discharge VA-2's own claim that the one-liner works
from a clean clone. It does not touch what the window shows, which is the one
thing reserved for the person who runs VH-1.

**Disclosure:** this environment has a live `DISPLAY`/`WAYLAND_DISPLAY`, so
`cargo run -p goad --bin goad` opened a real window on the machine's actual
screen for the few seconds the check above ran, before being killed — the
renderer test suite never does this (it installs Slint's testing backend), but
a real `cargo run` of the binary does. I did not look at what the window
showed and drew no conclusion from it; VH-1 stays open regardless. Flagging
this because it is an outward-facing effect I did not ask about first.

**VA-3 — the exit code, held by review**

`crates/goad/src/main.rs:21-29`, unchanged by this phase:

```rust
fn main() -> ExitCode {
  match run() {
    Ok(()) => ExitCode::SUCCESS,
    Err(error) => {
      diagnostics::report_startup(&error); // "goad: {error}" on stderr
      ExitCode::from(2)
    }
  }
}
```

One `match` over `run()`'s `Result`, mapping **every** `Err(error)` —
`StartupError`'s ninth variant included — to `ExitCode::from(2)`. No variant
can reach a different code without this `match` itself changing, and it did
not: `git diff main.rs` touches only `start`, never `main` or `run`.

**No STOP condition was reached.** S-1: the bind sits exactly where EX-3 puts
it; nothing about the reactor guard blocked it. S-2: `socat`'s form worked
unmodified from a clean clone with no quoting trick. S-3/S-4: not reachable by
this agent — VH-1's run has not happened yet; the runbook below is what a
person needs to check them.

### PHASE-07 — The sweep, the spec's own table, and the gate

**Entry check:** EN-1 as `plan.md` states it reads *"PHASE-06's exit criteria
are discharged, including VH-1, and `just check` exits 0."* **VH-1 is not
discharged** — it is a person's run, recorded in the Runbook above, and it has
not happened. This is a literal conflict between the plan's own EN-1 text and
the orchestrator's brief for this phase, which states in terms that VH-1 "is
not yours [to discharge]," must not be inferred from a green gate, and stays
open through this phase. Both cannot be honoured at once by rewriting neither:
I am not marking VH-1 discharged (it is not, and nothing in this phase can make
it so), and I am not treating EN-1 as silently satisfied. Per the orchestrator's
explicit brief — which is the delegating authority for this phase and pre-empts
a plan clause it identifies as conditional on the point — I am proceeding with
PHASE-07's work with **VH-1 named open** throughout, and reporting the
conflict rather than resolving it unilaterally. `just check` **exits 0** on the
working tree at entry (transcript below), which is the other half of EN-1 and
is satisfied outright.

**Reading list**

| what | where |
|---|---|
| the phase, entire | `plan.md:1869-1985` |
| `docs/AGENTS.md` §Phase plan, §Execute, §Tiers | `docs/AGENTS.md` |
| `CLAUDE.md` — the five invariants, `just check`, the four instruments | repo root |
| `draft-spec.md` §7, as it stood at entry (prose naming AC ids) | `draft-spec.md:296-321` |
| every phase's sheet, Findings and Verification table | this file, `### PHASE-01` through `### PHASE-06` above |
| the Harvest, whole | this file, `## Harvest` below |
| `canon-delta.md` CD-1..CD-3 | `canon-delta.md` |
| `docs/policy/001-the-phase-gate.md` §Verification, the counting rule | policy, whole |
| the plan's two Coverage tables | `plan.md:398-445` |
| the three seeded design amendments | `design-log.md`, 2026-09-08 *"the design is amended before implementation…"* |

**Assumptions**

- **The clean-clone gate is run at `HEAD` (`99abac4`), not with this phase's
  own uncommitted edits copied over it**, unlike PHASE-06's VA-2. This phase's
  changes are documentation only (`draft-spec.md` §7, `notes.md`, `research.md`)
  and touch nothing `just check` runs; PHASE-06's precedent for copying the
  working tree over the clone existed because that phase's *code* changes were
  what VA-2 had to prove worked from a clean checkout. What VA-1 here is
  actually guarding against — a file real only in the working tree, or a
  `.gitignore` entry hiding one that should not be — is a fact about the
  **committed** tree, which `99abac4` already is.
- **EX-1's "replacing the prose that names an AC id"** is read as: keep the
  descriptive clause of each row (what is tested and why), and replace the
  trailing `(AC-n, …)` citation with the concrete test function(s) and file(s)
  that discharge it — not strip the row to a bare citation list. A row that
  named a test with no description would be harder to audit, not easier.
- **EX-3's "every criterion id names a test that exists and passes"** is
  discharged by (a) `grep`-confirming every cited function exists at the name
  and file the phase sheets claim, and (b) the clean-clone `just check`
  exiting 0, which runs the whole suite — a full green run is stronger evidence
  of "passes" than re-running each case individually by `--exact`, since it is
  the same evidence the gate itself certifies phases on.

**STOP conditions** (`plan.md` S-1..S-4, not softened)

- S-1 — a Coverage row has no test.
- S-2 — the clean clone fails where the working tree passes.
- S-3 — a phase touched a path it did not declare.
- S-4 — EX-7 turns up a departure from `design.md` that no phase's criterion
  authorised.

**Tasks**

- [x] sheet written; status set to `in progress`.
- [x] EN-1 verified (with the VH-1 conflict recorded above, not silently
      resolved either way).
- [x] `draft-spec.md` §7 rewritten, every row naming a real test (EX-1); R-5
      left untouched (EX-2).
- [x] Coverage walk: every `plan.md` Coverage-table id cross-checked against
      the tree by name and by a green full-suite run (EX-3).
- [x] margin table compiled from PHASE-03/04/05's own VA-2 sections, plus
      F-15's numbers (EX-4).
- [x] Harvest updated in place (EX-5, below).
- [x] `research.md` refreshed — F10, F12 (EX-6); Thread 3 checked and found
      already accurate, nothing to refresh.
- [x] `## Design drift` section written (EX-7).
- [x] `just check` on the working tree, and the clean-clone gate (VA-1).
- [x] AC-11 walked instrument by instrument over the finished tree, plus the
      residue argument confirmed directly (`cargo tree -p goad-semantics -i
      tokio` finds nothing) (VA-2).
- [x] `git diff --stat b6ca5f7` walked against every phase's declared surfaces
      (VA-3).
- [x] vocabulary word list re-read against the new module's names (VA-4).
- [x] the PHASE-03 heading gap found and restored (bookkeeping, this phase's
      own surface).

**Verification — every criterion, discharged**

| id | discharged by |
|---|---|
| EX-1 | `draft-spec.md` §7: all sixteen rows (R-1..R-16) name real test functions and files, cross-checked against the tree by `grep`; R-5 untouched |
| EX-2 | R-5's row reads, verbatim, exactly what it read before: *"**review, not a test.** The absence of an unlink cannot be asserted without asserting the absence of code; R-3's reclaim test is what makes the absence safe"* |
| EX-3 | both `plan.md` Coverage tables walked; every named test exists (grep-confirmed) and passes (clean-clone `just check`, exit 0). One coverage residue found, not a gap in a Coverage row — see *Findings* below |
| EX-4 | the margin table, below |
| EX-5 | Harvest updated in place, below |
| EX-6 | `research.md` F10 and F12 refreshed to state the finished tree; Thread 3 checked line by line against the Harvest's own A-1 numbers and found already accurate — no edit needed there |
| EX-7 | `## Design drift` section below, seeded with the three amendments `design-log.md` already names, plus one further item this phase's own walk found |
| VA-1 | working tree: `just check` **exit 0** (transcript `…/scratchpad/phase07-working-tree-check.txt`, session-local). Clean clone of `99abac4` into a scratch directory, `nix develop -c just check`: **exit 0**, all six commands present in order, 19 `test result: ok` blocks, zero `FAILED` (transcript `…/scratchpad/phase07-clean-clone-check.txt`, session-local) |
| VA-2 | stated one instrument at a time, not merged (POL-001 §Verification): crate-edge rule — `cargo build --workspace` succeeds, which is what the rule is (Cargo resolution, not a separate command); manifest allowlist — `allowlist::the_real_stratum_1_manifest_is_clean`, `allowlist::the_real_stratum_2_manifest_is_clean`, both `ok` in the clean-clone run; stratum 1 purity scan — `purity::the_real_stratum_1_source_names_none_of_the_nine`, `ok`; `cargo test -p goad-semantics` — 30 unit + 5 protocol + 0 doc, all `ok`, run separately in both the working-tree and clean-clone gates; domain-vocabulary scan, separately — `vocabulary::no_workspace_member_names_the_users_domain`, `vocabulary::no_member_manifest_names_the_users_domain_in_its_own_crate_name`, both `ok`. Residue argued and confirmed directly: `cargo tree -p goad-semantics -i tokio` reports no matching package — `net`/`sync` (`crates/goad-shell/Cargo.toml:17`) do not reach stratum 1's graph, so POL-001's residue clause is discharged by an argument that is also now checked, not merely asserted |
| VA-3 | `git diff --stat b6ca5f7` — 26 files under `crates/`, `examples/`, `flake.nix`, `.gitignore`, `tests/support/`, `Cargo.lock`. Every one matches a phase's declared Surfaces line (walked file by file, below). `Cargo.lock`'s 11-line diff (`socket2` entering the dependency graph) is the mechanical consequence of PHASE-03/EX-1's declared manifest change, not an undeclared surface. **No undeclared path found** |
| VA-4 | `crates/goad-boundary/tests/checks/vocabulary.rs:18-25`'s `DOMAIN` list — `habit`, `streak`, `journal`, `site`, `goal`, `reminder`, `compliance` — re-read against `event`, `envelope`, `source`, `kind`, `listener`, `ingress`, `watcher`, `arrival`, `refusal`: none present. The scan's own test (`no_workspace_member_names_the_users_domain`) passed in both gate runs |

**VA-3, walked** — every path in `git diff --stat b6ca5f7` against the phase
that declared it (`plan.md`'s Surfaces lines, cross-checked against each
phase's own sheet):

| path | declared by |
|---|---|
| `.gitignore` | PHASE-06 |
| `Cargo.lock` | mechanical consequence of PHASE-03/EX-1 (`crates/goad-shell/Cargo.toml`'s feature change); not itself a named surface in any phase, and not source |
| `crates/goad-semantics/src/error.rs` | PHASE-02 (bounded to two edits, D-18) |
| `crates/goad-shell/Cargo.toml` | PHASE-03/EX-1 |
| `crates/goad-shell/src/config.rs` | PHASE-02 |
| `crates/goad-shell/src/error.rs` | PHASE-02 |
| `crates/goad-shell/src/ingress/envelope.rs` | PHASE-02 (new), PHASE-03/PHASE-08 (conditional) |
| `crates/goad-shell/src/ingress/mod.rs` | PHASE-02 (module decl), PHASE-03, PHASE-08 |
| `crates/goad-shell/src/lib.rs` | PHASE-02 |
| `crates/goad-shell/tests/integration/ingress.rs` | PHASE-03 (new), PHASE-08 |
| `crates/goad-shell/tests/integration/main.rs` | PHASE-03 (bounded to one `mod ingress;`) |
| `crates/goad/src/controller.rs` | PHASE-04 |
| `crates/goad/src/diagnostics.rs` | PHASE-04 |
| `crates/goad/src/main.rs` | PHASE-04 (call site), PHASE-06 (startup order) |
| `crates/goad/src/startup.rs` | PHASE-06 |
| `crates/goad/tests/event_loop/closing.rs` | PHASE-02 (`ingress: None`), PHASE-04 (call site) |
| `crates/goad/tests/event_loop_schedule/scheduling.rs` | PHASE-02, PHASE-04 |
| `crates/goad/tests/renderer/ingress.rs` | PHASE-04 (new), PHASE-05 (extends) |
| `crates/goad/tests/renderer/main.rs` | PHASE-04 (bounded to `mod` decl + two doc sentences) |
| `crates/goad/tests/renderer/scheduling.rs` | PHASE-02, PHASE-04 |
| `crates/goad/tests/renderer/startup.rs` | PHASE-06 |
| `crates/goad/tests/renderer/wiring.rs` | PHASE-04 |
| `examples/demo.toml` | PHASE-06 |
| `examples/shell/backend.sh` | PHASE-06 |
| `flake.nix` | PHASE-06 |
| `tests/support/driving.rs` | PHASE-02 |

`docs/slices/004/*` is excluded from this table — every file there is this
slice's own working authority and is declared by the slice itself, not by an
individual phase's Surfaces line.

**VA-2 — margin table (EX-4)**

Collected from PHASE-03/VA-2, PHASE-04/VA-2 and PHASE-05/VA-2, at the bound
that governs each case (`docs/memory/timed-test-margins-are-measured-at-the-bound.md`).
"Exempt" cases are ones whose wait chosen *is* the bound under test — a margin
under 10x there is the point, not a defect (D-5 rejected a configurable spacing
for exactly this reason: no test may buy margin by moving a bound).

| phase/case | elapsed | bound | ratio | kind |
|---|---|---|---|---|
| PHASE-03/VT-14 | 501.87 / 501.71 / 501.82 ms | `ENVELOPE_DEADLINE` 500 ms | ~1.004x | **exempt** — the wait *is* the bound R-7 states |
| PHASE-04/VT-1 | 160-170 ms | `LIVENESS_BOUND` 5 s | ~30x | liveness |
| PHASE-04/VT-2 | 162-169 ms | `LIVENESS_BOUND` 5 s | ~30x | liveness |
| PHASE-04/VT-3 | 368-378 ms | `@slow-view`'s 200 ms foreground sleep, vs. a 314-490 µs connect-to-reply | ~400-640x | liveness |
| PHASE-04/VT-4 | 3159-3177 ms | `LIVENESS_BOUND` 5 s over a wait that *is* `MINIMUM_SPACING` | ~1.6x | **exempt** |
| PHASE-04/VT-5 | 671-682 ms | 500 ms window (chosen, not a bound) + `LIVENESS_BOUND` 5 s settle, returned <5 ms | >1000x on the settle | liveness (plus a constructed window) |
| PHASE-04/VT-7 | 3163-3167 ms | 500 ms anti-fire window (equality, no ratio) + `LIVENESS_BOUND` 5 s for the scheduled firing at `MINIMUM_SPACING` | liveness ~1.7x | **exempt** on the anti-fire side |
| PHASE-05/VT-1 | 470 ms | `Duration::from_secs(2)`, chosen inside `MINIMUM_SPACING` | ~4.3x | **exempt** — the bound is the discriminator |
| PHASE-05/VT-2 | 3.26-3.28 s | anti-fire window (exempt) + `LIVENESS_BOUND` 5 s over the remaining ~0.3-0.5 s | ~10-15x on the liveness side | mixed |
| PHASE-05/VT-3 | 3.16-3.17 s | `LIVENESS_BOUND` 5 s over a wait that *is* `retry_after_ms` (~2.7 s) | ~1.85x | **exempt** |
| PHASE-05/VT-4 | 150-170 ms | `LIVENESS_BOUND` 5 s | ~29-33x | liveness |
| PHASE-05/VT-5 | 330-360 ms | `LIVENESS_BOUND` 5 s, vs. `@slow-view`'s 200 ms foreground sleep | ~14-15x | liveness |
| PHASE-05/VT-6 | 3.15-3.16 s | `LIVENESS_BOUND` 5 s over the remaining ~2.65 s of `MINIMUM_SPACING` after the 500 ms flood | ~1.9x | **exempt** |

No unexempted margin under 10x anywhere in the slice.

**The numbers F-15 asked for** (PHASE-04/VT-5, measured over a 500 ms flat-out
window with the loop idle): **845 refusals, 845 presentations — 1.000 per
refusal, ~1690 presentations per second.** `assert_eq!(cost, replies.len())`
fixes the cost at one; a later change that raised it, or added a second route
to the diagnostics surface, fails there rather than in front of a person.

**Findings**

- **F-1 — the wire's `unavailable` reason for the ingress-stopped cause is a
  hand-written literal, not read off `Refusal::reason()`, and no test checks
  it against the closed eight-token set the way VT-8 checks the other
  seven-plus-one.** `controller.rs:446-455`'s `ingress_stopped()` builds
  `Refused::Ingress { reason: "unavailable".to_owned(), … }` directly — the
  **only** other construction site (`controller.rs:420-421`) derives `reason`
  from `refusal.reason()`. This is `design.md` §5.2's own design (*"It folds
  one `Refused::Ingress` onto the diagnostics surface — reason `unavailable`,
  detail naming that ingress has stopped"*) — not a departure — because the
  ingress-stopped cause answers no envelope and so has no `Refusal` value to
  construct (`UnavailableCause`'s own doc comment says so in terms). The gap is
  narrower than a Coverage row with no test: `PHASE-08/VT-8`
  (`the_reason_token_set_is_closed_at_eight`) correctly and completely closes
  `Refusal`'s own eight-token set, and this literal is not `Refusal`'s to
  cover. What is missing is an independent check that the literal a person
  reading the diagnostics line sees (via `Refused::Ingress { reason, detail }`
  → `diagnostics.rs:158-160`'s `"({reason}): {detail}"`) is one of those eight
  tokens and not a typo — nothing today would fail if it read `"unavaliable"`.
  `PHASE-04/VT-7` (`a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_
  the_host_evaluating`) asserts the line `contains("ingress has stopped")` —
  `detail`, not `reason` — so it would not catch this either. **Not repaired
  here**: `crates/goad/src/controller.rs` is outside this phase's declared
  surfaces (`draft-spec.md` §7 and `notes.md`/`research.md` only), and this is
  exactly the class of defect this phase is told to report rather than fix on
  its own initiative. Reported to the orchestrator.
- **The PHASE-03 phase sheet's own `###` heading was lost, not its content**
  — see the note at the top of that section, above. Restored as bookkeeping on
  this phase's own surface (`notes.md`); no content was missing or changed.

**No STOP condition was reached.** S-1: every Coverage-table id names a test
that exists and passed in the clean-clone run — none was found missing. S-2:
the clean clone passed identically to the working tree; neither the
`.gitignore` nor any generated-but-untracked file caused a divergence. S-3: no
undeclared path — VA-3's walk above accounts for every changed file. S-4: not
fired — see `## Design drift` below; the one item this phase's own walk added
beyond the seeded three was authorised by an orchestrator ruling recorded in
`notes.md` (F-a, PHASE-04's Findings), not left unauthorised.

## Runbook (VH-1 — for the user to run)

1. **Start it.** From the repo root, in `nix develop` (or with direnv active):
   ```
   just demo
   ```
   A window opens showing the demo backend's fixed prompt — *"Fill in your
   interstitial journal?"* — because the startup evaluation's event carries
   `"source":"host"`. This is the same window `just demo` has always shown;
   nothing about it is new yet.

2. **Emit an event.** From a **second** shell, also in `nix develop`, from the
   repo root:
   ```
   printf '%s' '{"source":"reddit-watcher","kind":"reddit-opened","timestamp":"2026-08-22T17:10:00+10:00","data":{"count_last_hour":4}}' \
     | socat - UNIX-CONNECT:./goad-demo.sock
   ```
   (`nc -U ./goad-demo.sock` also works if you'd rather not use `socat` — send
   the same JSON followed by a newline instead of relying on EOF.)

3. **What you should see.** The window's title changes to:
   ```
   An event arrived: reddit-watcher / reddit-opened
   ```
   with body text naming that it came from the ingested envelope. **That
   change is the point** — it did not come from goad interpreting the event;
   the host forwarded the envelope's bytes to the backend untouched, and
   `examples/shell/backend.sh` — the one file in this project allowed to know
   what an event means — decided what to show. Pick your own `source` and
   `kind` in step 2 and they will appear verbatim in the title.

4. **Try it again immediately.** Run the same one-liner again right away, a
   second time. It should be refused on the emitting shell's own stdout —
   something like:
   ```
   {"protocol":1,"accepted":false,"reason":"too_soon","retry_after_ms":...}
   ```
   (or `"reason":"engaged"` if the first exchange is still in flight) — and the
   window should **not** change again. This is the event spacing: a watcher
   emitting as fast as it can does not get more than one evaluation per
   spacing.

5. **Try something malformed.** `printf 'not json' | socat - UNIX-CONNECT:./goad-demo.sock`
   should come back `{"protocol":1,"accepted":false,"reason":"malformed",...}`
   and the window should not change.

**What would indicate failure** (S-3/S-4 in `plan.md`): the window never
opens; step 3's title does not change at all, or changes to something that
does not name your `source`/`kind`; the process hangs or stops answering; a
socket file (`goad-demo.sock` in the repo root) is left behind after you quit
normally. Any of these — stop and report it; it is a finding, not a note.

## Design drift

<!-- Every place the tree departs from `design.md` as it stands: what the
     design says, what the tree does, which phase's criterion (or ruling)
     authorised it. Compiled at PHASE-07 from the phase sheets and its own
     Coverage walk (`docs/AGENTS.md` §Audit; `plan.md` PHASE-07/EX-7). This is
     what `audit.md`'s *Design drift not reconciled* is written from — this
     section does not itself decide anything, and `design.md` is not edited
     here. -->

**Three amendments taken before implementation, not drift.**
`design-log.md` (2026-09-08, *"the design is amended before implementation, in
three places, because it prescribed what this workspace cannot do"*) records
that `design.md` §5.4's startup order, and §9's AC-10 and AC-9 rows, were
amended — with `draft-spec.md` §7's R-2 and R-4 rows — before any phase wrote
code, because they prescribed a mechanism this workspace cannot legally
implement (no safe umask API; `Host::new` consuming `Config` before the bind
could read it; no test target linking the binary). These are decisions, not
departures: the tree was measured against the amended text throughout, and no
phase's implementation diverges from what `design.md` says today. Listed here,
as `plan.md` PHASE-07/EX-7 requires, so the auditor meets them as amendments
rather than rediscovering them as drift.

1. **`Refusal::Unavailable` gained a payload design.md's own interface block
   does not show.** `design.md` §5.2 (`:297-299`) lists `Refusal`'s payload
   set as `TooSoon { retry_after }`, `TooLarge { limit }`, `TimedOut { after }`,
   `InvalidEnvelope(EnvelopeFault)` — four named payloads, omitting
   `Unavailable` entirely, which is consistent with (and was read by PHASE-03
   as specifying) a **unit** `Unavailable` variant carrying nothing. PHASE-03
   built it that way, on that reading, stated in its own Assumptions note.
   PHASE-04's finding **F-a** then showed this contradicts `design.md` §5.4's
   own sentence two paragraphs later — *"`unavailable` means one thing about
   the host and three about why … and `detail` says which"* — because a unit
   variant's `Display` cannot vary by cause. Reported to the orchestrator and
   resolved by ruling (`notes.md` PHASE-04 Findings, F-a, `d823739`'s
   follow-on, one of the two orchestrator-ruled cross-phase repairs this slice
   took): `Refusal::Unavailable` now carries a two-variant payload,
   `UnavailableCause` (`Stopping`, `ClockUnreadable` —
   `crates/goad-shell/src/ingress/mod.rs:268-295`), and `Display` renders each
   distinctly. **The tree now makes §5.4's sentence true where the design's own
   payload list, unamended, would have kept it false.** Held by
   `unavailable_s_two_causes_carry_different_detail`
   (`crates/goad-shell/tests/integration/ingress.rs`). `design.md` §5.2's
   payload list is not edited here — that is reconciliation's, with the user —
   and is stale until it is: it should read five payloads, the fifth being
   `Unavailable(UnavailableCause)`.

No further departure was found. PHASE-07's Coverage walk (`## Phase sheets`,
`### PHASE-07`, above) turned up one further residue —
`controller.rs::ingress_stopped()`'s hand-written `"unavailable"` literal —
but it is not drift: `design.md` §5.2's own words already specify exactly this
mechanism (*"It folds one `Refused::Ingress` onto the diagnostics surface —
reason `unavailable`, detail naming that ingress has stopped"*), so the tree
matches the design precisely. It is recorded as a coverage finding (F-1,
above) instead.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-08 · PHASE-07 done (VH-1 still excepted — this phase
neither discharges it nor infers it from the gate)

### Produced
<!-- What now exists: modules, contracts, docs. -->

- `docs/slices/004/ingress-probe.local.rs` — the A-1 probe, gitignored and kept
  for re-running, on `docs/slices/003/timer-probe.local.rs`'s shape.
- `research.md` **Thread 3** — the A-1 result: topology, four cases, three runs,
  the verbatim output, and an explicit statement of what was *not* measured.
- `crates/goad-shell/src/config.rs` — `IngressConfig`, `Config.ingress`,
  `FileIngress`; `[ingress]` is now part of the canonical config shape.
- `crates/goad-shell/src/error.rs` — `ConfigError::EmptyPath`.
- `crates/goad-shell/src/ingress/mod.rs` — the module PHASE-03 builds the
  listener into; declares nothing beyond itself and `envelope` this phase.
- `crates/goad-shell/src/ingress/envelope.rs` — `normalize(bytes) ->
  Result<Event, EnvelopeFault>`, the only door from a watcher's bytes into a
  canonical `Event`; `EnvelopeFault`'s ten variants. No `Ingress`, `Arrival`,
  `Answer`, `bind` or `Refusal` yet — PHASE-03's.
- `goad-semantics/src/error.rs` — `json_type_name` is now `pub` (D-18),
  reachable from `goad-shell` without a second type-name table.
- `crates/goad-shell/Cargo.toml` — `tokio`'s `net` and `sync` features, the
  whole of this slice's manifest bill against the ADR-001 allowlist (nothing
  else adds a feature or a dependency for the rest of the slice).
- `crates/goad-shell/src/ingress/mod.rs` — `bind`, `IngressError`/`BindFault`,
  `Ingress`, `Arrival`, `Answer`, `Refusal` (seven variants: PHASE-03's five
  plus PHASE-08's `Engaged`, `TooSoon`), the accept task: reclaim, the
  owner-only mode, the newline-or-EOF framing, both read budgets, the one
  reply — `retry_after_ms` included. `SPEC-003/R-1..R-10`, `R-13`, `R-14` are
  now discharged in full; the wire's reason set is closed at eight tokens.
- `crates/goad-shell/tests/integration/ingress.rs` — the fake judge fixture
  (`judge`, `Verdict`, now including `Verdict::Refuse` for a scripted
  refusal), PHASE-03/VT-1..VT-6, VT-10..VT-14, and PHASE-08/VT-7, VT-8, VT-9.
- `crates/goad/src/controller.rs` — `serve`'s seventh parameter and `Served`'s
  seventh field; `Fired::Ingested`; the outer and inner ingress arms; the
  second anchor `event_floor_until` with its one write site; and the four
  private free functions the arms delegate to — `spacing_elapsed` (the
  boundary R-14 forces), `ingest` (§5.4 steps 1, 3-5), `refuse_during_exchange`
  (steps 1 and 2), `refuse_arrival` and `ingress_stopped`. `dispatch` is the
  four-command match, lifted out of `serve` so the ingested road and the
  command road meet at one `Option<Result<Pending, Refused>>`.
- `crates/goad/src/diagnostics.rs` — `Refused::Ingress { reason, detail }`:
  two **rendered** values, so this module still holds no ingress vocabulary.
- `crates/goad/tests/renderer/ingress.rs` — the renderer tier's ingress cases
  (PHASE-04/VT-1..VT-5, VT-7), the counting `Glass` decorator (PL-8), and the
  blocking writer's side of a connection. The first module in that target to
  open a socket. **PHASE-05 extends it**: six more cases —
  `an_ingested_firing_never_writes_the_scheduled_floor`,
  `an_ingested_firing_does_not_advance_the_scheduled_floor` (the case ADR-004
  named; CD-3's discharge),
  `a_scheduled_firing_does_not_clear_the_event_floor`,
  `a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface`,
  `a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`,
  `after_a_flood_of_malformed_envelopes_the_host_still_evaluates`. `flat_out`
  is generalised to take the envelope as a parameter, shared by PHASE-04/VT-5
  and PHASE-05/VT-6. No production code changed.
- **PHASE-06** — `crates/goad/src/startup.rs`: `StartupError::Ingress`,
  `listener(configured: Option<&IngressConfig>) -> Result<Ingress,
  StartupError>` — `main::start`'s only decision of `None` versus `Some`.
  `crates/goad/src/main.rs`: `start`'s nine numbered steps, the bind at step 3
  (after the runtime guard, before `Host::new`), `serve`'s real `Ingress`.
  `crates/goad/tests/renderer/startup.rs`: the `listener` module (VT-1, VT-2),
  the `Ingress` cases in `display_text`, `source_walk` and `stderr_outlets`
  (VT-3), the corrected module doc sentence. `flake.nix`: `pkgs.socat`.
  `examples/demo.toml`: `[ingress]`, the documented `socat`/`nc` one-liners.
  `examples/shell/backend.sh`: names an ingested event's `source`/`kind` in
  its view; the `"source":"host"` arm keeps the fixed prompt. `.gitignore`:
  `/goad-demo.sock`. **VH-1 open** — the runbook is in this file, above; a
  person has not yet run it.
- **PHASE-07** — `draft-spec.md` §7 completed: all sixteen rows name real test
  functions and files in place of the AC-id prose the design left there; R-5
  unchanged. `## Design drift` (this file) — the three seeded amendments, plus
  `Refusal::Unavailable`'s payload, which `design.md` §5.2's own interface
  block does not yet show. The margin table (`### PHASE-07`, above) — every
  timed assertion in the slice, its bound, its measured ratio, and F-15's
  numbers (845/845, 1.000 per refusal, ~1690/s) in one place. `research.md`
  F10 and F12 refreshed to the finished manifest. The clean-clone gate, run
  twice (working tree and a fresh clone of `99abac4`), both exit 0. **F-1**
  (this phase's own finding, not repaired here): `controller.rs`'s
  `ingress_stopped()` carries the wire token `"unavailable"` as a literal, not
  read off `Refusal::reason()`, and no test checks it against the closed
  eight-token set the way `PHASE-08/VT-8` checks the other seven-plus-one.

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

- **A test written after its implementation has never been asked to fail, and
  `Ok`/`is_ok()` alone rarely asks hard enough of an opaque return type.**
  PHASE-06 wrote `startup::listener`'s three cases after `listener` itself,
  then confirmed them red-first retroactively by reverting the implementation
  behind a scratch copy (never `git stash`) and re-running. Three of four
  broke as predicted; the fourth — `some_path_binds`, which only asserted
  `result.is_ok()` — did not, because `Ingress` exposes nothing outside its
  own crate to tell a real bound listener from `Ingress::none()` (a private
  `arrivals` field). A `listener` that silently stopped binding and always
  returned the empty handle would have shipped green. The retroactive
  break-and-revert is not just a scolding for skipping red-first — it is a
  usable remedy after the fact: it finds exactly the assertions red-first
  would have forced to be written stronger the first time, at the cost of one
  extra pass instead of zero. **How to apply:** when a test's assertion is
  `is_ok()`/`is_err()` (or similarly shaped) against a type with no
  `PartialEq` and no public way to inspect what actually happened, look for an
  independent, checkable side effect the real path leaves and the stub path
  does not — here, the filesystem entry `bind` creates
  (`std::os::unix::fs::FileTypeExt::is_socket()`), a case as available to any
  test as it was to this one. Strong candidate for `docs/memory/`.

- **`invocations(&log) >= n` proves an exchange *began*, not that it was
  *absorbed* — and under heavy machine load the gap between the two is wide
  enough to flip an assertion.** Confirmed by reproduction, not inference:
  under `just check`'s heavier concurrent load (twice, back to back), a
  second envelope sent right after `invocations(&log) >= 2` landed *before*
  the second exchange's `absorb`, so the loop was still `engaged` and the
  reply was `engaged` instead of the `too_soon` the case expected — in both
  PHASE-05/VT-3 (fixed here) and in PHASE-04's own
  `a_second_envelope_inside_the_spacing_is_refused_too_soon_and_says_how_long`
  (fixed by a bounded cross-phase repair, `d45901b`'s follow-on — see
  *Open*, below, now resolved). `scheduling.rs`'s `absorbed_line` doc comment already
  names this race for the *scheduled* firing case; it applies identically to
  an *ingested* one. **How to apply:** wherever a case sends a second
  envelope (or otherwise depends on state a prior exchange's `absorb` set)
  right after only an invocation-count wait, wait for the exchange's own
  rendered `next_check` to change instead (`window.get_next_check() ==
  next_check_line(...)` for that exchange's own instruction) — the pattern
  PHASE-04/VT-5 and PHASE-05/VT-2 and VT-4 already use. Strong candidate for
  `docs/memory/`.
- **A repair sweep finds prose and misses the binding site.** Three of the
  design review's four rounds yielded the same class: a repair correct where it
  landed, not carried to the artefact that states the same thing normatively.
  Both round-3 contests were this, and both times the missed site was the more
  binding one — a spec requirement (R-15's universal), and a sequence diagram in
  which position is time. Prose siblings get swept; a MUST and a picture do not.
  **How to apply:** when dispositioning a `doc-wrong`, name the most binding
  artefact by hand in the repair brief rather than trusting the repairer to
  sweep for it. Candidate for `docs/memory/`.

- **A reviewer's supporting example is not evidence until someone checks it.**
  The plan review's F-27 was a false clause the reviewer supplied in F-25's
  body, repeated in its round-3 reply, and adopted whole into the plan — that
  the design restates neither `SPEC-003/R-8` nor R-9, when it restates both. The
  reviewer caught it only because it was told to check the orchestrator's own
  edit hardest. **How to apply:** a reviewer is the last person who will check
  its own example, so an example adopted from a finding gets verified by whoever
  writes it into a document. A wrong reason beside a right one is worse than no
  reason: an agent who checks the wrong one has cause to doubt the right one.
  Candidate for `docs/memory/`.

- **A-1 holds, and the margin is 334×.** A `tokio::spawn`ed accept task on the
  multi-thread runtime delivers to a `slint::spawn_local` future while the main
  thread is inside Slint's event loop: 128–207 µs ordinarily, 243–300 µs with
  nothing else armed after a second of idle. `UnixListener::bind` succeeds under
  the `EnterGuard`, synchronously, where `design.md` §5.4 puts it; an arrival is
  observable by an **inner** `select!` mid-exchange, which is what makes
  `engaged` reachable. **How to apply:** the companion fact to
  `docs/memory/tokio-time-runs-under-slints-executor.md` — that one says a future
  on Slint's executor keeps tokio time; this one says the *runtime's own tasks*
  keep running and its reactor keeps polling while Slint owns the main thread. Do
  not reach for `slint::invoke_from_event_loop` or a second `spawn_local` accept
  loop on the assumption that a spawned task is starved. Candidate for
  `docs/memory/`, at close.

- **`EnvelopeFault` carries one variant, `Malformed`, that no `draft-spec.md`
  requirement names and no PHASE-02 `VT` id covers.** It exists because
  `normalize` takes raw bytes and `reject_duplicate_keys` can itself report
  "not a JSON document" as a side effect of the walk EX-5 requires reusing.
  **Resolved at PHASE-03:** `shape_refusal` maps it to `Refusal::Malformed`
  specifically (every other `EnvelopeFault` variant maps to
  `Refusal::InvalidEnvelope`), so it meets the wire's `malformed` reason
  exactly as `draft-spec.md` §6.3's table names it.

- **Closing an `AF_UNIX SOCK_STREAM` socket with the writer's bytes still
  unread resets the connection and can take an already-written reply down with
  it — and the fix must be non-blocking, not a second bounded wait.**
  `too_large` is the guaranteed case: the read stops at the byte cap, but the
  writer may have sent (or still be sending) more. Dropping the connection
  there produces `ECONNRESET` on the peer's read of the reply this host just
  wrote — measured directly (PHASE-03/VT-13 failed with exactly that error
  before a drain existed). The wrong fix is tempting and cheap to reach for:
  wrapping the drain in `tokio::time::timeout(ENVELOPE_DEADLINE, …)` the same
  shape the read itself uses. Measured, that **doubles** the time a silently
  stalled writer (`timed_out`) waits for its refusal — from ~502 ms to
  ~1.0025 s — because a writer with nothing queued gives the drain nothing to
  do *but* wait out its own copy of the deadline. The fix that costs nothing
  in the common case is non-blocking: `UnixStream::try_read` in a
  bytes-bounded loop, stopping the instant nothing is immediately readable,
  never waiting for more to arrive. **How to apply:** any refusal path that
  stops reading before a stream's peer necessarily has finished writing needs
  this same non-blocking drain before the connection closes — and the
  bounded-*wait* shape that is correct for the read itself (R-7) is the wrong
  shape to reuse for a post-refusal cleanup step, because the read's bound
  exists to end a wait, while the cleanup step's job is to end instantly when
  there is nothing left. Strong candidate for `docs/memory/` at close — this
  is a general fact about Unix domain stream sockets, not specific to this
  slice.

- **Rounding a `Duration` up to the millisecond needs no division and no
  cast, because `Duration::as_millis` already truncates — in the standard
  library, not the caller's arithmetic.** Adding `Duration::from_nanos(999_999)`
  before calling `as_millis()` turns that existing truncation into a ceiling;
  `u64::try_from` narrows the `u128` it returns. Both denied lints
  (`clippy::integer_division`, `clippy::as_conversions`) stay clear without an
  `#[allow]`. **How to apply:** the same shape works for any "round this
  bounded duration up to a coarser unit" need under this workspace's lint
  set — reach for `checked_add` + the coarser unit's own truncating accessor
  before reaching for a raw division or a cast.

- **A wire reason can be `reason()` special-casing one payload's inner value,
  not a new outer variant.** `reserved_source` (`SPEC-003/R-13`) is
  `Refusal::InvalidEnvelope(EnvelopeFault::ReservedSource)` read by a
  `reason()` arm matched ahead of the general `InvalidEnvelope(_)` arm — the
  payload carries what happened, the match on it decides which token reaches
  the wire. **How to apply:** before adding a variant to widen a reason set,
  check whether an existing payload's own inner value already distinguishes
  the case; a match arm is cheaper than a variant and keeps the payload count
  matching the design's own list.

- **One refused envelope costs exactly one presentation, and the number is
  845/845.** Measured over a 500 ms flat-out window with the loop idle:
  845 refusals, 845 presentations, 1.000 per refusal, ~1690 per second
  (`renderer/ingress.rs`'s VT-5). That settles `review-design.md` F-15 — the
  cost is fixed at one by an assertion, so a change that raised it, or that
  added a second route to the diagnostics surface, fails there. **How to
  apply:** this is the baseline `design.md` §8 R6's signal is measured
  *against*; R6's signal is a rise above it, not the one-for-one itself. The
  rate is the loop's throughput under a writer the host paces (I-2), not a
  claim about visible responsiveness — that is headless, and belongs to a
  human run.

- **`Controller::absorb` replaces the whole retained `Diagnostics`, so any
  assertion about a fold must be read on the *live* route if an exchange
  follows it.** `Served.controller`'s diagnostics is readable only after
  `serve` returns, and by then every exchange that completed in between has
  overwritten it. The live route is the window's own `get_diagnostic_lines()`,
  which `glass.rs:94-102` writes unconditionally on every present. **How to
  apply:** this is what makes `SPEC-003/R-15`'s bound work at all — a refusal
  decided during an exchange is superseded before anything is presented — and
  it is why PHASE-04/VT-7's *exactly one fold* is held by the presentation
  count rather than by the retained value. It caught a plan defect (F-b) and
  it will catch PHASE-05's R-15 negative case if that reads the retained value.

- **`tokio::net` is reachable from `crates/goad`'s tests only by feature
  unification, and that is a trap.** `crates/goad/Cargo.toml` declares `tokio`
  with `rt-multi-thread` and `sync`; `net` arrives because `goad-shell` enables
  it and cargo unifies features across the graph. Code in `crates/goad` that
  names `tokio::net` compiles today and breaks the day `goad-shell` stops
  needing `net`, for a reason nothing in `crates/goad` states.
  **How to apply:** for a socket a *test* opens, `std::os::unix::net` on
  `tokio::task::spawn_blocking` needs no feature at all and keeps the blocking
  call off the thread the loop runs on. Reach for the manifest only when
  production code needs the feature. Candidate for `docs/memory/`.

- **A second tokio runtime, entered around `bind` and then
  `shutdown_background()`ed, is how a test reaches a dead accept task.**
  `bind` spawns its accept task onto whatever runtime is entered when it is
  called, so shutting that runtime down drops the task and with it the
  channel's only sender — and `Ingress::arrival` then yields `None` and parks.
  `shutdown_background` rather than `drop`, because dropping a `Runtime` inside
  an async context panics. **How to apply:** it needs no production API and no
  panic to provoke, which is what kept `Ingress` from growing a test-only
  constructor (PHASE-04/S-5). The same shape reaches any "the task that feeds
  this channel is gone" state.

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

- ~~**`unavailable`'s wire `detail` cannot distinguish its three causes.**
  `Refusal::Unavailable` is a unit variant with a fixed `Display`, so the
  clock-unreadable cause reaches its writer with the dropped-`Answer`
  wording. `design.md` §5.4's *"`detail` says which"* is not true of the wire
  today (PHASE-04 finding F-a). Adding a payload to `Refusal` is a design
  question about what that type carries; the diagnostics surface names the
  clock correctly in the meantime.~~ **Resolved** (bounded cross-phase repair,
  orchestrator-ruled, `d823739`'s follow-on, recorded in PHASE-04's Findings as
  F-a): `Refusal::Unavailable` now carries `UnavailableCause` (`Stopping`,
  `ClockUnreadable`), and `Display` renders each distinctly on the wire —
  `unavailable_s_two_causes_carry_different_detail`
  (`crates/goad-shell/tests/integration/ingress.rs`) holds it. Left here,
  struck rather than deleted, so a reader who remembers this as open finds the
  resolution rather than a stale gap; this entry was found stale by PHASE-07's
  sweep, not repaired by it (`design.md` §5.2's payload list is the one thing
  this leaves open — see `## Design drift`, item 1). **A second, narrower**
  `unavailable` gap remains open — see PHASE-07's finding F-1, above: the
  ingress-stopped cause's wire token is a literal no test checks against the
  closed set.
- ~~**PHASE-04's own
  `a_second_envelope_inside_the_spacing_is_refused_too_soon_and_says_how_long`
  carries the same race PHASE-05's *Learned* entry above describes**: it
  sends its second envelope right after `invocations(&log) >= 1`, which
  proves only that the first exchange *began*, not that it was absorbed.
  Reproduced twice under `just check`'s heavier load, both times as
  `engaged` where `too_soon` was expected. Not repaired here: it is
  PHASE-04's own test body, outside this phase's declared surface to fix on
  its own initiative (`docs/AGENTS.md` §Execute — *"stop and ask"* on
  anything beyond the declared surface). The fix, when taken, is the same
  one PHASE-05/VT-3 applies: wait for the exchange's own rendered
  `next_check` before sending the next envelope.~~ **Resolved** (bounded
  cross-phase repair, orchestrator-ruled, `d45901b`'s follow-on): the case
  now waits for `window.get_next_check() ==
  goad::diagnostics::next_check_line(instant("2026-01-01T00:01:00Z"))`
  after `invocations(&log) >= 1` and before sending the second envelope —
  PHASE-04/VT-5's own pattern (the closer fit: same single exchange, same
  `NEXT_CHECK_A_MINUTE_OFF` instruction), applied here rather than
  PHASE-05/VT-3's three-exchange shape. Nothing else in
  `crates/goad/tests/renderer/ingress.rs` shares the vulnerable
  construction — every other case either already waits on the absorbed
  line before a dependent send (PHASE-04/VT-5; PHASE-05/VT-2, VT-3, VT-4)
  or deliberately depends on the *opposite* ordering to prove `engaged`
  (PHASE-04/VT-3, PHASE-05/VT-5, both against `@slow-view`'s foreground
  sleep). Proved by reproduction, not inference: under 48-way CPU
  oversubscription (loadavg climbing 13→50 on 32 cores), the pre-fix
  binary failed 13/25 runs of this case, every failure `left: "engaged",
  right: "too_soon"`; the same binary rebuilt with the fix passed 40/40
  runs under loadavg 42→61. `just check` exits 0 on a clean run
  afterward. No production code touched.

### PHASE-02 — The configuration key, and the envelope

**Entry check:** EN-1 — PHASE-01 exit criteria discharged, verdict *A-1 holds*
(above). EN-2 — `just check` exits 0 at `658b124` (PHASE-01's own VA-1 already
showed this; re-verified before editing). Both hold: proceeding.

**Reading list**

| what | where |
|---|---|
| the phase, entire | `docs/slices/004/plan.md:560-705` |
| overview / sequencing | `plan.md:23-50`, `:114-122`, `:196-244` |
| findings against the design that bear on this phase | `plan.md:344-397` (FD-3: the four struct literals; FD-5) |
| coverage this phase discharges | `plan.md:415`, `:440-444` |
| config block | `design.md:165-180` (§5.2) |
| envelope wire form | `design.md:182-237` (§5.2) |
| stratum 2 surface (`Ingress`, faults, constants) | `design.md:239-317` (§5.2) — only the `EnvelopeFault`/`IngressError`/`json_type_name` parts are this phase's; `Ingress`/`Arrival`/`Answer`/`bind` are PHASE-03's |
| D-18 — widening `json_type_name` | `design.md:311-316`, `:628` |
| draft-spec.md, this phase's requirements | R-9 `:101`, R-10 `:102`, R-13 `:105`, §6.2 `:195-214`, §6.3's `invalid_envelope`/`reserved_source` rows `:232-233` |
| existing config | `crates/goad-shell/src/config.rs`, whole file — `File`→`Config` is the permissive/canonical split to mirror |
| existing error taxonomy | `crates/goad-shell/src/error.rs` — `ConfigError`'s shape (`Display`, `source()`, no `_` arm) |
| stratum 1's own split, the model for `envelope.rs` | `crates/goad-semantics/src/protocol/wire.rs` (`reject_duplicate_keys:79`, `Object<T>:49`), `crates/goad-semantics/src/protocol/normalize.rs` (`read_response:97`) |
| the timestamp two-step to mirror | `crates/goad-semantics/src/schedule.rs:70-103` (`parse_instruction`) — envelope's version has no span fallback, R-10 admits only the absolute form |
| `json_type_name` | `crates/goad-semantics/src/error.rs:14-27` |
| `Event`, `Timestamp` | `crates/goad-semantics/src/protocol/canonical.rs:102-113`, `:489-496` — both `pub`, no accessor owed |
| the four bounded `Config` literals | `tests/support/driving.rs:46`, `crates/goad/tests/renderer/scheduling.rs:84`, `crates/goad/tests/event_loop/closing.rs:63`, `crates/goad/tests/event_loop_schedule/scheduling.rs:91` |
| lints that bind this code | `Cargo.toml:123-204` (workspace clippy table) — `expect_used`, `unwrap_used`, `panic`, `unreachable`, `map_err_ignore` all `deny`; `pedantic` `deny` (→ `missing_errors_doc`) |
| allowlist (no manifest edit needed) | `crates/goad-shell/Cargo.toml:12-18` — `jiff` and `serde_json` already present |

**Assumptions**

- `EnvelopeFault` is this phase's own type, not a reuse of `ProtocolError` —
  the two vocabularies serve different contracts (SPEC-001 backend wire vs.
  SPEC-003 envelope) even though the shape-diagnostic style is shared.
- `normalize`'s malformed-JSON case (bytes that are not one JSON document at
  all) is not one of R-9/R-10's named clauses and has no VT case this phase,
  but the function must still handle it soundly (bytes are attacker-controlled
  input) rather than panic — `EnvelopeFault::Malformed`, produced via
  `reject_duplicate_keys`'s own `ProtocolError::Json` arm, satisfies that
  without a second parse attempt needing `.expect()`.
- VT-4 ("each of the four keys wrong-typed") is read as the three keys with a
  declared wire type — `source`, `kind`, `timestamp`, all strings. `data`
  admits any JSON value (design.md's own table), so there is no wrong-typed
  case for it; VT-3 (missing) is the one that legitimately covers all four.
- `reserved_source` as its own **wire** reason is PHASE-08's (plan.md:444). This
  phase only needs `EnvelopeFault` to carry a distinct fault for `source ==
  "host"`, tested at the unit level (VT-9); wiring it to `Refusal`/the reply is
  PHASE-03's and PHASE-08's, and `ingress/mod.rs` beyond its module declaration
  is explicitly not this phase's (Surfaces).

**STOP conditions** (plan.md S-1..S-3, not softened)

- S-1 — normalizing requires reading a value's content (`kind` matched, `data`
  read, `timestamp` compared to now). Not reached: the one comparison made is
  `source == "host"`, which R-13/P-A permit by name.
- S-2 — `reject_duplicate_keys` doesn't reach the case, wanting a second walk.
  Not reached.
- S-3 — the four bounded test files need more than `ingress: None`. Not
  reached.

**Tasks**

- [x] phase sheet written; status set to `in progress`.
- [x] config half: `IngressConfig`, `Config.ingress`, `FileIngress`,
      `File.ingress`, `ConfigError::EmptyPath`, doc-comment correction, VT-1 +
      VT-10 tests.
- [x] semantics half: `json_type_name` → `pub`, doc sentence (EX-8), VA-3.
- [x] `ingress/mod.rs` — module declaration only (EX-3).
- [x] `ingress/envelope.rs` — normalize + `EnvelopeFault` over
      `serde_json::Map` directly (no separate `Envelope` struct — EX-4's "or
      the equivalent two-step; the name and the shape are the phase's"), EX-4..
      EX-7, EX-9, VT-2..VT-9, VT-11.
- [x] four bounded `Config` literals — `ingress: None` (EX-10, VA-4).
- [x] `just check` green; VA-1..VA-4 below.

**Decisions taken during execution**

- **No literal `Envelope` type.** EX-4 offers "the equivalent two-step" and
  says the name and shape are the phase's. Precise per-key diagnostics
  (missing vs. wrong-typed vs. empty, named individually per R-9) need
  bespoke field-by-field logic that a derived `Deserialize` struct would not
  give without collapsing them into one serde error — so `normalize` works
  directly over the `serde_json::Value` / `Map` the duplicate-key walk and a
  generic parse already produce, rather than binding an intermediate
  permissive struct nothing else uses.
- **VT-4 read as the three keys with a declared wire type.** `data` admits any
  JSON value (`design.md` §5.2's own field table), so "wrong-typed" has no
  case for it; VT-3 (missing) is the one that legitimately covers all four
  keys. Recorded as an assumption above before writing the tests, not
  discovered after.
- **One extra `EnvelopeFault` variant beyond R-9/R-10/R-13's clauses:
  `Malformed`**, for bytes that are not one JSON document at all. Not named by
  EX-4's clause list and not covered by a VT id, but structurally required —
  `normalize` takes raw bytes, and `reject_duplicate_keys` can itself report
  `ProtocolError::Json` for them. Exercised by one extra test
  (`bytes_that_are_not_json_are_refused_as_malformed`) for soundness, not a
  named criterion.

**Findings**

- (none against the plan or design; PHASE-01's two harvested findings about
  repair sweeps and reviewer examples don't recur here — nothing in this
  phase went through review yet)

**Verification — every criterion, discharged**

| id | discharged by |
|---|---|
| EX-1 | `config.rs`: `Config.ingress: Option<IngressConfig>`, `IngressConfig { pub path: PathBuf }`; `File.ingress: Option<FileIngress>` with `#[serde(deny_unknown_fields)]` on `FileIngress` |
| EX-2 | `error.rs`: `ConfigError::EmptyPath { key: &'static str }`, raised in `config::ingress_config` for `ingress.path = ""` with `key: "ingress.path"`; `Display`, `source()` (`None`, folded into the existing `EmptyCommand \| NonPositive` arm) and no `_` arm |
| EX-3 | `lib.rs` declares `pub mod ingress;`; `ingress/mod.rs`'s doc cites `SPEC-003` |
| EX-4 | `ingress/envelope.rs`: `pub fn normalize(bytes: &[u8]) -> Result<Event, EnvelopeFault>`; `EnvelopeFault` names `NotAnObject`, `Missing`, `WrongType`, `Empty`, `Unknown`, `Duplicate`, `MissingOffset`, `Unparseable` (R-9, R-10's clauses) plus `ReservedSource` (R-13) and `Malformed` (decision above) |
| EX-5 | `envelope::parse` calls `goad_semantics::protocol::wire::reject_duplicate_keys` once; no second walk |
| EX-6 | `take_timestamp`: absolute parse first, civil-datetime parse distinguishes `MissingOffset` from `Unparseable`, mirroring `schedule.rs:70-103`'s two-step with no span fallback |
| EX-7 | `source == "host"` is the only comparison `envelope()` makes on any field; `kind` checked only for emptiness; `data` never inspected |
| EX-8 | `goad-semantics/src/error.rs:18` `pub fn json_type_name`; `:16`'s sentence now "the one such table in the workspace"; nothing else in the crate changed (VA-3 below) |
| EX-9 | `ingress/mod.rs` and `ingress/envelope.rs` both carry `#![deny(clippy::arithmetic_side_effects)]` |
| EX-10 | four `Config` literals compile with `ingress: None` added, nothing else changed (VA-4 below) |
| VT-1 | `config::tests::an_ingress_section_loads_with_its_path`, `an_empty_ingress_path_is_refused`, `an_unknown_key_inside_ingress_is_refused_and_named`; `an_unknown_key_is_refused_and_named` untouched |
| VT-2 | `envelope::tests::a_non_object_top_level_is_refused_naming_the_type_found` — array, string, number, boolean, null |
| VT-3 | `envelope::tests::each_of_the_four_keys_missing_is_refused_naming_it` |
| VT-4 | `envelope::tests::each_typed_key_wrong_typed_is_refused_naming_it` — source, kind, timestamp |
| VT-5 | `envelope::tests::an_empty_source_or_kind_is_refused_naming_it` |
| VT-6 | `envelope::tests::a_fifth_key_beside_the_four_is_refused_naming_it` |
| VT-7 | `envelope::tests::a_top_level_duplicate_key_is_refused_naming_it`, `a_duplicate_key_nested_inside_data_is_refused_naming_it` |
| VT-8 | `envelope::tests::an_offsetless_instant_is_refused_distinctly_from_an_unparseable_one` — asserts discriminants |
| VT-9 | `envelope::tests::a_reserved_source_is_refused_with_every_other_field_valid` |
| VT-10 | `config::tests::with_no_ingress_section_ingress_is_none` |
| VT-11 | `envelope::tests::the_design_s_own_example_normalizes`, `a_null_data_is_accepted`, `data_carrying_a_nested_object_and_an_array_reaches_event_unchanged`, `timestamps_far_from_now_are_carried_unjudged` |
| VA-1 | `just check` **exit 0** — build, `cargo test --workspace` (15+0+1+1+138+0+43+30+5+35+58+6+0×4 = all green), `cargo test -p goad-semantics` (30+5+0), `deno check` silent, clippy clean, `cargo fmt --all --check` clean. Full transcript kept at `/tmp/claude-1000/-home-david-dev-goad/a10c38f4-3ff2-4c14-924e-3b2377d46bee/scratchpad/phase02-check.txt` for this session only — not part of the durable record |
| VA-2 | `cargo test -p goad-semantics`: 30 unit + 5 `tests/protocol/main.rs` + 0 doc-tests, all green, run **after** EX-8 |
| VA-3 | `git diff crates/goad-semantics/` — exactly the two lines EX-8 names: `pub(crate) fn` → `pub fn` at what is now `:18`, "crate" → "workspace" at `:16`. Nothing else in the crate changed |
| VA-4 | `git diff` over the four bounded files — exactly one `ingress: None,` line added in each, nothing else |

**No STOP condition was reached.** S-1: the only value comparison is
`source == "host"` (R-13 permits it by name); `kind`, `data` and `timestamp`'s
distance from now are never read. S-2: `reject_duplicate_keys` reached every
case (top-level and nested inside `data`); no second walk was written. S-3:
the four bounded files each gained exactly the one field.

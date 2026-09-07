# Plan log — Slice 003

Append-only working record for the plan stage. Survives compaction and
interruption; `plan.md` itself stays clean. Never rewrite an entry — supersede
it with a later one.

Decisions here are taken under the **standing autonomy grant** (`design-log.md`,
2026-09-07 *Gate autonomy*), and are recorded as a user's would be: Asked /
Decided / Why / Rejected / Consequence. **Plan acceptance is reserved to the
user** and is not one of these. Findings from an adversarial review of the plan
would live in `review-plan.md`, not here. Design-shaped decisions taken while
planning are cross-posted to `design-log.md`.

## Decisions

### 2026-09-07 — PL-1: six phases, split 01 / 02 / 03 / 04 / 05 / 06

- **Asked:** how many phases, and where the seams fall. The slice has one
  mechanism (the `select!` arm and its floor), eleven timed assertions, two
  structural instruments, one new `[[test]]` target and a documentary close-out.
  Four phases would put the mechanism and all eleven assertions in one session;
  eight would leave several phases too thin to justify their own sheet.
- **Decided:** six. PHASE-01 the two leaf changes (`wait_for`,
  `Stimulus::Scheduled`); PHASE-02 the whole loop mechanism plus the six
  well-behaved timed assertions; PHASE-03 the three failure-side assertions plus
  the `default_poll` doc comment; PHASE-04 the diagnostic line and AC-6's two
  scans; PHASE-05 the AC-10 target; PHASE-06 the sweep and the gate.
- **Why:** the seams fall where the *fixtures* change, not where the criteria
  do. PHASE-02's six tests share one scripted backend and one working clock;
  PHASE-03's three each need a misbehaving backend and one needs a clock fixture
  that does not exist. Slice 002's PHASE-10 — `serve` plus six `serve` tests —
  is the calibration for what one Sonnet session holds, and this plan does not
  exceed it in any phase.
- **Rejected:** *(a)* mechanism-only PHASE-02 with all nine tests in PHASE-03 —
  it lands a `select!` body nothing drives, which is the failure
  `docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` records.
  *(b)* Folding PHASE-05 into PHASE-06 — the AC-10 target also pays FD-3's
  helper split, and a rushed sweep is how a slice closes on a stale draft.
  *(c)* Folding PHASE-01 into PHASE-02 — PHASE-02 is already the heavy one, and
  stratum 1 has its own gate column worth seeing green alone.
- **Consequence:** `notes.md`'s status table has six rows. No two phases run in
  parallel; 02, 03 and 05 all touch `crates/goad/src/controller.rs`.

### 2026-09-07 — PL-2: the scheduling tests get their own renderer module

- **Asked:** whether the nine `serve`-driven scheduling tests join
  `crates/goad/tests/renderer/wiring.rs` as a ninth `mod`, or take a new file.
- **Decided:** a new module, `crates/goad/tests/renderer/scheduling.rs`,
  declared in `renderer/main.rs`.
- **Why:** `wiring.rs` is 1232 lines and already carries eight modules spanning
  three phases. Adding a ninth makes two phases edit one file and makes the
  audit's surface diff useless. The new module also has a different subject:
  `wiring.rs` is *item 11*, the wiring surfaces; this is the timer.
- **Rejected:** a ninth `mod` in `wiring.rs` — cheaper to write, worse to audit.
- **Consequence:** `until` and the window/tray fixtures now have two consumers
  in one target. PHASE-02 lifts `until` to a `super`-visible helper rather than
  copying it, and updates `wiring.rs`'s uses in the same change.

### 2026-09-07 — PL-3: the short `default_poll` is built in the test target, not in the shared helper

- **Asked:** every scheduling test needs a `default_poll` far shorter than
  `driving::DEFAULT_POLL`'s thirty minutes. Does `tests/support/driving.rs` gain
  a `host_with_poll` helper, or does the renderer target build the `Config`
  itself?
- **Decided:** the renderer target builds it, from
  `goad_shell::config::{Config, BackendConfig, ScheduleConfig}` — all of whose
  fields are `pub` — and hands it to the existing `driving::host_from`.
- **Why:** `host_from` exists for exactly this ("the same composition from a
  `Config` that came from somewhere else"), and slice 002's PL-4 rule says a
  shared helper carries only what **every** including target calls. A
  `host_with_poll` used by one target is the coupling that rule exists to
  prevent — and, under `-D warnings`, would be a hard error in the other target
  (see FD-3).
- **Rejected:** adding the helper to `driving.rs`; parameterising
  `driving::config`, which would touch every existing caller for one test module.
- **Consequence:** `tests/support/driving.rs` is **not** a PHASE-02 or PHASE-03
  surface. It moves once, at PHASE-05, for a different reason.

### 2026-09-07 — PL-4: AC-6's instruments extend `structure.rs` rather than adding a module

- **Asked:** `design.md` §9 places both AC-6 instruments in
  `crates/goad-boundary/tests/checks/`. That target has four modules, three of
  them one-per-ADR-001-instrument and the fourth (`structure.rs`) the
  source-count instrument slice 002 added under its own PL-6.
- **Decided:** two new `#[test]`s inside the existing `structure.rs`, which
  becomes directory-parameterised.
- **Why:** `structure.rs` already is *the* source-count instrument, with the
  production-code cut, the recursive walk, the vacuity guard and the
  counting controls AC-6 needs. A fifth module would restate all four.
- **Rejected:** a new `checks/schedule.rs`; building on `scan::Scan`, which
  `design.md` D-16 rejects on evidence (no `#[cfg(test)]` cutoff, so it is red on
  the tree today).
- **Consequence:** `structure.rs`'s module doc, which currently says the file is
  about `quit_event_loop` and `tokio::spawn` over one directory, is rewritten.
  `crates/goad-boundary/src/` is untouched, as `design.md` §9 asserts.

### 2026-09-07 — PL-5: `tests/support/` splits so the AC-10 target can include only what it uses

- **Asked:** the AC-10 target needs a scripted backend and an invocation log,
  both of which live in `tests/support/driving.rs`. Including that file whole
  leaves roughly nine `pub(crate)` symbols unreachable from the new target, and
  `dead_code` is promoted to an error by the gate's `-D warnings`. So: duplicate
  the helpers locally, or split the file?
- **Decided:** split. `tests/support/scripting.rs` takes `backend`, `marker`,
  `clear`, `logging_backend`, `invocations` and `scripted`, moved unchanged;
  `driving.rs` keeps the host-composition half. The AC-10 target includes
  `scripting.rs` only, and builds its `Config` and `Host` inline as
  `crates/goad/tests/event_loop/closing.rs` already does.
- **Why:** `CLAUDE.md` forbids parallel implementations, and the alternative is
  forty lines of path-resolution and log-counting restated in a third place.
  The cut is the same rule slice 002's PL-4 applied — a shared file carries what
  every includer calls — applied to a third includer whose needs are a strict
  subset.
- **Rejected:** *(a)* restating the helpers in the new target. *(b)* Having the
  new target include `driving.rs` and adding `#[expect(dead_code)]` per symbol —
  the expectation would be unfulfilled in the two targets that do use them, which
  is itself an error under `unfulfilled_lint_expectations`. *(c)* Making the AC-10
  test a second `#[test]` in `tests/event_loop/` — `design.md` D-12 rejects it
  (the testing backend initialises once per process, and the two targets need
  different init functions).
- **Consequence:** PHASE-05's surfaces include the `#[path]` and `use` lines of
  both existing targets. Its VA-2 asserts the move is a move: no renamed symbol,
  no changed body. Recorded as FD-3 in `plan.md`, because the design asserted
  reuse without noticing the constraint.

### 2026-09-07 — PL-6: the phase that lands a timed test measures it; PHASE-06 collects; `design.md` is not edited

- **Asked:** `design.md` §9's margin table is explicitly estimates, and the
  design says *"the plan re-measures"*. Which phase measures, and where do the
  numbers land — `design.md` or `notes.md`?
- **Decided:** each phase measures the rows it lands (PHASE-02: AC-1, AC-2 ×2,
  AC-3 ×2, AC-7; PHASE-03: AC-4, AC-5 ×2, AC-9 ×2; PHASE-05: AC-10) into its own
  phase sheet, alongside `cargo test --workspace` wall time before and after.
  PHASE-06 collects them into one measured table in `notes.md`, next to the
  estimate. **`design.md` is not retro-fitted.**
- **Why:** `docs/AGENTS.md:137` — a design is a record of intent at a point in
  time and must not be silently reconciled to the code. `notes.md` is *the work*
  and is where per-phase measurement belongs; the Harvest lifts what is durable.
  Measuring at the phase that lands the test is the only point at which the
  number is cheap and the context is present.
- **Rejected:** measuring everything at PHASE-06 — by then a bad margin is five
  phases old and the phase that could have fixed it is closed. Editing
  `design.md` §9's table in place — forbidden.
- **Consequence:** a threshold is stated rather than left to judgement
  (PHASE-06/EX-5): any liveness margin measured **below 5x**, or gate wall time
  more than **3 s** above slice 002's 5.276 s baseline, is a finding in `plan.md`
  and a follow-up in `slice-003.md`. Anything else is recorded and passed over.

### 2026-09-07 — PL-7: no new backend script and no new sentinel for the behaviour

- **Asked:** whether the slice needs `tests/backends/answers-as-instructed.sh`
  to grow a sentinel — a `@past`, a `@fails-forever`, a repeating mode.
- **Decided:** no. Every fixture the behavioural criteria need is either a
  response body the script already passes through verbatim (`{"view":null}`;
  `{"view":null,"next_check":"100 milliseconds"}`;
  `{"view":null,"next_check":"2020-01-01T00:00:00Z"}`) or an existing sentinel
  (`@garbage`, `@exit1`).
- **Why:** the script's `*)` arm prints any instruction verbatim
  (`answers-as-instructed.sh:83-86`), and each case needs only a bounded number
  of invocations — two or three — so a repeating mode buys nothing. Adding a
  sentinel would make the fixture file a PHASE-02 *and* PHASE-03 surface for no
  behaviour.
- **Rejected:** a `@past` sentinel; a per-case script.
- **Consequence:** `tests/backends/` is not a surface of PHASE-02 or PHASE-03.
  It stays permitted by `slice-003.md`'s Scope for one narrow case — a script
  that echoes the *request* back, if PHASE-02/VT-3 or VT-7 needs to assert
  `event.kind` on the wire rather than infer it from the invocation count. That
  is a wire assertion, not a behaviour one, and PHASE-02's notes say to prefer
  the count where the kind is not the point.

### 2026-09-07 — PL-8: FD-1 is escalated to the user rather than decided here

- **Asked:** `design.md` §5.2 has the next-check line appended to the model the
  markup's empty-surface sentinel is computed from, which silently deletes
  "Nothing to report." and reddens a slice 002 assertion. Repairing it needs
  `crates/goad/ui/app.slint`, which `slice-003.md`'s Scope does not declare.
  Decide it under the grant, or escalate?
- **Decided:** escalate. `plan.md`'s FD-1 states the defect, the evidence, the
  recommendation and the alternative; PHASE-04/EN-2 makes the ruling an entry
  criterion, and PHASE-04/S-10 stops the phase without it.
- **Why:** the grant covers plan questions. This is not one — it is a change to
  what a person sees on the glass, and one of the two answers deletes a
  behaviour slice 002 wrote a test to hold (DT-5). Either answer also widens the
  slice's declared scope by one file.
- **Rejected:** taking the recommendation silently — it would put a markup change
  into a slice whose charter says stratum 2 gains nothing and lists no markup;
  taking the other answer silently — it would retire a user-visible behaviour by
  side effect.
- **Consequence:** PHASE-04 cannot start until the ruling is recorded here. The
  recommendation is a dedicated `in property <string> next-check` on
  `PromptWindow`, rendered under the diagnostic list, on the same argument
  `design.md` §5.2 already makes for keeping the line outside `Diagnostics`.

### 2026-09-07 — PL-9: FD-1 ruled — the dedicated property, and the markup bound

- **Asked:** PL-8 escalated FD-1 rather than deciding it. The orchestrator ruled
  under the standing grant.
- **Decided:** the recommended repair. `PromptWindow` gains
  `in property <string> next-check`, rendered as its own line on the diagnostic
  surface, separate from `diagnostic-lines`, so the "Nothing to report."
  sentinel and slice 002's DT-5 behaviour are untouched.
  `crates/goad/ui/app.slint` enters the slice's Scope **for that property and
  its one markup line only**; anything more in the markup is a STOP.
- **Why:** recorded in full at `design-log.md`, 2026-09-07 (*FD-1: the
  next-check line gets its own window property*), as design decision D-17. This
  entry exists so the plan log is not silent about a ruling that changes a
  phase's surfaces.
- **Rejected:** appending to `diagnostic-lines` and rewriting `dt1`'s
  assertion.
- **Consequence:** PHASE-04 declares `app.slint` unconditionally; its EN-2 is
  now an agent check (the sentinel and `dt1` are green on entry) rather than a
  wait on a ruling; EX-4 names the property and requires `""` for `None`,
  because `SlintGlass::present` is total; VT-2 asserts the sentinel **still**
  renders on a clean outcome with a next check standing; and S-10 now guards
  the markup bound rather than the missing ruling. `design.md` §5.2, §7 D-9 and
  §7 D-17, and `slice-003.md`'s Scope all carry it.

### 2026-09-07 — PL-10: the `#[allow(dead_code)]` alternative to FD-3's split, evaluated and refused

- **Asked:** `review-plan.md` F-3 observes that PL-5's rejected list omits the
  obvious fourth alternative — `#[allow(dead_code)]` on the AC-10 target's own
  `#[path]` module declaration — and that PL-5's objection to the per-symbol
  `#[expect]` does not touch it: `allow` never raises
  `unfulfilled_lint_expectations`, and an attribute on the includer is invisible
  to the two targets that do use the symbols. One line against ten files.
- **Decided:** **refused. PL-5's split stands.**
- **Why:** it is refused by canon rather than by taste. POL-001 §Compliance
  states what lint discipline the gate permits: *"a **site-local**
  `#[expect(lint, reason = …)]` at the narrowest scope that works, argued where
  it is written, on code that genuinely cannot satisfy a lint — **never
  `allow`**, which is silent when it stops being true"*.
  `docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` reaches the
  same place from experience: *"treat any other spelling (a plain `#[allow]`, a
  bare `#[expect]`) as a defect."* And the positive rule the split obeys is
  already written down —
  `docs/memory/shared-test-helper-lives-at-workspace-root-via-path.md`: *"When a
  later change makes a shared symbol unused by one includer, move it out
  immediately rather than leaving dead weight in the shared file — this
  re-settlement is expected maintenance, not a sign the split was wrong."*
  F-3 is right that the argument was owed; this entry is it.
- **Rejected:** `#[allow(dead_code)]` on the includer (above).
  `#[expect(dead_code)]` on the includer, which would be *fulfilled* in the new
  target and so is green — but a module-wide expectation over a hand-written
  shared helper is not site-local, and POL-001's one module-scoped carve-out is
  the generated-code quarantine, which this is not. Re-tracing the closure to
  find a smaller cut: there is none, the six symbols are exactly `scripted`'s
  transitive closure plus `invocations`.
- **Consequence:** `plan.md`'s FD-3 carries the argument on the page. The split
  is unchanged; what changes is PL-13, which stops treating it as a plan
  decision.

### 2026-09-07 — PL-11: supersedes PL-6 — a breached margin is a STOP, not a follow-up

- **Asked:** `review-plan.md` F-6 observes that PL-6 attached a threshold to the
  margin measurements and no consequence: a liveness margin measured below 5x
  became a finding and a follow-up, and shipped — inside a slice whose AC-12
  says a test whose passing depends on machine load is *"a design defect, not a
  tolerated cost"*. PHASE-06 is the phase after which no phase can act.
- **Decided:** **the threshold stays and the consequence changes.** A breach is
  a STOP. The executor records the measurement, repairs nothing on its own
  authority, and consults the orchestrator.
- **Why:** an instrument with no consequence on breach is decoration; one whose
  consequence is a note is worse, because it looks like control. F-6 is right
  that the instrument was otherwise well chosen, so nothing else about PL-6
  moves: the phase that lands a test still measures it, PHASE-06 still collects,
  and `design.md` §9 is still not retro-fitted (`docs/AGENTS.md:137`).
- **Rejected:** lowering the threshold so fewer measurements breach it — that is
  adjusting the instrument to the reading. Moving all measurement to PHASE-06 —
  PL-6 rejected it for a reason that still holds.
- **Consequence:** the STOP is written into the three phases that measure, not
  only into the one that collects: PHASE-02/S-20, PHASE-03/S-22, PHASE-05/S-26
  and PHASE-06/S-19. PHASE-06/EX-5 no longer writes into `slice-003.md`'s
  Follow-ups, which is `docs/AGENTS.md` §Close's section (F-10); a divergence is
  recorded in `notes.md` and named there as a candidate.

### 2026-09-07 — PL-12: supersedes PL-2's Consequence — the shared renderer fixtures get a target-local `harness.rs`

- **Asked:** PL-2 gave the scheduling tests their own module and said PHASE-02
  would *"lift `until` to a `super`-visible helper … and update `wiring.rs`'s
  uses in the same change"*, while PHASE-02's surfaces named `wiring.rs` as
  **must not touch**. `review-plan.md` F-2 shows the lift is seven fixtures, not
  one — `TIMEOUT`, `now`, `stub_clock`, `window_and_tray`, `glass_over`,
  `current_view_token`, `until` — and all seven are private items of the
  `wiring` module, reachable from no sibling. So: where do they live?
- **Decided:** `crates/goad/tests/renderer/harness.rs`, a new module of the
  renderer target, declared in `renderer/main.rs`. PHASE-02 declares it, and
  declares `wiring.rs` and `table.rs` as surfaces bounded to the fixture move
  and the `absorb` migration.
- **Why:** the pattern already exists in this repository and its own header
  states the rule —
  `crates/goad-shell/tests/integration/harness.rs:5-10`: *"Anything two of the
  three case files need lives here; anything one of them needs stays there. What
  both tiers need lives in `tests/support/driving.rs`."* Exactly seven fixtures
  now have two consumers in the renderer target, so exactly seven move and
  nothing else does. `tests/support/` is the wrong home twice over: three of the
  seven name Slint types the `integration` target cannot see, and FD-3's own
  rule requires every symbol in a `#[path]`-shared file to be reachable from
  every includer.
- **Rejected:** putting the fixtures in `renderer/main.rs`, which is a
  declaration file and would carry seven definitions for no reason beyond
  avoiding a new file. Copying them into `scheduling.rs` — `CLAUDE.md` forbids
  a parallel implementation. Making `scheduling.rs` a ninth `mod` of
  `wiring.rs`, which PL-2 already rejected and F-2 does not reopen.
- **Consequence:** three files change and one is created. `wiring.rs` loses
  seven definitions and gains one `use crate::harness::{…}` line; its nine child
  modules are **not** edited, because `super::X` resolves through a private
  parent import — the tree relies on it today at `wiring.rs:706`, where
  `mod body_content` reaches `crate::driving`'s `host`, `quiet_event` and
  `scripted` that way. PHASE-02/VA-4 is the instrument that says the move was a
  move. `crates/goad/tests/renderer/` is already named in `slice-003.md`'s
  Scope, so no scope widening follows.

### 2026-09-07 — PL-13: FD-3's scope widening is escalated to design, as FD-1's was

- **Asked:** `review-plan.md` F-3's second limb. PL-8 escalated FD-1 rather than
  deciding it, on the stated reason *"Either answer also widens the slice's
  declared scope by one file."* FD-3 widens it by eight, including an entire
  test target of another crate, and PL-5 decided it under the grant. Same test,
  opposite treatment.
- **Decided:** give it FD-1's treatment. The split is recorded as a **design**
  decision — **D-18** in `design-log.md` and `design.md` §7 — and
  `slice-003.md`'s Scope names `tests/support/scripting.rs` and the
  include-and-import lines of both existing targets.
- **Why:** the finding is correct that a plan may not widen a slice's charter on
  its own recognisance, and that consistency between the two escalations is the
  point rather than the volume of files. The *substance* is unchanged — PL-10
  settles that the split is the right answer — so what is owed is the record,
  not a re-decision.
- **Rejected:** leaving it as a plan decision on the ground that it is
  test-only. `slice-003.md`'s Scope is explicit to the file for tests as well as
  for source, and a whole test target of another crate is not a detail.
- **Consequence:** `design.md` §7 gains D-18; `design-log.md` gains the matching
  entry; `slice-003.md`'s Scope gains the two lines. PHASE-05's surfaces are
  unchanged — they were already correct — and it gains S-25, which stops the
  phase if `renderer/harness.rs` turns out to import a moved symbol.

### 2026-09-07 — PL-14: an edit a lint forces is part of a phase's surface, and is billed before the phase starts

- **Asked:** `review-plan.md` F-13. PHASE-02's bound on `wiring.rs` admitted two
  edit classes — the `absorb` migration and the fixture move — and its own STOP
  (S-21) forbade a third. But the moved fixtures are the sole consumers of ten
  imported items over eight `use` lines, and `unused` is `deny` at the workspace
  root with `unused_imports` explicitly kept there (`Cargo.toml:99-102`), so the
  phase could not reach a green gate without making an edit its own STOP catches.
  Widen the bound, or leave the executor to stop?
- **Decided:** widen it, **and state it as the compiler's bill**. PHASE-02/EX-12
  names the eight lines one by one, with what each becomes; S-21 excludes exactly
  those; VA-4's expected diff gains them.
- **Why:** *"whatever the compiler demands"* is not a bound, and a STOP an
  executor must disobey to end green is worse than no STOP. A named bill is
  checkable at the phase and re-derivable at audit, and it turns the question
  *"is this edit in scope?"* from a judgement into a lookup. The measurement
  found one item the review missed — `Model`, which never appears by name and is
  the trait behind `.row_data` — which is itself the argument for billing rather
  than describing.
- **Rejected:** naming the class without listing the lines (unbounded); leaving
  the imports in place with an `#[allow]` (POL-001 §Compliance, and PL-10);
  moving the seven fixtures without removing their imports and letting a later
  phase clean up (a red gate is not a handover).
- **Consequence:** the class — **an edit a lint forces that the design did not
  ask for** — is now something the plan looks for rather than something an
  executor discovers. Swept once across every phase: the only other instance is
  PHASE-05's split, which orphans `use std::path::{Path, PathBuf};` in
  `tests/support/driving.rs`, now billed in EX-1 with S-27 as its guard.
  PHASE-01 and PHASE-03 are additive; PHASE-02's `table.rs` orphans nothing;
  PHASE-04's `structure.rs` and PHASE-06's documents are declared whole, so a
  forced edit inside them is already in bounds. A candidate for `docs/memory/` at
  close: a bounded surface must be billed from the compiler, and a move's bill
  includes the imports whose last consumer moved — traits included, which are the
  ones a grep for the name will miss.

### 2026-09-07 — PL-15: the plan is accepted

- **Asked:** whether the plan at `0b2e50f` — six sequential phases, all twelve
  acceptance criteria and draft SPEC-002 R-1..R-11 mapped, review-plan
  resolved at 13/13, D-17 and D-18 as scope widenings — is accepted, and
  whether execution pauses part-way.
- **Decided (the user):** **accepted; execute all six phases.** Sonnet
  executors, one phase per session, a stage commit per phase, Opus on a
  failed phase; STOP conditions come to the user through the orchestrator.
- **Consequence:** `slice-003.md` stage → `executing`. PHASE-01 begins.

### 2026-09-07 — PL-16: two PHASE-02 executor calls confirmed

- **Asked:** (1) EX-12's `use`-line bill in `wiring.rs` named eight lines; a
  ninth, `use std::time::Duration;`, was also consumed only by the moved
  fixtures and had to go (`unused_imports` denied). (2) PHASE-02's
  must-not-touch list names `tests/backends/`, but its own implementer notes
  pre-authorise the one new script VT-3/VT-7 need.
- **Decided (orchestrator, under the grant):** both confirmed as within the
  phase. (1) is PL-14's class — a bounded surface is billed by the compiler;
  the bill was one line short. (2) is a plan self-contradiction; the
  implementer note is the intent. PHASE-06's restatement sweep corrects
  PHASE-02's EX-12 prose/table and its must-not-touch list in `plan.md`.
- **Consequence:** no STOP; PHASE-02 committed as reported.

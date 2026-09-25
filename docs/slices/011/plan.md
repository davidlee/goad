# Plan — Slice 011: the refused arrival's present

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

<!-- Phase ids (PHASE-NN) and criterion ids (EN-/EX-/VT-/VA-/VH-N) are
     immutable: edits append, never renumber, so the sequence goes
     non-monotonic after a split and that is expected. Criterion ids are local
     to their phase — cite another phase's phase-qualified (PHASE-03/EX-2).
     Verification modes — VT: automated test. VA: agent check. VH: human
     acceptance.
     Progress is NOT recorded here. Status lives in `notes.md`. -->

## Overview

The phases below each end green on `just check`. The instrument comes first,
then the change it measures, then the same measurements taken under load and in
front of a person.

- **PHASE-01 — the recording glass.** `RecordingGlass` replaces `CountingGlass`
  in `crates/goad/tests/renderer/ingress.rs` (`design.md` D12). The writer
  records when it sent and when each reply came back. The cases that pass on
  today's loop move onto the log: T3 (VT-4's positive case, rewritten to read
  the window), T1 (VT-5 renamed, its presentation assertion dropped), and VT-7
  with its added assertion. M0 proves, before anything depends on it, that the
  log reads the window and not the model. `controller.rs` is not touched.
- **PHASE-02 — the coalescing loop.** T2 and T4 are written and seen red on
  today's loop. Then the `'idle` loop, `surface_stale`, `next_refusal_present`
  and its arm, and `REFUSAL_PRESENT_INTERVAL` (`design.md` §5.2) turn them
  green. The stale doc comments are rewritten. Every control in `design.md` §9
  is run at rest.
- **PHASE-03 — under load, and in front of a person.** Every timed bound's
  margin is measured at the bound, at rest and on an oversubscribed machine. M9
  is run under load. A flood writer is committed to the slice folder. AC-6 is
  run by a person. Every case name that `canon-delta.md` cites is checked to
  resolve.

What no phase does, and audit or close must:

- **Promotion** of `canon-delta.md` Changes 1–5, with the user's endorsement
  (AC-5).
- **AC-6's record** in `audit.md` §Evidence. PHASE-03/VH-1 is the run, and
  audit cites it.
- **FU-2's strike** with the restated *Dead when* (AC-7), and FU-3's row
  gaining this slice's citation (`design.md` R4).

**Test names are commitments.** `canon-delta.md` Change 3 and Change 4, and
`design.md` §9, name the cases. A phase that names a case differently updates
both in the same commit and says so in its phase sheet. SPEC-002 and SPEC-003
keep citing the old names until audit promotes the delta. That is expected
mid-slice. Nothing in the gate reads spec citations (checked at plan: no
`goad-boundary` check opens `docs/specs`).

## Sequencing & rationale

**Why the recorder is its own phase.** Every timed claim in the slice is read
from the log (D12). If the log read the retained model instead of the window,
every case built on it would stay green through a broken window write, and
nothing would say so. PHASE-01 gives that instrument its own red: M0 on T3,
before any coalescing exists to confuse the result. The cases that move in it
all pass on today's loop, so the phase ends green without touching
`controller.rs`.

**Why red and green share PHASE-02.** Every phase ends with `just check` exiting
0 (`docs/AGENTS.md` §Execute), so a case that is red on today's loop cannot be
committed apart from the code that turns it green. PHASE-02 records the red in
its phase sheet before writing the loop.

**What is red on today's loop, and what is not.** Traced at plan:

- **T2 is red on (a).** Today every refusal presents, so the count is the
  number of refusals, orders of magnitude over the ceiling. This is M1's
  behaviour, which is the tree itself.
- **T4 is red on its precondition.** B presents at once, so "the last present
  shows A and not B" fails. That is the old behaviour, but it is not the
  assertion T4 exists for. T4's own assertion is proven by M7 alone.
- **T3 is green on today's loop.** Both of its refusals present at once. Its
  red comes only from M5, M6 and M8, which need the new code.
- **VT-7's added assertion is green on today's loop.** Today the fold
  `continue`s to the top and presents at once. Its red is the R2 mutation.

So T3 and VT-7 are *green-first*. Their evidence is the mutation table, and
the phase sheets must say so rather than claim a red that never happened.

**Why T1 lands in PHASE-01, not PHASE-02.** VT-5's third assertion (one present
per refusal) is `CountingGlass`'s other consumer. Moving it onto the log only
to delete it one phase later would be churn. Dropping it in PHASE-01 leaves
the per-refusal cost unasserted for one phase, in which nothing changes the
loop (`plan-log.md`).

**Why load gets its own phase.** Measuring at the bound under
oversubscription takes wall time (tens of runs), not design attention. It
should run against the finished tree, and it must not overlap another
session's measurement (see PHASE-03's STOP conditions).

**Nothing runs in parallel.** Each phase needs the one before it. PHASE-01 and
PHASE-02 both write `ingress.rs`.

**Size.** Each phase fits in one session, bookkeeping included. PHASE-02 is the
largest: the loop change is small, but it carries most of the mutation
table, each run an edit, a scoped `cargo test`, a record and a restore.
PHASE-03 is long in wall time but light in tokens.

**Mutation evidence** goes in the phase's sheet in `notes.md`, under a
**Mutation evidence** heading, one row per mutation. Each row gives the edit
(quoted), the command, that the mutated build **compiled**, the cases that
went red **by name** and on which assertion, and that the restore is green. A
mutation that did not compile is not evidence
(`docs/memory/a-negative-control-that-does-not-compile.md`). Runs use
`--no-fail-fast`. Mutations are applied by copying the file to the scratchpad
and copying it back, never with `git checkout` or `git stash`, and
`git status` is clean after each restore.

**The scoped command** for a mutation is
`cargo test -p goad --test renderer --no-fail-fast -- ingress::`.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | done at design (`research.md` Thread 3; OQ-1) |
| AC-2 | PHASE-02/VT-1 (T2(a), the bound, M1 and M9) and PHASE-01/VT-1 (T3's R1, the leading edge, which PHASE-02's M5 reds); PHASE-03/EX-2 (the bound's margin under load) |
| AC-3 | PHASE-01/VT-1 and EX-3 (T3 reads the window; M0); PHASE-02/VT-1 and VT-2 (T2(c), T4), EX-5 (M0 over T2–T4) |
| AC-4 | PHASE-01/EX-5 and PHASE-02/EX-6 (every other case keeps its assertions); PHASE-01/VT-3 and PHASE-02/EX-5 (VT-7's added assertion and the R2 mutation); PHASE-02/VT-2 (T4: a command presents at once, M7); PHASE-02/VA-1 (I-3, I-4 by review) |
| AC-5 | `canon-delta.md` as drafted at design; PHASE-03/EX-5 (every case it cites resolves); **promoted at audit** |
| AC-6 | PHASE-03/VH-1 (the run); **recorded in `audit.md` §Evidence at audit** |
| AC-7 | **close only** — `docs/follow-ups.md` FU-2 |

`design.md` §9's mutations, by phase. Each must red the cases named, on the
assertion named. "Also" lists reds the plan's trace expects. They are recorded,
not required.

| id | edit, spelled so it compiles | must red | phase |
|---|---|---|---|
| M0 | `SlintGlass::present`: `write_if_changed(&self.diagnostics, lines);` → `drop(lines);` | PHASE-01: T3 (R1 is never shown), VT-7 (assertion 1 and the added one). PHASE-02: T2 ((b) and (c)), T3, T4 (precondition) | 01, 02 |
| M1 | every refusal presents: in the `Fired::Ingested`/`None` branch, insert `if !surface_stale { continue 'serving; }` before `surface_stale = true;` | T2(a), by orders of magnitude. Also T4's precondition. | 02 |
| M2 | leading edge only: set `surface_stale` only when `next_refusal_present.deadline() <= Instant::now()` | T2(c) | 02 |
| M3 | debounce: the `None` branch also resets `next_refusal_present` to `now + I` | T2(b) | 02 |
| M4 | `REFUSAL_PRESENT_INTERVAL` = 3 s | T2(b) | 02 |
| M5 | trailing edge only: when `surface_stale` is first set, reset `next_refusal_present` to `now + I` | T3 (R1's bound) | 02 |
| M6 | the arm resets to `now + 3I` | T3 (R2's bound) | 02 |
| M7 | a `Fired::Command` whose `dispatch` answers `None` `continue 'idle`s | T4 (`at − sent`) | 02 |
| M8 | the top present also resets `next_refusal_present` to `now + I` | T3 (R2's bound) | 02 |
| M9 | `REFUSAL_PRESENT_INTERVAL` = 600 ms | T2(a), by one present | 02 at rest; 03 under load |
| R2 | the outer ingress-stopped fold `continue 'idle`s instead of `'serving` | VT-7's added assertion | 02 |

**Why these spellings.** `unused` is `deny` workspace-wide (`Cargo.toml`
`[workspace.lints.rust]`). Deleting the `write_if_changed` call outright leaves
`lines` unused, and dropping the `surface_stale = true;` write leaves the flag's
`mut` unused. Either way the mutated build fails to compile and proves nothing.
If a spelling above still does not compile once the real code exists, respell
it so the behaviour is the one named, and record the respelling.

**Held by review, not by a test** (`design.md` §9). The yield per arrival
(R3), D4's ordering, and D8. PHASE-02/VA-1 records the review, and audit's code
review re-reads it.

---

## PHASE-01 — the recording glass

**Objective:** every timed claim in `ingress.rs` can be read from a log of what
the **window** held at each present, and the cases that already pass on
today's loop read it.

**Surfaces:** `crates/goad/tests/renderer/ingress.rs`. `design.md` §9 and
`canon-delta.md` only if a case is renamed. `crates/goad/src/glass.rs` for the
M0 mutation only, restored byte-identical.

**Entry**
- EN-1 — `plan.md` accepted by the user; HEAD at or after the acceptance
  commit; `just check` exits 0 on it.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `CountingGlass` is gone. `RecordingGlass` wraps the real
  `SlintGlass` and a strong clone of its `PromptWindow`. Each present
  **delegates first**, then appends `Presented { at, mode, lines }`, read from
  the window: `std::time::Instant::now()`, `get_mode()` and
  `get_diagnostic_lines()`. The log is shared as `Rc<RefCell<Vec<Presented>>>`.
  A count is the log's length. Its doc says why it reads the window and not the
  `Frame` (D12), and what M0 shows.
- EX-3 — M0 was run on this phase's tree, compiled, and redded T3 and VT-7.
  Recorded.
- EX-4 — the writer side records timing. One connection yields its `sent`
  instant (taken **before** connecting), the reply, and the reply's instant.
  `flat_out` takes an envelope-per-index function and returns those records.
  There is one writer loop. The constant-envelope callers keep their
  assertions, however their call is spelled.
- EX-5 — AC-4: `git diff` from the entry commit over `ingress.rs` changes no
  case other than VT-4 (now T3), VT-5 (now T1) and VT-7. VT-7's diff is the move
  onto the log plus one assertion. Every other assertion of those three that
  the design keeps is still there.
- EX-6 — the interval is mirrored:
  `const REFUSAL_PRESENT_INTERVAL: Duration = Duration::from_secs(1);`, with a
  doc in the shape of the file's `MINIMUM_SPACING` mirror (private on purpose,
  D11; the mirror is checked by nothing but its comment).

**Verification**
- VT-1 — `ingress::a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once`
  (T3; replaces `…reaches_the_diagnostics_surface`). An accepted `ENVELOPE`
  (pinned a minute off), then a `too_soon` refusal R1, then a wait until R1's
  present + 1.25·`I`, then `Command::OpenDiagnostics`, then at once a numbered
  shape refusal R2. Asserts R1's present has `at − sent ≤ I/2`, and R2's
  present is in `WindowMode::Diagnostic` with `at − sent ≤ I/2`. "R*n*'s
  present" is the first logged present whose lines name R*n*: `too_soon` for
  R1, its numbered key for R2. **Green on
  today's loop**; its controls are PHASE-02's M5, M6 and M8.
- VT-2 — `ingress::a_flat_out_writer_raises_no_evaluation_rate` (T1; VT-5
  renamed). Assertions 1 and 2 unchanged. Assertion 3, the `within` wait
  that served it, and the counting glass are gone. Its doc no longer claims
  a per-refusal cost, and says what the same flood costs a person is R-15's
  and is verified there (`canon-delta.md` Change 4).
- VT-3 — `ingress::a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
  (VT-7). Assertion 2 counts the log's length. Added: the first logged present
  whose lines contain `ingress has stopped` comes less than
  `MINIMUM_SPACING / 2` after an instant taken immediately before
  `spawn_local`. **Green on today's loop**; its control is PHASE-02's R2.
- VA-1 — M0 (EX-3).

**Notes for the implementer**

- **Citations in this file.** A bare `plan.md` or `design.md` in
  `ingress.rs` means **slice 004's**. New doc comments cite
  `docs/slices/011/design.md` (or "slice 011") explicitly. Cite by symbol,
  never by line.
- **Delegate first, then read.** Recording before the delegate returns reads
  the previous present's window, and M0 would not tell the two apart.
- `WindowMode` is `goad::generated::WindowMode`. `mode` is an `in` property,
  which has a getter, as `diagnostic_lines` does.
- **Numbered shape refusals** are an envelope with one unknown key:
  `EnvelopeFault::Unknown` names the key, and the check runs before any other
  field is read (A-2, verified at plan in `goad_shell::ingress::envelope`). The
  surface line is `no action taken: an event was refused (invalid_envelope): unknown key \`<key>\``.
  Give each case its own key prefix, so one case's log cannot match another
  case's key.
- **Each new socket gets its own `socket_path` case name.** `claim` enforces
  that the names are distinct.
- `within`/`until` poll every 5 ms, and the lag of that poll counts against
  nothing. Timed claims are read from the log's `at`, never from the moment a
  poll noticed.
- The `use std::cell::Cell` import goes with `CountingGlass`. `RefCell` takes
  its place.
- Tests are held by the workspace clippy set, `-D warnings` included:
  `as_conversions`, `integer_division` and `cast_*` are all denied here.
  Compute any ceiling in integer milliseconds with `u128::div_ceil`, and
  compare counts through `try_from`.
- M0 is the only edit outside `ingress.rs`. Copy `glass.rs` to the scratchpad,
  mutate it, run the scoped command, then copy it back, and confirm with
  `git diff --stat` that the file is unchanged.

---

## PHASE-02 — the coalescing loop

**Objective:** refused arrivals decided while idle cause at most one present
per interval, on both edges, and every other path to the top present is
unchanged.

**Surfaces:** `crates/goad/src/controller.rs` (`serve`, the new constant, and
the doc comments listed in EX-4), `crates/goad/tests/renderer/ingress.rs`.
`design.md` §9 and `canon-delta.md` only if a case is renamed.
`crates/goad/src/glass.rs` for the M0 mutation only, restored byte-identical.

**Entry**
- EN-1 — PHASE-01 is `done` in `notes.md`, and its EX criteria hold on HEAD.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `serve` is `design.md` §5.2, whole:
  - `REFUSAL_PRESENT_INTERVAL` is private, with the doc D11 and §5.2 give it.
  - `next_refusal_present` is initialised to `sleep_until(started)`. It has
    one write site, in the new arm:
    `now.checked_add(REFUSAL_PRESENT_INTERVAL).unwrap_or(now)`.
  - `surface_stale` is declared immediately before `'idle: loop`, outside its
    body. It has one write site, the `Fired::Ingested` branch when `ingest`
    answers `None`, which then `continue 'idle`s.
  - The new arm sits after `sleep` and immediately above
    `ingress.arrival()`, is guarded `if surface_stale`, builds no `Fired`,
    and `continue 'serving`s.
  - Every `continue` and `break` inside `'idle` is labelled.
- EX-3 — I-4 holds: `floor_until`, `event_floor_until` and `sleep` each keep
  their one write site, and the new arm writes none of them.
- EX-4 — the doc comments `design.md` §5.2 lists are true of the code:
  `refuse_arrival` (the outer arm no longer `continue`s to the top),
  `ingest`'s *why the loop `continue`s* sentence, the comment on the
  `let Some(attempted) … else` (a refused arrival no longer reaches it), and
  the inner arm's F-15 remark (*"the `Some` branch below … still presents
  nothing — `review-design.md` F-15's measured cost …"*). None of them counts
  anything nothing checks.
- EX-5 — every PHASE-02 mutation in the Coverage table was run, compiled, and
  redded its named cases on the named assertion. M9 went red at rest in each
  of at least five runs. Recorded.
- EX-6 — AC-4: `git diff` from PHASE-01's closing commit over `ingress.rs`
  adds T2 and T4 and changes no existing case, except to record in T3's doc
  which mutations are its controls. `scheduling.rs`, `wiring.rs` and the
  `event_loop*` targets are untouched, and pass.

**Verification**
- VT-1 — `ingress::a_flood_of_refusals_updates_the_window_once_per_interval_with_the_latest`
  (T2). `Command::Evaluate(Stimulus::Requested)` first, answered by
  `NEXT_CHECK_A_MINUTE_OFF`, and awaited until absorbed (the `next_check`
  line). Then a numbered shape flood runs for 2.5 s. The case waits until a
  present names the last key, then stops. It asserts (a), (b) and (c) exactly
  as `design.md` §9 states them, `ε = I/2`. **Red on today's loop on (a)**,
  recorded before the loop is written.
- VT-2 — `ingress::a_command_during_a_coalesced_interval_presents_at_once_and_carries_the_refusal`
  (T4). A pinning `Command::Evaluate` first, as in T2 (`plan-log.md`). Then
  numbered refusal A, B at once, and `Command::OpenDiagnostics`. The
  precondition and the assertion are exactly as in `design.md` §9. **Red on
  today's loop on the precondition**, which is recorded as such and is not
  T4's own evidence. That evidence is M7.
- VT-3 — T3, T1 and VT-7 (PHASE-01) pass unchanged against the new loop.
- VA-1 — **review of what no test reaches** (`design.md` §9). Record each by
  symbol in the phase sheet:
  - D4: the arm sits above `ingress.arrival()`.
  - D8: no present on the `Ending` arms.
  - D6: the ingress-stopped fold and the refusal site are not coalesced.
  - I-3: only the `Fired::Ingested`/`None` path and the new arm changed
    behaviour; every other `continue` reaches the top present.
  - R3: the yield per arrival still rests on `bind`'s `mpsc::channel(1)` and
    `accept_loop`'s `handle` awaiting the answer.
- VA-2 — the domain-vocabulary scan ran over the new identifiers. `site` is
  in its `DOMAIN` list and `code_of` strips only comments, so no new code
  identifier or string in `controller.rs` may contain it as a word (for
  example, no `refusal_site`).

**Notes for the implementer**

- **Order.** Write T2 and T4 → see them red and record it → the loop → green →
  refactor → rewrite the docs → run the mutations → `just check`.
- **The shape of `'idle`.** `let (attempted, refusal_re_arms) = if let Some(drained) = drained { … } else { let mut surface_stale = false; 'idle: loop { let fired = select! { … }; …; break 'idle (attempted, refusal_re_arms); } };`
  The `let Some(attempted) … else { continue; }` and the refusal site stay
  **outside** `'idle`, so their bare `continue` still targets `'serving`. If
  one of them ends up inside `'idle`, a refusal-site refusal would stop
  presenting, and only review would notice (R2).
- **The `Ending` arms become `break 'serving Ending::…`.** A bare `break`
  would target `'idle`, and `'idle` yields a tuple, so it will not compile.
  That is intended.
- **The pinned `Sleep`.** `&mut next_refusal_present` is polled exactly as
  `&mut sleep` is. An elapsed `Sleep` stays `Ready`, and a deadline already
  past fires once the driver advances (A-1). Verified at plan against tokio
  1.53.1: `STATE_DEREGISTERED` persists, and `Wheel::insert` answers
  `InsertError::Elapsed`, which the driver fires at once. A deadline in the
  past is therefore the leading edge, with no special case (D3).
- **T2(a)'s resolution goes beside the case**, in its doc (review Round 4).
  The count cannot tell the host's interval from one down to **about
  0.65 s**, a threshold that moves with the flood's phase. Below that, the
  constant's value is held by review and by the mirror's comment, as
  `MINIMUM_SPACING`'s is. Use that figure, not 0.7 s.
- **M9 reds by one present.** The plan's trace: over a 2.5 s flood at 600 ms,
  six presents against a ceiling of five. If each firing lags its deadline by
  more than about 25 ms, one firing drops out of the flood, and the count is
  five against five, which is green. At rest the lag is a few ms. Record the
  presents' spacing on each M9 run: it is the figure PHASE-03 needs.
- **Every timed assertion states which way load moves it** in a comment
  beside the bound (`design.md` §9's load column; memory
  `margin-size-is-not-margin-direction`). Measured figures do **not** go into
  comments. A performance figure in a comment is a measurement claim that
  nothing re-checks.
- **The mutation spellings** are in the Coverage table. M2 reads
  `next_refusal_present.deadline()`, which `Sleep` exposes.
- **Pedantic clippy is `deny`.** `needless_continue` may fire if a
  `continue 'idle` ends up as the last statement of the loop body. Fix it by
  **spelling** only. If the only fix changes which label a path takes, STOP.

**STOP conditions** (the phase sheet copies them)
- A mutation does not red its named case, or reds it on a different
  assertion. Do not add or tighten a case to make it red without consulting.
- T2 or T4 is green on today's loop before the change.
- Any existing case outside T1, T3 and VT-7 needs an edit to stay green. That
  is AC-4 failing.
- A lint can be satisfied only by changing a label, an arm's position, or a
  write site.

---

## PHASE-03 — under load, and in front of a person

**Objective:** every timed bound's margin is known at the bound on a loaded
machine, M9 is seen red under load, a person has seen the host stay
responsive under a flood, and every case the canon delta cites exists by that
name.

**Surfaces:** `docs/slices/011/flood.py` (new), `docs/slices/011/notes.md`.
`canon-delta.md` and `design.md` §9 only if EX-5 finds a name drifted.
`crates/goad/tests/renderer/ingress.rs` and `controller.rs` for temporary
instrumentation and M9 only, each restored byte-identical. No committed code
change: if a margin fails, STOP.

**Entry**
- EN-1 — PHASE-02 is `done`, and its EX criteria hold on HEAD.
- EN-2 — before generating load, the orchestrator has confirmed that no other
  session is measuring on this machine (`measure-011` or any successor). Load
  from this phase would corrupt that session's figures.

**Exit**
- EX-1 — `just check` exits 0 on the clean tree.
- EX-2 — **margin evidence** in the phase sheet: one row per timed quantity,
  giving the bound, which way load moves it, the worst value at rest and the
  worst under load, the run count for each, and the load average at the start
  and end. The quantities:
  - T2(a): count against ceiling, and the first flood present's lag behind its
    refusal's `sent`;
  - T2(b): the widest gap between consecutive flood presents, and how many
    presents precede the last reply;
  - T2(c): `at − last reply`;
  - T2's per-firing lag: each gap minus `I`;
  - T3: `at − sent` for R1 and for R2;
  - T4: `sent − A.at`, and `at − sent`;
  - VT-7: the first time `ingress has stopped` is shown, less the spawn
    instant.
- EX-3 — M9 at rest and under load, at least ten runs each, tallied red and
  green, with the per-firing lag of each run. Every run at rest is red, and at
  least one run under load is red.
- EX-4 — `just check` exits 0 once under the same load.
- EX-5 — every case name cited by `canon-delta.md` Changes 3, 4 and 5 appears
  in `cargo test -p goad --test renderer -- --list`. So does every name in
  `design.md` §9. Any drift is repaired in the delta and in the design in one
  commit, and recorded.
- EX-6 — `docs/slices/011/flood.py` exists, and is standard-library Python 3
  only. Its arguments are the socket path and a writer count (default 4). Each
  writer loops: connect, send one numbered unknown-key envelope, read the reply,
  close. Once a second it prints the refusals so far and the last key sent, and
  on Ctrl-C it prints the last key and exits 0. Its header says what it is for
  and that it is not part of the gate.
- EX-7 — `notes.md` Harvest is current, and §Open carries every residue this
  slice leaves: FU-3's added citation, the memory
  `a-refusal-is-recorded-not-shown` to re-check at close, and M9's tally if
  any run under load was green.

**Verification**
- VA-1 — **instrument at the bound.** Copy `ingress.rs` to the scratchpad and
  add temporary `eprintln!` lines that print each EX-2 quantity. Run
  `cargo test -p goad --test renderer -- --nocapture` ten times at rest and
  ten under load. The whole target, in parallel, as the gate runs it: the
  other modules' cases are part of the load a gate run puts on these bounds. Copy the file back. `print_stderr` is
  `deny` under clippy, so the instrumentation must be gone before EX-1.
- VA-2 — **load.** Launch four busy loops per core, each under `timeout`, so
  that they end on their own (for example
  `timeout 900 sh -c 'while :; do :; done'`, `4 × nproc` times). Launch them
  with the Bash tool's `run_in_background`, never with a trailing `&` (memory
  `gui-launch-needs-a-pipe`). Record `/proc/loadavg` at the start and end of
  every run. The user's own concurrent builds count toward the load and are
  welcome, but are not relied on. Slice 003 measured at a load average of
  164–170 on 32 cores. Aim for that.
- VH-1 — **AC-6.** Hand the user the steps below, pasted into chat (memory
  `hand-over-the-steps-not-the-pointer`). Record in the phase sheet what they
  report for each observation, in their words, and the commit it was run on.
  Audit cites it in `audit.md` §Evidence.

**AC-6 hand-over steps.** The user's shell is nu, so each command is one line
and uses no bash syntax.

1. In one terminal, at the repository root: `just demo`. The form comes up.
2. In a second terminal, at the repository root:
   `python3 docs/slices/011/flood.py ./goad-demo.sock`. It prints a refusal
   count and the last key about once a second.
3. While it runs, click into a text field in the form and type a sentence.
   *Look for:* each character appears as it is typed, and nothing typed is
   reverted.
4. Activate **Diagnostics** from the tray menu. *Look for:* the pane opens
   promptly. Before this slice it took 6.8–8.8 s (`research.md`, the
   responsiveness probe).
5. Watch the pane's line, and hover the tray icon for its tooltip. *Look
   for:* both change about once a second, each naming a `flood-<n>` key, with
   `n` rising.
6. Ctrl-C the flood. *Look for:* within about a second, the pane shows the
   last key `flood.py` printed, and then stays.
7. Quit the demo from the tray.

**Notes for the implementer**

- **Direction before ratio.** Rank the margins by which way load moves each
  one, then by size. T4's precondition (`sent − A.at < I/4`, a 250 ms budget)
  is the tightest bound that load pushes toward red. Slice 003 measured a
  worst loaded `until` of 185 ms. Watch it first.
- **A timed assertion that goes red under load is a defect, not a flake**
  (memory `margin-size-is-not-margin-direction`). STOP and report it with its
  figure. Do not widen the bound.
- **M9 green under load** is what review Round 4 predicted: a stall drops a
  firing out of the flood. It is not a gate failure, since M9 is a control,
  not a case. Record the tally and the lag. If **no** run under load is red,
  STOP and report, because whether the control still stands is the
  orchestrator's call.
- Stop the busy loops before EX-1's clean run, and before the AC-6 hand-over.
  Check with `pgrep -f "while :; do :; done"`.
- `flood.py`'s envelope: `{"source":"flood","kind":"flood","timestamp":"2026-01-01T00:00:00Z","data":{},"flood-<n>":0}`.
  The unknown key is refused before the other fields are read (A-2).

**STOP conditions**
- A timed assertion reds under load.
- No run of M9 under load is red.
- EN-2 cannot be confirmed.
- AC-6's person reports the window not staying responsive, or the pane not
  updating. That is a design question, and not one for this phase to fix.

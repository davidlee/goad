# Notes — Slice 011

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the recording glass | done | 2026-09-26 |
| PHASE-02 — the coalescing loop | done | 2026-09-26 |
| PHASE-03 — under load, and in front of a person | pending | 2026-09-26 |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — the recording glass

**Objective** (quoted, `plan.md` PHASE-01): *every timed claim in `ingress.rs`
can be read from a log of what the **window** held at each present, and the
cases that already pass on today's loop read it.*

**Written by the orchestrator, not the executor** (`plan-log.md`, plan
accepted with no plan review): this sheet is the plan's second reading.
Where it restates a plan criterion it quotes it; where it narrows one it says
so.

**Surfaces — a closed list. Anything else is a STOP.**
- `crates/goad/tests/renderer/ingress.rs` — the only file this phase commits
  code to.
- `crates/goad/src/glass.rs` — M0 only, copied to the scratchpad and back,
  byte-identical after (`git diff --stat` shows nothing for it).
- `docs/slices/011/design.md` §9 and `canon-delta.md` — only if a case is
  renamed away from the name the plan commits to.
- `docs/slices/011/notes.md` — this sheet, §Status, §Harvest.

**Which case is which — the names in the plan are ambiguous in the file.**
`ingress.rs` holds two numbering schemes: slice 004's PHASE-04 cases (socket
names `vt1`..`vt7`) and PHASE-05's (`p5vt1`..`p5vt6`, doc-labelled
`PHASE-05/VT-n`). The plan's three are, by symbol:

| plan says | symbol today | socket | becomes |
|---|---|---|---|
| VT-4 → T3 | `a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface` (PHASE-05/VT-4) | `p5vt4` | `a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once` |
| VT-5 → T1 | `a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal` (PHASE-04/VT-5) | `vt5` | `a_flat_out_writer_raises_no_evaluation_rate` |
| VT-7 | `a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating` (PHASE-04/VT-7) | `vt7` | name unchanged |

`PHASE-05/VT-5` (`a_shape_refusal_decided_during_an_exchange_…`) is **not**
the plan's VT-5 and must not change (AC-4).

**Reading list** (by symbol; use `grep -n` then `sed -n` ranges, not whole
files — `ingress.rs` is ~1560 lines)
- `docs/slices/011/plan.md` §PHASE-01 whole, and §Coverage's M0 row and
  *Mutation evidence* paragraph.
- `docs/slices/011/design.md` §9 — the T1, T3 and VT-7 rows, D12 (§7), and the
  *Read-the-window control* paragraph.
- `ingress.rs`: the module doc and `use` block (top of file); the helpers
  `write_one`, `send`, `flat_out`; `CountingGlass` and its `impl Glass`; the
  three cases in the table above; the file's `MINIMUM_SPACING` mirror (the
  shape EX-6's new mirror copies).
- `crates/goad/src/glass.rs`: `SlintGlass::present` — the `set_mode` call at
  its top and the `write_if_changed(&self.diagnostics, lines)` call (M0's
  target).
- `crates/goad/tests/renderer/harness.rs`: `window_and_tray`, `glass_over`,
  `until` (panics on timeout); `crate::waiting::within` (returns `bool`).
- `crates/goad/src/wire.rs`: `Command::OpenDiagnostics`.

**Exit criteria, quoted from the plan** — EX-1..EX-6 and VT-1..VT-3, VA-1 are
binding as `plan.md` states them; read them there. The points below are the
traps, not a restatement.

- **Delegate first, then read** (EX-2). `RecordingGlass::present` calls
  `inner.present(frame)` and only then reads `window.get_mode()` and
  `window.get_diagnostic_lines()` (collected to `Vec<String>`), stamping
  `Instant::now()`. Reading before would log the previous present, and M0
  could not tell.
- **Holds a strong `PromptWindow` clone**, not the `Frame` (D12). The log is
  `Rc<RefCell<Vec<Presented>>>`; a count is its length. The doc says why it
  reads the window, and what M0 shows.
- **The writer** (EX-4): one connection yields `sent` (taken **before**
  `connect`), the reply, and the reply's instant. `flat_out` takes an
  envelope-per-index function and returns those records; **one** writer loop
  (DRY — no second `flat_out`). Every existing constant-envelope caller keeps
  its assertions, however its call is spelled — including
  `after_a_flood_of_malformed_envelopes_the_host_still_evaluates` (PHASE-05/VT-6).
- **T3** needs a live `mpsc::Sender<Command>` (today's case discards `_tx`).
  R2 is a numbered shape refusal: an `ENVELOPE`-shaped body plus one unknown
  key; the line reads ``… (invalid_envelope): unknown key `<key>` ``. Give T3
  its own key prefix. "R*n*'s present" = the first logged present whose lines
  name R*n* (`too_soon` for R1, R2's key for R2). Both bounds are
  `at − sent ≤ I/2`, where `I` is the new mirrored `REFUSAL_PRESENT_INTERVAL`.
  T3 keeps its old case's EX-5 pin (`NEXT_CHECK_A_MINUTE_OFF`) and says so.
  Its old read of `Served.controller` is replaced by the log — that is the
  point of the rewrite.
- **T1**: assertions 1 and 2 unchanged; assertion 3, its `within` wait and the
  counting glass go. Doc per VT-2: no per-refusal-cost claim; says the flood's
  cost to a person is R-15's, verified there (`canon-delta.md` Change 4).
- **VT-7**: assertion 2 counts the log's length. Added: the first present
  whose lines contain `ingress has stopped` has
  `at − spawned < MINIMUM_SPACING / 2`, `spawned` taken immediately before
  `spawn_local`. State beside the bound which way load moves it (toward red).
- **Clippy is `-D warnings`, pedantic `deny`**: `as_conversions`,
  `integer_division`, `cast_*` all denied — halve a `Duration` with
  `Duration / 2` only if clippy accepts it, else `checked_div`/`mul_f64` —
  whichever passes; record which. Counts compared through `try_from`.
- **Citations**: bare `plan.md`/`design.md` in `ingress.rs` mean slice 004's;
  new doc comments say `docs/slices/011/…` or "slice 011". Symbols, never line
  numbers. No measured figures in comments.
- `use std::cell::Cell` goes; `RefCell` arrives. `WindowMode` is
  `goad::generated::WindowMode`.

**Assumptions**
- *Verified at plan:* all three cases are green on today's loop; M0's spelling
  `drop(lines);` compiles (`unused` is `deny`, so deleting the call would
  not); an unknown envelope key is refused before other fields are read (A-2).
- *First tested here:* that the window's `diagnostic_lines` read after
  `inner.present` reflects that present — M0 is the proof.

**Mutation evidence** — fill `compiled?`, `redded`, `restore green`.

| id | edit | command | must red | compiled? | redded (case, assertion) | restore green |
|---|---|---|---|---|---|---|
| M0 | `SlintGlass::present`: `write_if_changed(&self.diagnostics, lines);` → `drop(lines);` | `cargo test -p goad --test renderer --no-fail-fast -- ingress::` | T3 (R1 never shown); VT-7 (assertion 1, and the added one) | yes | T3: red, `until` timed out waiting for R1's `too_soon` present (never shown). VT-7: red, `until` timed out waiting for assertion 1's fold present (never shown) — the added assertion (3) was never reached, since assertion 1's `until` panics first. `ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface` (not in this phase's table; reads the live window directly) also reds under this global mutation — recorded here as an observation, not a finding. | yes: `cp` back from scratchpad, `md5sum` matched the pre-mutation copy, `git diff --stat` empty for `glass.rs`; full `ingress::` suite re-ran green (13/13). |

Mutations: copy the file to the scratchpad, edit, run, copy back. **Never**
`git checkout`, `git stash`, `reset`, `rebase`, `commit --amend`.

**STOP conditions** — stop at a compiling point, write what happened here,
report `STATUS: BLOCKED`:
- M0 fails to compile, or fails to red T3 or VT-7 on the named assertion.
- Any case outside the three in the table needs an edit to stay green (AC-4).
- T3, T1 or VT-7 is red on today's loop after the move.
- A lint can be satisfied only by weakening an assertion or a bound.
- Any file outside Surfaces needs to change.
- Session budget: at ~200k tokens, stop at a compiling point, write the
  handover here, report `STATUS: PARTIAL`.

**Tasks**
- [x] set §Status PHASE-01 `in progress`
- [x] EX-6 mirror of `REFUSAL_PRESENT_INTERVAL`
- [x] `Presented`, `RecordingGlass`; `CountingGlass` gone (EX-2)
- [x] timed writer records; `flat_out` over an envelope-per-index fn (EX-4)
- [x] VT-7 onto the log + added assertion (VT-3)
- [x] T1 (VT-2)
- [x] T3 (VT-1)
- [x] refactor pass
- [x] M0 run and recorded (EX-3, VA-1)
- [x] EX-5 diff check: `git diff <entry> -- crates/goad/tests/renderer/ingress.rs` touches no other case
- [x] `just check` exits 0 (EX-1)
- [x] §Status `done`; §Harvest updated

**Decisions taken during execution**
- `send`'s own signature (`async fn send(path, envelope) -> String`) is kept
  exactly as it was, for its ~13 unrelated callers (STOP condition: no edit to
  a case outside the table). `write_one` now returns a `Timed` record
  (`sent`, `reply`, `replied`); `send` is a one-line wrapper over a new
  `send_timed`, which is what T3 calls for its own timing. `flat_out` takes
  `impl Fn(usize) -> String + Send + 'static` (an owned `String` per index,
  not `&'static str`) so a future numbered flood — PHASE-02's T2 — can build
  a distinct key per request; this phase's own two constant floods (T1,
  PHASE-05/VT-6) just ignore the index.
- `Timed.replied` is written but read by nothing in this phase (EX-4 asks
  for it; only PHASE-02's T2(c) reads it). Left in with
  `#[expect(dead_code, reason = …)]`, the carve-out `Cargo.toml` already
  documents for exactly this shape (a phased field landing ahead of its
  caller), rather than dropped and re-added next phase.
- `Duration / 2` (I/2, and `MINIMUM_SPACING / 2` in VT-7's added assertion)
  passed `cargo clippy -p goad --tests -- -D warnings` unchanged — no
  `checked_div`/`mul_f64` fallback was needed for the halving. The 1.25×
  factor in T3 (`REFUSAL_PRESENT_INTERVAL.mul_f64(1.25)`) has no integer
  spelling and uses `mul_f64` throughout.
- VT-7's assertion 1 (the fold on the surface) was moved onto the
  `RecordingGlass` log rather than left reading `window.get_diagnostic_lines()`
  directly: the added assertion 3 needs the same log entry's `at`, and reading
  both off one source keeps the case's two window-derived claims consistent
  with the phase's objective ("every timed claim … read from a log"). The old
  numbering 1/2/3 became 1/2/3/4 (3 added, old 3 renumbered 4) — doc comment
  updated to match, cited by name rather than position where reused elsewhere
  in the same comment.
- PHASE-05/VT-6's `flat_out` call site
  (`after_a_flood_of_malformed_envelopes_the_host_still_evaluates`) was
  respelled (`MALFORMED` → `|_index| MALFORMED.to_owned()`, and the reason
  loop reads `record.reply`) to match the new `flat_out` signature. This is
  the one sanctioned exception in the phase sheet's own text ("every existing
  constant-envelope caller keeps its assertions, however its call is
  spelled … including `after_a_flood_of_malformed_envelopes…`") — its
  assertions are byte-for-byte the same, confirmed by the `git diff` (only
  the `flat_out` call line and the loop variable's name changed).

**Findings**
- M0 also reds `ingress::ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`
  (PHASE-04/"R-15's last clause" case), which is outside this phase's table
  and untouched by this phase's diff. It reads `window.get_diagnostic_lines()`
  directly rather than through a log, so a mutation to the shared production
  write path reds it too. Not a defect in this phase's work — recorded because
  the mutation table only names T3 and VT-7, and a future reader diffing the
  actual `cargo test --no-fail-fast` output against this table should not
  read the third failure as a regression.
- **Orchestrator review at the phase commit (`94251b6`) — two assertions the
  move weakened, repaired.** VT-7's assertion 1 had polled for the window
  holding **exactly one** line (`row_count() == 1`); the move kept only "some
  line names the fold", and its `assert!` re-tested the predicate the `find`
  had just matched, so it could not fail. It now asserts the fold present's
  `lines.len() == 1`. T3's R1 predicate had dropped the old case's
  `was refused` conjunct; restored at both sites. `just check` exits 0 after;
  M0 re-run by the orchestrator reds T3, VT-7 and the case above, and nothing
  else; `glass.rs` restored byte-identical.

### PHASE-02 — the coalescing loop

**Objective** (quoted, `plan.md` PHASE-02): *refused arrivals decided while
idle cause at most one present per interval, on both edges, and every other
path to the top present is unchanged.*

**Written by the orchestrator, not the executor.** Where it restates a plan
criterion it quotes it; where it narrows one it says so. It narrows none.

**Entry** — PHASE-01 `done`; its closing commit is **`485980b`** (the
orchestrator's review repair on top of `94251b6`). EX-6's AC-4 diff runs from
`485980b`.

**Surfaces — a closed list. Anything else is a STOP.**
- `crates/goad/src/controller.rs` — `serve`, the new constant, and the doc
  comments EX-4 lists. Nothing else in the file.
- `crates/goad/tests/renderer/ingress.rs` — T2 and T4 added; T3's doc gains
  the names of its controls; `Timed.replied`'s `#[expect(dead_code, …)]` goes
  (T2(c) reads it — the `expect` becomes unfulfilled and fails the gate if
  left).
- `crates/goad/src/glass.rs` — M0 only, scratchpad copy and back,
  byte-identical.
- `docs/slices/011/design.md` §9, `canon-delta.md` — only on a rename.
- `docs/slices/011/notes.md` — this sheet, §Status, §Harvest.
- `docs/slices/011/design-log.md` — **widened mid-phase** (orchestrator
  message, 2026-09-26): one append-only entry recording the M8/M8b decision.

**Reading list** (by symbol; `grep -n` then `sed -n`)
- `plan.md` §PHASE-02 whole; §Coverage (the M-table, *Why these spellings*,
  *Held by review*); §Sequencing (*What is red on today's loop*).
- `design.md` §5 whole (§5.1–§5.5 — the loop's shape, state table, dynamics,
  I-1..I-4, A-1), §7 D3–D9 and D11, §9 T2, T3, T4 rows.
- `controller.rs`: `serve` whole; `refuse_arrival`; `ingest`'s doc; the
  `MINIMUM_SPACING` constant and its doc (the new constant's doc sits beside
  it and says why it is **not** the "no second constant" that doc forbids).
- `ingress.rs`: the PHASE-01 additions — `Timed`, `write_one`, `send_timed`,
  `flat_out`, `Presented`, `RecordingGlass`, `REFUSAL_PRESENT_INTERVAL`,
  `T3_NUMBERED_REFUSAL`, and T3 (`a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once`)
  as the model for a log-reading case.

**The loop, as `design.md` §5.2 and the plan's note fix it.**

```
'serving: loop {
  drain; glass.present(..);                          // unchanged
  let (attempted, refusal_re_arms) = if let Some(drained) = drained { .. }
  else {
    let mut surface_stale = false;                   // fresh per entry
    'idle: loop {
      let fired = select! { biased;
        cancel      => break 'serving Ending::Stopped,
        commands    => None => break 'serving Ending::Closed, Some(c) => Fired::Command(c),
        &mut sleep  => { floor_until = ..; Fired::Scheduled }      // unchanged
        () = &mut next_refusal_present, if surface_stale => {       // NEW, D4: here
          next_refusal_present.as_mut().reset(now.checked_add(I).unwrap_or(now));
          continue 'serving;                                        // top presents
        }
        arrival     => None => { controller.refuse(&ingress_stopped()); continue 'serving; }
                       Some(a) => Fired::Ingested(a),
      };
      ..;  // Fired::Ingested whose ingest answers None:
           //   surface_stale = true; continue 'idle;
      break 'idle (attempted, refusal_re_arms);
    }
  };
  let Some(attempted) = attempted else { continue; };   // OUTSIDE 'idle — bare continue = 'serving
  refusal site ..                                       // OUTSIDE 'idle
  ..
}
```

- `next_refusal_present` is `Box::pin(tokio::time::sleep_until(started))`,
  declared beside `event_floor_until`. One write site: the new arm.
- A `Fired::Command` whose `dispatch` answers `None` must still leave `'idle`
  (to the top present). **That is T4's whole point**; M7 is the mutation that
  breaks it.
- `ingest` answers `None` for shape refusals, `too_soon`, **and** an unreadable
  clock (step 4). All three are "an arrival already answered and folded", and
  all three coalesce. That is the design's I-3 path, not a new decision.

**Test traps**
- **Key matching must be exact.** A numbered key `t2-17` is a substring of
  `t2-170`. Match the surface's rendering with its backticks —
  ``unknown key `t2-17` `` — never a bare `contains(key)`. Give T2 and T4
  their own prefixes, distinct from T3's `t3-r2`.
- **T2's flood** uses `flat_out` with an envelope-per-index closure (`ENVELOPE`'s
  shape plus one numbered unknown key). "Flood presents" are logged presents
  whose lines carry a T2 key. "The last key" is the last record's. The case
  waits (`until`) until a present names the last key, then stops.
- **T2(a)** — count ≤ `1 + ceil((last.at − first.at + ε) / I)`, `ε = I/2`, in
  integer milliseconds via `u128::div_ceil`; counts compared through
  `try_from`. Its doc states the resolution limit (≈0.65 s, phase-dependent —
  the plan's figure, not 0.7 s) and that below it the constant is held by
  review and the mirror, as `MINIMUM_SPACING`'s is.
- **T2(c)** reads `Timed.replied` of the last record: `at − replied ≤ 2I`.
- **T4's `sent`** for the command is an `Instant` the case stamps immediately
  before `tx.send(Command::OpenDiagnostics)`. The precondition is read from
  the log at that moment: the last present shows A and not B, and
  `sent − A.at < I/4`. Then the first present with `at > sent` is
  `WindowMode::Diagnostic`, shows B, `at − sent ≤ I/2`.
- **T2 and T4 pin first**: `Command::Evaluate(Stimulus::Requested)` answered by
  `NEXT_CHECK_A_MINUTE_OFF`, awaited until the `next_check` line shows (as T1
  and T3 await it). PL-6 says why.
- Each new case gets its own `socket_path` name and its own `scripted` name.
- Every timed bound carries a comment naming **which way load moves it**
  (design §9's load column). No measured figures in comments.

**Mutation evidence** — every row: quote the edit, compiled?, which cases red
**by name and assertion**, restore green, `git status` clean after. Command
unless noted: `cargo test -p goad --test renderer --no-fail-fast -- ingress::`.
If a spelling does not compile against the real code, respell to the same
behaviour and record the respelling.

| id | edit | must red | compiled? | redded (case, assertion) | restore green |
|---|---|---|---|---|---|
| M0 | `glass.rs` `SlintGlass::present`: `write_if_changed(&self.diagnostics, lines);` → `drop(lines);` | T2 (b), (c); T3; T4 (precondition) | yes | T2: red, `until` for the last key's present timed out (5 s). T3: red, `until` for R1's present timed out. T4: red, `until` for A's own present timed out. VT-7: red on assertion 1 (fold present), same as PHASE-01's M0 finding. `ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface` also reds (observation, not required, same as PHASE-01's finding — it reads the window directly). | yes: `cp` back from scratchpad, `diff -q` identical to the pre-mutation copy, `git diff --stat` empty for `glass.rs`; full `ingress::` suite re-ran green (15/15). |
| M1 | in the `Fired::Ingested`/`None` branch, insert `if !surface_stale { continue 'serving; }` before `surface_stale = true;` | T2(a), by orders of magnitude. Also T4's precondition | yes | T2(a): red, 56352 presents against a ceiling of 4. T4: red on the precondition, "the last present shows A and not B" (B presents at once). | yes, as above; suite green (15/15). |
| M2 | set `surface_stale` only when `next_refusal_present.deadline() <= Instant::now()` | T2(c) | yes | T2: red — the flood's leading-edge present is the only one that ever shows, so the `until` waiting for the *last* key's present times out (5 s). This is (c)'s violation manifesting as the test's own liveness wait rather than a comparison failure, since the assertions never execute. | yes; suite green (15/15). |
| M3 | the `None` branch also resets `next_refusal_present` to `now + I` | T2(b) | yes | T2(b): red, "at least two flood presents precede the last reply: 0" (the debounce means nothing presents until the flood goes quiet). **Also reds T3's R1 bound** (`1.000778568s after sent`) — observation, not required: T3 sends exactly one refusal, so "reset on every refusal" (M3) and "reset only on the first" (M5) are indistinguishable for it, and both delay R1's leading edge by a full `I`. | yes; suite green (15/15). |
| M4 | `REFUSAL_PRESENT_INTERVAL` = 3 s (production constant only) | T2(b) | yes | T2(b): red, "consecutive flood presents are at most 2I apart: 3.001001431s". **Also reds T3's R2 bound** (`1.752386596s after sent`) — observation: T3's own `1.25·I` wait between R1 and R2 is computed off the test's *mirror* (still 1 s), so a production-only interval change of this kind naturally desyncs the two, the same class of side effect PHASE-01 recorded for M0. | yes; suite green (15/15). |
| M5 | when `surface_stale` is first set, reset `next_refusal_present` to `now + I` | T3 (R1's bound) | yes | T3: red on R1's bound only (`1.00182714s after sent`); T2 and T4 stay green, exactly as the table predicts (only T3's single refusal is distinguishable under "first set" vs "every set"). | yes; suite green (15/15). |
| M6 | the arm resets to `now + 3I` | T3 (R2's bound) | yes | T3: red on R2's bound (`1.75313219s after sent`). **Also reds T2(b)** (`3.001371732s` gap) — observation: tripling the arm's own re-arm interval lengthens the gap between *any* two arm-caused presents, not just the one between R1 and R2, so a repeated flood (T2) is equally exposed. | yes; suite green (15/15). |
| M7 | a `Fired::Command` whose `dispatch` answers `None` `continue 'idle`s | T4 (`at − sent`) | yes | T4: red, "the command presents at once and carries B: 1.014437143s after sent" (`≥ 3I/4`, as the plan's trace predicts). T2, T3 and the rest of the suite stay green. | yes; suite green (15/15). |
| M8 | the top present also resets `next_refusal_present` to `now + I` | T3 (**R1's** bound — `design.md` §9's "R2" label is wrong; see Findings) | yes | T3: red on R1's bound, 3/3 runs (`998.817499ms`, `998.434945ms`, `997.729061ms` after `sent`, all ≈ `I`). T3's pin is an accepted envelope whose own top present, under this mutation, resets F before R1 ever arrives, so R1 pays the delay. T2, T4 and the rest of the suite stay green. **Accepted as evidence** (orchestrator decision, `design-log.md`); the table's original "→ R2" was corrected here, not in `design.md`. | yes; suite green (15/15) after each run. |
| M8b | in the `let Some(attempted) = attempted else { … }` branch, reset `next_refusal_present` to `now.checked_add(REFUSAL_PRESENT_INTERVAL).unwrap_or(now)` before the `continue` — D5's own rejected case, "a person's action holds back the next update" | T3 (R2's bound); R1 must stay green (the pin exchange does not take this branch) | yes | T3: red on R2's bound only, 3/3 runs (`1.004176922s after sent`, reproduced twice more). R1 stays green, exactly as predicted: T3's pin is an accepted envelope, which takes the exchange path, not this no-exchange branch. T2, T4 and the rest of the suite stay green. | yes; suite green (15/15) after each run. |
| M9 | `REFUSAL_PRESENT_INTERVAL` = 600 ms (production constant only) | T2(a), by one present — **≥ 5 runs at rest, every one red**; record each run's flood-present gaps | yes | T2(a): red in **5/5 runs at rest**, every one `6 presents against a ceiling of 5` — the test's own mirror stays at 1 s, so the ceiling is computed at the old interval while production presents every 600 ms; exactly the plan's own trace ("six presents against a ceiling of five"). Gaps recorded via a temporary `eprintln!` in T2 (added, run, then removed — `ingress.rs` restored byte-identical after): run 1 `[600.365604ms, 601.266288ms, 600.735586ms, 601.00196ms, 601.898939ms]`; run 2 `[600.677542ms, 601.00451ms, 601.022384ms, 600.96552ms, 601.88661ms]`; run 3 `[600.527576ms, 601.000203ms, 600.998619ms, 601.031062ms, 602.011755ms]`; run 4 `[600.089409ms, 601.001627ms, 600.996342ms, 601.006177ms, 601.466501ms]`; run 5 `[600.441637ms, 600.996266ms, 600.997022ms, 600.999941ms, 601.863772ms]`. Lag at rest is a few ms, as the plan's note predicts. | yes: `controller.rs` and `ingress.rs` both `diff -q` identical to their pre-mutation scratchpad copies; `git diff --stat` shows no change to either; full `ingress::` suite re-ran green (15/15). |
| R2 | the ingress-stopped fold `continue 'idle`s instead of `'serving` | VT-7's added assertion (3) | yes | VT-7: red on assertion 3, "the fold reaches the window at once, not after a scheduled firing: 3.001943844s after spawn" — the fold is trapped inside `'idle` until the scheduled arm eventually fires at `MINIMUM_SPACING`, matching `design.md` §9's own prediction ("shown only at `MINIMUM_SPACING`, by the engage present") exactly. Every other case, including T2/T3/T4, stays green. | yes; suite green (15/15). |

**Held by review (VA-1)** — record each by symbol here: D4 (arm above
`ingress.arrival()`); D8 (no present on the `Ending` arms); D6 (ingress-stopped
fold and refusal site not coalesced); I-3 (only the `Ingested`/`None` path and
the new arm changed behaviour; every other `continue` reaches the top); I-4
(`floor_until`, `event_floor_until`, `sleep` keep one write site each); R3 (the
per-arrival yield still rests on `bind`'s `mpsc::channel(1)` and
`accept_loop`'s `handle` awaiting the answer — cite by symbol in
`goad_shell::ingress`). **VA-2**: no new identifier or string in
`controller.rs` contains the word `site` (the domain scan's `DOMAIN` list).

**VA-1, reviewed 2026-09-26 (orchestrator's own read, not a fresh reviewer):**
- **D4** — confirmed by reading `serve`: the coalescing arm
  (`() = &mut next_refusal_present, if surface_stale => { .. }`) is the arm
  immediately above `arrival = ingress.arrival() => match arrival { .. }`,
  both inside the same `select! { biased; .. }`. `biased` ordering means the
  arm is checked, and wins if ready, before ingress is polled.
- **D8** — confirmed: both `Ending` arms (`break 'serving Ending::Stopped`,
  `break 'serving Ending::Closed`) leave `'serving` directly into
  `Served { ending, .. }`; nothing between the `break` and the function's
  return calls `glass.present`.
- **D6** — confirmed: the ingress-stopped fold's arm (`None => { .. continue
  'serving; }`) neither reads nor writes `surface_stale` or
  `next_refusal_present`; the one refusal site (`Err(refused) => { ..; if
  refusal_re_arms { sleep.as_mut().reset(floor_until); } continue; }`, outside
  `'idle`) likewise never touches either. Neither is coalesced with a refused
  arrival.
- **I-3** — confirmed by reading the `match fired` inside `'idle`: only the
  `Fired::Ingested(arrival)` arm's `None` case diverges (`surface_stale =
  true; continue 'idle;`); `Fired::Command` and `Fired::Scheduled` always
  `break 'idle (attempted, refusal_re_arms)` regardless of `Some`/`None`,
  exactly as before this phase.
- **I-4** — confirmed: `floor_until` is written only in the `&mut sleep` arm;
  `event_floor_until` only inside `ingest`, on an attempted ingested
  evaluation; `sleep` is reset only at the timer arm and at the exchange's
  own completion (`sleep.as_mut().reset(deadline_after(..))`). The new arm
  writes only `next_refusal_present`.
- **R3** — cited by symbol, unchanged this phase (`goad-shell` is outside
  Surfaces and untouched): `goad_shell::ingress::bind`
  (`crates/goad-shell/src/ingress/mod.rs`) creates `mpsc::channel(1)`, and
  `accept_loop` calls `handle(stream, &arrivals).await`, holding the accept
  task on each connection's reply before accepting the next. This is what
  paces an untrusted writer (`design.md` §5.4's remark by the inner arm) and
  is R-15's only witness beyond AC-6.

**VA-2** — verified two ways: (1) `git diff ccc9d61^ ccc9d61 -- crates/goad/src/controller.rs`
shows every occurrence of the substring `site` is inside a comment (`write
site`, `refusal site`), none in an identifier or string literal; (2)
`cargo test -p goad-boundary --test checks -- vocabulary` and the full
`no_workspace_member_names_the_users_domain` case both pass, and the scan's
own test suite confirms `code_of` strips comments before matching (so a
comment's "site" is not what the scan would catch in the first place).

**Order** (the plan's): T2, T4 → **see them red on today's loop and record
how** (T2 on (a); T4 on its precondition — expected; if either is green, STOP)
→ the loop → green → refactor → EX-4 docs → mutations → `just check`.

**STOP conditions** — stop at a compiling point, record here, commit the
sheet, report `STATUS: BLOCKED`:
- A mutation does not red its named case, or reds it on a different
  assertion. Do not add or tighten a case to make it red.
- T2 or T4 is green on today's loop before the change.
- Any existing case other than T1, T3 and VT-7 needs an edit to stay green
  (AC-4) — and T1, T3, VT-7 may not need one either (VT-3), except T3's doc.
- A lint (e.g. `needless_continue`) can be satisfied only by changing a label,
  an arm's position, or a write site. Spelling-only fixes are fine.
- A file outside Surfaces needs to change.
- ~200k tokens: stop at a compiling point, hand over here, `STATUS: PARTIAL`.

**Tasks**
- [x] §Status PHASE-02 `in progress`
- [x] T2 written; red on today's loop, recorded
- [x] T4 written; red on today's loop (precondition), recorded
- [x] `REFUSAL_PRESENT_INTERVAL`, `next_refusal_present`, `'idle`, `surface_stale`, the arm
- [x] green; refactor
- [x] EX-4 doc comments (`refuse_arrival`, `ingest`, the `let Some(attempted)` comment, the inner arm's F-15 remark)
- [x] T3's doc names M5, M6, M8 as its controls; `Timed.replied`'s `expect` gone (already done in PHASE-01's own text — no further edit needed)
- [x] mutations M0–M9, M8b, R2 run and recorded (M8b added per orchestrator decision, `design-log.md`)
- [x] VA-1, VA-2 recorded
- [x] EX-6: `git diff 485980b -- crates/goad/tests/renderer/ingress.rs` adds T2, T4 and changes no existing case beyond `Timed`'s `#[expect(dead_code, …)]` removal; `scheduling.rs` and `wiring.rs` untouched (empty diff); `event_loop*` targets all pass
- [x] `just check` exits 0
- [x] §Status `done`; §Harvest updated

**Decisions taken during execution**
- `ingest`'s `None` arm is spelled as a `let Some(pending) = ingest(..) else { surface_stale = true; continue 'idle; }; Some(Ok(pending))`, matching `dispatch`'s existing `Option<Result<Pending, Refused>>` shape (`.map(Ok)`'s equivalent) rather than restructuring the `match fired` arms' types.
- T2 and T4 are pinned via a live `tx.send(Command::Evaluate(Stimulus::Requested))` rather than an accepted envelope (PL-6), unlike T1/T3's inherited pin. Verified this makes no difference to `event_floor_until` for T2's own flood, since a shape refusal (unknown key) never reaches `ingest`'s spacing check (step 1 refuses before step 3).
- T2's own key is `t2-{index}` (numbered per flood request via `flat_out`'s per-index closure); T4's are the fixed `t4-a`/`t4-b`. Matched with a backtick-delimited `` `key` `` substring (`names_key`) rather than a bare `contains`, per the sheet's own trap warning.
- `epsilon_ms` (T2(a)'s `ε = I/2`) is computed as `(REFUSAL_PRESENT_INTERVAL / 2).as_millis()`, not `REFUSAL_PRESENT_INTERVAL.as_millis() / 2`: the latter is `u128` division and clippy's `integer_division` (pedantic, denied) catches it; `Duration / 2` is not flagged, confirming PHASE-01's own "Learned" note extends to this file's new arithmetic too.
- `Timed.replied`'s `#[expect(dead_code, …)]` removed outright (T2(c) reads it via `records.last().expect(..).replied`); no other change to `Timed`, `write_one`, `send`, `send_timed` or `flat_out` was needed.
- T3's doc already named M5, M6 and M8 as its controls (written ahead of this phase, in PHASE-01's own text) — the sheet's task is satisfied with no edit.

**Findings**
- **M8 reds T3, but on R1's bound, not R2's, as the mutation table and
  `design.md` §9's T3 row both predict.** Reproduced 3/3 runs (`998.817499ms`,
  `998.434945ms`, `997.729061ms` after `sent`, all ≈ `I` — never near R2).
  Traced: T3's pin is an **accepted envelope** (`send(&path, ENVELOPE)`,
  inherited from PHASE-01/PHASE-04, "T3 keeps this case's old pin"), and that
  evaluation's own completion produces a top-level present before R1 is ever
  sent. Under M8 ("the top present *also* resets `next_refusal_present` to
  `now + I`", the literal spelling of D5's rejected "moving F on any
  present"), that pin present already pushes `next_refusal_present` to
  `pin_present_time + I`. R1 is sent moments later (bounded only by the
  test's own `until` polling granularity), so R1's own leading edge is what
  waits out nearly the whole interval — not R2's. This is not a spelling
  problem: M8 does not fail to compile, and no alternate literal spelling of
  "the top present also resets F" changes which present happens first. T2 and
  T4 (pinned via a live `Command::Evaluate` rather than an accepted envelope,
  per PL-6) are unaffected by this particular ordering, which is why the
  mutation still isolates to T3 as the table says — just not to the
  assertion it names.
  **Per the phase sheet's own STOP condition** ("A mutation does not red its
  named case, or reds it on a different assertion... Do not add or tighten a
  case to make it red without consulting"), this phase stopped here and
  reported `STATUS: BLOCKED` rather than reinterpreting M8's spelling or T3's
  assertions unilaterally.
  **Resolved** (orchestrator, under the autonomy grant, `design-log.md`
  2026-09-26): M8 stands as written; its red on R1 is accepted as valid
  evidence that the mutation reds T3, and the mutation table above is
  corrected to name R1 rather than R2. **`design.md` §9's own "M8 → R2" label
  is wrong and is left as-is** — not retro-fitted mid-slice — and is recorded
  here for audit's *Design drift not reconciled*. M8b is added to the table
  to hold D5's actual rejected case ("a person's action holds back the next
  update"), spelled as resetting `next_refusal_present` in the
  `let Some(attempted) = attempted else { … }` exit rather than at every top
  present; it is the row that must red T3 on R2's bound specifically, with R1
  staying green (T3's pin exchange never takes that branch).
  **Design drift not reconciled, for audit:** `design.md` §9's T3 row states
  "M8: F also reset at every top present (against D5) → R2"; the actual
  behaviour is → R1, for the reason above. `design.md` is not amended
  mid-slice (`docs/AGENTS.md`); audit reconciles or formally amends it.

- **Orchestrator review at `f564c29`.** The loop matches `design.md` §5.2
  (labels, write sites, the arm's position, the `None` path through `'idle`).
  One EX-4 item was not met: the inner arm's F-15 remark still placed
  F-15's cost in the inner `Some` branch, when that cost was the outer arm's
  and is now coalesced. Rewritten. M8b re-run by the orchestrator: T3 red on
  R2's bound only (≈1.004 s after sent); restored. `just check` exits 0.

### PHASE-03 — under load, and in front of a person

**Objective** (quoted, `plan.md` PHASE-03): *every timed bound's margin is
known at the bound on a loaded machine, M9 is seen red under load, a person
has seen the host stay responsive under a flood, and every case the canon
delta cites exists by that name.*

**Narrowed by the user's decision** (`plan-log.md`, *load testing
de-emphasised*): the oversubscription work is cut. Stated exactly:

| plan criterion | as the plan states it | in this sheet |
|---|---|---|
| EN-2 | orchestrator confirms no other session is measuring | **dropped** — this phase generates no load |
| EX-2 | worst at rest **and under load**, run counts, loadavg at start and end | **at rest only**, ≥ 10 runs; `/proc/loadavg` still recorded at start and end of each batch, so the machine's ordinary load is on the record |
| EX-3 | M9 ≥ 10 runs at rest **and** under load; ≥ 1 loaded run red | M9 ≥ 10 runs **at rest**, every one red, per-firing lag recorded |
| EX-4 | `just check` once under load | **dropped** |
| VA-2 | four busy loops per core | **dropped** — no busy loops, no `timeout … while :` |
| STOPs | "a timed assertion reds under load"; "no M9 run under load is red"; "EN-2 cannot be confirmed" | replaced by: **a timed assertion reds at rest**, or **any M9 run at rest is green** |

Everything else — EX-1, EX-5, EX-6, EX-7, VA-1 (at rest), VH-1 — binds as
`plan.md` states it. **VH-1 is not the executor's**: the orchestrator hands
AC-6 to the user after this sheet's machine work is done.

**Entry** — PHASE-02 `done`; HEAD at or after `0533411` (its review commit).

**Surfaces — a closed list.**
- `docs/slices/011/flood.py` (new).
- `docs/slices/011/notes.md` — this sheet, §Status, §Harvest.
- `docs/slices/011/canon-delta.md`, `design.md` §9 — only if EX-5 finds a
  name drifted.
- `crates/goad/tests/renderer/ingress.rs` and `crates/goad/src/controller.rs`
  — **temporary** instrumentation and M9 only, scratchpad copy and back,
  byte-identical (`git diff --stat` empty). No committed code change: if a
  margin fails, STOP.

**Reading list**
- `plan.md` §PHASE-03 whole (EX-2's list of quantities is binding; its
  *Notes for the implementer*, less the load paragraphs).
- `design.md` §9 (T2, T3, T4, VT-7 rows — bounds and load direction).
- `ingress.rs` by symbol: T2 `a_flood_of_refusals_updates_the_window_once_per_interval_with_the_latest`,
  T3 `a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once`,
  T4 `a_command_during_a_coalesced_interval_presents_at_once_and_carries_the_refusal`,
  VT-7 `a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`.
- §PHASE-02 above: its M9 rows (the gaps it recorded are this phase's
  baseline).
- `canon-delta.md` Changes 3, 4, 5 — the case names they cite.

**Margin evidence (EX-2, at rest).** Temporary `eprintln!` lines print each
quantity; run `cargo test -p goad --test renderer -- --nocapture` (the
**whole** target, parallel, as the gate runs it) ≥ 10 times; remove the
instrumentation (`print_stderr` is `deny`). One row per quantity: bound,
which way load moves it, worst value, run count. The quantities (quoted
from EX-2):
- T2(a): count against ceiling, and the first flood present's lag behind its
  refusal's `sent`;
- T2(b): the widest gap between consecutive flood presents, and how many
  presents precede the last reply;
- T2(c): `at − last reply`;
- T2's per-firing lag: each gap minus `I`;
- T3: `at − sent` for R1 and for R2;
- T4: `sent − A.at`, and `at − sent`;
- VT-7: the first time `ingress has stopped` is shown, less the spawn instant.

Rank the rows by direction first, then size (memory
`margin-size-is-not-margin-direction`). T4's precondition (`sent − A.at <
I/4`) is the tightest bound load pushes toward red.

**flood.py (EX-6)** — as `plan.md` states: standard-library Python 3 only;
args socket path and writer count (default 4); each writer loops connect →
one numbered unknown-key envelope → read reply → close; once a second prints
refusals so far and the last key; Ctrl-C prints the last key and exits 0;
header says what it is for and that it is not part of the gate. Envelope:
`{"source":"flood","kind":"flood","timestamp":"2026-01-01T00:00:00Z","data":{},"flood-<n>":0}`.
**Smoke it** without the host: the host needs a display, and putting a window
on the user's screen is VH-1's job, not this phase's. Run a throwaway
standard-library listener in the scratchpad that accepts on a socket path,
logs each line it reads and answers `{"accepted":false,"reason":"invalid_envelope"}`.
Confirm `flood.py` connects, sends one valid JSON line per connection with a
rising `flood-<n>` key, prints once a second, and on SIGINT prints the last key
and exits 0. Record what was checked, and that the real host's replies are
first seen at VH-1.

**Tasks**
- [ ] §Status PHASE-03 `in progress`
- [ ] margin evidence at rest, ≥ 10 runs, table filled, instrumentation removed
- [ ] M9 ≥ 10 runs at rest, tally + per-firing lag, restored
- [ ] EX-5: every case name in `canon-delta.md` Changes 3–5 and `design.md` §9 appears in `cargo test -p goad --test renderer -- --list`; drift repaired in one commit, recorded
- [ ] `flood.py` written and smoke-checked
- [ ] EX-7: §Harvest current; §Open carries FU-3's added citation, memory `a-refusal-is-recorded-not-shown` to re-check at close, and the M8 label drift
- [ ] `just check` exits 0 on the clean tree (EX-1)
- [ ] §Status: PHASE-03 `in progress — awaiting VH-1` (the orchestrator sets `done` after the user's run)

**STOP conditions**
- A timed assertion reds at rest, or any M9 run at rest is green.
- A file outside Surfaces needs to change, or a committed code change looks
  necessary.
- ~200k tokens: hand over here, `STATUS: PARTIAL`.

**VH-1 record** (orchestrator fills after the user's run)

**Decisions taken during execution**

**Findings**

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-26 · PHASE-02 done · the
`011: PHASE-02 — the coalescing loop` commit

### Produced
- `RecordingGlass`/`Presented` (`ingress.rs`) — the log every timed claim in
  this file now reads, replacing `CountingGlass`. Delegates to `SlintGlass`
  first, then reads `(Instant::now(), window.get_mode(), window.get_diagnostic_lines())`.
- `Timed` (`ingress.rs`) — one connection's `sent`/`reply`/`replied`.
  `write_one` is the sole connect-write-read primitive; `send`, `send_timed`
  and `flat_out` all go through it. `replied` is now read (T2(c)).
- `flat_out`'s shape: `impl Fn(usize) -> String + Send + 'static` in place of
  a constant `&'static str`, so a numbered flood (T2) can give each request
  its own key without a second writer loop.
- `controller::REFUSAL_PRESENT_INTERVAL` (private, 1 s, D11) — the coalescing
  arm's own throttle, alongside `MINIMUM_SPACING` but bounding a different
  axis (how long a fold may sit unpresented, not how often the host fires).
  `ingress.rs`'s own mirror (from PHASE-01) now matches it.
- The coalescing loop itself (`controller.rs::serve`): `next_refusal_present`,
  `surface_stale`, the `'idle: loop`, and the arm sitting above
  `ingress.arrival()` (D4). `ingest`'s `None` path now marks the surface stale
  and `continue 'idle`s instead of reaching the top present at once — the one
  behavioural change in the whole function (I-3).
- Two cases added: `a_flood_of_refusals_updates_the_window_once_per_interval_with_the_latest`
  (T2) and `a_command_during_a_coalesced_interval_presents_at_once_and_carries_the_refusal`
  (T4), both pinned via a live `Command::Evaluate` rather than an accepted
  envelope (PL-6).
- Three cases carried over renamed/rewritten in PHASE-01:
  `a_flat_out_writer_raises_no_evaluation_rate` (T1, ex-VT-5),
  `a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once` (T3,
  ex-`…reaches_the_diagnostics_surface`), and
  `a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
  (VT-7) — all three pass unchanged against the new loop except T3's doc,
  which now names M5/M6/M8 as its controls.

### Learned
- `Duration / 2` (and `* 2`, `* 3`) is clippy-clean under this workspace's
  lint table (`integer_division` is type-gated to primitive integers, not
  `Duration`'s `Div<u32>`/`Mul<u32>`); `u128::as_millis() / 2` **is** flagged,
  so a duration halved for millisecond arithmetic should be halved as a
  `Duration` first, then converted.
- A mutation to `glass.rs`'s single write site reds cases outside the phase
  that touched it (PHASE-01's finding, reconfirmed in PHASE-02: the same M0
  also reds `ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`).
  More generally in PHASE-02: several mutations that change a *global*
  quantity (the production interval's value, or the arm's own re-arm
  interval) red more than their named case, because the same knob governs
  every case that relies on it — recorded per-row rather than fought.
- **A mutation can red the right case on the wrong assertion, and that is not
  automatically a bug in the mutation.** M8 ("the top present also resets F on
  any present", D5's own rejected alternative) reds T3 on R1 rather than R2,
  because T3's own pin (an accepted envelope) produces a top present that
  moves F before R1 arrives — a fact about *this test's* pin, not about the
  mutation. `design.md` §9's "M8 → R2" label was wrong; corrected in
  `notes.md`, left as a *design drift not reconciled* for audit rather than
  edited mid-slice, and D5's actual case (a person's action, i.e. any
  no-exchange command, holding back the update) was covered by a new row,
  M8b, spelled at the one place that case can actually occur — the
  `let Some(attempted) = attempted else { .. }` exit.

### Open
- **Design drift not reconciled** (for audit): `design.md` §9's T3 row states
  "M8: F also reset at every top present (against D5) → R2"; PHASE-02 found
  the actual red lands on R1. `design-log.md`'s 2026-09-26 entry records the
  decision; audit reconciles or formally amends `design.md`.

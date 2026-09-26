# Notes — Slice 011

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the recording glass | done | 2026-09-26 |
| PHASE-02 — the coalescing loop | pending | 2026-09-26 |
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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-26 · PHASE-01 done · the `011: PHASE-01 — the
recording glass` commit

### Produced
- `RecordingGlass`/`Presented` (`ingress.rs`) — the log every timed claim in
  this file now reads, replacing `CountingGlass`. Delegates to `SlintGlass`
  first, then reads `(Instant::now(), window.get_mode(), window.get_diagnostic_lines())`.
- `Timed` (`ingress.rs`) — one connection's `sent`/`reply`/`replied`.
  `write_one` is the sole connect-write-read primitive; `send`, `send_timed`
  and `flat_out` all go through it.
- `flat_out`'s new shape: `impl Fn(usize) -> String + Send + 'static` in place
  of a constant `&'static str`, so PHASE-02's numbered flood (T2) can give
  each request its own key without a second writer loop.
- `REFUSAL_PRESENT_INTERVAL` (`ingress.rs`, private, 1 s) — the test file's
  own mirror of PHASE-02's not-yet-written production constant, ahead of it
  (EX-6).
- Three cases carried over renamed/rewritten: `a_flat_out_writer_raises_no_evaluation_rate`
  (T1, ex-VT-5), `a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once`
  (T3, ex-`…reaches_the_diagnostics_surface`), and
  `a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
  (VT-7, name unchanged, body moved onto the log plus one assertion).

### Learned
- `Duration / 2` is clippy-clean under this workspace's lint table (`integer_division`
  is type-gated to primitive integers, not `Duration`'s `Div<u32>`); no
  `checked_div`/`mul_f64` fallback was needed for a Duration halving here.
- `expect(dead_code, reason = …)` on one struct field is the sanctioned way to
  land an EX-4-mandated field a later phase reads — `Cargo.toml`'s own comment
  names this shape, and it applies exactly here (`Timed.replied`).
- A mutation to `glass.rs`'s single write site reds cases outside the phase
  that touched it (`ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`,
  PHASE-04's), because it reads the window directly rather than through a
  log — worth knowing before treating an M0 run's failure list as exhaustive
  against a phase's own mutation table.

### Open
- PHASE-02 is what gives `controller::REFUSAL_PRESENT_INTERVAL` a production
  value and reads `Timed.replied` for the first time (T2(c)'s bound).

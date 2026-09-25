# Research — Slice 011

**Producers:** Thread 1 and Thread 2 — research agent, reading the tree and
canon. Thread 3 — research agent, measuring on the running host (release build,
real compositor), with an attribution control built in a scratch worktree.
**As of:** 2026-09-26 · `fa266da`

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- unmarked — researcher claim: cited, not checked.

Design and plan may only load-bear ✓ rows, or rows they verify at point of use.
Verify what you lean on, not everything.

## Citation forms

Canon claims cite the document id (`SPEC-003 §4`, `ADR-007`). **Code claims
cite by symbol** (`serve`, `Glass::present`, a test's function name), not
`path:line`: `CLAUDE.md` §Working here overrides the template's line-number
form. An uncited claim is unverifiable by definition.

Measurement claims cite the script that produced them and the raw table below.
Scripts are in the session scratchpad
(`/tmp/claude-1000/-home-david-dev-goad/6c6c55c8-54ff-4559-b124-444887286d5d/scratchpad/`),
which does not outlive the session: `measure.py`, `run-set.sh`, `probe.sh`,
`derive.py`, `backend.sh`, `scratch.toml`, `control.patch`, and the raw
`results.jsonl`. Their content that matters is restated here.

## Thread 1 — governing canon

### Binding

- **SPEC-003/R-12** — a refused arrival is never queued, delayed or coalesced.
  This binds the *writer's* reply and the host's evaluation, not the
  presentation; coalescing presents does not touch it.
- **SPEC-003/R-15** — a refusal decided while idle MUST also reach the
  diagnostics surface. It states no bound on *when*. That is the clause this
  slice amends.
- **SPEC-002/R-12, ADR-004** — the spacing and the floor. A presentation
  deadline must not become an evaluation.
- **ADR-001** — stratum 3 only.

### The verification rows that record one present per refusal

Both rows are in SPEC-003 §7.

- **R-12's row** cites
  `ingress::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal`
  and states, as a measured fact, *"845/845, 1.000 per refusal, ~1690/s —
  F-15's settlement"*. That figure is slice 004's headless tier
  (`init_no_event_loop`, no compositing; `docs/slices/004/notes.md`, *F-15's
  number*). The test's `assert_eq!(cost, replies.len())` fixes the ratio at
  one, so any coalescing repair turns it red by design — AC-2 supersedes it.
- **R-15's row** cites, positive,
  `ingress::a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface`
  — which reads `served.controller.frame(false).diagnostics` after the loop has
  ended, i.e. the **retained model, not the window** (FU-2's point: it would
  stay green if the present were suppressed outright); negative,
  `ingress::a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`,
  which does read the live window; and for the ingress-stopped clause
  `ingress::a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
  and `ingress::ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`.

### Checked, not applicable

SPEC-001 (nothing crosses the backend boundary), SPEC-004 (no exit path),
ADR-003 and ADR-005 (workspace shape, envelope normalization), POL-001 (no new
gate command) — as `slice-011.md` records.

### Amendment candidates

R-15 (a bound on *when* an idle refusal reaches the surface), R-15's row (a case
that reads the window), R-12's row (drop the one-per-refusal record). Thread 3
adds nothing new to this list but changes what R-15's *why* should say (see
*Cross-thread findings*).

## Thread 2 — code map

### Hotspots

- `serve` (`crates/goad`, `controller.rs`) — the outer loop. Its first
  statement after the drain is `glass.present(controller.frame(…))`; the
  refused-arrival path reaches it through `ingest` returning `None` and the
  `let Some(attempted) = attempted else { continue; }`.
- `Glass::present` / `SlintGlass` (`glass.rs`) — what one present does.

### Cited facts

- `ingest` answers the writer and folds the refusal via `refuse_arrival`
  (`Answer::refused` then `Controller::refuse`) before it returns `None`; the
  present is the loop's, not `ingest`'s.
- The `select!` in `serve` is `biased`: `cancel.stopped()`, then
  `commands.recv()`, then the schedule's `sleep`, then `ingress.arrival()`
  **last**. A command already in the channel is always served before an
  arrival.
- `accept_loop` (`goad-shell`, `ingress/mod.rs`) is sequential: one arrival is
  judged and replied to before the next connection is accepted (I-2). So the
  flood's rate is set by how fast `serve` turns an arrival around.
- `serve` runs on the UI thread: `main.rs`'s `start` hands it to
  `slint::spawn_local`.
- `SlintGlass::present`, per call: `set_mode`; when a view is shown,
  `option_models` (rows and values, rebuilt from scratch); `set_values` with a
  fresh model; `options.set_vec` only when the view changed; the epoch bump;
  diagnostics lines through `write_if_changed`; next-check and notice text;
  tray image and hover text; then `hide()` or `show()` by surface.
- The tray is a StatusNotifierItem on the session bus
  (`org.kde.StatusNotifierItem-<pid>-1`); its `ToolTip` property carries the
  latest refusal, and its `/MenuBar` dbusmenu exposes *Diagnostics* (item id
  `3` at `fa266da`). Both were used as instruments in Thread 3.

### Precedents

Slice 004's headless measurement (*F-15's number*) is the one prior number; it
explicitly disclaims the running platform. 009's F-R4 was reasoned, not run.

## Thread 3 — the cost of a refused arrival's present

### Instruments

1. **Per-thread CPU.** `/proc/<pid>/task/<tid>/stat` fields `utime + stime`,
   read at the start and end of a ~9.9 s window (`measure.py`). Clock ticks at
   100 Hz, so resolution is 10 ms per reading — ≤0.2 % of a 9.9 s window.
   The UI thread is the thread whose tid equals the pid (the Slint event loop,
   where `serve` runs). All host threads are also summed.
2. **Compositor CPU**, same window, whole process. **The compositor is
   `umbriel`, not niri** — niri is not running on this machine; `umbriel`
   owns `wayland-0`.
3. **Refusal count** — every reply the writers read, by reason. The
   denominator for per-refusal figures is the refusals in the writers' 10 s
   run, scaled to the sampled window (`derive.py`).
4. **Responsiveness probe** (`probe.sh`). Two seconds into a flood, the tray's
   *Diagnostics* item is activated over D-Bus (`com.canonical.dbusmenu.Event`,
   id 3, `clicked`); `umbriel windows` is polled every 0.25 s until the scratch
   host's window title becomes `goad — diagnostics`. The latency is from the
   click to the first poll that sees the new title.

**What they hold.** (1) is the UI thread's total CPU, whatever it spent it on.
(4) is end to end from a tray activation to the compositor reporting the
window's new state: it detects an event loop that is not getting round to
input-side work.

**What they do not reach.**

- (1) does not break a present down. It says how much the UI thread spent, and
  (with the control) how much of that the present was — not whether
  `option_models`, the Slint binding re-evaluation the epoch bump triggers, or
  `show()` dominates.
- (1) cannot see rendering done on other threads. Slint's renderer here runs
  on the UI thread (no other host thread was busy beyond the tokio workers),
  so this is a small gap.
- (2) is the compositor's **whole-process** CPU. It includes every other client
  on a busy desktop (idle baseline 2.2–5.6 % of a core varies with what the
  person at the machine was doing). Differences of a few points are noise.
- (4) is not typing. A tray activation reaches the loop by a different route
  from a key press, and nothing here types into a field. Whether a person's
  keystrokes lag under a flood is **unobserved** and belongs to a human run
  (AC-6).
- The machine was shared (32 cores; other agents and a model process at
  ~50 % of a core). Per-thread CPU is robust to that; wall-clock latency in (4)
  is less so.

### Setup

- **Build**: release, `cargo build --release -p goad` at `fa266da`, copied to
  the scratchpad as `goad-head`. The daily `goad` systemd service was left
  running and untouched; it has its own config, socket and backend, and its
  window was distinguished from the scratch host's by title.
- **Config** (`scratch.toml`): `default_poll = "1h"`, ingress socket in the
  scratchpad, backend `backend.sh`: the demo backend's form (every field kind;
  `next_check` 45 minutes) for the host's own evaluations; for any watcher's
  accepted envelope a null view, so the form stays outstanding and is never
  replaced. With `SCRATCH_HIDDEN=1` every answer is a null view, so there is no
  form and the window is hidden.
- **Writer**: four Python processes, each looping connect → send one line →
  read the one-line reply → close. Two envelopes: `shape` (`{"source":"flood"}`,
  refused `invalid_envelope`, identical text every time) and `too_soon` (a valid
  envelope; one is accepted per 3 s spacing and runs a null-view exchange, the
  rest are refused `too_soon` or, while that exchange is in flight, `engaged`).
  The writers are not the bottleneck against the head build: 0.2–0.6 CPU-s of
  their 40 available per window.
- **Conditions**: (a) idle, no flood; (b) window hidden (no form); (c) window up
  with the form; (d) the diagnostics pane up — reached by activating the tray's
  *Diagnostics* item over D-Bus, so (d) was measurable after all. Three 10 s
  runs of each kind per condition.

### Attribution control

`control.patch`, applied in a scratch worktree of `fa266da` and built release
as `goad-ctl`: `serve` gains `skip_present`, set when the ingested arm's
`ingest` returns `None`, and the top-of-loop present is skipped once when it is
set. Nothing else changes.

**Checked that it skips before trusting it.** With the form up, the tray's
`ToolTip` before and after a 2 s `shape` flood of 157 740 refusals was
`goad — waiting for an answer` both times. On the head build the same flood
leaves `goad — no action taken: an event was refused (invalid_envelope):
missing required key `kind``. The refusal is folded but not presented in the
control, exactly as intended. The worktree has been removed.

### Raw table

`derive.py` over `results.jsonl`. "% core" is CPU-s over the sampled window.
Idle runs (condition a): UI thread **0.00 CPU-s** in every idle window,
compositor 2.2–5.6 % of a core; not repeated below.

| run | refusals | refusals/s | UI thread CPU-s | UI thread % core | µs UI CPU / refusal | host all threads % core | compositor % core | writers CPU-s |
|---|---|---|---|---|---|---|---|---|
| form-head/shape/1 | 29937 | 2994 | 9.86 | 99.6 | 332.7 | 102.8 | 3.2 | 0.22 |
| form-head/shape/2 | 29176 | 2918 | 9.82 | 99.2 | 339.9 | 102.8 | 4.2 | 0.30 |
| form-head/shape/3 | 30703 | 3070 | 9.86 | 99.6 | 324.4 | 102.7 | 3.2 | 0.23 |
| form-head/too_soon/1 | 20355 | 2036 | 8.67 | 87.6 | 430.2 | 118.1 | 8.6 | 0.64 |
| form-head/too_soon/2 | 22511 | 2251 | 8.73 | 88.2 | 391.7 | 118.1 | 7.3 | 0.57 |
| form-head/too_soon/3 | 27627 | 2763 | 9.46 | 95.5 | 345.8 | 121.4 | 3.8 | 0.33 |
| form-ctl/shape/1 | 1036658 | 103666 | 3.71 | 37.5 | 3.6 | 139.9 | 2.4 | 6.91 |
| form-ctl/shape/2 | 491863 | 49186 | 1.97 | 19.9 | 4.0 | 75.3 | 7.0 | 4.32 |
| form-ctl/shape/3 | 676685 | 67668 | 2.70 | 27.3 | 4.0 | 115.7 | 3.5 | 5.53 |
| form-ctl/too_soon/1 | 660312 | 66031 | 2.69 | 27.2 | 4.1 | 107.1 | 4.7 | 5.15 |
| form-ctl/too_soon/2 | 886507 | 88651 | 3.32 | 33.5 | 3.8 | 127.8 | 3.9 | 5.98 |
| form-ctl/too_soon/3 | 589277 | 58928 | 2.45 | 24.7 | 4.2 | 99.2 | 5.4 | 4.93 |
| hidden-head/shape/1 | 1094590 | 109459 | 4.54 | 45.9 | 4.2 | 147.4 | 2.3 | 6.82 |
| hidden-head/shape/2 | 922253 | 92225 | 3.98 | 40.2 | 4.4 | 130.9 | 2.9 | 6.32 |
| hidden-head/shape/3 | 1048401 | 104840 | 4.49 | 45.3 | 4.3 | 145.5 | 2.2 | 6.83 |
| hidden-head/too_soon/1 | 778491 | 77849 | 3.73 | 37.7 | 4.8 | 153.1 | 4.4 | 6.18 |
| hidden-head/too_soon/2 | 524595 | 52460 | 2.79 | 28.2 | 5.4 | 130.3 | 5.6 | 4.91 |
| hidden-head/too_soon/3 | 713103 | 71310 | 3.40 | 34.3 | 4.8 | 140.3 | 3.3 | 5.67 |
| hidden-ctl/shape/1 | 813905 | 81390 | 3.11 | 31.4 | 3.9 | 135.5 | 2.5 | 6.66 |
| hidden-ctl/shape/2 | 616094 | 61609 | 2.52 | 25.5 | 4.1 | 102.1 | 5.3 | 5.43 |
| hidden-ctl/shape/3 | 691089 | 69109 | 2.75 | 27.8 | 4.0 | 113.5 | 3.8 | 6.03 |
| hidden-ctl/too_soon/1 | 971448 | 97145 | 3.62 | 36.6 | 3.8 | 139.8 | 2.3 | 6.66 |
| hidden-ctl/too_soon/2 | 949658 | 94966 | 3.52 | 35.6 | 3.7 | 134.6 | 2.5 | 6.45 |
| hidden-ctl/too_soon/3 | 980898 | 98090 | 3.61 | 36.5 | 3.7 | 138.8 | 2.3 | 6.63 |
| diag-head/shape/1 | 182238 | 18224 | 9.85 | 99.5 | 54.6 | 117.3 | 5.2 | 1.17 |
| diag-head/shape/2 | 182305 | 18230 | 9.86 | 99.6 | 54.6 | 117.1 | 5.0 | 1.18 |
| diag-head/shape/3 | 182742 | 18274 | 9.86 | 99.6 | 54.5 | 117.3 | 5.3 | 1.18 |
| diag-head/too_soon/1 | 162721 | 16272 | 9.69 | 97.9 | 60.1 | 138.4 | 5.7 | 1.32 |
| diag-head/too_soon/2 | 161686 | 16169 | 9.66 | 97.6 | 60.3 | 138.5 | 5.6 | 1.28 |
| diag-head/too_soon/3 | 157688 | 15769 | 9.36 | 94.5 | 60.0 | 135.1 | 5.4 | 1.23 |

The `too_soon` rows include about four accepted exchanges per window (null
view, so the form stays); their own presents and backend spawns are in these
figures, which is why `too_soon` reads slightly dearer than `shape`. The
control's rates vary run to run because there the writers, not the host, are
the limit (4–7 CPU-s of writer time).

**Responsiveness probe** — latency from the tray's *Diagnostics* click (t = 2 s)
to the window title changing, form up, 10 s `shape` flood:

| build | writers | runs | click → title change |
|---|---|---|---|
| head | 0 (no flood) | 1 | ≤0.04 s |
| head | 4 | 4 | **8.3 s (after the flood ended)**, 0.5 s, 0.5 s, **5.7 s** |
| head | 1 | 1 | 1.3 s |
| control | 4 | 1 | ≤0.02 s |

### Derived figures

- **UI-thread CPU per refused arrival, head build**, means of three runs:
  - form up: **332 µs** (`shape`), 389 µs (`too_soon`);
  - diagnostics pane up: **55 µs** (`shape`), 60 µs (`too_soon`);
  - hidden (no form): **4.3 µs** (`shape`), 5.0 µs (`too_soon`).
- **Control, present skipped**: **3.9 µs** per refusal, form up or hidden
  alike.
- **Share that is the present**, form up: (332 − 3.9) / 332 ≈ **99 %**. With
  the pane up: (55 − 3.9) / 55 ≈ 93 %. Hidden: within noise of zero — a hidden
  present with no view does almost nothing.
- **How much of a core a flooding local writer takes**, form up: the UI
  thread's **whole core** (99.2–99.6 % in every `shape` run) at ~3 000
  refusals/s; the host's threads together ~103 %. It is saturated, so the rate
  is the present's reciprocal, not the writer's. One writer alone also
  saturates it (probe row: 9.86 CPU-s in 9.9 s). Pane up: again the whole core,
  at ~18 000/s. Hidden: 28–46 % of a core at 50 000–110 000/s, writer-bound.
- **Without the present** (control, form up): 20–38 % of a core at
  50 000–100 000 refusals/s, and there the writers are the limit. At the head
  build's saturated rate of ~3 000/s the control would spend ~1.2 % of a core.
- **Compositor**: form-up floods ran 3.2–8.6 % of a core against a 2.2–5.6 %
  idle baseline; pane-up floods 5.0–5.7 %. At most a few points, inside the
  desktop's own noise. The flood does not make the compositor work hard — a
  saturated UI thread commits few frames.

### Conclusion

**The present is the cost.** With a form up, it is about 99 % of the UI
thread's work per refused arrival — ~330 µs against ~4 µs for everything else
(socket, `ingest`, reply, fold). No repair of FU-2 can remove the ~4 µs; a
repair that bounds presents removes nearly all the rest.

**And it matters, which the headless tier could not show.** On the real
platform, a local writer flooding refusals — one writer is enough — pins the UI
thread at 100 % of a core for as long as it writes, and the host's window
stops acting on input promptly: a tray activation took 0.5 s to 8+ s to reach
the window across four runs, once not until the flood stopped. With the present
skipped (the control), the same flood leaves the UI thread two-thirds idle and
the activation lands at once. The compositor is barely affected. Whether typing
into a field lags is unobserved (AC-6).

The cost depends on the surface: dearest with the form up, ~6× cheaper with the
diagnostics pane up, negligible with no view. The case to repair for is the
form being up — which is exactly when a person is in the window.

## Cross-thread findings

- **R-12's row records 1690/s as the measured rate; on the real platform the
  rate is ~3 000/s and it is the UI thread's ceiling, not the writer's.** The
  headless number was the loop's throughput under the test tier's backend,
  whose present is not this one's (its build profile is unrecorded). R-12's row is superseded anyway (AC-2), but no
  amended row should carry either number forward as though it described the
  running host.
- **The latency probe shows why "the last refusal reaches the window within a
  bound" (AC-3) is not only a diagnostics concern.** The unbounded presents
  delay everything else the event loop owes the person. Where the delay sits
  was not traced: `serve`'s `biased` `select!` polls `commands` before
  `ingress`, so once a command is in the channel it wins — the delay is
  upstream of that, in the event loop getting round to the tray activation.
  That is consistent with `serve` rarely yielding while an arrival is always
  ready, but it is an inference, not a measurement.

## Design-input deltas

- OQ-1 answered by measurement: **worth repairing**. The cost is not marginal
  — it is the whole UI thread under a flood from one local writer, with a
  person-visible symptom.
- The coalescing leaning is supported by the control: skipping the present
  took the UI thread from saturated to ~4 µs per refusal. A bound of one present
  per interval at 3 s would put the flood's present cost at ~0.01 % of a core.
- AC-6's human run should include typing into a field during a flood with the
  form up — the one symptom this research could not reach.

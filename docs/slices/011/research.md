# Research — Slice 011

**Producers:** research agent, for all three threads. Threads 1 and 2 come
from reading the tree and canon. Thread 3 measured the running host (release
build, real compositor) against an attribution control, with runs interleaved
and screened for contention.
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
which does not outlive the session: `measure.py`, `interleave.py`,
`probe.sh`, `derive2.py`, `backend.sh`, `scratch-head.toml`,
`scratch-ctl.toml`, `control.patch`, and the raw `results2.jsonl` and
`probes2.jsonl`. Their content that matters is restated here.

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

**Every figure below comes from the second measurement session.** The runs
were interleaved and screened for contention, as set out under *Contention*.
A first session measured the same conditions without either safeguard. Its
`shape` figures agreed with these to within 5 % on CPU per refusal. Its
`too_soon` form-up figure read 15 % higher (389 µs against 339 µs). It is
superseded, and it is not load-bearing anywhere in this file.

### Instruments

1. **UI-thread CPU per refusal: the lead figure.** It is the change in
   `utime + stime` from `/proc/<pid>/task/<tid>/stat`, read at the start and
   end of a ~9.9 s window (`measure.py`), divided by the refusals answered in
   that window. The UI thread is the thread whose tid equals the pid. That is
   the Slint event loop, and `serve` runs on it. The counter ticks at 100 Hz,
   so one reading resolves to 10 ms, which is ≤0.2 % of a window. The same
   figure is also computed over **all host threads** summed.
2. **Refusal count**: every reply the writers read, by reason, over their 10 s
   run, scaled to the sampled window (`derive2.py`). This is the denominator.
3. **Achieved refusals/s and % of a core**: reported, but **load-sensitive**
   (see *Contention*).
4. **Compositor CPU**: whole process, same window. **The compositor is
   `umbriel`, not niri.** niri is not running on this machine; `umbriel` owns
   `wayland-0`.
5. **Responsiveness probe** (`probe.sh`). Two seconds into a flood, the
   driver activates the tray's *Diagnostics* item over D-Bus
   (`com.canonical.dbusmenu.Event`, id 3, `clicked`). It then polls
   `umbriel windows` every 0.25 s until the scratch host's window title
   becomes `goad — diagnostics`. The figure is the time from the click to
   that title.

**What they hold.** (1) is how much CPU the UI thread spent per refusal,
whatever it spent it on. Set against the control, it shows how much of that
the present costs. (5) runs end to end, from a tray activation to the window's
new state as the compositor reports it. It detects an event loop that is not
getting round to input-side work.

**What they do not reach.**

- (1) does not break a present down. It cannot say whether `option_models`,
  the Slint binding re-evaluation that the epoch bump triggers, or `show()`
  dominates.
- (1) cannot see rendering done on other threads. Here the renderer runs on
  the UI thread, and no other host thread was busy beyond the tokio workers,
  so this gap is small.
- (4) counts the compositor's **whole process**, which includes every other
  client on a busy desktop. With no flood it ran 2.2–2.9 % of a core in
  the kept windows, and up to 5.6 % in the first session. A difference of a
  point or two is noise.
- (5) is not typing. A tray activation reaches the loop by a different route
  from a key press, and nothing here types into a field. Whether keystrokes
  lag under a flood is **unobserved** and belongs to AC-6's human run.

### Contention

Other agents on this machine, working in other repos, run cargo builds
intermittently. The machine has 32 cores, and load averages during the
session ranged from 1.9 to 14.7.

- **Screening.** `measure.py` polls `/proc` every 0.2 s through each window
  for any process whose comm is a build tool: `rustc`, `cargo`, `cc`,
  `gcc`, `clang`, `ld`, `mold`, `rust-lld`, `sccache`, or a
  `build-script-*`. This session ran no build while measuring; both
  binaries were built beforehand. So any such process is foreign. A window
  that saw one is **discarded and repeated**, and it stays in the raw table
  marked as discarded. Before each window the driver waits until no build
  process has been seen for 2 s (5 s for the probes after the first three).
  `/proc/loadavg` is recorded at each window's start and end.
- **Discarded.** 9 of the 57 CPU windows, and 6 of the 12 probe runs.
- **Interleaving.** Both builds ran at once as two scratch hosts, each on its
  own socket (`scratch-head.toml`, `scratch-ctl.toml`). The driver
  (`interleave.py`) alternated them every run: control then head, then head
  then control, and so on. The probes alternated control, head, head, control,
  control, head, with a fresh host for each. A build landing mid-session
  therefore falls on both arms, not on one.
- **What the screen reaches.** It catches build tools by process name. It
  does not catch other heavy work, such as test binaries, a browser or a
  model process (one was running at about 50 % of a core), and it misses a
  build that starts and ends between two 0.2 s polls. The load average is
  recorded so that such load is at least visible.
- **What contention does to each figure.** CPU-seconds per refusal (lead
  figure) is robust: a preempted thread accrues no CPU time. The discarded
  windows show this. Their per-refusal CPU stays close to the kept windows'.
  Form head `shape` read 325.5 µs discarded against 315.8–320.8 kept, which
  is +2.4 % on the mean. The control read 3.6–4.1 µs against 3.5–3.6, up to
  +14 % on a small figure. So contention nudges the figure up a little and
  cannot account for a hundredfold difference. Nothing here separates the
  cause of that nudge (cache, frequency). **Achieved rates, %
  of a core and probe latency are load-sensitive.** They depend on wall
  time and on scheduling. The probe was the most sensitive of all. Head
  probes that overlapped a build answered in 1.05 s and 0.53 s. Clean ones
  answered in 6.8–8.8 s. That the first session's 0.5 s head probes were
  contended too is a plausible inference; nothing recorded it.

### Setup

- **Builds**: release, `cargo build --release -p goad` at `fa266da`,
  installed in the scratchpad as `goad-head`. The control, `goad-ctl`, is the
  same with `control.patch` applied. Both were built before any measuring,
  and the control's worktree was removed after its build. The daily `goad`
  systemd service kept running untouched. It has its own config, socket and
  backend, and its window is told apart by title.
- **Config**: `default_poll = "1h"`; each host has its own ingress socket in
  the scratchpad. The backend is `backend.sh`:
  - for the host's own evaluations, the demo backend's form, with a field of
    every kind and `next_check` 45 minutes out;
  - for a watcher's accepted envelope, a null view, so the form stays
    outstanding;
  - with `SCRATCH_HIDDEN=1`, a null view for everything, so there is no form
    and the window is hidden.
- **Writer**: four Python processes. Each loops: connect, send one line, read
  the one-line reply, close. There are two envelopes:
  - `shape`, which is refused `invalid_envelope` with the same text every
    time;
  - `too_soon`, a valid envelope. One of these is accepted per 3 s spacing
    and runs a null-view exchange. The rest are refused `too_soon`, or
    `engaged` while that exchange is in flight.

  Against the head build with the form up, the writers used under 1 CPU-s of
  the 40 available per window, so they are not the bottleneck there.
- **Conditions**, each run as three 10 s windows per build per envelope:
  - (a) idle;
  - (b) window hidden (no form);
  - (c) form up;
  - (d) diagnostics pane up, reached by activating *Diagnostics* over D-Bus
    on both hosts.

### Attribution control

`control.patch` makes one change to `serve`. It adds `skip_present`, which is
set when the ingested arm's `ingest` returns `None`, and while it is set the
top-of-loop present is skipped once. Nothing else changes.

**Checked that it skips before trusting it.** With the form up, the control
host's tray `ToolTip` read `goad — waiting for an answer` both before and
after a 2 s `shape` flood of 157 740 refusals. After the same flood, the head
build's tooltip names the `invalid_envelope` refusal. So the control folds a
refusal but does not present it.

### Derived figures — CPU per refusal (lead)

Kept windows only, three per row. "all-thread" adds the tokio workers, which
do the accept, the read and the reply.

| condition | n | µs UI-thread CPU / refusal, mean (min–max) | µs all-thread CPU / refusal, mean | refusals/s mean (load-sensitive) |
|---|---|---|---|---|
| form up, head, `shape` | 3 | **317.8** (315.8–320.8) | 327.6 | 3136 |
| form up, control, `shape` | 3 | **3.6** (3.5–3.6) | 13.0 | 107143 |
| form up, head, `too_soon` | 3 | 339.1 (333.5–344.0) | 420.4 | 2818 |
| form up, control, `too_soon` | 3 | 3.7 (3.6–3.7) | 13.9 | 100298 |
| pane up, head, `shape` | 3 | **56.6** (56.5–56.6) | 66.2 | 17620 |
| pane up, control, `shape` | 3 | 3.5 (3.5–3.6) | 12.8 | 109532 |
| pane up, head, `too_soon` | 3 | 62.4 (61.4–63.6) | 88.9 | 14840 |
| pane up, control, `too_soon` | 3 | 3.6 (3.6–3.6) | 13.6 | 103016 |
| hidden, head, `shape` | 3 | **4.3** (4.2–4.5) | 14.1 | 104264 |
| hidden, control, `shape` | 3 | 3.5 (3.5–3.5) | 12.7 | 110249 |
| hidden, head, `too_soon` | 3 | 4.6 (4.5–4.6) | 17.9 | 92931 |
| hidden, control, `too_soon` | 3 | 3.7 (3.6–3.8) | 14.0 | 100214 |

Idle windows (condition a) recorded **0.00 UI-thread CPU-s** in every window,
on both builds.

- **The present's share of a refusal's UI-thread CPU:**
  - form up: (317.8 − 3.6) / 317.8 ≈ **99 %**, about 314 µs of present per
    refusal;
  - pane up: (56.6 − 3.5) / 56.6 ≈ 94 %;
  - hidden: 0.8 µs, 0.3–1 µs across the two envelopes. A hidden present with
    no view does almost nothing.
- **What no repair of FU-2 removes:** about 3.6 µs of UI-thread CPU per
  refusal, and about 13 µs across all threads. That covers the socket,
  `ingest`, the reply and the fold.
- The `too_soon` rows are slightly dearer than `shape` because they include
  about four accepted exchanges per window, each with its own presents and
  backend spawn.
- **How much of a core a flood takes (load-sensitive):**
  - With the form up, the head build's UI thread used 9.05–9.88 CPU-s in each
    ~9.9 s window, which is **saturated**. The ceiling it reaches, about 3 100
    refusals/s, is the reciprocal of the present's cost, not a property of
    the writer.
  - With the pane up, it is saturated again, at about 17 600/s.
  - The control never saturates (3.5–3.9 CPU-s per window). There the writers
    set the rate, about 100 000/s.
  - At the head build's ceiling of about 3 100/s, the control's cost would be
    about 1.1 % of a core.

### Compositor (load-sensitive)

These are `umbriel`'s whole-process CPU ranges over the kept windows. With no
flood, it used 2.2–2.9 % of a core.

| surface | head flood | control flood |
|---|---|---|
| form up | 2.8–4.2 % | 2.3–3.0 % |
| pane up | 5.0–5.8 % | 2.3–2.8 % |
| hidden | 2.2–3.3 % | 2.1–2.6 % |

The largest effect is with the pane up, and it is about 3 points of a core.
The flood costs the compositor little.

### Responsiveness probe (load-sensitive)

Form up, four writers, `shape` flood, click at t = 2 s. The flood ends at
t = 10 s, which is 8 s after the click.

| build | clean runs: click → title change | runs discarded (build seen) |
|---|---|---|
| head | **8.8 s, 6.8 s, 8.3 s** | 2 (they answered in 1.05 s and 0.53 s) |
| control | 0.014 s, 0.014 s, 0.014 s | 4 (all answered in 0.013–0.019 s) |

For reference, the first session measured the head build with no flood at
≤0.04 s.

On a quiet machine, a flood holds a tray activation off for 7–9 s: most or
all of the flood's remaining 8 s. With the present skipped, the activation
lands inside the first 0.25 s poll.

### Raw table

The output of `derive2.py` over `results2.jsonl`, with every window
included; discarded windows are marked. It repeats the per-refusal CPU for
each window and the load average at the window's start and end.

| run | refusals | UI thread CPU-s | µs UI CPU / refusal | µs all-thread CPU / refusal | refusals/s (load-sensitive) | loadavg start→end |
|---|---|---|---|---|---|---|
| form-ctl/idle/1 | 0 | 0.00 | — | — | 0 | 7.79→7.23 |
| form-head/idle/1 (discarded: build seen) | 0 | 0.00 | — | — | 0 | 6.81→6.15 |
| form-head/idle/1 | 0 | 0.00 | — | — | 0 | 10.16→10.08 |
| form-head/idle/2 | 0 | 0.00 | — | — | 0 | 9.43→8.45 |
| form-ctl/idle/2 | 0 | 0.00 | — | — | 0 | 7.85→6.88 |
| form-ctl/idle/3 | 0 | 0.00 | — | — | 0 | 6.20→5.48 |
| form-head/idle/3 | 0 | 0.00 | — | — | 0 | 5.12→4.64 |
| form-ctl/shape/1 (discarded: build seen) | 1065083 | 3.75 | 3.6 | 13.0 | 106508 | 4.64→4.56 |
| form-ctl/shape/1 (discarded: build seen) | 722809 | 2.96 | 4.1 | 16.9 | 72281 | 8.72→8.61 |
| form-ctl/shape/1 (discarded: build seen) | 803796 | 3.16 | 4.0 | 16.8 | 80380 | 7.66→7.39 |
| form-ctl/shape/1 (discarded: build seen) | 1073492 | 3.80 | 3.6 | 13.0 | 107349 | 7.39→6.88 |
| form-ctl/shape/1 (discarded: build seen) | 843002 | 3.24 | 3.9 | 14.2 | 84300 | 14.65→13.33 |
| form-ctl/shape/1 | 1034514 | 3.70 | 3.6 | 13.5 | 103451 | 7.94→7.41 |
| form-head/shape/1 | 31491 | 9.88 | 316.9 | 326.2 | 3149 | 6.90→6.15 |
| form-head/shape/2 | 31501 | 9.85 | 315.8 | 325.8 | 3150 | 6.15→5.58 |
| form-ctl/shape/2 | 1091435 | 3.81 | 3.5 | 12.8 | 109144 | 5.21→4.94 |
| form-ctl/shape/3 | 1088338 | 3.80 | 3.5 | 12.9 | 108834 | 4.94→4.64 |
| form-head/shape/3 (discarded: build seen) | 30537 | 9.84 | 325.5 | 337.0 | 3054 | 4.35→4.72 |
| form-head/shape/3 | 31075 | 9.87 | 320.8 | 330.9 | 3108 | 4.43→4.20 |
| form-ctl/too_soon/1 | 1025550 | 3.69 | 3.6 | 13.6 | 102555 | 4.20→4.17 |
| form-head/too_soon/1 | 28684 | 9.65 | 339.8 | 421.5 | 2868 | 4.00→4.21 |
| form-head/too_soon/2 | 27411 | 9.05 | 333.5 | 413.8 | 2741 | 4.21→4.02 |
| form-ctl/too_soon/2 | 1018599 | 3.68 | 3.6 | 13.7 | 101860 | 3.94→3.79 |
| form-ctl/too_soon/3 | 964788 | 3.56 | 3.7 | 14.4 | 96479 | 3.65→3.70 |
| form-head/too_soon/3 | 28447 | 9.69 | 344.0 | 426.1 | 2845 | 3.70→4.18 |
| diag-ctl/shape/1 (discarded: build seen) | 1080992 | 3.80 | 3.6 | 12.9 | 108099 | 3.69→3.58 |
| diag-ctl/shape/1 | 1093062 | 3.84 | 3.5 | 12.9 | 109306 | 3.58→3.57 |
| diag-head/shape/1 | 176092 | 9.87 | 56.6 | 66.3 | 17609 | 3.69→3.58 |
| diag-head/shape/2 | 175975 | 9.86 | 56.6 | 66.2 | 17598 | 3.37→3.24 |
| diag-ctl/shape/2 | 1095755 | 3.84 | 3.5 | 12.8 | 109576 | 3.24→3.13 |
| diag-ctl/shape/3 | 1097141 | 3.86 | 3.6 | 12.8 | 109714 | 2.96→3.48 |
| diag-head/shape/3 | 176537 | 9.87 | 56.5 | 66.0 | 17654 | 3.48→3.47 |
| diag-ctl/too_soon/1 | 1028546 | 3.67 | 3.6 | 13.6 | 102855 | 3.28→3.47 |
| diag-head/too_soon/1 | 157808 | 9.72 | 62.2 | 87.4 | 15781 | 3.27→3.63 |
| diag-head/too_soon/2 | 142851 | 9.00 | 63.6 | 91.1 | 14285 | 3.63→3.61 |
| diag-ctl/too_soon/2 | 1030546 | 3.70 | 3.6 | 13.6 | 103055 | 3.56→3.64 |
| diag-ctl/too_soon/3 | 1031374 | 3.67 | 3.6 | 13.6 | 103137 | 3.64→3.69 |
| diag-head/too_soon/3 (discarded: build seen) | 156591 | 9.65 | 62.2 | 88.2 | 15659 | 3.55→3.94 |
| diag-head/too_soon/3 | 144553 | 8.79 | 61.4 | 88.2 | 14455 | 3.87→3.96 |
| hidden-ctl/idle/1 | 0 | 0.00 | — | — | 0 | 3.34→2.98 |
| hidden-head/idle/1 | 0 | 0.00 | — | — | 0 | 2.82→2.61 |
| hidden-head/idle/2 | 0 | 0.00 | — | — | 0 | 2.61→2.52 |
| hidden-ctl/idle/2 | 0 | 0.00 | — | — | 0 | 2.48→2.25 |
| hidden-ctl/idle/3 | 0 | 0.00 | — | — | 0 | 2.15→1.97 |
| hidden-head/idle/3 | 0 | 0.00 | — | — | 0 | 1.97→1.91 |
| hidden-ctl/shape/1 | 1099013 | 3.78 | 3.5 | 12.7 | 109901 | 2.07→2.14 |
| hidden-head/shape/1 | 933431 | 4.16 | 4.5 | 15.4 | 93343 | 2.28→2.47 |
| hidden-head/shape/2 | 1097125 | 4.60 | 4.2 | 13.5 | 109712 | 2.47→2.63 |
| hidden-ctl/shape/2 | 1100052 | 3.84 | 3.5 | 12.7 | 110005 | 2.50→2.58 |
| hidden-ctl/shape/3 | 1108401 | 3.85 | 3.5 | 12.7 | 110840 | 2.61→2.75 |
| hidden-head/shape/3 | 1097374 | 4.55 | 4.2 | 13.5 | 109737 | 2.75→2.86 |
| hidden-ctl/too_soon/1 | 1038274 | 3.70 | 3.6 | 13.5 | 103827 | 2.71→2.75 |
| hidden-head/too_soon/1 | 930945 | 4.23 | 4.6 | 18.3 | 93094 | 2.69→3.12 |
| hidden-head/too_soon/2 | 905988 | 4.06 | 4.5 | 17.7 | 90599 | 3.12→3.72 |
| hidden-ctl/too_soon/2 | 932831 | 3.48 | 3.8 | 14.9 | 93283 | 3.91→3.85 |
| hidden-ctl/too_soon/3 | 1035311 | 3.68 | 3.6 | 13.5 | 103531 | 3.85→4.30 |
| hidden-head/too_soon/3 | 950998 | 4.30 | 4.6 | 17.8 | 95100 | 4.12→4.09 |

### Conclusion

**The present is the cost.** With a form up, it is about 99 % of a refused
arrival's UI-thread CPU: about 318 µs against 3.6 µs for everything else. That
holds in every clean window and in every discarded one too, so contention does
not explain it. Bounding presents would remove nearly all of it. The last
3.6 µs belong to ingress, and FU-2 cannot touch them.

**It matters, and the headless tier could not have shown it.** With the form
up, a single local writer is enough to saturate the UI thread. On a quiet
machine the window then stops acting on a tray activation for most of the
flood, typically 7–9 s of an 8 s span. With the present skipped, the same
flood leaves the activation at about 14 ms. The compositor is barely affected.
Whether typing into a field lags is unobserved (AC-6).

The cost depends on the surface. It is highest with the form up, about 5.6×
lower with the diagnostics pane up, and negligible with no view. The case to
repair for is the form being up, which is exactly when a person is in the
window.

## Cross-thread findings

- **R-12's row records 1690/s; on the real platform the ceiling is about
  3 100/s, and it belongs to the UI thread, not the writer.** Slice 004's
  number was the loop's throughput under the test tier's backend, whose
  present is a different one (and whose build profile went unrecorded).
  R-12's row is being superseded anyway (AC-2), but no amended row should
  carry either figure forward as a description of the running host. A rate
  is load-sensitive in any case; CPU per refusal is the figure that
  survives.
- **The probe shows why AC-3's bound is not only a diagnostics concern.**
  Unbounded presents delay everything else the event loop owes the person.
  Where the delay sits was not traced. `serve`'s `biased` `select!` polls
  `commands` before `ingress`, so once a command is in the channel it wins.
  The delay must therefore come before that, in the event loop getting round
  to the tray activation. That is consistent with `serve` rarely yielding
  while an arrival is always ready. It is an inference, not a measurement.
  The contended probes answering *faster* fit it too, since a preempted UI
  thread gives the loop gaps. That is also inference.

## Design-input deltas

- OQ-1 is answered by measurement: **worth repairing**. A flood from one local
  writer takes the whole UI thread, with a symptom a person can see.
- The control supports coalescing. Skipping the present takes the UI thread
  from saturated to about 3.6 µs per refusal. With at most one refusal-only
  present per interval, a flood's present cost becomes about 314 µs per
  interval, which is negligible at any interval of a second or more.
- AC-6's human run should include typing into a field during a flood with the
  form up. That is the one symptom this research could not reach.

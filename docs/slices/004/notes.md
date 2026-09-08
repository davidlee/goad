# Notes — Slice 004

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the A-1 probe | done — **A-1 holds** | 2026-09-08 |
| PHASE-02 — the configuration key, and the envelope | pending | |
| PHASE-03 — the socket's lifecycle, and the accepted path | pending | |
| PHASE-08 — the read budgets, and the closed reason set | pending | |
| PHASE-04 — `serve`'s ingress arms, the second anchor, and what a refusal costs | pending | |
| PHASE-05 — the two anchors, and what a person can see | pending | |
| PHASE-06 — binding at startup, and the demo a person runs | pending | |
| PHASE-07 — the sweep, the spec's own table, and the gate | pending | |

Rows are in **execution order** — PHASE-08 is the listener's second half and
runs between 03 and 04 (`plan.md` PL-10). Phase ids are immutable, so a split
appends a number rather than renumbering.

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-08 · PHASE-01 done · `658b124`

### Produced
<!-- What now exists: modules, contracts, docs. -->

- `docs/slices/004/ingress-probe.local.rs` — the A-1 probe, gitignored and kept
  for re-running, on `docs/slices/003/timer-probe.local.rs`'s shape.
- `research.md` **Thread 3** — the A-1 result: topology, four cases, three runs,
  the verbatim output, and an explicit statement of what was *not* measured.

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

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

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

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

### PHASE-03 — The socket's lifecycle, and the accepted path

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

**Fresh as of:** 2026-09-08 · PHASE-03 done

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
  `Ingress`, `Arrival`, `Answer`, `Refusal` (five variants), the accept task:
  reclaim, the owner-only mode, the newline-or-EOF framing, both read budgets,
  the one reply. `SPEC-003/R-1..R-8`'s listener-decided half is discharged;
  `R-9`/`R-10`/`R-13`'s wiring to the wire's `invalid_envelope` reason is
  discharged except `reserved_source`'s own token, which is `PHASE-08/EX-13`.
- `crates/goad-shell/tests/integration/ingress.rs` — the fake judge fixture
  (shared with `PHASE-08`), and PHASE-03/VT-1..VT-6, VT-10..VT-14.

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

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

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

# Research — Slice 004

**Producers:** design agent (single thread, 2026-09-08), reading canon and code
directly.
**As of:** 2026-09-08 · `b6ca5f7`

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- unmarked — researcher claim: cited, not checked.

Design and plan may only load-bear ✓ rows, or rows they verify at point of use.
Verify what you lean on, not everything.

## Citation forms

Canon claims cite the document id (`SPEC-003 §4`, `ADR-007`). Code claims cite
`path:line`. An uncited claim is unverifiable by definition.

## Thread 1 — governing canon

### Binding

| id | what binds this slice | ✓ |
|---|---|---|
| SPEC-001/R-7 | an `evaluate` carries the host's instant **and** an event with source, kind, timestamp, data | ✓ |
| SPEC-001/R-9 | the host carries `event.data` verbatim and MUST NOT interpret it | ✓ |
| SPEC-001/R-56 | every `evaluate` **the host originates** carries `source: "host"` and one of three kinds; the set is open; a backend MAY branch on the three | ✓ |
| SPEC-001/R-22 | an absolute instant without an explicit UTC offset is refused with its own error — the precedent for what the envelope's `timestamp` must be | ✓ |
| SPEC-001/R-45, R-46, R-47 | no backend failure terminates the host or leaves it unable to invoke the backend; no panic on derived values; every refusal is reported | ✓ |
| SPEC-001 §3 P-B | permissive wire, canonical internals; an *ambiguous* message fails rather than being guessed at | ✓ |
| SPEC-002/R-4, R-5, R-6 | the scheduled floor is 3 s, anchored to the previous scheduled firing, cleared by nothing, and bounds firing rather than what is stored. **R-5 obliges this slice to bound its new stimulus separately.** | ✓ |
| SPEC-002/R-9 | at most one exchange in flight | ✓ |
| SPEC-002/R-10 | cancellation takes precedence, and a pending wait leaves behind nothing a drop would fail to cancel | ✓ |
| SPEC-002 §6 | the spacing is a constant of the host, not configurable, not readable by a backend | ✓ |
| ADR-001 | event ingress is named in **stratum 2**; stratum 1 has no I/O, no runtime, no clock | ✓ |
| ADR-003 | stratum 2 is `crates/goad-shell`; the four instruments and what each does not reach | ✓ |
| ADR-004 | the scheduled anchor is written **only** by a scheduled firing; no stimulus clears it. Its Verification section says the anchor's independence is held by review because *no standing test can distinguish it today* — this slice is the case that can. | ✓ |
| POL-001 | the six gate commands; a feature added to a dependency **shared with stratum 1** is a design decision to be argued in the slice that takes it | ✓ |

### Checked, not applicable

- **ADR-002** — superseded by ADR-003. Its T2 trigger (a second binary) is slice
  005's; this slice's client is a shell one-liner. ✓
- **SPEC-001 §4 Process transport (R-36..R-43, R-48, R-54)** — governs the
  *backend* transport. The ingress socket is a different socket in the other
  direction (`slice-004.md` §Non-goals). Its bounds and budgets are precedent to
  imitate, not rules that reach. ✓

### Amendment candidates

- **SPEC-002** — CD-1, the event bound. Requirement ids run to R-11, so the new
  one is **SPEC-002/R-12**. ✓ (`docs/specs/002-host-scheduling-behaviour.md`
  §4). `draft-spec.md` independently numbers its ingress-side requirement R-12,
  so the pair is always cited qualified.
- **SPEC-001/R-56** — CD-2, `"host"` reserved as a source. ✓
- **No canon owns the ingress contract.** SPEC-001 owns the host/backend
  protocol and disclaims the timer; SPEC-002 owns the timer. The socket, the
  envelope, the reply and the refusal set are governed by **nothing**, and
  `docs/AGENTS.md` §*Canon that does not exist yet* says a rule a slice needs
  that was never written down is drafted in the slice folder rather than held in
  an agent's head. ✓ — see *Cross-thread findings* C-1.

## Thread 2 — code map

### Hotspots

| path | why it changes |
|---|---|
| `crates/goad-shell/src/` (new module) | the listener, the envelope wire type, its normalization, the reply |
| `crates/goad-shell/src/config.rs` | one optional section carrying the socket path |
| `crates/goad-shell/src/error.rs` | the ingress error type(s) — a sixth subject |
| `crates/goad/src/controller.rs` | `serve`'s arms, the event anchor |
| `crates/goad/src/wire.rs` | `Stimulus` cannot express an event the host did not author |
| `crates/goad/src/startup.rs`, `main.rs` | bind before the loop; a new `StartupError` variant |
| `crates/goad/src/diagnostics.rs` | an ingress refusal a person can read |
| `Cargo.toml` | `tokio`'s feature list |

### Cited facts

- **F1 ✓ — The loop does not poll its outer `select!` while an exchange is in
  flight.** `controller.rs:409` opens the loop; `:411-425` is the outer
  `select!` (cancel, commands, timer); `:505-513` is the *inner* `select!`
  (cancel, the exchange future). Between engaging and absorbing, only cancel and
  the call are polled. **Consequence:** an ingress arm added to the outer
  `select!` alone can never produce the "the host is engaged" refusal AC-4
  requires — a connection would instead wait in the accept backlog and be
  served late.
- **F2 ✓ — The scheduled anchor has exactly one write site.** `floor_until` is
  initialised at `controller.rs:407` and written only at `:420`, inside the
  timer arm. `MINIMUM_SPACING` is a private const at `:318`, 3 s. A second
  anchor must be as narrow, and neither may write the other (ADR-004).
- **F3 ✓ — `Stimulus` synthesises the whole event; it cannot carry one.**
  `wire.rs:41-70`: three unit variants, `kind()` returning one of three static
  strings, and `event(now)` building an `Event` with `source: "host"` and
  `data: Value::Null`. An ingested evaluation supplies all four fields from
  outside, so this type must gain a variant that *carries* an `Event` rather
  than one that names a fourth kind (which CD-2 also forbids).
- **F4 ✓ — `Event` is stratum 1, public-field, `Serialize`, and its
  `timestamp` is a `Timestamp`.** `canonical.rs:490-497`; `Timestamp` wraps
  `jiff::Timestamp` (`:103`) and serialises via `collect_str` over jiff's
  `Display` (`:118-122`). jiff's `Timestamp` displays as RFC 3339 **in UTC**,
  so an envelope written `+10:00` reaches the backend as the same instant
  spelled `Z`. — instant preserved ✓ (by construction of the type); the
  UTC spelling is a jiff claim design must verify at point of use.
- **F5 ✓ — `jiff::Timestamp` parses an offset form and rejects an offsetless
  one.** `schedule.rs:73-86`: absolute is tried first precisely because an
  offset form parses as both an instant and a civil datetime; the civil parse is
  what produces `MissingOffset`. The same two-step is available to the envelope.
- **F6 ✓ — Config is permissive-in/canonical-out, in stratum 2, with
  `deny_unknown_fields`.** `config.rs`: `File`/`FileBackend`/`FileSchedule` all
  carry it, and `an_unknown_key_is_refused_and_named` asserts the refusal names
  the key — its fixture literally plants `socket = "/tmp/goad.sock"` as the
  unknown key. The backend-authored format normalizes in stratum 1
  (`protocol/normalize.rs`). The two differ by **which contract the
  normalization holds** — `normalize.rs` holds SPEC-001, and holds it in one
  place — not by who wrote the bytes and not by stratum as such. Neither is a
  precedent that decides the envelope on its own; `design.md` D-3 makes that
  call deliberately, as ADR-001 §Consequences says such calls must be made.
- **F7 ✓ — Diagnostics is replaced wholesale, never appended to.**
  `controller.rs` `absorb` assigns `self.diagnostics = diagnostics`; `refuse`
  assigns `Diagnostics::refused(refused)`. A refusal recorded there is the whole
  surface until the next exchange. `Refused` (`diagnostics.rs:52-62`) has three
  variants today, all of them refusals *the renderer* made.
- **F8 ✓ — The startup order fixes where a bind can go.** `main.rs`
  `start()`: config → clock → backend → `Host::new` → build runtime → **`let
  _entered = runtime.enter()`** → window/tray → wire → glass → seed the startup
  evaluation → `slint::spawn_local(serve(..))` → `run_event_loop_until_quit`.
  A `tokio::net::UnixListener::bind` needs the reactor, so it belongs after the
  `enter()` guard and before `spawn_local`. `StartupError` (`startup.rs:28-50`)
  has eight variants and `main` maps any of them to exit code 2.
- **F9 ✓ — `serve` runs on Slint's executor as a `!Send` future.**
  `main.rs` wraps it in `slint::spawn_local`. Anything spawned with
  `tokio::spawn` alongside it must be `Send`; a `UnixListener` and an
  `mpsc::Sender` are.
- **F10 ✓ — at scoping, tokio's feature list had no `net`, and `goad-shell` had
  no `sync`.** Root `Cargo.toml`: `tokio = { features = ["process", "time",
  "rt", "io-util", "macros"] }`; `crates/goad/Cargo.toml` added
  `rt-multi-thread` and `sync`; `crates/goad-shell/Cargo.toml` took the base
  set. **Refreshed at PHASE-07:** PHASE-03/EX-1 added both — `goad-shell`'s
  `tokio` entry now reads `{ workspace = true, features = ["net", "sync"] }`
  (`crates/goad-shell/Cargo.toml:17`), the manifest's only change for the whole
  slice (PHASE-08/VA-5, PHASE-04's Findings F-c: neither reaches `crates/goad`
  except by feature unification, which is why the writer's side of every
  `crates/goad` test case uses `std::os::unix::net` on `spawn_blocking`
  instead).
- **F11 ✓ — tokio is not reachable from stratum 1, so POL-001's
  feature-unification residue does not bite here.** `crates/goad-semantics/
  Cargo.toml` names `jiff`, `serde`, `serde_json` and nothing else, and the
  allowlist's `STRATUM_1` (`allowlist.rs:19`) is the same three. Adding `net` to
  `tokio` cannot unify into a crate that does not depend on tokio.
- **F12 ✓ — The manifest allowlist is names-only.** `allowlist.rs:1-11, 19-27`:
  `tokio` is already permitted for stratum 2, and a *feature* added to a
  permitted dependency is invisible to the instrument. The check that a new
  feature is justified is review, per POL-001 §Verification's residue.
  **Refreshed at PHASE-07:** confirmed against the finished tree — PHASE-07/VA-2
  discharges POL-001's residue clause in writing (`net` and `sync` reach only
  stratum 2's own graph; neither reaches stratum 1's), and separately
  re-confirms the allowlist test passes unchanged with the two features
  present.
- **F13 ✓ — The purity scan binds stratum 1 only.** `purity.rs:17-27` forbids
  `std::fs`, `std::process`, `std::net`, `std::os`, `std::env`, `std::thread`,
  `std::io`, `std::time::SystemTime`, `std::time::Instant`. Setting a socket's
  mode through `std::os::unix::fs::PermissionsExt` is stratum 2's and is not
  reached by it.
- **F14 ✓ — The vocabulary scan's word list is `habit`, `streak`, `journal`,
  `site`, `goal`, `reminder`, `compliance`** (`vocabulary.rs:18-27`),
  case-insensitive, by word, over `.rs` and `.slint` in every member excluding
  `tests/` and `target/`. Nothing in an ingress vocabulary — event, envelope,
  source, kind, listener, ingress, watcher — is on it.
- **F15 ✓ — Nothing prevents two hosts running today.** `main.rs` `start()`
  takes no lock, checks no pidfile and consults no existing process. Two `goad`
  processes against one config already both start, both show a window and both
  invoke the backend. Ingress does not create this; it gives one symptom of it a
  name (`slice-004.md` OQ-6).
- **F16 ✓ — `examples/demo.toml` has `[backend]` and `[schedule]` only**, and
  `just demo` is `cargo run -p goad --bin goad -- examples/demo.toml`
  (`justfile`). `examples/shell/backend.sh` prompts on **every** evaluation that
  is not a `respond`, so an ingested evaluation would produce a window with no
  change to it.
- **F17 ✓ — The renderer test tier calls `serve` directly.**
  `tests/renderer/scheduling.rs` builds a `Host` over a scripted child process,
  a headless window, a stub clock and an `mpsc` channel, and drives production
  `serve` under a `LocalSet`. A new `serve` parameter is a change to every call
  site in that tier — `wiring.rs`, `scheduling.rs`, `table.rs` — which is the
  cost AC-7's "unchanged bodies" has to be read against.
- **F18 — `std::fs::File::try_lock` is stable since Rust 1.89**; the toolchain
  is `rustc 1.99.0-beta.3`. Relevant only if the design closes OQ-6's race.
  Unverified by compilation.

### Precedents

- **Permissive wire type beside a canonical one, normalization the only door**
  — `protocol/wire.rs` + `protocol/normalize.rs` (backend-authored), and
  `config.rs`'s `File` → `Config` (user-authored). Both shapes are available;
  what differs is the contract each door holds, not the stratum it sits in.
- **An error type per *subject*, not per stratum** — `error.rs`'s own doc
  comment argues `ConfigError` into existence as a fifth type because "a config
  file is none of the subjects the others are about". An ingress envelope is a
  sixth subject: written by neither the host nor the backend.
- **Bounded reads, stated budgets** — `SPEC-001/R-41`, `R-43`: every read from
  an untrusted writer is bounded, and a budget is stated rather than hidden.
- **A host operational constant lives beside the loop that applies it, in
  stratum 3** — ADR-004 §Consequences/Neutral, on `MINIMUM_SPACING`.
- **`#[cfg(test)] mod tests` for a stratum-internal pure function** —
  `controller.rs`'s own tail, `state.rs`, `schedule.rs`.

## Thread 3 — spike A-1

### Spike A-1 result — **run 2026-09-08, and it arrives.** Verdict: **A-1 holds.**

**What was run**, at `658b124` — a descendant of the design's `b6ca5f7` that
changes no source (`git diff --stat b6ca5f7 -- crates/ Cargo.toml justfile
flake.nix examples/ tests/` is empty). `docs/slices/004/ingress-probe.local.rs` (gitignored under
`*.local.*`, preserved for re-running), added to `crates/goad/Cargo.toml` as a
temporary `[[test]]` target pointing outside the crate, with `net` added to that
crate's tokio features for the duration (`crates/goad/Cargo.toml:22` has
`rt-multi-thread` and `sync` only). Run three times with
`cargo test -p goad --test ingress_probe -- --nocapture`, and **both** manifest
edits reverted immediately afterwards — the file is byte-identical to its state
at entry (sha256 `7aa0c09a…`), and `just check` exits 0 with it reverted. No
probe code was left in `crates/`.

**The topology.** `start`'s, in `start`'s order (`crates/goad/src/main.rs:47-121`,
`design.md` §5.4): `init_integration_test_with_system_time()` for a real headless
Slint event loop on real time; a multi-thread tokio runtime built with
`enable_all()`; its `EnterGuard` held for the life of the loop; **`bind` under
that guard and before the loop starts**, each `bind` `tokio::spawn`ing its own
accept task; a real `PromptWindow` and `Tray`; one `slint::spawn_local` future
doing all the waiting; `run_event_loop_until_quit` on the main thread outside it;
the process ended by `slint::quit_event_loop` from inside that future.

Each accept task is `design.md` §5.2's listener cut to the question: accept, read
to the first newline or EOF, hand an `Arrival` — the bytes plus a `oneshot`
sender — to the loop over an `mpsc::channel`, **await the judgement before
accepting again** (I-2), then write one JSON line and close. The four `select!`s
reproduce the design's arm ordering with ingress **last**: outer
`cancel → commands → sleep → ingress`, inner `cancel → call → ingress`. The
parked sleep is an hour out and the command channel keeps a live but silent
sender, so no arm above ingress can be ready. The client is what the slice's
client actually is: a blocking `std::os::unix::net::UnixStream` on a thread of
its own, off the runtime being measured.

**Four cases, one process.** Elapsed times are the range over three consecutive
runs.

| case | what it put under the arrangement | measured |
|---|---|---|
| P-A | a `tokio::spawn`ed accept task delivering a connection to the `spawn_local` future's `select!` | arrival observed **via the ingress arm** in **128–207 µs** from the client's `connect`; all 89 envelope bytes intact |
| P-B | the future answering on the `oneshot`; the task writes one line and closes | **connect→reply 105–130 µs**; **30 bytes** read — `{"protocol":1,"accepted":true}` — then the connection closed |
| P-C | the arrival landing while the future awaits a real `Host::evaluate` against a child process, pinned in an inner `select!` | arrival observed **before the exchange completed: true**, at **100.4 ms** into a **305 ms** exchange (connect→arrival 275–337 µs); the exchange still resolved, no failure, `next_check` re-armed |
| P-D | **the negative control** — P-A with nothing else armed and a second of deliberate idle first | arrival observed in **243–300 µs**, after **1.0004 s** parked with cancel untripped, the command channel live and silent, the timer parked 3600 s out and no click |

Total wall time 1.306 s per run, of which 1.0 s is P-D's deliberate idle and
0.305 s is P-C's exchange.

**Run 1 verbatim:**

```
P-A connect->arrival 206.561µs, via the ingress arm, 89 bytes read: {"source":"probe","kind":"connected","timestamp":"2026-09-08T10:00:00+10:00","data":null}
P-B connect->reply 130.401µs (arrival at 108.551µs), 30 bytes read, connection closed: {"protocol":1,"accepted":true}
P-C exchange took 305.104413ms; arrival observed BEFORE it completed = true, at Some(100.357381ms) into the exchange, connect->arrival Some(274.932µs); the exchange still resolved: failure=Some(false) next_check=Some(2026-09-08T10:34:25.171619037Z)
P-D connect->arrival 299.702µs after 1.000390744s parked with NOTHING ELSE ARMED (cancel untripped, command channel live and silent, timer parked 3600s out, no click)
total wall time 1.306623836s
```

**What it settles.** **A-1 holds.** A `tokio::spawn`ed task accepting on a
`tokio::net::UnixListener` runs, and its reactor polls that listener, while the
main thread sits inside Slint's event loop under the runtime guard; its `send`
wakes the `spawn_local` future across threads; and the future's reply travels
back out to the client on the same connection. `UnixListener::bind` succeeds
under the guard, called synchronously where `design.md` §5.4 puts it.

**P-D is the decisive one and the margin is large.** `plan.md` PHASE-01/S-1
treats *anything over 100 ms with nothing else armed* as red. The worst of three
runs is **299.7 µs — 334× under the bound**, after a full second in which the
loop had nothing else to wake for. P-A's number is therefore not an artefact of
some other source waking the loop.

**P-C is the one the design's shape depends on.** `design.md` §5.4 I-2 makes
`engaged` reachable only if an arrival can be observed by the **inner** `select!`
while an exchange is in flight; it was, 100 ms into a 305 ms exchange, and the
exchange completed normally afterwards. The inner arm's shape is buildable as
designed.

**What was measured, and what was not.** Measured: delivery, the reply's bytes
and its close, the inner-arm case, and the negative control — on this machine,
under `cargo test`, on Slint 1.17.1 and tokio 1.53.1. **Not** measured, and not
this phase's: any budget or bound (`ENVELOPE_LIMIT`, `ENVELOPE_DEADLINE`), the
mode, the reclaim, behaviour under a flat-out writer, and anything about
`serve`. The probe's listener is a cut-down stand-in for `design.md` §5.2's, not
that module: it holds the shape A-1 is a claim about — accept, bounded-by-framing
read, one arrival outstanding, one reply, close — and nothing else.

## Thread 4 — plan-stage verification (2026-09-08)

Checked by the plan agent against `b6ca5f7`, because the plan schedules against
them. Everything here is ✓ — a read or grep of the cited site.

| # | fact | ✓ |
|---|---|---|
| F19 | **`serve` has exactly 23 call sites**: `main.rs:103`; `event_loop/closing.rs:82`; `event_loop_schedule/scheduling.rs:110`; `renderer/wiring.rs:915, 992, 1053, 1108, 1155, 1187`; `renderer/scheduling.rs:166, 207, 245, 290, 335, 390, 451, 505, 572, 644, 687, 755, 809, 874`. `renderer/table.rs` has none | ✓ |
| F20 | **`Served` is never destructured.** Its only mentions outside `controller.rs` are two doc comments, `wiring.rs:899` and `:1088`; every use is a field read. Adding a field is safe | ✓ |
| F21 | **`Config` is built by struct literal at four sites** that will need `ingress: None`: `tests/support/driving.rs:46`, `renderer/scheduling.rs:84`, `event_loop/closing.rs:63`, `event_loop_schedule/scheduling.rs:91`. This is a separate bill from F19's | ✓ |
| F22 | **`goad-shell` has neither `net` nor `sync`**, and has **no `[dev-dependencies]` table at all**. `crates/goad`'s own entry adds `rt-multi-thread` and `sync`; the workspace base set is `["process","time","rt","io-util","macros"]` (`Cargo.toml:33-34`). F10 said "no `sync`"; the ingress module needs both | ✓ |
| F23 | **`tempfile` appears nowhere in the workspace.** The only temp-path precedents are `crates/goad-shell/src/config.rs:226` and `tests/support/scripting.rs::marker`, both `std::env::temp_dir()` + `std::process::id()`. Adding the crate would mean adding a name to the manifest allowlist's `STRATUM_2` — an ADR-001 instrument | ✓ |
| F24 | **`Glass` has exactly one implementation**, `SlintGlass` (`glass.rs:62`). `present` spans `:68-119` and writes eleven window properties, rebuilds two `VecModel`s, sets the tray image and tooltip, and calls `show()`/`hide()` — the cost `review-design.md` F-15 names. A presentation counter must therefore be a new decorator, and it must wrap the real glass to measure the real cost | ✓ |
| F25 | **`refusal_re_arms` is at `controller.rs:427`, not `:429`.** `design.md` §5.2 cites 429; 429 is inside the `match fired` below it. Every other `controller.rs` citation in the design checks out: `:407`, `:420`, `:505`, `:507-512` | ✓ |
| F26 | **`answers-as-instructed.sh` carries a `@slow-view` sentinel** — a *foreground* `sleep 0.2` before answering with a pinned view, so the exchange is provably still in flight for its duration. It is the vehicle for every "while an exchange is in flight" case: `engaged`, and R-15's negative. `@lingers*` background their sleep and do **not** hold the exchange open | ✓ |
| F27 | **The vocabulary scan is member-enumerated** (`vocabulary.rs:44-56` reads `workspace.members` and scans each directory), so `crates/goad-shell/src/ingress/` is covered with no hand-listed path — AC-11's "reaches the new module by walking members", confirmed. `tests/` is excluded, so no test file is scanned | ✓ |
| F28 | **`in` properties have generated getters** (`window.get_heading()` is used at `wiring.rs:273` and elsewhere), so `get_diagnostic_lines()` is available to read the diagnostics surface as a person sees it — R-15's positive case | ✓ |

## Cross-thread findings

- **C-1 — the ingress contract has no home in canon.** Thread 1's amendment
  candidates cover the *bound* (SPEC-002) and the *reserved source*
  (SPEC-001). They do not cover the socket, the envelope, the reply format or
  the refusal set, and no existing spec's Owns line reaches them. Slice 005 is
  a second client of that contract and brief §7 anticipates a third. Either this
  slice drafts a spec for it or it ships a wire format governed only by a closed
  slice's design — which is the failure POL-001 exists because of.
- **C-2 — "engaged" is a refusal only if something outside `serve`'s outer
  `select!` can answer promptly (F1).** Three shapes are available: a listener
  task that reads host state through a shared handle; a listener task that hands
  every arrival to the loop and lets the loop answer, with the loop also
  watching ingress *inside* the exchange; or a queue of one, which the
  2026-09-08 decision forbids. This is the load-bearing structural choice of the
  slice.
- **C-3 — "verbatim" (AC-1) cannot mean byte-identical for `timestamp`.**
  F4: the field is modelled as a `Timestamp`, and the host re-serialises it. The
  instant is preserved; the spelling is normalised. Either the design says so
  explicitly, or `Event.timestamp` stops being a `Timestamp` — which would
  weaken the host-originated path for the sake of the ingested one.
- **C-4 — the anchor independence AC-6 demands is exactly what ADR-004 says no
  test could reach.** ADR-004 §Verification: "no standing test can distinguish
  the anchor from the boolean alternative, because the two agree on every
  stimulus that exists today; the case that separates them is the one slice 004
  will introduce." AC-6 is the discharge of that debt, and it must be
  falsifiable in **both** directions.
- **C-5 — AC-3 settles OQ-3.** "Every envelope receives exactly one reply on the
  same connection, and the host then closes it" is one envelope per connection.
  A stream would be a second envelope on a closed connection. OQ-3 is answered
  by the accepted slice document, not by design.
- **C-6 — AC-7's "unchanged bodies" is about the *existing suite's assertions*,
  not about call sites (F17).** A new `serve` parameter touches every caller in
  the renderer tier. The design should say which reading it takes before the
  plan is written against the other.

## Design-input deltas

1. **C-1 raises a question scoping did not ask:** whether slice 004 drafts
   `draft-spec.md` for the ingress contract. It changes what OQ-2 and OQ-5 are
   answers *in*.
2. **C-2 makes the listener/loop split the first design section**, ahead of the
   envelope, the reply or the bound: the refusal set AC-4 enumerates is
   partitioned by which side can answer.
3. **F6 does not answer OQ-4 by itself.** Neither precedent covers a format
   written by a third party, so the placement is a deliberate call rather than
   an inference: `design.md` D-3 keeps normalization in stratum 2 so that
   stratum 1's normalization holds one contract, SPEC-001, in one place. The
   "pure things live in stratum 1" reading is rejected on the same ground — it
   would put a format no backend ever sees inside the protocol crate.
4. **F1 and F17 together** mean the plan's phases are: stratum 2 listener
   (testable alone), then `serve`'s arms and the anchor, then startup and the
   demo. Not the other order.

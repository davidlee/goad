# Plan — Slice 004: Event ingress

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

Seven phases open one door: a Unix domain socket that turns an opaque event
envelope into an `evaluate` on the path slice 003 built. They run **01, 02, 03,
04, 05, 06, 07** — no reordering and no parallelism past PHASE-02, because 04
and 05 both edit `crates/goad/src/controller.rs` and 03, 04, 06 and 07 all edit
`docs/slices/004/notes.md`.

The spine is the design's own three-part model (§5.1): **a listener that
determines shape, a loop that judges state, one reply site.** The phases follow
the direction of that dependency, and the first one measures the assumption the
whole thing stands on.

- **PHASE-01 is the A-1 probe**, and nothing else. Accepting from a
  `tokio::spawn`ed task while Slint owns the main thread is unmeasured
  (`design.md` §5.5 A-1, §8 R1). It is *weaker* than the case slice 003
  measured — that one was a future on Slint's own executor, this one is a task
  on the multi-thread runtime whose sends have to wake Slint's executor from
  another thread. It carries no other deliverable because its exit may be a
  STOP that ends the slice's plan, and a phase that may end that way must not
  carry work that would be thrown away with it.
- **PHASE-02** is everything stratum 2 that needs no socket: the `[ingress]`
  configuration key, and the permissive `Envelope` with its normalization into
  `Event`. Pure unit tests, no runtime, no filesystem. It is where the whole of
  `draft-spec.md` R-9, R-10 and R-13 is discharged, one case per clause.
- **PHASE-03** is the listener over a real socket: `bind`'s probe / reclaim /
  bind / mode sequence, the accept task, the budgets, the framing, the reply's
  bytes, and the closed reason set — against a **fake judge**, so the whole
  contract is held before `serve` has ever seen an `Arrival`.
- **PHASE-04** is the slice's mechanism: `serve`'s two ingress arms, the second
  anchor, the inner `select!` becoming a loop, `Refused::Ingress`, and the 23
  call sites the new parameter bills. It is **the phase `review-design.md` F-15
  names**, and AC-5's flat-out writer is extended there to record presentations
  as well as invocations.
- **PHASE-05** attacks the same mechanism from the discriminating side: AC-6's
  three anchor cases — of which (ii) is the one ADR-004 has been waiting for and
  CD-3 amends the record to name — R-15's bound held from **both** directions,
  and AC-12's malformed flood.
- **PHASE-06** is the host actually listening and a person actually using it:
  the bind at startup, the exit code when it cannot, `socat` in the devshell,
  `examples/demo.toml`'s socket, the documented one-liner, and the human run.
  **It is the one phase a green gate cannot discharge.**
- **PHASE-07** is the sweep: `draft-spec.md` §7's verification table completed
  with real test names, the margin table, the vocabulary scan over the finished
  tree, and the gate from a clean clone.

Five things hold for every phase.

1. **The gate is POL-001's six commands.** A phase is not green until
   `just check` exits 0. Every phase's VA-1 is the gate's pasted output, not a
   claim that it would pass. `just -n check` prints the sequence.
2. **`draft-spec.md` and `canon-delta.md` are the slice's working authority**
   (`docs/AGENTS.md` §*Canon that does not exist yet*). Phases cite them exactly
   as they would canon. **No phase writes into `docs/specs/`, `docs/policy/` or
   `docs/adr/`, and no phase edits `CLAUDE.md`.** CD-1, CD-2, CD-3, the new ADR
   and `draft-spec.md`'s promotion all land at audit, with user endorsement.
   PHASE-07 edits `draft-spec.md` §7 **only** — the table that says in its own
   words that slice 004's phases complete it.
3. **Code cites requirement ids, not finding ids**
   (`docs/memory/cite-requirements-not-finding-ids.md`). A comment that needs to
   say why cites `SPEC-001/R-9`, `SPEC-002/R-12` or `SPEC-003/R-14` — never
   `F-15`, `D-13` or `CD-1`. `SPEC-003` is `draft-spec.md`'s number at
   promotion and is the form the code uses; the two `R-12`s are **always**
   written qualified.
4. **Absence is never asserted alone.** Every anti-fire and anti-spin window in
   this slice is paired with a liveness assertion over the same mechanism
   (`docs/memory/a-bound-is-not-tested-at-the-bound.md`). A refusal test that
   would pass against a listener that accepts nothing at all is not evidence.
5. **Two ways this slice can quietly break the host, and both are lints.**
   `Ingress`, `Arrival`, `Answer` and `Refusal` must be `Send` — `serve`'s
   future is `!Send` only through its type parameters today, and
   `clippy::future_not_send` is `deny` (`Cargo.toml`) — and every new public
   type needs `Debug`, because `missing_debug_implementations` is `deny` and
   `Served`, `Fired` and `Refused` all derive it.

## Sequencing & rationale

**Why the probe is alone and first.** R1 says the slice does not work at all if
A-1 fails, and `design.md` §5.5 says in terms that *"the plan must spike it
before the phase that depends on it."* Everything from PHASE-03 onward depends
on it. Its exit is a measurement and a verdict, and the verdict has two
branches — one of which is *stop, and the design changes*. Bundling the
configuration key with it would mean landing production code inside a phase
whose stated failure mode is that the design was wrong.

**Why the config key and the envelope share a phase.** Neither fills a session.
Both are stratum 2, both are pure, neither needs a runtime or a socket, and both
are prerequisites of the listener: `bind` takes the path the config carries, and
the accept task normalizes with the envelope. They are also the two places where
the "permissive in, canonical past the door" shape already has a precedent in
this crate (`config.rs`'s `File` → `Config`), so one agent writing both writes
them the same way.

**Why the listener is a phase of its own, against a fake judge.** The listener's
whole contract — one envelope per connection, newline-or-EOF, one reply, then
close; the byte and time budgets; the reclaim; the mode; the closed reason set
— is decidable without `serve`. Testing it against a fake judge is what lets
PHASE-04 debug the *loop* rather than the loop and the socket at once, and it is
what makes the eight reason tokens assertable in one cheap unit test rather than
by provoking each one through the renderer tier.

**Why `serve`'s mechanism is one phase and its discriminating tests are
another.** The two arms, the anchor and the inner loop are one function body;
splitting them across phases means editing the same forty lines twice and
leaving a branch nothing drives in between, which is the failure
`docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` records. So
PHASE-04 lands all of it, plus the assertions that drive every branch on the
well-behaved path. PHASE-05 adds the evidence that separates hypotheses: AC-6's
three cases each need a scripted backend whose `next_check` sequence is part of
the argument (case (ii) especially — `slice-004.md` *Readings taken in design*),
and R-15's two cases need an exchange deliberately held open. Those are
fixtures, not mechanism, and they are where the slice's unavoidable wall time is
spent.

**Why startup and the demo are one phase.** The demo cannot run until the binary
binds, and the bind cannot be seen working until the demo runs. They are the
same fact from two sides. It is also the phase that owns the environment change
— `socat` in `flake.nix`, the socket path in `examples/demo.toml`, the socket in
`.gitignore` — because the one-liner has to work from a clean clone in the dev
shell, and the person who checks that is the person running the demo.

**Which phases can be green without the demo, and which cannot.** PHASE-01
through PHASE-05 and PHASE-07 are discharged by `just check` and their own
recorded checks. **PHASE-06 cannot**: VH-1 is a person starting goad with
ingress configured, writing an envelope from the documented one-liner, and
watching the prompt appear. `docs/AGENTS.md` §Tiers is explicit that a green
gate is not that evidence — *"slices 001–003 all closed green on a binary that
could not open a window"* — and AC-13 is the criterion. The observation is
recorded in `notes.md` by PHASE-06 and lifted into `audit.md` under Evidence at
audit.

**Reordering and parallelism.** PHASE-02's two halves are independent of each
other and could be done in either order within the phase. Nothing else moves:
01 → 02 → 03 → 04 → 05 is a dependency chain; 06 needs 02, 03 and 04; 07 needs
everything. **No two phases may run in parallel.**

**Size.** PHASE-01 is the lightest and PHASE-07 next. PHASE-03 and PHASE-04 are
the heavy ones — PHASE-03 for the number of integration cases, PHASE-04 for a
mechanism plus a 23-site migration plus five timed renderer tests. Slice 003's
PHASE-02 (one `select!` arm, a 22-site migration and six timed `serve` tests) is
the calibration, and neither exceeds it. If a phase overruns a session that is a
`PARTIAL` checkpoint and a finding in `notes.md`, not a reason to skip the sheet
(`docs/memory/stop-letter-vs-purpose-is-a-plan-log-adjudication.md`).

## Decisions taken during planning

Recorded in `plan-log.md` with reasoning and rejected alternatives, under the
standing autonomy grant (`design-log.md`, 2026-09-08 *Autonomy grant for 004*:
decide everything except canon; **plan acceptance is reserved to the user**).
Summarised here because each decides where a phase boundary falls or what a
phase may touch.

- **PL-1** — seven phases, in the order and split above.
- **PL-2** — the listener's cases are a new module
  `crates/goad-shell/tests/integration/ingress.rs` in the **existing**
  `integration` target (`design.md` §9 names it), reached by one `mod ingress;`
  line in `integration/main.rs`. No new `[[test]]` target and no new manifest
  entry. Its socket helpers stay in that file: `integration/harness.rs`'s own
  rule is *two or more consumers in this target live here, one consumer stays
  where it is*.
- **PL-3** — temp sockets come from `std::env::temp_dir()` joined with a
  per-case unique directory name, created and removed by the case. **The
  `tempfile` crate is not added.** It would mean adding a name to the manifest
  allowlist's `STRATUM_2` (`goad-boundary/tests/checks/allowlist.rs:19-27`) —
  one of the four ADR-001 instruments — for a test convenience. The precedents
  are `config.rs:226` and `tests/support/scripting.rs::marker`, both of which
  build a temp path from `temp_dir()` and the process id. This also settles
  FD-2.
- **PL-4** — `crates/goad-shell/Cargo.toml`'s tokio entry becomes
  `tokio = { workspace = true, features = ["net", "sync"] }`, mirroring what
  `crates/goad/Cargo.toml` already does with `rt-multi-thread` and `sync`.
  The workspace base set is **not** widened. See FD-1.
- **PL-5** — `crates/goad-shell/src/ingress/` carries a module-level
  `#![deny(clippy::arithmetic_side_effects)]` in both files. D53's rule is that
  the lint follows the data: this module computes over byte counts and durations
  an untrusted writer paces, which is exactly the case R-46 names.
- **PL-6** — the startup decision is a **named function**,
  `startup::listener(configured: Option<&IngressConfig>) -> Result<Ingress,
  StartupError>`, with `main::start` calling it. AC-7's *"no file is created at
  any path"* is otherwise a claim about code nobody can call; as a function it
  is a test. It stays inside the surface `design.md` §5.1's part table gives
  `startup`/`main` — *"binding before the loop starts, and the exit code when it
  cannot"*.
- **PL-7** — the renderer tier's ingress cases get their **own module**,
  `crates/goad/tests/renderer/ingress.rs`, not a further `mod` inside
  `wiring.rs` (1232 lines) or `scheduling.rs`. Same call slice 003 made as its
  own PL-2.
- **PL-8** — the counting `Glass` decorator AC-5 needs lands in
  `renderer/ingress.rs` in PHASE-04. If PHASE-05 needs it too it **moves** to
  `renderer/harness.rs` in PHASE-05, under that file's two-consumers rule. It is
  not put there speculatively.
- **PL-9** — the documented one-liner lives in `examples/demo.toml`'s header
  comment block. **No new `just` recipe and no new binary**: `goad emit` is
  slice 005's and `slice-004.md` §Non-goals says a shell one-liner is this
  slice's client.

## Findings against the design

Recorded rather than patched (`docs/AGENTS.md` §Plan). **None blocks the plan**,
and none changes a decision the design took. Every code citation in `design.md`
§§2, 5 and 9 was checked against the tree at `b6ca5f7`; the four that matter are
in FD-4.

### FD-1 — `tokio` needs `sync` as well as `net`, and §10 names only `net`

`design.md` §10's POL-001 row says *"`tokio` gains `net`"*. The stratum 2
interface in §5.2 is built on `mpsc::Receiver<Arrival>` and
`oneshot::Sender<Reply>`, and **`goad-shell` has neither feature today**: the
workspace base set is `["process", "time", "rt", "io-util", "macros"]`
(`Cargo.toml:33-34`) and `crates/goad-shell/Cargo.toml` takes it unmodified.
`sync` is `crates/goad/Cargo.toml`'s own addition and does not reach stratum 2.

Nothing about the argument changes. POL-001's residue is about a feature
*shared with stratum 1*, and `crates/goad-semantics/Cargo.toml` names `jiff`,
`serde` and `serde_json` only — tokio is not in its graph, so neither feature
can unify into it (`research.md` F11 ✓, re-verified). The manifest allowlist is
names-only and `tokio` is already on `STRATUM_2` (F12 ✓, re-verified), so
neither feature is visible to it. §10's row is right about the conclusion and
short by one feature name; PL-4 states where both go, and the audit's
reconciliation is where §10 is squared with what shipped.

### FD-2 — "a tempdir" cannot be taken literally without changing an instrument

`design.md` §9's stratum 2 integration row says *"the listener over a real socket
in a tempdir"*. There is no `tempfile` dependency anywhere in the workspace, and
`crates/goad-shell/Cargo.toml` has **no `[dev-dependencies]` table at all**.
Adding one would add a name to the manifest allowlist's `STRATUM_2`, which is an
ADR-001 instrument. PL-3 takes the existing precedent instead. Recorded because
a phase agent reading §9 alone would reach for the crate.

### FD-3 — `Config` gains a field, and four struct literals stop compiling

`AC-7`'s reading counts the 23 `serve(` call sites (verified: 1 in `main.rs`, 22
in tests — exactly 23). It does not count the **four** sites that build a
`Config` by struct literal and will need `ingress: None`:
`tests/support/driving.rs:46`, `crates/goad/tests/renderer/scheduling.rs:84`,
`crates/goad/tests/event_loop/closing.rs:63` and
`crates/goad/tests/event_loop_schedule/scheduling.rs:91`. All four are the
compiler's bill for PHASE-02 and are declared as bounded surfaces there, not
trespass. No assertion moves, so AC-7's reading survives intact.

### FD-4 — four citations checked; one is off by two lines

Checked at `b6ca5f7`, because the plan schedules against them:

| design cites | found |
|---|---|
| `controller.rs:407` `floor_until` initialised | ✓ `:407` |
| `controller.rs:420` the one scheduled-floor write site | ✓ `:420` |
| `controller.rs:429` `refusal_re_arms` | **`:427`** — `matches!(fired, Fired::Scheduled)` is at 427; 429 is inside the `match fired` below it |
| `controller.rs:505-513` the inner `select!`; `:507-512` the absorb and re-arm | ✓ both |
| `glass.rs:67-120` the presentation cost | ✓ `present` spans `:68-119`: eleven window properties, two `VecModel` rebuilds, the tray image and the tooltip, and `show()`/`hide()` |
| `goad-semantics/src/error.rs:18` `json_type_name`, `pub(crate)` | ✓ `:18` |
| `protocol/wire.rs:79` `reject_duplicate_keys`, already `pub` | ✓ `:79` |
| `canonical.rs:490-497` `Event`, four `pub` fields | ✓ (the derive is `:489`) |
| `allowlist.rs:19-27` stratum 2's allowlist | ✓, and `tokio` is on it |
| `wire.rs:41-70` `Stimulus` cannot carry an event | ✓ |

This plan cites `controller.rs:427`. Nothing else moves.

### FD-5 — AC-7's second clause names no instrument

`design.md` §9's AC-7 row is *"the existing suite with unchanged assertions,
plus: with no key configured, no file is created at any path."* The second half
is a claim about a four-line `match` inside `main::start`, which no test can
reach. PL-6 gives it one by naming the decision `startup::listener`, tested in
`startup.rs`'s own `#[cfg(test)] mod tests` — the same shape `arguments` already
has there, and for the same reason: it is a pure-over-its-input startup
decision.

## Coverage

Two tables, because the slice answers to two documents. The first walks
`slice-004.md`'s acceptance criteria; the second walks `draft-spec.md`'s
requirements, which are the slice's working authority. A gap in either is a gap
in the plan.

### Acceptance criteria

| AC | discharged by |
|----|---------------|
| AC-1 — a well-formed envelope produces one `evaluate` carrying all four fields, with the host's own `now` | PHASE-04/VT-1 |
| AC-2 — the view reaches the screen and is answerable, indistinguishably from a scheduled firing's | PHASE-04/VT-2 |
| AC-3 — exactly one reply per envelope, then close | PHASE-03/VT-5 (one line, then EOF; a second envelope on the same connection is never read), PHASE-03/VT-6 (a dropped `Answer` yields `unavailable`) |
| AC-4 — a refusal names its reason, from a closed set | PHASE-03/VT-7 (the five shape reasons, the non-object top-level among them), PHASE-03/VT-8 (**the exact token set** — R3's mitigation), PHASE-03/VT-9 (`retry_after_ms` on `too_soon` and nowhere else, rounded **up**), PHASE-04/VT-3 (`engaged`), PHASE-04/VT-4 (`too_soon`), PHASE-05/VT-4 and VT-5 (R-15's bound, both sides) |
| AC-5 — a flat-out writer does not raise the evaluation rate; the excess are refused | PHASE-04/VT-5 — **and this is F-15's settlement**: the same test records presentations over the window and asserts the cost stays at one per refusal |
| AC-6 — the two anchors are independent, falsifiably in both directions | PHASE-05/VT-1 (i, *does not delay*), PHASE-05/VT-2 (**ii, *does not advance* — the case ADR-004 named and CD-3 amends the record to cite**), PHASE-05/VT-3 (iii, a scheduled firing does not clear the event anchor) |
| AC-7 — with the key absent, nothing is bound and nothing behaves differently | PHASE-02/VT-10 (a config with no `[ingress]` yields `ingress: None`), PHASE-04/VT-6 + VA-3 (the existing suite, assertions unchanged), PHASE-06/VT-2 (`startup::listener(None)` creates no file in a directory watched for one) |
| AC-8 — a stale socket is reclaimed; a live one is a startup failure | PHASE-03/VT-1 (reclaim), PHASE-03/VT-2 (`InUse` naming the path, and the first listener keeps serving) |
| AC-9 — a regular file, or an uncreatable path, is a startup error naming what was found; exit non-zero | PHASE-03/VT-3 (a regular file), PHASE-03/VT-4 (an uncreatable path), PHASE-06/VT-3 (`StartupError::Ingress` renders and maps to exit 2) |
| AC-10 — the bound socket's mode is owner-only under any umask | PHASE-03/VT-10 |
| AC-11 — the boundary holds; the four instruments and the vocabulary scan pass; stratum 1 gains no dependency | PHASE-07/VA-2 over the finished tree; every phase's VA-1 in the working tree |
| AC-12 — a malformed envelope never reaches the backend; no ingress failure takes the host down | PHASE-03/VT-11 (no `Event` at the judge), PHASE-05/VT-6 (after a flood of malformed envelopes the host still evaluates) |
| AC-13 — a person has run it and watched the prompt appear | PHASE-06/VH-1 |

### `draft-spec.md`'s requirements

`draft-spec.md` §7 states a *verified by* row per requirement in prose and says
its table *"is completed by slice 004's plan and phases"*. This table names the
criterion that is that row; PHASE-07/EX-1 replaces the prose with the test names
those criteria produced.

| R | discharged by |
|---|---|
| R-1 — bind iff configured; otherwise behave exactly as before | PHASE-03/VT-12 (a configured path is bound and serves), PHASE-06/VT-2 (no path ⇒ no file), PHASE-04/VT-6 (the existing suite) |
| R-2 — the host sets the mode itself | PHASE-03/VT-10 |
| R-3 — a stale socket is reclaimed; a live one is a failure | PHASE-03/VT-1, PHASE-03/VT-2 |
| R-4 — anything else at the path is a startup failure naming it | PHASE-03/VT-3, PHASE-03/VT-4, PHASE-06/VT-3 |
| R-5 — no unlink on exit | **review, not a test.** `draft-spec.md` §7 says so in terms: the absence of an unlink cannot be asserted without asserting the absence of code. PHASE-07/EX-2 records it as such rather than letting it look discharged; R-3's reclaim test is what makes the absence safe |
| R-6 — one envelope per connection, newline or EOF | PHASE-03/VT-5 |
| R-7 — every read bounded in bytes and time | PHASE-03/VT-13 (`too_large`), PHASE-03/VT-14 (`timed_out`) |
| R-8 — exactly one reply, then close; unanswered only if the process is gone | PHASE-03/VT-5, PHASE-03/VT-6 |
| R-9 — one object, exactly four keys, each violation named | PHASE-02/VT-2 (a non-object top-level, naming the type found), VT-3 (missing), VT-4 (wrong-typed), VT-5 (empty `source`/`kind`), VT-6 (unknown), VT-7 (duplicate at depth) |
| R-10 — RFC 3339 with an explicit offset, refused distinctly | PHASE-02/VT-8 |
| R-11 — no interpretation; all four fields reach the backend; `now` is the host's | PHASE-04/VT-1 |
| R-12 (SPEC-003's) — engaged or inside the spacing ⇒ refused, never queued | PHASE-04/VT-3, VT-4, VT-5; PHASE-05/VT-1, VT-2, VT-3 |
| R-13 — `source: "host"` refused | PHASE-02/VT-9 |
| R-14 — a machine-readable reason from the closed set; `retry_after_ms` on `too_soon` alone, rounded up | PHASE-03/VT-8, PHASE-03/VT-9 |
| R-15 — a refusal decided while idle also reaches the diagnostics surface; one decided during an exchange does not | PHASE-05/VT-4 (positive), PHASE-05/VT-5 (negative — what makes the bound a claim rather than an excuse) |
| R-16 — no envelope terminates the host, panics it, or reaches the backend malformed | PHASE-03/VT-11, PHASE-05/VT-6 |

---

## PHASE-01 — The A-1 probe

**Objective:** A-1 is a measurement rather than an assumption. Either the design
stands as written, or the slice stops here.

**Surfaces:** `docs/slices/004/ingress-probe.local.rs` (**new**, gitignored by
`*.local.*`); `docs/slices/004/research.md` (a new Thread 3); a **temporary**
`[[test]]` entry in `crates/goad/Cargo.toml`, reverted before the phase ends;
`docs/slices/004/notes.md`.

**Must not touch:** anything under `crates/*/src`; any test file; any other
manifest; any document under `docs/specs/`, `docs/policy/` or `docs/adr/`;
`design.md`, `draft-spec.md`, `canon-delta.md`, `slice-004.md`,
`review-design.md`.

**Entry**
- EN-1 — the tree is at the accepted design (`b6ca5f7` or a descendant that
  changes no source), and `just check` exits 0 before anything is edited.

**Exit**
- EX-1 — `docs/slices/004/ingress-probe.local.rs` exists, on the shape of
  `docs/slices/003/timer-probe.local.rs`: a header comment stating the question,
  the temporary `[[test]]` stanza needed to run it, and the command; one process,
  several cases, each pushing a line into a shared report printed at the end.
- EX-2 — the probe has been run and its output is pasted verbatim into
  `research.md` as **Thread 3 — Spike A-1**, with a results table in the shape
  `docs/memory/tokio-time-runs-under-slints-executor.md` uses.
- EX-3 — `crates/goad/Cargo.toml` is byte-identical to its state at EN-1, and
  `git status` shows no untracked file other than the gitignored probe.
- EX-4 — a one-line **verdict** in `research.md` and in `notes.md`: *A-1 holds*,
  or *A-1 fails, and here is what was observed*.

**Verification**
- VT-1 — **P-A, the arrival.** Under the production arrangement — a multi-thread
  runtime whose `EnterGuard` is held for the life of a real (headless) Slint
  event loop, with a `slint::spawn_local` future doing the waiting — a
  `tokio::spawn`ed task accepting on a `tokio::net::UnixListener` delivers a
  connection to that future through an `mpsc::channel`. Record the elapsed time
  from the client's `connect` to the arrival being observed in the
  `spawn_local` future's `select!`.
- VT-2 — **P-B, the answer.** The `spawn_local` future replies on a
  `oneshot::Sender`; the accept task writes one line and closes; the client
  reads it. Record connect-to-reply elapsed, and the bytes read.
- VT-3 — **P-C, during an exchange.** The same, with the arrival landing while
  the `spawn_local` future is awaiting a real `Host::evaluate` against a child
  process, pinned in an inner `select!` alongside the receiver. This is the
  inner arm's shape and it is what makes `engaged` reachable at all
  (`design.md` §5.4, I-2). Record whether the arrival was observed **before**
  the exchange completed, and the elapsed time.
- VT-4 — **P-D, the negative control, and the decisive one.** VT-1 repeated with
  **nothing else driving the Slint loop**: the timer parked an hour out, no
  command channel traffic, no click, and a deliberate idle of at least a second
  before the client connects. If the arrival is observed only because something
  else woke the loop, VT-1's number is an artefact. Record the elapsed time and
  state explicitly that no other source was armed.
- VA-1 — `just check` exits 0 with `crates/goad/Cargo.toml` reverted, output
  pasted into the phase sheet.

**STOP**
- S-1 — **the falsification, and what happens.** Any of: an arrival not observed
  within a small multiple of the connect under VT-4 (treat *anything over
  100 ms with nothing else armed* as red); an arrival observed only after an
  unrelated wake; a hang; a panic; or `UnixListener::bind` failing under the
  runtime guard. On any of these the agent **stops**. It records the measurement
  and the verdict (EX-2, EX-4) and reports to the orchestrator. It does **not**
  improvise a fallback and does **not** proceed to PHASE-02.

  Two repairs are foreseeable and **both are design changes**, so both go back
  to the design stage with their own `design-log.md` entry rather than into this
  phase: running the accept loop as a second `slint::spawn_local` future on
  Slint's own executor — the arrangement
  `docs/memory/tokio-time-runs-under-slints-executor.md` already measured green,
  and therefore the strictly weaker claim — or waking the loop explicitly with
  `slint::invoke_from_event_loop`. Naming them here is so the report can be
  precise, not a licence to take one.
- S-2 — the probe cannot be written without touching `crates/*/src`. Stop: it is
  a probe, and a probe that needs production code changed is already the
  measurement failing.

**Notes for the implementer**

- `docs/slices/003/timer-probe.local.rs` is the precedent for **everything**
  about this: the header, the temporary `[[test]]` stanza, the
  `Rc<RefCell<Vec<String>>>` report, the `slint::quit_event_loop()` at the end,
  and the final `assert_eq!(report.len(), N)` that makes a silently-skipped case
  a failure. Copy its shape; do not invent a second one.
- It initialises with `i_slint_backend_testing::init_integration_test_with_system_time()`,
  then `PromptWindow::new()` / `Tray::new()`, then `runtime.enter()`, then
  `slint::spawn_local`, then `run_event_loop_until_quit`. Constructing a `Tray`
  under the headless backend logs a `Failed to create system tray icon` line;
  that is noise, not a failure (the memory file says so).
- The socket path: `std::env::temp_dir()` joined with a name carrying
  `std::process::id()`, unlinked before binding and after. Keep it short —
  `sun_path` is about 108 bytes.
- `tokio::net::UnixListener::bind` is a **synchronous** function that needs a
  reactor in context, which is what the `EnterGuard` provides. Call it on the
  main thread after `runtime.enter()`, exactly as `main::start` will
  (`main.rs:63`, `:102`).
- `crates/goad-shell` does not have tokio's `net` or `sync` features yet, but
  `crates/goad` has `sync` and the probe is a target of `crates/goad`. For
  `net`, the probe may add `features = ["net"]` to the **temporary** `[[test]]`
  arrangement's crate — if that turns out to need a manifest edit beyond the
  `[[test]]` stanza, make it, note it, and revert it with EX-3.
- This phase measures. It does not decide. A green result is not permission to
  start PHASE-02 in the same session — set the phase `done`, update the Harvest,
  and hand off.

---

## PHASE-02 — The configuration key, and the envelope

**Objective:** the host can be told where to listen, and stratum 2 can turn
bytes a watcher wrote into an `Event` or into a named refusal — with no socket,
no runtime and no clock anywhere in it.

**Surfaces:** `crates/goad-shell/src/config.rs`;
`crates/goad-shell/src/error.rs`; `crates/goad-shell/src/lib.rs`;
`crates/goad-shell/src/ingress/mod.rs` (**new**, this phase only declares the
module and its envelope submodule); `crates/goad-shell/src/ingress/envelope.rs`
(**new**); `crates/goad-semantics/src/error.rs` — **bounded to one word**, the
visibility of `json_type_name` at `:18`; `tests/support/driving.rs`,
`crates/goad/tests/renderer/scheduling.rs`,
`crates/goad/tests/event_loop/closing.rs`,
`crates/goad/tests/event_loop_schedule/scheduling.rs` — **bounded to adding
`ingress: None` to one `Config` struct literal each** (FD-3);
`docs/slices/004/notes.md`.

**Must not touch:** anything under `crates/goad/src`; `crates/goad-shell/src/`'s
other modules; any manifest; any `.slint` file; any test body in the four
bounded files beyond the one field each.

**Entry**
- EN-1 — PHASE-01's exit criteria are discharged and its verdict is *A-1 holds*.
- EN-2 — `just check` exits 0.

**Exit**
- EX-1 — `Config` carries `pub ingress: Option<IngressConfig>` and
  `IngressConfig { pub path: PathBuf }`. `File` carries
  `ingress: Option<FileIngress>` with `#[serde(deny_unknown_fields)]` like its
  siblings, so `[ingress]` exists in the canonical form before any config may
  write it.
- EX-2 — `ConfigError` gains `EmptyPath { key: &'static str }`, raised for
  `ingress.path = ""` with `key: "ingress.path"`, on `EmptyCommand`'s argument:
  the unusable value is not representable past the boundary. Its `Display`,
  its `source()` arm and the exhaustive matches over `ConfigError` are all
  updated; no `_` arm is introduced.
- EX-3 — `crates/goad-shell/src/lib.rs` declares `pub mod ingress;`. Its module
  doc says what the module owns and cites `SPEC-003`.
- EX-4 — `ingress/envelope.rs` carries a permissive `Envelope` and
  `pub fn normalize(bytes: &[u8]) -> Result<Event, EnvelopeFault>` (or the
  equivalent two-step; the **name and the shape** are the phase's, the rule is
  that normalization is the only door). `EnvelopeFault` names each clause of
  `SPEC-003/R-9` and `R-10` precisely: not an object (naming the type found),
  missing, wrong-typed, empty, unknown, duplicate, and the two timestamp faults.
  Every fault carries the key or the type name the requirement says it must.
- EX-5 — duplicate keys at any depth are refused by **reusing**
  `goad_semantics::protocol::wire::reject_duplicate_keys`
  (`protocol/wire.rs:79`, already `pub`). No second walk is written.
- EX-6 — the timestamp rule is the same jiff two-step `schedule.rs:73-86` uses:
  the absolute parse first, and the civil parse is what produces the *missing
  offset* fault distinct from a general parse failure. It is written to mirror
  `SPEC-001/R-22`'s terms, because the offset rule is now stated twice
  (`design.md` §5.1) and the two must not drift.
- EX-7 — `source == "host"` is refused with its own fault (`SPEC-003/R-13`), and
  it is **the only comparison the host makes on `source`**. `kind` is checked for
  emptiness and carried; `data` is never inspected.
- EX-8 — `goad_semantics::error::json_type_name` is `pub`, its doc comment
  amended to say it is now the workspace's one such table rather than the
  crate's, and **nothing else in `goad-semantics` changes**: no new dependency,
  no new module, no behaviour change (`design.md` D-18, AC-11).
- EX-9 — both new files carry `#![deny(clippy::arithmetic_side_effects)]`
  (PL-5).
- EX-10 — the four `Config` struct literals compile with `ingress: None` added
  and **nothing else changed** in those files.

**Verification**
- VT-1 — `config.rs`'s existing test module gains: a config carrying
  `[ingress]\npath = "./goad.sock"` loads with `ingress` `Some` and the path as
  written; `path = ""` is refused as
  `ConfigError::EmptyPath { key: "ingress.path" }`; and an unknown key inside
  `[ingress]` is refused naming it. **`an_unknown_key_is_refused_and_named` is
  not edited** — its `socket = "/tmp/goad.sock"` fixture plants that key under
  `[backend]`, which is still unknown.
- VT-2 — a well-formed JSON value that is **not** an object is refused naming
  the type found (`SPEC-003/R-9`'s first clause — the gap `review-design.md`
  F-7 closed).
- VT-3 — each of the four keys missing, one case each, the refusal naming it.
- VT-4 — each of the four keys wrong-typed, the refusal naming it.
- VT-5 — an empty `source` and an empty `kind`, each named.
- VT-6 — a fifth key beside the four, refused naming it.
- VT-7 — a duplicate key at the **top level** and one **nested inside `data`**,
  both refused naming the key.
- VT-8 — `SPEC-003/R-10`: an offsetless instant is refused with a fault
  **distinct** from an unparseable one, and both are distinct from a
  wrong-typed `timestamp`. Assert the discriminants, not the message.
- VT-9 — `SPEC-003/R-13`: `{"source":"host", …}` with every other field valid is
  refused with its own fault.
- VT-10 — **AC-7's first half**: a config with no `[ingress]` section loads with
  `ingress: None`.
- VT-11 — the accepted shapes, so nothing above is vacuous: the design's own
  example envelope normalizes; `"data": null` is accepted; a `data` carrying a
  nested object and an array is accepted and reaches `Event.data` **unchanged**;
  a timestamp in 1970 and one in 3000 are both carried, unjudged
  (`design.md` §5.5, OQ-10).
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — `cargo test -p goad-semantics` named separately in the sheet: it is the
  gate's own third command and the only one that builds stratum 1 with exactly
  its own feature set (POL-001 §Compliance). It must be green **after** EX-8.
- VA-3 — `git diff crates/goad-semantics/` pasted: one word, plus the doc
  comment sentence EX-8 names. Anything else in that crate is a breach of AC-11.
- VA-4 — `git diff` over the four bounded test files pasted: four lines, each
  `ingress: None`.

**STOP**
- S-1 — the envelope cannot be normalized without naming a value's *content* —
  a `kind` matched against a list, a `data` key read, a `timestamp` compared to
  now. Stop: that is `CLAUDE.md`'s first invariant and `design.md` P-1.
- S-2 — `reject_duplicate_keys` turns out not to reach the case, so a second
  walk is wanted. Stop: `design.md` §5.2 says *"no new code, and the same rule
  the backend wire follows"*, and a second duplicate-key walk is the parallel
  implementation `CLAUDE.md` forbids.
- S-3 — the four bounded test files need more than `ingress: None`. Stop: an
  assertion moving is AC-7's reading failing.

**Notes for the implementer**

- The two halves are independent; do the config half first — it is smaller and
  it has an existing test module to extend, so a failure there is unambiguous.
- `Config` is `#[derive(Debug)]` with public fields and no `#[non_exhaustive]`.
  `IngressConfig` needs `Debug`; `missing_debug_implementations` is `deny`.
- `config.rs`'s own doc comment says *"Brief §5's three values and nothing
  else (the OQ-4 decision)"*. That sentence is now false and this phase owns
  correcting it — a fourth value is a deliberate addition, not a drift.
- The permissive/canonical split is already written twice in this workspace and
  the second one is the model: `File` → `Config` in the same file you are
  editing, and `protocol/wire.rs` → `protocol/normalize.rs`. Read the second
  before writing `envelope.rs`; the fault vocabulary there
  (`ProtocolError`, and `ScheduleError`'s permissive-in / precise-diagnostic
  split) is the shape `EnvelopeFault` should have.
- `Event`'s four fields are `pub` (`canonical.rs:490-497`) and
  `Timestamp::new(jiff::Timestamp)` is `pub` (`canonical.rs:105`), so stratum 2
  can build one with no accessor owed. `jiff` is on stratum 2's allowlist.
- `json_type_name` returning `&'static str` is the whole point of widening it:
  a diagnostic names a type and never formats the offending value. Do not add a
  second table, and do not match on `serde_json::Value`'s six discriminants
  locally — `design.md` D-18 rejects both by name.
- `ingress/mod.rs` in this phase is a module declaration, its doc comment, and
  the module-level lint attribute. Everything else in it is PHASE-03's. Keep
  every item `pub` and reachable from `lib.rs` so `dead_code` — `warn` in the
  table, promoted to an error by the gate's `-D warnings` — does not fire on
  code whose caller lands next phase.

---

## PHASE-03 — The listener, the reply, and the socket's lifecycle

**Objective:** a bound socket answers every envelope exactly once, in a closed
vocabulary, with the whole of `SPEC-003`'s connection contract held against a
fake judge — before `serve` has ever seen an `Arrival`.

**Surfaces:** `crates/goad-shell/src/ingress/mod.rs`;
`crates/goad-shell/src/ingress/envelope.rs` (only if a fault needs adding for a
case PHASE-02 did not reach); `crates/goad-shell/Cargo.toml`;
`crates/goad-shell/tests/integration/main.rs` — **bounded to one `mod ingress;`
declaration**; `crates/goad-shell/tests/integration/ingress.rs` (**new**);
`docs/slices/004/notes.md`.

**Must not touch:** anything under `crates/goad/src` or
`crates/goad-semantics/src`; `crates/goad-shell/src/`'s other modules;
`tests/support/`; `tests/backends/`; the root `Cargo.toml`; the other case files
in `crates/goad-shell/tests/integration/`; `crates/goad-shell/tests/integration/harness.rs`.

**Entry**
- EN-1 — PHASE-02's exit criteria are discharged and `just check` exits 0.
- EN-2 — `envelope::normalize` exists and is covered by PHASE-02/VT-2..VT-11.

**Exit**
- EX-1 — `crates/goad-shell/Cargo.toml` reads
  `tokio = { workspace = true, features = ["net", "sync"] }` and nothing else in
  that manifest changes (PL-4, FD-1). The root `Cargo.toml` is untouched.
- EX-2 — `ingress/mod.rs` declares the three budgets as `pub const`, with the
  values and the doc comments `design.md` §5.2 gives them:
  `ENVELOPE_LIMIT = 64 * 1024`, `ENVELOPE_DEADLINE = 500 ms` — whose doc says in
  terms that it bounds the **read**, not the connection — and
  `SOCKET_MODE = 0o600`.
- EX-3 — `pub fn bind(path: &Path) -> Result<Ingress, IngressError>` probes,
  reclaims, binds, sets the mode and spawns the accept task, synchronously,
  under a runtime context its caller holds. `IngressError` is **one struct** —
  the path, and a fault naming what was found — so every message names the path
  once: not a socket, in use by a live host, unprobeable, unlinkable,
  unbindable, mode unsettable.
- EX-4 — `Ingress` carries `none()` and
  `async fn arrival(&mut self) -> Option<Arrival>`, cancel-safe, which **never
  resolves** under `none()` and yields `None` when a *bound* receiver's senders
  are all gone — dropping the receiver on the way out so the arm parks from then
  on instead of spinning. `Arrival::into_parts` yields
  `(Result<Event, Refusal>, Answer)`, each half consumed exactly once by type.
  `Answer::accepted(self)` and `Answer::refused(self, &Refusal)` both consume
  `self`; a **dropped** `Answer` is the defined `unavailable` path.
- EX-5 — `Refusal` carries its own payloads — `TooSoon { retry_after }`,
  `TooLarge { limit }`, `TimedOut { after }`, `InvalidEnvelope(EnvelopeFault)`
  and the rest — with `reason() -> &'static str` for the wire and `Display` for
  `detail`. The reason set is **closed at eight**: `malformed`,
  `invalid_envelope`, `reserved_source`, `too_large`, `timed_out`, `engaged`,
  `too_soon`, `unavailable`. `Refusal`'s match is exhaustive with no `_` arm.
- EX-6 — the reply is one JSON line then a close: `{"protocol":1,"accepted":true}`
  or `{"protocol":1,"accepted":false,"reason":…[,"retry_after_ms":…][,"detail":…]}`.
  `retry_after_ms` is present **exactly** when `reason` is `too_soon`, and is
  **rounded up** to the millisecond (`SPEC-003/R-14`).
- EX-7 — framing is one envelope per connection, terminated by the first newline
  **or** by end of input, whichever comes first. Bytes after the first newline
  are never read.
- EX-8 — the accept loop is **sequential**: it awaits the judgement of the
  arrival it handed over before accepting the next connection, so at most one
  arrival is outstanding and the host holds no queue (I-2). An `accept()` error
  does not end the task; the task ends when the loop's channel closes.
- EX-9 — `Ingress`, `Arrival`, `Answer`, `Refusal` and `IngressError` are all
  `Send` and all derive or implement `Debug`. This is a criterion, not a note:
  `clippy::future_not_send` and `missing_debug_implementations` are both `deny`,
  and PHASE-04 puts three of these types inside `serve`'s future and `Served`.

**Verification**

All cases below are in `crates/goad-shell/tests/integration/ingress.rs`, over a
real socket in a per-case directory under `std::env::temp_dir()` (PL-3), against
a **fake judge**: a test-side task that takes `Arrival`s and answers them
according to a script — accept, refuse with a given `Refusal`, or drop the
`Answer`.

- VT-1 — **AC-8, R-3 reclaim.** A socket file left behind with no listener is
  unlinked and rebound, and the new listener serves.
- VT-2 — **AC-8, R-3 live.** A second `bind` against a path a **live** listener
  holds fails with the `InUse` fault naming the path, and the first listener is
  still serving afterwards — asserted by writing an envelope to it and reading a
  reply, not by inspecting the error alone.
- VT-3 — **AC-9, R-4.** A regular file at the path: `bind` fails with the fault
  naming what was found.
- VT-4 — **AC-9, R-4.** A path that cannot be created (a component that is not a
  directory, or a directory with no write permission): `bind` fails naming the
  path.
- VT-5 — **AC-3, R-6, R-8.** Three arms in one shape: an envelope terminated by
  a newline is accepted; one terminated by closing the write side (no newline)
  is accepted; and a connection carrying `envelope\nsecond envelope\n` gets
  **exactly one** line back, then EOF, and the judge saw **one** arrival.
- VT-6 — **AC-3, R-8.** A judge that drops the `Answer` yields exactly one reply
  saying `unavailable`, then a close.
- VT-7 — **AC-4.** The five listener-decided shape reasons, one case each, read
  off the wire: `malformed` (bytes that are not one JSON document, and the empty
  payload), `invalid_envelope` (the non-object top level among them),
  `reserved_source`, `too_large`, `timed_out`. The judge is never reached for
  any of them — asserted, because shape refusals travel to the loop as refusals
  but must never arrive as an `Event`.
- VT-8 — **AC-4, R-14, and R3's mitigation.** One test asserts the **exact
  token set**: the eight strings `Refusal::reason()` can return, compared as a
  set against a literal list in the test. A reason added, removed or renamed
  fails here rather than at a client one slice later.
- VT-9 — **AC-4, R-14.** A `too_soon` reply carries `retry_after_ms`; **no other
  reply carries the key at all** (assert its absence over every other reason);
  and the value is **rounded up** — for a remaining spacing that is not a whole
  number of milliseconds, `retry_after_ms` is strictly greater than the
  truncated value, so a writer waiting exactly that long is outside the spacing.
  Assert the rounding on a value chosen to have a non-zero sub-millisecond
  remainder; a value that divides evenly proves nothing.
- VT-10 — **AC-10, R-2.** Under a deliberately permissive umask set inside the
  case, the bound socket's `mode() & 0o777 == 0o600`.
- VT-11 — **AC-12, R-16.** A malformed envelope produces **no `Event`** at the
  judge, and the listener accepts and answers a well-formed envelope
  immediately afterwards on a new connection — the liveness half, without which
  the absence assertion is vacuous.
- VT-12 — **R-1.** The positive control: a configured path is bound, a
  well-formed envelope reaches the judge as an `Event` whose four fields are the
  ones written, and the reply says `accepted`.
- VT-13 — **R-7.** More than `ENVELOPE_LIMIT` bytes before the envelope ends is
  refused `too_large`, and the connection is closed rather than left open.
- VT-14 — **R-7.** A connection that writes nothing at all is refused
  `timed_out` after `ENVELOPE_DEADLINE`, and the listener serves the next
  connection normally.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — the per-case elapsed time of VT-14 recorded against `ENVELOPE_DEADLINE`
  (500 ms), and of any other case that waits on a bound. Margins are measured at
  **the bound that governs**, not at the test's wall time
  (`docs/memory/timed-test-margins-are-measured-at-the-bound.md`). A margin
  under 10x is a STOP, not a note.
- VA-3 — `git status` after the whole test run shows **no** socket file left
  anywhere in the checkout, and the temp directories the cases created are gone.

**STOP**
- S-1 — a case cannot be written without a queue, a retry, or a second arrival
  outstanding. Stop: I-2 and `SPEC-003` §5 say the host holds no queue, and
  D-2 rejects `try_send` fullness as a refusal because *that is a queue of one*.
- S-2 — the read cannot be bounded in both bytes and time without a second
  concurrency dimension (a task per connection). Stop: D-9 rejects it by name.
- S-3 — a margin measured under VA-2 comes in under 10x.
- S-4 — an existing case in `crates/goad-shell/tests/integration/` goes red.
  Record what changed and stop; do not adjust it.

**Notes for the implementer**

- **Do not reach for `tempfile`.** PL-3 and FD-2 explain why, and
  `crates/goad-shell` has no `[dev-dependencies]` table at all. The precedents
  are `crates/goad-shell/src/config.rs:226` and
  `tests/support/scripting.rs::marker`, both of which build a unique temp path
  from `std::env::temp_dir()` and `std::process::id()`. Add the case name too,
  because `cargo test` runs cases in parallel in one process.
- **Cargo runs a test binary with the package root as its cwd**, not the
  workspace root (`docs/memory/cargo-test-cwd-is-package-root-not-workspace-root.md`).
  Every socket path in this file is absolute.
- **Rounding up without a division.** `clippy::integer_division` is `deny` and
  PL-5 puts `arithmetic_side_effects` on the module, so the obvious
  `(nanos + 999_999) / 1_000_000` is doubly out. The shape that is legal and
  says what it means is
  `remaining.checked_add(Duration::from_nanos(999_999)).unwrap_or(Duration::MAX).as_millis()`,
  narrowed to the wire's integer type with `u64::try_from` — `as_conversions`
  and the four `cast_*` lints are all `deny`, so there is no cast available.
  `review-design.md` F-16 is why the rounding is specified at all.
- The mode is set **after** the bind (A-5), so there is a window in which the
  socket carries the ambient umask. That is stated in the design, not defended:
  do not add a `umask` dance to close it.
- **No unlink on exit** (D-16). Nothing in this phase may add a `Drop` that
  unlinks; VT-1's reclaim path is what makes the absence safe, and it is
  exercised on every ordinary restart precisely because there is no unlink.
- A symlink at the path is *not a socket* and is refused rather than followed
  (`design.md` §5.5). A symlink **to** a socket is ambiguous and is refused too.
- `reject_duplicate_keys` and `normalize` are PHASE-02's and should need no
  change. If a case here reaches a fault the envelope cannot express, that is a
  finding for `notes.md` and a small addition to `envelope.rs` — not a new
  vocabulary in `mod.rs`.
- The fake judge is a fixture, not a framework. Keep it in `ingress.rs`; it has
  one consumer (PL-2).

---

## PHASE-04 — `serve`'s ingress arms, the second anchor, and what a refusal costs

**Objective:** an envelope becomes an evaluation. `serve` judges every arrival
in whichever `select!` takes it, shape before state, on a second anchor that
neither writes nor is written by the scheduled one — and the price a refused
envelope charges the UI thread is a measured number rather than an argument.

**This is the phase `review-design.md` F-15 names.** Its VT-5 is the test F-15
names. Read F-15's body before starting.

**Surfaces:** `crates/goad/src/controller.rs`;
`crates/goad/src/diagnostics.rs`; `crates/goad/tests/renderer/main.rs` —
**bounded to one `mod ingress;` declaration**;
`crates/goad/tests/renderer/ingress.rs` (**new**); `crates/goad/src/main.rs` and
the 22 test call sites — **bounded to adding one argument to each `serve(…)`
call** (see EX-7); `docs/slices/004/notes.md`.

**Must not touch:** `crates/goad/src/{glass.rs, wire.rs, reception.rs,
install.rs, startup.rs, view_model.rs}`; `crates/goad/ui/app.slint`; anything
under `crates/goad-shell/src` or `crates/goad-semantics/src`; `tests/support/`;
`tests/backends/`; any manifest; any test **body** in `wiring.rs`,
`scheduling.rs`, `table.rs`, `event_loop/closing.rs` or
`event_loop_schedule/scheduling.rs` beyond the one argument EX-7 names.

**Entry**
- EN-1 — PHASE-03's exit criteria are discharged and `just check` exits 0.
- EN-2 — `bind`, `Ingress`, `Arrival`, `Answer` and `Refusal` exist, are `Send`
  and `Debug` (PHASE-03/EX-9), and are covered by PHASE-03/VT-1..VT-14.

**Exit**
- EX-1 — `serve` takes one added parameter, `ingress: Ingress`, and `Served`
  gains `pub ingress: Ingress` so the loop hands it back the way it hands back
  `host`, `controller` and `glass`. **Safe**: no test destructures `Served` —
  verified, the only two mentions outside `controller.rs` are doc comments in
  `wiring.rs:899` and `:1088`.
- EX-2 — `Fired` gains `Ingested(Arrival)`. `Refused` (`diagnostics.rs:53-62`)
  gains `Ingress { reason, detail }`, and `Diagnostics::refused` gains its arm.
  `Refused` derives `Debug, Clone, PartialEq, Eq`, so both fields must.
- EX-3 — the **outer** `select!` gains a fourth arm, **last** in `biased` order:
  cancel → commands → sleep → ingress. Ingress sits **below the timer** so a
  watcher emitting at machine rate cannot starve a scheduled firing (D-14).
- EX-4 — the **inner** `select!` becomes a **loop** with three arms in `biased`
  order: cancel → call → ingress. `call` is pinned across iterations and the
  outer loop carries a label so cancellation still breaks out of it. Refusing an
  arrival resumes waiting on the **same** exchange; nothing re-invokes the
  backend.
- EX-5 — the **order of judgement** is `design.md` §5.4's five steps, and steps
  1 and 2 hold in **both** arms:
  1. the arrival carries a shape refusal → refuse with it. This is the inner
     arm's first step as much as the outer's: an exchange in flight does not
     turn a malformed envelope into `engaged`;
  2. an exchange is in flight → `engaged` (**the inner arm's only state
     answer**);
  3. inside the event spacing → `too_soon`;
  4. the clock is unreadable → `unavailable`, `detail` naming the clock, **and
     the anchor is still written** (D-15: an attempted firing writes it, so a
     broken clock produces one refusal per spacing rather than a spin);
  5. otherwise → `accepted`, write the anchor, build `Pending::Evaluate`
     directly and evaluate.
  Steps 3–5 are the **outer** arm's alone.
- EX-6 — `event_floor_until` is a second anchor on `serve`'s stack with
  **exactly one write site** — the outer ingress arm, on an *attempted* ingested
  evaluation (steps 4 and 5) — and one read site, the same arm's step 3. It
  never touches `floor_until` (`controller.rs:407`, `:420`) and `floor_until`
  never touches it. There is **one** `MINIMUM_SPACING` (`controller.rs:318`),
  used against both anchors; no second constant is introduced (D-5).
  `refusal_re_arms` (`controller.rs:427`) is unchanged and still
  `matches!(fired, Fired::Scheduled)`.
- EX-7 — the added argument is at **23** call sites: `main.rs:103` passes the
  real `Ingress` **only after PHASE-06**, and in this phase passes
  `Ingress::none()`; the 22 test sites pass `Ingress::none()` and **no assertion
  moves** (`slice-004.md` AC-7's reading). The sites are `main.rs:103`;
  `event_loop/closing.rs:82`; `event_loop_schedule/scheduling.rs:110`;
  `renderer/wiring.rs:915, 992, 1053, 1108, 1155, 1187`; and
  `renderer/scheduling.rs:166, 207, 245, 290, 335, 390, 451, 505, 572, 644, 687,
  755, 809, 874`.
- EX-8 — **`Fired` never sees a closed channel.** `arrival()` yields
  `Option<Arrival>` and `Fired::Ingested` cannot carry the `None`, so each arm
  disposes of it *where it observes it, before any `Fired` is built*: the outer
  arm folds one `Refused::Ingress` (reason `unavailable`, detail naming that
  ingress has stopped) onto the diagnostics surface and `continue`s; the inner
  arm folds the same refusal and resumes waiting on the exchange. Because no
  `Fired` is built on that path, `refusal_re_arms` is not reached, the standing
  deadline is not reset and neither anchor is written.
- EX-9 — **no `expect`, `unwrap` or `panic` is added to production code**, and
  no `#[expect]` is added anywhere in `crates/goad/src`. Nothing derived from an
  envelope is unwrapped (I-3, `SPEC-001/R-46`).
- EX-10 — the ingress arms' bodies are short and delegate. If `serve` has
  outgrown review, that is R2 firing and a finding for `notes.md`.

**Verification**

All cases in `crates/goad/tests/renderer/ingress.rs`, driving production `serve`
with a **real bound `Ingress`** and a scripted backend, on the shape
`renderer/scheduling.rs` already uses (`LocalSet`, `spawn_local`, `until`,
`invocations`, `logging_backend`/`scripted`).

- VT-1 — **AC-1, R-11.** A well-formed envelope written to the socket produces
  exactly one `evaluate` at the backend. Assert `source`, `kind` and `data`
  **byte-identical** to what was written, `timestamp` the **same instant**
  (`+10:00` in, `Z` out — A-3, verified at point of use rather than trusted),
  and the request's `now` the host's own instant, not the envelope's.
- VT-2 — **AC-2.** The view the backend returns for that evaluation reaches the
  window and is answerable: read the token off the options model with
  `current_view_token` — the same helper the scheduled tests use — and answer
  it, indistinguishably from a scheduled firing's view.
- VT-3 — **AC-4, R-12.** `engaged`: an envelope arriving while an exchange is in
  flight is refused `engaged` **promptly** — before the exchange completes, not
  after. Use `answers-as-instructed.sh`'s `@slow-view` sentinel, whose
  foreground `sleep 0.2` makes the exchange provably still in flight. Assert the
  reply arrived before the invocation count advanced.
- VT-4 — **AC-4, R-12.** `too_soon`: a second envelope inside the spacing is
  refused `too_soon`, and the reply carries `retry_after_ms`. Paired with a
  liveness control: an envelope **outside** the spacing is accepted, so the
  refusal is not the listener failing to serve anything at all.
- VT-5 — **AC-5, R-12, and F-15's settlement.** A writer emitting envelopes flat
  out over a window far shorter than the spacing. Three assertions:
  1. the **invocation** count is bounded — one accepted evaluation per spacing;
  2. the excess replies all say `too_soon`;
  3. the **presentation** count over that window is **recorded** and asserted
     against the number of refusals that caused it. One refusal costs one
     presentation by construction (`design.md` §5.5): the assertion fixes the
     cost **at one**, so a change that raised it, or that added a second route
     to the surface, fails here rather than in front of a person.
  Presentations are counted with a `Glass` decorator over `SlintGlass` that
  increments a counter and delegates (PL-8). **What this holds is the cost per
  refusal, not a ceiling on the writer** — a test detects, it does not prevent
  (`design.md` §8 R6). Record the measured numbers — presentations per refused
  envelope, and presentations per second under the flat-out writer — in the
  phase sheet.
- VT-6 — **AC-7.** `crates/goad/tests/renderer/`,
  `crates/goad/tests/event_loop/` and
  `crates/goad/tests/event_loop_schedule/` all pass with **their assertions
  unchanged**, `Ingress::none()` having been added at 22 call sites.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — the per-test elapsed time of VT-1..VT-5 recorded against the bound each
  one actually governs, in the phase sheet. **`design.md` is not edited.** A
  margin under 10x is S-3.
- VA-3 — **the bounded files are bounded.** `git diff` over `main.rs`,
  `wiring.rs`, `scheduling.rs`, `event_loop/closing.rs` and
  `event_loop_schedule/scheduling.rs` pasted, showing only the 23 one-argument
  edits and `renderer/main.rs`'s one `mod` line. No renamed symbol, no changed
  test body, no touched assertion. A mechanical churn across five files is
  exactly where a behaviour change hides.
- VA-4 — **break-and-revert on the second anchor.** Temporarily make the outer
  ingress arm skip its anchor write; confirm VT-4 goes red and VT-1..VT-3 stay
  green; revert; record it. This says the anchor is what VT-4 turns on, and it
  is cheap here because PHASE-05 will make the stronger claim.

**STOP**
- S-1 — the inner `select!` cannot become a loop without restructuring `serve`
  beyond recognition, or `call` cannot be pinned across iterations. Stop: the
  shape is `design.md` §5.4's, and a second loop or a second `select!` is a
  design change.
- S-2 — an ingested evaluation cannot be built without a `Stimulus` variant.
  Stop: D-13 rejects `Stimulus::Ingested(Event)` by name — `Stimulus` is `Copy`
  and its `event` hard-codes `source: "host"` (`wire.rs:62`), which CD-2 also
  forbids — and `Pending::Evaluate` is the vocabulary both paths share.
- S-3 — any margin under VA-2 comes in under 10x, or any existing test in the
  three `crates/goad` test targets goes red.
- S-4 — **VT-5's presentation number is bad.** If the cost per refusal cannot be
  bounded at one, or if the presentation rate under a flat-out writer is one a
  person could not sit in front of, **stop**. Per `review-design.md`'s Protocol
  a `settle-in-code` finding that survives its phase returns to the ledger
  `contested`: record the number, report it, and do not weaken `SPEC-003/R-15`
  to make the phase green. R-15 was deliberately not weakened at design; it is
  not this phase's to weaken.

**Notes for the implementer**

- Read `design.md` §5.4 whole, and its sequence diagram carefully: **position is
  time**. The reply leaves **before** the backend is called in the accepted
  branch. The loop hands the answer to the listener and then calls the backend;
  the listener's write races the exchange rather than waiting on it. A reply
  that waited for the exchange would make `engaged` unreachable, because the
  listener awaits the reply before accepting the next connection (I-2).
  `review-design.md` F-14 is the round-3 finding that this diagram had it wrong
  once already.
- `select! { biased; … }` starves what is below an always-ready arm. The order
  is a criterion (EX-3, EX-4), not a preference.
- `Ingress` must be `Send` or `serve`'s future trips `clippy::future_not_send`
  — which is `deny`, and which today is satisfied only because `serve` is
  `!Send` through its *type parameters*. `tokio::sync::mpsc::Receiver<T>` is
  `Send` when `T` is, and `Arrival`'s parts all are. If this bites, it is
  PHASE-03/EX-9 that failed, not this phase.
- The counting glass wraps `SlintGlass` rather than replacing it: VT-5 must
  measure the **real** presentation, because the cost F-15 names is
  `glass.rs:68-119`'s eleven properties, two `VecModel` rebuilds, tray image and
  tooltip. A stub glass would measure nothing. `Rc<Cell<usize>>` is fine —
  `serve`'s `G` is not required to be `Send`.
- `@slow-view` (`tests/backends/answers-as-instructed.sh`) is the vehicle for
  every "while an exchange is in flight" case: nothing is backgrounded, so the
  exchange is provably still running for the length of its foreground sleep.
  `@lingers*` background their sleep and do **not** hold the exchange open.
- The invocation log is written by the backend script *before* it reads the
  request, so `invocations(&log) >= n` proves the nth exchange **began**, not
  that it was absorbed. `renderer/scheduling.rs:141`'s `absorbed_line` helper
  exists for exactly that distinction; read its doc comment before asserting on
  anything post-absorb.
- `LIVENESS_BOUND` is 5 s (`tests/support/waiting.rs`), and `until` is the
  asserting wrapper in `renderer/harness.rs`. Use them; do not mint a third
  spelling of a poll loop.
- `MINIMUM_SPACING` is private to `controller.rs` on purpose. A test that needs
  to reason about it states the number locally, as
  `renderer/scheduling.rs:53`'s `FLOOR_MILLIS` does, with the same comment
  saying the mirror is checked by nothing.
- Do **not** widen `Refused` to carry a `goad_shell::ingress::Refusal`. The
  design says `Ingress { reason, detail }` — two rendered values — which keeps
  `diagnostics.rs` free of the ingress vocabulary and keeps `Refused`'s derives
  cheap.

---

## PHASE-05 — The two anchors, and what a person can see

**Objective:** the claims that separate this design from the alternatives it
rejected are asserted rather than argued. AC-6's case (ii) is the one ADR-004
has been waiting for since slice 003; R-15's bound is held from both sides; and
a flood of malformed envelopes leaves the host still evaluating.

**Surfaces:** `crates/goad/tests/renderer/ingress.rs`;
`crates/goad/tests/renderer/harness.rs` — **only** if PL-8's counting glass
gains a second consumer here, in which case it **moves** there unchanged in body
and signature; `docs/slices/004/notes.md`.

**Must not touch:** anything under `crates/*/src` — **this phase writes no
production code**. If a case cannot be written without changing `serve`, that is
S-1. Also: any manifest; `tests/support/`; any other test file.

**Entry**
- EN-1 — PHASE-04's exit criteria are discharged and `just check` exits 0.
- EN-2 — PHASE-04/VT-5's presentation number was good; F-15 did not return to
  the ledger `contested`.

**Exit**
- EX-1 — AC-6's three cases exist as three named tests, and each one's doc
  comment says **which alternative it falsifies** — not merely what it asserts.
- EX-2 — case (ii)'s setup **pins the ingested exchange's own `next_check`**,
  and its doc comment says why in one sentence citing `SPEC-001/R-26` and
  `controller.rs:507-512`. This is CD-3's discharge and it is the clause that
  makes the case the one ADR-004 named.
- EX-3 — R-15's bound is held from both directions by two tests, and the
  negative one's doc comment says that it is what makes the requirement a claim
  rather than an excuse.
- EX-4 — every anti-fire window in this phase is paired with a liveness
  assertion over the same mechanism.

**Verification**
- VT-1 — **AC-6 (i), *does not delay*.** An ingested exchange falling between a
  short `next_check` and its firing does **not** push that firing out by the
  spacing. Falsifies the third alternative ADR-004 lists: an anchor on *"the
  last thing the host did"*. An ingested firing never writes `floor_until`.
- VT-2 — **AC-6 (ii), *does not advance* — the case that discharges ADR-004's
  debt, and the one CD-3 amends the record to name.** A **scheduled** firing at
  T₀; an ingested firing at T₀+ε **whose own exchange resolves to a deadline no
  later than T₀+1 s**; and a `next_check` due at T₀+1 s. Assert the scheduled
  evaluation does **not** reach the backend before T₀+3 s.

  The clause about the ingested exchange's own deadline is **load-bearing, not
  decoration**. Every completed exchange re-arms the pending deadline from the
  instruction *that* exchange's backend returned (`controller.rs:507-512`,
  `SPEC-001/R-26`), so the scripted backend must answer the **ingested**
  evaluation with a `next_check` at least as short as the scheduled one's.
  Without it the deadline in force at T₀+1 s is whatever the script's default
  was, the two hypotheses do not disagree about it, and the assertion passes for
  a reason that would have held under the boolean alternative too — which is the
  defect `review-design.md` F-1 and F-12 found in the design's first two AC-6
  tests. **Under the anchor** the scheduled evaluation waits until T₀+3 s;
  **under a boolean cleared by "some other stimulus happened"** the intervening
  ingested firing clears the flag and it fires at T₀+1 s. That is the whole
  disagreement, and it is what this test turns on.
- VT-3 — **AC-6 (iii), *the event anchor is not cleared*.** An ingested firing,
  then a **scheduled** firing, then a second envelope still inside the ingested
  spacing: still `too_soon`. Holds CD-1's new rule, about which ADR-004 makes no
  claim.
- VT-4 — **R-15, positive.** A refusal the loop decides **while idle** — a
  `too_soon`, which is decided only while idle — appears on the diagnostics
  surface a person reads. Read it off the window's own
  `get_diagnostic_lines()` model, because that is what "a person can see it"
  means; `Served.controller`'s retained diagnostics after the loop stops is the
  deterministic fallback if the timing is awkward, and the phase sheet says
  which was used and why.
- VT-5 — **R-15, negative.** **The same refusal** decided **during** an exchange
  does **not** appear: a shape refusal arriving while an exchange is in flight
  (`@slow-view` again) is answered to its writer as the shape refusal it is —
  assert the reply — and is superseded by `absorb` before any presentation, so
  it never reaches the surface. This is the case `design.md` §5.4 step 1 in the
  inner arm exists to make buildable.
- VT-6 — **AC-12, R-16.** After a flood of malformed envelopes the host still
  evaluates: assert an invocation lands after the flood, and that **none** of
  the malformed envelopes produced one. Both halves, or the absence assertion is
  vacuous.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — the per-test elapsed time of VT-1..VT-6 recorded against the bound each
  governs. VT-2 spends a floor interval by construction — three seconds of gate
  time, deliberately, because that is what the case costs. Record it as such.
- VA-3 — **break-and-revert on the anchor's independence.** Temporarily make the
  ingested firing write `floor_until` as well; confirm VT-1 goes red; revert.
  Then temporarily make a scheduled firing clear `event_floor_until`; confirm
  VT-3 goes red; revert. Record both. This is what says AC-6's tests are
  falsifiable rather than merely passing, and it is what CD-3's amendment will
  rest on at reconciliation.

**STOP**
- S-1 — a case cannot be written without changing production code. Stop: either
  PHASE-04 is not done, or the design is wrong about what is observable.
- S-2 — VT-2 passes under **both** hypotheses when VA-3's break is applied — that
  is, breaking the anchor does not make it red. Stop: the case is not the one
  ADR-004 named, and CD-3 would write a false claim into canon
  (`review-design.md` F-1's original finding, recurring).
- S-3 — VT-5 shows the refusal on the surface after all. Stop: R-15's bound is
  wrong, or the inner arm is folding where it should only answer.

**Notes for the implementer**

- Read `slice-004.md` §*Readings taken in design*, the AC-6 paragraph, and
  `canon-delta.md` CD-3 whole before writing VT-2. The setup is the argument.
- `scripted(case, &[…])` gives one instruction per invocation, in order, and
  behaves past the end of the list (`tests/backends/answers-as-instructed.sh`).
  That is how each of AC-6's cases pins a different `next_check` per exchange.
  `INSTRUCT_100MS`, `INSTRUCT_60S` and `NOTHING_INSTRUCTED` in
  `renderer/scheduling.rs:31-37` are the constants to imitate; define this
  module's own rather than reaching into that one.
- The floor is three seconds and there is one constant for both anchors. VT-2's
  three seconds of wall time is not avoidable and is not a reason to shorten the
  case; `design.md` D-5 rejected a second, configurable constant precisely so
  that no test could buy time by moving a bound.
- `until(LIVENESS_BOUND, …)` is a liveness helper. An **anti-fire** window is
  the opposite shape: sleep the window, then assert the count did not move. Do
  not express an anti-fire window with `until`.

---

## PHASE-06 — Binding at startup, and the demo a person runs

**Objective:** the host binds the socket its configuration names before the
window exists, fails fatally and legibly when it cannot, and a person can start
it and poke it from a clean clone in the dev shell.

**Surfaces:** `crates/goad/src/startup.rs`; `crates/goad/src/main.rs`;
`flake.nix`; `examples/demo.toml`; `.gitignore`;
`crates/goad/tests/renderer/ingress.rs` (only if VT-3 lands there);
`docs/slices/004/notes.md`.

**Must not touch:** `crates/goad/src/controller.rs`, `glass.rs`,
`diagnostics.rs`; anything under `crates/goad-shell/src` or
`crates/goad-semantics/src`; the `justfile`'s `check` recipe or any recipe it
calls; any manifest; `docs/policy/`.

**Entry**
- EN-1 — PHASE-05's exit criteria are discharged and `just check` exits 0.

**Exit**
- EX-1 — `StartupError` gains `Ingress(IngressError)`, with a `Display` arm and
  a `source()` arm, and `main` maps it to exit code 2 like its eight siblings
  (`startup.rs:28-50`, `main.rs:21-29`). The message **names the path**.
- EX-2 — `startup.rs` carries
  `pub fn listener(configured: Option<&IngressConfig>) -> Result<Ingress, StartupError>`
  (PL-6): `None` yields `Ingress::none()` and touches nothing; `Some` calls
  `ingress::bind` and wraps its error. `main::start` calls it and nothing else
  decides this.
- EX-3 — the call sits **after** the runtime guard `main.rs:63` takes and
  **before** `slint::spawn_local` at `:102` — `design.md` §5.4's order: it needs
  the reactor, and a bind failure must be fatal before a window exists. The
  design puts it immediately after `runtime.enter()`, ahead of the window and
  tray; land it there.
- EX-4 — `main.rs:103`'s `serve(…)` passes the real `Ingress` rather than
  PHASE-04's `Ingress::none()`. That is the **only** change to that call.
- EX-5 — `flake.nix` gains `pkgs.socat`, beside `pkgs.deno` in `projectPkgs`, so
  it reaches the dev shell and the jails alike.
- EX-6 — `examples/demo.toml` gains
  `[ingress]\npath = "./goad-demo.sock"`, and its header comment block carries
  the **documented one-liner** verbatim — the `socat` form and, as its sibling,
  the `nc` form, since the newline-or-EOF framing exists for exactly those two
  (D-8, A-2). Relative paths resolve against goad's working directory, which
  `just` sets to the repository root; the existing comment already says so.
- EX-7 — `.gitignore` gains the demo socket, so `git status` is clean after a
  demo run (R5).
- EX-8 — `examples/shell/backend.sh` is **not** modified. It already prompts on
  every evaluation that is not a `respond` (`research.md` F16), so an ingested
  evaluation produces a window with no change to it — which is the point.

**Verification**
- VT-1 — `startup.rs`'s existing `#[cfg(test)] mod tests` gains: `listener`
  with a `Some` path binds and returns an `Ingress` (a `#[tokio::test]`, since
  `bind` needs a reactor); with a path that is a regular file it returns
  `StartupError::Ingress` whose `Display` **names the path**.
- VT-2 — **AC-7's second half, R-1.** `listener(None)` returns `Ok`, and a
  directory watched across the call contains **no new entry**. The negative is
  paired with VT-1's positive over the same function, so it is not vacuous.
- VT-3 — **AC-9's stratum 3 half, R-4.** `StartupError::Ingress` renders through
  `diagnostics::report_startup`'s path and maps to exit code 2. Assert the
  mapping the way the existing startup cases do.
- VA-1 — `just check` under `nix develop`, pasted.
- VA-2 — **the clean clone.** From a fresh clone in `nix develop`: `socat` is on
  `PATH`, `just demo` starts, and the documented one-liner runs without
  installing anything. Pasted into the phase sheet.
- VH-1 — **AC-13, and this phase cannot be green without it.** A person starts
  goad with ingress configured (`just demo`), runs the documented one-liner from
  a second shell, and **watches the prompt appear**. They also observe: a second
  one-liner inside the spacing being refused with `too_soon` on stdout, and a
  malformed envelope being refused naming its reason. Recorded in `notes.md`
  naming what was observed, and lifted into `audit.md` under Evidence at audit
  (`docs/AGENTS.md` §Tiers: *a green gate is not that evidence*).

**STOP**
- S-1 — `bind` cannot go where EX-3 puts it — it needs something the reactor
  guard does not provide, or a bind failure cannot be made fatal before a window
  exists. Stop: the order is `design.md` §5.4's.
- S-2 — the one-liner does not work from a clean clone with `socat` alone, or
  needs a shell quoting trick that makes it unreadable in a comment. Stop and
  consult: D-17 rejected `/tmp`, a second config and a `deno eval` one-liner by
  name, and choosing a fourth is a decision.
- S-3 — VH-1's run shows the prompt appearing but something else visibly wrong —
  a hang, a window that will not answer, a socket left behind. Record it and
  stop; it is a finding, not a note.

**Notes for the implementer**

- `main::start`'s numbered comment blocks are load-bearing documentation. Add
  the bind as its own numbered step with its own sentence, in the style of the
  seven around it.
- `StartupError` has **no** `PartialEq` — a `slint::PlatformError` inside it has
  none — so assert on `Display` or on a `matches!`, never on equality.
- `IngressError` is one struct carrying the path and a fault, so
  `StartupError::Ingress`'s `Display` can be `write!(f, "{error}")` like
  `Config`'s and `Clock`'s. Do not add a prefix that repeats the path.
- `socat`'s form half-closes its write side, so the envelope is terminated by
  EOF; `nc`'s does not, so it is terminated by the newline. Both are witnesses
  for A-2 and both belong in the comment. Test the one you write down.
- The socket in `examples/demo.toml` lives in the checkout deliberately (D-17):
  `/tmp` depends on a directory this design declines to defend. `.gitignore`
  is what keeps it out of `git status`; the boundary scans read `.rs` and
  `.slint` only, so a socket file cannot trip one.
- **`just check` must still be POL-001's six commands.** The `justfile` mirrors
  the policy and the policy is canon; this phase adds no recipe to it (PL-9).

---

## PHASE-07 — The sweep, the spec's own table, and the gate

**Objective:** the slice's working authority is true about the tree, every claim
in it names a test that exists, and the gate is green from a clean clone.

**Surfaces:** `docs/slices/004/draft-spec.md` — **§7 only**, the verification
table it says in its own words is *"completed by slice 004's plan and phases"*;
`docs/slices/004/notes.md`; `docs/slices/004/research.md` (refreshed in place
where it drifted).

**Must not touch:** any file under `crates/`; `examples/`; `flake.nix`;
`justfile`; `Cargo.toml`; anything under `docs/specs/`, `docs/policy/` or
`docs/adr/`; `design.md`, `canon-delta.md`, `slice-004.md`, `review-design.md`;
`draft-spec.md`'s §§1–6, 8 and 9. **This phase writes no code.**

**Entry**
- EN-1 — PHASE-06's exit criteria are discharged, including VH-1, and
  `just check` exits 0.

**Exit**
- EX-1 — every row of `draft-spec.md` §7 names the **test function and file**
  that discharges it, replacing the prose that names an AC id. A row naming no
  test is a row the spec may not be promoted holding, and §7 says so.
- EX-2 — R-5's row still reads **review, not a test**, with the reason it
  already carries. Recording it as such is the point; making it look discharged
  would be the defect.
- EX-3 — this plan's two Coverage tables are walked against the tree: every
  criterion id names a test that exists and passes. A criterion that no longer
  matches its test is a finding for `notes.md`, and if it is a **gap** it is a
  STOP.
- EX-4 — the margin table: every timed assertion in the slice, its bound, its
  worst measured elapsed **at the bound that governs**, and its ratio —
  collected from PHASE-03/VA-2, PHASE-04/VA-2 and PHASE-05/VA-2 into one table
  in `notes.md`. Plus the numbers F-15 asked for: presentations per refused
  envelope, and presentations per second under the flat-out writer.
- EX-5 — `notes.md`'s Harvest is complete: what now exists, what a future agent
  would otherwise rediscover, and what is still open. The A-1 measurement is a
  `docs/memory/` candidate and is named as one.
- EX-6 — `research.md` is refreshed in place where the slice moved it — F10's
  feature list, F12's allowlist reading, and Thread 3's spike result. No rounds
  appended.

**Verification**
- VT-1 — nothing new. This phase adds no test.
- VA-1 — **the gate from a clean clone.** `git clone` the working tree into a
  scratch directory, `nix develop`, `just check`, output pasted. This is what
  catches a file that exists only in the working tree and a `.gitignore` entry
  that hides one that should not be.
- VA-2 — **AC-11 over the finished tree**, stated one instrument at a time and
  **not** merged into a single number (POL-001 §Verification forbids it): the
  crate-edge rule; the manifest allowlist; the stratum 1 purity scan; `cargo
  test -p goad-semantics`; **and separately** the domain-vocabulary scan, which
  holds a different invariant and reaches the new module by walking
  `workspace.members` (verified: `vocabulary.rs`'s scan is member-enumerated, so
  `crates/goad-shell/src/ingress/` is covered without a hand-listed path). Plus
  the residue, argued rather than enforced: `tokio` gained `net` and `sync` at
  stratum 2, neither reaches stratum 1's graph, and POL-001's residue clause is
  discharged by that argument being written down.
- VA-3 — `git diff --stat` for the whole slice against `b6ca5f7`, with every
  path checked against the surfaces its phase declared. An undeclared path is
  the audit's strongest lead and is better found here.
- VA-4 — the vocabulary scan's word list re-read against the new module's names:
  event, envelope, source, kind, listener, ingress, watcher, arrival, refusal.
  None is on `vocabulary.rs:18-27`'s list — confirmed, and confirmed again
  against whatever the phases actually named things.

**STOP**
- S-1 — a Coverage row has no test. Stop: that is a planning defect discovered
  late, and it goes back to the phase that owed it rather than into the audit.
- S-2 — the clean clone fails where the working tree passes. Stop and find the
  file.
- S-3 — a phase touched a path it did not declare. Record it and stop; it is
  either a design change or scope creep, and both are the user's call.

**Notes for the implementer**

- `draft-spec.md` is the **only** document this phase edits, and only its §7.
  Everything else about the draft — its promotion to `docs/specs/`, CD-1, CD-2,
  CD-3 and the new ADR D-3 owes — happens at audit and reconciliation with
  explicit user endorsement, and none of it is this phase's.
- The margin discipline is
  `docs/memory/timed-test-margins-are-measured-at-the-bound.md`: instrument the
  helper that owns the bound, do not infer from `cargo test`'s wall time, and
  measure under real oversubscription. Temporary `eprintln!` instrumentation
  trips lints that are **not** test-exempt here, so it comes back out before the
  gate runs.
- `design.md` is **not** retro-fitted. Where the implementation departed and the
  design stands as written, that is a line under *Design drift not reconciled*
  in `audit.md`, and this phase's job is to make sure the auditor can find it.

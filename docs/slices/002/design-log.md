# Design log — Slice 002

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### 2026-09-04 — How the slice is driven, and where it stops

- **Asked:** what to set running now, given more tokens than attention.
  Options: a research spike alone; research then a drafted slice and design;
  or a standing loop through to an accepted plan.
- **Recommended:** the research spike alone, on the grounds that slice 002's
  cost estimate turns on one unknown (whether a Slint UI can be driven and
  asserted headlessly) and everything after that is cheaper to decide once it
  is answered.
- **Decided:** research, then draft `slice-002.md` and `design.md`. Stop before
  plan.
- **Consequence:** this slice runs research → slice → design → adversarial
  design review, and halts with the design presented for acceptance. `plan.md`
  is not begun.

### 2026-09-04 — Gate autonomy: an explicit deviation from `docs/AGENTS.md`

- **Asked:** how much autonomy at the gates the methodology reserves for the
  user — hold every gate as written; batch the small dispositions; or decide
  everything except canon.
- **Recommended:** hold every gate.
- **Decided:** **decide everything except canon.**
- **Consequence:** a standing deviation, recorded here because
  `docs/AGENTS.md` requires explicit user instruction to depart from the
  workflow and this is that instruction. For slice 002 only:
  - Design decisions that the methodology would put to the user one question at
    a time are taken by the agent and recorded in this log with the reasoning
    and the alternatives rejected, in the same shape as a decision the user
    took. The record is not weakened by the deviation; only the decider changes.
  - Adversarial-review findings are dispositioned by the agent without
    per-finding confirmation. The ledger still owns them end to end, ids stay
    immutable, and a disposition still states its reason.
  - **Canon still stops and waits.** Nothing in `docs/specs/`, `docs/policy/`
    or `docs/adr/` is written, amended or promoted without explicit user
    endorsement. A slice-002 `draft-spec.md` or `canon-delta.md` may be
    drafted — that is the mechanism the methodology provides for exactly this
    — but it is not promoted.
  - **ADR-002's T1 is a canon event, not a design decision.** Slint fires it;
    splitting the crate supersedes ADR-002 and therefore stops and waits.
    Research may dry-run the split for evidence; it may not land it.
  - STOP conditions in `docs/AGENTS.md` that are not decisions — an
    unanticipated obstacle that invalidates the design, a dependency the design
    did not admit — still stop, because they are not the agent's to take.


### 2026-09-04 — Scope extended: through plan, to autonomous execution

**Supersedes the first entry above.** That entry stopped the slice before
`plan.md`; this one does not.

- **Asked:** unprompted instruction, given mid-run.
- **Decided:** verbatim — "run design autonomously; work through it at a high
  level, walk me through it, ask any critical questions. Then thrash it after I
  approve the direction with codex to adversarial review out any implementation
  wrinkles." And: "from there the slice should be autonomously executable."
- **Consequence:** the slice now runs
  research → design → **walkthrough and user approval of the direction** →
  codex adversarial review → repair → plan → autonomous phase execution.

  Three things follow that the first entry did not imply:

  1. **There is one human gate, and it is the design direction.** Not the
     methodology's per-question interview — a single walkthrough at the level of
     the shape, with the questions that genuinely need the user attached to it.
     Everything before it and after it runs unattended.
  2. **The adversarial design review runs against codex** (`gpt-5.6-sol`, per
     `~/.claude/CLAUDE.md`), not only against fresh Claude agents, and it is
     briefed for *implementation wrinkles* rather than for intent. Findings land
     in `review-design.md` as the ledger requires; the deviation is only in who
     reviews and what they are pointed at.
  3. **`plan.md` acquires a hard requirement the methodology only implies:
     every phase must be executable by an agent with no user present.** Entry
     and exit criteria must be machine-checkable — `just check` exits 0, this
     test exists and fails first, this file contains this symbol — not
     judgements. STOP conditions must be stated as conditions an agent can
     recognise, not as advice. A phase that cannot be written that way is a
     phase whose design is not settled, and that is a finding against the
     design, not a licence to improvise.

  Canon is unchanged by this entry: it still stops and waits, and ADR-002's T1
  split still stops and waits.

### 2026-09-05 — ADR-002 T1: the crate splits, and it splits first

- **Asked:** T1 has fired. Split into a workspace; stay single-crate behind a
  lint quarantine; or split but fold it into the renderer work.
- **Recommended:** split, first, before Slint lands.
- **Decided:** split, first, before Slint lands.
- **Consequence:** `research.md` Thread 6 is the evidence — 111 renames, 91
  byte-identical, one substantive file change, ~6 minutes to a green gate, and
  the error-taxonomy split ADR-002 flagged as a cost was two lines. ADR-001's
  direction discipline held in production code; it slipped in two places in the
  test layout, and the split is what found them.

  Shape, as measured: `crates/goad-semantics` + `crates/goad-shell`, with the
  name `goad` reserved for stratum 3; `[workspace.dependencies]`,
  `[workspace.lints]`, one root `clippy.toml`; fixtures at `tests/fixtures/`.

  **This is the canon endorsement `docs/AGENTS.md` requires**, and it obliges
  four canon movements, all drafted in `canon-delta.md` during the slice and
  promoted at audit, none written into `docs/` mid-slice:
  1. A new ADR **superseding ADR-002**. ADR-002 says it is superseded, never
     amended, when the split happens.
  2. ADR-002's *stated reason* for expecting T1 is measurably false — an
     optional build-dependency plus `#[cfg(feature = "ui")]` gates cleanly, to
     one node. T1 fires on the un-gateable **dev**-dependency (16 → 223 crates
     in ADR-001's own `--no-default-features` column) and on the lint
     collision. The superseding ADR states the real ground.
  3. **SPEC-001 §7 names the fixture directory normatively**, so moving it is a
     canon change, not a file move.
  4. `docs/slices/001/design.md` §9 is the gate's canonical source and this
     slice changes the gate. Promoting §9 into canon is a canon creation.

### 2026-09-05 — The empty state is a tray icon, not a window

- **Asked:** what goad is when it has nothing to show — tray icon plus a prompt
  window; a hidden window with no tray; or an always-visible window.
- **Recommended:** tray plus prompt window.
- **Decided:** tray plus prompt window.
- **Consequence:** `SystemTrayIcon` is the only affordance Slint offers that
  answers brief §13's "unobtrusive but discoverable" ✓, `system-tray` is a
  default feature so it adds nothing to the 411 crates, and a visible tray keeps
  the event loop alive with no window at all — which is goad's steady state.

  Three mechanical facts from `research.md` Thread 5 that the design must state,
  each verified:
  - The tray is a separate top-level component and does **not** share globals
    with the `Window`; each gets its own copy.
  - `hide()` panics ("Constant property being changed") unless `visible` carries
    a binding.
  - The tray shows nothing until a non-empty icon image is assigned.

  Consequences beyond the tray: `run_event_loop_until_quit()` rather than
  `ComponentHandle::run()`; `slint::set_xdg_app_id` called before first show,
  since the app icon comes from the app id and the `icon` property is silently
  dropped; and the prompt window is destroyed and recreated per prompt, because
  Wayland has no unmap — so the view model lives in Rust, never in Slint
  properties that die with the window.

  What this does **not** buy: goad still cannot place a window, raise it, or
  steal focus. All three are verified no-ops on Wayland. Getting a prompt in
  front of the user is compositor policy, addressed by setting a stable xdg app
  id and documenting a window rule — not by host code.

### 2026-09-05 — The devshell supplies a font

- **Asked:** the headless test tier panics inside `fontique` with an empty
  fontconfig, and the face that makes it pass comes from the NixOS system
  profile rather than `flake.nix`. Add a font to the devshell; document the
  dependency; or measure it in a container first.
- **Recommended:** add a font package to `flake.nix`.
- **Decided:** add a font package to `flake.nix`.
- **Consequence:** slice 001's PHASE-09 verified `just check` from a clean clone
  under `nix develop`, and that guarantee becomes false the day the renderer
  lands unless the devshell supplies a font. This is the dependency addition
  `CLAUDE.md` requires be asked about rather than assumed, and it is now
  granted — scoped to a font package for the devshell, nothing else.

### 2026-09-05 — The gate is six commands, and `-p goad-semantics` earns its place

*Taken under the autonomy grant above (review round 2, F-6). Recorded as a user
decision would be.*

- **Asked:** the design said "five commands, plus one" and printed six; §9 said
  five; AC-1 said six; CD-5 said six. Round 2 further argued that `cargo test -p
  goad-semantics` adds no pass/fail coverage after `cargo test --workspace` and
  does not reject a runtime dependency, so the honest gate is five.
- **Decided:** **six**, and the sixth stays — but for a different reason than
  the one first given, and the false one is struck.
- **Why:** round 2's premise is refuted by measurement. `cargo test --workspace`
  unifies Cargo features across every member it builds, so stratum 1 is compiled
  there with whatever features stratum 2 and 3 switch on in shared dependencies.
  Probe, two members, `a` depending on `serde` with `default-features = false`
  and `b` on `serde` with `features = ["derive"]`: `cargo build --workspace`
  succeeds and `cargo build -p a` fails `error[E0433]` on `serde::Serialize`.
  `-p goad-semantics` is therefore the only gate command that builds and runs
  stratum 1 with exactly the features its own manifest asks for. Round 2 is
  right about the rest: it is **not** a purity check, and the claim that it
  "holds purity" is removed. A `tokio` entry in stratum 1's manifest passes it.
- **Rejected:** dropping it (loses the only feature-isolation check, and
  feature-unification leakage is silent — stratum 1 would compile only because
  something above it turned a feature on); keeping it with the purity claim
  intact (the claim is false and round 2 proved it).
- **Consequence:** six commands, identical text in `design.md` §5.6 and §9 item
  1, `slice-002.md` AC-1, `canon-delta.md` CD-5, and the `justfile`. The
  `test-stratum1` recipe keeps its name and changes its body.

### 2026-09-05 — The workspace invariant checks get their own member

*Autonomy grant. Review round 2, F-6 and F-8.*

- **Asked:** where `boundary.rs` ends up in a three-member workspace, and which
  member owns the new manifest test. `research.md:798-804` says its stratum-2
  home becomes wrong the moment stratum 3 arrives.
- **Decided:** a fourth member, `crates/goad-boundary` — test-only, depending on
  no other member — owns both the domain-vocabulary scan and the manifest
  allowlist test.
- **Why:** the checks are facts about the *workspace*, not about any member. Any
  member that hosts them reaches upward: in stratum 2 the scan reads stratum 3's
  markup, which is the exact defect the split exposed. A member above all of
  them reaches nowhere. It also keeps `cargo test -p goad-boundary` free of the
  Slint build, so the invariants can be run in seconds.
- **Rejected:** `crates/goad` (the top stratum sees everything, but running the
  invariants would then require compiling Slint, and the workspace's rules would
  be owned by its least stable member); `crates/goad-shell` (the upward reach
  research already named); a root package alongside the workspace table (works,
  but a member with a name is legible where a root package is not).
- **Consequence:** one more `[workspace.members]` entry, ~30 lines of manifest.
  `goad-boundary` takes `toml` from `[workspace.dependencies]` to read manifests
  — an entry that already exists in the tree, not a new dependency.

### 2026-09-05 — Members are enumerated, not listed; R7 is retired

*Autonomy grant. Review round 2, F-6.*

- **Asked:** how per-member scans are configured, and how a new member arriving
  with no scan is detected. R8 (as R7) deferred the answer to slice 004.
- **Decided:** there is no per-member configuration. The test reads
  `workspace.members` from the root manifest and applies one scan template to
  every entry.
- **Why:** a hand-maintained list of scans is the thing that gets forgotten;
  enumerating removes the failure mode rather than detecting it. A member with
  no scannable source fails the existing vacuity guard, naming itself.
- **Rejected:** a hand-written list plus a test comparing it against the members
  (two lists to keep in step, which is the same defect one indirection later);
  `cargo metadata` (a subprocess and a JSON dependency to learn what four lines
  of TOML already say). Glob entries in `workspace.members` are rejected by the
  test, because a glob hides from a reader exactly what this check exists to
  make visible.
- **Consequence:** R7 is retired from `design.md` §8 and does not become a
  follow-up.

### 2026-09-05 — The renderer inherits the workspace lint table unchanged

*Autonomy grant. Review round 2, F-16.*

- **Asked:** whether `crates/goad` inherits `[workspace.lints]`, defines its own
  laxer table, or inherits with exceptions. §5.1, A-2 and D8 each assumed a
  different answer.
- **Decided:** `lints.workspace = true`, no crate-level override. The only
  laxity is the twelve-lint `#![expect(...)]` on the generated-code quarantine
  module (D8).
- **Why:** the twelve lints are tripped by *generated* code, and a module-scoped
  expectation is exactly as wide as the problem. A crate-level laxer table would
  extend it over every hand-written renderer file — the largest body of new code
  in the slice — for the sake of code nobody wrote. §5.1's "the renderer owns
  its own laxer `[lints]`" was the loose phrasing of a true measurement, and it
  is corrected rather than implemented.
- **Rejected:** a laxer crate table (buys nothing the module expectation does
  not, and costs the discipline on the code that matters); inherit-with-
  exceptions at crate level (an exception with no site is an exception with no
  reviewer).
- **Consequence:** D3's measured grounds are unchanged — the ungateable Slint
  dev-dependency stays the decisive one. A-2 keeps its status as an assumption
  and gains a stop rule: the third distinct lint needing an `#[expect]` outside
  the quarantine stops the phase.

### 2026-09-05 — The presentation transition is a total function, and cleanup is not one of its inputs

*Autonomy grant. Review round 2, F-1.*

- **Asked:** what selects the presentation transition, given that `Host::accept`
  can return a successful new view **and** a cleanup failure together.
- **Decided:** the transition is a total function of `(which entry point produced
  the outcome, outcome.view, outcome.failure)` and nothing else. `cleanup` is
  diagnostic metadata and appears in no row. Three shifts: `Replaced`,
  `Retained`, `Closed`; seven rows over eight combinations.
- **Why:** cleanup enters at `host.rs:185-189` and is copied through `accept()`
  (`:246-253`) and `no_action()` (`:291-298`) untouched. It never reaches `State`
  — `State::issue`, `State::close` and `State::verify` are the only state writers
  and none takes it (`state.rs:64-113`). Reading it as a selector was reading a
  host-disposal fact as an interaction fact, which R-54 forbids in words and the
  code forbids by construction.
- **Rejected:** the four-row table keyed on "`failure.is_some()`, or
  cleanup-only", which is not disjoint — a successful `evaluate` with
  `view: Some` and `cleanup: Some` matched both row 1 and row 2, and a successful
  `respond` with `view: None` and `cleanup: Some` closed the interaction while
  row 1 said retain.
- **Consequence:** `Received::refused` is a `bool`; the failure's stratum (rows 5
  and 6) changes only the diagnostics, never the shift. Validation item 11 is
  written against seven rows.

### 2026-09-05 — `(view: Some, failure: Some)` is unreachable and is still written total

*Autonomy grant. Review round 2, F-1.*

- **Asked:** is the combination representable, and what does a total reducer do
  with it?
- **Decided:** unreachable — `accept()` is the only minting path and it writes
  `failure: None` — but the reducer is total anyway: a view present is `Replaced`
  whatever else the outcome carries.
- **Why:** a `Prepared` exists only because `State::issue` ran, so a view in hand
  is the newest fact about host state regardless of what else is reported. Making
  the arm total rather than unreachable costs one match arm and cannot become a
  panic.
- **Rejected:** treating the combination as a failure (it would hide an
  interaction `Host` considers outstanding, breaking I-4); and an
  `unreachable!()` arm (a panic on a value the host itself produced).

### 2026-09-05 — `Command::Choose` carries the view token

*Autonomy grant. Review round 2, F-13.*

- **Asked:** what must `Choose` carry so a delayed click cannot answer a
  different interaction?
- **Decided:** `Command::Choose { view: String, option: String }`, the view token
  being the `ViewId` string the markup was given, carried on every `OptionRow`.
  The controller compares it to the retained `Prepared::view_id.as_str()`; on a
  mismatch nothing is sent and `Refused::SupersededView` is reported.
- **Why:** during a slow exchange a second command can queue; an intervening
  `evaluate` returning a view makes `Host` replace the outstanding interaction and
  mint a new `ViewId` (`host.rs:231-237`, `state.rs:64-81`), and `Host::respond`
  validates the id it is handed but not the option's provenance
  (`host.rs:152-172`). Two views may legally reuse a backend-authored option id.
  The string is a **selector** matched against retained state, never a value
  reconstructed from a property — the same treatment the option id already gets,
  so it is one rule rather than two. `ViewId::new` is never called from a
  callback, and ids are unique by construction (`{now}#{seq}`, `state.rs:76`), so
  a match cannot be accidental.
- **Rejected:** a numeric generation counter — Slint's `int` is `i32`, so a `u64`
  counter truncates, and the round trip is exactly the numeric identity loss
  R-9/I-3 exists to prevent; and the bare option id, which is the defect.
- **Consequence:** `OptionRow` gains a third column and `chosen` takes two
  strings. `receive` returns the `ViewId` beside the presentation in `Prepared`,
  so the public copy is stated rather than claimed to live only in private
  `Host::State`.

### 2026-09-05 — Shutdown leaves the command channel; `serve` is one function both tiers call

*Autonomy grant. Review round 2, F-4 and F-5.*

- **Asked:** how does shutdown become observable while an exchange is in flight,
  and how does an item-11 test reach the *production* reduction when the loop
  lives inside `spawn_local` and the cheap tier cannot use it?
- **Decided:** two halves of one answer. A `Cancel` handle over
  `tokio::sync::watch::<bool>` is level-held and separately pollable; the loop
  `select!`s on it `biased`, idle and mid-exchange alike, so `Command` has no
  `Shutdown` variant. And `serve(host, controller, commands, cancel, clock,
  glass) -> impl Future<Output = Served<B, G>>` holds the whole loop: production
  wraps it in the one `spawn_local` block, and the cheap tier drives the identical
  call under `block_on`. `Controller::absorb` is the fold and is called only from
  `serve`.
- **Why:** a queue position cannot express a decision to stop — it can be full,
  and it is behind whatever is already in flight. `watch` is level-held, so a
  waiter arriving after the trip still completes, which is the failure mode a bare
  `Notify` has; `biased` makes the tie deterministic and `mpsc::Recv` is
  cancel-safe, so the un-polled branch loses nothing. `spawn_local` needs a
  `'static` future, so `serve` takes everything by value and hands it back in
  `Served` for a test to inspect; the only thing production adds around it is
  `slint::quit_event_loop()`, one line and the crate's only call site of it.
- **Rejected:** `Shutdown` as a queued command (the loop cannot receive it while
  awaiting — round 2's blocker); `tokio_util::sync::CancellationToken` (a new
  dependency, a hard stop); a loop body written inline in the `spawn_local` block
  (item 11 could then only duplicate the logic); and a test-only harness
  replicating the loop (a second implementation of the thing under test).
- **Consequence:** tokio's `sync` feature joins `rt-multi-thread` in the runtime
  seam's delta (`research.md:548`); `select!` comes from `macros`, already in the
  manifest. D9's tier boundary moves: cancellation becomes cheap-tier work and the
  event-loop target holds only the close-gesture wiring.

### 2026-09-05 — The queue policy is four mechanisms, and only one of them is the safety mechanism

*Autonomy grant. Review round 2, F-5.*

- **Asked:** what is the queue policy during an exchange, given that capacity 1
  does not mean one in flight?
- **Decided:** four mechanisms, each with one job. Controls are disabled while an
  exchange is in flight; the channel holds one, so at most one command survives an
  exchange; `try_send` returning `Full` reports rather than dropping; and a
  survivor bearing a superseded view token is refused. Shutdown is out of band, so
  it is never behind any of this.
- **Why:** the queue is not the safety mechanism; the token is. The bound and the
  disable only reduce how often a stale command is produced. Stating all four
  separately is what stops a future reader resting on the wrong one — round 2
  correctly showed that capacity 1 does not mean "an exchange is already
  running", because once the task takes the current command the slot is free.
- **Rejected:** rejecting every command while busy (the person's click vanishes
  with no record); an unbounded channel (a held-down control queues without
  limit); and resting on capacity 1.

### 2026-09-05 — A callback holds a `Wire`, and `busy` and `notice` are two properties

*Autonomy grant. Review round 2, F-5; integration decision.*

- **Asked:** what do the Slint callbacks hold, given a sender clone can neither
  report busy nor ask the loop to quit — and how is "busy" actually reported?
- **Decided:** one `Wire { commands: mpsc::Sender<Command>, cancel: Cancel,
  window: slint::Weak<PromptWindow> }`, cloned into every callback. `Wire::send`
  maps `Full` → write `BUSY_NOTICE` to the window's `notice` property,
  `Closed` → `slint::quit_event_loop()`; `Wire::stop` trips the cancel signal.
  `notice` is a **separate property** from `busy`.
- **Why:** `Wire` puts every callback's whole capability in one named, cloneable
  value the ownership table can state, instead of leaving "the UI reports that it
  is busy" as prose with nothing behind it. The window handle is weak because the
  component owns the callback and a strong capture is a reference cycle that leaks
  the window. The two properties are not redundancy: `busy` is already `true`
  during an exchange, so setting it again on a full channel changes nothing the
  person can see — which is the silent discard the bounded channel exists to
  prevent. `notice` is the one property `Glass::present` does not read from the
  frame; `present` writes `""` to it, so a notice survives exactly until the loop
  next presents, which is the next moment it can accept work.
- **Rejected:** a bare sender clone (cannot do either job); a strong component
  handle (a reference cycle); reporting a full channel by setting `busy` (the
  controller spec's original, refuted by its own argument); and putting the busy
  line into `Diagnostics` (back-pressure is not a fault and must not colour the
  tray).

### 2026-09-05 — Four shutdown sources, one path; `dismissed()` is deleted

*Autonomy grant. Review round 2, F-17.*

- **Asked:** what are the shutdown sources, and what happens to `dismissed()` and
  tray `quit()`?
- **Decided:** four sources — the window close (`on_close_requested` in both
  modes, returning `KeepWindowShown`), tray `quit()`, a closed command channel,
  and a startup failure before the loop exists. The first two are the identical
  `Wire::stop()` call. `dismissed()` is deleted from the markup and replaced by
  `close-diagnostics()`, the return path out of diagnostic mode.
- **Why:** SPEC-001 has no withdrawal — a view cannot be made to go away except by
  being answered or replaced — so the only honest local meaning of closing the one
  window is quitting. `dismissed()` had no producer and no transition because it
  had no meaning; `close-diagnostics()` has both.
- **Rejected:** window close = hide (it would strand an interaction `Host` still
  considers outstanding with no way to answer it — the shape I-4 forbids); and a
  close that means "quit" in one mode and "go back" in the other (a gesture whose
  meaning the person cannot predict).

### 2026-09-05 — AC-12 asserts what the host holds, not that the child is gone

*Autonomy grant. Review round 2, F-4.*

- **Asked:** what can AC-12 honestly observe, given SPEC-001's best-effort
  cancellation contract?
- **Decided:** four host-side facts — the task ends far inside the backend's
  configured timeout while an exchange is in flight; `serve` returns rather than
  being abandoned, so the exchange future was dropped; the drop happens inside the
  still-entered runtime; and `quit_event_loop()` has exactly one call site, on the
  task's completion path. That the child is gone is explicitly not asserted.
- **Why:** R-48 concedes exactly this for the dropped path — drop-time cleanup is
  the only mechanism and the host relies on it explicitly — and requires instead
  that the exchange leave behind no task or handle a drop would fail to cancel.
  SPEC-001 §7 records the precedent: slice 001's cancellation test asserts what
  the host holds and not that the child is gone.
- **Rejected:** "no backend child outlives the process" — `kill_on_drop`
  (`process.rs:64-72`) cannot await a reap once its future is dropped, so the
  claim exceeds what the transport can observe and would make AC-12 a flaky race.
  A flaky gate is worse than an honest one.

### 2026-09-05 — The glass is one total method, and the component is never recreated

*Autonomy grant. Review round 2, F-5/F-17 integration.*

- **Asked:** how does the window rebind to the process-lifetime `VecModel` after a
  hide destroys the surface?
- **Decided:** it never rebinds, because the component instance is created once at
  startup and never recreated — `hide()` destroys the Wayland surface, not the
  Rust handle. The design does not rest on that: `Glass::present` is total and
  writes every property, `set_options` included, before every show.
- **Why:** a total, idempotent write is correct under both readings of what a hide
  destroys, costs one function, and removes the question from the phase entirely.
- **Rejected:** recreating `PromptWindow` per prompt (it would drop and reinstall
  every callback and re-mint the `Wire`); and writing properties only on change
  (correct only if properties survive a hide, which `research.md:701-706` states
  two ways).

### 2026-09-05 — The clock is a `fn` pointer returning `Result`

*Autonomy grant. Review round 2, F-2 follow-through.*

- **Asked:** how is the wall clock read, and what happens when reading it fails?
- **Decided:** `pub type Clock = fn() -> Result<Timestamp, ClockError>`;
  production `wall_clock()` is `SystemTime::now().duration_since(UNIX_EPOCH)` then
  `jiff::Timestamp::from_nanosecond`. Failure before the loop is a startup failure
  (stderr, exit 2); failure inside the loop is `Refused::NoClock` — no backend
  contact, presentation retained, reported on the diagnostic surface.
- **Why:** a `fn` pointer is `Copy`, `Send`, needs no trait and no lifetime, and a
  test supplies a fixed instant in one line. Returning `Result` keeps a system
  clock outside jiff's range from becoming a panic.
- **Rejected:** `jiff::Timestamp::now()`, which needs jiff's `std` feature — and
  enabling it in stratum 3 unifies it into stratum 1's build, weakening the purity
  claim in a way the manifest test cannot see (`Cargo.toml:23`).

### 2026-09-05 — Config discovery, exactly; and one startup exit code

*Autonomy grant. Review round 2, F-2 follow-through; integration decision.*

- **Asked:** exactly how is the config path discovered, and what exit code does a
  startup failure use?
- **Decided:** zero arguments → `$XDG_CONFIG_HOME/goad/config.toml` when that
  variable is set, non-empty and absolute, else `$HOME/.config/goad/config.toml`;
  `HOME` unset is a startup failure. One argument → that path verbatim. Two or
  more → a usage error. `-h`/`--help` prints one usage block and exits 0; there is
  no other flag. Everything is read with `args_os`/`var_os`, so non-Unicode never
  needs decoding. **Every startup failure exits 2.**
- **Why:** the unset/empty/relative fallback is the XDG basedir rule as written,
  not an invention. One argument, one meaning, no guessing — the same posture the
  host takes toward an ambiguous backend message. One code for every startup
  failure keeps 1 available for a future "ran, then failed", which is the
  distinction a caller most wants.
- **Rejected:** searching several locations and taking the first hit (a config the
  person did not name is worse than an error); no help at all (a wrong path with
  no usage line is hostile); and exit code 1 for startup failures (the
  diagnostics area's original — it spends the conventional "ran and failed" code
  on a case that never ran).

### 2026-09-05 — The xdg app id is `"goad"`, and the window rule lives beside the binary

*Autonomy grant. Review round 2, F-17 follow-through.*

- **Asked:** what is the xdg app id, and where is the compositor window rule
  documented?
- **Decided:** `"goad"`, set before any component is shown. The recommended niri
  window rule keyed on that id goes in `crates/goad/README.md`, and is harvested
  to `docs/memory/` at close.
- **Why:** the id must equal the binary name so it stays stable when a `.desktop`
  file arrives. The rule is compositor policy, so it belongs beside the thing it
  configures rather than inside the host.
- **Rejected:** a reverse-DNS id (there is no owned domain, and the invented one
  would then have to match a `.desktop` file this slice does not ship); and
  documenting the rule only in the slice folder (a slice folder is not where a
  user looks).

### 2026-09-05 — `serve` is a plain fn returning `impl Future`

*Autonomy grant. Review round 2, F-4/F-16 follow-through.*

- **Asked:** how does the loop avoid `clippy::future_not_send`, which is `deny`
  and which the `Rc`-bearing Slint handles make unsatisfiable?
- **Decided:** `serve` is a plain `fn` returning `impl Future`, not a named
  `async fn`. Recorded as assumption A-5 with its fallback: if the lint still
  fires, it is answered in the renderer crate's own `[lints]`, never with an
  attribute in the source.
- **Why:** research measured the lint as live and as not reaching the inline
  `async {}` handed to `spawn_local` (`research.md:536-540`); the lint inspects
  `async fn` items. The fallback is the crate's lint table because that is a
  per-crate decision, not a per-site suppression.
- **Rejected:** a named `async fn serve` (the shape research measured the lint
  reaching); and an `#[expect]` at the function (forbidden outside the
  generated-code quarantine).

### 2026-09-05 — One consumption point for an `Outcome`, and it runs the mapper

*Autonomy grant. Review round 2, F-7.*

- **Asked:** what is the diagnostic reducer's input, given `undrawn` lives on
  `Presentation` and `Outcome` has no such field?
- **Decided:** one consuming seam function, `receive(outcome: Outcome) ->
  Received`, is the only consumer of an `Outcome` in the process. It calls
  `present` itself and hands `Presentation::undrawn` to `Diagnostics::of(reported:
  Reported, undrawn: &[Undrawn])`. `Received` carries `prepared`, `refused`,
  `next_check`, `diagnostics`.
- **Why:** `undrawn` cannot be omitted if the function that produces the
  presentation is the same function that produces the diagnostics. I-2 stops being
  a rule an agent must remember and becomes the only path the types admit. It also
  fixes the consumption point: `Outcome` is not `Clone` (`host.rs:70-96`), and
  `receive` is where it is taken apart.
- **Rejected:** `Diagnostics::from_outcome(Outcome) -> Diagnostics` as designed —
  it cannot see `undrawn`, so a successful view with undrawn parts reads as clean
  and *clears* the surface; a four-argument free function — it leaves the mapper's
  output and the outcome's residue joinable by a caller who can forget one of
  them, which is I-2 by vigilance; and making the controller build `Reported`
  itself — the same hazard, one layer up.

### 2026-09-05 — The display bound is applied last, and counted in characters

*Autonomy grant. Review round 2, F-7.*

- **Asked:** does the display bound apply before or after lossy decoding and
  newline escaping, and what stops a truncation splitting a codepoint?
- **Decided:** after. Every line is composed, then escaped, then bounded, and the
  bound counts `char`s. Truncation takes the first N `char`s, so a split codepoint
  is not representable, and the marker names elided **characters**. Limits: 4096
  for the stderr line, 1024 for every other line, 120 for the tray tooltip.
- **Why:** the bound exists for legibility of what is displayed, so it must be
  applied to the displayed form. Ordering it last makes the unit the same as the
  thing being bounded and removes the UTF-8 boundary question rather than
  answering it.
- **Rejected:** bounding the bytes first — escaping re-expands the result (one
  0x0A byte becomes two characters, one invalid byte becomes U+FFFD), so a 4 KiB
  budget can display as far more, and truncating arbitrary bytes before
  `from_utf8_lossy` can split a codepoint and manufacture a replacement character
  the backend never wrote; and bounding by grapheme cluster — it needs a
  segmentation dependency, which is a hard stop, and the failure it prevents is
  cosmetic.

### 2026-09-05 — Two truncations, two statements

*Autonomy grant. Review round 2, F-7.*

- **Asked:** how is the transport's capture truncation kept distinct from the
  display bound?
- **Decided:** two statements in two places. The display bound appends
  ` [{n} more characters not shown]` to the line it truncated.
  `Captured::truncated` gets its own line immediately before the stderr line:
  `stderr was cut at the host's capture limit; the backend wrote more than the
  host kept`. The capture line names no number.
- **Why:** two truncations happened for two reasons at two layers, and a person
  debugging needs to know which. The capture cut is the host declining to keep
  more; the display cut is the host declining to show more.
- **Rejected:** one combined truncation sentence (it would assert one cause for
  two independent events, and both can be true at once); and naming 262144 in the
  capture line — `STDERR_LIMIT` is a private const in stratum 2
  (`process.rs:26`), and copying its value to the glass creates a second statement
  of one fact that can silently drift.

### 2026-09-05 — The tray icon is a rule with no artefact

*Autonomy grant. Review round 2, F-7.*

- **Asked:** what is the tray icon, concretely?
- **Decided:** no asset file and no build-time generator. One pure function,
  `tray_icon(state: TrayState) -> slint::Image`, rasterising a 32×32 RGBA8 disc
  into a `SharedPixelBuffer<Rgba8Pixel>` with integer arithmetic and 4×4
  supersampled coverage. Idle is a ring in `#5A6B7D`; fault is a filled disc in
  `#C0392B`. Inner radius is the only parameter that differs.
- **Why:** a rule with no artefact cannot rot. The two states differ in *form* as
  well as hue, so the distinction survives a colour-blind viewer and a monochrome
  panel theme. Integer arithmetic throughout because goad's lint table refuses
  `as` conversions — the same lint that made `range.min() as f32` fail in
  `research.md` Thread 5.
- **Rejected:** two checked-in PNGs (the two binaries nobody can regenerate that
  the review named); a build-time generator emitting PNG (needs an image encoder,
  which is a dependency stop, and puts an artefact in `target/` the tests must
  then locate); and SVG assets (Slint's SVG path is a cargo feature this design
  has not measured, and betting the one image the host must show on an unverified
  rendering path is a bad trade for a filled circle).

### 2026-09-05 — Markdown is parsed once and the parse is retained

*Autonomy grant. Review round 2, F-7 follow-through.*

- **Asked:** is markdown parsed once and retained, or parsed once to classify and
  again to render?
- **Decided:** once. `Body::Rich(StyledText)` carries the parsed value, and the
  mapper is the only caller of `from_markdown`.
- **Why:** `from_markdown` is the classifier and the renderer; they are one call
  and must stay one call. `StyledText` derives `Debug, PartialEq, Clone, Default`,
  so retaining it costs `Presentation` none of its derives and no hand-written
  impl, and it is a value type rather than a component, so the mapper stays
  testable without an event loop.
- **Rejected:** classify in the mapper and re-parse at the setter (it puts the
  accept/reject decision in two places, and the second has no honest answer
  available because the surface has already reported the body as drawn); and
  keeping only the source string and re-parsing (same defect).

### 2026-09-05 — Stderr alone reports without raising fault; a renderer refusal does raise it

*Autonomy grant. Review round 2, F-7; integration decision.*

- **Asked:** does non-empty stderr raise the tray to fault, and do the renderer's
  own refusals?
- **Decided:** fault iff the diagnostics carry anything **other than stderr
  alone** — a `failure`, a `cleanup` failure, any `discarded`, any `undrawn`, or a
  `Refused`. Stderr is reported and leaves the icon idle.
- **Why:** the other five are the host refusing, losing, or failing to draw
  something. Stderr is output the backend author chose to write and asserts
  nothing about whether anything went wrong; R-42 requires it be carried with
  every outcome, so a backend that logs one line per successful run would hold the
  tray red permanently — the surface becoming noise.
- **Rejected:** fault on any diagnostic content (the permanent-red case); and
  suppressing stderr when the exchange succeeded (R-42 requires it, and the
  successful-exchange case is the one it exists for). Also rejected: exempting the
  renderer's own refusals from fault — the exemption was self-refuting, because
  `Failure::State` ("you answered a question that is not outstanding") raises
  fault while `Refused::SupersededView` is the same sentence one stratum up.

### 2026-09-05 — `ContentForm` is two variants and no payload

*Autonomy grant. Review round 2, F-19.*

- **Asked:** what does `ContentForm` actually look like?
- **Decided:** `pub enum ContentForm { Html, Uri }` in `view_model.rs`, deriving
  `Debug, Clone, Copy, PartialEq, Eq`, with a `Display` yielding the noun phrase
  `HTML` or `a URI` and no other rendering. It carries no payload.
- **Why:** naming only the two forms that reach the glass undrawn means a third
  content form added to SPEC-001 is a compile error in the mapper's `match`, not a
  silent omission. `Copy` because it is a fieldless two-variant enum; `PartialEq`
  so a mapper test asserts on `undrawn` directly.
- **Rejected:** two `Undrawn` variants instead (the shared sentence would be
  written twice); and carrying the content string in the variant (the bytes
  already reach the glass as the body, so carrying them here would render one
  value twice).

### 2026-09-05 — The window has one derived surface value, and a new question outranks a record

*Autonomy grant. Review round 2, F-15.*

- **Asked:** how are the five diagnostic-mode transitions defined, given the
  reducer's state was only presentation and visibility?
- **Decided:** `Surface` is **derived** from `(focus, shown)` — three inputs,
  three outputs, no combination unnamed. `Focus::Diagnostics` is set only by
  `OpenDiagnostics` and cleared by `CloseDiagnostics` and by a `Replaced` fold. A
  clean outcome under an open diagnostic window leaves it open showing `Nothing to
  report.`; a new view returns the window to prompt mode.
- **Why:** "window unchanged" has two meanings if visibility and mode are separate
  facts, and that ambiguity is the finding. Deriving removes the flag that could
  be preserved or lost. Clearing never closes a window a person opened, because
  auto-closing destroys the record at the moment the next quiet exchange succeeds,
  so the surface could never say "there was a fault and it is now clear". An
  interaction the person must answer outranks a record they can reopen.
- **Rejected:** separate visibility and mode fields; auto-closing on clear; and
  letting diagnostic mode hold the window against an incoming view (that hides
  the only means of answering an outstanding interaction — AC-7's own failure with
  a different cause).

### 2026-09-05 — The failure case table is written into the design, not delegated

*Autonomy grant. Review round 2, F-9.*

- **Asked:** how does one `Host<ProcessBackend>` execute the whole taxonomy of
  process behaviours, and who writes the rows?
- **Decided:** the design writes them. Slice 001's vehicle is inherited unchanged
  — one command (`bash tests/.../answers-as-instructed.sh`), the invocation log as
  argv[2], the ordered instruction list as argv[3…], handed out one per invocation
  by the script counting its own log lines. New behaviours are added as
  **sentinels to that one script**, quoted from the standalone script that already
  proves the behaviour. Thirty rows in five groups, each with its exact `Display`
  text, plus the sequence, the exemptions and the collapse table.
- **Why:** the design forbids inventing a second mechanism, and the existing one
  is the only shape a single-command `Host` admits. Adding sentinels rather than
  scripts keeps one vehicle; quoting the standalone script keeps two statements of
  a behaviour from drifting — in particular the presence or absence of `exec`,
  which is what makes a grandchild a grandchild.
- **Rejected:** a `Host` per case (defeats AC-7's one-retained-host claim, and
  slice 001 records that a fresh `Host` satisfies the reuse assertion by
  construction — the vacuity this suite exists to avoid); a dispatching wrapper
  script per behaviour (a `Host` owns one command, so the suite and the per-case
  tests would run different backends); and environment variables (process-wide and
  racy under `cargo test`'s in-process parallelism, as `harness.rs` states).

### 2026-09-05 — Rows assert the rendered text, not the Rust variant

*Autonomy grant. Review round 2, F-9.*

- **Asked:** does every row re-assert the Rust variant, as slice 001 does?
- **Decided:** no. Each row asserts the exact `Display` text of the value it
  expects, present in the reduced diagnostics **exactly once**. The variant is
  named in the row as provenance, and asserted directly only in the three cleanup
  rows, where slice 001 has no equivalent.
- **Why:** the text assertion is strictly stronger here. Slice 001's four
  `error.rs` display tests prove every variant's `Display` names every value it
  carries (`src/semantics/error.rs:270-286`), so a distinct rendered line implies
  a distinct variant, while a `matches!` on the variant proves nothing about what
  a person is shown. AC-7's claim is about the diagnostic surface, not the enum.
- **Rejected:** re-asserting `matches!(backend_error(&outcome), …)` per row —
  that is slice 001's `failure_matrix.rs` claim copied into a second tier, which
  is parallel implementation of an identical assertion.

### 2026-09-05 — Four rows read the element tree, one per channel

*Autonomy grant. Review round 2, F-9.*

- **Asked:** do all thirty rows read the element tree?
- **Decided:** no. Every row asserts the reduced `Diagnostics`; four rows — one
  per channel: a backend failure, a state refusal, a cleanup failure, a discard —
  additionally read the element tree in the same `block_on`, sharing one
  `PromptWindow` driven into `Surface::Diagnostics`.
- **Why:** the render path from `Diagnostics` to the surface is one binding shared
  by every row. What varies per row is the reduction, and that is what every row
  asserts. One row per channel is the smallest set that proves each channel is
  bound at all.
- **Rejected:** reading the tree on every row (thirty window shows to exercise one
  property binding, already exercised by item 11's transitions); and reading it on
  none (then no row proves a channel reaches the glass, and AC-7 says the surface
  shows the failure).

### 2026-09-05 — Where the exemptions, the refusals and the F-1 coda sit in the sequence

*Autonomy grant. Review round 2, F-9.*

- **Asked:** which cases cannot run inside the one-`Host` suite, where does the
  idle-state refusal fit, and how is F-1's "a view *and* a cleanup failure" case
  covered without disturbing the sequence?
- **Decided:** exactly one case gets its own `Host` — `BackendError::Spawn`, over
  a command that does not exist, as slice 001 does. The suite **opens** with
  `NoOutstandingView` while the host is idle; `StaleViewId` sits mid-sequence
  while the seeded interaction is outstanding. F-1's case is a **coda** after the
  trailing success: the seeded interaction is answered and closed, then one more
  `evaluate` runs `@lingers-with-a-view`, minting a second view alongside a
  cleanup failure, and a final `respond` answers it. Same `Host` throughout.
- **Why:** a `Host` is built around one command, and a command that cannot be
  spawned cannot first succeed. The two `StateError` variants are reachable only
  in different host states, so the sequence has to visit both, and opening idle
  costs nothing. Putting the view-minting cleanup row after the sequence keeps the
  R-34 witness the trailing `respond` depends on, and still ends the whole case on
  a success.
- **Rejected:** forcing a spawn failure inside the suite (impossible — the command
  is `bash`, which spawns; a missing script makes bash exit 127, which is
  `ExitStatus`, a different row); a separate `Host` for the idle refusal (breaks
  the one-retained-`Host` requirement for no reason); fabricating an id after the
  seed and expecting `NoOutstandingView` (it would be `StaleViewId`); and placing
  the view-minting cleanup row inside the main sequence (it replaces the
  outstanding interaction mid-run and destroys the R-34 witness).

### 2026-09-05 — The failure table drives the `Host`, and item 11 drives the channel

*Autonomy grant; integration decision reconciling the controller and failure-table
areas.*

- **Asked:** does the failure table go through `serve` and the command channel, or
  through `Host` directly?
- **Decided:** through `Host` directly, folding every `Outcome` through the
  production `receive`. Item 11 is where the command channel and `serve` are
  driven.
- **Why:** rows S1 and S2 hand a **fabricated** `ViewId` to `Host::respond`, and
  `Controller::answer` would refuse a fabricated token locally (F-13's repair), so
  those rows would never reach `State::verify` if they went through the channel.
  The table's claim is about the taxonomy surviving the real journey; the loop's
  claim is item 11's, and folding through `receive` means neither is asserting a
  copy of the reduction.
- **Rejected:** driving the table through `serve` (S1 and S2 become
  unrepresentable); and giving the table its own reduction (a second
  implementation — the defect F-4 named).

### 2026-09-05 — The driving helpers are shared by one included file, cut at the intersection

*Autonomy grant. Review round 2, F-9 prerequisite.*

- **Asked:** where do the driving helpers live once the glass tier is a different
  test crate from the shell tier?
- **Decided:** slice 001's `harness.rs` splits along a seam it already has. The
  helpers that drive a scripted process backend through a `Host` and read an
  `Outcome` move to a shared file both test crates include by `#[path]`; the
  transport-level helpers stay with the shell tier. The shared file resolves the
  script directory from `CARGO_MANIFEST_DIR` joined with `../../tests/…`, uniform
  because both members sit at depth two. `CLEANUP_LIMIT` is restated once, there.
- **Why:** the shared file must be exactly the intersection of what the two tiers
  use, or `-D warnings` fails on dead code. The intersection is precisely the
  host-driving set, which is a coherent unit and not an arbitrary cut.
- **Rejected:** copying the helpers into the renderer's test crate (parallel
  implementation, and the two copies drift on exactly the helpers that make the
  assertions mean anything); a `goad-test-support` workspace member (outside this
  slice's declared surfaces, and a dependency-shaped decision); and
  `#[allow(dead_code)]` on a wholesale include (the gate forbids silencing a lint
  outside the generated-code quarantine).

### 2026-09-05 — Integration: one vocabulary, one pair type, one home for `Refused`

*Autonomy grant. Reconciling four independently-written specifications.*

Four specifications were written in parallel by agents who could not see each
other. Where two overlapped they mostly agreed; where they disagreed the
disagreement was the useful part. Six reconciliations, each taken deliberately:

1. **"Diagnostics", not "report".** One area named the second window mode
   `Surface::Report` / `OpenReport` / `close-report()`; the other kept the
   existing "diagnostic mode", `Tray::show-diagnostics()` and the user-visible
   strings `Diagnostics` and `goad — diagnostics`. **Decided: diagnostics
   throughout** — the type is `Diagnostics`, the existing design and markup
   already say it, and the "report" naming collided with `Reported`, the struct
   that carries the outcome's residue. *Rejected:* "report" everywhere, which
   would have forced a rename of `Reported` and of every user-visible string that
   was already settled. The *substance* of the report-side proposal — that
   `dismissed()` is deleted and the return-from-mode callback is a real declared
   callback — is taken whole.
2. **`Prepared`, not `Shown`.** Both areas independently invented the pair
   `(ViewId, Presentation)`. **Decided: one type, `Prepared`**, produced by
   `receive` and retained by the controller as `shown: Option<Prepared>`. Two
   names for one struct is the drift this integration exists to prevent.
3. **`Reported` as the residue type, with `undrawn` as a second argument.** One
   area proposed `Diagnostics::reduce(Reported<'_>)` with `undrawn` as a borrowed
   *field*; the other proposed `Diagnostics::of(Report, &[Undrawn])`. **Decided:
   `Diagnostics::of(reported: Reported, undrawn: &[Undrawn])`** — the second
   area's shape, because `Reported` is then built only by `receive` and cannot be
   assembled partially, and the first area's name, because `Report` collided with
   the window mode. *Rejected:* `undrawn` as a field of `Reported`, which lets a
   caller forget it.
4. **`Refused` lives in `diagnostics.rs`.** It is a controller fact with a
   user-visible rendering. **Decided: it lives beside the strings that render it**,
   on the precedent that `StateError` lives in `error.rs` rather than in
   `state.rs` for exactly that reason, and because "every user-visible string in
   one file" is otherwise immediately false. The controller names it; it does not
   own it. `Diagnostics::refused` takes the enum, not a `&str`, so no caller can
   author a refusal sentence outside that file.
5. **`is_clear`, not `is_clean`.** Arbitrary, and therefore decided rather than
   left: `clear` matches the "clearing" language and `Nothing to report.`
6. **Cancellation moves to the cheap tier.** The expensive tier existed because
   shutdown was defined by `spawn_local`. With `serve` as one function taken by
   value, cancellation is drivable under `block_on`, so the event-loop target
   keeps only what exists nowhere else: a real close request reaching
   `Wire::stop`, `serve` returning, and the loop quitting. *Rejected:* leaving all
   of AC-12 in the expensive tier, which would pay per-target link cost for
   assertions the cheap tier can make.

The four areas also produced three straightforward contradictions of fact, each
resolved on evidence rather than preference: the startup exit code (2, not 1);
whether a renderer refusal raises the tray fault (it does); and whether a full
channel is reported through `busy` or through a separate `notice` (a separate
`notice`, because `busy` is already set). Each is recorded above with its
argument.

### 2026-09-05 — Round 3: four instruments for stratum 1, and the claim narrowed to fit

*Autonomy grant. Review round 3, F-6 — third raising of one defect.*

- **Asked:** does the gate enforce stratum 1's purity, and if not, what does it
  do?
- **Decided:** it does not, and no document says it does any more. Four
  instruments, each with a stated boundary: Cargo at crate edges; the manifest
  allowlist over dependency *names*; a new **stratum 1 purity scan** over
  stratum 1's sources for direct `std` reaches; and `cargo test -p
  goad-semantics`, which builds stratum 1 in isolation and rejects nothing. The
  feature residue is written down as a rule nothing enforces.
- **Why:** the previous two repairs each narrowed a word and left the *sum*
  implying the whole, which is how one defect got raised three times. And one of
  the boundaries turned out not to be residue but a gap: `std::fs` needs no
  manifest entry and is not a crate edge, so nothing was looking. The scan is one
  more configured `Scan` — `mentions` already matches `::`-bearing tokens as
  substrings (`boundary.rs:166-176`) and `code_of` already cuts comments, so the
  tree's one existing `std::time::` mention (a doc comment at
  `schedule.rs:215`) is not a hit. Verified against the tree before deciding.
- **Rejected:** narrowing the claim alone and leaving direct I/O as an unwritten
  review rule — an unwritten rule catches nothing, and a weak tripwire in the
  gate catches the regression people actually make; presenting the scan as proof
  — its three misses (`use std::{fs, process};`, aliases, I/O by a permitted
  dependency) are named beside it; and inventing a feature-graph test, which
  would be a fifth instrument arriving on argument rather than measurement,
  which is how this finding was created.

### 2026-09-05 — Round 3: the entry point is Rust, not a numbered list

*Autonomy grant. Review round 3, F-17 — second raising.*

- **Asked:** why did a repair that added a fifteen-step ordered sequence still
  omit the host construction?
- **Decided:** because a numbered sequence is a format that can omit a step and
  still read complete. The entry point is now Rust — `main -> ExitCode` over
  `run` over `start(&Path)` — and the markup carries the controls that produce
  every callback the Rust side installs.
- **Why:** the omissions were structural, not careless. A list has no compiler
  and no reader-side check that a value it uses was ever produced; code has both,
  even unread. Writing it forced three facts into the open that prose had been
  eliding: the command must be **cloned** before the config moves into
  `Host::new`; `main` cannot use `?`; and `Wire`'s fields are private, so a field
  literal was never going to compile.
- **Rejected:** another pass over the numbered sequence, which is the format that
  failed twice.

### 2026-09-05 — Round 3: the Slint API is read from the compiler, not inferred

*Autonomy grant. Review round 3, F-17's markup half.*

- **Asked:** how much of the markup can be pinned without a spike?
- **Decided:** all of it that this slice needs, from
  `i-slint-compiler-1.17.1`'s own sources in the cargo registry — `builtins.slint`
  for `SystemTrayIcon`'s single `Menu` child and `MenuItem`'s `activated()`,
  `widgets/fluent/button.slint` for `Button`'s built-in accessible role, label,
  default action and `FocusScope`, `typeregister.rs` for the reserved
  `accessible-*` properties, and `tests/syntax/accessibility/` for the rule that
  a component instance accepts `accessible-description` only when its own root
  declares a role.
- **Why:** the alternative was writing plausible Slint into a design an agent
  will type, which is the F-17 class of defect in a new place. Reading the
  compiler cost minutes and settled T-D and T-E as a by-product: stock `Button`
  already answers both, so the design owes neither a per-option `FocusScope` nor
  a disambiguating selector rule.
- **Found while reading, and recorded:** `StyledText` declares no accessible
  role and its property is `text`, so the body's value is not addressable through
  the accessibility tree. The body's content is therefore asserted on
  `Presentation`, not through the element tree. *Rejected:* a second
  `body-text: string` property carrying a plain copy for the label — one value
  rendered twice, which principle 4 forbids.

### 2026-09-05 — Round 3: the new gate policy is drafted from the policy template

*Autonomy grant. Review round 3, F-24. A methodology deviation, taken in the
open.*

- **Asked:** `docs/AGENTS.md` says new canon is drafted in `draft-spec.md`. The
  new canon here is a *policy*. Which template, and which filename?
- **Decided:** `docs/slices/002/draft-policy.md`, copied from
  `docs/templates/policy.md`. `canon-delta.md` CD-5 becomes the amendment it
  should always have been — repointing `CLAUDE.md` off a closed slice's design
  and onto the promoted policy — and says explicitly that it does not create the
  policy.
- **Why:** AGENTS.md names `draft-spec.md` because it assumes new canon is a
  specification; the rule that matters is that new canon is drafted in the slice
  folder from its governing template and never written into `docs/` mid-slice.
  A policy drafted from the spec template would be the wrong document in the
  right place. Two promotions, two endorsements, two Reconciliation rows: both
  land or neither does, because applying one alone leaves either a dangling
  pointer or two claimants to the gate.
- **Rejected:** stretching `canon-delta.md` to hold new canon, which its own
  preamble forbids; and deferring the policy to audit, which would leave the
  slice running with no stated authority for a gate it changes.

### 2026-09-05 — Round 3: three seams closed, one shape each

*Autonomy grant. Review round 3, F-20, F-21, F-22.*

- **Asked:** three defects at seams between a repaired passage and an unrepaired
  neighbour. Is there one repair, or three?
- **Decided:** three, but each is chosen to remove the *seam* rather than to
  patch the symptom.
  - **F-20, the quit:** the completion path owns it and `Wire::send`'s `Closed`
    arm does nothing — not by preference, but because `Wire` holds a `Sender`, so
    `Closed` can only be observed after the receiver is dropped, which happens one
    line before the completion path's own quit. A second call was never a first
    one.
  - **F-21, `engaged`:** `absorb` clears it, unconditionally. One setter, one
    clearer, one pair, in one place. *Rejected:* a separate `disengage()` (a third
    call the loop can forget) and moving `busy` into a `frame(busy)` parameter
    (it would make the caller own a fact `Controller` is otherwise sole owner of).
  - **F-22, the pending exchange:** a `Pending` enum and one `async` block, so
    `select!` races the exchange itself rather than a wrapper. *Rejected:*
    duplicated `select!`s per entry point, and boxing.
- **Why:** each of the three was a decision the design had left to the
  implementer while claiming to have settled it — and each decides borrowing,
  cancellation or the shape of a validation item, so none of them is local.

### 2026-09-05 — Round 3: the icon has numbers and the startup surface has strings

*Autonomy grant. Review round 3, F-25, F-26.*

- **Asked:** two "a rule, not an artefact / a controlled string surface" claims
  with no values behind them. What are the values?
- **Decided:** integer eighth-of-a-pixel geometry for the icon — centre 128,
  outer radius² 14 400, idle inner radius² 5 184, 4×4 sample centres at
  `(8x + 2i + 1, 8y + 2j + 1)`, both boundary comparisons inclusive. And every
  startup string written out: the usage block on stdout with `--help` as its only
  destination, `StartupError`'s eight variants, `ClockError`'s two, and
  `source()` returning `None` on both.
- **Why:** "a rule that regenerates the exact asset" is only true if the rule has
  numbers in it, and the pinning pays for itself — the centre-pixel assertion
  becomes arithmetic rather than a description. And a string nobody pinned is a
  string an implementer authors mid-phase, which is user-facing policy taken by
  whoever happened to be typing.
- **Decided alongside, because the lint table does not leave it free:**
  `print_stdout` and `print_stderr` are both `deny`, so both outlets go through
  `writeln!` on a locked handle, and the write's `Result` is discarded by
  matching — `let _ =` trips `let_underscore_must_use` and `.ok();` trips
  `unused_must_use`. Three lint interactions settled here rather than met one at
  a time inside a phase; A-2's stop rule is the fallback if the first `cargo
  clippy` disagrees.
- **Rejected:** reprinting the usage block beside a usage error (one fact in two
  places); and a `source()` chain on either error type, which is F-47's inherited
  defect re-introduced at the one outlet with no window to lose.

### 2026-09-05 — Round 3: the slice document was wrong about the clock, not the design

*Autonomy grant. Review round 3, F-23. The round's one `doc-wrong`.*

- **Asked:** the non-goal says slice 003 owns the clock; the design adds a
  wall-clock adapter. Which is wrong?
- **Decided:** the slice document. Slice 003 owns **scheduling and timers**;
  slice 002 reads wall time solely to stamp the events it sends and the calls
  that carry them. The non-goal, the slice's OQ-7, `design.md` §1 and §6's OQ-7
  are all restated in those terms.
- **Why:** every `Host` entry point has always required a caller-supplied
  `Timestamp`, including slice 001's tests, so supplying one is not owning a
  schedule — D15 drew that distinction correctly two rounds ago and is unchanged.
  The inconsistency was manufactured by OQ-7 having been *asked* with the wrong
  word ("with no clock"), and it propagated into the non-goal from there.
- **Rejected:** removing the adapter to satisfy the non-goal as written, which
  would leave the slice unable to make its first call; and leaving the non-goal
  standing as an unremarked contradiction, which `docs/AGENTS.md` explicitly
  forbids — a design change obliges revising the slice for consistency.

### 2026-09-05 — Round 3: the loop was built, and it changed `serve`'s signature

*Autonomy grant. Review round 3, F-22's repair, which produced F-27.*

- **Asked:** F-22's repair specifies a `Pending` enum and one `async` block
  raced against cancellation. The borrow analysis was reasoned. Is it right?
- **Decided:** build it. A standalone crate reproduced the loop shape —
  `Cancel` over `watch`, an `mpsc::Receiver`, `Pending`, an `Rc`-bearing glass
  presented across the loop, both `select!`s `biased` — and ran offline against
  tokio 1 (`research.md` Thread 7).
- **What held:** the borrow (released by `break`, so `Served { host, .. }` after
  the loop compiles); cancellation dropping the exchange (under 9 ms against a
  10 ms exchange, nothing folded); the level-held `Cancel`; and F-21's `busy`
  returning to false.
- **What did not, and is the reason this entry exists:** A-5. Under
  `deny(clippy::all)` + `deny(clippy::future_not_send)`, the plain-`fn`-
  returning-`impl Future` shape produces **two** errors once the future is
  really `!Send` — `future_not_send`, which A-5 assumed it dodged, and
  `manual_async_fn`, which nobody had considered and which `clippy::all` denies.
  `async fn` plus one `#[expect(clippy::future_not_send, reason = …)]` is clean
  and the expectation is fulfilled. So `serve` is an `async fn`, and that
  expectation is the first of A-2's three.
- **Why the assumption survived three rounds:** the earlier reading measured a
  `Send` future, against which the lint has nothing to fire. A vacuous
  measurement, in a lint costume —
  `docs/memory/a-bound-is-not-tested-at-the-bound.md` in yet another form.
- **Also found, by hitting it:** a capacity-1 channel with `send().await`
  deadlocks a producer that runs before the loop. Production never does this,
  but it pins a requirement the design had only implied: `Wire::send` is
  `try_send`, never `send().await`, because a Slint callback is synchronous and
  on the UI thread.
- **The generalisable lesson, recorded because it should change what the next
  round does:** "settled by running the gate on the first renderer commit" is a
  real mitigation and also a way of not finding out. A-5 sat on the
  risks-left-standing list for two rounds and was false; measuring it cost
  minutes and moved a signature at the centre of the design. Of the four
  remaining assumptions, A-6 and A-7 are reachable by one `.slint` file and
  should be measured before a phase starts rather than carried into one.

### 2026-09-05 — Round 3: the markup was compiled, and the tray could not be written

*Autonomy grant. Review round 3, F-17's repair, which produced F-28.*

- **Asked:** F-27 showed that carrying a cheap assumption costs a signature.
  A-7 — "§5.2's markup compiles as written" — is reachable by one `.slint` file.
  Measure it, or carry it?
- **Decided:** measure it. §5.2's block was extracted verbatim into a crate with
  `slint`/`slint-build` 1.17.1 and a `build.rs` calling
  `compile_with_config(.., with_debug_info(true))` (`research.md` Thread 8).
- **What held:** it compiles; the negative control (a nonsense
  `accessible-role`) fails the *build script*, so the check is not vacuous; the
  generated API is exactly what §5.3 and §5.4 name; and A-6 — the
  `Window.title` conditional over a `WindowMode` property — compiles, so its
  fallback is dropped rather than carried.
- **What did not:** the tray. `Tray` as written generated **no `set_icon` and no
  `set_tooltip`**, and the compiler marked `icon`, `title`, `tooltip` and
  `visible` constant. Every tray behaviour in the design had no route to the
  component. The repair that caused it was right about its premise —
  redeclaring an inherited property is `error: Cannot override property` — and
  wrong about the remedy: setting `visible: true` to a *literal* is not
  "carrying a binding", it folds.
- **Decided:** `Tray` declares `image`, `hover-text` and `shown` and binds the
  three builtins to them. Measured: `set_image`, `set_hover_text`, `set_shown`
  are generated and `visible` is not folded, so E-4's panic trap is closed by
  evidence rather than by belief. *Rejected:* naming them `tray-icon` /
  `tray-tooltip` (stutters against the component name); and leaving the tray
  statically configured, which would delete the two-state icon the diagnostic
  surface rests on.
- **The pattern, twice in one session:** F-27 came from building the loop, F-28
  from compiling the markup. Both were in text this round wrote. Both were
  invisible to careful reading of the upstream sources — `builtins.slint`
  documents neither behaviour. Two of the three assumptions a spike could reach
  were wrong, and both would have stopped a phase mid-flight.

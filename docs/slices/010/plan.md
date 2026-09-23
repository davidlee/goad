# Plan — Slice 010: the exit-code taxonomy

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

Three phases, each ending green on `just check`, in the order the value flows:
the pure layer first, then the seam that feeds it, then the tier that reads the
number off a real process and the consumer that depends on it.

- **PHASE-01 — the pure layer.** `exit.rs` (`Ended`, `exit::ended`,
  `exit::status`), `Cancel::is_stopped`, and `diagnostics::report_exit_line`,
  each asserted one tier down. Every piece of code the design has only ever
  compiled over stand-in types is built here, so the lint risk lands first and
  alone. Nothing calls the new code from `main` yet.
- **PHASE-02 — the seam.** `main`, `run` and `start` answer
  `Result<Ended, StartupError>`; `report_exit` replaces `report_startup`; the
  doc claims the old shape made false are repaired; and
  `structure::the_loop_s_ending_is_never_a_startup_failure` guards the call's
  line. The scan case is written first and reds on today's tree.
- **PHASE-03 — the binary tier and the consumer.** The display-free spawn
  (P-1), `exit_codes::an_unbindable_ingress_path_exits_2`, the two binary-tier
  module docs, and `nix/module.nix`'s reasoning.

What no phase does, and audit must: **AC-9** (a person on the running host sees
the lost display exit 1 and come back), the **canon promotion** (`draft-spec.md`
numbered and moved, `canon-delta.md` applied, `design.md` §10's D6 citation
obligations and the `DRAFT-ONLY` comment's removal), **AC-10** (FU-1 struck at
close), and the `research.md` count sweep (`notes.md` §Open). Each is named
under Coverage below.

`draft-spec.md` is the slice's working canon and is cited by every phase as
canon would be. `design.md` §9: **test names are commitments** — a phase that
names a case differently updates `draft-spec.md` §7 and `design.md` §9 in the
same commit, and says so in its phase sheet.

## Sequencing & rationale

**Why the pure layer first.** `exit::ended`, `Cancel::is_stopped` and the
`Option` arms of `report_exit_line` have never compiled in the tree (`notes.md`
§Open: round 6 ran them over stand-in types under the crate's lints). If
`clippy::wildcard_enum_match_arm` or `clippy::pedantic` fires on the real
`slint::PlatformError`, that must surface in a phase that touches nothing else,
so the fix is plainly a spelling and not tangled with the seam. The layer is
also self-contained: every function is `pub` in the library, so it is green
without a caller.

**Why `report_exit` waits for PHASE-02.** It would have no caller in PHASE-01,
and `report_startup` would still be `main`'s: two outlets onto one stream for a
phase. `report_exit_line` — the pure half, which the tests need — lands in
PHASE-01; the outlet lands with its caller and replaces the old one in one
movement.

**Why the scan case opens PHASE-02.** It is the one case in the slice with a
natural red: today's call line ends `.map_err(StartupError::Platform)?;`, which
the shape rule refuses. Written first, it reds on the tree; the seam turns it
green.

**Why PHASE-03 is last.** Its classifier mutation (`Err(_) => 1`) must red the
binary tier's failing cases, which read `exit::status` only once `main` does
(PHASE-02). `nix/module.nix`'s new comment states the phase rule, which is
true of the binary only after PHASE-02. The ingress case and the spawn change
could be written earlier — they pass on today's tree — but they are cheap and
share a target with that mutation.

**Nothing runs in parallel.** PHASE-02 and PHASE-03 each depend on the one
before. The surfaces are disjoint except for `diagnostics.rs` (PHASE-01 adds,
PHASE-02 removes and re-docs) and `tests/renderer/startup.rs` (PHASE-01 adds
modules, PHASE-02 rewrites its module doc).

**Size.** Each phase is well under one session (~200k tokens) including its
mutation runs. The mutation runs dominate: each is an edit, a scoped
`cargo test`, a record, and a restore. PHASE-01 carries the most.

**Mutation evidence** is recorded in the phase's sheet in `notes.md`, under a
**Mutation evidence** heading, one row per mutation: the edit (quoted), the
command, that the mutated build **compiled**, the cases that redded **by
name**, and that the restore is green. A mutation that did not compile is not
evidence (`docs/memory/a-negative-control-that-does-not-compile.md`). Runs use
`--no-fail-fast`, so the whole red set is seen and not just the first failure.
Audit cites these rows; it does not re-derive them.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | `draft-spec.md` as written at design; its §7 citations resolved by PHASE-01/EX-6, PHASE-02/EX-8, PHASE-03/EX-6; **promoted at audit** |
| AC-2 | `draft-spec.md` §Owns and §2 as written at design; **promoted at audit** |
| AC-3 | PHASE-02/EX-2, EX-3; PHASE-02/EX-6 holds it after the slice (the call's line) |
| AC-4 | PHASE-01/EX-2 and VT-1 (the pure function and every shape it sees, a real `PlatformError` among them), EX-5 (mutations); PHASE-02/EX-2 (`main` is only that function's caller) |
| AC-5 | PHASE-02/VT-3 (first run against the rewired `main`); PHASE-03/EX-3 — and every phase's `just check` |
| AC-6 | PHASE-01/EX-4 and VT-3 (the line, and its distinctness), PHASE-02/EX-2 (`main` writes it); the observation on the host is **audit** (AC-9) |
| AC-7 | PHASE-03/EX-4 |
| AC-8 | PHASE-03/EX-2 makes it applicable (the case Change 1 names exists); **applied at audit** (`canon-delta.md` Changes 1–3) |
| AC-9 | **audit only** — `audit.md` §Evidence. No phase can: nothing in the gate provides a display |
| AC-10 | **close only** — `docs/follow-ups.md` FU-1 |
| AC-11 | PHASE-01/EX-2, EX-3, VT-2 and VT-4 (`exit::ended` over each result with and without a request, one error value; `Cancel::is_stopped`); PHASE-02/EX-2 and VA-1 (`start` feeds it the post-call read) |

`design.md` §9's validation rows map onto the phases as follows: the
`exit_status`, `ended`, `stderr_outlets` and `Cancel::is_stopped` rows are
PHASE-01; the `structure` and `counting_itself` rows are PHASE-02; the
`exit_codes` rows are PHASE-03; the AC-9 row is audit. §9's mutations:

| mutation (`design.md` §9) | phase |
|---|---|
| classifier: `StoppedRunning(_) => 1` → `=> 2` | PHASE-01 |
| classifier: split arm, `StoppedRunning(None) => 0` (F-63) | PHASE-01 |
| classifier: `Err(_) => 2` → `=> 1` | PHASE-03 (reds both tiers) |
| line: `StoppedRunning(Some(_))` answers the never-started line | PHASE-01 |
| line: `StoppedRunning(None)` answers a never-started line | PHASE-01 |
| decision: `AsAsked` for every `Ok` | PHASE-01 |
| decision: `StoppedRunning` whatever was requested | PHASE-01 |
| decision: `AsAsked` whatever was requested | PHASE-01 |
| decision: `None` carried for an `Err` | PHASE-01 |
| scan: `.map_err(StartupError::Platform)?` restored | PHASE-02 |
| scan: the same through an imported variant | PHASE-02 |
| scan: a second production call | PHASE-02 |
| binary: the new case pointed at a bindable path | PHASE-03 |
| binary: `start` handing `startup::listener` `None` | PHASE-03 |

`design.md` §9's *what no mutation here can measure* — `start` passing a
constant `false`, or a read taken before the call — is PHASE-02/VA-1, a review,
and audit's code review re-reads it.

---

## PHASE-01 — the pure layer

**Objective:** how the process ended, which number that is, and which line
accompanies it are each a pure function in the library, asserted for every
shape it can see — and the stop signal can be read without waiting.

**Surfaces:** `crates/goad/src/exit.rs` (new), `crates/goad/src/lib.rs`,
`crates/goad/src/wire.rs`, `crates/goad/src/diagnostics.rs` (adding
`report_exit_line` only), `crates/goad/tests/renderer/startup.rs` (new
modules and cases only — its module doc is PHASE-02's). `draft-spec.md` §7 and
`design.md` §9 only if a case is renamed.

**Entry**
- EN-1 — `plan.md` accepted by the user; HEAD at or after the plan's
  acceptance commit; `just check` exits 0 on it.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `crates/goad/src/exit.rs` holds `Ended` (`AsAsked`,
  `StoppedRunning(Option<slint::PlatformError>)`), `exit::ended` and
  `exit::status` as `design.md` §5.2 writes them, docs included; `lib.rs`
  has its `pub mod exit;` line, and its header's running module count is
  replaced by the rule it was counting — one `pub mod` line per module, no
  number (`design.md` §10).
- EX-3 — `Cancel::is_stopped` exists, is a synchronous `bool` read of the
  `watch::Receiver` `Cancel` already holds, and is documented against
  `Cancel::stopped` so the two are not confused.
- EX-4 — `diagnostics::report_exit_line` exists with the three stderr
  sentences `design.md` §5.2 tabulates; `report_startup_line` is untouched in
  name, signature and text.
- EX-5 — every PHASE-01 mutation in the Coverage table was run, compiled, and
  redded exactly the cases `design.md` §9 names for it, recorded in the phase
  sheet.
- EX-6 — every case `draft-spec.md` §7 cites that this phase owns resolves in
  the tree by the name cited.

**Verification**
- VT-1 — `exit_status::as_asked_is_0`, `stopped_running_is_1` (a real
  `slint::PlatformError` built through `From<String>`, as
  `display_text::platform` builds one), `stopped_running_with_no_error_is_1`,
  `every_startup_failure_is_2` (representative variants, named not counted),
  in `crates/goad/tests/renderer/startup.rs`.
- VT-2 — `ended::a_loop_error_with_no_stop_requested_is_stopped_running`
  (asserts the carried error is the one given), `ended::a_loop_error_after_a_requested_stop_is_as_asked`
  — both over **one** error value —
  `ended::a_loop_that_returned_ok_with_no_stop_requested_is_stopped_running`
  (carries `None`) and
  `ended::a_loop_that_returned_ok_after_a_requested_stop_is_as_asked`, same
  file.
- VT-3 — `stderr_outlets::report_exit_line_says_nothing_when_the_end_was_as_asked`,
  `…report_exit_line_for_a_startup_failure_is_the_startup_line`,
  `…a_host_that_stopped_running_says_it_had_been_running`,
  `…a_host_that_stopped_running_with_no_error_says_it_had_been_running`, and
  `…the_stopped_line_is_not_the_line_a_host_that_never_started_writes`
  (both stopped lines against `report_startup_line` over
  `StartupError::Platform`), same file.
- VT-4 — `tests::is_stopped_is_false_until_stop_and_stays_true` in `wire.rs`'s
  own test module, beside `a_raised_notice_stays_raised_until_it_is_lowered`.
- VA-1 — `just check`'s lint pass over the real types. The lints were spiked
  on stand-ins only; see STOP conditions.
- VA-2 — no string literal or code token in `exit.rs`, or in the new sentences
  in `diagnostics.rs`, contains a word in the vocabulary scan's `DOMAIN` list —
  `journal` is the live risk (`design.md` §3). The gate's scan holds this;
  confirm it ran over the new file.

**STOP conditions** (carried from `notes.md` Handover; the phase sheet copies
them):
- If a lint fires on `exit::ended`, `exit::status`, `report_exit_line`'s
  `Option` arms or `Cancel::is_stopped`, the fix must be a **spelling** of the
  same decision. If the only fix changes a type, an arm's meaning or a
  signature, STOP — that is a design change.
- If `slint::PlatformError` cannot be carried in `Ended` as `design.md` writes
  it (a missing `Debug`, say), STOP.
- If a mutation does not red the cases `design.md` §9 names — or reds others
  — STOP and report; do not add a case to make it red without consulting.

**Notes for the implementer**

- Red is a failing assertion where it can be: write each case against a
  function whose body is deliberately wrong (e.g. `ended` answering `AsAsked`
  whatever it is given), watch the named case red, then write the body. A red
  that is only *does not compile* proves the case exists, not that it asserts.
- `Ended` has no `PartialEq` — `slint::PlatformError` has none — so the
  `ended` cases match on the variant (`let … else` or `matches!`). The
  `wildcard_enum_match_arm` deny is on `lib.rs` and `main.rs`, not on
  `tests/renderer/main.rs`, so the test target is not held by it — prefer the
  same spelling anyway. For the carried error, compare its `to_string()`
  against the value given.
- `Cancel::stopped` is the existing **future**. The new read is
  `*self.rx.borrow()`, exactly `Notice::raised`'s body. Do not touch
  `stopped`.
- `diagnostics.rs` gains `use crate::exit::Ended;`. `exit.rs` must not import
  `diagnostics` — the number and the line are separate readers of one value
  (`design.md` §5.1).
- `exit.rs`'s docs state the rule and cite no spec number (`design.md` §7 D6);
  they may cite `design.md` sections, as this crate's docs already do.
- `lib.rs`: replace only the header's counting sentences. The rest of that
  comment carries `path:line` citations (`fields.rs:2120` and others) that
  break the *by symbol* rule; they are not this slice's. Record them as a
  finding in the phase sheet for audit; do not edit them.
- Scoped command for mutations: `cargo test -p goad --test renderer
  --no-fail-fast` (and `cargo test -p goad --lib` for `wire.rs`).

---

## PHASE-02 — the seam

**Objective:** `main` has no branch: `run` answers `Result<Ended,
StartupError>`, the loop's end reaches `exit::ended` with a read of the stop
signal taken after the call, `StartupError` no longer carries the loop's end,
and a gate case holds the call's line.

**Surfaces:** `crates/goad/src/main.rs`, `crates/goad/src/startup.rs` (docs
only), `crates/goad/src/diagnostics.rs` (`report_exit` in, `report_startup`
out, and the two doc sites below), `crates/goad/tests/renderer/startup.rs`
(module doc only), `crates/goad-boundary/tests/checks/structure.rs`.
`draft-spec.md` §7 and `design.md` §9 only if a case is renamed.

**Entry**
- EN-1 — PHASE-01 `done` in `notes.md`'s status table, and its EX criteria
  hold on HEAD.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `main` is `design.md` §5.2's three lines: `run()`, then
  `diagnostics::report_exit(&outcome)`, then `ExitCode::from(exit::status(&outcome))`.
  `run`'s `Help` and `Version` arms answer `Ok(Ended::AsAsked)`. `start` keeps
  `let stop_signal = cancel.clone();` in step 6, before `cancel` moves into
  `serve`; its last two statements are `let call = slint::run_event_loop_until_quit();`
  and `Ok(exit::ended(call, stop_signal.is_stopped()))`, the call written by
  its path.
- EX-3 — `StartupError`'s docs are true of the type (`design.md` §5.2's
  `startup.rs` list): the type doc says where the loop's ending went; the
  *Every variant is exit 2: `main` has one `match`…* sentence is replaced by
  the rule (`exit::status`'s single `Err` arm, meaning owned by the spec, no
  spec number — D6); `Platform`'s doc names `set_xdg_app_id`,
  `PromptWindow::new` and `Tray::new` and not the loop call. **And `Launch`'s
  doc** — *"so `main` keeps its single exit-code decision"* — says where that
  decision now is (found at plan, `notes.md` Handover).
- EX-4 — `report_startup` is gone; `report_exit` is its replacement, with one
  caller. `report_startup_line`'s doc no longer opens on `report_startup`
  (F-34), and the module's `//!` doc names `report_exit` where it named
  `report_startup`.
- EX-5 — `crates/goad/tests/renderer/startup.rs`'s module doc states the moved
  cut: this tier holds the **numbers**, since they are a pure function's
  answers; the binary tier holds that the process answers them to a caller.
  No sentence says *no test here asserts an exit code*.
- EX-6 — `structure::the_loop_s_ending_is_never_a_startup_failure`: exactly one
  production line of `crates/goad/src` names `run_event_loop_until_quit`, and
  it satisfies `ends_at_the_loop_call` (its `code_of`-stripped, trimmed text
  ends `run_event_loop_until_quit();`). Its doc states what it does not reach
  (`design.md` §5.2, F-55). Controls in `counting_itself`:
  `the_bare_loop_call_ends_at_the_call` and
  `a_loop_call_with_its_result_re_filed_does_not`.
- EX-7 — the three PHASE-02 scan mutations were each run, compiled, and redded
  `the_loop_s_ending_is_never_a_startup_failure` and **nothing else** across
  `cargo test --workspace --no-fail-fast`; recorded in the phase sheet.
- EX-8 — every case `draft-spec.md` §7 cites that this phase owns resolves.

**Verification**
- VT-1 — `structure::the_loop_s_ending_is_never_a_startup_failure`, written
  first, reds on the tree at entry (today's line ends `…map_err(StartupError::Platform)?;`)
  and greens with EX-2. Record the red.
- VT-2 — `counting_itself::the_bare_loop_call_ends_at_the_call` (the string
  `    let call = slint::run_event_loop_until_quit();` passes) and
  `counting_itself::a_loop_call_with_its_result_re_filed_does_not` (the call
  followed by `.map_err(StartupError::Platform)?`, and by `?` alone, fail).
- VT-3 — every existing case in `crates/goad/tests/binary/exit_codes.rs`
  passes unmodified against the rewired `main` — the binary tier's first read
  of `exit::status`.
- VA-1 — **review of the one wiring no test reaches** (`design.md` §9, *what
  no mutation can measure*; `draft-spec.md` §7 R-1 and R-2): the `is_stopped`
  read is taken on a clone of the `Cancel` handed to `serve`, in the statement
  after the call's own; `exit::ended` receives the call's own result; no site
  in `crates/goad/src` other than `exit::ended` constructs
  `Ended::StoppedRunning`; nothing downstream of `exit::ended` turns an `Ended`
  into an `Err`. Record each as checked in the phase sheet, by symbol.
- VA-2 — `grep -rn "report_startup\b" crates docs/slices/010` finds no
  surviving reference to the removed outlet outside the slice's own history.

**Notes for the implementer**

- Order: VT-1 red → the seam (`main.rs`) with `report_exit` → VT-1 green →
  docs → controls → mutations.
- The three scan mutations must be spelled so the mutated build **compiles**
  (`design.md` §9): `let call = Ok(slint::run_event_loop_until_quit().map_err(StartupError::Platform)?);`,
  and the imported-variant form `use goad::startup::StartupError::Platform;`
  with `.map_err(Platform)?` in the same position. The second production call
  must be real code in `crates/goad/src` that compiles (a `#[allow(dead_code)]`
  function is fine for the mutation). Each is restored before the next.
- `production_lines` skips `#[cfg(test)]` items; `exit.rs` and `main.rs` have
  none today. `code_of` strips comments — the doc comments in `startup.rs` and
  `controller.rs` that name the function are not lines the count sees
  (verified at plan).
- `structure.rs`'s `//!` doc is not an inventory of its cases; do not make it
  one.
- `report_exit`'s doc is *stderr, once, last* (`design.md` §5.2). It writes
  through `line_to`, like the outlet it replaces.
- Test names in `main.rs` comments: this file cites `design.md` sections
  already; keep to that, no spec number (D6).

---

## PHASE-03 — the binary tier and the consumer

**Objective:** the binary tier cannot reach a display, a case reads an ingress
bind failure's status and its own line off the built binary, both binary-tier
module docs state the two-tier cut as it now is, and `nix/module.nix` argues
from phase.

**Surfaces:** `crates/goad/tests/binary/process.rs`,
`crates/goad/tests/binary/main.rs` (module doc only),
`crates/goad/tests/binary/exit_codes.rs` (one case added; module doc; no
existing case touched), `nix/module.nix` (the `Service` block's comment only).

**Entry**
- EN-1 — PHASE-02 `done`, its EX criteria hold on HEAD.

**Exit**
- EX-1 — `just check` exits 0.
- EX-2 — `exit_codes::an_unbindable_ingress_path_exits_2` spawns the built
  binary against a configuration that loads and names an ingress path that is
  a regular file; asserts status 2 and that standard error starts with
  `goad: ` followed by that socket path and `: ` — the prefix only the ingress
  arm writes (`draft-spec.md` §7 R-4, P-1). Its doc says why the status alone
  would not do.
- EX-3 — AC-5: `git diff` from the slice's base over `exit_codes.rs` shows its
  module doc, the added case and nothing else changed; every pre-existing case
  passes.
- EX-4 — AC-7: `nix/module.nix`'s comment has no exception paragraph and none
  of its three false claims — no known exception to its own directive, no
  *do not succeed on a retry*, no *`SPEC-003`'s failure vocabulary*. It states
  the phase rule: 2 is a host that never started, so a restart changes nothing
  a person has not changed first; 1 is a host that stopped running, which
  `Restart = "on-failure"` brings back after `RestartSec`; 0 is as asked. No
  spec number (D6). The directives are byte-identical.
- EX-5 — the PHASE-03 mutations were run, compiled, and redded the named
  cases, recorded in the phase sheet: `Err(_) => 2` → `=> 1` reds
  `exit_status::every_startup_failure_is_2` and every failing case in
  `exit_codes.rs`, the new one included; the bindable-path edit and `start`
  handing `startup::listener` `None` each red
  `an_unbindable_ingress_path_exits_2` **on the stderr prefix**, and **exit
  rather than hang**.
- EX-6 — every case `draft-spec.md` §7 and `canon-delta.md` Change 1 cite that
  this phase owns resolves.

**Verification**
- VT-1 — `exit_codes::an_unbindable_ingress_path_exits_2`.
- VT-2 — every pre-existing case in `exit_codes.rs` and `version.rs`, run with
  the display variables removed by `process::command`.
- VA-1 — `process::command` removes `WAYLAND_DISPLAY`, `WAYLAND_SOCKET` and
  `DISPLAY`, and its doc says why (`design.md` §5.2). Confirm by the bindable-
  path mutation finishing in seconds on this machine, where `WAYLAND_DISPLAY`
  is set, with the display line on stderr — the measurement P-1 predicted.
- VA-2 — `exit_codes.rs`'s module doc states the cut PHASE-02/EX-5 states from
  the other side: the renderer tier holds the numbers, this tier holds that
  the process answers them to a caller. It no longer says the renderer tier
  *cannot see the constant*, nor that a case reaching step 5 *would red*;
  `tests/binary/main.rs`'s doc says what the spawn now guarantees instead of
  the same *would red*.
- VA-3 — the diff of `nix/module.nix` touches comment lines only.

**Notes for the implementer**

- Write the case with `scratch_config` as it stands — it takes the contents —
  and a second scratch file for the socket path. `examples/demo.toml` shows the
  sections a loading configuration needs (`[backend]` with `command` and
  `timeout`, `[schedule]`, `[ingress]`); the plan measured
  `command = ["true"]`, `timeout = "5s"`, `default_poll = "30s"` and a
  regular-file ingress path exiting 2 with `goad: <path>: not a socket — found
  a regular file`. Name both scratch files for the case and the process id, as
  `scratch_config` does, and remove them.
- If `scratch_config` turns out to need a change, that is within the surface;
  record why in the phase sheet.
- The spawn change is the red for VA-1: before it, on this machine, the
  bindable-path mutation hangs. Do **not** run that mutation before the spawn
  change lands — it launches a real host on the desktop. Use `timeout` on any
  manual spawn.
- `nix/module.nix`'s comment argues from phase, not from the field evidence:
  the outage counts and dates belong to the slice's documents and to FU-1, not
  to a comment that must stay true after they are history.

---

## What no phase does

- **AC-9, and with it `design.md` §5.5 A2 and A5's converse.** A person on the
  running host loses the display connection and reads the unit's status: 1,
  the *stopped running* line, back within `RestartSec`. A 0 with no line is
  §8 R1's signal that the loss tripped `Cancel`. The same person exercises a
  quit and reads 0 — `draft-spec.md` §7 R-1's evidence for the edge no test
  reaches. Recorded in `audit.md` §Evidence.
- **Promotion** (`docs/AGENTS.md` §Audit, user-endorsed): `draft-spec.md` to
  `docs/specs/004-process-exit-status.md` (slug suggested there); every
  `SPEC-00N` in `canon-delta.md` substituted; Changes 1–3 applied to SPEC-003;
  the `SPEC-NNN` citation added to `nix/module.nix`'s comment, `exit.rs`'s
  module doc and `StartupError`'s type doc (D6); `draft-spec.md` §7's
  `DRAFT-ONLY` comment removed **after** checking that every case its rows name
  resolves. Each recorded in `audit.md`'s Reconciliation table.
- **Close**: FU-1 struck with its three corrections (AC-10); `research.md`'s
  count swept; the `lib.rs` line-citation finding from PHASE-01 dispositioned.

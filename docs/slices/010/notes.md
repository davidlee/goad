# Notes — Slice 010

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | pending | 2026-09-23 |
| PHASE-02 | pending | 2026-09-23 |
| PHASE-03 | pending | 2026-09-23 |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — the pure layer

**Objective:** how the process ended, which number that is, and which line
accompanies it are each a pure function in the library, asserted for every
shape it can see — and the stop signal can be read without waiting.

**Surfaces** (`plan.md` PHASE-01; anything outside this list is a STOP):
`crates/goad/src/exit.rs` (new), `crates/goad/src/lib.rs`,
`crates/goad/src/wire.rs`, `crates/goad/src/diagnostics.rs` (adding
`report_exit_line` **only**), `crates/goad/tests/renderer/startup.rs` (new
modules and cases only — its module doc is PHASE-02's). `draft-spec.md` §7 and
`design.md` §9 **only** if a case is renamed, and then in the same commit.

**Entry criteria, verified rather than assumed**

- **EN-1 — discharged**, measured in this session rather than inherited:
  - *`plan.md` accepted by the user* — `plan-log.md`, 2026-09-23, *the plan is
    accepted, and no plan review runs*. No `review-plan.md` exists, and none is
    expected.
  - *HEAD at or after the plan's acceptance commit* — HEAD is `448f678`,
    *"010: plan coverage cites the cases, not only the criteria"*, which is the
    commit the acceptance names. Working tree clean at the start of this
    session.
  - *`just check` exits 0 on it* — run here: **exit 0**. Gate total **615**;
    `cargo test --workspace` **580**. The gate runs `cargo test -p goad-semantics`
    as a command of its own, so that crate's 30 + 5 are counted twice and the
    gate total is always exactly 35 above the workspace one. **Always quote a
    count with its denominator.**

**Baseline for the two targets this phase adds to**, from the workspace run, so
the phase's own arithmetic has something to be checked against: `goad` lib
**58**, `goad` `tests/renderer` **208**. Every doc-test target is 0.

**Reading list**

Cited **by symbol**, not `path:line` (`CLAUDE.md` §Working here;
`docs/memory/cite-by-symbol-not-line-number.md`). This departs from slice 009's
sheets deliberately — the rule landed after them, and a line number in this
sheet is wrong the first time the file above it moves, which this phase will do
to `diagnostics.rs` and `startup.rs`.

*What is being written*

- `crates/goad/src/exit.rs` — **new**. `Ended`, `ended`, `status`, written as
  `design.md` §5.2 gives them, **doc comments included**: they are the design's
  load-bearing prose (D1), not decoration. It imports `crate::startup::StartupError`
  and must **not** import `diagnostics` — the number and the line are separate
  readers of one value (§5.1).
- `crates/goad/src/lib.rs` — one `pub mod exit;` line, alphabetical, between
  `draft` and `generated`. And the header comment's **counting sentences**
  (*"ten at PHASE-08, nine after 005 … Slice 009 adds two more"*) replaced by
  the rule they were counting: one `pub mod` line per module, no number
  (`design.md` §10; `docs/memory/a-count-in-a-comment-is-a-claim-nothing-checks.md`).
  Nothing else in that comment is touched — see §Findings.
- `crates/goad/src/wire.rs` — `Cancel` gains `is_stopped`. `Cancel::stop` and
  `Cancel::stopped` are **not** touched; `stopped` is the existing future, and
  the two must not be confused at a call site. The module's own
  `#[cfg(test)] mod tests` gains one case.
- `crates/goad/src/diagnostics.rs` — `report_exit_line` **only**. It gains
  `use crate::exit::Ended;`; `use crate::startup::StartupError;` is already
  there. `report_startup_line` keeps its name, its signature and its text; its
  doc comment and `report_startup` itself are **PHASE-02's**, and so is the
  module's `//!` doc.
- `crates/goad/tests/renderer/startup.rs` — two new modules, `exit_status` and
  `ended`, and five cases into the existing `stderr_outlets`. The file's
  top-level `use` list gains what they need from `goad::exit` and
  `goad::diagnostics`; each module's own `use super::{…}` follows the file's
  existing shape. **Its `//!` doc is PHASE-02's** and still says *"No test here
  runs the binary or asserts an exit code"* — leave that sentence alone, false
  though this phase makes it. That is the plan's sequencing, not an oversight.

*Design sections that bind*

- **§5.1** — the flowchart and the paragraph under it: one value leaves `run`
  and two pure functions read it, deliberately not one function answering a
  pair. The two winit falsehoods (`Err` for a requested stop, `Ok` for an
  unrequested end) are why `ended` decides on the request and reads the call's
  result only for the error it carries.
- **§5.2** — the `exit.rs` block and the `diagnostics.rs` block, as written;
  and the stderr-sentence table, which is where the new sentences come from.
- **§9** — the validation rows this phase owns and the mutations it must run
  (both enumerated below).
- **§3** — the vocabulary scan reads `crates/**/*.rs` outside `tests/`, and
  **`journal` is forbidden** in code and string literals in `exit.rs` and in the
  new sentences in `diagnostics.rs`. The evidence behind this whole slice lives
  in the systemd journal, so the word is a live temptation. Comments are cut off
  before matching (`goad_boundary::scan::mentions`, via `code_of`), so a comment
  is not a breach.
- **§7 D6** — `exit.rs`'s docs state the **rule** and cite **no spec number**.
  The draft is not canon and nothing outside the slice folder may cite it. They
  may cite `design.md` sections, as this crate's docs already do.
- **§10** — `lib.rs`'s counting header is replaced, not incremented.
- **`draft-spec.md` §7**, rows R-1 … R-6 — the working canon. Every case name
  below is cited there, and **test names are commitments**: renaming one is an
  edit to `draft-spec.md` §7 and `design.md` §9 in the same commit, recorded in
  this sheet.

*Prior art — copy these rather than inventing*

- `Notice::raised` (`crates/goad/src/wire.rs`) — the exact shape
  `Cancel::is_stopped` takes: `#[must_use]`, body `*self.rx.borrow()`, and a doc
  that says why the read is `watch::Receiver::borrow` and not
  `Cancel::stopped`'s `wait_for`.
- `a_raised_notice_stays_raised_until_it_is_lowered` (same file's `mod tests`)
  — the shape of the level-held case, and where the new one sits.
- `display_text::platform` (`crates/goad/tests/renderer/startup.rs`) —
  `slint::PlatformError::from("no display")`. This is the precedent for building
  a **real** platform error; the new cases build theirs the same way and invent
  no fixture.
- `stderr_outlets::report_startup_line_is_the_error_prefixed_with_goad` (same
  file) — the shape for asserting an exact line, verbatim, against a literal.
- `report_platform_line` / `report_platform` (`crates/goad/src/diagnostics.rs`)
  — the pure/impure cut this phase repeats: the `_line` half is asserted, the
  outlet is not (F-7).

*Memory that bears on this phase*

- `docs/memory/wildcard-enum-match-arm-counts-a-named-binding.md` — the
  crate-root `deny` on `lib.rs` reaches `exit.rs`, and it counts a **named**
  binding as a wildcard, not only `_`. Both matches this phase writes are over
  the enclosing `Result`, whose arms are tuple-struct patterns with the binding
  inside — which is exactly the shape that memory prescribes.
- `docs/memory/a-negative-control-that-does-not-compile.md` (also filed as
  `negative-control-must-compile.md`) — **a mutation that does not compile greps
  identically to one that passes.** Every row of the mutation table below
  records that the mutated build compiled.
- `docs/memory/a-count-in-a-comment-is-a-claim-nothing-checks.md` — why
  `lib.rs`'s header is replaced rather than incremented.
- `docs/memory/exit-2-means-two-different-failures.md` — the defect this slice
  exists to repair, and the standing rule that a new `StartupError` variant
  inherits exit 2 and the restart directive with it.
- `docs/memory/a-green-test-can-assert-a-proxy.md` — read before writing
  `every_startup_failure_is_2`: it must assert the number for named variants,
  not a property that would survive the classifier being wrong.

**Assumptions**

Each was verified at plan (`notes.md` §Handover) and is taken on that
verification rather than re-derived — except the first, which is the one thing
this phase is the first to test.

1. **The lints have never run over the real types.** `clippy::pedantic` and
   `clippy::wildcard_enum_match_arm` pass `ended`, `status` and
   `report_exit_line` as `design.md` §5.2 writes them — but over **stand-in
   types**, in a scratch crate, at rounds 5 and 6 of the design review. A
   stand-in is not `slint::PlatformError`. `just check`'s lint pass in this
   phase is the first real measurement. This is why the pure layer is PHASE-01:
   if a lint fires, it fires in a phase that touches nothing else.
2. `slint::PlatformError` is `#[non_exhaustive]`, has `From<String>` and a
   hand-written `Debug`, and has **no** `PartialEq`. So `Ended` cannot derive
   `PartialEq` either, and the `ended` cases match on the variant (`let … else`
   or `matches!`) and compare the carried error by `to_string()`.
3. `crates/goad/tests/renderer/main.rs` does **not** carry the
   `wildcard_enum_match_arm` deny, so the test target is not held by it. Prefer
   the same spelling anyway.
4. `Ended`, `status` and a module named `exit` are unused in the workspace
   today; the only near hit is the test module `exit_codes`.
5. `{:?}` in an **assertion message** is established practice in this target
   (`mapper.rs`, `ingress.rs`, `harness.rs`) and the gate is green with it.
   `clippy::use_debug` is denied workspace-wide all the same, and `clippy.toml`'s
   test carve-outs do not include it — so if it fires, the repair is the
   `to_string()` comparison the plan already prescribes, not an `expect`.

**STOP conditions**

Stop and consult the user. Do not improvise past any of these.

- **A lint fires and the fix is not a spelling.** If `clippy` objects to
  `ended`, `status`, `report_exit_line`'s `Option` arms or `Cancel::is_stopped`,
  the repair must be a different **spelling of the same decision**. If the only
  repair changes a type, an arm's meaning or a signature, that is a design
  change — STOP.
- **`slint::PlatformError` cannot be carried in `Ended`** as `design.md` §5.2
  writes it (a missing trait, an object-safety problem, anything) — STOP.
- **A mutation does not red the cases `design.md` §9 names for it, or reds
  others.** Report it; do **not** add a case to make it red.
- **A criterion compels a file the Surfaces line does not name.** This is the
  class that produced five plan amendments in slice 009. It is a STOP, and the
  amendment is the user's call recorded in `plan-log.md` — not a quiet edit.
- **The plan turns out wrong while expanding it.** `docs/AGENTS.md` §Phase plan:
  go back to plan or design. Do not repair it in this sheet.
- **Budget.** At ~200k tokens: stop, write a `PARTIAL` note here naming what is
  and is not done, leave the tree green, hand back.

**Forbidden.** `git stash`, `git checkout`, `git reset`, `git rebase`,
`git push`. Editing `design.md`, `design-log.md`, `plan.md`, `plan-log.md`,
`canon-delta.md`, `draft-spec.md` or any `review-*.md` — with the **one**
carve-out the Surfaces line names: a renamed case updates `draft-spec.md` §7 and
`design.md` §9 in the same commit, and says so here. Amending canon. Weakening
or deleting a test to go green. `git add <explicit paths>` and `git commit` on
`main` are allowed, with the session trailers.

**Tasks**

- [ ] T-1 — Record the baseline above by re-running nothing: it is measured in
      this sheet at `448f678`. Confirm the tree is still clean and HEAD still
      `448f678` before the first edit.
- [ ] T-2 — `exit.rs` with a **deliberately wrong** `status` and `ended` (e.g.
      `status` answering 0 for everything, `ended` answering `AsAsked` whatever
      it is given), plus `pub mod exit;` in `lib.rs`. This is the red fixture:
      *a red that is only "does not compile" proves the case exists, not that it
      asserts.*
- [ ] T-3 — **VT-1**, `mod exit_status` in `tests/renderer/startup.rs`:
      `as_asked_is_0`, `stopped_running_is_1` (a real `slint::PlatformError`
      through `From<String>`), `stopped_running_with_no_error_is_1`,
      `every_startup_failure_is_2` (**named** representative variants, never
      counted — include `Platform`, which stays 2 in the `Err` channel and is
      the variant this slice takes the loop's ending away from). Watch each red
      → write `status`'s body → green.
- [ ] T-4 — **VT-2**, `mod ended` in the same file:
      `a_loop_error_with_no_stop_requested_is_stopped_running` (asserts the
      carried error is the one given),
      `a_loop_error_after_a_requested_stop_is_as_asked` — both over **one**
      error value — `a_loop_that_returned_ok_with_no_stop_requested_is_stopped_running`
      (carries `None`) and
      `a_loop_that_returned_ok_after_a_requested_stop_is_as_asked`. Red → write
      `ended`'s body → green.
- [ ] T-5 — **VT-4**, `tests::is_stopped_is_false_until_stop_and_stays_true` in
      `wire.rs`'s own test module, beside
      `a_raised_notice_stays_raised_until_it_is_lowered`. Red against a wrong
      body (a constant `false`) → write `*self.rx.borrow()` → green. Document
      `is_stopped` against `Cancel::stopped` so the two cannot be confused.
- [ ] T-6 — **VT-3**, into the existing `stderr_outlets`:
      `report_exit_line_says_nothing_when_the_end_was_as_asked`,
      `report_exit_line_for_a_startup_failure_is_the_startup_line`,
      `a_host_that_stopped_running_says_it_had_been_running`,
      `a_host_that_stopped_running_with_no_error_says_it_had_been_running`,
      `the_stopped_line_is_not_the_line_a_host_that_never_started_writes` (both
      stopped lines against `report_startup_line` over `StartupError::Platform`
      — the one line either could plausibly have been made identical to). Red →
      write `report_exit_line` → green.
- [ ] T-7 — **Refactor.** Not optional. Read the Surfaces back as a reader
      would: do the docs say the rule rather than restate the code, is there a
      duplicated string, does anything in `exit.rs` name `diagnostics`.
- [ ] T-8 — **EX-2's second half**: `lib.rs`'s header counting sentences
      replaced by the rule. Leave the rest of that comment alone and record it
      under §Findings.
- [ ] T-9 — **EX-5**, the mutation evidence. Every row of the table below, each
      restored before the next, each recorded with the quoted edit, the command,
      that the build **compiled**, the cases that redded **by name**, and a green
      restore. Scoped: `cargo test -p goad --test renderer --no-fail-fast`, and
      `cargo test -p goad --lib` for `wire.rs`. `--no-fail-fast` is not optional
      — without it the red set looks thinner than it is.
- [ ] T-10 — **EX-6**: every case name `draft-spec.md` §7 cites that this phase
      owns resolves in the tree, by the name cited. The list is closed: every
      case name in T-3 … T-6. Grep each; a name that does not resolve is either
      a typo or a rename, and a rename is the same-commit obligation above.
- [ ] T-11 — **VA-2**: confirm the vocabulary scan ran over `exit.rs` and the
      new `diagnostics.rs` sentences. It is in `just check`; what needs
      confirming is that the new file is in its reach, not that the command ran.
- [ ] T-12 — **EX-1**: `just check` exits 0. Quote the new gate total **and**
      its workspace denominator, and the two per-target figures, against the
      baseline above.
- [ ] T-13 — Commit. Update §Status (PHASE-01 → `done`), update §Harvest **in
      place**, and leave §Findings and §Mutation evidence complete before
      handing off.

**Mutation evidence**

<!-- One row per mutation, filled as each is run. A mutation that did not
     compile is not evidence. -->

| # | the edit (quoted) | must red, by name | compiled? | redded | restore green? |
|---|---|---|---|---|---|
| M-1 | classifier: `Ok(Ended::StoppedRunning(_)) => 1` → `=> 2` | `stopped_running_is_1`, `stopped_running_with_no_error_is_1` | | | |
| M-2 | classifier: arm split so `Ok(Ended::StoppedRunning(None)) => 0` (F-53's exit 0, moved one function downstream) | `stopped_running_with_no_error_is_1` | | | |
| M-3 | line: `StoppedRunning(Some(_))` answers `report_startup_line`'s sentence over `StartupError::Platform` | `the_stopped_line_is_not_the_line_a_host_that_never_started_writes`, `a_host_that_stopped_running_says_it_had_been_running` | | | |
| M-4 | line: `StoppedRunning(None)` answers a never-started line | `the_stopped_line_is_not_the_line_a_host_that_never_started_writes`, `a_host_that_stopped_running_with_no_error_says_it_had_been_running` | | | |
| M-5 | decision: `AsAsked` for every `Ok` (F-53's wrong shape) | `a_loop_that_returned_ok_with_no_stop_requested_is_stopped_running` | | | |
| M-6 | decision: `StoppedRunning` whatever was requested | `a_loop_error_after_a_requested_stop_is_as_asked`, `a_loop_that_returned_ok_after_a_requested_stop_is_as_asked` | | | |
| M-7 | decision: `AsAsked` whatever was requested | `a_loop_error_with_no_stop_requested_is_stopped_running`, `a_loop_that_returned_ok_with_no_stop_requested_is_stopped_running` | | | |
| M-8 | decision: `None` carried for an `Err` | `a_loop_error_with_no_stop_requested_is_stopped_running` | | | |

**Decisions taken during execution**

<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

**Findings**

- **`lib.rs`'s header carries `path:line` citations that break the *cite by
  symbol* rule** — `fields.rs:2120`, `goad-semantics/src/error.rs:238`,
  `goad-shell/src/ingress/envelope.rs:106` and others. They are **not** this
  slice's to repair: EX-2 replaces only the counting sentences. Raised here for
  audit to disposition (`plan.md` §What no phase does).

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-23 · design review closed, F-1…F-68 `verified` · `8c1fabb` plus this slice's uncommitted documents

### Produced

- `review-design.md` rounds 1–3: F-1…F-34 disposed, repaired and verified.
  Round 4: F-35…F-50 raised, F-51 and F-52 raised by the responder; all
  `verified` in round 5. Rounds 5 and 6: F-53…F-68, disposed with the
  user and `verified` (round 6's by site check). The ledger is the
  artefact; nothing about it is restated here.

### Learned

**Checked during the review and clean — so a later stage does not pay for it
twice.** Each was verified at the symbol by the reviewing agent, which is what
`research.md`'s ✓ means; they are here rather than there because that file is
the scoping evidence base and these are review by-products.

**This list is not exempt from review.** Round 2's F-20 found a symbol here that
does not exist (`tests` for `counting_itself`) inside an entry whose substance
was sound, and F-16 found a surface recorded clean whose stated limit was
narrower than its real one. A *checked and clean* list is read by agents told
not to look again, so a wrong symbol in it costs more than no entry would. Audit
sweeps it at the same standard as the artefacts.

- **`startup::arguments`' doc table stays true.** Its rows say *exit 0* for
  `--help` / `--version` and *exit 2* for `Usage`, and both survive the slice.
  It is **not** a doc site needing repair, unlike `StartupError`'s own.
- **A1's quit wiring is as the design states it.** `Cancel::stop` reaches the
  loop from exactly two places — `window.on_close_requested` and `tray.on_quit`
  (`install`) — and `Wire::stop` is a pass-through to the same signal. No third
  shutdown source. **What drives the first was not checked until round 4
  (F-46)**: in winit 0.30.13 every `CloseRequested` on Linux is a message
  received — `WinitState::request_close` (a Wayland compositor's close),
  `FrameAction::Close` (a client-side decoration's button), `WM_DELETE_WINDOW`
  (X11) — and Slint's only other `request_close` caller is a `.slint` root
  `Window`'s `close()`, which `crates/goad/ui/` never calls on the root. A
  broken connection sends no message; a compositor closing its clients does.
- **No name collisions for what `exit.rs` introduces.** `Ended`, `status` and a
  module named `exit` are each unused in the workspace today; the only near hit
  is the test module `exit_codes`. §8 R5's concern is `controller::Ending`
  alone, and it is real.
- **The vocabulary scan's shape is as `design.md` §3 states it.** `journal` is
  in `DOMAIN`, the scan reads `rs`/`slint` and excludes `tests/`, so the word is
  free in `docs/` and `nix/module.nix` and forbidden in `exit.rs`'s code and
  string literals. **Not in its comments**: `mentions` cuts them off through
  `code_of` before matching, which this entry and `design.md` §3 missed until
  round 5 (F-57).
- **`slint::PlatformError::from("no display")` is already precedent**, in
  `display_text::platform` (`crates/goad/tests/renderer/startup.rs`). The new
  case builds its value the same way rather than inventing a fixture.
- **`structure.rs`'s machinery is sufficient for the new case.**
  `occurrences_where` already takes an arbitrary predicate over `code_of`-stripped
  production lines (`calls_resolve` is the precedent), and the `counting_itself`
  module controls the file's matchers with **string-literal** lines
  (`a_real_call_site_is_counted` is one). Nothing new is needed to write
  `the_loop_s_ending_is_never_a_startup_failure`. **This entry has been wrong
  twice.** Until round 2 it named the module `tests` (F-20); until round 4 it
  called `a_real_call_site_is_counted` a *compiled fixture* (F-45). It is a
  string, `goad-boundary` has one test target and `autotests = false`, and its
  `tests/fixtures/` files are read as text and compiled by nothing. The only
  evidence that the case holds the tree is a mutation of real source.

**The window is constructed at startup and shown only inside the loop.**
`PromptWindow::new` builds a component; the crate's only `window.show()` is in
`SlintGlass::present`, which runs in the loop and only for `Surface::Prompt |
Surface::Diagnostics`. A tray-resident host sits at `Surface::Hidden` and exits
1 having never shown a window. This killed F-11's own proposed repair wording
(*"its window opened"*) and is the durable form of the lesson: **the repair
reaches for an unobserved fact as readily as the defect did.** Any later
sentence about what a running host has been seen to do is checked against this.

### Open

- **The design review is closed** (F-1…F-68 `verified`). Round 5's blocker,
  F-53, changed the design: every end is decided on the request, and
  `Ended::StoppedRunning` carries `Option<slint::PlatformError>`. Round 6
  found one design-level gap in that repair (F-63, a missing case) and prose;
  both were repaired and closed by a site check instead of a round 7.
- **Spiked, not yet run in the tree:** `clippy::wildcard_enum_match_arm` and
  `clippy::pedantic` (the workspace's levels, same clippy 0.1.99 as the repo)
  pass `exit::status`'s match, over stand-in types. Round 6 re-ran it over
  `exit::ended`, `exit::status` and `report_exit_line` as `design.md` §5.2
  writes them, with `clippy::pedantic`, and they pass (stand-ins again). A negative control — a top-level `_ =>` over
  `Ended` — was confirmed red. Stand-ins are not `slint::PlatformError`, so the
  first executing phase still confirms it in the tree; if it fires, the arm
  needs a spelling, not a design change.

- **`Cancel::is_stopped` and `exit::ended` are specified and unbuilt.** The
  name is chosen around `Cancel::stopped`, which is the existing future; a
  phase agent must not conflate them, and the `watch::Receiver` read is
  `*self.rx.borrow()`, as `Notice::raised` already does. `start` binds the loop
  call's result in its own statement and reads `is_stopped` in the next.
- **`research.md` carries a count** — *"the numeral 2 keeps its meaning and its
  five tests"* — which `exit_codes.rs` will falsify the moment a case is added
  there. Not raised as a finding: `research.md` was context to this review and
  not its subject. Sweep it at audit.

## Handover — 2026-09-23, plan accepted, PHASE-01's sheet written

Written for a fresh agent. The slice is **executing**. `plan.md` is accepted at
`448f678` and **no plan review runs** (`plan-log.md`, and what that costs is
recorded there). No code has been written. **PHASE-01's sheet is written**, in
§Phase sheets above, and its EN-1 is discharged by measurement there — the gate
exits 0 at `448f678`, total **615**, `cargo test --workspace` **580**.
PHASE-02's and PHASE-03's sheets do not exist and must not be written ahead of
their phases (`docs/AGENTS.md` §Phase plan: a sheet written three phases early
is fiction).

**What happened at plan.** Verification of `design.md` against the tree at
`b444c6a` passed except for **P-1**: `exit_codes::an_unbindable_ingress_path_exits_2`,
asserting the status alone, was green for any startup failure — measured, a
bindable path exits 2 headlessly at `PromptWindow::new`, and on this machine
(`WAYLAND_DISPLAY` set) the corresponding mutant would launch a real host and
hang the gate. The user took both repairs and a site check over a review round
(`design-log.md`, *P-1, raised at plan*): the case asserts the ingress arm's
stderr prefix, and `process::command` removes the display variables.
`design.md` §5.2 and §9, `draft-spec.md` §7 R-4, `canon-delta.md` Change 1 and
`slice-010.md` §Scope were repaired in `9f0a503`; the site check found no other
sentence claiming the case holds only the status (the closed ledger's wording
is history).

**Verified clean at plan**, so a phase need not re-derive it: every symbol
`design.md` §5 names exists as described — the four `StartupError::Platform`
sites in `start`; `Cancel` holding its own receiver and `Notice::raised` as the
`*self.rx.borrow()` precedent; `install`'s two `stop` routes; `structure.rs`'s
`code_of`, `production_lines`, `occurrences_where`, `calls_resolve`,
`counting_itself`; `display_text::platform`; `report_startup`'s single caller
and its doc sites; `lib.rs`'s counting header; `nix/module.nix`'s directives;
`slint::PlatformError`'s `#[non_exhaustive]`, `From<String>`, a hand-written
`Debug` and no `PartialEq`. The only non-`start` production mentions of
`run_event_loop_until_quit` are doc comments `code_of` strips.
`tests/renderer/main.rs` does **not** carry the `wildcard_enum_match_arm` deny.

**PHASE-01 STOP conditions** (in `plan.md`): `Cancel::is_stopped`,
`exit::ended` and the `Option` arms have never compiled in the tree; a lint
that fires is a spelling, and a fix that changes a type or an arm's meaning is
a STOP. §9's mutations are unrun; each phase owns its share (`plan.md`
§Coverage) and records them under **Mutation evidence** in its sheet.

**For audit, beyond `plan.md` §What no phase does:** `lib.rs`'s header carries
`path:line` citations outside the counting sentence PHASE-01 replaces — not
this slice's, a finding to disposition; `research.md`'s count.

**Next:** PHASE-01 runs, from its sheet. One phase, one agent, one session:
set PHASE-01 to `in progress` in §Status, work the sheet's tasks in order, keep
the sheet current as you go rather than at the end, and end green with §Status,
§Mutation evidence, §Findings and §Harvest all written.

# Notes — Slice 010

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | done | 2026-09-23 |
| PHASE-02 | done | 2026-09-23 |
| PHASE-03 | done | 2026-09-23 |

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

- [x] T-1 — Record the baseline above by re-running nothing: it is measured in
      this sheet at `448f678`. Confirm the tree is still clean and HEAD still
      `448f678` before the first edit. **Done:** tree clean; HEAD is `aeca5a6`,
      which is *documentation only* on top of `448f678` (`notes.md`,
      `plan-log.md`, `slice-010.md` — the commit that wrote this sheet). No
      compiled file differs, so the measured baseline stands and EN-1's *at or
      after the plan's acceptance commit* holds.
- [x] T-2 — `exit.rs` with a **deliberately wrong** `status` and `ended` (e.g.
      `status` answering 0 for everything, `ended` answering `AsAsked` whatever
      it is given), plus `pub mod exit;` in `lib.rs`. This is the red fixture:
      *a red that is only "does not compile" proves the case exists, not that it
      asserts.* **Done:** both bodies were stand-ins (`status` answering 0,
      `ended` answering `AsAsked`) and the crate built green before a case was
      written, so every red below is a failing assertion.
- [x] T-3 — **VT-1**, `mod exit_status` in `tests/renderer/startup.rs`:
      `as_asked_is_0`, `stopped_running_is_1` (a real `slint::PlatformError`
      through `From<String>`), `stopped_running_with_no_error_is_1`,
      `every_startup_failure_is_2` (**named** representative variants, never
      counted — include `Platform`, which stays 2 in the `Err` channel and is
      the variant this slice takes the loop's ending away from). Watch each red
      → write `status`'s body → green.
- [x] T-4 — **VT-2**, `mod ended` in the same file:
      `a_loop_error_with_no_stop_requested_is_stopped_running` (asserts the
      carried error is the one given),
      `a_loop_error_after_a_requested_stop_is_as_asked` — both over **one**
      error value — `a_loop_that_returned_ok_with_no_stop_requested_is_stopped_running`
      (carries `None`) and
      `a_loop_that_returned_ok_after_a_requested_stop_is_as_asked`. Red → write
      `ended`'s body → green. **Done, and with two stand-ins rather than one.**
      The sheet's stand-in (`AsAsked` whatever it is given) reds only the two
      `…is_stopped_running` cases; the two `…is_as_asked` cases expect `AsAsked`
      and pass against it. So a second stand-in — `StoppedRunning(call.err())`
      whatever was requested — was run before the real body, and redded exactly
      the other two. Every case in the module has now been seen to fail an
      assertion. Both stand-ins are the shapes M-7 and M-6 mutate to, so this
      cost one extra build and measured the same thing twice over.
- [x] T-5 — **VT-4**, `tests::is_stopped_is_false_until_stop_and_stays_true` in
      `wire.rs`'s own test module, beside
      `a_raised_notice_stays_raised_until_it_is_lowered`. Red against a wrong
      body (a constant `false`) → write `*self.rx.borrow()` → green. Document
      `is_stopped` against `Cancel::stopped` so the two cannot be confused.
      **Done**, with both stand-ins as in T-4: a constant `false` reds
      `assert!(cancel.is_stopped())` after `stop`, a constant `true` reds
      *a fresh signal has not been tripped*. The case sits with the other
      `Cancel` cases and **before** the `// ---- the back-pressure signal ----`
      divider, not literally beside
      `a_raised_notice_stays_raised_until_it_is_lowered`, which is on the far
      side of that divider — see §Decisions taken during execution.
- [x] T-6 — **VT-3**, into the existing `stderr_outlets`:
      `report_exit_line_says_nothing_when_the_end_was_as_asked`,
      `report_exit_line_for_a_startup_failure_is_the_startup_line`,
      `a_host_that_stopped_running_says_it_had_been_running`,
      `a_host_that_stopped_running_with_no_error_says_it_had_been_running`,
      `the_stopped_line_is_not_the_line_a_host_that_never_started_writes` (both
      stopped lines against `report_startup_line` over `StartupError::Platform`
      — the one line either could plausibly have been made identical to). Red →
      write `report_exit_line` → green. **Done**, two stand-ins again: a
      constant unrelated line reds the four that pin a value, and a stand-in
      answering `report_startup_line` over `StartupError::Platform("no
      display")` for every end — M-3 and M-4 at once — reds the fifth, the
      distinctness case, which the first stand-in cannot. `renderer` is **221**
      after this (208 + 13: 4 + 4 + 5).
- [x] T-7 — **Refactor.** Not optional. Read the Surfaces back as a reader
      would: do the docs say the rule rather than restate the code, is there a
      duplicated string, does anything in `exit.rs` name `diagnostics`.
      **Done.** Three repairs, all to prose I had written (the design's own doc
      comments are unchanged): `exit.rs`'s module doc first claimed the module
      *never names* `diagnostics`, which `status`'s own doc — the design's
      wording — falsifies by naming `report_exit_line` and `report_exit`; then
      claimed the dependency *may not be added in either direction*, which is
      false the other way, since `diagnostics` imports `Ended`. It now states
      the one-way rule and says **imports**, which is the invariant. No
      duplicated string in the sources: the two stopped sentences exist once
      each in `diagnostics.rs`, and the tests pin them as
      `report_startup_line`'s cases already pin that one. No `DOMAIN` word in
      any of the three files.
- [x] T-8 — **EX-2's second half**: `lib.rs`'s header counting sentences
      replaced by the rule. Leave the rest of that comment alone and record it
      under §Findings. **Done:** the whole run from *"One `pub mod` line per
      phase; ten at PHASE-08 …"* to *"… and `pending`, which holds the
      debounce."* is replaced by the rule and why there is no number. The rest
      of the header — the `wildcard_enum_match_arm` argument and its `path:line`
      citations — is untouched; the finding stands below.
- [x] T-9 — **EX-5**, the mutation evidence. Every row of the table below, each
      restored before the next, each recorded with the quoted edit, the command,
      that the build **compiled**, the cases that redded **by name**, and a green
      restore. Scoped: `cargo test -p goad --test renderer --no-fail-fast`, and
      `cargo test -p goad --lib` for `wire.rs`. `--no-fail-fast` is not optional
      — without it the red set looks thinner than it is.
- [x] T-10 — **EX-6**: every case name `draft-spec.md` §7 cites that this phase
      owns resolves in the tree, by the name cited. The list is closed: every
      case name in T-3 … T-6. Grep each; a name that does not resolve is either
      a typo or a rename, and a rename is the same-commit obligation above.
- [x] T-11 — **VA-2**: confirm the vocabulary scan ran over `exit.rs` and the
      new `diagnostics.rs` sentences. It is in `just check`; what needs
      confirming is that the new file is in its reach, not that the command ran.
- [x] T-12 — **EX-1**: `just check` exits 0. Quote the new gate total **and**
      its workspace denominator, and the two per-target figures, against the
      baseline above. **Done: `just check` exits 0.** Gate total **629**;
      `cargo test --workspace` **594**. The 35 the gate counts twice is
      `goad-semantics`' 30 + 5, unchanged, so the gate total is exactly 35 above
      the workspace one as it was at the baseline. Against `448f678`: workspace
      580 → 594, **+14**, which is the whole of what this phase adds and no more
      — `goad` lib 58 → **59** (`is_stopped_is_false_until_stop_and_stays_true`)
      and `goad` `tests/renderer` 208 → **221** (4 `exit_status` + 4 `ended` +
      5 `stderr_outlets`). Every doc-test target is still 0.
- [x] T-13 — Commit. Update §Status (PHASE-01 → `done`), update §Harvest **in
      place**, and leave §Findings and §Mutation evidence complete before
      handing off.

**Mutation evidence**

<!-- One row per mutation, filled as each is run. A mutation that did not
     compile is not evidence. -->

| # | the edit (quoted) | must red, by name | compiled? | redded | restore green? |
|---|---|---|---|---|---|
| M-1 | `exit::status`: `Ok(Ended::StoppedRunning(_)) => 1,` → `=> 2,` | `stopped_running_is_1`, `stopped_running_with_no_error_is_1` | yes | exactly those two, and nothing else: `219 passed; 2 failed` | yes |
| M-2 | `exit::status`: arm split — `Ok(Ended::StoppedRunning(None)) => 0,` above `Ok(Ended::StoppedRunning(Some(_))) => 1,` | `stopped_running_with_no_error_is_1` | yes | exactly that one: `220 passed; 1 failed` | yes |
| M-3 | `report_exit_line`: `Ok(Ended::StoppedRunning(Some(error))) => Some(report_startup_line(&StartupError::Platform(slint::PlatformError::from(error.to_string()))))` | `the_stopped_line_is_not_the_line_a_host_that_never_started_writes`, `a_host_that_stopped_running_says_it_had_been_running` | yes | exactly those two: `219 passed; 2 failed` | yes |
| M-4 | `report_exit_line`: `Ok(Ended::StoppedRunning(None)) => Some(report_startup_line(&StartupError::Platform(slint::PlatformError::from("no display"))))` | `the_stopped_line_is_not_the_line_a_host_that_never_started_writes`, `a_host_that_stopped_running_with_no_error_says_it_had_been_running` | yes | exactly those two: `219 passed; 2 failed` | yes |
| M-5 | `exit::ended`: `if stop_requested \|\| call.is_ok() {` — `AsAsked` for every `Ok` (F-53's shape) | `a_loop_that_returned_ok_with_no_stop_requested_is_stopped_running` | yes | exactly that one: `220 passed; 1 failed` | yes |
| M-6 | `exit::ended`: body `let _ = stop_requested; Ended::StoppedRunning(call.err())` | `a_loop_error_after_a_requested_stop_is_as_asked`, `a_loop_that_returned_ok_after_a_requested_stop_is_as_asked` | yes | exactly those two: `219 passed; 2 failed` | yes |
| M-7 | `exit::ended`: body `let _ = (call, stop_requested); Ended::AsAsked` | `a_loop_error_with_no_stop_requested_is_stopped_running`, `a_loop_that_returned_ok_with_no_stop_requested_is_stopped_running` | yes | exactly those two: `219 passed; 2 failed` | yes |
| M-8 | `exit::ended`: the `else` arm becomes `let _ = call; Ended::StoppedRunning(None)` | `a_loop_error_with_no_stop_requested_is_stopped_running` | yes | exactly that one: `220 passed; 1 failed` | yes |

**How each row was measured.** One mutation at a time, applied to a tree
restored from a byte copy of `exit.rs` and `diagnostics.rs` taken before the
first; `cargo test -p goad --test renderer --no-fail-fast`; the restore
verified by `diff` against that copy. **compiled?** is read from the run
producing a `test result:` line at all and no `error[E…]` — a mutation that did
not build would be no evidence
(`docs/memory/negative-control-must-compile.md`). The green denominator is
**221**, so a row reading `219 passed; 2 failed` accounts for every case in the
target. No mutation redded a case its row does not name. `cargo test -p goad
--lib` was not needed: the table mutates `exit.rs` and `diagnostics.rs` only,
and `wire.rs`'s case is covered by T-5's own two stand-ins.

**Decisions taken during execution**

<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **Where `is_stopped_is_false_until_stop_and_stays_true` sits in `wire.rs`.**
  `design.md` §9 locates it *"beside
  `a_raised_notice_stays_raised_until_it_is_lowered`"*. That case is on the far
  side of the file's own `// ---- the back-pressure signal ----` divider, and
  putting a `Cancel` case after that divider would make the divider false. The
  case sits in the same `#[cfg(test)] mod tests`, with the other `Cancel` cases
  (`stopped_resolves_immediately_when_already_tripped`,
  `stopped_does_not_resolve_until_stop_is_called`) and immediately before the
  divider. The named case is still its shape template, which is what §9 was
  citing it for. No rename, so no `draft-spec.md` / `design.md` edit is owed.

**Findings**

- **`clippy::wildcard_enum_match_arm` does not reach `exit::status`'s match,
  and the gate therefore holds its exhaustiveness by nothing.** Measured here,
  by negative control: replacing the last two arms with `_ => 1` leaves
  `cargo clippy -p goad --all-targets -- -D warnings` **green** (the mutated
  build compiled — checked). Two probes in the same file say why: a wildcard
  over `&Ended` fires `match_wildcard_for_single_variants`, but a wildcard over
  `&Result<Ended, StartupError>` beneath an `Ok(Ended::AsAsked)` arm fires
  neither lint. This is the flip side of
  `docs/memory/wildcard-enum-match-arm-counts-a-named-binding.md`, which is
  cited in this sheet's §Assumptions as the reason to match on the enclosing
  `Result`: escaping the lint is exactly what that shape does, and the memory
  records the escape as a remedy without recording what it costs. **Not a STOP
  and not a defect** — no lint fired, and the code is written as `design.md`
  §5.2 gives it, without a wildcard. What is wrong is the *claim*: `design.md`
  §3's *"Every match this design adds is written without a wildcard over an
  enum"* is true of the code and reads as though the deny were what keeps it
  true. For `status` it is not; what keeps it true is `exit_status`'s four
  cases and M-1/M-2. For audit to disposition — the candidate repairs are a
  sentence in §3 and a line in that memory file, and neither is this phase's.

- **`lib.rs`'s header carries `path:line` citations that break the *cite by
  symbol* rule** — `fields.rs:2120`, `goad-semantics/src/error.rs:238`,
  `goad-shell/src/ingress/envelope.rs:106` and others. They are **not** this
  slice's to repair: EX-2 replaces only the counting sentences. Raised here for
  audit to disposition (`plan.md` §What no phase does).

### PHASE-02 — the seam

**Objective:** `main` has no branch: `run` answers `Result<Ended,
StartupError>`, the loop's end reaches `exit::ended` with a read of the stop
signal taken after the call, `StartupError` no longer carries the loop's end,
and a gate case holds the call's line.

Written by the orchestrator, not the executor (`notes.md` §Handover; the 010
split): with no plan review, this sheet is the plan's only adversarial reading
before code, so it is written by someone who will not execute it.

**Surfaces** (`plan.md` PHASE-02; anything outside this list is a STOP):
`crates/goad/src/main.rs`, `crates/goad/src/startup.rs` (**docs only**),
`crates/goad/src/diagnostics.rs` (`report_exit` in, `report_startup` out,
`report_startup_line`'s doc, the module's `//!` doc — nothing else),
`crates/goad/tests/renderer/startup.rs` (**module doc only**),
`crates/goad-boundary/tests/checks/structure.rs`. `draft-spec.md` §7 and
`design.md` §9 **only** if a case is renamed, and then in the same commit.

**Entry criteria, verified rather than assumed**

- **EN-1 — discharged by the orchestrator at `e7aca88`**, before this sheet was
  written:
  - *PHASE-01 `done` in §Status* — yes, 2026-09-23.
  - *its EX criteria hold on HEAD* — `just check` **exit 0**; gate total
    **629**, `cargo test --workspace` **594** (the 35 counted twice is
    `goad-semantics`' 30 + 5). `exit::{Ended, ended, status}`,
    `Cancel::is_stopped` and `diagnostics::report_exit_line` exist.
    HEAD `e7aca88` is documentation only on top of PHASE-01's `ab5604f`.

**Baseline for the targets this phase touches**, from that run: `goad`
`tests/renderer` **221**, `goad` `tests/binary` **6** (`exit_codes` 5 +
`version` 1), `goad-boundary` `tests/checks` **43**. This phase adds three
cases to `checks` (one in the file's top level, two in `counting_itself`), so
the expected end state is `checks` **46**, workspace **597**, gate **632** —
and every other figure unchanged. A figure that differs is a finding.

**Reading list** — by symbol, never `path:line`.

*What is being written*

- `crates/goad/src/main.rs`
  - `main` becomes `design.md` §5.2's three lines, exactly:
    `let outcome = run();`, `diagnostics::report_exit(&outcome);`,
    `ExitCode::from(exit::status(&outcome))`. Imports gain `goad::exit::{self,
    Ended}` (or the spelling rustfmt/clippy settle on).
  - `run` → `Result<Ended, StartupError>`; the `Help` and `Version` arms answer
    `Ok(Ended::AsAsked)`. `run`'s doc (*"This is the fallible half, and it is
    the only place a `StartupError` is produced"*) must still be true after the
    change — re-read it; `start` produces them too, as it does today.
  - `start` → `Result<Ended, StartupError>`. In step 6, **immediately after
    `let cancel = Cancel::new();`** and before `cancel` moves into `serve`:
    `let stop_signal = cancel.clone();`, with a comment in the file's voice
    saying why (the `pending` / `Rc::clone(&pending)` comment in step 6–7 is
    the precedent for "a clone kept back for a reason"). The last two
    statements become:
    ```rust
    let call = slint::run_event_loop_until_quit();
    Ok(exit::ended(call, stop_signal.is_stopped()))
    ```
    The call written **by its path**; no `use slint::run_event_loop_until_quit`.
    Two statements, not one: the read comes after the call on the page, not by
    argument-evaluation order (`design.md` §5.2). Step 9's comment and any
    comment that says the loop's error is a startup failure are repaired.
- `crates/goad/src/diagnostics.rs`
  - `report_startup` **removed**. `report_exit` in its place, doc *"stderr,
    once, last."* (`design.md` §5.2), body writing `report_exit_line`'s line,
    if any, through `line_to(std::io::stderr().lock(), …)` — as the outlet it
    replaces does.
  - `report_startup_line`'s doc opens *"The exact string `report_startup`
    writes"* — re-anchor it to `report_exit` (F-34). Its name, signature and
    text do not change: the binary tier names that text and AC-5 forbids
    touching those cases.
  - The module `//!` doc names `report_startup` among PHASE-08's outlets. It
    must name `report_exit` instead **and stay true as history** — `report_exit`
    was not added at PHASE-08. The sentence is the executor's to spell; the
    constraint is that no sentence in it is false. It also counts (*"two
    outlets"*, *"a third"*); `CLAUDE.md` *name, never count* applies to any
    sentence you rewrite. Do not rewrite sentences you are not otherwise
    touching.
- `crates/goad/src/startup.rs` — **docs only**, no signature or text change:
  - `Launch`'s doc: *"so `main` keeps its single exit-code decision"* — say
    where that decision now is (`exit::status`).
  - `StartupError`'s type doc: *every way `run` can fail to reach the event
    loop* becomes true; add a sentence saying where the loop's ending went
    (`Ended`, via `exit::ended`).
  - The paragraph *"Every variant is exit **2**: `main` has one `match` over
    `run`'s `Result`…"* is replaced by the rule: the number is `exit::status`'s
    single `Err` arm, which reads no variant; what 2 means is the spec's — **no
    spec number** (D6). Whether `nix/module.nix` depending on the numeral and
    `tests/binary/exit_codes.rs` holding it stay is your call; they are still
    true.
  - `Platform`'s variant doc names `set_xdg_app_id`, `PromptWindow::new` and
    `Tray::new`, and **not** `run_event_loop_until_quit`.
- `crates/goad/tests/renderer/startup.rs` — **module doc only.** It says *"No
  test here runs the binary or asserts an exit code"* and that `main`'s one
  `match` chooses the code. After this phase the numbers are a pure function's
  answers and this tier holds them; the binary tier holds that the **process**
  answers them to a caller (EX-5). No sentence may say this tier asserts no exit
  code. It also says *"the two stderr outlets' exact strings"* — PHASE-01 made
  that three `_line` functions; name them, don't count them.
- `crates/goad-boundary/tests/checks/structure.rs`
  - A named predicate **`ends_at_the_loop_call(code: &str) -> bool`**, beside
    `calls_resolve`: the line's `code_of`-stripped text, trimmed, ends
    `run_event_loop_until_quit();`. Documented as `calls_resolve` is.
  - **`the_loop_s_ending_is_never_a_startup_failure`**, top level, beside
    `quit_event_loop_has_exactly_one_call_site`: exactly one production line of
    `SUBJECT_DIR` names `run_event_loop_until_quit`, and that line satisfies
    `ends_at_the_loop_call`. Build it on `occurrences_where` /
    `occurrences_of` — **no second walk**. One spelling that needs no change to
    `Occurrence`: assert `occurrences_of(SUBJECT_DIR, "run_event_loop_until_quit")`
    has length 1, and `occurrences_where(SUBJECT_DIR, |code| code.contains(…) &&
    !ends_at_the_loop_call(code))` is empty — each with `report(&found)` in the
    message. Its **doc states what it does not reach** (`design.md` §5.2, *What
    the case does not reach*, F-55): a re-filing written off the call's line
    that does not name the function — of `call`, or of the `Ended` — in
    `start`, `run` or `main`; that is review. The vocabulary scan's doc is the
    precedent for a scan saying its own limit.
  - In `counting_itself`: **`the_bare_loop_call_ends_at_the_call`** (the
    string `    let call = slint::run_event_loop_until_quit();` passes) and
    **`a_loop_call_with_its_result_re_filed_does_not`** (the call followed by
    `.map_err(StartupError::Platform)?;`, and by `?;` alone, both fail). Add
    `ends_at_the_loop_call` to that module's `use super::{…}`.
  - The file's `//!` doc is **not** an inventory of its cases; do not make it
    one.

*Design sections that bind*

- **§5.1** — *The decision is a pure function, and `start` only feeds it*: the
  clone before `serve`, the call bound, **then** the read; every route that
  trips `Cancel` is a callback the loop runs, so once the call returns the value
  is final.
- **§5.2** — the `main.rs` block, the `diagnostics.rs` block and the paragraph
  under it (F-34, the `//!` doc), the `startup.rs` list, the `structure.rs`
  paragraphs through *What the case does not reach*, and *The two-tier cut
  moves*.
- **§9** — the `structure` and `counting_itself` rows, and the three **scan**
  mutations.
- **§3** — the vocabulary scan reads `crates/**/*.rs` outside `tests/`; comments
  are stripped before matching. `journal` must not appear in code or string
  literals in `main.rs` or `diagnostics.rs`.
- **§7 D6** — no spec number in any doc this phase writes.
- **`draft-spec.md` §7**, R-1's and R-2's rows — the working canon for VA-1.
  **Test names are commitments.**

*Prior art — copy these rather than inventing*

- `quit_event_loop_has_exactly_one_call_site` and
  `slint_spawn_local_is_the_one_spawn_this_crate_uses` — the count-of-one shape
  and its failure message.
- `calls_resolve` and `the_call_matcher_counts_calls_and_nothing_else` — a
  named line predicate beside the matchers, and its string controls in
  `counting_itself`.
- `report_platform` / `report_platform_line` — outlet over pure half.
- `pending` in `start` (step 6, and `Rc::clone(&pending)` in step 7) — a handle
  kept back from a move for a stated reason.

*Memory that bears on this phase*

- `docs/memory/negative-control-must-compile.md` — every scan mutation is
  recorded as having compiled. §9 spells the two re-filings so they compile;
  the obvious spelling (`let call = …map_err(…)?;`) makes `call` a `()` and
  does not.
- `a-standing-guard-may-not-reach-a-new-file` (the orchestrator's store, lifted
  at close) — the scan case is a **new instrument**. Its string controls prove
  the predicate reads a line; only the mutations, against real source, prove it
  reaches the tree.
- `docs/memory/a-green-test-can-assert-a-proxy.md` — the case asserts the
  **shape**, not a needle; a case that only counted `map_err` would be a proxy.

**Assumptions**

Verified at plan or by the orchestrator at `e7aca88`; not re-derived.

1. The only production line of `crates/goad/src` naming
   `run_event_loop_until_quit` is `start`'s last-but-one statement; the doc
   comments in `startup.rs` (`Platform`'s) and `controller.rs` (`Ending::Closed`)
   that name it are stripped by `code_of`. `controller.rs`'s sentence stays true
   after this phase (`tx` still outlives the call) and is **not** a surface.
2. `exit.rs` and `main.rs` have no `#[cfg(test)]` item, so `production_lines`
   reads them whole.
3. `report_startup` has exactly one caller (`main`) and no reference outside
   `crates/goad/src` except in slice folders' history.
4. `Cancel: Clone`, and `Cancel::is_stopped` is `&self → bool`.
5. The workspace denies `unused` except `dead_code = "warn"` (root
   `Cargo.toml` `[lints]`). So a dead function compiles under `cargo test` (a
   warning, not an error), and `unused_must_use` is denied — the second-call
   mutation must bind the result (`let _ = …;` or `let _call = …;`).
6. **The one thing this phase is first to test:** that the rewired `main`
   leaves every binary-tier case green **unmodified** (VT-3). No case there
   reaches the event loop; if one reds, it is not a wording problem — STOP.

**Known staleness outside the surfaces** — do **not** edit; they are other
phases' or close's, and listed so you do not mistake them for misses:
`nix/module.nix`'s comment quoting the old call line and
`tests/binary/exit_codes.rs`'s / `tests/binary/main.rs`'s module docs
(PHASE-03); `docs/memory/exit-2-means-two-different-failures.md` quoting the
old `match` and call line (close, from §Harvest — add it there).

**STOP conditions** — stop and consult; do not improvise past any.

- A criterion compels a file outside **Surfaces**.
- A binary-tier case reds against the rewired `main` (Assumption 6).
- A scan mutation does not compile in the §9 spelling, or does not red
  `the_loop_s_ending_is_never_a_startup_failure`, or reds **anything else**
  across the workspace.
- A lint fires on `main`, `run`, `start` or `report_exit` and the only fix
  changes a type, a signature or an arm's meaning.
- The plan turns out wrong while executing. Go back; do not repair it here.
- **Budget.** At ~200k tokens: stop at a green point, write `PARTIAL` here
  naming what is and is not done, hand back.

**Forbidden.** `git stash`, `git checkout`, `git reset`, `git rebase`,
`git commit --amend`, `git push`. Editing `design.md`, `design-log.md`,
`plan.md`, `plan-log.md`, `canon-delta.md`, `draft-spec.md` or any
`review-*.md` — except the renamed-case carve-out above. Amending canon.
Weakening or deleting a test to go green. `git add <explicit paths>` (never
`-A`) and `git commit` on `main` are allowed, with the session trailers. You
are the **only** writer on this tree while you run.

**Tasks**

- [x] T-1 — `git log --oneline -1` is `e7aca88` or a documentation-only
      descendant of it that adds this sheet; tree clean. Record it.

      `abdbba7` — "010: PHASE-02's sheet is written, EN-1 measured at
      e7aca88", a documentation-only descendant of `e7aca88` adding this
      sheet. Tree clean.
- [x] T-2 — **VT-1, the slice's one natural red.** Write
      `ends_at_the_loop_call` and `the_loop_s_ending_is_never_a_startup_failure`
      against today's tree. Run `cargo test -p goad-boundary --test checks
      the_loop_s_ending --no-fail-fast`; it must **red on an assertion**, naming
      `crates/goad/src/main.rs` and the `.map_err(StartupError::Platform)?;`
      line. Quote the failure here.

      Redded as expected, on the emptiness assertion (`structure.rs:310` at
      the time of the run):
      ```
      thread 'structure::the_loop_s_ending_is_never_a_startup_failure' panicked at crates/goad-boundary/tests/checks/structure.rs:310:3:
      found:
      /home/david/dev/goad/crates/goad-boundary/../../crates/goad/src/main.rs:189
      ```
      `main.rs:189` is `slint::run_event_loop_until_quit().map_err(StartupError::Platform)?;`
      — the count assertion passed (one occurrence), the shape assertion
      failed on it.
- [x] T-3 — **The seam** (`main.rs`) and `report_exit` in / `report_startup`
      out (`diagnostics.rs`), in one movement. `cargo build -p goad`; then T-2's
      command **green**. Quote it.

      `cargo build -p goad` — clean. `cargo test -p goad-boundary --test
      checks the_loop_s_ending --no-fail-fast`:
      ```
      test structure::the_loop_s_ending_is_never_a_startup_failure ... ok
      test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out
      ```
- [x] T-4 — **VT-3.** `cargo test -p goad --test binary --no-fail-fast`: all
      **6** green, `git diff` over `crates/goad/tests/binary/` empty.

      All 6 (`exit_codes`'s 5 + `version`'s 1) green, unmodified. Assumption 6
      holds: the rewired `main` leaves the binary tier untouched.
- [x] T-5 — **VT-2.** The two `counting_itself` controls. Red them first
      against a deliberately wrong `ends_at_the_loop_call` (e.g. `|_| true`
      reds the second, `|_| false` the first); then the real body.

      `|_| true`: `the_bare_loop_call_ends_at_the_call` ok,
      `a_loop_call_with_its_result_re_filed_does_not` FAILED (as predicted).
      `|_| false`: the first FAILED, the second ok (as predicted). Restored
      body byte-identical to T-2's (`diff` clean against the scratchpad
      copy); `cargo test -p goad-boundary --test checks structure
      --no-fail-fast`: **22** passed, 0 failed, 24 filtered out.
- [x] T-6 — **Docs** (EX-3, EX-4, EX-5): `startup.rs`'s four sites,
      `diagnostics.rs`'s two, `renderer/startup.rs`'s module doc, and every
      comment in `main.rs` the new shape made false.

      `startup.rs`: `Launch`'s doc now says the decision stays in
      `exit::status`; `StartupError`'s type doc adds where the loop's ending
      went (`Ended`, via `exit::ended`) and its `Err`-arm-number paragraph is
      replaced (no spec number, D6); `Platform`'s variant doc drops
      `run_event_loop_until_quit`. `diagnostics.rs`: `report_startup_line`'s
      doc re-anchored to `report_exit`; the module `//!` doc names
      `report_exit` and stays true as history ("the impure outlet renamed
      `report_exit` at 010/PHASE-02", with no counting word and — per T-10's
      grep — no literal occurrence of the removed name inside `crates/`).
      `renderer/startup.rs`'s module doc: names each `_line`
      function rather than counting them, and states the moved cut — this
      tier now holds the numbers, the binary tier holds that the process
      answers them to a caller. `main.rs`: step 6's new clone has its own
      comment; step 9's comment stands (never claimed the loop's error was a
      startup failure); the final block's comment states the seam.
- [x] T-7 — **Refactor.** Not optional. Read each surface back as a stranger
      would: does any doc still say the loop's error is a startup failure, that
      `main` matches, or count something; is `ends_at_the_loop_call` the only
      new matcher (no parallel walk); does `main.rs` still read top to bottom.

      Re-read all five surfaces. No remaining sentence claims the loop's
      error is a startup failure, or that `main` matches over anything (it
      has no `match`); the two doc sentences rewritten for this phase
      (`startup.rs`'s `Launch` doc, `diagnostics.rs`'s module `//!`) carry no
      counting word. `ends_at_the_loop_call` is the only new matcher, used
      through the existing `occurrences_where`/`occurrences_of` — no second
      walk. `main.rs` reads top to bottom: `main`, `run`, `start`, its nine
      numbered steps unchanged in shape, step 6's clone and the closing seam
      each carrying its own comment. Tidied two doc-comment line wraps left
      ragged by the edit (no content change). `cargo fmt --all -- --check`
      clean; `cargo clippy -p goad -p goad-boundary --all-targets -- -D
      warnings`: 0 warnings.
- [x] T-8 — **EX-7, the scan mutations.** Each applied alone to a tree restored
      from a byte copy of `main.rs` (scratchpad, never `git checkout`), run
      with `cargo test --workspace --no-fail-fast`, recorded below, restored and
      `diff`ed against the copy. Fill in *compiled?* and *redded*; the red set
      must be **exactly** the one case.
- [x] T-9 — **VA-1**, the review no test reaches. Record each, by symbol, under
      §VA-1 below: (a) `is_stopped` is read on `stop_signal`, a clone of the
      `Cancel` handed to `serve`, in the statement after the call's own; (b)
      `exit::ended` receives the call's own result, unmapped; (c) no site in
      `crates/goad/src` other than `exit::ended` constructs
      `Ended::StoppedRunning` — `grep -rn 'StoppedRunning(' crates/goad/src`,
      quoted, and each hit classified (construction vs pattern); (d) nothing
      downstream of `exit::ended` turns an `Ended` into an `Err`. See §VA-1.
- [x] T-10 — **VA-2.** `grep -rn "report_startup\b" crates docs/slices/010`,
      quoted; every hit outside `crates/` is slice history, and there is none
      in `crates/`.

      First pass had one hit in `crates/`: T-6's `diagnostics.rs` module doc
      named the removed identifier literally ("born `report_startup`") to
      keep the sentence historically true. Reworded to state the same fact —
      PHASE-08 added an outlet later renamed `report_exit` — without the
      literal name (§Decisions). Re-grepped: every remaining hit is under
      `docs/slices/010/` (`design.md`, `design-log.md`, `plan.md`,
      `review-design.md`, `slice-010.md`, this file) — the slice's own
      history. Zero in `crates/`. Rebuilt and reran the full workspace suite
      after the edit: unchanged, all green.
- [x] T-11 — **EX-8.** Every case `draft-spec.md` §7 cites that this phase owns
      resolves by the name cited — the closed list is the three case names in
      T-2 and T-5. Grep each.

      Grepped each of the three (`the_loop_s_ending_is_never_a_startup_failure`,
      `the_bare_loop_call_ends_at_the_call`,
      `a_loop_call_with_its_result_re_filed_does_not`) against `draft-spec.md`.
      Only the first is cited (§7, R-2's row, as
      `structure::the_loop_s_ending_is_never_a_startup_failure`), and it
      resolves exactly to `structure.rs:303`'s `fn
      the_loop_s_ending_is_never_a_startup_failure`. The other two are not
      cited in §7, so there is nothing stale for them to be.
- [x] T-12 — **EX-1.** `just check` exits 0. Quote the gate total **and** the
      workspace denominator, and `checks` / `renderer` / `binary`, against the
      baseline above.

      `just check` — **exit 0**. `cargo test --workspace` sums to **597**
      (unchanged from baseline); `cargo test -p goad-semantics`'s standalone
      step adds its 35 (30 + 5) again for a gate total of **632**
      (baseline 629 + the 3 new `checks` cases). Against the baseline:
      `checks` **46** (was 43, +3 — the `the_loop_s_ending_is_never_a_startup_failure`
      top-level case and the two `counting_itself` controls), `renderer`
      **221** (unchanged), `goad`'s `binary` **6** (unchanged). `deno check`,
      `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt
      --all --check` each produced no output — clean.
- [ ] T-13 — Commit (`010: PHASE-02 — …`). §Status PHASE-02 → `done`.
      §Harvest updated **in place** (add the stale memory file under what close
      must lift). §Findings, §Decisions and §Mutation evidence complete. Then go
      idle — make no commit after reporting.

**Mutation evidence**

| # | the edit (quoted) | must red, by name | compiled? | redded (workspace, `--no-fail-fast`) | restore green? |
|---|---|---|---|---|---|
| M-9 | `start`: `let call = Ok(slint::run_event_loop_until_quit().map_err(StartupError::Platform)?);` | `the_loop_s_ending_is_never_a_startup_failure` only | yes | exactly `the_loop_s_ending_is_never_a_startup_failure` | yes — restored from scratchpad byte copy, `diff` clean |
| M-10 | `main.rs`: add `use goad::startup::StartupError::Platform;`; `start`: `let call = Ok(slint::run_event_loop_until_quit().map_err(Platform)?);` | `the_loop_s_ending_is_never_a_startup_failure` only | yes | exactly `the_loop_s_ending_is_never_a_startup_failure` | yes — restored from scratchpad byte copy, `diff` clean |
| M-11 | `main.rs`: a second, real production call, **over several lines so its own line ends at the call** — this row isolates the count half — e.g. `#[allow(dead_code)]` / `fn loop_again() {` / `  let _ = slint::run_event_loop_until_quit();` / `}` | `the_loop_s_ending_is_never_a_startup_failure` only, on the **count** assertion | yes | exactly `the_loop_s_ending_is_never_a_startup_failure`, on the count assertion (`left: 2, right: 1`, naming `main.rs:197` and `main.rs:203`) | yes — restored from scratchpad byte copy, `diff` clean |

**VA-1**

<!-- (a)–(d) from T-9, each with the symbol it was checked against. -->

(a) `main.rs`'s `start`: `stop_signal` is bound at step 6 (`let stop_signal =
cancel.clone();`, immediately after `Cancel::new()` and before `cancel` moves
into the step-9 `spawn_local` closure that hands it to `serve`), and read at
`stop_signal.is_stopped()` in the statement immediately after `let call =
slint::run_event_loop_until_quit();` — after the call, not before it and not
by argument-evaluation order.

(b) The same statement: `Ok(exit::ended(call, stop_signal.is_stopped()))` —
`call` is bound on the line above with no `.map_err`, no `?`, nothing applied
to it; it is `exit::ended`'s first argument unmapped.

(c) `grep -rn 'StoppedRunning(' crates/goad/src`:
```
crates/goad/src/exit.rs:46:  StoppedRunning(Option<slint::PlatformError>),
crates/goad/src/exit.rs:61:    Ended::StoppedRunning(call.err())
crates/goad/src/exit.rs:75:    Ok(Ended::StoppedRunning(_)) => 1,
crates/goad/src/diagnostics.rs:453:    Ok(Ended::StoppedRunning(Some(error))) => {
crates/goad/src/diagnostics.rs:456:    Ok(Ended::StoppedRunning(None)) => {
```
Classified: `exit.rs:46` is the variant's own declaration (neither
construction nor pattern — the type itself); `exit.rs:61`, inside
`exit::ended`, is the one **construction**; `exit.rs:75`
(`exit::status`), `diagnostics.rs:453` and `diagnostics.rs:456`
(`report_exit_line`) are all **patterns**, matching on a value already
built. No construction site exists outside `exit::ended`.

(d) Every downstream reader of an `Ended` matches on `&Result<Ended,
StartupError>` and answers a plain value, never an `Err`: `exit::status`
(`Ok(Ended::AsAsked) => 0`, `Ok(Ended::StoppedRunning(_)) => 1`, `Err(_) =>
2` — all `u8`) and `diagnostics::report_exit_line`
(`Ok(Ended::AsAsked) => None`, the two `StoppedRunning` arms `=> Some(..)`,
`Err(error) => Some(report_startup_line(error))` — all `Option<String>`).
`main`'s `run` and `start` only ever wrap an `Ended` in `Ok`, never unwrap one
to re-raise it. `grep -rn "exit::ended|Ended::" crates/goad/src` outside
`exit.rs` shows no other site.

**Decisions taken during execution**

- The sheet's VA-2 (`grep … crates docs/slices/010`, "there is none in
  `crates/`") is stricter than `plan.md`'s original VA-2 ("no *surviving*
  reference to the removed outlet outside the slice's own history") — the
  sheet forbids the literal string anywhere under `crates/`, even inside a
  true historical aside. Followed the sheet, the operative instruction for
  this phase: `diagnostics.rs`'s module doc states the same historical fact
  (PHASE-08 added an outlet later renamed `report_exit`) without spelling
  the old identifier, satisfying both the letter of T-10 and design.md's
  "stay true as history" for T-6. Not a STOP — the two versions asked for
  the same outcome by a different route, and the stricter one was
  satisfiable without weakening any doc's truth.

**Findings**

- **Orchestrator re-measure at `5b23720`**: `just check` exit 0, gate **632**;
  M-9 re-run independently — compiled (no `error[E…]`), `cargo test
  --workspace --no-fail-fast` **596 passed, 1 failed**, the one being
  `structure::the_loop_s_ending_is_never_a_startup_failure`; restored and
  `diff`-clean. Matches the row above.
- **`diagnostics.rs`'s `//!` doc says `report_exit` was the outlet
  *"renamed"* at 010/PHASE-02. It was not renamed**: `report_startup(&StartupError)`
  was removed and `report_exit(&Result<Ended, StartupError>)`, a different
  function over a different value, replaced it (`design.md` §5.2). The cause is
  this sheet, not the executor: its VA-2 forbade the old identifier anywhere
  under `crates/`, stricter than `plan.md` PHASE-02/VA-2 (*"no surviving
  reference … outside the slice's own history"*), and the only way to keep the
  history while obeying it was a paraphrase. One-word repair (*replaced*, or
  name `report_startup` as history, which the plan's VA-2 admits); for audit's
  code review to disposition. The class — a sheet tightening a plan criterion
  without saying so — is the orchestrator's to carry.

### PHASE-03 — the binary tier and the consumer

**Objective:** the binary tier cannot reach a display, a case reads an ingress
bind failure's status and its own line off the built binary, both binary-tier
module docs state the two-tier cut as it now is, and `nix/module.nix` argues
from phase.

Written by the orchestrator, not the executor. **Where this sheet restates a
`plan.md` criterion it quotes it; where it narrows one it says so** — PHASE-02's
sheet narrowed VA-2 silently and cost a false sentence (§PHASE-02 §Findings).

**Surfaces** (`plan.md` PHASE-03, quoted; anything outside is a STOP):
*"`crates/goad/tests/binary/process.rs`, `crates/goad/tests/binary/main.rs`
(module doc only), `crates/goad/tests/binary/exit_codes.rs` (one case added;
module doc; no existing case touched), `nix/module.nix` (the `Service` block's
comment only)."* Plus this sheet, and `draft-spec.md` §7 / `design.md` §9 /
`canon-delta.md` Change 1 **only** if the new case is renamed, same commit.

**Entry criteria, verified rather than assumed**

- **EN-1 — discharged by the orchestrator at `5b23720`** (*"PHASE-02 `done`,
  its EX criteria hold on HEAD"*): §Status says `done`; `just check` **exit 0**,
  gate **632**, workspace **597**; M-9 re-run independently and matched.
  HEAD `38eb5f3` is documentation only on top of it.

**Baseline**: `goad` `tests/binary` **6** (`exit_codes` 5 + `version` 1),
`renderer` **221**, `checks` **46**, workspace **597**, gate **632**. This phase
adds one case: expected end `binary` **7**, workspace **598**, gate **633**,
nothing else moved. A figure that differs is a finding.

**Reading list** — by symbol, never `path:line`.

*What is being written*

- `crates/goad/tests/binary/process.rs` — `command` removes `WAYLAND_DISPLAY`,
  `WAYLAND_SOCKET` and `DISPLAY` from every spawn (`env_remove`, as
  `goad_with_no_config_home` already does for its two). Its doc says why
  (`design.md` §5.2, *The binary tier cannot reach a display*): past the socket
  the binary opens a real host on a machine with a display and
  `Command::output` waits for ever; with the three removed, the pinned winit
  answers *neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set* and
  the case fails fast. Also that **nothing but this doc holds the removal** —
  deleting it is green until a case gets past the socket, and then the gate
  hangs rather than reds (§5.2's last sentence).
- `crates/goad/tests/binary/exit_codes.rs`
  - **`an_unbindable_ingress_path_exits_2`** (the name is a commitment:
    `draft-spec.md` §7 R-3/R-4, `canon-delta.md` Change 1). Two scratch files
    named for the case and the process id, as `scratch_config` names its one:
    a **regular file** at the would-be socket path, and a configuration that
    loads and names it under `[ingress] path`. `plan.md` measured this body:
    `[backend] command = ["true"]`, `timeout = "5s"`; `[schedule]
    default_poll = "30s"`; `[ingress] path = "<the regular file>"` → exit 2,
    stderr `goad: <path>: not a socket — found a regular file`. Assert status
    **2** and that stderr **starts with** `format!("goad: {}: ", socket.display())`
    — the prefix only the ingress arm writes (P-1). Remove both files before
    asserting, as `an_unparseable_configuration_…` does. Its **doc says why
    the status alone would not do**: headless, a bindable path exits 2 too, at
    `PromptWindow::new`, with the display's line (P-1, measured).
  - `scratch_config` is used as it stands; if it must change, that is inside
    the surface — record why.
  - **Module doc** (AC-5's one permitted change; VA-2). Today it says
    *"`tests/renderer/startup.rs` covers the **arms** — every value `main`'s one
    `match` over `run()`'s `Result` can see. What it cannot see is the constant
    each arm names, because no pure test runs a process"*, and that a case past
    step 4 *"would red"*. Both false after PHASE-02/this phase. It states the
    cut from this side: the renderer tier holds **the numbers** (`exit::status`
    is pure); this tier holds that the **process** answers them to a caller.
    And what the spawn guarantees instead of *would red*. The history paragraph
    (*"Before these cases, `ExitCode::from(2)` could be changed…"*) is history
    and may stay. PHASE-02 rewrote `tests/renderer/startup.rs`'s module doc
    from the other side (EX-5) — read it and make the two agree.
  - **No existing case is touched** — not its body and not its doc comment
    (`slice-010.md` AC-5: *"The file's module doc is the one permitted
    change"*). See Assumption 4.
- `crates/goad/tests/binary/main.rs` — **module doc only.** It says *"The exit
  code is the half a pure test cannot reach at all"* and *"a case that reached
  step 5 would need a compositor and would red on every headless machine"*.
  After this phase: the numbers are held one tier down; this tier holds the
  process answering them; and a case past the socket fails fast on the
  display's line because `process::command` removes the display variables.
- `nix/module.nix` — the `Service` block's **comment** only (EX-4, quoted):
  *"no exception paragraph and none of its three false claims — no known
  exception to its own directive, no *do not succeed on a retry*, no
  *`SPEC-003`'s failure vocabulary*. It states the phase rule: 2 is a host that
  never started, so a restart changes nothing a person has not changed first;
  1 is a host that stopped running, which `Restart = "on-failure"` brings back
  after `RestartSec`; 0 is as asked. No spec number (D6). The directives are
  byte-identical."* Argue from phase, not from the field evidence: no outage
  counts or dates (`plan.md` notes). It currently names `main` mapping every
  `StartupError` to 2 — the number is now `exit::status`'s single `Err` arm.

*Design sections that bind*

- **§5.2** — *The two-tier cut moves, and both module docs say so*; *The
  binary tier cannot reach a display*; the `nix/module.nix` paragraph.
- **§9** — the `exit_codes::an_unbindable_ingress_path_exits_2` row; the
  mutations *classifier `Err(_) => 2` → `=> 1`* and *The new binary case*.
- **§7 D6** — no spec number anywhere this phase writes, the nix comment
  included.
- **`draft-spec.md` §7** R-3 and R-4 rows; **`canon-delta.md` Change 1** — the
  text that will cite this case at promotion.
- **`design-log.md`**, *P-1, raised at plan* — why the prefix and why the
  display-free spawn.

*Prior art*

- `an_unparseable_configuration_exits_2_and_says_only_what_its_own_arm_says`
  and `scratch_config` — scratch file, spawn, remove, then assert a prefix.
- `an_unreadable_configuration_…`'s doc — *"Each case asserts the prefix only
  its own arm produces"*; the new doc is the same argument for the ingress arm.
- `goad_with_no_config_home` — `env_remove` on the shared `command`.

*Memory*

- `docs/memory/negative-control-must-compile.md` — every mutation row records
  that the mutated build compiled.
- `docs/memory/tests-asserting-proxies.md` — the status alone is the proxy
  here; the mutations below are what prove the prefix is not.

**Assumptions**

1. Every existing binary-tier case settles before the first Slint call, so
   removing the display variables changes none of their outcomes.
2. `startup::listener` runs at `start` step 3, before any Slint call, and a
   regular file at the path is refused there with the line above (measured at
   plan).
3. **The one thing this phase is first to test** (VA-1): with the variables
   removed, a spawn that gets past the socket **exits in seconds on this
   machine** (where `WAYLAND_DISPLAY` is set) with the display's line. P-1
   predicted it; nobody has run it.
4. `help_prints_the_usage_block_on_stdout_and_exits_0`'s doc opens
   *"`Ok(())` is exit 0"* — false since PHASE-02 (`run` answers
   `Ok(Ended::AsAsked)`). AC-5 forbids touching it. **Do not edit it**; record
   it under §Findings for audit (the tension is AC-5's wording against a case
   doc going stale, and it is audit's to disposition).

**Hazard — read before anything else.** Until `process::command` removes the
display variables, a spawn that binds its socket launches a **real host on
the desktop** and `Command::output` never returns. So: land the spawn change
**first** (T-2); never run M-13 or M-14 before it; wrap every manual spawn in
`timeout 20`; and after M-13/M-14, `rm -f` any socket the bind left in the
temp directory.

**STOP conditions**

- A criterion compels a file outside **Surfaces**.
- An existing binary-tier case changes outcome when the display variables are
  removed.
- VA-1 does not hold: a past-the-socket spawn hangs, or exits other than 2
  with the display's line.
- A mutation does not compile, does not red the cases its row names, or reds
  others.
- The nix comment cannot state the phase rule without a directive changing.
- **Budget** ~200k: stop green, `PARTIAL` note here, commit, report.

**Forbidden.** `git stash`, `git checkout`, `git reset`, `git rebase`,
`git commit --amend`, `git push`. Editing `design.md`, `design-log.md`,
`plan.md`, `plan-log.md`, `canon-delta.md`, `draft-spec.md`, `slice-010.md` or
any `review-*.md`, except the rename carve-out. Amending canon. Weakening or
deleting a test. `git add <explicit paths>` only. You are the only writer.

**Tasks**

- [ ] T-1 — `git log --oneline -1` is `38eb5f3` or a documentation-only
      descendant adding this sheet; tree clean. Record it.
- [ ] T-2 — **The spawn change** (`process.rs`) and its doc. `cargo test -p
      goad --test binary --no-fail-fast`: **6** green (VT-2's first half; also
      run `version`).
- [ ] T-3 — **VT-1**, `an_unbindable_ingress_path_exits_2`. Red first on an
      assertion: write it asserting the wrong prefix (e.g. the config path
      instead of the socket path) and see it red on stderr, then correct it.
      Quote both.
- [ ] T-4 — **VA-1**, measured: run M-13 (below) under `time`; record wall
      time, status and stderr's first line. This is P-1's prediction checked.
- [ ] T-5 — **Docs**: `exit_codes.rs`'s module doc, `tests/binary/main.rs`'s
      module doc, `process::command`'s doc.
- [ ] T-6 — **`nix/module.nix`** comment. `git diff nix/module.nix` touches
      comment lines only (VA-3) — quote `git diff -U0 nix/module.nix | grep
      '^[-+][^-+]' | grep -v '^[-+] *#'` coming back empty. `nix-instantiate
      --parse nix/module.nix > /dev/null` exits 0.
- [ ] T-7 — **Refactor.** Read the three docs and the nix comment as a
      stranger: no *would red*, no *cannot see the constant*, no count, no spec
      number, the two tier docs agree with `tests/renderer/startup.rs`'s.
- [ ] T-8 — **EX-5, mutations** — table below, one at a time, byte-copy
      restore from the scratchpad, `diff`-verified. Scope: `cargo test -p goad
      --test renderer --test binary --no-fail-fast`.
- [ ] T-9 — **EX-3**: `git diff 5b23720 -- crates/goad/tests/binary/exit_codes.rs`
      shows the module doc, the added case (and any `use` it needs), and
      nothing else. Quote the stat and say which hunks are which.
- [ ] T-10 — **EX-6**: `an_unbindable_ingress_path_exits_2` resolves by that
      name; grep `draft-spec.md` and `canon-delta.md` for it.
- [ ] T-11 — **EX-1**: `just check` exit 0; figures against the baseline.
- [ ] T-12 — Commit (`010: PHASE-03 — …`). §Status PHASE-03 → `done`;
      §Harvest in place; §Findings (Assumption 4 at least), §Decisions,
      §Mutation evidence complete. Then go idle — no commit after reporting.

**Mutation evidence**

| # | the edit (quoted) | must red, by name | compiled? | redded | restore green? |
|---|---|---|---|---|---|
| M-12 | `exit::status`: `Err(_) => 2,` → `Err(_) => 1,` | `exit_status::every_startup_failure_is_2`; every failing case in `exit_codes.rs` — `too_many_arguments_exits_2_and_says_who_spoke`, `no_argument_and_no_configuration_home_exits_2`, `an_unreadable_configuration_exits_2_and_says_only_what_its_own_arm_says`, `an_unparseable_configuration_exits_2_and_says_only_what_its_own_arm_says`, `an_unbindable_ingress_path_exits_2` | yes | exactly the six named, nothing else (`cargo test -p goad --test renderer --test binary`: binary 2/7, renderer 220/221) | yes, `diff` against the scratchpad byte copy |
| M-13 | the new case's ingress path points at a **bindable** path — realised by *not* writing the regular file at `socket` before the spawn, so the path is free to bind (`an_unbindable_ingress_path_exits_2` itself, temporary) | `an_unbindable_ingress_path_exits_2`, **on the stderr prefix, not the status**, and it **exits rather than hangs** | yes | status assertion (2) passed; the prefix assertion panicked on `goad: the display could not be opened: Could not initialize backend. … neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set.`; `time` measured **0.545s** wall — VA-1 discharged | yes, `diff` against the scratchpad byte copy |
| M-14 | `start`: `startup::listener(config.ingress.as_ref())?` → `startup::listener(None)?` — **after T-2 only**; `crates/goad/src/main.rs` is outside Surfaces, so this edit is temporary by construction and its restore is `diff`-verified like the rest | `an_unbindable_ingress_path_exits_2`, on the prefix, exits rather than hangs | yes | status assertion (2) passed; the prefix assertion panicked on the same display line as M-13; exited well inside the `timeout 20` wrapper | yes, `diff` against the scratchpad byte copy |

**Decisions taken during execution**

- **A second scratch-path helper, `scratch_path`, added beside `scratch_config`.** The new case needs a path for the regular file occupying the would-be socket, which `scratch_config` does not build (it also writes TOML content). Rather than inline the naming scheme a second time, `scratch_path(case, extension)` factors the shared `goad-{case}-{pid}.{extension}` naming out for the one thing `scratch_config` does not cover. `scratch_config` itself is untouched — no rename-carve-out needed. Recorded per the sheet's instruction to say why if anything in this area changes.
- **VA-1 and M-13 run as one measurement.** T-4 asks to run M-13 under `time`; rather than construct a second, throwaway spawn harness for VA-1, the sheet's own M-13 mutation (temporarily removing the pre-write of the regular file at `socket`, so the path is bindable) is exactly the "spawn that gets past the socket" VA-1 asks for. Run once, under `timeout 20`, restored and `diff`-verified before moving on. `bind` leaves a sidecar `goad-an_unbindable_ingress_path-<pid>.socket.lock` in the temp directory even though the process never got to unbind it; it was removed by hand and confirmed gone with `ls`.
- **nix/module.nix comment rewritten from the phase rule, not edited line-by-line.** The exception paragraph and the "do not succeed on a retry" claim were removed as a block rather than patched, since keeping any sentence built on "Platform is the exception" would still be the argument-from-cause EX-4 forbids. The three directives (`Restart`, `RestartPreventExitStatus`, `RestartSec`) are byte-identical to before — confirmed by `git diff -U0 nix/module.nix | grep '^[-+][^-+]' | grep -v '^[-+] *#'` coming back empty.

**Findings**

- **Assumption 4 (recorded, not fixed — audit's to disposition).** `help_prints_the_usage_block_on_stdout_and_exits_0`'s doc comment (`crates/goad/tests/binary/exit_codes.rs`) opens *"`Ok(())` is exit 0"*, which has been false since PHASE-02: `run` now answers `Ok(Ended::AsAsked)`, not `Ok(())`. AC-5 forbids touching any existing case's body or doc comment in this file — its module doc is the one permitted edit — so the stale sentence stands. Tension: AC-5's wording (no existing case touched) against a case's doc comment that is now factually wrong. Not this phase's to resolve.

- **Orchestrator re-measure at `c67dd9c`**: `just check` exit 0, gate **633**
  (baseline 632 + the one case). Diff read against the surfaces: five paths,
  all declared.
- **`nix/module.nix`'s comment says *"0 is the window being closed, which was
  asked for"*.** Narrower than true: the tray's quit and `--help` / `--version`
  are 0 as well, and EX-4 quotes the rule as *"0 is as asked"*
  (`draft-spec.md` R-1 names both routes). Comment-only repair; for audit.
- **`tests/binary/main.rs`'s module doc says the startup failures settle in
  `start`'s *"first step"*.** A sentence this phase edited, and the case this
  phase added settles at step 3 (`startup::listener`). For audit.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-24 · slice closed · the `010: close` commit

### Produced

- `review-design.md` rounds 1–3: F-1…F-34 disposed, repaired and verified.
  Round 4: F-35…F-50 raised, F-51 and F-52 raised by the responder; all
  `verified` in round 5. Rounds 5 and 6: F-53…F-68, disposed with the
  user and `verified` (round 6's by site check). The ledger is the
  artefact; nothing about it is restated here.
- **PHASE-01**: `crates/goad/src/exit.rs` (`Ended`, `ended`, `status`),
  `Cancel::is_stopped`, `diagnostics::report_exit_line`, and the cases
  `draft-spec.md` §7 names for them. M-1…M-8 run, compiled and recorded in this
  sheet's §Mutation evidence; audit cites those rows rather than re-deriving
  them. Nothing calls the new code from `main` — that is PHASE-02.
- **PHASE-02**: `start` hands the event-loop call's result and
  `stop_signal.is_stopped()` to `exit::ended`; `main` is `run()` →
  `diagnostics::report_exit` → `exit::status`; `StartupError` no longer carries
  the loop's end, held by `structure::the_loop_s_ending_is_never_a_startup_failure`.
- **PHASE-03**: `crates/goad/tests/binary/process.rs`'s spawn removes
  `WAYLAND_DISPLAY`, `WAYLAND_SOCKET` and `DISPLAY` from every binary-tier
  spawn, so a case that gets past the socket fails fast at `PromptWindow::new`
  instead of opening a real host (measured, VA-1: **0.545s** wall, exit 2, the
  display's line). `exit_codes::an_unbindable_ingress_path_exits_2` reaches
  `StartupError::Ingress` headlessly and asserts the socket's own prefix, not
  the status alone. `exit_codes.rs`'s and `main.rs`'s module docs, and
  `nix/module.nix`'s `Service` comment, now argue from **phase** — no *would
  red*, no exception paragraph, no spec number (D6). M-12…M-14 run, compiled
  and recorded in this sheet's §Mutation evidence. `draft-spec.md` §7's R-3/R-4
  rows and `canon-delta.md` Change 1 already cite the case by its landed name —
  no rename, no canon edit needed this phase.

- **Audit and code review**: `audit.md` (evidence, AC-9 observed on the running
  host, reconciliation, verdict) and `review-code.md` (closed; its Synthesis).
  SPEC-004 promoted from the draft; SPEC-003 R-3/R-4/§9 amended.
- **Lifted to `docs/memory/` at close**:
  `losing-only-the-hosts-display-connection.md` (the `gdb` `shutdown` route,
  proven at AC-9); `wildcard-enum-match-arm-counts-a-named-binding.md` gained
  the cost measured here (P1-a); `exit-2-means-two-different-failures.md`
  rewritten to its lesson alone (R-2);
  `the-journal-is-the-audit-instrument-for-a-shipped-unit.md` corrected — the
  cause was a broken connection, not a departing compositor.

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
- **The workspace lints pass the pure layer over the real Slint types.**
  `clippy::pedantic` and the crate-root `wildcard_enum_match_arm` deny raise
  **nothing** on `exit::ended`, `exit::status`, `report_exit_line`'s `Option`
  arms or `Cancel::is_stopped` as `design.md` §5.2 writes them. The design
  review's spike over stand-ins predicted this correctly; the assumption is now
  measured and PHASE-01's likeliest STOP did not occur.
- **But the deny does not *hold* `exit::status`.** Measured by negative
  control, with the mutated build seen to compile: a `_` arm over
  `&Result<Ended, StartupError>` beneath `Ok(Ended::AsAsked)` fires neither
  `wildcard_enum_match_arm` nor `match_wildcard_for_single_variants`, while the
  same wildcard over `&Ended` fires the latter. Matching on the enclosing
  `Result` — the remedy
  `docs/memory/wildcard-enum-match-arm-counts-a-named-binding.md` prescribes —
  is also how a match leaves the lint's reach. Full entry in §Findings, for
  audit.
- **A decision function needs two stand-ins, not one, to red every case.** The
  plan's stand-in for `ended` (`AsAsked` whatever it is given) cannot red the
  two cases that *expect* `AsAsked`; the same holds for `is_stopped`'s constant
  `false` and for `report_exit_line`'s constant line, which cannot red the
  distinctness case. Running the opposite stand-in first costs one build and is
  what makes *every* case a seen failing assertion rather than a case merely
  present (`docs/memory/tests-asserting-proxies.md`).
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

**Swept at close, 2026-09-24**, against `slice-010.md` §Follow-ups. Every entry
below is dispositioned in its own bold lead; none carries into the ledger, since
each was settled inside the slice. The ledger rows the slice raised came from
§Follow-ups and `audit.md` (FU-5, FU-10, FU-42, FU-43, FU-44).

- **Settled.** **The design review is closed** (F-1…F-68 `verified`). Round 5's blocker,
  F-53, changed the design: every end is decided on the request, and
  `Ended::StoppedRunning` carries `Option<slint::PlatformError>`. Round 6
  found one design-level gap in that repair (F-63, a missing case) and prose;
  both were repaired and closed by a site check instead of a round 7.
- **Settled at close** (P1-a: memory, and `audit.md` §Design drift). **Settled at PHASE-01, and no longer open:** the lints were spiked over
  stand-in types at design rounds 5 and 6; they have now run in the tree over
  the real `slint::PlatformError` and raise nothing (§Learned). `exit::ended`,
  `exit::status`, `report_exit_line` and `Cancel::is_stopped` are built and
  asserted. What the negative control added is a **new** open item, in
  §Findings: the crate-root deny does not reach `exit::status`'s match at all,
  so `design.md` §3's sentence about it reads stronger than the gate is.

- **Settled by PHASE-02** (`5b23720`). **`start`'s wiring is still specified and unbuilt** — PHASE-02's. The read is
  `stop_signal.is_stopped()`, taken in the statement after the one that binds
  the loop call's result, on a clone kept before `cancel` moves into `serve`.
  `Cancel::is_stopped` now exists and is documented against `Cancel::stopped`,
  which is the existing **future**: awaiting the latter where the former is
  meant waits for ever on a host nobody asked to stop. Neither `stop` nor
  `stopped` was touched.
- **Settled at close** (`audit.md` R-3). **`research.md` carries a count** — *"the numeral 2 keeps its meaning and its
  five tests"* — which `exit_codes.rs` will falsify the moment a case is added
  there. Not raised as a finding: `research.md` was context to this review and
  not its subject. Sweep it at audit.
- **Settled at close** (`audit.md` R-2) — and this entry was wrong that the
  standing fact still held: after PHASE-02 exit 2 means *never started* only,
  so the file was rewritten to its lesson rather than re-quoted.
  **`docs/memory/exit-2-means-two-different-failures.md` is stale after
  PHASE-02, for close to lift.** It quotes `main` as having "one `match` over
  `run()`'s `Result`" and the pre-seam call line
  (`` slint::run_event_loop_until_quit().map_err(StartupError::Platform)?; ``)
  — `main` no longer matches over anything, and the call's own line now ends
  bare (`crates/goad/src/main.rs`'s `start`). The file is outside PHASE-02's
  surfaces, so it was not edited here; its standing fact (exit 2 is
  ambiguous, and a new `StartupError` variant inherits it) still holds and
  needs only its quoted code repaired to match the tree.

## Handover — 2026-09-23, audit under way: review round 1 in, repairs next

Written for a **fresh orchestrator**. Read `CLAUDE.md` → `docs/AGENTS.md`
§Audit & reconcile and §Close → this section → `audit.md` → `review-code.md`.

**Where it is.** All three phases `done` (`ab5604f`, `5b23720`, `c67dd9c`),
each re-measured by the orchestrator. Audit has:

- `audit.md` — Brief (`58df7c3`, written before the evidence), Evidence, the
  AC table, the VT/VA/VH walk, surface delta (**no undeclared paths**), a
  **draft** Reconciliation table (C-1…C-6, and rows for every carried finding
  with a recommended disposition), and **the AC-9 steps for the user**, in nu
  syntax, under *AC-9 — on the running host*. Gate at `f9620b6`: `just check`
  exit 0, **633**; workspace **598**.
- `review-code.md` — round 1 (`df2adde`, reviewer independent: it did not read
  the carried findings before writing). **No blocker; F-1, F-2 major; F-3…F-6
  minor; F-7, F-8 nit.** The ledger is the artefact — read it, not this list.
  F-1 overlaps audit's A-1 and PHASE-03's nix finding; F-6 overlaps P3-a/P3-c;
  F-2…F-5 are new.
- **User decisions at audit**, `design-log.md` (*at audit*): canon C-1…C-6
  **endorsed**; **AC-5 waived for doc comments** (record the waiver in
  `audit.md` against AC-5); A-1 **reworded as policy**. Nothing has been
  promoted or repaired yet — every `done` box in §Reconciliation is open.

**Next, in order.**

1. **Disposition round 1 with the user** — each finding, confirm before
   acting (`docs/AGENTS.md`). **F-2 is likely a decision, not a spelling**: the
   platform error's `Display` spans several lines, so the final stderr "line"
   is not one line and its last line names neither binary nor phase (R-4,
   P-B). How a multi-line error becomes one line — escape it through the
   module's existing escape/bound pipeline, or take its first line, or
   something else — is the user's call; check first whether the pipeline
   `diagnostics.rs`'s `//!` doc describes already applies to other lines and
   why these two bypass it. F-3 (`--help > /dev/full` exits 0) may be a spec
   question about R-1's *answered* rather than a code fix.
2. **One fresh repair agent** for the dispositioned findings plus the carried
   doc repairs (P2 *renamed*, P3-a under the waiver, P3-b, P3-c, A-1) and
   **C-6** (the `SPEC-004` citations — only after C-1 lands, or in the same
   commit). Fix the class, not the instance: F-1 is *any* retry prediction off
   status 2, in every file.
3. **Canon promotion C-1…C-5** — endorsed; apply exactly as `canon-delta.md`
   and §Reconciliation state, tick each row. C-2 only after re-checking every
   case §7 names resolves (true at `f9620b6`).
4. **Round 2** — a reviewer on the repairs, same ledger. Code review is
   unbounded at this tier; budget for round 3 (`audit-stage-needs-its-own-budget`
   in the orchestrator's memory: the repairs' review is half the cost).
5. **AC-9 with the user** — hand them the steps from `audit.md` pasted into
   chat, not a pointer to them. It needs the new build on their host. The
   reviewer's *not reached*: whether anything writes to stderr **after**
   `report_exit` on a real stop (a pending `serve` future dropped after
   `main` returns) — watch for it in the journal during AC-9.
6. **Close** — `docs/AGENTS.md` §Close: FU-1 struck (AC-10); `research.md`
   count sweep; §Open swept; P1-a into the wildcard memory file and
   `design.md` §3 under design drift; P1-b onto the FU-10 row;
   `docs/memory/exit-2-means-two-different-failures.md` updated; follow-ups
   ledger re-verified for rows naming touched files.

**Housekeeping.** The round-1 reviewer's worktree is still on disk
(`.claude/worktrees/agent-adaf66fe96d296402`, branch
`worktree-agent-adaf66fe96d296402`, its one commit cherry-picked as `df2adde`);
remove it with `git worktree remove` once round 2 is spawned. The older
`goad-009-proto` worktree is not this slice's.

### Repairs, round 1 — 2026-09-23

Repair agent, one writer, on `main` from `e412953`. Every `review-code.md`
round-1 finding repaired as its Response states; the Outcome column is left for
the round-2 reviewer.

- **F-2** — `aede3df`. `report_startup_line`, `report_exit_line`'s
  *stopped running* arm and `report_platform_line` go through `finish(…,
  LINE_LIMIT)`; `report_exit_line`'s `Err` arm inherits it from
  `report_startup_line`. Red first:
  `stderr_outlets::a_multi_line_platform_error_is_one_line_from_every_outlet`
  panicked with the raw three-line text (`goad: the display could not be
  opened: Could not initialize backend.` / `Error from Winit backend: …` /
  `No backends configured.`). The case also asserts the line still *ends*
  with the escaped `\nNo backends configured.`, so first-line-only (the
  rejected option) reds it too.
- **F-3** — `33c4654`. Shape chosen: **a sibling in stratum 2**,
  `goad_shell::report::try_line_to` (write, then flush, answering the
  `io::Result`), with `line_to` delegating to it and discarding. Why: one
  implementation of *write a line*, two policies over it; `print_usage` /
  `print_version` answer `io::Result<()>` and `run` maps it to the new
  `StartupError::AnswerUnwritten(io::Error)` (*"the answer could not be
  written to standard output: {error}"*). `goad-emit`'s `to_stdout` /
  `to_stderr` call `line_to` unchanged; the only difference they see is a
  `flush` after the `writeln!`, whose result is discarded with the write's —
  on a line-buffered stdout and an unbuffered stderr it writes nothing further,
  so no status or byte they produce moves (its binary tier is green). The
  flush is there because a buffered sink reports its failure only at the
  flush, and Rust's exit-time flush reports it to nobody. `line_to`'s *the
  exit code still carries the fact* is gone — false for an answering caller,
  and for `report_platform`, whose process keeps running. Red first:
  `exit_codes::an_answer_that_cannot_be_written_exits_2` — `left: 0, right: 2`
  for `--help` on `/dev/full`. Both questions are covered by that one
  binary-tier case: the failing write is the process's own stdout, which only a
  spawn can point at a refusing device (the print fns lock the stdout of
  whichever process calls them — at the renderer tier, the harness's).
  Also red-first by compile: `report::tests` gained the two `try_line_to`
  cases before the function existed. `StartupError::AnswerUnwritten` added to
  `display_text`, `source_walk` and `every_startup_failure_is_2`.
- **F-1 class grep** — `git diff b444c6a^..HEAD -- crates nix` for
  `restart|retry|retries|gains nothing|changes nothing|try again|trying again|recover|cannot start|would fail|brings back|comes back|next try`,
  plus the same over every touched file. Hits that were predictions:
  `exit.rs` `//!` (*gains nothing by restarting 2*), `nix/module.nix`'s
  `Service` comment (*a restart changes nothing…*, *suppresses the one retry
  that would only loop…*, *brings back*), `exit_codes.rs` `//!` (*a host that
  cannot start … until systemd's start limiter gives up*). All three
  repaired. Non-hits: `nix/module.nix`'s `Description` comment (*the cutover
  changes nothing a person reads* — about text, not status), `BUSY_NOTICE`
  (*try again in a moment* — back-pressure, not a status), and the directive
  names themselves. The unit comment now states policy: 0 as asked (tray
  quit, window closed, `--help` / `--version` answered) and not restarted; 1
  restarted after `RestartSec`; 2 left to a person, `RestartPreventExitStatus`
  being that choice. A-1's *rather than retry into the rate limiter* was not
  used: it predicts the retry fails, which is the class. Directives
  byte-identical (`git diff -U0 nix/module.nix | grep '^[-+][^-+]' | grep -v
  '^[-+] *#'` empty); `nix-instantiate --parse` exit 0.
- **F-4** (`exit::status`'s `u8` reason), **F-5** (`report_exit_line` names
  the on-entry exception; `report_exit` says `AsAsked` writes nothing), **F-6
  / P3-a / P3-c** (`exit_codes.rs` says `exit::status` chooses; the `help_…`
  doc says `Ok(Ended::AsAsked)`; `tests/binary/main.rs` names the rule — the
  questions `run` answers and the failures that settle before the first
  component — not *first step* or *the two zero-exits*), **F-7**
  (`process::command` says what the removal holds and that it rests on
  Slint's winit backend alone — `cargo tree -p goad -e normal -i
  i-slint-backend-linuxkms` prints nothing), **F-8 / P2** (`diagnostics` `//!`:
  *replaced*, and names `report_exit_line`). Also `stderr_outlets`' doc lost
  its count (*the two stderr outlets*).
- **`draft-spec.md` §7** — R-1 cites `an_answer_that_cannot_be_written_exits_2`
  for `AsAsked` being earned; R-3's list names it and drops its history clause
  (*All but one are unchanged by this document's arrival* — a count, and a
  changelog in an evergreen document); R-4 cites it as a prefix case, and
  cites `a_multi_line_platform_error_is_one_line_from_every_outlet` for *one
  line* in place of the single-literal proxy, leaving *nothing after it* as
  review. Every new citation resolves once (`grep -rn "fn <name>\b" crates`).
- `slice-010.md` §Follow-ups' `goad-emit` `--help` row notes that
  `try_line_to` exists, which lowers its price.
- **Gate** at the end of Part 1: `just check` exit 0, **638 passed**, 0 failed
  (633 + the F-2 case, `display_text::answer_unwritten`, the F-3 binary case,
  and two `report::tests`).

**Part 2 — canon promotion** (the commit after `ebaba86`). C-2's precondition
re-checked immediately before: every `module::case` citation in the draft and
in `canon-delta.md` resolves to one definition (`help_prints_…` to two, the
second being `goad-emit`'s namesake, as round 1 recorded). C-1 `git mv`
`draft-spec.md` → `docs/specs/004-process-exit-status.md`, Status → `active`
in SPEC-003's header form, `SPEC-NNN` → `SPEC-004`; C-2 `DRAFT-ONLY` comment
removed; C-3/C-4/C-5 applied to SPEC-003 as `canon-delta.md` states, with
`SPEC-00N` → `SPEC-004`; C-6 in the same commit — `nix/module.nix`,
`exit.rs` `//!`, `StartupError`'s doc — plus `report_exit_line`'s doc, whose
F-5 repair had cited `draft-spec.md` §5 until the number existed. Nix
directives byte-identical against `b444c6a^`; `nix-instantiate --parse` exit 0.
`audit.md`: C-1…C-6, P2, P3-a, P3-b, P3-c, A-1 ticked; the AC-5 waiver
recorded in the AC table; the Reconciliation preamble no longer says
*nothing here is applied*. Not touched: `audit.md`'s AC-8 row still reads
*pending — not applied* (the Evidence section is a snapshot at `58df7c3`; the
verdict is audit's to write). `just check` exit 0, **638 passed**.

### Repairs, round 2 — 2026-09-23

Scope: `review-code.md` F-1 (*Re-disposition, round 2*), F-9…F-15, as
dispositioned in `5b51c90` and endorsed in `design-log.md` (*code review round
2*). Commits `e0f1488` (code, tests, docs) and `07c71f3` (SPEC-004).

- **F-10/F-11/F-15** — a helper `one_line` beside `finish`: at most one
  trailing terminator dropped through `without_one_terminator`, then
  `Escaped`, and no bound. `report_startup_line`, `report_exit_line`'s
  `StoppedRunning(Some(_))` arm and `report_platform_line` take it; `finish`
  and `LINE_LIMIT` remain the in-window surface's. The `StoppedRunning(None)`
  literal is left as it was: the docs now claim the step for *composed*
  lines, not *every line*. Red first, `stderr_outlets` in
  `crates/goad/tests/renderer/startup.rs`:
  - `a_configuration_error_far_along_a_long_line_keeps_the_parser_s_message`
    (a real `Config::parse` error at column 1511):
    `goad: /home/someone/.config/goad/config.toml: configuration is not valid: TOML parse error at line 1, column 1511\n  |\n1 | note = "aaaa…` — cut before toml's message, on the *more characters not shown* assertion.
  - `no_stderr_outlet_ends_in_a_visible_terminator`:
    `…key with no value, expected `=`\n`.
  - `no_stderr_outlet_bounds_its_line`: `goad: the display could not be opened: xxxx…` cut before ` tail`.
  - `LINE_LIMIT` reliance checked: no test or canon relied on it bounding a
    stderr line. Docs amended — `diagnostics`' module doc, `LINE_LIMIT`'s,
    `finish`'s, `report_startup_line`'s; SPEC-004 §7 R-4's pipeline sentence
    (now citing the three cases). **Beyond the brief:** `docs/follow-ups.md`
    FU-4's *"every diagnostic line in the same binary passes `finish(..,
    LINE_LIMIT)`"* narrowed to the in-window surface, since it became false.
    `docs/slices/007/slice-007.md` carries the same sentence and was left as a
    closed slice's record.
- **F-12** — `StartupError::AnswerUnwritten`'s doc and SPEC-004 §7 R-1's row
  say *a write the stream refused*; the row states once that Rust's runtime
  reopens a closed fd 0–2 on `/dev/null` before `main`, and the doc points to
  it. No code change.
- **F-13** — `report::tests::a_refused_flush_is_reported_to_a_caller_whose_line_is_the_answer`,
  a sink (`RefusesFlush`) whose write succeeds and whose flush fails.
  **Mutation:** `sink.flush()` → `Ok(())` reds it (`4 passed; 1 failed`);
  restored by byte copy, 5/5.
- **F-14** — §5's diagram: `Invoked --> Question`, then `Question -->
  Answered: the answer was written` and `Question --> NeverStarted: the stream
  refused the answer`.
- **F-1/F-9** — §1's *today* paragraph replaced by the rule alone; *"Once this
  exists"* in the next paragraph became *"With the status stated"* (the same
  tense class, inside §1).
- **Class grep** (*restart*, *retry*, *next try*, *gains nothing*, *changes
  nothing*, *would have come back*, *today*, plus *trying again*, *comes
  back*) over SPEC-004, the lines `cc0db76` added to SPEC-003, `crates`, and
  `nix`. No retry prediction read off a status survives. SPEC-004's remaining
  hits state the rule (§1, §2, §3, §5, §6). Hits in `crates`/`nix` are
  unrelated (`retry_after_ms`, timers, `today_local`, the ingress lock's
  upgrade note) or state policy (`nix/module.nix`, `exit.rs`,
  `exit_codes.rs`'s *the unit would then restart* — the unit's behaviour
  under a changed numeral, not a retry outcome). **Not repaired, outside the
  endorsed scope:** SPEC-004 §2's two *today*s (*"What `goad-emit` does
  today"*, *"no owner today"*) and §8's *Not today* / *ungoverned today* /
  *today and inventing* — true of the tree, not predictions; any rewording
  is a canon edit outside §1/§5/R-1/pipeline sentence.
- **Gate:** `just check` exit 0, **642 passed**, 0 failed, 0 ignored, over
  31 `test result` lines (638 + three `stderr_outlets` cases + one
  `report::tests`).

### Repairs, round 3 — 2026-09-23

Scope: `review-code.md` F-16…F-23, prose only, as dispositioned in `5c0dbe3`
and endorsed in `design-log.md` (*code review round 3*). One commit.

- **SPEC-004** — §1 names the seam in the rule's sentence and points to §5
  *What the seam costs* (F-16), and its supervisor sentence is rewrapped
  (F-22); §2's SPEC-001 bullet drops *no owner today* for *nominally owned
  here, and not governed* (F-18); §5's *Both edges* prose names the refused
  answer as *never started* (F-17); OQ-1 keeps only the per-cause-judgement
  reason (F-19); R-1's §7 row says the closed-stdout behaviour is the Rust
  runtime's, evidenced by a traced run, and held by no test (F-21); R-4's §7
  row states the cost of no bound — journald's `LineMax=` split (F-20).
- **`diagnostics`** — `one_line`'s doc states the same cost and who controls
  the length (F-20); `finish`'s doc rewrapped and its *stderr* qualified
  (F-22, F-23); `STDERR_LIMIT`'s doc names a captured backend's stderr,
  distinct from the host's own standard error (F-23).
- **Gate:** `just check` exit 0, **642 passed**, 0 failed, 0 ignored, over
  31 `test result` lines — unchanged, as no code moved.

### Close — 2026-09-24

One writer, on `main` from `41a1bec`. `docs/AGENTS.md` §Close worked in full;
`audit.md` §Closure ticked.

- **Reconciliation** R-1 (FU-1 struck under the ledger's §Closed, with its
  three corrections and its *SPEC-003's failure vocabulary* error), R-2, R-3,
  P1-a, P1-b and A-2 done. `audit.md`'s AC-8, AC-10 and *Code review* rows
  brought current.
- **Ledger re-verified** for the rows naming a file the slice touched
  (`git diff --stat b444c6a^..HEAD`): FU-1 killed; FU-5's count was short —
  `exit_codes.rs`'s `scratch_config` and `scratch_path` are unheld helpers of
  its class, so the row was extended and its counts replaced by names; FU-4
  (`diagnostics.rs`, `finish`) still true as round 2 narrowed it; FU-10 extended
  (P1-b); FU-28 (`nix/module.nix`'s `extraConfig` type) still
  `attrsOf anything`. **Not repaired, and outside close:** `claim`'s own doc in
  `tests/support/scripting.rs` still counts *six helpers … this holds four* —
  a code comment, FU-5's to fix.
- **A-2, record only.** PHASE-03's task boxes T-1…T-12 and PHASE-02's T-13 are
  unticked, and T-3's red-first quote is missing. The work is evidenced by each
  sheet's §Mutation evidence and §Decisions and by the orchestrator's
  re-measure; M-13 and M-14, each redding
  `exit_codes::an_unbindable_ingress_path_exits_2` on its prefix, stand in for
  T-3's red. The boxes are left as they were: ticking them now would record a
  check at a time it was not made.
- **Nit:** `diagnostics::report_exit_line`'s doc rewrapped to 80 columns.
- **Gate:** see `audit.md` §Evidence, *At close*.

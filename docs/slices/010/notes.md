# Notes — Slice 010

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | done | 2026-09-23 |
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

- [ ] T-1 — `git log --oneline -1` is `e7aca88` or a documentation-only
      descendant of it that adds this sheet; tree clean. Record it.
- [ ] T-2 — **VT-1, the slice's one natural red.** Write
      `ends_at_the_loop_call` and `the_loop_s_ending_is_never_a_startup_failure`
      against today's tree. Run `cargo test -p goad-boundary --test checks
      the_loop_s_ending --no-fail-fast`; it must **red on an assertion**, naming
      `crates/goad/src/main.rs` and the `.map_err(StartupError::Platform)?;`
      line. Quote the failure here.
- [ ] T-3 — **The seam** (`main.rs`) and `report_exit` in / `report_startup`
      out (`diagnostics.rs`), in one movement. `cargo build -p goad`; then T-2's
      command **green**. Quote it.
- [ ] T-4 — **VT-3.** `cargo test -p goad --test binary --no-fail-fast`: all
      **6** green, `git diff` over `crates/goad/tests/binary/` empty.
- [ ] T-5 — **VT-2.** The two `counting_itself` controls. Red them first
      against a deliberately wrong `ends_at_the_loop_call` (e.g. `|_| true`
      reds the second, `|_| false` the first); then the real body.
- [ ] T-6 — **Docs** (EX-3, EX-4, EX-5): `startup.rs`'s four sites,
      `diagnostics.rs`'s two, `renderer/startup.rs`'s module doc, and every
      comment in `main.rs` the new shape made false.
- [ ] T-7 — **Refactor.** Not optional. Read each surface back as a stranger
      would: does any doc still say the loop's error is a startup failure, that
      `main` matches, or count something; is `ends_at_the_loop_call` the only
      new matcher (no parallel walk); does `main.rs` still read top to bottom.
- [ ] T-8 — **EX-7, the scan mutations.** Each applied alone to a tree restored
      from a byte copy of `main.rs` (scratchpad, never `git checkout`), run
      with `cargo test --workspace --no-fail-fast`, recorded below, restored and
      `diff`ed against the copy. Fill in *compiled?* and *redded*; the red set
      must be **exactly** the one case.
- [ ] T-9 — **VA-1**, the review no test reaches. Record each, by symbol, under
      §VA-1 below: (a) `is_stopped` is read on `stop_signal`, a clone of the
      `Cancel` handed to `serve`, in the statement after the call's own; (b)
      `exit::ended` receives the call's own result, unmapped; (c) no site in
      `crates/goad/src` other than `exit::ended` constructs
      `Ended::StoppedRunning` — `grep -rn 'StoppedRunning(' crates/goad/src`,
      quoted, and each hit classified (construction vs pattern); (d) nothing
      downstream of `exit::ended` turns an `Ended` into an `Err`.
- [ ] T-10 — **VA-2.** `grep -rn "report_startup\b" crates docs/slices/010`,
      quoted; every hit outside `crates/` is slice history, and there is none
      in `crates/`.
- [ ] T-11 — **EX-8.** Every case `draft-spec.md` §7 cites that this phase owns
      resolves by the name cited — the closed list is the three case names in
      T-2 and T-5. Grep each.
- [ ] T-12 — **EX-1.** `just check` exits 0. Quote the gate total **and** the
      workspace denominator, and `checks` / `renderer` / `binary`, against the
      baseline above.
- [ ] T-13 — Commit (`010: PHASE-02 — …`). §Status PHASE-02 → `done`.
      §Harvest updated **in place** (add the stale memory file under what close
      must lift). §Findings, §Decisions and §Mutation evidence complete. Then go
      idle — make no commit after reporting.

**Mutation evidence**

| # | the edit (quoted) | must red, by name | compiled? | redded (workspace, `--no-fail-fast`) | restore green? |
|---|---|---|---|---|---|
| M-9 | `start`: `let call = Ok(slint::run_event_loop_until_quit().map_err(StartupError::Platform)?);` | `the_loop_s_ending_is_never_a_startup_failure` only | | | |
| M-10 | `main.rs`: add `use goad::startup::StartupError::Platform;`; `start`: `let call = Ok(slint::run_event_loop_until_quit().map_err(Platform)?);` | `the_loop_s_ending_is_never_a_startup_failure` only | | | |
| M-11 | `main.rs`: a second, real production call, **over several lines so its own line ends at the call** — this row isolates the count half — e.g. `#[allow(dead_code)]` / `fn loop_again() {` / `  let _ = slint::run_event_loop_until_quit();` / `}` | `the_loop_s_ending_is_never_a_startup_failure` only, on the **count** assertion | | | |

**VA-1**

<!-- (a)–(d) from T-9, each with the symbol it was checked against. -->

**Decisions taken during execution**

**Findings**

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-23 · PHASE-01 `done`, the pure layer green in the tree · `aeca5a6` plus this phase's commit

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

- **The design review is closed** (F-1…F-68 `verified`). Round 5's blocker,
  F-53, changed the design: every end is decided on the request, and
  `Ended::StoppedRunning` carries `Option<slint::PlatformError>`. Round 6
  found one design-level gap in that repair (F-63, a missing case) and prose;
  both were repaired and closed by a site check instead of a round 7.
- **Settled at PHASE-01, and no longer open:** the lints were spiked over
  stand-in types at design rounds 5 and 6; they have now run in the tree over
  the real `slint::PlatformError` and raise nothing (§Learned). `exit::ended`,
  `exit::status`, `report_exit_line` and `Cancel::is_stopped` are built and
  asserted. What the negative control added is a **new** open item, in
  §Findings: the crate-root deny does not reach `exit::status`'s match at all,
  so `design.md` §3's sentence about it reads stronger than the gate is.

- **`start`'s wiring is still specified and unbuilt** — PHASE-02's. The read is
  `stop_signal.is_stopped()`, taken in the statement after the one that binds
  the loop call's result, on a clone kept before `cancel` moves into `serve`.
  `Cancel::is_stopped` now exists and is documented against `Cancel::stopped`,
  which is the existing **future**: awaiting the latter where the former is
  meant waits for ever on a host nobody asked to stop. Neither `stop` nor
  `stopped` was touched.
- **`research.md` carries a count** — *"the numeral 2 keeps its meaning and its
  five tests"* — which `exit_codes.rs` will falsify the moment a case is added
  there. Not raised as a finding: `research.md` was context to this review and
  not its subject. Sweep it at audit.

## Handover — 2026-09-23, PHASE-01 done, PHASE-02 next

Written for a fresh agent. The slice is **executing**. `plan.md` is accepted at
`448f678` and **no plan review runs** (`plan-log.md`, and what that costs is
recorded there). **PHASE-01 is `done`** at `ab5604f`: the pure layer is in the
tree, asserted one tier down, and nothing calls it from `main` yet.

The gate was re-run by the orchestrator at `ab5604f` rather than taken from the
phase agent's report: **exit 0**, gate total **629**, `cargo test --workspace`
**594** — the gate runs `cargo test -p goad-semantics` as a command of its own,
so that crate's 30 + 5 are counted twice and the total stays exactly 35 above
the workspace figure. `goad` lib **58 → 59**, `goad` `tests/renderer`
**208 → 221**.

**PHASE-01's negative control is the result worth carrying forward**, and it is
in §Findings: the crate-root `wildcard_enum_match_arm` deny does **not** reach
`exit::status`'s match. Re-measured by the orchestrator — `_ => 1` in place of
the last two arms compiles and leaves `cargo clippy -p goad --all-targets --
-D warnings` green. What holds that match's exhaustiveness is `exit_status`'s
cases and M-1/M-2, not the gate. No code is wrong; `design.md` §3's sentence
about the deny is what reads stronger than the gate is, and it is audit's to
disposition.

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

**Next:** write PHASE-02's sheet, then run it. One phase, one agent, one
session. PHASE-02/EN-1 is *PHASE-01 `done` in §Status, and its EX criteria hold
on HEAD* — verify that against the tree rather than against this paragraph.
PHASE-02 opens with a case that **reds on today's tree**
(`structure::the_loop_s_ending_is_never_a_startup_failure`), which is the one
natural red in the slice; record it.

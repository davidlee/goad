# Notes — Slice 010

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | pending / in progress / done / blocked | |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — <name>

**Objective:** <copied from plan.md>

**Reading list**
<!-- path:line references, the design sections that bind, prior art. -->

**Assumptions & STOP conditions**
<!-- What is being taken on faith, and the specific conditions under which the
     agent must stop and consult the user rather than improvise. -->

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [ ]

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

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

## Handover — 2026-09-23, plan stopped at design verification

Written for a fresh agent. The slice is **in plan**, and the planner has
**stopped before writing `plan.md`** on a design claim that failed verification
against the tree (`docs/AGENTS.md` §Plan: an unresolved design issue goes back
to design). `plan.md` is still the template and no code has been written.

**What verified clean, at `b444c6a`.** Every symbol and call site `design.md` §5
names: `main`/`run`/`start` and the four `StartupError::Platform` sites; the
`StartupError` doc sentences §5.2 repairs; `report_startup`, its one caller and
the `//!` sentence naming it; `report_startup_line`'s doc opening on
`report_startup`; `Cancel` holding its own `watch::Receiver`, `Cancel::stopped`
as the future, `Notice::raised` as the `*self.rx.borrow()` precedent and
`tests::a_raised_notice_stays_raised_until_it_is_lowered` beside it;
`install`'s `on_close_requested` and `tray.on_quit` as the two `stop` routes;
`structure.rs`'s `code_of`, `production_lines`, `occurrences_where`,
`calls_resolve` and `counting_itself`; `display_text::platform` building
`PlatformError::from`; `stderr_outlets::report_startup_line_renders_ingress_like_its_siblings`;
`scratch_config`; `lib.rs`'s counting header; `nix/module.nix`'s directives and
exception paragraph; `PlatformError`'s `#[non_exhaustive]` and
`From<String>`, and `loop_error`'s assignments in the vendored winit backend.
The only production-code mentions of `run_event_loop_until_quit` outside
`start` are doc comments (`StartupError::Platform`, `controller.rs`), which
`code_of` strips.

**What failed — P-1, a design defect: `exit_codes::an_unbindable_ingress_path_exits_2`
is a proxy.** As specified it asserts the status and nothing on standard error
(`draft-spec.md` §7 R-4's row says so in terms). But a configuration that gets
**past** the ingress step exits 2 as well, at `PromptWindow::new`, whenever no
display is reachable — so the case is green whether or not the ingress bind is
what failed. Measured with `target/debug/goad` against scratch configurations:

| configuration | environment | status | stderr |
|---|---|---|---|
| ingress path is a regular file | display present | 2 | `goad: <path>: not a socket — found a regular file` |
| ingress path bindable | `WAYLAND_DISPLAY`, `DISPLAY` unset | 2 | `goad: the display could not be opened: … neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set.` |

Consequences in the approved artefacts:

- `design.md` §9's mutation *"pointing the new case at a bindable path must red
  it"* is false headless (green, row 2 above). On the machine the gate actually
  runs on — `WAYLAND_DISPLAY` is set in the dev shell — the mutant launches a
  real host, and `process::goad`'s `Command::output()` has no timeout: the
  gate **hangs**, it does not red.
- `canon-delta.md` Change 1 cites the case as holding R-4's non-zero exit for an
  ingress bind failure; as specified it holds *some startup failure*, which the
  existing cases already held.
- Adjacent, not a new defect: `exit_codes.rs`'s module doc and
  `tests/binary/main.rs`'s say a case reaching step 5 *would red on every
  machine the gate runs on*. On this machine it would hang. The first of the two
  is rewritten by this slice anyway.

**A repair to put to the user, not taken.** Either (a) the case also asserts the
ingress arm's own line — the *says only what its own arm says* pattern its
siblings already follow — which amends `draft-spec.md` §7's R-4 row, `design.md`
§9, and possibly Change 1's wording; and/or (b) the binary tier's spawn strips
`WAYLAND_DISPLAY`, `WAYLAND_SOCKET` and `DISPLAY`, so no case and no mutant can
reach a real display — which touches `tests/binary/process.rs`, outside
§Scope. (a) is what kills the proxy; (b) is what makes the mutation a red and
not a hang.

**Minor, for the plan once design settles (not a stop).** `Launch`'s doc in
`startup.rs` says Help and Version are outcomes *"so `main` keeps its single
exit-code decision"*. After the slice `main` has no decision; `exit::status`
does. `design.md` §5.2's `startup.rs` list does not name this sentence. It is
inside a declared surface, so a phase can repair it.

**Still owed from the design handover:** `Cancel::is_stopped`, `exit::ended`
and the `Option` arms have never compiled in the tree (stand-in types only);
§9's mutations, F-63's split-arm mutant among them, are unrun. Both become
PHASE-01 STOP conditions once the plan is written. `research.md`'s count
(*"its five tests"*) is an audit sweep.

# Research — Slice 010

**Producers:** scoping session, 2026-09-23 — journal read on Sleipnir, source
read at `40caa4a`.
**As of:** 2026-09-23 · `40caa4a`

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
**by symbol** — `CLAUDE.md` §Working here, *name, never count*. The template
this file was copied from still says `path:line`; that sentence is itself an
instance of the rot the rule was written against, and is a follow-up rather
than this slice's to fix.

## Thread 1 — governing canon

### Binding

- **Nothing states the host's exit statuses.** ✓ Grepped `docs/specs/`,
  `docs/policy/` and `docs/adr/` for `exit`: the only host-side mentions are
  SPEC-003/R-4's *verification* prose and SPEC-001/R-40 and R-44, which are
  about the **backend's** exit status, not the host's. The numerals live in
  `crates/goad/tests/binary/exit_codes.rs`, in `StartupError`'s doc comment,
  and in `nix/module.nix` — a test, a comment and a consumer, with no
  normative statement behind any of them.
- **SPEC-003 §6.3's "failure vocabulary" is the ingress refusal-reason set.** ✓
  It is a closed set of machine-readable reasons carried in a reply to a
  writer, and says nothing about process exit. FU-1 priced this slice as
  reaching it; that pricing is wrong about the home.
- **SPEC-001 §2 puts the `goad emit` command line out of scope.** ✓ So
  `goad-emit`'s own 0/1/2 is owned by no spec either.
- **ADR-001 one-way strata.** The exit decision is stratum 3 (`crates/goad`)
  and names Slint types; nothing here reaches `src/semantics/`.

### Checked, not applicable

- **SPEC-002 (scheduling).** The exit decision is outside the loop it governs.
- **POL-001 (the phase gate).** No new gate instrument; the slice adds cases to
  two existing targets.
- **ADR-004 (firing spacing), ADR-005 (envelope normalization).** Untouched.

### Amendment candidates

- **SPEC-003/R-4's verification cell is stale.** ✓ It says *"The non-zero exit
  is review, not a test: no test target links the binary"*. **The literal
  sentence is still true and was never the point**: `process::goad` spawns
  `env!("CARGO_BIN_EXE_goad")` as a subprocess (`crates/goad/tests/binary/
  process.rs`), so nothing links the binary and nothing can. Linking was a
  proxy for *no test can observe the exit status*, and **that** is what slice
  006 falsified — its cases run the built binary and read the number a caller
  sees. A delta that corrects the sentence to a more accurate claim about
  linking would be repairing a distinction that was never load-bearing.
- **SPEC-003/R-4's same cell asserts the conflation as a virtue:** *"`main`'s
  single `match run()` maps every `Err` to exit 2"*. That sentence is the
  defect, written into canon as a fact.

## Thread 2 — code map

### Hotspots

| symbol | why |
|---|---|
| `main` (`crates/goad/src/main.rs`) | the single `match run()` that chooses the numeral |
| `run`, `start` (same file) | `start` ends on `run_event_loop_until_quit().map_err(StartupError::Platform)` — the mis-typing |
| `StartupError` (`crates/goad/src/startup.rs`) | its doc says "every way `run` can fail **to reach the event loop**", which `start`'s last line falsifies |
| `report_startup_line`, `report_platform_line` (`crates/goad/src/diagnostics.rs`) | the stderr wording for each class |
| `nix/module.nix` | `RestartPreventExitStatus = 2` and the exception paragraph |
| `crates/goad/tests/binary/exit_codes.rs` | holds the constant for the reachable classes |
| `crates/goad/tests/renderer/startup.rs` | holds the arms, one tier down |

### Cited facts

- ✓ `StartupError::Platform` is raised at **four** call sites in `start`:
  `PromptWindow::new`, `slint::set_xdg_app_id`, `Tray::new`, and
  `run_event_loop_until_quit`. The first three are *never started*; the last is
  *stopped running*. One variant, two phases.
- ✓ `slint::PlatformError` is `#[non_exhaustive]` (`i-slint-core-1.17.1`,
  `api.rs`, vendored citation) and the observed value is its `Other(String)`
  arm, so **the two phases cannot be told apart by inspecting the error.** The
  distinction has to be structural — which call site produced it.
- ✓ `PlatformError` implements `From<String>`, so a pure test **can** construct
  the real error value. The new class's classification is testable; only the
  call site that feeds it is not.
- ✓ Exit 0 is reached by `Cancel::stop` — `Tray::on_quit` and the window's
  `on_close_requested` (`crates/goad/src/install.rs`) — after which `serve`
  returns and `quit_event_loop` makes `run_event_loop_until_quit` return `Ok`.
  **A person asking is the only way the *shipped host* reaches 0, and that is
  not a property of the type.** `serve` returns on either arm of
  `controller::Ending`, and the second — `Closed`, every `Command` sender
  dropped — reaches `quit_event_loop` too, with nobody having asked. No
  production path reaches it because the senders outlive the loop (006/F-R9),
  which is an argument about code and not an assertion any test makes. It bears
  on the draft spec's R-1, which is an *if and only if*.
- ✓ `report_platform` already exists for a platform failure **during** the run
  that does *not* end the process ("the window could not be drawn"). So a
  during-the-run platform vocabulary is precedent, not invention.
- ✓ `nix/module.nix`'s `Service` block sets `Restart = "on-failure"`,
  `RestartPreventExitStatus = 2`, `RestartSec = 2`, and `Install.WantedBy =
  graphical-session.target`.

### Precedents

- **An unreachable clause is declared, not passed over.** SPEC-003/R-3's
  `LivenessUnknown` and R-4's non-zero exit each say in §7 what review holds
  and what no test can reach (004 `review-code.md` F-26). The new class takes
  the same form.
- **Two tiers, one contract.** `tests/renderer/startup.rs` holds the *arms*,
  `tests/binary/exit_codes.rs` holds the *constant*. 006 `review-code.md` F-1
  measured the gap the second closes.

## Thread 3 — field evidence

Read from the running user service's journal on Sleipnir, 2026-09-23. ✓

**Six exits, not four.** FU-1 says "four such exits in two days". Between
2026-09-21T14:23 and 2026-09-22T14:04 the journal shows six, every one the same
line:

```
goad: the display could not be opened: Error running winit event loop: Exit Failure: 1
goad.service: Main process exited, code=exited, status=2/INVALIDARGUMENT
```

So the premise is confirmed at the source: the exit is `StartupError::Platform`
from `run_event_loop_until_quit`, not a panic (which would be 101) and not a
startup failure.

**`RestartPreventExitStatus=2` suppressed the restart in all six.** What
recovered the four fast cases was not systemd's restart logic but
`Install.WantedBy = graphical-session.target`: the session cycled and re-wanted
the unit.

| exit | back at | gap | session target cycled? |
|---|---|---|---|
| 09-21 14:23:12 | 14:23:31 | 19s | yes |
| 09-21 14:32:53 | 14:33:02 | 9s | yes |
| 09-22 00:56:06 | 00:56:26 | 20s | yes |
| 09-22 09:11:04 | 11:23:27 | **2h 12m** | **no** |
| 09-22 11:25:40 | 11:25:53 | 13s | yes |
| 09-22 14:04:40 | 16:03:45 | **1h 59m** | **no** |

**The diagnosis in FU-1, the roadmap and `nix/module.nix` is wrong about the
cause of the harmful cases.** All three say *a compositor going away*. In both
two-hour outages the compositor did **not** go away — no `Stopped target mango
compositor session`, no `Stopped target Current graphical user session`. What
the journal shows instead is goad's own Wayland connection breaking:

```
09:11:04  goad[1053120]: Io error: Broken pipe (os error 32)   ×3
09:11:04  goad[1053120]: goad: the display could not be opened: …
```

The compositor was up the whole time, so **a restart two seconds later would
have succeeded**. The repair converts each two-hour outage into a two-second
one.

The fast cases are the opposite shape: `Stopped target mango compositor
session` immediately before the exit, and recovery when the target came back.
There a restart at +2s would find no compositor, fail in `PromptWindow::new`,
and exit *never started* — after which the session target recovers it exactly
as it does today. The repair is neutral in that shape, and never worse.

## Cross-thread findings

- **The measured harm and the phase cut agree.** The two damaging cases are
  precisely *stopped running with the display still there*, which is the class
  the new status names, and the class `Restart = "on-failure"` already handles
  once the numeral stops colliding with *never started*.
- **No change to the module's directives is needed — only to its reasoning.**
  `Restart = "on-failure"` with `RestartPreventExitStatus = 2` restarts an exit
  1 as written. The numeral 2 keeps its meaning and its tests. What
  changes in `nix/module.nix` is the comment: the exception paragraph goes, and
  the remaining argument stops claiming *these cannot succeed on a retry* —
  which is false for `Runtime` and for `Ingress` in-use — and says what it
  means instead.
- **A second-order risk the evidence bounds.** If a compositor cycle and a
  restart race, the restart finds no display and exits *never started*, which
  is suppressed. The journal says that shape already self-recovers by the
  session target in 9–20s, so it is an open question for design rather than a
  defect this slice must close.

## Design-input deltas

- FU-1's row needs three corrections at close: the count (six, not four), the
  cause (a broken connection, not a departing compositor), and the mechanism
  that recovered the fast cases (the session target, not systemd).
- The draft spec's §7 gains an evidence row rather than only tests: the audit's
  own observation on the running host, per `docs/AGENTS.md` §Tiers.

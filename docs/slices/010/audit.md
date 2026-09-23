# Audit & reconciliation — Slice 010

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `b444c6a..f9620b6` on `main` — `b444c6a` is the slice's first
commit (design approved), `f9620b6` the hand-over to audit. The code under
audit is PHASE-01 `ab5604f`, PHASE-02 `5b23720` and PHASE-03 `c67dd9c`.

**Question:** the slice is finished when all of these hold.

1. **The gate is green on the tree as committed** — `just check` exits 0, and
   the figures are re-measured here, not taken from `notes.md`.
2. **Every acceptance criterion in `slice-010.md` is met by something in the
   tree**, named by symbol: the draft spec's phase cut and §Owns (AC-1, AC-2);
   `StartupError` no longer carrying the loop's ending (AC-3); the pure exit
   decision and its one-tier-down cases, including a real
   `slint::PlatformError` (AC-4, AC-11); `exit_codes.rs`'s existing cases
   byte-identical outside the module doc (AC-5); the *stopped running* line
   (AC-6); `nix/module.nix`'s paragraph gone with all three false claims
   (AC-7); the SPEC-003 cells, which are canon-delta only until promotion
   (AC-8); the running-host observation (AC-9) and FU-1 (AC-10), which are
   audit/close work and are reported as pending, not met.
3. **Every VT/VA/VH in `plan.md` is discharged against the tree** — each
   phase sheet's claim is checked by reading the named symbol or re-running
   the named command; a mutation is re-run only where the claim cannot be read
   off the code.
4. **Nothing was touched that no phase declared.** `git diff --stat` per
   phase commit against that phase's Surfaces; every undeclared path named.
5. **The invariants hold at the new code**: no domain vocabulary in the new
   module; `src/semantics/` untouched (ADR-001); a backend or display failure
   still never leaves the host unable to be invoked again — here, that *stopped
   running* is not suppressed by the unit's `RestartPreventExitStatus`.
6. **The record can be made true**: every document the slice must change is a
   Reconciliation row, and every finding carried from the phases has a
   recommended disposition.

**Checked here:** 1–4, 6 fully; 5 by grep and reading the unit file. **Not
checked here:** AC-9 needs a person on the running host — this audit writes
the steps, it does not observe the result. Adversarial code review is the
separate `review-code.md` ledger. Canon is drafted as rows and not applied: it
waits for the user's endorsement.

<!-- This is the audit's scope — evidence, criteria, canon. The code review's
     own lines of attack belong in `review-code.md`'s Brief, not here. -->

## Evidence

<!-- What was run and what it said. Not a claim of correctness — the basis for
     one. -->

Gathered at `58df7c3` (the brief's commit, documentation only on top of
`f9620b6`). Every figure below was re-run or re-read here, not copied from a
phase sheet; where a sheet's record is the only evidence, the row says so.

### Tests / checks

`just check` — **exit 0**. It runs `cargo build --workspace`, `cargo test
--workspace`, `cargo test -p goad-semantics`, `deno check
examples/typescript/backend.ts`, `cargo clippy --workspace --all-targets -- -D
warnings`, `cargo fmt --all --check` (the sequence `just -n check` prints).

| figure | value |
|---|---|
| test results summed over every `test result:` line | **633 passed, 0 failed**, 0 ignored |
| of which `cargo test --workspace` | 598 |
| of which `cargo test -p goad-semantics` (counted again) | 35 (30 lib + 5 `protocol`) |
| `goad` lib | 59 |
| `goad` `tests/renderer` | 221 |
| `goad` `tests/binary` | 7 (re-run alone as well: 7 passed) |
| `goad-boundary` `tests/checks` | 46 |
| clippy `-D warnings` / fmt | no output — clean |

Matches the orchestrator's re-measure at `c67dd9c` (`notes.md` §Handover)
figure for figure. The table is a snapshot at `58df7c3`, before the code
review's repairs.

**At close** (2026-09-24, the tree of the `010: close` commit, re-run by the
closing agent): `just check` **exit 0** — **642 passed**, 0 failed, 0 ignored,
over 31 `test result:` lines. The rise from 633 is the repairs' cases
(`notes.md` §Handover, *Repairs, round 1* and *round 2*); clippy and fmt clean.

### Acceptance criteria

| AC | state | evidence, by symbol |
|---|---|---|
| AC-1 | **met** (draft) | `draft-spec.md` §4 R-1/R-2/R-3 cut on phase — 0 as asked, 1 stopped running, 2 never started; §2 *Out of scope* and §3 P-D put restart policy with the supervisor. Canon only at promotion (Reconciliation). |
| AC-2 | **met** (draft) | `draft-spec.md` **Owns** is *the exit status of this project's binaries*; §2 *Boundaries* says `goad-emit` is nominally owned and not yet governed; §4's head restricts every requirement to the host. |
| AC-3 | **met** | `start`'s last two statements are `let call = slint::run_event_loop_until_quit();` and `Ok(exit::ended(call, stop_signal.is_stopped()))` — no `StartupError` on the loop's end. `StartupError`'s type doc says the loop's ending travels in `Ended`; `StartupError::Platform`'s doc names `set_xdg_app_id`, `PromptWindow::new`, `Tray::new` and not the loop call. Held after the slice by `structure::the_loop_s_ending_is_never_a_startup_failure`. |
| AC-4 | **met** | `main` is `run()` → `diagnostics::report_exit` → `ExitCode::from(exit::status(&outcome))`; `exit::status` is pure over `&Result<Ended, StartupError>`. Every shape asserted in `exit_status` (`as_asked_is_0`, `stopped_running_is_1` over a real `slint::PlatformError::from`, `stopped_running_with_no_error_is_1`, `every_startup_failure_is_2`); the `Err` arm is `Err(_) => 2`, reading no variant. |
| AC-5 | **met; waived for doc comments** (user, `design-log.md` *at audit*: AC-5 protects the cases' behaviour, so a false doc comment on an existing case is repaired rather than kept — P3-a, `review-code.md` F-6) | `git diff b444c6a..HEAD -- crates/goad/tests/binary/exit_codes.rs` has **no removed line outside the `//!` module doc**; additions are `an_unbindable_ingress_path_exits_2` and a helper, `scratch_path` (see VT/VA, PHASE-03 EX-3). All pre-existing cases green. **Carried finding:** one existing case's doc comment is now false and AC-5 forbids the edit (Reconciliation, P3-a). |
| AC-6 | **met at the renderer tier; the process half is AC-9** | `diagnostics::report_exit_line` answers `goad: the host was running and stopped: {error}` and `…stopped, and no error was reported`; `stderr_outlets::the_stopped_line_is_not_the_line_a_host_that_never_started_writes` asserts both differ from `report_startup_line` over `StartupError::Platform`. `main` writes it through `report_exit`. |
| AC-7 | **met in its letter; one doubt raised** | `nix/module.nix`'s `Service` comment: no exception paragraph, no *do not succeed on a retry*, no `SPEC-003`, no `Platform`; argues from phase. Directives byte-identical (`git diff -U0 … \| grep '^[-+][^-+]' \| grep -v '^[-+] *#'` empty; `nix-instantiate --parse` exits 0). **Doubt, A-1:** its *"so a restart changes nothing a person has not changed first"* is itself a retryability claim, the class AC-7 calls false of `Runtime` — though `plan.md` PHASE-03/EX-4 dictated those words. |
| AC-8 | **met** — applied at `cc0db76`, endorsed (`design-log.md`, *at audit*) | SPEC-003 §7 R-4's cell names `exit_codes::an_unbindable_ingress_path_exits_2`, which exists and passes, and no longer claims no test target links the binary; R-3's *"same position"* narrowed to R-5's process exit (Reconciliation C-3…C-5). |
| AC-9 | **met** — observed 2026-09-23 | On the running host, build `0.1.0 (96a1704)`: a tray quit exited `0/SUCCESS` with no line and no restart; a lost display (route 2) exited `1/FAILURE` with the one *stopped running* line last, restarted 2 s later. Record under *AC-9 — on the running host*, §Observed. |
| AC-10 | **met** — at close | `docs/follow-ups.md` FU-1 struck under §Closed, with what killed it (`exit::ended`, `exit::status`, the `nix/module.nix` comment) and its three corrections — six exits, a broken connection, the session target — plus its *SPEC-003's failure vocabulary* error (Reconciliation R-1). |
| AC-11 | **met** | `exit::ended` decides on `stop_requested` alone (`if stop_requested { AsAsked } else { StoppedRunning(call.err()) }`); the four `ended::` cases cover each result × request, the two error cases over one `loop_error()`. `start` passes `stop_signal.is_stopped()` read in the statement after the call, on a clone taken before `cancel` moves into `serve` (read here in `start`, not taken from PHASE-02/VA-1). `Cancel::is_stopped` is `*self.rx.borrow()`, held by `tests::is_stopped_is_false_until_stop_and_stays_true`. |

### Verification criteria

Each VT's case was resolved by `grep -rn "fn <name>\b" crates` — every one
named below resolves, in the file the plan names — and the gate above ran it.
Mutations are **not** re-run here, except where noted: the sheets record each
as compiled, with the red set named and a `diff`-verified restore, and the
orchestrator re-ran M-9 independently at `5b23720`.

**PHASE-01** (`ab5604f`)

| id | discharged? | how checked here |
|---|---|---|
| VT-1 | yes | `exit_status`'s four cases resolve and pass; `stopped_running_is_1` builds via `slint::PlatformError::from`; `every_startup_failure_is_2` names variants, no count. |
| VT-2 | yes | `ended`'s four cases resolve; the two error cases share `loop_error()`; the carried error compared by `to_string()`. |
| VT-3 | yes | `stderr_outlets`' five named cases resolve and pass. |
| VT-4 | yes | `wire::tests::is_stopped_is_false_until_stop_and_stays_true` resolves and passes (`goad` lib 59). Sits before the back-pressure divider, not beside the named sibling — a recorded, sound decision. |
| VA-1 | yes | clippy `-D warnings` over the real types, clean in this gate run. |
| VA-2 | yes | the domain scan (`vocabulary::no_workspace_member_names_the_users_domain`) walks each member's directory excluding `tests`, so `crates/goad/src/exit.rs` is in its reach; it is green. By hand: no `DOMAIN` word in any code token or literal added under `crates/goad/src` — the only hits are *call site* in doc comments, which the scan cuts. |
| EX-5 (M-1…M-8) | yes, per the sheet | each compiled, each red set named and exact. Not re-run. |
| EX-6 | yes | every `module::case` citation in `draft-spec.md` and `canon-delta.md` resolves (all of them, not only PHASE-01's). |

**PHASE-01 finding, verified as stated:** `exit::status`'s match is not held
by the crate-root `wildcard_enum_match_arm` deny — not re-measured here; the
sheet's negative control compiled. Carried (P1-a).

**PHASE-02** (`5b23720`)

| id | discharged? | how checked here |
|---|---|---|
| VT-1 | yes, per the sheet | the red is quoted in T-2 (`main.rs`'s `.map_err(StartupError::Platform)?;` line, on the shape assertion); green now. |
| VT-2 | yes | `counting_itself::the_bare_loop_call_ends_at_the_call` and `…a_loop_call_with_its_result_re_filed_does_not` resolve and pass; the latter covers both the `map_err(…)?` and bare-`?` spellings. |
| VT-3 | yes | binary tier green; `exit_codes.rs` untouched at `5b23720` (`git show --stat` lists no binary-tier path). |
| VA-1 | yes, re-read here | (a) and (b): `start`'s last two statements as quoted under AC-11. (c) `grep -rn 'StoppedRunning(' crates/goad/src` — one construction, in `exit::ended`; the rest are the declaration and patterns in `exit::status` and `report_exit_line`. (d) both readers answer `u8` / `Option<String>`; `run` and `start` only wrap `Ended` in `Ok`. |
| VA-2 | yes | `grep -rn 'report_startup\b' crates` finds nothing (only `report_startup_line`). The sheet's narrowing of this criterion caused a false word — carried (P2). |
| EX-7 (M-9…M-11) | yes, per the sheet | M-9 re-run by the orchestrator. |
| EX-8 | yes | `structure::the_loop_s_ending_is_never_a_startup_failure` resolves. |

**PHASE-03** (`c67dd9c`)

| id | discharged? | how checked here |
|---|---|---|
| VT-1 | yes | `exit_codes::an_unbindable_ingress_path_exits_2` resolves and passes; asserts status 2 and the prefix `goad: <socket>: `. **Record gap:** T-3's red-first quote (a wrong prefix, then corrected) is not in the sheet; M-13 and M-14 each redding the case on its prefix are equivalent evidence that it asserts. |
| VT-2 | yes | binary tier 7 green with `process::command` removing the display variables. |
| VA-1 | yes, per the sheet | `process::command` removes `WAYLAND_DISPLAY`, `WAYLAND_SOCKET`, `DISPLAY`, doc says why; M-13 measured 0.545s wall with the display line on stderr. Not re-run (it would need the bindable-path edit). |
| VA-2 | yes, re-read here | `exit_codes.rs`'s doc states the cut from the process side, no *cannot see the constant*, no *would red*; `tests/binary/main.rs`'s doc says what the spawn guarantees. One stale phrase in the latter — carried (P3-c). |
| VA-3 | yes, re-run here | comment-only diff; `nix-instantiate --parse nix/module.nix` exit 0. |
| EX-3 | yes, with one addition | the diff is the module doc, the case, and `scratch_path` — a helper the case needs, recorded in §Decisions. Additive; no pre-existing line removed. |
| EX-5 (M-12…M-14) | yes, per the sheet | |
| EX-6 | yes | see PHASE-01 EX-6. |

**Record gap across PHASE-03's sheet:** every task box T-1…T-12 is unticked,
though §Mutation evidence, §Decisions and the orchestrator's re-measure show
the work done. PHASE-02's T-13 (commit) is likewise unticked. Neither is a
code finding (A-2).

### Surface delta

`git diff --stat b444c6a..f9620b6`, per phase commit against that phase's
**Surfaces**:

- **PHASE-01 `ab5604f`**: `exit.rs` (new), `lib.rs`, `wire.rs`,
  `diagnostics.rs` (additions only — no removed line), `tests/renderer/startup.rs`
  — all declared. `draft-spec.md` / `design.md` not touched, correctly: no case
  was renamed.
- **PHASE-02 `5b23720`**: `main.rs`, `startup.rs` (docs only),
  `diagnostics.rs`, `tests/renderer/startup.rs` (module doc only — no changed
  line outside `//!`), `goad-boundary/tests/checks/structure.rs` — all
  declared.
- **PHASE-03 `c67dd9c`**: `tests/binary/process.rs`, `tests/binary/main.rs`
  (module doc only), `tests/binary/exit_codes.rs`, `nix/module.nix` (comment
  only) — all declared.
- **Outside any phase**: `docs/slices/010/{canon-delta,design-log,design,draft-spec,slice-010}.md`
  changed at `9f0a503`, the P-1 repair made at the plan stage before any phase,
  endorsed in `design-log.md`. The remaining commits touch only `notes.md`,
  `plan.md`, `plan-log.md`.

**Undeclared paths: none.** Declared-but-untouched: none beyond the
conditional `draft-spec.md` / `design.md` rows, which were conditional on a
rename that did not happen. `crates/goad-semantics` untouched (ADR-001).

### AC-9 — on the running host

For a person on the machine. **Not observed by this audit.** Commands are for
nu, one per line; `<PID>` and `<FD>` are typed in by hand.

The host runs as the home-manager unit `goad.service`, built from
`/home/david/flakes`, whose `goad` input is `git+file:///home/david/dev/goad`
— the **committed** `main`, so nothing here needs the working tree clean
beyond what is committed. Measured at audit: the lock pins `40caa4a`
(pre-slice), the unit is `inactive`, and `/home/david/flakes/flake.lock`
already carries an unrelated uncommitted bump of the `satan` input that the
switch below will include.

**Deploy the slice**

- [x] Update the input and switch:
  ```nu
  cd /home/david/flakes
  nix flake update goad
  just home-switch
  systemctl --user start goad
  ```
- [x] Confirm the unit runs this slice's build — the revision printed must be
  `main`'s current short hash, not `40caa4a`:
  ```nu
  systemctl --user cat goad | lines | where {|l| $l | str starts-with "ExecStart="}
  ```
  then run the printed path with `--version`, and compare with
  `git -C /home/david/dev/goad log --oneline -1`.
- [x] Confirm the directives: `Restart=on-failure`,
  `RestartPreventExitStatus=2`, `RestartUSec=2s`:
  ```nu
  systemctl --user show goad -p Restart -p RestartPreventExitStatus -p RestartUSec -p ActiveState -p MainPID
  ```

**A quit is 0, with no line** (`draft-spec.md` §7 R-1's evidence for the edge
no test reaches)

- [x] Choose **Quit** from the tray menu.
- [x] Read the unit:
  ```nu
  systemctl --user status goad
  journalctl --user -u goad --since "5 min ago" -o short-iso
  ```
  Expect: `inactive (dead)`; the exit recorded as `status=0/SUCCESS`; **no**
  `goad: ` line from that process; **no** `Scheduled restart job` after it.
- [x] Bring it back: `systemctl --user start goad`.
- [ ] *(not run)* *(optional, the other route R-1 defines as asked)* with the prompt window
  shown, close it with the compositor's close binding; expect the same as a
  quit. Then `systemctl --user start goad`.

**A lost display is 1, with the *stopped running* line, back within
`RestartSec`** (`design.md` §5.5 A2; §8 R1)

**No known, tested way to lose only the host's display connection exists.**
Killing the compositor is not it: the unit is `PartOf` the graphical session,
so systemd stops the host with the session — a different end, and it takes the
desktop with it. A compositor close binding is not it either: that is a close
request, which is *as asked* and correctly 0. The observed failure was the
host's own Wayland connection breaking (`Broken pipe`) with the compositor up
(`research.md` §Thread 3). Two routes:

- **Route 1 — wait for it.** It happened six times in two days before this
  slice. Leave the host running; after the next one, read the journal as below.
  This is the real failure and needs no instrument.
- **Route 2 — provoke it. Untested; nobody has run this.** Shut down the
  host's end of its Wayland socket from outside, with `gdb` — the compositor
  stays up and sees the client go. `ptrace_scope` is 1 on this machine, so the
  attach needs `sudo`; `gdb` is not installed, so it is built from nixpkgs.
  If this route yields anything other than exit 1 with the line, record what
  was seen: that is a finding against the method before it is one against the
  code.
  - [x] Find the process and its Wayland socket:
    ```nu
    systemctl --user show goad -p MainPID --value
    ss -xpn | lines | where {|l| $l | str contains "wayland-0"}
    ss -xpn | lines | where {|l| $l | str contains "goad"}
    ```
    The compositor's row on `/run/user/1000/wayland-0` names a peer inode;
    the `goad` row whose own inode is that peer carries `fd=<FD>`.
  - [x] Build gdb, attach, shut the socket down, detach:
    ```nu
    let gdb = (nix build nixpkgs#gdb --no-link --print-out-paths | lines | first)
    sudo $"($gdb)/bin/gdb" -p <PID> -batch -ex 'call (int)shutdown(<FD>, 2)'
    ```

**Read the result** (either route)

- [x] Read the unit:
  ```nu
  journalctl --user -u goad --since "10 min ago" -o short-iso
  systemctl --user status goad
  systemctl --user show goad -p NRestarts -p ExecMainStatus -p ActiveState
  ```
  Expect, in order: a line `goad: the host was running and stopped: …` (or
  `…stopped, and no error was reported`) from the old process;
  `Main process exited, code=exited, status=1/FAILURE`; `Scheduled restart job`;
  `Started goad` about **2 s** after the exit; the unit `active (running)` with
  a new PID and `NRestarts` one higher.
- [x] **Any of these is a finding, and the slice does not close on it:**
  status **2**, or the line `goad: the display could not be opened: …` (the
  loss reached an earlier step — A2 is wrong); status **0** with no line (the
  loss tripped `Cancel` — A5, §8 R1); no restart within a few seconds.
- [x] Record here: the date, the route, the journal lines quoted, and the gap
  between the exit and `Started`.

**Observed — 2026-09-23, by the user, on `Sleipnir`**

- **Deploy.** `ExecStart=/nix/store/yjfd4n3bx8phis2rssr2ifb7y1542xxz-goad-0.1.0/bin/goad`;
  `--version` answered `0.1.0 (96a1704)`.
- **A quit is 0, with no line.** Tray **Quit**: `Active: inactive (dead)`,
  `Main PID: 2413919 (code=exited, status=0/SUCCESS)`; the journal tail after
  `Started` holds no `goad: ` line and no `Scheduled restart job`.
- **A lost display is 1, with the line, back in 2 s — route 2.** The host's
  Wayland socket was found from the compositor's side (the `wayland-0` row
  whose peer inode is one of the host's; fd 11 of PID 2447030) and shut down
  with `gdb -batch -ex 'call (int)shutdown(11, 2)'`. The route is no longer
  untested, and it reproduces the production failure: the three `Broken pipe`
  lines are the ones `research.md` §Thread 3 quotes from the six outages.

  ```
  23:51:22 systemd[1435]: Started goad — personal intervention shell.
  23:55:41 goad[2447030]: Io error: Broken pipe (os error 32)   (×3)
  23:55:41 goad[2447030]: goad: the host was running and stopped: Error running winit event loop: Exit Failure: 1
  23:55:41 systemd[1435]: goad.service: Main process exited, code=exited, status=1/FAILURE
  23:55:43 systemd[1435]: goad.service: Scheduled restart job, restart counter is at 1.
  23:55:43 systemd[1435]: Started goad — personal intervention shell.
  ```

  Then `active (running)`, new PID 2468699, `NRestarts=1`. Exit to `Started`:
  **2 s** (`RestartSec`). The `goad: ` line is the old process's **last** —
  nothing follows it from PID 2447030, which settles review round 1's *not
  reached* (a write after `report_exit` on a real stop): none observed.

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`
- **State:** closed 2026-09-23, after three rounds and a site check ·
  outstanding blockers: **0** (none raised) · F-1…F-23 all `verified`. Its
  Synthesis is the account; nothing is restated here.

## Verdict

<!-- The slice's closure story, written once, here. Draws on the ledger's
     synthesis and on the evidence above; restates neither. Does this slice do
     what it set out to do, and what is being accepted knowingly? -->

**The slice does what it set out to do.** It opened on a host that lost its
display after hours of running, exited with the status a bad configuration
exits with, and was held down for two hours by a directive written for the
bad configuration. The status now names the phase the process ended in; the
decision is a pure function asserted one tier down for every shape it can see;
the seam where the loop's end used to be re-filed is held by a structure scan;
and the running host, provoked the way production fails, exited 1 with its line
and was back in `RestartSec`. Canon was written for it (SPEC-004) and corrected
where it had recorded the defect as fact (SPEC-003 R-4). The code review raised
no blocker; what it found was the reach of the change — stderr lines that were
several lines, and a question answered 0 when the answer never arrived — and
both were repaired inside the slice rather than deferred.

**Accepted knowingly:**

- **The seam.** An event-loop call that fails on entry is reported as *stopped
  running*, though the loop never began: the host cannot tell that call from a
  loop that ran. Declared in SPEC-004 §5 *What the seam costs*, not hidden. The
  call site that hands the loop's end to `exit::ended` is held by review, and a
  real lost display reaching that arm by the AC-9 observation alone — no test
  tier can reach it.
- **No bound on a stderr line.** A line past journald's `LineMax=` is split into
  several records, so the last record the journal shows need not begin
  `goad: `. Chosen over a bound that would cut a parser's message, which comes
  last (SPEC-004 §7 R-4's row).
- **`goad-emit`'s statuses are owned and not governed** (FU-42), and it still
  answers `--help` / `--version` with 0 when the answer was not written — the
  defect F-3 repaired in the host (FU-43). Both wait for the slice that admits
  that binary to SPEC-004.

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

Rows marked **canon** need the user's explicit endorsement before they are
written; C-1…C-6 were endorsed (`design-log.md`, *at audit*) and applied with
the round-1 repairs, as were P2, P3-a, P3-b, P3-c and A-1 (`notes.md`
§Handover, *Repairs, round 1*). A-1's recommended wording was not used
verbatim: its *rather than retry into the rate limiter* predicts that the
retry fails, the class `review-code.md` F-1 removes, so the unit comment
states the policy alone. Row ids are this table's own, for reference in the
endorsement question.

**Canon — promotion (`plan.md` §What no phase does)**

| id | document | change | reason | done |
|----|----------|--------|--------|------|
| C-1 | `docs/slices/010/draft-spec.md` → `docs/specs/004-process-exit-status.md` | promote: number it SPEC-004; replace the **Status** paragraph (*draft … Not canon … suggested slug*) with the canon status line the other specs carry; `SPEC-NNN` → `SPEC-004` throughout | drafted during this slice (AC-1, AC-2); a slice does not close holding an unpromoted draft | [x] |
| C-2 | the promoted spec, §7 | remove the `DRAFT-ONLY` comment | promotion obligation (`design.md` §10, D6). Precondition **checked here**: every `module::case` citation in `draft-spec.md` resolves in the tree (§Evidence, PHASE-01 EX-6) | [x] |
| C-3 | `docs/specs/003-host-event-ingress.md` §7, R-4's cell | `canon-delta.md` Change 1, as stated there | AC-8; the stale *no test target links the binary* and the conflation recorded as fact. The case it names exists | [x] |
| C-4 | `docs/specs/003-host-event-ingress.md` §7, R-3's cell | `canon-delta.md` Change 3: *"the same position as R-4's exit code below and R-5's process exit"* → *"the same position as R-5's process exit"* | not separable from C-3 | [x] |
| C-5 | `docs/specs/003-host-event-ingress.md` §9 | `canon-delta.md` Change 2, with `SPEC-00N` → `SPEC-004` | separable from C-3/C-4; R-4's cell defers to the new spec | [x] |
| C-6 | `nix/module.nix` comment; `crates/goad/src/exit.rs` `//!`; `StartupError`'s type doc (`crates/goad/src/startup.rs`) | add the `SPEC-004` citation | D6: no spec number until promotion, then all three sites. Source edits — land with the code-review repairs | [x] |

**Close (AC-10 and `notes.md` §Open)**

| id | document | change | reason | done |
|----|----------|--------|--------|------|
| R-1 | `docs/follow-ups.md` FU-1 | struck, with what killed it (this slice's `exit::ended` / `exit::status` and the `nix/module.nix` comment) and its three corrections: six exits not four; a broken connection, not a departing compositor; the session target, not systemd, recovered the fast cases. Its *"SPEC-003's failure vocabulary"* goes too — FU-1's own error, per AC-7 | AC-10 | [x] |
| R-2 | `docs/memory/exit-2-means-two-different-failures.md` | **rewrite, not re-quote.** `notes.md` §Open says its standing fact still holds; it does not — after this slice exit 2 means *never started* only. What survives is the lesson (an enumeration of three variants argued for ten), which its *Why the enumeration was convincing* section already carries. Title and *The fact* restated as history-free: what exit 2 means now and why it once did not belongs in the spec and the slice, so the file keeps only the lesson | stale since PHASE-02 | [x] |
| R-3 | `docs/slices/010/research.md` §Cross-thread findings | *"keeps its meaning and its five tests"* → *"keeps its meaning and its tests"* | a count falsified by `an_unbindable_ingress_path_exits_2` (`CLAUDE.md` — never count) | [x] |

**Carried findings — recommended disposition (one line each)**

| id | finding (phase) | recommendation | done |
|----|-----------------|----------------|------|
| P1-a | the crate-root `wildcard_enum_match_arm` deny does not reach `exit::status`'s match (PHASE-01) | **memory, not code**: add the cost to `docs/memory/wildcard-enum-match-arm-counts-a-named-binding.md` (matching the enclosing `Result` escapes the lint, so exhaustiveness is then held by cases, not by the deny); `design.md` §3's sentence left as written and listed under *Design drift* below | [x] |
| P1-b | `lib.rs`'s header carries `path:line` citations (PHASE-01) | **follow-up, merged**: extend FU-10's citation (citation discipline enforced by nothing) with these sites rather than a new row; not this slice's code | [x] |
| P2 | `diagnostics.rs`'s `//!` says `report_exit` was *"renamed"*; it replaced a different function (PHASE-02) | **repair in the slice**, one word: *replaced*. Via the code-review ledger | [x] |
| P3-a | `help_prints_the_usage_block_on_stdout_and_exits_0`'s doc says *"`Ok(())` is exit 0"*, false since PHASE-02; AC-5 forbids the edit (PHASE-03) | **user decision**: waive AC-5's letter for doc comments — its purpose is that no case's *assertions* change and 2 keeps its consumers — and repair the sentence (*`run` answering `Ok(Ended::AsAsked)` is exit 0*). Leaving a known-false doc to honour an AC's wording is the worse outcome | [x] |
| P3-b | `nix/module.nix`: *"0 is the window being closed, which was asked for"* — narrower than true (PHASE-03) | **repair in the slice**, comment only: *0 is as asked — a quit from the tray, the window closed, or `--help` / `--version` answered*. Lands with C-6 | [x] |
| P3-c | `tests/binary/main.rs`'s doc: startup failures settle in `start`'s *"first step"*; the new case settles at step 3 (PHASE-03) | **repair in the slice**: *before the first Slint call* (the phrase the same sentence already uses), dropping the step number | [x] |
| A-1 | **raised at audit.** `nix/module.nix`: *"2 is a host that never started … so a restart changes nothing a person has not changed first"* is a retryability claim — the class AC-7 removes as false of `Runtime`. `plan.md` PHASE-03/EX-4 dictated the words, so the executor followed the plan | **user decision, recommended repair**: state it as the unit's policy, not a fact about retries — *2 is a host that never started; this unit leaves that to a person rather than retry into the rate limiter* — matching `draft-spec.md` §3 P-D (policy is built on the statuses, not asserted by them). Lands with C-6/P3-b | [x] |
| A-2 | **raised at audit.** PHASE-03's task boxes are all unticked and T-3's red-first quote is missing; PHASE-02's T-13 unticked | **record only**: a note in `notes.md` at close that the evidence is in §Mutation evidence / §Decisions and M-13/M-14 stand in for T-3's red. No code consequence | [x] |

**Design drift not reconciled:**

- `design.md` §3 says every match the design adds is written without a
  wildcard over an enum, beside the crate's `wildcard_enum_match_arm` deny, in
  a way that reads as though the deny holds it. For `exit::status` the deny
  does not reach; the four `exit_status` cases and M-1/M-2 hold it. The design
  stays as written (a record of intent); P1-a puts the fact where the next
  reader looks.
- `design.md` §9 places `is_stopped_is_false_until_stop_and_stays_true`
  *beside* `a_raised_notice_stays_raised_until_it_is_lowered`; it sits on the
  `Cancel` side of `wire.rs`'s own divider instead (PHASE-01 §Decisions). Left
  as written; no citation depends on the position.
- `design.md` §9 / `plan.md` PHASE-03/EX-3 allow the new case and nothing else
  in `exit_codes.rs`; a helper, `scratch_path`, came with it (PHASE-03
  §Decisions). Left as written; additive, and AC-5 holds.

## Closure

- [x] All findings dispositioned; no blockers outstanding
- [x] All acceptance criteria met, or explicitly waived by the user
- [x] Each verification criterion in `plan.md` walked against the code, or the gap measured and carried
- [x] Tests and checks green
- [x] Specs / policy / ADRs reconciled, with user endorsement where amended
- [x] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [x] `notes.md` §Open swept against `slice-010.md` §Follow-ups; every entry dispositioned
- [x] `slice-010.md` Summary and Follow-ups written
- [x] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [x] `slice-010.md` stage set to `done`

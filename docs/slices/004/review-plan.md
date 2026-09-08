# Review — plan — Slice 004

**Subject:** plan — `docs/slices/004/plan.md` (seven phases), with
`plan-log.md` PL-1..PL-9
**Reviewer:** fresh agent (Claude Opus 5), adversarial raiser
**Opened:** 2026-09-08
**State:** open

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

Written before reading `plan.md`, from the design, the slice card, the draft
spec, the canon delta, `review-design.md`'s Synthesis and F-15, and
`docs/AGENTS.md` §Plan / §Phase plan / §Execute.

A design review asks *is this right*. This review asks **will this happen**.
Two questions only:

1. **Executability.** Can one agent, in one session, having read only the design
   and its own phase, discharge that phase and end green? AGENTS.md §Execute
   binds each phase to one agent and one session, red/green/**refactor**, ending
   with tests passing and nothing half-applied. A phase that cannot fit is not a
   plan, it is a wish.
2. **Coverage.** AGENTS.md §Plan: *entry / exit criteria chosen such that if
   they are completed, the intent of the slice and the design will be observed.*
   So the union of the seven phases' exit criteria must reach all thirteen ACs,
   the design's §9 validation table, `draft-spec.md`'s requirements, and CD-1/
   CD-2/CD-3's verification promises — and each phase's entry criterion must be
   established by some earlier phase's exit.

**Invariants held over the plan**

- **CLAUDE.md's five.** No domain vocabulary in host names; permissive wire /
  canonical internals with normalization the only door; no narrowing of wire
  compatibility to the renderer's subset; a backend (or a writer) failure never
  takes the host down; ADR-001's one-way strata, with `cargo test -p
  goad-semantics` and the other three instruments as the gate.
- **POL-001.** `just check` is the gate; four ADR-001 instruments plus the
  vocabulary scan plus one residue, and no phase may end not-green.
- **The ledger Protocol above**, in particular `settle-in-code`: F-15 was
  dispositioned that way in `review-design.md` and owes a named phase and a
  named test, and returns `contested` if it survives its phase.

**Where I expect the bodies**

- **Phase sizing.** Slice 003's phases are the only calibration this repo has;
  PHASE-03 (bind, listener, framing, budgets, reply, socket lifecycle) and
  PHASE-04 (two `select!` arms, an anchor, `Fired::Ingested`, `Refused::Ingress`,
  23 call sites, plus AC-5's measured presentation count) are the two that look
  like more than one session.
- **Criteria that do not bind.** An exit criterion checkable only by the agent
  that wrote it; an entry criterion the previous phase's exit does not in fact
  establish.
- **The AC map.** Claimed complete for all thirteen. An AC assigned to a phase
  whose deliverables cannot reach it, or covered only by a test named and not
  specified, is a planning defect.
- **PHASE-01, the A-1 probe.** R1 says the slice does not work at all if A-1
  fails. The probe must measure the actual configuration (a `tokio::spawn`ed
  task accepting while Slint owns the main thread under the runtime guard), say
  what falsifies the design, say what happens then, and leave the tree as it
  found it. A decorative failure branch is worse than no probe.
- **F-15's settlement.** PHASE-04/VT-5 is claimed to be it. Can that test bound
  presentations at all, and does the phase know what to do if the number is bad?
- **AC-6 case (ii).** The setup must pin the *ingested* exchange's own
  `next_check`, or the test passes under ADR-004's boolean alternative too and
  discharges nothing. The plan claims the test would go red under the boolean;
  that claim is checkable and will be checked.
- **FD-1..FD-5.** Findings the plan raised against the design and recorded
  rather than escalated. Each rests on the claim that the design's reasoning
  already implies one resolution. That claim is tested one at a time — FD-3
  hardest, because it says `Config` gaining a field breaks struct literals that
  AC-7's reading did not count.
- **PL-4 and PL-6** — a manifest feature entry and a new named function — both
  touch surfaces the four ADR-001 instruments watch.
- **The gate versus the demo.** AGENTS.md: slices 001-003 closed green on a
  binary that could not open a window. PHASE-06 claims a person runs it; is what
  they would see the new behaviour, and can PHASE-06 be reached without it?
- **Briefing quality.** Every `path:line` range a phase agent is sent to is
  spot-checked against the code. A wrong range sends an agent to the wrong place
  with no signal that it is wrong.

Nothing is raised on assertion. Every finding cites a line, a criterion, an AC
or a rule.

**Round 1** — 2026-09-08 — executability and coverage of the seven-phase plan,
against the design, the thirteen ACs, canon, and the code as it stands at
`b6ca5f7`.

**Checked hard and found sound** (recorded so a later round does not re-spend
the budget on them):

- **The 23 `serve` call sites.** Enumerated against the tree: exactly 23, and
  every line number in PHASE-04/EX-7 matches — `main.rs:103`;
  `event_loop/closing.rs:82`; `event_loop_schedule/scheduling.rs:110`;
  `wiring.rs:915, 992, 1053, 1108, 1155, 1187`; `scheduling.rs:166, 207, 245,
  290, 335, 390, 451, 505, 572, 644, 687, 755, 809, 874`.
- **EX-1's `Served` claim.** `Served` is mentioned outside `controller.rs` at
  exactly two places, both doc comments (`wiring.rs:899`, `:1088`). Nothing
  destructures it, so adding a field is safe.
- **FD-3's four `Config` struct literals.** `tests/support/driving.rs:46`,
  `renderer/scheduling.rs:84`, `event_loop/closing.rs:63`,
  `event_loop_schedule/scheduling.rs:91` — all four confirmed, and no assertion
  sits on the added field.
- **PL-4 is invisible to the manifest allowlist.** `unpermitted`
  (`goad-boundary/src/manifest.rs:25-66`) reads dependency **keys** and a
  `package` override only; a `features` array is never inspected. `tokio` is
  already on `STRATUM_2`. FD-1's conclusion holds.
- **FD-2 is right about `tempfile`.** `crates/goad-shell/Cargo.toml` has no
  `[dev-dependencies]` table at all, and the precedents PL-3 names
  (`config.rs:226`, `scripting.rs::marker`) are real.
- **PHASE-07/VA-2's vocabulary-scan claim.** `vocabulary.rs`'s scan is
  member-enumerated through `members(&root_manifest)`
  (`goad-boundary/src/members.rs:21`), so `crates/goad-shell/src/ingress/`
  is covered with no hand-listed path. None of the nine names VA-4 lists is on
  `vocabulary.rs:18-26`'s `DOMAIN`.
- **PHASE-05/VT-2's discrimination.** Traced through
  `controller.rs:505-513`. Under the anchor, both the scheduled and the
  ingested exchange re-arm through `deadline_after(…, floor_until)`, so the
  scheduled firing waits to T₀+3 s; under ADR-004's rejected boolean the
  intervening ingested firing clears the floor and the pinned short
  `next_check` fires at T₀+1 s. The case does distinguish, and EX-2's clause
  about the ingested exchange's own deadline is genuinely load-bearing.
- **PHASE-05/VT-4 is buildable.** `SlintGlass::present` writes
  `set_diagnostic_lines` unconditionally (`glass.rs:102`), and
  `app.slint:21` declares the property, so `get_diagnostic_lines()` is readable
  in any window mode.
- **PHASE-01's reversibility claim is narrow and true.** EX-3 claims only that
  `crates/goad/Cargo.toml` is byte-identical and no untracked file survives but
  the gitignored probe (`.gitignore` does carry `*.local.*`); it does not claim
  the tree is unchanged, and `research.md`/`notes.md` edits are declared
  surfaces. PHASE-02/EN-1 gates on the verdict being *A-1 holds*, so nothing
  downstream assumes the probe passed silently.
- **PHASE-01/S-1 is a real branch.** It gives a numeric red line (>100 ms
  under VT-4 with nothing else armed), a negative control that is the decisive
  case, and two named repairs that both go back to design rather than into the
  phase.
- **PHASE-02/VT-1's care with `an_unknown_key_is_refused_and_named`.** That
  fixture plants `socket = "/tmp/goad.sock"` after `timeout = "5s"`, i.e. under
  `[backend]` (`config.rs:247-255`), so the new `[ingress]` section does not
  make it stale. Correct.
- **F-15's settlement is reachable in principle.** `controller.rs:474-483`'s
  refusal site `continue`s to `:410`, whose first statement is
  `glass.present`, so one refusal does cost exactly one presentation and a
  decorator over the real `SlintGlass` can count it.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | |
| F-2 | major | fix-now | |
| F-3 | major | fix-now | |
| F-4 | major | fix-now | |
| F-5 | major | fix-now | |
| F-6 | major | fix-now | |
| F-7 | major | fix-now | |
| F-8 | minor | fix-now | |
| F-9 | minor | fix-now | |
| F-10 | minor | fix-now | |
| F-11 | minor | fix-now | |
| F-12 | minor | fix-now | |
| F-13 | nit | fix-now | |
| F-14 | nit | fix-now | |

### F-1 — PHASE-06/EX-2 and EX-3 cannot both hold: the config is moved into `Host` before the point EX-3 puts the bind

**Severity:** major
**Location:** `plan.md` PHASE-06/EX-2, EX-3; PL-6; `crates/goad/src/main.rs:51-63`

**Expected:** EX-2 lands
`pub fn listener(configured: Option<&IngressConfig>) -> Result<Ingress, StartupError>`
with `main::start` calling it, and EX-3 puts that call **after** the runtime
guard at `main.rs:63` and before `slint::spawn_local` at `:102`, "immediately
after `runtime.enter()`, ahead of the window and tray".

**Observed:** `config` is **moved** into `Host::new(config, backend, now)` at
`main.rs:54`, nine lines before the runtime guard is taken at `:63`. At the
point EX-3 names there is no `Config` in scope, so no `Option<&IngressConfig>`
can be formed. `Host` stores it privately (`crates/goad-shell/src/host.rs:115`,
`config: Config`) and exposes no accessor, and `Config` derives `Debug` only
(`config.rs:27-28`) — there is no `Clone` to take a
copy with before `:54`, and `IngressConfig` is not required to be `Clone` by
PHASE-02/EX-1 either.

Every route out of this is closed to PHASE-06 as written. Adding an accessor to
`Host`, or `Clone` to `IngressConfig`, is `crates/goad-shell/src`, which
PHASE-06's *Must not touch* forbids in terms. Hoisting the runtime construction
above `Host::new` is inside `main.rs` and would work, but the plan neither
authorises nor anticipates it, and PHASE-06's own implementer note calls
`main::start`'s numbered comment blocks "load-bearing documentation" — a
reordering of steps 1 and 2 is not the "add the bind as its own numbered step"
that note describes. No STOP condition covers it: S-1 fires only if `bind`
"needs something the reactor guard does not provide", which is not what is
wrong here.

**Evidence:** `crates/goad/src/main.rs:51-63`;
`crates/goad-shell/src/host.rs:115` and `:127`;
`crates/goad-shell/src/config.rs:27-28` (`Config` derives `Debug` only, no `Clone`);
`plan.md` PHASE-06 *Surfaces* / *Must not touch*.

**Disposition:** fix-now
**Response:** Verified against the tree: `config` is moved at `main.rs:54`, the
guard is taken at `:63`, `Host` stores it privately (`host.rs:115`) with no
accessor, and `Config` derives `Debug` only (`config.rs:27-28`). The design's
placement survives — the bind still goes immediately after `runtime.enter()`,
ahead of the window and tray — so the repair is to the plan, not to the design.

PHASE-06/EX-3 now states the re-sequencing explicitly: **one moved line inside
`main.rs`**, this phase's own surface. `let host = Host::new(config, backend,
now);` moves from `:54` to below `let _entered = runtime.enter();` and below the
`listener` call, so `config.ingress.as_ref()` is still formable at the point the
bind happens. `Host::new` is pure construction — it stores three arguments and
calls `schedule::resolve` (`host.rs:127-134`) — so it neither needs nor minds
the guard, and nothing between the two positions reads `host`. EX-3 also names
the three routes the phase may **not** take instead: an accessor on `Host`,
`Clone` on `Config`/`IngressConfig` (both `crates/goad-shell/src`, forbidden),
or hoisting the runtime construction above `Config::load` (two numbered steps
reordered for no gain). The implementer note about `main::start`'s numbered
comment blocks now says step 1's *"the config is then moved into the host"* is
re-worded and the blocks renumbered.

**Outcome:**

### F-2 — PHASE-06/VT-1 and FD-5 rest on a `#[cfg(test)] mod tests` in `startup.rs` that does not exist

**Severity:** major
**Location:** `plan.md` FD-5; PHASE-06/VT-1; `crates/goad/src/startup.rs`

**Expected:** FD-5: PL-6 gives AC-7's second half a test "in `startup.rs`'s own
`#[cfg(test)] mod tests` — **the same shape `arguments` already has there**".
PHASE-06/VT-1: "`startup.rs`'s **existing** `#[cfg(test)] mod tests` gains: …".

**Observed:** `crates/goad/src/startup.rs` contains no `#[cfg(test)]` at all.
The only `#[cfg(test)] mod tests` blocks in `crates/goad/src` are
`controller.rs:530`, `wire.rs:188` and `diagnostics.rs:457`. `arguments` is
tested from `crates/goad/tests/renderer/startup.rs` — an integration target,
not an in-file module — so the precedent FD-5 invokes points the other way, and
the convention `controller.rs:524-530` states for an in-file module is
explicitly "the crate-external `tests/renderer/` tiers cannot reach a private
free function", which is not true of a `pub fn listener`.

The consequence is not only a wrong sentence. PHASE-06's *Surfaces* list does
not include `crates/goad/tests/renderer/startup.rs`, so the file where the
eight sibling `StartupError` cases actually live is an undeclared surface, and
`docs/AGENTS.md` §Execute makes touching it scope creep. The phase agent is
sent to a module that is not there and forbidden from the file that would
otherwise be its home.

**Evidence:** `grep -rn 'cfg(test)' crates/goad/src` returns
`controller.rs:530`, `wire.rs:188`, `diagnostics.rs:457` and nothing in
`startup.rs`; `crates/goad/tests/renderer/startup.rs:1-11`;
`crates/goad/src/controller.rs:524-530`; `plan.md` PHASE-06 *Surfaces*.

**Disposition:** fix-now
**Response:** Verified: `grep -rn 'cfg(test)' crates/goad/src` returns
`controller.rs:530`, `wire.rs:188` and `diagnostics.rs:457` only, and
`arguments` plus all eight `StartupError` variants are tested from
`crates/goad/tests/renderer/startup.rs`.

FD-5 is corrected in `plan.md`: it now states that `crates/goad/src/startup.rs`
has no such module and never had one, that the reason the workspace's three
exist (`controller.rs:524-530`: a private free function no integration target
can reach) does not apply to a `pub fn listener`, and that `listener`'s cases go
where `arguments`'s are. PHASE-06/VT-1 is restated to the same effect, and
`crates/goad/tests/renderer/startup.rs` is now a **declared PHASE-06 surface**,
bounded to the new cases plus the one sentence of that file's module doc they
falsify (*"as pure functions with no window"* — `listener` binds a socket),
which the phase owns correcting on the same discipline PHASE-02 applies to
`config.rs`'s doc and PHASE-04 now applies to `renderer/main.rs`'s.

**Outcome:**

### F-3 — PHASE-06/VT-3 asks for an exit-code assertion the target it names forbids in writing, and AC-9's exit-code clause has no other cover

**Severity:** major
**Location:** `plan.md` PHASE-06/VT-3, Coverage table AC-9;
`crates/goad/tests/renderer/startup.rs:9-11`

**Expected:** VT-3 — "**AC-9's stratum 3 half, R-4.** `StartupError::Ingress`
renders through `diagnostics::report_startup`'s path and maps to exit code 2.
**Assert the mapping the way the existing startup cases do.**" `slice-004.md`
AC-9 requires "The exit code is non-zero".

**Observed:** the existing startup cases do not assert the mapping, and say so
as a rule: `crates/goad/tests/renderer/startup.rs:9-11` — *"No test here runs
the binary or asserts an exit code (§9's own rule) — `main`'s one `match` over
`run()`'s `Result` is what chooses the code, and every value that `match` sees
is already covered here."* The mapping lives in `crates/goad/src/main.rs:21-29`,
in the **binary**, which no test target links. A `grep` for `ExitCode` across
`crates/goad/tests` returns only that comment and two prose mentions.

So there is no existing "way" to imitate, the instruction cannot be followed,
and the plan's Coverage table nonetheless lists PHASE-06/VT-3 as the discharge
for AC-9's exit-code clause. The clause is either discharged by the argument
already written at `startup.rs:9-11` (in which case the plan should say
*review, not a test*, the way it correctly does for `SPEC-003/R-5`) or it is
not discharged at all. As written it looks discharged and is not.

**Evidence:** `crates/goad/tests/renderer/startup.rs:9-11`;
`crates/goad/src/main.rs:21-29`; `plan.md` Coverage §Acceptance criteria, AC-9
row; `plan.md` PHASE-06/VT-3.

**Disposition:** fix-now
**Response:** Verified: `renderer/startup.rs:9-11` states the rule in terms, the
mapping lives in the binary at `main.rs:21-29`, and no test target links it. The
clause is not dropped and is not dressed as a test.

Split in two. **VT-3** now asserts what a test can: `StartupError::Ingress`'s
`Display`, and its rendering through `diagnostics::report_startup_line` — the
pure half of the stderr outlet its eight siblings are already asserted through —
in `renderer/startup.rs`'s `display_text` module. **VA-3** is new and holds the
exit code by **review**, recorded the way `SPEC-003/R-5`'s row is: paste
`main.rs:21-29`, record that it is unchanged, and state that its single
`match run()` maps *every* `Err` to `ExitCode::from(2)`, so a ninth variant
reaches exit 2 by the same line the other eight do. VA-3 says in terms not to
build a binary-running harness. The Coverage table's AC-9 row and the
`draft-spec.md` R-4 row both now say *the exit code itself is held by review,
not by a test*, and name VA-3.

**Flagged to the orchestrator, not repaired here:** AC-9 now needs a reading
recorded beside it in `slice-004.md` §*Readings taken in design*, the way AC-1,
AC-3, AC-6 and AC-7 have one — *"the exit code is non-zero" is discharged by
the argument at `main.rs:21-29` rather than by an instrument*. That is the
slice card, which this repair may not edit.

**Outcome:**

### F-4 — PHASE-03/VT-10 requires setting the process umask, which this workspace has no legal way to do

**Severity:** major
**Location:** `plan.md` PHASE-03/VT-10; PL-3; FD-2; `Cargo.toml:74`;
`crates/goad-boundary/tests/checks/allowlist.rs:19-26`

**Expected:** VT-10 — "**AC-10, R-2.** Under a **deliberately permissive umask
set inside the case**, the bound socket's `mode() & 0o777 == 0o600`." It is the
sole discharge the plan gives AC-10 and `SPEC-003/R-2`, whose whole content is
that the host "MUST set the socket's mode to owner-only itself, and MUST NOT
rely on the umask it was started under".

**Observed:** Rust's standard library has no umask API. The three ways to set
one are all closed here:

- `libc::umask` — `libc` is not a dependency of `crates/goad-shell`
  (`Cargo.toml` lists `goad-semantics`, `jiff`, `serde`, `serde_json`, `tokio`,
  `toml`), and the crate has no `[dev-dependencies]` table at all. Adding
  either would add a name to the manifest allowlist's `STRATUM_2`
  (`allowlist.rs:19-26`) — an ADR-001 instrument — which is exactly the reason
  PL-3 and FD-2 refuse `tempfile`.
- `CommandExt::pre_exec` — `unsafe`, and `unsafe_code = "deny"` at
  `Cargo.toml:74`, with `allow_attributes = "deny"` and
  `allow_attributes_without_reason = "deny"` above it.
- running the bind in a subprocess under `sh -c 'umask 000; …'` — there is no
  helper binary to run, and PL-9 forbids adding one.

The phase has no STOP condition covering this: S-1..S-4 name a queue, a second
concurrency dimension, a thin margin and a red existing case. The agent's
realistic exits are to breach an instrument, to breach `unsafe_code`, or to
quietly weaken VT-10 to a bare `== 0o600` assertion — which passes on a default
umask of `0o022` whether or not the host sets the mode at all, and therefore
discharges neither AC-10's *"whatever umask the host was started under"* nor
R-2's *"MUST NOT rely on the umask"*.

There is a separate hazard in the same criterion even if the umask problem is
solved: `umask(2)` is **process-global**, and PL-3's own implementer note
records that "`cargo test` runs cases in parallel in one process". A case that
mutates the umask mutates it for every concurrent case in the `integration`
target.

**Evidence:** `crates/goad-shell/Cargo.toml` (no `libc`, no
`[dev-dependencies]`); `crates/goad-boundary/tests/checks/allowlist.rs:19-26`;
`Cargo.toml:74` (`unsafe_code = "deny"`); `plan.md` PL-3 implementer note,
PHASE-03/VT-10, PHASE-03 STOP list; `draft-spec.md` R-2; `slice-004.md` AC-10.

**Disposition:** fix-now
**Response:** Verified: no `libc` and no `[dev-dependencies]` in
`crates/goad-shell/Cargo.toml`; `unsafe_code = "deny"` at `Cargo.toml:74` with
`allow_attributes`/`allow_attributes_without_reason` `deny` at `:132-133`;
`STRATUM_2` at `allowlist.rs:19-26`. All three routes to a umask are closed, and
the process-global hazard under parallel cases is real.

The umask is dropped entirely rather than worked around. PHASE-03/EX-3 now
states that the host sets the mode **itself**, after `bind`, with
`std::os::unix::fs::set_permissions(path, PermissionsExt::from_mode(SOCKET_MODE))`
— standard library, safe, no new dependency, no instrument breached — and says
**no `umask` call anywhere in this phase**, with the process-global reason.
That discharges `SPEC-003/R-2`'s *"MUST set the socket's mode itself, and MUST
NOT rely on the umask"* directly rather than by demonstrating independence from
a umask the case cannot set.

VT-10 is restated: after `bind`, `mode() & 0o777 == 0o600`, **no umask set
inside the case**, non-vacuous under any umask more permissive than `0o077` (the
usual default is `0o022`), with the run's actual umask recorded in the phase
sheet. The residue is stated rather than hidden: between `bind` and
`set_permissions` the socket is briefly more permissive, which is A-5 and is why
`design.md` §5.5 puts the containing directory on the user. A new **S-5** covers
the one way this can still fail — `set_permissions` not working on a bound
socket — and names the three instruments the alternatives would breach.

**Outcome:**

### F-5 — PHASE-06/VH-1 cannot observe AC-13: the demo backend already has that prompt on screen before the event is emitted

**Severity:** major
**Location:** `plan.md` PHASE-06/VH-1, EX-8; `examples/shell/backend.sh`;
`examples/demo.toml`; `crates/goad/src/main.rs:97`

**Expected:** VH-1 — "A person starts goad with ingress configured
(`just demo`), runs the documented one-liner from a second shell, and
**watches the prompt appear**." AC-13 is the criterion, and `docs/AGENTS.md`
§Tiers makes this the one thing a green gate cannot stand in for. EX-8 asserts
the demo backend needs no change because "it already prompts on every
evaluation that is not a `respond` … so an ingested evaluation produces a
window with no change to it — which is the point."

**Observed:** that property is what breaks the observation. `main::start`
enqueues `Command::Evaluate(Stimulus::Startup)` at `main.rs:97`, so the first
thing `just demo` does is evaluate. `examples/shell/backend.sh`'s `*)` arm
returns a **fixed** view — title *"Fill in your interstitial journal?"*, body
*"The last entry was a while ago."*, options *Yeah* / *Nah* — for every
non-`respond` request. `examples/demo.toml` sets `default_poll = "30m"` and the
script answers `next_check: "45 minutes"`, so nothing else fires.

The person therefore sits in front of an open window carrying exactly the view
the ingested evaluation will produce. Emitting the envelope replaces it with a
byte-identical presentation: same title, same body, same two options. What
changes is the view token, which is not a thing a person can see. **Nothing
appears.**

The sequence that would be observable — answer the startup prompt first
(`respond` → `{"view":null}` → the window hides), *then* emit — is not what
VH-1 says, and PHASE-06/S-3 does not cover it either: it fires when "VH-1's run
shows the prompt appearing but something else visibly wrong", which presumes
the appearance.

**Evidence:** `examples/shell/backend.sh` (the `case` at its foot, one fixed
view for every non-`respond` request); `examples/demo.toml`
(`default_poll = "30m"`); `crates/goad/src/main.rs:94-98`;
`crates/goad/src/glass.rs:67-121` (`present` writes the same properties either
way, and `Surface::Prompt` → `window.show()`); `plan.md` PHASE-06/EX-8, VH-1,
S-3; `slice-004.md` AC-13.

**Disposition:** fix-now
**Response:** Verified: `examples/shell/backend.sh`'s `*)` arm returns one fixed
view for every non-`respond` request; `main.rs:97` enqueues
`Command::Evaluate(Stimulus::Startup)` on every start; `examples/demo.toml` sets
`default_poll = "30m"` and the script answers `next_check: "45 minutes"`. A
person would have been looking at the ingested evaluation's view before emitting
anything, and the envelope would have replaced it with a byte-identical
presentation.

**PHASE-06/EX-8 is inverted.** `examples/shell/backend.sh` **is** modified, and
is now a declared PHASE-06 surface. It answers an ingested evaluation with a
view whose title or body **names the event's `source` and `kind`**: a `respond`
is unchanged; a request whose event carries `"source":"host"` — startup,
scheduled, a click — keeps today's fixed prompt unchanged, so the demo still
opens a window on its own; anything else came in through the socket and gets a
view naming it. The fix belongs there because it is user-authored example code
and interpreting an event is exactly the backend's job — which is also what
makes the demo demonstrate the slice: **the host forwards the envelope verbatim
and the backend decides what it means.** The extraction is shell parameter
expansion in the style of the file's existing `case`, with its own comment
extended rather than a parser added; an implementer note gives the two lines and
warns that the `"source":"host"` arm must precede the catch-all.

VH-1 is restated as five numbered things a person does and sees, of which step 3
is *the window changes to a prompt naming that `source` and that `kind`* — the
observation a byte-identical redraw could not have been. A new **S-4** fires if
step 3 shows no change at all, and says in terms that VH-1 is not discharged on
the reply alone.

**Outcome:**

### F-6 — PHASE-05/VT-1 repeats the setup defect `review-design.md` F-1 and F-12 found fatal, in the one AC-6 case the plan does not pin

**Severity:** major
**Location:** `plan.md` PHASE-05/VT-1, EX-2, VA-3, S-1, S-2;
`crates/goad/src/controller.rs:507-512`

**Expected:** PHASE-05/EX-2 pins the ingested exchange's own `next_check` for
case (ii) **and says why in one sentence**, because "every completed exchange
re-arms the pending deadline from the instruction *that* exchange's backend
returned (`controller.rs:507-512`, `SPEC-001/R-26`)". `review-design.md`'s
Synthesis records this as the blocker the design review found: a case whose
setup does not fix that deadline turns on something both hypotheses agree
about.

**Observed:** the same mechanism governs case (i), and the plan says nothing
about it. VT-1 is stated in full as: *"An ingested exchange falling between a
short `next_check` and its firing does **not** push that firing out by the
spacing."* The exchange it inserts is a **completed exchange**, so
`controller.rs:507-512` re-arms `sleep` from
`deadline_after(now, wait_for(absorbed.next_check, requested_at), floor_until)`
— using the **ingested** exchange's own `next_check`. Two outcomes, neither of
which VT-1's stated setup excludes:

- the ingested exchange's `next_check` is **longer** than the scheduled firing's
  remaining wait — the pending firing is pushed out, and VT-1 fails **against a
  correct implementation**, for a reason (SPEC-001/R-26) that has nothing to do
  with the anchor;
- it is short enough that `floor_until` never binds either hypothesis, and VT-1
  passes under both — which is exactly F-1/F-12's finding, re-instanced.

The falsification only reaches the anchor when the ingested exchange's own
`next_check` is pinned so that `floor_until` is what the two hypotheses
disagree about. That clause is written for VT-2 and absent for VT-1.

PHASE-05/VA-3's break-and-revert ("make the ingested firing write `floor_until`
as well; confirm VT-1 goes red") would eventually surface it, but S-2 names
only VT-2, and S-1 — "a case cannot be written without changing production
code … either PHASE-04 is not done, or the design is wrong about what is
observable" — points the agent at the design when what is incomplete is the
plan's fixture specification.

**Evidence:** `crates/goad/src/controller.rs:505-513` (the absorb and re-arm,
and `deadline_after(…, floor_until)` as the only floor in that expression);
`plan.md` PHASE-05/VT-1 versus VT-2 and EX-2; `review-design.md` Synthesis
(*"AC-6 named two tests and neither reached the case ADR-004 has been waiting
for"*) and F-1/F-12; `design.md` §9 AC-6 row (i), equally silent.

**Disposition:** fix-now
**Response:** Verified at `controller.rs:505-514`: the absorb re-arms `sleep`
from `deadline_after(now, wait_for(absorbed.next_check, requested_at),
floor_until)`, using the **ingested** exchange's own `next_check`. VT-1's stated
setup excludes neither of the two failure modes the finding names.

Treated as the class it is, in two places and at two levels.

**The instance.** PHASE-05/VT-1 now carries the pin and the one-sentence reason,
exactly as EX-2 requires of VT-2: the exchange it inserts is a *completed*
exchange, so pinned long the pending firing is pushed out and VT-1 fails against
a correct implementation for an R-26 reason unrelated to the anchor, and pinned
short `floor_until` binds neither hypothesis and VT-1 passes under both.

**The class.** Two new phase-level exit criteria state the rule over whole
files rather than case by case. **PHASE-05/EX-5**: every case pins the
`next_check` of every exchange it lets complete, and says in one line what it
pinned and why; VT-1..VT-6 are all named as members, and VT-3, VT-4, VT-5 and
VT-6 each gained an *EX-5 applies* clause saying what theirs pins and what goes
wrong unpinned. **PHASE-04/EX-11**: the same rule over
`renderer/ingress.rs`, naming VT-1, VT-3, VT-4 and VT-5 as members — VT-1 and
VT-5 count invocations, VT-5 counts presentations, VT-3 and VT-4 turn on when a
firing happens.

PHASE-05/S-1 is also amended: it now says explicitly that a case which will not
discriminate because its `next_check` sequence is wrong is EX-5 firing, not the
design being wrong about what is observable — the misdirection the finding
identified.

**Outcome:**

### F-7 — PHASE-04/EX-8's closed-channel path is an exit criterion no verification criterion drives

**Severity:** major
**Location:** `plan.md` PHASE-04/EX-8; Coverage AC-12 row; `design.md` §5.2,
§5.5 (last edge-case row); `draft-spec.md` §5, R-15, §6.3

**Expected:** the plan's own sequencing rationale rejects leaving "a branch
nothing drives in between" and cites
`docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md` for it. AC-12
requires that "no ingress failure after startup takes the host down or leaves
it unable to invoke the backend again". `design.md` §5.5 calls this path "what
'ingress cannot die silently' actually holds", and `draft-spec.md` §5 states
it normatively ("When ingress stops but the host does not").

**Observed:** EX-8 specifies a behaviour with five separable claims — on
`arrival()` yielding `None` the arm folds **one** `Refused::Ingress`
(`unavailable`, naming that ingress has stopped), drops the receiver so the arm
**parks** rather than spinning on a closed channel, builds **no** `Fired`, so
`refusal_re_arms` is not reached, the standing deadline is not reset, and
neither anchor is written. It is asserted in **both** arms.

No VT in any phase drives it. PHASE-04/VT-1..VT-6 cover AC-1, AC-2, `engaged`,
`too_soon`, the flat-out writer and the unchanged suite. PHASE-03/VT-6 is the
listener's side of a *dropped `Answer`*, which is a different mechanism in the
other direction. The Coverage table maps AC-12 to PHASE-03/VT-11 and
PHASE-05/VT-6, both of which are malformed-envelope cases — a malformed
envelope is not an ingress failure of this kind.

The failure this leaves unguarded is the expensive one. If the receiver is not
dropped on the way out, a closed `mpsc::Receiver` is immediately ready on every
poll, the `biased` outer `select!` puts ingress last, and the arm folds a
refusal and `continue`s to `controller.rs:410` — one full `glass.present` per
iteration, forever, on the main thread. That is `design.md` §8 R6's cost with
no bound at all, and it is the one thing PHASE-04/VT-5's flat-out-writer test
would not catch, because it needs no writer.

The plan does not record this as `review, not a test` either, the way it
correctly does for `SPEC-003/R-5`. It is simply absent.

**Evidence:** `plan.md` PHASE-04/EX-8 and its VT list; `plan.md` Coverage
§Acceptance criteria AC-12 row and §`draft-spec.md`'s requirements R-16 row;
`plan.md` §Sequencing (*"leaving a branch nothing drives in between"*);
`crates/goad/src/controller.rs:409-411` (the loop's first statement is
`glass.present`); `design.md` §5.2 (*What the loop does with `None`*) and §5.5
(last edge-case row); `draft-spec.md` §5.

**Disposition:** fix-now
**Response:** Verified: EX-8's five claims are driven by no VT in any phase, and
the Coverage rows for AC-12 and R-16 named only malformed-envelope cases, which
are a different mechanism.

**PHASE-04/VT-7** is new, and it is PHASE-04's because PHASE-04 owns EX-8 and
already builds PL-8's counting `Glass` for VT-5. It asserts three things: one
`Refused::Ingress` (`unavailable`) is folded — **exactly one**, read off
`Served.controller`; the arm **parks**, asserted as *the presentation count does
not advance* over a window after the fold; and the host still evaluates
afterwards, the liveness half without which the second is vacuous. The criterion
states plainly why the second assertion is the one the case exists for: the
unguarded failure is not a missing refusal but a spin — a closed
`mpsc::Receiver` is ready on every poll, and `controller.rs:409-410` charges one
full `glass.present` per iteration on the main thread, forever — and a case that
only observed the fold would pass against exactly that.

The criterion also names the mechanism, because otherwise the phase agent walks
into a wall: `design.md` §5.2 says the real cause of `None` is a panic in the
accept task, which no test can provoke. `bind` spawns with `tokio::spawn` onto
whatever runtime is entered, so the case builds a **second** multi-thread
runtime, calls `bind` under its `enter()` guard, keeps the `Ingress`, and
`shutdown_background()`s that runtime — dropping its tasks drops the sender.
`shutdown_background` rather than `drop`, because dropping a `Runtime` inside an
async context panics. No production API is added. A new **S-5** stops the phase
if that does not produce `None`, and forbids both adding a constructor to
`Ingress` (PHASE-03's surface, and a design question) and substituting a
criterion that only observes the fold.

Coverage: AC-12, R-15 (its last clause — the ingress-stopped `unavailable` is
the one refusal that answers no envelope) and R-16 now all name PHASE-04/VT-7.

**Outcome:**

### F-8 — PHASE-04's bound on `renderer/main.rs` leaves that file's module doc false, where PHASE-02 names the identical correction as its own

**Severity:** minor
**Location:** `plan.md` PHASE-04 *Surfaces*;
`crates/goad/tests/renderer/main.rs:1-11`; `plan.md` PHASE-02 implementer notes

**Expected:** PHASE-02's implementer notes handle exactly this case and handle
it well: "`config.rs`'s own doc comment says *'Brief §5's three values and
nothing else (the OQ-4 decision)'*. That sentence is now false and **this phase
owns correcting it** — a fourth value is a deliberate addition, not a drift."

**Observed:** PHASE-04 bounds `crates/goad/tests/renderer/main.rs` to "**one
`mod ingress;` declaration**", and that file's module doc opens:

> "The cheap tier: headless, no display server, **no socket opened**
> (design.md §5.1, `plan.md` EX-8). … **Eight modules today**: …"

PHASE-04/VT-1..VT-5 drive `serve` "with a **real bound `Ingress`**" — a real
Unix socket in the renderer target — and add a ninth module. Both sentences
become false, and the surface bound forbids repairing either. The declaration
itself is also two lines, not one: every sibling in that file carries
`#[cfg(test)]` above `mod X;`, for the `clippy::tests_outside_test_module`
reason the doc states.

`review-design.md`'s Synthesis names this exact class as the one to carry into
implementation: *"this design's claims are stated in more than one place, and
prose, tables and diagrams drift at different rates."*

**Evidence:** `crates/goad/tests/renderer/main.rs:1-11` and its
`#[cfg(test)] mod …` declarations; `plan.md` PHASE-04 *Surfaces*; `plan.md`
PHASE-02 *Notes for the implementer*, third bullet; `review-design.md`
Synthesis.

**Disposition:** fix-now
**Response:** Verified: `renderer/main.rs:1` says *"no socket opened"* and `:7`
says *"Eight modules today"*, both of which PHASE-04 falsifies, and every
sibling declaration there is two lines (`#[cfg(test)]` above `mod X;`).

PHASE-04's surface bound on that file now names **two** things rather than one:
the two-line `#[cfg(test)] mod ingress;` declaration, and the two module-doc
sentences the phase makes false — with the correction explicitly assigned to
this phase, on PHASE-02's stated discipline (*a doc that a deliberate addition
falsifies is corrected by the phase that adds it*). VA-3's diff description is
updated to match, so the auditor's instrument and the surface bound say the same
thing. The same discipline is now applied a third time, by PHASE-06 to
`renderer/startup.rs`'s module doc (F-2), so the class has one rule rather than
three ad-hoc rulings.

**Outcome:**

### F-9 — the blanket "a margin under 10x is a STOP" fires by construction on the tests that must wait out the spacing

**Severity:** minor
**Location:** `plan.md` PHASE-04/VA-2 and S-3; PHASE-05/VA-2;
`crates/goad/tests/renderer/scheduling.rs:48-53`

**Expected:** PHASE-04/VA-2 — "the per-test elapsed time of VT-1..VT-5 recorded
against the bound each one actually governs … A margin under 10x is S-3."
PHASE-04/S-3 — "any margin under VA-2 comes in under 10x … Stop."

**Observed:** several of this slice's cases wait out the three-second spacing
by construction, and the only bound that governs their liveness assertion is
`LIVENESS_BOUND`, 5 s. PHASE-04/VT-4 is the clearest: it refuses a second
envelope `too_soon` and is then "paired with a liveness control: an envelope
**outside** the spacing is accepted" — which cannot be observed before
`MINIMUM_SPACING` (`controller.rs:318`, three seconds) has elapsed. Elapsed
~3 s against a 5 s bound is a ratio of ~1.7x, so S-3 fires on a test that is
correct and cannot be made faster: `design.md` D-5 rejected a second,
configurable constant "precisely so that no test could buy time by moving a
bound", and PHASE-05's own implementer note says so.

PHASE-05 gets this right in prose — VA-2 says "VT-2 spends a floor interval by
construction — three seconds of gate time, deliberately … Record it as such" —
and PHASE-05's STOP list correctly omits a margin rule. PHASE-04 carries the
blanket rule and a VT-4 the rule condemns. The two phases state incompatible
things about the same discipline.

The distinction the plan needs is the one its own PHASE-05 note draws: a
**liveness** bound admits a ratio; an **anti-fire window**, and a wait that is
the bound under test, do not.

**Evidence:** `plan.md` PHASE-04/VT-4, VA-2, S-3; `plan.md` PHASE-05/VA-2 and
implementer notes 3 and 4; `crates/goad/src/controller.rs:318`;
`crates/goad/tests/renderer/scheduling.rs:48-53` (`FLOOR_MILLIS = 3_000` and
the comment on why the mirror exists);
`docs/memory/timed-test-margins-are-measured-at-the-bound.md`.

**Disposition:** fix-now
**Response:** Verified: `MINIMUM_SPACING` is 3 s (`controller.rs:318`),
`LIVENESS_BOUND` is 5 s, VT-4's liveness control cannot be observed before the
spacing has elapsed, and PHASE-05 already draws the right distinction in prose
while PHASE-04 carried a blanket rule that condemns VT-4 by construction.

PHASE-04/VA-2 now exempts **a case that waits out `MINIMUM_SPACING` by
construction**, names VT-4 as the clearest one, gives the ~1.7x number and its
cause, and cites `design.md` D-5 — a second configurable constant was rejected
precisely so that no test could buy time by moving a bound, so a rule condemning
such a case would condemn the design. It states the distinction PHASE-05's note
already draws: a **liveness** bound admits a ratio; an **anti-fire window**, and
a wait that *is* the bound under test, do not — and requires each case to be
recorded as one of the three. S-3 is narrowed to fire only on a case VA-2 does
not exempt.

PHASE-08/VA-2 (formerly PHASE-03/VA-2) keeps the 10x rule unweakened and states
why it is safe there: `ENVELOPE_DEADLINE` is 500 ms and is the subject of VT-14,
which is the ratio the rule is written for.

**Outcome:**

### F-10 — `event_floor_until`'s initial value is specified nowhere, and it decides whether the first event after startup is accepted

**Severity:** minor
**Location:** `plan.md` PHASE-04/EX-6; `design.md` §5.3;
`crates/goad/src/controller.rs:404-407`

**Expected:** EX-6 reads as a complete statement of the new anchor:
"`event_floor_until` is a second anchor on `serve`'s stack with **exactly one
write site** … and one read site". The scheduled anchor beside it has its
initial value written down *and argued*: `floor_until = started`
(`controller.rs:407`), under a three-line comment saying why — "the first
scheduled firing of the process is unfloored, which is what lets a
`default_poll` shorter than the spacing be honoured once (SPEC-002 §6)".

**Observed:** neither `plan.md` nor `design.md` §5.3 gives `event_floor_until`
an initial value. The two candidates are observably different: `started`, and
the first envelope after startup is accepted; `started + MINIMUM_SPACING`, and
every envelope inside the first three seconds of the process is refused
`too_soon`.

Three criteria turn on the choice. PHASE-04/VT-1 (AC-1) writes an envelope
shortly after `serve` starts and asserts exactly one `evaluate`; PHASE-04/VT-5
(AC-5, F-15's settlement) counts one acceptance and the rest `too_soon` over a
window "far shorter than the spacing"; PHASE-06/VH-1 has a person emit an
envelope after `just demo` starts.

The design's own reasoning does imply an answer — P-3 spaces a class "from the
previous firing of *its own class*", and there is no previous ingested firing —
which is the same standard FD-1..FD-5 were recorded against. It is the sixth
finding of that class and it is the one the plan did not record.

**Evidence:** `crates/goad/src/controller.rs:404-407`; `design.md` §5.3 state
table; `plan.md` PHASE-04/EX-6, VT-1, VT-5; `design.md` P-3.

**Disposition:** fix-now
**Response:** Verified: neither `plan.md` nor `design.md` §5.3 gives
`event_floor_until` an initial value, and the two candidates are observably
different for PHASE-04/VT-1, PHASE-04/VT-5 and PHASE-06/VH-1.

PHASE-04/EX-6 now states it where the anchor is built: **initialised to
`started`** — already elapsed by the time anything runs, so **the first envelope
after startup is accepted** — with the same three-line comment `floor_until`
carries at `controller.rs:404-407`. It is recorded as **forced, not chosen**:
the only alternative, `started + MINIMUM_SPACING`, is precisely the value the
anchor would hold if the **startup** evaluation had written it, and
`review-design.md` F-1's verified claim is that a scheduled firing never writes
the event floor and an ingested firing never writes the scheduled floor (I-4,
P-3). The design's own P-3 — a class is spaced from the previous firing of *its
own class*, and there is no previous ingested firing — reaches the same value.

Checked as instructed: VT-1 (an envelope shortly after `serve` starts produces
one `evaluate`), VT-5 (one acceptance and the rest `too_soon` over a window far
shorter than the spacing) and VH-1 (a person emits after `just demo` starts) all
read correctly under an already-elapsed floor, and all three would be wrong or
unbuildable under the other.

**Design gap reported, not filled:** `design.md` §5.3's state table gives
`event_floor_until`'s writer, reader and lifetime but **not its initial value**,
where the row beside it for `floor_until` has one argued in the code it cites.
That is the design's to close; the plan now states the value the design implies
and says which reasoning forces it.

**Outcome:**

### F-11 — the lint briefing names two traps and misses the two that sit on PHASE-03's core path

**Severity:** minor
**Location:** `plan.md` Overview item 5; PHASE-03/EX-5, EX-7; `Cargo.toml:142`,
`:183`

**Expected:** Overview item 5 — "**Two ways this slice can quietly break the
host, and both are lints.**" — names `clippy::future_not_send` and
`missing_debug_implementations`. PL-5 adds `arithmetic_side_effects` at module
level, and PHASE-03's implementer notes correctly warn about
`integer_division`, `as_conversions` and the four `cast_*` lints for the
rounding. The briefing is otherwise careful.

**Observed:** two `deny` lints that this phase will certainly meet are named
nowhere:

- **`clippy::indexing_slicing = "deny"`** (`Cargo.toml:142`). EX-7 is byte
  framing — "terminated by the first newline **or** by end of input" with
  "bytes after the first newline never read" — and the obvious spellings
  (`&buf[..n]`, `buf[i]`) are denied. The legal routes are
  `AsyncBufReadExt::read_until`, `slice::split_at_checked`, `get(..n)` or
  `strip_suffix`, and picking one is a decision the phase should not discover
  at lint time.
- **`clippy::pub_use = "deny"`** (`Cargo.toml:183`). EX-5 gives `Refusal` a
  variant `InvalidEnvelope(EnvelopeFault)` where `EnvelopeFault` is
  PHASE-02's, in `ingress/envelope.rs`. The natural
  `pub use envelope::EnvelopeFault;` in `ingress/mod.rs` is denied, so the
  module layout has to be chosen with that in mind.

`allow_attributes` and `allow_attributes_without_reason` are both `deny`
(`Cargo.toml:132-133`), so neither has a cheap escape hatch.

**Evidence:** `Cargo.toml:142` (`indexing_slicing = "deny"`), `:183`
(`pub_use = "deny"`), `:132-133`; `plan.md` Overview item 5, PHASE-03/EX-5 and
EX-7 and *Notes for the implementer*.

**Disposition:** fix-now
**Response:** Verified: `indexing_slicing = "deny"` at `Cargo.toml:142`,
`pub_use = "deny"` at `:183`, `allow_attributes` and
`allow_attributes_without_reason` `deny` at `:132-133`.

Both are named in the listener phases' implementer notes, where they land.
**`indexing_slicing`** is PHASE-03's, against EX-7's byte framing: the note says
`&buf[..n]` and `buf[i]` are denied, that there is no cheap hatch, lists the four
legal routes (`AsyncBufReadExt::read_until`, `slice::split_at_checked`,
`get(..n)`, `strip_suffix`), and says to choose deliberately rather than at lint
time. **`pub_use`** is named in PHASE-03 too, not only where the variant lands,
because the module layout that decides it is PHASE-03's: EX-10 puts PHASE-02's
`EnvelopeFault` inside a `Refusal` variant, so `pub use envelope::EnvelopeFault;`
in `ingress/mod.rs` is denied and the layout has to be chosen with that in mind.
PHASE-08's notes repeat both, pointing back at PHASE-03's choices rather than
re-deciding them. Overview item 5 now says in one sentence that two further
`deny` lints sit on the listener's core path and are named in those two phases'
notes, so a reader of the Overview is not left thinking the list of two is the
whole of it.

**Outcome:**

### F-12 — PHASE-03's parity with its stated calibration is asserted, and the asymmetry runs the other way

**Severity:** minor
**Location:** `plan.md` §Sequencing & rationale, *Size*; `docs/slices/003/plan.md:129-141`

**Expected:** `docs/AGENTS.md` §Plan requires phases sized so each is
"reasonable for a single agent to complete within a session, including
bookkeeping". The plan's *Size* paragraph says PHASE-03 and PHASE-04 are the
heavy ones, names slice 003's PHASE-02 as the calibration, and concludes
"neither exceeds it".

**Observed:** the conclusion is stated, not derived, and the one dimension the
calibration itself argued on runs against PHASE-03. Slice 003's own *Size*
paragraph justified not splitting PHASE-02 on the ground that its bulk was
**uniform**: "the 22 edits are uniform — `.shift` appended, no import added,
`Shift` already in scope at every assertion — and the fixture lift is a
seven-item move plus one `use` line. **Neither is re-reading, which is what a
session's budget is actually spent on.**"

PHASE-03's bulk is the opposite. Its 14 verification cases (VT-1..VT-14) each
name a distinct filesystem or protocol condition — reclaim, a live holder, a
regular file, an uncreatable path, three framing arms, a dropped `Answer`, five
shape reasons, the exact token set, `retry_after_ms`'s rounding, the umask, a
malformed-plus-liveness pair, the positive control, `too_large`, `timed_out` —
and each needs its own socket, its own temp directory and its own fake-judge
script. Slice 003's PHASE-02 had eight, sharing one new `harness.rs`. On top of
that PHASE-03 lands a production module from nothing: `bind`'s five-step
sequence, five public types, an eight-token closed vocabulary with four
payloads, an accept task with two budgets and the framing, and a reply
serializer with checked-arithmetic rounding.

The two phases have the same *count* of criteria (32 each), which is presumably
what "neither exceeds it" rests on. Counting criteria is the wrong instrument
when slice 003's own argument was about re-reading rather than about edits.

PHASE-04 is the closer analogue of 003/PHASE-02 — a `serve` change plus a
uniform 23-site migration plus six renderer cases — and is not what this
finding is about.

**Evidence:** `plan.md` §Sequencing & rationale, *Size*;
`docs/slices/003/plan.md:129-141`; `plan.md` PHASE-03/EX-1..EX-9 and
VT-1..VT-14; `docs/slices/003/plan.md` PHASE-02 criteria (8 VT of 32 total,
counted); `docs/AGENTS.md` §Plan.

**Disposition:** fix-now
**Response:** The honest argument does not hold, and **PHASE-03 is split**
(PL-10). Slice 003's own *Size* paragraph justified staying whole on the ground
that its bulk was **uniform** and *"neither is re-reading, which is what a
session's budget is actually spent on"* — and PHASE-03's bulk was the opposite:
fourteen bespoke cases naming fourteen distinct filesystem and protocol
conditions, each with its own socket, temp directory and fake-judge script, on
top of a production module landed from nothing. Counting criteria made the two
look equal at 32 each, which was the wrong instrument.

**The split is at the reply.** PHASE-03 owns everything the *filesystem* can get
wrong plus the one reply a well-formed envelope gets — `bind`, the socket's
lifecycle, `Ingress`/`Arrival`/`Answer`, the framing, the accepted path — with
eight cases (VT-1..VT-6, VT-10, VT-11, VT-12), which is exactly slice 003's
calibration. **PHASE-08** owns everything the *writer* can get wrong — the two
read budgets and their enforcement, the rest of `Refusal`'s payloads,
`retry_after_ms`'s rounding, the closed reason set — with five (VT-7, VT-8,
VT-9, VT-13, VT-14). It runs immediately after PHASE-03 and before PHASE-04,
because PHASE-04 constructs `engaged` and `too_soon`. Order: **01, 02, 03, 08,
04, 05, 06, 07**.

Each half is red/green complete, which is why the seam is there rather than
somewhere tidier: `ENVELOPE_LIMIT` and `ENVELOPE_DEADLINE` are declared **and
enforced** in PHASE-08 with the cases that drive them rather than declared inert
in PHASE-03, and PHASE-03's `Refusal` (EX-10) carries only the three variants
its own cases reach, with an exhaustive match so that PHASE-08 completing the
set to eight is the compiler's business. Nothing lands in one phase that only
the next phase's tests exercise — the failure PHASE-03's own sequencing
rationale objects to.

Criterion ids are preserved across the move, so the moved criteria are
`PHASE-08/VT-7`, `VT-8`, `VT-9`, `VT-13`, `VT-14`, `VA-2` and `EX-5` with their
original numbers, and PHASE-08's sequence is non-monotonic — which this file's
header comment says in terms is expected after a split. The Overview,
*Sequencing & rationale* (including a rewritten *Size*), the parallelism rule,
both Coverage tables, PHASE-04/EN-1 and EN-2, and PHASE-07/EX-4's margin-table
sources are all updated. PHASE-04's own sizing is now argued rather than
asserted, on the dimension 003 argued on: a mechanism in one function body, a
uniform 23-site migration where every edit is the identical added argument, and
six cases sharing one file's fixtures.

**Outcome:**

### F-13 — FD-4, the citation-checking finding, is itself off on three of its rows

**Severity:** nit
**Location:** `plan.md` FD-4; PHASE-02 implementer notes

**Expected:** FD-4 exists to be the precise one: "Every code citation in
`design.md` §§2, 5 and 9 was checked against the tree at `b6ca5f7`", and it
corrects `controller.rs:429` to `:427`, which is right.

**Observed:** three rows in or beside that table do not match the tree:

| written | found |
|---|---|
| `glass.rs` — "`present` spans `:68-119`" | `fn present` is at `:67`; its body closes at `:121` (`impl` closes at `:122`) |
| `canonical.rs:105` — "`Timestamp::new(jiff::Timestamp)` is `pub`" (PHASE-02 notes) | `impl Timestamp` is at `:105`; `pub fn new` is at `:106` |
| `allowlist.rs:19-27` — stratum 2's allowlist | `const STRATUM_2` spans `:19-26`; `:27` is blank |

None sends an agent anywhere useful-but-wrong, which is why this is a nit
rather than a repeat of the `:429`/`:427` class. It is worth recording only
because FD-4 is the artefact whose whole claim is that these were checked.

**Evidence:** `crates/goad/src/glass.rs:67`, `:121-122`;
`crates/goad-semantics/src/protocol/canonical.rs:105-106`;
`crates/goad-boundary/tests/checks/allowlist.rs:19-26`.

**Disposition:** fix-now
**Response:** All three verified — `fn present` at `glass.rs:67` with its body
closing at `:121` and the `impl` at `:122`; `impl Timestamp` at
`canonical.rs:105` with `pub fn new` at `:106`; `const STRATUM_2` spanning
`allowlist.rs:19-26` with `:27` blank. FD-4 is corrected on all three, retitled,
and its trailing sentence now names the four citations this plan carries rather
than one.

Because FD-4 is the citation-checking artefact, the rest of it was re-checked
and **every** `path:line` in `plan.md` was swept, not only FD-4's rows. Four
further errors were found and fixed, and two more rows recorded as off-by-one in
FD-4's table:

- `Cargo.toml:33-34` for tokio's workspace feature set → **`:36-37`**; `:33-34`
  are `serde` and `serde_json` (FD-1).
- `diagnostics.rs:53-62` for `Refused` → **`:53-61`**; `:62` is blank
  (PHASE-04/EX-2).
- `glass.rs:68-119` for the presentation cost → **`:67-121`**
  (PHASE-04 implementer notes).
- `startup.rs:28-50` for `StartupError`'s eight variants → **`:21-42`**; `:28-50`
  starts mid-enum and ends mid-`Display` (PHASE-06/EX-1).
- `renderer/scheduling.rs:53` for `FLOOR_MILLIS` → **`:52`**.
- `renderer/scheduling.rs:31-37` for the three instruction constants →
  **`:30-37`**; `NOTHING_INSTRUCTED` is at `:30`.
- `allowlist.rs:19-27` in PL-3 → **`:19-26`**, and `vocabulary.rs:18-27` in
  PHASE-07/VA-4 → **`:18-26`** (`DOMAIN` spans `:18-26`).

Also recorded in FD-4 as design-side off-by-one, since the design cites it:
`controller.rs:505-513` for the inner `select!` is `:505-514`. The remaining
citations — `controller.rs:407`, `:420`, `:427`, `:507-512`, `:524-530`, `:530`,
`main.rs:21-29`, `:63`, `:103`, `wire.rs:41-70`, `:62`, `:79`, `:188`,
`canonical.rs:490-497`, `error.rs:18`, `config.rs:226`, `wiring.rs:899`,
`renderer/scheduling.rs:141`, `renderer/startup.rs:9-11`, the four `Config`
struct-literal sites and the 23 `serve` call sites — were checked and hold.

**Outcome:**

### F-14 — PHASE-02's stratum 1 surface bound says "one word" and its criteria require two changes

**Severity:** nit
**Location:** `plan.md` PHASE-02 *Surfaces*, EX-8, VA-3

**Expected:** the surface bound and the exit criteria should say the same
thing, because the surface bound is what a phase agent reads first and what an
auditor diffs against.

**Observed:** *Surfaces* says `crates/goad-semantics/src/error.rs` is
"**bounded to one word**, the visibility of `json_type_name` at `:18`". EX-8
requires the same edit **plus** "its doc comment amended to say it is now the
workspace's one such table rather than the crate's". VA-3 reconciles them —
"one word, plus the doc comment sentence EX-8 names" — but only after the
agent has read three separate statements of the bound, one of which is wrong.
The doc-comment sentence in question is real and needs the change:
`error.rs:16` says "The one such table in the **crate**".

**Evidence:** `plan.md` PHASE-02 *Surfaces*, EX-8, VA-3;
`crates/goad-semantics/src/error.rs:14-18`.

**Disposition:** fix-now
**Response:** Verified: `error.rs:16` reads *"The one such table in the crate"*
and does need the change, so the surface bound was the wrong one of the three
statements.

PHASE-02's *Surfaces* now reads **bounded to two edits and no more** — the
visibility of `json_type_name` at `:18`, and the one sentence of its doc comment
at `:16` that EX-8 names — quoting the sentence. VA-3 is reworded to the same
count, so the surface bound, the exit criterion and the auditor's instrument now
all say two edits rather than one, one-plus-one, and one word.

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

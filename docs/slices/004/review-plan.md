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

**Round 2** — 2026-09-08 — verification of the fourteen repairs against the
tree, and the repairs attacked as new work. Aimed where the design review's
three-round lesson says to aim: at the **more binding site a repair did not
reach**. Specifically at the split's seam (does PHASE-03's exit establish
PHASE-08's entry; is anything lost or double-owned; is the numbering
readable), at `set_permissions` as a replacement mechanism for the umask, at
what a person would now actually see under PHASE-06/VH-1, at whether
PHASE-04/VT-7 can catch a spin, at an independent re-sweep of every
`path:line` in `plan.md`, and at both Coverage tables whole after the movement.

Mid-round, `design.md` §5.3 was amended to state both anchors' initial values —
the gap F-10's response reported and did not fill. F-10's outcome above was
written before that landed and describes the *response*, which is still what it
says; the gap itself is now closed, and what the amendment left is F-24.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | verified |
| F-2 | major | fix-now | verified |
| F-3 | major | fix-now | verified |
| F-4 | major | fix-now | verified |
| F-5 | major | fix-now | verified |
| F-6 | major | fix-now | verified |
| F-7 | major | fix-now | verified |
| F-8 | minor | fix-now | verified |
| F-9 | minor | fix-now | verified |
| F-10 | minor | fix-now | verified |
| F-11 | minor | fix-now | verified |
| F-12 | minor | fix-now | verified |
| F-13 | nit | fix-now | verified |
| F-14 | nit | fix-now | verified |
| F-15 | major | fix-now | |
| F-16 | minor | fix-now | |
| F-17 | minor | fix-now | |
| F-18 | minor | fix-now | |
| F-19 | minor | fix-now | |
| F-20 | minor | fix-now | |
| F-21 | nit | fix-now | |
| F-22 | nit | fix-now | |
| F-23 | nit | fix-now | |
| F-24 | minor | fix-now | |

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

**Outcome:** verified — `plan.md` PHASE-06/EX-3 now carries the re-sequencing as
a named, bounded edit inside `main.rs`: `let host = Host::new(config, backend,
now);` moves below `let _entered = runtime.enter();` and below the `listener`
call, so `config.ingress.as_ref()` is formable where the bind happens. Checked
against the tree: `Host::new` (`host.rs:127-134`) stores three arguments and
calls `schedule::resolve`, so it neither needs the reactor nor is affected by
it; `ProcessBackend::new` at `main.rs:53` already borrows `config` and stays
put; nothing between `:54` and `:73` reads `host`. The three forbidden routes
are named, and the numbered-comment-block correction is assigned. **One thing
the repair introduced is raised separately as F-15**: `design.md` §5.4's own
startup diagram puts `Host::new` *before* the runtime, and EX-3 claims to be
following that order while moving one of its steps.

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

**Outcome:** verified — FD-5 now states that `crates/goad/src/startup.rs` has
no such module and never had one, gives the reason the workspace's three exist
(`controller.rs:524-530`, a private free function no integration target can
reach) and why it does not apply to a `pub fn listener`, and points `listener`'s
cases where `arguments`'s are. `crates/goad/tests/renderer/startup.rs` is a
declared PHASE-06 surface, bounded to the new cases plus the one module-doc
sentence they falsify. VT-1 says in terms that the phase does not add a
`#[cfg(test)] mod tests` and why. The precedent is now stated the way the code
actually reads.

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

**Outcome:** verified — the clause is neither dropped nor dressed as a test.
VT-3 now asserts `Display` and `report_startup_line`, which is exactly what the
eight siblings assert and what the file permits; VA-3 holds the exit code by
**review**, quotes the mechanism (`main.rs:21-29`'s single `match run()`, every
`Err` to `ExitCode::from(2)`), and forbids building a binary-running harness.
Both Coverage tables now say *the exit code itself is held by review, not by a
test* and name VA-3, in the same shape `SPEC-003/R-5`'s row uses. The
`slice-004.md` reading was correctly flagged to the orchestrator rather than
written by the repair. **The two documents that state the old discharge —
`design.md` §9's AC-9 row and `draft-spec.md` §7's R-4 row — were not carried,
and that is F-15.**

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

**Outcome:** verified — the mechanism is replaced rather than worked around.
PHASE-03/EX-3 puts `std::os::unix::fs::set_permissions` immediately after
`bind`, which is safe, in `std`, adds no dependency, and discharges
`SPEC-003/R-2`'s *"MUST set the socket's mode itself"* directly. **No `umask`
call anywhere in this phase**, with the process-global reason stated. VT-10 is
restated with no umask in the case, S-5 covers the one residual failure and
names the three instruments the alternatives would breach, and the
`bind`→`set_permissions` window is written down as A-5's residue rather than
hidden. Swept as asked: no other criterion in `plan.md` assumes a umask. **Two
things survive and are raised separately** — VT-10's non-vacuity threshold is
arithmetically wrong (F-16), and `draft-spec.md` §7's R-2 row and `design.md`
§9's AC-10 row both still describe the abandoned mechanism (F-15).

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

**Outcome:** verified — and the repair is better than the finding asked for.
EX-8 is inverted: `examples/shell/backend.sh` is a declared surface and answers
an ingested evaluation with a view naming the event's `source` and `kind`, with
the `"source":"host"` arm preserving today's startup prompt. Checked: `Stimulus`
hard-codes `source: "host"` (`wire.rs:62`), so the arms are disjoint;
`SPEC-003/R-13` refuses an envelope claiming `"host"`, so a person cannot
collide with it; `Event`'s field order is `source, kind, timestamp, data`
(`canonical.rs:490-496`) and `process.rs:63` uses `serde_json::to_vec`, so the
compact-adjacency the extraction relies on holds and `${request#*'"source":"'}`
takes `event.source` rather than anything inside `data`. VH-1 is now five
numbered things a person does and sees, step 3 is an observation a
byte-identical redraw could not produce, and S-4 fires when it does not.
Putting the interpretation in the backend also makes the demo demonstrate the
invariant rather than merely exercise it. **The citation EX-8 offers for the
serialisation facts points at the wrong lines — F-20.**

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

**Outcome:** verified, and fixed as a class rather than an instance. VT-1 now
carries the pin and the one-sentence reason, and states both failure modes.
Above it, **PHASE-05/EX-5** and **PHASE-04/EX-11** state the rule over whole
files, define membership by what an assertion turns on, and enumerate members;
PHASE-05/VT-3..VT-6 each gained an *EX-5 applies* clause naming what it pins and
what goes wrong unpinned. PHASE-05/S-1 is amended so a wrong `next_check`
sequence is read as EX-5 firing rather than as the design being wrong about what
is observable, which was the misdirection the finding named. **PHASE-04/EX-11's
member list omits VT-7, added in the same round — F-17.**

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

**Outcome:** verified — PHASE-04/VT-7 exists, it is in the phase that owns
EX-8, and the assertion it is built around is the right one: *the presentation
count does not advance*, which is the spin, not the fold. The criterion says so
in terms and says a case that only observed the fold would pass against exactly
the failure. The mechanism checks out in this codebase: `bind` uses
`tokio::spawn` onto the entered runtime, so a second multi-thread runtime whose
`enter()` guard `bind` is called under owns the accept task; `Runtime::new` is
safe in an async context (only `drop` and `block_on` are not), and
`shutdown_background()` consumes the runtime without blocking, drops its tasks,
and so drops the sender — the accept task is parked in `accept().await` rather
than running, so it is dropped rather than leaked. S-5 forbids the two easy
escapes. Coverage now names VT-7 under AC-12, R-15's last clause and R-16.
**The case's own timing setup has two gaps — F-17.**

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

**Outcome:** verified — the surface bound now names two things, quotes both
false sentences with their line numbers, assigns the correction to this phase on
PHASE-02's stated discipline, and VA-3's diff description was updated to match,
so the auditor's instrument and the surface bound agree. The discipline is now
applied a third time by PHASE-06 to `renderer/startup.rs`'s module doc, which is
what makes it a rule rather than three separate rulings.

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

**Outcome:** verified — PHASE-04/VA-2 exempts a case that waits out
`MINIMUM_SPACING` by construction, names VT-4, gives the ~1.7x figure and its
cause, cites D-5 for why the rule would otherwise condemn the design, and
requires each case to be recorded as one of the three kinds. S-3 is narrowed to
match. PHASE-08/VA-2 keeps the 10x rule and states why it is safe there —
`ENVELOPE_DEADLINE` is 500 ms and is VT-14's subject. **PHASE-04/VA-2 attributes
the distinction to PHASE-05/VA-2, which does not draw it — F-23.**

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

**Outcome:** verified — PHASE-04/EX-6 states the value where the anchor is
built, argues it as *forced, not chosen* from two independent directions (P-3,
and the fact that `started + MINIMUM_SPACING` is the value a startup evaluation
would have written, which `review-design.md` F-1 forbids), and requires the same
three-line comment `floor_until` carries at `controller.rs:404-407`. The three
criteria that turn on it were re-read and all three read correctly. The response
also does the right thing with what it could not fix: `design.md` §5.3's state
table still has no initial-value column for this row where the row beside it has
one argued in code, and that is reported as the design's rather than filled in
by the plan.

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

**Outcome:** verified — both lints are named where they land, with the denied
spellings, the absence of a hatch (`allow_attributes`,
`allow_attributes_without_reason`), and the four legal routes for the framing.
`pub_use` is correctly placed in PHASE-03 rather than in the phase whose variant
raises it, because PHASE-03 chooses the module layout PHASE-08 inherits, and
PHASE-08's notes point back rather than re-deciding. Overview item 5 no longer
reads as an exhaustive list of two.

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

**Outcome:** verified — the re-argument was attempted honestly and abandoned,
and PHASE-03 is split at the reply (PL-10, recorded in `plan-log.md`). Checked
the seam criterion by criterion: **nothing is lost.** Every original EX, VT, VA
and S is placed — EX-2's three budgets split correctly between PHASE-03/EX-2
(mode) and PHASE-08/EX-11 (the two read budgets, declared with their
enforcement), EX-6's reply splits at `retry_after_ms` (PHASE-08/EX-12), and
EX-5, VT-7..VT-9, VT-13, VT-14 and VA-2 keep their numbers as the header comment
requires. **Nothing is double-owned**: PHASE-08's *Must not touch* names
PHASE-03's own cases and S-6 fires if one goes red. PHASE-03's exit does
establish PHASE-08's entry — EN-2 names `bind`, the five types, EX-10's partial
`Refusal`, the nine cases, and the fake judge's scripted-answer shape, all of
which PHASE-03 delivers. PHASE-04/EN-1 and EN-2, the parallelism rule, both
Coverage tables, PHASE-07/EX-4's sources and the Overview are all updated.
PHASE-04's own sizing is now argued on the dimension 003 argued on rather than
asserted. **Three things about the split are raised separately**: the seam's
effect on the read (F-18), the *Size* paragraph's arithmetic (F-21), and the
readability of the numbering for an agent picking up a sheet (F-22).

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

**Outcome:** verified, and the sweep was real. All three original rows are
corrected, and I re-swept every `path:line` in `plan.md` independently against
the tree — 58 distinct citations, resolved and read. The seven further
corrections the response claims all check out: `Cargo.toml:36-37` is tokio's
entry and `:33-34` is `serde`/`serde_json`; `Refused` spans
`diagnostics.rs:53-61` with `:62` blank; `StartupError` spans `startup.rs:21-42`;
`FLOOR_MILLIS` is at `renderer/scheduling.rs:52`; `NOTHING_INSTRUCTED` opens the
constants at `:30`; `STRATUM_2` is `allowlist.rs:19-26` and `DOMAIN` is
`vocabulary.rs:18-26`; the inner `select!` closes at `controller.rs:514`. FD-4's
three surviving *wrong* values at `plan.md:317-319` are the table quoting the
error beside the correction, which is what a correction table is for.
**Two citations the sweep did not reach are raised as F-20.**

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

**Outcome:** verified — *Surfaces*, EX-8 and VA-3 now all say two edits, the
sentence at `error.rs:16` is quoted so the agent can find it, and the auditor's
instrument counts the same two things the surface bound allows.

### F-15 — three repairs now diverge from `design.md`, and only one of the three divergences is recorded

**Severity:** major
**Location:** `plan.md` PHASE-06/EX-3, PHASE-03/EX-3, PHASE-06/VT-3 + VA-3;
`design.md` §5.4 (the startup diagram), §9 (AC-9 and AC-10 rows);
`draft-spec.md` §7 (R-2 and R-4 rows); `plan.md:3-4`

**Expected:** `plan.md`'s own second line — *"the plan never overrides the
design or canon; if it seems to, the plan is wrong."* `docs/AGENTS.md` §Plan —
*"If any unresolved design issues emerge, go back to the appropriate stage of
design and work forward from there."* And where a divergence is deliberate,
§Audit requires it be findable: *"where the implementation departed and the
design stands as written, say so under **Design drift not reconciled**."*

F-10's repair does this correctly. It states the value the plan will use,
argues it, and then says in terms: *"**Design gap reported, not filled:**
`design.md` §5.3's state table gives `event_floor_until`'s writer, reader and
lifetime but **not its initial value** … That is the design's to close."*

**Observed:** three other repairs changed what the plan will do relative to what
the design says, and none of the three is recorded anywhere as a divergence.

**(a) The startup order.** `design.md` §5.4 draws it:

> `Config::load → clock → backend → Host::new → runtime → runtime.enter()`
> `  → ingress::bind(path)?` …

`Host::new` is *before* the runtime. PHASE-06/EX-3 moves it to *after*
`runtime.enter()` and after the `listener` call — and opens by claiming the
placement is *"`design.md` §5.4's order"*. It is §5.4's order for the **bind**
and a different order for `Host::new`. The move is sound (verified: `Host::new`
is pure construction, `host.rs:127-134`), but the criterion asserts conformance
with the diagram it is departing from, which is the one phrasing that guarantees
nobody reconciles it.

**(b) AC-10's mechanism.** `design.md:666` (§9's AC-10 row) reads *"integration:
`mode() & 0o777 == 0o600`, **under a deliberately permissive umask**"*.
`draft-spec.md:306` (§7's R-2 row) reads *"integration: the bound
socket's mode is `0600` **under a deliberately permissive umask** (AC-10)"*. F-4's repair establishes that no such case can be
written in this workspace and replaces it with `set_permissions`. The plan's own
Coverage table says so; neither of the two documents that state it more bindingly
is flagged.

**(c) AC-9's exit code.** `design.md:665` (§9's AC-9 row) reads *"Stratum 3:
`StartupError::Ingress` renders **and maps to exit 2**"*; `draft-spec.md:308`
(§7's R-4 row) reads *"stratum 3 **maps it to a non-zero exit**"*. F-3's repair
establishes that the mapping cannot be asserted by a test and moves it to
review. Its response correctly flagged the **`slice-004.md`** consequence to the
orchestrator — and stopped there, at one of the three documents that carry the
same claim.

**Why (b) and (c) are not simply PHASE-07's.** `draft-spec.md` §7 is PHASE-07's
surface, so those two rows will be rewritten — but PHASE-07/EX-1's instruction is
*"replacing the prose that names an AC id"* with a test name, which cues an agent
to swap a citation, not to notice that a row describes a method the slice
abandoned. And `design.md` is **nobody's** surface: PHASE-07's *Must not touch*
names it explicitly, PHASE-07's implementer notes say design drift goes under
*Design drift not reconciled* in `audit.md` and that *"this phase's job is to
make sure the auditor can find it"* — and no criterion in PHASE-07 says how, or
lists what to look for. The plan currently carries three known divergences and
records zero of them for the auditor.

**Evidence:** `plan.md:3-4`; `design.md:396` (§5.4's startup diagram, whose
first line is `… → backend → Host::new → runtime → runtime.enter()`), `design.md:665` (§9's AC-9 row) and
`design.md:666` (§9's AC-10 row); `draft-spec.md:306` (§7's R-2 row) and
`draft-spec.md:308` (§7's R-4 row);
`plan.md` PHASE-06/EX-3 (*"`design.md` §5.4's order"*), PHASE-06/VA-3,
PHASE-03/EX-3, Coverage §Acceptance criteria AC-9 and AC-10 rows; `plan.md`
PHASE-07/EX-1, EX-6, *Must not touch*, and its third implementer note;
`review-plan.md` F-10's response (the one that does record its divergence);
`docs/AGENTS.md` §Audit.

**Disposition:** fix-now
**Response:** All three divergences verified at the line: `design.md:396`'s
startup block puts `Host::new` before `runtime`; `design.md:665` says
`StartupError::Ingress` *"renders and maps to exit 2"*; `design.md:666` and
`draft-spec.md:306` both name *"a deliberately permissive umask"*; and
`draft-spec.md:308` says *"stratum 3 maps it to a non-zero exit"*. Fixed at the
source rather than annotated as drift, because in each case **the design is
wrong about what is possible**, and that is amendment, not retro-fitting a
record of intent to a tree that departed from it (`docs/AGENTS.md` §Audit).
No code exists yet for any of the three.

- **The umask.** `design.md` §9's AC-10 row and `draft-spec.md` §7's R-2 row now
  say the host sets the mode **itself** with
  `std::os::unix::fs::set_permissions` after `bind`, and that no case sets a
  umask — with the two reasons: this workspace has no safe umask API (no `libc`
  and no `[dev-dependencies]` in `crates/goad-shell/Cargo.toml`,
  `unsafe_code = "deny"` at `Cargo.toml:74`, and adding either name breaches the
  manifest allowlist at `allowlist.rs:19-26`), and `umask(2)` is process-global
  while `cargo test` runs cases in parallel. `SPEC-003/R-2`'s **content** —
  `draft-spec.md:94`, *the host MUST set the mode itself and MUST NOT rely on
  the umask* — is untouched; only §7's prescribed means was wrong. The AC-10 row
  names §5.5 A-5's `bind`→`set_permissions` window as the residue, so a reader
  who meets the requirement meets the residue with it.
- **`Host::new`'s placement.** `design.md` §5.4's block now runs
  `Config::load → clock → backend → runtime → runtime.enter() →
  ingress::bind(path)? → Host::new → …`, with the one-clause reason in the
  prose above it and in the diagram: `Host::new` **consumes** the `Config`
  (`main.rs:54`, `host.rs:115`), which derives no `Clone` and exposes no
  accessor, so the bind must read `config.ingress` before the host takes it.
  PHASE-06/EX-3 now says it is conformance with the diagram rather than
  claiming §5.4's order while departing from it.
- **AC-9's exit code.** `design.md` §9's AC-9 row and `draft-spec.md` §7's R-4
  row now say the exit code is **held by review, not by a test**, in the shape
  `SPEC-003/R-5`'s row already uses, and give the argument rather than the
  instrument: no test target links the binary, and `main`'s single `match run()`
  maps every `Err` to `ExitCode::from(2)`.

**The missing instrument is added.** **PHASE-07/EX-7** requires a
`## Design drift` section in `notes.md` — one line per departure, saying what
the design says, what the tree does, and which criterion authorised it —
compiled from the phase sheets and EX-3's walk, and it is what `audit.md`'s
*Design drift not reconciled* is written from. It is **seeded** with the two
things known at plan time: F-10's `event_floor_until` gap (a design gap the plan
filled), and these three amendments, listed as amendments so the auditor meets
them as decisions rather than rediscovering them as departures. **PHASE-07/S-4**
stops the phase on a departure no criterion authorised. PHASE-07's third
implementer note, which previously said only that *"this phase's job is to make
sure the auditor can find it"*, now names EX-7 as how, and distinguishes
amending the design (with the user, logged in `design-log.md`) from recording
drift (written down at PHASE-07, reconciled at audit).

`design-log.md` carries one entry for the three amendments, saying plainly that
they were taken **before implementation** because the design prescribed a
mechanism this workspace cannot implement.

**Outcome:**

### F-16 — VT-10's non-vacuity threshold names the wrong umask, and understates the case

**Severity:** minor
**Location:** `plan.md` PHASE-03/VT-10; Coverage §Acceptance criteria, AC-10 row

**Expected:** the repair's own standard — VT-10 is to record the run's umask
against a stated threshold, so a phase agent can tell whether the case
discriminated. That is only useful if the threshold is right.

**Observed:** VT-10 says the case is *"**non-vacuous under any umask more
permissive than `0o077`**, which is every ordinary one (`0o022` is the usual
default); **under `0o077` alone it would also pass against a host that set
nothing**"*, and the AC-10 Coverage row repeats the first half.

`bind(2)` on a `AF_UNIX` socket creates the path with mode `0o777 & ~umask`. So
against a host that set nothing:

| umask | mode a bare `bind` leaves | `& 0o777 == 0o600`? |
|---|---|---|
| `0o022` (usual) | `0o755` | no |
| `0o077` (the value VT-10 names) | `0o700` | **no** — the owner-execute bit |
| `0o177` | `0o600` | yes — the only vacuous case |

The single umask under which the assertion would pass against a host that set
nothing is `0o177`, not `0o077`. `0o700 != 0o600`, and the criterion asserts
`mode() & 0o777 == 0o600` rather than a group/other mask.

The error is conservative — the case is *stronger* than the plan claims — but it
is the number a phase agent is told to check the run against, and it is the one
sentence in the criterion that decides whether VT-10 discharges AC-10 or merely
appears to. It also mis-describes the danger: an agent told to worry about
`0o077` will not think about `0o177`.

**Evidence:** `plan.md` PHASE-03/VT-10 and Coverage AC-10 row;
`plan.md` PHASE-03/EX-3 (`set_permissions(path, from_mode(SOCKET_MODE))`, so the
assertion is over the full `0o777` mask); POSIX/Linux `bind(2)` for `AF_UNIX`
(the socket file is created with `0o777 & ~umask`), which is the same rule
`design.md` §5.5 A-5 relies on when it says *"the socket exists with the ambient
umask's mode"*.

**Disposition:** fix-now
**Response:** Verified. `bind(2)` on an `AF_UNIX` path creates the socket file
with `0o777 & ~umask`, so against a host that sets nothing the assertion
`mode() & 0o777 == 0o600` fails under `0o022` (`0o755`) **and** under `0o077`
(`0o700` — the owner-execute bit, and the criterion asserts the full `0o777`
mask rather than a group/other one). The single vacuous value is `0o177`.

PHASE-03/VT-10 is restated with the arithmetic shown — the two ordinary umasks
and what a bare `bind` leaves under each — and the threshold a phase agent
checks the run against is now `0o177` rather than `0o077`. The Coverage table's
AC-10 row carries the same correction rather than the old half-sentence.

**Outcome:**

### F-17 — PHASE-04/VT-7 is a member of PHASE-04/EX-11 and is not listed, and its own two windows are in tension with no stated way to separate them

**Severity:** minor
**Location:** `plan.md` PHASE-04/EX-11, VT-7

**Expected:** EX-11 defines membership by what an assertion turns on: *"Any
assertion that **counts invocations, counts presentations, or turns on when a
firing happens** is therefore an assertion about the script's `next_check` as
much as about the mechanism, and a case that leaves it unpinned either goes red
against a correct implementation or passes for a reason that has nothing to do
with what it claims."* It then enumerates: *"VT-1, VT-3, VT-4 and VT-5 are all
members here."* PHASE-05/EX-5 does the same job for its file and enumerates all
six of its cases.

**Observed:** VT-7 — added in the same repair round — satisfies EX-11's
definition twice over and is not in its list. Its assertion 2 **counts
presentations** (*"over a window after the fold, the presentation count does not
advance"*) and its assertion 3 **turns on when a firing happens** (*"a scheduled
firing lands at the backend after the fold"*).

This is not bookkeeping. The two assertions are in direct tension: a scheduled
firing causes presentations (`controller.rs:410`, and again at `:488` for the
exchange), so assertion 3's firing must land **outside** assertion 2's window or
assertion 2 goes red against a correct implementation. Arranging that is exactly
a `next_check` pin — the thing EX-11 exists to force — and VT-7 says nothing
about it.

A second gap in the same criterion: assertion 1 is to be read *"off
`Served.controller`'s retained diagnostics"*, which is only available **after
`serve` returns**, while assertion 2 needs a window **during** the run, opened
at a moment the case can only recognise if it can observe the fold live. VT-4 in
PHASE-05 names the live route for exactly this (`get_diagnostic_lines()` off the
window, which `glass.rs:102` writes unconditionally) and names
`Served.controller` as its *fallback*; VT-7 names only the post-hoc one, and its
central assertion cannot use it.

**Evidence:** `plan.md` PHASE-04/EX-11 (its definition of membership and its
member list) and PHASE-04/VT-7 (assertions 1, 2 and 3);
`plan.md` PHASE-05/EX-5 (the same rule, with a complete member list) and
PHASE-05/VT-4 (the live-read route and its fallback);
`crates/goad/src/controller.rs:410` and `:488` (two `glass.present` calls per
exchange-bearing iteration); `crates/goad/src/glass.rs:102`.

**Disposition:** fix-now
**Response:** Verified on both counts. VT-7's assertion 2 counts presentations
and assertion 3 turns on when a firing happens, which is EX-11's membership
definition satisfied twice; and `controller.rs:410` and `:488` are two
`glass.present` calls per exchange-bearing iteration, so an unpinned scheduled
firing inside assertion 2's window turns it red against a correct
implementation.

**EX-11's member list now reads VT-1, VT-3, VT-4, VT-5 and VT-7**, and says VT-7
is a member twice over. **VT-7 states how the two windows are separated**: the
exchange it lets complete before the fold answers with a `next_check` long
enough that no scheduled firing is due inside assertion 2's window, and 3's
firing is provoked only after that window has closed and been asserted — with
the two `glass.present` sites cited, and the note that unpinned it is the
script's default that decides whether they overlap.

The read-route gap is closed in the same criterion: assertion 2 needs the count
**during** the run, so it reads PL-8's counting `Glass` and the window's own
`get_diagnostic_lines()` (`glass.rs:102`, written unconditionally) — the live
route PHASE-05/VT-4 names — while `Served.controller`'s retained diagnostics is
readable only after `serve` returns and is therefore assertion 1's route and
cannot be 2's.

**Outcome:**

### F-18 — the seam leaves the listener's read unbounded for a phase, and makes the read the one piece of production code written twice

**Severity:** minor
**Location:** `plan.md` PHASE-03/EX-7, *Must not touch*, implementer note 3;
PHASE-08/EX-11, implementer note 2; §Sequencing & rationale

**Expected:** the plan rejects a seam that costs a second edit, in terms, when
it argues `serve` must not be split: *"splitting them across phases means
editing the same forty lines twice and leaving a branch nothing drives in
between."* And §Sequencing's defence of the new split claims the property it
needs: *"Each phase writes its own production code red/green … Nothing lands in
one phase that only the next phase's tests exercise."*

**Observed:** the second claim is true and the first is not tested. PHASE-03/EX-7
lands the framing — *"terminated by the first newline **or** by end of input"* —
and PHASE-03's *Must not touch* forbids *"`ENVELOPE_LIMIT`, `ENVELOPE_DEADLINE`
and their enforcement"*. PHASE-08/EX-11 then declares both *"**and the accept
task enforces both**"*. So the read is written unbounded in one phase and
re-written bounded in the next.

Two consequences, neither stated:

- **PHASE-03 ends with an unbounded read from a socket.** `SPEC-003/R-7` —
  *"The host MUST bound every read from a connection in both bytes and time"* —
  is unheld for the length of a phase, and `draft-spec.md` §6.4 calls an
  unbounded read from an untrusted writer *"the defect SPEC-001/R-43 names on
  the other socket"*. Nothing ships and `just check` is green, so this is an
  intermediate state rather than a defect; but PHASE-03's *Must not touch*
  frames the budgets' absence purely as dead-code hygiene (*"a variant nothing
  drives until the next phase"*) and a reader of PHASE-03 alone would not know.
- **PHASE-08's implementer note assumes the read survives.** It says *"the byte
  budget is counted over the same read EX-7 frames. PHASE-03 chose the framing
  spelling; **use it** rather than a second one."* Of PHASE-03's four offered
  spellings, the most natural — `AsyncBufReadExt::read_until` — admits no byte
  cap and no deadline: adding them means changing the reader's construction
  (`BufReader::new(stream.take(LIMIT))`, or a hand-rolled loop) and wrapping in
  `tokio::time::timeout`. PHASE-03's note lists the four routes without saying
  which one survives a later cap, so the phase that must live with the choice is
  not the phase making it.

**Evidence:** `plan.md` §Sequencing & rationale (*"editing the same forty lines
twice"*, and *"Each phase writes its own production code red/green"*);
`plan.md` PHASE-03/EX-7, PHASE-03 *Must not touch* (the budgets clause), and
PHASE-03 implementer note 3 (the four legal routes);
`plan.md` PHASE-08/EX-11 and implementer note 2; `draft-spec.md` R-7 and §6.4.

**Disposition:** fix-now
**Response:** Verified: PHASE-03/EX-7 landed the framing while PHASE-03's *Must
not touch* forbade `ENVELOPE_LIMIT`, `ENVELOPE_DEADLINE` and their enforcement,
and PHASE-08/EX-11 then declared and enforced both over the same read. The read
was written unbounded and rewritten bounded, and `SPEC-003/R-7` was unheld for a
whole phase.

**The budgets move to PHASE-03**, and the seam moves with them. PHASE-03 now
carries `EX-11` (both constants **and** their enforcement, with the read they
bound), `VT-13` (`too_large`) and `VT-14` (`timed_out`), `VA-2` (the margin
check, whose subject is `ENVELOPE_DEADLINE`) and `S-3` (the 10x rule). Its
`EX-10` grows from three variants to **five** — `TooLarge { limit }` and
`TimedOut { after }` are now constructed by this phase's own code, which is what
makes the budgets' answers coherent with the `Refusal` it lands — and its *Must
not touch* now forbids what actually remains PHASE-08's: any variant beyond
those five, `retry_after_ms`, and `reserved_source` as a distinct wire reason.
PHASE-03/S-2 is widened to cover framing **and** both bounds as one decision,
and says in terms that a spelling admitting the framing but not the cap is a
reason to choose another spelling rather than to defer the cap. The
`indexing_slicing` note now says which of its four legal routes survives the cap:
`read_until` admits neither a byte cap nor a deadline on its own, so taking it
means `stream.take(ENVELOPE_LIMIT)` and `tokio::time::timeout` — decided in the
same sitting, because it is one piece of code.

**PHASE-08 keeps the writer-facing vocabulary and nothing else.** `EX-5` now
adds the two payloads PHASE-03 had no code to construct (`Engaged`,
`TooSoon { retry_after }`) beside PHASE-03's five, and still closes the set at
eight; `EX-12` and `EX-13` are unchanged; `VT-7` is re-scoped to the three
reasons this phase owns (`malformed`, `invalid_envelope`, `reserved_source`) and
says in terms that `too_large` and `timed_out` are read off the wire by
PHASE-03/VT-13 and VT-14 **with the enforcement that produces them**, so no
reason goes unasserted and none is asserted twice. PHASE-08 adds no bound,
re-writes no read, and now has no timed case at all — stated in its Verification
preamble, which is why it has no margin criterion and no margin STOP.

**§Sequencing is rewritten, not patched.** The seam is now named as the
**refusal vocabulary**, with a paragraph saying why it is not the reply: the read
is one piece of production code and must be written once, and the alternative
leaves `SPEC-003/R-7` unheld for a phase — the defect `draft-spec.md` §6.4 names
on the other socket. The red/green claim is re-checked and restated truthfully:
PHASE-03's budgets land with VT-13 and VT-14, which drive them; `Refusal` carries
in PHASE-03 only the five variants PHASE-03's own code constructs; PHASE-08 adds
no bound. Nothing lands in one phase that only the next phase's tests exercise.

**PHASE-03's size is re-checked against F-12's argument, and the answer is
stated rather than absorbed.** The halves are now **eleven cases and three**,
which is not even and is not meant to be — the seam is at the subject, not the
case count. Against slice 003's eight-case calibration, PHASE-03's eleven are
about **nine distinct conditions**: VT-13 and VT-14 are two further arms of the
**same** read VT-5 already exercises three arms of, under the same fixture and
the same fake judge, so they cost little re-reading. That is over the
calibration by one condition rather than by three cases. The *Size* paragraph
says so in those terms, names PHASE-03 as the largest phase in the slice and the
likeliest to want a `PARTIAL` checkpoint, and states what is bought: the read is
written once and bounded from the moment it exists.

Recorded as **PL-11** in `plan-log.md`, with PL-10 marked superseded in part on
where the seam falls.

**Outcome:**

### F-19 — `SPEC-003/R-13`'s Coverage row was not carried across the split, and the criterion the split created for it is named nowhere

**Severity:** minor
**Location:** `plan.md` Coverage §`draft-spec.md`'s requirements, R-13 row;
PHASE-08/EX-13; PHASE-08/VT-7

**Expected:** the Coverage tables were walked as part of the split — the R-7,
R-14, R-15 and R-16 rows all gained their PHASE-08 or PHASE-04/VT-7 references,
and the AC-4, AC-9, AC-10, AC-12 and AC-13 rows were all rewritten.

**Observed:** the R-13 row still reads, whole: *"R-13 — `source: "host"` refused
| **PHASE-02/VT-9**"*.

`SPEC-003/R-13` requires that such an envelope *"MUST be refused"*, and
`draft-spec.md` §6.3 gives `reserved_source` a wire reason of its own, distinct
from `invalid_envelope`. PHASE-02/VT-9 asserts the **`EnvelopeFault`** — the
unit-level half. The wire-level half is now owned by two criteria the split
created or moved, and neither is named in R-13's row:

- **PHASE-08/EX-13** — *"the fault-to-reason mapping PHASE-03/EX-10 left partial
  is completed: `reserved_source` is its own wire reason rather than
  `invalid_envelope` (`SPEC-003/R-13`, CD-2)"*. It cites R-13 itself.
- **PHASE-08/VT-7** — reads `reserved_source` off the wire.

Before the split, nothing owned that mapping and the row's incompleteness was
invisible. The split created the criterion that owns it — and cited R-13 from
the criterion rather than the criterion from R-13, which is the direction
PHASE-07/EX-3 walks (*"this plan's two Coverage tables are walked against the
tree: every criterion id names a test that exists"*). Under PHASE-03's partial
`Refusal` a reserved-source envelope is answered `invalid_envelope`, which is
wrong against `draft-spec.md` §6.3; that is deliberate and temporary, and R-13's
row is where the plan should say which criterion ends it.

**Evidence:** `plan.md` Coverage §`draft-spec.md`'s requirements, R-13 row
versus the R-7, R-14, R-15 and R-16 rows; `plan.md` PHASE-08/EX-13 and VT-7;
`plan.md` PHASE-03/EX-10 (the three variants, `reserved_source` not among them);
`draft-spec.md` R-13 and §6.3's reason table.

**Disposition:** fix-now
**Response:** Verified: the R-13 row read *"PHASE-02/VT-9"* whole, and neither
PHASE-08/EX-13 nor PHASE-08/VT-7 was named from it.

The row now carries both halves and says what separates them: PHASE-02/VT-9 is
the `EnvelopeFault`, unit-level; PHASE-08/EX-13 + PHASE-08/VT-7 are
`reserved_source` as its own wire reason, read off the wire. It also states the
temporary wrongness the split created and names its end — under PHASE-03/EX-10's
partial mapping a reserved-source envelope is answered `invalid_envelope`, which
`draft-spec.md` §6.3's reason table contradicts, and **PHASE-08/EX-13 is the
criterion that ends it**. The citation now runs in the direction PHASE-07/EX-3
walks: from the requirement to the criterion.

PHASE-08/VT-7 was re-scoped by F-18 in the same round and names `reserved_source`
and `SPEC-003/R-13` explicitly, so the row and the criterion now cite each other.

**Outcome:**

### F-20 — the citation sweep missed two, and both are worse than an off-by-one

**Severity:** minor
**Location:** `plan.md` PHASE-06/VA-3; PHASE-06/EX-8; FD-4

**Expected:** F-13's repair claims a complete sweep: *"**every** `path:line` in
`plan.md` was swept, not only FD-4's rows"*, listing the remaining citations as
*"checked and hold"*.

**Observed:** I re-swept all 58 distinct `path:line` citations in `plan.md`
independently, resolving each against the tree and reading the lines. The seven
corrections all hold. Two citations do not, and neither is an off-by-one —
each points at real content that is not what the sentence claims.

**(a) An ambiguous file, in the one phase that declares both.**
PHASE-06/VA-3 ends: *"**Do not build a binary-running harness for it**:
`startup.rs:9-11` is that file's rule and PL-9 forbids a helper binary."* The
rule is at `crates/goad/tests/renderer/startup.rs:9-11`. But
`crates/goad/src/startup.rs:9-11` also exists and says something else entirely —
it is the tail of `Launch`'s doc comment and the `#[derive]` above it. PHASE-06
declares **both** files as surfaces (`crates/goad/src/startup.rs` and
`crates/goad/tests/renderer/startup.rs`), so the bare form resolves ambiguously
inside the phase that has most reason to confuse them. The same phase's VT-3
writes the full path correctly, which is what makes VA-3's short form a slip
rather than a convention.

**(b) A citation that supports neither fact it is offered for.**
PHASE-06/EX-8 — the criterion F-5's whole repair rests on — says: *"Extract
`source` and `kind` with shell parameter expansion … (**the host serialises
compactly, so `"source":"…","kind":"…"` are adjacent — `canonical.rs:512-531`**)"*.
`canonical.rs:512-531` is `struct Envelope<'a>` and `enum Body<'a>` — the
request's outer `{"protocol":1,"type":"evaluate","now":…,"event":…}` shape. It
shows neither claim. The two facts are real and are elsewhere: field order is
`Event`'s declaration (`canonical.rs:490-496`: `source`, `kind`, `timestamp`,
`data`), and compactness is `serde_json::to_vec` at
`crates/goad-shell/src/backend/process.rs:63`. Both matter to the extraction —
order is why `${request#*'"source":"'}` takes `event.source` and not a
`"source"` key a watcher put inside `data`, and compactness is why the substring
matches at all.

FD-4's closing sentence should also now be read with care: it lists the
citations *"checked and hold"* and both of these were in scope.

**Evidence:** `plan.md:1581` (PHASE-06/VA-3) versus `plan.md` PHASE-06/VT-3,
which writes `crates/goad/tests/renderer/startup.rs:9-11` in full;
`crates/goad/src/startup.rs:9-11` and `crates/goad/tests/renderer/startup.rs:9-11`
(different content, both real); `plan.md` PHASE-06/EX-8;
`crates/goad-semantics/src/protocol/canonical.rs:505-531` (`Envelope` and
`Body`) versus `:489-496` (`Event`'s four fields in order);
`crates/goad-shell/src/backend/process.rs:63` (`serde_json::to_vec`).

**Disposition:** fix-now
**Response:** Both verified. (a) `crates/goad/src/startup.rs:9-11` is real and is
the tail of `Launch`'s doc comment plus the `#[derive]`, while the rule VA-3
means is at `crates/goad/tests/renderer/startup.rs:9-11`; PHASE-06 declares both
files. (b) `canonical.rs:512-531` is `struct Envelope<'a>` and `enum Body<'a>`
and shows neither field order nor compactness.

- **VA-3** now writes `crates/goad/tests/renderer/startup.rs:9-11` in full, and
  says why in a parenthesis — this phase declares two files named `startup.rs`,
  and the other one's `:9-11` is real and is something else.
- **EX-8** now cites the two facts where they actually live:
  `canonical.rs:490-496` for `Event`'s declaration order (`source`, `kind`,
  `timestamp`, `data`) and `crates/goad-shell/src/backend/process.rs:63` for
  `serde_json::to_vec`, with the consequence spelled out — the substring match
  takes `event.source` rather than a `"source"` key a watcher put inside `data`.

**Swept again for the same two shapes**, independently of F-13's sweep.

- *Ambiguous bare filename inside a phase declaring two of that name.* One more:
  **PHASE-06/EX-1**'s `(`startup.rs:21-42`, `main.rs:21-29`)`.
  `crates/goad/src/startup.rs:21-42` is `StartupError` with its derive — the
  eight siblings the criterion means — while
  `crates/goad/tests/renderer/startup.rs:21-42` is real (the file is 353 lines)
  and is two helper functions and the head of a `usage` test. Both are now
  written in full, as is **PHASE-06/EX-2**'s `crates/goad/src/startup.rs`.
  `main.rs:103` in PHASE-04 was checked for the same hazard and does **not**
  have it: `crates/goad/tests/renderer/main.rs` is 49 lines.
- *A citation offered for a fact it does not carry.* One narrowing:
  PHASE-04/S-2's `wire.rs:62` is `pub fn event(…)`, and the `source: "host"`
  literal it is offered for is at `:64`; it now reads `wire.rs:62-64`.

Spot-checked and holding, unchanged: `renderer/scheduling.rs:52` (`FLOOR_MILLIS`
and its comment), `:141` (`absorbed_line`), `:30-37` (all three `INSTRUCT_*`
constants), `Cargo.toml:36-37`, `:74`, `:142`, `:183`,
`crates/goad-semantics/src/schedule.rs:73-86` (the jiff two-step),
`diagnostics.rs:53-61`, `crates/goad/tests/renderer/main.rs:1` and `:7`,
`crates/goad/tests/renderer/startup.rs:1-2` and `:9-11`, `main.rs:21-29`,
`controller.rs:410` and `:488`, `glass.rs:102`, `canonical.rs:490-496`,
`process.rs:63`, and `docs/slices/003/plan.md:129-141`.

**Outcome:**

### F-21 — the *Size* paragraph miscounts the split it exists to justify

**Severity:** nit
**Location:** `plan.md` §Sequencing & rationale, *Size*

**Expected:** the split's justification is arithmetic against a calibration:
*"Slice 003's PHASE-02 had eight cases sharing one new `harness.rs`."* The
sentence that closes the argument is *"Split at the reply, **each half is eight
cases and six** against one module, which is the calibration rather than a claim
about it."*

**Observed:** neither number is the split that was made. PHASE-03's verification
list is VT-1, VT-2, VT-3, VT-4, VT-5, VT-6, VT-10, VT-11, VT-12 — **nine**.
PHASE-08's is VT-7, VT-8, VT-9, VT-13, VT-14 — **five**. The total is 14, which
matches; the halves do not. (F-12's own response says *"eight cases (VT-1..VT-6,
VT-10, VT-11, VT-12)"* — a list of nine described as eight, which is where the
error entered.)

Nine against a calibration of eight is still the calibration; the paragraph does
not need a different conclusion, it needs the right count. It is a nit rather
than more because the argument survives — but this is the one paragraph in the
plan whose whole content is a comparison of numbers.

**Evidence:** `plan.md` PHASE-03 *Verification* (nine VT entries) and PHASE-08
*Verification* (five); `plan.md` §Sequencing & rationale, *Size*, final
paragraph; `review-plan.md` F-12's Response.

**Disposition:** fix-now
**Response:** Verified: PHASE-03 had nine VT entries and PHASE-08 five, against
a paragraph claiming eight and six.

The paragraph is rewritten rather than renumbered, because F-18 changed the
split it describes in the same round. It now states the arrangement that
actually exists — **eleven cases and three**, PHASE-03's VT-1..VT-6 and
VT-10..VT-14 against PHASE-08's VT-7..VT-9 — says that the halves are uneven
because the seam is at the subject rather than the case count, and then makes
the comparison the paragraph exists for **in the unit the calibration was argued
in**: eleven cases are about nine distinct conditions, because VT-13 and VT-14
are further arms of the read VT-5 already exercises. The conclusion is stated as
one condition over the calibration rather than as parity, with what is bought for
it and what happens if the phase overruns.

**Outcome:**

### F-22 — the split's id bookkeeping is stated in one phase and incompletely, so the gaps read as omissions

**Severity:** nit
**Location:** `plan.md` PHASE-08 preamble; PHASE-03 *Exit*, *Verification*, *STOP*

**Expected:** the file's header comment says a non-monotonic sequence after a
split is expected, and PHASE-08's preamble undertakes to say which ids moved:
*"the criteria that were PHASE-03's VT-7..VT-9, VT-13, VT-14 and VA-2 keep their
numbers here … EX-5 likewise; **EX-11 and EX-12 are new**."* A phase sheet is
expanded from one phase's entry, so an agent reading PHASE-03 or PHASE-08 alone
must be able to tell a moved id from a missing one.

**Observed:** the enumeration is incomplete on both sides of the seam.

- **PHASE-08's "new" list is short.** `EX-13`, `VA-4` and `VA-5` are also new and
  are not named. `VA-3` is skipped entirely — PHASE-08 runs VA-1, VA-2, VA-4,
  VA-5 — although criterion ids are *local to their phase*, so nothing forced
  the gap; VA-4 duplicates PHASE-03/VA-3's content under a different number. The
  STOP list jumps S-1, S-2, S-3, S-6 with no note.
- **PHASE-03 says nothing at all.** Its exits run EX-1..EX-4, EX-6..EX-10 (EX-5
  gone); its cases run VT-1..VT-6, VT-10..VT-12; its checks run VA-1, VA-3
  (VA-2 gone); its STOPs run S-1, S-2, S-4, S-5 (S-3 gone). Four gaps, none
  explained where the agent meets them. PHASE-03/EX-2 and EX-6 do point forward
  to `PHASE-08/EX-11` and `PHASE-08/EX-12` for the *content* that left, which is
  the right instinct applied to two of the nine gaps.

One line under each phase's heading — *these ids moved to / from PHASE-NN* —
would close it. As it stands the header comment is the only thing standing
between a phase agent and the conclusion that a criterion was dropped.

**Evidence:** `plan.md` PHASE-08 preamble (paragraph 3) against PHASE-08's own
*Exit*, *Verification* and *STOP* lists; `plan.md` PHASE-03's *Exit*,
*Verification* and *STOP* lists; `plan.md:6-12` (the header comment); `plan.md`
PHASE-03/EX-2 and EX-6 (the two forward pointers that do exist).

**Disposition:** fix-now
**Response:** Verified: PHASE-08's *new* list named `EX-11` and `EX-12` and
omitted `EX-13`, `VA-4` and `VA-5`; `VA-3`, and `S-2`..`S-5`, were skipped with
no note; and PHASE-03 said nothing at all about its own four gaps.

**Both phases now carry an *Ids across the split* paragraph, immediately under
the phase's opening and above its Surfaces** — where an agent expanding a phase
sheet from that phase alone will meet it.

- **PHASE-03**: the four ids that left — `EX-5`, `VT-7`, `VT-8`, `VT-9` — named
  as gone to PHASE-08, `EX-11` named as new and this phase's, and the statement
  that those four are the **only** gaps in its lists.
- **PHASE-08**: which ids are PHASE-03's kept whole (`EX-5`, `VT-7`, `VT-8`,
  `VT-9`), which are new (`EX-12`, `EX-13`, `VA-4`, `VA-5`), why `VA-4` could
  not take the number `VA-3` — that number is PHASE-03's — and an enumeration of
  every skipped id (`EX-1..EX-4`, `EX-6..EX-11`, `VT-1..VT-6`, `VT-10..VT-14`,
  `VA-2`, `VA-3`, `S-2..S-5`) with where each stayed.

Both say nothing was renumbered and cite the header comment's rule. The lists
are stated against the phases as F-18 leaves them, so the bookkeeping and the
structure were written in the same pass rather than one trailing the other.

**Outcome:**

### F-23 — two cross-references introduced by the repairs say the opposite of what they point at

**Severity:** nit
**Location:** `plan.md` PHASE-04/VA-2; PHASE-08/EX-5 versus PHASE-08/VT-9

**Expected:** a repair that borrows an argument from elsewhere should point at
text that carries it, since the point of the pointer is that the reader can go
and check.

**Observed:** two, both introduced in this round.

**(a)** PHASE-04/VA-2, repairing F-9, says: *"The distinction is the one
**PHASE-05/VA-2 already draws**: a liveness bound admits a ratio; an anti-fire
window, and a wait that is the bound under test, do not."* PHASE-05/VA-2 reads,
whole: *"the per-test elapsed time of VT-1..VT-6 recorded against the bound each
governs. VT-2 spends a floor interval by construction — three seconds of gate
time, deliberately, because that is what the case costs. Record it as such."* It
records one case and states no margin rule at all, so it draws no distinction
between kinds of bound. The nearest thing is PHASE-05's **fourth implementer
note**, which contrasts `until(LIVENESS_BOUND, …)` with an anti-fire window —
about how to *write* a case, not about what ratio to accept. PHASE-04/VA-2
states the distinction fully in its own text, so no agent is stranded; the
citation is simply false.

**(b)** PHASE-08/EX-5 says the reason set closes at eight and then: *"`engaged`
and `too_soon` **are constructed by no code in this phase** — they are
PHASE-04's, and PHASE-04/EN-2 is what they are here for."* PHASE-08/VT-9 says:
*"**The judge is scripted to answer with `Refusal::TooSoon { retry_after }`**, so
no `serve` is needed."* Both are right under the reading *no **production** code
in this phase constructs them*, and that reading is almost certainly what EX-5
means — but EX-5 does not say it, and it is the criterion, while VT-9 is the
case that has to contradict it to exist.

**Evidence:** `plan.md` PHASE-04/VA-2 versus PHASE-05/VA-2 and PHASE-05's fourth
implementer note; `plan.md` PHASE-08/EX-5 versus PHASE-08/VT-9.

**Disposition:** fix-now
**Response:** Both verified. PHASE-05/VA-2 records VT-2's floor interval and
states no margin rule, so it draws no distinction between kinds of bound; and
PHASE-08/EX-5 said *"constructed by no code in this phase"* where PHASE-08/VT-9
scripts a `Refusal::TooSoon`.

- **PHASE-04/VA-2** no longer attributes the distinction to PHASE-05/VA-2. It
  states it in its own right — *stated here because no other criterion states it
  as a margin rule* — and points at what does carry the nearest thing,
  PHASE-05's **fourth implementer note**, with the note that it is about how to
  *write* a case rather than what ratio to accept.
- **PHASE-08/EX-5** now says **no *production* code in this phase** constructs
  `engaged` or `too_soon`, and then says what VT-9 does: it constructs a
  `Refusal::TooSoon` in **test** code by scripting the fake judge, which is how
  EX-12 is driven without `serve`. The criterion and the case now say the same
  thing.

**Outcome:**

### F-24 — the anchor's boundary is stated in two places and both overshoot: `engaged` can refuse the first envelope, and the comparison that makes the initial value mean what it says is written nowhere

**Severity:** minor
**Location:** `plan.md` PHASE-04/EX-6; `design.md` §5.3 (the paragraph added
under F-10); `design.md` §5.4 steps 2–3; `draft-spec.md` R-14, §6.3

**Expected:** F-10 asked for `event_floor_until`'s initial value to be written
down where the anchor is built. Both documents now do that, and they agree with
each other: initialised to `started`, forced rather than chosen, because
`started + MINIMUM_SPACING` is the value the host's own **startup** evaluation
would have written and P-3 denies that a firing of one class writes another
class's anchor. That argument is sound, and it is the argument the design and
the plan now both make. **This finding is not about the value.**

**Observed:** it is about the two sentences that state the *consequence*. Both
reach past what the anchor decides.

**(a) `accepted` is not the anchor's to promise.** `design.md` §5.3 closes:
*"So the first envelope after startup is accepted, and **nothing about the
startup evaluation is observable at the socket**."* PHASE-04/EX-6 says the same
in fewer words: *"already elapsed by the time anything runs, so **the first
envelope after startup is accepted**."*

The order of judgement has five steps and the anchor is step 3. An envelope that
arrives while the startup exchange is still in flight is refused at **step 2** —
`engaged` — and nothing exempts the startup exchange from that: `draft-spec.md`
§6.3 defines `engaged` as *"an exchange was already in flight"*, and
`design.md` §5.5's own edge-case table already contemplates an envelope arriving
that early (*"an envelope arriving between `bind` and `serve` starting"*). The
startup evaluation is therefore observable at the socket, as `engaged`, for as
long as the backend takes to answer — and under `examples/demo.toml` that is a
`bash` process spawn.

What the argument actually supports is the narrower claim: *the startup
evaluation does not write the event anchor, so it never makes an envelope
`too_soon`.* Everything past that belongs to step 2.

No criterion breaks on it today — the renderer cases seed
`Command::Evaluate(Stimulus::Requested)` explicitly
(`renderer/scheduling.rs:168` and its fifteen siblings) rather than inheriting a
startup evaluation, so PHASE-04/VT-1 has no exchange to race unless its author
adds one, and VH-1's person is seconds late. It is the **statement** that is
wrong, in the one place the repair was asked to be exact, and PHASE-04/VT-1's
author is the reader most likely to rely on it.

**(b) The comparison is unspecified, and `SPEC-003/R-14` forces it.**
Neither document says whether step 3 refuses on `now < event_floor_until` or
`now <= event_floor_until`. That is not a detail of the initial value — it is
what makes the initial value mean what both documents say it means. Initialised
to `started`, the anchor equals `started`; *"already elapsed"* is true under `<`
and false under `<=` at that instant, and true under both a moment later.

`draft-spec.md` R-14 settles it and is cited by neither: *"`retry_after_ms` …
**rounded up** … after which the spacing will have elapsed. Rounding up is
required rather than incidental — a truncated remainder leaves a writer that
waits exactly that long **still inside the spacing**, which would make this
requirement's own sentence false of the host's own field."* Rounding up
guarantees a writer that waits exactly `retry_after_ms` arrives at
`now >= event_floor_until`, and when the remaining spacing is a whole number of
milliseconds it arrives at exactly `event_floor_until`. Under `<=` that writer
is refused and R-14's sentence is false of the host — the same defect
`review-design.md` F-16 raised about the rounding, one layer down and on the
other side of the same equation.

Nothing would catch it. PHASE-08/VT-9 asserts the *value* of `retry_after_ms`
against a scripted judge with no `serve` in the picture; PHASE-04/VT-4 refuses a
second envelope well inside the spacing. Neither is at the boundary. The plan
specified the rounding to the nanosecond
(`checked_add(Duration::from_nanos(999_999))`) precisely because *"waiting
exactly that long"* is load-bearing; the comparison it is load-bearing against
is left to the implementer.

**Evidence:** `design.md` §5.3, the paragraph beginning *"Both anchors start
already elapsed"*; `plan.md` PHASE-04/EX-6; `design.md` §5.4's order of
judgement, steps 2 and 3, and §5.5's edge-case row *"an envelope arriving
between `bind` and `serve` starting"*; `draft-spec.md` R-14 and §6.3's
`engaged` row; `crates/goad/tests/renderer/scheduling.rs:168` (the renderer
tier seeds `Stimulus::Requested` explicitly, so no startup exchange is
inherited); `plan.md` PHASE-08/VT-9 and PHASE-04/VT-4 (neither at the
boundary); `review-design.md` F-16.

**Disposition:** fix-now
**Response:** Both parts verified. §5.4's steps are 1 shape / 2 `engaged` /
3 `too_soon`, and nothing exempts the startup exchange from step 2;
`draft-spec.md:236` defines `engaged` as *"an exchange was already in flight"*
and `design.md:576` already carries the *between `bind` and `serve` starting*
row. `draft-spec.md:106` (R-14) says in terms that rounding up exists so a
writer waiting exactly that long is **not** still inside the spacing. And
`renderer/scheduling.rs:168` does seed `Stimulus::Requested` explicitly, so no
criterion breaks today — the statement is what is wrong.

**(a) The consequence is narrowed, in both documents.** `design.md` §5.3 now
closes: *"So the startup evaluation never makes an envelope `too_soon`, which is
the whole of what the anchor decides: the anchor is §5.4's step 3, and an
envelope arriving while the startup exchange is still in flight is refused at
step 2, `engaged`, like any other."* PHASE-04/EX-6 says the same and adds the
consequence for its reader — *a case that assumes acceptance rather than the
narrow claim is testing its own fixture* — citing §6.3 and §5.5's edge-case row.
One sentence of `design.md` was replaced, under the authority the orchestrator
extended for it, and nothing else in §5.3 or §5.5 was touched.

**(b) The comparison is specified, and is now testable.** PHASE-04/EX-6 states
that step 3 refuses on `now < event_floor_until` and **never** on `<=`, with
`SPEC-003/R-14` as the forcing reason written out: rounding up puts a writer
that waits exactly `retry_after_ms` at `now >= event_floor_until`, and exactly
*at* it when the remaining spacing is a whole millisecond, so under `<=` R-14's
own sentence would be false of the host — `review-design.md` F-16's defect from
the other side of the same equation. It is also what makes EX-6's *"already
elapsed"* true of an anchor equal to `started`. EX-5's step 3 carries a pointer
so the order-of-judgement list and the anchor say the same thing.

**The comparison becomes a named private free function** (EX-6), because
otherwise nothing can reach the boundary — and **PHASE-04/VT-8** is the case:
three assertions in `controller.rs`'s **existing** `#[cfg(test)] mod tests`
(`:530`), at the floor, one nanosecond before, one nanosecond after. VT-8 states
why it cannot be a socket case: `retry_after_ms` rounds up and a real `sleep`
overshoots on top of that, so an end-to-end waiter arrives strictly past the
floor and **both** comparisons accept it — the reviewer's observation that
PHASE-08/VT-9 and PHASE-04/VT-4 are not at the boundary is true of *any*
socket-level case, not just those two. The precedent is the one
`controller.rs:524-530` already states in the same file for `deadline_after` and
`stamp`. Coverage's R-12 and R-14 rows both name PHASE-04/VT-8, R-14's saying
what it holds that PHASE-08/VT-9 cannot. PHASE-04/EX-11's member list now also
says which cases are **not** members and why — VT-2, VT-6 and VT-8 — so the
non-membership is a statement rather than an omission.

`design.md` §5.4's step 3 still does not name the comparison, and that is
**reported, not filled**: EX-6 records it as a design gap in the shape F-10's
repair used, and PHASE-07/EX-7's seeded list carries it to the auditor.

**The id collision, folded into F-22's remedy — and it is wider than reported.**
`VT-7` names **three** criteria, not two: PHASE-02/VT-7 (a duplicate key at
depth), PHASE-04/VT-7 (the closed channel) and PHASE-08/VT-7 (the shape
reasons). This repair makes `VT-8` a third-way clash too — PHASE-02/VT-8 (the
offsetless instant), PHASE-08/VT-8 (the token set) and the new PHASE-04/VT-8,
which is PHASE-04's next free number and not renumberable. All six are named in
three places: the file's **header comment**, where the phase-local rule is
stated and which every reader meets; **both** *Ids across the split* paragraphs,
because those are what discuss VT-7..VT-9 moving and so put the number in front
of a reader in a context suggesting it is one criterion; and at
**PHASE-04/VT-7** itself. Every citation of all six was already
phase-qualified and still is.

**One correction to the brief**, which changes nothing about the repair: `VT-7`
is not *the only* duplicate id in the plan. Ids are phase-local by the header
comment's own rule, so `VT-1` names five different tests (PHASE-02, 03, 04, 05,
06) and VT-2..VT-6 recur similarly. What is true, and what the repair records,
is that VT-7 and VT-8 are the numbers a reader is likeliest to conflate,
because the split's bookkeeping discusses exactly those numbers moving. The
header comment says both things — the general rule, and why these two are
singled out.

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

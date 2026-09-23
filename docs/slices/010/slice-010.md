# Slice 010: the exit-code taxonomy

**Stage:** executing — `plan.md` accepted 2026-09-23 at `448f678`, no plan review (`plan-log.md`)
**Tier:** 2 (full) — the slice writes new canon: a spec owning the exit status
of this project's binaries, and an amendment to SPEC-003 — its R-4 and R-3
verification cells, and its References.
**Depends on:** —

## Purpose

The host's exit status cannot say whether it never started or stopped running.
`StartupError::Platform` is raised at four call sites, three before the event
loop and one at its end, and `main` maps every `Err` to 2. A supervisor reading
2 is told a person must intervene — which is right for a bad configuration and
wrong for a host that ran for hours and lost its display.

Measured on the running service (`research.md` §Thread 3): six such exits in
two days, every one with `RestartPreventExitStatus=2` suppressing the restart.
Four were rescued in 9–20 seconds by the session target happening to cycle. The
other two left the host down for **2h 12m** and **1h 59m** — and in both the
compositor never went away, so a restart two seconds later would have worked.

Once this lands: three statuses cut on **phase**, a spec that says what each
means and what a supervisor may infer, and a `nix/module.nix` whose reasoning
matches its directives. The two-hour outage becomes a two-second one.

## Scope

**Code.**

- `main`, `run` and `start` (`crates/goad/src/main.rs`) — the exit decision,
  and the seam that separates the loop's ending from the failures before it.
- `StartupError` (`crates/goad/src/startup.rs`) — it stops carrying the loop's
  ending, which makes its own doc comment true again.
- `crates/goad/src/exit.rs` — **new** (endorsed 2026-09-23, `design-log.md`).
  It owns how the process ends: the value `run` answers, `exit::ended` — the
  pure decision over the event-loop call's result and whether a stop was
  requested — and the number. It is not `startup.rs`, whose stated job is what `run` needs *before* the loop, and
  not `diagnostics.rs`, which is what a person reads.
- `crates/goad/src/diagnostics.rs` — the stderr lines the new class writes,
  one for an end the call reported an error for and one for an end it did not.
  `report_startup`, the impure outlet with one caller, is replaced by
  `report_exit`; `report_startup_line` keeps its name, signature and text.
- `crates/goad/tests/renderer/startup.rs` — the arms, **and now the numbers**,
  since they become a pure function's answers, and `exit::ended`'s arms; its
  module doc states the moved two-tier cut, and `crates/goad/tests/binary/exit_codes.rs`'s states the other
  half — that the **process** answers them to a caller.
- `crates/goad/src/wire.rs` — **`Cancel::is_stopped`**, a synchronous read of
  the signal `Cancel` already holds (endorsed 2026-09-23, `design-log.md`), and
  its case in the module's own tests. `start` passes it to `exit::ended`, so a
  stop that was asked for reaches 0 however the event-loop call reports it. No
  new state: `Cancel` already keeps its own receiver.
- `crates/goad/tests/binary/exit_codes.rs` — **one case added**,
  `an_unbindable_ingress_path_exits_2`, asserting the status and the stderr
  prefix only the ingress arm writes, over a configuration that loads
  (endorsed 2026-09-23, `design-log.md`). AC-5 is unaffected: every existing
  case still passes unmodified, and an addition is not a modification.
- `crates/goad/tests/binary/process.rs` — `command` removes the display
  variables from every spawn, so no case or mutant at that tier can open a
  host; and the module doc of `crates/goad/tests/binary/main.rs`, which states
  that tier's headless premise (endorsed 2026-09-23, `design-log.md`, P-1).
- `crates/goad-boundary/tests/checks/structure.rs` — **one case**,
  `the_loop_s_ending_is_never_a_startup_failure`, the guard for `design.md` §8
  R2 (endorsed 2026-09-23, `design-log.md`): the event-loop call has one
  production line, and nothing is applied to its result there. Its predicate's
  controls go in the file's `counting_itself` module. No new file, no new
  command, and no canon amended.

**Packaging.** `nix/module.nix` — its exception paragraph, and the argument it
makes for `RestartPreventExitStatus`. The directives themselves are expected to
stand unchanged.

**Canon.** `draft-spec.md`, numbered only at promotion; `canon-delta.md` for
SPEC-003 — R-4's verification cell, which is stale in one sentence and asserts
the defect as a fact in another; R-3's, whose *"same position"* analogy that
repair falsifies; and a §9 References entry for the new spec.

**At close.** `docs/follow-ups.md` FU-1 — struck, with its count, its cause and
its recovery mechanism corrected.

## Non-goals

- **No transient/permanent split inside *never started*.** The candidate set
  collapses: `Ingress` in-use *wants* suppression, and `Runtime` and `Clock`
  are "the machine is broken", where a restart is harmless and pointless. A
  transient class would also reintroduce the per-variant judgement the phase
  axis exists to avoid, and a misfiled variant fails silently in both
  directions (`design-log.md`, 2026-09-23).
- **`goad-emit`'s statuses are nominally owned and not governed.** The spec's
  §Owns covers this project's binaries; its §4 writes requirements for the host
  alone, and §2 says so plainly so no reader takes silence for a rule. Adding
  the second column later is an append.
- **No in-process recovery from a lost display.** The host exits and says which
  phase it ended in; bringing it back is the supervisor's job. A host that
  reconnected to a compositor by itself would be a different slice with a
  different risk.
- **No change to restart timing or backoff.** `RestartSec`, `Restart` and the
  start limiter stay as they are.
- **`report_platform`'s during-the-run path is untouched** — a platform failure
  that does *not* end the process is not an exit status.
- **No headless compositor in the gate.** New tooling would have to be asked
  for (`CLAUDE.md` §Environment), and the one thing it would buy is bought by a
  person on the running host at audit.

## Acceptance criteria

- [ ] AC-1 — A draft spec in the slice folder states the host's exit statuses
      cut on **phase**: 0 as asked, 1 stopped running, 2 never started.
      It says what a supervisor may infer from each, and states that a
      restart policy is built on the statuses rather than asserted by them.
- [ ] AC-2 — Its §Owns is the wider boundary — the exit status of this
      project's binaries — and §2 records that `goad-emit` is nominally owned
      and not yet governed.
- [ ] AC-3 — The loop's ending no longer travels as a `StartupError`.
      `StartupError`'s own claim — "every way `run` can fail to reach the
      event loop" — is true of the type again.
- [ ] AC-4 — `main`'s numeral is chosen by a pure function over an outcome
      value, and every **shape** that function can see is asserted one tier
      down — including a real `slint::PlatformError` built through
      `From<String>` classifying as *stopped running*. The `Err` shape ranges
      over every `StartupError` variant, and what holds it is the arm reading
      none of them, not a case per variant.
- [ ] AC-5 — Every existing case in `tests/binary/exit_codes.rs` still passes
      **unmodified**: 2 keeps its meaning and its consumers. The file's module
      doc is the one permitted change, since it explains the two-tier cut and
      must name the new class.
- [ ] AC-6 — A host that stops running exits 1, and its stderr line says the
      host **was** running — distinguishable from *the display could not be
      opened*, which is now only said by a host that never started.
- [ ] AC-7 — `nix/module.nix` names no known exception to its own directive,
      and its comment states the phase rule rather than a retryability claim
      that is false of `Runtime` and of `Ingress` in-use. **Its third false
      claim goes with them**: that the repair reaches *SPEC-003's failure
      vocabulary*, which is the ingress refusal-reason set and never the exit
      status — FU-1's own error, repeated there. The whole paragraph is
      removed, so none of the three may survive the edit.
- [ ] AC-8 — SPEC-003/R-4's verification cell no longer says no test target
      links the binary, and no longer records the conflation as a fact. It
      **names a case**: SPEC-003 §7's preamble forbids amending that document to
      hold a row naming no test for a clause that can be reached, and removing
      the false unreachability claim removes the escape the old sentence used
      (`review-design.md` F-26). R-3's *"same position"* analogy is narrowed in
      the same movement (`canon-delta.md` Change 3).
- [ ] AC-9 — A person observes it on the running host: the display connection
      is lost, the journal shows `status=1`, and the unit is back within
      `RestartSec`. Recorded in `audit.md` §Evidence (`docs/AGENTS.md`
      §Tiers).
- [ ] AC-10 — `docs/follow-ups.md` FU-1 is struck with what killed it, and its
      three factual corrections are carried: six exits not four, a broken
      connection not a departing compositor, and the session target not
      systemd as what recovered the fast cases.
- [ ] AC-11 — Whether a stop was asked for decides between 0 and 1, however
      the event-loop call reports its end: a requested stop is 0 even when the
      call answers an error, and an unrequested end is 1 even when the call
      answers `Ok`. The decision is a pure function over the call's result and
      whether a stop was requested, asserted one tier down for each of the
      call's results with and without a request, the error cases over **one**
      error value; `start` passes it a read of the stop signal taken after the
      call has returned.

## Governing canon

**Binding.**

- **SPEC-003 (host event ingress)** — R-4's and R-3's verification cells are
  amended and a References entry added; no requirement of it changes.
- **ADR-001 (one-way strata)** — the exit decision is stratum 3 and names Slint
  types. Nothing here reaches `src/semantics/`.
- **`docs/AGENTS.md` §Tiers** — tier 2, and a person runs the software before
  close.
- **`CLAUDE.md` §Working here** — name, never count; cite by symbol. The spec
  must not state a count of statuses that a later class would falsify.

**Checked, not applicable.**

- **SPEC-001 (host/backend protocol)** — R-40 and R-44 are about the
  *backend's* exit status. §2 puts the `goad emit` command line out of its
  scope, which is why `goad-emit` has no owner today.
- **SPEC-002 (scheduling)** — the exit decision sits outside the loop it
  governs.
- **POL-001 (the phase gate)** — no new command and no new instrument in its
  own sense: its §Verification enumerates the four ADR-001 instruments and the
  domain-vocabulary scan, and `structure.rs`'s cases are none of them. Cases are
  added to three existing targets.
- **ADR-003, ADR-004, ADR-005** — workspace shape, firing spacing and envelope
  normalization are untouched.

## Open questions

- OQ-1 — The shape of the outcome value. One enum over all three classes, or a
  `Result` whose `Ok` carries the two endings the loop can produce? The flat
  enum puts every class in one exhaustive match, which reads 1:1 against the
  statuses. **Both keep `report_startup`'s signature** — an earlier draft of
  this question claimed otherwise, and the discriminator is elsewhere: whether
  `?` survives in `start`, and whether the type admits a value no channel can
  produce.
- OQ-2 — The stderr line for *stopped running*. Its own wording, or the
  existing "the window could not be drawn" vocabulary `report_platform_line`
  already uses for a during-the-run platform failure?
- OQ-3 — Does the spec require anything about the **content** of that line, or
  only that a non-zero exit is accompanied by one on stderr?
- OQ-4 — Should a **startup** platform failure also be restartable? The race:
  the compositor cycles, the restart at +2s finds no display, and exits *never
  started*, which is suppressed. The journal says that shape self-recovers via
  the session target in 9–20s (`research.md` §Thread 3), so the recommendation
  is no — recorded in the spec as a stated consequence rather than left for a
  reader to rediscover.
- OQ-5 — Does `goad-emit` appear in the draft spec as a named boundary only, or
  also as a non-normative table of what it does today?
- OQ-6 — Does the spec bind a **supervisor** (MUST/MAY language about restart
  policy), or only define what each status means and leave policy to the
  consumer? `nix/module.nix` is a consumer living in this repository, which
  makes the binding form tempting and possibly wrong.

## Summary

<!-- Written at close: what actually landed, in three or four lines. -->

## Follow-ups

<!-- Deferred work surfaced by this slice. Each becomes a future slice or a
     line in a spec. -->

- The **research template's citation form** says code claims cite `path:line`,
  which contradicts `CLAUDE.md` §Working here — *cite by symbol, never by line
  number*. `docs/templates/` was not swept when the rule landed in 009 and
  gained its second half in 006. Not this slice's to fix; raised here so the
  sweep has a record.

- **`goad-emit`'s exit statuses are nominally owned and not governed.** The new
  spec's §Owns is the exit status of this project's binaries, and its §4 writes
  requirements for the host alone; §2 says so plainly, so that silence about the
  second binary is not read as a rule (`design-log.md`, 2026-09-23). What that
  leaves is a real gap and not a settled boundary: `goad-emit`'s `main`
  (`crates/goad-emit/src/main.rs`) decides its statuses on a different axis from
  the host's — whether the host answered the envelope, and what it answered —
  and nothing normative states it, since SPEC-001 §2 puts that command line out
  of its own scope.

  **Why a follow-up and not a repair here.** Admitting the second binary means
  writing requirements for a contract this slice has only checked by reading,
  inside a slice whose every acceptance criterion is about the host. That is a
  second design under one review, and §3's principles were written to make the
  admission an *append* precisely so it need not be done now.

  **Priced at:** an append to the new spec — a column in its §6 and requirements
  of its own in §4, with §3 unchanged — plus a §7 row per requirement against
  the cases `crates/goad-emit/tests/binary/exchange.rs` already holds
  (`help_prints_the_usage_block_on_stdout_and_exits_0`,
  `an_accepted_envelope_exits_0_and_says_nothing`,
  `a_refusal_exits_1_with_the_reason_token_on_stderr` and their siblings). No
  code change is expected: the statuses this would govern are the ones that
  binary already answers.

  **Dead when** the spec's §4 carries a requirement whose subject is
  `goad-emit`, and its §2 no longer says that binary is ungoverned.

- ~~**No binary-tier case reaches an ingress bind failure.**~~ **Struck
  2026-09-23** — no longer deferred, and landed in this slice as
  `exit_codes::an_unbindable_ingress_path_exits_2`. The deferral was sound on
  its own terms and was overtaken: SPEC-003 §7's preamble forbids amending that
  document to hold a row that names no test for a clause the row itself calls
  reachable, so the R-4 cell repair AC-8 requires could not be applied without
  the case (`review-design.md` F-26). Its companion obligation fell due in the
  same movement, as this row always said it would: SPEC-003/R-3's *"same
  position"* analogy is narrowed by `canon-delta.md` Change 3.

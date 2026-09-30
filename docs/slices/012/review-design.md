# Review — design — Slice 012

**Subject:** design — `docs/slices/012/design.md` and `docs/slices/012/canon-delta.md`
at `ec5e0e8`, against `slice-012.md` and `research.md`
**Reviewer:** fresh agent, Opus (raiser); a Codex pass as an independent second witness
**Opened:** 2026-09-30
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

**Round 1** — 2026-09-30 — the whole design and canon delta.

Written before the review. What it probes, and where the bodies are likely
buried:

1. **R-59's tables are the design's centre.** Every `requirement()` and
   `fault()` answer in design.md §5.2.3 against SPEC-001 itself: is the id the
   one broken (backend/configuration) or the one left undischarged
   (host/environment)? Is each side defensible — `Timeout` → backend in
   particular? Is "one kind names one requirement" true of every kind?
2. **The checker's blame must be honest.** Its own responds must obey R-57/R-58,
   or it blames a backend for the checker's values (I-2, R6). The
   first-option-only answer: does it narrow anything the protocol admits?
3. **The wire-compatibility invariant.** Does `goad-check` refuse, or report
   as a failure, anything SPEC-001 admits? A renderer-subset habit in a
   checker is the failure the project exists to avoid.
4. **Strata (ADR-001, ADR-003).** The lifts into `goad-semantics` — do they stay
   pure? Does `goad-check`'s dependency set respect the one-way strata?
5. **Domain vocabulary.** The examples are domain-shaped by design; the host
   is not. Does anything domain-shaped reach a host crate, `goad-check`
   included, or does the boundary scan's reach change?
6. **SPEC-004's new rows.** Are statuses 0/1/2 total and disjoint for
   `goad-check` and `goad-emit`? Is R-14's stderr line achievable for every
   path that exits non-zero?
7. **The canon delta as prose.** CLAUDE.md's rule — name, never count; cite
   by symbol, never by line number — binds canon. R-59 was just narrowed
   (design-log 2026-09-30): is the narrowing consistent everywhere it is cited?
8. **The gate's reach.** Do the kit tests reach every fence and example they
   claim to (a standing guard may not reach a new file)? Would each named
   mutation check actually red?
9. **The walk (§5.2.8–§5.2.9) is new and unrun.** It moved to an oubliette
   capsule today. Can AC-1 be met as written? Can the negative control fail?
   Are the recorded measures obtainable from what a capsule returns?
10. **Completeness against slice-012.md.** Every AC and surface has a design
    home, and nothing in the design exceeds the slice's scope.

**Round 2** — 2026-09-30 — the round 1 repairs at `c821fc5`, and their
dispositions. Probing: (a) each Response against what actually changed, then
set Outcomes; (b) the new R-59 as a whole — its closed scope (including the
R-45 clause added while repairing), one reading of the id, the declared
imprecision, and `CleanupFailure` → environment in particular; (c) the
verdict cut for `goad-check`'s statuses against every end in §5.2.2 and §5.4,
`Failure::State` included; (d) whether any repair introduced a defect or a
new count, line citation, or stale restatement elsewhere; (e) the U8
override's fixture consequence.

## Findings

**Round 1** — raiser: fresh agent, Opus, at `bf7a161`.

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | doc-wrong | verified |
| F-2 | major | doc-wrong | verified |
| F-3 | major | doc-wrong | verified |
| F-4 | major | doc-wrong | verified |
| F-5 | minor | doc-wrong | verified |
| F-6 | major | doc-wrong | verified |
| F-7 | major | doc-wrong | verified |
| F-8 | major | doc-wrong | verified |
| F-9 | major | doc-wrong | verified |
| F-10 | major | doc-wrong | verified |
| F-11 | major | doc-wrong | verified |
| F-12 | minor | doc-wrong | verified |
| F-13 | minor | doc-wrong | verified |
| F-14 | minor | doc-wrong | verified |
| F-15 | minor | doc-wrong | verified |
| F-16 | minor | doc-wrong | verified |
| F-17 | major | doc-wrong | verified |
| F-18 | major | doc-wrong | verified |
| F-19 | minor | doc-wrong | verified |
| F-20 | minor | doc-wrong | verified |
| F-21 | minor | doc-wrong | verified |
| F-22 | minor | doc-wrong | contested |
| F-23 | minor | doc-wrong | verified |
| F-24 | minor | doc-wrong | verified |
| F-25 | minor | doc-wrong | verified |
| F-26 | minor | follow-up | verified |
| F-27 | minor | doc-wrong | verified |
| F-28 | nit | doc-wrong | verified |
| F-29 | nit | doc-wrong | verified |
| F-30 | minor | doc-wrong | verified |
| F-31 | minor | doc-wrong | verified |
| F-32 | nit | doc-wrong | verified |
| F-33 | major | doc-wrong | verified |
| F-34 | major | | |
| F-35 | major | | |
| F-36 | minor | | |
| F-37 | minor | | |
| F-38 | minor | | |
| F-39 | minor | | |
| F-40 | nit | | |

### F-1 — R-59's meaning of the id is false of most rows of its own table

**Severity:** blocker
**Location:** `canon-delta.md` SPEC-001 Change 1 (R-59, "For a backend or configuration side, the requirement is the one broken. For a host or environment side … the host obligation the refusal left undischarged"); `design.md` §5.2.3 *Meaning of the id* and the table

**Expected:** Canon states a reading of the requirement id that is true of every
answer the table gives, since R-59 is what AC-7 promotes and every report line
will be read through it.

**Observed:** The two readings fit almost none of the rows.
- *Backend side, "the one broken".* SPEC-001's transport and failure rows are
  host obligations; a backend cannot break them. R-40 ("A non-zero exit status
  MUST be reported as a failure"), R-41 ("the configured timeout bounds…"),
  R-43 ("Every read … MUST be bounded"), R-44 ("Each of these MUST map to its
  own distinct error") and R-38 all say what the host does. `ExitStatus` → R-40,
  `Timeout` → R-41, `OutputTooLarge` → R-43, `Json`/`Shape`/`DuplicateKey` →
  R-44 therefore name a rule the refusal *discharged*, not one the backend
  broke. The code already says so of the transport: `process.rs`'s write path
  comments "R-37 obliges the host to write and close, nothing obliges the
  backend to read".
- *Configuration side.* `Spawn` → R-36: R-36 is about the command's shape (an
  argv, no shell, empty refused at load). A non-empty argv naming a missing
  program breaks no clause of R-36; the design concedes it ("A spawn failure
  breaks no protocol rule").
- *Host/environment side, "left undischarged".* `Io` → R-45 ("No backend
  failure may terminate the host…"): reporting `Io` and carrying on is R-45
  **kept**. `CleanupFailure` → R-48: R-48 requires the host to initiate
  termination, wait a bounded interval, and report failure to observe it as a
  distinct outcome — the refusal is that report, so R-48 is discharged.
  `StateError` → R-32 ("MUST be rejected, and the backend MUST NOT be
  contacted"): the refusal *is* R-32 discharged.

Only the view/field rows phrased as backend-facing constraints (R-10, R-13,
R-14, R-52, R-17, R-21..R-25) fit "broken" naturally. Most rows fit neither
reading; a third one — "the requirement under which the host refused" — would
fit all of them, but the delta does not state it.

**Evidence:** SPEC-001 §4 rows R-37, R-38, R-40, R-41, R-43, R-44, R-45, R-48,
R-32, R-36 as written; `canon-delta.md` R-59 text; `design.md` §5.2.3 table and
its "`Spawn` → R-36" rationale; `crates/goad-shell/src/backend/process.rs`
`body`, the comment on its `BrokenPipe` arm (R-37).

**Disposition:** doc-wrong
**Response:** Held, and wider than raised: R-3, R-12, R-18, R-23 and R-25 are host obligations too. U1 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): R-59 states **one** reading — the id is the requirement *under which the host refused*: the one stating the rule the refusal enforces, R-44 only where no more specific requirement states one. Sides are defined by where the cause lies. Rewritten in `canon-delta.md` SPEC-001 Change 1 (R-59), `design.md` §5.2.3 *Meaning of the id* and its rationale, §6 OQ-1, §10, and `slice-012.md` AC-7 and OQ-2. Rows that move: `Spawn` → R-44, `Io` and `PipeMissing` → R-45 (the rule under which the host reports a failure no requirement makes a refusal, and carries on), `EmptyAlternatives` → R-44 (U8), whose fixture's list gains R-44 as the two R-17 `Json` fixtures' do. `ExitStatus` → R-40, `Timeout` → R-41, `OutputTooLarge` → R-43, the R-44 kinds, `CleanupFailure` → R-48 and `StateError` → R-32 keep their ids, which now read truly.

**Outcome:** verified

### F-2 — `StateError` → host contradicts R-59's own definition of the host side

**Severity:** major
**Location:** `design.md` §5.2.3 (`StateError` rows; rationale "`StateError` → host"); `canon-delta.md` R-59's **host** clause

**Expected:** A side of **host** means, per R-59, "the host's own code failed an
obligation this spec places on it".

**Observed:** A stale answer is reachable in the real host without any host
defect. `wire.rs::Command`'s doc (F-13) says the markup carries the `ViewId` so
that "a delayed click" does not answer whichever interaction is outstanding
when it is dequeued: a person clicking on a view R-33 has just replaced reaches
`Host::respond` with a superseded id, and `StaleViewId` is the host correctly
refusing it. Blaming **host** for that says the host's code failed, which is
false, and a person reading the renderer's diagnostics (should a later slice
carry R-59 there) is told the host is broken. The rationale ("that caller is the
host's own renderer or the checker") argues only *not backend*; it does not
reach *host at fault*.

**Evidence:** `crates/goad/src/wire.rs` `Command::Choose` doc (F-13, "a delayed
click"); SPEC-001/R-33, R-32; `crates/goad-shell/src/error.rs` `StateError` doc
("the backend did nothing wrong, and it was not asked"); `canon-delta.md` R-59
host clause.

**Disposition:** doc-wrong
**Response:** Held. U1 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): the **host** side becomes "the cause lies on the host's side of the seam — its own code, or whoever answered through it". A delayed click is a caller on the host's side, so `StateError` → host now fits without asserting a host defect. `canon-delta.md` R-59 and `design.md` §5.2.3's `StateError` rationale are rewritten to say so.

**Outcome:** verified

### F-3 — The R-56 probe blames the backend for any failure on it, including ones it never caused

**Severity:** major
**Location:** `design.md` §5.2.2 *What is judged*, row "any failure on the R-56 probe"; §5.2.5 example; canon-delta SPEC-001 Change 3 (R-56 row)

**Expected:** The checker adds an R-56 blame only where the failure is evidence
the backend did not tolerate an unrecognised kind (brief line 2: the checker's
blame must be honest).

**Observed:** The row adds "SPEC-001/R-56 … side backend" on **any** failure of
the probe exchange. So:
- an unspawnable command (`Spawn`, side configuration) fails every exchange, and
  the probe additionally reports that a backend which never ran broke R-56;
- `Io` (environment), `PipeMissing` (host), `Timeout` under load, likewise;
- a backend that fails identically on `startup`, `requested` and `scheduled`
  (say, a shape error in a view it always returns) is charged with R-56 on top,
  though the kind played no part.
The claim is only supported when the same backend succeeded on the known kinds
and the failure is backend-side.

**Evidence:** `design.md` §5.2.2 table; `BackendError` variants and their
sides in §5.2.3; SPEC-001/R-56 tolerance clause ("never as a protocol error").

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.2's probe row adds the R-56 line only when the probe's failure has `fault()` = backend **and** at least one of the three known-kind evaluates made no failure; otherwise the probe's refusal is reported as any other and nothing is added. `canon-delta.md` SPEC-001 Change 3 ("reports a backend that fails on it") is made equally precise. §9 gains `a_backend_failing_identically_on_every_kind_is_not_charged_with_r56`; §5.2.5's example is unchanged, since its backend succeeded on `startup`.

**Outcome:** verified

### F-4 — `Timeout` → backend contradicts R-59's environment clause, and the rationale says why

**Severity:** major
**Location:** `design.md` §5.2.3 rationale "`Timeout` → backend"; `canon-delta.md` R-59 **environment** clause

**Expected:** Each side assignment follows R-59's definitions.

**Observed:** R-59 defines **environment** to include "the host observed a
condition it cannot attribute to either program". The design's rationale for
`Timeout` is precisely that the host "cannot tell that from a slow backend" —
a too-short configured timeout, or a loaded machine (the design's own R3 names
load-induced `Timeout` in the gate), is not the backend's fault. By R-59's text
the answer is environment (or configuration), not backend. The same tension
touches `ExitStatus { code: None }` (a signal — the OOM killer or a person's
`kill` is not the backend) and `Spawn` carrying `EAGAIN`/`ENOMEM`/`EMFILE`
(resource exhaustion, not the configuration). "One kind names one side" forces
these; the delta should either say so as an accepted imprecision, as SPEC-004
§5 does for its seam, or define the sides so the answers follow.

**Evidence:** `canon-delta.md` R-59; `design.md` §5.2.3 rationale bullets for
`Timeout` and `Spawn`; §8 R3; `BackendError::ExitStatus` doc ("`None` means the
child was signalled").

**Disposition:** doc-wrong
**Response:** Held. U1 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): R-59 declares its imprecision rather than redefining sides to fit: one kind names one side; where a kind's cause can lie on another side — a timeout the configuration set too short, a signal sent from outside the backend, a spawn refused for want of resources — the kind keeps its side, and the refusal carries what lets a reader see the other: the configured timeout, that the backend was signalled, the operating system's error. The **environment** clause loses "a condition it cannot attribute to either program" and becomes "the operating system failed the host, or the host could not observe what it needed to". `Timeout` stays backend; `design.md` §5.2.3's rationale now cites the declared imprecision. Written in `canon-delta.md` R-59 and `design.md` §5.2.3.

**Outcome:** verified

### F-5 — `PipeMissing` → R-37 names the stdin rule for a variant raised for any of three handles

**Severity:** minor
**Location:** `design.md` §5.2.3 `PipeMissing` row; §6 OQ-1 (a)

**Expected:** The id names the obligation concerned.

**Observed:** `process.rs` raises `PipeMissing` when any of `stdin`, `stdout`
or `stderr` is `None` after spawn (one `let … else` over the triple). R-37 is
about stdin only; a missing stdout leaves R-38/R-39 concerned, a missing stderr
R-39/R-42. OQ-1's own argument against (b) — "the variant merges … so any
single transport id is wrong for some of them" — applies to `PipeMissing` too.

**Evidence:** `crates/goad-shell/src/backend/process.rs`, the `let (Some(stdin),
Some(stdout), Some(stderr)) = … else { … PipeMissing … }` in `exchange`;
SPEC-001/R-37, R-38, R-39.

**Disposition:** doc-wrong
**Response:** Held. U1 (`design-log.md` 2026-09-30, *design review round 1: dispositions*) chose `Io` and `PipeMissing` → R-45, so no transport id is claimed for a variant raised over three handles; the finding's mechanism no longer applies. `design.md` §5.2.3's `PipeMissing` row and §6 OQ-1 are rewritten. (The draft proposed `tolerated`, conditional on the rejected alternative; under the chosen one the artefact was the defect.)

**Outcome:** verified

### F-6 — R-59's reach is ambiguous, and on its plain reading covers refusals given no method

**Severity:** major
**Location:** `canon-delta.md` R-59 ("Every refusal the host reports — each distinct error R-44 requires, … — MUST name…"); `design.md` §5.2.3 "`ConfigError`, `EnvelopeFault` and `SpanFault` get neither method"

**Expected:** R-59's subject is closed and every member of it has a
`requirement()` and `fault()`.

**Observed:**
- "Every refusal the host reports" followed by a dash list does not say whether
  the list is the set or examples of it. Read plainly, it includes
  `ConfigError::EmptyCommand` — SPEC-001/R-36 itself says an empty vector "is
  refused when the command is loaded" — and `EnvelopeFault` (SPEC-003 refusals
  the host reports to a writer). Both get neither method by design.
- The list cites "each cleanup failure R-54 requires"; R-54 requires separate
  channels, R-48 requires the report, and the table answers R-48.
- R-59 also says the requirement is "of this spec", which an `EnvelopeFault`
  (a SPEC-003 refusal) cannot satisfy.

**Evidence:** SPEC-001/R-36 ("refused when the command is loaded"); `canon-delta.md`
R-59; `design.md` §5.2.3 last rationale bullet; `crates/goad-shell/src/error.rs`
`ConfigError::EmptyCommand`.

**Disposition:** doc-wrong
**Response:** Held in every sub-claim. U1 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): R-59's subject is closed — "each distinct error R-44 requires, each other exchange failure the host reports and survives under R-45, each discarded instruction R-25 requires, each cleanup failure R-48 requires, and each answer refused under R-32, and no other" — and says outright that a refusal of the configuration file at load and a refusal of a forwarded envelope (SPEC-003) are outside it. R-54 → R-48. The R-45 clause is added because `Io` and `PipeMissing` are in no R-44 list item and U1 gives them methods. Written in `canon-delta.md` R-59; `design.md` §5.2.3's last rationale bullet cites the exclusion.

**Outcome:** verified

### F-7 — The checker cannot reach time-gated behaviour, so its acceptance can cover no view at all

**Severity:** major
**Location:** `design.md` §5.2.1 (no `--now`), §5.2.2 ("`now` are the wall clock at each step"), §5.2.9 prompt; `slice-012.md` AC-1

**Expected:** AC-1's "writes a backend the checker accepts" is evidence that the
backend's interaction works.

**Observed:** The walk prompt asks for a backend that "should stay quiet until
17:30 local time". The checker sends every request at the wall clock and has
no way to set `now`. Run at 10:00 (by the agent, or by the orchestrator "after
the walk"), a correct wrap-up answers `view: null` to every request; the view,
its three fields (`number`, `boolean`, `datetime`) and the respond path are
never exercised, and the verdict is status 0 — "accepted". The same holds for
any time-gated backend a real author writes, and for the focus check's
`scheduled` arm. Neither the report nor the status distinguishes "no view was
ever returned" from "every view was answered and accepted". The prompt's "make
the time easy to change" may lead an agent to work around it, but nothing in
the design depends on that.

**Evidence:** `design.md` §5.2.1 synopsis; §5.2.2 request plan and the `now`
sentence; §5.2.9 prompt text; SPEC-004 canon-delta `goad-check` §6 row 0 ("Nothing
about requests it did not send").

**Disposition:** doc-wrong
**Response:** Held. U3 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): no `--now`. The report ends with a line when no exchange returned a view, saying respond was not exercised (`design.md` §5.2.5), and `canon-delta.md` SPEC-004 §6 row 0 names that line. `slice-012.md` AC-1 becomes "a backend the checker accepts, with at least one view answered"; the orchestrator runs the verdict at a time the backend speaks, or through the agent's own time setting (`design.md` §5.2.9). `--now` is a follow-up if walk friction names it (`notes.md` §Open). §9 gains `a_backend_that_returns_no_view_is_accepted_and_says_respond_was_not_exercised`.

**Outcome:** verified

### F-8 — `goad-check`'s statuses are not total

**Severity:** major
**Location:** `design.md` §5.2.5 status table, §5.4; `canon-delta.md` SPEC-004 R-11..R-13

**Expected:** Brief line 6: 0/1/2 total and disjoint over every way a run ends
(SPEC-004 R-15 as drafted forbids any other).

**Observed:** Ends no row assigns:
- **Clock unreadable mid-run.** `clock::wall_clock` returns
  `Result<_, ClockError>` and the plan reads it "at each step"; status 2 is
  only for failures before the first exchange, and 0/1 require "every planned
  exchange ran".
- **Chain bound hit and nothing else reported.** It is "a checker observation,
  not a protocol refusal", so R-11 reads 0 ("the host reported nothing") while
  the exchange did not run to completion; the design does not say.
- **The report cannot be written.** §5.2.5 calls stdout "the answer to the
  invocation"; `report::line_to`'s doc says a line that *is* the answer must
  use `try_line_to`, and SPEC-004 §5 files an unwritten answer as a failure for
  the host. R-11 still reads 0.
- **`Failure::State`**, which "cannot arise unless the checker itself is
  wrong", exits 1, "refused": a checker defect reads to a caller as a backend
  judgement.

**Evidence:** `crates/goad-shell/src/clock.rs` `wall_clock` signature;
`crates/goad-shell/src/report.rs` `line_to` doc; SPEC-004 §5 state diagram
(`Question → NeverStarted: the stream refused the answer`); `design.md` §5.2.2
chain-bound bullet and `Failure::State` row.

**Disposition:** doc-wrong
**Response:** Held. U2 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): the statuses cut on whether a **verdict** was delivered — every planned exchange made and the whole report written. 0 and 1 are a verdict with nothing / something reported; 2 is no verdict, whatever the cause: before the first exchange, a clock unreadable mid-run, a report that cannot be written (through `report::try_line_to`), or `Failure::State`, the checker's own defect, whose line says so. Hitting the chain bound is a report observation and does not change the status. `design.md` §5.2.2, §5.2.5, §5.4; `canon-delta.md` SPEC-004 R-11..R-13 and §6.

**Outcome:** verified

### F-9 — SPEC-004's new §2 prose contradicts the checker's own status assignments

**Severity:** major
**Location:** `canon-delta.md` SPEC-004 Change 1 ("`goad-check` reports **a judgement** — the host's own code refused something the backend did — and a checker that never reached a backend has none to give")

**Expected:** §2's characterisation of the class matches R-12/R-13.

**Observed:** Status 1 includes `Spawn` (design §5.2.5: "This includes `Spawn`")
— the checker never reached a backend, yet exits 1, not 2. It includes a
cleanup-only report (environment) and `Failure::State` (the checker's own
defect) — neither is "something the backend did". The §2 sentence would be
false of R-12 on promotion.

**Evidence:** `canon-delta.md` SPEC-004 Change 1 and Change 3 R-12; `design.md`
§5.2.5 status 1 row and "A cleanup-only report counts as 1".

**Disposition:** doc-wrong
**Response:** Held. Follows U2 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): SPEC-004 §2's sentence becomes "`goad-check` reports **a verdict** — what the host reported running this command — and a checker that delivered none has none to give", which is true of `Spawn` (a verdict of 1) and of `Failure::State` (now 2). `canon-delta.md` SPEC-004 Change 1.

**Outcome:** verified

### F-10 — Drafted R-8 is false of `goad-emit` today, and no code change is scoped

**Severity:** major
**Location:** `canon-delta.md` SPEC-004 Change 3 R-8 ("…or the invocation was a question … and its answer was written"); `slice-012.md` §Surfaces

**Expected:** Governing `goad-emit` states what it does, or the slice changes it
(design §2 says SPEC-004 "owns them but does not govern them"; no `goad-emit`
surface is declared).

**Observed:** `goad-emit`'s `--help` and `--version` write through `to_stdout`
→ `report::line_to`, which is best-effort and discards the write error, then
return `ExitCode::SUCCESS`. So `goad-emit` exits 0 when its answer was **not**
written — R-8's "only if" is false of it. The host holds the opposite
(`exit_codes::an_answer_that_cannot_be_written_exits_2`, cited in SPEC-004 R-1's
row). Either R-8 is weakened for `goad-emit`, or `crates/goad-emit` joins the
surfaces with a test; neither is in the design.

**Evidence:** `crates/goad-emit/src/main.rs` `main` (`Invocation::Help` /
`Version` arms) and `to_stdout`; `crates/goad-shell/src/report.rs` `line_to`
("Best effort … wrong for a line that **is** the answer"); SPEC-004 §7 R-1 row.

**Disposition:** doc-wrong
**Response:** Held. U6 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): `goad-emit` is fixed rather than R-8 weakened. Its `--help`/`--version` answers go through `report::try_line_to`; an answer not written exits 2 with a stderr line, as the host's `StartupError::AnswerUnwritten` does. `design.md` §5.2.5 gains the `goad-emit` paragraph and §9 its binary test, modelled on `exit_codes::an_answer_that_cannot_be_written_exits_2`; `canon-delta.md` R-10 names the unwritten answer among its causes; `crates/goad-emit` joins `slice-012.md` §Surfaces (F-30).

**Outcome:** verified

### F-11 — `goad-emit`'s status-1 row predicts a retry and misdescribes busy refusals

**Severity:** major
**Location:** `canon-delta.md` SPEC-004 Change 4, `goad-emit` table, row 1

**Expected:** SPEC-004 §3 P-D ("A status says what happened, never what to do
about it") and §6 ("Neither number is a prediction").

**Observed:** Row 1 says "The envelope was judged and found wanting … Sending
the same bytes again will be refused again unless the host's state changed."
SPEC-003 §6.3's refusal reasons include `engaged` ("retry, or do not"),
`too_soon` (with `retry_after_ms`, "a reader MAY act on it") and `unavailable`
("wait"). For those, the envelope was not found wanting, and the same bytes
sent later are expected to succeed. The inference is false for three of the
reasons that exit 1, and it is a prediction about retrying either way.

**Evidence:** SPEC-003 §6.3 reason table; SPEC-003/R-14; SPEC-004 §3 P-D and §6
"What may not be inferred"; `crates/goad-emit/src/main.rs` `exchange` (every
`Answered::Refused` → 1).

**Disposition:** doc-wrong
**Response:** Held, with one sub-claim narrowed: for an ingress-stopped `unavailable` the same bytes will not succeed later, but the row predicts either way (P-D). `canon-delta.md` SPEC-004 Change 4, `goad-emit` row 1's inference becomes "A host answered and refused this envelope; the reason is on standard error. Nothing about whether the same bytes would be accepted later." and its *what happened* no longer says "judged and found wanting".

**Outcome:** verified

### F-12 — SPEC-004 gains a stdout requirement while its scope excludes stdout, and §2's exception list is not extended

**Severity:** minor
**Location:** `canon-delta.md` SPEC-004 Change 3 R-12 ("Its report MUST carry…") and R-14/R-15

**Expected:** A spec's requirements sit inside its §Owns and §2 scope, and
edits keep the passages that enumerate its clauses true.

**Observed:**
- SPEC-004 §2 *Out of scope* lists "the content of standard output, which is
  an answer to an invocation"; §Owns names the status and the stderr line.
  R-12 now constrains the checker's stdout report. The delta amends neither.
- SPEC-004 §2 names the clauses carrying an exception under SPEC-003 P-D —
  "R-7's …, R-4's …, R-6's …". R-14 and R-15 each carry one ("an end this
  document does not assign"); the paragraph is not updated.
- §9 References gains no SPEC-001/R-59, which R-12 now depends on.

**Evidence:** SPEC-004 §Owns, §2 *Out of scope*, §2 P-D paragraph, §9;
`canon-delta.md` SPEC-004 Changes 1–6.

**Disposition:** doc-wrong
**Response:** Held. `canon-delta.md` SPEC-004 gains a Change for §Owns and §2 *Out of scope*, carving out R-12's claim about what `goad-check`'s report carries; the P-D paragraph names R-14's exception (the unassigned end writing no line, as R-4's) and R-15's (the end it does not choose, as R-7's) by clause; §9 gains SPEC-001/R-59 and SPEC-003 §6.3.

**Outcome:** verified

### F-13 — The delta writes new counts into canon and keeps old ones in passages it touches

**Severity:** minor
**Location:** `canon-delta.md` SPEC-004 Change 1 ("**Three binaries, three cuts.**"); SPEC-001 Change 4 (R-57 row)

**Expected:** CLAUDE.md's count rule, which the delta's own header adopts:
"Every count in a passage this delta touches is either replaced or justified
as exempt."

**Observed:**
- "Three binaries, three cuts" — the same sentence ends "A new binary is an
  append to §4", so the count is of a set it expects to grow.
- Change 4's replacement for R-57's closing keeps "a *sixth* kind"; R-16's
  kinds are an open list (SPEC-001 OQ-4 contemplates another).
- The untouched middle of R-57's row keeps "All five clauses" and "six keys",
  in a row the delta edits at both ends, without an exemption stated.

**Evidence:** `canon-delta.md` header, SPEC-004 Change 1, SPEC-001 Change 4;
SPEC-001 §7 R-57 row as it stands; CLAUDE.md *Name, never count*.

**Disposition:** doc-wrong
**Response:** Held. `canon-delta.md`: "Three binaries, three cuts" → "Each binary, its own cut"; "a sixth kind" → "a new kind"; R-57's kept middle: "All five clauses" → "Every clause", "six keys" → "keys each a different JSON type from its neighbour". Change 4 now names the middle as touched.

**Outcome:** verified

### F-14 — The rewritten R-57 row is wrong about what fails to compile, and leaves a stale sentence behind

**Severity:** minor
**Location:** `canon-delta.md` SPEC-001 Change 4

**Expected:** The row states the mechanism that forces a new kind to be decided,
and every sentence it keeps stays true after the move.

**Observed:**
- "The site that must change when the protocol grows a sixth kind is this
  match's type, `Submitted`, which must gain a variant before `to_json`
  compiles". Adding a `FieldKind` variant does not stop `to_json` compiling —
  `to_json` matches `Submitted`, not `FieldKind`. What fails is
  `Submitted::as_drawn(&FieldKind)`'s match (and `goad`'s `drawn_form`).
- The kept middle says the `datetime` format "is pinned by literals at
  `draft.rs`". The design moves those tests to stratum 1 (`design.md` §5.2.4:
  "`draft.rs::tests::a_boolean_field_submits_a_json_boolean` and its siblings
  move to stratum 1"), so the literals will not be at `draft.rs`.

**Evidence:** `canon-delta.md` SPEC-001 Change 4; SPEC-001 §7 R-57 row; `design.md`
§5.2.4 `Submitted` bullets.

**Disposition:** doc-wrong
**Response:** Held. `canon-delta.md` SPEC-001 Change 4's closing sentence: adding a kind stops `Submitted::as_drawn`'s match over `FieldKind` compiling until the kind has a `Submitted` variant and an untouched value, and `view_model.rs::drawn_form`'s until it is sorted into drawn or `Undrawn::FieldForm`. The kept middle's "literals at `draft.rs`" → "literals at `canonical.rs`".

**Outcome:** verified

### F-15 — The rewritten §7 closing paragraph omits R-59's own review-held part

**Severity:** minor
**Location:** `canon-delta.md` SPEC-001 Change 5 vs Change 2

**Expected:** The paragraph that names the review-held rows names all of them.

**Observed:** Change 2's R-59 row says "The transport, cleanup and state kinds
have no fixture, and their answers are held by review of the tables." Change 5's
closing paragraph names R-9/R-19, R-18, R-20, R-30, R-49 and R-56's emission half,
and not R-59's.

**Evidence:** `canon-delta.md` SPEC-001 Changes 2 and 5.

**Disposition:** doc-wrong
**Response:** Held. `canon-delta.md` SPEC-001 Change 5's closing paragraph adds: R-59's answers for the transport, cleanup and state kinds are held by review of the tables, having no fixture.

**Outcome:** verified

### F-16 — design.md still attributes the report obligation to SPEC-001 after R-59 was narrowed

**Severity:** minor
**Location:** `design.md` §5.2.5 ("What canon fixes (canon-delta, SPEC-001) is that **every refusal line names the side at fault and the requirement**")

**Expected:** Per design-log 2026-09-30 (*R-59's reach*), R-59 fixes what a
refusal names; the checker's obligation to print both is SPEC-004 R-12.

**Observed:** §5.2.5 still cites SPEC-001 for the report-line obligation. The
narrowing is applied in `canon-delta.md` and not carried into the design.
`slice-012.md` AC-7 ("Canon states that each refusal the checker reports names
…") likewise reads as the pre-narrowing SPEC-001 claim.

**Evidence:** `design-log.md` 2026-09-30 *design.md §7–§10; R-59's reach*;
`design.md` §5.2.5; `canon-delta.md` R-59 last sentence and SPEC-004 R-12.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.5 cites SPEC-004/R-12 for the report-line obligation and SPEC-001/R-59 only for what each refusal names. `slice-012.md` AC-7 follows U1 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): "the requirement under which the host refused".

**Outcome:** verified

### F-17 — The negative control's source probe cannot find `goad-source`

**Severity:** major
**Location:** `design.md` §5.2.8 *Negative control* ("a store path holding `docs/specs/` — goad's source"); `slice-012.md` AC-1 ("no goad source in its store")

**Expected:** The probe fails whenever goad source is in the guest store.

**Observed:** The crane source derivation, `goad-source`, is filtered by
`craneLib.filterCargoSources` plus `.slint` and `assets/`: it holds every
`crates/**/*.rs` — the normalizer, the taxonomy — and no `docs/`. If it reached
the guest store (a closure reference from any exported package), the
`docs/specs/` probe finds nothing and the control passes. The spike probed
"every `*-goad-source` path"; the design narrowed the probe to `docs/specs/`.
The host-side positive control does not catch this either: on the host it can
be satisfied by the flake's whole-repo `-source` copy, not by `goad-source`.

**Evidence:** `flake.nix` `src = … lib.cleanSourceWith { … filterCargoSources …;
name = "goad-source"; }`; `research.md` §"Spike: R1 and R2" (R2 bullet);
`design.md` §5.2.8.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.8's source probe matches any store path named `*-goad-source`, **or** holding `crates/goad-semantics/src/error.rs`, **or** holding `docs/specs/`. The host-side positive control must find a hit for each pattern separately, so the whole-repo `-source` copy cannot stand in for `goad-source`. `slice-012.md` AC-1 unchanged in substance.

**Outcome:** verified

### F-18 — The capsule's own target points at goad's source, and fetching it is only friction

**Severity:** major
**Location:** `design.md` §5.2.8 (`goad-walk/flake.nix`: `inputs.goad`), *Network* bullet, D19; `slice-012.md` AC-1, AC-9

**Expected:** AC-1: the walk has "the skill and the flake's exported goad
packages, nothing else from this repository".

**Observed:** The capsule clones `goad-walk`, whose `flake.nix` and
`flake.lock` name `goad` as an input — the full repository, canon included.
The proxy admits "a short package-manager allowlist"; if that includes nix's
fetchers or the forge (the design does not say), `nix flake archive` or a plain
fetch of the locked URL gives the agent SPEC-001. The design classifies any such
fetch as friction (AC-9), but a walk whose agent read the spec has not met
AC-1, and nothing says the transcript read fails AC-1 on it. The input's form
is also unspecified; memory `path-flake-ref-breaks-on-demo-socket` says a
`path:` reference to this repository breaks on the demo socket.

**Evidence:** `design.md` §5.2.8 tree and bullets; §5.2.9 *The transcript read*
(tags only); `slice-012.md` AC-1, AC-9; `docs/memory/path-flake-ref-breaks-on-demo-socket.md`.

**Disposition:** doc-wrong
**Response:** Held as a gap. U4 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): `goad-walk`'s `goad` input is a host-local `git+file:///…/goad` URL (the form memory `path-flake-ref-breaks-on-demo-socket` recommends), so the guest's lock names a path that does not exist there and the fetch route is closed whatever the proxy admits. `design.md` §5.2.8 states the input's form; `slice-012.md` AC-1 and `design.md` §5.2.9 add: a walk whose agent read goad's source fails AC-1 and is re-run.

**Outcome:** verified

### F-19 — How a person runs, and the orchestrator checks, the walk's backend is unspecified

**Severity:** minor
**Location:** `design.md` §5.2.9 *What is recorded* ("the `goad-check` verdict … run by the orchestrator after the walk"; "whether a person ran goad against it"); AC-1

**Expected:** AC-1's second half ("a person has run the host against it and
seen the behaviour") has a stated route.

**Observed:** The agent writes its configuration inside the capsule, so its
`command` names capsule paths (`/work/goad-walk/…`) and a capsule `ruby`. The
host devshell has no `ruby` (design §5.2.8: "`ruby` goes into the walk's tool
set only"), and no window opens in the capsule. Whether the checker is run in
the guest or on the collected tree, and how the person runs the host against a
Ruby backend on the host, is not said. Running it with a rewritten config would
no longer be the agent's config.

**Evidence:** `design.md` §5.2.8 devshell and tool-set bullets, *A walk*;
§5.2.9; `slice-012.md` AC-1.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.9: the verdict is `goad-check --config <the agent's config>` run by the orchestrator in the guest over ssh, unaltered, before collection. The person-run is on the collected tree on the host, with `ruby` from `nix shell nixpkgs#ruby`, and the config's command path rewritten from `/work/goad-walk/…`; the diff is recorded in `walks.md`.

**Outcome:** verified

### F-20 — Two recorded measures have a witness that cannot see what they count

**Severity:** minor
**Location:** `design.md` §5.2.9 *What is recorded*

**Expected:** Each measure is obtainable from what the capsule returns (brief
line 9).

**Observed:**
- *Fetch attempts.* "An attempt counts whether or not the proxy let it
  through", but the second witness is "the capsule proxy's log of **refused**
  requests"; an allowed fetch beyond the model API has only the transcript.
- *Tokens.* The design normalises Codex's `cached_input_tokens` as a subset of
  `input_tokens`; research R-e also lists `cache_write_input_tokens`, whose
  relation to `input_tokens` is not measured, so the Codex "uncached input" and
  "cache write" columns may double count.

**Evidence:** `design.md` §5.2.9; `research.md` R-e Codex bullet.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.9: fetch attempts use the proxy's full request log if oubliette keeps one; otherwise allowed fetches have the transcript as their only witness, and the row says so. Tokens: Codex's raw fields are recorded; the derived uncached column waits until the first walk shows whether `cache_write_input_tokens` is a subset of `input_tokens`.

**Outcome:** verified

### F-21 — The respond-request fence check needs an R-57 reader the design does not specify

**Severity:** minor
**Location:** `design.md` §5.2.6 tagged-fence table, `json goad:request` (respond) row; §5.5 I-2

**Expected:** Principle 2 and I-2: no second kind→JSON-type mapping.

**Observed:** The row passes when "each value must equal `Submitted::to_json`
of a value of its field's kind". To find that value the test must read a JSON
value back into `Submitted` per `FieldKind` — for `number` a `Finite`, for
`datetime` an RFC 3339 instant and offset, for `choice` an alternative — which
is the inverse of R-57 written a second time, in `goad-check`'s tests. No
reader exists (§2: "No request reader exists"). The evaluate row similarly
builds a `Request` from the block's own `now` and `event`, so it checks the
envelope keys and nothing about `source`/`kind`; that is the whole of what it
can check, and the design should say so.

**Evidence:** `design.md` §5.2.6 table; §2 fourth bullet; §5.5 I-2;
`crates/goad-semantics/src/protocol/canonical.rs` `Event`/`UserResponse`
(`Serialize` only).

**Disposition:** doc-wrong
**Response:** Held. U5 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): the respond row uses the lifted code as a type oracle — each value's JSON type equals that of `Submitted::as_drawn(kind).to_json()`, and a `choice` value is one of the field's alternative ids; over exactly the option's fields. It states what it does not hold: the RFC 3339 spelling of a `datetime`. No reader, no second mapping. The evaluate row says it holds framing only. `design.md` §5.2.6 table and §5.5 I-2.

**Outcome:** verified

### F-22 — The reference coverage test is vacuous for R-3 and misses new variants

**Severity:** minor
**Location:** `design.md` §5.2.6 *Coverage test*; §9 `every_requirement_a_refusal_can_name_is_explained_in_the_reference`

**Expected:** The test fails when an id a variant answers is absent from the
reference, including for a variant added later.

**Observed:**
- It "greps the reference for each id" in the form `SPEC-001/R-13`.
  `UnsupportedProtocolVersion` answers R-3; `SPEC-001/R-3` is a prefix of
  `SPEC-001/R-32`, `R-36`, `R-37`, all of which the reference will cite, so R-3
  is "found" whether or not it is explained. The design does not require a word
  boundary.
- It "builds one instance per variant, the `every_protocol_error` pattern …
  extended to stratum 2". That helper is a private `#[cfg(test)]` function in
  `goad-semantics`' `error.rs`, unreachable from `goad-check`'s tests, and it is
  a `vec![…]`, not a match: a variant added later gets its `requirement()` arm
  (the compiler forces it) and no instance here, so its id is never checked.

**Evidence:** `crates/goad-semantics/src/error.rs` `mod tests::every_protocol_error`;
`design.md` §5.2.3 (`UnsupportedProtocolVersion` → R-3), §5.2.6.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.6 and §9: the coverage test matches each id followed by a non-digit (or the end), so `SPEC-001/R-3` is not found inside `R-32`. `goad-check`'s tests build their own instances, and each builder carries an adjacent exhaustive `match` with no `_` arm, so a new variant does not compile until it has an instance there.

**Outcome:** contested — the repair asserts a compile gate the stated mechanism does not give. An exhaustive `match` with no `_` arm beside a builder forces a new variant to gain an **arm**, not an **instance** in the builder: the builder is a `vec![…]` (or equivalent list) and the match runs over whatever instances it holds, so a variant with an arm and no instance compiles and is never checked. SPEC-003 §7 R-14 row states exactly this limit of the same pattern ("whether the assertion then also fails depends on that author adding a witness beside the arm the compiler made them write"). `InapplicableKey` also needs **two** instances (`fields` and another key) to reach both ids, which one arm per variant cannot force. Either the builder must be generated from the match (each arm returning its own instance or instances, the builder calling it for one representative per arm), or `design.md` §5.2.6 must state the limit as SPEC-003 does. Its companion half — the non-digit boundary for R-3 — holds.

### F-23 — The corpus witness and the first mutation check are weaker than stated

**Severity:** minor
**Location:** `design.md` §5.2.3 *The witness*; §9 *Mutation checks* ("flip one `requirement()` arm → the witness fails")

**Expected:** The named mutation reds the named test.

**Observed:**
- Every schedule-corpus error fixture lists R-25; every `Shape` fixture lists
  R-44. Flipping `MissingOffset`, `TimeOfDay` or `CalendarUnit` to R-25 passes
  the discard witness; so does flipping `DuplicateOptionId` R-14 → R-52 or
  `EmptyAlternatives` R-52 → R-53 (both lists hold both). The witness catches
  a flip only to an id outside the fixture's list.
- A flip in any stratum-2 arm (`Spawn`, `Timeout`, …) has no fixture at all.
- The mutation check does not name which arm is flipped, so it can be
  discharged with one that happens to red.
- The green step edits two fixtures' lists to agree with the code; the witness
  is then independent only where the lists were not edited to fit.

**Evidence:** `tests/fixtures/schedule/*.json` `requirement` arrays (each error
case includes R-25); `tests/fixtures/protocol/R-14-duplicate-option-ids.json`
[R-14, R-52]; `R-52-a-choice-field-with-no-alternatives.json` [R-52, R-53];
`design.md` §9.

**Disposition:** doc-wrong
**Response:** Held in every sub-claim. U7 (`design-log.md` 2026-09-30, *design review round 1: dispositions*): the membership witness is kept, and its reach is stated where it is claimed. `canon-delta.md` SPEC-001 Change 2: the corpus "catches an answer outside the fixture's own list". `design.md` §5.2.3 *The witness* says the same and names what it misses (flips inside a list; stratum-2 arms). §9's mutation is named: flip `NestedHints` R-18 → R-3, outside [R-18, R-47]. Rationale for not tightening: an exact `names` key on each fixture would be written with the code it witnesses and lose the independence that is the witness's point; the lists predate this slice.

**Outcome:** verified

### F-24 — The fence extractor's reach is narrower than I-3 claims

**Severity:** minor
**Location:** `design.md` §5.2.6 *tagged-fence convention*; §5.5 I-3 ("every json/toml fence under `kit/` is checked, or the gate fails")

**Expected:** The checked set cannot shrink silently (D14).

**Observed:** The extractor keys on info strings `json` and `toml`. A wire
example written as ` ```jsonc `, ` ```JSON `, ` ```json5 `, ` ```js `, or as a
CommonMark **indented** code block, is not seen and not refused. An author (or
a walk-driven kit fix) can move an example out of the checked set without the
gate noticing.

**Evidence:** `design.md` §5.2.6 bullets ("backtick or tilde fences, info string
split on whitespace"); §5.5 I-3; §7 D14.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.6: the extractor refuses any fence whose first info word, lowercased, begins `json` or `toml` (`jsonc`, `json5`, `JSON`) unless it is exactly `json`/`toml` with a `goad:` role. Indented code blocks are not checked, and I-3 says so.

**Outcome:** verified

### F-25 — `Alternatives::first` already exists; the design proposes building it again

**Severity:** minor
**Location:** `design.md` §5.2.4 *`Submitted::as_drawn`* bullet; §6 OQ-2 (a) ("It also needs a total `Alternatives::first`"); §9 `alternatives_first_is_the_first_declared`

**Expected:** No parallel implementation (CLAUDE.md); design premises verified
against the tree.

**Observed:** `canonical.rs` has `pub fn first(&self) -> &Alternative`, total,
with its invariant argued beside it (slice 009, `d92e6ec`), and
`view_model.rs` already calls `alternatives.first().id()` to fill
`DrawnKind::Choice.first`. The design proposes a new total `first` returning
`&AlternativeId`, a changed storage representation ("store the first element
apart from the rest"), and a new test.

**Evidence:** `crates/goad-semantics/src/protocol/canonical.rs`
`Alternatives::first`; `crates/goad/src/view_model.rs` (`first:
alternatives.first().id().clone()`); `git log -S'pub fn first(&self) -> &Alternative'`.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.4: `Submitted::as_drawn` uses `alternatives.first().id().clone()`. The new `first`, the storage change and `alternatives_first_is_the_first_declared` are dropped; OQ-2 (a)'s "needs a total `Alternatives::first`" is struck, and `DrawnKind::Choice.first`'s removal stays a refactor-step candidate (its doc is already stale).

**Outcome:** verified

### F-26 — Nothing holds that `goad-check` links no renderer

**Severity:** minor
**Location:** `canon-delta.md` ADR-003 Change 1 bullet ("it links no renderer"); `design.md` D1, §5.5

**Expected:** A property canon asserts has an instrument, or its row says it is
review.

**Observed:** `allowlist.rs`'s doc states "a stratum-3 manifest is billed by
nothing here"; `goad-emit`'s freedom from Slint is "held by the crate edge and
by review", and its `Cargo.toml` argues it in a comment. The design lists no
invariant, test or manifest comment for `goad-check`; a later `goad` dependency
(to reach, say, a renderer helper) would pass the gate.

**Evidence:** `crates/goad-boundary/tests/checks/allowlist.rs` module doc;
`crates/goad-emit/Cargo.toml` comment; `design.md` §5.5 invariants.

**Disposition:** follow-up
**Response:** Held. The instrument is FU-7's (`docs/follow-ups.md`, stratum 3 carries no manifest allowlist row); its citation is extended at close, recorded in `notes.md` §Open. Now: `design.md` §5.5 gains I-6, "`goad-check` links no renderer: held by its manifest and review; FU-7", and `crates/goad-check/Cargo.toml` carries a comment arguing it as `goad-emit`'s does (`slice-012.md` §Surfaces).

**Outcome:** verified

### F-27 — How an example's config names its backend, and its event names its file, is unstated

**Severity:** minor
**Location:** `design.md` §5.2.6 *The examples* and table; I-4; §9 `each_shipped_example_is_accepted_by_the_checker`, `downloads_triage_moves_the_file_it_was_asked_about`

**Expected:** Each example runs from its `config.toml` "in under a minute", and
the gate runs it the same way.

**Observed:** The host passes `command` verbatim and spawns with its own
working directory; `Config` resolves nothing relative to the file
(`exercisers/demo.toml` is cwd-relative, research R-f). A kit example in a
read-only store path, or copied to `~/.config/goad/`, cannot name its backend
by a relative path that works for a desktop-launched host. The design does not
say what the example configs contain, nor what cwd the gate test uses. The
triage event file is static JSON, so the file it names cannot follow the gate's
temporary `XDG_DOWNLOAD_DIR`.

**Evidence:** `crates/goad-shell/src/config.rs` (no path resolution for
`command`); `research.md` R-f (`examples/demo.toml`, "cwd-relative"); `design.md`
§5.2.6.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.6: each example's `command` is relative to the example directory; the gate test copies the example to a temporary directory and runs there (the `round_trip.rs` rebasing precedent). The README's minute-long route is `goad-check --config config.toml` from the example directory; for goad, copy the directory and make `command` absolute. Triage events carry a file **name**, resolved by the backend under `$XDG_DOWNLOAD_DIR`; `watch.sh` emits the basename.

**Outcome:** verified

### F-28 — Risk ids R7 and R8 are each used twice

**Severity:** nit
**Location:** `design.md` §8

**Expected:** Doc-local ids are unique and immutable (the file's own header).

**Observed:** §8 has R7 (registering `goad-walk`) and R7 (network reads), R8
(proxy refuses) and R8 (stale counts).

**Evidence:** `design.md` §8 table.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §8: the second R7 and R8 become R9 and R10.

**Outcome:** verified

### F-29 — Stale references survive the move to capsules and the rename

**Severity:** nit
**Location:** `design.md` §2 (jail-library bullet), §6 OQ-8 ("consumer jail"), §8 R1 mitigation ("prototype the jail"), §5.2.1 (`examples/demo.toml`); `slice-012.md` OQ-3 and OQ-6 answers ("consumer jails", "jails load the store path"); §5.2.7 rename table

**Expected:** The artefact states current truth.

**Observed:** Each names the superseded bwrap jails, or the pre-rename path.
The rename table omits `docs/memory/cite-requirements-not-finding-ids.md`
(which names `examples/`) and the `justfile` `typecheck` comment ("The example
backend is documentation agents edit").

**Evidence:** the cited sections; `git grep -n 'examples/' -- ':!docs/slices'`.

**Disposition:** doc-wrong
**Response:** Held, plus one site: `exercisers/typescript/README.md`'s own `command` line, which `round_trip.rs::the_readme_s_own_config_loads_and_runs_the_example` parses. `design.md` §5.2.7's table gains it, `docs/memory/cite-requirements-not-finding-ids.md` and the `justfile` `typecheck` comment. Jail wording in `design.md` §2, OQ-8, §8 R1 and `slice-012.md` OQ-3/OQ-6 answers becomes capsule wording; §5.2.1's `examples/demo.toml` becomes `exercisers/demo.toml`.

**Outcome:** verified

### F-30 — Surfaces and scope in `slice-012.md` do not match the design

**Severity:** minor
**Location:** `slice-012.md` §Scope, §Surfaces (Canon bullet)

**Expected:** Brief line 10: every surface the design touches is declared, and
the slice says what the design decided.

**Observed:**
- The Canon bullet names SPEC-001, SPEC-004, POL-001; the delta also amends
  ADR-003.
- `crates/goad-boundary` (the allowlist doc, design §5.2.7) is not declared.
- §Scope still says the skill carries "scripts"; design §6 OQ-4 settled on
  none.
- If F-10 is fixed in code, `crates/goad-emit` is an undeclared surface.

**Evidence:** `slice-012.md` §Scope and §Surfaces; `design.md` §5.2.7, §6 OQ-4,
§10.

**Disposition:** doc-wrong
**Response:** Held. `slice-012.md` §Surfaces: Canon adds ADR-003; `crates/goad-boundary` (the allowlist doc), `crates/goad-emit` (U6) and `crates/goad-check/Cargo.toml`'s comment (F-26) are declared. §Scope drops "scripts" (OQ-4).

**Outcome:** verified

### F-31 — The new SPEC-004 §7 rows do not meet §7's own form

**Severity:** minor
**Location:** `canon-delta.md` SPEC-004 Change 5

**Expected:** SPEC-004 §7: where no cooperating test reaches a clause, the row
"says what review holds **and what it does not**".

**Observed:** The R-8..R-10 row defers its content ("What no case reaches is
named in the row as R-3's row names it"); R-14's row ("each binary's cases that
read standard error") does not say which hold the whole line, a prefix, or
*last line*, as R-4's row does case by case; R-15's "the compiler and review"
states no limit. R-13's clock and runtime causes are headless-unreachable, and
no row says so.

**Evidence:** SPEC-004 §7 intro and R-3/R-4 rows; `canon-delta.md` SPEC-004
Change 5.

**Disposition:** doc-wrong
**Response:** Held. `canon-delta.md` SPEC-004 Change 5: the R-8..R-10 row is written out, naming what no case reaches; R-14's row states per case whether it holds the whole line, a prefix or the last line; R-15's row says the compiler holds literal-only `ExitCode`s and no test holds a signal or panic end; R-13's row says the clock and runtime causes are headless-unreachable.

**Outcome:** verified

### F-32 — The reference cites a spec it does not ship

**Severity:** nit
**Location:** `design.md` §5.2.6 *The reference's structure* ("Each rule it states cites the SPEC-001/002/003 id it restates")

**Expected:** The kit stands alone (AC-1); a fetch beyond the model API is
friction (AC-9).

**Observed:** Every rule carries `SPEC-001/R-N`, and every report line prints
one, but no spec is in the kit. An agent following a citation to its source
has nowhere local to go, which invites exactly the fetch AC-9 counts. The
design may intend this; it does not say the reference tells the reader the ids
are resolved in the reference itself.

**Evidence:** `design.md` §5.2.6; I-5; `slice-012.md` AC-9.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.6: the reference states that the specs are not shipped, and that every id a report prints is explained in the reference, each at an anchor of its own.

**Outcome:** verified

### F-33 — The argv form does not reuse the host's empty-command refusal

*Raised by the Codex second witness (an independent read-only pass, not shown
this ledger), verified by the orchestrator against the source.*

**Severity:** major
**Location:** `design.md` §5.2.1 *Argv form*

**Expected:** An empty program is refused before transport, by the same rule
the host applies (SPEC-001/R-36), as §5.2.1 claims.

**Observed:** §5.2.1 builds the argv form's command with
`config::Command::new` and says an empty argv "is refused by the same rule".
`Command::new` accepts any program, the empty string included. The refusal is
`Command::from_argv`, which is private to `config` and reached only through
`Config::parse`.

**Evidence:** `crates/goad-shell/src/config.rs`: `Command::new`,
`Command::from_argv`, `Config::parse`.

**Disposition:** doc-wrong
**Response:** Held. `design.md` §5.2.1: the argv form is built with `config::Command::from_argv`, made public; an empty argv or program is a usage error, status 2. `crates/goad-shell` is already a declared surface.

**Outcome:** verified

### Second witness — corroboration

Codex raised five findings without seeing this ledger. Four match findings
above, raised independently: its clock-failure status gap is F-8; its
"names rules that were not broken" (Spawn → R-36, Timeout → R-41,
EmptyAlternatives → R-52) is F-1 and F-4, and adds `EmptyAlternatives` → R-52 as
an instance; its R-56 over-blame is F-3; its source-probe gap is F-17. The
fifth is F-33. It found sound: reuse of `Host<ProcessBackend>`, the strata
placement, `--event` through `envelope::normalize`, the fence scanner's
rejection of untagged fences, and the vocabulary boundary.

### Round 1 — what holds

Checked against source and found sound, so round 2 can narrow:

- **Taxonomy inventory.** Every variant in §5.2.3's table exists as named in
  `goad_semantics::error` and `goad_shell::error`, and none is missing.
  `ProtocolError::from(serde_json::Error)` sends `Syntax`/`Eof`/`Io` to `Json`
  (so R-38's empty stdout and trailing content arrive as `Json`), and `Data` to
  `Shape`.
- **Fixture tabulation.** The protocol and protocol-text error fixtures'
  `requirement` lists are as §5.2.3 states; the two R-17 `Json` fixtures are the
  only ones whose list lacks the design's id. `InapplicableKey`'s `fields` case
  is the only R-53 fixture. The schedule corpus's lists support §5.2.3's
  schedule ids (subject to F-23's weakness).
- **Headless reuse (brief line 2, AC-3).** `Host::respond` verifies the `view_id`
  before touching the transport; `Outcome` carries the channels §5.2.2 lists;
  `UserResponse` and `Event` have public fields; option and field ids can be
  cloned off a presented view, so the checker needs no `pub(super)` widening.
  `envelope::normalize(&[u8]) -> Result<Event, EnvelopeFault>` is the one door
  for `--event` files.
- **First option only (brief line 3).** Answering one option with values for
  exactly its fields, built as the host builds them, refuses nothing SPEC-001
  admits; R-58's "fields the host drew" is met by a checker that behaves as a
  host drawing every kind. No wire narrowing found in the checker's request set
  or in its handling of discards, hints or `view: null`. The `source: "host"`
  event-file refusal matches SPEC-003/R-13.
- **Strata (brief line 4).** The lifts (`Stimulus`, `Submitted`, `Finite`,
  `as_drawn`) need only `jiff`, `serde_json` and canonical types, all on
  stratum 1's allowlist; `Stimulus` in `goad`'s `wire.rs` carries nothing
  renderer-specific. `goad-check` names strata 1 and 2 only. Clippy's
  `pub_use` denial is correctly given as the reason for no re-export.
- **Vocabulary (brief line 5).** `goad-boundary`'s `members` enumerates
  `workspace.members` from the root manifest, so `goad-check`'s `src/` is
  scanned the moment it joins; `tests/` is excluded, and `kit/` is outside every
  member. Nothing domain-shaped is proposed for a host crate's `src/`.
- **`goad-emit` statuses 1 and 2** as drafted (R-9, R-10) match `main.rs`
  (every `Answered::Refused` → 1; `SendFault`, `StartupFault` and usage errors →
  2, each with a stderr line). Only R-8's "answer written" clause (F-10) and the
  §6 inference (F-11) diverge.
- **POL-001.** One `deno check` line with two paths keeps the block's shape;
  the "six commands" exemption is sound because `just -n check` holds it.
- **Rename reach.** Every `examples/` site outside `docs/slices/` is in the
  rename table except the two F-29 names.

Not reached in depth this round: whether `claude plugin validate` and the
Codex `interface` block accept the manifests as drafted (§5.2.6), and the
example behaviours' own protocol correctness (they do not exist yet).

**Round 2** — raiser: the round 1 raiser, kept, at `ea1108d`. Outcomes for
F-1..F-33 are set above; F-22 is contested. New findings follow.

### F-34 — `CleanupFailure` → environment rests on a clause that is not "where the cause lies"

**Severity:** major
**Location:** `canon-delta.md` SPEC-001 Change 1 (R-59, **environment**: "or the host could not observe what it needed to"); `design.md` §5.2.3 rationale "`CleanupFailure` → R-48, environment"; §6 OQ-5

**Expected:** U1: "Sides are defined by where the cause lies"; the declared
imprecision applies where a kind's cause *can* lie elsewhere, and "the refusal
carries what lets a reader see the other".

**Observed:**
- The design says the cause of a cleanup failure is "often the backend's — a
  child left holding a pipe", and OQ-5 still argues status 1 because "It
  usually means the backend left a child holding stderr, which the author can
  fix". So the usual location of the cause is the backend, and environment is
  chosen for another reason: R-54 forbids naming an unobserved process state.
  The new clause "the host could not observe what it needed to" is a
  description of the host's knowledge, not of where a cause lies — it makes
  **environment** mean "unknown", against the U1 definition in the same
  sentence.
- Read by its letter, that clause also fits `Timeout` (the host could not
  observe a response in time), so the four definitions no longer pick one
  side per kind; the table does.
- The imprecision's condition is not met here: a `CleanupFailure::TimedOut`
  carries only `after`, the limit it waited. Nothing in it lets an author see
  that their backend's child held the pipe; a kit reader told "environment"
  has no reason to look at their own process tree.
- R-54 forbids asserting a *process state*; naming the backend as the side a
  cause usually lies on asserts none. The design treats the two as the same
  claim without arguing it.

**Evidence:** `canon-delta.md` R-59 side clauses and imprecision sentence;
`design.md` §5.2.3 `CleanupFailure` and `Timeout` rationale bullets, §6 OQ-5;
`crates/goad-shell/src/error.rs` `CleanupFailure::TimedOut` (carries `after`
only) and its doc on the grandchild case; SPEC-001/R-54.

**Disposition:**
**Response:**

**Outcome:**

### F-35 — "R-44 only where no more specific requirement states one" is an instance rule applied to kinds, and is false of `Shape` and `Json`

**Severity:** major
**Location:** `canon-delta.md` R-59 ("That is the requirement stating the rule the refusal enforces. R-44 is named only where no more specific requirement states one" … "The id and the side are properties of the refusal's kind and not of the instance"); `design.md` §5.2.3 *Meaning of the id*, `Json` and `Shape` rows

**Expected:** R-59's reading of the id is true of every row (the class F-1 was
about).

**Observed:** R-59 fixes the id per kind, then chooses it by whether a more
specific requirement states the rule — which varies by instance within one
kind:
- `Shape` → R-44, yet the corpus's own `Shape` fixtures refuse under more
  specific rules: `R-15-a-misspelled-required-key` (R-15: "Each field MUST
  carry an id, a kind and a label"), `R-13-an-option-written-as-an-array`,
  `R-11-an-envelope-written-as-an-array`, `R-3-protocol-declared-as-a-string`.
  Research R-a already found the specific rule "not recoverable" from `Shape`
  — which is a reason the kind names R-44, not a case of "no more specific
  requirement states one".
- `Json` → R-44, yet its only fixtures are R-17's non-finite literals (R-17:
  "bounds MUST each be finite"), and R-38 states the rule for trailing content
  and empty stdout, which also arrive as `Json`.
Read literally, R-59 makes these rows false; the only sanctioned exception to
one id per kind is R-53's. The sentence needs to say that R-44 is named where
the **kind** cannot tell which more specific rule an instance broke, or
equivalent.

**Evidence:** `tests/fixtures/protocol/*` `Shape` fixtures' `requirement`
lists (each pairs R-44 with R-3, R-11, R-13, R-15, R-19 or R-52);
`tests/fixtures/protocol-text/R-17-*`; SPEC-001/R-15, R-17, R-38;
`research.md` R-a *Shape is coarse by construction*.

**Disposition:**
**Response:**

**Outcome:**

### F-36 — With a relative command, the checker does not run what the host will run

**Severity:** minor
**Location:** `design.md` §5.2.1 *Config form* ("The checker runs exactly the command and timeout the host will run"); §5.2.6 *How a config names its backend*; §5.2.9 verdict and person-run

**Expected:** A config the checker accepts is one the host can spawn.

**Observed:** The repair makes every example's `command` relative to its own
directory, and the README route is `goad-check --config config.toml` from
there. Both the checker and the host spawn with their own working directory,
and `Config` resolves nothing relative to the file; a desktop-launched host's
directory is not the example's. So the checker accepts a config the host will
fail to spawn, with no line saying the result depends on the directory it was
run from. The walk meets the same case: an agent that writes a relative
`command` and checks from `/work/goad-walk` passes. The report does not print
the working directory, and nothing warns on a relative program or argument.
The cited precedent differs too: `round_trip.rs`'s `rooted_at_the_workspace`
rewrites `./`-prefixed arguments against the workspace root; it does not copy
a directory or set a working directory.

**Evidence:** `crates/goad-shell/src/config.rs` (`Command`, no path
resolution); `crates/goad-shell/tests/integration/round_trip.rs`
`rooted_at_the_workspace`; `design.md` §5.2.1, §5.2.6.

**Disposition:**
**Response:**

**Outcome:**

### F-37 — A re-walk sees kit fixes only after `goad-walk`'s lock is bumped, and the design does not say so

**Severity:** minor
**Location:** `design.md` §5.2.8 (`inputs.goad` host-local `git+file:`; "provisioned at `goad-walk`'s `main`"); §5.2.9 *Re-walk rule*

**Expected:** "After the kit fixes land, each agent gets one fresh re-walk" —
against the fixed kit.

**Observed:** The capsule's tool set comes from `goad-walk`'s `flake.lock`,
which pins `goad` at a revision. Kit fixes landing on goad's `main` reach a
re-walk only if `goad-walk`'s lock is updated and committed to its `main`
first. Nothing in the walk sequence says so, or records the pinned goad
revision per walk in `walks.md`; a re-walk run on the old lock would measure
the unfixed kit and still read as "the re-walk".

**Evidence:** `design.md` §5.2.8 tree and *A walk* bullet; §5.2.9 *What is
recorded* and *Re-walk rule*.

**Disposition:**
**Response:**

**Outcome:**

### F-38 — After U8, `EmptyAlternatives` names a requirement that states no rule it enforces

**Severity:** minor
**Location:** `design.md` §5.2.3 `EmptyAlternatives` row and rationale; §6 OQ-6; `canon-delta.md` R-59

**Expected:** R-59: the id is "the requirement stating the rule the refusal
enforces".

**Observed:** The design says no requirement states that a `choice` field's
alternatives are non-empty, and names R-44 on that ground. But R-44 does not
state that rule either: its shape item is "a missing required key, a value of
the wrong type, an array where an object is required", and an empty array is
none of them. So the row is refused under a rule no requirement states, and
R-59's sentence is false of it until OQ-6's R-16 clause lands. The kit
reference must then explain, under R-44, a rule canon does not contain. The
fixture consequence is otherwise clean: `R-52-a-choice-field-with-no-alternatives`
gains R-44, is still cited in SPEC-001 §7's R-52 row, and the design declares
it one of the lists edited to agree with the code.

**Evidence:** SPEC-001/R-44, R-16, R-52; `design.md` §5.2.3, §6 OQ-6;
`notes.md` §Open (R-16 entry).

**Disposition:**
**Response:**

**Outcome:**

### F-39 — `PipeMissing` → R-45, though its own doc says it is not a backend failure

**Severity:** minor
**Location:** `canon-delta.md` R-59 ("each other exchange failure the host reports and survives under R-45"); `design.md` §5.2.3 `PipeMissing` row, §6 OQ-1

**Expected:** R-45 is the rule under which the failure is reported.

**Observed:** R-45 is "No **backend** failure may terminate the host". The
variant's doc says a missing pipe is "a value the host itself asked for, so
its absence is not backend-derived (F-35)", and the design gives it side
**host**. It is therefore outside R-45's subject, and not in R-59's scope as
drafted ("survives under R-45"). `Io` (side environment) is closer, but its
write/wait/read failures are not the backend's either.

**Evidence:** SPEC-001/R-45; `crates/goad-shell/src/error.rs`
`BackendError::PipeMissing` doc; `canon-delta.md` R-59 scope clause.

**Disposition:**
**Response:**

**Outcome:**

### F-40 — `Command`'s public fields contradict the same doc the opportunity note blames on `new`

**Severity:** nit
**Location:** `design.md` §5.2.1 *Opportunity, for the plan*

**Expected:** The note names every way an empty command becomes representable.

**Observed:** `config::Command` has `pub program` and `pub arguments`, so a
struct literal builds an empty command without `new`. Validating `new` alone
does not make the doc true.

**Evidence:** `crates/goad-shell/src/config.rs` `pub struct Command`.

**Disposition:**
**Response:**

**Outcome:**

### Round 2 — what holds

Checked against the artefacts at `ea1108d` and the source, and found sound:

- **R-59, apart from F-34, F-35, F-38 and F-39.** The single reading ("under
  which the host refused") is true of `ExitStatus` → R-40, `Timeout` → R-41,
  `OutputTooLarge` → R-43, `Spawn` → R-44, `StateError` → R-32, and the
  view/field kinds with their own rule. The scope is closed and excludes
  config-load and envelope refusals in terms. R-48 replaces R-54. `StateError`
  → host fits the new host clause.
- **The verdict cut (brief c).** Every end in §5.2.2 and §5.4 is assigned:
  pre-exchange faults, a mid-run clock, an unwritten report or answer, and
  `Failure::State` are 2; a delivered verdict is 0 or 1; the chain bound
  changes nothing; signals and panics are R-15's unassigned end. §5.4, §5.2.5,
  SPEC-004 R-11..R-13, §2's "verdict" sentence and §6's rows agree with one
  another. `Failure::State` stops the run and nothing else does, as §5.4
  says.
- **SPEC-004's repairs.** §Owns, *Out of scope*, the P-D paragraph, §3's
  opening, P-C, R-8..R-10's row (its test names match
  `crates/goad-emit/tests/binary/exchange.rs`, plus the new
  `an_answer_that_cannot_be_written_exits_2`), R-14's row (every
  `goad-emit` stderr line begins `goad-emit: ` in `render.rs`), and §9. The
  `goad-emit` status-1 row no longer predicts.
- **The R-56 probe** (F-3's condition), the no-view report line and AC-1's
  "at least one view answered", the type-oracle respond check and its stated
  limit, the widened fence detection, and `Alternatives::first` reused.
- **The walk.** The source probe's three patterns match what the crane filter
  produces; the host-local `git+file:` input closes the lock route; the guest
  verdict and the host person-run are both specified, with the config diff
  recorded.
- **Canon prose (brief d).** No line-number citation and no new count of a
  growing set in `canon-delta.md`, `design.md` or `slice-012.md`; ADR-003's
  "a third entry point" is a statement about a finished sequence.
  `slice-012.md` §Surfaces, AC-1, AC-7 and OQ-2/3/6 match the design.
- **U8's fixture consequence** (brief e) is clean in the corpus and in
  SPEC-001 §7; its canon consequence is F-38.

Open after round 2: F-22 (contested), F-34..F-40. Not reached: the plugin
manifests' validation (unchanged since round 1).

# Review — design — Slice 015

**Subject:** design — `docs/slices/015/design.md`, `research.md` and
`canon-delta.md` at `e231462` or later
**Reviewer:** fresh agent, Opus
**Opened:** 2026-10-19
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

**Round 1** — 2026-10-19 — the whole design, against canon and the tree.

Written by the orchestrator before the review. The invariants (`CLAUDE.md`)
hold throughout; these are the surfaces to attack:

- **The resolution door.** Where the config's directory is found and carried
  in the canonical types, and whether anything past that door still depends
  on the directory goad was started in. Every caller of the changed loading
  and parsing functions, across all three binaries and the tests.
- **Links and failures.** What resolving the config path does to every path a
  message names (SPEC-004's startup lines, SPEC-003/R-3 and R-4's), and every
  way resolution can fail.
- **The spawn.** How the program is located once the working directory
  changes: relative, absolute, bare, and `PATH` itself. What the child sees
  as its own name. Whether the stated mechanism is the one the platform
  actually runs.
- **The socket.** The resolved path, its lock sidecar, its length bound, and
  whether host and `goad-emit` provably agree.
- **The canon wording.** Each MUST is testable and has a verification row
  that would fail if the rule broke. Ids, placement, and the CLAUDE.md
  citation rules (by symbol, never count). Whether R-60 and R-17 say more or
  less than the design does.
- **The test plan.** Each AC is held by a case that the regression it guards
  would not survive (no proxies); negative controls compile; tests that
  relied on the old rule are all found.
- **Collateral.** The demo, `justfile`, `.gitignore`, the kit, the
  exercisers, slice 012's record, and anything the design's scope missed.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | | | |

### F-1 — <one-line claim>

**Severity:** blocker | major | minor | nit
**Location:** `path:line` or `design.md §5.2`

**Expected:** <what the artefact or canon says should be true>
**Observed:** <what is actually there>
**Evidence:** <the citation that makes this checkable rather than an opinion>

**Disposition:** aligned | fix-now | doc-wrong | follow-up | tolerated | settle-in-code
**Response:** <the responder's reasoning, and what was changed>

**Outcome:** verified | contested | withdrawn

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

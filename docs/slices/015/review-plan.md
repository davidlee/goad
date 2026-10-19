# Review — plan — Slice 015

**Subject:** plan — `docs/slices/015/plan.md` at `3190648`, read with
`design.md` (resolved, `review-design.md`)
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

**Round 1** — 2026-10-19 — the whole plan, against the design and the tree.
Bounded: at most two rounds (`plan-log.md` 2026-10-19; the person asked for a look
over).

Written by the orchestrator before the review. Surfaces to attack:

- **Coverage.** Every AC, every design §9 case and every canon-delta §7
  citation lands in a phase with a criterion that would fail if it were
  missing.
- **Red first.** Each case said to red today does red at its phase's start,
  for the reason claimed, and each mutation in the table compiles and reds the
  cases it names (no proxies).
- **Surfaces.** Each phase's closed list holds every file its tasks touch,
  and no two phases write the same file without saying so. Undeclared paths
  are the strongest lead.
- **Entry and exit.** Each phase's entry criteria are what the previous
  phase's exit criteria make true; each exit is checkable by an agent who did
  not write the code.
- **Size.** Each phase fits one agent in one session, bookkeeping included.
- **Mechanics.** The branch and worktree steps, the merges from `main`, the
  coordination with 012's walks, and the person's rule against `git stash`
  and `git checkout`.
- **The plan's own calls** (the kit not citing draft ids; the impossible
  arm as `ConfigError::Read`; per-file temp helpers; `chpwd.zsh`; the
  `justfile` comment) and anything the plan says that the design does not.

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

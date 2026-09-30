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

# Review — design — Slice 001

**Subject:** design — `docs/slices/001/design.md`, as at the tree state of
2026-08-23, together with the acceptance criteria in `slice-001.md` it claims to
discharge
**Reviewer:** fresh agent via codex MCP (GPT-5.5 Sol, default model)
**Opened:** 2026-08-23
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
past the gate. Reject a finding on **evidence**, never on assertion. Confirm each
disposition with the user before acting on it. Fix the class, not the instance,
and do not introduce new defects repairing old ones.

## Brief

Written before the review ran.

This design makes a large number of calls the brief does not settle, on a
codebase that does not exist yet. That combination is the risk: there is no code
to contradict a wrong decision, and no canon older than this slice to appeal to.
So the review is not looking for typos in signatures. It is looking for places
where the design has **talked itself into a position** that a reader in slice 002
or 005 will have to undo.

**Invariants the design is held to.**

1. Every acceptance criterion in `slice-001.md` is actually discharged by
   something in §9 — not by a plausible-sounding sentence elsewhere.
2. No decision in §7 contradicts another decision in §7.
3. Nothing in stratum 1 as designed can perform I/O, read a clock, or need a
   runtime (ADR-001, invariant I3).
4. Nothing designed here requires a protocol change in order for slice 002 to
   render, slice 003 to schedule, slice 004 to ingest an event, or slice 005 to
   add the socket transport. This is the slice's whole thesis; a breach is a
   blocker.
5. Where the design departs from `docs/brief.md`, it says so and says why. A
   silent departure is worse than a stated one.

**Where the bodies are likely buried.** Named in advance so the reviewer is not
credited for finding only what was easy:

- **P2's granularity rule** (§4). "A part may be discarded only when its absence
  is already a modelled state with defined semantics." Is that test sound, or
  does it license discarding more than intended? Apply it to `title`, to
  `options`, to `body`, and see whether the answers stay right.
- **AC-6 discharged through `discarded` rather than `Err`** (§5.2). AC-6 says
  each failure mode "maps to a distinct typed error". The design claims that
  arriving inside `Normalized::discarded` satisfies this. Reading or stretch?
- **AC-5 versus D18.** AC-5 requires stderr captured into diagnostics; D18
  accepts that a timed-out backend yields no stderr. Is AC-5 then false as
  written, and should it have been amended rather than reinterpreted?
- **Brief §13 versus D19.** The brief says a backend failure must not take down
  the host. D19 accepts an unbounded read that can OOM the host. Is "accepted
  with a follow-up" a legitimate disposition for a stated brief requirement, or
  is it a scope decision the user should have been asked to make explicitly?
- **`resolved_check` is not an `Option`** (§5.3) but brief §9 says "retain an
  existing valid scheduled check *if one exists*", and AC-4 takes an existing
  schedule as a parameter. Does the non-optional field quietly delete a state the
  brief distinguishes?
- **Clamping a past `next_check` to `now`** (§5.5). Brief §9 defines `next_check`
  as "do not evaluate before this point", which a past instant satisfies
  trivially. Is clamping an invention the host has no business making?
- **D16, replacement over queueing.** Brief §12 says "reject or ignore stale
  responses clearly" and "allow only one active interaction". Does replacement
  follow, or is it one reading among several?
- **D7's protocol-version asymmetry** and **D15's non-zero-exit precedence.**
  Both are choices presented as derivations. Check whether the brief actually
  supports them.
- **The claim that validation feedback is additive** (§5.5). Three properties are
  said to make it so. Try to construct a validation-feedback design that the
  types as specified cannot express without a version bump.
- **`Options` is reused** for `Choice.options` and `FieldKind::Choice.options`.
  Do those two actually have the same invariants?
- **`Outcome`'s stratum is never stated.** It is returned by `Host`, mentions
  views and diagnostics, and no section says which module owns it.
- **`Event` is mandatory on `evaluate`.** Is there a case — first startup, a
  manual poll — where there is no event, and what does the host send then?
- **Gold-plating in the type surface** (R7). The `FieldKind` and `Content`
  vocabularies are the largest thing being built and nothing renders them. Is
  every variant traceable to the brief, or did some arrive by symmetry?

**Round 1** — 2026-08-23 — the whole design document, against `docs/brief.md`,
ADR-001, ADR-002, `slice-001.md` and `research.md`.

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

**Disposition:** aligned | fix-now | doc-wrong | follow-up | tolerated
**Response:** <the responder's reasoning, and what was changed>

**Outcome:** verified | contested | withdrawn

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

# Review brief — Slice 007 design, round 2 (codex)

Hand this to a fresh agent. It is the statement of what was asked; it is not
canon and it is not a finding. Round 1's brief, `review-design-brief.md`, still
stands as the statement of what *that* round was for — read it, but do not treat
its priority list as yours.

Round 1 was run by a Claude agent. **Round 2 is run through `codex` with
`gpt-5.6-sol`**, deliberately: a second reviewer with different training is the
point, and a design that has just been repaired nineteen times is exactly
where a
fresh reader earns their keep.

---

## What you are reviewing

`docs/slices/007/design.md` **as repaired**, and
`docs/slices/007/canon-delta.md`
**as amended** — not the versions round 1 attacked. Nineteen findings were
raised, all nineteen disposed, and seventeen of them changed an artefact. The
subject is therefore a document that has been edited a great deal in a short
time by one responder, which is its own risk.

Findings go in `docs/slices/007/review-design.md`, the same ledger, appended as
**F-20 onward**. Ids are immutable and the file is append-only: do not edit,
renumber or tidy F-1..F-19. Add a `**Round 2**` line to the Brief section saying
what this round covered.

## Your three jobs, in priority order

**1. Attack the repairs.** Seventeen artefact changes landed in one session.
Each
one is a place a defect could have been introduced while removing another. The
responder's reasoning for each is written in that finding's **Response** field —
read it as a claim to be checked, never as a settled account.

This job is also, by user ruling of 2026-09-15, **round 1's verification pass**.
Round 1's raiser does not return, so you hold the raiser's outcome over
F-1..F-19: accept a repair and it is `verified`; find it wrong and the round-1
finding is `contested` and its substance becomes a new finding; judge that it
was never a defect and it is `withdrawn`. A repair you did not reach keeps an
**unset** outcome and is named in *Depth of the round* — do not mark an outcome
you did not form a view on. The ledger states the rule under *Outcome, and who
sets it*.

In particular:

- **F-4** rescoped R-58 to "the option being answered". Does the new wording
  interact correctly with R-8, R-35 and R-52? Is it now falsifiable by a backend
  author who has not read the design?
- **F-5** made `datetime` unsubmittable — a host MUST NOT submit a value and MUST
  report the field undrawn. Is *that* a narrowing under R-55, argued properly?
  The responder says it is not, because the protocol declines uniformly for every
  host rather than tracking one renderer. Test that argument.
- **F-3** moved `Draft` from `BTreeMap<(OptionId, FieldId), _>` to
  `Vec<(OptionId, FieldId, Edited)>` and dropped `PartialEq`. Does anything in the
  design still assume map semantics — ordering, deduplication, equality?
- **F-9/F-12** replaced the field selector with an option-scoped query and
  replaced `accessible-item-index` with tree-order `find_all()`. Check both
  against `i-slint-backend-testing-1.17.1/search_api.rs` rather than against the
  design's description of them.
- **F-6** moved AC-8's vehicle to stratum 3 and made three of R-57's four clauses
  "review, not a test". Is that convention being used correctly, or has it become
  a way to avoid writing tests?

**2. The threads round 1 recorded as unexamined.** They are listed in the
ledger's *Depth of the round*, and the honest ones are:

- **The grouping rule against a worked mixed sequence.** Round 1 read §5.5's
  edge cases for internal consistency and never traced grouped / ungrouped /
  undrawn fields interleaved in one option. Run-boundary rules usually fail
  exactly there. This is the largest unexamined surface in the design.
- **`examples/shell/backend.sh`** was never opened. AC-7 now requires the demo
  form to carry a field of a kind this renderer cannot draw (F-16); nobody has
  checked what the file does today beyond research F13's claim that it discards
  `values`.
- **`plan.md`**, which is still an unfilled template. Not your subject — it
  carries its own ledger — but its emptiness means no phase has yet been written
  against any of this.

**3. What round 1 missed entirely.** It found nothing in §5.3's ownership table
beyond F-2, nothing in the `Shift` / fold interaction, and nothing about
cancellation or the busy path. Either they are clean or they were not looked at;
the ledger does not distinguish. Say which, at the depth you reach.

## Known residues — do not re-raise these as findings

- **`research.md` carries the two wrong `glass.rs` citations** that F-17 fixed in
  the design (`:103-108` and `:148-164`, at `research.md:286` and `:573`). Left
  deliberately: research is a record of what research found, and rewriting it
  retroactively is a different act from correcting the design. Raise it only if
  you think the record should be annotated.
- **Whether the per-option container should declare `accessible-role: list` and
  `accessible-item-count`** is left open in §5.2, on purpose: it is a question
  about what a screen reader announces to a person, not about what a test can
  reach. Round 1's F-9/F-12 deliberately did not settle it. An argument that it
  *must* be settled in 007 is a legitimate finding; noting that it is unsettled
  is not.
- **D3 has no ADR**, by explicit user decision. Its rationale lives in
  `docs/roadmap.md` §Open decisions and SPEC-001/OQ-4. Arguing the record is in
  the wrong place is fair; arguing it should be an ADR has been decided.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md`.
2. `docs/slices/007/review-design.md` — **the Protocol section is binding on
   you**, and the nineteen dispositions tell you what has already been argued.
3. `docs/slices/007/design.md`, all of it. `canon-delta.md`. `slice-007.md`.
4. `docs/slices/007/research.md` — rows are claims with sites named, not checked
   claims. Where the design load-bears on one, check it.
5. `docs/specs/001-host-backend-protocol.md`, all of it. `docs/adr/001`, `003`,
   `004`. `docs/policy/001`. `docs/specs/002` §8.

`design-log.md` is read **last**, after you have formed your own view. It is the
argument that produced the design and it is persuasive by construction; reading
it early is how a reviewer talks themselves into `aligned`.

## Environment

- Repository at `/home/david/dev/goad`, **unjailed**. There is no `/workspace`
  and no display.
- Pinned Slint sources — the only admissible authority on Slint API facts, and a
  claim without a file and line in them is not evidence:
  - `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/i-slint-core-1.17.1/`
  - `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/i-slint-compiler-1.17.1/`
  - `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/i-slint-backend-testing-1.17.1/`
- `just check` is not yours to run. Change no code, amend no canon, edit no
  document but `review-design.md`. **Do not edit `design.md`** — a design defect
  is a finding, not a repair.

## Driving codex

Load the tool first: `ToolSearch("select:mcp__codex__codex")`, then open a
session with **`gpt-5.6-sol`**. Continue it with `mcp__codex__codex-reply`
rather
than opening a new session per question — the review needs one accumulating
context, not twelve cold starts.

You are the driver, not the reviewer. Codex reads and judges; you carry its
findings into the ledger. Two rules make that safe:

- **Prefer codex writing into `review-design.md` directly** if its sandbox
  permits it. If it does not, take findings **one at a time** — ask for F-20,
  write it, ask for the next — because a final report that lists twelve findings
  will truncate and the ledger is the deliverable, not the report.
- **Do not improve what codex says.** If a finding is wrong, it is still the
  finding: a wrong finding is withdrawn by the raiser at outcome time, not
  filtered out by the person holding the pen. If you disagree, note your
  disagreement as the responder — in a separate turn, wearing the other hat.

## Rules

- **Raiser only on your own findings.** Leave `disposition`, `response` and
  `outcome` blank on every new finding; the author disposes with the user, per
  the ledger's Protocol. The one column you do fill is the **outcome** of
  F-1..F-19, per job 1 — never their disposition or response.
- Each finding needs Expected / Observed / **Evidence**, where evidence is a
  citation that makes the claim checkable by someone who disagrees.
- Severity is set at raise time and not negotiated afterwards.
- **A round with no findings is not done** — it means the review has not run. If
  you genuinely find nothing at a given depth, say at what depth you looked, in
  the *Depth of the round* section, as round 1 did.
- If the work heads past roughly 200k tokens, write what you have into the
  ledger, say which threads are unexamined, and stop. A second agent continues
  from the file.
- If the environment contradicts this brief, say so in the ledger and work
  around it.

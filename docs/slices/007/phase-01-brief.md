# Hand-over brief — Slice 007, PHASE-01

Hand this to a fresh agent. It is the statement of what was asked. It is not
canon, it does not override `plan.md` or `design.md`, and where it seems to,
it is wrong.

**Do not start until both gates are recorded.** `plan.md` accepted, and
`design.md` re-approved after its §5.2 amendment — the entries live in
`plan-log.md` and `design-log.md`. Check for them; do not infer them from this
file existing.

---

## Where the slice stands

**Stage: execute, PHASE-01. Tier 2.**

Two ledgers are behind you and neither is yours to reopen:

- `review-design.md` — **resolved**, 32 findings, all verified, no blocker. Its
  **Synthesis** is the closure story; read that and you will not need the
  findings.
- `review-plan.md` — **one round, 13 findings** (1 blocker, 2 major, 7 minor,
  3 nit), all `doc-wrong`, all repaired in `plan.md` before you arrived. Its
  Synthesis says what changed. You do not need the findings either, with one
  exception named under *What the review taught*, below.

`canon-delta.md` is **draft canon**: R-57, R-58, and an amendment to SPEC-001
§7's existing R-18 row. Cite it exactly as you would SPEC-001. **Do not edit
`docs/specs/`** — promotion is audit's, with explicit user endorsement.

`notes.md` is an untouched template. Its Status table and its Phase sheets
section are yours.

## Your job, in order

1. **Phase-plan.** Expand `plan.md`'s PHASE-01 entry into a phase sheet under
   `## Phase sheets` in `notes.md`: reading list (`path:line`, the binding
   design sections, prior art), assumptions, STOP conditions, task breakdown.
   **The sheet's first task is to re-derive its criterion ids from `plan.md`**,
   not to trust your own copy of them — `plan.md`'s Overview carries the rule
   and `review-plan.md` F-7 is why it exists: three stale citations came out of
   a single renumber, in a document whose whole subject is that class of
   defect.
   Do this **now**, immediately before executing — not for any later phase. A
   sheet written three phases early is fiction.
2. **Verify EN-1** before you touch anything: `just check` exits 0 on a clean
   tree. Keep the transcript. If it is not green, that is the state of the
   world, not a reason to proceed.
3. **Execute.** Red / green / **refactor**. The refactor step is where the
   design survives contact and it is not optional.
4. **End green**, with every EX and VT discharged and the status table set to
   `done`, and the Harvest updated **in place** before you hand off.

**If expanding the phase shows the plan is wrong, go back to plan** — or to
design, if that is where it lives. Do not repair it quietly in the sheet.
`plan.md`'s nine STOP conditions (S-1..S-9) are the list; S-1 is that road.

## What PHASE-01 is

Back-pressure gets an owner. Today `Wire::send` writes `BUSY_NOTICE` straight to
the window on a full channel and `Glass::present` clears it unconditionally, so
the present that reverts a dropped action is the present that deletes the
explanation for it. The repair retains the notice at the **edge** — a `Notice`
watch channel following `Cancel`'s route edge for edge — has `serve` sample it
at present time, and makes `Glass::present` the only writer of the property.

`Controller` gains no field. `Frame` gains one, and becomes *more* total for it:
its doc-comment's "every property **but** `notice`" carve-out was the defect,
not a convenience.

**This phase discharges no acceptance criterion, and that is correct**, not a
sign it is optional. It arrived through design review after `slice-007.md`'s
criteria were written; its exit criteria are `design.md` §5.1, §5.3 and §5.4.

## The three things most likely to go wrong

- **EX-7's ten prose homes.** The compiler catches none of them. The review
  found two the original list missed, and both are invisible to VA-2's
  `grep -rin notice` because neither line contains the word. Work from the list,
  and treat VA-2 as a second instrument rather than the first.
- **The signature sweep.** `Controller::frame`, `serve` and `Wire::new` all
  change shape — 69, 36 and 6 call sites, measured, across six test files. Let
  the compiler drive it. It is call shape only and changes no assertion; that is
  S-2's stated allowance, not a breach of it. Expect `serve` to be the awkward
  one: thirteen of its sites are in `ingress.rs` and fourteen in `scheduling.rs`,
  which PHASE-01's Surfaces would not lead you to expect.
- **`Wire` loses its window handle,** and `design.md` does not say so. It is
  forced, not chosen: with `Glass::present` the only writer of `notice`, the
  field is never read, and that is an error under the gate's `-D warnings`.
  EX-4 enumerates the whole blast radius. Take it as an improvement — `Wire`
  afterwards names no Slint type at all — and not as licence to move anything
  else.

## What the review taught, and it applies to you

`docs/memory/a-repair-sweep-misses-the-binding-site.md`. The design review hit
this class three times, and the plan then committed it inside the criterion
written to prevent it. **Where you change a claim, find every live home of it
before you call the change done** — and remember that a text search for the
repaired wording matches none of the homes that matter: a requirements row, a
sequence diagram, a doc-comment that states the rule in different words.

Also in force: `docs/memory/cite-requirements-not-finding-ids.md`. A comment in
`src/` or `tests/` cites `SPEC-001/R-58`, a spec section, or an ADR — never a
slice-local `F-N` or `D-N`.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md` — §Phase plan, §Execute, §Where it goes.
2. `plan.md` — the Overview, Sequencing (including S-1..S-9), and PHASE-01
   whole. Skim the other five so you know what you are not doing.
3. `design.md` §5.1 (system model), §5.3 (state and ownership — the notice's two
   tables), §5.4 (*When it did not arrive, the correction is the feedback*).
4. `slice-007.md` §Scope — your surfaces are declared there as well as in the
   plan, and the audit diffs actual paths against it.
5. The code: `crates/goad/src/{wire.rs, controller.rs, main.rs, glass.rs,
   diagnostics.rs}`, then `crates/goad/tests/renderer/wiring.rs`'s
   `mod back_pressure`.
6. `review-plan.md` §Synthesis. The findings only if a repair puzzles you.

## Environment and the gate

Nix devshell (`nix develop`, or direnv). **Run unjailed** — the design cites
pinned Slint and tokio sources under `~/.cargo/registry/src/index.crates.io-*/`,
which the jail does not reach. If `nix develop` is involved, use the bare
git-input flake reference: a `path:` ref breaks on the demo socket
(`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).

`just check` is the gate — build, both test tiers, the example typecheck, lint,
format. Nothing is green until it exits 0, and PHASE-01/EX-9 is that check.
`just -n check` prints the sequence; `docs/policy/001-the-phase-gate.md`'s
command block is canonical and the `justfile` mirrors it.

**One writer per worktree.** Do not spawn a second agent that writes, and do not
rewrite history (`docs/memory/one-writer-per-worktree.md`,
`git-stash-forbidden-recover-read-only.md` — no `git stash`, ever).

## Not yours

- **PHASE-02 onward.** Do not write their phase sheets and do not start them.
- **Canon.** `docs/specs/`, `docs/policy/`, `docs/adr/` are untouched by every
  phase in this slice. `canon-delta.md` is promoted at audit.
- **The look.** 008's, bounded by AC-10, and PHASE-06's when it comes.
- **`crates/goad-semantics/`.** Nothing there changes (S-4).

## When you are done

- Every EX and VT in PHASE-01 discharged; `just check` exit 0.
- `notes.md`: the status table says `done`, the phase sheet is current — kept as
  you went, not written at the end — and the Harvest is updated in place.
- Tell the user what you did, what you found in passing (that is `notes.md`
  Findings, and it feeds the audit), and anything the next phase should know.

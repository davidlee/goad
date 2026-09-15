# Review brief — plan — Slice 007, round 1

Handed to a fresh agent. This is the statement of what was asked. It is not
canon, and it does not override `plan.md`, `design.md` or anything in
`docs/{specs,policy,adr}/`.

The ledger's own **Brief** section (`review-plan.md`) is the normative version
of the target list; this file is the working prompt around it.

---

## What you are reviewing

`docs/slices/007/plan.md` — the executable phase plan for slice 007, six
phases. Nothing else is the subject.

**`design.md` is not under review.** It is approved (`design-log.md`,
2026-09-15) and `review-design.md` is resolved at 32 findings, all verified,
none withdrawn, no blocker. If you believe the *design* is wrong, that is a
finding about the design and it goes to the user as a STOP, not into this
ledger as a plan defect. Say so plainly rather than quietly re-planning around
it.

**`canon-delta.md` is draft canon.** R-57 (a submitted value's JSON type is
fixed by the field's `kind`, for all five kinds) and R-58 (a `respond` carries
values for exactly the fields the host drew of the option being answered). Cite
it exactly as you would SPEC-001. It is promoted at audit, by someone else.

## The one question

**If an agent executed `plan.md` exactly as written, phase by phase, would the
design exist at the end — and could any phase be declared done while the thing
it claims to establish is false?**

Everything below is that question asked in a particular place.

## The six invariants (the ledger's Brief states these normatively)

1. **Coverage is total and honest.** All ten of `slice-007.md`'s acceptance
   criteria, each mapped to a named criterion in a named phase, and the named
   criterion actually reaches the observable `design.md` §9 gives for that AC.
2. **No criterion is a proxy.** §9's rule: every field test either reads the
   wire or asserts something about the screen — except AC-5, which needs both.
3. **Entry proves the previous phase done; exit makes the objective true.**
4. **Surfaces are complete.** A phase that must touch a path its Surfaces list
   does not name is a defect in the plan.
5. **One phase, one agent, one session, bookkeeping included.**
6. **The plan takes no decision that is not its to take.**

## Where to look hardest

- **PHASE-01/EX-7.** Eight live homes of the old `notice` rule, enumerated
  because the compiler catches none of them. Is the list complete? Is any entry
  on it wrong? `grep -rin notice crates/goad/src crates/goad/ui crates/goad/tests`
  is the instrument; read every hit against the list.
- **PHASE-01/EX-4**, which removes `Wire`'s `slint::Weak<PromptWindow>`. The
  design does not state this; the plan argues it is *forced*, because
  `Glass::present` becomes the only writer of the window's `notice` property,
  which leaves the field never read, and `dead_code` is an error under the
  gate's `-D warnings`. Check the claim and check its blast radius (six
  `Wire::new` call sites, the hand-written `Debug`, the module doc).
- **The 02/03/04 split.** Does PHASE-02 or PHASE-03 need something only PHASE-04
  provides, so that the phase cannot end green with `just check` at 0?
- **AC-6.** The plan claims the only existing assertions that change in the
  entire slice are `mapper.rs:156-194`'s (PHASE-03/VA-2) and
  `wiring.rs:346-385`'s (PHASE-01/VT-1), and that everything else is call shape.
  Test that: walk each phase's exit criteria and ask what existing test it
  breaks.
- **AC-5 (PHASE-05/VT-3).** Both halves required, neither implying the other.
  Does the criterion as written reach both, and is the ordering of the fold and
  the submit right?
- **PHASE-05/EX-2**, the lift of `logging_scripted` from
  `tests/renderer/scheduling.rs:95-105` to `harness.rs`. Does it breach the
  plan's own S-2, and is `harness.rs` the right home under that file's stated
  rule (what two or more modules in this target need)?
- **Anything implying an instrument checks what it does not.** POL-001
  §Verification and `design.md` §3: none of the four ADR-001 instruments reaches
  stratum 3, so `view_model.rs` staying pure and `Draft` never entering
  `Presentation` are held by review alone. The vocabulary scan checks seven
  words and `group` is not one of them.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md` — §Where it goes, §Plan, §Phase plan,
   §Tiers.
2. `docs/slices/007/plan.md`, whole.
3. `docs/slices/007/design.md` — §5.2 interfaces, §5.3 state and ownership,
   §5.4 lifecycle, §5.5 invariants and assumptions, §9 validation. §9 is the one
   the plan's verification column is derived from.
4. `docs/slices/007/slice-007.md` — the ten acceptance criteria and §Scope.
5. `docs/slices/007/canon-delta.md`.
6. `docs/policy/001-the-phase-gate.md` §Verification, and `docs/specs/001-*.md`
   where a requirement is cited at you.
7. The code the plan names, at the lines it names. Do not take a `path:line`
   citation on trust — several in this slice's history were stale.
8. `review-design.md` §Synthesis only, for what the design review already
   settled. Do not re-raise a question it closed; say so and move on.

## How to record what you find

Append to `docs/slices/007/review-plan.md` under `## Findings`, following the
**Protocol** section at the head of that file. You hold the **raiser** role and
only that role:

- Fill the summary table row and a `### F-N` block per finding: severity,
  location, **Expected / Observed / Evidence**.
- Leave **Disposition**, **Response** and **Outcome** blank. They are the
  responder's, and the responder confirms each with the user.
- Ids are immutable and append-only. The ledger currently has none, so start at
  `F-1`.
- Severity at raise time, not negotiated: `blocker` gates acceptance, `major` is
  a real defect that does not gate, `minor` is survivable, `nit` is taste.
- **Evidence, not assertion.** Every finding needs a citation that makes it
  checkable — a `path:line`, a requirement id, a section. A finding whose
  evidence is your own reasoning about what is likely is a `nit` at best.
- Do not edit `plan.md`, `design.md`, `slice-007.md` or any other file. The
  ledger is the only file you write.

**A ledger with no findings is not a passed review — it means the review has
not run.** If the plan genuinely holds, say so in your report with what you
attacked and found sound; do not manufacture findings to fill the table.

## Two of this project's own lessons bear on your output

- `docs/memory/cite-requirements-not-finding-ids.md` — cite requirements
  (`SPEC-001/R-58`), design sections, or `path:line`. Do not cite
  `review-design.md` finding ids at anyone.
- `docs/memory/a-repair-sweep-misses-the-binding-site.md` — the class this
  slice's design review hit three times. Where a plan changes a claim, its exit
  criteria should name **every live home** of that claim. That is the shape of
  defect most likely to still be in this plan.

## Environment

Nix devshell. **Run unjailed**: the plan cites pinned Slint and tokio sources at
`~/.cargo/registry/src/index.crates.io-*/`, which the jail does not reach.

You do not need to run `just check`, and should not — the tree is mid-slice and
nothing has been implemented. Read.

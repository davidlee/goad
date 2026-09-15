# Plan brief — Slice 007

Hand this to a fresh agent. It is the statement of what was asked; it is not
canon, and it does not override `design.md`.

---

## Where the slice stands

**Stage: plan. Tier 2.** The design is approved as it stands
(`design-log.md`, 2026-09-15).

`review-design.md` is **resolved**: 32 findings, all `verified`, none withdrawn,
no blocker outstanding. Two adversarial rounds plus a confirmation pass. Its
**Synthesis** is the closure story — read it and you will not need the findings.

`canon-delta.md` carries **R-57** (a submitted value's JSON type is fixed by the
field's `kind`, for all five kinds) and **R-58** (a `respond` carries values for
exactly the fields the host drew of the option being answered). It is **draft
canon**: it is this slice's working authority and you cite it exactly as you
would SPEC-001, but nothing outside the slice may cite it and it is promoted at
audit, not by you.

`plan.md` and `notes.md` are untouched templates. They are your work.

## Your job

Fill in `plan.md` from its template. Then stop and put two things to the user:
whether to subject the plan to adversarial review (`review-plan.md`, copied from
`docs/templates/review-ledger.md`), and their acceptance of the plan.

**No code.** Planning is not execution, and `docs/AGENTS.md` is explicit that
phase sheets are written immediately before a phase runs, not up front for all
of them. Do not write phase sheets into `notes.md` now.

**If planning surfaces an unresolved design issue, go back to design** and work
forward from there. Do not repair it quietly in the plan. `docs/AGENTS.md`
§Plan says this outright, and this slice has just spent two review rounds on the
cost of a claim living in two homes.

User decisions you take during planning go in **`plan-log.md`** (new file),
never in `plan.md`.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md` — §Where it goes, §Plan, §Phase plan,
   §Tiers.
2. `ls docs/{specs,policy,adr}/*` and read what bears. SPEC-001 is the one that
   does; POL-001 defines the gate.
3. `docs/slices/007/slice-007.md` — purpose, scope, the ten acceptance criteria,
   and the six closed open questions.
4. `docs/slices/007/design.md` — all of it. §5.2 is the interface inventory,
   §5.3 state and ownership, §5.4 lifecycle, §5.5 invariants and assumptions,
   §9 validation.
5. `docs/slices/007/canon-delta.md`.
6. `docs/slices/007/research.md` — 709 lines, and the findings are cited as
   `research F1`..`F13` throughout the design. **Two `glass.rs` citations in it
   are stale and were left so deliberately** (see *Residues* below).
7. `review-design.md` §Synthesis, then the findings only if you need them.
8. The code: `crates/goad/src/{controller,wire,glass,view_model,reception,
   install,diagnostics}.rs`, `crates/goad/ui/app.slint`, `crates/goad/build.rs`,
   `crates/goad/tests/renderer/`, `examples/shell/backend.sh`.

`design-log.md` is long and append-only; read the entries a decision cites
rather than the file.

## §9 has already done half of your verification column

`design.md` §9 names, for every AC, **the observable a regression would break**
and the vehicle that reaches it, with precedent. Your `VT-`/`VA-`/`VH-` criteria
should cite those rows, not reinvent them. Three things in it bind:

- **The proxy rule.** Every field test either reads the wire or asserts
  something about the screen. A test that reads the draft through `Controller`
  and never reads an invocation log would pass with `answer()` walking the
  draft's keys — D6, the defect most worth catching. This slice has already been
  bitten twice: `docs/memory/a-green-test-can-assert-a-proxy.md`.
- **AC-5 needs both halves**, and is the one row where a disjunction is wrong.
  The wire value is built from the draft, which no present writes, so a present
  that stops writing `checked` leaves the wire green and the screen wrong.
- **A-1 and A-2 are discharged from the pinned sources, not open.** §9 says it
  plainly: phase 1 **pins** each as a regression against a future Slint, and
  neither may be carried to a later phase as an open question. A plan that sends
  phase 1 to *discover* either is wrong.

## The `notice` repair is new work, and it is the easiest thing in the slice to miss

It arrived through the review (F-29 → F-32), not through the original design, so
it is absent from the slice card's scope list and from `research.md`. §5.1, §5.3
and §5.4 carry it. It must have a phase:

- `Frame` gains `notice: bool`; `Controller::frame` gains one parameter and
  stays `&self`; **`Controller` gains no field**.
- `main` constructs a `Notice` — a `watch` channel following `Cancel`'s route
  edge for edge (`wire.rs:142-180`, `main.rs:85-119`) — clones it into `Wire`,
  and passes it to `serve` beside `cancel` (`controller.rs:576-584`).
- `Wire::send` sets it `true` on `Full` and `false` on `Ok`. `serve` samples it
  at present time. `Glass::present` writes it from the frame and becomes the
  **only** writer to the window's `notice` property.
- **An existing green test inverts.**
  `crates/goad/tests/renderer/wiring.rs:346-385` asserts *"the next present must
  clear notice"* — the mechanism that causes the defect, asserted as the
  requirement. It must assert that the notice survives the next present and that
  the person's next successful send clears it. The design says the inversion is
  a **deliverable, not a fixup**; give it its own exit criterion.
- Two doc-comments state the old rule and must move with it: `glass.rs:23`
  (*"`notice` is written `""` here and set from nowhere else in this trait"*)
  and `controller.rs:92-93` (*"Total: every property **but `notice`** is written
  from this, every time"* — with the field present, the "but" goes, and that is
  §5.1's stated reason the change is an improvement rather than a cost).
- That test's assertion message cites `design.md §5.3`, which no longer holds
  the clearing rule.

## Sizing, and what each phase must declare

One phase, one agent, one session, **bookkeeping included**: a phase an agent
cannot finish and write up in one session is two phases. Every phase declares its
**surfaces**;
undeclared paths are what the audit hunts for, and disjoint surfaces are what
makes two phases parallelisable.

Entry criteria are checked *before* a phase starts and are how the previous
phase is proved done. Exit criteria are what makes the phase's objective true.
Ids (`PHASE-NN`, `EN-`/`EX-`/`VT-`/`VA-`/`VH-N`) are immutable and append-only —
a split leaves the sequence non-monotonic, and that is expected.

The **Coverage** table must map all ten ACs. A gap there is a gap in the plan.
Note where the vehicle is a person rather than a test: **AC-7** (`just demo`,
a multi-field form carrying at least one undrawn kind, submitted once) and
**AC-10** (a person judges legibility) are `VH-` criteria, must run outside the
jail because there is no display in it, and are recorded in `audit.md` under
Evidence. `docs/AGENTS.md` §Tiers: a green gate is not that evidence, and slices
001–003 all closed green on a binary that could not open a window.

## Four things that look like plan decisions and are not

- **AC-8's promotion happens at audit**, with explicit user endorsement — not in
  a phase. Canon is not amended mid-slice. But a slice does not close holding an
  unpromoted draft, so the plan must leave the reconciliation reachable.
- **The look is 008's.** AC-10 bounds what may *change* to what drawing fields
  forces: the block container and its separator, and the heading's treatment.
  Everything else is recorded verbatim and not actioned, and becomes 008's
  brief.
- **The six open questions are closed** (`slice-007.md` §Open questions, with
  the reasoning in `design.md` §6 and §7). So are D1–D13. Re-opening one is a
  design change, not a planning choice.
- **SPEC-002/OQ-4 stays deferred.** A scheduled firing can still replace a
  half-filled form. It was answered during design and withdrawn on scope (D13).

## Residues, known and not yours to fix

- `research.md` carries two stale `glass.rs` citations, left deliberately.
- The `accessible-role: list` / `accessible-item-count` announcement question is
  open, and is named in the design.
- **D3 has no ADR** — user decision, 2026-09-15.
- `canon-delta.md`'s R-57 reads "…`number` a JSON number, and `choice` … as a
  JSON string, **and** `datetime` an RFC 3339 `date-time` string": two `and`s,
  the seam where F-21's repair appended `datetime`. Copy-editing, noted so it is
  not rediscovered, and worth a comma before it is promoted into SPEC-001
  verbatim.

## Two of this project's recorded lessons bear directly on the plan

- `docs/memory/cite-requirements-not-finding-ids.md` — `plan.md` is
  current-truth and outlives this review. Cite **R-57**, **R-58**, an AC or a
  design section; do not cite `F-29` at an implementer who has no reason to read
  a ledger.
- `docs/memory/a-repair-sweep-misses-the-binding-site.md` — the same failure
  this review hit three times (F-22..F-27, again at F-30, again at F-32). Where
  a phase changes a claim, its exit criteria should name **every live home** of
  that claim, not the first one.

## Environment and verification

Nix devshell (`nix develop`, or direnv). **Run unjailed**: the design cites the
pinned Slint and tokio sources at
`~/.cargo/registry/src/index.crates.io-*/`, which the jail does not reach, and
AC-7/AC-10 need a display.

`just check` is the gate — build, both test tiers, the example typecheck, lint,
format. Nothing is green until it exits 0. `just -n check` prints the sequence;
the command block in `docs/policy/001-the-phase-gate.md` is canonical and the
`justfile` mirrors it.

What the gate holds is not one number: four ADR-001 instruments, the
domain-vocabulary scan (a different invariant), and one residue nothing
enforces. **Stratum 3's purity is reached by no instrument** — `view_model.rs`
staying pure, and `Draft` never entering `Presentation`, are held by review
alone. Say so where a phase depends on it; do not write a `VA-` criterion that
implies an instrument checks it.

## When you are done

- `plan.md` filled: Overview, Sequencing & rationale, Coverage (all ten ACs),
  and every phase with objective, surfaces, entry, exit and verification.
- Anything an implementer who has read the design still would not know goes in
  each phase's *Notes for the implementer* — prior art, gotchas, attack order,
  what not to touch.
- Put the adversarial-review choice to the user, then ask for their acceptance.

# Hand-over brief — Slice 007, PHASE-02

Hand this to a fresh agent. It is the statement of what was asked. It is not
canon, it does not override `plan.md` or `design.md`, and where it seems to,
it is wrong.

**No user gate is outstanding.** `plan.md` is accepted and `design.md` approved
(`plan-log.md`, `design-log.md`). Your entry condition is PHASE-01's exit, not a
person's signature — check it, do not assume it.

---

## Where the slice stands

**Stage: execute, PHASE-02 of six. Tier 2.**

PHASE-01 is `done` and committed as `fd4b162` ("007 PHASE-01: the notice gets an
owner"). The gate was green at that commit — `just check` exit 0, 506 tests
across 21 binaries — and verified independently of the agent that wrote it.

Three ledgers are behind you and none is yours to reopen: `review-design.md`
(resolved, 32 findings), `review-plan.md` (resolved, 13/13 verified), and
PHASE-01's record in `notes.md`. Read `notes.md`'s **Harvest** before anything
else in the slice folder — it is four phases' worth of hard-won detail compressed
into one screen, and it is current as of `9447973`.

`canon-delta.md` is **draft canon**: R-57, R-58, and an amendment to SPEC-001
§7's existing R-18 row. Cite it exactly as you would SPEC-001. **Do not edit
`docs/specs/`** — promotion is audit's, with explicit user endorsement.

## Your job, in order

1. **Phase-plan.** Expand `plan.md`'s PHASE-02 entry into a sheet under
   `## Phase sheets` in `notes.md`, beside PHASE-01's. Reading list
   (`path:line`, the binding design sections, prior art), assumptions, STOP
   conditions, task breakdown. **The sheet's first task is to re-derive its
   criterion ids from `plan.md`** — `plan.md:35-45` carries the rule and
   `review-plan.md` F-7 is why. PHASE-01's sheet is the worked example of the
   shape expected; follow it.
2. **Verify EN-1** before you touch anything: PHASE-01's exit criteria
   discharged, `just check` exit 0. Keep the transcript.
3. **Execute — and see VA-2 below, because it constrains the order of your
   first two commits' worth of work, not just your verification.**
   Red / green / **refactor**.
4. **End green**, every EX and VT discharged, status table `done`, Harvest
   updated **in place** — it is not append-only, and PHASE-01's entries stay.

**If expanding the phase shows the plan is wrong, go back to plan** — or to
design. Do not repair it quietly in the sheet. `plan.md:102-142` is S-1..S-9;
S-1 is that road.

## What PHASE-02 is

The markup learns to declare fields, a headless test learns to drive a checkbox
and address it **by option**, and the two Slint assumptions the design settled on
paper become regressions that fail here rather than three phases later.

It is placed second to meet the risks that come from **outside this workspace**
early: the Slint style default (R-7) and the codegen and model-reset assumptions
A-1 and A-2. It is markup and what a headless window can be asked on its own —
no mapper, no draft, no controller. Those are 03 and 04.

## The five things most likely to go wrong

- **VA-2 is an ordering discipline, not a check you run at the end.** Apply
  `build.rs` **first and alone**, run `cargo test -p goad`, record it green, and
  only then write markup. R-7's named signal is `cargo test -p goad` going red
  immediately after the `build.rs` change, and that signal is **worthless once
  two changes are in the same diff**. There is no recovering it afterwards by
  reasoning.
- **EX-4's guard is the criterion neither criterion implies.** One container
  **per option**, `if option.blocks.length > 0`. A container produced by the
  `for` over `blocks` would be per-*block* and would carry `option.id` once per
  block; an unguarded per-option container exists whether `blocks` is empty or
  not. The guard reconciles the two, and it is what makes AC-6's empty case
  *absent* rather than *empty*.
- **EX-7 disambiguates a helper, and two of its prose homes are invisible to the
  compiler.** EX-3 gives the container the same `accessible-description:
  option.id` the option's `Button` already carries, so two elements answer to one
  description and `find_first()` takes whichever the walk reaches first. The
  helper gains `match_inherits("Button")`. This is a change to a **helper's
  implementation** — expressly allowed by S-2, and not an assertion or a fixture.
  The two prose homes are `tree.rs:40-41` and `app.slint:49-51`, both claiming
  the description is *"the only unambiguous one"*. See *What PHASE-01 taught*.
- **A-1 and A-2 are settled from the pinned sources, not open.** Neither is
  yours to discover and neither may be carried forward as a question. If a pin
  goes red that is a finding about a changed dependency, not a licence to
  redesign — `design.md` §8/R-1 keeps the two-flat-models fallback named and
  unneeded, and S-1 is the road. **Do not reach for `set_row_data` to keep
  keyboard focus**: it keeps focus and silently detaches the checkbox from the
  model, which is the defect A-2 exists to exclude. Focus loss on every present
  is known, accepted, and already a Follow-up.
- **VT-4 uses `find_all()` against a standing warning that is too broad.**
  `tree.rs:66-68` says never `find_all().len()` because the list virtualises.
  Virtualisation is the `ListView` path only; `app.slint:40-46` is a `ScrollView`
  around a `VerticalLayout` with a plain `for`, and a plain repeater creates
  every instance. Correct `tree.rs:68` to say which markup it is true of.
  Leaving it standing hands the next reader a contradiction.

## What PHASE-01 taught, and it applies to you

One shape landed three times, each time in the artefact produced to fix the last:
the repair falsified a doc-comment (`controller.rs:113`), the lint exemption
taken to land the repair falsified another (`:575`), and the VA-2 accounting of
both then falsified itself — a class labelled `2` with five sites under it.

Three consequences you inherit directly:

- **An enumerated list of live homes is a floor.** EX-7 named ten; there were
  twelve. Where you change a claim, find every live home of it — and a text
  search for the repaired wording matches none of the homes that matter.
  `docs/memory/a-repair-sweep-misses-the-binding-site.md`.
- **Derive counts from the instrument; never reconcile them to a total.** A
  reconciled number adds up and is still false. If you write an accounting, give
  each class a rule an auditor can run.
- **A mechanical sweep edits comments too.** PHASE-01's `.frame()` sweep was
  right at 64 call sites and wrong at the two that were prose. Your EX-5 is
  another mechanical sweep (`blocks: ModelRc::default()` at two sites). Read the
  prose around your own fix before calling it done.

Also in force: `docs/memory/cite-requirements-not-finding-ids.md`. A comment in
`src/`, `ui/` or `tests/` cites `SPEC-001/R-58`, a spec section, or an ADR —
never a slice-local `F-N` or `D-N`.

## Left open deliberately

Whether the container should also declare `accessible-role: list` with
`accessible-item-count`, so a screen reader announces a field's position
(`design.md` §5.2). **Do not settle it by what a test happens to need.** If you
find yourself wanting it, that is S-1.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md` — §Phase plan, §Execute, §Where it goes.
2. `notes.md` — the Harvest first, then PHASE-01's sheet as the worked shape.
3. `plan.md` — Overview, Sequencing (including S-1..S-9), and PHASE-02 whole
   (`:304-442`). Skim 03–06 so you know what you are not doing.
4. `design.md` §5.2's *The window* (`:400-468`) and *The build* (`:633-653`) —
   the structs, the callback and the style read, as given; §5.4 and §5.5 for
   A-1, A-2 and the model-reset mechanism; §8 for the fallback that stays named
   and unneeded.
5. `slice-007.md` §Scope — your surfaces are declared there as well as in the
   plan, and the audit diffs actual paths against the card.
6. The code: `crates/goad/build.rs`, `ui/app.slint`, `src/glass.rs`'s
   `option_rows` — **`plan.md` EX-5 cites it at `:138-149` and PHASE-01 moved it
   to `:140`; find it by name, and take that as the first live reminder that a
   line number in this plan was measured before the phase ahead of you ran** —
   `tests/renderer/tree.rs` — `:28-38` (the rows builder),
   `:42-49` (`element_described`), `:66-68` (the virtualisation warning),
   `:81-83` (the `match_inherits` shape you will copy).

## Environment and the gate

Nix devshell (`nix develop`, or direnv). **Run unjailed** — the design cites
pinned Slint sources under `~/.cargo/registry/src/index.crates.io-*/`, which the
jail does not reach, and this phase's two pins are read from them. If `nix
develop` is involved, use the bare git-input flake reference: a `path:` ref
breaks on the demo socket (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).

`just check` is the gate; PHASE-02/EX-8 is that check. `just -n check` prints the
sequence, and `docs/policy/001-the-phase-gate.md`'s command block is canonical.

**One writer per worktree.** Do not spawn a second agent that writes, and do not
rewrite history (`docs/memory/one-writer-per-worktree.md`). **No `git stash`,
ever.** Do not commit — git is the user's; ask through the orchestrator if you
want a checkpoint.

## Not yours

- **PHASE-03 onward.** Do not write their sheets and do not start them. In
  particular `view_model.rs`, `draft.rs` and the controller are 03's and 04's,
  however obvious the next step looks from inside the markup.
- **Canon.** `docs/specs/`, `docs/policy/`, `docs/adr/` are untouched by every
  phase in this slice.
- **The look.** 008's, bounded by AC-10, and PHASE-06's when it comes. Typography,
  window sizing and the treatment of controls are not this phase's even though
  you are the phase holding the markup open.
- **`crates/goad-semantics/`.** Nothing there changes (S-4).

## When you are done

- Every EX and VT in PHASE-02 discharged; `just check` exit 0.
- `notes.md`: status table `done`, the sheet current — kept as you went, not
  written at the end — and the Harvest updated in place beside PHASE-01's.
- Tell the orchestrator what you did, what you found in passing (that is
  `notes.md` Findings, and it feeds the audit), and what PHASE-03 should know.

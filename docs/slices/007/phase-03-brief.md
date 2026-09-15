# Hand-over brief — Slice 007, PHASE-03

Hand this to a fresh agent. It is the statement of what was asked. It is not
canon, it does not override `plan.md` or `design.md`, and where it seems to,
it is wrong.

**No user gate is outstanding.** Your entry condition is PHASE-02's exit, not a
person's signature — check it, do not assume it.

---

## Where the slice stands

**Stage: execute, PHASE-03 of six. Tier 2.**

```
9447973  design resolved, plan accepted
fd4b162  PHASE-01  the notice gets an owner      506 tests
36c13ad  PHASE-02  the window draws a form       511 tests
```

The tree is clean at `36c13ad` and the gate was green there, verified
independently of the agents that wrote it. Read `notes.md`'s **Harvest** before
anything else in the slice folder — two phases of hard-won detail on one screen —
then PHASE-01's and PHASE-02's sheets as the worked shape yours should take.

`canon-delta.md` is **draft canon**: R-57, R-58, and an amendment to SPEC-001
§7's existing R-18 row. Cite it exactly as you would SPEC-001. **Do not edit
`docs/specs/`** — promotion is audit's, with explicit user endorsement.

## Your job, in order

1. **Phase-plan.** Expand `plan.md`'s PHASE-03 entry into a sheet under
   `## Phase sheets` in `notes.md`, beside the two that are there. **The sheet's
   first task is to re-derive its criterion ids from `plan.md`** — the rule is at
   `plan.md:35-45` and `review-plan.md` F-7 is why it exists.
2. **Verify EN-1**: PHASE-02's exit criteria discharged, `just check` exit 0.
   Keep the transcript. PHASE-02 is not a code dependency of yours — the
   ordering is the plan's, and EN-1 is how the previous phase is proved done.
3. **Execute.** Red / green / **refactor**.
4. **End green**, every EX and VT discharged, status table `done`, Harvest
   updated **in place** beside the other two.

**If expanding the phase shows the plan is wrong, go back to plan** — or to
design. Do not repair it quietly in the sheet. `plan.md:102-142` is S-1..S-9.

## What PHASE-03 is

A canonical view becomes blocks of drawn fields with everything else reported,
and there is one pure place a widget's state becomes a submitted value.

**This is the pure-Rust phase.** No window, no widget, no platform, no clock, no
async runtime. `draft.rs` is new and names no Slint type. It is placed third
because `Prepared` cannot hold a `Draft` that does not exist — and it is
independent of PHASE-02, which is why almost nothing travels into it from there.

You carry AC-2's rule half and AC-8's R-57 half. The two facts PHASE-02 left
behind — `wiring.rs:62-70`'s unscoped query, and the viewport a shown window
reads through — are **PHASE-04's, not yours**. You will not show a window.

## The five things most likely to go wrong

- **`resolve` fails the gate, not review.** No new identifier anywhere in
  `crates/goad/src` may contain the word — no `resolve_field`, no
  `resolve_draft`, no `resolved`. `crates/goad-boundary/tests/checks/structure.rs:306-314`
  matches by identifier word over production lines. This is the cheapest
  possible way to lose an hour at EX-8.
- **The domain-vocabulary invariant is yours to hold, and it is not ADR-001's.**
  `CLAUDE.md`: no domain vocabulary — habit, streak, journal, goal, site — in
  host types or module names, and a boundary test greps for it. EX-6 states the
  live case: `FieldBlock` and `heading` are **layout, not a concept** (R-6). You
  are naming several new types this phase. Name them for what they are on the
  screen, never for what a backend might mean by them.
- **`Draft`'s shape is three deliberate absences, and each has a reason.** Keyed
  by `(option id, field id)` over a `Vec`, **not** a `BTreeMap` — `OptionId`
  carries no `Ord` and deriving one is S-4's specific temptation. **No way to
  enumerate what it holds** — that absence is what makes PHASE-04's walk a
  property of the type rather than a convention. **No `PartialEq`** — over a
  `Vec` it is insertion-order sensitive; assert through `state_of`. If you find
  yourself adding any of the three because a test wants it, that is the test to
  rewrite, not the type.
- **VA-2 permits exactly one existing assertion to change, and it is named.**
  `mapper.rs:156-194`'s `an_option_with_fields_is_reported_undrawn_by_id_and_count`,
  because the variant it names is gone. **Replaced by VT-4, not adjusted.**
  Anything else is S-2 and you stop.
- **VA-1's test is a compile failure.** Add a second `Edited` variant without a
  `submitted` arm, confirm it fails to build, revert, and record that you did it.
  A total match asserted only by reading the code is not asserted.

## What PHASES 01 and 02 taught, and it applies to you

Both are in the Harvest in full. Four that bear directly on this phase:

- **An enumerated list of live homes is a floor, and the fix creates homes of its
  own.** EX-3 moves a doc-comment naming `OptionFields`; EX-5 replaces its
  diagnostic arm. Find every live home of the claim you are changing, and know
  that a search for the repaired wording matches none of the ones that matter.
  `docs/memory/a-repair-sweep-misses-the-binding-site.md`.
- **Run the negative control on every test that arrives green.** PHASE-02 had two
  that passed with the thing they verify deleted — one was rewritten, one was
  deleted rather than shipped as a green proxy. Your VT-1 and VT-3 are the
  likely candidates: assert values a default cannot supply.
- **If a criterion's stated warrant turns out false, say so — do not reconcile
  it quietly.** PHASE-02's F-3 is the model. The change can be right and its
  stated reason wrong at the same time, and both belong in the record.
- **Derive counts from the instrument; never reconcile them to a total.**

Also in force: `docs/memory/cite-requirements-not-finding-ids.md`. A comment in
`src/` or `tests/` cites `SPEC-001/R-57`, a spec section, or an ADR — never a
slice-local `F-N` or `D-N`.

## Not yours

- **Canon.** `docs/specs/`, `docs/policy/`, `docs/adr/` are untouched by every
  phase in this slice. In particular: reading the `group` hint makes SPEC-001
  §7's R-18 row stale, and **`canon-delta.md` already carries that** — do not
  write it, do not amend the spec, do not treat it as an open question. Promotion
  is audit's.
- **`crates/goad-semantics/`.** Nothing there changes (S-4). Everything you need
  is already `pub` and was verified accessor by accessor; id constructors are
  `pub(super)`, so you clone an id and cannot mint one.
- **PHASE-04 onward.** `reception.rs`'s retained value, the controller, the
  window wiring. `submitted` is `pub(crate)` *so that* 04 can call it — that is
  not an invitation to call it now.
- **The look, and the wire.** 06's and 05's.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md` — §Phase plan, §Execute, §Where it goes.
2. `notes.md` — Harvest first, then the two existing sheets.
3. `plan.md` — Overview, S-1..S-9, and PHASE-03 whole (`:457-578`).
4. `design.md` §5.2's *The mapper* and *The draft* (`:469-557`) — `Edited`,
   `Draft` and `submitted` as given; §5.3 for ownership; §5.5 for I-1 and I-2.
5. `canon-delta.md` — R-57 is the rule `submitted` applies; the R-18 amendment is
   context you must not act on.
6. `slice-007.md` §Scope, and AC-2, AC-3, AC-8.
7. The code: `crates/goad/src/view_model.rs` (`:29-41` `body_is_degraded`,
   `:70` the `OptionFields` variant, `:145` the site that pushes it — the compiler finds that one, the doc-comment at `:29` it does not), `diagnostics.rs:191-199`, `lib.rs`,
   `tests/renderer/mapper.rs:156-194`, and `Undrawn::ContentForm` — the pattern
   `FieldForm` mirrors rather than reinvents.

## Environment and the gate

Nix devshell (`nix develop`, or direnv). **Run unjailed.** If `nix develop` is
involved, use the bare git-input flake reference: a `path:` ref breaks on the
demo socket (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).

`just check` is the gate; EX-8 is that check. Note that it is wider than
`cargo test`: it runs `cargo test -p goad-semantics` as its own stratum-1 build,
the boundary checks that will catch `resolve`, clippy at `-D warnings`, and a
format check.

**One writer per worktree.** Do not spawn a second agent that writes, and do not
rewrite history. **No `git stash`, ever.** Do not commit — git is the user's;
ask through the orchestrator if you want a checkpoint.

## When you are done

- Every EX and VT in PHASE-03 discharged; `just check` exit 0.
- `notes.md`: status table `done`, the sheet current — kept as you went — and the
  Harvest updated in place.
- Tell the orchestrator what you did, what you found in passing, and what
  PHASE-04 should know. 04 is the phase that joins your pure types to the
  window, so anything you learned about `Draft`'s shape travels.

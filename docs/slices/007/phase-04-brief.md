# Hand-over brief — Slice 007, PHASE-04

Hand this to a fresh agent. It is the statement of what was asked. It is not
canon, it does not override `plan.md` or `design.md`, and where it seems to,
it is wrong.

**No user gate is outstanding.** Your entry condition is PHASE-02's and
PHASE-03's exit, not a person's signature — check it, do not assume it.

---

## Where the slice stands

**Stage: execute, PHASE-04 of six. Tier 2.** Halfway, and this is the big one.

```
9447973  design resolved, plan accepted
fd4b162  PHASE-01  the notice gets an owner      506 tests
36c13ad  PHASE-02  the window draws a form       511 tests
246ed93  PHASE-03  the mapper and the draft      525 tests
```

The tree is clean at `246ed93` and the gate was green there, verified
independently of the agents that wrote it. Read `notes.md`'s **Harvest** before
anything else — three phases of hard-won detail on one screen — then the three
existing sheets as the worked shape yours should take.

`canon-delta.md` is **draft canon**: R-57, R-58, and an amendment to SPEC-001
§7's R-18 row. Cite it exactly as you would SPEC-001. **Do not edit
`docs/specs/`** — promotion is audit's.

## Your job, in order

1. **Phase-plan, and do the re-derivation twice.** `plan.md`'s Overview singles
   your phase out: it grew from seven exit criteria to ten under repair and
   restates more claims in prose than any other phase, so **re-derive your
   criterion ids from `plan.md` and then check the derivation again** rather than
   trusting either your copy or your first pass. The rule is at `plan.md:35-45`;
   `review-plan.md` F-7 is why it exists — three stale citations came out of a
   single renumber in the document whose whole subject is that class of defect.
2. **Verify EN-1**: PHASE-02's *and* PHASE-03's exit criteria discharged,
   `just check` exit 0. Keep the transcript.
3. **Execute.** Red / green / **refactor**.
4. **End green**, every EX and VT discharged, status table `done`, Harvest
   updated **in place** beside the other three.

**If expanding the phase shows the plan is wrong, go back to plan** — or to
design. `plan.md:102-142` is S-1..S-9.

## What PHASE-04 is

The phase where the pure types meet the window. An edit travels from a checkbox
to retained state, the screen is written back from that state every present, and
`answer()` submits a value for every drawn field of the answered option and none
for any other.

**The structural claim is the whole point**, and `plan.md`'s implementer notes
put it first for a reason: `answer()` walks what was **drawn** — the answered
option's blocks — not the draft. Three properties fall out with no check
anywhere: a value for every drawn field, no value for an undrawn one, and a
stale draft key that cannot reach the wire. With `answer()`'s existing
`SupersededView` refusal, every submitted key provably came from a field the
currently-retained view declared. Write it any other way and R-58 becomes a check
someone has to remember.

You carry **AC-8's R-58 half** (VT-3) and **AC-2's middle link** (VT-7).

## What you inherit that is not in your plan entry

Three facts earlier phases paid for. The first two are PHASE-02's and were
recorded specifically for you.

- **F-4 — `wiring.rs:62-70` is an unscoped description query, and this is the
  phase it breaks in.** `accessible_enabled_of` selects a control by
  `accessible_description` alone, with no type filter — `element_described`
  before PHASE-02's EX-7 fixed it. It is correct today only because no option in
  that file carries blocks. The moment one does, the field container answers to
  the same description and the helper reads `accessible_enabled` off a
  `groupbox`, which declares none: `None`, from a different cause. **Your VT-5
  and VT-1 are what put fields in that file.** PHASE-02's `within_option` /
  `field_described` in `tree.rs` are the shape to copy.
- **A test reading a control off a *shown* window reads through a viewport.**
  Today that viewport fits one `material` control. You verify at the window over
  a form of several checkboxes, so you will meet this. The precedent is
  `with_room_for_every_control` in `wiring.rs`, and F-6 records the measurement.
  **F-6 is a finding about the product and is PHASE-06's** — do not treat a
  declared viewport in a fixture as having fixed anything real.
- **`submitted`'s `#[cfg_attr(not(test), expect(dead_code, …))]` self-clears.**
  The moment `controller.rs` calls it, `unfulfilled_lint_expectations` fails the
  gate. Removing that attribute is part of EX-3, and the gate will tell you.

## The five things most likely to go wrong

- **`FieldBlock` names two types in `glass.rs`, deliberately.** `design.md` §5.2
  gives the Slint struct and the `view_model.rs` struct the same name, and
  `glass.rs` is the one file holding both. Generated types keep their bare names
  as `OptionRow` does; the mapper's is path-qualified `view_model::FieldBlock` at
  its use sites. **Do not rename either.** The design names both, and a rename is
  a design change.
- **Four criteria have no instrument in the gate at all.** EX-7 (the draft does
  not enter `Presentation`, I-4), S-6, S-9 (`present` stays total, I-5), and
  VA-2 (`Draft` still exposes no enumeration). A green gate is not evidence for
  any of them — `cargo test -p goad-semantics` and the other ADR-001 instruments
  stop short of stratum 3. They are stated so review has something to match
  against; discharge them by reading, and say in the sheet that you did.
- **`answer()` must walk the blocks, never the draft (D6).** `Draft` has no
  enumeration precisely so this is a property of the type rather than a
  convention. If you find yourself wanting an iterator over the draft, that is
  S-1 and you stop — the absence is load-bearing and PHASE-03 recorded it as
  such.
- **A domain word inside a string literal fails the gate.** PHASE-03 measured
  this: the vocabulary scan cuts comments but **not** string literals, and `site`
  is one of the seven words — so `reason = "…the call site arrives"` breaches
  while the identical sentence in the doc-comment above does not. You are writing
  `#[expect]` reasons, test names and assertion messages all phase.
- **VA-1 excludes `reception.rs` and nothing else.** EX-9 extends one case there
  by design — S-2's third allowance, keeping the claim the test's name makes.
  `table.rs`, `wiring.rs`, `scheduling.rs` and `ingress.rs` must show no changed
  assertion and no changed fixture; `Prepared` gaining a defaulted field must be
  invisible to every existing case. If it is not, that is S-2 and you stop.

## What the earlier phases taught, and it applies to you

All of it is in the Harvest. Four that bear directly:

- **Run the negative control on every test that arrives green** — and ask first
  which kind of claim the test makes. Where the claim **is** the default, no
  control that breaks the mechanism can reach it (PHASE-03's corollary to
  PHASE-02's D-5). For a decided fork, the sharpest control is implementing the
  **rejected reading** and watching exactly one case go red.
- **An enumerated list of live homes is a floor, and the fix creates homes of
  its own.** EX-5 moves `install.rs`'s "six installations"; EX-8 adds a variant
  whose siblings' docs state a shape. Find every live home; a search for the
  repaired wording matches none of the ones that matter.
- **`design.md` §8 is a checklist to re-read at the *end* of a phase**, not only
  at the start. PHASE-03's first cut of the mapper wrote the exact type §8/R-6
  names as the thing not to write, and no instrument reached it.
- **If a criterion's stated warrant turns out false, say so** — do not reconcile
  it quietly. The change can be right and its stated reason wrong at once.

Also in force: `docs/memory/cite-requirements-not-finding-ids.md`. A comment in
`src/` or `tests/` cites `SPEC-001/R-58`, a spec section, or an ADR — never a
slice-local `F-N` or `D-N`.

## Not yours

- **Canon.** `docs/specs/`, `docs/policy/`, `docs/adr/`. `canon-delta.md` carries
  R-57, R-58 and the R-18 amendment; promotion is audit's, with user endorsement.
- **`crates/goad-semantics/`.** Nothing there changes (S-4).
- **The wire, and the look.** PHASE-05 reads the backend's log; PHASE-06 is the
  demo and AC-10. `examples/typescript/backend.ts` is a deliberate non-surface.
- **SPEC-002/OQ-4.** Deferred by D13: a scheduled firing can still replace a
  half-filled form, and that is decided behaviour, not a defect to fix in
  passing.
- **A guard against two edits racing an exchange.** `enabled: !root.busy` already
  makes the window inert exactly while `serve` is not reading commands.
  SPEC-002/R-9 is untouched; do not add one.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md` — §Phase plan, §Execute, §Where it goes.
2. `notes.md` — Harvest first, then the three existing sheets.
3. `plan.md` — Overview, S-1..S-9, and PHASE-04 whole (`:579-712`). Skim 05 and
   06 so you know what you are not doing.
4. `design.md` §5.2 (the controller, the command, the diagnostic lines), §5.3
   (state and ownership — what `Prepared` holds and why a draft and its view are
   one value), §5.4 (lifecycle, and the mechanism §5.4's closing section rests
   on), §5.5 (I-4, I-5, and the edge table — including the row added this week
   for a field both undrawn and badly grouped).
5. `canon-delta.md` — R-58 is what VT-3 is the vehicle for.
6. `slice-007.md` §Scope, and AC-2, AC-5, AC-6, AC-8.
7. The code: `controller.rs:216` (`answer`'s `values`), `:669-684` (the single
   refusal site), `reception.rs` (`receive`, `Prepared`), `wire.rs`
   (`Command::Choose`, the shape `Edit` follows), `install.rs` (the
   one-clone-per-callback convention), `glass.rs`'s `option_rows`,
   `diagnostics.rs:47-54` and `:148-163` (`Refused` and its exhaustive match),
   `tests/renderer/wiring.rs`'s `mod interaction`, and `tree.rs`'s
   `within_option` / `field_described`.

## Environment and the gate

Nix devshell (`nix develop`, or direnv). **Run unjailed.** If `nix develop` is
involved, use the bare git-input flake reference: a `path:` ref breaks on the
demo socket (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).

`just check` is the gate; EX-10 is that check. It is wider than `cargo test`:
a stratum-1 build, the boundary checks including the vocabulary scan, clippy at
`-D warnings`, and a format check.

**One writer per worktree.** Do not spawn a second agent that writes, and do not
rewrite history. **No `git stash`, ever.** Do not commit — git is the user's;
ask through the orchestrator if you want a checkpoint.

## When you are done

- Every EX and VT discharged; the four uninstrumented criteria discharged by
  reading, and said so; `just check` exit 0.
- `notes.md`: status table `done`, the sheet current — kept as you went — and the
  Harvest updated in place.
- Tell the orchestrator what you did, what you found in passing, and what
  PHASE-05 should know. 05 is the first phase that reads the wire, and it
  verifies at the backend's own log rather than at the window.

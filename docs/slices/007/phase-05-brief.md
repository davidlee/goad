# Hand-over brief — Slice 007, PHASE-05

Hand this to a fresh agent. It is the statement of what was asked. It is not
canon, it does not override `plan.md` or `design.md`, and where it seems to,
it is wrong.

**No user gate is outstanding.** Your entry condition is PHASE-04's exit — check
it, do not assume it.

---

## Where the slice stands

**Stage: execute, PHASE-05 of six. Tier 2.**

```
9447973  design resolved, plan accepted
fd4b162  PHASE-01  the notice gets an owner              506 tests
36c13ad  PHASE-02  the window draws a form               511 tests
246ed93  PHASE-03  the mapper and the draft              525 tests
69c617b  PHASE-04  the draft is retained, and answered   531 tests
```

The tree is clean at `69c617b` and the gate was green there, verified
independently of the agents that wrote it. Read `notes.md`'s **Harvest** first —
four phases of hard-won detail on one screen — then the four existing sheets.

`canon-delta.md` is **draft canon**: R-57, R-58, and an amendment to SPEC-001
§7's R-18 row. Cite it as you would SPEC-001; **do not edit `docs/specs/`**.

## Your job, in order

1. **Phase-plan.** Expand `plan.md`'s PHASE-05 entry into a sheet in `notes.md`.
   **Re-derive your criterion ids from `plan.md`** (`plan.md:35-45`).
2. **Verify EN-1**: PHASE-04's exit criteria discharged, `just check` exit 0.
3. **Execute.** Red / green / **refactor**.
4. **End green**, status table `done`, Harvest updated **in place**.

`plan.md:102-142` is S-1..S-9. **S-7 is yours in a way it has not been any other
phase's** — see below.

## What PHASE-05 is

**The first phase that reads the wire.** Everything before it verified at the
screen, at a pure function, or at the row model. You verify at the backend's own
invocation log: what the person ticked is shown to *leave the host* as JSON,
under the option they pressed, surviving a present that changed nothing.

You write one new file, `fields.rs`, and lift one helper. **No `src/` file is
yours.** If you find yourself wanting to change production code, that is S-1 and
you stop — the mechanism is finished and this phase is the proof it works.

You carry **AC-1** (VT-1), **AC-4** (VT-2), **AC-5** (VT-3) and **AC-3's
remaining half** (VT-4).

## The thing this phase exists for

`plan.md` says it outright and it is worth reading twice: **a test that reads the
draft through `Controller` and never reads an invocation log would pass with
`answer()` walking the draft's keys — which is D6, the defect most worth
catching.** Every case in this file either reads the invocation log or asserts
something about the screen (EX-4, S-7). That is not a stylistic preference; it is
the whole reason the phase is separate from PHASE-04.

`docs/memory/a-green-test-can-assert-a-proxy.md` is the scar. Four phases have
now each found a test that passed for the wrong reason — including two that
passed with the thing they verify deleted. Assume yours will too until you have
made each one fail for its own reason (VA-1).

## The four things most likely to go wrong

- **VT-3 is a conjunction, and every other case in the file is a disjunction.**
  EX-4's rule is "reads the log *or* asserts the screen"; VT-3 must do **both**,
  and neither half substitutes. The wire half alone stays green while the screen
  is wrong, because the wire value is built from the draft and no present writes
  the draft. The screen half alone stays green if the draft is dropped on
  `Retained`. Writing it as a disjunction is the one place the file's own rule
  will mislead you.
- **VT-2's shared field id is the point, not the fixture.** Two options carrying
  the same field id. With an unscoped selector the case is green both where the
  design is right *and* where the draft key's `option` half is ignored — so the
  case proves nothing until VA-1's injection is run. **That injection is the
  load-bearing one**: ignore the option half, and VT-2 must go red while VT-1
  stays green. If both go red, your VT-1 is also resting on the scoping.
- **The invocation log proves an exchange *began*, not that it was absorbed.**
  Verified in the script rather than taken from the plan: `request="$(cat)"` at
  line 19, the log append at 26, the answer at 32, and the host's fold later
  still. Where a case needs the exchange folded, poll for an observable the
  **production glass** wrote; `scheduling.rs`'s next-check-line helper is the
  precedent. Use `harness::until` with `TIMEOUT`, never a fixed sleep.
- **`scheduling.rs:127-133` states that backwards** — it says the log is written
  *before* the script reads the request, and the script reads first. Its
  conclusion still holds, for the reason above. **Do not fix it**; it is not your
  surface. Record it in `notes.md` Findings for the audit, as `plan.md` directs.

## One criterion is already partly discharged, and better than it asks

**EX-2 says `harness.rs:1-8`'s module doc "enumerates its consumers" and must
move with the lift. It no longer does.** PHASE-04 lifted the scoped field queries
into `harness.rs` and replaced the closed list with a rule — *the rule is two or
more, and the set that satisfies it is not fixed … which is why nothing here
names a closed list of case files.* Read it before you write anything.

So: lift `logging_scripted` (it is still at `scheduling.rs:101`), follow its
callers' imports, and **do not re-introduce an enumeration** to satisfy the
criterion's letter. The criterion's intent — that the doc not go stale when a
fourth consumer arrives — is already met by a better mechanism than the one it
names. Say so in the sheet rather than reconciling it quietly; that is the third
time this slice a criterion's stated warrant has not matched the code, and each
one has been worth recording.

## What the earlier phases taught, and it applies to you

All of it is in the Harvest. Five that bear directly:

- **Run a control on every case that arrives green**, and ask first which kind of
  claim it makes. Where the claim **is** the default, no control that breaks the
  mechanism can reach it. For a decided fork, implement the **rejected reading**
  and watch exactly one case go red.
- **A control on the fixture is not a control on the code.** PHASE-04's C-6:
  removing a declared viewport makes the form vanish from the query, and that
  says nothing about the product. Conflating them is how a product defect gets
  closed by a green test.
- **A test whose name is its claim is trusted instead of counted.** If you write
  a case whose name quantifies — *every*, *only*, *exactly* — make the name true
  and keep the check one line an auditor can run.
- **A domain word inside a string literal fails the gate.** The vocabulary scan
  cuts comments but not string literals. You are writing test names and assertion
  messages all phase.
- **`design.md` §8 is a checklist to re-read at the end of a phase**, not only at
  the start.

Also in force: `docs/memory/cite-requirements-not-finding-ids.md`.

## Not yours

- **Any `src/` file.** The mechanism is done. Wanting to change one is S-1.
- **Canon**, and `canon-delta.md`'s two known staleness points — it still
  describes `controller.rs:216` as sending an empty map. Audit's at promotion.
- **`scheduling.rs`'s backwards comment**, and `design.md` §9's disagreement with
  itself (PHASE-04's F-2). Both audit's.
- **PHASE-06.** The demo, the look, AC-7 and AC-10, and F-6 — the product clipping
  its own second option at its preferred size. A declared viewport in one of your
  fixtures fixes nothing real, and must not be written as though it did.
- **`examples/typescript/backend.ts`** — a deliberate non-surface.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md` — §Phase plan, §Execute, §Where it goes.
2. `notes.md` — Harvest first, then the four sheets.
3. `plan.md` — Overview, S-1..S-9, PHASE-05 whole (`:713-806`), and skim 06.
4. `design.md` §9 (validation — what each tier proves, and the proxy rule),
   §5.4 for the `Retained` fold VT-3 turns on.
5. `slice-007.md` — AC-1, AC-3, AC-4, AC-5.
6. The code: `tests/renderer/scheduling.rs:101` (`logging_scripted`) and its
   `request_kind` reader; `harness.rs` entire, module doc first;
   `tests/renderer/wiring.rs`'s `mod editing` — the only module whose fixtures
   carry fields, and the precedent for driving a checkbox;
   `tests/backends/logs-the-request-then-answers.sh` (note: repo root, not under
   `crates/`); `tests/renderer/main.rs`'s roll-call.

## Environment and the gate

Nix devshell (`nix develop`, or direnv). **Run unjailed.** If `nix develop` is
involved, use the bare git-input flake reference (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).

`just check` is the gate; EX-5 is that check. These cases drive a real child
process, so they are the slowest in the suite — budget for it rather than
reaching for a shortcut that stops driving `serve`.

**One writer per worktree.** No second writing agent, no history rewrite, **no
`git stash`, ever.** Do not commit — git is the user's; ask through the
orchestrator if you want a checkpoint.

## When you are done

- Every EX and VT discharged, VA-1 run case by case with each injection named and
  reverted; `just check` exit 0.
- `notes.md`: status `done`, sheet current, Harvest updated in place.
- Tell the orchestrator what you did, what you found in passing, and what
  PHASE-06 should know — 06 is a person running the software, and it is the phase
  that closes the slice.

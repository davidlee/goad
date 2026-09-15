# Hand-over brief — Slice 007, PHASE-06

Hand this to a fresh agent. It is the statement of what was asked. It is not
canon, it does not override `plan.md` or `design.md`, and where it seems to,
it is wrong.

---

## Where the slice stands

**Stage: execute, PHASE-06 of six — the last one. Tier 2.**

```
9447973  design resolved, plan accepted
fd4b162  PHASE-01  the notice gets an owner              506 tests
36c13ad  PHASE-02  the window draws a form               511 tests
246ed93  PHASE-03  the mapper and the draft              525 tests
69c617b  PHASE-04  the draft is retained, and answered   531 tests
10cfd27  PHASE-05  the form on the wire                  535 tests
```

The tree is clean at `10cfd27`, gate green there, verified independently of the
agents that wrote it. Read `notes.md`'s **Harvest** first — five phases on one
screen — then the five sheets.

## What makes this phase different from the five before it

**Its two headline criteria cannot be discharged by an agent.** VH-1 and VH-2
are a *person* running the software and saying what they see.
`docs/AGENTS.md` §Tiers: a slice does not close until a person has run it and
seen the new behaviour, and **a green gate is explicitly not that evidence** —
slices 001–003 all closed green on a binary that could not open a window.

So your job is to make the demo worth running, run the parts you can, and then
**stop and hand the software to the user**. Do not write VH-1 or VH-2's evidence
on their behalf, and do not infer it from a passing test.

**EN-2 is a display, and this phase runs outside the jail.** If you cannot reach
one, say so rather than substituting something you can reach.

## Your job, in order

1. **Phase-plan.** Expand `plan.md`'s PHASE-06 entry into a sheet in `notes.md`.
   Re-derive the criterion ids from `plan.md` (`plan.md:35-45`) — **and then read
   what each warrant cites.** That second half is PHASE-05's closing advice and
   it caught three stale warrants across this slice; it costs one task.
2. **Verify EN-1**: PHASE-05's exit criteria discharged, `just check` exit 0.
3. **EX-1 … EX-3 — the demo backend.** `examples/shell/backend.sh` sends a form
   and records what comes back.
4. **Hand it to the user for VH-1 and VH-2.** Then action AC-10's feedback within
   its bound, and record the rest verbatim.
5. **EX-5, EX-6 — `audit.md` Evidence and the Harvest.**
6. **End green**, status `done`.

## The demo backend — the part that is actually yours

- **EX-1 — the form must be protocol-shaped, not renderer-shaped.** At least two
  `group` values, so a heading change is visible, **and at least one field of a
  kind this renderer does not draw.** That last clause is the whole point: it is
  what makes the artefact a backend author copies a *protocol* form rather than
  this renderer's subset, and it is R-55's "or produce the effect of" clause,
  which **no other artefact in this slice discharges**.
- **EX-2 — the script must record the submitted `values` where a person can read
  them.** Today it discards them and answers `view: null`, so AC-7's *"the record
  shows every answer"* has no vehicle at all without this. Append to a file, or
  write to stderr, which the host reports on the diagnostics surface.
- **EX-3 — keep the script's existing discipline.** No value the host carries
  opaquely reaches its control flow, and any watcher-authored string it
  interpolates is escaped. Its header states the rule; a new branch must not
  quietly break it.
- It is `bash`, reads one JSON document on stdin, writes one on stdout, and is
  deliberately readable without a parser. **Match its style — it is
  documentation as much as it is a fixture.**
- `just demo` runs on `examples/demo.toml`, whose socket lives in the checkout.
  If `nix develop` is involved, use the bare git-input flake reference: a `path:`
  ref breaks on the demo socket
  (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).

## What you may change in the markup, and what you may not

**AC-10's bound (S-8): the block container and its separator, and the heading's
own treatment. Nothing else.** Typography, window sizing, the idle surface and
the look of the controls are 008's.

**EX-4 is the rule that makes the bound survive contact:** feedback is split *at
the moment it is given*. What drawing fields forces is actioned here; everything
else is recorded **verbatim and not actioned** in `notes.md`, and becomes 008's
brief. The bound is on what may be **changed**, never on what may be **said** —
so capture everything the user says, action only what falls inside it, and do
not argue the user out of an observation because it lands outside the bound.

**The keyboard-focus cost is known and accepted**: every present rebuilds the
rows, so the focus ring drops once per tick and the form is filled by mouse.
Not a defect to repair here; it is already a Follow-up.

## What you inherit, and F-6 is the one that matters

**F-6 — the product clips its own second option at its preferred size.** Under
`material`, a shown window at its preferred size puts the second option outside
the `Flickable`'s rect. Three fixtures in two files now declare a viewport so
their tests can see the form; **none of that fixed anything real**, and three
agents each wrote a sentence in their fixture saying so. It has been measured
three times, most recently by PHASE-05 (removing `fields.rs`'s viewport fails
all four cases with `no control described "read" under "morning"`).

**VH-1 and VH-2 are the only things that observe it.** If the demo window opens
and you cannot reach the second option, that is F-6 arriving in person — not a
new defect, and not one to fix inside AC-10's bound unless what you change is
the block container. `design.md`'s diagnosis: a window that has never been sized
meeting a taller control. 008 owns the repair.

Also inherited: **name any scripted fixture you add with a prefix.**
`scheduling.rs:415` and `wiring.rs:1570` both use `scripted("vt8", …)` and share
one invocation log in one pid — PHASE-05 measured one failure in six runs.

## What is audit's, not yours

Leave these **reachable**, do not close over them:

- **`canon-delta.md`'s promotion** — R-57 and R-58 into SPEC-001 with a §7 row
  each, plus the amendment to the **existing R-18 row**, which claims `hints` is
  read in `src/` only by `normalize.rs` and that the renderer "does not exist
  yet". PHASE-03 falsified both. Promotion needs explicit user endorsement, and
  **a slice does not close holding an unpromoted draft** — it lands or it is
  abandoned in writing.
  - **R-57's drafted text carries two `and`s** — *"…`choice` … as a JSON string,
    **and** `datetime` an RFC 3339 `date-time` string"* — a copy-edit worth
    making before it is promoted verbatim.
  - `canon-delta.md` still describes `controller.rs:216` as sending an empty map.
    It does not any more (PHASE-04 F-4).
- **The plan's per-phase Surfaces lines** — short in four phases (`app.slint`,
  `wiring.rs`, `draft.rs`, `tree.rs`). One finding about the lines generally,
  not four.
- **`design.md` §9 disagreeing with itself** about whether every field test reads
  the wire or the screen (PHASE-04 F-2), and **§9/AC-1 and §8/R-5 citing
  `scheduling.rs:95-125`** for a helper now in `harness.rs` (PHASE-05 F-4).
- **`docs/memory/cite-requirements-not-finding-ids.md` is unenforced** across at
  least eight files, including two lines PHASE-04 wrote. Amend the memory or
  scope a sweep — a decision, not a fix.
- **`accessible_enabled_of`'s one-line collapse** over the lifted
  `element_described` (PHASE-05 F-3, which carries the code verbatim), and **the
  viewport sizer's three copies** (F-7, which needs a design decision because the
  three differ in value and in why).

## VA-1 — and the sentence not to write

`just check` exit 0, the vocabulary scan and the four ADR-001 instruments pass.
Then **the two things a green gate does not say**: no host type or module is
named for grouping — `group` is not on the scanned word list and never will be —
and stratum 3's purity, `view_model.rs` staying pure and `Draft` never entering
`Presentation`, is held by **review alone** (POL-001 §Verification, `design.md`
§3). **Do not write a criterion, or a sentence in `audit.md`, that implies an
instrument checks either.**

## What the five phases taught

In the Harvest in full. The five that bear on you:

- **Read what a warrant cites, not the count it asserts.** Three stale warrants,
  three repairs, two vacuous tests not written.
- **A control on the fixture is not a control on the code.** Directly relevant:
  a viewport in a fixture says nothing about the window.
- **A test whose name is its claim is trusted instead of counted.**
- **`design.md` §8 is a checklist to re-read at the end of a phase.**
- **Before closing, diff the record against `git status`.** PHASE-05's D-3
  recorded the opposite of what its own diff showed, and nothing checks a sheet
  against the code it describes.

## Environment and the gate

Nix devshell. **Run unjailed — EN-2 requires a display.** `just check` is the
gate; EX-7 is that check. **One writer per worktree.** No history rewrite, **no
`git stash`, ever.** Do not commit — git is the user's.

## When you are done

- EX-1…EX-4 and EX-6…EX-7 discharged; VH-1 and VH-2 carried out **by the user**
  and written into `audit.md` Evidence in their own account.
- AC-10 feedback split: actioned inside the bound, recorded verbatim outside it.
- `notes.md`: status `done`, Harvest current.
- Tell the orchestrator what the user saw, what you changed, what you recorded
  for 008, and what audit must still reach.

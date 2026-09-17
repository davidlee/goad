# Notes — Slice 009

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Handover — design review, after round 2

Written 2026-09-17, mid-review, so a fresh agent can pick this up without
re-deriving it. Delete once the design closes.

### Where the slice is

Design drafted and accepted by the user (sections 1-10). **The review loop is
open**, two rounds run. Plan not started. Nothing is committed — HEAD is
`11f3366`.

### What holds the truth

| file | state |
|---|---|
| `review-design.md` | **the ledger, and the live one.** F-1 … F-29. F-1 … F-18 carry a disposition and a terminal round-2 outcome; **F-19 … F-29 carry neither** and are the open work |
| `design-log.md` | D-1 … D-5 scoping, D-6 … D-10 design, **D-11 … D-14 round 1's user decisions**. Append-only: never rewrite an entry, supersede it |
| `design.md` | current truth, repaired once against round 1. Round 2 contested five of those repairs |
| `canon-delta.md` | CD-1, CD-2. CD-2 was rewritten at round 1 and is contested again at F-10 |
| `research.md` | Thread 4's closing paragraph was corrected at F-12 |

### The review so far, in one line each

Round 1 (Codex, `gpt-5.6-sol`, via the `codex` MCP) raised F-1 … F-12; the
responder added F-13 … F-18 as a declared second raiser. Round 2 continued the
**same Codex thread**, `01a0ad06-ba40-7821-8d33-016b8cc4b0ee`, which is the
right choice for setting outcomes — that is the raiser's job — and it also
raised F-19 … F-27. The responder added F-28 and F-29 against its own repairs.

Round 2's shape is the thing to carry forward: **eleven of eighteen repairs
rewrote a section, and five of them were wrong.** A third round is not
optional, and it should attack the round-2 repairs the same way. Continue the
same Codex thread for outcomes; consider a second, genuinely fresh reviewer if
round 3 comes back quiet, because a thread that has agreed twice is cheap to
agree with a third time.

### Three facts verified by hand, because they overturn things

1. **Popups *are* reachable under `init_no_event_loop`.** `i-slint-backend-testing`'s
   own `test_popups` proves it, and `ElementQuery::from_root` traverses
   `active_popups` (`search_api.rs:296,309,343`). F-13 was wrong and is
   withdrawn; §9's driver table was built on it and F-25 is the consequence.
   The lesson is in the finding: absence of a case in this repository is not
   absence of a capability.
2. **The command channel is capacity 1** (`main.rs:86`), and `serve` shares the
   UI thread through `spawn_local`. A Slint callback has no await, so `serve`
   cannot drain between two sends inside one callback: the **second `try_send`
   of any flush always fails**. F-6 turns on this, and it was already true of
   the pre-repair design, where one edit plus `Choose` was two sends.
3. **`reception.rs:753` does not exist** — the file is 103 lines. F-10's
   contest cites it and is nonetheless upheld on two sites the round-1 inventory
   missed: `draft.rs:82` and `view_model.rs:31`, both doc comments naming
   `Undrawn::FieldForm` as the sixth-kind site.

### What is owed, in order

1. ~~Disposition F-19 … F-29 and re-disposition the five contested.~~ **Done**,
   with the user, 2026-09-17. D-15 … D-17 in `design-log.md`. Every finding now
   carries a disposition whose **Response** states what changes and why — the
   ledger is the integrator's brief and needs nothing from the session that
   wrote it.
2. ~~Integrate, by a fresh agent.~~ **Done**, 2026-09-17, by a fresh agent —
   the user's judgement, and the evidence agreed: the round-1 responder was
   wrong about four of its own repairs (F-23, F-24, F-28, F-29). The integrator
   then found **four more defects, two of them blockers**, now raised as
   **F-30 … F-33**, which carry no disposition yet and are the open work.
   It also audited the ledger's own citations — see *Citations known bad* below.
3. **Disposition F-30 … F-33 with the user**, integrate, then **round 3**.
4. **Revise `slice-009.md`**: it still lists all five OQs as open and its
   §Governing canon does not mention `canon-delta.md`. `AGENTS.md` puts this
   after the findings are integrated. §Scope also now owes
   `crates/goad/Cargo.toml` (D-11's `jiff` feature).
5. **Re-ask the user for acceptance**, the design having changed substantially.
6. **Plan**, with a fresh agent.
7. **Delete `spike-fields/`** when the design closes, once its facts are in
   `docs/memory/` — `research.md` Thread 3 already carries them. Restore with
   `git checkout a698217 -- spike-fields`; `tests/split.rs` and
   `numeric_guard.rs` are untracked additions and are the evidence behind
   §5.2's two-channel model and its numeric guard.
8. `just check` has **not** been run: nothing under `crates/` was touched, so
   the gate's subject is unchanged. It is owed once repairs reach code.

### Citations known bad, and what that means

The round-2 integrator opened every `path:line` in the ledger. Three did not
check out, **all three written by the responder, not by Codex**:

- `review-design.md` F-10's re-disposition says two `wiring.rs` sites reach
  `Refused::UnknownField` through the undrawn field. Only `:1245` does; `:1304`
  asserts a submitted key list. The class is one site.
- F-23's Response cites `wire.rs:126` for the discarded `_returned`; it is at
  `:130`, in a `send` spanning `:127-133`. Substance correct, line off by four.
- The pre-repair §9 said the renderer tier drives widgets with
  `invoke_accessible_default_action` **and** `set_accessible_value`, citing
  `fields.rs:223-230`. `set_accessible_value` appears **nowhere** in this
  repository, and that line is `click()`, which uses only the default action.
  Both halves were false. This was the foundation of the withdrawn F-13, which
  was therefore wrong on two independent grounds.

Round 2's citations checked out, including the ones that overturned things.
The lesson for round 3 is narrow and worth holding: **the responder's own
citations were the unreliable ones.** Verify them first.

### Traps worth naming

- `design-log.md` is append-only. D-9's falsification of Thread 4's claim is
  wrong (F-12) and is corrected **by D-14 citing the finding**, not by editing
  D-9. Do the same for anything else that turns.
- The `§7` decision table is *current truth*, so a decision that turns is
  rewritten there under its original id — D7 and D14 already were. Ids are
  immutable; their content is not.
- One event-loop **arrangement**, one `[[test]]` target
  (`docs/memory/slint-testing-backend-initialises-once-per-process.md`). D-10
  assumed one target would carry everything and §9 outgrew that.
- Two 64-to-32-bit narrowings were found in one round (§8 R7). Treat any
  host↔markup conversion as guilty until checked.
- **A Slint `changed <property>` handler fires on a *change*, not on a write.**
  F-31 is that, and it is the second time this design has been caught by it.
  Any convergence argument that says "the host writes it, so the widget follows"
  is wrong unless the written value differs from what is there.
- **The guard's comparand has now been wrong twice** — `f32`-collapsing (F-19)
  and text-comparing (F-30). Both were reasoned; `numeric_guard.rs` is the only
  thing that has ever been measured here. Treat a third proposal the same way.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | pending / in progress / done / blocked | |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — <name>

**Objective:** <copied from plan.md>

**Reading list**
<!-- path:line references, the design sections that bind, prior art. -->

**Assumptions & STOP conditions**
<!-- What is being taken on faith, and the specific conditions under which the
     agent must stop and consult the user rather than improvise. -->

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [ ]

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** <yyyy-mm-dd> · <phase or stage> · <commit>

### Produced
<!-- What now exists: modules, contracts, docs. -->

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

# Review brief — Slice 007 design, confirmation pass

Hand this to a fresh agent. It is the statement of what was asked; it is not
canon and it is not a finding.

**This is not round 3.** Rounds 1 and 2 are complete. This is the confirmation
pass that closes three findings the raiser has not yet signed off. Scope is
three ids and nothing else. If you find yourself reviewing the design, you have
misread this brief.

---

## Where the ledger stands

`docs/slices/007/review-design.md`, thirty-one findings:

| | |
|---|---|
| **28** | `verified` |
| **2** | `contested` — F-29, F-30 |
| **1** | outcome unset — F-31 |
| **0** | blockers outstanding (F-4 and F-21 both verified) |

`Done` = every finding `verified` or `withdrawn`, and no blocker outstanding.
The acceptance gate is already met. What is missing is the last three cells of
the outcome column.

## Your job

**Set the outcome on F-29, F-30 and F-31.** They are one thread and they move
together: F-30's repair answers F-29's contest, and F-31's repair corrects one
word in F-30's. All three repairs are applied to `design.md`.

Raise a new finding **only** if a repair introduced a defect. Do not open new
ground; three rounds have covered it and *Depth of the round* records what each
reached.

## The thread, so you can see its shape

| | severity | what it found | size of the repair |
|---|---|---|---|
| F-29 | major | a dropped edit's rollback erases the notice explaining it | new design surface |
| F-30 | major | the F-29 repair named no data path, and contradicted two standing claims | one route, three sections |
| F-31 | minor | that repair called a `watch` value "not retained state" when retention is the mechanism | one heading, one paragraph |

Severity falling, repair shrinking. That is a thread converging. Hold it to that
standard: a fourth finding smaller than F-31 is convergence and should be
disposed and closed, not treated as a reason for another pass.

## What to check, as claims rather than an account

The responder's reasoning is in each finding's **Response**. Read it as a claim.

- **§5.1** now says `Frame` gains exactly one field and becomes *more* total for
  it, on the grounds that `controller.rs:92-93` already reads "Total: every
  property but `notice` is written from this, every time". Is that carve-out the
  defect, and does the sentence lose its "but" once the field exists?
- **§5.3's heading** is scoped to "the complete state retained **by
  `Controller`**". Check that scoping makes the inventory true as written rather
  than true by omission — and that `Controller` really does gain no field.
- **§5.3's `Notice` paragraph** now says the value is retained at the **edge**,
  that its retention is load-bearing (`true` must survive the `Full` callback,
  the next present, and further presents until a successful send writes `false`),
  and that P-2 therefore holds for `notice` as for everything else. F-31 said the
  accurate distinction is *outside `Controller`*, not *unretained*. Is it stated
  that way now, or has the old claim survived somewhere?
- **The route** is claimed to be `Cancel`'s edge for edge: constructed in `main`,
  cloned into `Wire` for synchronous setting from a Slint callback, passed to
  `serve` beside `cancel`, read at present time, handed to `frame(notice)`. Check
  every edge against `wire.rs:142-172`, `main.rs:85-119` and `serve`'s signature
  at `controller.rs:576-584`.
- **§5.4 and A-4** — confirm no sentence still describes the old clearing
  behaviour or the superseded "the controller reads it into the frame" shape.

## Do not re-raise

- **The three residues from round 2's brief** still stand: `research.md`'s two
  stale `glass.rs` citations (left deliberately), the open `accessible-role:
  list` / `accessible-item-count` announcement question, and D3 having no ADR.
- **`plan.md` is an unfilled template.** Known. It carries its own ledger and is
  not this pass's subject.
- **The `notice` ownership decision itself** — controller-side rather than
  `Wire`-side — was taken by the user on 2026-09-15 and is recorded in F-29's
  Response. Arguing the route is wrong is fair; re-arguing who should own it is
  settled.

## Driving codex

Rounds 2 and its verification passes ran through `codex` with **`gpt-5.6-sol`**
on one accumulating thread:

```
threadId: 01a0a290-bd7c-7730-8651-f5da8f905664
```

Load the tool with `ToolSearch("select:mcp__codex__codex,mcp__codex__codex-reply")`
and try `codex-reply` on that thread first — it holds the whole review and is by
far the cheapest way in. If the thread cannot be resumed from a new session,
cold-start with `mcp__codex__codex` and give it the reading order below; say in
the ledger that you did, because a cold reviewer and a warm one are not the same
witness.

Sandbox `workspace-write`, approval-policy `never`, cwd `/home/david/dev/goad`.

- **Codex writes into `review-design.md` directly.** It has done so all round.
- **Do not improve what codex says.** A wrong finding is withdrawn by the raiser
  at outcome time, not filtered out by whoever holds the pen. Disagree as the
  responder, in a separate turn, wearing the other hat.
- **Edit no file but `review-design.md`** when acting as raiser. Repairs to
  `design.md` are the responder's act, after the user confirms the disposition.

## Two operational traps, both hit in this session

- **Codex edits the findings table itself.** If you script an update to a row,
  assert on the row's *current* state, not the state you left it in. A python
  `assert` that fires after you have already mutated the string but before you
  write means **nothing is written** — silent no-op, easy to misread as success.
  Write first, verify after, and re-read the file rather than trusting your last
  edit.
- **`**Outcome:**` lines are not uniform.** F-13's carries trailing prose on the
  same line. A naive search for `"\n**Outcome:** contested\n"` from a finding's
  start will skip past it and land the text in a *later* finding. Anchor inserts
  on the finding heading and bound them by the next heading.

## Reading order

1. `CLAUDE.md`, then `docs/AGENTS.md`.
2. `docs/slices/007/review-design.md` — the Protocol section is binding; the
   *Outcome, and who sets it* note states who holds this column and why.
3. F-29, F-30, F-31 in full, then the sections of `design.md` they touch: §5.1,
   §5.3, §5.4, A-4, P-2.
4. `crates/goad/src/{controller,wire,glass,main}.rs` for the route.

`design-log.md` is read **last**. It is persuasive by construction; reading it
early is how a reviewer talks themselves into `aligned`.

## When you are done

- Every finding `verified` or `withdrawn`; state the tally.
- A line in *Depth of the round* saying what this pass covered and what it did
  not, and whether the reviewer was the warm thread or a cold start.
- The ledger's **State** field at the top moves `open` → `resolved`, and the
  **Synthesis** section gets written — it is still the template. Synthesis is the
  closure story: what the review changed, what it confirmed, and the risks it
  knowingly leaves standing. A reader who trusts it should not need the findings.
  Two things belong in it and are easy to leave out: the **class** F-22..F-27
  established, including that the responder committed it again at F-30; and the
  fact that these artefacts hold the same claim in several homes at once, which
  is the root cause and which `docs/AGENTS.md` already warns against.
- Then `design.md` has been reviewed twice and repaired thirty-one times. The
  next stage is **plan**, with its own ledger, per `docs/AGENTS.md`.

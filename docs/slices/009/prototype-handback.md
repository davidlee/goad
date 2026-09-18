# Handback — the slice 009 prototype, to the design

**To:** whoever carries slice 009's design forward — round 4, the plan, or the
user's next decision.
**From:** the prototype run on branch `slice-009-prototype`, worktree
`/home/david/dev/goad-009-proto`, 2026-09-18.
**Status of this document:** evidence. It amends nothing and binds nothing.

## What the prototype is, and what it is not

An end-to-end build of `design.md` as it stood after round 3's integration
(`c065a60`), made while the review was still open, for two reasons the user
gave: to make progress, and to settle by measurement things that were being
settled by argument.

**The code is referenced, not used.** Nothing in that worktree is promoted. The
slice re-derives from its own plan. What travels back is this document.

It stopped after `text`. `number`, `choice` and `datetime` are **not built**, so
nothing below is evidence about them except where it says so.

### What it did not test

Say this out loud before citing it, because the temptation runs the other way:

- **§9's validation table.** Not attempted. Tiering, drivers and the injection
  discipline are the plan's, and they are most of the slice's cost.
- **The three kinds after `text`**, and therefore the pickers, the DST
  composition, `slider_bounds` *in use*, and the `ComboBox` popup layout
  question §8 R9 carries.
- **Everything AC-10 is for** — the caret, the picker chain, a drag across a
  present. No instrument in any tier reaches them and the prototype added none.
- **The existing suite's rewrite.** §5.1's ~20 `FieldForm` / `Reported` sites
  were left alone; they did not need touching, which is itself P-13.

## Where to look

`docs/slices/009/prototype-notes.md` in that worktree holds the charter and all
fifteen findings in full, with the reasoning. This document is the index and the
recommendation. Findings are `P-n` and immutable; **P-15 is withdrawn** and says
why in place.

Twelve commits, `e3f2bff` … `0c3ff59`. Gate at `0c3ff59`: `cargo build`,
`cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check`
clean; 187 renderer + 68 lib + 14 prototype-tier + 4 loop-tier tests green;
**one red**, which is P-10.

## How to cite this

A prototype finding is **not** a ledger finding. `docs/AGENTS.md` is explicit
that a finding is a reviewer's observation living in a ledger and ending
`verified` or `withdrawn`, and that a decision is the user's and lives in a log.
These are neither: they are measurements.

So: a `P-n` that becomes a design change needs either a ledger finding raised
against it in the ordinary way, or a `design-log.md` decision citing it. Do not
paste `P-n` ids into `review-design.md` as though a round had raised them.

**Do not hand this list to round 4's reviewer.** Prompt it with surfaces, as
`docs/memory/dont-feed-the-raiser-your-finding.md` says. If round 4 finds any of
these independently that is the second witness; if it does not, these still
stand on their own evidence.

---

## 1. Act before round 4 — one blocks closure

### P-10 — the design's `resolve` cannot be implemented under a live ADR-001 instrument

`crates/goad-boundary`'s
`structure::no_production_line_in_the_renderer_names_the_identifier_resolve`
asserts that **no production line under `crates/goad/src` names the identifier
`resolve` at all**. Its subject is `goad_semantics::schedule::resolve` — SPEC-002/R-2,
AC-6 (a) of slice 003 — and it is deliberately an identifier-word match rather
than a path grep, *"because a brace-grouped
`use goad_semantics::schedule::{resolve, wait_for};` would defeat a path grep
(F-3) but not this"*. A different function of the same name defeats it from the
other side.

§5.2 names the new pure function `resolve`, in `view_model.rs`, called from
`controller::edit` and `glass.rs`. The test is red from the first line that
lands. `just check` runs `cargo test --workspace`, so **the slice cannot close
as designed**.

Two amendments, neither free:

- **Rename the design's function.** Touches §5.2, §5.3, §5.5 I-G and §9.
  Cheaper, and it is a rename rather than a weakening.
- **Narrow the instrument** to match the import rather than the word — which is
  exactly the F-3 evasion it was written to close, and a real reduction in what
  ADR-001 holds.

This is canon-adjacent and belongs to the user. The prototype left it red
deliberately rather than choosing.

### P-1 — `resolve`'s `None` surface is one class short of what its signature admits

F-42's disposition says the surface is *"stated in full … there are **two**"* —
a choice index no alternative has, and a non-finite `AdjustedValue`. F-42's own
*Observed* paragraph names a third in passing — *"(and other report/kind
mismatches)"* — and the disposition places the two and not that one.

`resolve(&Reported, Option<&Edited>, &DrawnKind)` admits all thirty pairs, six
of them in-kind. Two of the mismatched arms **cannot** be `None` without
contradicting §5.2 elsewhere: `AdjustedText`'s rule is *the text is recorded
verbatim, always*, so a mismatched one still has to yield an `Edited::Adjusted`.
And `Chosen` against a non-`choice` kind falls into case 1 **only if** the
implementation answers *no alternatives* for the other four kinds — a
coincidence of spelling. An implementation with a `_ => None` arm satisfies the
same two-case sentence while adding a failure the design never sanctioned.

F-42's whole argument is that *a step left off the list becomes an `unwrap`*.
The list is one class short.

**One sentence fixes it**, and the prototype's own choice is a candidate: *a
report whose kind is not the field's is not a third `None` case — it takes the
most conservative in-kind answer.* The alternative is narrowing the signature so
the pairs cannot be formed, which is a larger change and probably not worth it.

## 2. One-sentence design repairs

Each is a place the design says something that does not survive contact. None
changes a decision.

| | what is wrong | where |
|---|---|---|
| **P-2** | `as_drawn`'s `choice` arm — *"Always defined: `Alternatives::new` rejects an empty list"* — is true of the protocol and invisible to the compiler. `.first()` is an `Option`; `unwrap_used`, `expect_used` and `indexing_slicing` are `deny` crate-wide; and `AlternativeId::new` is `pub(super)` so there is no fallback to construct. §5.2's own *"a total expression is cheaper than an argument about why an `expect` is unreachable"* closes the obvious escape. Three ways out exist and the design picks none | §5.2, the as-drawn bullets |
| **P-3** | `Edited::Adjusted` carries a **text** beside the number, so `as_drawn` cannot answer for a `number` without choosing a format — and the as-drawn bullet names only the number. That text is what the widget is drawn showing and what the guard compares, so it is not cosmetic. Measured: `f64`'s `Display` never goes scientific, so a field declared `min: f64::MAX` — legal under `R-17` — draws a **309-character** `LineEdit` | §5.2, beside the parse rule it is the inverse of |
| **P-7** | *"`-`, `.` and `-.` … are exactly the three texts the control admits that no parse accepts"* is wrong twice. Slint's decimal arm is two rules: a `len <= 2` escape admitting `-`, **the separator**, and `-`+separator — so the trio is right only in a dot locale, and F-26 removed the named separator from the parse rule one paragraph earlier. Anything longer goes to `string_to_float`, which is `parse::<f32>` and accepts `inf`, `infinity`, `nan` case-insensitively. **`inf` is reachable by pasting.** The behaviour is already correct — it parses, `Finite` refuses it, the last number stands — and is asserted; only the prose is short a set | §5.2, *One rule covers every text* |
| **P-8** | F-48 says `Eq` stops deriving on `Command`, `Edited` and `Reported`, and warns against the hand-written impl. The trap is reachable from the **leaf**: `impl Eq for Finite {}` is *sound* — `Finite` excludes `NaN` — and on its own restores the derives on `Edited` and `Command` under `-D warnings`. It shipped in the prototype's first commit and was deleted on measurement. Worth one line in F-48 saying the newtype carries no `Eq` either | F-48's Response |

## 3. Claims that moved from cited to measured

These let §9 rows be written with more confidence, and one unblocks a canon
question.

- **P-4 — the datetime *wire* behaviour needs no manifest change.** Under
  today's featureless `jiff`, reached from `crates/goad` as it stands,
  `jiff::tz::Offset`, `Offset::UTC`, `Offset::constant`,
  `Timestamp::UNIX_EPOCH` and `display_with_offset` all compile and run. So
  `Edited::Picked`, `Reported::Picked` and `submitted`'s `R-57` datetime arm
  land with no feature. **§10's argument gates `compose` and `today_local` and
  nothing else**, and the epoch's spelling is now asserted rather than
  predicted: `1970-01-01T00:00:00+00:00`, and `1969-12-31T19:00:00-05:00` for
  the same instant at `-05:00`.
  *Consequence:* `canon-delta.md` CD-1's open question — whether the epoch is
  stated normatively or descriptively — can be settled against a green test,
  before the `POL-001` residue argument is had at all.
- **P-11 — the timer re-arm reproduces.** Source-confirmed at §5.1's cited
  lines *and* measured under a real loop: two entries from one window both
  arrive down a capacity-one channel, the second only reachable through the
  re-arm. Removing the re-arm delivers one and the case fails.
- **P-12 — `init` runs under `init_no_event_loop`.** So A-1's *replacing
  `values` wholesale destroys no element* is a **cheap-tier** row, not a
  loop-tier one. §9 currently reasons about it under the `changed`-handler rule;
  `init` is not a `changed` handler and the two do not share a tier.

## 4. Work the plan has not yet priced

- **P-13 — the undrawn fixture migrates once per phase, and after the last one
  it cannot exist.** §5.1's table enumerates the `FieldForm` consumers and
  prices them as one pass. It is not one pass: each phase that draws a kind
  moves the fixture's undrawn field to a kind still undrawn, and after
  `datetime` there is no kind left to move it to. The prototype needed **no**
  legacy module disabled at P1b precisely because it drew one kind and moved the
  word once. The pass §5.1 describes happens at the *last* phase, not spread
  across them.
- **P-14 — *reported, and the answer still goes* has nowhere durable to be
  reported.** §5.2 gives a carried edit naming an unknown option or field the
  `Refused::UnknownField` posture *and no answer sent*; the prototype's
  `Controller::choose` reports through `self.refuse` and returns `Ok`, the shape
  `refuse_during_exchange` already uses. But `serve` has one refusal site and
  `absorb` replaces the diagnostics when the exchange folds, so a refusal raised
  beside an answer that proceeds is overwritten before anyone sees it. Any §9
  row asserting it needs a surface that survives the fold, or the design needs
  to say the refusal is not durable.

## 5. Two things still open against the slice, from before the prototype

Found while extracting round 3's dispositions; neither is a prototype finding
and both are still live. `notes.md` lists five responder citations as *known
bad*; two do not survive checking:

- *"the pre-repair §9's `set_accessible_value` claim appears nowhere here"* —
  **false as written.** §9's no-loop table uses `set_accessible_value(text)` for
  the `LineEdit` row, citing `fluent/lineedit.slint:16`, and four obligation
  rows name it as their driver (AC-4, AC-6, AC-9, the debounce timer). Acting on
  that note deletes a live driver.
- *"F-42's location line cites §5.5 I-G for a claim that is in §5.2"* — the
  claim is in **both**. §5.2 says `Reported` *"cannot express a non-finite
  number or an id nobody declared"*; §5.5 I-G says *"a non-finite one is not
  expressible at the boundary either"*. F-42's Location and Evidence both name
  the two. The location line is right and **both sentences need repair**.

## 6. Process, for the slice rather than the design

- **Writing the repair down is the strongest finding-generator this slice
  has.** F-37, then F-46 … F-49, all raised by the session integrating rather
  than by a reviewer — and F-46, F-47 and F-49 are one shape: *a repair that
  reads correctly and whose mechanism does not exist, or whose test would pass
  vacuously*. `docs/memory/writing-the-repair-down-is-the-review.md` already
  says this; the prototype is a third instance of the same thing one level down.
- **`review-code.md` will run against a moving tree.** The prototype lost a
  finding to it: P-15 was written from a read of `draft.rs` that a commit
  landing mid-phase had invalidated, and it described the code wrongly while
  every finding about the *design* survived the same commit untouched. A code
  review's findings are claims about the tree, and they go stale in a way a
  design review's do not.
- **An injection-pass harness must revert against a commit, not the working
  tree.** The prototype's runner did `git checkout -- crates/goad/src` against
  an uncommitted tree and reverted a whole phase. It was replayed and nothing
  was lost, but §9 commissions an injection pass for every new case and this is
  the foot-gun in it.

## 7. Recommendation

1. **Take P-10 and P-1 to the user as decisions**, before round 4. P-10 blocks
   closure whatever else happens and the choice is not the design's to make;
   P-1 is one sentence but two implementers diverge without it.
2. **Integrate the four one-sentence repairs** (P-2, P-3, P-7, P-8) as part of
   the same pass, so round 4 reviews the design that would actually be built.
3. **Re-tier the §9 rows P-12 moves**, and note P-4 against CD-1.
4. **Re-price P-13 and P-14** into the plan rather than the design.
5. **Then round 4**, with a fresh reviewer, prompted with surfaces and not with
   this list.

Whether to finish the prototype — `number`, `choice`, `datetime` — is a separate
call and not needed for any of the above. If it is wanted, **`datetime` first**:
`number` mostly exercises pure functions that already have coverage, `choice` is
small, and `datetime` is where the unmeasured mechanisms are.

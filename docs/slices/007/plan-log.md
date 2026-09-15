# Plan log — Slice 007

Append-only, time-ordered. What was asked, what the user decided, and why.
Decisions only: findings live in `review-plan.md`, current truth lives in
`plan.md`.

---

## 2026-09-15 — `slice-007.md` §Scope was stale against the design

**Raised.** Planning traced the design's surfaces against the slice card's
§Scope list and found three paths the design requires and the card does not
name:

- `crates/goad/src/main.rs` — `design.md` §5.3 has `main` construct the `Notice`
  signal beside `Cancel`, clone it into `Wire`, and pass it to `serve`. The
  notice repair arrived during design review, after the card's scope list was
  written.
- `crates/goad/src/lib.rs` — the new `draft` module must be declared.
- `crates/goad/tests/event_loop/`, `crates/goad/tests/event_loop_schedule/` —
  both call `serve` and `Wire::new`, whose signatures change. Call shape only.

The card's §Scope is what `audit.md`'s surface diff is read against, so a stale
list turns three declared paths into three undeclared ones — the strongest lead
an audit has, spent on a bookkeeping gap.

**Decided.** Amend the card rather than reopen design or leave it to audit. The
content of the amendment is derived from `design.md` §5.2 and §5.3 and takes no
new decision; what made it the user's call is that it edits a design-stage
artefact during plan.

**Done.** `slice-007.md` §Scope gains the three entries, each with its reason,
and two existing entries are extended to name the notice (`controller.rs`'s
`Frame`, `wire.rs`'s `Notice`, `glass.rs` writing `notice` from the frame).

## 2026-09-15 — the plan is reviewed adversarially, one round

**Raised.** `docs/AGENTS.md` §Plan offers the choice; tier 2 permits it. The
design review ran two rounds to 32 verified findings, and its own synthesis
names a repeating class — a repair landing at the cited site while the claim
stays live elsewhere — that a plan is equally exposed to.

**Decided.** One round. Reviewer: **a fresh Claude agent (Opus 5)** —
codex was chosen first and the user changed it before the review was spawned.
Ledger: `review-plan.md`, copied from `docs/templates/review-ledger.md`, subject
`plan`. The reviewer holds the **raiser** role only: it appends findings and
leaves disposition and outcome blank.

## 2026-09-15 — the review's thirteen findings, dispositioned

**Raised.** Round 1 returned **13 findings: 1 blocker, 2 major, 7 minor, 3 nit**
(`review-plan.md`). The blocker and the sharpest of the rest were re-verified
against the code before anything was dispositioned, rather than taken on the
reviewer's report.

Nine of the thirteen are one shape — a claim the plan changes staying live where
the plan's own instrument cannot see it
(`docs/memory/a-repair-sweep-misses-the-binding-site.md`). That is the class the
design review hit three times, and the plan committed it again in the very
criterion written to prevent it: PHASE-01/EX-7 enumerated eight prose homes of
the `notice` rule, and two more existed that contain no occurrence of the word
"notice", so VA-2's grep could not have found them.

**Decided.** All thirteen `doc-wrong` — the plan is the defect in every case.
None `aligned`, `tolerated` or `follow-up`. Confirmed en bloc after the table of
proposed fixes was put to the user.

**Decided separately — F-1's wording.** `Refused::UnknownField` needs a
user-visible string. `design.md` §5.2 states every other diagnostic wording
verbatim, for the stated reason that wording should be "reviewed once rather
than discovered in a diff", and the alternatives were to split the wordings
across two documents or to let one be discovered in a diff. **Amend `design.md`
§5.2**, one line:

> no action taken: the host could not match that control to a field of the
> option it names

Parallel to `UnknownOption`'s line and distinguishable from it: it says which of
the two selectors failed. This is a change to a design that was already
approved, so `docs/AGENTS.md` §Design's last line applies — **the design needs
the user's approval again**, and it is asked for alongside the plan's.

**Decided — F-8 goes to `canon-delta.md`, not to a phase.** SPEC-001 §7's
existing R-18 verification row states that `hints` is read in `src/` only by
`normalize.rs::normalize_field` and that "the renderer, the one component that
may, does not exist yet". PHASE-03 falsifies both. The requirement is unamended
and the verdict stays *review, not a test*; only the evidence beneath it moves.
CD-1's *Sections* now names it and a new subsection states it. Endorsement is
owed at promotion, during audit, not now.

**Done.** `plan.md` repaired throughout; `design.md` §5.2 amended;
`canon-delta.md` CD-1 extended; `review-plan.md` carries a disposition and a
response per finding. Criterion ids were appended rather than renumbered, except
two in PHASE-02 that were mis-numbered on first writing (EX-9/EX-10 in a phase
ending at EX-6) and were corrected to EX-7/EX-8 before anything cited them.

**Outstanding.** The terminal **Outcome** column is the raiser's, not the
responder's (`review-plan.md` §Protocol). The repairs are handed back to the
reviewer for a verification pass — not a second round — and the ledger resolves
when every finding is `verified` or `withdrawn`.

## 2026-09-15 — the ledger resolves, and what it cost to get there

**Outcome.** 13 findings, **13 `verified`**, none withdrawn, no blocker
outstanding. `review-plan.md` is `resolved` and carries the Synthesis.

**F-7 was contested and the contest was right.** Its repair named the correct
criteria in the ledger and left the wrong one in the artefact, so the Coverage
cell went on asserting something false about the plan's own Exit lists — the
defect F-7 raised, surviving its own fix. Two more of the same shape were
surfaced during verification. All three were casualties of one renumber in
PHASE-02, and all three were found by the raiser rather than by the responder.

**The correction that matters is not the three citations.** The plan's
instrument for this ledger's dominant class worked: EX-7's enumerated list plus
VA-2's grep found both sites the grep itself cannot reach, and documents why.
What had no instrument was the plan's **internal** consistency — its own ids,
counts and cross-references, which every repair edits and nothing checks. That
is why the repairs for the class reproduced the class.

**Decided, on the reviewer's closing advice and taken as an improvement rather
than as a finding:** `plan.md`'s Overview now carries a standing rule that
**every phase sheet re-derives its criterion ids from `plan.md` rather than
trusting the copy**, with PHASE-04's sheet doing it twice — it grew from seven
exit criteria to ten under repair and restates more claims in prose than any
other phase. A phase sheet is the next renumber-shaped edit in this slice's
future and there are six of them coming, performed by six agents, with nothing
checking a sheet against the plan it was copied from.

**Corrected while closing:** the plan estimated "~18 `serve(..)` calls". There
are **36**, thirteen of them in `ingress.rs` and fourteen in `scheduling.rs` —
neither of which PHASE-01's Surfaces would lead an implementer to expect. The
three signature sweeps total 111 measured call sites, not the ~90 the Sequencing
section claimed. Both figures are now measured rather than estimated.

**Outstanding — two user gates.** Acceptance of `plan.md`, and **re-approval of
`design.md`**, which changed after it was approved (§5.2, the
`Refused::UnknownField` line). `docs/AGENTS.md` §Design asks for approval again
when that happens.

## 2026-09-15 — both gates clear; the slice moves to execute

**Asked.** The two gates the ledger's closing entry left outstanding:
acceptance of `plan.md`, and re-approval of `design.md` after its §5.2
amendment.

**Decided.** Both given, together. `plan.md` is accepted as it stands — six
phases, `review-plan.md` resolved at 13/13 verified, no blocker outstanding, and
the internal consistency repair (every cross-reference resolving, no id gap or
duplicate) carried out before acceptance rather than promised for it. The design
re-approval is recorded in `design-log.md`, which is where a design-stage
decision belongs.

**Consequence.** `slice-007.md` Stage becomes `executing`. PHASE-01 proceeds
under `phase-01-brief.md`, whose first instruction is to check these two entries
rather than infer them. `notes.md` is still a template; its Status table and
PHASE-01 sheet are that phase's first work, not this stage's.

**Not taken by this gate.** `canon-delta.md` stays draft canon. Its promotion
into SPEC-001 is audit's, with its own endorsement (`docs/AGENTS.md` §Audit &
reconcile); accepting the plan is not that endorsement.

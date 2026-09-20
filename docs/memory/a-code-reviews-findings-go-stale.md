# A code review's findings go stale in a way a design review's do not

First seen in slice 009's prototype, which lost a finding to it; then four times
over in the audit, as worktrees arriving behind the tree.

## The fact

A finding about **code** is a claim about a tree at an instant. The tree moves —
another phase lands, a repair goes in, a reviewer's own worktree turns out to be
behind — and the claim silently stops describing anything.

A finding about the **design** does not have this property. The prototype's
`P-15` was written from a read of `draft.rs` that a commit landing mid-phase had
invalidated; it described the code wrongly and had to be withdrawn. Every
finding about the design survived the same commit untouched.

The audit then found the same asymmetry four more times, from the other side:
reviewers whose worktrees were 25, 38, 42 and 43 commits behind. Two of those
reviews would have been confident, coherent reports about code that no longer
existed.

## Why

A design claim is about intent, and intent changes only when someone decides to
change it — a visible, minuted event. A code claim is about a state that a
dozen ordinary actions invalidate, none of which is about the finding.

The dangerous part is that a stale code finding **reads exactly like a live
one**. It cites real symbols, quotes real source, and argues correctly from what
it saw. Nothing about its shape says "this was true forty commits ago."

## How to apply

- **Every code review states the commit it ran at**, in the ledger, at the top.
  `review-code.md`'s round sections each name their range.
- **Every subagent confirms its tree before reading** — see
  `subagent-worktrees-can-be-stale.md` for the Step 0 block, including the
  fast-forward recovery.
- **Re-verify a finding at the source before pricing it.** Slice 009's audit did
  this as standing practice and it changed an answer **seven times** across four
  rounds — twice in round 1, once in round 2, twice in round 3, twice in round
  4. The cost is minutes; the alternative is dispositioning something that is no
  longer true.
- Where a reviewer and the tree disagree, the tree wins — but check *which*
  tree the reviewer was looking at before concluding they were wrong.
- Prefer, in a ledger, the finding that names a **mechanism** over the one that
  names a line. The first survives the next commit.

Related: `subagent-worktrees-can-be-stale.md`,
`cite-by-symbol-not-line-number.md`, `a-reviewers-example-is-not-evidence.md`.

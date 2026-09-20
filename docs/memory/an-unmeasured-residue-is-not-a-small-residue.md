# "State the residue" is only honest once the residue has a number

Slice 009's audit, the `F-D3` decision — taken, and then taken again an hour
later when the residue was measured.

## The fact

A finding was repaired, and the repair left a known residue: some citations had
been converted, others had not, and nothing enforced the discipline going
forward. The disposition put to the user was *fix the named instances, state
the residue, and close* — priced on the audit's own description of that residue:

> smaller than 59 and larger than 3, and nobody has measured it

The measurement took about fifteen minutes of scripting. It came back:
**53 citations, 27 landing on a comment or a blank line, ~24 simply wrong** —
and **one file's drift accounted for thirteen of them**, nine being the same
citation repeated across seven files.

The decision reversed. Not because anyone argued better, but because *"state the
residue"* had silently meant *"write down that half of them are wrong and close
anyway"*, and nobody had known that when they agreed to it.

## Why

An unmeasured quantity does not sit neutrally in a decision. It gets filled in
by the shape of the sentence around it, and "residue" is a small-sounding word.
Everyone in the conversation — the reviewer who raised it, the agent who priced
it, the user who decided — was reasoning about a quantity they had each
imagined.

The concentration matters as much as the count. A residue spread thin is a
tolerable background cost; a residue where one edit broke thirteen citations at
once is a live failure mode with a cheap fix. Those two call for opposite
decisions and are indistinguishable without the number.

## How to apply

- **Before offering `tolerated`, `follow-up`, or "state the residue", measure
  the residue.** If you cannot measure it in the time you would spend writing
  the paragraph describing it, say that instead of estimating.
- Distinguish the two questions: *how many* and *how concentrated*. Report both.
- **Go back when the premise changes.** A decision taken on an estimate is not
  binding once the estimate is replaced by a fact. Both decisions here were
  correct on the information available; what changed was that the information
  stopped being an estimate.
- The converse of `price-the-rejected-option-against-code`: don't rule an option
  *out* on an estimate, and don't rule one *in* on one either.

Related: `enumerate-the-class-not-the-instances.md`,
`cite-by-symbol-not-line-number.md` for the class this was measured on.

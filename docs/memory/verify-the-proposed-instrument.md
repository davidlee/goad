# A finding can be right that nothing holds a property, and wrong about what would hold it

Slice 009's audit, `F-T1` — where the reviewer's own proposed repair was weaker
than the one that landed.

## The fact

A coverage finding has two halves, and reviewers are much better at the first:

1. **This property is held by nothing** — usually right, and usually
   demonstrated with a mutation.
2. **Here is what would hold it** — an afterthought, and often wrong.

`F-T1` found that a case's central reading was non-vacuous only by arithmetic
on a literal copy of a private constant, and that the copy going stale was
silent. Correct. Its proposal: derive each target's step schedule from the
constant.

**That would have been the weaker instrument.** Derivation rescales silently —
change the constant and every schedule follows, and the case stays green while
meaning something different. What landed instead was `pub` on the constant plus
eight lines of `const _: () = assert!(…)`, which makes the finding's own
`150 → 400` mutation **fail to compile**, with the reason spelled out: *"reading
B is vacuous unless the debounce deadline falls between step 3 and step 11."*

## Why

Demonstrating absence is mechanical — apply the mutation, watch the suite stay
green. Designing an instrument is a design problem, and it is being done in the
margin of a finding whose subject is something else.

The specific trap is that the obvious fix often **removes the symptom rather
than holding the property**: a derived constant removes the staleness without
holding the relationship the case depends on.

## How to apply

- **Take the first half and re-derive the second.** Accept *"nothing holds
  this"* on the mutation; treat *"here is the fix"* as a suggestion.
- **Ask what the proposed instrument goes red for**, and what it stays green
  for. If a plausible future change keeps it green while breaking the property,
  it is the wrong instrument.
- **Prefer an instrument that fails at compile time** to one that fails at test
  time, and one that fails at test time to a comment.
- **Price the instrument against canon, not only against code.** Two of slice
  009's proposed instruments were rejected because they would have been a fifth
  entry in a policy that enumerates its instruments — a policy amendment taken
  mid-audit for a `minor`. That is a real cost and it is not a code cost.

Related: `verify-the-enumeration-not-the-conclusion.md`,
`a-check-that-compares-two-derived-things-holds-nothing.md`,
`price-the-rejected-option-against-code.md`.

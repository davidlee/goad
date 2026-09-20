# "This adds coupling X" is a claim about the codebase — grep before ruling the option out

Slice 009's review, where an option was nearly rejected for introducing a
dependency the codebase already had in three places.

## The fact

*"That would couple A to B"*, *"that introduces a dependency on C"*, *"we don't
do that here"* — these sound like design principles. They are **empirical claims
about the repository**, and they are cheap to check and frequently false.

When the check is skipped, an option is rejected for a cost the codebase is
already paying, and the rejection goes into the record as a principle.

## Why it is worth the thirty seconds

Three outcomes, all useful:

- **Precedent exists.** The objection evaporates, and the existing sites tell
  you the idiom to follow.
- **Precedent exists and is regretted.** Now you have a real argument, with
  examples, and possibly a follow-up.
- **No precedent.** The objection stands and is now *evidenced* — which is what
  makes it survive the next reviewer who has the same idea.

Slice 009 saw the inverse too, and it is the stronger warning: a repair was
preferred **because** it matched what three loop-tier harnesses already did —
and that precedent turned out to be exactly why the rig could not see the
defect. Production was being made to do what the rig does. Precedent is
evidence, not authority.

## How to apply

- **Grep before you assert.** `grep -rn "TypeName" crates/`, or for the import,
  or for the pattern. Name the number of sites in the record.
- **Say what you found**, not just the conclusion: *"`glass.rs` and two test
  files already do this"* is checkable; *"this is the house style"* is not.
- **Distinguish "we do this" from "we should."** Finding three instances does
  not license a fourth if all three are known bad.
- The same rule covers invariants. Slice 009 rejected a repair because splitting
  one walk would reintroduce *a second counter that could fall out of step with
  the vector's own length* — and that was quotable from the invariant, not
  asserted from memory.

Related: `price-the-rejected-option-against-code.md`,
`enumerate-the-class-not-the-instances.md`,
`a-reviewers-example-is-not-evidence.md`.

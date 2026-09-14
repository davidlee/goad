# A green test can assert a proxy the regression it exists for would survive

Learned at slice 004, where five code-review findings were this one shape
(`review-code.md` F-5, F-6, F-21, F-23, F-24).

## The fact

In each case the test named a real requirement, passed honestly, and would have
gone on passing after the defect it was written to exclude was introduced —
because what it read sat downstream of something that erases the difference:

- an `absorb` that replaces the whole retained value before the read (F-23);
- a `cleanup` helper that ignores its errors, so a file that should not have
  been removed is removed silently (F-24);
- a rendered string that is true of two distinct states (F-21).

Nothing about such a test looks wrong. It is green, it is well named, and its
assertion is about the right subject.

## The rule

**The test for a test is not that it passes. It is that you have seen it fail
for the reason it exists.**

## How to apply

- **Inject the regression the case is written against, run it, read the
  message, revert.** Cheap, and it is the whole discipline. Confirm the revert
  with `git diff` before committing.
- **Where a case asserts two absences, inject twice — one per half.** It can be
  vacuous in one of them and green anyway (F-24).
- **Where the case is a *negative*, ask a further question first:** is there a
  moment at which the forbidden thing would be visible, and does the case read
  *at that moment*? A negative that reads after the evidence is erased is green
  in both worlds (F-23).
- Watch for the same shape in a **Verification row**: a row citing a case that
  cannot hold the claim is this defect one level up.

## Related

- `a-bound-is-not-tested-at-the-bound.md` — the same failure where the outcome
  is what two implementations agree on.

## Slice 005: three more, and the measurement that found them

`review-code.md` F-7. Three `render` cases asserted a property of the whole
variant **set** — that all five lines read differently from one another — rather
than which arm produced which line. Swapping two match arms leaves the set
distinct, so the swap stays green.

What makes this instance worth keeping is how it was settled. The repair agent
did not reason about it: it restored each case to its pre-repair body, injected
the swap, and ran them. **All three stayed green**, exactly as the finding
predicted. The fix is a table of (variant, a phrase only that arm says), so the
loop asserts a *mapping* rather than a property of the collection.

The distinctness check was **kept** beside the new per-arm pins rather than
replaced by them, and the reason generalises: two arms could say each other's
phrase *as well as* their own, and per-arm pins would not see that. A set
property and a mapping property are different assertions; neither implies the
other.

**The phase that produced them was the one phase with no injection pass.** The
same slice's PHASE-04 cases were mutation-checked against six injected defects
at the time of writing and every one was load-bearing. The pass is what
separates the two phases, not the care taken — the three weak cases were
written as carefully as the strong ones.

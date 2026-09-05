# Assert containment (⊇), not equality, on a set a mechanical process might do better than predicted

Learned at slice 002, PHASE-01, `review-plan.md` F-37.

## The fact

PHASE-01's exit criterion originally asserted an **equality** between the
predicted byte-identical rename count (88, from reading the artifact map)
and the measured one. The actual `git mv`-only split produced **92**
`R100` renames — four files (`error.rs` and three `mod.rs` files) needed
none of the change their map row had budgeted for, because a `git mv` with
no accompanying edit is *more* likely to register as byte-identical than a
prediction that assumed some import-path churn.

An equality check punishes the mechanism for doing its job better than
forecast — a wider byte-identical set is the underlying goal (files move
without being touched) being met *more* fully, not a discrepancy to
explain away.

## How to apply

- When a criterion predicts a count for something a mechanical process
  produces (renames, generated files, matched fixtures), state it as
  containment in the direction that rewards the process's actual goal —
  here, "every predicted file appears as `R100`, and every actual `R100`
  beyond that is named" — rather than as an equality.
- If a set comes back wider than predicted, name the extra members and
  confirm each is not a redesign masquerading as extra cleanliness — then
  accept it; do not treat the width itself as a defect.
- This generalizes past renames: any check comparing a prediction to a
  mechanically-produced outcome should ask "does an equality here punish a
  cleaner-than-expected result?" before writing it.

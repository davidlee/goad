# A bounded surface is billed from the compiler, and a move's bill includes the imports whose last consumer moved

Settled at slice 003, `plan-log.md` PL-14, from `review-plan.md` F-13.

## The fact

`unused` is `deny` at the workspace root with `unused_imports` kept
(`Cargo.toml`), and the gate promotes warnings to errors. So moving the last
consumer of an imported item out of a file makes that file's `use` line a build
error. If the phase's declared surface admits the move but not the import
removal, the phase cannot reach a green gate without breaking its own STOP.

"Whatever the compiler demands" is not a bound, and a STOP an executor must
disobey in order to end green is worse than no STOP at all.

## The rule

**Name the lines.** A phase that moves code lists the imports it orphans, one by
one, with what each becomes. The list is checkable at the phase and re-derivable
at audit, and it turns *"is this edit in scope?"* from a judgement into a lookup.

**Bill it from the compiler, not from a grep.** Slice 003's own measurement found
one item the reviewer's reading had missed: `Model`, a trait that never appears
by name in the moved code and is only there because `.row_data` needs it in
scope. A grep for identifiers cannot find a trait import; deleting the line and
reading the error can. The same measurement found a ninth `use` line
(`std::time::Duration`) beyond the eight the review had named.

## How to apply

- When planning a move, delete the candidate imports and compile. The errors are
  the bill. Traits and blanket-impl imports are the ones a name-based reading
  misses.
- Write the bill into the phase's own criteria and exclude exactly those lines
  from the phase's STOP, so the executor is not choosing between the STOP and the
  gate.
- Do not reach for `#[allow]` on an orphaned import to stay inside a stale bound
  (`docs/policy/001-the-phase-gate.md` §Compliance, and slice 003 PL-10). Widen
  the bound explicitly instead.
- Sweep the class across every phase once, not per phase as it bites. Slice 003
  found exactly one other instance this way.

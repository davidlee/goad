# A true statement can go stale by widening, and that is not a bad citation

Twice in one phase of slice 009, and it is a different animal from a wrong
number.

## The fact

A sentence that enumerates something can stop being useful without ever having
been wrong. The world got larger than the scope the sentence was right about.

- `design.md` §9 said **twelve** sites. It was accurate about *constructors* of
  `Command::Edit` and `Edited`, and silent about a **binder** of the markup
  callback — which is why a thirteenth site appeared that the count had never
  claimed to cover.
- `Refused::UnknownField`'s doc was accurate about the only path into it that
  existed when it was written. A later phase added a second path, and the doc's
  later clause — *"only reachable from a stale or malformed callback"* — became
  the more wrong of the two, because the new path is a well-formed, current
  callback.

Neither was wrong when written. Neither is a miscount. They do not belong on a
list of bad citations, which is about numbers that never pointed where they
said.

## How to apply

- **When a phase widens what a type or a rule *means*, re-read every sentence
  that enumerated it — including the ones that are still true.** That is the
  working rule the two cases share.
- **When you raise a stale doc, name every clause of it**, not the one that
  caught your eye. `Refused::UnknownField` had two, and the second — the worse
  one — was found by the person reading the report, not by the reporter.
- **Amend in the phase that makes the doc stale, not at audit.** Settled three
  times in slice 009. The line is the *discovery*: a divergence found at audit
  belongs in the Reconciliation table; one the slice creates knowingly does not.
  The corollary is that a phase whose criterion widens what a type means should
  check its Surfaces for the type's own file before it starts.
- A phase that widens a shared struct should **grep for its constructors, not
  for its subject**. A per-file scan for the phase's vocabulary came back clean
  while two files outside the declared surfaces broke — they build the struct by
  hand and are compelled by it gaining fields, which no search for a kind could
  find. `grep -rn "TypeName {" crates/` is the instrument.
- Beware `..Default::default()` in a hand-built fixture. It buys silence now and
  costs a phase later: the *next* field added will not break those files, so a
  phase that ought to look at them will not be made to. Record the trade where
  the next phase reads.

Related: `a-count-in-a-comment-is-a-claim-nothing-checks.md`,
`cite-by-symbol-not-line-number.md`,
`a-repair-sweep-misses-the-binding-site.md`.

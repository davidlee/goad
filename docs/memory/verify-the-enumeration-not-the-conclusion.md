# Verify the enumeration, not just the conclusion — a correct finding can carry a wrong sub-claim

Slice 009's audit, seven times across four rounds, and three of those changed an
answer.

## The fact

A reviewer's finding usually has two parts: a **conclusion** (*this property is
not held*) and an **enumeration** supporting it (*there are three of them, at
these lines, and here is the mutation*). The conclusion can be right while the
enumeration is wrong — and a Response that inherits the enumeration writes the
error into the artefact, where nothing will catch it again.

Four instances from one slice:

- A finding reported *"three citations the enumeration missed."* Counted at the
  source: **53 citations, 27 pointing at the wrong thing.** The conclusion — the
  class was never repaired — was right, and much stronger than stated.
- A finding reported *"three repeaters"* in a markup file. There are **four**;
  the fifth `for ` match was a word in a comment. Its conclusion, that a
  uniqueness claim was false, held.
- A finding's stated mutation *"does not lint clean"* — so the hole it described
  was one shape only, not the general one.
- A finding about a comment's wrap margin said the file *"wraps at 78"*. It does
  not; the convention is 80, and the file carries unwrappable 113-character
  identifiers. The three lines it named were nonetheless over the margin.

## Why

An enumeration is the part a reviewer produces under time pressure, by hand,
while the interesting thinking is going into the conclusion. It is also the part
that reads as *checked* — numbers and line references look measured — so the
responder takes it as given and builds the repair on it.

And the enumeration is what the repair's **scope** is set from. Get it wrong and
you repair the wrong number of things, then write a Response claiming you
repaired the class.

## How to apply

- **Re-derive every count and every location at the source before pricing the
  disposition.** Minutes each. It changed three answers in one slice.
- **Say in the finding that you corrected it**, and whether the correction
  widens or narrows the finding. Both directions matter; a widened finding may
  change the disposition, as it did here.
- **Do not let a corrected sub-claim reach the Response by inheritance.** The
  ledger is append-only, so the correction lives in the *new* finding and in the
  old one's Outcome — never by editing the original.
- Prefer an instrument over a hand count: `grep -c`, a script, an assertion. See
  `cite-from-an-instrument-that-prints-the-number.md`.

Related: `dont-feed-the-raiser-your-finding.md`,
`verify-the-proposed-instrument.md`, `a-reviewers-example-is-not-evidence.md`.

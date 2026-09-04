# State a grammar rule as a shape, and pin every edge as a fixture

Learned at slice 001's code review, rounds 1–6 (`review-code.md` F-2, F-37,
F-46, F-50, F-52, F-53, F-55): the refusal of a bare time of day in
`next_check` took **five rounds** to converge, and each round found the rule one
string further along than the last had tested.

## What went wrong, in order

The rule was first stated as "what `jiff::civil::Time` accepts" — a parser.
Then as "digits and colons" — a shape, but one that lost the fraction and the
`T` designator the parser had covered. Then both, and `::` became a time of day.
Then every group non-empty, and `"18:00:00 "` with a trailing space slipped
through because the span parser tolerated the space and the shape rule did not.
Then trimmed, and `"18"` was a clock hour by the parser and got a worse message
than before. Then the parser gated behind a colon-or-`T` check. Then the trim
applied to the span form only, and an absolute instant with a stray space was
called a time of day.

Every one of those is a **seam between two parsers that disagree** about the
same string. A rule stated as "what parser X accepts" has a seam wherever a
second parser sees the string first or last, and each seam is invisible until a
reviewer writes the string.

## The rule that held

State the rule as a **shape** the author can read — "two or more colon-separated
groups of ASCII digits, every group non-empty, an optional `.`/`,` fraction" —
and make the parser a **second conjunct behind a gate** that says when it is
even consulted. Normalise the input (trim) **once, before any rule sees it**, on
every path that reads the value, and keep the untrimmed value for the error.

Then pin **every edge a round found as a fixture**, named for what it is, not
for the round: padded and unpadded, with a fraction, with the designator, with
whitespace on each form, the unitless integer, the empty groups, the signed
form. Twelve fixtures hold the seam in `tests/protocol/fixtures/schedule/`.
SPEC-001 R-21 states the rule in the same words.

## How to apply

- A requirement that says "values parser X accepts" is not yet a requirement.
  Rewrite it as a shape a person can check by eye, and cite the parser only as
  a mechanism.
- When a review finds one string, ask which two parsers disagreed about it and
  write the fixture for the **class** — the other strings that seam admits —
  before repairing.
- Two entry points into one grammar (here the wire and the config file) must
  normalise identically, at the shared function, not at each caller.

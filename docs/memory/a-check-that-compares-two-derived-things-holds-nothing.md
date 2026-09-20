# A check comparing two things is worth nothing if one is derived from the other

Slice 009's markup, and the measurement that settled it.

## The fact

The host refuses a report whose variant is not the drawn field's kind. For that
refusal to be reachable from production at all, the two sides have to be able to
disagree.

The markup therefore writes the control's **own** kind as a literal —
`Kind.boolean` in the `CheckBox` — rather than the row's `field.kind`. The two
values are equal today and will stay equal. That is not the point. Writing
`field.kind` would make the report agree with the row **by construction**, and
`interpret`'s mismatch arm could then never fire from production markup.

Measured, not argued: planting the wrong literal turns five cases red across two
files. With `field.kind` there, nothing could ever turn red.

## The general shape

Wherever a check compares A against B, ask where B came from. If B is computed
from A, the check asserts that arithmetic is deterministic.

Other instances from the same slice:

- **A self-agreeing expected value.** An acceptance criterion's expected
  `datetime` was computed by the production `instant::compose` — so the case
  could only fail if `compose` disagreed with itself. What made it mean
  something was pinning the *format* by literals elsewhere, and saying in the
  record that the value half is self-agreeing.
- **A control read after the transient it is about has cleared.** One case's
  control asserted `!busy` at a step **after** the exchange had landed, and
  `absorb` clears `busy` on landing whatever engaged it. The reading was true
  regardless, and its own message claimed it held something it did not. The
  property was real and lived in a different target, whose arrangement reads
  `busy` while an exchange is still **outstanding**.
- **A count that a wildcard can absorb.** An exhaustive match is a compile-time
  guard against a new variant only while nobody adds a `_` arm — which compiles,
  lints clean under the default set, and leaves the gate green. What holds the
  property is the match **plus** `clippy::wildcard_enum_match_arm`.

## How to apply

- For every assertion, name the **two independent sources**. If you can only
  name one, the assertion is a tautology dressed as a test.
- Prefer a literal, a fixture, or a hand-written expected value over one the
  production code computes — and where you must compute it, pin a different
  axis (the format, the shape, the count) by hand.
- **Enumerate and count rather than sample.** A case walking all six reports
  against all five kinds and asserting `refused == 24` catches an injection that
  makes one whole kind permissive; sampling three pairs does not.
- Run the injection. The question *"could this ever go red?"* is cheap to
  answer and is not answerable by reading.

Related: `a-green-test-can-assert-a-proxy.md`,
`an-injection-that-reorders-a-fallback-is-not-an-injection.md`,
`a-test-rule-binds-to-a-defect-not-a-surface.md`.

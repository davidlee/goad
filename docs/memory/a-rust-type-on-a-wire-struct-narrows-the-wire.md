# A wire struct's Rust spelling makes claims about the wire that nobody reviews

Learned at slice 005, `review-code.md` F-1 — the clearest instance yet of the
failure `CLAUDE.md` says the project exists to avoid.

## The fact

`goad_shell::ingress::wire::Reply` typed two fields the host does not adjudicate
as `Option<u8>` and `Option<u64>`. Both are JSON **numbers**, and JSON has one
number type. Measured against the built binary before repair:

```
{"protocol":1,"accepted":true}                       exit 0
{"protocol":1.0,"accepted":true}                     exit 2  "is not one JSON object: invalid type: floating point `1.0`, expected u8"
{"protocol":300,"accepted":true}                     exit 2  "expected u8"
{...,"reason":"too_soon","retry_after_ms":1800.0}    exit 2  "expected u64"
```

Every one of those replies is conforming. `1.0` **is** `1`. A host spelling it
that way, or versioning past 255, was refused by a client whose own doc said
`protocol` *"is not read at all"* — and told the caller the reply was not one
JSON object when it plainly was one.

Nothing in the type looked wrong. `Option<u8>` for a small version number reads
as good taste. The narrowing is invisible at the site and invisible in review
unless someone asks the question below.

A second, quieter instance on the same struct: **`serde` reads a missing
`Option` field as `None` with no `#[serde(default)]`.** The attribute is inert
there, not wrong — so a doc comment saying "no field carries `#[serde(default)]`"
described a decision that was never taken, and three plan lines assumed an
attribute was load-bearing when it was decoration.

## The rule

**A field's Rust type may narrow the wire only as far as JSON's own types do.**
A JSON boolean is `bool`. A JSON string is `String`. A JSON number is a JSON
number — and if the client does not adjudicate its value, it is
`serde_json::Value` or it is absent from the struct.

The general form, which reaches past JSON: *a field whose value this client
adjudicates must be typed so its parse cannot fail, or its wrongness is
reported one step too early — as bytes, in a sentence that is false of them.*

## How to apply

- For every field on a wire type, ask: **does this code read this value?** If
  no, the type must not be able to refuse it. Unread and unrefusable are the
  same requirement.
- Put the adjudication where the meaning is, not in the deserializer. A field
  the client *does* judge (`retry_after_ms` must be a whole number of
  milliseconds, R-14) is parsed permissively and judged explicitly, so the
  refusal names the rule it broke instead of naming serde's expectation.
- Separate the pipeline so each step can only raise its own fault: bytes →
  document → meaning. Then "not one JSON object" is true whenever it is said,
  and a breach of the contract is never reported as malformed bytes.
- Treat `#[serde(default)]` on an `Option` as decoration. If a comment claims
  it is doing something, check.
- Watch for the tell: a doc comment saying a field is *not read*, on a field
  with a narrow type. The two cannot both be true.

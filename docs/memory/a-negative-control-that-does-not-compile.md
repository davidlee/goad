# A negative control that does not compile reads exactly like one that passes

Learned at slice 008, writing `src/zoom.rs`'s six cases. The first pass reported
two mutations as producing **no failure**. Both were compile errors.

## The fact

A negative control is run by breaking the production code and confirming the
test goes red. The check is usually "did anything say FAILED?" — and a build
that never produced a test binary says `FAILED` nowhere either. **Absence of red
is not evidence of a passing control; it is evidence of nothing.**

This crate makes the trap easy to fall into, because it denies warnings. Deleting
`.min(Self::CEILING)` from

```rust
Self(raw.max(Self::FLOOR).min(Self::CEILING))
```

leaves `Self((raw.max(Self::FLOOR)))`, which `unused_parens` rejects — so the
mutation that was supposed to prove the ceiling is tested instead proved the
lint is on.

## Why it matters here

It is a new face on `a-green-test-can-assert-a-proxy.md`. There, a green test
asserts something the defect would survive. Here, the *control* — the instrument
built to catch exactly that — fails silently in the same shape. The technique
that is supposed to make a green believable is itself unverified by default.

Slice 008 was bitten by the proxy trap twice before this, which is why the
controls were being run at all.

## How to apply

- **Confirm the control compiled and ran before believing its red.** Read the
  test count, not just the absence of `FAILED`. `0 passed` and `12 passed, 1
  failed` are different outcomes and grep does not distinguish them.
- Prefer a mutation that leaves valid code: change a constant, flip a
  comparison, drop a clause that stands alone. Deleting a method call from a
  chain is the shape that trips `unused_parens`, `unused_variables` and
  `unused_must_use`.
- If a control cannot be made to build, that is itself a finding: the property
  may be held by the type system or the lint set rather than by the test, and
  the test is then asserting something else.

Related: `a-green-test-can-assert-a-proxy.md`,
`a-bound-is-not-tested-at-the-bound.md`, `a-test-rule-binds-to-a-defect-not-a-surface.md`.

---

**Also cited as `negative-control-must-compile.md`.** Slice-009 and slice-007 artefacts reference this
lesson under that name; it has only ever lived here.

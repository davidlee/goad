# `autotests = false` makes an undeclared `tests/` target silently not built

Learned at slice 006, PHASE-03, where `crates/goad` grew its first binary tier.

## The fact

Both `goad` and `goad-emit` set `autotests = false`. With that flag, a directory
under `tests/` that has no matching `[[test]]` block in the manifest is **not
compiled and not run**, and cargo says nothing about it.

Measured, not assumed: with the `[[test]] name = "binary"` block removed,
`cargo test -p goad` prints no `Running tests/binary/…` line, reports no error,
and exits **0**.

## Why it matters more than it sounds

A whole tier of cases can be added to this workspace — written, reviewed,
committed — and leave the gate green while never executing once. The failure is
not a red test. It is the absence of a line in output nobody reads closely,
which is the same shape as a test asserting a proxy: the artefact claims
coverage it does not have.

## How to apply

- Adding a directory under `tests/` in this workspace is **two** edits. The
  second is the `[[test]]` block.
- Confirm the target actually ran before believing a new tier: look for the
  `Running tests/<name>/…` line by name, or count cases. Do not infer it from a
  green exit.
- The same reasoning applies to any new case file inside an existing tier — it
  reaches the runner only through a `mod` declaration in that target's root.

Related: `a-green-test-can-assert-a-proxy.md`, `tests-asserting-proxies.md`.

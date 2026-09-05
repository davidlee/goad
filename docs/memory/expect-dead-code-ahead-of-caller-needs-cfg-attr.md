# `#[expect(dead_code)]` on a helper landed ahead of its caller needs `cfg_attr(not(test), ...)`, and must be removed once the caller lands

Learned at slice 002, PHASE-06/PHASE-10 (`stamp`'s lifecycle,
`controller.rs`).

## The fact

A private function with no non-test caller is dead code in the plain lib
build, but *not* dead in the `--test` build if a `#[cfg(test)] mod tests`
in the same file calls it — the test module is itself a caller there. A
bare `#[expect(dead_code)]` is therefore unfulfilled in the test build and
fails `unfulfilled_lint_expectations` under `-D warnings`.

The fix is `#[cfg_attr(not(test), expect(dead_code, reason = "…"))]` —
apply the expectation only in the non-test build, where the function is
genuinely dead. This is the only shape that is green in both build
variants.

The obligation does not end there: the moment a real production caller
lands, the wrapper must be removed in the same change. Leaving it in place
after the function is called for the first time to hide a lint would be
silently widening what the gate lets through.

## Why it matters here

`stamp` was written in PHASE-06 ahead of its first production caller
(`serve`, PHASE-10) specifically so the reducer table it belongs to could
be built and tested whole. The wrapper existed for four phases and was
removed in the same diff that added `serve`'s call to it — confirmed by
grep as the phase's own discharge evidence, not left as an afterthought.

## How to apply

- Landing a helper before its production caller exists: use
  `#[cfg_attr(not(test), expect(dead_code, reason = "..."))]`, never a bare
  `#[expect(dead_code)]`, whenever a `#[cfg(test)]` module in the same file
  also calls it.
- When the real caller lands, remove the wrapper in that same change — a
  phase adding a caller is exactly the phase that owes this cleanup, and
  its own notes should say so.
- `Cargo.toml`'s own `dead_code` lint comment documents this shape; treat
  any other spelling (a plain `#[allow]`, a bare `#[expect]`) as a defect.

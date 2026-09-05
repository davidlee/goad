# Cargo runs a test binary with the package root as its cwd, not the workspace root

Learned at slice 002, PHASE-01/PHASE-02, `review-plan.md` F-35.

## The fact

Before a workspace split, a single crate's package root and the repository
root are the same directory, so a test that resolves a relative path (a
config file, a fixture) by current working directory happens to work.
After the split, a test binary's cwd is `crates/<member>` — its own
package root — and any such relative-path assumption breaks silently, with
no import-path change anywhere near the break.

## Why it matters here

Nothing about the split's import-path rewriting touches this; it is a
different axis entirely; a file can be moved with a perfect
`git mv`-and-fix-imports pass and still break this way, because the break
is about *where the process runs from*, not what it imports.

## How to apply

- The general rule this codebase now follows: build a path from
  `env!("CARGO_MANIFEST_DIR")` joined with `../..` to reach the workspace
  root from any member's test binary, rather than relying on cwd.
- Before splitting (or otherwise re-rooting) a crate, grep its tests for
  relative-path literals and cwd-relative file access — each one is a
  latent break that compiles fine and fails only when actually run from
  the new location.
- `crates/goad-boundary`'s scans use exactly this `CARGO_MANIFEST_DIR` +
  `../..` rule to find `workspace.members` from any member's own test
  binary.

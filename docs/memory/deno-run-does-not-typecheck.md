# `deno run` does not typecheck; `deno check` does

Measured at slice 001, PHASE-08, on deno 2.9.4: a `.ts` file with a type error
runs under `deno run -A` to exit 0. Type checking on `run` has been off by
default since deno 1.23.

## Why it matters here

The TypeScript example backend (`examples/typescript/backend.ts`) was chosen
so that a type error in a backend is caught before it runs, and so that its
types can refuse what the host refuses (`never` members on the `Field` union).
None of that holds under `deno run` alone. The gate therefore runs
`deno check examples/typescript/backend.ts` as its seventh command
(`justfile`, `design.md` §9), and it is that command — not the run — that makes
the example's types a check.

## How to apply

- A gate that wants a `.ts` file's types checked runs `deno check <file>`.
  `deno run` and `deno test` both strip.
- The example is invoked with `-A` because a backend is a trusted user program
  (brief §14); do not present deno's permission prompts as a sandbox.

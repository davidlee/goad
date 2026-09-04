# goad

A programmable personal intervention shell — a native desktop app that owns
interaction, while a user-supplied backend owns all domain meaning. See
`docs/brief.md`.

## Before you act

**Read `docs/AGENTS.md` first.** It is the methodology: the slice lifecycle
(slice → design → plan → phase → execute → audit → close), what goes in which
file, and the review protocol. It is not optional, and it is not a summary of
this file — this file is the pointer.

`docs/specs/`, `docs/policy/` and `docs/adr/` are **governing canon**: normative,
and binding on you. Read whatever may be relevant before you write code or make
a design choice:

```zsh
ls ./docs/{specs,policy,adr}/*
```

Canon is amended only with explicit user endorsement, and during audit — never
mid-slice on your own initiative.

## The invariants

Five rules the design exists to hold. Breaking one is a design change, not a
refactor.

> This application intentionally does not understand the user's domain.

Before adding host functionality, ask: *could this behaviour live entirely in the
backend?* If yes, it belongs in the backend. No domain vocabulary — habit,
streak, journal, goal, site — appears in host types or module names, and a
boundary test greps for it.

> Protocol parsing is permissive. Internal representations are canonical.

Wire types accept what a backend plausibly sends; normalization is the only door
into the canonical types, and past that door nothing is unvalidated. An
*ambiguous* message fails rather than being guessed at — permissiveness is about
fields the host does not model, never about the meaning of fields it does.

> Do not narrow wire compatibility merely because the current renderer
> implements only a subset of admitted protocol capabilities.

The protocol is the contract; a renderer is one consumer of it. This is the
failure the project exists to avoid.

**A backend failure never takes the host down**, and never leaves it unable to
invoke the backend again. Every refusal is reported and says which side was
wrong.

**Strata run one way** (ADR-001): `src/semantics/` is pure — no clock, no
filesystem, no subprocess, no async runtime — and never names `src/shell/`.
`cargo test --no-default-features` is the compiler enforcing it, not a
convention.

## The authoritative documents

| for | read |
|---|---|
| what the product is for, and the protocol as briefed | `docs/brief.md` |
| why the code is shaped as it is | `docs/adr/` — one-way strata, single crate until triggered |
| the normative protocol contract | `docs/specs/` — SPEC-001, the host/backend interaction protocol: wire formats, scheduling, interaction identity, the process transport, the failure taxonomy |
| how work is done here | `docs/AGENTS.md` |

## Verifying

`just check` is the gate — build, both test tiers, the example typecheck, clippy
in **both** feature columns, and a format check. Nothing is green until it exits
0, and a matrix checked in one column is unchecked.

`just -n check` prints the sequence. The command block in
`docs/slices/001/design.md` §9 is canonical and the `justfile` mirrors it:
change §9 first, then the recipe.

## Environment

Nix devshell (`nix develop`, or direnv). Rust toolchain, and `claude` / `codex`
both bare and jailed — `jcl` / `jcx` for the confined ones. If something is
missing, stop and ask rather than installing it.

## Working here

- Correctness over speed. Ask rather than infer.
- No code without an accepted plan.
- Red / green / **refactor**.
- Stop and consult on anything the design did not settle.

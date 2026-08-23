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

## Environment

Nix devshell (`nix develop`, or direnv). Rust toolchain, and `claude` / `codex`
both bare and jailed — `jcl` / `jcx` for the confined ones. If something is
missing, stop and ask rather than installing it.

## Working here

- Correctness over speed. Ask rather than infer.
- No code without an accepted plan.
- Red / green / **refactor**.
- Stop and consult on anything the design did not settle.

# POL-NNN: The phase gate

**Status:** required
**Date:** 2026-09-05

> **DRAFT.** This is slice 002's draft of new canon, held in the slice folder
> under `docs/AGENTS.md` *"Canon that does not exist yet, or must change"*. It is
> **not canon**: it is numbered `POL-NNN` until promotion, it is the slice's
> working authority for the duration, and nothing outside slice 002 may cite it.
> It is promoted to `docs/policy/NNN-the-phase-gate.md` at audit, with explicit
> user endorsement, and recorded in `audit.md`'s Reconciliation table. If it is
> not promoted it is abandoned in writing.
>
> Drafted from `docs/templates/policy.md` rather than from `docs/templates/spec.md`:
> AGENTS.md names `draft-spec.md` because it assumes new canon is a
> specification, and this new canon is a policy. The rule that matters — new
> canon is drafted in the slice folder from its governing template, never
> written into `docs/` mid-slice — is honoured (review `F-24`).

## Statement

`just check` is the phase gate. A phase is not green until it exits 0. The gate
is the command block in **§Compliance** below, in that order; the `justfile`
mirrors it, and a change goes into this policy first and into the recipe second.
No command may be removed, weakened, or made conditional to get a phase past the
gate.

## Rationale

The gate's command block was canonical in `docs/slices/001/design.md` §9 — a
*closed slice's design*. `docs/AGENTS.md` says a design is a record of intent at
a point in time and must not be retro-fitted, so the only two ways to record a
gate change were to edit a closed design or to leave `CLAUDE.md` pointing at a
stale block. Slice 002 changes the gate — a workspace changes every command's
scope, and the crate split retires the feature matrix that produced the
two-column clippy line — which forced the choice into the open.

A gate whose definition lives in a document nobody may edit is a gate that drifts
silently. This policy is the fix: one home, editable, with the closed slice left
standing as the record of what slice 001 intended.

## Scope

Applies to every phase of every slice, and to any agent or human proposing to
land a change in this repository.

It does **not** cover: what the individual tests assert; the invariant checks
those commands run (they are tests, and live in the tree); or the dev-shell
toolchain, which is `flake.nix`'s.

## Compliance

The gate is **six commands**:

```
cargo build --workspace
cargo test --workspace
cargo test -p goad-semantics
deno check examples/typescript/backend.ts
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

**Do:** change this block, then mirror it in the `justfile`; check
`just -n check` prints the same sequence.

**Don't:** delete or `#[ignore]` a test, add an `allow`/`expect` attribute to
silence a lint, or drop a command from the recipe, in order to make a phase
green. Each of those converts a failing gate into a false one.

**What the third command is for, and what it is not.** `cargo test --workspace`
unifies Cargo features across every member it builds, so stratum 1 is compiled
*there* with whatever features the strata above it switch on in shared
dependencies. `cargo test -p goad-semantics` is the only command in the gate that
builds and runs stratum 1 with exactly the features its own manifest asks for.
Measured: in a two-member probe where member `b` enables `serde/derive` and
member `a` does not, `cargo build --workspace` and `cargo test -p b` succeed while
`cargo test -p a` fails `error[E0433]` on `serde::Serialize`.

It is **not** a purity check, and must not be described as one: a `tokio` entry
in stratum 1's manifest passes it cleanly (`research.md:806`). Stratum 1's purity
is held by the instruments listed under **Verification**, not by this command.

**There is no feature matrix.** The pre-split gate ran seven commands in two
feature columns, because an optional `shell` feature gated `tokio` and `toml`.
The split makes both unconditional dependencies of stratum 2 and absent from
stratum 1, so the feature has nothing left to gate, `--no-default-features` stops
being a distinct column, and the second clippy line — with its
`-A dead_code -A unreachable_pub` carve-out — goes with it. A new workspace
member adds no column back: it is built and linted by the same `--workspace`
commands as every other.

## Verification

The gate verifies itself: it exits 0 or it does not. What each command holds, and
what it does not, is the part that must not be overstated:

| what is held | by what | what it does not reach |
|---|---|---|
| ADR-001's direction rule at **crate edges** | Cargo resolution — a `goad-semantics` source naming `goad_shell` or `tokio` is `error[E0433]` | anything not expressed as a crate edge |
| a runtime-, renderer- or filesystem-shaped **dependency entry** in a stratum 1 or 2 manifest | the manifest allowlist test in `crates/goad-boundary` | versions, features, and what a permitted dependency does |
| a **direct `std` reach** for the filesystem, processes, sockets, threads, the environment or a clock, in stratum 1's sources | the stratum 1 purity scan in `crates/goad-boundary` | aliased or brace-grouped imports, and I/O performed on stratum 1's behalf by a permitted dependency |
| **domain vocabulary** in any member's sources or markup | the vocabulary scan in `crates/goad-boundary` | nothing a line-based scan cannot see (its own documented limits) |
| stratum 1 compiled with **its own** feature set | `cargo test -p goad-semantics` | which features those are — it rejects nothing |

The residue is real and is a **review obligation, not an enforced rule**: a
feature switched on in a shared dependency by stratum 2 or 3 unifies into stratum
1's build under `--workspace`, and no command in this gate rejects it. Adding a
feature to a dependency shared with stratum 1 is therefore a design decision, and
is argued in the slice that takes it.

## References

- `docs/adr/001-one-way-strata.md` — the direction rule this gate is one
  instrument of.
- `docs/slices/001/design.md` §9 — the pre-split block this supersedes as
  canon. Left untouched as the record of slice 001's intent.
- `docs/slices/002/design.md` §5.6 — the split, and the derivation of the six
  commands and the four instruments above.
- `CLAUDE.md` — points here for the gate (slice 002 `canon-delta.md` CD-5, CD-7).

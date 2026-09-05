# POL-001: The phase gate

**Status:** required
**Date:** 2026-09-05

## Statement

`just check` is the phase gate. A phase is not green until it exits 0. The gate
is the command block in **§Compliance** below, in that order; the `justfile`
mirrors it, and a change goes into this policy first and into the recipe second.
No command may be removed, weakened, or made conditional, and no failing check
may be converted into a passing one, in order to get a phase past the gate.

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

This policy is **the definition of the gate, and nothing more**. It covers four
things, and each is derived from a document that already exists — the derivation
is `docs/slices/002/design.md` §10 C-5:

1. which commands the gate is, and in what order;
2. that the `justfile` mirrors this policy rather than the other way round;
3. that the gate may not be removed, weakened, made conditional, or falsified,
   which is what "`just check` is the gate" means and without which this
   document states a list rather than a rule;
4. what each enforcement instrument the gate runs holds, and what it does not.

Because the gate runs on every phase, (1)–(4) bind every phase of every slice,
and any agent or human proposing to land a change here.

It does **not** cover: what the individual tests assert; the invariant checks
those commands run (they are tests, and live in the tree); the dev-shell
toolchain, which is `flake.nix`'s; or **lint discipline** — whether a particular
`#[expect]` is legitimate is the workspace lint table's business and the
deciding slice's design, not this policy's (review `F-16`, `F-36`).

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

**Don't:** delete or `#[ignore]` a test, drop or condition a command in the
recipe, or suppress a lint **in order to make a phase green**. Each converts a
failing gate into a false one, and the last three words are the rule: the defect
is the motive, not the mechanism.

**What that does *not* forbid**, because this repository's own design authorises
both and a blanket prohibition would contradict them (review `F-16`):

- a **site-local** `#[expect(lint, reason = "…")]` at the narrowest scope that
  works, argued where it is written, on code that genuinely cannot satisfy a
  lint — never `allow`, which is silent when it stops being true, and never a
  crate-level `[lints]` override;
- a **generated-code quarantine**: one module-scoped `#![expect(…, reason = …)]`
  over the lints a code generator's output trips, on the single module that
  wraps the generated tree.

Both are lint discipline, which §Scope puts outside this policy. The line
between them and the prohibition above is not the attribute — it is whether a
reason was written and whether anything but the gate would have been served by
omitting it.

**What the third command is for, and what it is not.** `cargo test --workspace`
unifies Cargo features across every member it builds, so stratum 1 is compiled
*there* with whatever features the strata above it switch on in shared
dependencies. `cargo test -p goad-semantics` is the only command in the gate that
builds and runs stratum 1 with exactly the features its own manifest asks for.
Measured: in a two-member probe where member `b` enables `serde/derive` and
member `a` does not, `cargo build --workspace` and `cargo test -p b` succeed while
`cargo test -p a` fails `error[E0433]` on `serde::Serialize`.

It is **not** a purity check, and must not be described as one: a `tokio` entry
in stratum 1's manifest passes it cleanly
(`docs/slices/002/research.md:806`). Stratum 1's purity is held by the
instruments listed under **Verification**, not by this command.

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
what it does not, is the part that must not be overstated.

**The count follows `docs/slices/002/design.md` §5.1's counting rule**, and no
document may merge its parts into a single number: **four ADR-001
instruments**, plus **the domain-vocabulary check**, plus **one residue
nothing enforces**.

**The four ADR-001 instruments.** They hold four different parts of ADR-001's
one-way-strata rule, and their sum is not "purity, enforced":

| what is held | by what | what it does not reach |
|---|---|---|
| ADR-001's direction rule at **crate edges** | Cargo resolution — a `goad-semantics` source naming `goad_shell` or `tokio` is `error[E0433]` | anything not expressed as a crate edge |
| a runtime-, renderer- or filesystem-shaped **dependency entry** in a stratum 1 or 2 manifest | the manifest allowlist test in `crates/goad-boundary` | versions, features, and what a permitted dependency does |
| a **direct `std` reach** for the filesystem, processes, sockets, threads, the environment or a clock, in stratum 1's sources | the stratum 1 purity scan in `crates/goad-boundary` | aliased or brace-grouped imports, and I/O performed on stratum 1's behalf by a permitted dependency |
| stratum 1 compiled with **its own** feature set, so the three above are checking a configuration that stands on its own | `cargo test -p goad-semantics` | which features those are — it **rejects nothing**, and is not a purity check |

**The domain-vocabulary check is not one of the four.** It holds a different
invariant — `CLAUDE.md`'s first, which is about vocabulary rather than about
direction — and it runs over every member rather than over stratum 1. No
compiler objects to a domain-named type, so none of the four sees it:

| what is held | by what | what it does not reach |
|---|---|---|
| **domain vocabulary** in any member's sources or markup | the vocabulary scan in `crates/goad-boundary` | nothing a line-based scan cannot see (its own documented limits) |

**The residue** is real and is a **review obligation, not an enforced rule**: a
feature switched on in a shared dependency by stratum 2 or 3 unifies into stratum
1's build under `--workspace`, and no command in this gate rejects it. Adding a
feature to a dependency shared with stratum 1 is therefore a design decision, and
is argued in the slice that takes it.

## References

- `docs/adr/001-one-way-strata.md` — the direction rule this gate is one
  instrument of.
- `docs/slices/001/design.md` §9 — the pre-split block this supersedes as
  canon. Left untouched as the record of slice 001's intent.
- `docs/slices/002/design.md` §5.1 and §5.6 — the split; the derivation of the
  six commands; and the counting rule this policy's Verification section uses
  unchanged — four ADR-001 instruments, plus the domain-vocabulary scan, plus
  one residue.
- `docs/slices/002/design.md` §10 C-5 — the derivation of this policy's scope,
  clause by clause.
- `CLAUDE.md` — points here for the gate (slice 002 `canon-delta.md` CD-5, CD-7).

# Canon delta — Slice 002

**Status: drafted, not applied.** Applied during audit and reconciliation, with
explicit user endorsement, and recorded in `audit.md`'s Reconciliation table.

Changes this slice makes to canon that **already exists**. New canon is drafted
in `draft-spec.md`, not here. One entry per affected document: the document, the
section, the change as it will be stated, and why.

CD-1 carries a user endorsement already (`design-log.md` 2026-09-05) because the
split is a canon event the design could not take on its own. Endorsement of the
*decision* is not endorsement of the *wording*; the wording is still promoted at
audit like every other entry here.

---

## CD-1 — ADR-002 is superseded

**Document:** `docs/adr/002-single-crate-until-triggered.md` → a new ADR
**Section:** whole document
**Kind:** supersession. ADR-002 states its own terms: "This ADR is superseded,
not amended, when the split happens."
**Endorsed:** 2026-09-05 (`design-log.md`) — the decision, not this text.

### Why

T1 fired and the split was taken (`design.md` §5.6, D1). ADR-002's Verification
section requires that a fired trigger be recorded in the slice's design, and
that a fired trigger *without* a split needs its own ADR. This is the other
branch: the split happened, so ADR-002 is retired.

### The change

A new ADR — `docs/adr/003-*.md` — recording:

- **The decision as taken:** three crates along the ADR-001 strata, plus one
  that sits above all of them. `crates/goad-semantics` (stratum 1),
  `crates/goad-shell` (stratum 2), `crates/goad` (stratum 3, the binary), and
  `crates/goad-boundary` — a test-only member that owns the workspace-wide
  invariant checks and depends on no other member, because a scan over every
  member's sources belongs to none of them. `[workspace.dependencies]`,
  `[workspace.lints]` inherited by every member with `lints.workspace = true`
  and no crate-level override, one root `clippy.toml`, fixtures at
  `tests/fixtures/`.
- **The four things ADR-002 deliberately did not decide**, now decided with the
  code in view, which is the condition ADR-002 set: crate names, the `crates/`
  directory, the workspace dependency table, and the fixture corpus location.
- **What the split bought beyond satisfying the trigger, and exactly where the
  claim stops:** ADR-001's direction rule is enforced by Cargo's resolution **at
  crate edges** — a `goad-semantics` source file naming `goad_shell` or `tokio`
  is `error[E0433]`, confirmed by negative control (`research.md` Thread 6).
  That is the consequence ADR-001's Verification section said was unavailable in
  a single crate, and it covers only the half a compiler can see. A
  `tokio.workspace = true` line in stratum 1's *manifest* still leaves `cargo
  build --workspace` at exit 0 (`research.md:806`), so the manifest is held by a
  test rather than by the compiler; and the domain-vocabulary scan is not made
  redundant at all, because no compiler objects to a type called `Habit`.
- **What the split cost:** measured, not estimated — 111 renames, 91
  byte-identical, one substantive file change, ~6 minutes to a green gate. The
  error-taxonomy split ADR-002 flagged as a real cost was two lines.
- **Where ADR-001's discipline actually slipped:** not in production code, but
  in the test layout, in two places, and the split is what found them. ADR-002's
  Negative consequences predicted the opposite distribution.

---

## CD-2 — ADR-002's stated reason for T1 was false

**Document:** the superseding ADR (CD-1)
**Section:** the trigger record
**Kind:** correction of a factual premise, carried forward rather than buried.

### Why

ADR-002 says Slint is expected to fire T1 "because a build-dependency with a
conditional `build.rs` cannot be gated as cleanly". That is measurably false,
verified three independent ways (`research.md` Thread 6, `design.md` §5.6): an
optional `[build-dependencies]` entry plus `#[cfg(feature = "ui")]` inside
`fn main()` resolves the `--no-default-features` normal+build graph to one node,
the crate itself, and compiles no Slint crate at all.

T1 fires anyway, on two grounds ADR-002 did not name:

1. **The dev-dependency.** Cargo forbids optional dev-dependencies, so the Slint
   testing harness cannot be gated, and ADR-001's own `--no-default-features`
   column goes 16 → 223 crates.
2. **The lint collision.** `include_modules!()` splices generated code into the
   crate's module tree and trips twelve of goad's restriction lints; Slint's own
   blanket allow covers no restriction lint. A single crate answers that only
   with a suppression inside the crate whose lint discipline is the point.

A superseded ADR stays readable. Leaving the false premise unmarked lets a
future slice reason from it — the mechanism it describes is genuinely available,
and someone will one day reach for it on ADR-002's authority.

---

## CD-3 — SPEC-001 has no rule at the glass

**Document:** `docs/specs/001-host-backend-protocol.md`
**Section:** §2 (scope) and a new requirement in the renderer-facing section
**Kind:** a gap, closed. Not a change to an existing requirement.

### Why

R-20 forbids silently dropping a view part *at normalization*, and gives the
reason: dropping it would render a view the backend did not author. But §2 puts
drawing out of scope, so the guarantee stops before the renderer — and the
renderer is exactly where the pressure to drop things lives. Slice 002 is the
first renderer, and the first opportunity for the failure CLAUDE.md invariant 3
names to occur in code rather than in argument.

The concrete case: a backend sends `{"kind":"markdown","value":"# Take a
break"}`. It is legal SPEC-001. `StyledText::from_markdown` rejects headings.
Nothing in canon says what happens next, and the cheap answer — refuse the view
— is the failure the project exists to avoid.

### The change

A new requirement, stated for renderers rather than for normalization:

> A renderer MUST NOT refuse a view because it cannot draw part of it. It draws
> what it can, and reports what it did not draw. A capability the protocol
> admits and the renderer does not implement is a renderer subset, never a
> narrowing of the contract.

And §2's out-of-scope wording adjusted: drawing remains out of scope; what a
renderer owes the protocol does not.

---

## CD-4 — the fixture directory path

**Document:** `docs/specs/001-host-backend-protocol.md`
**Section:** §7
**Kind:** a normative path, restated.

### Why

§7 names `tests/protocol/fixtures/` normatively. The split moves it to
`tests/fixtures/` so both crates can reach it. A file move on disk; a canon
change on paper, and exactly the kind that goes unnoticed because it looks
mechanical.

---

## CD-5 — the gate's canonical command block

**Document:** `docs/slices/001/design.md` §9 → new canon
**Section:** the command block, and the prose describing the two feature columns
**Kind:** canon creation. Needs its own endorsement, separate from CD-1's.

### Why

`CLAUDE.md` states that the block in `docs/slices/001/design.md` §9 is canonical
and the `justfile` mirrors it: change §9 first, then the recipe. But §9 is a
*closed slice's design*, and `docs/AGENTS.md` says a design is a record of
intent at a point in time and must not be retro-fitted.

Slice 002 changes the gate: a workspace changes every command's scope, and the
split *retires* the feature matrix rather than adding to it. The renderer adds no
column — it is a workspace member, built and linted by the same `--workspace`
commands as every other. Under the current arrangement recording that means
editing a closed slice's design, which the methodology forbids, or leaving
`CLAUDE.md` pointing at a stale block.

### The change

Promote the command block and its rationale into canon of its own — a policy
under `docs/policy/`, which is currently empty and is the natural home for a
verification obligation. `CLAUDE.md` then points there, `docs/slices/001/design.md`
§9 stands untouched as the record of what slice 001 intended, and the closed
slice stops being load-bearing.

The block the policy carries is the post-split one (`design.md` §5.6), which is
**six commands, not seven**, and has no feature matrix:

```
cargo build --workspace
cargo test --workspace
cargo test -p goad-semantics
deno check examples/typescript/backend.ts
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

`cargo test -p goad-semantics` is inside the gate rather than beside it as a
diagnostic, and its job is narrow, measured, and not the one first claimed for
it. `cargo test --workspace` unifies Cargo features across every member it
builds, so stratum 1 is compiled *there* with whatever features stratum 2 and
stratum 3 switch on in shared dependencies. `-p goad-semantics` is the only
command in the gate that builds and runs stratum 1 with exactly the features its
own manifest asks for. Measured: in a two-member probe where member `b` enables
`serde/derive` and member `a` does not, `cargo build --workspace` and `cargo test
-p b` succeed while `cargo test -p a` fails `error[E0433]` on
`serde::Serialize`. It is **not** a purity check — a `tokio` entry in stratum 1's
manifest passes it — and the manifest test is what holds that.

---

## CD-6 — the boundary test and `.slint`

**Document:** `CLAUDE.md` (project instructions), invariant 1
**Section:** "No domain vocabulary … and a boundary test greps for it"
**Kind:** a claim that becomes false unless the test grows.

### Why

`tests/protocol/boundary.rs` scans `.rs` only. The day slice 002 lands, markup
exists — component names, labels, placeholder text, tooltips — and it is where
domain vocabulary is *most* tempting, because it is the text a person reads.
`CLAUDE.md`'s claim would be false for exactly the files most at risk.

### The change

None to `CLAUDE.md`, if D13 and D17 are implemented — but D13's shape is not the
one this entry first assumed. `Scan` has no extension field and the walk
hard-codes `.rs` (`tests/protocol/boundary.rs:13-20`, `:116`), so "extend the
configuration, not the walk" is not available: the walk changes **once**, to
consult a configured extension set. The header's rule is about vocabulary, and
generalising it to file types was the error (review `F-8`).

Two further shape corrections, both from round 2:

- The scan is no longer configured per member by hand. It reads
  `workspace.members` from the root manifest and applies one scan template to
  every member it finds, so a new member cannot arrive unscanned. That closes
  R7 in this slice instead of deferring it to slice 004.
- The scans move out of `tests/protocol/` and into `crates/goad-boundary`, a
  test-only workspace member that depends on no other member (D17). Their old
  home made a stratum-1 test target scan stratum 2 and, once the renderer
  exists, stratum 3 — the upward reach the split exposed
  (`research.md:798-804`).

The same finding makes slice 001's tolerated text-scan follow-up reachable rather
than hypothetical. `code_of` truncates at the first `//` (`:181`), accepted
because nothing in `src/` contained one — and markup contains URLs. The comment
cut becomes string-literal aware for both languages here, rather than being
deferred a second time. It stays **line-based**, and D13 names what that costs.

This entry exists so that the alternative — narrowing `CLAUDE.md`'s claim instead
of growing the test — is a decision taken in the open. If D13 is dropped, this
becomes a `CLAUDE.md` amendment.

---

## CD-7 — `CLAUDE.md`'s "both feature columns"

**Document:** `CLAUDE.md` (project instructions), the Verifying section
**Section:** "clippy in **both** feature columns", and "a matrix checked in one
column is unchecked"
**Kind:** a claim that becomes false on the day the split lands.

### Why

Both sentences describe a matrix produced by the optional `shell` feature
(`Cargo.toml:25`, `:38`). The split makes `tokio` and `toml` unconditional
dependencies of stratum 2 and absent from stratum 1, so the feature has nothing
left to gate and `--no-default-features` stops being a distinct column
(`research.md:817-835`). The second clippy column goes with it, along with its
`-A dead_code -A unreachable_pub` carve-out, which was already inert against
today's code.

This is the entry most likely to be missed, because nothing fails when a document
describes a column that no longer exists — it just quietly instructs every future
agent to check something that cannot be checked.

### The change

Replace both sentences with a pointer to the promoted policy (CD-5) and an
accurate statement of what replaced the matrix. Three mechanisms, each with a
different job, and the wording must not merge them:

- **Cargo, at crate edges** — a stratum 1 source file naming `goad_shell` or
  `tokio` does not compile.
- **The manifest test** (`crates/goad-boundary`) — the only instrument that
  rejects a runtime, renderer or filesystem-shaped *dependency entry*, which no
  compiler and no source scan can see.
- **`cargo test -p goad-semantics`** — the only gate command that builds stratum
  1 with exactly the features its own manifest asks for, because `--workspace`
  unifies features across every member it builds. This one is not a purity
  check and must not be described as one.

`docs/AGENTS.md` and any slice document repeating the two-column wording are
checked for the same phrase at reconciliation.

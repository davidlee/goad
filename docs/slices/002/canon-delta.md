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

- **The decision as taken:** three crates along the ADR-001 strata.
  `crates/goad-semantics` (stratum 1), `crates/goad-shell` (stratum 2),
  `crates/goad` (stratum 3, the binary). `[workspace.dependencies]`,
  `[workspace.lints]`, one root `clippy.toml`, fixtures at `tests/fixtures/`.
- **The four things ADR-002 deliberately did not decide**, now decided with the
  code in view, which is the condition ADR-002 set: crate names, the `crates/`
  directory, the workspace dependency table, and the fixture corpus location.
- **What the split bought beyond satisfying the trigger:** ADR-001's direction
  rule is now enforced by Cargo's resolution rather than by a grep in
  `boundary.rs`. That is the consequence ADR-001's own Verification section
  said was unavailable in a single crate.
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

Slice 002 changes the gate — a workspace changes the commands, and the renderer
adds a column. Under the current arrangement that means editing a closed slice's
design, which the methodology forbids, or leaving `CLAUDE.md` pointing at a
stale block.

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
diagnostic: it is the only remaining executable statement that stratum 1 stands
up without the runtime, and a purity claim that is not run is not a claim.

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

None to `CLAUDE.md`, if D13 is implemented — but D13's shape is not the one this
entry first assumed. `Scan` has no extension field and the walk hard-codes `.rs`
(`tests/protocol/boundary.rs:13-20`, `:116`), so "extend the configuration, not
the walk" is not available: the walk changes **once**, to consult a configured
extension set. The header's rule is about vocabulary, and generalising it to file
types was the error (review `F-8`).

The same finding makes slice 001's tolerated text-scan follow-up reachable rather
than hypothetical. `code_of` truncates at the first `//` (`:181`), accepted
because nothing in `src/` contained one — and markup contains URLs. The comment
cut becomes string-literal aware for both languages here, rather than being
deferred a second time.

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

Replace both sentences with a pointer to the promoted policy (CD-5) and a
statement of what replaced the matrix: stratum 1's purity is now held by
`cargo test -p goad-semantics` plus a manifest-reading test, not by a feature
column. `docs/AGENTS.md` and any slice document repeating the two-column wording
are checked for the same phrase at reconciliation.

# Canon delta — Slice 002

**Status: drafted, not applied.** Applied during audit and reconciliation, with
explicit user endorsement, and recorded in `audit.md`'s Reconciliation table.

Changes this slice makes to canon that **already exists**. New canon is drafted
in the slice folder from its governing template, not here — this slice's new
canon is `draft-policy.md`, the phase gate, drafted from
`docs/templates/policy.md` (review `F-24`). One entry per affected document: the
document, the section, the change as it will be stated, and why.

Two promotions, kept separate: the entries below are applied to the documents
they name, and `draft-policy.md` is moved into `docs/policy/` under its own
endorsement. Neither implies the other.

CD-1 carries a user endorsement already (`design-log.md` 2026-09-05) because the
split is a canon event the design could not take on its own. Endorsement of the
*decision* is not endorsement of the *wording*; the wording is still promoted at
audit like every other entry here.

**Citations are expanded at promotion.** Every bare `research.md`, `design.md`,
`design-log.md` or `slice-002.md` reference below resolves against *this folder*
and stops resolving the moment the wording lands in `docs/adr/` or `CLAUDE.md`.
Each becomes a full repository-relative path — `docs/slices/002/research.md:806`,
not `research.md:806` — as the text is transcribed, and the promoter checks that
before the entry is applied. It is a small rule and it was missed once already,
in `draft-policy.md` (review `F-35`).

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
  test rather than by the compiler; a direct `std::fs` call needs no manifest
  entry at all and is held by a source scan; and stratum 1 is built with its own
  feature set by exactly one gate command, which is what makes the other three
  checks about a configuration that stands on its own. Those are the **four**.
  Separately, and holding a different invariant, the domain-vocabulary scan is
  not made redundant by any of them, because no compiler objects to a type
  called `Habit`. The ADR states the four instruments and their boundaries,
  states the vocabulary scan beside rather than among them, and does **not**
  claim their sum is "stratum 1's purity, enforced" (review `F-6`). CD-7 carries
  the counting rule in full.
- **What the split cost:** measured against the executed split, not the dry
  run's estimate (PHASE-09 repair, 2026-09-05) — **113** renames, **92**
  byte-identical (`R100`), plus 24 additions, 3 deletions and 4 modifications
  elsewhere in the diff, and one substantive file change
  (`review-plan.md` F-34…F-37, `plan-log.md` PL-13). The dry run's own figures
  — 111 renames, 91 byte-identical (`research.md:881`) — were measured on a
  tree this branch never had, exactly the trap PL-13 named for §5.1's artifact
  map; this entry had quoted them uncorrected. The error-taxonomy split
  ADR-002 flagged as a real cost was two lines.
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

## CD-5 — `CLAUDE.md` points at a closed slice's design for the gate

**Document:** `CLAUDE.md` (project instructions), the Verifying section
**Section:** "The command block in `docs/slices/001/design.md` §9 is canonical
and the `justfile` mirrors it: change §9 first, then the recipe."
**Kind:** an amendment to an existing document, repointing it at new canon.

### Why

`CLAUDE.md` makes a *closed slice's design* the canonical source of the phase
gate. `docs/AGENTS.md` says a design is a record of intent at a point in time and
must not be retro-fitted, so that pointer has always obliged a future agent to
either edit a closed design or leave `CLAUDE.md` stale.

Slice 002 forces the choice: a workspace changes every command's scope, and the
split *retires* the feature matrix rather than adding to it. The renderer adds no
column — it is a workspace member, built and linted by the same `--workspace`
commands as every other.

### The change

`CLAUDE.md`'s pointer moves from `docs/slices/001/design.md` §9 to the promoted
phase-gate policy under `docs/policy/`; `docs/slices/001/design.md` §9 stands
untouched as the record of what slice 001 intended, and the closed slice stops
being load-bearing.

### What this entry does *not* do

It does not create the policy. **New canon is not drafted here** — this file
covers changes to canon that already exists (see the preamble, and AGENTS.md
*"Canon that does not exist yet, or must change"*). The policy itself is drafted
as `docs/slices/002/draft-policy.md` and promoted separately, under its own
explicit endorsement, and it is the draft that carries the six-command block, the
rationale for `cargo test -p goad-semantics`, and the statement of what each
enforcement instrument does and does not hold. Applying this entry without
promoting that draft would leave `CLAUDE.md` pointing at nothing; promoting the
draft without applying this entry would leave two claimants to the gate. Both
moves land, or neither does, and each is recorded in `audit.md`'s Reconciliation
table on its own row (review `F-24`).

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

Replace both sentences with a pointer to the phase-gate policy — drafted as
`draft-policy.md` and promoted under CD-5's neighbour endorsement — and an
accurate statement of what replaced the matrix. That statement is **three
things, not one number** — the counting rule of `design.md` §5.1, used
identically in `design.md` §5.6, §9 item 3 and §10 C-7, in `slice-002.md` AC-3
and in `draft-policy.md`'s Verification section (review `F-6`, fourth raising):

> **Four ADR-001 instruments**, plus **the domain-vocabulary scan**, which holds
> a different invariant, plus **one residue nothing enforces.**

The wording must not merge them or claim their sum is "purity enforced". The
four ADR-001 instruments, each with a different job:

- **Cargo, at crate edges** — a stratum 1 source file naming `goad_shell` or
  `tokio` does not compile.
- **The manifest allowlist test** (`crates/goad-boundary`) — the only instrument
  that rejects a runtime, renderer or filesystem-shaped *dependency entry*,
  which no compiler and no source scan can see. It reads names, not versions and
  not features.
- **The stratum 1 purity scan** (`crates/goad-boundary`) — the only instrument
  that rejects a *direct* `std` reach for the filesystem, processes, sockets,
  threads, the environment or a clock, none of which needs a manifest entry. A
  line-based source scan with the limits its own design names.
- **`cargo test -p goad-semantics`** — the only gate command that builds stratum
  1 with exactly the features its own manifest asks for, because `--workspace`
  unifies features across every member it builds. This one is not a purity
  check and must not be described as one.

**And the domain-vocabulary scan** (`crates/goad-boundary`), stated separately
because it is **not** one of the four: it holds `CLAUDE.md`'s *own* first
invariant — no domain word in any crate name, module, type, markup component,
accessible label or user-visible string — rather than ADR-001's direction rule,
and it runs over every member rather than over stratum 1. CD-6 is the entry that
grows it to `.slint`. Merging it into the four is how three documents ended up
with three different counts.

**And one residue**, stated as residue rather than omitted: a feature switched on
in a shared dependency by stratum 2 or 3 unifies into stratum 1's build under
`--workspace`, and nothing in the gate rejects it. That is a review obligation,
and `CLAUDE.md` says so rather than implying the instruments are exhaustive.

`docs/AGENTS.md` and any slice document repeating the two-column wording are
checked for the same phrase at reconciliation.

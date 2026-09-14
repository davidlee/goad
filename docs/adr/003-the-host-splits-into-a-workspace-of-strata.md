# ADR-003: The host splits into a workspace of one crate per stratum

**Status:** accepted
**Date:** 2026-09-05

## Context

ADR-002 kept the host as a single crate until one of three triggers fired, and
named T1 — a dependency stratum 1 must not need in order to build — as the one
Slint was expected to fire in slice 002. It gave a reason: "a build-dependency
with a conditional `build.rs` cannot be gated as cleanly" as the async runtime
was gated behind a feature.

That reason is false, verified three independent ways
(`docs/slices/002/research.md` Thread 6). An optional `[build-dependencies]`
entry plus `#[cfg(feature = "ui")]` inside `fn main()` resolves the
`--no-default-features` normal-plus-build graph to one node — the crate itself —
and compiles no Slint crate at all. The mechanism ADR-002 described as
unavailable is genuinely available.

T1 fired anyway, on two grounds ADR-002 did not name:

1. **Cargo forbids optional dev-dependencies.** The Slint testing harness
   (`i-slint-backend-testing`) cannot be gated behind a feature — dev-dependencies
   are built for `cargo test` regardless of features or `required-features` — so
   `cargo test --no-default-features`, the gate ADR-001's Verification section
   names, goes from 16 crates to 223 (`research.md` Thread 6). The one untested
   escape hatch, `[target.'cfg(feature = "ui")'.dev-dependencies]`, is not one:
   Cargo warns it "will not work as expected" and the dependency resolves in
   neither column.
2. **The lint collision.** `include_modules!()` splices generated code into the
   crate's module tree and trips twelve of goad's restriction lints. Slint's own
   blanket allow covers no restriction lint, so a single crate answers this only
   with a suppression inside the crate whose lint discipline is the point.

A superseded ADR stays readable, and a false premise left unmarked invites a
future slice to reason from it — the mechanism ADR-002 describes genuinely
works, and someone will one day reach for it on ADR-002's authority.

What was measured, at the split and after: warm `just check` went **1.808 s**
pre-split to **1.797 s** at the split itself (`docs/slices/002/notes.md` EN-3,
VA-1) — the split cost nothing, measured before Slint entered the graph — then
**2.128 s** once Slint and its element tree landed (`notes.md`, PHASE-03), and
**5.276 s** on the finished tree (`notes.md`, PHASE-09). All three post-split
figures sit inside ADR-002's own A-4 band of **≤ 120 s**; T3 (headless test
wall-clock dominated by renderer build time) has not fired.

The split ADR-002 deferred was executed as a relocation: **113 files renamed**,
**92 of them byte-identical** (`R100`), plus 24 additions, 3 deletions, 4
modifications elsewhere in the diff, and one substantive file change
(`docs/slices/002/review-plan.md` F-34…F-37, `docs/slices/002/plan-log.md`
PL-13). The dry run's own figures — 111 renames, 91 byte-identical
(`research.md:881`) — were measured on a tree this branch never had; that
mismatch is the same trap PL-13 named for the design's artifact map, not a
second finding.

## Decision

We will organise the host as a Cargo workspace of members, following
ADR-001's strata — four at this decision, five since slice 005:

- **`crates/goad-semantics`** — stratum 1, the pure semantic core.
- **`crates/goad-shell`** — stratum 2, the I/O shell.
- **`crates/goad`** — stratum 3, the Slint renderer and the host binary.
- **`crates/goad-emit`** — stratum 3, the `goad-emit` binary. Added by slice
  005 under this ADR's own rule rather than against it: stratum 3 holds entry
  points, and a second binary is a second entry point. See Alternatives.
- **`crates/goad-boundary`** — no stratum. A test-only member that owns the
  workspace-wide invariant checks (the manifest allowlist, the stratum 1 purity
  scan, and the domain-vocabulary scan) and depends on no other member, because
  a scan over every member's sources belongs to none of them.

Shared dependency versions live in `[workspace.dependencies]`; every member
inherits the shared lint table via `lints.workspace = true` with no crate-level
override; there is one root `clippy.toml`; the shared fixture corpus lives at
`tests/fixtures/`, addressed from each member's tests by a relative path.

These four choices are exactly the ones ADR-002 deliberately left open pending
the split — crate names, the `crates/` directory, the workspace dependency
table, and the fixture corpus location — and they are decided here with the
code in view, which is the condition ADR-002 set.

**What the split buys, and exactly where the claim stops.** ADR-001's direction
rule is now enforced by Cargo's own resolution **at crate edges**: a
`goad-semantics` source file naming `goad_shell` or `tokio` is `error[E0433]`,
confirmed by negative control (`research.md` Thread 6). That is the compiled
consequence ADR-001's Verification section said a single crate could not give.
It covers only the half a compiler can see. A `tokio.workspace = true` line
added to stratum 1's *manifest* still leaves `cargo build --workspace` at exit 0
(`research.md:806`), so the manifest is held by a test, not the compiler; a
direct `std::fs` call needs no manifest entry at all and is held by a source
scan; and stratum 1 is built with its own feature set by exactly one gate
command, `cargo test -p goad-semantics`, which is what makes the other three
checks about a configuration that stands on its own. Those are the **four**
ADR-001 instruments, each holding a different part of the rule and none holding
all of it.

Separately, and holding a different invariant, the domain-vocabulary scan is
not made redundant by any of the four — no compiler objects to a type called
`Habit`. It runs over every workspace member rather than over stratum 1 alone,
and it is stated beside the four instruments, not among them: their sum is not
"stratum 1's purity, enforced."

**Where ADR-001's discipline actually slipped.** Not in production code — the
split moved every production file with nothing beyond an import-path change —
but in the test layout, in two places the split is what found:
`tests/protocol/transport_shape.rs` sat in the stratum-1-shaped test target with
a stratum-2 source file as its subject, and the boundary scan itself sat in the
same target, scanning all of `src/`. Both were upward reaches the single crate
hid. ADR-002's Negative consequences predicted the opposite distribution — cost
concentrated in production code and the error taxonomy.

**What the split cost.** Measured against the executed split, not the dry run's
estimate: 113 renames, 92 byte-identical, 24 additions, 3 deletions, 4
modifications, one substantive file change. The error-taxonomy split ADR-002
flagged as a real cost was two lines — the taxonomy was already split by
stratum, and the stratum-2 file already wrapped the stratum-1 one across a named
seam.

## Alternatives considered

- **Stay single-crate, with the renderer behind a Cargo feature.** ADR-002's own
  fallback if the split were declined. Rejected: it requires an optional
  build-dependency and a conditional `build.rs`, which was never the obstacle —
  the obstacle was the dev-dependency and the lint collision, neither of which a
  feature gate solves inside one crate.
- **A fifth crate, one per stratum-3 concern.** Rejected, and **still
  rejected**: splitting the renderer from the host binary would be structure
  drawn around an imagined shape rather than an observed one — the same
  objection ADR-002 raised against a workspace from the outset. What has
  changed is the ground, not the verdict. This entry originally rested on "T2
  (a second binary) has not fired"; T2 fired in slice 005, and was answered by
  adding a member for the new binary, not by splitting `crates/goad`. An
  alternative left standing on a premise that has since gone false is the trap
  this ADR's own Context describes ADR-002 laying, so the premise is corrected
  here rather than left to be read as current.

## Consequences

### Positive

- ADR-001's direction rule is now partly a compile error rather than only a
  review gate, at crate edges.
- The four things ADR-002 deferred are chosen once, with the code in view,
  rather than guessed in advance.
- The split was cheap: six minutes from the first rename to a green gate
  (`research.md` Thread 6), and the measured cost above confirms it stayed
  cheap through the finished slice.

### Negative

- The four ADR-001 instruments plus the vocabulary scan are not exhaustive.
  A feature switched on in a shared dependency by stratum 2 or 3 unifies into
  stratum 1's build under `--workspace`, and nothing in the gate rejects it.
  This is a review obligation, not an enforced rule, and it did not exist as a
  named risk before the split created `--workspace` builds to unify against.
- `goad-boundary` is a further thing to keep in view when a member is added,
  though not in the way this ADR first stated. Measured at slice 005's audit,
  against a real new member: the **domain-vocabulary scan** reads
  `workspace.members` for itself, so a new member arrives covered with no edit;
  and no stratum-3 member carries a **manifest allowlist** row at all, because
  POL-001 §Verification scopes that instrument to "a stratum 1 or 2 manifest".
  The real residue is narrower and sharper: **a stratum-3 manifest is checked
  by nothing but review.** Stratum 3's freedom from the renderer is a fact
  about its own dependency table — one line added there dissolves it silently,
  as has been true of `crates/goad` since this ADR and of `crates/goad-emit`
  since 005. An instrument for it is a live Follow-up, not a rule in force.
- The manifest allowlist test and the stratum 1 purity scan are both line- and
  name-based. Neither claims to see an aliased import, a brace-grouped `use`, or
  I/O performed on stratum 1's behalf by a permitted dependency.

### Neutral

- `cargo test -p goad-semantics` is not a purity check and rejects nothing. It
  exists so the other three instruments check a configuration that stands on
  its own, because `cargo test --workspace` unifies features across members.

## Verification

The four instruments and the vocabulary scan are enforced in the gate `just
check` runs; what each holds and does not hold is stated in
`docs/policy/001-the-phase-gate.md` (Verification), not restated here. The
residue named under Negative consequences is a review obligation and has no
test.

## References

- ADR-001 (one-way strata), which this split gives a partial compiler behind.
- ADR-002 (superseded by this ADR).
- `docs/slices/002/research.md` Thread 6 — the refutation of ADR-002's stated
  reason for T1, and the measured cost of the split.
- `docs/slices/002/design.md` §5.1 (the workspace layout and the counting rule)
  and §5.6 (the split's derivation).
- `docs/policy/001-the-phase-gate.md` — the gate this split's instruments run
  inside.

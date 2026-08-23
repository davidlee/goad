# ADR-002: The host stays one crate until a renderer or a second binary arrives

**Status:** accepted
**Date:** 2026-08-23

## Context

ADR-001 fixes the host's internal strata but leaves open how many crates they
occupy. That question is live now because slice 001 is about to create the
first `Cargo.toml`, and it is easier to start as a workspace than to become one
by accident.

What was measured on 2026-08-23 (slice 001 `research.md`): Slint resolves **411**
unique dependencies and takes roughly 19 seconds for a clean debug build of a
hello-world binary, with a `build.rs` codegen step on every build. That is the
cost a single crate would eventually impose on every headless protocol test.

What is known about the near future: slice 001 is headless and adds no renderer
at all. Slice 002 adds the Slint GUI. Slice 004 adds a `goad emit` command-line
binary. Slice 005 adds the socket transport. So the cost above is entirely
hypothetical for the duration of slice 001, and becomes real in slice 002.

What is true now that will not be true later: there is no code. Boundaries drawn
today would be drawn around an imagined shape rather than an actual one, and
splitting a slice-001-sized crate is a move of a handful of files.

## Decision

We will build the host as a single crate for as long as none of the following
holds, and split it into a workspace along the ADR-001 strata as soon as any one
of them does:

- **T1** — a dependency is required that stratum 1 must not need in order to
  build. Slint is the first such dependency, expected in slice 002.
- **T2** — a second binary is required. `goad emit` is expected in slice 004.
- **T3** — headless test wall-clock becomes dominated by renderer build time.

Until then the strata are modules within the one crate. The split must be a
relocation of files, not a redesign; if it cannot be, ADR-001 was not being
honoured and that is the finding, not the split.

Crate names, a `crates/` directory, the workspace dependency table, and the
location of the shared fixture corpus are deliberately not decided here. They
are chosen when the split happens, with the code in view.

## Alternatives considered

- **A workspace from the outset.** Rejected: a workspace whose only member is a
  crate that does not exist yet is structure without content, and it fixes
  boundary placement at the moment of least information. The cost it avoids is
  real but does not arrive until slice 002.
- **A single crate permanently, with the renderer behind a Cargo feature.**
  Rejected as the standing position, retained as the fallback if the split is
  declined when T1 fires. It requires an optional build-dependency and a
  conditional `build.rs`, which keeps stratum 1 renderer-free only by vigilance,
  and it leaves `goad emit` linking the whole tree unless it too is gated.
- **Split at a fixed slice number rather than on a trigger.** Rejected: the
  triggers state the actual reason for splitting, so they stay correct if the
  slice order changes.

## Consequences

### Positive

- Slice 001 ships without workspace ceremony, and its code is identical to what
  a workspace would have contained.
- The triggers name an observable condition rather than a date, so the decision
  to split is not a judgement call taken under whatever pressure slice 002
  brings.
- Deferring crate names and directory shape means they are chosen once, with
  evidence, rather than guessed and then lived with.

### Negative

- Between now and the split, ADR-001's one-way rule has no compiler behind it.
  This is the whole risk of the decision, and it is accepted on the basis that
  T1 is expected to fire in the very next slice.
- Whoever performs the split pays a cost that would have been near-zero today:
  splitting the error taxonomy, relocating tests, deciding where fixtures live.
  The bet is that paying it with the code in front of you is cheaper than
  guessing now, not that it is free.
- A trigger can fire without anyone noticing, particularly T3, which is a
  gradual condition rather than an event.

### Neutral

- Nothing here changes what slice 001 builds. It changes only whether the
  strata are enforced by the compiler or by review.

## Verification

At the start of each slice that adds a dependency or a binary, check the three
triggers explicitly and record the answer in that slice's design. A fired
trigger with no split is a decision requiring its own ADR superseding this one.

This ADR is superseded, not amended, when the split happens.

## References

- ADR-001, which defines the strata the split would follow.
- `docs/slices/001/research.md` — the Slint dependency count and build timing.
- `docs/slices/001/slice-001.md` OQ-2, which raised it.
- `docs/brief.md` §4.1 (Rust and Slint), §15 (repository layout for agents).

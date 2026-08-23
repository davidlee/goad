# ADR-001: Host code flows one way through three strata

**Status:** accepted
**Date:** 2026-08-23

## Context

The host has no code yet, so every structural convention it will have is about
to be set by default rather than by decision.

Two forces make the internal shape a decision rather than an obvious step.

The brief draws a line through the host at normalization: permissive at the wire,
canonical after it (§3.3). Everything upstream of that line is I/O against
untrusted input; everything downstream is pure manipulation of canonical values.
That line is real regardless of how the code is packaged.

The brief also makes coding agents the intended editors of this repository
(§3.7). A boundary that exists only as a convention is crossed by whoever did
not read the convention, and the cost is paid later by whoever has to
disentangle it. This raises the value of stating the boundary explicitly, and of
being able to make a compiler enforce it later.

Concretely, the host will accumulate: protocol types and normalization,
schedule resolution, backend transport, configuration, host operational state,
event ingress, a Slint renderer, and at least one command-line binary. Left
unstated, those would form a mesh.

## Decision

We will organise host code into three strata, and dependencies will flow in one
direction only:

1. **Pure semantic core** — protocol types, wire-to-canonical normalization,
   schedule resolution. No I/O and no async runtime.
2. **I/O shell** — backend transport, configuration, host operational state,
   event ingress.
3. **Entry points** — the Slint renderer, command-line binaries.

A stratum may depend on a lower-numbered stratum. No stratum may depend on a
higher-numbered one. Stratum 1 in particular must remain buildable and testable
with no renderer and no runtime in its dependency graph.

A reader can check compliance by asking, of any import: does this arrow point
downward?

## Alternatives considered

- **Leave the internal shape to emerge.** Rejected: the normalization boundary
  in brief §3.3 is already known, and the set of components above is already
  known from the brief's phase list. Discovering a boundary that was legible in
  advance is not emergence, it is rework.
- **Two strata — pure core plus everything else.** Rejected: it puts the
  renderer and the transport in the same layer, so nothing prevents transport
  code from reaching into the renderer. The renderer is the component most
  likely to attract dependencies, which is the reason to keep it at the leaf.
- **Hexagonal / ports-and-adapters with inverted dependencies.** Rejected as
  disproportionate. It buys substitutability the host does not need: there is
  one renderer, and the transport already has an abstraction of its own because
  the brief requires two implementations (§6). Adding a second indirection would
  be structure without a beneficiary.

## Consequences

### Positive

- Stratum 1 stays cheap to test. Protocol and scheduling tests need neither a
  renderer nor a runtime, so the fastest-moving code has the fastest feedback.
- The error taxonomy gets a natural seam: parse and validation errors belong to
  stratum 1, transport errors to stratum 2 wrapping stratum 1's. This forecloses
  the single flat enum spanning both that slice 001 AC-6 would otherwise invite.
- The strata are a ready-made crate split if one is ever wanted, without a
  redesign.

### Negative

- Until and unless the strata become separate crates, the rule is enforced by
  review alone. An agent that does not read this ADR can violate it and the
  build will pass.
- Placement questions will arise that the three names do not settle on their
  own — host operational state and `view_id` generation are stratum 2, but the
  *types* they use are stratum 1, and that is a distinction someone has to make
  deliberately each time.
- Keeping stratum 1 runtime-free may occasionally cost an awkward signature,
  where a synchronous pure function is threaded through async calling code
  rather than simply being async itself.

### Neutral

- This says nothing about crate count, directory layout, or module naming. Those
  are separate questions; see ADR-002.

## Verification

By inspection during review, and by the placement discipline recorded in each
slice's design.

If the strata ever become separate crates, verification becomes mechanical: the
crate dependency graph either points one way or does not compile. Until then,
the honest answer is that this is a review gate, not a build gate.

## References

- `docs/brief.md` §3.3 (permissive wire, canonical internals), §3.7
  (agent-modifiability as a product requirement), §6 (two backend transports).
- `docs/slices/001/slice-001.md` OQ-2, which raised it.
- ADR-002, which decides how many crates the strata occupy today.

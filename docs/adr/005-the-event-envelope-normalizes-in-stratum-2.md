# ADR-005: The event envelope normalizes in stratum 2

**Status:** accepted
**Date:** 2026-09-10

## Context

ADR-001 fixes three strata and the direction they run in. Its §Decision assigns
work to them by name, and two of those names both apply to the event envelope
SPEC-003 admits:

> *wire-to-canonical normalization* is stratum 1's, and *event ingress* is
> stratum 2's.

The envelope is both. It arrives on a wire, it is permissive at that wire and
canonical past it, and it arrives through an ingress. So the names settle
nothing, and ADR-001 §Consequences says exactly that: where two assignments
reach for the same work, the placement is a decision to be made deliberately
and recorded, not one to be inferred from whichever name is read first.

Two precedents already exist in the workspace for the same shape —
permissive-in, canonical-out through a single function. `protocol/wire.rs` →
`protocol/normalize.rs` is stratum 1's, and `config.rs`'s `File` → `Config` is
stratum 3's. The shape does not decide the stratum; something else has to.

There is also a rule that looks like it decides this and does not.
`CLAUDE.md`'s second invariant says normalization is *the only door* into the
canonical types, and SPEC-003 normalizes an envelope into a canonical `Event`
outside stratum 1. Read carelessly, that forbids this decision outright.

## Decision

**The event envelope's permissive type and its normalization live in stratum 2,
beside the listener** — `crates/goad-shell/src/ingress/envelope.rs`. Stratum 1
gains nothing from SPEC-003.

**What decides it is which contract a normalization holds, not which shape it
has.** Stratum 1's normalization holds **one** contract — SPEC-001, the
host/backend protocol — in one place, so that a second host implementation is
held to the same normalization of the same wire. That is the property worth
protecting, and it is a property of stratum 1 holding *one* wire.

The envelope is a different contract, with different parties, that no backend
ever sees: a user-owned watcher writes it to a local socket. Putting it in
stratum 1 would make the protocol crate the home of two unrelated wires and give
`goad-semantics` a reason to change whenever the socket's contract does — a
coupling between two contracts that have no reason to move together.

**On the "only door" invariant.** It is a rule about the **protocol's** canonical
values, and the chain `protocol/wire.rs` → `protocol/normalize.rs` →
`protocol/canonical.rs` remains the sole admission point for anything a backend
sends. SPEC-003 does not touch it. `Event` itself holds no invariant a
constructor could enforce: it is a transparent record of four `pub` fields —
`source` and `kind` are `String`, `data` is opaque by SPEC-001/R-9, and the one
field that can fail to be canonical is `timestamp`, whose canonicality is
carried entirely by `jiff::Timestamp`, a type stratum 2 may already name.
Constructing an `Event` outside stratum 1 is also already the status quo:
stratum 3 has built one for every host-originated evaluation since slice 002.
So this adds a second **producer** of a transparent record, not a second gate on
a guarded one.

## Alternatives considered

**Stratum 1, beside SPEC-001's normalization.** Rejected on coupling, above. It
also inverts the reason stratum 1 exists: it is the stratum a second
implementation must reproduce, and a second implementation of the *host* has no
obligation to implement SPEC-003's socket at all — SPEC-003/R-1 makes the
listener conditional on configuration.

**Stratum 3, beside `config.rs`'s `File` → `Config`.** Rejected because the
listener is stratum 2's by ADR-001's own name for it, and splitting a contract's
wire type from the code that reads that wire puts the two on opposite sides of a
stratum edge for no gain.

**Reusing SPEC-001's `wire.rs` types for the envelope.** Rejected: the two wires
share no field set, and a shared permissive type would make each contract's
admissions the other's problem.

## Consequences

### Positive

- The two contracts move independently. `goad-semantics` has no reason to change
  when the socket's contract does.
- `goad-shell` owns SPEC-003 end to end — the socket, the envelope, the
  normalization and the refusal vocabulary — so the contract is legible in one
  crate.
- Stratum 1 stays the thing a second implementation must reproduce, and it
  contains exactly the wire a second implementation is obliged to speak.

### Negative

- **The offset rule is now stated twice.** SPEC-001/R-22 governs a backend's
  instant and SPEC-003/R-10 the envelope's, and two statements of one rule can
  drift. R-10 is written to mirror R-22 in terms, and both parse through the
  same two-step `jiff` sequence, which is mitigation and not prevention.
- A reader looking for "where wire types are normalized" now finds two places,
  and must know which contract they are asking about. §9 of both specs names the
  other.

### Neutral

- **No ADR-001 instrument sees this choice.** Crate edges see only crate edges,
  the manifest allowlist only dependency entries, the purity scan only stratum
  1's reaches into `std`, and `cargo test -p goad-semantics` rejects nothing by
  itself (`docs/policy/001-the-phase-gate.md` §Verification). The placement is
  held by this record and by review, not by the gate. That is stated rather than
  assumed, because the gate's silence here is easy to mistake for approval.

## Verification

There is nothing to test: the decision is where code lives, and the four
ADR-001 instruments do not reach it (§Consequences, Neutral). What holds it is
this record, `docs/policy/001-the-phase-gate.md` §Verification stating what each
instrument does *not* reach, and the fact that the alternative — stratum 1
importing a second wire — would be visible in `goad-semantics`'s own module list
to anyone reading it.

The one consequence that **is** tested is that the door invariant was not
breached: `crates/goad-semantics`'s normalization chain is unchanged by slice
004 except for one visibility keyword, and `cargo test -p goad-semantics` runs
stratum 1 with its own feature set.

## References

- ADR-001 (one-way strata) — §Decision, which names both sides of this question;
  §Consequences, which requires the case be decided deliberately when it arises.
- ADR-003 (the host splits into a workspace of strata) — the crate boundaries
  this decision places work within.
- SPEC-003 (host event ingress) — the contract that normalizes here; §9's
  ADR-001 entry states the same placement from the spec's side.
- SPEC-001 (the host/backend interaction protocol) — R-7, R-9 and R-22, the
  canonical `Event` this normalization produces and the offset rule R-10
  mirrors.
- `docs/policy/001-the-phase-gate.md` §Verification — what each of the four
  ADR-001 instruments holds, and what none of them reaches.
- `docs/slices/004/design.md` §2 F6 (what decides the placement), §5.1 (why
  normalizing into `Event` from stratum 2 is not a second door), §10 D-3.
- `CLAUDE.md` — the "normalization is the only door" invariant this record
  reads narrowly, and says why.

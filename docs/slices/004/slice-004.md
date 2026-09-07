# Slice 004: Event ingress

**Stage:** design
**Tier:** 2 (full). It **opened tier 1 and raised at scoping**, as
`docs/roadmap.md` §004 said it would: deciding how an event-triggered evaluation
is bounded amends SPEC-002, and reserving an event source amends SPEC-001. Both
are canon, and `docs/AGENTS.md` §Tiers requires the full lifecycle of a slice
that amends canon. A tier is never lowered.
**Depends on:** 003 — the evaluation path, `serve`'s loop, and the anchor this
slice must not disturb.

## Purpose

Nothing outside goad can make it ask its backend anything. The host evaluates
when it starts, when a person asks, and when a check comes due, and that is the
whole list. A user-owned watcher that has decided *something interesting just
happened* has nowhere to say so.

This slice gives it one: a Unix domain socket that accepts an opaque event
envelope and turns it into an `evaluate` on the path slice 003 built. The
watcher writes JSON, the host forwards the envelope verbatim, and the backend
decides whether anything should appear on screen. The host learns nothing about
what the event meant — brief §7's whole point is that filtering, classification
and debouncing already happened in the user's own code.

Once it lands, brief §19's scenario runs end to end for the first time: a
watcher emits `{"source": "reddit-watcher", "kind": "reddit-opened", …}`, and a
prompt the host does not understand appears.

## Scope

The boundary is *the envelope and the socket*. Everything past the envelope is
carried, not read.

**Stratum 2 — `crates/goad-shell/src/`.** A new ingress module: the listener,
the accept path, the permissive envelope wire type and its normalization, and
the reply. ADR-001 names event ingress in stratum 2 in terms, so the placement
is canon rather than a choice.

**`crates/goad-shell/src/config.rs`.** One new optional section carrying the
socket path. Permissive at the file, canonical after — the same split the module
already uses, and `deny_unknown_fields` means the canonical form must know the
key before any config may write it.

**Stratum 3 — `crates/goad/src/`.** `main.rs` and `startup.rs` bind the socket
before the event loop starts, and carry the new startup failures.
`controller.rs`'s `serve` gains an ingress arm on its `select!`, and the event
spacing and its anchor. `wire.rs`'s `Command`/`Stimulus` must be able to express
an evaluation whose `Event` the host did not author — today `Stimulus` can only
name one it did.

**Stratum 1 — `crates/goad-semantics/`.** `Event` already exists there. Whether
the envelope's normalization joins it or stays in stratum 2 is design's. What is
fixed: **stratum 1 gains no dependency**, and the four ADR-001 instruments and
the vocabulary scan pass unchanged.

**Canon.** `canon-delta.md` CD-1 (SPEC-002, the event bound) and CD-2
(SPEC-001/R-56, the reserved source). Neither spec is edited before
reconciliation.

**Tests and examples.** Listener behaviour in `goad-shell`; loop and end-to-end
behaviour in `crates/goad/tests/renderer/`; a documented shell one-liner that
emits an envelope, which is this slice's emitter.

## Non-goals

- **`goad emit`.** Slice 005, and it fires ADR-002's T2. A shell one-liner is
  this slice's client; if 005 needs a 300-line design, this slice's socket was
  wrong.
- **The socket *backend transport*** (brief §6.1) — slice 008. That is a
  different socket, in the other direction, under a different contract. The two
  share a word and nothing else, and conflating them is the obvious trap here.
- **An XDG default socket path.** Roadmap §006 owns default-path work; doing it
  here means doing it twice.
- **Any interpretation of the event.** Filtering, classification, debouncing,
  deduplication, rate policy per source, "is this event interesting" — all of it
  is the watcher's, per brief §7. A host that learns what an event means has
  crossed the first invariant.
- **Resolving SPEC-002 OQ-4.** An ingested evaluation may replace a view a
  person is mid-answering, exactly as a scheduled firing may. This slice raises
  how often that is reached and answers nothing about it. See OQ-8.
- **Persistence.** An event that arrives while the host is not running is lost,
  and there is nowhere to record it. SPEC-002 §2 already puts persistence out of
  scope and SPEC-001 OQ-3 carries it.
- **Peer-credential checks and multi-user hardening.** Access is the socket's
  mode; the containing directory is the user's responsibility, stated rather than
  defended.
- **A queue.** The host holds no pending event. An envelope it cannot act on is
  refused, and the watcher decides whether to retry.

## Acceptance criteria

- [ ] AC-1 — With the ingress key configured, a shell one-liner writing a
  well-formed envelope to the socket produces exactly one `evaluate` at the
  backend, carrying the envelope's `source`, `kind`, `timestamp` and `data`
  **verbatim**, and the host's own instant as the request's `now`.
- [ ] AC-2 — A view the backend returns in answer to that evaluation reaches the
  screen and is answerable, indistinguishably from one a scheduled firing
  produced. The ingested path is the evaluation path, not a parallel one.
- [ ] AC-3 — Every envelope receives exactly one reply on the same connection,
  and the host then closes it. Nothing is accepted in silence and nothing is
  refused in silence.
- [ ] AC-4 — A refusal names its reason, and the reasons are a closed set the
  design enumerates. At least: not one JSON document; a missing or wrong-typed
  envelope field; a `source` of `"host"` (CD-2); the host is engaged; inside the
  event spacing (CD-1).
- [ ] AC-5 — A writer emitting envelopes as fast as it can does not make the host
  begin event-triggered evaluations faster than one per spacing. The excess are
  **refused**, not delayed, and the refusal says so.
- [ ] AC-6 — The two anchors are independent, falsifiably in both directions: an
  event-triggered evaluation neither delays nor advances a scheduled firing, and
  a scheduled firing does not grant an event-triggered evaluation that would
  otherwise be too soon. This is the case ADR-004 says no existing test can
  distinguish.
- [ ] AC-7 — With the key absent, the host binds nothing and behaves exactly as
  it does today. The existing suite passes with unchanged bodies.
- [ ] AC-8 — A socket file left behind by a killed host is reclaimed: the host
  unlinks it and binds. A path a **live** host holds is a startup error naming
  it, and the running host keeps its socket.
- [ ] AC-9 — A path that is a regular file, or that cannot be created, is a
  startup error naming what was found. The exit code is non-zero and the message
  says which side was wrong.
- [ ] AC-10 — The bound socket's mode is owner-only, whatever umask the host was
  started under.
- [ ] AC-11 — The boundary holds: no domain vocabulary in the new module names or
  types; the vocabulary scan, the manifest allowlist, the stratum 1 purity scan
  and `cargo test -p goad-semantics` all pass, and `goad-semantics` gains no
  dependency.
- [ ] AC-12 — A malformed envelope never reaches the backend, and no ingress
  failure after startup takes the host down or leaves it unable to invoke the
  backend again.
- [ ] AC-13 — A person has run the software with ingress configured, emitted an
  event from a shell one-liner, and watched the prompt appear. Recorded in
  `audit.md` under Evidence, naming what was observed (`docs/AGENTS.md` §Tiers).

## Governing canon

**Binding, and amended by this slice:**

- **SPEC-001** (the host/backend interaction protocol) — R-7 (an `evaluate`
  carries the host's instant *and* an event with four fields), R-9 (the host
  interprets neither an event's data nor a submitted value), R-56 (the three
  host kinds; the set open). **CD-2 amends R-56** to reserve `"host"` as a
  source.
- **SPEC-002** (the host's scheduling behaviour) — R-5 is the requirement this
  slice exists to discharge: *"a host that adds a stimulus other than a due
  check MUST decide separately how that stimulus is bounded, and MUST NOT read
  this requirement as covering it."* Also R-9 (one exchange in flight), R-10
  (cancellation precedence), R-4 and R-6 (the scheduled floor, which this slice
  must leave exactly as it is), OQ-4. **CD-1 amends it** with the event bound.

**Binding, unamended:**

- **ADR-001** (one-way strata) — names event ingress in stratum 2, so the
  listener's home is decided.
- **ADR-003** (the workspace of strata) — stratum 2 is `crates/goad-shell`.
- **ADR-004** (the scheduled anchor) — the record this slice was written to
  argue with. Its premise stands: no stimulus clears the scheduled anchor, and
  the event bound is a separate rule on a separate anchor.
- **POL-001** (the phase gate) — six commands, four ADR-001 instruments plus the
  vocabulary scan plus one residue.

**Checked and not applicable:**

- **ADR-002** (single crate until triggered) — superseded by ADR-003. Its T2
  trigger, a second binary, is slice 005's and not this slice's: the client here
  is a shell one-liner.

## Open questions

- OQ-1 — The event spacing's **value**, and whether it is a constant of the host
  as R-4's three seconds is, or configurable. R-4's is not configurable on the
  argument that a bound a misconfiguration can remove is not a bound; whether
  that argument transfers to a bound on the user's own watcher is not obvious.
- OQ-2 — The reply's wire format, and whether it declares a version. It is a
  second contract, it is not SPEC-001's, and slice 005 is its first real client.
- OQ-3 — One envelope per connection, or a stream of them on one connection.
- OQ-4 — Where the envelope's normalization lives: stratum 1, beside the
  protocol's own permissive-in/canonical-out normalization, or stratum 2 with
  the listener. The first is the established pattern; the second keeps stratum 1
  free of a format no backend ever sees.
- OQ-5 — Whether CD-2's rule is stated in SPEC-001 — a rule about what a host
  *emits*, which is that spec's subject — or in this slice's own ingress rules,
  a rule about what a host *accepts*, which is not.
- OQ-6 — The probe/bind race. Two hosts starting at the same instant can both
  find the path stale and both bind. What, if anything, this slice does about it.
- OQ-7 — Whether an ingested evaluation is distinguishable in the diagnostics
  surface, and what a person needs to see there to debug their own watcher.
- OQ-8 — SPEC-002 OQ-4, **carried and not answered**: an ingested evaluation can
  supersede a view a person is mid-answering. Recorded so the decision to leave
  it is visible rather than absent.
- OQ-9 — Whether `examples/demo.toml` configures ingress, and what the
  documented one-liner is. Bears on AC-13.
- OQ-10 — Whether an envelope's `timestamp` is the host's business beyond its
  shape. It is a modelled field, unlike `data`, so a far-future or far-past value
  is a case the design must either refuse or carry deliberately.

## Summary

<!-- Written at close: what actually landed, in three or four lines. -->

## Follow-ups

<!-- Deferred work surfaced by this slice. Each becomes a future slice or a
     line in a spec. -->

- SPEC-002 OQ-4, now reachable from two stimuli rather than one (OQ-8).

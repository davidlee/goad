# Slice 004: Event ingress

**Stage:** execute
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
key before any config may write it. An empty path is refused at load rather than
represented past the boundary (`design.md` §5.2).

**Stratum 3 — `crates/goad/src/`.** `main.rs` and `startup.rs` bind the socket
before the event loop starts, and carry the new startup failures.
`controller.rs`'s `serve` gains an ingress arm on **both** of its `select!`s —
the outer one to judge an arrival, the inner one to refuse promptly while an
exchange is in flight — and the event spacing and its anchor. `diagnostics.rs`
gains `Refused::Ingress`, which is how a refusal the loop decided **while idle**
reaches a person who is not the writer.

`wire.rs` is **not** touched. An ingested evaluation is not a `Stimulus` at all:
that type names why the *host* is asking, and an ingested evaluation is not
host-originated. The vocabulary both paths already share is `Pending::Evaluate`,
and the **outer** ingress arm builds one directly — the inner arm only ever
refuses (`design.md` D-13, §5.4).

**Stratum 1 — `crates/goad-semantics/`.** `Event` already exists there, and the
envelope's normalization **stays in stratum 2**. ADR-001 names both sides of
that question — wire-to-canonical normalization is stratum 1's, event ingress is
stratum 2's — and the envelope is both, so the placement is the deliberate call
ADR-001 §Consequences says someone must make each time. It is made on **whose
contract the normalization serves**: stratum 1's holds SPEC-001, the host/backend
protocol, in one place; the envelope is a different contract with different
parties that no backend ever sees, and it belongs beside the listener that reads
it. Normalizing into `Event` from stratum 2 opens no second door — `Event` is a
transparent record whose only fallible field is a `jiff::Timestamp`, and stratum
3 has constructed one since slice 002 (`design.md` §2 F6, §5.1, D-3). The
decision gets its own ADR at reconciliation.

The one code touch is widening `json_type_name` from `pub(crate)` to `pub`,
which is a visibility change and not a dependency (`design.md` D-18). What is
fixed and holds: **stratum 1 gains no dependency**, and the four ADR-001
instruments and the vocabulary scan pass unchanged.

**Canon.** `canon-delta.md` CD-1 (SPEC-002, the event bound), CD-2
(SPEC-001/R-56, the reserved source) and **CD-3** (ADR-004, whose Verification
section predicted this slice and is amended to name the tests that discharge
it). Plus one **new ADR**, written at reconciliation: the envelope normalizes in
stratum 2 (`design.md` §10). Nothing is edited before reconciliation.

**New canon.** `draft-spec.md` — the ingress contract: the socket, the envelope,
the reply, the refusal taxonomy and the connection bounds. It takes its number
at promotion — **SPEC-003**, as the design and CD-1 name it — and is this
slice's working authority until then (`design.md` D-1). It numbers its own
ingress-side requirement **R-12**, as SPEC-002's new requirement is also
numbered R-12 and neither may be renumbered: every mention of either that
crosses a document boundary is written `SPEC-002/R-12` or `SPEC-003/R-12`.

**Tests and examples.** Listener behaviour in `goad-shell`; loop and end-to-end
behaviour in `crates/goad/tests/renderer/`; a documented shell one-liner that
emits an envelope, which is this slice's emitter. `examples/demo.toml` listens
at a path in the checkout, and `flake.nix` gains `socat` so the one-liner works
from a clean clone in the dev shell — this slice's only environment change
(`design.md` D-17).

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

### Readings taken in design

The criteria above are unchanged and their ids are immutable. Four needed a
reading before they could be built against; AC-1, AC-6 and AC-7 are recorded in
`design-log.md` (2026-09-08) and `design.md` D-19, AC-3's is `draft-spec.md`
R-8, and AC-9's was taken at plan acceptance.

- **AC-1's "verbatim"** holds as *the same instant*, not the same bytes, for
  `timestamp` alone. `Event.timestamp` is a modelled `Timestamp`, so the host
  re-serialises it and an envelope written `+10:00` reaches the backend spelled
  `Z`. `source`, `kind` and `data` are byte-for-byte.
- **AC-3's "every envelope"** admits one exception, and one only: a connection
  may close unanswered when the host process itself is gone (`draft-spec.md`
  R-8) — an envelope that arrives between `bind` and `serve` starting, on a host
  that then fails to start, gets EOF and no reply. AC-3 also quantifies over
  *envelopes*, so the one refusal that answers no envelope — the ingress-stopped
  `unavailable`, see Follow-ups — is outside it rather than a breach of it.
- **AC-6** is a claim about the two **anchors**: an ingested firing never writes
  the scheduled floor and a scheduled firing never clears the event floor. The
  pending *deadline* still moves after an ingested exchange, because the backend
  answered it with a `next_check` — SPEC-001/R-26, exactly as after a person's
  evaluation. That is why the *does not advance* case of AC-6 must fix the
  ingested exchange's own deadline as part of its setup, not only the scheduled
  one's: otherwise what the test turns on is a deadline both hypotheses agree
  about rather than the floor they disagree about (`design.md` §9). AC-6 is
  discharged by **three** tests, one per way the anchors could cross, and it is
  the *does not advance* one that is the case ADR-004 says no existing test can
  distinguish — the other two falsify a different alternative and hold CD-1's
  new event anchor, on which ADR-004 makes no claim.
- **AC-9's "the exit code is non-zero"** is held by **review**, not by an
  instrument, in the shape `draft-spec.md` R-5's verification row already uses.
  No test target links the binary, and `crates/goad/tests/renderer/startup.rs`
  states as that file's own rule that no test there runs it or asserts an exit
  code. What discharges the clause is the argument at `main.rs:21-29`: a single
  `match run()` mapping every `Err` to `ExitCode::from(2)`, unchanged by this
  slice. The rest of AC-9 — the error value and the message naming what was
  found — is held by tests.
- **AC-7's "unchanged bodies"** means unchanged assertions. `serve` gains one
  parameter, so 23 call sites pass `Ingress::none()`; no assertion moves.

## Governing canon

**Binding, and amended by this slice:**

- **SPEC-001** (the host/backend interaction protocol) — R-7 (an `evaluate`
  carries the host's instant *and* an event with four fields), R-9 (the host
  interprets neither an event's data nor a submitted value), R-22 (the offset
  rule the envelope's `timestamp` mirrors), R-45..R-47 (an untrusted input may
  not take the host down), R-56 (the three host kinds; the set open). **CD-2
  amends R-56** on both sides of one sentence: its first clause narrows to every
  `evaluate` the host originates **on its own account**, without which a
  conforming host breaches R-56 the moment it forwards an ingested event, and a
  clause is added reserving `"host"` as a source. The matching *refusal* is
  SPEC-003's, not SPEC-001's — a rule about what a host accepts.
- **SPEC-002** (the host's scheduling behaviour) — R-5 is the requirement this
  slice exists to discharge: *"a host that adds a stimulus other than a due
  check MUST decide separately how that stimulus is bounded, and MUST NOT read
  this requirement as covering it."* Also R-9 (one exchange in flight), R-10
  (cancellation precedence), R-4 and R-6 (the scheduled floor, which this slice
  must leave exactly as it is), OQ-4. **CD-1 amends it** with the event bound as
  **SPEC-002/R-12** and with the principle it instances, and amends two
  statements the new requirement would otherwise falsify: §2's count of what
  this spec abuts, and §6's sentence naming what the host owns.

**Binding, unamended:**

- **ADR-001** (one-way strata) — names event ingress in stratum 2, so the
  listener's home is decided. It names wire-to-canonical normalization in
  stratum 1, so the *envelope's* home is not: §Consequences says such questions
  arise and must be decided deliberately, and this slice decides it and records
  the decision as a new ADR.
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

**All ten are dispositioned in `design.md` §6.** OQ-3 was settled at scoping by
AC-3; OQ-8 is carried unanswered by decision; the other eight were answered
during design. They are left here as written, because their ids are cited from
the design and the log.

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
- **Pre-existing flaky tests, unrelated to ingress.** Chasing a one-off
  reclaim-case failure did not reproduce it (715/715 passes under load up to
  7.2x cores), but the same load window reproduced
  `failure_matrix::a_backend_that_never_answers_reaches_the_caller_as_a_timeout`
  five times and
  `transport::a_stdout_flood_is_refused_and_the_backend_sees_the_stream_close`
  eight, plus two more once each. All wait on a real subprocess and a real
  timeout. They predate this slice and are outside its surfaces, but they are a
  standing source of unexplained `just check` failures — and the likelier cause
  of the failure that prompted the chase. The reproduction numbers are in
  `notes.md` under PHASE-03's Findings so a future slice does not start from
  zero.

  **The reclaim case is reproduced — 2026-09-09, during `review-code.md`'s
  repair pass, and it is a fifth flake rather than one of the four.**
  `ingress::a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves`
  fails `bind` with `in use by a live host`: `reclaim`'s `connect` probe
  succeeds against a path whose listener the case has just dropped, so the stale
  socket is read as live. **It is not the repairs' doing** — 1 failure in 35
  sequential runs of the integration target on the repaired tree, and **2 in 60
  on a worktree at `93abab3`**, which predates every repair; same case, same
  message, comparable rate. It reproduces under **repeated sequential runs of
  the one target, unloaded** — which is why PHASE-03's chase missed it: that
  chase ran the suite under parallel load up to 7.2x cores, and this wants the
  opposite. A future slice should start from that condition, not from load.
- **Single-instance enforcement.** Nothing prevents two goad processes today —
  no lock, no pidfile, no check — and the probe/bind race (OQ-6) is one symptom
  of that rather than a fact about the socket. Closing it inside the ingress
  module would be a partial single-instance guarantee under another name
  (`design.md` D-10, `research.md` F15). The same gap has a second face after
  startup: nothing re-probes the path once bound, so a socket unlinked or
  replaced underneath a live listener leaves it holding a descriptor no
  `connect` can reach, and the host has nothing to report because nothing
  arrives (`design.md` §5.5, `draft-spec.md` §6.1).
- **An accepted ingested evaluation is not distinguishable on the diagnostics
  surface** (OQ-7, `draft-spec.md` OQ-3). The writer has its own answer; the
  person who is not the writer does not.
- **What the host's surfaces and vocabularies owe now that a process outside
  the host can reach them.** One question, three instances, all raised by
  `review-code.md` round 1 and all deferred on the same ground: each is a
  design decision about a surface this slice did not open, not a repair this
  slice withheld for size.

  - **The diagnostics surface holds one thing, and ingress is an unbounded
    author of it from outside the process** (F-9). `Diagnostics::refused`
    replaces the whole retained value, so every ingress refusal wipes whatever
    was there — measured at ~1690/s, which puts a backend failure a person
    needs to see out of reach within a millisecond. **The non-adversarial
    instance an operator meets first: a second `goad` start on a machine
    already running one.** Its reclaim probe is a bare `connect`, which the
    live host reads as an empty envelope and refuses `malformed`, so a failed
    start wipes the running host's surface and leaves *"an event was refused
    (malformed)"* on it — a message about the operator's own second process,
    phrased as if a watcher sent bad bytes. Deciding what the slot holds —
    retention, and precedence between host-authored faults and
    externally-triggered refusals — reaches the three refusal paths that
    predate this slice (`SupersededView`, `UnknownOption`, `NoClock`), so it
    cannot be settled inside the ingress arm.
  - **Whether ingress should serve connections concurrently** (F-10). The
    accept loop is sequential, so the per-read bound is also the longest one
    connection can deny every other. §6.4 now *states* that property, which is
    what F-10 required; whether the host should stop having it is the part left
    open.
  - **Whether the wire's reason set should be a type rather than a convention**
    (F-5). `Refusal::reason()` returns `&'static str`, so nothing structurally
    prevents a ninth token; the closure is held by an exhaustive `match` in the
    test file plus review, and R-14's Verification row says so in terms.
    `reason()` returning a closed `Reason` type would make a new token a new
    variant of a type whose only purpose is the wire vocabulary: a ninth
    `Refusal` variant could then not mint a token at all, because its forced arm
    would have to name an existing `Reason`. **A fresh unit of work with its own
    declared surfaces, and they are few** — `Refusal::reason()`'s signature
    (`crates/goad-shell/src/ingress/mod.rs`); its one remaining caller outside
    that module, `folded()` in `crates/goad/src/controller.rs`, F-4 having
    collapsed two call sites into it; and, in
    `crates/goad-shell/tests/integration/ingress.rs`, `Seen::Refused`'s payload
    type and the closure case itself. The renderer tier reads reasons off the
    reply JSON rather than off `reason()`, so it is untouched — every
    `reason(...)` in `crates/goad/tests/renderer/ingress.rs` is that file's own
    reply parser, which is why it looks like a caller list and is not one. The
    full caller census, so this need not be re-run: `ingress/mod.rs:339` (the
    definition) and **`:725`, the in-module `#[cfg(test)]` case
    `a_connection_that_faults_mid_read_is_unavailable_and_carries_the_error`,
    which asserts on the token and needs the same update** —
    `controller.rs:431` (`folded()`); `tests/integration/ingress.rs:86` and
    `:937`. It is a public
    API change and therefore a decision rather than a repair, which is why F-5
    left it here.

  They share a cause: slice 004 gave the host an input reached from outside the
  process, and the surfaces and vocabularies it feeds were all designed when
  every author of them was the host itself.
- **Refusals a person cannot see.** The diagnostics surface is one whole value,
  presented between exchanges, so only the refusals the host decides while idle
  survive to be presented (`draft-spec.md` R-15). Four never reach a person:
  `engaged`, which is by definition decided during an exchange and is
  overwritten by that exchange's own outcome; a shape refusal that happened to
  arrive during one; the `unavailable` written after the loop ends; and — in one
  interleaving where the loop notices it mid-exchange — the `unavailable` that
  says ingress has stopped. **`engaged` is the commonest refusal a real watcher
  will meet**, and it is invisible to the person debugging that watcher. Making
  it visible needs something the surface is not today — a count, or a log — and
  choosing between those is the follow-up, not a detail of this slice. AC-3 is
  unaffected: the reply is the guarantee, and **every envelope's** refusal
  reaches its writer. The ingress-stopped `unavailable` is the one refusal with
  no writer to fall back on — it answers no envelope, which is why the surface
  is its only report and why AC-3 does not quantify over it either.

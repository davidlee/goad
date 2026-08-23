# Design log — Slice 001

Append-only working record for the design stage. Survives compaction and
interruption; the design document itself stays clean. Never rewrite an entry —
supersede it with a later one.

## Decisions

### 2026-08-23 — What is the scope of slice 001?

- **Asked:** Three candidate first slices. (a) Protocol core plus process
  transport, headless — brief phases 1 and 2, with the repo foundations it
  needs folded in; Slint shell becomes slice 002. (b) Walking skeleton — a thin
  vertical cut including the GUI: poll, evaluate, render choice, respond,
  `next_check`. (c) Foundations only as a separate small slice — root
  `AGENTS.md`, a protocol spec as canon, cargo skeleton with a committed Slint
  hello world, verification commands — then protocol core after.
- **Recommended:** (a). A throwaway Slint spike had already shown 1.17.1
  building and opening a Wayland window in the dev shell, so GUI risk was
  retired without spending a slice on it. The live risk is the protocol
  contract, which brief §10.2 and §22.3 require to admit capabilities the v0
  renderer will not implement. Designing it headless under fixtures avoids
  letting a first renderer narrow it.
- **Decided:** (a) — protocol core plus process transport, headless.
- **Consequence:** Slice sequence becomes 001 protocol core and process
  transport, 002 minimal Slint shell (the spike's findings land there), 003
  scheduling, 004 external event ingress, 005 socket transport. Repo
  foundations — Cargo skeleton, root `AGENTS.md`, verification commands — are in
  scope for 001 rather than a slice of their own.

### 2026-08-23 — Does the protocol become `docs/specs/` canon in this slice?

- **Asked:** OQ-1. Either this slice writes a protocol spec into `docs/specs/`
  as canon, or the protocol lives only as Rust types plus fixtures until a later
  slice. Brief §15 treats a protocol document as authoritative, but nothing in
  `docs/specs/` exists yet to be authoritative.
- **Recommended:** none offered; the user answered before a recommendation was
  put.
- **Decided:** Yes — but written during **audit**, not during design or
  execution.
- **Consequence:** The spec is derived from what actually shipped rather than
  from intent, which is the reconcile step in `docs/AGENTS.md` ("document stale,
  code right → amend the spec / policy / ADR … get explicit user endorsement
  before writing"). No new acceptance criterion: the obligation belongs to
  `audit.md`'s reconciliation, and closure is already gated on it. Design and
  plan must therefore treat the Rust types plus the fixture corpus (AC-9) as the
  only contract during execution, and must not cite a spec that does not exist.

### 2026-08-23 — Async from the start, or `std::process` plus a thread?

- **Asked:** OQ-3. Brief §4.1 asks for the smallest reasonable async/event-loop
  integration. Adopting a runtime now is cheaper than retrofitting at the socket
  transport (slice 005); adopting one needlessly is a dependency this slice
  cannot justify.
- **Recommended:** none offered on the async question itself; the user answered
  first. On the follow-on runtime choice, tokio — see below.
- **Decided:** Async from the start.
- **Consequence:** The transport abstraction is async at its boundary from
  slice 001, so slice 005's socket implementation needs no signature change.
  Opens a follow-on question: which runtime.

### 2026-08-23 — Which async runtime? (follow-on to OQ-3)

- **Asked:** tokio versus the smol family (`async-process`, `async-net`,
  `async-io`, `futures-lite`). Brief §4.1's "smallest reasonable" reads as a
  nudge toward smol.
- **Recommended:** tokio. Measured against the actual feature set this slice
  needs, "smallest" points the other way: tokio with
  `["process", "net", "time", "rt", "macros", "io-util"]` resolves **14** unique
  dependencies; the smol combination resolves **31**. Tokio is one large crate
  with few dependencies; the smol ecosystem is many small ones. `tokio::process`
  also carries the mature child-reaping path that AC-6 depends on for non-zero
  exit and timeout.
- **Decided:** tokio. ("sure, tokio")
- **Consequence:** Slice 002 inherits the Slint integration problem, since
  Slint owns its own event loop; the pattern is the runtime on its own thread
  with `slint::invoke_from_event_loop` marshalling back. That is slice 002's
  work and is not free. Feature set for slice 001 is `process`, `time`, `rt`
  and `io-util`; `net` waits for slice 005 and `macros` only if the test
  harness wants it. Adding tokio is the event that arms ADR-002's T1 check for
  the next slice, but does not fire it: tokio is a stratum 2 dependency and
  stratum 1 must not acquire it.

### 2026-08-23 — One crate or a workspace? (OQ-2)

- **Asked:** OQ-2. A workspace makes ADR-001's one-way rule a compiler
  constraint rather than a review convention, keeps Slint's 411-dependency tree
  off the headless protocol tests, and lets `goad emit` be a small binary. A
  single crate avoids drawing boundaries around code that does not exist.
- **Recommended:** Single crate now, with the deferral written into canon
  rather than left as an intention — the failure mode being that slice 002
  quietly bolts Slint onto the one crate and the boundary is never drawn. The
  user proposed an ADR for this, which is the stronger instrument: canon binds
  slice 002 where a note in a slice document does not.
- **Decided:** Single crate, and both ADRs endorsed. ("sure, tokio. endorse the
  ADRs")
- **Consequence:** ADR-001 and ADR-002 move to `accepted` and become governing
  canon. Two effects on this slice: its Governing canon section is no longer
  empty, and AC-6 gains a constraint — the error taxonomy must split at the
  stratum 1 / stratum 2 seam rather than being discharged as one flat enum. The
  ADRs were split in two because the template requires one decision per record:
  the strata principle is durable, whereas the crate count is superseded when
  the split happens, and combining them would drag the former through a rewrite
  it does not need.

## Adversarial review

<!-- One block per review round. Findings are append-only and keep their ids
     across rounds. -->

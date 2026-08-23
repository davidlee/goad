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

### 2026-08-23 — How much configuration lands now? (OQ-4)

- **Asked:** OQ-4. Only the configuration this slice actually reads, or the
  whole illustrative file in brief §5 including socket path, poll interval and
  log path.
- **Recommended:** none put; the user answered first.
- **Decided:** just what this slice needs. ("just what this one needs")
- **Consequence:** `[backend]` command and timeout, plus a default poll
  interval — AC-4's schedule resolution takes the default as a parameter, so the
  value has to exist even though nothing wakes on it until slice 003. Excluded:
  the backend socket path (slice 005) and the log file path. Excluding the log
  path is only coherent because of the OQ-6 answer: diagnostics live in memory
  and surface on the host's own stderr, so there is no file to name. If that
  changes, this decision needs revisiting.

### 2026-08-23 — Which language for the example and fixture backends? (OQ-5)

- **Asked:** OQ-5. Brief §15.2 wants several languages eventually; this slice
  needs at least one, and a fixture backend has different requirements from a
  showcase example.
- **Recommended:** none put; the user answered first.
- **Decided:** TypeScript. ("let's say typescript, the esperanto of our times")
- **Consequence:** The AC-7 round trip runs against a real TypeScript backend,
  which is what makes the language-agnostic claim in brief §4.2 load-bearing
  rather than asserted. Raises OQ-9 (which TypeScript runtime, since the choice
  reaches into `flake.nix` and into whether `cargo test` needs a build step) and
  OQ-10 (whether the deliberately-misbehaving failure fixtures are also
  TypeScript).

### 2026-08-23 — Does host operational state persist to disk? (OQ-6)

- **Asked:** OQ-6. Outstanding `view_id` and resolved schedule either persist or
  stay in memory. Brief §20 phase 4 says "if required", which defers rather than
  answers.
- **Recommended:** none put; the user answered first.
- **Decided:** in memory. ("in memory for now i think")
- **Consequence:** No state store, no serialization format, no migration
  concern in this slice. AC-8's stale-`view_id` rejection is therefore scoped to
  a single process lifetime: a restart forgets the outstanding interaction
  rather than rejecting a response to it. That is a real semantic limit and
  belongs in the design's assumptions, not left implicit. Also settles the log
  path question under OQ-4.

### 2026-08-23 — What happens to the Slint spike? (OQ-7)

- **Asked:** OQ-7. Four options: harvest the mechanics into `docs/memory/` and
  let the code die with the session-local scratchpad; the same plus parking the
  code on a `spike/slint` branch; discard and rely on `research.md`; or commit
  it into `src/` as a skeleton.
- **Recommended:** the first — the note is durable, and forty lines are
  reconstructible from it.
- **Decided:** commit the spike first so the findings have a referent, then
  harvest and delete. ("is it in the commit history at all? if not I'd commit it
  and then 1.") Confirmed absent from history beforehand.
- **Consequence:** `99404f8` adds the spike; `c8ab319` removes it and adds
  `docs/memory/slint-build-mechanics.md`, which cites the sha. This is strictly
  better than the recommendation: the note is checkable against real code
  instead of being taken on trust, with no orphan branch to remember and reap.
  Committed at `spikes/slint/`, never `src/`, so no renderer ever entered the
  host tree and ADR-002's T1 did not fire.


### 2026-08-23 — Which TypeScript runtime? (OQ-9)

- **Asked:** OQ-9. deno, node, or bun. The choice reaches into `flake.nix` and
  determines whether a test that shells out to a backend needs a compile step or
  an installed `node_modules` first.
- **Recommended:** deno, invoked with `-A`. Node was ruled out on hermeticity:
  it needs `tsx` or a compile step plus an install before a backend runs, which
  is hostile to `cargo test`. Between deno and bun the decisive point is that
  **deno typechecks and bun strips types**. Brief §3.7 makes coding agents the
  authors of these backends, so if TypeScript was chosen for authoring
  ergonomics then the types should be enforced, or the annotations are
  decoration.
- **Decided:** deno. ("sold")
- **Consequence:** `flake.nix` gains deno in `devToolPkgs`, so `flake.nix`
  joins this slice's declared surfaces — it was not listed, and finding that out
  during execution would have been scope creep rather than a decision. Backends
  are invoked as `["deno", "run", "-A", "./backend.ts"]`, which the
  command-array form of the config already supports. The example must state that
  deno's default-deny permission model is *not* a security boundary here: brief
  §14 has backends launching applications and reading arbitrary files as trusted
  user programs, so `-A` is correct and nothing may imply isolation.


### 2026-08-23 — Are the misbehaving fixtures TypeScript too? (OQ-10)

- **Asked:** OQ-10. Whether AC-6's deliberately-failing fixtures — timeout,
  non-zero exit, malformed stdout, protocol-invalid response, invalid scheduling
  value, unsupported required primitive — are TypeScript like the example
  backend, or minimal helpers in something else.
- **Recommended:** TypeScript for all of them, since deno is already required by
  AC-7 so they cost no new dependency and the suite keeps one mental model. With
  two exceptions: command-not-found needs no fixture at all, only a path that
  does not exist; and **one** backend in the suite should not be TypeScript,
  because if every backend runs under deno the suite cannot distinguish "the
  transport works with any command" from "the transport works with deno".
- **Decided:** approved. Non-TypeScript guard in bash. ("whatever's easiest.
  Bash or python is my guess")
- **Consequence:** bash rather than python, because `bashInteractive` is already
  in `devToolPkgs` (`flake.nix`) and python is not — python would mean a second
  dependency added to the dev shell for a three-line fixture. The bash backend
  ignores its request entirely and emits a canned response: shell-side JSON
  matching is fragile, and the guard's only job is to show that the transport
  spawns and pipes for an arbitrary command. Invoked as
  `["bash", "./backend.sh"]` through the config command array, so no shebang is
  involved.


## Adversarial review

<!-- One block per review round. Findings are append-only and keep their ids
     across rounds. -->

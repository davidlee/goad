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


### 2026-08-23 — A draft spec during the slice, promoted at close (revises OQ-1)

- **Asked:** not asked — raised by the user against design §1, which stated that
  this slice has no spec to cite because the OQ-1 answer put the protocol
  specification in `docs/specs/` at audit.
- **Recommended:** nothing; the standing position was the OQ-1 answer, and this
  supersedes it.
- **Decided:** a draft spec is written in the slice folder over the course of
  design, execution and audit, and promoted to a real spec at the end. ("I
  reckon we write a draft spec in the slice folder during the course of design /
  execution / audit, and promote that to a 'real' spec at the end")
- **Consequence:** strictly better than the OQ-1 answer it replaces. That answer
  left execution with no prose contract at all — protocol semantics would be
  invented in code, and every ambiguity settled silently by whoever was typing,
  which is the failure brief §3.3 names. A draft closes that gap without making
  the spec canon before it is true: a draft is not canon, so it can be edited
  freely with no immutability constraint and no endorsement gate on each change.
  Mechanics settled with it: the file is `docs/slices/001/draft-spec.md`, copied
  from `docs/templates/spec.md` per the methodology's copy-never-write rule, and
  carries `**Status:** draft` — a state the template already provides, so
  promotion is a status flip plus a `git mv`, not a rewrite. It gets **no SPEC
  id** until promotion, because an id invites citation of `SPEC-001` before
  `SPEC-001` exists; until then it is cited by path. `R-N` requirement ids are
  assigned in the draft and survive promotion unchanged. Its first line states
  that it is not canon, because brief §3.7 makes agents the editors of this
  repository and a spec-shaped file under `docs/` will otherwise be read as
  normative. Adds AC-13 and AC-14. The residual risk the OQ-1 answer was
  avoiding — a spec written early becoming intent that drifts from what shipped
  — is now carried by audit: reconciliation compares the draft against the code
  before promotion, and a divergence is dispositioned per `docs/AGENTS.md`
  rather than promoted as-is.


### 2026-08-23 — Failure granularity: whole-message or per-part? (P2)

- **Asked:** by the user, against design §4's P2, which said an invalid value
  costs the sender its effect but was silent on granularity: is an invalid
  `next_check` inside an otherwise valid response accepted with the spurious
  value disregarded, or does it reject the message?
- **Recommended:** partial for `next_check` specifically, and — more usefully —
  a general rule rather than a case list. Brief §13 lists "invalid scheduling
  value" as a failure mode separate from "protocol-invalid response", which
  would be redundant if a bad `next_check` invalidated the message; brief §9
  says to report the error and preserve a sensible existing/default schedule.
  The rule proposed: **a part may be discarded on its own only when its absence
  is already a modelled state with defined semantics, distinct from "we failed
  to read it."**
- **Decided:** yes. ("yep")
- **Consequence:** the rule is what settles the case, not the citation. Absent
  `next_check` has a defined meaning (§9: retain existing valid, else default
  poll interval), so discarding it lands the host in a state it already knows
  how to occupy. `view` fails the same test: the only absent-view state is
  `view: null`, which brief §11 makes a positive assertion that there is nothing
  to show — degrading an unreadable view to `null` would have the host assert
  that on the backend's behalf when the truth is that it could not tell, which
  is exactly the invented semantics brief §3.3 forbids. So: whole-message
  rejection for envelope, protocol version, `view`/`choice` structure and
  unsupported required primitives; per-part discard for `next_check` alone.
  P1 is not weakened, which was the live worry — the canonical response holds
  `schedule: Option<CanonicalInstant>` where `None` means "no instruction
  supplied", and an invalid value normalizes to `None` *plus* a reported error
  carried alongside the canonical value rather than inside it. Normalization
  stays a total function into a genuinely canonical type; what changes is that
  its result carries a discard list instead of being a bare `Result`, which
  §5.2 must specify and every caller must handle.


### 2026-08-23 — Module layout, empty stratum 3, and a boundary test (§5.1)

- **Asked:** how the three ADR-001 strata are laid out inside the single crate,
  whether this slice ships a binary, and whether the one-way rule gets any
  mechanical check during the one-crate period.
- **Recommended:** group by stratum at the top of `src/` (`semantics/`,
  `shell/`, stratum 3 empty), naming the directories with ADR-001's own words;
  ship a library with no binary; and add a test asserting no file under
  `src/semantics/` mentions `crate::shell`, `crate::bin` or `tokio`.
- **Decided:** approved, including the new acceptance criterion. ("nod, add the
  AC")
- **Consequence:** AC-15 added, and the test plus the layout added to the
  slice's declared surfaces. Grouping by stratum is chosen over grouping by
  topic for two reasons that both trace to ADR-002's admitted weakness: an
  upward `use` is wrong *on sight* in the import line without the reader having
  to know which topic sits in which stratum, and the eventual split becomes a
  literal `git mv` rather than a prior exercise in deciding which topic is
  which — the redesign ADR-002 says must not be necessary. Stratum 3 is declared
  but empty: no AC needs a binary, and P3 permits building only what the brief
  names, which is a renderer (slice 002) and `goad emit` (slice 004). A useful
  side effect is that integration tests can reach the crate only through its
  public API. The AC-15 test is explicitly **not** a promotion of ADR-001's
  verification from review gate to build gate — it checks three known tokens, so
  it catches the common case rather than the class, and ADR-001's Verification
  section stands as written. Recorded in design §10 so audit disposes of that
  deliberately instead of finding it.


### 2026-08-23 — Date/time dependency for stratum 1: jiff

- **Asked:** `next_check` must normalize to a canonical instant (brief §9.1) and
  RFC 3339 parsing therefore happens in the pure core, so stratum 1 needs a
  wall-clock date/time crate. Which one, and with which features.
- **Recommended:** jiff 0.2.35 with `default-features = false, features =
  ["std", "serde"]`. Measured unique dependency counts, same method as the tokio
  sizing: jiff minimal **3** (`jiff`, `jiff-core`, `serde_core`), jiff default
  2, chrono 0.4.45 default 4, time 0.3.55 minimal 7 (including a proc macro).
- **Decided:** approved. ("ok")
- **Consequence:** three grounds, only the first of which is dependency count.
  (1) jiff parses brief §9.1's `"45 minutes"` example natively, along with
  `1h 30m`, `45m`, `2 hours` and ISO `PT45M` — verified by running it, not read
  from documentation — so AC-3's relative-duration support costs no hand-rolled
  parser. (2) Its failures are the right failures: `"tomorrow morning"`,
  `"45 minutez"` and `""` are rejected, and an offset-less
  `2026-08-22T18:00:00` is *refused* rather than assumed to be local time. That
  is brief §3.3's prohibition on invented semantics enforced by the library
  rather than by our vigilance. (3) Disabling default features is principled,
  not merely smaller — the defaults pull the system timezone database, which
  reads `/etc/localtime`, i.e. I/O inside stratum 1. Turning it off removes the
  capability, so ADR-001's no-I/O rule holds by construction. It costs one
  dependency (`serde_core`) relative to the default build, which is the right
  trade.
  Canonical instant is `jiff::Timestamp`; relative forms parse to `jiff::Span`
  and resolve as `now.checked_add(span)`, with `now` taken from the request
  envelope, so stratum 1 never reads a clock. The same duration grammar is
  reused for the TOML config values, which brief §5 already writes as strings
  (`timeout = "5s"`, `default_poll = "30m"`) — one duration grammar across the
  whole product, and no seconds-versus-milliseconds ambiguity.
  Edge case surfaced and carried to design §5.5: `"-45 minutes"` parses to an
  instant in the past. That is coherent under brief §9, which says only "do not
  evaluate *before* this point", so a past point is no constraint. Normalization
  accepts it and schedule resolution clamps the resolved instant to no earlier
  than `now`; a minimum wake interval to stop a backend busy-looping on it is
  slice 003's timer problem, not this slice's.


### 2026-08-23 — Protocol version is asymmetric between the directions (§5.2)

- **Asked:** brief §13 says the envelope is versioned from day one, but §8.2's
  response examples carry no `protocol` field. Must a response declare it?
- **Recommended:** asymmetry. The host always writes `"protocol": 1` on
  requests; a response may omit the field, but a response declaring a version
  the host does not know is rejected.
- **Decided:** accepted. ("reasonable. Accepted")
- **Consequence:** the asymmetry follows from authorship, not from convenience —
  the host controls what it emits and can always be strict with itself, whereas
  requiring the field inbound would reject every backend written against the
  brief's own examples. Ignoring a declared-but-unknown version was the other
  candidate and is worse: it is guessing at semantics, which brief §3.3 forbids.
  So "versioned from day one" is satisfied by the envelope carrying the field
  and by unknown declared versions failing, not by compelling both sides to send
  it. Recorded as a decision in design §7 because a later reader tidying the
  protocol for symmetry would otherwise reverse it by accident.


### 2026-08-23 — State ownership and `view_id` format (§5.3)

- **Asked:** what the host holds, who may write it, and what a `view_id` looks
  like — the one place the design chose legibility over the conventional answer.
- **Recommended:** a plain `State` struct behind `&mut self` with no lock; a
  non-optional `resolved_check` seeded from a `now` passed to `Host::new`;
  per-call diagnostics that are not retained; one outstanding interaction that
  is replaced rather than queued; and a `view_id` of `{now, RFC 3339}#{seq}`
  rather than a v4 UUID.
- **Decided:** accepted, `view_id` format included. ("I'm ok with it. accept")
- **Consequence:** the no-lock choice follows brief §12, which serializes backend
  exchanges and allows one outstanding interaction — a mutex would invent a state
  space §12 explicitly says to avoid. Making `resolved_check` non-optional
  removes an "unresolved" case that brief §9 never actually produces, and paying
  for it with a `now` parameter on `Host::new` keeps this slice entirely
  clock-free, leaving real time to slice 003's timer. Diagnostics are returned
  and forgotten because retaining a history that nothing can display would build
  the wrong half of brief §13's "discoverable diagnostic state"; retention lands
  with the renderer. The `view_id` format trades opacity for a value that is
  readable in a log (brief §13's debuggability), deterministic under a fixed
  `now` so fixtures can assert exact ids, and free of a `uuid`/`getrandom`
  dependency; nothing authenticates with it, and backends are trusted per brief
  §14, so opacity buys nothing here.


### 2026-08-23 — Accept the loss of stderr on timeout (§5.4)

- **Asked:** with `wait_with_output()` inside a `tokio::time::timeout`, a
  timed-out backend produces no stderr, because the output buffers drop with the
  future. Spend roughly ten lines now to drain stderr into a shared buffer via a
  spawned task, or accept the gap?
- **Recommended:** accept it for this slice. AC-5 asks for stderr capture and
  AC-6 for a distinct timeout error; neither requires both simultaneously, and
  there is no logging surface in this slice on which to display it.
- **Decided:** accept, and log the follow-up. ("accept it, log the follow-up.")
- **Consequence:** recorded under `slice-001.md` Follow-ups rather than only
  here, since `docs/AGENTS.md` is explicit that follow-ups must not be left
  where a later stage will lose them; the Follow-ups heading now carries a note
  that entries raised before close are marked with the raising stage. The design
  text states the gap in §5.4 and §5.5 rather than leaving it implicit in the
  code, with an explicit warning not to treat `kill_on_drop` as the thing to
  simplify away — removing it would leak the child process, which is a worse
  failure than the missing stderr.


### 2026-08-23 — The backend validates answers, and validation feedback stays additive (§5.5)

- **Asked:** whether `State` should retain the outstanding `View` so `respond`
  can reject an answer naming an option the view never offered.
- **Recommended:** retain the view and validate the option id — one field, buying
  the invariant that the host never sends a backend an answer to a question it
  did not ask — with a hard line at option id only, never field values.
- **Decided:** the backend validates. The user also named the eventual product
  requirement this implies: "host will need to (ultimately) retain entered
  values, highlight offending field(s), possibly (?) accept replacement values
  (eg stripped of invalid chars), and display any validation error messages."
- **Consequence:** `State` stays minimal and `respond` checks only the `view_id`;
  field values pass through opaque. The user's named requirement is a capability
  the brief does not cover at all — grep confirms nothing on validation feedback,
  line 1022's "configuration validation" being unrelated — so brief §22.3, *are
  we narrowing the protocol to match the current v0 renderer?*, was applied to it
  here where it is free. It is additive, and three decisions already taken are
  what keep it so. `UserResponse.values` is opaque, so the host can retain and
  echo entered values with no domain knowledge, which is what makes the user's
  "host retains entered values" possible without crossing brief §22.5's
  boundary. A returned view replaces the outstanding one and takes a fresh
  `view_id`, so a validation rejection is just another view and the host never
  needs a notion of "this is a retry of that" — the expensive version of the
  feature. And no inbound wire type uses `deny_unknown_fields`, so `field.value`,
  `field.error` and a form-level message can appear later with no protocol
  version bump, which is the test of a genuinely additive extension. That third
  property is now invariant I10 so it is not lost by someone tidying the wire
  types. One thing settled in advance because the shortcut is tempting and wrong:
  per-field validation errors are semantics, not presentation, so they must be
  typed fields rather than keys in the open `hints` map — a renderer may ignore
  `placeholder`, it must not be free to ignore a validation error. Recorded as a
  follow-up slice, after 002 since it needs a renderer.

### 2026-08-23 — Reverse the deferred pipe refactor (F-2, F-3; reverses D18, D19)

- **Raised by:** me, changing my own recommendation after the design review.
  Presented as a reversal rather than applied quietly, because the user had
  already approved the deferral: *"accept it, log the follow-up."*
- **Question:** the review's F-2 and F-3 objected to the two things that were
  accepted as a single deferred follow-up — unbounded stdout, and no stderr on
  the timeout path. Hold the approved decision, or reverse it?
- **Decided:** reverse it. The user's word was "reverse it".
- **Why my recommendation changed:** the deferral rested on two claims. The first
  — that both wants are one refactor, so doing them together is cheaper — was
  correct and still is; the design says so. The second — that no acceptance
  criterion demanded both at once, so the slice could ship without them — was
  optimising slice size against a stated requirement. Brief §13 says a backend
  failure must not take down the host, and an OOM against a looping backend is
  the host going down. That is not a rough edge with a scheduling question
  attached; it is the failure mode the brief names. F-2 is the same objection
  arriving from outside, which is the useful thing about an adversarial review:
  it does not have the sunk cost of the argument that produced the deferral.
- **Consequence:** `wait_with_output()` is gone. Stdout is capped at 8 MiB and
  exceeding it is fatal (`OutputTooLarge`); stderr is capped at 256 KiB and
  exceeding it truncates rather than fails, because a chatty backend that works
  is not broken and a truncated *stdout* would parse as malformed JSON and name
  the wrong fault. Caps are module constants, not config keys — brief §5 names no
  such keys and P3 forbids inventing configuration for an unrequested future.
  Stderr drains in its own task so its buffer outlives a timeout on the exchange
  future, and `BackendError::Timeout` now carries it. D18 and D19 are struck in
  §7, R3 in §8 closes as fixed rather than mitigated, I11 and I12 are new, and
  both follow-ups are withdrawn from `slice-001.md` with a note saying where they
  went.
- **A coupling worth recording:** this also killed D21. Its justification was
  that `wait_with_output()` consumes the child, so an explicit kill is
  unavailable and `kill_on_drop` is the only mechanism. Removing
  `wait_with_output()` removes that constraint, so F-14's separate complaint —
  that tokio's kill-on-drop is best-effort and needs a live runtime to reap —
  becomes fixable rather than merely acknowledged. Two findings that looked
  independent were coupled through one call. D26 supersedes D21: `start_kill()`
  then `wait()` on the path we know about, with `kill_on_drop(true)` retained as
  the backstop for panic and cancellation paths.

### 2026-08-23 — How hard to enforce P1, and what P1 governs (F-9)

- **Raised by:** the review. F-9 observed that the canonical types have public
  fields and no bounds validation, so `Choice { options: … }` and
  `Number { min: NaN, max: 1.0 }` are both constructible by anyone downstream —
  which makes P1 ("canonical is a type, not a promise") a promise.
- **Question:** enforce fully, or enforce and also scope what P1 covers?
- **Decided:** the user chose *"that scoping"* — enforce, and state the scope.
- **Enforcement.** Canonical fields become `pub(super)`, visible inside
  `semantics::protocol` where normalization lives and read-only elsewhere through
  accessors. The guarantee that buys is exact and worth stating in those terms:
  outside that module, a canonical value can only have come out of
  `normalize_response`. `NumberRange` replaces the bare `min`/`max` pair with a
  checked constructor rejecting non-finite bounds and inverted ranges. Bounds are
  semantics under brief §3.4 — they constrain which answers are valid — so an
  inverted range makes every answer invalid and `NaN` makes every comparison
  false. Neither is a state the protocol has a meaning for, so neither may be
  representable.
- **Scope, and why it was worth asking about.** Read literally, P1 covers every
  field, which would oblige the host to parse a `Content::Uri` it never
  dereferences and to give `hints` a closed type — the exact narrowing brief
  §22.3 warns against, arrived at by way of a principle meant to prevent it. So
  P1 is now stated as governing the values the host **interprets** — instants,
  bounds, identifiers, kind discriminants — and not the payloads it merely
  carries: `Content::Uri`, `hints`, `Event.data`, the `values` map of a response.
  The line is not arbitrary and it is not permanent: it tracks brief §3.4 and
  §14, and the moment the host starts interpreting one of those payloads it comes
  under P1.
- **Consequence:** D30 and D31; §4's P1 gains a *Scope* paragraph and an honest
  note that the scoping costs uniformity — a reader must now ask which side of
  the interpret/carry line a value sits on. I1 is restated in terms of the
  visibility boundary rather than "checked constructors" alone. R10 records the
  real risk, which is not the design but the erosion: `pub(super)` plus accessors
  is boilerplate, and boilerplate under deadline gets widened back to `pub`.

### 2026-08-23 — The bounded drain was built and run before being written down

- **Why:** F-2 and F-3's fix is the largest piece of new design written under
  review pressure, in the part of the system with the most ways to deadlock. A
  sketch that reads correctly is not evidence, and `review-design.md`'s own
  guardrail is to reject a finding on evidence rather than assertion — the same
  standard applies to accepting one.
- **What was built:** the whole §5.4 exchange as a throwaway binary in the
  scratchpad — spawn with three pipes, stderr drained in its own task, stdin
  written then dropped, stdout read through a capped reader, `wait()` inside one
  `tokio::time::timeout`, explicit `start_kill()` and `wait()` on elapse. Run
  against four backends: a well-behaved one, one that writes stderr then sleeps
  past the timeout, one that floods stdout, and one that reads stdin to EOF.
- **What it caught.** A borrow-checker defect in my own sketch:
  `match timeout(dur, exchange).await { … }` does not compile when `exchange`
  holds `&mut child` and an arm also needs `child`, because a temporary in a match
  scrutinee lives to the end of the match. The fix is to bind the result before
  the match. That is now in the design with the reason stated, because it is
  invisible until you try it and someone would otherwise rediscover it in
  execution.
- **What it confirmed.** Stderr does survive the timeout path — the killed
  backend's `boom` came back. A backend reading stdin to EOF completes, which is
  the stdin-close trap §5.4 already warned about. And one thing better than
  expected: hitting the stdout cap kills the flooding backend on its own, because
  the capped reader drops the stdout handle, the pipe closes and the process takes
  `SIGPIPE` — `wait()` returned a signal status immediately rather than blocking.
  So the cap bounds the work and not just the buffer. We still kill explicitly on
  that path, for a backend that ignores `SIGPIPE`.
- **Consequence:** §5.4 gains both facts. Nothing about the decisions changed;
  what changed is that they are now observed rather than argued.

### 2026-08-24 — The wire shape follows the brief's examples, flat keys and all (F-31, F-38)

- **Raised by:** the review's F-31, that the draft spec did not define the wire
  encoding of most admitted variants. Verifying it against brief §10 turned up a
  second and worse instance, which I raised as F-38.
- **What was wrong.** Brief §10.1's required v0 example writes
  `"body": "Optional context"` — a bare string — which a tagged-only `Content`
  rejects outright. Brief §10.2's field example writes
  `{"id":"notes","kind":"text","label":"Anything notable?","multiline":true}`,
  with `multiline` **flat on the field**; the design had `hints` as a nested
  member, so that key arrived unmodelled and the no-`deny_unknown_fields` rule
  discarded it *silently*. The brief's own worked examples were one rejection and
  one silent loss. Those examples are what a backend author copies, so a wire type
  that fails them is wrong however clean it reads.
- **Question put to the user:** flat keys only, or accept both a flat key and a
  nested `hints` object?
- **Decided:** *"fix both"* — flat only. Two accepted spellings for one thing is
  precisely the ambiguity brief §3.3 says must fail rather than be guessed at, and
  it would have doubled the normalization paths to buy compatibility with a
  spelling nothing uses.
- **Consequence, and the cost stated precisely.** `WireField` uses
  `#[serde(flatten)]`, so "every other key on the field object" is the definition
  of a hint — which is also the honest reading of §10.2's "likely presentation
  hints over time". Verified by running it: a misspelled **optional** key (`minn`)
  becomes a hint silently, while a misspelled **required** key still fails with
  `missing field 'label'`, because a declared field stays required after
  flattening. So the exposure is bounded by which keys are optional, which is
  narrower than flattening usually implies. Both cases are now edge-table rows.
- **One encoding trap avoided.** The obvious way to accept string-or-object is
  `#[serde(untagged)]`, and it is wrong here: it collapses every failure into
  "data did not match any variant", destroying the `UnsupportedPrimitive
  { kind, at }` error that F-6 was raised to obtain. So `body` stays
  `serde_json::Value` at the wire and `normalize` dispatches — the same shape, and
  the same reason, as `next_check`. D37, D38.

### 2026-08-24 — Keep an error variant JSON cannot reach (F-36)

- **Raised by:** the review. The spec asked for a fixture containing a `NaN`
  bound, and JSON has no NaN literal.
- **Verified before deciding**, because the disposition depends on where the
  failure lands: serde_json rejects `{"min": NaN}` with `expected value` and
  `{"min": 1e400}` with `number out of range` — both before any bounds check runs.
  So `BoundsError::NotFinite` is unreachable from the wire, and the only reachable
  bounds failure is `Inverted`.
- **Question put to the user:** drop the variant, or keep it as a constructor
  guard and fix the false claim?
- **Decided:** *"keep and correct"*.
- **Why that is the right way round.** `NumberRange::new` is public API, and P1's
  claim is about what the *type* can hold rather than about which caller supplied
  the value; one comparison now is cheaper than an argument in a later slice about
  whether the invariant really holds. The defect was never the guard, it was the
  claim: R-17's verification and §9's fixture now assert `Protocol(Json)` for a
  NaN literal, and §5.2 records that the variant is a constructor guard and not a
  wire failure mode.
- **The general lesson, worth more than the case:** a test asserting an
  unreachable error is a test that cannot fail, and it reads as coverage. That is
  the more dangerous of the two available mistakes — dropping a cheap guard is
  visible, a green test that proves nothing is not. D39.

### 2026-08-25 — Round 3: nine findings, all of them mine (F-39…F-47)

- **What the round was.** Nine findings, no blocker, and every one a defect in a
  round-2 *repair* rather than in the original design. That is the fact worth
  recording. The review is no longer finding design mistakes; it is finding the
  wake of the fixes, which means the marginal value of another round is now about
  my repair discipline rather than about the design.
- **Six of the nine were one defect.** F-43, F-44, F-46, F-47 and (in a different
  register) F-39 all have the same shape: a contract repaired at its primary site
  and left standing in a restatement — an invariant row, a decision-index line, a
  risk mitigation, an example, an acceptance criterion. F-34 had already named
  this class in round 2 and I had already written it down, which is the
  uncomfortable part: naming a failure mode does not fix it.
- **So it became a procedure rather than a resolution.** §9 now carries a
  restatement sweep as a review step: after any change to §5, re-read the
  invariant and edge tables, the decision index, the risks, the AC map and
  fixture list, the draft spec's requirements and examples, and the affected AC
  text in the slice card. The redundancy that causes this is deliberate — each
  contract is stated where a reader will meet it — so the answer is to pay its
  cost every time, not to remove it. No test can observe that two English
  sentences disagree.
- **F-39 is the same class turned inward.** D23 argues that a value every path
  produces must not live on the success branch of a `Result`; I applied it to
  `Outcome` and then built `Outcome` from `Result<Exchange, BackendError>`, which
  breaks it exactly. Restating a rule is not the same as applying it one level
  down. The transport now returns a bare `Exchange { result, stderr }`. D40.

### 2026-08-25 — Reap-failure precedence, and what R-47 is actually about (F-42)

- **Raised by:** the review. `start_kill` and `wait` are both fallible; the sketch
  called `reap` unconditionally and discarded its result, so a child that could
  not be killed would be reported as a clean exchange.
- **Question put to the user:** report both failures, or let the pre-existing one
  win?
- **Decided:** *"accept, narrow it to backend-supplied values"* — the proposed
  rule, with R-47 scoped accordingly.
- **The rule, in full.** *Already exited* is success: reaping unconditionally
  means most reaps run against a process that has already gone, and "idempotent"
  has to mean that or D35 is unimplementable. A reap failure with no prior error
  becomes `BackendError::Reap`. A reap failure alongside an existing error is
  dropped.
- **Why the last clause rather than reporting both.** "We also could not kill it"
  is a *consequence* of the timeout or overflow that made us abandon the child.
  Reporting both buries the cause under its effect, and the person reading the
  diagnostic needs the cause. This is an informational argument, not a tidiness
  one — if the two failures were independent the answer would be different.
- **The requirement had to move, and saying so is the point.** R-47's "every
  refusal MUST be reported" governs values the *backend supplied*, because the
  sender can act on those; that is the reason the requirement exists. It was never
  about the host's own cleanup telemetry, and read literally it contradicted the
  rule above. R-47 is now scoped, and **R-48 carries the reporting obligation for
  reap failures instead** — the obligation moved rather than evaporated, which is
  the difference between scoping a requirement and quietly weakening one. D42.

### 2026-08-25 — A requirement I could not meet, and did not pretend to (F-41)

- **Raised by:** the review, in two halves. The `?` on `PipeMissing` returned
  after the child existed and before the unconditional reap — a straight bug, now
  fixed with a `let … else` that reaps first, and a rule stated over the region:
  no `?` past the spawn.
- **The other half is not fixable.** If the exchange future is *dropped* —
  cancellation, or a panic unwinding past it — no code of ours runs. There is
  nothing to await a reap with. `kill_on_drop` is the only mechanism that exists
  on that path, and no design changes that.
- **Decided:** *"narrow it"*. R-48 now binds every path that **returns**, and
  states plainly that cancellation relies on `kill_on_drop`.
- **Why this is not the self-serving move it resembles.** Narrowing a requirement
  until the design meets it is exactly what an adversarial review is supposed to
  catch, so the distinction matters: the old wording was not merely unmet, it was
  *unmeetable* — it forbade the only mechanism available on the path it governed.
  A requirement no implementation can satisfy is not a high standard, it is a
  defect in the requirement, and leaving it standing would have made every future
  reader think the host does something it cannot. The half that *was* meetable
  got fixed rather than written down, which is the test of whether a narrowing was
  honest.

### 2026-08-25 — A known key in the wrong place is a contradiction, not a hint (F-45)

- **Raised by:** the review. `WireField` declares `min`, `max` and `options` for
  all five kinds, because one struct deserializes all five and `kind` is only read
  afterwards. Serde therefore consumes those keys *before* dispatch, so they can
  no longer fall through to `hints` — and a `min` on a text field vanished with no
  error and no hint.
- **Question put to the user:** reject with a distinct error, or let them become
  hints?
- **Decided:** *"reject"*.
- **Why, in D37's own terms.** The flatten decision rests on a division: unknown
  keys are presentation, known keys are contract. A contract key in a position
  where the contract gives it no meaning belongs to neither side of that division,
  so admitting it as a hint would not extend the rule — it would dissolve it. It
  is also the worse failure: `{"kind":"text","min":1}` would become a *successful*
  parse carrying a hint the renderer is forbidden to branch on, which looks like
  it worked. Silent absorption is what brief §3.3 and R-47 both forbid.
- **Consequence.** `ProtocolError::InapplicableKey { key, kind, at }`, carrying
  the path for the same reason `UnsupportedPrimitive` does. New requirement R-50,
  written so it does not collide with R-15: R-15 governs keys the spec does not
  name at all, R-50 governs named keys used where their kind gives them no
  meaning. Fixtures assert both directions — the misplaced key rejected, the
  unnamed key still becoming a hint — because a rule that only ever fires is
  indistinguishable from one that fires too often.
- **What this does to D37's cost statement, recorded because it was wrong.** D37
  claimed the exposure from flattening was narrow, being limited to misspelled
  *optional* keys. That was only true of a design in which misplaced *modelled*
  keys were caught, and they were not. The claim described the design I intended
  rather than the one I had written; it now describes the one I have. D43.

### 2026-08-25 — Round 4: the review reaches past its own wake (F-48…F-58)

- **What changed about the round.** Rounds 2 and 3 found defects in repairs.
  Round 4 found three blockers, two of them in the *original* design and
  untouched by 47 prior findings: `null` collapsing into omission (F-50) and
  ADR-001's dependency rule being false in a single crate (F-51). A fresh
  reviewer with no thread history, handed the ledger index and told to verify the
  round-3 repairs first, reached ground three rounds of an accumulating thread
  had not. That is worth remembering next time a review starts feeling
  exhausted: it was not the subject that was exhausted.
- **Three round-3 repairs reopened**, and two of them were reversed rather than
  extended. Repairing is where this design keeps failing, not designing.
- **Built before written, again.** The §5.4 structure was compiled and run
  against four backends before being described. Three of four facts it
  established changed the design. Four rounds in, the record is that executing a
  claim has been worth more than reasoning about it *every single time*, without
  exception, and the cost is minutes.

### 2026-08-25 — Cleanup is a second dimension, not a lower-ranked error (F-48, F-53)

- **The contradiction the design was carrying.** It wanted the configured timeout
  to bound the whole exchange *and* every returning path to have definitely
  reaped the child. Those cannot both be unconditional: once a backend is wedged,
  `wait` itself is something you have to bound. F-53 forced a third outcome into
  existence — *cleanup did not complete within the time the host allows*.
- **Question put to the user:** should an unreaped child outrank the exchange
  failure, or does D42's precedence stand?
- **Decided:** the user consulted the reviewer, and the reviewer's formulation was
  adopted over my proposal on all three points. Recording them as theirs:
  1. **Two channels, not a ranking.** `cleanup` sits beside `result`/`failure` on
     `Exchange` and `Outcome`. D42's error was forcing two independent facts into
     one precedence contest; with two dimensions the question stops existing
     rather than getting an answer. All four combinations are meaningful.
  2. **`CleanupTimeout`, not `Orphaned`.** My name asserted a process state the
     failure path does not establish. Running it proved the point: the case that
     actually fires is a backend that answered correctly and left a grandchild
     holding the pipes — the child exits and is reaped, and only the drain
     stalls. I had named the variant after the case I imagined rather than the
     case that occurs.
  3. **Keep the original failure too.** "Backend timed out, then cleanup also
     timed out" is more diagnostic than either alone, and two fields carry both
     without a recursive error type.
- **On the narrowing.** I13/R-48/AC-5 go from "has reaped" to "initiates
  termination and waits a bounded interval, and reports failure to observe
  cleanup". The test of an honest narrowing is whether an obligation vanished.
  Here *must reap, potentially forever* became *must attempt within a hard bound*
  **and** *must report inability to establish cleanup* — a stronger operational
  contract, because it also protects host liveness. Compare F-41's narrowing,
  which was honest for a different reason: there the old wording was unmeetable.
- **The deeper lesson, the reviewer's words:** backend outcome and host cleanup
  outcome should not share an error channel. Most of the precedence reasoning
  disappears once they are separated. D47, D48.

### 2026-08-25 — Three repairs to one mistake, and the mistake was the spawn (F-49)

- **The sequence, worth seeing whole.** F-27: the spawned task must not own the
  stderr buffer → add `Arc<Mutex<Captured>>`. F-40: the task must not outlive the
  exchange → add `abort()`. F-49: `abort()` cannot help on cancellation, because
  dropping a `JoinHandle` detaches rather than cancels. Three findings, three
  repairs, one decision underneath all of them.
- **The signal I missed.** Three consecutive repairs to the same decision is
  evidence about the decision, not about the repairs. I treated each as its own
  defect because each *was* one.
- **The mistake was `tokio::spawn`.** The drain never needed a task; it needed to
  make progress concurrently, and `select!` inside the existing task does that
  identically. So the repair is a deletion: no spawn, no `Arc`, no `Mutex`, no
  `abort`, no join handle, and a plain `&mut Captured` on the caller's stack.
- **Verified both shapes** rather than reasoning about cancellation semantics: a
  sub-future's destructor runs the instant its parent is dropped; a spawned
  task's is still not running 100 ms later. Also confirmed the concurrency the
  whole thing exists for — 4000 stderr lines past the 64 KiB pipe buffer while
  the body read stdout, no deadlock.
- **D36 retired.** It claimed this was "the one place a lock is right". D14 had
  already given the right answer — brief §12 gives the host no concurrency to
  protect against — and this design manufactured the concurrency first and then
  justified the lock. Inventing a state space and then defending it is exactly
  what D14 exists to refuse. D44.

### 2026-08-25 — `null` means omission, except once (F-50)

- **Raised by:** the review, as a blocker. `{"next_check": null}` reaches `None`
  silently while `{"next_check": 45}` produces a reported discard — two
  non-string values, two treatments, one of them silent.
- **Verified before deciding:** `{}` and `{"next_check": null}` both deserialize
  to `None`; likewise `{}` and `{"protocol": null}`. Only `view` distinguishes
  them, via its presence-preserving deserializer.
- **Question put to the user:** is `null` an invalid value, or a synonym for
  omission?
- **Decided:** *"1.a"* — `null` ≡ omitted, stated explicitly.
- **Why, and it is about the person on the other end.** `null` is what an
  ordinary serializer emits for an absent optional. `json.dumps({"next_check":
  None})` is not a backend doing anything wrong, and reporting a discard against
  it would mean most well-formed messages carry a diagnostic. A wrong *type* is
  different in kind: `45` is a value the backend meant, in a shape the protocol
  cannot use.
- **So the behaviour was right and the silence was the defect.** The repair is a
  rule stated once: *an explicit `null` means what omission means, except where
  the protocol defines a distinct meaning for `null`* — one exception, named.
  That also explains, for the first time, why `view` carries machinery no other
  field has. The fixture asserts an **empty** discard list, because here the
  silence is the contract rather than the bug. R-51, D50.

### 2026-08-25 — A binding constraint that was false, not merely unenforced (F-51)

- **Raised by:** the review, as a blocker. ADR-001's Decision requires stratum 1
  to "remain buildable and testable with no renderer and no runtime in its
  dependency graph". The design answered ADR-002's T1 with "tokio is a stratum 2
  dependency and stratum 1 does not link it".
- **That was false.** Cargo resolves dependencies per crate target, not per
  module. In one crate with a plain tokio dependency, `cargo test` builds one
  graph containing tokio and `semantics/` has no separately selectable graph at
  all. Worse: the design *had already written down* that `cargo tree` cannot
  observe a module boundary inside one crate, and then leaned on precisely what
  that sentence rules out. AC-15's grep proved only that `semantics/` contains no
  `tokio` token.
- **Question put to the user:** feature-gate the runtime, or split to a workspace
  now on the literal reading of T1?
- **Decided:** *"2.a"* — feature gate.
- **What it costs and buys.** tokio becomes `optional = true` behind a `shell`
  feature; `shell/` gets one `#[cfg]`. Verified by building it: `cargo tree
  --no-default-features` has no tokio node, `cargo test --no-default-features`
  compiles and runs stratum 1 against serde, serde_json and jiff alone. Half of
  ADR-001 is now a **build gate**, inside one crate, which the ADR assumed had to
  wait for the split — so CD-1 gets stronger rather than merely more accurate.
- **Canon consequences, recorded rather than absorbed.** ADR-002 names Slint as
  "the first such dependency"; tokio arrived a slice earlier and was admitted
  only by gating it. CD-2 corrects that and records that "make it optional" is an
  available answer to T1 — while noting, from ADR-002's own rejected
  alternatives, why it does not extend to a Slint build-dependency and so does
  not defer the split.
- **The general lesson.** A constraint stated in canon can be *false* rather than
  merely unenforced, and the two need different responses: an unenforced rule
  wants a test, a false claim wants either a mechanism or an amendment. Three
  rounds of review had read this sentence and checked whether the design honoured
  it, rather than whether it was true. D49.

### 2026-08-25 — Uniqueness, and a recursion no answer could express (F-52, F-54)

- **F-52, raised by the review.** `Options` is a checked newtype because
  duplicate option ids make `respond` ambiguous. `UserResponse.values` is a map
  keyed by field id — and fields were a bare `Vec<Field>`. Identical defect,
  identical consequence, my own argument left unapplied one level down. Same
  shape as F-39.
- **Repaired as a rule, not a case:** *every identifier a response names must be
  unique within the scope that names it.* `Options`, `Fields` and `Alternatives`
  are that rule with constructors behind it. The rule was first written as "uses
  as a key", which covered the two cases in front of me and not the third — see
  F-58 below. The
  fixture that matters is the **negative** one — the same field id in two
  different options is legal — because that is what shows the scope is right
  rather than merely strict.
- **F-54, raised by me while disposing F-52.** `FieldKind::Choice` reused
  `Options`, whose `Opt` carries `fields`, so a choice field's options could
  carry fields recursively — while a response addresses one option and one flat
  value map. No way to say which nested option was chosen; nested field ids
  sharing a namespace with outer ones.
- **Question put to the user:** narrow the type, or define nested submission?
- **Decided:** *"4.a"* — narrow it. `Alternative { id, label }`. Deleting the
  recursion beats documenting it, brief §10.2 never puts fields on a field's
  options, and this is R-7's gold-plating risk caught before it shipped.
- **Worth recording against F-20**, which examined this same reuse and found no
  defect. F-20 was not careless — it checked the view side, where the reuse is
  harmless. The defect appears only when the type is read against the message
  that must carry an answer to it, which is the method that found F-31 and F-38
  as well. **Checking a type against itself is not checking it against its round
  trip**, and three separate findings have now come from that one distinction.
  D45, D46, I15, I16.


### 2026-08-25 — Self-check before round 5: four findings against my own repairs (F-55…F-58)

The round-4 batch was repaired, and then read again before the round-5 packet
was built — against the round each repair came from rather than against the
finding it answered. Four findings, all mine, none of which needed a user
decision: each is a repair failing to be what it already claimed.

- **F-55 — the F-54 repair silently dropped `fields` on a choice field's
  option.** The edge row said the key was "unmodelled on `Alternative`, so
  ignored under I10", which confuses layers: modelled-ness is a fact about the
  *wire* type, and `WireOpt` was cited by `WireChoice.options` and defined
  nowhere. `WireOpt` is now defined as the **view's** option type — the only
  place `fields` is admitted — and because `WireField.options` is
  `serde_json::Value`, dispatched by normalization rather than bound by serde,
  the dispatch raises `InapplicableKey { key: "fields", kind: "choice", at }`.
  This was F-45's defect reintroduced by the F-54 repair, on the same page that
  repairs F-45. Two things it establishes: a dangling type name is a real defect,
  because the question it leaves open — *does a choice field's option
  deserialize through the same type as a view's?* — was the question the repair
  turned on; and R-53 had been a `MUST NOT` with no error behind it, which is the
  second time this ledger has produced that shape (R-30, at F-28). This entry is
  itself late: F-55 was repaired in the batch and left out of this log, which
  F-56 caught.
- **F-56 — the round-4 repairs were never swept through their restatement
  sites.** A dozen sites still stated the contracts those repairs replaced: §5.4's
  step 5 still had one timeout "covering the whole exchange" (F-53's defect
  verbatim, contradicting its own sketch 55 lines below); R-47 still carried
  D42's reversed precedence rule; §5.1 still denied being a build gate 35 lines
  after D49 made it one; I11 and I13 still cited D41 and D42, both retired; D40
  still described a two-field `Exchange`; §10 still counted one canon-delta entry
  beside a row citing the second; the AC map still counted four commands where
  five were listed. Plus three dangling names, found by the two mechanical checks
  the repair adds to §9: `cleanup_only`, called and never defined — F-55's
  defect again, in the section F-55's repair had just touched;
  `CleanupFailure::TimedOut`, which every piece of prose calls `CleanupTimeout`,
  so the design argued a name in a whole paragraph and then did not use it; and
  `ViewId`, `OptionId`, `FieldId`, `Timestamp`, `Hints` and `Config`, written
  throughout §5 and declared nowhere.
- **What that costs and what changed.** §9 already carried the sweep as a
  standing review step, written after six of round 3's nine findings turned out
  to be one contract restated. It failed here not through carelessness on one
  change but because it was **phrased per-change against a batch of eight**, and
  no single change in the batch obviously owned it. So the trigger is now the
  batch: *before any repair batch is claimed complete*. Two mechanical checks
  join it, both of which have now produced findings — chase every struck decision
  id to whatever cites it, and check that every type or function named in §5 is
  defined in §5.
- **F-57 — the `shell` feature gate had no test-target plumbing.** A feature
  selects dependencies; it does not stop cargo building every test target. With
  the integration tier spawning processes on tokio and no `required-features`,
  `cargo test --no-default-features` — AC-15's build gate — would fail to compile
  the moment that tier existed. The F-51 probe passed only because the probe
  crate had no integration tests. Declared test targets with
  `required-features = ["shell"]`, `autotests = false`, `main.rs` entry points in
  the layout, and a second clippy run under `--no-default-features`, because a
  feature-gated crate has a build matrix and one column checked is a matrix
  unchecked. **A constraint enforced by a build command acquires the build's
  failure modes** — and a build gate that cannot run is worth less than the review
  gate it replaced, because it reads as green.
- **F-58 — the F-52 rule did not cover the newtype F-54 added.** R-52 and I15
  both said *every identifier used as a **key** in a response*. An alternative id
  is never a key: the answer to a `choice` field is an alternative id submitted
  as the value at `values[field_id]`. So the rule justified `Options` and
  `Fields` and not `Alternatives`, which D45 introduced under it. Widened, not
  narrowed — the newtype is right and the sentence was wrong: *every identifier a
  response names must be unique within the scope that names it*, with keys and
  values named as the two ways it can be named. A duplicate key leaves one of a
  pair unaddressable; a duplicate value leaves the answer ambiguous. Same defect
  from opposite sides.
- **Method, since three of the four came from one move.** F-55, F-56 and F-58 all
  came from reading a repair against the round it came from, not against the
  finding it answered — F-55 against F-45, F-56 against round 3's sweep lesson,
  F-58 against F-52's own generalisation. The ledger's older lesson was *check a
  type against the message that carries its answer*; this round's is the same
  move one level up: **check a rule against the cases it did not come from.** A
  rule that covers exactly the cases that produced it has not been generalised
  yet, it has been summarised.

### 2026-08-26 — Round 5: two blockers in repairs nobody had re-read (F-59…F-63)

A second fresh reviewer, given the round-4 batch to verify. It found no defect in
any round-4 repair. It found two blockers in round 3's, which rounds 3, 4 and the
round-4 self-check had all walked past.

- **F-59 — `ExitStatus` was unreachable.** F-53's repair collapsed two grace
  timeouts into one cleanup budget, which was right, and took `child.wait()` out
  of the timed region with them, which was not. Worse than the reviewer saw:
  cleanup *kills* before it waits, so the status was destroyed on every path, not
  merely unreported. D15 and R-40 both require a non-zero exit to discard a body
  that parsed; the prose four paragraphs above the sketch still listed "await
  exit" as step 4; R-40's fixture could not have passed.
- **Repaired by putting the wait back inside `config.timeout`**, on the merits
  rather than for symmetry: waiting for a backend to exit is the *backend's*
  opportunity to respond, not the host's disposal of it. Disposal stays in
  `CLEANUP_LIMIT`; D48's total is unchanged. The status is read before the bytes
  are trusted, so no path lets a parsed response outlive the exit code that
  disclaimed it.
- **F-60 — AC-5 claimed what R-48 had already conceded.** F-41 narrowed R-48 for
  cancellation: no host code runs on a dropped future, so `kill_on_drop` is the
  only mechanism and the design says plainly it is best-effort. AC-5 was never
  narrowed with it and still said a cancelled exchange "leaves nothing behind".
  F-49's repair made the *task* half structural — there is nothing to detach — and
  the child half was never held.
- **Question put to the user:** narrow AC-5, or make it true?
- **Decided:** narrow it, no follow-up. Making it true needs a supervisor task
  outside the exchange — the detached task F-49 deleted, in a different hat — or a
  process-group kill, which brief §14 refuses. Both cost more than the gap. So
  the slice ships a stated, bounded gap on one path, and §5.4 names slice 003 as
  the slice that meets it written down rather than discovers it, since a timer is
  the first thing that can cancel an exchange. D54.
- **F-61 — the namespace comment I added in round 4 was false three lines below
  itself.** `Alternative` carried an `OptionId` while `UserResponse.option` also
  carried one. F-58 had answered this from the rule end and stopped. Now
  `AlternativeId`, plus `DuplicateAlternativeId` and `EmptyAlternatives`, because
  `DuplicateOptionId` raised against an alternative asserts the id *is* an option
  id — the F-48 naming mistake exactly. F-54, F-58, F-61: one defect, three
  findings, each looking complete when it landed. The tell was there again, since
  F-58's repair needed two clauses to describe one type.
- **F-62 — the lints I9 named were never on.** `unwrap_used`, `expect_used`,
  `indexing_slicing` and the arithmetic lints are restriction lints and
  allow-by-default; `-D warnings` never enabled them. Turned on per module, as
  R-46 already specified, because the blanket form is what F-35 caught this design
  violating on a value the host itself created. D53.
- **The shape all three share, now three rounds running:** *a claim is held by a
  mechanism or it is not held.* F-51 was canon no build enforced. F-57 was a build
  gate that could not run. F-62 is an invariant whose named enforcement was off by
  default. Every one of them read as green.
- **F-63, raised by me sweeping this batch — and the first finding against an
  empirical claim.** The document's headline measurement describes a backend that
  answers correctly while a grandchild holds the pipes: response delivered, only
  cleanup fails, 902 ms. The probe's case D is `(sleep 30) &`, and a bare subshell
  inherits stdout too — so stdout never reaches EOF, the body cannot complete, and
  that case *times out*. Its 902 ms is timeout **plus** cleanup budget: the cost of
  a failed exchange, not a delivered one. Five sites described a case that had
  never been run.
- **Run it, then.** `(sleep 30) >/dev/null &` holds stderr only: `Ok(response)`
  with `cleanup` set, 303 ms. Both cases are now tabulated separately, and the
  stdout-too case turns out to be the `Err` + `Some` row of §5.4's table, which
  the design had called meaningful without ever producing one. F-48's naming
  conclusion survives — the child exits and is reaped in both — and its evidence
  is now the run that actually shows it.
- **What that costs the method.** "Execute any claim that can be executed" has
  been this review's best instrument for four rounds. It does not protect against
  executing one case and describing another, and the gap was invisible precisely
  because the numbers were real. The narrower rule: **a measurement is evidence
  for the sentence next to it only if the sentence names the case that was run.**
- **And an honest limitation, now stated rather than implied:** a host cannot
  tell "the backend is still writing" from "the backend exited and something else
  holds the pipe". Both are a pipe with no EOF. `config.timeout` is the only
  answer available; stopping at the end of the first JSON document would silently
  accept a truncated response as complete.

### 2026-08-26 — Design accepted; review closed with repairs unverified

- **Question put to the user:** run round 6 against the round-5 repairs, or stop?
- **Decided:** stop. *"I think we're good"*, followed by an instruction to set up
  the handover for planning. Taken as acceptance of the design and as the
  authority to close `review-design.md`, per `docs/AGENTS.md`'s design stage,
  which requires the user's acceptance before planning begins.
- **What that accepts, stated plainly rather than left implicit.** The ledger's
  own definition of done is *every finding `verified` or `withdrawn`*, and it does
  not meet it. Sixteen repairs — F-48…F-63 — are `repaired` with no independent
  confirmation. All seven blockers were repaired, so nothing is outstanding by
  severity; what is outstanding is the second opinion. The ledger's State says
  this in those words rather than recording a closure it did not earn.
- **The magnitude, from the ledger's own base rate:** roughly 0.2 defects per
  repair and falling across rounds 2–4, so two to four defects are likely still
  resident, most of them in §5.4 — the section restructured three times in three
  rounds and the one whose last restructure (F-59) nothing has reviewed.
- **Why stopping is defensible anyway.** Round 5 found no defect in any round-4
  repair; the yield has moved from the repairs to the original material and then
  to nothing, which is what convergence looks like. The remaining defects are
  cheap to find where it matters, because §5.4's structure is executable and the
  probe that executes it is preserved. Planning is also not a one-way door: the
  plan stage re-reads the design against the code, and `docs/AGENTS.md` says
  explicitly that unresolved design issues emerging there go back to design.
- **Recorded as a risk rather than as a resolution.** The next agent inherits it
  in the handover, and audit inherits it in the Synthesis.

### 2026-08-26 — Lint config adopted from `../doctrine`, with the dead-code lints softened

- **Asked:** copy doctrine's lint configuration into goad, then soften the
  no-dead-code lints "just a smidge".
- **Landed:** `clippy.toml` verbatim minus doctrine's `std::fs::write` entry
  (it names a `fsutil::write_atomic` that does not exist here), and doctrine's
  `[workspace.lints.*]` tables de-workspaced into `Cargo.toml` `[lints.rust]` /
  `[lints.clippy]`. `Cargo.toml` otherwise is §5.1's manifest snippet verbatim.
  Writing it now runs ahead of PHASE-01/EX-1, which was the user's call when
  asked where the tables should live.
- **The mechanic, measured rather than assumed.** The first attempt demoted
  `dead_code` to `warn` and gave `warnings = "deny"` a lower Cargo `priority` so
  the per-lint level would win. It does not: rustc applies the `warnings`
  pseudo-group over an explicit per-lint `--warn` regardless of the order Cargo
  emits them, so `-D dead-code implied by -D warnings` and the build still fails.
  Verified in a scratch crate carrying the real lint table. With
  `warnings = "deny"` set, `warn` is not a reachable level for any lint below it
  — every entry is deny-or-allow.
- **Decided:** drop `warnings = "deny"` from the manifest entirely and leave the
  strictness on the command line, where design §9 already puts it. Consequence:
  lints the manifest names explicitly (`unused` minus the carve-outs, the whole
  clippy list) still hard-error in the inner loop; unenumerated warn-by-default
  lints warn locally and fail at the phase gate. That is the softening — it
  moves *when* strictness bites, not whether.
- **`dead_code` and `unreachable_pub` carved out of the `unused` group**, left
  explicitly at `warn` so the carve-out reads as a decision rather than an
  oversight. Both are transient by construction here: the
  `--no-default-features` column drops `shell`, so any `semantics/` item whose
  only caller is in stratum 2 is genuinely dead there. Denying it would buy
  nothing but `#[cfg_attr(not(feature = "shell"), expect(dead_code, …))]`
  scattered through stratum 1. The rest of the group stays denied —
  `unused_imports`, `unused_variables`, `unused_mut`, `path_statements` have no
  legitimate transient case, and `unused_must_use` is a correctness lint.
- **Only the second clippy line carries `-A dead_code -A unreachable_pub`.**
  The default-features line stays strict, so dead code still fails a phase gate;
  the carve-out covers the structural case and nothing wider. Measured: that
  column still fails on `unused_imports` and `unused_mut` with the carve-out in
  place.
- **D53 / R-46 partially superseded.** `unwrap_used`, `expect_used` and
  `indexing_slicing` are now crate-wide denies rather than module-level: they
  are cheap everywhere, and `allow_attributes_without_reason = "deny"` answers
  R-46's drift argument by pricing an allow at a written reason.
  `arithmetic_side_effects` stays module-level — crate-wide it fires on every
  loop counter, which is the case R-46 was right about. §9 records this inline.
- **`plan.md` PHASE-01 updated to match:** `clippy.toml` added to its surfaces,
  and EX-1's second clippy line now carries the two `-A`s, with an instruction to
  copy that line from §9 rather than from memory.

### 2026-08-26 — `just lint` added

- **Asked:** a lint task in the justfile. The file existed but was empty.
- **Landed:** `default: lint`, and a `lint` recipe running both clippy columns
  from §9 verbatim, with the reason the second column's two `-A`s are
  load-bearing written above it and a pointer saying §9 changes first.
  `justfile` added to PHASE-01's surfaces.
- **Verified by executing it**, not by reading it: against a scratch crate
  carrying the real lint table, `just lint` exits 101 on a dead private fn plus
  an unused import and 0 once both are gone — so column 1 does refuse dead code
  and just does abort before column 2 rather than reporting the last line's
  status.
- **Deliberately not added:** `fmt`, `test`, `build`, an aggregate `check`. Only
  a lint task was asked for, and §9's six commands are the authority — a
  half-populated `check` recipe that omits two of them is worse than none.
- **Open:** `just` is on PATH from the user's nix profile, not from
  `flake.nix` `devToolPkgs`, so the devshell is not self-contained for these
  recipes. Not changed without asking.

### 2026-08-27 — D53 amended, and `just` adopted as the canonical runner

Two decisions, both raised in `notes.md` as open against the design after
scaffolding landed outside the phase flow.

**D53 amended — the no-panic lints split by cost.**

- **Asked:** `Cargo.toml` and `design.md` §9 said `unwrap_used`, `expect_used`
  and `indexing_slicing` were crate-wide; I9, D53, §9's own AC-6 row and
  `draft-spec.md` §7's R-46 row still said module-level. Three ways out: amend
  D53 to the crate-wide form, revert the manifest to D53 as written, or defer to
  audit.
- **Decided:** amend. D53 now reads: `unwrap_used`, `expect_used` and
  `indexing_slicing` crate-wide in `[lints.clippy]`; `arithmetic_side_effects`
  module-level `#![deny(…)]` on the modules handling backend-derived data.
- **Why the substance is better than D53 as first written.** D53's per-module
  scoping existed to stop F-35's case — an `unwrap` on `child.stdin.take()`, a
  value the *host* created — being `#[allow]`ed away silently, on the argument
  that an allow-by-default lint that has been allowed back is indistinguishable
  from one that was never on. `allow_attributes_without_reason = "deny"`, with
  `allow_attributes = "deny"` beside it, answers that with a mechanism rather
  than with scope: the exception becomes an `#[expect(…, reason = …)]` that is
  greppable and countable. The F-35 case gets written down at the site instead
  of avoided. `arithmetic_side_effects` stays scoped because crate-wide it fires
  on every loop counter, where the allows would outnumber the catches — that is
  the case R-46 was right about, and no reason-carrying allow makes it tolerable.
- **Swept, this being the F-56 defect exactly — a contract repaired at its
  primary site and left standing in its restatements:** `design.md` §5.5 (I9's
  "held by"), §7 (D53), §9 (the passage, which had a supersession note four
  lines above a paragraph contradicting it — both are gone, replaced by one
  statement of the split), §9's AC-6 row, and `draft-spec.md` §7's R-46 row.
  `plan.md`'s Overview item 4 no longer reports the divergence as open.

**The crate-wide form fires inside tests, and `clippy.toml` carves that out.**

- **Found while implementing the amendment**, not decided: crate-wide includes
  both test targets, so `unwrap()` on a fixture, `v[0]` on a known vector or a
  `panic!` in a should-not-reach arm fails the phase gate, and the only escape is
  an `#[expect(…, reason = …)]` on every asserting test.
- **Measured, not assumed.** A scratch crate carrying goad's lint table fails
  `cargo clippy --all-targets -- -D warnings` with five errors across a
  `#[cfg(test)]` module and a `tests/` target, and exits 0 with
  `allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests` and
  `allow-indexing-slicing-in-tests` in `clippy.toml`. All four are accepted by
  this toolchain; none is an unknown key.
- **Nothing is given up.** I9 is about paths handling backend-derived data at run
  time; a test is not one, and a test that unwraps is asserting — the panic is
  the reporting mechanism. `unwrap_in_result = "deny"` is deliberately **not**
  carved out.
- **Consequence for every break-it-and-revert proof:** it must break the lint in
  host code. An `unwrap()` under `tests/` passes and proves nothing. That was
  review finding F-14, which found `plan.md` still saying "anywhere".
- **This is an author decision under a user decision**, taken because the
  amendment is unimplementable without it. Reversible: reverting the four keys
  restores the strict reading at the cost of an `#[expect]` per asserting test.

**`just` adopted as the canonical runner, in full.**

- **Asked:** `AGENTS.md` must name the verification commands (AC-10), and the
  justfile had one recipe. Adopt `just` fully, keep raw cargo, or adopt without
  fixing the dev shell.
- **Decided:** adopt fully.
- **`just` was never missing from the dev shell.** `notes.md` recorded it as
  coming from the user's nix profile, and the justfile's own header said so.
  Both were wrong: `just` has been in `flake.nix` `devToolPkgs` since commit
  `6489521`. Checked, not read — `nix develop --command` resolves both `just`
  (1.58.0) and `deno` (2.9.4) to store paths. A shell entered before those
  landed does not see them, which is what the stale claim was actually
  observing. So AC-1's "clean clone in the nix dev shell" holds with `just` in
  it, and no `flake.nix` change was needed.
- **Recipes:** `build`, `test`, `test-stratum1`, `lint` (both clippy columns),
  `fmt-check`, plus `check` as the phase gate and `fmt` outside it. `default` is
  now `check` rather than `lint`. This reverses 2026-08-26's "deliberately not
  added" — that entry's reasoning was that a half-populated `check` omitting two
  of the six is worse than none, and the answer is that it is no longer
  half-populated.
- **The mirroring is checkable rather than asserted.** `just -n check` prints
  the command list without running it, and it must be the same commands with the
  same arguments in the same order as §9's block. **Not the same characters** —
  §9 carries inline comments and wraps the second clippy line, and `just -n`
  prints neither, so a literal comparison fails on a correct justfile. That was
  this entry's first wording and PHASE-01/VA-3's, and review finding F-13 caught
  both. Verified: it emits §9's six, in §9's order.
- **`design.md` §9** now says outright that its block is canonical, that the
  justfile mirrors it, and that `AGENTS.md` names the recipes. **`plan.md`**:
  PHASE-01/EX-1 and every phase's VA-1 are `just check`; PHASE-09/EX-1 has
  `AGENTS.md` naming the recipes and VA-1 running the gate from a clean clone
  entered through `nix develop`.

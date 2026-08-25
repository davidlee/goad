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

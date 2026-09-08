# Design log — Slice 004

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

### 2026-09-08 — commit the working tree before opening the slice folder

- **Asked:** the tree carried the 2026-09-08 startup fix, the demo backend and
  config, the `justfile` run/demo recipes, README, `docs/AGENTS.md` §Tiers, the
  roadmap re-cut and the two template edits. Commit first, leave them, or hand
  the commit to the user?
- **Recommended:** commit first, so slice 004's folder lands on a clean base and
  the slice's own diff is only slice 004.
- **Decided:** commit now.
- **Consequence:** `3f35eb2`. Nothing about the slice's scope.

### 2026-09-08 — what bounds an event-triggered evaluation

- **Asked:** SPEC-002/R-5 states that the scheduled-firing spacing bounds no
  other stimulus and that a host adding one must decide separately how it is
  bounded. Four options put: (a) its own minimum spacing on its own anchor,
  mirroring R-4; (b) coalescing only — at most one event-triggered evaluation
  pending, no time bound; (c) both; (d) nothing new, naming the existing
  capacity-1 command channel as the bound.
- **Recommended:** (a). It bounds a watcher emitting at machine rate by
  construction, and it reads as a sibling of R-4 rather than as an exception to
  it.
- **Decided:** (a) — an event-triggered evaluation fires no sooner than a fixed
  minimum spacing after the event-triggered evaluation that preceded it, on an
  anchor of its own. The scheduled anchor (R-4/ADR-004) neither clears the event
  anchor nor is cleared by it; the host holds two monotonic anchors and neither
  writes the other.
- **Consequence:** **the slice raises to tier 2.** Writing this rule down amends
  SPEC-002, which is canon, so `docs/AGENTS.md` §Tiers requires the full
  lifecycle. `canon-delta.md` is opened in this folder and SPEC-002 is not
  edited until audit. The spacing's *value* is not decided here.

### 2026-09-08 — whose source, kind and timestamp reach the backend

- **Asked:** an ingested event becomes an `evaluate`. SPEC-001/R-7 requires that
  request to carry the host's own instant *and* an event with source, kind,
  timestamp and data; R-56 fixes `source: "host"` for evaluations the host
  originates, and an ingested one is originated by the watcher. Three options
  put: (a) pass the four envelope fields through verbatim and reserve the source
  string `"host"` to the host, refusing an envelope that claims it; (b) pass
  through with nothing reserved, exactly brief §7 and §19; (c) the host
  originates it — `source: "host"`, a fourth `kind`, the watcher's envelope
  nested in `data`.
- **Recommended:** (a). It keeps brief §19's shape on the wire and keeps
  `source == "host"` a discriminator a backend can trust, so R-56's three kinds
  cannot be forged by a watcher.
- **Decided:** (a).
- **Consequence:** the host's business in the envelope is the four fields'
  *presence and shape*, not their meaning; `data` is carried opaquely as it is
  everywhere else (SPEC-001/R-9). The request's `now` is the host's instant; the
  event's `timestamp` is the watcher's. **A second canon-delta entry** (CD-2) is
  owed against SPEC-001/R-56 reserving the `"host"` source. **No fourth
  `event.kind` is added** — R-56's open set is not exercised by this slice after
  all, because an ingested evaluation is not host-originated.

### 2026-09-08 — the socket answers the writer

- **Asked:** does ingress answer, or is it fire-and-forget? Three options put:
  (a) one reply line per envelope, accepted or refused-naming-why, then close;
  (b) fire and forget, reported only in the host's diagnostics and stderr;
  (c) reply on refusal only, silence meaning accepted.
- **Recommended:** (a). Slice 005's exit code has something real to mean, this
  slice's acceptance is checkable from a shell one-liner, and the success case is
  observable rather than inferred from a timeout.
- **Decided:** (a) — every envelope is answered on the same connection, then the
  host closes it. A refusal is *also* reported in the host's own diagnostics
  surface, so it is visible to a person who is not the writer.
- **Consequence:** the slice defines **a second wire format** — the ingress
  reply — which is a contract of its own and is design's to specify. It is not
  the host/backend protocol and SPEC-001 does not govern it. Carries an open
  question: whose voice a refusal speaks in. The host's failure taxonomy has two
  sides, host and backend; an ingress refusal is a third party's fault and
  neither existing side names it.

### 2026-09-08 — the socket path comes from an optional config key

- **Asked:** config key, XDG runtime default, or both? Three options put:
  (a) an optional config key, absent meaning no listener; (b) a required config
  key; (c) an XDG runtime default with an optional override.
- **Recommended:** (a). The XDG default-path work is roadmap §006's declared
  surface and doing it here duplicates it; an optional key leaves every existing
  config file valid, `examples/demo.toml` included; and a test binds inside a
  tempdir with no environment variable, which keeps the suite parallel-safe.
- **Decided:** (a) — a new optional key in `config.toml`. Absent means the host
  runs with no listener, which is today's behaviour exactly.
- **Consequence:** ingress is **opt-in**, so the "no listener" arm is a real
  state the design must name rather than an error case. The section and key
  names are design's; `Config` is `deny_unknown_fields`, so the key must exist
  in the canonical form before any config may write it. `examples/demo.toml`
  gains the key or does not, which is a demo decision, not a scoping one.

### 2026-09-08 — a stale socket is reclaimed; any other bind failure is fatal

- **Asked:** the path exists at startup, and more generally `bind` fails — what
  does the host do? Three options put: (a) connect-probe, then bind, with a
  residual failure fatal; (b) the same probe, with a residual failure degraded —
  the host starts with no listener and reports it; (c) never unlink, an existing
  path is always fatal.
- **Recommended:** (a). A crash never leaves a host that needs a manual `rm`,
  a second instance never steals the socket from the first, and writing the
  config key is asking for ingress — so failing to provide it is a
  half-configured host rather than a running one.
- **Decided:** (a). Connect to the path first: refused means stale, so unlink and
  bind; connected means a live host owns it, so refuse to start. A regular file
  at the path, a permission failure, or a missing parent directory is likewise a
  startup error naming what it found.
- **Consequence:** ingress failure joins `StartupError` (`crates/goad/src/
  startup.rs`) rather than the diagnostics surface, and the host's exit code
  carries it. **The `"a backend failure never takes the host down"` invariant is
  untouched** — this is a configuration failure, not a backend one, and the
  distinction is stated rather than assumed. A race remains and is design's:
  between the probe and the bind, a second host may bind the same path.

### 2026-09-08 — access is owner-only, by an explicitly set mode

- **Asked:** who may write to the socket? Three options put: (a) mode 0600, set
  explicitly rather than left to the ambient umask, as the whole rule;
  (b) 0600 plus an `SO_PEERCRED` uid check on every accepted connection;
  (c) nothing beyond the path, the umask deciding.
- **Recommended:** (a). It is a rule rather than an accident, it is `stat`-able
  so an acceptance criterion can assert it, and it needs no platform-specific
  syscall in stratum 2.
- **Decided:** (a) — the host sets the mode to owner-only itself and does not
  rely on the umask it was started under.
- **Consequence:** **the containing directory is the user's responsibility** and
  the design says so rather than defending it: a directory another user can write
  lets that user replace the socket, and the mode does not reach that. Stated as
  a documented limit, not a residual defect. No protocol-level authentication and
  no peer-credential check in this slice.

### 2026-09-08 — an event the host cannot act on now is refused, not queued

- **Asked:** an envelope arrives while an exchange is in flight (SPEC-002/R-9
  admits one), or inside the event spacing CD-1 introduces. Three options put:
  (a) refuse it, naming which of the two; (b) accept and hold one pending,
  refusing a second; (c) accept and block the connection until the evaluation is
  dispatched.
- **Recommended:** (a). Brief §7 puts filtering and debouncing in the watcher, so
  the retry decision is the watcher's; the reply already exists to hand it the
  fact; and it makes CD-1's bound observable at ingress instead of expressed as a
  silent delay.
- **Decided:** (a) — the host holds **no queue**. Every envelope gets one reply
  and that reply always tells the truth about what the host did with it.
- **Consequence:** CD-1's requirement gains a clause: an event arriving inside
  the spacing is refused, naming the bound. The refusal reasons are a small
  closed set the design enumerates — at least *malformed*, *reserved source*,
  *engaged*, *too soon* — and each is checkable from a shell one-liner. Events
  **are** lost under load, by design and visibly, and the design says so.

### 2026-09-08 — SPEC-002 OQ-4 stays open

- **Asked:** an accepted event's evaluation can replace a view a person is
  mid-answering, so their answer is refused as superseded. SPEC-002 OQ-4 carries
  this for scheduled firings and is deliberately unresolved. Three options put:
  (a) leave it, recording that a second stimulus raises the rate; (b) refuse an
  event while a view is outstanding, as a further refusal reason; (c) resolve
  OQ-4 for every stimulus here.
- **Recommended:** (a). (b) is nearly free but means a prompt nobody dismisses
  disables ingress indefinitely, which for an autostarted daily driver is a worse
  failure than a superseded answer; (c) takes a decision SPEC-002 says plausibly
  belongs to the backend, in a slice whose subject is a socket.
- **Decided:** (a).
- **Consequence:** no host behaviour changes. CD-1 gains one sentence noting that
  ingress raises the rate at which OQ-4's case is reached; `slice-004.md` carries
  it as an open question and as a follow-up. **This is a non-goal, not an
  omission.**

### 2026-09-08 — slice-004.md accepted

- **Asked:** accept the scoped slice — purpose, scope, non-goals, AC-1..AC-13,
  governing canon, OQ-1..OQ-10, tier 2.
- **Decided:** accepted, unmodified.
- **Consequence:** stage set to `design`. Scoping is closed; the acceptance
  criteria ids are immutable from here. Design is a fresh agent's job and starts
  from this file, `canon-delta.md` CD-1 and CD-2, and this log.

### 2026-09-08 — the ingress contract gets a draft spec of its own

- **Asked:** CD-1 amends SPEC-002 (the bound) and CD-2 amends SPEC-001 (the
  reserved source). Neither reaches the socket, the envelope, the reply format
  or the refusal set, and no existing spec's *Owns* line covers them
  (`research.md` C-1). Three options put: (a) draft a spec in the slice folder,
  promoted at audit; (b) design.md only, nothing promoted beyond CD-1 and CD-2;
  (c) extend SPEC-001 with an ingress section.
- **Recommended:** (a). Slice 005's CLI is a second client of that contract, and
  a wire format governed only by a closed slice's design is the failure POL-001
  exists because of.
- **Decided:** (a).
- **Consequence:** `docs/slices/004/draft-spec.md` is opened from
  `docs/templates/spec.md` and is this slice's working authority for the socket,
  the envelope, the reply and the refusal taxonomy, alongside `canon-delta.md`
  for the two amendments. It is numbered **SPEC-NNN until promotion**; nothing
  outside the slice may cite it (`docs/AGENTS.md` §*Canon that does not exist
  yet*). Audit owes a promotion or an abandonment in writing. **OQ-2 and OQ-5
  are now answers inside that document rather than inside `design.md`.**

### 2026-09-08 — the loop judges every envelope; ingress is watched in both selects

- **Asked:** AC-4 requires *"the host is engaged"* as a refusal reason, but
  `serve` does not poll its outer `select!` while an exchange is in flight
  (`research.md` F1, `controller.rs:505`), so an ingress arm added there alone
  could never produce it — the connection would wait in the accept backlog and
  be served late. Three options put: (a) the loop judges, with a listener task
  doing shape checks only and every arrival carrying its own reply channel, and
  `serve` watching ingress in **both** selects; (b) the listener judges, reading
  `engaged` and the event anchor through a shared handle; (c) a capacity-1
  channel whose fullness is the refusal.
- **Recommended:** (a). No state is shared or duplicated — the loop stays the
  only reader of its own anchors — and *engaged* is answered promptly because
  the arm that answers it is inside the exchange rather than outside it.
- **Decided:** (a).
- **Consequence:** the refusal set **partitions by who can answer it**: shape
  refusals (not one JSON document, a missing or wrong-typed field, a reserved
  source) are the listener's, in stratum 2, with no host state involved; state
  refusals (engaged, too soon) are the loop's, in stratum 3. The design must say
  what the inner arm does with an arrival that is *already* a shape refusal —
  it is answered with its own reason, not relabelled *engaged*. `serve` gains a
  parameter, which touches every call site in the renderer test tier (F17).

### 2026-09-08 — the envelope normalizes in stratum 2; the reserved-source rule splits by subject

- **Asked:** OQ-4 (where the envelope's normalization lives) and OQ-5 (where
  CD-2's rule is stated) are one seam from two sides. Three options put:
  (a) stratum 2 beside the listener, with SPEC-001/R-56 stating the *emission*
  rule and SPEC-003 stating the *ingress refusal*; (b) stratum 1 beside the
  protocol's own normalization, rule in SPEC-001; (c) stratum 2, with both
  halves of the rule in SPEC-001.
- **Recommended:** (a), on `research.md` F6: a **user-authored** format
  normalizes in stratum 2 — `config.rs`'s `File` → `Config` is the precedent —
  and a **backend-authored** one in stratum 1. The two differ by who wrote the
  bytes, not by what kind of work it is; the watcher is neither the host nor the
  backend.
- **Decided:** (a).
- **Consequence:** stratum 1 gains nothing and learns no format no backend ever
  sees; the ingress module owns a permissive wire type whose normalization is
  the only door into `Event`. **CD-2 is now two statements in two documents:**
  SPEC-001/R-56 says what a host may emit and gives a backend its licence to
  read `source == "host"`; SPEC-003 says an ingested envelope claiming it is
  refused, naming the reason and citing R-56 for why. `canon-delta.md` CD-2 is
  updated to say exactly that, so its "open at scoping" clause is discharged.

### 2026-09-08 — the event spacing is the same three seconds, stated per stimulus class

- **Asked:** OQ-1 and CD-1's three open items — the spacing's value, whether it
  is configurable, and how the rule is stated so a third stimulus inherits the
  shape rather than the number. Three options put: (a) the same
  `MINIMUM_SPACING` against a second independent anchor, not configurable, with
  the rule stated as a property of *a bounded stimulus class*; (b) its own,
  shorter constant; (c) a configured value with a default.
- **Recommended:** (a). ADR-004's argument against configurability transfers
  intact — a bound a misconfiguration can remove is not a bound — and two
  constants meaning the same kind of thing are two things free to drift when no
  evidence fixes either number.
- **Decided:** (a).
- **Consequence:** SPEC-002 gains a **principle** as well as a requirement:
  *each bounded stimulus class is spaced from the previous firing of its own
  class, on its own monotonic anchor, and no anchor is written by another's
  firing.* R-4 becomes the scheduled instance of it and the new R-12 the
  ingested one, so a third stimulus adds an anchor rather than a number.
  `canon-delta.md` CD-1 is updated to carry the principle and the value. In the
  code: **one** `MINIMUM_SPACING` const, **two** anchors in `serve`, each with
  one write site. A watcher emitting two distinct events inside three seconds
  loses the second — refused, visibly — and brief §7 puts that coalescing in the
  watcher.

### 2026-09-08 — the reply is one versioned JSON object, newline-terminated

- **Asked:** OQ-2, the reply's wire format and whether it declares a version.
  Three options put: (a) one JSON object with `protocol`, `accepted`, and on a
  refusal a machine-readable `reason` from the closed set plus a human `detail`;
  (b) the same without a version field; (c) a plain text line.
- **Recommended:** (a). A shell one-liner can grep it, slice 005 can parse it
  for an exit code, and the version field is the mistake SPEC-001/R-1 exists to
  avoid making twice.
- **Decided:** (a).
- **Consequence:** the **envelope carries no version and none is required of
  it** — brief §19's illustrative envelope has none, and demanding one would
  break AC-1's one-liner. That asymmetry is exactly SPEC-001/R-1 and R-2's: the
  host declares its own version and requires none of the sender. The `reason`
  vocabulary is closed and normative in SPEC-003, and AC-4's enumeration is its
  floor: *malformed*, *invalid_envelope*, *reserved_source*, *engaged*,
  *too_soon*. `detail` is prose for a person and nothing may branch on it.

### 2026-09-08 — the envelope is strict: four fields, and no key beside them

- **Asked:** what the envelope admits — unknown keys, required fields, and OQ-10
  (whether an envelope's `timestamp` is the host's business beyond its shape).
  Three options put: (a) strict — all four fields required, `source` and `kind`
  non-empty strings, `timestamp` an RFC 3339 instant with an explicit offset,
  `data` any JSON value carried opaquely, and an unknown top-level key refused
  naming it; (b) the same with unknown keys ignored, mirroring SPEC-001/R-4 and
  R-5; (c) strict but with `timestamp` optional, the host filling its own
  instant.
- **Recommended:** (a). The envelope has a **designed extension point** —
  `data` is opaque and unbounded — so a key beside the four is a mistake rather
  than a newer watcher, and ignoring it would silently discard something the
  writer meant to send, which is what SPEC-001/R-20 forbids in the analogous
  case. (c) would have the host author a field SPEC-001/R-7 gives to the event's
  originator.
- **Decided:** (a).
- **Consequence:** the host makes **no judgement about the timestamp's
  distance from now** — a 1970 or a 3000 value is carried, because judging it
  would be domain meaning. OQ-10 is answered: shape only, and the shape is
  R-22's. One consequence is owed in writing (`research.md` C-3): `Event`'s
  `timestamp` is a modelled `Timestamp` (`canonical.rs:490-497`), so the host
  **re-serialises** it — an envelope written `+10:00` reaches the backend as the
  same instant spelled `Z`. **AC-1's "verbatim" holds as the same instant, not
  as the same bytes**, and `slice-004.md` carries that reading when the design
  is accepted. The alternative — making `Event.timestamp` a string — would
  weaken the host-originated path for the sake of the ingested one.

### 2026-09-08 — the probe/bind race is documented, not closed

- **Asked:** OQ-6. Between the connect-probe and the bind, two hosts starting
  together can both find the path stale and both bind. Three options put:
  (a) state it as a limit and do not close it here; (b) close it with an atomic
  `hard_link` onto the target, EEXIST being the arbitration; (c) close it with an
  advisory lock file under `File::try_lock`.
- **Recommended:** (a), on `research.md` F15: **nothing prevents two goad
  processes today** — `main.rs` takes no lock, checks no pidfile and consults no
  existing process. Two hosts against one config already both start, both show a
  window and both invoke the backend. The race does not create that; it names
  one symptom of it, and closing it inside the ingress module is a partial
  single-instance guarantee smuggled in under another name.
- **Decided:** (a).
- **Consequence:** **AC-8 stays exactly as strong as it reads** — a path a live
  host holds is a startup error and the running host keeps its socket — and the
  design says in writing that it does not assert the race is closed. SPEC-003
  carries the limit as a non-normative note; `slice-004.md` §Follow-ups gains
  **single-instance enforcement** as a future slice, which is where the real fix
  belongs because it is not about the socket.

### 2026-09-08 — every connection is bounded in bytes and in time

- **Asked:** a connection that opens and never writes would hold the socket
  indefinitely. SPEC-001/R-41 and R-43 are the precedent — every read from an
  untrusted writer is bounded and the budget is stated. Three options put:
  (a) sequential accept with a fixed per-connection time budget and byte bound,
  each overrun answered before the close; (b) a task per connection with bounded
  concurrency; (c) a byte bound only.
- **Recommended:** (a). No unbounded spawning, head-of-line blocking bounded by
  the budget, and nothing refused in silence (AC-3).
- **Decided:** (a) — one connection at a time; **500 ms** to deliver an envelope,
  `CLEANUP_LIMIT`'s sibling (`process.rs:30`), and **64 KiB** of it.
- **Consequence:** the closed refusal set gains **two** reasons beyond AC-4's
  floor — `timed_out` and `too_large` — which AC-4 admits ("at least"). Both are
  shape refusals, so both are the listener's and neither reaches the loop. The
  budgets are host constants in stratum 2 and are stated in SPEC-003 rather than
  hidden, exactly as R-41 requires of the transport's own pair.

### 2026-09-08 — the diagnostics surface shows refusals only

- **Asked:** OQ-7 — whether an ingested evaluation is distinguishable in the
  diagnostics surface, and what a person needs there to debug their own watcher.
  Three options put: (a) refusals only, one line, through a new
  `Refused::Ingress` variant; (b) refusals plus a line naming the stimulus
  behind the current presentation; (c) refusals plus a retained ring buffer of
  recent arrivals.
- **Recommended:** (a). The **writer's own reply is the debugging channel** for
  the person running the watcher; the diagnostics surface exists for the person
  who is not the writer, and an accepted envelope is already visible through the
  evaluation it causes.
- **Decided:** (a).
- **Consequence:** `Refused` (`diagnostics.rs:52-62`) gains one variant carrying
  the reason and its detail. An ingress refusal **replaces** the surface, as
  every other refusal does (`research.md` F7) — that is the module's existing
  shape, not a new compromise. (b) is a follow-up if a watcher ever proves hard
  to debug without it; it would have to thread the stimulus through `receive`
  and `Diagnostics::of`, which is stratum-3 plumbing this slice does not need.

### 2026-09-08 — socat joins the devshell; the demo listens in the checkout

- **Asked:** two parts of AC-13's vehicle. First, what writes the envelope:
  `socat`, `nc` and `ncat` are on the user's profile PATH but **none is declared
  in `flake.nix`**, and the `justfile` holds its recipes to "works from a clean
  clone in the dev shell". Options: (a) add `socat` to the devshell; (b) use
  `deno`, already declared; (c) document `nc -U` and add nothing. Second,
  whether `examples/demo.toml` configures ingress: (a) yes, at a relative path
  in the checkout, gitignored; (b) yes, under `/tmp`; (c) no — a second example
  config and a second recipe.
- **Recommended:** (a) and (a). `socat` makes the one-liner read as the thing it
  is and keeps the clean-clone standard true; a socket in the user's own
  checkout **satisfies** the documented limit — the containing directory is the
  user's responsibility — rather than waiving it in a world-writable directory.
- **Decided:** (a) and (a).
- **Consequence:** `flake.nix` `devToolPkgs` gains `socat`, which is this slice's
  only environment change and is asked for rather than assumed (`CLAUDE.md`
  §Environment). `examples/demo.toml` gains the ingress section pointing at
  `./goad-demo.sock`, `.gitignore` gains that path, and **`just demo` is AC-13's
  vehicle with no second recipe**. `examples/shell/backend.sh` prompts on every
  evaluation that is not a `respond`, so an emitted event produces a window with
  no change to the demo backend.

### 2026-09-08 — AC-6 is a claim about the two anchors, not about the deadline

- **Asked:** AC-6's first clause — *"an event-triggered evaluation neither
  delays nor advances a scheduled firing"* — is not achievable as literally
  worded. Every completed exchange re-arms the pending deadline from the
  instruction the backend returned (`controller.rs:508-512`), and an ingested
  evaluation is an exchange: a backend answering `next_check: "60 seconds"`
  re-resolves from the ingested request's instant, moving the next scheduled
  firing. Avoiding that means discarding an instruction the backend actually
  sent, which SPEC-001/R-26 forbids. A person's *Check now* already does the
  same thing today. Three options put: (a) read AC-6 as a claim about the two
  **anchors**; (b) have an ingested exchange not re-arm the deadline; (c) take
  the criterion back to scoping.
- **Recommended:** (a). It is the claim ADR-004 §Verification says no existing
  test could reach, and it is falsifiable in both directions.
- **Decided:** (a).
- **Consequence:** the design asserts, and the tests hold: **an ingested firing
  never writes the scheduled floor, and a scheduled firing never writes the
  event floor** — two anchors, two write sites, neither reachable from the
  other. What moves the pending deadline after an ingested exchange is the
  backend's own `next_check`, exactly as after a person's evaluation, which
  SPEC-002/R-5 already permits. `slice-004.md` carries this reading beside AC-6
  when the design is accepted; the criterion's id and intent are unchanged.

### 2026-09-08 — AC-7's "unchanged bodies" is about assertions, not call sites

- **Asked:** `serve` gains one parameter, which touches **23 call sites** across
  `main.rs`, `tests/renderer/{wiring,scheduling}.rs`,
  `tests/event_loop/closing.rs` and `tests/event_loop_schedule/scheduling.rs`.
  Three options put: (a) unchanged assertions with mechanically updated call
  sites; (b) keep the signature and wrap it in a second entry point; (c) hang
  the ingress handle off `Host` or `Controller`.
- **Recommended:** (a). AC-7's substance is *with the key absent the host
  behaves exactly as it does today*, and unchanged assertions are what prove it.
  (b) puts two doors on one loop, which this codebase refuses on principle;
  (c) buys coupling to avoid an argument.
- **Decided:** (a).
- **Consequence:** every existing test keeps its assertions, its scripted
  backend and its bounds; each `serve(...)` gains `Ingress::none()` — the handle
  that **parks forever** because no socket is bound, which is what makes "no
  listener" a state named in the type rather than a branch in the loop.

### 2026-09-08 — Autonomy grant for 004, and the design review is spawned here

- **Asked:** whether the adversarial design review runs as an agent spawned by
  the orchestrator or as a prompt handed to a separate session; and whether
  003's autonomy grant carries into 004.
- **Recommended:** spawn here, on 003's shape — the reviewer is kept alive
  across rounds, repairs go to a fresh agent rather than back to the author.
- **Decided:** spawn here; grant renewed as **decide everything except canon**.
- **Consequence:** finding dispositions, repair agents and stage sequencing are
  the orchestrator's. Reserved to the user: canon endorsement, plan acceptance,
  and any product question touching the wire contract or a spec. The review is
  tier 2 — its own `review-design.md`, rounds unbounded.

### 2026-09-08 — round 1 of the design review is dispositioned under the standing grant

- **Asked:** how the eleven findings of `review-design.md` round 1 are
  dispositioned. Under the autonomy grant renewed earlier today — *decide
  everything except canon* — this is the orchestrator's call, not the user's.
- **Decided (orchestrator):** F-1 and F-5..F-10 `fix-now`; F-2, F-3, F-4 and
  F-11 `doc-wrong` — the artefact is the defect, the thing it describes is not.
  Nothing is `tolerated`, `follow-up` or `settle-in-code`; no finding was
  withdrawn, and every citation was checked before it was acted on.
- **Consequence — the substantive changes:**
  - **AC-6 gains a third test** (F-1). The *advances* direction was asserted by
    nothing, so ADR-004's undischarged case was uncovered: a scheduled firing at
    T₀, an ingested firing at T₀+ε, a `next_check` due at T₀+1 s, and the
    scheduled evaluation not reaching the backend before T₀+3 s. CD-3 now says
    which of the three tests discharges the debt and what the other two hold.
    AC-6 itself is unchanged.
  - **R-15 is restated to what the design can hold** (F-2). The diagnostics
    surface is one whole value presented between exchanges, so `engaged` — and
    any refusal decided inside an exchange, and the shutdown `unavailable` —
    is reply-only. Every refusal still reaches its writer, so AC-3 stands. A
    Follow-up carries making the rest visible to a person; its shape is not
    designed here.
  - **The stratum-2 placement stands, and its argument is replaced** (F-3). The
    user-authored / backend-authored split was invented in this slice and is
    deleted from `design.md` §2 F6 and D-3 and from `slice-004.md` §Scope. What
    decides it is whose contract the normalization serves. `Event` is a
    transparent record whose only fallible field is a `jiff::Timestamp`, and
    stratum 3 has constructed one since slice 002, so normalizing into it from
    stratum 2 opens no second door. The decision gets its own ADR at
    reconciliation.
  - **CD-2 qualifies R-56's first clause** (F-4) — "on its own account" — and
    stops claiming R-56's decided content is unaltered. SPEC-001 §9 References
    joins its section list.
  - **`retry_after_ms` goes on the wire now** (F-10), on `too_soon` only, which
    answers `draft-spec.md` OQ-1 rather than deferring a wire change into slice
    005.
  - Smaller: CD-1 reaches SPEC-002 §2 Boundaries and §6 (F-5); `unavailable`
    takes both deciders (F-6); a non-object top-level JSON value is
    `invalid_envelope` (F-7); `Ingress::arrival` returns `Option<Arrival>` so a
    dead accept task is distinguishable from `none()`'s park and is reported,
    with the unlinked-socket case stated as a residue rather than claimed away
    (F-8); §6.4's bounds are per **read**, with the unbounded wait for judgement
    stated and tied to the main thread everything else already depends on (F-9);
    D-18 states the three-way choice it actually was (F-11).
- **Not done, and reported:** `research.md` F6 and its C-3 conclusion still
  carry the invented split. It is a research record, outside the four artefacts
  this repair was scoped to, and it is the orchestrator's call whether it is
  corrected or left as the record of what was believed at research time.

### 2026-09-08 — the research record is corrected, not preserved as it was believed

- **Asked:** the repair of F-3 left `research.md` still carrying the inference
  F-3 called invented — F6, the Precedents paragraph and conclusion C-3 — and
  the repair agent reported it as outside its scope.
- **Decided:** correct it. `research.md` is verified research output that the
  plan agent reads, not an append-only log; `docs/AGENTS.md` §Slice says it is
  repeated as new details emerge. A false inference left standing there is a
  trap for the next stage.
- **Consequence:** the three sites keep their observations and lose the
  inference. What replaces it is D-3's reason: the two normalizations differ by
  which contract they hold, and stratum 1's holds SPEC-001 in one place. C-3
  now says F6 does not answer OQ-4 by itself.

### 2026-09-08 — review round 2 dispositioned: propagation, not redesign

- **Asked:** how `review-design.md` round 2's six findings, F-12..F-17, are
  dispositioned. Under the standing autonomy grant — *decide everything except
  canon* — this is the orchestrator's call, not the user's.
- **Decided (orchestrator):** F-12, F-13, F-14, F-16 and F-17 `fix-now`; F-15
  `settle-in-code`. Nothing `aligned`, `tolerated`, `follow-up` or withdrawn.
  Every citation was checked against the file and the line before it was acted
  on; none failed.
- **The class, named:** every round-2 finding was a **propagation gap the
  round-1 repairs opened** — a repair right at the site it touched and not
  carried to the artefacts that state the same thing elsewhere. Repairs were
  therefore made against the class: after each change, its siblings across
  `design.md`, `draft-spec.md`, `canon-delta.md`, `slice-004.md` and
  `research.md` were found and taken with it.
- **Consequence — the substantive changes:**
  - **AC-6 case (ii) gains the clause it depends on** (F-12). The ingested
    firing at T₀+ε must be one whose **own exchange resolves to a deadline no
    later than T₀+1 s**, because every exchange re-arms the pending deadline
    from what its own backend answered (SPEC-001/R-26,
    `controller.rs:507-512`). Without it the case ADR-004 was waiting for would
    be discharged by a test that could not have failed under the boolean either.
  - **`unavailable` admits a third cause** (F-13), and the `None` path is
    carried into the loop's shape. The closed channel is disposed of in the arm
    that observes it, **before** a `Fired` is built, so `refusal_re_arms` and
    the standing deadline are untouched; the third cause is **permanent for the
    life of the process** and is the one refusal with no writer to fall back on.
    The token set stays closed at eight — the definition widened, not the set.
  - **Shape beats state in both `select!`s** (F-14). The order of judgement is
    restated as the ingress **arms'**, with a shape refusal as step 1 inside an
    exchange as much as outside it, which is what makes R-15's negative test
    buildable. §5.4's sequence diagram is redrawn: it disagreed in more branches
    than the one the finding named.
  - **R-15's rate coupling is measured, not argued** (F-15, `settle-in-code`).
    R-15's MUST stands. The phase that builds `serve`'s ingress arms and the
    anchor settles it, and the test is **AC-5's flat-out writer extended to
    bound presentations**. The coupling is stated beside I-3 and named as risk
    **R6**. If the phase does not bound it, the finding returns `contested`.
  - **`retry_after_ms` rounds up** (F-16), pinned in R-14 rather than left to
    the phase, so the requirement's own sentence is true of the field.
  - **Cross-document requirement ids are qualified** (F-17). SPEC-002 and the
    draft both number their new requirement 12 and neither may be renumbered.
- **Checked and reported, not changed:** `draft-spec.md` does **not** number
  itself. It is titled *SPEC-NNN* and says it takes its number at promotion, so
  `docs/AGENTS.md`'s rule is already met; the cross-references were written in
  forms that survive it being given any number.

### 2026-09-08 — ingress dying mid-exchange is folded, not presented

- **Asked:** F-13's repair forced a statement the finding did not make. When the
  ingress channel closes **while an exchange is in flight**, the inner arm can
  fold the refusal but cannot present it — `absorb` overwrites the diagnostics
  slot before the loop's next `present`. Guaranteeing the presentation needs
  state consulted after the exchange, which falsifies §5.3's claim that the
  whole of the new retained state is one instant.
- **Decided:** fold and resume. No new state, no new mechanism.
- **Consequence:** a permanent, process-lifetime failure can go unseen by a
  person in that one interleaving. It is not silent to the **watcher**, which
  stops being able to connect. The interleaving is on `slice-004.md`
  §Follow-ups beside F-2's residue, and §5.5 states it. What is bought is
  `serve`'s stack unchanged in the function §8 R2 already names as the one at
  risk of outgrowing review.

### 2026-09-08 — review round 3: two contests upheld, two new findings, all four `fix-now`

- **Asked:** how round 3's four findings are dispositioned. F-13 and F-14 came
  back `contested`, which is not terminal — they return to open and need
  re-disposition. F-18 and F-19 are new. Under the standing autonomy grant this
  is the orchestrator's call.
- **Decided (orchestrator):** all four `fix-now`. Both contests are **upheld**:
  the round-2 repairs were right where they reached and each stopped one
  document short. Every citation was checked against the file and the line
  before it was acted on; none failed, and nothing was withdrawn.
- **The class, again, and for the third round running:** a repair correct at the
  site it touched and not carried to every artefact that states the same thing.
  Round 3 adds a second face of it — a repair that fixes the raised defect and
  states a *new* claim in the fixing, which then has to be true on its own terms.
  F-14's redraw and F-18's two sentences are both that.
- **Consequence — the substantive changes:**
  - **R-15's universal is narrowed to envelopes** (F-13). `draft-spec.md` R-15
    is the sentence a second host implementation is held to, and it still said
    *every refusal without exception*, which §6.3 falsifies two sections below
    about the ingress-stopped `unavailable`. It now says **every envelope's**,
    with the exception named. One sibling swept — `slice-004.md`'s *Refusals a
    person cannot see* follow-up; nothing else quantified that way.
  - **The reply is drawn when it happens** (F-14). Hoisting it below the `alt`
    made the accepted branch depict the writer being answered *after* the
    backend exchange, which `draft-spec.md` §5's diagram, I-2 (`engaged` would
    be unreachable) and §5.5's *writer hangs up* case all deny. It is drawn
    inside each branch, before the `evaluate` in the accepted one. The prose no
    longer cites **I-1** as the reason: I-1 is about how many replies leave, not
    when. The two diagrams were checked against each other and agree.
  - **Measurement is not prevention** (F-18). R6's signal was one-for-one
    presentations, which §5.5 states as the design — a signal already satisfied
    on the day it was written. It becomes a threshold: the number AC-5 records
    exceeding what a person can tolerate. `draft-spec.md` §7's R-12 row said the
    AC-5 bound *keeps* an untrusted writer from pacing the display; a test
    detects rather than prevents, so the row now says the count is **recorded**
    and what it holds is that a refusal keeps costing one presentation.
    `design.md` §9's AC-5 row carried the same overclaim and took the same fix.
    F-15's disposition is untouched — R-15's MUST stands.
  - **The arm is named** (F-19). §5.3's `event_floor_until` row said *the ingress
    arm* in both columns after §5.4 made the arms plural; it now says **the
    outer** ingress arm, citing the steps. Two siblings outside the table took
    the same word — `design.md` D-13 and `slice-004.md` §Scope.

### 2026-09-08 — the design is accepted as reviewed, and AC-3 keeps its reading

- **Asked:** acceptance again, the design having changed under a four-round
  review — the AC-6 blocker, R-15's narrowed bound, CD-2's qualifier and D-3's
  replaced argument — and whether AC-3 keeps the reading the consistency pass
  authored rather than transcribed.
- **Decided:** accepted, and the reading stays.
- **Consequence:** the design stage closes and the plan opens. AC-3 carries the
  one exception `draft-spec.md` R-8 admits — a connection may close unanswered
  when the host process itself is gone — and the note that AC-3 quantifies over
  envelopes, so the writer-less refusal falls outside it rather than breaching
  it. Without the reading AC-3 is unsatisfiable as literally worded, which is
  the same failure mode F-1 found in AC-6.

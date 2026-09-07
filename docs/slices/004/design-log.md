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

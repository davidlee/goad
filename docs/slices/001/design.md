# Design — Slice 001: Protocol core and process backend transport

<!-- The *current* design, not its history. Revision chronology, review
     findings, and dispositions live in `design-log.md`.
     Reference forms: canon by id (`SPEC-003 §4`, `ADR-007`, `POL-002`);
     doc-local refs bare — OQ-1 (§6), D1 (§7), R1 (§8). Ids are immutable. -->

## 1. Design problem

The repository contains no Rust code, no crate manifest, and no canon describing
the protocol. This design settles the internal shape of the host's first code:
the canonical protocol types, the normalization that produces them from
permissive wire input, schedule resolution, and one backend transport that
spawns a user program per invocation.

The thing being designed is a contract, not a feature. The host is defined by
what it refuses to understand: it moves `evaluate`, `respond` and `schedule`
between a renderer and a user-owned backend, and every domain meaning lives on
the backend side of that boundary (brief §3.1, §21.16). So the protocol types
*are* the product surface, and they must admit capabilities no renderer yet
implements — option-scoped fields, richer content forms, natural-language
schedules — without being narrowed to whatever a first renderer happens to need.

Settling that headless is the point of doing it before the GUI. Under fixtures
and integration tests, the contract is decided by the protocol's own
requirements. Once a renderer exists, every unresolved question in the contract
gets answered by what is convenient to draw.

The boundary of this design:

- **In.** Module layout of the three ADR-001 strata inside one crate; the
  canonical type shapes and their normalization from wire forms; the error
  taxonomy and where it splits at the stratum seam; the transport trait and its
  spawn-per-invocation implementation on tokio; host operational state held in
  memory; TOML configuration of three values; the fixture corpus layout and what
  the test suite must demonstrate.
- **Out.** Anything with a window in it (slice 002); anything that wakes on a
  clock (slice 003); anything that ingests an external event (slice 004); the
  persistent socket transport (slice 005). Each constrains this design only in
  that it must not be foreclosed.

This design's own contract is a draft, not canon. Per the OQ-1 decision the
protocol specification is written incrementally in
`docs/slices/001/draft-spec.md` as design, execution and audit settle it, and
promoted to `docs/specs/` at close once it has been reconciled against what
shipped. During execution that draft, the Rust types and the AC-9 fixture corpus
together are the whole contract: the draft is authoritative about intent and the
tests are authoritative about behaviour, and where the two disagree that is a
finding to be dispositioned, not a licence to pick whichever is convenient.

## 2. Current state

There is nothing to change; there is only nothing. `src/` and `tests/` exist and
are empty, and no `Cargo.toml` exists (`research.md` Thread 2, verified). Every
file this slice touches is a new file, so no existing code constrains the design
and no existing code can be cited as precedent.

Canon is nearly as bare. `docs/specs/` and `docs/policy/` are empty;
`docs/adr/` contained nothing when research ran and now holds exactly ADR-001
and ADR-002, both raised by this slice's own scoping.

Root `AGENTS.md` was zero bytes when research ran and is no longer: it now
carries a pointer to the methodology, the canon rule, the dev-shell facts, and
four working principles. AC-10 is therefore additive rather than from-scratch —
what it still needs is the protocol-specific material brief §15.1 asks for: that
the host does not understand the user's domain, the permissive-wire /
canonical-internal rule, the warning against narrowing the protocol to the
current renderer, and the verification commands. `CLAUDE.md` remains a symlink
to it.

What *is* established, and therefore load-bearing here:

- The toolchain works. `cargo 1.99.0-beta.1` from `rust-bin.beta.latest.default`,
  `pkg-config` on PATH, crates.io reachable (`research.md`, verified).
- The GUI stack is de-risked and out of scope. Slint 1.17.1 builds and opens a
  Wayland window in this dev shell with no `flake.nix` change; mechanics are
  harvested in `docs/memory/slint-build-mechanics.md`, and the spike was deleted
  in `c8ab319`. Nothing from it entered `src/`, so ADR-002's T1 has not fired.
- No async runtime is in the tree, so tokio is an addition this design makes
  rather than one it inherits.
- deno is not in `devToolPkgs`. The test suite cannot run a TypeScript backend
  until this slice adds it.

The consequence worth stating plainly, from `research.md`'s cross-thread
finding: the absence of code and the absence of canon coincide, so whatever this
slice does becomes precedent **by default rather than by decision**. That is the
argument for spending design effort on module placement and error-type shape
here, where they cost nothing to choose, rather than in slice 002 where they
cost a refactor.

## 3. Forces & constraints

**Canon that binds.**

- **ADR-001** — three strata, dependencies downward only. Stratum 1 (protocol
  types, normalization, schedule resolution) must build and test with no
  renderer and no async runtime in its graph. This is the single strongest
  constraint on §5.1 and it decides the error taxonomy's shape in §5.2.
- **ADR-002** — one crate until a trigger fires. Its Verification section
  requires the triggers to be checked and the answer recorded in the design of
  any slice that adds a dependency or a binary. This slice adds dependencies, so:
  **T1** (a dependency stratum 1 must not need) — does not fire; tokio is a
  stratum 2 dependency and stratum 1 does not link it. **T2** (a second binary)
  — does not fire; at most one binary. **T3** (renderer build dominating test
  wall-clock) — does not fire; there is no renderer. **Verdict: one crate,
  strata as modules.**

**From the brief, as intent rather than canon.**

- §3.3, liberal grammar and strict canonical semantics. The permissive/canonical
  seam is a *type* boundary, not a validation pass: after normalization there
  must be no way to hold an unnormalized value. Ambiguity fails; it is never
  guessed.
- §13 failure taxonomy, §12 `view_id` discipline, §9 latest-valid-wins with an
  invalid instruction preserving rather than disabling, §6.2 the process
  transport's shape.
- §14, decisive for tone throughout: backends are **trusted user programs, not
  sandboxed plugins**. Nothing in the code, the config, the example or its
  comments may imply isolation. deno runs with `-A` for exactly this reason, and
  that must be stated where a reader would otherwise mistake default-deny
  permissions for a security model.
- §3.7 and §15, agents are the intended editors. This is why a convention that
  exists only in prose is weak, and why `AGENTS.md` (AC-10) is a deliverable
  rather than a courtesy.

**Technical and cost.**

- tokio with `process`, `time`, `rt`, `io-util` resolves 14 unique dependencies;
  the smol family for the same job needs 31 (`research.md`, verified). `net`
  waits for slice 005.
- Slint's 411 dependencies and ~19s clean debug build are the number T1 will be
  paid in next slice. It constrains nothing today, but it is why module
  boundaries must stay *relocatable* — ADR-002 requires the split to be a file
  move, not a redesign.
- Hermeticity: `cargo test` must be able to spawn a backend with no build step
  and no `node_modules`. This is why deno, and it is a constraint on the example
  as much as on the fixtures.
- AC-1 sets the verification floor: build, test, lint at zero warnings, and
  format check, all from a clean clone in the dev shell.

**House style.** The one precedent that exists is documentary: `flake.nix`
comments explain *why* a non-obvious thing was done, at length, rather than
restating what the code does (`research.md`). Rust comments here follow that —
sparse, but expansive where the reason is not visible from the code.

## 4. Guiding principles

**P1 — Canonical is a type, not a promise.**
After normalization there must be no way to *hold* an unnormalized value. Not a
validated field, not a `String` that everyone agrees is an RFC 3339 timestamp —
a distinct type that cannot be constructed except by normalization succeeding.
Brief §3.3 draws this line and ADR-001 puts a stratum boundary on it; P1 is what
makes the line checkable by the compiler instead of by review.

*What this loses:* the single-struct-with-optional-fields design, which is fewer
types and less mapping code. It also loses the convenience of reaching for the
raw wire value later, downstream of the seam, when it would be handy.

**P2 — An invalid value costs the sender its effect, never the host its
function.**
The host reports the failure and continues from the state it already had. A
malformed `next_check` loses that instruction; it does not disable scheduling
(brief §9). A malformed response loses that response; it does not poison the
transport. A stale `view_id` loses that answer; it does not clear the
outstanding interaction.

This sits in tension with P1 by design, and the tension resolves by scope: P1
governs whether a bad value may be *represented* (it may not), P2 governs how
far its blast radius reaches (as far as the value, no further). Neither licenses
absorbing an error silently — every refusal is a named error and reaches
diagnostics.

*Granularity.* Failure is whole-message by default. A part may be discarded on
its own only when:

> its absence is already a modelled state with defined semantics, distinct from
> "we failed to read it."

`next_check` satisfies this. An absent `next_check` is a real state with a
defined meaning — brief §9's "retain an existing valid scheduled check, else use
the configured default poll interval" — so discarding the field lands the host
in a state it already knows how to occupy. Brief §13 corroborates by listing
"invalid scheduling value" as a failure mode *distinct from* "protocol-invalid
response"; the entry would be redundant if a bad `next_check` invalidated the
message.

`view` fails the test, which is why an invalid view rejects the whole message.
The only absent-view state is `view: null`, and per brief §11 that is a positive
assertion — "nothing to show right now". Degrading an unreadable view to `null`
would have the host assert on the backend's behalf that there is nothing to
show, when the truth is that it could not tell. That is the invented semantics
brief §3.3 forbids.

Applied to this slice:

| part invalid | outcome |
|---|---|
| `next_check` | discarded; §9 fallback applies; typed error to diagnostics; rest of the message accepted |
| envelope or protocol version | whole message rejected |
| `view` or `choice` structure | whole message rejected |
| unsupported required primitive | whole message rejected — brief §13, "fail clearly" |
| option-scoped field no renderer implements | not a normalization failure at all; admitted by the types, unimplemented downstream (brief §22.3) |

This costs P1 nothing. The canonical response holds `schedule:
Option<CanonicalInstant>` where `None` means "no instruction supplied"; an
invalid value normalizes to `None` **plus** a reported error, and the error
travels alongside the canonical value rather than inside it. So normalization
remains a total function into a genuinely canonical type — see §5.2, where its
result carries a discard list rather than being a bare `Result`.

*What this loses:* the simpler, louder discipline of failing hard on bad input.
The code carries a "keep the previous value" path a fail-fast design would not
have, and that path needs its own tests. The discard list is also a shape every
caller must handle, where a bare `Result` would have been ignorable.

**P3 — Shape the seams for the second implementation the brief already names,
and no further.**
The transport is a trait now, with one implementor, because brief §6 specifies
two transports and slice 005 builds the other; the trait's shape must not
presume spawn-per-invocation. The protocol types admit option-scoped fields and
richer content forms because brief §22.3 says a later renderer implements them.

The bound is the second half. A seam is justified by a named future
implementation, not by imagining one. Where the brief names nothing, this slice
builds the concrete thing — configuration is a struct with three fields, not a
provider abstraction; host state is a struct in memory, not a persistence trait
awaiting a backend.

*What this loses:* both directions. Against "do not abstract until you need it",
it accepts a trait with one implementor. Against "make it all pluggable", it
refuses abstractions for futures the brief has not committed to.

## 5. Proposed design

### 5.1 System model

One crate, three strata as ADR-001 defines them, **grouped by stratum at the top
level of `src/`** and named with the ADR's own words:

```
src/
  lib.rs              # stratum wiring and the crate's public surface
  semantics/          # STRATUM 1 — pure. No I/O, no async, no tokio.
    protocol/
      wire.rs         #   permissive deserialization targets
      canonical.rs    #   canonical types; constructible only via normalize
      normalize.rs    #   wire -> Normalized<canonical>
    schedule.rs       #   pure resolution: latest-valid-wins
    error.rs          #   parse and validation errors
  shell/              # STRATUM 2 — I/O. May use semantics. Never uses bin.
    backend/
      transport.rs    #   the trait (async at its boundary)
      process.rs      #   spawn-per-invocation on tokio
    config.rs         #   TOML: command, timeout, default poll interval
    state.rs          #   outstanding view_id, resolved schedule (in memory)
    host.rs           #   composition: transport -> normalize -> resolve -> state
    error.rs          #   transport errors, wrapping semantics::Error
tests/
  protocol/           # fixture-driven, stratum 1 only (AC-9)
  integration/        # round trip through the process transport (AC-7)
  backends/           # the deliberately-misbehaving fixtures + the bash guard
examples/
  typescript/         # the showcase backend, deno
```

```mermaid
flowchart TD
  subgraph S3["stratum 3 — entry points"]
    direction LR
    E["(empty this slice)<br/>slice 002 renderer · slice 004 goad emit"]
  end
  subgraph S2["stratum 2 — shell"]
    direction LR
    T["backend::transport (trait)"]
    P["backend::process (tokio)"]
    H["host (public surface)"]
    C["config"]
    ST["state"]
    SE["shell::error"]
  end
  subgraph S1["stratum 1 — semantics"]
    direction LR
    N["protocol::normalize"]
    CA["protocol::canonical"]
    SC["schedule"]
    PE["semantics::error"]
  end
  S3 --> S2
  S2 --> S1
  P --> T
  SE -.wraps.-> PE
  style S1 fill:#eef7ee,stroke:#4a4
  style S2 fill:#eef2f7,stroke:#46a
  style S3 fill:#f7f2ee,stroke:#a64,stroke-dasharray: 4 3
```

**Why grouped by stratum rather than by topic.** Two reasons, both about the
weakness ADR-002 admits — that until the split, ADR-001 has no compiler behind
it.

1. It makes a violation visible in the import line itself. `use crate::shell::…`
   inside `src/semantics/` is wrong on sight, without the reader needing to know
   which topic belongs to which stratum. Brief §3.7 makes agents the editors,
   and an agent reads the line in front of it.
2. ADR-002 requires the eventual split to be a file move. Grouped this way it
   literally is: `git mv src/semantics crates/goad-semantics/src`. Grouped by
   topic, the split first requires deciding which topic is which stratum — which
   is the redesign ADR-002 says must not be necessary.

The names are lifted verbatim from ADR-001 ("pure semantic core", "I/O shell")
so the path leads a reader to the governing document.

**Stratum 3 is declared and empty.** This slice ships a library with no binary.
No acceptance criterion needs one, and P3 says build the concrete thing only
where the brief names it — the brief names a renderer (slice 002) and `goad
emit` (slice 004), neither of which is here. A debug binary for manual poking
would be unrequested scope. Side benefit: the integration tests can only reach
the crate through its public API, so the library surface is forced to be usable
by something other than itself.

**Mechanical enforcement of the boundary (AC-15).** ADR-002 names its own whole
risk as ADR-001 having no compiler behind it during the one-crate period. That
is partly fixable here for very little: a test that reads the files under
`src/semantics/` and asserts none of them mentions `crate::shell`, `crate::bin`
or `tokio`, failing if it finds no files to inspect so that a rename cannot pass
it vacuously. Crude — it is a string search over source text — but it catches
the actual failure mode, which is an agent writing an upward `use`. It also
gives the no-tokio-in-stratum-1 constraint a check it could not otherwise have,
since `cargo tree` cannot see a boundary inside a single crate. AC-11 already
establishes grep-checkable structure as acceptable here, so this is the same
instrument aimed at the other invariant.

This does **not** promote ADR-001's verification from review gate to build gate.
It checks three known tokens, so it catches the common case, not the class; the
ADR's own statement that this is a review gate until the strata become crates
stands as written. Recorded in §10 so audit disposes of it deliberately rather
than discovering it.

### 5.2 Interfaces & contracts

**Wire and canonical are separate type families, and only inbound data has
both.** Requests are host-authored, so there is one type and it is canonical;
nothing untrusted arrives on that path. Responses arrive from user code, so they
have a permissive wire twin. This halves the type count relative to mirroring
everything, and the asymmetry is principled — we control what we emit.

#### Inbound: wire types

```rust
// semantics/protocol/wire.rs
#[derive(Deserialize)]
pub struct WireResponse {
  #[serde(default)] pub protocol:   Option<u32>,
  #[serde(default)] pub view:       Option<WireView>,
  #[serde(default)] pub next_check: Option<serde_json::Value>,
}
```

Three deliberate choices here:

- **No `deny_unknown_fields`, anywhere inbound.** Brief §13: unknown optional
  fields are ignored.
- **`view` absent and `view: null` are the same state.** `#[serde(default)]` on
  `Option` makes them identical without a code path. Brief §8.2 writes the
  explicit null; being permissive about the omission costs nothing.
- **`next_check` is typed `serde_json::Value`, not `Option<String>`.** This is
  the one place the wire type is looser than the JSON we expect, and P2 is the
  reason: if it were `Option<String>`, then `"next_check": 45` would be a serde
  failure that kills the whole message — violating the granularity rule, which
  requires that field to be discardable on its own. A loose wire type is what
  makes a precise error possible.

The protocol version is handled asymmetrically, because the two directions have
different authorship. The host always **writes** `"protocol": 1` on requests. On
a response it **accepts** the field's absence — brief §8.2's own examples omit
it, so requiring it would reject every backend written against the brief — but
**rejects** a response declaring a version the host does not know, since
proceeding would be guessing at semantics. Brief §13's "versioned from day one"
is satisfied by the envelope carrying the field, not by both sides being
compelled to send it.

#### Inbound: canonical types

```rust
// semantics/protocol/canonical.rs
pub struct Response { pub view: Option<View>, pub schedule: Option<Timestamp> }
//                              ^ None = nothing to show    ^ None = no instruction supplied

pub enum View { Choice(Choice) }

pub struct Choice {
  pub title:   String,
  pub body:    Option<Content>,
  pub options: Options,            // newtype: >= 1, ids unique
}

pub struct Opt { pub id: OptionId, pub label: String, pub fields: Vec<Field> }

pub enum Content { Text(String), Markdown(String), Html(String), Uri(String) }

pub struct Field { pub id: FieldId, pub kind: FieldKind, pub label: String, pub hints: Hints }

pub enum FieldKind {
  Text, Boolean, DateTime,
  Number { min: Option<f64>, max: Option<f64> },
  Choice { options: Options },
}
```

`Options` is a newtype with a checked constructor, not a bare `Vec`: a choice
with zero options is unrenderable, and duplicate option ids make a later
`respond` ambiguous about which option it names. Both are §3.3 ambiguities, so
both are rejected at normalization rather than tolerated. P1 says the canonical
type should not be *able* to hold either.

**The line between `FieldKind` and `hints` is brief §3.4's line**: "the backend
expresses semantics, the renderer chooses widgets". Bounds on a number are
semantics — they constrain valid answers — so they live in `FieldKind`.
`multiline`, `placeholder`, `units`, `suggestions` are presentation, so they live
in `hints`, which is an open map the host does not interpret. That matters
because brief §10.2 calls its hint list "likely presentation hints **over
time**": fixing them into a struct now would narrow the protocol to today's
guesses, while an open map admits new hints with no version bump. The host must
never branch on a hint key — if it needs to, the thing was semantics and belongs
in `FieldKind`.

All four `Content` variants and all `FieldKind`s are **admitted and rendered by
nobody** in this slice. That is P3 with brief §11.1 and §22.3 naming the future
implementor.

An unrecognised `view.kind` must produce `ProtocolError::UnsupportedPrimitive`
carrying the offending kind string, not a generic deserialization failure —
AC-6 requires it as a distinct error and brief §13 wants it debuggable. The
serde encoding that achieves this (a fallback variant, or reading `kind` before
dispatching) is left to implementation; the contract is the named error and the
retained string.

#### Outbound: requests

```rust
// semantics/protocol/canonical.rs
pub enum Request { Evaluate(Evaluate), Respond(Respond) }
//  serialized with "protocol": 1 and "type": "evaluate" | "respond"

pub struct Evaluate { pub now: Timestamp, pub event: Event }
pub struct Respond  { pub view_id: ViewId, pub now: Timestamp, pub response: UserResponse }

pub struct Event {
  pub source: String, pub kind: String,
  pub timestamp: Timestamp, pub data: serde_json::Value,   // opaque, brief §7
}
pub struct UserResponse { pub option: OptionId, pub values: BTreeMap<FieldId, serde_json::Value> }
```

#### Normalization

```rust
// semantics/protocol/normalize.rs
pub fn normalize_response(wire: WireResponse, now: Timestamp)
  -> Result<Normalized<Response>, ProtocolError>;

pub struct Normalized<T> { pub value: T, pub discarded: Vec<Discarded> }

pub enum Discarded {
  Schedule { raw: serde_json::Value, reason: ScheduleError },
}
```

`now` is a parameter, not a clock read — that is how relative durations resolve
without stratum 1 acquiring I/O, and it makes every normalization test
deterministic with no injected clock abstraction.

`Discarded` is a closed enum with one variant on purpose. P2's eligibility test
is meant to be applied deliberately, so adding a second variant is exactly the
moment someone has to justify it against that test. A `Vec<(String, Error)>`
would let a discard slip in without the argument.

#### Errors — the AC-6 taxonomy, split at the stratum seam

```rust
// semantics/error.rs  — stratum 1: parse and validation
pub enum ProtocolError {
  Json(serde_json::Error),
  UnsupportedProtocolVersion { found: u32 },
  UnsupportedPrimitive { kind: String },
  MissingField { field: &'static str },
  EmptyOptions,
  DuplicateOptionId { id: String },
  Schedule(ScheduleError),
}

pub enum ScheduleError {
  NotAString { found: &'static str },   // "next_check": 45
  MissingOffset { raw: String },        // 2026-08-22T18:00:00
  Unparseable { raw: String },          // "tomorrow morning"
}

// shell/error.rs  — stratum 2: transport, wrapping the above
pub enum BackendError {
  Spawn(std::io::Error),                        // command not found
  Timeout { after: Duration },
  ExitStatus { code: Option<i32>, stderr: String },
  Io(std::io::Error),
  Protocol(semantics::ProtocolError),
}
```

`MissingOffset` is broken out from `Unparseable` because an offset-less
timestamp is the single most likely backend mistake, and "you omitted the
offset" is a debuggable message where "unparseable" is not. Brief §13 asks for
enough information to debug the backend.

**One thing AC-6 needs stated plainly:** an invalid scheduling value is a
distinct typed error (`ScheduleError`) but it does **not** arrive as an `Err`.
It arrives inside `Normalized::discarded` on an otherwise successful parse.
AC-6's "maps to a distinct typed error" is satisfied; "the call returns an
error" was never what it said, and P2 forbids it.

#### Transport

```rust
// shell/backend/transport.rs
pub trait Backend {
  fn exchange(&self, request: &Request)
    -> impl Future<Output = Result<Vec<u8>, BackendError>> + Send;
}
```

- **The trait owns framing, not just transmission.** It takes a canonical
  `Request` and serializes internally, because the two implementations differ in
  exactly that respect — one JSON body per process, versus one JSONL line per
  exchange on a persistent socket (brief §6). A trait taking pre-serialized
  bytes would have baked the process transport's framing into the seam, which is
  the P3 mistake.
- **Returns `Vec<u8>`, not `String`.** Invalid UTF-8 then becomes a
  `Protocol(Json)` error via `from_slice` rather than a lossy replacement. No
  silent mangling of backend output.
- **Async fn in trait, static dispatch.** Rust 1.99 makes AFIT stable, so no
  `async_trait` dependency and no `Box::pin` on every call; the host is generic
  over `B: Backend`, and tests instantiate it with a fake. Cost, stated because
  it is real: AFIT traits are not `dyn`-compatible, so slice 005's socket-first,
  process-fallback selection will need an enum over the two concrete
  implementations rather than a `Box<dyn Backend>`. That is the cheaper side of
  the trade and it is slice 005's to pay.

#### Host — the crate's public surface

```rust
// shell/host.rs
pub struct Host<B: Backend> { backend: B, config: Config, state: State }

impl<B: Backend> Host<B> {
  pub async fn evaluate(&mut self, now: Timestamp, event: Event) -> Outcome;
  pub async fn respond(&mut self, now: Timestamp, view_id: ViewId, answer: UserResponse) -> Outcome;
}
```

This is the composition point — transport, then `serde_json::from_slice`, then
`normalize_response`, then schedule resolution and state update — and it is what
`tests/integration/` drives for AC-7. `Outcome` carries what the caller must act
on: the view to render if any, the resolved schedule, and the diagnostics
accumulated from discards and errors.

#### Config

```toml
[backend]
command = ["deno", "run", "-A", "./backend.ts"]
timeout = "5s"

[schedule]
default_poll = "30m"
```

Section and key names are brief §5's, minus `socket` and `logging` per the OQ-4
decision. Durations are strings parsed with jiff, the same grammar as
`next_check` — one duration syntax across the product, and no
seconds-versus-milliseconds ambiguity. `command` is an argv array, never a shell
string: no quoting rules, no shell injection surface, and it is what makes
AC-12's `["bash", "./backend.sh"]` work without a shebang.

### 5.3 Data, state & ownership

Nothing is written to disk. Per the OQ-6 decision all host operational state is
in memory, so the state space is one process lifetime wide.

```rust
// shell/state.rs
pub struct State {
  outstanding:    Option<Outstanding>,
  resolved_check: Timestamp,       // always concrete — see below
  next_seq:       u64,
}

struct Outstanding { view_id: ViewId, issued_at: Timestamp }
```

**Three owners, and only one writer.**

| datum | owner | written by | lifetime |
|---|---|---|---|
| `Config` | `Host` | loaded once at construction, immutable after | process |
| `State` | `Host`, private | `Host::evaluate` / `Host::respond` only | process |
| backend `B` | `Host` | never mutated | process |
| diagnostics | nobody | returned in `Outcome`, not retained | one call |

No hot reload of config, and no shared mutability — `State` is a plain struct
behind `&mut self`, not an `Arc<Mutex<…>>`. Brief §12 serializes backend
exchanges and allows one outstanding interaction, so there is no concurrency to
protect against. Adding a lock now would be inventing a state space the brief
explicitly says to avoid.

**`resolved_check` is not an `Option`.** Brief §9 resolves to a concrete instant
in every case — the new valid instruction, else the retained one, else `now +
default_poll` — so there is no "unresolved" state to represent or handle.
`Host::new` takes `now` and seeds it. That keeps the clock out of this slice
entirely: `now` is a parameter on every entry point, and whoever owns real time
is slice 003's timer.

**Diagnostics are per-call, not accumulated.** `Outcome` carries the discards
and errors from that exchange and the host forgets them. Brief §13 wants a
"discoverable diagnostic state", but discovering it requires somewhere to
surface it, and this slice has no renderer and — per OQ-4 — no log file. Holding
a history nothing can display would be building the wrong half. Retention is
slice 002's, and the shape of `Outcome` is what makes it cheap then.

**One outstanding interaction, replaced not queued.** When a response yields a
new view while one is already outstanding, the new `view_id` replaces the old and
the old immediately becomes stale — which is what AC-8 rejects. Brief §12 says
only one may be active; replacement is the only reading that does not require a
queue, and a queue is the general concurrency semantics §12 tells us not to
introduce.

**`view_id` generation is stratum 2, its type is stratum 1.** ADR-001's own
Negative section calls this exact distinction out as one someone must make
deliberately, so: `ViewId` is a newtype in `semantics/protocol/canonical.rs`;
`State::issue` mints one in `shell/`.

The value is `{now, RFC 3339}#{seq}` — for example `2026-08-23T04:12:00Z#3`.
Four reasons over a UUID:

1. No new dependency. `uuid` v4 needs `getrandom`, for a value nothing
   authenticates with.
2. Readable in a log, which is precisely what brief §13 asks of diagnostics.
   `2026-08-23T04:12:00Z#3` tells you when the interaction was issued; a v4
   UUID tells you nothing.
3. Deterministic under a fixed `now`, so fixtures can assert exact ids instead
   of capturing whatever came out.
4. Unique where it needs to be. The counter separates ids minted from the same
   `now`; a differing `now` separates them across restarts. Brief §12's
   one-outstanding-interaction rule means there is never more than one live id
   to collide with.

Not a security consideration either way: backends are trusted user programs
(brief §14), and nothing is authorised by holding a `view_id`.

### 5.4 Lifecycle & dynamics

**Startup.** Load TOML, construct the transport from `backend.command`, then
`Host::new(config, backend, now)` seeds `resolved_check` to
`now + schedule.default_poll`. A malformed or missing config is **fatal at
construction** — there is no backend to run without it, and guessing a command
is not available to us. A backend that cannot be spawned is *not* fatal; that
failure is per-exchange, per brief §13.

**One exchange, in order.** The process transport's five steps, each of which
has a specific way of going wrong:

1. **Spawn** with all three streams piped, and `Command::kill_on_drop(true)` set
   at spawn time.
2. **Write** the serialized request to stdin, then **drop the stdin handle.** The
   close is load-bearing: a backend that reads to EOF — which is the obvious way
   to write one — hangs forever if the host holds stdin open, and the symptom is
   a timeout on every call that looks like a slow backend rather than a host bug.
3. **Drain stdout and stderr concurrently.** `child.wait_with_output()` does
   this. Reading them in sequence deadlocks whenever a backend writes more than
   a pipe buffer (64 KiB on Linux) to the stream we are not yet reading, and it
   deadlocks only for chatty backends, which is the worst possible failure
   distribution.
4. **Await exit.**
5. All of the above inside **one `tokio::time::timeout`** covering the whole
   exchange, not just the read. On elapse the future drops, and `kill_on_drop`
   is what makes tokio SIGKILL and reap the child rather than leaving it behind.

```mermaid
sequenceDiagram
    participant T as tests/integration
    participant H as Host
    participant S as State
    participant P as backend::process
    participant B as backend (deno)

    T->>H: evaluate(now, event)
    H->>P: exchange(&Evaluate)
    P->>B: spawn, write JSON, close stdin
    B-->>P: stdout JSON, exit 0
    P-->>H: Vec<u8>
    H->>H: from_slice → normalize_response(wire, now)
    H->>S: resolve schedule, issue view_id
    H-->>T: Outcome { view: Some, view_id, resolved_check, diagnostics }
    T->>H: respond(now, view_id, answer)
    H->>S: check view_id matches outstanding
    H->>P: exchange(&Respond)
    P->>B: spawn, write JSON, close stdin
    B-->>P: {"view": null, "next_check": …}
    P-->>H: Vec<u8>
    H->>S: clear outstanding, resolve schedule
    H-->>T: Outcome { view: None, resolved_check }
```

Note that `respond` checks `view_id` against `State` **before** touching the
transport. A stale answer must not reach the backend at all — forwarding it and
letting the backend sort it out would make every backend author responsible for
ordering, which is what brief §12 puts in the host.

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Outstanding: view returned, view_id issued
    Outstanding --> Idle: respond(matching id), view: null
    Outstanding --> Outstanding: respond(matching id), new view — id replaced, old now stale
    Outstanding --> Outstanding: respond(stale id) — rejected, no backend call, state untouched
    Idle --> Idle: respond(any id) — rejected, nothing outstanding
```

**Failure does not move the schedule.** Every failure path leaves
`resolved_check` exactly as it was. That is P2 at the lifecycle level: a backend
that fails every invocation still gets polled on its existing cadence, because
the alternative — a failed exchange clearing or extending the schedule — turns a
broken backend into a silent host.

**No retry.** Nothing in the brief asks for one, and P3 forbids building the
seam for it. A failed exchange returns a diagnostic `Outcome` and the next
scheduled evaluation is the retry.

**Non-zero exit beats parseable stdout.** If a backend exits non-zero but wrote
valid JSON, the host reports `ExitStatus` and discards the output. The backend
told us it failed; trusting output it disclaimed would be the host deciding it
knows better.

**A timed-out backend yields no stderr.** `wait_with_output()`'s buffers drop
with the timed-out future, so the most confusing failure mode gives the least
diagnostic information. Accepted for this slice rather than fixed: AC-5 asks for
stderr capture and AC-6 for a distinct timeout error, and neither requires both
at once, and there is no logging surface here to display it on. The fix — take
`stderr` at spawn, drain it in a spawned task into a shared buffer, read that
buffer on the timeout path — is small and recorded as a follow-up in
`slice-001.md`. Do not let it become the reason a later reader "simplifies"
`kill_on_drop` away.

**Two things stated because they are lifecycle behaviour, not code detail:** the
timeout is per exchange rather than per byte, so a backend that streams slowly
but steadily still dies at the deadline; and there is no keep-alive, warm
process, or connection reuse — a fresh process per invocation is the whole
transport, which is why slice 005 exists.

### 5.5 Invariants, assumptions & edge cases

**Invariants.**

| id | invariant | held by |
|---|---|---|
| I1 | No canonical type can hold an unnormalized value | P1; checked constructors |
| I2 | No file in `semantics/` names `crate::shell`, `crate::bin` or `tokio` | AC-15 test |
| I3 | Stratum 1 never reads a clock, filesystem or network, and spawns nothing. `now` is always a parameter | ADR-001; jiff with default features off |
| I4 | `resolved_check` always holds a concrete instant | non-`Option` field |
| I5 | At most one outstanding interaction | `Option<Outstanding>` |
| I6 | Only one exchange in flight | `&mut self` — compiler-enforced, not convention |
| I7 | The host never branches on a `hints` key | review; brief §3.4 |
| I8 | No domain vocabulary in types or module names | AC-11 grep |
| I9 | No path panics on backend-derived data — no `unwrap`, `expect` or slicing on anything a backend produced | AC-6; clippy lints |
| I10 | No inbound wire type is a closed contract: no `deny_unknown_fields`, and the absence of a field never means more than "not supplied" | review; see the validation-feedback analysis below |

**Assumptions.** Each is a place this design can break.

- **A1 — stale-`view_id` rejection is scoped to one process lifetime.** From the
  OQ-6 in-memory decision: a restart *forgets* the outstanding interaction rather
  than rejecting a response to it. So AC-8 holds within a process and is silent
  across one. Nothing in this slice can answer an interaction that predates the
  process, because nothing renders one — the assumption becomes load-bearing in
  slice 002, when a user can be looking at a prompt when the host restarts.
- **A2 — modules respect the strata beyond the three tokens AC-15 checks.** The
  test catches an upward `use` naming `shell`; it does not catch a downward type
  leak, a re-export that flattens the boundary, or `std::fs` in `semantics/`. If
  discipline slips, ADR-002's split becomes a redesign rather than a file move,
  which ADR-002 says would itself be the finding. Carried as a risk in §8.
- **A3 — jiff's friendly duration grammar is stable across 0.2.x.** It is
  pre-1.0. `"45 minutes"`, `"1h 30m"`, `"2 hours"` and `PT45M` were verified
  against 0.2.35 by running them; if the grammar changes under a patch bump, what
  `next_check` accepts changes with it. Mitigated by the AC-9 fixture corpus
  pinning the accepted and rejected forms as tests.
- **A4 — the brief is intent, not canon, and three of its ambiguities were
  resolved by choice**: the protocol-version asymmetry, non-zero exit taking
  precedence over parseable stdout, and replacement rather than queueing for a
  second outstanding view. Each is defensible and each could be wrong; all three
  are in §7 so a reader knows they were chosen rather than derived.

**Edge cases.**

| case | behaviour |
|---|---|
| `view: null` with a valid `next_check` | nothing to show; schedule updates; any outstanding interaction is left alone |
| scheduled evaluate fires while an interaction is outstanding | permitted; a returned view replaces the outstanding one. *Whether* to poll while a prompt is unanswered is policy, and belongs to slice 003 |
| `next_check` in the past, incl. `"-45 minutes"` | parses; resolution clamps to no earlier than `now`. A minimum wake interval is slice 003's |
| `"next_check": 45` | `ScheduleError::NotAString`, discarded, message accepted |
| `"next_check": "2026-08-22T18:00:00"` | `MissingOffset`, discarded, message accepted |
| `options: []` | `EmptyOptions` — whole message rejected |
| duplicate option ids | `DuplicateOptionId` — whole message rejected |
| empty stdout, exit 0 | `Protocol(Json)` — unexpected EOF |
| two JSON documents on stdout | `Protocol(Json)` on trailing content. Strictness is correct here: framing is the transport's job and this transport's frame is "one document" |
| `command = []` in config | rejected at load — nothing to spawn |
| `timeout = "0s"` or `default_poll = "0s"` | rejected at load. A zero timeout fails every exchange; a zero poll is a busy loop |
| backend emits unbounded stdout | read unbounded — see below |

**The unbounded-output edge is the one real gap.** `wait_with_output()` reads
until EOF with no cap, so a backend stuck in a print loop can exhaust host
memory — which contradicts brief §13's "a backend failure must not take down the
host" directly, rather than merely being untidy.

Bounding the read requires replacing `wait_with_output()` with a manual
concurrent drain of both pipes — **the same change as the stderr-on-timeout
follow-up**. Both wants are therefore one refactor, and doing it once with both
requirements in hand beats doing it twice. Recorded as a single follow-up on
those grounds, not because the risk is negligible.

**The host does not validate answers; the backend does.** `respond` checks the
`view_id` and nothing else. It does not verify that the answer names an option
the view offered, and it does not inspect field values at all. Field values pass
through opaque.

That decision is only safe if the eventual validation-feedback round trip — the
backend rejecting a submission, and the user seeing which fields were wrong and
why — can be added without a protocol break. Checked here, while it is free,
because brief §22.3 asks exactly this: *are we narrowing the protocol to match
the current v0 renderer?* It is additive, and three decisions already made are
what keep it so:

1. **`UserResponse.values` is opaque.** The host can retain and echo submitted
   values without understanding any of them, so retention is a mechanical copy
   rather than domain knowledge — it does not cross brief §22.5's boundary.
   Whether the host retains them or the backend echoes them back in the
   re-presented view is a mechanism choice for that slice; neither is foreclosed.
2. **A returned view replaces the outstanding one and takes a fresh `view_id`**
   (§5.4). A validation rejection is therefore just another view, one that
   happens to resemble its predecessor. The host never needs a notion of "this is
   a retry of that", which is the expensive version of the feature. Everything it
   needs becomes additive fields on a view the host already carries: `field.value`
   for prefill or for a backend-corrected replacement, `field.error` for the
   per-field message, and a form-level message on the choice.
3. **No `deny_unknown_fields` inbound**, so an older host tolerates a backend
   already sending those fields and a newer host reads them. No protocol version
   bump — which is the test of whether an extension is genuinely additive. I10
   exists to keep this true.

One thing is deliberately settled in advance, because the tempting shortcut is
wrong: **per-field validation errors are semantics, not hints.** A renderer may
ignore `placeholder`; it must not be free to ignore a validation error. So they
belong in typed fields, never as keys in the open `hints` map.

## 6. Open questions

**None open.** OQ-1 through OQ-10 are answered in `slice-001.md`, each struck
through with its answer; OQ-8 was raised by OQ-3's answer during design and
closed in the same stage. OQ-1's answer was subsequently revised — the protocol
spec is drafted in the slice folder and promoted at close rather than written at
audit (AC-13, AC-14).

Two items are deliberately left to implementation rather than being open design
questions:

- The serde encoding that makes an unrecognised `view.kind` produce
  `UnsupportedPrimitive { kind }` rather than a generic deserialization error.
  The contract is fixed in §5.2; the attribute that achieves it is not.
- Whether `Hints` is a `BTreeMap<String, serde_json::Value>` or a thinner
  newtype over it. No behaviour depends on the answer.

Two items are deferred *work*, not open questions, and are recorded as
follow-ups in `slice-001.md`: the bounded/drained pipe refactor, and the
validation-feedback round trip.

## 7. Decisions, rationale & alternatives

Full rationale for each is in `design-log.md`, dated. This is the index a later
reader needs so they do not reverse one by accident.

| id | decision | rejected | why |
|---|---|---|---|
| D1 | `src/` grouped by stratum (`semantics/`, `shell/`) | grouping by topic | an upward `use` is wrong on sight; ADR-002's split becomes a literal `git mv` |
| D2 | library only, stratum 3 empty | a debug binary | no AC needs one; P3. Forces the public API to be usable by the tests |
| D3 | tokio, features `process`/`time`/`rt`/`io-util` | smol family | 14 deps vs 31, measured; `tokio::process` has the mature child-reaping path |
| D4 | jiff, `default-features = false` | chrono, time | parses `"45 minutes"` natively; defaults would pull tzdb, i.e. I/O into stratum 1 |
| D5 | wire/canonical duality **inbound only** | mirroring both directions | requests are host-authored; nothing untrusted arrives on that path |
| D6 | `next_check` typed `serde_json::Value` at the wire | `Option<String>` | a wrong-typed value must be *discardable*, not fatal — P2 |
| D7 | protocol version asymmetric: always written, optional inbound, unknown-declared rejected | required inbound; ignored inbound | requiring it rejects backends written against brief §8.2; ignoring it is guessing |
| D8 | unrecognised `view.kind` → `UnsupportedPrimitive { kind }` | generic serde failure | AC-6 wants a distinct error; brief §13 wants it debuggable |
| D9 | semantics in `FieldKind`, presentation in an open `hints` map | one flat struct of hints | brief §3.4's line. §10.2 calls its hint list provisional, so fixing it in a struct narrows the protocol |
| D10 | `Discarded` is a closed enum | `Vec<(String, Error)>` | adding a variant is the moment P2's eligibility test must be argued |
| D11 | AFIT trait, static dispatch, generic `Host<B>` | `async_trait` + `Box<dyn>` | no dependency, no per-call boxing. Cost: slice 005 needs an enum, not `dyn` |
| D12 | transport returns `Vec<u8>` | `String` | invalid UTF-8 becomes a JSON error, not a lossy replacement |
| D13 | `view_id` = `{now}#{seq}` | uuid v4 | no dep; readable in diagnostics; deterministic in fixtures; nothing authenticates with it |
| D14 | `State` behind `&mut self`, no lock | `Arc<Mutex<…>>` | brief §12 serializes exchanges; a lock invents a state space §12 says to avoid |
| D15 | non-zero exit beats parseable stdout | trust the output | the backend disclaimed it; trusting it anyway is the host overruling the backend |
| D16 | a second view replaces the outstanding one | queueing | brief §12 allows one active interaction; a queue is the concurrency it forbids |
| D17 | host validates `view_id` only; the backend validates answers | retain the view, check option ids | user decision. Validation feedback confirmed additive, §5.5 |
| D18 | accept: no stderr on timeout | drain via a spawned task | ~10 lines, but same refactor as D19; no surface to show it on yet |
| D19 | accept: unbounded stdout read | bounded manual drain | folded into one future refactor with D18 |
| D20 | draft spec in the slice folder, promoted at close | spec written at audit | gives execution a prose contract without making it canon early |
| D21 | `Command::kill_on_drop(true)` | explicit kill after timeout | `wait_with_output()` consumes the child, so explicit kill is unavailable. Removing this leaks the process |

## 8. Risks & mitigations

| id | risk | l/i | mitigation | signal it is happening |
|---|---|---|---|---|
| R1 | ADR-001's one-way rule has no compiler behind it (A2), so the strata erode and ADR-002's split becomes a redesign | med / high | AC-15 test on the three strongest tokens; D1's stratum-visible paths; review | a `use crate::shell::…` under `semantics/`, `std::fs` there, or slice 002 finding the split hard |
| R2 | jiff is pre-1.0; its friendly duration grammar could shift under a patch bump (A3) | low / med | AC-9 fixtures pin the accepted *and* rejected forms as tests | a dependency bump turns fixtures red |
| R3 | unbounded stdout exhausts host memory — contradicts brief §13 outright | low / high | accepted (D19); follow-up recorded, paired with D18 | host OOM against a looping backend |
| R4 | **the protocol gets narrowed to the first renderer anyway** — the failure this slice exists to prevent | med / high | fields and all `Content` variants admitted now; validation feedback proved additive; AC-10 puts the warning in `AGENTS.md` | slice 002 needing a protocol change, not just a renderer, to display something |
| R5 | the draft spec drifts from the code and is promoted as intent — the exact risk OQ-1's original answer avoided | med / med | AC-14: reconcile before promotion, divergences dispositioned per `docs/AGENTS.md` | audit finding the draft easier to believe than the code |
| R6 | the suite proves only that deno works, not that any command works | low / med | AC-12's bash backend | — mitigated by construction |
| R7 | the field and content type surface is the largest chunk here and nothing renders it, inviting gold-plating | med / low | only brief-named vocabulary; P3's second half | a `FieldKind` or hint the brief never mentions |
| R8 | the timeout diagnostic gap (D18) causes a hang to be misdiagnosed as a slow backend | med / low | §5.4 names the stdin-close trap explicitly, which is the usual cause | repeated timeouts against a backend that works when run by hand |

## 9. Validation

**Verification commands** (AC-1, and named in `AGENTS.md` per AC-10):

```
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

**Three test tiers, with different dependency reach:**

| tier | location | reaches | drives |
|---|---|---|---|
| protocol | `tests/protocol/` | stratum 1 only — no tokio, no subprocess | the AC-9 fixture corpus, table-driven |
| boundary | `tests/protocol/` | source text | AC-15 |
| integration | `tests/integration/` | full stack, spawns deno and bash | AC-7, AC-5, AC-6, AC-12 |

**Fixtures are data files, not Rust literals**, walked by a table-driven runner —
AC-9 calls for a corpus, and a corpus in `.json` files is reviewable by someone
reading the protocol rather than the tests, which matters for the draft spec.

**AC coverage map the plan must preserve:**

| AC | discharged by |
|---|---|
| AC-1 | the four commands above, from a clean clone |
| AC-2 | protocol tier: version present, unknown optional ignored, unknown required rejected |
| AC-3 | protocol tier: RFC 3339 and relative forms → one instant; `MissingOffset`, `Unparseable` rejected |
| AC-4 | protocol tier: pure resolution over (existing, incoming, default), latest-valid-wins, invalid preserves |
| AC-5 | integration: stdin write, stdout read, timeout, stderr captured |
| AC-6 | integration + protocol: each failure mode to its own variant; `ScheduleError` via `discarded`, not `Err` |
| AC-7 | integration: `view: null` → choice → `view_id` → respond → accepted |
| AC-8 | integration: stale and unknown `view_id` rejected, no backend spawn |
| AC-9 | the corpus itself |
| AC-10 | review against brief §15.1's list |
| AC-11 | grep over `src/` for the named vocabulary |
| AC-12 | integration: the bash backend completes a round trip |
| AC-13 | the draft exists, `R-N` ids present, every requirement in its §7 |
| AC-14 | at close: reconciled, endorsed, promoted |
| AC-15 | the boundary test |

**Deliberately misbehaving backends** the integration tier needs: sleeps past the
timeout; exits non-zero after writing valid JSON; writes malformed JSON; writes
nothing; declares an unknown protocol version; returns `options: []`; returns
duplicate option ids; returns an unknown `view.kind`; returns
`"next_check": 45`. Command-not-found needs no fixture, only a path that does
not exist.

**Not validated here, and named so nobody assumes otherwise:** nothing renders,
nothing wakes on a clock, no socket transport, and no cross-restart behaviour.

## 10. Canon impact

| canon | impact | settles how |
|---|---|---|
| ADR-001 | **decision unchanged; the record needs a line.** AC-15 mechanises part of what its Verification section calls "a review gate, not a build gate" | `canon-delta.md`. Per `docs/AGENTS.md` an ADR's *decision* is fixed while its record is kept accurate, so this is a delta entry, not a supersession — the one-way rule is untouched, only what verifies it. The delta must also state what AC-15 does *not* cover: three known tokens, so the common case rather than the class |
| ADR-002 | **no change.** Its Verification requires the triggers be checked and recorded in the design of any slice adding a dependency or binary; done in §3, all three negative | nothing to settle |
| protocol spec | **new canon, owed.** Drafted at `docs/slices/001/draft-spec.md`, promoted to `docs/specs/NNN-slug.md` with `Status: active` during audit | AC-13, AC-14. Promotion needs explicit user endorsement, and `docs/AGENTS.md` is explicit that a slice does not close holding an unpromoted draft |
| `canon-delta.md` | **owed**, for the ADR-001 record line above. One entry: the document, the section, the change as it will be stated, and why | applied during reconciliation, with endorsement |
| `docs/policy/` | none created. Nothing here is a policy rather than a decision | — |
| root `AGENTS.md` | not canon by `docs/AGENTS.md`'s definition, but a deliverable, and now additive rather than from-scratch | AC-10 |

**Design drift not reconciled:** empty. Audit fills this in; an entry here means
the implementation departed and the design stands as written.
